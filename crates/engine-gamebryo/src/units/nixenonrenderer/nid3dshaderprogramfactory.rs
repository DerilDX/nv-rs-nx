//! `NiXenonRenderer/nid3dshaderprogramfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a642a0` (`thiscall`, `RET 8`, no unit): stores a creator under a name in a map.
const MAP_SET_AT: u32 = 0x00a6_42a0;
/// `0126fa14`: the creator map object.
const CREATOR_MAP_SLOT: u32 = 0x0126_fa14;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00ec7690` (`cdecl`, `strchr`: text, character).
const STRCHR: u32 = 0x00ec_7690;
/// `00ec67aa` (`cdecl`, `tolower`: character).
const TOLOWER: u32 = 0x00ec_67aa;

// Translated from 00e93d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::CreateShaderCreatorMap` (Xbox PDB; no arguments): allocates
/// 0x14 bytes from the memory manager and, when that succeeds, sets the vtable
/// `010f0eb0`, the size 0x25 at `+4`, the word at `+0xc` to 0, a zeroed 0x94-byte slot
/// block at `+8` (`00aa1070`, `memset`), the byte at `+0x10` to 1 and the vtable
/// `010f0f50`, keeping the object in `0126fa14` (null when the allocation fails).
pub fn ni_d3d_shader_program_factory_create_shader_creator_map(e: &mut Engine) {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        return;
    }
    e.mem.set_u32(map, 0x010f_0eb0);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![0x94u32]).u32();
    let bytes = e.mem.u32(map + 4).wrapping_mul(4);
    e.mem.set_u32(map + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(map + 0x10, 1);
    e.mem.set_u32(map, 0x010f_0f50);
    e.mem.set_u32(CREATOR_MAP_SLOT, map);
}

// Translated from 00e93df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderprogramfactory.cpp`; no arguments): when the
/// creator map `0126fa14` is null, makes it with `00e93d80` (a jump in the exe).
pub fn fn_00e93df0(e: &mut Engine) {
    if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
        ni_d3d_shader_program_factory_create_shader_creator_map(e);
    }
}

// Translated from 00e93e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e93e70 - /GS stack cookie check (00ec408c) not modelled
// (the 0x100-byte name buffer is a zeroed heap block)
/// `NiD3DShaderProgramFactory::RegisterShaderCreator` (Xbox PDB; `cdecl`, two words):
/// answers `false` for a null `name` or one containing a '.'; otherwise copies the name
/// lower-cased (`tolower` per character, up to 0x100 characters including the terminator)
/// into a local buffer, makes the creator map when it is missing (`00e93d80`) and stores
/// `creator` in it under the copy (`00a642a0`); answers `true`. A name of 0x100 characters
/// or more leaves the copy without terminator (the exe's buffer is followed by its
/// cookie; here by zero bytes).
pub fn ni_d3d_shader_program_factory_register_shader_creator(
    e: &mut Engine,
    name: Ptr,
    creator: u32,
) -> bool {
    if name.is_null() {
        return false;
    }
    if !e.call(STRCHR, &args![name, 0x2eu32]).ptr::<u8>().is_null() {
        return false;
    }
    e.with_stack(0x104, |e, buffer| {
        let mut index = 0u32;
        let mut source = name.addr();
        loop {
            let character = e.mem.u8(source);
            let lower = e.call(TOLOWER, &args![character as i8 as i32]).u8();
            e.mem.set_u8(buffer.addr() + index, lower);
            if character == 0 {
                break;
            }
            index += 1;
            source = source.wrapping_add(1);
            if index >= 0x100 {
                break;
            }
        }
        if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
            ni_d3d_shader_program_factory_create_shader_creator_map(e);
        }
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        e.call(MAP_SET_AT, &args![map, buffer, creator]);
    });
    true
}

/// `NiD3DShaderProgramFactory`'s single instance (`GetInstance`, `00e92eb0`,
/// returns it).
const FACTORY_INSTANCE: u32 = 0x0126_fa10;
/// The Gamebryo `free` (`run_00aa1010.rs`), cdecl (block).
const NI_FREE: u32 = 0x00aa_10f0;
/// The sized Gamebryo free (`run_00aa1380.rs`), cdecl (block, size).
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The memory manager's free (`00aa4060`), thiscall on `011f6238` (block).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `_strcpy_s` (engine map), cdecl (destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// The map's `RemoveAll` (`run_00b601a0.rs`), thiscall.
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// The map's `GetNext` (thiscall: position, key, value out words), `RET 0xC`.
const MAP_GET_NEXT: u32 = 0x00b5_7f20;
/// The map's `RemoveAt` (`run_00e91fd0.rs`), thiscall (key), `RET 4`.
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
/// The list's `RemoveHead` (`run_00b5fd60.rs`), thiscall.
const LIST_REMOVE_HEAD: u32 = 0x00b6_02d0;
/// The list's `RemoveTail` (`run_00e7cd50.rs`), thiscall.
const LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;
/// The node allocator's deallocate (`run_0045cb90.rs`), thiscall on the
/// allocator (node), `RET 4`.
const NODE_DEALLOCATE: u32 = 0x0045_cee0;
/// Imported `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Vtables (RTTI) of the vertex shader, pixel shader and creator maps:
/// their `NiTStringTemplateMap`, `NiTPointerMap` and `NiTMapBase` levels.
const VERTEX_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0e30;
const VERTEX_POINTER_MAP_VTABLE: u32 = 0x010f_0e10;
const PIXEL_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0e90;
const PIXEL_POINTER_MAP_VTABLE: u32 = 0x010f_0e70;
const CREATOR_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0ef0;
const CREATOR_POINTER_MAP_VTABLE: u32 = 0x010f_0ed0;
const CREATOR_MAP_BASE_VTABLE: u32 = 0x010f_0eb0;

