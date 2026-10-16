//! `BSHavok/bhkspringdamperconstraintmotor.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cf45f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringDamperConstraintMotor::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8740.
pub fn bhk_spring_damper_constraint_motor_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8740
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf45f0, bhk_spring_damper_constraint_motor_get_r_t_t_i() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_spring_damper_constraint_motor_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_45f0, &args![]).u32(), 0x0126_8740);
    }
}
