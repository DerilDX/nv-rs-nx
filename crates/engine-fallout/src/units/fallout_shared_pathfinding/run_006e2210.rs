//! `fallout shared/pathfinding/run_006e2210` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006e22d0` calls on the member (`004ff7e0`).
const CALLEE_004FF7E0: u32 = 0x004f_f7e0;

// Translated from 006e22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `004ff7e0` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_006e22d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_004FF7E0, &args![this.byte_add(0x4)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_22d0, fn_006e22d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006e22d0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_004FF7E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_22d0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_004FF7E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}
