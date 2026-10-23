//! `fallout shared/facegen/bsfacegenkeyframe.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSFaceGenKeyframe` (0xc bytes: vtable, `eType`, `fTime`) and
//! `BSFaceGenKeyframeMultiple` (0x14 bytes: plus `pQuantities` and
//! `iNumQuantities`; its layout is declared in `unplaced/run_00649ba0.rs`).
//! The virtual slots the functions call are the Xbox PDB's (the PC offsets
//! the code uses match them): +0x4 `GetType`, +0x8 `SetType`, +0xc `GetTime`,
//! +0x10 `SetTime`, +0x2c `Compare`, +0x30 `GetStrength`, +0x44 `IsMultiple`,
//! +0x4c `IsValidQuantity`, +0x50 `ResetQuantityArray`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::unplaced::run_00649ba0::BSFaceGenKeyframeMultiple;

/// `BSFaceGenKeyframe`'s vtable.
const KEYFRAME_VTABLE: u32 = 0x0106_53fc;
/// `BSFaceGenKeyframeMultiple`'s vtable.
const MULTIPLE_VTABLE: u32 = 0x0106_544c;
/// The virtual slots (byte offsets) named in the module comment.
const SLOT_GET_TYPE: u32 = 0x4;
const SLOT_SET_TYPE: u32 = 0x8;
const SLOT_GET_TIME: u32 = 0xc;
const SLOT_SET_TIME: u32 = 0x10;
const SLOT_COMPARE: u32 = 0x2c;
const SLOT_GET_STRENGTH: u32 = 0x30;
const SLOT_IS_MULTIPLE: u32 = 0x44;
const SLOT_IS_VALID_QUANTITY: u32 = 0x4c;
const SLOT_RESET_QUANTITY_ARRAY: u32 = 0x50;
/// `0044edb0` (thiscall on a keyframe, returns a word: its quantity count by its
/// use; the map's name `BaseProcess::GetCurrentProcedureIndex` comes from folded code).
const QUANTITY_COUNT_0044EDB0: u32 = 0x0044_edb0;
/// `005dd4e0` (thiscall on a keyframe, one word: an index; the quantity as a
/// `float` in `ST0` by its use).
const QUANTITY_005DD4E0: u32 = 0x005d_d4e0;
/// `0064a4d0` (thiscall on a keyframe: index, `float`; returns `AL`, whether the
/// quantity changed, by its use).
const SET_QUANTITY_0064A4D0: u32 = 0x0064_a4d0;
/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0044ddc0` (thiscall on a list, returns a word; unnamed).
const LIST_0044DDC0: u32 = 0x0044_ddc0;
/// `007b5370` (thiscall on a list, returns the address of an item; unnamed).
const LIST_ITEM_007B5370: u32 = 0x007b_5370;
/// `004ed8c0` (thiscall on a list, one word: the address of the item to add; unnamed).
const LIST_ADD_004ED8C0: u32 = 0x004e_d8c0;
/// `0040eb10` (cdecl, three `float`s, returns `AL`; unnamed): compares a value with
/// another within a tolerance, by its use.
const NEARLY_EQUAL_0040EB10: u32 = 0x0040_eb10;
/// `00408860` (cdecl, one `float`, returns `ST0`; unnamed): its absolute value by its use.
const ABSOLUTE_00408860: u32 = 0x0040_8860;
/// `BSFaceGenManager::SetToZero` (`00653e20`, cdecl: array, count).
const MANAGER_SET_TO_ZERO: u32 = 0x0065_3e20;
/// `00653dc0` (cdecl: array, count, word; unnamed, `bsfacegenmanager.cpp`).
const MANAGER_RESET_00653DC0: u32 = 0x0065_3dc0;
/// `00403e20` (thiscall on a setting, returns the address of its value; unnamed).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `fn_0064fb10` and `fn_0064fb50` read.
const SETTING_011D57C4: u32 = 0x011d_57c4;
const SETTING_011D577C: u32 = 0x011d_577c;
/// `0.0` and `1.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `0.001` (`float`): the tolerance `MergeMorphDataFrom` passes to `0040eb10`.
const TOLERANCE: u32 = 0x0101_7d00;
/// `FLT_MAX` as a `double` and as a `float`.
const MAXIMUM: u32 = 0x0102_31b0;
const MAXIMUM_FLOAT: u32 = 0x0101_6970;

/// A quantity of `keyframe` (`005dd4e0`), in `ST0`.
fn quantity(e: &mut Engine, keyframe: u32, index: u32) -> f64 {
    e.call(QUANTITY_005DD4E0, &args![keyframe, index]).f64()
}

/// Sets a quantity of `keyframe` (`0064a4d0`); whether it changed.
fn set_quantity(e: &mut Engine, keyframe: u32, index: u32, value: u32) -> u8 {
    e.call(SET_QUANTITY_0064A4D0, &args![keyframe, index, value])
        .u8()
}

/// The quantity count of `keyframe` (`0044edb0`).
fn quantity_count(e: &mut Engine, keyframe: u32) -> u32 {
    e.call(QUANTITY_COUNT_0044EDB0, &args![keyframe]).u32()
}

