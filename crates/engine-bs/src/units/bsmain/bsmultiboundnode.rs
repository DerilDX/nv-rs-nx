//! `BSMain/bsmultiboundnode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c47610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c46af0` and frees 0xb4 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_multi_bound_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c4_6af0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb4u32]);
    }
    this
}

// Translated from 00c47650 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e74 without reading the receiver. The map names this body `BSMultiBoundNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c47650(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e74
}

/// `BSNiNode::BSNiNode` (engine map, `00c5f570`, `thiscall`, no stack words).
const BS_NI_NODE_CONSTRUCT: u32 = 0x00c5_f570;
/// `BSMultiBoundNode`'s vtable (`010c1d14`, RTTI `.?AVBSMultiBoundNode@@`).
const BS_MULTI_BOUND_NODE_VTABLE: u32 = 0x010c_1d14;
/// The `NiPointer` assignment `0066b0d0` (`thiscall`, the new pointer).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00c46970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::BSMultiBoundNode` (Xbox PDB), `thiscall`, no stack words; answers
/// `this`. Runs `BSNiNode`'s constructor, sets the vtable, constructs the `NiPointer` at
/// `+0xac` as null (the inlined constructor's increment of a non-null pointer reads the word
/// just written, so it never runs), assigns null to it (`0066b0d0`) and sets flag 0x800 of
/// `NiAVObject::m_uFlags` (+0x30). The flag is set through a test of the constant 1; the
/// other arm (clearing the flag) never runs and is translated with it.
// Translation incomplete: 00c46970 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
pub fn bs_multi_bound_node_bs_multi_bound_node(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_NI_NODE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BS_MULTI_BOUND_NODE_VTABLE);
    let pointer = base.wrapping_add(0xac);
    e.mem.set_u32(pointer, 0);
    let held = e.mem.u32(pointer);
    if held != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    }
    e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
    // `MOV ECX,1; TEST ECX,ECX`: the set arm always runs.
    let set_flag = std::hint::black_box(true);
    let flags = e.mem.u32(base.wrapping_add(0x30));
    if set_flag {
        e.mem.set_u32(base.wrapping_add(0x30), flags | 0x800);
    } else {
        e.mem.set_u32(base.wrapping_add(0x30), flags & !0x800);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_6970, bs_multi_bound_node_bs_multi_bound_node(Ptr) -> Ptr),
        entry!(0x00c4_7650, fn_00c47650(Ptr) -> u32),
        entry!(0x00c4_7610, bs_multi_bound_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_multi_bound_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c4_7610, 0x00c4_6af0, 0xb4);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c47650_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_7650, &args![0x1000u32]).u32(), 0x0120_2e74);
        assert_eq!(e.call(0x00c4_7650, &args![0u32]).u32(), 0x0120_2e74);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtable_the_null_pointer_and_flag_0x800() {
        let mut e = Engine::new();
        e.register(BS_NI_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x1);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this + 0xac, 0xdddd_dddd);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_6970, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00c4_6970, BS_NI_NODE_CONSTRUCT, NI_POINTER_ASSIGN]
        );
        let assign = log.iter().find(|c| c.0 == NI_POINTER_ASSIGN).unwrap();
        assert_eq!(assign.1, vec![this + 0xac, 0]);
        assert_eq!(e.mem.u32(this), BS_MULTI_BOUND_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0x801);
    }
}
