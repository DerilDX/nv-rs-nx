//! `BSShader/speedtreeleafshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bb2a00` and the method `00bb1d40` called directly (both of
/// this unit in the engine map, unnamed, `thiscall`, no words).
const SHADER_CONSTRUCT_00BB2A00: u32 = 0x00bb_2a00;
const SHADER_METHOD_00BB1D40: u32 = 0x00bb_1d40;

// Translated from 00bb2c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 6. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 4 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 7, 5, 1, 0)), calls slot +0x94 with (0, 3, 8, 0x80000000, 3, 2, 0, 0) and
/// slot +0xb4. Then allocates 0x38c bytes and runs the constructor `00bb2a00` (a null
/// allocation is used as it is), calls the shader's slots +0x4c and +0x13c, the method
/// `00bb1d40`, slot +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bb2c00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bb2c00(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 3, 3, 2, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 3u32, 8u32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x38cu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BB2A00, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.vcall(shader, 0x13c, &args![]);
    e.call(SHADER_METHOD_00BB1D40, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The leaf shader's vtable (`010b9190`; no RTTI locator before it).
const LEAF_SHADER_VTABLE_010B9190: u32 = 0x010b_9190;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// The sixteen `float`s the constructor sets (`01200638`..`01200674`): eight 1.0, eight 0.0.
const FLOATS_01200638: u32 = 0x0120_0638;

// Translated from 00bb2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the leaf shader's constructor
/// (the creator `00bb2c00` runs it), `thiscall`, no stack words. Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010b9190`, constructs the `NiPointer` arrays at +0x36c
/// (4) and +0x37c (2, both destructor `00b66f20`) and +0x384 (2, destructor `00b79930`), sets
/// +0x68 to 9, clears 0x300 bytes from +0x6c (`memset`), and stores 1.0 in the eight `float`s
/// from `01200638` and 0.0 in the eight from `01200658`. Answers `this`.
// Translation incomplete: 00bb2a00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bb2a00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, LEAF_SHADER_VTABLE_010B9190);
    construct_pointer_array(e, base + 0x36c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x37c, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x384, 2, DESTRUCT_00B79930);
    e.mem.set_u32(base + 0x68, 9);
    e.call(MEMSET, &args![base + 0x6c, 0u32, 0x300u32]);
    for index in 0..16u32 {
        let value = if index < 8 { 1.0f32 } else { 0.0 };
        e.set_global::<f32>(FLOATS_01200638 + 4 * index, value);
    }
    this
}

/// The render-state block release `00e7f6b0` (unnamed, `cdecl`, the block).
const RELEASE_RENDER_STATES_00E7F6B0: u32 = 0x00e7_f6b0;

/// A new render-state block for the pass (`00e7f5d0`), the old one at +0x30 released through
/// `00e7f6b0` when it differs.
fn renew_render_states(e: &mut Engine, pass: u32) {
    let block = e.call(NEW_RENDER_STATES_00E7F5D0, &args![]).u32();
    let old = e.mem.u32(pass + 0x30);
    if old != block {
        e.call(RELEASE_RENDER_STATES_00E7F6B0, &args![old]);
    }
    e.mem.set_u32(pass + 0x30, block);
}

// Translated from 00bb1d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the leaf shader's stages and
/// passes (the creator `00bb2c00` calls it), `thiscall`, no stack words. For the pass pointers
/// at +0x384 and +0x388 in turn: a new pass (`00e804d0`), a stage made into the same stack
/// pointer (`CreateNewStage`; the first stage is not dropped before the second is made),
/// `SetupDefaultStage` index 0 with 1, 2, 1 and no filter mode, `SetStage` with the pass's
/// word at +0x14, a new render-state block (the old one released when it differs), the
/// vertex shader (+0x370 for the first pass, +0x378 for the second) assigned to the pass's
/// +0x5c and the pixel shader +0x380 to its +0x44. Finally the stage reference is dropped.
// Translation incomplete: 00bb1d40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bb1d40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (pass_offset, vertex) in [(0x384u32, 0x370u32), (0x388, 0x378)] {
            let slot = base + pass_offset;
            e.call(NEW_PASS_00E804D0, &args![slot]);
            e.call(CREATE_NEW_STAGE, &args![local]);
            let stage = e.mem.u32(local);
            e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 1u32, 2u32, 1u32]);
            let pass = e.mem.u32(slot);
            let stage = e.mem.u32(local);
            let position = e.mem.u32(pass + 0x14);
            e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            let pass = e.mem.u32(slot);
            renew_render_states(e, pass);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + 0x380);
            ni_pointer_assign(e, pass + 0x44, shader);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
}

