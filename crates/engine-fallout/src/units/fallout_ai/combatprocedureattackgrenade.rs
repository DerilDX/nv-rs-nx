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

// Translated from 009cafa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010909bc` (RTTI `.?AVCombatProcedureAttackGrenade@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009cafa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_09bc);
    e.call(0x009c_a6c0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_afa0, fn_009cafa0(Ptr)),
        entry!(0x009c_afc0, combat_procedure_attack_grenade_shutdown(Ptr)),
    ]
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

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009cafa0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009c_afa0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_09bc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_09bc,
            "the vtable was stored before the call"
        );
    }
}
