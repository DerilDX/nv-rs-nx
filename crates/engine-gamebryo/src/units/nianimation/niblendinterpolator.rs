//! `NiAnimation/niblendinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_INTERPOLATOR_RTTI: u32 = 0x011f_36fc;

// Translated from 00a36960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_INTERPOLATOR_RTTI)
}

// Translated from 00a37670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a37170`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_7170;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_decrement};

/// `NiBlendInterpolator`'s vtable (written by its constructor and
/// destructor).
const NI_BLEND_INTERPOLATOR_VTABLE: u32 = 0x0109_697c;
/// `NiInterpolator::NiInterpolator` (`00a56ff0`, `thiscall`) and the
/// `NiInterpolator` destructor body (`00a57010`, reached by a tail jump).
const NI_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_6ff0;
const NI_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a5_7010;
/// `NiInterpolator::LoadBinary` (`00a88a40`, `RET 4`).
const NI_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a8_8a40;
/// `NiObject::ProcessClone` (`00a5d430`, `RET 4`).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiKeyBasedInterpolator::CopyMembers` (`00aa1000`, `RET 8`).
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_1000;
/// The clone map lookup (`00a53c30`, `thiscall (map, key, &value)`,
/// answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiBlendInterpolator::InterpArrayItem::LoadBinary` (`00a377b0`,
/// `thiscall (item, stream)`).
const ITEM_LOAD_BINARY: u32 = 0x00a3_77b0;
/// `NiStream::ResolveLinkID` (`00a64620`, `thiscall`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The memory manager's allocate and free (`00aa3e40`, `00aa4060`,
/// `thiscall` on the manager at `011f6238`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The compiler's vector constructor iterator (`00401050`, `stdcall
/// (array, element size, count, constructor)`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `NiInterpolator::INVALID_FLOAT` (Xbox PDB static; `-FLT_MAX`).
const INVALID_FLOAT: u32 = 0x0109_696c;
/// `NiBlendInterpolator::ms_ucArrayGrowBy` (Xbox PDB static, a byte).
const ARRAY_GROW_BY: u32 = 0x011a_83ff;
/// `-FLT_MAX` (`double`, `010241b0`), the unset sum of weights.
const LOWEST: u32 = 0x0102_41b0;
/// `-1.0` (`float`, `01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The `float` `1.0`, as the exe's `FLD1` / `FSTP` stores it.
const ONE_BITS: u32 = 0x3f80_0000;

/// `m_uFlags` (Xbox PDB) +0xC and its bits: 1 manager controlled, 2 only
/// the highest weight, 4 weights out of date.
const FLAGS: u32 = 0xc;
const MANAGER_CONTROLLED: u8 = 1;
const ONLY_USE_HIGHEST_WEIGHT: u8 = 2;
const COMPUTE_NORMALIZED_WEIGHTS: u8 = 4;
/// `m_ucArraySize`, `m_ucInterpCount`, `m_ucSingleIdx` (Xbox PDB).
const ARRAY_SIZE: u32 = 0xd;
const INTERP_COUNT: u32 = 0xe;
const SINGLE_INDEX: u32 = 0xf;
/// `m_cHighPriority`, `m_cNextHighPriority` (Xbox PDB).
const HIGH_PRIORITY: u32 = 0x10;
const NEXT_HIGH_PRIORITY: u32 = 0x11;
/// `m_pkInterpArray`, `m_pkSingleInterpolator` (Xbox PDB).
const INTERP_ARRAY: u32 = 0x14;
const SINGLE_INTERPOLATOR: u32 = 0x18;
/// `m_fWeightThreshold`, `m_fSingleTime`, `m_fHighSumOfWeights`,
/// `m_fNextHighSumOfWeights`, `m_fHighEaseSpinner` (Xbox PDB).
const WEIGHT_THRESHOLD: u32 = 0x1c;
const SINGLE_TIME: u32 = 0x20;
const HIGH_SUM_OF_WEIGHTS: u32 = 0x24;
const NEXT_HIGH_SUM_OF_WEIGHTS: u32 = 0x28;
const HIGH_EASE_SPINNER: u32 = 0x2c;

/// `NiBlendInterpolator::InterpArrayItem` (Xbox PDB), 0x18 bytes:
/// `m_spInterpolator` +0, `m_fWeight` +4, `m_fNormalizedWeight` +8,
/// `m_cPriority` +0xC, `m_fEaseSpinner` +0x10, `m_fUpdateTime` +0x14.
const ITEM_SIZE: u32 = 0x18;
const ITEM_WEIGHT: u32 = 0x4;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_PRIORITY: u32 = 0xc;
const ITEM_EASE_SPINNER: u32 = 0x10;
const ITEM_UPDATE_TIME: u32 = 0x14;

/// The address of item `index` of the interpolator's array.
fn item(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(INTERP_ARRAY))
        .wrapping_add(index.wrapping_mul(ITEM_SIZE))
}

fn byte(e: &Engine, this: Ptr, offset: u32) -> u8 {
    e.mem.u8(this.addr().wrapping_add(offset))
}

fn set_byte(e: &mut Engine, this: Ptr, offset: u32, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(offset), value);
}

/// `FLD dword` / `FSTP dword` of the float at `from` into `to`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// `FLD INVALID_FLOAT` / `FSTP` into `to`.
fn store_invalid(e: &mut Engine, to: u32) {
    copy_float(e, INVALID_FLOAT, to);
}

/// The compiler's `new InterpArrayItem[count]`: `count * 0x18 + 4` bytes
/// (all ones on overflow) from the memory manager, the count in the first
/// word and every item constructed (`00a370a0`) by the vector constructor
/// iterator. The array (after the count), or 0 when the allocation failed.
fn new_item_array(e: &mut Engine, count: u32) -> u32 {
    let size = count
        .checked_mul(ITEM_SIZE)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    let array = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![array, ITEM_SIZE, count, 0x00a3_70a0u32],
    );
    array
}

