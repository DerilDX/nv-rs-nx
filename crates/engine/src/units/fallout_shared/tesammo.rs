//! `fallout shared/tesammo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005026b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005026b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESAmmo::TESAmmo` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x29 (0xdc bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesammo_tesammo(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_50b0, &args![base + 0x80]);
    e.call(0x0047_5350, &args![base + 0x88]);
    e.call(0x0047_b4f0, &args![base + 0x90]);
    e.call(0x0048_cc50, &args![base + 0x9c]);
    e.mem.set_u32(base, 0x0102_6064);
    e.mem.set_u32(base + 0x30, 0x0102_604c);
    e.mem.set_u32(base + 0x3c, 0x0102_6028);
    e.mem.set_u32(base + 0x5c, 0x0102_6008);
    e.mem.set_u32(base + 0x68, 0x0102_5ff4);
    e.mem.set_u32(base + 0x78, 0x0102_5fd4);
    e.mem.set_u32(base + 0x80, 0x0102_5fc0);
    e.mem.set_u32(base + 0x88, 0x0102_5fac);
    e.mem.set_u32(base + 0x90, 0x0102_5f98);
    e.mem.set_u32(base + 0x9c, 0x0102_5f84);
    e.call(0x0040_37b0, &args![base + 0xc4]);
    e.call(0x0040_37b0, &args![base + 0xcc]);
    e.call(0x0096_a2d0, &args![base + 0xd4]);
    e.call(0x004f_15a0, &args![this, 0x29]);
    e.call(0x0050_29f0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_26b0, tesammo_tesammo(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesammo_tesammo_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_26b0,
            size: 0xdc,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_50b0, &[A::T(0x80)]),
                (0x0047_5350, &[A::T(0x88)]),
                (0x0047_b4f0, &[A::T(0x90)]),
                (0x0048_cc50, &[A::T(0x9c)]),
                (0x0040_37b0, &[A::T(0xc4)]),
                (0x0040_37b0, &[A::T(0xcc)]),
                (0x0096_a2d0, &[A::T(0xd4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x29)]),
                (0x0050_29f0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6064),
                (0x30, 4, 0x0102_604c),
                (0x3c, 4, 0x0102_6028),
                (0x5c, 4, 0x0102_6008),
                (0x68, 4, 0x0102_5ff4),
                (0x78, 4, 0x0102_5fd4),
                (0x80, 4, 0x0102_5fc0),
                (0x88, 4, 0x0102_5fac),
                (0x90, 4, 0x0102_5f98),
                (0x9c, 4, 0x0102_5f84),
            ],
        });
    }
}
