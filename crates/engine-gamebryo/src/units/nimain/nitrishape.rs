//! `NiMain/nitrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriShape::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_SHAPE_RTTI: u32 = 0x011f_4a40;

// Translated from 00a74520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_SHAPE_RTTI)
}

// Translated from 00a74530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a89760`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0xc4 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00a74530(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_9760;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

/// `NiTriBasedGeom::NiTriBasedGeom` (engine map, `00a89720`): `thiscall`, one stack word.
const NI_TRI_BASED_GEOM_CONSTRUCT: u32 = 0x00a8_9720;
/// `NiTriShape`'s vtable (`0109d454`, RTTI `.?AVNiTriShape@@`).
const NI_TRI_SHAPE_VTABLE: u32 = 0x0109_d454;

// Translated from 00a74480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::NiTriShape_ov2` (Xbox PDB). `thiscall`, `RET 4`: runs the
/// `NiTriBasedGeom` constructor (`00a89720`) on `this` with the one stack word (the
/// geometry data), then puts this class's vtable (`0109d454`) at +0. Answers `this`.
pub fn ni_tri_shape_ni_tri_shape_ov2(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(NI_TRI_BASED_GEOM_CONSTRUCT, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_VTABLE);
    this
}

/// `00aa13e0` (`cdecl`, one word): the allocator `NiTriShape` uses for its data and clones.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `NiTriShapeData::NiTriShapeData` (engine map, `00a7b630`; `thiscall`, `RET 0x24`).
const NI_TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b630;
/// `NiGeometry::RenderImmediate` (engine map, `00a80610`; `thiscall`, `RET 4`).
const NI_GEOMETRY_RENDER_IMMEDIATE: u32 = 0x00a8_0610;
/// `00a89740` (`thiscall`, no stack words, unnamed in the engine map): the base constructor
/// `CreateClone` runs on the new object.
const CLONE_BASE_CONSTRUCT_00A89740: u32 = 0x00a8_9740;
/// `00a9b140` (`thiscall`, `RET 8`: the clone, the cloning process; the engine map's
/// `NiParticleMeshes::CopyMembers`, a folded name).
const COPY_MEMBERS_00A9B140: u32 = 0x00a9_b140;

// Translated from 00a74410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::NiTriShape` (Xbox PDB), `thiscall`, `RET 0x24`: allocates 0x58 bytes
/// (`00aa13e0`) and constructs `NiTriShapeData` there (`00a7b630`) with the nine words, then
/// runs the `NiTriBasedGeom` constructor (`00a89720`) with that data (null when the
/// allocation failed) and stores the vtable `0109d454`. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn ni_tri_shape_ni_tri_shape(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    textures: u32,
    texture_sets: u32,
    data_flags: u32,
    triangle_count: u32,
    triangles: u32,
) -> Ptr {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x58u32]).u32();
    let data = if block != 0 {
        e.call(
            NI_TRI_SHAPE_DATA_CONSTRUCT,
            &args![
                block,
                vertex_count,
                vertices,
                normals,
                colors,
                textures,
                texture_sets,
                data_flags,
                triangle_count,
                triangles
            ],
        )
        .u32()
    } else {
        0
    };
    e.call(NI_TRI_BASED_GEOM_CONSTRUCT, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_VTABLE);
    this
}

// Translated from 00a74600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::OnlyRenderImmediate` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiGeometry::RenderImmediate` (`00a80610`), then when the renderer's word +0x200 is 1 or
/// 2 and its byte +0x208 is 1, the renderer's slot +0x1b4 with `this`.
pub fn ni_tri_shape_only_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(NI_GEOMETRY_RENDER_IMMEDIATE, &args![this, renderer]);
    let mode = e.mem.u32(renderer.addr() + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(renderer.addr() + 0x208) == 1 {
        e.vcall(renderer.addr(), 0x1b4, &args![this]);
    }
}

