//! `fallout shared/run_00501b40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00511860` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_00511860: u32 = 0x0051_1860;

// Translated from 00501b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01025968): forwards `this` and one pointer word to
/// `00511860`. Returns nothing: `00511860` has no value-producing path (it runs `00484d60` on
/// `this`, then maybe calls `00865e70` on the word and returns with whatever that call left in
/// `EAX`). Parameter count from the `RET 4` of both.
pub fn fn_00501b90(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_00511860, &args![this, argument]);
}

// Translated from 00501cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x3c` (`SUB ECX,0x3c`) and jumps to `004ffb70`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00501cc0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_fb70, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_1b90, fn_00501b90(Ptr, Ptr)),
        entry!(0x0050_1cc0, fn_00501cc0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00501b90_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_00511860, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_1b90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_00511860)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
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
    fn thunk_00501cc0() {
        assert_eq!(check(0x0050_1cc0, 0x004f_fb70, 0x3c), 0xdead_beef);
    }
}
