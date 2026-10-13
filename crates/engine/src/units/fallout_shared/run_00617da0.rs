//! `fallout shared/run_00617da0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00617da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617da0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0xf (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00617da0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_2c50, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x20]);
    e.mem.set_u32(base, 0x0104_cc0c);
    e.mem.set_u32(base + 0x18, 0x0104_cbf4);
    e.mem.set_u32(base + 0x20, 0x0104_cbd4);
    e.call(
        0x0040_1050,
        &args![this, base + 0x40, 0x8, 0x4, 0x0048_2c50],
    );
    e.call(0x004f_15a0, &args![this, 0xf]);
    e.call(0x0061_7e80, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_7da0, fn_00617da0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00617da0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_7da0,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_2c50, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x20)]),
                (
                    0x0040_1050,
                    &[
                        A::T(0x0),
                        A::T(0x40),
                        A::V(0x8),
                        A::V(0x4),
                        A::V(0x0048_2c50),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0xf)]),
                (0x0061_7e80, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_cc0c),
                (0x18, 4, 0x0104_cbf4),
                (0x20, 4, 0x0104_cbd4),
            ],
        });
    }
}
