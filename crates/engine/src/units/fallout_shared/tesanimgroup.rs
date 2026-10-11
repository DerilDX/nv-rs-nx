//! `fallout shared/tesanimgroup.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const GROUP_TYPE_TABLE: u32 = 0x0119_77e4;
const SECTION_NAME_TABLE: u32 = 0x0119_9a50;
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
const REALLOC: u32 = 0x0042_f5d0;
const SECONDS_SINCE_START: u32 = 0x0063_9aa0;
const STRCHR: u32 = 0x0043_b9d0;
const STRRCHR: u32 = 0x00ec_6e30;
const STRLEN: u32 = 0x0044_a670;
const STRNICMP: u32 = 0x00ec_7ec0;
const ISNAN: u32 = 0x00ec_75b1;
const INVALID_ACTION_LOG: u32 = 0x005b_5e40;
const TYPE_NAME_TABLE: u32 = 0x0119_77d8;
const MOVE_NAME_TABLE: u32 = 0x0119_7794;
const WEAPON_NAME_TABLE: u32 = 0x0119_77a4;
const PA_PREFIX: u32 = 0x0104_85c4;
const LOG_ANIM_GROUP_ACTION_LOW: u32 = 0x0104_8578;
const LOG_ANIM_GROUP_ACTION_HIGH: u32 = 0x0104_8528;
const LOG_ANIM_GROUP_TIME_NAN: u32 = 0x0104_84f8;
const NOTE_PRIORITY_SCALE: u32 = 0x0104_84f0;

fn group_type(group: u16) -> u32 {
    u32::from(group & 0xff)
}

fn group_category(e: &Engine, group: u16) -> u32 {
    let ty = group_type(group);
    e.mem
        .u32(GROUP_TYPE_TABLE.wrapping_add(ty.wrapping_mul(0x24)))
}

fn read_c_string_byte(e: &Engine, address: u32) -> u8 {
    e.mem.u8(address)
}

/// The PC's `FLD m32fp` / `FSTP m32fp` sequence quiets signaling NaNs.
fn x87_float_round_trip(bits: u32) -> u32 {
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_3800, fn_00493800(Ptr) -> f64),
        entry!(0x005f_21c0, fn_005f21c0(Ptr, u16) -> Ptr),
        entry!(0x005f_22a0, fn_005f22a0(Ptr, u32) -> Ptr),
        entry!(0x005f_22d0, fn_005f22d0(Ptr)),
        entry!(0x005f_2370, fn_005f2370(u32, u16, u32, u8) -> u32),
        entry!(0x005f_23a0, fn_005f23a0(Ptr) -> u32),
        entry!(0x005f_23c0, fn_005f23c0(u16) -> u32),
        entry!(0x005f_23e0, fn_005f23e0(Ptr) -> u32),
        entry!(0x005f_2400, fn_005f2400(u16) -> u32),
        entry!(0x005f_2420, fn_005f2420(Ptr) -> u32),
        entry!(0x005f_2440, fn_005f2440(u16) -> u8),
        entry!(0x005f_2450, fn_005f2450(Ptr, u32)),
        entry!(0x005f_2540, fn_005f2540(u16) -> bool),
        entry!(0x005f_25a0, fn_005f25a0(i32) -> bool),
        entry!(0x005f_25d0, fn_005f25d0(u16) -> bool),
        entry!(0x005f_2610, fn_005f2610(u32) -> bool),
        entry!(0x005f_2630, fn_005f2630(u16) -> bool),
        entry!(0x005f_2670, fn_005f2670(u16) -> bool),
        entry!(0x005f_26b0, fn_005f26b0(u32) -> bool),
        entry!(0x005f_26c0, fn_005f26c0(Ptr) -> bool),
        entry!(0x005f_2700, fn_005f2700(i32) -> bool),
        entry!(0x005f_2720, fn_005f2720(u16) -> bool),
        entry!(0x005f_2750, fn_005f2750(u32) -> bool),
        entry!(0x005f_2820, fn_005f2820(Ptr, u32)),
        entry!(0x005f_29f0, fn_005f29f0(Ptr, u32, f32)),
        entry!(0x005f_2a30, fn_005f2a30(Ptr, u32, u32)),
        entry!(0x005f_2a70, fn_005f2a70(Ptr, u32, u8, u32)),
        entry!(0x005f_2ac0, fn_005f2ac0(Ptr, u32, f32)),
        entry!(0x005f_2b20, fn_005f2b20(Ptr, u32, u32)),
        entry!(0x005f_3740, fn_005f3740(Ptr, u32, f32) -> bool),
        entry!(0x005f_3780, fn_005f3780(Ptr, u32) -> f64),
        entry!(0x005f_3870, fn_005f3870(Ptr) -> Ptr),
        entry!(0x005f_38d0, fn_005f38d0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x005f_4b70, fn_005f4b70(Ptr, u8)),
        entry!(0x005f_4b90, fn_005f4b90(Ptr, u8)),
        entry!(0x005f_4bb0, fn_005f4bb0(Ptr, u32)),
        entry!(0x005f_4bd0, fn_005f4bd0(Ptr, u32, u8)),
        entry!(0x005f_4c00, fn_005f4c00(Ptr, u32) -> u8),
        entry!(0x005f_4c40, fn_005f4c40(Ptr, Ptr)),
    ]
}