/// The `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor), `BSShader::~BSShader` (engine map, `00be0f10`, no stack words) and the Ni free
/// (`00aa1460`, `cdecl`: block, size).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bb1c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the leaf shader's destructor,
/// no stack words. Stores the vtable `010b9190`, clears (releasing what they hold) the
/// `NiPointer`s +0x36c to +0x378 and +0x37c, +0x380, destroys the arrays +0x384 (2, destructor
/// `00b79930`), +0x37c (2) and +0x36c (4) (destructor `00b66f20`) and runs `BSShader::~BSShader`
/// (`00be0f10`).
// Translation incomplete: 00bb1c10 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the arrays and the base) is not translated.
pub fn fn_00bb1c10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEAF_SHADER_VTABLE_010B9190);
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x36c + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x37c + 4 * index);
    }
    for (offset, count, destructor) in [
        (0x384u32, 2u32, DESTRUCT_00B79930),
        (0x37c, 2, DESTRUCT_00B66F20),
        (0x36c, 4, DESTRUCT_00B66F20),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The renderer's `00e6fbb0` (`thiscall` on the shader's +0x18, `RET 8`), `D3DXMatrixInverse`
/// (engine map, `00ee6dda`, `stdcall`: out, determinant, matrix), the light setup `00bb1960`
/// (unit `run_00bb04b0`, `thiscall`, `RET 0xc`) and the camera rows `00bb1a90` (no arguments).
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
const LIGHT_SETUP_00BB1960: u32 = 0x00bb_1960;
const CAMERA_ROWS_00BB1A90: u32 = 0x00bb_1a90;
/// Bits of `1.0f`.
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00bb25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b9190` slot +0x60): the leaf shader's `SetupGeometry`,
/// `thiscall`, `RET 0x20` (eight words: the geometry, three words, the property state, a word,
/// the world transform and the bound; only the state and the transform are read). Runs the
/// renderer's `00e6fbb0` (+0x18) with the transform and 0, builds on the stack the D3DX matrix
/// of the transform (rotation transposed and times the scale, translation, 1.0) and inverts it
/// (`D3DXMatrixInverse`); then `00bb1960` with the state's +0xc object when its word +0x1c is
/// 6 (otherwise null), the inverse and the transform's scale, and `00bb1a90`. Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bb25d0(
    e: &mut Engine,
    this: Ptr,
    _geometry: u32,
    _word_2: u32,
    _word_3: u32,
    _word_4: u32,
    state: u32,
    _word_6: u32,
    transform: u32,
    _bound: u32,
) -> u32 {
    let this = this.addr();
    let renderer = e.mem.u32(this + 0x18);
    e.call(RENDERER_CALL_00E6FBB0, &args![renderer, transform, 0u32]);
    e.with_stack(0x88, |e, frame| {
        let frame = frame.addr();
        let matrix = frame + 8;
        let inverse = frame + 0x48;
        let scale_bits = e.mem.u32(transform + 0x30);
        let first = x87_mul(x87_load32(scale_bits), x87_m32(e.mem.u32(transform)));
        e.mem.set_u32(matrix, x87_store32(first));
        for (row, place) in [
            (3u32, 0xcu32),
            (6, 0x10),
            (1, 0x18),
            (4, 0x1c),
            (7, 0x20),
            (2, 0x28),
            (5, 0x2c),
            (8, 0x30),
        ] {
            let value = x87_load32(e.mem.u32(transform + 4 * row));
            let product = x87_store32(x87_mul(value, x87_m32(scale_bits)));
            e.mem.set_u32(frame + place, product);
        }
        for (index, place) in [0x38u32, 0x3c, 0x40].iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(transform + 0x24 + 4 * index as u32));
            e.mem.set_u32(frame + place, value);
        }
        for place in [0x14u32, 0x24, 0x34] {
            e.mem.set_u32(frame + place, 0);
        }
        e.mem.set_u32(frame + 0x44, ONE_BITS);
        e.call(D3DX_MATRIX_INVERSE, &args![inverse, 0u32, matrix]);
        let candidate = e.mem.u32(state + 0xc);
        let property = if candidate != 0 && e.mem.u32(candidate + 0x1c) == 6 {
            candidate
        } else {
            0
        };
        let scale = x87_quiet_bits(e.mem.u32(transform + 0x30));
        e.call(LIGHT_SETUP_00BB1960, &args![this, property, inverse, scale]);
    });
    e.call(CAMERA_ROWS_00BB1A90, &args![]);
    0
}

/// The shader manager's pass state (`0126f74c`, its +0x24 the table of constant blocks).
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;

// Translated from 00bb26d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`; vtable `010b9190` slot +0x78),
/// `thiscall`, `RET 4` (one word, an object whose +0xc it reads). Holds the pass state's
/// (`0126f74c`) first constant block (its count +0x60 incremented when it is not null), stores
/// in its +8 the answer of the vtable slot +0xb4 of that +0xc object, runs the shader's slot
/// +0x108, then drops the block (count down by one, `00e7ecd0` at 0; both without a null
/// check, as the code has it).
// Translation incomplete: 00bb26d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held block) is not translated.
pub fn fn_00bb26d0(e: &mut Engine, this: Ptr, source: Ptr) {
    let this = this.addr();
    let state = e.global::<u32>(PASS_STATE_0126F74C);
    let object = e.mem.u32(source.addr() + 0xc);
    let table = e.mem.u32(state + 0x24);
    let block = e.mem.u32(table);
    if block != 0 {
        let count = e.mem.u32(block + 0x60).wrapping_add(1);
        e.mem.set_u32(block + 0x60, count);
    }
    let value = e.vcall(object, 0xb4, &args![]).u32();
    e.mem.set_u32(block + 8, value);
    e.vcall(this, 0x108, &args![]);
    let left = e.mem.u32(block + 0x60).wrapping_sub(1);
    e.mem.set_u32(block + 0x60, left);
    if left == 0 {
        e.call(STAGE_DELETE_00E7ECD0, &args![block]);
    }
}

