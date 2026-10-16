//! `NiParticle/nipsysboxemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c2f4f0`: the jump target of `00c1fdd0`.
const JUMP_TARGET_00C1FDD0: u32 = 0x00c2_f4f0;

/// `00c31c90`: the jump target of `00c20150`.
const JUMP_TARGET_00C20150: u32 = 0x00c3_1c90;

/// `00c31b40`: the jump target of `00c20160`.
const JUMP_TARGET_00C20160: u32 = 0x00c3_1b40;

// Translated from 00c1fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c2f4f0` (engine map: `NiPSysAirFieldModifier::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysBoxEmitter@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_box_emitter_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00C1FDD0, &args![this, stream]).bool()
}

// Translated from 00c20150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::ProcessClone` (Xbox PDB): a 5-byte jump to `00c31c90` (engine map: `NiPSysVolumeEmitter::ProcessClone`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?ProcessClone@NiPSysBoxEmitter@@UAAXAAVNiCloningProcess@@@Z`.
pub fn ni_p_sys_box_emitter_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(JUMP_TARGET_00C20150, &args![this, process]);
}

// Translated from 00c20160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::LinkObject` (Xbox PDB): a 5-byte jump to `00c31b40` (engine map: `NiPSysVolumeEmitter::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysBoxEmitter@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_box_emitter_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C20160, &args![this, stream]);
}

// Translated from 00c201c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2518.
pub fn ni_p_sys_box_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2518
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c201c0, ni_p_sys_box_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c1_fdd0, ni_p_sys_box_emitter_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_0150, ni_p_sys_box_emitter_process_clone(Ptr, Ptr)),
        entry!(0x00c2_0160, ni_p_sys_box_emitter_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c1fdd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c1_fdd0;
        const TARGET: u32 = 0x00c2_f4f0;
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
    fn thunk_00c20150_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_0150;
        const TARGET: u32 = 0x00c3_1c90;
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
    fn thunk_00c20160_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_0160;
        const TARGET: u32 = 0x00c3_1b40;
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
    fn ni_p_sys_box_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_01c0, &args![]).u32(), 0x0120_2518);
    }
}
