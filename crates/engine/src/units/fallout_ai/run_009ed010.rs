//! `fallout/ai/run_009ed010` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00679a90` (named `TESPackage::InitLoadGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_00679A90: u32 = 0x0067_9a90;

// Translated from 009ed010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 01092c40 and 01092de8): forwards `this` and one
/// pointer to `00679a90` (named `TESPackage::InitLoadGame`; Xbox PDB
/// `?InitLoadGame@TESPackage@@UAAXPAVBGSLoadFormBuffer@@@Z`, void, the overload with one
/// parameter matches its `RET 4`). Returns nothing.
pub fn fn_009ed010(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_00679A90, &args![this, buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009e_d010, fn_009ed010(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009ed010_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_00679A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009e_d010, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_00679A90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
