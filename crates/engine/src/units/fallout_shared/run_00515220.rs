//! `fallout shared/run_00515220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00515440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00515440 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1a (0x154 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00515440(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_cc50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x48]);
    e.call(0x0048_e7a0, &args![base + 0x58]);
    e.call(0x0048_eac0, &args![base + 0x60]);
    e.call(0x0048_02f0, &args![base + 0x68]);
    e.call(0x0047_5350, &args![base + 0x144]);
    e.call(0x0047_9320, &args![base + 0x14c]);
    e.mem.set_u32(base, 0x0102_ac0c);
    e.mem.set_u32(base + 0x30, 0x0102_abf8);
    e.mem.set_u32(base + 0x3c, 0x0102_abe4);
    e.mem.set_u32(base + 0x48, 0x0102_abcc);
    e.mem.set_u32(base + 0x58, 0x0102_abac);
    e.mem.set_u32(base + 0x60, 0x0102_ab98);
    e.mem.set_u32(base + 0x68, 0x0102_ab84);
    e.mem.set_u32(base + 0x144, 0x0102_ab70);
    e.mem.set_u32(base + 0x14c, 0x0102_ab5c);
    e.call(0x004f_15a0, &args![this, 0x1a]);
    e.call(0x0051_56b0, &args![this]);
    this
}

// Translated from 00515400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `005146a0`. `005146a0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00515400(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_46a0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00515410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `005152e0`. `005152e0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_00515410(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_52e0,
        &args![this.addr().wrapping_sub(0x84), first, second],
    );
}

// Translated from 00515420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00515290`. `00515290` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00515420(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_5290,
        &args![this.addr().wrapping_sub(0x84), argument],
    );
}

// Translated from 00515430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00515220`. `00515220` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00515430(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_5220,
        &args![this.addr().wrapping_sub(0x84), argument],
    )
    .u16()
}

// Translated from 00515580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005155b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00515580(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_55b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_5440, fn_00515440(Ptr) -> Ptr),
        entry!(0x0051_5400, fn_00515400(Ptr, u32) -> u32),
        entry!(0x0051_5410, fn_00515410(Ptr, u32, u32)),
        entry!(0x0051_5420, fn_00515420(Ptr, u32)),
        entry!(0x0051_5430, fn_00515430(Ptr, u32) -> u16),
        entry!(0x0051_5580, fn_00515580(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00515440_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_5440,
            size: 0x154,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_cc50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x48)]),
                (0x0048_e7a0, &[A::T(0x58)]),
                (0x0048_eac0, &[A::T(0x60)]),
                (0x0048_02f0, &[A::T(0x68)]),
                (0x0047_5350, &[A::T(0x144)]),
                (0x0047_9320, &[A::T(0x14c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1a)]),
                (0x0051_56b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_ac0c),
                (0x30, 4, 0x0102_abf8),
                (0x3c, 4, 0x0102_abe4),
                (0x48, 4, 0x0102_abcc),
                (0x58, 4, 0x0102_abac),
                (0x60, 4, 0x0102_ab98),
                (0x68, 4, 0x0102_ab84),
                (0x144, 4, 0x0102_ab70),
                (0x14c, 4, 0x0102_ab5c),
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
    fn le81_thunk_00515400() {
        assert_eq!(
            check(0x0051_5400, 0x0051_46a0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00515410() {
        check(0x0051_5410, 0x0051_52e0, 0x84, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_00515420() {
        check(0x0051_5420, 0x0051_5290, 0x84, &[0x77]);
    }

    #[test]
    fn le81_thunk_00515430() {
        assert_eq!(check(0x0051_5430, 0x0051_5220, 0x84, &[0x77]).u16(), 0xbeef);
    }
    #[test]
    fn at_00515580_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_55b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_5580, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_5580)
                .collect();
            let mut expected = vec![(0x0051_55b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
