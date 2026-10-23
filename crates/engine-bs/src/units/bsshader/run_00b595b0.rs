//! `BSShader/run_00b595b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The vtables the two constructors store (`010adc48`, `010adc68`; no RTTI locator before
/// them).
const VTABLE_010ADC48: u32 = 0x010a_dc48;
const VTABLE_010ADC68: u32 = 0x010a_dc68;
/// The `float` constants they copy: 0.5 (`01016248`), 1000.0 (`01013974`), the word at
/// `012023e0` and 600.0 (`0101786c`) (values read in Ghidra).
const HALF_01016248: u32 = 0x0101_6248;
const CONSTANT_01013974: u32 = 0x0101_3974;
const CONSTANT_012023E0: u32 = 0x0120_23e0;
const CONSTANT_0101786C: u32 = 0x0101_786c;
/// The bits of 1.0 (`FLD1`).
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b5a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc48`, sizes
/// the array at +0x28 to 2 (`00a90180`) and copies the `float` at `01016248` (`FLD`/`FST`, a
/// signalling NaN quieted) to +0x6c and +0x70. Answers `this`.
// Translation incomplete: 00b5a410 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b5a410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC48);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.mem.set_u32(base + 0x6c, half);
    e.mem.set_u32(base + 0x70, half);
    this
}

// Translated from 00b5a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc68`, stores
/// 0.0 at +0x5c to +0x98, clears the bytes +0xa4 to +0xa6 and the words +0xa8 to +0xb0,
/// constructs the two 16-byte textures at +0xb4 (`eh vector constructor iterator` with
/// `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 4
/// (`00a90180`), then stores (through the FPU, signalling NaNs quieted) 1.0 at +0x7c, the
/// `float` at `01013974` at +0x80, 0.0 at +0x84 and +0x6c, the word at `012023e0` at +0x88,
/// the `float` at `0101786c` at +0x9c, 1.0 at +0x70 and +0x74, 0.0 at +0x78, and clears +0x58.
/// Answers `this`.
// Translation incomplete: 00b5a590 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00b5a590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC68);
    for offset in (0x5cu32..=0x98).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0xa4u32, 0xa5, 0xa6] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0xa8u32, 0xac, 0xb0] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0xb4,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    let thousand = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
    let copied = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
    let six_hundred = x87_quiet_bits(e.global::<u32>(CONSTANT_0101786C));
    e.mem.set_u32(base + 0x7c, ONE_BITS);
    e.mem.set_u32(base + 0x80, thousand);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x6c, 0);
    e.mem.set_u32(base + 0x88, copied);
    e.mem.set_u32(base + 0x9c, six_hundred);
    e.mem.set_u32(base + 0x70, ONE_BITS);
    e.mem.set_u32(base + 0x74, ONE_BITS);
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x58, 0);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_clear, release_reference, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// The two hash maps these functions empty: the getters `00b585c0` and `00b58670` (unnamed,
/// no arguments; each answers a map whose hash size is at +4, bucket array at +8 and count at
/// +0xc), their next-item steps `00b57f20` and `00e6e630` (unnamed; `thiscall` on the map: the
/// position, the key and the value, by address), their `00b601a0` (unnamed, `thiscall`, no
/// stack words: run after the walk), the first map's value destructor `00b595b0` (unnamed,
/// `thiscall`) and `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one
/// word).
const FIRST_MAP_00B585C0: u32 = 0x00b5_85c0;
const SECOND_MAP_00B58670: u32 = 0x00b5_8670;
const FIRST_MAP_STEP_00B57F20: u32 = 0x00b5_7f20;
const SECOND_MAP_STEP_00E6E630: u32 = 0x00e6_e630;
const MAP_AFTER_WALK_00B601A0: u32 = 0x00b6_01a0;
const VALUE_DESTRUCT_00B595B0: u32 = 0x00b5_95b0;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
/// The two words `00b59890` clears (`011f963c`, `011f9640`).
const CLEARED_011F963C: u32 = 0x011f_963c;
const CLEARED_011F9640: u32 = 0x011f_9640;