// Translated from 00ae7f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::RemoveHead`
/// (engine map): unlinks the head node (the next one becomes the head with
/// no previous link, or the tail is cleared when there is none), takes its
/// element and clears it, gives the node back to the allocator (`0045cee0`
/// on +8) and counts one less. Returns the element.
pub fn ni_t_pointer_list_base_remove_head(
    e: &mut Engine,
    this: Ptr<crate::types::NiTPointerList>,
) -> u32 {
    use crate::types::NiTPointerList;
    let node = e.get(this, NiTPointerList::m_pkHead);
    let head = e.get(this, NiTPointerList::m_pkHead);
    let next = e.mem.u32(head);
    e.set(this, NiTPointerList::m_pkHead, next);
    if e.get(this, NiTPointerList::m_pkHead) != 0 {
        let head = e.get(this, NiTPointerList::m_pkHead);
        e.mem.set_u32(head + 4, 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
    }
    let element = e.mem.u32(node + 8);
    e.mem.set_u32(node + 8, 0);
    e.call(NODE_DEALLOCATE, &args![this.addr() + 8, node]);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_sub(1));
    element
}

// Translated from 00e92ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: deletes the factory instance (`0126fa10`,
/// vtable slot 0 with 1) when there is one and clears it.
pub fn fn_00e92ec0(e: &mut Engine) {
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    if instance != 0 {
        e.vcall(instance, 0, &args![1u32]);
    }
    e.set_global(FACTORY_INSTANCE, 0u32);
}

// Translated from 00e93000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetNextProgramDirectory` (engine map):
/// with a node at `*position`, moves `*position` to the next node and
/// returns the node's element (the directory); else 0.
pub fn ni_d3d_shader_program_factory_get_next_program_directory(
    e: &mut Engine,
    position: u32,
) -> u32 {
    let node = e.mem.u32(position);
    if node == 0 {
        return 0;
    }
    let next = e.mem.u32(node);
    e.mem.set_u32(position, next);
    e.mem.u32(node + 8)
}

