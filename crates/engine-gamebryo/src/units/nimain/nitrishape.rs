//! `NiMain/nitrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriShape::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_SHAPE_RTTI: u32 = 0x011f_4a40;

// Translated from 00a74520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_SHAPE_RTTI)
}

// Translated from 00a74530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a89760`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0xc4 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00a74530(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_9760;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

/// `NiTriBasedGeom::NiTriBasedGeom` (engine map, `00a89720`): `thiscall`, one stack word.
const NI_TRI_BASED_GEOM_CONSTRUCT: u32 = 0x00a8_9720;
/// `NiTriShape`'s vtable (`0109d454`, RTTI `.?AVNiTriShape@@`).
const NI_TRI_SHAPE_VTABLE: u32 = 0x0109_d454;

// Translated from 00a74480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::NiTriShape_ov2` (Xbox PDB). `thiscall`, `RET 4`: runs the
/// `NiTriBasedGeom` constructor (`00a89720`) on `this` with the one stack word (the
/// geometry data), then puts this class's vtable (`0109d454`) at +0. Answers `this`.
pub fn ni_tri_shape_ni_tri_shape_ov2(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(NI_TRI_BASED_GEOM_CONSTRUCT, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a74520, ni_tri_shape_get_rtti() -> Ptr),
        entry!(0x00a74530, fn_00a74530(Ptr, u32) -> Ptr),
        entry!(0x00a7_4480, ni_tri_shape_ni_tri_shape_ov2(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a74520, &args![]).u32(), NI_TRI_SHAPE_RTTI);
    }

    #[test]
    fn fn_00a74530_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_9760, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a74530, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a74530, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xc4]]);
        assert_eq!(calls_to(&e, 0x00a8_9760), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_tri_shape_ov2_constructs_the_base_with_the_data_then_sets_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.register(NI_TRI_BASED_GEOM_CONSTRUCT, |e, a| {
            // The base writes its own vtable; the class's replaces it afterwards.
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_4480, &args![this, 0x7000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_TRI_BASED_GEOM_CONSTRUCT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this, 0x7000]]);
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_VTABLE);
    }
}
