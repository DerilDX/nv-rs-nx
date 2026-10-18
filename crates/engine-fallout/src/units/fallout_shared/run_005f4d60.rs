//! `fallout shared/run_005f4d60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f4e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f4e00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x71 (0x7c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_891c`) is `TESChallenge`, so this is the constructor of `TESChallenge`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005f4e00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_cc50, &args![base + 0x2c]);
    e.call(0x0040_2c20, &args![base + 0x38]);
    e.call(0x0047_a130, &args![base + 0x44]);
    e.mem.set_u32(base, 0x0104_891c);
    e.mem.set_u32(base + 0x18, 0x0104_8908);
    e.mem.set_u32(base + 0x24, 0x0104_88f0);
    e.mem.set_u32(base + 0x2c, 0x0104_88dc);
    e.mem.set_u32(base + 0x38, 0x0104_88bc);
    e.mem.set_u32(base + 0x44, 0x0104_88a8);
    e.call(0x005f_4fd0, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x71u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_4e00, fn_005f4e00(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005f4e00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_4e00,
            size: 0x7c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_cc50, &[A::T(0x2c)]),
                (0x0040_2c20, &[A::T(0x38)]),
                (0x0047_a130, &[A::T(0x44)]),
                (0x005f_4fd0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x71)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_891c),
                (0x18, 4, 0x0104_8908),
                (0x24, 4, 0x0104_88f0),
                (0x2c, 4, 0x0104_88dc),
                (0x38, 4, 0x0104_88bc),
                (0x44, 4, 0x0104_88a8),
            ],
        });
    }
}
