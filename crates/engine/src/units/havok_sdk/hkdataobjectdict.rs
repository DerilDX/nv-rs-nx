//! `Havok SDK/hkdataobjectdict.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dc9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_96a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dc9a20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_96a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_a0f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dca190(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_a0f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_a6b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dca720(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_a6b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcb960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_b890`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcb960(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_b890, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcbf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_8080`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcbf90(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_8080, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcc100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_bfc0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcc100(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_bfc0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// ---- Callees outside this file, called by address (first word is `this` for the methods). ----

/// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (hkstringmap.cpp, Xbox PDB names):
/// the constructor takes one stack word (an `hkBool`) it never reads.
pub const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
pub const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
pub const STRING_MAP_GET_ITERATOR: u32 = 0x00d8_7410;
pub const STRING_MAP_GET_VALUE: u32 = 0x00d8_7440;
pub const STRING_MAP_GET_NEXT: u32 = 0x00d8_7480;
pub const STRING_MAP_IS_VALID: u32 = 0x00d8_74b0;
pub const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
pub const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
pub const STRING_MAP_REMOVE: u32 = 0x00d8_7820;
/// `hkString::strDup` / `hkString::strFree` (cdecl).
pub const STRING_DUP: u32 = 0x00cb_1410;
pub const STRING_FREE: u32 = 0x00cb_13c0;
/// `hkMapBase<unsigned long,unsigned long>`: `getWithDefault(key, default)`, `findKey(key)`,
/// `remove(index)`, `clearAndDeallocate(allocator)`, and the unnamed map reset at `0x00cb_1a60`.
pub const POINTER_MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
pub const POINTER_MAP_FIND_KEY: u32 = 0x00cb_18f0;
pub const POINTER_MAP_REMOVE: u32 = 0x00cb_19b0;
pub const POINTER_MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
pub const POINTER_MAP_RESET: u32 = 0x00cb_1a60;
/// The one-byte function (`RET`) called on a map after `clearAndDeallocate` (unnamed in the map).
pub const POINTER_MAP_AFTER_CLEAR: u32 = 0x00a2_9680;
/// The multi-map constructor-like routine of this unit (`0x00dc_9e70`, unnamed in the map).
pub const MULTI_MAP_CONSTRUCT: u32 = 0x00dc_9e70;
/// `hkSerializeMultiMap::insert` (`0x00dc_9ec0`; the map names it after another instance).
pub const MULTI_MAP_INSERT: u32 = 0x00dc_9ec0;
/// `hkReferencedObject` reference counting in `hkreferencedobject.cpp` (unnamed in the map):
/// the object is in ECX, i.e. the first word.
pub const ADD_REFERENCE: u32 = 0x00c9_0510;
pub const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArrayUtil` (hkarrayutil.cpp): the growth step `(allocator, array, element size)` and
/// `_reserve(allocator, array, count, element size)`, both cdecl.
pub const ARRAY_GROW: u32 = 0x00c9_0940;
pub const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// C runtime `memcpy(dst, src, n)` and `memset(dst, value, n)` (cdecl).
pub const MEM_COPY: u32 = 0x00cb_1260;
pub const MEM_SET: u32 = 0x00ec_61c0;
/// `hkDataClass` (hkdataobject.cpp, Xbox PDB names); the first word is the `hkDataClass` handle
/// (a block whose first word is the class implementation).
pub const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
pub const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
pub const DATA_CLASS_IS_NULL: u32 = 0x00db_a670;
pub const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
pub const DATA_CLASS_GET_VERSION: u32 = 0x00db_a690;
pub const DATA_CLASS_GET_PARENT: u32 = 0x00db_a6a0;
pub const DATA_CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00db_a6f0;
pub const DATA_CLASS_GET_ALL_DECLARED_MEMBER_INFO: u32 = 0x00db_af10;
/// `hkDataObject::hkDataObject(impl)`.
pub const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
/// `hkDataWorldDict::ObjectTracker` methods outside this part of the unit (called with the tracker first).
pub const TRACKER_RETRACT_CASTED_OBJECT: u32 = 0x00dc_ab50;
pub const TRACKER_TRACK_CLASS: u32 = 0x00dc_a820;
pub const TRACKER_FN_00DCAAF0: u32 = 0x00dc_aaf0;
/// `hkDataClassDict` constructor (`this`, world, name, version) and `hkDataClassDict::addMember`.
pub const DATA_CLASS_DICT_CONSTRUCT: u32 = 0x00dc_abb0;

/// Offsets inside the object tracker (`hkDataWorldDict + 8`, 0x88 bytes). The roles are what
/// the code in this unit does with them; the PDB has no layout for it.
pub mod tracker {
    /// `hkCachedHashMap` class name -> class implementation.
    pub const CLASS_MAP: u32 = 0x00;
    /// Multi-map, class -> its derived classes.
    pub const DERIVED_MAP: u32 = 0x10;
    /// Multi-map, class -> the objects of exactly that class.
    pub const OBJECT_MAP: u32 = 0x30;
    /// `hkCachedHashMap` of interned strings (`hkString::strDup` copies, key and value alike).
    pub const INTERNED_NAMES: u32 = 0x4c;
    /// `hkCachedHashMap` name -> small reference-counted record (`internClassNameHandle`).
    pub const CLASS_HANDLES: u32 = 0x5c;
    /// Multi-map, class -> the struct and object arrays of that class.
    pub const ARRAY_MAP: u32 = 0x6c;
}

/// Field offsets of a multi-map (`hkSerializeMultiMap`): an `hkArray` of 8-byte `{value, next}`
/// entries, an `hkMapBase` from key to the first entry, and the head of the free list.
pub mod multi_map {
    pub const ENTRIES: u32 = 0x00;
    pub const MAP: u32 = 0x0c;
    pub const FREE_HEAD: u32 = 0x18;
}

layout! {
    /// `ArrayOfTuplesImplementation::View` (Xbox PDB name; fields are roles read from the code):
    /// a window of one tuple of an array of tuples. It sits embedded at `+0xc` of its
    /// [`ArrayOfTuplesImplementation`], so its `+0xc` points back at that object.
    pub struct ArrayOfTuplesView: 0x14 {
        /// The owning array of tuples.
        0x0c tuples: Ptr<ArrayOfTuplesImplementation>,
        /// Index of the tuple's first element in the element array.
        0x10 first_index: i32,
    }
    /// `ArrayOfTuplesImplementation` (Xbox PDB name; fields are roles read from the code).
    pub struct ArrayOfTuplesImplementation: 0x28 {
        /// The embedded view.
        0x0c view: Inline<ArrayOfTuplesView>,
        /// The array implementation holding every element of every tuple.
        0x20 elements: Ptr,
        /// Elements per tuple.
        0x24 tuple_size: i32,
    }
    /// `hkDataWorldDict` (Xbox PDB name). Only the fields the translated code touches.
    pub struct HkDataWorldDict: 0x0c {
        /// Stored as 1 by the constructor (role unconfirmed).
        0x06 kind: u16,
        /// The object tracker.
        0x08 tracker: Ptr<ObjectTracker>,
    }
    /// The object tracker (`hkDataWorldDict::ObjectTracker`); see [`tracker`].
    pub struct ObjectTracker: 0x88 {
        /// The first object made by `newObject` (what `getContents` hands out).
        0x2c contents: Ptr,
    }
    /// `hkDataClassDict` (Xbox PDB name), 0x40 bytes; fields are roles read from the code.
    pub struct HkDataClassDict: 0x40 {
        0x08 reference_count: i32,
        /// The world the class belongs to (what `ObjectTracker::intern` reads the tracker from).
        0x0c world: Ptr<HkDataWorldDict>,
        /// Written by `setClassVersion`.
        0x14 version: i32,
        /// The parent class (reference counted).
        0x18 parent: Ptr<HkDataClassDict>,
        /// Array of 0x18-byte member records: `{name, class handle, _, _, type, extra}`.
        0x28 members: Ptr,
        0x2c member_count: i32,
    }
    /// `hkDataObjectDict` (Xbox PDB name), 0x1c bytes; fields are roles read from the code.
    pub struct HkDataObjectDict: 0x1c {
        /// 16-bit size of the block, kept for the memory router.
        0x04 memory_size: u16,
        /// Count of references from outside the world.
        0x06 external_count: i16,
        0x08 reference_count: i32,
        0x0c class: Ptr<HkDataClassDict>,
        /// `hkArray` of 0x10-byte value records: `{name, _, value low, value high}`.
        0x10 members: Ptr,
        0x14 member_count: i32,
        0x18 capacity_and_flags: u32,
    }
}

/// The allocator of the memory router (`router + 0x10`).
pub fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The inline `hkArray` destructor: size 0, the buffer returned through the router's allocator
/// (slot `0x10`) unless bit 31 of the capacity word says the array does not own it, then data 0
/// and capacity word `0x8000_0000`. `element_size` is the exe's per-element byte multiplier.
pub fn array_free(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// An empty `hkArray` that owns nothing: data 0, size 0, capacity word `0x8000_0000`.
pub fn array_init_empty(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// First half of `hkArray::pushBack`: when `size` equals the capacity the array grows through
/// `hkArrayUtil` with the router's allocator.
pub fn array_grow_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, array, element_size]);
    }
}

/// `hkArray<void*>::pushBack` as the exe inlines it.
pub fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    array_grow_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// `hkArray::reserve` growth as the exe inlines it: when the capacity (without its flag bits)
/// is below `wanted`, `hkArrayUtil::_reserve` gets the larger of `wanted` and twice the
/// capacity.
pub fn array_reserve(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let new_capacity = capacity.wrapping_add(capacity).max(wanted);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// Drops one reference of the counted object at `object` (field `+8`) and runs its deleting
/// destructor (slot `0`, argument `1`) when the count reaches zero.
pub fn release_object(e: &mut Engine, object: u32) {
    let count = e.mem.u32(object + 8).wrapping_sub(1);
    e.mem.set_u32(object + 8, count);
    if count == 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

/// [`release_object`] for a pointer that may be null.
pub fn release_object_if_set(e: &mut Engine, object: u32) {
    if object != 0 {
        release_object(e, object);
    }
}

/// `INC [object+8]`.
pub fn add_object_reference(e: &mut Engine, object: u32) {
    let count = e.mem.u32(object + 8).wrapping_add(1);
    e.mem.set_u32(object + 8, count);
}

/// The inlined string interning: the copy of `name` kept in `map` (the tracker's
/// [`tracker::INTERNED_NAMES`]), made with `hkString::strDup` and inserted when missing.
/// `name` must not be null.
pub fn intern_string(e: &mut Engine, map: u32, name: u32) -> u32 {
    let mut interned = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, name, 0u32])
        .u32();
    if interned == 0 {
        interned = e.call(STRING_DUP, &args![name]).u32();
        e.call(STRING_MAP_INSERT, &args![map, interned, interned]);
    }
    interned
}

/// [`intern_string`] through a tracker, with the null name staying null.
pub fn intern_name(e: &mut Engine, tracker_address: u32, name: u32) -> u32 {
    if name == 0 {
        0
    } else {
        intern_string(e, tracker_address + tracker::INTERNED_NAMES, name)
    }
}

/// `hkCachedHashMap::isValid(&out, iterator)`: the flag byte it stores.
pub fn string_map_is_valid(e: &mut Engine, map: u32, iterator: u32) -> bool {
    e.with_stack(4, |e, flag| {
        e.call(STRING_MAP_IS_VALID, &args![map, flag.addr(), iterator]);
        e.mem.u8(flag.addr()) != 0
    })
}

/// The iteration loop the exe inlines: `getIterator`, then while `isValid` calls `body` on the
/// entry's `getValue` and moves on with `getNext`.
pub fn string_map_for_each(e: &mut Engine, map: u32, mut body: impl FnMut(&mut Engine, u32)) {
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![map]).u32();
    while string_map_is_valid(e, map, iterator) {
        let value = e.call(STRING_MAP_GET_VALUE, &args![map, iterator]).u32();
        body(e, value);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![map, iterator]).u32();
    }
}

/// The array implementation behind a tuple view (`view.tuples.elements`).
fn view_elements(e: &Engine, this: Ptr<ArrayOfTuplesView>) -> u32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.get(tuples, ArrayOfTuplesImplementation::elements).addr()
}

/// The element index `index` of a tuple view in the element array.
fn view_element_index(e: &Engine, this: Ptr<ArrayOfTuplesView>, index: i32) -> i32 {
    e.get(this, ArrayOfTuplesView::first_index)
        .wrapping_add(index)
}

// Translated from 00dc6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::getType` (Xbox PDB): the type of the owning array of
/// tuples (virtual slot `4` of the object at `this + 0xc`) with the array bit `0x20` toggled.
pub fn array_of_tuples_view_get_type(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) -> u32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.vcall(tuples.addr(), 4, &args![]).u32() ^ 0x20
}

