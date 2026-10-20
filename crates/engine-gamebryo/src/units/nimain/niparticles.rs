//! `NiMain/niparticles.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticles::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLES_RTTI: u32 = 0x011f_5f64;

// Translated from 00a9b1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticles::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particles_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLES_RTTI)
}

// Translated from 00a9b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticles::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a80e70`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5f64`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_particles_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a8_0e70;
    const CLASS_NAME_POINTER: u32 = 0x011f_5f64;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// `00a96910` (`thiscall`, `RET 4`; engine map `NiParticlesData::SortParticles`).
const SORT_PARTICLES: u32 = 0x00a9_6910;
/// `00a7fd90` (`thiscall`, one stack word; engine map `NiGeometry::OnVisible`).
const GEOMETRY_ON_VISIBLE: u32 = 0x00a7_fd90;
/// `00a80610` (`thiscall`, `RET 4`; engine map `NiGeometry::RenderImmediate`).
const GEOMETRY_RENDER_IMMEDIATE: u32 = 0x00a8_0610;

// Translated from 00a9b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticles::OnVisible` (Xbox PDB; `thiscall`, `RET 4`): when the object at +0x9c is set,
/// has bit 0 of its flags word at +0x18 set and any of bits 5 to 8 (`0x1e0`) of that word,
/// sorts the particles of the data at +0xb8 (`00a96910`) with the word at +0xc of `culler`;
/// then `NiGeometry::OnVisible` (`00a7fd90`) with `culler`.
pub fn ni_particles_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let base = this.addr();
    let property = e.mem.u32(base + 0x9c);
    if property != 0
        && e.mem.u8(property + 0x18) & 1 != 0
        && e.mem.u16(property + 0x18) & 0x1e0 != 0
    {
        let data = e.mem.u32(base + 0xb8);
        let camera = e.mem.u32(culler.addr() + 0xc);
        e.call(SORT_PARTICLES, &args![data, camera]);
    }
    e.call(GEOMETRY_ON_VISIBLE, &args![this, culler]);
}

// Translated from 00a9b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticles::RenderImmediate` (Xbox PDB; `thiscall`, `RET 4`): `NiGeometry::RenderImmediate`
/// (`00a80610`) with `renderer`; then, when the renderer's word at +0x200 is 1 or 2 and its
/// byte at +0x208 is 1, the renderer's vtable slot +0x1bc with `this`.
pub fn ni_particles_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(GEOMETRY_RENDER_IMMEDIATE, &args![this, renderer]);
    let at = renderer.addr();
    let mode = e.mem.u32(at + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(at + 0x208) == 1 {
        e.vcall(at, 0x1bc, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b1b0, ni_particles_get_rtti() -> Ptr),
        entry!(0x00a9b2e0, ni_particles_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a9_b2a0, ni_particles_on_visible(Ptr, Ptr)),
        entry!(0x00a9_b1f0, ni_particles_render_immediate(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_particles_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9b1b0, &args![]).u32(), NI_PARTICLES_RTTI);
    }

    #[test]
    fn ni_particles_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a8_0e70;
        const NAME_POINTER: u32 = 0x011f_5f64;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_particles_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn on_visible_sorts_only_with_the_flag_bits_set() {
        let mut e = Engine::new();
        e.register(SORT_PARTICLES, |_, _| Ret::default());
        e.register(GEOMETRY_ON_VISIBLE, |_, _| Ret::default());
        let this = e.mem.alloc(0xc0);
        let property = e.mem.alloc(0x20);
        let culler = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xb8, 0x6000);
        e.mem.set_u32(culler + 0xc, 0x5000);
        for (pointer, flags, sorts) in [
            (0, 0x1e1u16, false),
            (property, 0x1e0, false),
            (property, 0x001, false),
            (property, 0x021, true),
            (property, 0x101, true),
        ] {
            e.mem.set_u32(this + 0x9c, pointer);
            e.mem.set_u16(property + 0x18, flags);
            e.call_log = Some(vec![]);
            e.call(0x00a9_b2a0, &args![this, culler]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![];
            if sorts {
                expected.push((SORT_PARTICLES, vec![0x6000, 0x5000]));
            }
            expected.push((GEOMETRY_ON_VISIBLE, vec![this, culler]));
            assert_eq!(log[1..], expected[..], "flags {flags:#x}");
        }
    }

    #[test]
    fn render_immediate_calls_the_renderer_in_modes_1_and_2() {
        let mut e = Engine::new();
        e.register(GEOMETRY_RENDER_IMMEDIATE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x70]);
        e.mem.set_u32(0x0150_01bc, 0x0150_1000);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0xc0);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, 0x0150_0000);
        for (mode, byte, calls) in [
            (1u32, 1u8, true),
            (2, 1, true),
            (3, 1, false),
            (1, 2, false),
        ] {
            e.mem.set_u32(renderer + 0x200, mode);
            e.mem.set_u8(renderer + 0x208, byte);
            e.call_log = Some(vec![]);
            e.call(0x00a9_b1f0, &args![this, renderer]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(GEOMETRY_RENDER_IMMEDIATE, vec![this, renderer])];
            if calls {
                expected.push((0x0150_1000, vec![renderer, this]));
            }
            assert_eq!(log[1..], expected[..]);
        }
    }
}
