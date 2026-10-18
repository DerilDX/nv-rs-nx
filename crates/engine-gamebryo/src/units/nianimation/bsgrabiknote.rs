//! `NiAnimation/bsgrabiknote.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSGrabIKNote::ms_RTTI` (the class's `NiRTTI` record).
const BS_GRAB_IK_NOTE_RTTI: u32 = 0x011f_3ef0;

// Translated from 00a44350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_grab_ik_note_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_GRAB_IK_NOTE_RTTI)
}

/// `00a724d0`: the jump target of `00a95ca0`.
const JUMP_TARGET_00A95CA0: u32 = 0x00a7_24d0;

// Translated from 00a95ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGrabIKNote::LinkObject` (Xbox PDB): a 5-byte jump to `00a724d0` (engine map: `NiPSysEmitterCtlrData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSGrabIKNote@@UAAXAAVNiStream@@@Z`.
pub fn bs_grab_ik_note_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A95CA0, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a44350, bs_grab_ik_note_get_rtti() -> Ptr),
        entry!(0x00a9_5ca0, bs_grab_ik_note_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_grab_ik_note_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44350, &args![]).u32(), BS_GRAB_IK_NOTE_RTTI);
    }

    #[test]
    fn thunk_00a95ca0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_5ca0;
        const TARGET: u32 = 0x00a7_24d0;
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