/// `other` when it is a multiple keyframe (slot +0x44), else null.
fn as_multiple(e: &mut Engine, other: u32) -> u32 {
    if other != 0 && e.vcall(other, SLOT_IS_MULTIPLE, &[]).bool() {
        other
    } else {
        0
    }
}

/// Whether `other` has this keyframe's type and quantity count.
fn same_shape(e: &mut Engine, this: u32, other: u32) -> bool {
    let first = e.vcall(this, SLOT_GET_TYPE, &[]).u32();
    let second = e.vcall(other, SLOT_GET_TYPE, &[]).u32();
    first == second && quantity_count(e, this) == quantity_count(e, other)
}

// Translated from 00488d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::GetTime` (Xbox PDB): `fTime`, loaded into `ST0`.
pub fn bs_face_gen_keyframe_get_time(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr() + 8))
}

// Translated from 0064ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe`'s constructor (unnamed in the map; `RET 4`): the vtable, `eType`
/// and an `fTime` of 0. Returns `this`.
pub fn fn_0064ea80(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.mem.set_u32(this.addr(), KEYFRAME_VTABLE);
    e.mem.set_u32(this.addr() + 4, kind);
    e.mem.set_u32(this.addr() + 8, x87_store32(0.0));
    this
}

// Translated from 0064eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe`'s destructor body (unnamed in the map): stores its vtable.
pub fn fn_0064eae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), KEYFRAME_VTABLE);
}

// Translated from 0064eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::SetType` (Xbox PDB): sets `eType`; whether it changed.
pub fn bs_face_gen_keyframe_set_type(e: &mut Engine, this: Ptr, kind: u32) -> bool {
    if kind == e.mem.u32(this.addr() + 4) {
        false
    } else {
        e.mem.set_u32(this.addr() + 4, kind);
        true
    }
}

// Translated from 0064eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::SetTime` (Xbox PDB): sets `fTime` (quiet) unless it equals `time`
/// (`FUCOMPP`: a NaN is never equal); whether it changed.
pub fn bs_face_gen_keyframe_set_time(e: &mut Engine, this: Ptr, time: f32) -> bool {
    let current = x87_load32(e.mem.u32(this.addr() + 8));
    if current == x87_load32(time.to_bits()) {
        false
    } else {
        e.mem
            .set_u32(this.addr() + 8, x87_quiet_bits(time.to_bits()));
        true
    }
}

// Translated from 0064eb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064eb60 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s constructor (unnamed in the map; `RET 8`): the base with
/// `kind`, its vtable, no quantities, then `ResetQuantityArray(count, 1)`. Returns `this`.
pub fn fn_0064eb60(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    kind: u32,
    count: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    fn_0064ea80(e, this.cast(), kind);
    e.mem.set_u32(this.addr(), MULTIPLE_VTABLE);
    e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
    e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, 0);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, count, 1);
    this
}

// Translated from 0064ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple`'s scalar deleting destructor (Xbox PDB): the destructor
/// body (`fn_0064ed00`), then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_face_gen_keyframe_multiple_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    flags: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    fn_0064ed00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0064ec10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ec10 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s copy constructor (unnamed in the map; `RET 4`): the base
/// with type -1, the vtable, `other`'s type and time (`SetType`, `SetTime`), a quantity
/// array of `other`'s count (`ResetQuantityArray(count, 1)`) and each of its quantities.
/// Returns `this`.
pub fn fn_0064ec10(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    let base = this.addr();
    fn_0064ea80(e, this.cast(), 0xffff_ffff);
    e.mem.set_u32(base, MULTIPLE_VTABLE);
    let kind = e.vcall(other, SLOT_GET_TYPE, &[]).u32();
    bs_face_gen_keyframe_set_type(e, this.cast(), kind);
    let time = e.vcall(other, SLOT_GET_TIME, &[]).f64();
    bs_face_gen_keyframe_set_time(e, this.cast(), f32::from_bits(x87_store32(time)));
    e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
    e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, 0);
    let count = quantity_count(e, other);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, count, 1);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let value = x87_store32(quantity(e, other, i));
        set_quantity(e, base, i, value);
        i += 1;
    }
    this
}

// Translated from 0064ed00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ed00 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s destructor body (unnamed in the map; Ghidra shows a
/// library-match name): the vtable, `ResetQuantityArray(0, 1)` (frees the array), then
/// the base's destructor body.
pub fn fn_0064ed00(e: &mut Engine, this: Ptr<BSFaceGenKeyframeMultiple>) {
    e.mem.set_u32(this.addr(), MULTIPLE_VTABLE);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, 0, 1);
    fn_0064eae0(e, this.cast());
}

