//! `NiMain/nigeometrydata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_decrement};

/// `NiGeometryData::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOMETRY_DATA_RTTI: u32 = 0x011f_4884;

// Translated from 00a67bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOMETRY_DATA_RTTI)
}

// Translated from 00a68800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a67c70`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_geometry_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_7c70;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

/// The base constructor `00a5d3a0` (engine map unit `niobject.cpp`, unnamed; `thiscall`, no
/// stack words).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
/// `NiGeometryData`'s vtable (`0109bf9c`; its RTTI type descriptor reads
/// `.?AVNiGeometryData@@`).
const NI_GEOMETRY_DATA_VTABLE: u32 = 0x0109_bf9c;

// Translated from 00a67c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::NiGeometryData` (Xbox PDB), `thiscall`, no stack words; PC offsets match
/// the Xbox PDB. Runs the base constructor `00a5d3a0`, stores the vtable `0109bf9c`, clears
/// `m_usVertices` (+0x8), `m_usDataFlags` (+0xc), `m_usDirtyFlags` (+0xe), the words +0x20 to
/// +0x34 (`m_pkVertex` .. `m_pkBuffData`) and the bytes +0x38 to +0x3b, and sets
/// `m_bSaveVertexData` (+0x3c) to 1. Then, unless `m_spAdditionalGeomData` (+0x30, just
/// cleared) is set and its vtable slot +0x8c answers a set byte, keeps only the bits 0x8fff of
/// `m_usDirtyFlags`. Answers `this`.
pub fn ni_geometry_data_ni_geometry_data(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![this]);
    e.mem.set_u16(base + 0xc, 0);
    e.mem.set_u32(base, NI_GEOMETRY_DATA_VTABLE);
    e.mem.set_u16(base + 0x8, 0);
    e.mem.set_u16(base + 0xe, 0);
    for offset in (0x20u32..=0x34).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in 0x38u32..=0x3b {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u8(base + 0x3c, 1);
    let additional = e.mem.u32(base + 0x30);
    let keeps_flags = additional != 0 && e.vcall(additional, 0x8c, &args![]).bool();
    if !keeps_flags {
        let flags = e.mem.u16(base + 0xe);
        e.mem.set_u16(base + 0xe, flags & 0x8fff);
    }
    this
}

/// `m_pkGroup` (Xbox PDB static of `NiGeometryData`, `011f4880`): the `NiObjectGroup` the
/// engine map's `GetGroup` (`00a669c0`) answers.
const GEOMETRY_GROUP: u32 = 0x011f_4880;
/// `ms_usNextID` (Xbox PDB static of `NiGeometryData`, `011a9b14`): the next `m_usID`.
const NEXT_ID: u32 = 0x011a_9b14;
/// The object at `011f4748` whose vtable slot +0xf0 the destructor gives `this` (other units
/// call it the renderer; unconfirmed here).
const RENDERER_011F4748: u32 = 0x011f_4748;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40` and free
/// `00aa4060` (`thiscall`, one stack word each).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `00aa1460` (`cdecl`: block, size): the sized delete.
const SIZED_DELETE_00AA1460: u32 = 0x00aa_1460;
/// `00a8a670` (`thiscall`, no stack words, unnamed in the engine map): run on an
/// `NiObjectGroup` whose count reaches zero, before its sized delete of 0x10 bytes.
const GROUP_DESTRUCT_00A8A670: u32 = 0x00a8_a670;
/// `NiBound::ComputeFromData` (engine map, `00a7ee30`; `thiscall` on `m_kBound`, `RET 8`:
/// vertex count, vertices).
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
/// The `NiObject` destructor body `00a5d3d0` (engine map unit `niobject.cpp`, unnamed).
const NI_OBJECT_DESTRUCT_00A5D3D0: u32 = 0x00a5_d3d0;
/// `NiObject::RegisterStreamables` (engine map, `00a5d2f0`; `thiscall`, `RET 4`).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// `NiObject::LoadBinary` (engine map, `00a5d610`; `thiscall`, `RET 4`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a89af0` (`RET 4`, three bytes, unnamed in the engine map): the base step both
/// `LinkObject` and `SaveBinary` start with.
const NI_OBJECT_BASE_STEP_00A89AF0: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (engine map, `00a63430`; `thiscall`, no stack words).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::GetGroupFromID` (engine map, `00a634e0`; `thiscall`, `RET 4`).
const STREAM_GET_GROUP_FROM_ID: u32 = 0x00a6_34e0;
/// `00a634f0` (`thiscall` on the stream, `RET 4`; unnamed, unit `nistream.cpp`): the id
/// `SaveBinary` writes for the group.
const STREAM_GROUP_ID_00A634F0: u32 = 0x00a6_34f0;
/// `00a645b0` (`thiscall` on the stream, no stack words; unit `nistream.cpp`): run at the
/// end of `LoadBinary` for files from 10.3.0.7 on.
const STREAM_LINK_STEP_00A645B0: u32 = 0x00a6_45b0;
/// `NiSphere::LoadBinary` / `NiSphere::SaveBinary` (engine map, `00a69ac0` / `00eb5410`;
/// `thiscall` on `m_kBound`, `RET 4`).
const SPHERE_LOAD_BINARY: u32 = 0x00a6_9ac0;
const SPHERE_SAVE_BINARY: u32 = 0x00eb_5410;
/// `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// `00401050` (`stdcall`: array, element size, count, constructor) with the 16-byte
/// element constructor `004a7800` handed to it for the colours.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
const COLOR_CONSTRUCT_004A7800: u32 = 0x004a_7800;
/// `00a67aa0` (`cdecl`: stream, buffer, count; unnamed, this unit, not translated here):
/// reads the extra texture sets into a scratch buffer.
const READ_EXTRA_TEXTURE_SETS_00A67AA0: u32 = 0x00a6_7aa0;
/// `00a29680` (`cdecl`, format and words): the warning print, with the format at
/// `0109c040` ("%s has a geometry with more than one texture coordinate set, wasting %dk
/// of disk space.").
const WARNING_PRINT_00A29680: u32 = 0x00a2_9680;
const EXTRA_TEXTURE_SETS_FORMAT: u32 = 0x0109_c040;
/// `NiAdditionalGeometryData` (Xbox PDB) methods this unit calls: `00a725d0` (`RET 4`),
/// `00a72590` (`RET 8`) and `00a72720` (`RET 0x1c`), all unnamed in the engine map.
const ADDITIONAL_DATA_00A725D0: u32 = 0x00a7_25d0;
const ADDITIONAL_DATA_00A72590: u32 = 0x00a7_2590;
const ADDITIONAL_DATA_00A72720: u32 = 0x00a7_2720;
/// Virtual slots: `GetGroup` (+0x80, Xbox PDB `NiObject`), `GetActiveVertexCount` (+0x90,
/// Xbox PDB `NiGeometryData`) and the additional data's `GetVBReady` (+0x8c, Xbox PDB
/// `NiAdditionalGeometryData`).
const SLOT_GET_GROUP: u32 = 0x80;
const SLOT_GET_ACTIVE_VERTEX_COUNT: u32 = 0x90;
const SLOT_GET_VB_READY: u32 = 0x8c;
/// `NiStream` fields: the file version (`m_uiNifFileVersion`, +0xd8), the file name (+0xe0) and the streams being read and written
/// (`m_pkIstr` +0x24c, `m_pkOstr` +0x250); `NiBinaryStream`'s position (+4), `m_pfnRead` (+8)
/// and `m_pfnWrite` (+0xc). All Xbox PDB names.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_FILE_NAME: u32 = 0xe0;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_POSITION: u32 = 0x4;
const BINARY_READ: u32 = 0x8;
const BINARY_WRITE: u32 = 0xc;

/// The word at +4 of the stream that `LoadBinary` compares with 0x13 and 0x1b (the
/// `m_BSStreamHeader` version word in `NiStream`'s layout).
fn stream_header_version(e: &Engine, stream: u32) -> u32 {
    e.mem.u32(stream.wrapping_add(4))
}

/// One transfer through a `NiBinaryStream` function (`m_pfnRead` at +8 or `m_pfnWrite` at
/// +0xc): `(binary, buffer, bytes, &element_size, 1)`, the element size held in a local, and
/// the stream's position advanced by the result.
fn binary_transfer(e: &mut Engine, binary: u32, slot: u32, buffer: u32, bytes: u32, size: u32) {
    let function = e.mem.u32(binary.wrapping_add(slot));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), size);
        let moved = e
            .call(
                function,
                &args![binary, buffer, bytes, element.addr(), 1u32],
            )
            .u32();
        let position = e.mem.u32(binary.wrapping_add(BINARY_POSITION));
        e.mem.set_u32(
            binary.wrapping_add(BINARY_POSITION),
            position.wrapping_add(moved),
        );
    });
}

