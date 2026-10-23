//! `NiParticle/nipsyscollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c327e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27e8.
pub fn ni_p_sys_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27e8
}

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a5d450` (unit `niobject.cpp`; two words: the clone, the cloning
/// process): the `NiObject` part of the member copy.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::ProcessClone` (one word).
const OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `00a89af0` (unnamed, one word, a bare `RET 4`): the `NiObject` link step.
const OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiObject::LoadBinary` (one word).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a5d3a0` (unit `niobject.cpp`, no words): the `NiObject` constructor.
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `00a53c30` (unnamed, on the cloning process's map; two words: the key, the
/// result slot): the clone of an object, `bool` in `AL`.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (unit `nistream.cpp`, no words): reads one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// The `NiPSysCollider` vtable.
const COLLIDER_VTABLE: u32 = 0x010c_0c4c;
/// The vector the constructor copies into the collision point (`011f426c`).
const ORIGIN: u32 = 0x011f_426c;

layout! {
    /// `NiPSysCollider` (Xbox PDB), 0x2C bytes.
    pub struct NiPSysCollider: 0x2C {
        /// `m_fBounce` (Xbox PDB), as bits.
        0x08 m_fBounce: u32,
        /// `m_bSpawnOnCollide` (Xbox PDB).
        0x0C m_bSpawnOnCollide: u8,
        /// `m_bDieOnCollide` (Xbox PDB).
        0x0D m_bDieOnCollide: u8,
        /// `m_pkSpawnModifier` (Xbox PDB).
        0x10 m_pkSpawnModifier: u32,
        /// `m_kCollisionPoint` (Xbox PDB), three floats.
        0x14 m_kCollisionPoint: u32,
        /// `m_fCollisionTime` (Xbox PDB), as bits.
        0x20 m_fCollisionTime: u32,
        /// `m_pkManager` (Xbox PDB).
        0x24 m_pkManager: u32,
        /// `m_spNext` (Xbox PDB): `NiPointer<NiPSysCollider>`.
        0x28 m_spNext: u32,
    }
}

/// Stores `value` in the `NiPointer` at `slot` when it differs: the old
/// object released (`DeleteThis`, vtable `+4`, at a zero count), the new
/// one's count incremented.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old != value {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 4, &[]);
        }
        e.mem.set_u32(slot, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

// Translated from 00c32510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the clone, the cloning
/// process; the `CopyMembers` shape): the `NiObject` part (`00a5d450`), the
/// bounce (`FLD`/`FSTP`, quieting a signalling NaN) and the two flags, then,
/// when there is a next collider, its `CreateClone` (vtable `+0x48`) goes into
/// the clone's `m_spNext`.
pub fn fn_00c32510(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    clone: Ptr<NiPSysCollider>,
    process: Ptr,
) {
    e.call(OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    let bounce = x87_quiet_bits(e.get(this, NiPSysCollider::m_fBounce));
    e.set(clone, NiPSysCollider::m_fBounce, bounce);
    let spawn = e.get(this, NiPSysCollider::m_bSpawnOnCollide);
    e.set(clone, NiPSysCollider::m_bSpawnOnCollide, spawn);
    let die = e.get(this, NiPSysCollider::m_bDieOnCollide);
    e.set(clone, NiPSysCollider::m_bDieOnCollide, die);
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next == 0 {
        return;
    }
    let copy = e.vcall(next, 0x48, &args![process]).u32();
    assign_pointer(e, clone.addr() + 0x28, copy);
}

// Translated from 00c32580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::ProcessClone` (Xbox PDB): the `NiObject` part; then this
/// collider's clone is looked up in the process's map (`00a53c30`, the result
/// slot is the argument slot, set to 0 first) and its spawn modifier and
/// manager become their clones when the map has them, else stay; then the
/// next collider's `ProcessClone` (vtable `+0x68`).
pub fn ni_p_sys_collider_process_clone(e: &mut Engine, this: Ptr<NiPSysCollider>, process: Ptr) {
    e.call(OBJECT_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = e.mem.u32(slot.addr());
        for offset in [0x10u32, 0x24] {
            let object = e.mem.u32(this.addr() + offset);
            if object != 0 {
                let map = e.mem.u32(process.addr());
                let value = if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
                    e.mem.u32(slot.addr())
                } else {
                    e.mem.u32(this.addr() + offset)
                };
                e.mem.set_u32(clone.wrapping_add(offset), value);
            }
        }
    });
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next != 0 {
        e.vcall(next, 0x68, &args![process]);
    }
}

