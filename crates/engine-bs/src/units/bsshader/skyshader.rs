//! `BSShader/skyshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `SkyShader::SkyShader` (`00b8a1f0`), `::LoadShaders` (`00b874f0`) and
/// `::LoadStagesAndPasses` (`00b887c0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_a1f0;
const LOAD_SHADERS: u32 = 0x00b8_74f0;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_87c0;

// Translated from 00b8a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0xa. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 3 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 5, 4, 3, 0), (2, 7, 5, 1, 0)) and calls slot
/// +0xb4. Then allocates 0xf0 bytes and runs the constructor (`00b8a1f0`; a null allocation
/// is used as it is), calls the shader's slot +0x4c, `LoadShaders` (`00b874f0`),
/// `LoadStagesAndPasses` (`00b887c0`), slot +0x11c, then +0xc0 with the declaration, and
/// answers the shader.
// Translation incomplete: 00b8a390 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn sky_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 5, 4, 3, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xf0u32]).u32();
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

/// The sky shader's vtable (`010afe18`; no RTTI locator before it).
const SKY_SHADER_VTABLE_010AFE18: u32 = 0x010a_fe18;
/// The four words copied (`011a9bd0`) and the seven four-word blocks they go to, in the
/// exe's order of their first words.
const COLOR_011A9BD0: u32 = 0x011a_9bd0;
const COLOR_BLOCKS: [u32; 7] = [
    0x011f_f910,
    0x011f_f920,
    0x011f_f930,
    0x011f_f8e0,
    0x011f_f8f0,
    0x011f_f900,
    0x011f_f8c8,
];

