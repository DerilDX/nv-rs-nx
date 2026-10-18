//! `Havok SDK/hkpworldoperationutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This is the group of `hkpWorldOperationUtil` functions that move constraints, actions and
//! entities between simulation islands and the broad phase, and the motion-type switch
//! (`setRigidBodyMotionType`) built on them.
//!
//! The objects are read by PC offset (names from the Xbox PDB):
//! - `hkpWorld`: +0x08 `m_simulation`, +0x20 `m_fixedIsland`, +0x28 `m_activeSimulationIslands`,
//!   +0x34 `m_inactiveSimulationIslands`, +0x40 `m_dirtySimulationIslands` (`hkArray`: data,
//!   size, capacity-and-flags), +0x58 `m_broadPhase`, +0x64 `m_broadPhaseDispatcher`, +0x78
//!   `m_collisionInput`, +0x7c `m_collisionFilter`, +0x8c `m_pendingOperationsCount`, +0x90
//!   `m_pendingBodyOperationsCount`, +0x94 `m_criticalOperationsLockCount`, +0x9c
//!   `m_blockExecutingPendingOperations`, +0x9d `m_criticalOperationsAllowed`, +0xa4
//!   `m_pendingOperationQueueCount`, +0xb4 `m_minDesiredIslandSize`, +0xc8
//!   `m_wantSimulationIslands`, +0x140 `m_constraintListeners` (size at +0x144), +0x338 the
//!   broad-phase query buffer size.
//! - `hkpSimulationIsland` (0x6c bytes): +0x1c constraint count, +0x20 index in its island
//!   array (16 bits), +0x22 index in the world's dirty list (16 bits, 0xffff = not in it),
//!   +0x24 a byte, +0x25 and +0x26 flag bytes, +0x4c entity count.
//! - `hkpEntity`: +0x08 `m_world`, +0x10 the collidable (its shape at +0x10, broad-phase
//!   handle at +0x24, AABB at +0x30), +0xac/+0xb0 `m_constraintsMaster` (small array of 0x30
//!   byte `hkConstraintInternal`, whose first word is the instance), +0xb4/+0xb8
//!   `m_constraintsSlave`, +0xcc `m_simulationIsland`, +0xd4 the unique id, +0xe0 the embedded
//!   `hkpMotion`, +0xe8 the motion type byte (5 is `MOTION_FIXED`, 4 `MOTION_KEYFRAMED`),
//!   +0x208/+0x20c `m_actions` (small array).
//!
//! Locks and arrays follow the exe: `hkArray` is `(data, size, capacityAndFlags)` with the
//! "do not free" flag in bit 31 and the capacity in the low 30 bits; an `hkLocalArray` is
//! the same plus the saved top of the memory router's stack in a fourth word.
//!
//! `00cb3d10` is `findSmallSparseActiveIsland`, which takes the world in `EAX` and the array
//! in `ECX` and two stack words; the uniform form passes them in that order (world, array,
//! the two stack words). Its one caller is `00cb4640`.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------------------
// Callees outside this file, by address.
// ---------------------------------------------------------------------------------------

/// The memory router: word +0 is the top of its stack, +0x10 the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` growing one more element: `(allocator, array, element size)`
/// (`00c90940`, unnamed in the engine map).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject` add and remove reference (`00c90510`, `00c905b0`; the engine map has
/// only the unit), `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Unnamed `hkpWorld` pieces run when the critical-operations lock is released: `00c91120`
/// when operations are pending, `00c91140` when exactly one queue holds body operations.
const WORLD_RUN_PENDING_OPERATIONS: u32 = 0x00c9_1120;
const WORLD_RUN_PENDING_BODY_OPERATIONS: u32 = 0x00c9_1140;
/// Unnamed `hkpWorld` function that appends a 12-byte operation record to the pending queue
/// (`this` = world, one stack word).
const WORLD_ENQUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpWorld::getCurrentTime` (Xbox PDB), result in `ST0`.
const WORLD_GET_CURRENT_TIME: u32 = 0x00c9_1020;
/// `hkpWorld::updateCollisionFilterOnEntity` (Xbox PDB): `this` = world, then the entity and two
/// stack words.
const WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY: u32 = 0x00c9_46e0;
/// `hkpWorldConstraintUtil::addConstraint` and `removeConstraint` (Xbox PDB).
const WORLD_CONSTRAINT_UTIL_ADD: u32 = 0x00d2_7ac0;
const WORLD_CONSTRAINT_UTIL_REMOVE: u32 = 0x00d2_7820;
/// `hkpWorldCallbackUtil::fireConstraintAdded`, `fireConstraintRemoved` and
/// `fireInactiveEntityMoved` (Xbox PDB).
const FIRE_CONSTRAINT_ADDED: u32 = 0x00d0_0960;
const FIRE_CONSTRAINT_REMOVED: u32 = 0x00d0_0a00;
const FIRE_INACTIVE_ENTITY_MOVED: u32 = 0x00d0_17b0;
/// `hkpSimulationIsland::internalAddEntity` and `addAction` (Xbox PDB), `this` = island.
const ISLAND_ADD_ENTITY: u32 = 0x00d0_7420;
const ISLAND_ADD_ACTION: u32 = 0x00d0_7470;
/// Unnamed `hkpSimulationIsland` functions: remove an entity (`00d071b0`) and register an
/// action (`00d07130`), `this` = island.
const ISLAND_REMOVE_ENTITY: u32 = 0x00d0_71b0;
const ISLAND_REGISTER_ACTION: u32 = 0x00d0_7130;
/// `hkpSimulationIsland::hkpSimulationIsland` (Xbox PDB): `this` = memory, one stack word (the
/// world), returns `this`.
const ISLAND_CONSTRUCTOR: u32 = 0x00d0_7830;
/// `hkpPhantom::setBoundingVolumeData` (Xbox PDB): `this` = phantom, the AABB buffer.
const PHANTOM_SET_BOUNDING_VOLUME_DATA: u32 = 0x00d0_7950;
/// `hkpLinkedCollidable` entries (`00d081e0`, translated in `hkplinkedcollidable.rs`),
/// sorted entries (`00d08210`) and sorted collection (`00d08260`), `this` = collidable.
const LINKED_COLLIDABLE_ENTRIES: u32 = 0x00d0_81e0;
const LINKED_COLLIDABLE_SORT_ENTRIES: u32 = 0x00d0_8210;
const LINKED_COLLIDABLE_GET_ENTRIES_SORTED: u32 = 0x00d0_8260;
/// `hkAgentNnMachine_CopyAndRelinkAgentEntry`, `InternalDeallocateEntry` and `AppendTrack`
/// (Xbox PDB).
const AGENT_COPY_AND_RELINK_ENTRY: u32 = 0x00cf_eca0;
const AGENT_DEALLOCATE_ENTRY: u32 = 0x00cf_e020;
const AGENT_APPEND_TRACK: u32 = 0x00cf_f290;
/// `hkpWorldAgentUtil::getIslandFromAgentEntry` (Xbox PDB): `(entry, island, island)`.
const AGENT_UTIL_GET_ISLAND_FROM_ENTRY: u32 = 0x00cc_0e20;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB): `(collision input, entity slots, count)`.
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpTypedBroadPhaseDispatcher` (Xbox PDB): `addPairs` (`this` = dispatcher, pairs, count,
/// filter), `removePairs` (`this`, pairs, count) and the cdecl `removeDuplicates`.
const DISPATCHER_ADD_PAIRS: u32 = 0x00cf_7080;
const DISPATCHER_REMOVE_PAIRS: u32 = 0x00cf_70f0;
const DISPATCHER_REMOVE_DUPLICATES: u32 = 0x00cf_7130;
/// `hkpWorldOperationUtil::internalMergeTwoIslands` (Xbox PDB).
const INTERNAL_MERGE_TWO_ISLANDS: u32 = 0x00cb_4c60;
/// `hkpEntity::sortConstraintsSlavesDeterministically` and `setCachedShapeData` (Xbox PDB).
const ENTITY_SORT_CONSTRAINTS_SLAVES: u32 = 0x00c9_c860;
const ENTITY_SET_CACHED_SHAPE_DATA: u32 = 0x00c9_c040;
/// `hkpMotion` (Xbox PDB): `getMass` (`this` = the copy, five stack words), `hkMotionState::
/// operator=`, and `getMotionStateAndVelocitiesAndDeactivationType` (`this` = source, the
/// destination motion).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
const MOTION_STATE_ASSIGN: u32 = 0x00c8_6660;
const MOTION_COPY_STATE_AND_VELOCITIES: u32 = 0x00c8_6610;
/// `hkpRigidBody::createDynamicRigidMotion` (Xbox PDB), cdecl.
const RIGID_BODY_CREATE_DYNAMIC_MOTION: u32 = 0x00ca_8240;
/// `hkSweptTransformUtil::freezeMotionState` (Xbox PDB), cdecl: `(time, motion state)`.
const FREEZE_MOTION_STATE: u32 = 0x00cf_1d50;
/// `hkpKeyframedRigidMotion` constructor and destructor, `hkpFixedRigidMotion` constructor
/// (Xbox PDB); `this` = the motion, constructors take two vector pointers.
const KEYFRAMED_MOTION_CONSTRUCTOR: u32 = 0x00d1_dfa0;
const KEYFRAMED_MOTION_DESTRUCTOR: u32 = 0x00d1_dff0;
const FIXED_MOTION_CONSTRUCTOR: u32 = 0x00d1_e160;
/// `hkString::memSet` (Xbox PDB), cdecl `(destination, value, count)`.
const MEM_SET: u32 = 0x00cb_1280;
/// Unnamed `hkpWorld` function called with the three flag bytes of the dynamics step info
/// (`00c90ed0`, `this` = the motion).
const WORLD_SET_MOTION_FLAGS: u32 = 0x00c9_0ed0;
/// `hkpSimulation::collideEntitiesBroadPhaseDiscrete` (Xbox PDB), cdecl.
const SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE: u32 = 0x00cf_9c60;
/// The table of floats indexed by the two bytes at +0xbc/+0xbd of an `hkpMotion` copy.
const MOTION_FLOAT_TABLE: u32 = 0x010c_7948;

// Fields of `hkpWorld` (see the header).
const WORLD_SIMULATION: u32 = 0x08;
const WORLD_FIXED_ISLAND: u32 = 0x20;
const WORLD_ACTIVE_ISLANDS: u32 = 0x28;
const WORLD_INACTIVE_ISLANDS: u32 = 0x34;
const WORLD_DIRTY_ISLANDS: u32 = 0x40;
const WORLD_BROAD_PHASE: u32 = 0x58;
const WORLD_DISPATCHER: u32 = 0x64;
const WORLD_COLLISION_INPUT: u32 = 0x78;
const WORLD_COLLISION_FILTER: u32 = 0x7c;
const WORLD_PENDING_OPERATIONS: u32 = 0x8c;
const WORLD_PENDING_BODY_OPERATIONS: u32 = 0x90;
const WORLD_LOCK_COUNT: u32 = 0x94;
const WORLD_BLOCK_PENDING: u32 = 0x9c;
const WORLD_CRITICAL_ALLOWED: u32 = 0x9d;
const WORLD_PENDING_QUEUE_COUNT: u32 = 0xa4;
const WORLD_MIN_ISLAND_SIZE: u32 = 0xb4;
const WORLD_WANT_ISLANDS: u32 = 0xc8;
const WORLD_CONSTRAINT_LISTENERS_SIZE: u32 = 0x144;
const WORLD_STEP_TIME: u32 = 0x1d0;
const WORLD_BROAD_PHASE_QUERY_SIZE: u32 = 0x338;

// Fields of `hkpSimulationIsland`.
const ISLAND_CONSTRAINT_COUNT: u32 = 0x1c;
const ISLAND_INDEX: u32 = 0x20;
const ISLAND_DIRTY_INDEX: u32 = 0x22;
const ISLAND_BYTE_24: u32 = 0x24;
const ISLAND_FLAGS_25: u32 = 0x25;
const ISLAND_FLAGS_26: u32 = 0x26;
const ISLAND_AGENT_TRACK: u32 = 0x58;
const ISLAND_ENTITY_COUNT: u32 = 0x4c;

// Fields of `hkpEntity`.
const ENTITY_WORLD: u32 = 0x08;
const ENTITY_COLLIDABLE: u32 = 0x10;
const ENTITY_BYTE_2A: u32 = 0x2a;
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_MOTION_TYPE: u32 = 0xe8;
const ENTITY_ISLAND: u32 = 0xcc;
const ENTITY_UNIQUE_ID: u32 = 0xd4;
const ENTITY_MOTION_STATE: u32 = 0xf0;
const ENTITY_ACTIONS: u32 = 0x208;
const ENTITY_ACTIONS_SIZE: u32 = 0x20c;

/// `MOTION_FIXED` and `MOTION_KEYFRAMED` (Xbox PDB `hkpMotion::MotionType`).
const MOTION_FIXED: u32 = 5;
const MOTION_KEYFRAMED: u32 = 4;
/// Size of the embedded `hkpMotion` copied when the motion object is replaced.
const MOTION_BYTES: u32 = 0x120;
/// The vtable pointer the exe stores at the start of its motion copy before the copy loop.
const MOTION_COPY_VTABLE: u32 = 0x010c_6c64;
/// The "not in the dirty list" value of an island's 16-bit dirty index.
const NOT_IN_DIRTY_LIST: u16 = 0xffff;

// ---------------------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------------------

/// Adds `delta` (wrapping) to the word at `addr` and returns the new value.
fn add_word(e: &mut Engine, addr: u32, delta: u32) -> u32 {
    let value = e.mem.u32(addr).wrapping_add(delta);
    e.mem.set_u32(addr, value);
    value
}

/// Appends one word to the `hkArray` at `array`, growing it first when it is full.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Puts the island in the world's dirty list unless it is already there.
fn mark_island_dirty(e: &mut Engine, world: u32, island: u32) {
    if e.mem.u16(island + ISLAND_DIRTY_INDEX) == NOT_IN_DIRTY_LIST {
        let dirty_count = e.mem.u16(world + WORLD_DIRTY_ISLANDS + 4);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, dirty_count);
        push_word(e, world + WORLD_DIRTY_ISLANDS, island);
    }
}

/// Runs the world's pending operations once a critical-operations lock is released.
fn run_pending_operations(e: &mut Engine, world: u32) {
    if e.mem.u32(world + WORLD_PENDING_OPERATIONS) != 0 {
        e.call(WORLD_RUN_PENDING_OPERATIONS, &args![world]);
    }
    if e.mem.u32(world + WORLD_PENDING_QUEUE_COUNT) == 1
        && e.mem.u32(world + WORLD_PENDING_BODY_OPERATIONS) != 0
    {
        e.call(WORLD_RUN_PENDING_BODY_OPERATIONS, &args![world]);
    }
}

/// Drops the lock count by one and, when it reaches zero with nothing blocking, runs the
/// pending operations.
fn release_critical_lock(e: &mut Engine, world: u32) {
    if add_word(e, world + WORLD_LOCK_COUNT, u32::MAX) == 0
        && e.mem.u8(world + WORLD_BLOCK_PENDING) == 0
    {
        run_pending_operations(e, world);
    }
}

/// The collision filter interface (`m_collisionFilter` + 8) or null.
fn collision_filter_interface(e: &Engine, world: u32) -> u32 {
    let filter = e.mem.u32(world + WORLD_COLLISION_FILTER);
    if filter != 0 {
        filter + 8
    } else {
        0
    }
}

/// Creates the `hkLocalArray` (data, size, capacity-and-flags, saved stack top; 8-byte
/// elements) at `array`, sized by the world's broad-phase query buffer size, taking its
/// storage from the memory router's stack.
fn scratch_array_create(e: &mut Engine, array: u32, world: u32) {
    let capacity = e.mem.u32(world + WORLD_BROAD_PHASE_QUERY_SIZE);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved = e.mem.u32(router);
    let bytes = capacity.wrapping_mul(8).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, bytes.wrapping_add(saved));
    e.mem.set_u32(array, saved);
    e.mem.set_u32(array + 8, capacity | 0x8000_0000);
    e.mem.set_u32(array + 0xc, saved);
}

/// Destroys an array: frees its storage through the allocator when it owns it
/// (`element_bytes` per element) and clears it.
fn array_free_storage(e: &mut Engine, array: u32, element_bytes: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.u32(array + 8);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_bytes);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Destroys a local array made by [`scratch_array_create`]: gives its storage back to the
/// router's stack, then frees it when it outgrew the stack.
fn scratch_array_destroy(e: &mut Engine, array: u32) {
    let saved = e.mem.u32(array + 0xc);
    if saved == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    array_free_storage(e, array, 8);
}

/// Initialises an inline array of 16 words at `array` (data, size, capacity-and-flags, storage).
fn inplace_array_init(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, array + 12);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0010);
}

/// `hkUnionFind::reindex` (Xbox PDB, `00d282c0`, `this` = the union-find object, `RET 0xC`:
/// the address of the word holding the group array, the number of groups, the parent array).
const UNION_FIND_REINDEX: u32 = 0x00d2_82c0;
/// The `KERNEL32` import slots of `EnterCriticalSection` and `LeaveCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkpWorld` words used by the island-splitting functions below: +0xC0 the critical section
/// pointer, +0xD5 a byte choosing how entities are ranked, +0x305 and +0x306 the two bytes the
/// ranks are compared against (PC offsets; the names are not confirmed).
const WORLD_CRITICAL_SECTION: u32 = 0xc0;
const WORLD_RANKING_ENABLED: u32 = 0xd5;
const WORLD_RANK_BYTE_FIRST: u32 = 0x305;
const WORLD_RANK_BYTE_SECOND: u32 = 0x306;
/// The 16-bit words of an `hkpEntity` the rank is read from (+0xEA, +0xEC) and the 16-bit
/// count of its master constraints (+0xB0).
const ENTITY_RANK_WORD_FIRST: u32 = 0xea;
const ENTITY_RANK_WORD_SECOND: u32 = 0xec;
const ENTITY_MASTER_COUNT: u32 = 0xb0;

