//! `BSShader/particleshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// Code the shaders' `CreateConstantMaps` functions expand in place: a constant map made into a
/// shader's `NiPointer` and the entries added to it through its vtable. The shader units of
/// this collection call these helpers where their functions contain that code.
pub(crate) mod constant_maps {
    use crate::prelude::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::ni_pointer_assign;

    /// The Ni allocation `00aa13e0` (`cdecl`, size).
    pub(crate) const NI_ALLOCATE: u32 = 0x00aa_13e0;
    /// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map, `00b7e330`, `thiscall`,
    /// `RET 8`: renderer, program type).
    pub(crate) const CONSTANT_MAP_CONSTRUCT: u32 = 0x00b7_e330;
    /// The empty string `""` (`01011584`) the entries pass.
    pub(crate) const EMPTY_STRING_01011584: u32 = 0x0101_1584;

    /// A new constant map into the `NiPointer` at `slot`: 0x44 bytes from `00aa13e0`, the
    /// constructor run with the renderer word and the program type when the block is not
    /// null (a null block gives a null map), then the pointer assigned (inlined
    /// `NiPointer::operator=`).
    pub(crate) fn make_constant_map(e: &mut Engine, slot: u32, renderer_word: u32, program: u32) {
        let block = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
        let map = if block != 0 {
            let renderer = e.mem.u32(renderer_word);
            e.call(CONSTANT_MAP_CONSTRUCT, &args![block, renderer, program])
                .u32()
        } else {
            0
        };
        ni_pointer_assign(e, slot, map);
    }

    /// One entry added through the map's vtable slot +0x8 (ten words; `AddEntry` in the Xbox
    /// PDB's slot order, not confirmed for the PC build): the map is read again from `slot`.
    pub(crate) fn add_entry(e: &mut Engine, slot: u32, words: &[u32; 10]) {
        let map = e.mem.u32(slot);
        e.vcall(map, 0x8, words);
    }

    /// One entry added through the map's vtable slot +0x68 (eight words; the Xbox PDB has an
    /// `AddPredefinedEntry` there, not confirmed for the PC build).
    pub(crate) fn add_predefined_entry(e: &mut Engine, slot: u32, words: &[u32; 8]) {
        let map = e.mem.u32(slot);
        e.vcall(map, 0x68, words);
    }
}

use constant_maps::*;
// This unit's own helper, not the shared one of the same name in shader_inline.
use constant_maps::make_constant_map;

