//! `fallout/misc/saveload/bgsloadgamesubbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00865570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded append wrapper: reads position at +0xc and forwards the low
/// byte of the last stack argument to the positioned save-buffer writer.
pub fn append_bytes(e: &mut Engine, this: Ptr, source: Ptr, size: u32, no_separator: u8) {
    let position = e.mem.u32(this.addr().wrapping_add(0xc));
    e.call(
        0x00865ce0,
        &args![this, source, size, position, no_separator],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00865570, append_bytes(Ptr, Ptr, u32, u8))]
}
