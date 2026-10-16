//! `Havok SDK/hkpconstrainedsystemfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dffb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00dffb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_fb30, fn_00dffb30(Ptr, u32) -> u32),
        entry!(0x00df_fb50, fn_00dffb50(Ptr, u32) -> u32),
    ]
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
    fn thunk_00dffb30() {
        assert_eq!(check(0x00df_fb30, 0x00df_fb00, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00dffb50() {
        assert_eq!(check(0x00df_fb50, 0x00df_fb00, 0x10), 0xdead_beef);
    }
}
