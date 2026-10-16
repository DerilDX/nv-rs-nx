//! `NiParticle/nipsysairfieldairfrictionctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a4fbd0`: the jump target of `00c29e80`.
const JUMP_TARGET_00C29E80: u32 = 0x00a4_fbd0;

/// `00a44cd0`: the jump target of `00c29f80`.
const JUMP_TARGET_00C29F80: u32 = 0x00a4_4cd0;

/// `00c32df0`: the jump target of `00c2a190`.
const JUMP_TARGET_00C2A190: u32 = 0x00c3_2df0;

/// `00c32e10`: the jump target of `00c2b450`.
const JUMP_TARGET_00C2B450: u32 = 0x00c3_2e10;

/// `00a41220`: the jump target of `00c2b8a0`.
const JUMP_TARGET_00C2B8A0: u32 = 0x00a4_1220;

// Translated from 00c29e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::LoadBinary` (Xbox PDB): a 5-byte jump to `00a4fbd0` (engine map: `NiPSysModifierBoolCtlr::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C29E80, &args![this, stream]);
}

// Translated from 00c29f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a44cd0` (engine map: `NiPSysModifierBoolCtlr::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysAirFieldAirFrictionCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C29F80, &args![this, stream]).bool()
}

// Translated from 00c2a190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00c32df0` (engine map: `NiPSysModifierBoolCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysAirFieldAirFrictionCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C2A190, &args![this, other]).bool()
}

// Translated from 00c2b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00c32e10` (engine map: `NiPSysModifierFloatCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2B450, &args![this, stream]);
}

// Translated from 00c2b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a41220` (engine map: `NiPSysModifierBoolCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2B8A0, &args![this, stream]);
}

// Translated from 00c307c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27b8.
pub fn ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27b8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c307c0, ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_9e80,
            ni_p_sys_air_field_air_friction_ctlr_load_binary(Ptr, Ptr)
        ),
        entry!(0x00c2_9f80, ni_p_sys_air_field_air_friction_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_a190, ni_p_sys_air_field_air_friction_ctlr_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_b450,
            ni_p_sys_air_field_air_friction_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c2_b8a0,
            ni_p_sys_air_field_air_friction_ctlr_save_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c29e80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_9e80;
        const TARGET: u32 = 0x00a4_fbd0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c29f80_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_9f80;
        const TARGET: u32 = 0x00a4_4cd0;
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
    fn thunk_00c2a190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_a190;
        const TARGET: u32 = 0x00c3_2df0;
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
    fn thunk_00c2b450_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_b450;
        const TARGET: u32 = 0x00c3_2e10;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c2b8a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_b8a0;
        const TARGET: u32 = 0x00a4_1220;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_07c0, &args![]).u32(), 0x0120_27b8);
    }
}