/// The current pass record (`011f91e0`; its +0 the geometry, byte +9 a light count, +0xc the
/// lights), the default bound (`011f4288`), the light call `00b71360` (unnamed, `cdecl`: the
/// index, the light, a `float`), the byte `011f91a7`, the word `011f91b8`, the `float` at
/// `010231a8` and the colour, wind and leaf constants written.
const CURRENT_RECORD_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const LIGHT_CALL_00B71360: u32 = 0x00b7_1360;
const FULL_BRIGHT_011F91A7: u32 = 0x011f_91a7;
const MODE_011F91B8: u32 = 0x011f_91b8;
const DIFFUSE_VALUE_010231A8: u32 = 0x0102_31a8;
const AMBIENT_011FA0C0: u32 = 0x011f_a0c0;
const DIFFUSE_011FA0D0: u32 = 0x011f_a0d0;
const SCALE_01200608: u32 = 0x0120_0608;
const SCALE_0120060C: u32 = 0x0120_060c;
const LEAF_0120065C: u32 = 0x0120_065c;
const LEAF_0120066C: u32 = 0x0120_066c;
const LEAF_01200660: u32 = 0x0120_0660;
const LEAF_01200670: u32 = 0x0120_0670;
const TREE_DATA_01200678: u32 = 0x0120_0678;
/// CRT `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00bb2760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`; vtable `010b9190` slot +0x7c),
/// `thiscall`, `RET 4` (the property state). With the current record's (`011f91e0`)
/// geometry and the state's +0xc object: runs `00b71360` for each of the record's lights
/// (byte count +9, array +0xc) with the index, the light and the object's `float` +0x6c; runs
/// the shader's slot +0x60 with the geometry, 0, 0, the word +0x34 of its +0xb8, the state, 0,
/// its transform (+0x68) and its bound (+0x20, or `011f4288`); with the byte `011f91a7` set,
/// stores 1.0 in the four floats at `011fa0c0` and in `011fa0d0` the `float` `010231a8`
/// followed by three 0.0 when the word `011f91b8` is 9, four 0.0 otherwise; copies the object's
/// slot +0x100 answer (0x300 bytes) to the shader's +0x6c; from the object's slot +0xfc answer
/// (asked again each time) its +0x18 times `01200608` into `0120065c`, its +0x1c times
/// `0120060c` into `0120066c`, its +0x14 into `01200660`, its +0x10 into `01200670`, four 0.0
/// at `01200678` and then its +0xc into `0120067c`. Last, the slot +0x78 of the shader's +0x34
/// and +0x30 objects with the pass state's (`0126f74c`) +0x5c and +0x44 objects, the geometry,
/// its word +0xbc, 0, the +0xb8 word, the state, 0, the transform, the bound, 0 and 1.
pub fn fn_00bb2760(e: &mut Engine, this: Ptr, state: Ptr) {
    let this = this.addr();
    let state = state.addr();
    let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
    let geometry = e.mem.u32(record);
    let mut bound = e.mem.u32(geometry + 0x20);
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    let passes = e.mem.u32(e.mem.u32(geometry + 0xb8) + 0x34);
    let word_bc = e.mem.u32(geometry + 0xbc);
    let object = e.mem.u32(state + 0xc);
    let light_value = x87_quiet_bits(e.mem.u32(object + 0x6c));
    let mut index = 0u8;
    while index < e.mem.u8(record + 9) {
        let lights = e.mem.u32(record + 0xc);
        let light = e.mem.u32(lights + 4 * u32::from(index));
        e.call(
            LIGHT_CALL_00B71360,
            &args![u32::from(index), light, light_value],
        );
        index = index.wrapping_add(1);
    }
    e.vcall(
        this,
        0x60,
        &args![
            geometry,
            0u32,
            0u32,
            passes,
            state,
            0u32,
            geometry + 0x68,
            bound
        ],
    );
    if e.global::<u8>(FULL_BRIGHT_011F91A7) != 0 {
        let nine = e.global::<u32>(MODE_011F91B8) == 9;
        for word in 0..4u32 {
            e.set_global::<u32>(AMBIENT_011FA0C0 + 4 * word, ONE_BITS);
        }
        if nine {
            let value = x87_quiet_bits(e.global::<u32>(DIFFUSE_VALUE_010231A8));
            e.set_global::<u32>(DIFFUSE_011FA0D0, 0);
            e.set_global::<u32>(DIFFUSE_011FA0D0 + 4, value);
            e.set_global::<u32>(DIFFUSE_011FA0D0 + 8, 0);
            e.set_global::<u32>(DIFFUSE_011FA0D0 + 0xc, 0);
        } else {
            for word in 0..4u32 {
                e.set_global::<u32>(DIFFUSE_011FA0D0 + 4 * word, 0);
            }
        }
    }
    let leaves = e.vcall(object, 0x100, &args![]).u32();
    e.call(MEMCPY, &args![this + 0x6c, leaves, 0x300u32]);
    let data = e.vcall(object, 0xfc, &args![]).u32();
    let value = x87_mul(
        x87_load32(e.mem.u32(data + 0x18)),
        x87_m32(e.global::<u32>(SCALE_01200608)),
    );
    e.set_global::<u32>(LEAF_0120065C, x87_store32(value));
    let data = e.vcall(object, 0xfc, &args![]).u32();
    let value = x87_mul(
        x87_load32(e.mem.u32(data + 0x1c)),
        x87_m32(e.global::<u32>(SCALE_0120060C)),
    );
    e.set_global::<u32>(LEAF_0120066C, x87_store32(value));
    let data = e.vcall(object, 0xfc, &args![]).u32();
    let value = x87_quiet_bits(e.mem.u32(data + 0x14));
    e.set_global::<u32>(LEAF_01200660, value);
    let data = e.vcall(object, 0xfc, &args![]).u32();
    let value = x87_quiet_bits(e.mem.u32(data + 0x10));
    e.set_global::<u32>(LEAF_01200670, value);
    for word in 0..4u32 {
        e.set_global::<u32>(TREE_DATA_01200678 + 4 * word, 0);
    }
    let data = e.vcall(object, 0xfc, &args![]).u32();
    let value = x87_quiet_bits(e.mem.u32(data + 0xc));
    e.set_global::<u32>(TREE_DATA_01200678 + 4, value);
    for (slot, entry) in [(0x34u32, 0x5cu32), (0x30, 0x44)] {
        let target = e.mem.u32(this + slot);
        if target == 0 {
            continue;
        }
        let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
        let constants = e.mem.u32(pass_state + entry);
        if constants == 0 {
            continue;
        }
        e.vcall(
            target,
            0x78,
            &args![
                constants,
                geometry,
                word_bc,
                0u32,
                passes,
                state,
                0u32,
                geometry + 0x68,
                bound,
                0u32,
                1u32
            ],
        );
    }
}

