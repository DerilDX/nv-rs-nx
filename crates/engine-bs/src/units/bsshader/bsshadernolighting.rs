//! `BSShader/bsshadernolighting.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderNoLighting::BSShaderNoLighting` (`00bc8f80`, `thiscall`, `RET 0x30`: twelve
/// declarations), `::LoadShaders` (`00bc5c40`) and `::LoadStagesAndPasses` (`00bc77e0`; both
/// `thiscall`, no words) (engine map).
const CONSTRUCT: u32 = 0x00bc_8f80;
const LOAD_SHADERS: u32 = 0x00bc_5c40;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bc_77e0;

/// Each declaration's entry count as made (`Create`'s second word) and the entries it gets
/// through vtable slot +0x8c (five words each), in the exe's order.
const DECLARATIONS: [(u32, &[[u32; 5]]); 12] = [
    (2, &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]]),
    (2, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]]),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 3, 2, 0],
            [2, 5, 4, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
        ],
    ),
    (
        6,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
            [5, 7, 5, 1, 0],
        ],
    ),
];

// Translated from 00bc93c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x21. No arguments. Makes twelve declarations, one after the other
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508` read each time, entry
/// counts 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 1 stream each), then in the same order sets
/// each one's entries through its vtable slot +0x8c (the lists of `DECLARATIONS`, read from
/// the pushes) and ends it (slot +0xb4). Allocates 0x1b4 bytes and runs the constructor
/// (`00bc8f80`) with the twelve declarations in order (a null allocation is used as it is),
/// calls the shader's slot +0x4c, `LoadShaders` (`00bc5c40`), `LoadStagesAndPasses`
/// (`00bc77e0`), slot +0x11c, then +0xc0 with the first declaration, and answers the shader.
// Translation incomplete: 00bc93c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_no_lighting_create_shader(e: &mut Engine) -> Ptr {
    let mut made = Vec::with_capacity(DECLARATIONS.len());
    for (count, _) in DECLARATIONS {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        made.push(
            e.call(DECLARATION_CREATE, &args![renderer, count, 1u32])
                .u32(),
        );
    }
    for (declaration, (_, entries)) in made.iter().zip(DECLARATIONS) {
        for entry in entries {
            e.vcall(*declaration, 0x8c, entry);
        }
        e.vcall(*declaration, 0xb4, &args![]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x1b4u32]).u32();
    let shader = if block != 0 {
        let mut words = vec![block];
        words.extend_from_slice(&made);
        e.call(CONSTRUCT, &words).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![made[0]]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bb230`; no RTTI locator before it).
const NO_LIGHTING_VTABLE_010BB230: u32 = 0x010b_b230;
/// The four words copied (`011a9be0` into `01202010`).
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const COLOR_01202010: u32 = 0x0120_2010;

