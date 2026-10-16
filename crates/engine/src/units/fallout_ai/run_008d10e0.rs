//! `fallout/ai/run_008d10e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c705c0` of unit bhkcharactercontroller (the
/// character controller's destructor body), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00c705c0`'s translated signature `fn_00c705c0(Ptr<Ctl>)` returns nothing.
pub fn fn_008d1160(e: &mut Engine, this: Ptr) {
    e.call(0x00c7_05c0, &args![this]);
}

// Translated from 008d1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x410` (`SUB ECX,0x410`) and jumps to `008d1130`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_008d1550(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_1130, &args![this.addr().wrapping_sub(0x410), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_1160, fn_008d1160(Ptr)),
        entry!(0x008d_1550, fn_008d1550(Ptr, u32) -> u32),
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
    fn fwd_a03_008d1160_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c7_05c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_1160, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c7_05c0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_008d1550() {
        assert_eq!(check(0x008d_1550, 0x008d_1130, 0x410), 0xdead_beef);
    }
}
