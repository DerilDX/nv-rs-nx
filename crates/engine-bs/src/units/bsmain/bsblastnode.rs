//! `BSMain/bsblastnode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c45220` (named `BSRangeNode::LoadBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00C45220: u32 = 0x00c4_5220;

// Translated from 00c4ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1f00 of the table whose `CreateClone` is
/// `BSBlastNode::CreateClone` (010c1efc). Forwards `this` and the stream to `00c45220`
/// (`BSRangeNode::LoadBinary`). Returns nothing: Xbox PDB
/// `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4ab40(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00C45220, &args![this, stream]);
}

/// `00c452a0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C452A0: u32 = 0x00c4_52a0;

// Translated from 00c4ab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1f04 of the `BSBlastNode` table, the slot after
/// `LoadBinary` (010c1f00). Forwards `this` and the stream to `00c452a0` (`bsrangenode.obj`,
/// unnamed; its body calls `NiNode::LinkObject` `00a5e9d0`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4ab60(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C452A0, &args![this, stream]);
}

/// `00c452f0` (named `BSRangeNode::SaveBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00C452F0: u32 = 0x00c4_52f0;

// Translated from 00c4abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1f0c of the `BSBlastNode` table, three words
/// after `LoadBinary` (010c1f00). Forwards `this` and the stream to `00c452f0`
/// (`BSRangeNode::SaveBinary`). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4abb0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00C452F0, &args![this, stream]);
}

// Translated from 00c4ac90 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e7c without reading the receiver. The map names this body `BSBlastNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c4ac90(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e7c
}

/// `BSBlastNode`'s vtable (its class confirmed by the RTTI before it), `operator new`
/// (`cdecl`, `(size)`), `BSRangeNode::BSRangeNode` (`thiscall`, no arguments) and
/// `BSRangeNode::CopyMembers` (`thiscall`, `(destination, cloning)`).
const BS_BLAST_NODE_VTABLE: u32 = 0x010c_1eb4;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const RANGE_NODE_CONSTRUCTOR: u32 = 0x00c4_50d0;
const RANGE_NODE_COPY_MEMBERS: u32 = 0x00c4_51b0;

/// A 0xB0-byte block (`operator new`) constructed as a `BSRangeNode` and given
/// `BSBlastNode`'s vtable; null when the allocation fails.
fn new_blast_node(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(RANGE_NODE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, BS_BLAST_NODE_VTABLE);
    Ptr::new(block)
}

// Translated from 00c4ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSBlastNode` factory (unnamed in the map): a 0xB0-byte block constructed as a
/// `BSRangeNode` with `BSBlastNode`'s vtable, or null when the allocation fails.
// Translation incomplete: 00c4ac00 - SEH exception unwinding is unsupported.
pub fn fn_00c4ac00(e: &mut Engine) -> Ptr {
    new_blast_node(e)
}

// Translated from 00c4aae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlastNode`'s `CopyMembers` (unnamed in the map), `(destination, cloning)`:
/// `BSRangeNode::CopyMembers` with the same arguments.
pub fn fn_00c4aae0(e: &mut Engine, this: Ptr, destination: Ptr, cloning: Ptr) {
    e.call(RANGE_NODE_COPY_MEMBERS, &args![this, destination, cloning]);
}

// Translated from 00c4aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlastNode::CreateClone` (Xbox PDB), `(cloning)`: a new blast node (as the factory
/// makes it) and this node's members copied into it (`fn_00c4aae0`, also with a null
/// destination when the allocation failed). Answers the clone.
// Translation incomplete: 00c4aa40 - SEH exception unwinding is unsupported.
pub fn bs_blast_node_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_blast_node(e);
    fn_00c4aae0(e, this, clone, cloning);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_ac90, fn_00c4ac90(Ptr) -> u32),
        entry!(0x00c4_ab40, fn_00c4ab40(Ptr, Ptr)),
        entry!(0x00c4_ab60, fn_00c4ab60(Ptr, Ptr)),
        entry!(0x00c4_abb0, fn_00c4abb0(Ptr, Ptr)),
        entry!(0x00c4ac00, fn_00c4ac00() -> Ptr),
        entry!(0x00c4aae0, fn_00c4aae0(Ptr, Ptr, Ptr)),
        entry!(0x00c4aa40, bs_blast_node_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c4ab40_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00C45220, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_ab40, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00C45220)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4ab60_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C452A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_ab60, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C452A0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4abb0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00C452F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_abb0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00C452F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c4ac90_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_ac90, &args![0x1000u32]).u32(), 0x0120_2e7c);
        assert_eq!(e.call(0x00c4_ac90, &args![0u32]).u32(), 0x0120_2e7c);
    }

    #[test]
    fn blast_node_factory_and_clone() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(RANGE_NODE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(RANGE_NODE_COPY_MEMBERS, |_, _| Ret::default());
        let made = e.call(0x00c4ac00, &args![]).u32();
        assert_eq!(e.mem.u32(made), BS_BLAST_NODE_VTABLE);
        let clone = e.call(0x00c4aa40, &args![0x5000u32, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), BS_BLAST_NODE_VTABLE);
        let log = e.call_log.take().unwrap();
        let copies: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == RANGE_NODE_COPY_MEMBERS)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(copies, vec![vec![0x5000, clone, 0x6000]]);
        let constructed = log.iter().filter(|c| c.0 == RANGE_NODE_CONSTRUCTOR).count();
        assert_eq!(constructed, 2);
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c4ac00, &args![]).u32(), 0);
        assert_eq!(e.call(0x00c4aa40, &args![0x5000u32, 0x6000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != RANGE_NODE_CONSTRUCTOR));
        assert!(log
            .iter()
            .any(|c| c.0 == RANGE_NODE_COPY_MEMBERS && c.1 == vec![0x5000, 0, 0x6000]));
    }

    #[test]
    fn blast_node_copy_members_forwards_to_the_range_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(RANGE_NODE_COPY_MEMBERS, |_, _| Ret::default());
        e.call(0x00c4aae0, &args![1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (RANGE_NODE_COPY_MEMBERS, vec![1, 2, 3]));
    }
}