/// The first item of a map: the first non-null bucket's head, or null.
fn first_position(e: &Engine, map: u32) -> u32 {
    let size = e.mem.u32(map + 4);
    let buckets = e.mem.u32(map + 8);
    for index in 0..size {
        let head = e.mem.u32(buckets.wrapping_add(4 * index));
        if head != 0 {
            return head;
        }
    }
    0
}

// Translated from 00b59890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`), no arguments: empties the map `00b585c0`
/// answers. With items in it (count at +0xc) and a first position, walks it (`00b57f20` with
/// the position, key and value locals, the map asked again each step) and destroys each
/// non-null value (`00b595b0`) and frees it (`MemoryManager::Deallocate`) until the position
/// is null; then `00b601a0` on the map, and clears `011f963c` and `011f9640`.
pub fn fn_00b59890(e: &mut Engine) {
    let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
    let first = first_position(e, map);
    e.with_stack(0xc, |e, locals| {
        // value +0, position +4, key +8 (the exe's frame order).
        let value_slot = locals.addr();
        let position_slot = value_slot + 4;
        let key_slot = value_slot + 8;
        e.mem.set_u32(position_slot, first);
        e.mem.set_u32(value_slot, 0);
        let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
        if e.mem.u32(map + 0xc) != 0 && first != 0 {
            loop {
                let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
                e.call(
                    FIRST_MAP_STEP_00B57F20,
                    &args![map, position_slot, key_slot, value_slot],
                );
                let value = e.mem.u32(value_slot);
                if value != 0 {
                    e.call(VALUE_DESTRUCT_00B595B0, &args![value]);
                    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, value]);
                }
                if e.mem.u32(position_slot) == 0 {
                    break;
                }
            }
        }
    });
    let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
    e.call(MAP_AFTER_WALK_00B601A0, &args![map]);
    e.set_global(CLEARED_011F963C, 0u32);
    e.set_global(CLEARED_011F9640, 0u32);
}

// Translated from 00b59940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b59940 - the C++ exception frame (FS:[0] chain, unwind state 0 for the value pointer) is not translated.
/// Unnamed in the engine map (`run_00b595b0`), no arguments: runs `00b59890`, then empties the
/// map `00b58670` answers. The value local is an `NiPointer`. With items in it and a first
/// position, walks it (`00e6e630`, the map asked again each step) and for each non-null value
/// releases and clears its pointers at +0x08, frees the block at +0x10 (cleared), then
/// releases and clears its pointers at +0x18, +0x20, +0x1c and +0x24, until the position is
/// null; then `00b601a0` on the map (not when the map has no items). Last the value local's
/// reference is released.
pub fn fn_00b59940(e: &mut Engine) {
    fn_00b59890(e);
    let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
    let first = first_position(e, map);
    let mut value = 0u32;
    e.with_stack(0xc, |e, locals| {
        // value +0, position +4, key +8 (the exe's frame order).
        let value_slot = locals.addr();
        let position_slot = value_slot + 4;
        let key_slot = value_slot + 8;
        e.mem.set_u32(position_slot, first);
        e.mem.set_u32(value_slot, 0);
        let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
        if e.mem.u32(map + 0xc) == 0 {
            return;
        }
        if first != 0 {
            loop {
                let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
                e.call(
                    SECOND_MAP_STEP_00E6E630,
                    &args![map, position_slot, key_slot, value_slot],
                );
                value = e.mem.u32(value_slot);
                if value != 0 {
                    ni_pointer_clear(e, value + 8);
                    let block = e.mem.u32(value + 0x10);
                    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, block]);
                    e.mem.set_u32(value + 0x10, 0);
                    for offset in [0x18u32, 0x20, 0x1c, 0x24] {
                        ni_pointer_clear(e, value + offset);
                    }
                }
                if e.mem.u32(position_slot) == 0 {
                    break;
                }
            }
        }
        let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
        e.call(MAP_AFTER_WALK_00B601A0, &args![map]);
    });
    if value != 0 {
        release_reference(e, value);
    }
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word) and `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

/// Effect parameter `index` (`pEffectParamList`, elements at +0x1c), when set, deleted through
/// its slot +0 with 1 (the element tested twice, as the compiler has it) and the element set
/// to null.
fn delete_param(e: &mut Engine, this: u32, index: u32) {
    let element = e.mem.u32(this + 0x1c) + 4 * index;
    if e.mem.u32(element) != 0 {
        let param = e.mem.u32(element);
        if param != 0 {
            e.vcall(param, 0, &args![1u32]);
        }
        set_at(e, this + 0x18, index, 0);
    }
}

// Translated from 00b59c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b59c20 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the destructor of the effect `00b5a410` builds,
/// `thiscall`, no stack words: the vtable `010adc48`, effect parameter 0 deleted, then
/// `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b59c20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADC48);
    delete_param(e, base, 0);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// `ImageSpaceShaderParam::SetPixelConstant` (engine map, `00b8a790`, `thiscall`: index, four
