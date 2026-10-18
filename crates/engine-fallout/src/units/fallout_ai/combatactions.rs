//! `fallout/ai/combat/combatactions.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006815c0` of unit run_006815c0, which answers
/// `this` itself, and returns its result (a pointer). Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. The return
/// type (`Ptr`) is that of `006815c0`'s translated signature `fn_006815c0(Ptr) -> Ptr` (its
/// doc: `mov eax, ecx`).
pub fn fn_0097b940(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]).ptr()
}

// Translated from 0097c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00979c80`, a destructor with an exception frame
/// that stores the vtable `0108cab8` and runs four list-clearing loops, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00979c80`'s disassembly ends with its exception-frame unlink and
/// returns no value (Ghidra types it void).
pub fn fn_0097c470(e: &mut Engine, this: Ptr) {
    e.call(0x0097_9c80, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_b940, fn_0097b940(Ptr) -> Ptr),
        entry!(0x0097_c470, fn_0097c470(Ptr)),
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
    fn fwd_a03_0097b940_returns_the_callees_pointer() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_b940, &args![0x0000_5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, 0x0068_15c0), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_0097c470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0097_9c80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_c470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0097_9c80), vec![vec![0x5000]]);
    }
}
