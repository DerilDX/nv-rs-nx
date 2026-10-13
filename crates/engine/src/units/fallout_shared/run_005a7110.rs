//! `fallout shared/run_005a7110` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a7160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6e (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_6854`) is `TESLoadScreenType`, so this is the constructor of `TESLoadScreenType`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a7160(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_6854);
    e.call(0x005a_71d0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x6eu32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005a_7160, fn_005a7160(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005a7160_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_7160,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x005a_71d0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6e)]),
            ],
            stores: &[(0x0, 4, 0x0103_6854)],
        });
    }
}
