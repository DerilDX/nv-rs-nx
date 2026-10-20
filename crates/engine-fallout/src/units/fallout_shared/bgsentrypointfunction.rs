//! `fallout shared/bgsentrypointfunction.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e64e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e6490` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e64e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_6490, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice` scalar deleting destructor (engine map name): runs the destructor body `005e6600` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bgs_entry_point_function_data_activate_choice_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_6600, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `005b5e40`: the warning printer (cdecl, format and its arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// `00401000`: `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030`: `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00401660`: the current chunk's size (`ECX` = the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680`: whether the file's data needs its byte order converted.
const NEEDS_BYTE_SWAP: u32 = 0x0040_1680;
/// `00401080`: byte-swaps the word at the address in place (cdecl; the
/// callers pass 0 as the second word).
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;
/// `TESFile::GetTESChunk` (`004726b0`, engine map).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (`004727f0`, engine map): one word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (`00472840`, engine map): one stack word.
const GET_CHUNK_DATA_OV2: u32 = 0x0047_2840;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size
/// (0: the whole chunk).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `0084e3a0`: the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The number of function arguments of each entry point function type
/// (`uint32[]`, read by `CheckFunctionArguments`).
const ARGUMENT_COUNTS: u32 = 0x0119_7400;
/// `"DEFAULT: Invalid function type %i passed to BGSEntryPointFunction::CheckFunctionArguments.  Expected %i."`.
const BAD_TYPE_WARNING: u32 = 0x0104_6250;
/// `"DEFAULT: Invalid argument count %i passed to ...  Expected %i."`.
const BAD_COUNT_WARNING: u32 = 0x0104_61e0;
/// `"DEFAULT: Invalid function argument %i passed to ...  Function arguments cannot be zero."`.
const ZERO_ARGUMENT_WARNING: u32 = 0x0104_6158;
/// `"DEFAULT: Invalid function data type %i passed to ...  Expected %i."`.
const BAD_DATA_TYPE_WARNING: u32 = 0x0104_60e8;
/// `"MASTERFILE: Unable to find leveled list (%08X) on owner object \"%s\"."`.
const NO_LEVELED_LIST_NAMED: u32 = 0x0104_63a8;
/// `"MASTERFILE: Unable to find leveled list (%08X) on owner object (%08X)."`.
const NO_LEVELED_LIST_ID: u32 = 0x0104_6360;
/// `"MASTERFILE: Errors were encountered during InitItem for result script for perk '%s' (%08X)..."`.
const RESULT_SCRIPT_ERRORS: u32 = 0x0104_63f0;
/// `BGSEntryPointFunctionData`'s vtable (RTTI).
const FUNCTION_DATA_VTABLE: u32 = 0x0104_62e0;
/// `BGSEntryPointFunctionDataOneValue`'s vtable (RTTI).
const ONE_VALUE_VTABLE: u32 = 0x0104_62c0;
/// `BGSEntryPointFunctionDataLeveledList`'s vtable (RTTI).
const LEVELED_LIST_VTABLE: u32 = 0x0104_6320;
/// `BGSEntryPointFunctionDataActivateChoice`'s vtable (RTTI).
const ACTIVATE_CHOICE_VTABLE: u32 = 0x0104_6340;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TESLevItem`'s RTTI type descriptor (`.?AVTESLevItem@@`): the target of
/// the leveled-list cast.
const TYPE_LEVELED_LIST_TARGET: u32 = 0x0118_6f7c;
/// The script variable `ActivateChoice::Load` is filling between its
/// `SLSD` and `SCVR` chunks (`ScriptVariable*`).
const CURRENT_VARIABLE: u32 = 0x011c_b0fc;

const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_EPF2: u32 = 0x3246_5045;
const CHUNK_EPF3: u32 = 0x3346_5045;

// Translated from 005e5b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunction::CheckFunctionArguments` (Xbox PDB), cdecl, seven
/// words: whether `function_type` is `expected_type`, `argument_count`
/// (low byte) is the type's count (`01197400[expected_type]`), none of the
/// `argument_count` arguments at `arguments` is 0, and `data` (when not
/// null) reports through vtable slot 1 the data type `expected_data_type`
/// (low byte). Each failure prints its warning and returns false. The third
/// word is never read.
pub fn bgs_entry_point_function_check_function_arguments(
    e: &mut Engine,
    expected_type: u32,
    expected_data_type: u8,
    _unused_3: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) -> bool {
    if expected_type != function_type {
        e.call(
            PRINT_WARNING,
            &args![BAD_TYPE_WARNING, function_type, expected_type],
        );
        return false;
    }
    let count_slot = ARGUMENT_COUNTS.wrapping_add(expected_type.wrapping_mul(4));
    let expected_count = e.mem.u32(count_slot);
    if expected_count != u32::from(argument_count) {
        e.call(
            PRINT_WARNING,
            &args![BAD_COUNT_WARNING, u32::from(argument_count), expected_count],
        );
        return false;
    }
    for index in 0..u32::from(argument_count) {
        if e.mem.u32(arguments.addr().wrapping_add(index * 4)) == 0 {
            e.call(PRINT_WARNING, &args![ZERO_ARGUMENT_WARNING, index]);
            return false;
        }
    }
    if !data.is_null() {
        let data_type = e.vcall(data.addr(), 0x4, &args![]).u32();
        if data_type != u32::from(expected_data_type) {
            let reported = e.vcall(data.addr(), 0x4, &args![]).u32();
            e.call(
                PRINT_WARNING,
                &args![
                    BAD_DATA_TYPE_WARNING,
                    reported,
                    u32::from(expected_data_type)
                ],
            );
            return false;
        }
    }
    true
}

