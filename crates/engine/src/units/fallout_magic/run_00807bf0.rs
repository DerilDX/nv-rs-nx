//! `fallout/magic/run_00807bf0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00807d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00807d60`, which, when bit 0x8000 of the
/// half-word at `+6` is clear, calls `00807da0` with 0x18 and passes the result and the word
/// at `+0` to `00807dc0`, and returns nothing. Parameters: one word, `this`; the disassembly
/// of the function ends in a plain `RET` and passes only `ECX` on. `00807d60` computes no
/// result on either path (the skipped path loads nothing, the other ends `ADD ESP,0xC`);
/// Ghidra types it void.
pub fn fn_00807d40(e: &mut Engine, this: Ptr) {
    e.call(0x0080_7d60, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_7d40, fn_00807d40(Ptr))]
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
    fn fwd_a03_00807d40_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_7d60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_7d40, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_7d60), vec![vec![0x5000]]);
    }
}
