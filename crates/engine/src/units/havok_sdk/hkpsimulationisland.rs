//! `Havok SDK/hkpsimulationisland.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSimulationIsland` (0x6c bytes on the Xbox, offsets checked against the
//! PC constructor `00d07830`) keeps its actions in an `hkArray` at +0x38 and
//! its entities in an `hkInplaceArray` at +0x48; an `hkArray` is the data
//! pointer, the size and a capacity word whose low 30 bits are the capacity
//! (bit 31 says the data is not owned). A full array is grown through
//! `hkArrayUtil::_reserveMore` (`00c90940`) with the memory router's
//! allocator (`00c85750`, field +0x10).
//!
//! The island's flag byte at +0x25 holds four 2-bit fields (Xbox PDB:
//! `m_splitCheckRequested`, `m_sparseEnabled`, `m_actionListCleanupNeeded`,
//! `m_allowIslandLocking`); the PDB gives their bit positions for the Xbox
//! (counting from the high end), and the PC code's masks are consistent with
//! the fields counting from bit 0 in declaration order, which is how the
//! translations below name them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (cdecl: allocator, the `hkArray`, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpSimulationIsland`'s vtable (the constructor stores it).
const ISLAND_VTABLE: u32 = 0x010c_cf28;
/// `float` stored in `m_timeOfDeactivation` by the constructor.
const TIME_OF_DEACTIVATION_INITIAL: u32 = 0x0104_eb20;

