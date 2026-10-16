//! `BSMain/run_00c612b0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a99a90` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00A99A90: u32 = 0x00a9_9a90;

// Translated from 00c61450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35c8, the word before 010c35cc (this class's
/// table has no named neighbours; position by the object order of the neighbouring tables).
/// Forwards `this` and the stream to `00a99a90`, an extra-data `LoadBinary` body. Returns
/// nothing: Xbox PDB `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c61450(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00A99A90, &args![this, stream]);
}

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c61470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35cc, after the word 010c35c8. Forwards `this`
/// and the stream to `00a9af00`, a one-instruction jump to `NiExtraData::LinkObject`
/// (`00a7b3b0`). Returns nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is
/// void; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn fn_00c61470(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

/// `00a89210` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00A89210: u32 = 0x00a8_9210;

// Translated from 00c614c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35d4, three words after 010c35c8 (the position
/// of `SaveBinary` in the neighbouring tables). Forwards `this` and the stream to `00a89210`
/// (`nifloatsextradata.cpp` in the engine map). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c614c0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00A89210, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_1450, fn_00c61450(Ptr, Ptr)),
        entry!(0x00c6_1470, fn_00c61470(Ptr, Ptr)),
        entry!(0x00c6_14c0, fn_00c614c0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c61450_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00A99A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1450, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00A99A90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c61470_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1470, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c614c0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00A89210, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_14c0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00A89210)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
