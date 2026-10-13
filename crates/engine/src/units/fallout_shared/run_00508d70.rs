//! `fallout shared/run_00508d70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00508e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508e00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x27 (0x94 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00508e00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.mem.set_u32(base, 0x0102_6d0c);
    e.mem.set_u32(base + 0x30, 0x0102_6cf4);
    e.mem.set_u32(base + 0x3c, 0x0102_6cd0);
    e.mem.set_u32(base + 0x5c, 0x0102_6cbc);
    e.mem.set_u32(base + 0x68, 0x0102_6ca8);
    e.mem.set_u32(base + 0x70, 0x0102_6c98);
    e.call(0x004f_15a0, &args![this, 0x27]);
    e.call(0x0050_8f70, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_8e00, fn_00508e00(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00508e00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_8e00,
            size: 0x94,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x27)]),
                (0x0050_8f70, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6d0c),
                (0x30, 4, 0x0102_6cf4),
                (0x3c, 4, 0x0102_6cd0),
                (0x5c, 4, 0x0102_6cbc),
                (0x68, 4, 0x0102_6ca8),
                (0x70, 4, 0x0102_6c98),
            ],
        });
    }
}
