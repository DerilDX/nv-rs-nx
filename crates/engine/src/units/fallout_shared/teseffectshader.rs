//! `fallout shared/teseffectshader.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4f (0x170 bytes); `ECX` is the object.
/// The engine map names it `TESEffectShader::TESEffectShader` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_685c`) is `TESEffectShader`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00505c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_685c);
    e.call(0x0050_64d0, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x14c]);
    e.call(0x0048_e270, &args![base + 0x158]);
    e.call(0x0048_e270, &args![base + 0x164]);
    e.call(0x004f_15a0, &args![this, 0x4fu32]);
    e.call(0x0050_5db0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_5c40, fn_00505c40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00505c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_5c40,
            size: 0x170,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0050_64d0, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x14c)]),
                (0x0048_e270, &[A::T(0x158)]),
                (0x0048_e270, &[A::T(0x164)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4f)]),
                (0x0050_5db0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_685c)],
        });
    }
}
