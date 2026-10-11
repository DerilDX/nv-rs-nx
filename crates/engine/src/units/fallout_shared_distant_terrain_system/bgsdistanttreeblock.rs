//! `fallout shared/distant terrain system/bgsdistanttreeblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map-item allocator: allocates twelve bytes without initializing them.
pub fn allocate_item(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x00401000, &args![12u32]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006f99e0, allocate_item(Ptr) -> Ptr)]
}
