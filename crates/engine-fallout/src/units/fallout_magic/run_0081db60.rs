//! `fallout/magic/run_0081db60` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008646f0` (`thiscall`, no stack words): the `BGSLoadGameBuffer` destructor body (stores
/// the vtable `01082048`, RTTI `.?AVBGSLoadGameBuffer@@`, and frees the buffer at +4).
const LOAD_GAME_BUFFER_DESTRUCT: u32 = 0x0086_46f0;

// Translated from 0081db90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs the `BGSLoadGameBuffer`
/// destructor body (`008646f0`) on `this`.
pub fn fn_0081db90(e: &mut Engine, this: Ptr) {
    e.call(LOAD_GAME_BUFFER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_db90, fn_0081db90(Ptr))]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0081db90_runs_the_destructor_body() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_BUFFER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_db90, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(LOAD_GAME_BUFFER_DESTRUCT, vec![0x5000])]);
    }
}
