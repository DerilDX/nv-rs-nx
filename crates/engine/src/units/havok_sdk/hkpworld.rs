//! `Havok SDK/hkpworld.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is large (115 functions). Sessions translate it in address order; the first
//! session did the 60 queue entries `00c90e40` to `00c929e0`; a later one added `00c90f40` and
//! the phantom, batch, action, constraint and filter-update functions `00c92c70` to
//! `00c97f50` (15 functions); the next one continues at `00c92a30` with what is left.
//!
//! Layouts and helpers come first. `HkpWorld` is the PC `hkpWorld` (0x350 bytes): its field
//! names and offsets are the Xbox PDB's, and every offset used here is read by the PC code at
//! the same place. Fields inside `m_dynamicsStepInfo` (+0x1d0) that the PDB does not name
//! separately are read by offset (the `DYNAMICS_*` constants).
//!
//! Conventions of this file:
//! - `hkpWorld` methods are `__thiscall` (`this` in ECX); the uniform form passes it first.
//! - A float passed through the x87 stack (`FLD`/`FSTP`) goes through [`x87_quiet`], which
//!   quiets a signalling NaN as the hardware does.
//! - Pieces the compiler outlined from a bigger function take registers the calling
//!   convention does not have (`ESI`, `EDI`, `EAX` set by the caller). They cannot be called
//!   by address in the uniform form: they are translated as plain Rust functions with the
//!   registers as leading parameters, are not registered, and carry a `Translation
//!   incomplete` marker saying so. `00c90f40` is such a piece (EDI, ESI and one stack
//!   argument).
//! - `RDTSC` (the profiling records of the ray and linear casts) has no model in the engine:
//!   the host clock stands in for the time stamp, which nothing but the profiler reads.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------------------
// Callees outside this file, by address.
// ---------------------------------------------------------------------------------------

/// `TlsGetValue` import slot, and the TLS index of the profiling record stream.
pub(crate) const TLS_GET_VALUE: u32 = 0x00fd_f044;
pub(crate) const PROFILE_SLOT: u32 = 0x0126_835c;
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Profiling labels: `"TtworldCastRayCollector"`, `"TtworldCastRay"`, `"TtworldLinCast"` and
/// the end-of-section label `"Et"`.
const PROFILE_CAST_RAY_COLLECTOR: u32 = 0x010c_6490;
const PROFILE_CAST_RAY: u32 = 0x010c_64a8;
const PROFILE_LINEAR_CAST: u32 = 0x010c_64b8;
pub(crate) const PROFILE_END: u32 = 0x010c_41f8;
/// Increment and decrement of the reference count of an `hkReferencedObject` (`this` only);
/// the count is the signed word at +6.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Hands out the thread's memory router (no arguments); `+0x10` of it is the allocator whose
/// vtable slot +4 allocates `(size)` and slot +8 frees `(block, size)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90490` / `00c904e0`: the calls around the `m_isLocked` count at +0xbc (`this` only).
const LOCK_ENTER: u32 = 0x00c9_0490;
const LOCK_LEAVE: u32 = 0x00c9_04e0;
/// The `m_pendingOperations` queue (`hkpWorldOperationQueue` in the Xbox PDB): `00cf7c10` and
/// `00cf7ae0` take only the queue; `00cf7690` takes the operation record.
const QUEUE_FLUSH_PENDING: u32 = 0x00cf_7c10;
const QUEUE_FLUSH_BODY_PENDING: u32 = 0x00cf_7ae0;
const QUEUE_ENQUEUE: u32 = 0x00cf_7690;
/// `hkpSimulation` calls: `00cf86e0` (one float), `00cf86f0` (bool result).
const SIMULATION_SET_TIME: u32 = 0x00cf_86e0;
const SIMULATION_TEST: u32 = 0x00cf_86f0;
/// Entity callees (`00c9c040(entity, world, shape)` ...).
const SET_CACHED_SHAPE_DATA: u32 = 0x00c9_c040;
const SET_TIME_INFORMATION: u32 = 0x00cf_1480;
const ADD_ENTITY_SI: u32 = 0x00cb_4640;
const ADD_ENTITY_BP: u32 = 0x00cb_59f0;
const FIRE_WORLD_ENTITY_ADDED: u32 = 0x00d0_0500;
const FIRE_ENTITY_ADDED: u32 = 0x00cf_fa00;
const REMOVE_ENTITY_SI: u32 = 0x00cb_5c10;
const FIRE_WORLD_ENTITY_REMOVED: u32 = 0x00d0_05a0;
const FIRE_ENTITY_REMOVED: u32 = 0x00cf_fab0;
const REMOVE_ENTITY_BP: u32 = 0x00cb_4740;
const ENTITY_RELEASE_BOUNDING_DATA: u32 = 0x00d0_1a80;
const FIND_AGENT: u32 = 0x00cf_dd70;
const ADD_PAIRS: u32 = 0x00cf_7080;
/// `00d01d20(world)` (cdecl, one argument) and `00d03290(simulation, a, b)` (thiscall).
const WATCH_HEAP_MEMORY: u32 = 0x00d0_1d20;
const SIMULATION_FINISH_MT_STEP: u32 = 0x00d0_3290;
/// Ray and linear casters (`this` = a caller-built object): `00cbf860`, `00cfab30`, `00cfaed0`.
const WORLD_RAY_CASTER_CAST_RAY: u32 = 0x00cb_f860;
const SIMPLE_RAY_CASTER_CAST_RAY: u32 = 0x00cf_ab30;
const LINEAR_CASTER_LINEAR_CAST: u32 = 0x00cf_aed0;
/// Vtables of the objects the casts build on their stack.
const RAY_CASTER_VTABLE: u32 = 0x010c_3bd4;
const SIMPLE_RAY_CASTER_VTABLE: u32 = 0x010c_647c;
const LINEAR_CASTER_VTABLE: u32 = 0x010c_6488;
/// `float` constant stored into the linear caster object at +0x80.
const LINEAR_CASTER_DEFAULT: u32 = 0x0104_da4c;
/// Job queue calls: `00d04b90(queue, job, 1)` and `00d04380(semaphore)` (acquire).
const JOB_QUEUE_ADD_JOB: u32 = 0x00d0_4b90;
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `00d05de0(job)`: initializer of a job header (a bare `RET`).
const JOB_HEADER_INIT: u32 = 0x00d0_5de0;
/// `hkpPhysicsSystem` calls (`this` = the system).
const PHYSICS_SYSTEM_CONSTRUCT: u32 = 0x00d0_65e0;
const PHYSICS_SYSTEM_ADD_RIGID_BODY: u32 = 0x00d0_64a0;
const PHYSICS_SYSTEM_ADD_CONSTRAINT: u32 = 0x00d0_6540;
const PHYSICS_SYSTEM_ADD_ACTION: u32 = 0x00d0_6590;
const PHYSICS_SYSTEM_ADD_PHANTOM: u32 = 0x00d0_64f0;
/// The x87 square root helper (`_CIsqrt`): argument in ST0.
const X87_SQRT: u32 = 0x00ec_6040;
/// `00cfd450(object, world)`, `00cfca20(object, world)` and `00cfb240(registry, object,
/// kind)`: constructors and registration used by the outlined constructor piece `00c912e0`.
const MAKE_HELPER_FIRST: u32 = 0x00cf_d450;
const MAKE_HELPER_SECOND: u32 = 0x00cf_ca20;
const REGISTER_HELPER: u32 = 0x00cf_b240;
/// The `double` 0.5 that `00c90e60` multiplies with.
const HALF: u32 = 0x0101_1588;

// ---------------------------------------------------------------------------------------
// Layouts.
// ---------------------------------------------------------------------------------------

/// Offsets inside `m_dynamicsStepInfo` (+0x1d0), named only by their place in the world.
const DYNAMICS_FLOAT_1E4: u32 = 0x1e4;
const DYNAMICS_FLOAT_1E8: u32 = 0x1e8;
const DYNAMICS_FLOAT_240: u32 = 0x240;
const DYNAMICS_WORD_2F4: u32 = 0x2f4;
const DYNAMICS_WORD_2F8: u32 = 0x2f8;
const DYNAMICS_BYTE_304: u32 = 0x304;
/// Three consecutive bytes (+0x305, +0x306, +0x307).
const DYNAMICS_BYTES_305: u32 = 0x305;
const DYNAMICS_FLOAT_308: u32 = 0x308;

layout! {
    /// `hkpWorld` (Xbox PDB), 0x350 bytes. Only the fields the translations of this file use
    /// are listed; names and offsets are the PDB's, confirmed against the PC code.
    pub struct HkpWorld: 0x350 {
        /// `m_simulation` (Xbox PDB): `hkpSimulation*`.
        0x08 m_simulation: Ptr,
        /// `m_fixedIsland` (Xbox PDB): `hkpSimulationIsland*`.
        0x20 m_fixedIsland: Ptr,
        /// `m_activeSimulationIslands` (Xbox PDB): `hkArray` data pointer.
        0x28 m_activeSimulationIslands: Ptr,
        /// Size of `m_activeSimulationIslands`.
        0x2c m_activeSimulationIslandsSize: i32,
        /// `m_inactiveSimulationIslands` (Xbox PDB): `hkArray` data pointer.
        0x34 m_inactiveSimulationIslands: Ptr,
        /// Size of `m_inactiveSimulationIslands`.
        0x38 m_inactiveSimulationIslandsSize: i32,
        /// `m_memoryWatchDog` (Xbox PDB): `hkRefPtr`.
        0x50 m_memoryWatchDog: Ptr,
        /// `m_broadPhase` (Xbox PDB): `hkpBroadPhase*`.
        0x58 m_broadPhase: Ptr,
        /// `m_kdTreeManager` (Xbox PDB): `hkpTreeWorldManager*`.
        0x5c m_kdTreeManager: Ptr,
        /// `m_autoUpdateTree` (Xbox PDB).
        0x60 m_autoUpdateTree: bool,
        /// `m_broadPhaseDispatcher` (Xbox PDB): `hkpTypedBroadPhaseDispatcher*`.
        0x64 m_broadPhaseDispatcher: Ptr,
        /// `m_collisionInput` (Xbox PDB): `hkpProcessCollisionInput*`.
        0x78 m_collisionInput: Ptr,
        /// `m_collisionFilter` (Xbox PDB): `hkpCollisionFilter*`.
        0x7c m_collisionFilter: Ptr,
        /// `m_collisionDispatcher` (Xbox PDB).
        0x80 m_collisionDispatcher: Ptr,
        /// `m_convexListFilter` (Xbox PDB).
        0x84 m_convexListFilter: Ptr,
        /// `m_pendingOperations` (Xbox PDB): `hkpWorldOperationQueue*`.
        0x88 m_pendingOperations: Ptr,
        /// `m_pendingOperationsCount` (Xbox PDB).
        0x8c m_pendingOperationsCount: i32,
        /// `m_pendingBodyOperationsCount` (Xbox PDB).
        0x90 m_pendingBodyOperationsCount: i32,
        /// `m_criticalOperationsLockCount` (Xbox PDB).
        0x94 m_criticalOperationsLockCount: i32,
        /// A second lock count the phantom functions add to the first (name unconfirmed).
        0x98 m_phantomOperationsLockCount: i32,
        /// `m_blockExecutingPendingOperations` (Xbox PDB).
        0x9c m_blockExecutingPendingOperations: bool,
        /// `m_criticalOperationsAllowed` (Xbox PDB).
        0x9d m_criticalOperationsAllowed: bool,
        /// `m_pendingOperationQueueCount` (Xbox PDB).
        0xa4 m_pendingOperationQueueCount: i32,
        /// `m_modifyConstraintCriticalSection` (Xbox PDB): `hkCriticalSection*`.
        0xb8 m_modifyConstraintCriticalSection: Ptr,
        /// `m_isLocked` (Xbox PDB).
        0xbc m_isLocked: i32,
        /// A flag (name unconfirmed): set, `addEntityBatch` builds a new island for the batch.
        0xc8 m_addsToNewIsland: bool,
        /// `m_lastEntityUid` (Xbox PDB).
        0x104 m_lastEntityUid: u32,
        /// `m_lastConstraintUid` (name unconfirmed): `addConstraint` counts it up.
        0x10c m_lastConstraintUid: u32,
        /// `m_phantoms` (Xbox PDB): `hkArray` data pointer.
        0x110 m_phantoms: Ptr,
        /// Size of `m_phantoms`.
        0x114 m_phantomsSize: i32,
        /// `m_broadPhaseBorder` (Xbox PDB): `hkpBroadPhaseBorder*`.
        0x1bc m_broadPhaseBorder: Ptr,
        /// `m_collisionCallbackUtil` (Xbox PDB).
        0x1c0 m_collisionCallbackUtil: Ptr,
    }

    /// `hkpEntity` (Xbox PDB) as far as the world's functions read it (PC offsets).
    pub struct HkpEntity: 0x220 {
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_collidable.m_shape` (Xbox PDB; `m_collidable` is at +0x10).
        0x10 m_collidableShape: Ptr,
        /// `m_collidable.m_motion` (Xbox PDB).
        0x18 m_collidableMotion: Ptr,
        /// A pointer inside the collidable's bounding-volume data (+0x54).
        0x54 m_boundingVolumeChildData: Ptr,
        /// `m_uid` (Xbox PDB).
        0xd4 m_uid: u32,
        /// Two words of the entity (+0xea, +0xec) the world clears when it adds the entity.
        0xea m_wordEa: u16,
        0xec m_wordEc: u16,
    }
}

/// `m_collidable` (Xbox PDB), +0x10 of the entity.
const ENTITY_COLLIDABLE: u32 = 0x10;
/// `m_collidable.m_broadPhaseHandle` (Xbox PDB), +0x24 of the entity.
const ENTITY_BROADPHASE_HANDLE: u32 = 0x24;
/// The entity's bounding-volume data (`m_collidable.m_boundingVolumeData`, +0x30).
const ENTITY_BOUNDING_DATA: u32 = 0x30;
/// The motion embedded in the entity (`m_motion`, +0xe0) and the swept transform inside it.
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_SWEPT_TRANSFORM: u32 = 0xf0;

// ---------------------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------------------

/// Low word of the time-stamp counter (`RDTSC`). The engine has no model of it: the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream
/// (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`) is below its limit
/// (`+0xC`); a record is 12 bytes.
pub(crate) fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Allocates `size` bytes through the memory router's allocator (`00c85750`, vtable slot 4).
fn router_alloc(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// Frees a block of `size` bytes through the router's allocator (vtable slot 8).
fn router_free(e: &mut Engine, block: u32, size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![block, size]);
}

/// `FLD float` / `FSTP float` copy of the `float` at `from` to `to`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let value = x87_quiet(e.mem.f32(from));
    e.mem.set_f32(to, value);
}

fn copy_word(e: &mut Engine, from: u32, to: u32) {
    let value = e.mem.u32(from);
    e.mem.set_u32(to, value);
}

fn copy_byte(e: &mut Engine, from: u32, to: u32) {
    let value = e.mem.u8(from);
    e.mem.set_u8(to, value);
}

/// `MOVAPS` copy of a 16-byte vector.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let bytes = e.mem.bytes(from, 16);
    e.mem.write(to, &bytes);
}

/// `SHUFPS` splat: the 32-bit pattern `bits` into the four lanes at `to`.
fn splat_float(e: &mut Engine, bits: u32, to: u32) {
    for lane in 0..4 {
        e.mem.set_u32(to + 4 * lane, bits);
    }
}

/// The world has a kd-tree manager (+0x5c) and its slot +0x10 says it is in use: the test
/// every cast makes before preferring the tree.
fn kd_tree_in_use(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool()
}

/// The number of 64-element jobs `count` elements split into, as the dispatchers compute it:
/// `((count - 1 + ((count - 1) >> 31 & 0x3f)) >> 6) + 1`.
fn job_block_count(count: i32) -> i32 {
    let last = count.wrapping_sub(1);
    (last.wrapping_add((last >> 31) & 0x3f) >> 6).wrapping_add(1)
}

/// The entry `index` of the `hkArray` whose data pointer is `data`.
fn array_entry(e: &Engine, data: u32, index: i32) -> u32 {
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

// ---------------------------------------------------------------------------------------
// Translations.
// ---------------------------------------------------------------------------------------

// Translated from 00c90e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactMgrFactory::calcContentStatistics` (Xbox PDB): tells the statistics collector
/// (first stack argument; its vtable slot +0x18 takes the class name `"hkpContactMgrFactory"`,
/// slot +0x1c takes nothing). The second stack argument is not read.
pub fn hkp_contact_mgr_factory_calc_content_statistics(
    e: &mut Engine,
    _this: Ptr,
    collector: Ptr,
    _unused_3: u32,
) {
    e.vcall(collector.addr(), 0x18, &args![0x010c_63a0u32]);
    e.vcall(collector.addr(), 0x1c, &args![]);
}

// Translated from 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown). Stores a first `float` `a` at +4, half of it (`a * 0.5`, the
/// `double` at `01011588`) at +0xC and a second `float` `b` at +8, then the ratios `b / a`
/// (+0x50), `b / (a * 0.5)` (+0x58), `a / b` (+0x54) and `(a * 0.5) / b` (+0x5C). The
/// 16-byte vectors at +0x30 and +0x40 hold `a / b` and `b / a` in all four lanes.
pub fn fn_00c90e60(e: &mut Engine, this: Ptr, a: f32, b: f32) {
    let base = this.addr();
    let a = x87_quiet(a);
    e.mem.set_f32(base + 4, a);
    let half: f64 = e.global(HALF);
    let half_a = (half * a as f64) as f32;
    e.mem.set_f32(base + 0xc, half_a);
    let b = x87_quiet(b);
    e.mem.set_f32(base + 8, b);
    e.mem.set_f32(base + 0x50, (b as f64 / a as f64) as f32);
    e.mem
        .set_f32(base + 0x58, (b as f64 / half_a as f64) as f32);
    e.mem.set_f32(base + 0x54, (a as f64 / b as f64) as f32);
    let a_over_b = e.mem.u32(base + 0x54);
    splat_float(e, a_over_b, base + 0x30);
    let b_over_a = e.mem.u32(base + 0x50);
    splat_float(e, b_over_a, base + 0x40);
    e.mem
        .set_f32(base + 0x5c, (half_a as f64 / b as f64) as f32);
}

// Translated from 00c90ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Called on the motion embedded in an entity (`+0xE0`) with the world's three
/// bytes at +0x305..+0x307. Compares `c & 3` with the low two bits of the byte at +9 (and
/// `c` with the whole byte): the first two arguments are complemented when the comparison
/// says "below", then shifted into the top two bits of the words at +0xA and +0xC.
pub fn fn_00c90ed0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: i32) {
    let base = this.addr();
    let reference = e.mem.u8(base + 9) as u32;
    let mut first = a;
    if ((c as u32 & 3) as u8) < (reference & 3) as u8 {
        first = !first;
    }
    let mut second = b;
    if c < reference as i32 {
        second = !second;
    }
    let word_a = e.mem.u16(base + 0xa);
    e.mem
        .set_u16(base + 0xa, (word_a & 0x3fff) | (first << 14) as u16);
    let word_b = e.mem.u16(base + 0xc);
    e.mem
        .set_u16(base + 0xc, (word_b & 0x3fff) | (second << 14) as u16);
}

// Translated from 00c90f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultithreadConfig::hkpMultithreadConfig` (Xbox PDB): stores 0x46 in its first word and
/// returns `this`.
pub fn hkp_multithread_config_hkp_multithread_config(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x46);
    this
}

// Translated from 00c91000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Copies the 16-byte vector at `source` to +0x10 (where the Xbox PDB layout has the
/// world's `m_gravity`).
pub fn fn_00c91000(e: &mut Engine, this: Ptr<HkpWorld>, source: Ptr) {
    copy_vector(e, source.addr(), this.addr() + 0x10);
}

// Translated from 00c91020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getCurrentTime` (Xbox PDB): the `float` at +0x14 of the simulation, in `ST0`.
pub fn hkp_world_get_current_time(e: &mut Engine, this: Ptr<HkpWorld>) -> f32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    x87_quiet(e.mem.f32(simulation.addr() + 0x14))
}

// Translated from 00c91030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getCurrentPsiTime` (Xbox PDB): the `float` at +0x18 of the simulation, in `ST0`.
pub fn hkp_world_get_current_psi_time(e: &mut Engine, this: Ptr<HkpWorld>) -> f32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    x87_quiet(e.mem.f32(simulation.addr() + 0x18))
}

// Translated from 00c91040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Passes a `float` (through `FLD`/`FSTP`) to the simulation's `00cf86e0`.
pub fn fn_00c91040(e: &mut Engine, this: Ptr<HkpWorld>, time: f32) {
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.call(SIMULATION_SET_TIME, &args![simulation, x87_quiet(time)]);
}

// Translated from 00c91060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Tail-calls the simulation's `00cf86f0` and returns what it leaves in `EAX` (the
/// caller tests `AL`).
pub fn fn_00c91060(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.call(SIMULATION_TEST, &args![simulation]).u32()
}

// Translated from 00c910a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Does nothing; takes one stack argument.
pub fn fn_00c910a0(_e: &mut Engine, _this: Ptr<HkpWorld>, _unused_2: u32) {}

// Translated from 00c910b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::checkDeterminism` (Xbox PDB): does nothing in this build.
pub fn hkp_world_check_determinism(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

// Translated from 00c910d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Replaces the world's `m_broadPhaseBorder` (+0x1bc): a previous border gets its
/// vtable slot +0x10 called and a reference removed (`00c905b0`), the new one (when not null)
/// gets a reference added (`00c90510`).
pub fn fn_00c910d0(e: &mut Engine, this: Ptr<HkpWorld>, border: Ptr) {
    let old = e.get(this, HkpWorld::m_broadPhaseBorder);
    if !old.is_null() {
        e.vcall(old.addr(), 0x10, &args![]);
        let current = e.get(this, HkpWorld::m_broadPhaseBorder);
        e.call(REMOVE_REFERENCE, &args![current]);
    }
    e.set(this, HkpWorld::m_broadPhaseBorder, border);
    if !border.is_null() {
        e.call(ADD_REFERENCE, &args![border]);
    }
}

// Translated from 00c91110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter of the world's `m_broadPhaseBorder` (+0x1bc).
pub fn fn_00c91110(e: &mut Engine, this: Ptr<HkpWorld>) -> Ptr {
    e.get(this, HkpWorld::m_broadPhaseBorder)
}

// Translated from 00c91120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Clears `m_pendingOperationsCount` and tail-calls `00cf7c10` on the pending
/// operation queue (+0x88).
pub fn fn_00c91120(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.set(this, HkpWorld::m_pendingOperationsCount, 0);
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_FLUSH_PENDING, &args![queue]);
}

// Translated from 00c91140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Clears `m_pendingBodyOperationsCount` and tail-calls `00cf7ae0` on the pending
/// operation queue.
pub fn fn_00c91140(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.set(this, HkpWorld::m_pendingBodyOperationsCount, 0);
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_FLUSH_BODY_PENDING, &args![queue]);
}

// Translated from 00c91160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Hands one operation record to the pending operation queue (`00cf7690`). Every
/// caller builds the record on its stack with the operation number in its first byte.
pub fn fn_00c91160(e: &mut Engine, this: Ptr<HkpWorld>, operation: Ptr) {
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_ENQUEUE, &args![queue, operation]);
}

// Translated from 00c91180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Calls `00c90490` on the world, then counts `m_isLocked` up.
pub fn fn_00c91180(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.call(LOCK_ENTER, &args![this]);
    let locked = e.get(this, HkpWorld::m_isLocked);
    e.set(this, HkpWorld::m_isLocked, locked.wrapping_add(1));
}

// Translated from 00c91190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Counts `m_isLocked` down, then tail-calls `00c904e0` on the world.
pub fn fn_00c91190(e: &mut Engine, this: Ptr<HkpWorld>) {
    let locked = e.get(this, HkpWorld::m_isLocked);
    e.set(this, HkpWorld::m_isLocked, locked.wrapping_sub(1));
    e.call(LOCK_LEAVE, &args![this]);
}

// Translated from 00c911a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Does nothing.
pub fn fn_00c911a0(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

// Translated from 00c911b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::unlockReadOnly` (Xbox PDB): does nothing in this build.
pub fn hkp_world_unlock_read_only(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

/// Calls the import at `import_slot` (`EnterCriticalSection` or `LeaveCriticalSection`) on the
/// world's constraint-update critical section (+0xb8) when it has one; the one stack
/// argument of these functions is overwritten with the section and never read.
fn critical_section_call(e: &mut Engine, this: Ptr<HkpWorld>, import_slot: u32) {
    let section = e.get(this, HkpWorld::m_modifyConstraintCriticalSection);
    if !section.is_null() {
        e.call(import_slot, &args![section]);
    }
}

// Translated from 00c911d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::lockIslandForConstraintUpdate` (Xbox PDB): enters the world's
/// `m_modifyConstraintCriticalSection` (+0xb8) when it exists.
pub fn hkp_world_lock_island_for_constraint_update(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    _unused_2: u32,
) {
    critical_section_call(e, this, ENTER_CRITICAL_SECTION);
}

// Translated from 00c911f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed twin of `00c911d0` (same body): enters the critical section at +0xb8 when it
/// exists.
pub fn fn_00c911f0(e: &mut Engine, this: Ptr<HkpWorld>, _unused_2: u32) {
    critical_section_call(e, this, ENTER_CRITICAL_SECTION);
}

// Translated from 00c91210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::unlockIslandForConstraintUpdate` (Xbox PDB): leaves the critical section at
/// +0xb8 when it exists.
pub fn hkp_world_unlock_island_for_constraint_update(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    _unused_2: u32,
) {
    critical_section_call(e, this, LEAVE_CRITICAL_SECTION);
}

// Translated from 00c91230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed twin of `00c91210` (same body): leaves the critical section at +0xb8 when it
/// exists.
pub fn fn_00c91230(e: &mut Engine, this: Ptr<HkpWorld>, _unused_2: u32) {
    critical_section_call(e, this, LEAVE_CRITICAL_SECTION);
}

// Translated from 00c91250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. When the world has a kd-tree manager (+0x5c), calls its vtable slot +0x14 with 0.
pub fn fn_00c91250(e: &mut Engine, this: Ptr<HkpWorld>) {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    if !tree.is_null() {
        e.vcall(tree.addr(), 0x14, &args![0u32]);
    }
}

// Translated from 00c91270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Tail-calls the kd-tree manager's vtable slot +0xc with two arguments (no null
/// check).
pub fn fn_00c91270(e: &mut Engine, this: Ptr<HkpWorld>, first: u32, second: u32) {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    e.vcall(tree.addr(), 0xc, &args![first, second]);
}

// Translated from 00c912e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c912e0 - takes EAX (the world) and EDI (a registry object) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as parameters and not registered.
/// Unnamed fragment the compiler outlined from the `hkpWorld` constructor (`EAX` = the
/// world, `EDI` = an object whose `00cfb240` takes `(object, kind)`). Makes three small
/// helper objects: two of 0xC bytes built by `00cfd450(world)` and `00cfca20(world)`, and one
/// of 0x14 bytes built inline (vtable `010c6420`, +8 = `010c6378`, +0x10 = 3); registers them
/// with `registry` as kinds 1, 2 and 3 and drops the creator's reference on each.
pub fn fn_00c912e0(e: &mut Engine, world: Ptr, registry: Ptr) {
    let first = router_alloc(e, 0xc);
    e.mem.set_u16(first + 4, 0xc);
    let first = e.call(MAKE_HELPER_FIRST, &args![first, world]).u32();
    let second = router_alloc(e, 0xc);
    e.mem.set_u16(second + 4, 0xc);
    let second = e.call(MAKE_HELPER_SECOND, &args![second, world]).u32();
    let third = router_alloc(e, 0x14);
    e.mem.set_u16(third + 4, 0x14);
    e.mem.set_u16(third + 6, 1);
    e.mem.set_u32(third, 0x010c_6420);
    e.mem.set_u16(third + 0xe, 1);
    e.mem.set_u32(third + 0x10, 3);
    e.mem.set_u32(third + 8, 0x010c_6378);
    e.call(REGISTER_HELPER, &args![registry, first, 1u32]);
    e.call(REGISTER_HELPER, &args![registry, second, 2u32]);
    e.call(REGISTER_HELPER, &args![registry, third, 3u32]);
    e.call(REMOVE_REFERENCE, &args![first]);
    e.call(REMOVE_REFERENCE, &args![second]);
    e.call(REMOVE_REFERENCE, &args![third]);
}

// Translated from 00c913b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::reenableCollisionBetweenEntityPair` (Xbox PDB). While critical operations are
/// locked (+0x94 non-zero) the request is queued as operation 0x13 (the two entities at +4
/// and +8). Otherwise, when the broad phase (slot +0x4c) accepts the two broad-phase handles
/// (entity +0x24), the collision filter (+0x7c, used as its base at +8; not null-checked)
/// enables the two collidables (entity +0x10), and no agent exists yet between them
/// (`00cfdd70`), the pair is added through the broad-phase dispatcher (`00cf7080`).
pub fn hkp_world_reenable_collision_between_entity_pair(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity_a: Ptr,
    entity_b: Ptr,
) {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x13);
            e.mem.set_u32(operation.addr() + 4, entity_a.addr());
            e.mem.set_u32(operation.addr() + 8, entity_b.addr());
            fn_00c91160(e, this, operation);
        });
        return;
    }
    let handle_a = entity_a.addr() + ENTITY_BROADPHASE_HANDLE;
    let handle_b = entity_b.addr() + ENTITY_BROADPHASE_HANDLE;
    let broad_phase = e.get(this, HkpWorld::m_broadPhase);
    if !e
        .vcall(broad_phase.addr(), 0x4c, &args![handle_a, handle_b])
        .bool()
    {
        return;
    }
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    let collidable_a = entity_a.addr() + ENTITY_COLLIDABLE;
    let collidable_b = entity_b.addr() + ENTITY_COLLIDABLE;
    // The filter's base at +8 answers "enabled" through a result pointer (slot +4).
    let enabled = e.with_stack(8, |e, result| {
        let base = filter.wrapping_add(8);
        let vtable = e.mem.u32(base);
        let method = e.mem.u32(vtable + 4);
        let returned = e
            .call(method, &args![base, result, collidable_a, collidable_b])
            .u32();
        e.mem.u8(returned) != 0
    });
    if !enabled {
        return;
    }
    if e.call(FIND_AGENT, &args![collidable_a, collidable_b]).u32() != 0 {
        return;
    }
    let filter_base = if filter == 0 { 0 } else { filter + 8 };
    let dispatcher = e.get(this, HkpWorld::m_broadPhaseDispatcher);
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), handle_a);
        e.mem.set_u32(pair.addr() + 4, handle_b);
        e.call(ADD_PAIRS, &args![dispatcher, pair, 1u32, filter_base]);
    });
}

