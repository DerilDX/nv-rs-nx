//! `fallout shared/tessound.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e2b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2b70 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESSound::TESSound` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0xd (0x6c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tessound_tessound(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0059_12d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_4ffc);
    e.mem.set_u32(base + 0x30, 0x0104_4fe4);
    e.call(0x0040_37b0, &args![base + 0x3c]);
    e.call(0x004f_15a0, &args![this, 0xd]);
    e.mem.set_u8(base + 0x68, 0x0);
    e.call(0x005e_2ef0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_2b70, tessound_tessound(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tessound_tessound_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_2b70,
            size: 0x6c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0059_12d0, &[A::T(0x30)]),
                (0x0040_37b0, &[A::T(0x3c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xd)]),
                (0x005e_2ef0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_4ffc),
                (0x30, 4, 0x0104_4fe4),
                (0x68, 1, 0x0),
            ],
        });
    }
}
