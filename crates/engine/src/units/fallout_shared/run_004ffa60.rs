//! `fallout shared/run_004ffa60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004ffaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ffaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x17 (0xa8 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004ffaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.call(0x0048_2c50, &args![base + 0x90]);
    e.mem.set_u32(base, 0x0102_5914);
    e.mem.set_u32(base + 0x30, 0x0102_58fc);
    e.mem.set_u32(base + 0x3c, 0x0102_58d8);
    e.mem.set_u32(base + 0x5c, 0x0102_58c4);
    e.mem.set_u32(base + 0x68, 0x0102_58b0);
    e.mem.set_u32(base + 0x70, 0x0102_58a0);
    e.mem.set_u32(base + 0x90, 0x0102_5888);
    e.call(0x0096_a2d0, &args![base + 0x98]);
    e.call(0x004f_15a0, &args![this, 0x17]);
    e.call(0x004f_fc50, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_faa0, fn_004ffaa0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004ffaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_faa0,
            size: 0xa8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x0048_2c50, &[A::T(0x90)]),
                (0x0096_a2d0, &[A::T(0x98)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x17)]),
                (0x004f_fc50, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_5914),
                (0x30, 4, 0x0102_58fc),
                (0x3c, 4, 0x0102_58d8),
                (0x5c, 4, 0x0102_58c4),
                (0x68, 4, 0x0102_58b0),
                (0x70, 4, 0x0102_58a0),
                (0x90, 4, 0x0102_5888),
            ],
        });
    }
}
