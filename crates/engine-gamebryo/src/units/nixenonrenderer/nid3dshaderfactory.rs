//! `NiXenonRenderer/nid3dshaderfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::ALLOCATE;

/// `00aa0390` (`thiscall`, no stack word, no unit): the base constructor.
const BASE_CONSTRUCTOR: u32 = 0x00aa_0390;
/// `00fdf054`: import slot of `InitializeCriticalSection` (`stdcall`, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00a29680` (`cdecl`, one word here): the empty call the exe makes with a string.
const EMPTY_CALL: u32 = 0x00a2_9680;
/// `0126fa0c`: the shader factory object `fn_00e92e60` builds.
const FACTORY_SLOT: u32 = 0x0126_fa0c;
/// `011f6044`: a second pointer to the same object.
const FACTORY_ALIAS_SLOT: u32 = 0x011f_6044;

/// Builds a hash-map-like object of `size` pointer slots in 0x14 bytes (the vtables
/// `first` then `second`, the size at `+4`, a zeroed slot block of `block_bytes` at `+8`,
/// the word at `+0xc` 0, the byte at `+0x10` `flag`); answers it, or 0 when the allocation
/// fails.
fn make_slot_table(
    e: &mut Engine,
    first: u32,
    second: u32,
    size: u32,
    block_bytes: u32,
    flag: u8,
) -> u32 {
    let table = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if table == 0 {
        return 0;
    }
    e.mem.set_u32(table, first);
    e.mem.set_u32(table + 4, size);
    e.mem.set_u32(table + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![block_bytes]).u32();
    let bytes = e.mem.u32(table + 4).wrapping_mul(4);
    e.mem.set_u32(table + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(table + 0x10, flag);
    e.mem.set_u32(table, second);
    table
}

// Translated from 00e92cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderfactory.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00aa0390`, sets the vtable
/// `010f0cc4`, clears the word at `+0x24`, initializes the critical section at `+0x80`,
/// then makes two 0x14-byte tables kept at `+0x28` (vtables `010f0bc0` then `010f0c80`,
/// 0x3b slots from a 0xec-byte block, flag byte 1) and `+0x30` (vtables `010f0c20` then
/// `010f0ca0`, 0x25 slots from a 0x94-byte block, flag byte 0); a table whose allocation
/// fails is stored as null. Returns `this`.
pub fn fn_00e92cf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, 0x010f_0cc4);
    e.mem.set_u32(base + 0x24, 0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![base + 0x80]);
    let first = make_slot_table(e, 0x010f_0bc0, 0x010f_0c80, 0x3b, 0xec, 1);
    e.mem.set_u32(base + 0x28, first);
    let second = make_slot_table(e, 0x010f_0c20, 0x010f_0ca0, 0x25, 0x94, 0);
    e.mem.set_u32(base + 0x30, second);
    this
}

// Translated from 00e92e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::_SDMInit` (Xbox PDB; run by the start-up `00e91dc0`, no arguments):
/// while `0126fa0c` is null, allocates 0x100 bytes (`00aa13e0`), constructs the factory
/// with `fn_00e92cf0` (null when the allocation fails) and keeps it in `0126fa0c` and
/// `011f6044`; when `011f6044` is null it calls `00a29680` with the string at `010f0d40`.
pub fn ni_d3d_shader_factory_sdm_init(e: &mut Engine) {
    if e.mem.u32(FACTORY_SLOT) == 0 {
        let block = e.call(ALLOCATE, &args![0x100u32]).u32();
        let factory = if block != 0 {
            fn_00e92cf0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(FACTORY_SLOT, factory);
        e.mem.set_u32(FACTORY_ALIAS_SLOT, factory);
    }
    if e.mem.u32(FACTORY_ALIAS_SLOT) == 0 {
        e.call(EMPTY_CALL, &args![0x010f_0d40u32]);
    }
}

/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The recursive lock of the shared item free list (`011c5f80`: owner, count at +4), taken
/// by `0040fbf0` (`thiscall`, one word the function does not read) and released inline.
const FREE_LIST_LOCK_011C5F80: u32 = 0x011c_5f80;
const LOCK_ACQUIRE_0040FBF0: u32 = 0x0040_fbf0;
/// The head of the free list (`011c5f58`).
const FREE_LIST_HEAD_011C5F58: u32 = 0x011c_5f58;

// Translated from 00e6e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *,NiPointer<NiShaderLibrary> >::DeleteItem` (engine map, from the
/// vtable; the body fits), `thiscall` with one stack word, the item (next +0, key +4, value
/// +8); `this` is not read. Releases the value's reference (`InterlockedDecrement` on its
/// count at +4, deleted through its vtable slot +4 when that answers 0) and clears it; then,
/// under the free-list lock `011c5f80` (`0040fbf0` with 0), clears the key, links the item in
/// front of the free list at `011c5f58` and releases the lock inline (the count at
/// `011c5f84` decremented, the owner cleared when it reaches 0).
pub fn ni_t_pointer_map_delete_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    let item = item.addr();
    let value = e.mem.u32(item + 8);
    if value != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![value + 4])
            .u32();
        if left == 0 {
            e.vcall(value, 4, &args![]);
        }
        e.mem.set_u32(item + 8, 0);
    }
    e.call(LOCK_ACQUIRE_0040FBF0, &args![FREE_LIST_LOCK_011C5F80, 0u32]);
    e.mem.set_u32(item + 4, 0);
    let head = e.global::<u32>(FREE_LIST_HEAD_011C5F58);
    e.mem.set_u32(item, head);
    let count = e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4).wrapping_sub(1);
    e.set_global::<u32>(FREE_LIST_LOCK_011C5F80 + 4, count);
    e.set_global::<u32>(FREE_LIST_HEAD_011C5F58, item);
    if count == 0 {
        e.set_global::<u32>(FREE_LIST_LOCK_011C5F80, 0);
    }
}

