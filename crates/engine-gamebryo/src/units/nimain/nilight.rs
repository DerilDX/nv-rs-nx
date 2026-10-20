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

/// The global object `NiLight::~NiLight` notifies (`011f4748`; called through
/// its vtable slot `+0xf8` with the light).
const LIGHT_LISTENER: u32 = 0x011f_4748;
/// `00a69030` (`nidynamiceffect.cpp`, `thiscall`, no stack words): the
/// `NiDynamicEffect` destructor body.
const NI_DYNAMIC_EFFECT_DESTRUCT: u32 = 0x00a6_9030;

// Translated from 00a72010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::~NiLight` (Xbox PDB), `thiscall`, no stack words: stores
/// `NiLight`'s vtable, tells the object in `011f4748` (when there is one)
/// through its vtable slot `+0xf8` with the light, then jumps to the
/// `NiDynamicEffect` destructor body `00a69030`.
pub fn ni_light_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_LIGHT_VTABLE);
    let listener = e.global::<u32>(LIGHT_LISTENER);
    if listener != 0 {
        e.vcall(listener, 0xf8, &args![this]);
    }
    e.call(NI_DYNAMIC_EFFECT_DESTRUCT, &args![this]);
}

/// `NiDynamicEffect::LoadBinary` (`00a68ea0`, `thiscall`, the stream).
const NI_DYNAMIC_EFFECT_LOAD_BINARY: u32 = 0x00a6_8ea0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the colour, the stream).
const NI_COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00a72080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLight::LoadBinary` (Xbox PDB), `thiscall`, one stack word (the stream):
/// `NiDynamicEffect::LoadBinary`, then one 4-byte item into the dimmer at
/// `+0xc4` through the stream's binary reader (the object at `stream+0x24c`:
/// its function at `+8` is called `cdecl` with the reader, the destination, 4,
/// the address of a word holding 4 (the argument's own stack slot) and 1, and
/// its result is added to the reader's `+4`), then the three colours at
/// `+0xc8`, `+0xd4` and `+0xe0` (`NiColor::LoadBinary`).
pub fn ni_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(NI_DYNAMIC_EFFECT_LOAD_BINARY, &args![this, stream]);
    // NiStream +0x24c: the binary reader.
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(reader + 8);
    let done = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 4);
        e.call(read, &args![reader, t + 0xc4, 4u32, slot, 1u32])
            .u32()
    });
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
    for colour in [0xc8u32, 0xd4, 0xe0] {
        e.call(NI_COLOR_LOAD_BINARY, &args![t + colour, stream]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_1f60, fn_00a71f60(Ptr) -> Ptr),
        entry!(0x00a72000, ni_light_get_rtti() -> Ptr),
        entry!(0x00a7_1f50, ni_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_0010, ni_light_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_2010, ni_light_destructor(Ptr)),
        entry!(0x00a7_2080, ni_light_load_binary(Ptr, Ptr)),
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

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_light_destructor_notifies_the_listener_when_there_is_one() {
        const NOTIFY: u32 = 0x0123_4000;
        let mut e = Engine::new();
        e.map(LIGHT_LISTENER, 4);
        e.register(NOTIFY, |_, _| Ret::default());
        e.register(NI_DYNAMIC_EFFECT_DESTRUCT, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xf8, NOTIFY);
        let listener = e.mem.alloc(4);
        e.mem.set_u32(listener, vtable);
        let this = e.mem.alloc(0xfc);
        let run = |e: &mut Engine| -> Vec<(u32, Vec<u32>)> {
            e.call_log = Some(vec![]);
            e.call(0x00a7_2010, &args![this]);
            e.call_log.take().unwrap()[1..].to_vec()
        };
        assert_eq!(run(&mut e), vec![(NI_DYNAMIC_EFFECT_DESTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), NI_LIGHT_VTABLE);
        e.set_global::<u32>(LIGHT_LISTENER, listener);
        assert_eq!(
            run(&mut e),
            vec![
                (NOTIFY, vec![listener, this]),
                (NI_DYNAMIC_EFFECT_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn ni_light_load_binary_reads_the_dimmer_and_three_colours() {
        const READ: u32 = 0x0123_5000;
        let mut e = Engine::new();
        e.register(NI_DYNAMIC_EFFECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x3f00_0000);
            4u32.into_ret()
        });
        let reader = e.mem.alloc(0x10);
        e.mem.set_u32(reader + 8, READ);
        e.mem.set_u32(reader + 4, 100);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0xfc);
        e.call_log = Some(vec![]);
        e.call(0x00a7_2080, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xc4), 0x3f00_0000);
        assert_eq!(e.mem.u32(reader + 4), 104);
        let log: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|c| c.0)
            .collect();
        assert_eq!(
            log,
            vec![
                0x00a7_2080,
                NI_DYNAMIC_EFFECT_LOAD_BINARY,
                READ,
                NI_COLOR_LOAD_BINARY,
                NI_COLOR_LOAD_BINARY,
                NI_COLOR_LOAD_BINARY
            ]
        );
    }
}
