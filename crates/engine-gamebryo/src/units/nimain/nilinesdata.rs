//! `NiMain/nilinesdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLinesData::ms_RTTI` (the class's `NiRTTI` record).
const NI_LINES_DATA_RTTI: u32 = 0x011f_5b88;

// Translated from 00a8a7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lines_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LINES_DATA_RTTI)
}

/// `00a677e0`: the jump target of `00a8a9a0`.
const JUMP_TARGET_00A8A9A0: u32 = 0x00a6_77e0;

/// `00a66f50`: the jump target of `00a8a9b0`.
const JUMP_TARGET_00A8A9B0: u32 = 0x00a6_6f50;

// Translated from 00a8a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::LinkObject` (Xbox PDB): a 5-byte jump to `00a677e0` (engine map: `NiGeometryData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLinesData@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A8A9A0, &args![this, stream]);
}

// Translated from 00a8a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a66f50` (engine map: `NiGeometryData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLinesData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_lines_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A8A9B0, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8a7b0, ni_lines_data_get_rtti() -> Ptr),
        entry!(0x00a8_a9a0, ni_lines_data_link_object(Ptr, Ptr)),
        entry!(0x00a8_a9b0, ni_lines_data_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lines_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8a7b0, &args![]).u32(), NI_LINES_DATA_RTTI);
    }

    #[test]
    fn thunk_00a8a9a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_a9a0;
        const TARGET: u32 = 0x00a6_77e0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a8a9b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_a9b0;
        const TARGET: u32 = 0x00a6_6f50;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