// Translated from 00dc6fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::clear` (Xbox PDB): tail-calls slot `8` of the element
/// array.
pub fn array_of_tuples_view_clear(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) {
    let elements = view_elements(e, this);
    e.vcall(elements, 8, &args![]);
}

// Translated from 00dc6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::getSize` (Xbox PDB): the number of elements per tuple.
pub fn array_of_tuples_view_get_size(e: &mut Engine, this: Ptr<ArrayOfTuplesView>) -> i32 {
    let tuples = e.get(this, ArrayOfTuplesView::tuples);
    e.get(tuples, ArrayOfTuplesImplementation::tuple_size)
}

// Translated from 00dc6ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setAll` (Xbox PDB): stores `count` bytes from `values`
/// one by one through slot `0x44` of the element array (`setInt`), at the tuple's indices.
pub fn array_of_tuples_view_set_all(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    values: u32,
    count: i32,
) {
    let mut i = 0;
    while i < count {
        let elements = view_elements(e, this);
        let value = u32::from(e.mem.u8(values.wrapping_add(i as u32)));
        let index = view_element_index(e, this, i);
        e.vcall(elements, 0x44, &args![index, value]);
        i += 1;
    }
}

// Translated from 00dc7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setAll_ov2` (Xbox PDB): stores `count` floats from
/// `values` one by one through slot `0x34` of the element array (`setReal`); each goes
/// through an `FLD`/`FSTP` pair.
pub fn array_of_tuples_view_set_all_ov2(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    values: u32,
    count: i32,
) {
    let mut i = 0;
    while i < count {
        let elements = view_elements(e, this);
        let bits = x87_quiet_bits(e.mem.u32(values.wrapping_add((i as u32).wrapping_mul(4))));
        let index = view_element_index(e, this, i);
        e.vcall(elements, 0x34, &args![index, bits]);
        i += 1;
    }
}

// Translated from 00dc7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asVec` (Xbox PDB): tail-calls slot `0x20` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_vec(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x20, &args![index]).u32()
}

// Translated from 00dc7090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setVec` (Xbox PDB): slot `0x24` of the element array
/// (`setVec`) with the element index and `value`.
pub fn array_of_tuples_view_set_vec(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x24, &args![index, value]);
}

// Translated from 00dc70c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asString` (Xbox PDB): tail-calls slot `0x28` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_string(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x28, &args![index]).u32()
}

// Translated from 00dc70e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setString` (Xbox PDB): slot `0x2c` of the element
/// array (`setString`) with the element index and `value`.
pub fn array_of_tuples_view_set_string(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x2c, &args![index, value]);
}

// Translated from 00dc7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asReal` (Xbox PDB): tail-calls slot `0x30` of the
/// element array with the element index. The callee's `ST0` result is passed through
/// unchanged, hence the `f64` result.
pub fn array_of_tuples_view_as_real(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> f64 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x30, &args![index]).f64()
}

// Translated from 00dc7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setReal` (Xbox PDB): slot `0x34` of the element array
/// (`setReal`) with the element index and the float, which goes through an `FLD`/`FSTP` pair.
pub fn array_of_tuples_view_set_real(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: f32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x34, &args![index, x87_quiet(value)]);
}

// Translated from 00dc7160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asInt` (Xbox PDB): tail-calls slot `0x40` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_int(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x40, &args![index]).u32()
}

// Translated from 00dc7180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setInt` (Xbox PDB): slot `0x44` of the element array
/// (`setInt`) with the element index and `value`.
pub fn array_of_tuples_view_set_int(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x44, &args![index, value]);
}

