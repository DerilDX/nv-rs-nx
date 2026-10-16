//! `fallout shared/run_004f9b80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f9cd0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x52 (0x24 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004f9cd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_4834);
    e.mem.set_u32(base + 0x18, 0x0102_4818);
    e.call(0x0096_a2d0, &args![base + 0x1c]);
    e.call(0x004f_15a0, &args![this, 0x52]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 004f9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b070`. `0051b070` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_004f9c90(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_b070,
        &args![this.addr().wrapping_sub(0x74), argument],
    );
}

// Translated from 004f9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b020`. `0051b020` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_004f9ca0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_b020,
        &args![this.addr().wrapping_sub(0x74), argument],
    )
    .u16()
}

// Translated from 004f9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `004f9480`. `004f9480` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004f9cb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_9480, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 004f9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b0a0`. `0051b0a0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_004f9cc0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_b0a0,
        &args![this.addr().wrapping_sub(0x74), first, second],
    );
}

// Translated from 004f9d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSPreloadable` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `01024970`, is
/// `.?AVBGSPreloadable@@`): runs the base constructor (`00402d30`) and stores
/// the vtable at `+0`. Returns `this`.
pub fn fn_004f9d90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_4970);
    this
}

// Translated from 004f9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f9db0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f9d60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_9db0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f9ef0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f9ec0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_9ef0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_9cd0, fn_004f9cd0(Ptr) -> Ptr),
        entry!(0x004f_9c90, fn_004f9c90(Ptr, u32)),
        entry!(0x004f_9ca0, fn_004f9ca0(Ptr, u32) -> u16),
        entry!(0x004f_9cb0, fn_004f9cb0(Ptr, u32) -> u32),
        entry!(0x004f_9cc0, fn_004f9cc0(Ptr, u32, u32)),
        entry!(0x004f_9d90, fn_004f9d90(Ptr) -> Ptr),
        entry!(0x004f_9d60, fn_004f9d60(Ptr, u32) -> Ptr),
        entry!(0x004f_9ec0, fn_004f9ec0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f9cd0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_9cd0,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x1c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x52)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_4834), (0x18, 4, 0x0102_4818)],
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
    fn le81_thunk_004f9c90() {
        check(0x004f_9c90, 0x0051_b070, 0x74, &[0x77]);
    }

    #[test]
    fn le81_thunk_004f9ca0() {
        assert_eq!(check(0x004f_9ca0, 0x0051_b020, 0x74, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_004f9cb0() {
        assert_eq!(
            check(0x004f_9cb0, 0x004f_9480, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_004f9cc0() {
        check(0x004f_9cc0, 0x0051_b0a0, 0x74, &[0x77, 0x88]);
    }
}

#[cfg(test)]
mod component_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f9d90_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_9d90,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0102_4970)],
        });
    }
    #[test]
    fn at_004f9d60_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_9db0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_9d60, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_9d60)
                .collect();
            let mut expected = vec![(0x004f_9db0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f9ec0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_9ef0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_9ec0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_9ec0)
                .collect();
            let mut expected = vec![(0x004f_9ef0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
