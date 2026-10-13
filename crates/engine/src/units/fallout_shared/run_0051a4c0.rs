//! `fallout shared/run_0051a4c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051a6a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1f (0xac bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051a6a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x74]);
    e.call(0x0048_eac0, &args![base + 0x7c]);
    e.call(0x0047_5350, &args![base + 0x84]);
    e.call(0x0047_a130, &args![base + 0x8c]);
    e.call(0x0047_b4f0, &args![base + 0x9c]);
    e.mem.set_u32(base, 0x0102_b844);
    e.mem.set_u32(base + 0x30, 0x0102_b82c);
    e.mem.set_u32(base + 0x3c, 0x0102_b808);
    e.mem.set_u32(base + 0x5c, 0x0102_b7e8);
    e.mem.set_u32(base + 0x68, 0x0102_b7d4);
    e.mem.set_u32(base + 0x74, 0x0102_b7b4);
    e.mem.set_u32(base + 0x7c, 0x0102_b7a0);
    e.mem.set_u32(base + 0x84, 0x0102_b78c);
    e.mem.set_u32(base + 0x8c, 0x0102_b778);
    e.mem.set_u32(base + 0x9c, 0x0102_b764);
    e.call(0x004f_15a0, &args![this, 0x1f]);
    e.call(0x0051_a980, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_a6a0, fn_0051a6a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051a6a0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_a6a0,
            size: 0xac,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x74)]),
                (0x0048_eac0, &[A::T(0x7c)]),
                (0x0047_5350, &[A::T(0x84)]),
                (0x0047_a130, &[A::T(0x8c)]),
                (0x0047_b4f0, &[A::T(0x9c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1f)]),
                (0x0051_a980, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b844),
                (0x30, 4, 0x0102_b82c),
                (0x3c, 4, 0x0102_b808),
                (0x5c, 4, 0x0102_b7e8),
                (0x68, 4, 0x0102_b7d4),
                (0x74, 4, 0x0102_b7b4),
                (0x7c, 4, 0x0102_b7a0),
                (0x84, 4, 0x0102_b78c),
                (0x8c, 4, 0x0102_b778),
                (0x9c, 4, 0x0102_b764),
            ],
        });
    }
}
