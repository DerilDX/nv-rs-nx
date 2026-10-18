//! `fallout shared/tesqueststage.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The members of a quest stage item (`TESQuestStageItem`, 0x74 bytes on PC;
//! its layout is declared in `tesquest.rs`, where the constructor sits):
//! the record loader `TESQuestStageItem::Load`, the item's `InitItem`
//! (resolving its owner quest and next quest after the plugin is loaded),
//! the "set done" that `SetStage` reaches through `0060f4a0` (log date,
//! completing or failing the quest, the player's log, running the result
//! script) and the two flag tests it uses.
//!
//! The engine map names `0060fa10` `TESQuestStage::InitItem` and `0060fb60`
//! `TESQuestStage::SetDone`, but both run on a stage *item* (`this + 4`
//! conditions, `this + 0xc` result script, `+0x68` log date, `+0x6c` owner,
//! `+0x70` next quest), so they are named by address here.
//!
//! Not translated: `TESQuestStageItem::GetLogEntry` (`0060fdb0`), which
//! neither the loaders nor `SetStage` reach.

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_0060bef0::TESQuest;
use super::tesquest::TESQuestStageItem;
use super::tesscript::ScriptHeader;

/// The file reader (`TESFile`): chunk type (`004726b0`), chunk size
/// (`00401660`), chunk data (`00472890`: buffer, size; 0 reads the whole
/// chunk), a dword (`004727f0`: address of the destination), next chunk
/// (`004726f0`), byte-swap flag (`00401680`) and the offset of the current
/// record (`00467bb0`, the word at `file + 0x264`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_GET_CHUNK_DWORD: u32 = 0x0047_27f0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_SWAP_FLAG: u32 = 0x0040_1680;
const FILE_RECORD_OFFSET: u32 = 0x0046_7bb0;
/// `operator new` (`00401000`, cdecl) and the `memset` wrapper (`00403d30`,
/// cdecl: destination, value, count).
const OPERATOR_NEW: u32 = 0x0040_1000;
const MEMSET: u32 = 0x0040_3d30;
/// `00500940`: `this + 0x18` (the `SCRIPT_HEADER` of a `Script`).
const SCRIPT_HEADER: u32 = 0x0050_0940;
/// `00414220` (this = the script header): swaps it in place.
const SCRIPT_HEADER_SWAP: u32 = 0x0041_4220;
/// `Script::SetCompileData` (`005aae70`: size, data).
const SCRIPT_SET_COMPILE_DATA: u32 = 0x005a_ae70;
/// `ScriptVariable` constructor (`005e28b0`, 0x20 bytes) and
/// `ScriptVariable::Load` (`005e2a20`, one stack word: the file).
const SCRIPT_VARIABLE_CONSTRUCT: u32 = 0x005e_28b0;
const SCRIPT_VARIABLE_LOAD: u32 = 0x005e_2a20;
/// The `SCRIPT_REFERENCED_OBJECT` constructor (`00500910`, 0x10 bytes).
const REFERENCED_OBJECT_CONSTRUCT: u32 = 0x0050_0910;
/// `005d43c0` / `005d43e0`: `this + 0x44` / `this + 0x4c` (a `Script`'s
/// referenced-object list and variable list; `005d43c0` on the player is
/// its extra data list).
const WORD_AT_44: u32 = 0x005d_43c0;
const WORD_AT_4C: u32 = 0x005d_43e0;
/// List insertion (`00905820`, this = the list, one stack word: the
/// address of the item pointer).
const LIST_INSERT: u32 = 0x0090_5820;
/// `004037f0` (this = a `BSStringT`): sets it from a string (string, a
/// length word).
const STRING_SET: u32 = 0x0040_37f0;
/// The `ScriptVariable` the loader filled last (`SLSD`), named by the next
/// `SCVR` chunk: a global word.
const CURRENT_SCRIPT_VARIABLE: u32 = 0x011c_b8ec;
/// `00717e50`: `this + 4` (the item's `TESCondition`); `0048d150`:
/// `this + 0xc` (the item's result `Script`).
const ITEM_CONDITIONS: u32 = 0x0071_7e50;
const ITEM_SCRIPT: u32 = 0x0048_d150;
/// `TESCondition::InitItem` (`00680a20`, one stack word: the form) and
/// `TESCondition::IsTrue` (`00680c30`: subject, target).
const CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// The thread's InitItem error count: get (`0046e8a0`) and set (`004fffe0`,
/// cdecl), the word at +0x2b8 of the thread's TLS block.
const GET_ERROR_COUNT: u32 = 0x0046_e8a0;
const SET_ERROR_COUNT: u32 = 0x004f_ffe0;
/// `MessageHandler::IncDisableWarningCount` (`0043b2b0`, cdecl, one word).
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
/// `Script::InitItem` (`005ab400`, one stack word: the quest).
const SCRIPT_INIT_ITEM: u32 = 0x005a_b400;
/// `0084e3a0`: the word at `this + 0xc` (the form ID).
const FORM_ID: u32 = 0x0084_e3a0;
/// `009373f0`: the byte at `this` (a stage's number).
const READ_BYTE_0: u32 = 0x0093_73f0;
/// The warning printer (`005b5e40`, cdecl, a format and its arguments).
const WARNING: u32 = 0x005b_5e40;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, 0, source type, target
/// type, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `TESForm` and `TESQuest`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_QUEST: u32 = 0x0118_6500;
/// `TESForm::GetFile` (`00484e60`, one stack word: the index, -1 the
/// last), `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of the
/// form ID, file) and the form lookup by ID (`004839c0`, cdecl).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `0057bd60` (one stack word): stores it at `this + 0x70`.
const SET_WORD_AT_70: u32 = 0x0057_bd60;
/// `TESQuest` vtable slot `0x130`: its editor ID string.
const SLOT_EDITOR_ID: u32 = 0x130;
/// The warning formats (`.rdata`).
const MSG_INIT_ITEM_ERRORS: u32 = 0x0104_ae98;
const MSG_NO_NEXT_QUEST: u32 = 0x0104_ae54;
const MSG_ALREADY_LOGGED: u32 = 0x0104_af30;
/// Quest bit 3 test (`0060d5e0`), the completed setter (`0060ca30`, one
/// stack word) and `TESQuest::SetFailed` (`0060caf0`, one stack word).
const QUEST_FLAG_BIT_3: u32 = 0x0060_d5e0;
const QUEST_SET_COMPLETED: u32 = 0x0060_ca30;
const QUEST_SET_FAILED: u32 = 0x0060_caf0;
/// The player (`PlayerCharacter *`, a global word).
const PLAYER: u32 = 0x011d_ea3c;
/// `005cbb50`: the player's active quest (`this + 0x6b8`);
/// `PlayerCharacter::SetActiveQuest` (`009529d0`, one stack word);
/// `PlayerCharacter::AddQuestStageItem` (`009528c0`, one stack word).
const PLAYER_ACTIVE_QUEST: u32 = 0x005c_bb50;
const PLAYER_SET_ACTIVE_QUEST: u32 = 0x0095_29d0;
const PLAYER_ADD_QUEST_STAGE_ITEM: u32 = 0x0095_28c0;
/// The game calendar (`011de7b8`) and its year (`00867c60`), month
/// (`Calendar::GetMonth`, `00867d20`) and day (`Calendar::GetDay`,
/// `00867d60`) getters.
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_YEAR: u32 = 0x0086_7c60;
const CALENDAR_MONTH: u32 = 0x0086_7d20;
const CALENDAR_DAY: u32 = 0x0086_7d60;
/// The 4-byte date constructor (`004b0fc0`: day, month, year).
const DATE_CONSTRUCT: u32 = 0x004b_0fc0;
/// `Script::GetProcessScripts` (`005ac740`), `ExtraDataList::GetScriptLocals`
/// (`00418830`) and `Script::Run` (`005ac1e0`, four stack words).
const PROCESS_SCRIPTS: u32 = 0x005a_c740;
const GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
const SCRIPT_RUN: u32 = 0x005a_c1e0;