/// The particle shader's vtable (`010b80a8`; no RTTI locator before it).
const PARTICLE_VTABLE_010B80A8: u32 = 0x010b_80a8;
/// The shared `NiPointer` the destructor clears (`012003f4`).
const SHARED_POINTER_012003F4: u32 = 0x0120_03f4;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`, no stack words).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
/// The sized free `00aa1460` (`cdecl`: block, size).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

/// A pass reference dropped without clearing the pointer (the members' own destructors): when
/// the pass is not null its count at +0x64 goes down by one and the pass is deleted through
/// `00e80190` at 0.
pub(crate) fn pass_release(e: &mut Engine, pass: u32) {
    if pass != 0 {
        let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
        e.mem.set_u32(pass + 0x64, left);
        if left == 0 {
            e.call(PASS_DELETE_00E80190, &args![pass]);
        }
    }
}

/// A reference-counted object's reference dropped when it is not null (the members' own
/// `NiPointer` destructors; the pointer is left as it is).
pub(crate) fn reference_release(e: &mut Engine, object: u32) {
    if object != 0 {
        release_reference(e, object);
    }
}

// Translated from 00ba5740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's destructor,
/// `thiscall`, no stack words. Stores the vtable `010b80a8`, clears the shared `NiPointer` at
/// `012003f4`, frees the block at +0x70 (the memory manager's free `00aa4060`) and clears the
/// word, clears the pass pointers at +0x74 and +0x78 and the `NiPointer`s at +0x7c and +0x80;
/// then the members' destructors run on what is left (+0x80, +0x7c, then the passes +0x78,
/// +0x74) and `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00ba5740 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the members) is not translated.
pub fn fn_00ba5740(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PARTICLE_VTABLE_010B80A8);
    ni_pointer_clear(e, SHARED_POINTER_012003F4);
    let block = e.mem.u32(base + 0x70);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    e.mem.set_u32(base + 0x70, 0);
    pass_pointer_clear(e, base + 0x74);
    pass_pointer_clear(e, base + 0x78);
    ni_pointer_clear(e, base + 0x7c);
    ni_pointer_clear(e, base + 0x80);
    for offset in [0x80u32, 0x7c] {
        let object = e.mem.u32(base + offset);
        reference_release(e, object);
    }
    for offset in [0x78u32, 0x74] {
        let pass = e.mem.u32(base + offset);
        pass_release(e, pass);
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The render states (state, value) `00ba59e0` sets on each of its two passes, in order.
const PARTICLE_RENDER_STATES: [(u32, u32); 9] = [
    (0xa8, 0xf),
    (0x1b, 1),
    (0x13, 5),
    (0x14, 6),
    (0xf, 0),
    (0x34, 0),
    (7, 1),
    (0x17, 4),
    (0xe, 0),
];

/// One pass set up the way the particle and bolt shaders' pass set-ups expand it: a texture
/// stage when the pass has none (`[pass + 0x18]` below 1: `CreateNewStage` into the stage
/// pointer, `SetupDefaultStage(stage, 0, 1, 2, 1)`, `NiD3DPass::SetStage` with the pass's word
/// at +0x14), the shader's vertex and pixel shaders (the words at `vertex` and `pixel`)
/// assigned to the pass's `NiPointer`s at +0x5c and +0x44, then the render states in order.
pub(crate) fn set_up_pass(
    e: &mut Engine,
    pass: u32,
    stage_local: u32,
    vertex: u32,
    pixel: u32,
    states: &[(u32, u32)],
) {
    if e.mem.u32(pass + 0x18) < 1 {
        e.call(CREATE_NEW_STAGE, &args![stage_local]);
        let stage = e.mem.u32(stage_local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 1u32, 2u32, 1u32]);
        let stage = e.mem.u32(stage_local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
    }
    let vertex = e.mem.u32(vertex);
    ni_pointer_assign(e, pass + 0x5c, vertex);
    let pixel = e.mem.u32(pixel);
    ni_pointer_assign(e, pass + 0x44, pixel);
    for (state, value) in states {
        pass_render_state(e, pass, *state, *value);
    }
}

/// One pass of `00ba59e0`: [`set_up_pass`] with the vertex shader at +0x7c and the pixel
/// shader at +0x80.
fn particle_pass(e: &mut Engine, this: u32, pass: u32, stage_local: u32) {
    set_up_pass(
        e,
        pass,
        stage_local,
        this + 0x7c,
        this + 0x80,
        &PARTICLE_RENDER_STATES,
    );
}

/// A pass held for the time of a set-up (a local `NiPointer` to a pass): the count at +0x64 goes
/// up by one when the pass is not null; answers the pass.
pub(crate) fn pass_hold(e: &mut Engine, pass: u32) -> u32 {
    if pass != 0 {
        let count = e.mem.u32(pass + 0x64).wrapping_add(1);
        e.mem.set_u32(pass + 0x64, count);
    }
    pass
}

/// The held pass dropped at the end of a set-up: its count at +0x64 goes down by one (no null
/// test) and it is deleted through `00e80190` at 0.
pub(crate) fn pass_drop(e: &mut Engine, pass: u32) {
    let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
    e.mem.set_u32(pass + 0x64, left);
    if left == 0 {
        e.call(PASS_DELETE_00E80190, &args![pass]);
    }
}

// Translated from 00ba59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's pass set-up,
/// `thiscall`, no stack words. Holds the pass at +0x74 (count at +0x64 up by one when not null)
/// and sets it up (`particle_pass`); then, when the pass at +0x78 is another one, drops the held
/// pass (deleted through `00e80190` at 0), holds +0x78 and sets it up the same way. Drops the
/// stage pointer's reference (one stage pointer for both passes) and the held pass's (no null
/// test on the pass; the code reads the pass's fields without one either).
// Translation incomplete: 00ba59e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage and the pass) is not translated.
pub fn fn_00ba59e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        let first = e.mem.u32(base + 0x74);
        let mut pass = pass_hold(e, first);
        particle_pass(e, base, pass, local);
        if pass != e.mem.u32(base + 0x78) {
            pass_drop(e, pass);
            let second = e.mem.u32(base + 0x78);
            pass = pass_hold(e, second);
        }
        particle_pass(e, base, pass, local);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
        pass_drop(e, pass);
    });
}

