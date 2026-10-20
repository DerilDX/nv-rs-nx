//! `fallout shared/pathfinding/navmeshinfomap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b81d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map key equality: compares full 32-bit words, returns AL.
pub fn keys_equal(_e: &mut Engine, _this: Ptr, left: u32, right: u32) -> u8 {
    u8::from(left == right)
}

/// The virtual slot (byte offset) of the map that answers the bucket index of a key.
const SLOT_BUCKET_OF_KEY: u32 = 0x04;

// Translated from 006b7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`navmeshinfomap.cpp`). `thiscall`, `RET 0xc`: the iterator step of
/// a chained hash map (`this`: vtable, bucket count at +4, bucket array at +8; an entry: next
/// at +0, key at +4, value at +8). `iterator` points to the current entry: its key and value are
/// stored through `key` and `value`, then the iterator moves to the entry's next, or, at the end
/// of the chain, to the first entry of a later non-empty bucket (the current key's bucket index
/// comes from virtual slot +4 of the map), or to null after the last bucket. The key is read
/// from the entry again for the slot call, as the exe does.
pub fn fn_006b7f20(e: &mut Engine, this: Ptr, iterator: Ptr, key: Ptr, value: Ptr) {
    let entry = e.mem.u32(iterator.addr());
    let entry_key = e.mem.u32(entry.wrapping_add(4));
    e.mem.set_u32(key.addr(), entry_key);
    let entry_value = e.mem.u32(entry.wrapping_add(8));
    e.mem.set_u32(value.addr(), entry_value);
    let next = e.mem.u32(entry);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let key_again = e.mem.u32(entry.wrapping_add(4));
    let bucket = e
        .vcall(this.addr(), SLOT_BUCKET_OF_KEY, &args![key_again])
        .u32();
    let mut index = bucket.wrapping_add(1);
    loop {
        if index >= e.mem.u32(this.addr().wrapping_add(4)) {
            e.mem.set_u32(iterator.addr(), 0);
            return;
        }
        let buckets = e.mem.u32(this.addr().wrapping_add(8));
        let first = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if first != 0 {
            e.mem.set_u32(iterator.addr(), first);
            return;
        }
        index = index.wrapping_add(1);
    }
}

/// `TESForm::TESForm` (`00483370`, engine map).
const TES_FORM_CTOR: u32 = 0x0048_3370;
/// `004f15a0` (unnamed in the engine map; `thiscall`, one word): stores the
/// byte at +4 (here `TESForm`'s form type, 0x38).
const SET_FORM_TYPE_004F15A0: u32 = 0x004f_15a0;
/// The vtable of `NavMeshInfoMap` (RTTI `.?AVNavMeshInfoMap@@`).
const NAV_MESH_INFO_MAP_VTABLE: u32 = 0x0106_bb8c;
/// The vtable of `NiTPointerMap<unsigned int, NavMeshInfo*>` (RTTI).
const INFO_MAP_VTABLE: u32 = 0x0106_c134;
/// The vtable of the `NiTMap` of worldspace ids to cell-key maps (RTTI
/// `.?AV?$NiTMap@IPAV?$NiTMap@IPAV?$BSSimpleArray@PAVNavMeshInfo@@...`).
const WORLD_MAP_VTABLE: u32 = 0x0106_c154;
/// The vtable of the `NiTMap` of cell keys to `BSSimpleArray<NavMeshInfo*>*` (RTTI).
const CELL_MAP_VTABLE: u32 = 0x0106_c174;
/// The vtable of the `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>` (RTTI).
const INFO_MAP_BASE_VTABLE: u32 = 0x0106_c194;
/// The vtable of the `NiTMapBase` under the worldspace map (RTTI `NiTMapBase<DFALL<NiTMapItem<...NiTMap...>>>`).
const WORLD_MAP_BASE_VTABLE: u32 = 0x0106_c1b4;
/// The vtable of the `NiTMapBase` under the cell map (RTTI `NiTMapBase<DFALL<NiTMapItem<...BSSimpleArray...>>>`).
const CELL_MAP_BASE_VTABLE: u32 = 0x0106_c1d4;
/// `00aa1070` (unnamed in the engine map; cdecl, the size): the map's
/// bucket array allocation.
const BUCKET_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// `00aa10f0` (unnamed in the engine map; cdecl, the block): frees the
/// bucket array.
const BUCKET_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `00403d30` (unnamed in the engine map; cdecl: destination, byte, count): fills memory.
const FILL_00403D30: u32 = 0x0040_3d30;
/// `00438af0` (unnamed in the engine map): empties a map (`RemoveAll`).
const MAP_REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `004b9ba0` (unnamed in the engine map): a map's first position (the
/// first entry, or null).
const MAP_FIRST_004B9BA0: u32 = 0x004b_9ba0;
/// `006b6390` (`NavMeshInfo` scalar deleting destructor; `thiscall`, `RET 4`).
const NAV_MESH_INFO_DELETE: u32 = 0x006b_6390;
/// `0044ddc0` (unnamed in the engine map): an array's size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `006a7ad0` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_006A7AD0: u32 = 0x006a_7ad0;
/// `00877a30` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `006286d0` (named `LowProcess::GetNumberOfItemsActivated` in the engine
/// map, a folded body): the word the cell check counts non-zero.
const INFO_VALUE_006286D0: u32 = 0x0062_86d0;
/// `00484730` (unnamed in the engine map; `thiscall`, `RET 4`): the
/// `TESForm` step `006b68b0` runs first with its byte.
const FORM_STEP_00484730: u32 = 0x0048_4730;
/// `004534f0` (unnamed in the engine map; `thiscall`, `RET 4`): run on each
/// info with 0.
const INFO_STEP_004534F0: u32 = 0x0045_34f0;
/// `00690830` (unnamed in the engine map): the object the cast checks.
const INFO_PARENT_00690830: u32 = 0x0069_0830;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The cast's source type descriptor (`.?AVTESForm@@`).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
/// The cast's target type descriptor (`.?AVTESObjectCELL@@`).
const CAST_TARGET_TYPE: u32 = 0x0118_3fb4;
/// `00464f30` (unnamed in the engine map; cdecl, one word): the key of a
/// cell or form.
const KEY_OF_00464F30: u32 = 0x0046_4f30;
/// `00853130` (unnamed in the engine map; `thiscall`, `RET 8`: key, the
/// address of the value): a map lookup (true when found).
const MAP_GET_AT_00853130: u32 = 0x0085_3130;
/// `NiTMapBase<..>::SetAt` (`00844700`, engine map; `thiscall`, `RET 8`: key, value).
const MAP_SET_AT_00844700: u32 = 0x0084_4700;
/// `004839c0` (unnamed in the engine map; cdecl, one word): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `005861b0` (unnamed in the engine map): a byte of the worldspace that
/// picks the larger cell map.
const WORLD_FLAG_005861B0: u32 = 0x0058_61b0;
/// `0069aee0` (unnamed in the engine map): the `BSSimpleArray` constructor.
const ARRAY_CTOR_0069AEE0: u32 = 0x0069_aee0;
/// `0069af10` (unnamed in the engine map): the `BSSimpleArray` destructor.
const ARRAY_DTOR_0069AF10: u32 = 0x0069_af10;
/// `00719b20` (unnamed in the engine map; `thiscall`, `RET 0xC`: the address
/// of the item, a start index, a comparison function): the item's index, or -1.
const ARRAY_FIND_00719B20: u32 = 0x0071_9b20;
/// The comparison function `0x009a3830` the search is given.
const COMPARE_009A3830: u32 = 0x009a_3830;
/// `007cb2e0` (unnamed in the engine map; `thiscall`, `RET 4`): appends the
/// item whose address it gets.
const ARRAY_ADD_007CB2E0: u32 = 0x007c_b2e0;
/// `00425fd0` (unnamed in the engine map): a byte of the cell that sends
/// the lookup to the interior map entry.
const CELL_IS_INTERIOR_00425FD0: u32 = 0x0042_5fd0;
/// `TESObjectCELL::GetDataX` (`00544c30`, engine map).
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
/// `TESObjectCELL::GetDataY` (`00544c60`, engine map).
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// `00587410` (unnamed in the engine map; cdecl, x and y): the cell key.
const CELL_KEY_00587410: u32 = 0x0058_7410;
/// `TESObjectCELL::GetWorldSpace` (`0054ddd0`, engine map).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `0084e3a0` (unnamed in the engine map): the worldspace's id word.
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `00408840` (unnamed in the engine map; cdecl, a float): a float result
/// (ST0) compared with the location limit.
const FLOAT_OF_00408840: u32 = 0x0040_8840;
/// The location limit (a double) the info's float at +0x18 is checked against.
const LOCATION_LIMIT: u32 = 0x0106_ab98;
/// `005b5e40` (unnamed in the engine map; cdecl, a format and its words): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find Parent Space with ID %08x for NavmeshInfo %08x"`.
const NO_PARENT_SPACE_FORMAT: u32 = 0x0106_bd60;
/// `"MASTERFILE: NavmeshInfo %08x has bad approx location, ignoring"`.
const BAD_LOCATION_FORMAT: u32 = 0x0106_bd1c;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `NiTMapBase<..>::RemoveAt` (`00405430`, engine map; `thiscall`, `RET 4`).
const MAP_REMOVE_AT_00405430: u32 = 0x0040_5430;
/// `NavMeshInfo::InitDoorPointers` (`006b5490`, engine map).
const INIT_DOOR_POINTERS: u32 = 0x006b_5490;
/// The virtual slot (byte offset) `006b6b20` calls on the map with 1 at the end.
const SLOT_MAP_FINISH: u32 = 0xc8;

