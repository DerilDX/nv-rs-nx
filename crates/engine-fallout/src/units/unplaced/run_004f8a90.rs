//! `(unplaced)/run_004f8a90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f8b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8b00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x58 (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_4214`) is `BGSAddonNode`, so this is the constructor of `BGSAddonNode`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004f8b00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_4214);
    e.mem.set_u32(base + 0x30, 0x0102_41f0);
    e.mem.set_u32(base + 0x50, 0xffff_ffff);
    e.call(0x004f_15a0, &args![this, 0x58u32]);
    e.call(0x004f_8c50, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_8b00, fn_004f8b00(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f8b00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_8b00,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x58)]),
                (0x004f_8c50, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4214),
                (0x30, 4, 0x0102_41f0),
                (0x50, 4, 0xffff_ffff),
            ],
        });
    }
}
