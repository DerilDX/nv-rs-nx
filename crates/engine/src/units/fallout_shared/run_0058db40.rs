//! `fallout shared/run_0058db40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058db40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058db40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5e (0x78 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_2f6c`) is `BGSImpactData`, so this is the constructor of `BGSImpactData`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058db40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_2f6c);
    e.mem.set_u32(base + 0x18, 0x0103_2f44);
    e.call(0x0059_3240, &args![base + 0x54]);
    e.call(0x004f_15a0, &args![this, 0x5eu32]);
    e.call(0x0058_dc90, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_db40, fn_0058db40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058db40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_db40,
            size: 0x78,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0059_3240, &[A::T(0x54)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5e)]),
                (0x0058_dc90, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_2f6c), (0x18, 4, 0x0103_2f44)],
        });
    }
}
