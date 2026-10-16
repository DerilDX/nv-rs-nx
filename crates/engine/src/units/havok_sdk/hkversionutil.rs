//! `Havok SDK/hkversionutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dc0fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionUtil::getCurrentVersion` (Xbox PDB): takes no arguments and returns the constant 0x010c_f830.
/// The constant is the address of a NUL-terminated string in the exe.
pub fn hk_version_util_get_current_version(_e: &mut Engine) -> u32 {
    0x010c_f830
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dc0fb0, hk_version_util_get_current_version() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_version_util_get_current_version_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dc_0fb0, &args![]).u32(), 0x010c_f830);
    }
}
