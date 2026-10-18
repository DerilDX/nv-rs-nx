//! `NiAnimation/run_00a47540` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::platform::MEMORY_MANAGER;

/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the singleton, one
/// stack word: the block).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The virtual slot of an object that deletes it when its count reaches zero.
const SLOT_DELETE_THIS: u32 = 0x04;

// Translated from 00a47600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: frees the storage of an array of counted object
/// references (`cdecl`, one stack word: the first element, with the element count in
/// the word in front of it; nothing for null; the storage `00c1d790` makes).
/// Walks the elements from the last to the first and, for each non-null one,
/// `InterlockedDecrement`s the count at +4 and runs the virtual slot at +4
/// (delete) when it reaches zero. Then the block, count word included, is given
/// back to the memory manager (`00aa4060`).
pub fn fn_00a47600(e: &mut Engine, elements: Ptr) {
    if elements.is_null() {
        return;
    }
    let count = e.mem.u32(elements.addr().wrapping_sub(4));
    let mut cursor = elements.addr().wrapping_add(count.wrapping_mul(4));
    let mut remaining = count.wrapping_sub(1) as i32;
    while remaining >= 0 {
        let object = e.mem.u32(cursor.wrapping_sub(4));
        cursor = cursor.wrapping_sub(4);
        if object != 0 {
            let left = e
                .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(object, SLOT_DELETE_THIS, &args![]);
            }
        }
        remaining = remaining.wrapping_sub(1);
    }
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER, elements.addr().wrapping_sub(4)],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_7600, fn_00a47600(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETE_FUNCTION: u32 = 0x0040_0010;

    #[test]
    fn null_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00a4_7600, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn elements_are_released_last_to_first_and_the_block_is_freed() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let deleted: Rc<RefCell<Vec<u32>>> = Rc::default();
        let log = deleted.clone();
        e.register_double(DELETE_FUNCTION, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_FUNCTION);
        let mut object = |references: u32| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 4, references);
            object
        };
        let (first, last) = (object(1), object(2));
        let block = e.mem.alloc(16);
        e.mem.set_u32(block, 3);
        // elements: an object that dies, a null, an object that survives
        e.mem.set_u32(block + 4, first);
        e.mem.set_u32(block + 12, last);
        e.call_log = Some(vec![]);
        e.call(0x00a4_7600, &args![block + 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(*deleted.borrow(), vec![first]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(last + 4), 1);
        let decrements: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INTERLOCKED_DECREMENT)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(decrements, vec![args![last + 4], args![first + 4]]);
        let frees: Vec<_> = log
            .iter()
            .filter(|call| call.0 == MEMORY_MANAGER_DEALLOCATE)
            .collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![MEMORY_MANAGER, block]);
    }
}
