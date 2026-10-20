//! `fallout/ai/run_0092c820` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040b460` (unnamed in the engine map, `magicitem.cpp` range): `cdecl`, one stack word.
const CALLEE_0040B460: u32 = 0x0040_b460;

// Translated from 0092c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: calls `0040b460` (`cdecl`) with the address of the
/// member at `this + 8`.
pub fn fn_0092c870(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0040B460, &args![this.byte_add(8)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0092_c870, fn_0092c870(Ptr))]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0092c870_passes_the_address_of_the_member_at_offset_8() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CALLEE_0040B460, |_, _| Ret::default());
        e.call(0x0092_c870, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CALLEE_0040B460)
            .map(|c| &c.1)
            .collect();
        assert_eq!(hits, vec![&vec![0x4008]]);
    }
}