// Translated from 00e91f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`), no arguments: the shader factory
/// word at `0126fa0c`.
pub fn fn_00e91f10(e: &mut Engine) -> u32 {
    e.mem.u32(FACTORY_SLOT)
}

use crate::types::NiTPointerMap;

/// `vsprintf_s(buffer, size, format, va_list)` (LIBCMT, `cdecl`).
const VSPRINTF_S: u32 = 0x00ec_6bb5;

// Translated from 00a305b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSprintf` (engine map), `cdecl` with a variable argument list: with a size of 0 answers 1
/// and writes nothing; else hands the variable arguments, as a `va_list` that points at them,
/// to `vsprintf_s(buffer, size, format, ...)`, writes 0 to the buffer's last byte
/// (`buffer[size - 1]`) and answers `vsprintf_s`'s result. `variable_arguments` are the words
/// the caller pushed after `format`, in order; they are copied to memory because the `va_list`
/// is the address of the first one.
pub fn ni_sprintf(
    e: &mut Engine,
    buffer: Ptr,
    size: u32,
    format: Ptr,
    variable_arguments: &[u32],
) -> i32 {
    if size == 0 {
        return 1;
    }
    let list = e.mem.alloc(4 * variable_arguments.len().max(1) as u32);
    for (i, word) in variable_arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = e.call(VSPRINTF_S, &args![buffer, size, format, list]).i32();
    e.mem.free(list);
    e.mem
        .set_u8(buffer.addr().wrapping_add(size).wrapping_sub(1), 0);
    result
}

/// The uniform form of [`ni_sprintf`]: it takes any number of argument words, so it cannot go
/// through `entry!`. The first three words are the buffer, its size and the format; the rest
/// are the variable arguments.
fn ni_sprintf_uniform(e: &mut Engine, a: &[u32]) -> Ret {
    ni_sprintf(e, Ptr::new(a[0]), a[1], Ptr::new(a[2]), &a[3..]).into_ret()
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// An `NiPointer` let go: the object's count (+4) down through `InterlockedDecrement` and, at
/// zero, its virtual slot 4 (delete).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a4d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,char const *,NiPointer<NiShaderLibrary> >
/// ::SetValue` (engine map; a virtual slot), `thiscall` with three stack words, `this` not
/// read: the item, the key and the value (an `NiPointer` passed by value, which the function
/// lets go at its end). The key goes to the item's +4; the item's value (+8) gets an
/// `NiPointer` assignment (unless it holds the value already: the old one let go, the new one
/// stored and, when not null, counted up; a null value ends the function there). Then the
/// by-value pointer is let go (count down, deleted at zero).
pub fn ni_t_map_base_set_value(e: &mut Engine, _this: Ptr, item: Ptr, key: u32, value: u32) {
    let item = item.addr();
    e.mem.set_u32(item.wrapping_add(4), key);
    let held = e.mem.u32(item.wrapping_add(8));
    if held != value {
        release_reference(e, held);
        e.mem.set_u32(item.wrapping_add(8), value);
        if value == 0 {
            return;
        }
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
    }
    release_reference(e, value);
}

// Translated from 00b4f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::DynamicCast` (engine map), `thiscall` with an RTTI record: walks the object's
/// RTTI (virtual slot 8) and its bases (+4); answers `this` when the record is among them,
/// else 0.
pub fn ni_shader_dynamic_cast(e: &mut Engine, this: Ptr, rtti: u32) -> u32 {
    let mut current = e.vcall(this.addr(), 8, &args![]).u32();
    while current != 0 {
        if current == rtti {
            return this.addr();
        }
        current = e.mem.u32(current.wrapping_add(4));
    }
    0
}

// Translated from 00e91e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::SetRenderer` (engine map), `thiscall` with the renderer. Unless it is
/// the one at +0x24 already, stores it and hands it to every shader (virtual slot 0x1c of
/// each), walking them through the factory's virtual slots 0x38 (first) and 0x3c (next).
pub fn ni_d3d_shader_factory_set_renderer(e: &mut Engine, this: Ptr, renderer: u32) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x24)) == renderer {
        return;
    }
    e.mem.set_u32(base.wrapping_add(0x24), renderer);
    let mut shader = e.vcall(base, 0x38, &args![]).u32();
    while shader != 0 {
        e.vcall(shader, 0x1c, &args![renderer]);
        shader = e.vcall(base, 0x3c, &args![]).u32();
    }
}

// Translated from 00e92080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`; no arguments): deletes the shader
/// factory at `0126fa0c` (its slot 0 with 1) when there is one and clears the word.
pub fn fn_00e92080(e: &mut Engine) {
    let factory = e.mem.u32(FACTORY_SLOT);
    if factory != 0 {
        e.vcall(factory, 0, &args![1u32]);
        e.mem.set_u32(FACTORY_SLOT, 0);
    }
}

