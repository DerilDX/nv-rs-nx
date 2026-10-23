//! `NiXenonRenderer/nid3dshaderconstantmap.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e81d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderconstantmap.cpp`): the constant map's renderer
/// set-up, `thiscall` with one stack word (the renderer), stored in `m_pkD3DRenderer` (+0x3c).
/// With a renderer: the old device in `m_pkD3DDevice` (+0x38) is released (its vtable slot +8,
/// `stdcall`), the renderer's device (+0x288) is kept there and referenced (slot +4) when not
/// null, and `m_pkD3DRenderState` (+0x40) takes the renderer's +0x8b8. Without one: the old
/// device is released, and the device and render state are cleared (Xbox PDB names).
pub fn fn_00e81d00(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 0x3c, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer + 0x288);
        let old = e.mem.u32(base + 0x38);
        if old != 0 {
            e.vcall(old, 8, &args![]);
        }
        e.mem.set_u32(base + 0x38, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(base + 0x3c);
        let state = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(base + 0x40, state);
        return;
    }
    let old = e.mem.u32(base + 0x38);
    if old != 0 {
        e.vcall(old, 8, &args![]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0);
}

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject`'s vtable (`0101dce4`) and its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
/// `NiShaderConstantMapEntry`'s vtable (`010ef4d8`, RTTI `.?AVNiShaderConstantMapEntry@@`),
/// `NiD3DShaderConstantMap`'s (`010ef544`, RTTI `.?AVNiD3DShaderConstantMap@@`) and the vtable of
/// its two entry arrays (`010ef53c`, RTTI
/// `.?AV?$NiTArray@V?$NiPointer@VNiShaderConstantMapEntry@@@@...`).
const ENTRY_VTABLE_010EF4D8: u32 = 0x010e_f4d8;
const MAP_VTABLE_010EF544: u32 = 0x010e_f544;
const ENTRY_ARRAY_VTABLE_010EF53C: u32 = 0x010e_f53c;
/// The `NiFixedString` handle the code compares every handle with before it touches the count
/// 8 bytes in front of it (`0109b220`), and the maker of a handle from a C string (`00a5b690`,
/// unnamed, `cdecl`, one word; it answers the handle with its count raised).
const FIXED_STRING_NULL_0109B220: u32 = 0x0109_b220;
const MAKE_FIXED_STRING_00A5B690: u32 = 0x00a5_b690;
/// The Ni allocator's calls the entries use (unnamed, `cdecl`; each goes through the allocator
/// at `011f6080`): `00aa13e0` (size; the entry objects), `00aa1070` (size; owned data),
/// `00aa10f0` (block; frees owned data) and `00aa1460` (block, size; frees an entry).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NI_ALLOCATE_DATA_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_DATA_00AA10F0: u32 = 0x00aa_10f0;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// CRT `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiStricmp` (`00aa1360`, `cdecl`: two strings).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (`00aa04e0`, `cdecl`: the address of a
/// fixed-string handle).
const RELEASE_GLOBAL_SHADER_CONSTANT: u32 = 0x00aa_04e0;
/// The entry-array helpers outside this unit (unnamed): `00b79a60` makes an array of
/// `NiPointer`s (`cdecl`, count), `00b7ce30` deletes one (`cdecl`, elements), `00b799c0`
/// empties an array (`thiscall`, no stack words).
const NEW_POINTER_ARRAY_00B79A60: u32 = 0x00b7_9a60;
const DELETE_POINTER_ARRAY_00B7CE30: u32 = 0x00b7_ce30;
const ARRAY_REMOVE_ALL_00B799C0: u32 = 0x00b7_99c0;
/// `SetAt`'s function-local `static` null element (`0126f910`), its guard word (bit 0,
/// `0126f914`), CRT `_atexit` (`00ec658f`) and the element's exit destructor (`00fde390`).
const NULL_ELEMENT_0126F910: u32 = 0x0126_f910;
const NULL_ELEMENT_GUARD_0126F914: u32 = 0x0126_f914;
const ATEXIT: u32 = 0x00ec_658f;
const NULL_ELEMENT_DESTRUCT_00FDE390: u32 = 0x00fd_e390;
/// The word table `AddEntry` reads for an object entry, indexed by bits 8 to 15 of the flags
/// (`011f5f90`), and the table `AddConstantEntry` reads, indexed by the low byte of the entry's
/// flags (`011f5fc4`), and the byte table `SetupPredefinedEntry` reads (`011f5fb4`), all
/// unnamed.
const OBJECT_TABLE_011F5F90: u32 = 0x011f_5f90;
const CONSTANT_TYPE_TABLE_011F5FC4: u32 = 0x011f_5fc4;
const PREDEFINED_TYPE_TABLE_011F5FB4: u32 = 0x011f_5fb4;
/// `NiShaderConstantMap::LookUpPredefinedMapping` (`00a9bfd0`, `cdecl`: key, out mapping, out
/// count; answers a `bool`) and `::LookUpPredefinedMappingType` (`00a9bbf0`, `cdecl`: mapping,
/// count) (engine map).
const LOOK_UP_PREDEFINED_MAPPING: u32 = 0x00a9_bfd0;
const LOOK_UP_PREDEFINED_MAPPING_TYPE: u32 = 0x00a9_bbf0;
/// The data the two predefined mappings 0x50 and 0x51 point their entries at (`011bf63c`,
/// `011bf64c`, 0x10 bytes each).
const PREDEFINED_DATA_011BF63C: u32 = 0x011b_f63c;
const PREDEFINED_DATA_011BF64C: u32 = 0x011b_f64c;

fn interlocked_increment(e: &mut Engine, count: u32) {
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![count]);
}

fn interlocked_decrement(e: &mut Engine, count: u32) -> u32 {
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![count]).u32()
}

/// A reference-counted object released: its count at +4 decremented and the object deleted
/// through its vtable slot +4 when that answers 0.
fn release_object(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An element slot of an `NiPointer` array given `object`, as `SetAt` and `AddFirstEmpty`
/// expand it: nothing when it holds `object` already; else the old object released when not
/// null, then the slot takes the word at `element` (read again), referenced when not null.
fn assign_slot(e: &mut Engine, slot: u32, object: u32, element: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        release_object(e, old);
    }
    let new = e.mem.u32(element);
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

/// A fixed-string field set from a C string as the entry builders expand it: the handle made
/// by `00a5b690` (the null handle `0109b220` for a null string); when the field holds another
/// handle the new one is referenced and the old one released (each unless it is the null
/// handle) and the field set; then the made handle's own reference is dropped.
fn set_fixed_string(e: &mut Engine, field: u32, text: u32) {
    let handle = if text == 0 {
        e.global::<u32>(FIXED_STRING_NULL_0109B220)
    } else {
        e.call(MAKE_FIXED_STRING_00A5B690, &args![text]).u32()
    };
    if e.mem.u32(field) != handle {
        if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_increment(e, handle.wrapping_sub(8));
        }
        let old = e.mem.u32(field);
        if old != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_decrement(e, old.wrapping_sub(8));
        }
        e.mem.set_u32(field, handle);
    }
    if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
        interlocked_decrement(e, handle.wrapping_sub(8));
    }
}

/// An entry's data source set as the entry builders do it: with `copy`, the entry owns a copy
/// (`m_bOwnData` set, the old block freed through `00aa10f0`, `size` bytes from `00aa1070`
/// stored at `m_pvDataSource` and filled by `memcpy`); without, it points at `source`.
fn set_data_source(e: &mut Engine, entry: u32, size: u32, source: u32, copy: bool) {
    if copy {
        let old = e.mem.u32(entry + 0x30);
        e.mem.set_u8(entry + 0x34, 1);
        e.call(NI_FREE_DATA_00AA10F0, &args![old]);
        let block = e.call(NI_ALLOCATE_DATA_00AA1070, &args![size]).u32();
        e.mem.set_u32(entry + 0x30, block);
        e.call(MEMCPY, &args![block, source, size]);
    } else {
        e.mem.set_u8(entry + 0x34, 0);
        e.mem.set_u32(entry + 0x30, source);
    }
}

/// A new entry: 0x38 bytes from `00aa13e0` and the constructor (`00e81ba0`) when the
/// allocation succeeded, else null (which the callers then use as it is).
fn new_entry(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x38u32]).u32();
    if block != 0 {
        ni_shader_constant_map_entry_ni_shader_constant_map_entry(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

// Translated from 00e81ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMapEntry::NiShaderConstantMapEntry` (Xbox PDB), `thiscall`, no stack words.
/// The `NiRefObject` part (vtable `0101dce4`, count cleared, `ms_uiObjects` at `011f4400`
/// raised), then the entry's vtable `010ef4d8`, `m_bEnabled` set, `m_kKey` and
/// `m_kVariableName` the null handle (`0109b220`), the words +0x14 to +0x20 and +0x28 to +0x30
/// and the three bytes +0x34 to +0x36 cleared (+0x10 is left as it is). Answers `this`.
pub fn ni_shader_constant_map_entry_ni_shader_constant_map_entry(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    interlocked_increment(e, NI_REF_OBJECT_COUNT_011F4400);
    e.mem.set_u32(base, ENTRY_VTABLE_010EF4D8);
    e.mem.set_u8(base + 8, 1);
    let null = e.global::<u32>(FIXED_STRING_NULL_0109B220);
    e.mem.set_u32(base + 0xc, null);
    for offset in [0x14u32, 0x18, 0x1c, 0x20] {
        e.mem.set_u32(base + offset, 0);
    }
    let null = e.global::<u32>(FIXED_STRING_NULL_0109B220);
    e.mem.set_u32(base + 0x24, null);
    for offset in [0x28u32, 0x2c, 0x30] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0x34u32, 0x35, 0x36] {
        e.mem.set_u8(base + offset, 0);
    }
    this
}

// Translated from 00e81c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiShaderConstantMapEntry`'s destructor body (from its scalar
/// deleting destructor), `thiscall`, no stack words. The entry's vtable `010ef4d8`; owned data
/// (`m_bOwnData`, +0x34) freed through `00aa10f0`; `m_kVariableName` (+0x24) and `m_kKey`
/// (+0xc) released (the count 8 bytes before each handle, unless it is the null handle
/// `0109b220`); then the `NiRefObject` vtable and `ms_uiObjects` lowered.
pub fn fn_00e81c00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ENTRY_VTABLE_010EF4D8);
    if e.mem.u8(base + 0x34) != 0 {
        let data = e.mem.u32(base + 0x30);
        e.call(NI_FREE_DATA_00AA10F0, &args![data]);
    }
    for field in [0x24u32, 0xc] {
        let handle = e.mem.u32(base + field);
        if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_decrement(e, handle.wrapping_sub(8));
        }
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    interlocked_decrement(e, NI_REF_OBJECT_COUNT_011F4400);
}

