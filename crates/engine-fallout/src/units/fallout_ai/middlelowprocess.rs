//! `fallout/ai/middlelowprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00406d70` (cdecl: actor value code, flag mask): whether the actor value
/// has the flag (0x100 here: no temporary modifiers).
const ACTOR_VALUE_HAS_FLAG: u32 = 0x0040_6d70;
/// `LowProcess::GetActorFloatValue` (`00907380`, thiscall, 3 words; ST0).
const LOW_PROCESS_GET_ACTOR_FLOAT_VALUE: u32 = 0x0090_7380;
/// `LowProcess::GetActorValue` (`009072d0`, thiscall, 3 words).
const LOW_PROCESS_GET_ACTOR_VALUE: u32 = 0x0090_72d0;
/// `ModifierList::GetModifier` (`00937730`, thiscall, actor value byte;
/// ST0).
const MODIFIER_LIST_GET_MODIFIER: u32 = 0x0093_7730;
/// `ModifierList::AdjustModifier` (`00937480`, thiscall: actor value byte,
/// float, kind).
const MODIFIER_LIST_ADJUST_MODIFIER: u32 = 0x0093_7480;
/// `ModifierList::SaveGame_ov2` (`00937b50`, thiscall, buffer).
const MODIFIER_LIST_SAVE_GAME: u32 = 0x0093_7b50;
/// `00937160` (thiscall, other list): copies a modifier list.
const MODIFIER_LIST_COPY: u32 = 0x0093_7160;
/// `00937030` (thiscall): the modifier list's destructor.
const MODIFIER_LIST_DESTRUCT: u32 = 0x0093_7030;
/// `LowProcess::SaveGame_ov2` (`00910450`, thiscall, buffer).
const LOW_PROCESS_SAVE_GAME: u32 = 0x0091_0450;
/// `00865e50` (thiscall on the save buffer: source, size, flag): writes
/// bytes.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// `00428110` (thiscall on the save buffer, out word): the buffer's change
/// flags.
const SAVE_BUFFER_CHANGE_FLAGS: u32 = 0x0042_8110;
/// `004280f0` (thiscall, flag): whether the change flags have the flag.
const CHANGE_FLAGS_HAS: u32 = 0x0042_80f0;
/// The lower process's destructor (`00906fe0`, thiscall).
const LOW_PROCESS_DESTRUCT: u32 = 0x0090_6fe0;
/// `LowProcess::CheckforNewPackage` (`0090a1a0`, thiscall: word, flag).
const LOW_PROCESS_CHECK_FOR_NEW_PACKAGE: u32 = 0x0090_a1a0;
/// `LowProcess::Update_ov2` (`0090c360`, thiscall: word, float).
const LOW_PROCESS_UPDATE_OV2: u32 = 0x0090_c360;
/// `Calendar::GetHour` (`00867da0`, thiscall on the calendar; ST0).
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// The game calendar.
const CALENDAR: u32 = 0x011d_e7b8;
/// `_ftol2_sse` (`00ec62c0`): truncates ST0 to an integer in EAX.
const FTOL: u32 = 0x00ec_62c0;
/// `00401030` (cdecl, pointer): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `MiddleLowProcess`'s vtable (RTTI `.?AVMiddleLowProcess@@`).
const MIDDLE_LOW_PROCESS_VTABLE: u32 = 0x0108_9bcc;
/// The hour of the last package check (`int`, +0xB4).
const LAST_PACKAGE_HOUR: u32 = 0xb4;
/// The temporary-modifier list (+0xB8).
const TEMPORARY_MODIFIERS: u32 = 0xb8;

/// Whether the actor value `code` has flag 0x100 (no temporary modifiers).
fn skips_modifiers(e: &mut Engine, code: u32) -> bool {
    e.call(ACTOR_VALUE_HAS_FLAG, &args![code, 0x100u32]).bool()
}

// Translated from 0092ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::GetActorFloatValue` (Xbox PDB name in the engine
/// map): `LowProcess::GetActorFloatValue`, plus the temporary modifier of
/// the actor value (`ModifierList::GetModifier`, the sum stored as a float)
/// unless the value has flag 0x100. Returns ST0.
pub fn middle_low_process_get_actor_float_value(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    code: u32,
    extra: u32,
) -> f64 {
    if skips_modifiers(e, code) {
        return e
            .call(
                LOW_PROCESS_GET_ACTOR_FLOAT_VALUE,
                &args![this, actor, code, extra],
            )
            .st0;
    }
    let base = e
        .call(
            LOW_PROCESS_GET_ACTOR_FLOAT_VALUE,
            &args![this, actor, code, extra],
        )
        .st0;
    let modifier = e
        .call(
            MODIFIER_LIST_GET_MODIFIER,
            &args![this.byte_add(TEMPORARY_MODIFIERS), code & 0xff],
        )
        .st0;
    x87_load32(x87_store32(x87_add(modifier, base)))
}

// Translated from 0092cd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::GetActorValue` (Xbox PDB name in the engine map):
/// `LowProcess::GetActorValue`, plus the temporary modifier truncated to an
/// integer (`_ftol2`) unless the value has flag 0x100.
pub fn middle_low_process_get_actor_value(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    code: u32,
    extra: u32,
) -> i32 {
    if skips_modifiers(e, code) {
        return e
            .call(
                LOW_PROCESS_GET_ACTOR_VALUE,
                &args![this, actor, code, extra],
            )
            .i32();
    }
    let base = e
        .call(
            LOW_PROCESS_GET_ACTOR_VALUE,
            &args![this, actor, code, extra],
        )
        .i32();
    let modifier = e
        .call(
            MODIFIER_LIST_GET_MODIFIER,
            &args![this.byte_add(TEMPORARY_MODIFIERS), code & 0xff],
        )
        .st0;
    let whole = e.call(FTOL, &args![modifier]).i32();
    whole.wrapping_add(base)
}

// Translated from 0092cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::TempModActorValue_ov2` (Xbox PDB name in the engine
/// map): unless the actor value has flag 0x100, adjusts its temporary
/// modifier by `amount` (`ModifierList::AdjustModifier`, kind 2).
pub fn middle_low_process_temp_mod_actor_value_ov2(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    code: u32,
    amount: f32,
) {
    if skips_modifiers(e, code) {
        return;
    }
    let amount = x87_quiet_bits(amount.to_bits());
    e.call(
        MODIFIER_LIST_ADJUST_MODIFIER,
        &args![
            this.byte_add(TEMPORARY_MODIFIERS),
            code & 0xff,
            amount,
            2u32
        ],
    );
}

// Translated from 0092cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::GetTemporaryModifier` (Xbox PDB name in the engine
/// map): `ModifierList::GetModifier` of the temporary modifiers; returns
/// its ST0.
pub fn middle_low_process_get_temporary_modifier(e: &mut Engine, this: Ptr, code: u8) -> f64 {
    e.call(
        MODIFIER_LIST_GET_MODIFIER,
        &args![this.byte_add(TEMPORARY_MODIFIERS), u32::from(code)],
    )
    .st0
}

