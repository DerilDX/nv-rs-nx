//! `NiMain/nitristripsdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiTriStripsData` (Xbox PDB), 0x50 bytes: only the fields used so far.
    pub struct NiTriStripsData: 0x50 {
        /// `m_pusStripLists` (Xbox PDB): `u16*`, the strips' vertex indices.
        0x4c m_pusStripLists: Ptr,
    }
}

// Translated from 00a75f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetStripLists` (map name): the strip index array.
pub fn ni_tri_strips_data_get_strip_lists(e: &mut Engine, this: Ptr<NiTriStripsData>) -> Ptr {
    e.get(this, NiTriStripsData::m_pusStripLists)
}

// Translated from 00a75f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x011f_4a70.
pub fn ni_tri_strips_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x011f_4a70
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a75f30, ni_tri_strips_data_get_r_t_t_i() -> u32),
        entry!(
            0x00a75f10,
            ni_tri_strips_data_get_strip_lists(Ptr<NiTriStripsData>) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_strip_lists_pointer() {
        let mut e = Engine::new();
        let data: Ptr<NiTriStripsData> = e.new_object();
        e.set(
            data,
            NiTriStripsData::m_pusStripLists,
            Ptr::<()>::new(0x7000),
        );
        assert_eq!(
            e.call(0x00a7_5f10, &args![data]).ptr::<()>(),
            Ptr::<()>::new(0x7000)
        );
    }

    #[test]
    fn ni_tri_strips_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7_5f30, &args![]).u32(), 0x011f_4a70);
    }
}
