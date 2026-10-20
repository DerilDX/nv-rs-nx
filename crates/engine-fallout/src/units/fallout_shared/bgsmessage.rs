//! `fallout shared/bgsmessage.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the form
/// flag setter.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (`00484e60`), `TESForm::AddCompileIndex` (`00485d50`,
/// cdecl), the form lookup by id (`004839c0`, cdecl), `__RTDynamicCast`
/// (`00ec43fb`, cdecl, five words) from `.?AVTESForm@@` to
/// `.?AVBGSMenuIcon@@` (`011863f0`), and the form id (`0084e3a0`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_MENU_ICON: u32 = 0x0118_63f0;
const FORM_ID: u32 = 0x0084_e3a0;
/// The message's icon: its form id or pointer (`0055b980`) and its setter
/// (`0086cf00`).
const ICON_GET: u32 = 0x0055_b980;
const ICON_SET: u32 = 0x0086_cf00;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find icon (%08X) on weapon '%s' (%08X)."`
const MISSING_ICON_FORMAT: u32 = 0x0103_38fc;
/// `"MASTERFILE: Errors were encountered during InitItem for conditions on
/// menu item %d for terminal ..."`
const ITEM_CONDITIONS_FORMAT: u32 = 0x0102_5a98;
/// The warning counter (`0046e8a0`, no arguments), the warning
/// suppression (`MessageHandler::IncDisableWarningCount`, `0043b2b0`,
/// cdecl) and the counter's setter (`004fffe0`, cdecl).
const WARNING_COUNT: u32 = 0x0046_e8a0;
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
const SET_WARNING_COUNT: u32 = 0x004f_ffe0;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_INAM: u32 = 0x4d41_4e49;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_TNAM: u32 = 0x4d41_4e54;
const CHUNK_ITXT: u32 = 0x5458_5449;
const CHUNK_CTDA: u32 = 0x4144_5443;

