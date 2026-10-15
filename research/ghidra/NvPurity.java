// Purity facts: what each function touches besides its own stack, for the
// automatic oracle check (docs/HANDOFF.md next task 10, #182).
//
// Args: out=<file.tsv>
//
// Read-only (run with -readOnly). Writes one row per function, sorted by
// entry:
//
//   entry, size, ret_pop, ebp_frame, calls, tail_calls, indirect, imports,
//   w_stack, w_global, w_pointer, w_implicit, r_pointer, fs
//
//   ret_pop     bytes the function pops on return (the largest `RET n`; 0 for
//               plain RET; -1 when the body has no RET, e.g. it only tail
//               calls or never returns). Callee cleanup (stdcall/thiscall
//               with stack arguments) when > 0.
//   ebp_frame   1 when the entry is `PUSH EBP; MOV EBP,ESP`, so EBP-based
//               operands are the frame; otherwise EBP is a general register
//               and EBP-based stores count as pointer stores.
//   calls       entries of directly called functions, ','-separated, call
//               order, repeats kept.
//   tail_calls  entries reached by an unconditional JMP to another
//               function's entry.
//   indirect    CALLs and JMPs whose target is not a known function and not
//               inside the body (vtable and function-pointer calls; a switch
//               JMP whose targets are all in the body does not count).
//   imports     calls or jumps through an import slot.
//   w_stack     stores to the frame (ESP-based, or EBP-based with ebp_frame);
//               PUSH and CALL count here too.
//   w_global    stores to an absolute address (a global, with or without an
//               index register).
//   w_pointer   stores through any other base register.
//   w_implicit  stores with no explicit memory destination that are not a
//               PUSH/CALL/ENTER (string instructions such as REP STOSD).
//   r_pointer   loads through a base register that is not the frame and
//               without an absolute displacement (reading an object that a
//               pointer argument or a global points at).
//   fs          number of operands that use the FS segment (TLS block, SEH
//               frame); any FS store is also counted in w_pointer.
//   reg_in      registers among ECX and EDX that the entry block reads before
//               writing ("ECX", "ECX,EDX" or empty): ECX alone means
//               __thiscall (or __fastcall with one register argument), both
//               __fastcall; empty means every argument is on the stack.
//   stack_args  bytes of stack arguments the body reads, measured from
//               [EBP+8+n] operands (a lower bound: an argument the body never
//               reads is not counted); -1 without an EBP frame.
//   consts_int  immediate operands that are not addresses or stack
//               adjustments (CMP EAX,0x14 -> 20), signed decimal, ','.
//   consts_float the values x87/SSE instructions load from absolute
//               addresses: f<8 hex> (dword) or d<16 hex> (qword), ','.
//               Both are test inputs for the oracle check (boundaries).
//
// A function is pure for the oracle check when it and every function it
// calls (transitively) has indirect = imports = w_global = w_pointer =
// w_implicit = fs = 0; whether r_pointer is acceptable depends on its
// arguments. The join with the translations is done outside Ghidra
// (research/oracle-pure).
//
// The output holds addresses and counts only (no code, no strings), but it
// describes a program you own: keep it in the private research tree.
//
// Gate: rows written == getFunctionCount() minus external functions,
// otherwise the script throws and writes nothing.
//
// @category NV

import java.io.BufferedWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.lang.Register;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.pcode.PcodeOp;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.FlowType;
import ghidra.program.model.symbol.RefType;
import ghidra.program.model.symbol.Reference;

public class NvPurity extends GhidraScript {

    private static final String NAME = "NvPurity";

    /** Lowest address of the image; an operand constant at or above it is an absolute address. */
    private long imageBase;

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(NAME, getScriptArgs(), Set.of("out"));
        Path out = Paths.get(NvCommon.require(NAME, args, "out"));
        if (out.getParent() != null) {
            Files.createDirectories(out.getParent());
        }
        imageBase = currentProgram.getImageBase().getOffset();

