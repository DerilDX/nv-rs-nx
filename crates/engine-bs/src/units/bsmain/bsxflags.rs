//! `BSMain/bsxflags.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegerExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTEGER_EXTRA_DATA_RTTI: u32 = 0x011f_5f2c;

// Translated from 00a99c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_integer_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTEGER_EXTRA_DATA_RTTI)
}

/// `00a99d30` (named `NiFloatExtraData::LoadBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00A99D30: u32 = 0x00a9_9d30;

// Translated from 00c430d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::LoadBinary` (engine map vtable name, vtable word 010c18a0): forwards `this` and
/// the stream to `00a99d30` (named `NiFloatExtraData::LoadBinary`, a body folded with the
/// integer extra data's). Returns nothing: Xbox PDB
/// `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn bsx_flags_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00A99D30, &args![this, stream]);
}

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c430f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSXFlags` vtable word 010c18a4, between `LoadBinary`
/// (010c18a0) and the word 010c18a8. Forwards `this` and the stream to `00a9af00`, a
/// one-instruction jump to `NiExtraData::LinkObject` (`00a7b3b0`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c430f0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

/// `00a9a4e0` (named `NiFloatExtraData::SaveBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00A9A4E0: u32 = 0x00a9_a4e0;

// Translated from 00c43140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::SaveBinary` (engine map vtable name, vtable word 010c18ac): forwards `this` and
/// the stream to `00a9a4e0` (named `NiFloatExtraData::SaveBinary`, a body folded with the
/// integer extra data's). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn bsx_flags_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00A9A4E0, &args![this, stream]);
}

// Translated from 00c43650 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202de0 without reading the receiver. The map names this body `BSXFlags::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c43650(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2de0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3650, fn_00c43650(Ptr) -> u32),
        entry!(0x00a99c60, ni_integer_extra_data_get_rtti() -> Ptr),
        entry!(0x00c4_30d0, bsx_flags_load_binary(Ptr, Ptr)),
        entry!(0x00c4_30f0, fn_00c430f0(Ptr, Ptr)),
        entry!(0x00c4_3140, bsx_flags_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_integer_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a99c60, &args![]).u32(),
            NI_INTEGER_EXTRA_DATA_RTTI
        );
    }

    #[test]
    fn forwarder_bsx_flags_load_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00A99D30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_30d0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00A99D30)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c430f0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_30f0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bsx_flags_save_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00A9A4E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3140, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00A9A4E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c43650_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_3650, &args![0x1000u32]).u32(), 0x0120_2de0);
        assert_eq!(e.call(0x00c4_3650, &args![0u32]).u32(), 0x0120_2de0);
    }
}
