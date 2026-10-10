//! `fallout/ai/aitaskmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The AI task manager runs the per-actor work of a frame as "tasklets"
//! (`MobileObjectTaskletData` and its five subclasses, each embedded in the
//! manager: Detection at +0x00, Animation +0x40, PackageUpdate +0x78,
//! ActorUpdate +0xBC, ActorsScript +0xF8, Movement +0x138). A tasklet owns a
//! message queue (at +0x1C) that the producer fills with count, mob and end
//! messages; `Process` / `RunToCompletion` drain it and dispatch each message
//! to the subclass's `HandleCountMessage` / `HandleMobMessage` /
//! `HandleEndMessage` (vtable slots +0x10 / +0x14 / +0x18).
//!
//! Layouts: the PC class is 0x38 bytes where the Xbox one is 0x58 (the Xbox
//! timing fields `StartTime`, `EndTime`, ... and `iThread` are not in the PC
//! build), so the subclass fields start at +0x38 here.
//!
//! Not translated: the compiler's exception-unwinding frames (`FS:[0]`
//! chains) in the constructors, the destructor `83a0` and
//! `PackageUpdateTaskData::HandleMobMessage`.
//!
//! The critical sections the count and end handlers enter and leave are
//! globals of other units; their roles are not named here.

#[allow(unused_imports)]
use crate::prelude::*;

/// The global holding the `PlayerCharacter` pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// `operator new(size)` (`00401000`) and `operator delete(ptr)` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Enters a critical section (`004538a0`, `this` = the section, one word
/// argument passed as 0) and leaves it (`004538c0`).
const CRITICAL_SECTION_ENTER: u32 = 0x0045_38a0;
const CRITICAL_SECTION_LEAVE: u32 = 0x0045_38c0;
/// The critical sections used by the tasklets' count/end handlers.
const CRITICAL_SECTION_011DFBE8: u32 = 0x011d_fbe8;
const CRITICAL_SECTION_011DFC04: u32 = 0x011d_fc04;
const CRITICAL_SECTION_011DFC38: u32 = 0x011d_fc38;

/// Vtables of the tasklet classes (RTTI locator in the word before).
const VTABLE_MOBILE_OBJECT_TASKLET_DATA: u32 = 0x0108_5748;
const VTABLE_DETECTION_TASK_DATA: u32 = 0x0108_5728;
const VTABLE_ANIMATION_TASK_DATA: u32 = 0x0108_5768;
const VTABLE_ACTOR_UPDATE_TASK_DATA: u32 = 0x0108_5788;
const VTABLE_PACKAGE_UPDATE_TASK_DATA: u32 = 0x0108_57a8;
const VTABLE_ACTORS_SCRIPT_TASK_DATA: u32 = 0x0108_57c8;

/// `BSXenonTaskletData` constructor (`006c7850`): the base of the tasklet.
const TASKLET_BASE_CONSTRUCTOR: u32 = 0x006c_7850;
/// `BSTCommonLLMessageQueue<MobileObjectMessage>` constructor (`008ca440`,
/// argument: the owner, here the manager's shared word) and destructor body
/// (`008ca480`).
const MESSAGE_QUEUE_CONSTRUCTOR: u32 = 0x008c_a440;
const MESSAGE_QUEUE_DESTRUCTOR: u32 = 0x008c_a480;
/// `BSXenonTaskletData` destructor body (`00b00ec0`).
const TASKLET_BASE_DESTRUCTOR: u32 = 0x00b0_0ec0;
/// Sets the "yielding" byte (+4) of a tasklet (`006ea330`).
const TASKLET_SET_YIELDING: u32 = 0x006e_a330;

/// `MobileObject` (here an `Actor`) members the handlers read.
/// `Actor::bProcessMe` (Xbox PDB `+0xCC`; PC `+0xBC`).
const ACTOR_PROCESS_ME: u32 = 0xbc;
/// `MobileObject::IsActor`-style test: vtable slot +0x100 answers true for
/// actors (the handlers use it to cast the mob).
const VSLOT_IS_ACTOR: u32 = 0x100;
/// `MobileObject::GetCurrentProcessType` (`00931850`).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MiddleHighProcess::GetSavedAcquireObject`'s body (`008d8520`): the map
/// names it so by identical-code folding; it returns the word at +0x68 of
/// the object (the actor's process).
const GET_PROCESS: u32 = 0x008d_8520;

layout! {
    /// `MobileObjectMessage` (Xbox PDB), 0xC bytes.
    pub struct MobileObjectMessage: 0x0C {
        /// `eMsgType` (Xbox PDB): 0 = count, 1 = mob, 2 = end, as the
        /// dispatch `fn_008c81c0` uses them.
        0x00 eMsgType: u32,
        /// `uiCount` (Xbox PDB).
        0x04 uiCount: u32,
        /// `pMob` (Xbox PDB): `MobileObject*`.
        0x08 pMob: Ptr,
    }

    /// `MobileObjectTaskletData` (Xbox PDB), 0x38 bytes on the PC (0x58 on
    /// the Xbox, which has timing fields after `eType`).
    pub struct MobileObjectTaskletData: 0x38 {
        /// `BSTaskletData::bYielding` (Xbox PDB).
        0x04 bYielding: bool,
        /// `pmoblist` (Xbox PDB): `BSSimpleList<MobileObject *>*`.
        0x18 pmoblist: Ptr,
        /// `uiHighActorCount` (Xbox PDB): set by a count message.
        0x30 uiHighActorCount: u32,
        /// `eType` (Xbox PDB, `AITaskType`): passed to `HandleEndMessage`.
        0x34 eType: u32,
    }

    /// `DetectionTaskData` (Xbox PDB).
    pub struct DetectionTaskData: 0x40 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// `fNumber` (Xbox PDB).
        0x38 fNumber: f32,
        /// `iUpdated` (Xbox PDB).
        0x3C iUpdated: i32,
    }

    /// `AnimationTaskData` (Xbox PDB): no fields of its own.
    pub struct AnimationTaskData: 0x38 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
    }

    /// `ActorUpdateTaskData` (Xbox PDB).
    pub struct ActorUpdateTaskData: 0x3C {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// `iUpdated` (Xbox PDB).
        0x38 iUpdated: i32,
    }

    /// `PackageUpdateTaskData` (Xbox PDB): two counters of its own (the
    /// Xbox PDB lists none for this build; the names follow the uses).
    pub struct PackageUpdateTaskData: 0x44 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// Number of mobs sent to `vtable +0x25C` (the plain update).
        0x3C iPackageUpdated: i32,
        /// Number of actors handed to the process (`vtable +0x24`).
        0x40 iActorsProcessed: i32,
    }

    /// `ActorsScriptTaskData` (Xbox PDB).
    pub struct ActorsScriptTaskData: 0x40 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// Number of mobs whose script ran.
        0x3C iScriptsRun: i32,
    }
}

/// Reads a word as the pointer it holds.
fn player_character(e: &Engine) -> Ptr {
    Ptr::new(e.global::<u32>(PLAYER_CHARACTER))
}

fn is_actor(e: &mut Engine, mob: Ptr) -> bool {
    e.vcall(mob.addr(), VSLOT_IS_ACTOR, &[]).bool()
}

