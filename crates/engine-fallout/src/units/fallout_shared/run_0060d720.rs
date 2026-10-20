//! `fallout shared/run_0060d720` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `strlen`-style length (`0044a670`, `cdecl`), `strcpy_s` (`00406d30`,
/// `cdecl` (dest, size, src)), `BSStringT<char>::StrCmp` (`00408a80`) and
/// the `BSStringT<char>` text assignment (`004037f0`).
const STRING_LENGTH: u32 = 0x0044_a670;
const STRCPY_S: u32 = 0x0040_6d30;
const STRING_COMPARE: u32 = 0x0040_8a80;
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// The quest's editor id (`cFormEditorID`, `BSStringT<char>`, +0x64).
const EDITOR_ID: u32 = 0x64;

// Translated from 0060dab0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060dab0 - the /GS stack cookie check (00ec408c) is not translated.
/// The quest's `SetEditorID` (vtable slot; `thiscall`, `RET 4`): copies
/// `name` to a stack buffer of its length + 1 (the compiler's `alloca`,
/// `00ec5ec0`, here the engine's stack), and answers false when the editor
/// id (+0x64) already equals it (`StrCmp` with case ignored); else stores
/// it there and answers true.
pub fn fn_0060dab0(e: &mut Engine, this: Ptr, name: Ptr) -> bool {
    let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(1);
    e.with_stack(size, |e, buffer| {
        e.call(STRCPY_S, &args![buffer, size, name]);
        let editor_id = this.addr() + EDITOR_ID;
        if e.call(STRING_COMPARE, &args![editor_id, buffer, 1u32])
            .u32()
            == 0
        {
            return false;
        }
        e.call(STRING_ASSIGN, &args![editor_id, buffer, 0u32]);
        true
    })
}

/// Copies the save buffer's change flags (the word at +0x17 of the
/// buffer) into a local (`00428110`, thiscall on the buffer, the local's
/// address; returns that address).
const COPY_CHANGE_FLAGS: u32 = 0x0042_8110;
/// Tests a mask against the flags at its `this` (`004280f0`, thiscall, the
/// mask; a `bool` in AL).
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `005621f0` (`tesobjectrefr.cpp`; thiscall on the buffer, one word).
const BUFFER_CALL_005621F0: u32 = 0x0056_21f0;
/// `00484d60` (`tesform.cpp`; thiscall on `this`, the buffer): the form's
/// own part of the save.
const FORM_SAVE_00484D60: u32 = 0x0048_4d60;
/// `BGSSaveGameBuffer`'s byte save (`00865e50`; source, size, a third word),
/// `StartVariableSizedValue` (`00865f20`) and `SaveVariableSizedValue_ov2`
/// (`00865ff0`; the count and the start position).
const SAVE_BYTES: u32 = 0x0086_5e50;
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const FINISH_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// List helpers: the item pointer's address of a node (`006815c0`) and the
/// next node (`00726070`; the code also runs it on an item at `0060e810`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// Per-item getters (`009373f0`, `005dc960`, `005dc980`, each a byte in AL;
/// `00717e50`, a list head; `008d8520`, a pointer; `007af430`, a word).
const ITEM_FLAG_009373F0: u32 = 0x0093_73f0;
const ITEM_FLAG_005DC960: u32 = 0x005d_c960;
const ITEM_FLAG_005DC980: u32 = 0x005d_c980;
const ITEM_LIST_00717E50: u32 = 0x0071_7e50;
const ITEM_POINTER_008D8520: u32 = 0x008d_8520;
const ITEM_VALUE_007AF430: u32 = 0x007a_f430;
/// `ScriptLocals::SaveGame` (`005a9db0`, thiscall on the locals, the
/// buffer).
const SCRIPT_LOCALS_SAVE_GAME: u32 = 0x005a_9db0;
/// The fields `0060e810` saves (offsets as the code reads them): a byte at
/// +0x3c, a word at +0x40, the lists whose first nodes sit at +0x44 and
/// +0x4c, and the script locals pointer at +0x5c (also read by `0060e7c0`).
const BYTE_3C: u32 = 0x3c;
const WORD_40: u32 = 0x40;
const LIST_44: u32 = 0x44;
const LIST_4C: u32 = 0x4c;
const SCRIPT_LOCALS: u32 = 0x5c;
/// Other list helpers of `0060efd0` and `0060f110`: the first node of the
/// object's list (`005d43e0`), a test of an item (`005f65d0`, thiscall on
/// the second argument, the item's address), a check of the item's word
/// (`007af430`), the item's sub-list (`007d6bb0`), its reference
/// (`006101b0`, one word), its conditions (`00717e50`), their test
/// (`00680c30`, two words), the end check (`008256d0`), the start check
/// (`0059e400`), `00470470` on the first argument and `005ec500` on a
/// matched item (one word).
const FIRST_NODE_005D43E0: u32 = 0x005d_43e0;
const ITEM_TEST_005F65D0: u32 = 0x005f_65d0;
const SUB_LIST_007D6BB0: u32 = 0x007d_6bb0;
const ITEM_REFERENCE_006101B0: u32 = 0x0061_01b0;
const CONDITIONS_TRUE_00680C30: u32 = 0x0068_0c30;
const CURSOR_DONE_008256D0: u32 = 0x0082_56d0;
const SKIP_CHECK_0059E400: u32 = 0x0059_e400;
const ARGUMENT_CALL_00470470: u32 = 0x0047_0470;
const ITEM_CALL_005EC500: u32 = 0x005e_c500;

