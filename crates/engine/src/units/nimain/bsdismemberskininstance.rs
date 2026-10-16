//! `NiMain/bsdismemberskininstance.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a86380`: the jump target of `00a6c670`.
const JUMP_TARGET_00A6C670: u32 = 0x00a8_6380;

// Translated from 00a6c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::LinkObject` (Xbox PDB): a 5-byte jump to `00a86380` (engine map: `NiSkinInstance::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSDismemberSkinInstance@@UAAXAAVNiStream@@@Z`.
pub fn bs_dismember_skin_instance_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A6C670, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a6_c670,
        bs_dismember_skin_instance_link_object(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a6c670_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_c670;
        const TARGET: u32 = 0x00a8_6380;
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