/// The key format `"%s%d"` (`01094c50`) and the shader map's lookup `00a53c30` (unnamed;
/// `thiscall`: the key, the address of the word it fills; answers a byte).
const SHADER_KEY_FORMAT_01094C50: u32 = 0x0109_4c50;
const MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;

// Translated from 00e921d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::FindShader` (engine map), `thiscall` with a name and a number:
/// without the shader map (+0x28) answers 0. Else the key `"%s%d"` of the two is written to a
/// 0x104-byte stack buffer ([`ni_sprintf`]) and looked up in the map (`00a53c30`, filling a
/// stack word cleared first); answers the word when the lookup answers true, else 0.
// Translation incomplete: 00e921d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_d3d_shader_factory_find_shader(e: &mut Engine, this: Ptr, name: u32, number: u32) -> u32 {
    let map = e.mem.u32(this.addr().wrapping_add(0x28));
    if map == 0 {
        return 0;
    }
    e.with_stack(0x108, |e, frame| {
        let found = frame.addr();
        let buffer = found + 4;
        e.mem.set_u32(found, 0);
        ni_sprintf(
            e,
            Ptr::new(buffer),
            0x104,
            Ptr::new(SHADER_KEY_FORMAT_01094C50),
            &[name, number],
        );
        let map = e.mem.u32(this.addr().wrapping_add(0x28));
        let hit = e.call(MAP_LOOKUP_00A53C30, &args![map, buffer, found]).u8();
        if hit != 0 {
            e.mem.u32(found)
        } else {
            0
        }
    })
}

/// `00b601a0` (unnamed; `thiscall` on a map, no stack words): empties the map.
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;

// Translated from 00e92460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveAllShaders` (engine map), `thiscall`: empties the shader map
/// (+0x28, `00b601a0`, a tail jump) when there is one.
pub fn ni_d3d_shader_factory_remove_all_shaders(e: &mut Engine, this: Ptr) {
    let map = e.mem.u32(this.addr().wrapping_add(0x28));
    if map != 0 {
        e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
    }
}

/// The free `00aa10f0` (`cdecl`, a block) and the tables of the two string maps' destructors
/// (RTTI `NiTStringTemplateMap<NiTPointerMap<char const *, NiShader *>, NiShader *>`, its
/// `NiTPointerMap` and `NiTMapBase`; the same three for `NiPointer<NiShaderLibrary>`).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const SHADER_STRING_MAP_VTABLES: [u32; 3] = [0x010f_0c00, 0x010f_0be0, 0x010f_0bc0];
const LIBRARY_STRING_MAP_VTABLES: [u32; 3] = [0x010f_0c60, 0x010f_0c40, 0x010f_0c20];

/// The shared body of the two `NiTStringTemplateMap` destructors: the first table; when the
/// map copies its keys (the byte +0x10), every item's key (+4) in every bucket is freed
/// (`00aa10f0`, the next item read first); then the second table and `00b601a0`, the third
/// table and `00b601a0` again, and the bucket array (+8) freed.
fn string_map_down(e: &mut Engine, this: Ptr<NiTPointerMap>, tables: [u32; 3]) {
    let base = this.addr();
    e.mem.set_u32(base, tables[0]);
    if e.mem.u8(base.wrapping_add(0x10)) != 0 {
        let mut bucket = 0u32;
        while bucket < e.get(this, NiTPointerMap::m_uiHashSize) {
            let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let key = e.mem.u32(item.wrapping_add(4));
                item = e.mem.u32(item);
                e.call(FREE_00AA10F0, &args![key]);
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(base, tables[1]);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    e.mem.set_u32(base, tables[2]);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_00AA10F0, &args![buckets]);
}

// Translated from 00e92520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the
/// `NiTStringTemplateMap<NiTPointerMap<char const *, NiShader *>, NiShader *>` destructor (the
/// RTTI of the table it stores), `thiscall`: with key copies (+0x10) every key is freed, then
/// the `NiTPointerMap` and `NiTMapBase` parts empty the map (`00b601a0`) and the bucket array
/// is freed (`00aa10f0`).
pub fn fn_00e92520(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_down(e, this, SHADER_STRING_MAP_VTABLES);
}

// Translated from 00e92590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`; a virtual slot of several maps,
/// `thiscall` with two words, `this` not read): whether the two keys are the same word.
pub fn fn_00e92590(_e: &mut Engine, _this: Ptr, first: u32, second: u32) -> bool {
    first == second
}

// Translated from 00e925a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the
/// `NiTStringTemplateMap<NiTPointerMap<char const *, NiPointer<NiShaderLibrary>>, ...>`
/// destructor (the RTTI of the table it stores), `thiscall`: as [`fn_00e92520`] with its own
/// tables.
pub fn fn_00e925a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_down(e, this, LIBRARY_STRING_MAP_VTABLES);
}

