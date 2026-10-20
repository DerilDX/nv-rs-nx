//! `BSShader/bsshaderbloodsplatter.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderBloodSplatter::BSShaderBloodSplatter` (`00b80d30`), `::LoadShaders`
/// (`00b80090`) and `::LoadStagesAndPasses` (`00b807d0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_0d30;
const LOAD_SHADERS: u32 = 0x00b8_0090;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_07d0;

// Translated from 00b80e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 7. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 2 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)) and calls slot +0xb4. Then
/// allocates 0x84 bytes and runs the constructor (`00b80d30`; a null allocation is used as it
/// is), calls the shader's slot +0x4c, `LoadShaders` (`00b80090`), `LoadStagesAndPasses`
/// (`00b807d0`), slot +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00b80e70 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_blood_splatter_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x84u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
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

/// The blood splatter shader's vtable (`010af760`; no RTTI locator before it).
const BLOOD_SPLATTER_VTABLE_010AF760: u32 = 0x010a_f760;
/// The two `float`s the constructor loads (`010177e8`, `0101e2bc`) and the four it stores
/// (`011fed94`, `011fed98` the first, `011fed9c` 0.0, `011feda0` the second).
const VALUE_010177E8: u32 = 0x0101_77e8;
const VALUE_0101E2BC: u32 = 0x0101_e2bc;
const FLOATS_011FED94: u32 = 0x011f_ed94;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`: file name, load byte, the
/// `NiPointer` to fill, mipmap byte, cube byte), the texture name it is given (`010af89c`)
/// and the `NiPointer` it fills (`011fedc4`).
const GET_TEXTURE: u32 = 0x00b5_5840;
const TEXTURE_NAME_010AF89C: u32 = 0x010a_f89c;
const TEXTURE_POINTER_011FEDC4: u32 = 0x011f_edc4;

// Translated from 00b80d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::BSShaderBloodSplatter` (Xbox PDB), `thiscall`, no stack words.
/// Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010af760`, constructs the
/// `NiPointer` arrays at +0x6c (2, destructor `00b79930`), +0x74 and +0x7c (2 each,
/// destructor `00b66f20`), sets +0x68 to 0x17, stores the `float` at `010177e8` (through the
/// FPU, a signalling NaN quieted) in `011fed94` and `011fed98`, 0.0 in `011fed9c` and the
/// `float` at `0101e2bc` in `011feda0`, then has `GetTexture` (`00b55840`) fill the pointer
/// at `011fedc4` with the texture named at `010af89c` (load byte 1, mipmap byte 1, not a cube
/// map). Answers `this`.
// Translation incomplete: 00b80d30 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn bs_shader_blood_splatter_bs_shader_blood_splatter(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BLOOD_SPLATTER_VTABLE_010AF760);
    construct_pointer_array(e, base + 0x6c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x74, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 2, DESTRUCT_00B66F20);
    let first = x87_quiet_bits(e.global::<u32>(VALUE_010177E8));
    let second = x87_quiet_bits(e.global::<u32>(VALUE_0101E2BC));
    e.mem.set_u32(base + 0x68, 0x17);
    e.set_global::<u32>(FLOATS_011FED94, first);
    e.set_global::<u32>(FLOATS_011FED94 + 4, first);
    e.set_global::<f32>(FLOATS_011FED94 + 8, 0.0);
    e.set_global::<u32>(FLOATS_011FED94 + 0xc, second);
    e.call(
        GET_TEXTURE,
        &args![
            TEXTURE_NAME_010AF89C,
            1u32,
            TEXTURE_POINTER_011FEDC4,
            1u32,
            0u32
        ],
    );
    this
}
/// Strings of `LoadShaders` (read in Ghidra): `"BloodSplatter\\BloodSplatter.v.hlsl"`
/// (`010af738`), `"BloodSplatter\\BloodSplatter.p.hlsl"` (`010af714`), the macro `"FLARE"`
/// (`010af70c`) with the value `""` (`01011584`), the compiled names `"BS.vso"`, `"BSF.vso"`,
/// `"BS.pso"`, `"BSF.pso"` and the profiles `"vs_1_1"` (`010a93ec`), `"ps_2_a"` (`010a93c4`).
const VERTEX_SOURCE_010AF738: u32 = 0x010a_f738;
const PIXEL_SOURCE_010AF714: u32 = 0x010a_f714;
const MACRO_FLARE_010AF70C: u32 = 0x010a_f70c;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const OUTPUT_BS_VSO_010AF704: u32 = 0x010a_f704;
const OUTPUT_BSF_VSO_010AF6FC: u32 = 0x010a_f6fc;
const OUTPUT_BS_PSO_010AF6F4: u32 = 0x010a_f6f4;
const OUTPUT_BSF_PSO_010AF6EC: u32 = 0x010a_f6ec;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;

