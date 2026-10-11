//! `fallout/misc/saveload/bgssaveloadreferencesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00853130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Map lookup, returning success in AL. A failed lookup leaves the output
/// untouched. Bucket pointers, links and values are read after the relevant
/// hash/equality callback, as in the PC instructions.
pub fn lookup(e: &mut Engine, this: Ptr, key: u32, out: Ptr<u32>) -> u8 {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket));
    while node != 0 {
        let existing_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, existing_key]).u8() != 0 {
            let value = e.mem.u32(node.wrapping_add(8));
            e.mem.set_u32(out.addr(), value);
            return 1;
        }
        node = e.mem.u32(node);
    }
    0
}

// Translated from 00853240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the item's value then releases it through the allocator at +0xc.
pub fn delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr().wrapping_add(8), 0);
    e.call(0x006b8310, &args![this.addr().wrapping_add(0xc), item]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00853130, lookup(Ptr, u32, Ptr<u32>) -> u8),
        entry!(0x00853240, delete_item(Ptr, Ptr)),
    ]
}