/// floats).
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;

// Translated from 00b59cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the `UpdateParams` slot of the effect
/// `00b5a410` builds, `thiscall`, one word it does not read. Effect parameter 0 takes pixel
/// constant 0 from the `float`s +0x58, +0x5c, +0x60, +0x64 and constant 1 from +0x6c, +0x70,
/// +0x68 and 0.0 (each through `FLD`/`FSTP`, a signalling NaN quieted). Answers true.
pub fn fn_00b59cb0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let param = e.mem.u32(e.mem.u32(base + 0x1c));
    let float = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(base + offset));
    let first = [
        float(e, 0x58),
        float(e, 0x5c),
        float(e, 0x60),
        float(e, 0x64),
    ];
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, first[0], first[1], first[2], first[3]],
    );
    let second = [float(e, 0x6c), float(e, 0x70), float(e, 0x68)];
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 1u32, second[0], second[1], second[2], 0u32],
    );
    true
}

/// The byte both effects' activity needs (`011f968c`).
const ENABLED_011F968C: u32 = 0x011f_968c;

/// 1 when `0.0 < value` (`FLDZ`, `FCOMP`, `TEST AH,5`, `JP`: false for NaN) and the byte
/// `011f968c` is set, else 0.
fn active_when_positive(e: &Engine, value: f32) -> u32 {
    u32::from(0.0 < value && e.global::<u8>(ENABLED_011F968C) != 0)
}

// Translated from 00b59d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): the `IsActive`
/// slot of the effect `00b5a410` builds, `thiscall`, no stack words: 1 when the `float` +0x58
/// is above 0 and the byte `011f968c` is set, else 0.
pub fn fn_00b59d10(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.f32(this.addr() + 0x58);
    active_when_positive(e, value)
}

/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;

// Translated from 00b5a110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a110 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the destructor of the effect `00b5a590` builds,
/// `thiscall`, no stack words. Stores the vtable `010adc68`; a texture 2 of `pTextures` (+0x2c
/// elements) is destroyed, freed and its element set to null (the element tested twice);
/// effect parameters 1 and 0 are deleted; the two textures at +0xb4 are destroyed (`eh vector
/// destructor iterator`), then the texture at +0xa4 and `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b5a110(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADC68);
    let element = e.mem.u32(base + 0x2c) + 8;
    if e.mem.u32(element) != 0 {
        let texture = e.mem.u32(element);
        if texture != 0 {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
            e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, texture]);
        }
        set_at(e, base + 0x28, 2, 0);
    }
    delete_param(e, base, 1);
    delete_param(e, base, 0);
    e.call(
        VECTOR_DESTRUCT_00EC5FCE,
        &args![base + 0xb4, 0x10u32, 2u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0xa4]);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00b5a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): the `IsActive`
/// slot of the effect `00b5a590` builds, `thiscall`, no stack words: 1 when the `float` +0x6c
/// is above 0 and the byte `011f968c` is set, else 0.
pub fn fn_00b5a3f0(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.f32(this.addr() + 0x6c);
    active_when_positive(e, value)
}