/// The rank of an entity as the splitting code computes it: each 16-bit word is shifted
/// right by 7 when its top two bits equal the world's byte, and masked with 0x7f; the larger
/// of the two (signed) is the rank.
fn entity_rank(e: &Engine, world: u32, entity: u32) -> i32 {
    let mut second = i32::from(e.mem.u16(entity + ENTITY_RANK_WORD_SECOND));
    if i32::from(e.mem.u8(world + WORLD_RANK_BYTE_SECOND)) == second >> 14 {
        second >>= 7;
    }
    let mut first = i32::from(e.mem.u16(entity + ENTITY_RANK_WORD_FIRST));
    if i32::from(e.mem.u8(world + WORLD_RANK_BYTE_FIRST)) == first >> 14 {
        first >>= 7;
    }
    (second & 0x7f).max(first & 0x7f)
}

/// Entry `index` of the integer array that the union-find object points at through two
/// levels (`**union_find`).
fn union_find_entry(e: &Engine, union_find: u32, index: u32) -> u32 {
    let inner = e.mem.u32(union_find);
    let data = e.mem.u32(inner);
    e.mem.u32(data.wrapping_add(index.wrapping_mul(4)))
}

/// Address of entry `index` of an array of words.
fn word_at(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(4))
}

/// Calls the sort's comparison function (cdecl, `(result byte, a, b)`) and reads the byte it
/// answers the address of.
fn sort_compare(e: &mut Engine, compare: u32, slot: u32, a: u32, b: u32) -> bool {
    let result = e.call(compare, &args![slot, a, b]).u32();
    e.mem.u8(result) != 0
}

// Translated from 00cb3c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addConstraintToCriticalLockedIsland` (Xbox PDB), cdecl `(world,
/// constraint)`: `hkpWorldConstraintUtil::addConstraint` (`00d27ac0`) then
/// `hkpWorldCallbackUtil::fireConstraintAdded` (`00d00960`), both with the same two words.
pub fn hkp_world_operation_util_add_constraint_to_critical_locked_island(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
) {
    e.call(WORLD_CONSTRAINT_UTIL_ADD, &args![world, constraint]);
    e.call(FIRE_CONSTRAINT_ADDED, &args![world, constraint]);
}

// Translated from 00cb3da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeConstraintFromCriticalLockedIsland` (Xbox PDB), cdecl `(world,
/// constraint)`: when the world's constraint listener count (+0x144) is not 0, runs
/// `hkpWorldCallbackUtil::fireConstraintRemoved` (`00d00a00`) with both words; then
/// `hkpWorldConstraintUtil::removeConstraint` (`00d27820`) on the constraint.
pub fn hkp_world_operation_util_remove_constraint_from_critical_locked_island(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
) {
    if e.mem.u32(world.addr() + WORLD_CONSTRAINT_LISTENERS_SIZE) != 0 {
        e.call(FIRE_CONSTRAINT_REMOVED, &args![world, constraint]);
    }
    e.call(WORLD_CONSTRAINT_UTIL_REMOVE, &args![constraint]);
}

// Translated from 00cb4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no caller in the exe). Takes `entities` in `EAX` (an `hkArray` of entity pointers),
/// then cdecl `(union_find, world, parents, group_flags)`; the register value comes first.
/// On the memory router's stack it makes a zeroed array of one word per element of `parents`
/// (count at `parents + 4`; the router's top moves up by the byte size rounded to 0x80). Every
/// entity whose rank (`entity_rank`) is at most 5 sets the word, at the index
/// `**union_find` holds for it, to 1. Then, with `group_flags` the address of a word holding
/// a byte array, byte 0 is set to 1 and each word is numbered: a set word is cleared, a clear
/// one gets the next number (from 1) and its byte (at the number) is cleared. If the next
/// number is above the count, byte 0 is cleared and the router's top is restored; otherwise
/// `hkUnionFind::reindex` (`00d282c0`, `this` = `union_find`) is called with the address of a
/// stack word holding the array, the next number and `parents`, and the router's top is set
/// to the word's value afterwards.
pub fn fn_00cb4890(
    e: &mut Engine,
    entities: u32,
    union_find: u32,
    world: u32,
    parents: u32,
    group_flags: u32,
) {
    let count = e.mem.i32(parents + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let marks = e.mem.u32(router);
    let size = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(marks));
    for index in 0..count.max(0) as u32 {
        e.mem.set_u32(marks + index * 4, 0);
    }
    let mut index = 0i32;
    while index < e.mem.i32(entities + 4) {
        let data = e.mem.u32(entities);
        let entity = e.mem.u32(word_at(data, index));
        if entity_rank(e, world, entity) <= 5 {
            let group = union_find_entry(e, union_find, index as u32);
            e.mem.set_u32(word_at(marks, group as i32), 1);
        }
        index += 1;
    }
    let flags = e.mem.u32(group_flags);
    e.mem.set_u8(flags, 1);
    let mut next = 1u32;
    for index in 0..count.max(0) {
        let slot = word_at(marks, index);
        if e.mem.u32(slot) != 0 {
            e.mem.set_u32(slot, 0);
        } else {
            e.mem.set_u32(slot, next);
            let flags = e.mem.u32(group_flags);
            e.mem.set_u8(flags.wrapping_add(next), 0);
            next += 1;
        }
    }
    if (next as i32) <= count {
        let restored = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), marks);
            e.call(UNION_FIND_REINDEX, &args![union_find, slot, next, parents]);
            e.mem.u32(slot.addr())
        });
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, restored);
    } else {
        let flags = e.mem.u32(group_flags);
        e.mem.set_u8(flags, 0);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, marks);
    }
}

// Translated from 00cb49e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no caller in the exe), cdecl `(world, union_find, entities, parents, group_flags)`:
/// splits entities into groups for new islands. Two zeroed word arrays (weights, then marks;
/// one word per element of `parents`, count at `parents + 4`) are taken from the memory
/// router's stack. Every entity adds its master constraint count (+0xB0, at least 1) to the
/// weight at the index `**union_find` holds for it and sets the mark to 1: with the world's
/// byte at +0xD5 clear always, otherwise only when its rank (`entity_rank`) is at most 5.
/// Weights with no mark become -1. Then the weights are walked: a weight of -2 is skipped;
/// otherwise the next group number is given to the element in the marks array, its flag byte
/// (the byte array `group_flags` points at) is cleared and, for a weight of 0 or more, the
/// later weights are merged in while the running total plus 10 stays below the world's
/// word at +0xB4: each later weight of 0 or more that keeps the total below it gets the
/// group number, becomes -2, adds to the total and sets the group's flag byte to 1. At the
/// end `hkUnionFind::reindex` (`00d282c0`, `this` = `union_find`) runs with the address of a
/// stack word holding the marks, the group count and `parents`; the router's top is set to the
/// word's value and then to the start of the weights.
pub fn fn_00cb49e0(
    e: &mut Engine,
    world: u32,
    union_find: u32,
    entities: u32,
    parents: u32,
    group_flags: u32,
) {
    let count = e.mem.i32(parents + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let weights = e.mem.u32(router);
    let size = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(weights));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let marks = e.mem.u32(router);
    e.mem.set_u32(router, size.wrapping_add(marks));
    for index in 0..count.max(0) {
        e.mem.set_u32(word_at(marks, index), 0);
        e.mem.set_u32(word_at(weights, index), 0);
    }
    let ranked = e.mem.u8(world + WORLD_RANKING_ENABLED) != 0;
    let mut index = 0i32;
    while index < e.mem.i32(entities + 4) {
        let data = e.mem.u32(entities);
        let entity = e.mem.u32(word_at(data, index));
        let group = union_find_entry(e, union_find, index as u32);
        if !ranked || entity_rank(e, world, entity) <= 5 {
            e.mem.set_u32(word_at(marks, group as i32), 1);
        }
        let weight = u32::from(e.mem.u16(entity + ENTITY_MASTER_COUNT)).max(1);
        add_word(e, word_at(weights, group as i32), weight);
        index += 1;
    }
    for index in 0..count.max(0) {
        if e.mem.u32(word_at(marks, index)) == 0 {
            e.mem.set_u32(word_at(weights, index), 0xffff_ffff);
        }
    }
    let mut next_group = 0i32;
    if count > 0 {
        let mut weight_cursor = weights;
        let mut mark_cursor = marks;
        let mut first_later = 1i32;
        let mut remaining = count;
        loop {
            let mut running = e.mem.i32(weight_cursor);
            if running != -2 {
                let group = next_group;
                next_group += 1;
                let flags = e.mem.u32(group_flags);
                e.mem.set_u8(flags.wrapping_add(group as u32), 0);
                e.mem.set_i32(mark_cursor, group);
                if running >= 0 && first_later < count {
                    let mut later = first_later;
                    let mut cursor = weight_cursor + 4;
                    loop {
                        let limit = e.mem.i32(world + WORLD_MIN_ISLAND_SIZE);
                        if running.wrapping_add(10) >= limit {
                            break;
                        }
                        let other = e.mem.i32(cursor);
                        if other >= 0 {
                            let total = other.wrapping_add(running);
                            if total < limit {
                                let flags = e.mem.u32(group_flags);
                                e.mem.set_u8(flags.wrapping_add(group as u32), 1);
                                let mark = marks.wrapping_sub(weights).wrapping_add(cursor);
                                e.mem.set_i32(mark, group);
                                running = total;
                                e.mem.set_i32(cursor, -2);
                            }
                        }
                        later += 1;
                        cursor += 4;
                        if later >= count {
                            break;
                        }
                    }
                }
            }
            mark_cursor += 4;
            first_later += 1;
            weight_cursor += 4;
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    let restored = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), marks);
        e.call(
            UNION_FIND_REINDEX,
            &args![union_find, slot, next_group as u32, parents],
        );
        e.mem.u32(slot.addr())
    });
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, restored);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, weights);
}

