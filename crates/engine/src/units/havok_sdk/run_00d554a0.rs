//! `Havok SDK/run_00d554a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d1f1e0` (`this`): counts the children of a shape container by iterating its virtual
/// first-key / next-key functions until the key is -1 (the map names it `hkpShapeContainer::
/// getNumChildShapes`).
const COUNT_CHILD_SHAPES: u32 = 0x00d1_f1e0;

// Translated from 00d554a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the count cached at `this + 0xD0`, computing it with `00d1f1e0` the first time
/// (while the cached value is negative). The class is unconfirmed.
pub fn fn_00d554a0(e: &mut Engine, this: Ptr) -> i32 {
    if e.mem.i32(this.addr() + 0xd0) < 0 {
        let count = e.call(COUNT_CHILD_SHAPES, &args![this]).i32();
        e.mem.set_i32(this.addr() + 0xd0, count);
    }
    e.mem.i32(this.addr() + 0xd0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5_54a0, fn_00d554a0(Ptr) -> i32)]
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

    #[test]
    fn the_count_is_computed_once_and_cached() {
        let mut e = test_engine();
        let calls = Rc::new(RefCell::new(0));
        let counter = Rc::clone(&calls);
        e.register_double(COUNT_CHILD_SHAPES, move |_, _| {
            *counter.borrow_mut() += 1;
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x100);
        e.mem.set_i32(this + 0xd0, -1);
        assert_eq!(e.call(0x00d5_54a0, &args![this]).i32(), 7);
        assert_eq!(e.call(0x00d5_54a0, &args![this]).i32(), 7);
        assert_eq!(*calls.borrow(), 1);
        assert_eq!(e.mem.i32(this + 0xd0), 7);
        // A count of zero is already cached.
        e.mem.set_i32(this + 0xd0, 0);
        assert_eq!(e.call(0x00d5_54a0, &args![this]).i32(), 0);
        assert_eq!(*calls.borrow(), 1);
    }
}