/// The offset of the item's result `Script` (`cResultScript`, Xbox PDB).
const RESULT_SCRIPT: u32 = 0x0c;

/// Chunk tags, as the little-endian words the file reader returns.
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_QSDT: u32 = 0x5444_5351;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_NAM0: u32 = 0x304d_414e;

/// A new `SCRIPT_REFERENCED_OBJECT` (0 when the allocation fails) whose
/// word at `field` is the chunk's dword, added to the result script's
/// referenced-object list.
fn load_referenced_object(e: &mut Engine, this: Ptr<TESQuestStageItem>, file: Ptr, field: u32) {
    let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let object = if memory != 0 {
        e.call(REFERENCED_OBJECT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    e.with_stack(8, |e, frame| {
        let (cell, value) = (frame.addr(), frame.addr() + 4);
        e.mem.set_u32(cell, object);
        e.call(FILE_GET_CHUNK_DWORD, &args![file, value]);
        let word = e.mem.u32(value);
        e.mem.set_u32(object + field, word);
        let list = e
            .call(WORD_AT_44, &args![this.addr() + RESULT_SCRIPT])
            .u32();
        e.call(LIST_INSERT, &args![list, cell]);
    });
}

// Translated from 0060f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuestStageItem::Load` (Xbox PDB): reads the file's current chunk
/// into the item, then moves to the next chunk. False for a null file.
/// `SCHR` fills the result script's header (`+0xc + 0x18`, swapped in place
/// when the reader swaps), `SCDA` hands a zeroed copy of the chunk to
/// `Script::SetCompileData`, `SLSD` creates a `ScriptVariable` (0x20
/// bytes, remembered in the global `011cb8ec`), loads it and adds it to the
/// script's variable list, `SCVR` names that variable (`+0x18`, up to 0x200
/// bytes), `SCRO` / `SCRV` add a referenced object holding the chunk's
/// dword at +8 / +0xc, `QSDT` stores the record's file offset (+0x60) and
/// reads the flags byte (+0). Then, after a successful move to the next
/// chunk, a `NAM0` chunk gives the next quest (+0x70, as a form ID) and the
/// reader moves once more. Returns whether the last move succeeded.
///
/// The chunk is read again for `SLSD` and `SCVR` (they share a branch).
/// Chunk buffers the exe takes from its stack (`alloca`) are stack blocks
/// here.
// Translation incomplete: 0060f6a0 - SEH exception unwinding and the stack security cookie (/GS) check 00ec408c are not translated.
pub fn tes_quest_stage_item_load(e: &mut Engine, this: Ptr<TESQuestStageItem>, file: Ptr) -> bool {
    if file == Ptr::NULL {
        return false;
    }
    let script = this.addr() + RESULT_SCRIPT;
    match e.call(FILE_CHUNK_TYPE, &args![file]).u32() {
        CHUNK_SCHR => {
            let header = e.call(SCRIPT_HEADER, &args![script]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, header, 0u32]);
            if e.call(FILE_SWAP_FLAG, &args![file]).bool() {
                let header = e.call(SCRIPT_HEADER, &args![script]).u32();
                e.call(SCRIPT_HEADER_SWAP, &args![header]);
            }
        }
        CHUNK_SCDA => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, buffer| {
                e.call(MEMSET, &args![buffer, 0u32, size]);
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                e.call(SCRIPT_SET_COMPILE_DATA, &args![script, size, buffer]);
            });
        }
        CHUNK_SLSD | CHUNK_SCVR => {
            if e.call(FILE_CHUNK_TYPE, &args![file]).u32() == CHUNK_SLSD {
                let memory = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let variable = if memory != 0 {
                    e.call(SCRIPT_VARIABLE_CONSTRUCT, &args![memory]).u32()
                } else {
                    0
                };
                e.set_global(CURRENT_SCRIPT_VARIABLE, variable);
                let variable: u32 = e.global(CURRENT_SCRIPT_VARIABLE);
                e.call(SCRIPT_VARIABLE_LOAD, &args![variable, file]);
                let list = e.call(WORD_AT_4C, &args![script]).u32();
                e.call(LIST_INSERT, &args![list, CURRENT_SCRIPT_VARIABLE]);
            } else if e.global::<u32>(CURRENT_SCRIPT_VARIABLE) != 0 {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    let variable: u32 = e.global(CURRENT_SCRIPT_VARIABLE);
                    // ScriptVariable::cName (Xbox PDB) +0x18
                    e.call(STRING_SET, &args![variable + 0x18, buffer, 0u32]);
                });
            }
        }
        CHUNK_SCRO => {
            // SCRIPT_REFERENCED_OBJECT::pForm (Xbox PDB) +0x08
            load_referenced_object(e, this, file, 0x08);
        }
        CHUNK_SCRV => {
            // SCRIPT_REFERENCED_OBJECT::uiVariableID (Xbox PDB) +0x0C
            load_referenced_object(e, this, file, 0x0c);
        }
        CHUNK_QSDT => {
            let offset = e.call(FILE_RECORD_OFFSET, &args![file]).u32();
            e.set(this, TESQuestStageItem::m_fileOffset, offset);
            e.call(FILE_GET_CHUNK_DATA, &args![file, this, 1u32]);
        }
        _ => {}
    }
    if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
        return false;
    }
    if e.call(FILE_CHUNK_TYPE, &args![file]).u32() == CHUNK_NAM0 {
        let next = e.with_stack(4, |e, value| {
            e.call(FILE_GET_CHUNK_DWORD, &args![file, value]);
            e.mem.u32(value.addr())
        });
        e.set(this, TESQuestStageItem::m_pNextQuest, Ptr::new(next));
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            return false;
        }
    }
    true
}

