//! `NiMain/niscreenelementsdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiScreenElementsData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_ELEMENTS_DATA_RTTI: u32 = 0x011f_5b50;

// Translated from 00a87a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_elements_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_ELEMENTS_DATA_RTTI)
}

layout! {
    /// `NiScreenElementsData` (Xbox PDB), 0x70 bytes; the PC offsets match the PDB's. The
    /// fields the constructor sets.
    pub struct NiScreenElementsData: 0x70 {
        /// `m_usVertices` (Xbox PDB, `NiGeometryData`).
        0x08 m_usVertices: u16,
        /// `m_usDataFlags` (Xbox PDB, `NiGeometryData`): bit 0 set with texture sets.
        0x0C m_usDataFlags: u16,
        /// `m_pkVertex` (Xbox PDB, `NiGeometryData`): `NiPoint3*`.
        0x20 m_pkVertex: u32,
        /// `m_pkNormal` (Xbox PDB, `NiGeometryData`): `NiPoint3*`.
        0x24 m_pkNormal: u32,
        /// `m_pkColor` (Xbox PDB, `NiGeometryData`): `NiColorA*`.
        0x28 m_pkColor: u32,
        /// `m_pkTexture` (Xbox PDB, `NiGeometryData`): `NiPoint2*`.
        0x2C m_pkTexture: u32,
        /// `m_usTriangles` (Xbox PDB, `NiTriBasedGeomData`).
        0x40 m_usTriangles: u16,
        /// `m_uiTriListLength` (Xbox PDB, `NiTriShapeData`).
        0x44 m_uiTriListLength: u32,
        /// `m_pusTriList` (Xbox PDB, `NiTriShapeData`).
        0x48 m_pusTriList: u32,
        /// `m_akPolygon` (Xbox PDB): 8 bytes per polygon.
        0x58 m_akPolygon: u32,
        /// `m_ausPIndexer` (Xbox PDB): one `u16` per polygon.
        0x5C m_ausPIndexer: u32,
        /// `m_usMaxPQuantity` (Xbox PDB).
        0x60 m_usMaxPQuantity: u16,
        /// `m_usPGrowBy` (Xbox PDB).
        0x62 m_usPGrowBy: u16,
        /// `m_usPQuantity` (Xbox PDB).
        0x64 m_usPQuantity: u16,
        /// `m_usMaxVQuantity` (Xbox PDB).
        0x66 m_usMaxVQuantity: u16,
        /// `m_usVGrowBy` (Xbox PDB).
        0x68 m_usVGrowBy: u16,
        /// `m_usMaxIQuantity` (Xbox PDB).
        0x6A m_usMaxIQuantity: u16,
        /// `m_usIGrowBy` (Xbox PDB).
        0x6C m_usIGrowBy: u16,
        /// `m_bBoundNeedsUpdate` (Xbox PDB).
        0x6E m_bBoundNeedsUpdate: u8,
    }
}

/// `NiTriShapeData::NiTriShapeData_ov2` (engine map, `00a7b690`, `thiscall`, no stack words).
const NI_TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b690;
/// `NiScreenElementsData`'s vtable (`0109ea94`, RTTI `.?AVNiScreenElementsData@@`).
const NI_SCREEN_ELEMENTS_DATA_VTABLE: u32 = 0x0109_ea94;
/// The allocation `00aa1070` (`cdecl`, size).
const NEW_BLOCK: u32 = 0x00aa_1070;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// CRT `memset` (`00ec61c0`).
const MEMSET: u32 = 0x00ec_61c0;
/// The three `float`s each normal is the negation of (`011a9484`, `011a9488`, `011a948c`).
const NORMAL_011A9484: u32 = 0x011a_9484;

/// `count * size` as the compiler's overflow-guarded allocation size: all ones when the
/// product does not fit 32 bits (`MUL; SETO; NEG; OR`).
fn guarded_size(count: u32, size: u32) -> u32 {
    count.checked_mul(size).unwrap_or(u32::MAX)
}

/// `MemoryManager::Allocate` on the singleton.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

