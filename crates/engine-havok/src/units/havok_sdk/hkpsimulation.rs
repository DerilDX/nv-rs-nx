//! `Havok SDK/hkpsimulation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSimulation` fields used here: `+0xc` the world, `+0x10` a flag byte, `+0x14` / `+0x18` /
//! `+0x1c` / `+0x20` / `+0x24` times, `+0x28` the stage of the running step (0 when idle).
//! The world's lock count is at `+0x94` and its step info at `+0x1d0`.
//!
//! Float code works on `float` memory the way the x87 does: loads copy through `x87_quiet`,
//! sums and differences are taken in `double` and rounded at each `float` store, and the compares
//! read the same flags the exe tests (NaN included).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsimpleconstraintutil::{add, cross, mul, splat, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, profile_record, store_vec, sum3, PROFILE_END,
    SIGN_MASK,
};
use std::cmp::Ordering;

/// `hkpWorld::getCurrentTime` and `getCurrentPsiTime` (`00c91020`, `00c91030`: thiscall on the
/// world, answer a float in ST0).
const WORLD_GET_CURRENT_TIME: u32 = 0x00c9_1020;
const WORLD_GET_CURRENT_PSI_TIME: u32 = 0x00c9_1030;
/// The two calls that finish releasing a world lock (thiscall on the world).
const WORLD_UNLOCK_FIRST: u32 = 0x00c9_1120;
const WORLD_UNLOCK_SECOND: u32 = 0x00c9_1140;
/// `hkpWorld::getMemoryWatchDog` (`00c93570`, thiscall on the world) and
/// `hkpWorldMemoryUtil::tryToRecoverFromMemoryErrors` (`00d01df0`, cdecl: the world).
const WORLD_GET_MEMORY_WATCH_DOG: u32 = 0x00c9_3570;
const TRY_TO_RECOVER_FROM_MEMORY_ERRORS: u32 = 0x00d0_1df0;
/// `hkpWorld::updateCollisionFilterOnWorld` (`00c96ff0`, thiscall on the world, two stack words).
const UPDATE_COLLISION_FILTER_ON_WORLD: u32 = 0x00c9_6ff0;
/// `hkpWorldCallbackUtil::firePostCollideCallback` (`00d014f0`, cdecl: world, step info) and
/// `firePostSimulationCallback` (`00d01380`, cdecl: world).
const FIRE_POST_COLLIDE_CALLBACK: u32 = 0x00d0_14f0;
const FIRE_POST_SIMULATION_CALLBACK: u32 = 0x00d0_1380;
/// `hkpSimulation::integrateInternal` (`00cf8da0`, thiscall on the simulation, one stack word:
/// the step info).
const INTEGRATE_INTERNAL: u32 = 0x00cf_8da0;
/// `hkSweptTransformUtil::backStepMotionState` (`00cf1b10`, cdecl: float time, motion state).
const BACK_STEP_MOTION_STATE: u32 = 0x00cf_1b10;
/// `hkRigidMotionUtilStep` (`00d28680`, cdecl: step info, entities, count, stride) and
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (`00d1a330`, cdecl: collision input, entities,
/// count).
const RIGID_MOTION_UTIL_STEP: u32 = 0x00d2_8680;
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpSimulation::collideEntitiesBroadPhaseDiscrete` (`00cf9c60`, cdecl: entities, count,
/// world).
const COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE: u32 = 0x00cf_9c60;
/// `00cfa7d0` (thiscall on the local collision input copy, one stack word: the original).
const COLLISION_INPUT_PREPARE_COPY: u32 = 0x00cf_a7d0;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// A global that is `1` while the world is being torn down (`01269200`): the step functions then
/// give up early.
const ABORT_FLAG: u32 = 0x0126_9200;
/// Float constants: `-1.0` (the "no snap time" marker) and `1e-4` (the snap tolerance).
const NO_SNAP_TIME: u32 = 0x0101_2054;
const SNAP_TOLERANCE: u32 = 0x0101_41cc;
/// Profiling labels: `reCollideAfterStepFailure`' post-collide record, `advanceTime`'s
/// post-simulation record and `integrate`'s begin record.
const PROFILE_POST_COLLIDE: u32 = 0x010c_c530;
const PROFILE_POST_SIMULATION: u32 = 0x010c_c540;
const PROFILE_INTEGRATE: u32 = 0x010c_c568;
/// `collideInternal`'s records: the whole call, the narrow phase of one island and the island
/// post-collide callback.
const PROFILE_COLLIDE: u32 = 0x010c_c64c;
const PROFILE_NARROW_PHASE: u32 = 0x010c_c54c;
const PROFILE_ISLAND_POST_COLLIDE: u32 = 0x010c_c634;
/// `hkAgentNnMachine_ProcessTrack` (`00cff3c0`, cdecl: island, the island's agent track at
/// `+0x58`, collision input) and `hkpWorldCallbackUtil::fireIslandPostCollideCallback`
/// (`00d01650`, cdecl: world, island, step info).
const PROCESS_TRACK: u32 = 0x00cf_f3c0;
const FIRE_ISLAND_POST_COLLIDE_CALLBACK: u32 = 0x00d0_1650;
/// The vtable of `hkpSimulation`.
const SIMULATION_VTABLE: u32 = 0x010c_c4f4;

/// The step info the exe builds from two times: `(start, end, end - start, 1 / (end - start))`
/// as `float` bits, the reciprocal being `0.0` when the difference is zero. Each time is loaded
/// through the x87 stack (signalling NaNs come out quiet), the difference is taken in `double`
/// and rounded to `float`, and so is the reciprocal.
fn make_step_info(start: f32, end: f32) -> [u32; 4] {
    let start = x87_quiet(start);
    let end = x87_quiet(end);
    let delta = (f64::from(end) - f64::from(start)) as f32;
    let inverse = if delta == 0.0 {
        0.0
    } else {
        (1.0f64 / f64::from(delta)) as f32
    };
    [
        start.to_bits(),
        end.to_bits(),
        delta.to_bits(),
        inverse.to_bits(),
    ]
}

fn read_step_info(e: &Engine, at: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
}

fn store_step_info(e: &mut Engine, at: u32, info: [u32; 4]) {
    for (index, word) in info.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * index as u32, word);
    }
}

/// The tail every function that locked the world ends with: decrements the lock count at
/// `world + 0x94` and, when it reaches zero with the byte at `+0x9c` clear, runs the pending
/// work: `00c91120` when `+0x8c` is set, `00c91140` when `+0xa4` is 1 and `+0x90` is set.
fn release_world_lock(e: &mut Engine, world: u32) {
    let count = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, count);
    if count != 0 || e.mem.u8(world + 0x9c) != 0 {
        return;
    }
    if e.mem.u32(world + 0x8c) != 0 {
        e.call(WORLD_UNLOCK_FIRST, &args![world]);
    }
    if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
        e.call(WORLD_UNLOCK_SECOND, &args![world]);
    }
}

// Translated from 00cf8730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::stepDeltaTime` (Xbox PDB), `(this, delta_time)`: runs the stages of a step
/// that the stage word at +0x28 allows, each through a virtual call on the simulation that
/// re-reads the word first: slot `0x10` with `delta_time` (when the stage is 0 or 1), slot
/// `0x14` (0 or 2), slot `0x18` (0 or 3). Then, when the stage is not 0 and the world has a
/// memory watchdog (`getMemoryWatchDog` answers non-zero), runs
/// `tryToRecoverFromMemoryErrors(world)`. Answers the stage word.
pub fn hkp_simulation_step_delta_time(e: &mut Engine, this: Ptr, delta_time: f32) -> u32 {
    let base = this.addr();
    let stage = e.mem.u32(base + 0x28);
    if stage == 0 || stage == 1 {
        e.vcall(base, 0x10, &args![x87_quiet(delta_time)]);
    }
    let stage = e.mem.u32(base + 0x28);
    if stage == 0 || stage == 2 {
        e.vcall(base, 0x14, &args![]);
    }
    let stage = e.mem.u32(base + 0x28);
    if stage == 0 || stage == 3 {
        e.vcall(base, 0x18, &args![]);
    }
    if e.mem.u32(base + 0x28) != 0 {
        let world = e.mem.u32(base + 0xc);
        if e.call(WORLD_GET_MEMORY_WATCH_DOG, &args![world]).u32() != 0 {
            let world = e.mem.u32(base + 0xc);
            e.call(TRY_TO_RECOVER_FROM_MEMORY_ERRORS, &args![world]);
        }
    }
    e.mem.u32(base + 0x28)
}

// Translated from 00cf87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::hkpSimulation` (Xbox PDB), `(this, world)`: zeroes the floats at +0x14 and
/// +0x18, stores `1` in the 16-bit field at +6, copies the floats `-1.0` (`01012054`) to +0x20
/// and `1e-4` (`010141cc`) to +0x24 (through the x87 stack), the vtable `010cc4f4`, the world at
/// +0xc, zero at +0x28 and +8, and `1` in the byte at +0x10. Returns `this`.
pub fn hkp_simulation_construct(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u16(base + 6, 1);
    let no_snap = x87_quiet_bits(e.global::<u32>(NO_SNAP_TIME));
    e.mem.set_u32(base + 0x20, no_snap);
    let tolerance = x87_quiet_bits(e.global::<u32>(SNAP_TOLERANCE));
    e.mem.set_u32(base, SIMULATION_VTABLE);
    e.mem.set_u32(base + 0x24, tolerance);
    e.mem.set_u32(base + 0xc, world);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u8(base + 0x10, 1);
    e.mem.set_u32(base + 8, 0);
    this
}

// Translated from 00cf8820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::reCollideAfterStepFailure` (Xbox PDB): runs
/// `updateCollisionFilterOnWorld(0, 1)` on the world, builds the step info (see
/// `make_step_info`) from `(+0x14, +0x18)` when the stage word at +0x28 is 3 and from
/// `(+0x18, +0x1c + +0x18)` otherwise, and passes it to virtual slot `0x34`. When the global
/// at `01269200` is 1 it ends the profile record and answers the stage word unchanged.
/// Otherwise, at stage 2 it adds `+0x1c` to the time at `+0x18`, fires the post-collide callback
/// (`00d014f0(world, step info)`, between profile records `010cc530` and the end label) when
/// the world's word at `+0x180` is set, and sets the byte at `+0x10`. Finally clears the stage
/// word and answers `0`.
pub fn hkp_simulation_re_collide_after_step_failure(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let world = e.mem.u32(base + 0xc);
    e.call(UPDATE_COLLISION_FILTER_ON_WORLD, &args![world, 0u32, 1u32]);
    let (start, end) = if e.mem.u32(base + 0x28) == 3 {
        (e.mem.f32(base + 0x14), e.mem.f32(base + 0x18))
    } else {
        let start = x87_quiet(e.mem.f32(base + 0x18));
        let end = (f64::from(x87_quiet(e.mem.f32(base + 0x1c)))
            + f64::from(x87_quiet(e.mem.f32(base + 0x18)))) as f32;
        (start, end)
    };
    let info = make_step_info(start, end);
    e.with_stack(0x10, |e, step| {
        store_step_info(e, step.addr(), info);
        e.vcall(base, 0x34, &args![step]);
        if e.global::<u32>(ABORT_FLAG) == 1 {
            profile_record(e, PROFILE_END);
            return e.mem.u32(base + 0x28);
        }
        if e.mem.u32(base + 0x28) == 2 {
            let sum = (f64::from(x87_quiet(e.mem.f32(base + 0x1c)))
                + f64::from(x87_quiet(e.mem.f32(base + 0x18)))) as f32;
            e.mem.set_f32(base + 0x18, sum);
            let world = e.mem.u32(base + 0xc);
            if e.mem.u32(world + 0x180) != 0 {
                profile_record(e, PROFILE_POST_COLLIDE);
                let world = e.mem.u32(base + 0xc);
                e.call(FIRE_POST_COLLIDE_CALLBACK, &args![world, step]);
                profile_record(e, PROFILE_END);
            }
            e.mem.set_u8(base + 0x10, 1);
        }
        e.mem.set_u32(base + 0x28, 0);
        0
    })
}

// Translated from 00cf89c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::snapSimulateTimeAndGetTimeToAdvanceTo` (engine map), answers a float (ST0).
/// With `marker` the global `-1.0` (`01012054`): unless the time at +0x20 equals `marker`
/// (NaN counts as different), when the tolerance at +0x24 is greater than `|(+0x20) - (+0x18)|`
/// the time at +0x20 is replaced by the one at +0x18. Then, if +0x20 equals `marker`, answers
/// the time at +0x18; otherwise the smaller of +0x18 and +0x20, the time at +0x20 when they
/// compare equal or unordered.
pub fn hkp_simulation_snap_simulate_time_and_get_time_to_advance_to(
    e: &mut Engine,
    this: Ptr,
) -> f32 {
    let base = this.addr();
    let marker = x87_quiet(e.global::<f32>(NO_SNAP_TIME));
    if marker.partial_cmp(&e.mem.f32(base + 0x20)) != Some(Ordering::Equal) {
        let difference = (f64::from(x87_quiet(e.mem.f32(base + 0x20)))
            - f64::from(x87_quiet(e.mem.f32(base + 0x18)))) as f32;
        let distance = x87_quiet(difference).abs();
        let tolerance = x87_quiet(e.mem.f32(base + 0x24));
        if tolerance.partial_cmp(&distance) == Some(Ordering::Greater) {
            let time = x87_quiet_bits(e.mem.u32(base + 0x18));
            e.mem.set_u32(base + 0x20, time);
        }
    }
    let snapped = marker.partial_cmp(&e.mem.f32(base + 0x20));
    let time = x87_quiet(e.mem.f32(base + 0x18));
    if snapped == Some(Ordering::Equal) {
        return time;
    }
    let snap_time = x87_quiet(e.mem.f32(base + 0x20));
    if snap_time.partial_cmp(&time) == Some(Ordering::Greater) {
        time
    } else {
        snap_time
    }
}

// Translated from 00cf8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::advanceTime` (Xbox PDB): stores the answer of
/// `snapSimulateTimeAndGetTimeToAdvanceTo` at +0x14. When the time at +0x20 is not greater than
/// it (and not unordered) and the world's word at +0x168 is set, fires the post-simulation
/// callback (`00d01380(world)`, between profile records `010cc540` and the end label). Clears
/// the stage word and answers `0`.
pub fn hkp_simulation_advance_time(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let advanced = hkp_simulation_snap_simulate_time_and_get_time_to_advance_to(e, this);
    let advanced = x87_quiet(advanced);
    e.mem.set_f32(base + 0x14, advanced);
    let limit = x87_quiet(e.mem.f32(base + 0x20));
    if matches!(
        limit.partial_cmp(&advanced),
        Some(Ordering::Less | Ordering::Equal)
    ) {
        let world = e.mem.u32(base + 0xc);
        if e.mem.u32(world + 0x168) != 0 {
            profile_record(e, PROFILE_POST_SIMULATION);
            let world = e.mem.u32(base + 0xc);
            e.call(FIRE_POST_SIMULATION_CALLBACK, &args![world]);
            profile_record(e, PROFILE_END);
        }
    }
    e.mem.set_u32(base + 0x28, 0);
    0
}

// Translated from 00cf8b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`hkpSimulation`, `__fastcall`): for each island in the world's array at
/// +0x28 (size +0x2c) and each entity of the island's array at +0x38 (size +0x3c), calls virtual
/// slot `0xc` of the entity with the world's step info (`world + 0x1d0`).
pub fn fn_00cf8b00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let islands = e.mem.u32(base + 0xc) + 0x28;
    let mut island_index = 0i32;
    while island_index < e.mem.i32(islands + 4) {
        let island = e.mem.u32(e.mem.u32(islands) + 4 * island_index as u32);
        let entities = island + 0x38;
        let mut entity_index = 0i32;
        while entity_index < e.mem.i32(entities + 4) {
            let entity = e.mem.u32(e.mem.u32(entities) + 4 * entity_index as u32);
            let world = e.mem.u32(base + 0xc);
            e.vcall(entity, 0xc, &args![world + 0x1d0]);
            entity_index += 1;
        }
        island_index += 1;
    }
}