// Translated from 00a74640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::CreateClone` (Xbox PDB), `thiscall`, `RET 4`: allocates 0xc4 bytes
/// (`00aa13e0`); when that succeeds runs the base constructor `00a89740` on it and stores
/// the vtable `0109d454`. Either way `00a9b140` copies the members into it (null when the
/// allocation failed). Answers the clone.
pub fn ni_tri_shape_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(ALLOCATE_00AA13E0, &args![0xc4u32]).u32();
    if block != 0 {
        e.call(CLONE_BASE_CONSTRUCT_00A89740, &args![block]);
        e.mem.set_u32(block, NI_TRI_SHAPE_VTABLE);
    }
    e.call(COPY_MEMBERS_00A9B140, &args![this, block, cloning]);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_4410,
            ni_tri_shape_ni_tri_shape(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a7_4600, ni_tri_shape_only_render_immediate(Ptr, Ptr)),
        entry!(0x00a7_4640, ni_tri_shape_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a74520, ni_tri_shape_get_rtti() -> Ptr),
        entry!(0x00a74530, fn_00a74530(Ptr, u32) -> Ptr),
        entry!(0x00a7_4480, ni_tri_shape_ni_tri_shape_ov2(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a74520, &args![]).u32(), NI_TRI_SHAPE_RTTI);
    }

    #[test]
    fn fn_00a74530_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_9760, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a74530, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a74530, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xc4]]);
        assert_eq!(calls_to(&e, 0x00a8_9760), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_tri_shape_ov2_constructs_the_base_with_the_data_then_sets_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.register(NI_TRI_BASED_GEOM_CONSTRUCT, |e, a| {
            // The base writes its own vtable; the class's replaces it afterwards.
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_4480, &args![this, 0x7000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_TRI_BASED_GEOM_CONSTRUCT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this, 0x7000]]);
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_VTABLE);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine(allocation: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(ALLOCATE_00AA13E0, move |_, _| allocation.into_ret());
        e.register(NI_TRI_SHAPE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_TRI_BASED_GEOM_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_GEOMETRY_RENDER_IMMEDIATE, |_, _| Ret::default());
        e.register(CLONE_BASE_CONSTRUCT_00A89740, |_, _| Ret::default());
        e.register(COPY_MEMBERS_00A9B140, |_, _| Ret::default());
        e.register(0x0e70_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_builds_the_data_from_the_nine_words() {
        let mut e = engine(0x6000);
        let this = e.mem.alloc(0xc4);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        assert_eq!(e.call(0x00a7_4410, &words).u32(), this);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x58]]);
        assert_eq!(
            calls(&e, NI_TRI_SHAPE_DATA_CONSTRUCT),
            vec![vec![0x6000, 1, 2, 3, 4, 5, 6, 7, 8, 9]]
        );
        assert_eq!(
            calls(&e, NI_TRI_BASED_GEOM_CONSTRUCT),
            vec![vec![this, 0x6000]]
        );
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_VTABLE);
        // A failed allocation: null data.
        let mut e = engine(0);
        let this = e.mem.alloc(0xc4);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        e.call(0x00a7_4410, &words);
        assert!(calls(&e, NI_TRI_SHAPE_DATA_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, NI_TRI_BASED_GEOM_CONSTRUCT), vec![vec![this, 0]]);
    }

    #[test]
    fn only_render_immediate_asks_renderers_in_modes_1_and_2() {
        let mut e = engine(0);
        let mut slots = vec![0u32; 0x70];
        slots[0x1b4 / 4] = 0x0e70_0000;
        e.put_vtable(0x0e71_0000, &slots);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, 0x0e71_0000);
        for (mode, flag, asked) in [(1u32, 1u8, 1usize), (2, 1, 2), (3, 1, 2), (1, 2, 2)] {
            e.mem.set_u32(renderer + 0x200, mode);
            e.mem.set_u8(renderer + 0x208, flag);
            e.call(0x00a7_4600, &args![0x5000u32, renderer]);
            assert_eq!(calls(&e, 0x0e70_0000).len(), asked, "{mode} {flag}");
        }
        assert_eq!(calls(&e, 0x0e70_0000)[0], vec![renderer, 0x5000]);
        assert_eq!(calls(&e, NI_GEOMETRY_RENDER_IMMEDIATE).len(), 4);
    }

    #[test]
    fn create_clone_copies_into_the_new_shape() {
        let mut e = engine(0);
        let block = e.mem.alloc(0xc4);
        e.register_double(ALLOCATE_00AA13E0, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00a7_4640, &args![0x5000u32, 0x77u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_TRI_SHAPE_VTABLE);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0xc4]]);
        assert_eq!(calls(&e, CLONE_BASE_CONSTRUCT_00A89740), vec![vec![block]]);
        assert_eq!(
            calls(&e, COPY_MEMBERS_00A9B140),
            vec![vec![0x5000, block, 0x77]]
        );
        // A failed allocation still copies, into null.
        let mut e = engine(0);
        assert_eq!(e.call(0x00a7_4640, &args![0x5000u32, 0x77u32]).u32(), 0);
        assert!(calls(&e, CLONE_BASE_CONSTRUCT_00A89740).is_empty());
        assert_eq!(
            calls(&e, COPY_MEMBERS_00A9B140),
            vec![vec![0x5000, 0, 0x77]]
        );
    }
}
