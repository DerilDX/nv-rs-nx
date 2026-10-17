//! `Havok SDK/hkmathtypes.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkMath::hkFloor`, `hkFloorToInt` and `hkFloatToInt` take a `float` on the stack (cdecl) and
//! work on its bits with integer instructions only. The argument is loaded and stored back with
//! `FLD`/`FSTP` first, which turns a signalling NaN into a quiet one ([`x87_quiet_bits`]); every
//! shift is a hardware shift (the count is taken modulo 32). Each function translates its own
//! disassembly; the results are `floor` / `trunc` for ordinary values, with the quirks the
//! integer tricks have (`hkFloorToInt` does not round negative non-integers down: `-1.5` gives
//! `-1`).

#[allow(unused_imports)]
use crate::prelude::*;

/// An arithmetic right shift with the hardware's count (modulo 32).
fn sar(value: u32, count: u32) -> u32 {
    ((value as i32) >> (count & 31)) as u32
}

/// The exponent of the float bits less the bias (`(bits >> 23 & 0xff) - 0x7f`).
fn exponent(bits: u32) -> u32 {
    ((bits >> 23) & 0xff).wrapping_sub(0x7f)
}

/// The mask that is all ones when the float is zero (either sign): `(|bits| - 1) >> 31`.
fn zero_mask(bits: u32) -> u32 {
    sar((bits & 0x7fff_ffff).wrapping_sub(1), 31)
}

/// The bits `hkFloor` answers for the bits `bits` of its argument.
fn floor_bits(bits: u32) -> u32 {
    let e = exponent(bits);
    let shift = 0x17u32.wrapping_sub(e);
    let shift_sign = sar(shift, 31);
    let x = bits & !zero_mask(bits);
    let kept = !shift_sign & shift;
    let count = (!shift_sign & 0x17).wrapping_sub(kept);
    let mask = sar(0xff80_0000, count) | shift_sign;
    let carry = !((!mask & x).wrapping_sub(1));
    let sign_of_x = sar(x, 31);
    let sign_of_e = sar(e, 31);
    let mut result = carry & sar(0x80_0000, e) & sign_of_x;
    result = result.wrapping_add(x);
    result &= !sign_of_e;
    result &= mask;
    result | (sign_of_e & sign_of_x & 0xbf80_0000)
}

// Translated from 00cbfd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMath::hkFloor` (Xbox PDB), cdecl `(float)`, the result in `ST0`: the largest whole number
/// not above `value`, computed on the bits (zero of either sign gives `+0.0`).
pub fn hk_math_hk_floor(_e: &mut Engine, value: f32) -> f32 {
    let bits = x87_quiet_bits(value.to_bits());
    f32::from_bits(x87_quiet_bits(floor_bits(bits)))
}

// Translated from 00cbfe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMath::hkFloorToInt` (Xbox PDB), cdecl `(float)`, the integer in `EAX`: `value` rounded
/// down to a whole number, as the exe's bit tricks do it (see the module notes for negative
/// non-integers).
pub fn hk_math_hk_floor_to_int(_e: &mut Engine, value: f32) -> i32 {
    let bits = x87_quiet_bits(value.to_bits());
    let e = exponent(bits);
    let low = e.wrapping_sub(0x17);
    let below_24 = sar(low.wrapping_sub(1), 31);
    let not_below_24 = !below_24;
    let left = low & not_below_24;
    let x = bits & !zero_mask(bits);
    let right = 0x17u32.wrapping_sub(e) & below_24;
    let count = (below_24 & 0x17).wrapping_sub(right);
    let mask = sar(0xff80_0000, count) | not_below_24;
    let sign_of_e = sar(e, 31);
    let sign_of_x = sar(x, 31);
    let not_sign_of_e = !sign_of_e;
    let mut part = !((!mask & x).wrapping_sub(1)) & sar(0x80_0000, e);
    part &= sign_of_x;
    part = part.wrapping_add(not_sign_of_e & x);
    let hidden = ((sign_of_e & sign_of_x) | not_sign_of_e) & 0x80_0000;
    part = ((part & 0x7f_ffff) | hidden) & mask;
    let mut magnitude = part.wrapping_add(part).wrapping_sub(1) | sign_of_x;
    magnitude = magnitude.wrapping_sub(part).wrapping_add(1);
    (sar(magnitude, right) << (left & 31)) as i32
}

