//! `fallout shared/tesdescription.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00482f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x620`. The engine map has no name for it; the
/// `TESForm` constructor (`00483370`) calls it on the data handler
/// (`011c3f2c`) and assigns a new form ID only when it answers 0.
pub fn fn_00482f20(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x620)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00482f20, fn_00482f20(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte_at_0x620() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x624);
        e.mem.set_u32(object + 0x620, 0x1122_3301);
        assert_eq!(e.call(0x0048_2f20, &[object]).u32() & 0xff, 1);
        e.mem.set_u8(object + 0x620, 0);
        assert_eq!(e.call(0x0048_2f20, &[object]).u32() & 0xff, 0);
    }
}