/// Reads `bytes` into `buffer` from the stream being loaded (`m_pkIstr`).
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size: u32) {
    let binary = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    binary_transfer(e, binary, BINARY_READ, buffer, bytes, size);
}

/// Writes `bytes` from `buffer` to the stream being saved (`m_pkOstr`).
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size: u32) {
    let binary = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    binary_transfer(e, binary, BINARY_WRITE, buffer, bytes, size);
}

/// Writes one byte held in a local.
fn stream_write_byte(e: &mut Engine, stream: u32, value: u8) {
    e.with_stack(4, |e, local| {
        e.mem.set_u8(local.addr(), value);
        stream_write(e, stream, local.addr(), 1, 1);
    });
}

/// The overflow-checked size `count * element` (`MUL`, then all ones on overflow) the
/// allocations ask for.
fn checked_size(count: u32, element: u32) -> u32 {
    count.checked_mul(element).unwrap_or(u32::MAX)
}

/// The memory manager's allocate (`00aa3e40`).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER_OBJECT, size])
        .u32()
}

/// The memory manager's free (`00aa4060`).
fn free(e: &mut Engine, block: u32) {
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, block]);
}

/// The block for one array of `bytes` bytes: from the object's group (`GetGroup`, slot
/// +0x80, asked twice as the exe does), whose `m_pvFree` (+8) moves on by `bytes` and whose
/// `m_uiRefCount` (+0xc) is raised, or else from `allocate(size)`.
fn group_or_allocate(e: &mut Engine, this: u32, bytes: u32, size: u32) -> u32 {
    if e.vcall(this, SLOT_GET_GROUP, &args![]).u32() == 0 {
        return allocate(e, size);
    }
    let group = e.vcall(this, SLOT_GET_GROUP, &args![]).u32();
    let block = e.mem.u32(group.wrapping_add(8));
    let count = e.mem.u32(group.wrapping_add(0xc)).wrapping_add(1);
    e.mem.set_u32(group.wrapping_add(0xc), count);
    e.mem
        .set_u32(group.wrapping_add(8), block.wrapping_add(bytes));
    block
}

/// Gives up one use of `group` (`m_uiRefCount` +0xc, a plain decrement): at zero it is
/// destroyed (`00a8a670`) and deleted (0x10 bytes).
fn release_group(e: &mut Engine, group: u32) {
    let count = e.mem.u32(group.wrapping_add(0xc)).wrapping_sub(1);
    e.mem.set_u32(group.wrapping_add(0xc), count);
    if count == 0 {
        e.call(GROUP_DESTRUCT_00A8A670, &args![group]);
        e.call(SIZED_DELETE_00AA1460, &args![group, 0x10u32]);
    }
}

/// Whether `m_spAdditionalGeomData` (+0x30) is set and its `GetVBReady` (+0x8c) answers
/// true.
fn additional_data_ready(e: &mut Engine, this: u32) -> bool {
    let additional = e.mem.u32(this.wrapping_add(0x30));
    additional != 0 && e.vcall(additional, SLOT_GET_VB_READY, &args![]).bool()
}

// Translated from 00a67090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::MarkAsChanged` (Xbox PDB), `thiscall`, `RET 4`: unless the additional
/// data is ready (`GetVBReady`), ors the low 12 bits of `flags` into `m_usDirtyFlags` (+0xe).
pub fn ni_geometry_data_mark_as_changed(e: &mut Engine, this: Ptr, flags: u32) {
    let base = this.addr();
    if !additional_data_ready(e, base) {
        let dirty = e.mem.u16(base + 0xe);
        e.mem.set_u16(base + 0xe, dirty | (flags & 0xfff) as u16);
    }
}

// Translated from 00a66960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::GetTextureSet` (Xbox PDB), `thiscall`, `RET 4`: answers `m_pkTexture`
/// (+0x2c); the set index is not read.
pub fn ni_geometry_data_get_texture_set(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    e.mem.u32(this.addr() + 0x2c)
}

// Translated from 00a67050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::SetConsistency` (Xbox PDB), `thiscall`, `RET 4`: unless the additional
/// data is ready, `m_usDirtyFlags` (+0xe) keeps its bits 0x8fff and takes the low 16 bits
/// of `consistency`.
pub fn ni_geometry_data_set_consistency(e: &mut Engine, this: Ptr, consistency: u32) {
    let base = this.addr();
    if !additional_data_ready(e, base) {
        let dirty = e.mem.u16(base + 0xe);
        e.mem
            .set_u16(base + 0xe, (dirty & 0x8fff) | consistency as u16);
    }
}

// Translated from 00a669c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::GetGroup` (Xbox PDB): answers the static `m_pkGroup` (`011f4880`).
pub fn ni_geometry_data_get_group(e: &mut Engine) -> u32 {
    e.mem.u32(GEOMETRY_GROUP)
}

// Translated from 00a669e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::GetActiveVertexCount` (Xbox PDB): `m_usVertices` (+8).
pub fn ni_geometry_data_get_active_vertex_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 8)
}

// Translated from 00a66bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::ReplaceData` (Xbox PDB), `thiscall`, `RET 0x1c`: stores the vertex
/// count (+8, 16 bits), the vertex, normal, colour and texture arrays (+0x20 to +0x2c), and
/// sets `m_usDataFlags` (+0xc) to its bits 0x0ffe, bit 0 of `texture_sets` and the low 16 bits
/// of `flags`. Nothing is freed.
#[allow(clippy::too_many_arguments)]
pub fn ni_geometry_data_replace_data(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    textures: u32,
    texture_sets: u32,
    flags: u32,
) {
    let base = this.addr();
    e.mem.set_u16(base + 8, vertex_count as u16);
    e.mem.set_u32(base + 0x20, vertices);
    e.mem.set_u32(base + 0x24, normals);
    e.mem.set_u32(base + 0x28, colors);
    e.mem.set_u32(base + 0x2c, textures);
    let data_flags = e.mem.u16(base + 0xc);
    e.mem.set_u16(
        base + 0xc,
        (data_flags & 0x0ffe) | (texture_sets & 1) as u16 | flags as u16,
    );
}

// Translated from 00a67b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`): the `NiGeometryData` constructor
/// with data (`thiscall`, `RET 0x1c`: vertex count, vertices, normals, colours, textures,
/// texture-set count, flags). Runs the `NiObject` constructor `00a5d3a0`, stores the count and
/// arrays, clears `m_usDataFlags` and `m_usDirtyFlags`, sets the vtable `0109bf9c`, clears
/// +0x30, +0x34 and the bytes +0x38 to +0x3b, sets `m_bSaveVertexData` (+0x3c), sets
/// `m_usDataFlags` to bit 0 of `texture_sets` or the low 16 bits of `flags`, computes the bound
/// (`00a7ee30` on +0x10) when there are vertices, takes the next id (`ms_usNextID`,
/// `011a9b14`, into +0xa) and keeps only the bits 0x8fff of `m_usDirtyFlags` unless the
/// additional data is ready. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a67b20(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    textures: u32,
    texture_sets: u32,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![this]);
    e.mem.set_u16(base + 8, vertex_count as u16);
    e.mem.set_u16(base + 0xc, 0);
    e.mem.set_u32(base + 0x20, vertices);
    e.mem.set_u16(base + 0xe, 0);
    e.mem.set_u32(base + 0x24, normals);
    e.mem.set_u32(base + 0x2c, textures);
    e.mem.set_u32(base + 0x28, colors);
    e.mem.set_u32(base, NI_GEOMETRY_DATA_VTABLE);
    e.mem.set_u32(base + 0x30, 0);
    let data_flags = (e.mem.u16(base + 0xc) & 0x0ffe) | (texture_sets & 1) as u16 | flags as u16;
    e.mem.set_u32(base + 0x34, 0);
    for offset in 0x38u32..=0x3b {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u8(base + 0x3c, 1);
    e.mem.set_u16(base + 0xc, data_flags);
    let count = e.mem.u16(base + 8);
    if count > 0 {
        let vertices = e.mem.u32(base + 0x20);
        if vertices != 0 {
            e.call(
                BOUND_COMPUTE_FROM_DATA,
                &args![base + 0x10, u32::from(count), vertices],
            );
        }
    }
    let id = e.mem.u16(NEXT_ID);
    e.mem.set_u16(base + 0xa, id);
    let next = e.mem.u16(NEXT_ID).wrapping_add(1);
    e.mem.set_u16(NEXT_ID, next);
    if !additional_data_ready(e, base) {
        let dirty = e.mem.u16(base + 0xe);
        e.mem.set_u16(base + 0xe, dirty & 0x8fff);
    }
    this
}

