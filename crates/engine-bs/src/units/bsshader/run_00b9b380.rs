//! `BSShader/run_00b9b380` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010b7e6c`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010B7E6C: u32 = 0x010b_7e6c;
/// The global that keeps the effect (`011ffe44`), the 256-byte table it copies twice
/// (`011ad900`) and the fifteen `float`s it clears (`011ffe4c` to `011ffe94`, three of every
/// four).
const EFFECT_011FFE44: u32 = 0x011f_fe44;
const TABLE_011AD900: u32 = 0x011a_d900;
const FLOATS_011FFE4C: u32 = 0x011f_fe4c;

// Translated from 00b9b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b7e6c`, clears four 16-byte blocks at +0x58, +0x68, +0x78 and +0x88 (three bytes, then
/// three words each), keeps the effect in `011ffe44`, clears the byte +0x2b8, copies the 256
/// bytes at `011ad900` to +0x198 and to +0x98 (byte by byte, both copies per byte) and stores
/// 0.0 in the `float`s `011ffe4c`, `011ffe50`, `011ffe54`, `011ffe5c` .. `011ffe94` (three of
/// each four words). Answers `this`.
pub fn fn_00b9b830(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7E6C);
    for block in [0x58u32, 0x68, 0x78, 0x88] {
        for byte in 0..3u32 {
            e.mem.set_u8(base + block + byte, 0);
        }
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(base + block + word, 0);
        }
    }
    e.set_global(EFFECT_011FFE44, base);
    e.mem.set_u8(base + 0x2b8, 0);
    for index in 0..0x100u32 {
        let byte = e.mem.u8(TABLE_011AD900 + index);
        e.mem.set_u8(base + 0x198 + index, byte);
        e.mem.set_u8(base + 0x98 + index, byte);
    }
    for group in 0..5u32 {
        for word in 0..3u32 {
            e.mem.set_u32(FLOATS_011FFE4C + 0x10 * group + 4 * word, 0);
        }
    }
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The destructor `00b9aa80` (unit `bsbatchrenderer.cpp`, unnamed, `thiscall`) and the sized
/// free `00aa1460` (unnamed, `cdecl`: the block, its size).
const DESTRUCT_00B9AA80: u32 = 0x00b9_aa80;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00b9b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): a scalar deleting destructor, `thiscall`, one
/// word: the destructor `00b9aa80`, then the 0x98-byte block freed (`00aa1460`) when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b9b380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00B9AA80, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x98u32]);
    }
    this
}

/// `00b9abc0` (unit `bsbatchrenderer.cpp`, unnamed; `thiscall`, two words).
const BATCH_00B9ABC0: u32 = 0x00b9_abc0;

// Translated from 00b9b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, three words: `00b9abc0` on the
/// object `index` of the array at +0x3c with the first and the third word; answers its
/// result.
pub fn fn_00b9b3b0(e: &mut Engine, this: Ptr, value: u32, index: u32, extra: u32) -> u32 {
    let object = e.mem.u32(
        this.addr()
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(0x3c),
    );
    e.call(BATCH_00B9ABC0, &args![object, value, extra]).u32()
}

// Translated from 00b9b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, two words: `00b9abc0` with the
/// first word and 0 on the object `index` of the array at +0x90 when the word +0x34 is above 0
/// (signed), else of the array at +0x70; answers its result.
pub fn fn_00b9b3d0(e: &mut Engine, this: Ptr, value: u32, index: u32) -> u32 {
    let base = this.addr();
    let array = if (e.mem.u32(base + 0x34) as i32) > 0 {
        0x90
    } else {
        0x70
    };
    let object = e
        .mem
        .u32(base.wrapping_add(index.wrapping_mul(4)).wrapping_add(array));
    e.call(BATCH_00B9ABC0, &args![object, value, 0u32]).u32()
}