// Translated from 00cb5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, cdecl `(world, island)`: clears bits 2 and 3 of the island's flag byte at +0x26,
/// then, inside the world's critical section (the pointer at +0xC0, `EnterCriticalSection` and
/// `LeaveCriticalSection`), puts the island in the world's dirty list (+0x40) unless its
/// 16-bit index at +0x22 shows it already is (`mark_island_dirty`).
pub fn fn_00cb5480(e: &mut Engine, world: Ptr, island: Ptr) {
    let (world, island) = (world.addr(), island.addr());
    let flags = e.mem.u8(island + ISLAND_FLAGS_26);
    e.mem.set_u8(island + ISLAND_FLAGS_26, flags & 0xf3);
    let lock = e.mem.u32(world + WORLD_CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    mark_island_dirty(e, world, island);
    let lock = e.mem.u32(world + WORLD_CRITICAL_SECTION);
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00cb7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall `(instance, info)` on the constraint owner (`RET 8`): the opposite of
/// `hkpConstraintOwner::addConstraintInfo` for an instance with a record (+0x30; without one
/// nothing happens). The owner's words at +0xC, +0x10 and +0x14 lose the info's words at +4,
/// +8 and +0xC; the record's 16-bit +0x14 loses the info's +4, its +0x16 loses the 16 bits of
/// the info's +8 and its +0x18 loses the info's +0xC.
pub fn fn_00cb7480(e: &mut Engine, this: Ptr, instance: Ptr, info: Ptr) {
    if e.mem.u32(instance.addr() + 0x30) == 0 {
        return;
    }
    let (owner, info) = (this.addr(), info.addr());
    for (field, word) in [(0xc, 4), (0x10, 8), (0x14, 0xc)] {
        let amount = e.mem.u32(info + word);
        add_word(e, owner + field, amount.wrapping_neg());
    }
    let record = e.mem.u32(instance.addr() + 0x30);
    let first = u32::from(e.mem.u16(record + 0x14));
    let third = u32::from(e.mem.u16(record + 0x18));
    let first = first.wrapping_sub(e.mem.u32(info + 4));
    let third = third.wrapping_sub(e.mem.u32(info + 0xc));
    let second = e.mem.u16(info + 8);
    let current = e.mem.u16(record + 0x16);
    e.mem.set_u16(record + 0x16, current.wrapping_sub(second));
    e.mem.set_u16(record + 0x14, first as u16);
    e.mem.set_u16(record + 0x18, third as u16);
}

// Translated from 00dd7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpSimulationIsland*, hkBool (*)(const hkpSimulationIsland*
/// const&, ...)>` (Xbox PDB name abbreviated), cdecl `(array, low, high, compare)`: sorts the
/// pointers `array[low..=high]` in place. The pivot is the element at `(low + high) >> 1`;
/// the scans use `compare(result slot, a, b)` (cdecl, answering the address of a byte): the
/// lower index advances while `compare(array[i], pivot)` is true, the upper one retreats
/// while `compare(pivot, array[j])` is true, elements are swapped while `i <= j`, the lower
/// part is sorted by recursion and the upper part by the outer loop. Index compares are
/// signed.
pub fn hk_algorithm_quick_sort_recursive_hkp_simulation_island(
    e: &mut Engine,
    array: u32,
    low: i32,
    high: i32,
    compare: u32,
) {
    e.with_stack(4, |e, slots| {
        let (first_slot, second_slot) = (slots.addr() + 2, slots.addr() + 3);
        let mut low = low;
        loop {
            let mut lower = low;
            let mut upper = high;
            let pivot = e.mem.u32(word_at(array, lower.wrapping_add(upper) >> 1));
            loop {
                let mut candidate = e.mem.u32(word_at(array, lower));
                while sort_compare(e, compare, first_slot, candidate, pivot) {
                    candidate = e.mem.u32(word_at(array, lower.wrapping_add(1)));
                    lower = lower.wrapping_add(1);
                }
                let mut candidate = e.mem.u32(word_at(array, upper));
                while sort_compare(e, compare, second_slot, pivot, candidate) {
                    candidate = e.mem.u32(word_at(array, upper.wrapping_sub(1)));
                    upper = upper.wrapping_sub(1);
                }
                if upper < lower {
                    break;
                }
                if upper != lower {
                    let at_lower = e.mem.u32(word_at(array, lower));
                    let at_upper = e.mem.u32(word_at(array, upper));
                    e.mem.set_u32(word_at(array, upper), at_lower);
                    e.mem.set_u32(word_at(array, lower), at_upper);
                }
                upper = upper.wrapping_sub(1);
                lower = lower.wrapping_add(1);
                if lower > upper {
                    break;
                }
            }
            if low < upper {
                hk_algorithm_quick_sort_recursive_hkp_simulation_island(
                    e, array, low, upper, compare,
                );
            }
            low = lower;
            if lower >= high {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb3ca0, hkp_world_operation_util_add_constraint_immediately(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb3d10, find_small_sparse_active_island(Ptr, Ptr, i32, Ptr) -> Ptr),
        entry!(0x00cb3dd0, hkp_world_operation_util_remove_constraint_immediately(Ptr, Ptr, u32)),
        entry!(0x00cb3ee0, hkp_world_operation_util_remove_attached_agents_connecting_the_entity_and_a_fixed_partner_entity_plus(Ptr, Ptr, Ptr, u32)),
        entry!(0x00cb3fd0, hkp_world_operation_util_remove_island_from_dirty_list(Ptr, Ptr)),
        entry!(0x00cb4000, fn_00cb4000(Ptr, Ptr)),
        entry!(0x00cb4230, hkp_world_operation_util_replace_motion_object(Ptr, u32, u32, u32, Ptr)),
        entry!(0x00cb4640, hkp_world_operation_util_add_entity_si(Ptr, Ptr, u32)),
        entry!(0x00cb4740, hkp_world_operation_util_remove_entity_si(Ptr, Ptr)),
        entry!(0x00cb4790, fn_00cb4790(Ptr, Ptr, Ptr)),
        entry!(0x00cb5000, fn_00cb5000(Ptr, Ptr)),
        entry!(0x00cb5500, fn_00cb5500(Ptr, Ptr)),
        entry!(0x00cb5570, hkp_world_operation_util_merge_islands(Ptr, Ptr, Ptr)),
        entry!(0x00cb5750, hkp_world_operation_util_update_entity_bp(Ptr, Ptr)),
        entry!(0x00cb59f0, hkp_world_operation_util_add_entity_bp(Ptr, Ptr)),
        entry!(0x00cb5b10, hkp_world_operation_util_add_phantom_bp(Ptr, Ptr)),
        entry!(0x00cb5c10, hkp_world_operation_util_remove_entity_bp(Ptr, Ptr)),
        entry!(0x00cb5d00, hkp_world_operation_util_remove_phantom_bp(Ptr, Ptr)),
        entry!(0x00cb5dd0, fn_00cb5dd0(Ptr, Ptr, Ptr)),
        entry!(0x00cb5f50, hkp_world_operation_util_add_actions_to_entitys_island(Ptr, Ptr, Ptr)),
        entry!(0x00cb6f70, hkp_world_operation_util_set_rigid_body_motion_type(Ptr, u32, u32, u32)),
        entry!(0x00cb7460, fn_00cb7460(Ptr, u32) -> u32),
        entry!(0x00cb74d0, hkp_constraint_owner_add_constraint_info(Ptr, Ptr, Ptr)),
        entry!(0x00cb_3c80, hkp_world_operation_util_add_constraint_to_critical_locked_island(Ptr, Ptr)),
        entry!(0x00cb_3da0, hkp_world_operation_util_remove_constraint_from_critical_locked_island(Ptr, Ptr)),
        entry!(0x00cb_4890, fn_00cb4890(u32, u32, u32, u32, u32)),
        entry!(0x00cb_49e0, fn_00cb49e0(u32, u32, u32, u32, u32)),
        entry!(0x00cb_5480, fn_00cb5480(Ptr, Ptr)),
        entry!(0x00cb_7480, fn_00cb7480(Ptr, Ptr, Ptr)),
        entry!(0x00dd_7d00, hk_algorithm_quick_sort_recursive_hkp_simulation_island(u32, i32, i32, u32)),
    ]
}

// Translated from 00cb3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addConstraintImmediately` (Xbox PDB): under the critical-operations
/// lock adds the constraint to the world (`hkpWorldConstraintUtil::addConstraint`), tells the
/// constraint listeners when `notify` is set, and returns the constraint.
pub fn hkp_world_operation_util_add_constraint_immediately(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
    notify: u32,
) -> Ptr {
    let world_addr = world.addr();
    add_word(e, world_addr + WORLD_LOCK_COUNT, 1);
    e.call(
        WORLD_CONSTRAINT_UTIL_ADD,
        &args![world_addr, constraint.addr()],
    );
    if notify != 0 {
        e.call(FIRE_CONSTRAINT_ADDED, &args![world_addr, constraint.addr()]);
    }
    release_critical_lock(e, world_addr);
    constraint
}

// Translated from 00cb3d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findSmallSparseActiveIsland` (Xbox PDB): looks through the last 20 islands of `array`
/// (the world's active islands, from the end) for one other than `skip` whose size (the larger of
/// its entity and constraint counts) plus `extra` stays under the world's minimum desired size
/// and that is marked active. The island found gets its "sparse" flag set and is returned;
/// null otherwise. Takes the world in `EAX` and the array in `ECX` in the exe.
pub fn find_small_sparse_active_island(
    e: &mut Engine,
    world: Ptr,
    array: Ptr,
    extra: i32,
    skip: Ptr,
) -> Ptr {
    let world_addr = world.addr();
    let minimum = e.mem.i32(world_addr + WORLD_MIN_ISLAND_SIZE);
    if extra >= minimum || e.mem.u8(world_addr + WORLD_WANT_ISLANDS) == 0 {
        return Ptr::new(0);
    }
    let size = e.mem.i32(array.addr() + 4);
    let below_window = size.wrapping_sub(0x14);
    let lowest = if below_window < 0 { 0 } else { below_window };
    let data = e.mem.u32(array.addr());
    let mut index = size.wrapping_sub(1);
    while index >= lowest {
        let island = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if island != skip.addr() {
            let entities = e.mem.i32(island + ISLAND_ENTITY_COUNT);
            let constraints = e.mem.i32(island + ISLAND_CONSTRAINT_COUNT);
            let island_size = if entities > constraints {
                entities
            } else {
                constraints
            };
            if e.mem.u8(island + ISLAND_FLAGS_26) & 0xc != 0
                && island_size.wrapping_add(extra) < minimum
            {
                let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                e.mem.set_u8(island + ISLAND_FLAGS_25, (flags & 0xf7) | 4);
                return Ptr::new(island);
            }
        }
        index -= 1;
    }
    Ptr::new(0)
}

// Translated from 00cb3dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeConstraintImmediately` (Xbox PDB): marks the object at
/// `constraint + 8` (flag byte +0x25: `& 0xfd | 1`), then removes the constraint
/// (`hkpWorldConstraintUtil::removeConstraint`), telling the constraint listeners first when
/// `notify` is set and the world has any. When no critical lock is held it takes one for the
/// removal and releases it afterwards.
pub fn hkp_world_operation_util_remove_constraint_immediately(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
    notify: u32,
) {
    let world_addr = world.addr();
    let constraint_addr = constraint.addr();
    let owner = e.mem.u32(constraint_addr + 8);
    let flags = e.mem.u8(owner + ISLAND_FLAGS_25);
    e.mem.set_u8(owner + ISLAND_FLAGS_25, (flags & 0xfd) | 1);
    let locked = e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0;
    if !locked {
        e.mem.set_u32(world_addr + WORLD_LOCK_COUNT, 1);
    }
    if notify != 0 && e.mem.u32(world_addr + WORLD_CONSTRAINT_LISTENERS_SIZE) != 0 {
        e.call(FIRE_CONSTRAINT_REMOVED, &args![world_addr, constraint_addr]);
    }
    e.call(WORLD_CONSTRAINT_UTIL_REMOVE, &args![constraint_addr]);
    if !locked {
        release_critical_lock(e, world_addr);
    }
}

// Translated from 00cb3ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeAttachedAgentsConnectingTheEntityAndAFixedPartnerEntityPlus`
/// (Xbox PDB): for each collision entry of the entity (cdecl, four words): when the partner's
/// owner is fixed, the agent is copied to `fixed_partner_track` and deallocated from
/// `island_track`; otherwise, when `new_type` is `MOTION_FIXED` and the agent's island is the
/// entity's, it is copied to the partner island's track (+0x58) and deallocated.
pub fn hkp_world_operation_util_remove_attached_agents_connecting_the_entity_and_a_fixed_partner_entity_plus(
    e: &mut Engine,
    island_track: Ptr,
    entity: Ptr,
    fixed_partner_track: Ptr,
    new_type: u32,
) {
    let entity_addr = entity.addr();
    let entries = e
        .call(
            LINKED_COLLIDABLE_ENTRIES,
            &args![entity_addr + ENTITY_COLLIDABLE],
        )
        .u32();
    let mut index = 0i32;
    while index < e.mem.i32(entries + 4) {
        let entry = e
            .mem
            .u32(entries)
            .wrapping_add((index as u32).wrapping_mul(8));
        let partner = e.mem.u32(entry + 4);
        let owner = partner.wrapping_add(e.mem.i8(partner + 0x10) as i32 as u32);
        if e.mem.u8(owner + ENTITY_MOTION_TYPE) as u32 == MOTION_FIXED {
            let agent = e.mem.u32(entry);
            e.call(
                AGENT_COPY_AND_RELINK_ENTRY,
                &args![fixed_partner_track.addr(), agent],
            );
            e.call(AGENT_DEALLOCATE_ENTRY, &args![island_track.addr(), agent]);
        } else if new_type == MOTION_FIXED {
            let agent = e.mem.u32(entry);
            let entity_island = e.mem.u32(entity_addr + ENTITY_ISLAND);
            let owner_island = e.mem.u32(owner + ENTITY_ISLAND);
            let found = e
                .call(
                    AGENT_UTIL_GET_ISLAND_FROM_ENTRY,
                    &args![agent, entity_island, owner_island],
                )
                .u32();
            if found == e.mem.u32(entity_addr + ENTITY_ISLAND) {
                let agent = e.mem.u32(entry);
                let owner_island = e.mem.u32(owner + ENTITY_ISLAND);
                e.call(
                    AGENT_COPY_AND_RELINK_ENTRY,
                    &args![owner_island + ISLAND_AGENT_TRACK, agent],
                );
                e.call(AGENT_DEALLOCATE_ENTRY, &args![island_track.addr(), agent]);
            }
        }
        index += 1;
    }
}

// Translated from 00cb3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeIslandFromDirtyList` (Xbox PDB): when the island is in the
/// world's dirty list, nulls its slot there and marks it as not in the list.
pub fn hkp_world_operation_util_remove_island_from_dirty_list(
    e: &mut Engine,
    world: Ptr,
    island: Ptr,
) {
    let index = e.mem.u16(island.addr() + ISLAND_DIRTY_INDEX);
    if index != NOT_IN_DIRTY_LIST {
        let data = e.mem.u32(world.addr() + WORLD_DIRTY_ISLANDS);
        e.mem.set_u32(data + index as u32 * 4, 0);
        e.mem
            .set_u16(island.addr() + ISLAND_DIRTY_INDEX, NOT_IN_DIRTY_LIST);
    }
}

// Translated from 00cb4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`): takes the island out of the world's active
/// islands (when its flag byte +0x26 has bits 0-1 set) or inactive islands, filling the hole
/// with the last island, then removes it from the dirty list.
pub fn fn_00cb4000(e: &mut Engine, world: Ptr, island: Ptr) {
    let world_addr = world.addr();
    let island_addr = island.addr();
    let list = if e.mem.u8(island_addr + ISLAND_FLAGS_26) & 3 != 0 {
        world_addr + WORLD_ACTIVE_ISLANDS
    } else {
        world_addr + WORLD_INACTIVE_ISLANDS
    };
    let slot = e.mem.u16(island_addr + ISLAND_INDEX) as u32;
    let data = e.mem.u32(list);
    let size = e.mem.u32(list + 4);
    let last = e
        .mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
    e.mem.set_u32(data.wrapping_add(slot * 4), last);
    let index = e.mem.u16(island_addr + ISLAND_INDEX);
    let data = e.mem.u32(list);
    let moved = e.mem.u32(data.wrapping_add(index as u32 * 4));
    e.mem.set_u16(moved + ISLAND_INDEX, index);
    add_word(e, list + 4, u32::MAX);
    hkp_world_operation_util_remove_island_from_dirty_list(e, world, island);
}

// Translated from 00cb4230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::replaceMotionObject` (Xbox PDB), cdecl, five words:
/// `new_is_dynamic` and `old_is_dynamic` are the bytes the caller computed from the new and
/// old motion types (neither fixed nor keyframed); `world` may be null.
///
/// Switching to a fixed or keyframed motion (`new_is_dynamic` clear): copies the entity's
/// 0x120-byte motion, builds the fixed or keyframed motion in place from the entity's position
/// and rotation (+0x120, +0x160), copies the motion state (a fixed motion is also frozen at
/// the world's time, or at `1 / rate + time` of the old state without a world), and either
/// keeps a heap copy of the old dynamic motion at +0x118 of the new one (`old_is_dynamic`) or
/// hands the old saved motion on. Switching to a dynamic motion: either restores the motion
/// saved at +0x1f8 (when the old one was not dynamic) or builds a dynamic motion with
/// `hkpRigidBody::createDynamicRigidMotion` from the old one's mass and inertia. The exe's
/// first stores to the copy (vtable pointer `0x010c6c64` and a zero saved-motion word at
/// +0x118) are overwritten by the copy loop; they are kept as the exe makes them. The two
/// `if (this)` tests of the placement new in front of the fixed and keyframed motion
/// constructors (`ECX = entity + 0xe0`) are kept too: a null motion gives a null new motion.
pub fn hkp_world_operation_util_replace_motion_object(
    e: &mut Engine,
    entity: Ptr,
    new_type: u32,
    new_is_dynamic: u32,
    old_is_dynamic: u32,
    world: Ptr,
) {
    let entity_addr = entity.addr();
    let world_addr = world.addr();
    let motion = entity_addr + ENTITY_MOTION;
    let new_is_dynamic = new_is_dynamic as u8 != 0;
    let old_is_dynamic = old_is_dynamic as u8 != 0;
    // The exe's stack frame: the 0x120-byte motion copy, followed by the inertia matrix.
    let copy = e.mem.alloc(0x160);
    let mut copied = false;
    if !new_is_dynamic {
        // The exe stores the copy's vtable pointer and a zero at +0x118 first.
        e.mem.set_u32(copy, MOTION_COPY_VTABLE);
        e.mem.set_u32(copy + 0x118, 0);
        let bytes = e.mem.bytes(motion, MOTION_BYTES);
        e.mem.write(copy, &bytes);
        copied = true;
        let new_motion;
        if new_type == MOTION_FIXED {
            new_motion = if motion != 0 {
                e.call(
                    FIXED_MOTION_CONSTRUCTOR,
                    &args![motion, entity_addr + 0x120, entity_addr + 0x160],
                )
                .u32()
            } else {
                0
            };
            e.call(MOTION_STATE_ASSIGN, &args![new_motion + 0x10, copy + 0x10]);
            let flag = e.mem.u8(copy + 9);
            e.mem.set_u8(new_motion + 9, flag);
            let rate = e.mem.f32(copy + 0x6c);
            if rate != 0.0 {
                let time = if world_addr != 0 {
                    e.call(WORLD_GET_CURRENT_TIME, &args![world_addr]).f32()
                } else {
                    let start = e.mem.f32(copy + 0x5c);
                    (1.0f64 / rate as f64 + start as f64) as f32
                };
                e.call(
                    FREEZE_MOTION_STATE,
                    &args![time.to_bits(), new_motion + 0x10],
                );
            }
        } else {
            new_motion = if motion != 0 {
                e.call(
                    KEYFRAMED_MOTION_CONSTRUCTOR,
                    &args![motion, entity_addr + 0x120, entity_addr + 0x160],
                )
                .u32()
            } else {
                0
            };
            e.call(MOTION_COPY_STATE_AND_VELOCITIES, &args![copy, new_motion]);
        }
        if old_is_dynamic {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let block = e.vcall(allocator, 4, &args![MOTION_BYTES]).u32();
            // The exe's stores into the new block that the copy overwrites.
            e.mem.set_u16(block + 4, MOTION_BYTES as u16);
            e.mem.set_u32(block + 0x118, 0);
            e.mem.set_u32(block, MOTION_COPY_VTABLE);
            let bytes = e.mem.bytes(copy, MOTION_BYTES);
            e.mem.write(block, &bytes);
            e.mem.set_u16(block + 4, MOTION_BYTES as u16);
            e.mem.set_u16(block + 6, 1);
            e.mem.set_u32(new_motion + 0x118, block);
            let byte_2a = e.mem.i8(entity_addr + ENTITY_BYTE_2A);
            e.mem.set_i16(new_motion + 0x11c, byte_2a as i16);
            let shape = e.mem.u32(entity_addr + ENTITY_COLLIDABLE);
            e.call(
                ENTITY_SET_CACHED_SHAPE_DATA,
                &args![entity_addr, world_addr, shape],
            );
        } else {
            let saved = e.mem.u32(copy + 0x118);
            e.vcall(new_motion, 0x6c, &args![saved]);
            let word = e.mem.u16(copy + 0x11c);
            e.mem.set_u16(new_motion + 0x11c, word);
        }
        e.mem.set_u32(entity_addr + 0xa4, 0);
        e.mem.set_u8(
            entity_addr + ENTITY_BYTE_2A,
            (new_type != MOTION_FIXED) as u8,
        );
    } else {
        if !old_is_dynamic {
            let saved = e.mem.u32(entity_addr + 0x1f8);
            e.vcall(motion, 0x64, &args![saved]);
            e.mem.set_u16(saved + 0xa, 0);
            e.mem.set_u16(saved + 0xc, 0);
            let byte = e.mem.u8(motion + 0x11c);
            e.mem.set_u8(entity_addr + ENTITY_BYTE_2A, byte);
            let bytes = e.mem.bytes(saved, MOTION_BYTES);
            e.mem.write(motion, &bytes);
            e.call(MEM_SET, &args![entity_addr + 0x200, 0u32, 0u32]);
            e.call(REMOVE_REFERENCE, &args![saved]);
            let shape = e.mem.u32(entity_addr + ENTITY_COLLIDABLE);
            e.call(
                ENTITY_SET_CACHED_SHAPE_DATA,
                &args![entity_addr, world_addr, shape],
            );
        }
        let old_type = e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32;
        if old_type != new_type && new_type != 1 {
            e.mem.set_u32(copy, MOTION_COPY_VTABLE);
            e.mem.set_u32(copy + 0x118, 0);
            let bytes = e.mem.bytes(motion, MOTION_BYTES);
            e.mem.write(copy, &bytes);
            copied = true;
            e.vcall(motion, 0x14, &args![copy + 0x120]);
            let second = e.mem.u8(copy + 0xbd) as u32;
            let first = e.mem.u8(copy + 0xbc) as u32;
            let second_value = x87_quiet_bits(e.mem.u32(MOTION_FLOAT_TABLE + second * 4));
            let first_value = x87_quiet_bits(e.mem.u32(MOTION_FLOAT_TABLE + first * 4));
            let mass = e
                .call(
                    MOTION_GET_MASS,
                    &args![
                        copy,
                        copy + 0x120,
                        copy + 0x90,
                        first_value,
                        second_value,
                        motion
                    ],
                )
                .f32();
            e.call(
                RIGID_BODY_CREATE_DYNAMIC_MOTION,
                &args![new_type, copy + 0x40, copy + 0x80, mass.to_bits()],
            );
            e.call(MOTION_COPY_STATE_AND_VELOCITIES, &args![copy, motion]);
            let first_bits = x87_quiet_bits(e.mem.u32(copy + 0xb4));
            e.mem.set_u32(entity_addr + 0x194, first_bits);
            let second_bits = x87_quiet_bits(e.mem.u32(copy + 0xb8));
            e.mem.set_u32(entity_addr + 0x198, second_bits);
        }
    }
    if copied {
        e.call(KEYFRAMED_MOTION_DESTRUCTOR, &args![copy]);
    }
    e.mem.free(copy);
    e.mem
        .set_u32(entity_addr + 0x18, entity_addr + ENTITY_MOTION_STATE);
    if world_addr != 0 {
        let first = e.mem.u8(world_addr + 0x305) as u32;
        let second = e.mem.u8(world_addr + 0x306) as u32;
        let third = e.mem.u8(world_addr + 0x307) as u32;
        e.call(WORLD_SET_MOTION_FLAGS, &args![motion, first, second, third]);
    }
}

// Translated from 00cb4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addEntitySI` (Xbox PDB), cdecl: puts the entity in the world and
/// in a simulation island. A fixed entity goes to the fixed island. Without island
/// management it goes to the first active island. Otherwise (`mode == 1`: active list, first
/// trying `findSmallSparseActiveIsland` with 4; else inactive list) a new island is built
/// and appended to the list. Ends with the entity's unique-id byte copied into the island's
/// byte +0x24.
pub fn hkp_world_operation_util_add_entity_si(e: &mut Engine, world: Ptr, entity: Ptr, mode: u32) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    e.mem.set_u32(entity_addr + ENTITY_WORLD, world_addr);
    if e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 == MOTION_FIXED {
        let fixed = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
        e.call(ISLAND_ADD_ENTITY, &args![fixed, entity_addr]);
        return;
    }
    let island;
    if e.mem.u8(world_addr + WORLD_WANT_ISLANDS) == 0 {
        let active = e.mem.u32(world_addr + WORLD_ACTIVE_ISLANDS);
        island = e.mem.u32(active);
    } else {
        let sparse = mode == 1;
        let list;
        if sparse {
            list = world_addr + WORLD_ACTIVE_ISLANDS;
            let found = e
                .call(0x00cb_3d10, &args![world_addr, list, 4u32, 0u32])
                .u32();
            if found != 0 {
                e.call(ISLAND_ADD_ENTITY, &args![found, entity_addr]);
                return;
            }
        } else {
            list = world_addr + WORLD_INACTIVE_ISLANDS;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![0x6cu32]).u32();
        e.mem.set_u16(block + 4, 0x6c);
        island = e.call(ISLAND_CONSTRUCTOR, &args![block, world_addr]).u32();
        let flag = sparse as u8;
        let old = e.mem.u8(island + ISLAND_FLAGS_26);
        e.mem.set_u8(
            island + ISLAND_FLAGS_26,
            (((flag << 2) | (old & 0xf3)) & 0xfc) | flag,
        );
        let count = e.mem.u16(list + 4);
        e.mem.set_u16(island + ISLAND_INDEX, count);
        e.mem.set_u8(island + ISLAND_BYTE_24, count as u8);
        push_word(e, list, island);
    }
    e.call(ISLAND_ADD_ENTITY, &args![island, entity_addr]);
    let id = e.mem.u8(entity_addr + ENTITY_UNIQUE_ID);
    e.mem.set_u8(island + ISLAND_BYTE_24, id);
}

// Translated from 00cb4740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeEntitySI` (Xbox PDB), cdecl: clears the entity's world and
/// removes it from its island. An island that is in its list, is now empty and belongs to a
/// world with island management is taken out of the list (`00cb4000`) and deleted (virtual
/// destructor, slot 0, flag 1).
pub fn hkp_world_operation_util_remove_entity_si(e: &mut Engine, world: Ptr, entity: Ptr) {
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    e.mem.set_u32(entity_addr + ENTITY_WORLD, 0);
    e.call(ISLAND_REMOVE_ENTITY, &args![island, entity_addr]);
    if e.mem.u16(island + ISLAND_INDEX) != 0xffff
        && e.mem.u32(island + ISLAND_ENTITY_COUNT) == 0
        && e.mem.u8(world.addr() + WORLD_WANT_ISLANDS) != 0
    {
        e.call(0x00cb_4000, &args![world.addr(), island]);
        e.vcall(island, 0, &args![1u32]);
    }
}

// Translated from 00cb4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: for each action of the entity that belongs
/// to the world's fixed island: appends it to `out`, adds a reference, registers it with the
/// island (`00d07130`), sets the island's "action cleanup needed" bit (flags +0x25 `& 0xdf |
/// 0x10`) and puts the island in the dirty list.
pub fn fn_00cb4790(e: &mut Engine, world: Ptr, entity: Ptr, out: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    if e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let actions = e.mem.u32(entity_addr + ENTITY_ACTIONS);
        let action = e.mem.u32(actions + index * 4);
        if e.mem.u32(action + 0xc) == e.mem.u32(world_addr + WORLD_FIXED_ISLAND) {
            push_word(e, out.addr(), action);
            e.call(ADD_REFERENCE, &args![action]);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            e.call(ISLAND_REGISTER_ACTION, &args![island, action]);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            let flags = e.mem.u8(island + ISLAND_FLAGS_25);
            e.mem
                .set_u8(island + ISLAND_FLAGS_25, (flags & 0xdf) | 0x10);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            mark_island_dirty(e, world_addr, island);
        }
        index += 1;
        if index >= e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) as u32 {
            break;
        }
    }
}

// Translated from 00cb5000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: removes all the entity's constraints
/// from its world without notifying (`removeConstraintImmediately`), holding a reference to
/// each and appending it to `out`: first the master constraints from the last to the first (the
/// instance is the first word of each 0x30-byte element), then the slave constraints.
pub fn fn_00cb5000(e: &mut Engine, entity: Ptr, out: Ptr) {
    let entity_addr = entity.addr();
    let world = e.mem.u32(entity_addr + ENTITY_WORLD);
    let mut index = e.mem.u16(entity_addr + 0xb0) as i32 - 1;
    while index >= 0 {
        let data = e.mem.u32(entity_addr + 0xac);
        let constraint = e.mem.u32(data.wrapping_add(index as u32 * 0x30));
        e.call(ADD_REFERENCE, &args![constraint]);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        push_word(e, out.addr(), constraint);
        index -= 1;
    }
    let mut index = e.mem.i32(entity_addr + 0xb8) - 1;
    while index >= 0 {
        let data = e.mem.u32(entity_addr + 0xb4);
        let constraint = e.mem.u32(data.wrapping_add(index as u32 * 4));
        e.call(ADD_REFERENCE, &args![constraint]);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        push_word(e, out.addr(), constraint);
        index -= 1;
    }
}

// Translated from 00cb5500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: marks an island for the next cleanup.
/// An island that is in the active list but not yet marked (flag byte +0x26 bits 2-3 clear,
/// bits 0-1 set) gets the "try to increase" bits (`& 0xdf | 0x10`); then it is marked
/// (`& 0xf7 | 4`) and put in the world's dirty list.
pub fn fn_00cb5500(e: &mut Engine, world: Ptr, island: Ptr) {
    let island_addr = island.addr();
    let flags = e.mem.u8(island_addr + ISLAND_FLAGS_26);
    if flags & 0xc == 0 && flags & 3 != 0 {
        e.mem
            .set_u8(island_addr + ISLAND_FLAGS_26, (flags & 0xdf) | 0x10);
    }
    let flags = e.mem.u8(island_addr + ISLAND_FLAGS_26);
    e.mem
        .set_u8(island_addr + ISLAND_FLAGS_26, (flags & 0xf7) | 4);
    mark_island_dirty(e, world.addr(), island_addr);
}

// Translated from 00cb5570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::mergeIslands` (Xbox PDB), cdecl: merges the islands of two
/// entities. While the critical-operations lock is held, a 12-byte operation record (byte
/// 0xc, then the two entities) is appended to the world's pending queue instead.
pub fn hkp_world_operation_util_merge_islands(e: &mut Engine, world: Ptr, first: Ptr, second: Ptr) {
    let world_addr = world.addr();
    if e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0 {
        let record = e.mem.alloc(12);
        e.mem.set_u8(record, 0xc);
        e.mem.set_u32(record + 4, first.addr());
        e.mem.set_u32(record + 8, second.addr());
        e.call(WORLD_ENQUEUE_OPERATION, &args![world_addr, record]);
        e.mem.free(record);
    } else {
        let first_island = e.mem.u32(first.addr() + ENTITY_ISLAND);
        let second_island = e.mem.u32(second.addr() + ENTITY_ISLAND);
        e.call(
            INTERNAL_MERGE_TWO_ISLANDS,
            &args![world_addr, first_island, second_island],
        );
    }
}

// Translated from 00cb5750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::updateEntityBP` (Xbox PDB), cdecl: re-queries the broad phase for
/// the entity's new AABB. While the critical lock is held a record (byte 3, then the entity)
/// is queued instead. Otherwise, for an entity with a shape: recalculates its AABB when
/// the new minimum (+0x30) is above the old (+0x40, unsigned), builds the integer box (the
/// byte extents at +0x3c..+0x3e and +0x4c..+0x4e shifted by the byte at +0x3f), asks the broad
/// phase (vtable slot `0x2c`) for added and removed pairs, removes the duplicates, and
/// removes and adds the pairs on the dispatcher.
pub fn hkp_world_operation_util_update_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    if e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0 {
        let record = e.mem.alloc(12);
        e.mem.set_u8(record, 3);
        e.mem.set_u32(record + 4, entity_addr);
        e.call(WORLD_ENQUEUE_OPERATION, &args![world_addr, record]);
        e.mem.free(record);
        return;
    }
    e.mem.set_u32(world_addr + WORLD_LOCK_COUNT, 1);
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) != 0 {
        // Frame: added array 0, removed array 0x10, handle slot 0x20, box 0x30 (min x, y, z,
        // spare, max x, y, z), entity slot 0x50.
        let frame = e.mem.alloc(0x60);
        let added = frame;
        let removed = frame + 0x10;
        scratch_array_create(e, added, world_addr);
        scratch_array_create(e, removed, world_addr);
        if e.mem.u32(entity_addr + 0x30) > e.mem.u32(entity_addr + 0x40) {
            e.mem.set_u32(frame + 0x50, entity_addr);
            let input = e.mem.u32(world_addr + WORLD_COLLISION_INPUT);
            e.call(ENTITY_BATCH_RECALC_AABB, &args![input, frame + 0x50, 1u32]);
        }
        let shift = e.mem.u8(entity_addr + 0x3f) as u32;
        for lane in 0..3u32 {
            let low = e.mem.u32(entity_addr + 0x30 + lane * 4);
            let low_extent = (e.mem.u8(entity_addr + 0x3c + lane) as u32).wrapping_shl(shift);
            e.mem
                .set_u32(frame + 0x30 + lane * 4, low.wrapping_sub(low_extent));
            let high = e.mem.u32(entity_addr + 0x40 + lane * 4);
            let high_extent = (e.mem.u8(entity_addr + 0x4c + lane) as u32).wrapping_shl(shift);
            e.mem
                .set_u32(frame + 0x40 + lane * 4, high_extent.wrapping_add(high));
        }
        e.mem.set_u32(frame + 0x20, collidable + 0x14);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(
            broad_phase,
            0x2c,
            &args![frame + 0x20, frame + 0x30, 1u32, added, removed],
        );
        let total = e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4));
        if total > 0 {
            e.call(DISPATCHER_REMOVE_DUPLICATES, &args![added, removed]);
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let removed_data = e.mem.u32(removed);
            let removed_size = e.mem.u32(removed + 4);
            e.call(
                DISPATCHER_REMOVE_PAIRS,
                &args![dispatcher, removed_data, removed_size],
            );
            let filter = collision_filter_interface(e, world_addr);
            let added_data = e.mem.u32(added);
            let added_size = e.mem.u32(added + 4);
            e.call(
                DISPATCHER_ADD_PAIRS,
                &args![dispatcher, added_data, added_size, filter],
            );
        }
        scratch_array_destroy(e, removed);
        scratch_array_destroy(e, added);
        e.mem.free(frame);
    }
    release_critical_lock(e, world_addr);
}

