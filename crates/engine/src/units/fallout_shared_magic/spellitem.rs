//! `fallout shared/magic/spellitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040e160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040e160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `SpellItem::SpellItem` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x14 (0x44 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn spellitem_spellitem(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_9190, &args![this]);
    e.mem.set_u32(base, 0x0101_3f8c);
    e.mem.set_u32(base + 0x18, 0x0101_3f44);
    e.mem.set_u32(base + 0x24, 0x0101_3f28);
    e.call(0x004f_15a0, &args![this, 0x14]);
    e.call(0x0040_e330, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_e160, spellitem_spellitem(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn spellitem_spellitem_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_e160,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_9190, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x14)]),
                (0x0040_e330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_3f8c),
                (0x18, 4, 0x0101_3f44),
                (0x24, 4, 0x0101_3f28),
            ],
        });
    }
}