// Translated from 005f21c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` constructor (Xbox PDB): initializes its base guard,
/// zeroes the owned arrays and flags, copies the default movement vector,
/// then initializes the selected group. The compiler's SEH unwind table is
/// not modeled.
// Translation incomplete: 005f21c0 - compiler SEH constructor-unwind cleanup is not modeled.
pub fn fn_005f21c0(e: &mut Engine, this: Ptr, group: u16) -> Ptr {
    e.call(0x0049_68b0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0104_84e8);
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(0x1c)]);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    for offset in [0x28, 0x29, 0x2a, 0x2b, 0x2c] {
        e.mem.set_u8(this.addr().wrapping_add(offset), 0);
    }
    for offset in [0x1c, 0x20, 0x24] {
        let value = e.global::<u32>(0x011f_426c + (offset - 0x1c));
        e.mem.set_u32(this.addr().wrapping_add(offset), value);
    }
    e.mem.set_u32(this.addr().wrapping_add(0x30), 0);
    fn_005f2450(e, this, u32::from(group));
    e.mem.set_u32(this.addr().wrapping_add(0x38), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x34), 0);
    this
}

// Translated from 005f22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` destructor body (Xbox PDB): frees its time and sound
/// arrays, clears their fields, and destroys the constructor guard. Its
/// compiler-generated SEH cleanup is not modeled.
// Translation incomplete: 005f22d0 - compiler SEH cleanup is not modeled.
pub fn fn_005f22d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_84e8);
    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    e.call(OPERATOR_DELETE, &args![times]);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    let sounds = e.mem.u32(this.addr().wrapping_add(0x38));
    e.call(OPERATOR_DELETE, &args![sounds]);
    e.mem.set_u32(this.addr().wrapping_add(0x38), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x34), 0);
    e.call(0x0049_6910, &args![this]);
}

// Translated from 005f22a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` scalar deleting destructor (Xbox PDB): invokes its
/// destructor and calls the engine sized-delete helper when bit 0 is set.
pub fn fn_005f22a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005f22d0(e, this);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 005f2450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `sType`, derives the section count by checking five names for the
/// type's category, and replaces the owned time array with a zeroed array.
pub fn fn_005f2450(e: &mut Engine, this: Ptr, group: u32) {
    let group = group as u16;
    e.mem.set_u16(this.addr().wrapping_add(0x10), group);
    let category = group_category(e, group);
    let mut count = 0u32;
    while count < 5 {
        let name_address = SECTION_NAME_TABLE
            .wrapping_add(count.wrapping_mul(0x2c))
            .wrapping_add(category.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        if read_c_string_byte(e, name) == 0 {
            break;
        }
        count += 1;
    }
    e.mem.set_u32(this.addr().wrapping_add(0x14), count);
    let old_times = e.mem.u32(this.addr().wrapping_add(0x18));
    if old_times != 0 {
        e.call(OPERATOR_DELETE, &args![old_times]);
    }
    let frame_count = e.mem.u32(this.addr().wrapping_add(0x14));
    if frame_count == 0 {
        e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    } else {
        let bytes = frame_count.saturating_mul(4);
        let times = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.mem.set_u32(this.addr().wrapping_add(0x18), times);
        let reloaded_times = e.mem.u32(this.addr().wrapping_add(0x18));
        let memset_bytes = e.mem.u32(this.addr().wrapping_add(0x14)).wrapping_shl(2);
        e.call(MEMSET, &args![reloaded_times, 0u32, memset_bytes]);
    }
}

// Translated from 005f2820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the 16-byte note-entry array. New entries receive a zero time,
/// priority byte 0xfa, weight 1.0 and a zero trailing word.
pub fn fn_005f2820(e: &mut Engine, this: Ptr, new_count: u32) {
    let old_count = e.mem.u32(this.addr().wrapping_add(0x34));
    e.mem.set_u32(this.addr().wrapping_add(0x34), new_count);
    let array_field = this.addr().wrapping_add(0x38);
    if new_count == 0 {
        let old = e.mem.u32(array_field);
        e.call(OPERATOR_DELETE, &args![old]);
        e.mem.set_u32(array_field, 0);
        return;
    }

    if old_count == 0 {
        let allocation_bytes = e.mem.u32(this.addr().wrapping_add(0x34)).wrapping_mul(0x10);
        let new_array = e.call(OPERATOR_NEW, &args![allocation_bytes]).u32();
        e.mem.set_u32(array_field, new_array);
        initialize_note_entries(e, this, 0);
        return;
    }

    let old_bytes = old_count.wrapping_mul(0x10);
    let preserved = e.call(OPERATOR_NEW, &args![old_bytes]).u32();
    let old_array = e.mem.u32(array_field);
    e.call(MEMCPY, &args![preserved, old_array, old_bytes]);
    let reloaded_count = e.mem.u32(this.addr().wrapping_add(0x34));
    let reloaded_array = e.mem.u32(array_field);
    let new_array = e
        .call(
            REALLOC,
            &args![reloaded_array, reloaded_count.wrapping_mul(0x10)],
        )
        .u32();
    e.mem.set_u32(array_field, new_array);
    let current_count = e.mem.u32(this.addr().wrapping_add(0x34));
    let copied_count = old_count.min(current_count);
    let copied_bytes = copied_count.wrapping_mul(0x10);
    let destination = e.mem.u32(array_field);
    e.call(MEMCPY, &args![destination, preserved, copied_bytes]);
    e.call(OPERATOR_DELETE, &args![preserved]);
    if old_count < e.mem.u32(this.addr().wrapping_add(0x34)) {
        initialize_note_entries(e, this, old_count);
    }
}

fn initialize_note_entries(e: &mut Engine, this: Ptr, from: u32) {
    let mut index = from;
    while index < e.mem.u32(this.addr().wrapping_add(0x34)) {
        let array = e.mem.u32(this.addr().wrapping_add(0x38));
        let entry = array.wrapping_add(index.wrapping_mul(0x10));
        e.mem.set_u32(entry.wrapping_add(0xc), 0);
        e.mem.set_f32(entry, 0.0);
        e.mem.set_u8(entry.wrapping_add(4), 0xfa);
        e.mem.set_f32(entry.wrapping_add(8), 1.0);
        index = index.wrapping_add(1);
    }
}

// Translated from 005f29f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's time when the index is below `noteCount` and the
/// array has been allocated.
pub fn fn_005f29f0(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array.wrapping_add(index.wrapping_mul(0x10)),
            x87_float_round_trip(value.to_bits()),
        );
    }
}