// Translated from 005e5d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a one-value data (type 1), cdecl,
/// five words: when `CheckFunctionArguments(0, 1, ...)` passes, the `float`
/// the first argument points at is multiplied by the data's value
/// (`006b9130`, in `ST0`): `FMUL dword`, `FSTP dword`.
pub fn fn_005e5d20(
    e: &mut Engine,
    first: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        1,
        first,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let value = e.call(0x006b_9130, &args![data]).f64();
    let product = x87_mul(value, x87_m32(e.mem.u32(target)));
    e.mem.set_u32(target, x87_store32(product));
}

/// A function-data constructor (`ECX` = the block; returns it).
type DataConstructor = fn(&mut Engine, Ptr) -> Ptr;

// Translated from 005e6280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6280 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Creates the function data object for a data type (cdecl, one word):
/// 1 a one-value data (8 bytes, [`fn_005e63f0`]), 2 a two-value data (12
/// bytes, `005e64b0`, vtable `01046300`, RTTI
/// `BGSEntryPointFunctionDataTwoValue`), 3 a leveled-list data (8 bytes, [`fn_005e6510`]), 4 an
/// activate-choice data (0x60 bytes, [`fn_005e6540`]); null for any other
/// type or when the allocation fails.
pub fn fn_005e6280(e: &mut Engine, data_type: u32) -> Ptr {
    let (size, construct): (u32, Option<DataConstructor>) = match data_type.wrapping_sub(1) {
        0 => (8, Some(fn_005e63f0)),
        1 => (0xc, None),
        2 => (8, Some(fn_005e6510)),
        3 => (0x60, Some(fn_005e6540)),
        _ => return Ptr::NULL,
    };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    match construct {
        Some(construct) => construct(e, Ptr::new(block)),
        None => e.call(0x005e_64b0, &args![block]).ptr(),
    }
}

// Translated from 005e63f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue` constructor (its vtable `010462c0`,
/// by RTTI): the base ([`fn_005e6420`]), the vtable, the value +0x4 = 0.0
/// (`FLDZ`). Returns `this`.
pub fn fn_005e63f0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), ONE_VALUE_VTABLE);
    e.mem.set_f32(this.addr() + 4, 0.0);
    this
}

// Translated from 005e6420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionData` constructor (its vtable `010462e0`, by
/// RTTI): stores the vtable. Returns `this`.
pub fn fn_005e6420(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FUNCTION_DATA_VTABLE);
    this
}

// Translated from 005e6470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionData` destructor body: stores its vtable back.
pub fn fn_005e6470(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FUNCTION_DATA_VTABLE);
}

// Translated from 005e6490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body that only runs the `BGSEntryPointFunctionData` one
/// ([`fn_005e6470`]).
pub fn fn_005e6490(e: &mut Engine, this: Ptr) {
    fn_005e6470(e, this);
}