// Translated from 00e83630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMapEntry::~NiShaderConstantMapEntry` scalar deleting destructor (Xbox PDB
/// `_scalar_deleting_destructor_`), `thiscall` with one stack word (flags). The destructor body
/// (`00e81c00`), then with flag bit 0 the 0x38 bytes freed through `00aa1460`. Answers `this`.
pub fn ni_shader_constant_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e81c00(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00e86f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiShaderConstantMapEntry>, NiTNewInterface<...>>::SetAt` (Xbox PDB),
/// `thiscall` with two stack words: the index and the address of the element. On first use the
/// function-local null element (`0126f910`) is cleared and its destructor `00fde390` given to
/// `_atexit`. An index at or past `m_usSize` makes the size index + 1 and counts a non-null
/// element in `m_usESize`; below it `m_usESize` follows the change from null to non-null and
/// back (null meaning equal to that null element). Then the slot is assigned as an `NiPointer`
/// (old object released, new one referenced).
pub fn ni_t_array_set_at(e: &mut Engine, this: Ptr, index: u32, element: Ptr) {
    let array = this.addr();
    let element = element.addr();
    let guard = e.global::<u32>(NULL_ELEMENT_GUARD_0126F914);
    if guard & 1 == 0 {
        e.set_global(NULL_ELEMENT_GUARD_0126F914, guard | 1);
        e.set_global(NULL_ELEMENT_0126F910, 0u32);
        e.call(ATEXIT, &args![NULL_ELEMENT_DESTRUCT_00FDE390]);
    }
    let size = u32::from(e.mem.u16(array + 0xa));
    if index >= size {
        e.mem.set_u16(array + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(element) != e.global::<u32>(NULL_ELEMENT_0126F910) {
            let count = e.mem.u16(array + 0xc).wrapping_add(1);
            e.mem.set_u16(array + 0xc, count);
        }
    } else {
        let null = e.global::<u32>(NULL_ELEMENT_0126F910);
        let elements = e.mem.u32(array + 4);
        let current = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(element) != null {
            if current == null {
                let count = e.mem.u16(array + 0xc).wrapping_add(1);
                e.mem.set_u16(array + 0xc, count);
            }
        } else if current != null {
            let count = e.mem.u16(array + 0xc).wrapping_add(0xffff);
            e.mem.set_u16(array + 0xc, count);
        }
    }
    let elements = e.mem.u32(array + 4);
    let slot = elements.wrapping_add(index.wrapping_mul(4));
    let object = e.mem.u32(element);
    assign_slot(e, slot, object, element);
}

// Translated from 00e87510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the entry array's `SetSize` (`NiTArray` of entry pointers),
/// `thiscall` with one stack word (the new capacity; nothing when it equals `m_usMaxSize`).
/// A capacity below `m_usSize` first releases and clears the elements from the new capacity
/// to the size (each non-null one lowering `m_usESize`) and makes it the size. The capacity is
/// stored; capacity 0 leaves no elements, else a new array from `00b79a60` takes the old
/// elements (each slot assigned as an `NiPointer`) and the slots past the size are released
/// and cleared. The old element array goes to `00b7ce30` either way (its elements are not
/// released there).
pub fn fn_00e87510(e: &mut Engine, this: Ptr, capacity: u32) {
    let array = this.addr();
    if capacity == u32::from(e.mem.u16(array + 8)) {
        return;
    }
    let size = e.mem.u16(array + 0xa);
    let capacity16 = capacity as u16;
    if capacity < u32::from(size) {
        let mut index = capacity16;
        if index < size {
            loop {
                let elements = e.mem.u32(array + 4);
                let slot = elements.wrapping_add(u32::from(index) * 4);
                if e.mem.u32(slot) != 0 {
                    let object = e.mem.u32(slot);
                    if object != 0 {
                        release_object(e, object);
                        e.mem.set_u32(slot, 0);
                    }
                    let count = e.mem.u16(array + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(array + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(array + 0xa, capacity16);
    }
    let old = e.mem.u32(array + 4);
    e.mem.set_u16(array + 8, capacity16);
    if capacity == 0 {
        e.mem.set_u32(array + 4, 0);
    } else {
        let elements = e
            .call(NEW_POINTER_ARRAY_00B79A60, &args![u32::from(capacity16)])
            .u32();
        e.mem.set_u32(array + 4, elements);
        let mut index: u16 = 0;
        if index < e.mem.u16(array + 0xa) {
            loop {
                let elements = e.mem.u32(array + 4);
                let offset = u32::from(index) * 4;
                let slot = elements.wrapping_add(offset);
                let source = old.wrapping_add(offset);
                let object = e.mem.u32(source);
                assign_slot(e, slot, object, source);
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 0xa) {
                    break;
                }
            }
        }
        let mut index = e.mem.u16(array + 0xa);
        if index < e.mem.u16(array + 8) {
            loop {
                let elements = e.mem.u32(array + 4);
                let slot = elements.wrapping_add(u32::from(index) * 4);
                let object = e.mem.u32(slot);
                if object != 0 {
                    release_object(e, object);
                    e.mem.set_u32(slot, 0);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 8) {
                    break;
                }
            }
        }
    }
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![old]);
}

// Translated from 00e87800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiShaderConstantMapEntry>, NiTNewInterface<...>>::AddFirstEmpty` (Xbox
/// PDB), `thiscall` with one stack word (the address of the element). A null element answers
/// -1. Else the first null slot below `m_usSize` takes it (assigned as an `NiPointer`,
/// `m_usESize` raised) and its index is answered; without one the array grows by `m_usGrowBy`
/// (`00e87510`) when the size has reached `m_usMaxSize`, `SetAt` puts the element at the size,
/// and that index is answered.
pub fn ni_t_array_add_first_empty(e: &mut Engine, this: Ptr, element: Ptr) -> u32 {
    let array = this.addr();
    let element = element.addr();
    let object = e.mem.u32(element);
    if object == 0 {
        return 0xffff_ffff;
    }
    let size = e.mem.u16(array + 0xa);
    let mut index: u16 = 0;
    if index < size {
        let elements = e.mem.u32(array + 4);
        loop {
            let slot = elements.wrapping_add(u32::from(index) * 4);
            if e.mem.u32(slot) == 0 {
                assign_slot(e, slot, object, element);
                let count = e.mem.u16(array + 0xc).wrapping_add(1);
                e.mem.set_u16(array + 0xc, count);
                return u32::from(index);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u16(array + 0xa) {
                break;
            }
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow = u32::from(e.mem.u16(array + 0xe));
        fn_00e87510(e, this, grow + size);
    }
    ni_t_array_set_at(e, this, size, Ptr::new(element));
    size
}

/// The element of `InsertEntry` and `InsertDefinedEntry`: the entry held for the call (count
/// raised), `AddFirstEmpty` on the array at `array` (+0xc or +0x1c) with the address of the
/// argument's stack word, the entry released, `m_eLastError` (+0x34) 0x30 when it answered -1;
/// answers `m_eLastError`.
fn insert_into(e: &mut Engine, this: Ptr, array: u32, entry: Ptr) -> u32 {
    let base = this.addr();
    let entry = entry.addr();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), entry);
        if entry != 0 {
            interlocked_increment(e, entry + 4);
        }
        let index = ni_t_array_add_first_empty(e, Ptr::new(base + array), local);
        let failed = index == 0xffff_ffff;
        if entry != 0 {
            release_object(e, entry);
        }
        if failed {
            e.mem.set_u32(base + 0x34, 0x30);
        }
        e.mem.u32(base + 0x34)
    })
}

// Translated from 00e878c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::InsertEntry` (Xbox PDB), `thiscall` with one stack word (the entry):
/// adds it to `m_aspEntries` (+0xc) through `AddFirstEmpty`; `m_eLastError` (+0x34) becomes
/// 0x30 when that fails, and is answered.
pub fn ni_d3d_shader_constant_map_insert_entry(e: &mut Engine, this: Ptr, entry: Ptr) -> u32 {
    insert_into(e, this, 0xc, entry)
}

// Translated from 00e87920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::InsertDefinedEntry` (Xbox PDB): as `InsertEntry`, into
/// `m_aspDefinedEntries` (+0x1c).
pub fn ni_d3d_shader_constant_map_insert_defined_entry(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
) -> u32 {
    insert_into(e, this, 0x1c, entry)
}

// Translated from 00e870c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the per-array step of `SortByEnabled`, `thiscall` on the map
/// (not read) with one stack word, the entry array. Arrays of fewer than two elements stay.
/// Walking the elements with one entry held (count raised while held), it remembers the first
/// disabled entry's index (`m_bEnabled`, +8, not 1); an enabled entry after it is swapped there
/// (`SetAt` of the disabled one at the enabled one's index, then of the held one at the
/// remembered index) and the remembered index moves on past the following enabled entries. The
/// held entry is released at the end. As in the exe, the step after a swap releases and reads
/// the held entry without a null check.
pub fn fn_00e870c0(e: &mut Engine, _unused_0: Ptr, array: Ptr) {
    let size = u32::from(e.mem.u16(array.addr() + 0xa));
    if size <= 1 {
        return;
    }
    let array_addr = array.addr();
    e.with_stack(4, |e, local| {
        let held_slot = local.addr();
        let mut held = 0u32;
        e.mem.set_u32(held_slot, held);
        let mut disabled = u32::MAX;
        let mut index = 0u32;
        loop {
            let elements = e.mem.u32(array_addr + 4);
            let slot = elements.wrapping_add(index.wrapping_mul(4));
            let mut skip = false;
            if held != e.mem.u32(slot) {
                if held != 0 {
                    release_object(e, held);
                }
                held = e.mem.u32(slot);
                e.mem.set_u32(held_slot, held);
                if held == 0 {
                    skip = true;
                } else {
                    interlocked_increment(e, held + 4);
                }
            }
            if !skip && held != 0 {
                if e.mem.u8(held + 8) == 1 {
                    if disabled < index {
                        let elements = e.mem.u32(array_addr + 4);
                        let other = elements.wrapping_add(disabled.wrapping_mul(4));
                        ni_t_array_set_at(e, array, index, Ptr::new(other));
                        ni_t_array_set_at(e, array, disabled, local);
                        disabled = disabled.wrapping_add(1);
                        if disabled >= size {
                            release_object(e, held);
                            return;
                        }
                        loop {
                            let elements = e.mem.u32(array_addr + 4);
                            let slot = elements.wrapping_add(disabled.wrapping_mul(4));
                            if held != e.mem.u32(slot) {
                                release_object(e, held);
                                held = e.mem.u32(slot);
                                e.mem.set_u32(held_slot, held);
                                if held != 0 {
                                    interlocked_increment(e, held + 4);
                                }
                            }
                            if e.mem.u8(held.wrapping_add(8)) == 0 {
                                break;
                            }
                            disabled = disabled.wrapping_add(1);
                            if disabled >= size {
                                release_object(e, held);
                                return;
                            }
                        }
                        if disabled >= size {
                            release_object(e, held);
                            return;
                        }
                    }
                } else if disabled > index {
                    disabled = index;
                }
            }
            index += 1;
            if index >= size {
                break;
            }
        }
        if held != 0 {
            release_object(e, held);
        }
    });
}

// Translated from 00e87200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SortByEnabled` (Xbox PDB), `thiscall`, no stack words: the step
/// `00e870c0` on `m_aspEntries` (+0xc), then on `m_aspDefinedEntries` (+0x1c).
pub fn ni_d3d_shader_constant_map_sort_by_enabled(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_00e870c0(e, this, Ptr::new(base + 0xc));
    fn_00e870c0(e, this, Ptr::new(base + 0x1c));
}

// Translated from 00e82640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::GetEntry` (Xbox PDB), `thiscall` with one stack word (the key):
/// the first entry of `m_aspDefinedEntries` (+0x1c), then of `m_aspEntries` (+0xc), whose key
/// (+0xc) `NiStricmp` finds equal, else null. Each array is walked up to its size as read at
/// the start.
pub fn ni_d3d_shader_constant_map_get_entry(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let base = this.addr();
    for array in [0x1cu32, 0xc] {
        let size = u32::from(e.mem.u16(base + array + 0xa));
        let mut index = 0u32;
        while index < size {
            let elements = e.mem.u32(base + array + 4);
            let entry = e.mem.u32(elements.wrapping_add(index * 4));
            if entry != 0 {
                let name = e.mem.u32(entry + 0xc);
                if e.call(NI_STRICMP, &args![key, name]).u32() == 0 {
                    return entry;
                }
            }
            index += 1;
        }
    }
    0
}

// Translated from 00e81d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddEntry` (Xbox PDB), `thiscall` with ten stack words: the key,
/// the flags, the extra word, then seven more passed on. `m_eLastError` (+0x34) is cleared;
/// an entry `GetEntry` (vtable +0x70) finds whose `m_uiExtra` (+0x18) equals the extra word
/// gives 0x20. Else the flags' top four bits choose the adder (Xbox PDB slot names):
/// 0x3 `AddAttributeEntry` (+0x10) and 0x1 `AddConstantEntry` (+0x14) and 0x4
/// `AddGlobalEntry` (+0x18) with all ten words, 0x2 `AddPredefinedEntry` (+0x68) with the key,
/// the extra word, the fourth and sixth words and four zeros, 0x5 `AddOperatorEntry` (+0x1c)
/// with the first six words, 0x6 `AddObjectEntry` (+0x20) with the key, the fourth, sixth and
/// third words and the word of the table at `011f5f90` that bits 8 to 15 of the flags index;
/// any other kind gives 0x40. The result is stored in `m_eLastError` and answered.
pub fn ni_d3d_shader_constant_map_add_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: u32,
) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base + 0x34, 0);
    let entry = e.vcall(base, 0x70, &args![key]).u32();
    if entry != 0 && e.mem.u32(entry + 0x18) == extra {
        e.mem.set_u32(base + 0x34, 0x20);
        return 0x20;
    }
    let all = args![
        key,
        flags,
        extra,
        register,
        register_count,
        variable,
        data_size,
        data_stride,
        data_source,
        copy_data
    ];
    let result = match flags & 0xf000_0000 {
        0x3000_0000 => e.vcall(base, 0x10, &all).u32(),
        0x1000_0000 => e.vcall(base, 0x14, &all).u32(),
        0x2000_0000 => e
            .vcall(
                base,
                0x68,
                &args![key, extra, register, variable, 0u32, 0u32, 0u32, 0u32],
            )
            .u32(),
        0x4000_0000 => e.vcall(base, 0x18, &all).u32(),
        0x5000_0000 => e
            .vcall(
                base,
                0x1c,
                &args![key, flags, extra, register, register_count, variable],
            )
            .u32(),
        0x6000_0000 => {
            let object = e.mem.u32(OBJECT_TABLE_011F5F90 + ((flags >> 8) & 0xff) * 4);
            e.vcall(base, 0x20, &args![key, register, variable, extra, object])
                .u32()
        }
        _ => {
            e.mem.set_u32(base + 0x34, 0x40);
            return e.mem.u32(base + 0x34);
        }
    };
    e.mem.set_u32(base + 0x34, result);
    result
}

// Translated from 00e81f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddPredefinedEntry` (Xbox PDB), `thiscall` with eight stack words:
/// the key, the extra word, the register, the variable name, the data size and stride, the
/// data source and whether to copy it (low byte). A new entry (`00aa13e0`, constructor
/// `00e81ba0`) gets the key and the variable name as fixed strings, `m_uiExtra` (+0x18),
/// `m_uiShaderRegister` (+0x1c), size (+0x28), stride (+0x2c), the data (copied into an owned
/// block, or pointed at) and the flags 0x20000000. `SetupPredefinedEntry` (vtable +0x88)
/// answers into `m_eLastError` (+0x34); 0 inserts the entry (`InsertDefinedEntry`, +0x84,
/// answered and stored), else the entry is deleted (its vtable slot 0 with 1) and the error
/// answered. A failed allocation is used as the null entry, as in the exe.
pub fn ni_d3d_shader_constant_map_add_predefined_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    extra: u32,
    register: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: bool,
) -> u32 {
    let base = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x28), data_size);
    e.mem.set_u32(entry.wrapping_add(0x2c), data_stride);
    set_data_source(e, entry, data_size, data_source, copy_data);
    e.mem.set_u32(entry.wrapping_add(0x14), 0x2000_0000);
    let error = e.vcall(base, 0x88, &args![entry]).u32();
    e.mem.set_u32(base + 0x34, error);
    if error == 0 {
        let error = e.vcall(base, 0x84, &args![entry]).u32();
        e.mem.set_u32(base + 0x34, error);
        return error;
    }
    e.vcall(entry, 0, &args![1u32]);
    e.mem.u32(base + 0x34)
}