// Translated from 00cb59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addEntityBP` (Xbox PDB), cdecl: for an entity with a shape,
/// recalculates its AABB, asks the broad phase (vtable slot `0x14`) for the new pairs with a
/// copy of the AABB (32 bytes from +0x30) and adds them on the dispatcher.
pub fn hkp_world_operation_util_add_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) == 0 {
        return;
    }
    // Frame: the pair array 0, the entity slot 0x10, the AABB copy 0x20.
    let frame = e.mem.alloc(0x40);
    scratch_array_create(e, frame, world_addr);
    e.mem.set_u32(frame + 0x10, entity_addr);
    let input = e.mem.u32(world_addr + WORLD_COLLISION_INPUT);
    e.call(ENTITY_BATCH_RECALC_AABB, &args![input, frame + 0x10, 1u32]);
    let aabb = e.mem.bytes(entity_addr + 0x30, 32);
    e.mem.write(frame + 0x20, &aabb);
    let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
    e.vcall(
        broad_phase,
        0x14,
        &args![collidable + 0x14, frame + 0x20, frame],
    );
    let count = e.mem.i32(frame + 4);
    if count > 0 {
        let filter = collision_filter_interface(e, world_addr);
        let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
        let data = e.mem.u32(frame);
        e.call(
            DISPATCHER_ADD_PAIRS,
            &args![dispatcher, data, count as u32, filter],
        );
    }
    scratch_array_destroy(e, frame);
    e.mem.free(frame);
}

// Translated from 00cb5b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addPhantomBP` (Xbox PDB), cdecl: fills an AABB buffer from the
/// phantom (vtable slot `0x18`), asks the broad phase (slot `0x10`) for the new pairs, stores
/// the AABB in the phantom (`hkpPhantom::setBoundingVolumeData`) and adds the pairs on the
/// dispatcher.
pub fn hkp_world_operation_util_add_phantom_bp(e: &mut Engine, world: Ptr, phantom: Ptr) {
    let world_addr = world.addr();
    let phantom_addr = phantom.addr();
    // Frame: the pair array 0, the AABB buffer 0x10.
    let frame = e.mem.alloc(0x40);
    scratch_array_create(e, frame, world_addr);
    e.vcall(phantom_addr, 0x18, &args![frame + 0x10]);
    let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
    e.vcall(
        broad_phase,
        0x10,
        &args![phantom_addr + 0x24, frame + 0x10, frame],
    );
    e.call(
        PHANTOM_SET_BOUNDING_VOLUME_DATA,
        &args![phantom_addr, frame + 0x10],
    );
    let count = e.mem.u32(frame + 4);
    if count != 0 {
        let filter = collision_filter_interface(e, world_addr);
        let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
        let data = e.mem.u32(frame);
        e.call(
            DISPATCHER_ADD_PAIRS,
            &args![dispatcher, data, count, filter],
        );
    }
    scratch_array_destroy(e, frame);
    e.mem.free(frame);
}

// Translated from 00cb5c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeEntityBP` (Xbox PDB), cdecl: for an entity with a shape:
/// resets its collision information through the simulation (vtable slot `0x20`), asks the
/// broad phase (slot `0x1c`) for the pairs to remove and removes them on the dispatcher.
pub fn hkp_world_operation_util_remove_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) == 0 {
        return;
    }
    // Frame: the pair array 0, the entity slot 0x10.
    let frame = e.mem.alloc(0x20);
    e.mem.set_u32(frame + 0x10, entity_addr);
    let simulation = e.mem.u32(world_addr + WORLD_SIMULATION);
    e.vcall(
        simulation,
        0x20,
        &args![frame + 0x10, 1u32, world_addr, 1u32],
    );
    scratch_array_create(e, frame, world_addr);
    let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
    e.vcall(broad_phase, 0x1c, &args![collidable + 0x14, frame]);
    let count = e.mem.i32(frame + 4);
    if count > 0 {
        let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
        let data = e.mem.u32(frame);
        e.call(
            DISPATCHER_REMOVE_PAIRS,
            &args![dispatcher, data, count as u32],
        );
    }
    scratch_array_destroy(e, frame);
    e.mem.free(frame);
}

// Translated from 00cb5d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removePhantomBP` (Xbox PDB), cdecl: asks the broad phase (vtable
/// slot `0x1c`) for the phantom's pairs and removes them on the dispatcher.
pub fn hkp_world_operation_util_remove_phantom_bp(e: &mut Engine, world: Ptr, phantom: Ptr) {
    let world_addr = world.addr();
    let frame = e.mem.alloc(0x10);
    scratch_array_create(e, frame, world_addr);
    let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
    e.vcall(broad_phase, 0x1c, &args![phantom.addr() + 0x24, frame]);
    let count = e.mem.u32(frame + 4);
    if count != 0 {
        let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
        let data = e.mem.u32(frame);
        e.call(DISPATCHER_REMOVE_PAIRS, &args![dispatcher, data, count]);
    }
    scratch_array_destroy(e, frame);
    e.mem.free(frame);
}

// Translated from 00cb5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: moves the entity's actions that involve
/// only fixed entities and this entity (each action lists its entities through vtable slot
/// `0x10` into an inline array that is not cleared between actions) to the entity's island:
/// appended to `out`, referenced, registered with the island (`00d07130`), the island's
/// "action cleanup needed" bit set and the island put in the dirty list.
pub fn fn_00cb5dd0(e: &mut Engine, world: Ptr, entity: Ptr, out: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    // The inline array (data, size, capacity-and-flags, 16 words of storage).
    let list = e.mem.alloc(0x50);
    inplace_array_init(e, list);
    if e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) != 0 {
        let mut index = 0u32;
        loop {
            let actions = e.mem.u32(entity_addr + ENTITY_ACTIONS);
            let action = e.mem.u32(actions + index * 4);
            e.vcall(action, 0x10, &args![list]);
            let count = e.mem.i32(list + 4);
            let data = e.mem.u32(list);
            let mut only_fixed_or_this = true;
            let mut position = 0i32;
            while position < count {
                let other = e.mem.u32(data.wrapping_add(position as u32 * 4));
                if e.mem.u8(other + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                    && other != entity_addr
                {
                    only_fixed_or_this = false;
                    break;
                }
                position += 1;
            }
            if only_fixed_or_this {
                push_word(e, out.addr(), action);
                e.call(ADD_REFERENCE, &args![action]);
                e.call(ISLAND_REGISTER_ACTION, &args![island, action]);
                let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                e.mem
                    .set_u8(island + ISLAND_FLAGS_25, (flags & 0xdf) | 0x10);
                mark_island_dirty(e, world_addr, island);
            }
            index += 1;
            if index >= e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) as u32 {
                break;
            }
        }
    }
    array_free_storage(e, list, 4);
    e.mem.free(list);
}

// Translated from 00cb5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addActionsToEntitysIsland` (Xbox PDB), cdecl: adds each action of
/// the array to the entity's island (`hkpSimulationIsland::addAction`) and drops the
/// reference the array held. Unless the island is the world's fixed island, each action also
/// lists its entities (vtable slot `0x10`) and every other non-fixed entity in a different island
/// than this entity's is merged with it (`mergeIslands`).
pub fn hkp_world_operation_util_add_actions_to_entitys_island(
    e: &mut Engine,
    world: Ptr,
    entity: Ptr,
    actions: Ptr,
) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    let list = e.mem.alloc(0x50);
    inplace_array_init(e, list);
    if e.mem.i32(actions.addr() + 4) > 0 {
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(actions.addr());
            let action = e.mem.u32(data + index * 4);
            let fixed_island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            e.call(ISLAND_ADD_ACTION, &args![island, action]);
            e.call(REMOVE_REFERENCE, &args![action]);
            if island != fixed_island {
                e.mem.set_u32(list + 4, 0);
                e.vcall(action, 0x10, &args![list]);
                let mut position = 0i32;
                while position < e.mem.i32(list + 4) {
                    let other = e.mem.u32(e.mem.u32(list).wrapping_add(position as u32 * 4));
                    if other != entity_addr
                        && e.mem.u8(other + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                        && e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                        && e.mem.u32(other + ENTITY_ISLAND)
                            != e.mem.u32(entity_addr + ENTITY_ISLAND)
                    {
                        let other_world = e.mem.u32(other + ENTITY_WORLD);
                        e.call(0x00cb_5570, &args![other_world, other, entity_addr]);
                    }
                    position += 1;
                }
            }
            index += 1;
            if index as i32 >= e.mem.i32(actions.addr() + 4) {
                break;
            }
        }
    }
    array_free_storage(e, list, 4);
    e.mem.free(list);
}

