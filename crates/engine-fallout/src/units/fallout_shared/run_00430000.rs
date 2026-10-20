//! `fallout shared/run_00430000` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004300a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the float at `this` to 0 (`FLDZ`, `FSTP`) and returns `this` (no
/// name in the engine map).
pub fn fn_004300a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_f32(this.addr(), 0.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0043_00a0, fn_004300a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004300a0_stores_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x0043_00a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
    }
}
