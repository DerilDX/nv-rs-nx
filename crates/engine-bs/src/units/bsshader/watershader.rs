//! `BSShader/watershader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1d while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_WATER: u32 = 0x1d;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `WaterShader::WaterShader` (`00b86bd0`), `WaterShader::LoadShaders` (`00b827c0`) and the
/// method `00b83dd0` of this unit (engine map; all `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_6bd0;
const LOAD_SHADERS: u32 = 0x00b8_27c0;
const WATER_METHOD_00B83DD0: u32 = 0x00b8_3dd0;

// Translated from 00b86de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0x11. No arguments. With the thread's allocation context (+0x2b4 of the thread
/// block) at 0x1d (put back at the end): makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 4 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 4, 0), (3, 7, 5, 1, 0)) and calls slot +0xb4. Then allocates 0x250
/// bytes and runs the constructor (`00b86bd0`; a null allocation is used as it is), calls
/// the shader's slot +0x4c, `LoadShaders` (`00b827c0`), `00b83dd0`, slot +0x11c, then +0xc0
/// with the declaration, and answers the shader.
// Translation incomplete: 00b86de0 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn water_shader_create_shader(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_WATER);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 4, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x250u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(WATER_METHOD_00B83DD0, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The water shader's vtable (`010af9e0`; no RTTI locator before it).
const WATER_SHADER_VTABLE_010AF9E0: u32 = 0x010a_f9e0;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
/// The globals the constructor sets: the shader itself (`011ff370`), four `float`s
/// (`011ff458`..`011ff464`) and the flags and counters at `011ff100`..`011ff11a`.
const WATER_SHADER_011FF370: u32 = 0x011f_f370;
const FLOATS_011FF458: u32 = 0x011f_f458;

// Translated from 00b86bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::WaterShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010af9e0`, constructs the `NiPointer`
/// arrays at +0x6c (0x4a, destructor `00b79930`) and +0x1a0 (0x26, destructor `00b66f20`) with
/// the words +0x194 to +0x19c cleared between them, clears +0x238 to +0x24c, sets +0x68 to 0xd
/// and the byte +0x24 to 1, clears (releasing what they hold) the `NiPointer`s at +0x23c,
/// +0x244, +0x238 and +0x248, keeps the shader in `011ff370`, stores 0.0 in the `float`s
/// `011ff458` to `011ff464`, clears the bytes `011ff104`, `011ff103`, `011ff102` and, unless the
/// shader model setting `011f91c0` is above 2 (signed), `011ff100`; then clears the 16-bit
/// `011ff118`, the byte `011ff11a`, the word `011ff110` and the 16-bit `011ff114`. Answers
/// `this`.
// Translation incomplete: 00b86bd0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xb for the base and the arrays) is not translated.
pub fn water_shader_water_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, WATER_SHADER_VTABLE_010AF9E0);
    construct_pointer_array(e, base + 0x6c, 0x4a, DESTRUCT_00B79930);
    for offset in [0x194u32, 0x198, 0x19c] {
        e.mem.set_u32(base + offset, 0);
    }
    construct_pointer_array(e, base + 0x1a0, 0x26, DESTRUCT_00B66F20);
    for offset in (0x238u32..=0x24c).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0xd);
    e.mem.set_u8(base + 0x24, 1);
    for offset in [0x23cu32, 0x244, 0x238, 0x248] {
        ni_pointer_clear(e, base + offset);
    }
    let above_model_2 = e.global::<i32>(SHADER_MODEL_011F91C0) > 2;
    e.set_global::<u32>(WATER_SHADER_011FF370, base);
    for index in 0..4u32 {
        e.set_global::<f32>(FLOATS_011FF458 + 4 * index, 0.0);
    }
    for flag in [0x011f_f104u32, 0x011f_f103, 0x011f_f102] {
        e.set_global::<u8>(flag, 0);
    }
    if !above_model_2 {
        e.set_global::<u8>(0x011f_f100, 0);
    }
    e.set_global::<u16>(0x011f_f118, 0);
    e.set_global::<u8>(0x011f_f11a, 0);
    e.set_global::<u32>(0x011f_f110, 0);
    e.set_global::<u16>(0x011f_f114, 0);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): the vertex file `"water\\2_ab\\v\\water.v.hlsl"`
