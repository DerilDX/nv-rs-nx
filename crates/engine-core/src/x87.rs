//! x87 load/store behaviour shared by the translations.
//!
//! An `FLD`/`FSTP` round trip of a single-precision value through the x87
//! stack quiets a signalling NaN: the quiet bit (bit 22) is set and the rest
//! of the payload is kept. Every other value, quiet NaNs and infinities
//! included, comes back unchanged. This is done on the bit pattern, because
//! Rust's `as` casts do not guarantee NaN payloads.

/// The bit pattern an `FLD dword` / `FSTP dword` pair leaves for `bits`.
pub fn x87_quiet_bits(bits: u32) -> u32 {
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// [`x87_quiet_bits`] on an `f32`.
pub fn x87_quiet(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()))
}

const F64_QUIET: u64 = 1 << 51;
const F64_EXPONENT: u64 = 0x7ff0_0000_0000_0000;
const F64_SIGNIFICAND: u64 = 0x000f_ffff_ffff_ffff;

fn is_nan_bits(bits: u64) -> bool {
    bits & F64_EXPONENT == F64_EXPONENT && bits & F64_SIGNIFICAND != 0
}

/// The NaN an x87 `FADD`/`FSUB`/`FMUL`/`FDIV` (and their `P`/`R` forms)
/// gives when an operand is a NaN, or `None` when neither is (Intel SDM
/// vol. 1, section 4.8.3.5, table 4-7, x87 column). One NaN: that NaN,
/// quieted. Two: a quiet NaN wins over a signalling one; otherwise the one
/// with the larger significand (compared quieted), and on a tie the
/// positive one. The operand order does not matter. f64 arithmetic does not
/// follow this: SSE keeps the first operand's NaN, and LLVM may swap the
/// operands of a commutative operation (#199).
fn nan_result(a: f64, b: f64) -> Option<f64> {
    let (a, b) = (a.to_bits(), b.to_bits());
    let chosen = match (is_nan_bits(a), is_nan_bits(b)) {
        (false, false) => return None,
        (true, false) => a,
        (false, true) => b,
        (true, true) => match (a & F64_QUIET != 0, b & F64_QUIET != 0) {
            (true, false) => a,
            (false, true) => b,
            _ => {
                let (sa, sb) = (
                    (a | F64_QUIET) & F64_SIGNIFICAND,
                    (b | F64_QUIET) & F64_SIGNIFICAND,
                );
                match sa.cmp(&sb) {
                    std::cmp::Ordering::Greater => a,
                    std::cmp::Ordering::Less => b,
                    // Equal significands: the sign bit decides, positive first.
                    std::cmp::Ordering::Equal => a.min(b),
                }
            }
        },
    };
    Some(f64::from_bits(chosen | F64_QUIET))
}

/// x87 `FADD`: `a + b` with the x87 choice of NaN ([`nan_result`]).
pub fn x87_add(a: f64, b: f64) -> f64 {
    nan_result(a, b).unwrap_or(a + b)
}

/// x87 `FSUB`: `a - b` with the x87 choice of NaN.
pub fn x87_sub(a: f64, b: f64) -> f64 {
    nan_result(a, b).unwrap_or(a - b)
}

/// x87 `FMUL`: `a * b` with the x87 choice of NaN.
pub fn x87_mul(a: f64, b: f64) -> f64 {
    nan_result(a, b).unwrap_or(a * b)
}

/// x87 `FDIV`: `a / b` with the x87 choice of NaN.
pub fn x87_div(a: f64, b: f64) -> f64 {
    nan_result(a, b).unwrap_or(a / b)
}

/// A `float` memory operand of an x87 arithmetic instruction (`FMUL dword
/// ptr [..]` and the like) as the instruction sees it: widened exactly, a
/// signalling NaN still signalling, so the helpers above can tell it from a
/// quiet one. A value loaded by `FLD dword` is quiet: use
/// `f64::from(x87_quiet(..))` for that one.
pub fn x87_m32(bits: u32) -> f64 {
    let sign = u64::from(bits >> 31) << 63;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = u64::from(bits & 0x007f_ffff);
    if exponent == 0xff && mantissa != 0 {
        f64::from_bits(sign | F64_EXPONENT | (mantissa << 29))
    } else {
        f64::from(f32::from_bits(bits))
    }
}

/// `FLD dword`: the value the x87 stack holds after loading the float with
/// the bit pattern `bits` (a signalling NaN quieted, everything else exact).
pub fn x87_load32(bits: u32) -> f64 {
    f64::from(f32::from_bits(x87_quiet_bits(bits)))
}

/// `FSTP dword`: the stack value rounded to a float, as its bits (a NaN keeps
/// its sign and the top of its payload).
pub fn x87_store32(value: f64) -> u32 {
    (value as f32).to_bits()
}

