//! `NiParticle/nipsysmodifierfloatctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c32f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c315b0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `012027f8`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_modifier_float_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_15b0;
    const CLASS_NAME_POINTER: u32 = 0x0120_27f8;
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

// Translated from 00c32e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27f8.
pub fn ni_p_sys_modifier_float_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27f8
}

/// `NiPSysModifierCtlr::LinkObject` (`00a57630`, `thiscall`, one word).
const MODIFIER_CTLR_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall`, no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a3c340` (unnamed, `thiscall`, one word: the linked object): the
/// constructor of the 0x18-byte object made from it.
const LINKED_OBJECT_CONSTRUCT: u32 = 0x00a3_c340;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The stream version below which the old layout's extra link is read
/// (`NiStream` `+0xD8`, compared unsigned).
const OLD_FORMAT_VERSION: u32 = 0x0a01_0068;

// Translated from 00c32e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::LinkObject` (Xbox PDB): the modifier-controller
/// part (`00a57630`); for a stream version (`+0xD8`) below `0a010068` the
/// next linked object is wrapped in a new 0x18-byte object (`00a3c340`, null
/// when the allocation fails) that goes into the `NiPointer` at `+0x34`.
pub fn ni_p_sys_modifier_float_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_CTLR_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) >= OLD_FORMAT_VERSION {
        return;
    }
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    let wrapped = if block == 0 {
        0
    } else {
        e.call(LINKED_OBJECT_CONSTRUCT, &args![block, object]).u32()
    };
    let slot = this.addr() + 0x34;
    let old = e.mem.u32(slot);
    if old != wrapped {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 4, &[]);
        }
        e.mem.set_u32(slot, wrapped);
        if wrapped != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![wrapped + 4]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_2e10,
            ni_p_sys_modifier_float_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(0x00c32e00, ni_p_sys_modifier_float_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c32f60,
            ni_p_sys_modifier_float_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_modifier_float_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array()
    {
        const PARENT: u32 = 0x00c3_15b0;
        const NAME_POINTER: u32 = 0x0120_27f8;
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
            ni_p_sys_modifier_float_ctlr_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
    fn ni_p_sys_modifier_float_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_2e00, &args![]).u32(), 0x0120_27f8);
    }
}

#[cfg(test)]
mod link_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn link_object_wraps_the_old_formats_link() {
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| word(0x4444));
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LINKED_OBJECT_CONSTRUCT, |_, a| word(a[0]));
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, vtable);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, old);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, OLD_FORMAT_VERSION);
        e.call_log = Some(vec![]);
        // The current format: only the controller part.
        e.call(0x00c3_2e10, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x34), old);
        e.mem.set_u32(stream + 0xd8, OLD_FORMAT_VERSION - 1);
        e.call(0x00c3_2e10, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(DELETE_THIS, vec![old])));
        let made = e.mem.u32(this + 0x34);
        assert!(log.contains(&(LINKED_OBJECT_CONSTRUCT, vec![made, 0x4444])));
        assert_eq!(e.mem.u32(made + 4), 1);
    }
}