// Translated from 00cb6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::setRigidBodyMotionType` (Xbox PDB), cdecl: `mode` goes to
/// `addEntitySI` (1 selects the sparse active list; its exact meaning is unconfirmed) and
/// `filter_update` to `hkpWorld::updateCollisionFilterOnEntity`.
///
/// Nothing happens when the type is unchanged, or when a fixed or keyframed entity is
/// switched to a dynamic type and has no saved dynamic motion (+0x1f8).
/// Leaving a fixed motion first sorts the collidable's entries and the entity's slave
/// constraints. The entity is held by a reference during the change. Without a world, or when
/// both or neither of the old and new type are fixed, only the motion object is replaced
/// (`00cb4230`) and the collision filter updated. Otherwise the entity is taken out of
/// the simulation: constraints removed (`00cb5000`), actions collected (`00cb5dd0` or
/// `00cb4790`), the agents to fixed partners moved (`00cb3ee0`), the island flagged
/// when it has more than 2 entities, the entity removed from its island (`00cb4740`), the
/// motion replaced, the entity re-added (`00cb4640`), actions re-added (`00cb5f50`),
/// constraints re-added (`00cb3ca0`, no notification), islands merged with the partners of
/// the sorted collision entries (unless fixed), the agent track appended to the island's, and
/// for a fixed entity the motion state frozen and the entity collided in the broad phase
/// (simulation slot `0x20`, then `collideEntitiesBroadPhaseDiscrete` and
/// `fireInactiveEntityMoved`). The pending operations then run if nothing holds the lock.
pub fn hkp_world_operation_util_set_rigid_body_motion_type(
    e: &mut Engine,
    entity: Ptr,
    new_type: u32,
    mode: u32,
    filter_update: u32,
) {
    let entity_addr = entity.addr();
    let old_type = e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32;
    if old_type == new_type {
        return;
    }
    if old_type == MOTION_FIXED {
        e.call(
            LINKED_COLLIDABLE_SORT_ENTRIES,
            &args![entity_addr + ENTITY_COLLIDABLE],
        );
        e.call(ENTITY_SORT_CONSTRAINTS_SLAVES, &args![entity_addr]);
    }
    let new_is_dynamic = !(new_type == MOTION_FIXED || new_type == MOTION_KEYFRAMED);
    let old_is_dynamic = !(old_type == MOTION_FIXED || old_type == MOTION_KEYFRAMED);
    if new_is_dynamic && !old_is_dynamic && e.mem.u32(entity_addr + 0x1f8) == 0 {
        return;
    }
    e.call(ADD_REFERENCE, &args![entity_addr]);
    let world = e.mem.u32(entity_addr + ENTITY_WORLD);
    if world == 0 || (old_type == MOTION_FIXED) == (new_type == MOTION_FIXED) {
        e.call(
            0x00cb_4230,
            &args![
                entity_addr,
                new_type,
                new_is_dynamic as u32,
                old_is_dynamic as u32,
                world
            ],
        );
        if world != 0 {
            e.call(
                WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                &args![world, entity_addr, filter_update, 1u32],
            );
        }
    } else {
        // Frame: the constraints array 0 (inline, 16 words from 0xc), the actions array 0x50
        // (inline, from 0x5c), the agent track 0xa0 (0x200, then an inline array of one word
        // from 0xa4), the collision entries array 0xc0, the entity slot 0xd0.
        let frame = e.mem.alloc(0x100);
        let constraints = frame;
        let actions = frame + 0x50;
        let track = frame + 0xa0;
        let entries = frame + 0xc0;
        inplace_array_init(e, constraints);
        inplace_array_init(e, actions);
        e.mem.set_u32(track, 0x200);
        e.mem.set_u32(track + 4, track + 0x10);
        e.mem.set_u32(track + 8, 0);
        e.mem.set_u32(track + 0xc, 0x8000_0001);
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 1);
        e.mem.set_u8(world + WORLD_CRITICAL_ALLOWED, 0);
        e.call(0x00cb_5000, &args![entity_addr, constraints]);
        if new_type == MOTION_FIXED {
            e.call(0x00cb_5dd0, &args![world, entity_addr, actions]);
        } else {
            e.call(0x00cb_4790, &args![world, entity_addr, actions]);
        }
        let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
        e.call(
            0x00cb_3ee0,
            &args![island + ISLAND_AGENT_TRACK, entity_addr, track, new_type],
        );
        if old_type != MOTION_FIXED {
            let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
            if e.mem.i32(island + ISLAND_ENTITY_COUNT) > 2 {
                let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                e.mem.set_u8(island + ISLAND_FLAGS_25, (flags & 0xfd) | 1);
            }
        }
        e.call(0x00cb_4740, &args![world, entity_addr]);
        e.call(
            0x00cb_4230,
            &args![
                entity_addr,
                new_type,
                new_is_dynamic as u32,
                old_is_dynamic as u32,
                world
            ],
        );
        e.call(0x00cb_4640, &args![world, entity_addr, mode]);
        e.call(0x00cb_5f50, &args![world, entity_addr, actions]);
        e.mem.set_u8(world + WORLD_CRITICAL_ALLOWED, 1);
        e.mem.set_u32(entries, 0);
        e.mem.set_u32(entries + 4, 0);
        e.mem.set_u32(entries + 8, 0x8000_0000);
        let mut index = 0i32;
        while index < e.mem.i32(constraints + 4) {
            let data = e.mem.u32(constraints);
            let constraint = e.mem.u32(data.wrapping_add(index as u32 * 4));
            e.call(0x00cb_3ca0, &args![world, constraint, 0u32]);
            e.call(REMOVE_REFERENCE, &args![constraint]);
            index += 1;
        }
        if new_type != MOTION_FIXED {
            e.call(
                LINKED_COLLIDABLE_GET_ENTRIES_SORTED,
                &args![entity_addr + ENTITY_COLLIDABLE, entries],
            );
            let mut index = 0i32;
            while index < e.mem.i32(entries + 4) {
                let data = e.mem.u32(entries);
                let partner = e.mem.u32(data.wrapping_add(index as u32 * 8 + 4));
                let owner = partner.wrapping_add(e.mem.i8(partner + 0x10) as i32 as u32);
                if e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                    && e.mem.u8(owner + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                    && e.mem.u32(entity_addr + ENTITY_ISLAND) != e.mem.u32(owner + ENTITY_ISLAND)
                {
                    let entity_world = e.mem.u32(entity_addr + ENTITY_WORLD);
                    e.call(0x00cb_5570, &args![entity_world, entity_addr, owner]);
                }
                index += 1;
            }
        }
        let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
        e.call(
            AGENT_APPEND_TRACK,
            &args![island + ISLAND_AGENT_TRACK, track],
        );
        if new_type == MOTION_FIXED {
            let step_time = x87_quiet_bits(e.mem.u32(world + WORLD_STEP_TIME));
            e.call(
                FREEZE_MOTION_STATE,
                &args![step_time, entity_addr + ENTITY_MOTION_STATE],
            );
            add_word(e, world + WORLD_LOCK_COUNT, 1);
            e.mem.set_u32(frame + 0xd0, entity_addr);
            let simulation = e.mem.u32(world + WORLD_SIMULATION);
            e.vcall(simulation, 0x20, &args![frame + 0xd0, 1u32, world, 7u32]);
            e.call(
                SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                &args![frame + 0xd0, 1u32, world],
            );
            let entity_world = e.mem.u32(entity_addr + ENTITY_WORLD);
            e.call(
                FIRE_INACTIVE_ENTITY_MOVED,
                &args![entity_world, entity_addr],
            );
            add_word(e, world + WORLD_LOCK_COUNT, u32::MAX);
        }
        e.call(
            WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
            &args![world, entity_addr, filter_update, 1u32],
        );
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 0);
        if e.mem.u32(world + WORLD_LOCK_COUNT) == 0 {
            run_pending_operations(e, world);
        }
        array_free_storage(e, entries, 8);
        array_free_storage(e, track + 4, 4);
        array_free_storage(e, actions, 4);
        array_free_storage(e, constraints, 4);
        e.mem.free(frame);
    }
    e.call(REMOVE_REFERENCE, &args![entity_addr]);
}

// Translated from 00cb7460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), `RET 4`: answers the word at +0 when `key + 1`
/// equals the word at +8, otherwise 0x200.
pub fn fn_00cb7460(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    if key.wrapping_add(1) == e.mem.u32(this.addr() + 8) {
        e.mem.u32(this.addr())
    } else {
        0x200
    }
}

