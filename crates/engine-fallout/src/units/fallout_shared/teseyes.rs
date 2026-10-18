//! `fallout shared/teseyes.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fc050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc050 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESEyes::TESEyes` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0xb (0x34 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn teseyes_teseyes(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_973c);
    e.mem.set_u32(base + 0x18, 0x0104_9724);
    e.mem.set_u32(base + 0x24, 0x0104_9704);
    e.call(0x004f_15a0, &args![this, 0xb]);
    e.call(0x005f_c1b0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_c050, teseyes_teseyes(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn teseyes_teseyes_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_c050,
            size: 0x34,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x24)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xb)]),
                (0x005f_c1b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_973c),
                (0x18, 4, 0x0104_9724),
                (0x24, 4, 0x0104_9704),
            ],
        });
    }
}
