//! `BSShader/bsshadermanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `sin` (`00eca0a0`): the argument is the x87 `ST0`, so a leading `f64`; the result is `ST0`.
const CRT_SIN: u32 = 0x00ec_a0a0;
/// CRT `cos` (`00ec9f70`), same convention.
const CRT_COS: u32 = 0x00ec_9f70;
/// The first `double` the argument is multiplied by.
const FIRST_FACTOR: u32 = 0x0101_1588;
/// The second `double` the argument is multiplied by.
const SECOND_FACTOR: u32 = 0x0102_3128;
/// The `double` angle the ratio is divided by `tan` of.
const REFERENCE_ANGLE: u32 = 0x010a_d6d8;
/// The `float` the result is stored in.
const RATIO_RESULT: u32 = 0x011a_dcc8;

// Translated from 00b54000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`). `cdecl`, one `float` stack word: scales it
/// by the doubles at `01011588` and `01023128` (stored as a `float`), takes
/// `sin / cos` of the result and of the double at `010ad6d8` (each `sin` and `cos` stored as
/// a `float` first) and stores the first quotient divided by the second in the `float` at
/// `011adcc8`. Computed on the x87 as the exe does (53-bit precision, NaN choice of the x87).
pub fn fn_00b54000(e: &mut Engine, value: f32) {
    let first_factor: f64 = e.global(FIRST_FACTOR);
    let second_factor: f64 = e.global(SECOND_FACTOR);
    let scaled = x87_store32(x87_mul(
        x87_mul(x87_load32(value.to_bits()), first_factor),
        second_factor,
    ));
    let angle = x87_load32(scaled);
    let sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![angle]).f64()));
    let cosine = x87_store32(e.call(CRT_COS, &args![angle]).f64());
    let quotient = x87_quiet_bits(x87_store32(x87_div(x87_m32(sine), x87_load32(cosine))));

    let reference: f64 = e.global(REFERENCE_ANGLE);
    let reference_sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![reference]).f64()));
    let reference_cosine = x87_store32(e.call(CRT_COS, &args![reference]).f64());
    let reference_quotient = x87_div(x87_m32(reference_sine), x87_load32(reference_cosine));
    let ratio = x87_div(x87_m32(quotient), reference_quotient);
    e.set_global(RATIO_RESULT, f32::from_bits(x87_store32(ratio)));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b5_4000, fn_00b54000(f32))]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(FIRST_FACTOR, 8);
        e.map(SECOND_FACTOR, 8);
        e.map(REFERENCE_ANGLE, 8);
        e.map(RATIO_RESULT, 4);
        e.set_global(FIRST_FACTOR, 0.5f64);
        e.set_global(SECOND_FACTOR, 2.0f64);
        e.set_global(REFERENCE_ANGLE, 3.0f64);
        // sin(x) = x + 1, cos(x) = 2: the quotients are exact in binary.
        e.register(CRT_SIN, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x + 1.0,
                ..Ret::default()
            }
        });
        e.register(CRT_COS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn at_00b54000_divides_tan_of_the_scaled_angle_by_tan_of_the_reference() {
        let mut e = rig();
        // 0.5 * 0.5 * 2.0 = 0.5; (0.5 + 1) / 2 = 0.75; (3 + 1) / 2 = 2; 0.75 / 2 = 0.375.
        e.call(0x00b5_4000, &args![0.5f32]);
        assert_eq!(e.global::<f32>(RATIO_RESULT), 0.375);
        let log = e.call_log.take().unwrap();
        let sines: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CRT_SIN)
            .map(|c| &c.1)
            .collect();
        let half = 0.5f64.to_bits();
        let three = 3.0f64.to_bits();
        assert_eq!(
            sines,
            vec![
                &vec![half as u32, (half >> 32) as u32],
                &vec![three as u32, (three >> 32) as u32]
            ]
        );
    }

    #[test]
    fn at_00b54000_rounds_each_stage_to_a_float() {
        let mut e = rig();
        // The scaled angle (0.2 * 0.5 * 1.0) is a float before sin sees it.
        e.set_global(SECOND_FACTOR, 1.0f64);
        e.call(0x00b5_4000, &args![0.2f32]);
        let scaled = f64::from(0.2f32 / 2.0);
        let sine = f64::from((scaled + 1.0) as f32);
        let quotient = f64::from((sine / 2.0) as f32);
        let expected = (quotient / 2.0) as f32;
        assert_eq!(e.global::<f32>(RATIO_RESULT), expected);
    }

    #[test]
    fn at_00b54000_gives_a_nan_for_a_nan_argument() {
        let mut e = rig();
        e.call(0x00b5_4000, &args![f32::from_bits(0x7f80_0001)]);
        assert!(e.global::<f32>(RATIO_RESULT).is_nan());
    }
}