// Translated from 00a881e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::NiScreenElementsData` (Xbox PDB), `thiscall`, nine words
/// (`RET 0x24`): with normals, with colours (bytes), texture sets (`u16`), maximum polygons,
/// polygon grow-by, maximum vertices, vertex grow-by, maximum triangles, triangle grow-by.
/// Answers `this`.
///
/// Runs `NiTriShapeData`'s constructor and sets the vtable. Each maximum and grow-by below 1
/// becomes 1 (the triangle ones become 3 triangles; the others are multiplied by 3 as
/// indices) and is stored as 16 bits. The polygon array (8 bytes each) and indexer (2 bytes
/// each, filled with 0xff) come from `00aa1070`; the vertices (12 bytes each), the normals
/// (12 bytes each, every one the negation of the three `float`s at `011a9484`, `FLD`/`FCHS`/
/// `FSTP`), the colours (16 bytes each, cleared; null when the allocation fails) and the
/// texture coordinates (`vertices * sets * 8` bytes, bit 0 of `m_usDataFlags` set to that of
/// the set count) from the memory manager, null where not asked for. The triangle list
/// (`2 * 3 * maximum triangles` bytes) comes from `00aa1070`; the polygon, vertex and
/// triangle counts start at 0 and `m_bBoundNeedsUpdate` is cleared.
#[allow(clippy::too_many_arguments)] // the exe's nine stack words
pub fn ni_screen_elements_data_ni_screen_elements_data(
    e: &mut Engine,
    this: Ptr<NiScreenElementsData>,
    with_normals: u8,
    with_colors: u8,
    texture_sets: u16,
    max_polygons: i32,
    polygon_grow_by: i32,
    max_vertices: i32,
    vertex_grow_by: i32,
    max_triangles: i32,
    triangle_grow_by: i32,
) -> Ptr<NiScreenElementsData> {
    type Data = NiScreenElementsData;
    e.call(NI_TRI_SHAPE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_SCREEN_ELEMENTS_DATA_VTABLE);
    let max_polygons = if max_polygons > 0 { max_polygons } else { 1 };
    e.set(this, Data::m_usMaxPQuantity, max_polygons as u16);
    let polygon_grow_by = if polygon_grow_by > 0 {
        polygon_grow_by
    } else {
        1
    };
    e.set(this, Data::m_usPGrowBy, polygon_grow_by as u16);
    e.set(this, Data::m_usPQuantity, 0);
    let polygons = e
        .call(NEW_BLOCK, &args![u32::from(max_polygons as u16) * 8])
        .u32();
    e.set(this, Data::m_akPolygon, polygons);
    let indexer_bytes = u32::from(e.get(this, Data::m_usMaxPQuantity)) * 2;
    let indexer = e.call(NEW_BLOCK, &args![indexer_bytes]).u32();
    let indexer_bytes = u32::from(e.get(this, Data::m_usMaxPQuantity)) * 2;
    e.set(this, Data::m_ausPIndexer, indexer);
    e.call(MEMSET, &args![indexer, 0xffu32, indexer_bytes]);

    let max_vertices = if max_vertices > 0 { max_vertices } else { 1 };
    e.set(this, Data::m_usMaxVQuantity, max_vertices as u16);
    let vertex_grow_by = if vertex_grow_by > 0 {
        vertex_grow_by
    } else {
        1
    };
    e.set(this, Data::m_usVGrowBy, vertex_grow_by as u16);
    e.set(this, Data::m_usVertices, 0);
    let vertices = allocate(e, guarded_size(u32::from(max_vertices as u16), 0xc));
    e.set(this, Data::m_pkVertex, vertices);

    if with_normals != 0 {
        let count = e.get(this, Data::m_usMaxVQuantity);
        let normals = allocate(e, guarded_size(u32::from(count), 0xc));
        e.set(this, Data::m_pkNormal, normals);
        let mut index = 0u16;
        while index < e.get(this, Data::m_usMaxVQuantity) {
            let normal = e
                .get(this, Data::m_pkNormal)
                .wrapping_add(u32::from(index) * 0xc);
            for component in 0..3 {
                let value = x87_quiet_bits(e.global::<u32>(NORMAL_011A9484 + 4 * component));
                e.mem
                    .set_u32(normal.wrapping_add(4 * component), value ^ 0x8000_0000);
            }
            index = index.wrapping_add(1);
        }
    } else {
        e.set(this, Data::m_pkNormal, 0);
    }

    let mut colors = 0;
    if with_colors != 0 {
        let count = u32::from(e.get(this, Data::m_usMaxVQuantity));
        let block = allocate(e, guarded_size(count, 0x10));
        if block != 0 {
            // `count - 1` down to 0 (signed): one cleared colour per vertex.
            let mut left = count as i32 - 1;
            let mut colour = block;
            while left >= 0 {
                for component in 0..4 {
                    e.mem.set_u32(colour + 4 * component, 0);
                }
                colour = colour.wrapping_add(0x10);
                left -= 1;
            }
            colors = block;
        }
    }
    e.set(this, Data::m_pkColor, colors);

    if texture_sets != 0 {
        let count = u32::from(e.get(this, Data::m_usMaxVQuantity));
        let per_set = count.wrapping_mul(u32::from(texture_sets));
        let texture = allocate(e, guarded_size(per_set, 8));
        e.set(this, Data::m_pkTexture, texture);
        let flags = e.get(this, Data::m_usDataFlags);
        e.set(
            this,
            Data::m_usDataFlags,
            flags ^ ((flags ^ texture_sets) & 1),
        );
    } else {
        e.set(this, Data::m_pkTexture, 0);
    }

    let max_indices = if max_triangles > 0 {
        max_triangles.wrapping_mul(3)
    } else {
        3
    } as u16;
    e.set(this, Data::m_usMaxIQuantity, max_indices);
    let index_grow_by = if triangle_grow_by > 0 {
        triangle_grow_by.wrapping_mul(3)
    } else {
        3
    } as u16;
    e.set(this, Data::m_usIGrowBy, index_grow_by);
    e.set(this, Data::m_usTriangles, 0);
    e.set(this, Data::m_uiTriListLength, 0);
    let list = e.call(NEW_BLOCK, &args![u32::from(max_indices) * 2]).u32();
    e.set(this, Data::m_pusTriList, list);
    e.set(this, Data::m_bBoundNeedsUpdate, 0);
    this
}

/// The Ni free `00aa10f0` (`cdecl`, one word), the memory manager's free `00aa4060`
/// (`thiscall`, one stack word), the sized delete `00aa1460` (`cdecl`: block, size) and CRT
/// `memcpy` (`00ec44d0`).
const NI_FREE: u32 = 0x00aa_10f0;
const MEMORY_FREE: u32 = 0x00aa_4060;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiTriShapeData::~NiTriShapeData` (engine map, `00a7b8c0`), `NiBound::ComputeFromData`
/// (`00a7ee30`, `thiscall` on the bound, `RET 8`), `NiGeometryData::MarkAsChanged`
/// (`00a67090`, `RET 4`) and `NiGeometryData::GetTextureSet` (`00a66960`, `RET 4`).
const NI_TRI_SHAPE_DATA_DESTRUCT: u32 = 0x00a7_b8c0;
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;
const GET_TEXTURE_SET: u32 = 0x00a6_6960;
/// The `double` 0.5 `Insert` adds before truncating (`01011588`).
const HALF_01011588: u32 = 0x0101_1588;

/// The new capacity `Insert` grows to when `excess` (positive) more items are needed than
/// `maximum`: `(int)((float)(excess / grow_by) + 0.5)` (x87 `FIDIV`-free: two `FILD`, `FDIVP`,
/// `FSTP float`, `FLD`, `FADD double`, then `FISTP` under chop), plus one, times `grow_by`,
/// plus `maximum`, all in 16 bits.
fn grown_capacity(e: &Engine, excess: i32, grow_by: u16, maximum: u16) -> u16 {
    let quotient = x87_store32(x87_div(f64::from(excess), f64::from(grow_by)));
    let half = f64::from_bits(e.mem.u64(HALF_01011588));
    let blocks = crate::x87::fistp_chop(x87_add(x87_load32(quotient), half));
    (blocks as u16)
        .wrapping_add(1)
        .wrapping_mul(grow_by)
        .wrapping_add(maximum)
}

/// Copies `bytes` from `source` to `destination` (`memcpy`).
fn copy(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    e.call(MEMCPY, &args![destination, source, bytes]);
}

/// The memory manager's free on the singleton.
fn memory_free(e: &mut Engine, block: u32) {
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a87900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::UpdateBound` (Xbox PDB; `thiscall`, no stack words): clears
/// `m_bBoundNeedsUpdate` and, when there are vertices, computes the bound (+0x10) from them
/// (`00a7ee30`).
pub fn ni_screen_elements_data_update_bound(e: &mut Engine, this: Ptr<NiScreenElementsData>) {
    let count = e.get(this, NiScreenElementsData::m_usVertices);
    e.set(this, NiScreenElementsData::m_bBoundNeedsUpdate, 0);
    if count > 0 {
        let vertices = e.get(this, NiScreenElementsData::m_pkVertex);
        e.call(
            BOUND_COMPUTE_FROM_DATA,
            &args![this.addr() + 0x10, u32::from(count), vertices],
        );
    }
}

