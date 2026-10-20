//! `(unplaced)/run_00ec36a6` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec3bda (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the two bytes of the low half (unnamed in the map; a runtime
/// library function, `cdecl`, one word; the half-word swap `00407a90`
/// calls it). The whole of `EAX` is the result: the low half is the swapped
/// half (`(value << 8) + (value >> 8)` in 16 bits), the high half is bits 8
/// to 23 of the argument, which callers ignore.
pub fn fn_00ec3bda(_e: &mut Engine, value: u32) -> u32 {
    let shifted = value << 8;
    let low = (shifted as u16).wrapping_add((value as u16) >> 8);
    (shifted & 0xffff_0000) | low as u32
}

/// `___report_gsfailure` (`00ed159c`): the stack-cookie failure handler; it
/// does not return.
const REPORT_GS_FAILURE: u32 = 0x00ed_159c;
/// The security cookie (`.data`).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;

// Translated from 00ec408c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__security_check_cookie` (CRT, `__fastcall`): returns when `cookie`
/// equals the security cookie at `011c16bc`, otherwise jumps to
/// `___report_gsfailure` (`00ed159c`), which never returns.
pub fn security_check_cookie(e: &mut Engine, cookie: u32) {
    if cookie == e.global::<u32>(SECURITY_COOKIE_011C16BC) {
        return;
    }
    e.call(REPORT_GS_FAILURE, &args![]);
}

/// The call at the end (`00ad0470`: the stream base at `this + offset` plus two stack words,
/// `RET 8`). The engine map puts it in `FaceGen/algs.obj`; its identity is unconfirmed.
const STREAM_BASE_INIT_00AD0470: u32 = 0x00ad_0470;
/// Vbtable stored at `+0` and at `+4` when the most-derived flag is set; the objects they
/// point at have RTTI names `basic_ostream` (`010a318c`) and `basic_ios` (`010a2fa4`).
const VBTABLE_010A3190: u32 = 0x010a_3190;
const VBTABLE_010A2FA4: u32 = 0x010a_2fa4;
/// The vftable stored in the virtual base subobject (RTTI class
/// `basic_ostream<char,std::char_traits<char>>`).
const VFTABLE_010A318C: u32 = 0x010a_318c;

// Translated from 00ec3a75 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3a75 - the compiler's exception frame (`__EH_prolog3` at
// `00ece255`, `__EH_epilog3` at `00ece2be`: FS:[0] chain) is not translated.
/// Unnamed in the engine map: the shape of a `std::basic_ostream` constructor with a virtual
/// base. When `most_derived` is non-zero it stores the vbtables `010a3190` at `+0` and
/// `010a2fa4` at `+4`; it then reads the virtual-base offset from `[[this] + 4]`, stores the
/// vftable `010a318c` at `this + offset` and calls `00ad0470(this + offset, first, second)`.
/// Returns `this`.
pub fn fn_00ec3a75(e: &mut Engine, this: Ptr, first: u32, second: u32, most_derived: u32) -> Ptr {
    let base = this.addr();
    if most_derived != 0 {
        e.mem.set_u32(base, VBTABLE_010A3190);
        e.mem.set_u32(base + 4, VBTABLE_010A2FA4);
    }
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.mem.set_u32(base.wrapping_add(offset), VFTABLE_010A318C);
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.call(
        STREAM_BASE_INIT_00AD0470,
        &args![base.wrapping_add(offset), first, second],
    );
    this
}