// Translated from 00dc71b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asInt64` (Xbox PDB): tail-calls slot `0x48` of the
/// element array with the element index; its `EDX:EAX` result is passed through.
pub fn array_of_tuples_view_as_int64(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u64 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x48, &args![index]).u64()
}

// Translated from 00dc71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setInt64` (Xbox PDB): slot `0x4c` of the element
/// array (`setInt64`) with the element index and the value as two words (low, high).
pub fn array_of_tuples_view_set_int64(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    low: u32,
    high: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x4c, &args![index, low, high]);
}

// Translated from 00dc7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asObject` (Xbox PDB): tail-calls slot `0x50` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_object(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x50, &args![index]).u32()
}

// Translated from 00dc7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setObject` (Xbox PDB): slot `0x54` of the element
/// array (`setObject`) with the element index and `value`.
pub fn array_of_tuples_view_set_object(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x54, &args![index, value]);
}

// Translated from 00dc7250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::asArray` (Xbox PDB): tail-calls slot `0x58` of the
/// element array with the element index; its result is passed through.
pub fn array_of_tuples_view_as_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
) -> u32 {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x58, &args![index]).u32()
}

// Translated from 00dc7270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::View::setArray` (Xbox PDB): slot `0x5c` of the element array
/// (`setArray`) with the element index and `value`.
pub fn array_of_tuples_view_set_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesView>,
    index: i32,
    value: u32,
) {
    let elements = view_elements(e, this);
    let index = view_element_index(e, this, index);
    e.vcall(elements, 0x5c, &args![index, value]);
}

// Translated from 00dc89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getType` (Xbox PDB): the type of the element array (slot `4`
/// of `this + 0x20`) with the array bit `0x20` set.
pub fn array_of_tuples_implementation_get_type(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> u32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 4, &args![]).u32() | 0x20
}

// Translated from 00dc89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getClass` (Xbox PDB): tail-calls slot `0x18` of the element
/// array; its result is passed through.
pub fn array_of_tuples_implementation_get_class(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> u32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 0x18, &args![]).u32()
}

// Translated from 00dc89f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::clear` (Xbox PDB): tail-calls slot `8` of the element array.
pub fn array_of_tuples_implementation_clear(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    e.vcall(elements, 8, &args![]);
}

// Translated from 00dc8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::reserve` (Xbox PDB): tail-calls slot `0xc` of the element
/// array with `count * tuple_size` elements.
pub fn array_of_tuples_implementation_reserve(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    count: i32,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.vcall(elements, 0xc, &args![tuple_size.wrapping_mul(count)]);
}

// Translated from 00dc8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::setSize` (Xbox PDB): tail-calls slot `0x10` of the element
/// array with `count * tuple_size` elements.
pub fn array_of_tuples_implementation_set_size(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    count: i32,
) {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.vcall(elements, 0x10, &args![tuple_size.wrapping_mul(count)]);
}

// Translated from 00dc8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::getSize` (Xbox PDB): the size of the element array (slot
/// `0x14`) divided, signed, by the tuple size. A zero divisor (or `i32::MIN / -1`) is the
/// divide error the `IDIV` raises.
pub fn array_of_tuples_implementation_get_size(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
) -> i32 {
    let elements = e.get(this, ArrayOfTuplesImplementation::elements).addr();
    let size = e.vcall(elements, 0x14, &args![]).i32();
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    match size.checked_div(tuple_size) {
        Some(count) => count,
        None => panic!("00dc8a40: IDIV faults (divide error)"),
    }
}

// Translated from 00dc8a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::asArray` (Xbox PDB): points the embedded view at tuple
/// `index` (`+0x1c` = `tuple_size * index`) and returns the view (`this + 0xc`).
pub fn array_of_tuples_implementation_as_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    index: i32,
) -> u32 {
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.mem
        .set_i32(this.addr() + 0x1c, tuple_size.wrapping_mul(index));
    this.addr() + 0xc
}

// Translated from 00dc8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArrayOfTuplesImplementation::setArray` (Xbox PDB): points the embedded view at tuple
/// `index`, asks the array `source` for its type (slot `4`) and copies its first `tuple_size`
/// elements into the view, one by one, with the getter and the view's setter of that type:
/// 1 int (`0x40` / `0x44`), 2 int64 (`0x48` / `0x4c`, two words), 3 real (`0x30` / `0x34`, the
/// `ST0` result stored as a float), 4 to 7 vector (`0x20` / `0x24`), 8 and 9 object (`0x50` /
/// `0x54`), 10 string (`0x28` / `0x2c`). Any other type copies nothing.
pub fn array_of_tuples_implementation_set_array(
    e: &mut Engine,
    this: Ptr<ArrayOfTuplesImplementation>,
    index: i32,
    source: u32,
) {
    let tuple_size = e.get(this, ArrayOfTuplesImplementation::tuple_size);
    e.mem
        .set_i32(this.addr() + 0x1c, tuple_size.wrapping_mul(index));
    let kind = e.vcall(source, 4, &args![]).u32().wrapping_sub(1);
    if kind > 9 {
        return;
    }
    let view = this.addr() + 0xc;
    let mut element = 0i32;
    while element < e.get(this, ArrayOfTuplesImplementation::tuple_size) {
        match kind {
            0 => {
                let value = e.vcall(source, 0x40, &args![element]).u32();
                e.vcall(view, 0x44, &args![element, value]);
            }
            1 => {
                let value = e.vcall(source, 0x48, &args![element]).u64();
                e.vcall(
                    view,
                    0x4c,
                    &args![element, value as u32, (value >> 32) as u32],
                );
            }
            2 => {
                let value = e.vcall(source, 0x30, &args![element]).f64();
                e.vcall(view, 0x34, &args![element, (value as f32).to_bits()]);
            }
            3..=6 => {
                let value = e.vcall(source, 0x20, &args![element]).u32();
                e.vcall(view, 0x24, &args![element, value]);
            }
            7 | 8 => {
                let value = e.vcall(source, 0x50, &args![element]).u32();
                e.vcall(view, 0x54, &args![element, value]);
            }
            _ => {
                let value = e.vcall(source, 0x28, &args![element]).u32();
                e.vcall(view, 0x2c, &args![element, value]);
            }
        }
        element += 1;
    }
}

// Translated from 00dc9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::isValid` (Xbox PDB): whether `index` (signed) is below the member count.
pub fn hk_data_object_dict_is_valid(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    index: i32,
) -> bool {
    index < e.get(this, HkDataObjectDict::member_count)
}

// Translated from 00dc9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::getMemberName` (Xbox PDB): the name word of the value record `index`
/// (16 bytes each).
pub fn hk_data_object_dict_get_member_name(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    index: i32,
) -> u32 {
    let records = e.get(this, HkDataObjectDict::members).addr();
    e.mem.u32(records.wrapping_add((index as u32) << 4))
}

// Translated from 00dc9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectDict::getMemberValue` (Xbox PDB): stores `this` and the name word of the value
/// record `index` at `out` and `out + 4`. `EAX` still holds `out` on return.
pub fn hk_data_object_dict_get_member_value(
    e: &mut Engine,
    this: Ptr<HkDataObjectDict>,
    out: u32,
    index: i32,
) -> u32 {
    e.mem.set_u32(out, this.addr());
    let records = e.get(this, HkDataObjectDict::members).addr();
    let name = e.mem.u32(records.wrapping_add((index as u32) << 4));
    e.mem.set_u32(out + 4, name);
    out
}

// Translated from 00dc9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `VecArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_6f4c` belongs to it): `+6` 0, reference count `+8` 0, vtable `0x010d_6f4c`, an empty
/// element array (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the
/// stride `stride` at `+0x18`. Returns `this`.
pub fn vec_array_implementation_construct(e: &mut Engine, this: u32, stride: i32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this, 0x010d_6f4c);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_i32(this + 0x18, stride);
    this
}

// Translated from 00dc9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::getType` (Xbox PDB): the type code for the stride at `+0x18`: 4
/// gives 4, 0xc gives 6, 0x10 gives 7 and anything else gives 0.
pub fn vec_array_implementation_get_type(e: &mut Engine, this: u32) -> u32 {
    match e.mem.u32(this + 0x18) {
        4 => 4,
        0xc => 6,
        0x10 => 7,
        _ => 0,
    }
}

