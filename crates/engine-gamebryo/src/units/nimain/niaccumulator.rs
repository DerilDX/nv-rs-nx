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

/// `NiAccumulator::m_pkCamera` (Xbox PDB) +8.
const ACCUMULATOR_CAMERA: u32 = 0x8;

// Translated from 00aa0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator::FinishAccumulating` (Xbox PDB): clears the camera
/// pointer at +8.
pub fn ni_accumulator_finish_accumulating(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr().wrapping_add(ACCUMULATOR_CAMERA), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa0b00, ni_accumulator_get_rtti() -> Ptr),
        entry!(0x00aa0fe0, ni_accumulator_finish_accumulating(Ptr)),
    ]
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

#[cfg(test)]
mod b014_tests {
    use super::*;

    #[test]
    fn finish_accumulating_clears_the_camera() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        e.mem.set_u32(this + 0xc, 0x5678);
        e.call(0x00aa0fe0, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x5678);
    }
}
