//! `NiParticle/nipsysemitterctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolData::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_DATA_RTTI: u32 = 0x011f_4054;

/// `NiPSysEmitterCtlr::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_EMITTER_CTLR_RTTI: u32 = 0x0120_24e8;

// Translated from 00a537e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_DATA_RTTI)
}

// Translated from 00c1c2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_p_sys_emitter_ctlr_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_P_SYS_EMITTER_CTLR_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a537e0, ni_bool_data_get_rtti() -> Ptr),
        entry!(0x00c1c2f0, ni_p_sys_emitter_ctlr_get_rtti() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a537e0, &args![]).u32(), NI_BOOL_DATA_RTTI);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c1c2f0, &args![]).u32(),
            NI_P_SYS_EMITTER_CTLR_RTTI
        );
    }
}