// Translated from 00a87aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`:
/// stores the vtable `0109ea94`, frees the polygon array and indexer (`00aa10f0`), runs
/// `~NiTriShapeData` (`00a7b8c0`) and, when bit 0 of `flags` is set, the sized delete of 0x70
/// bytes. Answers `this`.
pub fn ni_screen_elements_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiScreenElementsData>,
    flags: u32,
) -> Ptr<NiScreenElementsData> {
    let polygons = e.get(this, NiScreenElementsData::m_akPolygon);
    e.mem.set_u32(this.addr(), NI_SCREEN_ELEMENTS_DATA_VTABLE);
    e.call(NI_FREE, &args![polygons]);
    let indexer = e.get(this, NiScreenElementsData::m_ausPIndexer);
    e.call(NI_FREE, &args![indexer]);
    e.call(NI_TRI_SHAPE_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x70u32]);
    }
    this
}

// Translated from 00a87ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::Insert` (Xbox PDB), `thiscall`, `RET 0xc`: adds a polygon of
/// `vertex_count` vertices (16 bits; fewer than 3 answer -1) and `triangle_count` triangles
/// (16 bits; 0 means `vertex_count - 2`; given triangles need `indices`, else -1). The polygon
/// array and indexer grow by `m_usPGrowBy` when full (the new indexer entries 0xff); the
/// polygon (`{vertices, first vertex, triangles, first index}`) is written at the old
/// polygon count, whose number goes into the first free (0xffff) indexer slot, and that slot
/// is the answer (the polygon number when there is none). The vertex count grows; past
/// `m_usMaxVQuantity` the vertex, normal (new ones the negated `011a9484` triple), colour
/// (cleared) and texture arrays are reallocated to the grown capacity. The triangle counts
/// grow; past `m_usMaxIQuantity` the triangle list is reallocated. The polygon's indices are
/// `indices` offset by its first vertex, or a fan from its first vertex. Then
/// `MarkAsChanged(1)`.
pub fn ni_screen_elements_data_insert(
    e: &mut Engine,
    this: Ptr<NiScreenElementsData>,
    vertex_count: u32,
    triangle_count: u32,
    indices: u32,
) -> u32 {
    type D = NiScreenElementsData;
    let vertex_count = vertex_count as u16;
    if vertex_count < 3 {
        return u32::MAX;
    }
    let mut triangles = triangle_count;
    if triangles as u16 != 0 && indices == 0 {
        return u32::MAX;
    }
    if triangles as u16 == 0 {
        triangles = u32::from(vertex_count.wrapping_sub(2));
    }

    // Polygons.
    let polygon = e.get(this, D::m_usPQuantity);
    let mut handle = u32::from(polygon);
    let maximum = e.get(this, D::m_usMaxPQuantity);
    e.set(this, D::m_usPQuantity, polygon.wrapping_add(1));
    if polygon.wrapping_add(1) > maximum {
        let grown = e.get(this, D::m_usPGrowBy).wrapping_add(maximum);
        let capacity = u32::from(grown);
        let array = e.call(NEW_BLOCK, &args![capacity * 8]).u32();
        let old_bytes = u32::from(e.get(this, D::m_usMaxPQuantity)) * 8;
        let old = e.get(this, D::m_akPolygon);
        copy(e, array, old, old_bytes);
        let old = e.get(this, D::m_akPolygon);
        e.call(NI_FREE, &args![old]);
        e.set(this, D::m_akPolygon, array);
        let indexer = e.call(NEW_BLOCK, &args![capacity * 2]).u32();
        let old_count = u32::from(e.get(this, D::m_usMaxPQuantity));
        let old = e.get(this, D::m_ausPIndexer);
        copy(e, indexer, old, old_count * 2);
        let old_count = u32::from(e.get(this, D::m_usMaxPQuantity));
        e.call(
            MEMSET,
            &args![
                indexer.wrapping_add(old_count * 2),
                0xffu32,
                capacity.wrapping_sub(old_count).wrapping_mul(2)
            ],
        );
        let old = e.get(this, D::m_ausPIndexer);
        e.call(NI_FREE, &args![old]);
        e.set(this, D::m_ausPIndexer, indexer);
        e.set(this, D::m_usMaxPQuantity, grown);
    }
    let record = e
        .get(this, D::m_akPolygon)
        .wrapping_add(u32::from(handle as u16) * 8);
    e.mem.set_u16(record, vertex_count);
    let first_vertex = e.get(this, D::m_usVertices);
    e.mem.set_u16(record + 2, first_vertex);
    e.mem.set_u16(record + 4, triangles as u16);
    let first_index = e.get(this, D::m_usTriangles).wrapping_mul(3);
    e.mem.set_u16(record + 6, first_index);
    let mut slot = 0u16;
    while slot < e.get(this, D::m_usPQuantity) {
        let indexer = e.get(this, D::m_ausPIndexer);
        let at = indexer.wrapping_add(u32::from(slot) * 2);
        if e.mem.u16(at) == 0xffff {
            e.mem.set_u16(at, handle as u16);
            handle = u32::from(slot);
            break;
        }
        slot = slot.wrapping_add(1);
    }

    // Vertices.
    let added = e.mem.u16(record);
    let vertices = e.get(this, D::m_usVertices).wrapping_add(added);
    e.set(this, D::m_usVertices, vertices);
    let maximum = e.get(this, D::m_usMaxVQuantity);
    let excess = i32::from(vertices) - i32::from(maximum);
    if excess > 0 {
        let grow_by = e.get(this, D::m_usVGrowBy);
        let capacity = grown_capacity(e, excess, grow_by, maximum);
        let count = u32::from(capacity);
        let block = allocate(e, guarded_size(count, 0xc));
        let old_bytes = u32::from(e.get(this, D::m_usMaxVQuantity)) * 0xc;
        let old = e.get(this, D::m_pkVertex);
        copy(e, block, old, old_bytes);
        let old = e.get(this, D::m_pkVertex);
        memory_free(e, old);
        e.set(this, D::m_pkVertex, block);
        if e.get(this, D::m_pkNormal) != 0 {
            let block = allocate(e, guarded_size(count, 0xc));
            let old = e.get(this, D::m_pkNormal);
            copy(e, block, old, old_bytes);
            let old = e.get(this, D::m_pkNormal);
            memory_free(e, old);
            e.set(this, D::m_pkNormal, block);
            let first = e.get(this, D::m_usMaxVQuantity);
            if first < capacity {
                let normals = e.get(this, D::m_pkNormal);
                let mut at = normals.wrapping_add(u32::from(first) * 0xc);
                let mut left = capacity.wrapping_sub(first);
                loop {
                    for offset in [0u32, 4, 8] {
                        // FLD; FCHS; FSTP: the negated value (a signalling NaN quieted).
                        let bits = e.mem.u32(NORMAL_011A9484 + offset);
                        let negated = x87_store32(-x87_load32(bits));
                        e.mem.set_u32(at + offset, negated);
                    }
                    at = at.wrapping_add(0xc);
                    left = left.wrapping_sub(1);
                    if left == 0 {
                        break;
                    }
                }
            }
        }
        if e.get(this, D::m_pkColor) != 0 {
            let block = allocate(e, guarded_size(count, 0x10));
            if block != 0 {
                let zero = 0.0f32.to_bits();
                for index in 0..count {
                    for offset in [0u32, 4, 8, 0xc] {
                        e.mem.set_u32(block + index * 0x10 + offset, zero);
                    }
                }
            }
            let old_bytes = u32::from(e.get(this, D::m_usMaxVQuantity)) << 4;
            let old = e.get(this, D::m_pkColor);
            copy(e, block, old, old_bytes);
            let old = e.get(this, D::m_pkColor);
            memory_free(e, old);
            e.set(this, D::m_pkColor, block);
        }
        let sets = u32::from(e.get(this, D::m_usDataFlags) & 1);
        if sets != 0 {
            let block = allocate(e, guarded_size(sets.wrapping_mul(count), 8));
            let set_bytes = u32::from(e.get(this, D::m_usMaxVQuantity)) * 8;
            for set in 0..sets {
                let offset = u32::from(e.get(this, D::m_usVertices))
                    .wrapping_mul(set)
                    .wrapping_mul(8);
                let textures = e.get(this, D::m_pkTexture);
                copy(
                    e,
                    offset.wrapping_add(block),
                    textures.wrapping_add(offset),
                    set_bytes,
                );
            }
            let old = e.get(this, D::m_pkTexture);
            memory_free(e, old);
            e.set(this, D::m_pkTexture, block);
        }
        e.set(this, D::m_usMaxVQuantity, capacity);
    }

    // Triangles.
    let total = e.get(this, D::m_usTriangles).wrapping_add(triangles as u16);
    e.set(this, D::m_usTriangles, total);
    let index_count = u32::from(triangles as u16) * 3;
    let length = e.get(this, D::m_uiTriListLength).wrapping_add(index_count);
    e.set(this, D::m_uiTriListLength, length);
    let maximum = e.get(this, D::m_usMaxIQuantity);
    let excess = i32::from(total) * 3 - i32::from(maximum);
    if excess > 0 {
        let grow_by = e.get(this, D::m_usIGrowBy);
        let capacity = grown_capacity(e, excess, grow_by, maximum);
        let list = e.call(NEW_BLOCK, &args![u32::from(capacity) * 2]).u32();
        let old_bytes = u32::from(e.get(this, D::m_usMaxIQuantity)) * 2;
        let old = e.get(this, D::m_pusTriList);
        copy(e, list, old, old_bytes);
        let old = e.get(this, D::m_pusTriList);
        e.call(NI_FREE, &args![old]);
        e.set(this, D::m_pusTriList, list);
        e.set(this, D::m_usMaxIQuantity, capacity);
    }

    // The polygon's indices.
    let first_index = u32::from(e.mem.u16(record + 6));
    let mut destination = e.get(this, D::m_pusTriList).wrapping_add(first_index * 2);
    if indices != 0 {
        copy(e, destination, indices, u32::from(triangles as u16) * 6);
        let distance = indices.wrapping_sub(destination);
        let mut left = index_count;
        while left != 0 {
            let value = e
                .mem
                .u16(distance.wrapping_add(destination))
                .wrapping_add(e.mem.u16(record + 2));
            destination = destination.wrapping_add(2);
            left -= 1;
            e.mem.set_u16(destination.wrapping_sub(2), value);
        }
    } else {
        let mut index = 0u16;
        while index < triangles as u16 {
            let first = e.mem.u16(record + 2);
            e.mem.set_u16(destination, first);
            let second = e.mem.u16(record + 2).wrapping_add(index).wrapping_add(1);
            e.mem.set_u16(destination + 2, second);
            let third = e.mem.u16(record + 2).wrapping_add(index).wrapping_add(2);
            e.mem.set_u16(destination + 4, third);
            index = index.wrapping_add(1);
            destination = destination.wrapping_add(6);
        }
    }
    e.call(MARK_AS_CHANGED, &args![this, 1u32]);
    u32::from(handle as u16)
}

