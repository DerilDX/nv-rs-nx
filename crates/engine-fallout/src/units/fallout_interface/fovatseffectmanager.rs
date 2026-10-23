//! `fallout/interface/fovatseffectmanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC `FOVATSEffectManager` is larger than the Xbox PDB's (0x1C4
//! bytes); the offsets here are the PC code's.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPointer` constructor (`00633c90`, thiscall: the pointer) and
/// assignment (`0066b0d0`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `` `eh vector constructor iterator' `` (`00ec782f`, stdcall: array,
/// element size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The target destructor the iterator is given (`00800190`).
const TARGET_DESTRUCTOR: u32 = 0x0080_0190;
/// A float setting's value (`00403e20`) and an integer setting's value
/// (`0043d4d0`) (thiscall on the setting: a pointer to the value), and a
/// string setting's text (`00403df0`).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const SETTING_STRING_VALUE: u32 = 0x0040_3df0;
/// The settings: the distortion duration (`011db6bc`), the random part of
/// the next distortion time (`011db75c`) and the scanlines texture
/// (`011db744`).
const DISTORT_DURATION_SETTING: u32 = 0x011d_b6bc;
const DISTORT_INTERVAL_SETTING: u32 = 0x011d_b75c;
const SCANLINES_TEXTURE_SETTING: u32 = 0x011d_b744;
/// The game time in milliseconds (`00825c00`, thiscall on `011f6394`).
const TIME_GLOBALS: u32 = 0x011f_6394;
const GET_MILLISECONDS: u32 = 0x0082_5c00;
/// `rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// `BSShaderManager::GetTexture` (`00b55840`, Xbox PDB name, cdecl: name,
/// 1, out pointer, 0, 0).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// Gamebryo's object allocation (`00aa13e0`) and
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, Xbox PDB name;
/// three words).
const NI_NEW: u32 = 0x00aa_13e0;
const BS_SHADER_ACCUMULATOR_CONSTRUCTOR: u32 = 0x00b6_60d0;
/// Reads the word at `this` (`00559450`) and the step the accumulator is
/// given (`005db4c0`, thiscall: one word).
const READ_WORD: u32 = 0x0055_9450;
const ACCUMULATOR_STEP: u32 = 0x005d_b4c0;

// Translated from 007ffec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOVATSEffectManager::FOVATSEffectManager` (Xbox PDB): constructs the
/// single target (+4) and the 32 targets (+0x14, 8 bytes each), the stencil
/// property (+0x114), the scanlines texture (+0x2C4) and the accumulator
/// (+0x2CC) pointers; clears the occlusion flag (+0x118), sets +0x129, the
/// distortion state (+0x28C 0, +0x290 the duration setting, +0x294 the game
/// time plus `rand() %` the interval setting, +0x298 and +0x284 off), the
/// burst and pulse values (+0x2AC, +0x2B0, +0x2C0), the scanning flags
/// (+0x2B4, +0x2B5) and +0x240; empties the single target, loads the
/// scanlines texture into +0x2C4, resets the mode (+0), the target count
/// (+0xC) and selection (+0x10, -1), makes a new `BSShaderAccumulator`
/// (0x280 bytes; `(0x65, 1, 0x2F7)`) the accumulator and steps it
/// (`005db4c0` with 0), sets +0x2C8 and clears the 16 limb entries
/// (+0x119, +0x16C, +0x12C, +0x1AC). Returns `this`.
// Translation incomplete: 007ffec0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fovats_effect_manager_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    fn_00800170(e, this.byte_add(4));
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![t + 0x14, 8u32, 0x20u32, 0x0080_0170u32, TARGET_DESTRUCTOR],
    );
    for offset in [0x114u32, 0x2c4, 0x2cc] {
        e.call(NI_POINTER_CONSTRUCT, &args![t + offset, 0u32]);
    }
    e.mem.set_u8(t + 0x118, 0);
    e.mem.set_u8(t + 0x129, 1);
    e.mem.set_f32(t + 0x28c, 0.0);
    let duration = e
        .call(SETTING_FLOAT_VALUE, &args![DISTORT_DURATION_SETTING])
        .u32();
    let duration = x87_quiet_bits(e.mem.u32(duration));
    e.mem.set_u32(t + 0x290, duration);
    e.mem.set_u8(t + 0x298, 0);
    e.mem.set_u8(t + 0x284, 0);
    e.mem.set_f32(t + 0x2ac, 0.0);
    e.mem.set_f32(t + 0x2b0, 0.0);
    e.mem.set_f32(t + 0x2c0, 0.0);
    let now = e.call(GET_MILLISECONDS, &args![TIME_GLOBALS]).u32();
    let now = f64::from(now);
    let random = e.call(RAND, &args![]).i32();
    let interval = e
        .call(SETTING_INT_VALUE, &args![DISTORT_INTERVAL_SETTING])
        .u32();
    let part = random % e.mem.i32(interval);
    e.mem
        .set_u32(t + 0x294, x87_store32(x87_add(f64::from(part), now)));
    e.mem.set_u8(t + 0x2b4, 0);
    e.mem.set_u8(t + 0x2b5, 0);
    e.mem.set_u32(t + 0x240, 0);
    e.mem.set_u32(t + 4, 0);
    e.call(NI_POINTER_ASSIGN, &args![t + 8, 0u32]);
    let name = e
        .call(SETTING_STRING_VALUE, &args![SCANLINES_TEXTURE_SETTING])
        .u32();
    e.call(
        BS_SHADER_MANAGER_GET_TEXTURE,
        &args![name, 1u32, t + 0x2c4, 0u32, 0u32],
    );
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 0x0c, 0);
    e.mem.set_i32(t + 0x10, -1);
    let block = e.call(NI_NEW, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            BS_SHADER_ACCUMULATOR_CONSTRUCTOR,
            &args![block, 0x65u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![t + 0x2cc, accumulator]);
    let accumulator = e.call(READ_WORD, &args![t + 0x2cc]).u32();
    e.call(ACCUMULATOR_STEP, &args![accumulator, 0u32]);
    e.mem.set_u32(t + 0x2c8, 1);
    for i in 0..0x10u32 {
        e.mem.set_u8(t + i + 0x119, 0);
        e.mem.set_u32(t + i * 4 + 0x16c, 0);
        e.mem.set_u32(t + i * 4 + 0x12c, 0);
        e.mem.set_u8(t + i + 0x1ac, 0);
    }
    this
}

