//! `NiParticle/nipsyscolormodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2778.
pub fn ni_p_sys_color_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2778
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2d170, ni_p_sys_color_modifier_get_r_t_t_i() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_color_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d170, &args![]).u32(), 0x0120_2778);
    }
}
