//! `NiParticle/nipsysresetonloopctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a6d0f0`: the jump target of `00a2f8b0`.
const JUMP_TARGET_00A2F8B0: u32 = 0x00a6_d0f0;

/// `00a6d730`: the jump target of `00c27120`.
const JUMP_TARGET_00C27120: u32 = 0x00a6_d730;

/// `00a6d5e0`: the jump target of `00c283e0`.
const JUMP_TARGET_00C283E0: u32 = 0x00a6_d5e0;

/// `00a6cc80`: the jump target of `00c283f0`.
const JUMP_TARGET_00C283F0: u32 = 0x00a6_cc80;

// Translated from 00a2f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a6d0f0` (engine map: `NiTimeController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysResetOnLoopCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A2F8B0, &args![this, stream]).bool()
}

// Translated from 00c27120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a6d730` (engine map: `NiTimeController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysResetOnLoopCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C27120, &args![this, stream]);
}

// Translated from 00c283e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::LoadBinary` (Xbox PDB): a 5-byte jump to `00a6d5e0` (engine map: `NiTimeController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiPSysResetOnLoopCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C283E0, &args![this, stream]);
}

// Translated from 00c283f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00a6cc80` (engine map: `NiTimeController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysResetOnLoopCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C283F0, &args![this, other]).bool()
}

// Translated from 00c28400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26a8.
pub fn ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26a8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c28400, ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00a2_f8b0, ni_p_sys_reset_on_loop_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_7120,
            ni_p_sys_reset_on_loop_ctlr_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_83e0,
            ni_p_sys_reset_on_loop_ctlr_load_binary(Ptr, Ptr)
        ),
        entry!(0x00c2_83f0, ni_p_sys_reset_on_loop_ctlr_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a2f8b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a2_f8b0;
        const TARGET: u32 = 0x00a6_d0f0;
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
    fn thunk_00c27120_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_7120;
        const TARGET: u32 = 0x00a6_d730;
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
    fn thunk_00c283e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_83e0;
        const TARGET: u32 = 0x00a6_d5e0;
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
    fn thunk_00c283f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_83f0;
        const TARGET: u32 = 0x00a6_cc80;
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
    fn ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_8400, &args![]).u32(), 0x0120_26a8);
    }
}
