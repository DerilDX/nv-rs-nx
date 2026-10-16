//! `fallout shared/magic/run_0040c0e0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a word value (`00404920`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
/// The game-setting vtable.
const GAME_SETTING_VTABLE: u32 = 0x0101_2114;
/// The game-setting collection (`00404a70`: created on first use).
const GAME_SETTINGS: u32 = 0x0040_4a70;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0040c150 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a game setting with a word value (an integer, a
/// string pointer; `thiscall`, `RET 8`): the base constructor (`00404920`)
/// with the name and the value, the game-setting vtable (`01012114`), then
/// adds itself to the game-setting collection (`00404a70`, vtable slot +4).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_0040c150(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), GAME_SETTING_VTABLE);
    let collection = e.call(GAME_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file):
/// reads the record header into the form.
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file): the id
/// of the current chunk, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the byte at `+0x299`.
const FILE_FLAG_299: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `memset` through its wrapper `00403d30` (`cdecl`: buffer, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// The component loader `00487050` (`cdecl`: component, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `00408da0` (`ECX` = the component at `+0x30`): its name text, or the empty
/// string `01011584`.
const FULL_NAME_TEXT: u32 = 0x0040_8da0;
/// `EffectItemList::Load` (Xbox PDB name of `00405760`; `ECX` = the list
/// component at `+0x3c`, the file, the owner's name text).
const EFFECT_ITEM_LIST_LOAD: u32 = 0x0040_5760;
/// `VATS::GetCount` (the engine map name of `005ae380`; `ECX` = a list
/// head): how many nodes of the list hold a non-null item.
const LIST_COUNT: u32 = 0x005a_e380;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;

/// `"GAME EFFECTS: Effect Item (%08X) %s has no effects defined."`
const MESSAGE_NO_EFFECTS: u32 = 0x0101_361c;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_EFID: u32 = 0x4449_4645;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The object's virtual slot called when the file's flag at `+0x299` is set
/// after the sub-object read its own chunk.
const SLOT_AFTER_LOAD: u32 = 0x180;
/// The slots of the object at `+0x30` (the one the name text comes from):
/// the chunk offered to it (file, chunk id), the chunk id it reads itself,
/// the address and the size of the block it reads that chunk into.
const SLOT_OFFER_CHUNK: u32 = 0x40;
const SLOT_OWN_CHUNK_ID: u32 = 0x24;
const SLOT_OWN_BLOCK: u32 = 0x28;
const SLOT_OWN_BLOCK_SIZE: u32 = 0x2c;

// Translated from 0040c650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040c650 - the /GS stack cookie check (00ec408c) is not translated.
/// The engine map gives it no name. `thiscall` (`RET 4`, `file`), answers
/// true: `TESForm::LoadForm`, then for every chunk of the record: an `EFID` or
/// `CTDA` chunk goes to `EffectItemList::Load` on the list at `+0x3c` with the
/// name text of the `+0x30` component; `EDID` is read into a stack buffer
/// (at most 0x200 bytes) and passed to virtual slot `0x134`; `OBND` goes to
/// slot `0xe0`; `FULL` to `00487050` with the `+0x30` component (null for a
/// null object). Every chunk is then offered to the `+0x30` component's slot
/// `0x40`; when it equals the chunk id slot `0x24` of that component names,
/// its block (slots `0x28`, `0x2c`) is zeroed and read from the file, and if
/// the file's flag at `+0x299` is set the object's slot `0x180` runs. Last, a
/// record whose list at `+0x40` holds no item prints the `GAME EFFECTS`
/// message with the form id and name. The stack cookie check is not
/// translated.
pub fn fn_0040c650(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let component = object + 0x30;
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EFID | CHUNK_CTDA => {
                let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
                e.call(EFFECT_ITEM_LIST_LOAD, &args![object + 0x3c, file, name]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                let target = if object == 0 { 0 } else { component };
                e.call(LOAD_FULL_NAME, &args![target, file]);
            }
            _ => {}
        }
        e.vcall(component, SLOT_OFFER_CHUNK, &args![file, chunk]);
        if chunk == e.vcall(component, SLOT_OWN_CHUNK_ID, &[]).u32() {
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(MEMSET, &args![block, 0u32, size]);
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(GET_CHUNK_DATA, &args![file, block, size]);
            if e.call(FILE_FLAG_299, &args![file]).bool() {
                e.vcall(object, SLOT_AFTER_LOAD, &[]);
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(LIST_COUNT, &args![object + 0x40]).u32() == 0 {
        let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
        let form_id = e.call(FORM_ID, &args![this]).u32();
        e.call(PRINT, &args![MESSAGE_NO_EFFECTS, form_id, name]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040c150, fn_0040c150(Ptr, u32, u32) -> Ptr),
        entry!(0x0040c650, fn_0040c650(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_game_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(GAME_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), GAME_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040c150, &[s, 0x66, 7]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x66, 7]));
        assert_eq!(log[2].0, GAME_SETTINGS);
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
        assert_eq!(e.mem.u32(s + 4), 7);
        assert_eq!(e.mem.u32(s + 8), 0x66);
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const COMPONENT_TABLE: u32 = 0x0710_0000;
    const OWN_CHUNK: u32 = 0x1234;
    const OWN_BLOCK: u32 = 0x9000;
    const OWN_BLOCK_SIZE: u32 = 20;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;
    const TARGET_AFTER_LOAD: u32 = 0x0050_0003;
    const TARGET_OFFER_CHUNK: u32 = 0x0050_0004;
    const TARGET_OWN_CHUNK_ID: u32 = 0x0050_0005;
    const TARGET_OWN_BLOCK: u32 = 0x0050_0006;
    const TARGET_OWN_BLOCK_SIZE: u32 = 0x0050_0007;

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

    /// Runs the loader on a record of `chunks`; `count` is what the list
    /// count answers, `flag` the file flag at `+0x299`. Returns the call log
    /// (without the top-level call), the object and the result.
    fn run(chunks: Vec<u32>, count: u32, flag: u32) -> (CallLog, u32, bool) {
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
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            MEMSET,
            LOAD_FULL_NAME,
            EFFECT_ITEM_LIST_LOAD,
            PRINT,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
            TARGET_AFTER_LOAD,
            TARGET_OFFER_CHUNK,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CHUNK_SIZE, |_, _| ret(5));
        e.register_double(FILE_FLAG_299, move |_, _| ret(flag));
        e.register(FULL_NAME_TEXT, |_, _| ret(0x5000));
        e.register(FORM_ID, |_, _| ret(0x77));
        e.register_double(LIST_COUNT, move |_, _| ret(count));
        e.register(TARGET_OWN_CHUNK_ID, |_, _| ret(OWN_CHUNK));
        e.register(TARGET_OWN_BLOCK, |_, _| ret(OWN_BLOCK));
        e.register(TARGET_OWN_BLOCK_SIZE, |_, _| ret(OWN_BLOCK_SIZE));
        let mut object_table = vec![0u32; 0x61];
        object_table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        object_table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        object_table[0x180 / 4] = TARGET_AFTER_LOAD;
        e.put_vtable(OBJECT_TABLE, &object_table);
        let mut component_table = vec![0u32; 0x11];
        component_table[0x24 / 4] = TARGET_OWN_CHUNK_ID;
        component_table[0x28 / 4] = TARGET_OWN_BLOCK;
        component_table[0x2c / 4] = TARGET_OWN_BLOCK_SIZE;
        component_table[0x40 / 4] = TARGET_OFFER_CHUNK;
        e.put_vtable(COMPONENT_TABLE, &component_table);
        let object = e.mem.alloc(0x60);
        e.mem.set_u32(object, OBJECT_TABLE);
        e.mem.set_u32(object + 0x30, COMPONENT_TABLE);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040c650, &args![object, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), object, result)
    }

    #[test]
    fn every_chunk_kind_goes_to_its_reader() {
        let chunks = vec![
            CHUNK_EDID,
            CHUNK_FULL,
            CHUNK_OBND,
            CHUNK_EFID,
            CHUNK_CTDA,
            0x5555_5555,
        ];
        let (log, object, result) = run(chunks, 1, 0);
        assert!(result);
        let component = object + 0x30;
        assert_eq!(log[0], (TES_FORM_LOAD_FORM, vec![object, FILE]));
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![object, data[0][1]]]
        );
        assert_eq!(calls_to(&log, LOAD_FULL_NAME), vec![vec![component, FILE]]);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![object, FILE]]);
        assert_eq!(calls_to(&log, FULL_NAME_TEXT).len(), 2);
        assert_eq!(
            calls_to(&log, EFFECT_ITEM_LIST_LOAD),
            vec![
                vec![object + 0x3c, FILE, 0x5000],
                vec![object + 0x3c, FILE, 0x5000]
            ]
        );
        // Every chunk, known or not, is offered to the component.
        let offered: Vec<u32> = calls_to(&log, TARGET_OFFER_CHUNK)
            .iter()
            .map(|c| {
                assert_eq!(c[..2], [component, FILE]);
                c[2]
            })
            .collect();
        assert_eq!(
            offered,
            vec![
                CHUNK_EDID,
                CHUNK_FULL,
                CHUNK_OBND,
                CHUNK_EFID,
                CHUNK_CTDA,
                0x5555_5555
            ]
        );
        assert!(calls_to(&log, MEMSET).is_empty());
        assert!(calls_to(&log, PRINT).is_empty());
    }

    #[test]
    fn the_components_own_chunk_is_zeroed_and_read_into_its_block() {
        let (log, object, _) = run(vec![0x5555_5555, OWN_CHUNK], 1, 1);
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![OWN_BLOCK, 0, OWN_BLOCK_SIZE]]
        );
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, OWN_BLOCK, OWN_BLOCK_SIZE]]
        );
        assert_eq!(calls_to(&log, FILE_FLAG_299), vec![vec![FILE]]);
        assert_eq!(calls_to(&log, TARGET_AFTER_LOAD), vec![vec![object]]);
    }

    #[test]
    fn the_after_load_slot_needs_the_file_flag() {
        let (log, _, _) = run(vec![OWN_CHUNK], 1, 0);
        assert_eq!(calls_to(&log, FILE_FLAG_299).len(), 1);
        assert!(calls_to(&log, TARGET_AFTER_LOAD).is_empty());
    }

    #[test]
    fn a_record_without_chunks_reports_that_it_has_no_effects() {
        let (log, object, result) = run(vec![], 0, 0);
        assert!(result);
        assert!(calls_to(&log, TARGET_OFFER_CHUNK).is_empty());
        assert_eq!(calls_to(&log, LIST_COUNT), vec![vec![object + 0x40]]);
        assert_eq!(
            calls_to(&log, PRINT),
            vec![vec![MESSAGE_NO_EFFECTS, 0x77, 0x5000]]
        );
    }
}