// Translated from 0060fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stage item's `InitItem` (`this` = the item; the engine map's
/// `TESQuestStage::InitItem`, see the module note): initializes the item's
/// conditions (`TESCondition::InitItem` with the quest) when it has them,
/// then the result script (`Script::InitItem` with the quest) with warnings
/// disabled and the thread's error count cleared; when that count is
/// non-zero afterwards it prints a warning with the stage number, the
/// quest's editor ID (vtable slot `0x130`) and form ID. The count is put
/// back. The owner (+0x6c) becomes the quest cast to `TESQuest`; a next
/// quest form ID (+0x70) is adjusted to the quest's file
/// (`TESForm::AddCompileIndex`), looked up, cast to `TESQuest` (with a
/// warning when that fails) and stored back (`0057bd60`).
pub fn fn_0060fa10(e: &mut Engine, this: Ptr<TESQuestStageItem>, quest: Ptr<TESQuest>, stage: Ptr) {
    if e.call(ITEM_CONDITIONS, &args![this]).u32() != 0 {
        let conditions = e.call(ITEM_CONDITIONS, &args![this]).u32();
        e.call(CONDITION_INIT_ITEM, &args![conditions, quest]);
    }
    let saved_errors = e.call(GET_ERROR_COUNT, &args![]).u32();
    e.call(DISABLE_WARNINGS, &args![1u32]);
    e.call(SET_ERROR_COUNT, &args![0u32]);
    let script = e.call(ITEM_SCRIPT, &args![this]).u32();
    e.call(SCRIPT_INIT_ITEM, &args![script, quest]);
    e.call(DISABLE_WARNINGS, &args![0u32]);
    let errors = e.call(GET_ERROR_COUNT, &args![]).u32();
    e.call(SET_ERROR_COUNT, &args![saved_errors]);
    if errors != 0 {
        let form_id = e.call(FORM_ID, &args![quest]).u32();
        let editor_id = e.vcall(quest.addr(), SLOT_EDITOR_ID, &args![]).u32();
        let number = e.call(READ_BYTE_0, &args![stage]).u8();
        e.call(
            WARNING,
            &args![MSG_INIT_ITEM_ERRORS, u32::from(number), editor_id, form_id],
        );
    }
    let owner = e
        .call(
            DYNAMIC_CAST,
            &args![quest, 0u32, TYPE_TES_FORM, TYPE_TES_QUEST, 0u32],
        )
        .u32();
    e.set(this, TESQuestStageItem::m_pOwner, Ptr::new(owner));
    let next = e.get(this, TESQuestStageItem::m_pNextQuest).addr();
    if next == 0 {
        return;
    }
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), next);
        let file = e.call(FORM_GET_FILE, &args![quest, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![cell, file]);
        let form_id = e.mem.u32(cell.addr());
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let found = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_QUEST, 0u32],
            )
            .u32();
        if found == 0 {
            let form_id = e.mem.u32(cell.addr());
            e.call(WARNING, &args![MSG_NO_NEXT_QUEST, form_id]);
        }
        e.call(SET_WORD_AT_70, &args![this, found]);
    });
}

