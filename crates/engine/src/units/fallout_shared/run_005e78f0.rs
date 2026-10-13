//! `fallout shared/run_005e78f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e7a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e7a90 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x30 (0x40 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e7a90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0047_9790, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_664c);
    e.mem.set_u32(base + 0x30, 0x0104_6638);
    e.call(0x004f_15a0, &args![this, 0x30]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_7a90, fn_005e7a90(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e7a90_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_7a90,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0047_9790, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x30)]),
            ],
            stores: &[(0x0, 4, 0x0104_664c), (0x30, 4, 0x0104_6638)],
        });
    }
}
