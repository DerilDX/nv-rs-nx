//! `fallout shared/bgsperk.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005eaab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005eaab0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x56 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_6ec4`) is `BGSPerk`, so this is the constructor of `BGSPerk`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005eaab0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0040_2c20, &args![base + 0x2c]);
    e.mem.set_u32(base, 0x0104_6ec4);
    e.mem.set_u32(base + 0x18, 0x0104_6eac);
    e.mem.set_u32(base + 0x24, 0x0104_6e94);
    e.mem.set_u32(base + 0x2c, 0x0104_6e74);
    e.call(0x0068_0890, &args![base + 0x40]);
    e.call(0x0096_a2d0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x56u32]);
    e.call(0x005e_ac70, &args![this]);
    this
}

// Translated from 005eb680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 8`, zero-extended. The engine map has no
/// name for it; the condition item's `InitItem` (`006819e0`) calls it.
pub fn fn_005eb680(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 8))
}

// Translated from 005e9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry` scalar deleting destructor (engine map name): runs the destructor body `005ea000` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bgs_entry_point_perk_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_a000, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005eab80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005eabb0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005eab80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_abb0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `00401000`: `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030`: `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00401660`: the current chunk's size (`ECX` = the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680`: whether the file's data needs its byte order converted.
const NEEDS_BYTE_SWAP: u32 = 0x0040_1680;
/// `00401080`: byte-swaps the word at the address (cdecl; 0 as second word).
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;
/// `TESFile::GetTESForm` (`00472660`, engine map): the record type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (`004726b0`, engine map): 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false when there is none.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESForm::LoadForm` (`00485110`, engine map).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`tesform.cpp`): `ECX` = the form, one word.
const TES_FORM_MARK_00484AB0: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (`00484e60`, engine map).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map): cdecl.
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`tesform.cpp`): looks a form id up (cdecl, one word).
const FORM_LOOKUP_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`): five words.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `00474cb0`: the form's editor id length (0 when it has none).
const EDITOR_ID_LENGTH: u32 = 0x0047_4cb0;
/// `0084e3a0`: the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40`: the warning printer (cdecl, format and its arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// `006815c0`: returns its `ECX` (a list node's item is its first word).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070`: the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0`: whether a list node is empty.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `TESCondition::GetConditionItem` (`00680f70`, engine map): one word.
const GET_CONDITION_ITEM: u32 = 0x0068_0f70;
/// `00413f40`: a condition item's data (function index `u16` at +0, first
/// parameter at +4, second at +8).
const CONDITION_DATA: u32 = 0x0041_3f40;
/// `00406d00`: `sprintf_s` shape (cdecl: buffer, size, format, ...).
const STRING_FORMAT: u32 = 0x0040_6d00;
/// `00406d50`: `strcat_s` shape (cdecl: buffer, size, source).
const STRING_APPEND: u32 = 0x0040_6d50;
/// `00403df0`: a `Setting`'s string (`ECX` = the setting; its word at +4).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `_ftol2` (`00ec62c0`): the `ST0` value as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `004f1540`: a perk entry's rank byte (`AL`).
const ENTRY_RANK: u32 = 0x004f_1540;
/// The player (`PlayerCharacter*`) global the perk code reads.
const PLAYER: u32 = 0x011d_ea3c;

/// `BGSPerkEntry`'s vtable (RTTI).
const PERK_ENTRY_VTABLE: u32 = 0x0104_6b48;
/// `BGSQuestPerkEntry`'s vtable (RTTI).
const QUEST_PERK_ENTRY_VTABLE: u32 = 0x0104_6b84;
/// `BGSAbilityPerkEntry`'s vtable (RTTI).
const ABILITY_PERK_ENTRY_VTABLE: u32 = 0x0104_6c44;
/// `BGSEntryPointPerkEntry`'s vtable (RTTI).
const ENTRY_POINT_PERK_ENTRY_VTABLE: u32 = 0x0104_6d0c;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TESQuest`'s RTTI type descriptor (`.?AVTESQuest@@`).
const TYPE_TES_QUEST: u32 = 0x0118_6500;
/// `SpellItem`'s RTTI type descriptor (`.?AVSpellItem@@`).
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// `"MASTERFILE: Unable to find quest (%08X) on owner object \"%s\"."`.
const NO_QUEST_NAMED: u32 = 0x0104_6c00;
/// `"MASTERFILE: Unable to find quest (%08X) on owner object (%08X)."`.
const NO_QUEST_ID: u32 = 0x0104_6bc0;
/// `"MASTERFILE: Unable to find ability (%08X) on owner object \"%s\"."`.
const NO_ABILITY_NAMED: u32 = 0x0104_6cc8;
/// `"MASTERFILE: Unable to find ability (%08X) on owner object (%08X)."`.
const NO_ABILITY_ID: u32 = 0x0104_6c80;
/// `"FORMS: BGSEntryPointPerkEntry::CheckConditionFilters() was called with %i filter forms, but there are %i conditions"`.
const FILTER_COUNT_WARNING: u32 = 0x0104_6da0;
/// `"DEFAULT: Bad Filter form %08X %s sent to BGSEntryPointPerkEntry::CheckConditionFilters."`.
const BAD_FILTER_WARNING: u32 = 0x0104_6d48;
/// `"MASTERFILE: Function data chunk is loading without function data existing on perk entry"`.
const NO_FUNCTION_DATA_WARNING: u32 = 0x0104_6e18;

const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_PRKF: u32 = 0x464b_5250;
const CHUNK_PRKE: u32 = 0x454b_5250;
const CHUNK_PRKC: u32 = 0x434b_5250;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_EPFT: u32 = 0x5446_5045;
const CHUNK_EPFD: u32 = 0x4446_5045;
const CHUNK_EPF2: u32 = 0x3246_5045;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_SCTX: u32 = 0x5854_4353;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// The id-resolving shape shared by the `InitItem`s of the quest and
/// ability entries: the form id at `this + 8` (when not 0) gets `owner`'s
/// file's compile index, is looked up (`004839c0`) and cast from `TESForm`
/// to `target` into +8; when that gives null it warns with the id and the
/// owner's editor id (vtable slot 0x130) when `00474cb0` reports one, else
/// with the owner's form id.
fn resolve_entry_form(
    e: &mut Engine,
    this: Ptr,
    owner: Ptr,
    target: u32,
    named_warning: u32,
    id_warning: u32,
) {
    if e.mem.u32(this.addr() + 8) == 0 {
        return;
    }
    let id = e.with_stack(4, |e, local| {
        let id = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(local.addr(), id);
        let file = e
            .call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    });
    let found = e.call(FORM_LOOKUP_004839C0, &args![id]).u32();
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![found, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + 8, form);
    if form != 0 {
        return;
    }
    if e.call(EDITOR_ID_LENGTH, &args![owner]).u32() != 0 {
        let editor_id = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(PRINT_WARNING, &args![named_warning, id, editor_id]);
    } else {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        e.call(PRINT_WARNING, &args![id_warning, id, owner_id]);
    }
}

// Translated from 005e9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPerkEntry` constructor (its vtable `01046b48`, by RTTI): the vtable,
/// the rank (+0x4) and priority (+0x5) bytes 0. Returns `this`.
pub fn fn_005e9530(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), PERK_ENTRY_VTABLE);
    e.mem.set_u8(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 5, 0);
    this
}

// Translated from 005e96b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the bytes +1 and +2 of the 3-byte `PRKE` header at `header` (one
/// stack word) into +0x4 and +0x5.
pub fn fn_005e96b0(e: &mut Engine, this: Ptr, header: Ptr) {
    let first = e.mem.u8(header.addr() + 1);
    e.mem.set_u8(this.addr() + 4, first);
    let second = e.mem.u8(header.addr() + 2);
    e.mem.set_u8(this.addr() + 5, second);
}

// Translated from 005e96e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e96e0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Creates the perk entry for an entry type (cdecl, one word): 0 a quest
/// entry (0x10 bytes, [`fn_005e9800`]), 1 an ability entry (0xC bytes,
/// [`fn_005e9bd0`]), 2 an entry point entry (0x14 bytes, [`fn_005e9f60`]);
/// null for any other type or when the allocation fails.
pub fn fn_005e96e0(e: &mut Engine, entry_type: u32) -> Ptr {
    let (size, construct): (u32, fn(&mut Engine, Ptr) -> Ptr) = match entry_type {
        0 => (0x10, fn_005e9800),
        1 => (0xc, fn_005e9bd0),
        2 => (0x14, fn_005e9f60),
        _ => return Ptr::NULL,
    };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, Ptr::new(block))
}

// Translated from 005e9800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSQuestPerkEntry` constructor (its vtable `01046b84`, by RTTI): the
/// base ([`fn_005e9530`]), the vtable, the quest +0x8 = null and the stage
/// byte +0xC = 0. Returns `this`.
pub fn fn_005e9800(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e9530(e, this);
    e.mem.set_u32(this.addr(), QUEST_PERK_ENTRY_VTABLE);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u8(this.addr() + 0xc, 0);
    this
}

// Translated from 005e9860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body that only runs `005e9590` (the `BGSPerkEntry` one).
pub fn fn_005e9860(e: &mut Engine, this: Ptr) {
    e.call(0x005e_9590, &args![this]);
}

// Translated from 005e9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSQuestPerkEntry::InitItem` (Xbox PDB): resolves the quest id at +0x8
/// against `owner` (one stack word) as a `TESQuest`, warning when it is
/// not found.
pub fn bgs_quest_perk_entry_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    resolve_entry_form(e, this, owner, TYPE_TES_QUEST, NO_QUEST_NAMED, NO_QUEST_ID);
}

// Translated from 005e9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSQuestPerkEntry::Load` (Xbox PDB): reads the chunks of `file` (one
/// stack word) until none is left or a `PRKF` (the entry's end) comes,
/// returning true: `DATA` must be 8 bytes (else false) and goes into +0x8
/// (byte-swapped by `00503210` when the file needs it).
pub fn bgs_quest_perk_entry_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            return true;
        }
        if chunk == CHUNK_DATA {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            if size != 8 {
                return false;
            }
            e.call(GET_CHUNK_DATA, &args![file, this.addr() + 8, size]);
            if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
                e.call(0x0050_3210, &args![this.addr() + 8]);
            }
        } else if chunk == CHUNK_PRKF {
            return true;
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            return true;
        }
    }
}

