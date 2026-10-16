//! `fallout shared/run_0050ad90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050adf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050adf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2c (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050adf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_886c);
    e.mem.set_u32(base + 0x30, 0x0102_884c);
    e.mem.set_u32(base + 0x4c, 0x0102_8828);
    e.call(0x004f_15a0, &args![this, 0x2c]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050b400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b400 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x34 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b400(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8a64);
    e.mem.set_u32(base + 0x30, 0x0102_8a44);
    e.call(0x004f_15a0, &args![this, 0x34]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 0050b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4c (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8c5c);
    e.mem.set_u32(base + 0x30, 0x0102_8c38);
    e.call(0x004f_15a0, &args![this, 0x4c]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050a820`. `0050a820` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050ade0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_a820, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050aea0`. `0050aea0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050b3f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_aea0, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050aed0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050aea0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_aed0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b4c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b4c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b9b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b9b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_adf0, fn_0050adf0(Ptr) -> Ptr),
        entry!(0x0050_b400, fn_0050b400(Ptr) -> Ptr),
        entry!(0x0050_b8f0, fn_0050b8f0(Ptr) -> Ptr),
        entry!(0x0050_ade0, fn_0050ade0(Ptr, u32) -> u32),
        entry!(0x0050_b3f0, fn_0050b3f0(Ptr, u32) -> u32),
        entry!(0x0050_aea0, fn_0050aea0(Ptr, u32) -> Ptr),
        entry!(0x0050_b490, fn_0050b490(Ptr, u32) -> Ptr),
        entry!(0x0050_b980, fn_0050b980(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050adf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_adf0,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2c)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_886c),
                (0x30, 4, 0x0102_884c),
                (0x4c, 4, 0x0102_8828),
            ],
        });
    }

    #[test]
    fn fn_0050b400_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b400,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x34)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_8a64), (0x30, 4, 0x0102_8a44)],
        });
    }

    #[test]
    fn fn_0050b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b8f0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4c)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0102_8c5c), (0x30, 4, 0x0102_8c38)],
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
    fn le81_thunk_0050ade0() {
        assert_eq!(
            check(0x0050_ade0, 0x0050_a820, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050b3f0() {
        assert_eq!(
            check(0x0050_b3f0, 0x0050_aea0, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_0050aea0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_aed0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_aea0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_aea0)
                .collect();
            let mut expected = vec![(0x0050_aed0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b490_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b4c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b490, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b490)
                .collect();
            let mut expected = vec![(0x0050_b4c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b980_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b9b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b980, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b980)
                .collect();
            let mut expected = vec![(0x0050_b9b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
