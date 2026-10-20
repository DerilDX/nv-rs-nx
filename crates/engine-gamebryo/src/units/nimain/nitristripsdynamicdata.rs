//! `NiMain/nitristripsdynamicdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriStripsDynamicData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_STRIPS_DYNAMIC_DATA_RTTI: u32 = 0x011f_5e8c;

// Translated from 00a92470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_strips_dynamic_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_STRIPS_DYNAMIC_DATA_RTTI)
}

// Translated from 00a924b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a76040`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x54 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_tri_strips_dynamic_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_6040;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x54u32]);
    }
    this
}

// Translated from 00a923b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetActiveTriangleCount` (Xbox PDB): the 16-bit active count at
/// +0x52.
pub fn ni_tri_strips_dynamic_data_get_active_triangle_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(0x52))
}

// Translated from 00a92490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::SetActiveTriangleCount` (Xbox PDB): the active count at +0x52,
/// and, when the strip count (`NiTriStripsData::m_usStrips`, +0x44) is not 0, the first
/// strip's length (the first word of `m_pusStripLengths`, +0x48) too.
pub fn ni_tri_strips_dynamic_data_set_active_triangle_count(e: &mut Engine, this: Ptr, count: u16) {
    let strips = e.mem.u16(this.addr().wrapping_add(0x44));
    e.mem.set_u16(this.addr().wrapping_add(0x52), count);
    if strips != 0 {
        let lengths = e.mem.u32(this.addr().wrapping_add(0x48));
        e.mem.set_u16(lengths, count);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a92470, ni_tri_strips_dynamic_data_get_rtti() -> Ptr),
        entry!(0x00a924b0, ni_tri_strips_dynamic_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a923b0, ni_tri_strips_dynamic_data_get_active_triangle_count(Ptr) -> u16),
        entry!(
            0x00a92490,
            ni_tri_strips_dynamic_data_set_active_triangle_count(Ptr, u16)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_strips_dynamic_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a92470, &args![]).u32(),
            NI_TRI_STRIPS_DYNAMIC_DATA_RTTI
        );
    }

    #[test]
    fn ni_tri_strips_dynamic_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_6040, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a924b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a924b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x54]]);
        assert_eq!(calls_to(&e, 0x00a7_6040), vec![vec![this]; 2]);
    }

    #[test]
    fn active_triangle_count_get_and_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        let lengths = e.mem.alloc(4);
        e.mem.set_u32(this + 0x48, lengths);
        e.mem.set_u16(lengths, 9);
        e.call(0x00a92490, &args![this, 5u32]);
        assert_eq!(e.call(0x00a923b0, &args![this]).u32() & 0xffff, 5);
        assert_eq!(e.mem.u16(lengths), 9);
        e.mem.set_u16(this + 0x44, 1);
        e.call(0x00a92490, &args![this, 6u32]);
        assert_eq!(e.call(0x00a923b0, &args![this]).u32() & 0xffff, 6);
        assert_eq!(e.mem.u16(lengths), 6);
    }
}