// Translated from 00c91470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. While critical operations are locked, queues operation 0x17 (first argument at
/// +4, the low word of the second at +8, the low byte of the third at +0xA; the rest of the
/// record is uninitialized stack in the original and zero here). Otherwise calls the
/// simulation's vtable slot +0x30 with `(first, second, world, third)`.
pub fn fn_00c91470(e: &mut Engine, this: Ptr<HkpWorld>, first: u32, second: u32, third: u32) {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x17);
            e.mem.set_u32(operation.addr() + 4, first);
            e.mem.set_u16(operation.addr() + 8, second as u16);
            e.mem.set_u8(operation.addr() + 0xa, third as u8);
            fn_00c91160(e, this, operation);
        });
        return;
    }
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.vcall(simulation.addr(), 0x30, &args![first, second, this, third]);
}

/// What `addEntity` and `removeEntity` do once the outermost critical section ends: unless
/// executing pending operations is blocked (+0x9c), flush the pending operations when there
/// are some and the body operations when there are some and one queue is in use.
fn leave_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    if e.get(this, HkpWorld::m_blockExecutingPendingOperations) {
        return;
    }
    if e.get(this, HkpWorld::m_pendingOperationsCount) != 0 {
        fn_00c91120(e, this);
    }
    if e.get(this, HkpWorld::m_pendingOperationQueueCount) == 1
        && e.get(this, HkpWorld::m_pendingBodyOperationsCount) != 0
    {
        fn_00c91140(e, this);
    }
}

// Translated from 00c914d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addEntity` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 1: entity at +4, second argument at +8) and 0 is returned. Otherwise:
/// caches the shape data, makes sure the collidable has its motion (entity vtable slot
/// +0x10), resets the swept transform's time information, clears two words, gives the entity
/// the next uid (`m_lastEntityUid`), adds a reference, adds it to the simulation islands
/// (`00cb4640`), updates the motion's filter bits from the world's bytes at +0x305..+0x307
/// (`00c90ed0`), then inside the critical section adds it to the broad phase and fires the
/// world and entity "added" callbacks. When the outermost critical section ends the pending
/// operations are flushed. Returns the entity.
pub fn hkp_world_add_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity: Ptr<HkpEntity>,
    second: u32,
) -> Ptr<HkpEntity> {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 1);
            e.mem.set_u32(operation.addr() + 4, entity.addr());
            e.mem.set_u32(operation.addr() + 8, second);
            fn_00c91160(e, this, operation);
        });
        return Ptr::NULL;
    }
    let shape = e.get(entity, HkpEntity::m_collidableShape);
    e.call(SET_CACHED_SHAPE_DATA, &args![entity, this, shape]);
    if e.get(entity, HkpEntity::m_collidableMotion).is_null() {
        let motion = e.vcall(entity.addr(), 0x10, &args![]).ptr::<()>();
        e.set(entity, HkpEntity::m_collidableMotion, motion);
    }
    e.call(
        SET_TIME_INFORMATION,
        &args![0.0f32, 0.0f32, entity.addr() + ENTITY_SWEPT_TRANSFORM],
    );
    e.set(entity, HkpEntity::m_wordEa, 0);
    e.set(entity, HkpEntity::m_wordEc, 0);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, false);
    let uid = e.get(this, HkpWorld::m_lastEntityUid).wrapping_add(1);
    e.set(this, HkpWorld::m_lastEntityUid, uid);
    e.set(entity, HkpEntity::m_uid, uid);
    e.call(ADD_REFERENCE, &args![entity]);
    e.call(ADD_ENTITY_SI, &args![this, entity, second]);
    let world = this.addr();
    let byte_305 = e.mem.u8(world + DYNAMICS_BYTES_305) as u32;
    let byte_306 = e.mem.u8(world + DYNAMICS_BYTES_305 + 1) as u32;
    let byte_307 = e.mem.u8(world + DYNAMICS_BYTES_305 + 2) as i32;
    fn_00c90ed0(
        e,
        Ptr::new(entity.addr() + ENTITY_MOTION),
        byte_305,
        byte_306,
        byte_307,
    );
    let lock = e.get(this, HkpWorld::m_criticalOperationsLockCount);
    e.set(
        this,
        HkpWorld::m_criticalOperationsLockCount,
        lock.wrapping_add(1),
    );
    e.set(this, HkpWorld::m_criticalOperationsAllowed, true);
    e.call(ADD_ENTITY_BP, &args![this, entity]);
    e.call(FIRE_WORLD_ENTITY_ADDED, &args![this, entity]);
    e.call(FIRE_ENTITY_ADDED, &args![entity]);
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    if lock == 0 {
        leave_critical_section(e, this);
    }
    entity
}

// Translated from 00c91620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeEntity` (Xbox PDB), with the `hkBool*` result as the first stack
/// argument. While critical operations are locked the request is queued (operation 2, entity
/// at +4) and `*result` is 0. Otherwise the critical section is entered; the entity is
/// taken out of the simulation islands (`00cb5c10`), the world and entity "removed"
/// callbacks fire, the broad phase drops it (`00cb4740`); a bounding-volume pointer at +0x54
/// is released (`00d01a80` on +0x30); an entity whose `m_memSizeAndFlags` is 0 is deleted
/// through its vtable slot +0x14; its reference is removed. When the outermost critical
/// section ends the pending operations are flushed. `*result` is 1.
pub fn hkp_world_remove_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    result: Ptr,
    entity: Ptr<HkpEntity>,
) {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 2);
            e.mem.set_u32(operation.addr() + 4, entity.addr());
            fn_00c91160(e, this, operation);
        });
        e.mem.set_u8(result.addr(), 0);
        return;
    }
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    e.call(REMOVE_ENTITY_SI, &args![this, entity]);
    e.call(FIRE_WORLD_ENTITY_REMOVED, &args![this, entity]);
    e.call(FIRE_ENTITY_REMOVED, &args![entity]);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, false);
    e.call(REMOVE_ENTITY_BP, &args![this, entity]);
    if !e
        .get(entity, HkpEntity::m_boundingVolumeChildData)
        .is_null()
    {
        e.call(
            ENTITY_RELEASE_BOUNDING_DATA,
            &args![entity.addr() + ENTITY_BOUNDING_DATA],
        );
    }
    if e.get(entity, HkpEntity::m_memSizeAndFlags) == 0 {
        e.vcall(entity.addr(), 0x14, &args![]);
    }
    e.call(REMOVE_REFERENCE, &args![entity]);
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, true);
    if lock == 0 {
        leave_critical_section(e, this);
    }
    e.mem.set_u8(result.addr(), 1);
}

/// Nulls the first entry equal to `listener` in the `hkArray` (data pointer at `data_offset`,
/// size at `data_offset + 4`) of the world; when no entry matches (or the array is empty),
/// the word before the array is written (the original indexes with -1, and so do we).
fn null_listener(e: &mut Engine, this: Ptr<HkpWorld>, data_offset: u32, listener: u32) {
    let world = this.addr();
    let size = e.mem.i32(world + data_offset + 4);
    if size > 0 {
        let mut data = e.mem.u32(world + data_offset);
        for index in 0..size {
            if e.mem.u32(data) == listener {
                let data = e.mem.u32(world + data_offset);
                e.mem
                    .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), 0);
                return;
            }
            data = data.wrapping_add(4);
        }
    }
    let data = e.mem.u32(world + data_offset);
    e.mem.set_u32(data.wrapping_sub(4), 0);
}

// Translated from 00c91700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Nulls the entry equal to `listener` in the array at +0x11c (`m_actionListeners`
/// in the Xbox PDB layout); see `null_listener` for the no-match case.
pub fn fn_00c91700(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x11c, listener);
}

// Translated from 00c91750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x140 (`m_constraintListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91750(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x140, listener);
}

// Translated from 00c917a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x128 (`m_entityListeners` in the Xbox PDB layout).
pub fn fn_00c917a0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x128, listener);
}

// Translated from 00c917f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x134 (`m_phantomListeners` in the Xbox PDB
/// layout).
pub fn fn_00c917f0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x134, listener);
}

// Translated from 00c91840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x158 (`m_islandActivationListeners` in the Xbox
/// PDB layout).
pub fn fn_00c91840(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x158, listener);
}

// Translated from 00c918e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x164 (`m_worldPostSimulationListeners` in the
/// Xbox PDB layout).
pub fn fn_00c918e0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x164, listener);
}

// Translated from 00c91a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x1a0 (`m_contactListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91a20(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x1a0, listener);
}

// Translated from 00c91a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x14c (`m_worldDeletionListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91a70(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x14c, listener);
}

// Translated from 00c91b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::stepDeltaTime` (Xbox PDB). Locks the world (`00c91180`), lets the memory
/// watch-dog check (`00d01d20`), steps the simulation (vtable slot +0xc, the time through
/// `FLD`/`FSTP`); a kd-tree manager is told next (`00c91270(0, 0)` when `m_autoUpdateTree`,
/// else `00c91250`), then the world unlocks (`00c91190`). Returns the simulation's result.
pub fn hkp_world_step_delta_time(e: &mut Engine, this: Ptr<HkpWorld>, delta_time: f32) -> u32 {
    fn_00c91180(e, this);
    e.call(WATCH_HEAP_MEMORY, &args![this]);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .vcall(simulation.addr(), 0xc, &args![x87_quiet(delta_time)])
        .u32();
    if !e.get(this, HkpWorld::m_kdTreeManager).is_null() {
        if e.get(this, HkpWorld::m_autoUpdateTree) {
            fn_00c91270(e, this, 0, 0);
        } else {
            fn_00c91250(e, this);
        }
    }
    fn_00c91190(e, this);
    result
}

// Translated from 00c91b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Locks the world, calls the simulation's vtable slot +0x10 with a `float` (through
/// `FLD`/`FSTP`), unlocks and returns the result.
pub fn fn_00c91b70(e: &mut Engine, this: Ptr<HkpWorld>, time: f32) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .vcall(simulation.addr(), 0x10, &args![x87_quiet(time)])
        .u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Locks the world, calls the simulation's vtable slot +0x14, unlocks and returns
/// the result.
pub fn fn_00c91ba0(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e.vcall(simulation.addr(), 0x14, &args![]).u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91ba0` with the simulation's vtable slot +0x18.
pub fn fn_00c91bd0(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e.vcall(simulation.addr(), 0x18, &args![]).u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::finishMtStep` (Xbox PDB): finishes the simulation's multithreaded step
/// (`00d03290(simulation, a, b)`); a non-zero result is returned at once. Otherwise a
/// kd-tree manager is told (`00c91270(a, b)` when `m_autoUpdateTree`, else `00c91250`) and 0
/// is returned.
pub fn hkp_world_finish_mt_step(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    first: u32,
    second: u32,
) -> u32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .call(SIMULATION_FINISH_MT_STEP, &args![simulation, first, second])
        .u32();
    if result != 0 {
        return result;
    }
    if !e.get(this, HkpWorld::m_kdTreeManager).is_null() {
        if e.get(this, HkpWorld::m_autoUpdateTree) {
            fn_00c91270(e, this, first, second);
        } else {
            fn_00c91250(e, this);
        }
    }
    0
}

/// Copies the reference-counted pointer `new` into the word at `slot`: a reference is added to
/// the new object, one removed from the one replaced.
fn replace_reference(e: &mut Engine, slot: u32, new: u32) {
    if new != 0 {
        e.call(ADD_REFERENCE, &args![new]);
    }
    let replaced = e.mem.u32(slot);
    if replaced != 0 {
        e.call(REMOVE_REFERENCE, &args![replaced]);
    }
    e.mem.set_u32(slot, new);
}

// Translated from 00c91c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Fills the structure at `info` from the world's settings (the layout looks like
/// the world construction info; unconfirmed): vectors and words copied as they are, `float`s
/// through `FLD`/`FSTP`, the collision filter (+0x7c, into +0x58), the convex-list filter
/// (+0x84, into +0x5c) and the memory watch-dog (+0x50, into +0x6c) with a reference added
/// to each new one and removed from each one replaced, the border's two bytes (+0x30, +0x34;
/// 3 and 0 without a border), `info+0x50` = kd-tree manager present, and
/// `info+0x88` = the square root (`00ec6040`) of the step info float at +0x308 (the float at
/// `info+0x90` is written twice, the second value, from +0xd0, stays).
pub fn fn_00c91c50(e: &mut Engine, this: Ptr<HkpWorld>, info: Ptr) {
    let world = this.addr();
    let info = info.addr();
    copy_vector(e, world + 0x10, info + 0x10);
    copy_word(e, world + 0x338, info + 0x20);
    copy_vector(e, world + 0x310, info + 0x30);
    copy_vector(e, world + 0x320, info + 0x40);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    copy_float(e, collision_input + 8, info + 0x54);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    replace_reference(e, info + 0x58, filter);
    let convex = e.get(this, HkpWorld::m_convexListFilter).addr();
    replace_reference(e, info + 0x5c, convex);
    let border = e.get(this, HkpWorld::m_broadPhaseBorder).addr();
    let border_kind = if border == 0 {
        3
    } else {
        e.mem.u32(border + 0x30) as u8
    };
    e.mem.set_u8(info + 0x28, border_kind);
    let border_flag = if border == 0 {
        0
    } else {
        e.mem.u8(border + 0x34)
    };
    e.mem.set_u8(info + 0x29, border_flag);
    let tree_present = !e.get(this, HkpWorld::m_kdTreeManager).is_null();
    e.mem.set_u8(info + 0x50, tree_present as u8);
    copy_byte(e, world + 0x60, info + 0x53);
    let dispatcher = e.get(this, HkpWorld::m_collisionDispatcher).addr();
    copy_float(e, dispatcher + 0x1e40, info + 0x60);
    copy_float(e, dispatcher + 0x1e44, info + 0x68);
    copy_float(e, world + 0xcc, info + 0x90);
    copy_float(e, world + 0xd0, info + 0x90);
    copy_byte(e, world + 0xd4, info + 0x98);
    let watchdog = e.get(this, HkpWorld::m_memoryWatchDog).addr();
    replace_reference(e, info + 0x6c, watchdog);
    copy_word(e, world + 0x330, info + 0x70);
    copy_word(e, world + 0x334, info + 0x64);
    copy_byte(e, world + 0x340, info + 0x74);
    copy_byte(e, collision_input + 0x6d, info + 0x75);
    copy_float(e, world + DYNAMICS_FLOAT_240, info + 0x24);
    copy_float(e, world + DYNAMICS_FLOAT_1E4, info + 0x78);
    copy_float(e, world + DYNAMICS_FLOAT_1E8, info + 0x7c);
    copy_word(e, world + DYNAMICS_WORD_2F4, info + 0x80);
    copy_word(e, world + DYNAMICS_WORD_2F8, info + 0x84);
    let argument = x87_quiet(e.mem.f32(world + DYNAMICS_FLOAT_308));
    let root = e.call(X87_SQRT, &args![argument as f64]).f32();
    e.mem.set_f32(info + 0x88, x87_quiet(root));
    copy_byte(e, world + DYNAMICS_BYTE_304, info + 0x8c);
    copy_byte(e, world + DYNAMICS_BYTES_305, info + 0xa4);
    copy_byte(e, world + DYNAMICS_BYTES_305 + 1, info + 0xa5);
    copy_byte(e, world + DYNAMICS_BYTES_305 + 2, info + 0xa6);
    // The collision input's pointer at +0x70 leads to a float and a word.
    let input_extra = e.mem.u32(collision_input + 0x70);
    copy_float(e, input_extra, info + 0x9c);
    copy_word(e, input_extra + 4, info + 0xa0);
    copy_byte(e, collision_input + 0x6c, info + 0x99);
    copy_byte(e, world + 0xd6, info + 0xa7);
    copy_float(e, world + 0xdc, info + 0xac);
    copy_word(e, world + 0xe0, info + 0xb0);
    copy_byte(e, world + 0xe4, info + 0xb4);
    copy_word(e, world + 0xe8, info + 0xb8);
    copy_word(e, world + 0xec, info + 0xbc);
    copy_float(e, world + 0xd8, info + 0xa8);
    copy_float(e, world + 0xf4, info + 0xc0);
    copy_float(e, world + 0xf8, info + 0xc4);
    copy_float(e, world + 0xfc, info + 0xc8);
    copy_float(e, world + 0x100, info + 0xcc);
    copy_byte(e, world + 0xd5, info + 0xd0);
    copy_byte(e, world + 0xf0, info + 0xd1);
    let simulation = e.get(this, HkpWorld::m_simulation).addr();
    copy_float(e, simulation + 0x24, info + 0xdc);
    copy_byte(e, world + 0xc8, info + 0xd2);
    copy_byte(e, world + 0xb0, info + 0xd8);
    copy_byte(e, world + 0xb1, info + 0xd9);
    copy_word(e, world + 0xb4, info + 0xd4);
    let callbacks = !e.get(this, HkpWorld::m_collisionCallbackUtil).is_null();
    e.mem.set_u8(info + 0xe0, callbacks as u8);
}

