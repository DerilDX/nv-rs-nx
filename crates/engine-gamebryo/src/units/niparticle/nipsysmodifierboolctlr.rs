//! `NiParticle/nipsysmodifierboolctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a57130`: the jump target of `00a41220`.
const JUMP_TARGET_00A41220: u32 = 0x00a5_7130;

/// `00a57780`: the jump target of `00a44cd0`.
const JUMP_TARGET_00A44CD0: u32 = 0x00a5_7780;

/// `00c313a0`: the jump target of `00c32df0`.
const JUMP_TARGET_00C32DF0: u32 = 0x00c3_13a0;

// Translated from 00a41220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57130` (engine map: `NiPSysModifierCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysModifierBoolCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A41220, &args![this, stream]);
}

// Translated from 00a44cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57780` (engine map: `NiPSysModifierCtlr::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysModifierBoolCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A44CD0, &args![this, stream]).bool()
}

// Translated from 00c32df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00c313a0` (engine map: `NiPSysModifierCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysModifierBoolCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C32DF0, &args![this, other]).bool()
}

// Translated from 00c32be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27f0.
pub fn ni_p_sys_modifier_bool_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27f0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c32be0, ni_p_sys_modifier_bool_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00a4_1220,
            ni_p_sys_modifier_bool_ctlr_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_4cd0, ni_p_sys_modifier_bool_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c3_2df0, ni_p_sys_modifier_bool_ctlr_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a41220_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_1220;
        const TARGET: u32 = 0x00a5_7130;
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
    fn thunk_00a44cd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_4cd0;
        const TARGET: u32 = 0x00a5_7780;
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
    fn thunk_00c32df0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c3_2df0;
        const TARGET: u32 = 0x00c3_13a0;
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
    fn ni_p_sys_modifier_bool_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_2be0, &args![]).u32(), 0x0120_27f0);
    }
}