// Translated from 00a88020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::SetRectangle` (Xbox PDB), `thiscall`, `RET 0x14`: for a valid
/// polygon handle (0 to `m_usMaxPQuantity - 1`, whose indexer entry is not 0xffff) of a
/// polygon of four vertices, writes the corners `(x, y, 0)`, `(x, y + h, 0)`,
/// `(x + w, y + h, 0)`, `(x + w, y, 0)` (the sums rounded to `float` first, x87) at its first
/// vertex, `MarkAsChanged(1)`, sets `m_bBoundNeedsUpdate` and answers true; false otherwise.
pub fn ni_screen_elements_data_set_rectangle(
    e: &mut Engine,
    this: Ptr<NiScreenElementsData>,
    handle: i32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> bool {
    type D = NiScreenElementsData;
    let Some(record) = rectangle_polygon(e, this, handle) else {
        return false;
    };
    let first = u32::from(e.mem.u16(record + 2));
    let x0 = x87_load32(x.to_bits());
    let right = x87_store32(x87_add(x0, x87_m32(width.to_bits())));
    let y0 = x87_load32(y.to_bits());
    let bottom = x87_store32(x87_add(y0, x87_m32(height.to_bits())));
    let x_bits = x87_store32(x0);
    let y_bits = x87_store32(y0);
    let zero = 0.0f32.to_bits();
    let corners = [
        [x_bits, y_bits, zero],
        [x_bits, x87_store32(x87_load32(bottom)), zero],
        [
            x87_store32(x87_load32(right)),
            x87_store32(x87_load32(bottom)),
            zero,
        ],
        [x87_store32(x87_load32(right)), y_bits, zero],
    ];
    for (corner, values) in corners.iter().enumerate() {
        let at = (first + corner as u32) * 0xc;
        for (component, value) in values.iter().enumerate() {
            let vertices = e.get(this, D::m_pkVertex);
            e.mem
                .set_u32(vertices.wrapping_add(at + 4 * component as u32), *value);
        }
    }
    e.call(MARK_AS_CHANGED, &args![this, 1u32]);
    e.set(this, D::m_bBoundNeedsUpdate, 1);
    true
}

/// The polygon record of `handle` when it is valid and has four vertices: `handle` in
/// `0..m_usMaxPQuantity`, its indexer entry not 0xffff.
fn rectangle_polygon(e: &Engine, this: Ptr<NiScreenElementsData>, handle: i32) -> Option<u32> {
    type D = NiScreenElementsData;
    if handle < 0 || handle >= i32::from(e.get(this, D::m_usMaxPQuantity)) {
        return None;
    }
    let indexer = e.get(this, D::m_ausPIndexer);
    let polygon = e.mem.u16(indexer.wrapping_add(handle as u32 * 2));
    if polygon == 0xffff {
        return None;
    }
    let record = e
        .get(this, D::m_akPolygon)
        .wrapping_add(u32::from(polygon) * 8);
    (e.mem.u16(record) == 4).then_some(record)
}

// Translated from 00a88130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::SetTextures` (Xbox PDB), `thiscall`, `RET 0x18`: with texture
/// coordinates (+0x2c), for a valid four-vertex polygon handle and a texture set (low 16
/// bits) below the set count (bit 0 of `m_usDataFlags`), writes the coordinates
/// `(left, top)`, `(left, bottom)`, `(right, bottom)`, `(right, top)` (each `FLD`/`FST`, a
/// signalling NaN quieted) at the polygon's first vertex of `GetTextureSet(set)`,
/// `MarkAsChanged(8)` and answers true; false otherwise.
#[allow(clippy::too_many_arguments)]
pub fn ni_screen_elements_data_set_textures(
    e: &mut Engine,
    this: Ptr<NiScreenElementsData>,
    handle: i32,
    set: u32,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
) -> bool {
    type D = NiScreenElementsData;
    if e.get(this, D::m_pkTexture) == 0 {
        return false;
    }
    let Some(record) = rectangle_polygon(e, this, handle) else {
        return false;
    };
    if set as u16 >= e.get(this, D::m_usDataFlags) & 1 {
        return false;
    }
    let textures = e.call(GET_TEXTURE_SET, &args![this, set]).u32();
    let first = u32::from(e.mem.u16(record + 2));
    let at = textures.wrapping_add(first * 8);
    let (left, top, right, bottom) = (
        x87_quiet_bits(left.to_bits()),
        x87_quiet_bits(top.to_bits()),
        x87_quiet_bits(right.to_bits()),
        x87_quiet_bits(bottom.to_bits()),
    );
    for (offset, value) in [
        (0u32, left),
        (4, top),
        (8, left),
        (0xc, bottom),
        (0x10, right),
        (0x18, right),
        (0x14, bottom),
        (0x1c, top),
    ] {
        e.mem.set_u32(at + offset, value);
    }
    e.call(MARK_AS_CHANGED, &args![this, 8u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_7900,
            ni_screen_elements_data_update_bound(Ptr<NiScreenElementsData>)
        ),
        entry!(
            0x00a8_7aa0,
            ni_screen_elements_data_scalar_deleting_destructor(
                Ptr<NiScreenElementsData>,
                u32,
            )
                -> Ptr<NiScreenElementsData>
        ),
        entry!(
            0x00a8_7ae0,
            ni_screen_elements_data_insert(Ptr<NiScreenElementsData>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a8_8020,
            ni_screen_elements_data_set_rectangle(
                Ptr<NiScreenElementsData>,
                i32,
                f32,
                f32,
                f32,
                f32,
            ) -> bool
        ),
        entry!(
            0x00a8_8130,
            ni_screen_elements_data_set_textures(
                Ptr<NiScreenElementsData>,
                i32,
                u32,
                f32,
                f32,
                f32,
                f32,
            ) -> bool
        ),
        entry!(0x00a87a90, ni_screen_elements_data_get_rtti() -> Ptr),
        entry!(
            0x00a8_81e0,
            ni_screen_elements_data_ni_screen_elements_data(
                Ptr<NiScreenElementsData>,
                u8,
                u8,
                u16,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            )
                -> Ptr<NiScreenElementsData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_elements_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a87a90, &args![]).u32(),
            NI_SCREEN_ELEMENTS_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.set_global(NORMAL_011A9484, 1.0f32);
        e.set_global(NORMAL_011A9484 + 4, -2.0f32);
        e.set_global(NORMAL_011A9484 + 8, 0x7f80_0001u32);
        e.register(NI_TRI_SHAPE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NEW_BLOCK, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1].max(4));
            // Garbage the constructor must overwrite where it fills the block.
            for i in 0..a[1] / 4 {
                e.mem.set_u32(block + 4 * i, 0xdddd_dddd);
            }
            block.into_ret()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e
    }

    fn sizes(log: &[(u32, Vec<u32>)], addr: u32, word: usize) -> Vec<u32> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w[word])
            .collect()
    }

    #[test]
    fn constructor_with_every_array() {
        let mut e = rig();
        let this: Ptr<NiScreenElementsData> = e.new_object();
        e.set(this, NiScreenElementsData::m_usDataFlags, 0xfff0);
        e.call_log = Some(vec![]);
        let back = e.call(
            0x00a8_81e0,
            &args![this, 1u32, 1u32, 3u32, 4i32, 2i32, 5i32, 6i32, 7i32, 8i32],
        );
        assert_eq!(back.u32(), this.addr());
        let log = e.call_log.take().unwrap();
        type Data = NiScreenElementsData;
        assert_eq!(e.mem.u32(this.addr()), NI_SCREEN_ELEMENTS_DATA_VTABLE);
        assert_eq!(e.get(this, Data::m_usMaxPQuantity), 4);
        assert_eq!(e.get(this, Data::m_usPGrowBy), 2);
        assert_eq!(e.get(this, Data::m_usMaxVQuantity), 5);
        assert_eq!(e.get(this, Data::m_usVGrowBy), 6);
        assert_eq!(e.get(this, Data::m_usMaxIQuantity), 21);
        assert_eq!(e.get(this, Data::m_usIGrowBy), 24);
        // Polygons (4 * 8), indexer (4 * 2), triangle list (21 * 2).
        assert_eq!(sizes(&log, NEW_BLOCK, 0), vec![32, 8, 42]);
        // Vertices, normals (5 * 12), colours (5 * 16), texture (5 * 3 * 8).
        assert_eq!(sizes(&log, MEMORY_ALLOCATE, 1), vec![60, 60, 80, 120]);
        let indexer = e.get(this, Data::m_ausPIndexer);
        assert_eq!(e.mem.u32(indexer), 0xffff_ffff);
        assert_eq!(e.mem.u32(indexer + 4), 0xffff_ffff);
        let normals = e.get(this, Data::m_pkNormal);
        for i in 0..5 {
            let normal = normals + 12 * i;
            assert_eq!(e.mem.f32(normal), -1.0);
            assert_eq!(e.mem.f32(normal + 4), 2.0);
            // FLD quiets the signalling NaN, FCHS flips its sign.
            assert_eq!(e.mem.u32(normal + 8), 0xffc0_0001);
        }
        let colors = e.get(this, Data::m_pkColor);
        for i in 0..20 {
            assert_eq!(e.mem.u32(colors + 4 * i), 0);
        }
        assert_ne!(e.get(this, Data::m_pkTexture), 0);
        assert_eq!(e.get(this, Data::m_usDataFlags), 0xfff1);
        assert_eq!(e.get(this, Data::m_usVertices), 0);
        assert_eq!(e.get(this, Data::m_usTriangles), 0);
        assert_eq!(e.get(this, Data::m_uiTriListLength), 0);
        assert_eq!(e.get(this, Data::m_bBoundNeedsUpdate), 0);
    }

    #[test]
    fn constructor_clamps_counts_and_leaves_out_unasked_arrays() {
        let mut e = rig();
        let this: Ptr<NiScreenElementsData> = e.new_object();
        type Data = NiScreenElementsData;
        e.set(this, Data::m_pkNormal, 0xdead);
        e.set(this, Data::m_pkColor, 0xdead);
        e.set(this, Data::m_pkTexture, 0xdead);
        e.set(this, Data::m_usDataFlags, 0x0001);
        e.call_log = Some(vec![]);
        e.call(
            0x00a8_81e0,
            &args![this, 0u32, 0u32, 0u32, 0i32, -1i32, 0i32, 0i32, 0i32, -5i32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, Data::m_usMaxPQuantity), 1);
        assert_eq!(e.get(this, Data::m_usPGrowBy), 1);
        assert_eq!(e.get(this, Data::m_usMaxVQuantity), 1);
        assert_eq!(e.get(this, Data::m_usVGrowBy), 1);
        assert_eq!(e.get(this, Data::m_usMaxIQuantity), 3);
        assert_eq!(e.get(this, Data::m_usIGrowBy), 3);
        assert_eq!(e.get(this, Data::m_pkNormal), 0);
        assert_eq!(e.get(this, Data::m_pkColor), 0);
        assert_eq!(e.get(this, Data::m_pkTexture), 0);
        assert_eq!(e.get(this, Data::m_usDataFlags), 0x0001);
        assert_eq!(sizes(&log, MEMORY_ALLOCATE, 1), vec![12]);
    }

    #[test]
    fn constructor_with_a_vertex_count_that_truncates_to_zero() {
        let mut e = rig();
        let this: Ptr<NiScreenElementsData> = e.new_object();
        type Data = NiScreenElementsData;
        e.call_log = Some(vec![]);
        e.call(
            0x00a8_81e0,
            &args![
                this,
                1u32,
                1u32,
                0u32,
                1i32,
                1i32,
                0x1_0000i32,
                1i32,
                1i32,
                1i32
            ],
        );
        let log = e.call_log.take().unwrap();
        // 0x10000 stores as 0: no normals written, colours kept (count - 1 is negative).
        assert_eq!(e.get(this, Data::m_usMaxVQuantity), 0);
        assert_eq!(sizes(&log, MEMORY_ALLOCATE, 1), vec![0, 0, 0]);
        assert_ne!(e.get(this, Data::m_pkColor), 0);
        let normals = e.get(this, Data::m_pkNormal);
        assert_eq!(e.mem.u32(normals), 0);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    type D = NiScreenElementsData;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u64(HALF_01011588, 0.5f64.to_bits());
        e.set_global(NORMAL_011A9484, 1.0f32);
        e.set_global(NORMAL_011A9484 + 4, -2.0f32);
        e.set_global(NORMAL_011A9484 + 8, 0x7f80_0001u32);
        for double in [
            NI_FREE,
            MEMORY_FREE,
            SIZED_DELETE,
            NI_TRI_SHAPE_DATA_DESTRUCT,
            BOUND_COMPUTE_FROM_DATA,
            MARK_AS_CHANGED,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(NEW_BLOCK, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMCPY, |e, a| {
            if a[2] != 0 {
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(a[0], &bytes);
            }
            a[0].into_ret()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// Screen elements data with room for `polygons` polygons, `vertices` vertices and
    /// `indices` indices (grow-bys 1, 4 and 6), with normals, colours and one texture set.
    fn screen(e: &mut Engine, polygons: u16, vertices: u16, indices: u16) -> Ptr<D> {
        let this: Ptr<D> = e.new_object();
        let polygon_array = e.mem.alloc(u32::from(polygons) * 8 + 8);
        let indexer = e.mem.alloc(u32::from(polygons) * 2 + 8);
        e.mem.write(indexer, &vec![0xff; usize::from(polygons) * 2]);
        e.set(this, D::m_akPolygon, polygon_array);
        e.set(this, D::m_ausPIndexer, indexer);
        e.set(this, D::m_usMaxPQuantity, polygons);
        e.set(this, D::m_usPGrowBy, 1);
        e.set(this, D::m_usMaxVQuantity, vertices);
        e.set(this, D::m_usVGrowBy, 4);
        e.set(this, D::m_usMaxIQuantity, indices);
        e.set(this, D::m_usIGrowBy, 6);
        for (field, size) in [
            (D::m_pkVertex, 12u32),
            (D::m_pkNormal, 12),
            (D::m_pkColor, 16),
            (D::m_pkTexture, 8),
        ] {
            let block = e.mem.alloc(u32::from(vertices) * size + 8);
            e.set(this, field, block);
        }
        let list = e.mem.alloc(u32::from(indices) * 2 + 8);
        e.set(this, D::m_pusTriList, list);
        e.set(this, D::m_usDataFlags, 1);
        this
    }

    #[test]
    fn update_bound_computes_from_the_vertices() {
        let mut e = engine();
        let this = screen(&mut e, 1, 4, 6);
        e.set(this, D::m_bBoundNeedsUpdate, 1);
        e.call(0x00a8_7900, &args![this]);
        assert_eq!(e.get(this, D::m_bBoundNeedsUpdate), 0);
        assert!(calls(&e, BOUND_COMPUTE_FROM_DATA).is_empty());
        e.set(this, D::m_usVertices, 4);
        e.call(0x00a8_7900, &args![this]);
        let vertices = e.get(this, D::m_pkVertex);
        assert_eq!(
            calls(&e, BOUND_COMPUTE_FROM_DATA),
            vec![vec![this.addr() + 0x10, 4, vertices]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_frees_and_deletes_on_bit_0() {
        let mut e = engine();
        let this = screen(&mut e, 1, 4, 6);
        let polygons = e.get(this, D::m_akPolygon);
        let indexer = e.get(this, D::m_ausPIndexer);
        assert_eq!(e.call(0x00a8_7aa0, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_SCREEN_ELEMENTS_DATA_VTABLE);
        assert_eq!(calls(&e, NI_FREE), vec![vec![polygons], vec![indexer]]);
        assert_eq!(
            calls(&e, NI_TRI_SHAPE_DATA_DESTRUCT),
            vec![vec![this.addr()]]
        );
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a8_7aa0, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this.addr(), 0x70]]);
    }

    #[test]
    fn insert_rejects_bad_polygons() {
        let mut e = engine();
        let this = screen(&mut e, 1, 4, 6);
        assert_eq!(
            e.call(0x00a8_7ae0, &args![this, 2u32, 0u32, 0u32]).u32(),
            u32::MAX
        );
        assert_eq!(
            e.call(0x00a8_7ae0, &args![this, 4u32, 2u32, 0u32]).u32(),
            u32::MAX
        );
        assert_eq!(e.get(this, D::m_usPQuantity), 0);
    }

    #[test]
    fn insert_fits_a_quad_without_growing() {
        let mut e = engine();
        let this = screen(&mut e, 2, 4, 6);
        assert_eq!(e.call(0x00a8_7ae0, &args![this, 4u32, 0u32, 0u32]).u32(), 0);
        let record = e.get(this, D::m_akPolygon);
        assert_eq!(
            [
                e.mem.u16(record),
                e.mem.u16(record + 2),
                e.mem.u16(record + 4),
                e.mem.u16(record + 6)
            ],
            [4, 0, 2, 0]
        );
        let indexer = e.get(this, D::m_ausPIndexer);
        assert_eq!(e.mem.u16(indexer), 0);
        assert_eq!(e.get(this, D::m_usVertices), 4);
        assert_eq!(e.get(this, D::m_usTriangles), 2);
        assert_eq!(e.get(this, D::m_uiTriListLength), 6);
        let list = e.get(this, D::m_pusTriList);
        let fan: Vec<u16> = (0..6).map(|i| e.mem.u16(list + 2 * i)).collect();
        assert_eq!(fan, vec![0, 1, 2, 0, 2, 3]);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(calls(&e, MARK_AS_CHANGED), vec![vec![this.addr(), 1]]);
    }

    #[test]
    fn insert_grows_every_array() {
        let mut e = engine();
        let this = screen(&mut e, 1, 4, 6);
        e.call(0x00a8_7ae0, &args![this, 4u32, 0u32, 0u32]);
        let old_normals = e.get(this, D::m_pkNormal);
        e.mem.set_u32(old_normals, 0x1234_5678);
        // A second quad with its own indices: two more vertices than room (grow by 4: 8),
        // six more indices (grow by 6: 12), one more polygon (2).
        let indices = e.mem.alloc(12);
        for (i, value) in [0u16, 2, 1, 1, 2, 3].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *value);
        }
        assert_eq!(
            e.call(0x00a8_7ae0, &args![this, 4u32, 2u32, indices]).u32(),
            1
        );
        assert_eq!(e.get(this, D::m_usMaxPQuantity), 2);
        let indexer = e.get(this, D::m_ausPIndexer);
        assert_eq!((e.mem.u16(indexer), e.mem.u16(indexer + 2)), (0, 1));
        // (8 - 4) / 4 = 1.0, + 0.5 truncated = 1: (1 + 1) * 4 + 4 = 12 vertices.
        assert_eq!(e.get(this, D::m_usMaxVQuantity), 12);
        let normals = e.get(this, D::m_pkNormal);
        assert_eq!(e.mem.u32(normals), 0x1234_5678);
        assert_eq!(e.mem.f32(normals + 4 * 12), -1.0);
        assert_eq!(e.mem.f32(normals + 4 * 12 + 4), 2.0);
        assert_eq!(e.mem.u32(normals + 4 * 12 + 8), 0xffc0_0001);
        assert_eq!(e.mem.u32(normals + 11 * 12 + 8), 0xffc0_0001);
        let colors = e.get(this, D::m_pkColor);
        assert_eq!(e.mem.u32(colors + 11 * 16 + 12), 0);
        // (12 - 6) / 6 = 1: (1 + 1) * 6 + 6 = 18 indices.
        assert_eq!(e.get(this, D::m_usMaxIQuantity), 18);
        let list = e.get(this, D::m_pusTriList);
        let all: Vec<u16> = (0..12).map(|i| e.mem.u16(list + 2 * i)).collect();
        assert_eq!(all, vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 5, 6, 7]);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, vec![144, 144, 192, 96]);
    }

    #[test]
    fn set_rectangle_writes_four_corners() {
        let mut e = engine();
        let this = screen(&mut e, 2, 8, 12);
        e.call(0x00a8_7ae0, &args![this, 3u32, 0u32, 0u32]);
        e.call(0x00a8_7ae0, &args![this, 4u32, 0u32, 0u32]);
        // A triangle polygon, a bad handle, an empty indexer slot: false.
        for handle in [0i32, -1, 2] {
            assert!(!e
                .call(
                    0x00a8_8020,
                    &args![this, handle, 1.0f32, 2.0f32, 3.0f32, 4.0f32]
                )
                .bool());
        }
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a8_8020,
                &args![this, 1i32, 1.0f32, 2.0f32, 3.0f32, 4.0f32]
            )
            .bool());
        let vertices = e.get(this, D::m_pkVertex);
        let corners: Vec<f32> = (0..12).map(|i| e.mem.f32(vertices + 36 + 4 * i)).collect();
        assert_eq!(
            corners,
            vec![1.0, 2.0, 0.0, 1.0, 6.0, 0.0, 4.0, 6.0, 0.0, 4.0, 2.0, 0.0]
        );
        assert_eq!(e.get(this, D::m_bBoundNeedsUpdate), 1);
        assert_eq!(calls(&e, MARK_AS_CHANGED), vec![vec![this.addr(), 1]]);
    }

    #[test]
    fn set_textures_writes_four_coordinates() {
        let mut e = engine();
        let this = screen(&mut e, 1, 4, 6);
        e.call(0x00a8_7ae0, &args![this, 4u32, 0u32, 0u32]);
        let textures = e.get(this, D::m_pkTexture);
        e.register_double(GET_TEXTURE_SET, move |_, _| textures.into_ret());
        // Set 1 is past the one set: false.
        assert!(!e
            .call(
                0x00a8_8130,
                &args![this, 0i32, 1u32, 0.25f32, 0.5f32, 0.75f32, 1.0f32]
            )
            .bool());
        e.call_log = Some(vec![]);
        let signalling = f32::from_bits(0x7f80_0002);
        assert!(e
            .call(
                0x00a8_8130,
                &args![this, 0i32, 0u32, 0.25f32, 0.5f32, 0.75f32, signalling]
            )
            .bool());
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(textures + 4 * i)).collect();
        let (l, t, r) = (0.25f32.to_bits(), 0.5f32.to_bits(), 0.75f32.to_bits());
        let b = 0x7fc0_0002;
        assert_eq!(words, vec![l, t, l, b, r, b, r, t]);
        assert_eq!(calls(&e, MARK_AS_CHANGED), vec![vec![this.addr(), 8]]);
        assert_eq!(calls(&e, GET_TEXTURE_SET), vec![vec![this.addr(), 0]]);
        // No texture array: false.
        e.set(this, D::m_pkTexture, 0);
        assert!(!e
            .call(
                0x00a8_8130,
                &args![this, 0i32, 0u32, 0.25f32, 0.5f32, 0.75f32, 1.0f32]
            )
            .bool());
    }
}
