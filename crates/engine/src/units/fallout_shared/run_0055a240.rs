//! `fallout shared/run_0055a240` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0055a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0055a2b0`, which calls `006208d0` on the member
/// at `this + 8`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0055a2b0` ends right after that
/// call, and `006208d0` ends with its exception-frame unlink after `CALL 0045cec0`; no
/// result is computed and Ghidra types both void.
pub fn fn_0055a290(e: &mut Engine, this: Ptr) {
    e.call(0x0055_a2b0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0055_a290, fn_0055a290(Ptr))]
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
    fn fwd_a03_0055a290_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0055_a2b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0055_a290, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0055_a2b0), vec![vec![0x5000]]);
    }
}
