//! `fallout shared/run_005fda00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fdaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fdaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 10 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fdaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_8f50, &args![base + 0x24]);
    e.call(0x0048_e270, &args![base + 0x3c]);
    e.mem.set_u32(base, 0x0104_9b9c);
    e.mem.set_u32(base + 0x18, 0x0104_9b88);
    e.mem.set_u32(base + 0x24, 0x0104_9b64);
    e.mem.set_u32(base + 0x3c, 0x0104_9b44);
    e.call(0x004f_15a0, &args![this, 0xa]);
    e.call(0x005f_dc30, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_daa0, fn_005fdaa0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fdaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_daa0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_8f50, &[A::T(0x24)]),
                (0x0048_e270, &[A::T(0x3c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xa)]),
                (0x005f_dc30, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_9b9c),
                (0x18, 4, 0x0104_9b88),
                (0x24, 4, 0x0104_9b64),
                (0x3c, 4, 0x0104_9b44),
            ],
        });
    }
}