/// `operator new` (`00401000`, cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `std::_Lockit::_Lockit` (`00ec32e3`, thiscall, one word: the lock kind) and its destructor
/// (`00ec330b`, thiscall).
const LOCKIT_CTOR: u32 = 0x00ec_32e3;
const LOCKIT_DTOR: u32 = 0x00ec_330b;
/// `std::locale::_Locimp::_Locimp(bool)` (`00ec35ff`, thiscall, one word).
const LOCIMP_CTOR: u32 = 0x00ec_35ff;
/// `std::locale::_Setgloballocale` (`00ec34da`, cdecl, one word).
const SET_GLOBAL_LOCALE: u32 = 0x00ec_34da;
/// `0044a640` (thiscall, one word: a C string; Ghidra's library match is a
/// `basic_string::assign`, a name it shares with other bodies, so unconfirmed).
const STRING_ASSIGN_0044A640: u32 = 0x0044_a640;
/// `std::locale::facet::_Incref` (`00ace790`, thiscall; Ghidra library match).
const FACET_INCREF: u32 = 0x00ac_e790;
/// The global locale implementation (`.data`), the classic one, and its second copy.
const GLOBAL_LOCALE_0126FB98: u32 = 0x0126_fb98;
const CLASSIC_LOCALE_0126FB9C: u32 = 0x0126_fb9c;
const CLASSIC_LOCALE_0126FBB4: u32 = 0x0126_fbb4;
/// The C string `_Init` gives the new `_Locimp`'s name (`.rdata`).
const NAME_01063D10: u32 = 0x0106_3d10;

// Translated from 00ec36a6 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec36a6 - the compiler's exception frame (`__EH_prolog3` at 00ece255, `__EH_epilog3` at 00ece2be: FS:[0] chain) is not translated
/// `std::locale::_Init` (Ghidra library match; cdecl, no arguments): returns the global
/// `_Locimp` at `0126fb98` when there is one. Otherwise, under a `_Lockit(0)` (4 bytes on the
/// stack), checks again and, when still none, allocates 0x34 bytes, constructs a
/// `_Locimp(false)` in them (0 when the allocation failed), makes it the global locale, sets
/// its word at +0x10 to `0x3f`, assigns the name `01063d10` to the string at +0x18, stores it
/// at `0126fb9c`, `_Incref`s it and copies `0126fb9c` to `0126fbb4`. Returns the `_Locimp`.
pub fn locale_init(e: &mut Engine) -> Ptr {
    let current = e.mem.u32(GLOBAL_LOCALE_0126FB98);
    if current != 0 {
        return Ptr::new(current);
    }
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let mut locimp = e.mem.u32(GLOBAL_LOCALE_0126FB98);
        if locimp == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
            locimp = if block != 0 {
                e.call(LOCIMP_CTOR, &args![block, 0u32]).u32()
            } else {
                0
            };
            e.call(SET_GLOBAL_LOCALE, &args![locimp]);
            e.mem.set_u32(locimp.wrapping_add(0x10), 0x3f);
            e.call(
                STRING_ASSIGN_0044A640,
                &args![locimp.wrapping_add(0x18), NAME_01063D10],
            );
            e.mem.set_u32(CLASSIC_LOCALE_0126FB9C, locimp);
            e.call(FACET_INCREF, &args![locimp]);
            let classic = e.mem.u32(CLASSIC_LOCALE_0126FB9C);
            e.mem.set_u32(CLASSIC_LOCALE_0126FBB4, classic);
        }
        e.call(LOCKIT_DTOR, &args![lock]);
        Ptr::new(locimp)
    })
}

/// `_setlocale` (`00ed036e`, cdecl: category, locale string).
const SETLOCALE: u32 = 0x00ed_036e;
/// The empty C string (`.rdata`) and `"*"` (`.rdata`), the defaults `_Locinfo_ctor` assigns.
const EMPTY_01011584: u32 = 0x0101_1584;
const STAR_0101FFBC: u32 = 0x0101_ffbc;

// Translated from 00ec3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Locinfo::_Locinfo_ctor(_Locinfo *, const std::string &)` (Ghidra library match;
/// cdecl, two words). The name's characters are at `+4` of the string when its capacity
/// (`+0x18`) is below 0x10, else at the pointer at `+4`. Assigns `setlocale(0, NULL)` (or `""`
/// when that is null) to the string at `+0x3c`, then `setlocale(0, name)` (or `"*"` when the
/// name pointer or the result is null) to the string at `+0x58`.
pub fn locinfo_ctor(e: &mut Engine, locinfo: Ptr, name: Ptr) {
    let name = name.addr();
    let characters = if e.mem.u32(name + 0x18) >= 0x10 {
        e.mem.u32(name + 4)
    } else {
        name + 4
    };
    let mut current = e.call(SETLOCALE, &args![0u32, 0u32]).u32();
    if current == 0 {
        current = EMPTY_01011584;
    }
    let locinfo = locinfo.addr();
    e.call(STRING_ASSIGN_0044A640, &args![locinfo + 0x3c, current]);
    let mut chosen = STAR_0101FFBC;
    if characters != 0 {
        let result = e.call(SETLOCALE, &args![0u32, characters]).u32();
        if result != 0 {
            chosen = result;
        }
    }
    e.call(STRING_ASSIGN_0044A640, &args![locinfo + 0x58, chosen]);
}

