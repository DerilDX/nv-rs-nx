//! `fallout shared/bgsmovablestatic.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the
/// form type of the record (the result is not used here).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file): the id
/// of the current chunk, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`; `ECX` = file, the
/// address of a word): reads the chunk into the word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00403570` (`ECX` = file): a 16-bit value (the result is in `AX`) that the
/// texture swap unpacking takes as its last argument.
const FILE_WORD: u32 = 0x0040_3570;
/// The component loader `00487050` (`cdecl`: component, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`; `cdecl`: model,
/// file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB name of
/// `0048a7b0`; `ECX` = the model at `+0x30`, buffer, size, a 16-bit value).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB name of `004781e0`;
/// `cdecl`: destructible component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`; `ECX` = form, file,
/// destination, a flag).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `004fbf00` (`ECX` = the object at `this - 0x14`, one word): takes the
/// `SNAM` value.
const SET_SOUND: u32 = 0x004f_bf00;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_SNAM: u32 = 0x4d41_4e53;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;

/// How far below `this` the object the chunk readers work on starts.
const OBJECT_OFFSET: u32 = 0x14;
/// `this + 0x30`: the model component the model and texture swap chunks go to.
const MODEL_OFFSET: u32 = 0x30;
/// `this - 8`: the destructible component the destruction chunks go to.
const DESTRUCTIBLE_BACKWARD: u32 = 8;
/// `this + 0x5c`: where the `DATA` chunk is read to.
const DATA_OFFSET: u32 = 0x5c;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

// Translated from 004fbc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fbc70 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSMovableStatic::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`), answers
/// true. The code addresses the object relative to `this`: the object the
/// readers are given starts at `this - 0x14`, and a `this` of exactly `0x14`
/// counts as a null object for the model and destruction components. It reads
/// the form header (`TESFile::GetTESForm` is called and its result dropped,
/// `TESForm::LoadForm`), then every chunk of the record:
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`;
/// - `MODS`: the chunk is read into a stack buffer of its size and unpacked as
///   a texture swap on `this + 0x30`;
/// - `OBND`: virtual slot `0xe0` with the file;
/// - `EDID`: read into a stack buffer (at most 0x200 bytes) and passed to
///   virtual slot `0x134`;
/// - `FULL`: `00487050` on `this - 0x14`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this - 8`;
/// - `DATA`: `TESForm::LoadData` with the destination `this + 0x5c`;
/// - `SNAM`: the chunk word handed to `004fbf00` on `this - 0x14`.
///
/// The stack cookie check is not translated.
pub fn bgs_movable_static_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let base = object.wrapping_sub(OBJECT_OFFSET);
    // `this - 0x14` null: the model and destruction components are null.
    let model = if base == 0 { 0 } else { object + MODEL_OFFSET };
    let destructible = if base == 0 {
        0
    } else {
        object.wrapping_sub(DESTRUCTIBLE_BACKWARD)
    };
    e.call(GET_TES_FORM, &args![file]);
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![base, file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![object + MODEL_OFFSET, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![destructible, file]);
            }
            CHUNK_DATA => {
                e.call(
                    TES_FORM_LOAD_DATA,
                    &args![this, file, object + DATA_OFFSET, 1u32],
                );
            }
            CHUNK_SNAM => {
                let value = e.with_stack(4, |e, word| {
                    e.mem.set_u32(word.addr(), 0);
                    e.call(GET_CHUNK_WORD, &args![file, word]);
                    e.mem.u32(word.addr())
                });
                e.call(SET_SOUND, &args![base, value]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004fbc70, bgs_movable_static_load(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Runs the loader on a record of `chunks` with the object at `this`
    /// (when 0, a real object is allocated and its address used; the object
    /// holds the vtable at +0 either way). Returns the call log (without the
    /// top-level call), the address of `this` and the result.
    fn run(chunks: Vec<u32>, this: u32, chunk_size: u32) -> (CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        for addr in [
            GET_TES_FORM,
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            LOAD_DESTRUCTIBLE_CHUNK,
            TES_FORM_LOAD_DATA,
            SET_SOUND,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        e.register(FILE_WORD, |_, _| ret(0xabcd_0007));
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x0bad_f00d);
            Ret::default()
        });
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if this == 0 {
            let object = e.mem.alloc(0x80);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        } else {
            this
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x004fbc70, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (log, this, result) = run(vec![], 0, 0);
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[2], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let (log, this, _) = run(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555], 0, 5);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        // Nothing else is read for the unknown chunk.
        assert!(calls_to(&log, LOAD_MODEL_CHUNK).is_empty());
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn full_data_and_snam_use_offsets_from_this() {
        let (log, this, _) = run(vec![CHUNK_FULL, CHUNK_DATA, CHUNK_SNAM], 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_FULL_NAME),
            vec![vec![this - 0x14, FILE]]
        );
        assert_eq!(
            calls_to(&log, TES_FORM_LOAD_DATA),
            vec![vec![this, FILE, this + 0x5c, 1]]
        );
        let word = calls_to(&log, GET_CHUNK_WORD);
        assert_eq!(word.len(), 1);
        assert_eq!(word[0][0], FILE);
        assert_eq!(
            calls_to(&log, SET_SOUND),
            vec![vec![this - 0x14, 0x0bad_f00d]]
        );
    }

    #[test]
    fn the_model_chunks_go_to_the_model_component() {
        let kinds = vec![CHUNK_MODL, CHUNK_MODD, CHUNK_MOSD, CHUNK_MODT];
        let (log, this, _) = run(kinds, 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 4]
        );
    }

    #[test]
    fn the_destruction_chunks_go_to_the_destructible_component() {
        let (log, this, _) = run(vec![CHUNK_DEST, CHUNK_DSTD], 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this - 8, FILE]; 2]
        );
    }

    #[test]
    fn a_this_of_0x14_counts_as_a_null_object_for_the_components() {
        let (log, _, _) = run(vec![CHUNK_MODL, CHUNK_DEST], 0x14, 0);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn a_texture_swap_chunk_is_unpacked_from_a_buffer_of_its_size() {
        let (log, this, _) = run(vec![CHUNK_MODS], 0, 20);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 20));
        assert_eq!(
            calls_to(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![this + 0x30, data[0][1], 20, 7]]
        );
    }
}
