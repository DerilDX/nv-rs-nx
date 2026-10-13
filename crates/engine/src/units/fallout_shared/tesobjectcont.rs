//! `fallout shared/tesobjectcont.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005160f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005160f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESObjectCONT::TESObjectCONT` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x1b (0x9c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesobjectcont_tesobjectcont(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0048_1610, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x3c]);
    e.call(0x0048_a3d0, &args![base + 0x48]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_eac0, &args![base + 0x74]);
    e.call(0x0047_5350, &args![base + 0x7c]);
    e.call(0x0051_0620, &args![base + 0x84]);
    e.mem.set_u32(base, 0x0102_aeb4);
    e.mem.set_u32(base + 0x30, 0x0102_ae9c);
    e.mem.set_u32(base + 0x3c, 0x0102_ae88);
    e.mem.set_u32(base + 0x48, 0x0102_ae64);
    e.mem.set_u32(base + 0x68, 0x0102_ae50);
    e.mem.set_u32(base + 0x74, 0x0102_ae3c);
    e.mem.set_u32(base + 0x7c, 0x0102_ae28);
    e.mem.set_u32(base + 0x84, 0x0102_ae18);
    e.call(0x004f_15a0, &args![this, 0x1b]);
    e.call(0x0051_6330, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_60f0, tesobjectcont_tesobjectcont(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesobjectcont_tesobjectcont_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_60f0,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0048_1610, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x3c)]),
                (0x0048_a3d0, &[A::T(0x48)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_eac0, &[A::T(0x74)]),
                (0x0047_5350, &[A::T(0x7c)]),
                (0x0051_0620, &[A::T(0x84)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1b)]),
                (0x0051_6330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_aeb4),
                (0x30, 4, 0x0102_ae9c),
                (0x3c, 4, 0x0102_ae88),
                (0x48, 4, 0x0102_ae64),
                (0x68, 4, 0x0102_ae50),
                (0x74, 4, 0x0102_ae3c),
                (0x7c, 4, 0x0102_ae28),
                (0x84, 4, 0x0102_ae18),
            ],
        });
    }
}
