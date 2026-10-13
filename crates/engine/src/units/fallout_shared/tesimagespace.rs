//! `fallout shared/tesimagespace.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0052b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b310 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x53 (0xb0 bytes); `ECX` is the object.
/// The engine map names it `TESImageSpace::TESImageSpace` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_d7f4`) is `TESImageSpace`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0052b310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_d7f4);
    e.call(0x004f_15a0, &args![this, 0x53u32]);
    e.call(0x0052_b410, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_b310, fn_0052b310(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0052b310_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_b310,
            size: 0xb0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x53)]),
                (0x0052_b410, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d7f4)],
        });
    }
}
