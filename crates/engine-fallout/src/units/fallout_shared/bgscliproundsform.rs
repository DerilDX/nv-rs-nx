//! `fallout shared/bgscliproundsform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00475100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSClipRoundsForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the clip rounds byte (+4) 12.
pub fn bgs_clip_rounds_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 4, 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00475100,
        bgs_clip_rounds_form_initialize_data_component(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_rounds_start_at_twelve() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        e.call(0x0047_5100, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff0c);
    }
}
