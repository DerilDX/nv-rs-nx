//! `fallout shared/run_004f9b80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f9cd0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x52 (0x24 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004f9cd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_4834);
    e.mem.set_u32(base + 0x18, 0x0102_4818);
    e.call(0x0096_a2d0, &args![base + 0x1c]);
    e.call(0x004f_15a0, &args![this, 0x52]);
    e.call(0x0048_3710, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_9cd0, fn_004f9cd0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f9cd0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_9cd0,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x1c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x52)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_4834), (0x18, 4, 0x0102_4818)],
        });
    }
}
