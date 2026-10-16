//! `Havok SDK/hkpentitycallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every function takes the `hkpEntity` as its only (cdecl) argument. An entity keeps its
//! `hkpEntityListener`s in `m_extendedListeners` (+0x214, Xbox PDB `hkpEntity::
//! ExtendedListeners`), whose `m_entityListeners` `hkSmallArray` has its data pointer at +8
//! and its 16-bit size at +0xC; the listeners are called from the last to the first, null
//! ones skipped. The listener callbacks are vtable slots `0x04` (`entityAddedCallback`),
//! `0x08` (`entityRemovedCallback`) and `0x10` (`entitySetMotionTypeCallback`, Xbox PDB
//! `hkpEntityListener`).
//!
//! Each callback is wrapped in Havok's profiling records (`RDTSC` time stamps appended to the
//! thread's monitor stream, found with `TlsGetValue` of the index at `0126835c`); the engine
//! has no model of `RDTSC`, so the host clock stands in for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtentAddCb"`, `"TtentRemCb"`, `"TtsetMotionTypeCb"` (the `Tt` is the
/// record type) and the end label.
const LABEL_ADDED: u32 = 0x010c_c9dc;
const LABEL_REMOVED: u32 = 0x010c_c9e8;
const LABEL_SET_MOTION_TYPE: u32 = 0x010c_ca04;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpEntity::sortConstraintsSlavesDeterministically` (Xbox PDB, `00c9c860`, `this` only).
const SORT_CONSTRAINTS_SLAVES: u32 = 0x00c9_c860;

layout! {
    /// `hkpEntity` (Xbox PDB), as far as these functions read it (PC offsets).
    pub struct HkpEntityCallbacks: 0x220 {
        /// `m_constraintsMaster` (Xbox PDB, `hkSmallArray<hkConstraintInternal>`): data. The
        /// first word of an element is its `hkpConstraintInstance*`.
        0x0ac m_constraintsMaster: Ptr,
        /// Size of `m_constraintsMaster` (16 bits).
        0x0b0 m_constraintsMasterSize: u16,
        /// `m_constraintsSlave` (Xbox PDB, `hkArray<hkpConstraintInstance*>`): data.
        0x0b4 m_constraintsSlave: Ptr,
        /// Size of `m_constraintsSlave`.
        0x0b8 m_constraintsSlaveSize: i32,
        /// `m_actions` (Xbox PDB, `hkSmallArray<hkpAction*>`): data.
        0x208 m_actions: Ptr,
        /// Size of `m_actions` (16 bits).
        0x20c m_actionsSize: u16,
        /// `m_extendedListeners` (Xbox PDB): `ExtendedListeners*` or null.
        0x214 m_extendedListeners: Ptr,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cffa00,
            hkp_entity_callback_util_fire_entity_added(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cffab0,
            hkp_entity_callback_util_fire_entity_removed(Ptr<HkpEntityCallbacks>)
        ),
        entry!(
            0x00cffda0,
            hkp_entity_callback_util_fire_entity_set_motion_type(Ptr<HkpEntityCallbacks>)
        ),
    ]
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Calls vtable slot `slot` of every entity listener, last first, each between a record
/// `label` and an end record.
fn fire_listeners(e: &mut Engine, entity: Ptr<HkpEntityCallbacks>, label: u32, slot: u32) {
    let extended = e
        .get(entity, HkpEntityCallbacks::m_extendedListeners)
        .addr();
    if extended == 0 {
        return;
    }
    let mut index = i32::from(e.mem.u16(extended + 0xc)) - 1;
    while index >= 0 {
        let data = e.mem.u32(extended + 8);
        let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if listener != 0 {
            profile_record(e, label);
            e.vcall(listener, slot, &args![entity]);
            profile_record(e, LABEL_END);
        }
        index -= 1;
    }
}

/// One element of a list of objects that all get the same callback while the list is not
/// empty: each call is expected to remove itself from the list. `first` reads the object
/// at the start of the list, `pending` says whether the list still has any.
fn drain_callbacks(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
    slot: u32,
    pending: impl Fn(&Engine) -> bool,
    first: impl Fn(&Engine) -> u32,
) {
    while pending(e) {
        profile_record(e, LABEL_REMOVED);
        let object = first(e);
        e.vcall(object, slot, &args![entity]);
        profile_record(e, LABEL_END);
    }
}

// Translated from 00cffa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityAdded` (Xbox PDB): `entityAddedCallback` (slot `0x04`) of
/// every entity listener.
pub fn hkp_entity_callback_util_fire_entity_added(e: &mut Engine, entity: Ptr<HkpEntityCallbacks>) {
    fire_listeners(e, entity, LABEL_ADDED, 0x4);
}

