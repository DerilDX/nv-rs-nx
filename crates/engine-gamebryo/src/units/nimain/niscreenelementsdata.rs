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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
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
