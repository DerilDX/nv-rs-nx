//! `fallout shared/tesobjectdoor.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) `TESObjectDOOR::Load` tells apart.
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
const ANAM: u32 = u32::from_le_bytes(*b"ANAM");
const BNAM: u32 = u32::from_le_bytes(*b"BNAM");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const FNAM: u32 = u32::from_le_bytes(*b"FNAM");
const TNAM: u32 = u32::from_le_bytes(*b"TNAM");

// Translated from 005177f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005177f0 - the security-cookie check (00ec408c) at the end is not translated.
/// `TESObjectDOOR::Load` (Xbox PDB). `this` is the form and `file` the `TESFile` positioned on
/// its record. Answers false without touching the form when the record is not of type 0x1c,
/// otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x5c` with it and `0048cd90` on `+0x5c` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x68`;
/// - `SNAM`, `ANAM`, `BNAM`: read a form id, give it the file's compile index
///   (`TESForm::AddCompileIndex` `00485d50`), look the form up (`004839c0`), cast it with
///   `__RTDynamicCast` from the type at `0x01183028` to the type at `0x011832fc` and store the
///   result at `+0x78`, `+0x7c` and `+0x80`;
/// - `FNAM`: reads one byte into `+0x84`;
/// - `TNAM`: reads a word and runs `005ae3d0` on `+0x88` with the address of a copy of it.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn tesobjectdoor_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u8() != 0x1c {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
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
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
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
            DSTD | DEST => {
                let target = if base != 0 { base + 0x68 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            SNAM => read_form_reference(e, file, base + 0x78),
            ANAM => read_form_reference(e, file, base + 0x7c),
            BNAM => read_form_reference(e, file, base + 0x80),
            FNAM => {
                e.call(0x0047_2890, &args![file, base + 0x84, 1u32]);
            }
            TNAM => {
                let value = read_word(e, file);
                let copy = e.mem.alloc(4);
                e.mem.set_u32(copy, value);
                e.call(0x005a_e3d0, &args![base + 0x88, copy]);
                e.mem.free(copy);
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

/// Reads a form id, gives it the file's compile index, looks it up (`004839c0`), casts the form
/// (`__RTDynamicCast`, type at `0x01183028` to the type at `0x011832fc`) and stores the result
/// in the pointer at `field`.
fn read_form_reference(e: &mut Engine, file: Ptr, field: u32) {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    e.call(0x0048_5d50, &args![local, file]);
    let id = e.mem.u32(local);
    e.mem.free(local);
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_32fcu32, 0u32],
        )
        .u32();
    e.mem.set_u32(field, cast);
}

// Translated from 00519b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int_bool>` scalar deleting destructor (engine map name): runs the destructor body `00519bf0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_pointer_map_int_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0051_9bf0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00519c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>_int_bool>` scalar deleting destructor (engine map name): runs the destructor body `00519c50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_ni_t_pointer_allocator_unsigned_int_int_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0051_9c50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_77f0, tesobjectdoor_load(Ptr, Ptr) -> bool),
        entry!(0x0051_9b50, ni_t_pointer_map_int_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0051_9c80, ni_t_map_base_ni_t_pointer_allocator_unsigned_int_int_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a door form and a file holding `chunks` of a record of `form_type`.
    /// The form's virtual slots `+0xe0` and `+0x134` are the doubles `0x0090_0500` and
    /// `0x0090_0600`. The lookup `004839c0` answers the id xor `0x1000_0000`, the compile
    /// index goes into the id's top byte and the cast keeps the lookup's answer.
    fn load(form_type: u32, chunks: &[u32]) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, false);
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
            ],
        );
        // `005ae3d0` is given the address of a copy of the word: it keeps the word it finds there.
        e.register(0x005a_e3d0, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] ^ 0x1000_0000,
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x90,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_77f0, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn door(chunks: &[u32]) -> Loaded {
        load(0x1c, chunks)
    }

    #[test]
    fn another_form_type_is_refused_before_anything_is_loaded() {
        let (_, _, answer, calls) = load(0x1b, &[tag(b"EDID")]);
        assert!(!answer);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_form_is_loaded_first_and_the_flag_bit_cleared() {
        let (_, this, answer, calls) = door(&[]);
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
        let (_, this, _, calls) = door(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = door(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = door(&[tag(b"MODS")]);
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
            let (_, this, _, calls) = door(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_script_chunk_sets_the_script_and_initialises_it() {
        let (_, this, _, calls) = door(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x5c, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x5c, this]]);
    }

    #[test]
    fn the_sound_chunks_store_the_cast_form() {
        for (chunk, offset) in [
            (tag(b"SNAM"), 0x78),
            (tag(b"ANAM"), 0x7c),
            (tag(b"BNAM"), 0x80),
        ] {
            let (e, this, _, calls) = door(&[chunk]);
            // 0x12345678 with the compile index, then the lookup's xor.
            let id = CHUNK_VALUE | 0x0100_0000;
            assert_eq!(e.mem.u32(this + offset), id ^ 0x1000_0000);
            assert_eq!(calls_to(&calls, 0x0048_39c0), [vec![id]]);
            assert_eq!(
                calls_to(&calls, 0x00ec_43fb),
                [vec![id ^ 0x1000_0000, 0, 0x0118_3028, 0x0118_32fc, 0]]
            );
            let index = calls_to(&calls, 0x0048_5d50);
            assert_eq!((index.len(), index[0][1]), (1, FILE));
        }
    }

    #[test]
    fn the_flag_and_trigger_chunks() {
        let (_, this, _, calls) = door(&[tag(b"FNAM")]);
        assert_eq!(calls_to(&calls, 0x0047_2890), [vec![FILE, this + 0x84, 1]]);

        let (e, this, _, calls) = door(&[tag(b"TNAM")]);
        let call = calls_to(&calls, 0x005a_e3d0);
        assert_eq!(call.len(), 1);
        assert_eq!(call[0][0], this + 0x88);
        assert_eq!(e.mem.u32(this + 0x88), CHUNK_VALUE);
    }

    #[test]
    fn unknown_chunks_are_skipped() {
        let (_, _, answer, calls) = door(&[tag(b"XXXX"), tag(b"YYYY")]);
        assert!(answer);
        assert_eq!(calls.len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00519b50_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_9bf0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_9b50, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_9b50)
                .collect();
            let mut expected = vec![(0x0051_9bf0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00519c80_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_9c50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_9c80, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_9c80)
                .collect();
            let mut expected = vec![(0x0051_9c50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
