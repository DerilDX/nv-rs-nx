//! `NiMain/nitribasedgeomdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriBasedGeomData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_BASED_GEOM_DATA_RTTI: u32 = 0x011f_5b90;

// Translated from 00a8a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_based_geom_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_BASED_GEOM_DATA_RTTI)
}

/// `NiGeometryData::NiGeometryData` (engine map, `00a67c00`, `thiscall`, no words).
const NI_GEOMETRY_DATA_CONSTRUCT: u32 = 0x00a6_7c00;
/// `NiTriBasedGeomData`'s vtable (`0109f2f4`, named by its RTTI locator).
const NI_TRI_BASED_GEOM_DATA_VTABLE: u32 = 0x0109_f2f4;

// Translated from 00a8a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::NiTriBasedGeomData` (Xbox PDB): `thiscall`, no words. Runs
/// `NiGeometryData::NiGeometryData` (`00a67c00`), clears the 16-bit word at +0x40 (the
/// triangle count in the Xbox PDB's `NiTriBasedGeomData::m_usTriangles`; offset from the PC
/// code), sets the class's vtable (`0109f2f4`) and answers `this`.
pub fn ni_tri_based_geom_data_ni_tri_based_geom_data(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_GEOMETRY_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u16(this.addr().wrapping_add(0x40), 0);
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_DATA_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8a9f0, ni_tri_based_geom_data_get_rtti() -> Ptr),
        entry!(
            0x00a8_a970,
            ni_tri_based_geom_data_ni_tri_based_geom_data(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_based_geom_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a8a9f0, &args![]).u32(),
            NI_TRI_BASED_GEOM_DATA_RTTI
        );
    }

    #[test]
    fn ni_tri_based_geom_data_ni_tri_based_geom_data_clears_the_count_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x40, 0xffff_ffff);
            Ret::default()
        });
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_a970, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_GEOMETRY_DATA_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, [this]);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_DATA_VTABLE);
        // Only the 16-bit word is cleared.
        assert_eq!(e.mem.u32(this + 0x40), 0xffff_0000);
    }
}