/// `____lc_handle_func` (`00ed0cb9`), `____lc_codepage_func` (`00ed0c93`), `__calloc_crt`
/// (`00ed0d24`, cdecl: count, size), `___pctype_func` (`00ed08c8`) and `_memcpy` (`00ec44d0`).
const LC_HANDLE_FUNC: u32 = 0x00ed_0cb9;
const LC_CODEPAGE_FUNC: u32 = 0x00ed_0c93;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const PCTYPE_FUNC: u32 = 0x00ed_08c8;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00ec38a9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Getctype` (Ghidra library match; cdecl, one word: the `_Ctypevec` it fills and returns).
/// `+0` is the word at `+4` of `____lc_handle_func()`'s result, `+4` the code page. It
/// allocates 0x100 entries of 2 bytes (`__calloc_crt`); when that succeeds it copies the 0x200
/// bytes of `___pctype_func()` into them, keeps them at `+8` and sets `+0xc` (delete it) to 1;
/// otherwise `+0xc` is 0 and `+8` the `___pctype_func()` table itself.
pub fn getctype(e: &mut Engine, result: Ptr) -> Ptr {
    let vector = result.addr();
    let handles = e.call(LC_HANDLE_FUNC, &args![]).u32();
    let handle = e.mem.u32(handles + 4);
    e.mem.set_u32(vector, handle);
    let codepage = e.call(LC_CODEPAGE_FUNC, &args![]).u32();
    e.mem.set_u32(vector + 4, codepage);
    let table = e.call(CALLOC_CRT, &args![0x100u32, 2u32]).u32();
    e.mem.set_u32(vector + 8, table);
    if table != 0 {
        let source = e.call(PCTYPE_FUNC, &args![]).u32();
        let destination = e.mem.u32(vector + 8);
        e.call(MEMCPY, &args![destination, source, 0x200u32]);
        e.mem.set_u32(vector + 0xc, 1);
    } else {
        let source = e.call(PCTYPE_FUNC, &args![]).u32();
        e.mem.set_u32(vector + 0xc, 0);
        e.mem.set_u32(vector + 8, source);
    }
    result
}

/// The stream object at `0126fc10` (its vbtable pointer is its first word) and the words
/// `00ec3a1b` reads and writes beside it (`.data`).
const STREAM_0126FC10: u32 = 0x0126_fc10;
const SOURCE_0126FC50: u32 = 0x0126_fc50;
const STREAM_POINTER_0126FC54: u32 = 0x0126_fc54;
const FIELD_BASE_0126FC3C: u32 = 0x0126_fc3c;

// Translated from 00ec3a1b (disassembled; Ghidra has no function there, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (reached by the static initializer at `00fc791c`: `MOV ECX,0126fc48;
/// JMP 00ec3a1b`); thiscall, no stack words. Stores `0126fc10` at `0126fc54`, then with `off`
/// the virtual-base offset at `+4` of the vbtable `[0126fc10]` (read each time): stores the
/// word at `0126fc50` at `0126fc3c + off` and sets bit 1 of the word at `0126fc10 + off +
/// 0x10`. Returns `this` (`MOV EAX,ECX`).
pub fn fn_00ec3a1b(e: &mut Engine, this: u32) -> u32 {
    let value = e.mem.u32(SOURCE_0126FC50);
    let vbtable = e.mem.u32(STREAM_0126FC10);
    e.mem.set_u32(STREAM_POINTER_0126FC54, STREAM_0126FC10);
    let offset = e.mem.u32(vbtable + 4);
    e.mem
        .set_u32(offset.wrapping_add(FIELD_BASE_0126FC3C), value);
    let vbtable = e.mem.u32(STREAM_0126FC10);
    let offset = e.mem.u32(vbtable + 4);
    let state = offset.wrapping_add(STREAM_0126FC10).wrapping_add(0x10);
    let flags = e.mem.u32(state);
    e.mem.set_u32(state, flags | 2);
    this
}