        FunctionManager fm = currentProgram.getFunctionManager();
        Listing listing = currentProgram.getListing();
        int externals = 0;
        for (Function f : fm.getExternalFunctions()) {
            externals++;
        }
        int expected = fm.getFunctionCount() - externals;

        List<String> lines = new ArrayList<>();
        lines.add("entry\tsize\tret_pop\tebp_frame\tcalls\ttail_calls\tindirect\timports\t"
            + "w_stack\tw_global\tw_pointer\tw_implicit\tr_pointer\tfs\treg_in\tstack_args\t"
            + "consts_int\tconsts_float");
        for (Function f : fm.getFunctions(true)) {
            monitor.checkCancelled();
            if (f.isExternal()) {
                continue;
            }
            lines.add(row(f, fm, listing));
        }
        int rows = lines.size() - 1;
        if (rows != expected) {
            throw new IllegalStateException(NAME + ": wrote " + rows + " rows, the program has "
                + expected + " non-external functions");
        }
        try (BufferedWriter w = Files.newBufferedWriter(out, StandardCharsets.UTF_8)) {
            for (String l : lines) {
                w.write(l);
                w.write('\n');
            }
        }
        println(NAME + ": " + rows + " functions written to " + out);
    }

    private String row(Function f, FunctionManager fm, Listing listing) {
        AddressSetView body = f.getBody();
        boolean ebpFrame = hasEbpFrame(listing, f.getEntryPoint());
        long retPop = -1;
        List<String> calls = new ArrayList<>();
        List<String> tails = new ArrayList<>();
        int indirect = 0, imports = 0;
        int wStack = 0, wGlobal = 0, wPointer = 0, wImplicit = 0, rPointer = 0, fs = 0;
        java.util.LinkedHashSet<Long> intConsts = new java.util.LinkedHashSet<>();
        java.util.LinkedHashSet<String> floatConsts = new java.util.LinkedHashSet<>();

        for (Instruction ins : listing.getInstructions(body, true)) {
            String mnem = ins.getMnemonicString().toUpperCase();
            FlowType ft = ins.getFlowType();
            collectConstants(ins, mnem, intConsts, floatConsts);

            if (mnem.startsWith("RET")) {
                long pop = 0;
                if (ins.getNumOperands() > 0) {
                    Scalar s = ins.getScalar(0);
                    if (s != null) {
                        pop = s.getUnsignedValue();
                    }
                }
                retPop = Math.max(retPop, pop);
            }

            boolean call = ft.isCall();
            boolean jump = ft.isJump() && !ft.isConditional();
            if (call || jump) {
                boolean viaImport = false;
                for (Reference r : ins.getReferencesFrom()) {
                    if (r.isExternalReference()) {
                        viaImport = true;
                    }
                    else if (r.getReferenceType().isData() && r.getToAddress().isMemoryAddress()) {
                        for (Reference r2 : getReferencesFrom(r.getToAddress())) {
                            if (r2.isExternalReference()) {
                                viaImport = true;
                            }
                        }
                    }
                }
                Address[] flows = ins.getFlows();
                boolean resolved = false;
                for (Address t : flows) {
                    if (jump && body.contains(t)) {
                        resolved = true; // switch or loop inside the body
                        continue;
                    }
                    Function g = fm.getFunctionAt(t);
                    if (g == null) {
                        continue;
                    }
                    resolved = true;
                    if (g.isExternal()) {
                        viaImport = true;
                    }
                    else if (call) {
                        calls.add(NvCommon.addr(g.getEntryPoint()));
                    }
                    else {
                        tails.add(NvCommon.addr(g.getEntryPoint()));
                    }
                }
                if (viaImport) {
                    imports++;
                }
                else if (!resolved && (call || ft.isComputed())) {
                    indirect++;
                }
            }

            // Operand-level loads and stores.
            boolean explicitStore = false;
            for (int i = 0; i < ins.getNumOperands(); i++) {
                // A memory operand is written in brackets ([ESP+0x4], dword ptr [0x011dea3c],
                // FS:[0x0]); register and immediate operands are not.
                if (!isBracketed(ins, i)) {
                    continue;
                }
                if (ins.getDefaultOperandRepresentation(i).contains("FS:")) {
                    fs++;
                }
                // Ghidra has no reference type for some operands; then the p-code decides
                // (a STORE in the instruction means its memory operand is written).
                RefType rt = ins.getOperandRefType(i);
                boolean write = rt != null ? rt.isWrite() : hasStore(ins);
                boolean read = rt != null ? rt.isRead() : !write;
                Base base = base(ins, i, ebpFrame);
                if (write) {
                    explicitStore = true;
                    if (base == Base.STACK) {
                        wStack++;
                    }
                    else if (base == Base.ABSOLUTE) {
                        wGlobal++;
                    }
                    else {
                        wPointer++;
                    }
                }
                else if (read && base == Base.POINTER && !mnem.equals("LEA")) {
                    rPointer++;
                }
            }
            if (!explicitStore && hasStore(ins)) {
                if (mnem.startsWith("PUSH") || mnem.equals("CALL") || mnem.equals("ENTER")) {
                    wStack++;
                }
                else {
                    wImplicit++;
                }
            }
        }

        return NvCommon.addr(f.getEntryPoint()) + "\t" + body.getNumAddresses() + "\t" + retPop + "\t"
            + (ebpFrame ? 1 : 0) + "\t" + String.join(",", calls) + "\t" + String.join(",", tails) + "\t"
            + indirect + "\t" + imports + "\t" + wStack + "\t" + wGlobal + "\t" + wPointer + "\t"
            + wImplicit + "\t" + rPointer + "\t" + fs + "\t" + registersIn(listing, f.getEntryPoint(), body)
            + "\t" + stackArgBytes(listing, body, ebpFrame)
            + "\t" + joinLongs(intConsts) + "\t" + String.join(",", floatConsts);
    }

    private static final int MAX_CONSTS = 32;

    private static String joinLongs(java.util.Collection<Long> values) {
        List<String> out = new ArrayList<>();
        for (Long v : values) {
            out.add(Long.toString(v));
        }
        return String.join(",", out);
    }

    /**
     * The constants a function compares and computes with, as test inputs for the oracle check:
     * immediate operands that are not addresses or stack adjustments (CMP EAX,0x14; AND ECX,0x1FF),
     * and the float/double values that x87 and SSE instructions load from absolute addresses
     * (FCOMP double ptr [0x01012060] -> its 8 bytes as a double). Floats are written f&lt;8 hex
     * digits&gt;, doubles d&lt;16 hex digits&gt;; at most 32 of each kind.
     */
    private void collectConstants(Instruction ins, String mnem, java.util.Set<Long> ints,
            java.util.Set<String> floats) {
        // ADD/SUB ESP,n and the like: ESP or EBP as a bare register operand (not inside a
        // memory operand such as CMP dword ptr [EBP+0x8],0xa, whose 0xa is wanted).
        boolean stackAdjust = false;
        for (int i = 0; i < ins.getNumOperands(); i++) {
            if (isBracketed(ins, i)) {
                continue;
            }
            for (Object o : ins.getOpObjects(i)) {
                if (o instanceof Register) {
                    String n = ((Register) o).getName().toUpperCase();
                    if (n.equals("ESP") || n.equals("EBP")) {
                        stackAdjust = true;
                    }
                }
            }
        }
        boolean x87 = mnem.startsWith("F");
        boolean sse = mnem.endsWith("SS") || mnem.endsWith("SD") || mnem.startsWith("COMIS")
            || mnem.startsWith("UCOMIS") || mnem.startsWith("CVT");
        for (int i = 0; i < ins.getNumOperands(); i++) {
            String rep = ins.getDefaultOperandRepresentation(i);
            if (!isBracketed(ins, i)) {
                if (stackAdjust || mnem.startsWith("RET") || ins.getFlowType().isCall()
                    || ins.getFlowType().isJump()) {
                    continue;
                }
                for (Object o : ins.getOpObjects(i)) {
                    if (o instanceof Scalar) {
                        long v = ((Scalar) o).getSignedValue();
                        if (Math.abs(v) < imageBase && ints.size() < MAX_CONSTS) {
                            ints.add(v);
                        }
                    }
                }
                continue;
            }
            if (!(x87 || sse)) {
                continue;
            }
            // Ghidra writes x87 memory operands as "float ptr"/"double ptr", SSE ones as
            // "dword ptr"/"qword ptr"; 80-bit ("float10") operands are skipped.
            int size = rep.startsWith("dword ptr") || rep.startsWith("float ptr") ? 4
                : rep.startsWith("qword ptr") || rep.startsWith("double ptr") ? 8 : 0;
            if (size == 0) {
                continue;
            }
            Long abs = null;
            boolean indexed = false;
            for (Object o : ins.getOpObjects(i)) {
                if (o instanceof Register) {
                    indexed = true;
                }
                else if (o instanceof Scalar && ((Scalar) o).getUnsignedValue() >= imageBase) {
                    abs = ((Scalar) o).getUnsignedValue();
                }
                else if (o instanceof Address && ((Address) o).getOffset() >= imageBase) {
                    abs = ((Address) o).getOffset();
                }
            }
            if (abs == null || indexed || floats.size() >= MAX_CONSTS) {
                continue;
            }
            Address a = toAddr(abs);
            if (!NvCommon.inInitializedMemory(currentProgram, a)) {
                continue;
            }
            byte[] b = NvCommon.readBytes(currentProgram, a, size);
            if (b == null || b.length != size) {
                continue;
            }
            long bits = NvCommon.le(b, 0, size);
            floats.add(size == 4 ? String.format("f%08x", bits) : String.format("d%016x", bits));
        }
    }

    /**
     * ECX and EDX among the registers the entry block reads before it writes them, following
     * fall-through only (the first CALL, jump or return ends the block). Sub-registers count as
     * their full register (CL reads ECX).
     */
    private String registersIn(Listing listing, Address entry, AddressSetView body) {
        String[] watched = { "ECX", "EDX" };
        boolean[] written = new boolean[2];
        boolean[] readFirst = new boolean[2];
        Instruction ins = listing.getInstructionAt(entry);
        for (int n = 0; ins != null && body.contains(ins.getAddress()) && n < 64; n++) {
            String mnem = ins.getMnemonicString().toUpperCase();
            // Idioms that name a register without using its value: PUSH ECX in a prologue
            // reserves a local slot, XOR/SUB ECX,ECX zeroes it.
            boolean push = mnem.equals("PUSH");
            boolean zeroing = (mnem.equals("XOR") || mnem.equals("SUB")) && ins.getNumOperands() == 2
                && ins.getDefaultOperandRepresentation(0).equals(ins.getDefaultOperandRepresentation(1));
            for (int k = 0; k < 2; k++) {
                if (written[k] || readFirst[k]) {
                    continue;
                }
                if (push) {
                    continue;
                }
                if (zeroing && touches(ins.getResultObjects(), watched[k])) {
                    written[k] = true;
                }
                else if (touches(ins.getInputObjects(), watched[k])) {
                    readFirst[k] = true;
                }
                else if (touches(ins.getResultObjects(), watched[k])) {
                    written[k] = true;
                }
            }
            FlowType ft = ins.getFlowType();
            if (!ft.isFallthrough() || ft.isCall()) {
                break;
            }
            ins = ins.getNext();
        }
        List<String> out = new ArrayList<>();
        for (int k = 0; k < 2; k++) {
            if (readFirst[k]) {
                out.add(watched[k]);
            }
        }
        return String.join(",", out);
    }

    private static boolean touches(Object[] objects, String full) {
        for (Object o : objects) {
            if (o instanceof Register) {
                Register r = (Register) o;
                Register base = r.getBaseRegister();
                String n = r.getName().toUpperCase();
                String b = base == null ? n : base.getName().toUpperCase();
                if (n.equals(full) || b.equals(full) || b.equals("R" + full.substring(1))) {
                    return true;
                }
            }
        }
        return false;
    }

    /**
     * End of the stack arguments the body reads, in bytes past the return address: the largest
     * [EBP+d] with d >= 8, as d - 8 + 4 (one dword; an 8-byte FLD QWORD at the last slot reads as
     * 4 bytes short, so this is a lower bound). -1 without an EBP frame (ESP-relative offsets move
     * with every PUSH, so they are not measured).
     */
    private static long stackArgBytes(Listing listing, AddressSetView body, boolean ebpFrame) {
        if (!ebpFrame) {
            return -1;
        }
        long end = 0;
        for (Instruction ins : listing.getInstructions(body, true)) {
            for (int i = 0; i < ins.getNumOperands(); i++) {
                if (!isBracketed(ins, i)) {
                    continue;
                }
                boolean ebp = false;
                long disp = Long.MIN_VALUE;
                for (Object o : ins.getOpObjects(i)) {
                    if (o instanceof Register && ((Register) o).getName().equalsIgnoreCase("EBP")) {
                        ebp = true;
                    }
                    else if (o instanceof Scalar) {
                        disp = ((Scalar) o).getSignedValue();
                    }
                }
                if (ebp && disp >= 8 && disp < 0x400) {
                    end = Math.max(end, disp - 8 + 4);
                }
            }
        }
        return end;
    }

    private enum Base { STACK, ABSOLUTE, POINTER }

    /** Where a memory operand points: the frame, an absolute address (optionally indexed), or elsewhere. */
    private Base base(Instruction ins, int i, boolean ebpFrame) {
        boolean stack = false, absolute = false, otherReg = false;
        for (Object o : ins.getOpObjects(i)) {
            if (o instanceof Register) {
                String n = ((Register) o).getName().toUpperCase();
                if (n.equals("ESP") || (ebpFrame && n.equals("EBP"))) {
                    stack = true;
                }
                else if (n.equals("FS") || n.equals("DS") || n.equals("SS") || n.equals("ES")) {
                    // segment prefix: FS is reported separately; the others are the flat model
                    if (n.equals("FS")) {
                        otherReg = true;
                    }
                }
                else {
                    otherReg = true;
                }
            }
            else if (o instanceof Scalar) {
                if (((Scalar) o).getUnsignedValue() >= imageBase) {
                    absolute = true;
                }
            }
            else if (o instanceof Address) {
                if (((Address) o).getOffset() >= imageBase) {
                    absolute = true;
                }
            }
        }
        if (stack) {
            return Base.STACK;
        }
        if (absolute) {
            return Base.ABSOLUTE; // [abs] or [abs + index*scale]
        }
        return otherReg ? Base.POINTER : Base.ABSOLUTE;
    }

    private static boolean isBracketed(Instruction ins, int i) {
        String s = ins.getDefaultOperandRepresentation(i);
        return s.contains("[");
    }

    private static boolean hasStore(Instruction ins) {
        for (PcodeOp op : ins.getPcode()) {
            if (op.getOpcode() == PcodeOp.STORE) {
                return true;
            }
        }
        return false;
    }

    private static boolean hasEbpFrame(Listing listing, Address entry) {
        Instruction a = listing.getInstructionAt(entry);
        if (a == null || !a.toString().equalsIgnoreCase("PUSH EBP")) {
            return false;
        }
        Instruction b = a.getNext();
        return b != null && b.toString().replace(" ", "").equalsIgnoreCase("MOVEBP,ESP");
    }
}