// Translated from 005e6510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList` constructor (its vtable
/// `01046320`, by RTTI): the base, the vtable, the list +0x4 = null.
/// Returns `this`.
pub fn fn_005e6510(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), LEVELED_LIST_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 005e6540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6540 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataActivateChoice` constructor (its vtable
/// `01046340`, by RTTI): the base, the vtable, the label +0x4 = null, the
/// result script at +0x8 (`Script::Script`, `005aa0f0`), the word +0x5C =
/// 0, then [`fn_005e6c30`]. Returns `this`.
pub fn fn_005e6540(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), ACTIVATE_CHOICE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(0x005a_a0f0, &args![this.addr() + 8]);
    e.mem.set_u16(this.addr() + 0x5c, 0);
    fn_005e6c30(e, this);
    this
}

// Translated from 005e6600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6600 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataActivateChoice` destructor body: the vtable
/// `01046340`, the label freed ([`bgs_entry_point_function_data_activate_choice_set_label`]
/// with null), the script's destructor `005aa1a0` (+0x8), then the base
/// one ([`fn_005e6470`]).
pub fn fn_005e6600(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTIVATE_CHOICE_VTABLE);
    bgs_entry_point_function_data_activate_choice_set_label(e, this, Ptr::NULL);
    e.call(0x005a_a1a0, &args![this.addr() + 8]);
    fn_005e6470(e, this);
}

// Translated from 005e6790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue::Load` (Xbox PDB): false unless the
/// chunk of `file` (one stack word) is 4 bytes; reads it into +0x4,
/// byte-swapped when the file needs it. Returns true.
pub fn bgs_entry_point_function_data_one_value_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(CHUNK_SIZE, &args![file]).u32() != 4 {
        return false;
    }
    e.call(GET_CHUNK_DATA, &args![file, this.addr() + 4, 0u32]);
    if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
        e.call(SWAP_WORD_IN_PLACE, &args![this.addr() + 4, 0u32]);
    }
    true
}

// Translated from 005e6b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::Load` (Xbox PDB): false unless the
/// chunk is 4 bytes; reads the form id into a stack word (byte-swapped when
/// the file needs it) and stores it at +0x4. Returns true.
pub fn bgs_entry_point_function_data_leveled_list_load(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
) -> bool {
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
    e.mem.set_u32(this.addr() + 4, id);
    true
}

// Translated from 005e6b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::InitItem` (Xbox PDB): resolves
/// the form id at +0x4 (when not 0) against `owner`'s file
/// (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), looks it up
/// (`004839c0`) and casts it from `TESForm` to the type at `01186f7c`
/// (`__RTDynamicCast`) into +0x4. When that gives null it warns with the
/// id and the owner's editor id (slot 0x130) when `00474cb0` reports one,
/// else with the owner's form id.
pub fn bgs_entry_point_function_data_leveled_list_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    if e.mem.u32(this.addr() + 4) == 0 {
        return;
    }
    let id = e.with_stack(4, |e, local| {
        let id = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(local.addr(), id);
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![local, file]);
        e.mem.u32(local.addr())
    });
    let found = e.call(0x0048_39c0, &args![id]).u32();
    let list = e
        .call(
            0x00ec_43fb,
            &args![found, 0u32, TYPE_TES_FORM, TYPE_LEVELED_LIST_TARGET, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + 4, list);
    if list != 0 {
        return;
    }
    if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
        let editor_id = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(PRINT_WARNING, &args![NO_LEVELED_LIST_NAMED, id, editor_id]);
    } else {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        e.call(PRINT_WARNING, &args![NO_LEVELED_LIST_ID, id, owner_id]);
    }
}

// Translated from 005e6c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +0x5C and prepares the result script at +0x8:
/// `00484ab0(0)` and `TESForm::SetTemporary` (`00484490`, engine map).
pub fn fn_005e6c30(e: &mut Engine, this: Ptr) {
    e.mem.set_u16(this.addr() + 0x5c, 0);
    e.call(0x0048_4ab0, &args![this.addr() + 8, 0u32]);
    e.call(0x0048_4490, &args![this.addr() + 8]);
}

// Translated from 005e6c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::SetLabel` (Xbox PDB): frees
/// the label at +0x4 and clears it; with a `label` (one stack word), stores
/// a new copy of it (`00401000` of its length + 1, then `004046f0`).
pub fn bgs_entry_point_function_data_activate_choice_set_label(
    e: &mut Engine,
    this: Ptr,
    label: Ptr,
) {
    let old = e.mem.u32(this.addr() + 4);
    e.call(OPERATOR_DELETE, &args![old]);
    e.mem.set_u32(this.addr() + 4, 0);
    if label.is_null() {
        return;
    }
    let length = e.call(0x0044_a670, &args![label]).u32();
    let copy = e.call(OPERATOR_NEW, &args![length.wrapping_add(1)]).u32();
    e.mem.set_u32(this.addr() + 4, copy);
    e.call(0x0040_46f0, &args![copy, label]);
}

