//! `fallout shared/run_00504cc0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505010 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4a (0x108 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00505010(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_66e4);
    e.call(0x004f_15a0, &args![this, 0x4a]);
    e.call(0x0050_50d0, &args![this]);
    this
}

// Translated from 00504fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x78 (`SUB ECX,0x78`) and jumps to `00504420`. `00504420` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_00504fe0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0050_4420,
        &args![this.addr().wrapping_sub(0x78), first, second],
    );
}

// Translated from 00504ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `00503d10`. `00503d10` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00504ff0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_3d10, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00505080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005050b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00505080(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_50b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_5010, fn_00505010(Ptr) -> Ptr),
        entry!(0x0050_4fe0, fn_00504fe0(Ptr, u32, u32)),
        entry!(0x0050_4ff0, fn_00504ff0(Ptr, u32) -> u32),
        entry!(0x0050_5080, fn_00505080(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00505010_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_5010,
            size: 0x108,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4a)]),
                (0x0050_50d0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_66e4)],
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
    fn le81_thunk_00504fe0() {
        check(0x0050_4fe0, 0x0050_4420, 0x78, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_00504ff0() {
        assert_eq!(
            check(0x0050_4ff0, 0x0050_3d10, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_00505080_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_50b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_5080, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_5080)
                .collect();
            let mut expected = vec![(0x0050_50b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