// Translated from 005e9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAbilityPerkEntry` constructor (its vtable `01046c44`, by RTTI): the
/// base, the vtable, the ability +0x8 = null. Returns `this`.
pub fn fn_005e9bd0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e9530(e, this);
    e.mem.set_u32(this.addr(), ABILITY_PERK_ENTRY_VTABLE);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 005e9c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAbilityPerkEntry::InitItem` (Xbox PDB): resolves the ability id at
/// +0x8 against `owner` as a `SpellItem`, warning when it is not found.
pub fn bgs_ability_perk_entry_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    resolve_entry_form(
        e,
        this,
        owner,
        TYPE_SPELL_ITEM,
        NO_ABILITY_NAMED,
        NO_ABILITY_ID,
    );
}

// Translated from 005e9e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAbilityPerkEntry::Load` (Xbox PDB): like the quest entry's, with a
/// 4-byte `DATA` (else false) read into a stack word, byte-swapped
/// (`00401080`) when the file needs it, and stored at +0x8.
pub fn bgs_ability_perk_entry_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            return true;
        }
        if chunk == CHUNK_DATA {
            if e.call(CHUNK_SIZE, &args![file]).u32() != 4 {
                return false;
            }
            let id = e.with_stack(4, |e, local| {
                e.call(GET_CHUNK_DATA, &args![file, local, 0u32]);
                if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
                    e.call(SWAP_WORD_IN_PLACE, &args![local, 0u32]);
                }
                e.mem.u32(local.addr())
            });
            e.mem.set_u32(this.addr() + 8, id);
        } else if chunk == CHUNK_PRKF {
            return true;
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            return true;
        }
    }
}

// Translated from 005e9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAbilityPerkEntry::ApplyPerkEntry` (Xbox PDB): with an ability, adds
/// it to `actor` (vtable slot 0x3E0) and runs `Actor::UpdateMagic`
/// (`008c3c40`, with 0, 0). The second stack word is never read.
pub fn bgs_ability_perk_entry_apply_perk_entry(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    _unused_2: u32,
) {
    let ability = e.mem.u32(this.addr() + 8);
    if ability == 0 {
        return;
    }
    e.vcall(actor.addr(), 0x3e0, &args![ability]);
    e.call(0x008c_3c40, &args![actor, 0u32, 0u32]);
}

// Translated from 005e9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry` constructor (its vtable `01046d0c`, by RTTI):
/// the base, the vtable, the function data +0xC and the conditions +0x10
/// null, the bytes +0xA and +0x9 = 0. Returns `this`.
pub fn fn_005e9f60(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e9530(e, this);
    let base = this.addr();
    e.mem.set_u32(base, ENTRY_POINT_PERK_ENTRY_VTABLE);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u8(base + 0xa, 0);
    e.mem.set_u8(base + 0x9, 0);
    this
}

// Translated from 005e9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry::GetFunction` (Xbox PDB): the byte at +0x9.
pub fn bgs_entry_point_perk_entry_get_function(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 9)
}

// Translated from 005ea000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ea000 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointPerkEntry` destructor body: the vtable, its data cleared
/// ([`bgs_entry_point_perk_entry_clear_data`]), then the base destructor
/// `005e9590`. (Ghidra's library matcher names this body after unrelated
/// CRT destructors.)
pub fn fn_005ea000(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ENTRY_POINT_PERK_ENTRY_VTABLE);
    bgs_entry_point_perk_entry_clear_data(e, this);
    e.call(0x005e_9590, &args![this]);
}

// Translated from 005ea060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ea060 - the exception-unwinding frame (FS:[0]) and the /GS security-cookie check (00ec408c) are not translated.
/// `BGSEntryPointPerkEntry::CheckConditionFilters` (Xbox PDB): with exactly
/// as many filter forms (`count`, the array at `filters`) as conditions
/// (byte +0xA; else a warning and false), runs condition `i` (+0x10, 8
/// bytes each, `TESCondition::IsTrue_ov2` `00680c60`) on filter `i`: a
/// base form (vtable slot 0xE4 true) is put into a temporary reference on
/// the stack (`TESObjectREFR` `0055a2f0`, made temporary, then
/// `SetObjectReference`), a reference (slot 0xF0 true) is used as it is.
/// The first false condition gives false; true when all pass. The
/// temporary is destroyed (`0055a430`) after each filter.
///
/// A filter that is neither leaves the subject null, and the warning that
/// follows reads the vtable of that null pointer (slot 0x130) as the exe
/// does: the game faults there, and so does the translation.
pub fn bgs_entry_point_perk_entry_check_condition_filters(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    filters: Ptr,
) -> bool {
    let conditions = u32::from(e.mem.u8(this.addr() + 0xa));
    if count != conditions {
        e.call(
            PRINT_WARNING,
            &args![FILTER_COUNT_WARNING, count, conditions],
        );
        return false;
    }
    let mut index = 0u32;
    while index < count {
        let filter = e.mem.u32(filters.addr().wrapping_add(index * 4));
        // The reference on the stack (0x6C bytes from EBP-0x84) and the
        // condition's out byte.
        let failed = e.with_stack(0x70, |e, locals| {
            let reference = locals.addr();
            let out_byte = locals.addr() + 0x6c;
            let mut subject = 0u32;
            e.call(0x0055_a2f0, &args![reference]);
            e.call(0x0048_4490, &args![reference]);
            if e.vcall(filter, 0xe4, &args![]).bool() {
                e.call(0x0057_5690, &args![reference, filter]);
                subject = reference;
            } else if e.vcall(filter, 0xf0, &args![]).bool() {
                subject = filter;
            }
            if subject != 0 {
                let condition = e.mem.u32(this.addr() + 0x10).wrapping_add(index * 8);
                let passed = e
                    .call(
                        0x0068_0c60,
                        &args![condition, subject, 0u32, out_byte, 0u32],
                    )
                    .bool();
                if !passed {
                    e.call(0x0055_a430, &args![reference]);
                    return true;
                }
            } else {
                // `subject` is null here: the exe reads its vtable.
                let editor_id = e.vcall(subject, 0x130, &args![]).u32();
                let form_id = e.call(FORM_ID, &args![subject]).u32();
                e.call(
                    PRINT_WARNING,
                    &args![BAD_FILTER_WARNING, form_id, editor_id],
                );
            }
            e.call(0x0055_a430, &args![reference]);
            false
        });
        if failed {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 005ea230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry::ClearData` (Xbox PDB): deletes the condition
/// array (+0x10, [`fn_005ea2c0`] with 3) and the function data (+0xC,
/// vtable slot 0 with 1), then clears the bytes +0x8..+0xA and both
/// pointers.
pub fn bgs_entry_point_perk_entry_clear_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let conditions = e.mem.u32(base + 0x10);
    if conditions != 0 {
        fn_005ea2c0(e, Ptr::new(conditions), 3);
    }
    let data = e.mem.u32(base + 0xc);
    if data != 0 {
        e.vcall(data, 0x0, &args![1u32]);
    }
    e.mem.set_u8(base + 8, 0);
    e.mem.set_u8(base + 9, 0);
    e.mem.set_u8(base + 0xa, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0xc, 0);
}

// Translated from 005ea2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vector deleting destructor of the 8-byte condition (`TESCondition`,
/// destructor `006808b0`): with bit 1 of `flags`, destroys the array whose
/// count is the word before it (`` `eh vector destructor iterator' ``
/// `00ec5fce`) and releases the block from that count word (bit 0),
/// returning it; otherwise destroys the one object and releases it
/// (bit 0), returning `this`. (Ghidra's library matcher gives this body
/// CRT names.)
pub fn fn_005ea2c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(base.wrapping_sub(4));
        e.call(0x00ec_5fce, &args![base, 8u32, count, 0x0068_08b0u32]);
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![base.wrapping_sub(4)]);
        }
        return base.wrapping_sub(4);
    }
    e.call(0x0068_08b0, &args![base]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    base
}

// Translated from 005ea330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry::InitItem` (Xbox PDB): runs
/// `TESCondition::InitItem` (`00680a20`) on each condition (+0x10, byte
/// count +0xA) and the function data's `InitItem` (slot 0x18), with
/// `owner` (one stack word).
pub fn bgs_entry_point_perk_entry_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut index = 0u32;
    while index < u32::from(e.mem.u8(this.addr() + 0xa)) {
        let condition = e.mem.u32(this.addr() + 0x10).wrapping_add(index * 8);
        e.call(0x0068_0a20, &args![condition, owner]);
        index += 1;
    }
    let data = e.mem.u32(this.addr() + 0xc);
    if data != 0 {
        e.vcall(data, 0x18, &args![owner]);
    }
}

// Translated from 005ea790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ea790 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointPerkEntry::Load` (Xbox PDB): reads the chunks of `file`
/// (one stack word) until none is left or `PRKF` comes (true):
/// - `DATA` (3 bytes, else false) into +0x8..+0xA, then a new condition
///   array of byte +0xA entries (8 bytes each, a count word before them,
///   built by `` `eh vector constructor iterator' `` `00ec782f` with
///   `00680890`/`006808b0`) into +0x10;
/// - `PRKC`: the condition index byte; `CTDA`: that condition loads
///   (`TESCondition::Load`), false without an array;
/// - `EPFT` (1 byte, else false): the function data of that type
///   (`005e6280`) into +0xC, false when none is made;
/// - `EPFD`, `EPF2`, `SCDA`, `SCRO`, `SCHR`, `SCRV`, `SCTX`: the function
///   data's `Load` (slot 0x14), or a warning and false without data.
pub fn bgs_entry_point_perk_entry_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, locals| {
        // `locals + 0`: the condition index byte; `+1`: the function type.
        let condition_index = locals.addr();
        let function_type = locals.addr() + 1;
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                return true;
            }
            match chunk {
                CHUNK_PRKF => return true,
                CHUNK_DATA => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    if size != 3 {
                        return false;
                    }
                    e.call(GET_CHUNK_DATA, &args![file, base + 8, size]);
                    let count = u32::from(e.mem.u8(base + 0xa));
                    // `MUL`/`SETO`/`NEG`/`OR`, then `ADD 4`/`SETC`/`NEG`/`OR`:
                    // all ones on overflow.
                    let bytes = count
                        .checked_mul(8)
                        .unwrap_or(u32::MAX)
                        .checked_add(4)
                        .unwrap_or(u32::MAX);
                    let block = e.call(OPERATOR_NEW, &args![bytes]).u32();
                    let array = if block == 0 {
                        0
                    } else {
                        e.mem.set_u32(block, count);
                        e.call(
                            0x00ec_782f,
                            &args![block + 4, 8u32, count, 0x0068_0890u32, 0x0068_08b0u32],
                        );
                        block + 4
                    };
                    e.mem.set_u32(base + 0x10, array);
                }
                CHUNK_PRKC => {
                    e.call(GET_CHUNK_DATA, &args![file, condition_index, 1u32]);
                }
                CHUNK_CTDA => {
                    let conditions = e.mem.u32(base + 0x10);
                    if conditions == 0 {
                        return false;
                    }
                    let index = u32::from(e.mem.u8(condition_index));
                    e.call(
                        0x0068_0960,
                        &args![conditions.wrapping_add(index * 8), file],
                    );
                }
                CHUNK_EPFT => {
                    e.mem.set_u8(function_type, 0);
                    if e.call(CHUNK_SIZE, &args![file]).u32() != 1 {
                        return false;
                    }
                    e.call(GET_CHUNK_DATA, &args![file, function_type, 0u32]);
                    let kind = u32::from(e.mem.u8(function_type));
                    let data = e.call(0x005e_6280, &args![kind]).u32();
                    e.mem.set_u32(base + 0xc, data);
                    if data == 0 {
                        return false;
                    }
                }
                CHUNK_EPFD | CHUNK_EPF2 | CHUNK_SCDA | CHUNK_SCRO | CHUNK_SCHR | CHUNK_SCRV
                | CHUNK_SCTX => {
                    let data = e.mem.u32(base + 0xc);
                    if data == 0 {
                        e.call(PRINT_WARNING, &args![NO_FUNCTION_DATA_WARNING]);
                        return false;
                    }
                    e.vcall(data, 0x14, &args![file]);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                return true;
            }
        }
    })
}

