//! `fallout/dialogue/dialogueresponse.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0083d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `004048e0` of unit effectitem (the cached string
/// length, calculated when the field holds the sentinel 0xffff), and returns its result (a
/// `u32`). Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. The return type (`u32`) is that of `004048e0`'s
/// translated signature `fn_004048e0(Ptr) -> u32`.
pub fn fn_0083d890(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0040_48e0, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_d890, fn_0083d890(Ptr) -> u32)]
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
    fn fwd_a03_0083d890_returns_the_callees_word() {
        let mut e = Engine::new();
        e.register(0x0040_48e0, |_, _| Ret {
            eax: 0x1234_5678,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_d890, &args![0x0000_5000u32]).u32(),
            0x1234_5678
        );
        assert_eq!(calls_to(&e, 0x0040_48e0), vec![vec![0x5000]]);
    }
}
