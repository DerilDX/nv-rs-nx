//! `BSShader/bsdistanttreeshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `BSDistantTreeShader::BSDistantTreeShader` (`00bb6c30`, `thiscall`, `RET 4`: the
/// declaration), `::LoadShaders` (`00bb6060`) and `::LoadStagesAndPasses` (`00bb64a0`; both
/// `thiscall`, no words) (engine map).
const CONSTRUCT: u32 = 0x00bb_6c30;
const LOAD_SHADERS: u32 = 0x00bb_6060;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bb_64a0;

// Translated from 00bb6e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 8. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 4 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 1, 5, 1, 0), (2, 2, 3, 2, 0)), calls
/// slot +0x94 with (0, 3, 3, 0x80000000, 0, 5, 1, 0) and slot +0xb4. Then allocates 0xfc
/// bytes and runs the constructor (`00bb6c30`) with the declaration (a null allocation is
/// used as it is), calls the shader's slot +0x4c, `LoadShaders` (`00bb6060`),
/// `LoadStagesAndPasses` (`00bb64a0`), slot +0x11c, then +0xc0 with the declaration, and
/// answers the shader.
// Translation incomplete: 00bb6e30 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_distant_tree_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 1, 5, 1, 0], [2, 2, 3, 2, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 3u32, 3u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xfcu32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block, declaration]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The distant tree shader's vtable (`010b9aa8`; no RTTI locator before it).
const DISTANT_TREE_VTABLE_010B9AA8: u32 = 0x010b_9aa8;
/// The instance count shared by the distant tree shaders (`012007b8`), set on first use from
/// the shader model setting (`011f91c0`): 0x28 below model 2, else 0x72.
const INSTANCE_COUNT_012007B8: u32 = 0x0120_07b8;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bb6c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShader::BSDistantTreeShader` (Xbox PDB), `thiscall` with one stack word (the
/// declaration). Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010b9aa8`,
/// constructs the `NiPointer` arrays at +0x6c (2, destructor `00b79930`), +0x74 (1) and +0x78
/// (2; destructor `00b66f20`), clears the word +0x80, stores 0.0 at +0x84 to +0xa0 and +0xac
/// to +0xf8, sets +0x68 to 0x19, assigns the declaration to the `NiPointer` at +0x80; when the
/// word at `012007b8` is 0 it becomes 0x28 (shader model setting `011f91c0` below 2, signed)
/// or 0x72; then allocates that many 0x20-byte elements from the memory manager into +0xa8.
/// Answers `this`.
// Translation incomplete: 00bb6c30 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the base and the arrays) is not translated.
pub fn bs_distant_tree_shader_bs_distant_tree_shader(
    e: &mut Engine,
    this: Ptr,
    declaration: Ptr,
) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, DISTANT_TREE_VTABLE_010B9AA8);
    construct_pointer_array(e, base + 0x6c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x74, 1, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x78, 2, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x80, 0);
    for offset in (0x84u32..=0xa0).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    for offset in (0xacu32..=0xf8).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0x19);
    ni_pointer_assign(e, base + 0x80, declaration.addr());
    if e.global::<u32>(INSTANCE_COUNT_012007B8) == 0 {
        let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x28
        } else {
            0x72
        };
        e.set_global::<u32>(INSTANCE_COUNT_012007B8, count);
    }
    let count = e.global::<u32>(INSTANCE_COUNT_012007B8);
    let elements = allocate_array(e, count, 0x20);
    e.mem.set_u32(base + 0xa8, elements);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"distanttree\\DistantTree.v.hlsl"`
/// (`010b9a7c`), `"distanttree\\DistantTree.p.hlsl"` (`010b9a5c`), the macros `"DTTEST"`
/// (`010b9a9c`) and `"SILHOUETTE"` (`010aeea4`) with the value `""` (`01011584`), the
/// compiled names `"DISTTREE.vso"`, `"DISTTREE.pso"`, `"DISTTREESil.pso"` and the profiles
/// `"vs_1_1"` (`010a93ec`) and `"ps_2_a"` (`010a93c4`).
const VERTEX_SOURCE_010B9A7C: u32 = 0x010b_9a7c;
const PIXEL_SOURCE_010B9A5C: u32 = 0x010b_9a5c;
const MACRO_DTTEST_010B9A9C: u32 = 0x010b_9a9c;
const MACRO_SILHOUETTE_010AEEA4: u32 = 0x010a_eea4;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const OUTPUT_VSO_010B9A4C: u32 = 0x010b_9a4c;
const OUTPUT_PSO_010B9A3C: u32 = 0x010b_9a3c;
const OUTPUT_SILHOUETTE_PSO_010B9A2C: u32 = 0x010b_9a2c;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;

/// The source blocks of `LoadShaders`' frame (0x300 bytes up to the return address), in the
/// order the exe fills them.
const LOAD_SHADERS_SOURCES: [ShaderSource; 3] = [
    ShaderSource {
        offset: 0xa8,
        words: &[
            VERTEX_SOURCE_010B9A7C,
            MACRO_DTTEST_010B9A9C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x5c,
        words: &[
            PIXEL_SOURCE_010B9A5C,
            MACRO_DTTEST_010B9A9C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x10,
        words: &[
            PIXEL_SOURCE_010B9A5C,
            MACRO_DTTEST_010B9A9C,
            EMPTY_STRING_01011584,
            MACRO_SILHOUETTE_010AEEA4,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
];
const LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xf4,
    path: 0x1f8,
};
const LOAD_SHADERS_STEPS: [ShaderCompile; 3] = [
    ShaderCompile {
        source: 0xa8,
        output: OUTPUT_VSO_010B9A4C,
        profile: PROFILE_VS_1_1_010A93EC,
        pixel: false,
        slot: 0x74,
    },
    ShaderCompile {
        source: 0x5c,
        output: OUTPUT_PSO_010B9A3C,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x78,
    },
    ShaderCompile {
        source: 0x10,
        output: OUTPUT_SILHOUETTE_PSO_010B9A2C,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x7c,
    },
];

// Translated from 00bb6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills three
/// source blocks on its stack (the vertex and pixel HLSL files with the macro `DTTEST`, and
/// the pixel file with `DTTEST` and `SILHOUETTE`; all values `""`, the rest of each block
/// cleared by `memset`), then compiles the vertex shader as `"DISTTREE.vso"` (`vs_1_1`) into
/// the `NiPointer` at +0x74, the pixel shader as `"DISTTREE.pso"` (`ps_2_a`) into +0x78 and
/// the silhouette pixel shader as `"DISTTREESil.pso"` into +0x7c (each step `MakePath`, the
/// name copied by `00406d00`, the compile, the assignment).
// Translation incomplete: 00bb6060 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_distant_tree_shader_load_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0x300, |e, frame| {
        set_up_sources(e, frame.addr(), &LOAD_SHADERS_SOURCES);
        compile_shaders(
            e,
            this.addr(),
            frame.addr(),
            &LOAD_SHADERS_BUFFERS,
            &LOAD_SHADERS_STEPS,
        );
    });
}

// Translated from 00bb64a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShader::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers
/// 1. Gives the pass pointer at +0x6c a new pass (`00e804d0`) with one stage
/// (`CreateNewStage` into a stack pointer, `SetupDefaultStage` index 0 with 3, 2, 1,
/// `SetFilterMode(1)`, `SetStage`) and the vertex shader +0x74 (pass +0x5c) and pixel shader
/// +0x78 (pass +0x44); then the pass at +0x70 a new pass, after which the first stage is
/// dropped and the pointer cleared, a second stage made the same way for it, and the shaders
/// +0x74 and +0x7c; finally the second stage is dropped.
// Translation incomplete: 00bb64a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn bs_distant_tree_shader_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let add_stage = |e: &mut Engine, slot: u32| {
            e.call(CREATE_NEW_STAGE, &args![local]);
            let stage = e.mem.u32(local);
            e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
            let stage = e.mem.u32(local);
            e.call(SET_FILTER_MODE, &args![stage, 1u32]);
            let pass = e.mem.u32(slot);
            let stage = e.mem.u32(local);
            let position = e.mem.u32(pass + 0x14);
            e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        };
        let shaders = |e: &mut Engine, slot: u32, vertex: u32, pixel: u32| {
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
        };
        let first = base + 0x6c;
        e.call(NEW_PASS_00E804D0, &args![first]);
        add_stage(e, first);
        shaders(e, first, 0x74, 0x78);
        let second = base + 0x70;
        e.call(NEW_PASS_00E804D0, &args![second]);
        let stage = e.mem.u32(local);
        if stage != 0 {
            stage_release(e, stage);
            e.mem.set_u32(local, 0);
        }
        add_stage(e, second);
        shaders(e, second, 0x74, 0x7c);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, one word), the
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`), `BSShader::~BSShader` (engine
/// map, `00be0f10`, no stack words) and the Ni free (`00aa1460`, `cdecl`: block, size).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bb6300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsdistanttreeshader.cpp`): the distant tree shader's
/// destructor, no stack words. Stores the vtable `010b9aa8`, drops and clears the passes +0x6c
/// and +0x70, clears (releasing what they hold) the `NiPointer`s +0x74, +0x78 and +0x7c, gives
/// the instance array (+0xa8) back to the memory manager (`00aa4060`), clears +0x80, releases
/// +0x80 again when not null (it was just cleared), destroys the arrays +0x78 (2) and +0x74 (1)
/// (destructor `00b66f20`) and +0x6c (2, `00b79930`) and runs `BSShader::~BSShader`
/// (`00be0f10`).
// Translation incomplete: 00bb6300 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the members and the base) is not translated.
pub fn fn_00bb6300(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, DISTANT_TREE_VTABLE_010B9AA8);
    for index in 0..2u32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    ni_pointer_clear(e, base + 0x74);
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x78 + 4 * index);
    }
    let instances = e.mem.u32(base + 0xa8);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, instances]);
    ni_pointer_clear(e, base + 0x80);
    let held = e.mem.u32(base + 0x80);
    if held != 0 {
        release_reference(e, held);
    }
    for (offset, count, destructor) in [
        (0x78u32, 2u32, DESTRUCT_00B66F20),
        (0x74, 1, DESTRUCT_00B66F20),
        (0x6c, 2, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00bb6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsdistanttreeshader.cpp`): the scalar deleting destructor
/// (vtable `010b9aa8` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00bb6300`) and, when
/// bit 0 of the flags is set, frees the 0xfc bytes (`00aa1460`). Answers `this`.
pub fn fn_00bb6e00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb6300(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xfcu32]);
    }
    this
}