// Translated from 0092ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::SaveGame_ov2` (Xbox PDB name in the engine map):
/// `LowProcess::SaveGame_ov2`, the last package hour (4 bytes), then the
/// temporary modifiers when the buffer's change flags have 0x100000.
pub fn middle_low_process_save_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOW_PROCESS_SAVE_GAME, &args![this, buffer]);
    e.call(
        SAVE_BUFFER_WRITE,
        &args![buffer, this.byte_add(LAST_PACKAGE_HOUR), 4u32, 0u32],
    );
    let has = e.with_stack(4, |e, flags| {
        let flags = e
            .call(SAVE_BUFFER_CHANGE_FLAGS, &args![buffer, flags])
            .u32();
        e.call(CHANGE_FLAGS_HAS, &args![flags, 0x0010_0000u32])
            .bool()
    });
    if has {
        e.call(
            MODIFIER_LIST_SAVE_GAME,
            &args![this.byte_add(TEMPORARY_MODIFIERS), buffer],
        );
    }
}

// Translated from 0092ca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the last package hour and the
/// temporary modifiers (`00937160`) from `other`.
pub fn fn_0092ca70(e: &mut Engine, this: Ptr, other: Ptr) {
    let hour = e.mem.u32(other.addr() + LAST_PACKAGE_HOUR);
    e.mem.set_u32(this.addr() + LAST_PACKAGE_HOUR, hour);
    e.call(
        MODIFIER_LIST_COPY,
        &args![
            this.byte_add(TEMPORARY_MODIFIERS),
            other.byte_add(TEMPORARY_MODIFIERS)
        ],
    );
}

// Translated from 0092ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `MiddleLowProcess` destructor: stores the
/// vtable, destroys the temporary modifiers (`00937030`) and runs the lower
/// process's destructor (`00906fe0`).
// Translation incomplete: 0092ca00 - SEH exception unwinding is unsupported.
pub fn fn_0092ca00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MIDDLE_LOW_PROCESS_VTABLE);
    e.call(
        MODIFIER_LIST_DESTRUCT,
        &args![this.byte_add(TEMPORARY_MODIFIERS)],
    );
    e.call(LOW_PROCESS_DESTRUCT, &args![this]);
}

