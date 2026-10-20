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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_a410, fn_00b5a410(Ptr) -> Ptr),
        entry!(0x00b5_a590, fn_00b5a590(Ptr) -> Ptr),
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
