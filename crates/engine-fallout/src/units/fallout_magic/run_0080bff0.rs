//! `fallout/magic/run_0080bff0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `ConcussionEffect` constructor `0080bc70` (`thiscall`, three words).
const CONCUSSION_EFFECT_CONSTRUCTOR: u32 = 0x0080_bc70;

// Translated from 0080bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x4c-byte
/// `ConcussionEffect` (`00401000`) and constructs it with the three words (`0080bc70`);
/// returns it, or null when the allocation fails.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0080bff0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_0080bff0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        CONCUSSION_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_bff0, fn_0080bff0(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(CONCUSSION_EFFECT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bff0, &[7, 8, 9]).u32(), 0x5001);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![0x4c]));
        assert_eq!(
            log[2],
            (CONCUSSION_EFFECT_CONSTRUCTOR, vec![0x5000, 7, 8, 9])
        );

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bff0, &[7, 8, 9]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == CONCUSSION_EFFECT_CONSTRUCTOR));
    }
}
