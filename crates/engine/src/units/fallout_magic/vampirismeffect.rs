//! `fallout/magic/vampirismeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00804090` (named `ActiveEffect::CopyData` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const COPY_DATA_TARGET_00804090: u32 = 0x0080_4090;

// Translated from 00829b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VampirismEffect::CopyData` (Xbox PDB `?CopyData@VampirismEffect@@MAAXPAVActiveEffect@@@Z`,
/// void, vtable word 0107a110 and others): forwards `this` and the `ActiveEffect` pointer to
/// the base body `00804090` (`ActiveEffect::CopyData`). Returns nothing.
pub fn vampirism_effect_copy_data(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(COPY_DATA_TARGET_00804090, &args![this, target]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_9b90, vampirism_effect_copy_data(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_vampirism_effect_copy_data_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(COPY_DATA_TARGET_00804090, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_9b90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == COPY_DATA_TARGET_00804090)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
