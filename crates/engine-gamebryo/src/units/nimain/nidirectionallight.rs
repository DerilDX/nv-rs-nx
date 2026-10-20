//! `NiMain/nidirectionallight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDirectionalLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_DIRECTIONAL_LIGHT_RTTI: u32 = 0x011f_4a68;

// Translated from 00a75c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDirectionalLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_directional_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DIRECTIONAL_LIGHT_RTTI)
}

// Translated from 00a75c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDirectionalLight::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a72010`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xfc bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_directional_light_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_2010;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xfcu32]);
    }
    this
}

/// The base constructor `00a71f60` (engine map: `NiMain/nilight.cpp`, unnamed): `thiscall`,
/// no stack words.
const NI_LIGHT_BASE_CONSTRUCT: u32 = 0x00a7_1f60;
/// `NiDirectionalLight`'s vtable (`0109d7b4`, RTTI `.?AVNiDirectionalLight@@`).
const NI_DIRECTIONAL_LIGHT_VTABLE: u32 = 0x0109_d7b4;

// Translated from 00a75c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDirectionalLight::NiDirectionalLight` (Xbox PDB). `thiscall`, no stack words: runs the
/// base constructor `00a71f60`, puts this class's vtable (`0109d7b4`) at +0, sets the float
/// at +0xf0 to 1.0 (`FLD1`), the byte at +0x9d to 3 and the floats at +0xf4 and +0xf8 to 0.0
/// (`FLDZ`), in the exe's order. Answers `this`.
///
/// The Xbox PDB's last field of the class is `m_kWorldDir` (three floats) and its
/// `NiDynamicEffect` has `m_ucEffectType` one byte after `m_bOn`; the PC offsets of those
/// names were not confirmed, so the fields are named by offset.
pub fn ni_directional_light_ni_directional_light(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_LIGHT_BASE_CONSTRUCT, &args![this]);
    let this_addr = this.addr();
    e.mem.set_u32(this_addr, NI_DIRECTIONAL_LIGHT_VTABLE);
    e.mem
        .set_u32(this_addr.wrapping_add(0xf0), x87_store32(1.0));
    e.mem.set_u8(this_addr.wrapping_add(0x9d), 3);
    e.mem
        .set_u32(this_addr.wrapping_add(0xf4), x87_store32(0.0));
    e.mem
        .set_u32(this_addr.wrapping_add(0xf8), x87_store32(0.0));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_5c20, ni_directional_light_ni_directional_light(Ptr) -> Ptr),
        entry!(0x00a75c70, ni_directional_light_get_rtti() -> Ptr),
        entry!(0x00a75c80, ni_directional_light_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_directional_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a75c70, &args![]).u32(),
            NI_DIRECTIONAL_LIGHT_RTTI
        );
    }

    #[test]
    fn ni_directional_light_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_2010, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a75c80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a75c80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xfc]]);
        assert_eq!(calls_to(&e, 0x00a7_2010), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_directional_light_constructor_sets_its_fields_after_the_base() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xfc);
        for offset in (0..0xfc).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.register(NI_LIGHT_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5c20, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_LIGHT_BASE_CONSTRUCT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_DIRECTIONAL_LIGHT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xf0), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0xf4), 0);
        assert_eq!(e.mem.u32(this + 0xf8), 0);
        assert_eq!(e.mem.u8(this + 0x9d), 3);
        assert_eq!(e.mem.u8(this + 0x9c), 0xef);
        assert_eq!(e.mem.u8(this + 0x9e), 0xad);
    }
}
