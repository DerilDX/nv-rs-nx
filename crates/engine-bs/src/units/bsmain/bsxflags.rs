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

/// `NiObjectNET::GetExtraData` (Xbox PDB, `00a5bdd0`): `thiscall`, `RET 4`; the extra data
/// whose name the given pointer (to a `NiFixedString`) names, or null.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The global holding the pointer to the `NiFixedString` that names the `BSXFlags` extra data
/// (unconfirmed: set by a static initializer).
const BSX_FLAGS_NAME: u32 = 0x0120_2ddc;
/// `BSXFlags::ms_RTTI`, the address `BSXFlags::GetRTTI` (`00c43650`) answers.
const BSX_FLAGS_RTTI: u32 = 0x0120_2de0;
/// The virtual slot (byte offset) that answers the object's `NiRTTI`.
const SLOT_GET_RTTI: u32 = 0x08;

// Translated from 00c43490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsxflags.obj` range). `cdecl`, one stack word. For a non-null
/// object, looks up its extra data under the name the global at `01202ddc` points to
/// (`NiObjectNET::GetExtraData`) and answers it when it is a `BSXFlags` (its virtual slot +8
/// answers the `NiRTTI` at `01202de0`); null otherwise.
pub fn fn_00c43490(e: &mut Engine, object: Ptr) -> Ptr {
    if object.is_null() {
        return Ptr::NULL;
    }
    let name = e.global::<u32>(BSX_FLAGS_NAME);
    let data = e.call(GET_EXTRA_DATA, &args![object, name]).u32();
    if data != 0 && e.vcall(data, SLOT_GET_RTTI, &args![]).u32() == BSX_FLAGS_RTTI {
        Ptr::new(data)
    } else {
        Ptr::NULL
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3490, fn_00c43490(Ptr) -> Ptr),
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

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig(rtti_of_data: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(BSX_FLAGS_NAME, 4);
        e.set_global(BSX_FLAGS_NAME, 0x0120_3000);
        let mut slots = [0u32; 4];
        slots[2] = 0x0f00_c000;
        e.put_vtable(0x0f00_c100, &slots);
        e.register_double(0x0f00_c000, move |_, _| rtti_of_data.into_ret());
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x0f00_c100);
        e.register_double(GET_EXTRA_DATA, move |_, _| data.into_ret());
        (e, data)
    }

    #[test]
    fn at_00c43490_answers_the_extra_data_when_it_is_a_bsx_flags() {
        let (mut e, data) = rig(BSX_FLAGS_RTTI);
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), data);
        let lookups: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GET_EXTRA_DATA)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(lookups, vec![vec![0x4000, 0x0120_3000]]);
    }

    #[test]
    fn at_00c43490_answers_null_for_other_extra_data_nothing_or_a_null_object() {
        let (mut e, _) = rig(0x0120_2de4);
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), 0);
        e.register(GET_EXTRA_DATA, |_, _| Ret::default());
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3490, &args![0u32]).u32(), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 != GET_EXTRA_DATA));
    }
}
