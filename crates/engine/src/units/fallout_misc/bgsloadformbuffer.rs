//! `fallout/misc/saveload/bgsloadformbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00864450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `BGSSaveLoadFormHeader` (Xbox PDB), a nine-byte packed header:
/// three-byte FormIDIndex, iChangeFlags at +3, FormInfo at +7, cVersion at +8.
pub fn construct_header(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0084eaf0, &args![this]);
    e.call(0x0044dee0, &args![this.addr().wrapping_add(3)]);
    construct_form_info(e, Ptr::new(this.addr().wrapping_add(7)));
    e.mem.set_u8(this.addr().wrapping_add(8), 0);
    this
}

// Translated from 00864490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears only the header FormInfo byte (Xbox PDB) and returns the receiver.
pub fn construct_form_info(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00864450, construct_header(Ptr) -> Ptr),
        entry!(0x00864490, construct_form_info(Ptr) -> Ptr),
    ]
}
