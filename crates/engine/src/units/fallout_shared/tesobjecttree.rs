//! `fallout shared/tesobjecttree.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a5e930`, a destructor that stores the vtable
/// `0109b5ac`, releases the references that vtable slot +0xec returns for each of the
/// entries counted by the half-word at `+0xa6`, and tail-jumps into the base destructor
/// `00a5a390`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00a5e930` ends in `JMP
/// 00a5a390` (which itself ends in `JMP 00a5c240` and then `JMP 00a5d3d0`); no value is
/// computed for the caller and Ghidra types it void.
pub fn fn_0051cfe0(e: &mut Engine, this: Ptr) {
    e.call(0x00a5_e930, &args![this]);
}

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the form
/// type of the record, in `AL`.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the byte at `TESFile` `+0x299`, the swap flag.
const SWAP_FLAG: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESTexture::LoadTextureChunk` (`0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, `cdecl`: component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `00414290` (`ECX` = the `CNAM` block at `+0x6c`): its swap routine.
const SWAP_CNAM: u32 = 0x0041_4290;
/// `00401080` (`cdecl`: address, flag byte): byte-swaps the word at the address.
const SWAP_4_BYTES: u32 = 0x0040_1080;
/// `00401000` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block from `00401000`.
const FREE: u32 = 0x0040_1030;
/// `00403d30` (`cdecl`: destination, value, count): forwards to `_memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `005e03d0` (`ECX` = the list at `+0x5c`): clears it before the `SNAM` entries.
const LIST_CLEAR: u32 = 0x005e_03d0;
/// `0096a610` (`ECX` = the list at `+0x5c`, one word: the address of an entry).
const LIST_ADD: u32 = 0x0096_a610;

/// The form type of the record this loader accepts (`0x25`).
const FORM_TYPE: u8 = 0x25;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_BNAM: u32 = 0x4d41_4e42;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

/// Offsets from `this` of the members the chunks go to.
const MODEL_OFFSET: u32 = 0x30;
const TEXTURE_OFFSET: u32 = 0x48;
const DESTRUCTIBLE_OFFSET: u32 = 0x54;
const LIST_OFFSET: u32 = 0x5c;
const CNAM_OFFSET: u32 = 0x6c;
/// The signed word the `CNAM` chunk handling raises to `0x80` when negative.
const CNAM_LIMIT_OFFSET: u32 = 0x80;
const BNAM_OFFSET: u32 = 0x8c;
const BNAM_SECOND_OFFSET: u32 = 0x90;

/// A member address, or null when the object is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

