//! `Havok SDK/hkprigidbody.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ca9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ca_80a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ca9210(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ca_80a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`) every motion
/// type below starts with.
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// The world's operation queue (`00c91160`, `this` = the world, `(operation)`), used while
/// the world's lock count (+0x94) is non-zero.
const WORLD_QUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpEntity::isActive` (Xbox PDB, `00c9bff0`, `this` = the entity, `(out)`): returns a
/// pointer to a byte, non-zero when the entity is active.
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// `hkpEntity::activate` (Xbox PDB, `00c9c1d0`).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `hkpWorldCallbackUtil::fireInactiveEntityMoved` (Xbox PDB, `00d017b0`, cdecl `(world,
/// entity)`).
const FIRE_INACTIVE_ENTITY_MOVED: u32 = 0x00d0_17b0;
/// `hkpSimulation::collideEntitiesBroadPhaseDiscrete` (Xbox PDB, `00cf9c60`, cdecl
/// `(entities, count, world)`).
const COLLIDE_ENTITIES_BROAD_PHASE: u32 = 0x00cf_9c60;
/// Two world calls that finish a critical section (`00c91120`, `00c91140`, `this` = the
/// world).
const WORLD_FLUSH_FIRST: u32 = 0x00c9_1120;
const WORLD_FLUSH_SECOND: u32 = 0x00c9_1140;
/// `hkpWorldOperationUtil::setRigidBodyMotionType` (Xbox PDB, `00cb6f70`, cdecl `(body, a, b,
/// c)`).
const SET_RIGID_BODY_MOTION_TYPE: u32 = 0x00cb_6f70;
/// `hkpWorldCallbackUtil::fireEntitySetMotionType` (Xbox PDB, `00d006e0`, cdecl `(world,
/// entity)`) and `hkpEntityCallbackUtil::fireEntitySetMotionType` (`00cffda0`, cdecl
/// `(entity)`).
const WORLD_FIRE_SET_MOTION_TYPE: u32 = 0x00d0_06e0;
const ENTITY_FIRE_SET_MOTION_TYPE: u32 = 0x00cf_fda0;
/// The `float` helpers of `createDynamicRigidMotion` (cdecl `(float, float)`, result in
/// ST0): `00c9e0f0` answers the smaller of its two floats and `00c98210` the larger (read from
/// their bodies in Ghidra; their names are unknown).
const FLOAT_MIN: u32 = 0x00c9_e0f0;
const FLOAT_MAX: u32 = 0x00c9_8210;
/// `hkUFloat8::operator=(const float&)` (Xbox PDB, `00ca9360`, `this`, `(pointer to float)`).
const UFLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// The motion constructors `createDynamicRigidMotion` selects (`this` = the buffer,
/// `(position, rotation)`): `00ca9270` sphere, `00d1e690` box, `00d21930` (type 6),
/// `00d1dfa0` keyframed, `00ca9240` character, `00d1e160` fixed.
const SPHERE_MOTION_CONSTRUCT: u32 = 0x00ca_9270;
const BOX_MOTION_CONSTRUCT: u32 = 0x00d1_e690;
const MOTION_TYPE_SIX_CONSTRUCT: u32 = 0x00d2_1930;
const KEYFRAMED_MOTION_CONSTRUCT: u32 = 0x00d1_dfa0;
const CHARACTER_MOTION_CONSTRUCT: u32 = 0x00ca_9240;
const FIXED_MOTION_CONSTRUCT: u32 = 0x00d1_e160;
/// `0.8` as a `double` (`010217d8`) and the keyframed motion's `1.0e6f` (`0108cb00`).
const INERTIA_RATIO: u32 = 0x0102_17d8;
const KEYFRAMED_DAMPING: u32 = 0x0108_cb00;
/// Vtables the two constructors below write.
const CHARACTER_MOTION_VTABLE: u32 = 0x010c_6ea4;
const SPHERE_MOTION_VTABLE: u32 = 0x010c_6d54;

// Translated from 00ca9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkpCharacterMotion` (RTTI of the vtable `010c6ea4` confirmed): the motion
/// base constructor with a trailing 0, the vtable, and motion type 7 at +8.
pub fn fn_00ca9240(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), CHARACTER_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 7);
    this
}

