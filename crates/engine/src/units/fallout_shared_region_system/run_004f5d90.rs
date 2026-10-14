//! `fallout shared/region system/run_004f5d90` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f5d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float argument at `+4` (unnamed in the map; `ECX`, one stack
/// word, `RET 4`; an `FLD`/`FSTP` pair, so a signalling NaN is stored
/// quiet). `TESForm::LoadData` (`004861f0`) calls it as the setter of the
/// form's `TESWeightForm` component (the cast it makes first).
pub fn fn_004f5d90(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 4, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f5d90, fn_004f5d90(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_and_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        let this = Ptr::<()>::new(component);
        e.call(0x004f_5d90, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(component + 4), 2.5f32.to_bits());
        e.call(0x004f_5d90, &args![this, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(component + 4), 0xffc0_0001);
        e.call(0x004f_5d90, &args![this, 0x7fc0_1234u32]);
        assert_eq!(e.mem.u32(component + 4), 0x7fc0_1234);
        assert_eq!(e.mem.u32(component), 0);
    }
}