layout! {
    /// `hkpSimulationIsland` (Xbox PDB), 0x6c bytes.
    pub struct HkpSimulationIsland: 0x6c {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_constraintInfo.m_maxSizeOfSchema` (Xbox PDB).
        0x08 m_maxSizeOfSchema: i32,
        /// `m_constraintInfo.m_sizeOfSchemas` (Xbox PDB).
        0x0C m_sizeOfSchemas: i32,
        /// `m_constraintInfo.m_numSolverResults` (Xbox PDB).
        0x10 m_numSolverResults: i32,
        /// `m_constraintInfo.m_numSolverElemTemps` (Xbox PDB).
        0x14 m_numSolverElemTemps: i32,
        /// `m_world` (Xbox PDB): `hkpWorld*`.
        0x18 m_world: Ptr,
        /// `m_numConstraints` (Xbox PDB).
        0x1C m_numConstraints: i32,
        /// `m_storageIndex` (Xbox PDB).
        0x20 m_storageIndex: u16,
        /// `m_dirtyListIndex` (Xbox PDB).
        0x22 m_dirtyListIndex: u16,
        /// `m_splitCheckFrameCounter` (Xbox PDB).
        0x24 m_splitCheckFrameCounter: u8,
        /// The first flag byte: four 2-bit fields (see the module notes).
        0x25 flags: u8,
        /// The second flag byte (the constructor stores 5).
        0x26 flags2: u8,
        /// `m_multiThreadCheck.m_threadId` (Xbox PDB).
        0x28 m_threadId: u32,
        /// `m_multiThreadCheck.m_markCount` (Xbox PDB).
        0x2C m_markCount: u16,
        /// `m_timeSinceLastHighFrequencyCheck` (Xbox PDB).
        0x30 m_timeSinceLastHighFrequencyCheck: f32,
        /// `m_timeSinceLastLowFrequencyCheck` (Xbox PDB).
        0x34 m_timeSinceLastLowFrequencyCheck: f32,
        /// `m_actions` (Xbox PDB): the `hkArray` data pointer.
        0x38 m_actions: Ptr,
        /// Size of `m_actions`.
        0x3C m_actionsSize: i32,
        /// Capacity word of `m_actions`.
        0x40 m_actionsCapacity: u32,
        /// `m_timeOfDeactivation` (Xbox PDB).
        0x44 m_timeOfDeactivation: f32,
        /// `m_entities` (Xbox PDB): the `hkInplaceArray` data pointer.
        0x48 m_entities: Ptr,
        /// Size of `m_entities`.
        0x4C m_entitiesSize: i32,
        /// Capacity word of `m_entities`.
        0x50 m_entitiesCapacity: u32,
        // +0x54: the inplace storage of `m_entities` (one pointer).
        /// `m_agentTrack.m_bytesUsedInLastSector` (Xbox PDB).
        0x58 m_bytesUsedInLastSector: u32,
        /// `m_agentTrack.m_sectors` (Xbox PDB): the `hkInplaceArray` data pointer.
        0x5C m_sectors: Ptr,
        /// Size of `m_sectors`.
        0x60 m_sectorsSize: i32,
        /// Capacity word of `m_sectors`.
        0x64 m_sectorsCapacity: u32,
        // +0x68: the inplace storage of `m_sectors` (one pointer).
    }
}

/// `hkpEntity::m_storageIndex` (Xbox PDB), the entity's index in its island's
/// entity array (u16).
const ENTITY_STORAGE_INDEX: u32 = 0xa8;
/// `hkpEntity::m_simulationIsland` (Xbox PDB).
const ENTITY_ISLAND: u32 = 0xcc;
/// `hkpAction::m_island` (Xbox PDB).
const ACTION_ISLAND: u32 = 0x0c;

/// Appends `item` to the `hkArray` (data, size, capacity word) at `array`,
/// growing it through `_reserveMore` when the size equals the capacity.
fn array_push(e: &mut Engine, array: u32, item: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), item);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d07120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. An empty function (a single `RET`); callers pass an
/// island or a world in `ECX` and it does nothing with it.
pub fn fn_00d07120(_e: &mut Engine, _this: Ptr) {}

// Translated from 00d07130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Detaches `action` from the island: the first slot of
/// `m_actions` that holds it is set to null (the array keeps its size; a
/// missing action gives index -1, whose slot lies before the data and is
/// written all the same), `action.m_island` is cleared, and the flag byte
/// gets bits 0 and 4 set and bits 1 and 5 cleared (the first bit of the
/// `m_splitCheckRequested` and `m_actionListCleanupNeeded` fields, by the
/// module notes).
pub fn fn_00d07130(e: &mut Engine, this: Ptr<HkpSimulationIsland>, action: Ptr) {
    let size = e.get(this, HkpSimulationIsland::m_actionsSize);
    let data = e.get(this, HkpSimulationIsland::m_actions).addr();
    let mut index: i32 = -1;
    let mut candidate = 0;
    while candidate < size {
        if e.mem
            .u32(data.wrapping_add((candidate as u32).wrapping_mul(4)))
            == action.addr()
        {
            index = candidate;
            break;
        }
        candidate += 1;
    }
    let data = e.get(this, HkpSimulationIsland::m_actions).addr();
    e.mem
        .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), 0);
    e.mem.set_u32(action.addr() + ACTION_ISLAND, 0);
    let flags = e.get(this, HkpSimulationIsland::flags);
    e.set(this, HkpSimulationIsland::flags, (flags & 0xdd) | 0x11);
}

// Translated from 00d07180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Merges the constraint info (`hkpConstraintInfo` at +8)
/// of `other` into this island's: the larger `m_maxSizeOfSchema` (signed),
/// and the sums of `m_sizeOfSchemas`, `m_numSolverResults` and
/// `m_numSolverElemTemps`.
pub fn fn_00d07180(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    other: Ptr<HkpSimulationIsland>,
) {
    let other_max = e.get(other, HkpSimulationIsland::m_maxSizeOfSchema);
    let own_max = e.get(this, HkpSimulationIsland::m_maxSizeOfSchema);
    e.set(
        this,
        HkpSimulationIsland::m_maxSizeOfSchema,
        if own_max > other_max {
            own_max
        } else {
            other_max
        },
    );
    for field in [
        HkpSimulationIsland::m_sizeOfSchemas,
        HkpSimulationIsland::m_numSolverResults,
        HkpSimulationIsland::m_numSolverElemTemps,
    ] {
        let sum = e.get(this, field).wrapping_add(e.get(other, field));
        e.set(this, field, sum);
    }
}

// Translated from 00d071b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Takes `entity` out of the island's entity array by
/// swap-remove: the last entity moves into the slot at `entity.m_storageIndex`
/// and takes that index, the size drops by one, the entity's island is
/// cleared and its storage index becomes 0xffff. The flag byte gets bit 0 set
/// and bit 1 cleared.
pub fn fn_00d071b0(e: &mut Engine, this: Ptr<HkpSimulationIsland>, entity: Ptr) {
    let data = e.get(this, HkpSimulationIsland::m_entities).addr();
    let size = e.get(this, HkpSimulationIsland::m_entitiesSize);
    let last = e.mem.u32(
        data.wrapping_add((size as u32).wrapping_mul(4))
            .wrapping_sub(4),
    );
    let index = u32::from(e.mem.u16(entity.addr() + ENTITY_STORAGE_INDEX));
    e.mem.set_u32(data.wrapping_add(index * 4), last);
    let index = e.mem.u16(entity.addr() + ENTITY_STORAGE_INDEX);
    let data = e.get(this, HkpSimulationIsland::m_entities).addr();
    let moved = e.mem.u32(data.wrapping_add(u32::from(index) * 4));
    e.mem.set_u16(moved + ENTITY_STORAGE_INDEX, index);
    e.set(
        this,
        HkpSimulationIsland::m_entitiesSize,
        size.wrapping_sub(1),
    );
    e.mem.set_u32(entity.addr() + ENTITY_ISLAND, 0);
    e.mem.set_u16(entity.addr() + ENTITY_STORAGE_INDEX, 0xffff);
    let flags = e.get(this, HkpSimulationIsland::flags);
    e.set(this, HkpSimulationIsland::flags, (flags & 0xfd) | 0x01);
}

// Translated from 00d07420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::internalAddEntity` (Xbox PDB): appends `entity` to
/// the entity array (growing it when full) and records the island in
/// `entity.m_simulationIsland` and the index (low 16 bits of the old size) in
/// `entity.m_storageIndex`.
pub fn hkp_simulation_island_internal_add_entity(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    entity: Ptr,
) {
    e.mem.set_u32(entity.addr() + ENTITY_ISLAND, this.addr());
    let index = e.get(this, HkpSimulationIsland::m_entitiesSize) as u16;
    e.mem.set_u16(entity.addr() + ENTITY_STORAGE_INDEX, index);
    array_push(e, this.addr() + 0x48, entity.addr());
}

// Translated from 00d07470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::addAction` (Xbox PDB): appends `action` to the action
/// array (growing it when full) and records the island in `action.m_island`.
pub fn hkp_simulation_island_add_action(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    action: Ptr,
) {
    array_push(e, this.addr() + 0x38, action.addr());
    e.mem.set_u32(action.addr() + ACTION_ISLAND, this.addr());
}

// Translated from 00d07830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulationIsland::hkpSimulationIsland` (Xbox PDB): the constructor.
/// Stores the vtable and `world`, an empty (inplace, one-element) entity
/// array and sector array, an empty action array, zeroed constraint info,
/// reference count 1, storage index 0xffff in `m_dirtyListIndex`, the thread
/// check (`m_threadId` 0xfffffff1, `m_markCount` 0x8000), zero check timers,
/// `m_timeOfDeactivation` from the global at `0104eb20`, and clears the flag
/// byte. Returns `this`.
pub fn hkp_simulation_island_hkp_simulation_island(
    e: &mut Engine,
    this: Ptr<HkpSimulationIsland>,
    world: Ptr,
) -> Ptr<HkpSimulationIsland> {
    use HkpSimulationIsland as I;
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0xcc);
    e.set(this, I::m_world, world);
    e.set(this, I::vtable, ISLAND_VTABLE);
    e.set(this, I::flags2, 5);
    e.set(this, I::m_referenceCount, 1);
    e.set(this, I::m_dirtyListIndex, 0xffff);
    e.set(this, I::m_threadId, 0xffff_fff1);
    e.set(this, I::m_markCount, 0x8000);
    e.set(this, I::m_timeSinceLastHighFrequencyCheck, 0.0);
    e.set(this, I::m_timeSinceLastLowFrequencyCheck, 0.0);
    e.set(this, I::m_actions, Ptr::NULL);
    e.set(this, I::m_actionsSize, 0);
    e.set(this, I::m_actionsCapacity, 0x8000_0000);
    let initial = x87_quiet(e.global::<f32>(TIME_OF_DEACTIVATION_INITIAL));
    e.set(this, I::m_timeOfDeactivation, initial);
    e.set(this, I::m_entitiesSize, 0);
    e.set(this, I::m_entities, Ptr::new(this.addr() + 0x54));
    e.set(this, I::m_entitiesCapacity, 0x8000_0001);
    e.set(this, I::m_sectorsSize, 0);
    e.set(this, I::m_sectorsCapacity, 0x8000_0001);
    e.set(this, I::m_sectors, Ptr::new(this.addr() + 0x68));
    e.set(this, I::m_bytesUsedInLastSector, 0x200);
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0x3f);
    e.set(this, I::m_maxSizeOfSchema, 0);
    e.set(this, I::m_sizeOfSchemas, 0);
    e.set(this, I::m_numSolverResults, 0);
    e.set(this, I::m_numSolverElemTemps, 0);
    let flags = e.get(this, I::flags);
    e.set(this, I::flags, flags & 0xf3);
    e.set(this, I::m_numConstraints, 0);
    this
}