/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, the block) and the
/// sized free `00aa1460` (`cdecl`: block, size).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e92690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *,NiShader *>,NiShader *>::scalar deleting
/// destructor` (engine map), `thiscall` with the flags: the destructor ([`fn_00e92520`]), then
/// with bit 0 the block given back to the memory manager (`00aa4060`). Answers `this`.
pub fn ni_t_string_template_map_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00e92520(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e92760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): an `NiTPointerMap` lookup with an
/// `NiPointer` result, `thiscall` with the key and the address of the result. The bucket is the
/// map's hash of the key (virtual slot 4); its items are compared through the map's virtual
/// slot 8 (the key, the item's key at +4). Not found: answers false. Found: the result gets an
/// `NiPointer` assignment of the item's value (+8) (unless it holds it already: the old one
/// let go, the new one counted up when not null) and true is answered.
pub fn fn_00e92760(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, result: Ptr) -> bool {
    let base = this.addr();
    let index = e.vcall(base, 4, &args![key]).u32();
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
    loop {
        if item == 0 {
            return false;
        }
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(base, 8, &args![key, item_key]).u8() != 0 {
            break;
        }
        item = e.mem.u32(item);
    }
    let held = e.mem.u32(result.addr());
    if held != e.mem.u32(item.wrapping_add(8)) {
        release_reference(e, held);
        let value = e.mem.u32(item.wrapping_add(8));
        e.mem.set_u32(result.addr(), value);
        if value != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
        }
    }
    true
}

/// The library map's step to the next item (`00e6e630`, unnamed; `thiscall` on the map with
/// three words: the address of the iterator, of the key and of the value), the library's name
/// (`NiShaderLibrary::GetName`, `00d47a30`, `thiscall`) and the removal of a library by name
/// (`00e91fd0`, unnamed; `thiscall` on the library map, the name).
const MAP_STEP_00E6E630: u32 = 0x00e6_e630;
const LIBRARY_GET_NAME_00D47A30: u32 = 0x00d4_7a30;
const REMOVE_LIBRARY_00E91FD0: u32 = 0x00e9_1fd0;

/// The library map's next value from the iterator at `iterator` (`00e6e630` with a stack key
/// word and a stack `NiPointer` cleared first); the `NiPointer` is let go before the value is
/// answered.
fn next_library(e: &mut Engine, map: u32, iterator: u32) -> u32 {
    e.with_stack(8, |e, frame| {
        let value = frame.addr();
        let key = value + 4;
        e.mem.set_u32(value, 0);
        e.call(MAP_STEP_00E6E630, &args![map, iterator, key, value]);
        let library = e.mem.u32(value);
        release_reference(e, library);
        library
    })
}

// Translated from 00e92b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetFirstLibrary` (engine map), `thiscall`: without the library map
/// (+0x30) answers 0. Else the iterator (+0x2c) is set to the first item of the first non-empty
/// bucket (the bucket count read once), or null; with one, the next value is taken from it
/// (`00e6e630`, which also moves the iterator on) into a stack `NiPointer`, let go, and
/// answered; else 0.
pub fn ni_d3d_shader_factory_get_first_library(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let map = e.mem.u32(base.wrapping_add(0x30));
    if map == 0 {
        return 0;
    }
    let size = e.mem.u32(map.wrapping_add(4));
    let mut first = 0;
    if size != 0 {
        let buckets = e.mem.u32(map.wrapping_add(8));
        for bucket in 0..size {
            let item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            if item != 0 {
                first = item;
                break;
            }
        }
    }
    e.mem.set_u32(base.wrapping_add(0x2c), first);
    if first == 0 {
        return 0;
    }
    next_library(e, map, base.wrapping_add(0x2c))
}

// Translated from 00e92c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveAllLibraries` (engine map), `thiscall`: without the library map
/// (+0x30) does nothing. Else, from the first library ([`ni_d3d_shader_factory_get_first_library`]),
/// each library is removed by its name (`GetName`, then `00e91fd0` on the map) and the next one
/// taken from the iterator (+0x2c) while the map and the iterator are set; then the map is
/// emptied (`00b601a0` on the word at +0x30, read again).
pub fn ni_d3d_shader_factory_remove_all_libraries(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x30)) == 0 {
        return;
    }
    let mut library = ni_d3d_shader_factory_get_first_library(e, this);
    while library != 0 {
        let name = e.call(LIBRARY_GET_NAME_00D47A30, &args![library]).u32();
        let map = e.mem.u32(base.wrapping_add(0x30));
        e.call(REMOVE_LIBRARY_00E91FD0, &args![map, name]);
        let map = e.mem.u32(base.wrapping_add(0x30));
        if map == 0 || e.mem.u32(base.wrapping_add(0x2c)) == 0 {
            break;
        }
        library = next_library(e, map, base.wrapping_add(0x2c));
    }
    let map = e.mem.u32(base.wrapping_add(0x30));
    e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
}

// Translated from 00e92cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *,NiPointer<NiShaderLibrary> >,
/// NiPointer<NiShaderLibrary> >::scalar deleting destructor` (engine map), `thiscall` with the
/// flags: the destructor ([`fn_00e925a0`]), then with bit 0 the block given back to the memory
/// manager (`00aa4060`). Answers `this`.
pub fn ni_t_string_template_map_library_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00e925a0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

/// The imported `DeleteCriticalSection` (import slot) and `NiShaderFactory::~NiShaderFactory`
/// (`00aa05a0`, `thiscall`).
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const NI_SHADER_FACTORY_DOWN_00AA05A0: u32 = 0x00aa_05a0;