// Translated from 0064ed60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ed60 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple::AddToList` (Xbox PDB, `RET 4`): adds a copy of this
/// keyframe (0x14 bytes, `fn_0064ec10`; null when the allocation fails) to `list`
/// (`004ed8c0`) and returns true. Refused (false) without a list, for a negative time,
/// or when the list is not empty (`0044ddc0`) and its item (`007b5370`) is not a
/// multiple keyframe of the same type.
pub fn bs_face_gen_keyframe_multiple_add_to_list(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    list: u32,
) -> bool {
    let base = this.addr();
    if list == 0 {
        return false;
    }
    let time = e.vcall(base, SLOT_GET_TIME, &[]).f64();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,5; JP: refused only below zero.
    if time < zero {
        return false;
    }
    if e.call(LIST_0044DDC0, &args![list]).u32() != 0 {
        let slot = e.call(LIST_ITEM_007B5370, &args![list]).u32();
        let item = e.mem.u32(slot);
        let multiple = as_multiple(e, item);
        if multiple == 0 {
            return false;
        }
        let kind = e.vcall(multiple, SLOT_GET_TYPE, &[]).u32();
        if kind != e.vcall(base, SLOT_GET_TYPE, &[]).u32() {
            return false;
        }
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        fn_0064ec10(e, Ptr::new(block), base).addr()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), copy);
        e.call(LIST_ADD_004ED8C0, &args![list, slot]);
    });
    true
}

// Translated from 0064eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::MergeMorphDataFrom` (Xbox PDB, `RET 0x10`): blends the
/// quantities of `other` (a multiple keyframe of the same type and count, not this one)
/// into this one with `weight` clamped to 0..1: each quantity becomes
/// `other * w + this * (1 - w)`, or `other`'s when `weight >= 1` or this one is
/// `FLT_MAX`; with `skip_unset` clear, `other`'s quantities within 0.001 of 0
/// (`0040eb10`) are left alone; an `other` quantity of `FLT_MAX` sets `FLT_MAX` only with
/// `keep_unset`. Returns whether any quantity changed (false when refused).
pub fn bs_face_gen_keyframe_multiple_merge_morph_data_from(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
    weight: f32,
    skip_unset: u8,
    keep_unset: u8,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, other);
    let mut changed = 0u8;
    if source == 0 || source == base || !same_shape(e, base, source) {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let w = x87_load32(weight.to_bits());
    // FCOMP 0; TEST AH,5; JP: 0 only below zero. FCOMP 1; TEST AH,0x41; JNZ: the
    // weight itself unless above one.
    let clamped = if w < zero {
        x87_store32(0.0)
    } else if !(w > one) {
        x87_quiet_bits(weight.to_bits())
    } else {
        x87_store32(1.0)
    };
    let maximum: f64 = e.global(MAXIMUM);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        if skip_unset == 0 {
            let value = x87_store32(quantity(e, source, i));
            let tolerance = x87_quiet_bits(e.global::<u32>(TOLERANCE));
            if e.call(
                NEARLY_EQUAL_0040EB10,
                &args![value, x87_store32(0.0), tolerance],
            )
            .bool()
            {
                i += 1;
                continue;
            }
        }
        let from = quantity(e, source, i);
        // FCOMP FLT_MAX; TEST AH,5; JP: set quantities only below FLT_MAX.
        if from < maximum {
            let mut copy = !(w < one);
            if !copy {
                let current = quantity(e, base, i);
                copy = !(current < maximum);
            }
            let value = if copy {
                x87_store32(quantity(e, source, i))
            } else {
                let current = quantity(e, base, i);
                let rest = x87_sub(1.0, x87_load32(clamped));
                let kept = x87_mul(current, rest);
                let from = quantity(e, source, i);
                let blended = x87_add(x87_mul(from, x87_m32(clamped)), kept);
                x87_quiet_bits(x87_store32(blended))
            };
            changed |= set_quantity(e, base, i, value);
        } else if keep_unset != 0 {
            let value = x87_quiet_bits(e.global::<u32>(MAXIMUM_FLOAT));
            changed |= set_quantity(e, base, i, value);
        }
        i += 1;
    }
    changed != 0
}

// Translated from 0064f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::ClearData` (Xbox PDB, `RET 4`): sets the time to 0 and
/// resets the quantity array (`ResetQuantityArray(count, flag)`); returns whether there
/// was anything to clear (a time other than 0, or a valid quantity).
pub fn bs_face_gen_keyframe_multiple_clear_data(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    flag: u8,
) -> bool {
    let base = this.addr();
    let mut had_data = false;
    let time = e.vcall(base, SLOT_GET_TIME, &[]).f64();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,0x44; JNP: nothing only when equal to zero.
    if !(time == zero) {
        had_data = true;
    }
    e.vcall(base, SLOT_SET_TIME, &args![x87_store32(0.0)]);
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        if e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            had_data = true;
            break;
        }
        i += 1;
    }
    let count = quantity_count(e, base);
    e.vcall(
        base,
        SLOT_RESET_QUANTITY_ARRAY,
        &args![count, u32::from(flag)],
    );
    had_data
}