// Translated from 00cffda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntitySetMotionType` (Xbox PDB): `entitySetMotionTypeCallback`
/// (slot `0x10`) of every entity listener.
pub fn hkp_entity_callback_util_fire_entity_set_motion_type(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_SET_MOTION_TYPE, 0x10);
}

// Translated from 00cffab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityCallbackUtil::fireEntityRemoved` (Xbox PDB): `entityRemovedCallback` (slot
/// `0x08`) of every entity listener, then, each between `"TtentRemCb"` records:
///
/// - while the master constraint list (16-bit size at +0xB0) is not empty, slot `0x10`
///   (`entityRemovedCallback` of `hkpConstraintInstance`, Xbox PDB) of its first constraint,
///   which removes it from the list;
/// - `hkpEntity::sortConstraintsSlavesDeterministically` (`00c9c860`);
/// - the same for the slave constraint list (32-bit size at +0xB8);
/// - while the action list (16-bit size at +0x20C) is not empty, slot `0x18`
///   (`entityRemovedCallback` of `hkpAction`, Xbox PDB) of its first action.
pub fn hkp_entity_callback_util_fire_entity_removed(
    e: &mut Engine,
    entity: Ptr<HkpEntityCallbacks>,
) {
    fire_listeners(e, entity, LABEL_REMOVED, 0x8);
    let element = |e: &Engine, data: Ptr| e.mem.u32(data.addr());
    drain_callbacks(
        e,
        entity,
        0x10,
        |e| e.get(entity, HkpEntityCallbacks::m_constraintsMasterSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_constraintsMaster)),
    );
    e.call(SORT_CONSTRAINTS_SLAVES, &args![entity]);
    drain_callbacks(
        e,
        entity,
        0x10,
        |e| e.get(entity, HkpEntityCallbacks::m_constraintsSlaveSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_constraintsSlave)),
    );
    drain_callbacks(
        e,
        entity,
        0x18,
        |e| e.get(entity, HkpEntityCallbacks::m_actionsSize) != 0,
        |e| element(e, e.get(entity, HkpEntityCallbacks::m_actions)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTENER_VTABLE: u32 = 0x0200_0000;
    const OBJECT_VTABLE: u32 = 0x0200_0040;
    const ON_ADDED: u32 = 0x0200_0100;
    const ON_REMOVED: u32 = 0x0200_0104;
    const ON_MOTION: u32 = 0x0200_0108;
    const ON_CONSTRAINT: u32 = 0x0200_010c;
    const ON_ACTION: u32 = 0x0200_0114;

    struct Rig {
        e: Engine,
        entity: Ptr<HkpEntityCallbacks>,
        stream: u32,
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.put_vtable(LISTENER_VTABLE, &[0, ON_ADDED, ON_REMOVED, 0, ON_MOTION]);
        e.put_vtable(OBJECT_VTABLE, &[0, 0, 0, 0, ON_CONSTRAINT, 0, ON_ACTION]);
        for addr in [ON_ADDED, ON_REMOVED, ON_MOTION, SORT_CONSTRAINTS_SLAVES] {
            e.register(addr, |_, _| Ret::default());
        }
        // Each removal callback removes its object from the entity's list, as the real ones
        // do: the constraint slot serves the master list while it has entries, then the slaves.
        e.register(ON_CONSTRAINT, |e, a| {
            let masters = e.mem.u16(a[1] + 0xb0);
            if masters > 0 {
                e.mem.set_u16(a[1] + 0xb0, masters - 1);
            } else {
                let slaves = e.mem.i32(a[1] + 0xb8);
                e.mem.set_i32(a[1] + 0xb8, slaves - 1);
            }
            Ret::default()
        });
        e.register(ON_ACTION, |e, a| {
            let size = e.mem.u16(a[1] + 0x20c);
            e.mem.set_u16(a[1] + 0x20c, size - 1);
            Ret::default()
        });
        let stream = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x1f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        let entity = e.new_object::<HkpEntityCallbacks>();
        Rig { e, entity, stream }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Gives the entity three listeners (the middle one null).
    fn add_listeners(r: &mut Rig) -> [u32; 2] {
        let listeners = [r.e.mem.alloc(4), r.e.mem.alloc(4)];
        for l in listeners {
            r.e.mem.set_u32(l, LISTENER_VTABLE);
        }
        let extended = r.e.mem.alloc(0x10);
        let data = r.e.mem.alloc(12);
        r.e.mem.set_u32(data, listeners[0]);
        r.e.mem.set_u32(data + 8, listeners[1]);
        r.e.mem.set_u32(extended + 8, data);
        r.e.mem.set_u16(extended + 0xc, 3);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_extendedListeners,
            Ptr::new(extended),
        );
        listeners
    }

    fn records(r: &Rig) -> Vec<u32> {
        let cursor = r.e.mem.u32(r.stream + 4);
        (r.stream + 0x40..cursor)
            .step_by(12)
            .map(|at| r.e.mem.u32(at))
            .collect()
    }

    #[test]
    fn added_and_motion_type_call_listeners_last_first_skipping_null() {
        let mut r = rig();
        let listeners = add_listeners(&mut r);
        r.e.call(0x00cf_fa00, &args![r.entity]);
        let seen = calls(&r.e, ON_ADDED);
        assert_eq!(
            seen,
            vec![
                vec![listeners[1], r.entity.addr()],
                vec![listeners[0], r.entity.addr()]
            ]
        );
        assert_eq!(
            records(&r),
            vec![LABEL_ADDED, LABEL_END, LABEL_ADDED, LABEL_END]
        );
        r.e.call(0x00cf_fda0, &args![r.entity]);
        assert_eq!(calls(&r.e, ON_MOTION).len(), 2);
        assert_eq!(records(&r)[4], LABEL_SET_MOTION_TYPE);
    }

    #[test]
    fn no_extended_listeners_means_no_calls() {
        let mut r = rig();
        r.e.call(0x00cf_fa00, &args![r.entity]);
        assert!(calls(&r.e, ON_ADDED).is_empty());
        assert!(records(&r).is_empty());
    }

    #[test]
    fn removed_drains_constraints_then_sorts_then_actions() {
        let mut r = rig();
        let listeners = add_listeners(&mut r);
        // Two master constraints (the pointer array holds the instances), one action, and
        // slave constraints that remove themselves through their own slot.
        let instance = r.e.mem.alloc(4);
        r.e.mem.set_u32(instance, OBJECT_VTABLE);
        let masters = r.e.mem.alloc(8);
        r.e.mem.set_u32(masters, instance);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_constraintsMaster,
            Ptr::new(masters),
        );
        r.e.set(r.entity, HkpEntityCallbacks::m_constraintsMasterSize, 2);
        let action = r.e.mem.alloc(4);
        r.e.mem.set_u32(action, OBJECT_VTABLE);
        let actions = r.e.mem.alloc(4);
        r.e.mem.set_u32(actions, action);
        r.e.set(r.entity, HkpEntityCallbacks::m_actions, Ptr::new(actions));
        r.e.set(r.entity, HkpEntityCallbacks::m_actionsSize, 1);
        // The slaves: one entry, called through the constraint slot once the master list is
        // empty (the double then lowers the slave count).
        let slave = r.e.mem.alloc(4);
        r.e.mem.set_u32(slave, OBJECT_VTABLE);
        let slaves = r.e.mem.alloc(4);
        r.e.mem.set_u32(slaves, slave);
        r.e.set(
            r.entity,
            HkpEntityCallbacks::m_constraintsSlave,
            Ptr::new(slaves),
        );
        r.e.set(r.entity, HkpEntityCallbacks::m_constraintsSlaveSize, 1);
        let entity_word = r.entity.addr();
        r.e.call(0x00cf_fab0, &args![r.entity]);

        assert_eq!(calls(&r.e, ON_REMOVED).len(), 2);
        assert_eq!(calls(&r.e, ON_REMOVED)[0], vec![listeners[1], entity_word]);
        // Two master callbacks, then the sort, then one slave callback, then the action.
        let order: Vec<u32> =
            r.e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .filter(|a| [ON_CONSTRAINT, SORT_CONSTRAINTS_SLAVES, ON_ACTION].contains(a))
                .collect();
        assert_eq!(
            order,
            vec![
                ON_CONSTRAINT,
                ON_CONSTRAINT,
                SORT_CONSTRAINTS_SLAVES,
                ON_CONSTRAINT,
                ON_ACTION
            ]
        );
        assert_eq!(calls(&r.e, ON_CONSTRAINT)[0], vec![instance, entity_word]);
        assert_eq!(calls(&r.e, ON_ACTION), vec![vec![action, entity_word]]);
        assert_eq!(r.e.get(r.entity, HkpEntityCallbacks::m_actionsSize), 0);
        assert_eq!(
            r.e.get(r.entity, HkpEntityCallbacks::m_constraintsSlaveSize),
            0
        );
        // Records: two listener calls, then 2 + 1 constraint calls and 1 action call.
        assert_eq!(records(&r).len(), 2 * (2 + 3 + 1));
    }
}
