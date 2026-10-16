//! `Havok SDK/hkpfixedrigidmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpKeyframedRigidMotion::hkpKeyframedRigidMotion` (Xbox PDB, `00d1dfa0`, `this`,
/// `(position, rotation)`), the base constructor of the fixed motion.
const KEYFRAMED_MOTION_CONSTRUCT: u32 = 0x00d1_dfa0;
/// `hkpFixedRigidMotion`'s vtable (RTTI confirmed).
const FIXED_MOTION_VTABLE: u32 = 0x010c_6cdc;

// Translated from 00d1e160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::hkpFixedRigidMotion` (Xbox PDB): the keyframed motion constructor
/// on `this`, then the fixed motion's vtable and motion type 5 at +8. Returns `this`.
pub fn hkp_fixed_rigid_motion_construct(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) -> Ptr {
    e.call(KEYFRAMED_MOTION_CONSTRUCT, &args![this, position, rotation]);
    e.mem.set_u32(this.addr(), FIXED_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 5);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1_e160, hkp_fixed_rigid_motion_construct(Ptr, Ptr, Ptr) -> Ptr)]
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
    fn construct_runs_the_keyframed_constructor_then_marks_the_motion_fixed() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(KEYFRAMED_MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d1_e160, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6]]);
        assert_eq!(e.mem.u32(this), FIXED_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 5);
    }
}
