//! `NiMain/nigeometry.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometry::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOMETRY_RTTI: u32 = 0x011f_4acc;

// Translated from 00a80530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geometry_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOMETRY_RTTI)
}

/// `00a59dc0`: the jump target of `00a7fdb0`.
const JUMP_TARGET_00A7FDB0: u32 = 0x00a5_9dc0;

/// `00a5c6a0`: the jump target of `00a7fdc0`.
const JUMP_TARGET_00A7FDC0: u32 = 0x00a5_c6a0;

// Translated from 00a7fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetBound` (Xbox PDB): a 5-byte jump to `00a59dc0` (engine map: `NiAVObject::SetBound`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetBound@NiGeometry@@UAAXPAVBSNiBound@@@Z`.
pub fn ni_geometry_set_bound(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00A7FDB0, &args![this, arg]);
}

// Translated from 00a7fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::PostLinkObject` (Xbox PDB): a 5-byte jump to `00a5c6a0` (engine map: `NiObjectNET::PostLinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?PostLinkObject@NiGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_geometry_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A7FDC0, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a80530, ni_geometry_get_rtti() -> Ptr),
        entry!(0x00a7_fdb0, ni_geometry_set_bound(Ptr, Ptr)),
        entry!(0x00a7_fdc0, ni_geometry_post_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geometry_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a80530, &args![]).u32(), NI_GEOMETRY_RTTI);
    }

    #[test]
    fn thunk_00a7fdb0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdb0;
        const TARGET: u32 = 0x00a5_9dc0;
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
    fn thunk_00a7fdc0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdc0;
        const TARGET: u32 = 0x00a5_c6a0;
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