// Translated from 00e83650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddConstantEntry` (Xbox PDB), `thiscall` with ten stack words: the
/// key, the flags, the extra word, the register, the register count, the variable name, the
/// data size and stride, the data source and whether to copy it (low byte). A new entry gets the
/// key; its flags are the given ones' low 28 bits with 0x10000000, and by data size 8: 5,
/// 0xc: 6, 0x10: 7, 0x24: 8, 0x20/0x30/0x40: 9 or'ed in. Its `m_uiExtra` (+0x18) is 1 (program
/// type 0, the map's +8) or 2 (program type 2) for a size that is a multiple of 4 with stride 4
/// whose type in the table at `011f5fc4` (the flags' low byte) is neither 1 nor 3, else the
/// given extra word. Register (+0x1c), count (+0x20), variable name, stride (+0x2c), size
/// (+0x28) and the data follow. `InsertEntry` (vtable +0x80) answers into `m_eLastError`
/// (+0x34); on an error the entry is deleted (slot 0 with 1) and the error answered, else 0.
pub fn ni_d3d_shader_constant_map_add_constant_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: bool,
) -> u32 {
    let base = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    let mut entry_flags = (flags & 0x0fff_ffff) | 0x1000_0000;
    match data_size {
        8 => entry_flags |= 5,
        0xc => entry_flags |= 6,
        0x10 => entry_flags |= 7,
        0x24 => entry_flags |= 8,
        0x20 | 0x30 | 0x40 => entry_flags |= 9,
        _ => {}
    }
    e.mem.set_u32(entry.wrapping_add(0x14), entry_flags);
    let mut chosen = None;
    if data_size & 3 == 0 && data_stride == 4 {
        let kind = e
            .mem
            .u32(CONSTANT_TYPE_TABLE_011F5FC4 + (entry_flags & 0xff) * 4);
        if kind != 1 && kind != 3 {
            match e.mem.u32(base + 8) {
                0 => chosen = Some(data_stride.wrapping_sub(3)),
                2 => chosen = Some(2),
                _ => {}
            }
        }
    }
    e.mem
        .set_u32(entry.wrapping_add(0x18), chosen.unwrap_or(extra));
    e.mem.set_u32(entry.wrapping_add(0x20), register_count);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x2c), data_stride);
    e.mem.set_u32(entry.wrapping_add(0x28), data_size);
    set_data_source(e, entry, data_size, data_source, copy_data);
    let error = e.vcall(base, 0x80, &args![entry]).u32();
    e.mem.set_u32(base + 0x34, error);
    if error != 0 {
        e.vcall(entry, 0, &args![1u32]);
        return e.mem.u32(base + 0x34);
    }
    0
}

// Translated from 00e829b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupPredefinedEntry` (Xbox PDB), `thiscall` (the map is not read)
/// with one stack word, the entry. `LookUpPredefinedMapping` (`00a9bfd0`) of the entry's key
/// gives the mapping and a count (4 to start with); not found or mapping 0 answers 0x40. The
/// low byte of the entry's flags (+0x14) becomes the byte of the table at `011f5fb4` that
/// `LookUpPredefinedMappingType` (`00a9bbf0`) indexes. Below mapping 0x45 the register count
/// (+0x20) is set: 0x21/0x23 three times the extra word's high half (+0x1a), 0x22/0x24 four
/// times, 0x19 to 0x20 four (the arms for 0x60 to 0x63 are never reached there but are kept),
/// else the looked-up count. `m_uiInternal` (+0x10) takes the mapping; mapping 0x50 or 0x51
/// points the entry at its 0x10 bytes (`011bf63c`, `011bf64c`; not owned, size and stride
/// 0x10). Answers 0.
pub fn ni_d3d_shader_constant_map_setup_predefined_entry(
    e: &mut Engine,
    _unused_0: Ptr,
    entry: Ptr,
) -> u32 {
    let entry = entry.addr();
    e.with_stack(8, |e, locals| {
        let mapping_word = locals.addr();
        let count_word = locals.addr() + 4;
        e.mem.set_u32(count_word, 4);
        let key = e.mem.u32(entry + 0xc);
        let found = e
            .call(
                LOOK_UP_PREDEFINED_MAPPING,
                &args![key, mapping_word, count_word],
            )
            .bool();
        if !found {
            return 0x40;
        }
        let mapping = e.mem.u32(mapping_word);
        let count = e.mem.u32(count_word);
        let kind = e
            .call(LOOK_UP_PREDEFINED_MAPPING_TYPE, &args![mapping, count])
            .u32();
        let byte = u32::from(e.mem.u8(PREDEFINED_TYPE_TABLE_011F5FB4.wrapping_add(kind)));
        let flags = e.mem.u32(entry + 0x14);
        e.mem.set_u32(entry + 0x14, byte | (flags & 0xffff_ff00));
        let mapping = e.mem.u32(mapping_word);
        if mapping == 0 {
            return 0x40;
        }
        if mapping < 0x45 {
            let registers = match mapping {
                0x21 | 0x23 => u32::from(e.mem.u16(entry + 0x1a)) * 3,
                0x22 | 0x24 => u32::from(e.mem.u16(entry + 0x1a)) * 4,
                0x19..=0x20 | 0x60..=0x63 => 4,
                _ => e.mem.u32(count_word),
            };
            e.mem.set_u32(entry + 0x20, registers);
        }
        e.mem.set_u32(entry + 0x10, mapping);
        let data = match mapping {
            0x50 => PREDEFINED_DATA_011BF63C,
            0x51 => PREDEFINED_DATA_011BF64C,
            _ => return 0,
        };
        e.mem.set_u32(entry + 0x30, data);
        e.mem.set_u8(entry + 0x34, 0);
        e.mem.set_u32(entry + 0x2c, 0x10);
        e.mem.set_u32(entry + 0x28, 0x10);
        0
    })
}

// Translated from 00e826d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetShaderConstants` (Xbox PDB), `thiscall` with eleven stack words:
/// the shader program (kept in `m_pkLastShaderProgram`, +0x30) and ten words passed on. Every
/// enabled entry (`m_bEnabled`, +8) with a register (+0x1c not -1) or a variable name (+0x24)
/// is set. First the predefined ones (kind 0x2) of `m_aspDefinedEntries` through
/// `SetDefinedConstant` (vtable +0x8c: program, entry, the ten words). Then by kind those of
/// `m_aspEntries`: constants (0x1) with extra word 1 or 2 go to the device (+0x38) as vertex
/// (slot +0x178) or pixel (slot +0x1b4) float constants (register, data, register count; a
/// negative result fails), other constants through `SetConstantConstant` (+0x90: program,
/// entry, the tenth word); attributes (0x3, +0x94: program, entry, the ten words, 0), globals
/// (0x4, +0x98: program, entry, the tenth word), operators (0x5, +0x9c: program, entry, the
/// second, third and sixth to tenth words) and objects (0x6, +0xa4: program, entry, the ten
/// words); predefined ones are skipped, any other kind fails. A failure is a non-zero result.
/// Answers whether any failed.
pub fn ni_d3d_shader_constant_map_set_shader_constants(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + 0x30, program);
    let mut ok = true;
    let size = u32::from(e.mem.u16(base + 0x26));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base + 0x20);
        let entry = e.mem.u32(elements.wrapping_add(index * 4));
        index += 1;
        if entry == 0 || e.mem.u8(entry + 8) == 0 {
            continue;
        }
        if e.mem.u32(entry + 0x1c) == 0xffff_ffff && e.mem.u32(entry + 0x24) == 0 {
            continue;
        }
        if e.mem.u32(entry + 0x14) & 0xf000_0000 != 0x2000_0000 {
            continue;
        }
        let result = e
            .vcall(
                base,
                0x8c,
                &args![
                    program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9,
                    word_10
                ],
            )
            .u32();
        if result != 0 {
            ok = false;
        }
    }
    let size = u32::from(e.mem.u16(base + 0x16));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base + 0x10);
        let entry = e.mem.u32(elements.wrapping_add(index * 4));
        index += 1;
        if entry == 0 || e.mem.u8(entry + 8) == 0 {
            continue;
        }
        let register = e.mem.u32(entry + 0x1c);
        if register == 0xffff_ffff && e.mem.u32(entry + 0x24) == 0 {
            continue;
        }
        let result = match e.mem.u32(entry + 0x14) & 0xf000_0000 {
            0x2000_0000 => continue,
            0x1000_0000 => match e.mem.u32(entry + 0x18) {
                1 | 2 => {
                    let slot = if e.mem.u32(entry + 0x18) == 1 {
                        0x178
                    } else {
                        0x1b4
                    };
                    let data = e.mem.u32(entry + 0x30);
                    let count = e.mem.u32(entry + 0x20);
                    let device = e.mem.u32(base + 0x38);
                    let answer = e.vcall(device, slot, &args![register, data, count]).i32();
                    u32::from(answer < 0)
                }
                _ => e.vcall(base, 0x90, &args![program, entry, word_10]).u32(),
            },
            0x3000_0000 => e
                .vcall(
                    base,
                    0x94,
                    &args![
                        program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8,
                        word_9, word_10, word_11, 0u32
                    ],
                )
                .u32(),
            0x4000_0000 => e.vcall(base, 0x98, &args![program, entry, word_10]).u32(),
            0x5000_0000 => e
                .vcall(
                    base,
                    0x9c,
                    &args![program, entry, word_2, word_3, word_6, word_7, word_8, word_9, word_10],
                )
                .u32(),
            0x6000_0000 => e
                .vcall(
                    base,
                    0xa4,
                    &args![
                        program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8,
                        word_9, word_10
                    ],
                )
                .u32(),
            _ => 1,
        };
        if result != 0 {
            ok = false;
        }
    }
    !ok
}

// Translated from 00e87690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiD3DShaderConstantMap`'s destructor body (from the scalar
/// deleting destructor at `00b7e413`), `thiscall`, no stack words. The map's vtable
/// `010ef544`; every global entry (kind 0x4) of `m_aspEntries` gives its key to
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (through a held copy of the handle) and of
/// `m_aspDefinedEntries` the address of its key (each entry held across it); each array is then
/// emptied (`00b799c0`). The device (+0x38) is released (its slot +8) and the renderer, device
/// and render state cleared; the two arrays get their vtable `010ef53c` and their elements go to
/// `00b7ce30`; last the `NiRefObject` vtable and `ms_uiObjects` lowered.
pub fn fn_00e87690(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_VTABLE_010EF544);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let mut index = 0u32;
        if e.mem.u16(base + 0x14) != 0 {
            loop {
                let elements = e.mem.u32(base + 0x10);
                let entry = e.mem.u32(elements.wrapping_add(index * 4));
                if entry != 0 {
                    interlocked_increment(e, entry + 4);
                    if e.mem.u32(entry + 0x14) & 0xf000_0000 == 0x4000_0000 {
                        let key = e.mem.u32(entry + 0xc);
                        if key != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                            interlocked_increment(e, key.wrapping_sub(8));
                        }
                        let key = e.mem.u32(entry + 0xc);
                        e.mem.set_u32(local, key);
                        e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![local]);
                        let key = e.mem.u32(local);
                        if key != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                            interlocked_decrement(e, key.wrapping_sub(8));
                        }
                    }
                    release_object(e, entry);
                }
                index += 1;
                if index >= u32::from(e.mem.u16(base + 0x14)) {
                    break;
                }
            }
        }
    });
    e.call(ARRAY_REMOVE_ALL_00B799C0, &args![base + 0xc]);
    let mut index = 0u32;
    if e.mem.u16(base + 0x24) != 0 {
        loop {
            let elements = e.mem.u32(base + 0x20);
            let entry = e.mem.u32(elements.wrapping_add(index * 4));
            if entry != 0 {
                interlocked_increment(e, entry + 4);
                if e.mem.u32(entry + 0x14) & 0xf000_0000 == 0x4000_0000 {
                    e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![entry + 0xc]);
                }
                release_object(e, entry);
            }
            index += 1;
            if index >= u32::from(e.mem.u16(base + 0x24)) {
                break;
            }
        }
    }
    e.call(ARRAY_REMOVE_ALL_00B799C0, &args![base + 0x1c]);
    let device = e.mem.u32(base + 0x38);
    e.mem.set_u32(base + 0x3c, 0);
    if device != 0 {
        e.vcall(device, 8, &args![]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0);
    let elements = e.mem.u32(base + 0x20);
    e.mem.set_u32(base + 0x1c, ENTRY_ARRAY_VTABLE_010EF53C);
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![elements]);
    let elements = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, ENTRY_ARRAY_VTABLE_010EF53C);
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![elements]);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    interlocked_decrement(e, NI_REF_OBJECT_COUNT_011F4400);
}

/// What `SetDefinedConstant` calls: D3DX's `D3DXMatrixInverse` (`00ee6dda`; `stdcall`: out,
/// determinant, matrix), `D3DXMatrixTranspose` (`00ee6dd4`: out, matrix), `D3DXMatrixMultiply`
/// (`00ee6de0`: out, first, second) and `00ee6dfe` (unnamed; `stdcall`: out and two matrices);
/// `NiD3DShaderConstantMap::SetupTextureTransformMatrix` (`00e83550`, engine map; `cdecl`: out,
/// matrix, two bytes, a count); `NiTextureTransform::UpdateMatrix` (`00bc5c20`, engine map;
/// `thiscall`, answers its matrix); `00a463e0` (unit `nitexturetransformcontroller.cpp`;
/// `thiscall` on a texturing property with an index, answers a map); the extra data name
/// handle (`011bf65c`) with the `NiFixedString` constructor `00438170` (`thiscall`, the
/// handle) and destructor `004381b0`; `NiObjectNET::GetExtraData` (`00a5bdd0`, engine map;
/// `thiscall`, the address of the name); the CRT's x87 helpers `00eca0a0`, `00ec9f70` and
/// `00eca5d0` (the value in and out in ST0); `00653290` (unit `bsfacegenmanager.cpp`;
/// `thiscall` with the record `011f5f6c`, answers an object or 0) and `00633b40` (unnamed;
/// `thiscall` with the address of three floats and a float).
const MATRIX_INVERSE_00EE6DDA: u32 = 0x00ee_6dda;
const MATRIX_TRANSPOSE_00EE6DD4: u32 = 0x00ee_6dd4;
const MATRIX_MULTIPLY_00EE6DE0: u32 = 0x00ee_6de0;
const MATRIX_PRODUCT_00EE6DFE: u32 = 0x00ee_6dfe;
const SETUP_TEXTURE_TRANSFORM_00E83550: u32 = 0x00e8_3550;
const UPDATE_MATRIX_00BC5C20: u32 = 0x00bc_5c20;
const MAP_AT_00A463E0: u32 = 0x00a4_63e0;
const EXTRA_DATA_NAME_011BF65C: u32 = 0x011b_f65c;
const FIXED_STRING_CONSTRUCT_00438170: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT_004381B0: u32 = 0x0043_81b0;
const GET_EXTRA_DATA_00A5BDD0: u32 = 0x00a5_bdd0;
const CRT_HELPER_00ECA0A0: u32 = 0x00ec_a0a0;
const CRT_HELPER_00EC9F70: u32 = 0x00ec_9f70;
const CRT_HELPER_00ECA5D0: u32 = 0x00ec_a5d0;
const LIGHT_CAST_00653290: u32 = 0x0065_3290;
const LIGHT_RECORD_011F5F6C: u32 = 0x011f_5f6c;
const LIGHT_OFFSET_00633B40: u32 = 0x0063_3b40;
/// Three floats `SetDefinedConstant` reads for the constants 73 and 74 (`011f4998` to
/// `011f49a0`) and 2^32 as a float (`01099378`).
const VECTOR_011F4998: u32 = 0x011f_4998;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// The results `SetDefinedConstant` answers: the extra data not found (0x10) and the program
/// refusing the constant (0x50).
const NO_EXTRA_DATA: u32 = 0x10;
const NOT_SET: u32 = 0x50;
/// 1.0 as a float (what `FLD1` and a float store leave).
const ONE: u32 = 0x3f80_0000;

