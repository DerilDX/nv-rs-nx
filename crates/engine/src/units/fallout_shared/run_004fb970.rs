//! `fallout shared/run_004fb970` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fba00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fba00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x22 (0x74 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fba00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d00, &args![this]);
    e.call(0x0047_5350, &args![base + 0xc]);
    e.call(0x0051_b140, &args![base + 0x14]);
    e.mem.set_u32(base, 0x0102_4e84);
    e.mem.set_u32(base + 0xc, 0x0102_4e70);
    e.mem.set_u32(base + 0x14, 0x0102_4cec);
    e.mem.set_u32(base + 0x44, 0x0102_4cc8);
    e.call(0x0086_4490, &args![base + 0x70]);
    e.call(0x004f_15a0, &args![base + 0x14, 0x22]);
    e.call(0x004f_bb50, &args![base + 0x14]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_ba00, fn_004fba00(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fba00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_ba00,
            size: 0x74,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d00, &[A::T(0x0)]),
                (0x0047_5350, &[A::T(0xc)]),
                (0x0051_b140, &[A::T(0x14)]),
                (0x0086_4490, &[A::T(0x70)]),
                (0x004f_15a0, &[A::T(0x14), A::V(0x22)]),
                (0x004f_bb50, &[A::T(0x14)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4e84),
                (0xc, 4, 0x0102_4e70),
                (0x14, 4, 0x0102_4cec),
                (0x44, 4, 0x0102_4cc8),
            ],
        });
    }
}
