//! `(unplaced)/run_00a232d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `00a23390` tests before answering selectors 7 to 10.
const FLAG_011F35C8: u32 = 0x011f_35c8;
/// The four signed 16-bit words selectors 7 to 10 answer (sign-extended).
const VALUES_011F35B0: u32 = 0x011f_35b0;

// Translated from 00a23390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, two words: an index and a selector; `RET 8`). For the 0xA0-byte
/// entry `index` from `this + 0x34`: selectors 1 to 6 answer the entry's words +0x00 to
/// +0x14; selectors 7 to 10, when the byte at `011f35c8` is not 0, answer the signed 16-bit
/// words at `011f35b0`, `011f35b2`, `011f35b4` and `011f35b6` (sign-extended), else 0; every
/// other selector answers 0 (the exe's jump table at `00a23460`, as `match` arms).
pub fn fn_00a23390(e: &mut Engine, this: Ptr, index: i32, selector: u32) -> i32 {
    let entry = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(0xa0))
        .wrapping_add(0x34);
    match selector {
        1..=6 => e.mem.u32(entry.wrapping_add(4 * (selector - 1))) as i32,
        7..=10 => {
            if e.mem.u8(FLAG_011F35C8) != 0 {
                i32::from(e.mem.u16(VALUES_011F35B0 + 2 * (selector - 7)) as i16)
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a2_3390, fn_00a23390(Ptr, i32, u32) -> i32)]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00a23390_answers_the_entry_words_and_the_flagged_globals() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        let this = e.mem.alloc(0x34 + 2 * 0xa0);
        for word in 0..6u32 {
            e.mem.set_u32(this + 0x34 + 0xa0 + 4 * word, 100 + word);
        }
        for selector in 1..=6u32 {
            assert_eq!(
                e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(),
                99 + selector as i32
            );
        }
        // Selectors 7..=10 read the words only with the flag set.
        for (i, value) in [-5i16, 7, -32768, 12].iter().enumerate() {
            e.mem.set_u16(VALUES_011F35B0 + 2 * i as u32, *value as u16);
        }
        for selector in 7..=10u32 {
            assert_eq!(e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(), 0);
        }
        e.mem.set_u8(FLAG_011F35C8, 1);
        let got: Vec<i32> = (7..=10u32)
            .map(|s| e.call(0x00a2_3390, &args![this, 1u32, s]).i32())
            .collect();
        assert_eq!(got, vec![-5, 7, -32768, 12]);
        // Selector 0 and anything above 10 answer 0.
        for selector in [0u32, 11, 0xffff_ffff] {
            assert_eq!(e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(), 0);
        }
    }
}
