//! `NiMain/nilight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_RTTI: u32 = 0x011f_4a28;

// Translated from 00a72000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_RTTI)
}

/// `00a69070`: the jump target of `00a71f50`.
const JUMP_TARGET_00A71F50: u32 = 0x00a6_9070;

/// `00a6ff10`: the jump target of `00a90010`.
const JUMP_TARGET_00A90010: u32 = 0x00a6_ff10;

// Translated from 00a71f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a69070` (engine map: `NiDynamicEffect::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A71F50, &args![this, stream]);
}

// Translated from 00a90010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a6ff10` (engine map: `NiCamera::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A90010, &args![this, stream]).bool()
}

/// `NiDynamicEffect::NiDynamicEffect` (engine map, `00a69320`, `thiscall`, no words).
const NI_DYNAMIC_EFFECT_CONSTRUCT: u32 = 0x00a6_9320;
/// `NiLight`'s vtable (`0109ce4c`, named by its RTTI locator).
const NI_LIGHT_VTABLE: u32 = 0x0109_ce4c;

// Translated from 00a71f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nilight.cpp`): the `NiLight` constructor by its
/// body (it sets the vtable whose RTTI locator names `NiLight`). `thiscall`, no words. Runs
/// `NiDynamicEffect::NiDynamicEffect` (`00a69320`), sets the vtable `0109ce4c`, stores 0.0 in
/// the nine floats +0xc8..+0xe8 (the Xbox PDB's ambient, diffuse and specular `NiColor`s,
/// `m_kAmb`/`m_kDiff`/`m_kSpec`, 0x2c lower on PC), clears the word at +0xec
/// (`m_pvRendererData`), then stores 1.0 in the ten floats +0xc4..+0xe8 (`m_fDimmer` and the
/// three colours), in the exe's order. Answers `this`.
pub fn fn_00a71f60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_DYNAMIC_EFFECT_CONSTRUCT, &args![this]);
    let base = this.addr();
    e.mem.set_u32(base, NI_LIGHT_VTABLE);
    for offset in (0xc8..=0xe8).step_by(4) {
        e.mem.set_f32(base.wrapping_add(offset), 0.0);
    }
    e.mem.set_u32(base.wrapping_add(0xec), 0);
    for offset in (0xc4..=0xe8).step_by(4) {
        e.mem.set_f32(base.wrapping_add(offset), 1.0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_1f60, fn_00a71f60(Ptr) -> Ptr),
        entry!(0x00a72000, ni_light_get_rtti() -> Ptr),
        entry!(0x00a7_1f50, ni_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_0010, ni_light_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a72000, &args![]).u32(), NI_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a71f50_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_1f50;
        const TARGET: u32 = 0x00a6_9070;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a90010_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_0010;
        const TARGET: u32 = 0x00a6_ff10;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn fn_00a71f60_sets_the_dimmer_and_colours_to_one() {
        let mut e = Engine::new();
        e.register(NI_DYNAMIC_EFFECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0xec, 0xdead);
            Ret::default()
        });
        let this = e.mem.alloc(0xf0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_1f60, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_DYNAMIC_EFFECT_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, [this]);
        assert_eq!(e.mem.u32(this), NI_LIGHT_VTABLE);
        for offset in (0xc4..=0xe8).step_by(4) {
            assert_eq!(e.mem.f32(this + offset).to_bits(), 1.0f32.to_bits());
        }
        assert_eq!(e.mem.u32(this + 0xec), 0);
    }
}