/// (`010afc20`) with the macros `"VERTEX_COLORS"` (`010afc00`) and `"LIGHTING"` (`010afbe4`),
/// the names `"WATER000.vso"` .. `"WATER002.vso"` and the profile `"vs_1_1"` (`010a93ec`); the
/// pixel files `"water\\2_ab\\p\\water.p.hlsl"` (`010afbb8`, shader model above 4) or
/// `"water\\2_0\\p\\water.p.hlsl"` (`010afb9c`) and `"water\\2_ab\\p\\waterlighting.p.hlsl"`
/// (`010afb78`), copied into stack buffers; the pixel macros `"WATER"`, `"REFLECTIONS"`,
/// `"REFRACTIONS"`, `"DEPTH"`, `"FOG"`, `"INTERIORWATER"`, `"UNDERWATER"`, `"WADING"`,
/// `"STENCIL"`, `"LOD"`, each with the value `""` (`01011584`); the name format
/// `"WATER%03i.pso"` (`010afb1c`); the pixel profiles `"ps_2_0"`, `"ps_2_a"`, `"ps_2_b"`,
/// `"ps_3_0"`.
const W_VERTEX_SOURCE_010AFC20: u32 = 0x010a_fc20;
const W_MACRO_VERTEX_COLORS_010AFC00: u32 = 0x010a_fc00;
const W_MACRO_LIGHTING_010AFBE4: u32 = 0x010a_fbe4;
const W_PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const W_PIXEL_SOURCE_2_AB_010AFBB8: u32 = 0x010a_fbb8;
const W_PIXEL_SOURCE_2_0_010AFB9C: u32 = 0x010a_fb9c;
const W_LIGHTING_SOURCE_010AFB78: u32 = 0x010a_fb78;
const W_PIXEL_NAME_FORMAT_010AFB1C: u32 = 0x010a_fb1c;
const W_EMPTY: u32 = 0x0101_1584;
const W_WATER: u32 = 0x0102_3014;
const W_REFLECTIONS: u32 = 0x010a_fb6c;
const W_REFRACTIONS: u32 = 0x010a_fb58;
const W_DEPTH: u32 = 0x010a_fb64;
const W_FOG: u32 = 0x010a_ece0;
const W_INTERIORWATER: u32 = 0x010a_fb48;
const W_UNDERWATER: u32 = 0x010a_fb3c;
const W_WADING: u32 = 0x010a_fb34;
const W_STENCIL: u32 = 0x010a_fb2c;
const W_LOD: u32 = 0x0103_9578;
/// The pixel profiles by shader model: `"ps_3_0"` (`010a93b4`, model 7 and above), `"ps_2_b"`
/// (`010a93bc`, 6), `"ps_2_a"` (`010a93c4`, 5), `"ps_2_0"` (`010a93cc`, otherwise).
const W_PROFILE_PS_3_0_010A93B4: u32 = 0x010a_93b4;
const W_PROFILE_PS_2_B_010A93BC: u32 = 0x010a_93bc;
const W_PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;
const W_PROFILE_PS_2_0_010A93CC: u32 = 0x010a_93cc;
/// The `LoadShaders` frame (0x1070 bytes up to the return address) and its buffers: name
/// +0xd58, path +0xe5c, the pixel file +0xc58 and the lighting file +0xf60 (0x100 each).
const W_FRAME_BYTES: u32 = 0x1070;
const W_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xd58,
    path: 0xe5c,
};
const W_PIXEL_FILE_BUFFER: u32 = 0xc58;
const W_LIGHTING_FILE_BUFFER: u32 = 0xf60;
/// The vertex steps: the source block, the compiled name and the shader field.
const W_VERTEX_STEPS: [(ShaderSource, u32, u32); 3] = [
    (
        ShaderSource {
            offset: 0xb74,
            words: &[W_VERTEX_SOURCE_010AFC20, 0, 0, 0, 0],
            cleared: 0x38,
        },
        0x010a_fc10,
        0x194,
    ),
    (
        ShaderSource {
            offset: 0xc0c,
            words: &[
                W_VERTEX_SOURCE_010AFC20,
                W_MACRO_VERTEX_COLORS_010AFC00,
                W_EMPTY,
                0,
            ],
            cleared: 0x3c,
        },
        0x010a_fbf0,
        0x198,
    ),
    (
        ShaderSource {
            offset: 0xbc0,
            words: &[
                W_VERTEX_SOURCE_010AFC20,
                W_MACRO_LIGHTING_010AFBE4,
                W_EMPTY,
                0,
            ],
            cleared: 0x3c,
        },
        0x010a_fbd4,
        0x19c,
    ),
];

/// One pixel source block of the water `LoadShaders`: the lighting file or the water file in
/// its first word, the macro words after it and the bytes cleared after those.
struct WaterPixelBlock {
    lighting: bool,
    macros: &'static [u32],
    cleared: u32,
}

/// The 0x26 pixel source blocks, 0x4c bytes apart from +0x2c.
const W_PIXEL_BLOCKS: [WaterPixelBlock; 38] = [
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_DEPTH, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_REFRACTIONS, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_REFLECTIONS, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x34,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
        ],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_UNDERWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
        ],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER, W_EMPTY, W_WADING, W_EMPTY, W_DEPTH, W_EMPTY, W_FOG, W_EMPTY, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_WADING, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_LOD,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_LOD, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_STENCIL, W_EMPTY, 0],
        cleared: 0x3c,
    },
    WaterPixelBlock {
        lighting: true,
        macros: &[0],
        cleared: 0x44,
    },
    WaterPixelBlock {
        lighting: true,
        macros: &[W_WADING, W_EMPTY, 0],
        cleared: 0x3c,
    },
];

