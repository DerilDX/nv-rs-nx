//! `fallout shared/run_004fe520` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fe640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe640 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x16 (0x98 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fe640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.mem.set_u32(base, 0x0102_5594);
    e.mem.set_u32(base + 0x30, 0x0102_557c);
    e.mem.set_u32(base + 0x3c, 0x0102_5558);
    e.mem.set_u32(base + 0x5c, 0x0102_5544);
    e.mem.set_u32(base + 0x68, 0x0102_5530);
    e.mem.set_u32(base + 0x70, 0x0102_5520);
    e.call(0x004f_15a0, &args![this, 0x16]);
    e.call(0x004f_e7a0, &args![this]);
    this
}

// Translated from 004fe630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `004fe2e0`. `004fe2e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004fe630(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_e2e0, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 004fe6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004fe710` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004fe6e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_e710, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_e640, fn_004fe640(Ptr) -> Ptr),
        entry!(0x004f_e630, fn_004fe630(Ptr, u32) -> u32),
        entry!(0x004f_e6e0, fn_004fe6e0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fe640_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_e640,
            size: 0x98,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x16)]),
                (0x004f_e7a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_5594),
                (0x30, 4, 0x0102_557c),
                (0x3c, 4, 0x0102_5558),
                (0x5c, 4, 0x0102_5544),
                (0x68, 4, 0x0102_5530),
                (0x70, 4, 0x0102_5520),
            ],
        });
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
    fn le81_thunk_004fe630() {
        assert_eq!(
            check(0x004f_e630, 0x004f_e2e0, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_004fe6e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_e710, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_e6e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_e6e0)
                .collect();
            let mut expected = vec![(0x004f_e710, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
