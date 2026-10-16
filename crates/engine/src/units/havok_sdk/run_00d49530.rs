//! `Havok SDK/run_00d49530` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d495f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_fb34.
pub fn return_constant_00d495f0(_e: &mut Engine) -> u32 {
    0x010c_fb34
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d495f0, return_constant_00d495f0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00d495f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_95f0, &args![]).u32(), 0x010c_fb34);
    }
}
