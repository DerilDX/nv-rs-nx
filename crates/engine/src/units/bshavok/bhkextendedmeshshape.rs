//! `BSHavok/bhkextendedmeshshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ccc420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccbc00`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_extended_mesh_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_bc00;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00cccfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccc720`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_extended_mesh_shape_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_c720;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00ccb990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8684.
pub fn bhk_extended_mesh_shape_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8684
}

// Translated from 00ccbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_867c.
pub fn bhk_extended_mesh_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_867c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ccb990, bhk_extended_mesh_shape_data_get_r_t_t_i() -> u32),
        entry!(0x00ccbb30, bhk_extended_mesh_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccc420, bhk_extended_mesh_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cccfb0, bhk_extended_mesh_shape_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_extended_mesh_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_bc00, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ccc420, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ccc420, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00cc_bc00), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_extended_mesh_shape_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_c720, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cccfb0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cccfb0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_c720), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_extended_mesh_shape_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_b990, &args![]).u32(), 0x0126_8684);
    }

    #[test]
    fn bhk_extended_mesh_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_bb30, &args![]).u32(), 0x0126_867c);
    }
}
