//! `NiMain/niambientlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAmbientLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_AMBIENT_LIGHT_RTTI: u32 = 0x011f_5f6c;

// Translated from 00a9b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_ambient_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_AMBIENT_LIGHT_RTTI)
}

/// `00a72080`: the jump target of `00a75c00`.
const JUMP_TARGET_00A75C00: u32 = 0x00a7_2080;

/// `00a90010`: the jump target of `00a75c10`.
const JUMP_TARGET_00A75C10: u32 = 0x00a9_0010;

/// `00a71f50`: the jump target of `00a9b360`.
const JUMP_TARGET_00A9B360: u32 = 0x00a7_1f50;

/// `00a720f0`: the jump target of `00a9b370`.
const JUMP_TARGET_00A9B370: u32 = 0x00a7_20f0;

/// `00a72160`: the jump target of `00a9b380`.
const JUMP_TARGET_00A9B380: u32 = 0x00a7_2160;

// Translated from 00a75c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::LoadBinary` (Xbox PDB): a 5-byte jump to `00a72080` (engine map: `NiLight::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A75C00, &args![this, stream]);
}

// Translated from 00a75c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a90010` (engine map: `NiLight::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAmbientLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_ambient_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A75C10, &args![this, stream]).bool()
}

// Translated from 00a9b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a71f50` (engine map: `NiLight::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B360, &args![this, stream]);
}

// Translated from 00a9b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::SaveBinary` (Xbox PDB): a 5-byte jump to `00a720f0` (engine map: `NiLight::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B370, &args![this, stream]);
}

// Translated from 00a9b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::IsEqual` (Xbox PDB): a 5-byte jump to `00a72160` (engine map: `NiLight::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiAmbientLight@@UAA_NPAVNiObject@@@Z`.
pub fn ni_ambient_light_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B380, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b390, ni_ambient_light_get_rtti() -> Ptr),
        entry!(0x00a7_5c00, ni_ambient_light_load_binary(Ptr, Ptr)),
        entry!(0x00a7_5c10, ni_ambient_light_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_b360, ni_ambient_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_b370, ni_ambient_light_save_binary(Ptr, Ptr)),
        entry!(0x00a9_b380, ni_ambient_light_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_ambient_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9b390, &args![]).u32(), NI_AMBIENT_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a75c00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_5c00;
        const TARGET: u32 = 0x00a7_2080;
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
    fn thunk_00a75c10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_5c10;
        const TARGET: u32 = 0x00a9_0010;
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

    #[test]
    fn thunk_00a9b360_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b360;
        const TARGET: u32 = 0x00a7_1f50;
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
    fn thunk_00a9b370_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b370;
        const TARGET: u32 = 0x00a7_20f0;
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
    fn thunk_00a9b380_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b380;
        const TARGET: u32 = 0x00a7_2160;
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
