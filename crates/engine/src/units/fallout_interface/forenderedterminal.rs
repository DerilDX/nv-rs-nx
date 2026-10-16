//! `fallout/interface/forenderedterminal.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007ff8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007ff8e0`, which tests the flag `0x4000` of the
/// word at `+0x30` through `00456630`, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`) is from `00456630`'s disassembly (`AND EAX,[EBP+8]; NEG EAX;
/// SBB EAX,EAX; NEG EAX`: 1 when the masked word is non-zero, else 0), which `007ff8e0`
/// returns without change.
pub fn fn_007ff8c0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x007f_f8e0, &args![this]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007f_f8c0, fn_007ff8c0(Ptr) -> bool)]
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
    fn fwd_a03_007ff8c0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x007f_f8e0), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
    }
}
