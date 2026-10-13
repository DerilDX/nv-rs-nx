//! `fallout shared/magic/run_004098d0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004098f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004098f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1d (0xa4 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004098f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2b40, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.call(0x0040_2c20, &args![base + 0x6c]);
    e.call(0x0048_cc50, &args![base + 0x78]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0047_9320, &args![base + 0x8c]);
    e.call(0x0047_5350, &args![base + 0x94]);
    e.mem.set_u32(base, 0x0101_3284);
    e.mem.set_u32(base + 0x30, 0x0101_323c);
    e.mem.set_u32(base + 0x3c, 0x0101_3224);
    e.mem.set_u32(base + 0x4c, 0x0101_3200);
    e.mem.set_u32(base + 0x6c, 0x0101_31e0);
    e.mem.set_u32(base + 0x78, 0x0101_31cc);
    e.mem.set_u32(base + 0x84, 0x0101_31b8);
    e.mem.set_u32(base + 0x8c, 0x0101_31a4);
    e.mem.set_u32(base + 0x94, 0x0101_3190);
    e.call(0x004f_15a0, &args![this, 0x1d]);
    e.call(0x0040_9c40, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_98f0, fn_004098f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004098f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_98f0,
            size: 0xa4,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2b40, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x0040_2c20, &[A::T(0x6c)]),
                (0x0048_cc50, &[A::T(0x78)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0047_9320, &[A::T(0x8c)]),
                (0x0047_5350, &[A::T(0x94)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1d)]),
                (0x0040_9c40, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_3284),
                (0x30, 4, 0x0101_323c),
                (0x3c, 4, 0x0101_3224),
                (0x4c, 4, 0x0101_3200),
                (0x6c, 4, 0x0101_31e0),
                (0x78, 4, 0x0101_31cc),
                (0x84, 4, 0x0101_31b8),
                (0x8c, 4, 0x0101_31a4),
                (0x94, 4, 0x0101_3190),
            ],
        });
    }
}