// Translated from 00c91f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::castRay` (Xbox PDB; the collector variant): brackets the work in the profiling
/// records `"TtworldCastRayCollector"` and `"Et"`. With a kd-tree manager in use (+0x5c,
/// slot +0x10) its slot +0x18 gets `(input, collision filter, collector)`; otherwise a
/// ray-caster object (vtable `010c3bd4`, words at +0x40 and +0x44 zero) runs
/// `00cbf860(broad phase, input, filter, 0, collector)`.
pub fn hkp_world_cast_ray(e: &mut Engine, this: Ptr<HkpWorld>, input: u32, collector: u32) {
    profile_record(e, PROFILE_CAST_RAY_COLLECTOR);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        e.vcall(tree.addr(), 0x18, &args![input, filter, collector]);
    } else {
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        e.with_stack(0x54, |e, caster| {
            e.mem.set_u32(caster.addr(), RAY_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x40, 0);
            e.mem.set_u32(caster.addr() + 0x44, 0);
            e.call(
                WORLD_RAY_CASTER_CAST_RAY,
                &args![caster, broad_phase, input, filter, 0u32, collector],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00c92040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::castRay_ov2` (Xbox PDB; the plain variant): as `castRay` with the record
/// `"TtworldCastRay"`, the kd-tree manager's slot +0x1c, and a simple ray-caster object
/// (vtable `010c647c`, words at +0x30 and +0x34 zero) running `00cfab30`.
pub fn hkp_world_cast_ray_ov2(e: &mut Engine, this: Ptr<HkpWorld>, input: u32, output: u32) {
    profile_record(e, PROFILE_CAST_RAY);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        e.vcall(tree.addr(), 0x1c, &args![input, filter, output]);
    } else {
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        e.with_stack(0x44, |e, caster| {
            e.mem.set_u32(caster.addr(), SIMPLE_RAY_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x30, 0);
            e.mem.set_u32(caster.addr() + 0x34, 0);
            e.call(
                SIMPLE_RAY_CASTER_CAST_RAY,
                &args![caster, broad_phase, input, filter, 0u32, output],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

/// Tail of the three job dispatchers: waits for the jobs. Calls the notifier object's vtable
/// slot +0xc with `(queue, 0xb)` and slot +0x10, acquires the semaphore, and frees the
/// 0x10-byte completion block.
fn finish_job_dispatch(e: &mut Engine, queue: u32, notifier: u32, semaphore: u32, block: u32) {
    e.vcall(notifier, 0xc, &args![queue, 0xbu32]);
    e.vcall(notifier, 0x10, &args![]);
    e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
    router_free(e, block, 0x10);
}

/// Where a job keeps what the batching changes.
struct JobBatching {
    /// The job (game memory) and its size in bytes.
    job: u32,
    size: u32,
    /// Offsets of the element count and of the element pointer inside the job.
    count_offset: u32,
    pointer_offset: u32,
    /// Elements per batch, and bytes the element pointer advances per batch.
    batch: i32,
    stride: u32,
}

/// Splits the job into batches and adds each to `queue` (`00d04b90(queue, copy, 1)`): the
/// copy handed to the queue carries the smaller of the remaining count and the batch size;
/// the job's element pointer advances by the stride and its count drops by one batch per
/// round, until nothing is left.
fn add_batched_jobs(e: &mut Engine, queue: u32, batching: &JobBatching) {
    let job = batching.job;
    let mut count = e.mem.i32(job + batching.count_offset);
    if count <= 0 {
        return;
    }
    e.with_stack(batching.size, |e, copy| loop {
        let bytes = e.mem.bytes(job, batching.size);
        e.mem.write(copy.addr(), &bytes);
        let batch_count = if count < batching.batch {
            count
        } else {
            batching.batch
        };
        e.mem
            .set_i32(copy.addr() + batching.count_offset, batch_count);
        let pointer = e.mem.u32(job + batching.pointer_offset);
        e.mem.set_u32(
            job + batching.pointer_offset,
            pointer.wrapping_add(batching.stride),
        );
        count = count.wrapping_sub(batching.batch);
        e.mem.set_i32(job + batching.count_offset, count);
        e.call(JOB_QUEUE_ADD_JOB, &args![queue, copy, 1u32]);
        if count <= 0 {
            break;
        }
    });
}

// Translated from 00c92110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Dispatches `count` elements at `array` as batched jobs on `queue`: allocates a
/// 0x10-byte block whose first word is the number of 64-element jobs, builds a 0x50-byte job
/// (bytes 1, 3, 1, size 0x50, id 0xffff, the semaphore, the block, the collision input
/// `+0x78`, 0x40, the array, the count, the broad phase at +0x34; a kd-tree manager in use
/// swaps the broad phase for its words +0xc/+0x10 at +0x3c/+0x40), adds a copy per `batch`
/// elements (`batch * 0x40` bytes further each), then waits through `notifier` and
/// `semaphore`. The job's bytes the original never writes (+3, +8..+0xf, +0x44..+0x4f) are
/// stack garbage there and zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92110(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x50, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 1);
        e.mem.set_u8(job + 1, 3);
        e.mem.set_u8(job + 2, 1);
        e.mem.set_u16(job + 4, 0x50);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x1c, collision_input);
        e.mem.set_u32(job + 0x20, 0x40);
        e.mem.set_u32(job + 0x24, array);
        e.mem.set_u32(job + 0x28, count as u32);
        e.mem.set_u32(job + 0x34, broad_phase);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x3c, first);
            e.mem.set_u32(job + 0x40, second);
            e.mem.set_u32(job + 0x34, 0);
        }
        let batching = JobBatching {
            job,
            size: 0x50,
            count_offset: 0x28,
            pointer_offset: 0x24,
            batch,
            stride: (batch as u32).wrapping_shl(6),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

// Translated from 00c92290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Like `00c92110` with a 0x40-byte job (bytes 2, 2, 2, size 0x40, id 0xffff, the
/// semaphore, the block, the collision input at +0x18, the broad phase at +0x20, 0x40 at
/// +0x30, the array at +0x34, the count at +0x38, and at +0x2c the `float` at +8 of the
/// collision input through `FLD`/`FSTP`) after clearing the word at +0xc of each 0x10-byte
/// element of `array`; the job header is initialized by `00d05de0`; a kd-tree manager in use
/// puts its words +0xc/+0x10 at +0x24/+0x28 and clears +0x20. Batches advance `batch * 0x10`
/// bytes. Bytes the original leaves as stack garbage (+3, +8..+0xf, +0x3c) are zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92290(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let tolerance = x87_quiet(e.mem.f32(collision_input + 8));
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    for index in 0..count.max(0) as u32 {
        e.mem.set_u32(array.wrapping_add(0xc + 0x10 * index), 0);
    }
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x40, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 2);
        e.mem.set_u8(job + 1, 2);
        e.mem.set_u8(job + 2, 2);
        e.mem.set_u16(job + 4, 0x40);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x18, collision_input);
        e.mem.set_u32(job + 0x20, broad_phase);
        e.mem.set_f32(job + 0x2c, tolerance);
        e.mem.set_u32(job + 0x30, 0x40);
        e.mem.set_u32(job + 0x34, array);
        e.mem.set_u32(job + 0x38, count as u32);
        e.call(JOB_HEADER_INIT, &args![job]);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x24, first);
            e.mem.set_u32(job + 0x28, second);
            e.mem.set_u32(job + 0x20, 0);
        }
        let batching = JobBatching {
            job,
            size: 0x40,
            count_offset: 0x38,
            pointer_offset: 0x34,
            batch,
            stride: (batch as u32).wrapping_shl(4),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

// Translated from 00c92430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::linearCast` (Xbox PDB): brackets the work in the records `"TtworldLinCast"` and
/// `"Et"`. The collision filter's base (`+7c`, plus 8, or 0) and the collision input's word
/// at +0x70 are passed on. With a kd-tree manager in use its slot +0x20 gets `(a, b, filter
/// base, collision input, input+0x70, c, d)`; otherwise a linear-caster object (vtable
/// `010c6488`, +0x24 = 0, +0x80 = the `float` at `0104da4c`) runs `00cfaed0(broad phase, a,
/// b, filter base, collision input, input+0x70, 0, c, d)`.
pub fn hkp_world_linear_cast(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    profile_record(e, PROFILE_LINEAR_CAST);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    let filter_base = if filter == 0 { 0 } else { filter + 8 };
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        let extra = e.mem.u32(collision_input + 0x70);
        e.vcall(
            tree.addr(),
            0x20,
            &args![
                first,
                second,
                filter_base,
                collision_input,
                extra,
                third,
                fourth
            ],
        );
    } else {
        let default_value = x87_quiet(e.global::<f32>(LINEAR_CASTER_DEFAULT));
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        let extra = e.mem.u32(collision_input + 0x70);
        e.with_stack(0x84, |e, caster| {
            e.mem.set_f32(caster.addr() + 0x80, default_value);
            e.mem.set_u32(caster.addr(), LINEAR_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x24, 0);
            e.call(
                LINEAR_CASTER_LINEAR_CAST,
                &args![
                    caster,
                    broad_phase,
                    first,
                    second,
                    filter_base,
                    collision_input,
                    extra,
                    0u32,
                    third,
                    fourth
                ],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00c92540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Like `00c92110` with a 0x40-byte job (bytes 4, 2, 1, size 0x40, id 0xffff, the
/// semaphore, the block, the collision input at +0x18, 0x40 at +0x20, the array at +0x24,
/// the count at +0x28, the broad phase at +0x2c); a kd-tree manager in use clears +0x2c and
/// puts its words +0xc/+0x10 at +0x34/+0x38. Batches advance `batch * 0x30` bytes. Bytes the
/// original leaves as stack garbage (+3, +8..+0xf, +0x3c) are zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92540(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x40, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 4);
        e.mem.set_u8(job + 1, 2);
        e.mem.set_u16(job + 4, 0x40);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x18, collision_input);
        e.mem.set_u32(job + 0x20, 0x40);
        e.mem.set_u32(job + 0x24, array);
        e.mem.set_u32(job + 0x28, count as u32);
        e.mem.set_u32(job + 0x2c, broad_phase);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x2c, 0);
            e.mem.set_u32(job + 0x34, first);
            e.mem.set_u32(job + 0x38, second);
        }
        e.mem.set_u8(job + 2, 1);
        let batching = JobBatching {
            job,
            size: 0x40,
            count_offset: 0x28,
            pointer_offset: 0x24,
            batch,
            stride: (batch as u32).wrapping_mul(0x30),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

/// Adds every entity of the island's array (data at +0x48, size at +0x4c) to the physics
/// system as a rigid body.
fn add_island_bodies(e: &mut Engine, island: u32, system: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(island + 0x4c) {
        let data = e.mem.u32(island + 0x48);
        let body = array_entry(e, data, index);
        e.call(PHYSICS_SYSTEM_ADD_RIGID_BODY, &args![system, body]);
        index += 1;
    }
}

// Translated from 00c926a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(out, world, system)`, returns `out`). Adds the rigid bodies of the
/// world's fixed island (+0x20; the first one only when its word at +0x10 is not null) and
/// of every inactive island (+0x34) to the physics system (`00d064a0`); `*out` is 1 when
/// the fixed island had bodies or there are inactive islands.
pub fn fn_00c926a0(e: &mut Engine, out: Ptr, world: Ptr<HkpWorld>, system: Ptr) -> Ptr {
    let fixed = e.get(world, HkpWorld::m_fixedIsland).addr();
    let mut fixed_had_bodies = false;
    if fixed != 0 {
        let mut index = 0i32;
        while index < e.mem.i32(fixed + 0x4c) {
            let data = e.mem.u32(fixed + 0x48);
            let body = array_entry(e, data, index);
            let first_body = e.mem.u32(data);
            if index != 0 || e.mem.u32(first_body + 0x10) != 0 {
                e.call(PHYSICS_SYSTEM_ADD_RIGID_BODY, &args![system, body]);
            }
            index += 1;
        }
        fixed_had_bodies = e.mem.i32(fixed + 0x4c) > 0;
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_bodies(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let any = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize) > 0 || fixed_had_bodies;
    e.mem.set_u8(out.addr(), any as u8);
    out
}

// Translated from 00c92760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(out, world, system)`, returns `out`). Adds the rigid bodies of every
/// active island (+0x28) of the world to the physics system; `*out` is 1 when there are
/// active islands.
pub fn fn_00c92760(e: &mut Engine, out: Ptr, world: Ptr<HkpWorld>, system: Ptr) -> Ptr {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_bodies(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let any = e.get(world, HkpWorld::m_activeSimulationIslandsSize) > 0;
    e.mem.set_u8(out.addr(), any as u8);
    out
}

// Translated from 00c927c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c927c0 - takes ESI (the physics system) and EDI (the world) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as parameters and not registered.
/// Unnamed fragment outlined from `00c929e0`: runs `00c926a0` and `00c92760` on the world
/// into a local flag, then stores the second flag (active islands present) in the byte at
/// +0x40 of the physics system.
pub fn fn_00c927c0(e: &mut Engine, system: Ptr, world: Ptr<HkpWorld>) {
    e.with_stack(8, |e, flag| {
        fn_00c926a0(e, flag, world, system);
        let flag = fn_00c92760(e, flag, world, system);
        let value = e.mem.u8(flag.addr());
        e.mem.set_u8(system.addr() + 0x40, value);
    });
}

// Translated from 00c927f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(island, system)`). For every entity of the island (array at +0x48, size
/// +0x4c) and each of the entity's constraint entries (an array of 0x30-byte entries: data
/// at +0xac, count as a word at +0xb0): follows the entry's pointer at +0xc through the
/// chain of words at +0x14 while the entry's leading word is at least 0x17, and adds the
/// entry's first word to the physics system (`00d06540`) unless the chain ends in a leading
/// word of 0x16.
pub fn fn_00c927f0(e: &mut Engine, island: Ptr, system: Ptr) {
    let island = island.addr();
    let mut entity_index = 0i32;
    while entity_index < e.mem.i32(island + 0x4c) {
        let entities = e.mem.u32(island + 0x48);
        let entity = array_entry(e, entities, entity_index);
        let mut constraint = 0i32;
        while constraint < e.mem.u16(entity + 0xb0) as i32 {
            let data = e.mem.u32(entity + 0xac);
            let entry = data.wrapping_add(0x30 * constraint as u32);
            let mut chain = e.mem.u32(entry + 0xc);
            while e.mem.u16(chain) >= 0x17 {
                chain = e.mem.u32(chain + 0x14);
            }
            if e.mem.u16(chain) != 0x16 {
                let first = e.mem.u32(entry);
                e.call(PHYSICS_SYSTEM_ADD_CONSTRAINT, &args![system, first]);
            }
            constraint += 1;
        }
        entity_index += 1;
    }
}

// Translated from 00c92860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c92860 - takes ESI (the world) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the first parameter and not registered.
/// Unnamed fragment outlined from `00c929e0` (`ESI` = the world, the physics system on the
/// stack): runs `00c927f0` for every active island (+0x28) and then every inactive island
/// (+0x34).
pub fn fn_00c92860(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        fn_00c927f0(e, Ptr::new(island), system);
        cursor = cursor.wrapping_add(4);
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        fn_00c927f0(e, Ptr::new(island), system);
        cursor = cursor.wrapping_add(4);
    }
}

/// Adds every action of the island's action array (data at +0x38, size at +0x3c) to the
/// physics system (`00d06590`).
fn add_island_actions(e: &mut Engine, island: u32, system: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(island + 0x3c) {
        let data = e.mem.u32(island + 0x38);
        let action = array_entry(e, data, index);
        e.call(PHYSICS_SYSTEM_ADD_ACTION, &args![system, action]);
        index += 1;
    }
}

// Translated from 00c928c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(world, system)`). Adds the actions of every active island (+0x28) and
/// then of every inactive island (+0x34) to the physics system.
pub fn fn_00c928c0(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_actions(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_actions(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
}

// Translated from 00c92950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c92950 - takes EDI (the world) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the first parameter and not registered.
/// Unnamed fragment outlined from `00c929e0` (`EDI` = the world, the physics system on the
/// stack): adds the world's phantoms (array at +0x110, size +0x114) to the physics system
/// (`00d064f0`), skipping those equal to one of the six words at +0x18..+0x2c of the broad
/// phase border (+0x1bc) when the world has one.
pub fn fn_00c92950(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let border = fn_00c91110(e, world).addr();
    let mut index = 0i32;
    while index < e.get(world, HkpWorld::m_phantomsSize) {
        let data = e.get(world, HkpWorld::m_phantoms).addr();
        let phantom = array_entry(e, data, index);
        let skipped =
            border != 0 && (0..6).any(|slot| e.mem.u32(border + 0x18 + 4 * slot) == phantom);
        if !skipped {
            e.call(PHYSICS_SYSTEM_ADD_PHANTOM, &args![system, phantom]);
        }
        index += 1;
    }
}

// Translated from 00c929e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Builds a physics system holding the world's contents: allocates 0x44 bytes
/// through the memory router (word at +4 = 0x44), constructs it (`00d065e0`), then adds the
/// bodies (`00c927c0`), phantoms (`00c92950`), constraints (`00c92860`) and actions
/// (`00c928c0`) of the world. Returns the system.
pub fn fn_00c929e0(e: &mut Engine, this: Ptr<HkpWorld>) -> Ptr {
    let block = router_alloc(e, 0x44);
    e.mem.set_u16(block + 4, 0x44);
    let system = e.call(PHYSICS_SYSTEM_CONSTRUCT, &args![block]).ptr::<()>();
    fn_00c927c0(e, system, this);
    fn_00c92950(e, this, system);
    fn_00c92860(e, this, system);
    fn_00c928c0(e, this, system);
    system
}

// ---------------------------------------------------------------------------------------
// Callees and helpers of the actions, constraints, phantoms and collision filter updates
// (00c92c70 and the functions after it in address order).
// ---------------------------------------------------------------------------------------

/// `00c908c0(allocator, array, count, element_size)` and `00c90940(allocator, array,
/// element_size)`: reserve room for `count` elements in an `hkArray` / grow it by one
/// step (identities unconfirmed).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `00d003c0(world, action)` and `00d00460(world, action)`: the world's action added and
/// removed callbacks. `00d00780` / `00d00820` are the same for phantoms.
const FIRE_ACTION_ADDED: u32 = 0x00d0_03c0;
const FIRE_ACTION_REMOVED: u32 = 0x00d0_0460;
const FIRE_PHANTOM_ADDED: u32 = 0x00d0_0780;
const FIRE_PHANTOM_REMOVED: u32 = 0x00d0_0820;
/// `00d07130(island, action)` and `00d07470(island, action)`: take an action out of an
/// island / put it in (both with `this` = island, one stack argument).
const ISLAND_REMOVE_ACTION: u32 = 0x00d0_7130;
const ISLAND_ADD_ACTION: u32 = 0x00d0_7470;
/// `00d06cb0(array, element_size)`: grows an entity's action list.
const ACTION_LIST_GROW: u32 = 0x00d0_6cb0;
/// `00cb5570(world, first_entity, entity)`: merges the islands of two entities.
const MERGE_ISLANDS: u32 = 0x00cb_5570;
/// `00cf3e60(constraint)`, `00cb3ca0(world, constraint, 1)`, `00cb3dd0(world, constraint,
/// 1)` and `00cf3040(constraint, world)`: constraint bookkeeping around adding / removing.
const CONSTRAINT_PREPARE: u32 = 0x00cf_3e60;
const ADD_CONSTRAINT_SI: u32 = 0x00cb_3ca0;
const REMOVE_CONSTRAINT_SI: u32 = 0x00cb_3dd0;
const CONSTRAINT_DETACH: u32 = 0x00cf_3040;
/// Phantom steps: `00cb5b10(world, phantom)` / `00cb5d00(world, phantom)` add it to /
/// remove it from the broad phase; `00d07b90(phantom)` / `00d07b50(phantom)` are the steps
/// after adding / before removing; `00d07950(phantom, aabb)` sets its bounds.
const ADD_PHANTOM_BP: u32 = 0x00cb_5b10;
const REMOVE_PHANTOM_BP: u32 = 0x00cb_5d00;
const PHANTOM_AFTER_ADD: u32 = 0x00d0_7b90;
const PHANTOM_BEFORE_REMOVE: u32 = 0x00d0_7b50;
const PHANTOM_SET_AABB: u32 = 0x00d0_7950;
/// `00cb1260(destination, source, size)`: a memory copy.
const MEM_COPY: u32 = 0x00cb_1260;
/// `00c9c1d0(entity_collidable_owner)`: called by `activateRegion` on the bodies found.
const ACTIVATE_BODY: u32 = 0x00c9_c1d0;
/// `00d08210(collidable)` and `00d081e0(collidable)`: the collidable's agent list.
const COLLIDABLE_RESET_AGENTS: u32 = 0x00d0_8210;
const COLLIDABLE_AGENT_ENTRIES: u32 = 0x00d0_81e0;
/// `00cf7130(pairs, other_pairs)` removes the pairs of the first array from the second.
const REMOVE_DUPLICATE_PAIRS: u32 = 0x00cf_7130;
/// `00cc1160(agent_entry)`: removes an agent and its time-of-impact events.
const REMOVE_AGENT: u32 = 0x00cc_1160;
/// `00cfeba0(agent_entry, collision_input, island)`: updates a shape-collection agent.
const UPDATE_SHAPE_COLLECTION_FILTER: u32 = 0x00cf_eba0;
/// `00d07830(block, world)` constructs a simulation island, `00d07420(island, entity)` adds
/// an entity to it.
const ISLAND_CONSTRUCT: u32 = 0x00d0_7830;
const ISLAND_ADD_ENTITY: u32 = 0x00d0_7420;
/// `00cf70f0(dispatcher, data, size)`: the broad phase pairs to remove.
const REMOVE_PAIRS: u32 = 0x00cf_70f0;

/// Profiling labels of the functions below (strings in the exe).
const PROFILE_UPDATE_FILTER_ON_WORLD: u32 = 0x010c_6908;
const PROFILE_UPDATE_FILTER_ON_PHANTOM: u32 = 0x010c_6758;
const PROFILE_BROADPHASE_SECOND: u32 = 0x010c_6748;
const PROFILE_UPDATE_OVERLAPS: u32 = 0x010c_6734;
const PROFILE_COLLECTION_FILTER: u32 = 0x010c_6720;
const PROFILE_SECTION_END: u32 = 0x0107_6f38;
const PROFILE_UPDATE_FILTER_ON_ENTITY: u32 = 0x010c_679c;
const PROFILE_INIT: u32 = 0x010c_6794;
const PROFILE_PHANTOMS: u32 = 0x010c_6788;
const PROFILE_CHECK_AGENTS: u32 = 0x010c_677c;
const PROFILE_ADD_AGENTS: u32 = 0x010c_6770;
const PROFILE_ADD_ENTITIES: u32 = 0x010c_67e0;
const PROFILE_ADD_BROADPHASE: u32 = 0x010c_67d0;
const PROFILE_CREATE_AGENTS: u32 = 0x010c_67c0;
const PROFILE_ADDED_CALLBACKS: u32 = 0x010c_67b4;
const PROFILE_REMOVE_ENTITIES: u32 = 0x010c_6828;
const PROFILE_REMOVE_INIT: u32 = 0x010c_6818;
const PROFILE_REMOVE_BROADPHASE: u32 = 0x010c_6808;
const PROFILE_DELETE_AGENTS: u32 = 0x010c_67fc;
const PROFILE_REMOVE_CALLBACKS: u32 = 0x010c_67f0;

/// Appends the 16-byte profiling record of the functions that name a second label: `label`
/// at +0, the time stamp at +4, `second` at +0xC.
fn profile_record_pair(e: &mut Engine, label: u32, second: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// Hands an operation record (built by `fill`, kind byte first) to the pending queue.
fn queue_operation(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    kind: u8,
    fill: impl FnOnce(&mut Engine, u32),
) {
    e.with_stack(0x20, |e, operation| {
        e.mem.set_u8(operation.addr(), kind);
        fill(e, operation.addr());
        fn_00c91160(e, this, operation);
    });
}

/// `m_criticalOperationsLockCount != 0`: the request has to be queued.
fn operations_locked(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0
}

/// The phantom functions queue when the two lock counts (+0x94, +0x98) add up to non-zero.
fn phantom_operations_locked(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    e.get(this, HkpWorld::m_phantomOperationsLockCount)
        .wrapping_add(e.get(this, HkpWorld::m_criticalOperationsLockCount))
        != 0
}

fn begin_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_add(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
}

/// Counts the lock down and, when it reaches 0, flushes the pending operations.
fn end_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    if lock == 0 {
        leave_critical_section(e, this);
    }
}

/// A signed byte offset applied to a pointer (`MOVSX` then `ADD`).
fn offset_by_byte(e: &Engine, base: u32, at: u32) -> u32 {
    base.wrapping_add(e.mem.i8(at) as i32 as u32)
}

/// Grows the `hkArray` at `array` by one step through `00c90940` when it is full.
fn array_make_room(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, element_size]);
    }
}

/// `array.pushBack(value)` for 4-byte elements.
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    array_make_room(e, array, 4);
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Empties the array and returns its storage to the allocator at `router + allocator_offset`
/// unless the storage is the array's own (capacity flag bit 31 set).
fn array_release(e: &mut Engine, array: u32, element_size: u32, allocator_offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_offset);
    let data = e.mem.u32(array);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Reserves `bytes` (rounded up to 0x80) from the router's bump cursor (`router[0]`) and
/// returns the cursor before the allocation.
fn bump_allocate(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved = e.mem.u32(router);
    e.mem
        .set_u32(router, saved.wrapping_add(bytes.wrapping_add(0x7f) & !0x7f));
    saved
}

/// Starts an `hkArray` on the bump cursor: `capacity` elements of `element_size` bytes, the
/// storage flagged as not owned. Returns the saved cursor to give back at the end.
fn bump_array(e: &mut Engine, array: u32, capacity: u32, element_size: u32) -> u32 {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let saved = bump_allocate(e, capacity.wrapping_mul(element_size));
    e.mem.set_u32(array, saved);
    e.mem.set_u32(array + 8, capacity | 0x8000_0000);
    saved
}

/// Gives a bump-allocated array back: an array still on its storage is emptied, the cursor
/// goes back to `saved`, then the array is released like any other (a grown array owns
/// heap storage and frees it).
fn bump_array_release(e: &mut Engine, array: u32, saved: u32, element_size: u32) {
    if e.mem.u32(array) == saved {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    array_release(e, array, element_size, 0x10);
}

/// `m_collisionFilter + 8`, or 0 without a filter: the argument the pair creation takes.
fn filter_base(e: &Engine, world: u32) -> u32 {
    match e.mem.u32(world + 0x7c) {
        0 => 0,
        filter => filter + 8,
    }
}

/// The `bool` the collision filter's pair test (the vtable slot 4 of the object at
/// `filter + 8`) answers for the two collidables.
fn filter_accepts(e: &mut Engine, filter: u32, first: u32, second: u32) -> bool {
    e.with_stack(8, |e, scratch| {
        let answer = e
            .vcall(filter + 8, 4, &args![scratch.addr(), first, second])
            .u32();
        e.mem.u8(answer) != 0
    })
}

/// The collision group table lookup the filter updates make: a byte of the collision
/// dispatcher's table (+0x1bb0) indexed by the two collidables' group bytes (+0x1a).
fn group_pair_enabled(e: &Engine, world: u32, first_group: i8, second_group: i8) -> bool {
    let index = (second_group as i32) + (first_group as i32) * 10;
    let table = e.mem.u32(world + 0x80);
    e.mem
        .u8(table.wrapping_add(index as u32).wrapping_add(0x1bb0))
        != 0
}

// Translated from 00c90f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c90f40 - takes EDI (the phantom) and ESI (the other collidable) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as the leading parameters and not registered.
/// Piece of `hkpWorld::updateCollisionFilterOnPhantom` (Xbox PDB) that the compiler
/// outlined: `phantom` is in EDI, `other` (the other body's collidable owner, or the entity's
/// collidable) in ESI and the collision filter is the one stack argument. The pair is asked
/// of the filter; the phantom's overlap callbacks (vtable slots 0x1c / 0x24) and, when the
/// other body is a phantom (byte +0x18 = 2), that body's callbacks (slots 0x1c / 0x24, with
/// the phantom's collidable `phantom + 0x10`) are called so that both sides agree on
/// whether they overlap. Not registered: the register arguments cannot be called by
/// address.
pub fn fn_00c90f40(e: &mut Engine, phantom: u32, other: u32, filter: u32) {
    e.with_stack(8, |e, frame| {
        let flag = frame.addr();
        let scratch = frame.addr() + 4;
        e.mem.set_u32(scratch, filter);
        e.vcall(phantom, 0x20, &args![flag, other]);
        let collidable = phantom + 0x10;
        let answer = e
            .vcall(filter + 8, 4, &args![scratch, collidable, other])
            .u32();
        let accepted = e.mem.u8(answer) != 0;
        let was_overlapping = e.mem.u8(flag) != 0;
        if accepted {
            if !was_overlapping {
                e.vcall(phantom, 0x1c, &args![other]);
            }
        } else if was_overlapping {
            e.vcall(phantom, 0x24, &args![other]);
        }
        if e.mem.u8(other + 0x18) == 2 {
            let body = offset_by_byte(e, other, other + 0x10);
            e.vcall(body, 0x20, &args![scratch, collidable]);
            let overlapping = e.mem.u8(scratch) != 0;
            if accepted {
                if !overlapping {
                    e.vcall(body, 0x1c, &args![collidable]);
                }
            } else if overlapping {
                e.vcall(body, 0x24, &args![collidable]);
            }
        }
    });
}

// Translated from 00c92c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removePhantom` (Xbox PDB). Queued (operation 0xE, phantom at +4) while either
/// lock count (+0x94, +0x98) is non-zero. Otherwise: fires the phantom's "removed" callback
/// (`00d00820`), runs `00d07b50` on it, takes it out of the broad phase (`00cb5d00`), removes
/// it from `m_phantoms` (the last phantom moves into its place), clears its world pointer
/// (+8), deletes it through vtable slot 0x34 when its `m_memSizeAndFlags` is 0, removes the
/// reference and flushes the pending operations when the critical section ends.
pub fn hkp_world_remove_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xe, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
        });
        return;
    }
    begin_critical_section(e, this);
    e.call(FIRE_PHANTOM_REMOVED, &args![this, phantom]);
    e.call(PHANTOM_BEFORE_REMOVE, &args![phantom]);
    e.call(REMOVE_PHANTOM_BP, &args![this, phantom]);
    remove_phantom_from_list(e, this, phantom);
    if e.mem.u16(phantom + 4) == 0 {
        e.vcall(phantom, 0x34, &args![]);
    }
    e.call(REMOVE_REFERENCE, &args![phantom]);
    end_critical_section(e, this);
}

/// Takes `phantom` out of `m_phantoms` (+0x110, size +0x114): its slot (-1 when absent, as
/// the exe indexes) gets the last entry, then the size is one less. Clears the phantom's
/// world pointer first.
fn remove_phantom_from_list(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    let world = this.addr();
    e.mem.set_u32(phantom + 8, 0);
    let size = e.mem.i32(world + 0x114);
    let data = e.mem.u32(world + 0x110);
    let mut index = -1i32;
    for candidate in 0..size.max(0) {
        if e.mem.u32(data.wrapping_add(candidate as u32 * 4)) == phantom {
            index = candidate;
            break;
        }
    }
    let new_size = size.wrapping_sub(1);
    e.mem.set_i32(world + 0x114, new_size);
    if new_size != index {
        let data = e.mem.u32(world + 0x110);
        let last = e
            .mem
            .u32(data.wrapping_add((new_size as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), last);
    }
}

// Translated from 00c936a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addPhantom` (Xbox PDB). Queued (operation 0xD, phantom at +4; returns 0)
/// while either lock count is non-zero. Otherwise: gives the phantom its motion-like object
/// when it has none (+0x18, vtable slot 0x10), sets its world pointer (+8), adds a
/// reference, appends it to `m_phantoms`, adds it to the broad phase (`00cb5b10`), fires the
/// world's "phantom added" callback (`00d00780`), runs `00d07b90` on it and flushes the
/// pending operations when the critical section ends. Returns the phantom.
pub fn hkp_world_add_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) -> u32 {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xd, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
        });
        return 0;
    }
    begin_critical_section(e, this);
    prepare_phantom(e, this, phantom);
    e.call(ADD_PHANTOM_BP, &args![this, phantom]);
    e.call(FIRE_PHANTOM_ADDED, &args![this, phantom]);
    e.call(PHANTOM_AFTER_ADD, &args![phantom]);
    end_critical_section(e, this);
    phantom
}

/// The first steps of `addPhantom`: the phantom's object at +0x18, the world pointer at +8,
/// the reference and the entry in `m_phantoms`.
fn prepare_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    if e.mem.u32(phantom + 0x18) == 0 {
        let made = e.vcall(phantom, 0x10, &args![]).u32();
        e.mem.set_u32(phantom + 0x18, made);
    }
    e.mem.set_u32(phantom + 8, this.addr());
    e.call(ADD_REFERENCE, &args![phantom]);
    array_push_word(e, this.addr() + 0x110, phantom);
}

// Translated from 00c93c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::activateRegion` (Xbox PDB). While critical operations are locked, copies the
/// 0x20-byte AABB into a router block and queues operation 0x20 with the copy at +4.
/// Otherwise asks the broad phase (vtable slot 0x44) for the collidables overlapping the
/// AABB and calls `00c9c1d0` on the body behind each one that is a rigid body (byte +0x18
/// = 1) with a non-null body pointer.
pub fn hkp_world_activate_region(e: &mut Engine, this: Ptr<HkpWorld>, aabb: u32) {
    if operations_locked(e, this) {
        let copy = router_alloc(e, 0x20);
        e.call(MEM_COPY, &args![copy, aabb, 0x20u32]);
        queue_operation(e, this, 0x20, |e, operation| {
            e.mem.set_u32(operation + 4, copy);
        });
        return;
    }
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.with_stack(0x10, |e, found| {
        let found = found.addr();
        e.mem.set_u32(found + 8, 0x8000_0000);
        e.vcall(broad_phase, 0x44, &args![aabb, found]);
        let mut index = 0i32;
        while index < e.mem.i32(found + 4) {
            let data = e.mem.u32(found);
            let collidable = e
                .mem
                .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            let owner = offset_by_byte(e, collidable, collidable + 5);
            if e.mem.u8(owner + 0x18) == 1 {
                let body = offset_by_byte(e, owner, owner + 0x10);
                if body != 0 {
                    e.call(ACTIVATE_BODY, &args![body]);
                }
            }
            index += 1;
        }
        array_release(e, found, 8, 0x10);
    });
}

// Translated from 00c944a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnPhantom` (Xbox PDB). While critical operations are
/// locked the request is queued (operation 0x14: phantom at +4, the flag byte at +8).
/// Otherwise inside the critical section: asks the broad phase (vtable slot 0x48) for the
/// collidables overlapping the phantom's collidable (`phantom + 0x24` is its broad phase
/// handle) and, for every other one, runs the outlined `00c90f40` with the other body and
/// the world's collision filter; when `flag` is non-zero, calls the phantom's vtable slot
/// 0x30 afterwards. Profiling records mark the steps.
pub fn hkp_world_update_collision_filter_on_phantom(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    phantom: u32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x14, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
            e.mem.set_u8(operation + 8, flag as u8);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    profile_record_pair(
        e,
        PROFILE_UPDATE_FILTER_ON_PHANTOM,
        PROFILE_BROADPHASE_SECOND,
    );
    let capacity = e.mem.u32(world + 0x338);
    e.with_stack(0x10, |e, found| {
        let found = found.addr();
        let saved = bump_array(e, found, capacity, 8);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x48, &args![phantom + 0x24, found]);
        profile_record(e, PROFILE_UPDATE_OVERLAPS);
        let mut index = 0i32;
        while index < e.mem.i32(found + 4) {
            let data = e.mem.u32(found);
            let collidable = e
                .mem
                .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            if collidable != phantom + 0x24 {
                let owner = offset_by_byte(e, collidable, collidable + 5);
                let filter = e.mem.u32(world + 0x7c);
                fn_00c90f40(e, phantom, owner, filter);
            }
            index += 1;
        }
        if flag != 0 {
            profile_record(e, PROFILE_COLLECTION_FILTER);
            e.vcall(phantom, 0x30, &args![]);
        }
        end_critical_section(e, this);
        profile_record(e, PROFILE_SECTION_END);
        bump_array_release(e, found, saved, 8);
    });
}

// Translated from 00c946e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnEntity` (Xbox PDB). While critical operations are
/// locked the request is queued (operation 0x12: entity at +4, mode byte at +8, flag byte at
/// +9). Otherwise inside the critical section:
/// - an entity of type 5 first runs `00d08210` on its collidable;
/// - with `mode` 0 the broad phase (slot 0x48) is asked for the collidables overlapping the
///   entity; for each that is a phantom (byte +0x18 = 2) the outlined `00c90f40` runs with
///   the entity's collidable (and the phantom's slot 0x30 follows when `flag` is set) and
///   the entry leaves the list; then every agent of the entity (`00d081e0`) contributes the
///   pair of its two collidables' broad phase handles to a second list and `00cf7130` takes
///   those out of the first;
/// - every agent entry of the entity is asked of the collision filter and the group table;
///   a refused one has its agent removed (`00cc1160`) and the entity's island marked
///   (byte +0x25), an accepted one is updated (`00cfeba0`) when `mode` is 1;
/// - the pairs left in the first list are added to the broad phase dispatcher (`00cf7080`).
pub fn hkp_world_update_collision_filter_on_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity: u32,
    mode: i32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x12, |e, operation| {
            e.mem.set_u32(operation + 4, entity);
            e.mem.set_u8(operation + 8, mode as u8);
            e.mem.set_u8(operation + 9, flag as u8);
        });
        return;
    }
    let world = this.addr();
    profile_record_pair(e, PROFILE_UPDATE_FILTER_ON_ENTITY, PROFILE_INIT);
    begin_critical_section(e, this);
    if e.mem.u8(entity + 0xe8) == 5 {
        e.call(COLLIDABLE_RESET_AGENTS, &args![entity + 0x10]);
    }
    e.with_stack(0x830, |e, frame| {
        let pairs = frame.addr();
        let pairs_storage = pairs + 0x20;
        let second = pairs + 0x10;
        let second_storage = pairs + 0x420;
        e.mem.set_u32(pairs, pairs_storage);
        e.mem.set_u32(pairs + 8, 0x8000_0080);
        if mode == 0 {
            profile_record(e, PROFILE_BROADPHASE_SECOND);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x48, &args![entity + 0x24, pairs]);
            profile_record(e, PROFILE_PHANTOMS);
            let mut index = 0i32;
            while index < e.mem.i32(pairs + 4) {
                let data = e.mem.u32(pairs);
                let collidable = e
                    .mem
                    .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
                let owner = offset_by_byte(e, collidable, collidable + 5);
                if e.mem.u8(owner + 0x18) == 2 {
                    let phantom = offset_by_byte(e, owner, owner + 0x10);
                    if phantom != 0 {
                        let filter = e.mem.u32(world + 0x7c);
                        fn_00c90f40(e, phantom, entity + 0x10, filter);
                        if flag != 0 {
                            e.vcall(phantom, 0x30, &args![]);
                        }
                        let size = e.mem.i32(pairs + 4).wrapping_sub(1);
                        e.mem.set_i32(pairs + 4, size);
                        if size != index {
                            let data = e.mem.u32(pairs);
                            let last = e
                                .mem
                                .bytes(data.wrapping_add((size as u32).wrapping_mul(8)), 8);
                            e.mem
                                .write(data.wrapping_add((index as u32).wrapping_mul(8)), &last);
                        }
                        index -= 1;
                    }
                }
                index += 1;
            }
            let entries = e
                .call(COLLIDABLE_AGENT_ENTRIES, &args![entity + 0x10])
                .u32();
            e.mem.set_u32(second, second_storage);
            e.mem.set_u32(second + 4, 0);
            e.mem.set_u32(second + 8, 0x8000_0080);
            let mut entry_index = 0u32;
            while (entry_index as i32) < e.mem.i32(entries + 4) {
                array_make_room(e, second, 8);
                let element = e
                    .mem
                    .u32(second)
                    .wrapping_add(e.mem.u32(second + 4).wrapping_mul(8));
                let size = e.mem.u32(second + 4);
                e.mem.set_u32(second + 4, size.wrapping_add(1));
                let entry = e.mem.u32(e.mem.u32(entries) + entry_index * 8);
                let first_handle = e.mem.u32(entry + 0x10) + 0x14;
                e.mem.set_u32(element, first_handle);
                let entry = e.mem.u32(e.mem.u32(entries) + entry_index * 8);
                let second_handle = e.mem.u32(entry + 0x14) + 0x14;
                e.mem.set_u32(element + 4, second_handle);
                entry_index += 1;
            }
            e.call(REMOVE_DUPLICATE_PAIRS, &args![second, pairs]);
            array_release(e, second, 8, 0x10);
        }
        profile_record(e, PROFILE_CHECK_AGENTS);
        let family = e.mem.i8(entity + 0x2a);
        let entries = e
            .call(COLLIDABLE_AGENT_ENTRIES, &args![entity + 0x10])
            .u32();
        let mut index = 0i32;
        while index < e.mem.i32(entries + 4) {
            let element = e
                .mem
                .u32(entries)
                .wrapping_add((index as u32).wrapping_mul(8));
            let partner = e.mem.u32(element + 4);
            let filter = e.mem.u32(world + 0x7c);
            let accepted = filter_accepts(e, filter, entity + 0x10, partner);
            let partner_group = e.mem.i8(partner + 0x1a);
            if !accepted || !group_pair_enabled(e, world, family, partner_group) {
                let agent = e.mem.u32(element);
                e.call(REMOVE_AGENT, &args![agent]);
                let island = e.mem.u32(entity + 0xcc);
                let marks = e.mem.u8(island + 0x25);
                e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
                index -= 1;
            } else if mode == 1 {
                let island = if e.mem.u8(entity + 0xe8) == 5 {
                    let body = offset_by_byte(e, partner, partner + 0x10);
                    e.mem.u32(body + 0xcc)
                } else {
                    e.mem.u32(entity + 0xcc)
                };
                let agent = e.mem.u32(element);
                let input = e.mem.u32(world + 0x78);
                e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![agent, input, island]);
            }
            index += 1;
        }
        if e.mem.i32(pairs + 4) > 0 {
            profile_record(e, PROFILE_ADD_AGENTS);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(pairs);
            let size = e.mem.u32(pairs + 4);
            let base = filter_base(e, world);
            e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        }
        end_critical_section(e, this);
        array_release(e, pairs, 8, 0x10);
        profile_record(e, PROFILE_SECTION_END);
    });
}

