//! `fallout shared/run_0052a980` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0052aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052aaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x36 (0x58 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0052aaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_d5c4);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x0058_22c0, &args![base + 0x30]);
    e.call(
        0x00ec_782f,
        &args![base + 0x38, 0xc, 0x2, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(0x004f_15a0, &args![this, 0x36]);
    e.call(0x0052_ac20, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_aaa0, fn_0052aaa0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0052aaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_aaa0,
            size: 0x58,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0058_22c0, &[A::T(0x30)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x38),
                        A::V(0xc),
                        A::V(0x2),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0x36)]),
                (0x0052_ac20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d5c4)],
        });
    }
}