// Translated from 005eaa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointPerkEntry::ApplyPerkEntry` (Xbox PDB): hands the entry and
/// the flag (low byte of the second stack word) to `actor`'s vtable slot
/// 0x4A4.
pub fn bgs_entry_point_perk_entry_apply_perk_entry(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    flag: u8,
) {
    e.vcall(actor.addr(), 0x4a4, &args![this, u32::from(flag)]);
}

// Translated from 005eabb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005eabb0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSPerk` destructor body: the four vtables back (`01046ec4`, +0x18
/// `01046eac`, +0x24 `01046e94`, +0x2C `01046e74`), the entries freed
/// ([`fn_005eaca0`]), then the member destructors `0046ffb0` (+0x48),
/// `006808b0` (+0x40), `00402c40` (+0x2C), `00591300` (+0x18) and
/// `TESForm::~TESForm` (`00483630`, engine map).
pub fn fn_005eabb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_6ec4);
    e.mem.set_u32(base + 0x18, 0x0104_6eac);
    e.mem.set_u32(base + 0x24, 0x0104_6e94);
    e.mem.set_u32(base + 0x2c, 0x0104_6e74);
    fn_005eaca0(e, this);
    e.call(0x0046_ffb0, &args![base + 0x48]);
    e.call(0x0068_08b0, &args![base + 0x40]);
    e.call(0x0040_2c40, &args![base + 0x2c]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005eac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The perk's data defaults: bytes +0x38 = 0, +0x39 = 0, +0x3A = 1,
/// +0x3B = 1, +0x3C = 0.
pub fn fn_005eac70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x38, 0);
    e.mem.set_u8(base + 0x39, 0);
    e.mem.set_u8(base + 0x3a, 1);
    e.mem.set_u8(base + 0x3b, 1);
    e.mem.set_u8(base + 0x3c, 0);
}

// Translated from 005eaca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every perk entry of the list at +0x48 (vtable slot 0xC with 1),
/// empties the list (`00470470`) and clears the conditions at +0x40
/// (`00680fe0`).
pub fn fn_005eaca0(e: &mut Engine, this: Ptr) {
    let mut node = this.addr() + 0x48;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            e.vcall(entry, 0xc, &args![1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![this.addr() + 0x48]);
    e.call(0x0068_0fe0, &args![this.addr() + 0x40]);
}

// Translated from 005ead20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The perk's `InitItem` (nothing when `004013e0` is true): the conditions
/// (+0x40, `TESCondition::InitItem` with the perk), each entry's `InitItem`
/// (slot 0x18 with the perk), then `00484ab0(1)`.
pub fn fn_005ead20(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0068_0a20, &args![this.addr() + 0x40, this]);
    let mut node = this.addr() + 0x48;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            e.vcall(entry, 0x18, &args![this]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(TES_FORM_MARK_00484AB0, &args![this, 1u32]);
}

// Translated from 005eada0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005eada0 - the /GS security-cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) EDID buffer is a with_stack block of the chunk size rounded up to 16 bytes.
/// `BGSPerk::Load` (Xbox PDB): false unless the record type is 0x56; then
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk of `file` (one stack
/// word) until none is left: `OBND` (slot 0xE0), `EDID` (slot 0x134, at most
/// 0x200 bytes), `FULL` (`00487050` on +0x18), `DESC`
/// (`TESDescription::LoadDescriptionData` on +0x24), `ICON`
/// (`TESTexture::LoadTextureChunk` on +0x2C; each null when `this` is),
/// `CTDA` (`TESCondition::Load` on +0x40); `DATA` of 5 bytes into +0x38,
/// of 3 or 4 bytes over the defaults (0, 0, 1, 1) into +0x38.., any other
/// size false; `PRKE` (3 bytes, else false): a new entry of the header's
/// type ([`fn_005e96e0`], false when none), its rank and priority
/// ([`fn_005e96b0`]), its own `Load` (slot 0x2C, false when it fails), then
/// added to the perk ([`fn_005eb410`]). Returns true.
pub fn bgs_perk_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if u32::from(e.call(GET_TES_FORM, &args![file]).u8()) != 0x56 {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(TES_FORM_MARK_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            return true;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_FULL => {
                let name = if base == 0 { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![name, file]);
            }
            CHUNK_DESC => {
                let description = if base == 0 { 0 } else { base + 0x24 };
                e.call(0x0048_2d50, &args![description, file]);
            }
            CHUNK_ICON => {
                let texture = if base == 0 { 0 } else { base + 0x2c };
                e.call(0x0048_e3d0, &args![texture, file]);
            }
            CHUNK_CTDA => {
                e.call(0x0068_0960, &args![base + 0x40, file]);
            }
            CHUNK_PRKE => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size != 3 {
                    return false;
                }
                let added = e.with_stack(4, |e, header| {
                    e.call(GET_CHUNK_DATA, &args![file, header, size]);
                    let kind = u32::from(e.mem.u8(header.addr()));
                    let entry = fn_005e96e0(e, kind);
                    if entry.is_null() {
                        return false;
                    }
                    fn_005e96b0(e, entry, header);
                    if !e.vcall(entry.addr(), 0x2c, &args![file]).bool() {
                        return false;
                    }
                    fn_005eb410(e, this, entry);
                    true
                });
                if !added {
                    return false;
                }
            }
            CHUNK_DATA => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                match size {
                    5 => {
                        e.call(GET_CHUNK_DATA, &args![file, base + 0x38, 5u32]);
                    }
                    3 | 4 => {
                        let bytes = e.with_stack(4, |e, local| {
                            e.mem.write(local.addr(), &[0, 0, 1, 1]);
                            e.call(GET_CHUNK_DATA, &args![file, local, size]);
                            e.mem.bytes(local.addr(), size)
                        });
                        e.mem.write(base + 0x38, &bytes);
                    }
                    _ => return false,
                }
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            return true;
        }
    }
}

// Translated from 005eb410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `entry` (one stack word) to the entry list at +0x48 in its sorted
/// place (`BSSimpleList<ActiveEffect_P>::Insert` `007a7eb0` with the
/// comparison [`fn_005eb460`]) unless it is null or already there
/// (`005f65d0` on the address of the stack word). Returns whether it was
/// added.
pub fn fn_005eb410(e: &mut Engine, this: Ptr, entry: Ptr) -> bool {
    if entry.is_null() {
        return false;
    }
    let list = this.addr() + 0x48;
    let present = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry.addr());
        e.call(0x005f_65d0, &args![list, slot]).bool()
    });
    if present {
        return false;
    }
    e.call(0x007a_7eb0, &args![list, entry, 0x005e_b460u32]);
    true
}

// Translated from 005eb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The perk entry order (cdecl, two words): by rank (`004f1540`, the lower
/// first), then by priority (`004f6de0`, the higher first), then by vtable
/// slot 0x10 (signed, the lower first): -1, 1 or 0. Each test calls both
/// getters again, as the exe does.
pub fn fn_005eb460(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = e.call(ENTRY_RANK, &args![first]).u8();
    let b = e.call(ENTRY_RANK, &args![second]).u8();
    if a < b {
        return -1;
    }
    let a = e.call(ENTRY_RANK, &args![first]).u8();
    let b = e.call(ENTRY_RANK, &args![second]).u8();
    if a > b {
        return 1;
    }
    let a = e.call(0x004f_6de0, &args![first]).u8();
    let b = e.call(0x004f_6de0, &args![second]).u8();
    if a > b {
        return -1;
    }
    let a = e.call(0x004f_6de0, &args![first]).u8();
    let b = e.call(0x004f_6de0, &args![second]).u8();
    if a < b {
        return 1;
    }
    let a = e.vcall(first.addr(), 0x10, &args![]).i32();
    let b = e.vcall(second.addr(), 0x10, &args![]).i32();
    if a < b {
        return -1;
    }
    let a = e.vcall(first.addr(), 0x10, &args![]).i32();
    let b = e.vcall(second.addr(), 0x10, &args![]).i32();
    if a > b {
        return 1;
    }
    0
}

