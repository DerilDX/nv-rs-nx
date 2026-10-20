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

/// `NiGeometryData::LoadBinary` / `SaveBinary` (engine map, `00a67ea0` / `00a68520`;
/// `thiscall`, `RET 4`).
const NI_GEOMETRY_DATA_LOAD_BINARY: u32 = 0x00a6_7ea0;
const NI_GEOMETRY_DATA_SAVE_BINARY: u32 = 0x00a6_8520;

/// One transfer of the triangle count (+0x40, two bytes, element size 2) through the
/// stream's `NiBinaryStream` at `stream_slot` (`m_pkIstr` +0x24c or `m_pkOstr` +0x250) and its
/// function at `function_slot` (`m_pfnRead` +8 or `m_pfnWrite` +0xc), the element size kept
/// in the stream argument's own slot and the position (+4) advanced by the result.
fn transfer_triangle_count(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    stream_slot: u32,
    function_slot: u32,
) {
    let binary = e.mem.u32(stream.addr() + stream_slot);
    let function = e.mem.u32(binary.wrapping_add(function_slot));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), 2);
        let moved = e
            .call(
                function,
                &args![binary, this.addr() + 0x40, 2u32, element.addr(), 1u32],
            )
            .u32();
        let position = e.mem.u32(binary.wrapping_add(4)).wrapping_add(moved);
        e.mem.set_u32(binary.wrapping_add(4), position);
    });
}

// Translated from 00a8aa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiGeometryData::LoadBinary`, then the triangle count (`m_usTriangles`, +0x40).
pub fn ni_tri_based_geom_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_GEOMETRY_DATA_LOAD_BINARY, &args![this, stream]);
    transfer_triangle_count(e, this, stream, 0x24c, 8);
}

// Translated from 00a8aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::SaveBinary` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiGeometryData::SaveBinary`, then the triangle count (+0x40).
pub fn ni_tri_based_geom_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_GEOMETRY_DATA_SAVE_BINARY, &args![this, stream]);
    transfer_triangle_count(e, this, stream, 0x250, 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_aa30, ni_tri_based_geom_data_load_binary(Ptr, Ptr)),
        entry!(0x00a8_aa70, ni_tri_based_geom_data_save_binary(Ptr, Ptr)),
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

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    const TRANSFER: u32 = 0x0e72_0000;

    fn run(address: u32, base: u32, slot: u32, function_slot: u32) {
        let mut e = Engine::new();
        e.register(base, |_, _| Ret::default());
        e.register(TRANSFER, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            e.mem.set_u16(a[1], 0x1234);
            2u32.into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 10);
        e.mem.set_u32(binary + function_slot, TRANSFER);
        e.mem.set_u32(stream + slot, binary);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(order, vec![address, base, TRANSFER]);
        assert_eq!(log[1].1, vec![this, stream]);
        assert_eq!(log[2].1[..3], [binary, this + 0x40, 2]);
        assert_eq!(e.mem.u16(this + 0x40), 0x1234);
        assert_eq!(e.mem.u32(binary + 4), 12);
    }

    #[test]
    fn load_binary_reads_the_triangle_count() {
        run(0x00a8_aa30, NI_GEOMETRY_DATA_LOAD_BINARY, 0x24c, 8);
    }

    #[test]
    fn save_binary_writes_the_triangle_count() {
        run(0x00a8_aa70, NI_GEOMETRY_DATA_SAVE_BINARY, 0x250, 0xc);
    }
}
