//! `Havok SDK/hkpboxmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`).
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// `hkpBoxMotion`'s vtable (the vtable `010c6dc4` is that class; RTTI read at the vtable)
/// and the `float` constant `1.0` stored in four lanes.
const BOX_MOTION_VTABLE: u32 = 0x010c_6dc4;
const FLOAT_ONE: u32 = 0x0101_2068;

// Translated from 00d1e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the box motion (type 3): the motion base constructor with a trailing 0,
/// the vtable, motion type 3 at +8 and the float at `01012068` (1.0) in all four lanes of
/// the vector at +0xC0. Returns `this`.
pub fn fn_00d1e690(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    let one = e.mem.u32(FLOAT_ONE);
    e.mem.set_u32(this.addr(), BOX_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 3);
    for lane in 0..4 {
        e.mem.set_u32(this.addr() + 0xc0 + 4 * lane, one);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1_e690, fn_00d1e690(Ptr, Ptr, Ptr) -> Ptr)]
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
    fn construct_fills_the_box_motion_fields() {
        let mut e = test_engine();
        e.set_global(FLOAT_ONE, 1.0f32);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00d1_e690, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6, 0]]);
        assert_eq!(e.mem.u32(this), BOX_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 3);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0xc0 + 4 * lane), 1.0);
        }
    }
}
