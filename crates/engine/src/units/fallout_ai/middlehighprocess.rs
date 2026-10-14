//! `fallout/ai/middlehighprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MiddleHighProcess` is the AI process of actors near the player: package
//! processing, the equipment queue (`QueuedItem`s), the weapon-drawn state,
//! furniture and the per-frame update. Its layout ([`MiddleHighProcess`])
//! follows the Xbox PDB; the offsets the PC code uses here equal the PDB's.
//! Virtual calls name the byte offset of the slot; where a doc comment
//! gives a slot's Xbox PDB name, it is the name of that offset in the Xbox
//! vtable, not confirmed for the PC one.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MiddleHighProcess` (Xbox PDB), 0x25C bytes (base classes
    /// `MiddleLowProcess`, `LowProcess`, `BaseProcess`): the fields this
    /// unit uses, at the offsets the PC code reads.
    pub struct MiddleHighProcess: 0x25c {
        /// `LowProcess::pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x040 pTarget: Ptr,
        /// `fRadiationDelta` (Xbox PDB).
        0x234 fRadiationDelta: f32,
    }

    /// `QueuedItem` (Xbox PDB), 0x18 bytes: one entry of the equipment
    /// queue.
    pub struct QueuedItem: 0x18 {
        /// `pObject` (Xbox PDB): `TESBoundObject*`.
        0x00 pObject: Ptr,
        /// `pExtra` (Xbox PDB): `ExtraDataList*`.
        0x04 pExtra: Ptr,
        /// `iNumber` (Xbox PDB).
        0x08 iNumber: i32,
        /// `bEquip` (Xbox PDB).
        0x0c bEquip: u8,
        /// `bNow` (Xbox PDB).
        0x0d bNow: u8,
        /// `bForceEquip` (Xbox PDB).
        0x0e bForceEquip: u8,
        /// `bPlayPickUpSounds` (Xbox PDB).
        0x0f bPlayPickUpSounds: u8,
        /// `bForceUnequip` (Xbox PDB).
        0x10 bForceUnequip: u8,
        /// `bEquipCalling` (Xbox PDB).
        0x11 bEquipCalling: u8,
        /// `spQueuedFiles` (Xbox PDB): `NiPointer<QueuedFile>`.
        0x14 spQueuedFiles: Ptr,
    }
}

/// `operator new` (cdecl, the size).
pub(crate) const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (cdecl, the block).
pub(crate) const FREE: u32 = 0x0040_1030;
/// A form's type byte (`TESForm` +0x04), `(form)`.
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// The first word of an object (`(object)`; here the pointer an
/// `NiPointer` holds).
pub(crate) const FIRST_WORD: u32 = 0x0055_9450;
/// `NiPointer<QueuedFile>` constructor `(pointer, value)` (stores the
/// value and takes a reference).
pub(crate) const QUEUED_FILE_POINTER_INIT: u32 = 0x0052_8cb0;
/// `NiPointer<QueuedFile>::operator=` (Xbox PDB), `(pointer, value)`.
pub(crate) const QUEUED_FILE_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// `QueuedFile::QueuedFile` (Xbox PDB), `(file, 1)`.
pub(crate) const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
/// The `ModelLoader` singleton pointer (`this` of
/// `ModelLoader::BuildKFFileList`).
pub(crate) const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `(loader, object, 1, queued file, actor)` (modelloader.cpp).
pub(crate) const QUEUE_OBJECT_FILES: u32 = 0x0044_4d40;
/// `(object)`: an index into the animation-group table at `0118a838`
/// (modelloader.cpp).
pub(crate) const OBJECT_GROUP_INDEX: u32 = 0x0044_6390;
/// The table of animation-group words `OBJECT_GROUP_INDEX` indexes.
pub(crate) const OBJECT_GROUP_TABLE: u32 = 0x0118_a838;
/// `(loader, kf list, 1, queued file, 0)` (modelloader.cpp).
pub(crate) const QUEUE_KF_FILES: u32 = 0x0044_6500;
/// `ModelLoader::BuildKFFileList` (Xbox PDB), `(loader, model, 1, 0, group)`.
pub(crate) const BUILD_KF_FILE_LIST: u32 = 0x0044_7330;
/// A `BSSimpleList`'s scalar deleting destructor `(list, 1)` (also used
/// on the KF file list).
pub(crate) const LIST_DELETE: u32 = 0x0047_02f0;
/// `Animation::GroupLoaded` (Xbox PDB), `(animation, group)`.
pub(crate) const ANIMATION_GROUP_LOADED: u32 = 0x0049_4710;
/// `(animation)`: the first word of the animation's +0xD8.
pub(crate) const ANIMATION_HAS_DATA: u32 = 0x0049_6940;
/// `TESObjectREFR::GetModel` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_GET_MODEL: u32 = 0x0057_15d0;
/// `TESAnimGroup::AnimGroup` (Xbox PDB), cdecl `(0, group, 0x18, 0)`:
/// an animation-group word.
pub(crate) const ANIM_GROUP: u32 = 0x005f_2370;
/// The word at +0x68 of an actor (its process; the engine map's folded
/// name `MiddleHighProcess::GetSavedAcquireObject`).
pub(crate) const WORD_68: u32 = 0x008d_8520;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB), `(buffer, form, 0)`.
pub(crate) const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// Writes bytes to the save buffer `(buffer, data, size, 0)`.
pub(crate) const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer::LoadFormID` (Xbox PDB), `(buffer)`.
pub(crate) const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// Reads bytes from the load buffer `(buffer, data, size)`.
pub(crate) const LOAD_DATA: u32 = 0x0086_4980;
/// Looks a form up by its id (cdecl, `(id)`; tesform.cpp).
pub(crate) const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (cdecl, `(object, 0, source type, target type, 0)`).
pub(crate) const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptor of `TESForm`.
pub(crate) const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The RTTI type descriptor of `TESBoundObject`.
pub(crate) const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// `(reference)`: its extra data list (tesscriptfunctions.cpp).
pub(crate) const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB), `(list)`.
pub(crate) const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `(changes, object, extra list)`: the save index of an extra list
/// (inventorychanges.cpp).
pub(crate) const EXTRA_LIST_SAVE_INDEX: u32 = 0x004d_43c0;
/// `(changes, object, index)`: the extra list with a save index
/// (inventorychanges.cpp).
pub(crate) const EXTRA_LIST_FROM_INDEX: u32 = 0x004d_4480;
/// Releases an `NiPointer` member `(pointer)`.
pub(crate) const POINTER_RELEASE: u32 = 0x0045_cec0;
/// The `TESDataHandler` singleton pointer.
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `(data handler)`: its byte at +0x61D.
pub(crate) const DATA_HANDLER_BYTE_61D: u32 = 0x0042_26e0;
/// The `BGSSaveLoadGame` singleton pointer (`BGSSaveLoadGame::Create`).
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `(save/load game)`: bit 1 of its word at +0x244.
pub(crate) const SAVE_LOAD_FLAG_2: u32 = 0x0042_ce10;
/// `TESObjectREFR::SetMarkerReserved` (Xbox PDB), `(reference, slot,
/// reserved)`.
pub(crate) const SET_MARKER_RESERVED: u32 = 0x0056_8150;
/// `GuardActorPackageData::HandleDeletedReference` (Xbox PDB) (here on an
/// `ActorPackage`), `(package, reference)`.
pub(crate) const PACKAGE_HANDLE_DELETED: u32 = 0x008c_5ec0;
/// `(process)`: the word at +0x04 (the current package's first word,
/// through `00726070`; baseprocess.cpp).
pub(crate) const PROCESS_PACKAGE: u32 = 0x008d_0430;

pub(crate) fn word(e: &Engine, object: u32, offset: u32) -> u32 {
    e.mem.u32(object.wrapping_add(offset))
}

pub(crate) fn byte(e: &Engine, object: u32, offset: u32) -> u8 {
    e.mem.u8(object.wrapping_add(offset))
}
/// `MiddleHighProcess`'s vtable (RTTI `.?AVMiddleHighProcess@@`).
pub(crate) const MIDDLE_HIGH_PROCESS_VTABLE: u32 = 0x0108_904c;
/// `MiddleLowProcess`'s constructor `(process)` (`0092c950`).
pub(crate) const MIDDLE_LOW_CONSTRUCT: u32 = 0x0092_c950;
/// `MiddleLowProcess`'s destructor `(process)` (middlelowprocess.cpp).
pub(crate) const MIDDLE_LOW_DESTRUCT: u32 = 0x0092_ca00;
/// A `BSSimpleList`'s constructor `(list)` (playercharacter.cpp).
pub(crate) const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// A `BSSimpleList`'s destructor `(list)`.
pub(crate) const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Empties a `BSSimpleList` `(list)`.
pub(crate) const LIST_CLEAR: u32 = 0x0047_0470;
/// Whether a `BSSimpleList` (or node) is empty `(list)`: item and next
/// both null.
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Removes a `BSSimpleList`'s head `(list)`.
pub(crate) const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Returns its argument (`(object)`; the address of a list node's item).
pub(crate) const IDENTITY: u32 = 0x0068_15c0;
/// The word at +0x04 of an object (`(node)`: a list node's next node).
pub(crate) const WORD_4: u32 = 0x0072_6070;
/// `ActorPackage`'s constructor `(package)`.
pub(crate) const ACTOR_PACKAGE_CONSTRUCT: u32 = 0x008c_5e20;
/// `ActorPackage`'s destructor `(package)`.
pub(crate) const ACTOR_PACKAGE_DESTRUCT: u32 = 0x008c_5e70;
/// `ActorPackage::Clear` (Xbox PDB), `(package)`.
pub(crate) const ACTOR_PACKAGE_CLEAR: u32 = 0x008c_5ef0;
/// An `NiPointer` constructor `(pointer, value)`.
pub(crate) const POINTER_INIT: u32 = 0x0063_3c90;
/// An `NiPointer` assignment `(pointer, value)` (`0066b0d0`).
pub(crate) const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `FurnitureMark`'s constructor `(mark, 0xff)` (tesfurniture.cpp).
pub(crate) const FURNITURE_MARK_CONSTRUCT: u32 = 0x0050_9890;
/// `FurnitureMark::SetHeading` (Xbox PDB), `(mark, heading)`.
pub(crate) const FURNITURE_MARK_SET_HEADING: u32 = 0x00c5_4550;
/// Stores a byte at +0x0E `(object, value)` (tesobjectrefr.cpp).
pub(crate) const SET_BYTE_E: u32 = 0x0056_8ab0;
/// Copies three words `(destination, source)` (combatgroup.cpp).
pub(crate) const COPY_POINT: u32 = 0x0098_ddd0;
/// The point `COPY_POINT` copies into the furniture marker.
pub(crate) const FURNITURE_MARK_ORIGIN: u32 = 0x011f_426c;
/// `CachedValues`' constructor `(values)` (0x48 bytes).
pub(crate) const CACHED_VALUES_CONSTRUCT: u32 = 0x0086_7510;
/// `MiddleHighProcess::SetCurrentSpell` (Xbox PDB), `(process, spell)`.
pub(crate) const SET_CURRENT_SPELL: u32 = 0x008d_8140;
/// `MiddleHighProcess::SetDesiredTarget` (Xbox PDB), `(process, target)`.
pub(crate) const SET_DESIRED_TARGET: u32 = 0x008d_8180;
/// `memset` (cdecl, `(destination, value, size)`).
pub(crate) const MEMSET: u32 = 0x0040_3d30;
/// The `BSCloneReserver` pointer assignment `(pointer, value)`
/// (`0092c8b0`, this unit).
pub(crate) const CLONE_RESERVER_ASSIGN: u32 = 0x0092_c8b0;
/// The `BSCloneReserver` pointer destructor `(pointer)` (`0092c890`, this
/// unit).
pub(crate) const CLONE_RESERVER_RELEASE: u32 = 0x0092_c890;
/// `NiPointer<QueuedFile>`'s destructor `(pointer)`.
pub(crate) const QUEUED_FILE_POINTER_RELEASE: u32 = 0x0044_cbf0;
/// Stores a word at +0x28 `(object, value)` (testopicinfo.cpp).
pub(crate) const SET_WORD_28: u32 = 0x0061_cc40;
/// -1.0, the initial `fPackageIdleTimer`.
pub(crate) const MINUS_ONE: u32 = 0x0101_2054;
/// `ItemChange`'s scalar deleting destructor `(item, 1)` (modelloader.cpp).
pub(crate) const ITEM_CHANGE_DELETE: u32 = 0x0044_59e0;
/// `(process)`: the run-once package's package (the first word of +0xE4).
pub(crate) const RUN_ONCE_PACKAGE: u32 = 0x008d_83e0;
/// The `TESSaveLoadGame` singleton pointer.
pub(crate) const TES_SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `(save/load game)`: always false (tesactorbasedata.cpp).
pub(crate) const ALWAYS_FALSE: u32 = 0x0047_c850;
/// `TESSaveLoadGame::DeleteForm` (Xbox PDB), `(save/load game, form)`.
pub(crate) const DELETE_FORM: u32 = 0x0085_a2e0;
/// `MiddleHighProcess::ClearRunOncePackage` (Xbox PDB, this unit), `(process)`.
pub(crate) const CLEAR_RUN_ONCE_PACKAGE: u32 = 0x0091_ee70;
/// `MiddleHighProcess::SetAnimation` (Xbox PDB, this unit), `(process,
/// animation)`.
pub(crate) const SET_ANIMATION: u32 = 0x0091_fe80;
/// `(arrow)`: run on each attached arrow when the process goes.
pub(crate) const ARROW_RELEASE: u32 = 0x008d_0240;
/// `LipSynchAnim`'s scalar deleting destructor `(anim, 1)` (the map's
/// `LipSynchAnim::~LipSynchAnim`).
pub(crate) const LIP_SYNCH_ANIM_DELETE: u32 = 0x004d_5850;
/// `ActorPackage::MoveTo` (Xbox PDB), `(package, destination)`.
pub(crate) const ACTOR_PACKAGE_MOVE_TO: u32 = 0x008c_6130;
/// `Animation::ClearGroup` (Xbox PDB), `(animation, group, blend)`.
pub(crate) const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
/// Adds the value at `item` to a `BSSimpleList` `(list, item)`.
pub(crate) const LIST_ADD: u32 = 0x005a_e3d0;
/// `NiPointer<QueuedFile>` assignment from another pointer `(pointer,
/// other)`.
pub(crate) const QUEUED_FILE_POINTER_COPY: u32 = 0x0092_c820;
/// The `BSCloneReserver` pointer assignment from another pointer
/// `(pointer, other)` (`0092c900`, this unit).
pub(crate) const CLONE_RESERVER_COPY: u32 = 0x0092_c900;
/// `NiPointer<PathingDebugGeometryData>::operator=` (Xbox PDB; here on
/// the character controller pointer), `(pointer, other)`.
pub(crate) const CONTROLLER_POINTER_COPY: u32 = 0x006e_5cc0;
/// `TESObjectREFR::SetTargeted` (Xbox PDB), `(reference, targeted)`.
pub(crate) const SET_TARGETED: u32 = 0x0056_4db0;
/// `memcpy` wrapper (cdecl, `(destination, source, size)`).
pub(crate) const MEMCPY: u32 = 0x0040_1460;
/// An empty function `(form)` (tesform.cpp).
pub(crate) const EMPTY_FUNCTION: u32 = 0x0048_3710;
/// `(process, flag)`: sets or clears flag 0x40 of the process
/// (`008d7e00`; lowprocess.cpp).
pub(crate) const SET_PROCESS_FLAG_40: u32 = 0x0090_ac20;
/// Whether a queued file has finished `(file)` (modelloader.cpp).
pub(crate) const QUEUED_FILE_DONE: u32 = 0x0043_caa0;
/// Stores a form's type byte (+0x04), `(form, type)`.
pub(crate) const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `Actor::EquipObject` (Xbox PDB), `(actor, object, count, extra list,
/// now, force, sounds)`.
pub(crate) const ACTOR_EQUIP_OBJECT: u32 = 0x0088_c830;
/// `Actor::UnEquipObject` (Xbox PDB), `(actor, object, count, extra list,
/// force, calling, sounds)`.
pub(crate) const ACTOR_UNEQUIP_OBJECT: u32 = 0x0088_d7d0;
/// `Actor::SetBlock` (Xbox PDB), `(actor, block)`.
pub(crate) const ACTOR_SET_BLOCK: u32 = 0x0089_4cc0;
/// `Actor::GetCurrentWeapon` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Actor::GetAnimAction` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Animation::AddAnimationsFromList` (Xbox PDB), `(animation, list)`.
pub(crate) const ANIMATION_ADD_FROM_LIST: u32 = 0x0049_0400;
/// The task manager singleton pointer (`this` of
/// `BSTaskManager::TryCancelTask`).
pub(crate) const TASK_MANAGER: u32 = 0x0120_2d98;
/// `BSTaskManager<__int64>::TryCancelTask` (Xbox PDB), `(manager, task,
/// 0)`.
pub(crate) const TRY_CANCEL_TASK: u32 = 0x0052_8870;
/// Removes the value at `item` from a `BSSimpleList` `(list, item)`
/// (highprocess.cpp).
pub(crate) const LIST_REMOVE: u32 = 0x0090_5330;
/// Appends the value at `item` to a `BSSimpleList` `(list, item)`
/// (highprocess.cpp).
pub(crate) const LIST_APPEND: u32 = 0x0090_5820;
/// A debug scope guard's constructor `(guard, 0x32, 1, file, line)` and
/// destructor `(guard)`.
pub(crate) const SCOPE_ENTER: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_LEAVE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\ai\MiddleHighProcess.cpp"`.
pub(crate) const SOURCE_FILE: u32 = 0x0108_98c0;
/// The word at +0x20 of a reference (its base form; the engine map's
/// folded name `BGSSaveFormBuffer::GetForm`).
pub(crate) const REFERENCE_BASE: u32 = 0x007a_f430;
/// The RTTI type descriptor of `TESNPC`.
pub(crate) const TYPE_TES_NPC: u32 = 0x0118_3a1c;
/// The RTTI type descriptor of `TESCreature`.
pub(crate) const TYPE_TES_CREATURE: u32 = 0x0118_3a00;
/// A package's type (the signed byte at +0x20), `(package)`.
pub(crate) const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// Whether bit 0x200000 of a package's flags (+0x1C) is set `(package)`.
pub(crate) const PACKAGE_FLAG_200000: u32 = 0x0044_1b00;
/// `(npc, actor, a, b, c, 1)`: an NPC's default outfit on `actor`
/// (tesnpc.cpp).
pub(crate) const NPC_INIT_DEFAULT_WORN: u32 = 0x0060_47c0;
/// `TESCreature::InitDefaultWorn` (Xbox PDB), `(creature, actor, a, b, 1)`.
pub(crate) const CREATURE_INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// The player character singleton pointer.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB), `(item, effect)`.
pub(crate) const ITEM_CHANGE_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// An item change's form (its first word), `(item)`.
pub(crate) const ITEM_CHANGE_FORM: u32 = 0x0044_ddc0;
/// `(actor)`: with a process (+0x68), `00884940` with 0x2000 and 0x1000
/// (actor.cpp).
pub(crate) const ACTOR_UPDATE_DRAWN_FLAGS: u32 = 0x0088_4f80;
/// `(actor)`: the actor's byte at +0x70 (actor.cpp, through `0097f2f0`).
pub(crate) const ACTOR_BYTE_70: u32 = 0x008b_3b90;
/// Whether bit 0x20 of a form's flags (+0x08) is set, `(form)`.
pub(crate) const FORM_FLAG_20: u32 = 0x0044_0d80;
/// `TESPackage::GetLocationReference` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_GET_LOCATION_REFERENCE: u32 = 0x0067_6140;
/// `TESObjectREFR::IsFurniture` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_IS_FURNITURE: u32 = 0x0056_8680;
/// Two form pointers `ProcessActivate` compares the target's base form
/// with.
pub(crate) const ACTIVATE_FORM_A: u32 = 0x011c_a248;
pub(crate) const ACTIVATE_FORM_B: u32 = 0x011c_a244;
/// `(reference)`: `00418460` on its extra data list (the door teleport
/// data the door paths read; tesobjectrefr.cpp).
pub(crate) const REFERENCE_DOOR_TELEPORT: u32 = 0x0056_8e50;
/// `TESObjectREFR::GetLinkedDoorTeleportPosition` (Xbox PDB),
/// `(reference)`: a pointer to three floats.
pub(crate) const REFERENCE_DOOR_POSITION: u32 = 0x0056_8fa0;
/// `(point, out, other)`: `out = point - other` (three floats).
pub(crate) const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `(point)`: its length (`ST0`; tes.cpp).
pub(crate) const POINT_LENGTH: u32 = 0x0045_7990;
/// `(setting)`: the address of a game setting's value.
pub(crate) const SETTING_VALUE: u32 = 0x0043_d4d0;
/// The integer game setting `ProcessActivate` compares the door distance
/// with.
pub(crate) const DOOR_DISTANCE_SETTING: u32 = 0x011c_de98;
/// `TESObjectREFR::GetMarkerUsed` (Xbox PDB), `(reference, index, 1)`.
pub(crate) const REFERENCE_GET_MARKER_USED: u32 = 0x0056_7f80;
/// `Actor::IsPathingComplete` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// The word at +0x40 of a reference (its cell), `(reference)`.
pub(crate) const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetClosestFreeMarker` (Xbox PDB), `(reference, point,
/// 1, 1, mark, index out, 1)`.
pub(crate) const REFERENCE_GET_CLOSEST_FREE_MARKER: u32 = 0x0056_86b0;
/// `Calendar::GetHour` (Xbox PDB), `(calendar)`.
pub(crate) const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// The `Calendar` object (its address).
pub(crate) const CALENDAR: u32 = 0x011d_e7b8;
/// The float at +0x20 of an object (`ST0`), `(object)`.
pub(crate) const FLOAT_20: u32 = 0x007d_f1f0;
/// A global's value (`ST0`; the engine map's folded name
/// `BSMultiBoundCapsule::QMultiBoundRadius`), `(global)`.
pub(crate) const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB), `(actor)`.
pub(crate) const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MobileObject::GetDesiredProcessLevel` (Xbox PDB), `(actor)`.
pub(crate) const GET_DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
/// Whether bit 2 of a package's flags (+0x1C) is set, `(package)`.
pub(crate) const PACKAGE_FLAG_4: u32 = 0x0067_efd0;
/// `MiddleHighProcess::AlignToFurnitureAndAddPostAnimSitSleep` (Xbox PDB,
/// this unit), `(process, actor)`.
pub(crate) const ALIGN_TO_FURNITURE: u32 = 0x0092_1350;
/// `TESObjectREFR::Activate` (Xbox PDB), `(reference, actor, a, object,
/// 1)`.
pub(crate) const REFERENCE_ACTIVATE: u32 = 0x0057_3170;
/// `TESObjectREFR::RunScript` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_RUN_SCRIPT: u32 = 0x0056_5870;
/// The acquire object's scalar deleting destructor `(object, 1)`.
pub(crate) const ACQUIRE_OBJECT_DELETE: u32 = 0x007b_3fa0;
/// `CombatController::IsActoraCombatTarget` (Xbox PDB), `(controller,
/// actor)`.
pub(crate) const IS_COMBAT_TARGET: u32 = 0x0097_fa10;
/// `(player, actor)`: a signed count `EnterCombat` requires positive
/// before adding the actor to the player's combat list.
pub(crate) const PLAYER_COMBAT_STANDING: u32 = 0x008a_8230;
/// `PlayerCharacter::AddActorToPlayerCombatList` (Xbox PDB), `(player,
/// actor)`.
pub(crate) const ADD_TO_PLAYER_COMBAT_LIST: u32 = 0x0093_a690;
/// `LowProcess::EnterCombat` (Xbox PDB), 13 words after the process.
pub(crate) const LOW_PROCESS_ENTER_COMBAT: u32 = 0x0090_dc10;
/// `(actor)`: whether `004f8960` gives 5 (extradataobjects.cpp).
pub(crate) const ACTOR_STATE_IS_5: u32 = 0x0043_7bf0;
/// `(actor)`: whether `004f8960` gives 3 (extradataobjects.cpp).
pub(crate) const ACTOR_STATE_IS_3: u32 = 0x0043_7bd0;
/// The object `008d0300` is called on.
pub(crate) const COMBAT_CHECK_OBJECT: u32 = 0x011f_1958;
/// `(object, actor, target)`: always false (baseprocess.cpp).
pub(crate) const ALWAYS_FALSE_2: u32 = 0x008d_0300;
/// `PlayerCharacter::GetNumberActorsInCombat` (Xbox PDB), `(player)`.
pub(crate) const GET_NUMBER_ACTORS_IN_COMBAT: u32 = 0x0095_3c20;
/// The integer game setting the player's combat count is compared with.
pub(crate) const COMBAT_LIMIT_SETTING: u32 = 0x011c_cffc;
/// A reference's byte at +0x14D, `(reference)`.
pub(crate) const REFERENCE_BYTE_14D: u32 = 0x005a_2030;
/// `(actor)`: `005f0ba0` on the actor's `004181e0` (actor.cpp).
pub(crate) const ACTOR_0087F3B0: u32 = 0x0087_f3b0;
/// `Actor::SwimsOnly` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_SWIMS_ONLY: u32 = 0x0087_f570;
/// `TESPackage::IsInterruptPackage` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_IS_INTERRUPT: u32 = 0x0067_8610;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB), `(list)`.
pub(crate) const GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// Whether bit 0x400000 of a package's flags (+0x1C) is set, `(package)`.
pub(crate) const PACKAGE_FLAG_400000: u32 = 0x0067_a3f0;
/// `(package, actor)`: the package's radius as an unsigned integer
/// (package.cpp).
pub(crate) const PACKAGE_RADIUS: u32 = 0x0067_6280;
/// 1.0 (a double).
pub(crate) const ONE_DOUBLE: u32 = 0x0101_2070;
/// 600.0, the radius used below 1.0.
pub(crate) const DEFAULT_RADIUS: u32 = 0x0101_786c;
/// `(reference, point)`: its distance to the point (`ST0`;
/// tesobjectrefr.cpp).
pub(crate) const REFERENCE_DISTANCE_TO: u32 = 0x0057_2380;
/// A reference's byte at +0x104, `(reference)` (animation.cpp).
pub(crate) const REFERENCE_BYTE_104: u32 = 0x0049_3bb0;
/// `(actor)`: an item change built for the actor (actor.cpp; `ProcessEscort`
/// reads its first extra list and form, then deletes it).
pub(crate) const ACTOR_ITEM_CHANGE: u32 = 0x008a_2d40;
/// The word at +0x0C of an object, `(object)`.
pub(crate) const WORD_C: u32 = 0x0084_e3a0;
/// `(reference, form)`: with a container, `004cfe20` on its inventory changes.
pub(crate) const REFERENCE_INVENTORY_TEST: u32 = 0x0057_5450;
/// The RTTI type descriptor of `TESObjectREFR`.
pub(crate) const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// The RTTI type descriptor of `Actor`.
pub(crate) const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `TESPackage::GetIsCreated` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
/// The word at +0x30 of a package, `(package)`; `ProcessEscort` reads the
/// first word of what it points to as an integer distance.
pub(crate) const PACKAGE_WORD_30: u32 = 0x0067_1d10;
/// Bit 0 of a cell's byte at +0x24, `(cell)`.
pub(crate) const CELL_FLAG: u32 = 0x0042_5fd0;
/// `(setting)`: the address of a float game setting's value.
pub(crate) const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The float game settings `ProcessEscort` reads: the distance when the
/// actor's cell has `CELL_FLAG`, and the factor on the package's
/// distance.
pub(crate) const ESCORT_CELL_DISTANCE_SETTING: u32 = 0x011c_dbac;
pub(crate) const ESCORT_DISTANCE_FACTOR_SETTING: u32 = 0x011c_d82c;
/// 0.0 (a double).
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The word at +0x2C of a package (its location data), `(package)`.
pub(crate) const PACKAGE_LOCATION: u32 = 0x0055_b980;
/// `ExtraDataList::GetReferencePointer` (Xbox PDB), `(list)`.
pub(crate) const GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
/// `PackageLocation::GetLocReference` (Xbox PDB), `(location)`.
pub(crate) const LOCATION_GET_REFERENCE: u32 = 0x0067_f390;
/// `TESObjectREFR::HasContainer` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_HAS_CONTAINER: u32 = 0x0055_d310;
/// `TESPackage::GetInitialTargetCount` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_INITIAL_TARGET_COUNT: u32 = 0x0067_2930;
/// `(actor, form, extra list, container, count, owner)`, actor.cpp.
pub(crate) const ACTOR_ITEM_TO_CONTAINER: u32 = 0x0089_2e90;
/// `MobileObject::GetCurrentPackage` (Xbox PDB), `(actor)`.
pub(crate) const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `TESPackage::GetLocationCoord` (Xbox PDB), `(package, out, actor)`.
pub(crate) const PACKAGE_GET_LOCATION_COORD: u32 = 0x0067_5de0;
/// `TESPackage::GetLocationWorld` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_GET_LOCATION_WORLD: u32 = 0x0067_5a50;
/// `TESPackage::GetLocationCell` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_GET_LOCATION_CELL: u32 = 0x0067_5c20;
/// `Actor::EndMovement` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_END_MOVEMENT: u32 = 0x0087_faa0;
/// 3600.0, the seconds `ProcessAccompanyOneHour` passes.
pub(crate) const ACCOMPANY_SECONDS: u32 = 0x0108_4838;
/// `MiddleHighProcess::FindBedChairs` (Xbox PDB, this unit), `(process,
/// actor, 0, 0)`.
pub(crate) const FIND_BED_CHAIRS: u32 = 0x0092_2670;
/// A `BSSimpleList`'s item count `(list)` (the engine map's folded name
/// `VATS::GetCount`).
pub(crate) const LIST_COUNT: u32 = 0x005a_e380;
/// `TryEnterCriticalSection` wrapper, `(section)`.
pub(crate) const TRY_ENTER_LOCK: u32 = 0x0044_b130;
/// `LeaveCriticalSection` wrapper, `(section)`.
pub(crate) const LEAVE_LOCK: u32 = 0x0082_f1f0;
/// The critical section `ProcessSleep` and `ProcessEat` take furniture
/// under.
pub(crate) const FURNITURE_LOCK: u32 = 0x011e_043c;
/// `MiddleHighProcess::ClearFurnitureMarker` (Xbox PDB, this unit),
/// `(process)`.
pub(crate) const CLEAR_FURNITURE_MARKER: u32 = 0x0092_c680;
/// `TESObjectREFR::GetOwner` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
/// A random number (no arguments; `00476c00`, `00476be0`).
pub(crate) const RANDOM: u32 = 0x0048_7f50;
/// `Actor::GetBestFoodItem` (Xbox PDB), `(actor)`: an item change.
pub(crate) const ACTOR_GET_BEST_FOOD_ITEM: u32 = 0x0089_1d30;
/// `Actor::Eat` (Xbox PDB), `(actor, form, extra list, 1)`.
pub(crate) const ACTOR_EAT: u32 = 0x008c_1de0;
/// `(actor)`: slot 0x20 of the actor's object at +0x190 (0 without one).
pub(crate) const ACTOR_MOVEMENT_FLAGS: u32 = 0x0088_46e0;
/// `Actor::ClearMoveMode` (Xbox PDB), `(actor, flags)`.
pub(crate) const ACTOR_CLEAR_MOVE_MODE: u32 = 0x008b_3a80;
/// `MobileObject::IsinDialogue` (Xbox PDB), `(actor)`.
pub(crate) const MOBILE_IS_IN_DIALOGUE: u32 = 0x0093_36c0;
/// `Actor::IsAlarmed` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_IS_ALARMED: u32 = 0x008a_61b0;
/// `Actor::IsRunningRunOnce` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_IS_RUNNING_RUN_ONCE: u32 = 0x0088_1570;
/// `_ftol2`: the `ST0` value (a leading `f64`) truncated to an integer.
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `LowProcess::CheckforNewPackage` (Xbox PDB), `(process, actor, force)`.
pub(crate) const LOW_PROCESS_CHECKFOR_NEW_PACKAGE: u32 = 0x0090_a1a0;
/// `Actor::SetPathfindingGoal_ov2` (Xbox PDB), `(actor, point, cell,
/// world, radius, 0)`.
pub(crate) const ACTOR_SET_PATHFINDING_GOAL: u32 = 0x008b_3690;
/// Whether `bits` of the player's byte at +0x680 are set, `(player, bits)`.
pub(crate) const PLAYER_FLAGS_680: u32 = 0x005a_03f0;
/// The RTTI type descriptor of `TESPackage`.
pub(crate) const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
/// The RTTI type descriptor of `TrespassPackage`.
pub(crate) const TYPE_TRESPASS_PACKAGE: u32 = 0x011a_3328;
/// The RTTI type descriptor of `BaseProcess`.
pub(crate) const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
/// The RTTI type descriptor of `HighProcess`.
pub(crate) const TYPE_HIGH_PROCESS: u32 = 0x0118_a6d4;
/// The RTTI type descriptor of `Character`.
pub(crate) const TYPE_CHARACTER: u32 = 0x011a_0f14;
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB), `(reference,
/// other, 0, 0)` (`ST0`).
pub(crate) const REFERENCE_GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// 200.0 (a double): beyond it `ProcessPursue` follows instead.
pub(crate) const PURSUE_FOLLOW_DISTANCE: u32 = 0x0101_79e0;
/// `(actor, x, y, z, 0)` (actor.cpp).
pub(crate) const ACTOR_CALL_008BB520: u32 = 0x008b_b520;
/// The word at +0x84 of a package, `(package)`.
pub(crate) const PACKAGE_WORD_84: u32 = 0x004f_d3c0;
/// The float at +0x80 of a package (`ST0`), `(package)`.
pub(crate) const PACKAGE_FLOAT_80: u32 = 0x0064_4790;
/// Adds to the float at +0x80 of a package, `(package, value)`.
pub(crate) const PACKAGE_ADD_FLOAT_80: u32 = 0x008d_b4c0;
/// `(reference, actor, 0, -1)` (actor.cpp).
pub(crate) const ACTOR_CALL_008C0EC0: u32 = 0x008c_0ec0;
/// The float at +0x0C of an object (`ST0`), `(object)`.
pub(crate) const FLOAT_C: u32 = 0x0084_d030;
/// The global time data whose +0x0C is the seconds passed this frame.
pub(crate) const GLOBAL_TIME_DATA: u32 = 0x011f_6394;
/// -1.0 (a double).
pub(crate) const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// The float game setting `ProcessPursue` compares the package's +0x80
/// with.
pub(crate) const PURSUE_TIME_SETTING: u32 = 0x011c_d2c4;
/// `PlayerCharacter::IsSleepingorResting` (Xbox PDB), `(player)`.
pub(crate) const PLAYER_IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// Stores the player's +0x654 and byte +0x658, `(player, word, byte)`.
pub(crate) const PLAYER_SET_654: u32 = 0x005c_1a00;
/// An actor's byte at +0x7C4, `(actor)`.
pub(crate) const ACTOR_BYTE_7C4: u32 = 0x008a_08c0;
/// The RTTI type descriptor of `FleePackage`.
pub(crate) const TYPE_FLEE_PACKAGE: u32 = 0x0118_c6f4;
/// `FleePackage::AddAvoidedRef` (Xbox PDB), `(package, actor)`.
pub(crate) const FLEE_ADD_AVOIDED_REF: u32 = 0x009f_1310;
/// `MagicItem::IsLoaded` (Xbox PDB), `(item)`.
pub(crate) const MAGIC_ITEM_IS_LOADED: u32 = 0x0040_b470;
/// `MagicItem::IsLoading` (Xbox PDB), `(item)`.
pub(crate) const MAGIC_ITEM_IS_LOADING: u32 = 0x0040_b6b0;
/// `MagicItem::Preload` (Xbox PDB), `(item, 0)`.
pub(crate) const MAGIC_ITEM_PRELOAD: u32 = 0x0040_a420;
/// `MagicItem::Unload` (Xbox PDB), `(item, 0)`.
pub(crate) const MAGIC_ITEM_UNLOAD: u32 = 0x0040_b800;
/// `MagicCaster::CastSpell` (Xbox PDB), `(caster, spell, target, 0)`.
pub(crate) const MAGIC_CASTER_CAST_SPELL: u32 = 0x0081_5480;
/// `(package, process)` (fleepackage.cpp).
pub(crate) const FLEE_UPDATE: u32 = 0x009f_1830;
/// `PackageTarget::GetTargReference` (Xbox PDB), `(target)`.
pub(crate) const PACKAGE_TARGET_GET_REFERENCE: u32 = 0x0068_0020;
/// The float at +0x90 of a package (`ST0`), `(package)`.
pub(crate) const FLEE_FLOAT_90: u32 = 0x006c_a4e0;
/// Adds to the float at +0x90 of a package, `(package, value)`.
pub(crate) const FLEE_ADD_FLOAT_90: u32 = 0x0089_8250;
/// Stores the byte at +0x94 of a package, `(package, value)`.
pub(crate) const FLEE_SET_BYTE_94: u32 = 0x0081_0530;
/// `FleePackage::GetClosestAvoidedRef` (Xbox PDB), `(package, actor)`.
pub(crate) const FLEE_GET_CLOSEST_AVOIDED_REF: u32 = 0x009f_1210;
/// The float game setting `ProcessFlee` compares the package's +0x90
/// with.
pub(crate) const FLEE_TIME_SETTING: u32 = 0x011c_d348;
/// The float game settings for the flee distance (cells with `CELL_FLAG`,
/// and the others).
pub(crate) const FLEE_NEAR_CELL_SETTING: u32 = 0x011c_e570;
pub(crate) const FLEE_NEAR_SETTING: u32 = 0x011c_f998;
/// The word at +0xA4 of a package (`ProcessFlee` paths to it when set), `(package)`.
pub(crate) const FLEE_DOOR: u32 = 0x005e_3fc0;
/// `FleePackage::FindTeleportDoor` (Xbox PDB), `(package, actor, avoided)`.
pub(crate) const FLEE_FIND_TELEPORT_DOOR: u32 = 0x009f_1140;
/// `PathingRequestFlee::PathingRequestFlee` (Xbox PDB), `(request)`.
pub(crate) const PATHING_REQUEST_FLEE_CONSTRUCT: u32 = 0x006e_5390;
/// Calls on the pathing request, each `(request, value)`: `006e29f0` with
/// the actor; stores at +0x90 (and 1 at +0xA0), the byte +0x98, +0x74,
/// +0xB0; `006d61e0` (through `006dadb0`); the goal location assignment
/// (`PathingLocation::operator=`).
pub(crate) const PATHING_REQUEST_FROM_ACTOR: u32 = 0x006e_29f0;
pub(crate) const PATHING_REQUEST_SET_90: u32 = 0x006e_2bb0;
pub(crate) const PATHING_REQUEST_SET_98: u32 = 0x006e_2b50;
pub(crate) const PATHING_REQUEST_SET_74: u32 = 0x006d_3b00;
pub(crate) const PATHING_REQUEST_006D61E0: u32 = 0x006d_61e0;
pub(crate) const PATHING_REQUEST_SET_B0: u32 = 0x0050_7610;
pub(crate) const PATHING_REQUEST_SET_LOCATION: u32 = 0x006d_1bc0;
/// `(request)`: `006d2c00` on the request (highprocess.cpp).
pub(crate) const PATHING_REQUEST_008DF000: u32 = 0x008d_f000;
/// The pathing request's destructor `(request)`.
pub(crate) const PATHING_REQUEST_DESTRUCT: u32 = 0x006d_ad70;
/// `(actor)`: the object whose +0x30 `ProcessFlee` asks (slot 0x28) for
/// the request's flag (extradatalist.cpp).
pub(crate) const ACTOR_EXTRA_OBJECT: u32 = 0x0041_81e0;
/// `(package)`: the value `ProcessFlee` passes to `006d61e0`
/// (fleepackage.cpp).
pub(crate) const FLEE_PATH_MODE: u32 = 0x009f_1440;
/// 1000.0, the value `ProcessFlee` passes to `00507610`.
pub(crate) const FLEE_PATH_RANGE: u32 = 0x0101_3974;
/// `PathingLocation::PathingLocation_ov3` (Xbox PDB), `(location, reference)`.
pub(crate) const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cd70;
/// The pathing location's destructor `(location)`.
pub(crate) const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `Actor::SetPathfindingFlee` (Xbox PDB), `(actor, request)`.
pub(crate) const ACTOR_SET_PATHFINDING_FLEE: u32 = 0x008b_b630;
/// Whether bit 0x10000 of a package's flags (+0x1C) is set, `(package)`.
pub(crate) const PACKAGE_FLAG_10000: u32 = 0x008a_6290;
/// `(player, actor)`: a test on the player
/// (playercharacter.cpp).
pub(crate) const PLAYER_TEST_00962720: u32 = 0x0096_2720;
/// `(player)`: a count of the player's (playercharacter.cpp).
pub(crate) const PLAYER_COUNT_00962620: u32 = 0x0096_2620;
/// The integer game setting that count is compared with.
pub(crate) const FOLLOW_COUNT_LIMIT_SETTING: u32 = 0x011c_dad0;
/// `(package, 0)` (package.cpp).
pub(crate) const PACKAGE_CALL_00674F30: u32 = 0x0067_4f30;
/// `TESPackage::GetPackageRadiusActorToRefTarget` (Xbox PDB), `(package,
/// actor, 0)` (`ST0`).
pub(crate) const PACKAGE_GET_RADIUS_TO_TARGET: u32 = 0x0067_87e0;
/// `PackageLocation::GetLocType` (Xbox PDB), `(location)`.
pub(crate) const LOCATION_GET_TYPE: u32 = 0x0067_8ca0;
/// The float game setting `ProcessFollow` compares how far the target
/// moved with.
pub(crate) const FOLLOW_MOVED_SETTING: u32 = 0x011c_d950;
/// `Actor::SetMoveMode` (Xbox PDB), `(actor, mode)`.
pub(crate) const ACTOR_SET_MOVE_MODE: u32 = 0x008b_39f0;
/// 5000.0, the search radius `ProcessTravel` passes.
pub(crate) const TRAVEL_SEARCH_RADIUS: u32 = 0x0103_0020;
/// 20.0, the radius of travel packages of type 5.
pub(crate) const TRAVEL_SMALL_RADIUS: u32 = 0x0101_7868;
/// `TESDataHandler::EnumReferencesCloseToPoint` (Xbox PDB), `(handler,
/// cell, point, radius, point, radius, callback, actor)`.
pub(crate) const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
/// The two callbacks `ProcessTravel` passes to it (lowprocess.cpp).
pub(crate) const TRAVEL_SEARCH_CALLBACK_A: u32 = 0x0090_d480;
pub(crate) const TRAVEL_SEARCH_CALLBACK_B: u32 = 0x0090_d570;
/// `TESPackage::GetPackageRadiusActorToLocation` (Xbox PDB), `(package,
/// actor, 0)` (`ST0`).
pub(crate) const PACKAGE_GET_RADIUS_TO_LOCATION: u32 = 0x0067_8670;
/// Whether bit 0x20 / 0x100 of a package's flags (+0x1C) is set,
/// `(package)`.
pub(crate) const PACKAGE_FLAG_20: u32 = 0x0067_0f40;
pub(crate) const PACKAGE_FLAG_100: u32 = 0x0067_0f60;
/// `Actor::QueueUnEquipObject` (Xbox PDB), `(actor, form, 1, 0, 0, 0, 1)`.
pub(crate) const ACTOR_QUEUE_UNEQUIP_OBJECT: u32 = 0x0088_c790;
/// The RTTI type descriptor of `TESFurniture`.
pub(crate) const TYPE_TES_FURNITURE: u32 = 0x0118_a220;
/// A furniture marker's heading (its `u16` at +0x0C over 1000, `ST0`),
/// `(mark)`.
pub(crate) const MARK_HEADING: u32 = 0x0056_8650;
/// A furniture marker's byte at +0x0E, `(mark)`.
pub(crate) const MARK_BYTE_E: u32 = 0x0050_94b0;
/// `TESObjectREFR::GetScale` (Xbox PDB), `(reference)` (`ST0`).
pub(crate) const REFERENCE_GET_SCALE: u32 = 0x0056_7400;
/// `TESFurniture::GetMarkerTargetOffset` (Xbox PDB), `(furniture, out,
/// marker, scale)`.
pub(crate) const FURNITURE_GET_MARKER_TARGET_OFFSET: u32 = 0x0050_9920;
/// A rotation matrix from a heading, `(matrix, heading)`
/// (bgsdecalmanager.cpp).
pub(crate) const ROTATION_FROM_HEADING: u32 = 0x004a_0c90;
/// `out = matrix * point`, `(matrix, out, point)` (globalfunc.cpp).
pub(crate) const ROTATE_POINT: u32 = 0x004b_4500;
/// `TESObjectREFR::SetMarkerUsed` (Xbox PDB), `(reference, index, 1)`.
pub(crate) const REFERENCE_SET_MARKER_USED: u32 = 0x0056_8020;
/// `out = point + other`, `(point, out, other)`.
pub(crate) const POINT_ADD: u32 = 0x0043_9e90;
/// `(furniture, marker)`: a marker's angle (`ST0`).
pub(crate) const FURNITURE_MARKER_ANGLE: u32 = 0x0050_99c0;
/// `(mobile object, angle)`: its slot 0x2C4 with slot 0x2BC plus `angle`
/// (mobileobject.cpp).
pub(crate) const MOBILE_TURN_BY: u32 = 0x0093_1d30;
/// `Actor::GetFaceAnimationData` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_GET_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
/// The address of a reference's +0x24, `(reference)`.
pub(crate) const REFERENCE_PLUS_24: u32 = 0x0043_0830;
/// The word at +0x18 of a package, `(package)`.
pub(crate) const PACKAGE_WORD_18: u32 = 0x0096_11e0;
/// `Procedure::GetProcedureCount` (Xbox PDB), cdecl `(procedure)`.
pub(crate) const PROCEDURE_GET_COUNT: u32 = 0x0096_a820;
/// The object `Update` passes to the empty function `00483710` at its
/// start and end.
pub(crate) const UPDATE_SCOPE_OBJECT: u32 = 0x011e_0460;
/// Whether bit 0x800 of a form's flags (+0x08) is set, `(form)`.
pub(crate) const FORM_FLAG_800: u32 = 0x0044_0da0;
/// Whether `00450fd0` gives 3 for a cell, `(cell)` (tes.cpp).
pub(crate) const CELL_STATE_IS_3: u32 = 0x0045_0fb0;
/// cdecl `(actor, value)`: the new value of the process's byte +0x18A.
pub(crate) const BYTE_18A_UPDATE: u32 = 0x0080_ced0;
/// `(actor)` (actor.cpp), run when a new package was chosen.
pub(crate) const ACTOR_CALL_008A8F60: u32 = 0x008a_8f60;
/// The table of procedure tables (`011a3ff0[kind]` points at the
/// procedure values of one package kind, by procedure index).
pub(crate) const PROCEDURE_TABLES: u32 = 0x011a_3ff0;
/// `Script::SetActionFlag` (Xbox PDB), cdecl `(package, extra list,
/// flag)`.
pub(crate) const SCRIPT_SET_ACTION_FLAG: u32 = 0x005a_c750;
/// The address of a package's +0x38, `(package)`.
pub(crate) const PACKAGE_PLUS_38: u32 = 0x0041_d8a0;
/// `TESPackage::GetPatrolPackageData` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_PATROL_DATA: u32 = 0x0067_5920;
/// The byte at +0x04 of the patrol data, `(data)`.
pub(crate) const PATROL_BYTE_4: u32 = 0x004f_1540;
/// `TESPackage::GetOncePerDay` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_ONCE_PER_DAY: u32 = 0x0067_0f90;
/// `Calendar::GetDay` (Xbox PDB), `(calendar)`.
pub(crate) const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
/// `Interface::GetTargetREFR` (Xbox PDB), no arguments (its result is not
/// used here).
pub(crate) const INTERFACE_GET_TARGET_REFR: u32 = 0x0070_3180;
/// `Actor::IsWaitingOnPath` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
/// 2.0 (a double).
pub(crate) const TWO_DOUBLE: u32 = 0x0101_1590;
/// `TESPackage::IsTargetAnActor` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_IS_TARGET_AN_ACTOR: u32 = 0x0067_8ec0;
/// `(package, actor, 0)`: the package's actor target (package.cpp).
pub(crate) const PACKAGE_ACTOR_TARGET: u32 = 0x0067_80e0;
/// `PatrolActorPackageData::GetCurrentReference` (Xbox PDB), `(data)`.
pub(crate) const PATROL_GET_CURRENT_REFERENCE: u32 = 0x009f_3390;
/// `PatrolActorPackageData::GetInitialReferenceForSetup` (Xbox PDB), cdecl
/// `(actor, package)`.
pub(crate) const PATROL_GET_INITIAL_REFERENCE: u32 = 0x009f_2d40;
/// `DialoguePackage::GetTargetOfConversation` (Xbox PDB), `(package)`.
pub(crate) const DIALOGUE_GET_TARGET_OF_CONVERSATION: u32 = 0x008d_80e0;
/// Whether bit 0x200000 of a form's flags (+0x08) is set, `(form)`.
pub(crate) const FORM_FLAG_200000: u32 = 0x0057_6d30;
/// `PackageTarget::GetTargType` (Xbox PDB), `(target)`.
pub(crate) const TARGET_GET_TYPE: u32 = 0x0051_9b00;
/// `PackageTarget::GetTargObject` (Xbox PDB), `(target)`.
pub(crate) const TARGET_GET_OBJECT: u32 = 0x0068_0050;
/// `PackageTarget::GetTargObjectType` (Xbox PDB), `(target)`.
pub(crate) const TARGET_GET_OBJECT_TYPE: u32 = 0x0068_0080;
/// `TESPackage::SetNeverRun` (Xbox PDB), `(package, reference, 1)`.
pub(crate) const PACKAGE_SET_NEVER_RUN: u32 = 0x0067_4e70;
/// `(reference)`: `0041e410` on its extra data list (tesobjectrefr.cpp).
pub(crate) const REFERENCE_SELF_TARGET: u32 = 0x0056_9b80;
/// The address of a form list's +0x18 (its entries), `(list)`.
pub(crate) const FORM_LIST_ENTRIES: u32 = 0x0050_0940;
/// The RTTI type descriptor of `MagicItem`.
pub(crate) const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
/// `LowProcess::BuildMulitTargetObjectList` (Xbox PDB), `(process, actor,
/// location, type)`.
pub(crate) const BUILD_MULTI_TARGET_LIST: u32 = 0x0091_0e20;
/// An empty function taking one argument `(object, value)` (tes.cpp).
pub(crate) const EMPTY_FUNCTION_1: u32 = 0x0045_34f0;
/// The callback `SetTargetData` searches references with (lowprocess.cpp).
pub(crate) const ACQUIRE_SEARCH_CALLBACK: u32 = 0x0090_dd80;
/// `TESPackage::GetPackageSearchLocation` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_SEARCH_LOCATION: u32 = 0x0067_2f20;
/// `TESPackage::GetSearchLocationRadius` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_GET_SEARCH_RADIUS: u32 = 0x0067_58a0;
/// `TESPackage::GetSearchLocationCell` (Xbox PDB), `(package, actor)`.
pub(crate) const PACKAGE_GET_SEARCH_CELL: u32 = 0x0067_57e0;
/// `TESPackage::GetSearchLocationCoord` (Xbox PDB), `(package, out,
/// actor)`.
pub(crate) const PACKAGE_GET_SEARCH_COORD: u32 = 0x0067_5830;
/// `FLT_MAX`, the search radius in cells with `CELL_FLAG`.
pub(crate) const FLOAT_MAX: u32 = 0x0101_6970;
/// The float game setting of the default acquire search radius.
pub(crate) const ACQUIRE_RADIUS_SETTING: u32 = 0x011c_debc;
/// `TESPackage::GetAcquireRadius` (Xbox PDB), `(package, actor, out)`.
pub(crate) const PACKAGE_GET_ACQUIRE_RADIUS: u32 = 0x0067_28a0;
/// `TESPackage::GetShouldReserveTarget` (Xbox PDB), `(package)`.
pub(crate) const PACKAGE_GET_SHOULD_RESERVE_TARGET: u32 = 0x0067_8d00;
/// `ProcessLists::GetActorRefInHigh` (Xbox PDB), `(lists, form, 0)`.
pub(crate) const GET_ACTOR_REF_IN_HIGH: u32 = 0x0097_0a20;
/// The `ProcessLists` object (its address).
pub(crate) const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `TESObjectREFR::GetFirstFreeMarkerIndex` (Xbox PDB), `(reference, 0)`.
pub(crate) const REFERENCE_GET_FIRST_FREE_MARKER: u32 = 0x0056_82c0;
/// Stores the value at `item` as a list node's item when it is not null,
/// `(node, item)`.
pub(crate) const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// Whether bit 0x100 / 0x400 / 0x200 of a package's `u16` at +0x24 is
/// set, `(package)` (highprocess.cpp).
pub(crate) const PACKAGE_FLAG2_100: u32 = 0x008e_30e0;
pub(crate) const PACKAGE_FLAG2_400: u32 = 0x008e_3120;
pub(crate) const PACKAGE_FLAG2_200: u32 = 0x008e_3100;
/// A reference's byte at +0x18D, `(reference)` (the engine map's folded
/// name `MiddleHighProcess::GetForceNextUpdate`).
pub(crate) const BYTE_18D: u32 = 0x0056_6950;
/// `TESValueForm::GetFormValue` (Xbox PDB), cdecl `(form)`.
pub(crate) const FORM_VALUE: u32 = 0x0048_e8a0;
/// `Actor::GetGoldAmount` (Xbox PDB), `(actor)`.
pub(crate) const ACTOR_GET_GOLD_AMOUNT: u32 = 0x0089_1d70;
/// `TESAIForm::SellBuysItem` (Xbox PDB), `(ai form, item)`.
pub(crate) const AI_FORM_SELL_BUYS_ITEM: u32 = 0x0047_f600;
/// `Actor::HasObjects` (Xbox PDB), `(actor, form, 0, 1, 0, out)`.
pub(crate) const ACTOR_HAS_OBJECTS: u32 = 0x0089_1db0;
/// The form pointer `EvaluateOrderAcquireList` asks `Actor::HasObjects`
/// about.
pub(crate) const LOCK_KEY_FORM: u32 = 0x011c_a268;
/// `TESObjectREFR::GetLock` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_GET_LOCK: u32 = 0x0056_9160;
/// The RTTI type descriptor of `MobileObject`.
pub(crate) const TYPE_MOBILE_OBJECT: u32 = 0x0118_4920;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB), `(reference)`.
pub(crate) const REFERENCE_GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// A process's level (`BaseProcess::eLevel`, +0x28), `(process)`.
pub(crate) const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// `MagicCaster::CastAbility` (Xbox PDB), `(caster, spell, flag)`.
pub(crate) const MAGIC_CASTER_CAST_ABILITY: u32 = 0x0081_5600;
/// `MagicCaster::TransferDisease` (Xbox PDB), `(caster, first, second,
/// flag)`.
pub(crate) const MAGIC_CASTER_TRANSFER_DISEASE: u32 = 0x0081_57a0;
/// 128.0 (a double): the door distance `ProcessActivateOneHour` accepts.
pub(crate) const ACTIVATE_DOOR_DISTANCE: u32 = 0x0102_e430;
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB), `(reference, object, 0)`.
pub(crate) const REFERENCE_GET_INVENTORY_ITEM: u32 = 0x0057_6260;

// Translated from 00460ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetFurnitureMarker` (Xbox PDB): the address of the
/// process's `FurnitureMarker` (+0x148).
pub fn middle_high_process_get_furniture_marker(
    _e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) -> u32 {
    this.addr().wrapping_add(0x148)
}

// Translated from 008dd8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessNotify` (Xbox PDB): with no `pTarget`
/// (+0x40), slot 0x8C with `target` (the Xbox `SetTargetForPackage`);
/// then slot 0x288 with `(target, 1)`.
pub fn middle_high_process_process_notify(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    target: u32,
) {
    if e.get(this, MiddleHighProcess::pTarget).is_null() {
        e.vcall(this.addr(), 0x8c, &args![target]);
    }
    e.vcall(this.addr(), 0x288, &args![target, 1u32]);
}

// Translated from 009139b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedItem`'s constructor (unnamed in the map): clears the object,
/// extra list, count and the six flags (+0x00 to +0x11) and constructs the
/// `spQueuedFiles` pointer (+0x14) with null. Returns `this`.
pub fn fn_009139b0(e: &mut Engine, this: Ptr<QueuedItem>) -> Ptr<QueuedItem> {
    e.set(this, QueuedItem::pObject, Ptr::new(0));
    e.set(this, QueuedItem::pExtra, Ptr::new(0));
    e.set(this, QueuedItem::iNumber, 0);
    e.set(this, QueuedItem::bEquip, 0);
    e.set(this, QueuedItem::bNow, 0);
    e.set(this, QueuedItem::bForceEquip, 0);
    e.set(this, QueuedItem::bPlayPickUpSounds, 0);
    e.set(this, QueuedItem::bForceUnequip, 0);
    e.set(this, QueuedItem::bEquipCalling, 0);
    e.call(QUEUED_FILE_POINTER_INIT, &args![this.addr() + 0x14, 0u32]);
    this
}

/// A new `QueuedFile` (0x28 bytes, constructed with 1; null when the
/// allocation fails) assigned to the item's `spQueuedFiles`.
fn queue_new_file(e: &mut Engine, item: u32) {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    let file = if block == 0 {
        0
    } else {
        e.call(QUEUED_FILE_CONSTRUCT, &args![block, 1u32]).u32()
    };
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![item + 0x14, file]);
}

/// Builds the KF file list of `actor`'s model for animation group `group`
/// and queues it on the item's queued file, then deletes the list.
fn queue_kf_files(e: &mut Engine, item: u32, actor: u32, group: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    let model = e.call(REFERENCE_GET_MODEL, &args![actor]).u32();
    let list = e
        .call(BUILD_KF_FILE_LIST, &args![loader, model, 1u32, 0u32, group])
        .u32();
    let file = e.call(FIRST_WORD, &args![item + 0x14]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(QUEUE_KF_FILES, &args![loader, list, 1u32, file, 0u32]);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

/// The animation of `actor`'s process (slot 0x1B8 of the word at +0x68)
/// when it is set and has data (`00496940`), else 0.
fn loaded_animation(e: &mut Engine, actor: u32) -> u32 {
    let process = e.call(WORD_68, &args![actor]).u32();
    let animation = e.vcall(process, 0x1b8, &[]).u32();
    if animation != 0 && e.call(ANIMATION_HAS_DATA, &args![animation]).u32() != 0 {
        animation
    } else {
        0
    }
}

// Translated from 00913a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Preloads the files of a queued item for `actor` (unnamed in the map).
/// False without an actor or object. To equip (`bEquip`), an object of
/// form type 0x28 or 0x18 gets a new queued file with its model files
/// (`00444d40`); then, when `actor`'s slot 0x21C answers true and the
/// object's type is 0x28, unless its animation already has the group
/// (`0118a838[00446390(object)]`, kind 0x18) loaded, that group's KF files
/// of the actor's model are queued too; returns true (other types:
/// false). To unequip, when slot 0x21C answers true and the type is 0x28,
/// unless the animation has group 1 (kind 0x18) loaded, a new queued file
/// gets the KF files of group 1; returns false.
// Translation incomplete: 00913a20 - the C++ exception-unwinding frame (FS:[0]) that frees the new queued file when its constructor throws is not translated.
pub fn fn_00913a20(e: &mut Engine, this: Ptr<QueuedItem>, actor: u32) -> bool {
    let item = this.addr();
    if actor == 0 || word(e, item, 0) == 0 {
        return false;
    }
    if byte(e, item, 0xc) != 0 {
        let object = word(e, item, 0);
        let kind = e.call(FORM_TYPE, &args![object]).u32();
        if kind != 0x28 {
            let object = word(e, item, 0);
            if e.call(FORM_TYPE, &args![object]).u32() != 0x18 {
                return false;
            }
        }
        queue_new_file(e, item);
        let object = word(e, item, 0);
        let file = e.call(FIRST_WORD, &args![item + 0x14]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            QUEUE_OBJECT_FILES,
            &args![loader, object, 1u32, file, actor],
        );
        if !e.vcall(actor, 0x21c, &[]).bool() {
            return true;
        }
        let object = word(e, item, 0);
        if e.call(FORM_TYPE, &args![object]).u32() != 0x28 {
            return true;
        }
        let animation = loaded_animation(e, actor);
        if animation != 0 {
            let index = e.call(OBJECT_GROUP_INDEX, &args![object]).u32();
            let kind = e
                .mem
                .u32(OBJECT_GROUP_TABLE.wrapping_add(index.wrapping_mul(4)));
            let group = e.call(ANIM_GROUP, &args![0u32, kind, 0x18u32, 0u32]).u16();
            if e.call(ANIMATION_GROUP_LOADED, &args![animation, u32::from(group)])
                .bool()
            {
                return true;
            }
        }
        let index = e.call(OBJECT_GROUP_INDEX, &args![object]).u32();
        let kind = e
            .mem
            .u32(OBJECT_GROUP_TABLE.wrapping_add(index.wrapping_mul(4)));
        queue_kf_files(e, item, actor, kind);
        return true;
    }
    if !e.vcall(actor, 0x21c, &[]).bool() {
        return false;
    }
    let object = word(e, item, 0);
    if e.call(FORM_TYPE, &args![object]).u32() != 0x28 {
        return false;
    }
    let animation = loaded_animation(e, actor);
    if animation != 0 {
        let group = e.call(ANIM_GROUP, &args![0u32, 1u32, 0x18u32, 0u32]).u16();
        if e.call(ANIMATION_GROUP_LOADED, &args![animation, u32::from(group)])
            .bool()
        {
            return false;
        }
    }
    queue_new_file(e, item);
    queue_kf_files(e, item, actor, 1);
    false
}

// Translated from 00913d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedItem::SaveGame` (Xbox PDB): writes the object's form id, the
/// save index of its extra list (-1 without one, or when the buffer's
/// reference, slot 0x08, has no container changes), the count and the six
/// flag bytes.
pub fn queued_item_save_game(e: &mut Engine, this: Ptr<QueuedItem>, buffer: u32) {
    let item = this.addr();
    let reference = e.vcall(buffer, 0x8, &[]).u32();
    let object = word(e, item, 0);
    e.call(SAVE_FORM_ID, &args![buffer, object, 0u32]);
    e.with_stack(4, |e, index| {
        e.mem.set_u32(index.addr(), u32::MAX);
        if word(e, item, 4) != 0 {
            let changes = if reference == 0 {
                0
            } else {
                let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                e.call(GET_CONTAINER_CHANGES, &args![list]).u32()
            };
            if changes != 0 {
                let extra = word(e, item, 4);
                let object = word(e, item, 0);
                let saved = e
                    .call(EXTRA_LIST_SAVE_INDEX, &args![changes, object, extra])
                    .u32();
                e.mem.set_u32(index.addr(), saved);
            }
        }
        e.call(SAVE_DATA, &args![buffer, index, 4u32, 0u32]);
    });
    e.call(SAVE_DATA, &args![buffer, item + 0x8, 4u32, 0u32]);
    for offset in 0xcu32..=0x11 {
        e.call(SAVE_DATA, &args![buffer, item + offset, 1u32, 0u32]);
    }
}

// Translated from 00913e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedItem::LoadGame` (Xbox PDB): reads the object (a form id, looked
/// up and cast to `TESBoundObject`), the extra list's save index (resolved
/// through the container changes of the buffer's reference, slot 0x0C,
/// when it is not -1), the count and the six flag bytes.
pub fn queued_item_load_game(e: &mut Engine, this: Ptr<QueuedItem>, buffer: u32) {
    let item = this.addr();
    let reference = e.vcall(buffer, 0xc, &[]).u32();
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let object = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0u32],
        )
        .u32();
    e.mem.set_u32(item, object);
    e.with_stack(4, |e, index| {
        e.mem.set_u32(index.addr(), u32::MAX);
        e.call(LOAD_DATA, &args![buffer, index, 4u32]);
        let saved = e.mem.u32(index.addr());
        if saved != u32::MAX {
            let changes = if reference == 0 {
                0
            } else {
                let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                e.call(GET_CONTAINER_CHANGES, &args![list]).u32()
            };
            if changes != 0 {
                let object = word(e, item, 0);
                let extra = e
                    .call(EXTRA_LIST_FROM_INDEX, &args![changes, object, saved])
                    .u32();
                e.mem.set_u32(item + 4, extra);
            }
        }
    });
    e.call(LOAD_DATA, &args![buffer, item + 0x8, 4u32]);
    for offset in 0xcu32..=0x11 {
        e.call(LOAD_DATA, &args![buffer, item + offset, 1u32]);
    }
}

// Translated from 00913fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Preloads a queued item ([`fn_00913a20`]) for the reference `source`'s
/// slot 0x0C gives (unnamed in the map).
pub fn fn_00913fb0(e: &mut Engine, this: Ptr<QueuedItem>, source: u32) {
    let actor = e.vcall(source, 0xc, &[]).u32();
    fn_00913a20(e, this, actor);
}

// Translated from 009145c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetRadiationDelta` (Xbox PDB): `fRadiationDelta`
/// (+0x234).
pub fn middle_high_process_get_radiation_delta(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) -> f32 {
    x87_quiet(e.get(this, MiddleHighProcess::fRadiationDelta))
}

// Translated from 009145e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`00914610`), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn middle_high_process_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    flags: u32,
) -> Ptr<MiddleHighProcess> {
    middle_high_process_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00914ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::DeferredHideLimb`'s scalar deleting destructor
/// (unnamed in the map; `MiddleHighProcess`'s destructor runs it on the
/// entries of `DeferredHideLimbList`, +0x1B0): [`fn_00914ae0`],
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00914ab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00914ae0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00914ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::DeferredHideLimb`'s destructor (unnamed in the map;
/// Ghidra's library match `_ATL_WIN_MODULE70` is a folded body): releases
/// `spReplacementLimb` (+0x0C) and then `spDismemberedLimbRoot` (+0x08)
/// (Xbox PDB names) through `0045cec0`.
// Translation incomplete: 00914ae0 - the C++ exception-unwinding frame (FS:[0]) that still releases +0x08 when the first release throws is not translated.
pub fn fn_00914ae0(e: &mut Engine, this: Ptr) {
    e.call(POINTER_RELEASE, &args![this.addr() + 0xc]);
    e.call(POINTER_RELEASE, &args![this.addr() + 0x8]);
}

// Translated from 00914b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearReservation` (Xbox PDB): unless the data
/// handler's byte at +0x61D is set, frees the reserved marker slot
/// (`iReservationSlot`, +0x1A8, when not negative) of the target (+0x40)
/// and sets the slot to -1, and, unless bit 1 of the save/load game's
/// +0x244 is set, frees the current furniture's (+0x140) marker
/// `cCurrentFurnitureIndex` (+0x144).
pub fn middle_high_process_clear_reservation(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let process = this.addr();
    let handler = e.global::<u32>(DATA_HANDLER);
    if e.call(DATA_HANDLER_BYTE_61D, &args![handler]).bool() {
        return;
    }
    if word(e, process, 0x40) != 0 && (word(e, process, 0x1a8) as i32) >= 0 {
        let target = word(e, process, 0x40);
        let slot = word(e, process, 0x1a8);
        e.call(SET_MARKER_RESERVED, &args![target, slot, 0u32]);
        e.mem.set_u32(process + 0x1a8, u32::MAX);
    }
    if word(e, process, 0x140) != 0 {
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        if !e.call(SAVE_LOAD_FLAG_2, &args![save_load]).bool() {
            let furniture = word(e, process, 0x140);
            let index = u32::from(byte(e, process, 0x144));
            e.call(SET_MARKER_RESERVED, &args![furniture, index, 0u32]);
        }
    }
}

// Translated from 00914be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::HandleDeletedReference` (Xbox PDB): the current
/// package (+0x04) and its package form (slot 0x04) drop `reference`, then
/// the run-once package (+0xE4) and the package slot 0x270 returns (the
/// Xbox `GetRunOncePackageInstanceData`).
pub fn middle_high_process_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    reference: u32,
) {
    let process = this.addr();
    e.call(PACKAGE_HANDLE_DELETED, &args![process + 0x4, reference]);
    let package = e.call(PROCESS_PACKAGE, &args![process]).u32();
    if package != 0 {
        e.vcall(package, 0x4, &args![reference]);
    }
    e.call(PACKAGE_HANDLE_DELETED, &args![process + 0xe4, reference]);
    let data = e.vcall(process, 0x270, &[]).u32();
    if data != 0 {
        e.vcall(data, 0x4, &args![reference]);
    }
}

// Translated from 00913fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess`'s constructor (unnamed in the map; vtable
/// `0108904c`): the `MiddleLowProcess` part (`0092c950`), the member
/// constructors (lists +0xC8, +0xD0, +0x1B0, the run-once package +0xE4,
/// the pointers +0x11C, +0x120, +0x138, +0x220, the furniture mark +0x148
/// with 0xFF), a new `CachedValues` (0x48 bytes) at +0x2C, a new list (8
/// bytes) as `pActiveEffectList` (+0x1B8), and every field's start value
/// (`fAlphaMult` 1.0, `fPackageIdleTimer` -1.0 from `01012054`,
/// `iPackageIdleNumber`, `iReservationSlot` and `iWeaponConditionStage`
/// -1, `cCurrentFurnitureIndex` 0x7F, `cShotsToFire` 1,
/// `iAnimActionSuccess` 2, the rest zero). Returns `this`.
// Translation incomplete: 00913fe0 - the C++ exception-unwinding frame (FS:[0]) that destroys the constructed members when a constructor throws is not translated.
pub fn fn_00913fe0(e: &mut Engine, this: Ptr<MiddleHighProcess>) -> Ptr<MiddleHighProcess> {
    let p = this.addr();
    e.call(MIDDLE_LOW_CONSTRUCT, &args![p]);
    e.mem.set_u32(p, MIDDLE_HIGH_PROCESS_VTABLE);
    e.call(LIST_CONSTRUCT, &args![p + 0xc8]);
    e.call(LIST_CONSTRUCT, &args![p + 0xd0]);
    e.call(ACTOR_PACKAGE_CONSTRUCT, &args![p + 0xe4]);
    e.call(IDENTITY, &args![p + 0xfc]);
    e.call(QUEUED_FILE_POINTER_INIT, &args![p + 0x11c, 0u32]);
    e.call(QUEUED_FILE_POINTER_INIT, &args![p + 0x120, 0u32]);
    e.call(POINTER_INIT, &args![p + 0x138, 0u32]);
    e.call(FURNITURE_MARK_CONSTRUCT, &args![p + 0x148, 0xffu32]);
    e.call(LIST_CONSTRUCT, &args![p + 0x1b0]);
    e.call(POINTER_INIT, &args![p + 0x220, 0u32]);
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    let values = if block == 0 {
        0
    } else {
        e.call(CACHED_VALUES_CONSTRUCT, &args![block]).u32()
    };
    e.mem.set_u32(p + 0x2c, values);
    e.call(SET_CURRENT_SPELL, &args![p, 0u32]);
    e.call(SET_DESIRED_TARGET, &args![p, 0u32]);
    e.mem.set_u32(p + 0x114, 0);
    e.mem.set_u32(p + 0x118, 0);
    e.mem.set_u32(p + 0x130, 0);
    e.mem.set_u32(p + 0x12c, 0);
    for offset in [0x127u32, 0x128, 0x129, 0x12a, 0x124, 0x125, 0x126] {
        e.mem.set_u8(p + offset, 0);
    }
    e.mem.set_u32(p + 0x158, 0);
    e.mem.set_u32(p + 0x15c, 0);
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    let list = if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    };
    e.mem.set_u32(p + 0x1b8, list);
    e.call(ACTOR_PACKAGE_CLEAR, &args![p + 0xe4]);
    e.mem.set_u32(p + 0x1c0, 0);
    e.call(MEMSET, &args![p + 0x1c4, 0u32, 0xcu32]);
    e.call(MEMSET, &args![p + 0x1dc, 0u32, 0x3cu32]);
    e.mem.set_u32(p + 0x218, 0);
    e.mem.set_u32(p + 0x21c, 0);
    e.mem.set_f32(p + 0x1d0, 0.0);
    e.mem.set_f32(p + 0x1d4, 0.0);
    e.mem.set_u8(p + 0x1d8, 1);
    e.mem.set_u8(p + 0x134, 0);
    e.mem.set_u8(p + 0x135, 0);
    e.mem.set_u8(p + 0x13c, 0);
    e.mem.set_u32(p + 0x58, 0);
    e.mem.set_u8(p + 0x13d, 0);
    e.mem.set_u32(p + 0x140, 0);
    e.mem.set_u8(p + 0x144, 0x7f);
    e.call(
        FURNITURE_MARK_SET_HEADING,
        &args![p + 0x148, 0.0f32.to_bits()],
    );
    e.call(SET_BYTE_E, &args![p + 0x148, 0u32]);
    e.call(COPY_POINT, &args![p + 0x148, FURNITURE_MARK_ORIGIN]);
    e.mem.set_f32(p + 0xd8, 0.0);
    e.mem.set_u8(p + 0x1da, 0);
    e.call(POINTER_ASSIGN, &args![p + 0x220, 0u32]);
    e.mem.set_u32(p + 0x108, 0);
    e.mem.set_f32(p + 0xdc, 0.0);
    e.mem.set_u32(p + 0x164, 0);
    e.mem.set_u8(p + 0x168, 0);
    e.mem.set_u32(p + 0x16c, 0);
    e.mem.set_f32(p + 0x170, 1.0);
    e.mem.set_f32(p + 0x174, 0.0);
    e.mem.set_u32(p + 0x178, 0);
    e.mem.set_u32(p + 0x184, 0);
    e.mem.set_u8(p + 0x180, 0);
    e.mem.set_u8(p + 0xe0, 0);
    e.mem.set_f32(p + 0xa4, 0.0);
    for offset in [0x188u32, 0x189, 0x17c, 0x17d, 0x17e, 0x17f] {
        e.mem.set_u8(p + offset, 0);
    }
    e.mem.set_u32(p + 0x224, 0);
    for offset in [0x18au32, 0x18b, 0x18c, 0x18d] {
        e.mem.set_u8(p + offset, 0);
    }
    e.mem.set_u32(p + 0x190, u32::MAX);
    e.mem.set_u32(p + 0x194, 0);
    let idle_timer = x87_quiet(e.global::<f32>(MINUS_ONE));
    e.mem.set_f32(p + 0x198, idle_timer);
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![p + 0x11c, 0u32]);
    e.call(CLONE_RESERVER_ASSIGN, &args![p + 0x120, 0u32]);
    e.mem.set_u8(p + 0x228, 0);
    e.mem.set_u32(p + 0x230, 0);
    e.mem.set_u32(p + 0x1ac, 0);
    e.mem.set_u16(p + 0x22a, 2);
    e.mem.set_u8(p + 0xe1, 0);
    e.mem.set_u8(p + 0x19c, 0);
    e.mem.set_u8(p + 0x19d, 0);
    e.mem.set_u32(p + 0x1a8, u32::MAX);
    e.mem.set_u32(p + 0x10c, 0);
    e.mem.set_u8(p + 0xe2, 0);
    e.mem.set_u8(p + 0xe3, 0);
    e.mem.set_u32(p + 0x1a0, 0);
    e.mem.set_u32(p + 0x22c, 0);
    e.mem.set_f32(p + 0x238, 0.0);
    e.mem.set_f32(p + 0x23c, 0.0);
    e.mem.set_f32(p + 0x234, 0.0);
    e.mem.set_u32(p + 0x1a4, 0);
    e.mem.set_u32(p + 0x240, 0);
    e.mem.set_u32(p + 0x254, 0);
    e.mem.set_u8(p + 0x110, 0);
    e.mem.set_u32(p + 0x248, 0);
    e.mem.set_u32(p + 0x24c, 0);
    e.mem.set_u32(p + 0x250, 0);
    e.mem.set_u32(p + 0x244, u32::MAX);
    e.call(SET_WORD_28, &args![p, 1u32]);
    e.mem.set_u32(p + 0x258, 0);
    this
}

/// Runs `visit` on the item of each node from `node` on, until a node
/// whose item is null (`check_empty` false) or that is empty
/// (`LIST_IS_EMPTY`, `check_empty` true), following `WORD_4`.
fn for_each_item(
    e: &mut Engine,
    mut node: u32,
    check_empty: bool,
    mut visit: impl FnMut(&mut Engine, u32),
) {
    while node != 0 {
        if check_empty {
            if e.call(LIST_IS_EMPTY, &args![node]).bool() {
                break;
            }
        } else {
            let slot = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            visit(e, item);
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
}

// Translated from 00914610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess`'s destructor (unnamed in the map): frees the cached
/// values (+0x2C); deletes every active effect (slot 0x00 with 1) and the
/// list (+0x1B8); deletes the weapon and ammo item changes (+0x114,
/// +0x118); clears the character controller (+0x138); the run-once
/// package's package goes to `TESSaveLoadGame::DeleteForm` when `0047c850`
/// answers true, else `ClearRunOncePackage` runs; empties the lists +0xC8
/// (unless empty) and +0xD0; clears the animation (`SetAnimation` with
/// null) and +0x220; releases each attached arrow (`008d0240`) and
/// deletes their list (+0x1AC); clears +0x11C, +0x120 and the furniture
/// (+0x140); clears the reservations ([`middle_high_process_clear_reservation`]);
/// deletes every deferred hidden limb (+0x1B0); deletes the lip-synch
/// animation (+0x1A0) and frees +0x240; clears the face nodes (+0x248 to
/// +0x250); removes all queued items; then destroys the members and the
/// `MiddleLowProcess` part.
// Translation incomplete: 00914610 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated.
pub fn middle_high_process_destructor(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    e.mem.set_u32(p, MIDDLE_HIGH_PROCESS_VTABLE);
    if word(e, p, 0x2c) != 0 {
        let values = word(e, p, 0x2c);
        e.call(FREE, &args![values]);
        e.mem.set_u32(p + 0x2c, 0);
    }
    if word(e, p, 0x1b8) != 0 {
        let node = word(e, p, 0x1b8);
        for_each_item(e, node, false, |e, item| {
            e.vcall(item, 0, &args![1u32]);
        });
        let list = word(e, p, 0x1b8);
        e.call(LIST_CLEAR, &args![list]);
        let list = word(e, p, 0x1b8);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
    }
    let weapon = word(e, p, 0x114);
    if weapon != 0 {
        e.call(ITEM_CHANGE_DELETE, &args![weapon, 1u32]);
    }
    let ammo = word(e, p, 0x118);
    if ammo != 0 {
        e.call(ITEM_CHANGE_DELETE, &args![ammo, 1u32]);
    }
    e.mem.set_u32(p + 0x130, 0);
    e.mem.set_u32(p + 0x114, 0);
    e.mem.set_u32(p + 0x118, 0);
    e.call(POINTER_ASSIGN, &args![p + 0x138, 0u32]);
    if e.call(RUN_ONCE_PACKAGE, &args![p]).u32() != 0 {
        let save_load = e.global::<u32>(TES_SAVE_LOAD_GAME);
        if e.call(ALWAYS_FALSE, &args![save_load]).bool() {
            let package = e.call(RUN_ONCE_PACKAGE, &args![p]).u32();
            let save_load = e.global::<u32>(TES_SAVE_LOAD_GAME);
            e.call(DELETE_FORM, &args![save_load, package]);
        } else {
            e.call(CLEAR_RUN_ONCE_PACKAGE, &args![p]);
        }
    }
    if !e.call(LIST_IS_EMPTY, &args![p + 0xc8]).bool() {
        e.call(LIST_CLEAR, &args![p + 0xc8]);
    }
    e.call(SET_ANIMATION, &args![p, 0u32]);
    e.call(POINTER_ASSIGN, &args![p + 0x220, 0u32]);
    e.call(LIST_CLEAR, &args![p + 0xd0]);
    if word(e, p, 0x1ac) != 0 {
        let node = word(e, p, 0x1ac);
        for_each_item(e, node, true, |e, arrow| {
            e.call(ARROW_RELEASE, &args![arrow]);
        });
        let list = word(e, p, 0x1ac);
        e.call(LIST_CLEAR, &args![list]);
        let list = word(e, p, 0x1ac);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.mem.set_u32(p + 0x1ac, 0);
    }
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![p + 0x11c, 0u32]);
    e.call(CLONE_RESERVER_ASSIGN, &args![p + 0x120, 0u32]);
    e.mem.set_u32(p + 0x140, 0);
    middle_high_process_clear_reservation(e, this);
    while !e.call(LIST_IS_EMPTY, &args![p + 0x1b0]).bool() {
        let slot = e.call(IDENTITY, &args![p + 0x1b0]).u32();
        let limb = e.mem.u32(slot);
        if limb != 0 {
            fn_00914ab0(e, Ptr::new(limb), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![p + 0x1b0]);
    }
    let lip_synch = word(e, p, 0x1a0);
    if lip_synch != 0 {
        e.call(LIP_SYNCH_ANIM_DELETE, &args![lip_synch, 1u32]);
    }
    if word(e, p, 0x240) != 0 {
        let hit_data = word(e, p, 0x240);
        e.call(FREE, &args![hit_data]);
    }
    e.mem.set_u32(p + 0x248, 0);
    e.mem.set_u32(p + 0x24c, 0);
    e.mem.set_u32(p + 0x250, 0);
    middle_high_process_remove_all_queued_items(e, this);
    e.call(POINTER_RELEASE, &args![p + 0x220]);
    e.call(LIST_DESTRUCT, &args![p + 0x1b0]);
    e.call(POINTER_RELEASE, &args![p + 0x138]);
    e.call(CLONE_RESERVER_RELEASE, &args![p + 0x120]);
    e.call(QUEUED_FILE_POINTER_RELEASE, &args![p + 0x11c]);
    e.call(ACTOR_PACKAGE_DESTRUCT, &args![p + 0xe4]);
    e.call(LIST_DESTRUCT, &args![p + 0xd0]);
    e.call(LIST_DESTRUCT, &args![p + 0xc8]);
    e.call(MIDDLE_LOW_DESTRUCT, &args![p]);
}

/// An x87 `FLD`/`FSTP` copy of the float at `offset` from `source` to
/// `destination`.
fn copy_float(e: &mut Engine, destination: u32, source: u32, offset: u32) {
    let bits = x87_quiet_bits(word(e, source, offset));
    e.mem.set_u32(destination + offset, bits);
}

fn copy_word(e: &mut Engine, destination: u32, source: u32, offset: u32) {
    let value = word(e, source, offset);
    e.mem.set_u32(destination + offset, value);
}

fn copy_byte(e: &mut Engine, destination: u32, source: u32, offset: u32) {
    let value = byte(e, source, offset);
    e.mem.set_u8(destination + offset, value);
}

// Translated from 00914c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::MiddleHighCopy` (Xbox PDB): takes over the
/// middle-high state of `source` (a process of another level): the
/// run-once package (`ActorPackage::MoveTo`), the last-seen position, the
/// animation (+0x1C0, moved; its group 1 cleared with 0.0), the weapon and
/// ammo item changes and the armor/weapon flag bytes (moved), the
/// want-drawn and drawn bytes from `source`'s slots 0x44C and 0x454, the
/// dead-detect list items (+0xC8, appended), copies of the queued items
/// (+0x230, a new list when this has none), slot 0x4F0's byte as +0x1DA,
/// the furniture (+0x140, marked targeted) and its marker, the damage
/// nodes, the character controller (slot 0x4F8), the active effects
/// (slot 0x3B8's list, each item's slot 0x04 appended after emptying
/// +0x1B8), the arrows (+0x1AC, moved), the preload and clone-reserver
/// pointers and the remaining scalar fields.
// Translation incomplete: 00914c60 - the C++ exception-unwinding frame (FS:[0]) that frees a new list or queued item when its constructor throws is not translated.
pub fn middle_high_process_middle_high_copy(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    source: u32,
) {
    let p = this.addr();
    e.call(ACTOR_PACKAGE_MOVE_TO, &args![source + 0xe4, p + 0xe4]);
    for offset in [0xfcu32, 0x100, 0x104] {
        copy_float(e, p, source, offset);
    }
    copy_word(e, p, source, 0x1c0);
    e.mem.set_u32(source + 0x1c0, 0);
    if word(e, p, 0x1c0) != 0 {
        let animation = word(e, p, 0x1c0);
        e.call(
            ANIMATION_CLEAR_GROUP,
            &args![animation, 1u32, 0.0f32.to_bits()],
        );
    }
    copy_word(e, p, source, 0x114);
    copy_word(e, p, source, 0x118);
    for offset in 0x124u32..=0x12a {
        copy_byte(e, p, source, offset);
    }
    let want_drawn = e.vcall(source, 0x44c, &[]).u8();
    e.mem.set_u8(p + 0x134, want_drawn);
    let drawn = e.vcall(source, 0x454, &[]).u8();
    e.mem.set_u8(p + 0x135, drawn);
    copy_byte(e, p, source, 0x13c);
    copy_word(e, p, source, 0x164);
    copy_byte(e, p, source, 0xe2);
    copy_word(e, p, source, 0x130);
    copy_word(e, p, source, 0x12c);
    copy_byte(e, p, source, 0x188);
    copy_byte(e, p, source, 0x189);
    e.mem.set_u32(source + 0x130, 0);
    e.mem.set_u32(source + 0x114, 0);
    e.mem.set_u32(source + 0x118, 0);
    e.mem.set_u8(source + 0x124, 0);
    e.mem.set_u8(source + 0x126, 0);
    e.mem.set_u8(source + 0x125, 0);
    copy_word(e, p, source, 0x108);
    copy_float(e, p, source, 0xdc);
    copy_word(e, p, source, 0x10c);
    let mut node = source + 0xc8;
    while node != 0 {
        let slot = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        e.call(LIST_ADD, &args![p + 0xc8, slot]);
        node = e.call(WORD_4, &args![node]).u32();
    }
    let mut node = word(e, source, 0x230);
    while node != 0 {
        let slot = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        if word(e, p, 0x230) == 0 {
            let block = e.call(ALLOCATE, &args![8u32]).u32();
            let list = if block == 0 {
                0
            } else {
                e.call(LIST_CONSTRUCT, &args![block]).u32()
            };
            e.mem.set_u32(p + 0x230, list);
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        let from = e.mem.u32(slot);
        let block = e.call(ALLOCATE, &args![0x18u32]).u32();
        let item = if block == 0 {
            0
        } else {
            fn_009139b0(e, Ptr::new(block)).addr()
        };
        copy_word(e, item, from, 0);
        copy_byte(e, item, from, 0xc);
        copy_word(e, item, from, 8);
        copy_word(e, item, from, 4);
        for offset in [0xdu32, 0xe, 0x10, 0x11, 0xf] {
            copy_byte(e, item, from, offset);
        }
        e.call(QUEUED_FILE_POINTER_COPY, &args![item + 0x14, from + 0x14]);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            let list = word(e, p, 0x230);
            e.call(LIST_ADD, &args![list, local]);
        });
        node = e.call(WORD_4, &args![node]).u32();
    }
    let flag = e.vcall(source, 0x4f0, &[]).u8();
    e.mem.set_u8(p + 0x1da, flag);
    copy_byte(e, p, source, 0x13d);
    copy_word(e, p, source, 0x140);
    if word(e, p, 0x140) != 0 {
        let furniture = word(e, p, 0x140);
        e.call(SET_TARGETED, &args![furniture, 1u32]);
    }
    copy_byte(e, p, source, 0x144);
    e.call(MEMCPY, &args![p + 0x148, source + 0x148, 0x10u32]);
    e.call(MEMCPY, &args![p + 0x1dc, source + 0x1dc, 0x3cu32]);
    copy_word(e, p, source, 0x218);
    copy_word(e, p, source, 0x21c);
    let controller = e.vcall(source, 0x4f8, &[]).u32();
    e.call(POINTER_ASSIGN, &args![p + 0x220, controller]);
    let effects = word(e, p, 0x1b8);
    e.call(LIST_CLEAR, &args![effects]);
    let mut node = e.vcall(source, 0x3b8, &[]).u32();
    while node != 0 {
        if e.call(LIST_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        let effect = e.mem.u32(slot);
        let copy = e.vcall(effect, 0x4, &[]).u32();
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), copy);
            let list = word(e, p, 0x1b8);
            e.call(LIST_ADD, &args![list, local]);
        });
        node = e.call(WORD_4, &args![node]).u32();
    }
    copy_word(e, p, source, 0x158);
    copy_word(e, p, source, 0x15c);
    e.call(CONTROLLER_POINTER_COPY, &args![p + 0x138, source + 0x138]);
    copy_byte(e, p, source, 0x168);
    copy_word(e, p, source, 0x16c);
    copy_float(e, p, source, 0x170);
    copy_float(e, p, source, 0x174);
    copy_word(e, p, source, 0x224);
    copy_byte(e, p, source, 0x18a);
    copy_byte(e, p, source, 0x18c);
    copy_byte(e, p, source, 0x18d);
    copy_word(e, p, source, 0x190);
    copy_word(e, p, source, 0x194);
    copy_float(e, p, source, 0x198);
    copy_word(e, p, source, 0x1ac);
    e.mem.set_u32(source + 0x1ac, 0);
    e.call(QUEUED_FILE_POINTER_COPY, &args![p + 0x11c, source + 0x11c]);
    e.call(CLONE_RESERVER_COPY, &args![p + 0x120, source + 0x120]);
    copy_byte(e, p, source, 0x228);
    let success = e.mem.u16(source + 0x22a);
    e.mem.set_u16(p + 0x22a, success);
    copy_float(e, p, source, 0x1d0);
    copy_word(e, p, source, 0x22c);
    copy_float(e, p, source, 0x238);
    copy_float(e, p, source, 0x23c);
    copy_float(e, p, source, 0x234);
    copy_word(e, p, source, 0x248);
    copy_word(e, p, source, 0x24c);
    copy_word(e, p, source, 0x250);
}

// Translated from 00915400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetupNewPackage` (Xbox PDB): clears the item
/// being used (+0x34), sets `iAnimActionSuccess` (+0x22A) to 2, calls
/// slots 0x4E4, 0x118 and 0x4EC with 0, clears process flag 0x40
/// (`0090ac20`), calls slots 0x5A8 and 0x12C with 0 and slot 0x330, and
/// clears the reservations.
pub fn middle_high_process_setup_new_package(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    e.call(EMPTY_FUNCTION, &args![p]);
    e.mem.set_u32(p + 0x34, 0);
    e.mem.set_u16(p + 0x22a, 2);
    e.vcall(p, 0x4e4, &args![0u32]);
    e.vcall(p, 0x118, &args![0u32]);
    e.vcall(p, 0x4ec, &args![0u32]);
    e.call(SET_PROCESS_FLAG_40, &args![p, 0u32]);
    e.vcall(p, 0x5a8, &args![0u32]);
    e.vcall(p, 0x12c, &args![0u32]);
    e.vcall(p, 0x330, &[]);
    middle_high_process_clear_reservation(e, this);
}

/// Whether the item's queued file (+0x14) blocks it: set and not done.
fn queued_file_pending(e: &mut Engine, item: u32) -> bool {
    if e.call(FIRST_WORD, &args![item + 0x14]).u32() == 0 {
        return false;
    }
    let file = e.call(FIRST_WORD, &args![item + 0x14]).u32();
    !e.call(QUEUED_FILE_DONE, &args![file]).bool()
}

/// The item of a list node (`*IDENTITY(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 009154b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ResolveEquipmentQueue` (Xbox PDB): runs every
/// queued item (+0x230) whose queued file is absent or done, and removes
/// it. To equip: an actor in anim action 7 without a weapon stops
/// blocking, `Actor::EquipObject` runs, and a weapon (type 0x28) whose
/// group index (`00446390`) is 10 to 13 is equipped again as type 0x29
/// (without sounds) and set back to 0x28. To unequip:
/// `Actor::UnEquipObject`, then, when the animation (slot 0x1B8) lacks
/// group 1 (kind 0x18), the KF files of group 1 of the actor's model are
/// added to it. Items still loading stay. An emptied list is deleted.
pub fn middle_high_process_resolve_equipment_queue(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    if word(e, p, 0x230) == 0 {
        return;
    }
    let mut node = word(e, p, 0x230);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        let mut done = false;
        if item != 0 && !queued_file_pending(e, item) {
            done = true;
            if byte(e, item, 0xc) != 0 {
                if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).u32() == 7
                    && e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32() == 0
                {
                    e.call(ACTOR_SET_BLOCK, &args![actor, 0u32]);
                }
                let (object, count, extra) = (word(e, item, 0), word(e, item, 8), word(e, item, 4));
                let (now, force, sounds) =
                    (byte(e, item, 0xd), byte(e, item, 0xe), byte(e, item, 0xf));
                e.call(
                    ACTOR_EQUIP_OBJECT,
                    &args![
                        actor,
                        object,
                        count,
                        extra,
                        u32::from(now),
                        u32::from(force),
                        u32::from(sounds)
                    ],
                );
                let object = word(e, item, 0);
                if e.call(FORM_TYPE, &args![object]).u32() == 0x28 {
                    let weapon = word(e, item, 0);
                    let mut grouped = false;
                    for index in [10u32, 11, 12, 13] {
                        if e.call(OBJECT_GROUP_INDEX, &args![weapon]).u32() == index {
                            grouped = true;
                            break;
                        }
                    }
                    if grouped {
                        let object = word(e, item, 0);
                        e.call(SET_FORM_TYPE, &args![object, 0x29u32]);
                        let (object, count, extra) =
                            (word(e, item, 0), word(e, item, 8), word(e, item, 4));
                        let (now, force) = (byte(e, item, 0xd), byte(e, item, 0xe));
                        e.call(
                            ACTOR_EQUIP_OBJECT,
                            &args![
                                actor,
                                object,
                                count,
                                extra,
                                u32::from(now),
                                u32::from(force),
                                0u32
                            ],
                        );
                        let object = word(e, item, 0);
                        e.call(SET_FORM_TYPE, &args![object, 0x28u32]);
                    }
                }
            } else {
                let (object, count, extra) = (word(e, item, 0), word(e, item, 8), word(e, item, 4));
                let (force, calling, sounds) =
                    (byte(e, item, 0x10), byte(e, item, 0x11), byte(e, item, 0xf));
                e.call(
                    ACTOR_UNEQUIP_OBJECT,
                    &args![
                        actor,
                        object,
                        count,
                        extra,
                        u32::from(force),
                        u32::from(calling),
                        u32::from(sounds)
                    ],
                );
                let animation = e.vcall(p, 0x1b8, &[]).u32();
                if animation != 0 {
                    let group = e.call(ANIM_GROUP, &args![0u32, 1u32, 0x18u32, 0u32]).u16();
                    if !e
                        .call(ANIMATION_GROUP_LOADED, &args![animation, u32::from(group)])
                        .bool()
                    {
                        let model = e.call(REFERENCE_GET_MODEL, &args![actor]).u32();
                        let loader = e.global::<u32>(MODEL_LOADER);
                        let list = e
                            .call(BUILD_KF_FILE_LIST, &args![loader, model, 1u32, 0u32, 1u32])
                            .u32();
                        e.call(ANIMATION_ADD_FROM_LIST, &args![animation, list]);
                    }
                }
            }
        }
        if done {
            if item != 0 {
                fn_00915790(e, Ptr::new(item), 1);
            }
            e.call(LIST_REMOVE_HEAD, &args![node]);
        } else {
            node = e.call(WORD_4, &args![node]).u32();
        }
    }
    let list = word(e, p, 0x230);
    if node_item(e, list) == 0 {
        let list = word(e, p, 0x230);
        e.call(LIST_CLEAR, &args![list]);
        let list = word(e, p, 0x230);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.mem.set_u32(p + 0x230, 0);
    }
}

// Translated from 00915790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedItem`'s scalar deleting destructor (unnamed in the map): the
/// destructor ([`fn_009157c0`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00915790(e: &mut Engine, this: Ptr<QueuedItem>, flags: u32) -> Ptr<QueuedItem> {
    fn_009157c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009157c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedItem`'s destructor (unnamed in the map): releases
/// `spQueuedFiles` (+0x14).
pub fn fn_009157c0(e: &mut Engine, this: Ptr<QueuedItem>) {
    e.call(QUEUED_FILE_POINTER_RELEASE, &args![this.addr() + 0x14]);
}

// Translated from 009157e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsItemQueued` (Xbox PDB): whether a queued item
/// has this object, extra list and equip flag.
pub fn middle_high_process_is_item_queued(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    object: u32,
    extra: u32,
    equip: u8,
) -> bool {
    let mut found = false;
    let mut node = word(e, this.addr(), 0x230);
    while node != 0 && node_item(e, node) != 0 && !found {
        let item = node_item(e, node);
        if item != 0
            && byte(e, item, 0xc) == equip
            && word(e, item, 0) == object
            && extra == word(e, item, 4)
        {
            found = true;
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
    found
}

/// Cancels the loading task of the item's queued file (+0x14), if any.
fn cancel_queued_file(e: &mut Engine, item: u32) {
    if e.call(FIRST_WORD, &args![item + 0x14]).u32() != 0 {
        let file = e.call(FIRST_WORD, &args![item + 0x14]).u32();
        let manager = e.global::<u32>(TASK_MANAGER);
        e.call(TRY_CANCEL_TASK, &args![manager, file, 0u32]);
    }
}

// Translated from 00915870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::RemoveQueuedItem` (Xbox PDB): the first queued item
/// with this object and extra list has its loading task cancelled, is
/// deleted and is removed from the list.
pub fn middle_high_process_remove_queued_item(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    object: u32,
    extra: u32,
) {
    let p = this.addr();
    let mut node = word(e, p, 0x230);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if item != 0 && word(e, item, 0) == object && extra == word(e, item, 4) {
            cancel_queued_file(e, item);
            fn_00915790(e, Ptr::new(item), 1);
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), item);
                let list = word(e, p, 0x230);
                e.call(LIST_REMOVE, &args![list, local]);
            });
            return;
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
}

// Translated from 00915950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::AddQueuedItem` (Xbox PDB): unless the item is
/// already queued (slot 0x174 with object, extra list and `equip`), a new
/// `QueuedItem` with these values starts preloading for `actor`
/// ([`fn_00913a20`]) and is appended to the queue (+0x230, made when
/// missing), under a debug scope guard (line 0x343). The eighth word is
/// not read.
// Translation incomplete: 00915950 - the C++ exception-unwinding frame (FS:[0]) that frees a new item or list when its constructor throws and leaves the scope guard is not translated.
#[allow(clippy::too_many_arguments)]
pub fn middle_high_process_add_queued_item(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    equip: u8,
    object: u32,
    count: i32,
    extra: u32,
    now: u8,
    force_equip: u8,
    _unused_8: u32,
    force_unequip: u8,
    equip_calling: u8,
    play_sounds: u8,
) {
    let p = this.addr();
    e.with_stack(8, |e, locals| {
        let guard = locals.addr();
        let local_item = locals.addr() + 4;
        e.call(
            SCOPE_ENTER,
            &args![guard, 0x32u32, 1u32, SOURCE_FILE, 0x343u32],
        );
        if !e
            .vcall(p, 0x174, &args![object, extra, u32::from(equip)])
            .bool()
        {
            let block = e.call(ALLOCATE, &args![0x18u32]).u32();
            let item = if block == 0 {
                0
            } else {
                fn_009139b0(e, Ptr::new(block)).addr()
            };
            e.mem.set_u32(local_item, item);
            e.mem.set_u32(item, object);
            e.mem.set_u8(item + 0xc, equip);
            e.mem.set_u32(item + 8, count as u32);
            e.mem.set_u32(item + 4, extra);
            e.mem.set_u8(item + 0xd, now);
            e.mem.set_u8(item + 0xe, force_equip);
            e.mem.set_u8(item + 0x10, force_unequip);
            e.mem.set_u8(item + 0x11, equip_calling);
            e.mem.set_u8(item + 0xf, play_sounds);
            fn_00913a20(e, Ptr::new(item), actor);
            if word(e, p, 0x230) == 0 {
                let block = e.call(ALLOCATE, &args![8u32]).u32();
                let list = if block == 0 {
                    0
                } else {
                    e.call(LIST_CONSTRUCT, &args![block]).u32()
                };
                e.mem.set_u32(p + 0x230, list);
            }
            let list = word(e, p, 0x230);
            e.call(LIST_APPEND, &args![list, local_item]);
        }
        e.call(SCOPE_LEAVE, &args![guard]);
    });
}

// Translated from 00915ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::RemoveAllQueuedItems` (Xbox PDB): cancels the
/// loading task of each queued item, deletes it and removes the list's
/// head until the list is empty; then deletes the list (+0x230) and
/// clears the pointer.
pub fn middle_high_process_remove_all_queued_items(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    let p = this.addr();
    let node = word(e, p, 0x230);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if item != 0 {
            cancel_queued_file(e, item);
            fn_00915790(e, Ptr::new(item), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![node]);
    }
    let list = word(e, p, 0x230);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.mem.set_u32(p + 0x230, 0);
}

// Translated from 00915bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessRemoveWorn` (Xbox PDB): puts the default
/// outfit back on `actor`: its base form as `TESNPC` (`006047c0`, with
/// `keep_flag`) or as `TESCreature` (`InitDefaultWorn`). Both flags are
/// set, except that a current package (slot 0x22C) of type 4 clears both
/// and sets `keep_flag`, and one with flag 0x200000 clears the second.
/// Then the actor's process gets slot 0x468 with 1, and with `notify`
/// this process gets slot 0x288 with `(actor, 1)`.
pub fn middle_high_process_process_remove_worn(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    notify: u8,
    keep_flag: u8,
) {
    let p = this.addr();
    let mut keep_flag = keep_flag;
    let base = e.call(REFERENCE_BASE, &args![actor]).u32();
    let npc = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
        )
        .u32();
    let base = e.call(REFERENCE_BASE, &args![actor]).u32();
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_CREATURE, 0u32],
        )
        .u32();
    let package = e.vcall(p, 0x22c, &[]).u32();
    let mut first = 1u32;
    let mut second = 1u32;
    if package != 0 {
        if e.call(PACKAGE_TYPE, &args![package]).u32() == 4 {
            keep_flag = 1;
            first = 0;
            second = 0;
        } else if e.call(PACKAGE_FLAG_200000, &args![package]).bool() {
            second = 0;
        }
    }
    if npc != 0 {
        e.call(
            NPC_INIT_DEFAULT_WORN,
            &args![npc, actor, first, second, u32::from(keep_flag), 1u32],
        );
    } else if creature != 0 {
        e.call(
            CREATURE_INIT_DEFAULT_WORN,
            &args![creature, actor, first, second, 1u32],
        );
    }
    if actor != 0 && e.call(WORD_68, &args![actor]).u32() != 0 {
        let process = e.call(WORD_68, &args![actor]).u32();
        e.vcall(process, 0x468, &args![1u32]);
    }
    if notify != 0 {
        e.vcall(p, 0x288, &args![actor, 1u32]);
    }
}

// Translated from 00915d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetWantWeaponDrawn` (Xbox PDB): `bWantWeaponDrawn`
/// (+0x134).
pub fn middle_high_process_get_want_weapon_drawn(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) -> u8 {
    byte(e, this.addr(), 0x134)
}

// Translated from 00915d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetWantWeaponDrawn` (Xbox PDB): stores
/// `bWantWeaponDrawn` (+0x134).
pub fn middle_high_process_set_want_weapon_drawn(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    want: u8,
) {
    e.mem.set_u8(this.addr() + 0x134, want);
}

// Translated from 00915d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetWeaponDrawn` (Xbox PDB): `bWeaponDrawn`
/// (+0x135).
pub fn middle_high_process_get_weapon_drawn(e: &mut Engine, this: Ptr<MiddleHighProcess>) -> u8 {
    byte(e, this.addr(), 0x135)
}

// Translated from 00915d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetWeaponDrawn` (Xbox PDB): when the drawn state
/// (+0x135) changes it is stored, and on sheathing with an actor the
/// process's slot 0x1CC (`ForceWeaponDrawnSheathed`) gets `(drawn, the
/// actor's slot 0x1E8, the actor's slot 0x1E4, actor)`. A drawn weapon of
/// an actor other than the player (`011dea3c`) with a weapon item change
/// (+0x114) passes its form, 0, whether mod effect 2 is active and 0 to
/// the actor's slot 0x3EC. Then `00884f80` runs on the actor.
pub fn middle_high_process_set_weapon_drawn(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    drawn: u8,
) {
    let p = this.addr();
    if byte(e, p, 0x135) != drawn {
        e.mem.set_u8(p + 0x135, drawn);
        if actor != 0 && byte(e, p, 0x135) == 0 {
            let first = e.vcall(actor, 0x1e4, &[]).u32();
            let second = e.vcall(actor, 0x1e8, &[]).u32();
            let state = u32::from(byte(e, p, 0x135));
            e.vcall(p, 0x1cc, &args![state, second, first, actor]);
        }
    }
    if actor != 0
        && actor != e.global::<u32>(PLAYER)
        && byte(e, p, 0x135) != 0
        && word(e, p, 0x114) != 0
    {
        let weapon = word(e, p, 0x114);
        let active = e
            .call(ITEM_CHANGE_MOD_EFFECT_ACTIVE, &args![weapon, 2u32])
            .u8();
        let weapon = word(e, p, 0x114);
        let form = e.call(ITEM_CHANGE_FORM, &args![weapon]).u32();
        e.vcall(actor, 0x3ec, &args![form, 0u32, u32::from(active), 0u32]);
    }
    e.call(ACTOR_UPDATE_DRAWN_FLAGS, &args![actor]);
}

// Translated from 00915e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetFurniture` (Xbox PDB): stores the furniture
/// (+0x140) and marker index (+0x144); with a marker, copies it into
/// `FurnitureMarker` (+0x148: three words, the word at +0x0C and the byte
/// at +0x0E) and, with furniture and unless bit 1 of the save/load game's
/// +0x244 is set, reserves that marker.
pub fn middle_high_process_set_furniture(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    furniture: u32,
    index: u8,
    mark: u32,
) {
    let p = this.addr();
    e.mem.set_u32(p + 0x140, furniture);
    e.mem.set_u8(p + 0x144, index);
    if mark == 0 {
        return;
    }
    let last = byte(e, mark, 0xe);
    e.mem.set_u8(p + 0x156, last);
    for offset in [0u32, 4, 8] {
        let value = word(e, mark, offset);
        e.mem.set_u32(p + 0x148 + offset, value);
    }
    let value = e.mem.u16(mark + 0xc);
    e.mem.set_u16(p + 0x154, value);
    if word(e, p, 0x140) != 0 {
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        if !e.call(SAVE_LOAD_FLAG_2, &args![save_load]).bool() {
            let furniture = word(e, p, 0x140);
            e.call(
                SET_MARKER_RESERVED,
                &args![furniture, u32::from(index), 1u32],
            );
        }
    }
}

// Translated from 00915ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the byte `008b3b90` returns for `actor` (its byte
/// at +0x70, through `0097f2f0`); the process itself is not read.
pub fn fn_00915ef0(e: &mut Engine, _this: Ptr<MiddleHighProcess>, actor: u32) -> u8 {
    e.call(ACTOR_BYTE_70, &args![actor]).u8()
}

/// Slot 0x824 (`SetupPathFinding`) with `actor`, the three floats at
/// `point` and the target's cell and worldspace (computed first, in that
/// order, as the pushes run; `point_of` gives the point's address).
fn setup_path_to(
    e: &mut Engine,
    p: u32,
    actor: u32,
    point_of: impl FnOnce(&mut Engine) -> u32,
) -> bool {
    let target = word(e, p, 0x40);
    let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![target]).u32();
    let target = word(e, p, 0x40);
    let cell = e.call(REFERENCE_CELL, &args![target]).u32();
    let point = point_of(e);
    let (x, y, z) = (word(e, point, 0), word(e, point, 4), word(e, point, 8));
    e.vcall(p, 0x824, &args![actor, x, y, z, cell, world])
        .bool()
}

/// Clears the furniture (slot 0x84), ends the move (slot 0x294) and
/// clears the target (slot 0x12C with 0).
fn abandon_furniture(e: &mut Engine, p: u32, actor: u32) {
    e.vcall(p, 0x84, &args![actor]);
    e.vcall(p, 0x294, &args![actor]);
    e.vcall(p, 0x12c, &args![0u32]);
}

/// The furniture branch of the activation: an actor whose slot 0x214
/// answers 4 or 9 marks the target activated (slot 0x4EC with 1; first
/// slot 0x288 with `(actor, 1)` when `advance`) and finishes; with
/// `iAnimActionSuccess` (+0x22A) zero, slot 0x288 `(actor, 1)`, clears
/// +0x44, ends the move, sets +0x22A to 1 and finishes; else aligns to
/// the furniture. `Some(result)` when it finishes.
fn activate_furniture(e: &mut Engine, p: u32, actor: u32, advance: bool) -> Option<bool> {
    let state = e.vcall(actor, 0x214, &[]).u32();
    if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
        if advance {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        e.vcall(p, 0x4ec, &args![1u32]);
        return Some(true);
    }
    if e.mem.i16(p + 0x22a) == 0 {
        e.vcall(p, 0x288, &args![actor, 1u32]);
        e.mem.set_u32(p + 0x44, 0);
        e.vcall(p, 0x294, &args![actor]);
        e.mem.set_u16(p + 0x22a, 1);
        return Some(true);
    }
    e.call(ALIGN_TO_FURNITURE, &args![p, actor]);
    None
}

// Translated from 00915f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessActivate` (Xbox PDB): one step of an
/// activate package for `actor` on the target (+0x40). Without a target
/// (or a target with form flag 0x20) the package target is set (slot
/// 0x8C); without one the procedure advances. A furniture location whose
/// target's base form is `011ca248`/`011ca244` goes to slot 0x628. The
/// actor is close when a door target's teleport point is within the
/// integer setting `011cde98`, when sitting on furniture (slot 0x214),
/// or as the running package (slot 0x144) says; furniture whose marker
/// is in use while pathing abandons. Not close: sitting states 4 and 9
/// run slot 0x418, else pathing (slot 0x824) heads to the target, a free
/// furniture marker or the door's linked teleport point; still pathing,
/// a high-level actor asks the package again. Close: activates
/// (`TESObjectREFR::Activate` with the acquire object +0x64, or the
/// furniture branch), runs the target's script, deletes the acquire
/// object and clears the target. `advance_procedure` selects the variant that
/// advances the procedure itself.
pub fn middle_high_process_process_activate(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    advance_procedure: u8,
) -> bool {
    let p = this.addr();
    let package = e.vcall(p, 0x27c, &[]).u32();
    let set_target = if word(e, p, 0x40) == 0 {
        true
    } else {
        let target = word(e, p, 0x40);
        e.call(FORM_FLAG_20, &args![target]).bool()
    };
    if set_target {
        e.vcall(p, 0x8c, &args![actor]);
    }
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x288, &args![actor, 1u32]);
        return false;
    }
    let location = e
        .call(PACKAGE_GET_LOCATION_REFERENCE, &args![package, actor])
        .u32();
    if location != 0 && e.call(REFERENCE_IS_FURNITURE, &args![location]).bool() {
        let target = word(e, p, 0x40);
        let mut special =
            e.call(REFERENCE_BASE, &args![target]).u32() == e.global::<u32>(ACTIVATE_FORM_A);
        if !special {
            let target = word(e, p, 0x40);
            special =
                e.call(REFERENCE_BASE, &args![target]).u32() == e.global::<u32>(ACTIVATE_FORM_B);
        }
        if special {
            e.vcall(p, 0x628, &args![location]);
            return true;
        }
    }
    if e.call(PACKAGE_TYPE, &args![package]).u32() == 9
        && !e.vcall(package, 0x148, &args![actor, 300u32]).bool()
    {
        e.vcall(p, 0x288, &args![actor, u32::MAX]);
    }
    let target = word(e, p, 0x40);
    let door = e.call(REFERENCE_DOOR_TELEPORT, &args![target]).u32();
    let mut close = false;
    if door != 0 {
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        let length = e.with_stack(12, |e, offset| {
            let target = word(e, p, 0x40);
            let door_point = e.call(REFERENCE_DOOR_POSITION, &args![target]).u32();
            e.call(POINT_SUBTRACT, &args![door_point, offset, at]);
            e.call(POINT_LENGTH, &args![offset]).f64()
        });
        let value = e.call(SETTING_VALUE, &args![DOOR_DISTANCE_SETTING]).u32();
        let limit = f64::from(e.mem.i32(value));
        if limit >= length {
            close = true;
        }
    } else {
        let target = word(e, p, 0x40);
        let sitting = e.call(REFERENCE_IS_FURNITURE, &args![target]).bool()
            && e.vcall(actor, 0x214, &[]).u32() != 0;
        if sitting {
            close = true;
        } else {
            let target = word(e, p, 0x40);
            if e.call(REFERENCE_IS_FURNITURE, &args![target]).bool()
                && e.vcall(actor, 0x214, &[]).u32() == 0
            {
                let target = word(e, p, 0x40);
                let index = u32::from(byte(e, p, 0x144));
                if e.call(REFERENCE_GET_MARKER_USED, &args![target, index, 1u32])
                    .bool()
                    && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
                {
                    abandon_furniture(e, p, actor);
                    return false;
                }
            }
            close = e.vcall(package, 0x144, &args![actor, 0u32]).bool();
        }
    }
    if !close && fn_00915ef0(e, this, actor) == 0 {
        let state = e.vcall(actor, 0x214, &[]).u32();
        if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
            e.vcall(actor, 0x418, &[]);
            return true;
        }
        if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            let target = word(e, p, 0x40);
            if e.call(REFERENCE_DOOR_TELEPORT, &args![target]).u32() == 0 {
                let started = setup_path_to(e, p, actor, |e| {
                    let target = word(e, p, 0x40);
                    e.vcall(target, 0x1f4, &[]).u32()
                });
                if !started {
                    return false;
                }
            } else {
                let target = word(e, p, 0x40);
                if e.call(REFERENCE_IS_FURNITURE, &args![target]).bool() {
                    let at = e.vcall(actor, 0x1f4, &[]).u32();
                    let marker = e.with_stack(16, |e, locals| {
                        let point = locals.addr();
                        let index = locals.addr() + 12;
                        for offset in [0u32, 4, 8] {
                            let value = word(e, at, offset);
                            e.mem.set_u32(point + offset, value);
                        }
                        e.mem.set_u32(index, 0);
                        let target = word(e, p, 0x40);
                        let found = e
                            .call(
                                REFERENCE_GET_CLOSEST_FREE_MARKER,
                                &args![target, point, 1u32, 1u32, p + 0x148, index, 1u32],
                            )
                            .bool();
                        found.then(|| e.mem.u8(index))
                    });
                    let Some(index) = marker else {
                        abandon_furniture(e, p, actor);
                        return false;
                    };
                    if !setup_path_to(e, p, actor, |_| p + 0x148) {
                        return false;
                    }
                    e.mem.set_u8(p + 0x144, index);
                } else {
                    let started = setup_path_to(e, p, actor, |e| {
                        let target = word(e, p, 0x40);
                        e.call(REFERENCE_DOOR_POSITION, &args![target]).u32()
                    });
                    if !started {
                        return false;
                    }
                }
            }
        }
        if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            // The hours since the process's +0x20 (wrapped by 24) scaled by
            // 3600 / global 0x3A are stored in a local nothing reads; only
            // the calls remain.
            let global = e.call(LOOKUP_FORM, &args![0x3au32]).u32();
            e.call(CALENDAR_GET_HOUR, &args![CALENDAR]);
            e.call(FLOAT_20, &args![p]);
            e.call(FLOAT_20, &args![p]);
            e.call(GLOBAL_VALUE, &args![global]);
            if e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32() != 1
                || e.call(GET_DESIRED_PROCESS_LEVEL, &args![actor]).u32() != 1
            {
                return false;
            }
            close = e.vcall(package, 0x144, &args![actor, 0u32]).bool();
        }
    }
    if !close && fn_00915ef0(e, this, actor) == 0 {
        return false;
    }
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        e.vcall(p, 0x294, &args![actor]);
    }
    let current = e.vcall(p, 0x22c, &[]).u32();
    if e.call(PACKAGE_TYPE, &args![current]).u32() == 0x1c && fn_00915ef0(e, this, actor) != 0 {
        e.vcall(actor, 0x288, &[]);
        return false;
    }
    let saved = word(e, p, 0x40);
    if advance_procedure != 0 {
        if fn_00915ef0(e, this, actor) == 0 && !e.call(PACKAGE_FLAG_4, &args![package]).bool() {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        if fn_00915ef0(e, this, actor) == 0 {
            let target = word(e, p, 0x40);
            if e.call(REFERENCE_IS_FURNITURE, &args![target]).bool() {
                if let Some(result) = activate_furniture(e, p, actor, true) {
                    return result;
                }
            } else if word(e, p, 0x64) != 0 {
                let object = word(e, word(e, p, 0x64), 4);
                let target = word(e, p, 0x40);
                e.call(
                    REFERENCE_ACTIVATE,
                    &args![target, actor, 1u32, object, 1u32],
                );
            } else {
                let target = word(e, p, 0x40);
                if !e.vcall(target, 0x100, &[]).bool() {
                    let target = word(e, p, 0x40);
                    if !e
                        .call(REFERENCE_ACTIVATE, &args![target, actor, 1u32, 0u32, 1u32])
                        .bool()
                        && e.call(PACKAGE_FLAG_4, &args![package]).bool()
                    {
                        e.vcall(p, 0x288, &args![actor, u32::MAX]);
                    }
                }
            }
        }
    } else if fn_00915ef0(e, this, actor) == 0 {
        let target = word(e, p, 0x40);
        if e.call(REFERENCE_IS_FURNITURE, &args![target]).bool() {
            if let Some(result) = activate_furniture(e, p, actor, false) {
                return result;
            }
        } else {
            let object = if word(e, p, 0x64) != 0 {
                word(e, word(e, p, 0x64), 4)
            } else {
                0
            };
            let target = word(e, p, 0x40);
            e.call(
                REFERENCE_ACTIVATE,
                &args![target, actor, 0u32, object, 1u32],
            );
        }
    }
    if saved != 0 {
        e.call(REFERENCE_RUN_SCRIPT, &args![saved]);
    }
    let acquire = word(e, p, 0x64);
    if acquire != 0 {
        e.call(ACQUIRE_OBJECT_DELETE, &args![acquire, 1u32]);
    }
    e.mem.set_u32(p + 0x64, 0);
    e.vcall(p, 0x12c, &args![0u32]);
    fn_00915ef0(e, this, actor) == 0
}

/// Whether `actor`'s combat controller (slot 0x428) has `other` as a
/// combat target (`CombatController::IsActoraCombatTarget`).
fn is_combat_target_of(e: &mut Engine, actor: u32, other: u32) -> bool {
    let controller = e.vcall(actor, 0x428, &[]).u32();
    e.call(IS_COMBAT_TARGET, &args![controller, other]).bool()
}

/// `PlayerCharacter::AddActorToPlayerCombatList` for `actor` when
/// `008a8230` on the player gives a positive count.
fn add_to_player_combat(e: &mut Engine, actor: u32) {
    let player = e.global::<u32>(PLAYER);
    if e.call(PLAYER_COMBAT_STANDING, &args![player, actor]).i32() > 0 {
        let player = e.global::<u32>(PLAYER);
        e.call(ADD_TO_PLAYER_COMBAT_LIST, &args![player, actor]);
    }
}

// Translated from 00916880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::EnterCombat` (Xbox PDB): `actor` starts combat
/// with `target`. Nothing when slot 0x324 answers true, or when (unless
/// `flag_1`) a package of type 9 refuses (slot 0x140). Without a target
/// and `flag_5`, the actor's slot 0x424 gets zeros and `word_13`. A target
/// at another process level makes `LowProcess::EnterCombat` run first
/// (unless the actor's slot 0x1D0 answers). Refused for actors in the
/// states `00437bf0`/`00437bd0`, when already fighting the target,
/// against the player above the integer setting `011ccffc` (unless
/// `flag_10`), or across the water-only rules. A target out of reach of
/// an interrupting package's radius (default 600 below 1.0) that is not
/// fighting the actor sets slot 0xD0 with 1 and refuses. Otherwise an
/// actor not yet in combat clears slot 0xD0, runs its slot 0x288 and
/// keeps the acquire object (+0x64 to +0x68), the aggressor flag (slot
/// 0x1C8) is `flag_8` unless slot 0x304 or the target's combat says
/// otherwise, and the actor's slot 0x424 starts combat; the player's
/// combat list gains the actor. Returns whether combat started.
#[allow(clippy::too_many_arguments)]
pub fn middle_high_process_enter_combat(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    target: u32,
    flag_1: u8,
    flag_2: u8,
    word_5: u32,
    flag_6: u8,
    flag_7: u8,
    flag_8: u8,
    flag_9: u8,
    flag_10: u8,
    flag_11: u8,
    flag_12: u8,
    word_13: u32,
) -> bool {
    let p = this.addr();
    if e.vcall(p, 0x324, &[]).bool() {
        return false;
    }
    let package = e.vcall(p, 0x22c, &[]).u32();
    if flag_1 == 0 && package != 0 && e.call(PACKAGE_TYPE, &args![package]).u32() == 9 {
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        if !e
            .vcall(package, 0x140, &args![target, actor, 0u32, minus_one, 0u32])
            .bool()
        {
            return false;
        }
    }
    if target == 0 && word_5 == 0 {
        e.vcall(
            actor,
            0x424,
            &args![0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, word_13],
        );
        return true;
    }
    if target != 0 {
        let target_level = e.call(GET_CURRENT_PROCESS_TYPE, &args![target]).u32();
        let actor_level = e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32();
        if target_level != actor_level && e.vcall(actor, 0x1d0, &[]).u32() == 0 {
            e.call(
                LOW_PROCESS_ENTER_COMBAT,
                &args![
                    p,
                    actor,
                    target,
                    u32::from(flag_1),
                    u32::from(flag_2),
                    word_5,
                    u32::from(flag_6),
                    u32::from(flag_7),
                    u32::from(flag_8),
                    u32::from(flag_9),
                    u32::from(flag_11),
                    0u32,
                    1u32,
                    0u32
                ],
            );
        }
    }
    if e.call(ACTOR_STATE_IS_5, &args![actor]).bool()
        || e.call(ACTOR_STATE_IS_3, &args![actor]).bool()
    {
        return false;
    }
    if target != 0 && e.vcall(actor, 0x428, &[]).u32() == 0 && flag_10 == 0 {
        let object = e.global::<u32>(COMBAT_CHECK_OBJECT);
        if e.call(ALWAYS_FALSE_2, &args![object, actor, target]).bool() {
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            let minus_one_again = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            e.vcall(
                actor,
                0x410,
                &args![
                    target,
                    1u32,
                    1u32,
                    1u32,
                    0u32,
                    0u32,
                    minus_one_again,
                    minus_one
                ],
            );
            if target == e.global::<u32>(PLAYER) {
                add_to_player_combat(e, actor);
            }
            return true;
        }
    }
    if e.vcall(actor, 0x428, &[]).u32() != 0
        && target != 0
        && word_5 == 0
        && is_combat_target_of(e, actor, target)
    {
        return false;
    }
    if target == e.global::<u32>(PLAYER) && flag_10 == 0 {
        let player = e.global::<u32>(PLAYER);
        let count = e.call(GET_NUMBER_ACTORS_IN_COMBAT, &args![player]).i32();
        let limit = e.call(SETTING_VALUE, &args![COMBAT_LIMIT_SETTING]).u32();
        if count >= e.mem.i32(limit) {
            return false;
        }
    }
    if target != 0 {
        let target_in_water = e.call(REFERENCE_BYTE_14D, &args![target]).u8();
        if !e.call(ACTOR_0087F3B0, &args![actor]).bool() && target_in_water != 0 {
            return false;
        }
        if e.call(ACTOR_SWIMS_ONLY, &args![actor]).bool() && target_in_water == 0 {
            return false;
        }
    }
    e.vcall(p, 0x4a0, &[]);
    let mut running = e.vcall(p, 0x27c, &[]).u32();
    if running != 0 && e.call(PACKAGE_IS_INTERRUPT, &args![running]).bool() {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
        running = e.call(GET_PACKAGE_EXTRA, &args![list]).u32();
    }
    if target != 0
        && running != 0
        && e.call(PACKAGE_FLAG_400000, &args![running]).bool()
        && flag_10 == 0
        && flag_11 == 0
    {
        let refused = e.with_stack(12, |e, location| {
            e.vcall(p, 0x98, &args![location, actor, 0u32]);
            let count = e.call(PACKAGE_RADIUS, &args![running, actor]).u32();
            let mut radius = f64::from(count) as f32;
            if f64::from(radius) < e.global::<f64>(ONE_DOUBLE) {
                radius = x87_quiet(e.global::<f32>(DEFAULT_RADIUS));
            }
            let distance = e
                .call(REFERENCE_DISTANCE_TO, &args![target, location])
                .f64();
            if f64::from(radius) < distance
                && e.vcall(target, 0x428, &[]).u32() != 0
                && !is_combat_target_of(e, target, actor)
            {
                e.vcall(p, 0xd0, &args![1u32]);
                return true;
            }
            false
        });
        if refused {
            return false;
        }
    }
    if e.vcall(actor, 0x428, &[]).u32() == 0 {
        e.vcall(p, 0xd0, &args![0u32]);
        e.vcall(actor, 0x288, &[]);
        let acquire = word(e, p, 0x64);
        e.mem.set_u32(p + 0x68, acquire);
        e.mem.set_u32(p + 0x64, 0);
    }
    let mut aggressor = flag_8;
    if e.vcall(actor, 0x304, &[]).bool() {
        aggressor = 0;
    }
    if target != 0
        && e.call(REFERENCE_BYTE_104, &args![target]).bool()
        && e.vcall(target, 0x428, &[]).u32() != 0
        && is_combat_target_of(e, target, actor)
    {
        aggressor = 0;
    }
    e.vcall(p, 0x1c8, &args![u32::from(aggressor)]);
    let delay = if flag_10 != 0 { 200u32 } else { 0 };
    e.vcall(
        actor,
        0x424,
        &args![
            target,
            word_5,
            u32::from(flag_10),
            u32::from(aggressor),
            u32::from(flag_6),
            delay,
            u32::from(flag_12),
            word_13
        ],
    );
    if target == e.global::<u32>(PLAYER) && e.call(REFERENCE_BYTE_104, &args![actor]).bool() {
        add_to_player_combat(e, actor);
    }
    true
}

/// `out = point_of(a) - point_of(b)` through `POINT_SUBTRACT`, where the
/// minuend is `minuend` and the subtrahend `subtrahend` (addresses of
/// three floats); `scratch` receives the difference first.
fn subtract_into(e: &mut Engine, out: u32, scratch: u32, minuend: u32, subtrahend: u32) {
    let result = e
        .call(POINT_SUBTRACT, &args![minuend, scratch, subtrahend])
        .u32();
    for offset in [0u32, 4, 8] {
        let value = word(e, result, offset);
        e.mem.set_u32(out + offset, value);
    }
}

/// The calls of the elapsed-hours value `ProcessActivate` and
/// `ProcessEscort` compute into a local nothing reads.
fn unused_hours_calls(e: &mut Engine, p: u32) {
    let global = e.call(LOOKUP_FORM, &args![0x3au32]).u32();
    e.call(CALENDAR_GET_HOUR, &args![CALENDAR]);
    e.call(FLOAT_20, &args![p]);
    e.call(FLOAT_20, &args![p]);
    e.call(GLOBAL_VALUE, &args![global]);
}

// Translated from 00916df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessEscort` (Xbox PDB): one step of an escort
/// package for `actor`. Without a target the package target is set (slot
/// 0x8C). Without the actor's item change (`008a2d40`), a target that is
/// not an actor (slot 0x100) and fails `00575450` advances the procedure
/// (slot 0x288) and ends the move. An actor target other than the actor
/// is escorted: one other than the player whose package is not of type 1
/// nor created resets the procedure (slot 0x284). The escort distance is
/// the package's (`00671d10`, 200 when not positive) for the player, the
/// setting `011cdbac` in cells with `00425fd0`, else that distance times
/// the setting `011cd82c`. When the package location exists and slot
/// 0x13C answers true: without an escort the item change goes to the
/// location's container (`00892e90`) or through slot 0x3CC, is deleted
/// and the procedure advances; with an escort within the distance the
/// procedure advances and a created package of the escorted actor ends
/// (its slot 0x288, slot 0x10 with 1, slot 0x234, `EndMovement`) before
/// `CheckforNewPackage` (slot 0x24). Otherwise an escort that falls behind
/// waits (slot 0x120 with 1, slot 0x288 with -1) and the actor paths to
/// the package location (slot 0x824).
pub fn middle_high_process_process_escort(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let package = e.vcall(p, 0x27c, &[]).u32();
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x8c, &args![actor]);
    }
    if e.call(ACTOR_ITEM_CHANGE, &args![actor]).u32() == 0 {
        let fail = if word(e, p, 0x40) == 0 {
            true
        } else {
            let target = word(e, p, 0x40);
            if e.vcall(target, 0x100, &[]).bool() {
                false
            } else {
                let target = word(e, p, 0x40);
                let owner = e.call(WORD_C, &args![target]).u32();
                !e.call(REFERENCE_INVENTORY_TEST, &args![actor, owner])
                    .bool()
            }
        };
        if fail {
            e.vcall(p, 0x288, &args![actor, 1u32]);
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
            return;
        }
    }
    let mut escort = false;
    if word(e, p, 0x40) != 0 {
        let target = word(e, p, 0x40);
        if e.vcall(target, 0x100, &[]).bool() && word(e, p, 0x40) != actor {
            escort = true;
            let target = word(e, p, 0x40);
            let leader = e
                .call(
                    DYNAMIC_CAST,
                    &args![target, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
                )
                .u32();
            let process = e.call(WORD_68, &args![leader]).u32();
            let leader_package = e.vcall(process, 0x22c, &[]).u32();
            if word(e, p, 0x40) != e.global::<u32>(PLAYER)
                && (leader_package == 0
                    || (e.call(PACKAGE_TYPE, &args![leader_package]).u32() != 1
                        && !e
                            .call(PACKAGE_GET_IS_CREATED, &args![leader_package])
                            .bool()))
            {
                e.vcall(p, 0x284, &args![0u32]);
                return;
            }
        }
    }
    // Locals: the offset to the target (+0x00), the actor's and the
    // escorted actor's offsets to the package location (+0x0C, +0x18) and
    // a scratch point (+0x24).
    e.with_stack(0x30, |e, locals| {
        let offset = locals.addr();
        let actor_offset = offset + 0xc;
        let leader_offset = offset + 0x18;
        let scratch = offset + 0x24;
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        let again = e.vcall(actor, 0x1f4, &[]).u32();
        e.call(POINT_SUBTRACT, &args![again, offset, at]);
        let mut distance = 0.0f32;
        let mut leader = 0u32;
        e.call(IDENTITY, &args![actor_offset]);
        e.call(IDENTITY, &args![leader_offset]);
        if escort {
            let target = word(e, p, 0x40);
            let target_at = e.vcall(target, 0x1f4, &[]).u32();
            let actor_at = e.vcall(actor, 0x1f4, &[]).u32();
            subtract_into(e, offset, scratch, actor_at, target_at);
            leader = word(e, p, 0x40);
            distance = 0.0;
            let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
            let mut range = e.call(ITEM_CHANGE_FORM, &args![data]).i32();
            if range <= 0 {
                range = 200;
            }
            let zero = e.global::<f64>(ZERO_DOUBLE);
            if word(e, p, 0x40) == e.global::<u32>(PLAYER) && f64::from(distance) == zero {
                distance = range as f32;
            } else {
                let mut done = false;
                if f64::from(distance) == zero {
                    let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
                    if e.call(CELL_FLAG, &args![cell]).bool() {
                        let value = e
                            .call(FLOAT_SETTING_VALUE, &args![ESCORT_CELL_DISTANCE_SETTING])
                            .u32();
                        distance = x87_quiet(e.mem.f32(value));
                        done = true;
                    }
                }
                if !done && f64::from(distance) == zero {
                    let value = e
                        .call(FLOAT_SETTING_VALUE, &args![ESCORT_DISTANCE_FACTOR_SETTING])
                        .u32();
                    distance = (f64::from(e.mem.f32(value)) * f64::from(range)) as f32;
                }
            }
        }
        let location = e.call(PACKAGE_LOCATION, &args![package]).u32();
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        if location != 0
            && e.vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32])
                .bool()
        {
            if !escort {
                let change = e.call(ACTOR_ITEM_CHANGE, &args![actor]).u32();
                if change != 0 {
                    let mut extra = 0u32;
                    if e.call(FIRST_WORD, &args![change]).u32() != 0 {
                        let lists = e.call(FIRST_WORD, &args![change]).u32();
                        let slot = e.call(IDENTITY, &args![lists]).u32();
                        extra = e.mem.u32(slot);
                    }
                    let mut owner = 0u32;
                    if extra != 0 && e.call(GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
                        let reference = e.call(GET_REFERENCE_POINTER, &args![extra]).u32();
                        owner = e.call(WORD_C, &args![reference]).u32();
                    }
                    let location = e.call(PACKAGE_LOCATION, &args![package]).u32();
                    let mut container = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
                    if container == 0 {
                        container = word(e, p, 0x44);
                    }
                    if container != 0
                        && e.call(REFERENCE_HAS_CONTAINER, &args![container]).u32() != 0
                    {
                        let count = e
                            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
                            .u32();
                        let form = e.call(ITEM_CHANGE_FORM, &args![change]).u32();
                        e.call(
                            ACTOR_ITEM_TO_CONTAINER,
                            &args![actor, form, extra, container, count, owner],
                        );
                    } else {
                        if e.call(FIRST_WORD, &args![change]).u32() != 0 {
                            let lists = e.call(FIRST_WORD, &args![change]).u32();
                            let slot = e.call(IDENTITY, &args![lists]).u32();
                            extra = e.mem.u32(slot);
                        }
                        let mut point = 0u32;
                        let location = e.call(PACKAGE_LOCATION, &args![package]).u32();
                        if location != 0 {
                            let mut reference =
                                e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
                            if reference == 0 {
                                reference = word(e, p, 0x44);
                            }
                            if reference != 0 {
                                let mut special = e.call(REFERENCE_BASE, &args![reference]).u32()
                                    == e.global::<u32>(ACTIVATE_FORM_B);
                                if !special {
                                    special = e.call(REFERENCE_BASE, &args![reference]).u32()
                                        == e.global::<u32>(ACTIVATE_FORM_A);
                                }
                                if special {
                                    point = e.vcall(reference, 0x1f4, &[]).u32();
                                }
                            }
                        }
                        let count = e
                            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
                            .u32();
                        let form = e.call(ITEM_CHANGE_FORM, &args![change]).u32();
                        e.vcall(actor, 0x3cc, &args![form, extra, count, point, 0u32]);
                    }
                }
                if change != 0 {
                    e.call(ITEM_CHANGE_DELETE, &args![change, 1u32]);
                }
                e.vcall(p, 0x288, &args![actor, 1u32]);
            } else {
                let length = e.call(POINT_LENGTH, &args![offset]).f64();
                if f64::from(distance) >= length {
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                    let leader = word(e, p, 0x40);
                    let process = e.call(WORD_68, &args![leader]).u32();
                    if e.vcall(process, 0x27c, &[]).u32() != 0 {
                        let process = e.call(WORD_68, &args![leader]).u32();
                        e.vcall(process, 0x288, &args![actor, 1u32]);
                        let process = e.call(WORD_68, &args![leader]).u32();
                        let running = e.vcall(process, 0x27c, &[]).u32();
                        if e.call(PACKAGE_GET_IS_CREATED, &args![running]).bool() {
                            let process = e.call(WORD_68, &args![leader]).u32();
                            let running = e.vcall(process, 0x27c, &[]).u32();
                            if running != 0 {
                                e.vcall(running, 0x10, &args![1u32]);
                            }
                            let process = e.call(WORD_68, &args![leader]).u32();
                            e.vcall(process, 0x234, &[]);
                            if e.call(ACTOR_IS_PATHING_COMPLETE, &args![leader]).bool() {
                                e.call(ACTOR_END_MOVEMENT, &args![leader]);
                            }
                            e.vcall(p, 0x24, &args![actor, 0u32]);
                            return;
                        }
                    }
                }
            }
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
            return;
        }
        if escort {
            let coord = e
                .call(PACKAGE_GET_LOCATION_COORD, &args![package, scratch, actor])
                .u32();
            let actor_at = e.vcall(actor, 0x1f4, &[]).u32();
            e.with_stack(12, |e, difference| {
                subtract_into(e, actor_offset, difference.addr(), actor_at, coord);
            });
            let coord = e
                .call(PACKAGE_GET_LOCATION_COORD, &args![package, scratch, actor])
                .u32();
            let leader_at = e.vcall(leader, 0x1f4, &[]).u32();
            e.with_stack(12, |e, difference| {
                subtract_into(e, leader_offset, difference.addr(), leader_at, coord);
            });
            let length = e.call(POINT_LENGTH, &args![offset]).f64();
            let mut wait = false;
            let mut check_leader = true;
            if f64::from(distance) < length {
                let actor_length = e.call(POINT_LENGTH, &args![actor_offset]).f64();
                let leader_length = e.call(POINT_LENGTH, &args![leader_offset]).f64();
                // FCOMP: the jump to the wait is taken unless C0 (less or
                // unordered).
                if !(leader_length < actor_length
                    || leader_length.is_nan()
                    || actor_length.is_nan())
                {
                    wait = true;
                    check_leader = false;
                }
            }
            if check_leader
                && leader != e.global::<u32>(PLAYER)
                && e.call(MOBILE_GET_CURRENT_PACKAGE, &args![leader]).u32() != 0
            {
                let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![leader]).u32();
                if e.call(PACKAGE_IS_INTERRUPT, &args![current]).bool() {
                    wait = true;
                }
            }
            if wait && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x120, &args![1u32]);
                e.vcall(p, 0x288, &args![actor, u32::MAX]);
                return;
            }
        }
        if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            let go = if escort {
                let length = e.call(POINT_LENGTH, &args![offset]).f64();
                f64::from(distance) >= length
            } else {
                true
            };
            if go {
                let world = e
                    .call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
                    .u32();
                let cell = e
                    .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
                    .u32();
                let coord = e
                    .call(PACKAGE_GET_LOCATION_COORD, &args![package, scratch, actor])
                    .u32();
                let (x, y, z) = (word(e, coord, 0), word(e, coord, 4), word(e, coord, 8));
                if !e
                    .vcall(p, 0x824, &args![actor, x, y, z, cell, world])
                    .bool()
                {
                    return;
                }
            }
        }
        if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            unused_hours_calls(e, p);
        }
    });
}

// Translated from 00917770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessAccompanyOneHour` (Xbox PDB): an hour of
/// following ([`middle_high_process_process_follow_one_hour`]) with the
/// float at `01084838` (3600.0) and 1.
pub fn middle_high_process_process_accompany_one_hour(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let seconds = x87_quiet(e.global::<f32>(ACCOMPANY_SECONDS));
    middle_high_process_process_follow_one_hour(e, this, actor, seconds.to_bits(), 1);
}

// Translated from 009177a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessAccompany` (Xbox PDB): slot 0x298
/// (`ProcessFollow`) with `(actor, 1, 0x101, 0)`.
pub fn middle_high_process_process_accompany(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    e.vcall(this.addr(), 0x298, &args![actor, 1u32, 0x101u32, 0u32]);
}

// Translated from 009177d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessSleep` (Xbox PDB): an actor that is not
/// sitting (slot 0x214 zero) and passes slot 0x218 looks for beds
/// (`FindBedChairs`) when it has no furniture and the bed list (+0xD0) is
/// empty. Unless sleeping (state 9), it then takes a bed under the lock
/// `011e043c`: the first of the list when that one has an owner (and it
/// is removed from the list), else a random one; targets it (slot 0x12C),
/// activates it (slot 0x7C8) and drops it again when `00915ef0` answers.
/// Sleeping, or without furniture and beds, marks the action complete
/// (slot 0x118) and empties the list. Then slot 0x44 with `(actor, 0, 2,
/// 1, 0, 1)`.
pub fn middle_high_process_process_sleep(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) {
    let p = this.addr();
    if e.vcall(actor, 0x218, &[]).bool()
        && e.vcall(actor, 0x214, &[]).u32() == 0
        && word(e, p, 0x140) == 0
        && e.call(LIST_IS_EMPTY, &args![p + 0xd0]).bool()
    {
        e.call(FIND_BED_CHAIRS, &args![p, actor, 0u32, 0u32]);
    }
    if e.vcall(actor, 0x214, &[]).u32() != 9 && e.vcall(actor, 0x218, &[]).bool() {
        take_furniture(e, p, false);
        let furniture = word(e, p, 0x140);
        e.vcall(p, 0x12c, &args![furniture]);
        if word(e, p, 0x140) != 0 {
            e.vcall(p, 0x7c8, &args![actor, 0u32]);
        }
        if fn_00915ef0(e, this, actor) != 0 {
            e.mem.set_u32(p + 0x140, 0);
            e.call(CLEAR_FURNITURE_MARKER, &args![p]);
        }
    }
    let done = if e.vcall(actor, 0x218, &[]).bool() && e.vcall(actor, 0x214, &[]).u32() == 9 {
        true
    } else {
        word(e, p, 0x140) == 0 && e.call(LIST_IS_EMPTY, &args![p + 0xd0]).bool()
    };
    if done {
        e.vcall(p, 0x118, &args![1u32]);
        e.call(LIST_CLEAR, &args![p + 0xd0]);
    }
    e.vcall(p, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
}

/// Without furniture (+0x140) and with a non-empty furniture list
/// (+0xD0), under the lock `011e043c` (only when it can be entered at
/// once): takes the list's first entry; one with an owner is removed from
/// the list, else a random entry is taken instead (removed from the list
/// too when `remove_random`). The furniture marker is cleared after each
/// choice.
fn take_furniture(e: &mut Engine, p: u32, remove_random: bool) {
    if word(e, p, 0x140) != 0
        || e.call(LIST_COUNT, &args![p + 0xd0]).u32() == 0
        || !e.call(TRY_ENTER_LOCK, &args![FURNITURE_LOCK]).bool()
    {
        return;
    }
    let first = node_item(e, p + 0xd0);
    e.mem.set_u32(p + 0x140, first);
    e.call(CLEAR_FURNITURE_MARKER, &args![p]);
    let furniture = word(e, p, 0x140);
    if e.call(REFERENCE_GET_OWNER, &args![furniture]).u32() != 0 {
        e.call(LIST_REMOVE, &args![p + 0xd0, p + 0x140]);
    } else {
        let count = e.call(LIST_COUNT, &args![p + 0xd0]).u32();
        let random = e.call(RANDOM, &[]).u32();
        let mut index = random % count;
        if (index as i32) >= (count as i32) {
            index = count;
        }
        let mut node = p + 0xd0;
        let mut step = 0i32;
        while step < index as i32 {
            node = e.call(WORD_4, &args![node]).u32();
            step += 1;
        }
        let chosen = node_item(e, node);
        e.mem.set_u32(p + 0x140, chosen);
        e.call(CLEAR_FURNITURE_MARKER, &args![p]);
        if remove_random {
            e.call(LIST_REMOVE, &args![p + 0xd0, p + 0x140]);
        }
    }
    e.call(LEAVE_LOCK, &args![FURNITURE_LOCK]);
}

// Translated from 00917a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessEat` (Xbox PDB): with the action complete
/// (slot 0x11C) only slot 0x44 runs. Otherwise like
/// [`middle_high_process_process_sleep`] with chairs (`FindBedChairs` with
/// `(actor, 1, search)`, state 4 instead of 9, the random chair removed
/// from the list too, slot 0x84 instead of dropping the furniture); when
/// done the actor eats its best food item (`Actor::Eat` with the item's
/// form, its first extra list and 1). Then slot 0x44 with `(actor, 0, 2,
/// 1, 0, 1)` and, with bit 0x400 of `008846e0`, `00884f80` and
/// `Actor::ClearMoveMode` with 0x400.
pub fn middle_high_process_process_eat(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    search: u32,
) {
    let p = this.addr();
    e.vcall(p, 0x27c, &[]);
    let food = e.call(ACTOR_GET_BEST_FOOD_ITEM, &args![actor]).u32();
    if e.vcall(p, 0x11c, &[]).bool() {
        e.vcall(p, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
        return;
    }
    if e.vcall(actor, 0x218, &[]).bool()
        && e.vcall(actor, 0x214, &[]).u32() == 0
        && word(e, p, 0x140) == 0
        && e.call(LIST_IS_EMPTY, &args![p + 0xd0]).bool()
    {
        e.call(FIND_BED_CHAIRS, &args![p, actor, 1u32, search]);
    }
    if e.vcall(actor, 0x218, &[]).bool() && e.vcall(actor, 0x214, &[]).u32() != 4 {
        take_furniture(e, p, true);
        let furniture = word(e, p, 0x140);
        e.vcall(p, 0x12c, &args![furniture]);
        if word(e, p, 0x140) != 0 {
            e.vcall(p, 0x7c8, &args![actor, 0u32]);
        }
        if fn_00915ef0(e, this, actor) != 0 {
            e.vcall(p, 0x84, &args![actor]);
        }
    }
    let done = if e.vcall(actor, 0x218, &[]).bool() && e.vcall(actor, 0x214, &[]).u32() == 4 {
        true
    } else {
        word(e, p, 0x140) == 0 && e.call(LIST_IS_EMPTY, &args![p + 0xd0]).bool()
    };
    if done {
        if food != 0 {
            let mut extra = 0u32;
            if e.call(FIRST_WORD, &args![food]).u32() != 0 {
                let lists = e.call(FIRST_WORD, &args![food]).u32();
                let slot = e.call(IDENTITY, &args![lists]).u32();
                extra = e.mem.u32(slot);
            }
            let form = e.call(ITEM_CHANGE_FORM, &args![food]).u32();
            e.call(ACTOR_EAT, &args![actor, form, extra, 1u32]);
        }
        e.vcall(p, 0x118, &args![1u32]);
        e.call(LIST_CLEAR, &args![p + 0xd0]);
    }
    e.vcall(p, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
    if e.call(ACTOR_MOVEMENT_FLAGS, &args![actor]).u32() & 0x400 != 0 {
        e.call(ACTOR_UPDATE_DRAWN_FLAGS, &args![actor]);
        e.call(ACTOR_CLEAR_MOVE_MODE, &args![actor, 0x400u32]);
    }
}

/// Puts the default outfit back on `actor` as `CheckforNewPackage` does:
/// its base form as `TESNPC` (`006047c0` with `(1, second, 0, 1)`) or as
/// `TESCreature` (`InitDefaultWorn` with `(1, second, 1)`), where `second`
/// is cleared by a current package (slot 0x22C) with flag 0x200000.
fn reset_outfit(e: &mut Engine, p: u32, actor: u32) {
    let base = e.call(REFERENCE_BASE, &args![actor]).u32();
    let npc = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
        )
        .u32();
    let base = e.call(REFERENCE_BASE, &args![actor]).u32();
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_CREATURE, 0u32],
        )
        .u32();
    let package = e.vcall(p, 0x22c, &[]).u32();
    let mut second = 1u32;
    if package != 0 && e.call(PACKAGE_FLAG_200000, &args![package]).bool() {
        second = 0;
    }
    if npc != 0 {
        e.call(
            NPC_INIT_DEFAULT_WORN,
            &args![npc, actor, 1u32, second, 0u32, 1u32],
        );
    } else if creature != 0 {
        e.call(
            CREATURE_INIT_DEFAULT_WORN,
            &args![creature, actor, 1u32, second, 1u32],
        );
    }
}

// Translated from 00917df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::CheckforNewPackage` (Xbox PDB): nothing for an
/// actor with byte +0x104, in dialogue, alarmed or running a run-once
/// package, or whose slot 0x214 is not 0, 4 or 9. A run-once package
/// (slot 0x20C) is cleared (slot 0x214). With `force`, without a current
/// package (slot 0x22C) or once the game hour (truncated, `_ftol2`)
/// differs from `iHourPackageEvaluated` (+0xB4),
/// `LowProcess::CheckforNewPackage` runs and the hour is stored. A new
/// package resets the procedure state (+0x22A to 2, slots 0x4E4, 0x118,
/// 0x4EC with 0, process flag 0x40 cleared), deletes the objects to
/// acquire (+0x5C) and, when the previous package was of type 4 or had
/// flag 0x200000, puts the default outfit back. Returns whether a new
/// package was chosen.
pub fn middle_high_process_checkfor_new_package(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    force: u8,
) -> bool {
    let p = this.addr();
    if e.call(REFERENCE_BYTE_104, &args![actor]).bool()
        || e.call(MOBILE_IS_IN_DIALOGUE, &args![actor]).bool()
        || e.call(ACTOR_IS_ALARMED, &args![actor]).bool()
        || e.call(ACTOR_IS_RUNNING_RUN_ONCE, &args![actor]).bool()
    {
        return false;
    }
    let state = e.vcall(actor, 0x214, &[]).u32();
    if state != 0 && state != 4 && state != 9 {
        return false;
    }
    if e.vcall(p, 0x20c, &[]).u32() != 0 {
        e.vcall(p, 0x214, &[]);
    }
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64() as f32;
    let mut changed = false;
    let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32();
    let mut outfit = false;
    if current != 0
        && (e.call(PACKAGE_TYPE, &args![current]).u32() == 4
            || e.call(PACKAGE_FLAG_200000, &args![current]).bool())
    {
        outfit = true;
    }
    let evaluate = force != 0
        || e.vcall(p, 0x22c, &[]).u32() == 0
        || e.call(FTOL, &args![f64::from(hour)]).u32() != word(e, p, 0xb4);
    if evaluate {
        changed = e
            .call(
                LOW_PROCESS_CHECKFOR_NEW_PACKAGE,
                &args![p, actor, u32::from(force)],
            )
            .bool();
        let truncated = e.call(FTOL, &args![f64::from(hour)]).u32();
        e.mem.set_u32(p + 0xb4, truncated);
    }
    if !changed {
        return false;
    }
    e.mem.set_u16(p + 0x22a, 2);
    e.vcall(p, 0x4e4, &args![0u32]);
    e.vcall(p, 0x118, &args![0u32]);
    e.call(SET_PROCESS_FLAG_40, &args![p, 0u32]);
    e.vcall(p, 0x4ec, &args![0u32]);
    while !e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
        let item = node_item(e, p + 0x5c);
        if item != 0 {
            e.call(ACQUIRE_OBJECT_DELETE, &args![item, 1u32]);
        }
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            e.call(LIST_REMOVE, &args![p + 0x5c, local]);
        });
    }
    if outfit {
        reset_outfit(e, p, actor);
    }
    true
}

// Translated from 009180b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetupPathFinding` (Xbox PDB): an actor whose slot
/// 0x214 gives 4 or 9 runs its slot 0x418 and the result is false;
/// otherwise `Actor::SetPathfindingGoal_ov2` with the point `(x, y, z)`
/// (the stack words, passed by address), `cell`, `world`, -1.0
/// (`01012054`) and 0.
#[allow(clippy::too_many_arguments)]
pub fn middle_high_process_setup_path_finding(
    e: &mut Engine,
    _this: Ptr<MiddleHighProcess>,
    actor: u32,
    x: f32,
    y: f32,
    z: f32,
    cell: u32,
    world: u32,
) -> bool {
    let state = e.vcall(actor, 0x214, &[]).u32();
    if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
        e.vcall(actor, 0x418, &[]);
        return false;
    }
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.with_stack(12, |e, point| {
        e.mem.set_u32(point.addr(), x.to_bits());
        e.mem.set_u32(point.addr() + 4, y.to_bits());
        e.mem.set_u32(point.addr() + 8, z.to_bits());
        e.call(
            ACTOR_SET_PATHFINDING_GOAL,
            &args![actor, point, cell, world, minus_one, 0u32],
        )
        .bool()
    })
}

// Translated from 00918120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessPursue` (Xbox PDB): one step of pursuing
/// the target (+0x40, set through slot 0x8C when missing). Ends the move
/// against the player when flag 8 of its byte +0x680 is set. Under a
/// `TrespassPackage` a target whose slot 0x448 answers false ends the
/// procedure (slot 0x288 with `(target, 2)`). Farther than 200 units it
/// follows instead (slot 0x298 with `(actor, 0, follow_word, 1)`). A
/// `HighProcess` actor whose slot 0x4BC gives 0 gets `008bb520` with the
/// target's position. Under a trespass package: an `Actor` target whose
/// slot 0x448 answers false ends with 2; when the package's +0x94 is 0,
/// or its +0x84 exceeds +0x94 and its float +0x80 is not positive, a
/// target gets `008c0ec0`; with +0x80 not positive the procedure advances;
/// else +0x80 is lowered by the seconds passed (`0084d030`, `008db4c0`),
/// and once more past the setting `011cd2c4`. Then, when the actor is alarmed and the target is an
/// `Actor`: the player gets `005c1a00` (sleeping), an activation or combat;
/// another actor is attacked (slot 0x33C, `EnterCombat`).
pub fn middle_high_process_process_pursue(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    _unused_2: u32,
    _unused_3: u32,
    follow_word: u32,
) {
    let p = this.addr();
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x8c, &args![actor]);
    }
    let player = e.global::<u32>(PLAYER);
    if word(e, p, 0x40) == player && e.call(PLAYER_FLAGS_680, &args![player, 8u32]).bool() {
        e.vcall(p, 0x294, &args![actor]);
        return;
    }
    let running = e.vcall(p, 0x27c, &[]).u32();
    let trespass = e
        .call(
            DYNAMIC_CAST,
            &args![running, 0u32, TYPE_TES_PACKAGE, TYPE_TRESPASS_PACKAGE, 0u32],
        )
        .u32();
    let target = word(e, p, 0x40);
    if trespass != 0 && target != 0 && !e.vcall(target, 0x448, &[]).bool() {
        e.vcall(p, 0x288, &args![target, 2u32]);
        return;
    }
    let current = word(e, p, 0x40);
    let distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, current, 0u32, 0u32],
        )
        .f64();
    if distance > e.global::<f64>(PURSUE_FOLLOW_DISTANCE) {
        e.vcall(p, 0x298, &args![actor, 0u32, follow_word, 1u32]);
        return;
    }
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        e.vcall(p, 0x294, &args![actor]);
    }
    if word(e, p, 0x40) != 0 {
        let process = e.call(WORD_68, &args![actor]).u32();
        let high = e
            .call(
                DYNAMIC_CAST,
                &args![process, 0u32, TYPE_BASE_PROCESS, TYPE_HIGH_PROCESS, 0u32],
            )
            .u32();
        if high != 0 && e.call(WORD_68, &args![actor]).u32() != 0 {
            let process = e.call(WORD_68, &args![actor]).u32();
            if e.vcall(process, 0x4bc, &[]).u32() == 0 {
                let target = word(e, p, 0x40);
                let at = e.vcall(target, 0x1f4, &[]).u32();
                let (x, y, z) = (word(e, at, 0), word(e, at, 4), word(e, at, 8));
                e.call(ACTOR_CALL_008BB520, &args![actor, x, y, z, 0u32]);
            }
        }
    }
    if trespass != 0 {
        let current = word(e, p, 0x40);
        let character = e
            .call(
                DYNAMIC_CAST,
                &args![current, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
            )
            .u32();
        if character != 0 && !e.vcall(character, 0x448, &[]).bool() {
            e.vcall(p, 0x288, &args![actor, 2u32]);
            return;
        }
        let zero = e.global::<f64>(ZERO_DOUBLE);
        let mut warn = true;
        if fn_009185e0(e, trespass) != 0 {
            let warnings = e.call(PACKAGE_WORD_84, &args![trespass]).i32();
            warn = warnings > fn_009185e0(e, trespass) as i32
                && e.call(PACKAGE_FLOAT_80, &args![trespass]).f64() <= zero;
        }
        if warn && target != 0 {
            e.call(ACTOR_CALL_008C0EC0, &args![target, actor, 0u32, u32::MAX]);
            return;
        }
        if e.call(PACKAGE_FLOAT_80, &args![trespass]).f64() <= zero {
            e.vcall(p, 0x288, &args![actor, 1u32]);
            return;
        }
        let factor = e.global::<f64>(MINUS_ONE_DOUBLE);
        let step = (e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64() * factor) as f32;
        e.call(PACKAGE_ADD_FLOAT_80, &args![trespass, step.to_bits()]);
        let time = e.call(PACKAGE_FLOAT_80, &args![trespass]).f64();
        let cap = e
            .call(FLOAT_SETTING_VALUE, &args![PURSUE_TIME_SETTING])
            .u32();
        if f64::from(e.mem.f32(cap)) < time {
            let again = (e.call(PACKAGE_FLOAT_80, &args![trespass]).f64() * factor) as f32;
            e.call(PACKAGE_ADD_FLOAT_80, &args![trespass, again.to_bits()]);
        }
    }
    let current = word(e, p, 0x40);
    let as_character = e
        .call(
            DYNAMIC_CAST,
            &args![current, 0u32, TYPE_TES_OBJECT_REFR, TYPE_CHARACTER, 0u32],
        )
        .u32();
    let current = word(e, p, 0x40);
    let as_actor = e
        .call(
            DYNAMIC_CAST,
            &args![current, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if !e.call(ACTOR_IS_ALARMED, &args![actor]).bool() || as_actor == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    if as_character == player {
        if word(e, p, 0x40) == player
            && e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool()
            && byte(e, player, 0x20c) == 0
        {
            e.call(PLAYER_SET_654, &args![player, 0u32, 1u32]);
            return;
        }
        let attack =
            if !e.vcall(p, 0x2d8, &[]).bool() && !e.call(ACTOR_BYTE_7C4, &args![player]).bool() {
                if e.call(REFERENCE_ACTIVATE, &args![player, actor, 0u32, 0u32, 1u32])
                    .bool()
                {
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                }
                false
            } else {
                e.call(ACTOR_BYTE_7C4, &args![player]).bool()
            };
        if attack {
            let player = e.global::<u32>(PLAYER);
            e.vcall(
                p,
                0x33c,
                &args![
                    actor, player, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32
                ],
            );
        }
    } else {
        e.vcall(p, 0x288, &args![actor, 2u32]);
        e.vcall(
            p,
            0x33c,
            &args![
                actor, as_actor, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32
            ],
        );
    }
}

// Translated from 009185e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the word at +0x94 of an object (`ProcessPursue`
/// reads it from a `TrespassPackage`).
pub fn fn_009185e0(e: &mut Engine, this: u32) -> u32 {
    word(e, this, 0x94)
}

/// The flee distance of a package: the integer behind its +0x30 when it
/// has one, else its radius (`00676280`, unsigned), as a float.
fn flee_distance(e: &mut Engine, package: u32, actor: u32) -> f32 {
    if e.call(PACKAGE_WORD_30, &args![package]).u32() != 0 {
        let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
        e.call(ITEM_CHANGE_FORM, &args![data]).i32() as f32
    } else {
        let radius = e.call(PACKAGE_RADIUS, &args![package, actor]).u32();
        f64::from(radius) as f32
    }
}

// Translated from 00918600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessFleeNonCombat` (Xbox PDB): sets the package
/// target (slot 0x8C, when missing) and location (slot 0x90 with
/// `(actor, 0)`). A target nearer than the flee distance makes the actor
/// flee (its slot 0x410 with `(actor, 1, 1, 1, 0, 0, -1.0, -1.0)`) and,
/// under a `FleePackage`, an actor target is added to its avoided
/// references. Without a target and with the package's slot 0x13C false,
/// the actor paths to the package location (slot 0x824) when done
/// pathing, else slot 0x350 runs with `(actor, 0x201, 1)`; otherwise a
/// moving actor ends its move.
pub fn middle_high_process_process_flee_non_combat(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x8c, &args![actor]);
    }
    e.vcall(p, 0x90, &args![actor, 0u32]);
    let distance = flee_distance(e, running, actor);
    if word(e, p, 0x40) != 0 {
        let target = word(e, p, 0x40);
        let between = e
            .call(
                REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                &args![actor, target, 0u32, 0u32],
            )
            .f64();
        if f64::from(distance) < between {
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            let minus_one_again = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            e.vcall(
                actor,
                0x410,
                &args![
                    actor,
                    1u32,
                    1u32,
                    1u32,
                    0u32,
                    0u32,
                    minus_one_again,
                    minus_one
                ],
            );
            let running = e.vcall(p, 0x27c, &[]).u32();
            let flee = e
                .call(
                    DYNAMIC_CAST,
                    &args![running, 0u32, TYPE_TES_PACKAGE, TYPE_FLEE_PACKAGE, 0u32],
                )
                .u32();
            let target = word(e, p, 0x40);
            let avoided = e
                .call(
                    DYNAMIC_CAST,
                    &args![target, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
                )
                .u32();
            if flee != 0 && avoided != 0 {
                e.call(FLEE_ADD_AVOIDED_REF, &args![flee, avoided]);
            }
            return;
        }
    }
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    if word(e, p, 0x40) == 0
        && !e
            .vcall(running, 0x13c, &args![actor, 0u32, minus_one, 0u32])
            .bool()
    {
        let cell = e
            .call(PACKAGE_GET_LOCATION_CELL, &args![running, actor])
            .u32();
        e.with_stack(12, |e, point| {
            e.call(PACKAGE_GET_LOCATION_COORD, &args![running, point, actor]);
            if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
                let at = point.addr();
                let (x, y, z) = (word(e, at, 0), word(e, at, 4), word(e, at, 8));
                e.vcall(p, 0x824, &args![actor, x, y, z, cell, world]);
            } else {
                e.vcall(p, 0x350, &args![actor, 0x201u32, 1u32]);
            }
        });
    } else if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        e.vcall(p, 0x294, &args![actor]);
    }
}

/// The end of a flee step: the package's byte +0x94 set (`00810530` with
/// 1), the seconds passed added to its +0x90 (`00898250`), and the
/// procedure advanced (slot 0x288) once the setting `011cd348` is not
/// above `start` (the package's +0x90 at the start of the step).
fn flee_tick(e: &mut Engine, p: u32, actor: u32, flee: u32, start: f32) {
    e.call(FLEE_SET_BYTE_94, &args![flee, 1u32]);
    let passed = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64() as f32;
    e.call(FLEE_ADD_FLOAT_90, &args![flee, passed.to_bits()]);
    let limit = e.call(FLOAT_SETTING_VALUE, &args![FLEE_TIME_SETTING]).u32();
    let limit = e.mem.f32(limit);
    // FCOMPP with the setting on top; JP skips when it is greater or
    // unordered.
    if f64::from(limit) <= f64::from(start) {
        e.vcall(p, 0x288, &args![actor, 1u32]);
    }
}

// Translated from 00918860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessFlee` (Xbox PDB): a package spell (slot
/// 0x4C) is cast on the actor once loaded (`MagicCaster::CastSpell` from
/// the actor's +0x88 at its +0x94, then unloaded and cleared through slot
/// 0x50), or preloaded and the step ends. Under a `FleePackage` with a
/// target reference: `009f1830` updates it; without an avoided reference
/// nearby the flee time runs ([`flee_tick`]); an actor whose slot 0x214
/// gives 4 or 9 runs slot 0x418; otherwise, in a cell, the time runs down
/// and, nearer than the setting `011ce570` (cells with `00425fd0`) or
/// `011cf998` and done pathing, the actor flees: `+0xA4` counts down, and
/// without the package's door (+0xA4) a `PathingRequestFlee` (heading,
/// radius, `009f1440`, 1000.0) is handed to `Actor::SetPathfindingFlee`,
/// else it paths to the door (slot 0x824). Still pathing, only the unused
/// elapsed-hours calls run; done, the flee time runs.
// Translation incomplete: 00918860 - the C++ exception-unwinding frame (FS:[0]) that destroys the local PathingRequestFlee and PathingLocation when a call throws is not translated.
pub fn middle_high_process_process_flee(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    let flee = e
        .call(
            DYNAMIC_CAST,
            &args![running, 0u32, TYPE_TES_PACKAGE, TYPE_FLEE_PACKAGE, 0u32],
        )
        .u32();
    let mut target = 0u32;
    if e.vcall(p, 0x4c, &[]).u32() != 0 {
        let spell = e.vcall(p, 0x4c, &[]).u32();
        if e.call(MAGIC_ITEM_IS_LOADED, &args![spell]).bool() {
            let magic_target = if actor == 0 { 0 } else { actor + 0x94 };
            let spell = e.vcall(p, 0x4c, &[]).u32();
            e.call(
                MAGIC_CASTER_CAST_SPELL,
                &args![actor + 0x88, spell, magic_target, 0u32],
            );
            let spell = e.vcall(p, 0x4c, &[]).u32();
            e.call(MAGIC_ITEM_UNLOAD, &args![spell, 0u32]);
            e.vcall(p, 0x50, &args![0u32]);
        } else {
            let spell = e.vcall(p, 0x4c, &[]).u32();
            if !e.call(MAGIC_ITEM_IS_LOADING, &args![spell]).bool() {
                let spell = e.vcall(p, 0x4c, &[]).u32();
                e.call(MAGIC_ITEM_PRELOAD, &args![spell, 0u32]);
            }
            return;
        }
    }
    if flee == 0 {
        return;
    }
    e.call(FLEE_UPDATE, &args![flee, p]);
    if e.call(PACKAGE_WORD_30, &args![flee]).u32() != 0 {
        let data = e.call(PACKAGE_WORD_30, &args![flee]).u32();
        if e.call(PACKAGE_TARGET_GET_REFERENCE, &args![data]).u32() != 0 {
            let data = e.call(PACKAGE_WORD_30, &args![flee]).u32();
            target = e.call(PACKAGE_TARGET_GET_REFERENCE, &args![data]).u32();
        }
    }
    if target == 0 {
        return;
    }
    let start = e.call(FLEE_FLOAT_90, &args![flee]).f64() as f32;
    let avoided = e
        .call(FLEE_GET_CLOSEST_AVOIDED_REF, &args![flee, actor])
        .u32();
    if avoided == 0 {
        flee_tick(e, p, actor, flee, start);
        return;
    }
    let state = e.vcall(actor, 0x214, &[]).u32();
    if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
        e.vcall(actor, 0x418, &[]);
        return;
    }
    let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    if cell == 0 {
        return;
    }
    let factor = e.global::<f64>(MINUS_ONE_DOUBLE);
    let back = (e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64() * factor) as f32;
    e.call(FLEE_ADD_FLOAT_90, &args![flee, back.to_bits()]);
    let distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, avoided, 0u32, 0u32],
        )
        .f64() as f32;
    let setting = if e.call(CELL_FLAG, &args![cell]).bool() {
        FLEE_NEAR_CELL_SETTING
    } else {
        FLEE_NEAR_SETTING
    };
    let value = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    let near = x87_quiet(e.mem.f32(value));
    if f64::from(near) > f64::from(distance)
        && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
    {
        let door = e.call(FLEE_DOOR, &args![flee]).u32();
        let countdown = f64::from(e.mem.f32(p + 0xa4));
        if countdown > 0.0 || countdown.is_nan() {
            let passed = e.call(FLOAT_C, &args![GLOBAL_TIME_DATA]).f64();
            e.mem.set_f32(p + 0xa4, (countdown - passed) as f32);
        }
        if door == 0 {
            e.call(FLEE_FIND_TELEPORT_DOOR, &args![flee, actor, avoided]);
            e.with_stack(0xbc, |e, request| {
                e.call(PATHING_REQUEST_FLEE_CONSTRUCT, &args![request]);
                e.call(PATHING_REQUEST_FROM_ACTOR, &args![request, actor]);
                let heading = e.vcall(actor, 0x2bc, &args![0u32]).f64() as f32;
                e.call(PATHING_REQUEST_SET_90, &args![request, heading.to_bits()]);
                if e.vcall(actor, 0x21c, &[]).bool() {
                    let object = e.call(ACTOR_EXTRA_OBJECT, &args![actor]).u32() + 0x30;
                    let flag = e.vcall(object, 0x28, &[]).u8();
                    e.call(PATHING_REQUEST_SET_98, &args![request, u32::from(flag)]);
                }
                let radius = e.call(PACKAGE_RADIUS, &args![flee, actor]).u32();
                let radius = f64::from(radius) as f32;
                e.call(PATHING_REQUEST_SET_74, &args![request, radius.to_bits()]);
                let mode = e.call(FLEE_PATH_MODE, &args![flee]).u32();
                e.call(PATHING_REQUEST_006D61E0, &args![request, mode]);
                let range = x87_quiet_bits(e.global::<u32>(FLEE_PATH_RANGE));
                e.call(PATHING_REQUEST_SET_B0, &args![request, range]);
                if door != 0 {
                    e.with_stack(0x34, |e, location| {
                        let built = e
                            .call(PATHING_LOCATION_CONSTRUCT, &args![location, door])
                            .u32();
                        e.call(PATHING_REQUEST_SET_LOCATION, &args![request, built]);
                        e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
                    });
                } else {
                    e.call(PATHING_REQUEST_008DF000, &args![request]);
                }
                e.call(ACTOR_SET_PATHFINDING_FLEE, &args![actor, request]);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            });
        } else {
            let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![door]).u32();
            let door_cell = e.call(REFERENCE_CELL, &args![door]).u32();
            let at = e.vcall(door, 0x1f4, &[]).u32();
            let (x, y, z) = (word(e, at, 0), word(e, at, 4), word(e, at, 8));
            if !e
                .vcall(p, 0x824, &args![actor, x, y, z, door_cell, world])
                .bool()
            {
                return;
            }
        }
        e.call(FLEE_SET_BYTE_94, &args![flee, 0u32]);
        let undo = (e.call(FLEE_FLOAT_90, &args![flee]).f64() * factor) as f32;
        e.call(FLEE_ADD_FLOAT_90, &args![flee, undo.to_bits()]);
    }
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        unused_hours_calls(e, p);
    } else {
        flee_tick(e, p, actor, flee, start);
    }
}

/// Copies the three floats slot 0x1F4 of `reference` points at into the
/// process's last-seen position (+0xFC).
fn store_last_seen(e: &mut Engine, p: u32, reference: u32) {
    let at = e.vcall(reference, 0x1f4, &[]).u32();
    for offset in [0u32, 4, 8] {
        let value = word(e, at, offset);
        e.mem.set_u32(p + 0xfc + offset, value);
    }
}

// Translated from 00918ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessFollow` (Xbox PDB): one step of following
/// the target (+0x40). A package with flag 0x10000 stops when the player
/// test `00962720` answers true or the player's count (`00962620`)
/// reached the integer setting `011cdad0`, else `00674f30` runs with 0.
/// A missing or flagged (form flag 0x20) target is set again (slot 0x8C)
/// and its position remembered (+0xFC); without one the procedure
/// advances when `advance`. A follow package (type 1) whose location
/// (type below 2) the package's slot 0x13C accepts, ends: the procedure
/// advances when `advance`, and a created package is dropped (slot 0x214
/// or 0x234) before `CheckforNewPackage`. Otherwise the actor paths
/// (`Actor::SetMoveMode` with `move_mode`, slot 0x824) when the target
/// moved more than the setting `011cd950` since last seen, or is beyond
/// the package radius while the actor is done pathing. Within the radius
/// a moving actor stops (`Actor::EndMovement`). The fifth word is not
/// read.
pub fn middle_high_process_process_follow(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    advance: u8,
    move_mode: u32,
    _unused_4: u32,
) {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if running != 0 && e.call(PACKAGE_FLAG_10000, &args![running]).bool() {
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_TEST_00962720, &args![player, actor]).bool() {
            return;
        }
        let player = e.global::<u32>(PLAYER);
        let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
        let limit = e
            .call(SETTING_VALUE, &args![FOLLOW_COUNT_LIMIT_SETTING])
            .u32();
        if count >= e.mem.i32(limit) {
            return;
        }
        e.call(PACKAGE_CALL_00674F30, &args![running, 0u32]);
    }
    let reset = if word(e, p, 0x40) == 0 {
        true
    } else {
        let target = word(e, p, 0x40);
        e.call(FORM_FLAG_20, &args![target]).bool()
    };
    if reset {
        e.vcall(p, 0x8c, &args![actor]);
        if word(e, p, 0x40) != 0 {
            let target = word(e, p, 0x40);
            store_last_seen(e, p, target);
        }
    }
    if word(e, p, 0x40) == 0 {
        if advance != 0 {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        return;
    }
    let target = word(e, p, 0x40);
    let mut distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, target, 0u32, 0u32],
        )
        .f64() as f32;
    e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]);
    let target = word(e, p, 0x40);
    e.call(REFERENCE_GET_WORLD_SPACE, &args![target]);
    let radius = e
        .call(PACKAGE_GET_RADIUS_TO_TARGET, &args![running, actor, 0u32])
        .f64() as f32;
    let saved = word(e, p, 0x40);
    let mut finished = saved == 0;
    if !finished
        && e.call(PACKAGE_TYPE, &args![running]).u32() == 1
        && e.call(PACKAGE_LOCATION, &args![running]).u32() != 0
    {
        let location = e.call(PACKAGE_LOCATION, &args![running]).u32();
        if e.call(LOCATION_GET_TYPE, &args![location]).i32() < 2 {
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            finished = e
                .vcall(running, 0x13c, &args![actor, 0u32, minus_one, 0u32])
                .bool();
        }
    }
    if finished {
        if advance != 0 {
            e.vcall(p, 0x288, &args![actor, 1u32]);
            if e.call(PACKAGE_GET_IS_CREATED, &args![running]).bool() {
                if e.vcall(p, 0x20c, &[]).u32() != 0 {
                    e.vcall(p, 0x214, &[]);
                } else {
                    e.vcall(p, 0x234, &[]);
                }
                e.vcall(p, 0x24, &args![actor, 0u32]);
                return;
            }
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
        }
    } else {
        let target = word(e, p, 0x40);
        let at = e.vcall(target, 0x1f4, &[]).u32();
        let moved = e.with_stack(12, |e, offset| {
            let result = e.call(POINT_SUBTRACT, &args![p + 0xfc, offset, at]).u32();
            e.call(POINT_LENGTH, &args![result]).f64() as f32
        });
        let value = e
            .call(FLOAT_SETTING_VALUE, &args![FOLLOW_MOVED_SETTING])
            .u32();
        let limit = e.mem.f32(value);
        let go = if f64::from(limit) < f64::from(moved) {
            true
        } else {
            f64::from(radius) < f64::from(distance)
                && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
        };
        if go {
            e.call(ACTOR_SET_MOVE_MODE, &args![actor, move_mode]);
            if !setup_path_to(e, p, actor, |e| {
                let target = word(e, p, 0x40);
                e.vcall(target, 0x1f4, &[]).u32()
            }) {
                return;
            }
            let target = word(e, p, 0x40);
            store_last_seen(e, p, target);
        }
        if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            unused_hours_calls(e, p);
        }
    }
    distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, saved, 0u32, 0u32],
        )
        .f64() as f32;
    if f64::from(radius) > f64::from(distance)
        && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
    {
        e.call(ACTOR_END_MOVEMENT, &args![actor]);
    }
}

/// Slot 0x824 (`SetupPathFinding`) toward `reference`: its worldspace,
/// its cell and then its position (slot 0x1F4), as the pushes run.
fn setup_path_to_reference(e: &mut Engine, p: u32, actor: u32, reference: u32) -> bool {
    let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    let at = e.vcall(reference, 0x1f4, &[]).u32();
    let (x, y, z) = (word(e, at, 0), word(e, at, 4), word(e, at, 8));
    e.vcall(p, 0x824, &args![actor, x, y, z, cell, world])
        .bool()
}

/// Slot 0x824 toward the package location of `package` (its world, cell
/// and coordinates, in that order).
fn setup_path_to_location(e: &mut Engine, p: u32, actor: u32, package: u32) -> bool {
    let world = e
        .call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
        .u32();
    let cell = e
        .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
        .u32();
    e.with_stack(12, |e, out| {
        let coord = e
            .call(PACKAGE_GET_LOCATION_COORD, &args![package, out, actor])
            .u32();
        let (x, y, z) = (word(e, coord, 0), word(e, coord, 4), word(e, coord, 8));
        e.vcall(p, 0x824, &args![actor, x, y, z, cell, world])
            .bool()
    })
}

/// `TESDataHandler::EnumReferencesCloseToPoint` around `actor` with the
/// radius at `01030020` (5000.0) and `callback`.
fn enumerate_near_actor(e: &mut Engine, actor: u32, callback: u32) {
    let radius = x87_quiet_bits(e.global::<u32>(TRAVEL_SEARCH_RADIUS));
    let first = e.vcall(actor, 0x1f4, &[]).u32();
    let radius_again = x87_quiet_bits(e.global::<u32>(TRAVEL_SEARCH_RADIUS));
    let second = e.vcall(actor, 0x1f4, &[]).u32();
    let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(
        ENUM_REFERENCES_CLOSE_TO_POINT,
        &args![
            handler,
            cell,
            second,
            radius_again,
            first,
            radius,
            callback,
            actor
        ],
    );
}

/// The package's slot 0x13C with `(actor, 0, -1.0, 0)`.
fn package_reached(e: &mut Engine, package: u32, actor: u32) -> bool {
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32])
        .bool()
}

/// The sitting part of an arrived travel package while the player sleeps
/// or rests: the actor is placed on the furniture's marker (heading, the
/// marker's target offset rotated by the heading, the marker taken), its
/// sit state set (slot 0x4C0: 6 then 9 to sleep, 1 then 4 to sit) once
/// the dynamic idle is loaded (slot 0x4D8).
fn travel_place_in_furniture(e: &mut Engine, p: u32, actor: u32, running: u32, location: u32) {
    let base = e.call(REFERENCE_BASE, &args![location]).u32();
    let furniture = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_FURNITURE, 0u32],
        )
        .u32();
    let mark = e.vcall(p, 0x4d4, &[]).u32();
    let heading = e.call(MARK_HEADING, &args![mark]).f64() as f32;
    e.vcall(actor, 0x2c4, &args![heading.to_bits()]);
    e.with_stack(0x3c, |e, locals| {
        let offset = locals.addr();
        let rotation = offset + 0xc;
        let scratch = offset + 0x30;
        let scale = e.call(REFERENCE_GET_SCALE, &args![actor]).f64() as f32;
        let markers = e.call(MARK_BYTE_E, &args![mark]).u32();
        e.call(
            FURNITURE_GET_MARKER_TARGET_OFFSET,
            &args![furniture, offset, markers, scale.to_bits()],
        );
        e.call(IDENTITY, &args![rotation]);
        let heading = e.call(MARK_HEADING, &args![mark]).f64() as f32;
        e.call(ROTATION_FROM_HEADING, &args![rotation, heading.to_bits()]);
        let rotated = e
            .call(ROTATE_POINT, &args![rotation, scratch, offset])
            .u32();
        for index in [0u32, 4, 8] {
            let value = word(e, rotated, index);
            e.mem.set_u32(offset + index, value);
        }
        let index = u32::from(byte(e, p, 0x144));
        e.call(REFERENCE_SET_MARKER_USED, &args![location, index, 1u32]);
        let point = e.call(IDENTITY, &args![mark]).u32();
        let sum = e.call(POINT_ADD, &args![point, scratch, offset]).u32();
        e.vcall(actor, 0x2a8, &args![sum]);
    });
    let markers = e.call(MARK_BYTE_E, &args![mark]).u32();
    let angle = e
        .call(FURNITURE_MARKER_ANGLE, &args![furniture, markers])
        .f64() as f32;
    e.call(MOBILE_TURN_BY, &args![actor, angle.to_bits()]);
    let sleep = e.call(PACKAGE_TYPE, &args![running]).u32() == 4;
    let furniture_ref = word(e, p, 0x140);
    let index = u32::from(byte(e, p, 0x144));
    if sleep {
        e.vcall(p, 0x4c0, &args![actor, 6u32, furniture_ref, index]);
        let process = e.call(WORD_68, &args![actor]).u32();
        e.vcall(process, 0x284, &args![1u32]);
        let process = e.call(WORD_68, &args![actor]).u32();
        e.vcall(process, 0xc4, &args![actor, 0u32]);
    } else {
        e.vcall(p, 0x4c0, &args![actor, 1u32, furniture_ref, index]);
    }
    if !e.vcall(p, 0x4d8, &args![actor]).bool() {
        return;
    }
    let furniture_ref = word(e, p, 0x140);
    let index = u32::from(byte(e, p, 0x144));
    if sleep {
        let face = e.call(ACTOR_GET_FACE_ANIMATION_DATA, &args![actor]).u32();
        if face != 0 {
            e.vcall(face, 0xd8, &args![1u32, 0u32]);
        }
        let furniture_ref = word(e, p, 0x140);
        let index = u32::from(byte(e, p, 0x144));
        e.vcall(p, 0x4c0, &args![actor, 9u32, furniture_ref, index]);
    } else {
        e.vcall(p, 0x4c0, &args![actor, 4u32, furniture_ref, index]);
    }
}

// Translated from 00919320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessTravel` (Xbox PDB): one step of travelling
/// to the package location. The actor paths there (`Actor::SetMoveMode`
/// with `move_mode`, slot 0x824) when `check_location` and the package's
/// slot 0x13C says it is not there, or when done pathing, not there and
/// `00915ef0` answers false. The location reference is the package's (or
/// +0x44 without a run-once package); type 5 uses radius 20.0. Furniture
/// at the location becomes the process's furniture; an actor whose slot
/// 0x214 is non-zero has arrived, one whose marker is taken while still
/// pathing gives up (slot 0x84, procedure advanced). On arrival: searches
/// near the actor (`EnumReferencesCloseToPoint` with `0090d480` once,
/// locking the location, and with `0090d570`) as the package asks
/// (`00670f40`, `00670f60`), unequips the weapon once for packages with
/// flag 0x200000, then sits in the furniture (states 4 and 9 end the step;
/// a sleeping player has the actor placed at once, else it aligns), or at
/// `011ca248` or a location of type 3 takes its heading (slot 0x2C4);
/// ends the move and advances. Not arrived: furniture's closest free
/// marker is pathed to (or the procedure jumps to its last step), else an
/// actor at the location is followed when it moved more than the setting
/// `011cd950` or is out of the radius. The fifth word is not read.
pub fn middle_high_process_process_travel(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    check_location: u8,
    advance: u8,
    move_mode: u32,
    _unused_4: u32,
) {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    let go = if check_location != 0 && !package_reached(e, running, actor) {
        true
    } else {
        e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
            && !package_reached(e, running, actor)
            && fn_00915ef0(e, this, actor) == 0
    };
    if go {
        e.call(ACTOR_SET_MOVE_MODE, &args![actor, move_mode]);
        if !setup_path_to_location(e, p, actor, running) {
            return;
        }
    }
    let distance = e.with_stack(12, |e, out| {
        let coord = e
            .call(PACKAGE_GET_LOCATION_COORD, &args![running, out, actor])
            .u32();
        e.call(REFERENCE_DISTANCE_TO, &args![actor, coord]).f64() as f32
    });
    let mut radius = e
        .call(PACKAGE_GET_RADIUS_TO_LOCATION, &args![running, actor, 0u32])
        .f64() as f32;
    let mut location = 0u32;
    if e.call(PACKAGE_LOCATION, &args![running]).u32() != 0 {
        let data = e.call(PACKAGE_LOCATION, &args![running]).u32();
        location = e.call(LOCATION_GET_REFERENCE, &args![data]).u32();
    }
    if word(e, p, 0x44) != 0 && e.vcall(p, 0x20c, &[]).u32() == 0 {
        location = word(e, p, 0x44);
    }
    if e.call(PACKAGE_TYPE, &args![running]).u32() == 5 {
        radius = x87_quiet(e.global::<f32>(TRAVEL_SMALL_RADIUS));
    }
    let arrived = if location != 0
        && e.call(REFERENCE_IS_FURNITURE, &args![location]).bool()
        && e.call(PACKAGE_TYPE, &args![running]).u32() != 5
    {
        if word(e, p, 0x140) == 0 {
            e.mem.set_u32(p + 0x140, location);
            e.call(CLEAR_FURNITURE_MARKER, &args![p]);
        }
        if e.vcall(actor, 0x214, &[]).u32() != 0 {
            true
        } else {
            if e.vcall(actor, 0x214, &[]).u32() == 0 {
                let furniture = word(e, p, 0x140);
                let index = u32::from(byte(e, p, 0x144));
                if e.call(REFERENCE_GET_MARKER_USED, &args![furniture, index, 1u32])
                    .bool()
                    && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
                {
                    e.vcall(p, 0x84, &args![actor]);
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                    return;
                }
            }
            package_reached(e, running, actor)
        }
    } else {
        package_reached(e, running, actor)
    };
    if arrived || fn_00915ef0(e, this, actor) != 0 {
        if !e.vcall(p, 0x1bc, &[]).bool() {
            if e.call(PACKAGE_FLAG_20, &args![running]).bool() {
                enumerate_near_actor(e, actor, TRAVEL_SEARCH_CALLBACK_A);
            }
            e.vcall(p, 0x1c0, &args![1u32]);
        }
        if e.call(PACKAGE_FLAG_100, &args![running]).bool() {
            enumerate_near_actor(e, actor, TRAVEL_SEARCH_CALLBACK_B);
        }
        if running != 0
            && byte(e, p, 0x189) == 0
            && e.call(PACKAGE_FLAG_200000, &args![running]).bool()
        {
            e.mem.set_u8(p + 0x189, 1);
            let weapon = e.vcall(p, 0x148, &[]).u32();
            if weapon != 0 {
                let form = e.call(ITEM_CHANGE_FORM, &args![weapon]).u32();
                e.call(
                    ACTOR_QUEUE_UNEQUIP_OBJECT,
                    &args![actor, form, 1u32, 0u32, 0u32, 0u32, 1u32],
                );
            }
        }
        if location != 0 && e.call(REFERENCE_IS_FURNITURE, &args![location]).bool() {
            let state = e.vcall(actor, 0x214, &[]).u32();
            if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
                if advance != 0 {
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                }
                e.vcall(p, 0x294, &args![actor]);
                return;
            }
            let player = e.global::<u32>(PLAYER);
            if e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
                travel_place_in_furniture(e, p, actor, running, location);
            } else {
                e.call(ALIGN_TO_FURNITURE, &args![p, actor]);
            }
            return;
        }
        let faces = if location != 0
            && e.call(REFERENCE_BASE, &args![location]).u32() == e.global::<u32>(ACTIVATE_FORM_A)
        {
            true
        } else if e.call(PACKAGE_LOCATION, &args![running]).u32() != 0 {
            let data = e.call(PACKAGE_LOCATION, &args![running]).u32();
            e.call(LOCATION_GET_TYPE, &args![data]).i32() == 3
        } else {
            false
        };
        if faces && fn_00915ef0(e, this, actor) == 0 && e.vcall(actor, 0x214, &[]).u32() == 0 {
            let heading = if location != 0 {
                let data = e.call(REFERENCE_PLUS_24, &args![location]).u32();
                x87_quiet(e.mem.f32(data + 8))
            } else {
                e.with_stack(12, |e, out| {
                    let data = e.vcall(actor, 0x16c, &args![out]).u32();
                    x87_quiet(e.mem.f32(data + 8))
                })
            };
            e.vcall(actor, 0x2c4, &args![x87_quiet(heading).to_bits()]);
        }
        e.vcall(p, 0x294, &args![actor]);
        if advance != 0
            && (fn_00915ef0(e, this, actor) == 0 || !e.call(PACKAGE_FLAG_4, &args![running]).bool())
        {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        return;
    }
    if actor == 0 {
        return;
    }
    let state = e.vcall(actor, 0x214, &[]).u32();
    if state == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
        e.vcall(actor, 0x418, &[]);
        return;
    }
    if location != 0 && e.call(REFERENCE_IS_FURNITURE, &args![location]).bool() {
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        let marker = e.with_stack(16, |e, locals| {
            let point = locals.addr();
            let index = locals.addr() + 12;
            for offset in [0u32, 4, 8] {
                let value = word(e, at, offset);
                e.mem.set_u32(point + offset, value);
            }
            e.mem.set_u32(index, 0);
            if word(e, p, 0x140) == 0 {
                e.mem.set_u32(p + 0x140, location);
                e.call(CLEAR_FURNITURE_MARKER, &args![p]);
            }
            let furniture = word(e, p, 0x140);
            let found = e
                .call(
                    REFERENCE_GET_CLOSEST_FREE_MARKER,
                    &args![furniture, point, 1u32, 1u32, p + 0x148, index, 1u32],
                )
                .bool();
            found.then(|| e.mem.u8(index))
        });
        match marker {
            Some(index) => {
                let world = e
                    .call(PACKAGE_GET_LOCATION_WORLD, &args![running, actor])
                    .u32();
                let cell = e
                    .call(PACKAGE_GET_LOCATION_CELL, &args![running, actor])
                    .u32();
                let (x, y, z) = (word(e, p, 0x148), word(e, p, 0x14c), word(e, p, 0x150));
                if !e
                    .vcall(p, 0x824, &args![actor, x, y, z, cell, world])
                    .bool()
                {
                    return;
                }
                e.mem.set_u32(p + 0x140, location);
                e.mem.set_u8(p + 0x144, index);
            }
            None => {
                e.vcall(p, 0x294, &args![actor]);
                let procedure = e.call(PACKAGE_WORD_18, &args![running]).u32();
                let count = e.call(PROCEDURE_GET_COUNT, &args![procedure]).u32();
                e.vcall(p, 0x284, &args![count.wrapping_sub(1)]);
            }
        }
        return;
    }
    let mut destination = 0u32;
    if e.call(PACKAGE_LOCATION, &args![running]).u32() != 0 {
        let data = e.call(PACKAGE_LOCATION, &args![running]).u32();
        if e.call(LOCATION_GET_REFERENCE, &args![data]).u32() != 0 {
            let data = e.call(PACKAGE_LOCATION, &args![running]).u32();
            let reference = e.call(LOCATION_GET_REFERENCE, &args![data]).u32();
            destination = e
                .call(
                    DYNAMIC_CAST,
                    &args![reference, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
                )
                .u32();
        }
    }
    if destination == 0 {
        return;
    }
    let at = e.vcall(destination, 0x1f4, &[]).u32();
    let moved = e.with_stack(12, |e, offset| {
        let result = e.call(POINT_SUBTRACT, &args![p + 0xfc, offset, at]).u32();
        e.call(POINT_LENGTH, &args![result]).f64() as f32
    });
    let value = e
        .call(FLOAT_SETTING_VALUE, &args![FOLLOW_MOVED_SETTING])
        .u32();
    let limit = e.mem.f32(value);
    let follow = if f64::from(limit) < f64::from(moved) {
        true
    } else {
        f64::from(radius) < f64::from(distance)
            && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
    };
    if follow && setup_path_to_reference(e, p, actor, destination) {
        store_last_seen(e, p, destination);
    }
}

/// The escort-distance check of a follow-like procedure in
/// [`middle_high_process_update`] (procedure kind 13).
fn update_follow_target(e: &mut Engine, p: u32, actor: u32, running: u32) {
    e.call(PACKAGE_TYPE, &args![running]);
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x8c, &args![actor]);
    }
    let flagged = if word(e, p, 0x40) == 0 {
        true
    } else {
        let target = word(e, p, 0x40);
        e.call(FORM_FLAG_20, &args![target]).bool()
    };
    if flagged {
        e.vcall(p, 0x288, &args![actor, 1u32]);
    }
    if e.call(PACKAGE_FLAG_10000, &args![running]).bool() {
        let player = e.global::<u32>(PLAYER);
        if !e.call(PLAYER_TEST_00962720, &args![player, actor]).bool() {
            let player = e.global::<u32>(PLAYER);
            let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
            let limit = e
                .call(SETTING_VALUE, &args![FOLLOW_COUNT_LIMIT_SETTING])
                .u32();
            if count < e.mem.i32(limit) {
                e.call(PACKAGE_CALL_00674F30, &args![running, 0u32]);
            }
        }
        return;
    }
    let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32();
    if e.call(PACKAGE_TYPE, &args![current]).u32() == 1 {
        let package = e.vcall(p, 0x22c, &[]).u32();
        let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
        let reach = e.call(ITEM_CHANGE_FORM, &args![data]).i32() as f32;
        let target = word(e, p, 0x40);
        let distance = e
            .call(
                REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                &args![actor, target, 0u32, 0u32],
            )
            .f64();
        if f64::from(reach) >= distance {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        return;
    }
    let mut reach = 0.0f32;
    let data = e.call(PACKAGE_WORD_30, &args![running]).u32();
    let mut range = e.call(ITEM_CHANGE_FORM, &args![data]).i32();
    if range <= 0 {
        range = 200;
    }
    let zero = e.global::<f64>(ZERO_DOUBLE);
    let mut done = false;
    if f64::from(reach) == zero {
        let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
        if e.call(CELL_FLAG, &args![cell]).bool() {
            let value = e
                .call(FLOAT_SETTING_VALUE, &args![ESCORT_CELL_DISTANCE_SETTING])
                .u32();
            reach = x87_quiet(e.mem.f32(value));
            done = true;
        }
    }
    if !done && f64::from(reach) == zero {
        let value = e
            .call(FLOAT_SETTING_VALUE, &args![ESCORT_DISTANCE_FACTOR_SETTING])
            .u32();
        reach = (f64::from(e.mem.f32(value)) * f64::from(range)) as f32;
    }
    let target = word(e, p, 0x40);
    let distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, target, 0u32, 0u32],
        )
        .f64();
    let half = f64::from(reach) / e.global::<f64>(TWO_DOUBLE);
    if half >= distance {
        e.vcall(p, 0x288, &args![actor, 1u32]);
    }
}

/// The end-of-procedure block of [`middle_high_process_update`]
/// (procedure kind 54, also reached after kind 10). Returns true when
/// `Update` returns at once (without its last calls).
fn update_procedure_done(e: &mut Engine, p: u32, actor: u32, running: u32) -> bool {
    if !e.vcall(p, 0x5a4, &[]).bool()
        && !e.call(PACKAGE_GET_IS_CREATED, &args![running]).bool()
        && e.call(PACKAGE_IS_INTERRUPT, &args![running]).bool()
    {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
        e.call(SCRIPT_SET_ACTION_FLAG, &args![running, list, 0x400u32]);
        e.vcall(p, 0x5a0, &args![actor, running]);
        e.vcall(p, 0x5a8, &args![1u32]);
    }
    if e.call(PACKAGE_WORD_18, &args![running]).u32() == 0x26 {
        let data = e.call(PACKAGE_PLUS_38, &args![running]).u32();
        if data != 0 {
            let count = e.call(WORD_4, &args![data]).i32();
            let zero = e.global::<f64>(ZERO_DOUBLE);
            if f64::from(count) > zero {
                return true;
            }
        }
        let patrol = e.call(PACKAGE_GET_PATROL_DATA, &args![running]).u32();
        if patrol == 0 || e.call(PATROL_BYTE_4, &args![patrol]).bool() {
            return true;
        }
        e.vcall(p, 0x118, &args![1u32]);
        e.vcall(p, 0x524, &args![actor]);
        e.vcall(p, 0x234, &[]);
        e.vcall(p, 0x24, &args![actor, 1u32]);
        return true;
    }
    let kind = e.call(PACKAGE_TYPE, &args![running]).u32();
    if kind == 3 || e.call(PACKAGE_TYPE, &args![running]).u32() == 4 {
        e.vcall(p, 0x288, &args![actor, u32::MAX]);
        return true;
    }
    if e.call(PACKAGE_TYPE, &args![running]).u32() == 6 && !package_reached(e, running, actor) {
        e.vcall(p, 0x288, &args![actor, u32::MAX]);
        return true;
    }
    if running != 0 && e.call(PACKAGE_GET_ONCE_PER_DAY, &args![running]).bool() {
        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
        e.vcall(actor, 0x28c, &args![running, u32::from(day)]);
    }
    e.call(INTERFACE_GET_TARGET_REFR, &[]);
    let data = e.call(PACKAGE_PLUS_38, &args![running]).u32();
    if e.call(WORD_4, &args![data]).u32() != 0 {
        return false;
    }
    e.vcall(p, 0x12c, &args![0u32]);
    if e.vcall(p, 0x20c, &[]).u32() != 0 {
        e.vcall(p, 0x214, &[]);
    } else {
        let current = e.vcall(p, 0x22c, &[]).u32();
        if e.call(PACKAGE_GET_IS_CREATED, &args![current]).bool() {
            e.vcall(p, 0x234, &[]);
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
        }
    }
    let acquire = word(e, p, 0x64);
    if acquire != 0 {
        e.call(ACQUIRE_OBJECT_DELETE, &args![acquire, 1u32]);
    }
    e.mem.set_u32(p + 0x64, 0);
    while !e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
        let item = node_item(e, p + 0x5c);
        if item != 0 {
            e.call(ACQUIRE_OBJECT_DELETE, &args![item, 1u32]);
        }
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            e.call(LIST_REMOVE, &args![p + 0x5c, local]);
        });
    }
    e.mem.set_u32(p + 0x44, 0);
    e.call(LIST_CLEAR, &args![p + 0x6c]);
    false
}

// Translated from 00919f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Update` (Xbox PDB): the per-frame AI step of an
/// actor, between two calls of the empty function on `011e0460`. Without
/// an actor nothing runs; an actor whose slot 0x1D0 gives 0, with form
/// flag 0x20 or 0x800, or not in a cell with `00450fb0` only records
/// the time (slot 0x28) and returns. A non-actor (slot 0x100) only has its
/// process record the time. Otherwise: byte +0x18A is updated through
/// `0080ced0`; `CheckforNewPackage` (slot 0x24) runs, then the current data
/// is cleared (slot 0x524) for a new package or a moving actor without
/// one; the target is reset (slot 0x82C); a new package re-runs
/// `008a8f60` and the location (slot 0x90). The running package's
/// procedure (`011a3ff0[kind][index]`, slot 0x280) then dispatches to its
/// process slot: travel, activate, acquire, eat, follow, escort, alarm,
/// flee, dialogue, pursue, get up, notify, accompany, use item, ambush,
/// remove worn, summon, avoid, patrol and the done-checks of kinds 13, 17
/// and 54. Unless waiting on a path, the actor's process records the time.
pub fn middle_high_process_update(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) {
    let p = this.addr();
    e.call(EMPTY_FUNCTION, &args![UPDATE_SCOPE_OBJECT]);
    if actor == 0 {
        return;
    }
    let skip = e.vcall(actor, 0x1d0, &[]).u32() == 0
        || e.call(FORM_FLAG_20, &args![actor]).bool()
        || e.call(FORM_FLAG_800, &args![actor]).bool()
        || e.call(REFERENCE_CELL, &args![actor]).u32() == 0
        || {
            let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
            !e.call(CELL_STATE_IS_3, &args![cell]).bool()
        };
    if skip {
        e.vcall(p, 0x28, &[]);
        return;
    }
    let is_actor = e.vcall(actor, 0x100, &[]).bool();
    let mover = if is_actor { actor } else { 0 };
    if mover == 0 {
        let process = e.call(WORD_68, &args![actor]).u32();
        e.vcall(process, 0x28, &[]);
        e.call(EMPTY_FUNCTION, &args![UPDATE_SCOPE_OBJECT]);
        return;
    }
    if byte(e, p, 0x18a) != 0 {
        let flag = u32::from(byte(e, p, 0x18a));
        let updated = e.call(BYTE_18A_UPDATE, &args![mover, flag]).u8();
        e.mem.set_u8(p + 0x18a, updated);
    }
    let decided = e.vcall(p, 0x24, &args![mover, 0u32]).u8();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if decided != 0 || (running == 0 && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![mover]).bool()) {
        e.vcall(p, 0x524, &args![mover]);
    }
    let running = e.vcall(p, 0x27c, &[]).u32();
    e.vcall(p, 0x82c, &[]);
    if decided != 0 {
        e.call(ACTOR_CALL_008A8F60, &args![mover]);
        e.vcall(p, 0x90, &args![mover, 0u32]);
    }
    if running != 0
        && e.call(PACKAGE_WORD_18, &args![running]).u32() != u32::MAX
        && !e.call(PACKAGE_FLAG_10000, &args![running]).bool()
    {
        let kind = e.call(PACKAGE_WORD_18, &args![running]).u32();
        let index = e.vcall(p, 0x280, &[]).u32();
        let table = e
            .mem
            .u32(PROCEDURE_TABLES.wrapping_add(kind.wrapping_mul(4)));
        let procedure = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        let a = args![mover];
        match procedure {
            0 => {
                e.vcall(
                    p,
                    0x2bc,
                    &args![mover, u32::from(decided), 1u32, 0x101u32, 0u32],
                );
            }
            2 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x7c8, &args![mover, 1u32]);
            }
            3 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x7d0, &a);
            }
            4 => {
                e.call(INTERFACE_GET_TARGET_REFR, &[]);
            }
            5 => {
                e.vcall(p, 0x7bc, &args![mover, 0u32]);
            }
            6 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x298, &args![mover, 1u32, 0x101u32, 0u32]);
            }
            7 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x7b4, &a);
            }
            8 => {
                e.vcall(p, 0x7c4, &a);
            }
            10 | 54 => {
                if procedure == 10 {
                    e.call(PACKAGE_TYPE, &args![running]);
                    e.vcall(p, 0x2a0, &a);
                }
                if update_procedure_done(e, p, mover, running) {
                    return;
                }
            }
            12 => {
                e.vcall(p, 0x83c, &a);
            }
            13 => update_follow_target(e, p, mover, running),
            14 => {
                e.vcall(p, 0x7c8, &args![mover, 0u32]);
            }
            15 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x29c, &args![mover, 0u32, 1u32, 0x101u32]);
            }
            17 => {
                if word(e, p, 0x40) == 0 {
                    e.vcall(p, 0x8c, &a);
                }
                if word(e, p, 0x40) == 0 {
                    e.vcall(p, 0x288, &args![mover, 1u32]);
                    if !e.call(ACTOR_IS_PATHING_COMPLETE, &a).bool() {
                        e.vcall(p, 0x294, &a);
                    }
                }
                if word(e, p, 0x40) != e.global::<u32>(PLAYER) {
                    e.vcall(p, 0xc8, &args![mover, running, 1u32]);
                }
                e.vcall(p, 0x288, &args![mover, 1u32]);
            }
            22 => {
                e.vcall(p, 0x2b8, &a);
                if e.vcall(mover, 0x214, &[]).u32() == 0 {
                    e.vcall(p, 0x288, &args![mover, 1u32]);
                }
            }
            26 => {
                e.vcall(p, 0x808, &a);
            }
            27 => {
                e.call(PACKAGE_TYPE, &args![running]);
                e.vcall(p, 0x7cc, &a);
            }
            28 => {
                e.vcall(p, 0x7d8, &a);
            }
            30 => {
                e.vcall(p, 0x7d4, &a);
            }
            33 => {
                e.vcall(p, 0x7e0, &a);
            }
            34 => {
                e.vcall(p, 0x7e4, &a);
            }
            35 => {
                e.vcall(p, 0x848, &args![mover, 1u32, 0u32]);
            }
            38 => {
                e.vcall(p, 0x800, &a);
            }
            39 => {
                e.vcall(p, 0x7e8, &a);
            }
            41 => {
                e.vcall(p, 0x288, &args![mover, 2u32]);
            }
            44 => {
                e.vcall(p, 0x7fc, &a);
            }
            49 => {
                e.vcall(p, 0x840, &a);
            }
            50 => {
                e.vcall(p, 0x7dc, &a);
            }
            _ => {}
        }
    }
    if !e.call(ACTOR_IS_WAITING_ON_PATH, &args![mover]).bool() {
        let process = e.call(WORD_68, &args![actor]).u32();
        e.vcall(process, 0x28, &[]);
    }
    e.call(EMPTY_FUNCTION, &args![UPDATE_SCOPE_OBJECT]);
}

// Translated from 0091ab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetTargetForPackage` (Xbox PDB): the target of
/// `package` for `actor`, by package type: 1, 2 and 7 the actor target
/// (`006780e0`) when the target is an actor; 13 (when it is the current
/// package) the patrol data's current reference (from the word at +0x04),
/// and without a target the patrol's first reference; 15 `actor` itself
/// when it is the package target, else [`middle_high_process_set_target_data`];
/// 28 the conversation target; the other types
/// [`middle_high_process_set_target_data`] with the package's target data.
pub fn middle_high_process_get_target_for_package(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    package: u32,
) -> u32 {
    let p = this.addr();
    let mut result = 0u32;
    let current = e.vcall(p, 0x22c, &[]).u32() == package;
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    match kind {
        1 | 2 | 7 => {
            if e.call(PACKAGE_IS_TARGET_AN_ACTOR, &args![package, actor])
                .bool()
            {
                result = e
                    .call(PACKAGE_ACTOR_TARGET, &args![package, actor, 0u32])
                    .u32();
            }
        }
        13 => {
            if current {
                let data = e.call(PROCESS_PACKAGE, &args![p]).u32();
                if data != 0 {
                    result = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
                }
            }
            if word(e, p, 0x40) == 0 {
                result = e
                    .call(PATROL_GET_INITIAL_REFERENCE, &args![actor, package])
                    .u32();
            }
        }
        15 => {
            let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
            if e.call(PACKAGE_TARGET_GET_REFERENCE, &args![data]).u32() == actor {
                result = actor;
            } else {
                let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
                result = middle_high_process_set_target_data(e, this, actor, package, data);
            }
        }
        28 => {
            result = e
                .call(DIALOGUE_GET_TARGET_OF_CONVERSATION, &args![package])
                .u32();
        }
        _ => {
            let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
            result = middle_high_process_set_target_data(e, this, actor, package, data);
        }
    }
    result
}

// Translated from 0091aca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetTargetForPackage` (Xbox PDB): clears the target
/// (slot 0x12C), then, for a running package with target data (or of
/// type 12 or 13), sets the target slot 0x810 (`GetTargetForPackage`)
/// gives. A target with flag 0x200000 (+0x08) is dropped (false); one
/// with form flag 0x20 is replaced by its extra list's reference pointer
/// when it has one. The target's position is remembered (+0xFC) and it is
/// marked targeted. Returns whether a target was set.
pub fn middle_high_process_set_target_for_package(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> bool {
    let p = this.addr();
    e.vcall(p, 0x12c, &args![0u32]);
    let running = e.vcall(p, 0x27c, &[]).u32();
    if running == 0 {
        return false;
    }
    if e.call(PACKAGE_WORD_30, &args![running]).u32() == 0 {
        let kind = e.call(PACKAGE_TYPE, &args![running]).u32();
        if kind != 0xc && e.call(PACKAGE_TYPE, &args![running]).u32() != 0xd {
            return false;
        }
    }
    let target = e.vcall(p, 0x810, &args![actor, running]).u32();
    e.vcall(p, 0x12c, &args![target]);
    if word(e, p, 0x40) == 0 {
        return false;
    }
    let target = word(e, p, 0x40);
    if e.call(FORM_FLAG_200000, &args![target]).bool() {
        e.vcall(p, 0x12c, &args![0u32]);
        return false;
    }
    let target = word(e, p, 0x40);
    if e.call(FORM_FLAG_20, &args![target]).bool() {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![target]).u32();
        if e.call(GET_REFERENCE_POINTER, &args![list]).u32() != 0 {
            let target = word(e, p, 0x40);
            let list = e.call(REFERENCE_EXTRA_LIST, &args![target]).u32();
            let pointer = e.call(GET_REFERENCE_POINTER, &args![list]).u32();
            e.vcall(p, 0x12c, &args![pointer]);
        }
    }
    let target = word(e, p, 0x40);
    store_last_seen(e, p, target);
    let target = word(e, p, 0x40);
    e.call(SET_TARGETED, &args![target, 1u32]);
    true
}

/// A reference target that the package's slot 0x22C (with 1) refuses while
/// the package is not created makes the package never run (`SetNeverRun`)
/// and gives 0; otherwise the reference.
fn checked_reference_target(e: &mut Engine, package: u32, reference: u32) -> u32 {
    if reference != 0
        && e.vcall(reference, 0x22c, &args![1u32]).bool()
        && !e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool()
    {
        e.call(PACKAGE_SET_NEVER_RUN, &args![package, reference, 1u32]);
        return 0;
    }
    reference
}

/// Fills the objects-to-acquire list of the process for `SetTargetData`
/// (the block that runs while the list is empty): the target object
/// (`+0x84`), object type (`+0x8C`) and faction (`+0x90`) from the target
/// data, form lists stepped through `+0x4C`/`+0x50`, multi-target types
/// built by `LowProcess::BuildMulitTargetObjectList`, and otherwise a
/// search of the references around (`EnumReferencesCloseToPoint` with
/// `0090dd80`) ordered by slot 0x818 (`EvaluateOrderAcquireList`).
fn build_acquire_list(e: &mut Engine, p: u32, actor: u32, package: u32, data: u32, location: u32) {
    let mut search = true;
    let mut cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    e.with_stack(0x18, |e, locals| {
        let point = locals.addr();
        let scratch = point + 0xc;
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        for offset in [0u32, 4, 8] {
            let value = word(e, at, offset);
            e.mem.set_u32(point + offset, value);
        }
        let kind = e.call(TARGET_GET_TYPE, &args![data]).u32();
        if kind == 1 {
            let object = e.call(TARGET_GET_OBJECT, &args![data]).u32();
            if e.call(FORM_TYPE, &args![object]).u32() == 0x55 {
                if word(e, p, 0x4c) == 0 {
                    e.mem.set_u32(p + 0x4c, object);
                    e.mem.set_u16(p + 0x50, 0);
                } else {
                    let number = e.mem.u16(p + 0x50).wrapping_add(1);
                    e.mem.set_u16(p + 0x50, number);
                }
                let list = word(e, p, 0x4c);
                let mut node = e.call(FORM_LIST_ENTRIES, &args![list]).u32();
                if !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    let mut index = 0i32;
                    while node != 0
                        && !e.call(LIST_IS_EMPTY, &args![node]).bool()
                        && i32::from(e.mem.i16(p + 0x50)) <= index
                    {
                        node_item(e, node);
                        node = e.call(WORD_4, &args![node]).u32();
                        index += 1;
                    }
                }
                if object != 0 {
                    e.mem.set_u32(p + 0x84, object);
                }
                e.mem.set_u32(p + 0x8c, 0);
            } else if e.call(FORM_TYPE, &args![object]).u32() == 8 {
                e.mem.set_u32(p + 0x90, object);
                e.mem.set_u32(p + 0x8c, 0x1c);
            } else if e.call(TARGET_GET_OBJECT, &args![data]).u32() != 0 {
                let object = e.call(TARGET_GET_OBJECT, &args![data]).u32();
                if e.vcall(object, 0xe4, &[]).bool() {
                    let object = e.call(TARGET_GET_OBJECT, &args![data]).u32();
                    e.mem.set_u32(p + 0x84, object);
                    e.mem.set_u32(p + 0x8c, 0);
                }
            }
        } else if kind == 2 {
            e.mem.set_u32(p + 0x84, 0);
            let object_type = e.call(TARGET_GET_OBJECT_TYPE, &args![data]).u32();
            e.mem.set_u32(p + 0x8c, object_type);
            e.mem.set_u32(p + 0x108, object_type);
        }
        if word(e, p, 0x84) == 0 {
            match word(e, p, 0x8c) {
                0 => {
                    let object = e.call(TARGET_GET_OBJECT, &args![data]).u32();
                    let magic = e
                        .call(
                            DYNAMIC_CAST,
                            &args![object, 0u32, TYPE_TES_FORM, TYPE_MAGIC_ITEM, 0u32],
                        )
                        .u32();
                    if magic != 0 {
                        e.mem.set_u32(p + 0x164, magic);
                    }
                    search = false;
                }
                12 | 19 | 20 | 22 | 23 => {
                    if !e.call(REFERENCE_BYTE_104, &args![actor]).bool() {
                        let object_type = word(e, p, 0x8c);
                        e.call(
                            BUILD_MULTI_TARGET_LIST,
                            &args![p, actor, location, object_type],
                        );
                        e.mem.set_u32(p + 0x108, object_type);
                        search = false;
                    }
                }
                24..=27 => {
                    let object_type = word(e, p, 0x8c);
                    e.call(EMPTY_FUNCTION_1, &args![actor, object_type]);
                    let object_type = word(e, p, 0x8c);
                    e.mem.set_u32(p + 0x108, object_type);
                    search = false;
                }
                _ => {}
            }
        }
        if search {
            let handler_call = |e: &mut Engine, cell: u32, second: u32, radius: f32| {
                let handler = e.global::<u32>(DATA_HANDLER);
                let bits = x87_quiet(radius).to_bits();
                e.call(
                    ENUM_REFERENCES_CLOSE_TO_POINT,
                    &args![
                        handler,
                        cell,
                        point,
                        bits,
                        second,
                        bits,
                        ACQUIRE_SEARCH_CALLBACK,
                        actor
                    ],
                );
            };
            if e.call(PACKAGE_GET_SEARCH_LOCATION, &args![package]).u32() != 0 {
                let radius = e
                    .call(PACKAGE_GET_SEARCH_RADIUS, &args![package, actor])
                    .u32();
                let location_cell = e
                    .call(PACKAGE_GET_SEARCH_CELL, &args![package, actor])
                    .u32();
                if location_cell != 0 {
                    cell = location_cell;
                }
                let coord = e
                    .call(PACKAGE_GET_SEARCH_COORD, &args![package, scratch, actor])
                    .u32();
                for offset in [0u32, 4, 8] {
                    let value = word(e, coord, offset);
                    e.mem.set_u32(point + offset, value);
                }
                let range = if radius > 0 {
                    f64::from(radius) as f32
                } else if cell != 0 && e.call(CELL_FLAG, &args![cell]).bool() {
                    x87_quiet(e.global::<f32>(FLOAT_MAX))
                } else {
                    let value = e
                        .call(FLOAT_SETTING_VALUE, &args![ACQUIRE_RADIUS_SETTING])
                        .u32();
                    x87_quiet(e.mem.f32(value))
                };
                handler_call(e, cell, point, range);
            } else {
                let range = e.with_stack(4, |e, out| {
                    let current = e.vcall(p, 0x22c, &[]).u32();
                    let found = current != 0 && {
                        let current = e.vcall(p, 0x22c, &[]).u32();
                        e.call(PACKAGE_GET_ACQUIRE_RADIUS, &args![current, actor, out])
                            .bool()
                    };
                    if !found {
                        let value = e
                            .call(FLOAT_SETTING_VALUE, &args![ACQUIRE_RADIUS_SETTING])
                            .u32();
                        let radius = x87_quiet(e.mem.f32(value));
                        e.mem.set_f32(out.addr(), radius);
                    }
                    e.mem.f32(out.addr())
                });
                let at = e.vcall(actor, 0x1f4, &[]).u32();
                handler_call(e, cell, at, range);
            }
            e.mem.set_u32(p + 0x8c, 0);
            e.mem.set_u32(p + 0x84, 0);
            e.mem.set_u32(p + 0x90, 0);
            e.vcall(p, 0x818, &args![actor]);
        }
    });
    if data != 0 {
        let count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
        e.mem.set_u32(p + 0x58, count);
    }
}

// Translated from 0091ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetTargetData` (Xbox PDB): the target `actor` should
/// take for `package` from its target data. A reference target (type 0)
/// or the actor's own reference (type 3, `00569b80`) is returned unless
/// refused ([`checked_reference_target`]). Otherwise the objects to
/// acquire (+0x5C) are taken one by one (each becomes `pAcquireObject`,
/// +0x64, and leaves the list): an unowned non-actor of kind 2 stands for
/// its owner's high-process actor (owners of form type 0x2A only); when
/// targets are reserved (not the player, `GetShouldReserveTarget`), one
/// that is neither the current furniture nor the target and has no free
/// marker is skipped. An empty list is built ([`build_acquire_list`]) once
/// when the process has no target. Returns the chosen reference, or 0.
pub fn middle_high_process_set_target_data(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    package: u32,
    data: u32,
) -> u32 {
    let p = this.addr();
    if package == 0 || data == 0 {
        return 0;
    }
    let location = if e.call(PACKAGE_TYPE, &args![package]).u32() == 2
        || e.call(PACKAGE_LOCATION, &args![package]).u32() == 0
    {
        actor
    } else {
        let place = e.call(PACKAGE_LOCATION, &args![package]).u32();
        e.call(LOCATION_GET_REFERENCE, &args![place]).u32()
    };
    if e.call(TARGET_GET_TYPE, &args![data]).u32() == 0 {
        let reference = e.call(PACKAGE_TARGET_GET_REFERENCE, &args![data]).u32();
        return checked_reference_target(e, package, reference);
    }
    if e.call(TARGET_GET_TYPE, &args![data]).u32() == 3 {
        let reference = e.call(REFERENCE_SELF_TARGET, &args![actor]).u32();
        return checked_reference_target(e, package, reference);
    }
    let mut built = false;
    loop {
        if e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
            built = true;
            build_acquire_list(e, p, actor, package, data, location);
        }
        let player = e.global::<u32>(PLAYER);
        let reserve = actor != player
            && e.call(PACKAGE_GET_SHOULD_RESERVE_TARGET, &args![package])
                .bool();
        while !e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
            let item = node_item(e, p + 0x5c);
            e.mem.set_u32(p + 0x64, item);
            e.call(LIST_REMOVE, &args![p + 0x5c, p + 0x64]);
            let acquire = word(e, p, 0x64);
            let mut reference = word(e, acquire, 0);
            if word(e, acquire, 0x18) == 2 && !e.vcall(reference, 0x100, &[]).bool() {
                let acquire = word(e, p, 0x64);
                let first = word(e, acquire, 0);
                let owner = e.call(REFERENCE_GET_OWNER, &args![first]).u32();
                let mut owner_npc = 0u32;
                if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 0x2a {
                    owner_npc = owner;
                }
                if owner_npc == 0 {
                    continue;
                }
                reference = e
                    .call(
                        GET_ACTOR_REF_IN_HIGH,
                        &args![PROCESS_LISTS, owner_npc, 0u32],
                    )
                    .u32();
            }
            if reserve
                && reference != 0
                && e.vcall(p, 0x4c8, &[]).u32() != reference
                && word(e, p, 0x40) != reference
                && e.call(REFERENCE_GET_FIRST_FREE_MARKER, &args![reference, 0u32])
                    .i32()
                    < 0
            {
                continue;
            }
            return reference;
        }
        if word(e, p, 0x40) != 0 || built {
            return 0;
        }
    }
}

/// Bubble-sorts the objects to acquire (+0x5C) by the signed word at
/// `offset` of each entry, ascending: at most as many passes as entries
/// (`005ae380`), while a pass swapped. A pass compares the outer node's
/// entry with each node from the outer one on and swaps the two entries
/// (`00726c60`, which stores only non-null values) when the outer one is
/// greater. The `JZ` on `this + 0x5C` (its `ADD` flags) skips everything
/// when that sum is zero.
fn sort_acquire_list(e: &mut Engine, p: u32, offset: u32) {
    let mut outer = p.wrapping_add(0x5c);
    if outer == 0 {
        return;
    }
    let mut count = e.call(LIST_COUNT, &args![outer]).u32();
    let mut swapped = true;
    while count != 0 && swapped {
        swapped = false;
        let mut inner = outer;
        while inner != 0 {
            let first = node_item(e, outer);
            let second = node_item(e, inner);
            if (word(e, first, offset) as i32) > (word(e, second, offset) as i32) {
                e.with_stack(8, |e, locals| {
                    e.mem.set_u32(locals.addr(), second);
                    e.mem.set_u32(locals.addr() + 4, first);
                    e.call(NODE_SET_ITEM, &args![outer, locals]);
                    e.call(NODE_SET_ITEM, &args![inner, locals.addr() + 4]);
                });
                swapped = true;
            }
            inner = e.call(WORD_4, &args![inner]).u32();
        }
        outer = e.call(WORD_4, &args![outer]).u32();
        count = count.wrapping_sub(1);
    }
}

// Translated from 0091b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: sorts the objects to acquire (+0x5C) by the word
/// at +0x18 of each entry ([`sort_acquire_list`]).
pub fn fn_0091b580(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    sort_acquire_list(e, this.addr(), 0x18);
}

// Translated from 0091b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: sorts the objects to acquire (+0x5C) by the word
/// at +0x10 of each entry ([`sort_acquire_list`]).
pub fn fn_0091b640(e: &mut Engine, this: Ptr<MiddleHighProcess>) {
    sort_acquire_list(e, this.addr(), 0x10);
}

// Translated from 0091b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::EvaluateOrderAcquireList` (Xbox PDB): moves the
/// found objects (+0x74) into the objects to acquire (+0x5C), each marked
/// targeted, deleting those that do not qualify, then sorts the list (by
/// +0x18, then +0x10) and empties +0x74. The running package's flags
/// 0x100, 0x400 and 0x200 (`008e30e0`, `008e3120`, `008e3100`) allow
/// buying, taking and the third kind. An unowned non-actor gets its
/// distance (+0x10, `_ftol2`). Otherwise an affordable item an owner
/// (the reference itself, or the `TESNPC` owner's high-process actor)
/// sells (`TESAIForm::SellBuysItem`) is bought (+0x18 = 2, distance);
/// else the allowed kinds set +0x18 (4 for taking, 5 for the third kind,
/// 3/4 for non-actors with a lock check through `Actor::HasObjects` with
/// `011ca268`) and the flags +0x08/+0x09; with none allowed, an actor
/// owned by someone else than the actor's base is kept with +0x18 = 5.
/// Actors with byte +0x18D set are skipped in several places.
pub fn middle_high_process_evaluate_order_acquire_list(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) {
    let p = this.addr();
    let mut node = p + 0x74;
    let running = e.vcall(p, 0x27c, &[]).u32();
    let may_buy = running != 0 && e.call(PACKAGE_FLAG2_100, &args![running]).bool();
    let may_take = running != 0 && e.call(PACKAGE_FLAG2_400, &args![running]).bool();
    let may_other = running != 0 && e.call(PACKAGE_FLAG2_200, &args![running]).bool();
    loop {
        if node == 0 || node_item(e, node) == 0 {
            fn_0091b580(e, this);
            fn_0091b640(e, this);
            e.call(LIST_CLEAR, &args![p + 0x74]);
            return;
        }
        let object = node_item(e, node);
        let reference = word(e, object, 0);
        let as_actor = if e.vcall(reference, 0x100, &[]).bool() {
            reference
        } else {
            0
        };
        let item = word(e, object, 4);
        let mut bought = false;
        let mut keep = true;
        if as_actor != 0 && e.call(BYTE_18D, &args![as_actor]).bool() {
            keep = false;
        }
        let forced = |e: &mut Engine| as_actor != 0 && e.call(BYTE_18D, &args![as_actor]).bool();
        let owned = e.call(REFERENCE_GET_OWNER, &args![reference]).u32() != 0;
        if !owned && !e.vcall(reference, 0x100, &[]).bool() {
            let distance = e
                .call(
                    REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                    &args![actor, reference, 0u32, 0u32],
                )
                .f64();
            let distance = e.call(FTOL, &args![distance]).u32();
            e.mem.set_u32(object + 0x10, distance);
        } else {
            let mut take = false;
            let mut other = false;
            let value = e.call(FORM_VALUE, &args![item]).i32();
            if !e.vcall(reference, 0x22c, &args![0u32]).bool()
                && e.call(ACTOR_GET_GOLD_AMOUNT, &args![actor]).i32() >= value
            {
                let mut seller = 0u32;
                if e.vcall(reference, 0x218, &[]).bool() {
                    seller = reference;
                } else {
                    let owner = e.call(REFERENCE_GET_OWNER, &args![reference]).u32();
                    let npc = e
                        .call(
                            DYNAMIC_CAST,
                            &args![owner, 0u32, TYPE_TES_FORM, TYPE_TES_NPC, 0u32],
                        )
                        .u32();
                    if npc != 0 {
                        seller = e
                            .call(GET_ACTOR_REF_IN_HIGH, &args![PROCESS_LISTS, npc, 0u32])
                            .u32();
                    }
                }
                if seller != 0 && may_buy {
                    let base = e.call(ACTOR_EXTRA_OBJECT, &args![seller]).u32();
                    if e.call(AI_FORM_SELL_BUYS_ITEM, &args![base + 0x90, item])
                        .bool()
                    {
                        let gold = e.call(ACTOR_GET_GOLD_AMOUNT, &args![actor]).i32();
                        let price = word(e, object, 4);
                        if gold >= e.call(FORM_VALUE, &args![price]).i32() {
                            if !e.call(REFERENCE_BYTE_104, &args![actor]).bool() {
                                e.mem.set_u32(object + 0x18, 2);
                                let distance = e
                                    .call(
                                        REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                                        &args![actor, reference, 0u32, 0u32],
                                    )
                                    .f64();
                                let distance = e.call(FTOL, &args![distance]).u32();
                                e.mem.set_u32(object + 0x10, distance);
                                bought = true;
                            } else {
                                keep = false;
                            }
                        }
                    }
                }
            }
            if !bought && !forced(e) {
                if !e.vcall(reference, 0x100, &[]).bool() {
                    if may_take {
                        take = true;
                    }
                } else if e.vcall(reference, 0x22c, &args![0u32]).bool() {
                    keep = true;
                } else if e.vcall(reference, 0x218, &[]).bool() {
                    if may_take {
                        take = true;
                    }
                    if may_other {
                        other = true;
                    }
                }
                if (take || other) && !forced(e) {
                    if !e.vcall(reference, 0x100, &[]).bool() {
                        if word(e, object, 0x18) == 1 {
                            e.mem.set_u32(object + 0x18, 4);
                            let has = e.with_stack(4, |e, out| {
                                let key = e.global::<u32>(LOCK_KEY_FORM);
                                e.call(ACTOR_HAS_OBJECTS, &args![actor, key, 0u32, 1u32, 0u32, out])
                                    .bool()
                            });
                            if !has && e.call(REFERENCE_GET_LOCK, &args![reference]).u32() != 0 {
                                keep = false;
                            }
                        } else {
                            e.mem.set_u32(object + 0x18, 3);
                        }
                    } else if take {
                        if !e.call(REFERENCE_BYTE_104, &args![actor]).bool() {
                            e.mem.set_u32(object + 0x18, 4);
                        } else {
                            keep = false;
                        }
                    } else if other {
                        if !e.call(REFERENCE_BYTE_104, &args![actor]).bool()
                            && !e.call(BYTE_18D, &args![actor]).bool()
                        {
                            if !forced(e) {
                                e.mem.set_u32(object + 0x18, 5);
                            }
                        } else {
                            keep = false;
                        }
                    }
                    e.mem.set_u8(object + 8, u8::from(take));
                    e.mem.set_u8(object + 9, u8::from(other));
                } else if e.vcall(actor, 0x21c, &[]).bool() {
                    keep = false;
                } else if e.vcall(reference, 0x100, &[]).bool()
                    && !e.vcall(reference, 0x218, &[]).bool()
                    && e.call(REFERENCE_GET_OWNER, &args![reference]).u32()
                        != e.call(REFERENCE_BASE, &args![actor]).u32()
                    && !e.call(BYTE_18D, &args![actor]).bool()
                {
                    if !forced(e) && as_actor != 0 {
                        e.mem.set_u32(object + 0x18, 5);
                        keep = true;
                    }
                } else {
                    keep = false;
                }
            }
        }
        if keep {
            let first = word(e, object, 0);
            e.call(SET_TARGETED, &args![first, 1u32]);
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), object);
                e.call(LIST_APPEND, &args![p + 0x5c, local]);
            });
        } else if object != 0 {
            e.call(ACQUIRE_OBJECT_DELETE, &args![object, 1u32]);
        }
        node = e.call(WORD_4, &args![node]).u32();
    }
}

/// Slot 0x2AC (`ProcessGetUpOneHour`) for an actor whose slot 0x214 is
/// non-zero.
fn get_up_if_sitting(e: &mut Engine, p: u32, actor: u32) {
    if e.vcall(actor, 0x214, &[]).u32() != 0 {
        e.vcall(p, 0x2ac, &args![actor]);
    }
}

/// The end-of-procedure block of [`middle_high_process_update_ov2`] for a
/// high-level actor. Returns true when `Update_ov2` returns at once.
fn update_ov2_procedure_done(e: &mut Engine, p: u32, actor: u32) -> bool {
    if e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32() != 1 {
        return false;
    }
    let running = e.vcall(p, 0x27c, &[]).u32();
    if running == 0 || e.call(PACKAGE_WORD_18, &args![running]).u32() == u32::MAX {
        return false;
    }
    let kind = e.call(PACKAGE_WORD_18, &args![running]).u32();
    let index = e.vcall(p, 0x280, &[]).u32();
    let table = e
        .mem
        .u32(PROCEDURE_TABLES.wrapping_add(kind.wrapping_mul(4)));
    if e.mem.u32(table.wrapping_add(index.wrapping_mul(4))) != 0x36 {
        return false;
    }
    if e.call(PACKAGE_WORD_18, &args![running]).u32() == 0 {
        if !package_reached(e, running, actor) {
            e.vcall(p, 0x288, &args![actor, u32::MAX]);
            return true;
        }
    } else {
        if e.call(PACKAGE_WORD_18, &args![running]).u32() == 3 {
            e.vcall(p, 0x288, &args![actor, u32::MAX]);
            return true;
        }
        let kind = e.call(PACKAGE_TYPE, &args![running]).u32();
        if kind == 3 || e.call(PACKAGE_TYPE, &args![running]).u32() == 4 {
            e.vcall(p, 0x288, &args![actor, u32::MAX]);
            return true;
        }
    }
    e.vcall(p, 0x294, &args![actor]);
    if running != 0 && e.call(PACKAGE_GET_ONCE_PER_DAY, &args![running]).bool() {
        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
        e.vcall(actor, 0x28c, &args![running, u32::from(day)]);
    }
    let data = e.call(PACKAGE_PLUS_38, &args![running]).u32();
    if e.call(WORD_4, &args![data]).u32() != 0 {
        return false;
    }
    e.vcall(p, 0x12c, &args![0u32]);
    if e.vcall(p, 0x20c, &[]).u32() == 0 || e.vcall(p, 0x4e0, &[]).bool() {
        let current = e.vcall(p, 0x22c, &[]).u32();
        if e.call(PACKAGE_GET_IS_CREATED, &args![current]).bool() {
            e.vcall(p, 0x22c, &[]);
            let current = e.vcall(p, 0x22c, &[]).u32();
            if e.call(PACKAGE_IS_INTERRUPT, &args![current]).bool() {
                e.vcall(p, 0x714, &args![actor]);
            } else {
                e.vcall(p, 0x234, &[]);
            }
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
        }
    } else {
        e.vcall(p, 0x214, &[]);
    }
    let acquire = word(e, p, 0x64);
    if acquire != 0 {
        e.call(ACQUIRE_OBJECT_DELETE, &args![acquire, 1u32]);
    }
    e.mem.set_u32(p + 0x64, 0);
    e.mem.set_u32(p + 0x34, 0);
    while !e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
        let item = node_item(e, p + 0x5c);
        if item != 0 {
            e.call(ACQUIRE_OBJECT_DELETE, &args![item, 1u32]);
        }
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), item);
            e.call(LIST_REMOVE, &args![p + 0x5c, local]);
        });
    }
    e.mem.set_u32(p + 0x44, 0);
    e.call(LIST_CLEAR, &args![p + 0x6c]);
    false
}

/// Whether the actor's current package accepts it (`accepts`) and its
/// process (+0x68) is at level 1 (+0x28).
fn current_package_done_high(
    e: &mut Engine,
    actor: u32,
    accepts: impl FnOnce(&mut Engine, u32) -> bool,
) -> bool {
    if e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32() == 0 {
        return false;
    }
    let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32();
    if !accepts(e, current) {
        return false;
    }
    let process = e.call(WORD_68, &args![actor]).u32();
    e.call(PROCESS_LEVEL, &args![process]).u32() == 1
}

// Translated from 0091bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Update_ov2` (Xbox PDB): the hourly update of a
/// persistent actor (`MobileObject` cast to `Actor`, `GetRefPersists`):
/// `CheckforNewPackage` and `ResetTarget`, then while the running
/// package's procedure (as in [`middle_high_process_update`]) asks to go
/// on, its one-hour variant runs: travel (and whether the package is
/// reached at a non-furniture location), activate, eat, follow (3600.0),
/// `0091c810`, alarm, combat (with `hours`), flee, dialogue, the
/// follow-target checks of kinds 13 and 14, pursue, notify, accompany,
/// use item, avoid, remove worn; after each, a high-level actor's
/// finished procedure (value 54) ends the package as `Update` does.
/// Unless waiting on a path, the reference's process records the time.
pub fn middle_high_process_update_ov2(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    reference: u32,
    hours: f32,
) {
    let p = this.addr();
    if reference == 0 {
        return;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![reference, 0u32, TYPE_MOBILE_OBJECT, TYPE_ACTOR, 0u32],
        )
        .u32();
    let mut go_on = true;
    if actor != 0 && e.call(REFERENCE_GET_REF_PERSISTS, &args![actor]).bool() {
        e.vcall(p, 0x24, &args![actor, 0u32]);
        let mut running = e.vcall(p, 0x27c, &[]).u32();
        e.vcall(p, 0x82c, &[]);
        // The package checks run once; the loop itself only tests `go_on`
        // (the jump back lands after them).
        let usable = running != 0
            && e.call(PACKAGE_WORD_18, &args![running]).u32() != u32::MAX
            && !e.call(PACKAGE_FLAG_10000, &args![running]).bool();
        while usable && go_on {
            let kind = e.call(PACKAGE_WORD_18, &args![running]).u32();
            let index = e.vcall(p, 0x280, &[]).u32();
            let table = e
                .mem
                .u32(PROCEDURE_TABLES.wrapping_add(kind.wrapping_mul(4)));
            let procedure = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
            let a = args![actor];
            match procedure {
                0 => {
                    let mut location = 0u32;
                    if e.call(PACKAGE_LOCATION, &args![running]).u32() != 0 {
                        let data = e.call(PACKAGE_LOCATION, &args![running]).u32();
                        location = e.call(LOCATION_GET_REFERENCE, &args![data]).u32();
                    }
                    if word(e, p, 0x44) != 0 && e.vcall(p, 0x20c, &[]).u32() == 0 {
                        location = word(e, p, 0x44);
                    }
                    if e.vcall(actor, 0x214, &[]).u32() != 0 {
                        let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &a).u32();
                        if !package_reached(e, current, actor) {
                            e.vcall(p, 0x2ac, &a);
                        }
                    }
                    e.vcall(p, 0x2bc, &args![actor, 1u32, 1u32, 0x101u32, 0u32]);
                    go_on = e.vcall(actor, 0x214, &[]).u32() == 0
                        && current_package_done_high(e, actor, |e, current| {
                            e.call(PACKAGE_TYPE, &args![current]).u32() != 6 && {
                                let current =
                                    e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32();
                                package_reached(e, current, actor)
                            }
                        })
                        && (location == 0
                            || !e.call(REFERENCE_IS_FURNITURE, &args![location]).bool());
                }
                2 => go_on = middle_high_process_process_activate_one_hour(e, this, actor),
                5 => go_on = middle_high_process_process_eat_one_hour(e, this, actor),
                6 => {
                    get_up_if_sitting(e, p, actor);
                    let seconds = x87_quiet(e.global::<f32>(ACCOMPANY_SECONDS));
                    go_on = middle_high_process_process_follow_one_hour(
                        e,
                        this,
                        actor,
                        seconds.to_bits(),
                        1,
                    );
                }
                7 => {
                    get_up_if_sitting(e, p, actor);
                    go_on = fn_0091c810(e, this, actor);
                }
                8 => {
                    get_up_if_sitting(e, p, actor);
                    e.vcall(p, 0x7c4, &a);
                    go_on = false;
                }
                9 => {
                    let hours = x87_quiet_bits(hours.to_bits());
                    go_on = e.vcall(p, 0x81c, &args![actor, hours]).bool();
                }
                10 => {
                    get_up_if_sitting(e, p, actor);
                    e.vcall(p, 0x2a0, &a);
                    go_on = false;
                }
                12 => {
                    e.vcall(p, 0x83c, &a);
                    go_on = false;
                }
                13 => {
                    if word(e, p, 0x40) == 0 {
                        e.vcall(p, 0x8c, &a);
                    }
                    let flagged = word(e, p, 0x40) == 0 || {
                        let target = word(e, p, 0x40);
                        e.call(FORM_FLAG_20, &args![target]).bool()
                    };
                    if flagged {
                        e.vcall(p, 0x288, &args![actor, 1u32]);
                    } else {
                        let current = e.vcall(p, 0x22c, &[]).u32();
                        let radius = e.call(PACKAGE_RADIUS, &args![current, actor]).u32();
                        let value = if radius == 0 { 0x190u32 } else { 0 };
                        if e.vcall(running, 0x144, &args![actor, value]).bool() {
                            e.vcall(p, 0x288, &args![actor, 1u32]);
                        }
                    }
                    go_on = false;
                }
                14 => {
                    if word(e, p, 0x40) == 0 {
                        e.vcall(p, 0x8c, &a);
                    }
                    if word(e, p, 0x40) == 0 {
                        e.vcall(p, 0x288, &args![actor, 1u32]);
                    }
                    let target = word(e, p, 0x40);
                    if target != 0
                        && e.vcall(target, 0x100, &[]).bool()
                        && word(e, p, 0x40) != e.global::<u32>(PLAYER)
                        && e.call(PACKAGE_TYPE, &args![running]).u32() == 2
                    {
                        let current = e.vcall(p, 0x22c, &[]).u32();
                        e.vcall(p, 0xc8, &args![actor, current, 1u32]);
                    }
                    middle_high_process_process_activate_one_hour(e, this, actor);
                    go_on = current_package_done_high(e, actor, |e, current| {
                        e.vcall(current, 0x144, &args![actor, 0u32]).bool()
                    });
                }
                15 => {
                    get_up_if_sitting(e, p, actor);
                    e.vcall(p, 0x29c, &args![actor, 0u32, 1u32, 0x101u32]);
                    go_on = false;
                }
                18 | 23 => {
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                    go_on = false;
                }
                22 => {
                    e.vcall(p, 0x2ac, &a);
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                }
                26 => {
                    e.vcall(p, 0x808, &a);
                    go_on = false;
                }
                27 => {
                    e.vcall(p, 0x86c, &a);
                    go_on = false;
                }
                28 => {
                    e.vcall(p, 0x7d8, &a);
                    go_on = false;
                }
                35 => {
                    e.vcall(p, 0x848, &args![actor, 1u32, 0u32]);
                    go_on = false;
                }
                39 => {
                    e.vcall(p, 0x7e8, &a);
                    go_on = false;
                }
                41 => {
                    e.vcall(p, 0x288, &args![actor, 2u32]);
                }
                50 => {
                    e.vcall(p, 0x7dc, &a);
                    go_on = false;
                }
                _ => go_on = false,
            }
            if update_ov2_procedure_done(e, p, actor) {
                return;
            }
            running = e.vcall(p, 0x27c, &[]).u32();
        }
    }
    if !e.call(ACTOR_IS_WAITING_ON_PATH, &args![actor]).bool() {
        let process = e.call(WORD_68, &args![reference]).u32();
        e.vcall(process, 0x28, &[]);
    }
}

// Translated from 0091c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::CastAbility` (Xbox PDB): `MagicCaster::CastAbility`
/// on `caster` with `(spell, flag)`; the process itself is not read.
pub fn middle_high_process_cast_ability(
    e: &mut Engine,
    _this: Ptr<MiddleHighProcess>,
    caster: u32,
    spell: u32,
    flag: u8,
) {
    e.call(
        MAGIC_CASTER_CAST_ABILITY,
        &args![caster, spell, u32::from(flag)],
    );
}

// Translated from 0091c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::TransferDisease` (Xbox PDB):
/// `MagicCaster::TransferDisease` on `caster` with `(first, second,
/// flag)`; the process itself is not read.
pub fn middle_high_process_transfer_disease(
    e: &mut Engine,
    _this: Ptr<MiddleHighProcess>,
    caster: u32,
    first: u32,
    second: u32,
    flag: u8,
) {
    e.call(
        MAGIC_CASTER_TRANSFER_DISEASE,
        &args![caster, first, second, u32::from(flag)],
    );
}

/// The escort distance of a package as `ProcessEscort` and `0091c810`
/// compute it: the package's integer (`00671d10`, 200 when not positive)
/// for the player, the setting `011cdbac` in cells with `00425fd0`, else
/// that integer times the setting `011cd82c`.
fn escort_distance(e: &mut Engine, p: u32, actor: u32, package: u32) -> f32 {
    let mut distance = 0.0f32;
    let data = e.call(PACKAGE_WORD_30, &args![package]).u32();
    let mut range = e.call(ITEM_CHANGE_FORM, &args![data]).i32();
    if range <= 0 {
        range = 200;
    }
    let zero = e.global::<f64>(ZERO_DOUBLE);
    if word(e, p, 0x40) == e.global::<u32>(PLAYER) && f64::from(distance) == zero {
        return range as f32;
    }
    if f64::from(distance) == zero {
        let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
        if e.call(CELL_FLAG, &args![cell]).bool() {
            let value = e
                .call(FLOAT_SETTING_VALUE, &args![ESCORT_CELL_DISTANCE_SETTING])
                .u32();
            return x87_quiet(e.mem.f32(value));
        }
    }
    if f64::from(distance) == zero {
        let value = e
            .call(FLOAT_SETTING_VALUE, &args![ESCORT_DISTANCE_FACTOR_SETTING])
            .u32();
        distance = (f64::from(e.mem.f32(value)) * f64::from(range)) as f32;
    }
    distance
}

/// The first extra list of an item change (0 without one).
fn first_extra_list(e: &mut Engine, change: u32) -> u32 {
    if e.call(FIRST_WORD, &args![change]).u32() == 0 {
        return 0;
    }
    let lists = e.call(FIRST_WORD, &args![change]).u32();
    node_item(e, lists)
}

/// Hands the item change over as `0091c810` does: into `container` when
/// it has one (`00892e90` with the package's target count and `owner`),
/// else through the actor's slot 0x3CC (count 1 unless the target data
/// has no reference, and the position of a location reference whose base
/// is `011ca244`/`011ca248`).
#[allow(clippy::too_many_arguments)]
fn hand_over_item(
    e: &mut Engine,
    p: u32,
    actor: u32,
    package: u32,
    change: u32,
    extra: &mut u32,
    owner: u32,
    container: u32,
    location: u32,
    data: u32,
) {
    if container != 0 && e.call(REFERENCE_HAS_CONTAINER, &args![container]).u32() != 0 {
        let count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
        let form = e.call(ITEM_CHANGE_FORM, &args![change]).u32();
        e.call(
            ACTOR_ITEM_TO_CONTAINER,
            &args![actor, form, *extra, container, count, owner],
        );
        return;
    }
    if e.call(FIRST_WORD, &args![change]).u32() != 0 {
        *extra = first_extra_list(e, change);
    }
    let mut point = 0u32;
    if location != 0 {
        let mut reference = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
        if reference == 0 {
            reference = word(e, p, 0x44);
        }
        if reference != 0 {
            let mut special =
                e.call(REFERENCE_BASE, &args![reference]).u32() == e.global::<u32>(ACTIVATE_FORM_B);
            if !special {
                special = e.call(REFERENCE_BASE, &args![reference]).u32()
                    == e.global::<u32>(ACTIVATE_FORM_A);
            }
            if special {
                point = e.vcall(reference, 0x1f4, &[]).u32();
            }
        }
    }
    let mut count = 1u32;
    if e.call(PACKAGE_TARGET_GET_REFERENCE, &args![data]).u32() == 0 {
        count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
    }
    let form = e.call(ITEM_CHANGE_FORM, &args![change]).u32();
    e.vcall(actor, 0x3cc, &args![form, *extra, count, point, 0u32]);
}

// Translated from 0091c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`Update_ov2`'s procedure 7, the one-hour escort):
/// like [`middle_high_process_process_escort`] without pathing. Without
/// the actor's item change, the acquire object's item (+0x64) of this
/// actor is taken (`GetInventoryItem`). When the package location is
/// reached (slot 0x13C): without an escort the item change is handed
/// over ([`hand_over_item`]) and deleted, and for target object types
/// 0x0C and 0x13 to 0x17 every object still to acquire is handed over in
/// turn, then the default outfit is put back and the action completes
/// (slot 0x118); the procedure advances. With an escort within the
/// distance the procedure advances and the escorted actor's created
/// package ends as in `ProcessEscort`. Otherwise the radius and distance
/// are computed and not used. Always returns false.
///
/// The two offsets the escort branch compares (`-0x14`, `-0x30` in the
/// game's frame) are never written by the function; the game reads
/// whatever its stack holds there, the translation zero (both lengths
/// 0.0, so the comparison passes).
// Translation incomplete: 0091c810 - the escort branch compares the lengths of two offsets the game never writes (uninitialized stack); the translation reads zeros there.
pub fn fn_0091c810(e: &mut Engine, this: Ptr<MiddleHighProcess>, actor: u32) -> bool {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if running == 0 {
        return false;
    }
    let mut change = e.call(ACTOR_ITEM_CHANGE, &args![actor]).u32();
    if change == 0 {
        let accepted = |e: &mut Engine| {
            let target = word(e, p, 0x40);
            if e.vcall(target, 0x100, &[]).bool() {
                return true;
            }
            let target = word(e, p, 0x40);
            let owner = e.call(WORD_C, &args![target]).u32();
            e.call(REFERENCE_INVENTORY_TEST, &args![actor, owner])
                .bool()
        };
        let ok = word(e, p, 0x40) != 0 && accepted(e);
        if !ok {
            e.vcall(p, 0x8c, &args![actor]);
            if word(e, p, 0x40) == 0 || !accepted(e) {
                e.vcall(p, 0x288, &args![actor, 1u32]);
                if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                    e.vcall(p, 0x294, &args![actor]);
                }
                return false;
            }
        }
    }
    if change == 0 && word(e, p, 0x64) != 0 && word(e, word(e, p, 0x64), 0) == actor {
        let item = word(e, word(e, p, 0x64), 4);
        change = e
            .call(REFERENCE_GET_INVENTORY_ITEM, &args![actor, item, 0u32])
            .u32();
    }
    let mut escort = false;
    if word(e, p, 0x40) != 0 {
        let target = word(e, p, 0x40);
        if e.vcall(target, 0x100, &[]).bool() && word(e, p, 0x40) != actor {
            escort = true;
            let target = word(e, p, 0x40);
            let leader = e
                .call(
                    DYNAMIC_CAST,
                    &args![target, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
                )
                .u32();
            let process = e.call(WORD_68, &args![leader]).u32();
            let leader_package = e.vcall(process, 0x22c, &[]).u32();
            if word(e, p, 0x40) != e.global::<u32>(PLAYER)
                && (leader_package == 0
                    || (e.call(PACKAGE_TYPE, &args![leader_package]).u32() != 1
                        && !e
                            .call(PACKAGE_GET_IS_CREATED, &args![leader_package])
                            .bool()))
            {
                e.vcall(p, 0x284, &args![0u32]);
                return false;
            }
        }
    }
    e.with_stack(0x30, |e, locals| {
        let offset = locals.addr();
        let first = offset + 0xc;
        let second = offset + 0x18;
        let scratch = offset + 0x24;
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        for index in [0u32, 4, 8] {
            let value = word(e, at, index);
            e.mem.set_u32(offset + index, value);
        }
        if escort {
            let target = word(e, p, 0x40);
            let target_at = e.vcall(target, 0x1f4, &[]).u32();
            let actor_at = e.vcall(actor, 0x1f4, &[]).u32();
            subtract_into(e, offset, scratch, actor_at, target_at);
        }
        let mut distance = 0.0f32;
        e.call(IDENTITY, &args![first]);
        e.call(IDENTITY, &args![second]);
        if escort {
            distance = escort_distance(e, p, actor, running);
        }
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        let location = e.call(PACKAGE_LOCATION, &args![running]).u32();
        if location == 0
            || !e
                .vcall(running, 0x13c, &args![actor, 0u32, minus_one, 0u32])
                .bool()
        {
            let current = e.vcall(p, 0x22c, &[]).u32();
            let radius = e
                .call(PACKAGE_GET_RADIUS_TO_TARGET, &args![current, actor, 0u32])
                .f64();
            e.call(FTOL, &args![radius]);
            let target = word(e, p, 0x40);
            if target != 0 && e.vcall(target, 0x100, &[]).bool() {
                let target = word(e, p, 0x40);
                e.call(
                    REFERENCE_GET_DISTANCE_FROM_REFERENCE,
                    &args![actor, target, 0u32, 0u32],
                );
            }
            return;
        }
        if !escort {
            let place = e.call(PACKAGE_LOCATION, &args![running]).u32();
            let data = e.call(PACKAGE_WORD_30, &args![running]).u32();
            if change != 0 {
                let mut extra = first_extra_list(e, change);
                let mut owner = 0u32;
                if extra != 0 && e.call(GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
                    let reference = e.call(GET_REFERENCE_POINTER, &args![extra]).u32();
                    owner = e.call(WORD_C, &args![reference]).u32();
                }
                let location = e.call(PACKAGE_LOCATION, &args![running]).u32();
                let mut container = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
                if container == 0 {
                    container = word(e, p, 0x44);
                }
                hand_over_item(
                    e, p, actor, running, change, &mut extra, owner, container, place, data,
                );
                if change != 0 {
                    e.call(ITEM_CHANGE_DELETE, &args![change, 1u32]);
                }
                let kind = e.call(TARGET_GET_OBJECT_TYPE, &args![data]).i32();
                let several = kind == 0xc || {
                    let kind = e.call(TARGET_GET_OBJECT_TYPE, &args![data]).i32();
                    kind >= 0x13 && e.call(TARGET_GET_OBJECT_TYPE, &args![data]).i32() <= 0x17
                };
                if several {
                    while !e.call(LIST_IS_EMPTY, &args![p + 0x5c]).bool() {
                        let item = node_item(e, p + 0x5c);
                        e.mem.set_u32(p + 0x64, item);
                        e.call(LIST_REMOVE, &args![p + 0x5c, p + 0x64]);
                        let acquire = word(e, p, 0x64);
                        let reference = word(e, acquire, 0);
                        e.vcall(p, 0x12c, &args![reference]);
                        let acquire = word(e, p, 0x64);
                        let object = word(e, acquire, 4);
                        let next = e
                            .call(REFERENCE_GET_INVENTORY_ITEM, &args![actor, object, 0u32])
                            .u32();
                        let acquire = word(e, p, 0x64);
                        if acquire != 0 {
                            e.call(ACQUIRE_OBJECT_DELETE, &args![acquire, 1u32]);
                        }
                        e.mem.set_u32(p + 0x64, 0);
                        hand_over_item(
                            e, p, actor, running, next, &mut extra, owner, container, place, data,
                        );
                        if next != 0 {
                            e.call(ITEM_CHANGE_DELETE, &args![next, 1u32]);
                        }
                    }
                    reset_outfit_by_kind(e, p, actor);
                }
                e.vcall(p, 0x118, &args![1u32]);
            }
            e.vcall(p, 0x288, &args![actor, 1u32]);
        } else {
            let length = e.call(POINT_LENGTH, &args![offset]).f64();
            if f64::from(distance) >= length {
                let first_length = e.call(POINT_LENGTH, &args![first]).f64();
                let second_length = e.call(POINT_LENGTH, &args![second]).f64();
                if second_length >= first_length {
                    e.vcall(p, 0x288, &args![actor, 1u32]);
                    let leader = word(e, p, 0x40);
                    let process = e.call(WORD_68, &args![leader]).u32();
                    if e.vcall(process, 0x27c, &[]).u32() != 0 {
                        let process = e.call(WORD_68, &args![leader]).u32();
                        e.vcall(process, 0x288, &args![actor, 1u32]);
                        let process = e.call(WORD_68, &args![leader]).u32();
                        let package = e.vcall(process, 0x27c, &[]).u32();
                        if e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool() {
                            let process = e.call(WORD_68, &args![leader]).u32();
                            e.vcall(process, 0x234, &[]);
                            if e.call(ACTOR_IS_PATHING_COMPLETE, &args![leader]).bool() {
                                e.call(ACTOR_END_MOVEMENT, &args![leader]);
                            }
                            e.vcall(p, 0x118, &args![1u32]);
                            e.vcall(p, 0x24, &args![actor, 0u32]);
                            return;
                        }
                    }
                }
            }
        }
        if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            e.vcall(p, 0x294, &args![actor]);
        }
    });
    false
}

/// The outfit reset at the end of `0091c810`'s hand-over: both flags set
/// unless the current package (slot 0x22C) has flag 0x200000 (second
/// cleared); an actor whose slot 0x218 answers true as `TESNPC`
/// (`006047c0` with `(first, second, 0, 1)`), else as `TESCreature`
/// (`InitDefaultWorn`).
fn reset_outfit_by_kind(e: &mut Engine, p: u32, actor: u32) {
    let mut second = 1u32;
    if e.vcall(p, 0x22c, &[]).u32() != 0 {
        let current = e.vcall(p, 0x22c, &[]).u32();
        if e.call(PACKAGE_FLAG_200000, &args![current]).bool() {
            second = 0;
        }
    }
    if e.vcall(actor, 0x218, &[]).bool() {
        let base = e.call(REFERENCE_BASE, &args![actor]).u32();
        let npc = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
            )
            .u32();
        if npc != 0 {
            e.call(
                NPC_INIT_DEFAULT_WORN,
                &args![npc, actor, 1u32, second, 0u32, 1u32],
            );
        }
    } else {
        let base = e.call(REFERENCE_BASE, &args![actor]).u32();
        let creature = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_CREATURE, 0u32],
            )
            .u32();
        if creature != 0 {
            e.call(
                CREATURE_INIT_DEFAULT_WORN,
                &args![creature, actor, 1u32, second, 1u32],
            );
        }
    }
}

// Translated from 0091d3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessFollowOneHour` (Xbox PDB): an hour of
/// [`middle_high_process_process_follow`] without the move mode and the
/// unused elapsed-hours calls (the second word, a float, is not read).
/// Returns true only when pathing to the target (slot 0x824) fails.
pub fn middle_high_process_process_follow_one_hour(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
    _unused_2: u32,
    advance: u8,
) -> bool {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if running != 0 && e.call(PACKAGE_FLAG_10000, &args![running]).bool() {
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_TEST_00962720, &args![player, actor]).bool() {
            return false;
        }
        let player = e.global::<u32>(PLAYER);
        let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
        let limit = e
            .call(SETTING_VALUE, &args![FOLLOW_COUNT_LIMIT_SETTING])
            .u32();
        if count >= e.mem.i32(limit) {
            return false;
        }
        e.call(PACKAGE_CALL_00674F30, &args![running, 0u32]);
    }
    let reset = if word(e, p, 0x40) == 0 {
        true
    } else {
        let target = word(e, p, 0x40);
        e.call(FORM_FLAG_20, &args![target]).bool()
    };
    if reset {
        e.vcall(p, 0x8c, &args![actor]);
        if word(e, p, 0x40) != 0 {
            let target = word(e, p, 0x40);
            store_last_seen(e, p, target);
        }
    }
    if word(e, p, 0x40) == 0 {
        if advance != 0 {
            e.vcall(p, 0x288, &args![actor, 1u32]);
        }
        return false;
    }
    let target = word(e, p, 0x40);
    let distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, target, 0u32, 0u32],
        )
        .f64() as f32;
    e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]);
    let target = word(e, p, 0x40);
    e.call(REFERENCE_GET_WORLD_SPACE, &args![target]);
    let radius = e
        .call(PACKAGE_GET_RADIUS_TO_TARGET, &args![running, actor, 0u32])
        .f64() as f32;
    let saved = word(e, p, 0x40);
    let mut finished = saved == 0;
    if !finished
        && e.call(PACKAGE_TYPE, &args![running]).u32() == 1
        && e.call(PACKAGE_LOCATION, &args![running]).u32() != 0
    {
        let location = e.call(PACKAGE_LOCATION, &args![running]).u32();
        if e.call(LOCATION_GET_TYPE, &args![location]).i32() < 2 {
            finished = package_reached(e, running, actor);
        }
    }
    if finished {
        if advance != 0 {
            e.vcall(p, 0x288, &args![actor, 1u32]);
            if e.call(PACKAGE_GET_IS_CREATED, &args![running]).bool() {
                if e.vcall(p, 0x20c, &[]).u32() != 0 {
                    e.vcall(p, 0x214, &[]);
                } else {
                    e.vcall(p, 0x234, &[]);
                }
                e.vcall(p, 0x24, &args![actor, 0u32]);
                return false;
            }
            if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                e.vcall(p, 0x294, &args![actor]);
            }
        }
    } else {
        let target = word(e, p, 0x40);
        let at = e.vcall(target, 0x1f4, &[]).u32();
        let moved = e.with_stack(12, |e, offset| {
            let result = e.call(POINT_SUBTRACT, &args![p + 0xfc, offset, at]).u32();
            e.call(POINT_LENGTH, &args![result]).f64() as f32
        });
        let value = e
            .call(FLOAT_SETTING_VALUE, &args![FOLLOW_MOVED_SETTING])
            .u32();
        let limit = e.mem.f32(value);
        let go = if f64::from(limit) < f64::from(moved) {
            true
        } else {
            f64::from(radius) < f64::from(distance)
                && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
        };
        if go {
            if !setup_path_to(e, p, actor, |e| {
                let target = word(e, p, 0x40);
                e.vcall(target, 0x1f4, &[]).u32()
            }) {
                return true;
            }
            let target = word(e, p, 0x40);
            store_last_seen(e, p, target);
        }
    }
    let distance = e
        .call(
            REFERENCE_GET_DISTANCE_FROM_REFERENCE,
            &args![actor, saved, 0u32, 0u32],
        )
        .f64() as f32;
    if f64::from(radius) > f64::from(distance)
        && !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
    {
        e.call(ACTOR_END_MOVEMENT, &args![actor]);
    }
    false
}

// Translated from 0091d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessEatOneHour` (Xbox PDB): the actor eats its
/// best food item (`Actor::Eat` with its form, first extra list and 1)
/// and the procedure advances (slot 0x288 with `(actor, 1)`). Returns
/// false.
pub fn middle_high_process_process_eat_one_hour(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> bool {
    let food = e.call(ACTOR_GET_BEST_FOOD_ITEM, &args![actor]).u32();
    if food != 0 {
        let extra = first_extra_list(e, food);
        let form = e.call(ITEM_CHANGE_FORM, &args![food]).u32();
        e.call(ACTOR_EAT, &args![actor, form, extra, 1u32]);
        e.vcall(this.addr(), 0x288, &args![actor, 1u32]);
    }
    false
}

/// Whether the player is the target and sleeping or resting with its
/// byte +0x20C clear; then it gets `005c1a00` with `(0, 1)` and the
/// procedure goes back by `step` (slot 0x288).
fn wake_player_target(e: &mut Engine, p: u32, actor: u32, step: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    if word(e, p, 0x40) != player
        || !e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool()
        || byte(e, e.global::<u32>(PLAYER), 0x20c) != 0
    {
        return false;
    }
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_SET_654, &args![player, 0u32, 1u32]);
    e.vcall(p, 0x288, &args![actor, step]);
    true
}

// Translated from 0091d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ProcessActivateOneHour` (Xbox PDB): an hour of
/// activating the target (+0x40, set through slot 0x8C; a missing or
/// flagged target advances the procedure). A type-9 package whose slot
/// 0x148 refuses goes back. The actor is close within 128.0 of a door's
/// teleport point, or as the current package's slot 0x144 says. Not
/// close: a sitting actor gets up (slot 0x2AC) and the target point is
/// read; false. Close: a sleeping or resting player target is woken
/// (steps -2 and -1); a non-actor target is activated
/// (`TESObjectREFR::Activate`); the target is marked activated (slot
/// 0x4EC) and the procedure advances; true.
pub fn middle_high_process_process_activate_one_hour(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    actor: u32,
) -> bool {
    let p = this.addr();
    let running = e.vcall(p, 0x27c, &[]).u32();
    if word(e, p, 0x40) == 0 {
        e.vcall(p, 0x8c, &args![actor]);
    }
    let missing = word(e, p, 0x40) == 0 || {
        let target = word(e, p, 0x40);
        e.call(FORM_FLAG_20, &args![target]).bool()
    };
    if missing {
        e.vcall(p, 0x288, &args![actor, 1u32]);
        return false;
    }
    if e.call(PACKAGE_TYPE, &args![running]).u32() == 9
        && !e.vcall(running, 0x148, &args![actor, 300u32]).bool()
    {
        e.vcall(p, 0x288, &args![actor, u32::MAX]);
    }
    let target = word(e, p, 0x40);
    let door = e.call(REFERENCE_DOOR_TELEPORT, &args![target]).u32();
    let close = if door != 0 {
        let at = e.vcall(actor, 0x1f4, &[]).u32();
        let length = e.with_stack(12, |e, offset| {
            let target = word(e, p, 0x40);
            let door_point = e.call(REFERENCE_DOOR_POSITION, &args![target]).u32();
            e.call(POINT_SUBTRACT, &args![door_point, offset, at]);
            e.call(POINT_LENGTH, &args![offset]).f64()
        });
        length <= e.global::<f64>(ACTIVATE_DOOR_DISTANCE)
    } else {
        let current = e.vcall(p, 0x22c, &[]).u32();
        e.vcall(current, 0x144, &args![actor, 0u32]).bool()
    };
    if !close {
        if e.vcall(actor, 0x214, &[]).u32() != 0 {
            e.vcall(p, 0x2ac, &args![actor]);
        }
        e.with_stack(12, |e, point| {
            e.call(IDENTITY, &args![point]);
            let target = word(e, p, 0x40);
            let at = if e.call(REFERENCE_DOOR_TELEPORT, &args![target]).u32() == 0 {
                let target = word(e, p, 0x40);
                e.vcall(target, 0x1f4, &[]).u32()
            } else {
                let target = word(e, p, 0x40);
                e.call(REFERENCE_DOOR_POSITION, &args![target]).u32()
            };
            for offset in [0u32, 4, 8] {
                let value = word(e, at, offset);
                e.mem.set_u32(point.addr() + offset, value);
            }
        });
        return false;
    }
    if wake_player_target(e, p, actor, 0xffff_fffe) {
        return false;
    }
    let target = word(e, p, 0x40);
    if !e.vcall(target, 0x100, &[]).bool() {
        e.vcall(p, 0x4ec, &args![1u32]);
        let target = word(e, p, 0x40);
        e.call(REFERENCE_ACTIVATE, &args![target, actor, 0u32, 0u32, 1u32]);
        e.vcall(p, 0x288, &args![actor, 1u32]);
        return true;
    }
    if wake_player_target(e, p, actor, u32::MAX) {
        return false;
    }
    e.vcall(p, 0x4ec, &args![1u32]);
    e.vcall(p, 0x288, &args![actor, 1u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type Process = Ptr<MiddleHighProcess>;
    type Item = Ptr<QueuedItem>;
    vec![
        entry!(0x00460ff0, middle_high_process_get_furniture_marker(Process) -> u32),
        entry!(0x008dd8a0, middle_high_process_process_notify(Process, u32)),
        entry!(0x009139b0, fn_009139b0(Item) -> Item),
        entry!(0x00913a20, fn_00913a20(Item, u32) -> bool),
        entry!(0x00913d60, queued_item_save_game(Item, u32)),
        entry!(0x00913e80, queued_item_load_game(Item, u32)),
        entry!(0x00913fb0, fn_00913fb0(Item, u32)),
        entry!(0x00913fe0, fn_00913fe0(Process) -> Process),
        entry!(0x009145c0, middle_high_process_get_radiation_delta(Process) -> f32),
        entry!(0x009145e0, middle_high_process_scalar_deleting_destructor(Process, u32) -> Process),
        entry!(0x00914610, middle_high_process_destructor(Process)),
        entry!(0x00914ab0, fn_00914ab0(Ptr, u32) -> Ptr),
        entry!(0x00914ae0, fn_00914ae0(Ptr)),
        entry!(0x00914b40, middle_high_process_clear_reservation(Process)),
        entry!(
            0x00914be0,
            middle_high_process_handle_deleted_reference(Process, u32)
        ),
        entry!(
            0x00914c60,
            middle_high_process_middle_high_copy(Process, u32)
        ),
        entry!(0x00915400, middle_high_process_setup_new_package(Process)),
        entry!(
            0x009154b0,
            middle_high_process_resolve_equipment_queue(Process, u32)
        ),
        entry!(0x00915790, fn_00915790(Item, u32) -> Item),
        entry!(0x009157c0, fn_009157c0(Item)),
        entry!(0x009157e0, middle_high_process_is_item_queued(Process, u32, u32, u8) -> bool),
        entry!(
            0x00915870,
            middle_high_process_remove_queued_item(Process, u32, u32)
        ),
        entry!(
            0x00915950,
            middle_high_process_add_queued_item(
                Process, u32, u8, u32, i32, u32, u8, u8, u32, u8, u8, u8
            )
        ),
        entry!(
            0x00915ae0,
            middle_high_process_remove_all_queued_items(Process)
        ),
        entry!(
            0x00915bc0,
            middle_high_process_process_remove_worn(Process, u32, u8, u8)
        ),
        entry!(0x00915d00, middle_high_process_get_want_weapon_drawn(Process) -> u8),
        entry!(
            0x00915d20,
            middle_high_process_set_want_weapon_drawn(Process, u8)
        ),
        entry!(0x00915d40, middle_high_process_get_weapon_drawn(Process) -> u8),
        entry!(
            0x00915d60,
            middle_high_process_set_weapon_drawn(Process, u32, u8)
        ),
        entry!(
            0x00915e50,
            middle_high_process_set_furniture(Process, u32, u8, u32)
        ),
        entry!(0x00915ef0, fn_00915ef0(Process, u32) -> u8),
        entry!(0x00915f10, middle_high_process_process_activate(Process, u32, u8) -> bool),
        entry!(
            0x00916880,
            middle_high_process_enter_combat(Process, u32, u32, u8, u8, u32, u8, u8, u8, u8, u8, u8, u8, u32) -> bool
        ),
        entry!(0x00916df0, middle_high_process_process_escort(Process, u32)),
        entry!(
            0x00917770,
            middle_high_process_process_accompany_one_hour(Process, u32)
        ),
        entry!(
            0x009177a0,
            middle_high_process_process_accompany(Process, u32)
        ),
        entry!(0x009177d0, middle_high_process_process_sleep(Process, u32)),
        entry!(
            0x00917a70,
            middle_high_process_process_eat(Process, u32, u32)
        ),
        entry!(0x00917df0, middle_high_process_checkfor_new_package(Process, u32, u8) -> bool),
        entry!(
            0x009180b0,
            middle_high_process_setup_path_finding(Process, u32, f32, f32, f32, u32, u32) -> bool
        ),
        entry!(
            0x00918120,
            middle_high_process_process_pursue(Process, u32, u32, u32, u32)
        ),
        entry!(0x009185e0, fn_009185e0(u32) -> u32),
        entry!(
            0x00918600,
            middle_high_process_process_flee_non_combat(Process, u32)
        ),
        entry!(0x00918860, middle_high_process_process_flee(Process, u32)),
        entry!(
            0x00918ee0,
            middle_high_process_process_follow(Process, u32, u8, u32, u32)
        ),
        entry!(
            0x00919320,
            middle_high_process_process_travel(Process, u32, u8, u8, u32, u32)
        ),
        entry!(0x00919f60, middle_high_process_update(Process, u32)),
        entry!(0x0091ab30, middle_high_process_get_target_for_package(Process, u32, u32) -> u32),
        entry!(0x0091aca0, middle_high_process_set_target_for_package(Process, u32) -> bool),
        entry!(0x0091ae00, middle_high_process_set_target_data(Process, u32, u32, u32) -> u32),
        entry!(0x0091b580, fn_0091b580(Process)),
        entry!(0x0091b640, fn_0091b640(Process)),
        entry!(
            0x0091b700,
            middle_high_process_evaluate_order_acquire_list(Process, u32)
        ),
        entry!(
            0x0091bc60,
            middle_high_process_update_ov2(Process, u32, f32)
        ),
        entry!(
            0x0091c7c0,
            middle_high_process_cast_ability(Process, u32, u32, u8)
        ),
        entry!(
            0x0091c7e0,
            middle_high_process_transfer_disease(Process, u32, u32, u32, u8)
        ),
        entry!(0x0091c810, fn_0091c810(Process, u32) -> bool),
        entry!(0x0091d3c0, middle_high_process_process_follow_one_hour(Process, u32, u32, u8) -> bool),
        entry!(0x0091d790, middle_high_process_process_eat_one_hour(Process, u32) -> bool),
        entry!(0x0091d810, middle_high_process_process_activate_one_hour(Process, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fake vtable targets: slot `n` of a test process, actor or other
    /// object calls `<base> + n`.
    const PROCESS_SLOTS: u32 = 0x0300_0000;
    const ACTOR_SLOTS: u32 = 0x0310_0000;
    const OBJECT_SLOTS: u32 = 0x0320_0000;
    const VTABLE_SIZE: u32 = 0x900;
    /// Where the tests put the player character.
    const TEST_PLAYER: u32 = 0x0600_0000;

    const CALLEES: &[u32] = &[
        ALLOCATE,
        FREE,
        FORM_TYPE,
        FIRST_WORD,
        QUEUED_FILE_POINTER_INIT,
        QUEUED_FILE_POINTER_ASSIGN,
        QUEUED_FILE_CONSTRUCT,
        QUEUE_OBJECT_FILES,
        OBJECT_GROUP_INDEX,
        QUEUE_KF_FILES,
        BUILD_KF_FILE_LIST,
        LIST_DELETE,
        ANIMATION_GROUP_LOADED,
        ANIMATION_HAS_DATA,
        REFERENCE_GET_MODEL,
        ANIM_GROUP,
        WORD_68,
        SAVE_FORM_ID,
        SAVE_DATA,
        LOAD_FORM_ID,
        LOAD_DATA,
        LOOKUP_FORM,
        DYNAMIC_CAST,
        REFERENCE_EXTRA_LIST,
        GET_CONTAINER_CHANGES,
        EXTRA_LIST_SAVE_INDEX,
        EXTRA_LIST_FROM_INDEX,
        POINTER_RELEASE,
        DATA_HANDLER_BYTE_61D,
        SAVE_LOAD_FLAG_2,
        SET_MARKER_RESERVED,
        PACKAGE_HANDLE_DELETED,
        PROCESS_PACKAGE,
        MIDDLE_LOW_CONSTRUCT,
        MIDDLE_LOW_DESTRUCT,
        LIST_CONSTRUCT,
        LIST_DESTRUCT,
        LIST_CLEAR,
        LIST_IS_EMPTY,
        LIST_REMOVE_HEAD,
        IDENTITY,
        WORD_4,
        ACTOR_PACKAGE_CONSTRUCT,
        ACTOR_PACKAGE_DESTRUCT,
        ACTOR_PACKAGE_CLEAR,
        POINTER_INIT,
        POINTER_ASSIGN,
        FURNITURE_MARK_CONSTRUCT,
        FURNITURE_MARK_SET_HEADING,
        SET_BYTE_E,
        COPY_POINT,
        CACHED_VALUES_CONSTRUCT,
        SET_CURRENT_SPELL,
        SET_DESIRED_TARGET,
        MEMSET,
        CLONE_RESERVER_ASSIGN,
        CLONE_RESERVER_RELEASE,
        QUEUED_FILE_POINTER_RELEASE,
        SET_WORD_28,
        ITEM_CHANGE_DELETE,
        RUN_ONCE_PACKAGE,
        ALWAYS_FALSE,
        DELETE_FORM,
        CLEAR_RUN_ONCE_PACKAGE,
        SET_ANIMATION,
        ARROW_RELEASE,
        LIP_SYNCH_ANIM_DELETE,
        ACTOR_PACKAGE_MOVE_TO,
        ANIMATION_CLEAR_GROUP,
        LIST_ADD,
        QUEUED_FILE_POINTER_COPY,
        CLONE_RESERVER_COPY,
        CONTROLLER_POINTER_COPY,
        SET_TARGETED,
        MEMCPY,
        EMPTY_FUNCTION,
        SET_PROCESS_FLAG_40,
        QUEUED_FILE_DONE,
        SET_FORM_TYPE,
        ACTOR_EQUIP_OBJECT,
        ACTOR_UNEQUIP_OBJECT,
        ACTOR_SET_BLOCK,
        ACTOR_GET_CURRENT_WEAPON,
        ACTOR_GET_ANIM_ACTION,
        ANIMATION_ADD_FROM_LIST,
        TRY_CANCEL_TASK,
        LIST_REMOVE,
        LIST_APPEND,
        SCOPE_ENTER,
        SCOPE_LEAVE,
        REFERENCE_BASE,
        PACKAGE_TYPE,
        PACKAGE_FLAG_200000,
        NPC_INIT_DEFAULT_WORN,
        CREATURE_INIT_DEFAULT_WORN,
        ITEM_CHANGE_MOD_EFFECT_ACTIVE,
        ITEM_CHANGE_FORM,
        ACTOR_UPDATE_DRAWN_FLAGS,
        ACTOR_BYTE_70,
        FORM_FLAG_20,
        PACKAGE_GET_LOCATION_REFERENCE,
        REFERENCE_IS_FURNITURE,
        REFERENCE_DOOR_TELEPORT,
        REFERENCE_DOOR_POSITION,
        POINT_SUBTRACT,
        POINT_LENGTH,
        SETTING_VALUE,
        REFERENCE_GET_MARKER_USED,
        ACTOR_IS_PATHING_COMPLETE,
        REFERENCE_GET_WORLD_SPACE,
        REFERENCE_CELL,
        REFERENCE_GET_CLOSEST_FREE_MARKER,
        CALENDAR_GET_HOUR,
        FLOAT_20,
        GLOBAL_VALUE,
        GET_CURRENT_PROCESS_TYPE,
        GET_DESIRED_PROCESS_LEVEL,
        PACKAGE_FLAG_4,
        ALIGN_TO_FURNITURE,
        REFERENCE_ACTIVATE,
        REFERENCE_RUN_SCRIPT,
        ACQUIRE_OBJECT_DELETE,
        IS_COMBAT_TARGET,
        PLAYER_COMBAT_STANDING,
        ADD_TO_PLAYER_COMBAT_LIST,
        LOW_PROCESS_ENTER_COMBAT,
        ACTOR_STATE_IS_5,
        ACTOR_STATE_IS_3,
        ALWAYS_FALSE_2,
        GET_NUMBER_ACTORS_IN_COMBAT,
        REFERENCE_BYTE_14D,
        ACTOR_0087F3B0,
        ACTOR_SWIMS_ONLY,
        PACKAGE_IS_INTERRUPT,
        GET_PACKAGE_EXTRA,
        PACKAGE_FLAG_400000,
        PACKAGE_RADIUS,
        REFERENCE_DISTANCE_TO,
        REFERENCE_BYTE_104,
        ACTOR_ITEM_CHANGE,
        WORD_C,
        REFERENCE_INVENTORY_TEST,
        PACKAGE_GET_IS_CREATED,
        PACKAGE_WORD_30,
        CELL_FLAG,
        FLOAT_SETTING_VALUE,
        PACKAGE_LOCATION,
        GET_REFERENCE_POINTER,
        LOCATION_GET_REFERENCE,
        REFERENCE_HAS_CONTAINER,
        PACKAGE_GET_INITIAL_TARGET_COUNT,
        ACTOR_ITEM_TO_CONTAINER,
        MOBILE_GET_CURRENT_PACKAGE,
        PACKAGE_GET_LOCATION_COORD,
        PACKAGE_GET_LOCATION_WORLD,
        PACKAGE_GET_LOCATION_CELL,
        ACTOR_END_MOVEMENT,
        FIND_BED_CHAIRS,
        LIST_COUNT,
        TRY_ENTER_LOCK,
        LEAVE_LOCK,
        CLEAR_FURNITURE_MARKER,
        REFERENCE_GET_OWNER,
        RANDOM,
        ACTOR_GET_BEST_FOOD_ITEM,
        ACTOR_EAT,
        ACTOR_MOVEMENT_FLAGS,
        ACTOR_CLEAR_MOVE_MODE,
        MOBILE_IS_IN_DIALOGUE,
        ACTOR_IS_ALARMED,
        ACTOR_IS_RUNNING_RUN_ONCE,
        FTOL,
        LOW_PROCESS_CHECKFOR_NEW_PACKAGE,
        ACTOR_SET_PATHFINDING_GOAL,
        PLAYER_FLAGS_680,
        REFERENCE_GET_DISTANCE_FROM_REFERENCE,
        ACTOR_CALL_008BB520,
        PACKAGE_WORD_84,
        PACKAGE_FLOAT_80,
        PACKAGE_ADD_FLOAT_80,
        ACTOR_CALL_008C0EC0,
        FLOAT_C,
        PLAYER_IS_SLEEPING_OR_RESTING,
        PLAYER_SET_654,
        ACTOR_BYTE_7C4,
        FLEE_ADD_AVOIDED_REF,
        MAGIC_ITEM_IS_LOADED,
        MAGIC_ITEM_IS_LOADING,
        MAGIC_ITEM_PRELOAD,
        MAGIC_ITEM_UNLOAD,
        MAGIC_CASTER_CAST_SPELL,
        FLEE_UPDATE,
        PACKAGE_TARGET_GET_REFERENCE,
        FLEE_FLOAT_90,
        FLEE_ADD_FLOAT_90,
        FLEE_SET_BYTE_94,
        FLEE_GET_CLOSEST_AVOIDED_REF,
        FLEE_DOOR,
        FLEE_FIND_TELEPORT_DOOR,
        PATHING_REQUEST_FLEE_CONSTRUCT,
        PATHING_REQUEST_FROM_ACTOR,
        PATHING_REQUEST_SET_90,
        PATHING_REQUEST_SET_98,
        PATHING_REQUEST_SET_74,
        PATHING_REQUEST_006D61E0,
        PATHING_REQUEST_SET_B0,
        PATHING_REQUEST_SET_LOCATION,
        PATHING_REQUEST_008DF000,
        PATHING_REQUEST_DESTRUCT,
        ACTOR_EXTRA_OBJECT,
        FLEE_PATH_MODE,
        PATHING_LOCATION_CONSTRUCT,
        PATHING_LOCATION_DESTRUCT,
        ACTOR_SET_PATHFINDING_FLEE,
        PACKAGE_FLAG_10000,
        PLAYER_TEST_00962720,
        PLAYER_COUNT_00962620,
        PACKAGE_CALL_00674F30,
        PACKAGE_GET_RADIUS_TO_TARGET,
        LOCATION_GET_TYPE,
        ACTOR_SET_MOVE_MODE,
        ENUM_REFERENCES_CLOSE_TO_POINT,
        TRAVEL_SEARCH_CALLBACK_A,
        TRAVEL_SEARCH_CALLBACK_B,
        PACKAGE_GET_RADIUS_TO_LOCATION,
        PACKAGE_FLAG_20,
        PACKAGE_FLAG_100,
        ACTOR_QUEUE_UNEQUIP_OBJECT,
        MARK_HEADING,
        MARK_BYTE_E,
        REFERENCE_GET_SCALE,
        FURNITURE_GET_MARKER_TARGET_OFFSET,
        ROTATION_FROM_HEADING,
        ROTATE_POINT,
        REFERENCE_SET_MARKER_USED,
        POINT_ADD,
        FURNITURE_MARKER_ANGLE,
        MOBILE_TURN_BY,
        ACTOR_GET_FACE_ANIMATION_DATA,
        REFERENCE_PLUS_24,
        PACKAGE_WORD_18,
        PROCEDURE_GET_COUNT,
        FORM_FLAG_800,
        CELL_STATE_IS_3,
        BYTE_18A_UPDATE,
        ACTOR_CALL_008A8F60,
        SCRIPT_SET_ACTION_FLAG,
        PACKAGE_PLUS_38,
        PACKAGE_GET_PATROL_DATA,
        PATROL_BYTE_4,
        PACKAGE_GET_ONCE_PER_DAY,
        CALENDAR_GET_DAY,
        INTERFACE_GET_TARGET_REFR,
        ACTOR_IS_WAITING_ON_PATH,
        PACKAGE_IS_TARGET_AN_ACTOR,
        PACKAGE_ACTOR_TARGET,
        PATROL_GET_CURRENT_REFERENCE,
        PATROL_GET_INITIAL_REFERENCE,
        DIALOGUE_GET_TARGET_OF_CONVERSATION,
        FORM_FLAG_200000,
        TARGET_GET_TYPE,
        TARGET_GET_OBJECT,
        TARGET_GET_OBJECT_TYPE,
        PACKAGE_SET_NEVER_RUN,
        REFERENCE_SELF_TARGET,
        FORM_LIST_ENTRIES,
        BUILD_MULTI_TARGET_LIST,
        EMPTY_FUNCTION_1,
        ACQUIRE_SEARCH_CALLBACK,
        PACKAGE_GET_SEARCH_LOCATION,
        PACKAGE_GET_SEARCH_RADIUS,
        PACKAGE_GET_SEARCH_CELL,
        PACKAGE_GET_SEARCH_COORD,
        PACKAGE_GET_ACQUIRE_RADIUS,
        PACKAGE_GET_SHOULD_RESERVE_TARGET,
        GET_ACTOR_REF_IN_HIGH,
        REFERENCE_GET_FIRST_FREE_MARKER,
        NODE_SET_ITEM,
        PACKAGE_FLAG2_100,
        PACKAGE_FLAG2_400,
        PACKAGE_FLAG2_200,
        BYTE_18D,
        FORM_VALUE,
        ACTOR_GET_GOLD_AMOUNT,
        AI_FORM_SELL_BUYS_ITEM,
        ACTOR_HAS_OBJECTS,
        REFERENCE_GET_LOCK,
        REFERENCE_GET_REF_PERSISTS,
        PROCESS_LEVEL,
        MAGIC_CASTER_CAST_ABILITY,
        MAGIC_CASTER_TRANSFER_DISEASE,
        REFERENCE_GET_INVENTORY_ITEM,
    ];

    const GLOBALS: &[u32] = &[
        MODEL_LOADER,
        OBJECT_GROUP_TABLE,
        DATA_HANDLER,
        SAVE_LOAD_GAME,
        FURNITURE_MARK_ORIGIN,
        MINUS_ONE,
        TES_SAVE_LOAD_GAME,
        TASK_MANAGER,
        PLAYER,
        ACTIVATE_FORM_A,
        ACTIVATE_FORM_B,
        COMBAT_CHECK_OBJECT,
        ONE_DOUBLE,
        DEFAULT_RADIUS,
        ZERO_DOUBLE,
        ACCOMPANY_SECONDS,
        PURSUE_FOLLOW_DISTANCE,
        MINUS_ONE_DOUBLE,
        FLEE_PATH_RANGE,
        TRAVEL_SEARCH_RADIUS,
        TRAVEL_SMALL_RADIUS,
        PROCEDURE_TABLES,
        TWO_DOUBLE,
        FLOAT_MAX,
        LOCK_KEY_FORM,
        ACTIVATE_DOOR_DISTANCE,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    /// Every callee of this file a no-op returning 0 (the trivial list and
    /// word getters behave as in the game), the fake vtable slots likewise,
    /// the globals mapped with the game's constant values, calls logged.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        for base in [PROCESS_SLOTS, ACTOR_SLOTS, OBJECT_SLOTS] {
            for slot in (0..VTABLE_SIZE).step_by(4) {
                e.register(base + slot, |_, _| Ret::default());
            }
        }
        e.register(IDENTITY, |_, a| ret(a[0]));
        for addr in [
            PACKAGE_GET_LOCATION_COORD,
            POINT_SUBTRACT,
            POINT_ADD,
            ROTATE_POINT,
            PACKAGE_GET_SEARCH_COORD,
        ] {
            e.register(addr, |_, a| ret(a[1]));
        }
        e.register(WORD_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        for &addr in GLOBALS {
            e.map(addr, 8);
        }
        // Null pointers read zeros here (the game would fault); the tests
        // check calls, not those reads.
        e.map(0, 0x1000);
        e.map(TEST_PLAYER, 0x800);
        e.set_global(PLAYER, TEST_PLAYER);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(PURSUE_FOLLOW_DISTANCE, 200.0f64);
        e.set_global(ACTIVATE_DOOR_DISTANCE, 128.0f64);
        e.set_global(DEFAULT_RADIUS, 600.0f32);
        e.set_global(ACCOMPANY_SECONDS, 3600.0f32);
        e.set_global(FLEE_PATH_RANGE, 1000.0f32);
        e.set_global(TRAVEL_SEARCH_RADIUS, 5000.0f32);
        e.set_global(TRAVEL_SMALL_RADIUS, 20.0f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32, vtable: u32, base: u32) -> u32 {
        let slots: Vec<u32> = (0..VTABLE_SIZE).step_by(4).map(|s| base + s).collect();
        e.put_vtable(vtable, &slots);
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, vtable);
        p
    }

    fn process(e: &mut Engine) -> Ptr<MiddleHighProcess> {
        Ptr::new(object(e, 0x25c, 0x0500_0000, PROCESS_SLOTS))
    }

    fn actor(e: &mut Engine) -> u32 {
        object(e, 0x200, 0x0501_0000, ACTOR_SLOTS)
    }

    fn thing(e: &mut Engine) -> u32 {
        object(e, 0x100, 0x0502_0000, OBJECT_SLOTS)
    }

    /// The argument words of each logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn count(e: &Engine, addr: u32) -> usize {
        calls(e, addr).len()
    }

    /// A one-entry list node holding `item`.
    fn list_of(e: &mut Engine, item: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        node
    }

    #[test]
    fn get_furniture_marker_is_the_address_of_the_mark() {
        let mut e = engine();
        let p = process(&mut e);
        assert_eq!(
            middle_high_process_get_furniture_marker(&mut e, p),
            p.addr() + 0x148
        );
    }

    #[test]
    fn process_notify_sets_the_target_only_without_one() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_process_notify(&mut e, p, 0x55);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x8c), vec![vec![p.addr(), 0x55]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x288),
            vec![vec![p.addr(), 0x55, 1]]
        );
        e.mem.set_u32(p.addr() + 0x40, 7);
        middle_high_process_process_notify(&mut e, p, 0x55);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x8c), 1);
    }

    #[test]
    fn queued_item_constructor_clears_the_item() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        e.mem.set_u32(item.addr(), 9);
        e.mem.set_u8(item.addr() + 0xc, 1);
        e.mem.set_u8(item.addr() + 0x11, 1);
        assert_eq!(fn_009139b0(&mut e, item), item);
        assert_eq!(e.mem.u32(item.addr()), 0);
        assert_eq!(e.mem.u8(item.addr() + 0xc), 0);
        assert_eq!(e.mem.u8(item.addr() + 0x11), 0);
        assert_eq!(
            calls(&e, QUEUED_FILE_POINTER_INIT),
            vec![vec![item.addr() + 0x14, 0]]
        );
    }

    #[test]
    fn preload_queues_weapon_files_and_missing_groups() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        let weapon = e.mem.alloc(8);
        e.mem.set_u8(weapon + 4, 0x28);
        e.mem.set_u32(item.addr(), weapon);
        e.mem.set_u8(item.addr() + 0xc, 1);
        let a = actor(&mut e);
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(QUEUED_FILE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ACTOR_SLOTS + 0x21c, |_, _| ret(1));
        e.register(OBJECT_GROUP_INDEX, |_, _| ret(2));
        e.mem.set_u32(OBJECT_GROUP_TABLE + 8, 0x31);
        e.register(BUILD_KF_FILE_LIST, |_, _| ret(0x777));
        let process_object = thing(&mut e);
        e.register_double(WORD_68, move |_, _| ret(process_object));
        assert!(fn_00913a20(&mut e, item, a));
        assert_eq!(calls(&e, QUEUE_OBJECT_FILES)[0][4], a);
        assert_eq!(calls(&e, BUILD_KF_FILE_LIST)[0][4], 0x31);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![0x777, 1]]);
        // Unequipping never reports true.
        e.mem.set_u8(item.addr() + 0xc, 0);
        assert!(!fn_00913a20(&mut e, item, a));
        assert!(!fn_00913a20(&mut e, item, 0));
    }

    #[test]
    fn queued_item_save_writes_the_extra_list_index() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        e.mem.set_u32(item.addr(), 0x40);
        e.mem.set_u32(item.addr() + 4, 0x50);
        let buffer = thing(&mut e);
        let reference = thing(&mut e);
        e.register_double(OBJECT_SLOTS + 8, move |_, _| ret(reference));
        e.register(GET_CONTAINER_CHANGES, |_, _| ret(0x60));
        e.register(EXTRA_LIST_SAVE_INDEX, |_, _| ret(3));
        queued_item_save_game(&mut e, item, buffer);
        assert_eq!(calls(&e, SAVE_FORM_ID), vec![vec![buffer, 0x40, 0]]);
        assert_eq!(
            calls(&e, EXTRA_LIST_SAVE_INDEX),
            vec![vec![0x60, 0x40, 0x50]]
        );
        let saved = calls(&e, SAVE_DATA);
        assert_eq!(saved.len(), 8);
        assert_eq!(saved[1], vec![buffer, item.addr() + 8, 4, 0]);
        assert_eq!(saved[7], vec![buffer, item.addr() + 0x11, 1, 0]);
    }

    #[test]
    fn queued_item_load_resolves_the_extra_list() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        let buffer = thing(&mut e);
        let reference = thing(&mut e);
        e.register_double(OBJECT_SLOTS + 0xc, move |_, _| ret(reference));
        e.register(LOOKUP_FORM, |_, _| ret(0x44));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 5);
            }
            Ret::default()
        });
        e.register(GET_CONTAINER_CHANGES, |_, _| ret(0x60));
        e.register(EXTRA_LIST_FROM_INDEX, |_, _| ret(0x70));
        queued_item_load_game(&mut e, item, buffer);
        assert_eq!(e.mem.u32(item.addr()), 0x44);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![0x44, 0, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0]
        );
        assert_eq!(calls(&e, EXTRA_LIST_FROM_INDEX), vec![vec![0x60, 0x44, 5]]);
        assert_eq!(e.mem.u32(item.addr() + 4), 0x70);
        assert_eq!(count(&e, LOAD_DATA), 8);
    }

    #[test]
    fn preload_from_source_asks_its_slot() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        let source = thing(&mut e);
        fn_00913fb0(&mut e, item, source);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xc), vec![vec![source]]);
    }

    #[test]
    fn constructor_sets_the_vtable_and_start_values() {
        let mut e = engine();
        let p = Ptr::<MiddleHighProcess>::new(e.mem.alloc(0x25c));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CACHED_VALUES_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.mem.set_u32(MINUS_ONE, 0xbf80_0000);
        assert_eq!(fn_00913fe0(&mut e, p), p);
        let base = p.addr();
        assert_eq!(e.mem.u32(base), MIDDLE_HIGH_PROCESS_VTABLE);
        assert_ne!(e.mem.u32(base + 0x2c), 0);
        assert_ne!(e.mem.u32(base + 0x1b8), 0);
        assert_eq!(e.mem.f32(base + 0x170), 1.0);
        assert_eq!(e.mem.f32(base + 0x198), -1.0);
        assert_eq!(e.mem.u32(base + 0x1a8), u32::MAX);
        assert_eq!(e.mem.u8(base + 0x144), 0x7f);
        assert_eq!(e.mem.u16(base + 0x22a), 2);
        assert_eq!(
            calls(&e, MEMSET),
            vec![vec![base + 0x1c4, 0, 0xc], vec![base + 0x1dc, 0, 0x3c]]
        );
        assert_eq!(calls(&e, SET_WORD_28), vec![vec![base, 1]]);
    }

    #[test]
    fn radiation_delta_reads_the_field() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_f32(p.addr() + 0x234, 2.5);
        assert_eq!(middle_high_process_get_radiation_delta(&mut e, p), 2.5);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_scalar_deleting_destructor(&mut e, p, 0);
        assert_eq!(count(&e, FREE), 0);
        assert_eq!(count(&e, MIDDLE_LOW_DESTRUCT), 1);
        middle_high_process_scalar_deleting_destructor(&mut e, p, 1);
        assert_eq!(calls(&e, FREE), vec![vec![p.addr()]]);
    }

    #[test]
    fn destructor_deletes_owned_objects() {
        let mut e = engine();
        let p = process(&mut e);
        let base = p.addr();
        e.mem.set_u32(base + 0x2c, 0x1234);
        e.mem.set_u32(base + 0x114, 0x2000);
        e.mem.set_u32(base + 0x1a0, 0x3000);
        let effect = thing(&mut e);
        let effects = list_of(&mut e, effect);
        e.mem.set_u32(base + 0x1b8, effects);
        middle_high_process_destructor(&mut e, p);
        assert_eq!(calls(&e, FREE)[0], vec![0x1234]);
        assert_eq!(e.mem.u32(base + 0x2c), 0);
        assert_eq!(calls(&e, OBJECT_SLOTS), vec![vec![effect, 1]]);
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![0x2000, 1]]);
        assert_eq!(calls(&e, LIP_SYNCH_ANIM_DELETE), vec![vec![0x3000, 1]]);
        assert_eq!(calls(&e, CLEAR_RUN_ONCE_PACKAGE).len(), 0);
        assert_eq!(calls(&e, MIDDLE_LOW_DESTRUCT), vec![vec![base]]);
    }

    #[test]
    fn deferred_limb_deleting_destructor_releases_both_pointers() {
        let mut e = engine();
        let limb = Ptr::new(e.mem.alloc(0x14));
        fn_00914ab0(&mut e, limb, 1);
        assert_eq!(
            calls(&e, POINTER_RELEASE),
            vec![vec![limb.addr() + 0xc], vec![limb.addr() + 8]]
        );
        assert_eq!(calls(&e, FREE), vec![vec![limb.addr()]]);
    }

    #[test]
    fn deferred_limb_destructor_releases_in_reverse_order() {
        let mut e = engine();
        let limb = Ptr::new(e.mem.alloc(0x14));
        fn_00914ae0(&mut e, limb);
        assert_eq!(calls(&e, POINTER_RELEASE)[0], vec![limb.addr() + 0xc]);
    }

    #[test]
    fn clear_reservation_frees_both_markers() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_u32(p.addr() + 0x40, 0x100);
        e.mem.set_u32(p.addr() + 0x1a8, 2);
        e.mem.set_u32(p.addr() + 0x140, 0x200);
        e.mem.set_u8(p.addr() + 0x144, 3);
        middle_high_process_clear_reservation(&mut e, p);
        assert_eq!(
            calls(&e, SET_MARKER_RESERVED),
            vec![vec![0x100, 2, 0], vec![0x200, 3, 0]]
        );
        assert_eq!(e.mem.u32(p.addr() + 0x1a8), u32::MAX);
        e.register(DATA_HANDLER_BYTE_61D, |_, _| ret(1));
        middle_high_process_clear_reservation(&mut e, p);
        assert_eq!(count(&e, SET_MARKER_RESERVED), 2);
    }

    #[test]
    fn handle_deleted_reference_reaches_both_packages() {
        let mut e = engine();
        let p = process(&mut e);
        let data = thing(&mut e);
        e.register_double(PROCESS_SLOTS + 0x270, move |_, _| ret(data));
        middle_high_process_handle_deleted_reference(&mut e, p, 0x99);
        assert_eq!(
            calls(&e, PACKAGE_HANDLE_DELETED),
            vec![vec![p.addr() + 4, 0x99], vec![p.addr() + 0xe4, 0x99]]
        );
        assert_eq!(calls(&e, OBJECT_SLOTS + 4), vec![vec![data, 0x99]]);
    }

    #[test]
    fn middle_high_copy_moves_the_state() {
        let mut e = engine();
        let p = process(&mut e);
        let source = process(&mut e).addr();
        e.mem.set_u32(source + 0x1c0, 0x500);
        e.mem.set_u32(source + 0x114, 0x600);
        e.mem.set_u32(source + 0x140, 0x700);
        e.mem.set_f32(source + 0x234, 1.5);
        e.register(PROCESS_SLOTS + 0x44c, |_, _| ret(1));
        e.register(LIST_ADD, |_, _| Ret::default());
        middle_high_process_middle_high_copy(&mut e, p, source);
        let base = p.addr();
        assert_eq!(e.mem.u32(base + 0x1c0), 0x500);
        assert_eq!(e.mem.u32(source + 0x1c0), 0);
        assert_eq!(e.mem.u32(base + 0x114), 0x600);
        assert_eq!(e.mem.u32(source + 0x114), 0);
        assert_eq!(e.mem.u8(base + 0x134), 1);
        assert_eq!(e.mem.f32(base + 0x234), 1.5);
        assert_eq!(calls(&e, ANIMATION_CLEAR_GROUP), vec![vec![0x500, 1, 0]]);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x700, 1]]);
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_MOVE_TO),
            vec![vec![source + 0xe4, base + 0xe4]]
        );
    }

    #[test]
    fn setup_new_package_resets_the_procedure_state() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_u32(p.addr() + 0x34, 5);
        middle_high_process_setup_new_package(&mut e, p);
        assert_eq!(e.mem.u32(p.addr() + 0x34), 0);
        assert_eq!(e.mem.u16(p.addr() + 0x22a), 2);
        assert_eq!(calls(&e, SET_PROCESS_FLAG_40), vec![vec![p.addr(), 0]]);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x330), 1);
    }

    /// A queue (+0x230) holding one new item.
    fn queue_one(e: &mut Engine, p: Ptr<MiddleHighProcess>, object: u32, equip: u8) -> u32 {
        let item = e.mem.alloc(0x18);
        e.mem.set_u32(item, object);
        e.mem.set_u32(item + 8, 1);
        e.mem.set_u8(item + 0xc, equip);
        let list = list_of(e, item);
        e.mem.set_u32(p.addr() + 0x230, list);
        item
    }

    #[test]
    fn resolve_equipment_queue_equips_and_drops_the_list() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let weapon = e.mem.alloc(8);
        let item = queue_one(&mut e, p, weapon, 1);
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        middle_high_process_resolve_equipment_queue(&mut e, p, a);
        assert_eq!(
            calls(&e, ACTOR_EQUIP_OBJECT),
            vec![vec![a, weapon, 1, 0, 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, QUEUED_FILE_POINTER_RELEASE),
            vec![vec![item + 0x14]]
        );
        assert_eq!(calls(&e, FREE), vec![vec![item]]);
        assert_eq!(count(&e, LIST_DELETE), 1);
        assert_eq!(e.mem.u32(p.addr() + 0x230), 0);
    }

    #[test]
    fn resolve_equipment_queue_waits_for_loading_files() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let item = queue_one(&mut e, p, 0x40, 0);
        e.mem.set_u32(item + 0x14, 0x77);
        middle_high_process_resolve_equipment_queue(&mut e, p, a);
        assert_eq!(calls(&e, QUEUED_FILE_DONE), vec![vec![0x77]]);
        assert_eq!(count(&e, ACTOR_UNEQUIP_OBJECT), 0);
        assert_ne!(e.mem.u32(p.addr() + 0x230), 0);
    }

    #[test]
    fn queued_item_deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        fn_00915790(&mut e, item, 1);
        assert_eq!(
            calls(&e, QUEUED_FILE_POINTER_RELEASE),
            vec![vec![item.addr() + 0x14]]
        );
        assert_eq!(calls(&e, FREE), vec![vec![item.addr()]]);
    }

    #[test]
    fn queued_item_destructor_releases_the_file() {
        let mut e = engine();
        let item: Ptr<QueuedItem> = e.new_object();
        fn_009157c0(&mut e, item);
        assert_eq!(
            calls(&e, QUEUED_FILE_POINTER_RELEASE),
            vec![vec![item.addr() + 0x14]]
        );
    }

    #[test]
    fn is_item_queued_matches_object_extra_and_flag() {
        let mut e = engine();
        let p = process(&mut e);
        let item = queue_one(&mut e, p, 0x40, 1);
        e.mem.set_u32(item + 4, 0x50);
        assert!(middle_high_process_is_item_queued(&mut e, p, 0x40, 0x50, 1));
        assert!(!middle_high_process_is_item_queued(
            &mut e, p, 0x40, 0x50, 0
        ));
        assert!(!middle_high_process_is_item_queued(
            &mut e, p, 0x41, 0x50, 1
        ));
    }

    #[test]
    fn remove_queued_item_cancels_and_deletes() {
        let mut e = engine();
        let p = process(&mut e);
        let item = queue_one(&mut e, p, 0x40, 1);
        e.mem.set_u32(item + 0x14, 0x77);
        let list = e.mem.u32(p.addr() + 0x230);
        middle_high_process_remove_queued_item(&mut e, p, 0x40, 0);
        assert_eq!(calls(&e, TRY_CANCEL_TASK), vec![vec![0, 0x77, 0]]);
        assert_eq!(calls(&e, FREE), vec![vec![item]]);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], list);
    }

    #[test]
    fn add_queued_item_builds_and_appends() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        middle_high_process_add_queued_item(&mut e, p, 0, 1, 0x40, 2, 0x50, 1, 0, 9, 1, 0, 1);
        let list = e.mem.u32(p.addr() + 0x230);
        let item = e.mem.u32(list);
        assert_eq!(e.mem.u32(item), 0x40);
        assert_eq!(e.mem.u32(item + 8), 2);
        assert_eq!(e.mem.u32(item + 4), 0x50);
        assert_eq!(e.mem.u8(item + 0xc), 1);
        assert_eq!(e.mem.u8(item + 0x10), 1);
        assert_eq!(e.mem.u8(item + 0xf), 1);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x174),
            vec![vec![p.addr(), 0x40, 0x50, 1]]
        );
        assert_eq!(
            calls(&e, SCOPE_ENTER)[0][1..].to_vec(),
            vec![0x32, 1, SOURCE_FILE, 0x343]
        );
        assert_eq!(count(&e, SCOPE_LEAVE), 1);
    }

    #[test]
    fn remove_all_queued_items_empties_the_queue() {
        let mut e = engine();
        let p = process(&mut e);
        let item = queue_one(&mut e, p, 0x40, 1);
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        middle_high_process_remove_all_queued_items(&mut e, p);
        assert_eq!(calls(&e, FREE), vec![vec![item]]);
        assert_eq!(count(&e, LIST_DELETE), 1);
        assert_eq!(e.mem.u32(p.addr() + 0x230), 0);
    }

    #[test]
    fn process_remove_worn_resets_an_npc_outfit() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[3] == TYPE_TES_NPC { 0x900 } else { 0 })
        });
        let package = thing(&mut e);
        e.register_double(PROCESS_SLOTS + 0x22c, move |_, _| ret(package));
        e.register(PACKAGE_TYPE, |_, _| ret(4));
        middle_high_process_process_remove_worn(&mut e, p, a, 1, 0);
        assert_eq!(
            calls(&e, NPC_INIT_DEFAULT_WORN),
            vec![vec![0x900, a, 0, 0, 1, 1]]
        );
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }

    #[test]
    fn want_weapon_drawn_reads_its_byte() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_u8(p.addr() + 0x134, 1);
        assert_eq!(middle_high_process_get_want_weapon_drawn(&mut e, p), 1);
    }

    #[test]
    fn set_want_weapon_drawn_stores_its_byte() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_set_want_weapon_drawn(&mut e, p, 1);
        assert_eq!(e.mem.u8(p.addr() + 0x134), 1);
    }

    #[test]
    fn weapon_drawn_reads_its_byte() {
        let mut e = engine();
        let p = process(&mut e);
        e.mem.set_u8(p.addr() + 0x135, 1);
        assert_eq!(middle_high_process_get_weapon_drawn(&mut e, p), 1);
    }

    #[test]
    fn set_weapon_drawn_sheathes_and_draws() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.mem.set_u8(p.addr() + 0x135, 1);
        e.register(ACTOR_SLOTS + 0x1e4, |_, _| ret(0x11));
        e.register(ACTOR_SLOTS + 0x1e8, |_, _| ret(0x22));
        middle_high_process_set_weapon_drawn(&mut e, p, a, 0);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x1cc),
            vec![vec![p.addr(), 0, 0x22, 0x11, a]]
        );
        e.mem.set_u32(p.addr() + 0x114, 0x300);
        e.register(ITEM_CHANGE_FORM, |_, _| ret(0x44));
        e.register(ITEM_CHANGE_MOD_EFFECT_ACTIVE, |_, _| ret(1));
        middle_high_process_set_weapon_drawn(&mut e, p, a, 1);
        assert_eq!(calls(&e, ACTOR_SLOTS + 0x3ec), vec![vec![a, 0x44, 0, 1, 0]]);
        assert_eq!(count(&e, ACTOR_UPDATE_DRAWN_FLAGS), 2);
    }

    #[test]
    fn set_furniture_copies_the_mark_and_reserves() {
        let mut e = engine();
        let p = process(&mut e);
        let mark = e.mem.alloc(0x10);
        e.mem.set_u32(mark, 7);
        e.mem.set_u16(mark + 0xc, 0x1234);
        e.mem.set_u8(mark + 0xe, 5);
        middle_high_process_set_furniture(&mut e, p, 0x600, 2, mark);
        assert_eq!(e.mem.u32(p.addr() + 0x140), 0x600);
        assert_eq!(e.mem.u32(p.addr() + 0x148), 7);
        assert_eq!(e.mem.u16(p.addr() + 0x154), 0x1234);
        assert_eq!(e.mem.u8(p.addr() + 0x156), 5);
        assert_eq!(calls(&e, SET_MARKER_RESERVED), vec![vec![0x600, 2, 1]]);
    }

    #[test]
    fn byte_70_of_the_actor_is_passed_on() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(ACTOR_BYTE_70, |_, _| ret(1));
        assert_eq!(fn_00915ef0(&mut e, p, 0x40), 1);
        assert_eq!(calls(&e, ACTOR_BYTE_70), vec![vec![0x40]]);
    }

    /// A process whose running package (slot 0x27C) and current package
    /// (slot 0x22C) are a fresh object; returns the package.
    fn with_package(e: &mut Engine) -> u32 {
        let package = thing(e);
        e.register_double(PROCESS_SLOTS + 0x27c, move |_, _| ret(package));
        e.register_double(PROCESS_SLOTS + 0x22c, move |_, _| ret(package));
        package
    }

    #[test]
    fn process_activate_without_target_advances() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        assert!(!middle_high_process_process_activate(&mut e, p, a, 0));
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x8c), vec![vec![p.addr(), a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }

    #[test]
    fn process_activate_activates_a_close_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        let target = thing(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        e.mem.set_u32(p.addr() + 0x64, 0);
        e.register(OBJECT_SLOTS + 0x144, |_, _| ret(1));
        assert!(middle_high_process_process_activate(&mut e, p, a, 0));
        assert_eq!(calls(&e, OBJECT_SLOTS + 0x144), vec![vec![package, a, 0]]);
        assert_eq!(
            calls(&e, REFERENCE_ACTIVATE),
            vec![vec![target, a, 0, 0, 1]]
        );
        assert_eq!(calls(&e, REFERENCE_RUN_SCRIPT), vec![vec![target]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x12c), vec![vec![p.addr(), 0]]);
    }

    #[test]
    fn enter_combat_starts_combat_with_the_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let target = actor(&mut e);
        e.mem.set_u32(p.addr() + 0x64, 0x80);
        assert!(middle_high_process_enter_combat(
            &mut e, p, a, target, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0
        ));
        assert_eq!(
            calls(&e, ACTOR_SLOTS + 0x424),
            vec![vec![a, target, 0, 0, 1, 0, 0, 0, 0]]
        );
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x1c8), vec![vec![p.addr(), 1]]);
        assert_eq!(e.mem.u32(p.addr() + 0x68), 0x80);
        assert_eq!(e.mem.u32(p.addr() + 0x64), 0);
    }

    #[test]
    fn enter_combat_is_refused_by_slot_324() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(PROCESS_SLOTS + 0x324, |_, _| ret(1));
        assert!(!middle_high_process_enter_combat(
            &mut e, p, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ));
        assert_eq!(count(&e, ACTOR_SLOTS + 0x424), 0);
    }

    #[test]
    fn process_escort_without_item_or_target_advances() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        middle_high_process_process_escort(&mut e, p, a);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x294), vec![vec![p.addr(), a]]);
    }

    #[test]
    fn process_escort_paths_to_the_location() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        let point = e.mem.alloc(12);
        e.register_double(ACTOR_SLOTS + 0x1f4, move |_, _| ret(point));
        e.register(ACTOR_ITEM_CHANGE, |_, _| ret(0x10));
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| ret(1));
        e.register_double(PACKAGE_GET_LOCATION_COORD, move |_, _| ret(point));
        e.register(PACKAGE_GET_LOCATION_WORLD, |_, _| ret(0x31));
        e.register(PACKAGE_GET_LOCATION_CELL, |_, _| ret(0x32));
        middle_high_process_process_escort(&mut e, p, a);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x824),
            vec![vec![p.addr(), a, 0, 0, 0, 0x32, 0x31]]
        );
    }

    #[test]
    fn accompany_one_hour_follows_for_an_hour() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        middle_high_process_process_accompany_one_hour(&mut e, p, a);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x8c), vec![vec![p.addr(), a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }

    #[test]
    fn accompany_follows_through_slot_298() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_process_accompany(&mut e, p, 0x40);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x298),
            vec![vec![p.addr(), 0x40, 1, 0x101, 0]]
        );
    }

    #[test]
    fn process_sleep_takes_an_owned_bed() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(ACTOR_SLOTS + 0x218, |_, _| ret(1));
        e.mem.set_u32(p.addr() + 0xd0, 0x700);
        e.register(LIST_COUNT, |_, _| ret(1));
        e.register(TRY_ENTER_LOCK, |_, _| ret(1));
        e.register(REFERENCE_GET_OWNER, |_, _| ret(0x5));
        middle_high_process_process_sleep(&mut e, p, a);
        assert_eq!(e.mem.u32(p.addr() + 0x140), 0x700);
        assert_eq!(
            calls(&e, LIST_REMOVE),
            vec![vec![p.addr() + 0xd0, p.addr() + 0x140]]
        );
        assert_eq!(calls(&e, LEAVE_LOCK), vec![vec![FURNITURE_LOCK]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x7c8), vec![vec![p.addr(), a, 0]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x44),
            vec![vec![p.addr(), a, 0, 2, 1, 0, 1]]
        );
    }

    #[test]
    fn process_eat_eats_when_done() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let food = e.mem.alloc(8);
        e.register_double(ACTOR_GET_BEST_FOOD_ITEM, move |_, _| ret(food));
        e.register(ITEM_CHANGE_FORM, |_, _| ret(0x44));
        middle_high_process_process_eat(&mut e, p, a, 0);
        assert_eq!(calls(&e, ACTOR_EAT), vec![vec![a, 0x44, 0, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x118), vec![vec![p.addr(), 1]]);
        e.register(PROCESS_SLOTS + 0x11c, |_, _| ret(1));
        middle_high_process_process_eat(&mut e, p, a, 0);
        assert_eq!(count(&e, ACTOR_EAT), 1);
    }

    #[test]
    fn checkfor_new_package_resets_on_a_new_package() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(LOW_PROCESS_CHECKFOR_NEW_PACKAGE, |_, _| ret(1));
        e.register(FTOL, |_, _| ret(7));
        assert!(middle_high_process_checkfor_new_package(&mut e, p, a, 1));
        assert_eq!(e.mem.u32(p.addr() + 0xb4), 7);
        assert_eq!(e.mem.u16(p.addr() + 0x22a), 2);
        assert_eq!(
            calls(&e, LOW_PROCESS_CHECKFOR_NEW_PACKAGE),
            vec![vec![p.addr(), a, 1]]
        );
        e.register(ACTOR_IS_ALARMED, |_, _| ret(1));
        assert!(!middle_high_process_checkfor_new_package(&mut e, p, a, 1));
    }

    #[test]
    fn setup_path_finding_passes_the_point_by_address() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(ACTOR_SET_PATHFINDING_GOAL, |e, a| {
            ret(u32::from(e.mem.f32(a[1] + 4) == 2.0 && a[4] == 0xbf80_0000))
        });
        e.mem.set_u32(MINUS_ONE, 0xbf80_0000);
        assert!(middle_high_process_setup_path_finding(
            &mut e, p, a, 1.0, 2.0, 3.0, 5, 6
        ));
        assert_eq!(
            calls(&e, ACTOR_SET_PATHFINDING_GOAL)[0][2..4].to_vec(),
            vec![5, 6]
        );
        e.register(ACTOR_SLOTS + 0x214, |_, _| ret(4));
        assert!(!middle_high_process_setup_path_finding(
            &mut e, p, a, 1.0, 2.0, 3.0, 5, 6
        ));
        assert_eq!(count(&e, ACTOR_SLOTS + 0x418), 1);
    }

    #[test]
    fn process_pursue_follows_a_far_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let target = actor(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(250.0));
        middle_high_process_process_pursue(&mut e, p, a, 0, 0, 0x101);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x298),
            vec![vec![p.addr(), a, 0, 0x101, 1]]
        );
        assert_eq!(count(&e, PROCESS_SLOTS + 0x294), 0);
    }

    #[test]
    fn process_pursue_attacks_an_alarmed_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let target = actor(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[3] == TYPE_ACTOR { a[0] } else { 0 })
        });
        e.register(ACTOR_IS_ALARMED, |_, _| ret(1));
        middle_high_process_process_pursue(&mut e, p, a, 0, 0, 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 2]]);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x33c),
            vec![vec![p.addr(), a, target, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn word_94_is_read() {
        let mut e = engine();
        let object = e.mem.alloc(0x98);
        e.mem.set_u32(object + 0x94, 3);
        assert_eq!(fn_009185e0(&mut e, object), 3);
    }

    #[test]
    fn flee_non_combat_flees_a_near_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        e.mem.set_u32(p.addr() + 0x40, 0x300);
        e.register(PACKAGE_RADIUS, |_, _| ret(100));
        e.register(REFERENCE_GET_DISTANCE_FROM_REFERENCE, |_, _| ret_f(150.0));
        middle_high_process_process_flee_non_combat(&mut e, p, a);
        assert_eq!(calls(&e, PACKAGE_RADIUS), vec![vec![package, a]]);
        assert_eq!(
            calls(&e, ACTOR_SLOTS + 0x410),
            vec![vec![a, a, 1, 1, 1, 0, 0, 0xbf80_0000, 0xbf80_0000]]
        );
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x90), vec![vec![p.addr(), a, 0]]);
    }

    #[test]
    fn flee_casts_a_loaded_package_spell() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(PROCESS_SLOTS + 0x4c, |_, _| ret(0x500));
        e.register(MAGIC_ITEM_IS_LOADED, |_, _| ret(1));
        middle_high_process_process_flee(&mut e, p, a);
        assert_eq!(
            calls(&e, MAGIC_CASTER_CAST_SPELL),
            vec![vec![a + 0x88, 0x500, a + 0x94, 0]]
        );
        assert_eq!(calls(&e, MAGIC_ITEM_UNLOAD), vec![vec![0x500, 0]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x50), vec![vec![p.addr(), 0]]);
    }

    #[test]
    fn flee_without_avoided_reference_runs_the_time() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(DYNAMIC_CAST, |_, _| ret(0x600));
        e.register(PACKAGE_WORD_30, |_, _| ret(0x610));
        e.register(PACKAGE_TARGET_GET_REFERENCE, |_, _| ret(0x620));
        e.register(FLEE_FLOAT_90, |_, _| ret_f(5.0));
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 4.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(limit));
        middle_high_process_process_flee(&mut e, p, a);
        assert_eq!(calls(&e, FLEE_SET_BYTE_94), vec![vec![0x600, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }

    #[test]
    fn follow_paths_after_a_moving_target() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        let target = thing(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 9.0);
        e.register_double(OBJECT_SLOTS + 0x1f4, move |_, _| ret(point));
        e.register(POINT_LENGTH, |_, _| ret_f(500.0));
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 100.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(limit));
        e.register(PROCESS_SLOTS + 0x824, |_, _| ret(1));
        middle_high_process_process_follow(&mut e, p, a, 1, 0x101, 0);
        assert_eq!(calls(&e, ACTOR_SET_MOVE_MODE), vec![vec![a, 0x101]]);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x824), 1);
        assert_eq!(e.mem.f32(p.addr() + 0xfc), 9.0);
    }

    #[test]
    fn travel_paths_to_the_location() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        e.register(PROCESS_SLOTS + 0x824, |_, _| ret(1));
        middle_high_process_process_travel(&mut e, p, a, 1, 1, 0x101, 0);
        assert_eq!(calls(&e, ACTOR_SET_MOVE_MODE), vec![vec![a, 0x101]]);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x824), 1);
        assert_eq!(
            calls(&e, PACKAGE_GET_RADIUS_TO_LOCATION),
            vec![vec![package, a, 0]]
        );
    }

    #[test]
    fn travel_arrival_ends_the_move_and_advances() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        e.register(OBJECT_SLOTS + 0x13c, |_, _| ret(1));
        middle_high_process_process_travel(&mut e, p, a, 0, 1, 0x101, 0);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x824), 0);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x294), vec![vec![p.addr(), a]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x1c0), vec![vec![p.addr(), 1]]);
    }

    #[test]
    fn update_dispatches_the_procedure() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        e.register(ACTOR_SLOTS + 0x1d0, |_, _| ret(1));
        e.register(ACTOR_SLOTS + 0x100, |_, _| ret(1));
        e.register(REFERENCE_CELL, |_, _| ret(0x700));
        e.register(CELL_STATE_IS_3, |_, _| ret(1));
        e.register(PACKAGE_WORD_18, |_, _| ret(2));
        e.register(PROCESS_SLOTS + 0x280, |_, _| ret(1));
        let table = e.mem.alloc(8);
        e.mem.set_u32(table + 4, 5);
        e.mem.set_u32(PROCEDURE_TABLES + 8, table);
        let process_object = thing(&mut e);
        e.register_double(WORD_68, move |_, _| ret(process_object));
        middle_high_process_update(&mut e, p, a);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x7bc), vec![vec![p.addr(), a, 0]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x24), vec![vec![p.addr(), a, 0]]);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0x28), vec![vec![process_object]]);
        assert_eq!(calls(&e, EMPTY_FUNCTION).len(), 2);
        let _ = package;
    }

    #[test]
    fn update_without_cell_records_the_time() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        e.register(ACTOR_SLOTS + 0x1d0, |_, _| ret(1));
        middle_high_process_update(&mut e, p, a);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x28), vec![vec![p.addr()]]);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x24), 0);
    }

    #[test]
    fn get_target_for_package_by_type() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(PACKAGE_TYPE, |_, _| ret(28));
        e.register(DIALOGUE_GET_TARGET_OF_CONVERSATION, |_, _| ret(0x91));
        assert_eq!(
            middle_high_process_get_target_for_package(&mut e, p, 0x40, 0x50),
            0x91
        );
        e.register(PACKAGE_TYPE, |_, _| ret(1));
        e.register(PACKAGE_IS_TARGET_AN_ACTOR, |_, _| ret(1));
        e.register(PACKAGE_ACTOR_TARGET, |_, _| ret(0x92));
        assert_eq!(
            middle_high_process_get_target_for_package(&mut e, p, 0x40, 0x50),
            0x92
        );
        assert_eq!(calls(&e, PACKAGE_ACTOR_TARGET), vec![vec![0x50, 0x40, 0]]);
    }

    #[test]
    fn set_target_for_package_marks_the_target() {
        let mut e = engine();
        let p = process(&mut e);
        let package = with_package(&mut e);
        e.register(PACKAGE_WORD_30, |_, _| ret(1));
        let target = thing(&mut e);
        e.register_double(PROCESS_SLOTS + 0x810, move |_, _| ret(target));
        e.register(PROCESS_SLOTS + 0x12c, |e, a| {
            e.mem.set_u32(a[0] + 0x40, a[1]);
            Ret::default()
        });
        let point = e.mem.alloc(12);
        e.register_double(OBJECT_SLOTS + 0x1f4, move |_, _| ret(point));
        assert!(middle_high_process_set_target_for_package(&mut e, p, 0x40));
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x810),
            vec![vec![p.addr(), 0x40, package]]
        );
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![target, 1]]);
    }

    #[test]
    fn set_target_data_returns_a_reference_target() {
        let mut e = engine();
        let p = process(&mut e);
        let reference = thing(&mut e);
        e.register_double(PACKAGE_TARGET_GET_REFERENCE, move |_, _| ret(reference));
        assert_eq!(
            middle_high_process_set_target_data(&mut e, p, 0x40, 0x50, 0x60),
            reference
        );
        assert_eq!(
            middle_high_process_set_target_data(&mut e, p, 0x40, 0, 0x60),
            0
        );
    }

    #[test]
    fn set_target_data_takes_the_first_object_to_acquire() {
        let mut e = engine();
        let p = process(&mut e);
        e.register(TARGET_GET_TYPE, |_, _| ret(1));
        let reference = thing(&mut e);
        let acquire = e.mem.alloc(0x1c);
        e.mem.set_u32(acquire, reference);
        e.mem.set_u32(p.addr() + 0x5c, acquire);
        e.register(LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        assert_eq!(
            middle_high_process_set_target_data(&mut e, p, 0x40, 0x50, 0x60),
            reference
        );
        assert_eq!(e.mem.u32(p.addr() + 0x64), acquire);
    }

    /// A two-entry list at +0x5C with words `first` and `second` at
    /// `offset` of its entries.
    fn two_entries(
        e: &mut Engine,
        p: Ptr<MiddleHighProcess>,
        offset: u32,
        first: u32,
        second: u32,
    ) -> (u32, u32) {
        let a = e.mem.alloc(0x1c);
        let b = e.mem.alloc(0x1c);
        e.mem.set_u32(a + offset, first);
        e.mem.set_u32(b + offset, second);
        let next = list_of(e, b);
        e.mem.set_u32(p.addr() + 0x5c, a);
        e.mem.set_u32(p.addr() + 0x60, next);
        e.register(LIST_COUNT, |_, _| ret(2));
        e.register(NODE_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            if value != 0 {
                e.mem.set_u32(a[0], value);
            }
            Ret::default()
        });
        (a, b)
    }

    #[test]
    fn sort_by_18_orders_ascending() {
        let mut e = engine();
        let p = process(&mut e);
        let (a, b) = two_entries(&mut e, p, 0x18, 5, 2);
        fn_0091b580(&mut e, p);
        assert_eq!(e.mem.u32(p.addr() + 0x5c), b);
        let next = e.mem.u32(p.addr() + 0x60);
        assert_eq!(e.mem.u32(next), a);
    }

    #[test]
    fn sort_by_10_keeps_an_ordered_list() {
        let mut e = engine();
        let p = process(&mut e);
        let (a, _) = two_entries(&mut e, p, 0x10, 1, 2);
        fn_0091b640(&mut e, p);
        assert_eq!(e.mem.u32(p.addr() + 0x5c), a);
        assert_eq!(count(&e, NODE_SET_ITEM), 0);
    }

    #[test]
    fn evaluate_order_acquire_list_keeps_unowned_objects() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let reference = thing(&mut e);
        let object = e.mem.alloc(0x1c);
        e.mem.set_u32(object, reference);
        e.mem.set_u32(p.addr() + 0x74, object);
        e.register(FTOL, |_, _| ret(42));
        middle_high_process_evaluate_order_acquire_list(&mut e, p, a);
        assert_eq!(e.mem.u32(object + 0x10), 42);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![reference, 1]]);
        assert_eq!(calls(&e, LIST_APPEND)[0][0], p.addr() + 0x5c);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![p.addr() + 0x74]]);
    }

    #[test]
    fn update_ov2_runs_one_hour_procedures_until_told_to_stop() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(REFERENCE_GET_REF_PERSISTS, |_, _| ret(1));
        e.register(PACKAGE_WORD_18, |_, _| ret(2));
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, 41);
        e.mem.set_u32(PROCEDURE_TABLES + 8, table);
        e.register_double(PROCESS_SLOTS + 0x288, {
            let mut runs = 0u32;
            move |e, _| {
                runs += 1;
                if runs == 2 {
                    e.mem.set_u32(table, 26);
                }
                Ret::default()
            }
        });
        let process_object = thing(&mut e);
        e.register_double(WORD_68, move |_, _| ret(process_object));
        middle_high_process_update_ov2(&mut e, p, a, 1.0);
        assert_eq!(count(&e, PROCESS_SLOTS + 0x288), 2);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x808), vec![vec![p.addr(), a]]);
        assert_eq!(calls(&e, OBJECT_SLOTS + 0x28), vec![vec![process_object]]);
    }

    #[test]
    fn update_ov2_passes_the_hours_to_combat() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(REFERENCE_GET_REF_PERSISTS, |_, _| ret(1));
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, 9);
        e.mem.set_u32(PROCEDURE_TABLES, table);
        let process_object = thing(&mut e);
        e.register_double(WORD_68, move |_, _| ret(process_object));
        middle_high_process_update_ov2(&mut e, p, a, 2.0);
        assert_eq!(
            calls(&e, PROCESS_SLOTS + 0x81c),
            vec![vec![p.addr(), a, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn cast_ability_goes_to_the_caster() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_cast_ability(&mut e, p, 0x10, 0x20, 1);
        assert_eq!(
            calls(&e, MAGIC_CASTER_CAST_ABILITY),
            vec![vec![0x10, 0x20, 1]]
        );
    }

    #[test]
    fn transfer_disease_goes_to_the_caster() {
        let mut e = engine();
        let p = process(&mut e);
        middle_high_process_transfer_disease(&mut e, p, 0x10, 0x20, 0x30, 1);
        assert_eq!(
            calls(&e, MAGIC_CASTER_TRANSFER_DISEASE),
            vec![vec![0x10, 0x20, 0x30, 1]]
        );
    }

    #[test]
    fn escort_one_hour_hands_the_item_over() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        let change = e.mem.alloc(8);
        e.register_double(ACTOR_ITEM_CHANGE, move |_, _| ret(change));
        e.register(PACKAGE_LOCATION, |_, _| ret(0x40));
        e.register(OBJECT_SLOTS + 0x13c, |_, _| ret(1));
        e.register(LOCATION_GET_REFERENCE, |_, _| ret(0x50));
        e.register(REFERENCE_HAS_CONTAINER, |_, _| ret(1));
        e.register(PACKAGE_GET_INITIAL_TARGET_COUNT, |_, _| ret(3));
        e.register(ITEM_CHANGE_FORM, |_, _| ret(0x60));
        assert!(!fn_0091c810(&mut e, p, a));
        assert_eq!(
            calls(&e, ACTOR_ITEM_TO_CONTAINER),
            vec![vec![a, 0x60, 0, 0x50, 3, 0]]
        );
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![change, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x118), vec![vec![p.addr(), 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
        let _ = package;
    }

    #[test]
    fn follow_one_hour_reports_a_failed_path() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        with_package(&mut e);
        let target = thing(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        e.register(POINT_LENGTH, |_, _| ret_f(500.0));
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 100.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| ret(limit));
        assert!(middle_high_process_process_follow_one_hour(
            &mut e, p, a, 0, 1
        ));
        assert_eq!(count(&e, ACTOR_SET_MOVE_MODE), 0);
        e.register(PROCESS_SLOTS + 0x824, |_, _| ret(1));
        assert!(!middle_high_process_process_follow_one_hour(
            &mut e, p, a, 0, 1
        ));
    }

    #[test]
    fn eat_one_hour_eats_and_advances() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let food = e.mem.alloc(8);
        e.register_double(ACTOR_GET_BEST_FOOD_ITEM, move |_, _| ret(food));
        e.register(ITEM_CHANGE_FORM, |_, _| ret(0x44));
        assert!(!middle_high_process_process_eat_one_hour(&mut e, p, a));
        assert_eq!(calls(&e, ACTOR_EAT), vec![vec![a, 0x44, 0, 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }

    #[test]
    fn activate_one_hour_activates_a_close_object() {
        let mut e = engine();
        let p = process(&mut e);
        let a = actor(&mut e);
        let package = with_package(&mut e);
        let target = thing(&mut e);
        e.mem.set_u32(p.addr() + 0x40, target);
        e.register(OBJECT_SLOTS + 0x144, |_, _| ret(1));
        assert!(middle_high_process_process_activate_one_hour(&mut e, p, a));
        assert_eq!(calls(&e, OBJECT_SLOTS + 0x144), vec![vec![package, a, 0]]);
        assert_eq!(
            calls(&e, REFERENCE_ACTIVATE),
            vec![vec![target, a, 0, 0, 1]]
        );
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x4ec), vec![vec![p.addr(), 1]]);
        assert_eq!(calls(&e, PROCESS_SLOTS + 0x288), vec![vec![p.addr(), a, 1]]);
    }
}
