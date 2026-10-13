//! `fallout shared/run_0058bb50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058bd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058bd30 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5b (0x78 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_27f4`) is `BGSCameraShot`, so this is the constructor of `BGSCameraShot`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058bd30(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x0048_7420, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0103_27f4);
    e.mem.set_u32(base + 0x18, 0x0103_27cc);
    e.mem.set_u32(base + 0x30, 0x0103_27b8);
    e.call(0x0063_3c90, &args![base + 0x60, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x64, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x6c, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x70, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x5bu32]);
    e.call(0x0058_bf50, &args![this]);
    e.call(0x0066_b0d0, &args![base + 0x60, 0x0u32]);
    e.call(0x0066_b0d0, &args![base + 0x64, 0x0u32]);
    e.call(0x0066_b0d0, &args![base + 0x6c, 0x0u32]);
    e.mem.set_u32(base + 0x68, 0x0);
    e.call(0x0066_b0d0, &args![base + 0x70, 0x0u32]);
    e.mem.set_u8(base + 0x74, 0xff);
    e.mem.set_u8(base + 0x75, 0x0);
    e.mem.set_u8(base + 0x76, 0x0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_bd30, fn_0058bd30(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058bd30_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_bd30,
            size: 0x78,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0048_7420, &[A::T(0x30)]),
                (0x0063_3c90, &[A::T(0x60), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x64), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x6c), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x70), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5b)]),
                (0x0058_bf50, &[A::T(0x0)]),
                (0x0066_b0d0, &[A::T(0x60), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x64), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x6c), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x70), A::V(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_27f4),
                (0x18, 4, 0x0103_27cc),
                (0x30, 4, 0x0103_27b8),
                (0x68, 4, 0x0),
                (0x74, 1, 0xff),
                (0x75, 1, 0x0),
                (0x76, 1, 0x0),
            ],
        });
    }
}
