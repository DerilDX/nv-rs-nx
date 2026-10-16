//! `NiParticle/nipsyscollidermanager.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2780.
pub fn ni_p_sys_collider_manager_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2780
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2d540, ni_p_sys_collider_manager_get_r_t_t_i() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_collider_manager_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d540, &args![]).u32(), 0x0120_2780);
    }
}