/// The object (`011f6394`) whose float at +0xC (`0084d030`) scales the
/// per-actor delay and is passed to the process lists at the end of the
/// package update.
const TASK_SCALE_OBJECT_011F6394: u32 = 0x011f_6394;
/// A `float` the detection handler caps its random delay with.
const DELAY_CAP: u32 = 0x0101_712c;
/// `0.0` as a `double`.
const ZERO: u32 = 0x0101_2060;
/// A `float`: the amount restored to an actor of a resting player.
const RESTORE_AMOUNT: u32 = 0x0101_62c0;
/// A global whose value is `this` for `0047c850`.
const OBJECT_011DE45C: u32 = 0x011d_e45c;
/// The global holding the fader manager pointer (`007014e0` is
/// `FaderManager::GetFaderAlpha`).
const FADER_MANAGER: u32 = 0x011d_8804;
/// Game settings read with `00403e20` (a pointer to the setting's float):
/// the distance below which an actor is inserted close to the player, and
/// the detection level the actor is compared with.
const SETTING_011CCF94: u32 = 0x011c_cf94;
const SETTING_011D13C8: u32 = 0x011d_13c8;
/// The `ProcessLists` object (`0096e3c0` is
/// `ProcessLists::InsertActorCloseToPlayer`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// A global byte cleared by `fn_008c8bd0`.
const RESET_FLAG_011E01DD: u32 = 0x011e_01dd;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c8130,
            mobile_object_tasklet_data_process(Ptr<MobileObjectTaskletData>)
        ),
        entry!(
            0x008c8190,
            fn_008c8190(Ptr<MobileObjectMessage>) -> Ptr<MobileObjectMessage>
        ),
        entry!(
            0x008c81c0,
            fn_008c81c0(Ptr<MobileObjectTaskletData>, Ptr<MobileObjectMessage>, u32) -> bool
        ),
        entry!(
            0x008c8240,
            mobile_object_tasklet_data_run_to_completion(Ptr<MobileObjectTaskletData>, u32)
        ),
        entry!(
            0x008c82a0,
            fn_008c82a0(Ptr<DetectionTaskData>, u32) -> Ptr<DetectionTaskData>
        ),
        entry!(
            0x008c82e0,
            fn_008c82e0(Ptr<MobileObjectTaskletData>, u32) -> Ptr<MobileObjectTaskletData>
        ),
        entry!(
            0x008c8370,
            fn_008c8370(Ptr<MobileObjectTaskletData>, u32) -> Ptr<MobileObjectTaskletData>
        ),
        entry!(0x008c83a0, fn_008c83a0(Ptr<MobileObjectTaskletData>)),
        entry!(
            0x008c8400,
            detection_task_data_scalar_deleting_destructor(
                Ptr<DetectionTaskData>,
                u32,
            ) -> Ptr<DetectionTaskData>
        ),
        entry!(0x008c8430, fn_008c8430(Ptr<DetectionTaskData>)),
        entry!(
            0x008c8450,
            detection_task_data_handle_count_message(Ptr<DetectionTaskData>, u32)
        ),
        entry!(
            0x008c8490,
            detection_task_data_handle_end_message(Ptr<DetectionTaskData>, u32)
        ),
        entry!(
            0x008c84c0,
            detection_task_data_handle_mob_message(Ptr<DetectionTaskData>, Ptr)
        ),
        entry!(
            0x008c8830,
            fn_008c8830(Ptr<AnimationTaskData>, u32) -> Ptr<AnimationTaskData>
        ),
        entry!(
            0x008c8860,
            animation_task_data_scalar_deleting_destructor(
                Ptr<AnimationTaskData>,
                u32,
            ) -> Ptr<AnimationTaskData>
        ),
        entry!(0x008c8890, fn_008c8890(Ptr<AnimationTaskData>)),
        entry!(
            0x008c88b0,
            animation_task_data_handle_count_message(Ptr<AnimationTaskData>, u32)
        ),
        entry!(
            0x008c88e0,
            animation_task_data_handle_end_message(Ptr<AnimationTaskData>, u32)
        ),
        entry!(
            0x008c8910,
            animation_task_data_handle_mob_message(Ptr<AnimationTaskData>, Ptr)
        ),
        entry!(
            0x008c89a0,
            fn_008c89a0(Ptr<ActorUpdateTaskData>, u32) -> Ptr<ActorUpdateTaskData>
        ),
        entry!(
            0x008c89d0,
            actor_update_task_data_scalar_deleting_destructor(
                Ptr<ActorUpdateTaskData>,
                u32,
            )
                -> Ptr<ActorUpdateTaskData>
        ),
        entry!(0x008c8a00, fn_008c8a00(Ptr<ActorUpdateTaskData>)),
        entry!(
            0x008c8a20,
            actor_update_task_data_handle_count_message(Ptr<ActorUpdateTaskData>, u32)
        ),
        entry!(
            0x008c8a50,
            actor_update_task_data_handle_end_message(Ptr<ActorUpdateTaskData>, u32)
        ),
        entry!(
            0x008c8a70,
            actor_update_task_data_handle_mob_message(Ptr<ActorUpdateTaskData>, Ptr)
        ),
        entry!(
            0x008c8ae0,
            fn_008c8ae0(Ptr<PackageUpdateTaskData>, u32) -> Ptr<PackageUpdateTaskData>
        ),
        entry!(
            0x008c8b20,
            package_update_task_data_scalar_deleting_destructor(
                Ptr<PackageUpdateTaskData>,
                u32,
            )
                -> Ptr<PackageUpdateTaskData>
        ),
        entry!(0x008c8b50, fn_008c8b50(Ptr<PackageUpdateTaskData>)),
        entry!(
            0x008c8b70,
            package_update_task_data_handle_count_message(Ptr<PackageUpdateTaskData>, u32)
        ),
        entry!(0x008c8bb0, fn_008c8bb0(Ptr, u8)),
        entry!(0x008c8bd0, fn_008c8bd0()),
        entry!(
            0x008c8be0,
            package_update_task_data_handle_mob_message(Ptr<PackageUpdateTaskData>, Ptr)
        ),
        entry!(0x008c8fd0, fn_008c8fd0(Ptr, u8)),
        entry!(
            0x008c8ff0,
            package_update_task_data_handle_end_message(Ptr<PackageUpdateTaskData>, u32)
        ),
        entry!(
            0x008c9020,
            fn_008c9020(Ptr<ActorsScriptTaskData>, u32) -> Ptr<ActorsScriptTaskData>
        ),
        entry!(
            0x008c9050,
            actors_script_task_data_scalar_deleting_destructor(
                Ptr<ActorsScriptTaskData>,
                u32,
            )
                -> Ptr<ActorsScriptTaskData>
        ),
        entry!(0x008c9080, fn_008c9080(Ptr<ActorsScriptTaskData>)),
        entry!(
            0x008c90a0,
            actors_script_task_data_handle_count_message(Ptr<ActorsScriptTaskData>, u32)
        ),
        entry!(
            0x008c90e0,
            actors_script_task_data_handle_end_message(Ptr<ActorsScriptTaskData>, u32)
        ),
        entry!(
            0x008c9110,
            actors_script_task_data_handle_mob_message(Ptr<ActorsScriptTaskData>, Ptr)
        ),
    ]
}

// Translated from 008c8130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData::Process` (Xbox PDB): drains the message queue
/// (queue vtable slot +0x10 with a message buffer), dispatching each message
/// with the tasklet's `eType`. When the queue runs dry it marks the tasklet
/// as yielding (`006ea330`); when a message ends the run (an end message) it
/// returns without doing so.
pub fn mobile_object_tasklet_data_process(e: &mut Engine, this: Ptr<MobileObjectTaskletData>) {
    e.with_stack(12, |e, buffer| {
        let msg: Ptr<MobileObjectMessage> = buffer.cast();
        fn_008c8190(e, msg);
        loop {
            if !e.vcall(this.addr() + 0x1c, 0x10, &args![msg]).bool() {
                e.call(TASKLET_SET_YIELDING, &args![this]);
                return;
            }
            let end_arg = e.get(this, MobileObjectTaskletData::eType);
            if !fn_008c81c0(e, this, msg, end_arg) {
                return;
            }
        }
    })
}

// Translated from 008c8190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a `MobileObjectMessage`: type 2 (end), count 0, no mob.
/// Returns `msg`.
pub fn fn_008c8190(e: &mut Engine, msg: Ptr<MobileObjectMessage>) -> Ptr<MobileObjectMessage> {
    e.set(msg, MobileObjectMessage::eMsgType, 2);
    e.set(msg, MobileObjectMessage::uiCount, 0);
    e.set(msg, MobileObjectMessage::pMob, Ptr::NULL);
    msg
}

// Translated from 008c81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispatches one `MobileObjectMessage`: a count message stores the count in
/// `uiHighActorCount` and calls `HandleCountMessage` (slot +0x10) with it; a
/// mob message calls `HandleMobMessage` (+0x14) with the mob; an end message
/// calls `HandleEndMessage` (+0x18) with `end_arg` and returns false. Any
/// other type does nothing. Returns true to keep draining the queue.
pub fn fn_008c81c0(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    msg: Ptr<MobileObjectMessage>,
    end_arg: u32,
) -> bool {
    match e.get(msg, MobileObjectMessage::eMsgType) {
        0 => {
            let count = e.get(msg, MobileObjectMessage::uiCount);
            e.set(this, MobileObjectTaskletData::uiHighActorCount, count);
            e.vcall(this.addr(), 0x10, &args![count]);
            true
        }
        1 => {
            let mob = e.get(msg, MobileObjectMessage::pMob);
            e.vcall(this.addr(), 0x14, &args![mob]);
            true
        }
        2 => {
            e.vcall(this.addr(), 0x18, &args![end_arg]);
            false
        }
        _ => true,
    }
}

// Translated from 008c8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData::RunToCompletion` (Xbox PDB): stores `end_arg`
/// as `eType`, then drains the queue (queue vtable slot +0x0C, `Process`
/// uses +0x10), dispatching until the queue is empty or an end message
/// arrives.
pub fn mobile_object_tasklet_data_run_to_completion(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    end_arg: u32,
) {
    e.set(this, MobileObjectTaskletData::eType, end_arg);
    e.with_stack(12, |e, buffer| {
        let msg: Ptr<MobileObjectMessage> = buffer.cast();
        fn_008c8190(e, msg);
        loop {
            if !e.vcall(this.addr() + 0x1c, 0xc, &args![msg]).bool() {
                return;
            }
            if !fn_008c81c0(e, this, msg, end_arg) {
                return;
            }
        }
    })
}

// Translated from 008c82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData` constructor: the base constructor, the Detection
/// vtable, then `fNumber` and `iUpdated` cleared. `shared` is the manager's
/// object (+0x174). Returns `this`.
pub fn fn_008c82a0(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    shared: u32,
) -> Ptr<DetectionTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_DETECTION_TASK_DATA);
    e.set(this, DetectionTaskData::fNumber, 0.0);
    e.set(this, DetectionTaskData::iUpdated, 0);
    this
}

// Translated from 008c82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData` constructor: the `BSXenonTaskletData` base
/// constructor, this class's vtable, an empty mob list, the message queue
/// constructed at +0x1C, and `uiHighActorCount` / `eType` cleared. The
/// compiler's exception frame is not translated. Returns `this`.
pub fn fn_008c82e0(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    shared: u32,
) -> Ptr<MobileObjectTaskletData> {
    e.call(TASKLET_BASE_CONSTRUCTOR, &args![this]);
    e.mem
        .set_u32(this.addr(), VTABLE_MOBILE_OBJECT_TASKLET_DATA);
    e.set(this, MobileObjectTaskletData::pmoblist, Ptr::NULL);
    e.call(
        MESSAGE_QUEUE_CONSTRUCTOR,
        &args![this.addr() + 0x1c, shared],
    );
    e.set(this, MobileObjectTaskletData::uiHighActorCount, 0);
    e.set(this, MobileObjectTaskletData::eType, 0);
    this
}

