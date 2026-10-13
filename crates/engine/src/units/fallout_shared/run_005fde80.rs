//! `fallout shared/run_005fde80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fe040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fe040 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x48 (0x54 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fe040(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x005f_e160, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_9d0c);
    e.mem.set_u32(base + 0x18, 0x0104_9ce8);
    e.call(0x0068_0890, &args![base + 0x30]);
    e.call(0x0040_37b0, &args![base + 0x4c]);
    e.call(0x004f_15a0, &args![this, 0x48]);
    e.mem.set_u32(base + 0x40, 0x0);
    e.call(0x005f_e3b0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_e040, fn_005fe040(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fe040_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_e040,
            size: 0x54,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x005f_e160, &[A::T(0x18)]),
                (0x0068_0890, &[A::T(0x30)]),
                (0x0040_37b0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x48)]),
                (0x005f_e3b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_9d0c),
                (0x18, 4, 0x0104_9ce8),
                (0x40, 4, 0x0),
            ],
        });
    }
}
