//! `fallout shared/run_00503bb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00503bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503bd0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6c (0x9c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_63dc`) is `TESCasinoChips`, so this is the constructor of `TESCasinoChips`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00503bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_5350, &args![base + 0x80]);
    e.call(0x0047_b4f0, &args![base + 0x88]);
    e.mem.set_u32(base, 0x0102_63dc);
    e.mem.set_u32(base + 0x30, 0x0102_63c4);
    e.mem.set_u32(base + 0x3c, 0x0102_63a0);
    e.mem.set_u32(base + 0x5c, 0x0102_6380);
    e.mem.set_u32(base + 0x68, 0x0102_636c);
    e.mem.set_u32(base + 0x78, 0x0102_634c);
    e.mem.set_u32(base + 0x80, 0x0102_6338);
    e.mem.set_u32(base + 0x88, 0x0102_6324);
    e.call(0x0040_37b0, &args![base + 0x94]);
    e.call(0x004f_15a0, &args![this, 0x6cu32]);
    e.call(0x0050_3e70, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_3bd0, fn_00503bd0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00503bd0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_3bd0,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_5350, &[A::T(0x80)]),
                (0x0047_b4f0, &[A::T(0x88)]),
                (0x0040_37b0, &[A::T(0x94)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6c)]),
                (0x0050_3e70, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_63dc),
                (0x30, 4, 0x0102_63c4),
                (0x3c, 4, 0x0102_63a0),
                (0x5c, 4, 0x0102_6380),
                (0x68, 4, 0x0102_636c),
                (0x78, 4, 0x0102_634c),
                (0x80, 4, 0x0102_6338),
                (0x88, 4, 0x0102_6324),
            ],
        });
    }
}
