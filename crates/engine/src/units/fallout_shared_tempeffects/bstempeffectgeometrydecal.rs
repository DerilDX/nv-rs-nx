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

// Translated from 006838a0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6b04 without reading the receiver. The map names this body `BSTempEffectGeometryDecal::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006838a0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6b04
}

// Translated from 00688590 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f5b2c without reading the receiver. The map names this body `NiSkinInstance::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00688590(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_5b2c
}

// Translated from 006885d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f49d8 without reading the receiver. The map names this body `BSDismemberSkinInstance::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006885d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_49d8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_38a0, fn_006838a0(Ptr) -> u32),
        entry!(0x0068_8590, fn_00688590(Ptr) -> u32),
        entry!(0x0068_85d0, fn_006885d0(Ptr) -> u32),
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

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006838a0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_38a0, &args![0x1000u32]).u32(), 0x011d_6b04);
        assert_eq!(e.call(0x0068_38a0, &args![0u32]).u32(), 0x011d_6b04);
    }

    #[test]
    fn fn_00688590_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_8590, &args![0x1000u32]).u32(), 0x011f_5b2c);
        assert_eq!(e.call(0x0068_8590, &args![0u32]).u32(), 0x011f_5b2c);
    }

    #[test]
    fn fn_006885d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_85d0, &args![0x1000u32]).u32(), 0x011f_49d8);
        assert_eq!(e.call(0x0068_85d0, &args![0u32]).u32(), 0x011f_49d8);
    }
}
