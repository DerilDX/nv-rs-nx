//! `NiMain/run_00ee8910` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ee8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual function (in several virtual tables, also called
/// directly) that answers `false` (`XOR AL,AL`; plain `RET`, `this` not read).
pub fn fn_00ee8990(_e: &mut Engine, _this: Ptr) -> bool {
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ee_8990, fn_00ee8990(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00ee_8990, &args![0x1000u32]).bool());
        assert!(!e.call(0x00ee_8990, &args![0u32]).bool());
    }
}