// Helpers of several engine crates, kept here so each reaches them: `fistp`
// (from `units/bsmenu/font.rs`) and `rsqrt_estimate` (from
// `units/fallout_shared/tesobjectrefr.rs`, which the Havok units call too).

/// `FISTP` with the default rounding mode: nearest, ties to even; the
/// integer indefinite 0x80000000 for NaN and values out of range.
pub fn fistp(value: f64) -> i32 {
    // Round half to even (the FPU's default mode) without `round_ties_even`,
    // which is newer than the crate's MSRV: `round` goes half away from zero,
    // so an odd result of a tie steps back toward zero.
    let mut r = value.round();
    if (value - value.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r -= value.signum();
    }
    if r.is_nan() || r < i32::MIN as f64 || r > i32::MAX as f64 {
        i32::MIN
    } else {
        r as i32
    }
}

/// The `CMPEQSS`/`RSQRTSS` pair of the Havok normalizer: the hardware
/// estimate of `1 / sqrt(x)` on x86, exact elsewhere.
#[cfg(target_arch = "x86_64")]
pub fn rsqrt_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn rsqrt_estimate(x: f32) -> f32 {
    1.0 / x.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signalling_nan_gets_the_quiet_bit_and_keeps_its_payload() {
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0xff80_1234), 0xffc0_1234);
    }

    #[test]
    fn other_values_are_unchanged() {
        for bits in [
            0,
            0x8000_0000,
            0x3f80_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            0x7fc0_0005,
        ] {
            assert_eq!(x87_quiet_bits(bits), bits);
        }
        assert_eq!(x87_quiet(1.5), 1.5);
    }

    fn f32_result(value: f64) -> u32 {
        (value as f32).to_bits()
    }

    #[test]
    fn two_nans_keep_the_larger_significand_then_the_positive_one() {
        // The vectors of #199: -qNaN and +qNaN with equal significands.
        let (neg, pos) = (x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff));
        assert_eq!(f32_result(x87_mul(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(pos, neg)), 0x7fff_ffff);
        // Key 0x7fc00000 times scale 0xffffffff: the larger significand wins.
        let small = x87_m32(0x7fc0_0000);
        assert_eq!(f32_result(x87_mul(small, neg)), 0xffff_ffff);
        assert_eq!(f32_result(x87_mul(neg, small)), 0xffff_ffff);
        // x and |x| for x = 0xffc00000.
        let x = x87_m32(0xffc0_0000);
        assert_eq!(f32_result(x87_mul(x, x.abs())), 0x7fc0_0000);
        assert_eq!(f32_result(x87_sub(x.abs(), x)), 0x7fc0_0000);
        assert_eq!(f32_result(x87_div(x, x.abs())), 0x7fc0_0000);
    }

    #[test]
    fn a_quiet_nan_wins_over_a_signalling_one() {
        let signalling = x87_m32(0x7fbf_ffff);
        let quiet = x87_m32(0xffc0_0001);
        assert_eq!(f32_result(x87_mul(signalling, quiet)), 0xffc0_0001);
        assert_eq!(f32_result(x87_mul(quiet, signalling)), 0xffc0_0001);
        // Two signalling NaNs: the larger significand, quieted.
        let other = x87_m32(0xff80_0001);
        assert_eq!(f32_result(x87_add(signalling, other)), 0x7fff_ffff);
    }

    #[test]
    fn one_nan_comes_back_quieted_and_numbers_are_unchanged() {
        assert_eq!(f32_result(x87_mul(x87_m32(0xff80_0005), 2.0)), 0xffc0_0005);
        assert_eq!(f32_result(x87_sub(3.0, x87_m32(0x7fc0_1234))), 0x7fc0_1234);
        assert_eq!(x87_add(1.5, 2.25), 3.75);
        assert_eq!(x87_sub(1.5, 2.25), -0.75);
        assert_eq!(x87_mul(1.5, -2.0), -3.0);
        assert_eq!(x87_div(1.0, 4.0), 0.25);
        assert_eq!(x87_m32(0x3fc0_0000), 1.5);
        assert_eq!(x87_m32(0x7f80_0000), f64::INFINITY);
    }

    #[test]
    fn load32_quiets_and_store32_rounds_to_a_float() {
        assert_eq!(x87_load32(0x3fc0_0000), 1.5);
        assert_eq!(x87_store32(x87_load32(0x7f80_0001)), 0x7fc0_0001);
        assert_eq!(x87_store32(x87_load32(0xffc0_1234)), 0xffc0_1234);
        assert_eq!(x87_store32(x87_load32(0x0000_0001)), 0x0000_0001);
        // 1 + 2^-30 rounds to 1.0 in a float.
        assert_eq!(x87_store32(1.0 + 2f64.powi(-30)), 0x3f80_0000);
        assert_eq!(
            x87_store32(x87_add(x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff))),
            0x7fff_ffff
        );
    }
}
