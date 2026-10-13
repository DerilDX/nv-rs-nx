//! `fallout shared/run_005904a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590530 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x62 (0x40 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_37c4`) is `BGSMessage`, so this is the constructor of `BGSMessage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00590530(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_37c4);
    e.mem.set_u32(base + 0x18, 0x0103_37ac);
    e.mem.set_u32(base + 0x24, 0x0103_3794);
    e.call(0x0096_a2d0, &args![base + 0x30]);
    e.call(0x004f_15a0, &args![this, 0x62u32]);
    e.call(0x0059_06a0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_0530, fn_00590530(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00590530_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_0530,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x62)]),
                (0x0059_06a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_37c4),
                (0x18, 4, 0x0103_37ac),
                (0x24, 4, 0x0103_3794),
            ],
        });
    }
}