// Translated from 00c94bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addEntityBatch` (Xbox PDB), `(entities, count, mode)`. Nothing for a count of
/// 0. While critical operations are locked the request is queued (operation 6: entities at
/// +4, the 16-bit count at +8, the mode byte at +0xA). Otherwise inside the critical
/// section: reserves two lists (the entities' broad phase handles, 4 bytes each, and their
/// 0x20-byte bounding boxes); picks the island for the new bodies (the first active island
/// when `m_addsToNewIsland` is clear, with room made for them; otherwise a new island
/// constructed with `00d07830` whose flags and id follow `mode`); for every entity gives it
/// a uid (+0xD4), adds a reference, makes its motion (vtable slot 0x10) when it has none,
/// resets the swept transform times, clears two words, sets its world and the collision
/// filter bits from the world's bytes (`00c90ed0`), adds it to the island (`00d07420`; a
/// type 5 entity goes to the fixed island), asks its shape (slot 0x1c) for the bounding box
/// from the motion and half the collision input's delta time, and caches its shape data
/// (`00c9c040`). A new island is then registered with the world (or destroyed when no
/// entity used it). The broad phase (slot 0x18) adds the whole batch, the new overlaps
/// become agents (`00cf7080`), and the world's and the entities' "added" callbacks fire.
pub fn hkp_world_add_entity_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entities: u32,
    count: i32,
    mode: i32,
) {
    if count == 0 {
        return;
    }
    if operations_locked(e, this) {
        queue_operation(e, this, 6, |e, operation| {
            e.mem.set_u32(operation + 4, entities);
            e.mem.set_u16(operation + 8, count as u16);
            e.mem.set_u8(operation + 0xa, mode as u8);
        });
        return;
    }
    let world = this.addr();
    profile_record_pair(e, PROFILE_ADD_ENTITIES, PROFILE_INIT);
    begin_critical_section(e, this);
    e.with_stack(0x40, |e, frame| {
        let handles = frame.addr();
        let bounds = frame.addr() + 0x10;
        let broad_phase_results = frame.addr() + 0x20;
        for array in [handles, bounds] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        reserve_if_needed(e, handles, count as u32, 4);
        reserve_if_needed(e, bounds, count as u32, 0x20);
        let mut used_new_island = false;
        let island = if e.get(this, HkpWorld::m_addsToNewIsland) {
            construct_island(e, this, mode)
        } else {
            first_active_island_with_room(e, this, count)
        };
        let input = e.mem.u32(world + 0x78);
        let half: f64 = e.global(HALF);
        let half_time = (f64::from(x87_quiet(e.mem.f32(input + 8))) * half) as f32;
        let mut index = 0i32;
        while index < count {
            let entity = e
                .mem
                .u32(entities.wrapping_add((index as u32).wrapping_mul(4)));
            let uid = e.mem.u32(world + 0x104).wrapping_add(1);
            e.mem.set_u32(world + 0x104, uid);
            e.mem.set_u32(entity + 0xd4, uid);
            e.call(ADD_REFERENCE, &args![entity]);
            if e.mem.u32(entity + 0x18) == 0 {
                let motion = e.vcall(entity, 0x10, &args![]).u32();
                e.mem.set_u32(entity + 0x18, motion);
            }
            e.call(
                SET_TIME_INFORMATION,
                &args![0.0f32, 0.0f32, entity + ENTITY_SWEPT_TRANSFORM],
            );
            e.mem.set_u16(entity + 0xec, 0);
            e.mem.set_u16(entity + 0xea, 0);
            e.mem.set_u32(entity + 8, world);
            let first = e.mem.u8(world + DYNAMICS_BYTES_305) as u32;
            let second = e.mem.u8(world + DYNAMICS_BYTES_305 + 1) as u32;
            let third = e.mem.u8(world + DYNAMICS_BYTES_305 + 2) as i32;
            fn_00c90ed0(e, Ptr::new(entity + ENTITY_MOTION), first, second, third);
            if e.mem.u8(entity + 0xe8) == 5 {
                let fixed = e.mem.u32(world + 0x20);
                e.call(ISLAND_ADD_ENTITY, &args![fixed, entity]);
            } else {
                used_new_island = true;
                e.call(ISLAND_ADD_ENTITY, &args![island, entity]);
            }
            let shape = e.mem.u32(entity + 0x10);
            e.mem.set_u32(entity + 0x30, 1);
            e.mem.set_u32(entity + 0x40, 0);
            if shape != 0 {
                let slot = e
                    .mem
                    .u32(bounds)
                    .wrapping_add(e.mem.u32(bounds + 4).wrapping_mul(0x20));
                let used = e.mem.u32(bounds + 4);
                e.mem.set_u32(bounds + 4, used.wrapping_add(1));
                let motion = e.mem.u32(entity + 0x18);
                e.vcall(shape, 0x1c, &args![motion, half_time, slot]);
                let size = e.mem.u32(handles + 4);
                let data = e.mem.u32(handles);
                e.mem
                    .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity + 0x24);
                e.mem.set_u32(handles + 4, size.wrapping_add(1));
                e.call(SET_CACHED_SHAPE_DATA, &args![entity, this, shape]);
            }
            index += 1;
        }
        if e.get(this, HkpWorld::m_addsToNewIsland) {
            if used_new_island {
                let first_body = e.mem.u32(e.mem.u32(island + 0x48));
                let group = e.mem.u8(first_body + 0xd4);
                e.mem.set_u8(island + 0x24, group);
                let list = if mode == 1 {
                    world + 0x28
                } else {
                    world + 0x34
                };
                array_push_word(e, list, island);
            } else {
                e.vcall(island, 0, &args![1u32]);
            }
        }
        let capacity = e.mem.u32(world + 0x338);
        let saved = bump_array(e, broad_phase_results, capacity, 8);
        profile_record(e, PROFILE_ADD_BROADPHASE);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(
            broad_phase,
            0x18,
            &args![handles, bounds, broad_phase_results],
        );
        profile_record(e, PROFILE_CREATE_AGENTS);
        let dispatcher = e.mem.u32(world + 0x64);
        let data = e.mem.u32(broad_phase_results);
        let size = e.mem.u32(broad_phase_results + 4);
        let base = filter_base(e, world);
        e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        profile_record(e, PROFILE_ADDED_CALLBACKS);
        let mut index = 0i32;
        while index < count {
            let entity = e
                .mem
                .u32(entities.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(FIRE_WORLD_ENTITY_ADDED, &args![this, entity]);
            e.call(FIRE_ENTITY_ADDED, &args![entity]);
            index += 1;
        }
        end_critical_section(e, this);
        profile_record(e, PROFILE_SECTION_END);
        bump_array_release(e, broad_phase_results, saved, 8);
        array_release(e, bounds, 0x20, 0xc);
        array_release(e, handles, 4, 0xc);
    });
}

/// The part of `addEntityBatch` that takes the first active island and makes room in its
/// body list for `count` more bodies (at least twice the capacity when it has to grow).
fn first_active_island_with_room(e: &mut Engine, this: Ptr<HkpWorld>, count: i32) -> u32 {
    let first = e.get(this, HkpWorld::m_activeSimulationIslands).addr();
    let island = e.mem.u32(first);
    let bodies = island + 0x48;
    let need = e.mem.i32(island + 0x4c).wrapping_add(count);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(bodies + 8) & 0x3fff_ffff) as i32;
    if capacity < need {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if need < doubled { doubled } else { need };
        e.call(ARRAY_RESERVE, &args![allocator, bodies, wanted, 4u32]);
    }
    island
}

/// The part of `addEntityBatch` that builds a new simulation island (`00d07830` on a 0x6c
/// byte router block): flagged for a rebuild, its id and flags follow `mode`.
fn construct_island(e: &mut Engine, this: Ptr<HkpWorld>, mode: i32) -> u32 {
    let world = this.addr();
    let block = router_alloc(e, 0x6c);
    e.mem.set_u16(block + 4, 0x6c);
    let island = e.call(ISLAND_CONSTRUCT, &args![block, this]).u32();
    let marks = e.mem.u8(island + 0x25);
    e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
    let flags = e.mem.u8(island + 0x26);
    if mode == 1 {
        let id = e.mem.u16(world + 0x2c);
        e.mem.set_u16(island + 0x20, id);
        e.mem.set_u8(island + 0x26, (flags & 0xf5) | 5);
    } else {
        e.mem.set_u8(island + 0x26, flags & 0xf0);
        let id = e.mem.u16(world + 0x38);
        e.mem.set_u16(island + 0x20, id);
    }
    island
}

/// Reserves room for `count` elements (`00c908c0` with the router's second allocator) when
/// the array holds fewer.
fn reserve_if_needed(e: &mut Engine, array: u32, count: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < count as i32 {
        let allocator = e.mem.u32(router + 0xc);
        e.call(ARRAY_RESERVE, &args![allocator, array, count, element_size]);
    }
}

// Translated from 00c951b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeEntityBatch` (Xbox PDB), `(entities, count)`. Nothing for a count below
/// 1. While critical operations are locked the request is queued (operation 7: entities at
/// +4, the 16-bit count at +8). Otherwise inside the critical section: the simulation
/// (vtable slot 0x20) is told first; the broad phase handles (`entity + 0x24`) of the
/// entities that have a shape are collected, the broad phase (slot 0x20) removes them and
/// the pairs that results in are removed (`00cf70f0`); then every entity gets the world's
/// and its own "removed" callbacks, leaves the broad phase (`00cb4740`), releases its
/// bounding data (`00d01a80` when +0x54 is set), is deleted through vtable slot 0x14 when its
/// `m_memSizeAndFlags` is 0 and loses a reference.
pub fn hkp_world_remove_entity_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entities: u32,
    count: i32,
) {
    if count < 1 {
        return;
    }
    if operations_locked(e, this) {
        queue_operation(e, this, 7, |e, operation| {
            e.mem.set_u32(operation + 4, entities);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    profile_record_pair(e, PROFILE_REMOVE_ENTITIES, PROFILE_REMOVE_INIT);
    let simulation = e.mem.u32(world + 8);
    e.vcall(simulation, 0x20, &args![entities, count, this, 1u32]);
    e.with_stack(0x20, |e, frame| {
        let handles = frame.addr();
        let found = frame.addr() + 0x10;
        e.mem.set_u32(handles, 0);
        e.mem.set_u32(handles + 4, 0);
        e.mem.set_u32(handles + 8, 0x8000_0000);
        reserve_if_needed(e, handles, count as u32, 4);
        let end = entities.wrapping_add((count as u32).wrapping_mul(4));
        let mut at = entities;
        while at != end {
            let entity = e.mem.u32(at);
            if e.mem.u32(entity + 0x10) != 0 {
                let size = e.mem.u32(handles + 4);
                let data = e.mem.u32(handles);
                e.mem
                    .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity + 0x24);
                e.mem.set_u32(handles + 4, size.wrapping_add(1));
            }
            at = at.wrapping_add(4);
        }
        if e.mem.u32(handles + 4) != 0 {
            profile_record(e, PROFILE_REMOVE_BROADPHASE);
            let capacity = e.mem.u32(world + 0x338);
            let saved = bump_array(e, found, capacity, 8);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x20, &args![handles, found]);
            profile_record(e, PROFILE_DELETE_AGENTS);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(found);
            let size = e.mem.u32(found + 4);
            e.call(REMOVE_PAIRS, &args![dispatcher, data, size]);
            bump_array_release(e, found, saved, 8);
        }
        array_release(e, handles, 4, 0xc);
    });
    profile_record(e, PROFILE_REMOVE_CALLBACKS);
    let end = entities.wrapping_add((count as u32).wrapping_mul(4));
    let mut at = entities;
    while at != end {
        let entity = e.mem.u32(at);
        e.call(FIRE_WORLD_ENTITY_REMOVED, &args![this, entity]);
        e.call(FIRE_ENTITY_REMOVED, &args![entity]);
        e.call(REMOVE_ENTITY_BP, &args![this, entity]);
        if e.mem.u32(entity + 0x54) != 0 {
            e.call(
                ENTITY_RELEASE_BOUNDING_DATA,
                &args![entity + ENTITY_BOUNDING_DATA],
            );
        }
        if e.mem.u16(entity + 4) == 0 {
            e.vcall(entity, 0x14, &args![]);
        }
        e.call(REMOVE_REFERENCE, &args![entity]);
        at = at.wrapping_add(4);
    }
    profile_record(e, PROFILE_SECTION_END);
    end_critical_section(e, this);
}

// Translated from 00c954e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addPhantomBatch` (Xbox PDB), `(phantoms, count)`. While either lock count is
/// non-zero the request is queued (operation 0xF: phantoms at +4, the 16-bit count at +8).
/// Otherwise inside the critical section: for every phantom gives it its motion-like object
/// (+0x18) when it has none, sets its world pointer, records its broad phase handle
/// (`phantom + 0x24`), asks it for its bounding box (vtable slot 0x18), runs `00d07950` with
/// the box, adds a reference, appends it to `m_phantoms`, fires the world's "phantom added"
/// callback (`00d00780`) and runs `00d07b90`; then the broad phase (slot 0x18) adds all
/// handles with their boxes and the new overlaps become agents (`00cf7080`).
pub fn hkp_world_add_phantom_batch(e: &mut Engine, this: Ptr<HkpWorld>, phantoms: u32, count: i32) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xf, |e, operation| {
            e.mem.set_u32(operation + 4, phantoms);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    begin_critical_section(e, this);
    e.with_stack(0x30, |e, frame| {
        let handles = frame.addr();
        let bounds = frame.addr() + 0x10;
        let found = frame.addr() + 0x20;
        let saved_handles = bump_array(e, handles, count as u32, 4);
        e.mem.set_i32(handles + 4, count);
        let saved_bounds = bump_array(e, bounds, count as u32, 0x20);
        e.mem.set_i32(bounds + 4, count);
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            if e.mem.u32(phantom + 0x18) == 0 {
                let made = e.vcall(phantom, 0x10, &args![]).u32();
                e.mem.set_u32(phantom + 0x18, made);
            }
            e.mem.set_u32(phantom + 8, world);
            let data = e.mem.u32(handles);
            e.mem.set_u32(
                data.wrapping_add((index as u32).wrapping_mul(4)),
                phantom + 0x24,
            );
            let slot = e
                .mem
                .u32(bounds)
                .wrapping_add((index as u32).wrapping_mul(0x20));
            e.vcall(phantom, 0x18, &args![slot]);
            let slot = e
                .mem
                .u32(bounds)
                .wrapping_add((index as u32).wrapping_mul(0x20));
            e.call(PHANTOM_SET_AABB, &args![phantom, slot]);
            e.call(ADD_REFERENCE, &args![phantom]);
            array_push_word(e, world + 0x110, phantom);
            e.call(FIRE_PHANTOM_ADDED, &args![this, phantom]);
            e.call(PHANTOM_AFTER_ADD, &args![phantom]);
            index += 1;
        }
        let capacity = e.mem.u32(world + 0x338);
        let saved = bump_array(e, found, capacity, 8);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x18, &args![handles, bounds, found]);
        let dispatcher = e.mem.u32(world + 0x64);
        let data = e.mem.u32(found);
        let size = e.mem.u32(found + 4);
        let base = filter_base(e, world);
        e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        end_critical_section(e, this);
        bump_array_release(e, found, saved, 8);
        bump_array_release(e, bounds, saved_bounds, 0x20);
        bump_array_release(e, handles, saved_handles, 4);
    });
}

// Translated from 00c95800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removePhantomBatch` (Xbox PDB), `(phantoms, count)`. While either lock count
/// is non-zero the request is queued (operation 0x10: phantoms at +4, the 16-bit count at
/// +8). Otherwise inside the critical section: for every phantom records its broad phase
/// handle (`phantom + 0x24`), fires the world's "phantom removed" callback (`00d00820`) and
/// runs `00d07b50`; the broad phase (slot 0x20) removes the handles and the resulting pairs
/// are removed (`00cf70f0`); then every phantom loses its world pointer, leaves
/// `m_phantoms`, is deleted through vtable slot 0x34 when its `m_memSizeAndFlags` is 0 and
/// loses a reference.
pub fn hkp_world_remove_phantom_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    phantoms: u32,
    count: i32,
) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0x10, |e, operation| {
            e.mem.set_u32(operation + 4, phantoms);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    begin_critical_section(e, this);
    e.with_stack(0x20, |e, frame| {
        let handles = frame.addr();
        let found = frame.addr() + 0x10;
        let saved_handles = bump_array(e, handles, count as u32, 4);
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            let size = e.mem.u32(handles + 4);
            let data = e.mem.u32(handles);
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), phantom + 0x24);
            e.mem.set_u32(handles + 4, size.wrapping_add(1));
            e.call(FIRE_PHANTOM_REMOVED, &args![this, phantom]);
            e.call(PHANTOM_BEFORE_REMOVE, &args![phantom]);
            index += 1;
        }
        if e.mem.u32(handles + 4) != 0 {
            let capacity = e.mem.u32(world + 0x338);
            let saved = bump_array(e, found, capacity, 8);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x20, &args![handles, found]);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(found);
            let size = e.mem.u32(found + 4);
            e.call(REMOVE_PAIRS, &args![dispatcher, data, size]);
            bump_array_release(e, found, saved, 8);
        }
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            remove_phantom_from_list(e, this, phantom);
            if e.mem.u16(phantom + 4) == 0 {
                e.vcall(phantom, 0x34, &args![]);
            }
            e.call(REMOVE_REFERENCE, &args![phantom]);
            index += 1;
        }
        end_critical_section(e, this);
        bump_array_release(e, handles, saved_handles, 4);
    });
}

// Translated from 00c96ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnWorld` (Xbox PDB), `(mode, flag)`. While critical
/// operations are locked the request is queued (operation 0x15: mode byte at +1, flag byte
/// at +2). Otherwise blocks the execution of pending operations (+0x9C) while it works:
/// - mode 0: runs `updateCollisionFilterOnEntity(entity, 0, flag)` for every entity of every
///   active and then inactive island, and `updateCollisionFilterOnPhantom(phantom, flag)` for
///   every phantom;
/// - any other mode: inside the critical section, walks the agent entries of every island
///   (blocks of 0x200 bytes at the island's +0x5C, the last one `+0x58` bytes long; an
///   entry's length is its byte +3). An entry whose collidables (+0x10, +0x14) are refused
///   by the collision filter or whose group pair is disabled is collected, the island is
///   marked (byte +0x25), and its agents removed after the island's walk (`00cc1160`); with
///   mode 1 an accepted entry is updated instead (`00cfeba0`).
/// Then lifts the block, flushes the pending operations when allowed and the lock is 0.
pub fn hkp_world_update_collision_filter_on_world(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    mode: i32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x15, |e, operation| {
            e.mem.set_u8(operation + 1, mode as u8);
            e.mem.set_u8(operation + 2, flag as u8);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, true);
    profile_record(e, PROFILE_UPDATE_FILTER_ON_WORLD);
    if mode == 0 {
        for (list, size) in [(0x28u32, 0x2cu32), (0x34, 0x38)] {
            let mut island_index = 0i32;
            while island_index < e.mem.i32(world + size) {
                let islands = e.mem.u32(world + list);
                let island = e
                    .mem
                    .u32(islands.wrapping_add((island_index as u32).wrapping_mul(4)));
                let mut body_index = 0i32;
                while body_index < e.mem.i32(island + 0x4c) {
                    let bodies = e.mem.u32(island + 0x48);
                    let entity = e
                        .mem
                        .u32(bodies.wrapping_add((body_index as u32).wrapping_mul(4)));
                    hkp_world_update_collision_filter_on_entity(e, this, entity, 0, flag);
                    body_index += 1;
                }
                island_index += 1;
            }
        }
        let mut index = 0i32;
        while index < e.mem.i32(world + 0x114) {
            let phantoms = e.mem.u32(world + 0x110);
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            hkp_world_update_collision_filter_on_phantom(e, this, phantom, flag);
            index += 1;
        }
    } else {
        begin_critical_section(e, this);
        for list in [world + 0x28, world + 0x34] {
            e.with_stack(0x90, |e, collected| {
                let collected = collected.addr();
                e.mem.set_u32(collected, collected + 0x10);
                e.mem.set_u32(collected + 8, 0x8000_0020);
                let mut island_index = 0i32;
                while island_index < e.mem.i32(list + 4) {
                    let islands = e.mem.u32(list);
                    let island = e
                        .mem
                        .u32(islands.wrapping_add((island_index as u32).wrapping_mul(4)));
                    e.mem.set_u32(collected + 4, 0);
                    update_filter_on_island(e, this, island, collected, mode);
                    island_index += 1;
                }
                array_release(e, collected, 4, 0x10);
            });
        }
        let lock = e
            .get(this, HkpWorld::m_criticalOperationsLockCount)
            .wrapping_sub(1);
        e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    }
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, false);
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) == 0 {
        leave_critical_section(e, this);
    }
    profile_record(e, PROFILE_END);
}

/// The agent entry walk of `updateCollisionFilterOnWorld` for one island: `collected`
/// receives the entries to remove, which are removed once the island's blocks are done.
fn update_filter_on_island(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    island: u32,
    collected: u32,
    mode: i32,
) {
    let world = this.addr();
    if e.mem.i32(island + 0x60) <= 0 {
        return;
    }
    let mut block = 0i32;
    loop {
        let start = e.mem.u32(
            e.mem
                .u32(island + 0x5c)
                .wrapping_add((block as u32).wrapping_mul(4)),
        );
        block += 1;
        let end = if block == e.mem.i32(island + 0x60) {
            e.mem.u32(island + 0x58).wrapping_add(start)
        } else {
            start.wrapping_add(0x200)
        };
        let mut entry = start;
        while entry < end {
            let first = e.mem.u32(entry + 0x10);
            let second = e.mem.u32(entry + 0x14);
            let filter = e.mem.u32(world + 0x7c);
            let accepted = filter_accepts(e, filter, first, second);
            let first_group = e.mem.i8(e.mem.u32(entry + 0x10) + 0x1a);
            let second_group = e.mem.i8(e.mem.u32(entry + 0x14) + 0x1a);
            if !accepted || !group_pair_enabled(e, world, first_group, second_group) {
                array_push_word(e, collected, entry);
                let marks = e.mem.u8(island + 0x25);
                e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
            } else if mode == 1 {
                let input = e.mem.u32(world + 0x78);
                e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![entry, input, island]);
            }
            entry = entry.wrapping_add(e.mem.u8(entry + 3) as u32);
        }
        if block >= e.mem.i32(island + 0x60) {
            break;
        }
    }
    while e.mem.u32(collected + 4) != 0 {
        let size = e.mem.u32(collected + 4).wrapping_sub(1);
        e.mem.set_u32(collected + 4, size);
        let agent = e
            .mem
            .u32(e.mem.u32(collected).wrapping_add(size.wrapping_mul(4)));
        e.call(REMOVE_AGENT, &args![agent]);
    }
}

// Translated from 00c97370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addAction` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 0xA: action at +4) and 0 is returned. Otherwise inside the critical
/// section: adds a reference, asks the action for its entities (vtable slot 0x10, into a
/// list of up to 4 on the stack), sets the action's world (+8), appends the action to each
/// entity's action list (+0x208; grown with `00d06cb0` when full), and for each entity whose
/// island (+0xCC) is a real one (word +0x20 not 0xFFFF) puts the action into the first such
/// island (`00d07470`) and merges the islands of the other entities with it (`00cb5570`).
/// Without such an entity the action goes to the island of the first listed entity. Then
/// the critical section ends, the world's "action added" callback fires (`00d003c0`).
/// Returns the action.
pub fn hkp_world_add_action(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) -> u32 {
    if operations_locked(e, this) {
        queue_operation(e, this, 0xa, |e, operation| {
            e.mem.set_u32(operation + 4, action);
        });
        return 0;
    }
    e.call(ADD_REFERENCE, &args![action]);
    begin_critical_section(e, this);
    e.with_stack(0x20, |e, frame| {
        let entities = frame.addr();
        e.mem.set_u32(entities, entities + 0x10);
        e.mem.set_u32(entities + 8, 0x8000_0004);
        e.vcall(action, 0x10, &args![entities]);
        e.mem.set_u32(action + 8, this.addr());
        let mut first_entity = 0u32;
        let mut index = 0i32;
        while index < e.mem.i32(entities + 4) {
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let list = entity + 0x208;
            let capacity = e.mem.u16(list + 6) & 0x3fff;
            if e.mem.u16(list + 4) == capacity {
                e.call(ACTION_LIST_GROW, &args![list, 4u32]);
            }
            let size = e.mem.u16(list + 4);
            let data = e.mem.u32(list);
            e.mem
                .set_u32(data.wrapping_add(u32::from(size) * 4), action);
            e.mem.set_u16(list + 4, size.wrapping_add(1));
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let island = e.mem.u32(entity + 0xcc);
            if e.mem.u16(island + 0x20) != 0xffff {
                if first_entity == 0 {
                    e.call(ISLAND_ADD_ACTION, &args![island, action]);
                    first_entity = entity;
                } else if e.mem.u32(first_entity + 0xcc) != island {
                    e.call(MERGE_ISLANDS, &args![this, first_entity, entity]);
                }
            }
            index += 1;
        }
        if first_entity == 0 {
            let first = e.mem.u32(e.mem.u32(entities));
            let island = e.mem.u32(first + 0xcc);
            e.call(ISLAND_ADD_ACTION, &args![island, action]);
        }
        end_critical_section(e, this);
        e.call(FIRE_ACTION_ADDED, &args![this, action]);
        array_release(e, entities, 4, 0x10);
    });
    action
}

// Translated from 00c97510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeActionImmediately` (Xbox PDB). Adds a reference to the action, enters
/// the critical section, fires the world's "action removed" callback (`00d00460`), asks the
/// action for its entities (vtable slot 0x10) and takes it out of each one's action list
/// (+0x208: the slot found, or -1, gets the last entry; the 16-bit size drops by one), takes
/// it out of its island (+0xC; `00d07130`), clears its world (+8) and drops one reference.
/// An island whose word +0x22 is 0xFFFF is then recorded in the island's owner list (owner
/// at +0x18, list at +0x40) and given the list's size as its index. The critical section
/// ends (pending operations flushed) and the reference from the start is removed.
pub fn hkp_world_remove_action_immediately(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) {
    e.call(ADD_REFERENCE, &args![action]);
    begin_critical_section(e, this);
    e.call(FIRE_ACTION_REMOVED, &args![this, action]);
    e.with_stack(0x20, |e, frame| {
        let entities = frame.addr();
        e.mem.set_u32(entities, entities + 0x10);
        e.mem.set_u32(entities + 8, 0x8000_0004);
        e.vcall(action, 0x10, &args![entities]);
        let mut index = 0i32;
        while index < e.mem.i32(entities + 4) {
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let list = entity + 0x208;
            let count = i32::from(e.mem.u16(list + 4));
            let mut slot = -1i32;
            if count > 0 {
                let data = e.mem.u32(list);
                for candidate in 0..count {
                    if e.mem.u32(data.wrapping_add(candidate as u32 * 4)) == action {
                        slot = candidate;
                        break;
                    }
                }
            }
            let new_size = e.mem.u16(list + 4).wrapping_add(0xffff);
            e.mem.set_u16(list + 4, new_size);
            let data = e.mem.u32(list);
            let last = e.mem.u32(data.wrapping_add(u32::from(new_size) * 4));
            e.mem
                .set_u32(data.wrapping_add((slot as u32).wrapping_mul(4)), last);
            index += 1;
        }
        let island = e.mem.u32(action + 0xc);
        e.call(ISLAND_REMOVE_ACTION, &args![island, action]);
        e.mem.set_u32(action + 8, 0);
        e.call(REMOVE_REFERENCE, &args![action]);
        let owner = e.mem.u32(island + 0x18);
        if e.mem.u16(island + 0x22) == 0xffff {
            let index = e.mem.u16(owner + 0x44);
            e.mem.set_u16(island + 0x22, index);
            array_push_word(e, owner + 0x40, island);
        }
        array_release(e, entities, 4, 0x10);
    });
    end_critical_section(e, this);
    e.call(REMOVE_REFERENCE, &args![action]);
}

// Translated from 00c97d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addConstraint` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 8: constraint at +4) and 0 is returned. Otherwise: when the constraint's
/// data (+0xC) says it is of type 0xC (vtable slot 0x20) and its word +0x1C is 0, the
/// object at its +0x18 (slot 0x24) is asked for two words that go to +0x1C and +0x1E;
/// blocks pending operations (+0x9C) while it runs `00cf3e60`, gives the constraint the
/// next uid (`m_lastConstraintUid`, stored at +0x34), adds it to the simulation islands
/// (`00cb3ca0`); a constraint of kind 1 (vtable slot 0x18) also adds its action (+0x44) when
/// that has no world yet, and calls slot 0x40 of its data. Finally lifts the block and flushes
/// the pending operations when no critical section is open. Returns the `00cb3ca0` result.
pub fn hkp_world_add_constraint(e: &mut Engine, this: Ptr<HkpWorld>, constraint: u32) -> u32 {
    if operations_locked(e, this) {
        queue_operation(e, this, 8, |e, operation| {
            e.mem.set_u32(operation + 4, constraint);
        });
        return 0;
    }
    let world = this.addr();
    let data = e.mem.u32(constraint + 0xc);
    if e.vcall(data, 0x20, &args![]).u32() == 0xc && e.mem.u16(data + 0x1c) == 0 {
        let owner = e.mem.u32(data + 0x18);
        e.with_stack(8, |e, local| {
            let local = local.addr();
            // The flag byte is the low byte of a pushed register that held the local's
            // address: the upper bytes are whatever the address was.
            e.vcall(owner, 0x24, &args![(local & !0xff) | 1, local]);
            let first = e.mem.u16(local);
            let second = e.mem.u16(local + 4);
            e.mem.set_u16(data + 0x1c, first);
            e.mem.set_u16(data + 0x1e, second);
        });
    }
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, true);
    e.call(CONSTRAINT_PREPARE, &args![constraint]);
    let uid = e.get(this, HkpWorld::m_lastConstraintUid).wrapping_add(1);
    e.set(this, HkpWorld::m_lastConstraintUid, uid);
    e.mem.set_u32(constraint + 0x34, uid);
    let result = e
        .call(ADD_CONSTRAINT_SI, &args![this, constraint, 1u32])
        .u32();
    if e.vcall(constraint, 0x18, &args![]).u32() == 1 {
        let action = e.mem.u32(constraint + 0x44);
        if e.mem.u32(action + 8) == 0 {
            hkp_world_add_action(e, this, action);
        }
        let data = e.mem.u32(constraint + 0xc);
        e.vcall(data, 0x40, &args![]);
    }
    let unlocked = !operations_locked(e, this);
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, false);
    if unlocked && e.mem.u32(world + 0x8c) != 0 {
        fn_00c91120(e, this);
    }
    result
}

// Translated from 00c97e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeConstraint` (Xbox PDB), with the `hkBool*` result as the first stack
/// argument. While critical operations are locked the request is queued (operation 9:
/// constraint at +4) and `*result` is 0. Otherwise inside the critical section: a constraint
/// of kind 1 (vtable slot 0x18) whose action (+0x44) belongs to this world has the action
/// removed immediately; the constraint gets a reference, leaves the simulation islands
/// (`00cb3dd0`), is detached (`00cf3040`) and loses the reference; when the critical
/// section ends the pending operations are flushed. `*result` is 1.
pub fn hkp_world_remove_constraint(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    result: Ptr,
    constraint: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 9, |e, operation| {
            e.mem.set_u32(operation + 4, constraint);
        });
        e.mem.set_u8(result.addr(), 0);
        return;
    }
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    if e.vcall(constraint, 0x18, &args![]).u32() == 1 {
        let action = e.mem.u32(constraint + 0x44);
        if e.mem.u32(action + 8) == this.addr() {
            hkp_world_remove_action_immediately(e, this, action);
        }
    }
    e.call(ADD_REFERENCE, &args![constraint]);
    e.call(REMOVE_CONSTRAINT_SI, &args![this, constraint, 1u32]);
    e.call(CONSTRAINT_DETACH, &args![constraint, this]);
    e.call(REMOVE_REFERENCE, &args![constraint]);
    end_critical_section(e, this);
    e.mem.set_u8(result.addr(), 1);
}

