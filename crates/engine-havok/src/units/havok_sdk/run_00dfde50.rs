//! `Havok SDK/run_00dfde50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfde50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_3cc0.
pub fn return_constant_00dfde50(_e: &mut Engine) -> u32 {
    0x010d_3cc0
}

// Translated from 00dfdfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_3e5c.
pub fn return_constant_00dfdfc0(_e: &mut Engine) -> u32 {
    0x010e_3e5c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfde50, return_constant_00dfde50() -> u32),
        entry!(0x00dfdfc0, return_constant_00dfdfc0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfde50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_de50, &args![]).u32(), 0x010d_3cc0);
    }

    #[test]
    fn return_constant_00dfdfc0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_dfc0, &args![]).u32(), 0x010e_3e5c);
    }
}
