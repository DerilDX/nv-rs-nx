//! `NiMain/nilines.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLines::ms_RTTI` (the class's `NiRTTI` record).
const NI_LINES_RTTI: u32 = 0x011f_4a48;

// Translated from 00a74790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lines_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LINES_RTTI)
}

/// `00a80b50`: the jump target of `00a74780`.
const JUMP_TARGET_00A74780: u32 = 0x00a8_0b50;

/// `00a80470`: the jump target of `00a89770`.
const JUMP_TARGET_00A89770: u32 = 0x00a8_0470;

/// `00a80bf0`: the jump target of `00a9b180`.
const JUMP_TARGET_00A9B180: u32 = 0x00a8_0bf0;

/// `00a80430`: the jump target of `00a9b190`.
const JUMP_TARGET_00A9B190: u32 = 0x00a8_0430;

/// `00a80db0`: the jump target of `00a9b1a0`.
const JUMP_TARGET_00A9B1A0: u32 = 0x00a8_0db0;

// Translated from 00a74780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::LinkObject` (Xbox PDB): a 5-byte jump to `00a80b50` (engine map: `NiGeometry::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A74780, &args![this, stream]);
}

// Translated from 00a89770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::IsEqual` (Xbox PDB): a 5-byte jump to `00a80470` (engine map: `NiGeometry::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiLines@@UAA_NPAVNiObject@@@Z`.
pub fn ni_lines_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A89770, &args![this, other]).bool()
}

// Translated from 00a9b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::LoadBinary` (Xbox PDB): a 5-byte jump to `00a80bf0` (engine map: `NiGeometry::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B180, &args![this, stream]);
}

// Translated from 00a9b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a80430` (engine map: `NiGeometry::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLines@@UAA_NAAVNiStream@@@Z`.
pub fn ni_lines_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B190, &args![this, stream]).bool()
}

// Translated from 00a9b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::SaveBinary` (Xbox PDB): a 5-byte jump to `00a80db0` (engine map: `NiGeometry::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B1A0, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a74790, ni_lines_get_rtti() -> Ptr),
        entry!(0x00a7_4780, ni_lines_link_object(Ptr, Ptr)),
        entry!(0x00a8_9770, ni_lines_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_b180, ni_lines_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b190, ni_lines_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_b1a0, ni_lines_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lines_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a74790, &args![]).u32(), NI_LINES_RTTI);
    }

    #[test]
    fn thunk_00a74780_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_4780;
        const TARGET: u32 = 0x00a8_0b50;
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
    fn thunk_00a89770_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_9770;
        const TARGET: u32 = 0x00a8_0470;
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
    fn thunk_00a9b180_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b180;
        const TARGET: u32 = 0x00a8_0bf0;
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
    fn thunk_00a9b190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b190;
        const TARGET: u32 = 0x00a8_0430;
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
    fn thunk_00a9b1a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b1a0;
        const TARGET: u32 = 0x00a8_0db0;
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
