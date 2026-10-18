//! `NiAnimation/nicolorextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColorExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_3fdc;

// Translated from 00a4fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_color_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI)
}

/// `00a57100`: the jump target of `00a4eb00`.
const JUMP_TARGET_00A4EB00: u32 = 0x00a5_7100;

/// `00a571e0`: the jump target of `00a4faf0`.
const JUMP_TARGET_00A4FAF0: u32 = 0x00a5_71e0;

// Translated from 00a4eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57100` (engine map: `NiExtraDataController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiColorExtraDataController@@UAA_NAAVNiStream@@@Z`.
pub fn ni_color_extra_data_controller_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A4EB00, &args![this, stream]).bool()
}

// Translated from 00a4faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::IsEqual` (Xbox PDB): a 5-byte jump to `00a571e0` (engine map: `NiExtraDataController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiColorExtraDataController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_color_extra_data_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A4FAF0, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4fb00, ni_color_extra_data_controller_get_rtti() -> Ptr),
        entry!(0x00a4_eb00, ni_color_extra_data_controller_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a4_faf0, ni_color_extra_data_controller_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_color_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4fb00, &args![]).u32(),
            NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4eb00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_eb00;
        const TARGET: u32 = 0x00a5_7100;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a4faf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_faf0;
        const TARGET: u32 = 0x00a5_71e0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
