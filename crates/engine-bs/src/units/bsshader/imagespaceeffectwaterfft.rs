//! `BSShader/imagespaceeffectwaterfft.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays), the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word) and this effect's vtable
/// (`010b7f10`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const VTABLE_010B7F10: u32 = 0x010b_7f10;
/// The `float` constants it copies (values read in Ghidra: 5.0, 90.0, 0.5).
const CONSTANT_0101712C: u32 = 0x0101_712c;
const CONSTANT_010249D8: u32 = 0x0102_49d8;
const HALF_01016248: u32 = 0x0101_6248;
/// The effect's globals: the `float`s `0120005c` to `01200068`, the effect itself
/// (`0120006c`), the byte that picks the grid (`01200059`), the grid size (`01200054`) and its
/// base-2 logarithm (`01200050`), the byte `01200058` and the `float` reciprocal of the size
/// (`0120004c`).
const PARAMETER_0120005C: u32 = 0x0120_005c;
const PARAMETER_01200060: u32 = 0x0120_0060;
const PARAMETER_01200064: u32 = 0x0120_0064;
const PARAMETER_01200068: u32 = 0x0120_0068;
const EFFECT_0120006C: u32 = 0x0120_006c;
const LARGE_GRID_01200059: u32 = 0x0120_0059;
const GRID_SIZE_01200054: u32 = 0x0120_0054;
const GRID_SHIFT_01200050: u32 = 0x0120_0050;
const FLAG_01200058: u32 = 0x0120_0058;
const INVERSE_SIZE_0120004C: u32 = 0x0120_004c;

// Translated from 00ba15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT image-space
/// effect's constructor, `thiscall`, no stack words. Runs the base constructor `00ba4080`,
/// stores the vtable `010b7f10`, clears six 16-byte blocks at +0x58 to +0x98 (three bytes,
/// then three words each), sizes the array at +0x28 to 10 (`00a90180`), copies the `float`s at
/// `0101712c`, `010249d8` and `01016248` to `01200068`, `01200064` and `01200060` (through the
/// FPU, a signalling NaN quieted) and 1.0 to `0120005c`, keeps the effect in `0120006c`, stores
/// 0.0 at +0xb0 and +0xb4, sets the grid size and its logarithm to 0x100 and 8 when the byte
/// `01200059` is set, else 0x80 and 7, sets the byte `01200058` and stores `1.0 / size`
/// (`FILD`, `FDIVRP`) at `0120004c`. Answers `this`.
// Translation incomplete: 00ba15e0 - the C++ exception frame (FS:[0] chain, unwind state 5 for the base and the blocks) is not translated.
pub fn fn_00ba15e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7F10);
    for block in (0x58u32..=0x98).step_by(0x10) {
        for byte in 0..3u32 {
            e.mem.set_u8(base + block + byte, 0);
        }
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(base + block + word, 0);
        }
    }
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 10u32]);
    let five = x87_quiet_bits(e.global::<u32>(CONSTANT_0101712C));
    e.set_global(PARAMETER_01200068, five);
    e.set_global(EFFECT_0120006C, base);
    let ninety = x87_quiet_bits(e.global::<u32>(CONSTANT_010249D8));
    e.set_global(PARAMETER_01200064, ninety);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.set_global(PARAMETER_01200060, half);
    e.set_global(PARAMETER_0120005C, 0x3f80_0000u32);
    e.mem.set_u32(base + 0xb0, 0);
    e.mem.set_u32(base + 0xb4, 0);
    if e.global::<u8>(LARGE_GRID_01200059) != 0 {
        e.set_global(GRID_SIZE_01200054, 0x100u32);
        e.set_global(GRID_SHIFT_01200050, 8u32);
    } else {
        e.set_global(GRID_SIZE_01200054, 0x80u32);
        e.set_global(GRID_SHIFT_01200050, 7u32);
    }
    let size = f64::from(e.global::<i32>(GRID_SIZE_01200054));
    e.set_global(FLAG_01200058, 1u8);
    let inverse = x87_store32(x87_div(1.0, size));
    e.set_global(INVERSE_SIZE_0120004C, inverse);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// `ImageSpaceTexture::GetTexture` (`00ba3780`), `::ReturnRenderedTexture` (`00ba39a0`) and
/// the destructor (`00ba3a00`) (unit `imagespacetexture.cpp`, `thiscall`).
const TEXTURE_GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const TEXTURE_RETURN_RENDERED_00BA39A0: u32 = 0x00ba_39a0;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;