// Translated from 00800170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The VATS target constructor (no PDB name; `FOVATSEffectManager::
/// VATS_TARGET`, 8 bytes): constructs the pointer at +4 empty. Returns
/// `this`.
pub fn fn_00800170(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_CONSTRUCT, &args![this.byte_add(4), 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_fec0, fovats_effect_manager_constructor(Ptr) -> Ptr),
        entry!(0x0080_0170, fn_00800170(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00800170_constructs_the_pointer() {
        let mut e = Engine::new();
        e.register(NI_POINTER_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_0170, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, NI_POINTER_CONSTRUCT), vec![vec![0x5004, 0]]);
    }

    #[test]
    fn fovats_effect_manager_constructor_sets_up_the_effects() {
        let mut e = Engine::new();
        for address in [
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            VECTOR_CONSTRUCTOR_ITERATOR,
            BS_SHADER_MANAGER_GET_TEXTURE,
            ACCUMULATOR_STEP,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SETTING_FLOAT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_INT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_STRING_VALUE, |_, _| Ret {
            eax: 0x0000_7100,
            ..Ret::default()
        });
        e.map(DISTORT_DURATION_SETTING, 8);
        e.mem.set_f32(DISTORT_DURATION_SETTING + 4, 0.75);
        e.map(DISTORT_INTERVAL_SETTING, 8);
        e.mem.set_i32(DISTORT_INTERVAL_SETTING + 4, 1000);
        e.register(GET_MILLISECONDS, |_, _| Ret {
            eax: 5000,
            ..Ret::default()
        });
        e.register(RAND, |_, _| Ret {
            eax: 2345,
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BS_SHADER_ACCUMULATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(READ_WORD, |_, _| Ret {
            eax: 0x0000_8800,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x2d0);
        for i in 0..0x10 {
            e.mem.set_u8(this + 0x119 + i, 7);
            e.mem.set_u32(this + 0x16c + 4 * i, 7);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_fec0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0x14, 8, 0x20, 0x0080_0170, TARGET_DESTRUCTOR]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![
                vec![this + 8, 0],
                vec![this + 0x114, 0],
                vec![this + 0x2c4, 0],
                vec![this + 0x2cc, 0]
            ]
        );
        assert_eq!(e.mem.u8(this + 0x129), 1);
        assert_eq!(e.mem.f32(this + 0x290), 0.75);
        assert_eq!(e.mem.f32(this + 0x294), 5345.0);
        assert_eq!(
            calls_to(&e, BS_SHADER_MANAGER_GET_TEXTURE),
            vec![vec![0x7100, 1, this + 0x2c4, 0, 0]]
        );
        assert_eq!(e.mem.i32(this + 0x10), -1);
        let accumulator = calls_to(&e, BS_SHADER_ACCUMULATOR_CONSTRUCTOR)[0].clone();
        assert_eq!(accumulator[1..], [0x65, 1, 0x2f7]);
        assert!(calls_to(&e, NI_POINTER_ASSIGN).contains(&vec![this + 0x2cc, accumulator[0]]));
        assert_eq!(calls_to(&e, ACCUMULATOR_STEP), vec![vec![0x8800, 0]]);
        assert_eq!(e.mem.u32(this + 0x2c8), 1);
        assert_eq!(e.mem.u8(this + 0x128), 0);
        assert_eq!(e.mem.u8(this + 0x119 + 15), 0);
        assert_eq!(e.mem.u32(this + 0x16c + 60), 0);
    }
}
