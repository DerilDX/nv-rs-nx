//! `fallout/ai/patrolpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `PatrolActorPackageData` (Xbox PDB), 0x38 bytes on PC as on the Xbox (the offsets below
    /// are the ones the PC code uses).
    pub struct PatrolActorPackageData: 0x38 {
        /// `eState` (Xbox PDB): `PATROL_STATE`.
        0x04 eState: u32,
        /// `fCurrentTimeAtReference` (Xbox PDB).
        0x08 fCurrentTimeAtReference: u32,
        /// `bPatrolDirection` (Xbox PDB): true while walking forward through the references.
        0x0C bPatrolDirection: u8,
        /// `pActor` (Xbox PDB): `Actor *`.
        0x10 pActor: u32,
        /// `pPackage` (Xbox PDB): `TESPackage *`.
        0x14 pPackage: u32,
        /// `pPackageData` (Xbox PDB): `const TESPatrolPackageData *`.
        0x18 pPackageData: u32,
        /// `bCircularArray` (Xbox PDB).
        0x2C bCircularArray: u8,
        /// `iLoopAroundIndex` (Xbox PDB).
        0x30 iLoopAroundIndex: u32,
        /// `iCurrentReference` (Xbox PDB).
        0x34 iCurrentReference: u32,
    }
}

/// `References` (Xbox PDB): the embedded `BSSimpleArray<TESObjectREFR *,1024>` at `+0x1c`.
const REFERENCES: u32 = 0x1c;
/// `0044ddc0` (unnamed in the engine map): `thiscall`, no stack words; the array's size.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00877a30` (unnamed in the engine map, `main.cpp`): `thiscall`, `RET 4`; the address of
/// element `index`.
const ARRAY_AT: u32 = 0x0087_7a30;
/// `0076b610` (unnamed in the engine map): `thiscall`, no stack words; true (`AL`) when the
/// array is empty.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `005d43c0` (unnamed in the engine map): `thiscall`, no stack words; on a reference, its
/// `ExtraDataList` (the methods called on the result are `ExtraDataList`'s).
const REFERENCE_EXTRA_DATA: u32 = 0x005d_43c0;
/// `ExtraDataList::GetPatrolRefCanBeUsedBy` (`0042f3d0`, `RET 4`, result in `AL`).
const PATROL_REF_CAN_BE_USED_BY: u32 = 0x0042_f3d0;
/// `0042f300` (unnamed in the engine map, `extradatalist.cpp`): `thiscall`, `RET 4`; stores the
/// actor that uses the patrol reference (0 to release it).
const SET_PATROL_REF_USER: u32 = 0x0042_f300;
/// `004f1540` (unnamed in the engine map): `thiscall`, no stack words; on the
/// `TESPatrolPackageData`, whether the patrol repeats (`AL`).
const PATROL_REPEATS: u32 = 0x004f_1540;
/// `BSSimpleArray::SetSize` instance (`006f2290`, `RET 8`).
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `BGSSaveGameBuffer` writer of raw bytes (`00865e50`, `RET 0xc`: data, size, 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, `RET 8`).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// The actor's position getter: vtable slot `0x1f4` (no stack words).
const GET_POSITION_SLOT: u32 = 0x1f4;

fn references(this: Ptr<PatrolActorPackageData>) -> u32 {
    this.addr().wrapping_add(REFERENCES)
}

/// The reference at `index` of `References`.
fn reference_at(e: &mut Engine, this: Ptr<PatrolActorPackageData>, index: u32) -> u32 {
    let slot = e.call(ARRAY_AT, &args![references(this), index]).u32();
    e.mem.u32(slot)
}

/// The `ExtraDataList` of `reference` (`005d43c0`), or 0 for a null reference.
fn extra_data(e: &mut Engine, reference: u32) -> u32 {
    if reference != 0 {
        e.call(REFERENCE_EXTRA_DATA, &args![reference]).u32()
    } else {
        0
    }
}

fn array_size(e: &mut Engine, this: Ptr<PatrolActorPackageData>) -> u32 {
    e.call(ARRAY_SIZE, &args![references(this)]).u32()
}

fn array_is_empty(e: &mut Engine, this: Ptr<PatrolActorPackageData>) -> bool {
    e.call(ARRAY_IS_EMPTY, &args![references(this)]).bool()
}

