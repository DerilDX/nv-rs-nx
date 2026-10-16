//! `NiMain/nispotlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSpotLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_SPOT_LIGHT_RTTI: u32 = 0x011f_5ec4;

// Translated from 00a94d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_spot_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SPOT_LIGHT_RTTI)
}

/// `00a9b360`: the jump target of `00a94c60`.
const JUMP_TARGET_00A94C60: u32 = 0x00a9_b360;

/// `00a75c10`: the jump target of `00a94c70`.
const JUMP_TARGET_00A94C70: u32 = 0x00a7_5c10;

// Translated from 00a94c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a9b360` (engine map: `NiAmbientLight::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiSpotLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_spot_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A94C60, &args![this, stream]);
}

// Translated from 00a94c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a75c10` (engine map: `NiAmbientLight::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiSpotLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_spot_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A94C70, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a94d50, ni_spot_light_get_rtti() -> Ptr),
        entry!(0x00a9_4c60, ni_spot_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_4c70, ni_spot_light_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_spot_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a94d50, &args![]).u32(), NI_SPOT_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a94c60_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_4c60;
        const TARGET: u32 = 0x00a9_b360;
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
    fn thunk_00a94c70_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_4c70;
        const TARGET: u32 = 0x00a7_5c10;
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
