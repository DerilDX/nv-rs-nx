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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_ac90, fn_00c4ac90(Ptr) -> u32),
        entry!(0x00c4_ab40, fn_00c4ab40(Ptr, Ptr)),
        entry!(0x00c4_ab60, fn_00c4ab60(Ptr, Ptr)),
        entry!(0x00c4_abb0, fn_00c4abb0(Ptr, Ptr)),
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
}
