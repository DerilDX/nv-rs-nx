//! `NiMain/niaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_ACCUMULATOR_RTTI: u32 = 0x011f_6060;

// Translated from 00aa0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ACCUMULATOR_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa0b00, ni_accumulator_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa0b00, &args![]).u32(), NI_ACCUMULATOR_RTTI);
    }
}