// Translated from 0064f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::CopyFrom` (Xbox PDB, `RET 4`): from a multiple keyframe
/// `other`, takes its type and time (slots +0x8, +0x10), a quantity array of its count
/// (`ResetQuantityArray(count, 1)`) and each quantity.
pub fn bs_face_gen_keyframe_multiple_copy_from(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) {
    let base = this.addr();
    let source = as_multiple(e, other);
    if source == 0 {
        return;
    }
    let kind = e.vcall(source, SLOT_GET_TYPE, &[]).u32();
    e.vcall(base, SLOT_SET_TYPE, &args![kind]);
    let time = e.vcall(source, SLOT_GET_TIME, &[]).f64();
    e.vcall(base, SLOT_SET_TIME, &args![x87_store32(time)]);
    let count = quantity_count(e, source);
    e.vcall(base, SLOT_RESET_QUANTITY_ARRAY, &args![count, 1u32]);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let value = x87_store32(quantity(e, source, i));
        set_quantity(e, base, i, value);
        i += 1;
    }
}

// Translated from 0064f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::Compare` (Xbox PDB, `RET 4`): true when `other` is not a
/// multiple keyframe of the same type and count or any quantity differs (`FUCOMPP`: a NaN
/// differs), false when all are equal.
pub fn bs_face_gen_keyframe_multiple_compare(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, other);
    if source == 0 || !same_shape(e, source, base) {
        return true;
    }
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        let from = quantity(e, source, i);
        let current = quantity(e, base, i);
        if !(from == current) {
            return true;
        }
        i += 1;
    }
    false
}

// Translated from 0064f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::GetStrength` (Xbox PDB): the largest valid quantity (slot
/// +0x4c), at least 0; in `ST0`.
pub fn bs_face_gen_keyframe_multiple_get_strength(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> f64 {
    let base = this.addr();
    let mut strength = x87_store32(0.0);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        if e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
            let value = e.mem.u32(array.wrapping_add(i.wrapping_mul(4)));
            // FCOMPP (strength against the quantity); TEST AH,5; JP: taken only above.
            if x87_load32(strength) < x87_load32(value) {
                strength = x87_quiet_bits(value);
            }
        }
        i += 1;
    }
    x87_load32(strength)
}

// Translated from 0064f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::SetToZero` (Xbox PDB): `BSFaceGenManager::SetToZero` on the
/// quantity array.
pub fn bs_face_gen_keyframe_multiple_set_to_zero(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) {
    let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
    let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
    e.call(MANAGER_SET_TO_ZERO, &args![array, count]);
}

// Translated from 0064f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::IsTargetAvailable` (Xbox PDB): whether any quantity is in
/// `(0, 1]` (the exe's three compares; a NaN never is).
pub fn bs_face_gen_keyframe_multiple_is_target_available(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> bool {
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        let value = x87_load32(e.mem.u32(array.wrapping_add(i.wrapping_mul(4))));
        i += 1;
        // FCOMP 0; TEST AH,5; JNP: skipped below zero.
        if value < zero {
            continue;
        }
        // FCOMP 1; TEST AH,0x41; JNZ: skipped above one.
        if value > one {
            continue;
        }
        // FCOMP 0; TEST AH,0x41; JNZ: skipped unless above zero.
        if !(value > zero) {
            continue;
        }
        return true;
    }
    false
}

// Translated from 0064f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::UpdateToTarget` (Xbox PDB, `RET 8`): moves each valid
/// quantity towards `target`'s by at most `step = (strength * s2 + s1) * time` (`s1`, `s2`
/// from `fn_0064fb10`, `fn_0064fb50`, the strength from `target`'s slot +0x30), reaching
/// it when it is nearer than that. Only for a positive `time` (or NaN) and a multiple
/// `target` of the same type and count that differs (`Compare`); returns whether it ran.
pub fn bs_face_gen_keyframe_multiple_update_to_target(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    time: f32,
    target: u32,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, target);
    let zero: f64 = e.global(ZERO);
    let t = x87_load32(time.to_bits());
    // FCOMP; TEST AH,0x41; JNP: refused below or equal to zero.
    if t <= zero {
        return false;
    }
    if source == 0
        || !same_shape(e, source, base)
        || !e.vcall(base, SLOT_COMPARE, &args![source]).bool()
    {
        return false;
    }
    let offset = fn_0064fb10(e);
    let scale = fn_0064fb50(e);
    let strength = e.vcall(target, SLOT_GET_STRENGTH, &[]).f64();
    let step = x87_mul(
        x87_add(x87_mul(strength, scale), offset),
        x87_m32(time.to_bits()),
    );
    let step = x87_store32(step);
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        if !e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            i += 1;
            continue;
        }
        let goal = quantity(e, source, i);
        let current = quantity(e, base, i);
        let difference = x87_store32(x87_sub(goal, current));
        let distance = e
            .call(ABSOLUTE_00408860, &args![x87_quiet_bits(difference)])
            .f64();
        let d = x87_load32(difference);
        // FCOMPP (step against the distance); TEST AH,0x41; JNZ: reached only when the
        // step is above it.
        if x87_load32(step) > distance {
            let value = x87_store32(quantity(e, source, i));
            set_quantity(e, base, i, value);
        } else if d < zero {
            let value = x87_store32(x87_sub(quantity(e, base, i), x87_m32(step)));
            set_quantity(e, base, i, x87_quiet_bits(value));
        } else if d > zero {
            let value = x87_store32(x87_add(quantity(e, base, i), x87_m32(step)));
            set_quantity(e, base, i, x87_quiet_bits(value));
        }
        i += 1;
    }
    true
}

