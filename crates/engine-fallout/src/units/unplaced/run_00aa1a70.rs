//! `(unplaced)/run_00aa1a70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function-pointer global `00aa1a70` sets.
const CALLBACK_SLOT: u32 = 0x011a_c228;
/// Its default: `00aa1a40` (placed in `NiSystem/nifile.cpp` by the engine
/// map, 45 bytes; not named there).
const DEFAULT_CALLBACK: u32 = 0x00aa_1a40;

// Translated from 00aa1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): stores `callback` in the
/// function-pointer global `011ac228`, or the default `00aa1a40` when
/// `callback` is null (the exe writes the default first, then overwrites it).
pub fn fn_00aa1a70(e: &mut Engine, callback: u32) {
    e.set_global(CALLBACK_SLOT, DEFAULT_CALLBACK);
    if callback != 0 {
        e.set_global(CALLBACK_SLOT, callback);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_1a70, fn_00aa1a70(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_callback_or_the_default() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.call(0x00aa_1a70, &args![0x0055_0000u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), 0x0055_0000);
        e.call(0x00aa_1a70, &args![0u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), DEFAULT_CALLBACK);
    }
}
