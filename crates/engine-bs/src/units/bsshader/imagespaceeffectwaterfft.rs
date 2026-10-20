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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_15e0, fn_00ba15e0(Ptr) -> Ptr)]
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
    fn fn_00ba15e0_large_grid() {
        let (e, _) = construct(1);
        assert_eq!(e.global::<u32>(GRID_SIZE_01200054), 0x100);
        assert_eq!(e.global::<u32>(GRID_SHIFT_01200050), 8);
        assert_eq!(e.global::<f32>(INVERSE_SIZE_0120004C), 1.0 / 256.0);
    }
}
