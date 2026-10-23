//! `fallout/magic/magicfogprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00819a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a float setter (`thiscall`, `RET 4`): stores the argument at
/// +0x18 through the x87 stack (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_00819a50(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x18, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_9a50, fn_00819a50(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_setter_stores_at_0x18_quieting_nans() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        fn_00819a50(&mut e, this, 2.5);
        assert_eq!(e.mem.f32(this.addr() + 0x18), 2.5);
        fn_00819a50(&mut e, this, f32::from_bits(0x7f80_0001));
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
    }
}
