//! `NiParticle/nipsysdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a8a9b0`: the jump target of `00a75f00`.
const JUMP_TARGET_00A75F00: u32 = 0x00a8_a9b0;

/// `00a8a9a0`: the jump target of `00a7b7d0`.
const JUMP_TARGET_00A7B7D0: u32 = 0x00a8_a9a0;

// Translated from 00a75f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a8a9b0` (engine map: `NiLinesData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A75F00, &args![this, stream]).bool()
}

// Translated from 00a7b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::LinkObject` (Xbox PDB): a 5-byte jump to `00a8a9a0` (engine map: `NiLinesData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysData@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A7B7D0, &args![this, stream]);
}

// Translated from 00c24e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2558.
pub fn ni_p_sys_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2558
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c24e90, ni_p_sys_data_get_r_t_t_i() -> u32),
        entry!(0x00a7_5f00, ni_p_sys_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_b7d0, ni_p_sys_data_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a75f00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_5f00;
        const TARGET: u32 = 0x00a8_a9b0;
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
    fn thunk_00a7b7d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_b7d0;
        const TARGET: u32 = 0x00a8_a9a0;
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
    fn ni_p_sys_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_4e90, &args![]).u32(), 0x0120_2558);
    }
}