// Translated from 0092c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess`'s scalar deleting destructor (Xbox PDB name in the
/// engine map): the destructor (`0092ca00`), then frees `this`
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn middle_low_process_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0092ca00(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 0092cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::CheckforNewPackage` (Xbox PDB name in the engine map):
/// without `force`, when the process has a package (vtable +0x22C) and the
/// calendar's hour (truncated) is still the last package hour, answers
/// false; otherwise `LowProcess::CheckforNewPackage` and the hour becomes
/// the last package hour.
pub fn middle_low_process_checkfor_new_package(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    force: u8,
) -> bool {
    let t = this.addr();
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).st0;
    let hour = x87_store32(hour);
    if force == 0 && e.vcall(t, 0x22c, &[]).u32() != 0 {
        let whole = e.call(FTOL, &args![x87_load32(hour)]).u32();
        if e.mem.u32(t + LAST_PACKAGE_HOUR) == whole {
            return false;
        }
    }
    let result = e
        .call(
            LOW_PROCESS_CHECK_FOR_NEW_PACKAGE,
            &args![this, actor, u32::from(force)],
        )
        .bool();
    let whole = e.call(FTOL, &args![x87_load32(hour)]).u32();
    e.mem.set_u32(t + LAST_PACKAGE_HOUR, whole);
    result
}

// Translated from 0092e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::Update_ov2` (Xbox PDB name in the engine map): passes
/// on to `LowProcess::Update_ov2` (the float through an x87 load and
/// store).
pub fn middle_low_process_update_ov2(e: &mut Engine, this: Ptr, actor: u32, delta: f32) {
    let delta = x87_quiet_bits(delta.to_bits());
    e.call(LOW_PROCESS_UPDATE_OV2, &args![this, actor, delta]);
}

/// `00483710` (thiscall): an empty body, called on `011e0528` first.
const EMPTY_BODY: u32 = 0x0048_3710;
/// The object `Update` passes to `00483710`.
const UPDATE_SCOPE: u32 = 0x011e_0528;
/// `008d8520` `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB name in
/// the engine map; thiscall on the actor).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `008a8f60` (thiscall on the actor).
const ACTOR_008A8F60: u32 = 0x008a_8f60;
/// `009611e0` (thiscall on the package): the package type (-1 for none).
const PACKAGE_TYPE: u32 = 0x0096_11e0;
/// `008a6290` (thiscall on the package): a flag of the package (byte).
const PACKAGE_008A6290: u32 = 0x008a_6290;
/// `LowProcess::ResetTarget` (`0090a070`, thiscall).
const RESET_TARGET: u32 = 0x0090_a070;
/// `0055b980` (thiscall on the package): its location.
const PACKAGE_LOCATION: u32 = 0x0055_b980;
/// Per package type, the table of actions by procedure index.
const PROCEDURE_ACTIONS: u32 = 0x011a_3ff0;
/// `PackageLocation::GetLocReference` (`0067f390`, thiscall).
const LOCATION_GET_REFERENCE: u32 = 0x0067_f390;
/// `-1.0` (`float`), passed to the package's vtable +0x13C and +0x140.
const MINUS_ONE: u32 = 0x0101_2054;
/// `Actor::IsPathingComplete` (`008b3bb0`, thiscall).
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
/// `004839c0` (cdecl, one word): a form by its index (here 0x3A, a
/// `TESGlobal` whose value is read through `00526ac0`).
const FORM_BY_INDEX: u32 = 0x0048_39c0;
/// `007df1f0` (thiscall on the process; ST0): an hour the schedule compares
/// against.
const PROCESS_HOUR: u32 = 0x007d_f1f0;
/// `24.0` (`double`).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
/// `00526ac0` (thiscall; ST0): a `TESGlobal`'s value.
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `3600.0` (`double`).
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
/// `TESPackage::GetPackageRadiusActorToLocation` (`00678670`, thiscall:
/// actor, 0; ST0).
const PACKAGE_RADIUS: u32 = 0x0067_8670;
/// `0041ca90` (thiscall on the package): its procedure type.
const PACKAGE_PROCEDURE: u32 = 0x0041_ca90;
/// `TESPackage::GetLocationWorld` (`00675a50`, thiscall, actor).
const PACKAGE_LOCATION_WORLD: u32 = 0x0067_5a50;
/// `TESPackage::GetLocationCell` (`00675c20`, thiscall, actor).
const PACKAGE_LOCATION_CELL: u32 = 0x0067_5c20;
/// `TESPackage::GetLocationCoord` (`00675de0`, thiscall: out point, actor).
const PACKAGE_LOCATION_COORD: u32 = 0x0067_5de0;
/// `Actor::SetPathfindingGoal_ov2` (`008b3690`, thiscall, 5 words).
const SET_PATHFINDING_GOAL: u32 = 0x008b_3690;
/// `0045cd60` (thiscall on the acquire object): its state.
const ACQUIRE_STATE: u32 = 0x0045_cd60;
/// `00670f40` / `00670f60` (thiscall on the package): two package flags
/// that ask for a look around (callbacks `0090d480` / `0090d570`).
const PACKAGE_FLAG_00670F40: u32 = 0x0067_0f40;
const PACKAGE_FLAG_00670F60: u32 = 0x0067_0f60;
/// The callbacks `TESDataHandler::EnumReferencesCloseToPoint` gets.
const CLOSE_CALLBACK_0090D480: u32 = 0x0090_d480;
const CLOSE_CALLBACK_0090D570: u32 = 0x0090_d570;
/// `5000.0` (`float`): the search radius.
const CLOSE_RADIUS: u32 = 0x0103_0020;
/// `008d6f30` (thiscall on the actor): its parent cell.
const ACTOR_PARENT_CELL: u32 = 0x008d_6f30;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESDataHandler::EnumReferencesCloseToPoint` (`0046f280`, thiscall, 7
/// words).
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
/// `007af430` (thiscall): the target's form (named
/// `BGSSaveFormBuffer::GetForm` in the engine map, a folded body).
const TARGET_FORM: u32 = 0x007a_f430;
/// The form the target is compared with (a global pointer).
const TARGET_FORM_GLOBAL: u32 = 0x011c_a248;
/// `PackageLocation::GetLocType` (`00678ca0`, thiscall).
const LOCATION_GET_TYPE: u32 = 0x0067_8ca0;
/// `00430830` (thiscall on the target): the point whose +8 float is used.
const TARGET_POINT: u32 = 0x0043_0830;
/// `Actor::EndInterruptPackage` (`00881680`, thiscall, one word).
const END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// `00672dd0` (thiscall on the package).
const PACKAGE_00672DD0: u32 = 0x0067_2dd0;
/// The player singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// `Interface::GetTargetREFR` (`00703180`).
const GET_TARGET_REFR: u32 = 0x0070_3180;
/// The procedure names, by procedure type.
const PROCEDURE_NAMES: u32 = 0x0119_bcb0;
/// `00464f30` (cdecl: text, 0).
const PROCEDURE_NAME_TEXT: u32 = 0x0046_4f30;
/// `0055d520` (thiscall on the actor): its name.
const ACTOR_NAME: u32 = 0x0055_d520;
/// `sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `00703c00` (cdecl, text): shows the text.
const SHOW_TEXT: u32 = 0x0070_3c00;
/// `"%s is sleeping with %s"`, `"%s is eating with %s"`, `"%s is done with
/// %s"`.
const SLEEPING_FORMAT: u32 = 0x0108_8f2c;
const EATING_FORMAT: u32 = 0x0108_8f14;
const DONE_FORMAT: u32 = 0x0108_8f00;
/// `00440d80` (thiscall on the process's object at +0x40).
const OBJECT_00440D80: u32 = 0x0044_0d80;
/// `00962720` (thiscall on the player, actor).
const PLAYER_00962720: u32 = 0x0096_2720;
/// `00962620` (thiscall on the player).
const PLAYER_00962620: u32 = 0x0096_2620;
/// `0043d4d0` (thiscall on the setting at `011cdad0`): its value's address.
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_011CDAD0: u32 = 0x011c_dad0;
/// `00674f30` (thiscall on the package, one word).
const PACKAGE_00674F30: u32 = 0x0067_4f30;
/// `00671d10` (thiscall on the package): its data.
const PACKAGE_DATA: u32 = 0x0067_1d10;
/// `0044ddc0` (thiscall): the word at +8.
const WORD_AT_8: u32 = 0x0044_ddc0;
/// `TESObjectREFR::GetDistanceFromReference` (`005723b0`, thiscall, 3
/// words; ST0).
const DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `0.0` and `2.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `00425fd0` (thiscall on the cell): whether it is an interior.
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `00403e20` (thiscall): a setting's value address (`this + 4`).
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// The two distance settings (interior, default).
const SETTING_011CDBAC: u32 = 0x011c_dbac;
const SETTING_011CD82C: u32 = 0x011c_d82c;
/// `MobileObject::GetCurrentProcessType` (`00931850`, thiscall).
const CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `005d43c0` (thiscall): `this + 0x44` (the extra data list).
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `Script::SetActionFlag` (`005ac750`, cdecl, 3 words).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `TESPackage::GetOncePerDay` (`00670f90`, thiscall).
const PACKAGE_ONCE_PER_DAY: u32 = 0x0067_0f90;
/// `Calendar::GetDay` (`00867d60`, thiscall on the calendar; byte).
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
/// `TESPackage::GetIsCreated` (`00674d40`, thiscall).
const PACKAGE_IS_CREATED: u32 = 0x0067_4d40;
/// `007b3fa0` (thiscall, one word): deletes the object.
const DELETE_OBJECT: u32 = 0x007b_3fa0;
/// `008256d0` (thiscall on the list at +0x5C): whether it is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (thiscall): returns `this`.
const SAME_POINTER: u32 = 0x0068_15c0;
/// `00905330` (thiscall on the list, word address): removes the item.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `0041d8a0` (thiscall on the package), then `00726070` (the word at +4).
const PACKAGE_0041D8A0: u32 = 0x0041_d8a0;
const WORD_AT_4: u32 = 0x0072_6070;
/// `Actor::IsWaitingOnPath` (`008b3bf0`, thiscall).
const IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;

/// The process's current package (vtable +0x22C).
fn current_package(e: &mut Engine, process: u32) -> u32 {
    e.vcall(process, 0x22c, &[]).u32()
}

/// The action the table gives for the current package's type and the
/// process's procedure index (vtable +0x280).
fn procedure_action(e: &mut Engine, process: u32) -> u32 {
    let package = current_package(e, process);
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let index = e.vcall(process, 0x280, &[]).u32();
    let row = e.global::<u32>(PROCEDURE_ACTIONS.wrapping_add(kind.wrapping_mul(4)));
    e.global::<u32>(row.wrapping_add(index.wrapping_mul(4)))
}

/// The saved acquire object's vtable +0x28 (`Update`'s last step).
fn update_acquire_object(e: &mut Engine, actor: u32) {
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    e.vcall(object, 0x28, &[]);
}

/// The package's vtable +0x13C with (actor, 0, -1.0, 0).
fn package_test_13c(e: &mut Engine, process: u32, actor: u32) -> bool {
    let package = current_package(e, process);
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32])
        .bool()
}

/// `TESDataHandler::EnumReferencesCloseToPoint` around the actor (its
/// vtable +0x1F4 twice, radius 5000) with `callback`.
fn enum_close_references(e: &mut Engine, actor: u32, callback: u32) {
    let radius = x87_quiet_bits(e.global::<u32>(CLOSE_RADIUS));
    let first = e.vcall(actor, 0x1f4, &[]).u32();
    let radius_again = x87_quiet_bits(e.global::<u32>(CLOSE_RADIUS));
    let second = e.vcall(actor, 0x1f4, &[]).u32();
    let cell = e.call(ACTOR_PARENT_CELL, &args![actor]).u32();
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(
        ENUM_REFERENCES_CLOSE_TO_POINT,
        &args![
            handler,
            cell,
            second,
            radius_again,
            first,
            radius,
            callback,
            actor
        ],
    );
}

/// Shows "`format`" with the actor's name and the procedure's name when the
/// actor is the interface's target.
fn show_procedure_text(e: &mut Engine, process: u32, actor: u32, format: u32) {
    if e.call(GET_TARGET_REFR, &[]).u32() != actor {
        return;
    }
    let package = current_package(e, process);
    let procedure = e.call(PACKAGE_PROCEDURE, &args![package]).u32();
    let name_text = e.global::<u32>(PROCEDURE_NAMES.wrapping_add(procedure.wrapping_mul(4)));
    let procedure_name = e.call(PROCEDURE_NAME_TEXT, &args![name_text, 0u32]).u32();
    let actor_name = e.call(ACTOR_NAME, &args![actor]).u32();
    e.with_stack(0x130, |e, text| {
        e.call(SPRINTF, &args![text, format, actor_name, procedure_name]);
        e.call(SHOW_TEXT, &args![text]);
    });
}

/// `Update`'s action 0: travel to the package location and look around
/// when there.
fn update_travel(e: &mut Engine, process: u32, actor: u32, location: u32) {
    let mut target = 0u32;
    if location != 0 {
        target = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
    }
    if target == 0 {
        let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        target = if acquire != 0 {
            e.vcall(acquire, 0x514, &[]).u32()
        } else {
            0
        };
    }
    if !package_test_13c(e, process, actor) && e.call(IS_PATHING_COMPLETE, &args![actor]).bool() {
        let global = e.call(FORM_BY_INDEX, &args![0x3au32]).u32();
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).st0;
        let mut hour = x87_store32(hour);
        let start = e.call(PROCESS_HOUR, &args![process]).st0;
        if x87_load32(hour) < start {
            let day: f64 = e.global(HOURS_PER_DAY);
            hour = x87_store32(x87_add(x87_load32(hour), day));
        }
        let start = e.call(PROCESS_HOUR, &args![process]).st0;
        let elapsed = x87_store32(x87_sub(x87_m32(hour), start));
        let scale = e.call(GLOBAL_VALUE, &args![global]).st0;
        let seconds: f64 = e.global(SECONDS_PER_HOUR);
        // The seconds this many game hours take; the exe stores it and
        // never reads it again.
        let _elapsed_seconds = x87_store32(x87_mul(x87_div(seconds, scale), x87_m32(elapsed)));
        let package = current_package(e, process);
        let radius = e.call(PACKAGE_RADIUS, &args![package, actor, 0u32]).st0;
        let mut radius = x87_store32(radius);
        let package = current_package(e, process);
        if e.call(PACKAGE_PROCEDURE, &args![package]).u32() == 5 {
            radius = x87_store32(0.0);
        }
        let radius = x87_quiet_bits(radius);
        let package = current_package(e, process);
        let world = e.call(PACKAGE_LOCATION_WORLD, &args![package, actor]).u32();
        let package = current_package(e, process);
        let cell = e.call(PACKAGE_LOCATION_CELL, &args![package, actor]).u32();
        e.with_stack(0xc, |e, point| {
            let package = current_package(e, process);
            let coord = e
                .call(PACKAGE_LOCATION_COORD, &args![package, point, actor])
                .u32();
            e.call(
                SET_PATHFINDING_GOAL,
                &args![actor, coord, cell, world, radius, 0u32],
            );
        });
    }
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    if e.call(ACQUIRE_STATE, &args![acquire]).u32() != 2 {
        return;
    }
    if !package_test_13c(e, process, actor) {
        return;
    }
    if !e.vcall(process, 0x1bc, &[]).bool() {
        let package = current_package(e, process);
        if e.call(PACKAGE_FLAG_00670F40, &args![package]).bool() {
            enum_close_references(e, actor, CLOSE_CALLBACK_0090D480);
        }
        e.vcall(process, 0x1c0, &args![1u32]);
    }
    let package = current_package(e, process);
    if e.call(PACKAGE_FLAG_00670F60, &args![package]).bool() {
        enum_close_references(e, actor, CLOSE_CALLBACK_0090D570);
    }
    let face = (target != 0
        && e.call(TARGET_FORM, &args![target]).u32() == e.global::<u32>(TARGET_FORM_GLOBAL))
        || {
            let package = current_package(e, process);
            if e.call(PACKAGE_LOCATION, &args![package]).u32() != 0 {
                let package = current_package(e, process);
                let location = e.call(PACKAGE_LOCATION, &args![package]).u32();
                e.call(LOCATION_GET_TYPE, &args![location]).u32() == 3
            } else {
                false
            }
        };
    if face {
        // The exe stores 0.0 in the local first; both branches replace it.
        let heading = if target != 0 {
            let point = e.call(TARGET_POINT, &args![target]).u32();
            x87_quiet_bits(e.mem.u32(point.wrapping_add(8)))
        } else {
            e.with_stack(0xc, |e, out| {
                let point = e.vcall(actor, 0x16c, &args![out]).u32();
                x87_quiet_bits(e.mem.u32(point.wrapping_add(8)))
            })
        };
        e.vcall(actor, 0x2c4, &args![heading]);
    }
    e.vcall(process, 0x288, &args![actor, 1u32]);
}

/// `Update`'s action 0x31: sit or use the object at +0x40 when it is a
/// reference.
fn update_use_object(e: &mut Engine, process: u32, actor: u32) {
    let mut object = 0u32;
    let held = e.mem.u32(process + 0x40);
    if held != 0 && e.vcall(held, 0x100, &[]).bool() {
        object = e.mem.u32(process + 0x40);
    }
    if object == 0 {
        return;
    }
    let package = current_package(e, process);
    if e.call(PACKAGE_00672DD0, &args![package]).u32() != 0 {
        let package = current_package(e, process);
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        if !e
            .vcall(package, 0x140, &args![object, actor, 0u32, minus_one, 1u32])
            .bool()
        {
            return;
        }
    }
    e.vcall(process, 0x7c8, &args![actor, 1u32]);
}

/// `Update`'s action 0x11: follow (the object at +0x40, found through
/// vtable +0x8C when missing).
fn update_follow(e: &mut Engine, process: u32, actor: u32, package: u32) {
    if e.mem.u32(process + 0x40) == 0 {
        e.vcall(process, 0x8c, &args![actor]);
    }
    let object = e.mem.u32(process + 0x40);
    if object != 0 && object != e.global::<u32>(PLAYER) {
        e.vcall(process, 0xc8, &args![actor, package, 1u32]);
    }
    e.vcall(process, 0x288, &args![actor, 1u32]);
}

/// `Update`'s action 0xD: reach the object at +0x40.
fn update_reach_object(e: &mut Engine, process: u32, actor: u32, package: u32) {
    if e.mem.u32(process + 0x40) == 0 {
        e.vcall(process, 0x8c, &args![actor]);
    }
    let object = e.mem.u32(process + 0x40);
    if object == 0 || e.call(OBJECT_00440D80, &args![object]).bool() {
        e.vcall(process, 0x288, &args![actor, 1u32]);
        return;
    }
    if e.call(PACKAGE_008A6290, &args![package]).bool() {
        let player = e.global::<u32>(PLAYER);
        if !e.call(PLAYER_00962720, &args![player, actor]).bool() {
            let player = e.global::<u32>(PLAYER);
            let count = e.call(PLAYER_00962620, &args![player]).i32();
            let limit = e
                .call(SETTING_VALUE_ADDRESS, &args![SETTING_011CDAD0])
                .u32();
            if count < e.mem.i32(limit) {
                e.call(PACKAGE_00674F30, &args![package, 0u32]);
            }
        }
        return;
    }
    if e.call(PACKAGE_PROCEDURE, &args![package]).u32() == 0x1c
        || e.call(PACKAGE_008A6290, &args![package]).bool()
    {
        e.vcall(process, 0x288, &args![actor, 1u32]);
        return;
    }
    if e.call(PACKAGE_PROCEDURE, &args![package]).u32() == 1 {
        let data = e.call(PACKAGE_DATA, &args![package]).u32();
        let reach = e.call(WORD_AT_8, &args![data]).i32();
        let reach = x87_store32(f64::from(reach));
        let object = e.mem.u32(process + 0x40);
        let distance = e
            .call(DISTANCE_FROM_REFERENCE, &args![actor, object, 0u32, 0u32])
            .st0;
        // FCOMPP: no call when reach < distance or unordered.
        if distance <= x87_load32(reach) {
            e.vcall(process, 0x288, &args![actor, 1u32]);
        }
        return;
    }
    let mut reach = x87_store32(0.0);
    let package_now = current_package(e, process);
    let data = e.call(PACKAGE_DATA, &args![package_now]).u32();
    // The package's distance (200 when not above 0) is stored and never
    // read again.
    let distance_word = e.call(WORD_AT_8, &args![data]).i32();
    let _distance_word = if distance_word <= 0 {
        200
    } else {
        distance_word
    };
    let zero: f64 = e.global(ZERO_DOUBLE);
    let mut interior = false;
    if x87_load32(reach) == zero && e.call(ACTOR_PARENT_CELL, &args![actor]).u32() != 0 {
        let cell = e.call(ACTOR_PARENT_CELL, &args![actor]).u32();
        interior = e.call(CELL_IS_INTERIOR, &args![cell]).bool();
    }
    if interior {
        let value = e
            .call(SETTING_FLOAT_ADDRESS, &args![SETTING_011CDBAC])
            .u32();
        reach = x87_quiet_bits(e.mem.u32(value));
    } else if x87_load32(reach) == zero {
        let value = e
            .call(SETTING_FLOAT_ADDRESS, &args![SETTING_011CD82C])
            .u32();
        reach = x87_quiet_bits(e.mem.u32(value));
    }
    let object = e.mem.u32(process + 0x40);
    let distance = e
        .call(DISTANCE_FROM_REFERENCE, &args![actor, object, 0u32, 0u32])
        .st0;
    let two: f64 = e.global(TWO_DOUBLE);
    let half = x87_div(x87_load32(reach), two);
    if distance <= half {
        e.vcall(process, 0x288, &args![actor, 1u32]);
    }
}

/// `Update`'s switch on the procedure action (the jump tables at
/// `0092e3c8` and `0092e34c`, as `match` arms).
fn update_action(
    e: &mut Engine,
    process: u32,
    actor: u32,
    package: u32,
    location: u32,
    action: u32,
) {
    let one = |e: &mut Engine, slot: u32| {
        e.vcall(process, slot, &args![actor]);
    };
    match action {
        0 => update_travel(e, process, actor, location),
        1 => {
            let package = current_package(e, process);
            if e.call(PACKAGE_TYPE, &args![package]).u32() == 0x1a {
                e.vcall(process, 0x288, &args![actor, 0xffff_ffffu32]);
            }
        }
        2 => {
            e.vcall(process, 0x7c8, &args![actor, 1u32]);
        }
        3 => one(e, 0x7d0),
        4 => show_procedure_text(e, process, actor, SLEEPING_FORMAT),
        5 => show_procedure_text(e, process, actor, EATING_FORMAT),
        6 => {
            e.vcall(process, 0x298, &args![actor, 1u32, 0x101u32, 0u32]);
        }
        7 => one(e, 0x7b4),
        8 => one(e, 0x7c4),
        0xa => one(e, 0x2a0),
        0xc => {
            e.call(END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
        }
        0xd => update_reach_object(e, process, actor, package),
        0xe => {
            e.vcall(process, 0x7c8, &args![actor, 0u32]);
        }
        0xf => {
            e.vcall(process, 0x29c, &args![actor, 0u32, 1u32, 0x101u32]);
        }
        0x11 => update_follow(e, process, actor, package),
        0x1a => one(e, 0x808),
        0x1b => one(e, 0x7cc),
        0x1c => one(e, 0x7d8),
        0x1e => one(e, 0x7d4),
        0x21 => one(e, 0x7e0),
        0x22 => one(e, 0x7e4),
        0x26 => one(e, 0x800),
        0x27 => one(e, 0x7e8),
        0x29 => {
            e.vcall(process, 0x288, &args![actor, 2u32]);
        }
        0x2a => one(e, 0x7ec),
        0x2c => one(e, 0x7fc),
        0x2f => one(e, 0x7f4),
        0x31 => update_use_object(e, process, actor),
        0x32 => one(e, 0x7dc),
        0x33 => one(e, 0x7f0),
        _ => {}
    }
}

/// `Update` after the action, for a high process whose action is 0x36 (the
/// package is done). Returns false when `Update` stops right there (the
/// package's vtable +0x13C turned the actor away).
fn update_finish_package(e: &mut Engine, process: u32, actor: u32, package: &mut u32) -> bool {
    if !e.vcall(process, 0x5a4, &[]).bool() {
        let extra = e.call(EXTRA_LIST, &args![actor]).u32();
        let current = current_package(e, process);
        e.call(SET_ACTION_FLAG, &args![current, extra, 0x400u32]);
        let current = current_package(e, process);
        e.vcall(process, 0x5a0, &args![actor, current]);
        e.vcall(process, 0x5a8, &args![1u32]);
    }
    e.vcall(process, 0x1c8, &args![0u32]);
    if current_package(e, process) != 0 {
        let current = current_package(e, process);
        if e.call(PACKAGE_TYPE, &args![current]).u32() == 0 && !package_test_13c(e, process, actor)
        {
            e.vcall(process, 0x288, &args![actor, 0xffff_ffffu32]);
            return false;
        }
    }
    if current_package(e, process) != 0 {
        let current = current_package(e, process);
        if e.call(PACKAGE_ONCE_PER_DAY, &args![current]).bool() {
            let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u32() & 0xff;
            let current = current_package(e, process);
            e.vcall(actor, 0x28c, &args![current, day]);
        }
    }
    if current_package(e, process) != 0 {
        let current = current_package(e, process);
        if e.call(PACKAGE_IS_CREATED, &args![current]).bool() {
            e.call(END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
        }
    }
    show_procedure_text(e, process, actor, DONE_FORMAT);
    let object = e.mem.u32(process + 0x64);
    if object != 0 {
        e.call(DELETE_OBJECT, &args![object, 1u32]);
    }
    e.mem.set_u32(process + 0x64, 0);
    while !e.call(LIST_IS_EMPTY, &args![process + 0x5c]).bool() {
        let head = e.call(SAME_POINTER, &args![process + 0x5c]).u32();
        let item = e.mem.u32(head);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            if item != 0 {
                e.call(DELETE_OBJECT, &args![item, 1u32]);
            }
            e.call(LIST_REMOVE, &args![process + 0x5c, slot]);
        });
    }
    if current_package(e, process) != 0 {
        let current = current_package(e, process);
        let data = e.call(PACKAGE_0041D8A0, &args![current]).u32();
        if e.call(WORD_AT_4, &args![data]).u32() == 0 {
            let old = current_package(e, process);
            e.vcall(process, 0x24, &args![actor, 1u32]);
            *package = current_package(e, process);
            if old != current_package(e, process) {
                e.vcall(process, 0x284, &args![0u32]);
            }
        }
    }
    true
}

// Translated from 0092d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::Update` (Xbox PDB name in the engine map): for an
/// actor (its vtable +0x100), asks for a new package (vtable +0x24) and
/// re-evaluates it when there is one; with a current package that has a
/// type and no `008a6290` flag, resets the target and runs the action the
/// table at `011a3ff0` gives for the package type and procedure index
/// (travel, sleep, eat, follow, use an object, the many single vtable
/// calls, ...); a high process whose action is 0x36 finishes the package
/// (`update_finish_package`). Unless the actor waits on a path, the saved
/// acquire object's vtable +0x28 runs last (also for a non-actor). The
/// x87 operations are done in the exe's order with the exe's float
/// temporaries.
// Translation incomplete: 0092d050 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn middle_low_process_update(e: &mut Engine, this: Ptr, actor: u32) {
    let process = this.addr();
    e.call(EMPTY_BODY, &args![UPDATE_SCOPE]);
    if actor == 0 {
        return;
    }
    let is_actor = e.vcall(actor, 0x100, &[]).bool();
    let reference = if is_actor { actor } else { 0 };
    // The hour is stored and never read again.
    let _hour = x87_store32(e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).st0);
    if reference == 0 {
        update_acquire_object(e, actor);
        return;
    }
    if e.vcall(process, 0x24, &args![reference, 0u32]).bool() {
        e.vcall(process, 0x524, &args![reference]);
        e.call(ACTOR_008A8F60, &args![reference]);
        e.vcall(process, 0x90, &args![reference, 0u32]);
    }
    let mut package = current_package(e, process);
    let runs = current_package(e, process) != 0
        && {
            let current = current_package(e, process);
            e.call(PACKAGE_TYPE, &args![current]).i32() != -1
        }
        && !e.call(PACKAGE_008A6290, &args![package]).bool();
    if runs {
        e.call(RESET_TARGET, &args![this]);
        let current = current_package(e, process);
        let location = e.call(PACKAGE_LOCATION, &args![current]).u32();
        let action = procedure_action(e, process);
        if action <= 0x33 {
            update_action(e, process, reference, package, location, action);
        }
        if e.call(CURRENT_PROCESS_TYPE, &args![reference]).u32() == 2
            && current_package(e, process) != 0
            && procedure_action(e, process) == 0x36
            && !update_finish_package(e, process, reference, &mut package)
        {
            return;
        }
    }
    if !e.call(IS_WAITING_ON_PATH, &args![reference]).bool() {
        update_acquire_object(e, actor);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0092_ce00, middle_low_process_get_actor_float_value(Ptr, u32, u32, u32) -> f64),
        entry!(0x0092_cd90, middle_low_process_get_actor_value(Ptr, u32, u32, u32) -> i32),
        entry!(
            0x0092_cec0,
            middle_low_process_temp_mod_actor_value_ov2(Ptr, u32, u32, f32)
        ),
        entry!(0x0092_cf10, middle_low_process_get_temporary_modifier(Ptr, u8) -> f64),
        entry!(0x0092_ea30, middle_low_process_save_game_ov2(Ptr, Ptr)),
        entry!(0x0092_ca70, fn_0092ca70(Ptr, Ptr)),
        entry!(0x0092_ca00, fn_0092ca00(Ptr)),
        entry!(0x0092_c9d0, middle_low_process_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0092_cfd0, middle_low_process_checkfor_new_package(Ptr, u32, u8) -> bool),
        entry!(0x0092_e400, middle_low_process_update_ov2(Ptr, u32, f32)),
        entry!(0x0092_d050, middle_low_process_update(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles: an actor value has flag 0x100 when its low byte is 0x80 or
    /// more; the low process answers 10 (float 10.5); the modifier list
    /// answers 2.25.
    fn modifier_doubles(e: &mut Engine) {
        e.register(ACTOR_VALUE_HAS_FLAG, |_, a| Ret {
            eax: (a[0] & 0xff >= 0x80 && a[1] == 0x100) as u32,
            ..Ret::default()
        });
        e.register(LOW_PROCESS_GET_ACTOR_FLOAT_VALUE, |_, _| Ret {
            st0: 10.5,
            ..Ret::default()
        });
        e.register(LOW_PROCESS_GET_ACTOR_VALUE, |_, _| Ret {
            eax: 10,
            ..Ret::default()
        });
        e.register(MODIFIER_LIST_GET_MODIFIER, |_, _| Ret {
            st0: 2.25,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn get_actor_float_value_adds_the_modifier_unless_flagged() {
        let mut e = Engine::new();
        modifier_doubles(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x0092_ce00, &args![0x6000u32, 0x7000u32, 0x105u32, 9u32]);
        assert_eq!(r.st0, 12.75);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(
            LOW_PROCESS_GET_ACTOR_FLOAT_VALUE,
            vec![0x6000, 0x7000, 0x105, 9]
        )));
        assert!(log.contains(&(MODIFIER_LIST_GET_MODIFIER, vec![0x60b8, 5])));
        let r = e.call(0x0092_ce00, &args![0x6000u32, 0x7000u32, 0x85u32, 9u32]);
        assert_eq!(r.st0, 10.5);
    }

    #[test]
    fn get_actor_value_adds_the_truncated_modifier_unless_flagged() {
        let mut e = Engine::new();
        modifier_doubles(&mut e);
        assert_eq!(
            e.call(0x0092_cd90, &args![0x6000u32, 0u32, 5u32, 0u32])
                .i32(),
            12
        );
        assert_eq!(
            e.call(0x0092_cd90, &args![0x6000u32, 0u32, 0x85u32, 0u32])
                .i32(),
            10
        );
    }

    #[test]
    fn temp_mod_actor_value_adjusts_unless_flagged() {
        let mut e = Engine::new();
        modifier_doubles(&mut e);
        e.register(MODIFIER_LIST_ADJUST_MODIFIER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0092_cec0, &args![0x6000u32, 0u32, 0x105u32, 1.5f32]);
        e.call(0x0092_cec0, &args![0x6000u32, 0u32, 0x85u32, 1.5f32]);
        let adjusted: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MODIFIER_LIST_ADJUST_MODIFIER)
            .collect();
        assert_eq!(
            adjusted,
            vec![(
                MODIFIER_LIST_ADJUST_MODIFIER,
                vec![0x60b8, 5, 1.5f32.to_bits(), 2]
            )]
        );
    }

    #[test]
    fn get_temporary_modifier_asks_the_list() {
        let mut e = Engine::new();
        modifier_doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0092_cf10, &args![0x6000u32, 0x1234u32]).st0, 2.25);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (MODIFIER_LIST_GET_MODIFIER, vec![0x60b8, 0x34])
        );
    }

    #[test]
    fn save_game_writes_the_hour_and_maybe_the_modifiers() {
        let mut e = Engine::new();
        e.register(LOW_PROCESS_SAVE_GAME, |_, _| Ret::default());
        e.register(SAVE_BUFFER_WRITE, |_, _| Ret::default());
        e.register(MODIFIER_LIST_SAVE_GAME, |_, _| Ret::default());
        // The buffer's change flags are its word at +0.
        e.register(SAVE_BUFFER_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], flags);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(CHANGE_FLAGS_HAS, |e, a| Ret {
            eax: (e.mem.u32(a[0]) & a[1] != 0) as u32,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0092_ea30, &args![0x6000u32, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LOW_PROCESS_SAVE_GAME, vec![0x6000, buffer]));
        assert_eq!(log[2], (SAVE_BUFFER_WRITE, vec![buffer, 0x60b4, 4, 0]));
        assert!(!log.iter().any(|c| c.0 == MODIFIER_LIST_SAVE_GAME));
        e.mem.set_u32(buffer, 0x0010_0000);
        e.call_log = Some(vec![]);
        e.call(0x0092_ea30, &args![0x6000u32, buffer]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(MODIFIER_LIST_SAVE_GAME, vec![0x60b8, buffer])));
    }

    #[test]
    fn fn_0092ca70_copies_the_hour_and_the_modifiers() {
        let mut e = Engine::new();
        e.register(MODIFIER_LIST_COPY, |_, _| Ret::default());
        let this = e.mem.alloc(0xc0);
        let other = e.mem.alloc(0xc0);
        e.mem.set_u32(other + 0xb4, 17);
        e.call_log = Some(vec![]);
        e.call(0x0092_ca70, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0xb4), 17);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (MODIFIER_LIST_COPY, vec![this + 0xb8, other + 0xb8])
        );
    }

    #[test]
    fn destructors_tear_down_and_free_on_bit_0() {
        let mut e = Engine::new();
        e.register(MODIFIER_LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LOW_PROCESS_DESTRUCT, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x0092_ca00, &args![this]);
        assert_eq!(e.mem.u32(this), MIDDLE_LOW_PROCESS_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MODIFIER_LIST_DESTRUCT, vec![this + 0xb8]));
        assert_eq!(log[2], (LOW_PROCESS_DESTRUCT, vec![this]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0092_c9d0, &args![this, 0u32]).u32(), this);
        assert!(!e.call_log.take().unwrap().iter().any(|c| c.0 == FREE_BLOCK));
        e.call_log = Some(vec![]);
        e.call(0x0092_c9d0, &args![this, 1u32]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(FREE_BLOCK, vec![this])));
    }

    #[test]
    fn checkfor_new_package_skips_within_the_same_hour() {
        let mut e = Engine::new();
        e.register(CALENDAR_GET_HOUR, |_, _| Ret {
            st0: 7.75,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        e.register(LOW_PROCESS_CHECK_FOR_NEW_PACKAGE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        // vtable +0x22C answers the word at +0x10.
        e.put_vtable(0x9100_0000, &[0; 0x8c]);
        e.mem.set_u32(0x9100_022c, 0x7000_022c);
        e.register(0x7000_022c, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this, 0x9100_0000);
        assert!(
            e.call(0x0092_cfd0, &args![this, 0u32, 0u32]).bool(),
            "no package"
        );
        assert_eq!(e.mem.u32(this + 0xb4), 7);
        e.mem.set_u32(this + 0x10, 1);
        assert!(
            !e.call(0x0092_cfd0, &args![this, 0u32, 0u32]).bool(),
            "same hour"
        );
        assert!(
            e.call(0x0092_cfd0, &args![this, 0u32, 1u32]).bool(),
            "forced"
        );
        e.mem.set_u32(this + 0xb4, 6);
        assert!(
            e.call(0x0092_cfd0, &args![this, 0u32, 0u32]).bool(),
            "new hour"
        );
        assert_eq!(e.mem.u32(this + 0xb4), 7);
    }

    #[test]
    fn update_ov2_passes_on() {
        let mut e = Engine::new();
        e.register(LOW_PROCESS_UPDATE_OV2, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0092_e400, &args![0x6000u32, 0x7000u32, 0.5f32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (
                LOW_PROCESS_UPDATE_OV2,
                vec![0x6000, 0x7000, 0.5f32.to_bits()]
            )
        );
    }

    /// A rig for `Update`: the process, actor and package objects have
    /// vtables whose every slot is a double that answers the word stored for
    /// that slot (process answers at `PROCESS_ANSWERS + slot`, actor at
    /// `ACTOR_ANSWERS + slot`, package at `PACKAGE_ANSWERS + slot`).
    const PROCESS_ANSWERS: u32 = 0x9300_0000;
    const ACTOR_ANSWERS: u32 = 0x9301_0000;
    const PACKAGE_ANSWERS: u32 = 0x9302_0000;
    const ACQUIRE_ANSWERS: u32 = 0x9303_0000;

    struct Rig {
        process: u32,
        actor: u32,
        package: u32,
        acquire: u32,
    }

    fn object_with_answers(e: &mut Engine, vtable: u32, answers: u32, code: u32) -> u32 {
        let slots = 0x210u32;
        e.map(vtable, slots * 4);
        e.map(answers, slots * 4);
        for slot in 0..slots {
            e.mem.set_u32(vtable + 4 * slot, code + 4 * slot);
        }
        for slot in 0..slots {
            let at = answers + 4 * slot;
            e.register_double(code + 4 * slot, move |e, _| Ret {
                eax: e.mem.u32(at),
                ..Ret::default()
            });
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig(e: &mut Engine) -> Rig {
        let process = object_with_answers(e, 0x9400_0000, PROCESS_ANSWERS, 0x7400_0000);
        let actor = object_with_answers(e, 0x9401_0000, ACTOR_ANSWERS, 0x7401_0000);
        let package = object_with_answers(e, 0x9402_0000, PACKAGE_ANSWERS, 0x7402_0000);
        let acquire = object_with_answers(e, 0x9403_0000, ACQUIRE_ANSWERS, 0x7403_0000);
        // The actor is an actor; the process's package is `package`.
        e.mem.set_u32(ACTOR_ANSWERS + 0x100, 1);
        e.mem.set_u32(PROCESS_ANSWERS + 0x22c, package);
        for (addr, eax) in [
            (EMPTY_BODY, 0u32),
            (ACTOR_008A8F60, 0),
            (PACKAGE_008A6290, 0),
            (RESET_TARGET, 0),
            (PACKAGE_LOCATION, 0),
            (CURRENT_PROCESS_TYPE, 1),
            (IS_WAITING_ON_PATH, 0),
            (GET_SAVED_ACQUIRE_OBJECT, acquire),
        ] {
            e.register_double(addr, move |_, _| Ret {
                eax,
                ..Ret::default()
            });
        }
        e.register(CALENDAR_GET_HOUR, |_, _| Ret {
            st0: 8.0,
            ..Ret::default()
        });
        // Package type 3; the action table row 3 answers by procedure
        // index (process vtable +0x280).
        e.register(PACKAGE_TYPE, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.map(PROCEDURE_ACTIONS, 0x40);
        let row = e.mem.alloc(0x10);
        e.set_global(PROCEDURE_ACTIONS + 3 * 4, row);
        Rig {
            process,
            actor,
            package,
            acquire,
        }
    }

    /// Sets the action the table gives (procedure index 1 of row 3).
    fn set_action(e: &mut Engine, action: u32) {
        let row = e.global::<u32>(PROCEDURE_ACTIONS + 3 * 4);
        e.mem.set_u32(row + 4, action);
        e.mem.set_u32(PROCESS_ANSWERS + 0x280, 1);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn process_slot(slot: u32) -> u32 {
        0x7400_0000 + slot
    }

    #[test]
    fn update_runs_only_the_acquire_object_for_a_non_actor() {
        let mut e = Engine::new();
        let r = rig(&mut e);
        e.mem.set_u32(ACTOR_ANSWERS + 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(calls(&e, 0x7403_0028), vec![vec![r.acquire]]);
        assert!(calls(&e, process_slot(0x24)).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2, "only the empty body");
    }

    #[test]
    fn update_reevaluates_and_runs_a_single_slot_action() {
        let mut e = Engine::new();
        let r = rig(&mut e);
        e.mem.set_u32(PROCESS_ANSWERS + 0x24, 1);
        set_action(&mut e, 7);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(
            calls(&e, process_slot(0x524)),
            vec![vec![r.process, r.actor]]
        );
        assert_eq!(
            calls(&e, process_slot(0x90)),
            vec![vec![r.process, r.actor, 0]]
        );
        assert_eq!(calls(&e, RESET_TARGET), vec![vec![r.process]]);
        assert_eq!(
            calls(&e, process_slot(0x7b4)),
            vec![vec![r.process, r.actor]]
        );
        assert_eq!(calls(&e, 0x7403_0028).len(), 1, "acquire object last");

        for (action, slot, extra) in [
            (0x29u32, 0x288u32, vec![2u32]),
            (0xe, 0x7c8, vec![0]),
            (2, 0x7c8, vec![1]),
            (6, 0x298, vec![1, 0x101, 0]),
            (0xf, 0x29c, vec![0, 1, 0x101]),
            (0x33, 0x7f0, vec![]),
        ] {
            set_action(&mut e, action);
            e.call_log = Some(vec![]);
            e.call(0x0092_d050, &args![r.process, r.actor]);
            let mut expected = vec![r.process, r.actor];
            expected.extend(extra);
            assert_eq!(calls(&e, process_slot(slot)), vec![expected], "{action:#x}");
        }
        // An action above 0x33 runs nothing; no package skips everything.
        set_action(&mut e, 0x34);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert!(calls(&e, process_slot(0x288)).is_empty());
        e.mem.set_u32(PROCESS_ANSWERS + 0x22c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert!(calls(&e, RESET_TARGET).is_empty());
    }

    #[test]
    fn update_travel_sets_the_pathfinding_goal() {
        let mut e = Engine::new();
        let r = rig(&mut e);
        set_action(&mut e, 0);
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e.map(HOURS_PER_DAY, 8);
        e.set_global(HOURS_PER_DAY, 24.0f64);
        e.map(SECONDS_PER_HOUR, 8);
        e.set_global(SECONDS_PER_HOUR, 3600.0f64);
        e.register(IS_PATHING_COMPLETE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(FORM_BY_INDEX, |_, _| Ret {
            eax: 0x4400,
            ..Ret::default()
        });
        e.register(PROCESS_HOUR, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: 30.0,
            ..Ret::default()
        });
        e.register(PACKAGE_RADIUS, |_, _| Ret {
            st0: 128.0,
            ..Ret::default()
        });
        e.register(PACKAGE_PROCEDURE, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_WORLD, |_, _| Ret {
            eax: 0x5100,
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_CELL, |_, _| Ret {
            eax: 0x5200,
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_COORD, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(SET_PATHFINDING_GOAL, |_, _| Ret::default());
        e.register(ACQUIRE_STATE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        let goal = calls(&e, SET_PATHFINDING_GOAL);
        assert_eq!(goal.len(), 1);
        assert_eq!(goal[0][0], r.actor);
        assert_eq!(
            goal[0][2..].to_vec(),
            vec![0x5200, 0x5100, 128.0f32.to_bits(), 0]
        );
        let tested = calls(&e, 0x7402_013c);
        assert_eq!(
            tested[0],
            vec![r.package, r.actor, 0, (-1.0f32).to_bits(), 0]
        );
        assert!(
            calls(&e, process_slot(0x288)).is_empty(),
            "not at the location"
        );

        // At the location (vtable +0x13C true) with the acquire state 2:
        // no new goal, faces the target's heading and reports done.
        e.mem.set_u32(PACKAGE_ANSWERS + 0x13c, 1);
        e.register(ACQUIRE_STATE, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.mem.set_u32(PROCESS_ANSWERS + 0x1bc, 1);
        e.register(PACKAGE_FLAG_00670F60, |_, _| Ret::default());
        let location = e.mem.alloc(4);
        e.register_double(PACKAGE_LOCATION, move |_, _| Ret {
            eax: location,
            ..Ret::default()
        });
        let target = e.mem.alloc(0x10);
        e.register_double(LOCATION_GET_REFERENCE, move |_, _| Ret {
            eax: target,
            ..Ret::default()
        });
        e.register(TARGET_FORM, |_, _| Ret {
            eax: 0x0abc,
            ..Ret::default()
        });
        e.map(TARGET_FORM_GLOBAL, 4);
        e.set_global(TARGET_FORM_GLOBAL, 0x0abcu32);
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point + 8, 1.25);
        e.register_double(TARGET_POINT, move |_, _| Ret {
            eax: point,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert!(calls(&e, SET_PATHFINDING_GOAL).is_empty());
        assert_eq!(
            calls(&e, 0x7401_02c4),
            vec![vec![r.actor, 1.25f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, process_slot(0x288)),
            vec![vec![r.process, r.actor, 1]]
        );
    }

    #[test]
    fn update_reach_object_compares_the_distance() {
        let mut e = Engine::new();
        let r = rig(&mut e);
        set_action(&mut e, 0xd);
        let object = e.mem.alloc(8);
        e.mem.set_u32(r.process + 0x40, object);
        e.register(OBJECT_00440D80, |_, _| Ret::default());
        e.register(PACKAGE_PROCEDURE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(PACKAGE_DATA, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        e.register(WORD_AT_8, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.register(DISTANCE_FROM_REFERENCE, |e, a| Ret {
            st0: e.mem.f32(a[1] + 4) as f64,
            ..Ret::default()
        });
        e.mem.set_f32(object + 4, 150.0);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert!(calls(&e, process_slot(0x288)).is_empty(), "too far");
        e.mem.set_f32(object + 4, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(
            calls(&e, process_slot(0x288)),
            vec![vec![r.process, r.actor, 1]]
        );
        // No object: found through vtable +0x8C, and done at once.
        e.mem.set_u32(r.process + 0x40, 0);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(
            calls(&e, process_slot(0x8c)),
            vec![vec![r.process, r.actor]]
        );
        assert_eq!(calls(&e, process_slot(0x288)).len(), 1);
    }

    #[test]
    fn update_finishes_a_done_package() {
        let mut e = Engine::new();
        let r = rig(&mut e);
        set_action(&mut e, 0x36);
        e.register(CURRENT_PROCESS_TYPE, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        for addr in [
            EXTRA_LIST,
            SET_ACTION_FLAG,
            PACKAGE_ONCE_PER_DAY,
            PACKAGE_IS_CREATED,
            DELETE_OBJECT,
            LIST_REMOVE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(GET_TARGET_REFR, |_, _| Ret::default());
        e.register(SAME_POINTER, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // Two items in the list (its head word at +0x5c), then empty.
        let mut remaining = 2u32;
        e.register_double(LIST_IS_EMPTY, move |e, a| {
            let empty = remaining == 0;
            if !empty {
                e.mem.set_u32(a[0], 0x7700 + remaining);
                remaining -= 1;
            }
            Ret {
                eax: empty as u32,
                ..Ret::default()
            }
        });
        e.register(PACKAGE_0041D8A0, |_, _| Ret::default());
        e.register(WORD_AT_4, |_, _| Ret::default());
        e.mem.set_u32(r.process + 0x64, 0x7600);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(calls(&e, process_slot(0x5a8)), vec![vec![r.process, 1]]);
        assert_eq!(calls(&e, process_slot(0x1c8)), vec![vec![r.process, 0]]);
        assert_eq!(
            calls(&e, DELETE_OBJECT),
            vec![vec![0x7600, 1], vec![0x7702, 1], vec![0x7701, 1]]
        );
        assert_eq!(e.mem.u32(r.process + 0x64), 0);
        assert_eq!(
            calls(&e, process_slot(0x24)).last().unwrap(),
            &vec![r.process, r.actor, 1]
        );
        assert!(calls(&e, process_slot(0x284)).is_empty(), "same package");
        assert_eq!(calls(&e, 0x7403_0028).len(), 1);

        // A package of type 0 that turns the actor away ends `Update`.
        e.register(PACKAGE_TYPE, |_, _| Ret::default());
        e.map(PROCEDURE_ACTIONS, 0x40);
        let row = e.global::<u32>(PROCEDURE_ACTIONS + 3 * 4);
        e.set_global(PROCEDURE_ACTIONS, row);
        e.map(MINUS_ONE, 4);
        e.call_log = Some(vec![]);
        e.call(0x0092_d050, &args![r.process, r.actor]);
        assert_eq!(
            calls(&e, process_slot(0x288)),
            vec![vec![r.process, r.actor, 0xffff_ffff]]
        );
        assert!(calls(&e, 0x7403_0028).is_empty());
    }
}
