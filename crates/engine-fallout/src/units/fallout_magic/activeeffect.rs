//! `fallout/magic/activeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00804b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`activeeffect.cpp`). `cdecl`, one stack word: true (`AL`) when it
/// is 1, 4 or 10, false otherwise. The meaning of those values is unconfirmed.
pub fn fn_00804b30(_e: &mut Engine, value: i32) -> bool {
    value == 1 || value == 4 || value == 10
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_4b30, fn_00804b30(i32) -> bool)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00804b30_accepts_only_1_4_and_10() {
        let mut e = Engine::new();
        for value in [
            -1i32,
            0,
            1,
            2,
            3,
            4,
            5,
            9,
            10,
            11,
            0x100,
            0x401,
            i32::MIN,
            i32::MAX,
        ] {
            let expected = matches!(value, 1 | 4 | 10);
            assert_eq!(
                e.call(0x0080_4b30, &args![value]).bool(),
                expected,
                "{value}"
            );
        }
    }
}
