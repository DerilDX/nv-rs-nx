//! `(unplaced)/run_00649530` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006499c0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d57a4 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_006499c0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_57a4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_99c0, fn_006499c0(Ptr) -> u32)]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006499c0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0064_99c0, &args![0x1000u32]).u32(), 0x011d_57a4);
        assert_eq!(e.call(0x0064_99c0, &args![0u32]).u32(), 0x011d_57a4);
    }
}