// Translated from 0060fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks a stage item done (`this` = the item; the engine map's
/// `TESQuestStage::SetDone`, see the module note; `0060f4a0` calls it for
/// each item of a stage with its flag). An item that already has a log date
/// and an owner whose quest bit 3 (`0060d5e0`) is clear only prints a
/// warning. Unless `force` is set, the item's conditions must hold for the
/// player (`TESCondition::IsTrue`). Then: a new 4-byte date from the
/// calendar's day, month and year becomes the log date (+0x68; null when the
/// allocation fails); flag bit 0 (`0060fd70`) completes the owner quest
/// (`0060ca30` with 1) and bit 1 (`0060fd90`) fails it
/// (`TESQuest::SetFailed` with 1), each clearing the player's active quest
/// when it is the owner; an item with a log entry (+0x65) is added to the
/// player's log; with script processing on and a result script whose
/// header's data size is not zero, the header's quest-script byte is
/// cleared and the script runs on the player with the player's script
/// locals (`Script::Run` with 0 and 1).
// Translation incomplete: 0060fb60 - SEH exception unwinding (the frame around the date allocation) is not translated.
pub fn fn_0060fb60(e: &mut Engine, this: Ptr<TESQuestStageItem>, force: u8) {
    let owner = e.get(this, TESQuestStageItem::m_pOwner).addr();
    if e.get(this, TESQuestStageItem::m_pLogDate) != Ptr::NULL
        && owner != 0
        && !e.call(QUEST_FLAG_BIT_3, &args![owner]).bool()
    {
        e.call(WARNING, &args![MSG_ALREADY_LOGGED]);
        return;
    }
    if force == 0 {
        let player: u32 = e.global(PLAYER);
        let conditions = e.call(ITEM_CONDITIONS, &args![this]).u32();
        if !e
            .call(CONDITION_IS_TRUE, &args![conditions, player, 0u32])
            .bool()
        {
            return;
        }
    }
    let memory = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let date = if memory != 0 {
        let year = e.call(CALENDAR_YEAR, &args![CALENDAR]).u32();
        let month = e.call(CALENDAR_MONTH, &args![CALENDAR]).u32();
        let day = e.call(CALENDAR_DAY, &args![CALENDAR]).u8();
        // MOVSX CX,AL; MOVZX EDX,CX: the day byte sign-extended to 16 bits.
        let day = u32::from(i16::from(day as i8) as u16);
        e.call(DATE_CONSTRUCT, &args![memory, day, month, year])
            .u32()
    } else {
        0
    };
    e.set(this, TESQuestStageItem::m_pLogDate, Ptr::new(date));
    if fn_0060fd70(e, this) && e.get(this, TESQuestStageItem::m_pOwner) != Ptr::NULL {
        let owner = e.get(this, TESQuestStageItem::m_pOwner);
        e.call(QUEST_SET_COMPLETED, &args![owner, 1u32]);
        clear_active_quest_if_owner(e, this);
    }
    if fn_0060fd90(e, this) && e.get(this, TESQuestStageItem::m_pOwner) != Ptr::NULL {
        let owner = e.get(this, TESQuestStageItem::m_pOwner);
        e.call(QUEST_SET_FAILED, &args![owner, 1u32]);
        clear_active_quest_if_owner(e, this);
    }
    if e.get(this, TESQuestStageItem::m_bHasLogEntry) != 0 {
        let player: u32 = e.global(PLAYER);
        e.call(PLAYER_ADD_QUEST_STAGE_ITEM, &args![player, this]);
    }
    let script = e.call(ITEM_SCRIPT, &args![this]).u32();
    if !e.call(PROCESS_SCRIPTS, &args![]).bool() || script == 0 {
        return;
    }
    let header: Ptr<ScriptHeader> = e.call_as(SCRIPT_HEADER, &args![script]);
    if e.get(header, ScriptHeader::dataSize) == 0 {
        return;
    }
    let header: Ptr<ScriptHeader> = e.call_as(SCRIPT_HEADER, &args![script]);
    e.set(header, ScriptHeader::bIsQuestScript, false);
    let player: u32 = e.global(PLAYER);
    let extra = e.call(WORD_AT_44, &args![player]).u32();
    let locals = e.call(GET_SCRIPT_LOCALS, &args![extra]).u32();
    let player: u32 = e.global(PLAYER);
    e.call(SCRIPT_RUN, &args![script, player, locals, 0u32, 1u32]);
}

/// Clears the player's active quest (`SetActiveQuest(0)`) when it is the
/// item's owner.
fn clear_active_quest_if_owner(e: &mut Engine, this: Ptr<TESQuestStageItem>) {
    let player: u32 = e.global(PLAYER);
    let active = e.call(PLAYER_ACTIVE_QUEST, &args![player]).u32();
    if e.get(this, TESQuestStageItem::m_pOwner).addr() == active {
        let player: u32 = e.global(PLAYER);
        e.call(PLAYER_SET_ACTIVE_QUEST, &args![player, 0u32]);
    }
}

// Translated from 0060fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the item's flags byte (`m_Data`, +0): `SetDone` completes the
/// owner quest when it is set.
pub fn fn_0060fd70(e: &mut Engine, this: Ptr<TESQuestStageItem>) -> bool {
    e.get(this, TESQuestStageItem::iFlags) & 1 != 0
}

