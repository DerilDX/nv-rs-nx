//! `Havok SDK/run_00c90740` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue`, `TlsSetValue`, `EnterCriticalSection` and `LeaveCriticalSection` import
/// slots.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The Havok reference-counting lock: a structure (`012681d4` holds its address) whose word
/// +8 says whether locking is on, word +0xC counts recursive holds and +0x18 is the critical
/// section; `012681d8` holds the TLS index that marks the owning thread.
const LOCK_STRUCTURE: u32 = 0x0126_81d4;
const LOCK_TLS_INDEX: u32 = 0x0126_81d8;
/// The value stored in the TLS slot of the thread that holds the lock.
const LOCK_OWNER_MARK: u32 = 0x23df_4554;
/// `hkReferencedObject::removeReferenceLockUnchecked` (Xbox PDB, `00c903b0`, `this`).
const REMOVE_REFERENCE_UNCHECKED: u32 = 0x00c9_03b0;

/// Releases a reference on each of `count` objects whose pointers are `stride` bytes apart.
fn remove_references(e: &mut Engine, objects: u32, count: i32, stride: u32) {
    let mut cursor = objects;
    let mut remaining = count;
    while remaining > 0 {
        let object = e.mem.u32(cursor);
        e.call(REMOVE_REFERENCE_UNCHECKED, &args![object]);
        cursor = cursor.wrapping_add(stride);
        remaining -= 1;
    }
}

// Translated from 00c90740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a reference from each of `count` objects (pointers `stride` bytes apart from
/// `objects`) with `00c903b0`. When the reference lock is on (word +8 of the structure at
/// `012681d4`) and the calling thread is not marked as the owner (`TlsGetValue` of the
/// index at `012681d8` is not `0x23df4554`), the loop runs holding the lock: the recursive
/// count (+0xC) rises, or when the thread is still unmarked at the second look the critical
/// section (+0x18) is entered, the count set to 1 and the mark stored; afterwards the count
/// drops and, when it reaches zero, the mark is cleared and the section left. Otherwise
/// the loop runs unlocked. The class is unconfirmed.
pub fn fn_00c90740(e: &mut Engine, objects: Ptr, count: i32, stride: u32) {
    let lock = e.global::<u32>(LOCK_STRUCTURE);
    if e.mem.u32(lock + 8) != 0 {
        let index: u32 = e.global(LOCK_TLS_INDEX);
        if e.call(TLS_GET_VALUE, &args![index]).u32() != LOCK_OWNER_MARK {
            let index: u32 = e.global(LOCK_TLS_INDEX);
            if e.call(TLS_GET_VALUE, &args![index]).u32() == LOCK_OWNER_MARK {
                let holds = e.mem.u32(lock + 0xc);
                e.mem.set_u32(lock + 0xc, holds.wrapping_add(1));
            } else {
                e.call(ENTER_CRITICAL_SECTION, &args![lock + 0x18]);
                e.mem.set_u32(lock + 0xc, 1);
                let index: u32 = e.global(LOCK_TLS_INDEX);
                e.call(TLS_SET_VALUE, &args![index, LOCK_OWNER_MARK]);
            }
            remove_references(e, objects.addr(), count, stride);
            let holds = e.mem.u32(lock + 0xc).wrapping_sub(1);
            e.mem.set_u32(lock + 0xc, holds);
            if (holds as i32) <= 0 {
                let index: u32 = e.global(LOCK_TLS_INDEX);
                e.call(TLS_SET_VALUE, &args![index, 0u32]);
                e.call(LEAVE_CRITICAL_SECTION, &args![lock + 0x18]);
            }
            return;
        }
    }
    remove_references(e, objects.addr(), count, stride);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c9_0740, fn_00c90740(Ptr, i32, u32))]
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

    type Log = Rc<RefCell<Vec<(&'static str, u32, u32)>>>;

    /// The lock structure, three objects 8 bytes apart, and doubles for the imports; the
    /// thread's TLS slot starts at `tls`.
    fn rig(lock_on: bool, tls: u32) -> (Engine, Log, u32, u32) {
        let mut e = test_engine();
        let lock = e.mem.alloc(0x40);
        e.map(LOCK_STRUCTURE, 8);
        e.set_global(LOCK_STRUCTURE, lock);
        e.set_global(LOCK_TLS_INDEX, 5u32);
        e.mem.set_u32(lock + 8, u32::from(lock_on));
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let slot = Rc::new(RefCell::new(tls));
        let reads = Rc::clone(&slot);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: *reads.borrow(),
                ..Ret::default()
            }
        });
        let writes = Rc::clone(&slot);
        let sink = Rc::clone(&log);
        e.register_double(TLS_SET_VALUE, move |_, a| {
            sink.borrow_mut().push(("set", a[0], a[1]));
            *writes.borrow_mut() = a[1];
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ENTER_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("enter", a[0], 0));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(LEAVE_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("leave", a[0], 0));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE_UNCHECKED, move |_, a| {
            sink.borrow_mut().push(("release", a[0], 0));
            Ret::default()
        });
        let objects = e.mem.alloc(0x20);
        for i in 0..3 {
            e.mem.set_u32(objects + 8 * i, 0x100 + i);
        }
        (e, log, lock, objects)
    }

    fn releases(log: &Log) -> Vec<u32> {
        log.borrow()
            .iter()
            .filter(|(what, _, _)| *what == "release")
            .map(|(_, object, _)| *object)
            .collect()
    }

    #[test]
    fn without_the_lock_the_objects_are_released_directly() {
        let (mut e, log, _, objects) = rig(false, 0);
        e.call(0x00c9_0740, &args![objects, 3u32, 8u32]);
        assert_eq!(releases(&log), vec![0x100, 0x101, 0x102]);
        assert_eq!(log.borrow().len(), 3);
        // A count of zero (or less) releases nothing.
        e.call(0x00c9_0740, &args![objects, 0u32, 8u32]);
        e.call(0x00c9_0740, &args![objects, -2i32, 8u32]);
        assert_eq!(log.borrow().len(), 3);
    }

    #[test]
    fn an_unmarked_thread_takes_the_section_and_marks_itself() {
        let (mut e, log, lock, objects) = rig(true, 0);
        e.call(0x00c9_0740, &args![objects, 2u32, 8u32]);
        let seen: Vec<(&str, u32, u32)> = log.borrow().clone();
        assert_eq!(
            seen,
            vec![
                ("enter", lock + 0x18, 0),
                ("set", 5, LOCK_OWNER_MARK),
                ("release", 0x100, 0),
                ("release", 0x101, 0),
                ("set", 5, 0),
                ("leave", lock + 0x18, 0),
            ]
        );
        assert_eq!(e.mem.u32(lock + 0xc), 0);
    }

    #[test]
    fn the_marked_owner_releases_without_locking() {
        let (mut e, log, lock, objects) = rig(true, LOCK_OWNER_MARK);
        e.call(0x00c9_0740, &args![objects, 2u32, 8u32]);
        assert_eq!(releases(&log), vec![0x100, 0x101]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(lock + 0xc), 0);
    }
}