/// The compiler's `delete[]` of an item array (non-null): each item's
/// interpolator released, last first (`InterlockedDecrement` of its count
/// at +4; vtable slot +4 when it reaches 0), for as many items as the count
/// word says while the count minus one is not negative; then the block
/// freed through the memory manager.
fn delete_item_array(e: &mut Engine, array: u32) {
    let block = array.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut remaining = count.wrapping_sub(1) as i32;
    let mut at = array.wrapping_add(count.wrapping_mul(ITEM_SIZE));
    while remaining >= 0 {
        at = at.wrapping_sub(ITEM_SIZE);
        let interpolator = e.mem.u32(at);
        if interpolator != 0 && interlocked_decrement(e, interpolator.wrapping_add(4)) == 0 {
            e.vcall(interpolator, 4, &args![]);
        }
        remaining -= 1;
    }
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a370a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::InterpArrayItem`'s constructor (no Ghidra
/// function; the vector constructor iterator's callback): clears
/// `m_spInterpolator` and answers `this`.
pub fn fn_00a370a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 00a371f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `InterpArrayItem`'s assignment from `other`:
/// the reference-counted interpolator (released and counted as a smart
/// pointer), then the weight, normalized weight, priority, ease spinner and
/// update time. Answers `this`.
pub fn fn_00a371f0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let interpolator = e.mem.u32(other.addr());
    assign_ref_counted(e, this.addr(), interpolator);
    let (to, from) = (this.addr(), other.addr());
    copy_float(e, from + ITEM_WEIGHT, to + ITEM_WEIGHT);
    copy_float(
        e,
        from + ITEM_NORMALIZED_WEIGHT,
        to + ITEM_NORMALIZED_WEIGHT,
    );
    let priority = e.mem.u8(from + ITEM_PRIORITY);
    e.mem.set_u8(to + ITEM_PRIORITY, priority);
    copy_float(e, from + ITEM_EASE_SPINNER, to + ITEM_EASE_SPINNER);
    copy_float(e, from + ITEM_UPDATE_TIME, to + ITEM_UPDATE_TIME);
    this
}

// Translated from 00a370b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::NiBlendInterpolator` (Xbox PDB): the
/// `NiInterpolator` constructor, then the weight threshold, the four
/// cached sums and times `INVALID_FLOAT`, both priorities `0x80`, the
/// class's vtable, the array size, no interpolators (count 0, single index
/// `0xFF`, no array, no single interpolator), the manager-controlled flag
/// (`flags = manager_controlled != 0`), and with a non-zero size a new item
/// array of that size. Answers `this`.
pub fn ni_blend_interpolator_ni_blend_interpolator(
    e: &mut Engine,
    this: Ptr,
    manager_controlled: u8,
    weight_threshold: f32,
    array_size: u8,
) -> Ptr {
    e.call(NI_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.mem.set_u32(
        at(WEIGHT_THRESHOLD),
        x87_quiet_bits(weight_threshold.to_bits()),
    );
    store_invalid(e, at(SINGLE_TIME));
    store_invalid(e, at(HIGH_SUM_OF_WEIGHTS));
    set_byte(e, this, HIGH_PRIORITY, 0x80);
    set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
    store_invalid(e, at(NEXT_HIGH_SUM_OF_WEIGHTS));
    e.mem.set_u32(this.addr(), NI_BLEND_INTERPOLATOR_VTABLE);
    store_invalid(e, at(HIGH_EASE_SPINNER));
    set_byte(e, this, ARRAY_SIZE, array_size);
    set_byte(e, this, INTERP_COUNT, 0);
    set_byte(e, this, SINGLE_INDEX, 0xff);
    e.mem.set_u32(at(INTERP_ARRAY), 0);
    e.mem.set_u32(at(SINGLE_INTERPOLATOR), 0);
    set_byte(e, this, FLAGS, u8::from(manager_controlled != 0));
    if array_size > 0 {
        let array = new_item_array(e, u32::from(array_size));
        e.mem.set_u32(at(INTERP_ARRAY), array);
    }
    this
}

// Translated from 00a37170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::~NiBlendInterpolator` (Xbox PDB): puts the class's
/// vtable back, deletes the item array when there is one (releasing each
/// item's interpolator), then runs the `NiInterpolator` destructor.
pub fn ni_blend_interpolator_destructor(e: &mut Engine, this: Ptr) {
    let array = e.mem.u32(this.addr().wrapping_add(INTERP_ARRAY));
    e.mem.set_u32(this.addr(), NI_BLEND_INTERPOLATOR_VTABLE);
    if array != 0 {
        delete_item_array(e, array);
    }
    e.call(NI_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a368f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetHighestWeightedIndex` (Xbox PDB): with exactly
/// one interpolator its single index; otherwise the index of the first item
/// with the largest normalized weight above 0 (`0xFF` when none is above 0).
pub fn ni_blend_interpolator_get_highest_weighted_index(e: &mut Engine, this: Ptr) -> u8 {
    if byte(e, this, INTERP_COUNT) == 1 {
        return byte(e, this, SINGLE_INDEX);
    }
    let mut highest = 0.0f64;
    let mut index = 0xffu8;
    let size = byte(e, this, ARRAY_SIZE);
    let mut i = 0u8;
    while i < size {
        let weight = x87_load32(
            e.mem
                .u32(item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT),
        );
        if highest < weight {
            highest = weight;
            index = i;
        }
        i = i.wrapping_add(1);
    }
    index
}

/// The priorities' update for an item of priority `priority` being added:
/// above the next-highest it becomes the highest (the old highest moving to
/// next-highest) or, below the highest, the next-highest.
fn raise_priorities(e: &mut Engine, this: Ptr, priority: i8) {
    if priority <= byte(e, this, NEXT_HIGH_PRIORITY) as i8 {
        return;
    }
    let high = byte(e, this, HIGH_PRIORITY) as i8;
    if priority > high {
        set_byte(e, this, NEXT_HIGH_PRIORITY, high as u8);
        set_byte(e, this, HIGH_PRIORITY, priority as u8);
    } else if priority != high {
        set_byte(e, this, NEXT_HIGH_PRIORITY, priority as u8);
    }
}

/// Marks the weights out of date and the three cached sums unset (the end
/// of `AddInterpInfo` and `RemoveInterpInfo`).
fn invalidate_weights(e: &mut Engine, this: Ptr) {
    let flags = byte(e, this, FLAGS) | COMPUTE_NORMALIZED_WEIGHTS;
    set_byte(e, this, FLAGS, flags);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    store_invalid(e, at(HIGH_SUM_OF_WEIGHTS));
    store_invalid(e, at(NEXT_HIGH_SUM_OF_WEIGHTS));
    store_invalid(e, at(HIGH_EASE_SPINNER));
}

// Translated from 00a36970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::AddInterpInfo` (Xbox PDB): puts `interpolator`
/// (reference counted) with `weight`, `priority` and `ease_spinner` into
/// the first free item (normalized weight 0, update time `INVALID_FLOAT`),
/// growing the array through `ReallocateArrays` (vtable +0xE8) when all are
/// used (`0xFF` when that fails); updates the priorities; with it the only
/// interpolator it becomes the single one, otherwise there is none (single
/// index `0xFF`, single time `INVALID_FLOAT`); marks the weights out of
/// date. Answers the item's index.
pub fn ni_blend_interpolator_add_interp_info(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    weight: f32,
    priority: i8,
    ease_spinner: f32,
) -> u8 {
    let size = byte(e, this, ARRAY_SIZE);
    let mut index = 0u8;
    while index < size {
        if e.mem.u32(item(e, this, u32::from(index))) == 0 {
            break;
        }
        index = index.wrapping_add(1);
    }
    if index == size && !e.vcall(this.addr(), 0xe8, &args![]).bool() {
        return 0xff;
    }
    let slot = item(e, this, u32::from(index));
    assign_ref_counted(e, slot, interpolator.addr());
    e.mem
        .set_u32(slot + ITEM_WEIGHT, x87_quiet_bits(weight.to_bits()));
    e.mem.set_u8(slot + ITEM_PRIORITY, priority as u8);
    e.mem.set_u32(slot + ITEM_NORMALIZED_WEIGHT, 0);
    e.mem.set_u32(
        slot + ITEM_EASE_SPINNER,
        x87_quiet_bits(ease_spinner.to_bits()),
    );
    store_invalid(e, slot + ITEM_UPDATE_TIME);
    raise_priorities(e, this, priority);
    let count = byte(e, this, INTERP_COUNT).wrapping_add(1);
    set_byte(e, this, INTERP_COUNT, count);
    if count == 1 {
        set_byte(e, this, SINGLE_INDEX, index);
        e.mem
            .set_u32(this.addr() + SINGLE_INTERPOLATOR, interpolator.addr());
    } else {
        set_byte(e, this, SINGLE_INDEX, 0xff);
        store_invalid(e, this.addr() + SINGLE_TIME);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, 0);
    }
    invalidate_weights(e, this);
    index
}

// Translated from 00a36a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::RemoveInterpInfo` (Xbox PDB): releases item
/// `index`'s interpolator (cleared); when its priority was the highest or
/// next-highest, both are recomputed from the items still holding one
/// (from `0x80`); one fewer interpolator: with one left it becomes the
/// single one (for an array of two the other index, `index == 0`, otherwise
/// the first used item; the single index is left as it was when none is
/// found) with its update time as the single time; with none left the
/// priorities are `0x80`, the single index `0xFF`, no single interpolator,
/// single time `INVALID_FLOAT`. The weights are marked out of date.
pub fn ni_blend_interpolator_remove_interp_info(e: &mut Engine, this: Ptr, index: u8) {
    let slot = item(e, this, u32::from(index));
    let interpolator = e.mem.u32(slot);
    let priority = e.mem.u8(slot + ITEM_PRIORITY);
    if interpolator != 0 {
        if interlocked_decrement(e, interpolator.wrapping_add(4)) == 0 {
            e.vcall(interpolator, 4, &args![]);
        }
        e.mem.set_u32(slot, 0);
    }
    if priority == byte(e, this, HIGH_PRIORITY) || priority == byte(e, this, NEXT_HIGH_PRIORITY) {
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let other = item(e, this, u32::from(i));
            if e.mem.u32(other) != 0 {
                let p = e.mem.u8(other + ITEM_PRIORITY) as i8;
                if p > byte(e, this, NEXT_HIGH_PRIORITY) as i8 {
                    let high = byte(e, this, HIGH_PRIORITY) as i8;
                    if p > high {
                        set_byte(e, this, NEXT_HIGH_PRIORITY, high as u8);
                        let p = e.mem.u8(other + ITEM_PRIORITY);
                        set_byte(e, this, HIGH_PRIORITY, p);
                    } else if p < high {
                        set_byte(e, this, NEXT_HIGH_PRIORITY, p as u8);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let count = byte(e, this, INTERP_COUNT).wrapping_add(0xff);
    set_byte(e, this, INTERP_COUNT, count);
    if count == 1 {
        let size = byte(e, this, ARRAY_SIZE);
        if size == 2 {
            set_byte(e, this, SINGLE_INDEX, u8::from(index == 0));
        } else {
            let mut i = 0u8;
            while i < size {
                if e.mem.u32(item(e, this, u32::from(i))) != 0 {
                    set_byte(e, this, SINGLE_INDEX, i);
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        let single = item(e, this, u32::from(byte(e, this, SINGLE_INDEX)));
        let interpolator = e.mem.u32(single);
        e.mem
            .set_u32(this.addr() + SINGLE_INTERPOLATOR, interpolator);
        copy_float(e, single + ITEM_UPDATE_TIME, this.addr() + SINGLE_TIME);
    } else if count == 0 {
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        set_byte(e, this, SINGLE_INDEX, 0xff);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, 0);
        store_invalid(e, this.addr() + SINGLE_TIME);
    }
    invalidate_weights(e, this);
}

// Translated from 00a36bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ComputeNormalizedWeights`' case of exactly
/// two interpolators): takes the first two used items A and B (the exe
/// reads through null when there are fewer) and their weights times ease
/// spinners `wa`, `wb` (rounded to floats). Both 0: both normalized weights
/// 0. Otherwise by priority: A's higher with A's spinner 1 gives (1, 0),
/// else the weights `wa * ea` and `wb * (1 - ea)` normalized; B's higher
/// the same mirrored ((0, 1) for B's spinner 1, else `wa * (1 - eb)` and
/// `eb * wb`); equal priorities `wa` and `wb` normalized (each through the
/// reciprocal of the sum, rounded to a float). Then with "only the highest
/// weight" (flag 2) the larger gets 1 and the other 0 (A on a tie, B when
/// unordered); otherwise with a positive threshold a weight below it is
/// zeroed, and the other becomes 1 when either was.
pub fn fn_00a36bd0(e: &mut Engine, this: Ptr) {
    let size = byte(e, this, ARRAY_SIZE);
    let mut first = 0u32;
    let mut second = 0u32;
    let mut i = 0u8;
    while i < size {
        let candidate = item(e, this, u32::from(i));
        if e.mem.u32(candidate) != 0 {
            if first != 0 {
                second = candidate;
                break;
            }
            first = candidate;
        }
        i = i.wrapping_add(1);
    }
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let operand = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let wa = x87_store32(x87_mul(
        load(e, first + ITEM_WEIGHT),
        operand(e, first + ITEM_EASE_SPINNER),
    ));
    let wb = x87_store32(x87_mul(
        load(e, second + ITEM_WEIGHT),
        operand(e, second + ITEM_EASE_SPINNER),
    ));
    let (wa, wb) = (x87_load32(wa), x87_load32(wb));
    if wa == 0.0 && wb == 0.0 {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, 0);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, 0);
        return;
    }
    let set = |e: &mut Engine, a: u32, b: u32| {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, a);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, b);
    };
    let priority_a = e.mem.u8(first + ITEM_PRIORITY) as i8;
    let priority_b = e.mem.u8(second + ITEM_PRIORITY) as i8;
    let (a, b) = if priority_a > priority_b {
        if operand(e, first + ITEM_EASE_SPINNER) == 1.0 {
            set(e, ONE_BITS, 0);
            return;
        }
        let ease = load(e, first + ITEM_EASE_SPINNER);
        let one_minus = x87_load32(x87_store32(x87_sub(1.0, ease)));
        let ease = load(e, first + ITEM_EASE_SPINNER);
        let weighted_a = x87_mul(wa, ease);
        let weighted_b = x87_mul(wb, one_minus);
        let inverse = x87_load32(x87_store32(x87_div(1.0, x87_add(weighted_b, weighted_a))));
        (
            x87_store32(x87_mul(weighted_a, inverse)),
            x87_store32(x87_mul(weighted_b, inverse)),
        )
    } else if priority_a < priority_b {
        if operand(e, second + ITEM_EASE_SPINNER) == 1.0 {
            set(e, 0, ONE_BITS);
            return;
        }
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let one_minus = x87_load32(x87_store32(x87_sub(1.0, ease)));
        let weighted_a = x87_mul(wa, one_minus);
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let sum = x87_add(x87_mul(ease, wb), weighted_a);
        let inverse = x87_load32(x87_store32(x87_div(1.0, sum)));
        let a = x87_store32(x87_mul(weighted_a, inverse));
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let weighted_b = x87_mul(wb, ease);
        (a, x87_store32(x87_mul(inverse, weighted_b)))
    } else {
        let inverse = x87_load32(x87_store32(x87_div(1.0, x87_add(wb, wa))));
        (
            x87_store32(x87_mul(wa, inverse)),
            x87_store32(x87_mul(wb, inverse)),
        )
    };
    e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, a);
    e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, b);
    if byte(e, this, FLAGS) & ONLY_USE_HIGHEST_WEIGHT != 0 {
        let a = load(e, first + ITEM_NORMALIZED_WEIGHT);
        let b = load(e, second + ITEM_NORMALIZED_WEIGHT);
        if b <= a {
            set(e, ONE_BITS, 0);
        } else {
            set(e, 0, ONE_BITS);
        }
        return;
    }
    let threshold = this.addr() + WEIGHT_THRESHOLD;
    if !(0.0 < operand(e, threshold)) {
        return;
    }
    let mut zeroed = false;
    if load(e, threshold) > load(e, first + ITEM_NORMALIZED_WEIGHT) {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, 0);
        zeroed = true;
    }
    if load(e, threshold) > load(e, second + ITEM_NORMALIZED_WEIGHT) {
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, 0);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
        return;
    }
    if zeroed {
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
    }
}

// Translated from 00a37260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ComputeNormalizedWeights` (Xbox PDB): only when
/// the weights are out of date (flag 4, then cleared). One interpolator:
/// its normalized weight 1. Two: `fn_00a36bd0`. Otherwise, when the high
/// sum is unset (`-FLT_MAX`), the sums of `weight * ease spinner` of the
/// highest- and next-highest-priority items and the largest ease spinner
/// among the highest are recomputed (from 0); each used item's normalized
/// weight is then `weight * spinner * ease * r` (highest priority),
/// `weight * (1 - spinner) * ease * r` (next highest) or 0, with `r` the
/// reciprocal of `high * spinner + next * (1 - spinner)` (0 when that is
/// not positive). With a positive threshold, weights below it are zeroed
/// and the rest scaled to sum to 1 (by 0 when the sum is not positive;
/// untouched when it is exactly 1). With "only the highest weight" (flag 2)
/// the first largest weight becomes 1 and every other 0 (the exe writes
/// item `0xFF` for an empty array). The float roundings follow the exe's
/// stores.
pub fn ni_blend_interpolator_compute_normalized_weights(e: &mut Engine, this: Ptr) {
    let flags = byte(e, this, FLAGS);
    if flags & COMPUTE_NORMALIZED_WEIGHTS == 0 {
        return;
    }
    set_byte(e, this, FLAGS, flags & !COMPUTE_NORMALIZED_WEIGHTS);
    match byte(e, this, INTERP_COUNT) {
        1 => {
            let single = item(e, this, u32::from(byte(e, this, SINGLE_INDEX)));
            e.mem.set_u32(single + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
            return;
        }
        2 => {
            fn_00a36bd0(e, this);
            return;
        }
        _ => {}
    }
    let at = |offset: u32| this.addr().wrapping_add(offset);
    let load = |e: &Engine, address: u32| x87_load32(e.mem.u32(address));
    let operand = |e: &Engine, address: u32| x87_m32(e.mem.u32(address));
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    if load(e, at(HIGH_SUM_OF_WEIGHTS)) == lowest {
        e.mem.set_u32(at(HIGH_SUM_OF_WEIGHTS), 0);
        e.mem.set_u32(at(NEXT_HIGH_SUM_OF_WEIGHTS), 0);
        e.mem.set_u32(at(HIGH_EASE_SPINNER), 0);
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let entry = item(e, this, u32::from(i));
            if e.mem.u32(entry) != 0 {
                let weighted = x87_store32(x87_mul(
                    load(e, entry + ITEM_WEIGHT),
                    operand(e, entry + ITEM_EASE_SPINNER),
                ));
                let priority = e.mem.u8(entry + ITEM_PRIORITY);
                if priority == byte(e, this, HIGH_PRIORITY) {
                    let sum = x87_add(load(e, at(HIGH_SUM_OF_WEIGHTS)), x87_m32(weighted));
                    e.mem.set_u32(at(HIGH_SUM_OF_WEIGHTS), x87_store32(sum));
                    if load(e, at(HIGH_EASE_SPINNER)) < load(e, entry + ITEM_EASE_SPINNER) {
                        copy_float(e, entry + ITEM_EASE_SPINNER, at(HIGH_EASE_SPINNER));
                    }
                } else if priority == byte(e, this, NEXT_HIGH_PRIORITY) {
                    let sum = x87_add(
                        x87_load32(weighted),
                        operand(e, at(NEXT_HIGH_SUM_OF_WEIGHTS)),
                    );
                    e.mem
                        .set_u32(at(NEXT_HIGH_SUM_OF_WEIGHTS), x87_store32(sum));
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let one_minus_spinner = x87_load32(x87_store32(x87_sub(1.0, load(e, at(HIGH_EASE_SPINNER)))));
    let next_part = x87_mul(load(e, at(NEXT_HIGH_SUM_OF_WEIGHTS)), one_minus_spinner);
    let high_part = x87_mul(
        load(e, at(HIGH_SUM_OF_WEIGHTS)),
        operand(e, at(HIGH_EASE_SPINNER)),
    );
    let total = x87_load32(x87_store32(x87_add(next_part, high_part)));
    let reciprocal = if total > 0.0 {
        x87_div(1.0, total)
    } else {
        0.0
    };
    let reciprocal = x87_load32(x87_store32(reciprocal));
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let entry = item(e, this, u32::from(i));
        if e.mem.u32(entry) != 0 {
            let priority = e.mem.u8(entry + ITEM_PRIORITY);
            if priority == byte(e, this, HIGH_PRIORITY) {
                let w = x87_mul(
                    load(e, entry + ITEM_WEIGHT),
                    operand(e, at(HIGH_EASE_SPINNER)),
                );
                let w = x87_mul(w, operand(e, entry + ITEM_EASE_SPINNER));
                let w = x87_mul(w, reciprocal);
                e.mem
                    .set_u32(entry + ITEM_NORMALIZED_WEIGHT, x87_store32(w));
            } else if priority == byte(e, this, NEXT_HIGH_PRIORITY) {
                let w = x87_mul(load(e, entry + ITEM_WEIGHT), one_minus_spinner);
                let w = x87_mul(w, operand(e, entry + ITEM_EASE_SPINNER));
                let w = x87_mul(w, reciprocal);
                e.mem
                    .set_u32(entry + ITEM_NORMALIZED_WEIGHT, x87_store32(w));
            } else {
                e.mem.set_u32(entry + ITEM_NORMALIZED_WEIGHT, 0);
            }
        }
        i = i.wrapping_add(1);
    }
    if 0.0 < operand(e, at(WEIGHT_THRESHOLD)) {
        let mut sum = 0u32;
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let entry = item(e, this, u32::from(i));
            if e.mem.u32(entry) != 0 && !(operand(e, entry + ITEM_NORMALIZED_WEIGHT) == 0.0) {
                if load(e, at(WEIGHT_THRESHOLD)) > load(e, entry + ITEM_NORMALIZED_WEIGHT) {
                    e.mem.set_u32(entry + ITEM_NORMALIZED_WEIGHT, 0);
                }
                let added = x87_add(load(e, entry + ITEM_NORMALIZED_WEIGHT), x87_m32(sum));
                sum = x87_store32(added);
            }
            i = i.wrapping_add(1);
        }
        let sum = x87_load32(sum);
        if sum != 1.0 {
            let factor = if sum > 0.0 {
                x87_store32(x87_div(1.0, sum))
            } else {
                0
            };
            let factor = x87_load32(factor);
            let mut i = 0u8;
            while i < byte(e, this, ARRAY_SIZE) {
                let entry = item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT;
                if !(0.0 == operand(e, entry)) {
                    let scaled = x87_mul(load(e, entry), factor);
                    e.mem.set_u32(entry, x87_store32(scaled));
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if byte(e, this, FLAGS) & ONLY_USE_HIGHEST_WEIGHT == 0 {
        return;
    }
    let mut highest = x87_load32(e.mem.u32(MINUS_ONE));
    let mut index = 0xffu8;
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let entry = item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT;
        let weight = load(e, entry);
        if highest < weight {
            highest = weight;
            index = i;
        }
        e.mem.set_u32(entry, 0);
        i = i.wrapping_add(1);
    }
    let chosen = item(e, this, u32::from(index));
    e.mem.set_u32(chosen + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
}

// Translated from 00a36e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`,
/// looks up this object's clone in the cloning process's clone map
/// (`00a53c30` on the map at `cloning + 0`), runs `ProcessClone` (vtable
/// +0x68) of every item's interpolator, then sets the clone's single
/// interpolator to the clone of this one's (when the map has it) or this
/// one's own.
pub fn ni_blend_interpolator_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, found]);
        let clone = e.mem.u32(found.addr());
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let interpolator = e.mem.u32(item(e, this, u32::from(i)));
            if interpolator != 0 {
                e.vcall(interpolator, 0x68, &args![cloning]);
            }
            i = i.wrapping_add(1);
        }
        let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
        if single == 0 {
            return;
        }
        let map = e.mem.u32(cloning.addr());
        let value = if e.call(CLONE_MAP_LOOKUP, &args![map, single, found]).bool() {
            e.mem.u32(found.addr())
        } else {
            e.mem.u32(this.addr() + SINGLE_INTERPOLATOR)
        };
        e.mem
            .set_u32(clone.wrapping_add(SINGLE_INTERPOLATOR), value);
    });
}

// Translated from 00a37510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiBlendInterpolator`'s `CopyMembers`, the
/// target of `fn_00a40aa0`'s jump): `NiKeyBasedInterpolator::CopyMembers`,
/// then the flags and array size, a new item array of that size and the
/// weight threshold; unless the interpolator is manager controlled (flag
/// 1), every item is copied (its interpolator replaced by a clone made by
/// `CreateClone`, vtable +0x48, when it has one) and the interpolator
/// count, single index, single time and both priorities.
pub fn fn_00a37510(e: &mut Engine, this: Ptr, destination: Ptr, cloning: Ptr) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let flags = byte(e, this, FLAGS);
    set_byte(e, destination, FLAGS, flags);
    let size = byte(e, this, ARRAY_SIZE);
    set_byte(e, destination, ARRAY_SIZE, size);
    let size = byte(e, this, ARRAY_SIZE);
    let array = new_item_array(e, u32::from(size));
    e.mem.set_u32(destination.addr() + INTERP_ARRAY, array);
    copy_float(
        e,
        this.addr() + WEIGHT_THRESHOLD,
        destination.addr() + WEIGHT_THRESHOLD,
    );
    if byte(e, this, FLAGS) & MANAGER_CONTROLLED != 0 {
        return;
    }
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let from = item(e, this, u32::from(i));
        let to = item(e, destination, u32::from(i));
        let interpolator = e.mem.u32(from);
        if interpolator != 0 {
            let clone = e.vcall(interpolator, 0x48, &args![cloning]).u32();
            assign_ref_counted(e, to, clone);
        }
        copy_float(e, from + ITEM_WEIGHT, to + ITEM_WEIGHT);
        copy_float(
            e,
            from + ITEM_NORMALIZED_WEIGHT,
            to + ITEM_NORMALIZED_WEIGHT,
        );
        let priority = e.mem.u8(from + ITEM_PRIORITY);
        e.mem.set_u8(to + ITEM_PRIORITY, priority);
        copy_float(e, from + ITEM_EASE_SPINNER, to + ITEM_EASE_SPINNER);
        copy_float(e, from + ITEM_UPDATE_TIME, to + ITEM_UPDATE_TIME);
        i = i.wrapping_add(1);
    }
    for offset in [INTERP_COUNT, SINGLE_INDEX] {
        let value = byte(e, this, offset);
        set_byte(e, destination, offset, value);
    }
    copy_float(
        e,
        this.addr() + SINGLE_TIME,
        destination.addr() + SINGLE_TIME,
    );
    for offset in [HIGH_PRIORITY, NEXT_HIGH_PRIORITY] {
        let value = byte(e, this, offset);
        set_byte(e, destination, offset, value);
    }
}

