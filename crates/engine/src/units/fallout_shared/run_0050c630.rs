//! `fallout shared/run_0050c630` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050c6c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x1e (0xc8 bytes; `tesdatahandler.rs` lists it); the RTTI name of the
/// vtable it stores at `+0` (`01028ee4`) is `.?AVTESObjectLIGH@@`. It runs
/// the base constructor (`0050f6f0`) and the component constructors by
/// address (`00402d00` at `+0x30`, `0048a3d0` at `+0x3c`, `00402c20` at
/// `+0x5c`, `0047a130` at `+0x68`, `0048cc50` at `+0x78`, `0048eac0` at
/// `+0x84`, `0048e7a0` at `+0x8c`, `00475350` at `+0x94`), stores the nine
/// vtable pointers, runs `00416870` on `+0xbc` with three `float` zeros,
/// registers the form type 0x1e (`004f15a0`) and calls `0050c990`. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_0050c6c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_cc50, &args![base + 0x78]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0048_e7a0, &args![base + 0x8c]);
    e.call(0x0047_5350, &args![base + 0x94]);
    e.mem.set_u32(base, 0x0102_8ee4);
    e.mem.set_u32(base + 0x30, 0x0102_8ecc);
    e.mem.set_u32(base + 0x3c, 0x0102_8ea8);
    e.mem.set_u32(base + 0x5c, 0x0102_8e88);
    e.mem.set_u32(base + 0x68, 0x0102_8e74);
    e.mem.set_u32(base + 0x78, 0x0102_8e60);
    e.mem.set_u32(base + 0x84, 0x0102_8e4c);
    e.mem.set_u32(base + 0x8c, 0x0102_8e2c);
    e.mem.set_u32(base + 0x94, 0x0102_8e18);
    e.call(0x0041_6870, &args![base + 0xbc, 0.0f32, 0.0f32, 0.0f32]);
    e.call(0x004f_15a0, &args![this, 0x1e]);
    e.call(0x0050_c990, &args![this]);
    this
}

// Translated from 0050c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050c870` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050c840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_c870, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050c6c0, fn_0050c6c0(Ptr) -> Ptr),
        entry!(0x0050_c840, fn_0050c840(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050c6c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_c6c0,
            size: 0xc8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_cc50, &[A::T(0x78)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0048_e7a0, &[A::T(0x8c)]),
                (0x0047_5350, &[A::T(0x94)]),
                (0x0041_6870, &[A::T(0xbc), A::V(0), A::V(0), A::V(0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1e)]),
                (0x0050_c990, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_8ee4),
                (0x30, 4, 0x0102_8ecc),
                (0x3c, 4, 0x0102_8ea8),
                (0x5c, 4, 0x0102_8e88),
                (0x68, 4, 0x0102_8e74),
                (0x78, 4, 0x0102_8e60),
                (0x84, 4, 0x0102_8e4c),
                (0x8c, 4, 0x0102_8e2c),
                (0x94, 4, 0x0102_8e18),
            ],
        });
    }
    #[test]
    fn at_0050c840_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_c870, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_c840, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_c840)
                .collect();
            let mut expected = vec![(0x0050_c870, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
