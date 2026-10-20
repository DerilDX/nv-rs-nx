//! `(unplaced)/run_005a8080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the embedded struct at `+0x3c` of `this` (`this + 0x3c`).
/// `TESQuest` calls it to reach its `QUEST_DATA`; the engine map has no name
/// for the address, and other classes may share the code.
pub fn fn_005a8080(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x3c)
}

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `TESForm::GetFile` (`00484e60`) and `TESForm::AddCompileIndex`
/// (`00485d50`, cdecl).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The two lists the record's entries go to (`005d43c0`: the `RCOD`
/// entries; `005a8080`: the others) and the condition holder
/// (`006a9540`), each answering an address in `this`.
const RCOD_LIST: u32 = 0x005d_43c0;
const OTHER_LIST: u32 = 0x005d_43e0;
const CONDITIONS: u32 = 0x006a_9540;
/// Empties a list (`00470470`, on the list).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Adds an item to a list (`00905820`, given the item's address).
const LIST_ADD: u32 = 0x0090_5820;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_RCIL: u32 = 0x4c49_4352;
const CHUNK_RCOD: u32 = 0x444f_4352;
const CHUNK_RCQY: u32 = 0x5951_4352;

// Translated from 005a80a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the record (the engine map has no name): empties the condition
/// list (`00680fe0` on `006a9540`'s answer), the lists at `005a8080`,
/// `005d43c0` and `005d43e0` (`00470470`), then `007037c0(-1)`,
/// `0061cc40(0)`, `005a8040(0)` and `005a8060(0)`.
pub fn fn_005a80a0(e: &mut Engine, this: Ptr) {
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(0x0068_0fe0, &args![conditions]);
    let list = fn_005a8080(e, this);
    e.call(LIST_CLEAR, &args![list]);
    let list = e.call(RCOD_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
    let list = e.call(OTHER_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
    e.call(0x0070_37c0, &args![this, 0xffff_ffffu32]);
    e.call(0x0061_cc40, &args![this, 0u32]);
    e.call(0x005a_8040, &args![this, 0u32]);
    e.call(0x005a_8060, &args![this, 0u32]);
}

/// A new 8-byte entry (`00401000`, constructed by `005a7be0`, null when
/// the allocation fails).
fn new_entry(e: &mut Engine) -> u32 {
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(0x005a_7be0, &args![block]).u32()
    }
}

/// A form id chunk read into a zeroed local and adjusted to `this`'s file.
fn read_form_id(e: &mut Engine, this: Ptr, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        let owner = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, owner]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 005a8110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a8110 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// The record reader for form type 0x6A (`RET 4`: the `TESFile`; the
/// engine map has no name): vtable +0x18 first, then false unless the form
/// type byte is 0x6A; otherwise `TESForm::LoadForm`, `00484ab0(0)` and each
/// chunk: `EDID`, `OBND`, `FULL` (+0x18), `DATA` (0x10 bytes at +0x24,
/// swapped by `00414220`), `CTDA` (`TESCondition::Load` on `006a9540`'s
/// answer), `RCIL`/`RCOD` (a new entry, `005a7be0` on 8 allocated bytes,
/// and its form id adjusted to the record's file) and `RCQY` (`00537e90`
/// on the current entry with the chunk's word). After every chunk, while
/// there is a current entry whose `00559450` is not 0 and a form id, the
/// entry gets the id (`006ecd40`) and is added to the `RCOD` list
/// (`005d43c0`) after an `RCOD` chunk, to the list at `005a8080` after an
/// `RCIL` one. Returns true.
pub fn fn_005a8110(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    e.vcall(base, 0x18, &args![]);
    if form_type != 0x6a {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    let entry = e.mem.alloc(4);
    let mut id = 0u32;
    let mut rcod = false;
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_RCOD => {
                rcod = true;
                let new = new_entry(e);
                e.mem.set_u32(entry, new);
                id = read_form_id(e, this, file);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_CTDA => {
                let conditions = e.call(CONDITIONS, &args![this]).u32();
                e.call(0x0068_0960, &args![conditions, file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 0x10u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x0041_4220, &args![base + 0x24]);
                }
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_RCIL => {
                rcod = false;
                let new = new_entry(e);
                e.mem.set_u32(entry, new);
                id = read_form_id(e, this, file);
            }
            CHUNK_FULL => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            CHUNK_RCQY => {
                let word = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                let current = e.mem.u32(entry);
                e.call(0x0053_7e90, &args![current, word]);
            }
            _ => {}
        }
        let current = e.mem.u32(entry);
        if current != 0 && e.call(0x0055_9450, &args![current]).u32() != 0 && id != 0 {
            e.call(0x006e_cd40, &args![current, id]);
            let list = if rcod {
                e.call(RCOD_LIST, &args![this]).u32()
            } else {
                fn_005a8080(e, this).addr()
            };
            e.call(LIST_ADD, &args![list, entry]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.mem.free(entry);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a8080, fn_005a8080(Ptr) -> Ptr),
        entry!(0x005a_80a0, fn_005a80a0(Ptr)),
        entry!(0x005a_8110, fn_005a8110(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_0x3c_to_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005a_8080, &args![0x1000u32]).u32(), 0x103c);
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005a80a0_clears_the_lists_and_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x10));
        e.register(RCOD_LIST, |_, a| ret(a[0] + 0x50));
        e.register(OTHER_LIST, |_, a| ret(a[0] + 0x58));
        quiet(
            &mut e,
            &[
                0x0068_0fe0,
                LIST_CLEAR,
                0x0070_37c0,
                0x0061_cc40,
                0x005a_8040,
                0x005a_8060,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x005a_80a0, &args![this]);
        assert_eq!(calls(&e, 0x0068_0fe0), vec![vec![this + 0x10]]);
        assert_eq!(
            calls(&e, LIST_CLEAR),
            vec![vec![this + 0x3c], vec![this + 0x50], vec![this + 0x58]]
        );
        assert_eq!(calls(&e, 0x0070_37c0), vec![vec![this, 0xffff_ffff]]);
        assert_eq!(calls(&e, 0x0061_cc40), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x005a_8040), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x005a_8060), vec![vec![this, 0]]);
    }

    fn record(form_type: u32, chunks: Vec<(u32, u32)>) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x18u32, 0xe0, 0x134] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // New entries are numbered 0x6000, 0x6100, ...; `00559450` on an
        // entry is non-zero.
        let next = Rc::new(RefCell::new(0x6000u32));
        e.register(0x0040_1000, |_, _| ret(0x5000));
        e.register_double(0x005a_7be0, move |_, _| {
            let entry = *next.borrow();
            *next.borrow_mut() += 0x100;
            ret(entry)
        });
        e.register(0x0055_9450, |_, _| ret(1));
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x10));
        e.register(RCOD_LIST, |_, a| ret(a[0] + 0x50));
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x7000_0018,
                0x7000_00e0,
                0x7000_0134,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                0x0048_4ab0,
                0x0068_0960,
                0x0048_61f0,
                0x0041_4220,
                0x0048_7050,
                0x0053_7e90,
                0x006e_cd40,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_8110, &args![this, file]).bool();
        assert_eq!(result, form_type == 0x6a);
        (e, this, file)
    }

    fn added(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, LIST_ADD)
    }

    #[test]
    fn fn_005a8110_reads_the_chunks() {
        let (e, this, file) = record(
            0x6a,
            vec![
                (CHUNK_EDID, 0),
                (CHUNK_OBND, 0),
                (CHUNK_FULL, 0),
                (CHUNK_DATA, 0),
                (CHUNK_CTDA, 0),
            ],
        );
        assert_eq!(calls(&e, 0x7000_0018), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 0x10]]
        );
        assert_eq!(calls(&e, 0x0041_4220), vec![vec![this + 0x24]]);
        assert_eq!(calls(&e, 0x0068_0960), vec![vec![this + 0x10, file]]);
        assert!(added(&e).is_empty());
    }

    #[test]
    fn fn_005a8110_adds_the_entries_after_each_chunk() {
        let (e, this, _) = record(
            0x6a,
            vec![(CHUNK_RCIL, 0x12), (CHUNK_RCQY, 7), (CHUNK_RCOD, 0x34)],
        );
        assert_eq!(calls(&e, 0x005a_7be0).len(), 2);
        // RCIL: the entry is added to the list at +0x3c after its own chunk
        // and again after the RCQY chunk; RCOD's to the other list.
        let adds: Vec<u32> = added(&e).iter().map(|c| c[0]).collect();
        assert_eq!(adds, vec![this + 0x3c, this + 0x3c, this + 0x50]);
        assert_eq!(
            calls(&e, 0x006e_cd40),
            vec![
                vec![0x6000, 0x0100_0012],
                vec![0x6000, 0x0100_0012],
                vec![0x6100, 0x0100_0034]
            ]
        );
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x6000, 7]]);
    }

    #[test]
    fn fn_005a8110_refuses_other_records() {
        let (e, _, _) = record(0x69, vec![(CHUNK_DATA, 0)]);
        assert_eq!(calls(&e, 0x7000_0018).len(), 1);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}