// Translated from 005906f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessage::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// resolves the icon's form id (adjusted to the message's file, looked up
/// and cast to `BGSMenuIcon`; a warning with the id, the name and the form
/// id when nothing is found), then runs each menu item's conditions
/// (`TESCondition::InitItem` on the item's +8) with warnings suppressed and
/// warns (with the item's index) when the warning count changed, and
/// `00484ab0(1)`.
pub fn bgs_message_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let icon = e.call(ICON_GET, &args![this]).u32();
    if icon != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), icon);
            let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_MENU_ICON, 0u32],
            )
            .u32();
        e.call(ICON_SET, &args![this, cast]);
        if e.call(ICON_GET, &args![this]).u32() == 0 {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            e.call(WARNING, &args![MISSING_ICON_FORMAT, id, name, form_id]);
        }
    }
    let mut node = e.call(0x0043_6aa0, &args![this]).u32();
    let mut index = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(slot);
        node = e.call(0x0072_6070, &args![node]).u32();
        e.call(0x0068_0a20, &args![item.wrapping_add(8), this]);
        let before = e.call(WARNING_COUNT, &args![]).u32();
        e.call(DISABLE_WARNINGS, &args![1u32]);
        e.call(SET_WARNING_COUNT, &args![0u32]);
        e.call(DISABLE_WARNINGS, &args![0u32]);
        let after = e.call(WARNING_COUNT, &args![]).u32();
        e.call(SET_WARNING_COUNT, &args![before]);
        if after != 0 {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            e.call(
                WARNING,
                &args![ITEM_CONDITIONS_FORMAT, index, name, form_id],
            );
        }
        index = index.wrapping_add(1);
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 00590880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590880 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `BGSMessage::Load` (Xbox PDB): reads the form type (not checked),
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID` (vtable
/// +0x134), `OBND` (vtable +0xE0), `FULL` (+0x18, `00487050`), `DESC`
/// (+0x24, `00482d50`), `INAM` (the icon's form id, `0086cf00`), `DNAM`
/// (+0x38), `TNAM` (+0x3C), `ITXT` (a new menu item, 0x10 bytes built by
/// `00590b60`, its text set by `004037f0`, added by `005911d0`) and `CTDA`
/// (a condition for the last item, `00680960` on its +8; none before the
/// first item). Returns true.
pub fn bgs_message_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(FILE_FORM_TYPE, &args![file]);
    e.call(0x0048_5110, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    let or_null = |offset: u32| if this.is_null() { 0 } else { base + offset };
    let mut item = 0u32;
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                e.call(0x0048_7050, &args![or_null(0x18), file]);
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
                if item != 0 {
                    e.call(0x0068_0960, &args![item + 8, file]);
                }
            }
            CHUNK_DESC => {
                e.call(0x0048_2d50, &args![or_null(0x24), file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_TNAM => {
                e.call(FILE_CHUNK_DATA, &args![file, base + 0x3c]);
            }
            CHUNK_DNAM => {
                e.call(FILE_CHUNK_DATA, &args![file, base + 0x38]);
            }
            CHUNK_INAM => {
                let id = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.call(ICON_SET, &args![this, id]);
            }
            CHUNK_ITXT => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                item = e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0u32]);
                    let block = e.call(0x0040_1000, &args![0x10u32]).u32();
                    let new_item = if block == 0 {
                        0
                    } else {
                        e.call(0x0059_0b60, &args![block]).u32()
                    };
                    e.call(0x0040_37f0, &args![new_item, text, 0u32]);
                    e.call(0x0059_11d0, &args![this, new_item]);
                    new_item
                });
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00590b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590b60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of a message's menu item (0x10 bytes, `BGSMessage::Load`
/// makes them): the text string (`004037b0`) and the condition at +8
/// (`00680890`). Returns `this`.
pub fn fn_00590b60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_37b0, &args![this]);
    e.call(0x0068_0890, &args![this.addr() + 8]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_06f0, bgs_message_init_item(Ptr)),
        entry!(0x0059_0880, bgs_message_load(Ptr, Ptr) -> bool),
        entry!(0x0059_0b60, fn_00590b60(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
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

    /// A message whose vtable slots +0x130, +0x134 and +0xE0 are doubles.
    fn message(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    #[test]
    fn fn_00590b60_builds_the_item() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0040_37b0, 0x0068_0890]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_0b60, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(calls(&e, 0x0040_37b0), vec![vec![0x4000]]);
        assert_eq!(calls(&e, 0x0068_0890), vec![vec![0x4008]]);
    }

    /// Doubles for the icon (a word at +0x30 of the message), the form
    /// lookups, a two-item list and the warning counter (the second item's
    /// conditions add a warning).
    fn init_doubles(e: &mut Engine, found: bool) {
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        e.register(ICON_GET, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(ICON_SET, |e, a| {
            e.mem.set_u32(a[0] + 0x30, a[1]);
            Ret::default()
        });
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        if found {
            e.register(LOOKUP_FORM, |_, a| ret(a[0] & 0xffff));
        } else {
            e.register(LOOKUP_FORM, |_, _| ret(0));
        }
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_ID, |_, _| ret(0x0012_3456));
        quiet(e, &[WARNING, FORM_SET_FLAG, DISABLE_WARNINGS]);
        // Nodes: the list at `this + 0x20` holds items 0x9000 and 0x9100.
        let nodes = e.mem.alloc(8);
        e.mem.set_u32(nodes, 0x9000);
        e.mem.set_u32(nodes + 4, 0x9100);
        e.register_double(0x0043_6aa0, move |_, _| ret(nodes));
        e.register(0x0082_56d0, |_, _| ret(0));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register_double(0x0072_6070, move |_, a| {
            ret(if a[0] == nodes { nodes + 4 } else { 0 })
        });
        let count = Rc::new(RefCell::new(7u32));
        let current = Rc::new(RefCell::new(0u32));
        let c = current.clone();
        e.register_double(0x0068_0a20, move |_, a| {
            *c.borrow_mut() = a[0];
            Ret::default()
        });
        let (n, c) = (count.clone(), current);
        e.register_double(WARNING_COUNT, move |_, _| {
            // The second item's conditions leave one warning behind.
            if *n.borrow() == 0 && *c.borrow() == 0x9108 {
                *n.borrow_mut() = 1;
            }
            ret(*n.borrow())
        });
        let n = count;
        e.register_double(SET_WARNING_COUNT, move |_, a| {
            *n.borrow_mut() = a[0];
            Ret::default()
        });
    }

    #[test]
    fn init_item_resolves_the_icon_and_checks_each_item() {
        let mut e = Engine::new();
        let this = message(&mut e);
        e.mem.set_u32(this + 0x30, 0x77);
        init_doubles(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0059_06f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x30), 0x77);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x77, 0, TYPE_TES_FORM, TYPE_MENU_ICON, 0]]
        );
        assert_eq!(
            calls(&e, 0x0068_0a20),
            vec![vec![0x9008, this], vec![0x9108, this]]
        );
        assert_eq!(
            calls(&e, DISABLE_WARNINGS),
            vec![vec![1], vec![0], vec![1], vec![0]]
        );
        // Only the second item warns; the count is put back each time.
        assert_eq!(
            calls(&e, WARNING),
            vec![vec![ITEM_CONDITIONS_FORMAT, 1, 0x5555, 0x0012_3456]]
        );
        assert_eq!(
            calls(&e, SET_WARNING_COUNT),
            vec![vec![0], vec![7], vec![0], vec![7]]
        );
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
    }

    #[test]
    fn init_item_warns_about_a_missing_icon() {
        let mut e = Engine::new();
        let this = message(&mut e);
        e.mem.set_u32(this + 0x30, 0x77);
        init_doubles(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0059_06f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(
            calls(&e, WARNING)[0],
            vec![MISSING_ICON_FORMAT, 0x0100_0077, 0x5555, 0x0012_3456]
        );

        // No icon: no lookup; skipped: nothing.
        let mut e = Engine::new();
        let this = message(&mut e);
        init_doubles(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0059_06f0, &args![this]);
        assert!(calls(&e, LOOKUP_FORM).is_empty());
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0059_06f0, &args![this]);
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
    }

    #[test]
    fn load_reads_the_chunks_and_builds_the_items() {
        let mut e = Engine::new();
        let this = message(&mut e);
        let chunks = Rc::new(RefCell::new((
            vec![
                (CHUNK_CTDA, 0u32),
                (CHUNK_EDID, 0),
                (CHUNK_OBND, 0),
                (CHUNK_FULL, 0),
                (CHUNK_DESC, 0),
                (CHUNK_INAM, 0x42),
                (CHUNK_DNAM, 3),
                (CHUNK_TNAM, 4),
                (CHUNK_ITXT, 0x6f6c),
                (CHUNK_CTDA, 0),
            ],
            0usize,
        )));
        let file = e.mem.alloc(0x10);
        e.register(FILE_FORM_TYPE, |_, _| ret(0x31));
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
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register(0x0040_1000, |_, _| ret(0x6000));
        e.register(0x0059_0b60, |_, a| ret(a[0]));
        quiet(
            &mut e,
            &[
                0x0048_5110,
                FORM_SET_FLAG,
                0x0048_7050,
                0x0048_2d50,
                ICON_SET,
                0x0040_37f0,
                0x0059_11d0,
                0x0068_0960,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_0880, &args![this, file]).bool());
        assert_eq!(calls(&e, FILE_FORM_TYPE).len(), 1);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(calls(&e, 0x0048_2d50), vec![vec![this + 0x24, file]]);
        assert_eq!(calls(&e, ICON_SET), vec![vec![this, 0x42]]);
        assert_eq!(e.mem.u32(this + 0x38), 3);
        assert_eq!(e.mem.u32(this + 0x3c), 4);
        assert_eq!(calls(&e, 0x0040_1000), vec![vec![0x10]]);
        let set = calls(&e, 0x0040_37f0);
        assert_eq!((set[0][0], set[0][2]), (0x6000, 0));
        assert_eq!(calls(&e, 0x0059_11d0), vec![vec![this, 0x6000]]);
        // Only the condition after the item is read.
        assert_eq!(calls(&e, 0x0068_0960), vec![vec![0x6008, file]]);
    }
}
