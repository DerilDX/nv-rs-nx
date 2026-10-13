//! `fallout shared/run_006151d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006153c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006153c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x68 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_ba24`) is `TESReputation`, so this is the constructor of `TESReputation`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_006153c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0040_2c20, &args![base + 0x24]);
    e.call(0x0047_a130, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_ba24);
    e.mem.set_u32(base + 0x18, 0x0104_ba0c);
    e.mem.set_u32(base + 0x24, 0x0104_b9ec);
    e.mem.set_u32(base + 0x30, 0x0104_b9d8);
    e.call(0x0061_5560, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x68u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_53c0, fn_006153c0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_006153c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_53c0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2c20, &[A::T(0x24)]),
                (0x0047_a130, &[A::T(0x30)]),
                (0x0061_5560, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x68)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_ba24),
                (0x18, 4, 0x0104_ba0c),
                (0x24, 4, 0x0104_b9ec),
                (0x30, 4, 0x0104_b9d8),
            ],
        });
    }
}
