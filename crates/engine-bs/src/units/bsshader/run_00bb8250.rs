//! `BSShader/run_00bb8250` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The two effects' vtables (`010b9f04`, `010b9f58`; no RTTI locator before them).
const VTABLE_010B9F04: u32 = 0x010b_9f04;
const VTABLE_010B9F58: u32 = 0x010b_9f58;
/// The four words `00bb8900` copies (`011a9be0` to `011a9bec`, to `01200800` to `0120080c`).
const WORDS_011A9BE0: u32 = 0x011a_9be0;
const WORDS_01200800: u32 = 0x0120_0800;

// Translated from 00bb8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b9f04`, sizes the array at +0x28 to 5 (`00a90180`), clears the byte +4 and copies the
/// four words at `011a9be0` to `01200800` (stored in the order first, fourth, second, third).
/// Answers `this`.
// Translation incomplete: 00bb8900 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb8900(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B9F04);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.mem.set_u8(base + 4, 0);
    let words: Vec<u32> = (0..4u32)
        .map(|index| e.global::<u32>(WORDS_011A9BE0 + 4 * index))
        .collect();
    for index in [0usize, 3, 1, 2] {
        e.set_global(WORDS_01200800 + 4 * index as u32, words[index]);
    }
    this
}

// Translated from 00bb9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, the vtable `010b9f58`, the
/// array at +0x28 sized to 4 (`00a90180`). Answers `this`.
// Translation incomplete: 00bb9180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb9180(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B9F58);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_8900, fn_00bb8900(Ptr) -> Ptr),
        entry!(0x00bb_9180, fn_00bb9180(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00bb8900_builds_the_effect_and_copies_the_words() {
        let mut e = engine();
        for index in 0..4 {
            e.set_global(WORDS_011A9BE0 + 4 * index, 0x0e00_0010 + index);
        }
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 8]);
        assert_eq!(e.call(0x00bb_8900, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F04);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 0xcc);
        for index in 0..4 {
            assert_eq!(
                e.global::<u32>(WORDS_01200800 + 4 * index),
                0x0e00_0010 + index
            );
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 5]));
    }

    #[test]
    fn fn_00bb9180_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x58);
        assert_eq!(e.call(0x00bb_9180, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F58);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}