/// `FLD dword` then `FSTP dword` of the float at `at`: its bits, a signalling NaN quieted.
fn copy_float(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

/// 16 words copied (`REP MOVSD`).
fn copy_matrix(e: &mut Engine, from: u32, to: u32) {
    for word in 0..16u32 {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

/// The matrix at `matrix` inverted in place (`D3DXMatrixInverse` with no determinant) when
/// `inverse`, then transposed in place when `transpose`.
fn invert_transpose(e: &mut Engine, matrix: u32, inverse: bool, transpose: bool) {
    if inverse {
        e.call(MATRIX_INVERSE_00EE6DDA, &args![matrix, 0u32, matrix]);
    }
    if transpose {
        e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, matrix]);
    }
}

/// Stores four float bit patterns at `at`.
fn store_vector(e: &mut Engine, at: u32, values: [u32; 4]) {
    for (i, value) in values.iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * i as u32), *value);
    }
}

/// The value of the 16-bit dimension a texture's slot answers as `FILD` takes it (2^32 added,
/// `FADD` of the float at `01099378`, when it is negative as a signed word), stored as a float.
fn unsigned_to_float(e: &Engine, word: u32) -> u32 {
    let mut value = f64::from(word as i32);
    if (word as i32) < 0 {
        value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
    }
    x87_store32(value)
}

/// A CRT helper on the float `time` (`FLD`, the call with the value in ST0, its result stored
/// as a float, loaded and stored again): the result's bits.
fn crt_on(e: &mut Engine, helper: u32, time: u32) -> u32 {
    let result = e.call(helper, &args![x87_load32(time)]).f64();
    x87_quiet_bits(x87_store32(result))
}

// Translated from 00e83db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetDefinedConstant` (engine map), `thiscall` with eleven words (the
/// sixth and the last three not read): the shader program, the constant map entry, an object
/// with extra data, two words for the skinning constants, the property state and the dynamic
/// effect state. The entry's predefined constant (+0x10) picks the data handed to the program's
/// `SetShaderConstant` (its slot 0x74: the entry, the data, 0); answers 0, or 0x50 when the
/// program answers false. The compiler's switch over the constant (a byte map and a jump
/// table in the code) is the `match` below. By constant (the renderer is `m_pkD3DRenderer`
/// +0x3c, Xbox PDB; "inverted and transposed" means the second and fourth constant of a group
/// of four inverted with `D3DXMatrixInverse`, the third and fourth transposed with
/// `D3DXMatrixTranspose`):
/// - 1/2/13/14, 3/4/15/16, 5/6/17/18: a copy of the renderer's matrix at +0x9c0, +0x980 or
///   +0x940, inverted for 2/14, 4/16, 6/18 and transposed for 13/14, 15/16, 17/18.
/// - 7/8/19/20: the product (`D3DXMatrixMultiply`) of copies of +0x940 and +0x980, inverted for
///   8/20 and transposed for 19/20; 9/10/21/22 the product of +0x980 and +0x9c0 (10/22, 21/22).
/// - 11/12/23/24: `00ee6dfe` of +0x940 and +0xa00 into the stack.
/// - 25 to 28 (needing the fourth word): a copy of +0x980; 29 to 32 (needing it): the product of
///   +0x980 and +0x9c0; inverted and transposed.
/// - 33/35 (needing the fourth and fifth words) and 34/36 (needing the fourth): the program's
///   `SetShaderConstantArray` (slot 0x78) with the entry, the fourth word's +0x28, the fifth's
///   16-bit +0x20, the fourth's +0x20 and the fifth's +4.
/// - 37 to 60 and 88 to 95 in groups of four: the texture maps 0 to 5, then 6 and 7, of the
///   property state's texturing property (+0x14, its map array at +0x20); 61 to 64 the map at
///   the entry's +0x18 in the array from +0x20; 65 to 68 the map `00a463e0` answers for that
///   index. A map with a transform (+0xc) gives its matrix (`UpdateMatrix`) and the two flags
///   (inverted, transposed) to `SetupTextureTransformMatrix` (`00e83550`, with 4); without a
///   map neither.
/// - 73/74, 75, 76, 77: four floats from the property state's +8 object (none: answers 0): the
///   floats at `011f4998` and its +0x3c; its +0x1c to +0x24 and +0x3c; three floats from its
///   +0x34 (or else its +0x28) and +0x3c; its +0x38 four times.
/// - 78, 79: the last and the third row of a copy of the renderer's +0xa40 matrix.
/// - 80, 81: no data.
/// - 82 to 86: the float +0xc of the object's extra data named by `011bf65c` (none: the result
///   becomes 0x10 and the read is made at address 0xc, as the code makes it): four times as it
///   is, four times through each of the CRT helpers `00eca0a0`, `00ec9f70`, `00eca5d0`, or the
///   value and the three.
/// - 87: (0, 0, 0, 1) plus, for each light of the effect state's list (+0xc) that `00653290`
///   answers an object for, the three floats `00633b40` gives from its +0xc8 and +0xc4.
/// - 96 to 99 (needing the fourth word): the identity, inverted and transposed.
/// - 103, 104, 105: map 5's four floats at +0x18 (else 1s); its +0x14 and +0x10 (else 0 and
///   1); map 7's +0x10 (else 0).
/// - 106 to 115 (needing the texturing property): the width and height (slots 0x94 and 0x98 of
///   the map's texture +8, as unsigned) for maps 0 to 7, the map at the entry's index or the one
///   `00a463e0` answers (else 0 and 0).
/// - Any other: answers 0.
pub fn ni_d3d_shader_constant_map_set_defined_constant(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    entry: u32,
    object: u32,
    fourth: u32,
    fifth: u32,
    _unused_6: u32,
    properties: u32,
    effects: u32,
    _unused_9: u32,
    _unused_10: u32,
    _unused_11: u32,
) -> u32 {
    let base = this.addr();
    let id = e.mem.u32(entry.wrapping_add(0x10));
    // The inverted and transposed members of the group of four from `first`.
    let flags = |first: u32| {
        (
            id == first + 1 || id == first + 3,
            id == first + 2 || id == first + 3,
        )
    };
    let renderer = |e: &Engine| e.mem.u32(base.wrapping_add(0x3c));
    let texturing = |e: &Engine| e.mem.u32(properties.wrapping_add(0x14));
    // The texturing property's map array (+0x20, not checked) word at `offset`.
    let map_word = |e: &Engine, texturing: u32, offset: u32| {
        let maps = e.mem.u32(texturing.wrapping_add(0x20));
        e.mem.u32(maps.wrapping_add(offset))
    };
    let entry_index = |e: &Engine| e.mem.u32(entry.wrapping_add(0x18));
    e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let pair = frame + 0x1c;
        let single = frame + 0x24;
        let vector = frame + 0x30;
        let first_matrix = frame + 0x40;
        let name = frame + 0x8c;
        let second_matrix = frame + 0x90;
        let third_matrix = frame + 0xd0;
        let mut result = 0u32;
        // A copy of the renderer's matrix at `offset`, inverted and transposed as asked.
        let renderer_matrix = |e: &mut Engine, offset: u32, inverse: bool, transpose: bool| {
            let from = renderer(e).wrapping_add(offset);
            copy_matrix(e, from, first_matrix);
            invert_transpose(e, first_matrix, inverse, transpose);
            first_matrix
        };
        // The product of the renderer's matrices at `left` and `right` (copied into the second
        // and third stack matrices, the left one in whichever the code uses), inverted and
        // transposed as asked.
        let product = |e: &mut Engine,
                       left: (u32, u32),
                       right: (u32, u32),
                       inverse: bool,
                       transpose: bool| {
            let renderer = renderer(e);
            copy_matrix(e, renderer.wrapping_add(left.0), left.1);
            copy_matrix(e, renderer.wrapping_add(right.0), right.1);
            e.call(
                MATRIX_MULTIPLY_00EE6DE0,
                &args![first_matrix, left.1, right.1],
            );
            invert_transpose(e, first_matrix, inverse, transpose);
            first_matrix
        };
        // The texture transform of `map` (0 for none) set up into the second stack matrix.
        let texture_transform = |e: &mut Engine, map: u32, first: u32| {
            let mut matrix = 0;
            let (mut inverse, mut transpose) = (false, false);
            if map != 0 {
                let transform = e.mem.u32(map.wrapping_add(0xc));
                if transform != 0 {
                    matrix = e.call(UPDATE_MATRIX_00BC5C20, &args![transform]).u32();
                    (inverse, transpose) = flags(first);
                }
            }
            // The flags go as whole stack words whose upper three bytes the code never wrote;
            // the callee reads only their low bytes.
            e.call(
                SETUP_TEXTURE_TRANSFORM_00E83550,
                &args![
                    second_matrix,
                    matrix,
                    u32::from(inverse),
                    u32::from(transpose),
                    4u32
                ],
            );
            second_matrix
        };
        // A texture map of the texturing property, 0 without the property.
        let slot_map = |e: &Engine, offset: u32| {
            let texturing = texturing(e);
            if texturing == 0 {
                0
            } else {
                map_word(e, texturing, offset)
            }
        };
        let data = match id {
            1 | 2 | 13 | 14 => renderer_matrix(e, 0x9c0, id == 2 || id == 14, id >= 13),
            3 | 4 | 15 | 16 => renderer_matrix(e, 0x980, id == 4 || id == 16, id >= 15),
            5 | 6 | 17 | 18 => renderer_matrix(e, 0x940, id == 6 || id == 18, id >= 17),
            7 | 8 | 19 | 20 => product(
                e,
                (0x940, second_matrix),
                (0x980, third_matrix),
                id == 8 || id == 20,
                id >= 19,
            ),
            9 | 10 | 21 | 22 => product(
                e,
                (0x980, third_matrix),
                (0x9c0, second_matrix),
                id == 10 || id == 22,
                id >= 21,
            ),
            11 | 12 | 23 | 24 => {
                let renderer = renderer(e);
                e.call(
                    MATRIX_PRODUCT_00EE6DFE,
                    &args![
                        second_matrix,
                        renderer.wrapping_add(0x940),
                        renderer.wrapping_add(0xa00)
                    ],
                );
                second_matrix
            }
            25..=28 => {
                if fourth == 0 {
                    return 0;
                }
                let (inverse, transpose) = flags(0x19);
                renderer_matrix(e, 0x980, inverse, transpose)
            }
            29..=32 => {
                if fourth == 0 {
                    return 0;
                }
                let (inverse, transpose) = flags(0x1d);
                product(
                    e,
                    (0x980, third_matrix),
                    (0x9c0, second_matrix),
                    inverse,
                    transpose,
                )
            }
            33..=36 => {
                if fourth == 0 || ((id == 33 || id == 35) && fifth == 0) {
                    return 0;
                }
                let matrices = e.mem.u32(fourth.wrapping_add(0x28));
                let bones = u32::from(e.mem.u16(fifth.wrapping_add(0x20)));
                let registers = e.mem.u32(fourth.wrapping_add(0x20));
                let indices = e.mem.u32(fifth.wrapping_add(4));
                let set = e
                    .vcall(
                        program,
                        0x78,
                        &args![entry, matrices, bones, registers, indices],
                    )
                    .bool();
                return if set { result } else { NOT_SET };
            }
            37..=40 => texture_transform(e, slot_map(e, 0), 0x25),
            41..=44 => texture_transform(e, slot_map(e, 4), 0x29),
            45..=48 => texture_transform(e, slot_map(e, 8), 0x2d),
            49..=52 => texture_transform(e, slot_map(e, 0xc), 0x31),
            53..=56 => texture_transform(e, slot_map(e, 0x10), 0x35),
            57..=60 => texture_transform(e, slot_map(e, 0x14), 0x39),
            88..=91 => texture_transform(e, slot_map(e, 0x18), 0x58),
            92..=95 => texture_transform(e, slot_map(e, 0x1c), 0x5c),
            61..=64 => {
                let index = entry_index(e);
                let map = slot_map(e, index.wrapping_mul(4).wrapping_add(0x20));
                texture_transform(e, map, 0x3d)
            }
            65..=68 => {
                let texturing = texturing(e);
                let map = if texturing != 0 {
                    let index = entry_index(e);
                    e.call(MAP_AT_00A463E0, &args![texturing, index]).u32()
                } else {
                    0
                };
                texture_transform(e, map, 0x41)
            }
            73..=77 => {
                let source = e.mem.u32(properties.wrapping_add(8));
                if source == 0 {
                    return 0;
                }
                let at = |offset: u32| source.wrapping_add(offset);
                let values = match id {
                    73 | 74 => [
                        copy_float(e, VECTOR_011F4998),
                        copy_float(e, VECTOR_011F4998 + 4),
                        copy_float(e, VECTOR_011F4998 + 8),
                        copy_float(e, at(0x3c)),
                    ],
                    75 => [
                        copy_float(e, at(0x1c)),
                        copy_float(e, at(0x20)),
                        copy_float(e, at(0x24)),
                        copy_float(e, at(0x3c)),
                    ],
                    76 => {
                        let held = e.mem.u32(at(0x34));
                        let from = if held != 0 { held } else { at(0x28) };
                        [
                            copy_float(e, from),
                            copy_float(e, from.wrapping_add(4)),
                            copy_float(e, from.wrapping_add(8)),
                            copy_float(e, at(0x3c)),
                        ]
                    }
                    _ => [copy_float(e, at(0x38)); 4],
                };
                store_vector(e, vector, values);
                vector
            }
            78 | 79 => {
                let from = renderer(e).wrapping_add(0xa40);
                copy_matrix(e, from, first_matrix);
                let row = first_matrix + if id == 78 { 0x30 } else { 0x20 };
                let values = [0u32, 4, 8, 0xc].map(|offset| copy_float(e, row + offset));
                store_vector(e, vector, values);
                vector
            }
            80 | 81 => 0,
            82..=86 => {
                let handle = e.mem.u32(EXTRA_DATA_NAME_011BF65C);
                let held = e
                    .call(FIXED_STRING_CONSTRUCT_00438170, &args![name, handle])
                    .u32();
                let extra = e.call(GET_EXTRA_DATA_00A5BDD0, &args![object, held]).u32();
                e.call(FIXED_STRING_DESTRUCT_004381B0, &args![name]);
                if extra == 0 {
                    result = NO_EXTRA_DATA;
                }
                let time = copy_float(e, extra.wrapping_add(0xc));
                e.mem.set_u32(pair, time);
                let values = match id - 0x52 {
                    0 => [time; 4],
                    1 => [crt_on(e, CRT_HELPER_00ECA0A0, time); 4],
                    2 => [crt_on(e, CRT_HELPER_00EC9F70, time); 4],
                    3 => [crt_on(e, CRT_HELPER_00ECA5D0, time); 4],
                    4 => [
                        time,
                        crt_on(e, CRT_HELPER_00ECA0A0, time),
                        crt_on(e, CRT_HELPER_00EC9F70, time),
                        crt_on(e, CRT_HELPER_00ECA5D0, time),
                    ],
                    // The code's inner switch answers 0x10 for any other value (82 to 86 never
                    // reach it).
                    _ => return NO_EXTRA_DATA,
                };
                store_vector(e, vector, values);
                vector
            }
            87 => {
                store_vector(e, vector, [0, 0, 0, ONE]);
                let mut node = if effects != 0 {
                    e.mem.u32(effects.wrapping_add(0xc))
                } else {
                    0
                };
                while node != 0 {
                    let light = e.mem.u32(node.wrapping_add(4));
                    node = e.mem.u32(node);
                    if light == 0 {
                        continue;
                    }
                    let found = e
                        .call(LIGHT_CAST_00653290, &args![light, LIGHT_RECORD_011F5F6C])
                        .u32();
                    if found == 0 {
                        continue;
                    }
                    let strength = copy_float(e, found.wrapping_add(0xc4));
                    e.call(
                        LIGHT_OFFSET_00633B40,
                        &args![found.wrapping_add(0xc8), single, strength],
                    );
                    for component in 0..3u32 {
                        let at = vector + 4 * component;
                        let sum = x87_add(
                            x87_load32(e.mem.u32(at)),
                            x87_m32(e.mem.u32(single + 4 * component)),
                        );
                        e.mem.set_u32(at, x87_store32(sum));
                    }
                }
                vector
            }
            96..=99 => {
                if fourth == 0 {
                    return 0;
                }
                for word in 0..16u32 {
                    let value = if word % 5 == 0 { ONE } else { 0 };
                    e.mem.set_u32(first_matrix + 4 * word, value);
                }
                let (inverse, transpose) = flags(0x60);
                invert_transpose(e, first_matrix, inverse, transpose);
                first_matrix
            }
            103 => {
                store_vector(e, vector, [ONE; 4]);
                let map = slot_map(e, 0x14);
                if map != 0 {
                    let values = [0x18u32, 0x1c, 0x20, 0x24]
                        .map(|offset| copy_float(e, map.wrapping_add(offset)));
                    store_vector(e, vector, values);
                }
                vector
            }
            104 => {
                e.mem.set_u32(single, 0);
                e.mem.set_u32(single + 4, ONE);
                let map = slot_map(e, 0x14);
                if map != 0 {
                    let first = copy_float(e, map.wrapping_add(0x14));
                    e.mem.set_u32(single, first);
                    let second = copy_float(e, map.wrapping_add(0x10));
                    e.mem.set_u32(single + 4, second);
                }
                single
            }
            105 => {
                e.mem.set_u32(single, 0);
                let map = slot_map(e, 0x1c);
                if map != 0 {
                    let value = copy_float(e, map.wrapping_add(0x10));
                    e.mem.set_u32(single, value);
                }
                single
            }
            106..=115 => {
                let texturing = texturing(e);
                if texturing == 0 {
                    return 0;
                }
                let map = match id - 0x6a {
                    slot @ 0..=7 => map_word(e, texturing, 4 * slot),
                    8 => {
                        let index = entry_index(e);
                        map_word(e, texturing, index.wrapping_mul(4).wrapping_add(0x20))
                    }
                    9 => {
                        let index = entry_index(e);
                        e.call(MAP_AT_00A463E0, &args![texturing, index]).u32()
                    }
                    // The inner switch's default leaves no map (106 to 115 never reach it).
                    _ => 0,
                };
                e.mem.set_u32(pair, 0);
                e.mem.set_u32(pair + 4, 0);
                if map != 0 {
                    let texture = e.mem.u32(map.wrapping_add(8));
                    if texture != 0 {
                        for (slot, at) in [(0x94u32, pair), (0x98, pair + 4)] {
                            let size = e.vcall(texture, slot, &args![]).u32();
                            e.mem.set_u32(single, size);
                            let value = unsigned_to_float(e, size);
                            e.mem.set_u32(at, value);
                        }
                    }
                }
                pair
            }
            _ => return 0,
        };
        if !e.vcall(program, 0x74, &args![entry, data, 0u32]).bool() {
            result = NOT_SET;
        }
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_1d00, fn_00e81d00(Ptr, Ptr)),
        entry!(
            0x00e8_1ba0,
            ni_shader_constant_map_entry_ni_shader_constant_map_entry(Ptr) -> Ptr
        ),
        entry!(0x00e8_1c00, fn_00e81c00(Ptr)),
        entry!(
            0x00e8_3630,
            ni_shader_constant_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_6f40, ni_t_array_set_at(Ptr, u32, Ptr)),
        entry!(0x00e8_7510, fn_00e87510(Ptr, u32)),
        entry!(0x00e8_7800, ni_t_array_add_first_empty(Ptr, Ptr) -> u32),
        entry!(
            0x00e8_78c0,
            ni_d3d_shader_constant_map_insert_entry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_7920,
            ni_d3d_shader_constant_map_insert_defined_entry(Ptr, Ptr) -> u32
        ),
        entry!(0x00e8_70c0, fn_00e870c0(Ptr, Ptr)),
        entry!(0x00e8_7200, ni_d3d_shader_constant_map_sort_by_enabled(Ptr)),
        entry!(
            0x00e8_2640,
            ni_d3d_shader_constant_map_get_entry(Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_1d70,
            ni_d3d_shader_constant_map_add_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_1f10,
            ni_d3d_shader_constant_map_add_predefined_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, bool
            ) -> u32
        ),
        entry!(
            0x00e8_3650,
            ni_d3d_shader_constant_map_add_constant_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, bool
            ) -> u32
        ),
        entry!(
            0x00e8_29b0,
            ni_d3d_shader_constant_map_setup_predefined_entry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_26d0,
            ni_d3d_shader_constant_map_set_shader_constants(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(0x00e8_7690, fn_00e87690(Ptr)),
        entry!(
            0x00e8_3db0,
            ni_d3d_shader_constant_map_set_defined_constant(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e00_0000;
    const ADD_REFERENCE: u32 = 0x0e01_0004;
    const RELEASE: u32 = 0x0e01_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e81d00_swaps_the_device_and_takes_the_render_state() {
        let mut e = Engine::new();
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE, RELEASE]);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let old = e.mem.alloc(4);
        let device = e.mem.alloc(4);
        e.mem.set_u32(old, DEVICE_VTABLE);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x0e02_0000);
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map + 0x38, old);
        e.call(0x00e8_1d00, &args![map, renderer]);
        assert_eq!(e.mem.u32(map + 0x3c), renderer);
        assert_eq!(e.mem.u32(map + 0x38), device);
        assert_eq!(e.mem.u32(map + 0x40), 0x0e02_0000);
        assert_eq!(calls(&e, RELEASE), vec![vec![old]]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![device]]);
        // No renderer: the device released and both words cleared.
        e.call(0x00e8_1d00, &args![map, 0u32]);
        assert_eq!(calls(&e, RELEASE), vec![vec![old], vec![device]]);
        assert_eq!(e.mem.u32(map + 0x38), 0);
        assert_eq!(e.mem.u32(map + 0x40), 0);
        assert_eq!(e.mem.u32(map + 0x3c), 0);
    }
}

