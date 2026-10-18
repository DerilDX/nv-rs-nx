//! `NiMain/nialphaproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a47360`: the jump target of `00a950c0`.
const JUMP_TARGET_00A950C0: u32 = 0x00a4_7360;

/// `00a62130`: the jump target of `00a99e40`.
const JUMP_TARGET_00A99E40: u32 = 0x00a6_2130;

// Translated from 00a950c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::LinkObject` (Xbox PDB): a 5-byte jump to `00a47360` (engine map: `NiSequenceStreamHelper::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiAlphaProperty@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_property_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A950C0, &args![this, stream]);
}

// Translated from 00a99e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a62130` (engine map: `NiSequenceStreamHelper::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaProperty@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_property_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A99E40, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_50c0, ni_alpha_property_link_object(Ptr, Ptr)),
        entry!(0x00a9_9e40, ni_alpha_property_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a950c0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_50c0;
        const TARGET: u32 = 0x00a4_7360;
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
    fn thunk_00a99e40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_9e40;
        const TARGET: u32 = 0x00a6_2130;
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
