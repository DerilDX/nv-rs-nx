//! `fallout shared/pathfinding/run_006e3e70` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `006e3a50`. `006e3a50` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_006e3f10(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x006e_3a50, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 006e3f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `006e3880`. `006e3880` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_006e3f20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x006e_3880, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_3f10, fn_006e3f10(Ptr, u32) -> u32),
        entry!(0x006e_3f20, fn_006e3f20(Ptr, u32) -> u32),
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
    fn le81_thunk_006e3f10() {
        assert_eq!(
            check(0x006e_3f10, 0x006e_3a50, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_006e3f20() {
        assert_eq!(
            check(0x006e_3f20, 0x006e_3880, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
