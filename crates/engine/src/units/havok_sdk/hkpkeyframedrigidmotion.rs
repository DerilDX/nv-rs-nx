//! `Havok SDK/hkpkeyframedrigidmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`).
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpKeyframedRigidMotion`'s vtable (RTTI confirmed) and the `hkReferencedObject` base's.
const KEYFRAMED_MOTION_VTABLE: u32 = 0x010c_6bec;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00d1dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::hkpKeyframedRigidMotion` (Xbox PDB): the motion base
/// constructor (`00c86530` with `(position, rotation, 0)`), the vtable, a null pointer at
/// +0x118, a zero 16-bit word at +0x11C, a zero vector at +0xC0 and motion type 4 at +8.
/// Returns `this`.
pub fn hkp_keyframed_rigid_motion_construct(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), KEYFRAMED_MOTION_VTABLE);
    e.mem.set_u32(this.addr() + 0x118, 0);
    e.mem.set_u16(this.addr() + 0x11c, 0);
    e.mem.write(this.addr() + 0xc0, &[0; 16]);
    e.mem.set_u8(this.addr() + 8, 4);
    this
}

// Translated from 00d1dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::~hkpKeyframedRigidMotion` (Xbox PDB): releases the object held
/// at +0x118 (when there is one) and leaves the `hkReferencedObject` vtable.
pub fn hkp_keyframed_rigid_motion_destruct(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr() + 0x118);
    e.mem.set_u32(this.addr(), KEYFRAMED_MOTION_VTABLE);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(this.addr(), REFERENCED_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_dfa0, hkp_keyframed_rigid_motion_construct(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d1_dff0, hkp_keyframed_rigid_motion_destruct(Ptr)),
    ]
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
    fn construct_sets_up_the_keyframed_motion() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x120);
        e.mem.write(this + 0xc0, &[0xff; 16]);
        e.mem.set_u32(this + 0x118, 0x1234);
        e.mem.set_u16(this + 0x11c, 9);
        assert_eq!(e.call(0x00d1_dfa0, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6, 0]]);
        assert_eq!(e.mem.u32(this), KEYFRAMED_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 4);
        assert_eq!(e.mem.u32(this + 0x118), 0);
        assert_eq!(e.mem.u16(this + 0x11c), 0);
        assert_eq!(e.mem.bytes(this + 0xc0, 16), vec![0; 16]);
    }

    #[test]
    fn destruct_releases_the_held_object_only_when_there_is_one() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x120);
        e.call(0x00d1_dff0, &args![this]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u32(this), REFERENCED_OBJECT_VTABLE);
        e.mem.set_u32(this + 0x118, 0x77);
        e.call(0x00d1_dff0, &args![this]);
        assert_eq!(*seen.borrow(), vec![0x77]);
    }
}