// Translated from 009f2a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::scalar deleting destructor` (Xbox PDB): runs the destructor body
/// `009f2a70` of this unit and frees `this` (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn patrol_actor_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    flags: u32,
) -> Ptr<PatrolActorPackageData> {
    fn_009f2a70(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009f2a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f2a70 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Destructor body of `PatrolActorPackageData` (unnamed in the engine map): stores the vtable
/// `01093418`; unless `004226e0(TESDataHandler)` is true (the handler at `011c3f2c`), releases
/// the current reference (`009f2b30` of this unit); empties `References` (`SetSize(0, 1)`), sets
/// `eState` to 4 and clears `pActor`, `pPackage` and `pPackageData`; then destroys the array
/// (`005e04f0`) and runs the `ActorPackageData` destructor (`0067d970`).
pub fn fn_009f2a70(e: &mut Engine, this: Ptr<PatrolActorPackageData>) {
    e.mem.set_u32(this.addr(), 0x0109_3418);
    let data_handler = e.mem.u32(0x011c_3f2c);
    if !e.call(0x0042_26e0, &args![data_handler]).bool() {
        fn_009f2b30(e, this);
    }
    e.call(ARRAY_SET_SIZE, &args![references(this), 0u32, 1u32]);
    e.set(this, PatrolActorPackageData::eState, 4);
    e.set(this, PatrolActorPackageData::pActor, 0);
    e.set(this, PatrolActorPackageData::pPackage, 0);
    e.set(this, PatrolActorPackageData::pPackageData, 0);
    e.call(0x005e_04f0, &args![references(this)]);
    e.call(0x0067_d970, &args![this]);
}

// Translated from 009f2b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the current patrol reference (unnamed in the engine map): when `iCurrentReference`
/// is below the array's size, the reference's `ExtraDataList` (when the reference and its list
/// exist) gets 0 as its patrol user (`0042f300`).
pub fn fn_009f2b30(e: &mut Engine, this: Ptr<PatrolActorPackageData>) {
    let size = array_size(e, this);
    let current = e.get(this, PatrolActorPackageData::iCurrentReference);
    if current >= size {
        return;
    }
    let reference = reference_at(e, this, current);
    let extra = extra_data(e, reference);
    if extra != 0 {
        e.call(SET_PATROL_REF_USER, &args![extra, 0u32]);
    }
}

// Translated from 009f2ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::Init` (Xbox PDB), `RET 8`: stores the actor and the package, the
/// package's patrol data (`TESPackage::GetPatrolPackageData`, `00675920`), clears
/// `iCurrentReference` and `iLoopAroundIndex`, sets the direction forward and builds the
/// reference array (`BuildReferenceArray`). When that found references and the patrol repeats
/// (`004f1540`), the patrol resumes (`ResumeRepeatablePatrol`) from the actor's position (its
/// vtable slot `0x1f4`). Returns what `BuildReferenceArray` returned (`AL`).
pub fn patrol_actor_package_data_init(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    actor: u32,
    package: u32,
) -> u8 {
    e.set(this, PatrolActorPackageData::pActor, actor);
    e.set(this, PatrolActorPackageData::pPackage, package);
    let data = e.call(0x0067_5920, &args![package]).u32();
    e.set(this, PatrolActorPackageData::pPackageData, data);
    e.set(this, PatrolActorPackageData::iCurrentReference, 0);
    e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
    e.set(this, PatrolActorPackageData::iLoopAroundIndex, 0);
    let built = patrol_actor_package_data_build_reference_array(e, this) as u8;
    if built != 0 {
        let data = e.get(this, PatrolActorPackageData::pPackageData);
        if e.call(PATROL_REPEATS, &args![data]).bool() {
            let position = e.vcall(actor, GET_POSITION_SLOT, &args![]).u32();
            patrol_actor_package_data_resume_repeatable_patrol(e, this, position);
        }
    }
    built
}

// Translated from 009f2c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::HandleDeletedReference` (Xbox PDB), `RET 4`. The patrolling actor
/// itself: the current reference is released (`009f2b30`), `eState` becomes 3 and the actor,
/// package and patrol data are cleared. Any other reference is removed from `References`
/// wherever it occurs (`006bf8f0(index, 1)`), `iCurrentReference` stepping back (wrapping) when
/// it was at or past the removed index; then, with an actor, outside `0042ce10` of the object
/// at `011ddf38`, and when `SetCurrentReference(iCurrentReference)` fails, the patrol resumes
/// from the actor's position (vtable slot `0x1f4`).
pub fn patrol_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    reference: u32,
) {
    if reference == e.get(this, PatrolActorPackageData::pActor) {
        fn_009f2b30(e, this);
        e.set(this, PatrolActorPackageData::eState, 3);
        e.set(this, PatrolActorPackageData::pActor, 0);
        e.set(this, PatrolActorPackageData::pPackage, 0);
        e.set(this, PatrolActorPackageData::pPackageData, 0);
        return;
    }
    let mut index = 0u32;
    while index < array_size(e, this) {
        if reference_at(e, this, index) == reference {
            e.call(0x006b_f8f0, &args![references(this), index, 1u32]);
            let current = e.get(this, PatrolActorPackageData::iCurrentReference);
            if current >= index {
                e.set(
                    this,
                    PatrolActorPackageData::iCurrentReference,
                    current.wrapping_sub(1),
                );
            }
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
    if e.get(this, PatrolActorPackageData::pActor) == 0 {
        return;
    }
    let object = e.mem.u32(0x011d_df38);
    if e.call(0x0042_ce10, &args![object]).bool() {
        return;
    }
    let current = e.get(this, PatrolActorPackageData::iCurrentReference);
    if patrol_actor_package_data_set_current_reference(e, this, current) != 0 {
        return;
    }
    let actor = e.get(this, PatrolActorPackageData::pActor);
    let position = e.vcall(actor, GET_POSITION_SLOT, &args![]).u32();
    patrol_actor_package_data_resume_repeatable_patrol(e, this, position);
}

// Translated from 009f2d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::GetInitialReferenceForSetup` (Xbox PDB), `cdecl` (`actor`,
/// `package`): the reference of the package's location (`0055b980(package)`, then
/// `PackageLocation::GetLocReference`, `0067f390`), or else the actor's `00569b80` reference;
/// 0 without either.
pub fn patrol_actor_package_data_get_initial_reference_for_setup(
    e: &mut Engine,
    actor: u32,
    package: u32,
) -> u32 {
    let mut result = 0;
    let location = e.call(0x0055_b980, &args![package]).u32();
    if location != 0 {
        let reference = e.call(0x0067_f390, &args![location]).u32();
        if reference != 0 {
            result = reference;
        }
    }
    if result == 0 {
        let reference = e.call(0x0056_9b80, &args![actor]).u32();
        if reference != 0 {
            result = reference;
        }
    }
    result
}

// Translated from 009f2da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::BuildReferenceArray` (Xbox PDB): starting from
/// `GetInitialReferenceForSetup(actor, package)`, empties `References` (`SetSize(0, 1)`),
/// clears `bCircularArray` and, for each reference of the chain (`00569b80` gives the next),
/// marks it targeted (`TESObjectREFR::SetTargeted(1)`) and adds it (`007cb2e0`). When the next
/// reference is already in the array (`00719b20(&next, 0, 009a3830)` is not -1) the chain
/// stops: a repeating patrol (`004f1540` on the patrol data, when there is one) records the
/// loop (`bCircularArray`, `iLoopAroundIndex` = the found index), a one-way one adds it again.
/// Returns 1 when the array is not empty (`0076b610`), else 0.
pub fn patrol_actor_package_data_build_reference_array(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
) -> u32 {
    let actor = e.get(this, PatrolActorPackageData::pActor);
    let package = e.get(this, PatrolActorPackageData::pPackage);
    let first = patrol_actor_package_data_get_initial_reference_for_setup(e, actor, package);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, first);
        e.set(this, PatrolActorPackageData::bCircularArray, 0);
        e.call(ARRAY_SET_SIZE, &args![references(this), 0u32, 1u32]);
        let data = e.get(this, PatrolActorPackageData::pPackageData);
        let repeats = data != 0 && e.call(PATROL_REPEATS, &args![data]).bool();
        while e.mem.u32(slot) != 0 {
            let reference = e.mem.u32(slot);
            e.call(0x0056_4db0, &args![reference, 1u32]);
            e.call(0x007c_b2e0, &args![references(this), slot]);
            let reference = e.mem.u32(slot);
            let next = e.call(0x0056_9b80, &args![reference]).u32();
            e.mem.set_u32(slot, next);
            if next == 0 {
                break;
            }
            let found = e
                .call(
                    0x0071_9b20,
                    &args![references(this), slot, 0u32, 0x009a_3830u32],
                )
                .u32();
            if found != u32::MAX {
                if !repeats {
                    e.call(0x007c_b2e0, &args![references(this), slot]);
                } else {
                    e.set(this, PatrolActorPackageData::bCircularArray, 1);
                    e.set(this, PatrolActorPackageData::iLoopAroundIndex, found);
                }
                break;
            }
        }
    });
    u32::from(!array_is_empty(e, this))
}