// Translated from 00bb2bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `speedtreeleafshader.cpp`): the leaf shader's scalar
/// deleting destructor (vtable `010b9190` slot +0), `thiscall`, `RET 4`. Runs the destructor
/// (`00bb1c10`) and, when bit 0 of the flags is set, frees the 0x38c bytes (`00aa1460`).
/// Answers `this`.
pub fn fn_00bb2bb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb1c10(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x38cu32]);
    }
    this
}

/// The constant names of `CreateConstantMaps` (read in Ghidra) and the data they point at.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const AMBIENT_COLOR_010AF568: u32 = 0x010a_f568;
const DIFF_COLOR_0_010B92F8: u32 = 0x010b_92f8;
const DIFF_COLOR_1_010B92E8: u32 = 0x010b_92e8;
const FOG_PARAM_010AF60C: u32 = 0x010a_f60c;
const FOG_COLOR_010AF5E4: u32 = 0x010a_f5e4;
const TREE_DIMMER_010B92DC: u32 = 0x010b_92dc;
const GROUPED_CONSTANTS_010B8C34: u32 = 0x010b_8c34;
const LEAF_DATA_010B92D0: u32 = 0x010b_92d0;
const DIFFUSE_1_011FA0E0: u32 = 0x011f_a0e0;
const FOG_PARAM_011FA280: u32 = 0x011f_a280;
const FOG_COLOR_011FA290: u32 = 0x011f_a290;
const TREE_DIMMER_011F918C: u32 = 0x011f_918c;
const GROUPED_01200618: u32 = 0x0120_0618;
const DIMMER_SWITCH_011F941E: u32 = 0x011f_941e;
/// `SpeedTreeShader::SetShaderConstants` (Xbox PDB, `00bb2f50`, `cdecl`: the map, the first
/// register).
const SET_SHADER_CONSTANTS_00BB2F50: u32 = 0x00bb_2f50;

/// One `0x10000007` entry: one register of 0x10 bytes at `data`.
fn vector_entry(name: u32, register: u32, data: u32) -> [u32; 10] {
    [
        name,
        0x1000_0007,
        0,
        register,
        1,
        CONSTANT_EMPTY_NAME_01011584,
        0x10,
        4,
        data,
        0,
    ]
}

