//! `fallout/misc/saveload/bgssaveloadinitialdata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0084eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears exactly the three bytes of the compact form identifier and
/// returns the receiver. PC stores bytes individually; padding is untouched.
pub fn construct_compact_id(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in 0..3 {
        e.mem.set_u8(this.addr().wrapping_add(offset), 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084eaf0, construct_compact_id(Ptr) -> Ptr)]
}
