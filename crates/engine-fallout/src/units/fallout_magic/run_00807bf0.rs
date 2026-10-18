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

// Translated from 00807df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `008078e0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00807df0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0080_78e0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00807e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `008078e0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00807e00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0080_78e0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_7d40, fn_00807d40(Ptr)),
        entry!(0x0080_7df0, fn_00807df0(Ptr, u32) -> u32),
        entry!(0x0080_7e00, fn_00807e00(Ptr, u32) -> u32),
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
    fn fwd_a03_00807d40_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_7d60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_7d40, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_7d60), vec![vec![0x5000]]);
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
    fn thunk_00807df0() {
        assert_eq!(check(0x0080_7df0, 0x0080_78e0, 0x4), 0xdead_beef);
    }

    #[test]
    fn thunk_00807e00() {
        assert_eq!(check(0x0080_7e00, 0x0080_78e0, 0x8), 0xdead_beef);
    }
}
