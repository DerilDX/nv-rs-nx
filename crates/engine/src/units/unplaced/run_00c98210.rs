//! `(unplaced)/run_00c98210` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c98210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. cdecl `max(first, second)` on two `float`s, result in ST0:
/// `first` when `second < first` (an ordered compare, `FCOM` with
/// `TEST AH,5`), otherwise `second`, so a NaN in either gives `second`.
/// Both pass through `FLD`/`FSTP`, which quiets a signalling NaN.
pub fn fn_00c98210(_e: &mut Engine, first: f32, second: f32) -> f32 {
    let (first, second) = (x87_quiet(first), x87_quiet(second));
    if second < first {
        first
    } else {
        second
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c98210, fn_00c98210(f32, f32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c98210_is_the_larger_and_prefers_the_second_on_ties_and_nans() {
        let mut e = Engine::new();
        let mut max = |a: f32, b: f32| e.call(0x00c9_8210, &args![a, b]).f32();
        assert_eq!(max(3.0, 2.0), 3.0);
        assert_eq!(max(2.0, 3.0), 3.0);
        assert_eq!(max(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(max(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(max(f32::NAN, 1.0), 1.0);
        assert!(max(1.0, f32::NAN).is_nan());
    }
}
