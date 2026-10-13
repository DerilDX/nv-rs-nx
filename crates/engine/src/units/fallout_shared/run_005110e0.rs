//! `fallout shared/run_005110e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00511950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00511950 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4d (0x3c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00511950(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_a0a4);
    e.mem.set_u32(base + 0x18, 0x0102_a07c);
    e.call(0x004f_15a0, &args![this, 0x4d]);
    e.mem.set_u32(base + 0x38, 0x0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_1950, fn_00511950(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00511950_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_1950,
            size: 0x3c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4d)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a0a4),
                (0x18, 4, 0x0102_a07c),
                (0x38, 4, 0x0),
            ],
        });
    }
}
