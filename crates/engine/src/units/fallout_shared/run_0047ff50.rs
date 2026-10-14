//! `fallout shared/run_0047ff50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `u16` argument at `+4` (unnamed in the map; `ECX`, one stack
/// word, `RET 4`). `TESForm::LoadData` (`004861f0`) calls it as the setter
/// of the form's `TESAttackDamageForm` component (the cast it makes first).
pub fn fn_0047ff50(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 4, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047ff50, fn_0047ff50(Ptr, u16))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_low_half_of_the_word_at_plus_4() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        e.mem.set_u32(component, 0x1111_1111);
        e.mem.set_u32(component + 4, 0xaaaa_aaaa);
        e.call(
            0x0047_ff50,
            &args![Ptr::<()>::new(component), 0xdead_1234u32],
        );
        assert_eq!(e.mem.u32(component + 4), 0xaaaa_1234);
        assert_eq!(e.mem.u32(component), 0x1111_1111);
    }
}
