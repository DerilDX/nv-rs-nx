//! `fallout shared/run_00599550` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00599c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x72 (0x30 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_449c`) is `TESAmmoEffect`, so this is the constructor of `TESAmmoEffect`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00599c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_449c);
    e.mem.set_u32(base + 0x18, 0x0103_4484);
    e.call(0x0040_3d30, &args![this, base + 0x24, 0x0u32, 0xcu32]);
    e.call(0x004f_15a0, &args![this, 0x72u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_9c40, fn_00599c40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00599c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_9c40,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_3d30, &[A::T(0x0), A::T(0x24), A::V(0x0), A::V(0xc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x72)]),
            ],
            stores: &[(0x0, 4, 0x0103_449c), (0x18, 4, 0x0103_4484)],
        });
    }
}
