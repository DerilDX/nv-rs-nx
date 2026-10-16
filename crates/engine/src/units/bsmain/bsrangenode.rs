//! `BSMain/bsrangenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c457d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e0c without reading the receiver. The map names this body `BSRangeNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c457d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e0c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4_57d0, fn_00c457d0(Ptr) -> u32)]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c457d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_57d0, &args![0x1000u32]).u32(), 0x0120_2e0c);
        assert_eq!(e.call(0x00c4_57d0, &args![0u32]).u32(), 0x0120_2e0c);
    }
}
