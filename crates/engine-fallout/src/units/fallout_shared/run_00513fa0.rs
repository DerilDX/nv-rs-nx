//! `fallout shared/run_00513fa0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00514500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00514500 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x19 (0xc4 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00514500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_30a0, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x84]);
    e.call(0x0048_eac0, &args![base + 0x8c]);
    e.call(0x0048_2c50, &args![base + 0x94]);
    e.call(0x0047_5350, &args![base + 0x9c]);
    e.call(0x0047_a130, &args![base + 0xa4]);
    e.call(0x0047_b4f0, &args![base + 0xb4]);
    e.mem.set_u32(base, 0x0102_a9c4);
    e.mem.set_u32(base + 0x30, 0x0102_a9b0);
    e.mem.set_u32(base + 0x3c, 0x0102_a98c);
    e.mem.set_u32(base + 0x5c, 0x0102_a96c);
    e.mem.set_u32(base + 0x68, 0x0102_a958);
    e.mem.set_u32(base + 0x74, 0x0102_a940);
    e.mem.set_u32(base + 0x84, 0x0102_a920);
    e.mem.set_u32(base + 0x8c, 0x0102_a90c);
    e.mem.set_u32(base + 0x94, 0x0102_a8f4);
    e.mem.set_u32(base + 0x9c, 0x0102_a8e0);
    e.mem.set_u32(base + 0xa4, 0x0102_a8cc);
    e.mem.set_u32(base + 0xb4, 0x0102_a8b8);
    e.call(0x004f_15a0, &args![this, 0x19]);
    e.call(0x0051_4830, &args![this]);
    this
}

// Translated from 005144f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00514470`. `00514470` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005144f0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_4470,
        &args![this.addr().wrapping_sub(0x58), argument],
    )
    .u16()
}

// Translated from 005146a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005146d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005146a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_46d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_4500, fn_00514500(Ptr) -> Ptr),
        entry!(0x0051_44f0, fn_005144f0(Ptr, u32) -> u16),
        entry!(0x0051_46a0, fn_005146a0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00514500_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_4500,
            size: 0xc4,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_30a0, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x84)]),
                (0x0048_eac0, &[A::T(0x8c)]),
                (0x0048_2c50, &[A::T(0x94)]),
                (0x0047_5350, &[A::T(0x9c)]),
                (0x0047_a130, &[A::T(0xa4)]),
                (0x0047_b4f0, &[A::T(0xb4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x19)]),
                (0x0051_4830, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a9c4),
                (0x30, 4, 0x0102_a9b0),
                (0x3c, 4, 0x0102_a98c),
                (0x5c, 4, 0x0102_a96c),
                (0x68, 4, 0x0102_a958),
                (0x74, 4, 0x0102_a940),
                (0x84, 4, 0x0102_a920),
                (0x8c, 4, 0x0102_a90c),
                (0x94, 4, 0x0102_a8f4),
                (0x9c, 4, 0x0102_a8e0),
                (0xa4, 4, 0x0102_a8cc),
                (0xb4, 4, 0x0102_a8b8),
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
    fn le81_thunk_005144f0() {
        assert_eq!(check(0x0051_44f0, 0x0051_4470, 0x58, &[0x77]).u16(), 0xbeef);
    }
    #[test]
    fn at_005146a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_46d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_46a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_46a0)
                .collect();
            let mut expected = vec![(0x0051_46d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