// Translated from 00a670c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, `RET 0x1c`, the
/// arguments of `ReplaceData`): replaces the data and gives up the old arrays. For each of
/// the vertex, normal, colour and texture arrays (+0x20 to +0x2c) that changes: with a group
/// (`GetGroup`, slot +0x80) the group's use is given up (`m_uiRefCount` +0xc decremented,
/// destroyed and deleted at zero); without one the old array is freed (`00aa4060`). Then the
/// vertex array and count, the normal array, the bound (`00a7ee30` with
/// `GetActiveVertexCount`, slot +0x90, when it and the vertices are not zero), the colour and
/// texture arrays and `m_usDataFlags` (bits 0x0ffe, bit 0 of `texture_sets`, the low 16 bits of
/// `flags`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00a670c0(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    textures: u32,
    texture_sets: u32,
    flags: u32,
) {
    let base = this.addr();
    let arrays = [
        (0x20u32, vertices),
        (0x24, normals),
        (0x28, colors),
        (0x2c, textures),
    ];
    if e.vcall(base, SLOT_GET_GROUP, &args![]).u32() != 0 {
        for (offset, new) in arrays {
            if new != e.mem.u32(base + offset) {
                let group = e.vcall(base, SLOT_GET_GROUP, &args![]).u32();
                release_group(e, group);
            }
        }
    } else {
        for (offset, new) in arrays {
            let old = e.mem.u32(base + offset);
            if new != old {
                free(e, old);
            }
        }
    }
    e.mem.set_u32(base + 0x20, vertices);
    e.mem.set_u16(base + 8, vertex_count as u16);
    e.mem.set_u32(base + 0x24, normals);
    let active = e.vcall(base, SLOT_GET_ACTIVE_VERTEX_COUNT, &args![]).u16();
    if active != 0 {
        let vertices = e.mem.u32(base + 0x20);
        if vertices != 0 {
            e.call(
                BOUND_COMPUTE_FROM_DATA,
                &args![base + 0x10, u32::from(active), vertices],
            );
        }
    }
    e.mem.set_u32(base + 0x28, colors);
    let data_flags = (e.mem.u16(base + 0xc) & 0x0ffe) | (texture_sets & 1) as u16 | flags as u16;
    e.mem.set_u32(base + 0x2c, textures);
    e.mem.set_u16(base + 0xc, data_flags);
}

// Translated from 00a67c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`): the `NiGeometryData` destructor
/// body (`thiscall`, no stack words; the scalar deleting destructor `00a68800` runs it).
/// Sets the vtable `0109bf9c`, gives `this` to slot +0xf0 of the object at `011f4748` when
/// there is one, releases `m_spAdditionalGeomData` (+0x30: `InterlockedDecrement` on its +4,
/// slot +4 at zero) and clears it, then gives up the four arrays (+0x20 to +0x2c): with a
/// group (`m_pkGroup`, `011f4880`) one group use per array that is set, without one every
/// array is freed (`00aa4060`, null included). Then releases +0x30 again when set, and ends
/// in the `NiObject` destructor body `00a5d3d0`.
pub fn fn_00a67c70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_GEOMETRY_DATA_VTABLE);
    let renderer = e.mem.u32(RENDERER_011F4748);
    if renderer != 0 {
        e.vcall(renderer, 0xf0, &args![base]);
    }
    let additional = e.mem.u32(base + 0x30);
    if additional != 0 {
        if interlocked_decrement(e, additional.wrapping_add(4)) == 0 {
            e.vcall(additional, 4, &args![]);
        }
        e.mem.set_u32(base + 0x30, 0);
    }
    if e.mem.u32(GEOMETRY_GROUP) == 0 {
        for offset in [0x20u32, 0x24, 0x28, 0x2c] {
            let array = e.mem.u32(base + offset);
            free(e, array);
        }
    } else {
        for offset in [0x20u32, 0x24, 0x28, 0x2c] {
            if e.mem.u32(base + offset) != 0 {
                let group = e.mem.u32(GEOMETRY_GROUP);
                release_group(e, group);
            }
        }
    }
    let additional = e.mem.u32(base + 0x30);
    if additional != 0 && interlocked_decrement(e, additional.wrapping_add(4)) == 0 {
        e.vcall(additional, 4, &args![]);
    }
    e.call(NI_OBJECT_DESTRUCT_00A5D3D0, &args![this]);
}

// Translated from 00a66f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::RegisterStreamables` (Xbox PDB), `thiscall`, `RET 4`: false when
/// `NiObject::RegisterStreamables` (`00a5d2f0`) is false or the additional data (+0x30, when
/// set) answers false from its slot +0x54 (`RegisterStreamables`); true otherwise.
pub fn ni_geometry_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let additional = e.mem.u32(this.addr() + 0x30);
    if additional != 0 && !e.vcall(additional, 0x54, &args![stream]).bool() {
        return false;
    }
    true
}

// Translated from 00a677e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::LinkObject` (Xbox PDB), `thiscall`, `RET 4`: the base step `00a89af0`,
/// then for files from 10.3.0.7 on (`m_uiNifFileVersion` >= 0x0a030007) the object of the
/// next link id (`00a63430`) becomes `m_spAdditionalGeomData` (+0x30, counted).
pub fn ni_geometry_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_BASE_STEP_00A89AF0, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= 0x0a03_0007 {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        assign_ref_counted(e, this.addr() + 0x30, object);
    }
}

// Translated from 00a67260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, `RET 4`): makes
/// `object` the `m_spAdditionalGeomData` (+0x30): nothing when it is already there, else the
/// old one is released (deleted through slot +4 at zero) and the new one counted.
pub fn fn_00a67260(e: &mut Engine, this: Ptr, object: Ptr) {
    assign_ref_counted(e, this.addr() + 0x30, object.addr());
}

// Translated from 00a67360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, `RET 4`): locks the
/// vertex buffers. False when `m_bVBLocked` (+0x3a) is already set. Otherwise, when the
/// additional data is ready: for a write lock `00a725d0(0)` on it, else `00a72590(0, &zero)`
/// with a zeroed local; then `m_bVBLockWrite` (+0x3b) is `write`, `m_bVBLocked` set, true.
pub fn fn_00a67360(e: &mut Engine, this: Ptr, write: u32) -> bool {
    let base = this.addr();
    if e.mem.u8(base + 0x3a) != 0 {
        return false;
    }
    let write = write as u8;
    let additional = e.mem.u32(base + 0x30);
    if additional != 0 && e.vcall(additional, SLOT_GET_VB_READY, &args![]).bool() {
        if write != 0 {
            e.call(ADDITIONAL_DATA_00A725D0, &args![additional, 0u32]);
        } else {
            e.with_stack(4, |e, local| {
                e.call(
                    ADDITIONAL_DATA_00A72590,
                    &args![additional, 0u32, local.addr()],
                );
            });
        }
    }
    e.mem.set_u8(base + 0x3b, write);
    e.mem.set_u8(base + 0x3a, 1);
    true
}

// Translated from 00a673d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, no stack words):
/// unlocks the vertex buffers. Only when `m_bVBLocked` (+0x3a) is set: when the additional
/// data is ready and the lock was not a write lock (+0x3b clear), `00a725d0(0)` on it; then
/// `m_bVBLocked` is cleared.
pub fn fn_00a673d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x3a) == 0 {
        return;
    }
    let additional = e.mem.u32(base + 0x30);
    if additional != 0
        && e.vcall(additional, SLOT_GET_VB_READY, &args![]).bool()
        && e.mem.u8(base + 0x3b) == 0
    {
        e.call(ADDITIONAL_DATA_00A725D0, &args![additional, 0u32]);
    }
    e.mem.set_u8(base + 0x3a, 0);
}

