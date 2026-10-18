//! `NiMain/nibinaryextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b290`: the jump target of `00a89ae0`.
const JUMP_TARGET_00A89AE0: u32 = 0x00a7_b290;

// Translated from 00a89ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a7b290` (engine map: `NiExtraData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBinaryExtraData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_binary_extra_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A89AE0, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_9ae0, ni_binary_extra_data_register_streamables(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a89ae0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_9ae0;
        const TARGET: u32 = 0x00a7_b290;
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
