//! `BSShader/run_00ba02b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::ni_pointer_clear;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays), the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word) and this effect's vtable
/// (`010b7ec8`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const VTABLE_010B7EC8: u32 = 0x010b_7ec8;
/// The global that keeps the effect (`011ffff8`) and the byte it clears (`011f4ac0`).
const EFFECT_011FFFF8: u32 = 0x011f_fff8;
const FLAG_011F4AC0: u32 = 0x011f_4ac0;
/// The `float` constants it copies (values read in Ghidra: 0.1, 0.6, 0.985, 0.01, 0.4, 0.05).
const CONSTANT_0101E2BC: u32 = 0x0101_e2bc;
const CONSTANT_01018180: u32 = 0x0101_8180;
const CONSTANT_010231A4: u32 = 0x0102_31a4;
const CONSTANT_01013EA4: u32 = 0x0101_3ea4;
const CONSTANT_010231A8: u32 = 0x0102_31a8;
const CONSTANT_010181BC: u32 = 0x0101_81bc;
/// The `float` globals it sets (`011ffff4`, `01200004` to `01200020`).
const PARAMETER_011FFFF4: u32 = 0x011f_fff4;
const FLOATS_01200004: u32 = 0x0120_0004;

// Translated from 00ba02f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba02b0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b7ec8`, clears the `NiPointer` word +0x58, sizes the array at +0x28 to 2 (`00a90180`),
/// keeps the effect in `011ffff8` and clears the byte `011f4ac0`. Then, through the FPU (each
/// constant loaded once, a signalling NaN quieted): 0.0 at +0x5c and +0x60, the `float` at
/// `0101e2bc` at +0x64 and +0x68, 1.0 at +0x6c, 0.0 at +0x70 and `011ffff4`, the `float`s at
/// `0101e2bc`, `01018180`, `010231a4`, `01013ea4`, `010231a8`, `01018180`, `010231a4` and
/// `010181bc` at `01200004` to `01200020`. Finally the `NiPointer` at +0x58 is cleared (its
/// object released when set). Answers `this`.
// Translation incomplete: 00ba02f0 - the C++ exception frame (FS:[0] chain, unwind state 1 for the base and the pointer) is not translated.
pub fn fn_00ba02f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7EC8);
    e.mem.set_u32(base + 0x58, 0);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    e.set_global(EFFECT_011FFFF8, base);
    e.set_global(FLAG_011F4AC0, 0u8);
    let loaded = |e: &Engine, constant: u32| x87_quiet_bits(e.global::<u32>(constant));
    let tenth = loaded(e, CONSTANT_0101E2BC);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, tenth);
    e.mem.set_u32(base + 0x68, tenth);
    e.mem.set_u32(base + 0x6c, 0x3f80_0000);
    e.mem.set_u32(base + 0x70, 0);
    e.set_global(PARAMETER_011FFFF4, 0u32);
    let second = loaded(e, CONSTANT_01018180);
    let third = loaded(e, CONSTANT_010231A4);
    let words = [
        tenth,
        second,
        third,
        loaded(e, CONSTANT_01013EA4),
        loaded(e, CONSTANT_010231A8),
        second,
        third,
        loaded(e, CONSTANT_010181BC),
    ];
    for (index, word) in words.iter().enumerate() {
        e.set_global(FLOATS_01200004 + 4 * index as u32, *word);
    }
    ni_pointer_clear(e, base + 0x58);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_02f0, fn_00ba02f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba02f0_builds_the_effect_and_its_globals() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        for page in [
            0x0101_e000u32,
            0x0101_8000,
            0x0102_3000,
            0x0101_3000,
            0x011f_f000,
            0x0120_0000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        let constants = [
            (CONSTANT_0101E2BC, 0.1f32.to_bits()),
            (CONSTANT_01018180, 0x7f80_0001),
            (CONSTANT_010231A4, 0.985f32.to_bits()),
            (CONSTANT_01013EA4, 0.01f32.to_bits()),
            (CONSTANT_010231A8, 0.4f32.to_bits()),
            (CONSTANT_010181BC, 0.05f32.to_bits()),
        ];
        for (address, bits) in constants {
            e.set_global(address, bits);
        }
        e.set_global(FLAG_011F4AC0, 7u8);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x74);
        e.mem.write(this, &[0xcc; 0x74]);
        assert_eq!(e.call(0x00ba_02f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7EC8);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let tenth = 0.1f32.to_bits();
        let words: Vec<u32> = (0x5cu32..=0x70)
            .step_by(4)
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![0, 0, tenth, tenth, 0x3f80_0000, 0]);
        assert_eq!(e.global::<u32>(EFFECT_011FFFF8), this);
        assert_eq!(e.global::<u8>(FLAG_011F4AC0), 0);
        assert_eq!(e.global::<u32>(PARAMETER_011FFFF4), 0);
        let globals: Vec<u32> = (0..8)
            .map(|i| e.global::<u32>(FLOATS_01200004 + 4 * i))
            .collect();
        assert_eq!(
            globals,
            vec![
                tenth,
                0x7fc0_0001,
                0.985f32.to_bits(),
                0.01f32.to_bits(),
                0.4f32.to_bits(),
                0x7fc0_0001,
                0.985f32.to_bits(),
                0.05f32.to_bits()
            ]
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 2]),
            ]
        );
    }
}