// Translated from 005eb5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts from the byte at +0x3B and, while it is set, walks the condition
/// list at +0x40 up to its first empty node: each condition item whose
/// function (`fn_005eb680`) is 0x46 replaces it with `00681600`(player
/// `[011dea3c]`, 0, out byte). Returns the final value.
pub fn fn_005eb5e0(e: &mut Engine, this: Ptr) -> u8 {
    let mut result = e.mem.u8(this.addr() + 0x3b);
    if result == 0 {
        return result;
    }
    let mut node = e.call(NODE_ITEM, &args![this.addr() + 0x40]).u32();
    e.with_stack(4, |e, out_byte| {
        while result != 0 && node != 0 {
            if e.call(NODE_IS_EMPTY, &args![node]).bool() {
                break;
            }
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 && fn_005eb680(e, Ptr::new(item)) == 0x46 {
                let player = e.mem.u32(PLAYER);
                result = e
                    .call(0x0068_1600, &args![item, player, 0u32, out_byte])
                    .u8();
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    result
}

// Translated from 005eb6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies or removes the perk's entries for `actor` (four stack words: the
/// actor, two ranks and a flag; the ranks and flag are bytes). Without the
/// flag, an entry whose rank + 1 (`004f1540`) is `rank_apply` goes to its
/// slot 0x34, else one whose rank + 1 is `rank_remove` to slot 0x30 (both
/// with the actor and the flag). With the flag, when `005a4910` on the
/// player is positive, every entry goes to slot 0x30 once for each item of
/// the player's list (`005eb7e0`): with the player when its slot 0x10
/// reports 2, else with that item.
pub fn fn_005eb6a0(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    rank_apply: u8,
    rank_remove: u8,
    flag: u8,
) {
    let mut node = this.addr() + 0x48;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            if flag == 0 {
                let rank = u32::from(e.call(ENTRY_RANK, &args![entry]).u8());
                if rank + 1 == u32::from(rank_apply) {
                    e.vcall(entry, 0x34, &args![actor, u32::from(flag)]);
                } else {
                    let rank = u32::from(e.call(ENTRY_RANK, &args![entry]).u8());
                    if rank + 1 == u32::from(rank_remove) {
                        e.vcall(entry, 0x30, &args![actor, u32::from(flag)]);
                    }
                }
            } else {
                let player = e.mem.u32(PLAYER);
                if e.call(0x005a_4910, &args![player]).i32() > 0 {
                    let player = e.mem.u32(PLAYER);
                    let mut item_node = e.call(0x005e_b7e0, &args![player]).u32();
                    while item_node != 0 {
                        let item_slot = e.call(NODE_ITEM, &args![item_node]).u32();
                        let item = e.mem.u32(item_slot);
                        if e.vcall(entry, 0x10, &args![]).u32() == 2 {
                            let player = e.mem.u32(PLAYER);
                            e.vcall(entry, 0x30, &args![player, u32::from(flag)]);
                        } else {
                            e.vcall(entry, 0x30, &args![item, u32::from(flag)]);
                        }
                        item_node = e.call(NODE_NEXT, &args![item_node]).u32();
                    }
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 005eb900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first perk entry (list at +0x48) whose rank + 1 (`004f1540`) is
/// `rank` (one stack word, low byte); 0 when none comes before an empty
/// node or the end.
pub fn fn_005eb900(e: &mut Engine, this: Ptr, rank: u8) -> u32 {
    let mut found = 0u32;
    let mut node = this.addr() + 0x48;
    while found == 0 && node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let entry_rank = u32::from(e.call(ENTRY_RANK, &args![entry]).u8());
        if entry_rank + 1 == u32::from(rank) {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            found = e.mem.u32(slot);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    found
}

// Translated from 005eba00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPerk::IsPerkAttainable` (Xbox PDB): true unless a condition item
/// (`TESCondition::GetConditionItem` on +0x40, index 0, 1, ...) whose
/// function is 0x46 gives a value other than 1.0 for `actor` (one stack
/// word) through `Script::GetIsSexConditionFunction` (`0059f3c0`, the
/// item's first parameter, 0 and a `double` result on the stack); the last
/// such item decides (`FLD1`/`FCOMP`, `TEST AH,0x44`/`JP`: equal only,
/// false on NaN).
pub fn bgs_perk_is_perk_attainable(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    let mut result = true;
    let mut index = 0u32;
    let mut item = e
        .call(GET_CONDITION_ITEM, &args![this.addr() + 0x40, index])
        .u32();
    index += 1;
    while item != 0 {
        let data = e.call(CONDITION_DATA, &args![item]).u32();
        if u32::from(e.mem.u16(data)) == 0x46 {
            let value = e.with_stack(8, |e, out| {
                e.mem.set_f64(out.addr(), 0.0);
                let data = e.call(CONDITION_DATA, &args![item]).u32();
                let parameter = e.mem.u32(data + 4);
                e.call(0x0059_f3c0, &args![actor, parameter, 0u32, out]);
                e.mem.f64(out.addr())
            });
            result = value == 1.0;
        }
        item = e
            .call(GET_CONDITION_ITEM, &args![this.addr() + 0x40, index])
            .u32();
        index += 1;
    }
    result
}

/// `_ftol2` of an `ST0` value returned by `00681490` (a condition item's
/// comparison value).
fn condition_value_as_int(e: &mut Engine, item: u32) -> i32 {
    let value = e.call(0x0068_1490, &args![item]).f64();
    e.call(FTOL, &args![value]).i32()
}

// Translated from 005ebac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ebac0 - the /GS security-cookie check (00ec408c) is not translated.
/// `BGSPerk::BuildDescriptionString` (Xbox PDB): writes the perk's
/// description into `buffer` of `size` bytes (two stack words):
/// - the heading: `"%s: %s %d"` with the strings of the settings
///   `011d4678` and `011d3b2c` and the rank count (byte +0x39) when it is
///   above 1, else `"%s: "` with the first one;
/// - the requirements, walking the condition items (+0x40) at index 0,
///   then index `i + 1` while `i` steps by 2 (the exe's loop increments its
///   index twice per item): an actor-value test (function 0x1EF) other than
///   value 0x17 and with an abbreviation (`0066eb00`) gives
///   `abbreviation, comparison, value + adjustment` (`005ec060` 2: -1, 4:
///   `" < "`, 5: `" < "` and +1; skipped when that sum is 0), followed by
///   `" %s "` with the string of setting `011d324c` when `005ec040` is set
///   and an item follows, or `", "` when an item follows; a location test
///   (0x1C1) the location's name (`MapMarkerData::GetLocationName`), with
///   `" %d"` of its second parameter when that is not 0; the first one
///   after a long heading is preceded by `", "`;
/// - `"--"` when nothing was listed, `"\n\n"`, `"%s: %d\n\n"` with setting
///   `011d40e4` and byte +0x3A, and the description text (slot 0x10 of the
///   `TESDescription` at +0x24 with 0 and `'DESC'`).
///
/// The location branch appends the 0x80-byte work buffer even when the
/// condition has no location, so its previous contents are appended again,
/// as in the exe.
pub fn bgs_perk_build_description_string(e: &mut Engine, this: Ptr, buffer: Ptr, size: u32) {
    let base = this.addr();
    e.with_stack(0x84, |e, work| {
        let work = work.addr();
        let mut short_heading = false;
        let mut item = e.call(GET_CONDITION_ITEM, &args![base + 0x40, 0u32]).u32();
        let ranks = u32::from(e.mem.u8(base + 0x39));
        if ranks > 1 {
            let ranks_label = e.call(SETTING_STRING, &args![0x011d_3b2cu32]).u32();
            let perk_label = e.call(SETTING_STRING, &args![0x011d_4678u32]).u32();
            e.call(
                STRING_FORMAT,
                &args![buffer, size, 0x0104_7040u32, perk_label, ranks_label, ranks],
            );
        } else {
            let perk_label = e.call(SETTING_STRING, &args![0x011d_4678u32]).u32();
            e.call(
                STRING_FORMAT,
                &args![buffer, size, 0x0104_7038u32, perk_label],
            );
            short_heading = true;
        }
        let mut listed = false;
        let mut index = 0u32;
        while item != 0 {
            let first_separator = |e: &mut Engine, index: u32| {
                if index == 0 && !short_heading {
                    e.call(STRING_APPEND, &args![buffer, size, 0x0101_2630u32]);
                }
            };
            if fn_005eb680(e, Ptr::new(item)) == 0x1ef {
                let data = e.call(CONDITION_DATA, &args![item]).u32();
                let actor_value = e.mem.u32(data + 4);
                let mut comparison = 0x0102_0770u32;
                let mut adjustment = 0i32;
                match e.call(0x005e_c060, &args![item]).u32() {
                    2 => adjustment = -1,
                    4 => comparison = 0x0104_7034,
                    5 => {
                        comparison = 0x0104_7034;
                        adjustment = 1;
                    }
                    _ => {}
                }
                if actor_value != 0x17 && e.call(0x0066_eb00, &args![actor_value]).u32() != 0 {
                    let value = condition_value_as_int(e, item);
                    if value.wrapping_add(adjustment) == 0 {
                        index += 1;
                        item = e.call(GET_CONDITION_ITEM, &args![base + 0x40, index]).u32();
                        index += 1;
                        continue;
                    }
                    listed = true;
                    first_separator(e, index);
                    let joined = e.call(0x005e_c040, &args![item]).bool()
                        && e.call(GET_CONDITION_ITEM, &args![base + 0x40, index + 1])
                            .u32()
                            != 0;
                    if joined {
                        let joiner = e.call(SETTING_STRING, &args![0x011d_324cu32]).u32();
                        let value = condition_value_as_int(e, item).wrapping_add(adjustment);
                        let label = e.call(0x0066_eb00, &args![actor_value]).u32();
                        e.call(
                            STRING_FORMAT,
                            &args![
                                work,
                                0x80u32,
                                0x0104_7028u32,
                                label,
                                comparison,
                                value as u32,
                                joiner
                            ],
                        );
                    } else {
                        let follows = e
                            .call(GET_CONDITION_ITEM, &args![base + 0x40, index + 1])
                            .u32()
                            != 0;
                        let format = if follows { 0x0104_701cu32 } else { 0x0104_7014 };
                        let value = condition_value_as_int(e, item).wrapping_add(adjustment);
                        let label = e.call(0x0066_eb00, &args![actor_value]).u32();
                        e.call(
                            STRING_FORMAT,
                            &args![work, 0x80u32, format, label, comparison, value as u32],
                        );
                    }
                    e.call(STRING_APPEND, &args![buffer, size, work]);
                }
            } else if fn_005eb680(e, Ptr::new(item)) == 0x1c1 {
                listed = true;
                first_separator(e, index);
                let data = e.call(CONDITION_DATA, &args![item]).u32();
                let location = e.mem.u32(data + 4);
                let number = e.mem.u32(data + 8);
                if location != 0 && number > 0 {
                    let name = e.call(0x0040_8da0, &args![location + 0x18]).u32();
                    e.call(
                        STRING_FORMAT,
                        &args![work, 0x80u32, 0x0104_700cu32, name, number],
                    );
                } else if location != 0 {
                    let name = e.call(0x0040_8da0, &args![location + 0x18]).u32();
                    e.call(STRING_FORMAT, &args![work, 0x80u32, 0x0101_9f08u32, name]);
                }
                e.call(STRING_APPEND, &args![buffer, size, work]);
            } else {
                listed = true;
                first_separator(e, index);
            }
            index += 1;
            item = e.call(GET_CONDITION_ITEM, &args![base + 0x40, index]).u32();
            index += 1;
        }
        if !listed {
            e.call(STRING_APPEND, &args![buffer, size, 0x0104_7008u32]);
        }
        e.call(STRING_APPEND, &args![buffer, size, 0x0101_2648u32]);
        let level_label = e.call(SETTING_STRING, &args![0x011d_40e4u32]).u32();
        let level = u32::from(e.mem.u8(base + 0x3a));
        e.call(
            STRING_FORMAT,
            &args![work, 0x80u32, 0x0104_6ffcu32, level_label, level],
        );
        e.call(STRING_APPEND, &args![buffer, size, work]);
        let text = e
            .vcall(base + 0x24, 0x10, &args![0u32, 0x4353_4544u32])
            .u32();
        e.call(STRING_APPEND, &args![buffer, size, text]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_aab0, fn_005eaab0(Ptr) -> Ptr),
        entry!(0x005e_b680, fn_005eb680(Ptr) -> u32),
        entry!(0x005e_9fd0, bgs_entry_point_perk_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005e_ab80, fn_005eab80(Ptr, u32) -> Ptr),
        entry!(0x005e_9530, fn_005e9530(Ptr) -> Ptr),
        entry!(0x005e_96b0, fn_005e96b0(Ptr, Ptr)),
        entry!(0x005e_96e0, fn_005e96e0(u32) -> Ptr),
        entry!(0x005e_9800, fn_005e9800(Ptr) -> Ptr),
        entry!(0x005e_9860, fn_005e9860(Ptr)),
        entry!(0x005e_9880, bgs_quest_perk_entry_init_item(Ptr, Ptr)),
        entry!(0x005e_9b00, bgs_quest_perk_entry_load(Ptr, Ptr) -> bool),
        entry!(0x005e_9bd0, fn_005e9bd0(Ptr) -> Ptr),
        entry!(0x005e_9c00, bgs_ability_perk_entry_init_item(Ptr, Ptr)),
        entry!(0x005e_9e20, bgs_ability_perk_entry_load(Ptr, Ptr) -> bool),
        entry!(
            0x005e_9ee0,
            bgs_ability_perk_entry_apply_perk_entry(Ptr, Ptr, u32)
        ),
        entry!(0x005e_9f60, fn_005e9f60(Ptr) -> Ptr),
        entry!(0x005e_9fb0, bgs_entry_point_perk_entry_get_function(Ptr) -> u8),
        entry!(0x005e_a000, fn_005ea000(Ptr)),
        entry!(
            0x005e_a060,
            bgs_entry_point_perk_entry_check_condition_filters(Ptr, u32, Ptr) -> bool
        ),
        entry!(0x005e_a230, bgs_entry_point_perk_entry_clear_data(Ptr)),
        entry!(0x005e_a2c0, fn_005ea2c0(Ptr, u32) -> u32),
        entry!(0x005e_a330, bgs_entry_point_perk_entry_init_item(Ptr, Ptr)),
        entry!(0x005e_a790, bgs_entry_point_perk_entry_load(Ptr, Ptr) -> bool),
        entry!(
            0x005e_aa50,
            bgs_entry_point_perk_entry_apply_perk_entry(Ptr, Ptr, u8)
        ),
        entry!(0x005e_abb0, fn_005eabb0(Ptr)),
        entry!(0x005e_ac70, fn_005eac70(Ptr)),
        entry!(0x005e_aca0, fn_005eaca0(Ptr)),
        entry!(0x005e_ad20, fn_005ead20(Ptr)),
        entry!(0x005e_ada0, bgs_perk_load(Ptr, Ptr) -> bool),
        entry!(0x005e_b410, fn_005eb410(Ptr, Ptr) -> bool),
        entry!(0x005e_b460, fn_005eb460(Ptr, Ptr) -> i32),
        entry!(0x005e_b5e0, fn_005eb5e0(Ptr) -> u8),
        entry!(0x005e_b6a0, fn_005eb6a0(Ptr, Ptr, u8, u8, u8)),
        entry!(0x005e_b900, fn_005eb900(Ptr, u8) -> u32),
        entry!(0x005e_ba00, bgs_perk_is_perk_attainable(Ptr, Ptr) -> bool),
        entry!(
            0x005e_bac0,
            bgs_perk_build_description_string(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005eb680_answers_the_word_at_8() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 0xffff_8001);
        assert_eq!(e.call(0x005e_b680, &[object]).u32(), 0x8001);
    }

    #[test]
    fn fn_005eaab0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_aab0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0040_2c20, &[A::T(0x2c)]),
                (0x0068_0890, &[A::T(0x40)]),
                (0x0096_a2d0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x56)]),
                (0x005e_ac70, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_6ec4),
                (0x18, 4, 0x0104_6eac),
                (0x24, 4, 0x0104_6e94),
                (0x2c, 4, 0x0104_6e74),
            ],
        });
    }
    #[test]
    fn at_005e9fd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_a000, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_9fd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_9fd0)
                .collect();
            let mut expected = vec![(0x005e_a000, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005eab80_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_abb0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_ab80, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_ab80)
                .collect();
            let mut expected = vec![(0x005e_abb0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A vtable at `table` whose slots `(offset, target)` are set; the
    /// targets get no-op doubles (a test registers its own afterwards).
    fn vtable(e: &mut Engine, table: u32, slots: &[(u32, u32)]) {
        e.put_vtable(table, &[0; 0x140]);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
            e.register(*target, |_, _| Ret::default());
        }
    }

    fn object(e: &mut Engine, table: u32, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, table);
        p
    }

    /// A list of `items` starting at the inline head `head` ({item, next}).
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
    }

    #[test]
    fn the_entry_constructors_set_their_vtables_and_fields() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x18);
        e.mem.write(p, &[0xcc; 0x18]);
        assert_eq!(e.call(0x005e_9530, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.bytes(p + 4, 3), vec![0, 0, 0xcc]);

        e.mem.write(p, &[0xcc; 0x18]);
        e.call(0x005e_9800, &args![p]);
        assert_eq!(e.mem.u32(p), QUEST_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(p + 8), 0);
        assert_eq!(e.mem.bytes(p + 0xc, 2), vec![0, 0xcc]);

        e.mem.write(p, &[0xcc; 0x18]);
        e.call(0x005e_9bd0, &args![p]);
        assert_eq!(e.mem.u32(p), ABILITY_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(p + 8), 0);
        assert_eq!(e.mem.u8(p + 0xc), 0xcc);

        e.mem.write(p, &[0xcc; 0x18]);
        e.call(0x005e_9f60, &args![p]);
        assert_eq!(e.mem.u32(p), ENTRY_POINT_PERK_ENTRY_VTABLE);
        assert_eq!(
            e.mem.bytes(p + 4, 8),
            vec![0, 0, 0xcc, 0xcc, 0xcc, 0, 0, 0xcc]
        );
        assert_eq!(e.mem.u32(p + 0xc), 0);
        assert_eq!(e.mem.u32(p + 0x10), 0);
        assert_eq!(e.call(0x005e_9fb0, &args![p]).u32() & 0xff, 0);
        e.mem.set_u8(p + 9, 0x2a);
        assert_eq!(e.call(0x005e_9fb0, &args![p]).u32() & 0xff, 0x2a);
    }

    #[test]
    fn fn_005e96e0_makes_each_entry_type() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let quest = e.call(0x005e_96e0, &args![0u32]).u32();
        let ability = e.call(0x005e_96e0, &args![1u32]).u32();
        let entry_point = e.call(0x005e_96e0, &args![2u32]).u32();
        assert_eq!(e.call(0x005e_96e0, &args![3u32]).u32(), 0);
        assert_eq!(e.mem.u32(quest), QUEST_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(ability), ABILITY_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(entry_point), ENTRY_POINT_PERK_ENTRY_VTABLE);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x10, 0xc, 0x14]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x005e_96e0, &args![1u32]).u32(), 0);
    }

    #[test]
    fn fn_005e96b0_copies_the_header_bytes_and_fn_005e9860_destroys_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x005e_9590]);
        let p = e.mem.alloc(8);
        let header = e.mem.alloc(4);
        e.mem.write(header, &[9, 3, 7]);
        e.call(0x005e_96b0, &args![p, header]);
        assert_eq!(e.mem.bytes(p + 4, 2), vec![3, 7]);
        e.call_log = Some(vec![]);
        e.call(0x005e_9860, &args![p]);
        assert_eq!(e.call_log.take().unwrap()[1..], [(0x005e_9590, vec![p])]);
    }

    fn init_engine(cast: u32, named: bool) -> Engine {
        let mut e = Engine::new();
        quiet(&mut e, &[PRINT_WARNING]);
        e.register(TES_FORM_GET_FILE, |_, _| 0x0900_0000u32.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORM_LOOKUP_004839C0, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| cast.into_ret());
        e.register_double(EDITOR_ID_LENGTH, move |_, _| u32::from(named).into_ret());
        e.register(FORM_ID, |_, _| 0x0002_0000u32.into_ret());
        vtable(&mut e, 0x0200_0000, &[(0x130, 0x0090_0000)]);
        e.register(0x0090_0000, |_, _| 0x6000u32.into_ret());
        e
    }

    #[test]
    fn quest_and_ability_init_item_resolve_or_warn() {
        for (addr, target, named_warning, id_warning) in [
            (0x005e_9880, TYPE_TES_QUEST, NO_QUEST_NAMED, NO_QUEST_ID),
            (
                0x005e_9c00,
                TYPE_SPELL_ITEM,
                NO_ABILITY_NAMED,
                NO_ABILITY_ID,
            ),
        ] {
            let mut e = init_engine(0x8000, true);
            let owner = object(&mut e, 0x0200_0000, 8);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x55);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, owner]);
            assert_eq!(e.mem.u32(this + 8), 0x8000);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST),
                vec![vec![0x0100_0065, 0, TYPE_TES_FORM, target, 0]]
            );
            for (named, want) in [
                (true, vec![named_warning, 0x0100_0055, 0x6000]),
                (false, vec![id_warning, 0x0100_0055, 0x0002_0000]),
            ] {
                let mut e = init_engine(0, named);
                let owner = object(&mut e, 0x0200_0000, 8);
                let this = e.mem.alloc(0x10);
                e.mem.set_u32(this + 8, 0x55);
                e.call_log = Some(vec![]);
                e.call(addr, &args![this, owner]);
                assert_eq!(
                    calls_to(&e.call_log.take().unwrap(), PRINT_WARNING),
                    vec![want]
                );
            }
            // No id: nothing.
            let mut e = init_engine(0, false);
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 0x7000u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
        }
    }

    /// Chunk reader doubles over a list of (tag, data) chunks.
    fn chunk_engine(record: u32, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        let list = Rc::new(chunks);
        let current = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| record.into_ret());
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            l.get(c.get()).map_or(0, |chunk| chunk.0).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(NEXT_CHUNK, move |_, _| {
            c.set(c.get() + 1);
            u32::from(c.get() < l.len()).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(CHUNK_SIZE, move |_, _| {
            (l[c.get()].1.len() as u32).into_ret()
        });
        let (l, c) = (list, current);
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = &l[c.get()].1;
            let n = if a[2] == 0 {
                data.len()
            } else {
                (a[2] as usize).min(data.len())
            };
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        e.register(NEEDS_BYTE_SWAP, |_, _| Ret::default());
        e
    }

    #[test]
    fn quest_and_ability_load_read_their_data() {
        let mut e = chunk_engine(
            0,
            vec![
                (CHUNK_DATA, vec![1, 2, 3, 4, 5, 6, 7, 8]),
                (CHUNK_PRKF, vec![]),
            ],
        );
        let this = e.mem.alloc(0x10);
        assert!(e.call(0x005e_9b00, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.bytes(this + 8, 8), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let mut e = chunk_engine(0, vec![(CHUNK_DATA, vec![1, 2, 3, 4])]);
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x005e_9b00, &args![this, 0x7000u32]).bool());
        let mut e = chunk_engine(
            0,
            vec![(0x1234, vec![]), (CHUNK_DATA, vec![1, 2, 3, 4, 5, 6, 7, 8])],
        );
        let this = e.mem.alloc(0x10);
        e.register(NEEDS_BYTE_SWAP, |_, _| 1u32.into_ret());
        quiet(&mut e, &[0x0050_3210]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_9b00, &args![this, 0x7000u32]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0050_3210),
            vec![vec![this + 8]]
        );

        let mut e = chunk_engine(0, vec![(CHUNK_DATA, vec![0x44, 0x33, 0x22, 0x11])]);
        let this = e.mem.alloc(0x10);
        assert!(e.call(0x005e_9e20, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u32(this + 8), 0x1122_3344);
        let mut e = chunk_engine(0, vec![(CHUNK_DATA, vec![0x44, 0x33, 0x22, 0x11])]);
        let this = e.mem.alloc(0x10);
        e.register(NEEDS_BYTE_SWAP, |_, _| 1u32.into_ret());
        e.register(SWAP_WORD_IN_PLACE, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v.swap_bytes());
            Ret::default()
        });
        assert!(e.call(0x005e_9e20, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u32(this + 8), 0x4433_2211);
        let mut e = chunk_engine(0, vec![(CHUNK_DATA, vec![1, 2])]);
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x005e_9e20, &args![this, 0x7000u32]).bool());
        let mut e = chunk_engine(0, vec![(CHUNK_PRKF, vec![]), (CHUNK_DATA, vec![1, 2])]);
        let this = e.mem.alloc(0x10);
        assert!(e.call(0x005e_9e20, &args![this, 0x7000u32]).bool());
    }

    #[test]
    fn ability_apply_adds_the_spell_and_updates_magic() {
        let mut e = Engine::new();
        vtable(&mut e, 0x0200_0000, &[(0x3e0, 0x0090_0000)]);
        quiet(&mut e, &[0x008c_3c40]);
        let actor = object(&mut e, 0x0200_0000, 8);
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x005e_9ee0, &args![this, actor, 5u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x005e_9ee0, &args![this, actor, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0090_0000, vec![actor, 0x4444]),
                (0x008c_3c40, vec![actor, 0, 0])
            ]
        );
    }

    #[test]
    fn entry_point_apply_hands_itself_to_the_actor() {
        let mut e = Engine::new();
        vtable(&mut e, 0x0200_0000, &[(0x4a4, 0x0090_0000)]);
        let actor = object(&mut e, 0x0200_0000, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_aa50, &args![0x4000u32, actor, 0x101u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1..],
            [(0x0090_0000, vec![actor, 0x4000, 1])]
        );
    }

    #[test]
    fn clear_data_and_the_destructor_free_both_parts() {
        let mut e = Engine::new();
        vtable(&mut e, 0x0200_0000, &[(0, 0x0090_0000)]);
        quiet(
            &mut e,
            &[0x0068_08b0, OPERATOR_DELETE, 0x00ec_5fce, 0x005e_9590],
        );
        let data = object(&mut e, 0x0200_0000, 8);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, 2);
        let this = e.mem.alloc(0x14);
        e.mem.write(this + 8, &[1, 2, 3, 4]);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_a000, &args![this]);
        assert_eq!(e.mem.u32(this), ENTRY_POINT_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.bytes(this + 8, 4), vec![0, 0, 0, 4]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00ec_5fce, vec![block + 4, 8, 2, 0x0068_08b0]),
                (OPERATOR_DELETE, vec![block]),
                (0x0090_0000, vec![data, 1]),
                (0x005e_9590, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_005ea2c0_destroys_one_or_an_array() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0068_08b0, OPERATOR_DELETE, 0x00ec_5fce]);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, 2);
        for (flags, back, deletes) in [(3u32, block, true), (2, block, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005e_a2c0, &args![block + 4, flags]).u32(), back);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), usize::from(deletes));
            assert_eq!(calls_to(&log, 0x0068_08b0).len(), 0);
        }
        for (flags, deletes) in [(1u32, true), (0, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005e_a2c0, &args![block, flags]).u32(), block);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0068_08b0), vec![vec![block]]);
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), usize::from(deletes));
        }
    }

    #[test]
    fn entry_point_init_item_runs_each_condition_and_the_data() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0068_0a20]);
        vtable(&mut e, 0x0200_0000, &[(0x18, 0x0090_0000)]);
        let data = object(&mut e, 0x0200_0000, 8);
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xa, 2);
        e.mem.set_u32(this + 0x10, 0x5000);
        e.mem.set_u32(this + 0xc, data);
        e.call_log = Some(vec![]);
        e.call(0x005e_a330, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0068_0a20, vec![0x5000, 0x7000]),
                (0x0068_0a20, vec![0x5008, 0x7000]),
                (0x0090_0000, vec![data, 0x7000]),
            ]
        );
    }

    #[test]
    fn entry_point_load_builds_the_conditions_and_the_data() {
        let chunks = vec![
            (CHUNK_DATA, vec![4, 1, 2]),
            (CHUNK_PRKC, vec![1]),
            (CHUNK_CTDA, vec![0; 28]),
            (CHUNK_EPFT, vec![3]),
            (CHUNK_EPFD, vec![0; 4]),
            (CHUNK_SCTX, vec![0; 4]),
            (CHUNK_PRKF, vec![]),
            (CHUNK_EPF2, vec![0; 4]),
        ];
        let mut e = chunk_engine(0, chunks);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[0x00ec_782f, 0x0068_0960]);
        vtable(&mut e, 0x0200_0000, &[(0x14, 0x0090_0000)]);
        let data = object(&mut e, 0x0200_0000, 8);
        e.register_double(0x005e_6280, move |_, _| data.into_ret());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_a790, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.bytes(this + 8, 3), vec![4, 1, 2]);
        let array = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(array - 4), 2);
        assert_eq!(e.mem.u32(this + 0xc), data);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![2 * 8 + 4]]);
        assert_eq!(
            calls_to(&log, 0x00ec_782f),
            vec![vec![array, 8, 2, 0x0068_0890, 0x0068_08b0]]
        );
        assert_eq!(calls_to(&log, 0x0068_0960), vec![vec![array + 8, 0x7000]]);
        assert_eq!(calls_to(&log, 0x005e_6280), vec![vec![3]]);
        // EPFD and SCTX reach the data's Load; the EPF2 after PRKF does not.
        assert_eq!(calls_to(&log, 0x0090_0000).len(), 2);
    }

    #[test]
    fn entry_point_load_fails_on_bad_chunks() {
        let cases: Vec<Vec<(u32, Vec<u8>)>> = vec![
            vec![(CHUNK_DATA, vec![1, 2])],
            vec![(CHUNK_CTDA, vec![0])],
            vec![(CHUNK_EPFT, vec![1, 2])],
            vec![(CHUNK_EPFT, vec![9])],
            vec![(CHUNK_SCRO, vec![0; 4])],
        ];
        for chunks in cases {
            let mut e = chunk_engine(0, chunks);
            quiet(&mut e, &[0x005e_6280, PRINT_WARNING]);
            let this = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x005e_a790, &args![this, 0x7000u32]).bool());
        }
        let mut e = chunk_engine(0, vec![(CHUNK_SCHR, vec![0; 4])]);
        quiet(&mut e, &[PRINT_WARNING]);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x005e_a790, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), PRINT_WARNING),
            vec![vec![NO_FUNCTION_DATA_WARNING]]
        );
    }

    fn filter_engine(passes: bool) -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x0055_a2f0,
                0x0048_4490,
                0x0057_5690,
                0x0055_a430,
                PRINT_WARNING,
            ],
        );
        e.register_double(0x0068_0c60, move |_, _| u32::from(passes).into_ret());
        // Base forms: slot 0xE4 true; references: slot 0xF0 true.
        e.put_vtable(0x0200_0000, &[0; 0x50]);
        e.mem.set_u32(0x0200_0000 + 0xe4, 0x0090_0001);
        e.put_vtable(0x0200_1000, &[0; 0x50]);
        e.mem.set_u32(0x0200_1000 + 0xe4, 0x0090_0000);
        e.mem.set_u32(0x0200_1000 + 0xf0, 0x0090_0001);
        e.register(0x0090_0000, |_, _| Ret::default());
        e.register(0x0090_0001, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn check_condition_filters_runs_each_condition_on_its_filter() {
        let mut e = filter_engine(true);
        let base_form = object(&mut e, 0x0200_0000, 8);
        let reference = object(&mut e, 0x0200_1000, 8);
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xa, 2);
        e.mem.set_u32(this + 0x10, 0x5000);
        let filters = e.mem.alloc(8);
        e.mem.set_u32(filters, base_form);
        e.mem.set_u32(filters + 4, reference);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_a060, &args![this, 2u32, filters]).bool());
        let log = e.call_log.take().unwrap();
        let checks = calls_to(&log, 0x0068_0c60);
        let temporary = calls_to(&log, 0x0055_a2f0)[0][0];
        assert_eq!(checks[0][..3], [0x5000, temporary, 0]);
        assert_eq!(checks[1][..3], [0x5008, reference, 0]);
        assert_eq!(
            calls_to(&log, 0x0057_5690),
            vec![vec![temporary, base_form]]
        );
        assert_eq!(calls_to(&log, 0x0055_a430).len(), 2);

        // A failing condition stops at once.
        let mut e = filter_engine(false);
        let reference = object(&mut e, 0x0200_1000, 8);
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xa, 2);
        e.mem.set_u32(this + 0x10, 0x5000);
        let filters = e.mem.alloc(8);
        e.mem.set_u32(filters, reference);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_a060, &args![this, 2u32, filters]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0068_0c60).len(), 1);
        assert_eq!(calls_to(&log, 0x0055_a430).len(), 1);

        // A count mismatch warns.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_a060, &args![this, 3u32, filters]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), PRINT_WARNING),
            vec![vec![FILTER_COUNT_WARNING, 3, 2]]
        );
    }

    #[test]
    fn the_perk_destructor_and_its_entry_helpers() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        quiet(
            &mut e,
            &[
                0x0047_0470,
                0x0068_0fe0,
                0x0046_ffb0,
                0x0068_08b0,
                0x0040_2c40,
                0x0059_1300,
                0x0048_3630,
            ],
        );
        vtable(&mut e, 0x0200_0000, &[(0xc, 0x0090_0000)]);
        let first = object(&mut e, 0x0200_0000, 8);
        let second = object(&mut e, 0x0200_0000, 8);
        let perk = e.mem.alloc(0x50);
        fill_list(&mut e, perk + 0x48, &[first, 0, second]);
        e.call_log = Some(vec![]);
        e.call(0x005e_abb0, &args![perk]);
        assert_eq!(e.mem.u32(perk), 0x0104_6ec4);
        assert_eq!(e.mem.u32(perk + 0x2c), 0x0104_6e74);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0090_0000),
            vec![vec![first, 1], vec![second, 1]]
        );
        let tail: Vec<u32> = log[log.len() - 7..].iter().map(|c| c.0).collect();
        assert_eq!(
            tail,
            vec![
                0x0047_0470,
                0x0068_0fe0,
                0x0046_ffb0,
                0x0068_08b0,
                0x0040_2c40,
                0x0059_1300,
                0x0048_3630
            ]
        );

        let p = e.mem.alloc(0x40);
        e.mem.write(p + 0x38, &[9; 6]);
        e.call(0x005e_ac70, &args![p]);
        assert_eq!(e.mem.bytes(p + 0x38, 6), vec![0, 0, 1, 1, 0, 9]);
    }

    #[test]
    fn fn_005ead20_inits_the_conditions_and_entries() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        quiet(&mut e, &[0x0040_13e0, 0x0068_0a20, TES_FORM_MARK_00484AB0]);
        vtable(&mut e, 0x0200_0000, &[(0x18, 0x0090_0000)]);
        let entry = object(&mut e, 0x0200_0000, 8);
        let perk = e.mem.alloc(0x50);
        fill_list(&mut e, perk + 0x48, &[entry]);
        e.call_log = Some(vec![]);
        e.call(0x005e_ad20, &args![perk]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0068_0a20), vec![vec![perk + 0x40, perk]]);
        assert_eq!(calls_to(&log, 0x0090_0000), vec![vec![entry, perk]]);
        assert_eq!(
            log.last().unwrap(),
            &(TES_FORM_MARK_00484AB0, vec![perk, 1])
        );
        e.register(0x0040_13e0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_ad20, &args![perk]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn perk_load_reads_every_chunk() {
        let chunks = vec![
            (CHUNK_EDID, b"Perk\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_FULL, vec![0; 4]),
            (CHUNK_DESC, vec![0; 4]),
            (CHUNK_ICON, vec![0; 4]),
            (CHUNK_CTDA, vec![0; 28]),
            (CHUNK_DATA, vec![7, 8, 9]),
            (CHUNK_PRKE, vec![1, 2, 3]),
            (0x1234, vec![]),
        ];
        let mut e = chunk_engine(0x56, chunks);
        quiet(
            &mut e,
            &[
                TES_FORM_LOAD_FORM,
                TES_FORM_MARK_00484AB0,
                0x0048_7050,
                0x0048_2d50,
                0x0048_e3d0,
                0x0068_0960,
            ],
        );
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x005f_65d0, |_, _| Ret::default());
        quiet(&mut e, &[0x007a_7eb0]);
        vtable(
            &mut e,
            0x0200_0000,
            &[(0x134, 0x0090_0000), (0xe0, 0x0090_0010)],
        );
        // The ability entry's Load (slot 0x2C of its vtable) succeeds.
        e.put_vtable(ABILITY_PERK_ENTRY_VTABLE, &[0; 0x10]);
        e.mem.set_u32(ABILITY_PERK_ENTRY_VTABLE + 0x2c, 0x0090_0020);
        e.register(0x0090_0020, |_, _| 1u32.into_ret());
        let perk = object(&mut e, 0x0200_0000, 0x50);
        e.mem.set_u8(perk + 0x3b, 0xee);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_ada0, &args![perk, 0x7000u32]).bool());
        assert_eq!(e.mem.bytes(perk + 0x38, 4), vec![7, 8, 9, 0xee]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![perk + 0x18, 0x7000]]);
        assert_eq!(calls_to(&log, 0x0048_2d50), vec![vec![perk + 0x24, 0x7000]]);
        assert_eq!(calls_to(&log, 0x0048_e3d0), vec![vec![perk + 0x2c, 0x7000]]);
        assert_eq!(calls_to(&log, 0x0068_0960), vec![vec![perk + 0x40, 0x7000]]);
        let inserted = calls_to(&log, 0x007a_7eb0);
        let entry = inserted[0][1];
        assert_eq!(inserted[0], vec![perk + 0x48, entry, 0x005e_b460]);
        assert_eq!(e.mem.u32(entry), ABILITY_PERK_ENTRY_VTABLE);
        assert_eq!(e.mem.bytes(entry + 4, 2), vec![2, 3]);
    }

    #[test]
    fn perk_load_data_sizes_and_failures() {
        for (data, want) in [
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0xee]),
        ] {
            let mut e = chunk_engine(0x56, vec![(CHUNK_DATA, data)]);
            quiet(&mut e, &[TES_FORM_LOAD_FORM, TES_FORM_MARK_00484AB0]);
            let perk = e.mem.alloc(0x50);
            e.mem.write(perk + 0x38, &[0xee; 5]);
            assert!(e.call(0x005e_ada0, &args![perk, 0x7000u32]).bool());
            assert_eq!(e.mem.bytes(perk + 0x38, 5), want);
        }
        for chunks in [
            vec![(CHUNK_DATA, vec![1, 2])],
            vec![(CHUNK_PRKE, vec![1, 2])],
            vec![(CHUNK_PRKE, vec![7, 2, 3])],
        ] {
            let mut e = chunk_engine(0x56, chunks);
            quiet(&mut e, &[TES_FORM_LOAD_FORM, TES_FORM_MARK_00484AB0]);
            let perk = e.mem.alloc(0x50);
            assert!(!e.call(0x005e_ada0, &args![perk, 0x7000u32]).bool());
        }
        let mut e = chunk_engine(0x55, vec![]);
        let perk = e.mem.alloc(0x50);
        assert!(!e.call(0x005e_ada0, &args![perk, 0x7000u32]).bool());
    }

    #[test]
    fn fn_005eb410_inserts_new_entries_only() {
        let mut e = Engine::new();
        e.register(0x005f_65d0, |e, a| {
            u32::from(e.mem.u32(a[1]) == 0x7777).into_ret()
        });
        quiet(&mut e, &[0x007a_7eb0]);
        let perk = e.mem.alloc(0x50);
        assert!(!e.call(0x005e_b410, &args![perk, 0u32]).bool());
        assert!(!e.call(0x005e_b410, &args![perk, 0x7777u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_b410, &args![perk, 0x6666u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x007a_7eb0),
            vec![vec![perk + 0x48, 0x6666, 0x005e_b460]]
        );
    }

    #[test]
    fn fn_005eb460_orders_by_rank_priority_then_slot_0x10() {
        let mut e = Engine::new();
        e.register(ENTRY_RANK, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(0x004f_6de0, |e, a| u32::from(e.mem.u8(a[0] + 5)).into_ret());
        vtable(&mut e, 0x0200_0000, &[]);
        e.mem.set_u32(0x0200_0000 + 0x10, 0x0090_0000);
        e.register(0x0090_0000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let entry = |e: &mut Engine, rank: u8, priority: u8, order: i32| {
            let p = object(e, 0x0200_0000, 0xc);
            e.mem.set_u8(p + 4, rank);
            e.mem.set_u8(p + 5, priority);
            e.mem.set_i32(p + 8, order);
            p
        };
        let cases = [
            ((1, 0, 0), (2, 0, 0), -1),
            ((3, 0, 0), (2, 0, 0), 1),
            ((1, 5, 0), (1, 4, 0), -1),
            ((1, 4, 0), (1, 5, 0), 1),
            ((1, 4, -3), (1, 4, 2), -1),
            ((1, 4, 3), (1, 4, 2), 1),
            ((1, 4, 2), (1, 4, 2), 0),
        ];
        for ((r1, p1, o1), (r2, p2, o2), want) in cases {
            let a = entry(&mut e, r1, p1, o1);
            let b = entry(&mut e, r2, p2, o2);
            assert_eq!(e.call(0x005e_b460, &args![a, b]).i32(), want);
        }
    }

    #[test]
    fn fn_005eb5e0_checks_the_function_0x46_items() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(PLAYER, 0x0abc_0000);
        let results = Rc::new(Cell::new(0u32));
        let r = results.clone();
        e.register_double(0x0068_1600, move |_, _| {
            r.set(r.get() + 1);
            u32::from(r.get() > 1).into_ret()
        });
        let item = |e: &mut Engine, function: u16| {
            let p = e.mem.alloc(0x10);
            e.mem.set_u16(p + 8, function);
            p
        };
        let a = item(&mut e, 0x46);
        let b = item(&mut e, 0x10);
        let c = item(&mut e, 0x46);
        let perk = e.mem.alloc(0x50);
        fill_list(&mut e, perk + 0x40, &[a, b, c]);
        // The flag at +0x3B is 0: nothing is asked.
        assert_eq!(e.call(0x005e_b5e0, &args![perk]).u32() & 0xff, 0);
        assert_eq!(results.get(), 0);
        // First check returns 0: the walk stops at once.
        e.mem.set_u8(perk + 0x3b, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_b5e0, &args![perk]).u32() & 0xff, 0);
        let log = e.call_log.take().unwrap();
        let checks = calls_to(&log, 0x0068_1600);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][..3], [a, 0x0abc_0000, 0]);
        // Later checks pass: both function 0x46 items are asked.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_b5e0, &args![perk]).u32() & 0xff, 1);
        assert_eq!(calls_to(&e.call_log.take().unwrap(), 0x0068_1600).len(), 2);
    }

    #[test]
    fn fn_005eb6a0_applies_by_rank_or_for_each_player_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(PLAYER, 0x0abc_0000);
        e.register(ENTRY_RANK, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        vtable(
            &mut e,
            0x0200_0000,
            &[(0x30, 0x0090_0030), (0x34, 0x0090_0034)],
        );
        e.mem.set_u32(0x0200_0000 + 0x10, 0x0090_0010);
        e.register(0x0090_0010, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let entry = |e: &mut Engine, rank: u8, kind: u32| {
            let p = object(e, 0x0200_0000, 0xc);
            e.mem.set_u8(p + 4, rank);
            e.mem.set_u32(p + 8, kind);
            p
        };
        let low = entry(&mut e, 0, 0);
        let high = entry(&mut e, 2, 2);
        let perk = e.mem.alloc(0x50);
        fill_list(&mut e, perk + 0x48, &[low, 0, high]);
        e.call_log = Some(vec![]);
        e.call(0x005e_b6a0, &args![perk, 0x7000u32, 1u32, 3u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0090_0034), vec![vec![low, 0x7000, 0]]);
        assert_eq!(calls_to(&log, 0x0090_0030), vec![vec![high, 0x7000, 0]]);

        // With the flag: for each item of the player's list.
        let items = e.mem.alloc(8);
        fill_list(&mut e, items, &[0x0101, 0x0202]);
        e.register(0x005a_4910, |_, _| 2u32.into_ret());
        e.register_double(0x005e_b7e0, move |_, _| items.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_b6a0, &args![perk, 0x7000u32, 1u32, 3u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0090_0030),
            vec![
                vec![low, 0x0101, 1],
                vec![low, 0x0202, 1],
                vec![high, 0x0abc_0000, 1],
                vec![high, 0x0abc_0000, 1],
            ]
        );
        e.register(0x005a_4910, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_b6a0, &args![perk, 0x7000u32, 1u32, 3u32, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0090_0030).is_empty());
    }

    #[test]
    fn fn_005eb900_finds_the_entry_of_a_rank() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(ENTRY_RANK, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        let entry = |e: &mut Engine, rank: u8| {
            let p = e.mem.alloc(8);
            e.mem.set_u8(p + 4, rank);
            p
        };
        let a = entry(&mut e, 0);
        let b = entry(&mut e, 1);
        let c = entry(&mut e, 1);
        let perk = e.mem.alloc(0x50);
        fill_list(&mut e, perk + 0x48, &[a, b, c]);
        assert_eq!(e.call(0x005e_b900, &args![perk, 2u32]).u32(), b);
        assert_eq!(e.call(0x005e_b900, &args![perk, 1u32]).u32(), a);
        assert_eq!(e.call(0x005e_b900, &args![perk, 5u32]).u32(), 0);
        fill_list(&mut e, perk + 0x48, &[0]);
        assert_eq!(e.call(0x005e_b900, &args![perk, 1u32]).u32(), 0);
    }

    #[test]
    fn is_perk_attainable_follows_the_sex_conditions() {
        let mut e = Engine::new();
        let make = |e: &mut Engine, function: u16| {
            let p = e.mem.alloc(0x18);
            e.mem.set_u16(p, function);
            e.mem.set_u32(p + 4, 0x0099_0000);
            p
        };
        let a = make(&mut e, 0x46);
        let b = make(&mut e, 0x10);
        let c = make(&mut e, 0x46);
        let list = [a, b, c];
        e.register_double(GET_CONDITION_ITEM, move |_, args| {
            list.get(args[1] as usize).copied().unwrap_or(0).into_ret()
        });
        e.register(CONDITION_DATA, |_, a| a[0].into_ret());
        e.register(0x0059_f3c0, |_, a| {
            // The item's first parameter (+4) and 0; the result is left
            // at 0.0.
            assert_eq!(a[1], 0x0099_0000);
            assert_eq!(a[2], 0);
            Ret::default()
        });
        let perk = e.mem.alloc(0x50);
        // The double writes nothing: d stays 0.0, so the last 0x46 item
        // decides false.
        assert!(!e.call(0x005e_ba00, &args![perk, 0x7000u32]).bool());
        e.register(0x0059_f3c0, |e, a| {
            e.mem.set_f64(a[3], 1.0);
            Ret::default()
        });
        assert!(e.call(0x005e_ba00, &args![perk, 0x7000u32]).bool());
        e.register(0x0059_f3c0, |e, a| {
            e.mem.set_f64(a[3], f64::NAN);
            Ret::default()
        });
        assert!(!e.call(0x005e_ba00, &args![perk, 0x7000u32]).bool());
        // No items: true.
        e.register(GET_CONDITION_ITEM, |_, _| Ret::default());
        assert!(e.call(0x005e_ba00, &args![perk, 0x7000u32]).bool());
    }
}