// Translated from 0051bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051bbc0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESObjectTREE::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`): the `Load`
/// slot of the virtual table `0102bc94` (the form type `0x25` constructor
/// `0051b6e0` stores it). Answers false when the record is not of form type
/// `0x25`; otherwise it runs `TESForm::LoadForm`, reads every chunk and answers
/// true:
/// - `MODL`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`;
/// - `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x48`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x54`
///   (each of these three is null when `this` is null);
/// - `OBND`: virtual slot `0xe0` with the file; `EDID`: read into a stack buffer
///   (at most 0x200 bytes) and passed to virtual slot `0x134`;
/// - `CNAM`: only when the chunk is 0x20 bytes: read into `this + 0x6c`, swapped
///   by `00414290` when the swap flag of the file is set, and the signed word at
///   `+0x80` is set to `0x80` if negative;
/// - `BNAM`: only when the chunk is 8 bytes: read into `this + 0x8c`, and the
///   words at `+0x8c` and `+0x90` are byte-swapped when the swap flag is set;
/// - `SNAM`: when the size is a non-zero multiple of 4, the words are read into
///   a new block (zeroed first), the list at `this + 0x5c` is cleared, and each
///   non-zero word (swapped first when the swap flag is set) is added to it by
///   address (`0096a610`); the block is freed.
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn tes_object_tree_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != FORM_TYPE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_CNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 0x20 {
                    e.call(GET_CHUNK_DATA, &args![file, object + CNAM_OFFSET, 0x20u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_CNAM, &args![object + CNAM_OFFSET]);
                    }
                    if (e.mem.u32(object + CNAM_LIMIT_OFFSET) as i32) < 0 {
                        e.mem.set_u32(object + CNAM_LIMIT_OFFSET, 0x80);
                    }
                }
            }
            CHUNK_BNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 8 {
                    e.call(GET_CHUNK_DATA, &args![file, object + BNAM_OFFSET, 8u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_4_BYTES, &args![object + BNAM_OFFSET, 0u32]);
                        e.call(SWAP_4_BYTES, &args![object + BNAM_SECOND_OFFSET, 0u32]);
                    }
                }
            }
            CHUNK_SNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size != 0 && size % 4 == 0 {
                    let count = size >> 2;
                    // The allocation size is `count * 4`, which cannot overflow.
                    let block = e.call(ALLOCATE, &args![count * 4]).u32();
                    e.call(MEMSET, &args![block, 0u32, size]);
                    e.call(GET_CHUNK_DATA, &args![file, block, size]);
                    e.call(LIST_CLEAR, &args![object + LIST_OFFSET]);
                    for index in 0..count {
                        let entry = block + index * 4;
                        if e.call(SWAP_FLAG, &args![file]).bool() {
                            e.call(SWAP_4_BYTES, &args![entry, 0u32]);
                        }
                        if e.mem.u32(entry) != 0 {
                            e.call(LIST_ADD, &args![object + LIST_OFFSET, entry]);
                        }
                    }
                    e.call(FREE, &args![block]);
                }
            }
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
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL_OFFSET);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_ICON => {
                let texture = component(object, TEXTURE_OFFSET);
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            }
            CHUNK_DSTD | CHUNK_DEST => {
                let target = component(object, DESTRUCTIBLE_OFFSET);
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![target, file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0051d150 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0120044c without reading the receiver. The map names this body `BSShaderTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d150(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_044c
}

// Translated from 0051d1b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01200444 without reading the receiver. The map names this body `BSTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d1b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_0444
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_d150, fn_0051d150(Ptr) -> u32),
        entry!(0x0051_d1b0, fn_0051d1b0(Ptr) -> u32),
        entry!(0x0051_cfe0, fn_0051cfe0(Ptr)),
        entry!(0x0051_bbc0, tes_object_tree_load(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0051cfe0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_cfe0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod load_tests {
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

    struct Record {
        chunks: Vec<u32>,
        form_type: u32,
        null_this: bool,
        chunk_size: u32,
        swap: bool,
        /// Words the chunk data call writes at its buffer.
        data: Vec<u32>,
    }

    impl Record {
        fn of(chunks: Vec<u32>) -> Record {
            Record {
                chunks,
                form_type: 0x25,
                null_this: false,
                chunk_size: 0,
                swap: false,
                data: vec![],
            }
        }
    }

    /// Runs the loader; returns the engine, the call log (without the top-level
    /// call), the address of `this` and the result.
    fn run(record: Record) -> (Engine, CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (record.chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = record.chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        let form_type = record.form_type;
        e.register_double(GET_TES_FORM, move |_, _| ret(0x7700 | form_type));
        for addr in [
            TES_FORM_LOAD_FORM,
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_DESTRUCTIBLE_CHUNK,
            SWAP_CNAM,
            SWAP_4_BYTES,
            FREE,
            LIST_CLEAR,
            LIST_ADD,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        let data = record.data.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            for (i, word) in data.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            Ret::default()
        });
        let chunk_size = record.chunk_size;
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        let swap = record.swap;
        e.register_double(SWAP_FLAG, move |_, _| ret(0x5500 | u32::from(swap)));
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if record.null_this {
            0
        } else {
            let object = e.mem.alloc(0xa0);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x0051_bbc0, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (e, log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn a_record_of_another_form_type_is_refused() {
        let mut record = Record::of(vec![CHUNK_ICON]);
        record.form_type = 0x26;
        let (_, log, _, result) = run(record);
        assert!(!result);
        assert_eq!(log, vec![(GET_TES_FORM, vec![FILE])]);
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (_, log, this, result) = run(Record::of(vec![]));
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[2], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let mut record = Record::of(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555]);
        record.chunk_size = 5;
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn the_components_get_their_chunks() {
        let chunks = vec![CHUNK_MODL, CHUNK_MODT, CHUNK_ICON, CHUNK_DSTD, CHUNK_DEST];
        let (_, log, this, _) = run(Record::of(chunks));
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 2]
        );
        assert_eq!(
            calls_to(&log, LOAD_TEXTURE_CHUNK),
            vec![vec![this + 0x48, FILE]]
        );
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this + 0x54, FILE]; 2]
        );
    }

    #[test]
    fn a_null_object_has_null_components() {
        let mut record = Record::of(vec![CHUNK_MODL, CHUNK_ICON, CHUNK_DEST]);
        record.null_this = true;
        let (_, log, _, _) = run(record);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_TEXTURE_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn cnam_needs_exactly_0x20_bytes_and_raises_a_negative_limit() {
        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x1f;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![0; 8];
        let (e, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x6c, 0x20]]
        );
        assert!(calls_to(&log, SWAP_CNAM).is_empty());
        // The block is 0x20 bytes at +0x6c, so +0x80 is its sixth word.
        assert_eq!(e.mem.u32(this + 0x80), 0);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.swap = true;
        record.data = vec![1, 2, 3, 4, 5, 0xffff_fff0];
        let (e, log, this, _) = run(record);
        assert_eq!(calls_to(&log, SWAP_CNAM), vec![vec![this + 0x6c]]);
        assert_eq!(e.mem.u32(this + 0x80), 0x80);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![1, 2, 3, 4, 5, 0x7fff_ffff];
        let (e, _, this, _) = run(record);
        assert_eq!(e.mem.u32(this + 0x80), 0x7fff_ffff);
    }

    #[test]
    fn bnam_needs_exactly_8_bytes_and_swaps_both_words_on_request() {
        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 4;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x8c, 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        record.swap = true;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![this + 0x8c, 0], vec![this + 0x90, 0]]
        );
    }

    #[test]
    fn snam_adds_the_non_zero_words_and_frees_the_block() {
        for size in [0, 6] {
            let mut record = Record::of(vec![CHUNK_SNAM]);
            record.chunk_size = size;
            let (_, log, _, _) = run(record);
            assert!(calls_to(&log, ALLOCATE).is_empty(), "size {size}");
        }

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 12;
        record.data = vec![0x11, 0, 0x33];
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![12]]);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(calls_to(&log, MEMSET), vec![vec![block, 0, 12]]);
        assert_eq!(calls_to(&log, LIST_CLEAR), vec![vec![this + 0x5c]]);
        assert_eq!(
            calls_to(&log, LIST_ADD),
            vec![vec![this + 0x5c, block], vec![this + 0x5c, block + 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());
        assert_eq!(calls_to(&log, FREE), vec![vec![block]]);

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 8;
        record.swap = true;
        record.data = vec![1, 2];
        let (_, log, _, _) = run(record);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![block, 0], vec![block + 4, 0]]
        );
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0051d150_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d150, &args![0x1000u32]).u32(), 0x0120_044c);
        assert_eq!(e.call(0x0051_d150, &args![0u32]).u32(), 0x0120_044c);
    }

    #[test]
    fn fn_0051d1b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d1b0, &args![0x1000u32]).u32(), 0x0120_0444);
        assert_eq!(e.call(0x0051_d1b0, &args![0u32]).u32(), 0x0120_0444);
    }
}
