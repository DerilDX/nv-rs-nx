//! `(unplaced)/run_00b49e20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b49e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running checksum, a
/// buffer and its length). A null buffer answers 1. Otherwise the low and
/// high halves of the checksum sum the bytes and the running sums, in blocks
/// of at most 0x15b0 bytes (16 at a time while at least 16 remain, then one
/// by one), each block ending with both halves taken modulo 0xfff1. Answers
/// `high << 16 | low`.
pub fn fn_00b49e20(e: &mut Engine, checksum: u32, buffer: u32, length: u32) -> u32 {
    let mut low = checksum & 0xffff;
    let mut high = (checksum >> 16) & 0xffff;
    if buffer == 0 {
        return 1;
    }
    let mut at = buffer;
    let mut length = length;
    while length != 0 {
        let mut block = if length < 0x15b0 { length } else { 0x15b0 };
        length -= block;
        while (block as i32) >= 0x10 {
            for i in 0..0x10 {
                low = low.wrapping_add(u32::from(e.mem.u8(at + i)));
                high = high.wrapping_add(low);
            }
            at = at.wrapping_add(0x10);
            block -= 0x10;
        }
        while block != 0 {
            low = low.wrapping_add(u32::from(e.mem.u8(at)));
            at = at.wrapping_add(1);
            high = high.wrapping_add(low);
            block -= 1;
        }
        low %= 0xfff1;
        high %= 0xfff1;
    }
    (high << 16) | low
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b4_9e20, fn_00b49e20(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The same sums written out directly, for the check.
    fn expected(text: &[u8]) -> u32 {
        let (mut a, mut b) = (1u32, 0u32);
        for &c in text {
            a = (a + u32::from(c)) % 0xfff1;
            b = (b + a) % 0xfff1;
        }
        (b << 16) | a
    }

    #[test]
    fn fn_00b49e20_sums_short_and_long_buffers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b4_9e20, &args![5u32, 0u32, 3u32]).u32(), 1);
        let text = b"Wikipedia";
        let at = e.mem.alloc(16);
        e.mem.write(at, text);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, at, 9u32]).u32(),
            0x11e6_0398
        );
        let long: Vec<u8> = (0..0x2000u32).map(|i| (i * 7 + 3) as u8).collect();
        let big = e.mem.alloc(0x2000);
        e.mem.write(big, &long);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, big, 0x2000u32]).u32(),
            expected(&long)
        );
    }
}
