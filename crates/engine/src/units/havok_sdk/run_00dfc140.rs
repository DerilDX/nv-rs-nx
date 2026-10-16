//! `Havok SDK/run_00dfc140` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc140 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_ff44.
pub fn return_constant_00dfc140(_e: &mut Engine) -> u32 {
    0x010d_ff44
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dfc140, return_constant_00dfc140() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfc140_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c140, &args![]).u32(), 0x010d_ff44);
    }
}
