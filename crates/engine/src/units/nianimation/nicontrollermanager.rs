//! `NiAnimation/nicontrollermanager.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAVObjectPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_AV_OBJECT_PALETTE_RTTI: u32 = 0x011f_5b58;

/// `NiDefaultAVObjectPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_DEFAULT_AV_OBJECT_PALETTE_RTTI: u32 = 0x011f_4a04;

// Translated from 00a2e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObjectPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_av_object_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_AV_OBJECT_PALETTE_RTTI)
}

// Translated from 00a2ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_default_av_object_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DEFAULT_AV_OBJECT_PALETTE_RTTI)
}

// Translated from 00a2ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a2ebb0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x7c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_controller_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a2_ebb0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x7cu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2e1e0, ni_av_object_palette_get_rtti() -> Ptr),
        entry!(0x00a2ede0, ni_default_av_object_palette_get_rtti() -> Ptr),
        entry!(0x00a2ef80, ni_controller_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_av_object_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2e1e0, &args![]).u32(),
            NI_AV_OBJECT_PALETTE_RTTI
        );
    }

    #[test]
    fn ni_default_av_object_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2ede0, &args![]).u32(),
            NI_DEFAULT_AV_OBJECT_PALETTE_RTTI
        );
    }

    #[test]
    fn ni_controller_manager_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a2_ebb0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a2ef80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a2ef80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x7c]]);
        assert_eq!(calls_to(&e, 0x00a2_ebb0), vec![vec![this]; 2]);
    }
}
