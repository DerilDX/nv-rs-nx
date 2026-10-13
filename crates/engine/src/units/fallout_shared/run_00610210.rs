//! `fallout shared/run_00610210` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00610210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00610210 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0xc (0x524 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00610210(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_cfe0, &args![base + 0x2c]);
    e.call(0x0048_ba00, &args![base + 0x40]);
    e.mem.set_u32(base, 0x0104_b4bc);
    e.mem.set_u32(base + 0x18, 0x0104_b4a4);
    e.mem.set_u32(base + 0x24, 0x0104_b48c);
    e.mem.set_u32(base + 0x2c, 0x0104_b46c);
    e.mem.set_u32(base + 0x40, 0x0104_b458);
    e.call(0x0047_ff70, &args![base + 0x74]);
    e.call(0x0047_ff70, &args![base + 0x80]);
    e.call(0x0096_a2d0, &args![base + 0x8c]);
    e.call(0x0096_a2d0, &args![base + 0xa8]);
    e.call(
        0x00ec_782f,
        &args![base + 0xb0, 0x18, 0x10, 0x0048_8f50, 0x0048_9070],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x230, 0xc, 0x10, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x2f0, 0xc, 0x6, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x338, 0x18, 0x6, 0x0048_8f50, 0x0048_9070],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x3c8, 0x18, 0x2, 0x0061_04a0, 0x0051_b950],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x3f8, 0x20, 0x4, 0x0044_9610, 0x0044_9680],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x478, 0x20, 0x4, 0x0044_9610, 0x0044_9680],
    );
    e.call(0x0040_37b0, &args![base + 0x50c]);
    e.call(0x0060_b9e0, &args![base + 0x514, 0x0, 0x1]);
    e.call(0x004f_15a0, &args![this, 0xc]);
    e.call(0x0061_06a0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_0210, fn_00610210(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00610210_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_0210,
            size: 0x524,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_cfe0, &[A::T(0x2c)]),
                (0x0048_ba00, &[A::T(0x40)]),
                (0x0047_ff70, &[A::T(0x74)]),
                (0x0047_ff70, &[A::T(0x80)]),
                (0x0096_a2d0, &[A::T(0x8c)]),
                (0x0096_a2d0, &[A::T(0xa8)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0xb0),
                        A::V(0x18),
                        A::V(0x10),
                        A::V(0x0048_8f50),
                        A::V(0x0048_9070),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x230),
                        A::V(0xc),
                        A::V(0x10),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x2f0),
                        A::V(0xc),
                        A::V(0x6),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x338),
                        A::V(0x18),
                        A::V(0x6),
                        A::V(0x0048_8f50),
                        A::V(0x0048_9070),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x3c8),
                        A::V(0x18),
                        A::V(0x2),
                        A::V(0x0061_04a0),
                        A::V(0x0051_b950),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x3f8),
                        A::V(0x20),
                        A::V(0x4),
                        A::V(0x0044_9610),
                        A::V(0x0044_9680),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x478),
                        A::V(0x20),
                        A::V(0x4),
                        A::V(0x0044_9610),
                        A::V(0x0044_9680),
                    ],
                ),
                (0x0040_37b0, &[A::T(0x50c)]),
                (0x0060_b9e0, &[A::T(0x514), A::V(0x0), A::V(0x1)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xc)]),
                (0x0061_06a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_b4bc),
                (0x18, 4, 0x0104_b4a4),
                (0x24, 4, 0x0104_b48c),
                (0x2c, 4, 0x0104_b46c),
                (0x40, 4, 0x0104_b458),
            ],
        });
    }
}
