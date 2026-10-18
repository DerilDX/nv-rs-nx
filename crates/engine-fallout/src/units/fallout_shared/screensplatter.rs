//! `fallout shared/screensplatter.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004deb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a7b300`, which stores the vtable `0109da1c` at
/// `+0`, decrements the counter in the header of the block at `+8` through the import slot
/// `00fdf1c4` when `+8` is set, and tail-jumps into `00a5d3d0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00a7b300` ends in `JMP 00a5d3d0` (a base-destructor tail call
/// whose own result nothing here uses); Ghidra types it void.
pub fn fn_004deb10(e: &mut Engine, this: Ptr) {
    e.call(0x00a7_b300, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004d_eb10, fn_004deb10(Ptr))]
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
    fn fwd_a03_004deb10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a7_b300, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004d_eb10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a7_b300), vec![vec![0x5000]]);
    }
}
