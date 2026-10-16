//! `BSMain/bsstream.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two functions are the `NiTLargeArray<NiPointer<NiObject>>` instance the
//! stream's object array uses (`NiStream::m_kObjects`): elements are counted
//! references to objects whose reference count is at +4 and whose virtual
//! slot at +4 deletes them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00c1d790`: `cdecl(count)`, allocates the storage of `count` pointers (the
/// count in front of the elements, which are null).
const ARRAY_ALLOCATE: u32 = 0x00c1_d790;
/// `00a47600`: `cdecl(storage)`, releases every reference in the storage
/// `ARRAY_ALLOCATE` made and frees it.
const ARRAY_FREE_STORAGE: u32 = 0x00a4_7600;
/// The virtual slot of an object that deletes it when its count reaches zero.
const SLOT_DELETE_THIS: u32 = 0x04;

/// `InterlockedDecrement` of the count at +4; the delete slot runs at zero.
/// (The code tests for null first where the element can be null.)
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// `InterlockedIncrement` of the count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

// Translated from 00a64fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetSize`
/// (engine map; `thiscall`, one stack word): gives the array room for `new_max`
/// elements. Does nothing when that is the current capacity. When it is below
/// the used size, the elements from `new_max` on are released and cleared (the
/// element count `m_uiESize` goes down for each non-null one) and the used size
/// becomes `new_max`. Then a new storage block (`00c1d790`) takes the elements,
/// each one a new counted reference, the old storage is released and freed
/// (`00a47600`) and the capacity is `new_max` (the pointer is null for 0).
pub fn ni_t_large_array_set_size(e: &mut Engine, this: Ptr<NiTLargeArray>, new_max: u32) {
    if new_max == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    if new_max < e.get(this, NiTLargeArray::m_uiSize) {
        let mut index = new_max;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTLargeArray::m_uiSize, new_max);
    }
    let old = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_max);
    if new_max == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let storage = e.call(ARRAY_ALLOCATE, &args![new_max]).u32();
        e.set(this, NiTLargeArray::m_pBase, storage);
        let mut index = 0u32;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let current = e.mem.u32(slot);
            let carried = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            if current != carried {
                if current != 0 {
                    release(e, current);
                }
                e.mem.set_u32(slot, carried);
                if carried != 0 {
                    add_ref(e, carried);
                }
            }
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        while index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_add(1);
        }
    }
    e.call(ARRAY_FREE_STORAGE, &args![old]);
}