// Translated from 00dc9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::asVec` (Xbox PDB): the address of the vector `index`: the data
/// pointer plus `stride * index * 4` bytes (a signed 32-bit product).
pub fn vec_array_implementation_as_vec(e: &mut Engine, this: u32, index: i32) -> u32 {
    let stride = e.mem.i32(this + 0x18);
    let data = e.mem.u32(this + 0xc);
    data.wrapping_add((stride.wrapping_mul(index) as u32).wrapping_mul(4))
}

// Translated from 00dc9ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::setVec` (Xbox PDB): copies the `stride` floats at `values` to the
/// elements `stride * index + n` of the data, one at a time through an `FLD`/`FSTP` pair.
pub fn vec_array_implementation_set_vec(e: &mut Engine, this: u32, index: i32, values: u32) {
    let mut stride = e.mem.i32(this + 0x18);
    let mut n = 0i32;
    if stride > 0 {
        loop {
            let bits = x87_quiet_bits(e.mem.u32(values.wrapping_add((n as u32).wrapping_mul(4))));
            let data = e.mem.u32(this + 0xc);
            let position = stride.wrapping_mul(index).wrapping_add(n);
            n += 1;
            e.mem
                .set_u32(data.wrapping_add((position as u32).wrapping_mul(4)), bits);
            stride = e.mem.i32(this + 0x18);
            if n >= stride {
                break;
            }
        }
    }
}

// Translated from 00dc9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::getSize` (Xbox PDB): the element count (`+0x10`) divided, signed,
/// by the stride (`+0x18`). A zero divisor (or `i32::MIN / -1`) is the divide error `IDIV`
/// raises.
pub fn vec_array_implementation_get_size(e: &mut Engine, this: u32) -> i32 {
    let count = e.mem.i32(this + 0x10);
    let stride = e.mem.i32(this + 0x18);
    match count.checked_div(stride) {
        Some(size) => size,
        None => panic!("00dc9b00: IDIV faults (divide error)"),
    }
}

// Translated from 00dc9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::clear` (Xbox PDB): sets the element count (`+0x10`) to 0.
pub fn vec_array_implementation_clear(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 0x10, 0);
}

// Translated from 00dc9b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::reserve` (Xbox PDB): reserves `stride * count` elements (4 bytes
/// each) of the element array at `+0xc`.
pub fn vec_array_implementation_reserve(e: &mut Engine, this: u32, count: i32) {
    let wanted = e.mem.i32(this + 0x18).wrapping_mul(count);
    array_reserve(e, this + 0xc, wanted, 4);
}

// Translated from 00dc9bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed function (the map gives it only a range) that answers 8 and reads nothing.
pub fn fn_00dc9bc0(_e: &mut Engine) -> u32 {
    8
}

// Translated from 00dc9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<unsigned char,1>::reserve` (Xbox PDB): reserves `count` bytes of
/// the element array at `+0xc`.
pub fn basic_array_implementation_u8_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 1);
}

// Translated from 00dc9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<__int64,2>::reserve` (Xbox PDB): reserves `count` 8-byte elements
/// of the element array at `+0xc`.
pub fn basic_array_implementation_i64_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 8);
}

// Translated from 00dc9e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BasicArrayImplementation<hkDataObjectImpl*,8>::reserve` (Xbox PDB): reserves `count`
/// 4-byte elements of the element array at `+0xc`.
pub fn basic_array_implementation_object_reserve(e: &mut Engine, this: u32, count: i32) {
    array_reserve(e, this + 0xc, count, 4);
}

// Translated from 00dc9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecArrayImplementation::_scalar_deleting_destructor_` (Xbox PDB): frees the element array
/// at `+0xc` (4-byte elements, unless it does not own its buffer), sets the vtable to
/// `0x010d_62f4` and, when bit 0 of `flags` is set, returns the block to the router's
/// allocator (slot `8`) with the 16-bit size kept at `+4`. Returns `this`.
pub fn vec_array_implementation_deleting_destructor(e: &mut Engine, this: u32, flags: u8) -> u32 {
    array_free(e, this + 0xc, 4);
    e.mem.set_u32(this, 0x010d_62f4);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(this + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dca040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `PointerArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_6fe4` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`), vtable
/// `0x010d_6fe4`, and the element class `class` at `+0x18` (one more reference when not
/// null). Returns `this`.
pub fn pointer_array_implementation_construct(e: &mut Engine, this: u32, class: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_6fe4);
    if class != 0 {
        add_object_reference(e, class);
    }
    e.mem.set_u32(this + 0x18, class);
    this
}

// Translated from 00dca080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PointerArrayImplementation::asObject` (Xbox PDB): the pointer at element `index`.
pub fn pointer_array_implementation_as_object(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00dca090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PointerArrayImplementation::setObject` (Xbox PDB): the new object gains a reference (when
/// not null), the old element loses one (its deleting destructor runs at zero) and the element
/// `index` becomes the new object.
pub fn pointer_array_implementation_set_object(e: &mut Engine, this: u32, index: i32, object: u32) {
    if object != 0 {
        add_object_reference(e, object);
    }
    let data = e.mem.u32(this + 0xc);
    let old = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
    release_object_if_set(e, old);
    let data = e.mem.u32(this + 0xc);
    e.mem
        .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), object);
}

// Translated from 00dca0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `PointerArrayImplementation` (the map gives it only a range; the deleting
/// destructor `0x00dc_a190` runs it): sets the vtable to `0x010d_6fe4`, lets every non-null
/// element lose a reference, releases the element class at `+0x18` (cleared), frees the element
/// array (4-byte elements, unless it does not own its buffer) and sets the vtable to
/// `0x010d_62f4`.
pub fn fn_00dca0f0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, 0x010d_6fe4);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0x10) {
        let data = e.mem.u32(this + 0xc);
        let element = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        release_object_if_set(e, element);
        index += 1;
    }
    let class = e.mem.u32(this + 0x18);
    release_object_if_set(e, class);
    e.mem.set_u32(this + 0x18, 0);
    array_free(e, this + 0xc, 4);
    e.mem.set_u32(this, 0x010d_62f4);
}

// Translated from 00dca1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ByteArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_707c` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the vtable
/// `0x010d_707c`. Returns `this`.
pub fn byte_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_707c);
    this
}

// Translated from 00dca1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::asInt` (Xbox PDB): the byte at `index`, zero-extended.
pub fn byte_array_implementation_as_int(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    u32::from(e.mem.u8(data.wrapping_add(index as u32)))
}

// Translated from 00dca1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::setInt` (Xbox PDB): stores the low byte of `value` at `index`.
pub fn byte_array_implementation_set_int(e: &mut Engine, this: u32, index: i32, value: u8) {
    let data = e.mem.u32(this + 0xc);
    e.mem.set_u8(data.wrapping_add(index as u32), value);
}

// Translated from 00dca210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ByteArrayImplementation::setAll` (Xbox PDB): reserves `count` bytes, stores `count` as the
/// size and copies `count` bytes from `source` with `memcpy`.
pub fn byte_array_implementation_set_all(e: &mut Engine, this: u32, source: u32, count: i32) {
    let array = this + 0xc;
    array_reserve(e, array, count, 1);
    e.mem.set_i32(array + 4, count);
    let data = e.mem.u32(array);
    e.call(MEM_COPY, &args![data, source, count]);
}

// Translated from 00dca260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `IntArrayImplementation` (the map gives it only a range; the vtable
/// `0x010d_7114` belongs to it): `+6` 0, reference count `+8` 0, an empty element array
/// (`+0xc` data 0, `+0x10` size 0, `+0x14` capacity word `0x8000_0000`) and the vtable
/// `0x010d_7114`. Returns `this`.
pub fn int_array_implementation_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0x8000_0000);
    e.mem.set_u32(this, 0x010d_7114);
    this
}

// Translated from 00dca280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::asInt` (Xbox PDB): the low word of the 8-byte element `index`.
pub fn int_array_implementation_as_int(e: &mut Engine, this: u32, index: i32) -> u32 {
    let data = e.mem.u32(this + 0xc);
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8)))
}