// Translated from 008c8370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData`'s scalar deleting destructor (vtable slot 0 of
/// `01085748`; the map has no name): runs the destructor body `83a0` and,
/// when bit 0 of `flags` is set, `operator delete`. Returns `this`.
pub fn fn_008c8370(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    flags: u32,
) -> Ptr<MobileObjectTaskletData> {
    fn_008c83a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body shared by the tasklet classes: destroys the message queue
/// at +0x1C, then the `BSXenonTaskletData` base. The compiler's exception
/// frame is not translated.
pub fn fn_008c83a0(e: &mut Engine, this: Ptr<MobileObjectTaskletData>) {
    e.call(MESSAGE_QUEUE_DESTRUCTOR, &args![this.addr() + 0x1c]);
    e.call(TASKLET_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 008c8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8430`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn detection_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    flags: u32,
) -> Ptr<DetectionTaskData> {
    fn_008c8430(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData` destructor body: resets the vtable to the Detection
/// one, then runs `83a0`.
pub fn fn_008c8430(e: &mut Engine, this: Ptr<DetectionTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_DETECTION_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleCountMessage` (Xbox PDB): clears `fNumber` and
/// `iUpdated` and enters the critical sections `011dfc04` and `011dfbe8`
/// (the count it is given is not used).
pub fn detection_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    _unused_1: u32,
) {
    e.set(this, DetectionTaskData::fNumber, 0.0);
    e.set(this, DetectionTaskData::iUpdated, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleEndMessage` (Xbox PDB): lets the player update
/// its auto-aim actor (`PlayerCharacter::UpdateAutoAimActor`, `this` = the
/// player), then leaves the critical sections `011dfc04` and `011dfbe8`
/// (the end argument is not used).
pub fn detection_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<DetectionTaskData>,
    _unused_1: u32,
) {
    let player = player_character(e);
    e.call(0x0096_4260, &args![player]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
}

// Translated from 008c84c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleMobMessage` (Xbox PDB): the per-actor detection
/// step.
///
/// First it draws a random delay: the float at +0xC of `011f6394` (read with
/// `0084d030`) times `uiHighActorCount`, capped at `0101712c`, then a
/// uniform random in `[0, that]`. If the mob is an actor that is flagged
/// `bProcessMe`, is not hidden by its extra data (`004181e0` +0x30, slot
/// +0x20), is not the player and passes `0047c850`:
/// - when its slot +0x2B4 value is not negative, or `00437bd0` or slot
///   +0x22C(0) say so, the "idle" path runs: a negative slot +0x2B4 value
///   removes it from the player's perceived actors (`00967350`), and slot
///   +0x2B8 is called;
/// - otherwise, if the fader alpha is not positive, slot +0x2B0 gets the
///   random delay and, when its process (slot +0x70) accepts the mob, the
///   detection level against the player (`008a0d10`) and the attack/combat
///   tests (`008b06d0`, `008bc700`, the extra-data checks `00705cf0` /
///   `00825c00`) decide whether `00966f20` is told the mob is hostile, and
///   the mob is inserted into the close-to-player list when nearer than the
///   `011ccf94` setting, or dropped from the perceived actors.
///
/// Any actor, even one that is skipped, adds one to `iUpdated`.
pub fn detection_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    mob: Ptr,
) {
    let count = e.get(this, DetectionTaskData::uiHighActorCount);
    let scale = e
        .call(0x0084_d030, &args![TASK_SCALE_OBJECT_011F6394])
        .f32();
    let mut delay = (scale as f64 * count as f64) as f32;
    let delay_cap: f32 = e.global(DELAY_CAP);
    delay = e.call(0x0040_ebd0, &args![delay, delay_cap]).f32();
    delay = e.call(0x0047_6b70, &args![0.0f32, delay]).f32();

    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    let actor = mob;
    let player = player_character(e);
    if e.mem.u8(actor.addr() + ACTOR_PROCESS_ME) != 0 {
        // The extra-data list's embedded object at +0x30 (slot +0x20).
        let extra_data = e.call(0x0041_81e0, &args![actor]).u32() + 0x30;
        if !e.vcall(extra_data, 0x20, &[]).bool() {
            let distance = e.call(0x0057_23b0, &args![actor, player, 0u32, 0u32]).f32();
            let object_011de45c = e.global::<u32>(OBJECT_011DE45C);
            if actor != player && !e.call(0x0047_c850, &args![object_011de45c]).bool() {
                let zero: f64 = e.global(ZERO);
                // x87 compare: "not below zero" is also true for NaN.
                let value = e.vcall(actor.addr(), 0x2b4, &[]).f64();
                let mut idle = value.partial_cmp(&zero) != Some(std::cmp::Ordering::Less);
                idle = idle || e.call(0x0043_7bd0, &args![actor]).bool();
                idle = idle || e.vcall(actor.addr(), 0x22c, &args![0u32]).bool();
                if idle {
                    if e.vcall(actor.addr(), 0x2b4, &[]).f64() < zero {
                        e.call(0x0096_7350, &args![player, actor]);
                    }
                    e.vcall(actor.addr(), 0x2b8, &[]);
                } else {
                    let fader = e.global::<u32>(FADER_MANAGER);
                    let alpha = e.call(0x0070_14e0, &args![fader, 1u32]).f64();
                    if alpha <= zero {
                        detect_against_player(e, actor, player, distance, delay);
                    }
                }
            }
        }
    }
    let updated = e.get(this, DetectionTaskData::iUpdated);
    e.set(this, DetectionTaskData::iUpdated, updated.wrapping_add(1));
}

/// The part of `DetectionTaskData::HandleMobMessage` that runs when the
/// fader alpha is not positive: tells the actor the delay (slot +0x2B0),
/// asks its process whether it accepts the mob (slot +0x70), computes the
/// detection level against the player and reports it to the player.
fn detect_against_player(e: &mut Engine, actor: Ptr, player: Ptr, distance: f32, delay: f32) {
    e.vcall(actor.addr(), 0x2b0, &args![delay]);
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if !e.vcall(process, 0x70, &args![actor]).bool() {
        return;
    }
    // Locals the game keeps on its stack: +0 = detected by sound/proximity
    // flag byte, +1 = second flag byte, +4 = the should-attack result.
    e.with_stack(8, |e, locals| {
        let flag_a = locals.addr();
        let flag_b = locals.addr() + 1;
        let attack_state = locals.addr() + 4;
        e.mem.set_u8(flag_a, 0);
        let sneaking = e.call(0x0049_3bb0, &args![actor]).u8();
        e.mem.set_u8(flag_b, 0);
        let level = e
            .call(
                0x008a_0d10,
                &args![player, 1u32, actor, flag_a, sneaking, 0u32, 0u32, flag_b],
            )
            .i32();
        let mut hostile = false;
        e.mem.set_u32(attack_state, 0);
        let attack_args = args![actor, player, 0u32, attack_state, 0u32];
        if e.call(0x008b_06d0, &attack_args).bool()
            || e.call(0x008b_c700, &args![actor, player]).bool()
        {
            hostile = true;
        } else if e.mem.u32(attack_state) <= 1 {
            let extra = e.call(0x0041_81e0, &args![actor]).u32() + 0x90;
            if e.call(0x0070_5cf0, &args![extra]).bool() {
                let extra = e.call(0x0041_81e0, &args![actor]).u32() + 0x90;
                if e.call(0x0082_5c00, &args![extra]).u32() != 0 {
                    hostile = true;
                }
            }
        }
        let near_setting = e.call(0x0040_3e20, &args![SETTING_011CCF94]).u32();
        if e.mem.f32(near_setting) > distance {
            e.call(0x0096_e3c0, &args![PROCESS_LISTS, actor]);
        }
        let level_setting = e.call(0x0040_3e20, &args![SETTING_011D13C8]).u32();
        if e.mem.f32(level_setting) as f64 <= level as f64 {
            e.call(0x0096_6f20, &args![player, actor, hostile, 0u32]);
        } else if e.mem.u8(flag_b) != 0 || e.mem.u8(flag_a) != 0 {
            e.call(0x0096_6f20, &args![player, actor, hostile, 1u32]);
        } else {
            e.call(0x0096_7350, &args![player, actor]);
        }
    });
}

// Translated from 008c8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData` constructor: the base constructor and the Animation
/// vtable. `shared` is the manager's object (+0x174). Returns `this`.
pub fn fn_008c8830(
    e: &mut Engine,
    this: Ptr<AnimationTaskData>,
    shared: u32,
) -> Ptr<AnimationTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ANIMATION_TASK_DATA);
    this
}

// Translated from 008c8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8890`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn animation_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AnimationTaskData>,
    flags: u32,
) -> Ptr<AnimationTaskData> {
    fn_008c8890(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData` destructor body: resets the vtable to the Animation
/// one, then runs `83a0`.
pub fn fn_008c8890(e: &mut Engine, this: Ptr<AnimationTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ANIMATION_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleCountMessage` (Xbox PDB): enters the critical
/// sections `011dfc38` and `011dfc04` (the count is not used).
pub fn animation_task_data_handle_count_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    _unused_1: u32,
) {
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
}

// Translated from 008c88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// sections `011dfc04` and `011dfc38` (the end argument is not used).
pub fn animation_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleMobMessage` (Xbox PDB): for an actor that is
/// flagged `bProcessMe`, calls slot +0x268 with 0.0; for a non-null mob
/// that is not an actor it calls slot +0x268(0.0) when `00574900` answers
/// true. A null mob does nothing.
pub fn animation_task_data_handle_mob_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    mob: Ptr,
) {
    if !mob.is_null() && is_actor(e, mob) {
        if e.mem.u8(mob.addr() + ACTOR_PROCESS_ME) != 0 {
            e.vcall(mob.addr(), 0x268, &args![0.0f32]);
        }
    } else if !mob.is_null() && e.call(0x0057_4900, &args![mob]).bool() {
        e.vcall(mob.addr(), 0x268, &args![0.0f32]);
    }
}

// Translated from 008c89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData` constructor: the base constructor, the ActorUpdate
/// vtable and `iUpdated` cleared. `shared` is the manager's object (+0x174).
/// Returns `this`.
pub fn fn_008c89a0(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    shared: u32,
) -> Ptr<ActorUpdateTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ACTOR_UPDATE_TASK_DATA);
    e.set(this, ActorUpdateTaskData::iUpdated, 0);
    this
}

