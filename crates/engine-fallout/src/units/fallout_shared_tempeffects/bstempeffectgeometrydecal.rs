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

/// The word getter `00559450` (`thiscall`: the word at `+0`).
const WORD_AT_0_00559450: u32 = 0x0055_9450;
/// The global `fn_00683a60` answers (`011f5b04`).
const GLOBAL_011F5B04: u32 = 0x011f_5b04;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, four words cleaned by the
/// callee: array, element size, count, destructor), `operator delete` (`00401030`, `cdecl`),
/// the partition destructor `00a81bd0` and `NiSkinInstance`'s destructor `00a85580`
/// (`thiscall`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const PARTITION_DESTRUCTOR_00A81BD0: u32 = 0x00a8_1bd0;
const NI_SKIN_INSTANCE_DESTRUCTOR: u32 = 0x00a8_5580;
/// `BSDismemberSkinInstance`'s vtable (`01069a84`, by its RTTI name).
const BS_DISMEMBER_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9a84;

// Translated from 004b6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::Dot` (engine map; the body matches): `x*x' + y*y' + z*z'` in x87 (each product
/// of two `float`s, the sums in the exe's order), stored to a `float` local and loaded back
/// into `ST0`.
pub fn ni_point3_dot(e: &mut Engine, this: Ptr, other: u32) -> f32 {
    let a = this.addr();
    let x = x87_mul(x87_load32(e.mem.u32(a)), x87_m32(e.mem.u32(other)));
    let y = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(4))),
        x87_m32(e.mem.u32(other.wrapping_add(4))),
    );
    let sum = x87_add(x, y);
    let z = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(8))),
        x87_m32(e.mem.u32(other.wrapping_add(8))),
    );
    let sum = x87_add(sum, z);
    f32::from_bits(x87_store32(sum))
}

// Translated from 006838b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `+0x1c` (through the getter `00559450` on
/// `this + 0x1c`).
pub fn fn_006838b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(WORD_AT_0_00559450, &args![this.addr().wrapping_add(0x1c)])
        .u32()
}

// Translated from 00683a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the word at `011f5b04` (no arguments).
pub fn fn_00683a60(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_011F5B04)
}

// Translated from 00685e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::vector deleting destructor` (engine map, from the vtable):
/// with flag bit 1 the array before `this` (its count in the word at `this - 4`) is
/// destroyed element by element (`00ec5fce` with 0x2c-byte elements and `00a81bd0`) and,
/// with bit 0, the block from `this - 4` freed; answers `this - 4`. Otherwise the one
/// partition is destroyed (`00a81bd0`) and, with bit 0, freed; answers `this`.
pub fn ni_skin_partition_partition_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    let a = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(a.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCT,
            &args![a, 0x2cu32, count, PARTITION_DESTRUCTOR_00A81BD0],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![a.wrapping_sub(4)]);
        }
        return a.wrapping_sub(4);
    }
    e.call(PARTITION_DESTRUCTOR_00A81BD0, &args![a]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![a]);
    }
    a
}

// Translated from 00688430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies nine words (`REP MOVSD`) from the argument to
/// `this + 0x68`.
pub fn fn_00688430(e: &mut Engine, this: Ptr, source: u32) {
    for i in 0..9u32 {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(this.addr().wrapping_add(0x68 + 4 * i), word);
    }
}

// Translated from 00688610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance`'s destructor (unnamed in the engine map; its scalar deleting
/// destructor `006885e0` runs it): sets the vtable `01069a84`; with the array at `+0x38`,
/// frees it (`operator delete`) and clears it and the count at `+0x34`; then
/// `NiSkinInstance`'s destructor (`00a85580`).
pub fn bs_dismember_skin_instance_destructor(e: &mut Engine, this: Ptr) {
    let a = this.addr();
    e.mem.set_u32(a, BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
    let array = e.mem.u32(a.wrapping_add(0x38));
    if array != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
        e.mem.set_u32(a.wrapping_add(0x38), 0);
        e.mem.set_u32(a.wrapping_add(0x34), 0);
    }
    e.call(NI_SKIN_INSTANCE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_6190, ni_point3_dot(Ptr, u32) -> f32),
        entry!(0x0068_38b0, fn_006838b0(Ptr) -> u32),
        entry!(0x0068_3a60, fn_00683a60() -> u32),
        entry!(
            0x0068_5e80,
            ni_skin_partition_partition_vector_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x0068_8430, fn_00688430(Ptr, u32)),
        entry!(0x0068_8610, bs_dismember_skin_instance_destructor(Ptr)),
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

#[cfg(test)]
mod b060_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    #[test]
    fn dot_product_rounds_to_float() {
        let mut e = Engine::new();
        let a = point(&mut e, 1.5, -2.0, 0.25);
        let b = point(&mut e, 4.0, 3.0, 8.0);
        let r = e.call(0x004b_6190, &args![a, b]).st0;
        assert_eq!(r, 2.0);
        let c = point(&mut e, 0.1, 0.2, 0.3);
        let r = e.call(0x004b_6190, &args![c, c]).st0;
        let (x, y, z) = (f64::from(0.1f32), f64::from(0.2f32), f64::from(0.3f32));
        let expected = ((x * x + y * y) + z * z) as f32;
        assert_eq!(r, f64::from(expected));
    }

    #[test]
    fn word_getters() {
        let mut e = Engine::new();
        e.register(WORD_AT_0_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 0x1234);
        assert_eq!(e.call(0x0068_38b0, &args![this]).u32(), 0x1234);
        e.map(0x011f_5000, 0x1000);
        e.set_global(GLOBAL_011F5B04, 0x5678u32);
        assert_eq!(e.call(0x0068_3a60, &[]).u32(), 0x5678);
    }

    #[test]
    fn partition_vector_deleting_destructor_handles_arrays_and_single_ones() {
        let mut e = Engine::new();
        for addr in [
            VECTOR_DESTRUCT,
            OPERATOR_DELETE,
            PARTITION_DESTRUCTOR_00A81BD0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let block = e.mem.alloc(0x60);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        assert_eq!(e.call(0x0068_5e80, &args![array, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![array, 0x2c, 2, PARTITION_DESTRUCTOR_00A81BD0]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
        assert_eq!(e.call(0x0068_5e80, &args![array, 2u32]).u32(), block);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(e.call(0x0068_5e80, &args![array, 1u32]).u32(), array);
        assert_eq!(calls(&e, PARTITION_DESTRUCTOR_00A81BD0), vec![vec![array]]);
        assert_eq!(calls(&e, OPERATOR_DELETE)[1], vec![array]);
        e.call(0x0068_5e80, &args![array, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn fn_00688430_copies_nine_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        let source = e.mem.alloc(0x28);
        for i in 0..10 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        e.call(0x0068_8430, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0x68), 0x100);
        assert_eq!(e.mem.u32(this + 0x88), 0x108);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
    }

    #[test]
    fn dismember_skin_instance_destructor_frees_the_array() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(NI_SKIN_INSTANCE_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x9000);
        e.call(0x0068_8610, &args![this]);
        assert_eq!(e.mem.u32(this), BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x9000]]);
        assert_eq!((e.mem.u32(this + 0x34), e.mem.u32(this + 0x38)), (0, 0));
        e.call(0x0068_8610, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(calls(&e, NI_SKIN_INSTANCE_DESTRUCTOR), vec![vec![this]; 2]);
    }
}