/// The shared body of the three `NiTMapBase` constructors: stores
/// `vtable`, the bucket count, clears the entry count at +0xC, allocates
/// the bucket array (`00aa1070`, count * 4 bytes) into +8 and zero-fills it.
fn map_base_ctor(e: &mut Engine, this: Ptr, hash_size: u32, vtable: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + 4, hash_size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.call(BUCKET_ALLOCATE_00AA1070, &args![bytes]).u32();
    e.mem.set_u32(base + 8, buckets);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.mem.u32(base + 8);
    e.call(FILL_00403D30, &args![buckets, 0u32, bytes]);
    this
}

/// The shared body of the three `NiTMapBase` destructors: stores `vtable`,
/// empties the map (`00438af0`) and frees the bucket array (`00aa10f0`).
fn map_base_dtor(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(BUCKET_FREE_00AA10F0, &args![buckets]);
}

// Translated from 006b7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>`
/// constructor (RTTI of the vtable; unnamed in the engine map; `RET 4`).
pub fn fn_006b7eb0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, INFO_MAP_BASE_VTABLE)
}

// Translated from 006b8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor under the worldspace map (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`).
pub fn fn_006b8060(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, WORLD_MAP_BASE_VTABLE)
}

// Translated from 006b8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor under the cell map (RTTI of the vtable;
/// unnamed in the engine map; `RET 4`).
pub fn fn_006b8160(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, CELL_MAP_BASE_VTABLE)
}

// Translated from 006b7d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, NavMeshInfo*>` constructor (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`): the base constructor with
/// `hash_size`, then the class's vtable. Returns `this`.
pub fn fn_006b7d90(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b7eb0(e, this, hash_size);
    e.mem.set_u32(this.addr(), INFO_MAP_VTABLE);
    this
}

// Translated from 006b7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap` of worldspace ids to cell maps (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`). Returns `this`.
pub fn fn_006b7dc0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b8060(e, this, hash_size);
    e.mem.set_u32(this.addr(), WORLD_MAP_VTABLE);
    this
}

// Translated from 006b7df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap` of cell keys to info arrays (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`). Returns `this`.
pub fn fn_006b7df0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b8160(e, this, hash_size);
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    this
}

// Translated from 006b8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>`
/// (unnamed in the engine map).
pub fn fn_006b8030(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, INFO_MAP_BASE_VTABLE);
}

// Translated from 006b8130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMapBase` under the worldspace map (unnamed
/// in the engine map).
pub fn fn_006b8130(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, WORLD_MAP_BASE_VTABLE);
}

// Translated from 006b8250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMapBase` under the cell map (unnamed in the
/// engine map).
pub fn fn_006b8250(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, CELL_MAP_BASE_VTABLE);
}

// Translated from 006b7fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b7fd0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// `NiTPointerMap<unsigned int, NavMeshInfo*>` destructor body (unnamed in
/// the engine map): stores the class's vtable, empties the map
/// (`00438af0`), then the base destructor `006b8030`.
pub fn fn_006b7fd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), INFO_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8030(e, this);
}

// Translated from 006b80d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b80d0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// Destructor body of the worldspace `NiTMap` (unnamed in the engine map):
/// stores the class's vtable, empties the map, then the base destructor
/// `006b8130`.
pub fn fn_006b80d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WORLD_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8130(e, this);
}

// Translated from 006b81f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b81f0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// Destructor body of the cell `NiTMap` (unnamed in the engine map): stores
/// the class's vtable, empties the map, then the base destructor `006b8250`.
pub fn fn_006b81f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8250(e, this);
}