/// `00ad0630` (thiscall: the `basic_streambuf` part's constructor, unnamed in the map) and
/// `00acfc10` (thiscall, two words: the file and a flag; unnamed in the map).
const STREAMBUF_CTOR_00AD0630: u32 = 0x00ad_0630;
const FILEBUF_INIT_00ACFC10: u32 = 0x00ac_fc10;
/// The vftable `00ec3a4f` stores (RTTI `basic_filebuf<char,std::char_traits<char>>`).
const VFTABLE_010A3138: u32 = 0x010a_3138;

// Translated from 00ec3a4f (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the shape of a `basic_filebuf<char>` constructor (thiscall, one word,
/// `RET 4`). Calls `00ad0630(this)`, stores the vftable `010a3138` at `+0`, then calls
/// `00acfc10(this, file, 0)`. Returns `this`.
pub fn fn_00ec3a4f(e: &mut Engine, this: Ptr, file: u32) -> Ptr {
    e.call(STREAMBUF_CTOR_00AD0630, &args![this]);
    e.mem.set_u32(this.addr(), VFTABLE_010A3138);
    e.call(FILEBUF_INIT_00ACFC10, &args![this, file, 0u32]);
    this
}

/// KERNEL32 import slots (one word each): `InitializeCriticalSection` (`00fdf054`),
/// `DeleteCriticalSection` (`00fdf058`), `EnterCriticalSection` (`00fdf05c`),
/// `LeaveCriticalSection` (`00fdf100`), `InterlockedExchange` (`00fdf0e4`, two words).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const INTERLOCKED_EXCHANGE: u32 = 0x00fd_f0e4;

