//! `fallout shared/region system/run_004f2480` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the address of
/// the member at +0x764 of `this`.
pub fn fn_004f2770(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x764)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f2770, fn_004f2770(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004f_2770, &args![0x1000u32]).u32(), 0x1764);
    }
}
