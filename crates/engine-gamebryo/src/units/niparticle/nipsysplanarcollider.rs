//! `NiParticle/nipsysplanarcollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c32680`: the jump target of `00c28d00`.
const JUMP_TARGET_00C28D00: u32 = 0x00c3_2680;

// Translated from 00c28d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c32680` (engine map: `NiPSysCollider::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysPlanarCollider@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_planar_collider_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C28D00, &args![this, stream]).bool()
}

// Translated from 00c299d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26c8.
pub fn ni_p_sys_planar_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26c8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c299d0, ni_p_sys_planar_collider_get_r_t_t_i() -> u32),
        entry!(0x00c2_8d00, ni_p_sys_planar_collider_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c28d00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_8d00;
        const TARGET: u32 = 0x00c3_2680;
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
    fn ni_p_sys_planar_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_99d0, &args![]).u32(), 0x0120_26c8);
    }
}
