//! `fallout shared/run_005904a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590530 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x62 (0x40 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_37c4`) is `BGSMessage`, so this is the constructor of `BGSMessage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00590530(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_37c4);
    e.mem.set_u32(base + 0x18, 0x0103_37ac);
    e.mem.set_u32(base + 0x24, 0x0103_3794);
    e.call(0x0096_a2d0, &args![base + 0x30]);
    e.call(0x004f_15a0, &args![this, 0x62u32]);
    e.call(0x0059_06a0, &args![this]);
    this
}

// Translated from 005905e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00590610` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005905e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_0610, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_0530, fn_00590530(Ptr) -> Ptr),
        entry!(0x0059_05e0, fn_005905e0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00590530_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_0530,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x62)]),
                (0x0059_06a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_37c4),
                (0x18, 4, 0x0103_37ac),
                (0x24, 4, 0x0103_3794),
            ],
        });
    }
    #[test]
    fn at_005905e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_0610, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_05e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_05e0)
                .collect();
            let mut expected = vec![(0x0059_0610, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