// Translated from 005f2a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's trailing word when the index is valid.
pub fn fn_005f2a30(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array
                .wrapping_add(index.wrapping_mul(0x10))
                .wrapping_add(0xc),
            value,
        );
    }
}

// Translated from 005f2a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the note entry's priority byte and weight. The PC accepts index
/// equal to `noteCount`, matching the disassembly's unsigned `JBE` branch.
pub fn fn_005f2a70(e: &mut Engine, this: Ptr, index: u32, priority: u8, weight_bits: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index <= count && array != 0 {
        let entry = array.wrapping_add(index.wrapping_mul(0x10));
        e.mem.set_u8(entry.wrapping_add(4), priority);
        e.mem.set_u32(entry.wrapping_add(8), weight_bits);
    }
}

// Translated from 005f2ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `trunc(value * 250)` in the note priority byte when
/// the index is valid.
pub fn fn_005f2ac0(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        let scale = e.global::<f64>(NOTE_PRIORITY_SCALE);
        let scaled = (f64::from(value) * scale).trunc();
        let byte = if scaled.is_finite() && scaled >= i32::MIN as f64 && scaled <= i32::MAX as f64 {
            (scaled as i32) as u8
        } else {
            0
        };
        e.mem.set_u8(
            array.wrapping_add(index.wrapping_mul(0x10)).wrapping_add(4),
            byte,
        );
    }
}

// Translated from 005f2b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's weight bits when the index is valid.
pub fn fn_005f2b20(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array.wrapping_add(index.wrapping_mul(0x10)).wrapping_add(8),
            value,
        );
    }
}

// Translated from 005f3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores one animation time and reports success when the array exists and
/// the index is below `frameCount`.
pub fn fn_005f3740(e: &mut Engine, this: Ptr, index: u32, time: f32) -> bool {
    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    let count = e.mem.u32(this.addr().wrapping_add(0x14));
    if times == 0 || index >= count {
        return false;
    }
    e.mem.set_u32(
        times.wrapping_add(index.wrapping_mul(4)),
        x87_float_round_trip(time.to_bits()),
    );
    true
}

// Translated from 00493800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the sequence's `time` at +0xd0 to the engine's elapsed time. The
/// `TESAnimGroup::IsJumpingLoopAnim` map name is folded onto this body.
pub fn fn_00493800(e: &mut Engine, animation: Ptr) -> f64 {
    let seconds = e.call(SECONDS_SINCE_START, &args![]).f64();
    let offset = f64::from(e.mem.f32(animation.addr().wrapping_add(0xd0)));
    seconds + offset
}

// Translated from 005f2370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the movement, weapon, type, and flag selectors into an animation
/// group id, preserving the arithmetic and truncation performed by the PC.
pub fn fn_005f2370(_e: &mut Engine, movement: u32, weapon: u16, group: u32, flag: u8) -> u32 {
    let selectors = group
        .wrapping_add(movement.wrapping_mul(0x1000))
        .wrapping_add(u32::from(flag).wrapping_mul(0x8000));
    let low = (i32::from(weapon as i16)
        .wrapping_mul(0x100)
        .wrapping_add(i32::from(selectors as u16 as i16))) as u16;
    (selectors & 0xffff_0000) | u32::from(low)
}

// Translated from 005f23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the movement selector from bits 12 through 14 of the group id.
pub fn fn_005f23c0(_e: &mut Engine, group: u16) -> u32 {
    u32::from((group & 0x7000) >> 12)
}

