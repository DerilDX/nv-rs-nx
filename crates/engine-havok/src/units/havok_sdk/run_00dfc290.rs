//! `Havok SDK/run_00dfc290` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_00cc.
pub fn return_constant_00dfc290(_e: &mut Engine) -> u32 {
    0x010e_00cc
}

// Translated from 00dfc2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_00dc.
pub fn return_constant_00dfc2b0(_e: &mut Engine) -> u32 {
    0x010e_00dc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfc290, return_constant_00dfc290() -> u32),
        entry!(0x00dfc2b0, return_constant_00dfc2b0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfc290_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c290, &args![]).u32(), 0x010e_00cc);
    }

    #[test]
    fn return_constant_00dfc2b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c2b0, &args![]).u32(), 0x010e_00dc);
    }
}