// Translated from 00b827c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. With the thread
/// allocation context (+0x2b4 of the thread block) at 0x1d (put back at the end): compiles the
/// three vertex shaders (`"WATER000.vso"` .. `"WATER002.vso"`, `vs_1_1`, without macros, with
/// `VERTEX_COLORS` and with `LIGHTING`) into the `NiPointer`s at +0x194, +0x198, +0x19c; picks
/// the pixel profile from the shader model setting (`011f91c0`: 7 and above `ps_3_0`, 6
/// `ps_2_b`, 5 `ps_2_a`, else `ps_2_0`); copies the water pixel file (the `2_ab` one above model
/// 4, else the `2_0` one) and the lighting file into 0x100-byte stack buffers (`00406d00`);
/// fills the 0x26 pixel source blocks (the buffers' addresses as file names, their macro sets,
/// the rest of some blocks cleared by `memset`) and compiles each whose file word is set as
/// `"WATER%03i.pso"` with its index into the `NiPointer` at +0x1a0 + 4 * index.
// Translation incomplete: 00b827c0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS stack cookie check (00ec408c) are not translated.
pub fn water_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_WATER);
    e.with_stack(W_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        let path = frame + W_BUFFERS.path;
        let name = frame + W_BUFFERS.name;
        for (block, output, slot) in W_VERTEX_STEPS {
            set_up_sources(e, frame, core::slice::from_ref(&block));
            let step = ShaderCompile {
                source: block.offset,
                output,
                profile: W_PROFILE_VS_1_1_010A93EC,
                pixel: false,
                slot,
            };
            compile_shaders(e, base, frame, &W_BUFFERS, &[step]);
        }
        let model = e.global::<i32>(SHADER_MODEL_011F91C0);
        let profile = if model >= 7 {
            W_PROFILE_PS_3_0_010A93B4
        } else if model >= 6 {
            W_PROFILE_PS_2_B_010A93BC
        } else if model >= 5 {
            W_PROFILE_PS_2_A_010A93C4
        } else {
            W_PROFILE_PS_2_0_010A93CC
        };
        let pixel_file = frame + W_PIXEL_FILE_BUFFER;
        let lighting_file = frame + W_LIGHTING_FILE_BUFFER;
        let water_source = if model > 4 {
            W_PIXEL_SOURCE_2_AB_010AFBB8
        } else {
            W_PIXEL_SOURCE_2_0_010AFB9C
        };
        e.call(FORMAT_00406D00, &args![pixel_file, 0x100u32, water_source]);
        e.call(
            FORMAT_00406D00,
            &args![lighting_file, 0x100u32, W_LIGHTING_SOURCE_010AFB78],
        );
        for (index, block) in W_PIXEL_BLOCKS.iter().enumerate() {
            let at = frame + 0x2c + 0x4c * index as u32;
            let file = if block.lighting {
                lighting_file
            } else {
                pixel_file
            };
            e.mem.set_u32(at, file);
            for (word, value) in block.macros.iter().enumerate() {
                e.mem.set_u32(at + 4 + 4 * word as u32, *value);
            }
            if block.cleared != 0 {
                let rest = at + 4 + 4 * block.macros.len() as u32;
                e.call(MEMSET, &args![rest, 0u32, block.cleared]);
            }
        }
        for index in 0..0x26u32 {
            let source = frame + 0x2c + 0x4c * index;
            let file = e.mem.u32(source);
            if file == 0 {
                continue;
            }
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, W_PIXEL_NAME_FORMAT_010AFB1C, index],
            );
            let shader = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![base, path, source + 4, profile, name],
                )
                .u32();
            ni_pointer_assign(e, base + 0x1a0 + 4 * index, shader);
        }
    });
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
}

/// The shader setters `00b79950` (vertex: the pass +0x5c) and `00b80600` (pixel: the pass
/// +0x44) (unnamed; `thiscall` on the pass, the shader) and the blend setter `00b80040`
/// (unnamed; `thiscall` on the pass, three words: the states 0x13 and 0x14, an unread word).
const SET_VERTEX_SHADER_00B79950: u32 = 0x00b7_9950;
const SET_PIXEL_SHADER_00B80600: u32 = 0x00b8_0600;
const SET_BLEND_00B80040: u32 = 0x00b8_0040;

/// How a pass of the second part of `00b83dd0` gets its shaders: both `NiPointer`
/// assignments expanded inline, the vertex one inline and the pixel one through `00b80600`,
/// or both through the setters.
#[derive(Clone, Copy)]
enum WaterAssign {
    Inline,
    InlineVertex,
    Helpers,
}

