//! `Havok SDK/hkpsimplecontactconstraintinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d72090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleContactConstraintDataAddALittleSurfaceViscosity` (Xbox PDB), cdecl: clears the
/// floats at +0xc, +0x14 and +0x1c of `data` (the exe stores the x87 zero with `FST`).
pub fn hk_simple_contact_constraint_data_add_a_little_surface_viscosity(e: &mut Engine, data: Ptr) {
    let base = data.addr();
    e.mem.set_f32(base + 0xc, 0.0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x1c, 0.0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d7_2090,
        hk_simple_contact_constraint_data_add_a_little_surface_viscosity(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_viscosity_terms_are_cleared() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x30);
        for offset in [0xc, 0x10, 0x14, 0x1c] {
            e.mem.set_f32(data + offset, 5.0);
        }
        e.call(0x00d7_2090, &args![data]);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
        assert_eq!(e.mem.u32(data + 0x1c), 0);
        // The float between the cleared ones is left alone.
        assert_eq!(e.mem.f32(data + 0x10), 5.0);
    }
}