// Translated from 00bc8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::BSShaderNoLighting` (Xbox PDB), `thiscall` with twelve stack words
/// (`RET 0x30`), the twelve declarations. Runs `BSShader::BSShader` (`00be2200`), stores the
/// vtable `010bb230`, constructs the `NiPointer` arrays at +0x6c (0x1e, destructor
/// `00b79930`), +0xe4 (0x1c) and +0x154 (7; destructor `00b66f20`), clears +0x184 to +0x1b0,
/// sets +0x68 to 0x16, assigns the declarations in order to the `NiPointer`s at +0x184 to
/// +0x1b0, copies the four words at `011a9be0` to `01202010` and clears +0x170. Answers
/// `this`.
// Translation incomplete: 00bc8f80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xf for the base and the arrays) is not translated.
pub fn bs_shader_no_lighting_bs_shader_no_lighting(
    e: &mut Engine,
    this: Ptr,
    declaration_0: Ptr,
    declaration_1: Ptr,
    declaration_2: Ptr,
    declaration_3: Ptr,
    declaration_4: Ptr,
    declaration_5: Ptr,
    declaration_6: Ptr,
    declaration_7: Ptr,
    declaration_8: Ptr,
    declaration_9: Ptr,
    declaration_10: Ptr,
    declaration_11: Ptr,
) -> Ptr {
    let declarations = [
        declaration_0,
        declaration_1,
        declaration_2,
        declaration_3,
        declaration_4,
        declaration_5,
        declaration_6,
        declaration_7,
        declaration_8,
        declaration_9,
        declaration_10,
        declaration_11,
    ];
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NO_LIGHTING_VTABLE_010BB230);
    construct_pointer_array(e, base + 0x6c, 0x1e, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0xe4, 0x1c, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x154, 7, DESTRUCT_00B66F20);
    for index in 0..12u32 {
        e.mem.set_u32(base + 0x184 + 4 * index, 0);
    }
    e.mem.set_u32(base + 0x68, 0x16);
    for (index, declaration) in declarations.iter().enumerate() {
        ni_pointer_assign(e, base + 0x184 + 4 * index as u32, declaration.addr());
    }
    for word in [0u32, 8, 4, 0xc] {
        let value = e.global::<u32>(COLOR_011A9BE0 + word);
        e.set_global::<u32>(COLOR_01202010 + word, value);
    }
    e.mem.set_u32(base + 0x170, 0);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): the HLSL files `"lighting\1x\v\NoLighting.v.hlsl"`
