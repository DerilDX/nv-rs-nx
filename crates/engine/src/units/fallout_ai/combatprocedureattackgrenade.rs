//! `fallout/ai/combat/combatprocedureattackgrenade.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009cafc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00996ea0` (Ghidra name
/// `CombatProcedure::ClearLookAtTarget`), which clears the look-at target of the controller
/// at `+4` through `00981720`. The Xbox PDB name `CombatProcedureAttackGrenade::Shutdown` is
/// the class's vslot +0x00c (pdbdump `type ... CombatProcedureAttackGrenade`; engine map row
/// `vt 827db8b8`), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00996ea0` ends right after
/// calling `00981720` (`RET 4`, a byte store only), so no result is computed (Ghidra types
/// it void).
pub fn combat_procedure_attack_grenade_shutdown(e: &mut Engine, this: Ptr) {
    e.call(0x0099_6ea0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009c_afc0,
        combat_procedure_attack_grenade_shutdown(Ptr)
    )]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_009cafc0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0099_6ea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_afc0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0099_6ea0), vec![vec![0x5000]]);
    }
}