// Translated from 00cf9340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::integrate` (Xbox PDB), `(this, delta_time)`: between profile records
/// (`010cc568` and the end label) stores `delta_time` at +0x1c (through the x87 stack), builds
/// the step info from `(+0x18, delta_time + +0x18)`, writes it to the world's `+0x1d0` and to
/// `+0x50` of the world's collision input (the word at `world + 0x78`), calls virtual slot
/// `0x10` of the object at `world + 0x4c` with `(world, step info)` and runs `integrateInternal`
/// on the simulation. The result is stored in the stage word at +0x28; when it is zero the byte
/// at +0x10 is cleared. Answers the result.
pub fn hkp_simulation_integrate(e: &mut Engine, this: Ptr, delta_time: f32) -> u32 {
    let base = this.addr();
    profile_record(e, PROFILE_INTEGRATE);
    let delta_time = x87_quiet(delta_time);
    e.mem.set_f32(base + 0x1c, delta_time);
    let start = x87_quiet(e.mem.f32(base + 0x18));
    let end = (f64::from(delta_time) + f64::from(x87_quiet(e.mem.f32(base + 0x18)))) as f32;
    let info = make_step_info(start, end);
    let world = e.mem.u32(base + 0xc);
    store_step_info(e, world + 0x1d0, info);
    let collision_input = e.mem.u32(world + 0x78);
    store_step_info(e, collision_input + 0x50, info);
    let result = e.with_stack(0x10, |e, step| {
        store_step_info(e, step.addr(), info);
        let world = e.mem.u32(base + 0xc);
        let object = e.mem.u32(world + 0x4c);
        e.vcall(object, 0x10, &args![world, step]);
        e.call(INTEGRATE_INTERNAL, &args![this, step]).u32()
    });
    e.mem.set_u32(base + 0x28, result);
    if result == 0 {
        e.mem.set_u8(base + 0x10, 0);
    }
    profile_record(e, PROFILE_END);
    result
}

// Translated from 00cfa400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`hkpSimulation`, `RET 0x10`): runs the unnamed function at `00cf9480`
/// with the callback `00cf92c0`: `(entities, count, collision_input, 0x00cf92c0, word)`.
/// Register value first: ECX (`this`) is never read.
pub fn fn_00cfa400(
    e: &mut Engine,
    _this: Ptr,
    entities: u32,
    count: i32,
    collision_input: u32,
    word: u32,
) {
    e.call(
        0x00cf_9480,
        &args![entities, count, collision_input, 0x00cf_92c0u32, word],
    );
}

// Translated from 00cfa430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::resetCollisionInformationForEntities` (Xbox PDB), `(this, entities, count,
/// world, flags)`, `RET 0x10` (the low byte of the last word is the flags). When bit 1 of
/// `flags` is set, runs `00cf9480(entities, count, world's collision input (+0x78), 0x00cf87a0,
/// 0)`; when bit 2 is set, `00d1a300(entities, count)` (cdecl). Register value first: ECX
/// (`this`) is never read.
pub fn hkp_simulation_reset_collision_information_for_entities(
    e: &mut Engine,
    _this: Ptr,
    entities: u32,
    count: i32,
    world: u32,
    flags: u8,
) {
    if flags & 2 != 0 {
        let collision_input = e.mem.u32(world + 0x78);
        e.call(
            0x00cf_9480,
            &args![entities, count, collision_input, 0x00cf_87a0u32, 0u32],
        );
    }
    if flags & 4 != 0 {
        e.call(0x00d1_a300, &args![entities, count]);
    }
}

// Translated from 00cfa470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::reintegrateAndRecollideEntities` (Xbox PDB), `(this, entities, count, world,
/// flags)`, `RET 0x10`. Locks the world (+0x94), builds the step info from the world's current
/// time and current PSI time (`getCurrentTime`, `getCurrentPsiTime`) into `+0x50` of the world's
/// collision input, and then, by `flags`: bit 0: for every entity whose byte at +0xe8 is not 5,
/// `backStepMotionState(world time, entity + 0xf0)`, then `hkRigidMotionUtilStep(step info,
/// entities, count, 0xe0)` and `entityBatchRecalcAabb(collision input, entities, count)`; bit 1:
/// `collideEntitiesBroadPhaseDiscrete(entities, count, world)`; bit 2: the unnamed `00cfa400`
/// with the collision input; else, with bit 0 set, virtual slot `0x20` of the simulation with
/// `(entities, count, world, 2)`. Unlocks the world.
pub fn hkp_simulation_reintegrate_and_recollide_entities(
    e: &mut Engine,
    this: Ptr,
    entities: u32,
    count: i32,
    world: u32,
    flags: u32,
) {
    e.mem
        .set_u32(world + 0x94, e.mem.u32(world + 0x94).wrapping_add(1));
    let start = e.call(WORLD_GET_CURRENT_TIME, &args![world]).f32();
    let end = e.call(WORLD_GET_CURRENT_PSI_TIME, &args![world]).f32();
    let info = make_step_info(start, end);
    let collision_input = e.mem.u32(world + 0x78);
    store_step_info(e, collision_input + 0x50, info);
    let integrate = flags & 1 != 0;
    if integrate {
        for index in 0..count.max(0) as u32 {
            let entity = e.mem.u32(entities + 4 * index);
            if e.mem.u8(entity + 0xe8) != 5 {
                let time = e.call(WORLD_GET_CURRENT_TIME, &args![world]).f32();
                e.call(BACK_STEP_MOTION_STATE, &args![time, entity + 0xf0]);
            }
        }
        e.with_stack(0x10, |e, step| {
            store_step_info(e, step.addr(), info);
            e.call(
                RIGID_MOTION_UTIL_STEP,
                &args![step, entities, count, 0xe0u32],
            );
        });
        let collision_input = e.mem.u32(world + 0x78);
        e.call(
            ENTITY_BATCH_RECALC_AABB,
            &args![collision_input, entities, count],
        );
    }
    if flags & 2 != 0 {
        e.call(
            COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
            &args![entities, count, world],
        );
    }
    if flags & 4 != 0 {
        let collision_input = e.mem.u32(world + 0x78);
        fn_00cfa400(e, this, entities, count, collision_input, 0);
    } else if integrate {
        e.vcall(this.addr(), 0x20, &args![entities, count, world, 2u32]);
    }
    release_world_lock(e, world);
}

