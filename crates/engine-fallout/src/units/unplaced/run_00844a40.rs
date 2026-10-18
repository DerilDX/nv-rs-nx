//! `(unplaced)/run_00844a40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00844a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates through the folded map's allocator subobject at +0xc.
pub fn new_item(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x006f99e0, &args![this.addr().wrapping_add(0xc)])
        .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00844a70, new_item(Ptr) -> Ptr)]
}