// Translated from 005f2400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the weapon selector from bits 8 through 11 of the group id.
pub fn fn_005f2400(_e: &mut Engine, group: u16) -> u32 {
    u32::from((group & 0x0f00) >> 8)
}

// Translated from 005f2440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the low byte of an animation group id.
pub fn fn_005f2440(_e: &mut Engine, group: u16) -> u8 {
    group as u8
}

// Translated from 005f23a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the movement selector from the group's `sType` field.
pub fn fn_005f23a0(e: &mut Engine, this: Ptr) -> u32 {
    fn_005f23c0(e, e.mem.u16(this.addr().wrapping_add(0x10)))
}

// Translated from 005f23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the weapon selector from the group's `sType` field.
pub fn fn_005f23e0(e: &mut Engine, this: Ptr) -> u32 {
    fn_005f2400(e, e.mem.u16(this.addr().wrapping_add(0x10)))
}

// Translated from 005f2420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the low byte of the group's `sType` field.
pub fn fn_005f2420(e: &mut Engine, this: Ptr) -> u32 {
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    u32::from(fn_005f2440(e, group))
}

// Translated from 005f2610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a type-table category equals 9.
pub fn fn_005f2610(_e: &mut Engine, category: u32) -> bool {
    category == 9
}

// Translated from 005f26b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a type-table category equals 6.
pub fn fn_005f26b0(_e: &mut Engine, category: u32) -> bool {
    category == 6
}

// Translated from 005f25a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether an animation type is between 5 and 9, inclusive.
pub fn fn_005f25a0(_e: &mut Engine, value: i32) -> bool {
    (5..=9).contains(&value)
}

// Translated from 005f2700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether an animation type is between 0xb1 and 0xc7, inclusive.
pub fn fn_005f2700(_e: &mut Engine, value: i32) -> bool {
    (0xb1..=0xc7).contains(&value)
}

// Translated from 005f25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group's type-table category is 9; group id 0xff is
/// rejected before indexing the table.
pub fn fn_005f25d0(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let category = group_category(e, group);
    fn_005f2610(e, category)
}

// Translated from 005f26c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group's low-byte type is in the inclusive range 0x5c..=0x65.
pub fn fn_005f26c0(e: &mut Engine, this: Ptr) -> bool {
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    let ty = u32::from(fn_005f2440(e, group));
    (0x5c..=0x65).contains(&ty)
}

// Translated from 005f2540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the type is 0x4a..=0x4f or its category is 5..=9.
pub fn fn_005f2540(e: &mut Engine, group: u16) -> bool {
    let ty = u32::from(fn_005f2440(e, group));
    if ty == 0xff {
        false
    } else if (0x4a..=0x4f).contains(&ty) {
        true
    } else {
        let category = group_category(e, group) as i32;
        fn_005f25a0(e, category)
    }
}

// Translated from 005f2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group type is in the inclusive range 0x11..=0x16,
/// except the invalid id 0xff.
pub fn fn_005f2630(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let ty = u32::from(fn_005f2440(e, group));
    (0x11..=0x16).contains(&ty)
}

// Translated from 005f2670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a group's type-table category is 6, except the invalid id.
pub fn fn_005f2670(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let category = group_category(e, group);
    fn_005f26b0(e, category)
}

// Translated from 005f2720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests a 16-bit group id against the game's iron-sights type set.
pub fn fn_005f2720(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let ty = u32::from(fn_005f2440(e, group));
    fn_005f2750(e, ty)
}

// Translated from 005f2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns whether `ty` belongs to the 60-value iron-sights type set.
pub fn fn_005f2750(_e: &mut Engine, ty: u32) -> bool {
    matches!(ty, 0x14..=0x16 | 0x1d..=0x1f | 0x23..=0x25 | 0x29..=0x2b | 0x2f..=0x31 | 0x35..=0x37 | 0x3b..=0x3d | 0x41..=0x43 | 0x47..=0x49 | 0x4d..=0x4f | 0x53..=0x55 | 0x59..=0x5b | 0x69..=0x6b | 0x6f..=0x71 | 0x75..=0x77 | 0x7b..=0x7d | 0x81..=0x83 | 0x87..=0x89 | 0x8d..=0x8f | 0x93..=0x95)
}

// Translated from 005f4b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `cBlendFrames`.
pub fn fn_005f4b70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x29), value);
}

// Translated from 005f4b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `cBlendOutFrames`.
pub fn fn_005f4b90(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x2b), value);
}

// Translated from 005f4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pParentName`.
pub fn fn_005f4bb0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x30), value);
}

// Translated from 005f4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three float words from `value` into `speed`.
pub fn fn_005f4c40(e: &mut Engine, this: Ptr, value: Ptr) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(value.addr().wrapping_add(offset));
        e.mem.set_u32(this.addr().wrapping_add(0x1c + offset), word);
    }
}

// Translated from 005f4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the per-sound flag at entry `index` when it is below `soundCount`.
pub fn fn_005f4bd0(e: &mut Engine, this: Ptr, index: u32, value: u8) {
    if index < e.mem.u32(this.addr().wrapping_add(0x0c)) {
        let sounds = e.mem.u32(this.addr().wrapping_add(0x14));
        e.mem.set_u8(
            sounds
                .wrapping_add(0x0d)
                .wrapping_add(index.wrapping_mul(0x10)),
            value,
        );
    }
}