// Translated from 00ba6370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's scalar deleting
/// destructor, `thiscall`, one stack word (the flags). Runs the destructor `00ba5740`; with bit
/// 0 of the flags set, frees the 0x12c bytes (`00aa1460`). Answers `this`.
pub fn fn_00ba6370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba5740(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x12cu32]);
    }
    this
}

// Translated from 00ba63a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): `thiscall`, no stack words. Gives
/// the pass pointers at +0x74 and +0x78 a new pass each (`00e804d0`, with the pointer's
/// address) when they are null, sets the passes up (`00ba59e0`) and answers true.
pub fn fn_00ba63a0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for offset in [0x74u32, 0x78] {
        if e.mem.u32(base + offset) == 0 {
            e.call(NEW_PASS_00E804D0, &args![base + offset]);
        }
    }
    fn_00ba59e0(e, this);
    true
}

/// `NiObject::NiObject` (`00a5d3a0`, `thiscall`, no stack words).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiAdditionalGeometryData`'s vtable (`0102e274`, RTTI `.?AVNiAdditionalGeometryData@@`) and
/// its data block array's (`0102e474`, RTTI
/// `.?AV?$NiTPrimitiveArray@PAVNiAGDDataBlock@NiAdditionalGeometryData@@@@`).
const ADDITIONAL_GEOMETRY_VTABLE_0102E274: u32 = 0x0102_e274;
const DATA_BLOCK_ARRAY_VTABLE_0102E474: u32 = 0x0102_e474;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element size,
/// count, constructor) and the stream entries' constructor (`00639940`).
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const STREAM_CONSTRUCT_00639940: u32 = 0x0063_9940;

layout! {
    /// `NiAdditionalGeometryData` (Xbox PDB), 0x2c bytes; the PC offsets match.
    pub struct NiAdditionalGeometryData: 0x2c {
        /// `m_uiRevID` (Xbox PDB).
        0x08 m_uiRevID: u32,
        /// `m_usVertexCount` (Xbox PDB).
        0x0c m_usVertexCount: u16,
        /// `m_uiDataStreamCount` (Xbox PDB).
        0x10 m_uiDataStreamCount: u32,
        /// `m_pkDataStreamEntries` (Xbox PDB): 0x1c bytes each.
        0x14 m_pkDataStreamEntries: u32,
        /// `m_uiD3DStream` (Xbox PDB).
        0x18 m_uiD3DStream: u32,
        /// `m_aDataBlocks` (Xbox PDB): a `NiTPrimitiveArray` (vtable +0x1c, base +0x20, max
        /// size +0x24, size +0x26, element count +0x28, grow-by +0x2a).
        0x1c m_aDataBlocks: u32,
    }
}

// Translated from 00ba63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::NiAdditionalGeometryData` (engine map, `_ov2`), `thiscall` with two
/// stack words: the vertex count (its low 16 bits) and the data stream count. Runs
/// `NiObject::NiObject` (`00a5d3a0`), stores the vtable `0102e274`, makes the empty data block
/// array (max size, size and element count 0, grow-by 1, no base, vtable `0102e474`), stores
/// the vertex count, clears `m_uiRevID`, stores the stream count and allocates the stream
/// entries (`operator new[]` of count elements of 0x1c bytes from the memory manager; the
/// non-null block's entries built by the `vector constructor iterator` with `00639940`). Then
/// clears the data block array's elements up to its size (none: it was just made empty), its
/// element count and size, and `m_uiD3DStream`. Answers `this`.
// Translation incomplete: 00ba63e0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the array) is not translated.
pub fn ni_additional_geometry_data_ni_additional_geometry_data_ov2(
    e: &mut Engine,
    this: Ptr<NiAdditionalGeometryData>,
    vertex_count: u16,
    stream_count: u32,
) -> Ptr<NiAdditionalGeometryData> {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, ADDITIONAL_GEOMETRY_VTABLE_0102E274);
    e.mem.set_u16(base + 0x24, 0);
    e.mem.set_u16(base + 0x2a, 1);
    e.mem.set_u16(base + 0x26, 0);
    e.mem.set_u16(base + 0x28, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x1c, DATA_BLOCK_ARRAY_VTABLE_0102E474);
    e.set(
        this,
        NiAdditionalGeometryData::m_usVertexCount,
        vertex_count,
    );
    e.set(this, NiAdditionalGeometryData::m_uiRevID, 0u32);
    e.set(
        this,
        NiAdditionalGeometryData::m_uiDataStreamCount,
        stream_count,
    );
    let block = allocate_array(e, stream_count, 0x1c);
    let entries = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT_00401050,
            &args![block, 0x1cu32, stream_count, STREAM_CONSTRUCT_00639940],
        );
        block
    } else {
        0
    };
    e.set(
        this,
        NiAdditionalGeometryData::m_pkDataStreamEntries,
        entries,
    );
    let mut index = 0u16;
    while index < e.mem.u16(base + 0x26) {
        let elements = e.mem.u32(base + 0x20);
        e.mem.set_u32(elements + 4 * u32::from(index), 0);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(base + 0x28, 0);
    e.mem.set_u16(base + 0x26, 0);
    e.set(this, NiAdditionalGeometryData::m_uiD3DStream, 0u32);
    this
}