/// The constant names of `CreateConstantMaps` (read in Ghidra).
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const FOG_0103BFC8: u32 = 0x0103_bfc8;
const INSTANCES_010B9C18: u32 = 0x010b_9c18;
const DIFFUSE_LIGHT_DIRECTION_010B9C00: u32 = 0x010b_9c00;
const DIFFUSE_LIGHT_COLOR_010B9BEC: u32 = 0x010b_9bec;
const AMBIENT_COLOR_010B8FE4: u32 = 0x010b_8fe4;

// Translated from 00bb6f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShader::CreateConstantMaps` (Xbox PDB), no stack words. When the pixel
/// constant map (+0x30) is null, makes one (kind 2) without entries; when the vertex one
/// (+0x34) is null, makes one (kind 0) with `"WorldViewProjTranspose"` (0x20000009, 0, 0,
/// 4), `"fog"` (0x10000009, register 4, 2 registers, 0x20 bytes at +0x84), `"instances"`
/// (register 9, twice the instance count `012007b8` registers, 0x20 bytes each, the array
/// +0xa8), `"Diffuse Light direction"` (+0xbc, 6), `"Diffuse Light color"` (+0xcc, 7) and
/// `"ambient color"` (+0xac, 8).
// Translation incomplete: 00bb6f40 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn bs_distant_tree_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let empty = CONSTANT_EMPTY_NAME_01011584;
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
    }
    if e.mem.u32(base + 0x34) != 0 {
        return;
    }
    make_constant_map(e, base, 0x34, 0);
    add_constant_entry(
        e,
        base,
        0x34,
        [
            WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
            0x2000_0009,
            0,
            0,
            4,
            0,
            0,
            0,
            0,
            0,
        ],
    );
    add_constant_entry(
        e,
        base,
        0x34,
        [
            FOG_0103BFC8,
            0x1000_0009,
            0,
            4,
            2,
            empty,
            0x20,
            4,
            base + 0x84,
            0,
        ],
    );
    let instances = e.mem.u32(base + 0xa8);
    let count = e.global::<u32>(INSTANCE_COUNT_012007B8);
    add_constant_entry(
        e,
        base,
        0x34,
        [
            INSTANCES_010B9C18,
            0x1000_0009,
            0,
            9,
            count.wrapping_add(count),
            empty,
            count << 5,
            4,
            instances,
            0,
        ],
    );
    for (name, register, offset) in [
        (DIFFUSE_LIGHT_DIRECTION_010B9C00, 6u32, 0xbcu32),
        (DIFFUSE_LIGHT_COLOR_010B9BEC, 7, 0xcc),
        (AMBIENT_COLOR_010B8FE4, 8, 0xac),
    ] {
        add_constant_entry(
            e,
            base,
            0x34,
            [
                name,
                0x1000_0007,
                0,
                register,
                1,
                empty,
                0x10,
                4,
                base + offset,
                0,
            ],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_6300, fn_00bb6300(Ptr)),
        entry!(0x00bb_6e00, fn_00bb6e00(Ptr, u32) -> Ptr),
        entry!(
            0x00bb_6f40,
            bs_distant_tree_shader_create_constant_maps(Ptr)
        ),
        entry!(0x00bb_6c30, bs_distant_tree_shader_bs_distant_tree_shader(Ptr, Ptr) -> Ptr),
        entry!(0x00bb_6060, bs_distant_tree_shader_load_shaders(Ptr)),
        entry!(0x00bb_64a0, bs_distant_tree_shader_load_stages_and_passes(Ptr) -> bool),
        entry!(
            0x00bb_6e30,
            bs_distant_tree_shader_create_shader() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_distant_tree_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bb_6e30);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 1, 5, 1, 0]),
                (declaration_slot(0x8c), vec![d, 2, 2, 3, 2, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 3, 3, 0x8000_0000, 0, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xfc]),
                (CONSTRUCT, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (LOAD_STAGES_AND_PASSES, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod b000h_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global::<u32>(INSTANCE_COUNT_012007B8, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(CONSTRUCT, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn bs_distant_tree_shader_constructor_builds_the_shader() {
        let (e, this, declaration) = construct(0, 3);
        assert_eq!(e.mem.u32(this), DISTANT_TREE_VTABLE_010B9AA8);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x74, 1, DESTRUCT_00B66F20),
                pointer_array(this + 0x78, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x84u32..=0xa0)
            .step_by(4)
            .chain((0xacu32..=0xf8).step_by(4))
        {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xa4), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0x68), 0x19);
        assert_eq!(e.mem.u32(this + 0x80), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(e.global::<u32>(INSTANCE_COUNT_012007B8), 0x72);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40]]
        );
        assert_ne!(e.mem.u32(this + 0xa8), 0);
    }

    #[test]
    fn bs_distant_tree_shader_constructor_count_follows_the_model_once() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(INSTANCE_COUNT_012007B8), 0x28);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x500]]
        );
        let (e, _, _) = construct(7, 1);
        assert_eq!(e.global::<u32>(INSTANCE_COUNT_012007B8), 7);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0xe0]]);
    }

    #[test]
    fn bs_distant_tree_shader_load_shaders_compiles_three_shaders() {
        let mut e = loader_rig();
        let this = e.mem.alloc(0x100);
        e.call(LOAD_SHADERS, &args![this]);
        let test = (MACRO_DTTEST_010B9A9C, EMPTY_STRING_01011584);
        let silhouette = (MACRO_SILHOUETTE_010AEEA4, EMPTY_STRING_01011584);
        let expected = [
            (
                0x74u32,
                VERTEX_SOURCE_010B9A7C,
                OUTPUT_VSO_010B9A4C,
                PROFILE_VS_1_1_010A93EC,
                false,
                macro_words(&[test]),
            ),
            (
                0x78,
                PIXEL_SOURCE_010B9A5C,
                OUTPUT_PSO_010B9A3C,
                PROFILE_PS_2_A_010A93C4,
                true,
                macro_words(&[test]),
            ),
            (
                0x7c,
                PIXEL_SOURCE_010B9A5C,
                OUTPUT_SILHOUETTE_PSO_010B9A2C,
                PROFILE_PS_2_A_010A93C4,
                true,
                macro_words(&[test, silhouette]),
            ),
        ];
        for (slot, file, output, profile, pixel, macros) in expected {
            let shader = e.mem.u32(this + slot);
            assert_eq!(
                compiled(&e, shader),
                Compiled {
                    file,
                    output,
                    profile,
                    pixel,
                    macros
                }
            );
        }
        let cleared: Vec<u32> = calls(&e, MEMSET).iter().map(|a| a[2]).collect();
        assert_eq!(cleared, vec![0x3c, 0x3c, 0x34]);
    }

    #[test]
    fn bs_distant_tree_shader_load_stages_and_passes_builds_two_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x100);
        let shaders: Vec<u32> = (0..3).map(|_| object(&mut e, 1)).collect();
        for (index, shader) in shaders.iter().enumerate() {
            e.mem.set_u32(this + 0x74 + 4 * index as u32, *shader);
        }
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x6c], vec![this + 0x70]]
        );
        let first = e.mem.u32(this + 0x6c);
        let second = e.mem.u32(this + 0x70);
        assert_eq!(stages(&e, first), vec![(0, [3, 2, 1], 1)]);
        assert_eq!(stages(&e, second), vec![(0, [3, 2, 1], 1)]);
        assert_eq!(e.mem.u32(first + 0x5c), shaders[0]);
        assert_eq!(e.mem.u32(first + 0x44), shaders[1]);
        assert_eq!(e.mem.u32(second + 0x5c), shaders[0]);
        assert_eq!(e.mem.u32(second + 0x44), shaders[2]);
        assert_eq!(e.mem.u32(shaders[0] + 4), 3);
        for a in calls(&e, PASS_SET_STAGE) {
            assert_eq!(e.mem.u32(a[2] + 0x60), 1);
        }
    }
}

