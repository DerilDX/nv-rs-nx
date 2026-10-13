//! `fallout shared/tescaravan.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059ae60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059ae60 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x74 (0xcc bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_49b4`) is `TESCaravanMoney`, so this is the constructor of `TESCaravanMoney`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0059ae60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_b4f0, &args![base + 0x80]);
    e.mem.set_u32(base, 0x0103_49b4);
    e.mem.set_u32(base + 0x30, 0x0103_499c);
    e.mem.set_u32(base + 0x3c, 0x0103_4978);
    e.mem.set_u32(base + 0x5c, 0x0103_4958);
    e.mem.set_u32(base + 0x68, 0x0103_4944);
    e.mem.set_u32(base + 0x78, 0x0103_4924);
    e.mem.set_u32(base + 0x80, 0x0103_4910);
    e.call(
        0x00ec_782f,
        &args![base + 0x8c, 0x20u32, 0x2u32, 0x0048_a3d0u32, 0x0048_a490u32],
    );
    e.call(0x004f_15a0, &args![this, 0x74u32]);
    e.call(0x0059_b120, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_ae60, fn_0059ae60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0059ae60_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_ae60,
            size: 0xcc,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_b4f0, &[A::T(0x80)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x8c),
                        A::V(0x20),
                        A::V(0x2),
                        A::V(0x0048_a3d0),
                        A::V(0x0048_a490),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0x74)]),
                (0x0059_b120, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_49b4),
                (0x30, 4, 0x0103_499c),
                (0x3c, 4, 0x0103_4978),
                (0x5c, 4, 0x0103_4958),
                (0x68, 4, 0x0103_4944),
                (0x78, 4, 0x0103_4924),
                (0x80, 4, 0x0103_4910),
            ],
        });
    }
}