/// Strings of `ParticleShader::CreateConstantMaps` (read in Ghidra): `"WorldViewProjTranspose"`
/// (`010a1e50`), `"Wind"` (`010b8234`), `"Color1"` (`010aff68`), `"Color2"` (`010aff60`),
/// `"Color3"` (`010b822c`), `"Velocity"` (`010b8220`), `"Acceleration"` (`010b8210`),
/// `"Scale Adjustment"` (`010b81fc`), `"fVars"` (`010af454`) and `"particle data"`
/// (`010b81ec`).
const NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const NAME_WIND_010B8234: u32 = 0x010b_8234;
const NAME_COLOR1_010AFF68: u32 = 0x010a_ff68;
const NAME_COLOR2_010AFF60: u32 = 0x010a_ff60;
const NAME_COLOR3_010B822C: u32 = 0x010b_822c;
const NAME_VELOCITY_010B8220: u32 = 0x010b_8220;
const NAME_ACCELERATION_010B8210: u32 = 0x010b_8210;
const NAME_SCALE_ADJUSTMENT_010B81FC: u32 = 0x010b_81fc;
const NAME_FVARS_010AF454: u32 = 0x010a_f454;
const NAME_PARTICLE_DATA_010B81EC: u32 = 0x010b_81ec;
/// The scale adjustment constant's data (`011adccc`).
const SCALE_ADJUSTMENT_011ADCCC: u32 = 0x011a_dccc;
/// The particle count the shader's constants are sized for (`012003ec`, set on first use from
/// the shader model setting `011f91c0`).
const PARTICLE_COUNT_012003EC: u32 = 0x0120_03ec;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00ba69f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShader::CreateConstantMaps` (engine map), `thiscall`, no stack words. When the
/// `NiPointer` at +0x34 is null: a constant map (program type 0, the word at +0x18 as the
/// renderer) into it, the predefined entry `"WorldViewProjTranspose"` (0x20000009) and the
/// entries `"Wind"`, `"Color1"` to `"Color3"`, `"Velocity"`, `"Acceleration"`, `"Scale
/// Adjustment"` (0x10000007, registers 0xf, 9 to 0xe; the data at +0xec, +0xfc, +0x10c,
/// +0x11c, +0xd4, +0xe0 and `011adccc`), `"fVars"` (0x10000009, register 4, 5 registers, 0x50
/// bytes at +0x84) and `"particle data"` (0x10000009, register 0x12, 3 registers per particle,
/// 0x30 bytes each, the block at +0x70). The particle count (`012003ec`) becomes 0x1a (shader
/// model setting below 2, signed) or 0x4e when it is 0. When the `NiPointer` at +0x30 is null:
/// a constant map of program type 2 into it.
// Translation incomplete: 00ba69f0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn particle_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vertex_map = base + 0x34;
    if e.mem.u32(vertex_map) == 0 {
        make_constant_map(e, vertex_map, base + 0x18, 0);
        add_predefined_entry(
            e,
            vertex_map,
            &[
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                EMPTY_STRING_01011584,
                0,
                0,
                0,
                0,
            ],
        );
        for (name, register, size, data) in [
            (NAME_WIND_010B8234, 0xfu32, 0x10u32, base + 0xec),
            (NAME_COLOR1_010AFF68, 9, 0x10, base + 0xfc),
            (NAME_COLOR2_010AFF60, 0xa, 0x10, base + 0x10c),
            (NAME_COLOR3_010B822C, 0xb, 0x10, base + 0x11c),
            (NAME_VELOCITY_010B8220, 0xc, 0xc, base + 0xd4),
            (NAME_ACCELERATION_010B8210, 0xd, 0xc, base + 0xe0),
            (
                NAME_SCALE_ADJUSTMENT_010B81FC,
                0xe,
                0x10,
                SCALE_ADJUSTMENT_011ADCCC,
            ),
        ] {
            add_entry(
                e,
                vertex_map,
                &[
                    name,
                    0x1000_0007,
                    0,
                    register,
                    1,
                    EMPTY_STRING_01011584,
                    size,
                    4,
                    data,
                    0,
                ],
            );
        }
        add_entry(
            e,
            vertex_map,
            &[
                NAME_FVARS_010AF454,
                0x1000_0009,
                0,
                4,
                5,
                EMPTY_STRING_01011584,
                0x50,
                4,
                base + 0x84,
                0,
            ],
        );
        if e.global::<u32>(PARTICLE_COUNT_012003EC) == 0 {
            let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
                0x1a
            } else {
                0x4e
            };
            e.set_global::<u32>(PARTICLE_COUNT_012003EC, count);
        }
        let count = e.global::<u32>(PARTICLE_COUNT_012003EC);
        let data = e.mem.u32(base + 0x70);
        add_entry(
            e,
            vertex_map,
            &[
                NAME_PARTICLE_DATA_010B81EC,
                0x1000_0009,
                0,
                0x12,
                count.wrapping_mul(3),
                EMPTY_STRING_01011584,
                count.wrapping_mul(3).wrapping_shl(4),
                4,
                data,
                0,
            ],
        );
    }
    let pixel_map = base + 0x30;
    if e.mem.u32(pixel_map) == 0 {
        make_constant_map(e, pixel_map, base + 0x18, 2);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_5740, fn_00ba5740(Ptr)),
        entry!(0x00ba_59e0, fn_00ba59e0(Ptr)),
        entry!(0x00ba_6370, fn_00ba6370(Ptr, u32) -> Ptr),
        entry!(0x00ba_63a0, fn_00ba63a0(Ptr) -> bool),
        entry!(
            0x00ba_63e0,
            ni_additional_geometry_data_ni_additional_geometry_data_ov2(
                Ptr<NiAdditionalGeometryData>,
                u16,
                u32,
            ) -> Ptr<
                NiAdditionalGeometryData,
            >
        ),
        entry!(0x00ba_69f0, particle_shader_create_constant_maps(Ptr)),
    ]
}