// Translated from 00dca290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setInt` (Xbox PDB): stores `value` as a sign-extended 64-bit value
/// in the element `index`.
pub fn int_array_implementation_set_int(e: &mut Engine, this: u32, index: i32, value: i32) {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    e.mem.set_i32(element, value);
    e.mem.set_i32(element + 4, value >> 31);
}

// Translated from 00dca2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::asInt64` (Xbox PDB): the 8-byte element `index` (`EDX:EAX`).
pub fn int_array_implementation_as_int64(e: &mut Engine, this: u32, index: i32) -> u64 {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    let low = e.mem.u32(element);
    let high = e.mem.u32(element + 4);
    u64::from(low) | (u64::from(high) << 32)
}

// Translated from 00dca2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setInt64` (Xbox PDB): stores the two words `low` and `high` in the
/// element `index`.
pub fn int_array_implementation_set_int64(
    e: &mut Engine,
    this: u32,
    index: i32,
    low: u32,
    high: u32,
) {
    let data = e.mem.u32(this + 0xc);
    let element = data.wrapping_add((index as u32).wrapping_mul(8));
    e.mem.set_u32(element, low);
    e.mem.set_u32(element + 4, high);
}

/// The shared body of the three `IntArrayImplementation::setAll` variants: reserves `count`
/// 8-byte elements, stores `count` as the size and, for each byte of `source`, stores the
/// `(low, high)` words `convert` makes of it in the element of that index.
fn int_array_set_all_with(
    e: &mut Engine,
    this: u32,
    source: u32,
    count: i32,
    convert: impl Fn(u8) -> (u32, u32),
) {
    let array = this + 0xc;
    array_reserve(e, array, count, 8);
    e.mem.set_i32(array + 4, count);
    let mut index = 0i32;
    while index < count {
        let byte = e.mem.u8(source.wrapping_add(index as u32));
        let (low, high) = convert(byte);
        let data = e.mem.u32(array);
        let element = data.wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_u32(element, low);
        e.mem.set_u32(element + 4, high);
        index += 1;
    }
}

// Translated from 00dca2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll` (Xbox PDB): sets the array to the `count` bytes at `source`
/// read as booleans (non-zero is 1, zero is 0), as 64-bit values.
pub fn int_array_implementation_set_all(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| (u32::from(byte != 0), 0));
}

// Translated from 00dca360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov2` (Xbox PDB): sets the array to the `count` bytes at
/// `source` read as signed, sign-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov2(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| {
        let value = i32::from(byte as i8);
        (value as u32, (value >> 31) as u32)
    });
}

