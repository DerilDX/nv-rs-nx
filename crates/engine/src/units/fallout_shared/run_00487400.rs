//! `fallout shared/run_00487400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00487400 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01017f88 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00487400(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0101_7f88
}

// Translated from 00487410 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0101cbb0 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00487410(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0101_cbb0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_7400, fn_00487400(Ptr) -> u32),
        entry!(0x0048_7410, fn_00487410(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00487400_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0048_7400, &args![0x1000u32]).u32(), 0x0101_7f88);
        assert_eq!(e.call(0x0048_7400, &args![0u32]).u32(), 0x0101_7f88);
    }

    #[test]
    fn fn_00487410_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0048_7410, &args![0x1000u32]).u32(), 0x0101_cbb0);
        assert_eq!(e.call(0x0048_7410, &args![0u32]).u32(), 0x0101_cbb0);
    }
}
