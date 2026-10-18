//! `Havok SDK/run_00dfbd30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfbd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_60f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfbd40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_60f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dfbd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_ec3c.
pub fn return_constant_00dfbd30(_e: &mut Engine) -> u32 {
    0x010d_ec3c
}

// Translated from 00dfbd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `8` (`SUB ECX,0x8`) and jumps to the deleting destructor
/// `00dfbd40` (`RET 4`); the caller gets its `EAX`, `this - 8`. The class is unconfirmed.
pub fn fn_00dfbd70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00dfbd40(e, Ptr::new(this.addr().wrapping_sub(0x8)), flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfbd30, return_constant_00dfbd30() -> u32),
        entry!(0x00df_bd40, deleting_destructor_00dfbd40(Ptr, u8) -> Ptr),
        entry!(0x00df_bd70, fn_00dfbd70(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00dfbd40_frees_only_when_bit_zero_is_set() {
        check(0x00df_bd40, 0x00e3_60f0);
    }

    #[test]
    fn return_constant_00dfbd30_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_bd30, &args![]).u32(), 0x010d_ec3c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The thunk runs the deleting destructor on `this - 8` (the base destructor `00e360f0`
    /// is a double) and returns that address; without bit 0 nothing is freed.
    #[test]
    fn fn_00dfbd70_runs_the_destructor_on_this_minus_eight() {
        let mut e = Engine::new();
        e.register(0x00e3_60f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20) + 8;
        let got = e.call(0x00df_bd70, &args![this, 0u32]).u32();
        assert_eq!(got, this - 8);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00e3_60f0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this - 8]);
        assert!(log.iter().all(|(a, _)| *a != 0x00c8_5750));
    }
}
