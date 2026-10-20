//! `fallout shared/pathfinding/run_006dc890` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSSaveGameBuffer`'s byte save (`00865e50`, thiscall on the buffer:
/// source, size, a third word).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// Offsets the function saves from (as the code reads them): a 12-byte
/// block at +0x00, a second 12-byte block at +0x0c, words at +0x18 and
/// +0x1c and the kind word at +0x20 (only its low byte is saved).
const FIRST_BLOCK: u32 = 0x00;
const SECOND_BLOCK: u32 = 0x0c;
const WORD_18: u32 = 0x18;
const WORD_1C: u32 = 0x1c;
const KIND: u32 = 0x20;

// Translated from 006dc890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object into the save-game buffer `buffer` (thiscall, `RET 4`;
/// unnamed in the engine map) through `00865e50(source, size, 0)`: the low
/// byte of the kind word (+0x20, copied to a stack byte first), the words
/// at +0x18 and +0x1c, the 12 bytes at +0x00 and, when the kind word is 1,
/// the 12 bytes at +0x0c.
pub fn fn_006dc890(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let kind_byte = e.mem.u8(base + KIND);
    e.with_stack(4, |e, local| {
        e.mem.set_u8(local.addr(), kind_byte);
        e.call(SAVE_BYTES, &args![buffer, local, 1u32, 0u32]);
    });
    e.call(SAVE_BYTES, &args![buffer, base + WORD_18, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, base + WORD_1C, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, base + FIRST_BLOCK, 0xcu32, 0u32]);
    if e.mem.u32(base + KIND) == 1 {
        e.call(
            SAVE_BYTES,
            &args![buffer, base + SECOND_BLOCK, 0xcu32, 0u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d_c890, fn_006dc890(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles the byte save: appends the bytes it is given to the
    /// recording at `RECORD` (its length word first).
    const RECORD: u32 = 0x0300_0000;

    fn record_saves(e: &mut Engine) {
        e.map(RECORD, 0x100);
        e.register(SAVE_BYTES, |e, a| {
            assert_eq!(a[0], 0x0abc_0000);
            assert_eq!(a[3], 0);
            let used = e.mem.u32(RECORD);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(RECORD + 4 + used, &bytes);
            e.mem.set_u32(RECORD, used + a[2]);
            Ret::default()
        });
    }

    fn object(e: &mut Engine, kind: u32) -> u32 {
        let this = e.mem.alloc(0x24);
        for i in 0..0x20 {
            e.mem.set_u8(this + i, i as u8 + 1);
        }
        e.mem.set_u32(this + KIND, kind);
        this
    }

    #[test]
    fn saves_the_kind_byte_the_words_and_the_first_block() {
        let mut e = Engine::new();
        record_saves(&mut e);
        let this = object(&mut e, 0x0000_0102);
        e.call(0x006d_c890, &args![this, 0x0abc_0000u32]);
        let used = e.mem.u32(RECORD);
        assert_eq!(used, 1 + 4 + 4 + 12);
        let saved = e.mem.bytes(RECORD + 4, used);
        assert_eq!(saved[0], 0x02);
        assert_eq!(&saved[1..5], &[0x19, 0x1a, 0x1b, 0x1c]);
        assert_eq!(&saved[5..9], &[0x1d, 0x1e, 0x1f, 0x20]);
        assert_eq!(&saved[9..21], &(1..=12).collect::<Vec<u8>>()[..]);
    }

    #[test]
    fn kind_one_also_saves_the_second_block() {
        let mut e = Engine::new();
        record_saves(&mut e);
        let this = object(&mut e, 1);
        e.call(0x006d_c890, &args![this, 0x0abc_0000u32]);
        let used = e.mem.u32(RECORD);
        assert_eq!(used, 1 + 4 + 4 + 12 + 12);
        let saved = e.mem.bytes(RECORD + 4, used);
        assert_eq!(saved[0], 1);
        assert_eq!(&saved[21..33], &(13..=24).collect::<Vec<u8>>()[..]);
    }
}