// Translated from 006b7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the cell `NiTMap` (engine map name
/// `NiTMap<unsigned_int_BSSimpleArray<NavMeshInfo_P_1024>_P>::_scalar_deleting_destructor_`;
/// `RET 4`): the destructor body `006b81f0`, then `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006b7e80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b81f0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b5c30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5c30 - its FS:[0] exception frame (the unwind states that destroy the form and the maps when a later constructor throws) is not translated
/// `NavMeshInfoMap::NavMeshInfoMap` (Xbox PDB): the `TESForm` constructor,
/// the class's vtable, clears the byte at +0x18, constructs the info map at
/// +0x1C (`006b7d90`, 0x2719 buckets) and the worldspace map at +0x2C
/// (`006b7dc0`, 0x67 buckets), clears the byte at +0x3C and sets the form
/// type to 0x38 (`004f15a0`). Returns `this`.
pub fn nav_mesh_info_map_nav_mesh_info_map(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(TES_FORM_CTOR, &args![this]);
    e.mem.set_u32(base, NAV_MESH_INFO_MAP_VTABLE);
    e.mem.set_u8(base + 0x18, 0);
    fn_006b7d90(e, Ptr::new(base + 0x1c), 0x2719);
    fn_006b7dc0(e, Ptr::new(base + 0x2c), 0x67);
    e.mem.set_u8(base + 0x3c, 0);
    e.call(SET_FORM_TYPE_004F15A0, &args![this, 0x38u32]);
    this
}

// Translated from 006b77b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `__RTDynamicCast` of what `00690830` returns
/// for `this` from `TESForm` to `TESObjectCELL` (the type descriptors
/// `01183028` and `01183fb4`). Returns the cast result.
pub fn fn_006b77b0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.call(INFO_PARENT_00690830, &args![this]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
    )
    .u32()
}

// Translated from 006b7b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the key (`00464f30`) of the
/// cast object `006b77b0` finds for `info`, or, when that is null, of the
/// word at `info` + 0xC.
pub fn fn_006b7b30(e: &mut Engine, info: Ptr) -> u32 {
    let object = fn_006b77b0(e, info);
    if object != 0 {
        e.call(KEY_OF_00464F30, &args![object]).u32()
    } else {
        let word = e.mem.u32(info.addr() + 0xc);
        e.call(KEY_OF_00464F30, &args![word]).u32()
    }
}

/// `00853130(map, key, out)` with `out` a stack slot: the found flag and
/// the value.
fn map_get(e: &mut Engine, map: u32, key: u32, slot: u32) -> bool {
    e.call(MAP_GET_AT_00853130, &args![map, key, slot]).u8() != 0
}

// Translated from 006b7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the info array of the
/// cell `key` in `world`: the worldspace map at +0x2C looked up with the
/// worldspace's id (`0084e3a0`), then that map looked up with
/// `00464f30(key)`. Null when either lookup fails.
pub fn fn_006b7850(e: &mut Engine, this: Ptr, world: Ptr, key: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let inner_slot = locals.addr();
        let array_slot = locals.addr() + 4;
        let id = e.call(FORM_ID_0084E3A0, &args![world]).u32();
        if !map_get(e, this.addr() + 0x2c, id, inner_slot) {
            return 0;
        }
        let cell_key = e.call(KEY_OF_00464F30, &args![key]).u32();
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, cell_key, array_slot) {
            return 0;
        }
        e.mem.u32(array_slot)
    })
}

// Translated from 006b78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the info array of an
/// interior `cell`: the worldspace map's entry 0, then that map looked up
/// with `00464f30(cell)`. Null when either lookup fails.
pub fn fn_006b78b0(e: &mut Engine, this: Ptr, cell: Ptr) -> u32 {
    e.with_stack(8, |e, locals| {
        let inner_slot = locals.addr();
        let array_slot = locals.addr() + 4;
        if !map_get(e, this.addr() + 0x2c, 0, inner_slot) {
            return 0;
        }
        let cell_key = e.call(KEY_OF_00464F30, &args![cell]).u32();
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, cell_key, array_slot) {
            return 0;
        }
        e.mem.u32(array_slot)
    })
}

// Translated from 006b77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoMap::FindNavMeshInfoForCell` (Xbox PDB; `RET 4`): null for
/// a null cell; for a cell whose byte `00425fd0` is set, `006b78b0`;
/// otherwise `006b7850` with the cell's worldspace and the key
/// `00587410(x, y)` of its grid position (y read first).
pub fn nav_mesh_info_map_find_nav_mesh_info_for_cell(e: &mut Engine, this: Ptr, cell: Ptr) -> u32 {
    if cell.is_null() {
        return 0;
    }
    if e.call(CELL_IS_INTERIOR_00425FD0, &args![cell]).u8() != 0 {
        return fn_006b78b0(e, this, cell);
    }
    let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
    let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
    let key = e.call(CELL_KEY_00587410, &args![x, y]).u32();
    let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr();
    fn_006b7850(e, this, world, key)
}

