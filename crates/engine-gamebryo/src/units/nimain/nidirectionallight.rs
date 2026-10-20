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

/// `NiAVObject::UpdateWorldData` (`00a68c60`, `thiscall`, one stack word).
const NI_AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;

// Translated from 00a75cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDirectionalLight::UpdateWorldData` (Xbox PDB), `thiscall`, one stack
/// word (passed on): `NiAVObject::UpdateWorldData`, then the world direction
/// (+0xf0, +0xf4, +0xf8) from the first column of the world rotation (+0x68,
/// +0x74, +0x80; `FLD`/`FSTP`, which quiets a signalling NaN), and the
/// revision word at +0xa8 counts up.
pub fn ni_directional_light_update_world_data(e: &mut Engine, this: Ptr, update: u32) {
    let t = this.addr();
    e.call(NI_AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update]);
    for (from, to) in [(0x68u32, 0xf0u32), (0x74, 0xf4), (0x80, 0xf8)] {
        let bits = x87_quiet_bits(e.mem.u32(t + from));
        e.mem.set_u32(t + to, bits);
    }
    let revision = e.mem.u32(t + 0xa8).wrapping_add(1);
    e.mem.set_u32(t + 0xa8, revision);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_5c20, ni_directional_light_ni_directional_light(Ptr) -> Ptr),
        entry!(0x00a75c70, ni_directional_light_get_rtti() -> Ptr),
        entry!(0x00a75c80, ni_directional_light_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_5cb0,
            ni_directional_light_update_world_data(Ptr, u32)
        ),
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

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_directional_light_update_world_data_copies_the_direction() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0xfc);
        e.mem.set_f32(this + 0x68, 0.5);
        e.mem.set_u32(this + 0x74, 0x7f80_0001);
        e.mem.set_f32(this + 0x80, -2.0);
        e.mem.set_u32(this + 0xa8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a7_5cb0, &args![this, 9u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (NI_AV_OBJECT_UPDATE_WORLD_DATA, vec![this, 9])
        );
        assert_eq!(e.mem.f32(this + 0xf0), 0.5);
        assert_eq!(e.mem.u32(this + 0xf4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0xf8), -2.0);
        assert_eq!(e.mem.u32(this + 0xa8), 5);
    }
}
