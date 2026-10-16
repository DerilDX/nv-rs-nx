//! `fallout shared/tempeffects/bstempeffectgeometrydecal.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006838d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00688770` and frees 0x64 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_geometry_decal_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_8770, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x64u32]);
    }
    this
}

// Translated from 006885a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00a85580` and frees 0x34 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn ni_skin_instance_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00a8_5580, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x34u32]);
    }
    this
}

// Translated from 006885e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00688610` and frees 0x40 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_dismember_skin_instance_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_8610, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x40u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_38d0, bs_temp_effect_geometry_decal_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_85a0, ni_skin_instance_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_85e0, bs_dismember_skin_instance_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_temp_effect_geometry_decal_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_38d0, 0x0068_8770, 0x64);
    }

    #[test]
    fn ni_skin_instance_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_85a0, 0x00a8_5580, 0x34);
    }

    #[test]
    fn bs_dismember_skin_instance_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_85e0, 0x0068_8610, 0x40);
    }
}