// Translated from 009f2ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::SetNextReferenceForPatrol` (Xbox PDB), `RET 4`, result in `AL`.
/// Steps `iCurrentReference` in the patrol direction. Forward, reaching the array's size: a
/// repeating patrol (`004f1540` on the patrol data, false without one) goes to
/// `iLoopAroundIndex` when circular, otherwise turns back (to 0 for a one-element array, else
/// size - 2) and flips the direction; a one-way patrol ends (index 0, result 0). Backward,
/// reaching -1: a circular repeating patrol ends (0, result 0), another repeating one turns
/// forward again (0 for a one-element array, else 1, direction flipped), a one-way one ends.
/// With `apply` (its low byte) the new index is only taken through `SetCurrentReference`; when
/// that refuses it and no other reference (the current one excepted) can be used by the actor
/// (`ExtraDataList::GetPatrolRefCanBeUsedBy`, a reference without a list counting as unusable),
/// the result is that of `SetNextReferenceForPatrol(0)`. Without `apply` the index is stored.
pub fn patrol_actor_package_data_set_next_reference_for_patrol(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    apply: u32,
) -> u8 {
    let mut next = e.get(this, PatrolActorPackageData::iCurrentReference);
    let mut result = 1u8;
    let data = e.get(this, PatrolActorPackageData::pPackageData);
    let repeats = data != 0 && e.call(PATROL_REPEATS, &args![data]).bool();
    let flip = |e: &mut Engine| {
        let direction = e.get(this, PatrolActorPackageData::bPatrolDirection);
        e.set(
            this,
            PatrolActorPackageData::bPatrolDirection,
            u8::from(direction == 0),
        );
    };
    if e.get(this, PatrolActorPackageData::bPatrolDirection) != 0 {
        next = next.wrapping_add(1);
        if next == array_size(e, this) {
            if repeats {
                if e.get(this, PatrolActorPackageData::bCircularArray) != 0 {
                    next = e.get(this, PatrolActorPackageData::iLoopAroundIndex);
                } else {
                    next = if array_size(e, this) == 1 {
                        0
                    } else {
                        next.wrapping_sub(2)
                    };
                    flip(e);
                }
            } else {
                next = 0;
                result = 0;
            }
        }
    } else {
        next = next.wrapping_sub(1);
        if next == u32::MAX {
            if repeats {
                if e.get(this, PatrolActorPackageData::bCircularArray) != 0 {
                    next = 0;
                    result = 0;
                } else {
                    next = u32::from(array_size(e, this) != 1);
                    flip(e);
                }
            } else {
                next = 0;
                result = 0;
            }
        }
    }
    if apply as u8 == 0 {
        e.set(this, PatrolActorPackageData::iCurrentReference, next);
        return result;
    }
    if next == e.get(this, PatrolActorPackageData::iCurrentReference) {
        return result;
    }
    if patrol_actor_package_data_set_current_reference(e, this, next) != 0 {
        return result;
    }
    let size = array_size(e, this);
    let mut usable = size.wrapping_sub(1);
    let mut index = 0u32;
    while index < size {
        let reference = reference_at(e, this, index);
        let extra = extra_data(e, reference);
        if index != e.get(this, PatrolActorPackageData::iCurrentReference) {
            let actor = e.get(this, PatrolActorPackageData::pActor);
            if extra == 0
                || !e
                    .call(PATROL_REF_CAN_BE_USED_BY, &args![extra, actor])
                    .bool()
            {
                usable = usable.wrapping_sub(1);
            }
        }
        index = index.wrapping_add(1);
    }
    if usable == 0 {
        return patrol_actor_package_data_set_next_reference_for_patrol(e, this, 0);
    }
    result
}

