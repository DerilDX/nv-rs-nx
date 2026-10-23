//! `BSAudio/run_00ad8790` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ad87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that answers 0 (no PDB name; `this` is not read, no stack
/// words).
pub fn fn_00ad87c0(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ad_87c0, fn_00ad87c0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ad87c0_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ad_87c0, &args![0x5000u32]).u32(), 0);
    }
}