// Translated from 00b8a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::SkyShader` (Xbox PDB), `thiscall`, no stack words. Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010afe18`, constructs the `NiPointer` arrays at +0x6c (0xb,
/// destructor `00b79930`), +0x98 (0xb) and +0xc4 (8; destructor `00b66f20`), sets +0x68 to
/// 0xb, copies the four words at `011a9bd0` into each of the blocks at `011ff910`,
/// `011ff920`, `011ff930`, `011ff8e0`, `011ff8f0`, `011ff900` and `011ff8c8`, and sets the
/// byte +0x24 to 1. Answers `this`.
// Translation incomplete: 00b8a1f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn sky_shader_sky_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SKY_SHADER_VTABLE_010AFE18);
    construct_pointer_array(e, base + 0x6c, 0xb, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x98, 0xb, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xc4, 8, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 0xb);
    let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(COLOR_011A9BD0 + offset));
    for block in COLOR_BLOCKS {
        e.set_global::<u32>(block, color[0]);
    }
    for block in COLOR_BLOCKS {
        for word in 1..4u32 {
            e.set_global::<u32>(block + 4 * word, color[word as usize]);
        }
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"sky\\v\\sky.v.hlsl"` (`010afe04`),
/// `"sky\\v\\sky_quad.v.hlsl"` (`010afcbc`), `"sky\\p\\sky.p.hlsl"` (`010afd50`); the macros
/// `"TEX"`, `"DEPTH_VALUE"` (with `"0.99995f"`, `"1.0f"` or `"0.999999"`), `"HORIZFADE"`,
/// `"OCCLUSION"`, `"CLOUDS"`, `"SI"`, `"SI_CLOUDS"`, `"SI_SUN"`; the compiled names from
/// `"SKY.vso"` to `"SKYSICLOUDS.pso"` (see the steps).
const SKY_VERTEX_SOURCE_010AFE04: u32 = 0x010a_fe04;
const SKY_QUAD_VERTEX_SOURCE_010AFCBC: u32 = 0x010a_fcbc;
const SKY_PIXEL_SOURCE_010AFD50: u32 = 0x010a_fd50;
const SKY_MACRO_TEX_010AECAC: u32 = 0x010a_ecac;
const MACRO_DEPTH_VALUE_010AFDE4: u32 = 0x010a_fde4;
const DEPTH_0_99995F_010AFDD8: u32 = 0x010a_fdd8;
const DEPTH_1_0F_010AFDC4: u32 = 0x010a_fdc4;
const DEPTH_0_999999_010AFCFC: u32 = 0x010a_fcfc;
const MACRO_HORIZFADE_010AFDB8: u32 = 0x010a_fdb8;
const MACRO_OCCLUSION_010AFD9C: u32 = 0x010a_fd9c;
const MACRO_CLOUDS_010AFD88: u32 = 0x010a_fd88;
const MACRO_SI_010AEC48: u32 = 0x010a_ec48;
const MACRO_SI_CLOUDS_010AFCE4: u32 = 0x010a_fce4;
const MACRO_SI_SUN_010AFC98: u32 = 0x010a_fc98;
const SKY_EMPTY_STRING_01011584: u32 = 0x0101_1584;
/// The vertex profile chooser `00b4f3e0` (no arguments) and the pixel one `00b4f380` (`cdecl`,
/// a byte; given 0) (unit `bsshadermanager.cpp`, unnamed; they answer a profile string).
const VERTEX_PROFILE_00B4F3E0: u32 = 0x00b4_f3e0;
const SKY_PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// CRT `_gcvt` (`00ecdd16`, `cdecl`: the `double`, the digit count, the buffer) and the two
/// `float`s it is given (`011ff8ac`, `011ff8b0`, each through `FLD`).
const GCVT: u32 = 0x00ec_dd16;
const SUN_VALUE_011FF8AC: u32 = 0x011f_f8ac;
const SI_VALUE_011FF8B0: u32 = 0x011f_f8b0;
/// The `LoadShaders` frame (0x8d4 bytes up to the return address), its name and path buffers
/// and the `_gcvt` buffer.
const SKY_FRAME_BYTES: u32 = 0x8d4;
const SKY_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x568,
    path: 0x66c,
};
const SKY_NUMBER_BUFFER: u32 = 0x770;

/// One step of the sky `LoadShaders`: the `float` formatted by `_gcvt` first (0 for none), the
/// source block filled first (with the word that receives the number buffer's address, if
/// any), the file word's frame offset (or the file itself when `file_value` is set), the
/// compiled name, vertex or pixel, the macros' frame offset and the shader field.
struct SkyStep {
    gcvt: u32,
    block: ShaderSource,
    number_word: Option<u32>,
    file_offset: u32,
    file_value: bool,
    output: u32,
    pixel: bool,
    macros: u32,
    slot: u32,
}

/// The sky steps in the exe's order.
const SKY_STEPS: [SkyStep; 18] = [
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x4d0,
            words: &[SKY_VERTEX_SOURCE_010AFE04, 0, 0],
            cleared: 0x40,
        },
        number_word: None,
        file_offset: 0x4d0,
        file_value: false,
        output: 0x010a_fdfc,
        pixel: false,
        macros: 0x4d4,
        slot: 0x98,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x308,
            words: &[SKY_VERTEX_SOURCE_010AFE04, SKY_MACRO_TEX_010AECAC, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x308,
        file_value: false,
        output: 0x010a_fdf0,
        pixel: false,
        macros: 0x30c,
        slot: 0x9c,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x224,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_99995F_010AFDD8,
                SKY_MACRO_TEX_010AECAC,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x224,
        file_value: false,
        output: 0x010a_fdcc,
        pixel: false,
        macros: 0x228,
        slot: 0xa0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0xf4,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_1_0F_010AFDC4,
                MACRO_HORIZFADE_010AFDB8,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0xf4,
        file_value: false,
        output: 0x010a_fda8,
        pixel: false,
        macros: 0xf8,
        slot: 0xa8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x484,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_OCCLUSION_010AFD9C,
                0,
                0,
                0,
            ],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x484,
        file_value: false,
        output: 0x010a_fd90,
        pixel: false,
        macros: 0x488,
        slot: 0xac,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0xa8,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                SKY_MACRO_TEX_010AECAC,
                0,
                MACRO_CLOUDS_010AFD88,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0xa8,
        file_value: false,
        output: 0x010a_fd78,
        pixel: false,
        macros: 0xac,
        slot: 0xb0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x5c,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_HORIZFADE_010AFDB8,
                0,
                SKY_MACRO_TEX_010AECAC,
                0,
                MACRO_CLOUDS_010AFD88,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            cleared: 0,
        },
        number_word: None,
        file_offset: 0xa8,
        file_value: false,
        output: 0x010a_fd64,
        pixel: false,
        macros: 0x60,
        slot: 0xb4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x51c,
            words: &[SKY_PIXEL_SOURCE_010AFD50, 0, 0],
            cleared: 0x40,
        },
        number_word: None,
        file_offset: 0x51c,
        file_value: false,
        output: 0x010a_fd48,
        pixel: true,
        macros: 0x520,
        slot: 0xc4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x270,
            words: &[SKY_PIXEL_SOURCE_010AFD50, SKY_MACRO_TEX_010AECAC, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x270,
        file_value: false,
        output: 0x010a_fd3c,
        pixel: true,
        macros: 0x274,
        slot: 0xc8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x2bc,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_HORIZFADE_010AFDB8,
                0,
                SKY_MACRO_TEX_010AECAC,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x270,
        file_value: false,
        output: 0x010a_fd2c,
        pixel: true,
        macros: 0x2c0,
        slot: 0xcc,
    },
    SkyStep {
        gcvt: SUN_VALUE_011FF8AC,
        block: ShaderSource {
            offset: 0x438,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_OCCLUSION_010AFD9C, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x438,
        file_value: false,
        output: 0x010a_fd1c,
        pixel: true,
        macros: 0x43c,
        slot: 0xd0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x3a0,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_HORIZFADE_010AFDB8, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x3a0,
        file_value: false,
        output: 0x010a_fd08,
        pixel: true,
        macros: 0x3a4,
        slot: 0xd4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x1d8,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_999999_010AFCFC,
                MACRO_SI_010AEC48,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x1d8,
        file_value: false,
        output: 0x010a_fcf0,
        pixel: false,
        macros: 0x1dc,
        slot: 0xb8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x10,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_999999_010AFCFC,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_CLOUDS_010AFCE4,
                SKY_EMPTY_STRING_01011584,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            cleared: 0,
        },
        number_word: None,
        file_offset: SKY_VERTEX_SOURCE_010AFE04,
        file_value: true,
        output: 0x010a_fcd4,
        pixel: false,
        macros: 0x14,
        slot: 0xbc,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x3ec,
            words: &[SKY_QUAD_VERTEX_SOURCE_010AFCBC, MACRO_SI_010AEC48, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x3ec,
        file_value: false,
        output: 0x010a_fcac,
        pixel: false,
        macros: 0x3f0,
        slot: 0xc0,
    },
    SkyStep {
        gcvt: SI_VALUE_011FF8B0,
        block: ShaderSource {
            offset: 0x354,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_SI_010AEC48, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: Some(2),
        file_offset: 0x354,
        file_value: false,
        output: 0x010a_fca0,
        pixel: true,
        macros: 0x358,
        slot: 0xe0,
    },
    SkyStep {
        gcvt: SUN_VALUE_011FF8AC,
        block: ShaderSource {
            offset: 0x140,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_SUN_010AFC98,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: Some(2),
        file_offset: 0x140,
        file_value: false,
        output: 0x010a_fc88,
        pixel: true,
        macros: 0x144,
        slot: 0xd8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x18c,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_CLOUDS_010AFCE4,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x18c,
        file_value: false,
        output: 0x010a_fc78,
        pixel: true,
        macros: 0x190,
        slot: 0xdc,
    },
];

// Translated from 00b874f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Eighteen steps in the exe's
/// order (see `SKY_STEPS`), each filling one source block on its stack (the sky, sky-quad and
/// sky pixel HLSL files with their macros; two blocks written word by word without
/// `memset`) and compiling it: `MakePath` of the step's file word (two steps take another
/// block's word, one the file string itself), the compiled name copied by `00406d00`, the
/// profile from `00b4f3e0()` for vertex shaders or `00b4f380(0)` for pixel shaders, the
/// compile with the step's macros, the result assigned to the shader's `NiPointer` (+0x98 to
/// +0xe0). Three steps first format a `float` (`011ff8ac` or `011ff8b0`, through the FPU) with
/// `_gcvt(value, 12, buffer)`; two of them give the buffer's address as the `SI` macro's value
/// (the first one's text is not used).
// Translation incomplete: 00b874f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn sky_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(SKY_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        let numbers = frame + SKY_NUMBER_BUFFER;
        let path = frame + SKY_BUFFERS.path;
        let name = frame + SKY_BUFFERS.name;
        for step in &SKY_STEPS {
            if step.gcvt != 0 {
                let value = x87_load32(e.global::<u32>(step.gcvt));
                e.call(GCVT, &args![value, 0xcu32, numbers]);
            }
            set_up_sources(e, frame, core::slice::from_ref(&step.block));
            if let Some(index) = step.number_word {
                e.mem
                    .set_u32(frame + step.block.offset + 4 * index, numbers);
            }
            let file = if step.file_value {
                step.file_offset
            } else {
                e.mem.u32(frame + step.file_offset)
            };
            e.call(MAKE_PATH, &args![file, path]);
            e.call(FORMAT_00406D00, &args![name, 0x104u32, step.output]);
            let (compile, profile) = if step.pixel {
                let profile = e.call(SKY_PIXEL_PROFILE_00B4F380, &args![0u32]).u32();
                (COMPILE_PIXEL_SHADER, profile)
            } else {
                let profile = e.call(VERTEX_PROFILE_00B4F3E0, &args![]).u32();
                (COMPILE_VERTEX_SHADER, profile)
            };
            let shader = e
                .call(
                    compile,
                    &args![base, path, frame + step.macros, profile, name],
                )
                .u32();
            ni_pointer_assign(e, base + step.slot, shader);
        }
    });
}

/// One pass of `SkyShader::LoadStagesAndPasses` (immediates of its straight-line code): the
/// pass pointer's offset, the number of texture stages, the render states (state, value,
/// whether the exe sets it through the pass it read for the state before), and the vertex and
/// pixel shader `NiPointer`s it takes.
type SkyPass = (u32, u32, &'static [(u32, u32, bool)], u32, u32);

/// The sky passes in the exe's order.
const SKY_PASSES: [SkyPass; 11] = [
    (
        0x6c,
        1,
        &[
            (0x1b, 1, false),
            (0x13, 2, false),
            (0x14, 2, true),
            (0xe, 0, false),
            (0xa8, 0, false),
        ],
        0xac,
        0xd0,
    ),
    (
        0x8c,
        1,
        &[
            (0xf, 0, false),
            (0x1b, 0, false),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xc0,
        0xe0,
    ),
    (
        0x90,
        1,
        &[
            (0x1b, 1, false),
            (0x13, 2, false),
            (0x14, 2, true),
            (0xf, 0, false),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xb8,
        0xd8,
    ),
    (
        0x94,
        2,
        &[
            (0xf, 0, false),
            (0x1b, 1, false),
            (0x13, 1, false),
            (0x14, 6, true),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xbc,
        0xdc,
    ),
    (
        0x70,
        2,
        &[
            (7, 0, false),
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 2, true),
            (0x34, 0, false),
            (0xf, 0, false),
            (0xa8, 7, false),
        ],
        0x9c,
        0xc8,
    ),
    (
        0x78,
        1,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 2, true),
            (0xa8, 7, false),
        ],
        0xa8,
        0xd4,
    ),
    (
        0x7c,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0xb0,
        0xc8,
    ),
    (
        0x80,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0xb4,
        0xcc,
    ),
    (
        0x84,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0x9c,
        0xc8,
    ),
    (
        0x74,
        2,
        &[
            (0xf, 1, false),
            (0x18, 0, false),
            (0x19, 5, false),
            (0x1b, 1, false),
            (0x13, 1, false),
            (0x14, 2, true),
        ],
        0xa0,
        0xc8,
    ),
    (
        0x88,
        0,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0x98,
        0xc4,
    ),
];

// Translated from 00b887c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers 1. For each
/// of the eleven pass pointers in the exe's order (see `SKY_PASSES`): when the pointer is null,
/// a new pass (`00e804d0`), its texture stages (one or two, made into one stack pointer by
/// `CreateNewStage` without dropping the one before, `SetupDefaultStage` with the stage's
/// index and 3, 2, 1, no filter mode, `SetStage` with the pass's word at +0x14) and its render
/// states (the pass read again for each but 0x14, which goes through the pass read for 0x13);
/// then, whether or not the pass was made, its vertex (+0x5c) and pixel (+0x44) shader
/// `NiPointer`s are assigned from the shader's. Finally the stage reference is dropped.
// Translation incomplete: 00b887c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn sky_shader_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (pass_offset, stages, states, vertex, pixel) in SKY_PASSES {
            let slot = base + pass_offset;
            if e.mem.u32(slot) == 0 {
                e.call(NEW_PASS_00E804D0, &args![slot]);
                for index in 0..stages {
                    e.call(CREATE_NEW_STAGE, &args![local]);
                    let stage = e.mem.u32(local);
                    e.call(SETUP_DEFAULT_STAGE, &args![stage, index, 3u32, 2u32, 1u32]);
                    let pass = e.mem.u32(slot);
                    let stage = e.mem.u32(local);
                    let position = e.mem.u32(pass + 0x14);
                    e.call(PASS_SET_STAGE, &args![pass, position, stage]);
                }
                let mut pass = e.mem.u32(slot);
                for &(state, value, reuse) in states {
                    if !reuse {
                        pass = e.mem.u32(slot);
                    }
                    pass_render_state(e, pass, state, value);
                }
            }
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_87c0, sky_shader_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00b8_74f0, sky_shader_load_shaders(Ptr)),
        entry!(0x00b8_a1f0, sky_shader_sky_shader(Ptr) -> Ptr),
        entry!(0x00b8_a390, sky_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn sky_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_a390);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xf0]),
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
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn sky_shader_constructor_builds_the_shader_and_copies_the_color() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        for word in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BD0 + 4 * word, 0x30 + word);
        }
        let this = e.mem.alloc(0xe4);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SKY_SHADER_VTABLE_010AFE18);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0xb, DESTRUCT_00B79930),
                pointer_array(this + 0x98, 0xb, DESTRUCT_00B66F20),
                pointer_array(this + 0xc4, 8, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0xb);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        for block in [
            0x011f_f8c8u32,
            0x011f_f8e0,
            0x011f_f8f0,
            0x011f_f900,
            0x011f_f910,
            0x011f_f920,
            0x011f_f930,
        ] {
            for word in 0..4u32 {
                assert_eq!(e.global::<u32>(block + 4 * word), 0x30 + word);
            }
        }
        assert_eq!(e.global::<u32>(0x011f_f8d8), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const VERTEX_PROFILE: u32 = 0x0f40_0000;
    const PIXEL_PROFILE: u32 = 0x0f40_0010;

    #[test]
    fn sky_shader_load_shaders_compiles_eighteen_shaders() {
        let mut e = loader_rig();
        e.register(VERTEX_PROFILE_00B4F3E0, |_, _| VERTEX_PROFILE.into_ret());
        e.register(SKY_PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a[0], 0);
            PIXEL_PROFILE.into_ret()
        });
        e.register(GCVT, |e, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            assert_eq!(a[2], 0xc);
            e.mem.set_u32(a[3], value as u32);
            Ret::default()
        });
        e.map(0x011f_f000, 0x1000);
        e.set_global::<f32>(SUN_VALUE_011FF8AC, 3.0);
        e.set_global::<f32>(SI_VALUE_011FF8B0, 5.0);
        let this = e.mem.alloc(0xe4);
        e.call(LOAD_SHADERS, &args![this]);
        let mut slots: Vec<u32> = SKY_STEPS.iter().map(|s| s.slot).collect();
        slots.sort_unstable();
        assert_eq!(
            slots,
            (0x98u32..=0xe0)
                .step_by(4)
                .filter(|s| *s != 0xa4)
                .collect::<Vec<_>>()
        );
        for step in &SKY_STEPS {
            let seen = compiled(&e, e.mem.u32(this + step.slot));
            assert_eq!(seen.output, step.output);
            assert_eq!(seen.pixel, step.pixel);
            let profile = if step.pixel {
                PIXEL_PROFILE
            } else {
                VERTEX_PROFILE
            };
            assert_eq!(seen.profile, profile);
        }
        // The cloud-fade vertex shader takes the file of the clouds block and the macros of
        // its own full block.
        let fade = compiled(&e, e.mem.u32(this + 0xb4));
        assert_eq!(fade.file, SKY_VERTEX_SOURCE_010AFE04);
        assert_eq!(
            fade.macros,
            macro_words(&[
                (MACRO_HORIZFADE_010AFDB8, 0),
                (SKY_MACRO_TEX_010AECAC, 0),
                (MACRO_CLOUDS_010AFD88, 0)
            ])
        );
        let quad = compiled(&e, e.mem.u32(this + 0xc0));
        assert_eq!(quad.file, SKY_QUAD_VERTEX_SOURCE_010AFCBC);
        // The SI pixel shaders see the number buffer's address as the SI value: the compile
        // double kept the macro words, so the value is a frame address holding the number.
        let si = compiled(&e, e.mem.u32(this + 0xe0));
        assert_eq!(si.macros[0], MACRO_SI_010AEC48);
        assert_ne!(si.macros[1], 0);
        let gcvt = calls(&e, GCVT);
        assert_eq!(gcvt.len(), 3);
        let bits = 3.0f64.to_bits();
        assert_eq!(gcvt[0][..2], [bits as u32, (bits >> 32) as u32]);
        assert_eq!(gcvt[1][3], si.macros[1]);
        assert_eq!(calls(&e, MEMSET).len(), 16);
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn sky_shader_load_stages_and_passes_builds_the_missing_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xe4);
        for offset in (0x98u32..=0xe0).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        // One pass exists already: it only gets its shaders.
        let kept = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x88, kept);
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        let made: Vec<u32> = calls(&e, NEW_PASS_00E804D0).iter().map(|a| a[0]).collect();
        let order = [
            0x6cu32, 0x8c, 0x90, 0x94, 0x70, 0x78, 0x7c, 0x80, 0x84, 0x74,
        ];
        assert_eq!(made, order.iter().map(|o| this + o).collect::<Vec<_>>());
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 16);
        let setups: Vec<u32> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(setups, vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 61);
        let first = e.mem.u32(this + 0x6c);
        assert_eq!(
            render_states(&e, first),
            vec![(0x1b, 1), (0x13, 2), (0x14, 2), (0xe, 0), (0xa8, 0)]
        );
        assert_eq!(e.mem.u32(first + 0x5c), e.mem.u32(this + 0xac));
        assert_eq!(e.mem.u32(first + 0x44), e.mem.u32(this + 0xd0));
        let fade = e.mem.u32(this + 0x74);
        assert_eq!(
            render_states(&e, fade),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x13, 1),
                (0x14, 2)
            ]
        );
        assert_eq!(e.mem.u32(kept + 0x5c), e.mem.u32(this + 0x98));
        assert_eq!(e.mem.u32(kept + 0x44), e.mem.u32(this + 0xc4));
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != kept));
    }
}