// Translated from 00c97f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeAction` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 0xB: action at +4); otherwise it is `removeActionImmediately`.
pub fn hkp_world_remove_action(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0xb, |e, operation| {
            e.mem.set_u32(operation + 4, action);
        });
        return;
    }
    hkp_world_remove_action_immediately(e, this, action);
}

// Translated from 00c98310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98310(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00c98320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98320(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00c98330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98330(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00c98340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98340(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9_0e40,
            hkp_contact_mgr_factory_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00c9_0e60, fn_00c90e60(Ptr, f32, f32)),
        entry!(0x00c9_0ed0, fn_00c90ed0(Ptr, u32, u32, i32)),
        entry!(0x00c9_0f30, hkp_multithread_config_hkp_multithread_config(Ptr) -> Ptr),
        entry!(0x00c9_1000, fn_00c91000(Ptr<HkpWorld>, Ptr)),
        entry!(
            0x00c9_1020,
            hkp_world_get_current_time(Ptr<HkpWorld>) -> f32
        ),
        entry!(
            0x00c9_1030,
            hkp_world_get_current_psi_time(Ptr<HkpWorld>) -> f32
        ),
        entry!(0x00c9_1040, fn_00c91040(Ptr<HkpWorld>, f32)),
        entry!(0x00c9_1060, fn_00c91060(Ptr<HkpWorld>) -> u32),
        entry!(0x00c9_10a0, fn_00c910a0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_10b0, hkp_world_check_determinism(Ptr<HkpWorld>)),
        entry!(0x00c9_10d0, fn_00c910d0(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1110, fn_00c91110(Ptr<HkpWorld>) -> Ptr),
        entry!(0x00c9_1120, fn_00c91120(Ptr<HkpWorld>)),
        entry!(0x00c9_1140, fn_00c91140(Ptr<HkpWorld>)),
        entry!(0x00c9_1160, fn_00c91160(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1180, fn_00c91180(Ptr<HkpWorld>)),
        entry!(0x00c9_1190, fn_00c91190(Ptr<HkpWorld>)),
        entry!(0x00c9_11a0, fn_00c911a0(Ptr<HkpWorld>)),
        entry!(0x00c9_11b0, hkp_world_unlock_read_only(Ptr<HkpWorld>)),
        entry!(
            0x00c9_11d0,
            hkp_world_lock_island_for_constraint_update(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_11f0, fn_00c911f0(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_1210,
            hkp_world_unlock_island_for_constraint_update(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_1230, fn_00c91230(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1250, fn_00c91250(Ptr<HkpWorld>)),
        entry!(0x00c9_1270, fn_00c91270(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00c9_13b0,
            hkp_world_reenable_collision_between_entity_pair(Ptr<HkpWorld>, Ptr, Ptr)
        ),
        entry!(0x00c9_1470, fn_00c91470(Ptr<HkpWorld>, u32, u32, u32)),
        entry!(
            0x00c9_14d0,
            hkp_world_add_entity(Ptr<HkpWorld>, Ptr<HkpEntity>, u32) -> Ptr<HkpEntity>
        ),
        entry!(
            0x00c9_1620,
            hkp_world_remove_entity(Ptr<HkpWorld>, Ptr, Ptr<HkpEntity>)
        ),
        entry!(0x00c9_1700, fn_00c91700(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1750, fn_00c91750(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_17a0, fn_00c917a0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_17f0, fn_00c917f0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1840, fn_00c91840(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_18e0, fn_00c918e0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1a20, fn_00c91a20(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1a70, fn_00c91a70(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_1b10,
            hkp_world_step_delta_time(Ptr<HkpWorld>, f32) -> u32
        ),
        entry!(0x00c9_1b70, fn_00c91b70(Ptr<HkpWorld>, f32) -> u32),
        entry!(0x00c9_1ba0, fn_00c91ba0(Ptr<HkpWorld>) -> u32),
        entry!(0x00c9_1bd0, fn_00c91bd0(Ptr<HkpWorld>) -> u32),
        entry!(
            0x00c9_1c00,
            hkp_world_finish_mt_step(Ptr<HkpWorld>, u32, u32) -> u32
        ),
        entry!(0x00c9_1c50, fn_00c91c50(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1f70, hkp_world_cast_ray(Ptr<HkpWorld>, u32, u32)),
        entry!(0x00c9_2040, hkp_world_cast_ray_ov2(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00c9_2110,
            fn_00c92110(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(
            0x00c9_2290,
            fn_00c92290(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(
            0x00c9_2430,
            hkp_world_linear_cast(Ptr<HkpWorld>, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_2540,
            fn_00c92540(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(0x00c9_26a0, fn_00c926a0(Ptr, Ptr<HkpWorld>, Ptr) -> Ptr),
        entry!(0x00c9_2760, fn_00c92760(Ptr, Ptr<HkpWorld>, Ptr) -> Ptr),
        entry!(0x00c9_27f0, fn_00c927f0(Ptr, Ptr)),
        entry!(0x00c9_28c0, fn_00c928c0(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_29e0, fn_00c929e0(Ptr<HkpWorld>) -> Ptr),
        entry!(0x00c9_2c70, hkp_world_remove_phantom(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_36a0,
            hkp_world_add_phantom(Ptr<HkpWorld>, u32) -> u32
        ),
        entry!(0x00c9_3c30, hkp_world_activate_region(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_44a0,
            hkp_world_update_collision_filter_on_phantom(Ptr<HkpWorld>, u32, u32)
        ),
        entry!(
            0x00c9_46e0,
            hkp_world_update_collision_filter_on_entity(Ptr<HkpWorld>, u32, i32, u32)
        ),
        entry!(
            0x00c9_4bd0,
            hkp_world_add_entity_batch(Ptr<HkpWorld>, u32, i32, i32)
        ),
        entry!(
            0x00c9_51b0,
            hkp_world_remove_entity_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_54e0,
            hkp_world_add_phantom_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_5800,
            hkp_world_remove_phantom_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_6ff0,
            hkp_world_update_collision_filter_on_world(Ptr<HkpWorld>, i32, u32)
        ),
        entry!(0x00c9_7370, hkp_world_add_action(Ptr<HkpWorld>, u32) -> u32),
        entry!(
            0x00c9_7510,
            hkp_world_remove_action_immediately(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00c9_7d30,
            hkp_world_add_constraint(Ptr<HkpWorld>, u32) -> u32
        ),
        entry!(
            0x00c9_7e80,
            hkp_world_remove_constraint(Ptr<HkpWorld>, Ptr, u32)
        ),
        entry!(0x00c9_7f50, hkp_world_remove_action(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_8310, fn_00c98310(Ptr, u32) -> u32),
        entry!(0x00c9_8320, fn_00c98320(Ptr, u32) -> u32),
        entry!(0x00c9_8330, fn_00c98330(Ptr, u32) -> u32),
        entry!(0x00c9_8340, fn_00c98340(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Addresses for test doubles that stand in for virtual functions.
    const FAKE_1: u32 = 0x0900_0001;
    const FAKE_2: u32 = 0x0900_0002;
    const FAKE_3: u32 = 0x0900_0003;
    const FAKE_4: u32 = 0x0900_0004;
    const FAKE_5: u32 = 0x0900_0005;

    type Records = Rc<RefCell<Vec<Vec<u32>>>>;
    type Operations = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    fn logging() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `address`, in order.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(called, _)| *called == address)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The addresses of the logged calls that are in `wanted`, in order.
    fn order_of(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .map(|(called, _)| *called)
                    .filter(|called| wanted.contains(called))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Doubles that do nothing and return 0.
    fn silent(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn returning(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that stores its argument words.
    fn recording(e: &mut Engine, address: u32) -> Records {
        let records: Records = Rc::new(RefCell::new(vec![]));
        let sink = records.clone();
        e.register_double(address, move |_, words| {
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        records
    }

    /// An object of `size` bytes whose vtable (also on the heap) holds `(slot, target)`.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    fn world(e: &mut Engine) -> Ptr<HkpWorld> {
        e.new_object()
    }

    /// A heap array of the words; one spare word sits before its data.
    fn array_of(e: &mut Engine, values: &[u32]) -> u32 {
        let block = e.mem.alloc(8 + 4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(block + 4 + 4 * index as u32, *value);
        }
        block + 4
    }

    /// The 16 bytes of an operation record as the queue functions build them.
    fn op_bytes(kind: u8, word_4: u32, word_8: u32, byte_a: u8) -> Vec<u8> {
        let mut bytes = vec![0u8; 16];
        bytes[0] = kind;
        bytes[4..8].copy_from_slice(&word_4.to_le_bytes());
        bytes[8..12].copy_from_slice(&word_8.to_le_bytes());
        bytes[10] = byte_a;
        bytes
    }

    /// Records `(queue, record bytes)` of every operation handed to the queue.
    fn operation_recorder(e: &mut Engine) -> Operations {
        let records: Operations = Rc::new(RefCell::new(vec![]));
        let sink = records.clone();
        e.register_double(QUEUE_ENQUEUE, move |e, words| {
            sink.borrow_mut()
                .push((words[0], e.mem.bytes(words[1], 16)));
            Ret::default()
        });
        records
    }

    #[test]
    fn contact_mgr_factory_names_its_class_to_the_collector() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1, FAKE_2]);
        let collector = object_with_vtable(&mut e, 0x10, &[(0x18, FAKE_1), (0x1c, FAKE_2)]);
        e.call(0x00c9_0e40, &args![0u32, collector, 0u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![collector, 0x010c_63a0]]);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![collector]]);
    }

    #[test]
    fn fn_00c90e60_stores_the_values_and_ratios() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(HALF, 0.5f64);
        let object = e.mem.alloc(0x60);
        e.call(0x00c9_0e60, &args![object, 4.0f32, 2.0f32]);
        assert_eq!(e.mem.f32(object + 4), 4.0);
        assert_eq!(e.mem.f32(object + 0xc), 2.0);
        assert_eq!(e.mem.f32(object + 8), 2.0);
        assert_eq!(e.mem.f32(object + 0x50), 0.5);
        assert_eq!(e.mem.f32(object + 0x58), 1.0);
        assert_eq!(e.mem.f32(object + 0x54), 2.0);
        assert_eq!(e.mem.f32(object + 0x5c), 1.0);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(object + 0x30 + 4 * lane), 2.0);
            assert_eq!(e.mem.f32(object + 0x40 + 4 * lane), 0.5);
        }
        // A signalling NaN is quieted by the FLD/FSTP pair.
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(0x00c9_0e60, &args![object, signalling, 2.0f32]);
        assert_eq!(e.mem.u32(object + 4), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c90ed0_shifts_the_arguments_into_the_top_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        // The byte at +9 is 1: `c = 3` is not below it, so nothing is complemented.
        e.mem.set_u8(object + 9, 1);
        e.mem.set_u16(object + 0xa, 0x1234);
        e.mem.set_u16(object + 0xc, 0xffff);
        e.call(0x00c9_0ed0, &args![object, 1u32, 2u32, 3i32]);
        assert_eq!(e.mem.u16(object + 0xa), 0x1234 | 0x4000);
        assert_eq!(e.mem.u16(object + 0xc), 0x3fff | 0x8000);
        // The byte is 3 and `c = 1` is below it in both comparisons: both are complemented.
        e.mem.set_u8(object + 9, 3);
        e.mem.set_u16(object + 0xa, 0);
        e.mem.set_u16(object + 0xc, 0);
        e.call(0x00c9_0ed0, &args![object, 0u32, 1u32, 1i32]);
        assert_eq!(e.mem.u16(object + 0xa), 0xc000);
        assert_eq!(e.mem.u16(object + 0xc), 0x8000);
    }

    #[test]
    fn multithread_config_constructor_sets_its_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        let returned = e.call(0x00c9_0f30, &args![object]).u32();
        assert_eq!(returned, object);
        assert_eq!(e.mem.u32(object), 0x46);
    }

    #[test]
    fn fn_00c91000_copies_a_vector() {
        let mut e = Engine::new();
        let world = world(&mut e);
        let source = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(source + 4 * lane, 0x1111_1111 * (lane + 1));
        }
        e.call(0x00c9_1000, &args![world, source]);
        for lane in 0..4 {
            let copied = e.mem.u32(world.addr() + 0x10 + 4 * lane);
            assert_eq!(copied, 0x1111_1111 * (lane + 1));
        }
    }

    #[test]
    fn current_times_come_from_the_simulation() {
        let mut e = Engine::new();
        let world = world(&mut e);
        let simulation = e.mem.alloc(0x40);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.mem.set_f32(simulation + 0x14, 1.5);
        e.mem.set_f32(simulation + 0x18, 2.5);
        assert_eq!(e.call(0x00c9_1020, &args![world]).f32(), 1.5);
        assert_eq!(e.call(0x00c9_1030, &args![world]).f32(), 2.5);
        // FLD quiets a signalling NaN.
        e.mem.set_u32(simulation + 0x14, 0x7f80_0001);
        let quieted = e.call(0x00c9_1020, &args![world]).f32();
        assert_eq!(quieted.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c91040_passes_the_time_to_the_simulation() {
        let mut e = logging();
        silent(&mut e, &[SIMULATION_SET_TIME]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        e.call(0x00c9_1040, &args![world, 0.25f32]);
        assert_eq!(
            calls_to(&e, SIMULATION_SET_TIME),
            vec![vec![0x1234, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn fn_00c91060_returns_the_simulations_answer() {
        let mut e = logging();
        returning(&mut e, SIMULATION_TEST, 0x0102);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        assert_eq!(e.call(0x00c9_1060, &args![world]).u32(), 0x0102);
        assert_eq!(calls_to(&e, SIMULATION_TEST), vec![vec![0x1234]]);
    }

    /// A function that does nothing makes no calls of its own.
    fn assert_does_nothing(address: u32, words: &[u32]) {
        let mut e = logging();
        e.call(address, words);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn fn_00c910a0_does_nothing() {
        assert_does_nothing(0x00c9_10a0, &[0x1000, 5]);
    }

    #[test]
    fn check_determinism_does_nothing() {
        assert_does_nothing(0x00c9_10b0, &[0x1000]);
    }

    #[test]
    fn fn_00c911a0_does_nothing() {
        assert_does_nothing(0x00c9_11a0, &[0x1000]);
    }

    #[test]
    fn unlock_read_only_does_nothing() {
        assert_does_nothing(0x00c9_11b0, &[0x1000]);
    }

    #[test]
    fn fn_00c910d0_swaps_the_border_and_its_references() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1, ADD_REFERENCE, REMOVE_REFERENCE]);
        let world = world(&mut e);
        let old = object_with_vtable(&mut e, 0x40, &[(0x10, FAKE_1)]);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(old));
        e.call(0x00c9_10d0, &args![world, 0x5000u32]);
        assert_eq!(e.get(world, HkpWorld::m_broadPhaseBorder).addr(), 0x5000);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![old]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![old]]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x5000]]);
        // Clearing the border releases the old one and adds nothing.
        let second = object_with_vtable(&mut e, 0x40, &[(0x10, FAKE_1)]);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(second));
        e.call(0x00c9_10d0, &args![world, 0u32]);
        assert_eq!(e.get(world, HkpWorld::m_broadPhaseBorder).addr(), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![old], vec![second]]
        );
        // With no old border nothing is released.
        e.call(0x00c9_10d0, &args![world, 0x6000u32]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 2);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 2);
    }

    #[test]
    fn fn_00c91110_returns_the_border() {
        let mut e = Engine::new();
        let world = world(&mut e);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(0x4321));
        assert_eq!(e.call(0x00c9_1110, &args![world]).u32(), 0x4321);
    }

    #[test]
    fn fn_00c91120_clears_the_count_and_flushes_the_queue() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_FLUSH_PENDING]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingOperationsCount, 7);
        e.call(0x00c9_1120, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_00c91140_clears_the_body_count_and_flushes_the_queue() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_FLUSH_BODY_PENDING]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingBodyOperationsCount, 7);
        e.call(0x00c9_1140, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_pendingBodyOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_BODY_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_00c91160_enqueues_the_operation() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_ENQUEUE]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1160, &args![world, 0x6000u32]);
        assert_eq!(calls_to(&e, QUEUE_ENQUEUE), vec![vec![0x5000, 0x6000]]);
    }

    #[test]
    fn lock_and_unlock_count_the_world_and_call_the_helpers() {
        let mut e = logging();
        silent(&mut e, &[LOCK_ENTER, LOCK_LEAVE]);
        let world = world(&mut e);
        e.call(0x00c9_1180, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 1);
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![world.addr()]]);
        e.call(0x00c9_1190, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 0);
        assert_eq!(calls_to(&e, LOCK_LEAVE), vec![vec![world.addr()]]);
    }

    /// Runs the critical-section function at `address` with and without a section.
    fn check_critical_section(address: u32, import_slot: u32) {
        let mut e = logging();
        silent(&mut e, &[import_slot]);
        let world = world(&mut e);
        e.call(address, &args![world, 0u32]);
        assert!(calls_to(&e, import_slot).is_empty());
        e.set(
            world,
            HkpWorld::m_modifyConstraintCriticalSection,
            Ptr::new(0x7000),
        );
        e.call(address, &args![world, 0u32]);
        assert_eq!(calls_to(&e, import_slot), vec![vec![0x7000]]);
    }

    #[test]
    fn lock_island_for_constraint_update_enters_the_section() {
        check_critical_section(0x00c9_11d0, ENTER_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c911f0_enters_the_section() {
        check_critical_section(0x00c9_11f0, ENTER_CRITICAL_SECTION);
    }

    #[test]
    fn unlock_island_for_constraint_update_leaves_the_section() {
        check_critical_section(0x00c9_1210, LEAVE_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c91230_leaves_the_section() {
        check_critical_section(0x00c9_1230, LEAVE_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c91250_tells_the_tree_when_there_is_one() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        e.call(0x00c9_1250, &args![world]);
        assert!(calls_to(&e, FAKE_1).is_empty());
        let tree = object_with_vtable(&mut e, 0x20, &[(0x14, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1250, &args![world]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0]]);
    }

    #[test]
    fn fn_00c91270_calls_the_trees_slot_with_both_arguments() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1270, &args![world, 5u32, 6u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 5, 6]]);
    }

    #[test]
    fn fn_00c912e0_registers_three_helpers() {
        let mut e = logging();
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x20)).collect();
        let next = Rc::new(RefCell::new(blocks.clone().into_iter()));
        e.register_double(FAKE_1, move |_, _| Ret {
            eax: next.borrow_mut().next().unwrap_or(0),
            ..Ret::default()
        });
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        returning(&mut e, MAKE_HELPER_FIRST, 0xf100);
        returning(&mut e, MAKE_HELPER_SECOND, 0xf200);
        silent(&mut e, &[REGISTER_HELPER, REMOVE_REFERENCE]);
        fn_00c912e0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(
            calls_to(&e, MAKE_HELPER_FIRST),
            vec![vec![blocks[0], 0x1000]]
        );
        assert_eq!(
            calls_to(&e, MAKE_HELPER_SECOND),
            vec![vec![blocks[1], 0x1000]]
        );
        assert_eq!(e.mem.u16(blocks[0] + 4), 0xc);
        assert_eq!(e.mem.u16(blocks[1] + 4), 0xc);
        let third = blocks[2];
        assert_eq!(e.mem.u16(third + 4), 0x14);
        assert_eq!(e.mem.u16(third + 6), 1);
        assert_eq!(e.mem.u16(third + 0xe), 1);
        assert_eq!(e.mem.u32(third), 0x010c_6420);
        assert_eq!(e.mem.u32(third + 8), 0x010c_6378);
        assert_eq!(e.mem.u32(third + 0x10), 3);
        assert_eq!(
            calls_to(&e, REGISTER_HELPER),
            vec![
                vec![0x2000, 0xf100, 1],
                vec![0x2000, 0xf200, 2],
                vec![0x2000, third, 3]
            ]
        );
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0xf100], vec![0xf200], vec![third]]
        );
    }

    /// Runs `reenableCollisionBetweenEntityPair` on a world whose broad phase answers
    /// `accept`, whose filter says `enabled` and whose agent search finds `agent`. Returns
    /// the pairs handed to the dispatcher as `[dispatcher, handle a, handle b, count, filter
    /// base]`, and the two entities and the filter.
    fn reenable_scenario(accept: u32, enabled: u8, agent: u32) -> (Vec<Vec<u32>>, [u32; 3]) {
        let mut e = logging();
        let pairs: Records = Rc::new(RefCell::new(vec![]));
        let sink = pairs.clone();
        e.register_double(ADD_PAIRS, move |e, words| {
            let pair = words[1];
            sink.borrow_mut().push(vec![
                words[0],
                e.mem.u32(pair),
                e.mem.u32(pair + 4),
                words[2],
                words[3],
            ]);
            Ret::default()
        });
        returning(&mut e, FIND_AGENT, agent);
        returning(&mut e, FAKE_3, accept);
        e.register_double(FAKE_2, move |e, words| {
            e.mem.set_u8(words[1], enabled);
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        let world = world(&mut e);
        let broad_phase = object_with_vtable(&mut e, 0x20, &[(0x4c, FAKE_3)]);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        // The filter's base is at +8: its vtable pointer sits there and slot +4 answers.
        let filter = e.mem.alloc(0x40);
        let filter_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(filter + 8, filter_vtable);
        e.mem.set_u32(filter_vtable + 4, FAKE_2);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        e.set(world, HkpWorld::m_broadPhaseDispatcher, Ptr::new(0x8000));
        let entity_a = e.mem.alloc(0x40);
        let entity_b = e.mem.alloc(0x40);
        e.call(0x00c9_13b0, &args![world, entity_a, entity_b]);
        let recorded = pairs.borrow().clone();
        (recorded, [entity_a, entity_b, filter])
    }

    #[test]
    fn reenable_collision_adds_the_pair_when_everything_allows() {
        let (pairs, [entity_a, entity_b, filter]) = reenable_scenario(1, 1, 0);
        assert_eq!(
            pairs,
            vec![vec![
                0x8000,
                entity_a + 0x24,
                entity_b + 0x24,
                1,
                filter + 8
            ]]
        );
    }

    #[test]
    fn reenable_collision_stops_at_each_refusal() {
        assert!(reenable_scenario(0, 1, 0).0.is_empty());
        assert!(reenable_scenario(1, 0, 0).0.is_empty());
        assert!(reenable_scenario(1, 1, 0x99).0.is_empty());
    }

    #[test]
    fn reenable_collision_is_queued_while_locked() {
        let mut e = logging();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_13b0, &args![world, 0x111u32, 0x222u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x13, 0x111, 0x222, 0))]
        );
    }

    #[test]
    fn fn_00c91470_queues_or_calls_the_simulation() {
        let mut e = logging();
        let queued = operation_recorder(&mut e);
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        let simulation = object_with_vtable(&mut e, 0x40, &[(0x30, FAKE_1)]);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1470, &args![world, 1u32, 0x12345u32, 0x1ffu32]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 1, 0x12345, world.addr(), 0x1ff]]
        );
        assert!(queued.borrow().is_empty());
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.call(0x00c9_1470, &args![world, 1u32, 0x12345u32, 0x1ffu32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x17, 1, 0x2345, 0xff))]
        );
        assert_eq!(calls_to(&e, FAKE_1).len(), 1);
    }

    /// An entity whose vtable slot +0x10 makes a motion and slot +0x14 is the deleter.
    fn entity(e: &mut Engine) -> Ptr<HkpEntity> {
        Ptr::new(object_with_vtable(
            e,
            0x220,
            &[(0x10, FAKE_1), (0x14, FAKE_2)],
        ))
    }

    fn add_entity_engine() -> Engine {
        let mut e = logging();
        silent(
            &mut e,
            &[
                SET_CACHED_SHAPE_DATA,
                SET_TIME_INFORMATION,
                ADD_REFERENCE,
                ADD_ENTITY_SI,
                ADD_ENTITY_BP,
                FIRE_WORLD_ENTITY_ADDED,
                FIRE_ENTITY_ADDED,
                QUEUE_FLUSH_PENDING,
                QUEUE_FLUSH_BODY_PENDING,
                FAKE_2,
            ],
        );
        returning(&mut e, FAKE_1, 0x5555);
        e
    }

    #[test]
    fn add_entity_runs_the_whole_sequence() {
        let mut e = add_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(world, HkpWorld::m_lastEntityUid, 41);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(world, HkpWorld::m_pendingOperationQueueCount, 1);
        e.set(world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.mem.set_u8(world.addr() + 0x305, 3);
        e.mem.set_u8(world.addr() + 0x306, 2);
        e.mem.set_u8(world.addr() + 0x307, 1);
        e.mem.set_u8(entity.addr() + 0xe9, 2);
        e.set(entity, HkpEntity::m_collidableShape, Ptr::new(0x777));
        e.set(entity, HkpEntity::m_wordEa, 0x1111);
        e.set(entity, HkpEntity::m_wordEc, 0x2222);
        let returned = e.call(0x00c9_14d0, &args![world, entity, 9u32]).u32();
        assert_eq!(returned, entity.addr());
        assert_eq!(e.get(entity, HkpEntity::m_collidableMotion).addr(), 0x5555);
        assert_eq!(e.get(entity, HkpEntity::m_uid), 42);
        assert_eq!(e.get(world, HkpWorld::m_lastEntityUid), 42);
        assert_eq!(e.get(world, HkpWorld::m_criticalOperationsLockCount), 0);
        assert!(e.get(world, HkpWorld::m_criticalOperationsAllowed));
        // The two words were cleared, then `00c90ed0(3, 2, 1)` on the motion with the byte 2
        // at +9: both arguments are complemented, giving 0 and 0x4000.
        assert_eq!(e.get(entity, HkpEntity::m_wordEa), 0);
        assert_eq!(e.get(entity, HkpEntity::m_wordEc), 0x4000);
        assert_eq!(
            calls_to(&e, SET_CACHED_SHAPE_DATA),
            vec![vec![entity.addr(), world.addr(), 0x777]]
        );
        assert_eq!(
            calls_to(&e, SET_TIME_INFORMATION),
            vec![vec![0, 0, entity.addr() + 0xf0]]
        );
        assert_eq!(
            calls_to(&e, ADD_ENTITY_SI),
            vec![vec![world.addr(), entity.addr(), 9]]
        );
        let sequence = [
            SET_CACHED_SHAPE_DATA,
            SET_TIME_INFORMATION,
            ADD_REFERENCE,
            ADD_ENTITY_SI,
            ADD_ENTITY_BP,
            FIRE_WORLD_ENTITY_ADDED,
            FIRE_ENTITY_ADDED,
            QUEUE_FLUSH_PENDING,
            QUEUE_FLUSH_BODY_PENDING,
        ];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 0);
        assert_eq!(e.get(world, HkpWorld::m_pendingBodyOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn add_entity_keeps_an_existing_motion_and_respects_the_block_flag() {
        let mut e = add_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(entity, HkpEntity::m_collidableMotion, Ptr::new(0x6666));
        e.set(world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(world, HkpWorld::m_blockExecutingPendingOperations, true);
        e.call(0x00c9_14d0, &args![world, entity, 9u32]);
        assert_eq!(e.get(entity, HkpEntity::m_collidableMotion).addr(), 0x6666);
        assert!(calls_to(&e, FAKE_1).is_empty());
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 3);
        assert!(calls_to(&e, QUEUE_FLUSH_PENDING).is_empty());
    }

    #[test]
    fn add_entity_is_queued_inside_a_critical_section() {
        let mut e = add_entity_engine();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 2);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let returned = e.call(0x00c9_14d0, &args![world, entity, 9u32]).u32();
        assert_eq!(returned, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(1, entity.addr(), 9, 0))]
        );
        assert!(calls_to(&e, ADD_ENTITY_SI).is_empty());
    }

    fn remove_entity_engine() -> Engine {
        let mut e = logging();
        silent(
            &mut e,
            &[
                REMOVE_ENTITY_SI,
                FIRE_WORLD_ENTITY_REMOVED,
                FIRE_ENTITY_REMOVED,
                REMOVE_ENTITY_BP,
                ENTITY_RELEASE_BOUNDING_DATA,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING,
                QUEUE_FLUSH_BODY_PENDING,
                FAKE_1,
                FAKE_2,
            ],
        );
        e
    }

    #[test]
    fn remove_entity_runs_the_whole_sequence() {
        let mut e = remove_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.set(world, HkpWorld::m_pendingOperationsCount, 1);
        e.set(
            entity,
            HkpEntity::m_boundingVolumeChildData,
            Ptr::new(0x9000),
        );
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.get(world, HkpWorld::m_criticalOperationsLockCount), 0);
        assert!(e.get(world, HkpWorld::m_criticalOperationsAllowed));
        let sequence = [
            REMOVE_ENTITY_SI,
            FIRE_WORLD_ENTITY_REMOVED,
            FIRE_ENTITY_REMOVED,
            REMOVE_ENTITY_BP,
            ENTITY_RELEASE_BOUNDING_DATA,
            FAKE_2,
            REMOVE_REFERENCE,
            QUEUE_FLUSH_PENDING,
        ];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(
            calls_to(&e, ENTITY_RELEASE_BOUNDING_DATA),
            vec![vec![entity.addr() + 0x30]]
        );
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![entity.addr()]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![entity.addr()]]);
    }

    #[test]
    fn remove_entity_skips_the_optional_steps() {
        let mut e = remove_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.set(entity, HkpEntity::m_memSizeAndFlags, 0x20);
        e.set(world, HkpWorld::m_blockExecutingPendingOperations, true);
        e.set(world, HkpWorld::m_pendingOperationsCount, 1);
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert!(calls_to(&e, ENTITY_RELEASE_BOUNDING_DATA).is_empty());
        assert!(calls_to(&e, FAKE_2).is_empty());
        assert!(calls_to(&e, QUEUE_FLUSH_PENDING).is_empty());
        assert_eq!(e.mem.u8(result), 1);
    }

    #[test]
    fn remove_entity_is_queued_inside_a_critical_section() {
        let mut e = remove_entity_engine();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.mem.set_u8(result, 7);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(2, entity.addr(), 0, 0))]
        );
        assert_eq!(e.mem.u8(result), 0);
        assert!(calls_to(&e, REMOVE_ENTITY_SI).is_empty());
    }

    /// Nulls a listener in the array at `data_offset` through the function at `address`.
    fn check_listener_removal(address: u32, data_offset: u32) {
        let mut e = Engine::new();
        let world = world(&mut e);
        let data = array_of(&mut e, &[0x10, 0x20, 0x30]);
        e.mem.set_u32(world.addr() + data_offset, data);
        e.mem.set_i32(world.addr() + data_offset + 4, 3);
        e.call(address, &args![world, 0x20u32]);
        assert_eq!(
            (e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)),
            (0x10, 0, 0x30)
        );
        // No match: the word before the array is cleared.
        e.mem.set_u32(data - 4, 0xdead);
        e.call(address, &args![world, 0x99u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
        assert_eq!(e.mem.u32(data), 0x10);
        // An empty array behaves the same way.
        e.mem.set_u32(data - 4, 0xdead);
        e.mem.set_i32(world.addr() + data_offset + 4, 0);
        e.call(address, &args![world, 0x10u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
        assert_eq!(e.mem.u32(data), 0x10);
    }

    #[test]
    fn fn_00c91700_removes_a_listener() {
        check_listener_removal(0x00c9_1700, 0x11c);
    }

    #[test]
    fn fn_00c91750_removes_a_listener() {
        check_listener_removal(0x00c9_1750, 0x140);
    }

    #[test]
    fn fn_00c917a0_removes_a_listener() {
        check_listener_removal(0x00c9_17a0, 0x128);
    }

    #[test]
    fn fn_00c917f0_removes_a_listener() {
        check_listener_removal(0x00c9_17f0, 0x134);
    }

    #[test]
    fn fn_00c91840_removes_a_listener() {
        check_listener_removal(0x00c9_1840, 0x158);
    }

    #[test]
    fn fn_00c918e0_removes_a_listener() {
        check_listener_removal(0x00c9_18e0, 0x164);
    }

    #[test]
    fn fn_00c91a20_removes_a_listener() {
        check_listener_removal(0x00c9_1a20, 0x1a0);
    }

    #[test]
    fn fn_00c91a70_removes_a_listener() {
        check_listener_removal(0x00c9_1a70, 0x14c);
    }

    /// A world whose simulation's slots +0xc, +0x10, +0x14 and +0x18 all answer 0x77.
    fn stepping_world(e: &mut Engine) -> (Ptr<HkpWorld>, u32) {
        silent(e, &[LOCK_ENTER, LOCK_LEAVE, WATCH_HEAP_MEMORY]);
        let world = world(e);
        let slots = [
            (0xc, FAKE_1),
            (0x10, FAKE_1),
            (0x14, FAKE_1),
            (0x18, FAKE_1),
        ];
        let simulation = object_with_vtable(e, 0x40, &slots);
        returning(e, FAKE_1, 0x77);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        (world, simulation)
    }

    #[test]
    fn step_delta_time_steps_inside_the_lock() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        let result = e.call(0x00c9_1b10, &args![world, 0.5f32]).u32();
        assert_eq!(result, 0x77);
        let sequence = [LOCK_ENTER, WATCH_HEAP_MEMORY, FAKE_1, LOCK_LEAVE];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(calls_to(&e, WATCH_HEAP_MEMORY), vec![vec![world.addr()]]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 0.5f32.to_bits()]]
        );
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 0);
    }

    #[test]
    fn step_delta_time_tells_the_tree_afterwards() {
        let mut e = logging();
        let (world, _) = stepping_world(&mut e);
        silent(&mut e, &[FAKE_2, FAKE_3]);
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_2), (0x14, FAKE_3)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1b10, &args![world, 0.5f32]);
        assert!(calls_to(&e, FAKE_2).is_empty());
        assert_eq!(calls_to(&e, FAKE_3), vec![vec![tree, 0]]);
        e.set(world, HkpWorld::m_autoUpdateTree, true);
        e.call(0x00c9_1b10, &args![world, 0.5f32]);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![tree, 0, 0]]);
        assert_eq!(calls_to(&e, FAKE_3).len(), 1);
    }

    #[test]
    fn fn_00c91b70_passes_the_time_to_slot_0x10() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        let result = e.call(0x00c9_1b70, &args![world, 2.0f32]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 2.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn fn_00c91ba0_calls_slot_0x14_locked() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        assert_eq!(e.call(0x00c9_1ba0, &args![world]).u32(), 0x77);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![simulation]]);
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn fn_00c91bd0_calls_slot_0x18_locked() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        assert_eq!(e.call(0x00c9_1bd0, &args![world]).u32(), 0x77);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![simulation]]);
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn finish_mt_step_returns_a_failure_and_otherwise_tells_the_tree() {
        let mut e = logging();
        silent(&mut e, &[FAKE_2, FAKE_3]);
        returning(&mut e, SIMULATION_FINISH_MT_STEP, 5);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_2), (0x14, FAKE_3)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 5);
        assert_eq!(
            calls_to(&e, SIMULATION_FINISH_MT_STEP),
            vec![vec![0x1234, 7, 8]]
        );
        assert!(calls_to(&e, FAKE_2).is_empty() && calls_to(&e, FAKE_3).is_empty());
        returning(&mut e, SIMULATION_FINISH_MT_STEP, 0);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_3), vec![vec![tree, 0]]);
        e.set(world, HkpWorld::m_autoUpdateTree, true);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![tree, 7, 8]]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::NULL);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_2).len(), 1);
    }

    /// The plain copies of `00c91c50`: (offset in the world, offset in the info, bytes).
    const INFO_COPIES: [(u32, u32, u32); 33] = [
        (0xd0, 0x90, 4),
        (0xdc, 0xac, 4),
        (0xd8, 0xa8, 4),
        (0xf4, 0xc0, 4),
        (0xf8, 0xc4, 4),
        (0xfc, 0xc8, 4),
        (0x100, 0xcc, 4),
        (0x240, 0x24, 4),
        (0x1e4, 0x78, 4),
        (0x1e8, 0x7c, 4),
        (0x338, 0x20, 4),
        (0x330, 0x70, 4),
        (0x334, 0x64, 4),
        (0x2f4, 0x80, 4),
        (0x2f8, 0x84, 4),
        (0xe0, 0xb0, 4),
        (0xe8, 0xb8, 4),
        (0xec, 0xbc, 4),
        (0xb4, 0xd4, 4),
        (0x60, 0x53, 1),
        (0xd4, 0x98, 1),
        (0x340, 0x74, 1),
        (0x304, 0x8c, 1),
        (0x305, 0xa4, 1),
        (0x306, 0xa5, 1),
        (0x307, 0xa6, 1),
        (0xd6, 0xa7, 1),
        (0xe4, 0xb4, 1),
        (0xd5, 0xd0, 1),
        (0xf0, 0xd1, 1),
        (0xc8, 0xd2, 1),
        (0xb0, 0xd8, 1),
        (0xb1, 0xd9, 1),
    ];

    #[test]
    fn get_info_copies_the_worlds_settings() {
        let mut e = logging();
        silent(&mut e, &[ADD_REFERENCE, REMOVE_REFERENCE]);
        e.register(X87_SQRT, |_, words| Ret {
            st0: f64::take(words, &mut 0).sqrt(),
            ..Ret::default()
        });
        let world = world(&mut e);
        let w = world.addr();
        let info = e.mem.alloc(0x100);
        for (index, (from, _, size)) in INFO_COPIES.iter().enumerate() {
            let value = 0x3f00_0000 + index as u32 + 1;
            if *size == 4 {
                e.mem.set_u32(w + from, value);
            } else {
                e.mem.set_u8(w + from, value as u8);
            }
        }
        // The word at +0xcc is copied first and then overwritten by the one at +0xd0, which
        // is a signalling NaN here and comes out quieted.
        e.mem.set_u32(w + 0xcc, 0x3e99_999a);
        e.mem.set_u32(w + 0xd0, 0x7f80_0001);
        for lane in 0..4 {
            e.mem.set_u32(w + 0x10 + 4 * lane, 0xa0 + lane);
            e.mem.set_u32(w + 0x310 + 4 * lane, 0xb0 + lane);
            e.mem.set_u32(w + 0x320 + 4 * lane, 0xc0 + lane);
        }
        let collision_input = e.mem.alloc(0x80);
        let extra = e.mem.alloc(8);
        e.mem.set_f32(collision_input + 8, 1.25);
        e.mem.set_u8(collision_input + 0x6d, 0x6d);
        e.mem.set_u8(collision_input + 0x6c, 0x6c);
        e.mem.set_u32(collision_input + 0x70, extra);
        e.mem.set_f32(extra, 3.5);
        e.mem.set_u32(extra + 4, 0x1357);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(collision_input));
        let dispatcher = e.mem.alloc(0x1e50);
        e.mem.set_f32(dispatcher + 0x1e40, 0.75);
        e.mem.set_f32(dispatcher + 0x1e44, 0.125);
        e.set(world, HkpWorld::m_collisionDispatcher, Ptr::new(dispatcher));
        let simulation = e.mem.alloc(0x40);
        e.mem.set_f32(simulation + 0x24, 9.5);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.mem.set_f32(w + 0x308, 16.0);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1001));
        e.set(world, HkpWorld::m_convexListFilter, Ptr::new(0x1002));
        e.set(world, HkpWorld::m_memoryWatchDog, Ptr::new(0x1003));
        e.mem.set_u32(info + 0x58, 0x2001);
        e.mem.set_u32(info + 0x5c, 0);
        e.mem.set_u32(info + 0x6c, 0x2003);
        e.call(0x00c9_1c50, &args![world, info]);
        for (index, (from, to, size)) in INFO_COPIES.iter().enumerate() {
            let value = 0x3f00_0000 + index as u32 + 1;
            if *from == 0xd0 {
                assert_eq!(e.mem.u32(info + to), 0x7fc0_0001);
            } else if *size == 4 {
                assert_eq!(e.mem.u32(info + to), value, "word {from:#x}");
            } else {
                assert_eq!(e.mem.u8(info + to), value as u8, "byte {from:#x}");
            }
        }
        for lane in 0..4 {
            assert_eq!(e.mem.u32(info + 0x10 + 4 * lane), 0xa0 + lane);
            assert_eq!(e.mem.u32(info + 0x30 + 4 * lane), 0xb0 + lane);
            assert_eq!(e.mem.u32(info + 0x40 + 4 * lane), 0xc0 + lane);
        }
        assert_eq!(e.mem.f32(info + 0x54), 1.25);
        assert_eq!(e.mem.u8(info + 0x75), 0x6d);
        assert_eq!(e.mem.u8(info + 0x99), 0x6c);
        assert_eq!(e.mem.f32(info + 0x9c), 3.5);
        assert_eq!(e.mem.u32(info + 0xa0), 0x1357);
        assert_eq!(e.mem.f32(info + 0x60), 0.75);
        assert_eq!(e.mem.f32(info + 0x68), 0.125);
        assert_eq!(e.mem.f32(info + 0xdc), 9.5);
        assert_eq!(e.mem.f32(info + 0x88), 4.0);
        // No border: kind 3 and flag 0; no tree, no callback util.
        assert_eq!(e.mem.u8(info + 0x28), 3);
        assert_eq!(e.mem.u8(info + 0x29), 0);
        assert_eq!(e.mem.u8(info + 0x50), 0);
        assert_eq!(e.mem.u8(info + 0xe0), 0);
        // References: added to each new pointer, removed from each one replaced.
        assert_eq!(
            (
                e.mem.u32(info + 0x58),
                e.mem.u32(info + 0x5c),
                e.mem.u32(info + 0x6c)
            ),
            (0x1001, 0x1002, 0x1003)
        );
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![0x1001], vec![0x1002], vec![0x1003]]
        );
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0x2001], vec![0x2003]]
        );
        // With a border, a tree and the callback util present.
        let border = e.mem.alloc(0x40);
        e.mem.set_u8(border + 0x30, 5);
        e.mem.set_u8(border + 0x34, 1);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(border));
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(0x3000));
        e.set(world, HkpWorld::m_collisionCallbackUtil, Ptr::new(0x3001));
        e.call(0x00c9_1c50, &args![world, info]);
        assert_eq!(e.mem.u8(info + 0x28), 5);
        assert_eq!(e.mem.u8(info + 0x29), 1);
        assert_eq!(e.mem.u8(info + 0x50), 1);
        assert_eq!(e.mem.u8(info + 0xe0), 1);
    }

    /// An engine with a profiling stream that holds `records` records; returns it and the
    /// stream block and the first record address.
    fn profiling_engine(records: u32) -> (Engine, u32, u32) {
        let mut e = logging();
        e.map(0x0126_8000, 0x1000);
        e.set_global(PROFILE_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let area = e.mem.alloc(12 * records);
        e.mem.set_u32(stream + 4, area);
        e.mem.set_u32(stream + 0xc, area + 12 * records);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, stream, area)
    }

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32, start: u32) -> Vec<u32> {
        let cursor = e.mem.u32(stream + 4);
        (start..cursor)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    #[test]
    fn cast_ray_uses_the_tree_when_it_is_in_use() {
        let (mut e, stream, start) = profiling_engine(4);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x18, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_1f70, &args![world, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0x11, 0x1234, 0x22]]);
        assert!(calls_to(&e, WORLD_RAY_CASTER_CAST_RAY).is_empty());
        assert_eq!(
            labels(&e, stream, start),
            vec![PROFILE_CAST_RAY_COLLECTOR, PROFILE_END]
        );
    }

    #[test]
    fn cast_ray_builds_a_caster_without_the_tree() {
        let (mut e, stream, start) = profiling_engine(1);
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(WORLD_RAY_CASTER_CAST_RAY, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x40),
                e.mem.u32(caster + 0x44),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        let world = world(&mut e);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_1f70, &args![world, 0x11u32, 0x22u32]);
        let log = calls_to(&e, WORLD_RAY_CASTER_CAST_RAY);
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][1..], [0x2000, 0x11, 0x1234, 0, 0x22]);
        assert_eq!(words.borrow().clone(), vec![RAY_CASTER_VTABLE, 0, 0]);
        // Only one record fits: the end record is dropped.
        assert_eq!(labels(&e, stream, start), vec![PROFILE_CAST_RAY_COLLECTOR]);
    }

    #[test]
    fn cast_ray_ov2_uses_the_tree_slot_0x1c_or_the_simple_caster() {
        let (mut e, stream, start) = profiling_engine(4);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x1c, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_2040, &args![world, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0x11, 0x1234, 0x22]]);
        // The tree reports it is not in use: the simple caster runs.
        returning(&mut e, FAKE_2, 0);
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(SIMPLE_RAY_CASTER_CAST_RAY, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x30),
                e.mem.u32(caster + 0x34),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        e.call(0x00c9_2040, &args![world, 0x11u32, 0x22u32]);
        let log = calls_to(&e, SIMPLE_RAY_CASTER_CAST_RAY);
        assert_eq!(log[0][1..], [0x2000, 0x11, 0x1234, 0, 0x22]);
        assert_eq!(words.borrow().clone(), vec![SIMPLE_RAY_CASTER_VTABLE, 0, 0]);
        assert_eq!(
            labels(&e, stream, start),
            vec![PROFILE_CAST_RAY, PROFILE_END, PROFILE_CAST_RAY, PROFILE_END]
        );
    }

    #[test]
    fn linear_cast_uses_the_tree_or_a_linear_caster() {
        let (mut e, stream, start) = profiling_engine(8);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(collision_input + 0x70, 0x4444);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(collision_input));
        let filter = e.mem.alloc(0x20);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x20, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![tree, 1, 2, filter + 8, collision_input, 0x4444, 3, 4]]
        );
        // No tree: the linear caster object is built with the default float.
        e.map(0x0104_d000, 0x1000);
        e.set_global(LINEAR_CASTER_DEFAULT, f32::from_bits(0x7f80_0001));
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::NULL);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(LINEAR_CASTER_LINEAR_CAST, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x24),
                e.mem.u32(caster + 0x80),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        let log = calls_to(&e, LINEAR_CASTER_LINEAR_CAST);
        assert_eq!(
            log[0][1..],
            [0x2000, 1, 2, filter + 8, collision_input, 0x4444, 0, 3, 4]
        );
        assert_eq!(
            words.borrow().clone(),
            vec![LINEAR_CASTER_VTABLE, 0, 0x7fc0_0001]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![
                PROFILE_LINEAR_CAST,
                PROFILE_END,
                PROFILE_LINEAR_CAST,
                PROFILE_END
            ]
        );
        // Without a filter the base passed on is 0.
        e.set(world, HkpWorld::m_collisionFilter, Ptr::NULL);
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(calls_to(&e, LINEAR_CASTER_LINEAR_CAST)[1][4], 0);
    }

    /// What a job dispatch test observes: the jobs handed to the queue (their bytes at the
    /// time of the call) and the blocks freed.
    struct Dispatch {
        e: Engine,
        jobs: Rc<RefCell<Vec<Vec<u8>>>>,
        frees: Records,
        world: Ptr<HkpWorld>,
        block: u32,
        notifier: u32,
    }

    fn dispatch_engine(job_size: u32) -> Dispatch {
        let mut e = logging();
        let jobs: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));
        let sink = jobs.clone();
        e.register_double(JOB_QUEUE_ADD_JOB, move |e, words| {
            sink.borrow_mut().push(e.mem.bytes(words[1], job_size));
            Ret::default()
        });
        let block = e.mem.alloc(0x10);
        returning(&mut e, FAKE_1, block);
        let frees = recording(&mut e, FAKE_2);
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (8, FAKE_2)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        silent(
            &mut e,
            &[FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, JOB_HEADER_INIT],
        );
        let notifier = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_3), (0x10, FAKE_4)]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(0x3000));
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x4000));
        Dispatch {
            e,
            jobs,
            frees,
            world,
            block,
            notifier,
        }
    }

    /// Gives the world a kd-tree manager that is in use and holds two words.
    fn use_tree(d: &mut Dispatch) {
        returning(&mut d.e, FAKE_5, 1);
        let tree = object_with_vtable(&mut d.e, 0x20, &[(0x10, FAKE_5)]);
        d.e.mem.set_u32(tree + 0xc, 0xaaaa);
        d.e.mem.set_u32(tree + 0x10, 0xbbbb);
        d.e.set(d.world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
    }

    /// The word at `offset` of a recorded job.
    fn job_word(job: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([
            job[offset],
            job[offset + 1],
            job[offset + 2],
            job[offset + 3],
        ])
    }

    /// The jobs were added, then the notifier and the semaphore waited, then the block freed.
    fn assert_waited(d: &Dispatch, queue: u32, semaphore: u32, job_count: usize) {
        let mut expected = vec![JOB_QUEUE_ADD_JOB; job_count];
        expected.extend([FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, FAKE_2]);
        let wanted = [JOB_QUEUE_ADD_JOB, FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, FAKE_2];
        assert_eq!(order_of(&d.e, &wanted), expected);
        assert_eq!(calls_to(&d.e, FAKE_3), vec![vec![d.notifier, queue, 0xb]]);
        assert_eq!(calls_to(&d.e, FAKE_4), vec![vec![d.notifier]]);
        assert_eq!(calls_to(&d.e, SEMAPHORE_ACQUIRE), vec![vec![semaphore]]);
        let frees = d.frees.borrow().clone();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [d.block, 0x10]);
        for add in calls_to(&d.e, JOB_QUEUE_ADD_JOB) {
            assert_eq!(add[0], queue);
            assert_eq!(add[2], 1);
        }
    }

    #[test]
    fn fn_00c92110_splits_the_array_into_batched_jobs() {
        let mut d = dispatch_engine(0x50);
        let (queue, semaphore, array) = (0x5000, 0x6000, 0x7000);
        d.e.call(
            0x00c9_2110,
            &args![d.world, array, 130i32, queue, d.notifier, semaphore, 64i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(d.e.mem.u32(d.block), 3);
        assert_eq!(jobs[0][..3], [1, 3, 1]);
        assert_eq!(u16::from_le_bytes([jobs[0][4], jobs[0][5]]), 0x50);
        assert_eq!(u16::from_le_bytes([jobs[0][6], jobs[0][7]]), 0xffff);
        assert_eq!(job_word(&jobs[0], 0x10), semaphore);
        assert_eq!(job_word(&jobs[0], 0x14), d.block);
        assert_eq!(job_word(&jobs[0], 0x1c), 0x3000);
        assert_eq!(job_word(&jobs[0], 0x20), 0x40);
        assert_eq!(job_word(&jobs[0], 0x34), 0x4000);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x28)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x24)).collect();
        assert_eq!(counts, vec![64, 64, 2]);
        assert_eq!(arrays, vec![array, array + 0x1000, array + 0x2000]);
        assert_waited(&d, queue, semaphore, 3);
    }

    #[test]
    fn fn_00c92110_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x50);
        use_tree(&mut d);
        d.e.call(
            0x00c9_2110,
            &args![d.world, 0x7000u32, 10i32, 0x5000u32, d.notifier, 0x6000u32, 64i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 1);
        assert_eq!(job_word(&jobs[0], 0x34), 0);
        assert_eq!(job_word(&jobs[0], 0x3c), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x40), 0xbbbb);
        assert_eq!(job_word(&jobs[0], 0x28), 10);
    }

    #[test]
    fn fn_00c92110_with_no_elements_adds_no_job() {
        let mut d = dispatch_engine(0x50);
        d.e.call(
            0x00c9_2110,
            &args![d.world, 0x7000u32, 0i32, 0x5000u32, d.notifier, 0x6000u32, 64i32],
        );
        assert!(d.jobs.borrow().is_empty());
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_waited(&d, 0x5000, 0x6000, 0);
    }

    #[test]
    fn fn_00c92290_clears_the_elements_and_batches_the_jobs() {
        let mut d = dispatch_engine(0x40);
        let collision_input = d.e.mem.alloc(0x20);
        d.e.mem.set_u32(collision_input + 8, 0x7f80_0001);
        d.e.set(
            d.world,
            HkpWorld::m_collisionInput,
            Ptr::new(collision_input),
        );
        let array = d.e.mem.alloc(0x10 * 5);
        for index in 0..5 {
            d.e.mem.set_u32(array + 0xc + 0x10 * index, 0xffff);
        }
        d.e.call(
            0x00c9_2290,
            &args![d.world, array, 5i32, 0x5000u32, d.notifier, 0x6000u32, 2i32],
        );
        for index in 0..5 {
            assert_eq!(d.e.mem.u32(array + 0xc + 0x10 * index), 0);
        }
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0][..3], [2, 2, 2]);
        assert_eq!(job_word(&jobs[0], 0x2c), 0x7fc0_0001);
        assert_eq!(job_word(&jobs[0], 0x30), 0x40);
        assert_eq!(job_word(&jobs[0], 0x20), 0x4000);
        assert_eq!(job_word(&jobs[0], 0x18), collision_input);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x38)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x34)).collect();
        assert_eq!(counts, vec![2, 2, 1]);
        assert_eq!(arrays, vec![array, array + 0x20, array + 0x40]);
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_eq!(calls_to(&d.e, JOB_HEADER_INIT).len(), 1);
        assert_waited(&d, 0x5000, 0x6000, 3);
    }

    #[test]
    fn fn_00c92290_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x40);
        let collision_input = d.e.mem.alloc(0x20);
        d.e.set(
            d.world,
            HkpWorld::m_collisionInput,
            Ptr::new(collision_input),
        );
        use_tree(&mut d);
        let array = d.e.mem.alloc(0x40);
        d.e.call(
            0x00c9_2290,
            &args![d.world, array, 1i32, 0x5000u32, d.notifier, 0x6000u32, 8i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 1);
        assert_eq!(job_word(&jobs[0], 0x20), 0);
        assert_eq!(job_word(&jobs[0], 0x24), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x28), 0xbbbb);
    }

    #[test]
    fn fn_00c92540_batches_with_a_stride_of_0x30() {
        let mut d = dispatch_engine(0x40);
        let array = 0x7000u32;
        d.e.call(
            0x00c9_2540,
            &args![d.world, array, 5i32, 0x5000u32, d.notifier, 0x6000u32, 2i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0][..3], [4, 2, 1]);
        assert_eq!(job_word(&jobs[0], 0x18), 0x3000);
        assert_eq!(job_word(&jobs[0], 0x20), 0x40);
        assert_eq!(job_word(&jobs[0], 0x2c), 0x4000);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x28)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x24)).collect();
        assert_eq!(counts, vec![2, 2, 1]);
        assert_eq!(arrays, vec![array, array + 0x60, array + 0xc0]);
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_waited(&d, 0x5000, 0x6000, 3);
    }

    #[test]
    fn fn_00c92540_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x40);
        use_tree(&mut d);
        d.e.call(
            0x00c9_2540,
            &args![d.world, 0x7000u32, 1i32, 0x5000u32, d.notifier, 0x6000u32, 8i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(job_word(&jobs[0], 0x2c), 0);
        assert_eq!(job_word(&jobs[0], 0x34), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x38), 0xbbbb);
    }

    /// An island whose entity array holds `bodies`; its action array holds `actions`.
    fn make_island(e: &mut Engine, bodies: &[u32], actions: &[u32]) -> u32 {
        let island = e.mem.alloc(0x80);
        let body_array = array_of(e, bodies);
        e.mem.set_u32(island + 0x48, body_array);
        e.mem.set_i32(island + 0x4c, bodies.len() as i32);
        let action_array = array_of(e, actions);
        e.mem.set_u32(island + 0x38, action_array);
        e.mem.set_i32(island + 0x3c, actions.len() as i32);
        island
    }

    fn set_islands(e: &mut Engine, world: Ptr<HkpWorld>, active: &[u32], inactive: &[u32]) {
        let active_array = array_of(e, active);
        e.set(
            world,
            HkpWorld::m_activeSimulationIslands,
            Ptr::new(active_array),
        );
        e.set(
            world,
            HkpWorld::m_activeSimulationIslandsSize,
            active.len() as i32,
        );
        let inactive_array = array_of(e, inactive);
        e.set(
            world,
            HkpWorld::m_inactiveSimulationIslands,
            Ptr::new(inactive_array),
        );
        e.set(
            world,
            HkpWorld::m_inactiveSimulationIslandsSize,
            inactive.len() as i32,
        );
    }

    #[test]
    fn fn_00c926a0_adds_the_fixed_and_inactive_bodies() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let first = e.mem.alloc(0x20);
        let (second, third) = (0x2222, 0x3333);
        let fixed = make_island(&mut e, &[first, second], &[]);
        let inactive = make_island(&mut e, &[third], &[]);
        e.set(world, HkpWorld::m_fixedIsland, Ptr::new(fixed));
        set_islands(&mut e, world, &[], &[inactive]);
        let out = e.mem.alloc(8);
        let returned = e.call(0x00c9_26a0, &args![out, world, 0x9000u32]).u32();
        assert_eq!(returned, out);
        assert_eq!(e.mem.u8(out), 1);
        // The first fixed body has no word at +0x10: it is skipped.
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![0x9000, second], vec![0x9000, third]]
        );
        // With a word there it is added too; without any island the flag is clear.
        e.mem.set_u32(first + 0x10, 1);
        e.call(0x00c9_26a0, &args![out, world, 0x9000u32]);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY).len(), 5);
        e.set(world, HkpWorld::m_fixedIsland, Ptr::NULL);
        set_islands(&mut e, world, &[], &[]);
        e.call(0x00c9_26a0, &args![out, world, 0x9000u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY).len(), 5);
    }

    #[test]
    fn fn_00c92760_adds_the_active_bodies() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let one = make_island(&mut e, &[0x11, 0x22], &[]);
        let two = make_island(&mut e, &[0x33], &[]);
        set_islands(&mut e, world, &[one, two], &[]);
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00c9_2760, &args![out, world, 0x9000u32]).u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![0x9000, 0x11], vec![0x9000, 0x22], vec![0x9000, 0x33]]
        );
        set_islands(&mut e, world, &[], &[]);
        e.call(0x00c9_2760, &args![out, world, 0x9000u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00c927c0_stores_the_active_flag_in_the_system() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let active = make_island(&mut e, &[0x11], &[]);
        set_islands(&mut e, world, &[active], &[]);
        let system = Ptr::new(e.mem.alloc(0x50));
        fn_00c927c0(&mut e, system, world);
        assert_eq!(e.mem.u8(system.addr() + 0x40), 1);
        set_islands(&mut e, world, &[], &[]);
        fn_00c927c0(&mut e, system, world);
        assert_eq!(e.mem.u8(system.addr() + 0x40), 0);
    }

    /// Writes a constraint entry at `entry` whose first word is `first` and whose chain holds
    /// nodes of the given kinds, linked through the word at +0x14.
    fn constraint_entry(e: &mut Engine, entry: u32, first: u32, kinds: &[u16]) {
        e.mem.set_u32(entry, first);
        let mut link = 0;
        for &kind in kinds.iter().rev() {
            let node = e.mem.alloc(0x20);
            e.mem.set_u16(node, kind);
            e.mem.set_u32(node + 0x14, link);
            link = node;
        }
        e.mem.set_u32(entry + 0xc, link);
    }

    #[test]
    fn fn_00c927f0_adds_the_constraints_that_do_not_end_in_0x16() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_CONSTRAINT]);
        let entity = e.mem.alloc(0xc0);
        let entries = e.mem.alloc(0x30 * 4);
        constraint_entry(&mut e, entries, 0xa1, &[0x10]);
        constraint_entry(&mut e, entries + 0x30, 0xa2, &[0x16]);
        constraint_entry(&mut e, entries + 0x60, 0xa3, &[0x20, 0x16]);
        constraint_entry(&mut e, entries + 0x90, 0xa4, &[0x20, 0x21, 0x15]);
        e.mem.set_u32(entity + 0xac, entries);
        e.mem.set_u16(entity + 0xb0, 4);
        let island = make_island(&mut e, &[entity], &[]);
        e.call(0x00c9_27f0, &args![island, 0x9000u32]);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT),
            vec![vec![0x9000, 0xa1], vec![0x9000, 0xa4]]
        );
        // An entity without constraints adds none.
        e.mem.set_u16(entity + 0xb0, 0);
        e.call(0x00c9_27f0, &args![island, 0x9000u32]);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT).len(), 2);
    }

    /// An island holding one entity with one constraint entry whose first word is `first`.
    fn island_with_constraint(e: &mut Engine, first: u32) -> u32 {
        let entity = e.mem.alloc(0xc0);
        let entry = e.mem.alloc(0x30);
        constraint_entry(e, entry, first, &[0x10]);
        e.mem.set_u32(entity + 0xac, entry);
        e.mem.set_u16(entity + 0xb0, 1);
        make_island(e, &[entity], &[])
    }

    #[test]
    fn fn_00c92860_visits_active_then_inactive_islands() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_CONSTRAINT]);
        let world = world(&mut e);
        let active = island_with_constraint(&mut e, 0xb1);
        let inactive = island_with_constraint(&mut e, 0xb2);
        set_islands(&mut e, world, &[active], &[inactive]);
        fn_00c92860(&mut e, world, Ptr::new(0x9000));
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT),
            vec![vec![0x9000, 0xb1], vec![0x9000, 0xb2]]
        );
    }

    #[test]
    fn fn_00c928c0_adds_the_actions_of_every_island() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_ACTION]);
        let world = world(&mut e);
        let active = make_island(&mut e, &[], &[0xc1, 0xc2]);
        let inactive = make_island(&mut e, &[], &[0xc3]);
        set_islands(&mut e, world, &[active], &[inactive]);
        e.call(0x00c9_28c0, &args![world, 0x9000u32]);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_ACTION),
            vec![vec![0x9000, 0xc1], vec![0x9000, 0xc2], vec![0x9000, 0xc3]]
        );
    }

    #[test]
    fn fn_00c92950_skips_the_borders_phantoms() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_PHANTOM]);
        let world = world(&mut e);
        let phantoms = array_of(&mut e, &[0x11, 0x22, 0x33]);
        e.set(world, HkpWorld::m_phantoms, Ptr::new(phantoms));
        e.set(world, HkpWorld::m_phantomsSize, 3);
        fn_00c92950(&mut e, world, Ptr::new(0x9000));
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM).len(), 3);
        let border = e.mem.alloc(0x40);
        e.mem.set_u32(border + 0x18, 0x22);
        e.mem.set_u32(border + 0x2c, 0x33);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(border));
        fn_00c92950(&mut e, world, Ptr::new(0x9000));
        let log = calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM);
        assert_eq!(log.len(), 4);
        assert_eq!(log[3], vec![0x9000, 0x11]);
    }

    #[test]
    fn fn_00c929e0_builds_a_system_from_the_world() {
        let mut e = logging();
        let allocated = e.mem.alloc(0x44);
        returning(&mut e, FAKE_1, allocated);
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        let system = e.mem.alloc(0x50);
        returning(&mut e, PHYSICS_SYSTEM_CONSTRUCT, system);
        silent(
            &mut e,
            &[
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_CONSTRAINT,
                PHYSICS_SYSTEM_ADD_ACTION,
                PHYSICS_SYSTEM_ADD_PHANTOM,
            ],
        );
        let world = world(&mut e);
        let body_a = e.mem.alloc(0xc0);
        let body_b = e.mem.alloc(0xc0);
        let active = make_island(&mut e, &[body_a], &[0x21]);
        let inactive = make_island(&mut e, &[body_b], &[0x22]);
        set_islands(&mut e, world, &[active], &[inactive]);
        let phantoms = array_of(&mut e, &[0x31]);
        e.set(world, HkpWorld::m_phantoms, Ptr::new(phantoms));
        e.set(world, HkpWorld::m_phantomsSize, 1);
        let returned = e.call(0x00c9_29e0, &args![world]).u32();
        assert_eq!(returned, system);
        assert_eq!(e.mem.u16(allocated + 4), 0x44);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_CONSTRUCT),
            vec![vec![allocated]]
        );
        assert_eq!(e.mem.u8(system + 0x40), 1);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![system, body_b], vec![system, body_a]]
        );
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM),
            vec![vec![system, 0x31]]
        );
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_ACTION),
            vec![vec![system, 0x21], vec![system, 0x22]]
        );
        let sequence = [
            PHYSICS_SYSTEM_CONSTRUCT,
            PHYSICS_SYSTEM_ADD_RIGID_BODY,
            PHYSICS_SYSTEM_ADD_PHANTOM,
            PHYSICS_SYSTEM_ADD_ACTION,
        ];
        assert_eq!(
            order_of(&e, &sequence),
            vec![
                PHYSICS_SYSTEM_CONSTRUCT,
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_PHANTOM,
                PHYSICS_SYSTEM_ADD_ACTION,
                PHYSICS_SYSTEM_ADD_ACTION,
            ]
        );
    }

    // -----------------------------------------------------------------------------------
    // Phantoms, batches, actions, constraints and collision filter updates.
    // -----------------------------------------------------------------------------------

    type CallLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// The memory router with a bump area and two allocators that log what is handed back,
    /// the array growth callees doubled to really grow, a profiling stream and a world.
    struct Rig {
        e: Engine,
        world: Ptr<HkpWorld>,
        stream: u32,
        area: u32,
        router: u32,
        freed_first: Records,
        freed_second: Records,
    }

    fn rig() -> Rig {
        let (mut e, stream, area) = profiling_engine(40);
        e.map(0x0101_1000, 0x1000);
        e.set_global(HALF, 0.5f64);
        let freed_first = recording(&mut e, FAKE_4);
        let freed_second = recording(&mut e, FAKE_5);
        e.register(FAKE_1, |e, words| Ret {
            eax: e.mem.alloc(words[1].max(8)),
            ..Ret::default()
        });
        let first = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (0x10, FAKE_4)]);
        let second = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (0x10, FAKE_5)]);
        let cursor = e.mem.alloc(0x4000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, cursor);
        e.mem.set_u32(router + 0xc, second);
        e.mem.set_u32(router + 0x10, first);
        returning(&mut e, MEMORY_ROUTER, router);
        e.register(ARRAY_GROW, |e, words| {
            let (array, element) = (words[1], words[2]);
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff).max(2) * 2;
            let block = e.mem.alloc(capacity * element);
            let count = e.mem.u32(array + 4) * element;
            if count != 0 {
                let old = e.mem.bytes(e.mem.u32(array), count);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, words| {
            let (array, count, element) = (words[1], words[2], words[3]);
            let block = e.mem.alloc((count * element).max(8));
            let used = e.mem.u32(array + 4) * element;
            if used != 0 {
                let old = e.mem.bytes(e.mem.u32(array), used);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
        silent(&mut e, &[QUEUE_FLUSH_PENDING, QUEUE_FLUSH_BODY_PENDING]);
        let world = world(&mut e);
        Rig {
            e,
            world,
            stream,
            area,
            router,
            freed_first,
            freed_second,
        }
    }

    /// The labels of the profiling records written so far; the records of the labels in
    /// `wide` are 16 bytes long, the others 12.
    fn record_labels(rig: &Rig, wide: &[u32]) -> Vec<u32> {
        let cursor = rig.e.mem.u32(rig.stream + 4);
        let mut labels = vec![];
        let mut at = rig.area;
        while at < cursor {
            let label = rig.e.mem.u32(at);
            labels.push(label);
            at += if wide.contains(&label) { 16 } else { 12 };
        }
        labels
    }

    fn lock(rig: &mut Rig, count: i32) {
        rig.e
            .set(rig.world, HkpWorld::m_criticalOperationsLockCount, count);
    }

    /// An object whose vtable slots 0x1c / 0x20 / 0x24 are the overlap callbacks of a
    /// phantom: slot 0x20 reports `was` in the byte it is given, the others log
    /// `(slot, this)`.
    fn overlap_object(e: &mut Engine, log: &CallLog, was: u8, base: u32) -> u32 {
        e.register_double(base, move |e, words| {
            e.mem.set_u8(words[1], was);
            Ret::default()
        });
        for (slot, offset) in [(0x1cu32, 1u32), (0x24, 2)] {
            let sink = log.clone();
            e.register_double(base + offset, move |_, words| {
                sink.borrow_mut().push((slot, words[0]));
                Ret::default()
            });
        }
        object_with_vtable(
            e,
            0x240,
            &[(0x20, base), (0x1c, base + 1), (0x24, base + 2)],
        )
    }

    /// A filter object whose pair test answers `accepts`.
    fn filter_answering(e: &mut Engine, accepts: u8) -> u32 {
        let answer = e.mem.alloc(4);
        e.mem.set_u8(answer, accepts);
        e.register_double(FAKE_3, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, FAKE_3);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(filter + 8, table);
        filter
    }

    /// Runs the outlined filter piece; the log is `(slot, 1 for the phantom or 2 for the
    /// other body)`.
    fn run_overlap_piece(was: u8, accepts: u8, other_was: Option<u8>) -> Vec<(u32, u32)> {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = overlap_object(&mut e, &log, was, 0x0900_0010);
        let other = overlap_object(&mut e, &log, other_was.unwrap_or(0), 0x0900_0020);
        e.mem
            .set_u8(other + 0x18, if other_was.is_some() { 2 } else { 1 });
        let filter = filter_answering(&mut e, accepts);
        fn_00c90f40(&mut e, phantom, other, filter);
        let entries = log.borrow().clone();
        entries
            .into_iter()
            .map(|(slot, this)| (slot, if this == phantom { 1 } else { 2 }))
            .collect()
    }

    #[test]
    fn fn_00c90f40_tells_the_phantom_when_the_filter_changes_its_mind() {
        assert_eq!(run_overlap_piece(0, 1, None), vec![(0x1c, 1)]);
        assert_eq!(run_overlap_piece(1, 1, None), vec![]);
        assert_eq!(run_overlap_piece(1, 0, None), vec![(0x24, 1)]);
        assert_eq!(run_overlap_piece(0, 0, None), vec![]);
    }

    #[test]
    fn fn_00c90f40_tells_a_phantom_on_the_other_side_too() {
        assert_eq!(run_overlap_piece(0, 1, Some(0)), vec![(0x1c, 1), (0x1c, 2)]);
        assert_eq!(run_overlap_piece(1, 1, Some(1)), vec![]);
        assert_eq!(run_overlap_piece(0, 0, Some(1)), vec![(0x24, 2)]);
        assert_eq!(run_overlap_piece(1, 0, Some(0)), vec![(0x24, 1)]);
    }

    /// A phantom object (size 0x40) with a deletion callback (slot 0x34) and the motion
    /// maker (slot 0x10) that log themselves.
    fn phantom_object(e: &mut Engine, log: &CallLog) -> u32 {
        let sink = log.clone();
        e.register_double(0x0900_0030, move |_, words| {
            sink.borrow_mut().push((0x34, words[0]));
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(0x0900_0031, move |_, words| {
            sink.borrow_mut().push((0x10, words[0]));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(0x0900_0032, move |e, words| {
            e.mem.set_u32(words[1], 0xb0b0_0000 + words[0]);
            sink.borrow_mut().push((0x18, words[0]));
            Ret::default()
        });
        object_with_vtable(
            e,
            0x40,
            &[
                (0x34, 0x0900_0030),
                (0x10, 0x0900_0031),
                (0x18, 0x0900_0032),
            ],
        )
    }

    fn set_phantoms(rig: &mut Rig, phantoms: &[u32]) {
        let data = array_of(&mut rig.e, phantoms);
        rig.e.set(rig.world, HkpWorld::m_phantoms, Ptr::new(data));
        rig.e
            .set(rig.world, HkpWorld::m_phantomsSize, phantoms.len() as i32);
        rig.e
            .mem
            .set_u32(rig.world.addr() + 0x118, phantoms.len() as u32);
    }

    fn phantoms_of(rig: &Rig) -> Vec<u32> {
        let data = rig.e.mem.u32(rig.world.addr() + 0x110);
        (0..rig.e.mem.u32(rig.world.addr() + 0x114))
            .map(|i| rig.e.mem.u32(data + 4 * i))
            .collect()
    }

    const PHANTOM_CALLEES: [u32; 7] = [
        FIRE_PHANTOM_ADDED,
        FIRE_PHANTOM_REMOVED,
        ADD_PHANTOM_BP,
        REMOVE_PHANTOM_BP,
        PHANTOM_AFTER_ADD,
        PHANTOM_BEFORE_REMOVE,
        ADD_REFERENCE,
    ];

    #[test]
    fn remove_phantom_runs_the_whole_sequence() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let phantom = phantom_object(&mut rig.e, &log);
        set_phantoms(&mut rig, &[phantom, 0x21, 0x22]);
        rig.e.mem.set_u32(phantom + 8, 0x5555);
        rig.e.set(rig.world, HkpWorld::m_pendingOperationsCount, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        let world = rig.world.addr();
        assert_eq!(phantoms_of(&rig), vec![0x22, 0x21]);
        assert_eq!(rig.e.mem.u32(phantom + 8), 0);
        assert_eq!(log.borrow().clone(), vec![(0x34, phantom)]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    FIRE_PHANTOM_REMOVED,
                    PHANTOM_BEFORE_REMOVE,
                    REMOVE_PHANTOM_BP,
                    REMOVE_REFERENCE,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                FIRE_PHANTOM_REMOVED,
                PHANTOM_BEFORE_REMOVE,
                REMOVE_PHANTOM_BP,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_REMOVED),
            vec![vec![world, phantom]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_phantom_keeps_a_phantom_that_still_has_a_size_and_missing_ones_index_minus_one() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let phantom = phantom_object(&mut rig.e, &log);
        rig.e.mem.set_u16(phantom + 4, 0x40);
        set_phantoms(&mut rig, &[0x21, phantom]);
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        assert_eq!(phantoms_of(&rig), vec![0x21]);
        assert!(log.borrow().is_empty());
        // Not in the list: the last entry is copied over the word before the data.
        let data = rig.e.mem.u32(rig.world.addr() + 0x110);
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x114), 0);
        assert_eq!(rig.e.mem.u32(data - 4), 0x21);
    }

    #[test]
    fn phantom_requests_are_queued_while_either_lock_is_held() {
        for (critical, phantom_lock) in [(1, 0), (0, 1)] {
            let mut rig = rig();
            let queued = operation_recorder(&mut rig.e);
            rig.e
                .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
            lock(&mut rig, critical);
            rig.e.set(
                rig.world,
                HkpWorld::m_phantomOperationsLockCount,
                phantom_lock,
            );
            rig.e.call(0x00c9_2c70, &args![rig.world, 0x1234u32]);
            let added = rig.e.call(0x00c9_36a0, &args![rig.world, 0x1235u32]).u32();
            assert_eq!(added, 0);
            let batch = op_bytes(0xf, 0x1236, 3, 0);
            rig.e.call(0x00c9_54e0, &args![rig.world, 0x1236u32, 3i32]);
            rig.e.call(0x00c9_5800, &args![rig.world, 0x1237u32, 4i32]);
            assert_eq!(
                queued.borrow().clone(),
                vec![
                    (0x5000, op_bytes(0xe, 0x1234, 0, 0)),
                    (0x5000, op_bytes(0xd, 0x1235, 0, 0)),
                    (0x5000, batch),
                    (0x5000, op_bytes(0x10, 0x1237, 4, 0)),
                ]
            );
        }
    }

    #[test]
    fn add_phantom_runs_the_whole_sequence() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        let phantom = phantom_object(&mut rig.e, &log);
        set_phantoms(&mut rig, &[0x21]);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.set(rig.world, HkpWorld::m_pendingOperationsCount, 2);
        let world = rig.world.addr();
        let returned = rig.e.call(0x00c9_36a0, &args![rig.world, phantom]).u32();
        assert_eq!(returned, phantom);
        assert_eq!(rig.e.mem.u32(phantom + 0x18), 0x7777);
        assert_eq!(rig.e.mem.u32(phantom + 8), world);
        assert_eq!(phantoms_of(&rig), vec![0x21, phantom]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    ADD_PHANTOM_BP,
                    FIRE_PHANTOM_ADDED,
                    PHANTOM_AFTER_ADD,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                ADD_REFERENCE,
                ADD_PHANTOM_BP,
                FIRE_PHANTOM_ADDED,
                PHANTOM_AFTER_ADD,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(calls_to(&rig.e, ADD_PHANTOM_BP), vec![vec![world, phantom]]);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
        // A phantom that already has its object keeps it.
        let second = phantom_object(&mut rig.e, &log);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        rig.e.call(0x00c9_36a0, &args![rig.world, second]);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
    }

    /// A broad phase whose slot `slot` stores the `(unused, collidable)` pairs into the array
    /// it is handed (its last argument); returns the object.
    fn broad_phase_finding(e: &mut Engine, slot: u32, collidables: &[u32]) -> u32 {
        let found = collidables.to_vec();
        e.register_double(0x0900_0040 + slot, move |e, words| {
            let array = *words.last().unwrap();
            let data = e.mem.alloc(8 * found.len().max(1) as u32);
            for (i, collidable) in found.iter().enumerate() {
                e.mem.set_u32(data + 8 * i as u32 + 4, *collidable);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 4, found.len() as u32);
            Ret::default()
        });
        object_with_vtable(e, 0x20, &[(slot, 0x0900_0040 + slot)])
    }

    /// A collidable (size 0x40) at +0x10 of an owner: the collidable's byte +5 is the offset
    /// back to the owner; returns `(collidable, owner)`.
    fn collidable_in(e: &mut Engine, kind: u8, to_body: i8) -> (u32, u32) {
        let owner = e.mem.alloc(0x240);
        let collidable = owner + 0x10;
        e.mem.set_i8(collidable + 5, -0x10);
        e.mem.set_u8(owner + 0x18, kind);
        e.mem.set_i8(owner + 0x10, to_body);
        (collidable, owner)
    }

    #[test]
    fn activate_region_activates_the_rigid_bodies_found() {
        let mut rig = rig();
        let activated = recording(&mut rig.e, ACTIVATE_BODY);
        let (rigid, rigid_owner) = collidable_in(&mut rig.e, 1, 0);
        let (phantom, _) = collidable_in(&mut rig.e, 2, 0);
        let (shifted, shifted_owner) = collidable_in(&mut rig.e, 1, 0x20);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x44, &[rigid, phantom, shifted]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00c9_3c30, &args![rig.world, aabb]);
        assert_eq!(
            activated.borrow().clone(),
            vec![vec![rigid_owner], vec![shifted_owner + 0x20]]
        );
        let query = calls_to(&rig.e, 0x0900_0084);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][..2], [broad_phase, aabb]);
    }

    #[test]
    fn activate_region_copies_the_aabb_when_queued() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e.register(MEM_COPY, |e, words| {
            let bytes = e.mem.bytes(words[1], words[2]);
            e.mem.write(words[0], &bytes);
            Ret::default()
        });
        lock(&mut rig, 1);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(aabb + 0x1c, 0xfeed);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_3c30, &args![rig.world, aabb]);
        let queued = queued.borrow().clone();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].1[0], 0x20);
        let copy = u32::from_le_bytes(queued[0].1[4..8].try_into().unwrap());
        assert_ne!(copy, aabb);
        assert_eq!(rig.e.mem.u32(copy + 0x1c), 0xfeed);
        assert_eq!(calls_to(&rig.e, MEM_COPY), vec![vec![copy, aabb, 0x20]]);
    }

    /// A phantom the world updates its filter on: slot 0x30 logs, slots 0x1c / 0x20 / 0x24
    /// are the overlap callbacks.
    fn filtered_phantom(e: &mut Engine, log: &CallLog) -> u32 {
        let phantom = overlap_object(e, log, 0, 0x0900_0010);
        let sink = log.clone();
        e.register_double(0x0900_0013, move |_, words| {
            sink.borrow_mut().push((0x30, words[0]));
            Ret::default()
        });
        let table = e.mem.u32(phantom);
        e.mem.set_u32(table + 0x30, 0x0900_0013);
        phantom
    }

    #[test]
    fn update_filter_on_phantom_asks_every_other_overlapping_body() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = filtered_phantom(&mut rig.e, &log);
        let own = phantom + 0x24;
        let (other, _) = collidable_in(&mut rig.e, 1, 0);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[own, other]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e.mem.set_u32(rig.world.addr() + 0x338, 8);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 1u32]);
        // The phantom itself is skipped; the other body made it overlap, and the flag
        // calls slot 0x30.
        assert_eq!(log.borrow().clone(), vec![(0x1c, phantom), (0x30, phantom)]);
        let query = calls_to(&rig.e, 0x0900_0088);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][..2], [broad_phase, own]);
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_PHANTOM]),
            vec![
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_COLLECTION_FILTER,
                PROFILE_SECTION_END
            ]
        );
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
    }

    #[test]
    fn update_filter_on_phantom_without_flag_skips_slot_0x30_and_queues_when_locked() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = filtered_phantom(&mut rig.e, &log);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_PHANTOM]),
            vec![
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_SECTION_END
            ]
        );
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 1u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x14, phantom, 1, 0))]
        );
    }

    /// An entity (size 0x240) of the given type and collision family.
    fn filtered_entity(e: &mut Engine, kind: u8, family: i8) -> u32 {
        let entity = e.mem.alloc(0x240);
        e.mem.set_u8(entity + 0xe8, kind);
        e.mem.set_i8(entity + 0x2a, family);
        entity
    }

    const ENTITY_FILTER_CALLEES: [u32; 5] = [
        COLLIDABLE_RESET_AGENTS,
        REMOVE_DUPLICATE_PAIRS,
        UPDATE_SHAPE_COLLECTION_FILTER,
        ADD_PAIRS,
        QUEUE_FLUSH_PENDING,
    ];

    /// The `hkArray` of agent entries `(entry, partner collidable)` that `00d081e0` answers
    /// with; returns its address. `00cc1160` takes the entry it is given out of it, like the
    /// real one.
    fn agent_entries(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let data = e.mem.alloc(8 * pairs.len().max(1) as u32);
        for (i, (entry, partner)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *entry);
            e.mem.set_u32(data + 8 * i as u32 + 4, *partner);
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, pairs.len() as u32);
        returning(e, COLLIDABLE_AGENT_ENTRIES, array);
        e.register_double(REMOVE_AGENT, move |e, words| {
            let data = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let at = (0..size).find(|i| e.mem.u32(data + 8 * i) == words[0]);
            if let Some(at) = at {
                for i in at..size - 1 {
                    let next = e.mem.bytes(data + 8 * (i + 1), 8);
                    e.mem.write(data + 8 * i, &next);
                }
                e.mem.set_u32(array + 4, size - 1);
            }
            Ret::default()
        });
        array
    }

    /// A collision table of the world (a byte per group pair at +0x1bb0) with the given
    /// pairs enabled.
    fn group_table(rig: &mut Rig, enabled: &[usize]) {
        let table = rig.e.mem.alloc(0x2000);
        for index in enabled {
            rig.e.mem.set_u8(table + 0x1bb0 + *index as u32, 1);
        }
        rig.e
            .set(rig.world, HkpWorld::m_collisionDispatcher, Ptr::new(table));
    }

    #[test]
    fn update_filter_on_entity_removes_refused_agents_and_updates_the_others() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 1, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        rig.e.mem.set_u8(island + 0x25, 0x06);
        // Partner groups 1 and 3: the table allows (family 2, group 3) only.
        let partner_a = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner_a + 0x1a, 1);
        let partner_b = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner_b + 0x1a, 3);
        agent_entries(&mut rig.e, &[(0x601, partner_a), (0x602, partner_b)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 0u32]);
        assert_eq!(calls_to(&rig.e, REMOVE_AGENT), vec![vec![0x601]]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![0x602, 0x7000, island]]
        );
        assert_eq!(rig.e.mem.u8(island + 0x25), 0x05);
        assert!(calls_to(&rig.e, ADD_PAIRS).is_empty());
        assert!(calls_to(&rig.e, COLLIDABLE_RESET_AGENTS).is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_ENTITY]),
            vec![
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_CHECK_AGENTS,
                PROFILE_SECTION_END
            ]
        );
    }

    #[test]
    fn update_filter_on_entity_refuses_by_the_filter_and_resets_type_5() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 5, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        let partner = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner + 0x1a, 3);
        agent_entries(&mut rig.e, &[(0x601, partner)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 0);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 0u32]);
        assert_eq!(calls_to(&rig.e, REMOVE_AGENT), vec![vec![0x601]]);
        assert_eq!(
            calls_to(&rig.e, COLLIDABLE_RESET_AGENTS),
            vec![vec![entity + 0x10]]
        );
        assert!(calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER).is_empty());
    }

    #[test]
    fn update_filter_on_entity_updates_a_type_5_entity_in_its_partners_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 5, 2);
        let partner_island = rig.e.mem.alloc(0x40);
        let partner_owner = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(partner_owner + 0xcc, partner_island);
        // The partner collidable is at +0x10 of its owner: byte +0x10 of the partner points
        // back to the owner (offset -0x10).
        let partner = partner_owner + 0x10;
        rig.e.mem.set_i8(partner + 0x1a, 3);
        rig.e.mem.set_i8(partner + 0x10, -0x10);
        agent_entries(&mut rig.e, &[(0x601, partner)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 0u32]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![0x601, 0x7000, partner_island]]
        );
    }

    #[test]
    fn update_filter_on_entity_mode_0_collects_the_overlapping_bodies_first() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                COLLIDABLE_RESET_AGENTS,
                UPDATE_SHAPE_COLLECTION_FILTER,
                ADD_PAIRS,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let entity = filtered_entity(&mut rig.e, 1, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        // A phantom that the broad phase reports through its collidable (+0x10), and a
        // rigid body.
        let phantom = filtered_phantom(&mut rig.e, &log);
        rig.e.mem.set_u8(phantom + 0x18, 2);
        rig.e.mem.set_i8(phantom + 0x15, -0x10);
        let (rigid, _) = collidable_in(&mut rig.e, 1, 0);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[phantom + 0x10, rigid]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e.set(
            rig.world,
            HkpWorld::m_broadPhaseDispatcher,
            Ptr::new(0x6000),
        );
        // One agent entry whose two collidables' handles go to the second list.
        let first_body = rig.e.mem.alloc(0x40);
        let second_body = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(second_body + 0x1a, 3);
        let entry = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entry + 0x10, first_body);
        rig.e.mem.set_u32(entry + 0x14, second_body);
        agent_entries(&mut rig.e, &[(entry, second_body)]);
        group_table(&mut rig, &[23]);
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        rig.e
            .register_double(REMOVE_DUPLICATE_PAIRS, move |e, words| {
                let second = words[0];
                let data = e.mem.u32(second);
                sink.borrow_mut().push((
                    e.mem.u32(second + 4),
                    e.mem.u32(data),
                    e.mem.u32(data + 4),
                    e.mem.u32(words[1] + 4),
                ));
                Ret::default()
            });
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 0i32, 1u32]);
        // The overlapping phantom was asked about the entity's collidable and told to
        // update (flag set); the rigid body stays in the list and gets pairs added.
        assert_eq!(log.borrow().clone(), vec![(0x1c, phantom), (0x30, phantom)]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(1, first_body + 0x14, second_body + 0x14, 1)]
        );
        let added = calls_to(&rig.e, ADD_PAIRS);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], 0x6000);
        assert_eq!(added[0][2], 1);
        assert_eq!(added[0][3], filter + 8);
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_ENTITY]),
            vec![
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_BROADPHASE_SECOND,
                PROFILE_PHANTOMS,
                PROFILE_CHECK_AGENTS,
                PROFILE_ADD_AGENTS,
                PROFILE_SECTION_END
            ]
        );
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
    }

    #[test]
    fn update_filter_on_entity_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e
            .call(0x00c9_46e0, &args![rig.world, 0x4000u32, 1i32, 1u32]);
        let mut expected = op_bytes(0x12, 0x4000, 1, 0);
        expected[9] = 1;
        assert_eq!(queued.borrow().clone(), vec![(0x5000, expected)]);
    }

    const ADD_BATCH_CALLEES: [u32; 7] = [
        SET_CACHED_SHAPE_DATA,
        SET_TIME_INFORMATION,
        ADD_REFERENCE,
        ISLAND_ADD_ENTITY,
        FIRE_WORLD_ENTITY_ADDED,
        FIRE_ENTITY_ADDED,
        QUEUE_FLUSH_PENDING,
    ];

    /// The arrays a broad phase call received: `(entries of the first array, size of the
    /// second)`.
    type BroadPhaseSeen = Rc<RefCell<Vec<(Vec<u32>, u32)>>>;

    /// A broad phase whose slot `slot` notes the 4-byte entries of its first array and the
    /// size of its second array.
    fn broad_phase_noting(e: &mut Engine, slot: u32) -> (u32, BroadPhaseSeen) {
        let seen: BroadPhaseSeen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(0x0900_0100 + slot, move |e, words| {
            let data = e.mem.u32(words[1]);
            let entries = (0..e.mem.u32(words[1] + 4))
                .map(|i| e.mem.u32(data + 4 * i))
                .collect();
            sink.borrow_mut().push((entries, e.mem.u32(words[2] + 4)));
            Ret::default()
        });
        (
            object_with_vtable(e, 0x20, &[(slot, 0x0900_0100 + slot)]),
            seen,
        )
    }

    /// An island with `bodies` in its body list.
    fn island_of(rig: &mut Rig, bodies: &[u32]) -> u32 {
        let island = rig.e.mem.alloc(0x80);
        let data = array_of(&mut rig.e, bodies);
        rig.e.mem.set_u32(island + 0x48, data);
        rig.e.mem.set_u32(island + 0x4c, bodies.len() as u32);
        island
    }

    #[test]
    fn add_entity_batch_adds_the_bodies_to_the_first_active_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ADD_BATCH_CALLEES);
        silent(&mut rig.e, &[ADD_PAIRS]);
        let shape_calls = recording(&mut rig.e, FAKE_2);
        returning(&mut rig.e, FAKE_3, 0x5555);
        let shape = object_with_vtable(&mut rig.e, 0x20, &[(0x1c, FAKE_2)]);
        let first = object_with_vtable(&mut rig.e, 0x240, &[(0x10, FAKE_3)]);
        let second = object_with_vtable(&mut rig.e, 0x240, &[(0x10, FAKE_3)]);
        rig.e.mem.set_u32(first + 0x10, shape);
        rig.e.mem.set_u8(second + 0xe8, 5);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        let island = island_of(&mut rig, &[0x31, 0x32]);
        rig.e.mem.set_u32(island + 0x50, 3);
        let islands = array_of(&mut rig.e, &[island]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, islands);
        rig.e.mem.set_u32(world + 0x2c, 1);
        rig.e.mem.set_u32(world + 0x20, 0x9999);
        let input = rig.e.mem.alloc(0x10);
        rig.e.mem.set_f32(input + 8, 1.0);
        rig.e.mem.set_u32(world + 0x78, input);
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x104, 10);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let entities = array_of(&mut rig.e, &[first, second]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 2i32, 1i32]);
        // Uids, motions and the world.
        assert_eq!(rig.e.mem.u32(first + 0xd4), 11);
        assert_eq!(rig.e.mem.u32(second + 0xd4), 12);
        assert_eq!(rig.e.mem.u32(world + 0x104), 12);
        assert_eq!(rig.e.mem.u32(first + 0x18), 0x5555);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
        assert_eq!(rig.e.mem.u32(first + 8), world);
        assert_eq!(rig.e.mem.u32(first + 0x30), 1);
        assert_eq!(rig.e.mem.u32(first + 0x40), 0);
        // The island of the entity, the fixed island for the type 5 entity.
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![island, first], vec![0x9999, second]]
        );
        // Room made in the island's body list (at least twice its capacity).
        let reserves = calls_to(&rig.e, ARRAY_RESERVE);
        let allocator_10 = rig.e.mem.u32(rig.router + 0x10);
        let allocator_c = rig.e.mem.u32(rig.router + 0xc);
        assert!(reserves.contains(&vec![allocator_10, island + 0x48, 6, 4]));
        assert!(reserves.contains(&vec![allocator_c, reserves[0][1], 2, 4]));
        assert!(reserves
            .iter()
            .any(|call| call[0] == allocator_c && call[2] == 2 && call[3] == 0x20));
        // The shape got the motion, half the delta time and a slot for its box.
        let shape_calls = shape_calls.borrow().clone();
        assert_eq!(shape_calls.len(), 1);
        assert_eq!(shape_calls[0][..3], [shape, 0x5555, 0.5f32.to_bits()]);
        assert_eq!(
            calls_to(&rig.e, SET_CACHED_SHAPE_DATA),
            vec![vec![first, world, shape]]
        );
        assert_eq!(calls_to(&rig.e, SET_TIME_INFORMATION).len(), 2);
        assert_eq!(seen.borrow().clone(), vec![(vec![first + 0x24], 1)]);
        assert_eq!(
            calls_to(&rig.e, ADD_PAIRS),
            vec![vec![0x6000, calls_to(&rig.e, ADD_PAIRS)[0][1], 0, 0]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_WORLD_ENTITY_ADDED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ENTITY_ADDED),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            record_labels(&rig, &[PROFILE_ADD_ENTITIES]),
            vec![
                PROFILE_ADD_ENTITIES,
                PROFILE_ADD_BROADPHASE,
                PROFILE_CREATE_AGENTS,
                PROFILE_ADDED_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
        // The two lists reserved for the batch go back to the second allocator.
        let freed: Vec<u32> = rig
            .freed_second
            .borrow()
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(freed, vec![0x40, 8]);
        assert!(rig.freed_first.borrow().is_empty());
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    const ISLAND_DESTRUCTOR: u32 = 0x0900_0060;

    /// Sets up a batch that builds a new island; returns the island, which `00d07830`
    /// answers with.
    fn new_island_rig(rig: &mut Rig, bodies: &[u32]) -> u32 {
        silent(&mut rig.e, &ADD_BATCH_CALLEES);
        silent(&mut rig.e, &[ADD_PAIRS]);
        recording(&mut rig.e, ISLAND_DESTRUCTOR);
        let island = object_with_vtable(&mut rig.e, 0x80, &[(0, ISLAND_DESTRUCTOR)]);
        let data = array_of(&mut rig.e, bodies);
        rig.e.mem.set_u32(island + 0x48, data);
        returning(&mut rig.e, ISLAND_CONSTRUCT, island);
        let world = rig.world.addr();
        rig.e.set(rig.world, HkpWorld::m_addsToNewIsland, true);
        let input = rig.e.mem.alloc(0x10);
        rig.e.mem.set_u32(world + 0x78, input);
        let (broad_phase, _) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        island
    }

    #[test]
    fn add_entity_batch_registers_a_new_island_with_the_world() {
        let mut rig = rig();
        let first = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(first + 0x18, 0x4444);
        let island = new_island_rig(&mut rig, &[first]);
        let world = rig.world.addr();
        // Seven active islands already, room for exactly those.
        let list = array_of(&mut rig.e, &[0; 7]);
        rig.e.mem.set_u32(world + 0x28, list);
        rig.e.mem.set_u32(world + 0x2c, 7);
        rig.e.mem.set_u32(world + 0x30, 7);
        let entities = array_of(&mut rig.e, &[first]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 1i32, 1i32]);
        let block = calls_to(&rig.e, ISLAND_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&rig.e, ISLAND_CONSTRUCT), vec![vec![block, world]]);
        assert_eq!(rig.e.mem.u16(block + 4), 0x6c);
        assert_eq!(rig.e.mem.u8(island + 0x25), 1);
        assert_eq!(rig.e.mem.u16(island + 0x20), 7);
        assert_eq!(rig.e.mem.u8(island + 0x26), 5);
        assert_eq!(rig.e.mem.u8(island + 0x24), 1);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![island, first]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x2c), 8);
        let data = rig.e.mem.u32(world + 0x28);
        assert_eq!(rig.e.mem.u32(data + 28), island);
        assert!(calls_to(&rig.e, ISLAND_DESTRUCTOR).is_empty());
    }

    #[test]
    fn add_entity_batch_deletes_a_new_island_nothing_used() {
        let mut rig = rig();
        let fixed_only = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(fixed_only + 0x18, 0x4444);
        let island = new_island_rig(&mut rig, &[]);
        rig.e.mem.set_u8(fixed_only + 0xe8, 5);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x20, 0x9999);
        let entities = array_of(&mut rig.e, &[fixed_only]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 1i32, 0i32]);
        // Mode 0: the id comes from the inactive list's size, the flags are cleared.
        assert_eq!(rig.e.mem.u8(island + 0x26), 0);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![0x9999, fixed_only]]
        );
        assert_eq!(calls_to(&rig.e, ISLAND_DESTRUCTOR), vec![vec![island, 1]]);
    }

    #[test]
    fn add_entity_batch_does_nothing_for_zero_and_queues_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, 0x4000u32, 0i32, 1i32]);
        assert!(queued.borrow().is_empty());
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, 0x4000u32, 3i32, 1i32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(6, 0x4000, 3, 1))]
        );
        assert_eq!(rig.e.mem.u32(rig.stream + 4), rig.area);
    }

    const REMOVE_BATCH_CALLEES: [u32; 6] = [
        FIRE_WORLD_ENTITY_REMOVED,
        FIRE_ENTITY_REMOVED,
        REMOVE_ENTITY_BP,
        ENTITY_RELEASE_BOUNDING_DATA,
        REMOVE_REFERENCE,
        QUEUE_FLUSH_PENDING,
    ];

    #[test]
    fn remove_entity_batch_removes_the_pairs_and_runs_the_callbacks() {
        let mut rig = rig();
        silent(&mut rig.e, &REMOVE_BATCH_CALLEES);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let told = recording(&mut rig.e, FAKE_2);
        let simulation = object_with_vtable(&mut rig.e, 0x20, &[(0x20, FAKE_2)]);
        let deleted = recording(&mut rig.e, FAKE_3);
        let first = object_with_vtable(&mut rig.e, 0x240, &[(0x14, FAKE_3)]);
        let second = object_with_vtable(&mut rig.e, 0x240, &[(0x14, FAKE_3)]);
        rig.e.mem.set_u32(first + 0x10, 1);
        rig.e.mem.set_u32(first + 0x54, 1);
        rig.e.mem.set_u16(second + 4, 0x40);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 8, simulation);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x20);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let entities = array_of(&mut rig.e, &[first, second]);
        rig.e.call(0x00c9_51b0, &args![rig.world, entities, 2i32]);
        assert_eq!(
            told.borrow().clone(),
            vec![vec![simulation, entities, 2, world, 1]]
        );
        assert_eq!(seen.borrow().clone(), vec![(vec![first + 0x24], 0)]);
        let removed = removed.borrow().clone();
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], 0x6000);
        assert_eq!(removed[0][2], 0);
        assert_eq!(
            calls_to(&rig.e, FIRE_WORLD_ENTITY_REMOVED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ENTITY_REMOVED),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_ENTITY_BP),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, ENTITY_RELEASE_BOUNDING_DATA),
            vec![vec![first + 0x30]]
        );
        assert_eq!(deleted.borrow().clone(), vec![vec![first]]);
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            record_labels(&rig, &[PROFILE_REMOVE_ENTITIES]),
            vec![
                PROFILE_REMOVE_ENTITIES,
                PROFILE_REMOVE_BROADPHASE,
                PROFILE_DELETE_AGENTS,
                PROFILE_REMOVE_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
        // The handle list was reserved through the second allocator and goes back to it.
        let freed: Vec<u32> = rig
            .freed_second
            .borrow()
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(freed, vec![8]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_entity_batch_skips_the_broad_phase_without_shapes() {
        let mut rig = rig();
        silent(&mut rig.e, &REMOVE_BATCH_CALLEES);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let simulation = object_with_vtable(&mut rig.e, 0x20, &[(0x20, FAKE_2)]);
        silent(&mut rig.e, &[FAKE_2]);
        let entity = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u16(entity + 4, 0x40);
        rig.e.mem.set_u32(rig.world.addr() + 8, simulation);
        let entities = array_of(&mut rig.e, &[entity]);
        rig.e.call(0x00c9_51b0, &args![rig.world, entities, 1i32]);
        assert!(removed.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_REMOVE_ENTITIES]),
            vec![
                PROFILE_REMOVE_ENTITIES,
                PROFILE_REMOVE_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
    }

    #[test]
    fn remove_entity_batch_does_nothing_for_no_entities_and_queues_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 0i32]);
        lock(&mut rig, 1);
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 0i32]);
        assert!(queued.borrow().is_empty());
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 2i32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(7, 0x4000, 2, 0))]
        );
    }

    #[test]
    fn add_phantom_batch_adds_them_all_to_the_broad_phase() {
        let mut rig = rig();
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[PHANTOM_SET_AABB, ADD_PAIRS]);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let first = phantom_object(&mut rig.e, &log);
        let second = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        // The second phantom's own bounding box maker is the same double.
        let table = rig.e.mem.u32(first);
        rig.e.mem.set_u32(second, table);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let phantoms = array_of(&mut rig.e, &[first, second]);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_54e0, &args![rig.world, phantoms, 2i32]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(vec![first + 0x24, second + 0x24], 2)]
        );
        assert_eq!(phantoms_of(&rig), vec![first, second]);
        assert_eq!(rig.e.mem.u32(first + 0x18), 0x7777);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
        assert_eq!(rig.e.mem.u32(first + 8), world);
        assert_eq!(rig.e.mem.u32(second + 8), world);
        // Each phantom was asked for its box into its own slot, which `00d07950` got too.
        let boxes = calls_to(&rig.e, PHANTOM_SET_AABB);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[1][1], boxes[0][1] + 0x20);
        assert_eq!(rig.e.mem.u32(boxes[0][1]), 0xb0b0_0000 + first);
        assert_eq!(rig.e.mem.u32(boxes[1][1]), 0xb0b0_0000 + second);
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_ADDED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, PHANTOM_AFTER_ADD),
            vec![vec![first], vec![second]]
        );
        assert_eq!(calls_to(&rig.e, ADD_PAIRS).len(), 1);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_phantom_batch_removes_them_all() {
        let mut rig = rig();
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let first = phantom_object(&mut rig.e, &log);
        let second = rig.e.mem.alloc(0x40);
        let table = rig.e.mem.u32(first);
        rig.e.mem.set_u32(second, table);
        set_phantoms(&mut rig, &[first, second, 0x21]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x20);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let phantoms = array_of(&mut rig.e, &[first, second]);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_5800, &args![rig.world, phantoms, 2i32]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(vec![first + 0x24, second + 0x24], 0)]
        );
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(phantoms_of(&rig), vec![0x21]);
        assert_eq!(rig.e.mem.u32(first + 8), 0);
        assert_eq!(log.borrow().clone(), vec![(0x34, first), (0x34, second)]);
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_REMOVED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, PHANTOM_BEFORE_REMOVE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
    }

    /// A world with `agents` blocks: a single active island whose single block holds two
    /// agent entries 0x18 bytes long. The first entry's bodies are in groups (1, 2), the
    /// second's in (0, 1). Returns `(island, first entry, second entry)`.
    fn world_with_agent_entries(rig: &mut Rig) -> (u32, u32, u32) {
        let block = rig.e.mem.alloc(0x100);
        let island = rig.e.mem.alloc(0x80);
        let blocks = array_of(&mut rig.e, &[block]);
        rig.e.mem.set_u32(island + 0x5c, blocks);
        rig.e.mem.set_u32(island + 0x60, 1);
        rig.e.mem.set_u32(island + 0x58, 0x30);
        let groups = [(1i8, 2i8), (0, 1)];
        for (i, (first, second)) in groups.into_iter().enumerate() {
            let entry = block + 0x18 * i as u32;
            rig.e.mem.set_u8(entry + 3, 0x18);
            let body_a = rig.e.mem.alloc(0x40);
            let body_b = rig.e.mem.alloc(0x40);
            rig.e.mem.set_i8(body_a + 0x1a, first);
            rig.e.mem.set_i8(body_b + 0x1a, second);
            rig.e.mem.set_u32(entry + 0x10, body_a);
            rig.e.mem.set_u32(entry + 0x14, body_b);
        }
        let islands = array_of(&mut rig.e, &[island]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, islands);
        rig.e.mem.set_u32(world + 0x2c, 1);
        group_table(rig, &[12]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        (island, block, block + 0x18)
    }

    #[test]
    fn update_filter_on_world_removes_refused_agents_after_each_island() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[UPDATE_SHAPE_COLLECTION_FILTER, QUEUE_FLUSH_PENDING],
        );
        let removed = recording(&mut rig.e, REMOVE_AGENT);
        let (island, first, second) = world_with_agent_entries(&mut rig);
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_6ff0, &args![rig.world, 1i32, 0u32]);
        assert_eq!(removed.borrow().clone(), vec![vec![second]]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![first, 0x7000, island]]
        );
        assert_eq!(rig.e.mem.u8(island + 0x25) & 3, 1);
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(
            record_labels(&rig, &[]),
            vec![PROFILE_UPDATE_FILTER_ON_WORLD, PROFILE_END]
        );
        // Any other mode only removes: a mode 2 run updates nothing.
        rig.e.call(0x00c9_6ff0, &args![rig.world, 2i32, 0u32]);
        assert_eq!(calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER).len(), 1);
        assert_eq!(removed.borrow().len(), 2);
    }

    #[test]
    fn update_filter_on_world_mode_0_updates_every_entity_and_phantom() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                COLLIDABLE_RESET_AGENTS,
                REMOVE_DUPLICATE_PAIRS,
                QUEUE_FLUSH_PENDING,
            ],
        );
        agent_entries(&mut rig.e, &[]);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[]);
        let entity = filtered_entity(&mut rig.e, 1, 0);
        let island = island_of(&mut rig, &[entity]);
        let active = array_of(&mut rig.e, &[island]);
        let phantom = filtered_phantom(&mut rig.e, &log);
        set_phantoms(&mut rig, &[phantom]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, active);
        rig.e.mem.set_u32(world + 0x2c, 1);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        rig.e.mem.set_u32(world + 0x338, 2);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_6ff0, &args![rig.world, 0i32, 1u32]);
        // One broad phase query for the entity, one for the phantom; the flag makes the
        // phantom's slot 0x30 run once.
        let queries = calls_to(&rig.e, 0x0900_0088);
        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0][1], entity + 0x24);
        assert_eq!(queries[1][1], phantom + 0x24);
        assert_eq!(log.borrow().clone(), vec![(0x30, phantom)]);
        // The pending operations were flushed once, at the end.
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING).len(), 1);
        assert_eq!(
            order_of(&rig.e, &[QUEUE_FLUSH_PENDING, 0x0900_0088]),
            vec![0x0900_0088, 0x0900_0088, QUEUE_FLUSH_PENDING]
        );
        assert_eq!(
            record_labels(
                &rig,
                &[
                    PROFILE_UPDATE_FILTER_ON_ENTITY,
                    PROFILE_UPDATE_FILTER_ON_PHANTOM
                ]
            ),
            vec![
                PROFILE_UPDATE_FILTER_ON_WORLD,
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_BROADPHASE_SECOND,
                PROFILE_PHANTOMS,
                PROFILE_CHECK_AGENTS,
                PROFILE_SECTION_END,
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_COLLECTION_FILTER,
                PROFILE_SECTION_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn update_filter_on_world_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_6ff0, &args![rig.world, 1i32, 1u32]);
        let mut expected = vec![0u8; 16];
        expected[..3].copy_from_slice(&[0x15, 1, 1]);
        assert_eq!(queued.borrow().clone(), vec![(0x5000, expected)]);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
    }

    /// An action whose vtable slot 0x10 hands out `entities`.
    fn action_with_entities(e: &mut Engine, entities: &[u32]) -> u32 {
        let list = entities.to_vec();
        e.register_double(0x0900_0070, move |e, words| {
            let array = words[1];
            let data = e.mem.u32(array);
            for (i, entity) in list.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *entity);
            }
            e.mem.set_u32(array + 4, list.len() as u32);
            Ret::default()
        });
        object_with_vtable(e, 0x50, &[(0x10, 0x0900_0070)])
    }

    /// An entity in `island` whose action list (+0x208) holds `actions` with room for
    /// `capacity`.
    fn entity_with_actions(e: &mut Engine, island: u32, actions: &[u32], capacity: u16) -> u32 {
        let entity = e.mem.alloc(0x240);
        e.mem.set_u32(entity + 0xcc, island);
        let block = e
            .mem
            .alloc(4 * u32::from(capacity).max(actions.len() as u32).max(1) + 8);
        let data = block + 4;
        for (i, action) in actions.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *action);
        }
        e.mem.set_u32(entity + 0x208, data);
        e.mem.set_u16(entity + 0x20c, actions.len() as u16);
        e.mem.set_u16(entity + 0x20e, capacity);
        entity
    }

    fn actions_of(e: &Engine, entity: u32) -> Vec<u32> {
        let data = e.mem.u32(entity + 0x208);
        (0..u32::from(e.mem.u16(entity + 0x20c)))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn island_with_id(e: &mut Engine, id: u16) -> u32 {
        let island = e.mem.alloc(0x80);
        e.mem.set_u16(island + 0x20, id);
        island
    }

    const ACTION_CALLEES: [u32; 6] = [
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        ISLAND_ADD_ACTION,
        MERGE_ISLANDS,
        FIRE_ACTION_ADDED,
        QUEUE_FLUSH_PENDING,
    ];

    fn growing_action_lists(e: &mut Engine) {
        e.register(ACTION_LIST_GROW, |e, words| {
            let list = words[0];
            let size = u32::from(e.mem.u16(list + 4));
            let capacity = u32::from(e.mem.u16(list + 6) & 0x3fff).max(2) * 2;
            let block = e.mem.alloc(capacity * 4);
            if size != 0 {
                let old = e.mem.bytes(e.mem.u32(list), size * 4);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(list, block);
            e.mem.set_u16(list + 6, capacity as u16);
            Ret::default()
        });
    }

    #[test]
    fn add_action_puts_the_action_in_the_first_real_island_and_merges_the_others() {
        let mut rig = rig();
        silent(&mut rig.e, &ACTION_CALLEES);
        growing_action_lists(&mut rig.e);
        let first_island = island_with_id(&mut rig.e, 1);
        let second_island = island_with_id(&mut rig.e, 2);
        let same_island_entity = entity_with_actions(&mut rig.e, first_island, &[], 4);
        let first = entity_with_actions(&mut rig.e, first_island, &[], 4);
        let second = entity_with_actions(&mut rig.e, second_island, &[0x61], 1);
        let action = action_with_entities(&mut rig.e, &[first, second, same_island_entity]);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        let returned = rig.e.call(0x00c9_7370, &args![rig.world, action]).u32();
        let world = rig.world.addr();
        assert_eq!(returned, action);
        assert_eq!(rig.e.mem.u32(action + 8), world);
        assert_eq!(actions_of(&rig.e, first), vec![action]);
        assert_eq!(actions_of(&rig.e, second), vec![0x61, action]);
        assert_eq!(actions_of(&rig.e, same_island_entity), vec![action]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![first_island, action]]
        );
        assert_eq!(
            calls_to(&rig.e, MERGE_ISLANDS),
            vec![vec![world, first, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ACTION_ADDED),
            vec![vec![world, action]]
        );
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    ISLAND_ADD_ACTION,
                    QUEUE_FLUSH_PENDING,
                    FIRE_ACTION_ADDED
                ]
            ),
            vec![
                ADD_REFERENCE,
                ISLAND_ADD_ACTION,
                QUEUE_FLUSH_PENDING,
                FIRE_ACTION_ADDED
            ]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn add_action_falls_back_to_the_first_entitys_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ACTION_CALLEES);
        growing_action_lists(&mut rig.e);
        let loose = island_with_id(&mut rig.e, 0xffff);
        let first = entity_with_actions(&mut rig.e, loose, &[], 4);
        let second = entity_with_actions(&mut rig.e, loose, &[], 4);
        let action = action_with_entities(&mut rig.e, &[first, second]);
        rig.e.call(0x00c9_7370, &args![rig.world, action]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![loose, action]]
        );
        assert!(calls_to(&rig.e, MERGE_ISLANDS).is_empty());
        assert_eq!(actions_of(&rig.e, second), vec![action]);
    }

    #[test]
    fn add_action_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let returned = rig.e.call(0x00c9_7370, &args![rig.world, 0x1234u32]).u32();
        assert_eq!(returned, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0xa, 0x1234, 0, 0))]
        );
    }

    #[test]
    fn remove_action_immediately_takes_the_action_out_of_everything() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let owner = rig.e.mem.alloc(0x80);
        let list = array_of(&mut rig.e, &[0x91, 0x92]);
        rig.e.mem.set_u32(owner + 0x40, list);
        rig.e.mem.set_u32(owner + 0x44, 2);
        rig.e.mem.set_u32(owner + 0x48, 2);
        let island = island_with_id(&mut rig.e, 1);
        rig.e.mem.set_u32(island + 0x18, owner);
        rig.e.mem.set_u16(island + 0x22, 0xffff);
        let first = entity_with_actions(&mut rig.e, island, &[0x71, 0x72, 0x73], 3);
        let second = entity_with_actions(&mut rig.e, island, &[0x72], 2);
        let action = action_with_entities(&mut rig.e, &[first, second]);
        // The first entity's list gets the action at index 1, the second does not have it.
        rig.e.mem.set_u32(action + 0xc, island);
        rig.e.mem.set_u32(action + 8, 0x5555);
        let data = rig.e.mem.u32(first + 0x208);
        rig.e.mem.set_u32(data + 4, action);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e.call(0x00c9_7510, &args![rig.world, action]);
        let world = rig.world.addr();
        // The last entry moved into the action's slot.
        assert_eq!(actions_of(&rig.e, first), vec![0x71, 0x73]);
        // The second entity did not hold it: its list lost one entry all the same (the slot
        // -1 got the last entry).
        assert_eq!(rig.e.mem.u16(second + 0x20c), 0);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![island, action]]
        );
        assert_eq!(rig.e.mem.u32(action + 8), 0);
        assert_eq!(
            calls_to(&rig.e, FIRE_ACTION_REMOVED),
            vec![vec![world, action]]
        );
        // The island got its index in the owner's list and was added to it.
        assert_eq!(rig.e.mem.u16(island + 0x22), 2);
        assert_eq!(rig.e.mem.u32(owner + 0x44), 3);
        let data = rig.e.mem.u32(owner + 0x40);
        assert_eq!(rig.e.mem.u32(data + 8), island);
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![action], vec![action]]
        );
        assert_eq!(calls_to(&rig.e, ADD_REFERENCE), vec![vec![action]]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING).len(), 1);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_action_is_queued_or_immediate() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_7f50, &args![rig.world, 0x1234u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0xb, 0x1234, 0, 0))]
        );
        let (mut immediate, action) = rig_with_removable_action();
        let world = immediate.world;
        immediate.e.call(0x00c9_7f50, &args![world, action]);
        assert_eq!(
            calls_to(&immediate.e, FIRE_ACTION_REMOVED),
            vec![vec![world.addr(), action]]
        );
    }

    /// A world and an action with one entity and an island, for the paths that really
    /// remove it.
    fn rig_with_removable_action() -> (Rig, u32) {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let owner = rig.e.mem.alloc(0x80);
        let island = island_with_id(&mut rig.e, 1);
        rig.e.mem.set_u32(island + 0x18, owner);
        rig.e.mem.set_u16(island + 0x22, 0);
        let entity = entity_with_actions(&mut rig.e, island, &[], 1);
        let action = action_with_entities(&mut rig.e, &[entity]);
        rig.e.mem.set_u32(action + 0xc, island);
        let data = rig.e.mem.u32(entity + 0x208);
        rig.e.mem.set_u32(data, action);
        rig.e.mem.set_u16(entity + 0x20c, 1);
        (rig, action)
    }

    /// A constraint whose data (+0xC) is of `data_kind` (vtable slot 0x20), whose own kind
    /// (slot 0x18) is `kind`, and whose action (+0x44) has the world `action_world`.
    /// The data object's slot 0x40 and its owner's slot 0x24 are logged in `log`.
    fn constraint_object(
        e: &mut Engine,
        log: &Records,
        kind: u32,
        data_kind: u32,
        action_world: u32,
    ) -> u32 {
        returning(e, 0x0900_0081, kind);
        returning(e, 0x0900_0082, data_kind);
        let sink = log.clone();
        e.register_double(0x0900_0083, move |_, words| {
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(0x0900_0084, move |e, words| {
            e.mem.set_u16(words[2], 0x1111);
            e.mem.set_u16(words[2] + 4, 0x2222);
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let owner = object_with_vtable(e, 0x20, &[(0x24, 0x0900_0084)]);
        let data = object_with_vtable(e, 0x40, &[(0x20, 0x0900_0082), (0x40, 0x0900_0083)]);
        e.mem.set_u32(data + 0x18, owner);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 8, action_world);
        let constraint = object_with_vtable(e, 0x60, &[(0x18, 0x0900_0081)]);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x44, action);
        constraint
    }

    #[test]
    fn add_constraint_numbers_the_constraint_and_adds_it_to_the_islands() {
        let mut rig = rig();
        silent(&mut rig.e, &[CONSTRAINT_PREPARE, QUEUE_FLUSH_PENDING]);
        returning(&mut rig.e, ADD_CONSTRAINT_SI, 0x77);
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 1, 0xc, 0x1000);
        let data = rig.e.mem.u32(constraint + 0xc);
        let owner = rig.e.mem.u32(data + 0x18);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x10c, 4);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let result = rig.e.call(0x00c9_7d30, &args![rig.world, constraint]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(rig.e.mem.u16(data + 0x1c), 0x1111);
        assert_eq!(rig.e.mem.u16(data + 0x1e), 0x2222);
        assert_eq!(rig.e.mem.u32(world + 0x10c), 5);
        assert_eq!(rig.e.mem.u32(constraint + 0x34), 5);
        assert_eq!(
            calls_to(&rig.e, ADD_CONSTRAINT_SI),
            vec![vec![world, constraint, 1]]
        );
        let log = log.borrow().clone();
        // The owner got the flag in the low byte of its first argument, and the data object
        // was told about the new constraint (kind 1).
        assert_eq!(log.len(), 2);
        assert_eq!(log[0][0], owner);
        assert_eq!(log[0][1] & 0xff, 1);
        assert_eq!(log[1], vec![data]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
    }

    #[test]
    fn add_constraint_skips_the_data_update_for_other_kinds() {
        let mut rig = rig();
        silent(&mut rig.e, &[CONSTRAINT_PREPARE, QUEUE_FLUSH_PENDING]);
        returning(&mut rig.e, ADD_CONSTRAINT_SI, 0x77);
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 0, 5, 0x1000);
        let data = rig.e.mem.u32(constraint + 0xc);
        rig.e.call(0x00c9_7d30, &args![rig.world, constraint]);
        assert_eq!(rig.e.mem.u16(data + 0x1c), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn add_constraint_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let result = rig.e.call(0x00c9_7d30, &args![rig.world, 0x1234u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(8, 0x1234, 0, 0))]
        );
    }

    #[test]
    fn remove_constraint_detaches_it_and_reports_success() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let log = Rc::new(RefCell::new(vec![]));
        // Kind 1, but its action belongs to another world: it stays.
        let constraint = constraint_object(&mut rig.e, &log, 1, 0, 0x1000);
        let result = rig.e.mem.alloc(4);
        rig.e.mem.set_u8(result, 0x55);
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, constraint]);
        let world = rig.world.addr();
        assert_eq!(rig.e.mem.u8(result), 1);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    REMOVE_CONSTRAINT_SI,
                    CONSTRAINT_DETACH,
                    REMOVE_REFERENCE,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                ADD_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_CONSTRAINT_SI),
            vec![vec![world, constraint, 1]]
        );
        assert_eq!(
            calls_to(&rig.e, CONSTRAINT_DETACH),
            vec![vec![constraint, world]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_constraint_removes_the_action_of_a_kind_1_constraint_in_this_world() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                QUEUE_FLUSH_PENDING,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
            ],
        );
        let (action, island) = {
            let owner = rig.e.mem.alloc(0x80);
            let island = island_with_id(&mut rig.e, 1);
            rig.e.mem.set_u32(island + 0x18, owner);
            rig.e.mem.set_u16(island + 0x22, 0);
            let entity = entity_with_actions(&mut rig.e, island, &[], 1);
            let action = action_with_entities(&mut rig.e, &[entity]);
            rig.e.mem.set_u32(action + 0xc, island);
            let data = rig.e.mem.u32(entity + 0x208);
            rig.e.mem.set_u32(data, action);
            rig.e.mem.set_u16(entity + 0x20c, 1);
            (action, island)
        };
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 1, 0, rig.world.addr());
        rig.e.mem.set_u32(constraint + 0x44, action);
        rig.e.mem.set_u32(action + 8, rig.world.addr());
        let result = rig.e.mem.alloc(4);
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, constraint]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![island, action]]
        );
        assert_eq!(rig.e.mem.u8(result), 1);
    }

    #[test]
    fn remove_constraint_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let result = rig.e.mem.alloc(4);
        rig.e.mem.set_u8(result, 0x55);
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, 0x1234u32]);
        assert_eq!(rig.e.mem.u8(result), 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(9, 0x1234, 0, 0))]
        );
    }
}

/// Test fixtures shared with the other `Havok SDK` units that write profiling records.
#[cfg(test)]
pub(crate) mod profiling_support {
    use super::*;

    /// An engine with a profiling stream that holds `records` records; returns it and the
    /// stream block and the first record address.
    pub(crate) fn profiling_engine(records: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(PROFILE_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let area = e.mem.alloc(12 * records.max(1));
        e.mem.set_u32(stream + 4, area);
        e.mem.set_u32(stream + 0xc, area + 12 * records);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, stream, area)
    }

    /// The labels of the profiling records written so far.
    pub(crate) fn labels(e: &Engine, stream: u32, start: u32) -> Vec<u32> {
        let cursor = e.mem.u32(stream + 4);
        (start..cursor)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00c98310() {
        assert_eq!(
            check(0x00c9_8310, 0x00c9_82e0, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98320() {
        assert_eq!(
            check(0x00c9_8320, 0x00c9_82e0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98330() {
        assert_eq!(
            check(0x00c9_8330, 0x00c9_82e0, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98340() {
        assert_eq!(
            check(0x00c9_8340, 0x00c9_82e0, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