/// A setting's `float` (`00403e20`) when it is above zero, else 0; in `ST0`.
fn positive_setting(e: &mut Engine, setting: u32) -> f64 {
    let address = e.call(SETTING_VALUE, &args![setting]).u32();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,0x41; JNZ: 0 unless above zero.
    let value = if x87_load32(e.mem.u32(address)) > zero {
        let address = e.call(SETTING_VALUE, &args![setting]).u32();
        x87_quiet_bits(e.mem.u32(address))
    } else {
        x87_store32(0.0)
    };
    x87_load32(value)
}

// Translated from 0064fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the setting at `011d57c4` when above zero, else 0 (`ST0`).
pub fn fn_0064fb10(e: &mut Engine) -> f64 {
    positive_setting(e, SETTING_011D57C4)
}

// Translated from 0064fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the setting at `011d577c` when above zero, else 0 (`ST0`).
pub fn fn_0064fb50(e: &mut Engine) -> f64 {
    positive_setting(e, SETTING_011D577C)
}

// Translated from 0064fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::IsValidQuantity` (Xbox PDB, `RET 4`): whether `index` is
/// in range and its quantity is in `[0, 1]` (a NaN is not).
pub fn bs_face_gen_keyframe_multiple_is_valid_quantity(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    index: u32,
) -> bool {
    if index >= e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
    let value = x87_load32(e.mem.u32(array.wrapping_add(index.wrapping_mul(4))));
    // FCOMP 0; TEST AH,1; JNZ: invalid below zero or unordered.
    if value < zero || value.is_nan() {
        return false;
    }
    // FCOMP 1; TEST AH,0x41; JP: valid only below or equal to one.
    value <= one
}

