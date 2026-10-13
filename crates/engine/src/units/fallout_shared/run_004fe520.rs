//! `fallout shared/run_004fe520` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fe640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe640 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x16 (0x98 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fe640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.mem.set_u32(base, 0x0102_5594);
    e.mem.set_u32(base + 0x30, 0x0102_557c);
    e.mem.set_u32(base + 0x3c, 0x0102_5558);
    e.mem.set_u32(base + 0x5c, 0x0102_5544);
    e.mem.set_u32(base + 0x68, 0x0102_5530);
    e.mem.set_u32(base + 0x70, 0x0102_5520);
    e.call(0x004f_15a0, &args![this, 0x16]);
    e.call(0x004f_e7a0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_e640, fn_004fe640(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fe640_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_e640,
            size: 0x98,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x16)]),
                (0x004f_e7a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_5594),
                (0x30, 4, 0x0102_557c),
                (0x3c, 4, 0x0102_5558),
                (0x5c, 4, 0x0102_5544),
                (0x68, 4, 0x0102_5530),
                (0x70, 4, 0x0102_5520),
            ],
        });
    }
}