// Translated from 00a37690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ReallocateArrays` (Xbox PDB): `false` when the
/// grow step (`ms_ucArrayGrowBy`) is 0. Otherwise a new item array of
/// `size + step` items (a byte; at least 4), the old items assigned into it
/// (`fn_00a371f0`), the old array deleted, and the new array and size kept.
/// Answers `true`.
pub fn ni_blend_interpolator_reallocate_arrays(e: &mut Engine, this: Ptr) -> bool {
    let grow_by = e.mem.u8(ARRAY_GROW_BY);
    if grow_by == 0 {
        return false;
    }
    let mut size = byte(e, this, ARRAY_SIZE).wrapping_add(grow_by);
    if size < 4 {
        size = 4;
    }
    let array = new_item_array(e, u32::from(size));
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let offset = u32::from(i) * ITEM_SIZE;
        let from = item(e, this, u32::from(i));
        fn_00a371f0(e, Ptr::new(offset.wrapping_add(array)), Ptr::new(from));
        i = i.wrapping_add(1);
    }
    let old = e.mem.u32(this.addr() + INTERP_ARRAY);
    if old != 0 {
        delete_item_array(e, old);
    }
    e.mem.set_u32(this.addr() + INTERP_ARRAY, array);
    set_byte(e, this, ARRAY_SIZE, size);
    true
}

