//! `fallout shared/bgsmenuicon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590210 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSMenuIcon::BGSMenuIcon` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 5 (0x24 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgsmenuicon_bgsmenuicon(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2c20, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_3654);
    e.mem.set_u32(base + 0x18, 0x0103_3634);
    e.call(0x0048_3710, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x5]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_0210, bgsmenuicon_bgsmenuicon(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgsmenuicon_bgsmenuicon_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_0210,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2c20, &[A::T(0x18)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5)]),
            ],
            stores: &[(0x0, 4, 0x0103_3654), (0x18, 4, 0x0103_3634)],
        });
    }
}
