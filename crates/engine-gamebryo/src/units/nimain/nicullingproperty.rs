//! `NiMain/nicullingproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCullingProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_CULLING_PROPERTY_RTTI: u32 = 0x011f_5d20;

// Translated from 00a8f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_culling_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CULLING_PROPERTY_RTTI)
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5d28`: the pointer `00a8f5b0` stores the object it builds in.
const CULLING_PROPERTY_SLOT: u32 = 0x011f_5d28;

// Translated from 00a8f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nicullingproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): builds a 0x1c-byte object (allocation `00aa13e0`, base
/// constructor `00a5c200`, bit 0 of the word at `+0x18` cleared, vtable `0109f68c`) and
/// stores it, counted, in `011f5d28`, releasing the object held there before. A failed
/// allocation stores null.
pub fn fn_00a8f5b0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        let flags = e.mem.u16(block + 0x18);
        e.mem.set_u16(block + 0x18, flags & 0xfffe);
        e.mem.set_u32(block, 0x0109_f68c);
        block
    };
    assign_ref_counted(e, CULLING_PROPERTY_SLOT, object);
}

/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a8f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): releases the object that `00a8f5b0` stores in `011f5d28`. When it is not null, drops one reference
/// (`InterlockedDecrement` of the count at +4; at zero it
/// calls the object's vtable slot +4) and clears the pointer.
pub fn fn_00a8f620(e: &mut Engine) {
    let object = e.mem.u32(CULLING_PROPERTY_SLOT);
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(CULLING_PROPERTY_SLOT, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8f520, ni_culling_property_get_rtti() -> Ptr),
        entry!(0x00a8_f5b0, fn_00a8f5b0()),
        entry!(0x00a8_f620, fn_00a8f620()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_culling_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f520, &args![]).u32(), NI_CULLING_PROPERTY_RTTI);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn clears_bit_zero_of_the_word_the_base_constructor_left() {
        let run = run_default_instance(0x00a8_f5b0, CULLING_PROPERTY_SLOT, |e| {
            e.register(OBJECT_BASE_CONSTRUCTOR, |e, a| {
                e.mem.set_u16(a[0] + 0x18, 0x0003);
                Ret::default()
            });
        });
        let e = &run.engine;
        assert_eq!(e.mem.u16(run.object + 0x18), 0x0002);
        assert_eq!(e.mem.u32(run.object), 0x0109_f68c);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a8_f5b0, CULLING_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a8f620_releases_and_clears_the_held_object() {
        let mut e = Engine::new();
        e.map(CULLING_PROPERTY_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        let remaining = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let counter = remaining.clone();
        e.register_double(INTERLOCKED_DECREMENT, move |_, _| {
            counter.set(counter.get() - 1);
            counter.get().into_ret()
        });
        // Still referenced elsewhere: no destructor.
        remaining.set(2);
        e.mem.set_u32(CULLING_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![object + 4])));
        assert!(log.iter().all(|call| call.0 != 0x0150_1000));
        assert_eq!(e.mem.u32(CULLING_PROPERTY_SLOT), 0);
        // Last reference: vtable slot +4 runs on the object.
        e.mem.set_u32(CULLING_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0150_1000, vec![object])));
        assert_eq!(e.mem.u32(CULLING_PROPERTY_SLOT), 0);
        // Null: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
