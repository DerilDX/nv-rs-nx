//! `Havok SDK/hkpdefaulttoiresourcemgr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d9fc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_955c.
pub fn hkp_default_toi_resource_mgr_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_955c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9fc60, hkp_default_toi_resource_mgr_get_class_type() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_default_toi_resource_mgr_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d9_fc60, &args![]).u32(), 0x0126_955c);
    }
}