#[cfg(test)]
mod distant_tree_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00bb6300_drops_the_members_and_destroys() {
        let mut e = rig();
        noop(
            &mut e,
            &[MEMORY_DEALLOCATE, VECTOR_DESTRUCT, BS_SHADER_DESTRUCT],
        );
        let this = e.mem.alloc(0xfc);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x6c, pass);
        let texture = object(&mut e, 1);
        e.mem.set_u32(this + 0x74, texture);
        let held = object(&mut e, 2);
        e.mem.set_u32(this + 0x80, held);
        e.mem.set_u32(this + 0xa8, 0x9000);
        e.call(0x00bb_6300, &args![this]);
        assert_eq!(e.mem.u32(this), DISTANT_TREE_VTABLE_010B9AA8);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![texture]]);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9000]]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x78, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x74, 4, 1, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 2, DESTRUCT_00B79930],
            ]
        );
    }

    #[test]
    fn fn_00bb6e00_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                MEMORY_DEALLOCATE,
                VECTOR_DESTRUCT,
                BS_SHADER_DESTRUCT,
                NI_FREE_00AA1460,
            ],
        );
        let this = e.mem.alloc(0xfc);
        assert_eq!(e.call(0x00bb_6e00, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xfc]]);
        e.call(0x00bb_6e00, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn create_constant_maps_adds_the_tree_constants() {
        const MAP_SLOTS: u32 = 0x0e30_0000;
        const MAP_VTABLE: u32 = 0x0e31_0000;
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.set_global(INSTANCE_COUNT_012007B8, 0x20u32);
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        noop(&mut e, &[MAP_SLOTS + 8]);
        let this = e.mem.alloc(0xfc);
        e.mem.set_u32(this + 0xa8, 0x9000);
        e.call(0x00bb_6f40, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 6);
        assert_eq!(
            entries[2],
            vec![
                vertex,
                INSTANCES_010B9C18,
                0x1000_0009,
                0,
                9,
                0x40,
                CONSTANT_EMPTY_NAME_01011584,
                0x400,
                4,
                0x9000,
                0
            ]
        );
        assert_eq!(entries[1][..2], [vertex, FOG_0103BFC8]);
        assert_eq!(entries[5][1], AMBIENT_COLOR_010B8FE4);
        assert_eq!(entries[5][9], this + 0xac);
        let maps = calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330);
        assert_eq!((maps[0][2], maps[1][2]), (2, 0));
        // The vertex map present: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x00bb_6f40, &args![this]);
        assert!(calls(&e, MAP_SLOTS + 8).is_empty());
    }
}