// Translated from 008c89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8a00`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn actor_update_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    flags: u32,
) -> Ptr<ActorUpdateTaskData> {
    fn_008c8a00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData` destructor body: resets the vtable to the
/// ActorUpdate one, then runs `83a0`.
pub fn fn_008c8a00(e: &mut Engine, this: Ptr<ActorUpdateTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ACTOR_UPDATE_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleCountMessage` (Xbox PDB): clears `iUpdated`
/// and enters the critical section `011dfc38` (the count is not used).
pub fn actor_update_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    _unused_1: u32,
) {
    e.set(this, ActorUpdateTaskData::iUpdated, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
}

// Translated from 008c8a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// section `011dfc38` (the end argument is not used).
pub fn actor_update_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<ActorUpdateTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleMobMessage` (Xbox PDB): for an actor, calls
/// slot +0x2F8(0.0) when it is flagged `bProcessMe`, and counts it in
/// `iUpdated` either way. A null or non-actor mob does nothing.
pub fn actor_update_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    mob: Ptr,
) {
    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    if e.mem.u8(mob.addr() + ACTOR_PROCESS_ME) != 0 {
        e.vcall(mob.addr(), 0x2f8, &args![0.0f32]);
    }
    let updated = e.get(this, ActorUpdateTaskData::iUpdated);
    e.set(this, ActorUpdateTaskData::iUpdated, updated.wrapping_add(1));
}

// Translated from 008c8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData` constructor: the base constructor, the
/// PackageUpdate vtable and its two counters cleared. `shared` is the
/// manager's object (+0x174). Returns `this`.
pub fn fn_008c8ae0(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    shared: u32,
) -> Ptr<PackageUpdateTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    e.set(this, PackageUpdateTaskData::iPackageUpdated, 0);
    e.set(this, PackageUpdateTaskData::iActorsProcessed, 0);
    this
}

// Translated from 008c8b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8b50`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn package_update_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    flags: u32,
) -> Ptr<PackageUpdateTaskData> {
    fn_008c8b50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData` destructor body: resets the vtable to the
/// PackageUpdate one, then runs `83a0`.
pub fn fn_008c8b50(e: &mut Engine, this: Ptr<PackageUpdateTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleCountMessage` (Xbox PDB): clears the two
/// counters, clears the byte at `011e01dd` (`fn_008c8bd0`), clears the byte
/// at +0x5F8 of the player (`fn_008c8bb0`) and enters the critical section
/// `011dfbe8` (the count is not used).
pub fn package_update_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    _unused_1: u32,
) {
    e.set(this, PackageUpdateTaskData::iPackageUpdated, 0);
    e.set(this, PackageUpdateTaskData::iActorsProcessed, 0);
    fn_008c8bd0(e);
    let player = player_character(e);
    fn_008c8bb0(e, player, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c8bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x5F8 of the object (the player character
/// at its callers).
pub fn fn_008c8bb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x5f8, value);
}

// Translated from 008c8bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the global byte at `011e01dd`.
pub fn fn_008c8bd0(e: &mut Engine) {
    e.set_global(RESET_FLAG_011E01DD, 0u8);
}

// Translated from 008c8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleMobMessage` (Xbox PDB): runs the package
/// update for one mob.
///
/// Skipped unless the mob passes `00576d30`, `00440d80` and `00440da0` with
/// false, has a process (`008d8520`) and its current process type
/// (`00931850`) is 0. When the mob is an actor that is not flagged
/// `bProcessMe` nothing more happens (and nothing is counted). If it is an
/// actor for which `008b0520` answers true, its process (slot +0x24) is
/// handed the actor, `iActorsProcessed` is incremented and the process's
/// byte at +0x2C7 is set to 1; otherwise the mob's slot +0x25C is called
/// with 0.0 and `iPackageUpdated` is incremented. A sleeping or resting
/// player restores the actor's health and fatigue by the `010162c0` amount.
/// A mob whose process type is not 0 afterwards clears `pmoblist`. Finally,
/// when the actor was not handed to its process, the entries of a list
/// (at +0xC of the object `00422700` returns for the mob's `005d43c0`
/// object) that are neither the player nor accepted by `008bc860` for the
/// actor are passed to `00422690` (named `ExtraDataList::RemoveFollower` in
/// the Xbox PDB), through two temporary lists; and when `008256d0` answers
/// true for that list, `00422720` (`ExtraDataList::RemoveFollowerExtra`) runs.
///
/// The compiler's exception frame is not translated.
pub fn package_update_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    mob: Ptr,
) {
    if mob.is_null()
        || e.call(0x0057_6d30, &args![mob]).bool()
        || e.call(0x0044_0d80, &args![mob]).bool()
        || e.call(0x0044_0da0, &args![mob]).bool()
        || e.call(GET_PROCESS, &args![mob]).u32() == 0
        || e.call(GET_CURRENT_PROCESS_TYPE, &args![mob]).u32() != 0
    {
        return;
    }
    let mut actor = Ptr::NULL;
    let mut not_handed_over = true;
    if is_actor(e, mob) {
        actor = mob;
    }
    if !actor.is_null() && e.mem.u8(actor.addr() + ACTOR_PROCESS_ME) == 0 {
        return;
    }
    if !actor.is_null() && e.call(0x008b_0520, &args![actor]).bool() {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x24, &args![actor, 0u32]);
        not_handed_over = false;
        let n = e.get(this, PackageUpdateTaskData::iActorsProcessed);
        e.set(
            this,
            PackageUpdateTaskData::iActorsProcessed,
            n.wrapping_add(1),
        );
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        fn_008c8fd0(e, Ptr::new(process), 1);
    } else {
        e.vcall(mob.addr(), 0x25c, &args![0.0f32]);
        let n = e.get(this, PackageUpdateTaskData::iPackageUpdated);
        e.set(
            this,
            PackageUpdateTaskData::iPackageUpdated,
            n.wrapping_add(1),
        );
    }

    let player = player_character(e);
    if !actor.is_null() && e.call(0x0094_df60, &args![player]).bool() {
        let amount: f32 = e.global(RESTORE_AMOUNT);
        e.call(0x0088_b510, &args![actor, amount]);
        e.call(0x0088_b5a0, &args![actor, amount]);
    }

    if e.call(GET_CURRENT_PROCESS_TYPE, &args![mob]).u32() != 0 {
        e.set(this, PackageUpdateTaskData::pmoblist, Ptr::NULL);
    }

    if not_handed_over {
        reconcile_followers(e, mob, actor, player);
    }
}

/// The follower clean-up at the end of
/// `PackageUpdateTaskData::HandleMobMessage`: `005d43c0` on the mob gives an
/// object, `00422700` on that an owner whose list (at +0xC) is copied into
/// one temporary list; the entries that are neither null, the player nor
/// accepted by `008bc860` for `actor` are collected in a second, and each of
/// those is passed to `00422690` on the mob's `005d43c0` object. Both
/// temporaries are then cleared and freed.
fn reconcile_followers(e: &mut Engine, mob: Ptr, actor: Ptr, player: Ptr) {
    let extra = e.call(0x005d_43c0, &args![mob]).u32();
    let owner = if extra != 0 {
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2700, &args![extra]).u32()
    } else {
        0
    };
    if owner == 0 {
        return;
    }
    let first_list = new_list(e);
    let second_list = new_list(e);

    // Copy the owner's list into `second_list`.
    let mut node = e.mem.u32(owner + 0xc);
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        e.call(0x0090_5820, &args![second_list, item_slot]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    // Collect into `first_list` the entries that are not the player and
    // cannot follow `actor`.
    let mut node = second_list;
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        if item != 0 && item != player.addr() && !e.call(0x008b_c860, &args![item, actor]).bool() {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                e.call(0x005a_e3d0, &args![first_list, slot]);
            });
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    // Remove each collected entry from the owner.
    let mut node = first_list;
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2690, &args![extra, item]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    for list in [first_list, second_list] {
        e.call(0x0047_0470, &args![list]);
        if list != 0 {
            e.call(0x0047_02f0, &args![list, 1u32]);
        }
    }
    let owner_list = e.mem.u32(owner + 0xc);
    if e.call(0x0082_56d0, &args![owner_list]).bool() {
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2720, &args![extra]);
    }
}

/// `new BSSimpleList` as the handler writes it: 8 bytes from `operator new`
/// constructed with `0096a2d0` (null stays null).
fn new_list(e: &mut Engine) -> u32 {
    let list = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if list == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![list]).u32()
    }
}

// Translated from 008c8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x2C7 of the object (a process, at the
/// callers).
pub fn fn_008c8fd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2c7, value);
}

// Translated from 008c8ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleEndMessage` (Xbox PDB): passes the float at
/// +0xC of `011f6394` (`0084d030`) to the process lists (`00977130`), then
/// leaves the critical section `011dfbe8` (the end argument is not used).
pub fn package_update_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<PackageUpdateTaskData>,
    _unused_1: u32,
) {
    let scale = e
        .call(0x0084_d030, &args![TASK_SCALE_OBJECT_011F6394])
        .f32();
    e.call(0x0097_7130, &args![PROCESS_LISTS, scale]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
}

// Translated from 008c9020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData` constructor: the base constructor, the
/// ActorsScript vtable and its counter cleared. `shared` is the manager's
/// object (+0x174). Returns `this`.
pub fn fn_008c9020(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    shared: u32,
) -> Ptr<ActorsScriptTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    e.set(this, ActorsScriptTaskData::iScriptsRun, 0);
    this
}