// Translated from 005f4c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the per-sound flag, or 0xff when `index` is outside `soundCount`.
pub fn fn_005f4c00(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    let count = e.mem.u32(this.addr().wrapping_add(0x0c));
    if index >= count {
        0xff
    } else {
        let sounds = e.mem.u32(this.addr().wrapping_add(0x14));
        e.mem.u8(sounds
            .wrapping_add(0x0d)
            .wrapping_add(index.wrapping_mul(0x10)))
    }
}

// Translated from 005f3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns a finite time for `index`, or zero after logging a negative or
/// out-of-range index and a NaN entry.
pub fn fn_005f3780(e: &mut Engine, this: Ptr, index: u32) -> f64 {
    let signed_index = index as i32;
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    let ty = u32::from(fn_005f2440(e, group));
    let type_name = e
        .mem
        .u32(TYPE_NAME_TABLE.wrapping_add(ty.wrapping_mul(0x24)));
    if signed_index < 0 {
        e.call(
            INVALID_ACTION_LOG,
            &args![LOG_ANIM_GROUP_ACTION_LOW, index, type_name],
        );
        return 0.0;
    }

    let count = e.mem.u32(this.addr().wrapping_add(0x14));
    if index >= count {
        e.call(
            INVALID_ACTION_LOG,
            &args![
                LOG_ANIM_GROUP_ACTION_HIGH,
                index,
                count,
                type_name,
                u32::from(group)
            ],
        );
        return 0.0;
    }

    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    if times == 0 {
        return 0.0;
    }
    let time = e.mem.f32(times.wrapping_add(index.wrapping_mul(4)));
    if e.call(ISNAN, &args![f64::from(time)]).bool() {
        e.call(
            INVALID_ACTION_LOG,
            &args![LOG_ANIM_GROUP_TIME_NAN, index, u32::from(group)],
        );
        0.0
    } else {
        f64::from(time)
    }
}

// Translated from 005f3870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the next line-feed and skips consecutive CR/LF bytes; returns null
/// for a missing line-feed or at the end of the string.
pub fn fn_005f3870(e: &mut Engine, string: Ptr) -> Ptr {
    let found = e.call(STRCHR, &args![string, 10u32]).u32();
    if found == 0 {
        return Ptr::NULL;
    }
    let mut next = found;
    loop {
        let byte = e.mem.u8(next);
        if byte == 0 || (byte != b'\r' && byte != b'\n') {
            break;
        }
        next = next.wrapping_add(1);
    }
    if e.mem.u8(next) == 0 {
        Ptr::NULL
    } else {
        Ptr::new(next)
    }
}

