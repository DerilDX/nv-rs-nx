//! `fallout shared/run_005128e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005129c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00616640`. `00616640` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005129c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0061_6640,
        &args![this.addr().wrapping_sub(0x58), first, second],
    );
}

// Translated from 005129d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `0059adb0`. `0059adb0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005129d0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0059_adb0,
        &args![this.addr().wrapping_sub(0x58), argument],
    );
}

// Translated from 005129e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `005a7770`. `005a7770` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005129e0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005a_7770,
        &args![this.addr().wrapping_sub(0x58), argument],
    )
    .u16()
}

// Translated from 005129f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005129f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x18; `ECX` is the object.
/// The engine map gives it no name, so the class is unconfirmed.
/// It runs the base (`0050e820`) and component constructors by address, stores the vtable pointers and
/// finishes with the form-type registration (`004f15a0`) and `00512e10`.
/// The exception-unwinding frame is not translated.
pub fn fn_005129f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_cc50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x48]);
    e.call(0x0048_e7a0, &args![base + 0x58]);
    e.call(0x0048_eac0, &args![base + 0x60]);
    e.call(0x0048_71d0, &args![base + 0x68]);
    e.call(0x0048_02f0, &args![base + 0x70]);
    e.call(0x0047_5350, &args![base + 0x14c]);
    e.call(0x0047_9320, &args![base + 0x154]);
    e.call(0x0047_b850, &args![base + 0x15c]);
    e.call(0x0047_4d10, &args![base + 0x164]);
    e.call(0x0047_b4f0, &args![base + 0x16c]);
    for (offset, vtable) in [
        (0x00, 0x0102_a62c),
        (0x30, 0x0102_a614),
        (0x3c, 0x0102_a600),
        (0x48, 0x0102_a5e8),
        (0x58, 0x0102_a5c8),
        (0x60, 0x0102_a5b4),
        (0x68, 0x0102_a59c),
        (0x70, 0x0102_a588),
        (0x14c, 0x0102_a574),
        (0x154, 0x0102_a560),
        (0x15c, 0x0102_a54c),
        (0x164, 0x0102_a538),
        (0x16c, 0x0102_a524),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x004f_15a0, &args![this, 0x18u32]);
    e.call(0x0051_2e10, &args![this]);
    this
}

// Translated from 00512ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00512bd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00512ba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_2bd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_29c0, fn_005129c0(Ptr, u32, u32)),
        entry!(0x0051_29d0, fn_005129d0(Ptr, u32)),
        entry!(0x0051_29e0, fn_005129e0(Ptr, u32) -> u16),
        entry!(0x0051_29f0, fn_005129f0(Ptr) -> Ptr),
        entry!(0x0051_2ba0, fn_00512ba0(Ptr, u32) -> Ptr),
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
    fn le81_thunk_005129c0() {
        check(0x0051_29c0, 0x0061_6640, 0x58, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005129d0() {
        check(0x0051_29d0, 0x0059_adb0, 0x58, &[0x77]);
    }

    #[test]
    fn le81_thunk_005129e0() {
        assert_eq!(check(0x0051_29e0, 0x005a_7770, 0x58, &[0x77]).u16(), 0xbeef);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005129f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_29f0,
            size: 0x180,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_cc50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x48)]),
                (0x0048_e7a0, &[A::T(0x58)]),
                (0x0048_eac0, &[A::T(0x60)]),
                (0x0048_71d0, &[A::T(0x68)]),
                (0x0048_02f0, &[A::T(0x70)]),
                (0x0047_5350, &[A::T(0x14c)]),
                (0x0047_9320, &[A::T(0x154)]),
                (0x0047_b850, &[A::T(0x15c)]),
                (0x0047_4d10, &[A::T(0x164)]),
                (0x0047_b4f0, &[A::T(0x16c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x18)]),
                (0x0051_2e10, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a62c),
                (0x30, 4, 0x0102_a614),
                (0x3c, 4, 0x0102_a600),
                (0x48, 4, 0x0102_a5e8),
                (0x58, 4, 0x0102_a5c8),
                (0x60, 4, 0x0102_a5b4),
                (0x68, 4, 0x0102_a59c),
                (0x70, 4, 0x0102_a588),
                (0x14c, 4, 0x0102_a574),
                (0x154, 4, 0x0102_a560),
                (0x15c, 4, 0x0102_a54c),
                (0x164, 4, 0x0102_a538),
                (0x16c, 4, 0x0102_a524),
            ],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00512ba0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_2bd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_2ba0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_2ba0)
                .collect();
            let mut expected = vec![(0x0051_2bd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