// Translated from 009f30d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::SetCurrentReference` (Xbox PDB), `RET 4`, result in `AL`. An empty
/// array or a non-circular one takes `index` as it is (1). Otherwise the index is clamped to
/// `[0, size - 1]` (`00647ba0`) and the reference there, when it has an `ExtraDataList`, must
/// accept the actor (`ExtraDataList::GetPatrolRefCanBeUsedBy`; 0 when it does not): the
/// current reference is released (`009f2b30`), the new one gets the actor as its user
/// (`0042f300`) and becomes current (1). A reference without a list changes nothing (1).
pub fn patrol_actor_package_data_set_current_reference(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    index: u32,
) -> u8 {
    if array_is_empty(e, this) || e.get(this, PatrolActorPackageData::bCircularArray) == 0 {
        e.set(this, PatrolActorPackageData::iCurrentReference, index);
        return 1;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, index);
        let last = array_size(e, this).wrapping_sub(1);
        e.call(0x0064_7ba0, &args![slot, 0u32, last]);
        let mut result = 1u8;
        let clamped = e.mem.u32(slot);
        let reference = reference_at(e, this, clamped);
        let extra = extra_data(e, reference);
        if extra != 0 {
            let actor = e.get(this, PatrolActorPackageData::pActor);
            if e.call(PATROL_REF_CAN_BE_USED_BY, &args![extra, actor])
                .bool()
            {
                fn_009f2b30(e, this);
                let actor = e.get(this, PatrolActorPackageData::pActor);
                e.call(SET_PATROL_REF_USER, &args![extra, actor]);
                let clamped = e.mem.u32(slot);
                e.set(this, PatrolActorPackageData::iCurrentReference, clamped);
            } else {
                result = 0;
            }
        }
        result
    })
}

// Translated from 009f31c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::ResumeRepeatablePatrol` (Xbox PDB), `RET 4` (`position`, the address
/// of a point). With an actor: starts again at reference 0, forward, and walks the whole patrol
/// once (`SetNextReferenceForPatrol(0)` per step, as many steps as the array has elements),
/// measuring for each reached reference the distance from the previous reference's position
/// (vtable slot `0x1f4`) to `position` (`00439ef0` gives the difference, `00595c80` its length;
/// a second difference, between the two references, is computed and not used). References
/// whose `ExtraDataList` refuses the actor are skipped; the walk stops when there is no current
/// reference. The step with the shortest distance (below `FLT_MAX`, `01016970`) wins: its index
/// is set through `SetCurrentReference` and its direction restored; the direction is then set
/// forward again in every case.
pub fn patrol_actor_package_data_resume_repeatable_patrol(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    position: u32,
) {
    if e.get(this, PatrolActorPackageData::pActor) == 0 {
        return;
    }
    let steps = array_size(e, this) as i32;
    patrol_actor_package_data_set_current_reference(e, this, 0);
    e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
    let mut current = patrol_actor_package_data_get_current_reference(e, this);
    if current == 0 {
        return;
    }
    let mut current_position = e.vcall(current, GET_POSITION_SLOT, &args![]).u32();
    let mut best = x87_quiet_bits(e.mem.u32(0x0101_6970));
    let mut best_index = 0u32;
    let mut best_direction = 1u8;
    let mut found = false;
    e.with_stack(0x18, |e, vectors| {
        let difference = vectors.addr();
        let unused = difference + 0xc;
        let mut step = 0i32;
        while step < steps {
            let index = e.get(this, PatrolActorPackageData::iCurrentReference);
            let direction = e.get(this, PatrolActorPackageData::bPatrolDirection);
            let previous_position = current_position;
            patrol_actor_package_data_set_next_reference_for_patrol(e, this, 0);
            let reached = patrol_actor_package_data_get_current_reference(e, this);
            let extra = extra_data(e, reached);
            let usable = extra == 0 || {
                let actor = e.get(this, PatrolActorPackageData::pActor);
                e.call(PATROL_REF_CAN_BE_USED_BY, &args![extra, actor])
                    .bool()
            };
            if usable {
                current = patrol_actor_package_data_get_current_reference(e, this);
                if current == 0 {
                    break;
                }
                current_position = e.vcall(current, GET_POSITION_SLOT, &args![]).u32();
                e.call(0x0043_9ef0, &args![previous_position, difference, position]);
                e.call(
                    0x0043_9ef0,
                    &args![current_position, unused, previous_position],
                );
                let length = x87_store32(e.call(0x0059_5c80, &args![difference]).f64());
                if fpu_greater(x87_load32(best), x87_load32(length)) {
                    best = length;
                    best_index = index;
                    best_direction = direction;
                    found = true;
                }
            }
            step += 1;
        }
    });
    if found {
        patrol_actor_package_data_set_current_reference(e, this, best_index);
        e.set(
            this,
            PatrolActorPackageData::bPatrolDirection,
            best_direction,
        );
    }
    e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
}

/// `FCOMPP` with `st0` against `st1`, then `TEST AH,0x41` / `JNZ`: true only when `st0 > st1`
/// (ordered).
fn fpu_greater(st0: f64, st1: f64) -> bool {
    st0 > st1
}

// Translated from 009f3390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::GetCurrentReference` (Xbox PDB): 0 for an empty array, else the
/// reference at `iCurrentReference`.
pub fn patrol_actor_package_data_get_current_reference(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
) -> u32 {
    if array_is_empty(e, this) {
        return 0;
    }
    let current = e.get(this, PatrolActorPackageData::iCurrentReference);
    reference_at(e, this, current)
}

