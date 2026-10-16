//! `(unplaced)/run_009f98e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009f9c80`: the jump target of `009f9d50`.
const JUMP_TARGET_009F9D50: u32 = 0x009f_9c80;

// Translated from 009f9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `009f9c80`, passing on `this` and its one argument; returns the target result word. Return type (u32): the target `009f9c80` returns EAX at both returns (0, or `0x8007000e`); its meaning is unconfirmed.
pub fn fn_009f9d50(e: &mut Engine, this: Ptr, arg: Ptr) -> u32 {
    e.call(JUMP_TARGET_009F9D50, &args![this, arg]).u32()
}

// Translated from 009fdd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `009fddd0`. `009fddd0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_009fdd50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x009f_ddd0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_9d50, fn_009f9d50(Ptr, Ptr) -> u32),
        entry!(0x009f_dd50, fn_009fdd50(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_009f9d50_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x009f_9d50;
        const TARGET: u32 = 0x009f_9c80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 0x8007_000e,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(),
            0x8007_000e
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_009fdd50() {
        assert_eq!(
            check(0x009f_dd50, 0x009f_ddd0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