// Translated from 00ca9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkpSphereMotion` (RTTI of the vtable `010c6d54` confirmed): the motion
/// base constructor with a trailing 0, the vtable, and motion type 2 at +8.
pub fn fn_00ca9270(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), SPHERE_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 2);
    this
}

// Translated from 00ca8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setPositionAndRotation` (Xbox PDB name; the body matches): calls the
/// motion embedded at +0xE0 (vtable slot 0x38 with the two arguments), then `00ca8700` on
/// the body.
pub fn hkp_rigid_body_set_position_and_rotation(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) {
    e.vcall(this.addr() + 0xe0, 0x38, &args![position, rotation]);
    fn_00ca8700(e, this);
}

// Translated from 00ca8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setMotionType` (Xbox PDB): when the body's world (+8) is locked (its +0x94
/// is non-zero) the change is queued as an operation (type 4 at +0, the body at +4 and the
/// three arguments as bytes at +8, +9 and +0xA); otherwise it is applied now: the world
/// operation `00cb6f70`, then the world's and the entity's callbacks.
pub fn hkp_rigid_body_set_motion_type(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) {
    let world = e.mem.u32(this.addr() + 8);
    if world != 0 && e.mem.u32(world + 0x94) != 0 {
        e.with_stack(0xc, |e, operation| {
            let op = operation.addr();
            e.mem.set_u8(op, 4);
            e.mem.set_u32(op + 4, this.addr());
            e.mem.set_u8(op + 8, first as u8);
            e.mem.set_u8(op + 9, second as u8);
            e.mem.set_u8(op + 0xa, third as u8);
            e.call(WORLD_QUEUE_OPERATION, &args![world, operation]);
        });
        return;
    }
    e.call(
        SET_RIGID_BODY_MOTION_TYPE,
        &args![this, first, second, third],
    );
    let world = e.mem.u32(this.addr() + 8);
    if world != 0 {
        e.call(WORLD_FIRE_SET_MOTION_TYPE, &args![world, this]);
    }
    e.call(ENTITY_FIRE_SET_MOTION_TYPE, &args![this]);
}

// Translated from 00ca8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::updateBroadphaseAndResetCollisionInformationOfWarpedBody` (Xbox PDB): with
/// no world (+8) nothing happens; with the world locked (+0x94 non-zero) the update is
/// queued as an operation of type 0x16 with the body at +4. Otherwise the lock count becomes
/// 1 while the world's simulation (+8, vtable slot 0x20) gets the body as a one-entity list
/// with 7, the broad phase is updated when the body has a collidable (+0x10), an inactive
/// body is activated (when the world allows it, +0xD6, and the body is not fixed, +0xE8
/// is not 5) and reported moved, the world's listener (+0x5C, slot 0x14) is called with 0,
/// and when the count returns to 0 with no pending flag (+0x9C) the two world flushes run.
pub fn fn_00ca8700(e: &mut Engine, body: Ptr) {
    let world = e.mem.u32(body.addr() + 8);
    if world == 0 {
        return;
    }
    if e.mem.u32(world + 0x94) != 0 {
        e.with_stack(8, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x16);
            e.mem.set_u32(operation.addr() + 4, body.addr());
            e.call(WORLD_QUEUE_OPERATION, &args![world, operation]);
        });
        return;
    }
    e.mem.set_u32(world + 0x94, 1);
    let simulation = e.mem.u32(world + 8);
    e.with_stack(4, |e, entities| {
        e.mem.set_u32(entities.addr(), body.addr());
        e.vcall(simulation, 0x20, &args![entities, 1u32, world, 7u32]);
        if e.mem.u32(body.addr() + 0x10) != 0 {
            e.call(COLLIDE_ENTITIES_BROAD_PHASE, &args![entities, 1u32, world]);
        }
    });
    let active = e.with_stack(4, |e, out| {
        let flag = e.call(ENTITY_IS_ACTIVE, &args![body, out]).u32();
        e.mem.u8(flag) != 0
    });
    if !active {
        if e.mem.u8(world + 0xd6) != 0 && e.mem.u8(body.addr() + 0xe8) != 5 {
            e.call(ENTITY_ACTIVATE, &args![body]);
        }
        e.call(FIRE_INACTIVE_ENTITY_MOVED, &args![world, body]);
    }
    let listener = e.mem.u32(world + 0x5c);
    if listener != 0 {
        e.vcall(listener, 0x14, &args![0u32]);
    }
    let lock_count = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, lock_count);
    if lock_count == 0 && e.mem.u8(world + 0x9c) == 0 {
        if e.mem.u32(world + 0x8c) != 0 {
            e.call(WORLD_FLUSH_FIRST, &args![world]);
        }
        if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
            e.call(WORLD_FLUSH_SECOND, &args![world]);
        }
    }
}

