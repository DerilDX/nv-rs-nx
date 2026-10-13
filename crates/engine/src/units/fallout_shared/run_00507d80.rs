//! `fallout shared/run_00507d80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00508450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508450 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x26 (0x9c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00508450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_b560, &args![this]);
    e.call(0x0051_04f0, &args![base + 0xc]);
    e.mem.set_u32(base, 0x0102_6bd0);
    e.mem.set_u32(base + 0xc, 0x0102_6a4c);
    e.mem.set_u32(base + 0x3c, 0x0102_6a34);
    e.mem.set_u32(base + 0x48, 0x0102_6a10);
    e.mem.set_u32(base + 0x68, 0x0102_69fc);
    e.mem.set_u32(base + 0x74, 0x0102_69e8);
    e.mem.set_u32(base + 0x7c, 0x0102_69d8);
    e.call(0x004f_15a0, &args![base + 0xc, 0x26]);
    e.call(0x0048_4ab0, &args![base + 0xc, 0x1]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_8450, fn_00508450(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00508450_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_8450,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_b560, &[A::T(0x0)]),
                (0x0051_04f0, &[A::T(0xc)]),
                (0x004f_15a0, &[A::T(0xc), A::V(0x26)]),
                (0x0048_4ab0, &[A::T(0xc), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6bd0),
                (0xc, 4, 0x0102_6a4c),
                (0x3c, 4, 0x0102_6a34),
                (0x48, 4, 0x0102_6a10),
                (0x68, 4, 0x0102_69fc),
                (0x74, 4, 0x0102_69e8),
                (0x7c, 4, 0x0102_69d8),
            ],
        });
    }
}
