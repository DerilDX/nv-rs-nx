//! `NiAnimation/bsfrustumfovcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSFrustumFOVController::ms_RTTI` (the class's `NiRTTI` record).
const BS_FRUSTUM_FOV_CONTROLLER_RTTI: u32 = 0x011f_3ec8;

// Translated from 00a42e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_frustum_fov_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_FRUSTUM_FOV_CONTROLLER_RTTI)
}

/// `00a57c00`: the jump target of `00a44ce0`.
const JUMP_TARGET_00A44CE0: u32 = 0x00a5_7c00;

/// `00a57c10`: the jump target of `00a44cf0`.
const JUMP_TARGET_00A44CF0: u32 = 0x00a5_7c10;

// Translated from 00a44ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57c00` (engine map: `NiBoolInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSFrustumFOVController@@UAAXAAVNiStream@@@Z`.
pub fn bs_frustum_fov_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A44CE0, &args![this, stream]);
}

// Translated from 00a44cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::IsEqual` (Xbox PDB): a 5-byte jump to `00a57c10` (engine map: `NiBoolInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSFrustumFOVController@@UAA_NPAVNiObject@@@Z`.
pub fn bs_frustum_fov_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A44CF0, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42e80, bs_frustum_fov_controller_get_rtti() -> Ptr),
        entry!(0x00a4_4ce0, bs_frustum_fov_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4cf0, bs_frustum_fov_controller_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_frustum_fov_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a42e80, &args![]).u32(),
            BS_FRUSTUM_FOV_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a44ce0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_4ce0;
        const TARGET: u32 = 0x00a5_7c00;
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
    fn thunk_00a44cf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_4cf0;
        const TARGET: u32 = 0x00a5_7c10;
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
