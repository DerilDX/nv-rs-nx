//! `fallout shared/run_005099c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00509a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00509a10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x24 (0x68 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00509a10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_814c);
    e.mem.set_u32(base + 0x30, 0x0102_8128);
    e.call(0x004f_15a0, &args![this, 0x24]);
    e.call(0x0050_9ee0, &args![this]);
    this
}

// Translated from 0050a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a5a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2e (0xac bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050a5a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_a6a0, &args![this]);
    e.mem.set_u32(base, 0x0102_8444);
    e.mem.set_u32(base + 0x30, 0x0102_842c);
    e.mem.set_u32(base + 0x3c, 0x0102_8408);
    e.mem.set_u32(base + 0x5c, 0x0102_83e8);
    e.mem.set_u32(base + 0x68, 0x0102_83d4);
    e.mem.set_u32(base + 0x74, 0x0102_83b4);
    e.mem.set_u32(base + 0x7c, 0x0102_83a0);
    e.mem.set_u32(base + 0x84, 0x0102_838c);
    e.mem.set_u32(base + 0x8c, 0x0102_8378);
    e.mem.set_u32(base + 0x9c, 0x0102_8364);
    e.call(0x004f_15a0, &args![this, 0x2e]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050a770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2d (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050a770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_864c);
    e.mem.set_u32(base + 0x30, 0x0102_862c);
    e.mem.set_u32(base + 0x4c, 0x0102_8608);
    e.call(0x004f_15a0, &args![this, 0x2d]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 00509a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `00508eb0`. `00508eb0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00509a00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_8eb0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 0050a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00509c90`. `00509c90` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050a590(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_9c90, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 0050a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `0050a670`. `0050a670` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050a760(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_a670, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00509c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00509cc0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00509c90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_9cc0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050a6a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050a670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_a6a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050a820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050a850` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050a820(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_a850, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_9a10, fn_00509a10(Ptr) -> Ptr),
        entry!(0x0050_a5a0, fn_0050a5a0(Ptr) -> Ptr),
        entry!(0x0050_a770, fn_0050a770(Ptr) -> Ptr),
        entry!(0x0050_9a00, fn_00509a00(Ptr, u32) -> u32),
        entry!(0x0050_a590, fn_0050a590(Ptr, u32) -> u32),
        entry!(0x0050_a760, fn_0050a760(Ptr, u32) -> u32),
        entry!(0x0050_9c90, fn_00509c90(Ptr, u32) -> Ptr),
        entry!(0x0050_a670, fn_0050a670(Ptr, u32) -> Ptr),
        entry!(0x0050_a820, fn_0050a820(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00509a10_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_9a10,
            size: 0x68,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x24)]),
                (0x0050_9ee0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_814c), (0x30, 4, 0x0102_8128)],
        });
    }

    #[test]
    fn fn_0050a5a0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_a5a0,
            size: 0xac,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_a6a0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2e)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_8444),
                (0x30, 4, 0x0102_842c),
                (0x3c, 4, 0x0102_8408),
                (0x5c, 4, 0x0102_83e8),
                (0x68, 4, 0x0102_83d4),
                (0x74, 4, 0x0102_83b4),
                (0x7c, 4, 0x0102_83a0),
                (0x84, 4, 0x0102_838c),
                (0x8c, 4, 0x0102_8378),
                (0x9c, 4, 0x0102_8364),
            ],
        });
    }

    #[test]
    fn fn_0050a770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_a770,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2d)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_864c),
                (0x30, 4, 0x0102_862c),
                (0x4c, 4, 0x0102_8608),
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
    fn le81_thunk_00509a00() {
        assert_eq!(
            check(0x0050_9a00, 0x0050_8eb0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050a590() {
        assert_eq!(
            check(0x0050_a590, 0x0050_9c90, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050a760() {
        assert_eq!(
            check(0x0050_a760, 0x0050_a670, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_00509c90_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_9cc0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_9c90, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_9c90)
                .collect();
            let mut expected = vec![(0x0050_9cc0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050a670_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_a6a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_a670, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_a670)
                .collect();
            let mut expected = vec![(0x0050_a6a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050a820_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_a850, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_a820, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_a820)
                .collect();
            let mut expected = vec![(0x0050_a850, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
