//! `BSMain/bsbound.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c54e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSBound` vtable word 010c2bb4, right after
/// `BSBound::LoadBinary` (010c2bb0). Forwards `this` and the stream to `00a7b3b0`
/// (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c54e70(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

// Translated from 00c55320 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031e8 without reading the receiver. The map names this body `BSBound::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c55320(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31e8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_5320, fn_00c55320(Ptr) -> u32),
        entry!(0x00c5_4e70, fn_00c54e70(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c54e70_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4e70, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c55320_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_5320, &args![0x1000u32]).u32(), 0x0120_31e8);
        assert_eq!(e.call(0x00c5_5320, &args![0u32]).u32(), 0x0120_31e8);
    }
}
