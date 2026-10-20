//! `fallout shared/bgsterminal.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00500940 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns receiver +0x18 without reading memory. The source-unit mapping
/// is by address range; no Xbox PDB name is available for this accessor.
/// Script callers use it to obtain their embedded header.
pub fn fn_00500940(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x18))
}

/// The string constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The thread-local word `004fffe0` stores (`+0x2b8` of the TLS block).
const TLS_WORD: u32 = 0x2b8;

// Translated from 004fffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+0x2b8` of the calling thread's TLS block
/// (`*(FS:[0x2c])[_tls_index] + 0x2b8`). cdecl, one word. The quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_004fffe0(e: &mut Engine, value: u32) {
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_WORD, value);
}

// Translated from 00500910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an object of the `SCRIPT_REFERENCED_OBJECT` layout (Xbox PDB):
/// the editor ID string at `+0` (`004037b0`), then `pForm` (`+8`) and
/// `uiVariableID` (`+0xc`) zeroed. Returns `this`. The engine map's name
/// (`BSSimpleList<SCRIPT_REFERENCED_OBJECT_P>::AddTail`) belongs to folded
/// code; the body is a constructor. The stage item loader `0060f6a0` calls it.
pub fn fn_00500910(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

// Translated from 005010c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005010f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005010c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_10f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

layout! {
    /// `BGSTerminal` (Xbox PDB, 0xB8 bytes there): 0xA8 bytes on PC
    /// (`CreateFormOfType` allocates that much), `TESForm` being 0x18.
    pub struct BGSTerminal: 0xA8 {
        /// `TESDescription` base (Xbox PDB +0xA0).
        0x90 Description: u32,
        /// `MenuItemList` (Xbox PDB +0xA8): `BSSimpleList<TERMINAL_MENU_ITEM *>`.
        0x98 MenuItemList: Inline<crate::types::BSSimpleList>,
        /// `pPassword` (Xbox PDB +0xB0): `BGSNote*`.
        0xA0 pPassword: u32,
        /// `Data.cDifficulty` (Xbox PDB +0xB4).
        0xA4 cDifficulty: u8,
        /// `Data.cFlags` (Xbox PDB +0xB5).
        0xA5 cFlags: u8,
    }
}

layout! {
    /// `TERMINAL_MENU_ITEM` (Xbox PDB, 0x88 bytes there): 0x78 bytes on PC
    /// (`Load` allocates that much).
    pub struct TerminalMenuItem: 0x78 {
        /// `ResponseText` (Xbox PDB): `BSStringT<char>`.
        0x00 ResponseText: Inline<crate::types::BSStringT>,
        /// `ResultText` (Xbox PDB): `BSStringT<char>`.
        0x08 ResultText: Inline<crate::types::BSStringT>,
        /// `ResultScript` (Xbox PDB): a `Script`.
        0x10 ResultScript: u32,
        /// `Conditions` (Xbox PDB): `TESCondition`.
        0x64 Conditions: u32,
        /// `pDisplayNote` (Xbox PDB): `BGSNote*` (a form id until `InitItem`).
        0x6C pDisplayNote: u32,
        /// `pSubMenu` (Xbox PDB): `BGSTerminal*` (a form id until `InitItem`).
        0x70 pSubMenu: u32,
        /// `cFlags` (Xbox PDB).
        0x74 cFlags: u8,
    }
}

/// The string destructor (`004037d0`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// Sets a string from a C string and a length (`004037f0`, `RET 8`).
const STRING_SET: u32 = 0x0040_37f0;
/// A `BSSimpleList` node's empty test (`008256d0`, `bool`).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The address of a node's item (`006815c0`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// A node's next node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Removes a list's first item (`0063f7b0`).
const LIST_REMOVE_FIRST: u32 = 0x0063_f7b0;
/// Appends the item a pointer points at to a list (`00905820`, `RET 4`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// The `MemoryManager` allocation wrapper (`00401000`, cdecl).
const ALLOCATE: u32 = 0x0040_1000;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@`, the note's (`0118640c`) and
/// `.?AVBGSTerminal@@`'s (`01189ebc`), as the code passes them.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_NOTE: u32 = 0x0118_640c;
const TYPE_TERMINAL: u32 = 0x0118_9ebc;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// The warning count (`0046e8a0`, no arguments).
const WARNING_COUNT: u32 = 0x0046_e8a0;
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB, `0043b2b0`, cdecl).
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
/// "MASTERFILE: Unable to find password (%08X) on terminal '%s' (%08X)."
const MISSING_PASSWORD_MESSAGE: u32 = 0x0102_5c80;
/// "MASTERFILE: Unable to find display note (%08X) for menu item %i on
/// terminal '%s' (%08X)."
const MISSING_NOTE_MESSAGE: u32 = 0x0102_5c20;
/// "MASTERFILE: Unable to find submenu (%08X) for menu item %i on terminal
/// '%s' (%08X)."
const MISSING_SUBMENU_MESSAGE: u32 = 0x0102_5bc8;
/// "MASTERFILE: Errors were encountered during InitItem for result script
/// on menu item %d for terminal '%s' (%08X)..."
const SCRIPT_ERRORS_MESSAGE: u32 = 0x0102_5b30;
/// "MASTERFILE: Errors were encountered during InitItem for conditions on
/// menu item %d for terminal '%s' (%08X)..."
const CONDITION_ERRORS_MESSAGE: u32 = 0x0102_5a98;
/// The `ExtraDataList` of a reference (`005d43c0`).
const EXTRA_DATA: u32 = 0x005d_43c0;
/// A reference's lock level from its extra data (`0041c060`, signed byte).
const EXTRA_LOCK_LEVEL: u32 = 0x0041_c060;
/// `ExtraDataList::GetTerminalState` (Xbox PDB, `0041bf50`, a byte).
const EXTRA_TERMINAL_STATE: u32 = 0x0041_bf50;
/// The address of a `Setting`'s value (`00403e20`, the setting in `ECX`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `fHackLevelMult` (the `Setting` at `011c83c4`).
const HACK_LEVEL_MULT: u32 = 0x011c_83c4;
/// `fHackingMinSkillVeryEasy`, `Easy`, `Average`, `Hard`, `VeryHard` (the
/// settings at these addresses), by lock level.
const HACKING_MIN_SKILL: [u32; 5] = [
    0x011d_961c,
    0x011d_9660,
    0x011d_9684,
    0x011d_95d4,
    0x011d_9640,
];
/// The `float` past every lock level (`01016970`).
const UNHACKABLE_MIN_SKILL: u32 = 0x0101_6970;
/// The player (a pointer at `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `double` the hacking entry point's result is compared with
/// (`01012060`).
const LOCKOUT_COMPARE: u32 = 0x0101_2060;
/// The `ScriptVariable` being loaded (a global at `011c83d4`).
const CURRENT_VARIABLE: u32 = 0x011c_83d4;

/// Chunk signatures, as the words the exe compares.
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_CTDA: u32 = 0x4144_5443;
const SIG_DESC: u32 = 0x4353_4544;
const SIG_DEST: u32 = 0x5453_4544;
const SIG_DNAM: u32 = 0x4d41_4e44;
const SIG_DSTD: u32 = 0x4454_5344;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_INAM: u32 = 0x4d41_4e49;
const SIG_ITXT: u32 = 0x5458_5449;
const SIG_MODD: u32 = 0x4444_4f4d;
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODS: u32 = 0x5344_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_MOSD: u32 = 0x4453_4f4d;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_PNAM: u32 = 0x4d41_4e50;
const SIG_RNAM: u32 = 0x4d41_4e52;
const SIG_SCDA: u32 = 0x4144_4353;
const SIG_SCHR: u32 = 0x5248_4353;
const SIG_SCRI: u32 = 0x4952_4353;
const SIG_SCRO: u32 = 0x4f52_4353;
const SIG_SCRV: u32 = 0x5652_4353;
const SIG_SCVR: u32 = 0x5256_4353;
const SIG_SLSD: u32 = 0x4453_4c53;
const SIG_SNAM: u32 = 0x4d41_4e53;
const SIG_TNAM: u32 = 0x4d41_4e54;

/// `FISTP qword` under the control word with rounding toward zero (`OR
/// 0xC00`): the integer part, the integer indefinite for NaN and values out
/// of range.
pub(crate) fn fistp_truncated_qword(value: f64) -> i64 {
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if value.is_nan() || value >= LIMIT || value < -LIMIT {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

/// A form id at `id_at`, given the compile index of `form`'s file and looked
/// up, cast from `TESForm` to `target` (`__RTDynamicCast`).
fn resolve(e: &mut Engine, form: Ptr, id_at: Ptr, target: u32) -> u32 {
    let file = e.call(GET_FILE, &args![form, u32::MAX]).u32();
    e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
    let id = e.mem.u32(id_at.addr());
    let found = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![found, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// `TESForm`'s form id and vtable slot 0x130's answer (its name), in the
/// order the code computes them.
fn id_and_name(e: &mut Engine, form: Ptr) -> (u32, u32) {
    let form_id = e.call(FORM_ID, &args![form]).u32();
    let name = e.vcall(form.addr(), 0x130, &args![]).u32();
    (form_id, name)
}

// Translated from 004ffcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), `TESObjectACTI::InitItem` (Xbox PDB, `00510770`), then
/// resolves the password note id at `+0xA0` when not zero (cast to
/// `0118640c`; missing, reported with the id, name and form id), then for
/// every menu item of the list `00461150` gives (counting from 0): its
/// display note id (`+0x6C`, cast to `0118640c`) and submenu id (`+0x70`,
/// cast to `.?AVBGSTerminal@@`) when not zero, each missing one reported
/// with the id and the item's number; then its result script's
/// `Script::InitItem` (`005ab400` on `+0x10`) and its conditions'
/// `TESCondition::InitItem` (`00680a20` on `+0x64`), each with the terminal,
/// warnings disabled (`0043b2b0` with 1, then 0) and the TLS word
/// (`004fffe0`) cleared around it, then put back to its value before
/// (the warning count `0046e8a0`); a count that is not zero afterwards is
/// reported with the item's number. Finally form flag 0x8 is set.
pub fn bgs_terminal_init_item(e: &mut Engine, this: Ptr<BGSTerminal>) {
    let form = this.cast::<()>();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.call(0x0051_0770, &args![this]);
    e.with_stack(4, |e, id_at| {
        let password = e.get(this, BGSTerminal::pPassword);
        e.mem.set_u32(id_at.addr(), password);
        if password != 0 {
            let note = resolve(e, form, id_at, TYPE_NOTE);
            e.set(this, BGSTerminal::pPassword, note);
            if e.get(this, BGSTerminal::pPassword) == 0 {
                let (form_id, name) = id_and_name(e, form);
                let id = e.mem.u32(id_at.addr());
                e.call(WARNING, &args![MISSING_PASSWORD_MESSAGE, id, name, form_id]);
            }
        }
        let mut index = 0u32;
        let mut node = e.call(0x0046_1150, &args![this]).u32();
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let item_at = e.call(NODE_ITEM, &args![node]).u32();
            let item: Ptr<TerminalMenuItem> = Ptr::new(e.mem.u32(item_at));
            node = e.call(NODE_NEXT, &args![node]).u32();
            for (field, target, message) in [
                (
                    TerminalMenuItem::pDisplayNote,
                    TYPE_NOTE,
                    MISSING_NOTE_MESSAGE,
                ),
                (
                    TerminalMenuItem::pSubMenu,
                    TYPE_TERMINAL,
                    MISSING_SUBMENU_MESSAGE,
                ),
            ] {
                let id = e.get(item, field);
                if id == 0 {
                    continue;
                }
                e.mem.set_u32(id_at.addr(), id);
                let found = resolve(e, form, id_at, target);
                e.set(item, field, found);
                if e.get(item, field) == 0 {
                    let (form_id, name) = id_and_name(e, form);
                    let id = e.mem.u32(id_at.addr());
                    e.call(WARNING, &args![message, id, index, name, form_id]);
                }
            }
            for (callee, offset, message) in [
                (0x005a_b400, 0x10, SCRIPT_ERRORS_MESSAGE),
                (0x0068_0a20, 0x64, CONDITION_ERRORS_MESSAGE),
            ] {
                let saved = e.call(WARNING_COUNT, &args![]).u32();
                e.call(DISABLE_WARNINGS, &args![true]);
                fn_004fffe0(e, 0);
                e.call(callee, &args![item.addr() + offset, this]);
                e.call(DISABLE_WARNINGS, &args![false]);
                let errors = e.call(WARNING_COUNT, &args![]).u32();
                fn_004fffe0(e, saved);
                if errors != 0 {
                    let (form_id, name) = id_and_name(e, form);
                    e.call(WARNING, &args![message, index, name, form_id]);
                }
            }
            index = index.wrapping_add(1);
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

/// A stack buffer of `size` bytes (`__alloca_probe_16`, `00ec5ec0`).
fn alloca<R>(e: &mut Engine, size: u32, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(size, f)
}

/// A new `SCRIPT_REFERENCED_OBJECT` (0x10 bytes, `00401000`, constructed by
/// `00500910` when the allocation succeeds, else null) with the chunk's
/// word read into the local at `value_at` (not cleared first) and stored at
/// `+field`; then the address of the local `object_at` holding it is
/// appended to the list `005d43c0` gives for the script at `item + 0x10`.
fn load_referenced_object(
    e: &mut Engine,
    file: Ptr,
    item: u32,
    object_at: Ptr,
    value_at: Ptr,
    field: u32,
) {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let object = if block != 0 {
        fn_00500910(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(object_at.addr(), object);
    e.call(0x0047_27f0, &args![file, value_at]);
    let value = e.mem.u32(value_at.addr());
    e.mem.set_u32(object.wrapping_add(field), value);
    let list = e.call(EXTRA_DATA, &args![item.wrapping_add(0x10)]).u32();
    e.call(LIST_ADD_TAIL, &args![list, object_at]);
}

/// A word read from the chunk into a cleared local.
fn read_word(e: &mut Engine, file: Ptr, local: Ptr) -> u32 {
    e.mem.set_u32(local.addr(), 0);
    e.call(0x0047_27f0, &args![file, local]);
    e.mem.u32(local.addr())
}

// Translated from 00500000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00500000 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `BGSTerminal`'s `Load` (`RET 4`): `TESForm::LoadForm` (Xbox PDB,
/// `00485110`), clears form flag 0x8, then reads every chunk of `file`.
/// The terminal's own chunks: `EDID` (vtable slot 0x134), `OBND` (slot
/// 0xE0), `FULL` (`00487050` on `+0x30`), `MODL`/`MODD`/`MOSD`/`MODT`
/// (`TESModel::LoadModelChunk` on `+0x3C`), `MODS` (texture swaps into
/// `+0x3C`), `SCRI` (`006ecd40` and `TESScriptableForm`'s `InitItem` on
/// `+0x5C`), `DSTD`/`DEST` (`+0x68`), `DESC` (`+0x90`), `SNAM` (`004febb0`),
/// `PNAM` (`00500a20`) and `DNAM` (4 bytes at `+0xA4`); the component
/// address is null when `this` is null, as in the exe.
///
/// `ITXT` starts a menu item (0x78 bytes, `00500960`, its response text
/// set from the chunk, appended through `00501170`); the item chunks that
/// follow fill the current item (the ones checking for one do nothing
/// without it): `RNAM` (result text), `ANAM` (1 byte flags), `INAM`
/// (display note id), `TNAM` (submenu id), `CTDA` (`TESCondition::Load`),
/// `SCHR` (the script header into `00500940`'s address, `00414220` on it
/// when `00401680` says so), `SCDA` (`Script::SetCompileData` with a zeroed
/// buffer of the chunk's size), `SCRO`/`SCRV` (referenced objects), and
/// `SLSD`/`SCVR`: the next chunk is read; an `SLSD` there makes a new
/// `ScriptVariable` (0x20 bytes, `005e28b0`) kept in `011c83d4`, loaded
/// (`005e2a20`) and appended to the script's variable list (`005d43e0`);
/// anything else names the current variable (`004037f0` on its `+0x18`)
/// when there is one. Returns true.
pub fn fn_00500000(e: &mut Engine, this: Ptr<BGSTerminal>, file: Ptr) -> bool {
    let base = this.addr();
    let component = |offset: u32| if base != 0 { base + offset } else { 0 };
    e.call(0x0048_5110, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    // The current menu item (`[EBP-0x10]`).
    let mut item = 0u32;
    e.with_stack(0x20, |e, frame| {
        let word_at = frame;
        let scro_object_at = frame.byte_add(4);
        let scro_value_at = frame.byte_add(8);
        let scrv_object_at = frame.byte_add(12);
        let scrv_value_at = frame.byte_add(16);
        loop {
            let chunk = e.call(0x0047_26b0, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                SIG_DNAM => {
                    e.call(0x0047_2890, &args![file, base + 0xa4, 4u32]);
                }
                SIG_SLSD | SIG_SCVR => {
                    let next = e.call(0x0047_26b0, &args![file]).u32();
                    if next == SIG_SLSD {
                        let block = e.call(ALLOCATE, &args![0x20u32]).u32();
                        let variable = if block != 0 {
                            e.call(0x005e_28b0, &args![block]).u32()
                        } else {
                            0
                        };
                        e.set_global(CURRENT_VARIABLE, variable);
                        let variable = e.global::<u32>(CURRENT_VARIABLE);
                        e.call(0x005e_2a20, &args![variable, file]);
                        let list = e.call(0x005d_43e0, &args![item.wrapping_add(0x10)]).u32();
                        e.call(LIST_ADD_TAIL, &args![list, CURRENT_VARIABLE]);
                    } else if e.global::<u32>(CURRENT_VARIABLE) != 0 {
                        let size = e.call(0x0040_1660, &args![file]).u32();
                        alloca(e, size, |e, buffer| {
                            e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                            let variable = e.global::<u32>(CURRENT_VARIABLE);
                            e.call(STRING_SET, &args![variable + 0x18, buffer, 0u32]);
                        });
                    }
                }
                SIG_MODD | SIG_MODL | SIG_MOSD | SIG_MODT => {
                    e.call(0x0048_92d0, &args![component(0x3c), file]);
                }
                SIG_SCDA => {
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    alloca(e, size, |e, buffer| {
                        e.call(0x0040_3d30, &args![buffer, 0u32, size]);
                        e.call(0x0047_2890, &args![file, buffer, 0u32]);
                        e.call(0x005a_ae70, &args![item.wrapping_add(0x10), size, buffer]);
                    });
                }
                SIG_CTDA => {
                    if item != 0 {
                        e.call(0x0068_0960, &args![item + 0x64, file]);
                    }
                }
                SIG_DESC => {
                    e.call(0x0048_2d50, &args![component(0x90), file]);
                }
                SIG_EDID => {
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    alloca(e, size, |e, buffer| {
                        e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                        e.vcall(base, 0x134, &args![buffer]);
                    });
                }
                SIG_OBND => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                SIG_DSTD | SIG_DEST => {
                    e.call(0x0047_81e0, &args![component(0x68), file]);
                }
                SIG_SCRI => {
                    let script = read_word(e, file, word_at);
                    e.call(0x006e_cd40, &args![base + 0x5c, script]);
                    e.call(0x0048_cd90, &args![base + 0x5c, this]);
                }
                SIG_FULL => {
                    e.call(0x0048_7050, &args![component(0x30), file]);
                }
                SIG_ANAM => {
                    if item != 0 {
                        e.call(0x0047_2890, &args![file, item + 0x74, 1u32]);
                    }
                }
                SIG_SCHR => {
                    let header = fn_00500940(e, Ptr::new(item.wrapping_add(0x10)));
                    e.call(0x0047_2890, &args![file, header, 0u32]);
                    if e.call(0x0040_1680, &args![file]).bool() {
                        let header = fn_00500940(e, Ptr::new(item.wrapping_add(0x10)));
                        e.call(0x0041_4220, &args![header]);
                    }
                }
                SIG_PNAM => {
                    let note = read_word(e, file, word_at);
                    fn_00500a20(e, this, note);
                }
                SIG_INAM => {
                    if item != 0 {
                        let note = read_word(e, file, word_at);
                        e.mem.set_u32(item + 0x6c, note);
                    }
                }
                SIG_TNAM => {
                    if item != 0 {
                        let submenu = read_word(e, file, word_at);
                        e.mem.set_u32(item + 0x70, submenu);
                    }
                }
                SIG_RNAM => {
                    if item != 0 {
                        let size = e.call(0x0040_1660, &args![file]).u32();
                        alloca(e, size, |e, buffer| {
                            e.call(0x0047_2890, &args![file, buffer, 0u32]);
                            e.call(STRING_SET, &args![item + 8, buffer, 0u32]);
                        });
                    }
                }
                SIG_SNAM => {
                    let sound = read_word(e, file, word_at);
                    e.call(0x004f_ebb0, &args![this, sound]);
                }
                SIG_SCRO => {
                    load_referenced_object(e, file, item, scro_object_at, scro_value_at, 8);
                }
                SIG_SCRV => {
                    load_referenced_object(e, file, item, scrv_object_at, scrv_value_at, 0xc);
                }
                SIG_MODS => {
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    alloca(e, size, |e, buffer| {
                        e.call(0x0047_2890, &args![file, buffer, size]);
                        let swap_flags = e.call(0x0040_3570, &args![file]).u16();
                        e.call(
                            0x0048_a7b0,
                            &args![base + 0x3c, buffer, size, u32::from(swap_flags)],
                        );
                    });
                }
                SIG_ITXT => {
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    alloca(e, size, |e, buffer| {
                        e.call(0x0047_2890, &args![file, buffer, 0u32]);
                        let block = e.call(ALLOCATE, &args![0x78u32]).u32();
                        let new_item = if block != 0 {
                            fn_00500960(e, Ptr::new(block)).addr()
                        } else {
                            0
                        };
                        e.call(STRING_SET, &args![new_item, buffer, 0u32]);
                        fn_00501170(e, this, new_item);
                        item = new_item;
                    });
                }
                _ => {}
            }
            if !e.call(0x0047_26f0, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

// Translated from 00500960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00500960 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `TERMINAL_MENU_ITEM` constructor: the two strings (`004037b0` at
/// `+0` and `+8`), the result script (`Script::Script`, `005aa0f0`, at
/// `+0x10`), the conditions (`00680890` at `+0x64`); the script gets form
/// flag 0x8 cleared (`00484ab0`), is made temporary
/// (`TESForm::SetTemporary`, `00484490`) and the byte at `+0x10` of its
/// header (`00500940`) cleared; no display note or submenu, no flags.
/// Returns `this`.
pub fn fn_00500960(e: &mut Engine, this: Ptr<TerminalMenuItem>) -> Ptr<TerminalMenuItem> {
    let base = this.addr();
    e.call(STRING_CONSTRUCT, &args![base]);
    e.call(STRING_CONSTRUCT, &args![base + 8]);
    e.call(0x005a_a0f0, &args![base + 0x10]);
    e.call(0x0068_0890, &args![base + 0x64]);
    e.call(SET_FORM_FLAG_8, &args![base + 0x10, false]);
    e.call(0x0048_4490, &args![base + 0x10]);
    let header = fn_00500940(e, Ptr::new(base + 0x10));
    e.mem.set_u8(header.addr() + 0x10, 0);
    e.set(this, TerminalMenuItem::pDisplayNote, 0);
    e.set(this, TerminalMenuItem::pSubMenu, 0);
    e.set(this, TerminalMenuItem::cFlags, 0);
    this
}

// Translated from 00500a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `this + 0xA0` (`pPassword` of a
/// `BGSTerminal`; `Load` gives it the `PNAM` word, and the folded body has
/// other callers); `RET 4`.
pub fn fn_00500a20(e: &mut Engine, this: Ptr<BGSTerminal>, value: u32) {
    e.set(this, BGSTerminal::pPassword, value);
}

// Translated from 00501050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the menu item list at `+0x98`: while its first node is not
/// empty (`008256d0`), takes the item (`006815c0`), removes the first node
/// (`0063f7b0`) and deletes the item (`005010c0` with 1) when not null.
pub fn fn_00501050(e: &mut Engine, this: Ptr<BGSTerminal>) {
    let list = this.addr() + 0x98;
    while !e.call(NODE_IS_EMPTY, &args![list]).bool() {
        let item_at = e.call(NODE_ITEM, &args![list]).u32();
        let item = e.mem.u32(item_at);
        e.call(LIST_REMOVE_FIRST, &args![list]);
        if item != 0 {
            fn_005010c0(e, Ptr::new(item), 1);
        }
    }
}

// Translated from 005010f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005010f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `TERMINAL_MENU_ITEM` destructor body: the conditions (`006808b0` on
/// `+0x64`), the result script (`005aa1a0` on `+0x10`), then the two strings
/// (`004037d0` on `+8` and `+0`).
pub fn fn_005010f0(e: &mut Engine, this: Ptr<TerminalMenuItem>) {
    let base = this.addr();
    e.call(0x0068_08b0, &args![base + 0x64]);
    e.call(0x005a_a1a0, &args![base + 0x10]);
    e.call(STRING_DESTRUCT, &args![base + 8]);
    e.call(STRING_DESTRUCT, &args![base]);
}

// Translated from 00501170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a menu item to the list at `+0x98` (`00905820` with the address
/// of the argument) when it is not null; `RET 4`.
pub fn fn_00501170(e: &mut Engine, this: Ptr<BGSTerminal>, item: u32) {
    if item != 0 {
        e.with_stack(4, |e, item_at| {
            e.mem.set_u32(item_at.addr(), item);
            e.call(LIST_ADD_TAIL, &args![this.addr() + 0x98, item_at]);
        });
    }
}

// Translated from 005011a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::GetHackDifficultyLockLevel` (Xbox PDB): the raw difficulty
/// (`00501a30` with `reference`); for a terminal with flag bit 0
/// (`00501250`) it is raised by `fHackLevelMult` times a level, the
/// reference's (`TESObjectREFR::GetCalcLevel`, `00567e10`, with 0) or,
/// without a reference, the player's (`0087f9f0` on `*011dea3c`, a `u16`),
/// the product truncated (`FISTP qword` toward zero) and its low word
/// added; the result is at most 4 (unsigned); `RET 4`.
pub fn bgs_terminal_get_hack_difficulty_lock_level(
    e: &mut Engine,
    this: Ptr<BGSTerminal>,
    reference: u32,
) -> u32 {
    let mut level = bgs_terminal_get_raw_difficulty(e, this, reference) as u32;
    if fn_00501250(e, this) {
        let actor_level = if reference != 0 {
            e.call(0x0056_7e10, &args![reference, 0u32]).u32()
        } else {
            let player = e.global::<u32>(PLAYER);
            u32::from(e.call(0x0087_f9f0, &args![player]).u16())
        };
        // FILD qword of (level, 0).
        let level_f = f64::from(actor_level);
        let mult_at = e.call(SETTING_VALUE, &args![HACK_LEVEL_MULT]).u32();
        let mult = x87_load32(e.mem.u32(mult_at));
        let product = x87_mul(mult, level_f);
        let raise = fistp_truncated_qword(product) as u32;
        level = raise.wrapping_add(level);
        if level > 4 {
            level = 4;
        }
    }
    level
}

// Translated from 00501250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of `Data.cFlags` (`+0xA5`) is set (the flag that makes
/// `GetHackDifficultyLockLevel` scale with level).
pub fn fn_00501250(e: &mut Engine, this: Ptr<BGSTerminal>) -> bool {
    e.get(this, BGSTerminal::cFlags) & 1 != 0
}

// Translated from 005015d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of `Data.cFlags` (`+0xA5`) is set (the answer
/// `IsUnlocked` starts from).
pub fn fn_005015d0(e: &mut Engine, this: Ptr<BGSTerminal>) -> bool {
    e.get(this, BGSTerminal::cFlags) & 2 != 0
}

// Translated from 00501760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::IsHackEligible` (Xbox PDB): whether `skill` is at least
/// the minimum skill to hack (`005017a0` with `reference`); `FCOMPP` with
/// the `C0` test, so a NaN is not eligible; `RET 8`.
pub fn bgs_terminal_is_hack_eligible(
    e: &mut Engine,
    this: Ptr<BGSTerminal>,
    reference: u32,
    skill: f32,
) -> bool {
    let minimum = bgs_terminal_get_min_skill_required_to_hack(e, this, reference);
    let skill = x87_load32(skill.to_bits());
    // C0 is set for skill < minimum or unordered: eligible otherwise.
    skill >= minimum
}

// Translated from 005017a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::GetMinSkillRequiredToHack` (Xbox PDB): by the hack lock
/// level (`005011a0` with `reference`), the value of
/// `fHackingMinSkillVeryEasy`, `Easy`, `Average`, `Hard` or `VeryHard`
/// (levels 0 to 4, the compiler's jump table at `0050181c`), the `float` at
/// `01016970` past them; returned in `ST0` (`FLD dword`); `RET 4`.
pub fn bgs_terminal_get_min_skill_required_to_hack(
    e: &mut Engine,
    this: Ptr<BGSTerminal>,
    reference: u32,
) -> f64 {
    let level = bgs_terminal_get_hack_difficulty_lock_level(e, this, reference);
    let setting = match level {
        0 => HACKING_MIN_SKILL[0],
        1 => HACKING_MIN_SKILL[1],
        2 => HACKING_MIN_SKILL[2],
        3 => HACKING_MIN_SKILL[3],
        4 => HACKING_MIN_SKILL[4],
        _ => return x87_load32(e.global::<u32>(UNHACKABLE_MIN_SKILL)),
    };
    let value_at = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value_at))
}

// Translated from 00501870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::IsHacked` (Xbox PDB), cdecl with one word (`ECX` is not
/// read): whether `reference`'s terminal state (`ExtraDataList::
/// GetTerminalState`, `0041bf50`, on its extra data `005d43c0`) has bit
/// 0x80 set; false without a reference.
pub fn bgs_terminal_is_hacked(e: &mut Engine, reference: u32) -> bool {
    if reference == 0 {
        return false;
    }
    let extra = e.call(EXTRA_DATA, &args![reference]).u32();
    let state = e.call(EXTRA_TERMINAL_STATE, &args![extra]).u8();
    state & 0x80 != 0
}

// Translated from 00501990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::IsLockedOut` (Xbox PDB), cdecl with one word: false
/// without a reference; otherwise by the low 7 bits of its terminal state:
/// 0 is false; 1 runs the hacking entry point
/// (`BGSEntryPoint::HandleEntryPoint`, `005e58f0`, cdecl: point 0x14, the
/// player, the address of a `float` starting at 0.0) and is true when that
/// `float` equals the `double` at `01012060` (`FCOMP`, `TEST AH,0x44`: not
/// for NaN); any other state is true.
pub fn bgs_terminal_is_locked_out(e: &mut Engine, reference: u32) -> bool {
    if reference == 0 {
        return false;
    }
    let extra = e.call(EXTRA_DATA, &args![reference]).u32();
    let state = e.call(EXTRA_TERMINAL_STATE, &args![extra]).u8() & 0x7f;
    match state {
        0 => false,
        1 => e.with_stack(4, |e, value_at| {
            e.mem.set_f32(value_at.addr(), 0.0);
            let player = e.global::<u32>(PLAYER);
            e.call(0x005e_58f0, &args![0x14u32, player, value_at]);
            let value = x87_load32(e.mem.u32(value_at.addr()));
            let compare = e.global::<f64>(LOCKOUT_COMPARE);
            value == compare
        }),
        _ => true,
    }
}

// Translated from 00501a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::GetRawDifficulty` (Xbox PDB): the lock level of
/// `reference`'s extra data (`0041c060` on `005d43c0`'s answer, a signed
/// byte), or -1 without a reference; when negative, `Data.cDifficulty`
/// (`+0xA4`) instead; sign-extended; `RET 4`.
pub fn bgs_terminal_get_raw_difficulty(
    e: &mut Engine,
    this: Ptr<BGSTerminal>,
    reference: u32,
) -> i32 {
    let mut level: i8 = -1;
    if reference != 0 {
        let extra = e.call(EXTRA_DATA, &args![reference]).u32();
        level = e.call(EXTRA_LOCK_LEVEL, &args![extra]).u8() as i8;
    }
    if level < 0 {
        level = e.get(this, BGSTerminal::cDifficulty) as i8;
    }
    i32::from(level)
}

// Translated from 00501ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerminal::IsUnlocked` (Xbox PDB): starts from flag bit 1
/// (`005015d0`); with a reference, its lock level (`0041c060` on its extra
/// data) of -2 makes it true, any level but -1 false, -1 keeps it;
/// `RET 4`.
pub fn bgs_terminal_is_unlocked(e: &mut Engine, this: Ptr<BGSTerminal>, reference: u32) -> bool {
    let mut unlocked = fn_005015d0(e, this);
    if reference != 0 {
        let extra = e.call(EXTRA_DATA, &args![reference]).u32();
        let level = e.call(EXTRA_LOCK_LEVEL, &args![extra]).u8() as i8;
        if level == -2 {
            unlocked = true;
        } else if level != -1 {
            unlocked = false;
        }
    }
    unlocked
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fffe0, fn_004fffe0(u32)),
        entry!(0x00500910, fn_00500910(Ptr) -> Ptr),
        entry!(0x00500940, fn_00500940(Ptr) -> Ptr),
        entry!(0x0050_10c0, fn_005010c0(Ptr, u32) -> Ptr),
        entry!(0x004f_fcb0, bgs_terminal_init_item(Ptr<BGSTerminal>)),
        entry!(0x0050_0000, fn_00500000(Ptr<BGSTerminal>, Ptr) -> bool),
        entry!(
            0x0050_0960,
            fn_00500960(Ptr<TerminalMenuItem>) -> Ptr<TerminalMenuItem>
        ),
        entry!(0x0050_0a20, fn_00500a20(Ptr<BGSTerminal>, u32)),
        entry!(0x0050_1050, fn_00501050(Ptr<BGSTerminal>)),
        entry!(0x0050_10f0, fn_005010f0(Ptr<TerminalMenuItem>)),
        entry!(0x0050_1170, fn_00501170(Ptr<BGSTerminal>, u32)),
        entry!(
            0x0050_11a0,
            bgs_terminal_get_hack_difficulty_lock_level(Ptr<BGSTerminal>, u32) -> u32
        ),
        entry!(0x0050_1250, fn_00501250(Ptr<BGSTerminal>) -> bool),
        entry!(0x0050_15d0, fn_005015d0(Ptr<BGSTerminal>) -> bool),
        entry!(
            0x0050_1760,
            bgs_terminal_is_hack_eligible(Ptr<BGSTerminal>, u32, f32) -> bool
        ),
        entry!(
            0x0050_17a0,
            bgs_terminal_get_min_skill_required_to_hack(Ptr<BGSTerminal>, u32) -> f64
        ),
        entry!(0x0050_1870, bgs_terminal_is_hacked(u32) -> bool),
        entry!(0x0050_1990, bgs_terminal_is_locked_out(u32) -> bool),
        entry!(
            0x0050_1a30,
            bgs_terminal_get_raw_difficulty(Ptr<BGSTerminal>, u32) -> i32
        ),
        entry!(
            0x0050_1ae0,
            bgs_terminal_is_unlocked(Ptr<BGSTerminal>, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_word_takes_the_value() {
        let mut e = Engine::new();
        e.call(0x004f_ffe0, &[0x1234_5678]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD), 0x1234_5678);
        e.call(0x004f_ffe0, &[0]);
        assert_eq!(e.mem.u32(tls + TLS_WORD), 0);
    }

    #[test]
    fn referenced_object_constructor_builds_the_string_and_zeroes() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        for offset in [8, 0xc] {
            e.mem.set_u32(object + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_0910, &[object]).u32(), object);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(STRING_CONSTRUCT, vec![object])]);
    }

    #[test]
    fn header_address_matches_native_without_dereferencing_receiver() {
        // Five native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f:
        // EAX is receiver+24 modulo 2^32, RET 0 and no faults.
        let mut e = Engine::new();
        for receiver in [0u32, 1, 0x1000_0000, u32::MAX - 15, u32::MAX] {
            assert_eq!(
                e.call(0x00500940, &[receiver]).u32(),
                receiver.wrapping_add(24)
            );
        }
    }
    #[test]
    fn at_005010c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_10f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_10c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_10c0)
                .collect();
            let mut expected = vec![(0x0050_10f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b023_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    fn terminal(e: &mut Engine) -> Ptr<BGSTerminal> {
        e.new_object()
    }

    /// References 0x100 + n have the lock level n (as a byte) in their
    /// extra data 0x200 + n, and the terminal state n.
    fn reference_doubles(e: &mut Engine) {
        e.register(EXTRA_DATA, |_, a| ret(a[0] + 0x100));
        e.register(EXTRA_LOCK_LEVEL, |_, a| ret(0xff00 | (a[0] - 0x200)));
        e.register(EXTRA_TERMINAL_STATE, |_, a| ret(0xff00 | (a[0] - 0x200)));
    }

    /// Settings: the value of each at `setting + 4` (the double of
    /// `00403e20` gives that address).
    fn setting_doubles(e: &mut Engine, mult: f32) {
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.map(0x011c_8000, 0x2000);
        e.map(0x011d_9000, 0x1000);
        e.map(PLAYER & !0xfff, 0x1000);
        e.mem.set_f32(HACK_LEVEL_MULT + 4, mult);
        for (i, s) in HACKING_MIN_SKILL.iter().enumerate() {
            e.mem.set_f32(s + 4, 10.0 * (i as f32 + 1.0));
        }
        e.map(UNHACKABLE_MIN_SKILL & !0xfff, 0x1000);
        e.set_global(UNHACKABLE_MIN_SKILL, f32::MAX);
        e.set_global(PLAYER, 0x0abc_0000u32);
    }

    #[test]
    fn flags_raw_difficulty_and_unlocked() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let t = terminal(&mut e);
        e.set(t, BGSTerminal::cFlags, 1);
        assert!(e.call(0x0050_1250, &args![t]).bool());
        assert!(!e.call(0x0050_15d0, &args![t]).bool());
        e.set(t, BGSTerminal::cFlags, 2);
        assert!(!e.call(0x0050_1250, &args![t]).bool());
        assert!(e.call(0x0050_15d0, &args![t]).bool());

        e.set(t, BGSTerminal::cDifficulty, 2);
        assert_eq!(e.call(0x0050_1a30, &args![t, 0u32]).i32(), 2);
        assert_eq!(e.call(0x0050_1a30, &args![t, 0x103u32]).i32(), 3);
        assert_eq!(e.call(0x0050_1a30, &args![t, 0x1ffu32]).i32(), 2);
        e.set(t, BGSTerminal::cDifficulty, 0xfe);
        assert_eq!(e.call(0x0050_1a30, &args![t, 0x1ffu32]).i32(), -2);

        // IsUnlocked: flag bit 1 set; -2 unlocks, -1 keeps, others lock.
        assert!(e.call(0x0050_1ae0, &args![t, 0u32]).bool());
        assert!(e.call(0x0050_1ae0, &args![t, 0x1ffu32]).bool());
        assert!(!e.call(0x0050_1ae0, &args![t, 0x105u32]).bool());
        e.set(t, BGSTerminal::cFlags, 0);
        assert!(!e.call(0x0050_1ae0, &args![t, 0x1ffu32]).bool());
        assert!(e.call(0x0050_1ae0, &args![t, 0x1feu32]).bool());
    }

    #[test]
    fn hack_lock_level_scales_with_level() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        setting_doubles(&mut e, 0.25);
        e.register(0x0056_7e10, |_, a| {
            assert_eq!(a[1], 0);
            ret(a[0] - 0x100 + 9)
        });
        e.register(0x0087_f9f0, |_, a| {
            assert_eq!(a[0], 0x0abc_0000);
            ret(0x7_0005)
        });
        let t = terminal(&mut e);
        e.set(t, BGSTerminal::cDifficulty, 1);
        // Without flag bit 0 the raw difficulty stays, uncapped.
        assert_eq!(e.call(0x0050_11a0, &args![t, 0x107u32]).u32(), 7);
        e.set(t, BGSTerminal::cFlags, 1);
        // Reference 0x101: level 10, 0.25 * 10 = 2.5 -> 2, plus 1.
        assert_eq!(e.call(0x0050_11a0, &args![t, 0x101u32]).u32(), 3);
        // Reference 0x103: level 12 -> 3 + 3 = 6, capped at 4.
        assert_eq!(e.call(0x0050_11a0, &args![t, 0x103u32]).u32(), 4);
        // No reference: the player's level as a u16 (5): 1 + 1.
        e.set(t, BGSTerminal::cDifficulty, 1);
        assert_eq!(e.call(0x0050_11a0, &args![t, 0u32]).u32(), 2);
        // A negative product wraps the unsigned sum above 4.
        e.mem.set_f32(HACK_LEVEL_MULT + 4, -0.25);
        assert_eq!(e.call(0x0050_11a0, &args![t, 0x101u32]).u32(), 4);
        // NaN: the integer indefinite has a low word of 0.
        e.mem.set_u32(HACK_LEVEL_MULT + 4, 0x7fc0_0000);
        assert_eq!(e.call(0x0050_11a0, &args![t, 0x101u32]).u32(), 1);
        assert_eq!(fistp_truncated_qword(-2.9), -2);
        assert_eq!(fistp_truncated_qword(1e19), i64::MIN);
    }

    #[test]
    fn min_skill_and_eligibility() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        setting_doubles(&mut e, 0.0);
        let t = terminal(&mut e);
        for level in 0..5u32 {
            let r = 0x100 + level;
            let skill = e.call(0x0050_17a0, &args![t, r]).f64();
            assert_eq!(skill, 10.0 * f64::from(level + 1));
            assert!(e.call(0x0050_1760, &args![t, r, skill as f32]).bool());
            let below = skill as f32 - 0.5;
            assert!(!e.call(0x0050_1760, &args![t, r, below]).bool());
            assert!(!e.call(0x0050_1760, &args![t, r, f32::NAN]).bool());
        }
        // A raw difficulty past 4 (no level scaling) reads 01016970.
        let skill = e.call(0x0050_17a0, &args![t, 0x10au32]).f64();
        assert_eq!(skill, f64::from(f32::MAX));
    }

    #[test]
    fn hacked_and_locked_out() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        e.map(PLAYER & !0xfff, 0x1000);
        e.set_global(PLAYER, 0x0abc_0000u32);
        e.map(LOCKOUT_COMPARE & !0xfff, 0x1000);
        e.set_global(LOCKOUT_COMPARE, 0.0f64);
        assert!(!e.call(0x0050_1870, &[0]).bool());
        assert!(e.call(0x0050_1870, &[0x180]).bool());
        assert!(!e.call(0x0050_1870, &[0x17f]).bool());

        assert!(!e.call(0x0050_1990, &[0]).bool());
        assert!(!e.call(0x0050_1990, &[0x180]).bool());
        assert!(e.call(0x0050_1990, &[0x102]).bool());
        // State 1: the entry point decides through the float.
        let seen: Log = Rc::default();
        let s = seen.clone();
        e.register_double(0x005e_58f0, move |e, a| {
            s.borrow_mut().push(vec![a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        assert!(e.call(0x0050_1990, &[0x181]).bool());
        assert_eq!(*seen.borrow(), vec![vec![0x14, 0x0abc_0000, 0]]);
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        assert!(!e.call(0x0050_1990, &[0x101]).bool());
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_u32(a[2], 0x7fc0_0000);
            Ret::default()
        });
        assert!(!e.call(0x0050_1990, &[0x101]).bool());
    }

    #[test]
    fn menu_item_construction_and_list() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                0x005a_a0f0,
                0x0068_0890,
                SET_FORM_FLAG_8,
                0x0048_4490,
                0x0068_08b0,
                0x005a_a1a0,
                STRING_DESTRUCT,
            ],
        );
        let item = e.mem.alloc(0x78);
        e.mem.write(item, &[0xcd; 0x78]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_0960, &[item]).u32(), item);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (STRING_CONSTRUCT, vec![item]),
                (STRING_CONSTRUCT, vec![item + 8]),
                (0x005a_a0f0, vec![item + 0x10]),
                (0x0068_0890, vec![item + 0x64]),
                (SET_FORM_FLAG_8, vec![item + 0x10, 0]),
                (0x0048_4490, vec![item + 0x10]),
            ]
        );
        assert_eq!(e.mem.u8(item + 0x38), 0);
        assert_eq!(e.mem.u32(item + 0x6c), 0);
        assert_eq!(e.mem.u32(item + 0x70), 0);
        assert_eq!(e.mem.u8(item + 0x74), 0);
        assert_eq!(e.mem.u8(item + 0x75), 0xcd);

        e.call_log = Some(vec![]);
        e.call(0x0050_10f0, &[item]);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (0x0068_08b0, vec![item + 0x64]),
                (0x005a_a1a0, vec![item + 0x10]),
                (STRING_DESTRUCT, vec![item + 8]),
                (STRING_DESTRUCT, vec![item]),
            ]
        );

        let t = terminal(&mut e);
        e.call(0x0050_0a20, &args![t, 0x77u32]);
        assert_eq!(e.get(t, BGSTerminal::pPassword), 0x77);

        let added: Log = Rc::default();
        let a = added.clone();
        e.register_double(LIST_ADD_TAIL, move |e, w| {
            a.borrow_mut().push(vec![w[0], e.mem.u32(w[1])]);
            Ret::default()
        });
        e.call(0x0050_1170, &args![t, 0u32]);
        e.call(0x0050_1170, &args![t, 0x55u32]);
        assert_eq!(*added.borrow(), vec![vec![t.addr() + 0x98, 0x55]]);
    }

    #[test]
    fn remove_all_menu_items() {
        let mut e = Engine::new();
        let t = terminal(&mut e);
        // A list of items 0x11, 0 and 0x33 held in a vector (last first).
        let items = Rc::new(RefCell::new(vec![0x33u32, 0, 0x11]));
        let i = items.clone();
        e.register_double(NODE_IS_EMPTY, move |_, _| {
            ret(u32::from(i.borrow().is_empty()))
        });
        let slot = e.mem.alloc(4);
        let i = items.clone();
        e.register_double(NODE_ITEM, move |e, _| {
            let item = *i.borrow().last().unwrap();
            e.mem.set_u32(slot, item);
            ret(slot)
        });
        let i = items.clone();
        e.register_double(LIST_REMOVE_FIRST, move |_, _| {
            i.borrow_mut().pop();
            Ret::default()
        });
        quiet(&mut e, &[0x0050_10f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        e.call(0x0050_1050, &args![t]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_REMOVE_FIRST).len(), 3);
        assert_eq!(calls_to(&log, 0x0050_10f0), vec![vec![0x11], vec![0x33]]);
        assert_eq!(calls_to(&log, 0x0040_1030), vec![vec![0x11], vec![0x33]]);
        assert_eq!(calls_to(&log, NODE_IS_EMPTY)[0], vec![t.addr() + 0x98]);
    }

    #[test]
    fn init_item_resolves_password_notes_and_submenus() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        let t = terminal(&mut e);
        let ta = t.addr();
        e.call_log = Some(vec![]);
        e.call(0x004f_fcb0, &args![t]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        quiet(
            &mut e,
            &[0x0051_0770, SET_FORM_FLAG_8, WARNING, DISABLE_WARNINGS],
        );
        e.register(GET_FILE, |_, _| ret(0x0400_0000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        // Ids ending in 1 resolve.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] & 0xf == 1 { a[0] + 0x1000 } else { 0 })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(ta, 0x0600_0000);
        e.set(t, BGSTerminal::pPassword, 2);
        // Two menu items in nodes (item, next).
        let first = e.mem.alloc(0x78);
        let second = e.mem.alloc(0x78);
        e.mem.set_u32(first + 0x6c, 0x11);
        e.mem.set_u32(first + 0x70, 0x12);
        e.mem.set_u32(second + 0x70, 0x21);
        let nodes = e.mem.alloc(16);
        e.mem.set_u32(nodes, first);
        e.mem.set_u32(nodes + 4, nodes + 8);
        e.mem.set_u32(nodes + 8, second);
        e.register_double(0x0046_1150, move |_, _| ret(nodes));
        e.register(NODE_IS_EMPTY, |_, _| ret(0));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        // The warning count: 0x40 before the first init, and warnings left
        // by both inits of the second item (calls 6 and 8).
        let count = Rc::new(RefCell::new(0u32));
        let c = count.clone();
        e.register_double(WARNING_COUNT, move |_, _| {
            let mut n = c.borrow_mut();
            *n += 1;
            ret(match *n {
                1 => 0x40,
                6 | 8 => 1,
                _ => 0,
            })
        });
        quiet(&mut e, &[0x005a_b400, 0x0068_0a20]);
        e.call_log = Some(vec![]);
        e.call(0x004f_fcb0, &args![t]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0051_0770), vec![vec![ta]]);
        assert_eq!(e.get(t, BGSTerminal::pPassword), 0);
        assert_eq!(e.mem.u32(first + 0x6c), 0x1111);
        assert_eq!(e.mem.u32(first + 0x70), 0);
        assert_eq!(e.mem.u32(second + 0x70), 0x1121);
        assert_eq!(
            calls_to(&log, WARNING),
            vec![
                vec![MISSING_PASSWORD_MESSAGE, 0x102, 0x0102_0304, 0x00ab_cdef],
                vec![MISSING_SUBMENU_MESSAGE, 0x112, 0, 0x0102_0304, 0x00ab_cdef],
                vec![SCRIPT_ERRORS_MESSAGE, 1, 0x0102_0304, 0x00ab_cdef],
                vec![CONDITION_ERRORS_MESSAGE, 1, 0x0102_0304, 0x00ab_cdef],
            ]
        );
        let casts: Vec<u32> = calls_to(&log, RT_DYNAMIC_CAST)
            .iter()
            .map(|w| w[3])
            .collect();
        assert_eq!(
            casts,
            vec![TYPE_NOTE, TYPE_NOTE, TYPE_TERMINAL, TYPE_TERMINAL]
        );
        assert_eq!(
            calls_to(&log, 0x005a_b400),
            vec![vec![first + 0x10, ta], vec![second + 0x10, ta]]
        );
        assert_eq!(
            calls_to(&log, 0x0068_0a20),
            vec![vec![first + 0x64, ta], vec![second + 0x64, ta]]
        );
        assert_eq!(calls_to(&log, DISABLE_WARNINGS).len(), 8);
        // The TLS word gets back the count read before the last init.
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD), 0);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![ta, 1]]);
    }

    /// Doubles for `Load`: chunks popped from `chunks`, word reads giving
    /// 0x500 + the read's number, sizes of 8, allocations from the heap.
    fn load_doubles(e: &mut Engine, chunks: Vec<u32>) {
        let chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        let c = chunks.clone();
        e.register_double(0x0047_26f0, move |_, _| {
            ret(u32::from(!c.borrow().is_empty()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        let n = Rc::new(RefCell::new(0x500u32));
        e.register_double(0x0047_27f0, move |e, a| {
            let mut k = n.borrow_mut();
            *k += 1;
            e.mem.set_u32(a[1], *k);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0040_3570, |_, _| ret(0xffff_1234));
        e.register(0x005d_43c0, |_, a| ret(a[0] + 0x1000));
        e.register(0x005d_43e0, |_, a| ret(a[0] + 0x2000));
        e.register(0x005e_28b0, |_, a| ret(a[0]));
        quiet(
            e,
            &[
                0x0048_5110,
                SET_FORM_FLAG_8,
                0x0047_2890,
                0x7000_0000,
                0x7000_0004,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x0048_2d50,
                0x004f_ebb0,
                0x0068_0960,
                0x0041_4220,
                0x005a_ae70,
                0x0040_3d30,
                STRING_SET,
                STRING_CONSTRUCT,
                0x005a_a0f0,
                0x0068_0890,
                0x0048_4490,
                0x005e_2a20,
                LIST_ADD_TAIL,
            ],
        );
        e.register(0x0040_1680, |_, _| ret(1));
    }

    #[test]
    fn load_reads_the_terminal_chunks() {
        let mut e = Engine::new();
        e.map(CURRENT_VARIABLE & !0xfff, 0x1000);
        // Before any item: the item chunks that check for one do nothing.
        let mut chunks = vec![
            SIG_EDID, SIG_OBND, SIG_FULL, SIG_MODD, SIG_MODL, SIG_MOSD, SIG_MODT, SIG_MODS,
            SIG_DSTD, SIG_DEST, SIG_DESC, SIG_SCRI, SIG_SNAM, SIG_PNAM, SIG_DNAM, SIG_CTDA,
            SIG_TNAM, SIG_INAM, SIG_ANAM, SIG_RNAM,
        ];
        chunks.reverse();
        load_doubles(&mut e, chunks);
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        let t = terminal(&mut e);
        let ta = t.addr();
        e.mem.set_u32(ta, 0x0600_0000);
        let file = 0x0500_0000u32;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_0000, &[ta, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_5110), vec![vec![ta, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![ta, 0]]);
        let sized = calls_to(&log, 0x0047_2890);
        assert_eq!(sized[0][2], 0x200);
        assert_eq!((sized[1][0], sized[1][2]), (file, 8));
        assert_eq!(sized[2], vec![file, ta + 0xa4, 4]);
        assert_eq!(sized.len(), 3);
        assert_eq!(calls_to(&log, 0x7000_0000).len(), 1);
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![ta, file]]);
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![ta + 0x30, file]]);
        assert_eq!(calls_to(&log, 0x0048_92d0), vec![vec![ta + 0x3c, file]; 4]);
        let swaps = calls_to(&log, 0x0048_a7b0);
        assert_eq!(
            (swaps[0][0], swaps[0][2], swaps[0][3]),
            (ta + 0x3c, 8, 0x1234)
        );
        assert_eq!(calls_to(&log, 0x0047_81e0), vec![vec![ta + 0x68, file]; 2]);
        assert_eq!(calls_to(&log, 0x0048_2d50), vec![vec![ta + 0x90, file]]);
        assert_eq!(calls_to(&log, 0x006e_cd40), vec![vec![ta + 0x5c, 0x501]]);
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![ta + 0x5c, ta]]);
        assert_eq!(calls_to(&log, 0x004f_ebb0), vec![vec![ta, 0x502]]);
        assert_eq!(e.get(t, BGSTerminal::pPassword), 0x503);
        assert!(calls_to(&log, 0x0068_0960).is_empty());
        assert!(calls_to(&log, STRING_SET).is_empty());
        assert_eq!(calls_to(&log, 0x0047_27f0).len(), 3);
    }

    #[test]
    fn load_fills_the_current_menu_item() {
        let mut e = Engine::new();
        e.map(CURRENT_VARIABLE & !0xfff, 0x1000);
        let mut chunks = vec![
            SIG_SCVR, 0x1234, SIG_ITXT, SIG_RNAM, SIG_ANAM, SIG_INAM, SIG_TNAM, SIG_CTDA, SIG_SCHR,
            SIG_SCDA, SIG_SCRO, SIG_SCRV, SIG_SLSD, SIG_SLSD, SIG_SCVR, 0x5678,
        ];
        chunks.reverse();
        load_doubles(&mut e, chunks);
        let added: Log = Rc::default();
        let a = added.clone();
        e.register_double(LIST_ADD_TAIL, move |e, w| {
            a.borrow_mut().push(vec![w[0], w[1], e.mem.u32(w[1])]);
            Ret::default()
        });
        let t = terminal(&mut e);
        let ta = t.addr();
        let file = 0x0500_0000u32;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_0000, &[ta, file]).bool());
        let log = e.call_log.take().unwrap();
        // The first SCVR, without a variable, only reads the next chunk.
        let item = calls_to(&log, 0x005a_a0f0)[0][0] - 0x10;
        let sets = calls_to(&log, STRING_SET);
        assert_eq!(sets[0][0], item);
        assert_eq!(sets[1][0], item + 8);
        let added = added.borrow();
        assert_eq!((added[0][0], added[0][2]), (ta + 0x98, item));
        let sized = calls_to(&log, 0x0047_2890);
        assert!(sized.contains(&vec![file, item + 0x74, 1]));
        assert!(sized.contains(&vec![file, item + 0x28, 0]));
        assert_eq!(e.mem.u32(item + 0x6c), 0x501);
        assert_eq!(e.mem.u32(item + 0x70), 0x502);
        assert_eq!(calls_to(&log, 0x0068_0960), vec![vec![item + 0x64, file]]);
        assert_eq!(calls_to(&log, 0x0041_4220), vec![vec![item + 0x28]]);
        let compile = calls_to(&log, 0x005a_ae70);
        assert_eq!((compile[0][0], compile[0][1]), (item + 0x10, 8));
        assert_eq!(calls_to(&log, 0x0040_3d30), vec![vec![compile[0][2], 0, 8]]);
        // SCRO and SCRV: new objects with the words at +8 and +0xC.
        let (scro, scrv) = (added[1][2], added[2][2]);
        assert_eq!(added[1][0], item + 0x1010);
        assert_eq!(added[2][0], item + 0x1010);
        assert_eq!(e.mem.u32(scro + 8), 0x503);
        assert_eq!(e.mem.u32(scrv + 0xc), 0x504);
        // SLSD then SLSD: a new variable, loaded and appended.
        let variable = e.global::<u32>(CURRENT_VARIABLE);
        assert_ne!(variable, 0);
        assert_eq!(calls_to(&log, 0x005e_2a20), vec![vec![variable, file]]);
        assert_eq!(added[3], vec![item + 0x2010, CURRENT_VARIABLE, variable]);
        // SCVR then another chunk names the variable.
        assert_eq!(sets[2][0], variable + 0x18);
        assert_eq!(sets[2][2], 0);
        assert_eq!(sets.len(), 3);
    }
}
