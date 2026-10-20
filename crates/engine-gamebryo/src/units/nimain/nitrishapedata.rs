//! `NiMain/nitrishapedata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriShapeData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_SHAPE_DATA_RTTI: u32 = 0x011f_4a88;

// Translated from 00a7b860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_shape_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_SHAPE_DATA_RTTI)
}

// Translated from 00a7b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7b8c0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x58 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_tri_shape_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_b8c0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x58u32]);
    }
    this
}

layout! {
    /// `NiTriShapeData` (Xbox PDB), an `NiTriBasedGeomData`; 0x58 bytes on PC as on the
    /// Xbox (the sized delete of `00a7b920`). The fields its constructor sets, at the
    /// Xbox PDB's offsets, which the PC constructor writes.
    pub struct NiTriShapeData: 0x58 {
        /// `m_uiTriListLength` (Xbox PDB).
        0x44 m_uiTriListLength: u32,
        /// `m_pusTriList` (Xbox PDB): `u16*`.
        0x48 m_pusTriList: u32,
        /// `m_pkSharedNormals` (Xbox PDB): `NiTriShapeData::SharedNormalArray*`.
        0x4C m_pkSharedNormals: u32,
        /// `m_usSharedNormalsArraySize` (Xbox PDB).
        0x50 m_usSharedNormalsArraySize: u16,
        /// `m_pkSNAMemoryBlocks` (Xbox PDB): `NiTriShapeData::SNAMemBlock*`.
        0x54 m_pkSNAMemoryBlocks: u32,
    }
}

/// `NiTriBasedGeomData::NiTriBasedGeomData` (engine map, `00a8a970`): `thiscall`, no stack
/// words.
const NI_TRI_BASED_GEOM_DATA_CONSTRUCT: u32 = 0x00a8_a970;
/// `NiTriShapeData`'s vtable (`0109dac4`, RTTI `.?AVNiTriShapeData@@`).
const NI_TRI_SHAPE_DATA_VTABLE: u32 = 0x0109_dac4;

// Translated from 00a7b690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::NiTriShapeData_ov2` (Xbox PDB). `thiscall`, no stack words: runs the
/// `NiTriBasedGeomData` constructor (`00a8a970`), clears the triangle list length, the
/// triangle list, the shared normals and their memory blocks, puts this class's vtable
/// (`0109dac4`) at +0 and clears the 16-bit shared normals count, in the exe's order.
/// Answers `this`.
pub fn ni_tri_shape_data_ni_tri_shape_data_ov2(
    e: &mut Engine,
    this: Ptr<NiTriShapeData>,
) -> Ptr<NiTriShapeData> {
    e.call(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, &args![this]);
    e.set(this, NiTriShapeData::m_uiTriListLength, 0);
    e.set(this, NiTriShapeData::m_pusTriList, 0);
    e.set(this, NiTriShapeData::m_pkSharedNormals, 0);
    e.set(this, NiTriShapeData::m_pkSNAMemoryBlocks, 0);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_DATA_VTABLE);
    e.set(this, NiTriShapeData::m_usSharedNormalsArraySize, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7b860, ni_tri_shape_data_get_rtti() -> Ptr),
        entry!(0x00a7b920, ni_tri_shape_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_b690,
            ni_tri_shape_data_ni_tri_shape_data_ov2(Ptr<NiTriShapeData>) -> Ptr<NiTriShapeData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_shape_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7b860, &args![]).u32(), NI_TRI_SHAPE_DATA_RTTI);
    }

    #[test]
    fn ni_tri_shape_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_b8c0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7b920, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7b920, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x58]]);
        assert_eq!(calls_to(&e, 0x00a7_b8c0), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_tri_shape_data_ov2_runs_the_base_then_clears_its_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        for offset in (0..0x58).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.register(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_b690, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_TRI_BASED_GEOM_DATA_CONSTRUCT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_DATA_VTABLE);
        for offset in [0x44, 0x48, 0x4c, 0x54] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        // Only the low half of the word at +0x50 is the 16-bit count.
        assert_eq!(e.mem.u32(this + 0x50), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0x40), 0xdead_beef);
    }
}