// Translated from 00ba1250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectWaterFFT::FreeUpTextureMemory` (Xbox PDB), `thiscall`, no stack words: each
/// of the effect's textures at +0x78, +0x88, +0x58, +0x68 and +0x98 (in that order) that has a
/// texture is returned to the texture manager (`ReturnRenderedTexture`); then the byte +0xb8
/// is cleared.
pub fn image_space_effect_water_fft_free_up_texture_memory(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x78u32, 0x88, 0x58, 0x68, 0x98] {
        let holder = base + offset;
        if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32() != 0 {
            e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![holder]);
        }
    }
    e.mem.set_u8(base + 0xb8, 0);
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`) and
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one word).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
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

// Translated from 00ba12d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba12d0 - the C++ exception frame (FS:[0] chain, unwind states 5 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// destructor, `thiscall`, no stack words. Stores the vtable `010b7f10`; each of the 12 effect
/// parameters (+0x1c elements) that is set is deleted through its slot +0 with 1 (the element
/// tested twice) and its element set to null; the textures at +0x98, +0x88, +0x78, +0x68 and
/// +0x58 are destroyed; then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00ba12d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B7F10);
    for index in 0..0xcu32 {
        let element = e.mem.u32(base + 0x1c).wrapping_add(4 * index);
        if e.mem.u32(element) != 0 {
            let param = e.mem.u32(element);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    for offset in [0x98u32, 0x88, 0x78, 0x68, 0x58] {
        e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + offset]);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00ba1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// scalar deleting destructor, `thiscall`, one word: `00ba12d0`, then the memory freed when
/// bit 0 of `flags` is set. Answers `this`.
pub fn fn_00ba1750(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba12d0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (unnamed; `thiscall`, one word),
/// `ImageSpaceShaderParam`'s constructor (`00b8a9e0`) and `InitConstantMap` (`00b8ab10`, two
/// words), and the effect setter `00ba3c70` (unit `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;