// Translated from 00ec3acf (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxinit` (Ghidra library match; cdecl, one word): `InitializeCriticalSection(mutex)`.
pub fn mtxinit(e: &mut Engine, mutex: u32) {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3adf (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `__Mtxdst` shape; cdecl, one word): `DeleteCriticalSection(mutex)`.
pub fn fn_00ec3adf(e: &mut Engine, mutex: u32) {
    e.call(DELETE_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3aef (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxlock` (Ghidra library match; cdecl, one word): `EnterCriticalSection(mutex)`.
pub fn mtxlock(e: &mut Engine, mutex: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3aff (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxunlock` (Ghidra library match; cdecl, one word): `LeaveCriticalSection(mutex)`.
pub fn mtxunlock(e: &mut Engine, mutex: u32) {
    e.call(LEAVE_CRITICAL_SECTION, &args![mutex]);
}

/// `_abort` (`00ecf335`).
const ABORT: u32 = 0x00ec_f335;
/// The number of free slots of `_Atexit`'s table (`.data`, 10 at start) and the table (10
/// function pointers, filled from the end).
const ATEXIT_FREE_011C165C: u32 = 0x011c_165c;
const ATEXIT_TABLE_0126FC6C: u32 = 0x0126_fc6c;

// Translated from 00ec3b0f (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Atexit` (Ghidra library match `_Atexit`; cdecl, one word): with no free slot left
/// it ends in `_abort` (a tail jump); otherwise it takes the next free slot from the end of the
/// table at `0126fc6c` and stores the function there.
pub fn std_atexit(e: &mut Engine, function: u32) {
    let free = e.mem.u32(ATEXIT_FREE_011C165C);
    if free == 0 {
        e.call(ABORT, &args![]);
        return;
    }
    let free = free - 1;
    e.mem.set_u32(ATEXIT_FREE_011C165C, free);
    e.mem.set_u32(ATEXIT_TABLE_0126FC6C + free * 4, function);
}

// Translated from 00ec3b35 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00ec3b4f` (the `atexit` destructor row at `00fde581` jumps here with `ECX`
/// `0126fc95`, which neither reads).
pub fn fn_00ec3b35(e: &mut Engine) {
    fn_00ec3b4f(e);
}

// Translated from 00ec3b4f (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): runs the functions `std::_Atexit` registered, from the most recent:
/// while the free count at `011c165c` is below 10, takes the entry at that index, increments
/// the count and calls the entry when it is not null.
pub fn fn_00ec3b4f(e: &mut Engine) {
    loop {
        let index = e.mem.u32(ATEXIT_FREE_011C165C);
        if index >= 10 {
            return;
        }
        let function = e.mem.u32(ATEXIT_TABLE_0126FC6C + index * 4);
        e.mem.set_u32(ATEXIT_FREE_011C165C, index + 1);
        if function != 0 {
            e.call(function, &args![]);
        }
    }
}

/// `00ecf80b` (cdecl, one word; unnamed in the map) and `_free` (`00ecd291`).
const RELEASE_00ECF80B: u32 = 0x00ec_f80b;
const FREE: u32 = 0x00ec_d291;
/// The list heads `00ec3b5a` empties: one word each from `0126fc98` to before `0126fd90`.
const LISTS_0126FC98: u32 = 0x0126_fc98;
const LISTS_END_0126FD90: u32 = 0x0126_fd90;

// Translated from 00ec3b5a (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; `00ec3b9a` gives it to `_atexit`): for each list head from
/// `0126fc98` to before `0126fd90`, takes the list with `InterlockedExchange(head, 0)` and, for
/// each node (next at `+0`), calls `00ecf80b` on the node's word at `+8` and frees the node.
pub fn fn_00ec3b5a(e: &mut Engine) {
    let mut head = LISTS_0126FC98;
    loop {
        let mut node = e.call(INTERLOCKED_EXCHANGE, &args![head, 0u32]).u32();
        while node != 0 {
            let next = e.mem.u32(node);
            let word = e.mem.u32(node + 8);
            e.call(RELEASE_00ECF80B, &args![word]);
            e.call(FREE, &args![node]);
            node = next;
        }
        head += 4;
        if (head as i32) >= LISTS_END_0126FD90 as i32 {
            return;
        }
    }
}

/// `_atexit` (`00ec658f`, cdecl, one word).
const ATEXIT: u32 = 0x00ec_658f;

// Translated from 00ec3b9a (disassembled; Ghidra has no function there, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (a C initializer of `_initterm_e`'s table, entry `010113e0`; no
/// arguments): `_atexit(00ec3b5a)`, then 0 when that returned 0 and `0x18` otherwise (`NEG;
/// SBB; AND 0x18`).
pub fn fn_00ec3b9a(e: &mut Engine) -> u32 {
    let result = e.call(ATEXIT, &args![0x00ec_3b5au32]).u32();
    if result != 0 {
        0x18
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec408c, security_check_cookie(u32)),
        entry!(0x00ec3bda, fn_00ec3bda(u32) -> u32),
        entry!(0x00ec_3a75, fn_00ec3a75(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ec_36a6, locale_init() -> Ptr),
        entry!(0x00ec_3740, locinfo_ctor(Ptr, Ptr)),
        entry!(0x00ec_38a9, getctype(Ptr) -> Ptr),
        entry!(0x00ec_3a1b, fn_00ec3a1b(u32) -> u32),
        entry!(0x00ec_3a4f, fn_00ec3a4f(Ptr, u32) -> Ptr),
        entry!(0x00ec_3acf, mtxinit(u32)),
        entry!(0x00ec_3adf, fn_00ec3adf(u32)),
        entry!(0x00ec_3aef, mtxlock(u32)),
        entry!(0x00ec_3aff, mtxunlock(u32)),
        entry!(0x00ec_3b0f, std_atexit(u32)),
        entry!(0x00ec_3b35, fn_00ec3b35()),
        entry!(0x00ec_3b4f, fn_00ec3b4f()),
        entry!(0x00ec_3b5a, fn_00ec3b5a()),
        entry!(0x00ec_3b9a, fn_00ec3b9a() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_low_half_and_keeps_the_shifted_high_half() {
        let mut e = Engine::new();
        let mut swap = |value: u32| e.call(0x00ec_3bda, &args![value]).u32();
        assert_eq!(swap(0x0000_1234), 0x0012_3412);
        assert_eq!(swap(0x0000_ff01), 0x00ff_01ff);
        assert_eq!(swap(0xabcd_1234), 0xcd12_3412);
        assert_eq!(swap(0), 0);
        assert_eq!(swap(0x0000_00ff) as u16, 0xff00);
    }

    #[test]
    fn security_check_cookie_returns_on_a_match_and_reports_otherwise() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xdead_beefu32);
        e.register_double(REPORT_GS_FAILURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beefu32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beeeu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == REPORT_GS_FAILURE)
                .count(),
            1
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vbtables_and_initialises_the_virtual_base() {
        let mut e = Engine::new();
        e.put_vtable(VBTABLE_010A3190, &[0, 0x10]);
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3a75, &args![this, 0x11u32, 0x22u32, 1u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VBTABLE_010A3190);
        assert_eq!(e.mem.u32(this + 4), VBTABLE_010A2FA4);
        assert_eq!(e.mem.u32(this + 0x10), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x10, 0x11, 0x22]]
        );
    }

    #[test]
    fn keeps_the_existing_vbtable_when_not_most_derived() {
        let mut e = Engine::new();
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x18);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vbtable);
        e.mem.set_u32(this + 4, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3a75, &args![this, 1u32, 2u32, 0u32]);
        assert_eq!(e.mem.u32(this), vbtable);
        assert_eq!(e.mem.u32(this + 4), 0x77);
        assert_eq!(e.mem.u32(this + 0x18), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x18, 1, 2]]
        );
    }
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn data_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e
    }

    #[test]
    fn locale_init_returns_the_existing_global_locale() {
        let mut e = data_engine();
        e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), 0x4444);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn locale_init_builds_the_classic_locale_under_the_lock() {
        let mut e = data_engine();
        let block = e.mem.alloc(0x34);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(LOCKIT_CTOR, |_, a| a[0].into_ret());
        e.register(LOCKIT_DTOR, |_, _| Ret::default());
        e.register(LOCIMP_CTOR, |_, a| a[0].into_ret());
        e.register(SET_GLOBAL_LOCALE, |_, _| Ret::default());
        e.register(STRING_ASSIGN_0044A640, |_, a| a[0].into_ret());
        e.register(FACET_INCREF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), block);
        let lock = calls_to(&e, LOCKIT_CTOR);
        assert_eq!(lock.len(), 1);
        assert_eq!(lock[0][1], 0);
        assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![lock[0][0]]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, LOCIMP_CTOR), vec![vec![block, 0]]);
        assert_eq!(calls_to(&e, SET_GLOBAL_LOCALE), vec![vec![block]]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![vec![block + 0x18, NAME_01063D10]]
        );
        assert_eq!(calls_to(&e, FACET_INCREF), vec![vec![block]]);
        assert_eq!(e.mem.u32(block + 0x10), 0x3f);
        assert_eq!(e.mem.u32(CLASSIC_LOCALE_0126FB9C), block);
        assert_eq!(e.mem.u32(CLASSIC_LOCALE_0126FBB4), block);
    }

    #[test]
    fn locale_init_keeps_a_locale_set_while_waiting_for_the_lock() {
        let mut e = data_engine();
        e.register(LOCKIT_CTOR, |e, a| {
            e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x7777);
            a[0].into_ret()
        });
        e.register(LOCKIT_DTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), 0x7777);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&e, LOCKIT_DTOR).len(), 1);
    }

    #[test]
    fn locinfo_ctor_assigns_the_current_and_the_named_locale() {
        let mut e = Engine::new();
        let locinfo = e.mem.alloc(0x80);
        let short = e.mem.alloc(0x20);
        e.mem.set_u32(short + 0x18, 0xf);
        let long = e.mem.alloc(0x20);
        e.mem.set_u32(long + 0x18, 0x10);
        e.mem.set_u32(long + 4, 0x9000);
        e.register(STRING_ASSIGN_0044A640, |_, a| a[0].into_ret());
        // setlocale(0, NULL) answers 0x1111; setlocale(0, name) answers name + 1.
        e.register(SETLOCALE, |_, a| {
            if a[1] == 0 {
                0x1111u32.into_ret()
            } else {
                (a[1] + 1).into_ret()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, short]);
        assert_eq!(
            calls_to(&e, SETLOCALE),
            vec![vec![0, 0], vec![0, short + 4]]
        );
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![
                vec![locinfo + 0x3c, 0x1111],
                vec![locinfo + 0x58, short + 5]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(calls_to(&e, SETLOCALE)[1], vec![0, 0x9000]);
        // Null results: the empty string and the star.
        e.register(SETLOCALE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![
                vec![locinfo + 0x3c, EMPTY_01011584],
                vec![locinfo + 0x58, STAR_0101FFBC]
            ]
        );
        // A null name pointer: the star without a second setlocale.
        e.mem.set_u32(long + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(calls_to(&e, SETLOCALE).len(), 1);
        assert_eq!(calls_to(&e, STRING_ASSIGN_0044A640)[1][1], STAR_0101FFBC);
    }

    #[test]
    fn getctype_copies_the_table_or_keeps_the_global_one() {
        for allocated in [true, false] {
            let mut e = Engine::new();
            let handles = e.mem.alloc(0x20);
            e.mem.set_u32(handles + 4, 0x0409);
            e.register_double(LC_HANDLE_FUNC, move |_, _| handles.into_ret());
            e.register(LC_CODEPAGE_FUNC, |_, _| 1252u32.into_ret());
            let table = e.mem.alloc(0x200);
            let answer = if allocated { table } else { 0 };
            e.register_double(CALLOC_CRT, move |_, _| answer.into_ret());
            e.register(PCTYPE_FUNC, |_, _| 0x00ab_0000u32.into_ret());
            e.register(MEMCPY, |_, a| a[0].into_ret());
            let vector = e.mem.alloc(0x10);
            e.mem.set_u32(vector + 0xc, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ec_38a9, &args![vector]).u32(), vector);
            assert_eq!(e.mem.u32(vector), 0x0409);
            assert_eq!(e.mem.u32(vector + 4), 1252);
            assert_eq!(calls_to(&e, CALLOC_CRT), vec![vec![0x100, 2]]);
            if allocated {
                assert_eq!(calls_to(&e, MEMCPY), vec![vec![table, 0x00ab_0000, 0x200]]);
                assert_eq!(e.mem.u32(vector + 8), table);
                assert_eq!(e.mem.u32(vector + 0xc), 1);
            } else {
                assert!(calls_to(&e, MEMCPY).is_empty());
                assert_eq!(e.mem.u32(vector + 8), 0x00ab_0000);
                assert_eq!(e.mem.u32(vector + 0xc), 0);
            }
        }
    }

    #[test]
    fn stream_tail_writes_through_the_virtual_base_offset() {
        let mut e = data_engine();
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x8);
        e.mem.set_u32(STREAM_0126FC10, vbtable);
        e.mem.set_u32(SOURCE_0126FC50, 0xabcd);
        e.mem.set_u32(STREAM_0126FC10 + 8 + 0x10, 0x10);
        assert_eq!(
            e.call(0x00ec_3a1b, &args![0x0126_fc48u32]).u32(),
            0x0126_fc48
        );
        assert_eq!(e.mem.u32(STREAM_POINTER_0126FC54), STREAM_0126FC10);
        assert_eq!(e.mem.u32(FIELD_BASE_0126FC3C + 8), 0xabcd);
        assert_eq!(e.mem.u32(STREAM_0126FC10 + 8 + 0x10), 0x12);
    }

    #[test]
    fn filebuf_ctor_sets_the_vftable_between_its_calls() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.register(STREAMBUF_CTOR_00AD0630, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(FILEBUF_INIT_00ACFC10, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VFTABLE_010A3138);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3a4f, &args![this, 0x77u32]).u32(), this);
        assert_eq!(
            calls_to(&e, FILEBUF_INIT_00ACFC10),
            vec![vec![this, 0x77, 0]]
        );
        assert_eq!(calls_to(&e, STREAMBUF_CTOR_00AD0630), vec![vec![this]]);
    }

    #[test]
    fn mutex_wrappers_pass_the_mutex_to_their_import() {
        for (wrapper, import) in [
            (0x00ec_3acfu32, INITIALIZE_CRITICAL_SECTION),
            (0x00ec_3adf, DELETE_CRITICAL_SECTION),
            (0x00ec_3aef, ENTER_CRITICAL_SECTION),
            (0x00ec_3aff, LEAVE_CRITICAL_SECTION),
        ] {
            let mut e = Engine::new();
            e.register(import, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(wrapper, &args![0x0126_fb30u32]);
            assert_eq!(calls_to(&e, import), vec![vec![0x0126_fb30]]);
        }
    }

    #[test]
    fn std_atexit_fills_from_the_end_and_aborts_when_full() {
        let mut e = data_engine();
        e.register(ABORT, |_, _| Ret::default());
        e.mem.set_u32(ATEXIT_FREE_011C165C, 10);
        e.call(0x00ec_3b0f, &args![0xaaaau32]);
        e.call(0x00ec_3b0f, &args![0xbbbbu32]);
        assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 8);
        assert_eq!(e.mem.u32(ATEXIT_TABLE_0126FC6C + 9 * 4), 0xaaaa);
        assert_eq!(e.mem.u32(ATEXIT_TABLE_0126FC6C + 8 * 4), 0xbbbb);
        e.mem.set_u32(ATEXIT_FREE_011C165C, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3b0f, &args![0xccccu32]);
        assert_eq!(calls_to(&e, ABORT).len(), 1);
        assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 0);
    }

    #[test]
    fn atexit_runner_calls_the_entries_from_the_free_index_on() {
        for entry in [0x00ec_3b4fu32, 0x00ec_3b35] {
            let mut e = data_engine();
            e.register(0x0050_0000, |_, _| Ret::default());
            e.register(0x0050_0010, |_, _| Ret::default());
            e.mem.set_u32(ATEXIT_FREE_011C165C, 7);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 7 * 4, 0x0050_0000);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 8 * 4, 0);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 9 * 4, 0x0050_0010);
            e.call_log = Some(vec![]);
            e.call(entry, &args![0x0126_fc95u32]);
            let order: Vec<u32> = e
                .call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .filter(|a| *a == 0x0050_0000 || *a == 0x0050_0010)
                .collect();
            assert_eq!(order, vec![0x0050_0000, 0x0050_0010]);
            assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 10);
        }
    }

    #[test]
    fn list_release_empties_every_head_and_frees_each_node() {
        let mut e = data_engine();
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 8, 0x22);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(LISTS_0126FC98 + 4, first);
        e.register(INTERLOCKED_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], a[1]);
            old.into_ret()
        });
        e.register(RELEASE_00ECF80B, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3b5a, &args![]);
        let exchanges = calls_to(&e, INTERLOCKED_EXCHANGE);
        assert_eq!(exchanges.len(), 62);
        assert_eq!(exchanges[0], vec![LISTS_0126FC98, 0]);
        assert_eq!(exchanges[61][0], LISTS_END_0126FD90 - 4);
        assert_eq!(calls_to(&e, RELEASE_00ECF80B), vec![vec![0x11], vec![0x22]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![first], vec![second]]);
        assert_eq!(e.mem.u32(LISTS_0126FC98 + 4), 0);
    }

    #[test]
    fn initializer_registers_the_list_release_and_maps_the_result() {
        let mut e = Engine::new();
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3b9a, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![0x00ec_3b5a]]);
        e.register(ATEXIT, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ec_3b9a, &args![]).u32(), 0x18);
    }
}