// Translated from 00dca3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntArrayImplementation::setAll_ov3` (Xbox PDB): sets the array to the `count` bytes at
/// `source` read as unsigned, zero-extended to 64-bit values.
pub fn int_array_implementation_set_all_ov3(e: &mut Engine, this: u32, source: u32, count: i32) {
    int_array_set_all_with(e, this, source, count, |byte| (u32::from(byte), 0));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_9a20, deleting_destructor_00dc9a20(Ptr, u8) -> Ptr),
        entry!(0x00dc_a190, deleting_destructor_00dca190(Ptr, u8) -> Ptr),
        entry!(0x00dc_a720, deleting_destructor_00dca720(Ptr, u8) -> Ptr),
        entry!(0x00dc_b960, deleting_destructor_00dcb960(Ptr, u8) -> Ptr),
        entry!(0x00dc_bf90, deleting_destructor_00dcbf90(Ptr, u8) -> Ptr),
        entry!(0x00dc_c100, deleting_destructor_00dcc100(Ptr, u8) -> Ptr),
        entry!(
            0x00dc_89d0,
            array_of_tuples_implementation_get_type(Ptr<ArrayOfTuplesImplementation>) -> u32
        ),
        entry!(
            0x00dc_89e0,
            array_of_tuples_implementation_get_class(Ptr<ArrayOfTuplesImplementation>) -> u32
        ),
        entry!(
            0x00dc_89f0,
            array_of_tuples_implementation_clear(Ptr<ArrayOfTuplesImplementation>)
        ),
        entry!(
            0x00dc_8a00,
            array_of_tuples_implementation_reserve(Ptr<ArrayOfTuplesImplementation>, i32)
        ),
        entry!(
            0x00dc_8a20,
            array_of_tuples_implementation_set_size(Ptr<ArrayOfTuplesImplementation>, i32)
        ),
        entry!(
            0x00dc_8a40,
            array_of_tuples_implementation_get_size(Ptr<ArrayOfTuplesImplementation>) -> i32
        ),
        entry!(
            0x00dc_8a60,
            array_of_tuples_implementation_as_array(Ptr<ArrayOfTuplesImplementation>, i32) -> u32
        ),
        entry!(
            0x00dc_8a80,
            array_of_tuples_implementation_set_array(Ptr<ArrayOfTuplesImplementation>, i32, u32)
        ),
        entry!(
            0x00dc_9530,
            hk_data_object_dict_is_valid(Ptr<HkDataObjectDict>, i32) -> bool
        ),
        entry!(
            0x00dc_9540,
            hk_data_object_dict_get_member_name(Ptr<HkDataObjectDict>, i32) -> u32
        ),
        entry!(
            0x00dc_9550,
            hk_data_object_dict_get_member_value(Ptr<HkDataObjectDict>, u32, i32) -> u32
        ),
        entry!(0x00dc_9a50, vec_array_implementation_construct(u32, i32) -> u32),
        entry!(0x00dc_9a80, vec_array_implementation_get_type(u32) -> u32),
        entry!(0x00dc_9ab0, vec_array_implementation_as_vec(u32, i32) -> u32),
        entry!(0x00dc_9ad0, vec_array_implementation_set_vec(u32, i32, u32)),
        entry!(0x00dc_9b00, vec_array_implementation_get_size(u32) -> i32),
        entry!(0x00dc_9b10, vec_array_implementation_clear(u32)),
        entry!(0x00dc_9b20, vec_array_implementation_reserve(u32, i32)),
        entry!(0x00dc_9bc0, fn_00dc9bc0() -> u32),
        entry!(0x00dc_9cd0, basic_array_implementation_u8_reserve(u32, i32)),
        entry!(
            0x00dc_9d70,
            basic_array_implementation_i64_reserve(u32, i32)
        ),
        entry!(
            0x00dc_9e20,
            basic_array_implementation_object_reserve(u32, i32)
        ),
        entry!(0x00dc_9fd0, vec_array_implementation_deleting_destructor(u32, u8) -> u32),
        entry!(0x00dc_a040, pointer_array_implementation_construct(u32, u32) -> u32),
        entry!(0x00dc_a080, pointer_array_implementation_as_object(u32, i32) -> u32),
        entry!(
            0x00dc_a090,
            pointer_array_implementation_set_object(u32, i32, u32)
        ),
        entry!(0x00dc_a0f0, fn_00dca0f0(u32)),
        entry!(0x00dc_a1c0, byte_array_implementation_construct(u32) -> u32),
        entry!(0x00dc_a1e0, byte_array_implementation_as_int(u32, i32) -> u32),
        entry!(0x00dc_a1f0, byte_array_implementation_set_int(u32, i32, u8)),
        entry!(
            0x00dc_a210,
            byte_array_implementation_set_all(u32, u32, i32)
        ),
        entry!(0x00dc_a260, int_array_implementation_construct(u32) -> u32),
        entry!(0x00dc_a280, int_array_implementation_as_int(u32, i32) -> u32),
        entry!(0x00dc_a290, int_array_implementation_set_int(u32, i32, i32)),
        entry!(0x00dc_a2b0, int_array_implementation_as_int64(u32, i32) -> u64),
        entry!(
            0x00dc_a2d0,
            int_array_implementation_set_int64(u32, i32, u32, u32)
        ),
        entry!(0x00dc_a2f0, int_array_implementation_set_all(u32, u32, i32)),
        entry!(
            0x00dc_a360,
            int_array_implementation_set_all_ov2(u32, u32, i32)
        ),
        entry!(
            0x00dc_a3c0,
            int_array_implementation_set_all_ov3(u32, u32, i32)
        ),
        entry!(
            0x00dc_6fc0,
            array_of_tuples_view_get_type(Ptr<ArrayOfTuplesView>) -> u32
        ),
        entry!(
            0x00dc_6fd0,
            array_of_tuples_view_clear(Ptr<ArrayOfTuplesView>)
        ),
        entry!(
            0x00dc_6fe0,
            array_of_tuples_view_get_size(Ptr<ArrayOfTuplesView>) -> i32
        ),
        entry!(
            0x00dc_6ff0,
            array_of_tuples_view_set_all(Ptr<ArrayOfTuplesView>, u32, i32)
        ),
        entry!(
            0x00dc_7030,
            array_of_tuples_view_set_all_ov2(Ptr<ArrayOfTuplesView>, u32, i32)
        ),
        entry!(
            0x00dc_7070,
            array_of_tuples_view_as_vec(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7090,
            array_of_tuples_view_set_vec(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_70c0,
            array_of_tuples_view_as_string(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_70e0,
            array_of_tuples_view_set_string(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_7110,
            array_of_tuples_view_as_real(Ptr<ArrayOfTuplesView>, i32) -> f64
        ),
        entry!(
            0x00dc_7130,
            array_of_tuples_view_set_real(Ptr<ArrayOfTuplesView>, i32, f32)
        ),
        entry!(
            0x00dc_7160,
            array_of_tuples_view_as_int(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7180,
            array_of_tuples_view_set_int(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_71b0,
            array_of_tuples_view_as_int64(Ptr<ArrayOfTuplesView>, i32) -> u64
        ),
        entry!(
            0x00dc_71d0,
            array_of_tuples_view_set_int64(Ptr<ArrayOfTuplesView>, i32, u32, u32)
        ),
        entry!(
            0x00dc_7200,
            array_of_tuples_view_as_object(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7220,
            array_of_tuples_view_set_object(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
        entry!(
            0x00dc_7250,
            array_of_tuples_view_as_array(Ptr<ArrayOfTuplesView>, i32) -> u32
        ),
        entry!(
            0x00dc_7270,
            array_of_tuples_view_set_array(Ptr<ArrayOfTuplesView>, i32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dc9a20_frees_only_when_bit_zero_is_set() {
        check(0x00dc_9a20, 0x00dc_96a0);
    }

    #[test]
    fn deleting_destructor_00dca190_frees_only_when_bit_zero_is_set() {
        check(0x00dc_a190, 0x00dc_a0f0);
    }

    #[test]
    fn deleting_destructor_00dca720_frees_only_when_bit_zero_is_set() {
        check(0x00dc_a720, 0x00dc_a6b0);
    }

    #[test]
    fn deleting_destructor_00dcb960_frees_only_when_bit_zero_is_set() {
        check(0x00dc_b960, 0x00dc_b890);
    }

    #[test]
    fn deleting_destructor_00dcbf90_frees_only_when_bit_zero_is_set() {
        check(0x00dc_bf90, 0x00dc_8080);
    }

    #[test]
    fn deleting_destructor_00dcc100_frees_only_when_bit_zero_is_set() {
        check(0x00dc_c100, 0x00dc_bfc0);
    }
}

#[cfg(test)]
mod translated_tests {
    use super::*;

    const TUP: u32 = 0x0900_1000;
    const ELS: u32 = 0x0900_2000;
    const VWS: u32 = 0x0900_3000;
    const SRC: u32 = 0x0900_4000;
    const FREE_SLOT: u32 = 0x0900_5008;
    const BUF_FREE_SLOT: u32 = 0x0900_5010;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn fresh() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument words of every call to `addr`, in order.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == addr)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn words(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(addr + 4 * i)).collect()
    }

    /// A vtable whose slot `s` (every word up to 0x80) is a no-op double at `base + s`.
    fn stub_vtable(e: &mut Engine, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        vtable
    }

    fn stub_object(e: &mut Engine, base: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = stub_vtable(e, base);
        e.mem.set_u32(object, vtable);
        object
    }

    fn grow(e: &mut Engine, array: u32, element_size: u32, capacity: u32) {
        let data = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let fresh_block = e.mem.alloc(capacity * element_size);
        for i in 0..size * element_size {
            let byte = e.mem.u8(data + i);
            e.mem.set_u8(fresh_block + i, byte);
        }
        e.mem.set_u32(array, fresh_block);
        e.mem.set_u32(array + 8, capacity);
    }

    /// The memory router double and its allocator (slot 8 at `FREE_SLOT`, slot 0x10 at
    /// `BUF_FREE_SLOT`), with doubles of `hkArrayUtil::_reserve` and `memcpy`.
    fn router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        e.mem.set_u32(vtable + 0x10, BUF_FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router_block = e.mem.alloc(0x20);
        e.mem.set_u32(router_block + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router_block));
        e.register(FREE_SLOT, |_, _| Ret::default());
        e.register(BUF_FREE_SLOT, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[3], a[2]);
            Ret::default()
        });
        e.register(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        allocator
    }

    /// A tuple array (tuple size 3) with its embedded view; returns `(view, tuples, elements)`.
    fn view_world(e: &mut Engine) -> (u32, u32, u32) {
        let tuples = stub_object(e, TUP, 0x28);
        let elements = stub_object(e, ELS, 0x20);
        let vtable = stub_vtable(e, VWS);
        e.mem.set_u32(tuples + 0xc, vtable);
        e.mem.set_u32(tuples + 0x18, tuples);
        e.mem.set_u32(tuples + 0x20, elements);
        e.mem.set_u32(tuples + 0x24, 3);
        e.mem.set_i32(tuples + 0x1c, 10);
        (tuples + 0xc, tuples, elements)
    }

    #[test]
    fn view_get_type_toggles_the_array_bit() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        e.register(TUP + 4, |_, _| ret(0x2a));
        assert_eq!(e.call(0x00dc_6fc0, &args![view]).u32(), 0x0a);
        assert_eq!(calls(&e, TUP + 4), vec![vec![tuples]]);
    }

    #[test]
    fn view_clear_calls_slot_8_of_the_elements() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_6fd0, &args![view]);
        assert_eq!(calls(&e, ELS + 8), vec![vec![elements]]);
    }

    #[test]
    fn view_get_size_is_the_tuple_size() {
        let mut e = fresh();
        let (view, _, _) = view_world(&mut e);
        assert_eq!(e.call(0x00dc_6fe0, &args![view]).i32(), 3);
    }

    #[test]
    fn view_set_all_stores_each_byte_through_set_int() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        let values = e.mem.alloc(8);
        e.mem.write(values, &[5, 6, 7]);
        e.call(0x00dc_6ff0, &args![view, values, 3i32]);
        assert_eq!(
            calls(&e, ELS + 0x44),
            vec![
                vec![elements, 10, 5],
                vec![elements, 11, 6],
                vec![elements, 12, 7]
            ]
        );
    }

    #[test]
    fn view_set_all_ov2_stores_each_float_through_set_real() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 0x3f80_0000);
        e.mem.set_u32(values + 4, 0x7f80_0001);
        e.call(0x00dc_7030, &args![view, values, 2i32]);
        assert_eq!(
            calls(&e, ELS + 0x34),
            vec![
                vec![elements, 10, 0x3f80_0000],
                vec![elements, 11, 0x7fc0_0001]
            ]
        );
    }

    #[test]
    fn view_as_vec_forwards_slot_0x20() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x20, |_, _| ret(0xabcd));
        assert_eq!(e.call(0x00dc_7070, &args![view, 2i32]).u32(), 0xabcd);
        assert_eq!(calls(&e, ELS + 0x20), vec![vec![elements, 12]]);
    }

    #[test]
    fn view_set_vec_forwards_slot_0x24() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7090, &args![view, 2i32, 0x55u32]);
        assert_eq!(calls(&e, ELS + 0x24), vec![vec![elements, 12, 0x55]]);
    }

    #[test]
    fn view_as_string_forwards_slot_0x28() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x28, |_, _| ret(0x77));
        assert_eq!(e.call(0x00dc_70c0, &args![view, 1i32]).u32(), 0x77);
        assert_eq!(calls(&e, ELS + 0x28), vec![vec![elements, 11]]);
    }

    #[test]
    fn view_set_string_forwards_slot_0x2c() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_70e0, &args![view, 1i32, 0x66u32]);
        assert_eq!(calls(&e, ELS + 0x2c), vec![vec![elements, 11, 0x66]]);
    }

    #[test]
    fn view_as_real_passes_the_st0_result_through() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x30, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00dc_7110, &args![view, 0i32]).f64(), 2.5);
        assert_eq!(calls(&e, ELS + 0x30), vec![vec![elements, 10]]);
    }

    #[test]
    fn view_set_real_passes_the_float_as_quieted_bits() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7130, &args![view, 1i32, 1.5f32]);
        e.call(0x00dc_7130, &args![view, 2i32, f32::from_bits(0x7f80_0001)]);
        assert_eq!(
            calls(&e, ELS + 0x34),
            vec![
                vec![elements, 11, 1.5f32.to_bits()],
                vec![elements, 12, 0x7fc0_0001]
            ]
        );
    }

    #[test]
    fn view_as_int_forwards_slot_0x40() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x40, |_, _| ret(9));
        assert_eq!(e.call(0x00dc_7160, &args![view, 4i32]).u32(), 9);
        assert_eq!(calls(&e, ELS + 0x40), vec![vec![elements, 14]]);
    }

    #[test]
    fn view_set_int_forwards_slot_0x44() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7180, &args![view, 4i32, 9u32]);
        assert_eq!(calls(&e, ELS + 0x44), vec![vec![elements, 14, 9]]);
    }

    #[test]
    fn view_as_int64_passes_both_result_words_through() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x48, |_, _| Ret {
            eax: 1,
            edx: 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00dc_71b0, &args![view, 0i32]).u64(), 0x2_0000_0001);
        assert_eq!(calls(&e, ELS + 0x48), vec![vec![elements, 10]]);
    }

    #[test]
    fn view_set_int64_forwards_both_words() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_71d0, &args![view, 0i32, 7u32, 8u32]);
        assert_eq!(calls(&e, ELS + 0x4c), vec![vec![elements, 10, 7, 8]]);
    }

    #[test]
    fn view_as_object_forwards_slot_0x50() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x50, |_, _| ret(0x31));
        assert_eq!(e.call(0x00dc_7200, &args![view, 1i32]).u32(), 0x31);
        assert_eq!(calls(&e, ELS + 0x50), vec![vec![elements, 11]]);
    }

    #[test]
    fn view_set_object_forwards_slot_0x54() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7220, &args![view, 1i32, 0x32u32]);
        assert_eq!(calls(&e, ELS + 0x54), vec![vec![elements, 11, 0x32]]);
    }

    #[test]
    fn view_as_array_forwards_slot_0x58() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.register(ELS + 0x58, |_, _| ret(0x41));
        assert_eq!(e.call(0x00dc_7250, &args![view, 2i32]).u32(), 0x41);
        assert_eq!(calls(&e, ELS + 0x58), vec![vec![elements, 12]]);
    }

    #[test]
    fn view_set_array_forwards_slot_0x5c() {
        let mut e = fresh();
        let (view, _, elements) = view_world(&mut e);
        e.call(0x00dc_7270, &args![view, 2i32, 0x42u32]);
        assert_eq!(calls(&e, ELS + 0x5c), vec![vec![elements, 12, 0x42]]);
    }

    #[test]
    fn tuples_get_type_sets_the_array_bit() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.register(ELS + 4, |_, _| ret(5));
        assert_eq!(e.call(0x00dc_89d0, &args![tuples]).u32(), 0x25);
    }

    #[test]
    fn tuples_get_class_forwards_slot_0x18() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.register(ELS + 0x18, |_, _| ret(0x99));
        assert_eq!(e.call(0x00dc_89e0, &args![tuples]).u32(), 0x99);
        assert_eq!(calls(&e, ELS + 0x18), vec![vec![elements]]);
    }

    #[test]
    fn tuples_clear_forwards_slot_8() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_89f0, &args![tuples]);
        assert_eq!(calls(&e, ELS + 8), vec![vec![elements]]);
    }

    #[test]
    fn tuples_reserve_multiplies_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_8a00, &args![tuples, 3i32]);
        assert_eq!(calls(&e, ELS + 0xc), vec![vec![elements, 9]]);
    }

    #[test]
    fn tuples_set_size_multiplies_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, elements) = view_world(&mut e);
        e.call(0x00dc_8a20, &args![tuples, 3i32]);
        assert_eq!(calls(&e, ELS + 0x10), vec![vec![elements, 9]]);
    }

    #[test]
    fn tuples_get_size_divides_by_the_tuple_size() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.register(ELS + 0x14, |_, _| ret(12));
        assert_eq!(e.call(0x00dc_8a40, &args![tuples]).i32(), 4);
    }

    #[test]
    #[should_panic(expected = "IDIV faults")]
    fn tuples_get_size_with_a_zero_tuple_size_faults() {
        let mut e = fresh();
        let (_, tuples, _) = view_world(&mut e);
        e.mem.set_u32(tuples + 0x24, 0);
        e.call(0x00dc_8a40, &args![tuples]);
    }

    #[test]
    fn tuples_as_array_positions_the_view() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        assert_eq!(e.call(0x00dc_8a60, &args![tuples, 2i32]).u32(), view);
        assert_eq!(e.mem.i32(tuples + 0x1c), 6);
    }

    #[test]
    fn tuples_set_array_copies_ints_and_reals_through_the_view() {
        let mut e = fresh();
        let (view, tuples, _) = view_world(&mut e);
        let source = stub_object(&mut e, SRC, 0x20);
        e.register(SRC + 4, |_, _| ret(1));
        e.register(SRC + 0x40, |_, _| ret(0x33));
        e.call(0x00dc_8a80, &args![tuples, 3i32, source]);
        assert_eq!(e.mem.i32(tuples + 0x1c), 9);
        assert_eq!(
            calls(&e, VWS + 0x44),
            vec![
                vec![view, 0, 0x33],
                vec![view, 1, 0x33],
                vec![view, 2, 0x33]
            ]
        );
        e.register(SRC + 4, |_, _| ret(3));
        e.register(SRC + 0x30, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.call(0x00dc_8a80, &args![tuples, 0i32, source]);
        assert_eq!(
            calls(&e, VWS + 0x34),
            vec![
                vec![view, 0, 0x3fc0_0000],
                vec![view, 1, 0x3fc0_0000],
                vec![view, 2, 0x3fc0_0000]
            ]
        );
        e.register(SRC + 4, |_, _| ret(11));
        e.call(0x00dc_8a80, &args![tuples, 0i32, source]);
        assert_eq!(calls(&e, VWS + 0x44).len(), 3);
    }

    #[test]
    fn object_dict_is_valid_compares_with_the_member_count() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        e.mem.set_u32(object + 0x14, 2);
        assert!(e.call(0x00dc_9530, &args![object, 1i32]).bool());
        assert!(!e.call(0x00dc_9530, &args![object, 2i32]).bool());
        assert!(e.call(0x00dc_9530, &args![object, -1i32]).bool());
    }

    #[test]
    fn object_dict_get_member_name_reads_the_record_name() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        let records = e.mem.alloc(0x20);
        e.mem.set_u32(records + 0x10, 0xbeef);
        e.mem.set_u32(object + 0x10, records);
        assert_eq!(e.call(0x00dc_9540, &args![object, 1i32]).u32(), 0xbeef);
    }

    #[test]
    fn object_dict_get_member_value_stores_the_object_and_name() {
        let mut e = fresh();
        let object = e.mem.alloc(0x1c);
        let records = e.mem.alloc(0x20);
        e.mem.set_u32(records + 0x10, 0xbeef);
        e.mem.set_u32(object + 0x10, records);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_9550, &args![object, out, 1i32]).u32(), out);
        assert_eq!(words(&e, out, 2), vec![object, 0xbeef]);
    }

    #[test]
    fn vec_array_construct_makes_an_empty_array_with_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x00dc_9a50, &args![this, 12i32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_6f4c);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.i32(this + 0x18), 0xc);
    }

    #[test]
    fn vec_array_get_type_maps_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        for (stride, expected) in [(4, 4), (0xc, 6), (0x10, 7), (8, 0)] {
            e.mem.set_u32(this + 0x18, stride);
            assert_eq!(e.call(0x00dc_9a80, &args![this]).u32(), expected);
        }
    }

    #[test]
    fn vec_array_as_vec_is_the_address_of_the_vector() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x18, 3);
        assert_eq!(e.call(0x00dc_9ab0, &args![this, 2i32]).u32(), 0x1018);
    }

    #[test]
    fn vec_array_set_vec_copies_the_floats_quieted() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x18, 2);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 0x3f80_0000);
        e.mem.set_u32(values + 4, 0x7f80_0001);
        e.call(0x00dc_9ad0, &args![this, 1i32, values]);
        assert_eq!(words(&e, data, 4), vec![0, 0, 0x3f80_0000, 0x7fc0_0001]);
    }

    #[test]
    fn vec_array_get_size_divides_by_the_stride() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 12);
        e.mem.set_u32(this + 0x18, 4);
        assert_eq!(e.call(0x00dc_9b00, &args![this]).i32(), 3);
    }

    #[test]
    #[should_panic(expected = "IDIV faults")]
    fn vec_array_get_size_with_a_zero_stride_faults() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 12);
        e.call(0x00dc_9b00, &args![this]);
    }

    #[test]
    fn vec_array_clear_zeroes_the_size() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 5);
        e.call(0x00dc_9b10, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn vec_array_reserve_asks_for_stride_times_count() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        e.mem.set_u32(this + 0x18, 2);
        e.call(0x00dc_9b20, &args![this, 3i32]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![allocator, this + 0xc, 6, 4]]
        );
    }

    #[test]
    fn unnamed_function_9bc0_answers_8() {
        let mut e = fresh();
        assert_eq!(e.call(0x00dc_9bc0, &args![]).u32(), 8);
    }

    #[test]
    fn basic_array_reserves_use_their_element_sizes() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        e.call(0x00dc_9cd0, &args![this, 5i32]);
        e.call(0x00dc_9d70, &args![this, 20i32]);
        e.call(0x00dc_9e20, &args![this, 40i32]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![allocator, this + 0xc, 5, 1],
                vec![allocator, this + 0xc, 20, 8],
                vec![allocator, this + 0xc, 40, 4]
            ]
        );
    }

    #[test]
    fn vec_array_deleting_destructor_frees_the_data_and_the_block() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u16(this + 4, 0x20);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 4);
        assert_eq!(e.call(0x00dc_9fd0, &args![this, 1u8]).u32(), this);
        assert_eq!(calls(&e, BUF_FREE_SLOT), vec![vec![allocator, data, 16]]);
        assert_eq!(calls(&e, FREE_SLOT), vec![vec![allocator, this, 0x20]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        e.call(0x00dc_9fd0, &args![this, 0u8]);
        assert_eq!(calls(&e, FREE_SLOT).len(), 1);
    }

    #[test]
    fn pointer_array_construct_takes_a_reference_of_the_class() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let class = e.mem.alloc(0x10);
        e.mem.set_u32(class + 8, 1);
        assert_eq!(e.call(0x00dc_a040, &args![this, class]).u32(), this);
        assert_eq!(e.mem.u32(class + 8), 2);
        assert_eq!(e.mem.u32(this + 0x18), class);
        assert_eq!(e.mem.u32(this), 0x010d_6fe4);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
        let other = e.mem.alloc(0x20);
        e.call(0x00dc_a040, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other + 0x18), 0);
    }

    #[test]
    fn pointer_array_as_object_reads_the_element() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 7);
        e.mem.set_u32(data + 4, 8);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a080, &args![this, 1i32]).u32(), 8);
    }

    #[test]
    fn pointer_array_set_object_swaps_the_references() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        let old = stub_object(&mut e, SRC, 0x10);
        let fresh_object = e.mem.alloc(0x10);
        e.mem.set_u32(old + 8, 2);
        e.mem.set_u32(fresh_object + 8, 1);
        e.mem.set_u32(data, old);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a090, &args![this, 0i32, fresh_object]);
        assert_eq!(e.mem.u32(old + 8), 1);
        assert_eq!(e.mem.u32(fresh_object + 8), 2);
        assert_eq!(e.mem.u32(data), fresh_object);
        assert!(calls(&e, SRC).is_empty());
        let newer = e.mem.alloc(0x10);
        e.call(0x00dc_a090, &args![this, 0i32, newer]);
        assert_eq!(e.mem.u32(fresh_object + 8), 1);
        e.mem.set_u32(fresh_object + 8, 1);
        e.mem.set_u32(data, old);
        e.mem.set_u32(old + 8, 1);
        e.call(0x00dc_a090, &args![this, 0i32, 0u32]);
        assert_eq!(calls(&e, SRC), vec![vec![old, 1]]);
        assert_eq!(e.mem.u32(data), 0);
    }

    #[test]
    fn pointer_array_destructor_body_releases_the_elements_and_the_class() {
        let mut e = fresh();
        let allocator = router(&mut e);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        let element = e.mem.alloc(0x10);
        let class = stub_object(&mut e, SRC, 0x10);
        e.mem.set_u32(element + 8, 2);
        e.mem.set_u32(class + 8, 1);
        e.mem.set_u32(data, element);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, class);
        e.call(0x00dc_a0f0, &args![this]);
        assert_eq!(e.mem.u32(element + 8), 1);
        assert_eq!(calls(&e, SRC), vec![vec![class, 1]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, BUF_FREE_SLOT), vec![vec![allocator, data, 8]]);
        assert_eq!(e.mem.u32(this), 0x010d_62f4);
    }

    #[test]
    fn byte_array_construct_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_a1c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_707c);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn byte_array_as_int_reads_a_byte() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.write(data, &[1, 0xfe]);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a1e0, &args![this, 1i32]).u32(), 0xfe);
    }

    #[test]
    fn byte_array_set_int_stores_the_low_byte() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a1f0, &args![this, 1i32, 0xabu8]);
        assert_eq!(e.mem.u8(data + 1), 0xab);
    }

    #[test]
    fn byte_array_set_all_copies_the_bytes() {
        let mut e = fresh();
        router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3]);
        e.call(0x00dc_a210, &args![this, source, 3i32]);
        assert_eq!(e.mem.i32(this + 0x10), 3);
        let data = e.mem.u32(this + 0xc);
        assert_eq!(
            [e.mem.u8(data), e.mem.u8(data + 1), e.mem.u8(data + 2)],
            [1, 2, 3]
        );
    }

    #[test]
    fn int_array_construct_makes_an_empty_array() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_a260, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_7114);
        assert_eq!(words(&e, this + 0xc, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn int_array_as_int_reads_the_low_word() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 0x1234);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a280, &args![this, 1i32]).u32(), 0x1234);
    }

    #[test]
    fn int_array_set_int_sign_extends() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a290, &args![this, 1i32, -2i32]);
        assert_eq!(words(&e, data + 8, 2), vec![0xffff_fffe, 0xffff_ffff]);
    }

    #[test]
    fn int_array_as_int64_reads_both_words() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 1);
        e.mem.set_u32(data + 12, 2);
        e.mem.set_u32(this + 0xc, data);
        assert_eq!(e.call(0x00dc_a2b0, &args![this, 1i32]).u64(), 0x2_0000_0001);
    }

    #[test]
    fn int_array_set_int64_stores_both_words() {
        let mut e = fresh();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, data);
        e.call(0x00dc_a2d0, &args![this, 1i32, 5u32, 6u32]);
        assert_eq!(words(&e, data + 8, 2), vec![5, 6]);
    }

    /// Runs one of the three `setAll` variants on the bytes `[0, 5, 0x80]` and returns the
    /// six words of the array.
    fn int_set_all(address: u32) -> Vec<u32> {
        let mut e = fresh();
        router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0, 5, 0x80]);
        e.call(address, &args![this, source, 3i32]);
        assert_eq!(e.mem.i32(this + 0x10), 3);
        let data = e.mem.u32(this + 0xc);
        words(&e, data, 6)
    }

    #[test]
    fn int_array_set_all_stores_booleans() {
        assert_eq!(int_set_all(0x00dc_a2f0), vec![0, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn int_array_set_all_ov2_sign_extends_the_bytes() {
        assert_eq!(
            int_set_all(0x00dc_a360),
            vec![0, 0, 5, 0, 0xffff_ff80, 0xffff_ffff]
        );
    }

    #[test]
    fn int_array_set_all_ov3_zero_extends_the_bytes() {
        assert_eq!(int_set_all(0x00dc_a3c0), vec![0, 0, 5, 0, 0x80, 0]);
    }
}