// Translated from 00cb74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintOwner::addConstraintInfo` (Xbox PDB), `RET 8`: for a constraint with an
/// internal block (+0x30) folds the four-word `info` into the owner (`this`): the maximum
/// at +8 of the first and second word, sums of the second, third and fourth word at +0xc,
/// +0x10 and +0x14; adds them to the internal block's 16-bit counters (+0x14, +0x16, +0x18);
/// and finally raises +8 to the block's +0x14 counter.
pub fn hkp_constraint_owner_add_constraint_info(
    e: &mut Engine,
    this: Ptr,
    constraint: Ptr,
    info: Ptr,
) {
    let internal = e.mem.u32(constraint.addr() + 0x30);
    if internal == 0 {
        return;
    }
    let owner = this.addr();
    let info = info.addr();
    let first = e.mem.i32(info);
    let current = e.mem.i32(owner + 8);
    let mut maximum = if current > first { current } else { first };
    e.mem.set_i32(owner + 8, maximum);
    let second = e.mem.i32(info + 4);
    if maximum <= second {
        maximum = second;
    }
    e.mem.set_i32(owner + 8, maximum);
    let (second_word, third_word, fourth_word) = (
        e.mem.u32(info + 4),
        e.mem.u32(info + 8),
        e.mem.u32(info + 0xc),
    );
    add_word(e, owner + 0xc, second_word);
    add_word(e, owner + 0x10, third_word);
    add_word(e, owner + 0x14, fourth_word);
    let fourth_sum = (e.mem.u16(internal + 0x18) as u32).wrapping_add(e.mem.u32(info + 0xc));
    let second_sum = (e.mem.u16(internal + 0x14) as u32).wrapping_add(e.mem.u32(info + 4));
    let third = e.mem.u16(info + 8);
    let counter = e.mem.u16(internal + 0x16).wrapping_add(third);
    e.mem.set_u16(internal + 0x16, counter);
    e.mem.set_u16(internal + 0x18, fourth_sum as u16);
    e.mem.set_u16(internal + 0x14, second_sum as u16);
    let counter = e.mem.u16(internal + 0x14) as i32;
    let current = e.mem.i32(owner + 8);
    e.mem
        .set_i32(owner + 8, if current > counter { current } else { counter });
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATOR_VTABLE: u32 = 0x0200_0000;
    const ALLOCATE: u32 = 0x0200_0100;
    const FREE: u32 = 0x0200_0110;
    const OBJECT_VTABLE: u32 = 0x0200_0200;
    const OBJECT_SLOT_10: u32 = 0x0200_0300;
    const OBJECT_SLOT_14: u32 = 0x0200_0304;
    const OBJECT_SLOT_18: u32 = 0x0200_0308;
    const OBJECT_SLOT_1C: u32 = 0x0200_030c;
    const OBJECT_SLOT_20: u32 = 0x0200_0310;
    const OBJECT_SLOT_2C: u32 = 0x0200_0314;
    const OBJECT_SLOT_64: u32 = 0x0200_0318;
    const OBJECT_SLOT_6C: u32 = 0x0200_031c;

    /// An engine with the memory router, an allocator and the array-grow function doubled.
    /// Returns the engine and the router object.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, ALLOCATE, 0, 0, FREE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(FREE, |_, _| 0u32.into_ret());
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = ((e.mem.u32(array + 8) & 0x3fff_ffff) * 2).max(4);
            let fresh = e.mem.alloc(capacity * element);
            let old = e.mem.bytes(e.mem.u32(array), size * element);
            e.mem.write(fresh, &old);
            e.mem.set_u32(array, fresh);
            e.mem.set_u32(array + 8, capacity);
            0u32.into_ret()
        });
        // A generic object whose slots are all recorded.
        let mut words = vec![0u32; 28];
        words[0x10 / 4] = OBJECT_SLOT_10;
        words[0x14 / 4] = OBJECT_SLOT_14;
        words[0x18 / 4] = OBJECT_SLOT_18;
        words[0x1c / 4] = OBJECT_SLOT_1C;
        words[0x20 / 4] = OBJECT_SLOT_20;
        words[0x2c / 4] = OBJECT_SLOT_2C;
        words[0x64 / 4] = OBJECT_SLOT_64;
        words[0x6c / 4] = OBJECT_SLOT_6C;
        e.put_vtable(OBJECT_VTABLE, &words);
        for slot in [
            OBJECT_SLOT_10,
            OBJECT_SLOT_14,
            OBJECT_SLOT_18,
            OBJECT_SLOT_1C,
            OBJECT_SLOT_20,
            OBJECT_SLOT_2C,
            OBJECT_SLOT_64,
            OBJECT_SLOT_6C,
        ] {
            e.register(slot, |_, _| 0u32.into_ret());
        }
        (e, router)
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| 0u32.into_ret());
        }
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

    /// Order of the logged calls whose address is in `wanted`.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| wanted.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    /// A zeroed block holding an object whose vtable is [`OBJECT_VTABLE`].
    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, OBJECT_VTABLE);
        block
    }

    /// An `hkArray` header with `values` as its word elements.
    fn word_array(e: &mut Engine, header: u32, values: &[u32]) {
        let data = e.mem.alloc(values.len() as u32 * 4 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + i as u32 * 4, *v);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, values.len() as u32);
        e.mem.set_u32(header + 8, values.len() as u32 | 0x8000_0000);
    }

    fn array_words(e: &Engine, header: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..e.mem.u32(header + 4))
            .map(|i| e.mem.u32(data + i * 4))
            .collect()
    }

    const PENDING: [u32; 2] = [
        WORLD_RUN_PENDING_OPERATIONS,
        WORLD_RUN_PENDING_BODY_OPERATIONS,
    ];

    #[test]
    fn add_constraint_runs_pending_operations_after_the_lock() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                WORLD_CONSTRAINT_UTIL_ADD,
                FIRE_CONSTRAINT_ADDED,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 3);
        e.mem.set_u32(world + WORLD_PENDING_QUEUE_COUNT, 1);
        e.mem.set_u32(world + WORLD_PENDING_BODY_OPERATIONS, 2);
        let returned = e.call(0x00cb_3ca0, &args![world, 0x5000u32, 1u32]).u32();
        assert_eq!(returned, 0x5000);
        assert_eq!(
            calls(&e, WORLD_CONSTRAINT_UTIL_ADD),
            vec![vec![world, 0x5000]]
        );
        assert_eq!(calls(&e, FIRE_CONSTRAINT_ADDED), vec![vec![world, 0x5000]]);
        assert_eq!(
            order(
                &e,
                &[
                    WORLD_CONSTRAINT_UTIL_ADD,
                    FIRE_CONSTRAINT_ADDED,
                    PENDING[0],
                    PENDING[1]
                ]
            ),
            vec![
                WORLD_CONSTRAINT_UTIL_ADD,
                FIRE_CONSTRAINT_ADDED,
                PENDING[0],
                PENDING[1]
            ]
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);

        // Already locked by the caller: no notification, nothing run, the lock stays.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 2);
        e.call(0x00cb_3ca0, &args![world, 0x5000u32, 0u32]);
        assert!(calls(&e, FIRE_CONSTRAINT_ADDED).is_empty());
        assert!(calls(&e, PENDING[0]).is_empty());
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 2);

        // Blocked pending operations are not run.
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 0);
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 1);
        e.call(0x00cb_3ca0, &args![world, 0x5000u32, 0u32]);
        assert!(calls(&e, PENDING[0]).is_empty());
    }

    #[test]
    fn find_small_sparse_island_picks_a_marked_small_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_MIN_ISLAND_SIZE, 10);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let islands: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x6c)).collect();
        // Island 2 (last) is the skipped one, island 1 is too big, island 0 qualifies.
        e.mem.set_u32(islands[0] + ISLAND_ENTITY_COUNT, 2);
        e.mem.set_u32(islands[0] + ISLAND_CONSTRAINT_COUNT, 3);
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 4);
        e.mem.set_u32(islands[1] + ISLAND_ENTITY_COUNT, 20);
        e.mem.set_u8(islands[1] + ISLAND_FLAGS_26, 4);
        e.mem.set_u8(islands[2] + ISLAND_FLAGS_26, 4);
        let array = e.mem.alloc(12);
        word_array(&mut e, array, &islands);
        let found = e
            .call(0x00cb_3d10, &args![world, array, 4i32, islands[2]])
            .u32();
        assert_eq!(found, islands[0]);
        assert_eq!(e.mem.u8(islands[0] + ISLAND_FLAGS_25), 4);
        // An island without the active mark is passed over.
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 0);
        let found = e
            .call(0x00cb_3d10, &args![world, array, 4i32, islands[2]])
            .u32();
        assert_eq!(found, 0);
        // The requested extra size at the minimum, or no island management: nothing.
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 4);
        assert_eq!(
            e.call(0x00cb_3d10, &args![world, array, 10i32, 0u32]).u32(),
            0
        );
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 0);
        assert_eq!(
            e.call(0x00cb_3d10, &args![world, array, 4i32, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn remove_constraint_takes_a_lock_only_when_none_is_held() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                WORLD_CONSTRAINT_UTIL_REMOVE,
                FIRE_CONSTRAINT_REMOVED,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_CONSTRAINT_LISTENERS_SIZE, 1);
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 1);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(owner + ISLAND_FLAGS_25, 0xff);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 8, owner);
        e.call(0x00cb_3dd0, &args![world, constraint, 1u32]);
        assert_eq!(e.mem.u8(owner + ISLAND_FLAGS_25), 0xfd | 1);
        assert_eq!(
            order(
                &e,
                &[
                    FIRE_CONSTRAINT_REMOVED,
                    WORLD_CONSTRAINT_UTIL_REMOVE,
                    PENDING[0]
                ]
            ),
            vec![
                FIRE_CONSTRAINT_REMOVED,
                WORLD_CONSTRAINT_UTIL_REMOVE,
                PENDING[0]
            ]
        );
        assert_eq!(
            calls(&e, WORLD_CONSTRAINT_UTIL_REMOVE),
            vec![vec![constraint]]
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);

        // Under a lock: removal only, the lock count untouched, no notification when not asked.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 3);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        assert!(calls(&e, FIRE_CONSTRAINT_REMOVED).is_empty());
        assert!(calls(&e, PENDING[0]).is_empty());
        assert_eq!(calls(&e, WORLD_CONSTRAINT_UTIL_REMOVE).len(), 1);
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 3);

        // Under a lock with a notification and listeners.
        e.call(0x00cb_3dd0, &args![world, constraint, 1u32]);
        assert_eq!(calls(&e, FIRE_CONSTRAINT_REMOVED).len(), 1);
    }

    #[test]
    fn remove_attached_agents_moves_agents_to_the_fixed_tracks() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[AGENT_COPY_AND_RELINK_ENTRY, AGENT_DEALLOCATE_ENTRY],
        );
        let entity = e.mem.alloc(0x220);
        let entity_island = e.mem.alloc(0x6c);
        e.mem.set_u32(entity + ENTITY_ISLAND, entity_island);
        // Two partners: a fixed one and a dynamic one in another island.
        let fixed = e.mem.alloc(0x220);
        e.mem.set_u8(fixed + ENTITY_MOTION_TYPE, 5);
        let dynamic = e.mem.alloc(0x220);
        e.mem.set_u8(dynamic + ENTITY_MOTION_TYPE, 1);
        let partner_island = e.mem.alloc(0x6c);
        e.mem.set_u32(dynamic + ENTITY_ISLAND, partner_island);
        // Each partner's collidable is at +0x10 with the owner offset byte (-0x10) at +0x10.
        for partner in [fixed, dynamic] {
            e.mem.set_i8(partner + 0x20, -0x10);
        }
        let entries = e.mem.alloc(12);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, fixed + 0x10);
        e.mem.set_u32(data + 8, 0xa2);
        e.mem.set_u32(data + 12, dynamic + 0x10);
        e.mem.set_u32(entries, data);
        e.mem.set_u32(entries + 4, 2);
        e.register_double(LINKED_COLLIDABLE_ENTRIES, move |_, _| entries.into_ret());
        e.register_double(AGENT_UTIL_GET_ISLAND_FROM_ENTRY, move |_, _| {
            entity_island.into_ret()
        });
        e.call(
            0x00cb_3ee0,
            &args![0x7000u32, entity, 0x8000u32, MOTION_FIXED],
        );
        assert_eq!(
            calls(&e, AGENT_COPY_AND_RELINK_ENTRY),
            vec![vec![0x8000, 0xa1], vec![partner_island + 0x58, 0xa2]]
        );
        assert_eq!(
            calls(&e, AGENT_DEALLOCATE_ENTRY),
            vec![vec![0x7000, 0xa1], vec![0x7000, 0xa2]]
        );
        assert_eq!(
            calls(&e, AGENT_UTIL_GET_ISLAND_FROM_ENTRY),
            vec![vec![0xa2, entity_island, partner_island]]
        );
        // Not turning fixed: the dynamic partner is left alone.
        e.call_log = Some(Vec::new());
        e.call(0x00cb_3ee0, &args![0x7000u32, entity, 0x8000u32, 1u32]);
        assert_eq!(calls(&e, AGENT_COPY_AND_RELINK_ENTRY).len(), 1);
    }

    #[test]
    fn remove_island_from_dirty_list_nulls_the_slot() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &[0x11, island, 0x33]);
        e.mem.set_u32(world + WORLD_DIRTY_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_DIRTY_ISLANDS + 4, 3);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 1);
        e.call(0x00cb_3fd0, &args![world, island]);
        assert_eq!(array_words(&e, list), vec![0x11, 0, 0x33]);
        assert_eq!(e.mem.u16(island + ISLAND_DIRTY_INDEX), 0xffff);
        // Not in the list: nothing changes.
        e.call(0x00cb_3fd0, &args![world, island]);
        assert_eq!(array_words(&e, list), vec![0x11, 0, 0x33]);
    }

    #[test]
    fn fn_00cb4000_fills_the_hole_with_the_last_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let islands: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x6c)).collect();
        for (i, island) in islands.iter().enumerate() {
            e.mem.set_u16(island + ISLAND_INDEX, i as u16);
            e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        }
        // Inactive list (flag bits 0-1 clear): remove island 0.
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &islands);
        e.mem
            .set_u32(world + WORLD_INACTIVE_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_INACTIVE_ISLANDS + 4, 3);
        e.call(0x00cb_4000, &args![world, islands[0]]);
        assert_eq!(
            array_words(&e, world + WORLD_INACTIVE_ISLANDS)[..2],
            [islands[2], islands[1]]
        );
        assert_eq!(e.mem.u32(world + WORLD_INACTIVE_ISLANDS + 4), 2);
        assert_eq!(e.mem.u16(islands[2] + ISLAND_INDEX), 0);
        // Active list: the island's flag selects it.
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &islands[..2]);
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 2);
        e.mem.set_u8(islands[1] + ISLAND_FLAGS_26, 1);
        e.mem.set_u16(islands[1] + ISLAND_INDEX, 1);
        e.call(0x00cb_4000, &args![world, islands[1]]);
        assert_eq!(e.mem.u32(world + WORLD_ACTIVE_ISLANDS + 4), 1);
    }

    /// Motion fixture: an entity with an embedded motion whose vtable is the generic object.
    fn motion_entity(e: &mut Engine) -> u32 {
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_MOTION, OBJECT_VTABLE);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        e.mem.write(entity + 0xe4, &[0x11, 0x22, 0x33, 0x44]);
        entity
    }

    const MOTION_DOUBLES: [u32; 11] = [
        KEYFRAMED_MOTION_DESTRUCTOR,
        MOTION_STATE_ASSIGN,
        MOTION_COPY_STATE_AND_VELOCITIES,
        MOTION_GET_MASS,
        RIGID_BODY_CREATE_DYNAMIC_MOTION,
        FREEZE_MOTION_STATE,
        ENTITY_SET_CACHED_SHAPE_DATA,
        WORLD_SET_MOTION_FLAGS,
        MEM_SET,
        REMOVE_REFERENCE,
        WORLD_GET_CURRENT_TIME,
    ];

    #[test]
    fn replace_motion_object_to_fixed_freezes_the_state_and_keeps_the_old_motion() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(FIXED_MOTION_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(WORLD_GET_CURRENT_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let entity = motion_entity(&mut e);
        e.mem.set_u8(entity + ENTITY_BYTE_2A, 0xfe);
        e.mem.set_u8(entity + ENTITY_MOTION + 9, 0x77);
        e.mem.set_f32(entity + ENTITY_MOTION + 0x6c, 4.0);
        e.mem.set_f32(entity + ENTITY_MOTION + 0x5c, 10.0);
        e.mem.set_u32(entity + 0xa4, 9);
        let world = e.mem.alloc(0x350);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u8(world + 0x307, 3);
        // Fixed, from a dynamic motion, with a world: frozen at the world's time.
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 1u32, world]);
        assert_eq!(
            calls(&e, FIXED_MOTION_CONSTRUCTOR),
            vec![vec![entity + 0xe0, entity + 0x120, entity + 0x160]]
        );
        let freeze = calls(&e, FREEZE_MOTION_STATE);
        assert_eq!(freeze, vec![vec![2.5f32.to_bits(), entity + 0xf0]]);
        assert_eq!(e.mem.u8(entity + ENTITY_MOTION + 9), 0x77);
        assert_eq!(e.mem.u32(entity + 0xa4), 0);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 0);
        assert_eq!(e.mem.u32(entity + 0x18), entity + 0xf0);
        assert_eq!(
            calls(&e, WORLD_SET_MOTION_FLAGS),
            vec![vec![entity + 0xe0, 1, 2, 3]]
        );
        // The old motion was copied to a heap block hung off the new one at +0x118 with size
        // and reference count in its header.
        let block = e.mem.u32(entity + ENTITY_MOTION + 0x118);
        assert_eq!(e.mem.u16(block + 4), 0x120);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), OBJECT_VTABLE);
        assert_eq!(e.mem.u16(entity + ENTITY_MOTION + 0x11c), 0xfffe);
        assert_eq!(calls(&e, ENTITY_SET_CACHED_SHAPE_DATA).len(), 1);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).len(), 1);

        // Without a world the freeze time is 1 / rate + start; no flags call.
        e.call_log = Some(Vec::new());
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 0u32, 0u32]);
        let expected = (1.0f64 / 4.0 + 10.0) as f32;
        assert_eq!(
            calls(&e, FREEZE_MOTION_STATE),
            vec![vec![expected.to_bits(), entity + 0xf0]]
        );
        assert!(calls(&e, WORLD_SET_MOTION_FLAGS).is_empty());
        // A zero rate means no freeze.
        e.call_log = Some(Vec::new());
        e.mem.set_f32(entity + ENTITY_MOTION + 0x6c, 0.0);
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 0u32, 0u32]);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
    }

    #[test]
    fn replace_motion_object_to_keyframed_hands_on_the_saved_motion() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(KEYFRAMED_MOTION_CONSTRUCTOR, |_, a| a[0].into_ret());
        let entity = motion_entity(&mut e);
        e.mem.set_u32(entity + 0x1f8, 0xabcd);
        e.mem.set_u16(entity + 0x1fc, 0x1234);
        e.call(0x00cb_4230, &args![entity, 4u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_CONSTRUCTOR).len(), 1);
        assert_eq!(calls(&e, MOTION_COPY_STATE_AND_VELOCITIES).len(), 1);
        assert_eq!(calls(&e, OBJECT_SLOT_6C), vec![vec![entity + 0xe0, 0xabcd]]);
        assert_eq!(e.mem.u16(entity + ENTITY_MOTION + 0x11c), 0x1234);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 1);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
    }

    #[test]
    fn replace_motion_object_to_dynamic_restores_or_rebuilds() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(RIGID_BODY_CREATE_DYNAMIC_MOTION, |_, _| 0u32.into_ret());
        e.register(MOTION_GET_MASS, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        // The saved motion block and the table entries.
        let entity = motion_entity(&mut e);
        e.mem.set_u32(MOTION_FLOAT_TABLE + 4, 1.5f32.to_bits());
        e.mem.set_u32(MOTION_FLOAT_TABLE + 8, 2.5f32.to_bits());
        // From a non-dynamic type: restore the motion saved at +0x1f8.
        let saved = e.mem.alloc(0x120);
        e.mem.set_u32(saved, OBJECT_VTABLE);
        e.mem.set_u8(saved + 8, 2);
        e.mem.set_u8(saved + 9, 0x55);
        e.mem.set_u16(saved + 0xa, 0x99);
        e.mem.set_u8(entity + ENTITY_MOTION + 0x11c, 7);
        e.mem.set_u32(entity + 0x1f8, saved);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 4);
        e.call(0x00cb_4230, &args![entity, 2u32, 1u32, 0u32, 0u32]);
        assert_eq!(calls(&e, OBJECT_SLOT_64), vec![vec![entity + 0xe0, saved]]);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 7);
        assert_eq!(e.mem.u8(entity + ENTITY_MOTION_TYPE), 2);
        assert_eq!(e.mem.u16(entity + 0xea), 0);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![saved]]);
        assert_eq!(calls(&e, MEM_SET), vec![vec![entity + 0x200, 0, 0]]);
        assert_eq!(calls(&e, ENTITY_SET_CACHED_SHAPE_DATA).len(), 1);
        // Type 2 is now the entity's type: nothing is rebuilt.
        assert!(calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION).is_empty());

        // From a dynamic type to another one: mass and inertia rebuild.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION + 0xbc, 2);
        e.mem.set_u8(entity + ENTITY_MOTION + 0xbd, 1);
        e.mem.set_f32(entity + ENTITY_MOTION + 0xb4, 0.25);
        e.mem.set_f32(entity + ENTITY_MOTION + 0xb8, 0.5);
        e.call(0x00cb_4230, &args![entity, 3u32, 1u32, 1u32, 0u32]);
        let mass = calls(&e, MOTION_GET_MASS);
        assert_eq!(mass.len(), 1);
        assert_eq!(mass[0][3], 2.5f32.to_bits());
        assert_eq!(mass[0][4], 1.5f32.to_bits());
        assert_eq!(mass[0][5], entity + 0xe0);
        let create = calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION);
        assert_eq!(create.len(), 1);
        assert_eq!(create[0][0], 3);
        assert_eq!(create[0][3], 3.0f32.to_bits());
        assert_eq!(calls(&e, OBJECT_SLOT_14).len(), 1);
        assert_eq!(e.mem.f32(entity + 0x194), 0.25);
        assert_eq!(e.mem.f32(entity + 0x198), 0.5);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).len(), 1);

        // The same type, or the generic type 1: unchanged.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 3);
        e.call(0x00cb_4230, &args![entity, 3u32, 1u32, 1u32, 0u32]);
        e.call(0x00cb_4230, &args![entity, 1u32, 1u32, 1u32, 0u32]);
        assert!(calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION).is_empty());
        assert!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).is_empty());
    }

    /// A world with one active island and an island vtable of recorded calls.
    fn island_world(e: &mut Engine) -> (u32, u32) {
        let world = e.mem.alloc(0x350);
        let fixed = e.mem.alloc(0x6c);
        e.mem.set_u32(world + WORLD_FIXED_ISLAND, fixed);
        (world, fixed)
    }

    #[test]
    fn add_entity_si_picks_the_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ENTITY, 0x00cb_3d10]);
        let (world, fixed) = island_world(&mut e);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_UNIQUE_ID, 0x42);
        // A fixed entity goes to the fixed island and gets no id byte.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.call(0x00cb_4640, &args![world, entity, 0u32]);
        assert_eq!(e.mem.u32(entity + ENTITY_WORLD), world);
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![fixed, entity]]);
        assert_eq!(e.mem.u8(fixed + ISLAND_BYTE_24), 0);

        // Without island management: the first active island.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        let first = e.mem.alloc(0x6c);
        let active = e.mem.alloc(12);
        word_array(&mut e, active, &[first]);
        e.mem
            .set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(active));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 1);
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![first, entity]]);
        assert_eq!(e.mem.u8(first + ISLAND_BYTE_24), 0x42);

        // Island management, mode 1 with a small sparse island found: no id byte.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let small = e.mem.alloc(0x6c);
        e.register_double(0x00cb_3d10, move |_, _| small.into_ret());
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(
            calls(&e, 0x00cb_3d10),
            vec![vec![world, world + 0x28, 4, 0]]
        );
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![small, entity]]);
        assert_eq!(e.mem.u8(small + ISLAND_BYTE_24), 0);
    }

    #[test]
    fn add_entity_si_builds_a_new_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ENTITY]);
        let (world, _) = island_world(&mut e);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let built = e.mem.alloc(0x6c);
        e.register_double(ISLAND_CONSTRUCTOR, move |_, _| built.into_ret());
        e.register(0x00cb_3d10, |_, _| 0u32.into_ret());
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_UNIQUE_ID, 0x21);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // Mode 1: a sparse island appended to the active list.
        let active = e.mem.alloc(12);
        word_array(&mut e, active, &[0x1111, 0x2222]);
        e.mem
            .set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(active));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 2);
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 8, 2);
        e.mem.set_u8(built + ISLAND_FLAGS_26, 0xff);
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(calls(&e, ISLAND_CONSTRUCTOR).len(), 1);
        assert_eq!(calls(&e, ISLAND_CONSTRUCTOR)[0][1], world);
        // 4 | (0xff & 0xf3), then & 0xfc, | 1.
        assert_eq!(e.mem.u8(built + ISLAND_FLAGS_26), 0xf5);
        assert_eq!(e.mem.u16(built + ISLAND_INDEX), 2);
        assert_eq!(
            array_words(&e, world + WORLD_ACTIVE_ISLANDS),
            vec![0x1111, 0x2222, built]
        );
        assert_eq!(e.mem.u16(calls(&e, ISLAND_CONSTRUCTOR)[0][0] + 4), 0x6c);
        assert_eq!(e.mem.u8(built + ISLAND_BYTE_24), 0x21);

        // Mode 0: appended to the inactive list; the flag bits are 0.
        e.mem.set_u8(built + ISLAND_FLAGS_26, 0xff);
        e.call(0x00cb_4640, &args![world, entity, 0u32]);
        assert_eq!(e.mem.u8(built + ISLAND_FLAGS_26), 0xf0);
        assert_eq!(array_words(&e, world + WORLD_INACTIVE_ISLANDS), vec![built]);
    }

    #[test]
    fn remove_entity_si_deletes_an_emptied_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_REMOVE_ENTITY, 0x00cb_4000]);
        let world = e.mem.alloc(0x350);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let island = object(&mut e, 0x6c);
        e.put_vtable(OBJECT_VTABLE, &[OBJECT_SLOT_10]);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u16(island + ISLAND_INDEX, 3);
        e.call(0x00cb_4740, &args![world, entity]);
        assert_eq!(e.mem.u32(entity + ENTITY_WORLD), 0);
        assert_eq!(calls(&e, ISLAND_REMOVE_ENTITY), vec![vec![island, entity]]);
        assert_eq!(calls(&e, 0x00cb_4000), vec![vec![world, island]]);
        assert_eq!(calls(&e, OBJECT_SLOT_10), vec![vec![island, 1]]);

        // An island that still holds entities, or is not in a list, is kept.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 1);
        e.call(0x00cb_4740, &args![world, entity]);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 0);
        e.mem.set_u16(island + ISLAND_INDEX, 0xffff);
        e.call(0x00cb_4740, &args![world, entity]);
        assert!(calls(&e, 0x00cb_4000).is_empty());
        assert_eq!(calls(&e, ISLAND_REMOVE_ENTITY).len(), 2);
    }

    #[test]
    fn fn_00cb4790_takes_over_the_actions_of_the_fixed_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, ISLAND_REGISTER_ACTION]);
        let (world, fixed) = island_world(&mut e);
        e.mem.set_u16(fixed + ISLAND_DIRTY_INDEX, 0xffff);
        let entity = e.mem.alloc(0x220);
        let mine = e.mem.alloc(0x20);
        e.mem.set_u32(mine + 0xc, fixed);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, 0x1234);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, other);
        e.mem.set_u32(data + 4, mine);
        e.mem.set_u32(entity + ENTITY_ACTIONS, data);
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 2);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00cb_4790, &args![world, entity, out]);
        assert_eq!(array_words(&e, out), vec![mine]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![mine]]);
        assert_eq!(calls(&e, ISLAND_REGISTER_ACTION), vec![vec![fixed, mine]]);
        assert_eq!(e.mem.u8(fixed + ISLAND_FLAGS_25), 0x10);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![fixed]);
        assert_eq!(e.mem.u16(fixed + ISLAND_DIRTY_INDEX), 0);
        // No actions: nothing.
        e.call_log = Some(Vec::new());
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 0);
        e.call(0x00cb_4790, &args![world, entity, out]);
        assert!(calls(&e, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn fn_00cb5000_removes_masters_then_slaves_last_first() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, 0x00cb_3dd0]);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, 0x9000);
        let masters = e.mem.alloc(0x60);
        e.mem.set_u32(masters, 0xa0);
        e.mem.set_u32(masters + 0x30, 0xa1);
        e.mem.set_u32(entity + 0xac, masters);
        e.mem.set_u16(entity + 0xb0, 2);
        let slaves = e.mem.alloc(8);
        e.mem.set_u32(slaves, 0xb0);
        e.mem.set_u32(slaves + 4, 0xb1);
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_u32(entity + 0xb8, 2);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00cb_5000, &args![entity, out]);
        assert_eq!(array_words(&e, out), vec![0xa1, 0xa0, 0xb1, 0xb0]);
        assert_eq!(
            calls(&e, 0x00cb_3dd0),
            vec![
                vec![0x9000, 0xa1, 0],
                vec![0x9000, 0xa0, 0],
                vec![0x9000, 0xb1, 0],
                vec![0x9000, 0xb0, 0]
            ]
        );
        assert_eq!(calls(&e, ADD_REFERENCE).len(), 4);
    }

    #[test]
    fn fn_00cb5500_marks_and_lists_the_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        // In the active list, unmarked: the try-increase bits then the mark.
        e.mem.set_u8(island + ISLAND_FLAGS_26, 0x21);
        e.call(0x00cb_5500, &args![world, island]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_26), 0x15);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![island]);
        // Already marked: only the mark bits are normalised; already listed: not listed twice.
        e.mem.set_u8(island + ISLAND_FLAGS_26, 0x28);
        e.call(0x00cb_5500, &args![world, island]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_26), 0x24);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS).len(), 1);
    }

    #[test]
    fn merge_islands_queues_while_locked() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[WORLD_ENQUEUE_OPERATION, INTERNAL_MERGE_TWO_ISLANDS],
        );
        let world = e.mem.alloc(0x350);
        let (first, second) = (e.mem.alloc(0x220), e.mem.alloc(0x220));
        e.mem.set_u32(first + ENTITY_ISLAND, 0x111);
        e.mem.set_u32(second + ENTITY_ISLAND, 0x222);
        e.call(0x00cb_5570, &args![world, first, second]);
        assert_eq!(
            calls(&e, INTERNAL_MERGE_TWO_ISLANDS),
            vec![vec![world, 0x111, 0x222]]
        );
        assert!(calls(&e, WORLD_ENQUEUE_OPERATION).is_empty());
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 1);
        e.register_double(WORLD_ENQUEUE_OPERATION, move |e, a| {
            // The record: byte 0xc, then the two entities.
            assert_eq!(e.mem.u8(a[1]), 0xc);
            assert_eq!(e.mem.u32(a[1] + 4), first);
            assert_eq!(e.mem.u32(a[1] + 8), second);
            0u32.into_ret()
        });
        e.call(0x00cb_5570, &args![world, first, second]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION).len(), 1);
        assert!(calls(&e, INTERNAL_MERGE_TWO_ISLANDS).is_empty());
    }

    /// A world with a broad phase and a dispatcher (both generic objects) and a filter.
    fn broad_phase_world(e: &mut Engine) -> (u32, u32, u32) {
        let world = e.mem.alloc(0x350);
        let broad_phase = object(e, 0x10);
        let dispatcher = e.mem.alloc(0x10);
        e.mem.set_u32(world + WORLD_BROAD_PHASE, broad_phase);
        e.mem.set_u32(world + WORLD_DISPATCHER, dispatcher);
        e.mem.set_u32(world + WORLD_COLLISION_INPUT, 0x4000);
        e.mem.set_u32(world + WORLD_BROAD_PHASE_QUERY_SIZE, 4);
        e.mem.set_u32(world + WORLD_COLLISION_FILTER, 0x6000);
        (world, broad_phase, dispatcher)
    }

    #[test]
    fn update_entity_bp_exchanges_the_pairs() {
        let (mut e, router) = rig();
        stub(
            &mut e,
            &[
                ENTITY_BATCH_RECALC_AABB,
                DISPATCHER_REMOVE_DUPLICATES,
                DISPATCHER_REMOVE_PAIRS,
                DISPATCHER_ADD_PAIRS,
                WORLD_ENQUEUE_OPERATION,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let (world, _broad_phase, dispatcher) = broad_phase_world(&mut e);
        let entity = e.mem.alloc(0x220);
        // The AABB recalculation gets the entity slot.
        e.register_double(ENTITY_BATCH_RECALC_AABB, move |e, a| {
            assert_eq!((a[0], a[2]), (0x4000, 1));
            assert_eq!(e.mem.u32(a[1]), entity);
            0u32.into_ret()
        });
        // The broad phase gets the handle slot, the box and 1, and fills the "added" array
        // with two pairs and "removed" with one.
        e.register_double(OBJECT_SLOT_2C, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), entity + 0x24);
            assert_eq!(a[3], 1);
            let low: Vec<u32> = (0..3).map(|i| e.mem.u32(a[2] + i * 4)).collect();
            assert_eq!(low, vec![100 - 4, 200 - 8, 300 - 12]);
            let high: Vec<u32> = (0..3).map(|i| e.mem.u32(a[2] + 0x10 + i * 4)).collect();
            assert_eq!(high, vec![(4 << 2) + 50, (5 << 2) + 150, (6 << 2) + 250]);
            let (added, removed) = (a[4], a[5]);
            let data = e.mem.u32(added);
            e.mem.set_u32(data, 1);
            e.mem.set_u32(added + 4, 2);
            let data = e.mem.u32(removed);
            e.mem.set_u32(data, 2);
            e.mem.set_u32(removed + 4, 1);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        for (i, v) in [100u32, 200, 300].iter().enumerate() {
            e.mem.set_u32(entity + 0x30 + i as u32 * 4, *v);
            e.mem.set_u32(entity + 0x40 + i as u32 * 4, *v - 50);
        }
        // Extents 1, 2, 3 and 4, 5, 6 shifted by 2.
        e.mem.write(entity + 0x3c, &[1, 2, 3, 2]);
        e.mem.write(entity + 0x4c, &[4, 5, 6]);
        let top = e.mem.u32(router);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB).len(), 1);
        assert_eq!(calls(&e, OBJECT_SLOT_2C).len(), 1);
        assert_eq!(calls(&e, DISPATCHER_REMOVE_DUPLICATES).len(), 1);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 1));
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 2, 0x6008)
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(router), top);

        // A locked world only queues the entity.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 1);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION).len(), 1);
        assert!(calls(&e, OBJECT_SLOT_2C).is_empty());

        // An entity without a shape only takes and drops the lock.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 0);
        e.mem.set_u32(entity + 0x10, 0);
        e.call(0x00cb_5750, &args![world, entity]);
        assert!(calls(&e, OBJECT_SLOT_2C).is_empty());
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
    }

    #[test]
    fn add_entity_bp_adds_the_new_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[ENTITY_BATCH_RECALC_AABB, DISPATCHER_ADD_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let entity = e.mem.alloc(0x220);
        e.register_double(OBJECT_SLOT_14, move |e, a| {
            // The handle, the copy of the entity's AABB, the pair array.
            assert_eq!(a[1], entity + 0x24);
            assert_eq!(e.mem.bytes(a[2], 32), vec![9u8; 32]);
            let array = a[3];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xdead);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        e.mem.write(entity + 0x30, &[9u8; 32]);
        let top = e.mem.u32(router);
        e.call(0x00cb_59f0, &args![world, entity]);
        assert_eq!(calls(&e, OBJECT_SLOT_14).len(), 1);
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 1, 0x6008)
        );
        assert_eq!(e.mem.u32(added[0][1]), 0xdead);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB).len(), 1);
        assert_eq!(e.mem.u32(router), top);
        // No pairs: nothing is added; no shape: nothing at all.
        e.call_log = Some(Vec::new());
        e.register(OBJECT_SLOT_14, |_, _| 0u32.into_ret());
        e.call(0x00cb_59f0, &args![world, entity]);
        assert!(calls(&e, DISPATCHER_ADD_PAIRS).is_empty());
        e.mem.set_u32(entity + 0x10, 0);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_59f0, &args![world, entity]);
        assert!(calls(&e, ENTITY_BATCH_RECALC_AABB).is_empty());
    }

    #[test]
    fn add_phantom_bp_stores_the_bounding_volume() {
        let (mut e, router) = rig();
        stub(
            &mut e,
            &[PHANTOM_SET_BOUNDING_VOLUME_DATA, DISPATCHER_ADD_PAIRS],
        );
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let phantom = object(&mut e, 0x40);
        e.register(OBJECT_SLOT_10, |e, a| {
            let array = a[3];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xbeef);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_5b10, &args![world, phantom]);
        // The phantom's own slot 0x18 filled the buffer; the broad phase was asked (slot 0x10).
        let fill = calls(&e, OBJECT_SLOT_18);
        assert_eq!(fill.len(), 1);
        let query = calls(&e, OBJECT_SLOT_10);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][1], phantom + 0x24);
        assert_eq!(query[0][2], fill[0][1]);
        assert_eq!(
            calls(&e, PHANTOM_SET_BOUNDING_VOLUME_DATA),
            vec![vec![phantom, fill[0][1]]]
        );
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 1, 0x6008)
        );
        assert_eq!(e.mem.u32(router), top);
    }

    #[test]
    fn remove_entity_bp_resets_collision_information_and_removes_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[DISPATCHER_REMOVE_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let simulation = object(&mut e, 0x10);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation);
        e.register(OBJECT_SLOT_1C, |e, a| {
            let array = a[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xfeed);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        let entity = e.mem.alloc(0x220);
        // The collision information reset gets the entity slot, 1, the world and 1.
        e.register_double(OBJECT_SLOT_20, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), entity);
            assert_eq!(a[2..], [1, world, 1]);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        let top = e.mem.u32(router);
        e.call(0x00cb_5c10, &args![world, entity]);
        assert_eq!(calls(&e, OBJECT_SLOT_20).len(), 1);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 1));
        assert_eq!(e.mem.u32(router), top);
        // No shape: nothing at all.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(entity + 0x10, 0);
        e.call(0x00cb_5c10, &args![world, entity]);
        assert!(calls(&e, OBJECT_SLOT_20).is_empty());
    }

    #[test]
    fn remove_phantom_bp_removes_the_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[DISPATCHER_REMOVE_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        e.register(OBJECT_SLOT_1C, |e, a| {
            let array = a[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xfeed);
            e.mem.set_u32(array + 4, 2);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_5d00, &args![world, 0x3000u32]);
        let query = calls(&e, OBJECT_SLOT_1C);
        assert_eq!(query[0][1], 0x3000 + 0x24);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 2));
        assert_eq!(e.mem.u32(router), top);
        e.call_log = Some(Vec::new());
        e.register(OBJECT_SLOT_1C, |_, _| 0u32.into_ret());
        e.call(0x00cb_5d00, &args![world, 0x3000u32]);
        assert!(calls(&e, DISPATCHER_REMOVE_PAIRS).is_empty());
    }

    #[test]
    fn fn_00cb5dd0_moves_actions_that_involve_only_fixed_entities() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, ISLAND_REGISTER_ACTION]);
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        let fixed = e.mem.alloc(0x220);
        e.mem.set_u8(fixed + ENTITY_MOTION_TYPE, 5);
        let stranger = e.mem.alloc(0x220);
        e.mem.set_u8(stranger + ENTITY_MOTION_TYPE, 1);
        // Action 0 involves the fixed one and this entity; action 1 a stranger.
        let actions = e.mem.alloc(8);
        let action_a = object(&mut e, 0x20);
        let action_b = e.mem.alloc(0x20);
        e.mem.set_u32(action_b, OBJECT_VTABLE);
        e.mem.set_u32(actions, action_a);
        e.mem.set_u32(actions + 4, action_b);
        e.mem.set_u32(entity + ENTITY_ACTIONS, actions);
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 2);
        e.register_double(OBJECT_SLOT_10, move |e, a| {
            let (list, this) = (a[1], a[0]);
            let data = e.mem.u32(list);
            let members: Vec<u32> = if this == action_a {
                vec![fixed, entity]
            } else {
                vec![stranger]
            };
            for (i, m) in members.iter().enumerate() {
                e.mem.set_u32(data + i as u32 * 4, *m);
            }
            e.mem.set_u32(list + 4, members.len() as u32);
            0u32.into_ret()
        });
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00cb_5dd0, &args![world, entity, out]);
        assert_eq!(array_words(&e, out), vec![action_a]);
        assert_eq!(
            calls(&e, ISLAND_REGISTER_ACTION),
            vec![vec![island, action_a]]
        );
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![action_a]]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 0x10);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![island]);
    }

    #[test]
    fn add_actions_to_entitys_island_merges_other_islands() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ACTION, REMOVE_REFERENCE, 0x00cb_5570]);
        let (world, fixed) = island_world(&mut e);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        let other = e.mem.alloc(0x220);
        e.mem.set_u32(other + ENTITY_WORLD, world);
        e.mem.set_u32(other + ENTITY_ISLAND, 0x5555);
        e.mem.set_u8(other + ENTITY_MOTION_TYPE, 1);
        let action = object(&mut e, 0x20);
        e.register_double(OBJECT_SLOT_10, move |e, a| {
            let data = e.mem.u32(a[1]);
            e.mem.set_u32(data, entity);
            e.mem.set_u32(data + 4, other);
            e.mem.set_u32(a[1] + 4, 2);
            0u32.into_ret()
        });
        let actions = e.mem.alloc(12);
        word_array(&mut e, actions, &[action]);
        e.call(0x00cb_5f50, &args![world, entity, actions]);
        assert_eq!(calls(&e, ISLAND_ADD_ACTION), vec![vec![island, action]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![action]]);
        assert_eq!(calls(&e, 0x00cb_5570), vec![vec![world, other, entity]]);

        // The fixed island: just added, no entity listing.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(entity + ENTITY_ISLAND, fixed);
        e.call(0x00cb_5f50, &args![world, entity, actions]);
        assert_eq!(calls(&e, ISLAND_ADD_ACTION), vec![vec![fixed, action]]);
        assert!(calls(&e, OBJECT_SLOT_10).is_empty());
    }

    const SET_TYPE_DOUBLES: [u32; 20] = [
        LINKED_COLLIDABLE_SORT_ENTRIES,
        ENTITY_SORT_CONSTRAINTS_SLAVES,
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        0x00cb_4230,
        WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
        0x00cb_5000,
        0x00cb_5dd0,
        0x00cb_4790,
        0x00cb_3ee0,
        0x00cb_4740,
        0x00cb_4640,
        0x00cb_5f50,
        0x00cb_3ca0,
        0x00cb_5570,
        AGENT_APPEND_TRACK,
        FREEZE_MOTION_STATE,
        SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
        FIRE_INACTIVE_ENTITY_MOVED,
        WORLD_RUN_PENDING_OPERATIONS,
    ];

    #[test]
    fn set_rigid_body_motion_type_without_leaving_fixedness() {
        let (mut e, _) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // Same type: nothing at all.
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 1);
        // Keyframed to dynamic with no saved motion: nothing happens.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 4);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert!(calls(&e, ADD_REFERENCE).is_empty());
        // Dynamic to dynamic without a world: motion replaced, held by a reference.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 2u32, 0u32, 7u32]);
        assert_eq!(calls(&e, 0x00cb_4230), vec![vec![entity, 2, 1, 1, 0]]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![entity]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![entity]]);
        assert!(calls(&e, WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY).is_empty());
        // With a world, the collision filter is updated.
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 3u32, 0u32, 7u32]);
        assert_eq!(
            calls(&e, WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY),
            vec![vec![world, entity, 7, 1]]
        );
        // Leaving fixed sorts the collidable entries and the slave constraints first.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.mem.set_u32(entity + ENTITY_WORLD, 0);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 4u32, 0u32, 7u32]);
        assert_eq!(
            calls(&e, LINKED_COLLIDABLE_SORT_ENTRIES),
            vec![vec![entity + 0x10]]
        );
        assert_eq!(
            calls(&e, ENTITY_SORT_CONSTRAINTS_SLAVES),
            vec![vec![entity]]
        );
    }

    #[test]
    fn set_rigid_body_motion_type_to_fixed_runs_the_whole_sequence() {
        let (mut e, router) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        stub(&mut e, &[WORLD_RUN_PENDING_BODY_OPERATIONS]);
        let simulation = object(&mut e, 0x10);
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation);
        e.mem.set_u32(world + WORLD_STEP_TIME, 0.75f32.to_bits());
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 1);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 3);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // The constraint collector puts two constraints in the array.
        e.register(0x00cb_5000, |e, a| {
            let array = a[1];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xc1);
            e.mem.set_u32(data + 4, 0xc2);
            e.mem.set_u32(array + 4, 2);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_6f70, &args![entity, 5u32, 1u32, 9u32]);
        assert_eq!(
            order(
                &e,
                &[
                    ADD_REFERENCE,
                    0x00cb_5000,
                    0x00cb_5dd0,
                    0x00cb_3ee0,
                    0x00cb_4740,
                    0x00cb_4230,
                    0x00cb_4640,
                    0x00cb_5f50,
                    0x00cb_3ca0,
                    AGENT_APPEND_TRACK,
                    FREEZE_MOTION_STATE,
                    SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                    FIRE_INACTIVE_ENTITY_MOVED,
                    WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                    WORLD_RUN_PENDING_OPERATIONS,
                    REMOVE_REFERENCE,
                ]
            ),
            vec![
                ADD_REFERENCE,
                0x00cb_5000,
                0x00cb_5dd0,
                0x00cb_3ee0,
                0x00cb_4740,
                0x00cb_4230,
                0x00cb_4640,
                0x00cb_5f50,
                0x00cb_3ca0,
                REMOVE_REFERENCE,
                0x00cb_3ca0,
                REMOVE_REFERENCE,
                AGENT_APPEND_TRACK,
                FREEZE_MOTION_STATE,
                SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                FIRE_INACTIVE_ENTITY_MOVED,
                WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                WORLD_RUN_PENDING_OPERATIONS,
                REMOVE_REFERENCE,
            ]
        );
        assert_eq!(
            calls(&e, 0x00cb_3ca0),
            vec![vec![world, 0xc1, 0], vec![world, 0xc2, 0]]
        );
        assert_eq!(calls(&e, 0x00cb_4640), vec![vec![world, entity, 1]]);
        assert_eq!(
            calls(&e, FREEZE_MOTION_STATE),
            vec![vec![0.75f32.to_bits(), entity + 0xf0]]
        );
        // The island with more than 2 entities is flagged before the entity leaves it.
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 1);
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
        assert_eq!(e.mem.u8(world + WORLD_BLOCK_PENDING), 0);
        assert_eq!(e.mem.u8(world + WORLD_CRITICAL_ALLOWED), 1);
        let broadcast = calls(&e, OBJECT_SLOT_20);
        assert_eq!(broadcast.len(), 1);
        assert_eq!(&broadcast[0][2..], &[1, world, 7]);
        assert_eq!(e.mem.u32(router), top);
        // The fixed entity goes through the fixed-island action collector.
        assert!(calls(&e, 0x00cb_4790).is_empty());
    }

    #[test]
    fn set_rigid_body_motion_type_from_fixed_merges_with_the_partners() {
        let (mut e, _) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.mem.set_u32(entity + 0x1f8, 0x1234);
        let partner = e.mem.alloc(0x220);
        e.mem.set_u32(partner + ENTITY_ISLAND, 0x7777);
        e.mem.set_u8(partner + ENTITY_MOTION_TYPE, 1);
        e.mem.set_i8(partner + 0x20, -0x10);
        e.register_double(0x00cb_4230, move |e, a| {
            e.mem.set_u8(a[0] + ENTITY_MOTION_TYPE, a[1] as u8);
            0u32.into_ret()
        });
        e.register_double(LINKED_COLLIDABLE_GET_ENTRIES_SORTED, move |e, a| {
            let array = a[1];
            let data = e.mem.alloc(8);
            e.mem.set_u32(data + 4, partner + 0x10);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        // The entity becomes dynamic: actions via the fixed-island collector, no flagging of
        // the island (it was fixed), a merge with the partner, no freeze.
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert_eq!(calls(&e, 0x00cb_4790).len(), 1);
        assert!(calls(&e, 0x00cb_5dd0).is_empty());
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 0);
        assert_eq!(calls(&e, 0x00cb_5570), vec![vec![world, entity, partner]]);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
        assert_eq!(calls(&e, AGENT_APPEND_TRACK).len(), 1);
    }

    #[test]
    fn fn_00cb7460_answers_the_word_or_0x200() {
        let (mut e, _) = rig();
        let this = e.mem.alloc(16);
        e.mem.set_u32(this, 0x77);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x00cb_7460, &args![this, 4u32]).u32(), 0x77);
        assert_eq!(e.call(0x00cb_7460, &args![this, 5u32]).u32(), 0x200);
        // -1 + 1 wraps to 0.
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x00cb_7460, &args![this, u32::MAX]).u32(), 0x77);
    }

    #[test]
    fn add_constraint_info_folds_the_counts() {
        let (mut e, _) = rig();
        let owner = e.mem.alloc(0x20);
        let internal = e.mem.alloc(0x20);
        let constraint = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x10);
        for (i, v) in [5u32, 7, 9, 11].iter().enumerate() {
            e.mem.set_u32(info + i as u32 * 4, *v);
        }
        e.mem.set_u32(owner + 8, 3);
        e.mem.set_u32(owner + 0xc, 100);
        e.mem.set_u32(owner + 0x10, 200);
        e.mem.set_u32(owner + 0x14, 300);
        e.mem.set_u16(internal + 0x14, 20);
        e.mem.set_u16(internal + 0x16, 0xfffe);
        e.mem.set_u16(internal + 0x18, 40);
        // No internal block: nothing changes.
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.u32(owner + 8), 3);
        e.mem.set_u32(constraint + 0x30, internal);
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 107);
        assert_eq!(e.mem.u32(owner + 0x10), 209);
        assert_eq!(e.mem.u32(owner + 0x14), 311);
        assert_eq!(e.mem.u16(internal + 0x14), 27);
        assert_eq!(e.mem.u16(internal + 0x16), 7);
        assert_eq!(e.mem.u16(internal + 0x18), 51);
        // max(3, 5, 7) = 7, then raised to the new counter 27.
        assert_eq!(e.mem.i32(owner + 8), 27);
        // A counter below the running maximum leaves it.
        e.mem.set_u32(owner + 8, 99);
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.i32(owner + 8), 99);
    }
}

