//! `Havok SDK/hkpshapephantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_8130`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9eb00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_8130, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpPhantom::updateBroadPhase` (Xbox PDB, `00d07ec0`, `this`, `(aabb)`).
const PHANTOM_UPDATE_BROAD_PHASE: u32 = 0x00d0_7ec0;
/// The `double` `0.5` the collidable's radius is scaled by (`01011588`).
const HALF_DOUBLE: u32 = 0x0101_1588;

/// The shape's box for the phantom's broad-phase update: `half = collidable radius * 0.5`
/// rounded to `float` (the radius is the `float` at +8 of the structure at +0x78 of the
/// collidable at +8).
fn collidable_half_radius(e: &Engine, collidable: u32) -> f32 {
    let owner = e.mem.u32(collidable + 0x78);
    let radius = f64::from(x87_quiet(e.mem.f32(owner + 8)));
    (radius * e.global::<f64>(HALF_DOUBLE)) as f32
}

// Translated from 00c9e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::getMotionState` (Xbox PDB): the motion state embedded at `this + 0xB0`.
pub fn hkp_shape_phantom_get_motion_state(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0xb0)
}

// Translated from 00c9e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::setTransform` (Xbox PDB): copies the 4x4 transform (64 bytes) to the
/// motion state at +0xB0; when the phantom has a collidable (+8), asks the shape (+0x10,
/// slot `0x1c`) for its box at `(transform, radius * 0.5, out)` and passes it to
/// `hkpPhantom::updateBroadPhase`.
pub fn hkp_shape_phantom_set_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    for offset in (0..0x40).step_by(4) {
        let word = e.mem.u32(transform.addr() + offset);
        e.mem.set_u32(this.addr() + 0xb0 + offset, word);
    }
    let collidable = e.mem.u32(this.addr() + 8);
    if collidable != 0 {
        let tolerance = x87_quiet(collidable_half_radius(e, collidable));
        let shape = e.mem.u32(this.addr() + 0x10);
        e.with_stack(0x30, |e, aabb| {
            e.vcall(shape, 0x1c, &args![transform, tolerance, aabb]);
            e.call(PHANTOM_UPDATE_BROAD_PHASE, &args![this, aabb]);
        });
    }
}

// Translated from 00c9e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::setPosition` (Xbox PDB): copies the position vector (16 bytes) to the
/// translation column of the motion state (+0xE0); when the phantom has a collidable (+8),
/// asks the shape (+0x10, slot `0x1c`) for its box at `(motion state, radius * 0.5 +
/// tolerance, out)` (each step rounded to `float`) and passes it to
/// `hkpPhantom::updateBroadPhase`.
pub fn hkp_shape_phantom_set_position(e: &mut Engine, this: Ptr, position: Ptr, tolerance: f32) {
    for offset in (0..0x10).step_by(4) {
        let word = e.mem.u32(position.addr() + offset);
        e.mem.set_u32(this.addr() + 0xe0 + offset, word);
    }
    let collidable = e.mem.u32(this.addr() + 8);
    if collidable != 0 {
        let half = x87_quiet(collidable_half_radius(e, collidable));
        let total = x87_quiet((f64::from(half) + f64::from(x87_quiet(tolerance))) as f32);
        let shape = e.mem.u32(this.addr() + 0x10);
        let motion_state = this.addr() + 0xb0;
        e.with_stack(0x30, |e, aabb| {
            e.vcall(shape, 0x1c, &args![motion_state, total, aabb]);
            e.call(PHANTOM_UPDATE_BROAD_PHASE, &args![this, aabb]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_e810, hkp_shape_phantom_get_motion_state(Ptr) -> Ptr),
        entry!(0x00c9_e910, hkp_shape_phantom_set_transform(Ptr, Ptr)),
        entry!(0x00c9_e990, hkp_shape_phantom_set_position(Ptr, Ptr, f32)),
        entry!(0x00c9_eb00, deleting_destructor_00c9eb00(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00c9eb00_frees_only_when_bit_zero_is_set() {
        check(0x00c9_eb00, 0x00d0_8130);
    }

    const SHAPE_VTABLE: u32 = 0x0a00_0000;
    const SHAPE_AABB: u32 = 0x0a10_0001;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A phantom with a collidable whose radius structure holds `radius`, a shape whose
    /// `getAabb` (slot 0x1c) and the broad-phase update log their arguments.
    fn phantom(e: &mut Engine, radius: f32) -> (u32, CallLog) {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        let mut slots = vec![0u32; 8];
        slots[0x1c / 4] = SHAPE_AABB;
        e.put_vtable(SHAPE_VTABLE, &slots);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, SHAPE_VTABLE);
        let owner = e.mem.alloc(0x20);
        e.mem.set_f32(owner + 8, radius);
        let collidable = e.mem.alloc(0x80);
        e.mem.set_u32(collidable + 0x78, owner);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, collidable);
        e.mem.set_u32(this + 0x10, shape);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let aabb_log = Rc::clone(&log);
        e.register_double(SHAPE_AABB, move |_, a| {
            aabb_log.borrow_mut().push(vec![1, a[0], a[1], a[2]]);
            Ret::default()
        });
        let update_log = Rc::clone(&log);
        e.register_double(PHANTOM_UPDATE_BROAD_PHASE, move |_, a| {
            update_log.borrow_mut().push(vec![2, a[0], a[1]]);
            Ret::default()
        });
        (this, log)
    }

    #[test]
    fn get_motion_state_is_this_plus_0xb0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_e810, &args![0x1000u32]).u32(), 0x10b0);
    }

    #[test]
    fn set_transform_copies_the_transform_and_updates_the_broad_phase() {
        let mut e = Engine::new();
        let (this, log) = phantom(&mut e, 3.0);
        let transform = e.mem.alloc(0x40);
        for offset in (0..0x40).step_by(4) {
            e.mem.set_u32(transform + offset, 0x100 + offset);
        }
        e.call(0x00c9_e910, &args![this, transform]);
        for offset in (0..0x40).step_by(4) {
            assert_eq!(e.mem.u32(this + 0xb0 + offset), 0x100 + offset);
        }
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        // getAabb(this = shape, transform, radius * 0.5, out)
        let shape = e.mem.u32(this + 0x10);
        assert_eq!(log[0][..3], [1, shape, transform]);
        assert_eq!(f32::from_bits(log[0][3]), 1.5);
        assert_eq!(log[1][..2], [2, this]);
    }

    #[test]
    fn set_position_adds_the_tolerance_and_skips_the_update_without_a_collidable() {
        let mut e = Engine::new();
        let (this, log) = phantom(&mut e, 3.0);
        let position = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(position + offset, 0x200 + offset);
        }
        e.call(0x00c9_e990, &args![this, position, 0.25f32]);
        for offset in (0..0x10).step_by(4) {
            assert_eq!(e.mem.u32(this + 0xe0 + offset), 0x200 + offset);
        }
        {
            let log = log.borrow();
            assert_eq!(log.len(), 2);
            // getAabb(this = shape, motion state, radius * 0.5 + tolerance, out)
            assert_eq!(log[0][2], this + 0xb0);
            assert_eq!(f32::from_bits(log[0][3]), 1.75);
            assert_eq!(log[1][..2], [2, this]);
        }
        log.borrow_mut().clear();
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(position, 0x300);
        e.call(0x00c9_e990, &args![this, position, 0.25f32]);
        assert_eq!(e.mem.u32(this + 0xe0), 0x300);
        assert!(log.borrow().is_empty());
    }
}
