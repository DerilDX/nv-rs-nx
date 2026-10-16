//! `NiAnimation/nistringpalette.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStringPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_STRING_PALETTE_RTTI: u32 = 0x011f_3f30;

// Translated from 00a47140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_string_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_STRING_PALETTE_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a47140, ni_string_palette_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_string_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a47140, &args![]).u32(), NI_STRING_PALETTE_RTTI);
    }
}
