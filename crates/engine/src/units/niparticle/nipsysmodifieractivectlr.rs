//! `NiParticle/nipsysmodifieractivectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c32bf0`: the jump target of `00c29d30`.
const JUMP_TARGET_00C29D30: u32 = 0x00c3_2bf0;

// Translated from 00c29d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00c32bf0` (engine map: `NiPSysModifierBoolCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysModifierActiveCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_active_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C29D30, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c2_9d30,
        ni_p_sys_modifier_active_ctlr_link_object(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c29d30_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_9d30;
        const TARGET: u32 = 0x00c3_2bf0;
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
