//! `NiAnimation/nitextkeyextradata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTextKeyExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TEXT_KEY_EXTRA_DATA_RTTI: u32 = 0x011f_3f28;

// Translated from 00a46b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_text_key_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TEXT_KEY_EXTRA_DATA_RTTI)
}

/// `00a7b3b0`: the jump target of `00a9af00`.
const JUMP_TARGET_00A9AF00: u32 = 0x00a7_b3b0;

// Translated from 00a9af00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::LinkObject` (Xbox PDB): a 5-byte jump to `00a7b3b0` (engine map: `NiExtraData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiTextKeyExtraData@@UAAXAAVNiStream@@@Z`.
pub fn ni_text_key_extra_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9AF00, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a46b60, ni_text_key_extra_data_get_rtti() -> Ptr),
        entry!(0x00a9_af00, ni_text_key_extra_data_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_text_key_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a46b60, &args![]).u32(),
            NI_TEXT_KEY_EXTRA_DATA_RTTI
        );
    }

    #[test]
    fn thunk_00a9af00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_af00;
        const TARGET: u32 = 0x00a7_b3b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}