// Translated from 00e93260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes every entry of the creator map
/// (`0126fa14`) whose value is `creator`: from the first non-empty bucket,
/// `GetNext` (`00b57f20`) gives each key and value and `RemoveAt`
/// (`00e91fd0`) removes the matching keys.
pub fn fn_00e93260(e: &mut Engine, creator: u32) {
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    if map == 0 {
        return;
    }
    let size = e.mem.u32(map + 4);
    let mut first = 0u32;
    let mut index = 0u32;
    while index < size {
        let buckets = e.mem.u32(map + 8);
        let bucket = e.mem.u32(buckets + 4 * index);
        if bucket != 0 {
            first = bucket;
            break;
        }
        index += 1;
    }
    if first == 0 {
        return;
    }
    // The locals: the position, the key and the value.
    e.with_stack(0xc, |e, locals| {
        let value_slot = locals.addr();
        let key_slot = locals.addr() + 4;
        let position = locals.addr() + 8;
        e.mem.set_u32(position, first);
        loop {
            let map = e.mem.u32(CREATOR_MAP_SLOT);
            e.mem.set_u32(key_slot, 0);
            e.mem.set_u32(value_slot, 0);
            e.call(MAP_GET_NEXT, &args![map, position, key_slot, value_slot]);
            if e.mem.u32(value_slot) == creator {
                let key = e.mem.u32(key_slot);
                let map = e.mem.u32(CREATOR_MAP_SLOT);
                e.call(MAP_REMOVE_AT, &args![map, key]);
            }
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
}

// Translated from 00e933d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the creator map (`0126fa14`,
/// `00b601a0`), deletes it (vtable slot 0 with 1) when there is one and
/// clears it.
pub fn fn_00e933d0(e: &mut Engine) {
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    if map != 0 {
        e.vcall(map, 0, &args![1u32]);
    }
    e.mem.set_u32(CREATOR_MAP_SLOT, 0);
}

/// The destructor body the three `NiTStringTemplateMap` instances share:
/// the template vtable; with the copy flag (+0x10) every key string of
/// every bucket's items freed; the pointer-map vtable and `RemoveAll`; the
/// map-base vtable and `RemoveAll` again; the bucket array (+8) freed.
fn string_template_map_destruct(e: &mut Engine, this: Ptr, vtables: [u32; 3]) {
    let base = this.addr();
    e.mem.set_u32(base, vtables[0]);
    if e.mem.u8(base + 0x10) != 0 {
        let mut index = 0u32;
        while index < e.mem.u32(base + 4) {
            let buckets = e.mem.u32(base + 8);
            let mut item = e.mem.u32(buckets + 4 * index);
            while item != 0 {
                let key = e.mem.u32(item + 4);
                item = e.mem.u32(item);
                e.call(NI_FREE, &args![key]);
            }
            index += 1;
        }
    }
    e.mem.set_u32(base, vtables[1]);
    e.call(MAP_REMOVE_ALL, &args![base]);
    e.mem.set_u32(base, vtables[2]);
    e.call(MAP_REMOVE_ALL, &args![base]);
    let buckets = e.mem.u32(base + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00e93400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *, NiD3DVertexShader
/// *>, ...>` destructor (RTTI of the vtables it stores): see
/// `string_template_map_destruct`.
pub fn fn_00e93400(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            VERTEX_TEMPLATE_MAP_VTABLE,
            VERTEX_POINTER_MAP_VTABLE,
            FIRST_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e93470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string map's `SetValue` (unnamed in the engine map): with the copy
/// flag (+0x10) the item's key (+4) becomes a copy of `key` (`strlen + 1`
/// bytes from `00aa1070`, `_strcpy_s`), else `key` itself; the item's value
/// (+8) becomes `value`.
pub fn fn_00e93470(e: &mut Engine, this: Ptr, item: u32, key: u32, value: u32) {
    if e.mem.u8(this.addr() + 0x10) == 0 {
        e.mem.set_u32(item + 4, key);
        e.mem.set_u32(item + 8, value);
        return;
    }
    let mut length = 0u32;
    while e.mem.u8(key.wrapping_add(length)) != 0 {
        length += 1;
    }
    let size = length + 1;
    let copy = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    e.mem.set_u32(item + 4, copy);
    e.call(STRCPY_S, &args![copy, size, key]);
    e.mem.set_u32(item + 8, value);
}

// Translated from 00e934d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *, NiD3DPixelShader
/// *>, ...>` destructor: see `string_template_map_destruct`.
pub fn fn_00e934d0(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            PIXEL_TEMPLATE_MAP_VTABLE,
            PIXEL_POINTER_MAP_VTABLE,
            SECOND_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e93540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *,
/// NiD3DShaderProgramCreator *>, ...>` destructor: see
/// `string_template_map_destruct`.
pub fn fn_00e93540(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            CREATOR_TEMPLATE_MAP_VTABLE,
            CREATOR_POINTER_MAP_VTABLE,
            CREATOR_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e935b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string map's `ClearValue` (unnamed in the engine map): with the copy
/// flag (+0x10), frees the item's key (+4).
pub fn fn_00e935b0(e: &mut Engine, this: Ptr, item: u32) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let key = e.mem.u32(item + 4);
        e.call(NI_FREE, &args![key]);
    }
}

/// A scalar deleting destructor's tail: with bit 0 of `flags` the object
/// freed through the memory manager.
fn free_if_flagged(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e93650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DVertexShader *>::scalar deleting
/// destructor` (engine map): `00e93400`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e93650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e93400(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 00e936f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DPixelShader *>::scalar deleting
/// destructor` (engine map): `00e934d0`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e936f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e934d0(e, this);
    free_if_flagged(e, this, flags)
}

/// `00e93790` and `00e937c0`: with a factory instance and a non-null
/// `shader`, removes the shader's name (its vtable +0x50) from the
/// factory's map at `map_offset` when both exist.
fn remove_shader_name(e: &mut Engine, shader: u32, map_offset: u32) {
    if e.global::<u32>(FACTORY_INSTANCE) == 0 || shader == 0 {
        return;
    }
    let name = e.vcall(shader, 0x50, &args![]).u32();
    if name == 0 {
        return;
    }
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    let map = e.mem.u32(instance + map_offset);
    if map != 0 {
        e.call(MAP_REMOVE_AT, &args![map, name]);
    }
}

// Translated from 00e93790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes the vertex shader's name from
/// the factory's vertex shader map (+0x14).
pub fn fn_00e93790(e: &mut Engine, shader: u32) {
    remove_shader_name(e, shader, 0x14);
}

// Translated from 00e937c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes the pixel shader's name from
/// the factory's pixel shader map (+0x18).
pub fn fn_00e937c0(e: &mut Engine, shader: u32) {
    remove_shader_name(e, shader, 0x18);
}

// Translated from 00e93ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DShaderProgramCreator *>::scalar deleting
/// destructor` (engine map): `00e93540`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e93ab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e93540(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 00e93cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::RemoveAllProgramDirectories` (engine map):
/// for each node of the directory list (+8), takes its directory; a non-null
/// one is looked up in the list from the head and its node removed (the
/// head through `RemoveHead`, the tail through `RemoveTail`, any other one
/// unlinked here and pushed back on the shared node pool under its lock,
/// the count dropping by one), then the directory string is freed.
pub fn ni_d3d_shader_program_factory_remove_all_program_directories(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 8;
    let mut node = e.mem.u32(list);
    while node != 0 {
        let directory = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if directory == 0 {
            continue;
        }
        let head = e.mem.u32(list);
        let mut cursor = head;
        let mut found = 0u32;
        while cursor != 0 {
            let candidate = cursor;
            cursor = e.mem.u32(cursor);
            if directory == e.mem.u32(candidate + 8) {
                found = candidate;
                break;
            }
        }
        if found != 0 {
            if found == head {
                e.call(LIST_REMOVE_HEAD, &args![list]);
            } else if found == e.mem.u32(list + 4) {
                e.call(LIST_REMOVE_TAIL, &args![list]);
            } else {
                let previous = e.mem.u32(found + 4);
                let next = e.mem.u32(found);
                if previous != 0 {
                    e.mem.set_u32(previous, next);
                }
                if next != 0 {
                    e.mem.set_u32(next + 4, previous);
                }
                e.mem.set_u32(found + 8, 0);
                e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
                e.mem.set_u32(found + 4, 0);
                let free = e.mem.u32(NODE_POOL_FREE_LIST);
                e.mem.set_u32(found, free);
                let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
                e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
                e.mem.set_u32(NODE_POOL_FREE_LIST, found);
                if count == 0 {
                    e.mem.set_u32(NODE_POOL_LOCK, 0);
                }
                let remaining = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, remaining.wrapping_sub(1));
            }
        }
        e.call(NI_FREE, &args![directory]);
    }
}

// Translated from 00e93e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::~NiD3DShaderProgramFactory` (engine map):
/// the vertex and pixel shader maps (+0x14, +0x18) emptied and deleted
/// (vtable slot 0 with 1) when they exist, the program directories removed
/// and the list emptied (`00b71300`), then the `NiRefObject` vtable and one
/// live object less (`011f4400`).
pub fn ni_d3d_shader_program_factory_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PROGRAM_FACTORY_VTABLE);
    for offset in [0x14, 0x18] {
        let map = e.mem.u32(base + offset);
        if map != 0 {
            e.call(MAP_REMOVE_ALL, &args![map]);
        }
        let map = e.mem.u32(base + offset);
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
    }
    ni_d3d_shader_program_factory_remove_all_program_directories(e, this);
    ni_t_pointer_list_base_remove_all(e, Ptr::new(base + 8));
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e93f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::scalar deleting destructor` (engine map): the
/// destructor, then with bit 0 of `flags` the 0x1C bytes freed
/// (`00aa1460`). Returns `this`.
pub fn ni_d3d_shader_program_factory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_d3d_shader_program_factory_destruct(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x1cu32]);
    }
    this
}

/// The program factory's renderer globals (`0126fa1c` the renderer, `0126fa18` its device,
/// counted with the device's `AddRef`/`Release`, slots 4 and 8, `0126fa20` its shader
/// library word, the renderer's +0x8b8).
const FACTORY_RENDERER_0126FA1C: u32 = 0x0126_fa1c;
const FACTORY_DEVICE_0126FA18: u32 = 0x0126_fa18;
const FACTORY_LIBRARY_0126FA20: u32 = 0x0126_fa20;

// Translated from 00e92f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`), `cdecl`, the renderer: keeps
/// it at `0126fa1c`. With a renderer, the device held at `0126fa18` is released (slot 8) and
/// replaced by the renderer's device (+0x288; counted, slot 4, when not null), and `0126fa20`
/// gets the renderer's +0x8b8. Without one, `0126fa20` is cleared, the held device released
/// and `0126fa18` cleared.
pub fn fn_00e92f70(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(FACTORY_RENDERER_0126FA1C, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(FACTORY_RENDERER_0126FA1C);
        let library = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, library);
    } else {
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, 0);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, 0);
    }
}

