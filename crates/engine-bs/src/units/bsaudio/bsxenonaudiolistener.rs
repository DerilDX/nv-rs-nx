//! `BSAudio/bsxenonaudiolistener.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAudioListener::iEvironmentType` (Xbox PDB spelling), +0x44.
const ENVIRONMENT_TYPE: u32 = 0x44;

// Translated from 00aeab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::SetEnvironmentType` (Xbox PDB): stores `kind` in
/// `iEvironmentType` (+0x44).
pub fn bs_audio_listener_set_environment_type(e: &mut Engine, this: Ptr, kind: u32) {
    e.mem
        .set_u32(this.addr().wrapping_add(ENVIRONMENT_TYPE), kind);
}

// Translated from 00aeab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's environment-type getter (the vtable slot after the
/// setter; `GetEnvironmentType` in the Xbox PDB's order, not confirmed):
/// returns `iEvironmentType` (+0x44).
pub fn fn_00aeab70(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(ENVIRONMENT_TYPE))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ae_ab50,
            bs_audio_listener_set_environment_type(Ptr, u32)
        ),
        entry!(0x00ae_ab70, fn_00aeab70(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_type_round_trip() {
        let mut e = Engine::new();
        let listener = e.mem.alloc(0x7c);
        e.call(0x00ae_ab50, &args![listener, 6u32]);
        assert_eq!(e.mem.u32(listener + 0x44), 6);
        e.mem.set_u32(listener + 0x44, 9);
        assert_eq!(e.call(0x00ae_ab70, &args![listener]).u32(), 9);
    }
}