// Translated from 00b5a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the scalar deleting destructor of the effect
/// `00b5a410` builds, `thiscall`, one word: `00b59c20`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b5a480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b59c20(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), `ImageSpaceShaderParam`'s
/// constructor (`00b8a9e0`) and `InitConstantMap` (`00b8ab10`, two words), the input setters
/// `00ba3b10` and `00ba3ba0` and the effect setter `00ba3c70` (unit `run_00ba3aa0`), the
/// manager's effect index (`011f9698`) and the `float` constants the parameters take
/// (`0101e2bc`, `01013974`, `01013d84`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_INPUT_00BA3B10: u32 = 0x00ba_3b10;
const SET_INPUTS_00BA3BA0: u32 = 0x00ba_3ba0;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
const EFFECT_INDEX_011F9698: u32 = 0x011f_9698;
const CONSTANT_0101E2BC: u32 = 0x0101_e2bc;
const CONSTANT_01013D84: u32 = 0x0101_3d84;

/// A parameter of 0x88 bytes from the memory manager (`ImageSpaceShaderParam`'s constructor
/// runs only on a block, else the parameter is null) with `InitConstantMap(0, map)`.
fn new_param(e: &mut Engine, map: u32) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let param = if block != 0 {
        e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
    } else {
        0
    };
    e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 0u32, map]);
    param
}

/// The manager's effect picked by `011f9698` (its +8 list, +0xd8 onwards).
fn indexed_effect(e: &Engine, manager: u32) -> u32 {
    let index = e.global::<u32>(EFFECT_INDEX_011F9698);
    let list = e.mem.u32(manager + 8);
    e.mem
        .u32(list.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xd8))
}

// Translated from 00b5a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a4a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter allocation) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the `Setup` slot of the effect `00b5a410`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 1
/// (`00ba3f90`); one parameter (`InitConstantMap(0, 2)`, pixel constant 0 = the `float` at
/// `0101e2bc` (through `FLD`/`FSTP`), 0, 0, 0); effect 0 is the manager's effect picked by
/// `011f9698`; the parameter is stored as 0 and input 0 is texture 1 (filter mode 0).
pub fn fn_00b5a4a0(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 1u32]);
    let param = new_param(e, 2);
    let scale = x87_quiet_bits(e.global::<u32>(CONSTANT_0101E2BC));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, scale, 0u32, 0u32, 0u32],
    );
    let effect = indexed_effect(e, manager);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, param, 0u32]);
    set_at(e, base + 0x18, 0, param);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
}