#[cfg(test)]
mod b028_description_tests {
    use super::*;

    /// A condition item: +8 function (`u16`), data at +0x10 (+4 the actor
    /// value or location, +8 the number), +0x20 the comparison kind, +0x24
    /// the joined flag, +0x28 the value (`double`).
    fn item(
        e: &mut Engine,
        function: u16,
        parameter: u32,
        number: u32,
        kind: u32,
        joined: bool,
        value: f64,
    ) -> u32 {
        let p = e.mem.alloc(0x30);
        e.mem.set_u16(p + 8, function);
        e.mem.set_u32(p + 0x14, parameter);
        e.mem.set_u32(p + 0x18, number);
        e.mem.set_u32(p + 0x20, kind);
        e.mem.set_u8(p + 0x24, u8::from(joined));
        e.mem.set_f64(p + 0x28, value);
        p
    }

    fn set_items(e: &mut Engine, items: Vec<(u32, u32)>) {
        e.register_double(GET_CONDITION_ITEM, move |_, a| {
            items
                .iter()
                .find(|(index, _)| *index == a[1])
                .map_or(0, |(_, item)| *item)
                .into_ret()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(CONDITION_DATA, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x005e_c060, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(0x005e_c040, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24)).into_ret()
        });
        e.register(0x0068_1490, |e, a| e.mem.f64(a[0] + 0x28).into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        // Abbreviations: value 0x99 has none.
        e.register(0x0066_eb00, |_, a| {
            let label = if a[0] == 0x99 {
                0u32
            } else {
                0x0aa0_0000 + a[0]
            };
            label.into_ret()
        });
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(0x0040_8da0, |_, a| a[0].into_ret());
        e.register(STRING_FORMAT, |_, _| Ret::default());
        e.register(STRING_APPEND, |_, _| Ret::default());
        e.put_vtable(0x0200_0000, &[0; 8]);
        e.mem.set_u32(0x0200_0000 + 0x10, 0x0090_0000);
        e.register(0x0090_0000, |_, _| 0x0dd0_0000u32.into_ret());
        e
    }

    fn perk(e: &mut Engine, ranks: u8) -> u32 {
        let p = e.mem.alloc(0x50);
        e.mem.set_u32(p + 0x24, 0x0200_0000);
        e.mem.set_u8(p + 0x39, ranks);
        e.mem.set_u8(p + 0x3a, 12);
        p
    }

    /// The string calls (format or append) with their words, in order.
    fn strings(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == STRING_FORMAT || c.0 == STRING_APPEND)
            .collect()
    }

