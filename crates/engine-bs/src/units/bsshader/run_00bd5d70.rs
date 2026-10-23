//! `BSShader/run_00bd5d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00bd6ec0` (`run_00bd5d70`, unnamed; `thiscall`, no stack words), the
/// `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word), the set-up
/// `00bd6ae0` (unnamed; `thiscall` on +0x58, one word) and `00b8bc20` (unit
/// `imagespacemanager.cpp`, unnamed; `thiscall`, no stack words).
const BASE_CONSTRUCT_00BD6EC0: u32 = 0x00bd_6ec0;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const SET_UP_00BD6AE0: u32 = 0x00bd_6ae0;
const CALL_00B8BC20: u32 = 0x00b8_bc20;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// This effect's vtable (`010bc410`; no RTTI locator before it).
const VTABLE_010BC410: u32 = 0x010b_c410;

// Translated from 00bd7120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00bd6ec0`, stores the vtable
/// `010bc410`, clears the bytes +0x7c to +0x7e and the words +0x80 to +0x88, constructs the
/// three 16-byte textures at +0x8c (`eh vector constructor iterator` with `004ebb70` and
/// `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 6 (`00a90180`),
/// stores 0.0 at +0x68 to +0x70, clears +0x74, runs `00bd6ae0` on +0x58 with 4 and `00b8bc20`.
/// Answers `this`.
// Translation incomplete: 00bd7120 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00bd7120(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00BD6EC0, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC410);
    for offset in [0x7cu32, 0x7d, 0x7e] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x80u32, 0x84, 0x88] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x8c,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 6u32]);
    for offset in [0x68u32, 0x6c, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_UP_00BD6AE0, &args![base + 0x58, 4u32]);
    e.call(CALL_00B8BC20, &args![base]);
    this
}

/// The Ni allocation `00aa1070` and free `00aa10f0` (`cdecl`, one word each).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00bd6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the set-size of an `NiTArray` of bytes (base +4,
/// max size +8, size +0xa, element count +0xc, all 16-bit but the base), `thiscall` with one
/// stack word: the new size (compared whole with the 16-bit fields). Nothing when it equals the
/// max size. A size below the current size clears the set bytes from the new size up (each one
/// lowering the element count) and becomes the size. The max size takes the new size; with 0
/// the base is cleared, else a new block of that many bytes (`00aa1070`) takes the old bytes up
/// to the size and zeros up to the max size. The old block is freed (`00aa10f0`).
pub fn fn_00bd6ae0(e: &mut Engine, this: Ptr, size: u32) {
    let base = this.addr();
    if size == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    let narrow = size as u16;
    if size < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = narrow;
        if index < e.mem.u16(base + 0xa) {
            loop {
                let byte = e.mem.u32(base + 4).wrapping_add(u32::from(index));
                if e.mem.u8(byte) != 0 {
                    e.mem.set_u8(byte, 0);
                    let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(base + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(base + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(base + 0xa, narrow);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, narrow);
    if size == 0 {
        e.mem.set_u32(base + 4, 0);
        e.call(NI_FREE_00AA10F0, &args![old]);
        return;
    }
    let block = e
        .call(NI_ALLOCATE_00AA1070, &args![u32::from(narrow)])
        .u32();
    e.mem.set_u32(base + 4, block);
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let byte = e.mem.u8(old.wrapping_add(u32::from(index)));
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), byte);
        index = index.wrapping_add(1);
    }
    let mut index = e.mem.u16(base + 0xa);
    while index < e.mem.u16(base + 8) {
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), 0);
        index = index.wrapping_add(1);
    }
    e.call(NI_FREE_00AA10F0, &args![old]);
}

/// The image-space effect base constructor `00ba4080` (`thiscall`, no stack words), this
/// effect's vtable (`010bc3f0`) and its byte array's (`010b9d24`) (no RTTI locators before
/// them).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BC3F0: u32 = 0x010b_c3f0;
const ARRAY_VTABLE_010B9D24: u32 = 0x010b_9d24;

