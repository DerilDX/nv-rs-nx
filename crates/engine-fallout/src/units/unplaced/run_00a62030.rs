//! `(unplaced)/run_00a62030` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Source of the 0x44 bytes (17 words) `00a62110` copies, `011aa2a0`.
const COPY_SOURCE: u32 = 0x011a_a2a0;
/// Destination of the copy, `011f4600`.
const COPY_DESTINATION: u32 = 0x011f_4600;

// Translated from 00a62110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no unit in the engine map, next to `00a62030` and `00a62090`; run by the
/// start-up `00a89cc0`, no arguments): copies 17 words from `011aa2a0` to `011f4600`
/// (`REP MOVSD`).
pub fn fn_00a62110(e: &mut Engine) {
    for index in 0..0x11u32 {
        let word = e.mem.u32(COPY_SOURCE + 4 * index);
        e.mem.set_u32(COPY_DESTINATION + 4 * index, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_2110, fn_00a62110())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_seventeen_words() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for index in 0..0x12u32 {
            e.mem.set_u32(COPY_SOURCE + 4 * index, 0x1000 + index);
        }
        e.call(0x00a6_2110, &args![]);
        for index in 0..0x11u32 {
            assert_eq!(e.mem.u32(COPY_DESTINATION + 4 * index), 0x1000 + index);
        }
        assert_eq!(e.mem.u32(COPY_DESTINATION + 0x44), 0);
    }
}