/// (`010bb20c`), `"lighting\1x\v\particlesystem.v.hlsl"` (`010bb1e0`),
/// `"lighting\2x\v\waterfog.v.hlsl"` (`010bb1a8`), `"lighting\1x\p\NoLighting.p.hlsl"`
/// (`010bb178`) and `"lighting\2x\p\waterfog.p.hlsl"` (`010bb148`); the macros `"TEX"`,
/// `"VC"`, `"FALLOFF"`, `"SKIN"`, `"PSYS_SUBTEX"`, `"PSYS_STRIP"`, `"TEXCOORD"`,
/// `"PREMULT_ALPHA"`, `"ALPHATESTING"`, each with the value `""` (`01011584`); the vertex
/// name format `"NOLIGHT%03i.vso"` (`010bb198`) and profile `"vs_2_0"` (`010a93e4`); the pixel
/// names `"NOLIGHTTEX.pso"`, `"NOLIGHTVCONLY.pso"`, `"NOLIGHTTEXVC.pso"`,
/// `"NOLIGHTTEXVCPMA.pso"`, `"NOLIGHTMAT.pso"`, `"NOLIGHTWF.pso"`, `"NOLIGHTWFAt.pso"` and the
/// pixel profile `"ps_2_a"` (`010a93c4`).
const NL_VERTEX_SOURCE_010BB20C: u32 = 0x010b_b20c;
const PARTICLE_VERTEX_SOURCE_010BB1E0: u32 = 0x010b_b1e0;
const WATERFOG_VERTEX_SOURCE_010BB1A8: u32 = 0x010b_b1a8;
const NL_PIXEL_SOURCE_010BB178: u32 = 0x010b_b178;
const WATERFOG_PIXEL_SOURCE_010BB148: u32 = 0x010b_b148;
const MACRO_TEX_010AECAC: u32 = 0x010a_ecac;
const MACRO_VC_010AED64: u32 = 0x010a_ed64;
const MACRO_FALLOFF_010BB204: u32 = 0x010b_b204;
const MACRO_SKIN_010AEC10: u32 = 0x010a_ec10;
const MACRO_PSYS_SUBTEX_010BB1D4: u32 = 0x010b_b1d4;
const MACRO_PSYS_STRIP_010BB1C8: u32 = 0x010b_b1c8;
const MACRO_TEXCOORD_010AE930: u32 = 0x010a_e930;
const MACRO_PREMULT_ALPHA_010BB168: u32 = 0x010b_b168;
const MACRO_ALPHATESTING_010AEF00: u32 = 0x010a_ef00;
const NL_EMPTY_STRING_01011584: u32 = 0x0101_1584;
const VERTEX_NAME_FORMAT_010BB198: u32 = 0x010b_b198;
const PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
const NL_PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;
/// The compiled names of the seven pixel shaders, in compile order.
const PIXEL_OUTPUTS: [u32; 7] = [
    0x010b_b138,
    0x010b_b124,
    0x010b_b110,
    0x010b_b0fc,
    0x010b_b0ec,
    0x010b_b0dc,
    0x010b_b0cc,
];
/// The `LoadShaders` frame (0xc90 bytes up to the return address) and its buffers.
const NL_FRAME_BYTES: u32 = 0xc90;
const NL_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xb88,
    path: 0xa84,
};
/// The 0x1c vertex source blocks, 0x4c bytes apart from +0x19c (the loop's order).
const NL_VERTEX_BLOCKS: [ShaderSource; 28] = [
    ShaderSource {
        offset: 0x19c,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1e8,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x234,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x280,
        words: &[NL_VERTEX_SOURCE_010BB20C, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2cc,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x318,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x364,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3b0,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3fc,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x448,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x494,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x4e0,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x52c,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x578,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5c4,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
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
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x610,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x65c,
        words: &[PARTICLE_VERTEX_SOURCE_010BB1E0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x6a8,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_SUBTEX_010BB1D4,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6f4,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_STRIP_010BB1C8,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x740,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_STRIP_010BB1C8,
            NL_EMPTY_STRING_01011584,
            MACRO_PSYS_SUBTEX_010BB1D4,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x78c,
        words: &[WATERFOG_VERTEX_SOURCE_010BB1A8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x7d8,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x824,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x870,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x8bc,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x908,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x954,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x9a0,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
];
/// The seven pixel source blocks, in the order the exe fills and compiles them.
const NL_PIXEL_BLOCKS: [ShaderSource; 7] = [
    ShaderSource {
        offset: 0xb8,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x150,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6c,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x20,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_PREMULT_ALPHA_010BB168,
            NL_EMPTY_STRING_01011584,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xa38,
        words: &[NL_PIXEL_SOURCE_010BB178, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x9ec,
        words: &[WATERFOG_PIXEL_SOURCE_010BB148, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x104,
        words: &[
            WATERFOG_PIXEL_SOURCE_010BB148,
            MACRO_ALPHATESTING_010AEF00,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
];

// Translated from 00bc5c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills the 0x1c
/// vertex source blocks on its stack (the no-lighting, particle-system and waterfog vertex
/// files with their macro sets; each block's rest cleared by `memset` except one written in
/// full), then compiles each block whose file word is set: `MakePath`, the name formatted from
/// `"NOLIGHT%03i.vso"` with the block's index (`00406d00`), `CompileVertexShaderHLSL` with
/// `"vs_2_0"`, the result assigned to the `NiPointer` at +0xe4 + 4 * index. Then fills the seven
/// pixel source blocks and compiles them in order (`"ps_2_a"`, the names `NOLIGHTTEX.pso` ..
/// `NOLIGHTWFAt.pso`) into the `NiPointer`s at +0x154 to +0x16c.
// Translation incomplete: 00bc5c40 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_shader_no_lighting_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(NL_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &NL_VERTEX_BLOCKS);
        let path = frame + NL_BUFFERS.path;
        let name = frame + NL_BUFFERS.name;
        for (index, block) in NL_VERTEX_BLOCKS.iter().enumerate() {
            let index = index as u32;
            let source = frame + block.offset;
            let file = e.mem.u32(source);
            if file == 0 {
                continue;
            }
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, VERTEX_NAME_FORMAT_010BB198, index],
            );
            let shader = e
                .call(
                    COMPILE_VERTEX_SHADER,
                    &args![base, path, source + 4, PROFILE_VS_2_0_010A93E4, name],
                )
                .u32();
            ni_pointer_assign(e, base + 0xe4 + 4 * index, shader);
        }
        set_up_sources(e, frame, &NL_PIXEL_BLOCKS);
        for (index, block) in NL_PIXEL_BLOCKS.iter().enumerate() {
            let step = ShaderCompile {
                source: block.offset,
                output: PIXEL_OUTPUTS[index],
                profile: NL_PROFILE_PS_2_A_010A93C4,
                pixel: true,
                slot: 0x154 + 4 * index as u32,
            };
            compile_shaders(e, base, frame, &NL_BUFFERS, &[step]);
        }
    });
}

/// The passes of `BSShaderNoLighting::LoadStagesAndPasses`, in the exe's order (immediates of
/// its straight-line code): the pass pointer's offset, whether the pass gets a texture stage,
/// the vertex and pixel shader `NiPointer`s of the shader it takes, and whether it gets the
/// render state 0xa8 = 3.
const NO_LIGHTING_PASSES: [(u32, bool, u32, u32, bool); 30] = [
    (0x6c, true, 0xe4, 0x154, false),
    (0x70, true, 0xec, 0x15c, false),
    (0x74, false, 0xe8, 0x158, false),
    (0x78, false, 0xf0, 0x164, false),
    (0x7c, true, 0xf4, 0x154, false),
    (0x80, true, 0xfc, 0x15c, false),
    (0x84, false, 0xf8, 0x158, false),
    (0x88, false, 0x100, 0x164, false),
    (0x8c, true, 0x104, 0x154, false),
    (0x90, true, 0x10c, 0x15c, false),
    (0x94, false, 0x108, 0x158, false),
    (0x98, false, 0x110, 0x164, false),
    (0x9c, true, 0x114, 0x154, false),
    (0xa0, true, 0x11c, 0x15c, false),
    (0xa4, false, 0x118, 0x158, false),
    (0xa8, false, 0x120, 0x164, false),
    (0xac, true, 0x124, 0x15c, false),
    (0xb0, true, 0x128, 0x15c, false),
    (0xb4, true, 0x124, 0x160, false),
    (0xb8, true, 0x128, 0x160, false),
    (0xbc, true, 0x12c, 0x15c, false),
    (0xc0, true, 0x130, 0x15c, false),
    (0xc4, false, 0x134, 0x168, true),
    (0xc8, true, 0x138, 0x16c, true),
    (0xcc, false, 0x13c, 0x168, true),
    (0xd0, true, 0x140, 0x16c, true),
    (0xd4, false, 0x144, 0x168, true),
    (0xd8, true, 0x148, 0x16c, true),
    (0xdc, false, 0x14c, 0x168, true),
    (0xe0, true, 0x150, 0x16c, true),
];

// Translated from 00bc77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers
/// 1. For each of the 30 pass pointers from +0x6c to +0xe0 in order: a new pass (`00e804d0`);
/// for 18 of them a texture stage made into one stack pointer (`CreateNewStage`; earlier
/// stages are not dropped before the next is made), `SetupDefaultStage` index 0 with 3, 2, 1
/// (no filter mode) and `SetStage` with the pass's word at +0x14; the pass's vertex shader
/// (+0x5c) and pixel shader (+0x44) `NiPointer`s assigned from the shader's (vertex +0xe4 to
/// +0x150, pixel +0x154 to +0x16c, see the table); for the last eight the render state 0xa8 = 3.
/// Finally the stage reference is dropped.
// Translation incomplete: 00bc77e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn bs_shader_no_lighting_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (pass_offset, stage, vertex, pixel, alpha_state) in NO_LIGHTING_PASSES {
            let slot = base + pass_offset;
            e.call(NEW_PASS_00E804D0, &args![slot]);
            if stage {
                e.call(CREATE_NEW_STAGE, &args![local]);
                let stage = e.mem.u32(local);
                e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
                let pass = e.mem.u32(slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            }
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
            if alpha_state {
                let pass = e.mem.u32(slot);
                pass_render_state(e, pass, 0xa8, 3);
            }
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_77e0, bs_shader_no_lighting_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00bc_5c40, bs_shader_no_lighting_load_shaders(Ptr)),
        entry!(0x00bc_8f80, bs_shader_no_lighting_bs_shader_no_lighting(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00bc_93c0,
            bs_shader_no_lighting_create_shader() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_no_lighting_create_shader_makes_twelve_declarations_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bc_93c0);
        assert_eq!(shader, SHADER);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let mut want: Vec<(u32, Vec<u32>)> = [2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        // The entries as pushed in the exe, declaration by declaration.
        let lists: [&[[u32; 5]]; 12] = [
            &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 3, 2, 0],
                [2, 5, 4, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
                [5, 7, 5, 1, 0],
            ],
        ];
        for (n, list) in lists.iter().enumerate() {
            let declaration = d(n as u32);
            for entry in list.iter() {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                want.push((declaration_slot(0x8c), words));
            }
            want.push((declaration_slot(0xb4), vec![declaration]));
        }
        want.push((NI_ALLOCATE, vec![0x1b4]));
        let mut construct = vec![SHADER];
        construct.extend((0..12).map(d));
        want.push((CONSTRUCT, construct));
        want.push((shader_slot(0x4c), vec![SHADER]));
        want.push((LOAD_SHADERS, vec![SHADER]));
        want.push((LOAD_STAGES_AND_PASSES, vec![SHADER]));
        want.push((shader_slot(0x11c), vec![SHADER]));
        want.push((shader_slot(0xc0), vec![SHADER, d(0)]));
        assert_eq!(log, want);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_constructor_keeps_the_twelve_declarations() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BE0 + 4 * index, 0x60 + index);
        }
        let this = e.mem.alloc(0x1b4);
        e.mem.write(this, &[0xcc; 0x1b4]);
        let declarations: Vec<u32> = (0..12).map(|_| object(&mut e, 0)).collect();
        let mut words = vec![this];
        words.extend_from_slice(&declarations);
        assert_eq!(e.call(CONSTRUCT, &words).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NO_LIGHTING_VTABLE_010BB230);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x1e, DESTRUCT_00B79930),
                pointer_array(this + 0xe4, 0x1c, DESTRUCT_00B66F20),
                pointer_array(this + 0x154, 7, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0x16);
        for (index, declaration) in declarations.iter().enumerate() {
            assert_eq!(e.mem.u32(this + 0x184 + 4 * index as u32), *declaration);
            assert_eq!(e.mem.u32(declaration + 4), 1);
        }
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(COLOR_01202010 + 4 * index), 0x60 + index);
        }
        assert_eq!(e.mem.u32(this + 0x170), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_load_shaders_compiles_28_vertex_and_7_pixel_shaders() {
        let mut e = loader_rig();
        let this = e.mem.alloc(0x1b4);
        e.call(LOAD_SHADERS, &args![this]);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 35);
        for (index, format) in formats[..28].iter().enumerate() {
            assert_eq!(format[2..], [VERTEX_NAME_FORMAT_010BB198, index as u32]);
        }
        let tex = (MACRO_TEX_010AECAC, NL_EMPTY_STRING_01011584);
        let vc = (MACRO_VC_010AED64, NL_EMPTY_STRING_01011584);
        let falloff = (MACRO_FALLOFF_010BB204, NL_EMPTY_STRING_01011584);
        let skin = (MACRO_SKIN_010AEC10, NL_EMPTY_STRING_01011584);
        // A few vertex shaders: index 0 (TEX), 3 (no macros), 14 (TEX VC FALLOFF SKIN, the
        // block written in full), 15 (FALLOFF SKIN), 16 (particles), 20 (waterfog).
        let expected = [
            (0usize, NL_VERTEX_SOURCE_010BB20C, macro_words(&[tex])),
            (3, NL_VERTEX_SOURCE_010BB20C, [0; 8]),
            (
                14,
                NL_VERTEX_SOURCE_010BB20C,
                macro_words(&[tex, vc, falloff, skin]),
            ),
            (15, NL_VERTEX_SOURCE_010BB20C, macro_words(&[falloff, skin])),
            (16, PARTICLE_VERTEX_SOURCE_010BB1E0, [0; 8]),
            (20, WATERFOG_VERTEX_SOURCE_010BB1A8, [0; 8]),
        ];
        for (index, file, macros) in expected {
            let shader = e.mem.u32(this + 0xe4 + 4 * index as u32);
            let seen = compiled(&e, shader);
            assert_eq!(seen.file, file, "{index}");
            assert_eq!(seen.output, VERTEX_NAME_FORMAT_010BB198);
            assert_eq!(seen.profile, PROFILE_VS_2_0_010A93E4);
            assert!(!seen.pixel);
            assert_eq!(seen.macros, macros, "{index}");
        }
        for index in 0..28u32 {
            assert_ne!(e.mem.u32(this + 0xe4 + 4 * index), 0);
        }
        let pixel_files = [
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            WATERFOG_PIXEL_SOURCE_010BB148,
            WATERFOG_PIXEL_SOURCE_010BB148,
        ];
        for (index, file) in pixel_files.iter().enumerate() {
            let shader = e.mem.u32(this + 0x154 + 4 * index as u32);
            let seen = compiled(&e, shader);
            assert_eq!(seen.file, *file);
            assert_eq!(seen.output, PIXEL_OUTPUTS[index]);
            assert_eq!(seen.profile, NL_PROFILE_PS_2_A_010A93C4);
            assert!(seen.pixel);
        }
        let premultiplied = compiled(&e, e.mem.u32(this + 0x160));
        assert_eq!(
            premultiplied.macros[..6],
            [
                MACRO_PREMULT_ALPHA_010BB168,
                NL_EMPTY_STRING_01011584,
                MACRO_TEX_010AECAC,
                NL_EMPTY_STRING_01011584,
                MACRO_VC_010AED64,
                NL_EMPTY_STRING_01011584
            ]
        );
        let alpha_tested = compiled(&e, e.mem.u32(this + 0x16c));
        assert_eq!(
            alpha_tested.macros[..2],
            [MACRO_ALPHATESTING_010AEF00, NL_EMPTY_STRING_01011584]
        );
        // 0x1c + 7 blocks, one of them written in full.
        assert_eq!(calls(&e, MEMSET).len(), 34);
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_load_stages_and_passes_builds_thirty_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x1b4);
        for offset in (0xe4u32..=0x16c).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        let made: Vec<u32> = calls(&e, NEW_PASS_00E804D0).iter().map(|a| a[0]).collect();
        assert_eq!(
            made,
            (0x6cu32..=0xe0)
                .step_by(4)
                .map(|o| this + o)
                .collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 18);
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        for setup in calls(&e, SETUP_DEFAULT_STAGE) {
            assert_eq!(setup[1..], [0, 3, 2, 1]);
        }
        // Spot checks: a pass with a stage, one without, one with the render state.
        let first = e.mem.u32(this + 0x6c);
        assert_eq!(calls(&e, PASS_SET_STAGE)[0][..2], [first, 0]);
        assert_eq!(e.mem.u32(first + 0x5c), e.mem.u32(this + 0xe4));
        assert_eq!(e.mem.u32(first + 0x44), e.mem.u32(this + 0x154));
        let third = e.mem.u32(this + 0x74);
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != third));
        assert_eq!(e.mem.u32(third + 0x5c), e.mem.u32(this + 0xe8));
        assert_eq!(e.mem.u32(third + 0x44), e.mem.u32(this + 0x158));
        assert!(render_states(&e, third).is_empty());
        let last = e.mem.u32(this + 0xe0);
        assert_eq!(render_states(&e, last), vec![(0xa8, 3)]);
        assert_eq!(e.mem.u32(last + 0x5c), e.mem.u32(this + 0x150));
        assert_eq!(e.mem.u32(last + 0x44), e.mem.u32(this + 0x16c));
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 8);
        // Only the last stage's reference is dropped.
        let stages_set: Vec<u32> = calls(&e, PASS_SET_STAGE).iter().map(|a| a[2]).collect();
        assert_eq!(e.mem.u32(stages_set[17] + 0x60), 1);
        assert_eq!(e.mem.u32(stages_set[0] + 0x60), 2);
    }
}