// Translated from 00e92dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the `NiD3DShaderFactory` destructor
/// (the RTTI of the table it stores), `thiscall`. Its table; the shader map (+0x28) emptied
/// (`00b601a0`) when there is one; all libraries removed
/// ([`ni_d3d_shader_factory_remove_all_libraries`]); both maps (+0x28, +0x30) deleted (their
/// slot 0 with 1) when set; +0x28, +0x30 and the renderer (+0x24) cleared; the critical
/// section at +0x80 deleted; then `NiShaderFactory::~NiShaderFactory` (a tail jump).
pub fn fn_00e92dc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010f_0cc4);
    let shaders = e.mem.u32(base.wrapping_add(0x28));
    if shaders != 0 {
        e.call(MAP_REMOVE_ALL_00B601A0, &args![shaders]);
    }
    ni_d3d_shader_factory_remove_all_libraries(e, this);
    for offset in [0x28u32, 0x30] {
        let map = e.mem.u32(base.wrapping_add(offset));
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
    }
    e.mem.set_u32(base.wrapping_add(0x28), 0);
    e.mem.set_u32(base.wrapping_add(0x30), 0);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    e.call(
        IMPORT_DELETE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.call(NI_SHADER_FACTORY_DOWN_00AA05A0, &args![base]);
}

// Translated from 00e92e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::scalar deleting destructor` (engine map), `thiscall` with the flags:
/// the destructor ([`fn_00e92dc0`]), then with bit 0 the 0x100 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_d3d_shader_factory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e92dc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x100u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_1f10, fn_00e91f10() -> u32),
        entry!(0x00e6_e830, ni_t_pointer_map_delete_item(Ptr, Ptr)),
        entry!(0x00e9_2cf0, fn_00e92cf0(Ptr) -> Ptr),
        entry!(0x00e9_2e60, ni_d3d_shader_factory_sdm_init()),
        (0x00a3_05b0, ni_sprintf_uniform as AbiFn),
        entry!(0x00a4_d5f0, ni_t_map_base_set_value(Ptr, Ptr, u32, u32)),
        entry!(0x00b4_f580, ni_shader_dynamic_cast(Ptr, u32) -> u32),
        entry!(0x00e9_1e60, ni_d3d_shader_factory_set_renderer(Ptr, u32)),
        entry!(0x00e9_2080, fn_00e92080()),
        entry!(0x00e9_21d0, ni_d3d_shader_factory_find_shader(Ptr, u32, u32) -> u32),
        entry!(0x00e9_2460, ni_d3d_shader_factory_remove_all_shaders(Ptr)),
        entry!(0x00e9_2520, fn_00e92520(Ptr<NiTPointerMap>)),
        entry!(0x00e9_2590, fn_00e92590(Ptr, u32, u32) -> bool),
        entry!(0x00e9_25a0, fn_00e925a0(Ptr<NiTPointerMap>)),
        entry!(
            0x00e9_2690,
            ni_t_string_template_map_shader_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e9_2760,
            fn_00e92760(Ptr<NiTPointerMap>, u32, Ptr) -> bool
        ),
        entry!(0x00e9_2b50, ni_d3d_shader_factory_get_first_library(Ptr) -> u32),
        entry!(0x00e9_2c40, ni_d3d_shader_factory_remove_all_libraries(Ptr)),
        entry!(
            0x00e9_2cd0,
            ni_t_string_template_map_library_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00e9_2dc0, fn_00e92dc0(Ptr)),
        entry!(0x00e9_2e30, ni_d3d_shader_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x2000);
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| 1u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(EMPTY_CALL, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_builds_two_tables() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_2cf0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![this + 0x80]]
        );
        assert_eq!(e.mem.u32(this), 0x010f_0cc4);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        let first = e.mem.u32(this + 0x28);
        assert_eq!(e.mem.u32(first), 0x010f_0c80);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        assert_eq!(e.mem.u32(first + 0xc), 0);
        assert_eq!(e.mem.u8(first + 0x10), 1);
        let slots = e.mem.u32(first + 8);
        assert_eq!(e.mem.bytes(slots, 0xec), vec![0; 0xec]);
        let second = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(second), 0x010f_0ca0);
        assert_eq!(e.mem.u32(second + 4), 0x25);
        assert_eq!(e.mem.u8(second + 0x10), 0);
        let allocations = calls_to(&e, ALLOCATE_BLOCK);
        assert_eq!(allocations, vec![vec![0xec], vec![0x94]]);
    }

    #[test]
    fn constructor_stores_null_for_a_failed_table() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.call(0x00e9_2cf0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn start_up_makes_the_factory_once() {
        let mut e = engine();
        e.call(0x00e9_2e60, &args![]);
        let factory = e.mem.u32(FACTORY_SLOT);
        assert_ne!(factory, 0);
        assert_eq!(e.mem.u32(FACTORY_ALIAS_SLOT), factory);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), factory);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(calls_to(&e, EMPTY_CALL).is_empty());
    }

    #[test]
    fn start_up_with_a_failed_allocation_calls_the_empty_function() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), 0);
        assert_eq!(calls_to(&e, EMPTY_CALL), vec![vec![0x010f_0d40]]);
    }
}

#[cfg(test)]
mod delete_item_tests {
    use super::*;

    const VALUE_DELETE: u32 = 0x0e20_0000;

