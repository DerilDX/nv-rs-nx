//! `BSHavok/bhkwindlistener.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class is `bhkWindListener` (RTTI of vtable `010c4b2c`, whose slot 0 is `00c74780`); the
//! destructor body is shared with `bhkWaterListener` (vtable `010c88b8`, RTTI-confirmed, slot 0
//! the same function).

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable of `bhkWindListener`.
const VTABLE_WIND_LISTENER: u32 = 0x010c_4b2c;
/// The destructor body (`005545f0`, thiscall on `this`).
const DESTRUCTOR_BODY: u32 = 0x0055_45f0;
/// `MemoryManager::Deallocate` (`00aa4060`, thiscall `(block)`) on the manager `011f6238`.
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;

// Translated from 00c74760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWindListener` constructor (RTTI of vtable `010c4b2c`): clears `+0xc` and `+0x10`, sets
/// the word at `+4` and the byte at `+8` to 1, stores the vtable. Answers `this`.
pub fn bhk_wind_listener_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 4, 1);
    e.mem.set_u8(me + 8, 1);
    e.mem.set_u32(me, VTABLE_WIND_LISTENER);
    this
}

// Translated from 00c74780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWindListener` scalar deleting destructor (slot 0 of vtable `010c4b2c`, shared with
/// `bhkWaterListener`; the map names it `bhkWaterListener::_scalar_deleting_destructor_`): runs
/// the destructor body `005545f0` and, when bit 0 of `flags` is set, hands the block to
/// `MemoryManager::Deallocate`. Answers `this`.
pub fn bhk_wind_listener_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c74760, bhk_wind_listener_construct(Ptr) -> Ptr),
        entry!(0x00c74780, bhk_wind_listener_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn construct_sets_the_fields_and_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in 0..0x20 {
            e.mem.set_u8(this + offset, 0xee);
        }
        assert_eq!(e.call(0x00c74760, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_WIND_LISTENER);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u8(this + 9), 0xee);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_on_bit_0() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for address in [DESTRUCTOR_BODY, MEMORY_DEALLOCATE] {
            let kept = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = 0x5000u32;
        assert_eq!(e.call(0x00c74780, &args![this, 0xfeu8]).u32(), this);
        assert_eq!(*log.borrow(), vec![(DESTRUCTOR_BODY, vec![this])]);
        assert_eq!(e.call(0x00c74780, &args![this, 1u8]).u32(), this);
        assert_eq!(
            log.borrow()[1..],
            [
                (DESTRUCTOR_BODY, vec![this]),
                (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, this])
            ]
        );
    }
}
