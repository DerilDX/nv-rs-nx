//! `Havok SDK/run_00da0910` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00da0910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6bec.
pub fn return_constant_00da0910(_e: &mut Engine) -> u32 {
    0x010c_6bec
}

// Translated from 00da0950 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6c64.
pub fn return_constant_00da0950(_e: &mut Engine) -> u32 {
    0x010c_6c64
}

// Translated from 00da0b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6fd0.
pub fn return_constant_00da0b20(_e: &mut Engine) -> u32 {
    0x010c_6fd0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da0910, return_constant_00da0910() -> u32),
        entry!(0x00da0950, return_constant_00da0950() -> u32),
        entry!(0x00da0b20, return_constant_00da0b20() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00da0910_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0910, &args![]).u32(), 0x010c_6bec);
    }

    #[test]
    fn return_constant_00da0950_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0950, &args![]).u32(), 0x010c_6c64);
    }

    #[test]
    fn return_constant_00da0b20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0b20, &args![]).u32(), 0x010c_6fd0);
    }
}