/// The second part of `00b83dd0`, in the exe's order: the pass pointer, the vertex and pixel
/// shader `NiPointer`s it takes, the word stored after them and its value, and how the
/// shaders are assigned.
const WATER_ASSIGNMENTS: [(u32, u32, u32, u32, u32, WaterAssign); 74] = [
    (0x6c, 0x194, 0x1a0, 0x011f_f120, 0x3e, WaterAssign::Inline),
    (0x70, 0x194, 0x1a4, 0x011f_f124, 0x3e, WaterAssign::Inline),
    (0x74, 0x194, 0x1a8, 0x011f_f128, 0x3e, WaterAssign::Inline),
    (0x78, 0x194, 0x1ac, 0x011f_f12c, 0x3e, WaterAssign::Inline),
    (0x7c, 0x194, 0x1b0, 0x011f_f130, 0x2e, WaterAssign::Inline),
    (0x80, 0x194, 0x1b4, 0x011f_f134, 0x2e, WaterAssign::Inline),
    (0x84, 0x194, 0x1b8, 0x011f_f138, 0x2e, WaterAssign::Inline),
    (0x88, 0x194, 0x1bc, 0x011f_f13c, 0x2e, WaterAssign::Inline),
    (0x8c, 0x194, 0x1c0, 0x011f_f140, 0x1e, WaterAssign::Inline),
    (0x90, 0x194, 0x1c4, 0x011f_f144, 0x1e, WaterAssign::Inline),
    (0x94, 0x194, 0x1c8, 0x011f_f148, 0x1e, WaterAssign::Inline),
    (0x98, 0x194, 0x1cc, 0x011f_f14c, 0x1e, WaterAssign::Inline),
    (0x9c, 0x194, 0x1d0, 0x011f_f150, 0xe, WaterAssign::Inline),
    (0xa0, 0x194, 0x1d4, 0x011f_f154, 0xe, WaterAssign::Inline),
    (0xa4, 0x194, 0x1d8, 0x011f_f158, 0xe, WaterAssign::Inline),
    (0xa8, 0x194, 0x1dc, 0x011f_f15c, 0xe, WaterAssign::Inline),
    (0xac, 0x194, 0x1e0, 0x011f_f160, 0xe, WaterAssign::Inline),
    (0xb0, 0x194, 0x1e4, 0x011f_f164, 0x3e, WaterAssign::Inline),
    (0xb4, 0x194, 0x1e8, 0x011f_f168, 0x3e, WaterAssign::Inline),
    (0xb8, 0x194, 0x1ec, 0x011f_f16c, 0x3e, WaterAssign::Inline),
    (0xbc, 0x194, 0x1f0, 0x011f_f170, 0x3e, WaterAssign::Inline),
    (0xc0, 0x194, 0x1f4, 0x011f_f174, 0x2e, WaterAssign::Inline),
    (0xc4, 0x194, 0x1f8, 0x011f_f178, 0x2e, WaterAssign::Inline),
    (0xc8, 0x194, 0x1fc, 0x011f_f17c, 0x2e, WaterAssign::Inline),
    (0xcc, 0x194, 0x200, 0x011f_f180, 0x2e, WaterAssign::Inline),
    (0xd0, 0x194, 0x204, 0x011f_f184, 0x1e, WaterAssign::Inline),
    (0xd4, 0x194, 0x208, 0x011f_f188, 0x1e, WaterAssign::Inline),
    (0xd8, 0x194, 0x20c, 0x011f_f18c, 0x1e, WaterAssign::Inline),
    (0xdc, 0x194, 0x210, 0x011f_f190, 0x1e, WaterAssign::Inline),
    (0xe0, 0x194, 0x214, 0x011f_f190, 0xe, WaterAssign::Inline),
    (0xe4, 0x194, 0x218, 0x011f_f198, 0xe, WaterAssign::Inline),
    (0xe8, 0x194, 0x21c, 0x011f_f19c, 0xe, WaterAssign::Inline),
    (0xec, 0x194, 0x220, 0x011f_f1a0, 0xe, WaterAssign::Inline),
    (0xf0, 0x194, 0x224, 0x011f_f1a4, 0x2e, WaterAssign::Inline),
    (0xf4, 0x194, 0x228, 0x011f_f1a8, 0x2e, WaterAssign::Inline),
    (0xf8, 0x194, 0x22c, 0x011f_f1ac, 0x0, WaterAssign::Inline),
    (0xfc, 0x19c, 0x230, 0x011f_f1b0, 0x4e, WaterAssign::Inline),
    (0x100, 0x19c, 0x234, 0x011f_f1b4, 0x4e, WaterAssign::Inline),
    (0x104, 0x198, 0x1a0, 0x011f_f1b8, 0x3e, WaterAssign::Inline),
    (0x108, 0x198, 0x1a4, 0x011f_f1bc, 0x3e, WaterAssign::Inline),
    (0x10c, 0x198, 0x1a8, 0x011f_f1c0, 0x3e, WaterAssign::Inline),
    (0x110, 0x198, 0x1ac, 0x011f_f1c4, 0x3e, WaterAssign::Inline),
    (0x114, 0x198, 0x1b0, 0x011f_f1c8, 0x2e, WaterAssign::Inline),
    (0x118, 0x198, 0x1b4, 0x011f_f1cc, 0x2e, WaterAssign::Inline),
    (0x11c, 0x198, 0x1b8, 0x011f_f1d0, 0x2e, WaterAssign::Inline),
    (0x120, 0x198, 0x1bc, 0x011f_f1d4, 0x2e, WaterAssign::Inline),
    (0x124, 0x198, 0x1c0, 0x011f_f1d8, 0x1e, WaterAssign::Inline),
    (0x128, 0x198, 0x1c4, 0x011f_f1dc, 0x1e, WaterAssign::Inline),
    (0x12c, 0x198, 0x1c8, 0x011f_f1e0, 0x1e, WaterAssign::Inline),
    (0x130, 0x198, 0x1cc, 0x011f_f1e4, 0x1e, WaterAssign::Inline),
    (
        0x134,
        0x198,
        0x1d0,
        0x011f_f1e8,
        0xe,
        WaterAssign::InlineVertex,
    ),
    (0x138, 0x198, 0x1d4, 0x011f_f1ec, 0xe, WaterAssign::Helpers),
    (0x13c, 0x198, 0x1d8, 0x011f_f1f0, 0xe, WaterAssign::Helpers),
    (0x140, 0x198, 0x1dc, 0x011f_f1f4, 0xe, WaterAssign::Helpers),
    (0x144, 0x198, 0x1e0, 0x011f_f1f8, 0xe, WaterAssign::Helpers),
    (0x148, 0x198, 0x1e4, 0x011f_f1fc, 0x3e, WaterAssign::Helpers),
    (0x14c, 0x198, 0x1e8, 0x011f_f200, 0x3e, WaterAssign::Helpers),
    (0x150, 0x198, 0x1ec, 0x011f_f204, 0x3e, WaterAssign::Helpers),
    (0x154, 0x198, 0x1f0, 0x011f_f208, 0x3e, WaterAssign::Helpers),
    (0x158, 0x198, 0x1f4, 0x011f_f20c, 0x2e, WaterAssign::Helpers),
    (0x15c, 0x198, 0x1f8, 0x011f_f210, 0x2e, WaterAssign::Helpers),
    (0x160, 0x198, 0x1fc, 0x011f_f214, 0x2e, WaterAssign::Helpers),
    (0x164, 0x198, 0x200, 0x011f_f218, 0x2e, WaterAssign::Helpers),
    (0x168, 0x194, 0x204, 0x011f_f21c, 0x1e, WaterAssign::Helpers),
    (0x16c, 0x194, 0x208, 0x011f_f220, 0x1e, WaterAssign::Helpers),
    (0x170, 0x194, 0x20c, 0x011f_f224, 0x1e, WaterAssign::Helpers),
    (0x174, 0x194, 0x210, 0x011f_f228, 0x1e, WaterAssign::Helpers),
    (0x178, 0x194, 0x214, 0x011f_f228, 0xe, WaterAssign::Helpers),
    (0x17c, 0x194, 0x218, 0x011f_f230, 0xe, WaterAssign::Helpers),
    (0x180, 0x194, 0x21c, 0x011f_f234, 0xe, WaterAssign::Helpers),
    (0x184, 0x194, 0x220, 0x011f_f238, 0xe, WaterAssign::Helpers),
    (0x188, 0x198, 0x22c, 0x011f_f23c, 0x0, WaterAssign::Helpers),
    (0x18c, 0x19c, 0x230, 0x011f_f240, 0x4e, WaterAssign::Helpers),
    (0x190, 0x19c, 0x234, 0x011f_f244, 0x4e, WaterAssign::Helpers),
];