/// The common body of `00a67410` and `00a67720`: when the buffers are locked, asks the ready
/// additional data for stream `stream` (`00a72720` with six locals, four of them zeroed),
/// then writes `{array, stride, 0}` into `out`: the locals' array and stride when it gave an
/// array, else `fallback` and `fallback_stride`. Nothing is written when not locked.
fn locked_array(
    e: &mut Engine,
    this: u32,
    out: u32,
    stream: impl FnOnce(&Engine, u32) -> u32,
    fallback_offset: u32,
    fallback_stride: u32,
) {
    if e.mem.u8(this + 0x3a) == 0 {
        return;
    }
    let additional = e.mem.u32(this + 0x30);
    // Locals at +0 (array), +4 (stride), +8, +0xc, +0x10, +0x14; +0, +4, +8 and +0xc are
    // zeroed.
    let (array, stride) = e.with_stack(0x18, |e, locals| {
        let l = locals.addr();
        if additional != 0 && e.vcall(additional, SLOT_GET_VB_READY, &args![]).bool() {
            let index = stream(e, additional);
            e.call(
                ADDITIONAL_DATA_00A72720,
                &args![
                    additional,
                    index,
                    l,
                    l + 0x14,
                    l + 0xc,
                    l + 0x10,
                    l + 8,
                    l + 4
                ],
            );
        }
        (e.mem.u32(l), e.mem.u32(l + 4))
    });
    let (array, stride) = if array != 0 {
        (array, stride)
    } else {
        (e.mem.u32(this + fallback_offset), fallback_stride)
    };
    e.mem.set_u32(out, array);
    e.mem.set_u32(out + 4, stride);
    e.mem.set_u8(out + 8, 0);
}

// Translated from 00a67410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, `RET 4`): while the
/// buffers are locked, writes the locked vertex array into `out` (`{array, stride, byte 0}`):
/// the additional data's stream 0 when it gives one, else `m_pkVertex` (+0x20) with stride
/// 0xc. Writes nothing when not locked.
pub fn fn_00a67410(e: &mut Engine, this: Ptr, out: Ptr) {
    locked_array(e, this.addr(), out.addr(), |_, _| 0, 0x20, 0xc);
}

// Translated from 00a67720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `thiscall`, `RET 8`: a set index it
/// does not read, then `out`): like `00a67410` for the texture coordinates: the stream is
/// `2 + 2 * (bit 0 of the additional data's +0x2c) + bit 1 of it`, the fallback
/// `m_pkTexture` (+0x2c) with stride 8.
pub fn fn_00a67720(e: &mut Engine, this: Ptr, _unused_1: u32, out: Ptr) {
    locked_array(
        e,
        this.addr(),
        out.addr(),
        |e, additional| {
            let mode = e.mem.u32(additional + 0x2c);
            u32::from(mode & 1 != 0) * 2 + 2 + ((mode >> 1) & 1)
        },
        0x2c,
        8,
    );
}

// Translated from 00a67ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometrydata.cpp`; `cdecl`: stream, texture array,
/// count): writes `count * 8` bytes of the array to the stream being saved with 4-byte
/// elements (the element size is kept in the stream argument's own slot). Answers what the
/// write function answered.
pub fn fn_00a67ae0(e: &mut Engine, stream: Ptr, textures: u32, count: u32) -> u32 {
    let binary = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let function = e.mem.u32(binary.wrapping_add(BINARY_WRITE));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), 4);
        let moved = e
            .call(
                function,
                &args![
                    binary,
                    textures,
                    count.wrapping_mul(8),
                    element.addr(),
                    1u32
                ],
            )
            .u32();
        let position = e.mem.u32(binary.wrapping_add(BINARY_POSITION));
        e.mem.set_u32(
            binary.wrapping_add(BINARY_POSITION),
            position.wrapping_add(moved),
        );
        moved
    })
}

/// Reads a presence flag the way `LoadBinary` does: before 4.1.0.0 the array pointer
/// itself (4 bytes into `field`) and its being non-zero; from 4.1.0.0 one byte into `local`
/// (whose low byte starts as `initial`).
fn read_presence(e: &mut Engine, stream: u32, field: u32, initial: u8) -> bool {
    if e.mem.u32(stream + STREAM_VERSION) < 0x0401_0000 {
        stream_read(e, stream, field, 4, 4);
        e.mem.u32(field) != 0
    } else {
        e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), initial);
            stream_read(e, stream, local.addr(), 1, 1);
            e.mem.u8(local.addr()) != 0
        })
    }
}

/// Whether `LoadBinary` reads an array's contents: header version (+4) below 0x13 or
/// `m_bSaveVertexData` (+0x3c) set.
fn reads_contents(e: &Engine, this: u32, stream: u32) -> bool {
    stream_header_version(e, stream) < 0x13 || e.mem.u8(this + 0x3c) != 0
}

// Translated from 00a67ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`. After
/// `NiObject::LoadBinary`: from 10.1.0.114 the group id (`GetGroupFromID` into `m_pkGroup`),
/// the vertex count, from 10.0.1.16 `m_ucKeepFlags` and `m_ucCompressFlags`; then the
/// vertices, the data flags (from 10.0.0.2; three times the normals when a bit of 0xf000 is
/// set), the normals, the bound (`NiSphere::LoadBinary`), the colours (constructed with
/// `004a7800` when allocated), the data flags before 5.0.0.10 (only bit 0 kept, or read
/// whole up to 10.0.0.1), the texture pointer before 4.1.0.0, then for header versions (+4) below
/// 0x1b the texture-set count from the data flags, the texture array (with the sets beyond
/// the first read into a scratch buffer by `00a67aa0`, freed, and a warning printed), from
/// 5.0.0.10 `m_usDirtyFlags` and from 10.3.0.7 the link step `00a645b0`. Each array comes
/// from the group or the memory manager; its contents are read when the header version (+4) is
/// below 0x13 or `m_bSaveVertexData` is set (the vertices are zeroed otherwise).
pub fn ni_geometry_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + STREAM_VERSION) >= 0x0a01_0072 {
        let id = e.with_stack(4, |e, local| {
            stream_read(e, stream, local.addr(), 4, 4);
            e.mem.u32(local.addr())
        });
        let group = e.call(STREAM_GET_GROUP_FROM_ID, &args![stream, id]).u32();
        e.mem.set_u32(GEOMETRY_GROUP, group);
    }
    stream_read(e, stream, base + 8, 2, 2);
    if e.mem.u32(stream + STREAM_VERSION) >= 0x0a00_0110 {
        stream_read(e, stream, base + 0x38, 1, 1);
        stream_read(e, stream, base + 0x39, 1, 1);
    }

    // Vertices (the flag's local is the stream argument's own slot).
    if read_presence(e, stream, base + 0x20, stream as u8) {
        let count = u32::from(e.mem.u16(base + 8));
        let block = group_or_allocate(e, base, count * 0xc, checked_size(count, 0xc));
        e.mem.set_u32(base + 0x20, block);
        if reads_contents(e, base, stream) {
            let bytes = u32::from(e.mem.u16(base + 8)) * 0xc;
            let vertices = e.mem.u32(base + 0x20);
            stream_read(e, stream, vertices, bytes, 4);
        } else {
            let bytes = u32::from(e.mem.u16(base + 8)) * 0xc;
            let vertices = e.mem.u32(base + 0x20);
            e.call(MEMSET, &args![vertices, 0u32, bytes]);
        }
    }

    let mut normal_count = u32::from(e.mem.u16(base + 8));
    if e.mem.u32(stream + STREAM_VERSION) >= 0x0a00_0002 {
        stream_read(e, stream, base + 0xc, 2, 2);
        if e.mem.u16(base + 0xc) & 0xf000 != 0 {
            normal_count *= 3;
        }
    }
    if read_presence(e, stream, base + 0x24, 0) {
        let block = group_or_allocate(
            e,
            base,
            normal_count.wrapping_mul(0xc),
            checked_size(normal_count, 0xc),
        );
        e.mem.set_u32(base + 0x24, block);
        if reads_contents(e, base, stream) {
            let normals = e.mem.u32(base + 0x24);
            stream_read(e, stream, normals, normal_count.wrapping_mul(0xc), 4);
        }
    }

    e.call(SPHERE_LOAD_BINARY, &args![base + 0x10, stream]);

    if read_presence(e, stream, base + 0x28, 0) {
        let block = if e.vcall(base, SLOT_GET_GROUP, &args![]).u32() != 0 {
            let bytes = u32::from(e.mem.u16(base + 8)) << 4;
            let group = e.vcall(base, SLOT_GET_GROUP, &args![]).u32();
            let block = e.mem.u32(group + 8);
            let uses = e.mem.u32(group + 0xc).wrapping_add(1);
            e.mem.set_u32(group + 0xc, uses);
            e.mem.set_u32(group + 8, bytes.wrapping_add(block));
            block
        } else {
            let count = u32::from(e.mem.u16(base + 8));
            let block = allocate(e, checked_size(count, 0x10));
            if block != 0 {
                e.call(
                    VECTOR_CONSTRUCTOR_ITERATOR,
                    &args![block, 0x10u32, count, COLOR_CONSTRUCT_004A7800],
                );
            }
            block
        };
        e.mem.set_u32(base + 0x28, block);
        if reads_contents(e, base, stream) {
            let bytes = u32::from(e.mem.u16(base + 8)) << 4;
            let colors = e.mem.u32(base + 0x28);
            stream_read(e, stream, colors, bytes, 4);
        }
    }

    // Before 5.0.0.10 the flags are read into the local that holds the normal count (and
    // later the texture-set count).
    let version = e.mem.u32(stream + STREAM_VERSION);
    if version < 0x0500_000a {
        let read = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), normal_count);
            stream_read(e, stream, local.addr(), 2, 2);
            e.mem.u32(local.addr())
        });
        let data_flags = e.mem.u16(base + 0xc);
        e.mem
            .set_u16(base + 0xc, data_flags ^ ((data_flags ^ read as u16) & 1));
    } else if version < 0x0a00_0002 {
        stream_read(e, stream, base + 0xc, 2, 2);
    }
    if e.mem.u32(stream + STREAM_VERSION) < 0x0401_0000 {
        stream_read(e, stream, base + 0x2c, 4, 4);
    }
    let sets = if stream_header_version(e, stream) < 0x1b {
        let data_flags = u32::from(e.mem.u16(base + 0xc));
        let sets = data_flags & 0x1f;
        if sets as u16 > 1 {
            e.mem.set_u16(base + 0xc, (data_flags | 1) as u16);
        }
        sets
    } else {
        0
    };

    let data_flags = u32::from(e.mem.u16(base + 0xc));
    if data_flags & 1 != 0 {
        let count = (data_flags & 1).wrapping_mul(u32::from(e.mem.u16(base + 8)));
        let block = group_or_allocate(e, base, count.wrapping_mul(8), checked_size(count, 8));
        e.mem.set_u32(base + 0x2c, block);
        if reads_contents(e, base, stream) {
            let textures = e.mem.u32(base + 0x2c);
            stream_read(e, stream, textures, count.wrapping_mul(8), 4);
            if sets as u16 > 1 {
                let extra = u32::from(sets as u16 - 1).wrapping_mul(u32::from(e.mem.u16(base + 8)));
                let scratch = allocate(e, checked_size(extra, 8));
                e.call(
                    READ_EXTRA_TEXTURE_SETS_00A67AA0,
                    &args![stream, scratch, extra],
                );
                free(e, scratch);
                e.call(
                    WARNING_PRINT_00A29680,
                    &args![
                        EXTRA_TEXTURE_SETS_FORMAT,
                        stream + STREAM_FILE_NAME,
                        extra.wrapping_mul(8) >> 10
                    ],
                );
            }
        }
    }

    if e.mem.u32(stream + STREAM_VERSION) >= 0x0500_000a {
        stream_read(e, stream, base + 0xe, 2, 2);
    }
    if e.mem.u32(stream + STREAM_VERSION) >= 0x0a03_0007 {
        e.call(STREAM_LINK_STEP_00A645B0, &args![stream]);
    }
}