// Translated from 00d078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_7720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d078e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_7720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_7120, fn_00d07120(Ptr)),
        entry!(0x00d0_7130, fn_00d07130(Ptr<HkpSimulationIsland>, Ptr)),
        entry!(
            0x00d0_7180,
            fn_00d07180(Ptr<HkpSimulationIsland>, Ptr<HkpSimulationIsland>)
        ),
        entry!(0x00d0_71b0, fn_00d071b0(Ptr<HkpSimulationIsland>, Ptr)),
        entry!(
            0x00d0_7420,
            hkp_simulation_island_internal_add_entity(Ptr<HkpSimulationIsland>, Ptr)
        ),
        entry!(
            0x00d0_7470,
            hkp_simulation_island_add_action(Ptr<HkpSimulationIsland>, Ptr)
        ),
        entry!(
            0x00d0_7830,
            hkp_simulation_island_hkp_simulation_island(
                Ptr<HkpSimulationIsland>,
                Ptr,
            ) -> Ptr<HkpSimulationIsland>
        ),
        entry!(0x00d0_78e0, deleting_destructor_00d078e0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d078e0_frees_only_when_bit_zero_is_set() {
        check(0x00d0_78e0, 0x00d0_7720);
    }

    // ---- the island's own functions

    fn island(e: &mut Engine) -> Ptr<HkpSimulationIsland> {
        e.new_object::<HkpSimulationIsland>()
    }

    /// An `hkArray` header at `array` over a fresh block of `capacity` words
    /// holding `items`; returns the data pointer.
    fn make_array(e: &mut Engine, array: u32, capacity: u32, items: &[u32]) -> u32 {
        // One spare word in front so that index -1 stays inside the block.
        let words = (capacity & 0x3fff_ffff).max(items.len() as u32);
        let block = e.mem.alloc(4 * words + 4) + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *item);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 4, items.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        block
    }

    #[test]
    fn empty_function_does_nothing() {
        let mut e = engine();
        e.call(0x00d0_7120, &args![0x1234u32]);
    }

    #[test]
    fn remove_action_nulls_its_slot_and_flags_the_island() {
        let mut e = engine();
        let this = island(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 0xc, this.addr());
        let block = make_array(&mut e, this.addr() + 0x38, 4, &[first, second]);
        e.mem.set_u8(this.addr() + 0x25, 0xee);
        e.call(0x00d0_7130, &args![this, second]);
        assert_eq!(e.mem.u32(block), first);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.get(this, HkpSimulationIsland::m_actionsSize), 2);
        assert_eq!(e.mem.u32(second + 0xc), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x25), (0xee & 0xdd) | 0x11);
    }

    #[test]
    fn remove_action_that_is_not_there_writes_before_the_data() {
        let mut e = engine();
        let this = island(&mut e);
        let first = e.mem.alloc(0x20);
        let stranger = e.mem.alloc(0x20);
        let block = make_array(&mut e, this.addr() + 0x38, 4, &[first]);
        e.mem.set_u32(block - 4, 0x5555);
        e.call(0x00d0_7130, &args![this, stranger]);
        assert_eq!(e.mem.u32(block - 4), 0);
        assert_eq!(e.mem.u32(block), first);
        // An empty array gives index -1 as well.
        e.mem.set_u32(this.addr() + 0x3c, 0);
        e.mem.set_u32(block - 4, 0x5555);
        e.call(0x00d0_7130, &args![this, stranger]);
        assert_eq!(e.mem.u32(block - 4), 0);
    }

    #[test]
    fn merge_constraint_info_takes_the_larger_schema_and_adds_the_rest() {
        let mut e = engine();
        let this = island(&mut e);
        let other = island(&mut e);
        for (target, values) in [(this, [10, 1, 2, 3]), (other, [4, 10, 20, 30])] {
            for (i, value) in values.iter().enumerate() {
                e.mem.set_i32(target.addr() + 8 + 4 * i as u32, *value);
            }
        }
        e.call(0x00d0_7180, &args![this, other]);
        let merged: Vec<i32> = (0..4).map(|i| e.mem.i32(this.addr() + 8 + 4 * i)).collect();
        assert_eq!(merged, vec![10, 11, 22, 33]);
        // The maximum is signed: a larger other wins.
        e.mem.set_i32(this.addr() + 8, -5);
        e.mem.set_i32(other.addr() + 8, 7);
        e.call(0x00d0_7180, &args![this, other]);
        assert_eq!(e.mem.i32(this.addr() + 8), 7);
    }

    #[test]
    fn remove_entity_moves_the_last_one_into_its_slot() {
        let mut e = engine();
        let this = island(&mut e);
        let entities: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xe0)).collect();
        for (i, entity) in entities.iter().enumerate() {
            e.mem.set_u16(entity + 0xa8, i as u16);
            e.mem.set_u32(entity + 0xcc, this.addr());
        }
        let block = make_array(&mut e, this.addr() + 0x48, 4, &entities);
        e.mem.set_u8(this.addr() + 0x25, 0xfe);
        e.call(0x00d0_71b0, &args![this, entities[0]]);
        assert_eq!(e.mem.u32(block), entities[2]);
        assert_eq!(e.mem.u16(entities[2] + 0xa8), 0);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
        assert_eq!(e.mem.u32(entities[0] + 0xcc), 0);
        assert_eq!(e.mem.u16(entities[0] + 0xa8), 0xffff);
        assert_eq!(e.mem.u8(this.addr() + 0x25), (0xfe & 0xfd) | 1);
        // Removing the last entity leaves it with the marker only.
        e.call(0x00d0_71b0, &args![this, entities[2]]);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 1);
        assert_eq!(e.mem.u16(entities[2] + 0xa8), 0xffff);
    }

    type GrowthLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Doubles for the memory router and `_reserveMore`: the latter replaces
    /// the array's data with a block of twice the capacity (at least 4) and
    /// logs its arguments.
    fn install_growth(e: &mut Engine) -> GrowthLog {
        let log: GrowthLog = Rc::new(RefCell::new(Vec::new()));
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let grow_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push(a.to_vec());
            let array = a[1];
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff).max(2) * 2;
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let block = e.mem.alloc(4 * capacity);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(block + 4 * i, word);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        log
    }

    #[test]
    fn add_entity_appends_and_records_island_and_index() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let existing = e.mem.alloc(0xe0);
        let block = make_array(&mut e, this.addr() + 0x48, 4, &[existing]);
        let entity = e.mem.alloc(0xe0);
        e.call(0x00d0_7420, &args![this, entity]);
        assert_eq!(e.mem.u32(block + 4), entity);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
        assert_eq!(e.mem.u32(entity + 0xcc), this.addr());
        assert_eq!(e.mem.u16(entity + 0xa8), 1);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn add_entity_grows_a_full_array_first() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let existing = e.mem.alloc(0xe0);
        // Capacity 1 with the not-owned flag in bit 31 (as the inplace array has).
        make_array(&mut e, this.addr() + 0x48, 0x8000_0001, &[existing]);
        let entity = e.mem.alloc(0xe0);
        e.call(0x00d0_7420, &args![this, entity]);
        let calls = log.borrow().clone();
        assert_eq!(calls.len(), 1);
        // The array and the element size follow the allocator.
        assert_eq!(calls[0][1], this.addr() + 0x48);
        assert_eq!(calls[0][2], 4);
        let data = e.get(this, HkpSimulationIsland::m_entities).addr();
        assert_eq!(e.mem.u32(data), existing);
        assert_eq!(e.mem.u32(data + 4), entity);
        assert_eq!(e.get(this, HkpSimulationIsland::m_entitiesSize), 2);
    }

    #[test]
    fn add_action_appends_and_records_the_island() {
        let mut e = engine();
        let log = install_growth(&mut e);
        let this = island(&mut e);
        let block = make_array(&mut e, this.addr() + 0x38, 2, &[]);
        let action = e.mem.alloc(0x20);
        e.call(0x00d0_7470, &args![this, action]);
        assert_eq!(e.mem.u32(block), action);
        assert_eq!(e.get(this, HkpSimulationIsland::m_actionsSize), 1);
        assert_eq!(e.mem.u32(action + 0xc), this.addr());
        assert!(log.borrow().is_empty());
        // A full array grows through the helper.
        let second = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x20);
        e.call(0x00d0_7470, &args![this, second]);
        e.call(0x00d0_7470, &args![this, third]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0][1], this.addr() + 0x38);
        let data = e.get(this, HkpSimulationIsland::m_actions).addr();
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [action, second, third]
        );
    }

    #[test]
    fn constructor_sets_the_empty_island_up() {
        let mut e = engine();
        e.set_global(TIME_OF_DEACTIVATION_INITIAL, -10.0f32);
        let this = island(&mut e);
        // Fill with a pattern to see what the constructor leaves alone.
        for offset in (0..0x6c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xcdcd_cdcd);
        }
        let world = e.mem.alloc(0x10);
        let back = e.call(0x00d0_7830, &args![this, world]).u32();
        assert_eq!(back, this.addr());
        let word = |e: &Engine, offset: u32| e.mem.u32(this.addr() + offset);
        assert_eq!(word(&e, 0), ISLAND_VTABLE);
        assert_eq!(e.mem.u16(this.addr() + 6), 1);
        assert_eq!(word(&e, 0x18), world);
        assert_eq!(word(&e, 0x1c), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x22), 0xffff);
        assert_eq!(e.mem.u8(this.addr() + 0x25), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x26), 5);
        assert_eq!(word(&e, 0x28), 0xffff_fff1);
        assert_eq!(e.mem.u16(this.addr() + 0x2c), 0x8000);
        assert_eq!(word(&e, 0x30), 0);
        assert_eq!(word(&e, 0x34), 0);
        assert_eq!(
            [word(&e, 0x38), word(&e, 0x3c), word(&e, 0x40)],
            [0, 0, 0x8000_0000]
        );
        assert_eq!(e.mem.f32(this.addr() + 0x44), -10.0);
        assert_eq!(
            [word(&e, 0x48), word(&e, 0x4c), word(&e, 0x50)],
            [this.addr() + 0x54, 0, 0x8000_0001]
        );
        assert_eq!(
            [
                word(&e, 0x58),
                word(&e, 0x5c),
                word(&e, 0x60),
                word(&e, 0x64)
            ],
            [0x200, this.addr() + 0x68, 0, 0x8000_0001]
        );
        for offset in [0x8, 0xc, 0x10, 0x14] {
            assert_eq!(word(&e, offset), 0);
        }
        // Not touched: the flags at +4, the storage index, the split counter
        // and the inplace storage.
        assert_eq!(e.mem.u16(this.addr() + 4), 0xcdcd);
        assert_eq!(e.mem.u16(this.addr() + 0x20), 0xcdcd);
        assert_eq!(e.mem.u8(this.addr() + 0x24), 0xcd);
        assert_eq!(word(&e, 0x54), 0xcdcd_cdcd);
        assert_eq!(word(&e, 0x68), 0xcdcd_cdcd);
    }
}
