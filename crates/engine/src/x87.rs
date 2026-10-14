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
}