// Translated from 00a68520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::SaveBinary` (Xbox PDB), `thiscall`, `RET 4`: the base step `00a89af0`,
/// the group's id (`00a634f0`), the vertex count, `m_ucKeepFlags`, `m_ucCompressFlags`, a
/// presence byte and (when present and `m_bSaveVertexData`) the vertices; clears bit 0 of
/// `m_usDataFlags` when there are no texture coordinates and writes it; the normals' presence
/// and contents (three times as many when a bit of 0xf000 is set), the bound
/// (`NiSphere::SaveBinary`), the colours' presence and contents, the texture coordinates
/// through `00a67ae0` (when bit 0 of the data flags, the vertex count, the array and
/// `m_bSaveVertexData` all allow), the bits 0x7000 of `m_usDirtyFlags` and the additional
/// data's link id (the stream's slot +0x30).
pub fn ni_geometry_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(NI_OBJECT_BASE_STEP_00A89AF0, &args![this, stream]);
    let group = e.mem.u32(GEOMETRY_GROUP);
    let id = e
        .call(STREAM_GROUP_ID_00A634F0, &args![stream, group])
        .u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        stream_write(e, stream, local.addr(), 4, 4);
    });
    stream_write(e, stream, base + 8, 2, 2);
    stream_write(e, stream, base + 0x38, 1, 1);
    stream_write(e, stream, base + 0x39, 1, 1);

    let has_vertices = e.mem.u32(base + 0x20) != 0;
    stream_write_byte(e, stream, u8::from(has_vertices));
    if has_vertices && e.mem.u8(base + 0x3c) != 0 {
        let bytes = u32::from(e.mem.u16(base + 8)) * 0xc;
        let vertices = e.mem.u32(base + 0x20);
        stream_write(e, stream, vertices, bytes, 4);
    }

    if e.mem.u32(base + 0x2c) == 0 {
        let data_flags = e.mem.u16(base + 0xc);
        e.mem.set_u16(base + 0xc, data_flags & 0xfffe);
    }
    stream_write(e, stream, base + 0xc, 2, 2);
    let mut normal_count = u32::from(e.mem.u16(base + 8));
    if e.mem.u16(base + 0xc) & 0xf000 != 0 {
        normal_count *= 3;
    }
    let has_normals = e.mem.u32(base + 0x24) != 0;
    stream_write_byte(e, stream, u8::from(has_normals));
    if has_normals && e.mem.u8(base + 0x3c) != 0 {
        let normals = e.mem.u32(base + 0x24);
        stream_write(e, stream, normals, normal_count * 0xc, 4);
    }

    e.call(SPHERE_SAVE_BINARY, &args![base + 0x10, stream]);

    let has_colors = e.mem.u32(base + 0x28) != 0;
    stream_write_byte(e, stream, u8::from(has_colors));
    if has_colors && e.mem.u8(base + 0x3c) != 0 {
        let bytes = u32::from(e.mem.u16(base + 8)) << 4;
        let colors = e.mem.u32(base + 0x28);
        stream_write(e, stream, colors, bytes, 4);
    }

    let texture_count = u32::from(e.mem.u16(base + 0xc) & 1) * u32::from(e.mem.u16(base + 8));
    if texture_count != 0 {
        let textures = e.mem.u32(base + 0x2c);
        if textures != 0 && e.mem.u8(base + 0x3c) != 0 {
            fn_00a67ae0(e, Ptr::new(stream), textures, texture_count);
        }
    }

    let dirty = u32::from(e.mem.u16(base + 0xe) & 0x7000);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), dirty);
        stream_write(e, stream, local.addr(), 2, 2);
    });
    let additional = e.mem.u32(base + 0x30);
    e.vcall(stream, 0x30, &args![additional]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_7090, ni_geometry_data_mark_as_changed(Ptr, u32)),
        entry!(0x00a6_6960, ni_geometry_data_get_texture_set(Ptr, u32) -> u32),
        entry!(0x00a6_7050, ni_geometry_data_set_consistency(Ptr, u32)),
        entry!(0x00a6_69c0, ni_geometry_data_get_group() -> u32),
        entry!(0x00a6_69e0, ni_geometry_data_get_active_vertex_count(Ptr) -> u16),
        entry!(
            0x00a6_6bb0,
            ni_geometry_data_replace_data(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a6_7b20,
            fn_00a67b20(Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00a6_70c0,
            fn_00a670c0(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00a6_7c70, fn_00a67c70(Ptr)),
        entry!(
            0x00a6_6f50,
            ni_geometry_data_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a6_77e0, ni_geometry_data_link_object(Ptr, Ptr)),
        entry!(0x00a6_7260, fn_00a67260(Ptr, Ptr)),
        entry!(0x00a6_7360, fn_00a67360(Ptr, u32) -> bool),
        entry!(0x00a6_73d0, fn_00a673d0(Ptr)),
        entry!(0x00a6_7410, fn_00a67410(Ptr, Ptr)),
        entry!(0x00a6_7720, fn_00a67720(Ptr, u32, Ptr)),
        entry!(0x00a6_7ae0, fn_00a67ae0(Ptr, u32, u32) -> u32),
        entry!(0x00a6_7ea0, ni_geometry_data_load_binary(Ptr, Ptr)),
        entry!(0x00a6_8520, ni_geometry_data_save_binary(Ptr, Ptr)),
        entry!(0x00a6_7c00, ni_geometry_data_ni_geometry_data(Ptr) -> Ptr),
        entry!(0x00a67bf0, ni_geometry_data_get_rtti() -> Ptr),
        entry!(0x00a68800, ni_geometry_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geometry_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a67bf0, &args![]).u32(), NI_GEOMETRY_DATA_RTTI);
    }

    #[test]
    fn ni_geometry_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_7c70, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a68800, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a68800, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00a6_7c70), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn ni_geometry_data_ni_geometry_data_clears_the_fields() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        for offset in (0x4u32..0x40).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00a6_7c00, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u16(this + 0x8), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0xffff);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
        for offset in (0x20u32..0x3c).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(e.mem.u8(this + 0x3d), 0xff);
        let log = e.call_log.take().unwrap();
        let constructed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_OBJECT_CONSTRUCT_00A5D3A0)
            .collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![this]);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Test doubles: the data's vtable (slot +0x80 `GetGroup` answering the word at
    /// `GROUP_ANSWER`, +0x90 answering the vertex count), the additional data's vtable
    /// (+4 delete, +0x54 answering `ADDITIONAL_REGISTER`, +0x8c answering `READY`).
    const DATA_VTABLE: u32 = 0x0310_0000;
    const ADDITIONAL_VTABLE: u32 = 0x0311_0000;
    const GET_GROUP_DOUBLE: u32 = 0x0312_0000;
    const ACTIVE_COUNT_DOUBLE: u32 = 0x0312_0010;
    const DELETE_DOUBLE: u32 = 0x0312_0020;
    const REGISTER_DOUBLE: u32 = 0x0312_0030;
    const READY_DOUBLE: u32 = 0x0312_0040;
    const RENDERER_SLOT_DOUBLE: u32 = 0x0312_0050;
    const STREAM_LINK_SLOT_DOUBLE: u32 = 0x0312_0060;
    const READ_DOUBLE: u32 = 0x0312_0070;
    const WRITE_DOUBLE: u32 = 0x0312_0080;
    const STREAM_VTABLE: u32 = 0x0313_0000;
    /// Words the doubles answer, set by each test.
    const GROUP_ANSWER: u32 = 0x0314_0000;
    const READY: u32 = 0x0314_0004;
    const ADDITIONAL_REGISTER: u32 = 0x0314_0008;

    type Chunks = Rc<RefCell<Vec<Vec<u8>>>>;
    type Log = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0314_0000, 0x1000);
        let mut slots = vec![0u32; 0x28];
        slots[0x80 / 4] = GET_GROUP_DOUBLE;
        slots[0x90 / 4] = ACTIVE_COUNT_DOUBLE;
        e.put_vtable(DATA_VTABLE, &slots);
        let mut slots = vec![0u32; 0x28];
        slots[1] = DELETE_DOUBLE;
        slots[0x54 / 4] = REGISTER_DOUBLE;
        slots[0x8c / 4] = READY_DOUBLE;
        e.put_vtable(ADDITIONAL_VTABLE, &slots);
        let mut slots = vec![0u32; 0x40];
        slots[0xf0 / 4] = RENDERER_SLOT_DOUBLE;
        slots[0x30 / 4] = STREAM_LINK_SLOT_DOUBLE;
        e.put_vtable(STREAM_VTABLE, &slots);
        e.register(GET_GROUP_DOUBLE, |e, _| e.mem.u32(GROUP_ANSWER).into_ret());
        e.register(ACTIVE_COUNT_DOUBLE, |e, a| {
            u32::from(e.mem.u16(a[0] + 8)).into_ret()
        });
        e.register(READY_DOUBLE, |e, _| e.mem.u32(READY).into_ret());
        e.register(REGISTER_DOUBLE, |e, _| {
            e.mem.u32(ADDITIONAL_REGISTER).into_ret()
        });
        for double in [
            DELETE_DOUBLE,
            RENDERER_SLOT_DOUBLE,
            STREAM_LINK_SLOT_DOUBLE,
            BOUND_COMPUTE_FROM_DATA,
            NI_OBJECT_CONSTRUCT_00A5D3A0,
            NI_OBJECT_DESTRUCT_00A5D3D0,
            GROUP_DESTRUCT_00A8A670,
            SIZED_DELETE_00AA1460,
            MEMORY_MANAGER_FREE,
            ADDITIONAL_DATA_00A725D0,
            ADDITIONAL_DATA_00A72590,
            NI_OBJECT_BASE_STEP_00A89AF0,
            NI_OBJECT_LOAD_BINARY,
            SPHERE_LOAD_BINARY,
            SPHERE_SAVE_BINARY,
            MEMSET,
            VECTOR_CONSTRUCTOR_ITERATOR,
            READ_EXTRA_TEXTURE_SETS_00A67AA0,
            WARNING_PRINT_00A29680,
            STREAM_LINK_STEP_00A645B0,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(STREAM_GET_GROUP_FROM_ID, |_, a| (a[1] + 0x100).into_ret());
        e.register(STREAM_GROUP_ID_00A634F0, |_, _| 0x77u32.into_ret());
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, DATA_VTABLE);
        data
    }

    fn additional(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object, ADDITIONAL_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn group(e: &mut Engine, free: u32, uses: u32) -> u32 {
        let group = e.mem.alloc(0x10);
        e.mem.set_u32(group + 8, free);
        e.mem.set_u32(group + 0xc, uses);
        group
    }

    #[test]
    fn mark_as_changed_ors_twelve_bits_unless_ready() {
        let mut e = engine();
        let this = data(&mut e);
        e.mem.set_u16(this + 0xe, 0x8000);
        e.call(0x00a6_7090, &args![this, 0xffff_f123u32]);
        assert_eq!(e.mem.u16(this + 0xe), 0x8123);
        let extra = additional(&mut e, 1);
        e.mem.set_u32(this + 0x30, extra);
        e.mem.set_u32(READY, 1);
        e.call(0x00a6_7090, &args![this, 0x0456u32]);
        assert_eq!(e.mem.u16(this + 0xe), 0x8123);
        e.mem.set_u32(READY, 0x100);
        e.call(0x00a6_7090, &args![this, 0x0456u32]);
        assert_eq!(e.mem.u16(this + 0xe), 0x8577);
    }

    #[test]
    fn get_texture_set_answers_the_texture_array() {
        let mut e = engine();
        let this = data(&mut e);
        e.mem.set_u32(this + 0x2c, 0x1234);
        assert_eq!(e.call(0x00a6_6960, &args![this, 5u32]).u32(), 0x1234);
    }

    #[test]
    fn set_consistency_keeps_bits_8fff_unless_ready() {
        let mut e = engine();
        let this = data(&mut e);
        e.mem.set_u16(this + 0xe, 0xffff);
        e.call(0x00a6_7050, &args![this, 0x0001_2000u32]);
        assert_eq!(e.mem.u16(this + 0xe), 0xafff);
        let extra = additional(&mut e, 1);
        e.mem.set_u32(this + 0x30, extra);
        e.mem.set_u32(READY, 1);
        e.call(0x00a6_7050, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + 0xe), 0xafff);
    }

    #[test]
    fn get_group_and_active_vertex_count() {
        let mut e = engine();
        e.mem.set_u32(GEOMETRY_GROUP, 0x5555);
        assert_eq!(e.call(0x00a6_69c0, &args![]).u32(), 0x5555);
        let this = data(&mut e);
        e.mem.set_u16(this + 8, 321);
        assert_eq!(e.call(0x00a6_69e0, &args![this]).u16(), 321);
    }

    #[test]
    fn replace_data_stores_arrays_and_flags() {
        let mut e = engine();
        let this = data(&mut e);
        e.mem.set_u16(this + 0xc, 0xf00f);
        e.call(
            0x00a6_6bb0,
            &args![
                this,
                0x1_0005u32,
                1u32,
                2u32,
                3u32,
                4u32,
                3u32,
                0x0001_0010u32
            ],
        );
        assert_eq!(e.mem.u16(this + 8), 5);
        for (i, offset) in [0x20u32, 0x24, 0x28, 0x2c].iter().enumerate() {
            assert_eq!(e.mem.u32(this + offset), i as u32 + 1);
        }
        assert_eq!(e.mem.u16(this + 0xc), 0x001f);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_FREE).len(), 0);
    }

    #[test]
    fn constructor_with_data_takes_an_id_and_computes_the_bound() {
        let mut e = engine();
        e.mem.set_u16(NEXT_ID, 0xffff);
        let this = e.mem.alloc(0x40);
        e.mem.write(this + 4, &[0xcc; 0x3c]);
        let back = e.call(
            0x00a6_7b20,
            &args![this, 3u32, 0x100u32, 0x200u32, 0x300u32, 0x400u32, 1u32, 0x40u32],
        );
        assert_eq!(back.u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + 8), 3);
        assert_eq!(e.mem.u16(this + 0xa), 0xffff);
        assert_eq!(e.mem.u16(NEXT_ID), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0x41);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(
            calls_to(&e, BOUND_COMPUTE_FROM_DATA),
            vec![vec![this + 0x10, 3, 0x100]]
        );
        // No vertices: no bound.
        let other = e.mem.alloc(0x40);
        e.call(
            0x00a6_7b20,
            &args![other, 3u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls_to(&e, BOUND_COMPUTE_FROM_DATA).len(), 1);
        assert_eq!(e.mem.u16(other + 0xa), 0);
    }

    #[test]
    fn replace_and_release_frees_changed_arrays_without_a_group() {
        let mut e = engine();
        let this = data(&mut e);
        for (i, offset) in [0x20u32, 0x24, 0x28, 0x2c].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x10 * (i as u32 + 1));
        }
        e.mem.set_u16(this + 0xc, 0xffff);
        e.call(
            0x00a6_70c0,
            &args![this, 4u32, 0x10u32, 0x99u32, 0x30u32, 0x98u32, 0u32, 0x1000u32],
        );
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, 0x20],
                vec![MEMORY_MANAGER_OBJECT, 0x40]
            ]
        );
        assert_eq!(
            calls_to(&e, BOUND_COMPUTE_FROM_DATA),
            vec![vec![this + 0x10, 4, 0x10]]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0x99);
        assert_eq!(e.mem.u32(this + 0x2c), 0x98);
        assert_eq!(e.mem.u16(this + 0xc), 0x1ffe);
    }

    #[test]
    fn replace_and_release_gives_up_group_uses() {
        let mut e = engine();
        let this = data(&mut e);
        let group = group(&mut e, 0, 2);
        e.mem.set_u32(GROUP_ANSWER, group);
        e.mem.set_u32(this + 0x20, 0x10);
        e.call(
            0x00a6_70c0,
            &args![this, 0u32, 0x11u32, 1u32, 0u32, 0u32, 1u32, 0u32],
        );
        // Two arrays changed: the group's count reaches zero and it is deleted.
        assert_eq!(e.mem.u32(group + 0xc), 0);
        assert_eq!(calls_to(&e, GROUP_DESTRUCT_00A8A670), vec![vec![group]]);
        assert_eq!(calls_to(&e, SIZED_DELETE_00AA1460), vec![vec![group, 0x10]]);
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        assert!(calls_to(&e, BOUND_COMPUTE_FROM_DATA).is_empty());
        assert_eq!(e.mem.u16(this + 0xc), 1);
    }

    #[test]
    fn destructor_releases_the_additional_data_and_arrays() {
        let mut e = engine();
        let this = data(&mut e);
        let extra = additional(&mut e, 1);
        e.mem.set_u32(this + 0x30, extra);
        e.mem.set_u32(this + 0x24, 0x44);
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, STREAM_VTABLE);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        e.call(0x00a6_7c70, &args![this]);
        assert_eq!(
            calls_to(&e, RENDERER_SLOT_DOUBLE),
            vec![vec![renderer, this]]
        );
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![extra]]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_FREE).len(), 4);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCT_00A5D3D0), vec![vec![this]]);

        // With a group: one use per set array.
        let this = data(&mut e);
        e.mem.set_u32(RENDERER_011F4748, 0);
        let group = group(&mut e, 0, 5);
        e.mem.set_u32(GEOMETRY_GROUP, group);
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_u32(this + 0x2c, 1);
        e.call(0x00a6_7c70, &args![this]);
        assert_eq!(e.mem.u32(group + 0xc), 3);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_FREE).len(), 4);
    }

    #[test]
    fn register_streamables_asks_the_additional_data() {
        let mut e = engine();
        let this = data(&mut e);
        assert!(e.call(0x00a6_6f50, &args![this, 0x900u32]).bool());
        let extra = additional(&mut e, 1);
        e.mem.set_u32(this + 0x30, extra);
        assert!(!e.call(0x00a6_6f50, &args![this, 0x900u32]).bool());
        e.mem.set_u32(ADDITIONAL_REGISTER, 1);
        assert!(e.call(0x00a6_6f50, &args![this, 0x900u32]).bool());
        assert_eq!(calls_to(&e, REGISTER_DOUBLE).len(), 2);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00a6_6f50, &args![this, 0x900u32]).bool());
        assert_eq!(calls_to(&e, REGISTER_DOUBLE).len(), 2);
    }

    #[test]
    fn link_object_takes_the_linked_additional_data_from_10_3_0_7() {
        let mut e = engine();
        let this = data(&mut e);
        let extra = additional(&mut e, 0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| extra.into_ret());
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a03_0006);
        e.call(0x00a6_77e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a03_0007);
        e.call(0x00a6_77e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x30), extra);
        assert_eq!(e.mem.u32(extra + 4), 1);
        assert_eq!(calls_to(&e, NI_OBJECT_BASE_STEP_00A89AF0).len(), 2);
    }

    #[test]
    fn set_additional_data_swaps_references() {
        let mut e = engine();
        let this = data(&mut e);
        let old = additional(&mut e, 1);
        let new = additional(&mut e, 0);
        e.mem.set_u32(this + 0x30, old);
        e.call(0x00a6_7260, &args![this, new]);
        assert_eq!(e.mem.u32(this + 0x30), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![old]]);
    }

    #[test]
    fn lock_and_unlock_the_vertex_buffers() {
        let mut e = engine();
        let this = data(&mut e);
        // No additional data: locked for writing.
        assert!(e.call(0x00a6_7360, &args![this, 0x101u32]).bool());
        assert_eq!((e.mem.u8(this + 0x3a), e.mem.u8(this + 0x3b)), (1, 1));
        assert!(!e.call(0x00a6_7360, &args![this, 0u32]).bool());
        // Unlock of a write lock with ready data: nothing called.
        let extra = additional(&mut e, 1);
        e.mem.set_u32(this + 0x30, extra);
        e.mem.set_u32(READY, 1);
        e.call(0x00a6_73d0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x3a), 0);
        assert!(calls_to(&e, ADDITIONAL_DATA_00A725D0).is_empty());
        // Read lock with ready data: 00a72590(0, &zero); unlock: 00a725d0(0).
        assert!(e.call(0x00a6_7360, &args![this, 0u32]).bool());
        let read_lock = calls_to(&e, ADDITIONAL_DATA_00A72590);
        assert_eq!(read_lock.len(), 1);
        assert_eq!(read_lock[0][..2], [extra, 0]);
        e.call(0x00a6_73d0, &args![this]);
        assert_eq!(calls_to(&e, ADDITIONAL_DATA_00A725D0), vec![vec![extra, 0]]);
        // Write lock with ready data: 00a725d0(0).
        assert!(e.call(0x00a6_7360, &args![this, 1u32]).bool());
        assert_eq!(calls_to(&e, ADDITIONAL_DATA_00A725D0).len(), 2);
        // Unlock while not locked: nothing.
        e.mem.set_u8(this + 0x3a, 0);
        e.call(0x00a6_73d0, &args![this]);
        assert_eq!(calls_to(&e, ADDITIONAL_DATA_00A725D0).len(), 2);
    }

    #[test]
    fn locked_arrays_come_from_the_additional_data_or_the_fields() {
        let mut e = engine();
        let this = data(&mut e);
        e.mem.set_u32(this + 0x20, 0x1000);
        e.mem.set_u32(this + 0x2c, 0x2000);
        let out = e.mem.alloc(0xc);
        e.mem.write(out, &[0xee; 0xc]);
        // Not locked: nothing written.
        e.call(0x00a6_7410, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0xeeee_eeee);
        e.mem.set_u8(this + 0x3a, 1);
        e.call(0x00a6_7410, &args![this, out]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x1000, 0xc));
        assert_eq!(e.mem.u8(out + 8), 0);
        e.call(0x00a6_7720, &args![this, 3u32, out]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x2000, 8));
        // Ready additional data that gives an array.
        let extra = additional(&mut e, 1);
        e.mem.set_u32(extra + 0x2c, 3);
        e.mem.set_u32(this + 0x30, extra);
        e.mem.set_u32(READY, 1);
        e.register(ADDITIONAL_DATA_00A72720, |e, a| {
            e.mem.set_u32(a[2], 0x7000 + a[1]);
            e.mem.set_u32(a[7], 0x24);
            Ret::default()
        });
        e.call(0x00a6_7410, &args![this, out]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x7000, 0x24));
        e.call(0x00a6_7720, &args![this, 0u32, out]);
        // Stream 2 + 2 * 1 + 1.
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x7005, 0x24));
        let asked = calls_to(&e, ADDITIONAL_DATA_00A72720);
        assert_eq!(asked[0][0], extra);
        assert_eq!(asked[0].len(), 8);
    }

    /// A stream object whose input and output streams use the read and write doubles: reads
    /// serve `chunks` in order; both log (bytes, element size, first byte or count).
    fn stream(e: &mut Engine, version: u32, header: u32, chunks: Vec<Vec<u8>>) -> (u32, Log) {
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream, STREAM_VTABLE);
        e.mem.set_u32(stream + 4, header);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_READ, READ_DOUBLE);
        e.mem.set_u32(binary + BINARY_WRITE, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        let log: Log = Rc::default();
        let chunks: Chunks = Rc::new(RefCell::new(chunks));
        let reads = log.clone();
        e.register_double(READ_DOUBLE, move |e, a| {
            reads.borrow_mut().push((a[2], e.mem.u32(a[3]), a[4]));
            let chunk = chunks.borrow_mut().remove(0);
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            a[2].into_ret()
        });
        let writes = log.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            let first = u32::from(e.mem.u8(a[1]));
            writes.borrow_mut().push((a[2], e.mem.u32(a[3]), first));
            a[2].into_ret()
        });
        (stream, log)
    }

    #[test]
    fn write_texture_coordinates_writes_eight_bytes_each() {
        let mut e = engine();
        let (stream, log) = stream(&mut e, 0x1404_0000, 0, vec![]);
        let textures = e.mem.alloc(0x20);
        e.mem.set_u8(textures, 0x5a);
        assert_eq!(
            e.call(0x00a6_7ae0, &args![stream, textures, 3u32]).u32(),
            24
        );
        assert_eq!(*log.borrow(), vec![(24, 4, 0x5a)]);
        assert_eq!(e.mem.u32(e.mem.u32(stream + STREAM_OUTPUT) + 4), 24);
    }

    #[test]
    fn load_binary_of_a_current_file_reads_every_array() {
        let mut e = engine();
        let chunks = vec![
            vec![7, 0, 0, 0], // group id
            vec![2, 0],       // vertex count
            vec![0x11],       // keep flags
            vec![0x22],       // compress flags
            vec![1],          // has vertices
            vec![1; 24],      // vertices
            vec![0x01, 0x10], // data flags (texture bit, a bit of 0xf000)
            vec![1],          // has normals
            vec![2; 72],      // normals (three times)
            vec![1],          // has colours
            vec![3; 32],      // colours
            vec![4; 16],      // textures
            vec![0x34, 0x12], // dirty flags
        ];
        let (stream, log) = stream(&mut e, 0x1404_0000, 0x22, chunks);
        let this = data(&mut e);
        e.mem.set_u8(this + 0x3c, 1);
        e.call(0x00a6_7ea0, &args![this, stream]);
        assert_eq!(e.mem.u32(GEOMETRY_GROUP), 0x107);
        assert_eq!(e.mem.u16(this + 8), 2);
        assert_eq!((e.mem.u8(this + 0x38), e.mem.u8(this + 0x39)), (0x11, 0x22));
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x20) + 23), 1);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x24) + 71), 2);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x28) + 31), 3);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x2c) + 15), 4);
        assert_eq!(e.mem.u16(this + 0xe), 0x1234);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR)[0][1..],
            [0x10, 2, COLOR_CONSTRUCT_004A7800]
        );
        assert_eq!(
            calls_to(&e, SPHERE_LOAD_BINARY),
            vec![vec![this + 0x10, stream]]
        );
        assert_eq!(calls_to(&e, STREAM_LINK_STEP_00A645B0), vec![vec![stream]]);
        let sizes: Vec<(u32, u32)> = log.borrow().iter().map(|r| (r.0, r.1)).collect();
        assert_eq!(
            sizes,
            vec![
                (4, 4),
                (2, 2),
                (1, 1),
                (1, 1),
                (1, 1),
                (24, 4),
                (2, 2),
                (1, 1),
                (72, 4),
                (1, 1),
                (32, 4),
                (16, 4),
                (2, 2)
            ]
        );
        assert!(calls_to(&e, WARNING_PRINT_00A29680).is_empty());
    }

    #[test]
    fn load_binary_of_an_old_file_uses_pointers_and_the_group() {
        let mut e = engine();
        let group = group(&mut e, 0x8000, 0);
        e.mem.set_u32(GROUP_ANSWER, group);
        let chunks = vec![
            vec![1, 0],       // vertex count
            vec![1, 0, 0, 0], // vertex pointer (present)
            vec![0, 0, 0, 0], // normal pointer (absent)
            vec![0, 0, 0, 0], // colour pointer (absent)
            vec![3, 0],       // data flags into the local: texture bit
            vec![9, 0, 0, 0], // texture pointer
        ];
        // Version 4.0.0.2, header version 0x1b: no contents are read, no sets counted.
        let (stream, log) = stream(&mut e, 0x0400_0002, 0x1b, chunks);
        let this = data(&mut e);
        e.call(0x00a6_7ea0, &args![this, stream]);
        assert_eq!(log.borrow().len(), 6);
        // Vertices zeroed through memset, both arrays from the group.
        assert_eq!(e.mem.u32(this + 0x20), 0x8000);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x8000, 0, 12]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x8000 + 12);
        assert_eq!(e.mem.u32(group + 8), 0x8000 + 12 + 8);
        assert_eq!(e.mem.u32(group + 0xc), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert!(calls_to(&e, STREAM_LINK_STEP_00A645B0).is_empty());
    }

    #[test]
    fn load_binary_warns_about_extra_texture_sets() {
        let mut e = engine();
        let chunks = vec![
            vec![2, 0],  // vertex count
            vec![0],     // no vertices
            vec![3, 0],  // data flags (10.0.0.2 on): three sets
            vec![0],     // no normals
            vec![0],     // no colours
            vec![5; 16], // first texture set
            vec![0, 0],  // dirty flags
        ];
        let (stream, _) = stream(&mut e, 0x0a00_0002, 0x1a, chunks);
        let this = data(&mut e);
        e.mem.set_u8(this + 0x3c, 1);
        e.call(0x00a6_7ea0, &args![this, stream]);
        let extra = calls_to(&e, READ_EXTRA_TEXTURE_SETS_00A67AA0);
        assert_eq!(extra.len(), 1);
        assert_eq!((extra[0][0], extra[0][2]), (stream, 4));
        assert_eq!(
            calls_to(&e, WARNING_PRINT_00A29680),
            vec![vec![
                EXTRA_TEXTURE_SETS_FORMAT,
                stream + STREAM_FILE_NAME,
                0
            ]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, extra[0][1]]]
        );
    }

    #[test]
    fn save_binary_writes_the_fields_in_order() {
        let mut e = engine();
        let (stream, log) = stream(&mut e, 0x1404_0000, 0, vec![]);
        let this = data(&mut e);
        e.mem.set_u16(this + 8, 2);
        e.mem.set_u16(this + 0xc, 0x1001);
        e.mem.set_u16(this + 0xe, 0xffff);
        let vertices = e.mem.alloc(24);
        let normals = e.mem.alloc(72);
        e.mem.set_u32(this + 0x20, vertices);
        e.mem.set_u32(this + 0x24, normals);
        e.mem.set_u32(this + 0x30, 0x4321);
        e.mem.set_u8(this + 0x3c, 1);
        e.call(0x00a6_8520, &args![this, stream]);
        let written: Vec<(u32, u32, u32)> = log.borrow().clone();
        assert_eq!(
            written,
            vec![
                (4, 4, 0x77),
                (2, 2, 2),
                (1, 1, 0),
                (1, 1, 0),
                (1, 1, 1),
                (24, 4, 0),
                (2, 2, 0),
                (1, 1, 1),
                (72, 4, 0),
                (1, 1, 0),
                (2, 2, 0)
            ]
        );
        // No texture array: bit 0 of the data flags is cleared before it is written.
        assert_eq!(e.mem.u16(this + 0xc), 0x1000);
        assert_eq!(calls_to(&e, SPHERE_SAVE_BINARY).len(), 1);
        assert_eq!(
            calls_to(&e, STREAM_LINK_SLOT_DOUBLE),
            vec![vec![stream, 0x4321]]
        );
    }

    #[test]
    fn save_binary_writes_texture_coordinates() {
        let mut e = engine();
        let (stream, log) = stream(&mut e, 0x1404_0000, 0, vec![]);
        let this = data(&mut e);
        e.mem.set_u16(this + 8, 3);
        e.mem.set_u16(this + 0xc, 1);
        e.mem.set_u16(this + 0xe, 0x7123);
        let textures = e.mem.alloc(24);
        e.mem.set_u32(this + 0x2c, textures);
        e.mem.set_u8(this + 0x3c, 1);
        e.call(0x00a6_8520, &args![this, stream]);
        let log = log.borrow();
        assert_eq!(log[log.len() - 2], (24, 4, 0));
        // The dirty flags' bits 0x7000, low byte first.
        assert_eq!(log[log.len() - 1], (2, 2, 0));
    }
}
