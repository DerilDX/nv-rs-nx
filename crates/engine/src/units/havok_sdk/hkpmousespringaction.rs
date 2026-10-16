//! `Havok SDK/hkpmousespringaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cbb8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8a14.
pub fn hkp_unary_action_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8a14
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cbb8a0, hkp_unary_action_get_class_type() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_unary_action_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_b8a0, &args![]).u32(), 0x0126_8a14);
    }
}
