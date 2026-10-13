//! `fallout shared/bgsplaceablewater.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fc1b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSPlaceableWater::BGSPlaceableWater` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x23 (0x50 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgsplaceablewater_bgsplaceablewater(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_4f4c);
    e.mem.set_u32(base + 0x30, 0x0102_4f28);
    e.call(0x0040_19f0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x23]);
    e.call(0x004f_c2e0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_c1b0, bgsplaceablewater_bgsplaceablewater(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgsplaceablewater_bgsplaceablewater_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_c1b0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x0040_19f0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x23)]),
                (0x004f_c2e0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_4f4c), (0x30, 4, 0x0102_4f28)],
        });
    }
}
