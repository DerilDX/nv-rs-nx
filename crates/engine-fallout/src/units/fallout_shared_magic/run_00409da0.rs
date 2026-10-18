//! `fallout shared/magic/run_00409da0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00409a70`. `00409a70` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0040a110(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0040_9a70,
        &args![this.addr().wrapping_sub(0x30), argument],
    );
}

// Translated from 0040a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00409a50`. `00409a50` (plain `RET`) returns 0 or 1 in `EAX` (`MOVZX EAX,AL; NEG EAX; SBB EAX,EAX; ADD EAX,1`), so the thunk returns `bool`; it takes no stack word.
pub fn fn_0040a120(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0040_9a50, &args![this.addr().wrapping_sub(0x30)])
        .bool()
}

// Translated from 0040a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `00409b10`. `00409b10` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0040a130(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0040_9b10, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_a110, fn_0040a110(Ptr, u32)),
        entry!(0x0040_a120, fn_0040a120(Ptr) -> bool),
        entry!(0x0040_a130, fn_0040a130(Ptr, u32) -> u32),
    ]
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
    fn le81_thunk_0040a110() {
        check(0x0040_a110, 0x0040_9a70, 0x30, &[0x77]);
    }

    #[test]
    fn le81_thunk_0040a120() {
        assert!(check(0x0040_a120, 0x0040_9a50, 0x30, &[]).bool());
    }

    #[test]
    fn le81_thunk_0040a130() {
        assert_eq!(
            check(0x0040_a130, 0x0040_9b10, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
