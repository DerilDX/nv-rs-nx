//! `fallout shared/pathfinding/run_006b8310` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map allocator release, forwarding even a null item.
pub fn free_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    e.call(0x00401030, &args![item]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b8310, free_item(Ptr, Ptr))]
}