// Translated from 00bb2d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpeedTreeLeafShader::CreateConstantMaps` (Xbox PDB), no stack words. When the vertex
/// constant map (+0x34) is null: makes one (kind 0) and adds `"WorldViewProjTranspose"`
/// (0x20000009, 0, 0, 4), `"Ambient Color"` (`011fa0c0`, register 5), `"Diff Color 0"`
/// (`011fa0d0`, 6), `"Diff Color 1"` (`011fa0e0`, 7), `"FogParam | ShadowVolumeFatness"`
/// (`011fa280`, 8), `"FogColor | ShadowVolumeExtrudeDistance"` (`011fa290`, 9), with the byte
/// `011f941e` set `"Tree Dimmer"` (0x10000004, register 0xa, 4 bytes at `011f918c`),
/// `"grouped constants"` (`01200618`, register 0xb, 6 registers, 0x60 bytes); runs
/// `SpeedTreeShader::SetShaderConstants` with the map and 0x11; adds `"leaf data"` (+0x6c,
/// register 0x22, 0x30 registers, 0x300 bytes).
// Translation incomplete: 00bb2d10 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn speed_tree_leaf_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) != 0 {
        return;
    }
    make_constant_map(e, base, 0x34, 0);
    let empty = CONSTANT_EMPTY_NAME_01011584;
    let entries = [
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
        vector_entry(AMBIENT_COLOR_010AF568, 5, AMBIENT_011FA0C0),
        vector_entry(DIFF_COLOR_0_010B92F8, 6, DIFFUSE_011FA0D0),
        vector_entry(DIFF_COLOR_1_010B92E8, 7, DIFFUSE_1_011FA0E0),
        vector_entry(FOG_PARAM_010AF60C, 8, FOG_PARAM_011FA280),
        vector_entry(FOG_COLOR_010AF5E4, 9, FOG_COLOR_011FA290),
    ];
    for words in entries {
        add_constant_entry(e, base, 0x34, words);
    }
    if e.global::<u8>(DIMMER_SWITCH_011F941E) != 0 {
        add_constant_entry(
            e,
            base,
            0x34,
            [
                TREE_DIMMER_010B92DC,
                0x1000_0004,
                0,
                0xa,
                1,
                empty,
                4,
                4,
                TREE_DIMMER_011F918C,
                0,
            ],
        );
    }
    add_constant_entry(
        e,
        base,
        0x34,
        [
            GROUPED_CONSTANTS_010B8C34,
            0x1000_0009,
            0,
            0xb,
            6,
            empty,
            0x60,
            4,
            GROUPED_01200618,
            0,
        ],
    );
    let map = e.mem.u32(base + 0x34);
    e.call(SET_SHADER_CONSTANTS_00BB2F50, &args![map, 0x11u32]);
    add_constant_entry(
        e,
        base,
        0x34,
        [
            LEAF_DATA_010B92D0,
            0x1000_0009,
            0,
            0x22,
            0x30,
            empty,
            0x300,
            4,
            base + 0x6c,
            0,
        ],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_1d40, fn_00bb1d40(Ptr)),
        entry!(0x00bb_2a00, fn_00bb2a00(Ptr) -> Ptr),
        entry!(0x00bb_2c00, fn_00bb2c00() -> Ptr),
        entry!(0x00bb_1c10, fn_00bb1c10(Ptr)),
        entry!(
            0x00bb_25d0,
            fn_00bb25d0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_26d0, fn_00bb26d0(Ptr, Ptr)),
        entry!(0x00bb_2760, fn_00bb2760(Ptr, Ptr)),
        entry!(0x00bb_2bb0, fn_00bb2bb0(Ptr, u32) -> Ptr),
        entry!(
            0x00bb_2d10,
            speed_tree_leaf_shader_create_constant_maps(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bb2c00_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BB2A00, &[SHADER_METHOD_00BB1D40]);
        let (shader, log) = run(&mut e, 0x00bb_2c00);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 3, 8, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x38c]),
                (SHADER_CONSTRUCT_00BB2A00, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (SHADER_METHOD_00BB1D40, vec![SHADER]),
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
    fn fn_00bb2a00_builds_the_leaf_shader() {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.map(0x0120_0000, 0x1000);
        e.mem.write(FLOATS_01200638, &[0xff; 0x40]);
        let this = e.mem.alloc(0x390);
        assert_eq!(e.call(0x00bb_2a00, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), LEAF_SHADER_VTABLE_010B9190);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x36c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x37c, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x384, 2, DESTRUCT_00B79930),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 9);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x6c, 0, 0x300]]);
        for index in 0..8u32 {
            assert_eq!(e.global::<u32>(FLOATS_01200638 + 4 * index), 0x3f80_0000);
            assert_eq!(e.global::<u32>(FLOATS_01200638 + 0x20 + 4 * index), 0);
        }
    }
}

