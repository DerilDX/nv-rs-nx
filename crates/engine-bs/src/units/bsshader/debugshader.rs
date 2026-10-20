//! `BSShader/debugshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `DebugShader::DebugShader` (`00bd1950`), `::LoadShaders` (`00bd16f0`) and
/// `::LoadStagesAndPasses` (`00bd20e0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bd_1950;
const LOAD_SHADERS: u32 = 0x00bd_16f0;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bd_20e0;

// Translated from 00bd23f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 9. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 2 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)) and calls slot +0xb4. Then allocates
/// 0xc0 bytes and runs the constructor (`00bd1950`; a null allocation is used as it is),
/// calls the shader's slot +0x4c, `LoadShaders` (`00bd16f0`), `LoadStagesAndPasses`
/// (`00bd20e0`), then slot +0xc0 with the declaration (no slot +0x11c here), and answers the
/// shader.
// Translation incomplete: 00bd23f0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn debug_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The debug shader's vtable (`010bbd08`; no RTTI locator before it).
const DEBUG_SHADER_VTABLE_010BBD08: u32 = 0x010b_bd08;
/// The element constructor and destructor of the 0x10-pointer array at +0x6c (`006694e0`,
/// `0045cec0`).
const POINTER_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00bd1950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::DebugShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010bbd08`, constructs the 0x10
/// pointers at +0x6c (constructor `006694e0`, destructor `0045cec0`), clears the words +0xac
/// and +0xb0, constructs the `NiPointer` array at +0xb4 (2, destructor `00b66f20`), sets +0x68
/// to 10, then clears (releasing what they hold) the `NiPointer`s at +0xb0, +0xb4, +0xb8,
/// +0xac and +0x6c to +0xa8, and sets the byte +0x24 to 1. Answers `this`.
// Translation incomplete: 00bd1950 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the base and the arrays) is not translated.
pub fn debug_shader_debug_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, DEBUG_SHADER_VTABLE_010BBD08);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x6c,
            4u32,
            0x10u32,
            POINTER_CONSTRUCT_006694E0,
            POINTER_DESTRUCT_0045CEC0
        ],
    );
    e.mem.set_u32(base + 0xac, 0);
    e.mem.set_u32(base + 0xb0, 0);
    construct_pointer_array(e, base + 0xb4, 2, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 10);
    ni_pointer_clear(e, base + 0xb0);
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0xb4 + 4 * index);
    }
    ni_pointer_clear(e, base + 0xac);
    for index in 0..0x10u32 {
        ni_pointer_clear(e, base + 0x6c + 4 * index);
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"imagespace\\1x\\v\\base_old.v.hlsl"`
/// (`010bbce8`), `"imagespace\\1x\\p\\copy.p.hlsl"` (`010b0820`), the macro `"CHANNELMASK"`
/// (`010bbcc4`) with the value `""` (`01011584`), the compiled names `"DEBUG.vso"`,
/// `"DEBUG00.pso"`, `"DEBUG01.pso"` and the vertex profile `"vs_1_1"` (`010a93ec`).
const VERTEX_SOURCE_010BBCE8: u32 = 0x010b_bce8;
const PIXEL_SOURCE_010B0820: u32 = 0x010b_0820;
const MACRO_CHANNELMASK_010BBCC4: u32 = 0x010b_bcc4;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const OUTPUT_VSO_010BBCDC: u32 = 0x010b_bcdc;
const OUTPUT_PSO_010BBCD0: u32 = 0x010b_bcd0;
const OUTPUT_PSO_010BBCB8: u32 = 0x010b_bcb8;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// The pixel profile chooser `00b4f380` (unit `bsshadermanager.cpp`, unnamed; `cdecl`, a byte:
/// answers the profile string of a shader model setting).
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;

/// The source blocks of `LoadShaders`' frame (0x300 bytes up to the return address).
const VERTEX_BLOCK: ShaderSource = ShaderSource {
    offset: 0xa8,
    words: &[VERTEX_SOURCE_010BBCE8, 0, 0],
    cleared: 0x40,
};
const PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x5c,
    words: &[PIXEL_SOURCE_010B0820, 0, 0],
    cleared: 0x40,
};
const MASKED_PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x10,
    words: &[
        PIXEL_SOURCE_010B0820,
        MACRO_CHANNELMASK_010BBCC4,
        EMPTY_STRING_01011584,
        0,
    ],
    cleared: 0x3c,
};
const LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xf4,
    path: 0x1f8,
};