// Translated from 006b7980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b7980 - its FS:[0] exception frame (the unwind states that free the new map or array block when its constructor throws) is not translated
/// `NavMeshInfoMap::AddInfoToCellKeyMap` (Xbox PDB; `RET 4`): files `info`
/// under its worldspace id (0 when `006b77b0` finds the cast object, else
/// the word at `info` + 4) and its cell key (`006b7b30`). A missing
/// worldspace entry gets a new cell map (0x10 bytes, `006b7df0`) of 0x67
/// buckets, or 0x3F1 for id 0 or a worldspace form whose byte `005861b0`
/// is clear (null when the allocation fails), stored with `SetAt`. A
/// missing cell entry gets a new array (0x10 bytes, `0069aee0`). The info
/// is appended (`007cb2e0`) unless the array already holds it (`00719b20`
/// with the comparison `009a3830` from index 0 is not -1). Both calls get
/// the address of the argument's own stack slot.
pub fn nav_mesh_info_map_add_info_to_cell_key_map(e: &mut Engine, this: Ptr, info: u32) {
    // The argument's slot, the cell map (EBP-0x1C) and the array (EBP-0x10).
    e.with_stack(0xc, |e, locals| {
        let info_slot = locals.addr();
        let inner_slot = locals.addr() + 4;
        let array_slot = locals.addr() + 8;
        e.mem.set_u32(info_slot, info);
        let key = fn_006b7b30(e, Ptr::new(info));
        let world = if fn_006b77b0(e, Ptr::new(info)) != 0 {
            0
        } else {
            e.mem.u32(info.wrapping_add(4))
        };
        if !map_get(e, this.addr() + 0x2c, world, inner_slot) {
            let mut size = 0x67;
            if world == 0 {
                size = 0x3f1;
            } else {
                let form = e.call(LOOKUP_FORM_004839C0, &args![world]).u32();
                if form != 0 && e.call(WORLD_FLAG_005861B0, &args![form]).u8() == 0 {
                    size = 0x3f1;
                }
            }
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let inner = if block != 0 {
                fn_006b7df0(e, Ptr::new(block), size).addr()
            } else {
                0
            };
            e.mem.set_u32(inner_slot, inner);
            e.call(
                MAP_SET_AT_00844700,
                &args![this.addr() + 0x2c, world, inner],
            );
        }
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, key, array_slot) {
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let array = if block != 0 {
                e.call(ARRAY_CTOR_0069AEE0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(array_slot, array);
            let inner = e.mem.u32(inner_slot);
            e.call(MAP_SET_AT_00844700, &args![inner, key, array]);
        }
        let array = e.mem.u32(array_slot);
        let index = e
            .call(
                ARRAY_FIND_00719B20,
                &args![array, info_slot, 0u32, COMPARE_009A3830],
            )
            .u32();
        if index == 0xffff_ffff {
            let array = e.mem.u32(array_slot);
            e.call(ARRAY_ADD_007CB2E0, &args![array, info_slot]);
        }
    });
}

// Translated from 006b7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a check over the worldspace map at +0x2C.
/// For every info array (of every cell map) with more than one entry, it
/// counts the entries whose info word `006286d0` (on the element
/// `006a7ad0` gives) is non-zero; the result is false as soon as one array
/// counts fewer than its size minus one, else true.
pub fn fn_006b7cb0(e: &mut Engine, this: Ptr) -> bool {
    let world_map = Ptr::new(this.addr() + 0x2c);
    // it (EBP-4), key (EBP-0x10), value (EBP-0x14), it2 (EBP-0xC), key2
    // (EBP-0x1C), value2 (EBP-0x18).
    e.with_stack(0x18, |e, locals| {
        let base = locals.addr();
        let (it, key, value) = (base, base + 4, base + 8);
        let (it2, key2, value2) = (base + 0xc, base + 0x10, base + 0x14);
        let mut result = true;
        let first = e.call(MAP_FIRST_004B9BA0, &args![world_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, world_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let inner = e.mem.u32(value);
            let first = e.call(MAP_FIRST_004B9BA0, &args![inner]).u32();
            e.mem.set_u32(it2, first);
            while e.mem.u32(it2) != 0 {
                let inner = e.mem.u32(value);
                fn_006b7f20(
                    e,
                    Ptr::new(inner),
                    Ptr::new(it2),
                    Ptr::new(key2),
                    Ptr::new(value2),
                );
                let array = e.mem.u32(value2);
                if e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() > 1 {
                    let mut counted: u32 = 0;
                    let mut index: u32 = 0;
                    loop {
                        let array = e.mem.u32(value2);
                        if index >= e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
                            break;
                        }
                        let element = e.call(ARRAY_AT_006A7AD0, &args![array, index]).u32();
                        let info = e.mem.u32(element);
                        if e.call(INFO_VALUE_006286D0, &args![info]).u32() != 0 {
                            counted = counted.wrapping_add(1);
                        }
                        index = index.wrapping_add(1);
                    }
                    let array = e.mem.u32(value2);
                    let size = e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32();
                    if counted < size.wrapping_sub(1) {
                        result = false;
                    }
                }
            }
        }
        result
    })
}

// Translated from 006b68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): runs the `TESForm`
/// step `00484730` with `flag`; when `flag` is clear, runs `004534f0(info, 0)`
/// on every info of the map at +0x1C.
pub fn fn_006b68b0(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(FORM_STEP_00484730, &args![this, u32::from(flag)]);
    if flag != 0 {
        return;
    }
    let info_map = Ptr::new(this.addr() + 0x1c);
    // it (EBP-4), key (EBP-0xC), value (EBP-8).
    e.with_stack(0xc, |e, locals| {
        let (it, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let info = e.mem.u32(value);
            e.call(INFO_STEP_004534F0, &args![info, 0u32]);
        }
    });
}

// Translated from 006b6b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the navmesh info map. Every info of
/// the map at +0x1C is deleted (`006b6390(info, 1)`, skipped for null),
/// the map emptied (`00438af0`); then for every cell map of the worldspace
/// map at +0x2C, each of its arrays and then the cell map itself are
/// deleted through their vtable slot 0 with 1 (null skipped), and the
/// worldspace map is emptied. Last, the form's virtual slot +0xC8 runs
/// with 1.
pub fn fn_006b6b20(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    let world_map = Ptr::new(this.addr() + 0x2c);
    // it (EBP-4), key (EBP-0xC), value (EBP-8); the nested walk's it2
    // (EBP-0x14), key2 (EBP-0x20), value2 (EBP-0x1C).
    e.with_stack(0x18, |e, locals| {
        let base = locals.addr();
        let (it, key, value) = (base, base + 4, base + 8);
        let (it2, key2, value2) = (base + 0xc, base + 0x10, base + 0x14);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let info = e.mem.u32(value);
            if info != 0 {
                e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
            }
        }
        e.call(MAP_REMOVE_ALL_00438AF0, &args![info_map]);
        let first = e.call(MAP_FIRST_004B9BA0, &args![world_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, world_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let inner = e.mem.u32(value);
            let first = e.call(MAP_FIRST_004B9BA0, &args![inner]).u32();
            e.mem.set_u32(it2, first);
            while e.mem.u32(it2) != 0 {
                let inner = e.mem.u32(value);
                fn_006b7f20(
                    e,
                    Ptr::new(inner),
                    Ptr::new(it2),
                    Ptr::new(key2),
                    Ptr::new(value2),
                );
                let array = e.mem.u32(value2);
                if array != 0 {
                    e.vcall(array, 0, &args![1u32]);
                }
            }
            let inner = e.mem.u32(value);
            if inner != 0 {
                e.vcall(inner, 0, &args![1u32]);
            }
        }
        e.call(MAP_REMOVE_ALL_00438AF0, &args![world_map]);
    });
    e.vcall(this.addr(), SLOT_MAP_FINISH, &args![1u32]);
}

// Translated from 006b63c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b63c0 - its FS:[0] exception frame (the unwind state that destroys the local array when a call throws) is not translated
/// `NavMeshInfoMap::InitItem` (Xbox PDB): with a local array (`0069aee0`),
/// walks the info map at +0x1C. For each info: its parent space id (+4)
/// is looked up (`004839c0`); a missing one prints the parent-space
/// warning (id, then the map key) and marks the info bad; the float at
/// +0x18 through `00408840` above the location limit prints the
/// bad-location warning (key) and marks it bad too. A bad info goes to the
/// local array (`007cb2e0` with the address of its slot); a good one gets
/// the parent at +0x1C and is filed by `AddInfoToCellKeyMap`. While the
/// byte at +0x3C is clear, each info also runs `InitDoorPointers`. Then
/// every bad info is removed from the map (`RemoveAt` with its first word,
/// `00559450`) and deleted (`006b6390(info, 1)`, null skipped), the byte at
/// +0x3C is set, the check `006b7cb0` runs and the local array is
/// destroyed (`0069af10`).
pub fn nav_mesh_info_map_init_item(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    // The local array (EBP-0x20, 0x10 bytes), it (EBP-0x10), key
    // (EBP-0x30), info (EBP-0x2C).
    e.with_stack(0x1c, |e, locals| {
        let array = locals.addr();
        let (it, key, info_slot) = (array + 0x10, array + 0x14, array + 0x18);
        e.call(ARRAY_CTOR_0069AEE0, &args![array]);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(
                e,
                info_map,
                Ptr::new(it),
                Ptr::new(key),
                Ptr::new(info_slot),
            );
            let mut bad = false;
            let info = e.mem.u32(info_slot);
            let parent_id = e.mem.u32(info.wrapping_add(4));
            let parent = e.call(LOOKUP_FORM_004839C0, &args![parent_id]).u32();
            if parent == 0 {
                let map_key = e.mem.u32(key);
                let parent_id = e.mem.u32(info.wrapping_add(4));
                e.call(
                    PRINT_WARNING_005B5E40,
                    &args![NO_PARENT_SPACE_FORMAT, parent_id, map_key],
                );
                bad = true;
            }
            let info = e.mem.u32(info_slot);
            let location = x87_quiet_bits(e.mem.u32(info.wrapping_add(0x18)));
            let value = e.call(FLOAT_OF_00408840, &args![location]).f64();
            if value > e.global::<f64>(LOCATION_LIMIT) {
                let map_key = e.mem.u32(key);
                e.call(PRINT_WARNING_005B5E40, &args![BAD_LOCATION_FORMAT, map_key]);
                bad = true;
            }
            if bad {
                e.call(ARRAY_ADD_007CB2E0, &args![array, info_slot]);
            } else {
                let info = e.mem.u32(info_slot);
                e.mem.set_u32(info.wrapping_add(0x1c), parent);
                nav_mesh_info_map_add_info_to_cell_key_map(e, this, info);
            }
            if e.mem.u8(this.addr() + 0x3c) == 0 {
                let info = e.mem.u32(info_slot);
                e.call(INIT_DOOR_POINTERS, &args![info]);
            }
        }
        let mut index: u32 = 0;
        while index < e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
            let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
            let info = e.mem.u32(element);
            let info_key = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.call(MAP_REMOVE_AT_00405430, &args![info_map, info_key]);
            let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
            let info = e.mem.u32(element);
            if info != 0 {
                e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
            }
            index = index.wrapping_add(1);
        }
        e.mem.set_u8(this.addr() + 0x3c, 1);
        fn_006b7cb0(e, this);
        e.call(ARRAY_DTOR_0069AF10, &args![array]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_7f20, fn_006b7f20(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x006b81d0, keys_equal(Ptr, u32, u32) -> u8),
        entry!(0x006b_7eb0, fn_006b7eb0(Ptr, u32) -> Ptr),
        entry!(0x006b_8060, fn_006b8060(Ptr, u32) -> Ptr),
        entry!(0x006b_8160, fn_006b8160(Ptr, u32) -> Ptr),
        entry!(0x006b_7d90, fn_006b7d90(Ptr, u32) -> Ptr),
        entry!(0x006b_7dc0, fn_006b7dc0(Ptr, u32) -> Ptr),
        entry!(0x006b_7df0, fn_006b7df0(Ptr, u32) -> Ptr),
        entry!(0x006b_8030, fn_006b8030(Ptr)),
        entry!(0x006b_8130, fn_006b8130(Ptr)),
        entry!(0x006b_8250, fn_006b8250(Ptr)),
        entry!(0x006b_7fd0, fn_006b7fd0(Ptr)),
        entry!(0x006b_80d0, fn_006b80d0(Ptr)),
        entry!(0x006b_81f0, fn_006b81f0(Ptr)),
        entry!(0x006b_7e80, fn_006b7e80(Ptr, u32) -> Ptr),
        entry!(0x006b_5c30, nav_mesh_info_map_nav_mesh_info_map(Ptr) -> Ptr),
        entry!(0x006b_77b0, fn_006b77b0(Ptr) -> u32),
        entry!(0x006b_7b30, fn_006b7b30(Ptr) -> u32),
        entry!(0x006b_7850, fn_006b7850(Ptr, Ptr, u32) -> u32),
        entry!(0x006b_78b0, fn_006b78b0(Ptr, Ptr) -> u32),
        entry!(0x006b_77e0, nav_mesh_info_map_find_nav_mesh_info_for_cell(Ptr, Ptr) -> u32),
        entry!(
            0x006b_7980,
            nav_mesh_info_map_add_info_to_cell_key_map(Ptr, u32)
        ),
        entry!(0x006b_7cb0, fn_006b7cb0(Ptr) -> bool),
        entry!(0x006b_68b0, fn_006b68b0(Ptr, u8)),
        entry!(0x006b_6b20, fn_006b6b20(Ptr)),
        entry!(0x006b_63c0, nav_mesh_info_map_init_item(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_equality_matches_native_low_byte_vectors() {
        // Generated native vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80. Only AL is the boolean result.
        let mut e = Engine::new();
        for (left, right, expected) in [
            (0u32, 0u32, 1u8),
            (0x12345678, 0x12345678, 1),
            (0x12345678, 0x12345679, 0),
            (0, 0x80000000, 0),
        ] {
            assert_eq!(e.call(0x006b81d0, &args![0u32, left, right]).u8(), expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    /// A map with `buckets` buckets whose slot +4 answers `key % buckets`; entries are
    /// (next, key, value).
    fn map(e: &mut Engine, buckets: u32) -> u32 {
        let mut slots = [0u32; 3];
        slots[1] = 0x0f00_a000;
        e.put_vtable(0x0f00_a100, &slots);
        e.register_double(0x0f00_a000, move |_, a| (a[1] % buckets).into_ret());
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map, 0x0f00_a100);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        map
    }

    fn entry(e: &mut Engine, next: u32, key: u32, value: u32) -> u32 {
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, next);
        e.mem.set_u32(entry + 4, key);
        e.mem.set_u32(entry + 8, value);
        entry
    }

    fn step(e: &mut Engine, map: u32, current: u32) -> (u32, u32, u32) {
        let out = e.mem.alloc(0xc);
        e.mem.set_u32(out, current);
        e.call(0x006b_7f20, &args![map, out, out + 4, out + 8]);
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8))
    }

    #[test]
    fn at_006b7f20_follows_the_chain_when_the_entry_has_a_next() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let second = entry(&mut e, 0, 8, 0xbbbb);
        let first = entry(&mut e, second, 4, 0xaaaa);
        assert_eq!(step(&mut e, map, first), (second, 4, 0xaaaa));
    }

    #[test]
    fn at_006b7f20_moves_to_the_next_non_empty_bucket_at_the_end_of_a_chain() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let table = e.mem.u32(map + 8);
        let in_bucket_three = entry(&mut e, 0, 7, 0xdddd);
        e.mem.set_u32(table + 12, in_bucket_three);
        // Key 5 sits in bucket 1; buckets 2 is empty, bucket 3 has an entry.
        let last = entry(&mut e, 0, 5, 0xcccc);
        assert_eq!(step(&mut e, map, last), (in_bucket_three, 5, 0xcccc));
    }

    #[test]
    fn at_006b7f20_ends_with_null_after_the_last_bucket() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let last = entry(&mut e, 0, 7, 0xeeee);
        // Key 7 is in the last bucket (3): nothing follows.
        assert_eq!(step(&mut e, map, last), (0, 7, 0xeeee));
    }
}

