//! `NiParticle/nipsysmodifierctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a35850`: the jump target of `00a57630`.
const JUMP_TARGET_00A57630: u32 = 0x00a3_5850;

/// `00a38fe0`: the jump target of `00a57780`.
const JUMP_TARGET_00A57780: u32 = 0x00a3_8fe0;

// Translated from 00a57630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00a35850` (engine map: `NiSingleInterpController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysModifierCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57630, &args![this, stream]);
}

// Translated from 00a57780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a38fe0` (engine map: `NiSingleInterpController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysModifierCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57780, &args![this, stream]).bool()
}

// Translated from 00c31290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27c8.
pub fn ni_p_sys_modifier_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27c8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c31290, ni_p_sys_modifier_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00a5_7630, ni_p_sys_modifier_ctlr_link_object(Ptr, Ptr)),
        entry!(0x00a5_7780, ni_p_sys_modifier_ctlr_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a57630_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7630;
        const TARGET: u32 = 0x00a3_5850;
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
    fn thunk_00a57780_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7780;
        const TARGET: u32 = 0x00a3_8fe0;
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
    fn ni_p_sys_modifier_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1290, &args![]).u32(), 0x0120_27c8);
    }
}
