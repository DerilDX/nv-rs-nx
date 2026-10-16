//! `Havok SDK/hkreferencedobject.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkReferencedObject` keeps its size at `+4` (a `u16`; 0 means the object is not reference
//! counted) and its reference count at `+6` (a `u16`). The lock-taking variants guard the
//! count with a recursive lock: a record at the pointer held in `0x012681d4` with the mode word
//! at `+8` (1 = locking enabled), the recursion depth at `+0xC` and the critical section at
//! `+0x18`; the owning thread is marked in the TLS slot whose index is held in `0x012681d8`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Pointer to the lock record, and the TLS index of the owner mark.
const LOCK_RECORD: u32 = 0x0126_81d4;
const LOCK_TLS_INDEX: u32 = 0x0126_81d8;
/// The value stored in the TLS slot while this thread holds the lock.
const OWNER_MARK: u32 = 0x23df_4554;
/// Import slots: `TlsGetValue`, `TlsSetValue`, `EnterCriticalSection`, `LeaveCriticalSection`.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// Takes the recursive lock of the record at `lock`: bumps the depth when this thread already
/// owns it, otherwise enters the critical section, sets depth 1 and marks the thread.
fn lock_enter(e: &mut Engine, lock: u32) {
    let index = e.mem.u32(LOCK_TLS_INDEX);
    let owner = e.call(TLS_GET_VALUE, &args![index]).u32();
    if owner == OWNER_MARK {
        let depth = e.mem.u32(lock + 0xc);
        e.mem.set_u32(lock + 0xc, depth.wrapping_add(1));
        return;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![lock + 0x18]);
    e.mem.set_u32(lock + 0xc, 1);
    let index = e.mem.u32(LOCK_TLS_INDEX);
    e.call(TLS_SET_VALUE, &args![index, OWNER_MARK]);
}

/// Drops one level of the recursive lock; at depth 0 or below the owner mark is cleared and
/// the critical section left.
fn lock_leave(e: &mut Engine, lock: u32) {
    let depth = (e.mem.u32(lock + 0xc) as i32).wrapping_sub(1);
    e.mem.set_u32(lock + 0xc, depth as u32);
    if depth < 1 {
        let index = e.mem.u32(LOCK_TLS_INDEX);
        e.call(TLS_SET_VALUE, &args![index, 0u32]);
        e.call(LEAVE_CRITICAL_SECTION, &args![lock + 0x18]);
    }
}

/// Whether the locked path applies: the mode word at `+8` is 1 and this thread is not
/// already marked as the owner.
fn needs_lock(e: &mut Engine, lock: u32) -> bool {
    if e.mem.u32(lock + 8) != 1 {
        return false;
    }
    let index = e.mem.u32(LOCK_TLS_INDEX);
    e.call(TLS_GET_VALUE, &args![index]).u32() != OWNER_MARK
}

/// Adds `delta` to the `u16` reference count at `this + 6`.
fn adjust_count(e: &mut Engine, this: Ptr, delta: u16) {
    let count = e.mem.u16(this.addr() + 6);
    e.mem.set_u16(this.addr() + 6, count.wrapping_add(delta));
}

// Translated from 00c903a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::addReferenceLockUnchecked` (Xbox PDB): bumps the reference count
/// (`+6`) unless the object is not counted (size word `+4` is 0). No lock is taken.
pub fn add_reference_lock_unchecked(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) != 0 {
        adjust_count(e, this, 1);
    }
}

// Translated from 00c903b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::removeReferenceLockUnchecked` (Xbox PDB): drops the reference count
/// and, when it reaches 0, calls the deleting destructor (vtable slot 0) with flag 1. Not
/// counted objects (size word 0) are left alone. No lock is taken.
pub fn remove_reference_lock_unchecked(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    adjust_count(e, this, 0xffff);
    if e.mem.u16(this.addr() + 6) == 0 {
        e.vcall(this.addr(), 0, &args![1u32]);
    }
}