// Translated from 00ba1770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba1770 - the C++ exception frame (FS:[0] chain, unwind states 0 to 6 for the parameter allocations) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// `Setup` slot, `thiscall`, two words (the manager; a word it does not read). Sizes the
/// arrays to 12 (`00ba3f90`) and `pTextures` to 10; then seven parameters (0x88 bytes each
/// from the memory manager; the constructor only on a block) with `InitConstantMap(2, 3)` and
/// the byte +0x55 cleared, parameter `i` stored at `i` and given with the manager's effect at
/// +0x174 + 4 `i` to effect `i` (output 0).
pub fn fn_00ba1770(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 0xcu32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 0xau32]);
    for index in 0..7u32 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 2u32, 3u32]);
        e.mem.set_u8(param + 0x55, 0);
        set_at(e, base + 0x18, index, param);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 0x174 + 4 * index);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, index, effect, param, 0u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_15e0, fn_00ba15e0(Ptr) -> Ptr),
        entry!(
            0x00ba_1250,
            image_space_effect_water_fft_free_up_texture_memory(Ptr)
        ),
        entry!(0x00ba_12d0, fn_00ba12d0(Ptr)),
        entry!(0x00ba_1750, fn_00ba1750(Ptr, u32) -> Ptr),
        entry!(0x00ba_1770, fn_00ba1770(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct(large: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        for page in [0x0101_7000u32, 0x0102_4000, 0x0101_6000, 0x0120_0000] {
            e.map(page, 0x1000);
        }
        e.set_global(CONSTANT_0101712C, 5.0f32);
        e.set_global(CONSTANT_010249D8, 0x7f80_0002u32);
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(LARGE_GRID_01200059, large);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xb8);
        e.mem.write(this, &[0xcc; 0xb8]);
        assert_eq!(e.call(0x00ba_15e0, &args![this]).u32(), this);
        (e, this)
    }

    #[test]
    fn fn_00ba15e0_builds_the_effect() {
        let (mut e, this) = construct(0);
        assert_eq!(e.mem.u32(this), VTABLE_010B7F10);
        for block in (0x58u32..=0x98).step_by(0x10) {
            assert_eq!(e.mem.bytes(this + block, 3), vec![0; 3]);
            assert_eq!(e.mem.u8(this + block + 3), 0xcc);
            assert_eq!(e.mem.bytes(this + block + 4, 0xc), vec![0; 0xc]);
        }
        assert_eq!(e.mem.u32(this + 0xa8), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.global::<f32>(PARAMETER_01200068), 5.0);
        assert_eq!(e.global::<u32>(PARAMETER_01200064), 0x7fc0_0002);
        assert_eq!(e.global::<f32>(PARAMETER_01200060), 0.5);
        assert_eq!(e.global::<f32>(PARAMETER_0120005C), 1.0);
        assert_eq!(e.global::<u32>(EFFECT_0120006C), this);
        assert_eq!(e.global::<u32>(GRID_SIZE_01200054), 0x80);
        assert_eq!(e.global::<u32>(GRID_SHIFT_01200050), 7);
        assert_eq!(e.global::<u8>(FLAG_01200058), 1);
        assert_eq!(e.global::<f32>(INVERSE_SIZE_0120004C), 1.0 / 128.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 10]),
            ]
        );
    }

    #[test]
    fn fn_00ba1250_returns_the_held_textures() {
        let mut e = Engine::new();
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TEXTURE_RETURN_RENDERED_00BA39A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        for offset in [0x88u32, 0x58] {
            e.mem.set_u32(this + offset + 4, 0x77);
        }
        e.mem.set_u8(this + 0xb8, 1);
        e.call(0x00ba_1250, &args![this]);
        let log = e.call_log.take().unwrap();
        let asked: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == TEXTURE_GET_TEXTURE_00BA3780)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(asked, vec![0x78, 0x88, 0x58, 0x68, 0x98]);
        let returned: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == TEXTURE_RETURN_RENDERED_00BA39A0)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(returned, vec![0x88, 0x58]);
        assert_eq!(e.mem.u8(this + 0xb8), 0);
    }

    #[test]
    fn fn_00ba12d0_and_00ba1750_delete_the_parameters_and_textures() {
        let mut e = Engine::new();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            TEXTURE_DESTRUCT_00BA3A00,
            0x0e6e_0000,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e6f_0000, &[0x0e6e_0000]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        let list = e.mem.alloc(0x30);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e6f_0000);
        e.mem.set_u32(list + 4 * 0xb, param);
        e.mem.set_u32(this + 0x1c, list);
        assert_eq!(e.call(0x00ba_1750, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7F10);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(0x0e6e_0000), vec![vec![param, 1]]);
        assert_eq!(of(ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 0xb]);
        let destroyed: Vec<u32> = of(TEXTURE_DESTRUCT_00BA3A00)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(destroyed, vec![0x98, 0x88, 0x78, 0x68, 0x58]);
        assert_eq!(of(EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            of(MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00ba1770_sets_up_seven_parameters() {
        let mut e = Engine::new();
        e.map(0x0e70_0000, 0x1000);
        e.mem.write(0x0e70_0000, &[0xcc; 0x1000]);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let mut next = 0x0e70_0000u32;
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
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x190);
        for index in 0..7u32 {
            e.mem
                .set_u32(effects + 0x174 + 4 * index, 0x0e71_0000 + index);
        }
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00ba_1770, &args![this, manager, 0u32]);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(SIZE_ARRAYS_00BA3F90), vec![vec![this, 0xc]]);
        assert_eq!(of(ARRAY_SET_SIZE_00A90180), vec![vec![this + 0x28, 0xa]]);
        let expected: Vec<Vec<u32>> = (0..7u32)
            .map(|i| vec![this, i, 0x0e71_0000 + i, 0x0e70_0100 + 0x100 * i, 0])
            .collect();
        assert_eq!(of(SET_EFFECT_00BA3C70), expected);
        let maps: Vec<Vec<u32>> = (0..7u32)
            .map(|i| vec![0x0e70_0100 + 0x100 * i, 2, 3])
            .collect();
        assert_eq!(of(PARAM_INIT_CONSTANT_MAP_00B8AB10), maps);
        for i in 0..7u32 {
            let param = 0x0e70_0100 + 0x100 * i;
            assert_eq!(e.mem.u8(param + 0x55), 0);
            assert_eq!(e.mem.u8(param + 0x56), 0xcc);
        }
        let indexes: Vec<u32> = of(ARRAY_SET_AT_00BA3AA0).iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..7).collect::<Vec<u32>>());
    }

    #[test]
    fn fn_00ba15e0_large_grid() {
        let (e, _) = construct(1);
        assert_eq!(e.global::<u32>(GRID_SIZE_01200054), 0x100);
        assert_eq!(e.global::<u32>(GRID_SHIFT_01200050), 8);
        assert_eq!(e.global::<f32>(INVERSE_SIZE_0120004C), 1.0 / 256.0);
    }
}