/// `004280f0(00428110(buffer, &local), mask)`: whether the save buffer's
/// change flags (copied to a stack local) have any bit of `mask`.
fn change_flags_have(e: &mut Engine, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(COPY_CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, mask]).u32() & 0xff != 0
    })
}

/// `00865e50(buffer, source, size, 0)`.
fn save_bytes(e: &mut Engine, buffer: Ptr, source: u32, size: u32) {
    e.call(SAVE_BYTES, &args![buffer, source, size, 0u32]);
}

/// Saves a stack copy of `value`'s low `size` bytes.
fn save_local(e: &mut Engine, buffer: Ptr, value: u32, size: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        save_bytes(e, buffer, local.addr(), size);
    });
}

/// The item a list node holds (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 0060e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the buffer's change flags have bit 30 (`0x40000000`) and the word
/// at +0x5c of `this` is null, calls `005621f0(buffer, 0x40000000)`
/// (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_0060e7c0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    if change_flags_have(e, buffer, 0x4000_0000) && e.mem.u32(this.addr() + SCRIPT_LOCALS) == 0 {
        e.call(BUFFER_CALL_005621F0, &args![buffer, 0x4000_0000u32]);
    }
}

// Translated from 0060e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object into the save buffer (thiscall, `RET 4`; unnamed in
/// the engine map), as the buffer's change flags select:
/// the form's part (`00484d60`) always; flag 2: the byte at +0x3c; flag 4:
/// the word at +0x40; flag `0x80000000`: a counted list of the items of the
/// list at +0x44, each two bytes (`009373f0`, `005dc960`) and a counted
/// list of its sub-items (`00717e50`), each a byte (`005dc980`), a byte
/// saying whether `008d8520` gives a pointer and then the 4 bytes that
/// pointer points to; flag `0x40000000`: the script locals at +0x5c
/// (`ScriptLocals::SaveGame`); flag `0x20000000`: a counted list of the
/// items of the list at +0x4c whose word (`007af430`) is non-zero, each
/// `00726070(item)` and that word (4 bytes each). Counts are
/// variable-sized values patched in after their lists.
pub fn fn_0060e810(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(FORM_SAVE_00484D60, &args![this, buffer]);
    if change_flags_have(e, buffer, 2) {
        save_bytes(e, buffer, base + BYTE_3C, 1);
    }
    if change_flags_have(e, buffer, 4) {
        save_bytes(e, buffer, base + WORD_40, 4);
    }
    if change_flags_have(e, buffer, 0x8000_0000) {
        let mut count = 0u32;
        let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let mut node = base + LIST_44;
        while node != 0 {
            let item = node_item(e, node);
            if item != 0 {
                let first = e.call(ITEM_FLAG_009373F0, &args![item]).u32() & 0xff;
                let second = e.call(ITEM_FLAG_005DC960, &args![item]).u32() & 0xff;
                save_local(e, buffer, first, 1);
                save_local(e, buffer, second, 1);
                let mut inner_count = 0u32;
                let inner_start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
                let mut sub = e.call(ITEM_LIST_00717E50, &args![item]).u32();
                while sub != 0 {
                    let entry = node_item(e, sub);
                    if entry != 0 {
                        let byte = e.call(ITEM_FLAG_005DC980, &args![entry]).u32() & 0xff;
                        let pointer = e.call(ITEM_POINTER_008D8520, &args![entry]).u32();
                        let has_pointer = u32::from(pointer != 0);
                        save_local(e, buffer, byte, 1);
                        save_local(e, buffer, has_pointer, 1);
                        if has_pointer != 0 {
                            save_bytes(e, buffer, pointer, 4);
                        }
                        inner_count = inner_count.wrapping_add(1);
                    }
                    sub = e.call(NODE_NEXT, &args![sub]).u32();
                }
                e.call(
                    FINISH_VARIABLE_SIZED_VALUE,
                    &args![buffer, inner_count, inner_start],
                );
                count = count.wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
    }
    if change_flags_have(e, buffer, 0x4000_0000) {
        let locals = e.mem.u32(base + SCRIPT_LOCALS);
        e.call(SCRIPT_LOCALS_SAVE_GAME, &args![locals, buffer]);
    }
    if change_flags_have(e, buffer, 0x2000_0000) {
        let mut count = 0u32;
        let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let mut node = base + LIST_4C;
        while node != 0 {
            let item = node_item(e, node);
            if item != 0 {
                let word = e.call(ITEM_VALUE_007AF430, &args![item]).u32();
                if word != 0 {
                    let other = e.call(NODE_NEXT, &args![item]).u32();
                    save_local(e, buffer, other, 4);
                    save_local(e, buffer, word, 4);
                    count = count.wrapping_add(1);
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
    }
}

// Translated from 0060efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks the object's list against the list `cursor` (thiscall, `RET 8`;
/// unnamed in the engine map). Without a cursor or a first node
/// (`005d43e0`) only the end check runs. Otherwise for each item of the
/// object's list whose word (`007af430`) is 1 and that `005f65d0(other,
/// item)` accepts, each non-null entry of the item's sub-list
/// (`007d6bb0`) whose conditions (`00717e50`) hold for its reference
/// (`00680c30(conditions, 006101b0(entry, 0), 0)`) must be the cursor's
/// current item (false otherwise, also when the cursor or its item is
/// null), and the cursor moves on. At the end a cursor left non-null must
/// pass `008256d0`. Returns true when everything matched.
pub fn fn_0060efd0(e: &mut Engine, this: Ptr, cursor: u32, other: u32) -> bool {
    let mut cursor = cursor;
    let mut node = e.call(FIRST_NODE_005D43E0, &args![this]).u32();
    if cursor != 0 && node != 0 {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) != 0 {
                let item = node_item(e, node);
                if e.call(ITEM_VALUE_007AF430, &args![item]).u32() == 1 {
                    let item_slot = e.call(NODE_ITEM, &args![node]).u32();
                    if e.call(ITEM_TEST_005F65D0, &args![other, item_slot]).u32() & 0xff != 0 {
                        let item = node_item(e, node);
                        let mut entry_node = e.call(SUB_LIST_007D6BB0, &args![item]).u32();
                        while entry_node != 0 {
                            let entry = node_item(e, entry_node);
                            if entry != 0 {
                                let reference =
                                    e.call(ITEM_REFERENCE_006101B0, &args![entry, 0u32]).u32();
                                let conditions = e.call(ITEM_LIST_00717E50, &args![entry]).u32();
                                let holds = e
                                    .call(
                                        CONDITIONS_TRUE_00680C30,
                                        &args![conditions, reference, 0u32],
                                    )
                                    .u32()
                                    & 0xff
                                    != 0;
                                if holds {
                                    if cursor == 0 {
                                        return false;
                                    }
                                    if node_item(e, cursor) == 0 {
                                        return false;
                                    }
                                    if node_item(e, cursor) != entry {
                                        return false;
                                    }
                                    if cursor != 0 {
                                        cursor = e.call(NODE_NEXT, &args![cursor]).u32();
                                    }
                                }
                            }
                            entry_node = e.call(NODE_NEXT, &args![entry_node]).u32();
                        }
                    }
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    if cursor != 0 && e.call(CURSOR_DONE_008256D0, &args![cursor]).u32() & 0xff == 0 {
        return false;
    }
    true
}

// Translated from 0060f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a non-null `target` (thiscall, `RET 8`; unnamed in the engine
/// map): `00470470(target)`; unless `0059e400(this)` answers true, every
/// item of the object's list (from `005d43e0`, stopping at the first null
/// item) that `005f65d0(other, item)` accepts gets
/// `005ec500(item, target)`.
pub fn fn_0060f110(e: &mut Engine, this: Ptr, target: u32, other: u32) {
    if target == 0 {
        return;
    }
    e.call(ARGUMENT_CALL_00470470, &args![target]);
    if e.call(SKIP_CHECK_0059E400, &args![this]).u32() & 0xff != 0 {
        return;
    }
    let mut node = e.call(FIRST_NODE_005D43E0, &args![this]).u32();
    while node != 0 {
        if node_item(e, node) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.call(ITEM_TEST_005F65D0, &args![other, slot]).u32() & 0xff != 0 {
            let item = node_item(e, node);
            e.call(ITEM_CALL_005EC500, &args![item, target]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060dab0, fn_0060dab0(Ptr, Ptr) -> bool),
        entry!(0x0060_e7c0, fn_0060e7c0(Ptr, Ptr)),
        entry!(0x0060_e810, fn_0060e810(Ptr, Ptr)),
        entry!(0x0060_efd0, fn_0060efd0(Ptr, u32, u32) -> bool),
        entry!(0x0060_f110, fn_0060f110(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_editor_id_unless_it_is_already_that() {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.write(a[0], &s);
            e.mem.set_u8(a[0] + s.len() as u32, 0);
            Ret::default()
        });
        e.register_double(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        let quest = e.mem.alloc(0x6c);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"VMS49\0");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 1);
        let log = e.call_log.take().unwrap();
        let assign = log.iter().find(|(a, _)| *a == STRING_ASSIGN).unwrap();
        assert_eq!(assign.1[0], quest + 0x64);
        assert_eq!(e.mem.cstr(assign.1[1]), b"VMS49");
        assert_eq!(assign.1[2], 0);
        // Equal already: nothing stored.
        e.register_double(STRING_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == STRING_ASSIGN));
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The test buffer: its change flags at +0x17, as `00428110` reads.
    fn buffer_with_flags(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0x17, flags);
        e.register(COPY_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| Ret {
            eax: 0xffff_ff00 | u32::from(e.mem.u32(a[0]) & a[1] != 0),
            ..Ret::default()
        });
        buffer
    }

    /// A list node (`{item, next}`); the doubles read it as the game's
    /// helpers do.
    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    #[test]
    fn fn_0060e7c0_calls_out_only_with_the_flag_and_no_locals() {
        for (flags, locals, calls) in [
            (0x4000_0000u32, 0u32, true),
            (0xbfff_ffff, 0, false),
            (0x4000_0000, 0x1234, false),
        ] {
            let mut e = Engine::new();
            let buffer = buffer_with_flags(&mut e, flags);
            e.register(BUFFER_CALL_005621F0, |_, _| Ret::default());
            let this = e.mem.alloc(0x60);
            e.mem.set_u32(this + SCRIPT_LOCALS, locals);
            e.call_log = Some(vec![]);
            e.call(0x0060_e7c0, &args![this, buffer]);
            let seen = calls_to(&e, BUFFER_CALL_005621F0);
            if calls {
                assert_eq!(seen, vec![vec![buffer, 0x4000_0000]]);
            } else {
                assert!(seen.is_empty());
            }
            assert_eq!(calls_to(&e, FLAGS_TEST)[0][1], 0x4000_0000);
        }
    }

    /// Records saved bytes (`RECORD`: a length word, then the bytes) and
    /// the variable-sized counts (`COUNTS`: a length word, then count and
    /// start pairs).
    const RECORD: u32 = 0x0340_0000;
    const COUNTS: u32 = 0x0341_0000;

    fn record_saves(e: &mut Engine) {
        e.map(RECORD, 0x100);
        e.map(COUNTS, 0x100);
        e.register(SAVE_BYTES, |e, a| {
            assert_eq!(a[3], 0);
            let used = e.mem.u32(RECORD);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(RECORD + 4 + used, &bytes);
            e.mem.set_u32(RECORD, used + a[2]);
            Ret::default()
        });
        e.register(START_VARIABLE_SIZED_VALUE, |e, _| {
            // Start positions 0x100, 0x101, ... in call order.
            let started = e.mem.u32(COUNTS + 0x80);
            e.mem.set_u32(COUNTS + 0x80, started + 1);
            (0x100 + started).into_ret()
        });
        e.register(FINISH_VARIABLE_SIZED_VALUE, |e, a| {
            let used = e.mem.u32(COUNTS);
            e.mem.set_u32(COUNTS + 4 + used * 8, a[1]);
            e.mem.set_u32(COUNTS + 8 + used * 8, a[2]);
            e.mem.set_u32(COUNTS, used + 1);
            Ret::default()
        });
    }

    fn recorded(e: &Engine) -> Vec<u8> {
        e.mem.bytes(RECORD + 4, e.mem.u32(RECORD))
    }

    fn counts(e: &Engine) -> Vec<(u32, u32)> {
        (0..e.mem.u32(COUNTS))
            .map(|i| (e.mem.u32(COUNTS + 4 + i * 8), e.mem.u32(COUNTS + 8 + i * 8)))
            .collect()
    }

    #[test]
    fn fn_0060e810_saves_only_the_form_part_without_flags() {
        let mut e = Engine::new();
        let buffer = buffer_with_flags(&mut e, 0x1fff_fff9);
        record_saves(&mut e);
        e.register(FORM_SAVE_00484D60, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x0060_e810, &args![this, buffer]);
        assert_eq!(calls_to(&e, FORM_SAVE_00484D60), vec![vec![this, buffer]]);
        assert!(recorded(&e).is_empty());
        assert!(counts(&e).is_empty());
    }

    #[test]
    fn fn_0060e810_saves_every_selected_part() {
        let mut e = Engine::new();
        let buffer = buffer_with_flags(&mut e, 0xe000_0006);
        record_saves(&mut e);
        list_doubles(&mut e);
        e.register(FORM_SAVE_00484D60, |_, _| Ret::default());
        e.register(SCRIPT_LOCALS_SAVE_GAME, |_, _| Ret::default());
        e.register(ITEM_FLAG_009373F0, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0])).into_ret()
        });
        e.register(ITEM_FLAG_005DC960, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0] + 4)).into_ret()
        });
        e.register(ITEM_LIST_00717E50, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ITEM_FLAG_005DC980, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0])).into_ret()
        });
        e.register(ITEM_POINTER_008D8520, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_VALUE_007AF430, |e, a| e.mem.u32(a[0]).into_ret());

        let this = e.mem.alloc(0x60);
        e.mem.set_u8(this + BYTE_3C, 0xab);
        e.mem.set_u32(this + WORD_40, 0x0403_0201);
        e.mem.set_u32(this + SCRIPT_LOCALS, 0x0777_0000);
        // List at +0x44: one item with two sub-entries (the second without
        // a pointer), then a null item.
        let pointed = e.mem.alloc(4);
        e.mem.set_u32(pointed, 0x0d0c_0b0a);
        let entry_a = e.mem.alloc(8);
        e.mem.set_u32(entry_a, 0x11);
        e.mem.set_u32(entry_a + 4, pointed);
        let entry_b = e.mem.alloc(8);
        e.mem.set_u32(entry_b, 0x22);
        let sub_second = node(&mut e, entry_b, 0);
        let sub_null = node(&mut e, 0, sub_second);
        let sub_first = node(&mut e, entry_a, sub_null);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item, 0x05);
        e.mem.set_u32(item + 4, 0x06);
        e.mem.set_u32(item + 8, sub_first);
        let second = node(&mut e, 0, 0);
        e.mem.set_u32(this + LIST_44, item);
        e.mem.set_u32(this + LIST_44 + 4, second);
        // List at +0x4c: an item with a zero word (skipped), one with a
        // word; `00726070` on the item reads its +4.
        let skipped = e.mem.alloc(8);
        let kept = e.mem.alloc(8);
        e.mem.set_u32(kept, 0x0000_0033);
        e.mem.set_u32(kept + 4, 0x4444_4444);
        let kept_node = node(&mut e, kept, 0);
        e.mem.set_u32(this + LIST_4C, skipped);
        e.mem.set_u32(this + LIST_4C + 4, kept_node);

        e.call_log = Some(vec![]);
        e.call(0x0060_e810, &args![this, buffer]);
        let mut expected = vec![0xab, 1, 2, 3, 4];
        expected.extend([0x05, 0x06]);
        expected.extend([0x11, 1, 0x0a, 0x0b, 0x0c, 0x0d]);
        expected.extend([0x22, 0]);
        expected.extend([0x44, 0x44, 0x44, 0x44, 0x33, 0, 0, 0]);
        assert_eq!(recorded(&e), expected);
        // Inner count first (start 0x101), then the outer (0x100), then the
        // last list (0x102).
        assert_eq!(counts(&e), vec![(2, 0x101), (1, 0x100), (1, 0x102)]);
        assert_eq!(
            calls_to(&e, SCRIPT_LOCALS_SAVE_GAME),
            vec![vec![0x0777_0000, buffer]]
        );
    }

    fn efd0_setup(e: &mut Engine) {
        list_doubles(e);
        // Items: word at +0 (`007af430`), sub-list at +4 (`007d6bb0`).
        e.register(ITEM_VALUE_007AF430, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SUB_LIST_007D6BB0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // `005f65d0` accepts every item slot except when `other` is 0.
        e.register(ITEM_TEST_005F65D0, |_, a| u32::from(a[0] != 0).into_ret());
        // Entries: their reference is themselves, conditions hold when the
        // entry's word is odd.
        e.register(ITEM_REFERENCE_006101B0, |_, a| a[0].into_ret());
        e.register(ITEM_LIST_00717E50, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CONDITIONS_TRUE_00680C30, |_, a| (a[0] & 1).into_ret());
        e.register(CURSOR_DONE_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) != 0).into_ret()
        });
        e.register(FIRST_NODE_005D43E0, |e, a| e.mem.u32(a[0]).into_ret());
    }

    #[test]
    fn fn_0060efd0_matches_holding_entries_against_the_cursor() {
        let mut e = Engine::new();
        efd0_setup(&mut e);
        let entry_on = e.mem.alloc(4);
        e.mem.set_u32(entry_on, 1);
        let entry_off = e.mem.alloc(4);
        e.mem.set_u32(entry_off, 2);
        let sub_b = node(&mut e, entry_on, 0);
        let sub_a = node(&mut e, entry_off, sub_b);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 1);
        e.mem.set_u32(item + 4, sub_a);
        let first = node(&mut e, item, 0);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, first);
        // The cursor's item is the holding entry; its next is an end node
        // whose done flag (+8) decides.
        let end = e.mem.alloc(12);
        let cursor = e.mem.alloc(12);
        e.mem.set_u32(cursor, entry_on);
        e.mem.set_u32(cursor + 4, 0);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // A cursor that does not match.
        e.mem.set_u32(cursor, entry_off);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        // A cursor left over that is not done fails, one that is passes.
        e.mem.set_u32(cursor, entry_on);
        e.mem.set_u32(cursor + 4, end);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        e.mem.set_u32(end + 8, 1);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // Running out of cursor while an entry still holds fails.
        e.mem.set_u32(cursor + 4, 0);
        let sub_c = node(&mut e, entry_on, 0);
        e.mem.set_u32(sub_b + 4, sub_c);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        // Items the test refuses, or whose word is not 1, are skipped (the
        // untouched cursor then has to be done).
        e.mem.set_u32(cursor + 8, 1);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 0u32]).u32() & 0xff,
            1
        );
        e.mem.set_u32(item, 2);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // No cursor: true without walking.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, 0u32, 1u32]).u32() & 0xff,
            1
        );
        assert!(calls_to(&e, NODE_ITEM).is_empty());
        assert!(calls_to(&e, CURSOR_DONE_008256D0).is_empty());
        // No first node: only the end check on the cursor.
        e.mem.set_u32(this, 0);
        e.mem.set_u32(cursor + 8, 0);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn fn_0060f110_calls_out_for_accepted_items_until_a_null_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(FIRST_NODE_005D43E0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARGUMENT_CALL_00470470, |_, _| Ret::default());
        e.register(SKIP_CHECK_0059E400, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // Accepts item slots whose item is even.
        e.register(ITEM_TEST_005F65D0, |e, a| {
            assert_eq!(a[0], 0x0999_0000);
            u32::from(e.mem.u32(a[1]) % 2 == 0).into_ret()
        });
        e.register(ITEM_CALL_005EC500, |_, _| Ret::default());
        let after_null = node(&mut e, 0x40, 0);
        let null_node = node(&mut e, 0, after_null);
        let third = node(&mut e, 0x30, null_node);
        let second = node(&mut e, 0x21, third);
        let first = node(&mut e, 0x10, second);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, first);
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0x0555_0000u32, 0x0999_0000u32]);
        assert_eq!(
            calls_to(&e, ARGUMENT_CALL_00470470),
            vec![vec![0x0555_0000]]
        );
        assert_eq!(
            calls_to(&e, ITEM_CALL_005EC500),
            vec![vec![0x10, 0x0555_0000], vec![0x30, 0x0555_0000]]
        );
        // The skip check stops it after the first call.
        e.mem.set_u32(this + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0x0555_0000u32, 0x0999_0000u32]);
        assert_eq!(calls_to(&e, ARGUMENT_CALL_00470470).len(), 1);
        assert!(calls_to(&e, FIRST_NODE_005D43E0).is_empty());
        // A null target does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0u32, 0x0999_0000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
