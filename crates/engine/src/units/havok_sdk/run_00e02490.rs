//! `Havok SDK/run_00e02490` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e02490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9fa4.
pub fn return_constant_00e02490(_e: &mut Engine) -> u32 {
    0x010e_9fa4
}

// Translated from 00e024d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a0a4.
pub fn return_constant_00e024d0(_e: &mut Engine) -> u32 {
    0x010e_a0a4
}

// Translated from 00e02510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_310c.
pub fn return_constant_00e02510(_e: &mut Engine) -> u32 {
    0x010e_310c
}

// Translated from 00e02550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a10c.
pub fn return_constant_00e02550(_e: &mut Engine) -> u32 {
    0x010e_a10c
}

// Translated from 00e02590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a154.
pub fn return_constant_00e02590(_e: &mut Engine) -> u32 {
    0x010e_a154
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e02490, return_constant_00e02490() -> u32),
        entry!(0x00e024d0, return_constant_00e024d0() -> u32),
        entry!(0x00e02510, return_constant_00e02510() -> u32),
        entry!(0x00e02550, return_constant_00e02550() -> u32),
        entry!(0x00e02590, return_constant_00e02590() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00e02490_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2490, &args![]).u32(), 0x010e_9fa4);
    }

    #[test]
    fn return_constant_00e024d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_24d0, &args![]).u32(), 0x010e_a0a4);
    }

    #[test]
    fn return_constant_00e02510_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2510, &args![]).u32(), 0x010e_310c);
    }

    #[test]
    fn return_constant_00e02550_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2550, &args![]).u32(), 0x010e_a10c);
    }

    #[test]
    fn return_constant_00e02590_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2590, &args![]).u32(), 0x010e_a154);
    }
}