// Translated from 00b5a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the scalar deleting destructor of the effect
/// `00b5a590` builds, `thiscall`, one word: `00b5a110`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b5a700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b5a110(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b5a720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a720 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the `Setup` slot of the effect `00b5a590`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 2
/// (`00ba3f90`). Parameter 0 (`InitConstantMap(0, 2)`, pixel constant 0 = `0101e2bc`, 0, 0, 0)
/// goes with the manager's effect picked by `011f9698` to effect 0 (output 2), input 0 is
/// texture 1 (filter 1). Parameter 1 (`InitConstantMap(0, 3)`, pixel constant 0 = 1.0 and the
/// `float`s at `01013974`, `01013d84`, `012023e0`, byte +0x56 cleared) is stored as 1, input 1
/// is textures 1, 2, 3 (filters 0, 1, 0) and effect 1 is the manager's effect at +0xcc.
pub fn fn_00b5a720(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 2u32]);
    let first = new_param(e, 2);
    let scale = x87_quiet_bits(e.global::<u32>(CONSTANT_0101E2BC));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![first, 0u32, scale, 0u32, 0u32, 0u32],
    );
    let effect = indexed_effect(e, manager);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, first, 2u32]);
    set_at(e, base + 0x18, 0, first);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 1u32]);
    let second = new_param(e, 3);
    let last = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
    let middle = x87_quiet_bits(e.global::<u32>(CONSTANT_01013D84));
    let thousand = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![second, 0u32, ONE_BITS, thousand, middle, last],
    );
    e.mem.set_u8(second + 0x56, 0);
    set_at(e, base + 0x18, 1, second);
    e.call(
        SET_INPUTS_00BA3BA0,
        &args![base, 1u32, 1u32, 2u32, 3u32, 0u32, 1u32, 0u32],
    );
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 0xcc);
    e.call(
        SET_EFFECT_00BA3C70,
        &args![base, 1u32, effect, second, 0u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_a410, fn_00b5a410(Ptr) -> Ptr),
        entry!(0x00b5_a590, fn_00b5a590(Ptr) -> Ptr),
        entry!(0x00b5_9890, fn_00b59890()),
        entry!(0x00b5_9940, fn_00b59940()),
        entry!(0x00b5_9c20, fn_00b59c20(Ptr)),
        entry!(0x00b5_9cb0, fn_00b59cb0(Ptr, u32) -> bool),
        entry!(0x00b5_9d10, fn_00b59d10(Ptr) -> u32),
        entry!(0x00b5_a110, fn_00b5a110(Ptr)),
        entry!(0x00b5_a3f0, fn_00b5a3f0(Ptr) -> u32),
        entry!(0x00b5_a480, fn_00b5a480(Ptr, u32) -> Ptr),
        entry!(0x00b5_a4a0, fn_00b5a4a0(Ptr, u32, u32)),
        entry!(0x00b5_a700, fn_00b5a700(Ptr, u32) -> Ptr),
        entry!(0x00b5_a720, fn_00b5a720(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the base constructor, the set-size and the vector constructor doubled
    /// and the constants mapped (a signalling NaN at `012023e0`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        for page in [0x0101_6000u32, 0x0101_3000, 0x0120_2000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(CONSTANT_01013974, 1000.0f32);
        e.set_global(CONSTANT_012023E0, 0x7f80_0001u32);
        e.set_global(CONSTANT_0101786C, 600.0f32);
        e.call_log = Some(vec![]);
        e
    }

    fn log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    #[test]
    fn fn_00b5a410_builds_the_effect() {
        let mut e = engine();
        e.set_global(HALF_01016248, 0x7fa0_0000u32);
        let this = e.mem.alloc(0x74);
        assert_eq!(e.call(0x00b5_a410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC48);
        assert_eq!(e.mem.u32(this + 0x6c), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0x70), 0x7fe0_0000);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 2]),
            ]
        );
    }

    #[test]
    fn fn_00b5a590_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0xd4);
        e.mem.write(this, &[0xcc; 0xd4]);
        assert_eq!(e.call(0x00b5_a590, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC68);
        let floats = |e: &Engine, offset: u32| e.mem.u32(this + offset);
        assert_eq!(floats(&e, 0x58), 0);
        assert_eq!(floats(&e, 0x5c), 0);
        assert_eq!(floats(&e, 0x6c), 0);
        assert_eq!(floats(&e, 0x70), ONE_BITS);
        assert_eq!(floats(&e, 0x74), ONE_BITS);
        assert_eq!(floats(&e, 0x78), 0);
        assert_eq!(floats(&e, 0x7c), ONE_BITS);
        assert_eq!(floats(&e, 0x80), 1000.0f32.to_bits());
        assert_eq!(floats(&e, 0x84), 0);
        assert_eq!(floats(&e, 0x88), 0x7fc0_0001);
        assert_eq!(floats(&e, 0x8c), 0);
        assert_eq!(floats(&e, 0x98), 0);
        assert_eq!(floats(&e, 0x9c), 600.0f32.to_bits());
        assert_eq!(floats(&e, 0xa0), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 0xa4, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0xa7), 0xcc);
        assert_eq!(e.mem.bytes(this + 0xa8, 0xc), vec![0; 0xc]);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0xb4,
                        0x10,
                        2,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// A map of `size` buckets with heads at the given indexes (heads are just non-null words)
    /// and the count `count`.
    fn map(e: &mut Engine, size: u32, heads: &[(u32, u32)], count: u32) -> u32 {
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(4 * size.max(1));
        for (index, head) in heads {
            e.mem.set_u32(buckets + 4 * index, *head);
        }
        e.mem.set_u32(map + 4, size);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(map + 0xc, count);
        map
    }

    /// The next-item double: hands out `values` in turn, the position null with the last.
    fn walker(e: &mut Engine, next: u32, values: Vec<u32>) {
        let mut index = 0usize;
        e.register_double(next, move |e, a| {
            e.mem.set_u32(a[3], values[index]);
            index += 1;
            let position = if index < values.len() { 1 } else { 0 };
            e.mem.set_u32(a[1], position);
            Ret::default()
        });
    }

    fn first_rig(heads: &[(u32, u32)], count: u32, values: Vec<u32>) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(CLEARED_011F963C, 7u32);
        e.set_global(CLEARED_011F9640, 8u32);
        let first = map(&mut e, 4, heads, count);
        e.register_double(FIRST_MAP_00B585C0, move |_, _| first.into_ret());
        walker(&mut e, FIRST_MAP_STEP_00B57F20, values);
        for callee in [
            MAP_AFTER_WALK_00B601A0,
            VALUE_DESTRUCT_00B595B0,
            MEMORY_DEALLOCATE_00AA4060,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        (e, first)
    }

    #[test]
    fn fn_00b59890_destroys_and_frees_every_value() {
        let (mut e, first) = first_rig(&[(2, 0x99)], 3, vec![0x1000, 0, 0x3000]);
        e.call(0x00b5_9890, &args![]);
        let walks = calls(&e, FIRST_MAP_STEP_00B57F20);
        assert_eq!(walks.len(), 3);
        assert_eq!(walks[0][0], first);
        assert_eq!(walks[0][1], walks[0][3] + 4);
        assert_eq!(walks[0][2], walks[0][3] + 8);
        assert_eq!(
            calls(&e, VALUE_DESTRUCT_00B595B0),
            vec![vec![0x1000], vec![0x3000]]
        );
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x1000], vec![MEMORY_MANAGER, 0x3000]]
        );
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0), vec![vec![first]]);
        assert_eq!(e.global::<u32>(CLEARED_011F963C), 0);
        assert_eq!(e.global::<u32>(CLEARED_011F9640), 0);
    }

    #[test]
    fn fn_00b59890_skips_an_empty_map_but_still_finishes() {
        let (mut e, first) = first_rig(&[], 5, vec![]);
        e.call(0x00b5_9890, &args![]);
        assert!(calls(&e, FIRST_MAP_STEP_00B57F20).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0), vec![vec![first]]);
        let (mut e, _) = first_rig(&[(0, 0x99)], 0, vec![]);
        e.call(0x00b5_9890, &args![]);
        assert!(calls(&e, FIRST_MAP_STEP_00B57F20).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 1);
    }

    /// The second map's walk over `values` (after an empty first map).
    fn second_rig(heads: &[(u32, u32)], count: u32, values: Vec<u32>) -> (Engine, u32) {
        let (mut e, _) = first_rig(&[], 0, vec![]);
        let second = map(&mut e, 2, heads, count);
        e.register_double(SECOND_MAP_00B58670, move |_, _| second.into_ret());
        walker(&mut e, SECOND_MAP_STEP_00E6E630, values);
        (e, second)
    }

    #[test]
    fn fn_00b59940_clears_each_value_and_releases_the_last() {
        let (mut e, second) = second_rig(&[(1, 0x99)], 1, vec![]);
        let value = object(&mut e, 1);
        let members: Vec<u32> = (0..5).map(|_| object(&mut e, 1)).collect();
        for (offset, member) in [0x08u32, 0x18, 0x20, 0x1c, 0x24].iter().zip(&members) {
            e.mem.set_u32(value + offset, *member);
        }
        e.mem.set_u32(value + 0x10, 0x5555);
        walker(&mut e, SECOND_MAP_STEP_00E6E630, vec![value]);
        e.call(0x00b5_9940, &args![]);
        // 00b59890 ran first (its map's after-walk), then this map's.
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 2);
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0)[1], vec![second]);
        let deleted = calls(&e, REFERENCE_DELETE);
        let expected: Vec<Vec<u32>> = [0usize, 1, 2, 3, 4]
            .iter()
            .map(|i| vec![members[*i]])
            .chain([vec![value]])
            .collect();
        assert_eq!(deleted, expected);
        for offset in [0x08u32, 0x10, 0x18, 0x1c, 0x20, 0x24] {
            assert_eq!(e.mem.u32(value + offset), 0);
        }
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x5555]]
        );
    }

    #[test]
    fn fn_00b59940_with_no_items_skips_the_after_walk() {
        let (mut e, _) = second_rig(&[(0, 0x99)], 0, vec![]);
        e.call(0x00b5_9940, &args![]);
        assert!(calls(&e, SECOND_MAP_STEP_00E6E630).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 1);
        // Items but no first position: the after-walk runs.
        let (mut e, second) = second_rig(&[], 2, vec![]);
        e.call(0x00b5_9940, &args![]);
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0)[1], vec![second]);
    }

    const PARAM_DELETE: u32 = 0x0e0a_0000;
    const PARAM_VTABLE: u32 = 0x0e0b_0000;

    /// An effect whose parameters 0 and 1 are deletable objects (when `params` is true).
    fn effect_rig(params: bool) -> (Engine, u32, [u32; 2]) {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            TEXTURE_DESTRUCT_00BA3A00,
            VECTOR_DESTRUCT_00EC5FCE,
            PARAM_DELETE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(PARAM_VTABLE, &[PARAM_DELETE]);
        let this = e.mem.alloc(0xd4);
        let list = e.mem.alloc(8);
        let mut made = [0u32; 2];
        if params {
            for (index, slot) in made.iter_mut().enumerate() {
                let param = e.mem.alloc(8);
                e.mem.set_u32(param, PARAM_VTABLE);
                e.mem.set_u32(list + 4 * index as u32, param);
                *slot = param;
            }
        }
        e.mem.set_u32(this + 0x1c, list);
        let textures = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x2c, textures);
        (e, this, made)
    }

    fn set_ats(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect()
    }

    #[test]
    fn fn_00b59c20_and_00b5a480_delete_parameter_0() {
        let (mut e, this, params) = effect_rig(true);
        e.call(0x00b5_9c20, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC48);
        assert_eq!(calls(&e, PARAM_DELETE), vec![vec![params[0], 1]]);
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0)]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        let (mut e, this, _) = effect_rig(false);
        assert_eq!(e.call(0x00b5_a480, &args![this, 1u32]).u32(), this);
        assert!(calls(&e, ARRAY_SET_AT_00BA3AA0).is_empty());
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00b5a110_and_00b5a700_free_the_texture_and_both_parameters() {
        let (mut e, this, params) = effect_rig(true);
        let texture = e.mem.alloc(0x10);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 8, texture);
        e.call(0x00b5_a110, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC68);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![texture], vec![this + 0xa4]]
        );
        assert_eq!(
            calls(&e, PARAM_DELETE),
            vec![vec![params[1], 1], vec![params[0], 1]]
        );
        assert_eq!(
            set_ats(&e),
            vec![(this + 0x28, 2), (this + 0x18, 1), (this + 0x18, 0)]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![vec![this + 0xb4, 0x10, 2, TEXTURE_DESTRUCT_00BA3A00]]
        );
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        let (mut e, this, _) = effect_rig(false);
        assert_eq!(e.call(0x00b5_a700, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert!(calls(&e, ARRAY_SET_AT_00BA3AA0).is_empty());
    }

    #[test]
    fn fn_00b59cb0_sets_both_constants() {
        let mut e = rig();
        e.register(PARAM_SET_PIXEL_CONSTANT_00B8A790, |_, _| Ret::default());
        let this = e.mem.alloc(0x74);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x0e0c_0000);
        e.mem.set_u32(this + 0x1c, list);
        for (index, offset) in (0x58u32..=0x70).step_by(4).enumerate() {
            e.mem.set_f32(this + offset, index as f32 + 1.0);
        }
        e.mem.set_u32(this + 0x70, 0x7f80_0003);
        assert!(e.call(0x00b5_9cb0, &args![this, 0u32]).bool());
        let bits = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![0x0e0c_0000, 0, bits(1.0), bits(2.0), bits(3.0), bits(4.0)],
                vec![0x0e0c_0000, 1, bits(6.0), 0x7fc0_0003, bits(5.0), 0],
            ]
        );
    }

    #[test]
    fn fn_00b59d10_and_00b5a3f0_need_a_positive_value_and_the_byte() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let this = e.mem.alloc(0x74);
        e.mem.set_f32(this + 0x58, 0.5);
        e.mem.set_f32(this + 0x6c, 0.5);
        assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 0);
        e.set_global(ENABLED_011F968C, 1u8);
        assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 1);
        assert_eq!(e.call(0x00b5_a3f0, &args![this]).u32(), 1);
        for value in [0.0f32, -1.0, f32::NAN] {
            e.mem.set_f32(this + 0x58, value);
            e.mem.set_f32(this + 0x6c, value);
            assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 0);
            assert_eq!(e.call(0x00b5_a3f0, &args![this]).u32(), 0);
        }
    }

    /// The set-ups' rig: parameters from 0x0e0d_0100 in steps of 0x100, the constants and the
    /// manager with effects at +0xd8 (index 2 picked) and +0xcc.
    fn setup_rig() -> (Engine, u32, u32) {
        let mut e = rig();
        for page in [
            0x0101_e000u32,
            0x0101_3000,
            0x0120_2000,
            0x011f_9000,
            0x0e0d_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.write(0x0e0d_0000, &[0xcc; 0x1000]);
        e.set_global(CONSTANT_0101E2BC, 0.1f32);
        e.set_global(CONSTANT_01013974, 1000.0f32);
        e.set_global(CONSTANT_01013D84, 0.25f32);
        e.set_global(CONSTANT_012023E0, 0x7f80_0001u32);
        e.set_global(EFFECT_INDEX_011F9698, 2u32);
        let mut next = 0x0e0d_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            SET_EFFECT_00BA3C70,
            ARRAY_SET_AT_00BA3AA0,
            SET_INPUT_00BA3B10,
            SET_INPUTS_00BA3BA0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xd4);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x100);
        e.mem.set_u32(effects + 0xd8 + 8, 0x0e0e_0000);
        e.mem.set_u32(effects + 0xcc, 0x0e0f_0000);
        e.mem.set_u32(manager + 8, effects);
        (e, this, manager)
    }

    #[test]
    fn fn_00b5a4a0_sets_up_one_parameter() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00b5_a4a0, &args![this, manager, 0u32]);
        let param = 0x0e0d_0100;
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![param, 0, 2]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![param, 0, 0.1f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![vec![this, 0, 0x0e0e_0000, param, 0]]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0)]);
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 0]]);
    }

    #[test]
    fn fn_00b5a720_sets_up_two_parameters() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00b5_a720, &args![this, manager, 0u32]);
        let (first, second) = (0x0e0d_0100, 0x0e0d_0200);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![first, 0, 2], vec![second, 0, 3]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![first, 0, 0.1f32.to_bits(), 0, 0, 0],
                vec![
                    second,
                    0,
                    ONE_BITS,
                    1000.0f32.to_bits(),
                    0.25f32.to_bits(),
                    0x7fc0_0001
                ],
            ]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e0e_0000, first, 2],
                vec![this, 1, 0x0e0f_0000, second, 0],
            ]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0), (this + 0x18, 1)]);
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 1]]);
        assert_eq!(
            calls(&e, SET_INPUTS_00BA3BA0),
            vec![vec![this, 1, 1, 2, 3, 0, 1, 0]]
        );
        assert_eq!(e.mem.u8(second + 0x56), 0);
        assert_eq!(e.mem.u8(second + 0x55), 0xcc);
        assert_eq!(e.mem.u8(first + 0x56), 0xcc);
    }
}
