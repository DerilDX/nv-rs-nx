//! `fallout shared/havok/run_0062e350` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0062e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x1c (`SUB ECX,0x1c`) and jumps to `0062d870`. `0062d870` (`RET 4`, which the Xbox PDB names `TESTrapListener::_scalar_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d870, &args![this.addr().wrapping_sub(0x1c), flags])
        .u32()
}

// Translated from 0062e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `0062d870`. `0062d870` (`RET 4`, which the Xbox PDB names `TESTrapListener::_scalar_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d870, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 0062e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `0062d6c0`. `0062d6c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d6c0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_e3a0, fn_0062e3a0(Ptr, u32) -> u32),
        entry!(0x0062_e3b0, fn_0062e3b0(Ptr, u32) -> u32),
        entry!(0x0062_e3c0, fn_0062e3c0(Ptr, u32) -> u32),
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
    fn le81_thunk_0062e3a0() {
        assert_eq!(
            check(0x0062_e3a0, 0x0062_d870, 0x1c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0062e3b0() {
        assert_eq!(
            check(0x0062_e3b0, 0x0062_d870, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0062e3c0() {
        assert_eq!(
            check(0x0062_e3c0, 0x0062_d6c0, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
