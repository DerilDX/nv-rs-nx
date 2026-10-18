//! `NiParticle/nipsysemitterctlrdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a89af0`: the jump target of `00a724d0`.
const JUMP_TARGET_00A724D0: u32 = 0x00a8_9af0;

// Translated from 00a724d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::LinkObject` (Xbox PDB): a 5-byte jump to `00a89af0`, passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysEmitterCtlrData@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_emitter_ctlr_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A724D0, &args![this, stream]);
}

// Translated from 00c2baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2760.
pub fn ni_p_sys_emitter_ctlr_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2760
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2baa0, ni_p_sys_emitter_ctlr_data_get_r_t_t_i() -> u32),
        entry!(
            0x00a7_24d0,
            ni_p_sys_emitter_ctlr_data_link_object(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a724d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_24d0;
        const TARGET: u32 = 0x00a8_9af0;
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
    fn ni_p_sys_emitter_ctlr_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_baa0, &args![]).u32(), 0x0120_2760);
    }
}
