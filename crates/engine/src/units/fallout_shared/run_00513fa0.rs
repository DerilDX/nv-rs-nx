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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_4500, fn_00514500(Ptr) -> Ptr)]
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
