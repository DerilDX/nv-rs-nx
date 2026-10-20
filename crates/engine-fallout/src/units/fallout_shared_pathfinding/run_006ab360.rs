//! `fallout shared/pathfinding/run_006ab360` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ab360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0xB4` (`ECX` only, no stack
/// words).
pub fn fn_006ab360(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xb4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_b360, fn_006ab360(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_006ab360_returns_this_plus_0xb4() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x006a_b360, &args![0x4000u32]).u32(), 0x40b4);
    }
}
