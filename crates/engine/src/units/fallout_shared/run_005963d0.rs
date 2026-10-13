//! `fallout shared/run_005963d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00598040 (decompiled, FalloutNV.exe 1.4.0.525)
/// A plain getter: the `float` at `+0x3c` of `this`, returned in `ST0`. The
/// engine map has no name for it; the translations that call it
/// (`animation.rs`, `inventorychanges.rs`) use it on a sequence object, which
/// this body does not confirm.
pub fn fn_00598040(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00598040, fn_00598040(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_offset_0x3c() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x3c, 1.5);
        e.mem.set_f32(object + 0x38, 9.0);
        assert_eq!(e.call(0x0059_8040, &args![object]).f32(), 1.5);
    }
}
