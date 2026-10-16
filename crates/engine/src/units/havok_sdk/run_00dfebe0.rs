//! `Havok SDK/run_00dfebe0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5b4c.
pub fn return_constant_00dfebe0(_e: &mut Engine) -> u32 {
    0x010e_5b4c
}

// Translated from 00dfec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5bfc.
pub fn return_constant_00dfec20(_e: &mut Engine) -> u32 {
    0x010e_5bfc
}

// Translated from 00dfec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5d94.
pub fn return_constant_00dfec80(_e: &mut Engine) -> u32 {
    0x010e_5d94
}

// Translated from 00dfeca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5da8.
pub fn return_constant_00dfeca0(_e: &mut Engine) -> u32 {
    0x010e_5da8
}

// Translated from 00dfece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5e9c.
pub fn return_constant_00dfece0(_e: &mut Engine) -> u32 {
    0x010e_5e9c
}

// Translated from 00dfed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5ff4.
pub fn return_constant_00dfed20(_e: &mut Engine) -> u32 {
    0x010e_5ff4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfebe0, return_constant_00dfebe0() -> u32),
        entry!(0x00dfec20, return_constant_00dfec20() -> u32),
        entry!(0x00dfec80, return_constant_00dfec80() -> u32),
        entry!(0x00dfeca0, return_constant_00dfeca0() -> u32),
        entry!(0x00dfece0, return_constant_00dfece0() -> u32),
        entry!(0x00dfed20, return_constant_00dfed20() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfebe0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ebe0, &args![]).u32(), 0x010e_5b4c);
    }

    #[test]
    fn return_constant_00dfec20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ec20, &args![]).u32(), 0x010e_5bfc);
    }

    #[test]
    fn return_constant_00dfec80_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ec80, &args![]).u32(), 0x010e_5d94);
    }

    #[test]
    fn return_constant_00dfeca0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eca0, &args![]).u32(), 0x010e_5da8);
    }

    #[test]
    fn return_constant_00dfece0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ece0, &args![]).u32(), 0x010e_5e9c);
    }

    #[test]
    fn return_constant_00dfed20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ed20, &args![]).u32(), 0x010e_5ff4);
    }
}
