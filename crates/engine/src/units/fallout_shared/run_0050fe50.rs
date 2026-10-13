//! `fallout shared/run_0050fe50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005104f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005104f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x15 (0x90 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005104f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0048_cc50, &args![base + 0x5c]);
    e.call(0x0047_5350, &args![base + 0x68]);
    e.call(0x0051_0620, &args![base + 0x70]);
    e.mem.set_u32(base, 0x0102_9d5c);
    e.mem.set_u32(base + 0x30, 0x0102_9d44);
    e.mem.set_u32(base + 0x3c, 0x0102_9d20);
    e.mem.set_u32(base + 0x5c, 0x0102_9d0c);
    e.mem.set_u32(base + 0x68, 0x0102_9cf8);
    e.mem.set_u32(base + 0x70, 0x0102_9ce8);
    e.call(0x0040_37b0, &args![base + 0x88]);
    e.call(0x004f_15a0, &args![this, 0x15]);
    e.call(0x0051_0710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_04f0, fn_005104f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005104f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_04f0,
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
                (0x0040_37b0, &[A::T(0x88)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x15)]),
                (0x0051_0710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_9d5c),
                (0x30, 4, 0x0102_9d44),
                (0x3c, 4, 0x0102_9d20),
                (0x5c, 4, 0x0102_9d0c),
                (0x68, 4, 0x0102_9cf8),
                (0x70, 4, 0x0102_9ce8),
            ],
        });
    }
}
