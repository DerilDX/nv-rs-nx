//! `Havok SDK/hkpconstraintinstance.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cf4160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_40b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cf4160(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_40b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::addReference` / `removeReference` (`00c90510`, `00c905b0`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00cf3040 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two entity pointers at `this + 0x14` and `this + 0x18`: when it equals the
/// word at `+0x24` of `world` (the world's fixed body; the only caller is
/// `hkpWorld::removeConstraint`, `00c97ef9`) the reference is dropped (`00c905b0`) and the
/// slot cleared. The class is unconfirmed.
pub fn fn_00cf3040(e: &mut Engine, this: Ptr, world: Ptr) {
    for slot in 0..2 {
        let at = this.addr() + 0x14 + 4 * slot;
        let entity = e.mem.u32(at);
        if entity == e.mem.u32(world.addr() + 0x24) {
            e.call(REMOVE_REFERENCE, &args![entity]);
            e.mem.set_u32(at, 0);
        }
    }
}

// Translated from 00cf3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two entity pointers at `this + 0x14` and `this + 0x18` that is null: the
/// other one (`[+0x18] ^ [+0x14]`, which is the other entity while this slot is null) is
/// taken, and when it exists and has a world (its word +8) that world's word at +0x24 (the
/// fixed body) becomes this slot and gets a reference (`00c90510`). The only caller is
/// `hkpWorld::addConstraint` (`00c97db3`). The class is unconfirmed.
pub fn fn_00cf3e60(e: &mut Engine, this: Ptr) {
    for slot in 0..2 {
        let at = this.addr() + 0x14 + 4 * slot;
        if e.mem.u32(at) == 0 {
            let other = e.mem.u32(this.addr() + 0x18) ^ e.mem.u32(this.addr() + 0x14);
            if other != 0 {
                let world = e.mem.u32(other + 8);
                if world != 0 {
                    let fixed = e.mem.u32(world + 0x24);
                    e.mem.set_u32(at, fixed);
                    e.call(ADD_REFERENCE, &args![fixed]);
                }
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_4160, deleting_destructor_00cf4160(Ptr, u8) -> Ptr),
        entry!(0x00cf_3040, fn_00cf3040(Ptr, Ptr)),
        entry!(0x00cf_3e60, fn_00cf3e60(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cf4160_frees_only_when_bit_zero_is_set() {
        check(0x00cf_4160, 0x00cf_40b0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn rig() -> (Engine, Rc<RefCell<Vec<(u32, u32)>>>) {
        let mut e = test_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        (e, log)
    }

    #[test]
    fn detaching_clears_the_slots_that_hold_the_fixed_body() {
        let (mut e, log) = rig();
        let world = e.mem.alloc(0x40);
        let fixed = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x24, fixed);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, fixed);
        e.mem.set_u32(this + 0x18, other);
        e.call(0x00cf_3040, &args![this, world]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), other);
        assert_eq!(*log.borrow(), vec![(REMOVE_REFERENCE, fixed)]);
        // Both slots can hold it.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x14, fixed);
        e.mem.set_u32(this + 0x18, fixed);
        e.call(0x00cf_3040, &args![this, world]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn attaching_fills_a_missing_entity_with_the_fixed_body_of_the_other_world() {
        let (mut e, log) = rig();
        let world = e.mem.alloc(0x40);
        let fixed = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x24, fixed);
        let entity = e.mem.alloc(0x10);
        e.mem.set_u32(entity + 8, world);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, entity);
        e.call(0x00cf_3e60, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), fixed);
        assert_eq!(e.mem.u32(this + 0x18), entity);
        assert_eq!(*log.borrow(), vec![(ADD_REFERENCE, fixed)]);
        // Both entities missing, or the other one without a world: nothing happens.
        log.borrow_mut().clear();
        let empty = e.mem.alloc(0x40);
        e.call(0x00cf_3e60, &args![empty]);
        let no_world = e.mem.alloc(0x40);
        let orphan = e.mem.alloc(0x10);
        e.mem.set_u32(no_world + 0x14, orphan);
        e.call(0x00cf_3e60, &args![no_world]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(no_world + 0x18), 0);
    }
}
