//! `fallout shared/bgsterminal.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00500940 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns receiver +0x18 without reading memory. The source-unit mapping
/// is by address range; no Xbox PDB name is available for this accessor.
/// Script callers use it to obtain their embedded header.
pub fn fn_00500940(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x18))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00500940, fn_00500940(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_address_matches_native_without_dereferencing_receiver() {
        // Five native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f:
        // EAX is receiver+24 modulo 2^32, RET 0 and no faults.
        let mut e = Engine::new();
        for receiver in [0u32, 1, 0x1000_0000, u32::MAX - 15, u32::MAX] {
            assert_eq!(
                e.call(0x00500940, &[receiver]).u32(),
                receiver.wrapping_add(24)
            );
        }
    }
}