// Translated from 0060fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of the item's flags byte (`m_Data`, +0): `SetDone` fails the
/// owner quest when it is set.
pub fn fn_0060fd90(e: &mut Engine, this: Ptr<TESQuestStageItem>) -> bool {
    e.get(this, TESQuestStageItem::iFlags) & 2 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0060f6a0,
            tes_quest_stage_item_load(Ptr<TESQuestStageItem>, Ptr) -> bool
        ),
        entry!(
            0x0060fa10,
            fn_0060fa10(Ptr<TESQuestStageItem>, Ptr<TESQuest>, Ptr)
        ),
        entry!(0x0060fb60, fn_0060fb60(Ptr<TESQuestStageItem>, u8)),
        entry!(0x0060fd70, fn_0060fd70(Ptr<TESQuestStageItem>) -> bool),
        entry!(0x0060fd90, fn_0060fd90(Ptr<TESQuestStageItem>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A double that returns `value` whatever it is called with.
    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| eax(value));
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    // ---- TESQuestStageItem::Load ----------------------------------------

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// An engine with a file reader over `chunks` (tag, bytes); the first
    /// is current. `next chunk` drops the current one and fails when none
    /// is left.
    fn load_world(
        chunks: Vec<(u32, Vec<u8>)>,
        swap: bool,
    ) -> (Engine, Ptr<TESQuestStageItem>, Chunks) {
        let mut e = Engine::new();
        let chunks: Chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            eax(c.borrow().first().map_or(0, |(t, _)| *t))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            eax(c.borrow().first().map_or(0, |(_, d)| d.len() as u32))
        });
        let c = chunks.clone();
        e.register_double(FILE_GET_CHUNK_DATA, move |e, a| {
            let data = c.borrow()[0].1.clone();
            let n = if a[2] == 0 {
                data.len()
            } else {
                data.len().min(a[2] as usize)
            };
            e.mem.write(a[1], &data[..n]);
            eax(1)
        });
        let c = chunks.clone();
        e.register_double(FILE_GET_CHUNK_DWORD, move |e, a| {
            let data = c.borrow()[0].1.clone();
            e.mem.write(a[1], &data[..4]);
            eax(1)
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.remove(0);
            eax(!c.is_empty() as u32)
        });
        stub(&mut e, FILE_SWAP_FLAG, swap as u32);
        stub(&mut e, FILE_RECORD_OFFSET, 0x1234);
        e.register(SCRIPT_HEADER, |_, a| eax(a[0] + 0x18));
        e.register(WORD_AT_44, |_, a| eax(a[0] + 0x44));
        e.register(WORD_AT_4C, |_, a| eax(a[0] + 0x4c));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            eax(a[0])
        });
        e.register(REFERENCED_OBJECT_CONSTRUCT, |_, a| eax(a[0]));
        e.register(SCRIPT_VARIABLE_CONSTRUCT, |_, a| eax(a[0]));
        for addr in [
            SCRIPT_HEADER_SWAP,
            SCRIPT_SET_COMPILE_DATA,
            SCRIPT_VARIABLE_LOAD,
            LIST_INSERT,
            STRING_SET,
        ] {
            stub(&mut e, addr, 0);
        }
        e.map(CURRENT_SCRIPT_VARIABLE, 4);
        let item: Ptr<TESQuestStageItem> = e.new_object();
        e.call_log = Some(vec![]);
        (e, item, chunks)
    }

    #[test]
    fn item_load_null_file_returns_false() {
        let (mut e, item, _) = load_world(vec![], false);
        assert!(!tes_quest_stage_item_load(&mut e, item, Ptr::NULL));
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn item_load_qsdt_reads_flags_offset_and_next_quest() {
        let (mut e, item, chunks) = load_world(
            vec![
                (CHUNK_QSDT, vec![3]),
                (CHUNK_NAM0, 0x0001_0203u32.to_le_bytes().to_vec()),
                (CHUNK_SCHR, vec![0; 4]),
            ],
            false,
        );
        let file = Ptr::new(0x5000);
        assert!(tes_quest_stage_item_load(&mut e, item, file));
        assert_eq!(e.get(item, TESQuestStageItem::iFlags), 3);
        assert_eq!(e.get(item, TESQuestStageItem::m_fileOffset), 0x1234);
        assert_eq!(
            e.get(item, TESQuestStageItem::m_pNextQuest).addr(),
            0x0001_0203
        );
        assert_eq!(
            calls_to(&e, FILE_GET_CHUNK_DATA),
            vec![vec![0x5000, item.addr(), 1]]
        );
        assert_eq!(chunks.borrow().len(), 1);
        assert_eq!(calls_to(&e, FILE_NEXT_CHUNK).len(), 2);
    }

    #[test]
    fn item_load_returns_false_when_the_move_fails() {
        let (mut e, item, _) = load_world(vec![(CHUNK_QSDT, vec![1])], false);
        assert!(!tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        // NAM0 last: the second move fails.
        let (mut e, item, _) = load_world(
            vec![(CHUNK_QSDT, vec![1]), (CHUNK_NAM0, vec![1, 0, 0, 0])],
            false,
        );
        assert!(!tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        assert_eq!(e.get(item, TESQuestStageItem::m_pNextQuest).addr(), 1);
    }

    #[test]
    fn item_load_schr_fills_and_swaps_the_header() {
        let (mut e, item, _) =
            load_world(vec![(CHUNK_SCHR, vec![7; 0x14]), (0x1111, vec![])], true);
        assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        let header = item.addr() + 0xc + 0x18;
        assert_eq!(e.mem.u8(header), 7);
        assert_eq!(
            calls_to(&e, FILE_GET_CHUNK_DATA),
            vec![vec![0x5000, header, 0]]
        );
        assert_eq!(calls_to(&e, SCRIPT_HEADER_SWAP), vec![vec![header]]);
        let (mut e, item, _) =
            load_world(vec![(CHUNK_SCHR, vec![7; 0x14]), (0x1111, vec![])], false);
        tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000));
        assert!(calls_to(&e, SCRIPT_HEADER_SWAP).is_empty());
    }

    #[test]
    fn item_load_scda_passes_size_and_a_copy_to_set_compile_data() {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let (mut e, item, _) =
            load_world(vec![(CHUNK_SCDA, vec![1, 2, 3]), (0x1111, vec![])], false);
        let s = seen.clone();
        e.register_double(SCRIPT_SET_COMPILE_DATA, move |e, a| {
            s.borrow_mut().push((a[0], a[1], e.mem.bytes(a[2], a[1])));
            eax(0)
        });
        assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        assert_eq!(*seen.borrow(), vec![(item.addr() + 0xc, 3, vec![1, 2, 3])]);
        let memset = calls_to(&e, MEMSET);
        assert_eq!((memset[0][1], memset[0][2]), (0, 3));
    }

    #[test]
    fn item_load_slsd_creates_a_variable_and_scvr_names_it() {
        let (mut e, item, _) = load_world(vec![(CHUNK_SLSD, vec![0; 24]), (0x1111, vec![])], false);
        assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        let variable: u32 = e.global(CURRENT_SCRIPT_VARIABLE);
        assert_ne!(variable, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x20]]);
        assert_eq!(
            calls_to(&e, SCRIPT_VARIABLE_LOAD),
            vec![vec![variable, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, LIST_INSERT),
            vec![vec![item.addr() + 0xc + 0x4c, CURRENT_SCRIPT_VARIABLE]]
        );

        let (mut e, item, _) = load_world(
            vec![(CHUNK_SCVR, b"fTimer\0".to_vec()), (0x1111, vec![])],
            false,
        );
        e.set_global(CURRENT_SCRIPT_VARIABLE, 0x7000u32);
        assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        let set = calls_to(&e, STRING_SET);
        assert_eq!((set[0][0], set[0][2]), (0x7018, 0));
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_DATA)[0][2], 0x200);

        // No variable yet: SCVR is skipped.
        let (mut e, item, _) =
            load_world(vec![(CHUNK_SCVR, b"x\0".to_vec()), (0x1111, vec![])], false);
        assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
        assert!(calls_to(&e, STRING_SET).is_empty());
        assert!(calls_to(&e, FILE_CHUNK_SIZE).is_empty());
    }

    #[test]
    fn item_load_scro_and_scrv_add_referenced_objects() {
        for (tag, field) in [(CHUNK_SCRO, 8), (CHUNK_SCRV, 0xc)] {
            let inserted = Rc::new(RefCell::new(Vec::new()));
            let (mut e, item, _) = load_world(
                vec![(tag, 0x55u32.to_le_bytes().to_vec()), (0x1111, vec![])],
                false,
            );
            let i = inserted.clone();
            e.register_double(LIST_INSERT, move |e, a| {
                i.borrow_mut().push((a[0], e.mem.u32(a[1])));
                eax(0)
            });
            assert!(tes_quest_stage_item_load(&mut e, item, Ptr::new(0x5000)));
            let (list, object) = inserted.borrow()[0];
            assert_eq!(list, item.addr() + 0xc + 0x44);
            assert_eq!(e.mem.u32(object + field), 0x55);
            assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        }
    }

    // ---- InitItem ---------------------------------------------------------

    const QUEST_EDITOR_ID: u32 = 0x00ab_0000;

    fn init_world(
        conditions: bool,
        errors: u32,
        next: u32,
        found: u32,
    ) -> (Engine, Ptr<TESQuestStageItem>, Ptr<TESQuest>, Ptr) {
        let mut e = Engine::new();
        e.register_double(ITEM_CONDITIONS, move |_, a| {
            eax(if conditions { a[0] + 4 } else { 0 })
        });
        e.register(ITEM_SCRIPT, |_, a| eax(a[0] + 0xc));
        let count = Rc::new(RefCell::new(vec![5u32, errors]));
        e.register_double(GET_ERROR_COUNT, move |_, _| {
            eax(count.borrow_mut().remove(0))
        });
        e.register(FORM_ID, |e, a| eax(e.mem.u32(a[0] + 0xc)));
        e.register(READ_BYTE_0, |e, a| eax(u32::from(e.mem.u8(a[0]))));
        e.register_double(QUEST_EDITOR_ID, |_, _| eax(0x6000));
        e.register_double(DYNAMIC_CAST, move |_, a| {
            eax(if a[0] == 0x9100 { found } else { a[0] })
        });
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            eax(0)
        });
        e.register(LOOKUP_FORM_BY_ID, |_, _| eax(0x9100));
        e.register(SET_WORD_AT_70, |e, a| {
            e.mem.set_u32(a[0] + 0x70, a[1]);
            eax(0)
        });
        for addr in [
            CONDITION_INIT_ITEM,
            DISABLE_WARNINGS,
            SET_ERROR_COUNT,
            SCRIPT_INIT_ITEM,
            WARNING,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, FORM_GET_FILE, 0x4400);
        let item: Ptr<TESQuestStageItem> = e.new_object();
        e.set(item, TESQuestStageItem::m_pNextQuest, Ptr::new(next));
        let quest: Ptr<TESQuest> = e.new_object();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_EDITOR_ID, QUEST_EDITOR_ID);
        e.mem.set_u32(quest.addr(), vtable);
        e.mem.set_u32(quest.addr() + 0xc, 0x0012_3456);
        let stage = Ptr::new(e.mem.alloc(0xc));
        e.mem.set_u8(stage.addr(), 20);
        e.call_log = Some(vec![]);
        (e, item, quest, stage)
    }

    #[test]
    fn init_item_initializes_conditions_and_script_and_sets_the_owner() {
        let (mut e, item, quest, stage) = init_world(true, 0, 0, 0);
        fn_0060fa10(&mut e, item, quest, stage);
        let q = quest.addr();
        assert_eq!(
            calls_to(&e, CONDITION_INIT_ITEM),
            vec![vec![item.addr() + 4, q]]
        );
        assert_eq!(
            calls_to(&e, SCRIPT_INIT_ITEM),
            vec![vec![item.addr() + 0xc, q]]
        );
        assert_eq!(calls_to(&e, DISABLE_WARNINGS), vec![vec![1], vec![0]]);
        assert_eq!(calls_to(&e, SET_ERROR_COUNT), vec![vec![0], vec![5]]);
        assert!(calls_to(&e, WARNING).is_empty());
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![q, 0, TYPE_TES_FORM, TYPE_TES_QUEST, 0]]
        );
        assert_eq!(e.get(item, TESQuestStageItem::m_pOwner), quest.cast());
        assert!(calls_to(&e, LOOKUP_FORM_BY_ID).is_empty());
    }

    #[test]
    fn init_item_warns_on_script_errors_and_skips_missing_conditions() {
        let (mut e, item, quest, stage) = init_world(false, 2, 0, 0);
        fn_0060fa10(&mut e, item, quest, stage);
        assert!(calls_to(&e, CONDITION_INIT_ITEM).is_empty());
        assert_eq!(
            calls_to(&e, WARNING),
            vec![vec![MSG_INIT_ITEM_ERRORS, 20, 0x6000, 0x0012_3456]]
        );
    }

    #[test]
    fn init_item_resolves_the_next_quest() {
        let (mut e, item, quest, stage) = init_world(false, 0, 0x42, 0x9200);
        fn_0060fa10(&mut e, item, quest, stage);
        assert_eq!(
            calls_to(&e, FORM_GET_FILE),
            vec![vec![quest.addr(), 0xffff_ffff]]
        );
        assert_eq!(calls_to(&e, LOOKUP_FORM_BY_ID), vec![vec![0x0100_0042]]);
        assert_eq!(e.get(item, TESQuestStageItem::m_pNextQuest).addr(), 0x9200);
        assert!(calls_to(&e, WARNING).is_empty());

        let (mut e, item, quest, stage) = init_world(false, 0, 0x42, 0);
        fn_0060fa10(&mut e, item, quest, stage);
        assert_eq!(
            calls_to(&e, WARNING),
            vec![vec![MSG_NO_NEXT_QUEST, 0x0100_0042]]
        );
        assert_eq!(e.get(item, TESQuestStageItem::m_pNextQuest), Ptr::NULL);
    }

    // ---- SetDone ----------------------------------------------------------

    struct DoneWorld {
        e: Engine,
        item: Ptr<TESQuestStageItem>,
        owner: u32,
        script: u32,
    }

    fn done_world(conditions_true: bool, bit_3: bool, process: bool) -> DoneWorld {
        let mut e = Engine::new();
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x8800u32);
        e.register(ITEM_CONDITIONS, |_, a| eax(a[0] + 4));
        e.register(ITEM_SCRIPT, |_, a| eax(a[0] + 0xc));
        e.register(SCRIPT_HEADER, |_, a| eax(a[0] + 0x18));
        e.register(WORD_AT_44, |_, a| eax(a[0] + 0x44));
        stub(&mut e, CONDITION_IS_TRUE, conditions_true as u32);
        stub(&mut e, QUEST_FLAG_BIT_3, bit_3 as u32);
        stub(&mut e, PROCESS_SCRIPTS, process as u32);
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        stub(&mut e, CALENDAR_YEAR, 2281);
        stub(&mut e, CALENDAR_MONTH, 9);
        stub(&mut e, CALENDAR_DAY, 0xff);
        e.register(DATE_CONSTRUCT, |_, a| eax(a[0]));
        stub(&mut e, PLAYER_ACTIVE_QUEST, 0);
        stub(&mut e, GET_SCRIPT_LOCALS, 0x7700);
        for addr in [
            WARNING,
            QUEST_SET_COMPLETED,
            QUEST_SET_FAILED,
            PLAYER_SET_ACTIVE_QUEST,
            PLAYER_ADD_QUEST_STAGE_ITEM,
            SCRIPT_RUN,
        ] {
            stub(&mut e, addr, 0);
        }
        let item: Ptr<TESQuestStageItem> = e.new_object();
        let owner = e.mem.alloc(0x6c);
        e.set(item, TESQuestStageItem::m_pOwner, Ptr::new(owner));
        let script = item.addr() + 0xc;
        e.call_log = Some(vec![]);
        DoneWorld {
            e,
            item,
            owner,
            script,
        }
    }

    #[test]
    fn set_done_warns_for_an_item_already_logged() {
        let mut w = done_world(true, false, false);
        w.e.set(w.item, TESQuestStageItem::m_pLogDate, Ptr::new(0x1000));
        fn_0060fb60(&mut w.e, w.item, 1);
        assert_eq!(calls_to(&w.e, WARNING), vec![vec![MSG_ALREADY_LOGGED]]);
        assert_eq!(
            w.e.get(w.item, TESQuestStageItem::m_pLogDate).addr(),
            0x1000
        );
        // With bit 3 set the item is logged again.
        let mut w = done_world(true, true, false);
        w.e.set(w.item, TESQuestStageItem::m_pLogDate, Ptr::new(0x1000));
        fn_0060fb60(&mut w.e, w.item, 1);
        assert!(calls_to(&w.e, WARNING).is_empty());
        assert_ne!(
            w.e.get(w.item, TESQuestStageItem::m_pLogDate).addr(),
            0x1000
        );
    }

    #[test]
    fn set_done_checks_conditions_unless_forced() {
        let mut w = done_world(false, false, false);
        fn_0060fb60(&mut w.e, w.item, 0);
        assert_eq!(
            calls_to(&w.e, CONDITION_IS_TRUE),
            vec![vec![w.item.addr() + 4, 0x8800, 0]]
        );
        assert_eq!(w.e.get(w.item, TESQuestStageItem::m_pLogDate), Ptr::NULL);
        assert!(calls_to(&w.e, OPERATOR_NEW).is_empty());

        let mut w = done_world(false, false, false);
        fn_0060fb60(&mut w.e, w.item, 1);
        assert!(calls_to(&w.e, CONDITION_IS_TRUE).is_empty());
        let date = w.e.get(w.item, TESQuestStageItem::m_pLogDate).addr();
        assert_ne!(date, 0);
        // The day byte 0xff is sign-extended to the 16-bit 0xffff.
        assert_eq!(
            calls_to(&w.e, DATE_CONSTRUCT),
            vec![vec![date, 0xffff, 9, 2281]]
        );
    }

    #[test]
    fn set_done_completes_or_fails_the_owner_and_clears_the_active_quest() {
        let mut w = done_world(true, false, false);
        w.e.set(w.item, TESQuestStageItem::iFlags, 3);
        w.e.set(w.item, TESQuestStageItem::m_bHasLogEntry, 1);
        let owner = w.owner;
        stub(&mut w.e, PLAYER_ACTIVE_QUEST, owner);
        fn_0060fb60(&mut w.e, w.item, 0);
        assert_eq!(calls_to(&w.e, QUEST_SET_COMPLETED), vec![vec![owner, 1]]);
        assert_eq!(calls_to(&w.e, QUEST_SET_FAILED), vec![vec![owner, 1]]);
        assert_eq!(
            calls_to(&w.e, PLAYER_SET_ACTIVE_QUEST),
            vec![vec![0x8800, 0], vec![0x8800, 0]]
        );
        assert_eq!(
            calls_to(&w.e, PLAYER_ADD_QUEST_STAGE_ITEM),
            vec![vec![0x8800, w.item.addr()]]
        );
        let order: Vec<u32> = addresses(&w.e)
            .into_iter()
            .filter(|a| {
                [
                    QUEST_SET_COMPLETED,
                    QUEST_SET_FAILED,
                    PLAYER_ADD_QUEST_STAGE_ITEM,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                QUEST_SET_COMPLETED,
                QUEST_SET_FAILED,
                PLAYER_ADD_QUEST_STAGE_ITEM
            ]
        );

        // No flags, another active quest: nothing.
        let mut w = done_world(true, false, false);
        fn_0060fb60(&mut w.e, w.item, 0);
        assert!(calls_to(&w.e, QUEST_SET_COMPLETED).is_empty());
        assert!(calls_to(&w.e, PLAYER_SET_ACTIVE_QUEST).is_empty());
        assert!(calls_to(&w.e, PLAYER_ADD_QUEST_STAGE_ITEM).is_empty());
    }

    #[test]
    fn set_done_runs_the_result_script_on_the_player() {
        let mut w = done_world(true, false, true);
        let header = w.script + 0x18;
        w.e.mem.set_u32(header + 8, 0x40);
        w.e.mem.set_u8(header + 0x10, 1);
        fn_0060fb60(&mut w.e, w.item, 1);
        assert_eq!(w.e.mem.u8(header + 0x10), 0);
        assert_eq!(calls_to(&w.e, GET_SCRIPT_LOCALS), vec![vec![0x8800 + 0x44]]);
        assert_eq!(
            calls_to(&w.e, SCRIPT_RUN),
            vec![vec![w.script, 0x8800, 0x7700, 0, 1]]
        );

        // An empty script, or script processing off: no run.
        let mut w = done_world(true, false, true);
        w.e.mem.set_u8(w.script + 0x18 + 0x10, 1);
        fn_0060fb60(&mut w.e, w.item, 1);
        assert!(calls_to(&w.e, SCRIPT_RUN).is_empty());
        assert_eq!(w.e.mem.u8(w.script + 0x18 + 0x10), 1);
        let mut w = done_world(true, false, false);
        w.e.mem.set_u32(w.script + 0x18 + 8, 0x40);
        fn_0060fb60(&mut w.e, w.item, 1);
        assert!(calls_to(&w.e, SCRIPT_RUN).is_empty());
    }

    #[test]
    fn flag_bits_0_and_1() {
        let mut e = Engine::new();
        let item: Ptr<TESQuestStageItem> = e.new_object();
        for (flags, complete, fail) in [
            (0, false, false),
            (1, true, false),
            (2, false, true),
            (0xff, true, true),
        ] {
            e.set(item, TESQuestStageItem::iFlags, flags);
            assert_eq!(fn_0060fd70(&mut e, item), complete);
            assert_eq!(fn_0060fd90(&mut e, item), fail);
        }
    }
}