// Translated from 009f33d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::SaveGame` (Xbox PDB), `RET 4`: writes `eState`,
/// `fCurrentTimeAtReference`, `iCurrentReference` (4 bytes each), `bPatrolDirection`,
/// `bCircularArray` (1 byte each), `iLoopAroundIndex`, the package's form id, the array's
/// size (`SaveVariableSizedValue`, `00865f60`; 0 when the array's address is null, which the exe
/// tests) and each reference's form id (`006a7ad0` gives the element's address).
pub fn patrol_actor_package_data_save_game(
    e: &mut Engine,
    this: Ptr<PatrolActorPackageData>,
    buffer: u32,
) {
    let base = this.addr();
    for (offset, size) in [
        (4u32, 4u32),
        (8, 4),
        (0x34, 4),
        (0xc, 1),
        (0x2c, 1),
        (0x30, 4),
    ] {
        e.call(
            SAVE_BYTES,
            &args![buffer, base.wrapping_add(offset), size, 0u32],
        );
    }
    let package = e.get(this, PatrolActorPackageData::pPackage);
    e.call(SAVE_FORM_ID, &args![buffer, package, 0u32]);
    let count = if references(this) != 0 {
        array_size(e, this)
    } else {
        0
    };
    e.call(0x0086_5f60, &args![buffer, count]);
    for index in 0..count {
        let slot = e.call(0x006a_7ad0, &args![references(this), index]).u32();
        let reference = e.mem.u32(slot);
        e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009f_2a40,
            patrol_actor_package_data_scalar_deleting_destructor(
                Ptr<PatrolActorPackageData>,
                u32,
            )
                -> Ptr<PatrolActorPackageData>
        ),
        entry!(0x009f_2a70, fn_009f2a70(Ptr<PatrolActorPackageData>)),
        entry!(0x009f_2b30, fn_009f2b30(Ptr<PatrolActorPackageData>)),
        entry!(
            0x009f_2ba0,
            patrol_actor_package_data_init(Ptr<PatrolActorPackageData>, u32, u32) -> u8
        ),
        entry!(
            0x009f_2c30,
            patrol_actor_package_data_handle_deleted_reference(Ptr<PatrolActorPackageData>, u32)
        ),
        entry!(0x009f_2d40, patrol_actor_package_data_get_initial_reference_for_setup(u32, u32) -> u32),
        entry!(
            0x009f_2da0,
            patrol_actor_package_data_build_reference_array(Ptr<PatrolActorPackageData>) -> u32
        ),
        entry!(
            0x009f_2ea0,
            patrol_actor_package_data_set_next_reference_for_patrol(
                Ptr<PatrolActorPackageData>,
                u32,
            ) -> u8
        ),
        entry!(
            0x009f_30d0,
            patrol_actor_package_data_set_current_reference(Ptr<PatrolActorPackageData>, u32) -> u8
        ),
        entry!(
            0x009f_31c0,
            patrol_actor_package_data_resume_repeatable_patrol(Ptr<PatrolActorPackageData>, u32)
        ),
        entry!(
            0x009f_3390,
            patrol_actor_package_data_get_current_reference(Ptr<PatrolActorPackageData>) -> u32
        ),
        entry!(
            0x009f_33d0,
            patrol_actor_package_data_save_game(Ptr<PatrolActorPackageData>, u32)
        ),
    ]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// Doubles for the `References` array: buffer at `+4`, size at `+8` of the array.
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(ARRAY_AT, |e, a| (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret());
        e.register(0x006a_7ad0, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(0x007c_b2e0, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(0x006b_f8f0, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            for index in a[1]..size - 1 {
                let next = e.mem.u32(buffer + 4 * (index + 1));
                e.mem.set_u32(buffer + 4 * index, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        e.register(0x0071_9b20, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let wanted = e.mem.u32(a[1]);
            let found = (0..size)
                .find(|index| e.mem.u32(buffer + 4 * index) == wanted)
                .unwrap_or(u32::MAX);
            found.into_ret()
        });
    }

    /// A patrol object holding `refs`.
    fn patrol(e: &mut Engine, refs: &[u32]) -> Ptr<PatrolActorPackageData> {
        let this = e.mem.alloc(0x38);
        let buffer = e.mem.alloc(0x40);
        for (index, reference) in refs.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *reference);
        }
        e.mem.set_u32(this + REFERENCES + 4, buffer);
        e.mem.set_u32(this + REFERENCES + 8, refs.len() as u32);
        Ptr::new(this)
    }

    /// Each reference `r` has the list `r + 1` (none for `r` = 0); the list accepts every actor
    /// except those listed in `refusing` (by list).
    fn extra_doubles(e: &mut Engine, refusing: Vec<u32>) {
        e.register(REFERENCE_EXTRA_DATA, |_, a| (a[0] + 1).into_ret());
        e.register_double(PATROL_REF_CAN_BE_USED_BY, move |_, a| {
            (!refusing.contains(&a[0])).into_ret()
        });
        quiet(e, &[SET_PATROL_REF_USER]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.map(0x011c_3000, 0x1000);
        e.register(0x0042_26e0, |_, _| true.into_ret());
        quiet(&mut e, &[0x005e_04f0, 0x0067_d970, 0x0040_1030]);
        let this = patrol(&mut e, &[0x100]);
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_2a40, &args![this, 1u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this.addr());
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![this.addr()]]);
        assert_eq!(e.mem.u32(this.addr()), 0x0109_3418);
        assert_eq!(e.get(this, PatrolActorPackageData::eState), 4);
        e.call_log = Some(vec![]);
        e.call(0x009f_2a40, &args![this, 2u32]);
        assert!(calls(&e.call_log.take().unwrap(), 0x0040_1030).is_empty());
    }

    #[test]
    fn destructor_releases_the_reference_unless_the_handler_says_otherwise() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(0x011c_3f2c, 0x900);
        e.register(0x0042_26e0, |_, _| false.into_ret());
        quiet(&mut e, &[0x005e_04f0, 0x0067_d970]);
        let this = patrol(&mut e, &[0x100]);
        e.set(this, PatrolActorPackageData::pActor, 0x50);
        e.call_log = Some(vec![]);
        e.call(0x009f_2a70, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0042_26e0), vec![vec![0x900]]);
        assert_eq!(calls(&log, SET_PATROL_REF_USER), vec![vec![0x101, 0]]);
        assert_eq!(e.mem.u32(this.addr() + REFERENCES + 8), 0);
        assert_eq!(e.get(this, PatrolActorPackageData::pActor), 0);
        assert_eq!(calls(&log, 0x005e_04f0), vec![vec![this.addr() + 0x1c]]);
        assert_eq!(calls(&log, 0x0067_d970), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_009f2b30_needs_an_index_inside_the_array() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        let this = patrol(&mut e, &[0x100, 0]);
        for (current, expected) in [(0u32, vec![vec![0x101, 0]]), (1, vec![]), (2, vec![])] {
            e.set(this, PatrolActorPackageData::iCurrentReference, current);
            e.call_log = Some(vec![]);
            e.call(0x009f_2b30, &args![this]);
            assert_eq!(
                calls(&e.call_log.take().unwrap(), SET_PATROL_REF_USER),
                expected
            );
        }
    }

    #[test]
    fn get_initial_reference_prefers_the_package_location() {
        let mut e = Engine::new();
        e.register(0x0055_b980, |_, a| (a[0] + 1).into_ret());
        e.register(0x0067_f390, |_, a| {
            if a[0] == 0x201 { 0x300u32 } else { 0 }.into_ret()
        });
        e.register(0x0056_9b80, |_, _| 0x400u32.into_ret());
        assert_eq!(e.call(0x009f_2d40, &args![0x10u32, 0x200u32]).u32(), 0x300);
        assert_eq!(e.call(0x009f_2d40, &args![0x10u32, 0x210u32]).u32(), 0x400);
        e.register(0x0055_b980, |_, _| 0u32.into_ret());
        e.register(0x0056_9b80, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x009f_2d40, &args![0x10u32, 0x200u32]).u32(), 0);
    }

    fn chain_doubles(e: &mut Engine, last_points_back_to: u32) {
        // The chain 0x100 -> 0x200 -> 0x300 -> `last_points_back_to`.
        e.register(0x0055_b980, |_, _| 0x80u32.into_ret());
        e.register(0x0067_f390, |_, _| 0x100u32.into_ret());
        e.register_double(0x0056_9b80, move |_, a| {
            match a[0] {
                0x100 => 0x200,
                0x200 => 0x300,
                0x300 => last_points_back_to,
                _ => 0,
            }
            .into_ret()
        });
        quiet(e, &[0x0056_4db0]);
    }

    #[test]
    fn build_reference_array_follows_the_chain() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        chain_doubles(&mut e, 0);
        let this = patrol(&mut e, &[0x999]);
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_2da0, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 1);
        let buffer = e.mem.u32(this.addr() + REFERENCES + 4);
        assert_eq!(e.mem.u32(this.addr() + REFERENCES + 8), 3);
        assert_eq!(
            (
                e.mem.u32(buffer),
                e.mem.u32(buffer + 4),
                e.mem.u32(buffer + 8)
            ),
            (0x100, 0x200, 0x300)
        );
        assert_eq!(e.get(this, PatrolActorPackageData::bCircularArray), 0);
        assert_eq!(
            calls(&log, 0x0056_4db0),
            vec![vec![0x100, 1], vec![0x200, 1], vec![0x300, 1]]
        );
    }

    #[test]
    fn build_reference_array_closes_a_loop() {
        // One-way patrol (no patrol data): the repeated reference is added again.
        let mut e = Engine::new();
        array_doubles(&mut e);
        chain_doubles(&mut e, 0x200);
        let this = patrol(&mut e, &[]);
        assert_eq!(e.call(0x009f_2da0, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(this.addr() + REFERENCES + 8), 4);
        assert_eq!(e.get(this, PatrolActorPackageData::bCircularArray), 0);
        // Repeating patrol: the loop is recorded instead.
        e.set(this, PatrolActorPackageData::pPackageData, 0x77);
        e.register(PATROL_REPEATS, |_, _| true.into_ret());
        assert_eq!(e.call(0x009f_2da0, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(this.addr() + REFERENCES + 8), 3);
        assert_eq!(e.get(this, PatrolActorPackageData::bCircularArray), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::iLoopAroundIndex), 1);
        // No reference at all: 0.
        e.register(0x0067_f390, |_, _| 0u32.into_ret());
        e.register(0x0056_9b80, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x009f_2da0, &args![this]).u32(), 0);
    }

    #[test]
    fn init_stores_the_package_and_resumes_a_repeating_patrol() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        chain_doubles(&mut e, 0);
        e.register(0x0067_5920, |_, _| 0x77u32.into_ret());
        e.register(PATROL_REPEATS, |_, _| false.into_ret());
        let this = patrol(&mut e, &[]);
        e.set(this, PatrolActorPackageData::iCurrentReference, 5);
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_2ba0, &args![this, 0x50u32, 0x60u32]).u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 1);
        assert_eq!(e.get(this, PatrolActorPackageData::pActor), 0x50);
        assert_eq!(e.get(this, PatrolActorPackageData::pPackage), 0x60);
        assert_eq!(e.get(this, PatrolActorPackageData::pPackageData), 0x77);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 0);
        assert_eq!(e.get(this, PatrolActorPackageData::bPatrolDirection), 1);
        // Not repeating: the actor's position is not asked for.
        assert_eq!(calls(&log, PATROL_REPEATS).len(), 2);
    }

    #[test]
    fn get_current_reference_reads_the_current_slot() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = patrol(&mut e, &[0x100, 0x200]);
        e.set(this, PatrolActorPackageData::iCurrentReference, 1);
        assert_eq!(e.call(0x009f_3390, &args![this]).u32(), 0x200);
        let empty = patrol(&mut e, &[]);
        assert_eq!(e.call(0x009f_3390, &args![empty]).u32(), 0);
    }

    #[test]
    fn set_current_reference_checks_a_circular_array() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![0x301]);
        e.register(0x0064_7ba0, |e, a| {
            let value = e.mem.u32(a[0]).clamp(a[1], a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let this = patrol(&mut e, &[0x100, 0x200, 0x300]);
        e.set(this, PatrolActorPackageData::pActor, 0x50);
        // Not circular: taken as it is.
        assert_eq!(e.call(0x009f_30d0, &args![this, 9u32]).u8(), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 9);
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        e.set(this, PatrolActorPackageData::iCurrentReference, 0);
        e.call_log = Some(vec![]);
        // Clamped to 2, whose list refuses the actor.
        assert_eq!(e.call(0x009f_30d0, &args![this, 9u32]).u8(), 0);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 0);
        // Index 1 accepts: the old one is released, the new one taken.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_30d0, &args![this, 1u32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SET_PATROL_REF_USER),
            vec![vec![0x101, 0], vec![0x201, 0x50]]
        );
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
    }

    #[test]
    fn set_next_reference_walks_and_turns() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        let this = patrol(&mut e, &[0x100, 0x200, 0x300]);
        e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
        // One-way, forward: 0 -> 1 -> 2 -> end (0, result 0).
        for expected in [(1u32, 1u8), (2, 1), (0, 0)] {
            let result = e.call(0x009f_2ea0, &args![this, 0u32]).u8();
            let index = e.get(this, PatrolActorPackageData::iCurrentReference);
            assert_eq!((index, result), expected);
        }
        // Repeating, not circular: at the end it turns back.
        e.set(this, PatrolActorPackageData::pPackageData, 0x77);
        e.register(PATROL_REPEATS, |_, _| true.into_ret());
        e.set(this, PatrolActorPackageData::iCurrentReference, 2);
        assert_eq!(e.call(0x009f_2ea0, &args![this, 0u32]).u8(), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::bPatrolDirection), 0);
        // Backward: 1 -> 0 -> turn forward to 1.
        e.call(0x009f_2ea0, &args![this, 0u32]);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 0);
        e.call(0x009f_2ea0, &args![this, 0u32]);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::bPatrolDirection), 1);
        // Circular: forward at the end goes to the loop index; backward at -1 ends.
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        e.set(this, PatrolActorPackageData::iLoopAroundIndex, 1);
        e.set(this, PatrolActorPackageData::iCurrentReference, 2);
        e.call(0x009f_2ea0, &args![this, 0u32]);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        e.set(this, PatrolActorPackageData::bPatrolDirection, 0);
        e.set(this, PatrolActorPackageData::iCurrentReference, 0);
        assert_eq!(e.call(0x009f_2ea0, &args![this, 0u32]).u8(), 0);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 0);
    }

    #[test]
    fn set_next_reference_with_apply_retries_when_nothing_is_usable() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        // Every list refuses the actor.
        extra_doubles(&mut e, vec![0x101, 0x201, 0x301]);
        e.register(0x0064_7ba0, |_, _| Ret::default());
        let this = patrol(&mut e, &[0x100, 0x200, 0x300]);
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
        e.call_log = Some(vec![]);
        // 0 -> 1 is refused; nothing else usable: the retry without apply stores 1.
        let result = e.call(0x009f_2ea0, &args![this, 0x101u32]).u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 1);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        assert_eq!(calls(&log, REFERENCE_EXTRA_DATA).len(), 1 + 3);
        // One other reference usable: the refused step only returns its result.
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![0x201]);
        e.register(0x0064_7ba0, |_, _| Ret::default());
        let this = patrol(&mut e, &[0x100, 0x200, 0x300]);
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        e.set(this, PatrolActorPackageData::bPatrolDirection, 1);
        assert_eq!(e.call(0x009f_2ea0, &args![this, 1u32]).u8(), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 0);
    }

    #[test]
    fn handle_deleted_reference_of_the_actor_clears_it() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        let this = patrol(&mut e, &[0x100]);
        e.set(this, PatrolActorPackageData::pActor, 0x50);
        e.set(this, PatrolActorPackageData::pPackage, 0x60);
        e.call(0x009f_2c30, &args![this, 0x50u32]);
        assert_eq!(e.get(this, PatrolActorPackageData::eState), 3);
        assert_eq!(e.get(this, PatrolActorPackageData::pActor), 0);
        assert_eq!(e.get(this, PatrolActorPackageData::pPackage), 0);
    }

    #[test]
    fn handle_deleted_reference_removes_every_occurrence() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        e.map(0x011d_d000, 0x1000);
        e.register(0x0042_ce10, |_, _| true.into_ret());
        let this = patrol(&mut e, &[0x200, 0x100, 0x200, 0x300]);
        e.set(this, PatrolActorPackageData::pActor, 0x50);
        e.set(this, PatrolActorPackageData::iCurrentReference, 3);
        e.call(0x009f_2c30, &args![this, 0x200u32]);
        let buffer = e.mem.u32(this.addr() + REFERENCES + 4);
        assert_eq!(e.mem.u32(this.addr() + REFERENCES + 8), 2);
        assert_eq!((e.mem.u32(buffer), e.mem.u32(buffer + 4)), (0x100, 0x300));
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        // The current index 0 steps back past 0 when its element goes.
        let this = patrol(&mut e, &[0x200, 0x100]);
        e.call(0x009f_2c30, &args![this, 0x200u32]);
        assert_eq!(
            e.get(this, PatrolActorPackageData::iCurrentReference),
            u32::MAX
        );
    }

    #[test]
    fn handle_deleted_reference_resumes_when_the_current_reference_is_refused() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.map(0x011d_d000, 0x1000);
        e.register(0x0042_ce10, |_, _| false.into_ret());
        e.register(0x0064_7ba0, |_, _| Ret::default());
        // The actor and both references answer the position slot.
        let mut objects = vec![];
        for _ in 0..3 {
            let object = e.mem.alloc(4);
            let vtable = e.mem.alloc(0x1f8);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(vtable + GET_POSITION_SLOT, 0x7000_01f4);
            objects.push(object);
        }
        let actor = objects[0];
        e.map(0x0101_6000, 0x1000);
        extra_doubles(&mut e, vec![objects[1] + 1, objects[2] + 1]);
        e.register(0x7000_01f4, |_, a| (a[0] + 0x10).into_ret());
        let this = patrol(&mut e, &objects[1..]);
        e.set(this, PatrolActorPackageData::pActor, actor);
        e.set(this, PatrolActorPackageData::bCircularArray, 1);
        // Every step is refused: the walk measures nothing and ends forward.
        e.call_log = Some(vec![]);
        e.call(0x009f_2c30, &args![this, 0x999u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x7000_01f4)[0], vec![actor]);
        assert!(calls(&log, 0x0059_5c80).is_empty());
        assert_eq!(e.get(this, PatrolActorPackageData::bPatrolDirection), 1);
    }

    #[test]
    fn resume_repeatable_patrol_picks_the_closest_step() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        extra_doubles(&mut e, vec![]);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(0x0101_6970, f32::MAX.to_bits());
        // Each reference's position is the reference plus 0x10.
        e.register(0x7000_01f4, |_, a| (a[0] + 0x10).into_ret());
        let mut refs = vec![];
        for _ in 0..3 {
            let reference = e.mem.alloc(4);
            let vtable = e.mem.alloc(0x1f8);
            e.mem.set_u32(reference, vtable);
            e.mem.set_u32(vtable + GET_POSITION_SLOT, 0x7000_01f4);
            refs.push(reference);
        }
        let this = patrol(&mut e, &refs);
        e.set(this, PatrolActorPackageData::pActor, 0x50);
        // The difference writes the first word's distance: from reference 1's position the
        // distance is 1.0, from the others 5.0.
        let second = refs[1] + 0x10;
        e.register_double(0x0043_9ef0, move |e, a| {
            let length = if a[0] == second { 1.0f32 } else { 5.0 };
            e.mem.set_u32(a[1], length.to_bits());
            Ret::default()
        });
        e.register(0x0059_5c80, |e, a| Ret {
            st0: f64::from(f32::from_bits(e.mem.u32(a[0]))),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_31c0, &args![this, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        // Steps from 0 to 1, 1 to 2, then the one-way end back at 0.
        let differences = calls(&log, 0x0043_9ef0);
        assert_eq!(differences.len(), 6);
        assert_eq!(differences[0][0], refs[0] + 0x10);
        assert_eq!(differences[0][2], 0x4444);
        assert_eq!(differences[1][0], refs[1] + 0x10);
        assert_eq!(differences[1][2], refs[0] + 0x10);
        // The step from index 1 measured 1.0: index 1 is current again.
        assert_eq!(e.get(this, PatrolActorPackageData::iCurrentReference), 1);
        assert_eq!(e.get(this, PatrolActorPackageData::bPatrolDirection), 1);
        // Without an actor nothing happens.
        let idle = patrol(&mut e, &refs);
        e.call_log = Some(vec![]);
        e.call(0x009f_31c0, &args![idle, 0x4444u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn save_game_writes_the_fields_and_the_references() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        quiet(&mut e, &[SAVE_BYTES, SAVE_FORM_ID, 0x0086_5f60]);
        let this = patrol(&mut e, &[0x100, 0x200]);
        e.set(this, PatrolActorPackageData::pPackage, 0x60);
        e.call_log = Some(vec![]);
        e.call(0x009f_33d0, &args![this, 0x900u32]);
        let log = e.call_log.take().unwrap();
        let base = this.addr();
        assert_eq!(
            calls(&log, SAVE_BYTES),
            vec![
                vec![0x900, base + 4, 4, 0],
                vec![0x900, base + 8, 4, 0],
                vec![0x900, base + 0x34, 4, 0],
                vec![0x900, base + 0xc, 1, 0],
                vec![0x900, base + 0x2c, 1, 0],
                vec![0x900, base + 0x30, 4, 0],
            ]
        );
        assert_eq!(calls(&log, 0x0086_5f60), vec![vec![0x900, 2]]);
        assert_eq!(
            calls(&log, SAVE_FORM_ID),
            vec![
                vec![0x900, 0x60, 0],
                vec![0x900, 0x100, 0],
                vec![0x900, 0x200, 0]
            ]
        );
    }
}