// Translated from 005e6ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6ee0 - the exception-unwinding frame (FS:[0]) and the /GS security-cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size rounded up to 16 bytes.
/// `BGSEntryPointFunctionDataActivateChoice::Load` (Xbox PDB): reads the
/// current chunk of `file` (one stack word) into the result script at
/// +0x8 or the label, and returns true:
/// - `SCRO`/`SCRV`: a new referenced-object entry (0x10 bytes,
///   `00500910`) gets the form id at +0x8 (`SCRO`) or the variable index at
///   +0xC (`SCRV`) and goes onto the script's reference list (`005d43c0`,
///   `00905820`);
/// - `SLSD`: a new script variable (0x20 bytes, `005e28b0`) becomes the
///   current one (`011cb0fc`), loads (`ScriptVariable::Load`) and goes onto
///   the script's variable list (`005d43e0`, `00905820`); `SCVR` (when the
///   chunk is not also `SLSD`) names the current variable (+0x18,
///   `004037f0`, at most 0x200 bytes read) when there is one;
/// - `SCHR`: the script header (`00500940`), byte-swapped (`00414220`)
///   when the file needs it; `SCDA`: the compiled data, zeroed then read
///   into a stack buffer, given to `Script::SetCompileData`;
/// - `EPF2`: the label ([`bgs_entry_point_function_data_activate_choice_set_label`]);
///   `EPF3`: one word into +0x5C (`GetChunkData_ov2`).
pub fn bgs_entry_point_function_data_activate_choice_load(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
) -> bool {
    let script = this.addr() + 8;
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    match chunk {
        CHUNK_SCRO | CHUNK_SCRV => {
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let entry = if block == 0 {
                0
            } else {
                e.call(0x0050_0910, &args![block]).u32()
            };
            e.with_stack(8, |e, locals| {
                // `locals + 0`: the entry pointer; `+4`: the word read.
                e.mem.set_u32(locals.addr(), entry);
                e.call(GET_CHUNK_WORD, &args![file, locals.addr() + 4]);
                let value = e.mem.u32(locals.addr() + 4);
                let field = if chunk == CHUNK_SCRO { 8 } else { 0xc };
                e.mem.set_u32(entry.wrapping_add(field), value);
                let list = e.call(0x005d_43c0, &args![script]).u32();
                e.call(0x0090_5820, &args![list, locals]);
            });
        }
        CHUNK_SLSD | CHUNK_SCVR => {
            if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_SLSD {
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let variable = if block == 0 {
                    0
                } else {
                    e.call(0x005e_28b0, &args![block]).u32()
                };
                e.mem.set_u32(CURRENT_VARIABLE, variable);
                e.call(0x005e_2a20, &args![variable, file]);
                let list = e.call(0x005d_43e0, &args![script]).u32();
                e.call(0x0090_5820, &args![list, CURRENT_VARIABLE]);
            } else if e.mem.u32(CURRENT_VARIABLE) != 0 {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    let variable = e.mem.u32(CURRENT_VARIABLE);
                    e.call(0x0040_37f0, &args![variable + 0x18, buffer, 0u32]);
                });
            }
        }
        CHUNK_SCHR => {
            let header = e.call(0x0050_0940, &args![script]).u32();
            e.call(GET_CHUNK_DATA, &args![file, header, 0u32]);
            if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
                let header = e.call(0x0050_0940, &args![script]).u32();
                e.call(0x0041_4220, &args![header]);
            }
        }
        CHUNK_SCDA => {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            let reserved = size.wrapping_add(15) & !15;
            e.with_stack(reserved, |e, buffer| {
                e.call(0x0040_3d30, &args![buffer, 0u32, size]);
                e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                e.call(0x005a_ae70, &args![script, size, buffer]);
            });
        }
        CHUNK_EPF2 => {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            let reserved = size.wrapping_add(15) & !15;
            e.with_stack(reserved, |e, buffer| {
                e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                bgs_entry_point_function_data_activate_choice_set_label(e, this, buffer);
            });
        }
        CHUNK_EPF3 => {
            e.call(GET_CHUNK_DATA_OV2, &args![file, this.addr() + 0x5c]);
        }
        _ => {}
    }
    true
}