// Translated from 00b9b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, one word (a seed): the start is
/// `(seed * 0x5391313d + 0x22fc2945) >> 24`; byte `i` of the two 256-byte tables at +0x198 and
/// +0x98 becomes byte `(start + i) & 0xff` of the table at `011ad900`.
pub fn fn_00b9b400(e: &mut Engine, this: Ptr, seed: u32) {
    let start = seed.wrapping_mul(0x5391_313d).wrapping_add(0x22fc_2945) >> 24;
    for index in 0..0x100u32 {
        let byte = e.mem.u8(TABLE_011AD900 + ((index + start) & 0xff));
        e.mem.set_u8(this.addr() + 0x198 + index, byte);
        e.mem.set_u8(this.addr() + 0x98 + index, byte);
    }
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`),
/// `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`) and `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on the manager, one word) (engine map).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00b9b450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9b450 - the C++ exception frame (FS:[0] chain, unwind states 4 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (`run_00b9b380`): the destructor of the effect `00b9b830` builds,
/// `thiscall`, no stack words. Stores the vtable `010b7e6c`; effect parameters 0 and 1 (+0x1c
/// elements), when set, are deleted through their slot +0 with 1 (each tested twice) and
/// their elements set to null; the textures at +0x88, +0x78, +0x68 and +0x58 are destroyed;
/// then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b9b450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B7E6C);
    for index in 0..2u32 {
        let element = e.mem.u32(base + 0x1c) + 4 * index;
        if e.mem.u32(element) != 0 {
            let param = e.mem.u32(element);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    for offset in [0x88u32, 0x78, 0x68, 0x58] {
        e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + offset]);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00b9b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): the scalar deleting destructor of the effect
/// `00b9b830` builds, `thiscall`, one word: `00b9b450`, then the memory freed
/// (`MemoryManager::Deallocate`) when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00b9b930(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b9b450(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), the `NiTArray` set-size
/// `00a90180`, `ImageSpaceShaderParam`'s constructor (`00b8a9e0`) and `InitConstantMap`
/// (`00b8ab10`, two words), the effect setter `00ba3c70` (unit `run_00ba3aa0`) and the
/// `float` seed `011f9410`.
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
const SEED_011F9410: u32 = 0x011f_9410;

// Translated from 00b9b950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9b950 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00b9b380`): the `Setup` slot of the effect `00b9b830`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 2
/// (`00ba3f90`) and `pTextures` to 4. The first time (byte +0x2b8 clear) the tables are filled
/// (`00b9b400`) from the `float` seed `011f9410` truncated (`FISTP qword` with the rounding
/// set to chop; its low word) and the byte set. Then two parameters (0x88 bytes; the
/// constructor only on a block) with `InitConstantMap(3, 6)` and the byte +0x55 cleared,
/// stored at 0 and 1 and given with the manager's effects at +0x1c8 and +0x1cc to effects 0
/// and 1 (output 0).
pub fn fn_00b9b950(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 2u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    if e.mem.u8(base + 0x2b8) == 0 {
        let seed = X87Ext::load32(e.global::<u32>(SEED_011F9410));
        let seed = x87_fistp64_chop(seed) as u32;
        fn_00b9b400(e, this, seed);
        e.mem.set_u8(base + 0x2b8, 1);
    }
    for index in 0..2u32 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 3u32, 6u32]);
        e.mem.set_u8(param + 0x55, 0);
        set_at(e, base + 0x18, index, param);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 0x1c8 + 4 * index);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, index, effect, param, 0u32],
        );
    }
}

/// The `float` divisor `011ad834` and the `double` floor `01012060`.
const DIVISOR_011AD834: u32 = 0x011a_d834;
const FLOOR_01012060: u32 = 0x0101_2060;