/// The `NiRefObject` table and the global object count (`011f4400`, counted up with
/// `InterlockedIncrement`), the factory's table (`010f0f70`) and its two maps' base and
/// final tables (`010f0df0` then `010f0f10`, `010f0e50` then `010f0f30`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const PROGRAM_FACTORY_VTABLE: u32 = 0x010f_0f70;
const FIRST_MAP_BASE_VTABLE: u32 = 0x010f_0df0;
const FIRST_MAP_VTABLE: u32 = 0x010f_0f10;
const SECOND_MAP_BASE_VTABLE: u32 = 0x010f_0e50;
const SECOND_MAP_VTABLE: u32 = 0x010f_0f30;

/// One of the factory's maps (0x14 bytes from the memory manager): base table, 0x3b buckets
/// (0xec bytes of `00aa1070`, cleared), count 0, byte +0x10 cleared, final table; null when
/// the allocation fails.
fn program_map(e: &mut Engine, base_vtable: u32, vtable: u32) -> u32 {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        return 0;
    }
    e.mem.set_u32(map, base_vtable);
    e.mem.set_u32(map + 4, 0x3b);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOCATE_BLOCK, &args![0xecu32]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u8(map + 0x10, 0);
    e.mem.set_u32(map, vtable);
    map
}

// Translated from 00e93ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`): the shader program factory
/// constructor, `thiscall`, no stack words. The `NiRefObject` start, the factory's table, +8,
/// +0xc and +0x10 cleared, and its two maps at +0x14 and +0x18. Answers `this`.
pub fn fn_00e93ad0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, PROGRAM_FACTORY_VTABLE);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let first = program_map(e, FIRST_MAP_BASE_VTABLE, FIRST_MAP_VTABLE);
    e.mem.set_u32(base + 0x14, first);
    let second = program_map(e, SECOND_MAP_BASE_VTABLE, SECOND_MAP_VTABLE);
    e.mem.set_u32(base + 0x18, second);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_3d80,
            ni_d3d_shader_program_factory_create_shader_creator_map()
        ),
        entry!(0x00e9_3df0, fn_00e93df0()),
        entry!(0x00e9_3e70, ni_d3d_shader_program_factory_register_shader_creator(Ptr, u32) -> bool),
        entry!(
            0x00b7_1300,
            ni_t_pointer_list_base_remove_all(Ptr<crate::types::NiTPointerList>)
        ),
        entry!(
            0x00ae_7f90,
            ni_t_pointer_list_base_remove_head(Ptr<crate::types::NiTPointerList>) -> u32
        ),
        entry!(0x00e9_2ec0, fn_00e92ec0()),
        entry!(0x00e9_2f70, fn_00e92f70(u32)),
        entry!(0x00e9_3000, ni_d3d_shader_program_factory_get_next_program_directory(u32) -> u32),
        entry!(0x00e9_3260, fn_00e93260(u32)),
        entry!(0x00e9_33d0, fn_00e933d0()),
        entry!(0x00e9_3400, fn_00e93400(Ptr)),
        entry!(0x00e9_3470, fn_00e93470(Ptr, u32, u32, u32)),
        entry!(0x00e9_34d0, fn_00e934d0(Ptr)),
        entry!(0x00e9_3540, fn_00e93540(Ptr)),
        entry!(0x00e9_35b0, fn_00e935b0(Ptr, u32)),
        entry!(0x00e9_3650, fn_00e93650(Ptr, u32) -> Ptr),
        entry!(0x00e9_36f0, fn_00e936f0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3790, fn_00e93790(u32)),
        entry!(0x00e9_37c0, fn_00e937c0(u32)),
        entry!(0x00e9_3ab0, fn_00e93ab0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3ad0, fn_00e93ad0(Ptr) -> Ptr),
        entry!(
            0x00e9_3cc0,
            ni_d3d_shader_program_factory_remove_all_program_directories(Ptr)
        ),
        entry!(0x00e9_3e00, ni_d3d_shader_program_factory_destruct(Ptr)),
        entry!(0x00e9_3f30, ni_d3d_shader_program_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

/// The node pool's lock object (`this` of the lock enter `0040fbf0`, thiscall,
/// one word); its owner word.
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's entry count, which the inlined leave decrements.
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The lock enter (`0040fbf0`; thiscall, one word).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// Head of the node pool's free list.
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00b71300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::RemoveAll`
/// (engine map): returns every node of the list to the shared node pool:
/// for each node clears its element (+0x08), enters the pool's lock
/// (`0040fbf0(0)`), clears its previous link (+0x04), pushes it on the free
/// list at `011c5f58` and leaves the lock (the count at `011c5f84` drops by
/// one; at zero the owner word `011c5f80` is cleared). Then clears the
/// count, head and tail.
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr<crate::types::NiTPointerList>) {
    use crate::types::NiTPointerList;
    let mut next = e.get(this, NiTPointerList::m_pkHead);
    while next != 0 {
        let node = next;
        next = e.mem.u32(node);
        e.mem.set_u32(node + 8, 0);
        e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
        e.mem.set_u32(node + 4, 0);
        let free = e.mem.u32(NODE_POOL_FREE_LIST);
        e.mem.set_u32(node, free);
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        e.mem.set_u32(NODE_POOL_FREE_LIST, node);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

#[cfg(test)]
mod remove_all_tests {
    use super::*;

    #[test]
    fn remove_all_returns_each_node_to_the_pool_and_empties_the_list() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        // The lock enter double takes the lock: owner 1, count + 1.
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        // Already held once by the caller: the owner stays after each leave.
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0099_0000);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        let locks: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == LOCK_ENTER)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(locks, vec![vec![NODE_POOL_LOCK, 0]; 2]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first), 0x0099_0000);
        for node in [first, second] {
            assert_eq!(e.mem.u32(node + 4), 0);
            assert_eq!(e.mem.u32(node + 8), 0);
        }
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 1);
        assert_eq!(e.mem.bytes(list, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn remove_all_clears_the_owner_when_the_lock_count_reaches_zero() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        let list = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(list, node);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(list + 8, 1);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), node);
        // An empty list only clears its words.
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn creator_map_is_made_once_with_a_zeroed_slot_block() {
        let mut e = engine();
        e.call(0x00e9_3df0, &args![]);
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), 0x010f_0f50);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(e.mem.u8(map + 0x10), 1);
        let slots = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(slots, 0x94), vec![0; 0x94]);
        e.call(0x00e9_3df0, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), map);
    }

    #[test]
    fn failed_allocation_stores_null() {
        let mut e = engine();
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3d80, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), 0);
    }

    fn register(name: &[u8]) -> (bool, Vec<(Vec<u8>, u32)>) {
        let mut e = engine();
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|c| u32::from(*c) == a[1]) {
                Some(at) => (a[0] + at as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(TOLOWER, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        let stored: Rc<RefCell<Vec<(Vec<u8>, u32)>>> = Rc::default();
        let log = stored.clone();
        e.register_double(MAP_SET_AT, move |e, a| {
            log.borrow_mut().push((e.mem.cstr(a[1]), a[2]));
            Ret::default()
        });
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, name);
        let accepted = e.call(0x00e9_3e70, &args![text, 0x1234u32]).bool();
        let entries = stored.borrow().clone();
        (accepted, entries)
    }

    #[test]
    fn registers_the_lower_case_copy() {
        assert_eq!(
            register(b"NiD3DShader"),
            (true, vec![(b"nid3dshader".to_vec(), 0x1234)])
        );
    }

    #[test]
    fn refuses_names_with_a_dot() {
        assert_eq!(register(b"Shader.vsh"), (false, vec![]));
    }

    #[test]
    fn refuses_a_null_name() {
        let mut e = engine();
        assert!(!e.call(0x00e9_3e70, &args![0u32, 1u32]).bool());
    }
}

#[cfg(test)]
mod factory_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e
    }

    /// An object whose vtable has the test functions `0x7000 + slot`.
    fn object(e: &mut Engine) -> u32 {
        let slots: Vec<u32> = (0..0x18).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_0000);
        object
    }

    #[test]
    fn remove_head_unlinks_the_first_node() {
        let mut e = engine();
        stub(&mut e, &[NODE_DEALLOCATE]);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_7f90, &args![list]).u32(), 0x11);
        assert_eq!(e.mem.u32(list), second);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(calls_to(&e, NODE_DEALLOCATE), vec![vec![list + 8, first]]);
        assert_eq!(e.call(0x00ae_7f90, &args![list]).u32(), 0x22);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(list + 8), 0);
    }

    #[test]
    fn fn_00e92ec0_deletes_the_instance() {
        let mut e = engine();
        let instance = object(&mut e);
        stub(&mut e, &[0x0000_7000]);
        e.set_global(FACTORY_INSTANCE, instance);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2ec0, &args![]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![instance, 1]]);
        assert_eq!(e.global::<u32>(FACTORY_INSTANCE), 0);
        e.call(0x00e9_2ec0, &args![]);
        assert_eq!(calls_to(&e, 0x0000_7000).len(), 1);
    }

    #[test]
    fn fn_00e92f70_keeps_the_renderer_and_its_device() {
        let mut e = engine();
        let old = object(&mut e);
        let device = object(&mut e);
        stub(&mut e, &[0x0000_7004, 0x0000_7008]);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x55);
        e.set_global(FACTORY_DEVICE_0126FA18, old);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![renderer]);
        assert_eq!(e.global::<u32>(FACTORY_RENDERER_0126FA1C), renderer);
        assert_eq!(e.global::<u32>(FACTORY_DEVICE_0126FA18), device);
        assert_eq!(e.global::<u32>(FACTORY_LIBRARY_0126FA20), 0x55);
        assert_eq!(calls_to(&e, 0x0000_7008), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![device]]);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![0u32]);
        assert_eq!(e.global::<u32>(FACTORY_RENDERER_0126FA1C), 0);
        assert_eq!(e.global::<u32>(FACTORY_DEVICE_0126FA18), 0);
        assert_eq!(e.global::<u32>(FACTORY_LIBRARY_0126FA20), 0);
        assert_eq!(calls_to(&e, 0x0000_7008), vec![vec![device]]);
    }

    #[test]
    fn get_next_program_directory_steps_the_position() {
        let mut e = engine();
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node, 0x4444);
        e.mem.set_u32(node + 8, 0x5555);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, node);
        assert_eq!(e.call(0x00e9_3000, &args![position]).u32(), 0x5555);
        assert_eq!(e.mem.u32(position), 0x4444);
        e.mem.set_u32(position, 0);
        assert_eq!(e.call(0x00e9_3000, &args![position]).u32(), 0);
    }

    #[test]
    fn fn_00e93260_removes_the_creator_entries() {
        let mut e = engine();
        stub(&mut e, &[MAP_REMOVE_AT]);
        // Three entries in turn: (1, creator), (2, other), (3, creator).
        let mut step = 0u32;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            step += 1;
            e.mem.set_u32(a[2], step);
            e.mem.set_u32(a[3], if step == 2 { 0x99 } else { 0x77 });
            e.mem
                .set_u32(a[1], if step == 3 { 0 } else { 0x1000 + step });
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(buckets + 8, 0x1000);
        e.mem.set_u32(CREATOR_MAP_SLOT, map);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3260, &args![0x77u32]);
        assert_eq!(calls_to(&e, MAP_GET_NEXT).len(), 3);
        assert_eq!(
            calls_to(&e, MAP_REMOVE_AT),
            vec![vec![map, 1], vec![map, 3]]
        );
        // An empty map asks nothing.
        e.mem.set_u32(buckets + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3260, &args![0x77u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e933d0_destroys_the_creator_map() {
        let mut e = engine();
        let map = object(&mut e);
        stub(&mut e, &[MAP_REMOVE_ALL, 0x0000_7000]);
        e.mem.set_u32(CREATOR_MAP_SLOT, map);
        e.call_log = Some(vec![]);
        e.call(0x00e9_33d0, &args![]);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), 0);
    }

    fn string_map(e: &mut Engine, copies: bool) -> (u32, u32) {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x6001);
        e.mem.set_u32(second + 4, 0x6002);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, u8::from(copies));
        (map, buckets)
    }

    #[test]
    fn string_map_destructors_free_the_copied_keys() {
        for (address, deleting, vtable) in [
            (0x00e9_3400u32, 0x00e9_3650u32, FIRST_MAP_BASE_VTABLE),
            (0x00e9_34d0, 0x00e9_36f0, SECOND_MAP_BASE_VTABLE),
            (0x00e9_3540, 0x00e9_3ab0, CREATOR_MAP_BASE_VTABLE),
        ] {
            let mut e = engine();
            stub(&mut e, &[NI_FREE, MAP_REMOVE_ALL, MEMORY_MANAGER_FREE]);
            let (map, buckets) = string_map(&mut e, true);
            e.call_log = Some(vec![]);
            e.call(address, &args![map]);
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(
                calls_to(&e, NI_FREE),
                vec![vec![0x6001], vec![0x6002], vec![buckets]]
            );
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map], vec![map]]);
            let (map, buckets) = string_map(&mut e, false);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(deleting, &args![map, 1u32]).u32(), map);
            assert_eq!(calls_to(&e, NI_FREE), vec![vec![buckets]]);
            assert_eq!(
                calls_to(&e, MEMORY_MANAGER_FREE),
                vec![vec![MEMORY_MANAGER_OBJECT, map]]
            );
            e.call_log = Some(vec![]);
            e.call(deleting, &args![map, 0u32]);
            assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        }
    }

    #[test]
    fn set_and_clear_value_copy_keys_with_the_flag() {
        let mut e = engine();
        e.register(ALLOCATE_BLOCK, |_, _| eax(0x8800));
        stub(&mut e, &[STRCPY_S, NI_FREE]);
        let map = e.mem.alloc(0x14);
        let item = e.mem.alloc(0xc);
        let key = e.mem.alloc(8);
        e.mem.set_cstr(key, b"abc");
        e.call_log = Some(vec![]);
        e.call(0x00e9_3470, &args![map, item, key, 0x99u32]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (key, 0x99));
        e.call(0x00e9_35b0, &args![map, item]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
        e.mem.set_u8(map + 0x10, 1);
        e.call(0x00e9_3470, &args![map, item, key, 0x98u32]);
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK), vec![vec![4]]);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x8800, 4, key]]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (0x8800, 0x98));
        e.call(0x00e9_35b0, &args![map, item]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x8800]]);
    }

    #[test]
    fn shader_names_leave_their_maps() {
        let mut e = engine();
        let shader = object(&mut e);
        e.register(0x0000_7050, |_, _| eax(0x6100));
        stub(&mut e, &[MAP_REMOVE_AT]);
        let instance = e.mem.alloc(0x1c);
        e.mem.set_u32(instance + 0x14, 0x4100);
        e.mem.set_u32(instance + 0x18, 0x4200);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3790, &args![shader]);
        assert!(calls_to(&e, 0x0000_7050).is_empty());
        e.set_global(FACTORY_INSTANCE, instance);
        e.call(0x00e9_3790, &args![shader]);
        e.call(0x00e9_37c0, &args![shader]);
        e.call(0x00e9_37c0, &args![0u32]);
        assert_eq!(
            calls_to(&e, MAP_REMOVE_AT),
            vec![vec![0x4100, 0x6100], vec![0x4200, 0x6100]]
        );
        e.mem.set_u32(instance + 0x18, 0);
        e.call(0x00e9_37c0, &args![shader]);
        assert_eq!(calls_to(&e, MAP_REMOVE_AT).len(), 2);
    }

    #[test]
    fn constructor_makes_both_maps() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| eax(e.mem.alloc(a[1])));
        e.register(ALLOCATE_BLOCK, |_, _| eax(0x9900));
        stub(&mut e, &[MEMSET, IMPORT_INTERLOCKED_INCREMENT]);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_3ad0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROGRAM_FACTORY_VTABLE);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        let vertex = e.mem.u32(this + 0x14);
        let pixel = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(vertex), FIRST_MAP_VTABLE);
        assert_eq!(e.mem.u32(pixel), SECOND_MAP_VTABLE);
        assert_eq!(e.mem.u32(vertex + 4), 0x3b);
        assert_eq!(e.mem.u32(vertex + 8), 0x9900);
        assert_eq!(e.mem.u8(pixel + 0x10), 0);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x9900, 0, 0xec]; 2]);
        // Failed allocations leave null maps.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| eax(0));
        e.call(0x00e9_3ad0, &args![this]);
        assert_eq!((e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)), (0, 0));
    }

    #[test]
    fn remove_all_program_directories_frees_each_directory() {
        let mut e = engine();
        stub(
            &mut e,
            &[LIST_REMOVE_HEAD, LIST_REMOVE_TAIL, NI_FREE, LOCK_ENTER],
        );
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 2);
        let this = e.mem.alloc(0x1c);
        let nodes: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc)).collect();
        for (index, &node) in nodes.iter().enumerate() {
            e.mem
                .set_u32(node, nodes.get(index + 1).copied().unwrap_or(0));
            e.mem
                .set_u32(node + 4, if index == 0 { 0 } else { nodes[index - 1] });
            e.mem
                .set_u32(node + 8, if index == 2 { 0 } else { 0x6000 + index as u32 });
        }
        e.mem.set_u32(this + 8, nodes[0]);
        e.mem.set_u32(this + 0xc, nodes[3]);
        e.mem.set_u32(this + 0x10, 4);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3cc0, &args![this]);
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD), vec![vec![this + 8]]);
        // The second node is in the middle: unlinked here.
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![NODE_POOL_LOCK, 0]]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), nodes[1]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        assert_eq!(calls_to(&e, LIST_REMOVE_TAIL), vec![vec![this + 8]]);
        assert_eq!(
            calls_to(&e, NI_FREE),
            vec![vec![0x6000], vec![0x6001], vec![0x6003]]
        );
    }

    #[test]
    fn destructor_and_deleting_destructor() {
        let mut e = engine();
        let map = object(&mut e);
        stub(
            &mut e,
            &[
                MAP_REMOVE_ALL,
                0x0000_7000,
                INTERLOCKED_DECREMENT,
                NI_FREE_SIZED,
            ],
        );
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, map);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_3f30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, 0x1c]]);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3f30, &args![this, 0u32]);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00e9_3e00, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e92f70_swaps_the_held_device() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.put_vtable(0x6000, &[0, 0x6104, 0x6108]);
        e.register(0x6104, |_, _| Ret::default());
        e.register(0x6108, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x66);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![renderer]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), device);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0x66);
        e.call(0x00e9_2f70, &args![0u32]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), 0);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6104, vec![device])));
        assert!(log.contains(&(0x6108, vec![device])));
    }

    #[test]
    fn fn_00e93ad0_makes_the_two_maps() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00e9_3ad0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROGRAM_FACTORY_VTABLE);
        let first = e.mem.u32(this + 0x14);
        let second = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(first), FIRST_MAP_VTABLE);
        assert_eq!(e.mem.u32(second), SECOND_MAP_VTABLE);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3ad0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }
}