// Translated from 0064fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::ResetQuantityArray` (Xbox PDB, `RET 8`): for a new count,
/// frees the array and allocates `count` floats (`operator new`, the size all ones when
/// `count * 4` overflows), filled by `00653dc0` with `flag`; then fills the array again
/// (`00653dc0`) whenever there are quantities.
pub fn bs_face_gen_keyframe_multiple_reset_quantity_array(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    count: u32,
    flag: u8,
) {
    if count != e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        if array != 0 {
            e.call(OPERATOR_DELETE, &args![array]);
            e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
        }
        e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, count);
        if count != 0 {
            let size = count.checked_mul(4).unwrap_or(u32::MAX);
            let block = e.call(OPERATOR_NEW, &args![size]).u32();
            e.set(this, BSFaceGenKeyframeMultiple::pQuantities, block);
            e.call(
                MANAGER_RESET_00653DC0,
                &args![block, count, u32::from(flag)],
            );
        }
    }
    let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
    if count != 0 {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        e.call(
            MANAGER_RESET_00653DC0,
            &args![array, count, u32::from(flag)],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_8d50, bs_face_gen_keyframe_get_time(Ptr) -> f64),
        entry!(0x0064_ea80, fn_0064ea80(Ptr, u32) -> Ptr),
        entry!(0x0064_eae0, fn_0064eae0(Ptr)),
        entry!(0x0064_eb00, bs_face_gen_keyframe_set_type(Ptr, u32) -> bool),
        entry!(0x0064_eb30, bs_face_gen_keyframe_set_time(Ptr, f32) -> bool),
        entry!(
            0x0064_eb60,
            fn_0064eb60(Ptr<BSFaceGenKeyframeMultiple>, u32, u32) -> Ptr<BSFaceGenKeyframeMultiple>
        ),
        entry!(
            0x0064_ebe0,
            bs_face_gen_keyframe_multiple_scalar_deleting_destructor(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
            ) -> Ptr<
                BSFaceGenKeyframeMultiple,
            >
        ),
        entry!(
            0x0064_ec10,
            fn_0064ec10(Ptr<BSFaceGenKeyframeMultiple>, u32) -> Ptr<BSFaceGenKeyframeMultiple>
        ),
        entry!(0x0064_ed00, fn_0064ed00(Ptr<BSFaceGenKeyframeMultiple>)),
        entry!(
            0x0064_ed60,
            bs_face_gen_keyframe_multiple_add_to_list(Ptr<BSFaceGenKeyframeMultiple>, u32) -> bool
        ),
        entry!(
            0x0064_eea0,
            bs_face_gen_keyframe_multiple_merge_morph_data_from(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
                f32,
                u8,
                u8,
            ) -> bool
        ),
        entry!(
            0x0064_f480,
            bs_face_gen_keyframe_multiple_clear_data(Ptr<BSFaceGenKeyframeMultiple>, u8) -> bool
        ),
        entry!(
            0x0064_f5b0,
            bs_face_gen_keyframe_multiple_copy_from(Ptr<BSFaceGenKeyframeMultiple>, u32)
        ),
        entry!(
            0x0064_f690,
            bs_face_gen_keyframe_multiple_compare(Ptr<BSFaceGenKeyframeMultiple>, u32) -> bool
        ),
        entry!(
            0x0064_f760,
            bs_face_gen_keyframe_multiple_get_strength(Ptr<BSFaceGenKeyframeMultiple>) -> f64
        ),
        entry!(
            0x0064_f850,
            bs_face_gen_keyframe_multiple_set_to_zero(Ptr<BSFaceGenKeyframeMultiple>)
        ),
        entry!(
            0x0064_f880,
            bs_face_gen_keyframe_multiple_is_target_available(
                Ptr<BSFaceGenKeyframeMultiple>,
            ) -> bool
        ),
        entry!(
            0x0064_f900,
            bs_face_gen_keyframe_multiple_update_to_target(
                Ptr<BSFaceGenKeyframeMultiple>,
                f32,
                u32,
            ) -> bool
        ),
        entry!(0x0064_fb10, fn_0064fb10() -> f64),
        entry!(0x0064_fb50, fn_0064fb50() -> f64),
        entry!(
            0x0064_fb90,
            bs_face_gen_keyframe_multiple_is_valid_quantity(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
            ) -> bool
        ),
        entry!(
            0x0064_fc00,
            bs_face_gen_keyframe_multiple_reset_quantity_array(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
                u8,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    const GET_TYPE: u32 = 0x0340_0004;
    const IS_MULTIPLE: u32 = 0x0340_0044;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The keyframe vtable for the tests: this unit's translations in the slots it
    /// has, doubles for the type (the word at +4) and `IsMultiple` (the word at +0x14).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x60);
        for (slot, target) in [
            (SLOT_GET_TYPE, GET_TYPE),
            (SLOT_SET_TYPE, 0x0064_eb00),
            (SLOT_GET_TIME, 0x0048_8d50),
            (SLOT_SET_TIME, 0x0064_eb30),
            (SLOT_COMPARE, 0x0064_f690),
            (SLOT_GET_STRENGTH, 0x0064_f760),
            (SLOT_IS_MULTIPLE, IS_MULTIPLE),
            (SLOT_IS_VALID_QUANTITY, 0x0064_fb90),
            (SLOT_RESET_QUANTITY_ARRAY, 0x0064_fc00),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.register(GET_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(IS_MULTIPLE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(QUANTITY_COUNT_0044EDB0, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(QUANTITY_005DD4E0, |e, a| Ret {
            st0: f64::from(e.mem.f32(e.mem.u32(a[0] + 0xc) + 4 * a[1])),
            ..Ret::default()
        });
        e.register(SET_QUANTITY_0064A4D0, |e, a| {
            let slot = e.mem.u32(a[0] + 0xc) + 4 * a[1];
            let changed = e.mem.u32(slot) != a[2];
            e.mem.set_u32(slot, a[2]);
            u32::from(changed).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        answer(&mut e, OPERATOR_DELETE, 0);
        answer(&mut e, MANAGER_RESET_00653DC0, 0);
        for (addr, value) in [(ZERO, 0.0f64), (ONE, 1.0), (MAXIMUM, f64::from(f32::MAX))] {
            e.map(addr, 8);
            e.mem.set_f64(addr, value);
        }
        e.map(TOLERANCE, 4);
        e.mem.set_f32(TOLERANCE, 0.001);
        e.map(MAXIMUM_FLOAT, 4);
        e.mem.set_f32(MAXIMUM_FLOAT, f32::MAX);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    /// A multiple keyframe of `kind` with `values` (0x18 bytes: the multiple flag at +0x14).
    fn keyframe(
        e: &mut Engine,
        vtable: u32,
        kind: u32,
        values: &[f32],
    ) -> Ptr<BSFaceGenKeyframeMultiple> {
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, kind);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 0xc, array);
        e.mem.set_u32(this + 0x10, values.len() as u32);
        e.mem.set_u32(this + 0x14, 1);
        Ptr::new(this)
    }

    fn values(e: &Engine, this: Ptr<BSFaceGenKeyframeMultiple>) -> Vec<f32> {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
        (0..count).map(|i| e.mem.f32(array + 4 * i)).collect()
    }

    #[test]
    fn keyframe_base_type_and_time() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0064_ea80, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), KEYFRAME_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.f32(this + 8), 0.0);
        assert!(!e.call(0x0064_eb00, &args![this, 3u32]).bool());
        assert!(e.call(0x0064_eb00, &args![this, 4u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 4);
        assert!(!e.call(0x0064_eb30, &args![this, 0.0f32]).bool());
        assert!(e.call(0x0064_eb30, &args![this, 2.5f32]).bool());
        assert_eq!(e.call(0x0048_8d50, &args![this]).f64(), 2.5);
        // A NaN never equals: always set, quiet.
        assert!(e.call(0x0064_eb30, &args![this, 0x7f80_0001u32]).bool());
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert!(e.call(0x0064_eb30, &args![this, 0x7f80_0001u32]).bool());
        e.mem.set_u32(this, 0);
        e.call(0x0064_eae0, &args![this]);
        assert_eq!(e.mem.u32(this), KEYFRAME_VTABLE);
    }

    #[test]
    fn multiple_construct_reset_and_destroy() {
        let (mut e, _) = engine();
        let this = Ptr::<BSFaceGenKeyframeMultiple>::new(e.mem.alloc(0x14));
        e.call(0x0064_eb60, &args![this, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this.addr()), MULTIPLE_VTABLE);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 2);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities), 3);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![12]]);
        assert_eq!(
            calls(&e, MANAGER_RESET_00653DC0),
            vec![vec![array, 3, 1], vec![array, 3, 1]]
        );
        // The same count only refills.
        e.call_log = Some(vec![]);
        e.call(0x0064_fc00, &args![this, 3u32, 0u32]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&e, MANAGER_RESET_00653DC0), vec![vec![array, 3, 0]]);
        // A huge count asks for all ones.
        e.call_log = Some(vec![]);
        answer(&mut e, OPERATOR_NEW, 0x7000);
        e.call(0x0064_fc00, &args![this, 0x4000_0001u32, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array]]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![u32::MAX]]);
        // Destroying frees the array; the deleting destructor frees the object on bit 0.
        e.call_log = Some(vec![]);
        e.call(0x0064_ebe0, &args![this, 1u32]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x7000], vec![this.addr()]]
        );
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities), 0);
        assert_eq!(e.mem.u32(this.addr()), KEYFRAME_VTABLE);
    }

    #[test]
    fn copy_constructor_copies_type_time_and_quantities() {
        let (mut e, vtable) = engine();
        let other = keyframe(&mut e, vtable, 5, &[0.25, 0.5]);
        e.set(other, BSFaceGenKeyframeMultiple::fTime, 1.5);
        let this = Ptr::<BSFaceGenKeyframeMultiple>::new(e.mem.alloc(0x14));
        e.call(0x0064_ec10, &args![this, other]);
        assert_eq!(e.mem.u32(this.addr()), MULTIPLE_VTABLE);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 5);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 1.5);
        assert_eq!(values(&e, this), vec![0.25, 0.5]);
    }

    #[test]
    fn add_to_list_copies_matching_keyframes() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[0.25]);
        answer(&mut e, LIST_ADD_004ED8C0, 0);
        answer(&mut e, LIST_0044DDC0, 0);
        assert!(!e.call(0x0064_ed60, &args![this, 0u32]).bool());
        e.set(this, BSFaceGenKeyframeMultiple::fTime, -1.0);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.set(this, BSFaceGenKeyframeMultiple::fTime, 0.0);
        e.register(LIST_ADD_004ED8C0, |e, a| {
            let copy = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(copy), MULTIPLE_VTABLE);
            assert_eq!(e.mem.u32(copy + 4), 5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        assert_eq!(calls(&e, OPERATOR_NEW)[0], vec![0x14]);
        assert_eq!(calls(&e, LIST_ADD_004ED8C0).len(), 1);
        // A non-empty list: only behind a multiple keyframe of the same type.
        answer(&mut e, LIST_0044DDC0, 1);
        let last = keyframe(&mut e, vtable, 6, &[]);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, last.addr());
        answer(&mut e, LIST_ITEM_007B5370, slot);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.mem.set_u32(last.addr() + 4, 5);
        assert!(e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.mem.set_u32(last.addr() + 0x14, 0);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
    }

    #[test]
    fn merge_blends_by_the_clamped_weight() {
        let (mut e, vtable) = engine();
        e.register(NEARLY_EQUAL_0040EB10, |_, a| {
            u32::from(f32::from_bits(a[0]).abs() <= f32::from_bits(a[2])).into_ret()
        });
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, f32::MAX, 0.5]);
        let other = keyframe(&mut e, vtable, 5, &[1.0, 0.0, 0.5, f32::MAX]);
        // Weight 0.25: 0.25, 0.75; FLT_MAX here takes the other's; FLT_MAX there is left.
        assert!(e
            .call(0x0064_eea0, &args![this, other, 0.25f32, 1u32, 0u32])
            .bool());
        assert_eq!(values(&e, this), vec![0.25, 0.75, 0.5, 0.5]);
        // keep_unset writes FLT_MAX; skip_unset clear leaves the other's zero alone.
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.25, 0.5]);
        assert!(e
            .call(0x0064_eea0, &args![this, other, 2.0f32, 0u32, 1u32])
            .bool());
        assert_eq!(values(&e, this), vec![1.0, 1.0, 0.5, f32::MAX]);
        // A negative weight blends with 0.
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.25, 0.5]);
        assert!(!e
            .call(0x0064_eea0, &args![this, other, -1.0f32, 1u32, 0u32])
            .bool());
        // Refused: itself, another type or count, not multiple.
        assert!(!e
            .call(0x0064_eea0, &args![this, this, 0.5f32, 1u32, 0u32])
            .bool());
        let shorter = keyframe(&mut e, vtable, 5, &[1.0]);
        assert!(!e
            .call(0x0064_eea0, &args![this, shorter, 0.5f32, 1u32, 0u32])
            .bool());
        let typed = keyframe(&mut e, vtable, 6, &[1.0, 0.0, 0.5, 0.5]);
        assert!(!e
            .call(0x0064_eea0, &args![this, typed, 0.5f32, 1u32, 0u32])
            .bool());
        assert!(!e
            .call(0x0064_eea0, &args![this, 0u32, 0.5f32, 1u32, 0u32])
            .bool());
    }

    #[test]
    fn clear_copy_and_compare() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[2.0, 0.5]);
        e.set(this, BSFaceGenKeyframeMultiple::fTime, 1.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0064_f480, &args![this, 0u32]).bool());
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 0.0);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, MANAGER_RESET_00653DC0), vec![vec![array, 2, 0]]);
        // Time 0 and no valid quantity: nothing to clear.
        let empty = keyframe(&mut e, vtable, 5, &[2.0, -1.0]);
        assert!(!e.call(0x0064_f480, &args![empty, 1u32]).bool());
        // CopyFrom takes the other's type, time and values.
        let other = keyframe(&mut e, vtable, 7, &[0.75, 0.125]);
        e.set(other, BSFaceGenKeyframeMultiple::fTime, 3.0);
        e.call(0x0064_f5b0, &args![this, other]);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 7);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 3.0);
        assert_eq!(values(&e, this), vec![0.75, 0.125]);
        assert!(!e.call(0x0064_f690, &args![this, other]).bool());
        let array = e.get(other, BSFaceGenKeyframeMultiple::pQuantities);
        e.mem.set_f32(array + 4, f32::NAN);
        assert!(e.call(0x0064_f690, &args![this, other]).bool());
        assert!(e.call(0x0064_f690, &args![this, 0u32]).bool());
        let typed = keyframe(&mut e, vtable, 8, &[0.75, 0.125]);
        assert!(e.call(0x0064_f690, &args![this, typed]).bool());
    }

    #[test]
    fn strength_availability_and_validity() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[0.25, 1.5, 0.75, -0.5, f32::NAN]);
        assert_eq!(e.call(0x0064_f760, &args![this]).f64(), 0.75);
        assert!(e.call(0x0064_f880, &args![this]).bool());
        let unavailable = keyframe(&mut e, vtable, 5, &[0.0, 1.5, -1.0, f32::NAN]);
        assert!(!e.call(0x0064_f880, &args![unavailable]).bool());
        assert_eq!(e.call(0x0064_f760, &args![unavailable]).f64(), 0.0);
        for (index, valid) in [
            (0u32, true),
            (1, false),
            (2, true),
            (3, false),
            (4, false),
            (5, false),
        ] {
            assert_eq!(
                e.call(0x0064_fb90, &args![this, index]).bool(),
                valid,
                "{index}"
            );
        }
        let edges = keyframe(&mut e, vtable, 5, &[0.0, 1.0]);
        assert!(e.call(0x0064_fb90, &args![edges, 0u32]).bool());
        assert!(e.call(0x0064_fb90, &args![edges, 1u32]).bool());
        answer(&mut e, MANAGER_SET_TO_ZERO, 0);
        e.call_log = Some(vec![]);
        e.call(0x0064_f850, &args![this]);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, MANAGER_SET_TO_ZERO), vec![vec![array, 5]]);
    }

    #[test]
    fn positive_settings() {
        let (mut e, _) = engine();
        let value = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE, value);
        e.mem.set_f32(value, 0.5);
        assert_eq!(e.call(0x0064_fb10, &[]).f64(), 0.5);
        assert_eq!(e.call(0x0064_fb50, &[]).f64(), 0.5);
        e.mem.set_f32(value, -0.5);
        assert_eq!(e.call(0x0064_fb10, &[]).f64(), 0.0);
        assert_eq!(
            calls(&e, SETTING_VALUE)[..3],
            [
                vec![SETTING_011D57C4],
                vec![SETTING_011D57C4],
                vec![SETTING_011D577C]
            ]
        );
    }

    #[test]
    fn update_to_target_steps_towards_the_target() {
        let (mut e, vtable) = engine();
        // Offset 0.25 and scale 0: the step is 0.25 * time.
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 0.25);
        let none = e.mem.alloc(4);
        e.register_double(SETTING_VALUE, move |_, a| {
            (if a[0] == SETTING_011D57C4 {
                value
            } else {
                none
            })
            .into_ret()
        });
        e.register(ABSOLUTE_00408860, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Ret::default()
        });
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.5, 0.0]);
        let target = keyframe(&mut e, vtable, 5, &[1.0, 0.0, 0.625, 0.0]);
        assert!(e.call(0x0064_f900, &args![this, 2.0f32, target]).bool());
        // Step 0.5: 0.5 up, 0.5 down, reached (0.125 < 0.5), unchanged.
        assert_eq!(values(&e, this), vec![0.5, 0.5, 0.625, 0.0]);
        // No time, the same values, or another type: refused.
        assert!(!e.call(0x0064_f900, &args![this, 0.0f32, target]).bool());
        let same = keyframe(&mut e, vtable, 5, &[0.5, 0.5, 0.625, 0.0]);
        assert!(!e.call(0x0064_f900, &args![this, 1.0f32, same]).bool());
        let typed = keyframe(&mut e, vtable, 6, &[1.0, 0.0, 0.625, 0.0]);
        assert!(!e.call(0x0064_f900, &args![this, 1.0f32, typed]).bool());
    }
}
