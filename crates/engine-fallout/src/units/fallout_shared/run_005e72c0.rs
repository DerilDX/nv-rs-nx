//! `fallout shared/run_005e72c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e72c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e72c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 9 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e72c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_a3d0, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_64b4);
    e.mem.set_u32(base + 0x18, 0x0104_64a0);
    e.mem.set_u32(base + 0x24, 0x0104_647c);
    e.call(0x0096_a2d0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x9]);
    e.call(0x005e_7440, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_72c0, fn_005e72c0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e72c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_72c0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_a3d0, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x9)]),
                (0x005e_7440, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_64b4),
                (0x18, 4, 0x0104_64a0),
                (0x24, 4, 0x0104_647c),
            ],
        });
    }
}