#[cfg(test)]
mod info_map_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const MAP_VTABLE: u32 = 0x0f03_0000;
    const BUCKET_OF_KEY: u32 = 0x0f03_1000;

    /// The doubles that walk maps made by `map_of` (one bucket; slot +4
    /// answers bucket 0; `004b9ba0` gives the first entry).
    fn walkable(e: &mut Engine) {
        e.put_vtable(MAP_VTABLE, &[0, BUCKET_OF_KEY]);
        e.register(BUCKET_OF_KEY, |_, _| Ret::default());
        e.register(MAP_FIRST_004B9BA0, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            e.mem.u32(buckets).into_ret()
        });
    }

    /// A one-bucket chained map holding `entries` (key, value) in order.
    fn map_of(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4);
        e.mem.set_u32(map, MAP_VTABLE);
        e.mem.set_u32(map + 4, 1);
        e.mem.set_u32(map + 8, table);
        let mut next = 0;
        for (key, value) in entries.iter().rev() {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry, next);
            e.mem.set_u32(entry + 4, *key);
            e.mem.set_u32(entry + 8, *value);
            next = entry;
        }
        e.mem.set_u32(table, next);
        map
    }

    /// Writes a map made by `map_of` into the map member at `at`.
    fn place_map(e: &mut Engine, at: u32, entries: &[(u32, u32)]) {
        let map = map_of(e, entries);
        for i in 0..4 {
            let word = e.mem.u32(map + 4 * i);
            e.mem.set_u32(at + 4 * i, word);
        }
    }

    /// `00853130` over maps made by `map_of`, `00464f30` as key + 1 and the
    /// worldspace id at +0xC.
    fn lookups(e: &mut Engine) {
        e.register(MAP_GET_AT_00853130, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            let mut entry = e.mem.u32(buckets);
            while entry != 0 {
                if e.mem.u32(entry + 4) == a[1] {
                    let value = e.mem.u32(entry + 8);
                    e.mem.set_u32(a[2], value);
                    return 1u32.into_ret();
                }
                entry = e.mem.u32(entry);
            }
            Ret::default()
        });
        e.register(KEY_OF_00464F30, |_, a| (a[0] + 1).into_ret());
        e.register(FORM_ID_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
    }

    #[test]
    fn at_006b7eb0_family_builds_the_bucket_arrays() {
        for (address, vtable) in [
            (0x006b_7eb0u32, INFO_MAP_BASE_VTABLE),
            (0x006b_8060, WORLD_MAP_BASE_VTABLE),
            (0x006b_8160, CELL_MAP_BASE_VTABLE),
            (0x006b_7d90, INFO_MAP_VTABLE),
            (0x006b_7dc0, WORLD_MAP_VTABLE),
            (0x006b_7df0, CELL_MAP_VTABLE),
        ] {
            let mut e = Engine::new();
            e.register(BUCKET_ALLOCATE_00AA1070, |_, _| 0x0bad_0000u32.into_ret());
            e.register(FILL_00403D30, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0x67u32]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 0x67);
            assert_eq!(e.mem.u32(this + 8), 0x0bad_0000);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            assert_eq!(
                calls_to(&log, BUCKET_ALLOCATE_00AA1070),
                vec![vec![0x67 * 4]]
            );
            assert_eq!(
                calls_to(&log, FILL_00403D30),
                vec![vec![0x0bad_0000, 0, 0x67 * 4]]
            );
        }
    }

    #[test]
    fn at_006b8030_family_empties_and_frees() {
        for (address, removes, vtable) in [
            (0x006b_8030u32, 1usize, INFO_MAP_BASE_VTABLE),
            (0x006b_8130, 1, WORLD_MAP_BASE_VTABLE),
            (0x006b_8250, 1, CELL_MAP_BASE_VTABLE),
            (0x006b_7fd0, 2, INFO_MAP_BASE_VTABLE),
            (0x006b_80d0, 2, WORLD_MAP_BASE_VTABLE),
            (0x006b_81f0, 2, CELL_MAP_BASE_VTABLE),
        ] {
            let mut e = Engine::new();
            e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
            e.register(BUCKET_FREE_00AA10F0, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x0bad_0000);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL_00438AF0).len(), removes);
            assert_eq!(
                calls_to(&log, BUCKET_FREE_00AA10F0),
                vec![vec![0x0bad_0000]]
            );
        }
    }

    #[test]
    fn at_006b7e80_destroys_then_deletes_on_flag_one() {
        for (flags, deletes) in [(0u32, 0usize), (1, 1), (2, 0)] {
            let mut e = Engine::new();
            e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
            e.register(BUCKET_FREE_00AA10F0, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006b_7e80, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), CELL_MAP_BASE_VTABLE);
            assert_eq!(calls_to(&log, DEALLOCATE).len(), deletes);
        }
    }

    #[test]
    fn at_006b5c30_builds_the_form_and_its_maps() {
        let mut e = Engine::new();
        e.register(TES_FORM_CTOR, |_, _| Ret::default());
        e.register(BUCKET_ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(FILL_00403D30, |_, _| Ret::default());
        e.register(SET_FORM_TYPE_004F15A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcd; 0x40]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_5c30, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), NAV_MESH_INFO_MAP_VTABLE);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), INFO_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x20), 0x2719);
        assert_eq!(e.mem.u32(this + 0x2c), WORLD_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x67);
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        assert_eq!(calls_to(&log, TES_FORM_CTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&log, SET_FORM_TYPE_004F15A0),
            vec![vec![this, 0x38]]
        );
    }

    #[test]
    fn at_006b77b0_casts_the_parent_and_006b7b30_keys_it() {
        let mut e = Engine::new();
        e.register(INFO_PARENT_00690830, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(KEY_OF_00464F30, |_, a| (a[0] ^ 0xffff).into_ret());
        let info = e.mem.alloc(0x44);
        e.mem.set_u32(info + 0xc, 0x0c0c);
        e.mem.set_u32(info + 0x40, 0x4040);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_77b0, &args![info]).u32(), 0x4040);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x4040, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(e.call(0x006b_7b30, &args![info]).u32(), 0x4040 ^ 0xffff);
        e.mem.set_u32(info + 0x40, 0);
        assert_eq!(e.call(0x006b_7b30, &args![info]).u32(), 0x0c0c ^ 0xffff);
    }

    #[test]
    fn at_006b7850_and_006b78b0_look_up_both_levels() {
        let mut e = Engine::new();
        lookups(&mut e);
        let cells = map_of(&mut e, &[(0x11, 0xaaaa), (0x501, 0xbbbb)]);
        let interiors = map_of(&mut e, &[(0x601, 0xcccc)]);
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[(0x0100_0001, cells), (0, interiors)]);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0xc, 0x0100_0001);
        assert_eq!(
            e.call(0x006b_7850, &args![this, world, 0x10u32]).u32(),
            0xaaaa
        );
        assert_eq!(e.call(0x006b_7850, &args![this, world, 0x99u32]).u32(), 0);
        e.mem.set_u32(world + 0xc, 0x0200_0000);
        assert_eq!(e.call(0x006b_7850, &args![this, world, 0x10u32]).u32(), 0);
        assert_eq!(e.call(0x006b_78b0, &args![this, 0x600u32]).u32(), 0xcccc);
        assert_eq!(e.call(0x006b_78b0, &args![this, 0x500u32]).u32(), 0);
    }

    #[test]
    fn at_006b77e0_picks_the_interior_or_exterior_lookup() {
        let mut e = Engine::new();
        lookups(&mut e);
        e.register(CELL_IS_INTERIOR_00425FD0, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(CELL_GET_DATA_X, |_, _| 3u32.into_ret());
        e.register(CELL_GET_DATA_Y, |_, _| 5u32.into_ret());
        e.register(CELL_KEY_00587410, |_, a| (a[0] * 0x100 + a[1]).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let cells = map_of(&mut e, &[(0x306, 0xaaaa)]);
        let this = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0xc, 0x77);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell + 4, world);
        place_map(&mut e, this + 0x2c, &[(0x77, cells)]);
        assert_eq!(e.call(0x006b_77e0, &args![this, 0u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_77e0, &args![this, cell]).u32(), 0xaaaa);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| [CELL_GET_DATA_X, CELL_GET_DATA_Y].contains(a))
            .collect();
        assert_eq!(order, vec![CELL_GET_DATA_Y, CELL_GET_DATA_X]);
        assert_eq!(calls_to(&log, CELL_KEY_00587410), vec![vec![3, 5]]);
        // Interior: entry 0, keyed by the cell itself (+1 by the double).
        let interior = e.mem.alloc(8);
        e.mem.set_u8(interior, 1);
        let rooms = map_of(&mut e, &[(interior + 1, 0xdddd)]);
        place_map(&mut e, this + 0x2c, &[(0, rooms)]);
        assert_eq!(e.call(0x006b_77e0, &args![this, interior]).u32(), 0xdddd);
    }

    fn filing(e: &mut Engine) {
        lookups(e);
        e.register(INFO_PARENT_00690830, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(LOOKUP_FORM_004839C0, |_, a| a[0].into_ret());
        e.register(MAP_SET_AT_00844700, |_, _| Ret::default());
    }

    #[test]
    fn at_006b7980_creates_missing_maps_and_appends_once() {
        let mut e = Engine::new();
        filing(&mut e);
        e.register(WORLD_FLAG_005861B0, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BUCKET_ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(FILL_00403D30, |_, _| Ret::default());
        e.register(ARRAY_CTOR_0069AEE0, |_, a| a[0].into_ret());
        e.register(ARRAY_FIND_00719B20, |_, _| 0xffff_ffffu32.into_ret());
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let info = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, info);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[]);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x0100_0001);
        e.mem.set_u32(info + 0xc, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        let sets = calls_to(&log, MAP_SET_AT_00844700);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][..2], [this + 0x2c, 0x0100_0001]);
        let cell_map = sets[0][2];
        // The worldspace's byte is clear: the larger cell map.
        assert_eq!(e.mem.u32(cell_map), CELL_MAP_VTABLE);
        assert_eq!(e.mem.u32(cell_map + 4), 0x3f1);
        assert_eq!(sets[1][..2], [cell_map, 0x43]);
        let array = sets[1][2];
        assert_eq!(e.mem.u32(array + 4), info);
        let find = &calls_to(&log, ARRAY_FIND_00719B20)[0];
        assert_eq!(find[0], array);
        assert_eq!(find[2..], [0, COMPARE_009A3830]);

        // Both maps present and the info already filed: nothing new.
        let filed = map_of(&mut e, &[(0x43, array)]);
        place_map(&mut e, this + 0x2c, &[(0x0100_0001, filed)]);
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MAP_SET_AT_00844700).is_empty());
        assert!(calls_to(&log, ARRAY_ADD_007CB2E0).is_empty());
    }

    #[test]
    fn at_006b7980_stores_null_maps_when_allocation_fails() {
        let mut e = Engine::new();
        filing(&mut e);
        e.register(WORLD_FLAG_005861B0, |_, _| 1u32.into_ret());
        e.register(ALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        // Nothing is ever found (the null cell map is not read).
        e.register(MAP_GET_AT_00853130, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x0100_0001);
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_SET_AT_00844700),
            vec![vec![this + 0x2c, 0x0100_0001, 0], vec![0, 1, 0]]
        );
        assert_eq!(calls_to(&log, WORLD_FLAG_005861B0), vec![vec![0x0100_0001]]);
    }

    #[test]
    fn at_006b7cb0_checks_each_array_count() {
        let mut e = Engine::new();
        walkable(&mut e);
        // An array: size at +0, element words from +4.
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_AT_006A7AD0, |_, a| (a[0] + 4 + 4 * a[1]).into_ret());
        e.register(INFO_VALUE_006286D0, |e, a| e.mem.u32(a[0]).into_ret());
        let array = |e: &mut Engine, flags: &[u32]| {
            let array = e.mem.alloc(4 + 4 * flags.len() as u32);
            e.mem.set_u32(array, flags.len() as u32);
            for (i, flag) in flags.iter().enumerate() {
                let info = e.mem.alloc(4);
                e.mem.set_u32(info, *flag);
                e.mem.set_u32(array + 4 + 4 * i as u32, info);
            }
            array
        };
        let good = array(&mut e, &[1, 0, 1]);
        let single = array(&mut e, &[0]);
        let cells = map_of(&mut e, &[(1, good), (2, single)]);
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[(7, cells)]);
        assert!(e.call(0x006b_7cb0, &args![this]).bool());
        let bad = array(&mut e, &[1, 0, 0]);
        let cells = map_of(&mut e, &[(1, good), (2, bad)]);
        place_map(&mut e, this + 0x2c, &[(7, cells)]);
        assert!(!e.call(0x006b_7cb0, &args![this]).bool());
    }

    #[test]
    fn at_006b68b0_runs_the_form_step_then_each_info() {
        let mut e = Engine::new();
        walkable(&mut e);
        e.register(FORM_STEP_00484730, |_, _| Ret::default());
        e.register(INFO_STEP_004534F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x1c, &[(1, 0x1111), (2, 0x2222)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_68b0, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_STEP_00484730), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&log, INFO_STEP_004534F0),
            vec![vec![0x1111, 0], vec![0x2222, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006b_68b0, &args![this, 0x101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_STEP_00484730), vec![vec![this, 1]]);
        assert!(calls_to(&log, INFO_STEP_004534F0).is_empty());
    }

    #[test]
    fn at_006b6b20_deletes_everything() {
        let mut e = Engine::new();
        walkable(&mut e);
        e.register(NAV_MESH_INFO_DELETE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
        // Deletable objects: slot 0 is the deleting destructor; the cell
        // map's slot +4 is also its bucket-of-key.
        e.put_vtable(0x0f03_2000, &[0x0f03_3000, BUCKET_OF_KEY]);
        e.register(0x0f03_3000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x33];
        slots[0x32] = 0x0f03_4000;
        e.put_vtable(0x0f03_5000, &slots);
        e.register(0x0f03_4000, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f03_2000);
        let cells = map_of(&mut e, &[(1, array), (2, 0)]);
        e.mem.set_u32(cells, 0x0f03_2000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0f03_5000);
        place_map(&mut e, this + 0x1c, &[(1, 0x1111), (2, 0)]);
        place_map(&mut e, this + 0x2c, &[(9, cells)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_6b20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![0x1111, 1]]);
        assert_eq!(
            calls_to(&log, MAP_REMOVE_ALL_00438AF0),
            vec![vec![this + 0x1c], vec![this + 0x2c]]
        );
        assert_eq!(
            calls_to(&log, 0x0f03_3000),
            vec![vec![array, 1], vec![cells, 1]]
        );
        assert_eq!(calls_to(&log, 0x0f03_4000), vec![vec![this, 1]]);
    }

    #[test]
    fn at_006b63c0_files_good_infos_and_drops_bad_ones() {
        let mut e = Engine::new();
        walkable(&mut e);
        filing(&mut e);
        e.map(0x0106_a000, 0x1000);
        e.set_global::<f64>(LOCATION_LIMIT, 100.0);
        e.register(ARRAY_CTOR_0069AEE0, |_, _| Ret::default());
        e.register(ARRAY_DTOR_0069AF10, |_, _| Ret::default());
        // Parent forms exist for ids below 0x1000.
        e.register(LOOKUP_FORM_004839C0, |_, a| {
            let form = if a[0] < 0x1000 { a[0] + 0x10_0000 } else { 0 };
            form.into_ret()
        });
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        e.register(FLOAT_OF_00408840, |_, a| {
            f32::from_bits(a[0]).abs().into_ret()
        });
        // Arrays: size at +0, items from +4.
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let size = e.mem.u32(a[0]);
            let info = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4 + 4 * size, info);
            e.mem.set_u32(a[0], size + 1);
            Ret::default()
        });
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_AT_00877A30, |_, a| (a[0] + 4 + 4 * a[1]).into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MAP_REMOVE_AT_00405430, |_, _| Ret::default());
        e.register(NAV_MESH_INFO_DELETE, |_, _| Ret::default());
        e.register(INIT_DOOR_POINTERS, |_, _| Ret::default());
        // The good info is already filed.
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        let info = |e: &mut Engine, id: u32, parent: u32, location: f32| {
            let info = e.mem.alloc(0x20);
            e.mem.set_u32(info, id);
            e.mem.set_u32(info + 4, parent);
            e.mem.set_u32(info + 0xc, 0x42);
            e.mem.set_f32(info + 0x18, location);
            info
        };
        let good = info(&mut e, 0xa, 0x0100, 5.0);
        let orphan = info(&mut e, 0xb, 0x2000, 5.0);
        let far = info(&mut e, 0xc, 0x0100, -500.0);
        let this = e.mem.alloc(0x40);
        place_map(
            &mut e,
            this + 0x1c,
            &[(0xa, good), (0xb, orphan), (0xc, far)],
        );
        // An empty array, so the closing check (`006b7cb0`) passes it.
        let array = e.mem.alloc(0x10);
        let filed = map_of(&mut e, &[(0x43, array)]);
        place_map(&mut e, this + 0x2c, &[(0x0100, filed)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_63c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(good + 0x1c), 0x0100 + 0x10_0000);
        assert_eq!(e.mem.u32(orphan + 0x1c), 0);
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![
                vec![NO_PARENT_SPACE_FORMAT, 0x2000, 0xb],
                vec![BAD_LOCATION_FORMAT, 0xc],
            ]
        );
        assert_eq!(calls_to(&log, FLOAT_OF_00408840)[0], vec![5.0f32.to_bits()]);
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT_00405430),
            vec![vec![this + 0x1c, 0xb], vec![this + 0x1c, 0xc]]
        );
        assert_eq!(
            calls_to(&log, NAV_MESH_INFO_DELETE),
            vec![vec![orphan, 1], vec![far, 1]]
        );
        assert_eq!(calls_to(&log, INIT_DOOR_POINTERS).len(), 3);
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(calls_to(&log, ARRAY_DTOR_0069AF10).len(), 1);
        // With the byte set, no door pointers.
        place_map(&mut e, this + 0x1c, &[(0xa, good)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_63c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, INIT_DOOR_POINTERS).is_empty());
    }
}
