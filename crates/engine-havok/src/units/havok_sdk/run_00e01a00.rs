//! `Havok SDK/run_00e01a00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e01a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_350c.
pub fn return_constant_00e01a00(_e: &mut Engine) -> u32 {
    0x010e_350c
}

// Translated from 00e01a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_96c8.
pub fn return_constant_00e01a40(_e: &mut Engine) -> u32 {
    0x010e_96c8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e01a00, return_constant_00e01a00() -> u32),
        entry!(0x00e01a40, return_constant_00e01a40() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00e01a00_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1a00, &args![]).u32(), 0x010e_350c);
    }

    #[test]
    fn return_constant_00e01a40_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1a40, &args![]).u32(), 0x010e_96c8);
    }
}