/// A render state set on the pass the slot holds, read again first.
fn water_state(e: &mut Engine, slot: u32, state: u32, value: u32) {
    let pass = e.mem.u32(slot);
    pass_render_state(e, pass, state, value);
}

// Translated from 00b83dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `watershader.cpp`): the water shader's stages and passes
/// (the creator `00b86de0` runs it after `LoadShaders`), `thiscall`, no stack words; answers 1.
///
/// For each index from 0 to 0x49 whose pass pointer (+0x6c + 4 * index) is null: a new pass
/// (`00e804d0`) with five texture stages (made into one stack pointer cleared first,
/// `SetupDefaultStage` with the stage index and 3, 2, 1 (stages 2 and 3: 1, 2, 1),
/// `SetFilterMode(1)`, `SetStage`), the render states 0xf = 1, 0x18 = 0, 0x19 = 5; unless the
/// index is 0x21, 0x1b = 1 and then 0x13 = 5, 0x14 = 2 (through the same pass read) for
/// indices 0x24, 0x48, 0x25, 0x49, else the blend setter `00b80040(5, 6, 0)`; for indices
/// 0x11 to 0x1c and 0x37 to 0x42 the state 0x34 = 1, else for 0 to 0x1c, 0x26 to 0x42, 0x24 and
/// 0x48 the states 0x34 = 1, 0x38 = 6, 0x39 = 1, 0x3a = 1, 0x37 = 1; for indices 0x23 and 0x47
/// the states 0xa8 = 0, 0xe = 0, 0x34 = 1, 0x38 = 3, 0x39 = 0xff, 0x37 = 3, 0x3a = 0; then the
/// stage reference is dropped.
///
/// Then, for the 0x4a passes in order (see `WATER_ASSIGNMENTS`), the vertex shader (+0x5c) and
/// pixel shader (+0x44) assigned (inline or through `00b79950` / `00b80600`) and a word of the
/// table at `011ff120` set (the exe writes `011ff190` and `011ff228` twice and never
/// `011ff194` and `011ff22c`).
// Translation incomplete: 00b83dd0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00b83dd0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in 0..0x4au32 {
            let slot = base + 0x6c + 4 * index;
            if e.mem.u32(slot) != 0 {
                continue;
            }
            e.call(NEW_PASS_00E804D0, &args![slot]);
            e.mem.set_u32(local, 0);
            for (stage_index, setup) in [
                (0u32, [3u32, 2, 1]),
                (1, [3, 2, 1]),
                (2, [1, 2, 1]),
                (3, [1, 2, 1]),
                (4, [3, 2, 1]),
            ] {
                e.call(CREATE_NEW_STAGE, &args![local]);
                let stage = e.mem.u32(local);
                e.call(
                    SETUP_DEFAULT_STAGE,
                    &args![stage, stage_index, setup[0], setup[1], setup[2]],
                );
                let stage = e.mem.u32(local);
                e.call(SET_FILTER_MODE, &args![stage, 1u32]);
                let pass = e.mem.u32(slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            }
            water_state(e, slot, 0xf, 1);
            water_state(e, slot, 0x18, 0);
            water_state(e, slot, 0x19, 5);
            if index != 0x21 {
                if matches!(index, 0x24 | 0x48 | 0x25 | 0x49) {
                    water_state(e, slot, 0x1b, 1);
                    let pass = e.mem.u32(slot);
                    pass_render_state(e, pass, 0x13, 5);
                    pass_render_state(e, pass, 0x14, 2);
                } else {
                    water_state(e, slot, 0x1b, 1);
                    let pass = e.mem.u32(slot);
                    e.call(SET_BLEND_00B80040, &args![pass, 5u32, 6u32, 0u32]);
                }
            }
            if (0x11..=0x1c).contains(&index) || index.wrapping_sub(0x37) <= 0xb {
                water_state(e, slot, 0x34, 1);
            } else if (0..=0x1c).contains(&index)
                || (0x26..=0x42).contains(&index)
                || index == 0x24
                || index == 0x48
            {
                water_state(e, slot, 0x34, 1);
                water_state(e, slot, 0x38, 6);
                water_state(e, slot, 0x39, 1);
                water_state(e, slot, 0x3a, 1);
                water_state(e, slot, 0x37, 1);
            }
            if index == 0x23 || index == 0x47 {
                for (state, value) in [
                    (0xa8u32, 0u32),
                    (0xe, 0),
                    (0x34, 1),
                    (0x38, 3),
                    (0x39, 0xff),
                    (0x37, 3),
                    (0x3a, 0),
                ] {
                    water_state(e, slot, state, value);
                }
            }
            let stage = e.mem.u32(local);
            stage_release(e, stage);
        }
    });
    for (pass_offset, vertex, pixel, word, value, assign) in WATER_ASSIGNMENTS {
        let slot = base + pass_offset;
        match assign {
            WaterAssign::Inline => {
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + vertex);
                ni_pointer_assign(e, pass + 0x5c, shader);
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + pixel);
                ni_pointer_assign(e, pass + 0x44, shader);
            }
            WaterAssign::InlineVertex => {
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + vertex);
                ni_pointer_assign(e, pass + 0x5c, shader);
                let shader = e.mem.u32(base + pixel);
                let pass = e.mem.u32(slot);
                e.call(SET_PIXEL_SHADER_00B80600, &args![pass, shader]);
            }
            WaterAssign::Helpers => {
                let shader = e.mem.u32(base + vertex);
                let pass = e.mem.u32(slot);
                e.call(SET_VERTEX_SHADER_00B79950, &args![pass, shader]);
                let shader = e.mem.u32(base + pixel);
                let pass = e.mem.u32(slot);
                e.call(SET_PIXEL_SHADER_00B80600, &args![pass, shader]);
            }
        }
        e.set_global::<u32>(word, value);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_3dd0, fn_00b83dd0(Ptr) -> bool),
        entry!(0x00b8_27c0, water_shader_load_shaders(Ptr)),
        entry!(0x00b8_6bd0, water_shader_water_shader(Ptr) -> Ptr),
        entry!(0x00b8_6de0, water_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn water_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, WATER_METHOD_00B83DD0]);
        let (shader, log) = run(&mut e, 0x00b8_6de0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 4, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x250]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (WATER_METHOD_00B83DD0, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_WATER);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.mem.write(0x011f_f100, &[0xee; 0x20]);
        e.mem.write(FLOATS_011FF458, &[0xee; 0x10]);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x250);
        e.mem.write(this, &[0xcc; 0x250]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        (e, this)
    }

    #[test]
    fn water_shader_constructor_builds_the_shader_and_resets_the_globals() {
        let (e, this) = construct(2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_VTABLE_010AF9E0);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x4a, DESTRUCT_00B79930),
                pointer_array(this + 0x1a0, 0x26, DESTRUCT_00B66F20),
            ]
        );
        for offset in [
            0x194u32, 0x198, 0x19c, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0xd);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.global::<u32>(WATER_SHADER_011FF370), this);
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(FLOATS_011FF458 + 4 * index), 0);
        }
        for byte in [0x011f_f100u32, 0x011f_f102, 0x011f_f103, 0x011f_f104] {
            assert_eq!(e.global::<u8>(byte), 0, "{byte:#x}");
        }
        assert_eq!(e.global::<u8>(0x011f_f101), 0xee);
        assert_eq!(e.global::<u16>(0x011f_f118), 0);
        assert_eq!(e.global::<u8>(0x011f_f11a), 0);
        assert_eq!(e.global::<u32>(0x011f_f110), 0);
        assert_eq!(e.global::<u16>(0x011f_f114), 0);
        assert_eq!(e.global::<u8>(0x011f_f116), 0xee);
    }

    #[test]
    fn water_shader_constructor_keeps_byte_011ff100_above_model_2() {
        let (e, _) = construct(3);
        assert_eq!(e.global::<u8>(0x011f_f100), 0xee);
        assert_eq!(e.global::<u8>(0x011f_f102), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load(model: i32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 0x33);
        e.register(FORMAT_00406D00, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            if a[1] == 0x100 {
                // The file buffers keep the source string address for the test.
                e.mem.set_u32(a[0] + 4, 0x5eed);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x240);
        e.call(LOAD_SHADERS, &args![this]);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0x33);
        (e, this)
    }

    #[test]
    fn water_shader_load_shaders_compiles_three_vertex_and_38_pixel_shaders() {
        let (e, this) = load(5);
        let expected_vertex = [
            (0x194u32, 0x010a_fc10u32, macro_words(&[])),
            (
                0x198,
                0x010a_fbf0,
                macro_words(&[(W_MACRO_VERTEX_COLORS_010AFC00, W_EMPTY)]),
            ),
            (
                0x19c,
                0x010a_fbd4,
                macro_words(&[(W_MACRO_LIGHTING_010AFBE4, W_EMPTY)]),
            ),
        ];
        for (slot, output, macros) in expected_vertex {
            let seen = compiled(&e, e.mem.u32(this + slot));
            assert_eq!(
                seen,
                Compiled {
                    file: W_VERTEX_SOURCE_010AFC20,
                    output,
                    profile: W_PROFILE_VS_1_1_010A93EC,
                    pixel: false,
                    macros
                }
            );
        }
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats[3][1..], [0x100, W_PIXEL_SOURCE_2_AB_010AFBB8]);
        assert_eq!(formats[4][1..], [0x100, W_LIGHTING_SOURCE_010AFB78]);
        let pixel_names: Vec<&Vec<u32>> = formats.iter().skip(5).collect();
        assert_eq!(pixel_names.len(), 38);
        for (index, format) in pixel_names.iter().enumerate() {
            assert_eq!(format[2..], [W_PIXEL_NAME_FORMAT_010AFB1C, index as u32]);
        }
        let first = compiled(&e, e.mem.u32(this + 0x1a0));
        assert_eq!(first.profile, W_PROFILE_PS_2_A_010A93C4);
        assert!(first.pixel);
        assert_eq!(
            first.macros,
            [
                W_WATER,
                W_EMPTY,
                W_REFLECTIONS,
                W_EMPTY,
                W_REFRACTIONS,
                W_EMPTY,
                W_DEPTH,
                W_EMPTY
            ]
        );
        // MakePath got the stack buffer holding the pixel file, then the lighting file for the
        // last two blocks.
        let paths = calls(&e, MAKE_PATH);
        assert_eq!(paths.len(), 41);
        let water_buffer = paths[3][0];
        let lighting_buffer = paths[40][0];
        assert_ne!(water_buffer, lighting_buffer);
        assert!(paths[3..39].iter().all(|a| a[0] == water_buffer));
        assert_eq!(paths[39][0], lighting_buffer);
        for index in 0..38u32 {
            assert_ne!(e.mem.u32(this + 0x1a0 + 4 * index), 0);
        }
        let last = compiled(&e, e.mem.u32(this + 0x1a0 + 4 * 37));
        assert_eq!(last.macros[..2], [W_WADING, W_EMPTY]);
    }

    #[test]
    fn water_shader_load_shaders_profile_and_file_follow_the_shader_model() {
        for (model, profile, source) in [
            (7, W_PROFILE_PS_3_0_010A93B4, W_PIXEL_SOURCE_2_AB_010AFBB8),
            (6, W_PROFILE_PS_2_B_010A93BC, W_PIXEL_SOURCE_2_AB_010AFBB8),
            (4, W_PROFILE_PS_2_0_010A93CC, W_PIXEL_SOURCE_2_0_010AFB9C),
            (1, W_PROFILE_PS_2_0_010A93CC, W_PIXEL_SOURCE_2_0_010AFB9C),
        ] {
            let (e, this) = load(model);
            assert_eq!(compiled(&e, e.mem.u32(this + 0x1a0)).profile, profile);
            assert_eq!(calls(&e, FORMAT_00406D00)[3][2], source);
        }
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn run(existing: Option<u32>) -> (Engine, u32) {
        let mut e = pass_rig();
        e.register(SET_BLEND_00B80040, |_, _| Ret::default());
        e.register(SET_VERTEX_SHADER_00B79950, |_, _| Ret::default());
        e.register(SET_PIXEL_SHADER_00B80600, |_, _| Ret::default());
        e.map(0x011f_f000, 0x1000);
        let this = e.mem.alloc(0x250);
        for offset in (0x194u32..=0x234).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        if let Some(offset) = existing {
            let pass = e.mem.alloc(0x70);
            e.mem.set_u32(this + offset, pass);
        }
        assert!(e.call(WATER_METHOD_00B83DD0, &args![this]).bool());
        (e, this)
    }

    #[test]
    fn fn_00b83dd0_builds_seventy_four_passes() {
        let (e, this) = run(None);
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 0x4a);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 5 * 0x4a);
        let setups: Vec<Vec<u32>> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .take(5)
            .map(|a| a[1..].to_vec())
            .collect();
        assert_eq!(
            setups,
            vec![
                vec![0, 3, 2, 1],
                vec![1, 3, 2, 1],
                vec![2, 1, 2, 1],
                vec![3, 1, 2, 1],
                vec![4, 3, 2, 1]
            ]
        );
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 521);
        assert_eq!(calls(&e, SET_BLEND_00B80040).len(), 69);
        let pass = |index: u32| e.mem.u32(this + 0x6c + 4 * index);
        assert_eq!(
            render_states(&e, pass(0)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x34, 1),
                (0x38, 6),
                (0x39, 1),
                (0x3a, 1),
                (0x37, 1)
            ]
        );
        assert_eq!(
            render_states(&e, pass(0x11)),
            vec![(0xf, 1), (0x18, 0), (0x19, 5), (0x1b, 1), (0x34, 1)]
        );
        assert_eq!(
            render_states(&e, pass(0x21)),
            vec![(0xf, 1), (0x18, 0), (0x19, 5)]
        );
        assert_eq!(
            render_states(&e, pass(0x23)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0xa8, 0),
                (0xe, 0),
                (0x34, 1),
                (0x38, 3),
                (0x39, 0xff),
                (0x37, 3),
                (0x3a, 0)
            ]
        );
        assert_eq!(
            render_states(&e, pass(0x24)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x13, 5),
                (0x14, 2),
                (0x34, 1),
                (0x38, 6),
                (0x39, 1),
                (0x3a, 1),
                (0x37, 1)
            ]
        );
        assert_eq!(calls(&e, SET_BLEND_00B80040)[0], vec![pass(0), 5, 6, 0]);
        // The second part: inline for the first passes, the setters for the last ones.
        assert_eq!(e.mem.u32(pass(0) + 0x5c), e.mem.u32(this + 0x194));
        assert_eq!(e.mem.u32(pass(0) + 0x44), e.mem.u32(this + 0x1a0));
        assert_eq!(e.global::<u32>(0x011f_f120), 0x3e);
        assert_eq!(e.global::<u32>(0x011f_f190), 0xe);
        assert_eq!(e.global::<u32>(0x011f_f194), 0);
        assert_eq!(e.global::<u32>(0x011f_f244), 0x4e);
        assert_eq!(calls(&e, SET_PIXEL_SHADER_00B80600).len(), 24);
        assert_eq!(calls(&e, SET_VERTEX_SHADER_00B79950).len(), 23);
        assert_eq!(
            calls(&e, SET_PIXEL_SHADER_00B80600)[0],
            vec![e.mem.u32(this + 0x134), e.mem.u32(this + 0x1d0)]
        );
        assert_eq!(
            e.mem.u32(e.mem.u32(this + 0x134) + 0x5c),
            e.mem.u32(this + 0x198)
        );
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn fn_00b83dd0_keeps_an_existing_pass() {
        let (e, this) = run(Some(0x70));
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 0x49);
        let kept = e.mem.u32(this + 0x70);
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != kept));
        assert_eq!(e.mem.u32(kept + 0x5c), e.mem.u32(this + 0x194));
        assert_eq!(e.mem.u32(kept + 0x44), e.mem.u32(this + 0x1a4));
    }
}