// Translated from 00cfa5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::collideEntitiesDiscrete` (Xbox PDB), `(this, entities, count, world,
/// step_info, word)`, `RET 0x14`. Copies the first 0x14 words of the world's collision input
/// (+0x78) to a local, clears the local's word at +0x64, lets `00cfa7d0` prepare it from the
/// original, copies the four words of `step_info` into the local's `+0x50` .. `+0x5c`, locks
/// the world, runs `collideEntitiesBroadPhaseDiscrete(entities, count, world)` and the unnamed
/// `00cfa400(entities, count, local, word)`, unlocks the world and, when the local's word at
/// +0x64 is set, returns that block (0x200 bytes) to the allocator (slot 8).
pub fn hkp_simulation_collide_entities_discrete(
    e: &mut Engine,
    this: Ptr,
    entities: u32,
    count: i32,
    world: u32,
    step_info: Ptr,
    word: u32,
) {
    let collision_input = e.mem.u32(world + 0x78);
    e.with_stack(0x80, |e, local| {
        let copy = local.addr();
        for index in 0..0x14u32 {
            let value = e.mem.u32(collision_input + 4 * index);
            e.mem.set_u32(copy + 4 * index, value);
        }
        e.mem.set_u32(copy + 0x64, 0);
        e.call(COLLISION_INPUT_PREPARE_COPY, &args![local, collision_input]);
        for index in 0..4u32 {
            let value = e.mem.u32(step_info.addr() + 4 * index);
            e.mem.set_u32(copy + 0x50 + 4 * index, value);
        }
        e.mem
            .set_u32(world + 0x94, e.mem.u32(world + 0x94).wrapping_add(1));
        e.call(
            COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
            &args![entities, count, world],
        );
        fn_00cfa400(e, this, entities, count, copy, word);
        release_world_lock(e, world);
        let block = e.mem.u32(copy + 0x64);
        if block != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![block, 0x200u32]);
        }
    });
}

// Translated from 00cfa080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimulation::collideInternal` (Xbox PDB), `(this, step_info)`, `RET 4`. Between profile
/// records (`010cc64c` and the end label) copies the step info into the world's `+0x1d0` and the
/// collision input's `+0x50` (the word at `world + 0x78`), stores `world[+0x300] * delta`
/// at `world + 0x2ec` and `(float)(int world[+0x2f4]) * inverse_delta` at `world + 0x2f0`, locks
/// the world (+0x94) and runs `collideEntitiesBroadPhaseDiscrete(island[+0x48], island[+0x4c],
/// world)` for every island of the array at world `+0x28`; when the global at `01269200`
/// becomes 1 after one of them it unlocks, ends the profile record and returns. Otherwise it
/// unlocks, locks again, and for every island runs the narrow phase (between the records
/// `010cc54c` and the end label): `collision_input[+0x60] = collision_input[0] + 0x1c60`,
/// `collision_input[+0x14] = 0`, `00cff3c0(island, island + 0x58, collision_input)` (cdecl);
/// stops after an island once the global is 1, and fires `00d01650(world, island, step_info)`
/// (record `010cc634`) when the world's word at +0x198 is set. Finally unlocks the world.
pub fn hkp_simulation_collide_internal(e: &mut Engine, this: Ptr, step_info: Ptr) {
    let base = this.addr();
    let step = step_info.addr();
    profile_record(e, PROFILE_COLLIDE);
    let world = e.mem.u32(base + 0xc);
    store_step_info(e, world + 0x1d0, read_step_info(e, step));
    let collision_input = e.mem.u32(e.mem.u32(base + 0xc) + 0x78);
    store_step_info(e, collision_input + 0x50, read_step_info(e, step));
    let world = e.mem.u32(base + 0xc);
    let scale = (f64::from(x87_quiet(e.mem.f32(world + 0x300)))
        * f64::from(x87_quiet(e.mem.f32(step + 8)))) as f32;
    e.mem.set_f32(world + 0x2ec, scale);
    let count_scale =
        (f64::from(e.mem.i32(world + 0x2f4)) * f64::from(x87_quiet(e.mem.f32(step + 0xc)))) as f32;
    e.mem.set_f32(world + 0x2f0, count_scale);
    let world = e.mem.u32(base + 0xc);
    e.mem
        .set_u32(world + 0x94, e.mem.u32(world + 0x94).wrapping_add(1));
    let world = e.mem.u32(base + 0xc);
    let islands = world + 0x28;
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(islands + 4) {
        let island = e.mem.u32(e.mem.u32(islands) + 4 * index);
        let world = e.mem.u32(base + 0xc);
        let (entities, count) = (e.mem.u32(island + 0x48), e.mem.u32(island + 0x4c));
        e.call(
            COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
            &args![entities, count, world],
        );
        if e.global::<u32>(ABORT_FLAG) == 1 {
            // The world is being torn down: unlock and end the record.
            let world = e.mem.u32(base + 0xc);
            release_world_lock(e, world);
            profile_record(e, PROFILE_END);
            return;
        }
        index += 1;
    }
    let world = e.mem.u32(base + 0xc);
    release_world_lock(e, world);
    let world = e.mem.u32(base + 0xc);
    e.mem
        .set_u32(world + 0x94, e.mem.u32(world + 0x94).wrapping_add(1));
    let world = e.mem.u32(base + 0xc);
    let islands = world + 0x28;
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(islands + 4) {
        let island = e.mem.u32(e.mem.u32(islands) + 4 * index);
        let collision_input = e.mem.u32(e.mem.u32(base + 0xc) + 0x78);
        profile_record(e, PROFILE_NARROW_PHASE);
        let agent_pool = e.mem.u32(collision_input) + 0x1c60;
        e.mem.set_u32(collision_input + 0x60, agent_pool);
        e.mem.set_u32(collision_input + 0x14, 0);
        e.call(
            PROCESS_TRACK,
            &args![island, island + 0x58, collision_input],
        );
        profile_record(e, PROFILE_END);
        if e.global::<u32>(ABORT_FLAG) == 1 {
            break;
        }
        let world = e.mem.u32(base + 0xc);
        if e.mem.u32(world + 0x198) != 0 {
            profile_record(e, PROFILE_ISLAND_POST_COLLIDE);
            let world = e.mem.u32(base + 0xc);
            e.call(
                FIRE_ISLAND_POST_COLLIDE_CALLBACK,
                &args![world, island, step],
            );
            profile_record(e, PROFILE_END);
        }
        index += 1;
    }
    let world = e.mem.u32(base + 0xc);
    release_world_lock(e, world);
    profile_record(e, PROFILE_END);
}