#[cfg(test)]
mod stages_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bb1d40_builds_two_passes() {
        let mut e = pass_rig();
        e.register(RELEASE_RENDER_STATES_00E7F6B0, |_, _| Ret::default());
        let this = e.mem.alloc(0x390);
        let shaders: Vec<u32> = (0..3).map(|_| object(&mut e, 1)).collect();
        e.mem.set_u32(this + 0x370, shaders[0]);
        e.mem.set_u32(this + 0x378, shaders[1]);
        e.mem.set_u32(this + 0x380, shaders[2]);
        e.call(SHADER_METHOD_00BB1D40, &args![this]);
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x384], vec![this + 0x388]]
        );
        let first = e.mem.u32(this + 0x384);
        let second = e.mem.u32(this + 0x388);
        let setups = calls(&e, SETUP_DEFAULT_STAGE);
        assert_eq!(setups.len(), 2);
        for setup in &setups {
            assert_eq!(setup[1..], [0, 1, 2, 1]);
        }
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        for pass in [first, second] {
            assert_ne!(e.mem.u32(pass + 0x30), 0);
        }
        // The pass rig's passes start without a block: the null one is released.
        assert_eq!(
            calls(&e, RELEASE_RENDER_STATES_00E7F6B0),
            vec![vec![0], vec![0]]
        );
        assert_eq!(e.mem.u32(first + 0x5c), shaders[0]);
        assert_eq!(e.mem.u32(second + 0x5c), shaders[1]);
        assert_eq!(e.mem.u32(first + 0x44), shaders[2]);
        assert_eq!(e.mem.u32(second + 0x44), shaders[2]);
        assert_eq!(e.mem.u32(shaders[2] + 4), 3);
        // Only the second stage is dropped: the first keeps its own count and its pass's.
        let set = calls(&e, PASS_SET_STAGE);
        assert_eq!(e.mem.u32(set[0][2] + 0x60), 2);
        assert_eq!(e.mem.u32(set[1][2] + 0x60), 1);
    }
}

