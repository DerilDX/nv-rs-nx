//! `fallout shared/teslandtexture.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00540c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00540c50 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESLandTexture::TESLandTexture` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x12 (0x28 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn teslandtexture_teslandtexture(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_e6c4);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.call(0x004f_15a0, &args![this, 0x12]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    e.call(0x0054_0d80, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0054_0c50, teslandtexture_teslandtexture(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn teslandtexture_teslandtexture_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0054_0c50,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x20)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x12)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
                (0x0054_0d80, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_e6c4)],
        });
    }
}