/// Test doubles the collection's constant map tests share.
#[cfg(test)]
pub(crate) mod constant_map_rig {
    use super::constant_maps::*;
    use crate::prelude::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// The doubles behind a test map's vtable slots +0x8 and +0x68.
    pub(crate) const MAP_ADD_ENTRY: u32 = 0x0e72_0008;
    pub(crate) const MAP_ADD_PREDEFINED: u32 = 0x0e72_0068;
    /// The double behind slot +0x70 (the entry lookup by name): answers the name plus 1.
    pub(crate) const MAP_LOOKUP: u32 = 0x0e72_0070;
    const MAP_VTABLE: u32 = 0x0e73_0000;

    /// [`rig`] with doubles for the allocation (a new block), the map constructor (the test map
    /// vtable, count 0, the renderer and program type noted at +0x10 and +0x14) and the map's
    /// slots +0x8, +0x68 and +0x70.
    pub(crate) fn map_rig() -> Engine {
        let mut e = rig();
        let mut slots = vec![0u32; 0x1d];
        slots[1] = REFERENCE_DELETE;
        slots[2] = MAP_ADD_ENTRY;
        slots[0x1a] = MAP_ADD_PREDEFINED;
        slots[0x1c] = MAP_LOOKUP;
        e.register(MAP_LOOKUP, |_, a| (a[1] + 1).into_ret());
        e.put_vtable(MAP_VTABLE, &slots);
        e.register(MAP_ADD_ENTRY, |_, _| Ret::default());
        e.register(MAP_ADD_PREDEFINED, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 0x10, a[1]);
            e.mem.set_u32(a[0] + 0x14, a[2]);
            a[0].into_ret()
        });
        e
    }

    /// The (renderer, program type) a map made by [`map_rig`] was constructed with.
    pub(crate) fn map_made_with(e: &Engine, map: u32) -> (u32, u32) {
        (e.mem.u32(map + 0x10), e.mem.u32(map + 0x14))
    }

    /// The words (without the map) of every entry added to `map` through slot +0x8.
    pub(crate) fn entries(e: &Engine, map: u32) -> Vec<Vec<u32>> {
        calls(e, MAP_ADD_ENTRY)
            .into_iter()
            .filter(|a| a[0] == map)
            .map(|a| a[1..].to_vec())
            .collect()
    }

    /// The words (without the map) of every entry added to `map` through slot +0x68.
    pub(crate) fn predefined(e: &Engine, map: u32) -> Vec<Vec<u32>> {
        calls(e, MAP_ADD_PREDEFINED)
            .into_iter()
            .filter(|a| a[0] == map)
            .map(|a| a[1..].to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::constant_map_rig::*;
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn destructor_engine() -> Engine {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        e.map(0x0120_0000, 0x1000);
        e
    }

    #[test]
    fn fn_00ba5740_releases_the_members_then_the_base() {
        let mut e = destructor_engine();
        let shared = object(&mut e, 1);
        e.set_global::<u32>(SHARED_POINTER_012003F4, shared);
        let this = e.mem.alloc(0x12c);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        let kept = e.mem.alloc(0x70);
        e.mem.set_u32(kept + 0x64, 2);
        let vertex = object(&mut e, 2);
        e.mem.set_u32(this + 0x70, 0x0e55_0000);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, kept);
        e.mem.set_u32(this + 0x7c, vertex);
        e.call(0x00ba_5740, &args![this]);
        assert_eq!(e.mem.u32(this), PARTICLE_VTABLE_010B80A8);
        assert_eq!(e.global::<u32>(SHARED_POINTER_012003F4), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shared]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0e55_0000]]
        );
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u32(kept + 0x64), 1);
        assert_eq!(e.mem.u32(vertex + 4), 1);
        for offset in [0x70u32, 0x74, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00ba6370_frees_only_with_bit_0() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x12c);
        assert_eq!(e.call(0x00ba_6370, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
        e.call(0x00ba_6370, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x12c]]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT).len(), 2);
    }

    /// A shader with a vertex and a pixel shader at +0x7c and +0x80.
    fn pass_shader(e: &mut Engine) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x12c);
        let vertex = object(e, 1);
        let pixel = object(e, 1);
        e.mem.set_u32(this + 0x7c, vertex);
        e.mem.set_u32(this + 0x80, pixel);
        (this, vertex, pixel)
    }

    #[test]
    fn fn_00ba63a0_makes_two_passes_and_sets_them_up() {
        let mut e = pass_rig();
        let (this, vertex, pixel) = pass_shader(&mut e);
        assert!(e.call(0x00ba_63a0, &args![this]).bool());
        let first = e.mem.u32(this + 0x74);
        let second = e.mem.u32(this + 0x78);
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x74], vec![this + 0x78]]
        );
        for pass in [first, second] {
            assert_eq!(stage_setups(&e, pass), vec![(0, [1, 2, 1])]);
            assert_eq!(e.mem.u32(pass + 0x5c), vertex);
            assert_eq!(e.mem.u32(pass + 0x44), pixel);
            assert_eq!(render_states(&e, pass), PARTICLE_RENDER_STATES.to_vec());
            // Held and dropped again: the count is back to the new pass's 1.
            assert_eq!(e.mem.u32(pass + 0x64), 1);
        }
        assert_eq!(e.mem.u32(vertex + 4), 3);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), Vec::<Vec<u32>>::new());
    }

    /// The (index, three words) `SetupDefaultStage` got for each stage `SetStage` put on the
    /// pass (the particle passes set no filter mode).
    fn stage_setups(e: &Engine, pass: u32) -> Vec<(u32, [u32; 3])> {
        let setups = calls(e, SETUP_DEFAULT_STAGE);
        calls(e, PASS_SET_STAGE)
            .iter()
            .filter(|a| a[0] == pass)
            .map(|a| {
                let setup = setups.iter().find(|s| s[0] == a[2]).unwrap();
                (setup[1], [setup[2], setup[3], setup[4]])
            })
            .collect()
    }

    #[test]
    fn fn_00ba59e0_skips_stages_on_passes_that_have_them_and_one_shared_pass() {
        let mut e = pass_rig();
        let (this, _, _) = pass_shader(&mut e);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(pass + 0x18, 1);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, pass);
        e.call(0x00ba_59e0, &args![this]);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert_eq!(render_states(&e, pass).len(), 18);
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        // A pass with the last reference: deleted when the second pass replaces it.
        let (this, _, _) = pass_shader(&mut e);
        let other = e.mem.alloc(0x70);
        e.mem.set_u32(other + 0x64, 1);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, other);
        e.mem.set_u32(pass + 0x64, 0);
        e.call(0x00ba_59e0, &args![this]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 1);
    }

    #[test]
    fn additional_geometry_data_constructor_allocates_the_stream_entries() {
        let mut e = rig();
        e.register(NI_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(VECTOR_CONSTRUCT_00401050, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        let answer = e
            .call(0x00ba_63e0, &args![this, 0xabcd_0012u32, 3u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), ADDITIONAL_GEOMETRY_VTABLE_0102E274);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0x12);
        assert_eq!(e.mem.u16(this + 0xe), 0xcccc);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        let entries = e.mem.u32(this + 0x14);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x54]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![entries, 0x1c, 3, STREAM_CONSTRUCT_00639940]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), DATA_BLOCK_ARRAY_VTABLE_0102E474);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.bytes(this + 0x24, 6), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u16(this + 0x2a), 1);
        // An overflowing count asks for all ones; a null block keeps no entries.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00ba_63e0, &args![this, 1u32, 0x1000_0000u32]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE)[1],
            vec![MEMORY_MANAGER, u32::MAX]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(calls(&e, VECTOR_CONSTRUCT_00401050).len(), 1);
    }

    #[test]
    fn particle_create_constant_maps_makes_both_maps_once() {
        let mut e = map_rig();
        for page in [0x0120_0000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 1);
        let this = e.mem.alloc(0x12c);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.mem.set_u32(this + 0x70, 0x0e56_0000);
        e.call(0x00ba_69f0, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(map_made_with(&e, vertex), (0x0e50_0000, 0));
        assert_eq!(map_made_with(&e, pixel), (0x0e50_0000, 2));
        assert_eq!(e.mem.u32(vertex + 4), 1);
        assert_eq!(
            predefined(&e, vertex),
            vec![vec![
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                EMPTY_STRING_01011584,
                0,
                0,
                0,
                0
            ]]
        );
        let added = entries(&e, vertex);
        assert_eq!(added.len(), 9);
        assert_eq!(
            added[0],
            vec![
                NAME_WIND_010B8234,
                0x1000_0007,
                0,
                0xf,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                this + 0xec,
                0
            ]
        );
        assert_eq!(added[6][8], SCALE_ADJUSTMENT_011ADCCC);
        assert_eq!(
            added[7][3..9],
            [4, 5, EMPTY_STRING_01011584, 0x50, 4, this + 0x84]
        );
        assert_eq!(e.global::<u32>(PARTICLE_COUNT_012003EC), 0x1a);
        assert_eq!(
            added[8],
            vec![
                NAME_PARTICLE_DATA_010B81EC,
                0x1000_0009,
                0,
                0x12,
                0x4e,
                EMPTY_STRING_01011584,
                0x4e0,
                4,
                0x0e56_0000,
                0
            ]
        );
        // Both maps there: nothing more.
        e.call(0x00ba_69f0, &args![this]);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 2);
        // A set count is kept; a model from 2 gives 0x4e.
        let mut e = map_rig();
        for page in [0x0120_0000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 2);
        let this = e.mem.alloc(0x12c);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x30, held);
        e.call(0x00ba_69f0, &args![this]);
        assert_eq!(e.global::<u32>(PARTICLE_COUNT_012003EC), 0x4e);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 1);
    }
}