    #[test]
    fn the_description_lists_each_requirement() {
        let mut e = engine();
        let a = item(&mut e, 0x1ef, 5, 0, 2, true, 6.0);
        let location = e.mem.alloc(0x20);
        let b = item(&mut e, 0x1c1, location, 3, 0, false, 0.0);
        let c = item(&mut e, 0x1ef, 0x17, 0, 0, false, 9.0);
        let d = item(&mut e, 0x1ef, 7, 0, 5, false, 2.0);
        set_items(&mut e, vec![(0, a), (1, b), (3, c), (5, d)]);
        let this = perk(&mut e, 3);
        let buffer = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        e.call(0x005e_bac0, &args![this, buffer, 0x200u32]);
        let calls = strings(&mut e);
        let work = calls[2].1[0];
        assert_ne!(work, buffer);
        assert_eq!(
            calls,
            vec![
                (
                    STRING_FORMAT,
                    vec![buffer, 0x200, 0x0104_7040, 0x011d_4679, 0x011d_3b2d, 3]
                ),
                (STRING_APPEND, vec![buffer, 0x200, 0x0101_2630]),
                (
                    STRING_FORMAT,
                    vec![
                        work,
                        0x80,
                        0x0104_7028,
                        0x0aa0_0005,
                        0x0102_0770,
                        5,
                        0x011d_324d
                    ]
                ),
                (STRING_APPEND, vec![buffer, 0x200, work]),
                (
                    STRING_FORMAT,
                    vec![work, 0x80, 0x0104_700c, location + 0x18, 3]
                ),
                (STRING_APPEND, vec![buffer, 0x200, work]),
                (
                    STRING_FORMAT,
                    vec![work, 0x80, 0x0104_7014, 0x0aa0_0007, 0x0104_7034, 3]
                ),
                (STRING_APPEND, vec![buffer, 0x200, work]),
                (STRING_APPEND, vec![buffer, 0x200, 0x0101_2648]),
                (
                    STRING_FORMAT,
                    vec![work, 0x80, 0x0104_6ffc, 0x011d_40e5, 12]
                ),
                (STRING_APPEND, vec![buffer, 0x200, work]),
                (STRING_APPEND, vec![buffer, 0x200, 0x0dd0_0000]),
            ]
        );
    }

