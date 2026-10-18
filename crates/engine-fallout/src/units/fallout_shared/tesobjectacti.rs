//! `fallout shared/tesobjectacti.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `00510a90` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const FULL: u32 = u32::from_le_bytes(*b"FULL");
const SCRI: u32 = u32::from_le_bytes(*b"SCRI");
const DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const DEST: u32 = u32::from_le_bytes(*b"DEST");
const INAM: u32 = u32::from_le_bytes(*b"INAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const VNAM: u32 = u32::from_le_bytes(*b"VNAM");
const WNAM: u32 = u32::from_le_bytes(*b"WNAM");
const XATO: u32 = u32::from_le_bytes(*b"XATO");

// Translated from 00510a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00510a90 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `tesobjectacti.cpp` (the engine map gives the function
/// no name, so the class is unconfirmed). `this` is the form and `file` the `TESFile`
/// positioned on its record. Always answers true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x5c` with it and `0048cd90` on `+0x5c` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x68`;
/// - `INAM`, `RNAM`, `SNAM`, `VNAM`, `WNAM`: reads a word and gives it to `004febd0`,
///   `004fb090`, `004febb0`, `00442a80` and `008ac890` on the form;
/// - `XATO`: when the chunk is longer than one byte, reads it into a heap block (`00401000`,
///   freed with `00401030`), builds an 8 byte string from it (`0040c0e0`) and passes that by
///   value to `00510ea0`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_00510a90(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            INAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebd0, &args![this, value]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x68 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x5c, value]);
                e.call(0x0048_cd90, &args![base + 0x5c, this]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x004f_b090, &args![this, value]);
            }
            SNAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebb0, &args![this, value]);
            }
            VNAM => {
                let value = read_word(e, file);
                e.call(0x0044_2a80, &args![this, value]);
            }
            WNAM => {
                let value = read_word(e, file);
                e.call(0x008a_c890, &args![this, value]);
            }
            XATO => {
                let size = e.call(0x0040_1660, &args![file]).i32();
                if size > 1 {
                    let buffer = e.call(0x0040_1000, &args![size]).u32();
                    e.call(0x0047_2890, &args![file, buffer, size]);
                    let text = e.mem.alloc(8);
                    e.call(0x0040_c0e0, &args![text, buffer]);
                    let (first, second) = (e.mem.u32(text), e.mem.u32(text + 4));
                    e.mem.free(text);
                    e.call(0x0051_0ea0, &args![this, first, second]);
                    e.call(0x0040_1030, &args![buffer]);
                }
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// A zeroed local read with `TESFile::GetChunkData` (`004727f0`).
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    let value = e.mem.u32(local);
    e.mem.free(local);
    value
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_0a90, fn_00510a90(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a form and a file holding `chunks`. The form's virtual slots `+0xe0`
    /// and `+0x134` are the doubles `0x0090_0500` and `0x0090_0600`.
    fn load(chunks: &[u32]) -> Loaded {
        let mut e = load_test::engine(0, chunks, false);
        stub(
            &mut e,
            &[
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x004f_ebd0,
                0x004f_b090,
                0x004f_ebb0,
                0x0044_2a80,
                0x008a_c890,
                0x0051_0ea0,
                0x0040_1030,
            ],
        );
        // `00401000` hands out a block, `0040c0e0` builds the two words of the string in `text`.
        e.register(0x0040_1000, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(0x0040_c0e0, |e, a| {
            e.mem.set_u32(a[0], 0x1111_2222);
            e.mem.set_u32(a[0] + 4, 0x3333_4444);
            Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x70,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_0a90, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    #[test]
    fn an_empty_record_loads_the_form_and_answers_true() {
        let (_, this, answer, calls) = load(&[]);
        assert!(answer);
        assert_eq!(
            calls,
            [
                (0x0048_5110, vec![this, FILE]),
                (0x0048_4ab0, vec![this, 0])
            ]
        );
    }

    #[test]
    fn the_buffer_and_virtual_chunks() {
        let (_, this, _, calls) = load(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = load(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = load(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x3c, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_components_get_their_chunks() {
        for (chunk, callee, offset) in [
            (tag(b"FULL"), 0x0048_7050, 0x30),
            (tag(b"MODL"), 0x0048_92d0, 0x3c),
            (tag(b"MODD"), 0x0048_92d0, 0x3c),
            (tag(b"MOSD"), 0x0048_92d0, 0x3c),
            (tag(b"MODT"), 0x0048_92d0, 0x3c),
            (tag(b"DSTD"), 0x0047_81e0, 0x68),
            (tag(b"DEST"), 0x0047_81e0, 0x68),
        ] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_word_chunks_reach_their_setters() {
        let (_, this, _, calls) = load(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x5c, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x5c, this]]);
        for (chunk, callee) in [
            (tag(b"INAM"), 0x004f_ebd0),
            (tag(b"RNAM"), 0x004f_b090),
            (tag(b"SNAM"), 0x004f_ebb0),
            (tag(b"VNAM"), 0x0044_2a80),
            (tag(b"WNAM"), 0x008a_c890),
        ] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(calls_to(&calls, callee), [vec![this, CHUNK_VALUE]]);
        }
        // `TNAM` and `UNAM` are not read.
        let (_, _, _, calls) = load(&[tag(b"TNAM"), tag(b"UNAM")]);
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn the_xato_chunk_becomes_a_string_passed_by_value() {
        let (_, this, _, calls) = load(&[tag(b"XATO")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        let buffer = read[0][1];
        assert_eq!(
            calls_to(&calls, 0x0051_0ea0),
            [vec![this, 0x1111_2222, 0x3333_4444]]
        );
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![buffer]]);
        assert_eq!(calls_to(&calls, 0x0040_1000), [vec![CHUNK_SIZE]]);
    }

    #[test]
    fn a_one_byte_xato_chunk_is_ignored() {
        let mut e = load_test::engine(0, &[tag(b"XATO")], false);
        stub(&mut e, &[0x0048_5110, 0x0048_4ab0]);
        e.register(0x0040_1660, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x70);
        let (_, calls) = run(&mut e, 0x0051_0a90, &[this, FILE]);
        assert_eq!(calls.len(), 2);
    }
}
