//! `fallout shared/tesobjectmisc.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `0051aaa0` tells apart.
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
const DATA: u32 = u32::from_le_bytes(*b"DATA");
const ICON: u32 = u32::from_le_bytes(*b"ICON");
const MICO: u32 = u32::from_le_bytes(*b"MICO");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const YNAM: u32 = u32::from_le_bytes(*b"YNAM");
const ZNAM: u32 = u32::from_le_bytes(*b"ZNAM");

// Translated from 0051aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051aaa0 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `tesobjectmisc.cpp` (the engine map gives the function no name, so the class
/// is unconfirmed). `this` is the form and `file` the `TESFile` positioned on its record.
/// Always answers true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `ICON`: `0048e3d0` on `+0x5c`; `MICO`: `0047a290` on `+0x8c`;
/// - `SCRI`: reads a word, `006ecd40` on `+0x68` with it and `0048cd90` on `+0x68` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x84`;
/// - `DATA`: `TESForm::LoadData` (`004861f0`) with a null buffer and size 0;
/// - `YNAM`: reads a word and runs `006ecd40` on `+0x9c`; `ZNAM`: reads a word and runs
///   `00403550` on `+0x9c`; `RNAM`: reads a word and gives it to `009db930` on the form.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_0051aaa0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
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
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            ICON => {
                let target = if base != 0 { base + 0x5c } else { 0 };
                e.call(0x0048_e3d0, &args![target, file]);
            }
            MICO => {
                let target = if base != 0 { base + 0x8c } else { 0 };
                e.call(0x0047_a290, &args![target, file]);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x68, value]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x84 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            YNAM => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x9c, value]);
            }
            ZNAM => {
                let value = read_word(e, file);
                e.call(0x0040_3550, &args![base + 0x9c, value]);
            }
            DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x009d_b930, &args![this, value]);
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
    vec![entry!(0x0051_aaa0, fn_0051aaa0(Ptr, Ptr) -> bool)]
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
                0x0048_e3d0,
                0x0047_a290,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x0040_3550,
                0x0048_61f0,
                0x009d_b930,
            ],
        );
        let this = load_test::object(
            &mut e,
            0xa0,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_aaa0, &[this, FILE]);
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
            (tag(b"ICON"), 0x0048_e3d0, 0x5c),
            (tag(b"MICO"), 0x0047_a290, 0x8c),
            (tag(b"DSTD"), 0x0047_81e0, 0x84),
            (tag(b"DEST"), 0x0047_81e0, 0x84),
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
            [vec![this + 0x68, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x68, this]]);

        let (_, this, _, calls) = load(&[tag(b"YNAM")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x9c, CHUNK_VALUE]]
        );
        let (_, this, _, calls) = load(&[tag(b"ZNAM")]);
        assert_eq!(
            calls_to(&calls, 0x0040_3550),
            [vec![this + 0x9c, CHUNK_VALUE]]
        );
        let (_, this, _, calls) = load(&[tag(b"RNAM")]);
        assert_eq!(calls_to(&calls, 0x009d_b930), [vec![this, CHUNK_VALUE]]);
    }

    #[test]
    fn the_data_chunk_loads_nothing() {
        let (_, this, _, calls) = load(&[tag(b"DATA")]);
        assert_eq!(calls_to(&calls, 0x0048_61f0), [vec![this, FILE, 0, 0]]);
    }

    #[test]
    fn unknown_chunks_are_skipped() {
        let (_, _, answer, calls) = load(&[tag(b"XXXX"), tag(b"YYYY")]);
        assert!(answer);
        assert_eq!(calls.len(), 2);
    }
}
