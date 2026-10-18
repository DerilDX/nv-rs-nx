//! `NiMain/nialphaaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAlphaAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_ALPHA_ACCUMULATOR_RTTI: u32 = 0x011f_5f74;

// Translated from 00a9b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_alpha_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ALPHA_ACCUMULATOR_RTTI)
}

/// `00aa0ac0`: the jump target of `00a9b4f0`.
const JUMP_TARGET_00A9B4F0: u32 = 0x00aa_0ac0;

/// `00aa0ad0`: the jump target of `00a9b500`.
const JUMP_TARGET_00A9B500: u32 = 0x00aa_0ad0;

// Translated from 00a9b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::LoadBinary` (Xbox PDB): a 5-byte jump to `00aa0ac0` (engine map: `NiKeyBasedInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiAlphaAccumulator@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B4F0, &args![this, stream]);
}

// Translated from 00a9b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00aa0ad0`, passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaAccumulator@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B500, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b530, ni_alpha_accumulator_get_rtti() -> Ptr),
        entry!(0x00a9_b4f0, ni_alpha_accumulator_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b500, ni_alpha_accumulator_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_alpha_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9b530, &args![]).u32(),
            NI_ALPHA_ACCUMULATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a9b4f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b4f0;
        const TARGET: u32 = 0x00aa_0ac0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a9b500_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b500;
        const TARGET: u32 = 0x00aa_0ad0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