// Translated from 00ca8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::createDynamicRigidMotion` (Xbox PDB): builds the motion for `motion_type`
/// in `buffer` with the constructor of its class (a null buffer gives a null motion): 1 picks
/// sphere (`00ca9270`) or box (`00d1e690`) from the diagonal of the inertia matrix (box
/// unless the smallest of the three entries is above 0.8 of the largest; an unordered
/// comparison also gives box), 2 sphere, 3 box, 4 keyframed (`00d1dfa0`; both damping values
/// become the constant `1.0e6` and the virtual set-up calls are skipped), 6 `00d21930`,
/// 7 character, anything else fixed. For every type but 4 the motion's vtable slots 0x1C
/// (the inertia), 0x2C (`sixth`) and 0xC (`mass`) are called. Then the two damping values
/// are assigned to the `hkUFloat8`s at +0xBC and +0xBD of the motion. The exe pushes nine
/// words and the caller removes them.
#[allow(clippy::too_many_arguments)]
pub fn hkp_rigid_body_create_dynamic_rigid_motion(
    e: &mut Engine,
    motion_type: u32,
    position: Ptr,
    rotation: Ptr,
    mass: f32,
    inertia: Ptr,
    sixth: u32,
    mut first_damping: f32,
    mut second_damping: f32,
    buffer: Ptr,
) {
    let constructor = match motion_type.wrapping_sub(1) {
        0 => {
            let diagonal = |e: &Engine, off: u32| x87_quiet(e.mem.f32(inertia.addr() + off));
            let (a, b, c) = (diagonal(e, 0), diagonal(e, 0x14), diagonal(e, 0x28));
            let inner_min = e.call(FLOAT_MIN, &args![b, c]).f32();
            let smallest = e.call(FLOAT_MIN, &args![a, inner_min]).f32();
            let inner_max = e.call(FLOAT_MAX, &args![b, c]).f32();
            let largest = e.call(FLOAT_MAX, &args![a, inner_max]).f64();
            let ratio: f64 = e.global(INERTIA_RATIO);
            // `FCOMPP` then `TEST AH,0x41`: box unless `smallest > largest * 0.8`.
            if f64::from(smallest) > largest * ratio {
                SPHERE_MOTION_CONSTRUCT
            } else {
                BOX_MOTION_CONSTRUCT
            }
        }
        1 => SPHERE_MOTION_CONSTRUCT,
        2 => BOX_MOTION_CONSTRUCT,
        3 => {
            let damping = x87_quiet(e.global::<f32>(KEYFRAMED_DAMPING));
            first_damping = damping;
            second_damping = damping;
            KEYFRAMED_MOTION_CONSTRUCT
        }
        5 => MOTION_TYPE_SIX_CONSTRUCT,
        6 => CHARACTER_MOTION_CONSTRUCT,
        _ => FIXED_MOTION_CONSTRUCT,
    };
    let motion = if buffer.addr() != 0 {
        e.call(constructor, &args![buffer, position, rotation])
            .u32()
    } else {
        0
    };
    if motion_type != 4 {
        e.vcall(motion, 0x1c, &args![inertia]);
        e.vcall(motion, 0x2c, &args![sixth]);
        e.vcall(motion, 0xc, &args![x87_quiet(mass)]);
    }
    e.with_stack(8, |e, floats| {
        e.mem.set_f32(floats.addr(), first_damping);
        e.mem.set_f32(floats.addr() + 4, second_damping);
        e.call(UFLOAT8_ASSIGN, &args![motion + 0xbc, floats]);
        e.call(UFLOAT8_ASSIGN, &args![motion + 0xbd, floats.byte_add(4)]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_9210, deleting_destructor_00ca9210(Ptr, u8) -> Ptr),
        entry!(
            0x00ca_8240,
            hkp_rigid_body_create_dynamic_rigid_motion(u32, Ptr, Ptr, f32, Ptr, u32, f32, f32, Ptr)
        ),
        entry!(
            0x00ca_8420,
            hkp_rigid_body_set_motion_type(Ptr, u32, u32, u32)
        ),
        entry!(0x00ca_8700, fn_00ca8700(Ptr)),
        entry!(
            0x00ca_8860,
            hkp_rigid_body_set_position_and_rotation(Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_9240, fn_00ca9240(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ca_9270, fn_00ca9270(Ptr, Ptr, Ptr) -> Ptr),
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
    fn deleting_destructor_00ca9210_frees_only_when_bit_zero_is_set() {
        check(0x00ca_9210, 0x00ca_80a0);
    }
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

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Registers a double at each address that logs `(address, arguments)` and answers
    /// `eax` / `st0` from `answers` (default 0).
    fn log_calls(e: &mut Engine, addresses: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn character_and_sphere_constructors_write_vtable_and_type() {
        let mut e = test_engine();
        let log = log_calls(&mut e, &[MOTION_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00ca_9240, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CHARACTER_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 7);
        assert_eq!(e.call(0x00ca_9270, &args![this, 7u32, 8u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SPHERE_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 2);
        assert_eq!(
            calls(&log, MOTION_CONSTRUCT),
            vec![vec![this, 5, 6, 0], vec![this, 7, 8, 0]]
        );
    }

    #[test]
    fn set_position_and_rotation_calls_the_embedded_motion_then_updates() {
        let mut e = test_engine();
        let log = log_calls(&mut e, &[0x0900_0001]);
        let body = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x38, 0x0900_0001);
        e.mem.set_u32(body + 0xe0, vtable);
        e.call(0x00ca_8860, &args![body, 11u32, 12u32]);
        assert_eq!(calls(&log, 0x0900_0001), vec![vec![body + 0xe0, 11, 12]]);
    }

    #[test]
    fn set_motion_type_queues_when_the_world_is_locked() {
        let mut e = test_engine();
        let queued = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&queued);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            sink.borrow_mut().push((
                e.mem.u8(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.bytes(a[1] + 8, 3),
            ));
            Ret::default()
        });
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x94, 1);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8420, &args![body, 0x101u32, 2u32, 3u32]);
        assert_eq!(*queued.borrow(), vec![(4, body, vec![1, 2, 3])]);
    }

    #[test]
    fn set_motion_type_applies_when_the_world_is_not_locked() {
        let mut e = test_engine();
        let log = log_calls(
            &mut e,
            &[
                SET_RIGID_BODY_MOTION_TYPE,
                WORLD_FIRE_SET_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
                WORLD_QUEUE_OPERATION,
            ],
        );
        let world = e.mem.alloc(0x100);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8420, &args![body, 1u32, 2u32, 3u32]);
        e.mem.set_u32(body + 8, 0);
        e.call(0x00ca_8420, &args![body, 4u32, 5u32, 6u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                SET_RIGID_BODY_MOTION_TYPE,
                WORLD_FIRE_SET_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
                SET_RIGID_BODY_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
            ]
        );
        assert_eq!(
            calls(&log, SET_RIGID_BODY_MOTION_TYPE),
            vec![vec![body, 1, 2, 3], vec![body, 4, 5, 6]]
        );
        assert_eq!(
            calls(&log, WORLD_FIRE_SET_MOTION_TYPE),
            vec![vec![world, body]]
        );
    }

    /// A world with a simulation object (slot 0x20 double at `0x0900_0020`), a listener
    /// (slot 0x14 double at `0x0900_0014`) and a body.
    fn world_rig(e: &mut Engine) -> (u32, u32, Log) {
        let log = log_calls(
            e,
            &[
                0x0900_0020,
                0x0900_0014,
                COLLIDE_ENTITIES_BROAD_PHASE,
                ENTITY_ACTIVATE,
                FIRE_INACTIVE_ENTITY_MOVED,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
                WORLD_QUEUE_OPERATION,
            ],
        );
        let simulation_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(simulation_vtable + 0x20, 0x0900_0020);
        let simulation = e.mem.alloc(0x10);
        e.mem.set_u32(simulation, simulation_vtable);
        let listener_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(listener_vtable + 0x14, 0x0900_0014);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, listener_vtable);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 8, simulation);
        e.mem.set_u32(world + 0x5c, listener);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        (world, body, log)
    }

    #[test]
    fn warped_body_update_without_a_world_or_with_a_locked_one() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        e.mem.set_u32(body + 8, 0);
        e.call(0x00ca_8700, &args![body]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(body + 8, world);
        e.mem.set_u32(world + 0x94, 2);
        e.call(0x00ca_8700, &args![body]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(calls(&log, WORLD_QUEUE_OPERATION)[0][0], world);
        assert_eq!(e.mem.u32(world + 0x94), 2);
    }

    #[test]
    fn warped_body_update_of_an_inactive_body_activates_and_flushes() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        // isActive answers a pointer to a zero byte.
        let flag = e.mem.alloc(4);
        e.register_double(ENTITY_IS_ACTIVE, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.mem.set_u32(body + 0x10, 0x1234);
        e.mem.set_u8(world + 0xd6, 1);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        e.call(0x00ca_8700, &args![body]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x0900_0020,
                COLLIDE_ENTITIES_BROAD_PHASE,
                ENTITY_ACTIVATE,
                FIRE_INACTIVE_ENTITY_MOVED,
                0x0900_0014,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
            ]
        );
        let simulation_call = &calls(&log, 0x0900_0020)[0];
        assert_eq!(simulation_call[2..], [1, world, 7]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn warped_body_update_of_an_active_body_skips_activation() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(ENTITY_IS_ACTIVE, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.mem.set_u8(world + 0x9c, 1);
        e.call(0x00ca_8700, &args![body]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![0x0900_0020, 0x0900_0014]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    /// `createDynamicRigidMotion` with every callee a double; the motion object has the
    /// slots 0x1C, 0x2C and 0xC.
    fn motion_rig(e: &mut Engine) -> (u32, Log) {
        let log = log_calls(
            e,
            &[
                SPHERE_MOTION_CONSTRUCT,
                BOX_MOTION_CONSTRUCT,
                MOTION_TYPE_SIX_CONSTRUCT,
                KEYFRAMED_MOTION_CONSTRUCT,
                CHARACTER_MOTION_CONSTRUCT,
                FIXED_MOTION_CONSTRUCT,
                UFLOAT8_ASSIGN,
                0x0900_001c,
                0x0900_002c,
                0x0900_000c,
            ],
        );
        e.set_global(INERTIA_RATIO, 0.800_000_011_920_929f64);
        e.set_global(KEYFRAMED_DAMPING, 1.0e6f32);
        e.register(FLOAT_MIN, |_, a| Ret {
            st0: f64::from(propagate_nan(a, f32::min)),
            ..Ret::default()
        });
        e.register(FLOAT_MAX, |_, a| Ret {
            st0: f64::from(propagate_nan(a, f32::max)),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        for slot in [0x1c, 0x2c, 0xc] {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
        }
        let motion = e.mem.alloc(0x100);
        e.mem.set_u32(motion, vtable);
        // The constructors answer the motion.
        for constructor in [
            SPHERE_MOTION_CONSTRUCT,
            BOX_MOTION_CONSTRUCT,
            MOTION_TYPE_SIX_CONSTRUCT,
            KEYFRAMED_MOTION_CONSTRUCT,
            CHARACTER_MOTION_CONSTRUCT,
            FIXED_MOTION_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(constructor, move |_, a| {
                sink.borrow_mut().push((constructor, a.to_vec()));
                Ret {
                    eax: motion,
                    ..Ret::default()
                }
            });
        }
        (motion, log)
    }

    /// `op` of the two floats, NaN when either is.
    fn propagate_nan(a: &[u32], op: fn(f32, f32) -> f32) -> f32 {
        let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
        if x.is_nan() || y.is_nan() {
            f32::NAN
        } else {
            op(x, y)
        }
    }

    fn inertia(e: &mut Engine, diagonal: [f32; 3]) -> u32 {
        let matrix = e.mem.alloc(0x30);
        for (i, v) in diagonal.iter().enumerate() {
            e.mem.set_f32(matrix + 0x14 * i as u32, *v);
        }
        matrix
    }

    #[allow(clippy::too_many_arguments)]
    fn create(e: &mut Engine, motion_type: u32, matrix: u32, buffer: u32) {
        e.call(
            0x00ca_8240,
            &args![
                motion_type,
                1u32,
                2u32,
                3.5f32,
                matrix,
                0x55u32,
                0.25f32,
                0.5f32,
                buffer
            ],
        );
    }

    #[test]
    fn dynamic_motion_picks_sphere_or_box_from_the_inertia_diagonal() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let similar = inertia(&mut e, [1.0, 0.9, 1.0]);
        create(&mut e, 1, similar, buffer);
        assert_eq!(
            calls(&log, SPHERE_MOTION_CONSTRUCT),
            vec![vec![buffer, 1, 2]]
        );
        let lopsided = inertia(&mut e, [1.0, 0.5, 1.0]);
        create(&mut e, 1, lopsided, buffer);
        assert_eq!(calls(&log, BOX_MOTION_CONSTRUCT), vec![vec![buffer, 1, 2]]);
        // Not a number: unordered, so box.
        let unordered = inertia(&mut e, [f32::NAN, 1.0, 1.0]);
        create(&mut e, 1, unordered, buffer);
        assert_eq!(calls(&log, BOX_MOTION_CONSTRUCT).len(), 2);
        // The set-up calls of the first creation.
        assert_eq!(calls(&log, 0x0900_001c)[0], vec![motion, similar]);
        assert_eq!(calls(&log, 0x0900_002c)[0], vec![motion, 0x55]);
        assert_eq!(calls(&log, 0x0900_000c)[0], vec![motion, 3.5f32.to_bits()]);
        assert_eq!(calls(&log, UFLOAT8_ASSIGN)[0][0], motion + 0xbc,);
    }

    #[test]
    fn dynamic_motion_by_type() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let matrix = inertia(&mut e, [1.0, 1.0, 1.0]);
        for (motion_type, constructor) in [
            (2, SPHERE_MOTION_CONSTRUCT),
            (3, BOX_MOTION_CONSTRUCT),
            (6, MOTION_TYPE_SIX_CONSTRUCT),
            (7, CHARACTER_MOTION_CONSTRUCT),
            (5, FIXED_MOTION_CONSTRUCT),
            (99, FIXED_MOTION_CONSTRUCT),
            (0, FIXED_MOTION_CONSTRUCT),
        ] {
            let before = calls(&log, constructor).len();
            create(&mut e, motion_type, matrix, buffer);
            assert_eq!(
                calls(&log, constructor).len(),
                before + 1,
                "type {motion_type}"
            );
        }
        // The two damping values reach the hkUFloat8s.
        let assigned = calls(&log, UFLOAT8_ASSIGN);
        assert_eq!(assigned[0][0], motion + 0xbc);
        assert_eq!(assigned[1][0], motion + 0xbd);
    }

    #[test]
    fn dynamic_keyframed_motion_forces_the_damping_and_skips_the_setup() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let matrix = inertia(&mut e, [1.0, 1.0, 1.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(UFLOAT8_ASSIGN, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.f32(a[1])));
            Ret::default()
        });
        create(&mut e, 4, matrix, buffer);
        assert_eq!(
            calls(&log, KEYFRAMED_MOTION_CONSTRUCT),
            vec![vec![buffer, 1, 2]]
        );
        assert!(calls(&log, 0x0900_001c).is_empty());
        assert_eq!(
            *seen.borrow(),
            vec![(motion + 0xbc, 1.0e6), (motion + 0xbd, 1.0e6)]
        );
    }
}
