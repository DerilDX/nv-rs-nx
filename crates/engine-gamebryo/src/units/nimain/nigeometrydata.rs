//! `NiMain/nigeometrydata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryData::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOMETRY_DATA_RTTI: u32 = 0x011f_4884;

// Translated from 00a67bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOMETRY_DATA_RTTI)
}

// Translated from 00a68800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a67c70`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_geometry_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_7c70;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

/// The base constructor `00a5d3a0` (engine map unit `niobject.cpp`, unnamed; `thiscall`, no
/// stack words).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
/// `NiGeometryData`'s vtable (`0109bf9c`; its RTTI type descriptor reads
/// `.?AVNiGeometryData@@`).
const NI_GEOMETRY_DATA_VTABLE: u32 = 0x0109_bf9c;

// Translated from 00a67c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::NiGeometryData` (Xbox PDB), `thiscall`, no stack words; PC offsets match
/// the Xbox PDB. Runs the base constructor `00a5d3a0`, stores the vtable `0109bf9c`, clears
/// `m_usVertices` (+0x8), `m_usDataFlags` (+0xc), `m_usDirtyFlags` (+0xe), the words +0x20 to
/// +0x34 (`m_pkVertex` .. `m_pkBuffData`) and the bytes +0x38 to +0x3b, and sets
/// `m_bSaveVertexData` (+0x3c) to 1. Then, unless `m_spAdditionalGeomData` (+0x30, just
/// cleared) is set and its vtable slot +0x8c answers a set byte, keeps only the bits 0x8fff of
/// `m_usDirtyFlags`. Answers `this`.
pub fn ni_geometry_data_ni_geometry_data(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![this]);
    e.mem.set_u16(base + 0xc, 0);
    e.mem.set_u32(base, NI_GEOMETRY_DATA_VTABLE);
    e.mem.set_u16(base + 0x8, 0);
    e.mem.set_u16(base + 0xe, 0);
    for offset in (0x20u32..=0x34).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in 0x38u32..=0x3b {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u8(base + 0x3c, 1);
    let additional = e.mem.u32(base + 0x30);
    let keeps_flags = additional != 0 && e.vcall(additional, 0x8c, &args![]).bool();
    if !keeps_flags {
        let flags = e.mem.u16(base + 0xe);
        e.mem.set_u16(base + 0xe, flags & 0x8fff);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_7c00, ni_geometry_data_ni_geometry_data(Ptr) -> Ptr),
        entry!(0x00a67bf0, ni_geometry_data_get_rtti() -> Ptr),
        entry!(0x00a68800, ni_geometry_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geometry_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a67bf0, &args![]).u32(), NI_GEOMETRY_DATA_RTTI);
    }

    #[test]
    fn ni_geometry_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_7c70, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a68800, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a68800, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00a6_7c70), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn ni_geometry_data_ni_geometry_data_clears_the_fields() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        for offset in (0x4u32..0x40).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00a6_7c00, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u16(this + 0x8), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0xffff);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
        for offset in (0x20u32..0x3c).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(e.mem.u8(this + 0x3d), 0xff);
        let log = e.call_log.take().unwrap();
        let constructed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_OBJECT_CONSTRUCT_00A5D3A0)
            .collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![this]);
    }
}
