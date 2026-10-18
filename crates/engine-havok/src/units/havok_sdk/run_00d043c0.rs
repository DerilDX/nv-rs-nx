//! `Havok SDK/run_00d043c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkSemaphore::release` (`00d04390`, `this` = the semaphore, one word: the count to release;
/// named by Ghidra from the Xbox PDB). It releases the Windows semaphore whose handle is the
/// first word of the object and returns the import's result.
const SEMAPHORE_RELEASE: u32 = 0x00d0_4390;

// Translated from 00d043c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(semaphore, count)`: `hkSemaphore::release(semaphore, count)` with the arguments
/// swapped into its `thiscall` form. Returns what that call left in `EAX`.
pub fn fn_00d043c0(e: &mut Engine, semaphore: Ptr, count: u32) -> u32 {
    e.call(SEMAPHORE_RELEASE, &args![semaphore, count]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d0_43c0, fn_00d043c0(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The semaphore and the count reach `release` and its result comes back.
    #[test]
    fn passes_the_semaphore_and_count_on() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SEMAPHORE_RELEASE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00d0_43c0, &args![0x1234u32, 3u32]).u32(), 1);
        assert_eq!(*seen.borrow(), vec![vec![0x1234, 3]]);
    }
}
