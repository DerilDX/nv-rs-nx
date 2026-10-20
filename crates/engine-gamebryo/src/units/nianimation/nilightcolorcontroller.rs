//! `NiAnimation/nilightcolorcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLightColorController::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_COLOR_CONTROLLER_RTTI: u32 = 0x011f_3fa8;

// Translated from 00a4d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_color_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_COLOR_CONTROLLER_RTTI)
}

// Translated from 00a4d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a57d10`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3fa8`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_light_color_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_7d10;
    const CLASS_NAME_POINTER: u32 = 0x011f_3fa8;
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

/// The base constructor `00a57c80` runs first (`00a38e00`, named
/// `NiBoolInterpController::CopyMembers` in the engine map; thiscall).
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a3_8e00;
/// `NiPoint3InterpController`'s vtable (RTTI
/// `.?AVNiPoint3InterpController@@`).
const NI_POINT3_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_ad74;

// Translated from 00a57c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiLightColorController::CopyMembers` in the engine map, but the
/// body is the `NiPoint3InterpController` constructor: the base constructor
/// (`00a38e00`), then that class's vtable. Returns `this`.
pub fn fn_00a57c80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(this.addr(), NI_POINT3_INTERP_CONTROLLER_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4d200, ni_light_color_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4d5a0,
            ni_light_color_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a5_7c80, fn_00a57c80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_light_color_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4d200, &args![]).u32(),
            NI_LIGHT_COLOR_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_light_color_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_7d10;
        const NAME_POINTER: u32 = 0x011f_3fa8;
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
            ni_light_color_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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

    #[test]
    fn fn_00a57c80_runs_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00a5_7c80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_POINT3_INTERP_CONTROLLER_VTABLE);
    }
}