// Translated from 00cbfef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMath::hkFloatToInt` (Xbox PDB), cdecl `(float)`, the integer in `EAX`: `value` truncated
/// toward zero, computed on the bits.
pub fn hk_math_hk_float_to_int(_e: &mut Engine, value: f32) -> i32 {
    let bits = x87_quiet_bits(value.to_bits());
    let e = exponent(bits);
    let low = e.wrapping_sub(0x17);
    let x = bits & !zero_mask(bits);
    let below_24 = sar(low.wrapping_sub(1), 31);
    let right = 0x17u32.wrapping_sub(e) & below_24;
    let count = (below_24 & 0x17).wrapping_sub(right);
    let not_below_24 = !below_24;
    let mask = sar(0xff80_0000, count) | not_below_24;
    let mantissa = ((x & 0x7f_ffff) | 0x80_0000) & mask;
    let sign_of_x = sar(x, 31);
    let mut signed = mantissa.wrapping_add(mantissa).wrapping_sub(1) | sign_of_x;
    signed = signed.wrapping_sub(mantissa).wrapping_add(1);
    signed &= !sar(e, 31);
    (sar(signed, right) << ((low & not_below_24) & 31)) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbfd80, hk_math_hk_floor(f32) -> f32),
        entry!(0x00cbfe20, hk_math_hk_floor_to_int(f32) -> i32),
        entry!(0x00cbfef0, hk_math_hk_float_to_int(f32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floor(e: &mut Engine, value: f32) -> f32 {
        e.call(0x00cb_fd80, &args![value]).f32()
    }

    fn floor_to_int(e: &mut Engine, value: f32) -> i32 {
        e.call(0x00cb_fe20, &args![value]).i32()
    }

    fn float_to_int(e: &mut Engine, value: f32) -> i32 {
        e.call(0x00cb_fef0, &args![value]).i32()
    }

    #[test]
    fn hk_floor_rounds_down() {
        let mut e = Engine::new();
        for value in [
            0.5f32,
            1.0,
            1.5,
            2.9999,
            -0.5,
            -1.0,
            -1.5,
            -2.25,
            -2.9999,
            100.75,
            -100.75,
            8_388_607.5,
            8_388_608.0,
            123_456_789.0,
            1e-10,
            -1e-10,
            3.0e30,
            -3.0e30,
        ] {
            assert_eq!(floor(&mut e, value), value.floor(), "{value}");
        }
        // Zero of either sign gives positive zero.
        assert_eq!(floor(&mut e, 0.0).to_bits(), 0);
        assert_eq!(floor(&mut e, -0.0).to_bits(), 0);
    }

    #[test]
    fn hk_floor_of_a_signalling_nan_is_quiet() {
        let mut e = Engine::new();
        let result = floor(&mut e, f32::from_bits(0x7f80_0001));
        assert!(result.is_nan());
        assert_ne!(result.to_bits() & 0x0040_0000, 0);
    }

    #[test]
    fn hk_floor_to_int_rounds_down_non_negative_values() {
        let mut e = Engine::new();
        for value in [
            0.0f32,
            0.25,
            0.999,
            1.0,
            1.5,
            2.5,
            7.9,
            255.5,
            1000.01,
            65_535.75,
            8_388_607.0,
            8_388_608.0,
            1.0e9,
        ] {
            assert_eq!(floor_to_int(&mut e, value), value.floor() as i32, "{value}");
        }
        // Whole negative numbers are exact.
        for value in [-1.0f32, -2.0, -100.0, -8_388_608.0] {
            assert_eq!(floor_to_int(&mut e, value), value as i32, "{value}");
        }
    }

    #[test]
    fn hk_floor_to_int_does_not_round_negative_non_integers_down() {
        // The exe's bit tricks drop the carry for these (checked against the decompiler's own
        // expression for the same function): they come out closer to zero than `floor`.
        let mut e = Engine::new();
        assert_eq!(floor_to_int(&mut e, -1.5), -1);
        assert_eq!(floor_to_int(&mut e, -0.5), -1);
        assert_eq!(floor_to_int(&mut e, -3.5), -2);
        assert_eq!(floor_to_int(&mut e, -7.9), -4);
    }

    #[test]
    fn hk_float_to_int_truncates_toward_zero() {
        let mut e = Engine::new();
        for value in [
            0.0f32,
            0.5,
            0.999,
            1.0,
            1.5,
            -0.5,
            -1.5,
            -2.99,
            127.9,
            -127.9,
            12_345.678,
            -12_345.678,
            8_388_607.5,
            -8_388_607.5,
            8_388_608.0,
            100_000_000.0,
            -100_000_000.0,
            1.0e-20,
            -1.0e-20,
        ] {
            assert_eq!(float_to_int(&mut e, value), value as i32, "{value}");
        }
    }
}
