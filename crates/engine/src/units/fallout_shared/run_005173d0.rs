//! `fallout shared/run_005173d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005173e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005173e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1c (0x90 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005173e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0048_cc50, &args![base + 0x5c]);
    e.call(0x0047_5350, &args![base + 0x68]);
    e.call(0x0051_0620, &args![base + 0x70]);
    e.mem.set_u32(base, 0x0102_b1fc);
    e.mem.set_u32(base + 0x30, 0x0102_b1e4);
    e.mem.set_u32(base + 0x3c, 0x0102_b1c0);
    e.mem.set_u32(base + 0x5c, 0x0102_b1ac);
    e.mem.set_u32(base + 0x68, 0x0102_b198);
    e.mem.set_u32(base + 0x70, 0x0102_b188);
    e.call(0x0096_a2d0, &args![base + 0x88]);
    e.call(0x004f_15a0, &args![this, 0x1c]);
    e.call(0x0051_75d0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_73e0, fn_005173e0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005173e0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_73e0,
            size: 0x90,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0048_cc50, &[A::T(0x5c)]),
                (0x0047_5350, &[A::T(0x68)]),
                (0x0051_0620, &[A::T(0x70)]),
                (0x0096_a2d0, &[A::T(0x88)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1c)]),
                (0x0051_75d0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b1fc),
                (0x30, 4, 0x0102_b1e4),
                (0x3c, 4, 0x0102_b1c0),
                (0x5c, 4, 0x0102_b1ac),
                (0x68, 4, 0x0102_b198),
                (0x70, 4, 0x0102_b188),
            ],
        });
    }
}