#[cfg(test)]
mod b054_tests {
    use super::*;

    /// A vtable whose delete slot (+4) is a recording double, and the slot.
    const OBJECT_VTABLE: u32 = 0x0e30_0000;
    const OBJECT_DELETE: u32 = 0x0e31_0004;
    /// The map's own vtable for the tests (slots by offset at `MAP_SLOT + offset`).
    const MAP_TEST_VTABLE: u32 = 0x0e32_0000;
    const MAP_SLOT: u32 = 0x0e33_0000;
    /// The device's vtable (slots at `DEVICE_SLOT + offset`).
    const DEVICE_VTABLE: u32 = 0x0e34_0000;
    const DEVICE_SLOT: u32 = 0x0e35_0000;
    /// The null fixed-string handle the tests map at `0109b220`.
    const NULL_HANDLE: u32 = 0x0e36_0008;
    /// The delete slot (0) of the entry vtable in the tests.
    const ENTRY_DELETE: u32 = 0x0e37_0000;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(OBJECT_DELETE, |_, _| Ret::default());
        e.register(ENTRY_DELETE, |_, _| Ret::default());
        e.put_vtable(OBJECT_VTABLE, &[0, OBJECT_DELETE]);
        e.put_vtable(ENTRY_VTABLE_010EF4D8, &[ENTRY_DELETE, OBJECT_DELETE]);
        let map_slots: Vec<u32> = (0..0x2c).map(|slot| MAP_SLOT + 4 * slot).collect();
        e.put_vtable(MAP_TEST_VTABLE, &map_slots);
        let device_slots: Vec<u32> = (0..0x70).map(|slot| DEVICE_SLOT + 4 * slot).collect();
        e.put_vtable(DEVICE_VTABLE, &device_slots);
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_NULL_0109B220, NULL_HANDLE);
        e.map(0x011f_4000, 0x2000);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// A reference-counted object with the given count.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// An entry-like object (count 1) with `m_bEnabled`, register, flags and extra word.
    fn entry(e: &mut Engine, enabled: bool, register: u32, flags: u32, extra: u32) -> u32 {
        let entry = object(e, 1);
        e.mem.set_u8(entry + 8, u8::from(enabled));
        e.mem.set_u32(entry + 0x1c, register);
        e.mem.set_u32(entry + 0x14, flags);
        e.mem.set_u32(entry + 0x18, extra);
        entry
    }

    /// An array (0x10 bytes) over the given elements, capacity `max`, grow-by 2.
    fn array(e: &mut Engine, elements: &[u32], max: u16) -> u32 {
        let array = e.mem.alloc(0x10);
        let block = e.mem.alloc(4 * u32::from(max).max(1));
        for (index, element) in elements.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *element);
        }
        e.mem.set_u32(array + 4, block);
        e.mem.set_u16(array + 8, max);
        e.mem.set_u16(array + 0xa, elements.len() as u16);
        let used = elements.iter().filter(|x| **x != 0).count() as u16;
        e.mem.set_u16(array + 0xc, used);
        e.mem.set_u16(array + 0xe, 2);
        array
    }

    fn elements(e: &Engine, array: u32) -> Vec<u32> {
        let size = u32::from(e.mem.u16(array + 0xa));
        let block = e.mem.u32(array + 4);
        (0..size)
            .map(|index| e.mem.u32(block + 4 * index))
            .collect()
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A map (0x44 bytes) with the test vtable and its two arrays (capacity 0x10 each).
    fn map(e: &mut Engine, entries: &[u32], defined: &[u32]) -> u32 {
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map, MAP_TEST_VTABLE);
        let entries = array(e, entries, 0x10);
        let defined = array(e, defined, 0x10);
        let bytes = e.mem.bytes(entries, 0x10);
        e.mem.write(map + 0xc, &bytes);
        let bytes = e.mem.bytes(defined, 0x10);
        e.mem.write(map + 0x1c, &bytes);
        map
    }

    #[test]
    fn entry_constructor_sets_up_the_entry() {
        let mut e = rig();
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xcc; 0x38]);
        let answer = e.call(0x00e8_1ba0, &args![this]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), ENTRY_VTABLE_010EF4D8);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), NULL_HANDLE);
        assert_eq!(e.mem.u32(this + 0x10), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 0x14, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(this + 0x24), NULL_HANDLE);
        assert_eq!(e.mem.bytes(this + 0x28, 0xf), vec![0; 0xf]);
        assert_eq!(e.mem.u8(this + 0x37), 0xcc);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 1);
    }

    #[test]
    fn entry_destructor_frees_owned_data_and_releases_the_names() {
        let mut e = rig();
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.set_global(NI_REF_OBJECT_COUNT_011F4400, 3u32);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 2);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0xc, NULL_HANDLE);
        e.mem.set_u32(this + 0x24, name);
        e.mem.set_u32(this + 0x30, 0x0e40_0000);
        e.mem.set_u8(this + 0x34, 1);
        e.call(0x00e8_1c00, &args![this]);
        assert_eq!(calls(&e, NI_FREE_DATA_00AA10F0), vec![vec![0x0e40_0000]]);
        assert_eq!(e.mem.u32(name - 8), 1);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 2);
        // Without owned data nothing is freed.
        e.mem.set_u8(this + 0x34, 0);
        e.call(0x00e8_1c00, &args![this]);
        assert_eq!(calls(&e, NI_FREE_DATA_00AA10F0).len(), 1);
    }

    #[test]
    fn entry_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0xc, NULL_HANDLE);
        e.mem.set_u32(this + 0x24, NULL_HANDLE);
        assert_eq!(e.call(0x00e8_3630, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.call(0x00e8_3630, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x38]]);
    }

    #[test]
    fn set_at_counts_and_assigns() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let old = object(&mut e, 1);
        let new = object(&mut e, 0);
        let array = array(&mut e, &[old, 0], 4);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, new);
        // Below the size: a null slot made non-null raises the element count.
        e.call(0x00e8_6f40, &args![array, 1u32, element]);
        assert_eq!(elements(&e, array), vec![old, new]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(e.mem.u32(new + 4), 1);
        // A non-null slot made null lowers it and releases the old object.
        e.mem.set_u32(element, 0);
        e.call(0x00e8_6f40, &args![array, 0u32, element]);
        assert_eq!(elements(&e, array), vec![0, new]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![old]]);
        // Past the size: the size follows the index.
        e.mem.set_u32(element, old);
        e.call(0x00e8_6f40, &args![array, 3u32, element]);
        assert_eq!(e.mem.u16(array + 0xa), 4);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(elements(&e, array)[3], old);
        // The null element is set up once.
        assert_eq!(
            calls(&e, ATEXIT),
            vec![vec![NULL_ELEMENT_DESTRUCT_00FDE390]]
        );
        assert_eq!(e.global::<u32>(NULL_ELEMENT_GUARD_0126F914) & 1, 1);
    }

    #[test]
    fn set_size_shrinks_and_moves_the_elements() {
        let mut e = rig();
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(NEW_POINTER_ARRAY_00B79A60, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        let (a, b, c) = (object(&mut e, 1), object(&mut e, 1), object(&mut e, 1));
        let array = array(&mut e, &[a, b, c], 4);
        let old = e.mem.u32(array + 4);
        // The same capacity does nothing.
        e.call(0x00e8_7510, &args![array, 4u32]);
        assert!(calls(&e, DELETE_POINTER_ARRAY_00B7CE30).is_empty());
        e.call(0x00e8_7510, &args![array, 2u32]);
        assert_eq!(e.mem.u16(array + 8), 2);
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![c]]);
        assert_eq!(elements(&e, array), vec![a, b]);
        assert_ne!(e.mem.u32(array + 4), old);
        // The moved elements are referenced again; the old block is not released.
        assert_eq!(e.mem.u32(a + 4), 2);
        assert_eq!(calls(&e, NEW_POINTER_ARRAY_00B79A60), vec![vec![2]]);
        assert_eq!(calls(&e, DELETE_POINTER_ARRAY_00B7CE30), vec![vec![old]]);
        // Capacity 0 leaves no elements.
        e.call(0x00e8_7510, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 0xa), 0);
    }

    #[test]
    fn add_first_empty_fills_a_hole_or_appends() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(NEW_POINTER_ARRAY_00B79A60, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        let (a, b) = (object(&mut e, 1), object(&mut e, 0));
        let array = array(&mut e, &[a, 0], 2);
        let element = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00e8_7800, &args![array, element]).u32(),
            0xffff_ffff
        );
        e.mem.set_u32(element, b);
        assert_eq!(e.call(0x00e8_7800, &args![array, element]).u32(), 1);
        assert_eq!(elements(&e, array), vec![a, b]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        // Full: grows by the grow-by count (2) and appends at the size.
        assert_eq!(e.call(0x00e8_7800, &args![array, element]).u32(), 2);
        assert_eq!(calls(&e, NEW_POINTER_ARRAY_00B79A60), vec![vec![4]]);
        assert_eq!(elements(&e, array), vec![a, b, b]);
        assert_eq!(e.mem.u16(array + 0xc), 3);
    }

    #[test]
    fn insert_entry_reports_a_failed_add() {
        let mut e = rig();
        let entry = object(&mut e, 1);
        let map = map(&mut e, &[0], &[]);
        e.mem.set_u32(map + 0x34, 7);
        assert_eq!(e.call(0x00e8_78c0, &args![map, entry]).u32(), 7);
        let block = e.mem.u32(map + 0x10);
        assert_eq!(e.mem.u32(block), entry);
        assert_eq!(e.mem.u32(entry + 4), 2);
        assert_eq!(e.call(0x00e8_78c0, &args![map, 0u32]).u32(), 0x30);
    }

    #[test]
    fn insert_defined_entry_adds_to_the_defined_array() {
        let mut e = rig();
        let entry = object(&mut e, 1);
        let map = map(&mut e, &[], &[0]);
        e.mem.set_u32(map + 0x34, 0);
        assert_eq!(e.call(0x00e8_7920, &args![map, entry]).u32(), 0);
        let block = e.mem.u32(map + 0x20);
        assert_eq!(e.mem.u32(block), entry);
        assert_eq!(e.call(0x00e8_7920, &args![map, 0u32]).u32(), 0x30);
    }

    #[test]
    fn sort_step_moves_enabled_entries_first() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let disabled_a = entry(&mut e, false, 0, 0, 0);
        let enabled_b = entry(&mut e, true, 0, 0, 0);
        let disabled_c = entry(&mut e, false, 0, 0, 0);
        let enabled_d = entry(&mut e, true, 0, 0, 0);
        let sorted = array(&mut e, &[disabled_a, enabled_b, disabled_c, enabled_d], 4);
        e.call(0x00e8_70c0, &args![0u32, sorted]);
        assert_eq!(
            elements(&e, sorted),
            vec![enabled_b, enabled_d, disabled_c, disabled_a]
        );
        for entry in [disabled_a, enabled_b, disabled_c, enabled_d] {
            assert_eq!(e.mem.u32(entry + 4), 1);
        }
        assert_eq!(e.mem.u16(sorted + 0xc), 4);
        // One element: nothing moves.
        let single = array(&mut e, &[disabled_a], 1);
        e.call(0x00e8_70c0, &args![0u32, single]);
        assert_eq!(elements(&e, single), vec![disabled_a]);
    }

    #[test]
    fn sort_by_enabled_sorts_both_arrays() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let disabled = entry(&mut e, false, 0, 0, 0);
        let enabled = entry(&mut e, true, 0, 0, 0);
        let map = map(&mut e, &[disabled, enabled], &[disabled, enabled]);
        e.call(0x00e8_7200, &args![map]);
        let entries = e.mem.u32(map + 0x10);
        let defined = e.mem.u32(map + 0x20);
        for block in [entries, defined] {
            assert_eq!(e.mem.u32(block), enabled);
            assert_eq!(e.mem.u32(block + 4), disabled);
        }
    }

    #[test]
    fn get_entry_looks_in_the_defined_entries_first() {
        let mut e = rig();
        e.register(NI_STRICMP, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let key = |e: &mut Engine, word: u32| {
            let text = e.mem.alloc(4);
            e.mem.set_u32(text, word);
            text
        };
        let (first, second, third) = (key(&mut e, 1), key(&mut e, 2), key(&mut e, 3));
        let plain = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(plain + 0xc, first);
        let defined = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(defined + 0xc, first);
        let other = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(other + 0xc, second);
        let map = map(&mut e, &[0, other, plain], &[defined]);
        assert_eq!(e.call(0x00e8_2640, &args![map, first]).u32(), defined);
        assert_eq!(e.call(0x00e8_2640, &args![map, second]).u32(), other);
        assert_eq!(e.call(0x00e8_2640, &args![map, third]).u32(), 0);
    }

    #[test]
    fn add_entry_dispatches_by_kind() {
        let mut e = rig();
        let found = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x70, move |e, _| e.mem.u32(found).into_ret());
        for slot in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x68] {
            e.register(MAP_SLOT + slot, |_, a| (a.len() as u32).into_ret());
        }
        e.map(0x011f_5000, 0x1000);
        e.set_global(OBJECT_TABLE_011F5F90 + 4 * 3, 0x0e50_0000u32);
        let map = map(&mut e, &[], &[]);
        let words = |flags: u32| {
            args![map, 0x11u32, flags, 0x13u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32]
        };
        // Each kind's adder answers its word count (with `this`).
        for (flags, slot, count) in [
            (0x3000_0000u32, 0x10u32, 11u32),
            (0x1000_0000, 0x14, 11),
            (0x4000_0000, 0x18, 11),
            (0x5000_0000, 0x1c, 7),
            (0x2000_0000, 0x68, 9),
            (0x6000_0300, 0x20, 6),
        ] {
            assert_eq!(e.call(0x00e8_1d70, &words(flags)).u32(), count);
            assert_eq!(e.mem.u32(map + 0x34), count);
            assert!(!calls(&e, MAP_SLOT + slot).is_empty());
        }
        assert_eq!(
            calls(&e, MAP_SLOT + 0x68)[0],
            vec![map, 0x11, 0x13, 4, 6, 0, 0, 0, 0]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x20)[0],
            vec![map, 0x11, 4, 6, 0x13, 0x0e50_0000]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x1c)[0],
            vec![map, 0x11, 0x5000_0000, 0x13, 4, 5, 6]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x10)[0],
            vec![map, 0x11, 0x3000_0000, 0x13, 4, 5, 6, 7, 8, 9, 10]
        );
        // Any other kind is error 0x40.
        assert_eq!(e.call(0x00e8_1d70, &words(0x7000_0000)).u32(), 0x40);
        // An entry with the same extra word is error 0x20.
        let existing = entry(&mut e, true, 0, 0, 0x13);
        e.mem.set_u32(found, existing);
        assert_eq!(e.call(0x00e8_1d70, &words(0x1000_0000)).u32(), 0x20);
        assert_eq!(e.mem.u32(map + 0x34), 0x20);
    }

    /// The doubles for the entry builders: the 0x38-byte allocation, the fixed-string maker
    /// (handle = a block + 8 over a count of 1, holding the text address), the data allocator,
    /// free and `memcpy`.
    fn builder_rig() -> Engine {
        let mut e = rig();
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MAKE_FIXED_STRING_00A5B690, |e, a| {
            let handle = e.mem.alloc(0x10) + 8;
            e.mem.set_u32(handle - 8, 1);
            e.mem.set_u32(handle, a[0]);
            handle.into_ret()
        });
        e.register(NI_ALLOCATE_DATA_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn add_predefined_entry_builds_and_inserts() {
        let mut e = builder_rig();
        let answer = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x88, move |e, _| e.mem.u32(answer).into_ret());
        e.register(MAP_SLOT + 0x84, |_, _| 0x55u32.into_ret());
        let map = map(&mut e, &[], &[]);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0xdead_beef);
        let result = e
            .call(
                0x00e8_1f10,
                &args![
                    map,
                    0x0e60_0000u32,
                    2u32,
                    3u32,
                    0u32,
                    8u32,
                    4u32,
                    source,
                    1u32
                ],
            )
            .u32();
        assert_eq!(result, 0x55);
        assert_eq!(e.mem.u32(map + 0x34), 0x55);
        let entry = calls(&e, MAP_SLOT + 0x84)[0][1];
        let key = e.mem.u32(entry + 0xc);
        assert_eq!(e.mem.u32(key), 0x0e60_0000);
        // The kept key holds one reference: the made one's was dropped.
        assert_eq!(e.mem.u32(key - 8), 1);
        assert_eq!(e.mem.u32(entry + 0x24), NULL_HANDLE);
        assert_eq!(e.mem.u32(entry + 0x14), 0x2000_0000);
        assert_eq!(e.mem.u32(entry + 0x18), 2);
        assert_eq!(e.mem.u32(entry + 0x1c), 3);
        assert_eq!(e.mem.u32(entry + 0x28), 8);
        assert_eq!(e.mem.u32(entry + 0x2c), 4);
        assert_eq!(e.mem.u8(entry + 0x34), 1);
        let copy = e.mem.u32(entry + 0x30);
        assert_ne!(copy, source);
        assert_eq!(e.mem.u32(copy), 0xdead_beef);
        assert!(calls(&e, ENTRY_DELETE).is_empty());
        // A failed set-up deletes the entry (slot 0 with 1) and answers the error.
        e.mem.set_u32(answer, 0x40);
        let result = e
            .call(
                0x00e8_1f10,
                &args![map, 0u32, 2u32, 3u32, 0u32, 8u32, 4u32, source, 0u32],
            )
            .u32();
        assert_eq!(result, 0x40);
        let entry = calls(&e, MAP_SLOT + 0x88)[1][1];
        assert_eq!(e.mem.u32(entry + 0xc), NULL_HANDLE);
        assert_eq!(e.mem.u32(entry + 0x30), source);
        assert_eq!(e.mem.u8(entry + 0x34), 0);
        assert_eq!(calls(&e, ENTRY_DELETE), vec![vec![entry, 1]]);
        assert_eq!(calls(&e, MAP_SLOT + 0x84).len(), 1);
    }

    #[test]
    fn add_constant_entry_picks_the_flags_and_extra_word() {
        let mut e = builder_rig();
        let answer = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x80, move |e, _| e.mem.u32(answer).into_ret());
        e.map(0x011f_5000, 0x1000);
        // Type 1 for flags byte 7: such entries keep the extra word.
        e.set_global(CONSTANT_TYPE_TABLE_011F5FC4 + 4 * 7, 1u32);
        let map = map(&mut e, &[], &[]);
        e.mem.set_u32(map + 8, 0);
        let build = |e: &mut Engine, size: u32, stride: u32| {
            e.call(
                0x00e8_3650,
                &args![
                    map,
                    0u32,
                    0xf000_0100u32,
                    9u32,
                    4u32,
                    2u32,
                    0u32,
                    size,
                    stride,
                    0x0e70_0000u32,
                    0u32
                ],
            )
            .u32()
        };
        let last = |e: &Engine| calls(e, MAP_SLOT + 0x80).last().unwrap()[1];
        assert_eq!(build(&mut e, 8, 4), 0);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0105);
        assert_eq!(e.mem.u32(entry + 0x18), 1);
        assert_eq!(e.mem.u32(entry + 0x1c), 4);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
        assert_eq!(e.mem.u32(entry + 0x28), 8);
        assert_eq!(e.mem.u32(entry + 0x2c), 4);
        assert_eq!(e.mem.u32(entry + 0x30), 0x0e70_0000);
        // Size 0x10 is type 7 (table 1): the extra word stays; so for stride 8.
        build(&mut e, 0x10, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0107);
        assert_eq!(e.mem.u32(entry + 0x18), 9);
        build(&mut e, 8, 8);
        assert_eq!(e.mem.u32(last(&e) + 0x18), 9);
        // Program type 2 gives 2; another program type keeps the extra word.
        e.mem.set_u32(map + 8, 2);
        build(&mut e, 0x40, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0109);
        assert_eq!(e.mem.u32(entry + 0x18), 2);
        e.mem.set_u32(map + 8, 1);
        build(&mut e, 0x24, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0108);
        assert_eq!(e.mem.u32(entry + 0x18), 9);
        build(&mut e, 0x14, 4);
        assert_eq!(e.mem.u32(last(&e) + 0x14), 0x1000_0100);
        // An insert error deletes the entry and is answered.
        e.mem.set_u32(answer, 0x30);
        assert_eq!(build(&mut e, 0xc, 4), 0x30);
        assert_eq!(e.mem.u32(map + 0x34), 0x30);
        assert_eq!(calls(&e, ENTRY_DELETE), vec![vec![last(&e), 1]]);
    }

    #[test]
    fn setup_predefined_entry_sets_the_mapping() {
        let mut e = rig();
        let lookup = e.mem.alloc(8);
        e.register_double(LOOK_UP_PREDEFINED_MAPPING, move |e, a| {
            let mapping = e.mem.u32(lookup);
            let count = e.mem.u32(lookup + 4);
            assert_eq!(e.mem.u32(a[2]), 4);
            e.mem.set_u32(a[1], mapping);
            e.mem.set_u32(a[2], count);
            u32::from(mapping != 0xffff).into_ret()
        });
        e.register(LOOK_UP_PREDEFINED_MAPPING_TYPE, |_, _| 2u32.into_ret());
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u8(PREDEFINED_TYPE_TABLE_011F5FB4 + 2, 0x0b);
        let entry = entry(&mut e, true, 0, 0x1234_5600, 0x0003_0000);
        let run = |e: &mut Engine, mapping: u32, count: u32| {
            e.mem.set_u32(lookup, mapping);
            e.mem.set_u32(lookup + 4, count);
            e.mem.set_u32(entry + 0x20, 0xaaaa);
            e.call(0x00e8_29b0, &args![0u32, entry]).u32()
        };
        assert_eq!(run(&mut e, 0xffff, 1), 0x40);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1234_5600);
        assert_eq!(run(&mut e, 0, 1), 0x40);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1234_560b);
        assert_eq!(run(&mut e, 0x21, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 9);
        assert_eq!(e.mem.u32(entry + 0x10), 0x21);
        assert_eq!(run(&mut e, 0x24, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 12);
        assert_eq!(run(&mut e, 0x1c, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 4);
        assert_eq!(run(&mut e, 0x30, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 7);
        assert_eq!(run(&mut e, 0x50, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 0xaaaa);
        assert_eq!(e.mem.u32(entry + 0x30), PREDEFINED_DATA_011BF63C);
        assert_eq!(e.mem.u32(entry + 0x28), 0x10);
        assert_eq!(e.mem.u32(entry + 0x2c), 0x10);
        assert_eq!(run(&mut e, 0x51, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x30), PREDEFINED_DATA_011BF64C);
    }

    #[test]
    fn set_shader_constants_sets_each_kind() {
        let mut e = rig();
        let failing = e.mem.alloc(4);
        for slot in [0x8cu32, 0x90, 0x94, 0x98, 0x9c, 0xa4] {
            e.register_double(MAP_SLOT + slot, move |e, a| {
                u32::from(a[2] == e.mem.u32(failing)).into_ret()
            });
        }
        e.register(DEVICE_SLOT + 0x178, |_, _| 0u32.into_ret());
        e.register(DEVICE_SLOT + 0x1b4, |_, _| 0x8000_0000u32.into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let defined = entry(&mut e, true, 1, 0x2000_0000, 0);
        let skipped = entry(&mut e, true, 0xffff_ffff, 0x1000_0000, 1);
        let disabled = entry(&mut e, false, 1, 0x1000_0000, 1);
        let vertex = entry(&mut e, true, 3, 0x1000_0000, 1);
        e.mem.set_u32(vertex + 0x30, 0x0e80_0000);
        e.mem.set_u32(vertex + 0x20, 2);
        let constant = entry(&mut e, true, 3, 0x1000_0000, 5);
        let attribute = entry(&mut e, true, 3, 0x3000_0000, 0);
        let global = entry(&mut e, true, 3, 0x4000_0000, 0);
        let operator = entry(&mut e, true, 3, 0x5000_0000, 0);
        let object_entry = entry(&mut e, true, 3, 0x6000_0000, 0);
        let predefined = entry(&mut e, true, 3, 0x2000_0000, 0);
        let map = map(
            &mut e,
            &[
                skipped,
                0,
                disabled,
                vertex,
                constant,
                attribute,
                global,
                operator,
                object_entry,
                predefined,
            ],
            &[defined],
        );
        e.mem.set_u32(map + 0x38, device);
        let words: Vec<u32> = (2..=11).collect();
        let mut all = vec![map, 0x0e90_0000];
        all.extend(&words);
        e.mem.set_u32(failing, 0xffff);
        assert!(!e.call(0x00e8_26d0, &all).bool());
        assert_eq!(e.mem.u32(map + 0x30), 0x0e90_0000);
        let mut expected = vec![map, 0x0e90_0000, defined];
        expected.extend(&words[..9]);
        assert_eq!(calls(&e, MAP_SLOT + 0x8c), vec![expected]);
        assert_eq!(
            calls(&e, DEVICE_SLOT + 0x178),
            vec![vec![device, 3, 0x0e80_0000, 2]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x90),
            vec![vec![map, 0x0e90_0000, constant, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x94)[0],
            vec![
                map,
                0x0e90_0000,
                attribute,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                0
            ]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x98),
            vec![vec![map, 0x0e90_0000, global, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x9c),
            vec![vec![map, 0x0e90_0000, operator, 2, 3, 6, 7, 8, 9, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0xa4)[0],
            vec![map, 0x0e90_0000, object_entry, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        // A failing setter, a negative device result or an unknown kind is reported.
        e.mem.set_u32(failing, global);
        assert!(e.call(0x00e8_26d0, &all).bool());
        e.mem.set_u32(failing, 0xffff);
        e.mem.set_u32(vertex + 0x18, 2);
        assert!(e.call(0x00e8_26d0, &all).bool());
        e.mem.set_u32(vertex + 0x18, 1);
        e.mem.set_u32(global + 0x14, 0x7000_0000);
        assert!(e.call(0x00e8_26d0, &all).bool());
    }

    #[test]
    fn map_destructor_releases_globals_and_the_device() {
        let mut e = rig();
        e.register(RELEASE_GLOBAL_SHADER_CONSTANT, |_, _| Ret::default());
        e.register(ARRAY_REMOVE_ALL_00B799C0, |_, _| Ret::default());
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(DEVICE_SLOT + 8, |_, _| Ret::default());
        e.set_global(NI_REF_OBJECT_COUNT_011F4400, 5u32);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        let global = entry(&mut e, true, 0, 0x4000_0000, 0);
        e.mem.set_u32(global + 0xc, name);
        let plain = entry(&mut e, true, 0, 0x1000_0000, 0);
        let defined_global = entry(&mut e, true, 0, 0x4000_0000, 0);
        let map = map(&mut e, &[global, 0, plain], &[defined_global]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(map + 0x38, device);
        e.mem.set_u32(map + 0x3c, 0x1234);
        e.mem.set_u32(map + 0x40, 0x5678);
        let entries = e.mem.u32(map + 0x10);
        let defined = e.mem.u32(map + 0x20);
        e.call(0x00e8_7690, &args![map]);
        let released = calls(&e, RELEASE_GLOBAL_SHADER_CONSTANT);
        assert_eq!(released.len(), 2);
        assert_eq!(released[1], vec![defined_global + 0xc]);
        // The handle was held across the call and dropped again.
        assert_eq!(e.mem.u32(name - 8), 1);
        for entry in [global, plain, defined_global] {
            assert_eq!(e.mem.u32(entry + 4), 1);
        }
        assert_eq!(
            calls(&e, ARRAY_REMOVE_ALL_00B799C0),
            vec![vec![map + 0xc], vec![map + 0x1c]]
        );
        assert_eq!(calls(&e, DEVICE_SLOT + 8), vec![vec![device]]);
        for offset in [0x38u32, 0x3c, 0x40] {
            assert_eq!(e.mem.u32(map + offset), 0);
        }
        assert_eq!(
            calls(&e, DELETE_POINTER_ARRAY_00B7CE30),
            vec![vec![defined], vec![entries]]
        );
        assert_eq!(e.mem.u32(map + 0xc), ENTRY_ARRAY_VTABLE_010EF53C);
        assert_eq!(e.mem.u32(map + 0x1c), ENTRY_ARRAY_VTABLE_010EF53C);
        assert_eq!(e.mem.u32(map), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 4);
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const PROGRAM_VTABLE: u32 = 0x0e00_0000;
    const SET_CONSTANT: u32 = 0x0e01_0074;
    const SET_CONSTANT_ARRAY: u32 = 0x0e01_0078;
    const TEXTURE_VTABLE: u32 = 0x0e02_0000;
    const TEXTURE_WIDTH: u32 = 0x0e03_0094;
    const TEXTURE_HEIGHT: u32 = 0x0e03_0098;

    /// What the program's `SetShaderConstant` saw: the data address and its first 16 words.
    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        map: u32,
        program: u32,
        entry: u32,
        renderer: u32,
        properties: u32,
        seen: Seen,
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

    /// A constant map whose renderer holds the words `0x9400 + i` at +0x940, `0x9800 + i` at
    /// +0x980, `0x9c00 + i` at +0x9c0, `0xa000 + i` at +0xa00 and `0xa400 + i` at +0xa40; a
    /// program that accepts (`accepts`) every constant.
    fn rig(accepts: bool) -> Rig {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x2000);
        e.map(0x0109_9000, 0x1000);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        let mut slots = vec![0u32; 0x1f];
        slots[0x74 / 4] = SET_CONSTANT;
        slots[0x78 / 4] = SET_CONSTANT_ARRAY;
        e.put_vtable(PROGRAM_VTABLE, &slots);
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(SET_CONSTANT, move |e, a| {
            let words = if a[2] != 0 {
                (0..16).map(|i| e.mem.u32(a[2] + 4 * i)).collect()
            } else {
                vec![]
            };
            log.borrow_mut().push((a[2], words));
            u32::from(accepts).into_ret()
        });
        e.register_double(SET_CONSTANT_ARRAY, move |_, _| {
            u32::from(accepts).into_ret()
        });
        for callee in [
            MATRIX_INVERSE_00EE6DDA,
            MATRIX_TRANSPOSE_00EE6DD4,
            MATRIX_MULTIPLY_00EE6DE0,
            MATRIX_PRODUCT_00EE6DFE,
            SETUP_TEXTURE_TRANSFORM_00E83550,
            FIXED_STRING_DESTRUCT_004381B0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let renderer = e.mem.alloc(0xa80);
        for (offset, tag) in [
            (0x940u32, 0x9400u32),
            (0x980, 0x9800),
            (0x9c0, 0x9c00),
            (0xa00, 0xa000),
            (0xa40, 0xa400),
        ] {
            for i in 0..16 {
                e.mem.set_u32(renderer + offset + 4 * i, tag + i);
            }
        }
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map + 0x3c, renderer);
        let program = e.mem.alloc(4);
        e.mem.set_u32(program, PROGRAM_VTABLE);
        let entry = e.mem.alloc(0x38);
        let properties = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        Rig {
            e,
            map,
            program,
            entry,
            renderer,
            properties,
            seen,
        }
    }

    impl Rig {
        /// The function for constant `id` with the fourth, fifth and effect words.
        fn run(&mut self, id: u32, fourth: u32, fifth: u32, effects: u32) -> u32 {
            self.e.mem.set_u32(self.entry + 0x10, id);
            self.seen.borrow_mut().clear();
            self.e.call_log = Some(vec![]);
            let call = args![
                self.map,
                self.program,
                self.entry,
                0x0e40_0000u32,
                fourth,
                fifth,
                0u32,
                self.properties,
                effects,
                0u32,
                0u32,
                0u32
            ];
            self.e.call(0x00e8_3db0, &call).u32()
        }

        fn seen(&self) -> Vec<(u32, Vec<u32>)> {
            self.seen.borrow().clone()
        }

        fn tags(&self, tag: u32) -> Vec<u32> {
            (0..16).map(|i| tag + i).collect()
        }

        fn floats(&self, count: usize) -> Vec<f32> {
            self.seen()[0].1[..count]
                .iter()
                .map(|w| f32::from_bits(*w))
                .collect()
        }
    }

    #[test]
    fn renderer_matrices_are_copied_inverted_and_transposed() {
        let mut r = rig(true);
        assert_eq!(r.run(1, 0, 0, 0), 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9c00));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        r.run(14, 0, 0, 0);
        let data = r.seen()[0].0;
        assert_eq!(
            calls(&r.e, MATRIX_INVERSE_00EE6DDA),
            vec![vec![data, 0, data]]
        );
        assert_eq!(
            calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4),
            vec![vec![data, data]]
        );
        r.run(16, 0, 0, 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9800));
        r.run(17, 0, 0, 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9400));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        // The program refusing: 0x50.
        let mut r = rig(false);
        assert_eq!(r.run(2, 0, 0, 0), NOT_SET);
    }

    #[test]
    fn products_multiply_the_renderer_matrices_in_order() {
        let mut r = rig(true);
        let factors: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = factors.clone();
        r.e.register_double(MATRIX_MULTIPLY_00EE6DE0, move |e, a| {
            log.borrow_mut().push((e.mem.u32(a[1]), e.mem.u32(a[2])));
            Ret::default()
        });
        r.run(8, 0, 0, 0);
        assert_eq!(factors.borrow()[0], (0x9400, 0x9800));
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        r.run(21, 0, 0, 0);
        assert_eq!(factors.borrow()[1], (0x9800, 0x9c00));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        let multiplied = calls(&r.e, MATRIX_MULTIPLY_00EE6DE0)[0][0];
        assert_eq!(r.seen()[0].0, multiplied);
        // 11 to 24: 00ee6dfe of +0x940 and +0xa00, handed over as it is.
        r.run(12, 0, 0, 0);
        let product = &calls(&r.e, MATRIX_PRODUCT_00EE6DFE)[0];
        assert_eq!(product[1..], [r.renderer + 0x940, r.renderer + 0xa00]);
        assert_eq!(r.seen()[0].0, product[0]);
    }

    #[test]
    fn constants_needing_the_fourth_word() {
        let mut r = rig(true);
        for id in [25u32, 30, 96] {
            assert_eq!(r.run(id, 0, 0, 0), 0);
            assert!(r.seen().is_empty());
        }
        r.run(28, 1, 0, 0);
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        r.run(97, 1, 0, 0);
        let one = 1.0f32.to_bits();
        let identity: Vec<u32> = (0..16).map(|i| if i % 5 == 0 { one } else { 0 }).collect();
        assert_eq!(r.seen()[0].1, identity);
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
    }

    #[test]
    fn skinning_constants_go_to_the_array_slot() {
        let mut r = rig(true);
        let fourth = r.e.mem.alloc(0x2c);
        r.e.mem.set_u32(fourth + 0x20, 8);
        r.e.mem.set_u32(fourth + 0x28, 0x0e50_0000);
        let fifth = r.e.mem.alloc(0x24);
        r.e.mem.set_u32(fifth + 4, 0x0e51_0000);
        r.e.mem.set_u16(fifth + 0x20, 3);
        assert_eq!(r.run(33, fourth, 0, 0), 0);
        assert!(calls(&r.e, SET_CONSTANT_ARRAY).is_empty());
        assert_eq!(r.run(36, fourth, fifth, 0), 0);
        assert_eq!(
            calls(&r.e, SET_CONSTANT_ARRAY),
            vec![vec![r.program, r.entry, 0x0e50_0000, 3, 8, 0x0e51_0000]]
        );
        let mut r2 = rig(false);
        let fourth = r2.e.mem.alloc(0x2c);
        let fifth = r2.e.mem.alloc(0x24);
        assert_eq!(r2.run(35, fourth, fifth, 0), NOT_SET);
    }

    /// A texturing property with map arrays: the map at `slot` holds a transform.
    fn texturing(r: &mut Rig, slot: u32) -> u32 {
        let texturing = r.e.mem.alloc(0x24);
        let maps = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(texturing + 0x20, maps);
        let map = r.e.mem.alloc(0x14);
        r.e.mem.set_u32(map + 0xc, 0x0e60_0000);
        r.e.mem.set_u32(maps + slot, map);
        r.e.mem.set_u32(r.properties + 0x14, texturing);
        map
    }

    #[test]
    fn texture_transforms_set_up_the_map_matrix() {
        let mut r = rig(true);
        r.e.register(UPDATE_MATRIX_00BC5C20, |_, _| 0x0e61_0000u32.into_ret());
        // No texturing property: no matrix, no flags.
        r.run(40, 0, 0, 0);
        let setup = &calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0];
        assert_eq!(setup[1..], [0, 0, 0, 4]);
        assert_eq!(r.seen()[0].0, setup[0]);
        texturing(&mut r, 4);
        r.run(42, 0, 0, 0);
        assert_eq!(calls(&r.e, UPDATE_MATRIX_00BC5C20), vec![vec![0x0e60_0000]]);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 0, 4]
        );
        r.run(44, 0, 0, 0);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 1, 4]
        );
        // Map 6 for 88 to 91; the entry's index for 61 to 64.
        texturing(&mut r, 0x18);
        r.run(90, 0, 0, 0);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 0, 1, 4]
        );
        r.e.mem.set_u32(r.entry + 0x18, 2);
        texturing(&mut r, 0x28);
        r.run(62, 0, 0, 0);
        assert_eq!(calls(&r.e, UPDATE_MATRIX_00BC5C20).len(), 1);
        // 65 to 68: the map 00a463e0 answers for the index.
        let map = texturing(&mut r, 0x3c);
        r.e.register_double(MAP_AT_00A463E0, move |_, _| map.into_ret());
        r.run(68, 0, 0, 0);
        let texturing = r.e.mem.u32(r.properties + 0x14);
        assert_eq!(calls(&r.e, MAP_AT_00A463E0), vec![vec![texturing, 2]]);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 1, 4]
        );
    }

    #[test]
    fn property_vectors_and_renderer_rows() {
        let mut r = rig(true);
        assert_eq!(r.run(75, 0, 0, 0), 0);
        assert!(r.seen().is_empty());
        let source = r.e.mem.alloc(0x40);
        for i in 0..16u32 {
            r.e.mem.set_u32(source + 4 * i, (i as f32).to_bits());
        }
        r.e.mem.set_u32(r.properties + 8, source);
        for i in 0..3u32 {
            r.e.mem
                .set_u32(VECTOR_011F4998 + 4 * i, (100.0 + i as f32).to_bits());
        }
        r.run(74, 0, 0, 0);
        assert_eq!(r.floats(4), vec![100.0, 101.0, 102.0, 15.0]);
        r.run(75, 0, 0, 0);
        assert_eq!(r.floats(4), vec![7.0, 8.0, 9.0, 15.0]);
        // 76 without a word at +0x34: the floats from +0x28 on.
        r.e.mem.set_u32(source + 0x34, 0);
        r.run(76, 0, 0, 0);
        assert_eq!(r.floats(4), vec![10.0, 11.0, 12.0, 15.0]);
        let held = r.e.mem.alloc(0xc);
        r.e.mem.set_u32(held, 0x7f80_0001);
        r.e.mem.set_u32(source + 0x34, held);
        r.run(76, 0, 0, 0);
        // The copy through FLD/FSTP quiets the signalling NaN.
        assert_eq!(r.seen()[0].1[0], 0x7fc0_0001);
        r.run(77, 0, 0, 0);
        assert_eq!(r.floats(4), vec![14.0; 4]);
        r.run(78, 0, 0, 0);
        assert_eq!(r.seen()[0].1[..4], [0xa40c, 0xa40d, 0xa40e, 0xa40f]);
        r.run(79, 0, 0, 0);
        assert_eq!(r.seen()[0].1[..4], [0xa408, 0xa409, 0xa40a, 0xa40b]);
        r.run(81, 0, 0, 0);
        assert_eq!(r.seen()[0].0, 0);
    }

    #[test]
    fn time_constants_read_the_extra_data() {
        let mut r = rig(true);
        r.e.mem.set_u32(EXTRA_DATA_NAME_011BF65C, 0x0e70_0000);
        r.e.register(FIXED_STRING_CONSTRUCT_00438170, |_, a| a[0].into_ret());
        let extra = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(extra + 0xc, 2.0f32.to_bits());
        r.e.register_double(GET_EXTRA_DATA_00A5BDD0, move |_, _| extra.into_ret());
        r.e.register(CRT_HELPER_00ECA0A0, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) + 1.0,
            ..Ret::default()
        });
        r.e.register(CRT_HELPER_00EC9F70, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        r.e.register(CRT_HELPER_00ECA5D0, |_, _| Ret {
            st0: -4.0,
            ..Ret::default()
        });
        assert_eq!(r.run(82, 0, 0, 0), 0);
        assert_eq!(r.floats(4), vec![2.0; 4]);
        let name = calls(&r.e, FIXED_STRING_CONSTRUCT_00438170)[0].clone();
        assert_eq!(name[1], 0x0e70_0000);
        assert_eq!(
            calls(&r.e, GET_EXTRA_DATA_00A5BDD0),
            vec![vec![0x0e40_0000, name[0]]]
        );
        assert_eq!(
            calls(&r.e, FIXED_STRING_DESTRUCT_004381B0),
            vec![vec![name[0]]]
        );
        r.run(83, 0, 0, 0);
        assert_eq!(r.floats(4), vec![3.0; 4]);
        r.run(86, 0, 0, 0);
        assert_eq!(r.floats(4), vec![2.0, 3.0, 0.5, -4.0]);
        // No extra data: the result 0x10, the time read at address 0xc.
        r.e.map(0, 0x1000);
        r.e.mem.set_u32(0xc, 5.0f32.to_bits());
        r.e.register(GET_EXTRA_DATA_00A5BDD0, |_, _| 0u32.into_ret());
        assert_eq!(r.run(84, 0, 0, 0), NO_EXTRA_DATA);
        assert_eq!(r.floats(4), vec![0.5; 4]);
    }

    #[test]
    fn light_constant_sums_the_lights() {
        let mut r = rig(true);
        let light = r.e.mem.alloc(0xd0);
        r.e.mem.set_u32(light + 0xc4, 0.25f32.to_bits());
        let second = r.e.mem.alloc(4);
        r.e.register_double(LIGHT_CAST_00653290, move |_, a| {
            if a[0] == second {
                0u32.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        r.e.register(LIGHT_OFFSET_00633B40, |e, a| {
            assert_eq!(f32::from_bits(a[2]), 0.25);
            for i in 0..3u32 {
                e.mem.set_u32(a[1] + 4 * i, (i as f32 + 1.0).to_bits());
            }
            Ret::default()
        });
        let effects = r.e.mem.alloc(0x10);
        let first_node = r.e.mem.alloc(8);
        let second_node = r.e.mem.alloc(8);
        let third_node = r.e.mem.alloc(8);
        r.e.mem.set_u32(effects + 0xc, first_node);
        r.e.mem.set_u32(first_node, second_node);
        r.e.mem.set_u32(first_node + 4, light);
        r.e.mem.set_u32(second_node, third_node);
        r.e.mem.set_u32(second_node + 4, second);
        r.e.mem.set_u32(third_node + 4, light);
        r.run(87, 0, 0, effects);
        assert_eq!(r.floats(4), vec![2.0, 4.0, 6.0, 1.0]);
        assert_eq!(calls(&r.e, LIGHT_OFFSET_00633B40)[0][0], light + 0xc8);
        r.run(87, 0, 0, 0);
        assert_eq!(r.floats(4), vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn map_values_and_texture_sizes() {
        let mut r = rig(true);
        r.run(103, 0, 0, 0);
        assert_eq!(r.floats(4), vec![1.0; 4]);
        r.run(104, 0, 0, 0);
        assert_eq!(r.floats(2), vec![0.0, 1.0]);
        r.run(105, 0, 0, 0);
        assert_eq!(r.floats(1), vec![0.0]);
        assert_eq!(r.run(106, 0, 0, 0), 0);
        assert!(r.seen().is_empty());
        let fifth_map = texturing(&mut r, 0x14);
        let seventh_map = r.e.mem.alloc(0x14);
        let maps = r.e.mem.u32(r.e.mem.u32(r.properties + 0x14) + 0x20);
        r.e.mem.set_u32(maps + 0x1c, seventh_map);
        let fifth = r.e.mem.alloc(0x28);
        for i in 0..10u32 {
            r.e.mem.set_u32(fifth + 4 * i, (i as f32).to_bits());
        }
        r.e.mem.set_u32(maps + 0x14, fifth);
        let _ = fifth_map;
        r.e.mem.set_u32(seventh_map + 0x10, 9.5f32.to_bits());
        r.run(103, 0, 0, 0);
        assert_eq!(r.floats(4), vec![6.0, 7.0, 8.0, 9.0]);
        r.run(104, 0, 0, 0);
        assert_eq!(r.floats(2), vec![5.0, 4.0]);
        r.run(105, 0, 0, 0);
        assert_eq!(r.floats(1), vec![9.5]);
        // The texture sizes: map 5 (constant 111) and the indexed map (114).
        let texture = r.e.mem.alloc(4);
        r.e.mem.set_u32(texture, TEXTURE_VTABLE);
        let slots: Vec<u32> = (0..0x27)
            .map(|i| match i * 4 {
                0x94 => TEXTURE_WIDTH,
                0x98 => TEXTURE_HEIGHT,
                _ => 0,
            })
            .collect();
        r.e.put_vtable(TEXTURE_VTABLE, &slots);
        r.e.register(TEXTURE_WIDTH, |_, _| 256u32.into_ret());
        r.e.register(TEXTURE_HEIGHT, |_, _| 0x8000_0000u32.into_ret());
        r.e.mem.set_u32(fifth + 8, texture);
        r.run(111, 0, 0, 0);
        assert_eq!(r.floats(2), vec![256.0, 2_147_483_648.0]);
        r.e.mem.set_u32(r.entry + 0x18, 1);
        r.e.mem.set_u32(maps + 0x24, fifth);
        r.run(114, 0, 0, 0);
        assert_eq!(r.floats(2), vec![256.0, 2_147_483_648.0]);
        // No map 6: zeros.
        r.run(112, 0, 0, 0);
        assert_eq!(r.floats(2), vec![0.0, 0.0]);
    }

    #[test]
    fn unknown_constants_answer_zero() {
        let mut r = rig(true);
        for id in [0u32, 70, 101, 116] {
            assert_eq!(r.run(id, 0, 0, 0), 0);
        }
        assert!(r.seen().is_empty());
    }
}