// Translated from 00bd16f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Takes the pixel profile
/// from `00b4f380(1)`, then fills each source block on its stack just before its compile: the
/// vertex file `base_old.v.hlsl` (no macros) as `"DEBUG.vso"` (`vs_1_1`) into the `NiPointer`
/// at +0xb0, the pixel file `copy.p.hlsl` (no macros) as `"DEBUG00.pso"` into +0xb4, and the
/// same pixel file with `CHANNELMASK` = `""` as `"DEBUG01.pso"` into +0xb8 (each step
/// `MakePath`, the name copied by `00406d00`, the compile, the assignment).
// Translation incomplete: 00bd16f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn debug_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let pixel_profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
    let steps = [
        (
            VERTEX_BLOCK,
            ShaderCompile {
                source: 0xa8,
                output: OUTPUT_VSO_010BBCDC,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                slot: 0xb0,
            },
        ),
        (
            PIXEL_BLOCK,
            ShaderCompile {
                source: 0x5c,
                output: OUTPUT_PSO_010BBCD0,
                profile: pixel_profile,
                pixel: true,
                slot: 0xb4,
            },
        ),
        (
            MASKED_PIXEL_BLOCK,
            ShaderCompile {
                source: 0x10,
                output: OUTPUT_PSO_010BBCB8,
                profile: pixel_profile,
                pixel: true,
                slot: 0xb8,
            },
        ),
    ];
    e.with_stack(0x300, |e, frame| {
        for (block, step) in steps {
            set_up_sources(e, frame.addr(), &[block]);
            compile_shaders(e, this.addr(), frame.addr(), &LOAD_SHADERS_BUFFERS, &[step]);
        }
    });
}

// Translated from 00bd20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers 1. Gives
/// the pass pointer at +0x5c a new pass (`00e804d0`) with one stage (`CreateNewStage` into a
/// stack pointer, `SetupDefaultStage` index 0 with 3, 2, 1, `SetFilterMode(0)`, `SetStage`),
/// the vertex shader +0xb0 (pass +0x5c) and pixel shader +0xb4 (pass +0x44), the render states
/// 7 = 0, 0xe = 0, 0x1b = 0, 0x13 = 2, 0x14 = 2 (through the pass read for the one before) and
/// 0xa8 = 0xf; then drops the stage reference.
// Translation incomplete: 00bd20e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn debug_shader_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let slot = base + 0x5c;
    e.call(NEW_PASS_00E804D0, &args![slot]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let stage = e.mem.u32(local);
        e.call(SET_FILTER_MODE, &args![stage, 0u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + 0xb0);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + 0xb4);
        ni_pointer_assign(e, pass + 0x44, pixel);
        for (state, value) in [(7u32, 0u32), (0xe, 0), (0x1b, 0)] {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0x13, 2);
        pass_render_state(e, pass, 0x14, 2);
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0xa8, 0xf);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_1950, debug_shader_debug_shader(Ptr) -> Ptr),
        entry!(0x00bd_16f0, debug_shader_load_shaders(Ptr)),
        entry!(0x00bd_20e0, debug_shader_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00bd_23f0, debug_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn debug_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bd_23f0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xc0]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (LOAD_STAGES_AND_PASSES, vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod b000h_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn debug_shader_constructor_builds_and_clears_the_pointers() {
        let mut e = rig();
        let this = e.mem.alloc(0xbc);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), DEBUG_SHADER_VTABLE_010BBD08);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                vec![
                    this + 0x6c,
                    4,
                    0x10,
                    POINTER_CONSTRUCT_006694E0,
                    POINTER_DESTRUCT_0045CEC0
                ],
                pointer_array(this + 0xb4, 2, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 10);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        // Everything just cleared: nothing to release.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn debug_shader_load_shaders_compiles_three_shaders_with_the_chosen_profile() {
        const PROFILE: u32 = 0x0ed0_0000;
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a[0], 1);
            PROFILE.into_ret()
        });
        let this = e.mem.alloc(0xbc);
        e.call(LOAD_SHADERS, &args![this]);
        let mask = macro_words(&[(MACRO_CHANNELMASK_010BBCC4, EMPTY_STRING_01011584)]);
        let expected = [
            (
                0xb0u32,
                VERTEX_SOURCE_010BBCE8,
                OUTPUT_VSO_010BBCDC,
                PROFILE_VS_1_1_010A93EC,
                false,
                [0; 8],
            ),
            (
                0xb4,
                PIXEL_SOURCE_010B0820,
                OUTPUT_PSO_010BBCD0,
                PROFILE,
                true,
                [0; 8],
            ),
            (
                0xb8,
                PIXEL_SOURCE_010B0820,
                OUTPUT_PSO_010BBCB8,
                PROFILE,
                true,
                mask,
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
        assert_eq!(cleared, vec![0x40, 0x40, 0x3c]);
    }

    #[test]
    fn debug_shader_load_stages_and_passes_builds_one_pass() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xbc);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0xb0, vertex);
        e.mem.set_u32(this + 0xb4, pixel);
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x5c]]);
        let pass = e.mem.u32(this + 0x5c);
        assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 0)]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(
            render_states(&e, pass),
            vec![
                (7, 0),
                (0xe, 0),
                (0x1b, 0),
                (0x13, 2),
                (0x14, 2),
                (0xa8, 0xf)
            ]
        );
        let stage = calls(&e, PASS_SET_STAGE)[0][2];
        assert_eq!(e.mem.u32(stage + 0x60), 1);
    }
}
