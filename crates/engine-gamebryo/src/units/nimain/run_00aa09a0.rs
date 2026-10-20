//! `NiMain/run_00aa09a0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method (in the vtables of many
/// classes) that takes one argument it does not read and answers `true`.
pub fn fn_00aa09d0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa09d0, fn_00aa09d0(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_true() {
        let mut e = Engine::new();
        assert!(e.call(0x00aa09d0, &args![0x1000u32, 0u32]).bool());
    }
}
