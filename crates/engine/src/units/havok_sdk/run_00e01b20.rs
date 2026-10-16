//! `Havok SDK/run_00e01b20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e01b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_977c.
pub fn return_constant_00e01b20(_e: &mut Engine) -> u32 {
    0x010e_977c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e01b20, return_constant_00e01b20() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00e01b20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1b20, &args![]).u32(), 0x010e_977c);
    }
}