#[cfg(test)]
mod island_split_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router with a stack of 0x1000 bytes; returns the router and the stack base.
    fn router_with_stack(e: &mut Engine) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, stack)
    }

    #[test]
    fn constraint_added_and_removed_in_a_critical_locked_island() {
        let mut e = engine();
        let log: Log = Rc::default();
        for address in [
            WORLD_CONSTRAINT_UTIL_ADD,
            FIRE_CONSTRAINT_ADDED,
            FIRE_CONSTRAINT_REMOVED,
            WORLD_CONSTRAINT_UTIL_REMOVE,
        ] {
            log_double(&mut e, &log, address);
        }
        e.call(0x00cb_3c80, &args![0x1000u32, 0x2000u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![WORLD_CONSTRAINT_UTIL_ADD, FIRE_CONSTRAINT_ADDED]
        );
        assert_eq!(
            calls(&log, WORLD_CONSTRAINT_UTIL_ADD),
            vec![vec![0x1000, 0x2000]]
        );
        assert_eq!(
            calls(&log, FIRE_CONSTRAINT_ADDED),
            vec![vec![0x1000, 0x2000]]
        );
        // No listeners: only the removal itself.
        log.borrow_mut().clear();
        let world = e.mem.alloc(0x200);
        e.call(0x00cb_3da0, &args![world, 0x2000u32]);
        assert!(calls(&log, FIRE_CONSTRAINT_REMOVED).is_empty());
        assert_eq!(
            calls(&log, WORLD_CONSTRAINT_UTIL_REMOVE),
            vec![vec![0x2000]]
        );
        // With listeners the callback runs first.
        log.borrow_mut().clear();
        e.mem.set_u32(world + 0x144, 1);
        e.call(0x00cb_3da0, &args![world, 0x2000u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![FIRE_CONSTRAINT_REMOVED, WORLD_CONSTRAINT_UTIL_REMOVE]
        );
        assert_eq!(
            calls(&log, FIRE_CONSTRAINT_REMOVED),
            vec![vec![world, 0x2000]]
        );
    }

    #[test]
    fn island_goes_into_the_dirty_list_once_inside_the_critical_section() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, ENTER_CRITICAL_SECTION);
        log_double(&mut e, &log, LEAVE_CRITICAL_SECTION);
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, ARRAY_RESERVE_MORE);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0xc0, 0x7000);
        let list = e.mem.alloc(16);
        e.mem.set_u32(world + 0x40, list);
        e.mem.set_u32(world + 0x44, 2);
        e.mem.set_u32(world + 0x48, 0x8000_0004);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0xff);
        e.mem.set_u16(island + 0x22, 0xffff);
        e.call(0x00cb_5480, &args![world, island]);
        assert_eq!(e.mem.u8(island + 0x26), 0xf3);
        assert_eq!(e.mem.u16(island + 0x22), 2);
        assert_eq!(e.mem.u32(world + 0x44), 3);
        assert_eq!(e.mem.u32(list + 8), island);
        assert_eq!(calls(&log, ENTER_CRITICAL_SECTION), vec![vec![0x7000]]);
        assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION), vec![vec![0x7000]]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]);
        // Already listed: the index stays and nothing is added, but the section is still used.
        e.mem.set_u16(island + 0x22, 1);
        e.call(0x00cb_5480, &args![world, island]);
        assert_eq!(e.mem.u16(island + 0x22), 1);
        assert_eq!(e.mem.u32(world + 0x44), 3);
        assert_eq!(calls(&log, ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn owner_info_is_taken_off_the_owner_and_the_record() {
        let mut e = engine();
        let owner = e.mem.alloc(0x40);
        for (offset, value) in [(0xc, 100u32), (0x10, 200), (0x14, 300)] {
            e.mem.set_u32(owner + offset, value);
        }
        let info = e.mem.alloc(0x20);
        for (offset, value) in [(4, 5u32), (8, 6), (0xc, 7)] {
            e.mem.set_u32(info + offset, value);
        }
        let instance = e.mem.alloc(0x40);
        // Without a record nothing changes.
        e.call(0x00cb_7480, &args![owner, instance, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 100);
        let record = e.mem.alloc(0x40);
        e.mem.set_u16(record + 0x14, 50);
        e.mem.set_u16(record + 0x16, 3);
        e.mem.set_u16(record + 0x18, 60);
        e.mem.set_u32(instance + 0x30, record);
        e.call(0x00cb_7480, &args![owner, instance, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 95);
        assert_eq!(e.mem.u32(owner + 0x10), 194);
        assert_eq!(e.mem.u32(owner + 0x14), 293);
        assert_eq!(e.mem.u16(record + 0x14), 45);
        // 3 - 6 wraps in 16 bits.
        assert_eq!(e.mem.u16(record + 0x16), 0xfffd);
        assert_eq!(e.mem.u16(record + 0x18), 53);
    }

    /// A comparison that is "a is less than b" on the pointer values themselves.
    fn less_double(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(0x0900_0001, move |e, a| {
            log.borrow_mut().push((0x0900_0001, a.to_vec()));
            e.mem.set_u8(a[0], u8::from(a[1] < a[2]));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn quick_sort_orders_the_given_range_only() {
        let mut e = engine();
        let log: Log = Rc::default();
        less_double(&mut e, &log);
        let values = [9u32, 3, 7, 3, 1, 8, 2, 5, 4, 6, 0];
        let array = e.mem.alloc(4 * values.len() as u32);
        for (i, &v) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, v);
        }
        e.call(0x00dd_7d00, &args![array, 1i32, 9i32, 0x0900_0001u32]);
        let got: Vec<u32> = (0..values.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect();
        let mut expected = values.to_vec();
        expected[1..=9].sort_unstable();
        assert_eq!(got, expected);
        // Two comparison slots are used, and every comparison goes through the callback.
        assert!(log.borrow().len() >= 9);
        let slots: Vec<u32> = log.borrow().iter().map(|(_, a)| a[0]).collect();
        assert!(slots.iter().any(|&s| s != slots[0]));
        // A single element and an already sorted run need no change.
        e.call(0x00dd_7d00, &args![array, 4i32, 4i32, 0x0900_0001u32]);
        e.call(0x00dd_7d00, &args![array, 1i32, 9i32, 0x0900_0001u32]);
        let again: Vec<u32> = (0..values.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect();
        assert_eq!(again, expected);
    }

    /// The structures the splitting functions read: a world with the rank bytes, `count`
    /// parents, the entities `(first rank word, second rank word, master count)`, a union-find
    /// whose two-level array maps entity `i` to `groups[i]`, and a flag byte array.
    struct Rig {
        world: u32,
        parents: u32,
        entities: u32,
        union_find: u32,
        flags: u32,
        flag_bytes: u32,
    }

    fn rig(e: &mut Engine, count: u32, entities: &[(u16, u16, u16)], groups: &[u32]) -> Rig {
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u32(world + 0xb4, 30);
        let parents = e.mem.alloc(0x10);
        e.mem.set_u32(parents + 4, count);
        let entity_data = e.mem.alloc(4 * entities.len().max(1) as u32);
        for (i, &(first, second, masters)) in entities.iter().enumerate() {
            let entity = e.mem.alloc(0x100);
            e.mem.set_u16(entity + 0xea, first);
            e.mem.set_u16(entity + 0xec, second);
            e.mem.set_u16(entity + 0xb0, masters);
            e.mem.set_u32(entity_data + 4 * i as u32, entity);
        }
        let entity_array = e.mem.alloc(0x10);
        e.mem.set_u32(entity_array, entity_data);
        e.mem.set_u32(entity_array + 4, entities.len() as u32);
        let group_data = e.mem.alloc(4 * groups.len().max(1) as u32);
        for (i, &g) in groups.iter().enumerate() {
            e.mem.set_u32(group_data + 4 * i as u32, g);
        }
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, group_data);
        let union_find = e.mem.alloc(0x10);
        e.mem.set_u32(union_find, inner);
        let flag_bytes = e.mem.alloc(0x20);
        for i in 0..0x20 {
            e.mem.set_u8(flag_bytes + i, 0x77);
        }
        let flags = e.mem.alloc(8);
        e.mem.set_u32(flags, flag_bytes);
        Rig {
            world,
            parents,
            entities: entity_array,
            union_find,
            flags,
            flag_bytes,
        }
    }

    /// The reindex double: records the marks array (`count` words) it is given through the
    /// slot, the group count and the parents.
    type Seen = Rc<RefCell<Vec<(Vec<i32>, u32, u32)>>>;

    fn reindex_double(e: &mut Engine, count: u32, seen: &Seen) {
        let seen = Rc::clone(seen);
        e.register_double(UNION_FIND_REINDEX, move |e, a| {
            let marks = e.mem.u32(a[1]);
            let words = (0..count)
                .map(|i| e.mem.i32(marks + 4 * i))
                .collect::<Vec<_>>();
            seen.borrow_mut().push((words, a[2], a[3]));
            Ret::default()
        });
    }

    // Entities `(word at +0xEA, word at +0xEC, master count)` with the ranks 4 (0x4185 is
    // `1 << 14 | 3 << 7 | 5` and 0x8209 is `2 << 14 | 4 << 7 | 9`), 32 and 3.
    const ENTITIES: [(u16, u16, u16); 3] = [(0x4185, 0x8209, 2), (0x0020, 0, 0), (0, 3, 1)];

    #[test]
    fn marking_numbers_the_marked_groups_and_reindexes() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[2, 0, 1]);
        e.call(
            0x00cb_4890,
            &args![
                rig.entities,
                rig.union_find,
                rig.world,
                rig.parents,
                rig.flags
            ],
        );
        // Entities 0 and 2 (groups 2 and 1) have a rank of at most 5; element 0 is unmarked
        // and gets number 1, the marked ones are cleared.
        assert_eq!(*seen.borrow(), vec![(vec![1, 0, 0], 2, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0x77);
        // The router's top is back where it was.
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn marking_with_nothing_marked_gives_up_without_reindexing() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 2, &seen);
        let rig = rig(&mut e, 2, &[], &[]);
        e.mem.set_u8(rig.flag_bytes, 9);
        e.call(
            0x00cb_4890,
            &args![
                rig.entities,
                rig.union_find,
                rig.world,
                rig.parents,
                rig.flags
            ],
        );
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u8(rig.flag_bytes), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_merges_light_groups_without_ranking() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        // The byte at +0xD5 is clear: every entity marks its group, weights are 2, 1 and 1.
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        // One group takes all three (total 4 stays below the limit of 30): marks 0, 0, 0.
        assert_eq!(*seen.borrow(), vec![(vec![0, 0, 0], 1, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0x77);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_with_ranks_leaves_unmarked_elements_apart() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        e.mem.set_u8(rig.world + 0xd5, 1);
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        // Weights are 2, -1 (the middle entity's rank is above 5, so nothing marks its group)
        // and 1; the first element takes the third (total 3), so groups 0 and 1 remain:
        // marks 0, 1, 0.
        assert_eq!(*seen.borrow(), vec![(vec![0, 1, 0], 2, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_stops_merging_when_the_total_reaches_the_limit() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        // With a limit of 11 even the smallest total (1 + 10) is not below it: nothing merges
        // and every element is its own group.
        e.mem.set_u32(rig.world + 0xb4, 11);
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        assert_eq!(*seen.borrow(), vec![(vec![0, 1, 2], 3, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0);
        assert_eq!(e.mem.u32(router), stack);
    }
}