// Translated from 005e7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::InitItem` (Xbox PDB): resolves
/// the result script (`Script::InitItem`, `005ab400`, with `owner`) with
/// warnings off (`MessageHandler::IncDisableWarningCount` 1, then 0) and the
/// thread's error count (`0046e8a0`/`004fffe0`) cleared around it, then
/// restored; when that count is not 0 afterwards it warns with the owner's
/// editor id (slot 0x130) and form id.
pub fn bgs_entry_point_function_data_activate_choice_init_item(
    e: &mut Engine,
    this: Ptr,
    owner: Ptr,
) {
    let saved = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x0043_b2b0, &args![1u32]);
    e.call(0x004f_ffe0, &args![0u32]);
    e.call(0x005a_b400, &args![this.addr() + 8, owner]);
    e.call(0x0043_b2b0, &args![0u32]);
    let errors = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x004f_ffe0, &args![saved]);
    if errors != 0 {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        let editor_id = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(
            PRINT_WARNING,
            &args![RESULT_SCRIPT_ERRORS, editor_id, owner_id],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_64e0, fn_005e64e0(Ptr, u32) -> Ptr),
        entry!(0x005e_65d0, bgs_entry_point_function_data_activate_choice_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x005e_5b80,
            bgs_entry_point_function_check_function_arguments(u32, u8, u32, u32, u8, Ptr, Ptr)
                -> bool
        ),
        entry!(0x005e_5d20, fn_005e5d20(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_6280, fn_005e6280(u32) -> Ptr),
        entry!(0x005e_63f0, fn_005e63f0(Ptr) -> Ptr),
        entry!(0x005e_6420, fn_005e6420(Ptr) -> Ptr),
        entry!(0x005e_6470, fn_005e6470(Ptr)),
        entry!(0x005e_6490, fn_005e6490(Ptr)),
        entry!(0x005e_6510, fn_005e6510(Ptr) -> Ptr),
        entry!(0x005e_6540, fn_005e6540(Ptr) -> Ptr),
        entry!(0x005e_6600, fn_005e6600(Ptr)),
        entry!(
            0x005e_6790,
            bgs_entry_point_function_data_one_value_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6b00,
            bgs_entry_point_function_data_leveled_list_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6b70,
            bgs_entry_point_function_data_leveled_list_init_item(Ptr, Ptr)
        ),
        entry!(0x005e_6c30, fn_005e6c30(Ptr)),
        entry!(
            0x005e_6c60,
            bgs_entry_point_function_data_activate_choice_set_label(Ptr, Ptr)
        ),
        entry!(
            0x005e_6ee0,
            bgs_entry_point_function_data_activate_choice_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_7230,
            bgs_entry_point_function_data_activate_choice_init_item(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_005e64e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6490, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_64e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_64e0)
                .collect();
            let mut expected = vec![(0x005e_6490, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e65d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6600, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_65d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_65d0)
                .collect();
            let mut expected = vec![(0x005e_6600, vec![this.addr()])];
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

    /// An engine with the argument-count table (type 0: 1, type 1: 1,
    /// type 2: 2) and a data object whose slot 1 returns the word at +4.
    fn check_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0119_7000, 0x1000);
        e.mem.set_u32(ARGUMENT_COUNTS, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 4, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 8, 2);
        quiet(&mut e, &[PRINT_WARNING]);
        e.put_vtable(0x0200_0000, &[0, 0x0090_0000]);
        e.register(0x0090_0000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0200_0000);
        e.mem.set_u32(data + 4, 1);
        (e, data)
    }

    #[test]
    fn check_function_arguments_tests_each_condition() {
        let (mut e, data) = check_engine();
        let arguments = e.mem.alloc(8);
        e.mem.set_u32(arguments, 0x40);
        e.mem.set_u32(arguments + 4, 0x50);
        let check = |e: &mut Engine, words: [u32; 7]| e.call(0x005e_5b80, &words).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e, [2, 1, 9, 2, 2, arguments, data]));
        assert!(check(&mut e, [2, 7, 9, 2, 2, arguments, 0]));
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != PRINT_WARNING));

        e.call_log = Some(vec![]);
        assert!(!check(&mut e, [2, 1, 9, 1, 2, arguments, data]));
        assert!(!check(&mut e, [2, 1, 9, 2, 0x103, arguments, data]));
        e.mem.set_u32(arguments + 4, 0);
        assert!(!check(&mut e, [2, 1, 9, 2, 2, arguments, data]));
        e.mem.set_u32(arguments + 4, 0x50);
        assert!(!check(&mut e, [2, 0x102, 9, 2, 2, arguments, data]));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![BAD_TYPE_WARNING, 1, 2],
                vec![BAD_COUNT_WARNING, 3, 2],
                vec![ZERO_ARGUMENT_WARNING, 1],
                vec![BAD_DATA_TYPE_WARNING, 1, 2],
            ]
        );
    }

    #[test]
    fn fn_005e5d20_multiplies_the_target_by_the_data_value() {
        let (mut e, data) = check_engine();
        e.register(0x006b_9130, |_, _| 2.5f64.into_ret());
        let target = e.mem.alloc(4);
        e.mem.set_f32(target, 3.0);
        let arguments = e.mem.alloc(4);
        e.mem.set_u32(arguments, target);
        e.call(0x005e_5d20, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 7.5);
        // A failed check (wrong type) leaves it.
        e.call(0x005e_5d20, &args![9u32, 1u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 7.5);
    }

    #[test]
    fn the_data_constructors_and_destructors_store_their_vtables() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x005a_a0f0,
                0x005a_a1a0,
                0x0048_4ab0,
                0x0048_4490,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0xcc; 0x60]);
        assert_eq!(e.call(0x005e_6420, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        assert_eq!(e.call(0x005e_63f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ONE_VALUE_VTABLE);
        assert_eq!(e.mem.f32(this + 4), 0.0);
        e.mem.set_u32(this + 4, 0xcccc_cccc);
        assert_eq!(e.call(0x005e_6510, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LEVELED_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x005e_6470, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x005e_6490, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);

        e.mem.write(this, &[0xcc; 0x60]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_6540, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ACTIVATE_CHOICE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0x5c), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x005a_a0f0, vec![this + 8]),
                (0x0048_4ab0, vec![this + 8, 0]),
                (0x0048_4490, vec![this + 8]),
            ]
        );
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x005e_6600, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (OPERATOR_DELETE, vec![0x7000]),
                (0x005a_a1a0, vec![this + 8])
            ]
        );
    }

    #[test]
    fn fn_005e6280_makes_the_data_of_each_type() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x005a_a0f0, 0x0048_4ab0, 0x0048_4490]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x005e_64b0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let one = e.call(0x005e_6280, &args![1u32]).u32();
        assert_eq!(e.mem.u32(one), ONE_VALUE_VTABLE);
        let two = e.call(0x005e_6280, &args![2u32]).u32();
        let three = e.call(0x005e_6280, &args![3u32]).u32();
        assert_eq!(e.mem.u32(three), LEVELED_LIST_VTABLE);
        let four = e.call(0x005e_6280, &args![4u32]).u32();
        assert_eq!(e.mem.u32(four), ACTIVATE_CHOICE_VTABLE);
        assert_eq!(e.call(0x005e_6280, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x005e_6280, &args![5u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![8, 0xc, 8, 0x60]);
        assert_eq!(calls_to(&log, 0x005e_64b0), vec![vec![two]]);

        // A failed allocation gives null without constructing.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_6280, &args![2u32]).u32(), 0);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x005e_64b0).is_empty());
    }

    /// Doubles for the chunk size, reads (the chunk's bytes) and the swap.
    fn chunk_engine(chunk: u32, data: Vec<u8>, swap: bool) -> Engine {
        let mut e = Engine::new();
        let size = data.len() as u32;
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register_double(GET_TES_CHUNK, move |_, _| chunk.into_ret());
        let bytes = data.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let n = if a[2] == 0 {
                bytes.len()
            } else {
                (a[2] as usize).min(bytes.len())
            };
            e.mem.write(a[1], &bytes[..n]);
            Ret::default()
        });
        let bytes = data;
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.write(a[1], &bytes[..4]);
            Ret::default()
        });
        e.register_double(NEEDS_BYTE_SWAP, move |_, _| u32::from(swap).into_ret());
        e.register(SWAP_WORD_IN_PLACE, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v.swap_bytes());
            Ret::default()
        });
        e
    }

    #[test]
    fn one_value_and_leveled_list_load_four_bytes() {
        for swap in [false, true] {
            let mut e = chunk_engine(0, vec![0x11, 0x22, 0x33, 0x44], swap);
            let this = e.mem.alloc(8);
            let want = if swap { 0x1122_3344 } else { 0x4433_2211 };
            assert!(e.call(0x005e_6790, &args![this, 0x7000u32]).bool());
            assert_eq!(e.mem.u32(this + 4), want);
            e.mem.set_u32(this + 4, 0);
            assert!(e.call(0x005e_6b00, &args![this, 0x7000u32]).bool());
            assert_eq!(e.mem.u32(this + 4), want);
        }
        let mut e = chunk_engine(0, vec![1, 2, 3], false);
        let this = e.mem.alloc(8);
        assert!(!e.call(0x005e_6790, &args![this, 0x7000u32]).bool());
        assert!(!e.call(0x005e_6b00, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    fn init_engine(cast_result: u32, has_name: bool) -> Engine {
        let mut e = Engine::new();
        quiet(&mut e, &[PRINT_WARNING]);
        e.register(0x0048_4e60, |_, _| 0x0900_0000u32.into_ret());
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(0x00ec_43fb, move |_, _| cast_result.into_ret());
        e.register_double(0x0047_4cb0, move |_, _| u32::from(has_name).into_ret());
        e.register(FORM_ID, |_, _| 0x0002_0000u32.into_ret());
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0000);
        e.register(0x0090_0000, |_, _| 0x6000u32.into_ret());
        e
    }

    #[test]
    fn leveled_list_init_item_resolves_or_warns() {
        let mut e = init_engine(0x8000, true);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0200_0000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x005e_6b70, &args![this, owner]);
        assert_eq!(e.mem.u32(this + 4), 0x8000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![
                0x0100_0065,
                0,
                TYPE_TES_FORM,
                TYPE_LEVELED_LIST_TARGET,
                0
            ]]
        );
        assert!(calls_to(&log, PRINT_WARNING).is_empty());

        // Not found, with and without an editor id.
        for (has_name, want) in [
            (true, vec![NO_LEVELED_LIST_NAMED, 0x0100_0055, 0x6000]),
            (false, vec![NO_LEVELED_LIST_ID, 0x0100_0055, 0x0002_0000]),
        ] {
            let mut e = init_engine(0, has_name);
            let owner = e.mem.alloc(8);
            e.mem.set_u32(owner, 0x0200_0000);
            let this = e.mem.alloc(8);
            e.mem.set_u32(this + 4, 0x55);
            e.call_log = Some(vec![]);
            e.call(0x005e_6b70, &args![this, owner]);
            assert_eq!(e.mem.u32(this + 4), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, PRINT_WARNING), vec![want]);
        }

        // No id: nothing happens.
        let mut e = init_engine(0, false);
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x005e_6b70, &args![this, 0x7000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn set_label_replaces_the_copy() {
        let mut e = Engine::new();
        quiet(&mut e, &[OPERATOR_DELETE]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0044_a670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0040_46f0, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x7000);
        let label = e.mem.alloc(8);
        e.mem.set_cstr(label, b"Take");
        e.call_log = Some(vec![]);
        e.call(0x005e_6c60, &args![this, label]);
        let copy = e.mem.u32(this + 4);
        assert_eq!(e.mem.cstr(copy), b"Take".to_vec());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![5]]);
        e.call(0x005e_6c60, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn fn_005e6c30_clears_the_word_and_prepares_the_script() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0048_4ab0, 0x0048_4490]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x5c, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x005e_6c30, &args![this]);
        assert_eq!(e.mem.u16(this + 0x5c), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_4ab0, vec![this + 8, 0]),
                (0x0048_4490, vec![this + 8])
            ]
        );
    }

    fn choice_engine(chunk: u32, data: Vec<u8>, swap: bool) -> Engine {
        let mut e = chunk_engine(chunk, data, swap);
        e.map(0x011c_b000, 0x1000);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0050_0910, |_, a| a[0].into_ret());
        e.register(0x005e_28b0, |_, a| a[0].into_ret());
        e.register(0x005d_43c0, |_, a| (a[0] + 0x40).into_ret());
        e.register(0x005d_43e0, |_, a| (a[0] + 0x44).into_ret());
        e.register(0x0050_0940, |_, a| (a[0] + 0x10).into_ret());
        quiet(
            &mut e,
            &[
                0x0090_5820,
                0x005e_2a20,
                0x0040_37f0,
                0x0041_4220,
                0x0040_3d30,
                0x005a_ae70,
                GET_CHUNK_DATA_OV2,
                OPERATOR_DELETE,
                0x0044_a670,
                0x0040_46f0,
            ],
        );
        e
    }

    #[test]
    fn activate_choice_load_adds_referenced_objects() {
        for (chunk, field) in [(CHUNK_SCRO, 8u32), (CHUNK_SCRV, 0xc)] {
            let mut e = choice_engine(chunk, vec![0x78, 0x56, 0x34, 0x12], false);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
            let log = e.call_log.take().unwrap();
            let entry = calls_to(&log, 0x0050_0910)[0][0];
            assert_eq!(e.mem.u32(entry + field), 0x1234_5678);
            let added = calls_to(&log, 0x0090_5820);
            assert_eq!(added[0][0], this + 8 + 0x40);
            // The pushed word is the address of a stack slot holding the entry.
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        }
    }

    #[test]
    fn activate_choice_load_reads_variables() {
        let mut e = choice_engine(CHUNK_SLSD, vec![0; 24], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
        let variable = e.mem.u32(CURRENT_VARIABLE);
        assert_ne!(variable, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x005e_2a20), vec![vec![variable, 0x7000]]);
        assert_eq!(
            calls_to(&log, 0x0090_5820),
            vec![vec![this + 8 + 0x44, CURRENT_VARIABLE]]
        );

        // SCVR names the current variable; without one it is skipped.
        let mut e = choice_engine(CHUNK_SCVR, b"fVar\0".to_vec(), false);
        e.mem.set_u32(CURRENT_VARIABLE, 0x0050_0000);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let named = calls_to(&log, 0x0040_37f0);
        assert_eq!(named.len(), 1);
        assert_eq!((named[0][0], named[0][2]), (0x0050_0018, 0));
        assert_eq!(calls_to(&log, GET_CHUNK_DATA)[0][2], 0x200);
        e.mem.set_u32(CURRENT_VARIABLE, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0040_37f0).is_empty());
    }

    #[test]
    fn activate_choice_load_reads_the_script_data_and_the_label() {
        for swap in [false, true] {
            let mut e = choice_engine(CHUNK_SCHR, vec![0; 20], swap);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            e.call(0x005e_6ee0, &args![this, 0x7000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, GET_CHUNK_DATA)[0][1..], [this + 0x18, 0]);
            assert_eq!(calls_to(&log, 0x0041_4220).len(), usize::from(swap));
        }
        let mut e = choice_engine(CHUNK_SCDA, vec![1, 2, 3], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let buffer = calls_to(&log, 0x0040_3d30)[0][0];
        assert_eq!(calls_to(&log, 0x0040_3d30), vec![vec![buffer, 0, 3]]);
        assert_eq!(calls_to(&log, 0x005a_ae70), vec![vec![this + 8, 3, buffer]]);

        let mut e = choice_engine(CHUNK_EPF2, b"Open\0".to_vec(), false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0044_a670).len(), 1);

        let mut e = choice_engine(CHUNK_EPF3, vec![2, 0], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA_OV2),
            vec![vec![0x7000, this + 0x5c]]
        );

        // Any other chunk does nothing and still returns true.
        let mut e = choice_engine(0x1234_5678, vec![], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn activate_choice_init_item_warns_after_errors() {
        for errors in [0u32, 3] {
            let mut e = Engine::new();
            let reads = std::rc::Rc::new(std::cell::Cell::new(0u32));
            let r = reads.clone();
            e.register_double(0x0046_e8a0, move |_, _| {
                r.set(r.get() + 1);
                let value = if r.get() == 1 { 7u32 } else { errors };
                value.into_ret()
            });
            quiet(
                &mut e,
                &[0x0043_b2b0, 0x004f_ffe0, 0x005a_b400, PRINT_WARNING],
            );
            e.register(FORM_ID, |_, _| 0x0003_0000u32.into_ret());
            e.put_vtable(0x0200_0000, &[0; 0x80]);
            e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0000);
            e.register(0x0090_0000, |_, _| 0x6000u32.into_ret());
            let owner = e.mem.alloc(8);
            e.mem.set_u32(owner, 0x0200_0000);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            e.call(0x005e_7230, &args![this, owner]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0043_b2b0), vec![vec![1], vec![0]]);
            assert_eq!(calls_to(&log, 0x004f_ffe0), vec![vec![0], vec![7]]);
            assert_eq!(calls_to(&log, 0x005a_b400), vec![vec![this + 8, owner]]);
            let warnings = calls_to(&log, PRINT_WARNING);
            if errors == 0 {
                assert!(warnings.is_empty());
            } else {
                assert_eq!(
                    warnings,
                    vec![vec![RESULT_SCRIPT_ERRORS, 0x6000, 0x0003_0000]]
                );
            }
        }
    }
}