// Translated from 00bd6ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the base constructor of `00bd7120`'s effect,
/// `thiscall`, no stack words. Runs the image-space effect base constructor `00ba4080`, stores
/// the vtable `010bc3f0` and makes the empty array at +0x58 (max size +0x60 and element count
/// +0x64 0, base +0x5c null, grow-by +0x66 1, size +0x62 0, vtable `010b9d24`, in that order).
/// Answers `this`.
// Translation incomplete: 00bd6ec0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bd6ec0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC3F0);
    e.mem.set_u16(base + 0x60, 0);
    e.mem.set_u16(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u16(base + 0x66, 1);
    e.mem.set_u16(base + 0x62, 0);
    e.mem.set_u32(base + 0x58, ARRAY_VTABLE_010B9D24);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The array store `00ba3aa0` (unnamed; `thiscall` on an effect array, `RET 8`: index, the
/// address of the word to store).
const ARRAY_SET_00BA3AA0: u32 = 0x00ba_3aa0;
/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`, `RET 4`) and
/// `::ReturnRenderedTexture` (`00ba39a0`, no stack words) (engine map; `thiscall` on the
/// texture), the texture setter `00ba37f0` (unnamed, `RET 4`) and `00b54090` (unnamed, unit
/// `bsshadermanager.cpp`, no arguments; answers a word).
const BORROW_RENDERED_TEXTURE: u32 = 0x00ba_3840;
const RETURN_RENDERED_TEXTURE: u32 = 0x00ba_39a0;
const TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const SHADER_MANAGER_CALL_00B54090: u32 = 0x00b5_4090;

// Translated from 00bd6980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x10), `thiscall`
/// with one stack word the code never reads (`RET 4`). Borrows rendered texture 0x25 into the
/// texture at +0x8c and 0x26 into +0x9c, storing them as elements 2 and 3 of the array at
/// +0x28 (`00ba3aa0`, through one stack word); element 5 is the texture at +0x7c when the byte
/// +3 of the object at +0x5c is set, else that texture is returned (`ReturnRenderedTexture`) and
/// element 5 cleared; the texture at +0xac gets `00b54090`'s answer (`00ba37f0`) and becomes
/// element 4.
pub fn fn_00bd6980(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let array = base + 0x28;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, base + 0x8c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x8c, 0x25u32]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 2u32, local]);
        e.mem.set_u32(local, base + 0x9c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x9c, 0x26u32]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 3u32, local]);
        let flags = e.mem.u32(base + 0x5c);
        if e.mem.u8(flags + 3) != 0 {
            e.mem.set_u32(local, base + 0x7c);
        } else {
            e.call(RETURN_RENDERED_TEXTURE, &args![base + 0x7c]);
            e.mem.set_u32(local, 0);
        }
        e.call(ARRAY_SET_00BA3AA0, &args![array, 5u32, local]);
        e.mem.set_u32(local, base + 0xac);
        let texture = e.call(SHADER_MANAGER_CALL_00B54090, &args![]).u32();
        e.call(TEXTURE_SET_00BA37F0, &args![base + 0xac, texture]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 4u32, local]);
    });
}

// Translated from 00bd6a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x14), `thiscall`,
/// no stack words. Returns the rendered textures of elements 2 and 3 of the array at +0x28
/// (`ReturnRenderedTexture` on the elements, base at +0x2c) and clears the elements 2, 3 and
/// 4 (`00ba3aa0` with a null word); when element 5 is set, returns the texture at +0x7c and
/// clears element 5.
pub fn fn_00bd6a30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let array = base + 0x28;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let elements = e.mem.u32(base + 0x2c);
        let texture = e.mem.u32(elements + 8);
        e.call(RETURN_RENDERED_TEXTURE, &args![texture]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 2u32, local]);
        let elements = e.mem.u32(base + 0x2c);
        let texture = e.mem.u32(elements + 0xc);
        e.call(RETURN_RENDERED_TEXTURE, &args![texture]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 3u32, local]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 4u32, local]);
        let elements = e.mem.u32(base + 0x2c);
        if e.mem.u32(elements + 0x14) != 0 {
            e.call(RETURN_RENDERED_TEXTURE, &args![base + 0x7c]);
            e.mem.set_u32(local, 0);
            e.call(ARRAY_SET_00BA3AA0, &args![array, 5u32, local]);
        }
    });
}

/// The byte setting `00bd6ab0` tests (`012023d4`).
const ENABLED_012023D4: u32 = 0x0120_23d4;

// Translated from 00bd6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x18), `thiscall`,
/// no stack words: true when the `float` at +0x70 is above 0.0 (`FLDZ; FCOMP`; NaN is not), the
/// byte setting `012023d4` is set and `00b54090` answers non-zero.
pub fn fn_00bd6ab0(e: &mut Engine, this: Ptr) -> bool {
    let value = x87_load32(e.mem.u32(this.addr() + 0x70));
    if 0.0 < value
        && e.global::<u8>(ENABLED_012023D4) != 0
        && e.call(SHADER_MANAGER_CALL_00B54090, &args![]).u32() != 0
    {
        return true;
    }
    false
}

/// `ImageSpaceShaderParam::SetPixelConstant` (engine map, `00b8a790`, `thiscall`, `RET 0x14`:
/// index, four `float`s).
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
/// The object whose `float`s `00bd66c0` reads (`011f917c`; +0x6c to +0x88 and +0xec, +0xf0).
const SCENE_011F917C: u32 = 0x011f_917c;
/// The `float`s the constants take: `012023e0`, the first word of constant 2 (`01070218`), the
/// vector `012004f0` to `012004f8`, the divisor setting `011f9410` with the `double` 30.0
/// (`0101db88`), and the 3.0 (`01017718`) stored in `012003c4`.
const CONSTANT_012023E0: u32 = 0x0120_23e0;
const CONSTANT_01070218: u32 = 0x0107_0218;
const VECTOR_012004F0: u32 = 0x0120_04f0;
const DIVISOR_011F9410: u32 = 0x011f_9410;
const THIRTY_0101DB88: u32 = 0x0101_db88;
const THREE_01017718: u32 = 0x0101_7718;
const RESULT_012003C4: u32 = 0x0120_03c4;

// Translated from 00bd66c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot +0x1c),
/// `thiscall` with one stack word the code never reads (`RET 4`); answers true. With `a`, `b`
/// the `float`s +0xec and +0xf0 of the object at `011f917c` and the effect's mode at +0x74,
/// sets on the shader parameters of elements 2 and then 3 of the array at +0x18 (base +0x1c)
/// the pixel constants 0 (`[+0x68] - [+0x6c]`, `[+0x6c] + [+0x68]`, `[+0x68]`, `012023e0`), 1
/// (`[+0x70]`, 1.0 for modes 0 and 1 else 0.0, 1.0 for modes 0 and 2 else 0.0, 0.0) and 2
/// (`01070218`, `a`, `b - a`, `a * b`). When the setting `011f9410` is above 0.0, element 3
/// also gets constant 3: the dot products of the vector `012004f0` with the object's +0x70,
/// +0x7c, +0x88 and with the negated +0x6c, +0x78, +0x84, then `[+0x78] / (011f9410 * 30.0)`
/// and 0.0. Finally stores 3.0 (`01017718`) in `012003c4`. x87 arithmetic in the exe's order,
/// rounded to `float` where it stores.
pub fn fn_00bd66c0(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let scene = e.global::<u32>(SCENE_011F917C);
    let a_bits = x87_quiet_bits(e.mem.u32(scene + 0xec));
    let b_bits = x87_quiet_bits(e.mem.u32(scene + 0xf0));
    let mode = e.mem.u32(base + 0x74);
    let a = x87_load32(a_bits);
    let b = x87_load32(b_bits);
    let b_minus_a = x87_store32(x87_sub(b, a));
    let a_times_b = x87_store32(x87_mul(b, a));
    let first_flag = if mode == 1 || mode == 0 { 1.0f32 } else { 0.0 };
    let second_flag = if mode == 2 || mode == 0 { 1.0f32 } else { 0.0 };
    let p_bits = e.mem.u32(base + 0x68);
    let q_bits = e.mem.u32(base + 0x6c);
    let difference = x87_store32(x87_sub(x87_load32(p_bits), x87_m32(q_bits)));
    let sum = x87_store32(x87_add(x87_load32(q_bits), x87_m32(p_bits)));
    let mut parameter = 0;
    for element in [8u32, 0xc] {
        let elements = e.mem.u32(base + 0x1c);
        parameter = e.mem.u32(elements + element);
        let constant = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
        let p = x87_quiet_bits(e.mem.u32(base + 0x68));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 0u32, difference, sum, p, constant],
        );
        let value = x87_quiet_bits(e.mem.u32(base + 0x70));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 1u32, value, first_flag, second_flag, 0.0f32],
        );
        let first_word = x87_quiet_bits(e.global::<u32>(CONSTANT_01070218));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 2u32, first_word, a_bits, b_minus_a, a_times_b],
        );
    }
    let negated = [0x6cu32, 0x78, 0x84]
        .map(|offset| x87_store32(-x87_load32(x87_quiet_bits(e.mem.u32(scene + offset)))));
    let plain = [0x70u32, 0x7c, 0x88].map(|offset| x87_quiet_bits(e.mem.u32(scene + offset)));
    let vector = [0u32, 4, 8].map(|offset| e.global::<u32>(VECTOR_012004F0 + offset));
    let divisor = x87_quiet_bits(e.global::<u32>(DIVISOR_011F9410));
    let divisor = x87_load32(divisor);
    if divisor > 0.0 {
        let thirty = f64::from_bits(e.global::<u64>(THIRTY_0101DB88));
        let numerator = x87_load32(e.mem.u32(base + 0x78));
        let ratio = x87_store32(x87_div(numerator, x87_mul(divisor, thirty)));
        let [v0, v1, v2] = vector.map(x87_load32);
        let m1 = x87_mul(v1, x87_m32(negated[1]));
        let m2 = x87_mul(x87_load32(negated[0]), v0);
        let sum1 = x87_add(m2, m1);
        let m3 = x87_mul(v2, x87_m32(negated[2]));
        let second = x87_store32(x87_add(sum1, m3));
        let m4 = x87_mul(v1, x87_load32(plain[1]));
        let m5 = x87_mul(v0, x87_load32(plain[0]));
        let sum3 = x87_add(m5, m4);
        let m6 = x87_mul(v2, x87_load32(plain[2]));
        let first = x87_store32(x87_add(m6, sum3));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 3u32, first, second, ratio, 0.0f32],
        );
    }
    let three = x87_quiet_bits(e.global::<u32>(THREE_01017718));
    e.set_global::<u32>(RESULT_012003C4, three);
    true
}

/// `00ba3f90` (unnamed; `thiscall`, one word), the renderer word (`011f9508`),
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`) and `::InitConstantMap`
/// (`00b8ab10`, `RET 8`) (engine map), `00b8a8b0` (unnamed, `RET 0xc`), and the effect calls
/// `00ba3b10` (`RET 0xc`), `00ba3b50` (`RET 0x14`), `00ba3ba0` (`RET 0x1c`), `00ba3c00` (`RET
/// 0x24`) and `00ba3c70` (`RET 0x10`) (unnamed; `thiscall` on the effect).
const EFFECT_SET_UP_00BA3F90: u32 = 0x00ba_3f90;
const RENDERER_011F9508: u32 = 0x011f_9508;
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_CALL_00B8A8B0: u32 = 0x00b8_a8b0;
const EFFECT_CALL_00BA3B10: u32 = 0x00ba_3b10;
const EFFECT_CALL_00BA3B50: u32 = 0x00ba_3b50;
const EFFECT_CALL_00BA3BA0: u32 = 0x00ba_3ba0;
const EFFECT_CALL_00BA3C00: u32 = 0x00ba_3c00;
const EFFECT_CALL_00BA3C70: u32 = 0x00ba_3c70;
/// The `float` 2^32 added to a negative `FILD` (`01099378`), the data `00b8a8b0` takes
/// (`011add48`) and the second and third `float`s of the last two constants (`01013974`,
/// `01013d84`).
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const DATA_011ADD48: u32 = 0x011a_dd48;
const CONSTANT_01013974: u32 = 0x0101_3974;
const CONSTANT_01013D84: u32 = 0x0101_3d84;

/// A renderer dimension: vtable slot +0xc8 of the renderer at `011f9508` gives an object whose
/// slot `slot` answers it with 0; loaded as signed (`FILD`), 2^32 added when negative, rounded
/// to a `float` (its bits).
fn renderer_dimension(e: &mut Engine, slot: u32) -> u32 {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let target = e.vcall(renderer, 0xc8, &args![]).u32();
    let value = e.vcall(target, slot, &args![0u32]).u32();
    let mut loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        loaded = x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_store32(loaded)
}

/// A shader parameter: 0x88 bytes from the memory manager, the constructor run when the block
/// is not null; answers the parameter, or 0 for a null block.
fn new_parameter(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    if block != 0 {
        e.call(SHADER_PARAM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 00bd6bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot +0x8),
/// `thiscall` with two stack words (`RET 8`): an owner whose word at +0x8 holds the textures
/// (+0x104, +0x48, +0xcc, +0xd0), and a word the code never reads. Runs `00ba3f90` with 4, reads
/// the renderer's two dimensions (slots +0x8c and +0x90 of the object its slot +0xc8 gives).
/// Shader parameter 0 (`InitConstantMap(0, 0x18)`, `00b8a8b0(3, 011add48, 0x40)`, pixel
/// constant 2 (1 / first dimension, 1 / second, 0.0, 0.0), byte +0x55 cleared) goes to
/// `00ba3c70(0, texture +0x104, parameter, 2)` and element 0 of the array at +0x18; element 1 is
/// cleared after `00ba3b10(0, 1, 1)`, then `00ba3b50(1, 2, 3, 0, 0)` and `00ba3c70(1, texture
/// +0x48, 0, 2)`. Parameters 2 (`InitConstantMap(0, 3)`) and 3 (`InitConstantMap(0, 4)`) get
/// pixel constant 0 (1.0, `01013974`, `01013d84`, `012023e0`), byte +0x56 cleared, become
/// elements 2 and 3 with `00ba3ba0(2, 1, 2, 4, 0, 1, 0)` / `00ba3c00(3, 1, 2, 4, 5, 0, 1, 0, 0)`
/// and `00ba3c70` with the textures +0xcc / +0xd0 and 0. A null parameter block gives a null
/// parameter.
// Translation incomplete: 00bd6bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the parameters' allocations) is not translated.
pub fn fn_00bd6bb0(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let base = this.addr();
    let array = base + 0x18;
    e.call(EFFECT_SET_UP_00BA3F90, &args![base, 4u32]);
    let first_dimension = renderer_dimension(e, 0x8c);
    let second_dimension = renderer_dimension(e, 0x90);
    let parameter = new_parameter(e);
    e.call(
        SHADER_PARAM_INIT_CONSTANT_MAP,
        &args![parameter, 0u32, 0x18u32],
    );
    e.call(
        SHADER_PARAM_CALL_00B8A8B0,
        &args![parameter, 3u32, DATA_011ADD48, 0x40u32],
    );
    let inverse_second = x87_store32(x87_div(1.0, x87_load32(second_dimension)));
    let inverse_first = x87_store32(x87_div(1.0, x87_m32(first_dimension)));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![
            parameter,
            2u32,
            x87_quiet_bits(inverse_first),
            x87_quiet_bits(inverse_second),
            0.0f32,
            0.0f32
        ],
    );
    e.mem.set_u8(parameter + 0x55, 0);
    let textures = e.mem.u32(owner.addr() + 8);
    let texture = e.mem.u32(textures + 0x104);
    e.call(
        EFFECT_CALL_00BA3C70,
        &args![base, 0u32, texture, parameter, 2u32],
    );
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, parameter);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 0u32, local]);
        e.call(EFFECT_CALL_00BA3B10, &args![base, 0u32, 1u32, 1u32]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 1u32, local]);
        e.call(
            EFFECT_CALL_00BA3B50,
            &args![base, 1u32, 2u32, 3u32, 0u32, 0u32],
        );
        let textures = e.mem.u32(owner.addr() + 8);
        let texture = e.mem.u32(textures + 0x48);
        e.call(
            EFFECT_CALL_00BA3C70,
            &args![base, 1u32, texture, 0u32, 2u32],
        );
        let one = 1.0f32.to_bits();
        for (index, registers, texture_offset) in [(2u32, 3u32, 0xccu32), (3, 4, 0xd0)] {
            let parameter = new_parameter(e);
            e.call(
                SHADER_PARAM_INIT_CONSTANT_MAP,
                &args![parameter, 0u32, registers],
            );
            let fourth = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
            let third = x87_quiet_bits(e.global::<u32>(CONSTANT_01013D84));
            let second = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
            e.call(
                SHADER_PARAM_SET_PIXEL_CONSTANT,
                &args![parameter, 0u32, one, second, third, fourth],
            );
            e.mem.set_u8(parameter + 0x56, 0);
            e.mem.set_u32(local, parameter);
            e.call(ARRAY_SET_00BA3AA0, &args![array, index, local]);
            if index == 2 {
                e.call(
                    EFFECT_CALL_00BA3BA0,
                    &args![base, 2u32, 1u32, 2u32, 4u32, 0u32, 1u32, 0u32],
                );
            } else {
                e.call(
                    EFFECT_CALL_00BA3C00,
                    &args![base, 3u32, 1u32, 2u32, 4u32, 5u32, 0u32, 1u32, 0u32, 0u32],
                );
            }
            let textures = e.mem.u32(owner.addr() + 8);
            let texture = e.mem.u32(textures + texture_offset);
            e.call(
                EFFECT_CALL_00BA3C70,
                &args![base, index, texture, parameter, 0u32],
            );
        }
    });
}

