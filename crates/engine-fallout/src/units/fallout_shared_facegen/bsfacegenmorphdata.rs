//! `fallout shared/facegen/bsfacegenmorphdata.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00661d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `00661d40` and frees 0x8 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_00661d60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0066_1d40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x8u32]);
    }
    this
}

// Translated from 00661db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHead` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00661de0` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_morph_data_head_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_1de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 006623a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHair` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006623d0` and frees 0xc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_morph_data_hair_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_23d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xcu32]);
    }
    this
}

// Translated from 006626f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBaseMorphExtraData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00662720` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_base_morph_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_2720, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00661d30 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5b94 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00661d30(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5b94
}

// Translated from 00661d90 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5bb8 without reading the receiver. The map names this body `BSFaceGenMorphDataHead::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00661d90(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5bb8
}

// Translated from 00662390 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5ba8 without reading the receiver. The map names this body `BSFaceGenMorphDataHair::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00662390(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5ba8
}

// Translated from 006626e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5ba0 without reading the receiver. The map names this body `BSFaceGenBaseMorphExtraData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006626e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5ba0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_1d30, fn_00661d30(Ptr) -> u32),
        entry!(0x0066_1d90, fn_00661d90(Ptr) -> u32),
        entry!(0x0066_2390, fn_00662390(Ptr) -> u32),
        entry!(0x0066_26e0, fn_006626e0(Ptr) -> u32),
        entry!(0x0066_1d60, fn_00661d60(Ptr, u32) -> Ptr),
        entry!(0x0066_1db0, bs_face_gen_morph_data_head_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_23a0, bs_face_gen_morph_data_hair_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_26f0, bs_face_gen_base_morph_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
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
    fn fn_00661d60_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_1d60, 0x0066_1d40, 0x8);
    }

    #[test]
    fn bs_face_gen_morph_data_head_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_1db0, 0x0066_1de0, 0x18);
    }

    #[test]
    fn bs_face_gen_morph_data_hair_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_23a0, 0x0066_23d0, 0xc);
    }

    #[test]
    fn bs_face_gen_base_morph_extra_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero()
    {
        check_deleting_destructor(0x0066_26f0, 0x0066_2720, 0x18);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00661d30_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_1d30, &args![0x1000u32]).u32(), 0x011d_5b94);
        assert_eq!(e.call(0x0066_1d30, &args![0u32]).u32(), 0x011d_5b94);
    }

    #[test]
    fn fn_00661d90_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_1d90, &args![0x1000u32]).u32(), 0x011d_5bb8);
        assert_eq!(e.call(0x0066_1d90, &args![0u32]).u32(), 0x011d_5bb8);
    }

    #[test]
    fn fn_00662390_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_2390, &args![0x1000u32]).u32(), 0x011d_5ba8);
        assert_eq!(e.call(0x0066_2390, &args![0u32]).u32(), 0x011d_5ba8);
    }

    #[test]
    fn fn_006626e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_26e0, &args![0x1000u32]).u32(), 0x011d_5ba0);
        assert_eq!(e.call(0x0066_26e0, &args![0u32]).u32(), 0x011d_5ba0);
    }
}
