//! `BSMain/bsdecalpvextradata.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c644b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSDecalPlacementVectorExtraData` vtable word 010c38fc,
/// right after its `LoadBinary` (010c38f8). Forwards `this` and the stream to `00a9af00`, a
/// one-instruction jump to `NiExtraData::LinkObject` (`00a7b3b0`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c644b0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

// Translated from 00c64600 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043d4 without reading the receiver. The map names this body `BSDecalPlacementVectorExtraData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c64600(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43d4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_4600, fn_00c64600(Ptr) -> u32),
        entry!(0x00c6_44b0, fn_00c644b0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c644b0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_44b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c64600_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_4600, &args![0x1000u32]).u32(), 0x0120_43d4);
        assert_eq!(e.call(0x00c6_4600, &args![0u32]).u32(), 0x0120_43d4);
    }
}