// Translated from 00c90490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the Havok reference-count lock (recursive; see the module comment). The function
/// has no `this`; its name is unconfirmed.
pub fn fn_00c90490(e: &mut Engine) {
    let lock = e.mem.u32(LOCK_RECORD);
    lock_enter(e, lock);
}

// Translated from 00c904e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases one level of the Havok reference-count lock taken by `00c90490`.
pub fn fn_00c904e0(e: &mut Engine) {
    let lock = e.mem.u32(LOCK_RECORD);
    lock_leave(e, lock);
}

// Translated from 00c90510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locked `hkReferencedObject::addReference`: for a counted object, bumps the count under
/// the recursive lock when locking is enabled and this thread does not own it, otherwise
/// bumps it directly.
pub fn fn_00c90510(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    let lock = e.mem.u32(LOCK_RECORD);
    if needs_lock(e, lock) {
        lock_enter(e, lock);
        adjust_count(e, this, 1);
        lock_leave(e, lock);
    } else {
        adjust_count(e, this, 1);
    }
}

// Translated from 00c905b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locked `hkReferencedObject::removeReference`: for a counted object whose count is 1,
/// clears the count and calls the deleting destructor (slot 0, flag 1) without locking;
/// otherwise decrements the count under the recursive lock when locking is enabled and this
/// thread does not own it, or directly.
pub fn fn_00c905b0(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    if e.mem.u16(this.addr() + 6) == 1 {
        e.mem.set_u16(this.addr() + 6, 0);
        e.vcall(this.addr(), 0, &args![1u32]);
        return;
    }
    let lock = e.mem.u32(LOCK_RECORD);
    if needs_lock(e, lock) {
        lock_enter(e, lock);
        adjust_count(e, this, 0xffff);
        lock_leave(e, lock);
    } else {
        adjust_count(e, this, 0xffff);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_03a0, add_reference_lock_unchecked(Ptr)),
        entry!(0x00c9_03b0, remove_reference_lock_unchecked(Ptr)),
        entry!(0x00c9_0490, fn_00c90490()),
        entry!(0x00c9_04e0, fn_00c904e0()),
        entry!(0x00c9_0510, fn_00c90510(Ptr)),
        entry!(0x00c9_05b0, fn_00c905b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const LOCK: u32 = 0x0900_0000;
    const DESTRUCTOR_DOUBLE: u32 = 0x0900_0100;

    type CallLog = Rc<RefCell<Vec<(u32, u32)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(LOCK, 0x100);
        e.mem.set_u32(LOCK_RECORD, LOCK);
        e.mem.set_u32(LOCK_TLS_INDEX, 3);
        e
    }

    /// Doubles for the four imports; the TLS slot is shared state, and every call is logged
    /// as (import address, last argument or the critical section).
    fn install_imports(e: &mut Engine) -> (Rc<RefCell<u32>>, CallLog) {
        let slot = Rc::new(RefCell::new(0u32));
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let (s, l) = (Rc::clone(&slot), Rc::clone(&log));
        e.register_double(TLS_GET_VALUE, move |_, a| {
            l.borrow_mut().push((TLS_GET_VALUE, a[0]));
            Ret {
                eax: *s.borrow(),
                ..Ret::default()
            }
        });
        let (s, l) = (Rc::clone(&slot), Rc::clone(&log));
        e.register_double(TLS_SET_VALUE, move |_, a| {
            l.borrow_mut().push((TLS_SET_VALUE, a[1]));
            *s.borrow_mut() = a[1];
            Ret::default()
        });
        for import in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION] {
            let l = Rc::clone(&log);
            e.register_double(import, move |_, a| {
                l.borrow_mut().push((import, a[0]));
                Ret::default()
            });
        }
        (slot, log)
    }

    /// An object with a vtable whose slot 0 is a logging double.
    fn object(e: &mut Engine, size: u16, count: u16) -> (u32, Rc<RefCell<Vec<u32>>>) {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, DESTRUCTOR_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u16(object + 4, size);
        e.mem.set_u16(object + 6, count);
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&log);
        e.register_double(DESTRUCTOR_DOUBLE, move |_, a| {
            l.borrow_mut().extend_from_slice(a);
            Ret::default()
        });
        (object, log)
    }

    fn entered(log: &CallLog) -> bool {
        log.borrow().iter().any(|c| c.0 == ENTER_CRITICAL_SECTION)
    }

    #[test]
    fn add_reference_lock_unchecked_counts_only_counted_objects() {
        let mut e = engine();
        let (counted, _) = object(&mut e, 0x20, 5);
        e.call(0x00c9_03a0, &args![counted]);
        assert_eq!(e.mem.u16(counted + 6), 6);
        let (plain, _) = object(&mut e, 0, 5);
        e.call(0x00c9_03a0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 5);
    }

    #[test]
    fn remove_reference_lock_unchecked_destroys_at_zero() {
        let mut e = engine();
        let (obj, log) = object(&mut e, 0x20, 2);
        e.call(0x00c9_03b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 1);
        assert!(log.borrow().is_empty());
        e.call(0x00c9_03b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 0);
        assert_eq!(*log.borrow(), vec![obj, 1]);
        let (plain, plain_log) = object(&mut e, 0, 1);
        e.call(0x00c9_03b0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 1);
        assert!(plain_log.borrow().is_empty());
    }

    #[test]
    fn lock_enter_takes_the_critical_section_once_and_recurses() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        e.call(0x00c9_0490, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 1);
        assert_eq!(*slot.borrow(), OWNER_MARK);
        assert!(log
            .borrow()
            .contains(&(ENTER_CRITICAL_SECTION, LOCK + 0x18)));
        log.borrow_mut().clear();
        e.call(0x00c9_0490, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 2);
        assert!(!entered(&log));
    }

    #[test]
    fn lock_leave_releases_at_depth_zero() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        e.mem.set_u32(LOCK + 0xc, 2);
        *slot.borrow_mut() = OWNER_MARK;
        e.call(0x00c9_04e0, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 1);
        assert!(!log.borrow().iter().any(|c| c.0 == LEAVE_CRITICAL_SECTION));
        e.call(0x00c9_04e0, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 0);
        assert_eq!(*slot.borrow(), 0);
        assert!(log
            .borrow()
            .contains(&(LEAVE_CRITICAL_SECTION, LOCK + 0x18)));
    }

    #[test]
    fn add_reference_locks_only_when_enabled_and_not_owner() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        let (obj, _) = object(&mut e, 0x20, 1);
        // Locking disabled: plain increment.
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(!entered(&log));
        // Locking enabled, thread not the owner: locked.
        e.mem.set_u32(LOCK + 8, 1);
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 3);
        assert!(log
            .borrow()
            .contains(&(ENTER_CRITICAL_SECTION, LOCK + 0x18)));
        assert!(log
            .borrow()
            .contains(&(LEAVE_CRITICAL_SECTION, LOCK + 0x18)));
        // Not counted: untouched.
        let (plain, _) = object(&mut e, 0, 1);
        e.call(0x00c9_0510, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 1);
    }

    #[test]
    fn add_reference_skips_the_lock_for_the_owner() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        *slot.borrow_mut() = OWNER_MARK;
        e.mem.set_u32(LOCK + 8, 1);
        let (obj, _) = object(&mut e, 0x20, 1);
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(!entered(&log));
    }

    #[test]
    fn remove_reference_destroys_the_last_reference_without_locking() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        e.mem.set_u32(LOCK + 8, 1);
        let (obj, destroyed) = object(&mut e, 0x20, 1);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 0);
        assert_eq!(*destroyed.borrow(), vec![obj, 1]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn remove_reference_decrements_under_the_lock() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        let (obj, destroyed) = object(&mut e, 0x20, 3);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(LOCK + 8, 1);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 1);
        assert!(entered(&log));
        assert!(destroyed.borrow().is_empty());
        let (plain, _) = object(&mut e, 0, 3);
        e.call(0x00c9_05b0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 3);
    }
}