// Translated from 005f38d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses move, weapon, and prefix selectors from the filename component
/// after its final backslash. Every string comparison uses the game's
/// installed tables and CRT routines.
pub fn fn_005f38d0(
    e: &mut Engine,
    filename: Ptr,
    out_move: Ptr,
    out_weapon: Ptr,
    out_prefixed: Ptr,
) {
    let base = e.call(STRRCHR, &args![filename, 0x5cu32]).u32();
    e.mem.set_u8(out_prefixed.addr(), 0);
    e.mem.set_u32(out_move.addr(), 0);
    e.mem.set_u32(out_weapon.addr(), 0);
    if base == 0 {
        return;
    }
    let mut cursor = base.wrapping_add(1);
    let prefix_len = e.call(STRLEN, &args![PA_PREFIX]).u32();
    if e.call(STRNICMP, &args![cursor, PA_PREFIX, prefix_len])
        .i32()
        == 0
    {
        e.mem.set_u8(out_prefixed.addr(), 1);
        let prefix_len = e.call(STRLEN, &args![PA_PREFIX]).u32();
        cursor = cursor.wrapping_add(prefix_len);
    }
    for index in 1..4u32 {
        let name_address = MOVE_NAME_TABLE.wrapping_add(index.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        let len = e.call(STRLEN, &args![name]).u32();
        let compare_name = e.mem.u32(name_address);
        if e.call(STRNICMP, &args![cursor, compare_name, len]).i32() == 0 {
            e.mem.set_u32(out_move.addr(), index);
            let name = e.mem.u32(name_address);
            let len = e.call(STRLEN, &args![name]).u32();
            cursor = cursor.wrapping_add(len);
            break;
        }
    }
    for index in 1..12u32 {
        let name_address = WEAPON_NAME_TABLE.wrapping_add(index.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        let len = e.call(STRLEN, &args![name]).u32();
        let compare_name = e.mem.u32(name_address);
        if e.call(STRNICMP, &args![cursor, compare_name, len]).i32() == 0 {
            e.mem.set_u32(out_weapon.addr(), index);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ret;

    #[test]
    fn animation_group_packing_matches_native_vectors() {
        let mut e = Engine::new();
        let cases = [
            ([0, 0, 0, 0], 0x0000_0000),
            ([7, 15, 255, 1], 0x0000_ffff),
            ([0, 1, 65_535, 0], 0x0000_00ff),
            ([0, 65_535, 65_536, 0], 0x0001_ff00),
            ([u32::MAX, 0x1234, 0xdead_beef, 255], 0xdf2d_62ef),
            ([524_288, 0x1234_5678, u32::MAX, 128], 0x803f_77ff),
        ];
        for (input, expected) in cases {
            assert_eq!(
                e.call(0x005f_2370, &input).u32(),
                expected,
                "input={input:?}"
            );
        }
    }

    #[test]
    fn aim_action_matches_native_type_vectors() {
        let mut e = Engine::new();
        for (group, expected) in [
            (16, false),
            (17, true),
            (22, true),
            (23, false),
            (255, false),
            (273, true),
            (65_535, false),
        ] {
            assert_eq!(
                e.call(0x005f_2630, &[group]).bool(),
                expected,
                "group={group}"
            );
        }
    }

    #[test]
    fn note_resize_reloads_fields_after_allocator_callbacks() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x2000);
        e.map(0x2000, 0x40);
        e.map(0x3000, 0x40);
        e.map(0x4000, 0x40);
        e.map(0x6000, 0x40);

        e.register_double(OPERATOR_NEW, |_, _| Ret {
            eax: 0x3000,
            ..Ret::default()
        });
        let mut copy_count = 0;
        e.register_double(MEMCPY, move |e, _| {
            copy_count += 1;
            if copy_count == 1 {
                e.mem.set_u32(this + 0x34, 5);
                e.mem.set_u32(this + 0x38, 0x2222);
            }
            Ret::default()
        });
        e.register_double(REALLOC, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        e.register_double(OPERATOR_DELETE, move |e, _| {
            e.mem.set_u32(this + 0x34, 4);
            e.mem.set_u32(this + 0x38, 0x6000);
            Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x005f_2820, &[this, 7]);

        let log = e.call_log.take().unwrap();
        let realloc_args = log.iter().find(|(addr, _)| *addr == REALLOC).unwrap();
        assert_eq!(realloc_args.1, [0x2222, 5 * 0x10]);
        let copies: Vec<_> = log.iter().filter(|(addr, _)| *addr == MEMCPY).collect();
        assert_eq!(copies[0].1, [0x3000, 0x2000, 3 * 0x10]);
        assert_eq!(copies[1].1, [0x4000, 0x3000, 3 * 0x10]);
        assert_eq!(e.mem.u32(this + 0x34), 4);
        assert_eq!(e.mem.u32(this + 0x38), 0x6000);
        assert_eq!(e.mem.u32(0x6030 + 0xc), 0);
        assert_eq!(e.mem.u32(0x6030), 0);
        assert_eq!(e.mem.u8(0x6030 + 4), 0xfa);
        assert_eq!(e.mem.u32(0x6030 + 8), 1.0f32.to_bits());
    }

    #[test]
    fn frame_count_uses_section_stride_and_reloads_after_allocator_callbacks() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x18, 0x5000);
        e.map(0x5000, 0x40);
        e.map(GROUP_TYPE_TABLE, 4 * 0x24);
        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 2);
        e.map(SECTION_NAME_TABLE, 0x200);
        e.map(0x2000, 0x20);
        e.mem.set_u32(SECTION_NAME_TABLE + 2 * 4, 0x2000);
        e.mem.set_u8(0x2000, b'A');
        e.mem.set_u32(SECTION_NAME_TABLE + 0x2c + 2 * 4, 0x2010);
        e.mem.set_u8(0x2010, 0);
        // The incorrect `(row * stride + category) * 4` expression would
        // read this non-empty decoy instead of the row-one terminator.
        e.mem.set_u32(SECTION_NAME_TABLE + (0x2c + 2) * 4, 0x2000);

        e.register_double(OPERATOR_DELETE, move |e, _| {
            e.mem.set_u32(this + 0x14, 3);
            Ret::default()
        });
        e.register_double(OPERATOR_NEW, move |e, args| {
            assert_eq!(args, [12]);
            e.mem.set_u32(this + 0x14, 4);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(MEMSET, |_, args| {
            assert_eq!(args, [0x6000, 0, 16]);
            Ret::default()
        });

        e.call(0x005f_2450, &[this, 3]);

        assert_eq!(e.mem.u32(this + 0x14), 4);
        assert_eq!(e.mem.u32(this + 0x18), 0x6000);
    }

    #[test]
    fn animation_times_and_note_times_quiet_signaling_nan_on_x87_store() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, 0x2000);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.map(0x2000, 8);
        e.map(0x3000, 0x20);

        assert!(e.call(0x005f_3740, &[this, 1, 0x7f80_0001]).bool());
        e.call(0x005f_29f0, &[this, 1, 0x7f80_0001]);

        assert_eq!(e.mem.u32(0x2004), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x3010), 0x7fc0_0001);
    }

    #[test]
    fn constructor_does_not_destroy_its_base_on_normal_return() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.map(0, 1);
        e.map(GROUP_TYPE_TABLE, 0x24);
        e.map(SECTION_NAME_TABLE, 4);
        e.map(0x011f_426c, 12);
        e.register_double(0x0049_68b0, |_, _| Ret::default());
        e.register_double(0x0068_15c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);

        assert_eq!(e.call(0x005f_21c0, &[this, 0]).u32(), this);

        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, _)| *address == 0x0049_68b0));
        assert!(calls.iter().any(|(address, _)| *address == 0x0068_15c0));
        assert!(!calls.iter().any(|(address, _)| *address == 0x0049_6910));
    }

    #[test]
    fn destructor_frees_arrays_before_destroying_its_base() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.mem.set_u32(this + 0x18, 0x2000);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.register_double(OPERATOR_DELETE, |_, _| Ret::default());
        e.register_double(0x0049_6910, |_, _| Ret::default());
        e.call_log = Some(vec![]);

        e.call(0x005f_22d0, &[this]);

        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        let calls = e.call_log.take().unwrap();
        let calls: Vec<_> = calls
            .into_iter()
            .filter(|(address, _)| *address == OPERATOR_DELETE || *address == 0x0049_6910)
            .collect();
        assert_eq!(calls[0], (OPERATOR_DELETE, [0x2000].to_vec()));
        assert_eq!(calls[1], (OPERATOR_DELETE, [0x3000].to_vec()));
        assert_eq!(calls[2], (0x0049_6910, [this].to_vec()));
    }

    #[test]
    fn filename_parser_reloads_table_values_and_prefix_length_after_callbacks() {
        let mut e = Engine::new();
        e.map(PA_PREFIX, 0x20);
        e.map(MOVE_NAME_TABLE, 0x30);
        e.map(WEAPON_NAME_TABLE, 0x30);
        e.map(0x3000, 12);
        e.mem.set_u32(MOVE_NAME_TABLE + 4, 0x2100);
        e.mem.set_u32(WEAPON_NAME_TABLE + 4, 0x2300);
        e.register_double(STRRCHR, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        let mut prefix_lengths = 0;
        e.register_double(STRLEN, move |e, args| {
            let length = match args[0] {
                PA_PREFIX => {
                    prefix_lengths += 1;
                    if prefix_lengths == 1 {
                        2
                    } else {
                        3
                    }
                }
                0x2100 => {
                    e.mem.set_u32(MOVE_NAME_TABLE + 4, 0x2200);
                    2
                }
                0x2200 => 3,
                0x2300 => 2,
                address => panic!("unexpected STRLEN argument {address:#x}"),
            };
            Ret {
                eax: length,
                ..Ret::default()
            }
        });
        e.register_double(STRNICMP, |_, _| Ret {
            eax: 0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x005f_38d0, &[0x4000, 0x3000, 0x3004, 0x3008]);

        assert_eq!(e.mem.u32(0x3000), 1);
        assert_eq!(e.mem.u32(0x3004), 1);
        assert_eq!(e.mem.u8(0x3008), 1);
        let calls = e.call_log.take().unwrap();
        let lengths: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == STRLEN)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(lengths, [PA_PREFIX, PA_PREFIX, 0x2100, 0x2200, 0x2300]);
        let comparisons: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == STRNICMP)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(comparisons[0], [0x4001, PA_PREFIX, 2]);
        assert_eq!(comparisons[1], [0x4004, 0x2200, 2]);
        assert_eq!(comparisons[2], [0x4007, 0x2300, 2]);
    }

    #[test]
    fn group_selectors_and_category_predicates_match_pc_ranges() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.map(GROUP_TYPE_TABLE, 0x24 * 8);
        e.mem.set_u16(this + 0x10, 0x5c32);
        assert_eq!(e.call(0x005f_23a0, &[this]).u32(), 5);
        assert_eq!(e.call(0x005f_23c0, &[0x5c32]).u32(), 5);
        assert_eq!(e.call(0x005f_23e0, &[this]).u32(), 12);
        assert_eq!(e.call(0x005f_2400, &[0x5c32]).u32(), 12);
        assert_eq!(e.call(0x005f_2420, &[this]).u32(), 0x32);
        assert_eq!(e.call(0x005f_2440, &[0x5c32]).u32(), 0x32);
        e.mem.set_u16(this + 0x10, 0x5c5c);
        assert!(e.call(0x005f_26c0, &[this]).bool());

        for (value, expected) in [(4, false), (5, true), (9, true), (10, false)] {
            assert_eq!(e.call(0x005f_25a0, &[value]).bool(), expected);
        }
        for (value, expected) in [(0xb0, false), (0xb1, true), (0xc7, true), (0xc8, false)] {
            assert_eq!(e.call(0x005f_2700, &[value]).bool(), expected);
        }
        assert!(e.call(0x005f_2610, &[9]).bool());
        assert!(!e.call(0x005f_2610, &[8]).bool());
        assert!(e.call(0x005f_26b0, &[6]).bool());
        assert!(!e.call(0x005f_26b0, &[5]).bool());

        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 9);
        assert!(e.call(0x005f_25d0, &[3]).bool());
        assert!(!e.call(0x005f_25d0, &[0xff]).bool());
        assert!(e.call(0x005f_2540, &[0x4a]).bool());
        assert!(e.call(0x005f_2540, &[3]).bool());
        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 6);
        assert!(e.call(0x005f_2670, &[3]).bool());
        assert!(!e.call(0x005f_2670, &[0xff]).bool());
        assert!(e.call(0x005f_2720, &[0x14]).bool());
        assert!(!e.call(0x005f_2720, &[0xff]).bool());
        assert!(e.call(0x005f_2750, &[0x14]).bool());
        assert!(!e.call(0x005f_2750, &[0x18]).bool());
    }

    #[test]
    fn sequence_clock_time_lookup_and_line_skip_use_native_helpers() {
        let mut e = Engine::new();
        let animation = 0x1000;
        e.map(animation, 0xe0);
        e.mem.set_f32(animation + 0xd0, 1.25);
        e.register_double(SECONDS_SINCE_START, |_, _| Ret {
            st0: 42.5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0049_3800, &[animation]).f64(), 43.75);

        e.map(GROUP_TYPE_TABLE, 0x24 * 4);
        e.map(TYPE_NAME_TABLE, 0x24 * 4);
        e.map(0x3000, 1);
        e.mem.set_u16(animation + 0x10, 3);
        e.mem.set_u32(TYPE_NAME_TABLE + 3 * 0x24, 0x3000);
        e.mem.set_u32(animation + 0x14, 2);
        e.mem.set_u32(animation + 0x18, 0x4000);
        e.map(0x4000, 8);
        e.mem.set_f32(0x4000 + 4, 1.5);
        e.register_double(ISNAN, |_, _| Ret::default());
        assert_eq!(e.call(0x005f_3780, &[animation, 1]).f64(), 1.5);

        e.map(0x2000, 8);
        e.mem.set_u8(0x2000, b'a');
        e.mem.set_u8(0x2001, b'\n');
        e.mem.set_u8(0x2002, b'\r');
        e.mem.set_u8(0x2003, b'b');
        e.register_double(STRCHR, |_, _| Ret {
            eax: 0x2001,
            ..Ret::default()
        });
        assert_eq!(e.call(0x005f_3870, &[0x2000]).u32(), 0x2003);
    }

    #[test]
    fn note_mutators_and_sound_flags_write_expected_fields() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.mem.set_u32(this + 0x0c, 2);
        e.mem.set_u32(this + 0x14, 0x4000);
        e.map(0x3000, 0x20);
        e.map(0x4000, 0x20);
        e.map(NOTE_PRIORITY_SCALE, 8);
        e.set_global(NOTE_PRIORITY_SCALE, 250.0f64);

        e.call(0x005f_2a30, &[this, 0, 0xdead_beef]);
        e.call(0x005f_2a70, &[this, 1, 0x12, 2.5f32.to_bits()]);
        e.call(0x005f_2ac0, &[this, 1, 0.02f32.to_bits()]);
        e.call(0x005f_2b20, &[this, 1, 3.25f32.to_bits()]);
        e.call(0x005f_4bd0, &[this, 1, 0x7e]);
        assert_eq!(e.call(0x005f_4c00, &[this, 1]).u32(), 0x7e);
        assert_eq!(e.call(0x005f_4c00, &[this, 2]).u32(), 0xff);

        assert_eq!(e.mem.u32(0x3000 + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u8(0x3010 + 4), 4);
        assert_eq!(e.mem.u32(0x3010 + 8), 3.25f32.to_bits());
        assert_eq!(e.mem.u8(0x4000 + 0x0d + 0x10), 0x7e);
    }

    #[test]
    fn blend_parent_and_speed_setters_use_class_offsets() {
        let mut e = Engine::new();
        let this = 0x1000;
        let source = 0x2000;
        e.map(this, 0x40);
        e.map(source, 12);
        e.mem.set_u32(source, 0x3f80_0000);
        e.mem.set_u32(source + 4, 0x4000_0000);
        e.mem.set_u32(source + 8, 0x4040_0000);
        e.call(0x005f_4b70, &[this, 0x123]);
        e.call(0x005f_4b90, &[this, 0x45]);
        e.call(0x005f_4bb0, &[this, 0x8765_4321]);
        e.call(0x005f_4c40, &[this, source]);

        assert_eq!(e.mem.u8(this + 0x29), 0x23);
        assert_eq!(e.mem.u8(this + 0x2b), 0x45);
        assert_eq!(e.mem.u32(this + 0x30), 0x8765_4321);
        assert_eq!(e.mem.u32(this + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x20), 0x4000_0000);
        assert_eq!(e.mem.u32(this + 0x24), 0x4040_0000);
    }

    #[test]
    fn scalar_deleting_destructor_runs_sized_delete_when_flag_is_set() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.register_double(OPERATOR_DELETE, |_, _| Ret::default());
        e.register_double(0x0049_6910, |_, _| Ret::default());
        e.register_double(0x00aa_1460, move |_, args| {
            assert_eq!(args, [this, 0x3c]);
            Ret::default()
        });
        assert_eq!(e.call(0x005f_22a0, &[this, 1]).u32(), this);
    }
}