// Translated from 00cc1a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetAt`
/// (engine map; `thiscall`, two stack words): stores the reference that
/// `value` points at into element `index`. Keeps the used size (`index + 1`
/// when the index is at or beyond it) and the non-null count (`m_uiESize`) right,
/// and, when the element changes, releases the old object and counts a new
/// reference to the new one. The capacity is not checked.
pub fn ni_t_large_array_set_at(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: Ptr) {
    let present = e.get(this, NiTLargeArray::m_uiESize);
    let wanted = e.mem.u32(value.addr());
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    if index < e.get(this, NiTLargeArray::m_uiSize) {
        let held = e.mem.u32(slot);
        if wanted == 0 {
            if held != 0 {
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
        } else if held == 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    } else {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if wanted != 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    let held = e.mem.u32(slot);
    if held != e.mem.u32(value.addr()) {
        if held != 0 {
            release(e, held);
        }
        let new = e.mem.u32(value.addr());
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

// Translated from 00c3b9e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043dc without reading the receiver. The map names this body `BSReference::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3b9e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43dc
}

// Translated from 00c3ba70 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043e4 without reading the receiver. The map names this body `BSNodeReferences::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3ba70(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43e4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_b9e0, fn_00c3b9e0(Ptr) -> u32),
        entry!(0x00c3_ba70, fn_00c3ba70(Ptr) -> u32),
        entry!(
            0x00a6_4fe0,
            ni_t_large_array_set_size(Ptr<NiTLargeArray>, u32)
        ),
        entry!(
            0x00cc_1a10,
            ni_t_large_array_set_at(Ptr<NiTLargeArray>, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETE_FUNCTION: u32 = 0x0040_0010;
    type Deleted = Rc<RefCell<Vec<u32>>>;

    struct Fixture {
        e: Engine,
        array: Ptr<NiTLargeArray>,
        vtable: u32,
        deleted: Deleted,
        freed: Rc<RefCell<Vec<u32>>>,
    }

    impl Fixture {
        fn new() -> Fixture {
            let mut e = Engine::new();
            e.register(INTERLOCKED_INCREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_add(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(INTERLOCKED_DECREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_sub(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0] * 4 + 4).into_ret());
            let deleted: Deleted = Rc::default();
            let log = deleted.clone();
            e.register_double(DELETE_FUNCTION, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let freed: Rc<RefCell<Vec<u32>>> = Rc::default();
            let log = freed.clone();
            e.register_double(ARRAY_FREE_STORAGE, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, DELETE_FUNCTION);
            let array = e.new_object::<NiTLargeArray>();
            Fixture {
                e,
                array,
                vtable,
                deleted,
                freed,
            }
        }

        fn object(&mut self, references: u32) -> u32 {
            let object = self.e.mem.alloc(8);
            self.e.mem.set_u32(object, self.vtable);
            self.e.mem.set_u32(object + 4, references);
            object
        }

        /// Gives the array storage for `max` elements holding `items`.
        fn fill(&mut self, max: u32, items: &[u32]) -> u32 {
            let base = self.e.mem.alloc(max * 4 + 4);
            for (i, item) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *item);
            }
            let array = self.array;
            self.e.set(array, NiTLargeArray::m_pBase, base);
            self.e.set(array, NiTLargeArray::m_uiMaxSize, max);
            self.e
                .set(array, NiTLargeArray::m_uiSize, items.len() as u32);
            let present = items.iter().filter(|item| **item != 0).count() as u32;
            self.e.set(array, NiTLargeArray::m_uiESize, present);
            base
        }

        fn state(&self) -> (u32, u32, u32) {
            (
                self.e.get(self.array, NiTLargeArray::m_uiMaxSize),
                self.e.get(self.array, NiTLargeArray::m_uiSize),
                self.e.get(self.array, NiTLargeArray::m_uiESize),
            )
        }

        fn element(&self, index: u32) -> u32 {
            let base = self.e.get(self.array, NiTLargeArray::m_pBase);
            self.e.mem.u32(base + index * 4)
        }

        fn references(&self, object: u32) -> u32 {
            self.e.mem.u32(object + 4)
        }
    }

    #[test]
    fn set_size_to_the_current_capacity_does_nothing() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let base = fx.fill(3, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 3u32]);
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), base);
        assert!(fx.freed.borrow().is_empty());
        assert_eq!(fx.references(a), 1);
    }

    #[test]
    fn set_size_grows_into_new_storage_and_counts_a_reference_per_element() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        let old = fx.fill(4, &[a, 0, b]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 8u32]);
        assert_eq!(fx.state(), (8, 3, 2));
        assert_eq!(*fx.freed.borrow(), vec![old]);
        assert_eq!([fx.element(0), fx.element(1), fx.element(2)], [a, 0, b]);
        assert_eq!(fx.element(5), 0);
        assert_eq!((fx.references(a), fx.references(b)), (2, 2));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_size_below_the_used_size_releases_the_cut_elements() {
        let mut fx = Fixture::new();
        let (a, b, c) = (fx.object(1), fx.object(2), fx.object(1));
        let old = fx.fill(3, &[a, b, c]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 1u32]);
        assert_eq!(fx.state(), (1, 1, 1));
        // b only loses the array's reference, c is deleted.
        assert_eq!((fx.references(b), fx.references(c)), (1, 0));
        assert_eq!(*fx.deleted.borrow(), vec![c]);
        // The cut slots of the old storage were cleared before the copy.
        assert_eq!(fx.e.mem.u32(old + 4), 0);
        assert_eq!(fx.e.mem.u32(old + 8), 0);
        assert_eq!(fx.element(0), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    #[test]
    fn set_size_to_zero_clears_the_pointer() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let old = fx.fill(2, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 0u32]);
        assert_eq!(fx.state(), (0, 0, 0));
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), 0);
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    fn holder(fx: &mut Fixture, object: u32) -> u32 {
        let holder = fx.e.mem.alloc(4);
        fx.e.mem.set_u32(holder, object);
        holder
    }

    #[test]
    fn set_at_stores_a_new_reference_in_an_empty_slot() {
        let mut fx = Fixture::new();
        fx.fill(4, &[0, 0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 1u32, value]);
        assert_eq!(fx.element(1), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(fx.state(), (4, 2, 1));
    }

    #[test]
    fn set_at_beyond_the_used_size_extends_it() {
        let mut fx = Fixture::new();
        fx.fill(8, &[0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 5u32, value]);
        assert_eq!(fx.element(5), a);
        assert_eq!(fx.state(), (8, 6, 1));
        let nothing = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 7u32, nothing]);
        assert_eq!(fx.state(), (8, 8, 1));
    }

    #[test]
    fn set_at_replaces_and_releases_the_old_object() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        fx.fill(4, &[a]);
        let value = holder(&mut fx, b);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), b);
        assert_eq!((fx.references(a), fx.references(b)), (0, 2));
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(fx.state(), (4, 1, 1));
    }

    #[test]
    fn set_at_with_null_clears_the_slot_and_the_count() {
        let mut fx = Fixture::new();
        let a = fx.object(2);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), 0);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 0));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_at_with_the_same_object_changes_no_count() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 1));
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c3b9e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_b9e0, &args![0x1000u32]).u32(), 0x0120_43dc);
        assert_eq!(e.call(0x00c3_b9e0, &args![0u32]).u32(), 0x0120_43dc);
    }

    #[test]
    fn fn_00c3ba70_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_ba70, &args![0x1000u32]).u32(), 0x0120_43e4);
        assert_eq!(e.call(0x00c3_ba70, &args![0u32]).u32(), 0x0120_43e4);
    }
}
