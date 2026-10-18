//! `fallout/magic/paralysiseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0082cd10` (named `ValueModifierEffect::FinishInitLoadGame` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const FINISH_INIT_LOAD_GAME_TARGET_0082CD10: u32 = 0x0082_cd10;

// Translated from 00826550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::FinishInitLoadGame` (Xbox PDB
/// `?FinishInitLoadGame@ParalysisEffect@@MAAXPAVActor@@@Z`, void, vtable word 0107bfe8):
/// forwards `this` and the `Actor` pointer to the base body `0082cd10`
/// (`ValueModifierEffect::FinishInitLoadGame`). Returns nothing.
pub fn paralysis_effect_finish_init_load_game(e: &mut Engine, this: Ptr, actor: Ptr) {
    e.call(FINISH_INIT_LOAD_GAME_TARGET_0082CD10, &args![this, actor]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0082_6550,
        paralysis_effect_finish_init_load_game(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_paralysis_effect_finish_init_load_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FINISH_INIT_LOAD_GAME_TARGET_0082CD10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_6550, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FINISH_INIT_LOAD_GAME_TARGET_0082CD10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