// Translated from 00c32610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::LinkObject` (Xbox PDB): the `NiObject` link step
/// (`00a89af0`), then three linked objects: the spawn modifier, the manager
/// and the next collider (a `NiPointer`).
pub fn ni_p_sys_collider_link_object(e: &mut Engine, this: Ptr<NiPSysCollider>, stream: Ptr) {
    e.call(OBJECT_LINK_OBJECT, &args![this, stream]);
    let spawn = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysCollider::m_pkSpawnModifier, spawn);
    let manager = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysCollider::m_pkManager, manager);
    let next = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x28, next);
}

// Translated from 00c32780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::NiPSysCollider` (Xbox PDB; `thiscall`, four words: the
/// bounce, spawn-on-collide, die-on-collide, the spawn modifier): the
/// `NiObject` constructor, the fields (the bounce through `FLD`/`FSTP`), the
/// vtable, the collision point from `011f426c`, time 0, no manager, no next.
pub fn ni_p_sys_collider_ni_p_sys_collider(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    bounce: f32,
    spawn_on_collide: u8,
    die_on_collide: u8,
    spawn_modifier: u32,
) -> Ptr<NiPSysCollider> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.set(
        this,
        NiPSysCollider::m_fBounce,
        x87_quiet_bits(bounce.to_bits()),
    );
    e.set(this, NiPSysCollider::m_bSpawnOnCollide, spawn_on_collide);
    e.mem.set_u32(this.addr(), COLLIDER_VTABLE);
    e.set(this, NiPSysCollider::m_bDieOnCollide, die_on_collide);
    e.set(this, NiPSysCollider::m_pkSpawnModifier, spawn_modifier);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(this.addr() + 0x14 + 4 * i, word);
    }
    e.set(this, NiPSysCollider::m_fCollisionTime, 0);
    e.set(this, NiPSysCollider::m_pkManager, 0);
    e.set(this, NiPSysCollider::m_spNext, 0);
    this
}

