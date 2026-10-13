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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_5440, fn_00515440(Ptr) -> Ptr)]
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