#[cfg(test)]
mod leaf_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const SLOTS: u32 = 0x0e00_0000;
    const VTABLE: u32 = 0x0e80_0000;

    fn vtable(e: &mut Engine, n: u32) -> u32 {
        let table = VTABLE + n * 0x1000;
        let targets: Vec<u32> = (0..0x60u32).map(|i| SLOTS + n * 0x1000 + 4 * i).collect();
        e.put_vtable(table, &targets);
        table
    }

    fn slot(n: u32, offset: u32) -> u32 {
        SLOTS + n * 0x1000 + offset
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn map_globals(e: &mut Engine) {
        for page in [
            0x0102_3000u32,
            0x011f_4000,
            0x011f_9000,
            0x011f_a000,
            0x0120_0000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
    }

    #[test]
    fn fn_00bb1c10_releases_and_destroys() {
        let mut e = rig();
        noop(&mut e, &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0x38c);
        let shader = object(&mut e, 1);
        e.mem.set_u32(this + 0x380, shader);
        e.call(0x00bb_1c10, &args![this]);
        assert_eq!(e.mem.u32(this), LEAF_SHADER_VTABLE_010B9190);
        assert_eq!(e.mem.u32(this + 0x380), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shader]]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x384, 4, 2, DESTRUCT_00B79930],
                vec![this + 0x37c, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x36c, 4, 4, DESTRUCT_00B66F20],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bb2bb0_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(
            &mut e,
            &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460],
        );
        let this = e.mem.alloc(0x38c);
        assert_eq!(e.call(0x00bb_2bb0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x38c]]);
        e.call(0x00bb_2bb0, &args![this, 2u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bb25d0_inverts_the_transform_and_sets_up_the_light() {
        for kind in [6u32, 5] {
            let mut e = rig();
            noop(
                &mut e,
                &[
                    RENDERER_CALL_00E6FBB0,
                    LIGHT_SETUP_00BB1960,
                    CAMERA_ROWS_00BB1A90,
                ],
            );
            e.register(D3DX_MATRIX_INVERSE, |e, a| {
                let matrix: Vec<f32> = (0..16).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
                assert_eq!(
                    matrix,
                    vec![
                        0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 1.0, 2.0, 3.0,
                        1.0
                    ]
                );
                Ret::default()
            });
            let this = e.mem.alloc(0x38c);
            e.mem.set_u32(this + 0x18, 0x5151);
            let transform = e.mem.alloc(0x34);
            for (i, v) in [
                0.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 3.0,
            ]
            .iter()
            .enumerate()
            {
                e.mem.set_f32(transform + 4 * i as u32, *v);
            }
            let state = e.mem.alloc(0x10);
            let property = e.mem.alloc(0x20);
            e.mem.set_u32(property + 0x1c, kind);
            e.mem.set_u32(state + 0xc, property);
            let answer = e
                .call(
                    0x00bb_25d0,
                    &args![this, 1u32, 2u32, 3u32, 4u32, state, 6u32, transform, 8u32],
                )
                .u32();
            assert_eq!(answer, 0);
            assert_eq!(
                calls(&e, RENDERER_CALL_00E6FBB0),
                vec![vec![0x5151, transform, 0]]
            );
            let light = calls(&e, LIGHT_SETUP_00BB1960);
            let expected = if kind == 6 { property } else { 0 };
            assert_eq!(light[0][..2], [this, expected]);
            assert_eq!(light[0][3], 3.0f32.to_bits());
            assert_eq!(light[0][2], calls(&e, D3DX_MATRIX_INVERSE)[0][0]);
            assert_eq!(calls(&e, CAMERA_ROWS_00BB1A90).len(), 1);
        }
    }

    #[test]
    fn fn_00bb26d0_holds_the_block_while_it_fills_it() {
        let mut e = rig();
        map_globals(&mut e);
        let state = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x10);
        let block = e.mem.alloc(0x64);
        e.mem.set_u32(block + 0x60, 1);
        e.mem.set_u32(table, block);
        e.mem.set_u32(state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, state);
        let this = e.mem.alloc(0x38c);
        let table0 = vtable(&mut e, 0);
        e.mem.set_u32(this, table0);
        let object = e.mem.alloc(8);
        let table1 = vtable(&mut e, 1);
        e.mem.set_u32(object, table1);
        e.register(slot(1, 0xb4), |_, _| 0x7777u32.into_ret());
        e.register_double(slot(0, 0x108), move |e, _| {
            // The block is held while the shader's slot runs.
            assert_eq!(e.mem.u32(block + 0x60), 2);
            Ret::default()
        });
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, object);
        e.call(0x00bb_26d0, &args![this, source]);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block + 0x60), 1);
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
        // A block only this function held is deleted at the end.
        e.mem.set_u32(block + 0x60, 0);
        e.register(slot(0, 0x108), |_, _| Ret::default());
        e.call(0x00bb_26d0, &args![this, source]);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![block]]);
    }

    /// A rig for `00bb2760`: a record with two lights, the state's object (vtable 1) whose
    /// slot +0x100 answers leaf data and slot +0xfc an object with floats.
    fn draw_rig(full_bright: u8, mode: u32) -> (Engine, u32, u32, u32) {
        let mut e = rig();
        map_globals(&mut e);
        e.set_global(FULL_BRIGHT_011F91A7, full_bright);
        e.set_global(MODE_011F91B8, mode);
        e.set_global(DIFFUSE_VALUE_010231A8, 0.4f32);
        e.set_global(SCALE_01200608, 2.0f32);
        e.set_global(SCALE_0120060C, 3.0f32);
        let pass_state = e.mem.alloc(0x60);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        let this = e.mem.alloc(0x38c);
        let table0 = vtable(&mut e, 0);
        e.mem.set_u32(this, table0);
        noop(&mut e, &[slot(0, 0x60), LIGHT_CALL_00B71360, MEMCPY]);
        let record = e.mem.alloc(0x10);
        let geometry = e.mem.alloc(0xc0);
        let passes = e.mem.alloc(0x38);
        e.mem.set_u32(passes + 0x34, 3);
        e.mem.set_u32(geometry + 0xb8, passes);
        e.mem.set_u32(geometry + 0xbc, 0xbcbc);
        e.mem.set_u32(geometry + 0x20, 0x2020);
        e.mem.set_u32(record, geometry);
        e.mem.set_u8(record + 9, 2);
        let lights = e.mem.alloc(8);
        e.mem.set_u32(lights, 0x1111);
        e.mem.set_u32(lights + 4, 0x2222);
        e.mem.set_u32(record + 0xc, lights);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        let object = e.mem.alloc(0x70);
        let table1 = vtable(&mut e, 1);
        e.mem.set_u32(object, table1);
        e.mem.set_f32(object + 0x6c, 0.5);
        let data = e.mem.alloc(0x20);
        for (offset, value) in [
            (0xcu32, 1.0f32),
            (0x10, 2.0),
            (0x14, 3.0),
            (0x18, 4.0),
            (0x1c, 5.0),
        ] {
            e.mem.set_f32(data + offset, value);
        }
        e.register(slot(1, 0x100), |_, _| 0x6000u32.into_ret());
        e.register_double(slot(1, 0xfc), move |_, _| data.into_ret());
        let state = e.mem.alloc(0x10);
        e.mem.set_u32(state + 0xc, object);
        (e, this, state, geometry)
    }

    #[test]
    fn fn_00bb2760_sets_up_the_leaves() {
        let (mut e, this, state, geometry) = draw_rig(0, 0);
        e.set_global(AMBIENT_011FA0C0, 0x1234u32);
        e.call(0x00bb_2760, &args![this, state]);
        let half = 0.5f32.to_bits();
        assert_eq!(
            calls(&e, LIGHT_CALL_00B71360),
            vec![vec![0, 0x1111, half], vec![1, 0x2222, half]]
        );
        assert_eq!(
            calls(&e, slot(0, 0x60)),
            vec![vec![
                this,
                geometry,
                0,
                0,
                3,
                state,
                0,
                geometry + 0x68,
                0x2020
            ]]
        );
        assert_eq!(e.global::<u32>(AMBIENT_011FA0C0), 0x1234);
        assert_eq!(calls(&e, MEMCPY), vec![vec![this + 0x6c, 0x6000, 0x300]]);
        assert_eq!(e.global::<f32>(LEAF_0120065C), 8.0);
        assert_eq!(e.global::<f32>(LEAF_0120066C), 15.0);
        assert_eq!(e.global::<f32>(LEAF_01200660), 3.0);
        assert_eq!(e.global::<f32>(LEAF_01200670), 2.0);
        let tree: Vec<f32> = (0..4)
            .map(|i| e.global::<f32>(TREE_DATA_01200678 + 4 * i))
            .collect();
        assert_eq!(tree, vec![0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00bb2760_full_bright_colours_and_the_constant_users() {
        for mode in [9u32, 8] {
            let (mut e, this, state, geometry) = draw_rig(1, mode);
            e.set_global(DIFFUSE_011FA0D0, 0x5555u32);
            let users = [e.mem.alloc(8), e.mem.alloc(8)];
            let table2 = vtable(&mut e, 2);
            for user in users {
                e.mem.set_u32(user, table2);
            }
            e.mem.set_u32(this + 0x34, users[0]);
            e.mem.set_u32(this + 0x30, users[1]);
            let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
            e.mem.set_u32(pass_state + 0x5c, 0x5c5c);
            e.mem.set_u32(pass_state + 0x44, 0x4444);
            noop(&mut e, &[slot(2, 0x78)]);
            e.call(0x00bb_2760, &args![this, state]);
            let ambient: Vec<u32> = (0..4)
                .map(|i| e.global::<u32>(AMBIENT_011FA0C0 + 4 * i))
                .collect();
            assert_eq!(ambient, vec![ONE_BITS; 4]);
            let diffuse: Vec<f32> = (0..4)
                .map(|i| e.global::<f32>(DIFFUSE_011FA0D0 + 4 * i))
                .collect();
            let expected = if mode == 9 {
                vec![0.0, 0.4, 0.0, 0.0]
            } else {
                vec![0.0; 4]
            };
            assert_eq!(diffuse, expected);
            let used = calls(&e, slot(2, 0x78));
            assert_eq!(used.len(), 2);
            assert_eq!(
                used[0],
                vec![
                    users[0],
                    0x5c5c,
                    geometry,
                    0xbcbc,
                    0,
                    3,
                    state,
                    0,
                    geometry + 0x68,
                    0x2020,
                    0,
                    1
                ]
            );
            assert_eq!(used[1][..2], [users[1], 0x4444]);
        }
    }

    #[test]
    fn create_constant_maps_adds_the_leaf_constants() {
        const MAP_SLOTS: u32 = 0x0e30_0000;
        const MAP_VTABLE: u32 = 0x0e31_0000;
        for dimmer in [0u8, 1] {
            let mut e = rig();
            map_globals(&mut e);
            e.set_global(DIMMER_SWITCH_011F941E, dimmer);
            let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
            e.put_vtable(MAP_VTABLE, &targets);
            e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
                e.mem.set_u32(a[0], MAP_VTABLE);
                a[0].into_ret()
            });
            noop(&mut e, &[MAP_SLOTS + 8, SET_SHADER_CONSTANTS_00BB2F50]);
            let this = e.mem.alloc(0x38c);
            e.call(0x00bb_2d10, &args![this]);
            let map = e.mem.u32(this + 0x34);
            let entries = calls(&e, MAP_SLOTS + 8);
            assert_eq!(entries.len(), 8 + usize::from(dimmer));
            let last = entries.last().unwrap();
            assert_eq!(
                last,
                &vec![
                    map,
                    LEAF_DATA_010B92D0,
                    0x1000_0009,
                    0,
                    0x22,
                    0x30,
                    CONSTANT_EMPTY_NAME_01011584,
                    0x300,
                    4,
                    this + 0x6c,
                    0
                ]
            );
            assert_eq!(entries[entries.len() - 2][1], GROUPED_CONSTANTS_010B8C34);
            assert_eq!(
                calls(&e, SET_SHADER_CONSTANTS_00BB2F50),
                vec![vec![map, 0x11]]
            );
            // The constants call comes between the grouped constants and the leaf data.
            let log = e.call_log.as_ref().unwrap();
            let order: Vec<u32> = log
                .iter()
                .filter(|c| c.0 == SET_SHADER_CONSTANTS_00BB2F50 || c.0 == MAP_SLOTS + 8)
                .map(|c| c.0)
                .collect();
            assert_eq!(order[order.len() - 2], SET_SHADER_CONSTANTS_00BB2F50);
            e.call_log = Some(vec![]);
            e.call(0x00bb_2d10, &args![this]);
            assert!(calls(&e, MAP_SLOTS + 8).is_empty());
        }
    }
}
