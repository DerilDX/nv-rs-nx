//! `fallout shared/run_005805c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005807f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005807f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x35 (0x36c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005807f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0058_1fd0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_168c);
    e.mem.set_u32(base + 0x18, 0x0103_1674);
    e.call(
        0x00ec_782f,
        &args![this, base + 0x34, 0xc, 0x4, 0x0051_b9b0, 0x0040_2c40],
    );
    e.call(0x0048_8f50, &args![base + 0xc8]);
    e.call(0x0068_0890, &args![base + 0x1f8]);
    e.call(0x004f_15a0, &args![this, 0x35]);
    e.call(0x0058_1b40, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_07f0, fn_005807f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005807f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_07f0,
            size: 0x36c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0058_1fd0, &[A::T(0x18)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x0),
                        A::T(0x34),
                        A::V(0xc),
                        A::V(0x4),
                        A::V(0x0051_b9b0),
                        A::V(0x0040_2c40),
                    ],
                ),
                (0x0048_8f50, &[A::T(0xc8)]),
                (0x0068_0890, &[A::T(0x1f8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x35)]),
                (0x0058_1b40, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_168c), (0x18, 4, 0x0103_1674)],
        });
    }
}