/// The source blocks of `LoadShaders`' frame (0x34c bytes up to the return address), in the order the exe fills them.
const LOAD_SHADERS_SOURCES: [ShaderSource; 4] = [
    ShaderSource {
        offset: 0xa8,
        words: &[VERTEX_SOURCE_010AF738, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0xf4,
        words: &[PIXEL_SOURCE_010AF714, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x10,
        words: &[
            VERTEX_SOURCE_010AF738,
            MACRO_FLARE_010AF70C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x5c,
        words: &[
            PIXEL_SOURCE_010AF714,
            MACRO_FLARE_010AF70C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
];
/// The compiled-name and path buffers of that frame.
const LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x140,
    path: 0x244,
};
/// The compile steps, in the exe's order.
const LOAD_SHADERS_STEPS: [ShaderCompile; 4] = [
    ShaderCompile {
        source: 0xa8,
        output: OUTPUT_BS_VSO_010AF704,
        profile: PROFILE_VS_1_1_010A93EC,
        pixel: false,
        slot: 0x74,
    },
    ShaderCompile {
        source: 0x10,
        output: OUTPUT_BSF_VSO_010AF6FC,
        profile: PROFILE_VS_1_1_010A93EC,
        pixel: false,
        slot: 0x78,
    },
    ShaderCompile {
        source: 0xf4,
        output: OUTPUT_BS_PSO_010AF6F4,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x7c,
    },
    ShaderCompile {
        source: 0x5c,
        output: OUTPUT_BSF_PSO_010AF6EC,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x80,
    },
];

// Translated from 00b80090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills four
/// source blocks on its stack (the vertex and pixel HLSL files, each without macros and with
/// the macro `FLARE` = `""`; the rest of each block cleared by `memset`), then compiles in
/// order: the plain vertex source as `"BS.vso"` (`vs_1_1`) into the `NiPointer` at +0x74, the
/// `FLARE` vertex source as `"BSF.vso"` into +0x78, the plain pixel source as `"BS.pso"`
/// (`ps_2_a`) into +0x7c and the `FLARE` pixel source as `"BSF.pso"` into +0x80. Each step:
/// `BSShader::MakePath` (`00be0c00`) of the file into the path buffer, the compiled name copied
/// by `00406d00`, `CompileVertexShaderHLSL` (`00be0fe0`) or `CompilePixelShaderHLSL`
/// (`00be1750`), the result assigned to the pointer.
// Translation incomplete: 00b80090 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_shader_blood_splatter_load_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0x34c, |e, frame| {
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
// Translated from 00b807d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words;
/// answers 1. Two passes, at +0x6c and +0x70: each gets a new pass (`00e804d0`), two default
/// stages (`SetupDefaultStage` with 3, 2, 1, filter mode 1), the render states 0x1b = 1,
/// 7 = 0, 0xe = 0, then 0x13 and 0x14 (1 and 3 for the first pass, 2 and 2 for the second;
/// the last one through the pass read for the one before), and its vertex shader (+0x5c) and
/// pixel shader (+0x44) `NiPointer`s assigned from the loader's shaders: +0x74 and +0x7c for
/// the first pass, +0x78 and +0x80 for the second.
// Translation incomplete: 00b807d0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage pointers) is not translated.
pub fn bs_shader_blood_splatter_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for (pass_offset, last_states, shaders) in [
        (0x6cu32, [1u32, 3], [0x74u32, 0x7c]),
        (0x70, [2, 2], [0x78, 0x80]),
    ] {
        let slot = base + pass_offset;
        e.call(NEW_PASS_00E804D0, &args![slot]);
        add_default_stages(e, slot, 2, [3, 2, 1], 1);
        for (state, value) in [(0x1bu32, 1u32), (7, 0), (0xe, 0)] {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0x13, last_states[0]);
        pass_render_state(e, pass, 0x14, last_states[1]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + shaders[0]);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + shaders[1]);
        ni_pointer_assign(e, pass + 0x44, pixel);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_0d30, bs_shader_blood_splatter_bs_shader_blood_splatter(Ptr) -> Ptr),
        entry!(0x00b8_0090, bs_shader_blood_splatter_load_shaders(Ptr)),
        entry!(0x00b8_07d0, bs_shader_blood_splatter_load_stages_and_passes(Ptr) -> bool),
        entry!(
            0x00b8_0e70,
            bs_shader_blood_splatter_create_shader() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_blood_splatter_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_0e70);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x84]),
                (CONSTRUCT, vec![SHADER]),
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

    #[test]
    fn bs_shader_blood_splatter_constructor_sets_the_globals_and_gets_the_texture() {
        let mut e = rig();
        e.register(GET_TEXTURE, |_, _| Ret::default());
        e.map(0x0101_7000, 0x8000);
        e.map(0x011f_e000, 0x1000);
        // A signalling NaN is quieted by the FPU load.
        e.set_global::<u32>(VALUE_010177E8, 0x7f80_0001);
        e.set_global::<f32>(VALUE_0101E2BC, 2.5);
        e.mem.write(FLOATS_011FED94, &[0xff; 0x10]);
        let this = e.mem.alloc(0x84);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BLOOD_SPLATTER_VTABLE_010AF760);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x74, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 2, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0x17);
        assert_eq!(e.global::<u32>(FLOATS_011FED94), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(FLOATS_011FED94 + 4), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(FLOATS_011FED94 + 8), 0);
        assert_eq!(e.global::<f32>(FLOATS_011FED94 + 0xc), 2.5);
        assert_eq!(
            calls(&e, GET_TEXTURE),
            vec![vec![
                TEXTURE_NAME_010AF89C,
                1,
                TEXTURE_POINTER_011FEDC4,
                1,
                0
            ]]
        );
    }

    #[test]
    fn bs_shader_blood_splatter_load_shaders_compiles_four_shaders() {
        let mut e = loader_rig();
        let this = e.mem.alloc(0x84);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x78, old);
        e.call(LOAD_SHADERS, &args![this]);
        let flare = macro_words(&[(MACRO_FLARE_010AF70C, EMPTY_STRING_01011584)]);
        let expected = [
            (
                0x74u32,
                VERTEX_SOURCE_010AF738,
                OUTPUT_BS_VSO_010AF704,
                false,
                [0; 8],
            ),
            (
                0x78,
                VERTEX_SOURCE_010AF738,
                OUTPUT_BSF_VSO_010AF6FC,
                false,
                flare,
            ),
            (
                0x7c,
                PIXEL_SOURCE_010AF714,
                OUTPUT_BS_PSO_010AF6F4,
                true,
                [0; 8],
            ),
            (
                0x80,
                PIXEL_SOURCE_010AF714,
                OUTPUT_BSF_PSO_010AF6EC,
                true,
                flare,
            ),
        ];
        for (slot, file, output, pixel, macros) in expected {
            let shader = e.mem.u32(this + slot);
            let profile = if pixel {
                PROFILE_PS_2_A_010A93C4
            } else {
                PROFILE_VS_1_1_010A93EC
            };
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
            assert_eq!(e.mem.u32(shader + 4), 1);
        }
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        let cleared: Vec<u32> = calls(&e, MEMSET).iter().map(|a| a[2]).collect();
        assert_eq!(cleared, vec![0x44, 0x44, 0x3c, 0x3c]);
    }

    #[test]
    fn bs_shader_blood_splatter_load_stages_and_passes_builds_two_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x84);
        let shaders: Vec<u32> = (0..4).map(|_| object(&mut e, 1)).collect();
        for (index, shader) in shaders.iter().enumerate() {
            e.mem.set_u32(this + 0x74 + 4 * index as u32, *shader);
        }
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x6c], vec![this + 0x70]]
        );
        for (pass_offset, last, vertex, pixel) in [
            (0x6cu32, (1u32, 3u32), 0usize, 2usize),
            (0x70, (2, 2), 1, 3),
        ] {
            let pass = e.mem.u32(this + pass_offset);
            assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 1), (1, [3, 2, 1], 1)]);
            assert_eq!(
                render_states(&e, pass),
                vec![(0x1b, 1), (7, 0), (0xe, 0), (0x13, last.0), (0x14, last.1)]
            );
            assert_eq!(e.mem.u32(pass + 0x5c), shaders[vertex]);
            assert_eq!(e.mem.u32(pass + 0x44), shaders[pixel]);
        }
        // Each stage is held by its pass only.
        for a in calls(&e, PASS_SET_STAGE) {
            assert_eq!(e.mem.u32(a[2] + 0x60), 1);
        }
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }
}
