//! `NiAnimation/bsanimnote.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAnimNotes::ms_RTTI` (the class's `NiRTTI` record).
const BS_ANIM_NOTES_RTTI: u32 = 0x011f_3f00;

// Translated from 00a44920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_anim_notes_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_ANIM_NOTES_RTTI)
}

/// `00a5d6e0`: the jump target of `00a44910`.
const JUMP_TARGET_00A44910: u32 = 0x00a5_d6e0;

// Translated from 00a44910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::GetViewerStrings` (Xbox PDB): a 5-byte jump to `00a5d6e0` (engine map: `NiObject::GetViewerStrings`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?GetViewerStrings@BSAnimNotes@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z`.
pub fn bs_anim_notes_get_viewer_strings(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00A44910, &args![this, arg]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a44920, bs_anim_notes_get_rtti() -> Ptr),
        entry!(0x00a4_4910, bs_anim_notes_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_anim_notes_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44920, &args![]).u32(), BS_ANIM_NOTES_RTTI);
    }

    #[test]
    fn thunk_00a44910_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_4910;
        const TARGET: u32 = 0x00a5_d6e0;
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
