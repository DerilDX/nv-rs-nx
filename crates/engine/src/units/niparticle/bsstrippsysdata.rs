//! `NiParticle/bsstrippsysdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b7d0`: the jump target of `00a817a0`.
const JUMP_TARGET_00A817A0: u32 = 0x00a7_b7d0;

/// `00a75f00`: the jump target of `00a817b0`.
const JUMP_TARGET_00A817B0: u32 = 0x00a7_5f00;

// Translated from 00a817a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::LinkObject` (Xbox PDB): a 5-byte jump to `00a7b7d0` (engine map: `NiPSysData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSStripPSysData@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_p_sys_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A817A0, &args![this, stream]);
}

// Translated from 00a817b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a75f00` (engine map: `NiPSysData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSStripPSysData@@UAA_NAAVNiStream@@@Z`.
pub fn bs_strip_p_sys_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A817B0, &args![this, stream]).bool()
}

// Translated from 00c25ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2680.
pub fn b_s_strip_p_sys_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2680
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c25ee0, b_s_strip_p_sys_data_get_r_t_t_i() -> u32),
        entry!(0x00a8_17a0, bs_strip_p_sys_data_link_object(Ptr, Ptr)),
        entry!(0x00a8_17b0, bs_strip_p_sys_data_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a817a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_17a0;
        const TARGET: u32 = 0x00a7_b7d0;
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
    fn thunk_00a817b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_17b0;
        const TARGET: u32 = 0x00a7_5f00;
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
    fn b_s_strip_p_sys_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_5ee0, &args![]).u32(), 0x0120_2680);
    }
}