/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8 and `m_pkIstr` +0x24C.
const STREAM_FILE_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
/// The file versions `LoadBinary` tells apart.
const VERSION_SINGLE_LINK: u32 = 0x0a01_006c;
const VERSION_BYTE_COUNTS: u32 = 0x0a01_006e;
const VERSION_FLAGS_FIRST: u32 = 0x0a01_0070;

/// One read through the stream's input `m_pfnRead` (`cdecl (istr, buffer,
/// size, &component size, 1)`, component size `width` written first), the
/// position (+4) advanced by the bytes read.
fn read(e: &mut Engine, stream: Ptr, buffer: u32, size: u32, component_size: u32, width: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let function = e.mem.u32(input.wrapping_add(8));
    e.mem.set_u32(component_size, width);
    let got = e
        .call(function, &args![input, buffer, size, component_size, 1u32])
        .u32();
    let position = input.wrapping_add(4);
    let advanced = e.mem.u32(position).wrapping_add(got);
    e.mem.set_u32(position, advanced);
}

// Translated from 00a37980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::LoadBinary` (Xbox PDB): `NiInterpolator::LoadBinary`,
/// then by file version. From `0x0A010070`: the flags (1 byte, "weights out
/// of date" cleared), the array size (1 byte) and a new item array, the
/// threshold, and unless manager controlled the count, single index, both
/// priorities (1 byte each), single time, the three sums (4 bytes each),
/// every item (`InterpArrayItem::LoadBinary`) and the single interpolator's
/// link (`ResolveLinkID`). Before it: the array size (2 bytes, a second 2
/// discarded before `0x0A01006E`, else 1 byte) and the array, the items,
/// the manager-controlled and only-highest flags (1 byte each), the
/// threshold, the count and single index (2 bytes each before `0x0A01006E`,
/// else 1), from `0x0A01006C` the single link and time, and the priorities
/// (4-byte integers, `0x80000000` meaning `0x80`, before `0x0A01006E`; else 1
/// byte each). The buffers and component sizes are the stack words the exe
/// uses (the stream argument's word and three locals).
pub fn ni_blend_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    let version = |e: &Engine| e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION));
    e.with_stack(16, |e, frame| {
        // The stream argument's stack word and the three locals.
        let argument = frame.addr();
        let first_local = argument + 4;
        let second_local = argument + 8;
        let third_local = argument + 0xc;
        e.mem.set_u32(argument, stream.addr());
        if version(e) >= VERSION_FLAGS_FIRST {
            read(e, stream, at(FLAGS), 1, argument, 1);
            let flags = byte(e, this, FLAGS) & !COMPUTE_NORMALIZED_WEIGHTS;
            set_byte(e, this, FLAGS, flags);
        }
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, argument, 2, first_local, 2);
            let size = e.mem.u8(argument);
            set_byte(e, this, ARRAY_SIZE, size);
            read(e, stream, second_local, 2, first_local, 2);
        } else {
            read(e, stream, at(ARRAY_SIZE), 1, argument, 1);
        }
        let size = byte(e, this, ARRAY_SIZE);
        let array = new_item_array(e, u32::from(size));
        if array != 0 {
            e.mem.set_u32(argument, array);
        }
        e.mem.set_u32(at(INTERP_ARRAY), array);
        let load_items = |e: &mut Engine| {
            e.mem.set_u8(argument, 0);
            while e.mem.u8(argument) < byte(e, this, ARRAY_SIZE) {
                let index = u32::from(e.mem.u8(argument));
                let entry = item(e, this, index);
                e.call(ITEM_LOAD_BINARY, &args![entry, stream]);
                let next = e.mem.u8(argument).wrapping_add(1);
                e.mem.set_u8(argument, next);
            }
        };
        if version(e) >= VERSION_FLAGS_FIRST {
            read(e, stream, at(WEIGHT_THRESHOLD), 4, argument, 4);
            if byte(e, this, FLAGS) & MANAGER_CONTROLLED != 0 {
                return;
            }
            for offset in [
                INTERP_COUNT,
                SINGLE_INDEX,
                HIGH_PRIORITY,
                NEXT_HIGH_PRIORITY,
            ] {
                read(e, stream, at(offset), 1, argument, 1);
            }
            for offset in [
                SINGLE_TIME,
                HIGH_SUM_OF_WEIGHTS,
                NEXT_HIGH_SUM_OF_WEIGHTS,
                HIGH_EASE_SPINNER,
            ] {
                read(e, stream, at(offset), 4, argument, 4);
            }
            load_items(e);
            let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(at(SINGLE_INTERPOLATOR), link);
            return;
        }
        load_items(e);
        read(e, stream, third_local + 2, 1, argument, 1);
        let flags = if e.mem.u8(third_local + 2) != 0 {
            byte(e, this, FLAGS) | MANAGER_CONTROLLED
        } else {
            byte(e, this, FLAGS) & !MANAGER_CONTROLLED
        };
        set_byte(e, this, FLAGS, flags);
        read(e, stream, at(WEIGHT_THRESHOLD), 4, argument, 4);
        read(e, stream, third_local + 3, 1, argument, 1);
        let flags = if e.mem.u8(third_local + 3) != 0 {
            byte(e, this, FLAGS) | ONLY_USE_HIGHEST_WEIGHT
        } else {
            byte(e, this, FLAGS) & !ONLY_USE_HIGHEST_WEIGHT
        };
        set_byte(e, this, FLAGS, flags);
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, argument, 2, first_local, 2);
            let count = e.mem.u8(argument);
            set_byte(e, this, INTERP_COUNT, count);
            read(e, stream, second_local, 2, first_local, 2);
            let index = e.mem.u8(second_local);
            set_byte(e, this, SINGLE_INDEX, index);
        } else {
            read(e, stream, at(INTERP_COUNT), 1, argument, 1);
            read(e, stream, at(SINGLE_INDEX), 1, argument, 1);
        }
        if version(e) >= VERSION_SINGLE_LINK {
            let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(at(SINGLE_INTERPOLATOR), link);
            read(e, stream, at(SINGLE_TIME), 4, argument, 4);
        }
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, first_local, 4, argument, 4);
            let high = e.mem.u32(first_local);
            let high = if high == 0x8000_0000 {
                0x80
            } else {
                high as u8
            };
            set_byte(e, this, HIGH_PRIORITY, high);
            read(e, stream, second_local, 4, argument, 4);
            let next = e.mem.u32(second_local);
            let next = if next == 0x8000_0000 {
                0x80
            } else {
                next as u8
            };
            set_byte(e, this, NEXT_HIGH_PRIORITY, next);
        } else {
            read(e, stream, at(HIGH_PRIORITY), 1, argument, 1);
            read(e, stream, at(NEXT_HIGH_PRIORITY), 1, argument, 1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a36960, ni_blend_interpolator_get_rtti() -> Ptr),
        entry!(0x00a37670, ni_blend_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a370a0, fn_00a370a0(Ptr) -> Ptr),
        entry!(0x00a371f0, fn_00a371f0(Ptr, Ptr) -> Ptr),
        entry!(0x00a370b0, ni_blend_interpolator_ni_blend_interpolator(Ptr, u8, f32, u8) -> Ptr),
        entry!(0x00a37170, ni_blend_interpolator_destructor(Ptr)),
        entry!(0x00a368f0, ni_blend_interpolator_get_highest_weighted_index(Ptr) -> u8),
        entry!(
            0x00a36970,
            ni_blend_interpolator_add_interp_info(Ptr, Ptr, f32, i8, f32) -> u8
        ),
        entry!(
            0x00a36a90,
            ni_blend_interpolator_remove_interp_info(Ptr, u8)
        ),
        entry!(0x00a36bd0, fn_00a36bd0(Ptr)),
        entry!(
            0x00a37260,
            ni_blend_interpolator_compute_normalized_weights(Ptr)
        ),
        entry!(0x00a36e80, ni_blend_interpolator_process_clone(Ptr, Ptr)),
        entry!(0x00a37510, fn_00a37510(Ptr, Ptr, Ptr)),
        entry!(0x00a37690, ni_blend_interpolator_reallocate_arrays(Ptr) -> bool),
        entry!(0x00a37980, ni_blend_interpolator_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a36960, &args![]).u32(),
            NI_BLEND_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_blend_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_7170, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a37670, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a37670, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00a3_7170), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A vtable for counted test objects: slot +4 (delete) logs `this`,
    /// +0x48 (`CreateClone`) answers `this + 0x100`, +0x68 (`ProcessClone`)
    /// and +0xE8 (`ReallocateArrays`, answers false) are doubles too.
    const VTABLE: u32 = 0x7000_0000;
    const DELETE: u32 = 0x7200_0004;
    const CREATE_CLONE: u32 = 0x7200_0048;
    const CHILD_PROCESS_CLONE: u32 = 0x7200_0068;
    const REALLOCATE: u32 = 0x7200_00e8;

    type Log = Rc<RefCell<Vec<u32>>>;

    /// The engine every test uses: counting interlocked imports, the
    /// memory manager's allocate/free and the vector constructor iterator
    /// as doubles (the iterator runs `00a370a0` on every element), the
    /// constants mapped, and the test vtable.
    fn engine() -> (Engine, Log) {
        let mut e = engine_with_counting();
        e.map(0x0109_6000, 0x1000);
        e.mem.set_f32(INVALID_FLOAT, f32::MIN);
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST, -3.4028234663852886e38f64);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.map(0x011a_8000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(0x00a3_70a0, &args![a[0] + i * a[1]]);
            }
            Ret::default()
        });
        let mut slots = vec![0u32; 0xec / 4];
        slots[1] = DELETE;
        slots[0x48 / 4] = CREATE_CLONE;
        slots[0x68 / 4] = CHILD_PROCESS_CLONE;
        slots[0xe8 / 4] = REALLOCATE;
        e.put_vtable(VTABLE, &slots);
        let deleted: Log = Rc::default();
        let log = deleted.clone();
        e.register_double(DELETE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register(CREATE_CLONE, |_, a| (a[0] + 0x100).into_ret());
        e.register(CHILD_PROCESS_CLONE, |_, _| Ret::default());
        e.register(REALLOCATE, |_, _| false.into_ret());
        (e, deleted)
    }

    /// A counted object (count `count`) with the test vtable.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A blend interpolator with an item array of `size` items (count word
    /// in front), the test vtable.
    fn blend(e: &mut Engine, size: u8) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x30));
        e.mem.set_u32(this.addr(), VTABLE);
        let block = e.mem.alloc(4 + ITEM_SIZE * u32::from(size));
        e.mem.set_u32(block, u32::from(size));
        e.mem.set_u32(this.addr() + INTERP_ARRAY, block + 4);
        set_byte(e, this, ARRAY_SIZE, size);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, SINGLE_INDEX, 0xff);
        this
    }

    /// Fills item `index`.
    fn put_item(
        e: &mut Engine,
        this: Ptr,
        index: u32,
        interp: u32,
        weight: f32,
        priority: i8,
        ease: f32,
    ) {
        let at = item(e, this, index);
        e.mem.set_u32(at, interp);
        e.mem.set_f32(at + ITEM_WEIGHT, weight);
        e.mem.set_u8(at + ITEM_PRIORITY, priority as u8);
        e.mem.set_f32(at + ITEM_EASE_SPINNER, ease);
    }

    fn normalized(e: &Engine, this: Ptr, index: u32) -> f32 {
        e.mem.f32(item(e, this, index) + ITEM_NORMALIZED_WEIGHT)
    }

    #[test]
    fn item_constructor_clears_the_interpolator() {
        let (mut e, _) = engine();
        let at = e.mem.alloc(ITEM_SIZE);
        e.mem.set_u32(at, 0x1234);
        assert_eq!(e.call(0x00a370a0, &args![at]).u32(), at);
        assert_eq!(e.mem.u32(at), 0);
    }

    #[test]
    fn item_assignment_counts_the_interpolator_and_copies_the_rest() {
        let (mut e, deleted) = engine();
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 1);
        let to = e.mem.alloc(ITEM_SIZE);
        let from = e.mem.alloc(ITEM_SIZE);
        e.mem.set_u32(to, old);
        e.mem.set_u32(from, new);
        for (offset, value) in [(4u32, 1.5f32), (8, 2.5), (0x10, 3.5), (0x14, 4.5)] {
            e.mem.set_f32(from + offset, value);
        }
        e.mem.set_u8(from + 0xc, 7);
        assert_eq!(e.call(0x00a371f0, &args![to, from]).u32(), to);
        assert_eq!(e.mem.u32(to), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(*deleted.borrow(), [old]);
        assert_eq!(e.mem.f32(to + 0x14), 4.5);
        assert_eq!(e.mem.u8(to + 0xc), 7);
    }

    #[test]
    fn constructor_sets_the_defaults_and_allocates_the_array() {
        for size in [0u8, 3] {
            let (mut e, _) = engine();
            e.register(NI_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
            let this = Ptr::new(e.mem.alloc(0x30));
            e.call_log = Some(vec![]);
            let r = e
                .call(0x00a370b0, &args![this, 5u32, 0.25f32, u32::from(size)])
                .u32();
            assert_eq!(r, this.addr());
            assert_eq!(e.mem.u32(this.addr()), NI_BLEND_INTERPOLATOR_VTABLE);
            assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.25);
            for offset in [
                SINGLE_TIME,
                HIGH_SUM_OF_WEIGHTS,
                NEXT_HIGH_SUM_OF_WEIGHTS,
                HIGH_EASE_SPINNER,
            ] {
                assert_eq!(e.mem.f32(this.addr() + offset), f32::MIN);
            }
            assert_eq!(byte(&e, this, FLAGS), 1);
            assert_eq!(byte(&e, this, ARRAY_SIZE), size);
            assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
            assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
            let array = e.mem.u32(this.addr() + INTERP_ARRAY);
            if size == 0 {
                assert_eq!(array, 0);
                assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
            } else {
                assert_eq!(e.mem.u32(array - 4), 3);
                assert_eq!(
                    calls_to(&e, MEMORY_MANAGER_ALLOCATE),
                    vec![vec![MEMORY_MANAGER, 3 * 0x18 + 4]]
                );
                assert_eq!(
                    calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
                    vec![vec![array, 0x18, 3, 0x00a3_70a0]]
                );
            }
        }
    }

    #[test]
    fn destructor_releases_every_item_and_frees_the_array() {
        let (mut e, deleted) = engine();
        e.register(NI_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        let this = blend(&mut e, 3);
        let kept = counted(&mut e, 2);
        let dropped = counted(&mut e, 1);
        put_item(&mut e, this, 0, kept, 1.0, 0, 1.0);
        put_item(&mut e, this, 2, dropped, 1.0, 0, 1.0);
        let array = e.mem.u32(this.addr() + INTERP_ARRAY);
        e.call_log = Some(vec![]);
        e.call(0x00a37170, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_BLEND_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(*deleted.borrow(), [dropped]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, array - 4]]
        );
        assert_eq!(
            calls_to(&e, NI_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn highest_weighted_index() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 3);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        set_byte(&mut e, this, SINGLE_INDEX, 2);
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 2);
        set_byte(&mut e, this, INTERP_COUNT, 3);
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 0xff);
        for (i, w) in [0.25f32, 0.5, 0.5].iter().enumerate() {
            e.mem
                .set_f32(item(&e, this, i as u32) + ITEM_NORMALIZED_WEIGHT, *w);
        }
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 1);
    }

    #[test]
    fn add_interp_info_fills_the_first_free_item() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        let r = e
            .call(0x00a36970, &args![this, first, 0.5f32, 3u32, 0.75f32])
            .u8();
        assert_eq!(r, 0);
        assert_eq!(e.mem.u32(first + 4), 1);
        let at = item(&e, this, 0);
        assert_eq!(e.mem.f32(at + ITEM_WEIGHT), 0.5);
        assert_eq!(e.mem.f32(at + ITEM_EASE_SPINNER), 0.75);
        assert_eq!(e.mem.f32(at + ITEM_UPDATE_TIME), f32::MIN);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 3);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), first);
        assert_eq!(byte(&e, this, FLAGS) & 4, 4);

        let r = e
            .call(0x00a36970, &args![this, second, 1.0f32, 5u32, 1.0f32])
            .u8();
        assert_eq!(r, 1);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 5);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 3);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), f32::MIN);

        // Full and the reallocation refuses: 0xFF, nothing changes.
        let r = e
            .call(0x00a36970, &args![this, second, 1.0f32, 5u32, 1.0f32])
            .u8();
        assert_eq!(r, 0xff);
        assert_eq!(byte(&e, this, INTERP_COUNT), 2);
    }

    #[test]
    fn remove_interp_info_recomputes_priorities_and_the_single_item() {
        let (mut e, deleted) = engine();
        let this = blend(&mut e, 3);
        let a = counted(&mut e, 1);
        let b = counted(&mut e, 2);
        put_item(&mut e, this, 0, a, 1.0, 5, 1.0);
        put_item(&mut e, this, 2, b, 1.0, 2, 1.0);
        e.mem.set_f32(item(&e, this, 2) + ITEM_UPDATE_TIME, 9.0);
        set_byte(&mut e, this, HIGH_PRIORITY, 5);
        set_byte(&mut e, this, NEXT_HIGH_PRIORITY, 2);
        set_byte(&mut e, this, INTERP_COUNT, 2);
        e.call(0x00a36a90, &args![this, 0u32]);
        assert_eq!(*deleted.borrow(), [a]);
        assert_eq!(e.mem.u32(item(&e, this, 0)), 0);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 2);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 2);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), b);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), 9.0);
        // The last one: nothing left.
        e.call(0x00a36a90, &args![this, 2u32]);
        assert_eq!(byte(&e, this, INTERP_COUNT), 0);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), f32::MIN);
        assert_eq!(e.mem.f32(this.addr() + HIGH_SUM_OF_WEIGHTS), f32::MIN);
    }

    #[test]
    fn remove_from_two_items_takes_the_other_index() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        let a = counted(&mut e, 2);
        let b = counted(&mut e, 2);
        put_item(&mut e, this, 0, a, 1.0, 0, 1.0);
        put_item(&mut e, this, 1, b, 1.0, 0, 1.0);
        set_byte(&mut e, this, INTERP_COUNT, 2);
        e.call(0x00a36a90, &args![this, 1u32]);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), a);
    }

    fn two(e: &mut Engine, (wa, pa, ea): (f32, i8, f32), (wb, pb, eb): (f32, i8, f32)) -> Ptr {
        let this = blend(e, 3);
        put_item(e, this, 0, 1, wa, pa, ea);
        put_item(e, this, 2, 1, wb, pb, eb);
        this
    }

    #[test]
    fn two_items_by_priority() {
        let (mut e, _) = engine();
        // Equal priorities: proportional.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
        // A higher with spinner 1: (1, 0); B higher with spinner 1: (0, 1).
        let this = two(&mut e, (1.0, 2, 1.0), (3.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (1.0, 0.0)
        );
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 2, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // A higher with spinner 0.5: 0.25 against 0.5.
        let this = two(&mut e, (1.0, 2, 0.5), (1.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        let (a, b) = (normalized(&e, this, 0), normalized(&e, this, 2));
        assert!(
            (a - 1.0 / 3.0).abs() < 1e-6 && (b - 2.0 / 3.0).abs() < 1e-6,
            "{a} {b}"
        );
        // B higher with spinner 0.5: A 1 * 0.5 against B (1 * 0.5) * 0.5.
        let this = two(&mut e, (1.0, 1, 1.0), (1.0, 2, 0.5));
        e.call(0x00a36bd0, &args![this]);
        let (a, b) = (normalized(&e, this, 0), normalized(&e, this, 2));
        assert!(
            (a - 2.0 / 3.0).abs() < 1e-6 && (b - 1.0 / 3.0).abs() < 1e-6,
            "{a} {b}"
        );
        // Both zero.
        let this = two(&mut e, (0.0, 1, 1.0), (0.0, 1, 1.0));
        e.mem
            .set_f32(item(&e, this, 0) + ITEM_NORMALIZED_WEIGHT, 7.0);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 0.0)
        );
    }

    #[test]
    fn two_items_highest_only_and_threshold() {
        let (mut e, _) = engine();
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        let this = two(&mut e, (1.0, 1, 1.0), (1.0, 1, 1.0));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (1.0, 0.0)
        );
        // Threshold 0.3: A's 0.25 is dropped and B becomes 1.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.3);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // Threshold 0.8: B's 0.75 also: B is zeroed, then set to 1.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.8);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // Threshold 0.1: nothing changes.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.1);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
    }

    fn three(e: &mut Engine) -> Ptr {
        let this = blend(e, 4);
        put_item(e, this, 0, 1, 1.0, 4, 1.0);
        put_item(e, this, 1, 1, 1.0, 4, 1.0);
        put_item(e, this, 3, 1, 2.0, 4, 1.0);
        set_byte(e, this, HIGH_PRIORITY, 4);
        set_byte(e, this, INTERP_COUNT, 3);
        set_byte(e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        for offset in [
            HIGH_SUM_OF_WEIGHTS,
            NEXT_HIGH_SUM_OF_WEIGHTS,
            HIGH_EASE_SPINNER,
        ] {
            e.mem.set_f32(this.addr() + offset, f32::MIN);
        }
        this
    }

    #[test]
    fn compute_normalized_weights_for_many() {
        let (mut e, _) = engine();
        let this = three(&mut e);
        e.call(0x00a37260, &args![this]);
        assert_eq!(byte(&e, this, FLAGS) & 4, 0);
        assert_eq!(e.mem.f32(this.addr() + HIGH_SUM_OF_WEIGHTS), 4.0);
        assert_eq!(e.mem.f32(this.addr() + HIGH_EASE_SPINNER), 1.0);
        let weights: Vec<f32> = [0, 1, 3].iter().map(|i| normalized(&e, this, *i)).collect();
        assert_eq!(weights, [0.25, 0.25, 0.5]);
        // Not out of date: nothing happens.
        e.mem
            .set_f32(item(&e, this, 0) + ITEM_NORMALIZED_WEIGHT, 9.0);
        e.call(0x00a37260, &args![this]);
        assert_eq!(normalized(&e, this, 0), 9.0);

        // A threshold of 0.3 drops the quarters; the half is scaled to 1.
        let this = three(&mut e);
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.3);
        e.call(0x00a37260, &args![this]);
        let weights: Vec<f32> = [0, 1, 3].iter().map(|i| normalized(&e, this, *i)).collect();
        assert_eq!(weights, [0.0, 0.0, 1.0]);

        // Only the highest weight.
        let this = three(&mut e);
        set_byte(
            &mut e,
            this,
            FLAGS,
            COMPUTE_NORMALIZED_WEIGHTS | ONLY_USE_HIGHEST_WEIGHT,
        );
        e.call(0x00a37260, &args![this]);
        let weights: Vec<f32> = [0, 1, 2, 3]
            .iter()
            .map(|i| normalized(&e, this, *i))
            .collect();
        assert_eq!(weights, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn compute_normalized_weights_for_one_or_two() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        set_byte(&mut e, this, SINGLE_INDEX, 1);
        set_byte(&mut e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        e.call(0x00a37260, &args![this]);
        assert_eq!(normalized(&e, this, 1), 1.0);
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        set_byte(&mut e, this, INTERP_COUNT, 2);
        set_byte(&mut e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        e.call(0x00a37260, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
    }

    #[test]
    fn process_clone_maps_the_single_interpolator() {
        const MAP: u32 = 0x7300_0000;
        for found in [true, false] {
            let (mut e, _) = engine();
            e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
            let this = blend(&mut e, 2);
            let child = counted(&mut e, 1);
            put_item(&mut e, this, 1, child, 1.0, 0, 1.0);
            e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, child);
            let clone = e.mem.alloc(0x30);
            e.map(MAP, 0x10);
            e.mem.set_u32(MAP + 4, clone);
            e.mem.set_u32(MAP + 8, u32::from(found));
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                // The object itself maps to the clone; its child to child + 1.
                if a[1] == e.mem.u32(MAP + 0xc) {
                    e.mem.set_u32(a[2], e.mem.u32(MAP + 4));
                    true.into_ret()
                } else {
                    e.mem.set_u32(a[2], a[1] + 1);
                    (e.mem.u32(MAP + 8) != 0).into_ret()
                }
            });
            e.mem.set_u32(MAP + 0xc, this.addr());
            let cloning = e.mem.alloc(4);
            e.mem.set_u32(cloning, MAP);
            e.call_log = Some(vec![]);
            e.call(0x00a36e80, &args![this, cloning]);
            assert_eq!(
                calls_to(&e, CHILD_PROCESS_CLONE),
                vec![vec![child, cloning]]
            );
            let expected = if found { child + 1 } else { child };
            assert_eq!(e.mem.u32(clone + SINGLE_INTERPOLATOR), expected);
        }
    }

    #[test]
    fn copy_members_clones_the_items_unless_manager_controlled() {
        for managed in [false, true] {
            let (mut e, _) = engine();
            e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
            let this = blend(&mut e, 2);
            let child = counted(&mut e, 1);
            put_item(&mut e, this, 0, child, 0.5, 3, 0.75);
            set_byte(&mut e, this, FLAGS, u8::from(managed));
            set_byte(&mut e, this, INTERP_COUNT, 1);
            set_byte(&mut e, this, SINGLE_INDEX, 0);
            set_byte(&mut e, this, HIGH_PRIORITY, 3);
            e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.125);
            e.mem.set_f32(this.addr() + SINGLE_TIME, 6.0);
            let destination = Ptr::new(e.mem.alloc(0x30));
            e.call(0x00a37510, &args![this, destination, 0x5000u32]);
            assert_eq!(byte(&e, destination, ARRAY_SIZE), 2);
            assert_eq!(byte(&e, destination, FLAGS), u8::from(managed));
            assert_eq!(e.mem.f32(destination.addr() + WEIGHT_THRESHOLD), 0.125);
            let copied = item(&e, destination, 0);
            if managed {
                assert_eq!(e.mem.u32(copied), 0);
                assert_eq!(byte(&e, destination, INTERP_COUNT), 0);
            } else {
                assert_eq!(e.mem.u32(copied), child + 0x100);
                assert_eq!(e.mem.f32(copied + ITEM_EASE_SPINNER), 0.75);
                assert_eq!(e.mem.u8(copied + ITEM_PRIORITY), 3);
                assert_eq!(byte(&e, destination, INTERP_COUNT), 1);
                assert_eq!(byte(&e, destination, HIGH_PRIORITY), 3);
                assert_eq!(e.mem.f32(destination.addr() + SINGLE_TIME), 6.0);
            }
        }
    }

    #[test]
    fn reallocate_arrays_grows_by_the_step_to_at_least_4() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 1);
        let child = counted(&mut e, 1);
        put_item(&mut e, this, 0, child, 0.5, 3, 0.75);
        let old = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert!(!e.call(0x00a37690, &args![this]).bool());
        e.mem.set_u8(ARRAY_GROW_BY, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a37690, &args![this]).bool());
        assert_eq!(byte(&e, this, ARRAY_SIZE), 4);
        let new = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert_eq!(e.mem.u32(new - 4), 4);
        assert_eq!(e.mem.u32(new), child);
        assert_eq!(e.mem.f32(new + ITEM_EASE_SPINNER), 0.75);
        assert_eq!(e.mem.u32(child + 4), 1);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, old - 4]]
        );
        // A bigger array grows by the step.
        set_byte(&mut e, this, ARRAY_SIZE, 4);
        e.mem.set_u8(ARRAY_GROW_BY, 3);
        e.call(0x00a37690, &args![this]);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 7);
    }

    /// A stream whose reads answer successive byte strings, logging
    /// (size, component size) of each.
    fn stream(
        e: &mut Engine,
        version: u32,
        chunks: Vec<Vec<u8>>,
    ) -> (u32, Rc<RefCell<Vec<(u32, u32)>>>) {
        const FAKE_READ: u32 = 0x7100_0000;
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, FAKE_READ);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, input);
        let log: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let seen = log.clone();
        let mut next = chunks.into_iter();
        e.register_double(FAKE_READ, move |e, a| {
            seen.borrow_mut().push((a[2], e.mem.u32(a[3])));
            let chunk = next.next().unwrap();
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            (chunk.len() as u32).into_ret()
        });
        (stream, log)
    }

    #[test]
    fn load_binary_current_format() {
        let (mut e, _) = engine();
        e.register(NI_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(ITEM_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0x4444u32.into_ret());
        let f = |v: f32| v.to_le_bytes().to_vec();
        let (stream, log) = stream(
            &mut e,
            0x0a01_0070,
            vec![
                vec![0x06],
                vec![2],
                f(0.5),
                vec![1],
                vec![0],
                vec![7],
                vec![6],
                f(1.5),
                f(2.5),
                f(3.5),
                f(4.5),
            ],
        );
        let this = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        e.call(0x00a37980, &args![this, stream]);
        assert_eq!(byte(&e, this, FLAGS), 2);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 2);
        assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.5);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 7);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 6);
        assert_eq!(e.mem.f32(this.addr() + HIGH_EASE_SPINNER), 4.5);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0x4444);
        let array = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert_eq!(
            calls_to(&e, ITEM_LOAD_BINARY),
            vec![vec![array, stream], vec![array + 0x18, stream]]
        );
        let sizes: Vec<u32> = log.borrow().iter().map(|r| r.0).collect();
        assert_eq!(sizes, [1, 1, 4, 1, 1, 1, 1, 4, 4, 4, 4]);
        assert!(log.borrow().iter().all(|r| r.0 == r.1));
    }

    #[test]
    fn load_binary_old_format() {
        let (mut e, _) = engine();
        e.register(NI_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(ITEM_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0x4444u32.into_ret());
        let f = |v: f32| v.to_le_bytes().to_vec();
        let (stream, log) = stream(
            &mut e,
            0x0a01_006c,
            vec![
                vec![1, 0],
                vec![9, 9],
                vec![1],
                f(0.25),
                vec![0],
                vec![1, 0],
                vec![0, 0],
                f(8.0),
                vec![0, 0, 0, 0x80],
                vec![5, 0, 0, 0],
            ],
        );
        let this = Ptr::new(e.mem.alloc(0x30));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a37980, &args![this, stream]);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 1);
        assert_eq!(byte(&e, this, FLAGS), MANAGER_CONTROLLED);
        assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.25);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0x4444);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), 8.0);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 5);
        let sizes: Vec<(u32, u32)> = log.borrow().clone();
        assert_eq!(
            sizes,
            [
                (2, 2),
                (2, 2),
                (1, 1),
                (4, 4),
                (1, 1),
                (2, 2),
                (2, 2),
                (4, 4),
                (4, 4),
                (4, 4)
            ]
        );
    }
}