    fn engine(left: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register_double(IMPORT_INTERLOCKED_DECREMENT, move |_, _| left.into_ret());
        e.register(LOCK_ACQUIRE_0040FBF0, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            let count = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, count);
            Ret::default()
        });
        e.register(VALUE_DELETE, |_, _| Ret::default());
        e.put_vtable(0x0e10_0000, &[0, VALUE_DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn delete_item_releases_the_value_and_frees_the_item() {
        let mut e = engine(0);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x0e10_0000);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, 0x1234);
        e.mem.set_u32(item + 4, 0x5678);
        e.mem.set_u32(item + 8, value);
        e.set_global::<u32>(FREE_LIST_HEAD_011C5F58, 0x4444);
        e.call(0x00e6_e830, &args![0x9999u32, item]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![value + 4]]
        );
        assert_eq!(calls(&e, VALUE_DELETE), vec![vec![value]]);
        assert_eq!(
            calls(&e, LOCK_ACQUIRE_0040FBF0),
            vec![vec![FREE_LIST_LOCK_011C5F80, 0]]
        );
        assert_eq!(e.mem.u32(item), 0x4444);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_HEAD_011C5F58), item);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80), 0);
    }

    #[test]
    fn delete_item_keeps_a_referenced_value_and_a_held_lock() {
        let mut e = engine(1);
        e.set_global::<u32>(FREE_LIST_LOCK_011C5F80 + 4, 1);
        let value = e.mem.alloc(8);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, value);
        e.call(0x00e6_e830, &args![0u32, item]);
        assert!(calls(&e, VALUE_DELETE).is_empty());
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4), 1);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80), 0x77);
        // Without a value nothing is released.
        let mut e = engine(0);
        let item = e.mem.alloc(0xc);
        e.call(0x00e6_e830, &args![0u32, item]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.global::<u32>(FREE_LIST_HEAD_011C5F58), item);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e91f10_reads_the_factory_word() {
        let mut e = Engine::new();
        e.map(FACTORY_SLOT, 4);
        e.mem.set_u32(FACTORY_SLOT, 0x4321);
        assert_eq!(e.call(0x00e9_1f10, &args![]).u32(), 0x4321);
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const OBJECT_VTABLE: u32 = 0x6000;
    const DELETE: u32 = 0x7004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for no_op in [
            FREE_00AA10F0,
            FREE_00AA1460,
            MEMORY_MANAGER_FREE,
            MAP_REMOVE_ALL_00B601A0,
            REMOVE_LIBRARY_00E91FD0,
            IMPORT_DELETE_CRITICAL_SECTION,
            NI_SHADER_FACTORY_DOWN_00AA05A0,
            DELETE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        // The interlocked doubles move the count they are given and answer it.
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn ni_sprintf_passes_the_arguments_and_ends_the_buffer() {
        let mut e = engine();
        e.register(VSPRINTF_S, |e, a| {
            let first = e.mem.u32(a[3]);
            let second = e.mem.u32(a[3] + 4);
            e.mem.write(a[0], &[b'x'; 8]);
            (first + second).into_ret()
        });
        let buffer = e.mem.alloc(8);
        assert_eq!(e.call(0x00a3_05b0, &args![buffer, 0u32, 0x99u32]).i32(), 1);
        assert!(calls(&e, VSPRINTF_S).is_empty());
        let result = e
            .call(0x00a3_05b0, &args![buffer, 8u32, 0x99u32, 3u32, 4u32])
            .i32();
        assert_eq!(result, 7);
        let call = &calls(&e, VSPRINTF_S)[0];
        assert_eq!(&call[..3], &[buffer, 8, 0x99]);
        assert_eq!(e.mem.u8(buffer + 6), b'x');
        assert_eq!(e.mem.u8(buffer + 7), 0);
    }

    #[test]
    fn ni_t_map_base_set_value_assigns_and_lets_go_of_the_argument() {
        let mut e = engine();
        let item = e.mem.alloc(0xc);
        let old = object(&mut e, 1);
        let new = object(&mut e, 1);
        e.mem.set_u32(item + 8, old);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x55u32, new]);
        assert_eq!(e.mem.u32(item + 4), 0x55);
        assert_eq!(e.mem.u32(item + 8), new);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        // Counted up by the item, down by the argument.
        assert_eq!(e.mem.u32(new + 4), 1);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x56u32, new]);
        assert_eq!(e.mem.u32(new + 4), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![old], vec![new]]);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x57u32, 0u32]);
        assert_eq!(e.mem.u32(item + 8), 0);
    }

    #[test]
    fn ni_shader_dynamic_cast_walks_the_bases() {
        let mut e = engine();
        let records = e.mem.alloc(0x18);
        e.mem.set_u32(records + 4, records + 8);
        e.mem.set_u32(records + 0xc, 0);
        e.register_double(0x7008, move |_, _| records.into_ret());
        let shader = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0, 0, 0x7008]);
        e.mem.set_u32(shader, 0x6100);
        assert_eq!(
            e.call(0x00b4_f580, &args![shader, records + 8]).u32(),
            shader
        );
        assert_eq!(e.call(0x00b4_f580, &args![shader, records]).u32(), shader);
        assert_eq!(e.call(0x00b4_f580, &args![shader, 0x1234u32]).u32(), 0);
    }

    #[test]
    fn set_renderer_hands_the_renderer_to_every_shader() {
        let mut e = engine();
        let shaders = e.mem.alloc(0x10);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0, 0, 0, 0, 0, 0, 0, 0x701c]);
        e.mem.set_u32(first, 0x6100);
        e.mem.set_u32(second, 0x6100);
        e.mem.set_u32(shaders, first);
        e.mem.set_u32(shaders + 4, second);
        e.register(0x701c, |_, _| Ret::default());
        e.register_double(0x7038, move |e, _| {
            e.mem.set_u32(shaders + 0xc, 1);
            e.mem.u32(shaders).into_ret()
        });
        e.register_double(0x703c, move |e, _| {
            let at = e.mem.u32(shaders + 0xc);
            e.mem.set_u32(shaders + 0xc, at + 1);
            e.mem.u32(shaders + 4 * at).into_ret()
        });
        let mut slots = vec![0u32; 0x10];
        slots[0xe] = 0x7038;
        slots[0xf] = 0x703c;
        e.put_vtable(0x6200, &slots);
        let factory = e.mem.alloc(0x28);
        e.mem.set_u32(factory, 0x6200);
        e.call(0x00e9_1e60, &args![factory, 0x99u32]);
        assert_eq!(e.mem.u32(factory + 0x24), 0x99);
        assert_eq!(
            calls(&e, 0x701c),
            vec![vec![first, 0x99], vec![second, 0x99]]
        );
        e.call(0x00e9_1e60, &args![factory, 0x99u32]);
        assert_eq!(calls(&e, 0x7038).len(), 1);
    }

    #[test]
    fn fn_00e92080_deletes_the_factory() {
        let mut e = engine();
        e.call(0x00e9_2080, &args![]);
        e.register(0x7000, |_, _| Ret::default());
        let factory = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0x7000]);
        e.mem.set_u32(factory, 0x6100);
        e.mem.set_u32(FACTORY_SLOT, factory);
        e.call(0x00e9_2080, &args![]);
        assert_eq!(calls(&e, 0x7000), vec![vec![factory, 1]]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), 0);
    }

    #[test]
    fn find_shader_looks_the_key_up() {
        let mut e = engine();
        let hit = e.mem.alloc(4);
        e.register(VSPRINTF_S, |_, _| 0u32.into_ret());
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            e.mem.set_u32(a[2], 0x4321);
            e.mem.u32(hit).into_ret()
        });
        let factory = e.mem.alloc(0x34);
        assert_eq!(e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(), 0);
        assert!(calls(&e, VSPRINTF_S).is_empty());
        e.mem.set_u32(factory + 0x28, 0x5000);
        assert_eq!(e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(), 0);
        e.mem.set_u32(hit, 0x101);
        assert_eq!(
            e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(),
            0x4321
        );
        let format = &calls(&e, VSPRINTF_S)[0];
        assert_eq!(format[1..3], [0x104, SHADER_KEY_FORMAT_01094C50]);
        let lookup = &calls(&e, MAP_LOOKUP_00A53C30)[0];
        assert_eq!(lookup[0], 0x5000);
        assert_eq!(lookup[1], format[0]);
        assert_eq!(lookup[2] + 4, format[0]);
    }

    #[test]
    fn remove_all_shaders_empties_a_present_map() {
        let mut e = engine();
        let factory = e.mem.alloc(0x34);
        e.call(0x00e9_2460, &args![factory]);
        assert!(calls(&e, MAP_REMOVE_ALL_00B601A0).is_empty());
        e.mem.set_u32(factory + 0x28, 0x5000);
        e.call(0x00e9_2460, &args![factory]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![0x5000]]);
    }

    /// A string map of three buckets: items with keys 0x11, 0x12 in bucket 0 and 0x13 in 2.
    fn string_map(e: &mut Engine, copies: u8) -> u32 {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(item + 4, 0x11 + i as u32);
        }
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 8, items[2]);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, copies);
        map
    }

    #[test]
    fn string_map_destructors_free_copied_keys() {
        let mut e = engine();
        let map = string_map(&mut e, 1);
        let buckets = e.mem.u32(map + 8);
        e.call(0x00e9_2520, &args![map]);
        assert_eq!(
            calls(&e, FREE_00AA10F0),
            vec![vec![0x11], vec![0x12], vec![0x13], vec![buckets]]
        );
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL_00B601A0),
            vec![vec![map], vec![map]]
        );
        assert_eq!(e.mem.u32(map), 0x010f_0bc0);
        e.call_log = Some(vec![]);
        let map = string_map(&mut e, 0);
        let buckets = e.mem.u32(map + 8);
        e.call(0x00e9_25a0, &args![map]);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![buckets]]);
        assert_eq!(e.mem.u32(map), 0x010f_0c20);
    }

    #[test]
    fn string_map_deleting_destructors_free_with_bit_0() {
        let mut e = engine();
        for addr in [0x00e9_2690u32, 0x00e9_2cd0] {
            let map = string_map(&mut e, 0);
            assert_eq!(e.call(addr, &args![map, 0u32]).u32(), map);
            assert!(calls(&e, MEMORY_MANAGER_FREE).is_empty());
            e.call(addr, &args![map, 1u32]);
            assert_eq!(
                calls(&e, MEMORY_MANAGER_FREE),
                vec![vec![MEMORY_MANAGER_OBJECT, map]]
            );
            e.call_log = Some(vec![]);
        }
    }

    #[test]
    fn fn_00e92590_compares_the_words() {
        let mut e = engine();
        assert!(e.call(0x00e9_2590, &args![0u32, 5u32, 5u32]).bool());
        assert!(!e.call(0x00e9_2590, &args![0u32, 5u32, 6u32]).bool());
    }

    #[test]
    fn fn_00e92760_finds_the_value_and_assigns_it() {
        let mut e = engine();
        e.register(0x7104, |_, a| (a[1] % 3).into_ret());
        e.register(0x7108, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.put_vtable(0x6100, &[0, 0x7104, 0x7108]);
        let map = string_map(&mut e, 0);
        e.mem.set_u32(map, 0x6100);
        let value = object(&mut e, 1);
        let buckets = e.mem.u32(map + 8);
        let second = e.mem.u32(e.mem.u32(buckets));
        e.mem.set_u32(second + 8, value);
        let result = e.mem.alloc(4);
        let old = object(&mut e, 1);
        e.mem.set_u32(result, old);
        // Key 0x12 hashes to bucket 0 (0x12 % 3) and is its second item.
        assert!(e.call(0x00e9_2760, &args![map, 0x12u32, result]).bool());
        assert_eq!(e.mem.u32(result), value);
        assert_eq!(e.mem.u32(value + 4), 2);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(calls(&e, 0x7108).len(), 2);
        // Held already: nothing moves.
        assert!(e.call(0x00e9_2760, &args![map, 0x12u32, result]).bool());
        assert_eq!(e.mem.u32(value + 4), 2);
        // Key 0x15 hashes to bucket 0 but no item matches.
        assert!(!e.call(0x00e9_2760, &args![map, 0x15u32, result]).bool());
    }

    /// A factory whose library map iterates `libraries` (the step double hands them out).
    fn library_factory(e: &mut Engine, libraries: Vec<u32>) -> u32 {
        let factory = e.mem.alloc(0x34);
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(buckets + 4, 0x9990);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(factory + 0x30, map);
        let mut next = 0usize;
        e.register_double(MAP_STEP_00E6E630, move |e, a| {
            let library = libraries[next];
            next += 1;
            e.mem.set_u32(a[3], library);
            if library != 0 {
                let count = e.mem.u32(library + 4) + 1;
                e.mem.set_u32(library + 4, count);
            }
            let more = if next < libraries.len() { 0x9991 } else { 0 };
            e.mem.set_u32(a[1], more);
            Ret::default()
        });
        e.register(LIBRARY_GET_NAME_00D47A30, |_, a| (a[0] + 1).into_ret());
        factory
    }

    #[test]
    fn get_first_library_starts_the_iterator() {
        let mut e = engine();
        let library = object(&mut e, 1);
        let factory = library_factory(&mut e, vec![library]);
        let map = e.mem.u32(factory + 0x30);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), library);
        let step = &calls(&e, MAP_STEP_00E6E630)[0];
        assert_eq!(step[..2], [map, factory + 0x2c]);
        assert_eq!(e.mem.u32(library + 4), 1);
        assert!(calls(&e, DELETE).is_empty());
        // An empty map: the iterator is cleared and nothing is taken.
        let buckets = e.mem.u32(map + 8);
        e.mem.set_u32(buckets + 4, 0);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), 0);
        assert_eq!(e.mem.u32(factory + 0x2c), 0);
        assert_eq!(calls(&e, MAP_STEP_00E6E630).len(), 1);
        e.mem.set_u32(factory + 0x30, 0);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), 0);
    }

    #[test]
    fn remove_all_libraries_removes_each_by_name() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        let factory = library_factory(&mut e, vec![first, second]);
        let map = e.mem.u32(factory + 0x30);
        e.call(0x00e9_2c40, &args![factory]);
        assert_eq!(
            calls(&e, REMOVE_LIBRARY_00E91FD0),
            vec![vec![map, first + 1], vec![map, second + 1]]
        );
        assert_eq!(calls(&e, MAP_STEP_00E6E630).len(), 2);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![map]]);
        let factory = e.mem.alloc(0x34);
        e.call(0x00e9_2c40, &args![factory]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0).len(), 1);
    }

    #[test]
    fn factory_destructors_take_everything_down() {
        let mut e = engine();
        e.register(0x7000, |_, _| Ret::default());
        e.put_vtable(0x6100, &[0x7000]);
        let factory = library_factory(&mut e, vec![0]);
        let libraries = e.mem.u32(factory + 0x30);
        let buckets = e.mem.u32(libraries + 8);
        e.mem.set_u32(buckets + 4, 0);
        e.mem.set_u32(libraries, 0x6100);
        let shaders = e.mem.alloc(4);
        e.mem.set_u32(shaders, 0x6100);
        e.mem.set_u32(factory + 0x28, shaders);
        e.mem.set_u32(factory + 0x24, 0x99);
        assert_eq!(e.call(0x00e9_2e30, &args![factory, 1u32]).u32(), factory);
        assert_eq!(e.mem.u32(factory), 0x010f_0cc4);
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL_00B601A0),
            vec![vec![shaders], vec![libraries]]
        );
        assert_eq!(
            calls(&e, 0x7000),
            vec![vec![shaders, 1], vec![libraries, 1]]
        );
        for offset in [0x24u32, 0x28, 0x30] {
            assert_eq!(e.mem.u32(factory + offset), 0);
        }
        assert_eq!(
            calls(&e, IMPORT_DELETE_CRITICAL_SECTION),
            vec![vec![factory + 0x80]]
        );
        assert_eq!(
            calls(&e, NI_SHADER_FACTORY_DOWN_00AA05A0),
            vec![vec![factory]]
        );
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![factory, 0x100]]);
    }
}
