//! `Havok SDK/hkptoicontactpointviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d68710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d68740`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68710(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8740, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d68720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d68740`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68720(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8740, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_8710, fn_00d68710(Ptr, u32) -> u32),
        entry!(0x00d6_8720, fn_00d68720(Ptr, u32) -> u32),
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
    fn thunk_00d68710() {
        assert_eq!(check(0x00d6_8710, 0x00d6_8740, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d68720() {
        assert_eq!(check(0x00d6_8720, 0x00d6_8740, 0x8), 0xdead_beef);
    }
}