// Translated from 00c32840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::LoadBinary` (Xbox PDB): the `NiObject` part, the bounce
/// (four bytes), then two one-byte booleans (the first read into the
/// argument slot, whose low byte holds the stream's until the read, the
/// second into a byte of the frame) stored as `byte != 0`, then three link
/// IDs (`00a645b0`). Each read goes through the stream's input file (`+0x24c`,
/// its read function `+8`, `cdecl (file, dest, size, &count, 1)`, the result
/// added to its position `+4`); the count word is a local.
pub fn ni_p_sys_collider_load_binary(e: &mut Engine, this: Ptr<NiPSysCollider>, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        // frame + 0: the argument slot; + 4: the frame's byte (at +7) and
        // + 8: the count word, as the exe lays them out.
        let f = frame.addr();
        e.mem.set_u32(f, stream.addr());
        let read = |e: &mut Engine, dest: u32, size: u32| {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let function = e.mem.u32(file + 8);
            e.mem.set_u32(f + 8, size);
            let got = e
                .call(function, &args![file, dest, size, f + 8, 1u32])
                .u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(got));
        };
        read(e, this.addr() + 8, 4);
        read(e, f, 1);
        let spawn = u8::from(e.mem.u8(f) != 0);
        e.set(this, NiPSysCollider::m_bSpawnOnCollide, spawn);
        read(e, f + 7, 1);
        let die = u8::from(e.mem.u8(f + 7) != 0);
        e.set(this, NiPSysCollider::m_bDieOnCollide, die);
    });
    for _ in 0..3 {
        e.call(READ_LINK_ID, &args![stream]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c327e0, ni_p_sys_collider_get_r_t_t_i() -> u32),
        entry!(
            0x00c3_2510,
            fn_00c32510(Ptr<NiPSysCollider>, Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2580,
            ni_p_sys_collider_process_clone(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2610,
            ni_p_sys_collider_link_object(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2780,
            ni_p_sys_collider_ni_p_sys_collider(
                Ptr<NiPSysCollider>,
                f32,
                u8,
                u8,
                u32,
            ) -> Ptr<NiPSysCollider>
        ),
        entry!(
            0x00c3_2840,
            ni_p_sys_collider_load_binary(Ptr<NiPSysCollider>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_27e0, &args![]).u32(), 0x0120_27e8);
    }
}

#[cfg(test)]
mod collider_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn with_refcounts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
    }

    const DELETE_THIS: u32 = 0x0100_0004;
    const CREATE_CLONE: u32 = 0x0100_0048;
    const PROCESS_CLONE: u32 = 0x0100_0068;

    fn collider(e: &mut Engine, count: u32) -> Ptr<NiPSysCollider> {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.mem.set_u32(vtable + 0x48, CREATE_CLONE);
        e.mem.set_u32(vtable + 0x68, PROCESS_CLONE);
        let object: Ptr<NiPSysCollider> = e.new_object();
        e.mem.set_u32(object.addr(), vtable);
        e.mem.set_u32(object.addr() + 4, count);
        object
    }

    #[test]
    fn copy_members_copies_the_fields_and_clones_the_next() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = collider(&mut e, 1);
        let clone = collider(&mut e, 1);
        let copy = collider(&mut e, 0);
        e.register_double(CREATE_CLONE, move |_, _| word(copy.addr()));
        e.set(this, NiPSysCollider::m_fBounce, 0xff80_0001);
        e.set(this, NiPSysCollider::m_bSpawnOnCollide, 1);
        e.set(this, NiPSysCollider::m_bDieOnCollide, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2510, &args![this, clone, 0x7000u32]);
        assert_eq!(e.get(clone, NiPSysCollider::m_fBounce), 0xffc0_0001);
        assert_eq!(e.get(clone, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(clone, NiPSysCollider::m_bDieOnCollide), 1);
        assert!(calls_to(&e, CREATE_CLONE).is_empty());
        let next = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_spNext, next.addr());
        e.call(0x00c3_2510, &args![this, clone, 0x7000u32]);
        assert_eq!(calls_to(&e, CREATE_CLONE), vec![vec![next.addr(), 0x7000]]);
        assert_eq!(e.get(clone, NiPSysCollider::m_spNext), copy.addr());
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS)[0],
            vec![this.addr(), clone.addr(), 0x7000]
        );
    }

    #[test]
    fn process_clone_maps_the_spawn_modifier_and_manager() {
        let mut e = Engine::new();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(PROCESS_CLONE, |_, _| Ret::default());
        let this = collider(&mut e, 1);
        let clone = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_pkSpawnModifier, 0x4440);
        e.set(this, NiPSysCollider::m_pkManager, 0x5550);
        let (key, value) = (this.addr(), clone.addr());
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = match a[1] {
                k if k == key => value,
                0x4440 => 0x4441,
                _ => return word(0),
            };
            e.mem.set_u32(a[2], found);
            word(1)
        });
        let process = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2580, &args![this, process]);
        assert_eq!(e.get(clone, NiPSysCollider::m_pkSpawnModifier), 0x4441);
        assert_eq!(e.get(clone, NiPSysCollider::m_pkManager), 0x5550);
        assert!(calls_to(&e, PROCESS_CLONE).is_empty());
        let next = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_spNext, next.addr());
        e.set(this, NiPSysCollider::m_pkManager, 0);
        e.call(0x00c3_2580, &args![this, process]);
        assert_eq!(
            calls_to(&e, PROCESS_CLONE),
            vec![vec![next.addr(), process]]
        );
        assert_eq!(calls_to(&e, CLONE_MAP_LOOKUP).len(), 5);
    }

    #[test]
    fn link_object_takes_three_linked_objects() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(OBJECT_LINK_OBJECT, |_, _| Ret::default());
        let next = collider(&mut e, 0);
        let mut linked = vec![0x4440, 0x5550, next.addr()].into_iter();
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            word(linked.next().unwrap())
        });
        let this = collider(&mut e, 1);
        e.call(0x00c3_2610, &args![this, 0x6000u32]);
        assert_eq!(e.get(this, NiPSysCollider::m_pkSpawnModifier), 0x4440);
        assert_eq!(e.get(this, NiPSysCollider::m_pkManager), 0x5550);
        assert_eq!(e.get(this, NiPSysCollider::m_spNext), next.addr());
        assert_eq!(e.mem.u32(next.addr() + 4), 1);
    }

    #[test]
    fn constructor_sets_the_fields() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.map(ORIGIN, 0xc);
        e.mem.set_f32(ORIGIN + 8, 3.0);
        let this: Ptr<NiPSysCollider> = e.new_object();
        let r = e
            .call(0x00c3_2780, &args![this, 0.5f32, 0x101u32, 0u32, 0x4440u32])
            .u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), COLLIDER_VTABLE);
        assert_eq!(e.get(this, NiPSysCollider::m_fBounce), 0.5f32.to_bits());
        assert_eq!(e.get(this, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(this, NiPSysCollider::m_bDieOnCollide), 0);
        assert_eq!(e.get(this, NiPSysCollider::m_pkSpawnModifier), 0x4440);
        assert_eq!(e.mem.f32(this.addr() + 0x1c), 3.0);
    }

    #[test]
    fn load_binary_reads_the_bounce_flags_and_link_ids() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        let mut bytes = vec![2u8, 0].into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            if a[2] == 4 {
                e.mem.set_f32(a[1], 0.25);
            } else {
                e.mem.set_u8(a[1], bytes.next().unwrap());
            }
            word(a[2])
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_bDieOnCollide, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2840, &args![this, stream]);
        assert_eq!(e.mem.f32(this.addr() + 8), 0.25);
        assert_eq!(e.get(this, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(this, NiPSysCollider::m_bDieOnCollide), 0);
        assert_eq!(e.mem.u32(file + 4), 6);
        assert_eq!(calls_to(&e, READ_LINK_ID).len(), 3);
    }
}
