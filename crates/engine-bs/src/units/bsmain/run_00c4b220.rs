//! `BSMain/run_00c4b220` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObjectNET::GetController` (`00a5c570`, `thiscall (object, rtti)`)
/// and the `NiRTTI` record it is given (`011f36ac`).
const GET_CONTROLLER: u32 = 0x00a5_c570;
const CONTROLLER_RTTI: u32 = 0x011f_36ac;
/// Makes a pooled `NiFixedString` handle from a C string (`00a5b690`,
/// `cdecl (text)`).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// `InterlockedDecrement` (import slot, one word: the address).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;

/// Releases the name handle: `InterlockedDecrement` of its count unless it
/// is the empty string.
fn release_name(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

// Translated from 00c4b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4b310 - its C++ exception frame (FS:[0], the name handle's unwind) is not translated
/// Finds something by name under `object` (the engine map has no name;
/// `cdecl (object, name, also_object)`): null for a null object or name.
/// The name becomes a pooled string handle (`00a5b690`) held in a stack
/// local. With a controller of the class at `011f36ac` (`GetController`),
/// the object at the controller's +0x78 is asked first (vtable +0x8C with
/// the handle's address); a non-null answer is returned. Otherwise, when
/// `also_object` is set, `object` itself is asked (vtable +0x9C with the
/// handle's address) and its answer returned; else null. The handle is
/// released before each return.
pub fn fn_00c4b310(e: &mut Engine, object: Ptr, name: Ptr, also_object: bool) -> u32 {
    if object.is_null() || name.is_null() {
        return 0;
    }
    let controller = e
        .call(GET_CONTROLLER, &args![object, CONTROLLER_RTTI])
        .u32();
    e.with_stack(4, |e, handle_slot| {
        // `name` is non-null here; the exe still tests it again.
        let handle = if name.is_null() {
            e.global::<u32>(FIXED_STRING_EMPTY)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
        };
        e.mem.set_u32(handle_slot.addr(), handle);
        if controller != 0 {
            let owner = e.mem.u32(controller.wrapping_add(0x78));
            let found = e.vcall(owner, 0x8c, &args![handle_slot]).u32();
            if found != 0 {
                let handle = e.mem.u32(handle_slot.addr());
                release_name(e, handle);
                return found;
            }
        }
        let found = if also_object {
            e.vcall(object.addr(), 0x9c, &args![handle_slot]).u32()
        } else {
            0
        };
        let handle = e.mem.u32(handle_slot.addr());
        release_name(e, handle);
        found
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4_b310, fn_00c4b310(Ptr, Ptr, bool) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn by_name_engine(controller: u32, owner_answer: u32) -> (Engine, u32, u32) {
        const OWNER_FIND: u32 = 0x0300_0000;
        const OBJECT_FIND: u32 = 0x0300_0010;
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 2);
        e.register_double(FIXED_STRING_FROM_TEXT, move |_, _| handle.into_ret());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register_double(GET_CONTROLLER, move |_, _| controller.into_ret());
        e.register_double(OWNER_FIND, move |_, _| owner_answer.into_ret());
        e.register(OBJECT_FIND, |_, _| 0x6666u32.into_ret());
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(vtable + 0x8c, OWNER_FIND);
        e.mem.set_u32(vtable + 0x9c, OBJECT_FIND);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        if controller != 0 {
            e.map(controller, 0x80);
            e.mem.set_u32(controller + 0x78, object);
        }
        e.call_log = Some(vec![]);
        (e, object, handle)
    }

    #[test]
    fn find_by_name_asks_the_controller_owner_then_the_object() {
        let name = 0x0700_0000u32;
        // The controller's owner answers.
        let (mut e, object, handle) = by_name_engine(0x0400_0000, 0x5555);
        e.map(name, 4);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x5555
        );
        assert_eq!(e.mem.u32(handle - 8), 1);
        let log = e.call_log.take().unwrap();
        let get: Vec<_> = log.iter().filter(|call| call.0 == GET_CONTROLLER).collect();
        assert_eq!(get[0].1, vec![object, CONTROLLER_RTTI]);
        // No answer from the owner: the object is asked when allowed.
        let (mut e, object, handle) = by_name_engine(0x0400_0000, 0);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x6666
        );
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.call(0x00c4_b310, &args![object, name, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(handle - 8), 0);
        // No controller.
        let (mut e, object, _) = by_name_engine(0, 0);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x6666
        );
        // A null object or name answers null at once.
        assert_eq!(e.call(0x00c4_b310, &args![0u32, name, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00c4_b310, &args![object, 0u32, 1u32]).u32(), 0);
    }
}
