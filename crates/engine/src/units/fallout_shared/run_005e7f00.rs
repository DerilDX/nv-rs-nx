//! `fallout shared/run_005e7f00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e8060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e8060 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x31 (0x80 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e8060(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x48]);
    e.call(0x0040_2c20, &args![base + 0x54]);
    e.call(0x0047_b4f0, &args![base + 0x60]);
    e.mem.set_u32(base, 0x0104_6874);
    e.mem.set_u32(base + 0x30, 0x0104_6850);
    e.mem.set_u32(base + 0x48, 0x0104_683c);
    e.mem.set_u32(base + 0x54, 0x0104_681c);
    e.mem.set_u32(base + 0x60, 0x0104_6808);
    e.call(0x0096_a2d0, &args![base + 0x74]);
    e.call(0x004f_15a0, &args![this, 0x31]);
    e.call(0x005e_8240, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_8060, fn_005e8060(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e8060_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_8060,
            size: 0x80,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x48)]),
                (0x0040_2c20, &[A::T(0x54)]),
                (0x0047_b4f0, &[A::T(0x60)]),
                (0x0096_a2d0, &[A::T(0x74)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x31)]),
                (0x005e_8240, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_6874),
                (0x30, 4, 0x0104_6850),
                (0x48, 4, 0x0104_683c),
                (0x54, 4, 0x0104_681c),
                (0x60, 4, 0x0104_6808),
            ],
        });
    }
}
