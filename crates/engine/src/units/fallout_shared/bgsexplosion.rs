//! `fallout shared/bgsexplosion.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fabf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fabf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x51 (0xa8 bytes); `ECX` is the object.
/// The engine map names it `BGSExplosion::BGSExplosion` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_4a94`) is `BGSExplosion`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fabf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_8f50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x54]);
    e.call(0x004f_9d90, &args![base + 0x64]);
    e.call(0x0048_7420, &args![base + 0x68]);
    e.mem.set_u32(base, 0x0102_4a94);
    e.mem.set_u32(base + 0x30, 0x0102_4a80);
    e.mem.set_u32(base + 0x3c, 0x0102_4a5c);
    e.mem.set_u32(base + 0x54, 0x0102_4a44);
    e.mem.set_u32(base + 0x64, 0x0102_4a2c);
    e.mem.set_u32(base + 0x68, 0x0102_4a18);
    e.call(0x004f_aa00, &args![base + 0x74]);
    e.call(0x004f_15a0, &args![this, 0x51u32]);
    e.call(0x004f_ade0, &args![this]);
    e.call(0x0041_fd00, &args![base + 0x54, 0x2u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_abf0, fn_004fabf0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fabf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_abf0,
            size: 0xa8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_8f50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x54)]),
                (0x004f_9d90, &[A::T(0x64)]),
                (0x0048_7420, &[A::T(0x68)]),
                (0x004f_aa00, &[A::T(0x74)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x51)]),
                (0x004f_ade0, &[A::T(0x0)]),
                (0x0041_fd00, &[A::T(0x54), A::V(0x2)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4a94),
                (0x30, 4, 0x0102_4a80),
                (0x3c, 4, 0x0102_4a5c),
                (0x54, 4, 0x0102_4a44),
                (0x64, 4, 0x0102_4a2c),
                (0x68, 4, 0x0102_4a18),
            ],
        });
    }
}
