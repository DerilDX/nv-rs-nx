//! `fallout/magic/telekinesiseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0082bbe0` (named `ValueModifierEffect::CopyData` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const COPY_DATA_TARGET_0082BBE0: u32 = 0x0082_bbe0;

// Translated from 0082a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::CopyData` (Xbox PDB
/// `?CopyData@TelekinesisEffect@@MAAXPAVActiveEffect@@@Z`, void, vtable word 010792f0 and
/// others): forwards `this` and the `ActiveEffect` pointer to the base body `0082bbe0`
/// (`ValueModifierEffect::CopyData`). Returns nothing.
pub fn telekinesis_effect_copy_data(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(COPY_DATA_TARGET_0082BBE0, &args![this, target]);
}

/// `004ae750` (`thiscall`; engine map `bhkCharacterProxy::operatorP`, a folded body):
/// an object of the effect, or null.
const OBJECT_004AE750: u32 = 0x004a_e750;
/// `009611e0` (`thiscall`): the word at +0x18.
const WORD_18_009611E0: u32 = 0x0096_11e0;
/// `004b5a80` (`globalfunc.cpp`, `cdecl`, one word).
const GLOBAL_004B5A80: u32 = 0x004b_5a80;

// Translated from 0082aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): passes [`fn_0082aee0`]'s result
/// to `004b5a80` and returns what that returns.
pub fn fn_0082aeb0(e: &mut Engine, this: Ptr) -> u32 {
    let object = fn_0082aee0(e, this);
    e.call(GLOBAL_004B5A80, &args![object]).u32()
}

// Translated from 0082aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at +0x18 (`009611e0`) of
/// the object `004ae750` returns for `this`, or null when there is none.
pub fn fn_0082aee0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.call(OBJECT_004AE750, &args![this]).u32();
    if object == 0 {
        return 0;
    }
    e.call(WORD_18_009611E0, &args![object]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_a650, telekinesis_effect_copy_data(Ptr, Ptr)),
        entry!(0x0082_aeb0, fn_0082aeb0(Ptr) -> u32),
        entry!(0x0082_aee0, fn_0082aee0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod object_tests {
    use super::*;

    #[test]
    fn fn_0082aee0_follows_the_object_to_its_word_18() {
        let mut e = Engine::new();
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORD_18_009611E0, |_, a| (a[0] + 0x18).into_ret());
        assert_eq!(e.call(0x0082_aee0, &[0x1000]).u32(), 0x1118);
        e.register(OBJECT_004AE750, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_aee0, &[0x1000]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == WORD_18_009611E0));
    }

    #[test]
    fn fn_0082aeb0_passes_it_on() {
        let mut e = Engine::new();
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORD_18_009611E0, |_, a| (a[0] + 0x18).into_ret());
        e.register(GLOBAL_004B5A80, |_, a| (a[0] ^ 0xffff).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_aeb0, &[0x1000]).u32(), 0x1118 ^ 0xffff);
        let log = e.call_log.take().unwrap();
        let passed: Vec<_> = log.iter().filter(|(a, _)| *a == GLOBAL_004B5A80).collect();
        assert_eq!(passed[0].1, vec![0x1118]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_telekinesis_effect_copy_data_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(COPY_DATA_TARGET_0082BBE0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_a650, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == COPY_DATA_TARGET_0082BBE0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
