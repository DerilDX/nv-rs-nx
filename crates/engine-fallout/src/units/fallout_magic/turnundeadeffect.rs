//! `fallout/magic/turnundeadeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActiveEffect::CheckDeletedCaster` (engine map), thiscall (effect, reference).
const ACTIVE_EFFECT_CHECK_DELETED_CASTER: u32 = 0x0080_6200;
/// `ActiveEffect::Dispel` (engine map), thiscall (effect, flag).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;

// Translated from 0080ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::CheckDeletedCaster` (Xbox PDB), thiscall
/// `(reference)`: runs `ActiveEffect::CheckDeletedCaster` and, when it
/// answers true, dispels the effect (`ActiveEffect::Dispel(1)`); answers
/// the base's answer.
pub fn turn_undead_effect_check_deleted_caster(e: &mut Engine, this: Ptr, reference: u32) -> bool {
    let deleted = e
        .call(ACTIVE_EFFECT_CHECK_DELETED_CASTER, &args![this, reference])
        .u8();
    if deleted != 0 {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 1u32]);
    }
    deleted != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0080ba50,
        turn_undead_effect_check_deleted_caster(Ptr, u32) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_deleted_caster_dispels_the_effect() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CHECK_DELETED_CASTER, |_, a| Ret {
            eax: u32::from(a[1] == 7),
            ..Ret::default()
        });
        e.register(ACTIVE_EFFECT_DISPEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0080_ba50, &args![0x100u32, 7u32]).bool());
        assert!(!e.call(0x0080_ba50, &args![0x100u32, 8u32]).bool());
        let log = e.call_log.take().unwrap();
        let dispels: Vec<_> = log.iter().filter(|c| c.0 == ACTIVE_EFFECT_DISPEL).collect();
        assert_eq!(dispels.len(), 1);
        assert_eq!(dispels[0].1, vec![0x100, 1]);
    }
}