    #[test]
    fn the_description_marks_an_empty_list_and_skips_zero_sums() {
        let mut e = engine();
        // Value 1.0 with kind 2 (-1) sums to 0: skipped. A location test
        // without a location appends the work buffer as it is.
        let zero = item(&mut e, 0x1ef, 5, 0, 2, false, 1.0);
        let nowhere = item(&mut e, 0x1c1, 0, 0, 0, false, 0.0);
        let unknown = item(&mut e, 0x1ef, 0x99, 0, 0, false, 4.0);
        set_items(&mut e, vec![(0, zero), (1, unknown)]);
        let this = perk(&mut e, 1);
        let buffer = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        e.call(0x005e_bac0, &args![this, buffer, 0x200u32]);
        let calls = strings(&mut e);
        assert_eq!(
            calls[0],
            (STRING_FORMAT, vec![buffer, 0x200, 0x0104_7038, 0x011d_4679])
        );
        assert_eq!(calls[1], (STRING_APPEND, vec![buffer, 0x200, 0x0104_7008]));
        assert_eq!(calls[2], (STRING_APPEND, vec![buffer, 0x200, 0x0101_2648]));

        set_items(&mut e, vec![(0, nowhere)]);
        let this = perk(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x005e_bac0, &args![this, buffer, 0x200u32]);
        let calls = strings(&mut e);
        // Long heading, then the separator, then the untouched work buffer.
        assert_eq!(calls[1], (STRING_APPEND, vec![buffer, 0x200, 0x0101_2630]));
        assert_eq!(calls[2].0, STRING_APPEND);
        assert_ne!(calls[2].1[2], 0x0104_7008);
        assert_eq!(calls[3], (STRING_APPEND, vec![buffer, 0x200, 0x0101_2648]));
    }
}