/// The byte array's vtable restored by the base destructor (`010b9d24`), the Ni free
/// (`00aa10f0`, `cdecl`) and `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`).
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

// Translated from 00bd6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the destructor of `00bd6ec0`'s base, `thiscall`,
/// no stack words. Restores the byte array's vtable at +0x58 (`010b9d24`), frees its base at
/// +0x5c (`00aa10f0`) and runs `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`).
// Translation incomplete: 00bd6f30 - the C++ exception frame (FS:[0] chain, unwind state 0 for the array) is not translated.
pub fn fn_00bd6f30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base + 0x5c);
    e.mem.set_u32(base + 0x58, ARRAY_VTABLE_010B9D24);
    e.call(NI_FREE_00AA10F0, &args![block]);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The compiler's `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size,
/// count, destructor) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;

// Translated from 00bd6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the destructor of `00bd7120`'s effect,
/// `thiscall`, no stack words. Stores the vtable `010bc410`; for the elements 2, 1, 0, 0 again
/// and 3 of the array at +0x18 (base +0x1c): when set, deletes it through its vtable slot 0
/// with 1 (when not null) and clears it (`00ba3aa0` with a null word). Destroys the three
/// textures at +0x8c (`eh vector destructor iterator`) and the texture at +0x7c, then runs the
/// base destructor `00bd6f30`.
// Translation incomplete: 00bd6fc0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the textures and the base) is not translated.
pub fn fn_00bd6fc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC410);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in [2u32, 1, 0, 0, 3] {
            let elements = e.mem.u32(base + 0x1c);
            if e.mem.u32(elements + 4 * index) != 0 {
                let parameter = e.mem.u32(elements + 4 * index);
                if parameter != 0 {
                    e.vcall(parameter, 0, &args![1u32]);
                }
                e.mem.set_u32(local, 0);
                e.call(ARRAY_SET_00BA3AA0, &args![base + 0x18, index, local]);
            }
        }
    });
    e.call(
        VECTOR_DESTRUCT_00EC5FCE,
        &args![base + 0x8c, 0x10u32, 3u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0x7c]);
    fn_00bd6f30(e, this);
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00bd71e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot 0): the
/// effect's scalar deleting destructor, `thiscall`, one stack word (the flags). Runs `00bd6fc0`;
/// with bit 0 set, frees `this` through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bd71e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd6fc0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_7120, fn_00bd7120(Ptr) -> Ptr),
        entry!(0x00bd_6ae0, fn_00bd6ae0(Ptr, u32)),
        entry!(0x00bd_6ec0, fn_00bd6ec0(Ptr) -> Ptr),
        entry!(0x00bd_6980, fn_00bd6980(Ptr, u32)),
        entry!(0x00bd_6a30, fn_00bd6a30(Ptr)),
        entry!(0x00bd_6ab0, fn_00bd6ab0(Ptr) -> bool),
        entry!(0x00bd_66c0, fn_00bd66c0(Ptr, u32) -> bool),
        entry!(0x00bd_6bb0, fn_00bd6bb0(Ptr, Ptr, u32)),
        entry!(0x00bd_6f30, fn_00bd6f30(Ptr)),
        entry!(0x00bd_6fc0, fn_00bd6fc0(Ptr)),
        entry!(0x00bd_71e0, fn_00bd71e0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd7120_builds_the_effect() {
        let mut e = Engine::new();
        for target in [
            BASE_CONSTRUCT_00BD6EC0,
            ARRAY_SET_SIZE_00A90180,
            SET_UP_00BD6AE0,
            CALL_00B8BC20,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        e.mem.write(this, &[0xcc; 0xbc]);
        assert_eq!(e.call(0x00bd_7120, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(e.mem.bytes(this + 0x68, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.bytes(this + 0x7c, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x7f), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x80, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BASE_CONSTRUCT_00BD6EC0, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x8c,
                        0x10,
                        3,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 6]),
                (SET_UP_00BD6AE0, vec![this + 0x58, 4]),
                (CALL_00B8BC20, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An array of max size 4 and size 4 holding 1, 0, 2, 3 (element count 3).
    fn array(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let old = e.mem.alloc(4);
        e.mem.write(old, &[1, 0, 2, 3]);
        e.mem.set_u32(this + 4, old);
        e.mem.set_u16(this + 8, 4);
        e.mem.set_u16(this + 0xa, 4);
        e.mem.set_u16(this + 0xc, 3);
        (this, old)
    }

    #[test]
    fn fn_00bd6ae0_shrinks_and_grows_the_byte_array() {
        let mut e = engine();
        let (this, old) = array(&mut e);
        e.call(0x00bd_6ae0, &args![this, 2u32]);
        // Bytes 2 and 3 cleared (both set): count 1, size 2, max 2, a new block of 2.
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 2);
        let block = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(block, 2), vec![1, 0]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070), vec![vec![2]]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![old]]);
        // Growing keeps the size and zeros the rest.
        e.mem.write(block, &[7, 8]);
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        let grown = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(grown, 5), vec![7, 8, 0, 0, 0]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 5);
        // The same max size: nothing.
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070).len(), 2);
        // Size 0: base cleared, the old block freed.
        e.call(0x00bd_6ae0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(calls(&e, NI_FREE_00AA10F0).last().unwrap(), &vec![grown]);
    }

    #[test]
    fn fn_00bd6ec0_builds_the_effect_and_its_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00bd_6ec0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC3F0);
        assert_eq!(e.mem.u32(this + 0x58), ARRAY_VTABLE_010B9D24);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xa), vec![0; 0xa]);
        assert_eq!(e.mem.u16(this + 0x66), 1);
        assert_eq!(calls(&e, EFFECT_CONSTRUCT_00BA4080), vec![vec![this]]);
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;

    /// No-op doubles for every callee outside the file, the array store as a real store and
    /// the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            BORROW_RENDERED_TEXTURE,
            RETURN_RENDERED_TEXTURE,
            TEXTURE_SET_00BA37F0,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            EFFECT_SET_UP_00BA3F90,
            SHADER_PARAM_INIT_CONSTANT_MAP,
            SHADER_PARAM_CALL_00B8A8B0,
            EFFECT_CALL_00BA3B10,
            EFFECT_CALL_00BA3B50,
            EFFECT_CALL_00BA3BA0,
            EFFECT_CALL_00BA3C00,
            EFFECT_CALL_00BA3C70,
            EFFECT_DESTRUCT_00BA3D30,
            NI_FREE_00AA10F0,
            VECTOR_DESTRUCT_00EC5FCE,
            TEXTURE_DESTRUCT_00BA3A00,
            MEMORY_DEALLOCATE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SHADER_MANAGER_CALL_00B54090, |_, _| {
            0x0e66_0000u32.into_ret()
        });
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            let word = e.mem.u32(a[2]);
            let elements = e.mem.u32(a[0] + 4);
            e.mem.set_u32(elements + 4 * a[1], word);
            Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SHADER_PARAM_CONSTRUCT, |_, a| a[0].into_ret());
        for page in [
            0x0120_0000u32,
            0x0120_2000,
            0x011f_9000,
            0x0107_0000,
            0x0101_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0101_7000, 0x1000);
        e.map(0x0101_3000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An effect with the arrays at +0x18 (base +0x1c) and +0x28 (base +0x2c), six slots each.
    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xbc);
        let first = e.mem.alloc(0x18);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x1c, first);
        e.mem.set_u32(this + 0x2c, second);
        this
    }

    fn element(e: &Engine, this: u32, base: u32, index: u32) -> u32 {
        let elements = e.mem.u32(this + base);
        e.mem.u32(elements + 4 * index)
    }

    #[test]
    fn fn_00bd6980_borrows_the_textures() {
        let mut e = engine();
        let this = effect(&mut e);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(this + 0x5c, flags);
        e.mem.set_u8(flags + 3, 1);
        e.call(0x00bd_6980, &args![this, 0xdeadu32]);
        assert_eq!(
            calls(&e, BORROW_RENDERED_TEXTURE),
            vec![vec![this + 0x8c, 0x25], vec![this + 0x9c, 0x26]]
        );
        assert_eq!(element(&e, this, 0x2c, 2), this + 0x8c);
        assert_eq!(element(&e, this, 0x2c, 3), this + 0x9c);
        assert_eq!(element(&e, this, 0x2c, 5), this + 0x7c);
        assert_eq!(element(&e, this, 0x2c, 4), this + 0xac);
        assert!(calls(&e, RETURN_RENDERED_TEXTURE).is_empty());
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![this + 0xac, 0x0e66_0000]]
        );
        // Without the byte: the texture at +0x7c is returned and element 5 cleared.
        e.mem.set_u8(flags + 3, 0);
        e.call(0x00bd_6980, &args![this, 0u32]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE), vec![vec![this + 0x7c]]);
        assert_eq!(element(&e, this, 0x2c, 5), 0);
    }

    #[test]
    fn fn_00bd6a30_returns_the_textures() {
        let mut e = engine();
        let this = effect(&mut e);
        let elements = e.mem.u32(this + 0x2c);
        for index in 2..6u32 {
            e.mem.set_u32(elements + 4 * index, 0x0e70_0000 + index);
        }
        e.call(0x00bd_6a30, &args![this]);
        assert_eq!(
            calls(&e, RETURN_RENDERED_TEXTURE),
            vec![vec![0x0e70_0002], vec![0x0e70_0003], vec![this + 0x7c]]
        );
        for index in 2..6u32 {
            assert_eq!(element(&e, this, 0x2c, index), 0);
        }
        // Element 5 empty: no third return.
        e.call(0x00bd_6a30, &args![this]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE).len(), 5);
    }

    #[test]
    fn fn_00bd6ab0_needs_a_positive_value_the_setting_and_the_manager() {
        let mut e = engine();
        let this = effect(&mut e);
        e.mem.set_f32(this + 0x70, 0.5);
        assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
        e.set_global::<u8>(ENABLED_012023D4, 1);
        assert!(e.call(0x00bd_6ab0, &args![this]).bool());
        for value in [0.0f32, -1.0, f32::NAN] {
            e.mem.set_f32(this + 0x70, value);
            assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
        }
        e.mem.set_f32(this + 0x70, 2.0);
        e.register(SHADER_MANAGER_CALL_00B54090, |_, _| Ret::default());
        assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
    }

    #[test]
    fn fn_00bd66c0_sets_the_constants_of_both_parameters() {
        let mut e = engine();
        let this = effect(&mut e);
        let scene = e.mem.alloc(0x100);
        e.set_global::<u32>(SCENE_011F917C, scene);
        e.mem.set_f32(scene + 0xec, 2.0);
        e.mem.set_f32(scene + 0xf0, 5.0);
        for (offset, value) in [
            (0x6cu32, 1.0f32),
            (0x78, 2.0),
            (0x84, 3.0),
            (0x70, 4.0),
            (0x7c, 5.0),
            (0x88, 6.0),
        ] {
            e.mem.set_f32(scene + offset, value);
        }
        for (index, value) in [1.0f32, 0.5, 0.25].iter().enumerate() {
            e.set_global::<f32>(VECTOR_012004F0 + 4 * index as u32, *value);
        }
        e.set_global::<f32>(CONSTANT_012023E0, 7.0);
        e.set_global::<f32>(CONSTANT_01070218, -1.0e8);
        e.set_global::<f32>(THREE_01017718, 3.0);
        e.set_global::<f64>(THIRTY_0101DB88, 30.0);
        e.set_global::<f32>(DIVISOR_011F9410, 2.0);
        e.mem.set_f32(this + 0x68, 3.0);
        e.mem.set_f32(this + 0x6c, 1.0);
        e.mem.set_f32(this + 0x70, 9.0);
        e.mem.set_u32(this + 0x74, 2);
        e.mem.set_f32(this + 0x78, 120.0);
        let elements = e.mem.u32(this + 0x1c);
        e.mem.set_u32(elements + 8, 0x0e80_0000);
        e.mem.set_u32(elements + 0xc, 0x0e81_0000);
        assert!(e.call(0x00bd_66c0, &args![this, 0u32]).bool());
        let f = |v: f32| v.to_bits();
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 7);
        for (parameter, rows) in [(0x0e80_0000u32, &set[0..3]), (0x0e81_0000, &set[3..6])] {
            assert_eq!(
                rows,
                &[
                    vec![parameter, 0, f(2.0), f(4.0), f(3.0), f(7.0)],
                    vec![parameter, 1, f(9.0), f(0.0), f(1.0), 0],
                    vec![parameter, 2, f(-1.0e8), f(2.0), f(3.0), f(10.0)],
                ]
            );
        }
        // 1*4 + 0.5*5 + 0.25*6 = 8, -(1*1 + 0.5*2 + 0.25*3) = -2.75, 120 / 60 = 2.
        assert_eq!(set[6], vec![0x0e81_0000, 3, f(8.0), f(-2.75), f(2.0), 0]);
        assert_eq!(e.global::<f32>(RESULT_012003C4), 3.0);
        // Mode 0 gives both flags; a divisor of 0 (or NaN) no constant 3.
        e.mem.set_u32(this + 0x74, 0);
        e.set_global::<f32>(DIVISOR_011F9410, 0.0);
        e.call(0x00bd_66c0, &args![this, 0u32]);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 13);
        assert_eq!(set[8][2..], [f(9.0), f(1.0), f(1.0), 0]);
        e.mem.set_u32(this + 0x74, 1);
        e.set_global::<f32>(DIVISOR_011F9410, f32::NAN);
        e.call(0x00bd_66c0, &args![this, 0u32]);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 19);
        assert_eq!(set[14][2..], [f(9.0), f(1.0), f(0.0), 0]);
    }

    #[test]
    fn fn_00bd6bb0_builds_the_four_passes() {
        let mut e = engine();
        // The renderer: slot +0xc8 gives an object whose +0x8c answers 0x400, +0x90 0x80000000.
        let slots: Vec<u32> = (0..0x40u32).map(|i| 0x0e91_0000 + 4 * i).collect();
        e.put_vtable(0x0e90_0000, &slots);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, 0x0e90_0000);
        e.set_global::<u32>(RENDERER_011F9508, renderer);
        e.register_double(0x0e91_0000 + 0xc8, move |_, _| renderer.into_ret());
        e.register(0x0e91_0000 + 0x8c, |_, _| 0x400u32.into_ret());
        e.register(0x0e91_0000 + 0x90, |_, _| 0x8000_0000u32.into_ret());
        e.set_global::<f32>(TWO_TO_32_01099378, 4_294_967_296.0);
        e.set_global::<f32>(CONSTANT_01013974, 1000.0);
        e.set_global::<f32>(CONSTANT_01013D84, 500.0);
        e.set_global::<f32>(CONSTANT_012023E0, 7.0);
        let this = effect(&mut e);
        let owner = e.mem.alloc(0xc);
        let textures = e.mem.alloc(0x108);
        e.mem.set_u32(owner + 8, textures);
        for offset in [0x48u32, 0xcc, 0xd0, 0x104] {
            e.mem.set_u32(textures + offset, 0x0ea0_0000 + offset);
        }
        e.call(0x00bd_6bb0, &args![this, owner, 0u32]);
        assert_eq!(calls(&e, EFFECT_SET_UP_00BA3F90), vec![vec![this, 4]]);
        let parameters: Vec<u32> = (0..4).map(|i| element(&e, this, 0x1c, i)).collect();
        assert_eq!(parameters[1], 0);
        let f = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![
                vec![
                    parameters[0],
                    2,
                    f(1.0 / 1024.0),
                    f(1.0 / 2_147_483_648.0),
                    0,
                    0
                ],
                vec![parameters[2], 0, f(1.0), f(1000.0), f(500.0), f(7.0)],
                vec![parameters[3], 0, f(1.0), f(1000.0), f(500.0), f(7.0)],
            ]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![
                vec![parameters[0], 0, 0x18],
                vec![parameters[2], 0, 3],
                vec![parameters[3], 0, 4]
            ]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_CALL_00B8A8B0),
            vec![vec![parameters[0], 3, DATA_011ADD48, 0x40]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![
                vec![this, 0, 0x0ea0_0104, parameters[0], 2],
                vec![this, 1, 0x0ea0_0048, 0, 2],
                vec![this, 2, 0x0ea0_00cc, parameters[2], 0],
                vec![this, 3, 0x0ea0_00d0, parameters[3], 0],
            ]
        );
        assert_eq!(calls(&e, EFFECT_CALL_00BA3B10), vec![vec![this, 0, 1, 1]]);
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3B50),
            vec![vec![this, 1, 2, 3, 0, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3BA0),
            vec![vec![this, 2, 1, 2, 4, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C00),
            vec![vec![this, 3, 1, 2, 4, 5, 0, 1, 0, 0]]
        );
    }

    #[test]
    fn fn_00bd71e0_destroys_the_effect() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let this = effect(&mut e);
        e.mem.set_u32(this + 0x5c, 0x0e57_0000);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let elements = e.mem.u32(this + 0x1c);
        e.mem.set_u32(elements, parameter);
        e.mem.set_u32(elements + 0xc, parameter);
        assert_eq!(e.call(0x00bd_71e0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(
            calls(&e, 0x0e74_0000),
            vec![vec![parameter, 1], vec![parameter, 1]]
        );
        let cleared: Vec<u32> = calls(&e, ARRAY_SET_00BA3AA0).iter().map(|a| a[1]).collect();
        assert_eq!(cleared, vec![0, 3]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![vec![this + 0x8c, 0x10, 3, TEXTURE_DESTRUCT_00BA3A00]]
        );
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![this + 0x7c]]
        );
        assert_eq!(e.mem.u32(this + 0x58), ARRAY_VTABLE_010B9D24);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![0x0e57_0000]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        e.call(0x00bd_71e0, &args![this, 0u32]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
    }
}