/// `hkMapBase<unsigned long, unsigned long>::reserve(this = map; allocator, count)`.
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `00cb1870(this = map; allocator, key, value)`, the map's insert.
const MAP_INSERT: u32 = 0x00cb_1870;
/// `findKey(this = map; key)`: the slot of the key, or the map's `+8` word plus one.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// `clearAndDeallocate(this = map; allocator)`.
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// The map's destructor, `00a29680(this = map)`.
const MAP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkpLinkedCollidable::getCollisionEntriesSorted(this = collidable; array)`.
const GET_COLLISION_ENTRIES_SORTED: u32 = 0x00d0_8260;
/// `hkAgentNnMachine_InvalidateTimInAgent(entry, info)` (cdecl).
const INVALIDATE_TIM_IN_AGENT: u32 = 0x00cf_dbe0;
/// `00ce0c40(value)`, cdecl, the arc cosine in `ST0`.
const ARC_COSINE: u32 = 0x00ce_0c40;
/// `hkQuaternion::setAxisAngle(this = quaternion; axis, angle)`.
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// `hkQuaternion::setMul(this = result; first, second)`.
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// `00c74b70(this = result; first, second)`, the same shape as `setMul`.
const QUATERNION_SET_MUL_INVERSE: u32 = 0x00c7_4b70;
/// `00cb2d90(this = transform; quaternion)`, which writes the rotation rows of a transform.
const TRANSFORM_SET_ROTATION: u32 = 0x00cb_2d90;
/// The float copied into the output block's `+0x3030` word (`0x7f7fffee`).
const OUTPUT_INITIAL_FLOAT: u32 = 0x0104_da48;
/// The smallest rotation axis length that still gets swept (float).
const MIN_AXIS_LENGTH: u32 = 0x0107_18c0;
/// The largest cosine that still gets swept (double, `0.9999`).
const MAX_SWEEP_COSINE: u32 = 0x010c_c5d0;
/// The factor on the time step during the sweep (double, `20.0`).
const SWEEP_TIME_SCALE: u32 = 0x0102_fc70;
/// The factor on the scaled time step that is taken off the sweep distance (double, `0.05`).
const SWEEP_DISTANCE_SCALE: u32 = 0x0108_2cb0;

/// The four values every call of the collision callback needs.
#[derive(Clone, Copy)]
struct AgentPass {
    callback: u32,
    entry: u32,
    info: u32,
    /// The output block (the callback's third argument).
    output: u32,
}

impl AgentPass {
    /// `hkAgentNnMachine_InvalidateTimInAgent(entry, info)`, then `callback(entry, info, output)`.
    fn collide(self, e: &mut Engine) {
        e.call(INVALIDATE_TIM_IN_AGENT, &args![self.entry, self.info]);
        e.call(self.callback, &args![self.entry, self.info, self.output]);
    }
}

/// Puts the body's transform translation back at `body[+0x140] - r * rotation`, with `r` the
/// body's `+0x170` vector and the rotation the three rows at `body + 0xf0`.
fn follow_body_rotation(e: &mut Engine, body: u32) {
    let transform = body + 0xf0;
    let offset = load_vec(e, body + 0x170);
    let row_y = load_vec(e, transform + 0x10);
    let row_x = load_vec(e, transform);
    let row_z = load_vec(e, transform + 0x20);
    let swept = add(
        add(mul(splat(offset[1]), row_y), mul(splat(offset[0]), row_x)),
        mul(splat(offset[2]), row_z),
    );
    let position = sub(load_vec(e, body + 0x140), swept);
    store_vec(e, transform + 0x30, position);
}

/// The time-of-impact sweep of `00cf9480`, which runs for a body when the callback left contact
/// points in the output block and the mode is 1. `frame` is the routine's stack frame: the
/// locals the exe leaves there (the quaternions at `+0x70`, `+0xd0`, `+0xe0`, `+0x100` and
/// `+0x130`, the axes at `+0xb0` and `+0x110`, the saved transform at `+0x140`) are passed to
/// the quaternion routines by address, so they live at the same offsets here.
///
/// The exe also copies the body's transform into stack temporaries at `+0x80 .. +0xcf`; those
/// are never read again, so they are not reproduced.
fn sweep_time_of_impact(e: &mut Engine, pass: AgentPass, body: u32, frame: u32) {
    let output = pass.output;
    let info = pass.info;
    let transform = body + 0xf0;
    let old_rotation = frame + 0x70;
    for word in 0..4 {
        let value = e.mem.u32(body + 0x160 + 4 * word);
        e.mem.set_u32(old_rotation + 4 * word, value);
    }
    for part in 0..4 {
        copy_vec(e, transform + 0x10 * part, frame + 0x140 + 0x10 * part);
    }
    // The contact normal, turned around when the entry's first collidable is not the body's.
    let mut normal = load_vec(e, output + 0x20);
    if e.mem.u32(pass.entry + 0x10) != body + 0x10 {
        let signs = load_bits(e, SIGN_MASK);
        normal = std::array::from_fn(|lane| f32::from_bits(normal[lane].to_bits() ^ signs[lane]));
    }
    let contact = load_vec(e, output + 0x10);
    let gap = sub(load_vec(e, body + 0x140), contact);
    let swept_axis = cross(normal, gap);
    store_vec(e, frame + 0xb0, swept_axis);
    let axis_length = sum3(mul(swept_axis, swept_axis)).sqrt();
    let along = sum3(mul(gap, normal));
    let distance = sum3(mul(gap, gap)).sqrt();
    let cosine = (f64::from(along) / f64::from(distance)) as f32;
    let smallest_axis: f32 = e.global(MIN_AXIS_LENGTH);
    let largest_cosine = e.mem.f64(MAX_SWEEP_COSINE);
    // The exe leaves the sweep, with the x87 stack emptied, unless `smallest_axis < axis_length`
    // and `cosine < 0.9999` (an unordered compare leaves it too).
    if smallest_axis < axis_length && f64::from(cosine) < largest_cosine {
        let step = x87_quiet(e.mem.f32(info + 8));
        let saved_step_bits = x87_quiet_bits(e.mem.u32(info + 8));
        let scaled = (f64::from(step) * e.mem.f64(SWEEP_TIME_SCALE)) as f32;
        e.mem.set_f32(info + 8, scaled);
        let limits = e.mem.u32(info + 0x60);
        let saved_first_bits = x87_quiet_bits(e.mem.u32(limits + 4));
        let saved_second_bits = x87_quiet_bits(e.mem.u32(limits + 8));
        e.mem.set_f32(limits + 4, scaled);
        let limits = e.mem.u32(info + 0x60);
        e.mem.set_f32(limits + 8, scaled);
        // `(along - scaled * 0.05) / distance`, kept at full precision until it is stored,
        // and replaced by zero unless it is above zero.
        let target = ((f64::from(along) - f64::from(scaled) * e.mem.f64(SWEEP_DISTANCE_SCALE))
            / f64::from(distance)) as f32;
        let target = if target > 0.0 { target } else { 0.0 };
        let first_angle = e.call(ARC_COSINE, &args![cosine]).st0 as f32;
        let second_angle = e.call(ARC_COSINE, &args![target]).st0;
        let angle = (f64::from(first_angle) - second_angle) as f32;
        let axis = splat((1.0 / f64::from(axis_length)) as f32);
        let axis = mul(axis, swept_axis);
        store_vec(e, frame + 0xb0, axis);
        // First rotation: about the swept axis.
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + 0x100, frame + 0xb0, angle],
        );
        e.call(
            QUATERNION_SET_MUL,
            &args![frame + 0x130, frame + 0x100, body + 0x160],
        );
        e.call(TRANSFORM_SET_ROTATION, &args![transform, frame + 0x130]);
        follow_body_rotation(e, body);
        pass.collide(e);
        // Second rotation: about the axis perpendicular to the swept axis and the normal.
        let across = cross(axis, normal);
        let scale = inverse_length(e, sum3(mul(across, across)));
        let across = mul(splat(scale), across);
        store_vec(e, frame + 0x110, across);
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + 0xe0, frame + 0x110, angle],
        );
        e.call(
            QUATERNION_SET_MUL,
            &args![frame + 0xd0, frame + 0xe0, old_rotation],
        );
        e.call(TRANSFORM_SET_ROTATION, &args![transform, frame + 0xd0]);
        follow_body_rotation(e, body);
        pass.collide(e);
        // Third: the second rotation combined the other way.
        e.call(
            QUATERNION_SET_MUL_INVERSE,
            &args![frame + 0xd0, frame + 0xe0, old_rotation],
        );
        e.call(TRANSFORM_SET_ROTATION, &args![transform, frame + 0xd0]);
        follow_body_rotation(e, body);
        pass.collide(e);
        // Put the step, the two limits and the body's transform back.
        e.mem.set_u32(info + 8, saved_step_bits);
        let limits = e.mem.u32(info + 0x60);
        e.mem.set_u32(limits + 4, saved_first_bits);
        let limits = e.mem.u32(info + 0x60);
        e.mem.set_u32(limits + 8, saved_second_bits);
        e.vcall(body + 0xe0, 0x3c, &args![frame + 0x140]);
        pass.collide(e);
    }
}

// Translated from 00cf9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf9480`, unnamed (the `hkpSimulation` collision pass over a set of bodies, called
/// with the callbacks `00cf92c0` and `00cf87a0`). Stack (`RET 0x14`): `bodies` (an array of
/// rigid body pointers), `count`, `info`, `callback` (an address called with `(entry, info,
/// output)`, callee-cleaned) and `mode`. `ECX` is not an input: the routine hands whatever it
/// holds on to the callback, and both callbacks ignore it, so the callback is called with its
/// three stack words only.
///
/// A map keyed by collidable keeps the bodies already seen. For every body it adds the body's
/// collidable (`body + 0x10`) to the map, then walks the collidable's sorted collision entries
/// (`00d08260`) and, for each entry whose other collidable is not in the map yet (`findKey` above
/// the map's `+8` word), stores the owner's `+0xcc` word at `output[+4]` (the owner is the
/// entry's first collidable's, or the second's when the first's `+0xe8` byte is 5), invalidates
/// the entry's time of impact (`00cfdbe0`) and calls the callback with the output block.
/// With `mode == 1` and contact points in the output (`output[0] != output + 0x10`), the
/// time-of-impact sweep runs: the step in `info[+8]` and the two limits at `info[+0x60] + 4`
/// and `+8` are set to twenty times the step, the body is rotated about the axis of
/// `normal x (body[+0x140] - contact)` by the difference of two arc cosines, rotated again about
/// the axis perpendicular to it, and rotated back, with the callback called after each, before
/// the step, the limits and the transform are restored (the transform through virtual slot
/// `0x3c` of `body + 0xe0`). A global abort flag of 1 stops the pass after any entry that was
/// processed. Last it frees the entry array and the map.
///
/// The output block (stack from `+0x180`) is initialised only as far as
/// `output[+4]`, `+0x3030` (the word at `0x0104da48`), `+0x3050` and `+0x3054` (zero); the
/// callback fills in the rest, including `output[0]`.
pub fn fn_00cf9480(e: &mut Engine, bodies: u32, count: i32, info: u32, callback: u32, mode: u32) {
    e.with_stack(0x3220, |e, frame| {
        let base = frame.addr();
        let map = base + 0x3c;
        let list = base + 0x24;
        let output = base + 0x180;
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, u32::MAX);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_RESERVE, &args![map, allocator, count as u32]);
        let bias_bits = x87_quiet_bits(e.mem.u32(OUTPUT_INITIAL_FLOAT));
        e.mem.set_u32(base + 0x31b0, bias_bits);
        e.mem.set_u32(output + 4, 0);
        e.mem.set_u32(base + 0x31d0, 0);
        e.mem.set_u32(base + 0x31d4, 0);
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        'bodies: for index in 0..count.max(0) as u32 {
            let body = e.mem.u32(bodies + 4 * index);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_INSERT, &args![map, allocator, body + 0x10, 0u32]);
            e.call(GET_COLLISION_ENTRIES_SORTED, &args![body + 0x10, list]);
            let mut position = 0u32;
            while (position as i32) < e.mem.i32(list + 4) {
                let slot = e.mem.u32(list) + position * 8;
                position += 1;
                let key = e.mem.u32(slot + 4);
                let found = e.call(MAP_FIND_KEY, &args![map, key]).i32();
                if found <= e.mem.i32(map + 8) {
                    continue;
                }
                let entry = e.mem.u32(slot);
                let first = e.mem.u32(entry + 0x10);
                let owner = first.wrapping_add(i32::from(e.mem.i8(first + 0x10)) as u32);
                let second = e.mem.u32(entry + 0x14);
                let value = if e.mem.u8(owner + 0xe8) == 5 {
                    let other = second.wrapping_add(i32::from(e.mem.i8(second + 0x10)) as u32);
                    e.mem.u32(other + 0xcc)
                } else {
                    e.mem.u32(owner + 0xcc)
                };
                e.mem.set_u32(output + 4, value);
                let pass = AgentPass {
                    callback,
                    entry,
                    info,
                    output,
                };
                e.call(INVALIDATE_TIM_IN_AGENT, &args![entry, info]);
                e.call(callback, &args![entry, info, output]);
                if mode == 1 && e.mem.u32(output) != output + 0x10 {
                    sweep_time_of_impact(e, pass, body, base);
                }
                if e.global::<u32>(ABORT_FLAG) == 1 {
                    break 'bodies;
                }
            }
        }
        // The entry array and the map are released on every path.
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(list + 4, 0);
        let capacity = e.mem.u32(list + 8);
        if (capacity as i32) >= 0 {
            let size = (capacity & 0x3fff_ffff).wrapping_mul(8);
            let block = e.mem.u32(list);
            e.vcall(allocator, 0x10, &args![block, size]);
        }
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(MAP_DESTRUCTOR, &args![map]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_8730, hkp_simulation_step_delta_time(Ptr, f32) -> u32),
        entry!(0x00cf_87c0, hkp_simulation_construct(Ptr, u32) -> Ptr),
        entry!(0x00cf_8820, hkp_simulation_re_collide_after_step_failure(Ptr) -> u32),
        entry!(
            0x00cf_89c0,
            hkp_simulation_snap_simulate_time_and_get_time_to_advance_to(Ptr) -> f32
        ),
        entry!(0x00cf_8a40, hkp_simulation_advance_time(Ptr) -> u32),
        entry!(0x00cf_8b00, fn_00cf8b00(Ptr)),
        entry!(0x00cf_9340, hkp_simulation_integrate(Ptr, f32) -> u32),
        entry!(0x00cf_9480, fn_00cf9480(u32, i32, u32, u32, u32)),
        entry!(0x00cf_a080, hkp_simulation_collide_internal(Ptr, Ptr)),
        entry!(0x00cf_a400, fn_00cfa400(Ptr, u32, i32, u32, u32)),
        entry!(
            0x00cf_a430,
            hkp_simulation_reset_collision_information_for_entities(Ptr, u32, i32, u32, u8)
        ),
        entry!(
            0x00cf_a470,
            hkp_simulation_reintegrate_and_recollide_entities(Ptr, u32, i32, u32, u32)
        ),
        entry!(
            0x00cf_a5f0,
            hkp_simulation_collide_entities_discrete(Ptr, u32, i32, u32, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{
        HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE,
    };
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The double address standing for virtual slot `slot` (a byte offset) of a test simulation.
    const SLOT_DOUBLES: u32 = 0x0900_1000;
    /// The unnamed collision routine `00cf9480` and its callbacks.
    const COLLIDE_ROUTINE: u32 = 0x00cf_9480;
    const COLLIDE_CALLBACK: u32 = 0x00cf_92c0;
    const RESET_CALLBACK: u32 = 0x00cf_87a0;
    const CLEAR_COLLISION_STATE: u32 = 0x00d1_a300;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A profile stream with room for two records.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        stream
    }

    /// The arguments of every call to `addr` logged so far.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addr: u32) {
        e.register_double(addr, |_, _| Ret::default());
    }

    /// A simulation on `world` whose virtual slots are the doubles at `SLOT_DOUBLES + slot`
    /// (none registered yet).
    fn simulation(e: &mut Engine, world: u32) -> u32 {
        let table = 0x0900_0f00;
        let slots: Vec<u32> = (0..16).map(|slot| SLOT_DOUBLES + 4 * slot).collect();
        e.put_vtable(table, &slots);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 0xc, world);
        this
    }

    fn step_words(e: &Engine, at: u32) -> Vec<f32> {
        (0..4).map(|word| e.mem.f32(at + 4 * word)).collect()
    }

    fn step_bits(values: [f32; 4]) -> Vec<u32> {
        values.map(f32::to_bits).to_vec()
    }

    fn words_at(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    /// Doubles for the world's lock release: nothing is pending.
    fn unlock_doubles(e: &mut Engine) {
        noop(e, WORLD_UNLOCK_FIRST);
        noop(e, WORLD_UNLOCK_SECOND);
    }

    #[test]
    fn step_delta_time_runs_every_stage_from_idle() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let this = simulation(&mut e, world);
        for slot in [0x10, 0x14, 0x18] {
            noop(&mut e, SLOT_DOUBLES + slot);
        }
        let result = e.call(0x00cf8730, &args![this, 0.25f32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls(&e, SLOT_DOUBLES + 0x10),
            vec![vec![this, 0.25f32.to_bits()]]
        );
        assert_eq!(calls(&e, SLOT_DOUBLES + 0x14), vec![vec![this]]);
        assert_eq!(calls(&e, SLOT_DOUBLES + 0x18), vec![vec![this]]);
        assert!(calls(&e, WORLD_GET_MEMORY_WATCH_DOG).is_empty());
    }

    #[test]
    fn step_delta_time_stops_at_the_stage_the_first_stage_leaves() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let this = simulation(&mut e, world);
        e.register_double(SLOT_DOUBLES + 0x10, move |e, _| {
            e.mem.set_u32(this + 0x28, 2);
            Ret::default()
        });
        noop(&mut e, SLOT_DOUBLES + 0x14);
        e.register_double(WORLD_GET_MEMORY_WATCH_DOG, |_, _| ret(1));
        noop(&mut e, TRY_TO_RECOVER_FROM_MEMORY_ERRORS);
        let result = e.call(0x00cf8730, &args![this, 1.0f32]).u32();
        assert_eq!(result, 2);
        assert_eq!(calls(&e, SLOT_DOUBLES + 0x14).len(), 1);
        assert!(calls(&e, SLOT_DOUBLES + 0x18).is_empty());
        assert_eq!(calls(&e, WORLD_GET_MEMORY_WATCH_DOG), vec![vec![world]]);
        assert_eq!(
            calls(&e, TRY_TO_RECOVER_FROM_MEMORY_ERRORS),
            vec![vec![world]]
        );
    }

    #[test]
    fn construct_fills_the_header_and_quiets_the_times() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.set_global(NO_SNAP_TIME, (-1.0f32).to_bits());
        e.set_global(SNAP_TOLERANCE, 0x7fa0_0000u32);
        let result = e.call(0x00cf87c0, &args![this, 0x1234u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x20), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(this + 0x24), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this), 0x010c_c4f4);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    /// A simulation on a world with a post-collide callback pending, a double for slot `0x34`
    /// that logs the step info it is given, and one for the callback that logs the world and the
    /// step info.
    fn re_collide_setup(e: &mut Engine, stage: u32) -> (u32, u32, EventLog) {
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x180, 1);
        let this = simulation(e, world);
        e.mem.set_f32(this + 0x14, 1.0);
        e.mem.set_f32(this + 0x18, 2.0);
        e.mem.set_f32(this + 0x1c, 0.5);
        e.mem.set_u32(this + 0x28, stage);
        e.mem.set_u8(this + 0x10, 0);
        noop(e, UPDATE_COLLISION_FILTER_ON_WORLD);
        let steps: EventLog = Rc::new(RefCell::new(Vec::new()));
        let slot_log = Rc::clone(&steps);
        e.register_double(SLOT_DOUBLES + 0x34, move |e, a| {
            slot_log.borrow_mut().push(words_at(e, a[1], 4));
            Ret::default()
        });
        let callback_log = Rc::clone(&steps);
        e.register_double(FIRE_POST_COLLIDE_CALLBACK, move |e, a| {
            let mut entry = vec![a[0]];
            entry.extend(words_at(e, a[1], 4));
            callback_log.borrow_mut().push(entry);
            Ret::default()
        });
        (world, this, steps)
    }

    #[test]
    fn re_collide_from_stage_three_uses_the_two_times() {
        let mut e = engine();
        profile_stream(&mut e);
        let (world, this, steps) = re_collide_setup(&mut e, 3);
        let result = e.call(0x00cf8820, &args![this]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls(&e, UPDATE_COLLISION_FILTER_ON_WORLD),
            vec![vec![world, 0, 1]]
        );
        assert_eq!(*steps.borrow(), vec![step_bits([1.0, 2.0, 1.0, 1.0])]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.f32(this + 0x18), 2.0);
    }

    #[test]
    fn re_collide_from_stage_two_advances_the_time_and_fires_the_callback() {
        let mut e = engine();
        profile_stream(&mut e);
        let (world, this, steps) = re_collide_setup(&mut e, 2);
        let result = e.call(0x00cf8820, &args![this]).u32();
        assert_eq!(result, 0);
        let expected = step_bits([2.0, 2.5, 0.5, 2.0]);
        let mut callback = vec![world];
        callback.extend(&expected);
        assert_eq!(*steps.borrow(), vec![expected, callback]);
        assert_eq!(e.mem.f32(this + 0x18), 2.5);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn re_collide_gives_up_when_the_abort_flag_is_set() {
        let mut e = engine();
        profile_stream(&mut e);
        let (_, this, steps) = re_collide_setup(&mut e, 2);
        e.set_global(ABORT_FLAG, 1u32);
        let result = e.call(0x00cf8820, &args![this]).u32();
        assert_eq!(result, 2);
        assert_eq!(steps.borrow().len(), 1);
        assert_eq!(e.mem.f32(this + 0x18), 2.0);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    fn snap_time(snap: f32, time: f32) -> (f32, f32) {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.set_global(NO_SNAP_TIME, (-1.0f32).to_bits());
        e.mem.set_f32(this + 0x18, time);
        e.mem.set_f32(this + 0x20, snap);
        e.mem.set_f32(this + 0x24, 1e-4);
        let answer = e.call(0x00cf89c0, &args![this]).f32();
        (answer, e.mem.f32(this + 0x20))
    }

    #[test]
    fn snap_time_picks_the_earlier_of_the_two_times() {
        // No snap time set: the simulation time.
        assert_eq!(snap_time(-1.0, 3.0), (3.0, -1.0));
        // A snap time well away from the simulation time: the earlier one.
        assert_eq!(snap_time(5.0, 3.0), (3.0, 5.0));
        assert_eq!(snap_time(2.0, 3.0), (2.0, 2.0));
        // Within the tolerance the snap time becomes the simulation time.
        assert_eq!(snap_time(3.00005, 3.0), (3.0, 3.0));
        // A NaN snap time differs from the marker and is never snapped.
        let (answer, stored) = snap_time(f32::NAN, 3.0);
        assert!(answer.is_nan());
        assert!(stored.is_nan());
    }

    #[test]
    fn advance_time_fires_the_callback_when_the_snap_time_is_not_beyond() {
        let mut e = engine();
        profile_stream(&mut e);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x168, 1);
        let this = simulation(&mut e, world);
        e.set_global(NO_SNAP_TIME, (-1.0f32).to_bits());
        e.mem.set_f32(this + 0x18, 1.0);
        e.mem.set_f32(this + 0x20, -1.0);
        e.mem.set_f32(this + 0x24, 1e-4);
        e.mem.set_u32(this + 0x28, 3);
        noop(&mut e, FIRE_POST_SIMULATION_CALLBACK);
        assert_eq!(e.call(0x00cf8a40, &args![this]).u32(), 0);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(calls(&e, FIRE_POST_SIMULATION_CALLBACK), vec![vec![world]]);
        // A snap time later than the simulation time suppresses the callback.
        e.mem.set_f32(this + 0x20, 5.0);
        assert_eq!(e.call(0x00cf8a40, &args![this]).u32(), 0);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(calls(&e, FIRE_POST_SIMULATION_CALLBACK).len(), 1);
    }

    #[test]
    fn island_entities_are_each_asked_with_the_world_step_info() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let this = simulation(&mut e, world);
        let entity_table = 0x0900_2000;
        e.put_vtable(entity_table, &[0, 0, 0, 0x0900_2100]);
        noop(&mut e, 0x0900_2100);
        let entities: Vec<u32> = (0..3)
            .map(|_| {
                let entity = e.mem.alloc(0x10);
                e.mem.set_u32(entity, entity_table);
                entity
            })
            .collect();
        let islands: Vec<u32> = [&entities[..2], &entities[2..]]
            .iter()
            .map(|members| {
                let island = e.mem.alloc(0x50);
                let data = e.mem.alloc(0x10);
                for (index, member) in members.iter().enumerate() {
                    e.mem.set_u32(data + 4 * index as u32, *member);
                }
                e.mem.set_u32(island + 0x38, data);
                e.mem.set_u32(island + 0x3c, members.len() as u32);
                island
            })
            .collect();
        let island_data = e.mem.alloc(0x10);
        e.mem.set_u32(island_data, islands[0]);
        e.mem.set_u32(island_data + 4, islands[1]);
        e.mem.set_u32(world + 0x28, island_data);
        e.mem.set_u32(world + 0x2c, 2);
        e.call(0x00cf8b00, &args![this]);
        let expected: Vec<Vec<u32>> = entities
            .iter()
            .map(|entity| vec![*entity, world + 0x1d0])
            .collect();
        assert_eq!(calls(&e, 0x0900_2100), expected);
    }

    #[test]
    fn integrate_publishes_the_step_info_and_records_the_stage() {
        for (stage, flag_after) in [(0u32, 0u8), (4, 1)] {
            let mut e = engine();
            profile_stream(&mut e);
            let world = e.mem.alloc(0x400);
            let this = simulation(&mut e, world);
            let collision_input = e.mem.alloc(0x80);
            e.mem.set_u32(world + 0x78, collision_input);
            let object = e.mem.alloc(0x10);
            let object_table = 0x0900_2200;
            e.put_vtable(object_table, &[0, 0, 0, 0, 0x0900_2300]);
            e.mem.set_u32(object, object_table);
            e.mem.set_u32(world + 0x4c, object);
            let seen: EventLog = Rc::new(RefCell::new(Vec::new()));
            let slot_log = Rc::clone(&seen);
            e.register_double(0x0900_2300, move |e, a| {
                let mut entry = vec![a[0], a[1]];
                entry.extend(words_at(e, a[2], 4));
                slot_log.borrow_mut().push(entry);
                Ret::default()
            });
            let integrate_log = Rc::clone(&seen);
            e.register_double(INTEGRATE_INTERNAL, move |e, a| {
                let mut entry = vec![a[0]];
                entry.extend(words_at(e, a[1], 4));
                integrate_log.borrow_mut().push(entry);
                ret(stage)
            });
            e.mem.set_f32(this + 0x18, 1.0);
            e.mem.set_u8(this + 0x10, 1);
            let result = e.call(0x00cf9340, &args![this, 0.5f32]).u32();
            assert_eq!(result, stage);
            let info = step_bits([1.0, 1.5, 0.5, 2.0]);
            let mut slot_entry = vec![object, world];
            slot_entry.extend(&info);
            let mut integrate_entry = vec![this];
            integrate_entry.extend(&info);
            assert_eq!(*seen.borrow(), vec![slot_entry, integrate_entry]);
            assert_eq!(step_words(&e, world + 0x1d0), vec![1.0, 1.5, 0.5, 2.0]);
            assert_eq!(
                step_words(&e, collision_input + 0x50),
                vec![1.0, 1.5, 0.5, 2.0]
            );
            assert_eq!(e.mem.f32(this + 0x1c), 0.5);
            assert_eq!(e.mem.u32(this + 0x28), stage);
            // The flag byte is cleared only when the stage came back as zero.
            assert_eq!(e.mem.u8(this + 0x10), flag_after);
        }
    }

    #[test]
    fn the_collide_wrapper_passes_the_callback_and_the_extra_word() {
        let mut e = engine();
        noop(&mut e, COLLIDE_ROUTINE);
        e.call(0x00cfa400, &args![0u32, 0x100u32, 3i32, 0x200u32, 7u32]);
        assert_eq!(
            calls(&e, COLLIDE_ROUTINE),
            vec![vec![0x100, 3, 0x200, COLLIDE_CALLBACK, 7]]
        );
    }

    #[test]
    fn reset_collision_information_follows_the_flag_bits() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, 0x300);
        noop(&mut e, COLLIDE_ROUTINE);
        noop(&mut e, CLEAR_COLLISION_STATE);
        let entities = 0x100u32;
        e.call(0x00cfa430, &args![0u32, entities, 2i32, world, 6u8]);
        assert_eq!(
            calls(&e, COLLIDE_ROUTINE),
            vec![vec![entities, 2, 0x300, RESET_CALLBACK, 0]]
        );
        assert_eq!(calls(&e, CLEAR_COLLISION_STATE), vec![vec![entities, 2]]);
        e.call(0x00cfa430, &args![0u32, entities, 2i32, world, 1u8]);
        assert_eq!(calls(&e, COLLIDE_ROUTINE).len(), 1);
        assert_eq!(calls(&e, CLEAR_COLLISION_STATE).len(), 1);
    }

    /// The doubles `reintegrateAndRecollideEntities` calls; time 1.0, PSI time 2.0.
    fn reintegrate_doubles(e: &mut Engine) {
        e.register_double(WORLD_GET_CURRENT_TIME, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(WORLD_GET_CURRENT_PSI_TIME, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        noop(e, BACK_STEP_MOTION_STATE);
        noop(e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE);
        noop(e, ENTITY_BATCH_RECALC_AABB);
        noop(e, COLLIDE_ROUTINE);
        unlock_doubles(e);
    }

    #[test]
    fn reintegrate_runs_every_stage_the_flags_ask_for() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x78, collision_input);
        let this = simulation(&mut e, world);
        reintegrate_doubles(&mut e);
        let motion_steps: EventLog = Rc::new(RefCell::new(Vec::new()));
        let motion_log = Rc::clone(&motion_steps);
        e.register_double(RIGID_MOTION_UTIL_STEP, move |e, a| {
            let mut entry = a[1..].to_vec();
            entry.extend(words_at(e, a[0], 4));
            motion_log.borrow_mut().push(entry);
            Ret::default()
        });
        // Entity 0 is the kind with state 5 (skipped by the back step).
        let entity_array = e.mem.alloc(0x10);
        let entities: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x200)).collect();
        e.mem.set_u8(entities[0] + 0xe8, 5);
        e.mem.set_u32(entity_array, entities[0]);
        e.mem.set_u32(entity_array + 4, entities[1]);
        // The pending-work words make the unlock run both helpers.
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        e.call(0x00cfa470, &args![this, entity_array, 2i32, world, 7u32]);
        assert_eq!(
            calls(&e, BACK_STEP_MOTION_STATE),
            vec![vec![1.0f32.to_bits(), entities[1] + 0xf0]]
        );
        let mut motion = vec![entity_array, 2, 0xe0];
        motion.extend(step_bits([1.0, 2.0, 1.0, 1.0]));
        assert_eq!(*motion_steps.borrow(), vec![motion]);
        assert_eq!(
            calls(&e, ENTITY_BATCH_RECALC_AABB),
            vec![vec![collision_input, entity_array, 2]]
        );
        assert_eq!(
            calls(&e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE),
            vec![vec![entity_array, 2, world]]
        );
        assert_eq!(
            calls(&e, COLLIDE_ROUTINE),
            vec![vec![entity_array, 2, collision_input, COLLIDE_CALLBACK, 0]]
        );
        assert_eq!(calls(&e, WORLD_UNLOCK_FIRST), vec![vec![world]]);
        assert_eq!(calls(&e, WORLD_UNLOCK_SECOND), vec![vec![world]]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
        assert_eq!(
            step_words(&e, collision_input + 0x50),
            vec![1.0, 2.0, 1.0, 1.0]
        );
    }

    #[test]
    fn reintegrate_without_the_collide_flag_asks_the_simulation_to_recollide() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x78, collision_input);
        let this = simulation(&mut e, world);
        reintegrate_doubles(&mut e);
        noop(&mut e, RIGID_MOTION_UTIL_STEP);
        noop(&mut e, SLOT_DOUBLES + 0x20);
        // A world that stays locked (lock count 2): no unlock helper runs.
        e.mem.set_u32(world + 0x94, 1);
        e.mem.set_u32(world + 0x8c, 1);
        let entity_array = e.mem.alloc(0x10);
        e.call(0x00cfa470, &args![this, entity_array, 0i32, world, 1u32]);
        assert_eq!(
            calls(&e, SLOT_DOUBLES + 0x20),
            vec![vec![this, entity_array, 0, world, 2]]
        );
        assert!(calls(&e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE).is_empty());
        assert!(calls(&e, COLLIDE_ROUTINE).is_empty());
        assert!(calls(&e, WORLD_UNLOCK_FIRST).is_empty());
        assert_eq!(e.mem.u32(world + 0x94), 1);
    }

    #[test]
    fn collide_discrete_works_on_a_copy_of_the_collision_input() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        let collision_input = e.mem.alloc(0x80);
        for word in 0..0x14u32 {
            e.mem.set_u32(collision_input + 4 * word, 0x1000 + word);
        }
        e.mem.set_u32(world + 0x78, collision_input);
        let this = simulation(&mut e, world);
        let block = 0x0123_4000u32;
        e.register_double(COLLISION_INPUT_PREPARE_COPY, move |e, a| {
            e.mem.set_u32(a[0] + 0x64, block);
            Ret::default()
        });
        noop(&mut e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE);
        unlock_doubles(&mut e);
        let seen: EventLog = Rc::new(RefCell::new(Vec::new()));
        let seen_log = Rc::clone(&seen);
        e.register_double(COLLIDE_ROUTINE, move |e, a| {
            let mut entry = a.to_vec();
            entry.push(e.mem.u32(a[2] + 0x4c));
            entry.extend(words_at(e, a[2] + 0x50, 4));
            entry.push(e.mem.u32(a[2] + 0x64));
            seen_log.borrow_mut().push(entry);
            Ret::default()
        });
        let freed: EventLog = Rc::new(RefCell::new(Vec::new()));
        let free_log = Rc::clone(&freed);
        let free_slot = 0x0900_2400;
        e.register_double(free_slot, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let allocator_table = 0x0900_2500;
        e.put_vtable(allocator_table, &[0, 0, free_slot]);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        let step = e.mem.alloc(0x10);
        for (word, value) in [1.0f32, 2.0, 1.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(step + 4 * word as u32, value);
        }
        e.call(0x00cfa5f0, &args![this, 0x500u32, 4i32, world, step, 9u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let entry = &seen[0];
        assert_eq!(&entry[..2], &[0x500, 4]);
        assert_ne!(entry[2], collision_input);
        assert_eq!(&entry[3..5], &[COLLIDE_CALLBACK, 9]);
        // The copy holds the first words of the original, then the new step info.
        assert_eq!(entry[5], 0x1000 + 0x13);
        assert_eq!(&entry[6..10], &step_bits([1.0, 2.0, 1.0, 1.0])[..]);
        assert_eq!(entry[10], block);
        // The original collision input is untouched; the copy's block is returned.
        assert_eq!(e.mem.u32(collision_input + 0x50), 0);
        assert_eq!(*freed.borrow(), vec![vec![allocator, block, 0x200]]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    /// A world with two islands (entity arrays at `+0x48` / `+0x4c`) and a collision input
    /// whose first word is `0x1000`. Returns the simulation, the step info, the world and the
    /// islands.
    fn collide_internal_world(e: &mut Engine) -> (u32, u32, u32, Vec<u32>) {
        let world = e.mem.alloc(0x400);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(collision_input, 0x1000);
        e.mem.set_u32(world + 0x78, collision_input);
        e.mem.set_f32(world + 0x300, 2.0);
        e.mem.set_u32(world + 0x2f4, 3);
        let islands: Vec<u32> = (0..2)
            .map(|index| {
                let island = e.mem.alloc(0x80);
                e.mem.set_u32(island + 0x48, 0x700 + index);
                e.mem.set_u32(island + 0x4c, 5 + index);
                island
            })
            .collect();
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, islands[0]);
        e.mem.set_u32(data + 4, islands[1]);
        e.mem.set_u32(world + 0x28, data);
        e.mem.set_u32(world + 0x2c, 2);
        let this = simulation(e, world);
        let step = e.mem.alloc(0x10);
        for (word, value) in [1.0f32, 2.0, 0.5, 2.0].into_iter().enumerate() {
            e.mem.set_f32(step + 4 * word as u32, value);
        }
        unlock_doubles(e);
        (this, step, world, islands)
    }

    #[test]
    fn collide_internal_runs_the_broad_phase_then_the_narrow_phase_per_island() {
        let mut e = engine();
        profile_stream(&mut e);
        let (this, step, world, islands) = collide_internal_world(&mut e);
        let collision_input = e.mem.u32(world + 0x78);
        e.mem.set_u32(world + 0x198, 1);
        e.mem.set_u32(collision_input + 0x14, 99);
        noop(&mut e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE);
        noop(&mut e, FIRE_ISLAND_POST_COLLIDE_CALLBACK);
        let tracks: EventLog = Rc::new(RefCell::new(Vec::new()));
        let track_log = Rc::clone(&tracks);
        e.register_double(PROCESS_TRACK, move |e, a| {
            let mut entry = a.to_vec();
            entry.extend([e.mem.u32(a[2] + 0x60), e.mem.u32(a[2] + 0x14)]);
            track_log.borrow_mut().push(entry);
            Ret::default()
        });
        e.call(0x00cfa080, &args![this, step]);
        assert_eq!(
            calls(&e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE),
            vec![vec![0x700, 5, world], vec![0x701, 6, world]]
        );
        let pool = 0x1000 + 0x1c60;
        assert_eq!(
            *tracks.borrow(),
            vec![
                vec![islands[0], islands[0] + 0x58, collision_input, pool, 0],
                vec![islands[1], islands[1] + 0x58, collision_input, pool, 0],
            ]
        );
        assert_eq!(
            calls(&e, FIRE_ISLAND_POST_COLLIDE_CALLBACK),
            vec![vec![world, islands[0], step], vec![world, islands[1], step]]
        );
        assert_eq!(step_words(&e, world + 0x1d0), vec![1.0, 2.0, 0.5, 2.0]);
        assert_eq!(
            step_words(&e, collision_input + 0x50),
            vec![1.0, 2.0, 0.5, 2.0]
        );
        assert_eq!(e.mem.f32(world + 0x2ec), 1.0);
        assert_eq!(e.mem.f32(world + 0x2f0), 6.0);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn collide_internal_stops_after_the_first_broad_phase_when_the_world_is_going_away() {
        let mut e = engine();
        profile_stream(&mut e);
        let (this, step, world, _) = collide_internal_world(&mut e);
        e.set_global(ABORT_FLAG, 1u32);
        noop(&mut e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE);
        e.call(0x00cfa080, &args![this, step]);
        assert_eq!(calls(&e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE).len(), 1);
        assert!(calls(&e, PROCESS_TRACK).is_empty());
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn collide_internal_stops_the_narrow_phase_when_the_flag_comes_up() {
        let mut e = engine();
        profile_stream(&mut e);
        let (this, step, world, _) = collide_internal_world(&mut e);
        e.mem.set_u32(world + 0x198, 1);
        noop(&mut e, COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE);
        noop(&mut e, FIRE_ISLAND_POST_COLLIDE_CALLBACK);
        e.register_double(PROCESS_TRACK, |e, _| {
            e.set_global(ABORT_FLAG, 1u32);
            Ret::default()
        });
        e.call(0x00cfa080, &args![this, step]);
        assert_eq!(calls(&e, PROCESS_TRACK).len(), 1);
        assert!(calls(&e, FIRE_ISLAND_POST_COLLIDE_CALLBACK).is_empty());
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    // ---- fn_00cf9480 ----

    /// Doubles for the callback, the allocator's free (virtual slot `0x10`) and the transform
    /// setter (virtual slot `0x3c` of the object at `body + 0xe0`).
    const PASS_CALLBACK: u32 = 0x0900_8000;
    const ALLOCATOR_FREE: u32 = 0x0900_8010;
    const TRANSFORM_SETTER: u32 = 0x0900_8020;
    const ALLOCATOR_VTABLE: u32 = 0x0900_8100;
    const TRANSFORM_VTABLE: u32 = 0x0900_8200;

    /// The memory router (the allocator at its `+0x10`), and no-op doubles for the map, the
    /// allocator's free and the time-of-impact invalidation. Returns the allocator.
    fn pass_doubles(e: &mut Engine) -> u32 {
        let allocator = e.mem.alloc(0x20);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let mut slots = [0u32; 16];
        slots[4] = ALLOCATOR_FREE;
        e.put_vtable(ALLOCATOR_VTABLE, &slots);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        for addr in [
            ALLOCATOR_FREE,
            MAP_INSERT,
            MAP_CLEAR_AND_DEALLOCATE,
            MAP_DESTRUCTOR,
            INVALIDATE_TIM_IN_AGENT,
        ] {
            noop(e, addr);
        }
        allocator
    }

    /// A double that logs its arguments followed by `count` words from each `(index, count)`
    /// span's address.
    fn logger(e: &mut Engine, addr: u32, spans: &'static [(usize, u32)]) -> EventLog {
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&log);
        e.register_double(addr, move |e, a| {
            let mut entry = a.to_vec();
            for &(index, count) in spans {
                entry.extend(words_at(e, a[index], count));
            }
            kept.borrow_mut().push(entry);
            Ret::default()
        });
        log
    }

    /// A collidable whose owner is itself, with the given owner type byte and `+0xcc` word.
    fn collidable(e: &mut Engine, owner_type: u8, word: u32) -> u32 {
        let block = e.mem.alloc(0x200);
        e.mem.set_u8(block + 0xe8, owner_type);
        e.mem.set_u32(block + 0xcc, word);
        block
    }

    /// A collision entry between the two collidables.
    fn collision_entry(e: &mut Engine, first: u32, second: u32) -> u32 {
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0x10, first);
        e.mem.set_u32(entry + 0x14, second);
        entry
    }

    #[test]
    fn collision_pass_calls_the_callback_for_unseen_entries_and_releases_its_storage() {
        let mut e = engine();
        let allocator = pass_doubles(&mut e);
        e.mem.set_f32(OUTPUT_INITIAL_FLOAT, 1.5);
        let body_a = e.mem.alloc(0x40);
        let body_b = e.mem.alloc(0x40);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, body_a);
        e.mem.set_u32(bodies + 4, body_b);
        // The first entry's owner is its first collidable; the third entry's first collidable
        // has the type byte 5, so the word comes from the second one, `4` bytes past its
        // collidable (the signed offset byte at `+0x10`).
        let plain = collidable(&mut e, 0, 0x111);
        let other = collidable(&mut e, 0, 0);
        let typed = collidable(&mut e, 5, 0);
        let offset = collidable(&mut e, 0, 0);
        e.mem.set_u8(offset + 0x10, 4);
        e.mem.set_u32(offset + 4 + 0xcc, 0x222);
        let entries = [
            collision_entry(&mut e, plain, other),
            collision_entry(&mut e, plain, other),
            collision_entry(&mut e, typed, offset),
        ];
        let list_a = e.mem.alloc(0x20);
        for (slot, (entry, key)) in [(entries[0], 100u32), (entries[1], 101)].iter().enumerate() {
            e.mem.set_u32(list_a + 8 * slot as u32, *entry);
            e.mem.set_u32(list_a + 8 * slot as u32 + 4, *key);
        }
        let list_b = e.mem.alloc(0x20);
        e.mem.set_u32(list_b, entries[2]);
        e.mem.set_u32(list_b + 4, 102);
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            // The first body's array is borrowed (sign bit set), the second one's is owned.
            let (array, size, capacity) = if a[0] == body_a + 0x10 {
                (list_a, 2, 0x8000_0002)
            } else {
                (list_b, 1, 5)
            };
            e.mem.set_u32(a[1], array);
            e.mem.set_u32(a[1] + 4, size);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        let find_count = Rc::new(Cell::new(0usize));
        let counted = Rc::clone(&find_count);
        // The second key is already in the map: `findKey` answers the map's `+8` word.
        e.register_double(MAP_FIND_KEY, move |_, _| {
            let answer = [0i32, -1, 0][counted.get()];
            counted.set(counted.get() + 1);
            ret(answer as u32)
        });
        let reserved = logger(&mut e, MAP_RESERVE, &[(0, 3)]);
        let seen: EventLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        e.register_double(PASS_CALLBACK, move |e, a| {
            let mut entry = a.to_vec();
            entry.extend(words_at(e, a[2] + 4, 1));
            entry.extend(words_at(e, a[2] + 0x3030, 1));
            entry.extend(words_at(e, a[2] + 0x3050, 2));
            kept.borrow_mut().push(entry);
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        e.call(0x00cf_9480, &args![bodies, 2i32, info, PASS_CALLBACK, 0u32]);

        let map = reserved.borrow()[0][0];
        assert_eq!(
            reserved.borrow()[0][..],
            [map, allocator, 2, 0, 0, u32::MAX][..]
        );
        let output = map + 0x144;
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![
                vec![map, allocator, body_a + 0x10, 0],
                vec![map, allocator, body_b + 0x10, 0]
            ]
        );
        let list = calls(&e, GET_COLLISION_ENTRIES_SORTED)[0][1];
        assert_eq!(list, map - 0x18);
        assert_eq!(
            calls(&e, GET_COLLISION_ENTRIES_SORTED),
            vec![vec![body_a + 0x10, list], vec![body_b + 0x10, list]]
        );
        assert_eq!(
            calls(&e, MAP_FIND_KEY),
            vec![vec![map, 100], vec![map, 101], vec![map, 102]]
        );
        // Only the entries whose key was not found reach the callback; mode 0 does nothing else.
        assert_eq!(
            calls(&e, INVALIDATE_TIM_IN_AGENT),
            vec![vec![entries[0], info], vec![entries[2], info]]
        );
        let bias = 1.5f32.to_bits();
        assert_eq!(
            seen.borrow()[..],
            [
                vec![entries[0], info, output, 0x111, bias, 0, 0],
                vec![entries[2], info, output, 0x222, bias, 0, 0]
            ][..]
        );
        // Only the second body's array was owned: 5 elements of 8 bytes.
        assert_eq!(calls(&e, ALLOCATOR_FREE), vec![vec![allocator, list_b, 40]]);
        assert_eq!(
            calls(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![map, allocator]]
        );
        assert_eq!(calls(&e, MAP_DESTRUCTOR), vec![vec![map]]);
        assert_eq!(calls(&e, MEMORY_ROUTER).len(), 5);
    }

    #[test]
    fn collision_pass_stops_when_the_abort_flag_comes_up_and_keeps_borrowed_arrays() {
        let mut e = engine();
        pass_doubles(&mut e);
        let body_a = e.mem.alloc(0x40);
        let body_b = e.mem.alloc(0x40);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, body_a);
        e.mem.set_u32(bodies + 4, body_b);
        let first = collidable(&mut e, 0, 1);
        let second = collidable(&mut e, 0, 2);
        let entry = collision_entry(&mut e, first, second);
        let list = e.mem.alloc(0x20);
        for slot in 0..2 {
            e.mem.set_u32(list + 8 * slot, entry);
            e.mem.set_u32(list + 8 * slot + 4, 50 + slot);
        }
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            e.mem.set_u32(a[1], list);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 0x8000_0001);
            Ret::default()
        });
        e.register_double(MAP_FIND_KEY, |_, _| ret(0));
        noop(&mut e, MAP_RESERVE);
        e.register_double(PASS_CALLBACK, |e, _| {
            e.set_global(ABORT_FLAG, 1u32);
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        e.call(0x00cf_9480, &args![bodies, 2i32, info, PASS_CALLBACK, 0u32]);
        // One entry of one body was processed.
        assert_eq!(calls(&e, MAP_INSERT).len(), 1);
        assert_eq!(calls(&e, MAP_FIND_KEY).len(), 1);
        assert_eq!(calls(&e, PASS_CALLBACK).len(), 1);
        assert!(calls(&e, ALLOCATOR_FREE).is_empty());
        assert_eq!(calls(&e, MAP_CLEAR_AND_DEALLOCATE).len(), 1);
        assert_eq!(calls(&e, MAP_DESTRUCTOR).len(), 1);
        assert_eq!(calls(&e, MEMORY_ROUTER).len(), 4);
    }

    #[test]
    fn collision_pass_without_bodies_only_sets_up_and_releases_the_map() {
        for count in [0i32, -3] {
            let mut e = engine();
            let allocator = pass_doubles(&mut e);
            noop(&mut e, MAP_RESERVE);
            e.call(0x00cf_9480, &args![0u32, count, 0u32, PASS_CALLBACK, 1u32]);
            let map = calls(&e, MAP_DESTRUCTOR)[0][0];
            assert_eq!(
                calls(&e, MAP_RESERVE),
                vec![vec![map, allocator, count as u32]]
            );
            assert!(calls(&e, MAP_INSERT).is_empty());
            assert!(calls(&e, GET_COLLISION_ENTRIES_SORTED).is_empty());
            assert!(calls(&e, ALLOCATOR_FREE).is_empty());
            assert_eq!(
                calls(&e, MAP_CLEAR_AND_DEALLOCATE),
                vec![vec![map, allocator]]
            );
        }
    }

    /// What `sweep_run` saw.
    struct SweepRun {
        e: Engine,
        /// Per callback call: the step and the two limits, then the body transform's `+0x30`
        /// vector, as words.
        seen: EventLog,
        axis_angle: EventLog,
        set_mul: EventLog,
        set_mul_inverse: EventLog,
        rotation: EventLog,
        transform: EventLog,
        body: u32,
        entry: u32,
        info: u32,
        limits: u32,
    }

    const SWEEP_MATRIX: [[f32; 4]; 4] = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];

    /// One body with one entry whose callback leaves a contact at the origin with the normal
    /// (0, 0, 1) (unless `contacts` is false), the body `position` at `+0x140`, and the
    /// sweep constants of the exe. With `negate` the entry's first collidable is not the body's.
    fn sweep_run(position: [f32; 4], negate: bool, contacts: bool, mode: u32) -> SweepRun {
        let mut e = engine();
        pass_doubles(&mut e);
        e.set_global(MIN_AXIS_LENGTH, 0.001f32);
        e.mem.set_f64(MAX_SWEEP_COSINE, 0.9999);
        e.mem.set_f64(SWEEP_TIME_SCALE, 20.0);
        e.mem.set_f64(SWEEP_DISTANCE_SCALE, 0.05);
        store_vec(&mut e, SIGN_MASK, [f32::from_bits(0x8000_0000); 4]);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        let body = e.mem.alloc(0x200);
        let bodies = e.mem.alloc(4);
        e.mem.set_u32(bodies, body);
        // The body is its own owner: the signed offset byte at `body + 0x20` is -16.
        e.mem.set_u8(body + 0x20, 0xf0);
        e.mem.set_u32(body + 0xcc, 7);
        let stranger = collidable(&mut e, 0, 9);
        let other = collidable(&mut e, 0, 0);
        let first = if negate { stranger } else { body + 0x10 };
        let entry = collision_entry(&mut e, first, other);
        for (row, vector) in SWEEP_MATRIX.iter().enumerate() {
            store_vec(&mut e, body + 0xf0 + 0x10 * row as u32, *vector);
        }
        store_vec(&mut e, body + 0x140, position);
        store_vec(&mut e, body + 0x160, [0.25, 0.5, 0.75, 1.0]);
        store_vec(&mut e, body + 0x170, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(body + 0xe0, TRANSFORM_VTABLE);
        let mut slots = [0u32; 16];
        slots[15] = TRANSFORM_SETTER;
        e.put_vtable(TRANSFORM_VTABLE, &slots);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, entry);
        e.mem.set_u32(list + 4, 7);
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            e.mem.set_u32(a[1], list);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 0x8000_0001);
            Ret::default()
        });
        e.register_double(MAP_FIND_KEY, |_, _| ret(0));
        noop(&mut e, MAP_RESERVE);
        let info = e.mem.alloc(0x80);
        let limits = e.mem.alloc(0x10);
        e.mem.set_f32(info + 8, 0.5);
        e.mem.set_u32(info + 0x60, limits);
        e.mem.set_f32(limits + 4, 3.0);
        e.mem.set_f32(limits + 8, 4.0);
        let seen: EventLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        let first_call = Cell::new(true);
        e.register_double(PASS_CALLBACK, move |e, a| {
            let mut words = vec![
                e.mem.u32(info + 8),
                e.mem.u32(limits + 4),
                e.mem.u32(limits + 8),
            ];
            words.extend(words_at(e, body + 0xf0 + 0x30, 4));
            kept.borrow_mut().push(words);
            if first_call.replace(false) {
                let output = a[2];
                if contacts {
                    e.mem.set_u32(output, output + 0x40);
                } else {
                    e.mem.set_u32(output, output + 0x10);
                }
                store_vec(e, output + 0x10, [0.0; 4]);
                store_vec(e, output + 0x20, [0.0, 0.0, 1.0, 0.0]);
            }
            Ret::default()
        });
        e.register_double(ARC_COSINE, |_, a| Ret {
            st0: 10.0 * f64::from(f32::from_bits(a[0])) + 1.0,
            ..Ret::default()
        });
        let axis_angle = logger(&mut e, QUATERNION_SET_AXIS_ANGLE, &[(1, 4)]);
        let set_mul = logger(&mut e, QUATERNION_SET_MUL, &[(2, 4)]);
        let set_mul_inverse = logger(&mut e, QUATERNION_SET_MUL_INVERSE, &[(2, 4)]);
        let rotation = logger(&mut e, TRANSFORM_SET_ROTATION, &[]);
        let transform = logger(&mut e, TRANSFORM_SETTER, &[(1, 16)]);
        e.call(0x00cf_9480, &args![bodies, 1i32, info, PASS_CALLBACK, mode]);
        SweepRun {
            e,
            seen,
            axis_angle,
            set_mul,
            set_mul_inverse,
            rotation,
            transform,
            body,
            entry,
            info,
            limits,
        }
    }

    fn float_words(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    /// The sweep's angle for the default setup: `acos(cos) - acos(target)` with the stand-in
    /// arc cosine `10 x + 1`, the first result rounded to a float.
    fn sweep_angle(cosine: f32, target: f32) -> f32 {
        let first = (10.0 * f64::from(cosine) + 1.0) as f32;
        let second = 10.0 * f64::from(target) + 1.0;
        (f64::from(first) - second) as f32
    }

    #[test]
    fn collision_pass_sweeps_the_body_through_three_rotations_and_restores_the_step() {
        let run = sweep_run([1.0, 0.0, 1.0, 0.0], false, true, 1);
        let seen = run.seen.borrow();
        assert_eq!(seen.len(), 5);
        let step = |step: f32, first: f32, second: f32| [step, first, second].map(f32::to_bits);
        // The step and both limits are twenty times the step during the sweep.
        assert_eq!(seen[0][..3], step(0.5, 3.0, 4.0));
        for call in &seen[1..4] {
            assert_eq!(call[..3], step(10.0, 10.0, 10.0));
        }
        assert_eq!(seen[4][..3], step(0.5, 3.0, 4.0));
        assert_eq!(run.e.mem.f32(run.info + 8), 0.5);
        assert_eq!(run.e.mem.f32(run.limits + 4), 3.0);
        assert_eq!(run.e.mem.f32(run.limits + 8), 4.0);
        // The transform's translation: the body position minus the offset times the rotation
        // rows, after each of the rotations.
        assert_eq!(float_words(&seen[0][3..7]), SWEEP_MATRIX[3]);
        let position = [1.0f32, 0.0, 1.0, 0.0];
        let swept: Vec<f32> = (0..4)
            .map(|lane| {
                position[lane]
                    - (2.0 * SWEEP_MATRIX[1][lane]
                        + SWEEP_MATRIX[0][lane]
                        + 3.0 * SWEEP_MATRIX[2][lane])
            })
            .collect();
        for call in &seen[1..5] {
            assert_eq!(float_words(&call[3..7]), swept);
        }

        // The rotations: the contact normal (0, 0, 1), the gap (1, 0, 1), so the axis is
        // (0, 1, 0), the cosine is 1 / sqrt(2) and the target is (1 - 0.5) / sqrt(2).
        let length = 2.0f32.sqrt();
        let cosine = (1.0 / f64::from(length)) as f32;
        let target = ((1.0 - 10.0 * 0.05) / f64::from(length)) as f32;
        let angle = sweep_angle(cosine, target);
        let axis_angle = run.axis_angle.borrow();
        let frame = axis_angle[0][1] - 0xb0;
        assert_eq!(
            axis_angle[0][..3],
            [frame + 0x100, frame + 0xb0, angle.to_bits()]
        );
        assert_eq!(float_words(&axis_angle[0][3..7]), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(
            axis_angle[1][..3],
            [frame + 0xe0, frame + 0x110, angle.to_bits()]
        );
        let second_axis = float_words(&axis_angle[1][3..7]);
        for (lane, wanted) in [1.0f32, 0.0, 0.0, 0.0].iter().enumerate() {
            assert!((second_axis[lane] - wanted).abs() < 1e-3, "axis {lane}");
        }
        let quaternion = [0.25f32, 0.5, 0.75, 1.0].map(f32::to_bits);
        let set_mul = run.set_mul.borrow();
        assert_eq!(set_mul.len(), 2);
        assert_eq!(
            set_mul[0][..3],
            [frame + 0x130, frame + 0x100, run.body + 0x160]
        );
        assert_eq!(set_mul[0][3..], quaternion);
        assert_eq!(set_mul[1][..3], [frame + 0xd0, frame + 0xe0, frame + 0x70]);
        assert_eq!(set_mul[1][3..], quaternion);
        let inverse = run.set_mul_inverse.borrow();
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0][..3], [frame + 0xd0, frame + 0xe0, frame + 0x70]);
        assert_eq!(inverse[0][3..], quaternion);
        let transform = run.body + 0xf0;
        assert_eq!(
            run.rotation.borrow()[..],
            [
                vec![transform, frame + 0x130],
                vec![transform, frame + 0xd0],
                vec![transform, frame + 0xd0]
            ][..]
        );
        // The original transform is put back through the object at `body + 0xe0`.
        let restored = run.transform.borrow();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0][..2], [run.body + 0xe0, frame + 0x140]);
        let original: Vec<f32> = SWEEP_MATRIX.iter().flatten().copied().collect();
        assert_eq!(float_words(&restored[0][2..]), original);
        assert_eq!(
            calls(&run.e, INVALIDATE_TIM_IN_AGENT),
            vec![vec![run.entry, run.info]; 5]
        );
    }

    #[test]
    fn collision_pass_turns_the_normal_around_for_the_other_collidable() {
        let run = sweep_run([1.0, 0.0, 1.0, 0.0], true, true, 1);
        assert_eq!(run.seen.borrow().len(), 5);
        // The normal becomes (0, 0, -1): the axis (0, -1, 0), the cosine -1 / sqrt(2) and a
        // negative target, which is replaced by zero.
        let length = 2.0f32.sqrt();
        let cosine = (-1.0 / f64::from(length)) as f32;
        let angle = sweep_angle(cosine, 0.0);
        let axis_angle = run.axis_angle.borrow();
        assert_eq!(axis_angle[0][2], angle.to_bits());
        assert_eq!(float_words(&axis_angle[0][3..7]), [0.0, -1.0, 0.0, 0.0]);
        let second_axis = float_words(&axis_angle[1][3..7]);
        for (lane, wanted) in [1.0f32, 0.0, 0.0, 0.0].iter().enumerate() {
            assert!((second_axis[lane] - wanted).abs() < 1e-3, "axis {lane}");
        }
    }

    #[test]
    fn collision_pass_skips_the_sweep_without_contacts_or_a_usable_angle() {
        // No contact points in the output block, or a mode other than 1.
        for (contacts, mode) in [(false, 1), (true, 0), (true, 2)] {
            let run = sweep_run([1.0, 0.0, 1.0, 0.0], false, contacts, mode);
            assert_eq!(run.seen.borrow().len(), 1);
            assert!(run.axis_angle.borrow().is_empty());
        }
        // A gap nearly along the normal (cosine above 0.9999), and one exactly along it (the
        // axis is too short).
        for position in [[0.01, 0.0, 1.0, 0.0], [0.0, 0.0, 1.0, 0.0]] {
            let run = sweep_run(position, false, true, 1);
            assert_eq!(run.seen.borrow().len(), 1);
            assert!(run.axis_angle.borrow().is_empty());
            assert!(run.transform.borrow().is_empty());
            assert_eq!(run.e.mem.f32(run.info + 8), 0.5);
            assert_eq!(run.e.mem.f32(run.limits + 4), 3.0);
        }
    }
}
