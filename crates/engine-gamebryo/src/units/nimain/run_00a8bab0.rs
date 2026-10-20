//! `NiMain/run_00a8bab0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The code address `00a8bd20` stores at +4 (`00a8b8c0`).
const FUNCTION_00A8B8C0: u32 = 0x00a8_b8c0;
/// The `double`s the nibble table is scaled with (`01015a38`, divided by; `0101e568`,
/// multiplied by) and the `float` 2^32 the unsigned conversion adds (`01099378`).
const DIVISOR_01015A38: u32 = 0x0101_5a38;
const FACTOR_0101E568: u32 = 0x0101_e568;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// One entry of the nibble table: `FILD` of the nibble (plus 2^32 when negative, which a
/// nibble never is), `FDIV ST0,ST2` by the divisor, `FMUL ST0,ST1` by the factor (bytes
/// `D8 F2`, `D8 C9`), `FISTP` with the rounding control set to chop; the integer's low byte.
fn scaled_nibble(e: &Engine, nibble: u32, divisor: f64, factor: f64) -> u8 {
    let mut value = f64::from(nibble as i32);
    if (nibble as i32) < 0 {
        // Unreachable: a nibble is never negative; the exe still tests it.
        value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
    }
    let scaled = x87_mul(x87_div(value, divisor), factor);
    crate::x87::fistp_chop(scaled) as u8
}

// Translated from 00a8bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8bab0`), `thiscall`, no stack words: clears +0, +8 and
/// +0xc, stores the code address `00a8b8c0` at +4, and fills the 256 byte pairs from +0x10:
/// for each index, its low nibble then its high nibble, each divided by the `double` at
/// `01015a38`, multiplied by the one at `0101e568` and truncated (both `double`s loaded once
/// on the x87 stack). Answers `this`.
pub fn fn_00a8bd20(e: &mut Engine, this: Ptr) -> Ptr {
    let divisor = x87_load64(e.mem.u64(DIVISOR_01015A38));
    let factor = x87_load64(e.mem.u64(FACTOR_0101E568));
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base.wrapping_add(4), FUNCTION_00A8B8C0);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    for index in 0..0x100u32 {
        let pair = base.wrapping_add(0x10 + 2 * index);
        let low = scaled_nibble(e, index & 0xf, divisor, factor);
        e.mem.set_u8(pair, low);
        let high = scaled_nibble(e, (index >> 4) & 0xf, divisor, factor);
        e.mem.set_u8(pair.wrapping_add(1), high);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_bd20, fn_00a8bd20(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    #[test]
    fn fn_00a8bd20_fills_the_scaled_nibble_pairs() {
        let mut e = Engine::new();
        e.map(DIVISOR_01015A38, 8);
        e.map(FACTOR_0101E568, 8);
        e.map(TWO_TO_32_01099378, 4);
        e.set_global(DIVISOR_01015A38, 15.0f64);
        e.set_global(FACTOR_0101E568, 255.0f64);
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x00a8_bd20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), FUNCTION_00A8B8C0);
        let expected = |n: u32| (f64::from(n) / 15.0 * 255.0).trunc() as u8;
        for index in [0u32, 0x21, 0xd7, 0xff] {
            let pair = this + 0x10 + 2 * index;
            assert_eq!(e.mem.u8(pair), expected(index & 0xf));
            assert_eq!(e.mem.u8(pair + 1), expected(index >> 4));
        }
        assert_eq!(e.mem.u16(this + 0x10 + 2 * 0xff), 0xffff);
        assert_eq!(e.mem.u16(this + 0x10), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8bd20_fills_the_nibble_table() {
        let mut e = Engine::new();
        e.map(DIVISOR_01015A38, 8);
        e.mem.set_u64(DIVISOR_01015A38, 15f64.to_bits());
        e.map(FACTOR_0101E568, 8);
        e.mem.set_u64(FACTOR_0101E568, 255f64.to_bits());
        e.map(TWO_TO_32_01099378, 4);
        e.mem.set_u32(TWO_TO_32_01099378, 0x4f80_0000);
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 9);
        assert_eq!(e.call(0x00a8_bd20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), FUNCTION_00A8B8C0);
        for index in 0u32..0x100 {
            assert_eq!(
                e.mem.u8(this + 0x10 + 2 * index),
                ((index & 0xf) * 17) as u8
            );
            assert_eq!(e.mem.u8(this + 0x11 + 2 * index), ((index >> 4) * 17) as u8);
        }
    }
}
