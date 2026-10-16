//! `fallout shared/pathfinding/run_006c00d0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006c0170 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006c0150`, which stores the vtable `0106c3c4` at
/// `+0`, and returns nothing. Parameters: one word, `this`; the disassembly of the function
/// ends in a plain `RET` and passes only `ECX` on. `006c0150`'s disassembly ends on that
/// vtable store with no result computed (Ghidra types it void); its `MOV EAX,[EBP-4]` only
/// addresses the store.
pub fn fn_006c0170(e: &mut Engine, this: Ptr) {
    e.call(0x006c_0150, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_0170, fn_006c0170(Ptr))]
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
    fn fwd_a03_006c0170_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x006c_0150, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006c_0170, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x006c_0150), vec![vec![0x5000]]);
    }
}