// Translated from 008c9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c9080`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn actors_script_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    flags: u32,
) -> Ptr<ActorsScriptTaskData> {
    fn_008c9080(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData` destructor body: resets the vtable to the
/// ActorsScript one, then runs `83a0`.
pub fn fn_008c9080(e: &mut Engine, this: Ptr<ActorsScriptTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleCountMessage` (Xbox PDB): clears the script
/// counter and enters the critical sections `011dfc38`, `011dfc04` and
/// `011dfbe8` (the count is not used).
pub fn actors_script_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    _unused_1: u32,
) {
    e.set(this, ActorsScriptTaskData::iScriptsRun, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// sections `011dfbe8`, `011dfc04` and `011dfc38` (the end argument is not
/// used).
pub fn actors_script_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<ActorsScriptTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c9110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleMobMessage` (Xbox PDB): for an actor, when
/// scripts are being processed (`Script::GetProcessScripts`, `005ac740`),
/// runs the reference's script (`TESObjectREFR::RunScript`, `00565870`) and
/// counts it in `iScriptsRun`.
pub fn actors_script_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    mob: Ptr,
) {
    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    if e.call(0x005a_c740, &[]).bool() {
        e.call(0x0056_5870, &args![mob]);
        let n = e.get(this, ActorsScriptTaskData::iScriptsRun);
        e.set(this, ActorsScriptTaskData::iScriptsRun, n.wrapping_add(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const HANDLE_COUNT: u32 = 0x7000_0010;
    const HANDLE_MOB: u32 = 0x7000_0014;
    const HANDLE_END: u32 = 0x7000_0018;
    const QUEUE_RUN: u32 = 0x7000_0020;
    const QUEUE_DRAIN: u32 = 0x7000_0030;

    fn ok(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Engine with the pages of the globals the handlers read and write.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_e000u32,
            0x011d_8000,
            0x011e_0000,
            0x0101_2000,
            0x0101_7000,
            0x0101_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e
    }

    /// An object whose first word points at a fresh vtable holding the
    /// given (byte offset, target) slots.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> Ptr {
        let table = e.mem.alloc(0x300);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        Ptr::new(object)
    }

    /// A tasklet (any subclass, `size` bytes) with the handler slots and a
    /// queue object at +0x1C whose vtable has the two drain slots.
    fn tasklet(e: &mut Engine, size: u32, handlers: &[(u32, u32)]) -> Ptr {
        let this = object_with_slots(e, size, handlers);
        let queue_table = e.mem.alloc(0x40);
        e.mem.set_u32(queue_table + 0xc, QUEUE_RUN);
        e.mem.set_u32(queue_table + 0x10, QUEUE_DRAIN);
        e.mem.set_u32(this.addr() + 0x1c, queue_table);
        this
    }

    fn standard_handlers() -> [(u32, u32); 3] {
        [(0x10, HANDLE_COUNT), (0x14, HANDLE_MOB), (0x18, HANDLE_END)]
    }

    fn log_calls(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn address_log(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// A queue double that hands out the given (type, count, mob) messages
    /// one by one and then reports an empty queue.
    fn queue_script(e: &mut Engine, slot: u32, messages: Vec<(u32, u32, u32)>) {
        let mut messages = messages.into_iter();
        e.register_double(slot, move |e, a| match messages.next() {
            Some((kind, count, mob)) => {
                e.mem.set_u32(a[1], kind);
                e.mem.set_u32(a[1] + 4, count);
                e.mem.set_u32(a[1] + 8, mob);
                ok(1)
            }
            None => ok(0),
        });
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn process_dispatches_messages_until_the_end_message() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this = tasklet(&mut e, 0x38, &standard_handlers());
        e.mem.set_u32(this.addr() + 0x34, 0xe7e7);
        queue_script(
            &mut e,
            QUEUE_DRAIN,
            vec![(0, 5, 0), (1, 0, 0x1234), (2, 0, 0)],
        );
        log_calls(&mut e);
        e.call(0x008c_8130, &args![this]);
        assert_eq!(calls_to(&e, HANDLE_COUNT), vec![vec![this.addr(), 5]]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x1234]]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 0xe7e7]]);
        assert!(calls_to(&e, TASKLET_SET_YIELDING).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x30), 5);
    }

    #[test]
    fn process_yields_when_the_queue_runs_dry() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_DRAIN, vec![(1, 0, 0x55)]);
        log_calls(&mut e);
        e.call(0x008c_8130, &args![this]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x55]]);
        assert!(calls_to(&e, HANDLE_END).is_empty());
        assert_eq!(calls_to(&e, TASKLET_SET_YIELDING), vec![vec![this.addr()]]);
    }

    #[test]
    fn message_initializer_makes_an_end_message() {
        let mut e = engine();
        let msg: Ptr<MobileObjectMessage> = e.new_object();
        e.mem.set_u32(msg.addr(), 9);
        e.mem.set_u32(msg.addr() + 4, 9);
        e.mem.set_u32(msg.addr() + 8, 9);
        assert_eq!(e.call(0x008c_8190, &args![msg]).ptr::<()>(), msg.cast());
        assert_eq!(e.mem.u32(msg.addr()), 2);
        assert_eq!(e.mem.u32(msg.addr() + 4), 0);
        assert_eq!(e.mem.u32(msg.addr() + 8), 0);
    }

    #[test]
    fn dispatch_handles_each_message_type() {
        let mut e = engine();
        quiet(&mut e, &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END]);
        let this = tasklet(&mut e, 0x38, &standard_handlers());
        let msg: Ptr<MobileObjectMessage> = e.new_object();
        log_calls(&mut e);
        for (kind, expected_continue) in [(0u32, true), (1, true), (2, false), (3, true)] {
            e.mem.set_u32(msg.addr(), kind);
            e.mem.set_u32(msg.addr() + 4, 7);
            e.mem.set_u32(msg.addr() + 8, 0x99);
            let r = e.call(0x008c_81c0, &args![this, msg, 0xabu32]).bool();
            assert_eq!(r, expected_continue, "type {kind}");
        }
        assert_eq!(calls_to(&e, HANDLE_COUNT), vec![vec![this.addr(), 7]]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x99]]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 0xab]]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 7);
    }

    #[test]
    fn run_to_completion_stores_the_type_and_drains_the_queue() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_RUN, vec![(1, 0, 0x21), (1, 0, 0x22)]);
        log_calls(&mut e);
        e.call(0x008c_8240, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 3);
        assert_eq!(
            calls_to(&e, HANDLE_MOB),
            vec![vec![this.addr(), 0x21], vec![this.addr(), 0x22]]
        );
        // The queue ran dry: no end message was dispatched, nothing yields.
        assert!(calls_to(&e, HANDLE_END).is_empty());
        assert!(calls_to(&e, TASKLET_SET_YIELDING).is_empty());
    }

    #[test]
    fn run_to_completion_stops_at_the_end_message() {
        let mut e = engine();
        quiet(&mut e, &[HANDLE_MOB, HANDLE_END]);
        let this = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_RUN, vec![(2, 0, 0), (1, 0, 0x21)]);
        log_calls(&mut e);
        e.call(0x008c_8240, &args![this, 4u32]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 4]]);
        assert!(calls_to(&e, HANDLE_MOB).is_empty());
    }

    #[test]
    fn base_constructor_builds_the_queue_and_clears_the_fields() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<MobileObjectTaskletData> = e.new_object();
        for off in [0x18, 0x30, 0x34] {
            e.mem.set_u32(this.addr() + off, 0xdead);
        }
        log_calls(&mut e);
        let r = e.call(0x008c_82e0, &args![this, 0x5150u32]).ptr::<()>();
        assert_eq!(r, this.cast());
        assert_eq!(
            address_log(&e),
            vec![
                0x008c_82e0,
                TASKLET_BASE_CONSTRUCTOR,
                MESSAGE_QUEUE_CONSTRUCTOR
            ]
        );
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_CONSTRUCTOR),
            vec![vec![this.addr() + 0x1c, 0x5150]]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_MOBILE_OBJECT_TASKLET_DATA);
        for off in [0x18, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this.addr() + off), 0);
        }
    }

    #[test]
    fn destructor_body_destroys_the_queue_then_the_base() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr<MobileObjectTaskletData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_83a0, &args![this]);
        assert_eq!(
            address_log(&e),
            vec![
                0x008c_83a0,
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR
            ]
        );
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR),
            vec![vec![this.addr() + 0x1c]]
        );
        assert_eq!(
            calls_to(&e, TASKLET_BASE_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    /// Checks a scalar deleting destructor: it runs the destructor body,
    /// deletes only when bit 0 of the flags is set, and returns `this`.
    fn check_deleting_destructor(entry: u32, vtable: u32, size: u32) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(size));
        for (flags, deleted) in [(0u32, false), (1, true)] {
            log_calls(&mut e);
            let r = e.call(entry, &args![this, flags]).ptr::<()>();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(deleted));
        }
    }

    #[test]
    fn base_scalar_deleting_destructor_deletes_on_bit_zero() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(0x38));
        for (flags, deleted) in [(0u32, false), (1, true)] {
            log_calls(&mut e);
            assert_eq!(e.call(0x008c_8370, &args![this, flags]).ptr::<()>(), this);
            assert_eq!(calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(deleted));
        }
    }

    #[test]
    fn detection_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8400, VTABLE_DETECTION_TASK_DATA, 0x40);
    }

    #[test]
    fn detection_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call(0x008c_8430, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_DETECTION_TASK_DATA);
    }

    #[test]
    fn detection_constructor_sets_vtable_and_clears_the_fields() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_f32(this.addr() + 0x38, 9.0);
        e.mem.set_u32(this.addr() + 0x3c, 9);
        let r = e.call(0x008c_82a0, &args![this, 1u32]).ptr::<()>();
        assert_eq!(r, this.cast());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_DETECTION_TASK_DATA);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 0.0);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    #[test]
    fn detection_count_message_clears_and_enters_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_f32(this.addr() + 0x38, 9.0);
        e.mem.set_u32(this.addr() + 0x3c, 9);
        log_calls(&mut e);
        e.call(0x008c_8450, &args![this, 77u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 0.0);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC04, 0],
                vec![CRITICAL_SECTION_011DFBE8, 0]
            ]
        );
    }

    #[test]
    fn detection_end_message_updates_the_auto_aim_actor_and_leaves_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[0x0096_4260, CRITICAL_SECTION_LEAVE]);
        e.set_global(PLAYER_CHARACTER, 0x4242_0000u32);
        let this: Ptr<DetectionTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8490, &args![this, 0u32]);
        assert_eq!(
            e.call_log.clone().unwrap(),
            vec![
                (0x008c_8490, vec![this.addr(), 0]),
                (0x0096_4260, vec![0x4242_0000]),
                (CRITICAL_SECTION_LEAVE, vec![CRITICAL_SECTION_011DFC04]),
                (CRITICAL_SECTION_LEAVE, vec![CRITICAL_SECTION_011DFBE8]),
            ]
        );
    }

    // ---- DetectionTaskData::HandleMobMessage -------------------------

    const FAKE_IS_ACTOR: u32 = 0x7000_0100;
    const FAKE_HIDDEN: u32 = 0x7000_0104;
    const FAKE_SLOT_2B4: u32 = 0x7000_0108;
    const FAKE_SLOT_22C: u32 = 0x7000_010c;
    const FAKE_SLOT_2B0: u32 = 0x7000_0110;
    const FAKE_SLOT_2B8: u32 = 0x7000_0114;
    const FAKE_PROCESS_ACCEPTS: u32 = 0x7000_0118;
    const SETTING_CELL_NEAR: u32 = 0x7100_0000;
    const SETTING_CELL_LEVEL: u32 = 0x7100_0010;

    /// What the detection scenario varies.
    struct Scene {
        is_actor: bool,
        process_me: bool,
        hidden: bool,
        blocked: bool,
        is_player: bool,
        slot_2b4: f32,
        alpha: f64,
        process_accepts: bool,
        attacked: bool,
        distance: f32,
        near_setting: f32,
        level_setting: f32,
        level: u32,
        flags: (u8, u8),
    }

    impl Scene {
        fn new() -> Scene {
            Scene {
                is_actor: true,
                process_me: true,
                hidden: false,
                blocked: false,
                is_player: false,
                slot_2b4: -1.0,
                alpha: 0.0,
                process_accepts: true,
                attacked: false,
                distance: 100.0,
                near_setting: 200.0,
                level_setting: 3.0,
                level: 5,
                flags: (0, 0),
            }
        }
    }

    /// Runs the handler on the scene; returns the engine (with its call
    /// log), the tasklet and the mob.
    fn run_detection(scene: &Scene) -> (Engine, Ptr<DetectionTaskData>, Ptr) {
        let mut e = engine();
        e.map(0x7100_0000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(DELAY_CAP, 4.0f32);
        e.mem.set_f32(SETTING_CELL_NEAR, scene.near_setting);
        e.mem.set_f32(SETTING_CELL_LEVEL, scene.level_setting);

        let process = object_with_slots(&mut e, 0x10, &[(0x70, FAKE_PROCESS_ACCEPTS)]);
        let extra_data = object_with_slots(&mut e, 0x100, &[]);
        let extra_table = e.mem.alloc(0x40);
        e.mem.set_u32(extra_table + 0x20, FAKE_HIDDEN);
        e.mem.set_u32(extra_data.addr() + 0x30, extra_table);

        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[
                (0x100, FAKE_IS_ACTOR),
                (0x22c, FAKE_SLOT_22C),
                (0x2b0, FAKE_SLOT_2B0),
                (0x2b4, FAKE_SLOT_2B4),
                (0x2b8, FAKE_SLOT_2B8),
            ],
        );
        e.mem
            .set_u8(actor.addr() + 0xbc, u8::from(scene.process_me));
        let player = if scene.is_player {
            actor
        } else {
            Ptr::new(e.mem.alloc(0x40))
        };
        e.set_global(PLAYER_CHARACTER, player.addr());

        // Doubles: virtual slots first, then the helper functions.
        let is_actor = scene.is_actor;
        e.register_double(FAKE_IS_ACTOR, move |_, _| ok(u32::from(is_actor)));
        let hidden = scene.hidden;
        e.register_double(FAKE_HIDDEN, move |_, _| ok(u32::from(hidden)));
        let slot_2b4 = scene.slot_2b4;
        e.register_double(FAKE_SLOT_2B4, move |_, _| slot_2b4.into_ret());
        quiet(&mut e, &[FAKE_SLOT_22C, FAKE_SLOT_2B0, FAKE_SLOT_2B8]);
        let accepts = scene.process_accepts;
        e.register_double(FAKE_PROCESS_ACCEPTS, move |_, _| ok(u32::from(accepts)));

        e.register(0x0084_d030, |_, _| 2.0f32.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0047_6b70, |_, a| (f32::from_bits(a[1]) / 2.0).into_ret());
        let extra_addr = extra_data.addr();
        e.register_double(0x0041_81e0, move |_, _| ok(extra_addr));
        let distance = scene.distance;
        e.register_double(0x0057_23b0, move |_, _| distance.into_ret());
        e.register(0x0047_c850, |_, _| ok(0));
        let blocked = scene.blocked;
        e.register_double(0x0043_7bd0, move |_, _| ok(u32::from(blocked)));
        let alpha = scene.alpha;
        e.register_double(0x0070_14e0, move |_, _| alpha.into_ret());
        let process_addr = process.addr();
        e.register_double(GET_PROCESS, move |_, _| ok(process_addr));
        e.register(0x0049_3bb0, |_, _| ok(0));
        let (flag_a, flag_b, level) = (scene.flags.0, scene.flags.1, scene.level);
        e.register_double(0x008a_0d10, move |e, a| {
            e.mem.set_u8(a[3], flag_a);
            e.mem.set_u8(a[7], flag_b);
            ok(level)
        });
        let attacked = scene.attacked;
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ok(u32::from(attacked))
        });
        e.register(0x008b_c700, |_, _| ok(0));
        e.register(0x0070_5cf0, |_, _| ok(0));
        e.register(0x0082_5c00, |_, _| ok(0));
        e.register(0x0040_3e20, |_, a| {
            ok(if a[0] == SETTING_011CCF94 {
                SETTING_CELL_NEAR
            } else {
                SETTING_CELL_LEVEL
            })
        });
        quiet(&mut e, &[0x0096_e3c0, 0x0096_6f20, 0x0096_7350]);

        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x30, 3);
        log_calls(&mut e);
        e.call(0x008c_84c0, &args![this, actor]);
        (e, this, actor)
    }

    fn iupdated(e: &Engine, this: Ptr<DetectionTaskData>) -> u32 {
        e.mem.u32(this.addr() + 0x3c)
    }

    #[test]
    fn detection_mob_message_reports_a_detected_actor() {
        let scene = Scene::new();
        let (e, this, actor) = run_detection(&scene);
        // Delay: 2.0 * 3 = 6.0, capped at 4.0, random in [0, 4.0] = 2.0.
        assert_eq!(
            calls_to(&e, 0x0040_ebd0),
            vec![vec![6.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0047_6b70),
            vec![vec![0.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, FAKE_SLOT_2B0),
            vec![vec![actor.addr(), 2.0f32.to_bits()]]
        );
        let player = e.global::<u32>(PLAYER_CHARACTER);
        // Level 5 against a setting of 3.0 reports without the second flag.
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 0, 0]]
        );
        // The actor (100.0) is nearer than the setting (200.0).
        assert_eq!(
            calls_to(&e, 0x0096_e3c0),
            vec![vec![PROCESS_LISTS, actor.addr()]]
        );
        assert!(calls_to(&e, 0x0096_7350).is_empty());
        assert_eq!(iupdated(&e, this), 1);
    }

    #[test]
    fn detection_mob_message_marks_attackers_hostile() {
        let mut scene = Scene::new();
        scene.attacked = true;
        scene.near_setting = 50.0;
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 1, 0]]
        );
        // 100.0 is not nearer than 50.0.
        assert!(calls_to(&e, 0x0096_e3c0).is_empty());
    }

    #[test]
    fn detection_mob_message_below_the_level_setting_keeps_or_drops_the_perception() {
        let mut scene = Scene::new();
        scene.level_setting = 10.0;
        scene.flags = (0, 1);
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 0, 1]]
        );

        scene.flags = (0, 0);
        let (e, _, actor) = run_detection(&scene);
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert_eq!(calls_to(&e, 0x0096_7350), vec![vec![player, actor.addr()]]);
    }

    #[test]
    fn detection_mob_message_idle_actor_is_removed_when_negative() {
        let mut scene = Scene::new();
        scene.slot_2b4 = 1.0;
        let (e, this, _) = run_detection(&scene);
        // A non-negative slot value takes the idle path: slot +0x2B8 only.
        assert_eq!(calls_to(&e, FAKE_SLOT_2B8).len(), 1);
        assert!(calls_to(&e, 0x0096_7350).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert_eq!(iupdated(&e, this), 1);

        // `00437bd0` also sends a negative-valued actor down the idle path,
        // where it is removed from the player's perceived actors first.
        let mut scene = Scene::new();
        scene.blocked = true;
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(calls_to(&e, 0x0096_7350), vec![vec![player, actor.addr()]]);
        assert_eq!(calls_to(&e, FAKE_SLOT_2B8).len(), 1);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
    }

    #[test]
    fn detection_mob_message_does_nothing_while_the_fader_is_visible() {
        let mut scene = Scene::new();
        scene.alpha = 1.0;
        let (e, this, _) = run_detection(&scene);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B8).is_empty());
        assert_eq!(iupdated(&e, this), 1);
    }

    #[test]
    fn detection_mob_message_stops_when_the_process_refuses() {
        let mut scene = Scene::new();
        scene.process_accepts = false;
        let (e, this, actor) = run_detection(&scene);
        assert_eq!(calls_to(&e, FAKE_SLOT_2B0).len(), 1);
        assert!(calls_to(&e, 0x008a_0d10).is_empty());
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert_eq!(iupdated(&e, this), 1);
        let _ = actor;
    }

    #[test]
    fn detection_mob_message_skips_ineligible_mobs_but_counts_actors() {
        let mut scene = Scene::new();
        scene.is_actor = false;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 0);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());

        let mut scene = Scene::new();
        scene.process_me = false;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, 0x0057_23b0).is_empty());

        let mut scene = Scene::new();
        scene.hidden = true;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, 0x0057_23b0).is_empty());

        let mut scene = Scene::new();
        scene.is_player = true;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B8).is_empty());
    }

    #[test]
    fn detection_mob_message_with_no_mob_only_draws_the_delay() {
        let mut e = engine();
        e.set_global(DELAY_CAP, 4.0f32);
        e.register(0x0084_d030, |_, _| 2.0f32.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0047_6b70, |_, a| (f32::from_bits(a[1]) / 2.0).into_ret());
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.call(0x008c_84c0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    // ---- AnimationTaskData -------------------------------------------

    #[test]
    fn animation_constructor_sets_the_vtable() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<AnimationTaskData> = e.new_object();
        assert_eq!(
            e.call(0x008c_8830, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIMATION_TASK_DATA);
    }

    #[test]
    fn animation_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8860, VTABLE_ANIMATION_TASK_DATA, 0x38);
    }

    #[test]
    fn animation_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call(0x008c_8890, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIMATION_TASK_DATA);
    }

    #[test]
    fn animation_count_message_enters_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<AnimationTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_88b0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC38, 0],
                vec![CRITICAL_SECTION_011DFC04, 0]
            ]
        );
    }

    #[test]
    fn animation_end_message_leaves_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<AnimationTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_88e0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![
                vec![CRITICAL_SECTION_011DFC04],
                vec![CRITICAL_SECTION_011DFC38]
            ]
        );
    }

    #[test]
    fn animation_mob_message_updates_flagged_actors_and_asks_about_others() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        let this: Ptr<AnimationTaskData> = e.new_object();
        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x268, 0x7000_0200)],
        );
        log_calls(&mut e);
        // Flagged actor: slot +0x268(0.0).
        e.mem.set_u8(actor.addr() + 0xbc, 1);
        e.call(0x008c_8910, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![actor.addr(), 0]]);
        // Unflagged actor: nothing.
        e.mem.set_u8(actor.addr() + 0xbc, 0);
        e.call(0x008c_8910, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
        // Null mob: nothing.
        e.call(0x008c_8910, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
    }

    #[test]
    fn animation_mob_message_for_a_non_actor_follows_the_other_test() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(0));
        let this: Ptr<AnimationTaskData> = e.new_object();
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x268, 0x7000_0200)],
        );
        // `00574900` answers false: nothing happens.
        e.register_double(0x0057_4900, |_, _| ok(0));
        log_calls(&mut e);
        e.call(0x008c_8910, &args![this, mob]);
        assert!(calls_to(&e, 0x7000_0200).is_empty());
        // `00574900` answers true for this mob: slot +0x268(0.0).
        let addr = mob.addr();
        e.register_double(0x0057_4900, move |_, a| ok(u32::from(a[0] == addr)));
        e.call(0x008c_8910, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![addr, 0]]);
    }

    // ---- ActorUpdateTaskData -----------------------------------------

    #[test]
    fn actor_update_constructor_sets_the_vtable_and_clears_the_counter() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x38, 6);
        assert_eq!(
            e.call(0x008c_89a0, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTOR_UPDATE_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
    }

    #[test]
    fn actor_update_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_89d0, VTABLE_ACTOR_UPDATE_TASK_DATA, 0x3c);
    }

    #[test]
    fn actor_update_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x3c));
        e.call(0x008c_8a00, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTOR_UPDATE_TASK_DATA);
    }

    #[test]
    fn actor_update_count_message_clears_the_counter_and_enters_the_section() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x38, 6);
        log_calls(&mut e);
        e.call(0x008c_8a20, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![vec![CRITICAL_SECTION_011DFC38, 0]]
        );
    }

    #[test]
    fn actor_update_end_message_leaves_the_section() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8a50, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![vec![CRITICAL_SECTION_011DFC38]]
        );
    }

    #[test]
    fn actor_update_mob_message_updates_flagged_actors_and_counts_all_actors() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x2f8, 0x7000_0200)],
        );
        log_calls(&mut e);
        e.mem.set_u8(actor.addr() + 0xbc, 1);
        e.call(0x008c_8a70, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![actor.addr(), 0]]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 1);
        // Unflagged: counted, not updated.
        e.mem.set_u8(actor.addr() + 0xbc, 0);
        e.call(0x008c_8a70, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 2);
        // Null mob: neither.
        e.call(0x008c_8a70, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 2);
    }

    #[test]
    fn actor_update_mob_message_ignores_non_actors() {
        let mut e = engine();
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(0));
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        let mob = object_with_slots(&mut e, 0x1c4, &[(0x100, FAKE_IS_ACTOR)]);
        e.call(0x008c_8a70, &args![this, mob]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
    }

    // ---- PackageUpdateTaskData ---------------------------------------

    #[test]
    fn package_update_constructor_sets_the_vtable_and_clears_the_counters() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        e.mem.set_u32(this.addr() + 0x40, 6);
        assert_eq!(
            e.call(0x008c_8ae0, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_PACKAGE_UPDATE_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
    }

    #[test]
    fn package_update_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8b20, VTABLE_PACKAGE_UPDATE_TASK_DATA, 0x44);
    }

    #[test]
    fn package_update_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x44));
        e.call(0x008c_8b50, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    }

    #[test]
    fn package_update_count_message_resets_the_counters_and_flags() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let player: Ptr = Ptr::new(e.mem.alloc(0x700));
        e.mem.set_u8(player.addr() + 0x5f8, 1);
        e.set_global(PLAYER_CHARACTER, player.addr());
        e.set_global(RESET_FLAG_011E01DD, 1u8);
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        e.mem.set_u32(this.addr() + 0x40, 7);
        log_calls(&mut e);
        e.call(0x008c_8b70, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
        assert_eq!(e.global::<u8>(RESET_FLAG_011E01DD), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x5f8), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![vec![CRITICAL_SECTION_011DFBE8, 0]]
        );
    }

    #[test]
    fn byte_setters_store_their_byte() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x700));
        e.call(0x008c_8bb0, &args![object, 0x7fu8]);
        assert_eq!(e.mem.u8(object.addr() + 0x5f8), 0x7f);
        e.call(0x008c_8fd0, &args![object, 1u8]);
        assert_eq!(e.mem.u8(object.addr() + 0x2c7), 1);
        e.set_global(RESET_FLAG_011E01DD, 1u8);
        e.call(0x008c_8bd0, &[]);
        assert_eq!(e.global::<u8>(RESET_FLAG_011E01DD), 0);
    }

    #[test]
    fn package_update_end_message_passes_the_scale_and_leaves_the_section() {
        let mut e = engine();
        e.register(0x0084_d030, |_, _| 0.25f32.into_ret());
        quiet(&mut e, &[0x0097_7130, CRITICAL_SECTION_LEAVE]);
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8ff0, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0084_d030),
            vec![vec![TASK_SCALE_OBJECT_011F6394]]
        );
        assert_eq!(
            calls_to(&e, 0x0097_7130),
            vec![vec![PROCESS_LISTS, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![vec![CRITICAL_SECTION_011DFBE8]]
        );
    }

    // ---- PackageUpdateTaskData::HandleMobMessage ---------------------

    const FAKE_SLOT_24: u32 = 0x7000_0300;
    const FAKE_SLOT_25C: u32 = 0x7000_0304;

    /// What the package scenario varies.
    struct Package {
        guard_576d30: bool,
        has_process: bool,
        process_type: u32,
        later_process_type: u32,
        is_actor: bool,
        process_me: bool,
        hands_over: bool,
        resting: bool,
    }

    impl Package {
        fn new() -> Package {
            Package {
                guard_576d30: false,
                has_process: true,
                process_type: 0,
                later_process_type: 0,
                is_actor: true,
                process_me: true,
                hands_over: false,
                resting: false,
            }
        }
    }

    /// A node-based list model for the follower clean-up: the list object
    /// is its own first node (`item` at +0, `next` at +4).
    fn list_append(e: &mut Engine, list: u32, item: u32) {
        if e.mem.u32(list) == 0 {
            e.mem.set_u32(list, item);
            return;
        }
        let mut node = list;
        while e.mem.u32(node + 4) != 0 {
            node = e.mem.u32(node + 4);
        }
        let fresh = e.mem.alloc(8);
        e.mem.set_u32(fresh, item);
        e.mem.set_u32(node + 4, fresh);
    }

    const OWNER_EXTRA: u32 = 0x7300_0077;
    /// The player's address in the package scenario.
    const PACKAGE_PLAYER: u32 = 0x7500_0000;
    const NODE_DATA: u32 = 0x0068_15c0;
    const NODE_NEXT: u32 = 0x0072_6070;

    fn run_package(
        scene: &Package,
        owner_items: &[u32],
    ) -> (Engine, Ptr<PackageUpdateTaskData>, Ptr, Ptr, u32) {
        let mut e = engine();
        e.set_global(RESTORE_AMOUNT, 2.0f32);
        e.map(PACKAGE_PLAYER, 0x1000);
        let player: Ptr = Ptr::new(PACKAGE_PLAYER);
        e.set_global(PLAYER_CHARACTER, player.addr());

        let process = object_with_slots(&mut e, 0x300, &[(0x24, FAKE_SLOT_24)]);
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x25c, FAKE_SLOT_25C)],
        );
        e.mem.set_u8(mob.addr() + 0xbc, u8::from(scene.process_me));
        quiet(&mut e, &[FAKE_SLOT_24, FAKE_SLOT_25C]);
        let is_actor = scene.is_actor;
        e.register_double(FAKE_IS_ACTOR, move |_, _| ok(u32::from(is_actor)));
        let guard = scene.guard_576d30;
        e.register_double(0x0057_6d30, move |_, _| ok(u32::from(guard)));
        e.register(0x0044_0d80, |_, _| ok(0));
        e.register(0x0044_0da0, |_, _| ok(0));
        let (has_process, process_addr) = (scene.has_process, process.addr());
        e.register_double(GET_PROCESS, move |_, _| {
            ok(if has_process { process_addr } else { 0 })
        });
        // The first answer is the guard's, later ones are the re-check
        // after the update.
        let (process_type, later_type) = (scene.process_type, scene.later_process_type);
        let mut asked = 0;
        e.register_double(GET_CURRENT_PROCESS_TYPE, move |_, _| {
            asked += 1;
            ok(if asked == 1 { process_type } else { later_type })
        });
        let hands_over = scene.hands_over;
        e.register_double(0x008b_0520, move |_, _| ok(u32::from(hands_over)));
        let resting = scene.resting;
        e.register_double(0x0094_df60, move |_, _| ok(u32::from(resting)));
        quiet(&mut e, &[0x0088_b510, 0x0088_b5a0]);

        // Follower clean-up doubles: a real node model.
        let owner = e.mem.alloc(0x20);
        let first = e.mem.alloc(8);
        for item in owner_items {
            list_append(&mut e, first, *item);
        }
        e.mem.set_u32(owner + 0xc, first);
        e.register_double(0x005d_43c0, |_, _| ok(OWNER_EXTRA));
        e.register_double(0x0042_2700, move |_, _| ok(owner));
        e.register(NODE_DATA, |_, a| ok(a[0]));
        e.register(NODE_NEXT, |e, a| ok(e.mem.u32(a[0] + 4)));
        e.register(0x0096_a2d0, |_, a| ok(a[0]));
        e.register(0x0090_5820, |e, a| {
            let item = e.mem.u32(a[1]);
            list_append(e, a[0], item);
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            list_append(e, a[0], item);
            Ret::default()
        });
        // `could follow`: true for the item 0xb000.
        e.register(0x008b_c860, |_, a| ok(u32::from(a[0] == 0xb000)));
        quiet(
            &mut e,
            &[0x0042_2690, 0x0047_0470, 0x0047_02f0, 0x0042_2720],
        );
        e.register(0x0082_56d0, |_, a| ok(u32::from(a[0] != 0)));

        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x18, 0x5555);
        log_calls(&mut e);
        e.call(0x008c_8be0, &args![this, mob]);
        (e, this, mob, player, owner)
    }

    fn counters(e: &Engine, this: Ptr<PackageUpdateTaskData>) -> (u32, u32) {
        (e.mem.u32(this.addr() + 0x3c), e.mem.u32(this.addr() + 0x40))
    }

    #[test]
    fn package_mob_message_skips_mobs_the_guards_reject() {
        for scene in [
            Package {
                guard_576d30: true,
                ..Package::new()
            },
            Package {
                has_process: false,
                ..Package::new()
            },
            Package {
                process_type: 2,
                ..Package::new()
            },
        ] {
            let (e, this, _, _, _) = run_package(&scene, &[]);
            assert_eq!(counters(&e, this), (0, 0));
            assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
            assert!(calls_to(&e, FAKE_SLOT_24).is_empty());
            assert_eq!(e.mem.u32(this.addr() + 0x18), 0x5555);
        }
    }

    #[test]
    fn package_mob_message_ignores_actors_not_flagged_for_processing() {
        let scene = Package {
            process_me: false,
            ..Package::new()
        };
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(counters(&e, this), (0, 0));
        assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
    }

    #[test]
    fn package_mob_message_hands_the_actor_to_its_process() {
        let scene = Package {
            hands_over: true,
            resting: true,
            ..Package::new()
        };
        let (e, this, mob, _, _) = run_package(&scene, &[0xa000]);
        assert_eq!(counters(&e, this), (0, 1));
        assert_eq!(calls_to(&e, FAKE_SLOT_24).len(), 1);
        assert_eq!(calls_to(&e, FAKE_SLOT_24)[0][1..], [mob.addr(), 0]);
        assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
        let process = calls_to(&e, FAKE_SLOT_24)[0][0];
        assert_eq!(e.mem.u8(process + 0x2c7), 1);
        // The rest restores health and fatigue by the same amount.
        assert_eq!(
            calls_to(&e, 0x0088_b510),
            vec![vec![mob.addr(), 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0088_b5a0),
            vec![vec![mob.addr(), 2.0f32.to_bits()]]
        );
        // Handed over: no follower clean-up.
        assert!(calls_to(&e, 0x0042_2690).is_empty());
    }

    #[test]
    fn package_mob_message_plain_update_cleans_up_followers() {
        let scene = Package::new();
        let (e, this, mob, _, _) = run_package(&scene, &[0xa000, 0, 0xb000, 0xc000]);
        // The list ends at the first null entry, as the game's loops do.
        assert_eq!(counters(&e, this), (1, 0));
        assert_eq!(calls_to(&e, FAKE_SLOT_25C), vec![vec![mob.addr(), 0]]);
        assert!(calls_to(&e, 0x0088_b510).is_empty());
        assert_eq!(calls_to(&e, 0x0042_2690), vec![vec![OWNER_EXTRA, 0xa000]]);
        // Both temporary lists are cleared and freed.
        assert_eq!(calls_to(&e, 0x0047_0470).len(), 2);
        assert_eq!(calls_to(&e, 0x0047_02f0).len(), 2);
        // The follower extra is removed once the owner's list is checked.
        assert_eq!(calls_to(&e, 0x0042_2720), vec![vec![OWNER_EXTRA]]);
    }

    #[test]
    fn package_mob_message_follower_cleanup_filters_the_player_and_followers() {
        let scene = Package::new();
        let (e, _, _, _, _) = run_package(&scene, &[PACKAGE_PLAYER, 0xb000, 0xc000, 0xd000]);
        // The player and `0xb000` (which could follow) are skipped.
        assert_eq!(
            calls_to(&e, 0x0042_2690),
            vec![vec![OWNER_EXTRA, 0xc000], vec![OWNER_EXTRA, 0xd000]]
        );
    }

    #[test]
    fn package_mob_message_without_a_follower_owner_does_no_cleanup() {
        // `005d43c0` answers 0 for the guard-free mob: no lists are built.
        let scene = Package::new();
        let (mut e, _, _, _, _) = run_package(&scene, &[0xa000]);
        e.register_double(0x005d_43c0, |_, _| ok(0));
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x25c, FAKE_SLOT_25C)],
        );
        e.mem.set_u8(mob.addr() + 0xbc, 1);
        log_calls(&mut e);
        e.call(0x008c_8be0, &args![this, mob]);
        assert!(calls_to(&e, 0x0042_2690).is_empty());
        assert!(calls_to(&e, 0x0047_0470).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
    }

    #[test]
    fn package_mob_message_clears_the_mob_list_when_the_process_type_changes() {
        let scene = Package {
            later_process_type: 1,
            ..Package::new()
        };
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0);

        let scene = Package::new();
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x5555);
    }

    // ---- ActorsScriptTaskData ----------------------------------------

    #[test]
    fn actors_script_constructor_sets_the_vtable_and_clears_the_counter() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        assert_eq!(
            e.call(0x008c_9020, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTORS_SCRIPT_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    #[test]
    fn actors_script_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_9050, VTABLE_ACTORS_SCRIPT_TASK_DATA, 0x40);
    }

    #[test]
    fn actors_script_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call(0x008c_9080, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    }

    #[test]
    fn actors_script_count_message_clears_the_counter_and_enters_three_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        log_calls(&mut e);
        e.call(0x008c_90a0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC38, 0],
                vec![CRITICAL_SECTION_011DFC04, 0],
                vec![CRITICAL_SECTION_011DFBE8, 0]
            ]
        );
    }

    #[test]
    fn actors_script_end_message_leaves_three_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_90e0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![
                vec![CRITICAL_SECTION_011DFBE8],
                vec![CRITICAL_SECTION_011DFC04],
                vec![CRITICAL_SECTION_011DFC38]
            ]
        );
    }

    #[test]
    fn actors_script_mob_message_runs_scripts_only_when_enabled() {
        let mut e = engine();
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        quiet(&mut e, &[0x0056_5870]);
        let enabled = Rc::new(RefCell::new(true));
        let flag = enabled.clone();
        e.register_double(0x005a_c740, move |_, _| ok(u32::from(*flag.borrow())));
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        let mob = object_with_slots(&mut e, 0x1c4, &[(0x100, FAKE_IS_ACTOR)]);
        log_calls(&mut e);
        e.call(0x008c_9110, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x0056_5870), vec![vec![mob.addr()]]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
        *enabled.borrow_mut() = false;
        e.call(0x008c_9110, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x0056_5870).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
        // A null mob does nothing.
        e.call(0x008c_9110, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
    }
}
