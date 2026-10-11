//! `fallout shared/pathfinding/navmeshinfomap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b81d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map key equality: compares full 32-bit words, returns AL.
pub fn keys_equal(_e: &mut Engine, _this: Ptr, left: u32, right: u32) -> u8 {
    u8::from(left == right)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b81d0, keys_equal(Ptr, u32, u32) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_equality_matches_native_low_byte_vectors() {
        // Generated native vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80. Only AL is the boolean result.
        let mut e = Engine::new();
        for (left, right, expected) in [
            (0u32, 0u32, 1u8),
            (0x12345678, 0x12345678, 1),
            (0x12345678, 0x12345679, 0),
            (0, 0x80000000, 0),
        ] {
            assert_eq!(e.call(0x006b81d0, &args![0u32, left, right]).u8(), expected);
        }
    }
}
