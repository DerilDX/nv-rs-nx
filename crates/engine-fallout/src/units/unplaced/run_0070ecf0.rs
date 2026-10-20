//! `(unplaced)/run_0070ecf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global `0070ed10` answers.
const FLAG_011DEA2F: u32 = 0x011d_ea2f;

// Translated from 0070ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dea2f` (in
/// `AL`).
pub fn fn_0070ed10(e: &mut Engine) -> u8 {
    e.mem.u8(FLAG_011DEA2F)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070ed10, fn_0070ed10() -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u8(FLAG_011DEA2F, 0x42);
        assert_eq!(e.call(0x0070_ed10, &[]).u8(), 0x42);
    }
}
