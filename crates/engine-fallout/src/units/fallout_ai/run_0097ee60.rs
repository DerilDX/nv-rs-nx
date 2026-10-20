//! `fallout/ai/run_0097ee60` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x70 of `this` (no name in the engine map; callers use the
/// body on more than one class, so the field is not named here).
pub fn fn_0097f2f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x70))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0097_f2f0, fn_0097f2f0(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0097f2f0_reads_the_byte_at_0x70() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x70, 0xaabb_cc01);
        assert_eq!(e.call(0x0097_f2f0, &args![this]).u8(), 1);
        e.mem.set_u8(this + 0x70, 0);
        assert_eq!(e.call(0x0097_f2f0, &args![this]).u8(), 0);
    }
}