// Translated from 00b9bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, two words (a `float`, a byte).
/// Nothing when the `float` equals +0xd8 (`FUCOMP`; a NaN never does). Else with the byte set
/// +0xdc becomes 1.0; without it `q` = +0xdc / `[011ad834]` (stored as a `float`), 0 when below
/// the `double` at `01012060`, at most 1.0 (`FCOM`; a NaN kept), and +0xdc becomes
/// `[011ad834] * (1 - q)`. The `float` is stored at +0xd8 (through the FPU, a signalling NaN
/// quieted).
pub fn fn_00b9bb10(e: &mut Engine, this: Ptr, value: u32, reset: u8) {
    let base = this.addr();
    let current = x87_load32(e.mem.u32(base + 0xd8));
    let value = x87_load32(value);
    if value == current {
        return;
    }
    let time = if reset != 0 {
        0x3f80_0000
    } else {
        let previous = x87_load32(e.mem.u32(base + 0xdc));
        let divisor = x87_load32(e.global::<u32>(DIVISOR_011AD834));
        let ratio = x87_load32(x87_store32(x87_div(previous, divisor)));
        let floor = e.global::<f64>(FLOOR_01012060);
        let ratio = if ratio < floor { 0.0 } else { ratio };
        let ratio = x87_store32(ratio);
        let clamped = if 1.0 < x87_load32(ratio) {
            0x3f80_0000
        } else {
            ratio
        };
        x87_store32(x87_mul(divisor, x87_sub(1.0, x87_m32(clamped))))
    };
    e.mem.set_u32(base + 0xdc, time);
    e.mem.set_u32(base + 0xd8, x87_store32(value));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_b830, fn_00b9b830(Ptr) -> Ptr),
        entry!(0x00b9_b380, fn_00b9b380(Ptr, u32) -> Ptr),
        entry!(0x00b9_b3b0, fn_00b9b3b0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00b9_b3d0, fn_00b9b3d0(Ptr, u32, u32) -> u32),
        entry!(0x00b9_b400, fn_00b9b400(Ptr, u32)),
        entry!(0x00b9_b450, fn_00b9b450(Ptr)),
        entry!(0x00b9_b930, fn_00b9b930(Ptr, u32) -> Ptr),
        entry!(0x00b9_b950, fn_00b9b950(Ptr, u32, u32)),
        entry!(0x00b9_bb10, fn_00b9bb10(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b9b830_builds_the_effect_and_its_tables() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let table: Vec<u8> = (0..=255u8).map(|b| b ^ 0x5a).collect();
        e.mem.write(TABLE_011AD900, &table);
        e.mem.write(FLOATS_011FFE4C - 4, &[0xff; 0x50]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2bc);
        e.mem.write(this, &[0xcc; 0x2bc]);
        assert_eq!(e.call(0x00b9_b830, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7E6C);
        for block in [0x58u32, 0x68, 0x78, 0x88] {
            assert_eq!(e.mem.bytes(this + block, 3), vec![0; 3]);
            assert_eq!(e.mem.u8(this + block + 3), 0xcc);
            assert_eq!(e.mem.bytes(this + block + 4, 0xc), vec![0; 0xc]);
        }
        assert_eq!(e.mem.bytes(this + 0x98, 0x100), table);
        assert_eq!(e.mem.bytes(this + 0x198, 0x100), table);
        assert_eq!(e.mem.u8(this + 0x2b8), 0);
        assert_eq!(e.mem.u8(this + 0x2b9), 0xcc);
        assert_eq!(e.global::<u32>(EFFECT_011FFE44), this);
        for group in 0..5u32 {
            let first = FLOATS_011FFE4C + 0x10 * group;
            assert_eq!(e.mem.bytes(first, 0xc), vec![0; 0xc]);
            assert_eq!(e.mem.u32(first - 4), 0xffff_ffff);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EFFECT_CONSTRUCT_00BA4080, vec![this]));
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00b9b380_destroys_and_frees_the_block() {
        let mut e = rig();
        e.register(DESTRUCT_00B9AA80, |_, _| Ret::default());
        e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
        assert_eq!(e.call(0x00b9_b380, &args![0x2000u32, 0u32]).u32(), 0x2000);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.call(0x00b9_b380, &args![0x2000u32, 1u32]).u32(), 0x2000);
        assert_eq!(calls(&e, DESTRUCT_00B9AA80).len(), 2);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![0x2000, 0x98]]);
    }

    #[test]
    fn fn_00b9b3b0_and_00b9b3d0_forward_to_the_indexed_object() {
        let mut e = rig();
        e.register(BATCH_00B9ABC0, |_, a| (a[1] + a[2]).into_ret());
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x3c + 8, 0x3c3c);
        e.mem.set_u32(this + 0x70 + 4, 0x7070);
        e.mem.set_u32(this + 0x90 + 4, 0x9090);
        assert_eq!(
            e.call(0x00b9_b3b0, &args![this, 5u32, 2u32, 7u32]).u32(),
            12
        );
        assert_eq!(calls(&e, BATCH_00B9ABC0), vec![vec![0x3c3c, 5, 7]]);
        assert_eq!(e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]).u32(), 3);
        e.mem.set_u32(this + 0x34, 1);
        e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]);
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]);
        let objects: Vec<u32> = calls(&e, BATCH_00B9ABC0)[1..]
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(objects, vec![0x7070, 0x9090, 0x7070]);
        assert_eq!(calls(&e, BATCH_00B9ABC0)[1], vec![0x7070, 3, 0]);
    }

    #[test]
    fn fn_00b9b400_fills_both_tables_from_the_seed() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        let table: Vec<u8> = (0..=255u8).collect();
        e.mem.write(TABLE_011AD900, &table);
        let this = e.mem.alloc(0x2bc);
        e.call(0x00b9_b400, &args![this, 0u32]);
        // (0 * k + 0x22fc2945) >> 24 = 0x22.
        assert_eq!(e.mem.u8(this + 0x98), 0x22);
        assert_eq!(e.mem.u8(this + 0x198 + 0xff), 0x21);
        e.call(0x00b9_b400, &args![this, 1u32]);
        let start = 0x5391_313du32.wrapping_add(0x22fc_2945) >> 24;
        assert_eq!(e.mem.u8(this + 0x98), start as u8);
        assert_eq!(
            e.mem.bytes(this + 0x98, 0x100),
            e.mem.bytes(this + 0x198, 0x100)
        );
    }

    #[test]
    fn fn_00b9b450_and_00b9b930_delete_parameters_and_textures() {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            TEXTURE_DESTRUCT_00BA3A00,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            0x0e76_0000,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e77_0000, &[0x0e76_0000]);
        let this = e.mem.alloc(0x2bc);
        let list = e.mem.alloc(8);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e77_0000);
        e.mem.set_u32(list + 4, param);
        e.mem.set_u32(this + 0x1c, list);
        assert_eq!(e.call(0x00b9_b930, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7E6C);
        assert_eq!(calls(&e, 0x0e76_0000), vec![vec![param, 1]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 1]);
        let destroyed: Vec<u32> = calls(&e, TEXTURE_DESTRUCT_00BA3A00)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(destroyed, vec![0x88, 0x78, 0x68, 0x58]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00b9b950_fills_the_tables_once_and_sets_up_two_parameters() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0e78_0000, 0x1000);
        let table: Vec<u8> = (0..=255u8).collect();
        e.mem.write(TABLE_011AD900, &table);
        e.set_global(SEED_011F9410, -1.75f32);
        let mut next = 0x0e78_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            ARRAY_SET_AT_00BA3AA0,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x2bc);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x1d0);
        e.mem.set_u32(effects + 0x1c8, 0x0e79_0000);
        e.mem.set_u32(effects + 0x1cc, 0x0e79_0001);
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00b9_b950, &args![this, manager, 0u32]);
        // -1.75 chops to -1 (0xffffffff).
        let start = 0xffff_ffffu32
            .wrapping_mul(0x5391_313d)
            .wrapping_add(0x22fc_2945)
            >> 24;
        assert_eq!(e.mem.u8(this + 0x98), start as u8);
        assert_eq!(e.mem.u8(this + 0x2b8), 1);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 4]]
        );
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![0x0e78_0100, 3, 6], vec![0x0e78_0200, 3, 6]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e79_0000, 0x0e78_0100, 0],
                vec![this, 1, 0x0e79_0001, 0x0e78_0200, 0]
            ]
        );
        // A second set-up keeps the tables.
        e.mem.set_u8(this + 0x98, 0x77);
        e.call(0x00b9_b950, &args![this, manager, 0u32]);
        assert_eq!(e.mem.u8(this + 0x98), 0x77);
    }

    fn fade_rig(current: f32, time: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(DIVISOR_011AD834, 2.0f32);
        e.set_global(FLOOR_01012060, 0.0f64);
        let this = e.mem.alloc(0xe0);
        e.mem.set_f32(this + 0xd8, current);
        e.mem.set_f32(this + 0xdc, time);
        (e, this)
    }

    #[test]
    fn fn_00b9bb10_restarts_or_reverses_the_fade() {
        let (mut e, this) = fade_rig(1.0, 0.5);
        e.call(0x00b9_bb10, &args![this, 1.0f32.to_bits(), 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 0.5);
        e.call(0x00b9_bb10, &args![this, 3.0f32.to_bits(), 1u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 1.0);
        assert_eq!(e.mem.f32(this + 0xd8), 3.0);
        // 0.5 / 2 = 0.25: 2 * (1 - 0.25) = 1.5.
        let (mut e, this) = fade_rig(1.0, 0.5);
        e.call(0x00b9_bb10, &args![this, 0x7f80_0001u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 1.5);
        assert_eq!(e.mem.u32(this + 0xd8), 0x7fc0_0001);
        // A ratio above 1 counts as 1, one below 0 as 0.
        let (mut e, this) = fade_rig(1.0, 6.0);
        e.call(0x00b9_bb10, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 0.0);
        let (mut e, this) = fade_rig(1.0, -6.0);
        e.call(0x00b9_bb10, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 2.0);
    }
}
