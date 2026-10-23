//! `fallout shared/facegen/bsfacegenninode.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00662ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00662b00` and frees 0xf0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_ni_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_2b00, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xf0u32]);
    }
    this
}

// Translated from 006629e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5bf8 without reading the receiver. The map names this body `BSFaceGenNiNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006629e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5bf8
}

// Translated from 006629f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::GetAnimationData` (Xbox PDB, engine map vtable name). `thiscall`, no
/// stack words: the smart pointer `get` (`00559450`) of the pointer at `+0xac`, whose result
/// it answers.
pub fn bs_face_gen_ni_node_get_animation_data(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0055_9450, &args![this.addr().wrapping_add(0xac)])
        .ptr()
}

layout! {
    /// `BSFaceGenNiNode` (Xbox PDB), 0xf0 bytes on PC (the deleting destructor frees
    /// 0xf0); its own fields sit 0x24 (up to `BaseRotation`) and 0x3c (after it) below the
    /// Xbox PDB's, as the PC code uses them.
    pub struct BSFaceGenNiNode: 0xf0 {
        /// `spAnimationData` (Xbox PDB): `NiPointer<BSFaceGenAnimationData>`.
        0xAC spAnimationData: u32,
        /// `bForceBaseMorph` (Xbox PDB).
        0xD4 bForceBaseMorph: u8,
        /// `bAnimationUpdate` (Xbox PDB).
        0xD6 bAnimationUpdate: u8,
        /// `bApplyRotationToParent` (Xbox PDB).
        0xD8 bApplyRotationToParent: u8,
        /// `fLastTime` (Xbox PDB).
        0xDC fLastTime: f32,
        /// `bIAmInDialogue` (Xbox PDB).
        0xE2 bIAmInDialogue: u8,
        /// `bDisableMorphUpdates` (Xbox PDB).
        0xE3 bDisableMorphUpdates: u8,
        /// `bForceMorphUpdates` (Xbox PDB).
        0xE4 bForceMorphUpdates: u8,
        /// `pActor` (Xbox PDB): `Actor*`.
        0xE8 pActor: u32,
        /// `bCycleFaceGenCoord` (Xbox PDB).
        0xEC bCycleFaceGenCoord: u8,
    }
}

/// `BSFaceGenNiNode`'s vtable.
const VTABLE: u32 = 0x0106_60dc;
/// The `GetAnimationData` slot (+0x100) the node's functions call.
const SLOT_GET_ANIMATION_DATA: u32 = 0x100;
/// `0066b0d0` (thiscall on an `NiPointer`, one word: assigns it; unnamed).
const POINTER_SET: u32 = 0x0066_b0d0;
/// `00559450` (thiscall on an `NiPointer`, returns the object; unnamed).
const POINTER_GET: u32 = 0x0055_9450;
/// `0045cec0` (thiscall on an `NiPointer`: destroys it; unnamed).
const POINTER_DESTROY: u32 = 0x0045_cec0;
/// `NiNode`'s destructor body (`00a5e930`).
const NI_NODE_DESTROY: u32 = 0x00a5_e930;
/// `00653340` (cdecl: geometry, buffer; unnamed, `bsfacegenmanager.cpp`).
const MORPH_PREPARE_00653340: u32 = 0x0065_3340;
/// `00662030` (thiscall on the morph data: kind, index, geometry, word, `float`;
/// unnamed, `bsfacegenmorphdata.cpp`): applies one morph.
const MORPH_APPLY_00662030: u32 = 0x0066_2030;
/// `005dd4a0` (thiscall on the animation data, one word: an index; `float` in `ST0`).
const ANIMATION_VALUE_005DD4A0: u32 = 0x005d_d4a0;
/// `00653d70` (cdecl: two `float`s, two out pointers; unnamed): splits a pair of
/// values.
const SPLIT_PAIR_00653D70: u32 = 0x0065_3d70;
/// `00450410` (thiscall on a setting, returns its `float` in `ST0`; unnamed).
const SETTING_REAL_00450410: u32 = 0x0045_0410;
/// The settings `fn_006633d0` and `fn_006633e0` read.
const SETTING_011D5B28: u32 = 0x011d_5b28;
const SETTING_011D5B34: u32 = 0x011d_5b34;
/// The global `fn_006633f0` tests (`011d59ec`) and its fallback (`011f426c`).
const CAMERA_HOLDER_011D59EC: u32 = 0x011d_59ec;
const DEFAULT_POSITION_011F426C: u32 = 0x011f_426c;
/// `0045bb80` (thiscall, returns a pointer; unnamed, `tes.cpp`).
const GETTER_0045BB80: u32 = 0x0045_bb80;
/// `00534550` (thiscall: constructs the buffer at +4 of a morph context; unnamed).
const BUFFER_INIT_00534550: u32 = 0x0053_4550;
/// `00408d60` (thiscall on a setting, returns the address of its value; unnamed).
const SETTING_ADDRESS_00408D60: u32 = 0x0040_8d60;
/// The settings `fn_00663a30` and `UpdateMorphing` read through `00408d60`.
const SETTING_011D5A5C: u32 = 0x011d_5a5c;
const SETTING_011DF6D0: u32 = 0x011d_f6d0;
/// The timer object `00663a50` and `0084d030` are asked (`011f6394`).
const TIMER: u32 = 0x011f_6394;
/// `0043b480` (child count), `0043b4a0` (child at an index): thiscall on a node.
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `005495f0` (thiscall on the geometry, returns its data; unnamed).
const GEOMETRY_DATA_005495F0: u32 = 0x0054_95f0;
/// `00653230`, `00653430` (cdecl: geometry; unnamed, `bsfacegenmanager.cpp`).
const MORPH_DATA_00653230: u32 = 0x0065_3230;
const MORPH_VALUE_00653430: u32 = 0x0065_3430;
/// `00a67360` (thiscall on the geometry data, one word, returns `AL`), `00a67410`
/// (one word: a buffer), `00a673d0` (no words): `nigeometrydata.cpp`, unnamed.
const GEOMETRY_DATA_00A67360: u32 = 0x00a6_7360;
const GEOMETRY_DATA_00A67410: u32 = 0x00a6_7410;
const GEOMETRY_DATA_00A673D0: u32 = 0x00a6_73d0;
/// `00456650` (thiscall, returns a `u16`; unnamed).
const HALF_00456650: u32 = 0x0045_6650;
/// `00439ef0` (thiscall on a position: result, other position; unnamed).
const POSITION_DIFFERENCE_00439EF0: u32 = 0x0043_9ef0;
/// `00457990` (thiscall on a vector, `float` in `ST0`: its length by its use).
const LENGTH_00457990: u32 = 0x0045_7990;
/// `00525420` (no arguments, returns `AL`; unnamed).
const TEST_00525420: u32 = 0x0052_5420;
/// `Interface::IsInMenuMode` (`00702360`).
const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `009611e0` (thiscall on an `NiAVObject`, returns its parent; unnamed).
const PARENT_009611E0: u32 = 0x0096_11e0;
/// `10.0` (`float`).
const TEN: u32 = 0x0101_7b78;
/// `0.25` (`double`).
const QUARTER: u32 = 0x0102_90b0;
/// `0.0` and `1.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `00456610` (thiscall on an `NiAVObject`, returns `AL`: whether it is hidden).
const IS_APP_CULLED_00456610: u32 = 0x0045_6610;
/// `009d7c60` (thiscall on the node, returns `AL`; unnamed).
const TEST_009D7C60: u32 = 0x009d_7c60;
/// `00663450` (thiscall on the node, one word; this unit, not translated).
const NODE_00663450: u32 = 0x0066_3450;
/// `00403e20` (thiscall on a setting, returns the address of its value; unnamed).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting `UpdateMorphing` scales the frame time by (`011c5724`).
const SETTING_011C5724: u32 = 0x011c_5724;
/// `0084d030` (thiscall on the timer, `float` in `ST0`: the frame time by its use).
const FRAME_TIME_0084D030: u32 = 0x0084_d030;
/// `008d8520` (thiscall on the actor, returns its process; the map's name
/// `MiddleHighProcess::GetSavedAcquireObject` comes from folded code).
const ACTOR_PROCESS_008D8520: u32 = 0x008d_8520;
/// `004f0140` (thiscall on the node, returns `AL`; unnamed, `bsbipnode.cpp`).
const NODE_TEST_004F0140: u32 = 0x004f_0140;
/// `0064be40` (thiscall on the animation data: a `float`, the parent; returns `AL`).
const ANIMATION_UPDATE_0064BE40: u32 = 0x0064_be40;
/// `0096e7d0` (thiscall on the process lists at `011e0e80`: actor, word; returns `AL`).
const PROCESS_LISTS_TEST_0096E7D0: u32 = 0x0096_e7d0;
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The player pointer (as other units name `011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `MobileObject::GetCurrentProcessType` (`00931850`).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `00652510` (cdecl: the actor; unnamed, `bsfacegenmanager.cpp`).
const MANAGER_00652510: u32 = 0x0065_2510;
/// `NiNode::UpdateDownwardPass` (`00a5dd70`) and `NiNode::OnVisible` (`00a5dbe0`).
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
/// `NiAVObject::GetWorldBound` (`0043d450`).
const NI_AV_OBJECT_GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `BSShaderAccumulator::StartGroupingAlphas` (`00b63500`) and `StopGroupingAlphas`
/// (`00b63090`).
const START_GROUPING_ALPHAS: u32 = 0x00b6_3500;
const STOP_GROUPING_ALPHAS: u32 = 0x00b6_3090;
/// `0043fad0` (thiscall on the geometry, returns its skin instance; unnamed).
const SKIN_INSTANCE_0043FAD0: u32 = 0x0043_fad0;
/// `005585e0` (thiscall on the skin instance, returns its skin data; unnamed).
const SKIN_DATA_005585E0: u32 = 0x0055_85e0;
/// `008041a0` (thiscall on the skin data, returns the bone count by its use).
const BONE_COUNT_008041A0: u32 = 0x0080_41a0;
/// `00825c00` (thiscall on the skin instance, returns its bone array by its use).
const BONES_00825C00: u32 = 0x0082_5c00;
/// `00413f40` (thiscall on a bone, returns its name; unnamed).
const NAME_00413F40: u32 = 0x0041_3f40;
/// `0043b1b0` (thiscall, returns the name as a C string; unnamed).
const NAME_STRING_0043B1B0: u32 = 0x0043_b1b0;
/// `004adda0` (thiscall on the skin instance: index, bone; unnamed).
const SET_BONE_004ADDA0: u32 = 0x004a_dda0;
/// `00437730` (thiscall on the skin instance, one word: the root; unnamed).
const SET_ROOT_00437730: u32 = 0x0043_7730;
/// `005b5e40` (cdecl, a format and its arguments: a debug message).
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `"MODELS: Could not find bone \"%s\" for the head node."`.
const NO_BONE_MESSAGE: u32 = 0x0106_631c;

// Translated from 00439920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::SetAnimationData` (Xbox PDB): assigns `spAnimationData`.
pub fn bs_face_gen_ni_node_set_animation_data(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    data: u32,
) {
    e.call(POINTER_SET, &args![this.addr() + 0xac, data]);
}

// Translated from 00662a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::SetAnimationUpdate` (Xbox PDB): sets `bAnimationUpdate`.
pub fn bs_face_gen_ni_node_set_animation_update(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    value: u8,
) {
    e.set(this, BSFaceGenNiNode::bAnimationUpdate, value);
}

// Translated from 0097ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::SetApplyRotToParent` (Xbox PDB): sets `bApplyRotationToParent`.
pub fn bs_face_gen_ni_node_set_apply_rot_to_parent(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    value: u8,
) {
    e.set(this, BSFaceGenNiNode::bApplyRotationToParent, value);
}

// Translated from 00662b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00662b00 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenNiNode`'s destructor body (the engine map gives it no name; Ghidra shows a
/// library-match name): the vtable, `pActor` cleared, `spAnimationData` destroyed, then
/// the `NiNode` destructor.
pub fn fn_00662b00(e: &mut Engine, this: Ptr<BSFaceGenNiNode>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, BSFaceGenNiNode::pActor, 0);
    e.call(POINTER_DESTROY, &args![this.addr() + 0xac]);
    e.call(NI_NODE_DESTROY, &args![this]);
}

/// Applies one morph of `kind` and `index` with `value` when `0 < value <= 1` (the
/// exe's three compares, NaN never passing): prepares the context's geometry once
/// (`00653340`, the flag at +0x1c), then `00662030` on the context's morph data.
fn apply_morph(e: &mut Engine, context: u32, kind: u32, index: u32, value: u32) {
    let v = x87_load32(value);
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,0x41; JNZ: on only above zero.
    if !(v > zero) {
        return;
    }
    // FCOMP; TEST AH,1; JNZ: off below zero or unordered.
    if v < zero || v.is_nan() {
        return;
    }
    // FCOMP; TEST AH,0x41; JP: on only below or equal to one.
    if !(v <= one) {
        return;
    }
    if e.mem.u8(context + 0x1c) == 0 {
        let geometry = e.mem.u32(context);
        e.call(MORPH_PREPARE_00653340, &args![geometry, context + 4]);
        e.mem.set_u8(context + 0x1c, 1);
    }
    let morph_data = e.mem.u32(context + 0x10);
    let geometry = e.mem.u32(context);
    let word = e.mem.u32(context + 0x14);
    e.call(
        MORPH_APPLY_00662030,
        &args![
            morph_data,
            kind,
            index,
            geometry,
            word,
            x87_quiet_bits(value)
        ],
    );
}

/// The shape of `fn_00662bc0`, `fn_00662e90` and `fn_00662f70`: for each index below
/// `count`, the animation data's value from `slot` (stored as a `float`) is applied as a
/// morph of `kind`.
fn apply_animation_morphs(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    context: u32,
    count: i32,
    slot: u32,
    kind: u32,
) {
    let mut i = 0i32;
    while i < count {
        let data = e.vcall(this.addr(), SLOT_GET_ANIMATION_DATA, &[]).u32();
        let value = e.vcall(data, slot, &args![i]).f64();
        apply_morph(e, context, kind, i as u32, x87_store32(value));
        i += 1;
    }
}

// Translated from 00662bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): applies the 15 morphs of kind 1 from the animation data's slot
/// +0x94 to the morph context.
pub fn fn_00662bc0(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, context: u32) {
    apply_animation_morphs(e, this, context, 0xf, 0x94, 1);
}

// Translated from 00662e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): applies the 16 morphs of kind 0 from the animation data's slot
/// +0xa4.
pub fn fn_00662e90(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, context: u32) {
    apply_animation_morphs(e, this, context, 0x10, 0xa4, 0);
}

// Translated from 00662f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): applies the single morph of kind 3 from the animation data's slot
/// +0xac.
pub fn fn_00662f70(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, context: u32) {
    apply_animation_morphs(e, this, context, 1, 0xac, 3);
}

/// One value of the animation data (`005dd4a0` with `index`), as a `float`.
fn animation_value(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, index: u32) -> u32 {
    let data = e.vcall(this.addr(), SLOT_GET_ANIMATION_DATA, &[]).u32();
    let value = e.call(ANIMATION_VALUE_005DD4A0, &args![data, index]).f64();
    x87_store32(value)
}

// Translated from 00662ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): applies the morphs of kind 2 for the indices 0 to 13 with the
/// animation data's values (`005dd4a0`). Indices 8 and 11, and 9 and 10, are pairs:
/// their two values go through `00653d70` (the lower index's first), which gives the
/// first and second value of the pair. Indices 14 to 16 are skipped.
pub fn fn_00662ca0(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, context: u32) {
    e.with_stack(0x10, |e, pair| {
        let first = pair.addr();
        let second = pair.addr() + 4;
        let mut i = 0i32;
        while i < 0x11 {
            let value = match i {
                8 | 0xb => {
                    let upper = animation_value(e, this, 0xb);
                    let lower = animation_value(e, this, 8);
                    e.call(SPLIT_PAIR_00653D70, &args![lower, upper, first, second]);
                    let chosen = if i == 8 { first } else { second };
                    Some(x87_quiet_bits(e.mem.u32(chosen)))
                }
                9 | 0xa => {
                    let upper = animation_value(e, this, 0xa);
                    let lower = animation_value(e, this, 9);
                    e.call(SPLIT_PAIR_00653D70, &args![lower, upper, first, second]);
                    let chosen = if i == 9 { first } else { second };
                    Some(x87_quiet_bits(e.mem.u32(chosen)))
                }
                0xe..=0x10 => None,
                _ => Some(animation_value(e, this, i as u32)),
            };
            if let Some(value) = value {
                apply_morph(e, context, 2, i as u32, value);
            }
            i += 1;
        }
    });
}

// Translated from 006633d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `float` of the setting at `011d5b28` (`00450410`), left in `ST0`.
pub fn fn_006633d0(e: &mut Engine) -> f64 {
    e.call(SETTING_REAL_00450410, &args![SETTING_011D5B28])
        .f64()
}

// Translated from 006633e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `float` of the setting at `011d5b34` (`00450410`), left in `ST0`.
pub fn fn_006633e0(e: &mut Engine) -> f64 {
    e.call(SETTING_REAL_00450410, &args![SETTING_011D5B34])
        .f64()
}

// Translated from 006633f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `0045bb80` of the object at `011d59ec`, or the address `011f426c` without one.
pub fn fn_006633f0(e: &mut Engine) -> u32 {
    let holder = e.global::<u32>(CAMERA_HOLDER_011D59EC);
    if holder == 0 {
        DEFAULT_POSITION_011F426C
    } else {
        e.call(GETTER_0045BB80, &args![holder]).u32()
    }
}

// Translated from 00663430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: constructs a morph context (`00534550` on its buffer at +4); returns `this`.
pub fn fn_00663430(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BUFFER_INIT_00534550, &args![this.addr() + 4]);
    this
}

// Translated from 006639f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter: the byte at `this + 0x189` (the animation data's, as `UpdateMorphing`
/// calls it).
pub fn fn_006639f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x189)
}

// Translated from 00663a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): sets the byte at `this + 0x189`.
pub fn fn_00663a10(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x189, value);
}

/// The byte at the address `00408d60` gives for `setting`.
fn setting_byte(e: &mut Engine, setting: u32) -> u8 {
    let address = e.call(SETTING_ADDRESS_00408D60, &args![setting]).u32();
    e.mem.u8(address)
}

// Translated from 00663a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the byte value of the setting at `011d5a5c` (`00408d60`).
pub fn fn_00663a30(e: &mut Engine) -> u8 {
    setting_byte(e, SETTING_011D5A5C)
}

// Translated from 00663a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter: the byte at `this + 0x1d` (called on the timer at `011f6394`).
pub fn fn_00663a50(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1d)
}

// Translated from 00663b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the object of the `NiPointer` at `this + 0xc4` (`00559450`), left in `EAX`.
pub fn fn_00663b40(e: &mut Engine, this: Ptr) -> u32 {
    e.call(POINTER_GET, &args![this.addr() + 0xc4]).u32()
}

// Translated from 00663b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter: the word at `this + 0x19c`.
pub fn fn_00663b60(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x19c)
}

// Translated from 00663050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`; called by `UpdateMorphing`): applies the face morphs to each child
/// geometry with facegen morph data (`00653230`, `00653430`) whose data accepts it
/// (`00a67360(1)`). Close enough to the camera (the distance to `006633f0`'s position
/// within `006633e0 + 10`, both settings quartered when `00525420`), in dialogue or in
/// menu mode, the kind-0 morphs apply (unless morph updates are disabled); within
/// `006633d0 + 10` also kinds 1, 2 and 3. Otherwise kind 2 applies when the animation
/// data's slot +0xd4 is true and its value 0 is positive with a parent's slot +0x10; or
/// the animation data is reset (slot +0xb4) once the head is beyond `006633d0 - 10`.
/// `force` or `bForceBaseMorph` (cleared) and `bForceMorphUpdates` prepare the base
/// morph first. Clears `bForceMorphUpdates` at the end.
pub fn fn_00663050(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, force: u8) {
    let base = this.addr();
    // The frame below `EBP` (0x70 bytes): `local(n)` is `EBP - n`.
    e.with_stack(0x70, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x70 - offset;
        let context = local(0x24);
        fn_00663430(e, Ptr::new(context));
        let count = e.call(CHILD_COUNT, &args![this]).u32();
        let mut i = 0u32;
        while i < count {
            e.mem.set_u8(context + 0x1c, 0);
            let child = e.call(CHILD_AT, &args![this, i]).u32();
            i += 1;
            if child == 0 {
                continue;
            }
            let geometry = e.vcall(child, 0x1c, &[]).u32();
            e.mem.set_u32(context, geometry);
            if geometry == 0 || e.call(GEOMETRY_DATA_005495F0, &args![geometry]).u32() == 0 {
                continue;
            }
            let morph_data = e.call(MORPH_DATA_00653230, &args![geometry]).u32();
            e.mem.set_u32(context + 0x10, morph_data);
            if morph_data == 0 {
                continue;
            }
            let word = e.call(MORPH_VALUE_00653430, &args![geometry]).u32();
            e.mem.set_u32(context + 0x14, word);
            if word == 0 {
                continue;
            }
            let data = e.call(GEOMETRY_DATA_005495F0, &args![geometry]).u32();
            if !e.call(GEOMETRY_DATA_00A67360, &args![data, 1u32]).bool() {
                continue;
            }
            let data = e.call(GEOMETRY_DATA_005495F0, &args![geometry]).u32();
            e.call(GEOMETRY_DATA_00A67410, &args![data, context + 4]);
            let word = u32::from(e.call(HALF_00456650, &args![geometry]).u16());
            e.mem.set_u32(context + 0x18, word);
            if force != 0 || e.get(this, BSFaceGenNiNode::bForceBaseMorph) != 0 {
                e.call(MORPH_PREPARE_00653340, &args![geometry, context + 4]);
                e.mem.set_u8(context + 0x1c, 1);
                e.set(this, BSFaceGenNiNode::bForceBaseMorph, 0);
            }
            let mut all_applied = false;
            let mut base_applied = false;
            let position = fn_006633f0(e);
            let camera = local(0x3c);
            for k in 0..3 {
                let word = e.mem.u32(position + 4 * k);
                e.mem.set_u32(camera + 4 * k, word);
            }
            let ten = x87_quiet_bits(e.global::<u32>(TEN));
            let own = e.call(GETTER_0045BB80, &args![this]).u32();
            let difference = e
                .call(
                    POSITION_DIFFERENCE_00439EF0,
                    &args![camera, local(0x5c), own],
                )
                .u32();
            let distance = e.call(LENGTH_00457990, &args![difference]).f64();
            let distance = x87_store32(distance);
            let mut far = x87_store32(fn_006633d0(e));
            let mut near = x87_store32(fn_006633e0(e));
            if e.call(TEST_00525420, &[]).bool() {
                let quarter: f64 = e.global(QUARTER);
                far = x87_store32(x87_mul(x87_load32(far), quarter));
                near = x87_store32(x87_mul(x87_load32(near), quarter));
            }
            let within = |e: &mut Engine, limit: u32| -> bool {
                let limit = x87_add(x87_load32(limit), x87_m32(ten));
                let d = x87_load32(distance);
                // FCOMPP; TEST AH,0x41; JNP: below or equal goes on at once.
                d <= limit
                    || e.get(this, BSFaceGenNiNode::bIAmInDialogue) != 0
                    || e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
            };
            let mut near_enough = false;
            let mut close_enough = false;
            if within(e, near) {
                near_enough = true;
                if within(e, far) {
                    close_enough = true;
                }
            }
            if e.get(this, BSFaceGenNiNode::bForceMorphUpdates) != 0
                && e.mem.u8(context + 0x1c) == 0
            {
                e.call(MORPH_PREPARE_00653340, &args![geometry, context + 4]);
                e.mem.set_u8(context + 0x1c, 1);
            }
            if near_enough && e.get(this, BSFaceGenNiNode::bDisableMorphUpdates) == 0 {
                fn_00662e90(e, this, context);
                base_applied = true;
                if close_enough {
                    fn_00662bc0(e, this, context);
                    fn_00662ca0(e, this, context);
                    fn_00662f70(e, this, context);
                    all_applied = true;
                }
            }
            let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
            if e.vcall(animation, 0xd4, &[]).bool() {
                if !all_applied {
                    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
                    let value = e
                        .call(ANIMATION_VALUE_005DD4A0, &args![animation, 0u32])
                        .f64();
                    let zero: f64 = e.global(ZERO);
                    // FCOMP; TEST AH,0x41; JNZ: only above zero.
                    if value > zero {
                        let parent = e.call(PARENT_009611E0, &args![this]).u32();
                        if e.vcall(parent, 0x10, &[]).u32() != 0 {
                            fn_00662ca0(e, this, context);
                        }
                    }
                }
            } else if !base_applied {
                let limit = x87_sub(fn_006633d0(e), x87_m32(ten));
                // FCOMPP; TEST AH,0x41; JNZ: only when the distance is above.
                if x87_load32(distance) > limit {
                    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
                    e.vcall(
                        animation,
                        0xb4,
                        &args![x87_store32(0.0), 1u32, 1u32, 1u32, 1u32, 0u32],
                    );
                }
            }
            let data = e.call(GEOMETRY_DATA_005495F0, &args![geometry]).u32();
            e.call(GEOMETRY_DATA_00A673D0, &args![data]);
        }
    });
    e.set(this, BSFaceGenNiNode::bForceMorphUpdates, 0);
}

// Translated from 00663510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::UpdateMorphing` (Xbox PDB, `RET 4`; `update` points at the frame
/// time `float` and a byte): for a shown node with animation data that is updating (the
/// byte, or the data's slot +0xcc), records the time in `fLastTime`, updates the
/// animation data (slot +0xc8, or with `bAnimationUpdate` slots +0xd0 and +0x128 with the
/// scaled frame time, or 0 when the timer's byte says so; for a speaking actor in view
/// also `0064be40`), and when that changed something or updates are forced runs
/// `fn_00663050` for a head in view of the player, or else defers it (`bForceBaseMorph`,
/// `bForceMorphUpdates`). Marks the animation data updated (`00663a10`) and cycles the
/// facegen coordinate (`00652510`) when asked.
pub fn bs_face_gen_ni_node_update_morphing(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    update: u32,
) {
    let base = this.addr();
    if e.call(IS_APP_CULLED_00456610, &args![this]).bool() {
        return;
    }
    // The exe clears this byte first; every path below sets it.
    let mut changed: u8;
    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
    e.vcall(animation, 0xd4, &[]);
    if fn_00663a30(e) != 0 && !e.call(TEST_009D7C60, &args![this]).bool() {
        let zero: f64 = e.global(ZERO);
        // FCOMP; TEST AH,0x41; JNZ: only above zero.
        if x87_load32(e.mem.u32(update)) > zero {
            e.call(NODE_00663450, &args![this, 0xffff_ffffu32]);
        }
    }
    if e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32() == 0 {
        return;
    }
    if e.mem.u8(update + 4) == 0 {
        let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
        if !e.vcall(animation, 0xcc, &[]).bool() {
            return;
        }
    }
    let time = x87_quiet_bits(e.mem.u32(update));
    e.mem.set_u32(base + 0xdc, time);
    if e.get(this, BSFaceGenNiNode::bAnimationUpdate) != 0 {
        let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
        e.vcall(animation, 0xd0, &args![0u32]);
        if fn_00663a50(e, Ptr::new(TIMER)) != 0 {
            let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
            changed = e.vcall(animation, 0x128, &args![x87_store32(0.0)]).u8();
        } else {
            let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
            let address = e.call(SETTING_VALUE, &args![SETTING_011C5724]).u32();
            let scale = x87_load32(e.mem.u32(address));
            let frame = e.call(FRAME_TIME_0084D030, &args![TIMER]).f64();
            let scaled = x87_store32(x87_mul(frame, scale));
            changed = e
                .vcall(animation, 0x128, &args![x87_quiet_bits(scaled)])
                .u8();
        }
    } else {
        let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
        changed = e.vcall(animation, 0xc8, &[]).u8();
    }
    let actor = e.get(this, BSFaceGenNiNode::pActor);
    if e.get(this, BSFaceGenNiNode::bAnimationUpdate) != 0
        && actor != 0
        && e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32() != 0
    {
        let actor = e.get(this, BSFaceGenNiNode::pActor);
        let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
        if e.vcall(process, 0x678, &[]).u32() != 0 {
            let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
            if fn_006639f0(e, Ptr::new(animation)) != 0
                && e.get(this, BSFaceGenNiNode::bApplyRotationToParent) != 0
                && setting_byte(e, SETTING_011DF6D0) == 0
                && e.call(NODE_TEST_004F0140, &args![this]).bool()
            {
                let speaking = if fn_00663a50(e, Ptr::new(TIMER)) != 0 {
                    let parent = e.call(PARENT_009611E0, &args![this]).u32();
                    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
                    e.call(
                        ANIMATION_UPDATE_0064BE40,
                        &args![animation, x87_store32(0.0), parent],
                    )
                    .u8()
                } else {
                    let parent = e.call(PARENT_009611E0, &args![this]).u32();
                    let frame = e.call(FRAME_TIME_0084D030, &args![TIMER]).f64();
                    let frame = x87_store32(frame);
                    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
                    e.call(ANIMATION_UPDATE_0064BE40, &args![animation, frame, parent])
                        .u8()
                };
                changed = u8::from(changed != 0 || speaking != 0);
            }
        }
    }
    if changed != 0 || e.get(this, BSFaceGenNiNode::bForceMorphUpdates) != 0 {
        let mut in_view = false;
        if e.call(NODE_TEST_004F0140, &args![this]).bool() {
            let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
            if e.vcall(animation, 0xd4, &[]).bool()
                || e.call(
                    PROCESS_LISTS_TEST_0096E7D0,
                    &args![PROCESS_LISTS, e.get(this, BSFaceGenNiNode::pActor), 10u32],
                )
                .bool()
                || e.get(this, BSFaceGenNiNode::pActor) == e.global::<u32>(PLAYER_POINTER)
                || e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
            {
                in_view = true;
            }
            let actor = e.get(this, BSFaceGenNiNode::pActor);
            if actor != 0 && e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32() == 0 {
                let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                if e.vcall(process, 0x78, &[]).bool() {
                    let actor = e.get(this, BSFaceGenNiNode::pActor);
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    if e.vcall(process, 0xa8, &[]).u32() != 0 {
                        let actor = e.get(this, BSFaceGenNiNode::pActor);
                        let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                        if e.vcall(process, 0xa8, &[]).u32() == e.global::<u32>(PLAYER_POINTER) {
                            in_view = true;
                        }
                    }
                }
            }
        }
        if in_view || e.get(this, BSFaceGenNiNode::bForceMorphUpdates) != 0 {
            fn_00663050(e, this, changed);
        } else {
            e.set(this, BSFaceGenNiNode::bForceBaseMorph, 1);
            e.set(this, BSFaceGenNiNode::bForceMorphUpdates, 1);
        }
    }
    let animation = e.vcall(base, SLOT_GET_ANIMATION_DATA, &[]).u32();
    fn_00663a10(e, Ptr::new(animation), 1);
    let actor = e.get(this, BSFaceGenNiNode::pActor);
    if e.get(this, BSFaceGenNiNode::bCycleFaceGenCoord) != 0 && actor != 0 {
        e.call(MANAGER_00652510, &args![actor]);
    }
}

// Translated from 00663a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::UpdateDownwardPass` (Xbox PDB, `RET 8`): `UpdateMorphing` with
/// `update` for a shown node, then the `NiNode` pass with both words.
pub fn bs_face_gen_ni_node_update_downward_pass(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    update: u32,
    flags: u32,
) {
    if !e.call(IS_APP_CULLED_00456610, &args![this]).bool() {
        bs_face_gen_ni_node_update_morphing(e, this, update);
    }
    e.call(NI_NODE_UPDATE_DOWNWARD_PASS, &args![this, update, flags]);
}

// Translated from 00663ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::OnVisible` (Xbox PDB, `RET 4`): when the culler's accumulator
/// (`fn_00663b40`) is not grouping already (`fn_00663b60` zero), groups the node's alphas
/// around the `NiNode` visit (`StartGroupingAlphas` with the world bound and 0,
/// `StopGroupingAlphas` after).
pub fn bs_face_gen_ni_node_on_visible(e: &mut Engine, this: Ptr<BSFaceGenNiNode>, culler: u32) {
    let accumulator = fn_00663b40(e, Ptr::new(culler));
    let current = fn_00663b40(e, Ptr::new(culler));
    let grouping = fn_00663b60(e, Ptr::new(current));
    if grouping == 0 {
        let bound = e.call(NI_AV_OBJECT_GET_WORLD_BOUND, &args![this]).u32();
        e.call(START_GROUPING_ALPHAS, &args![accumulator, bound, 0u32]);
    }
    e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
    if grouping == 0 {
        e.call(STOP_GROUPING_ALPHAS, &args![accumulator]);
    }
}

// Translated from 006642e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode::FixSkinInstances` (Xbox PDB, `RET 8`): with a `root`, points every
/// bone of each child geometry's skin instance at the root's node of the same name
/// (slot +0x9c; a missing one is reported when `report` is set) and sets the skin
/// instance's root to this node.
pub fn bs_face_gen_ni_node_fix_skin_instances(
    e: &mut Engine,
    this: Ptr<BSFaceGenNiNode>,
    root: u32,
    report: u8,
) {
    if root == 0 {
        return;
    }
    let count = e.call(CHILD_COUNT, &args![this]).u32();
    let mut i = 0u32;
    while i < count {
        let child = e.call(CHILD_AT, &args![this, i]).u32();
        i += 1;
        if child == 0 {
            continue;
        }
        let geometry = e.vcall(child, 0x1c, &[]).u32();
        if geometry == 0 {
            continue;
        }
        let skin = e.call(SKIN_INSTANCE_0043FAD0, &args![geometry]).u32();
        if skin == 0 || e.call(SKIN_DATA_005585E0, &args![skin]).u32() == 0 {
            continue;
        }
        let data = e.call(SKIN_DATA_005585E0, &args![skin]).u32();
        let bone_count = e.call(BONE_COUNT_008041A0, &args![data]).u32();
        let bones = e.call(BONES_00825C00, &args![skin]).u32();
        if bones == 0 || bone_count == 0 {
            continue;
        }
        let mut b = 0u32;
        while b < bone_count {
            let bone = e.mem.u32(bones.wrapping_add(b.wrapping_mul(4)));
            let name = e.call(NAME_00413F40, &args![bone]).u32();
            let found = e.vcall(root, 0x9c, &args![name]).u32();
            if found != 0 {
                e.call(SET_BONE_004ADDA0, &args![skin, b, found]);
            } else if report != 0 {
                let bone = e.mem.u32(bones.wrapping_add(b.wrapping_mul(4)));
                let name = e.call(NAME_00413F40, &args![bone]).u32();
                let text = e.call(NAME_STRING_0043B1B0, &args![name]).u32();
                e.call(DEBUG_MESSAGE, &args![NO_BONE_MESSAGE, text]);
            }
            b += 1;
        }
        e.call(SET_ROOT_00437730, &args![skin, this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0066_29f0,
            bs_face_gen_ni_node_get_animation_data(Ptr) -> Ptr
        ),
        entry!(0x0066_29e0, fn_006629e0(Ptr) -> u32),
        entry!(0x0066_2ac0, bs_face_gen_ni_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0043_9920,
            bs_face_gen_ni_node_set_animation_data(Ptr<BSFaceGenNiNode>, u32)
        ),
        entry!(
            0x0066_2a60,
            bs_face_gen_ni_node_set_animation_update(Ptr<BSFaceGenNiNode>, u8)
        ),
        entry!(
            0x0097_ce70,
            bs_face_gen_ni_node_set_apply_rot_to_parent(Ptr<BSFaceGenNiNode>, u8)
        ),
        entry!(0x0066_2b00, fn_00662b00(Ptr<BSFaceGenNiNode>)),
        entry!(0x0066_2bc0, fn_00662bc0(Ptr<BSFaceGenNiNode>, u32)),
        entry!(0x0066_2ca0, fn_00662ca0(Ptr<BSFaceGenNiNode>, u32)),
        entry!(0x0066_2e90, fn_00662e90(Ptr<BSFaceGenNiNode>, u32)),
        entry!(0x0066_2f70, fn_00662f70(Ptr<BSFaceGenNiNode>, u32)),
        entry!(0x0066_3050, fn_00663050(Ptr<BSFaceGenNiNode>, u8)),
        entry!(0x0066_33d0, fn_006633d0() -> f64),
        entry!(0x0066_33e0, fn_006633e0() -> f64),
        entry!(0x0066_33f0, fn_006633f0() -> u32),
        entry!(0x0066_3430, fn_00663430(Ptr) -> Ptr),
        entry!(
            0x0066_3510,
            bs_face_gen_ni_node_update_morphing(Ptr<BSFaceGenNiNode>, u32)
        ),
        entry!(0x0066_39f0, fn_006639f0(Ptr) -> u8),
        entry!(0x0066_3a10, fn_00663a10(Ptr, u8)),
        entry!(0x0066_3a30, fn_00663a30() -> u8),
        entry!(0x0066_3a50, fn_00663a50(Ptr) -> u8),
        entry!(
            0x0066_3a70,
            bs_face_gen_ni_node_update_downward_pass(Ptr<BSFaceGenNiNode>, u32, u32)
        ),
        entry!(
            0x0066_3ae0,
            bs_face_gen_ni_node_on_visible(Ptr<BSFaceGenNiNode>, u32)
        ),
        entry!(0x0066_3b40, fn_00663b40(Ptr) -> u32),
        entry!(0x0066_3b60, fn_00663b60(Ptr) -> u32),
        entry!(
            0x0066_42e0,
            bs_face_gen_ni_node_fix_skin_instances(Ptr<BSFaceGenNiNode>, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_face_gen_ni_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_2ac0, 0x0066_2b00, 0xf0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006629e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_29e0, &args![0x1000u32]).u32(), 0x011d_5bf8);
        assert_eq!(e.call(0x0066_29e0, &args![0u32]).u32(), 0x011d_5bf8);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn get_animation_data_answers_the_pointer_at_0xac() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0xf0);
        e.mem.set_u32(this + 0xac, 0x1234_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_29f0, &args![this]).u32(), 0x1234_0000);
        let log = e.call_log.take().unwrap();
        let get: Vec<_> = log.iter().filter(|c| c.0 == 0x0055_9450).collect();
        assert_eq!(get[0].1, vec![this + 0xac]);
    }
}

#[cfg(test)]
mod b066_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_float(e: &mut Engine, addr: u32, st0: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet_all(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            answer(e, *a, 0);
        }
    }

    const SLOTS: u32 = 0x0330_0000;

    /// An object whose vtable sends every slot `s` to `SLOTS + base + s`.
    fn object(e: &mut Engine, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for s in (0..0x200).step_by(4) {
            e.mem.set_u32(vtable + s, SLOTS + base + s);
        }
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, vtable);
        this
    }

    const NODE: u32 = 0;
    const ANIMATION: u32 = 0x1000;

    /// A node whose animation data (slot +0x100) is an object whose value slots
    /// +0x94, +0xa4 and +0xac and `005dd4a0` answer the `float` at `+0x100 + 4 * index`
    /// of the animation data.
    fn node_with_animation(e: &mut Engine) -> (Ptr<BSFaceGenNiNode>, u32) {
        let node = object(e, NODE);
        let animation = object(e, ANIMATION);
        answer(e, SLOTS + NODE + 0x100, animation);
        for slot in [0x94u32, 0xa4, 0xac] {
            e.register(SLOTS + ANIMATION + slot, |e, a| Ret {
                st0: f64::from(e.mem.f32(a[0] + 0x100 + 4 * a[1])),
                ..Ret::default()
            });
        }
        e.register(ANIMATION_VALUE_005DD4A0, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x100 + 4 * a[1])),
            ..Ret::default()
        });
        e.map(ZERO, 8);
        e.mem.set_f64(ZERO, 0.0);
        e.map(ONE, 8);
        e.mem.set_f64(ONE, 1.0);
        quiet_all(e, &[MORPH_PREPARE_00653340, MORPH_APPLY_00662030]);
        (Ptr::new(node), animation)
    }

    /// A morph context: geometry 0x6000, morph data 0x6100, word 0x6200.
    fn context(e: &mut Engine) -> u32 {
        let context = e.mem.alloc(0x20);
        e.mem.set_u32(context, 0x6000);
        e.mem.set_u32(context + 0x10, 0x6100);
        e.mem.set_u32(context + 0x14, 0x6200);
        context
    }

    /// The (kind, index, value) of each applied morph.
    fn applied(e: &Engine) -> Vec<(u32, u32, f32)> {
        calls(e, MORPH_APPLY_00662030)
            .iter()
            .map(|c| {
                assert_eq!(c[0], 0x6100);
                assert_eq!(c[3..5], [0x6000, 0x6200]);
                (c[1], c[2], f32::from_bits(c[5]))
            })
            .collect()
    }

    #[test]
    fn setters_store_their_fields() {
        let mut e = Engine::new();
        answer(&mut e, POINTER_SET, 0);
        let node = Ptr::<BSFaceGenNiNode>::new(e.mem.alloc(0xf0));
        e.call_log = Some(vec![]);
        e.call(0x0043_9920, &args![node, 0x7000u32]);
        assert_eq!(
            calls(&e, POINTER_SET),
            vec![vec![node.addr() + 0xac, 0x7000]]
        );
        e.call(0x0066_2a60, &args![node, 1u32]);
        assert_eq!(e.get(node, BSFaceGenNiNode::bAnimationUpdate), 1);
        e.call(0x0097_ce70, &args![node, 1u32]);
        assert_eq!(e.get(node, BSFaceGenNiNode::bApplyRotationToParent), 1);
    }

    #[test]
    fn destructor_clears_the_actor_and_destroys_the_bases() {
        let mut e = Engine::new();
        quiet_all(&mut e, &[POINTER_DESTROY, NI_NODE_DESTROY]);
        let node = Ptr::<BSFaceGenNiNode>::new(e.mem.alloc(0xf0));
        e.set(node, BSFaceGenNiNode::pActor, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0066_2b00, &args![node]);
        assert_eq!(e.mem.u32(node.addr()), VTABLE);
        assert_eq!(e.get(node, BSFaceGenNiNode::pActor), 0);
        assert_eq!(calls(&e, POINTER_DESTROY), vec![vec![node.addr() + 0xac]]);
        assert_eq!(calls(&e, NI_NODE_DESTROY), vec![vec![node.addr()]]);
    }

    #[test]
    fn morph_sets_apply_values_in_zero_to_one() {
        let mut e = Engine::new();
        let (node, animation) = node_with_animation(&mut e);
        for (i, v) in [0.5f32, 0.0, 1.0, 1.5, -0.25, f32::NAN].iter().enumerate() {
            e.mem.set_f32(animation + 0x100 + 4 * i as u32, *v);
        }
        let ctx = context(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0066_2e90, &args![node, ctx]);
        assert_eq!(applied(&e), vec![(0, 0, 0.5), (0, 2, 1.0)]);
        // Prepared once, then marked.
        assert_eq!(
            calls(&e, MORPH_PREPARE_00653340),
            vec![vec![0x6000, ctx + 4]]
        );
        assert_eq!(e.mem.u8(ctx + 0x1c), 1);
        assert_eq!(calls(&e, SLOTS + ANIMATION + 0xa4).len(), 16);
        e.call_log = Some(vec![]);
        e.call(0x0066_2bc0, &args![node, ctx]);
        assert_eq!(applied(&e), vec![(1, 0, 0.5), (1, 2, 1.0)]);
        assert!(calls(&e, MORPH_PREPARE_00653340).is_empty());
        assert_eq!(calls(&e, SLOTS + ANIMATION + 0x94).len(), 15);
        e.call_log = Some(vec![]);
        e.call(0x0066_2f70, &args![node, ctx]);
        assert_eq!(applied(&e), vec![(3, 0, 0.5)]);
    }

    #[test]
    fn kind_two_morphs_split_the_pairs_and_skip_the_last_three() {
        let mut e = Engine::new();
        let (node, animation) = node_with_animation(&mut e);
        for i in 0..17u32 {
            e.mem.set_f32(animation + 0x100 + 4 * i, 0.25);
        }
        // The pair split writes (lower / 2, upper / 2) as the two values.
        e.register(SPLIT_PAIR_00653D70, |e, a| {
            let lower = f32::from_bits(a[0]);
            let upper = f32::from_bits(a[1]);
            e.mem.set_f32(a[2], lower / 2.0);
            e.mem.set_f32(a[3], upper);
            Ret::default()
        });
        e.mem.set_f32(animation + 0x100 + 4 * 8, 0.5);
        e.mem.set_f32(animation + 0x100 + 4 * 11, 0.75);
        let ctx = context(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0066_2ca0, &args![node, ctx]);
        let got = applied(&e);
        let indices: Vec<u32> = got.iter().map(|m| m.1).collect();
        assert_eq!(indices, (0..14).collect::<Vec<u32>>());
        assert!(got.iter().all(|m| m.0 == 2));
        assert_eq!(got[8].2, 0.25);
        assert_eq!(got[11].2, 0.75);
        assert_eq!(got[9].2, 0.125);
        assert_eq!(got[10].2, 0.25);
        // Each pair index computes its pair: (8, 11) and (9, 10) twice each.
        let splits = calls(&e, SPLIT_PAIR_00653D70);
        assert_eq!(splits.len(), 4);
        assert_eq!(splits[0][..2], [0.5f32.to_bits(), 0.75f32.to_bits()]);
    }

    #[test]
    fn small_getters_and_setters() {
        let mut e = Engine::new();
        answer_float(&mut e, SETTING_REAL_00450410, 7.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_33d0, &[]).f64(), 7.5);
        assert_eq!(e.call(0x0066_33e0, &[]).f64(), 7.5);
        assert_eq!(
            calls(&e, SETTING_REAL_00450410),
            vec![vec![SETTING_011D5B28], vec![SETTING_011D5B34]]
        );
        e.map(CAMERA_HOLDER_011D59EC, 4);
        assert_eq!(e.call(0x0066_33f0, &[]).u32(), DEFAULT_POSITION_011F426C);
        e.mem.set_u32(CAMERA_HOLDER_011D59EC, 0x7100);
        answer(&mut e, GETTER_0045BB80, 0x7200);
        assert_eq!(e.call(0x0066_33f0, &[]).u32(), 0x7200);
        assert_eq!(calls(&e, GETTER_0045BB80), vec![vec![0x7100]]);
        answer(&mut e, BUFFER_INIT_00534550, 0);
        assert_eq!(e.call(0x0066_3430, &args![0x7300u32]).u32(), 0x7300);
        assert_eq!(calls(&e, BUFFER_INIT_00534550), vec![vec![0x7304]]);
        let object = e.mem.alloc(0x1a0);
        e.call(0x0066_3a10, &args![object, 0x105u32]);
        assert_eq!(e.mem.u8(object + 0x189), 5);
        assert_eq!(e.call(0x0066_39f0, &args![object]).u32(), 5);
        e.mem.set_u8(object + 0x1d, 9);
        assert_eq!(e.call(0x0066_3a50, &args![object]).u32(), 9);
        e.mem.set_u32(object + 0x19c, 0x1234);
        assert_eq!(e.call(0x0066_3b60, &args![object]).u32(), 0x1234);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.mem.set_u32(object + 0xc4, 0x4321);
        assert_eq!(e.call(0x0066_3b40, &args![object]).u32(), 0x4321);
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, 3);
        answer(&mut e, SETTING_ADDRESS_00408D60, byte);
        assert_eq!(e.call(0x0066_3a30, &[]).u32(), 3);
        assert_eq!(
            calls(&e, SETTING_ADDRESS_00408D60),
            vec![vec![SETTING_011D5A5C]]
        );
    }

    /// A node with one child geometry ready for morphing, at `distance` from the
    /// camera, with `near` and `far` limits.
    fn morph_engine(distance: f64, near: f64, far: f64) -> (Engine, Ptr<BSFaceGenNiNode>, u32) {
        let mut e = Engine::new();
        let (node, animation) = node_with_animation(&mut e);
        for i in 0..17u32 {
            e.mem.set_f32(animation + 0x100 + 4 * i, 0.5);
        }
        answer(&mut e, CHILD_COUNT, 1);
        let child = object(&mut e, 0x2000);
        answer(&mut e, CHILD_AT, child);
        answer(&mut e, SLOTS + 0x2000 + 0x1c, 0x6000);
        answer(&mut e, GEOMETRY_DATA_005495F0, 0x6300);
        answer(&mut e, MORPH_DATA_00653230, 0x6100);
        answer(&mut e, MORPH_VALUE_00653430, 0x6200);
        answer(&mut e, GEOMETRY_DATA_00A67360, 1);
        answer(&mut e, HALF_00456650, 0x1_0007);
        e.map(CAMERA_HOLDER_011D59EC, 4);
        e.map(DEFAULT_POSITION_011F426C, 12);
        e.map(TEN, 4);
        e.mem.set_f32(TEN, 10.0);
        e.map(QUARTER, 8);
        e.mem.set_f64(QUARTER, 0.25);
        answer(&mut e, GETTER_0045BB80, 0x6400);
        answer(&mut e, POSITION_DIFFERENCE_00439EF0, 0x6500);
        answer_float(&mut e, LENGTH_00457990, distance);
        e.register_double(SETTING_REAL_00450410, move |_, a| Ret {
            st0: if a[0] == SETTING_011D5B28 { far } else { near },
            ..Ret::default()
        });
        quiet_all(
            &mut e,
            &[
                BUFFER_INIT_00534550,
                GEOMETRY_DATA_00A67410,
                GEOMETRY_DATA_00A673D0,
                TEST_00525420,
                INTERFACE_IS_IN_MENU_MODE,
                SLOTS + ANIMATION + 0xd4,
                SLOTS + ANIMATION + 0xb4,
                SPLIT_PAIR_00653D70,
            ],
        );
        e.call_log = Some(vec![]);
        (e, node, animation)
    }

    fn kinds(e: &Engine) -> Vec<u32> {
        let mut kinds: Vec<u32> = calls(e, MORPH_APPLY_00662030)
            .iter()
            .map(|c| c[1])
            .collect();
        kinds.dedup();
        kinds
    }

    #[test]
    fn morphing_applies_by_distance() {
        // Near and far: every kind.
        let (mut e, node, _) = morph_engine(5.0, 0.0, 0.0);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert_eq!(kinds(&e), vec![0, 1, 2, 3]);
        let data = calls(&e, GEOMETRY_DATA_00A67410);
        assert_eq!(data[0][0], 0x6300);
        assert_eq!(calls(&e, GEOMETRY_DATA_00A673D0), vec![vec![0x6300]]);
        // Within the near limit only: kind 0.
        let (mut e, node, _) = morph_engine(15.0, 10.0, 0.0);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert_eq!(kinds(&e), vec![0]);
        // Disabled morph updates: nothing; beyond the far limit minus 10 the
        // animation data is reset.
        let (mut e, node, animation) = morph_engine(15.0, 10.0, 0.0);
        e.set(node, BSFaceGenNiNode::bDisableMorphUpdates, 1);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert!(kinds(&e).is_empty());
        assert_eq!(
            calls(&e, SLOTS + ANIMATION + 0xb4),
            vec![vec![animation, 0, 1, 1, 1, 1, 0]]
        );
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceMorphUpdates), 0);
    }

    #[test]
    fn morphing_far_away_uses_dialogue_and_the_quartered_limits() {
        // Too far: nothing unless in dialogue.
        let (mut e, node, _) = morph_engine(100.0, 0.0, 0.0);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert!(kinds(&e).is_empty());
        let (mut e, node, _) = morph_engine(100.0, 0.0, 0.0);
        e.set(node, BSFaceGenNiNode::bIAmInDialogue, 1);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert_eq!(kinds(&e), vec![0, 1, 2, 3]);
        // Quartered: 100 * 0.25 + 10 = 35 is below 40.
        let (mut e, node, _) = morph_engine(40.0, 100.0, 100.0);
        answer(&mut e, TEST_00525420, 1);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert!(kinds(&e).is_empty());
        // Forced: the base morph is prepared and the flag cleared.
        let (mut e, node, _) = morph_engine(100.0, 0.0, 0.0);
        e.set(node, BSFaceGenNiNode::bForceBaseMorph, 1);
        e.call(0x0066_3050, &args![node, 0u32]);
        assert_eq!(calls(&e, MORPH_PREPARE_00653340).len(), 1);
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceBaseMorph), 0);
    }

    fn update_engine() -> (Engine, Ptr<BSFaceGenNiNode>, u32, u32) {
        let (mut e, node, animation) = morph_engine(5.0, 0.0, 0.0);
        quiet_all(
            &mut e,
            &[
                IS_APP_CULLED_00456610,
                TEST_009D7C60,
                NODE_00663450,
                SLOTS + ANIMATION + 0xd0,
                SLOTS + ANIMATION + 0x128,
                SLOTS + ANIMATION + 0xc8,
                SLOTS + ANIMATION + 0xcc,
                NODE_TEST_004F0140,
                MANAGER_00652510,
                PROCESS_LISTS_TEST_0096E7D0,
            ],
        );
        let byte = e.mem.alloc(4);
        answer(&mut e, SETTING_ADDRESS_00408D60, byte);
        e.map(PLAYER_POINTER, 4);
        let update = e.mem.alloc(8);
        e.mem.set_f32(update, 0.5);
        e.call_log = Some(vec![]);
        (e, node, animation, update)
    }

    #[test]
    fn update_morphing_stops_for_hidden_or_idle_nodes() {
        let (mut e, node, _, update) = update_engine();
        answer(&mut e, IS_APP_CULLED_00456610, 1);
        e.call(0x0066_3510, &args![node, update]);
        assert!(calls(&e, SLOTS + NODE + 0x100).is_empty());
        // Not updating: only the checks run.
        let (mut e, node, _, update) = update_engine();
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(calls(&e, SLOTS + ANIMATION + 0xcc).len(), 1);
        assert!(calls(&e, SLOTS + ANIMATION + 0xc8).is_empty());
        assert_eq!(e.get(node, BSFaceGenNiNode::fLastTime), 0.0);
    }

    #[test]
    fn update_morphing_defers_out_of_view_and_morphs_in_view() {
        // Changed, but not in view: deferred.
        let (mut e, node, animation, update) = update_engine();
        e.mem.set_u8(update + 4, 1);
        answer(&mut e, SLOTS + ANIMATION + 0xc8, 1);
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(e.get(node, BSFaceGenNiNode::fLastTime), 0.5);
        assert!(calls(&e, MORPH_APPLY_00662030).is_empty());
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceBaseMorph), 1);
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceMorphUpdates), 1);
        assert_eq!(e.mem.u8(animation + 0x189), 1);
        // Forced: the next update morphs (and clears the force).
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(kinds(&e), vec![0, 1, 2, 3]);
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceMorphUpdates), 0);
        // In view through the menu mode: morphs at once.
        let (mut e, node, _, update) = update_engine();
        e.mem.set_u8(update + 4, 1);
        answer(&mut e, SLOTS + ANIMATION + 0xc8, 1);
        answer(&mut e, NODE_TEST_004F0140, 1);
        answer(&mut e, INTERFACE_IS_IN_MENU_MODE, 1);
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(kinds(&e), vec![0, 1, 2, 3]);
        assert_eq!(e.get(node, BSFaceGenNiNode::bForceMorphUpdates), 0);
    }

    #[test]
    fn update_morphing_runs_the_animation_update_with_the_scaled_frame_time() {
        let (mut e, node, animation, update) = update_engine();
        e.mem.set_u8(update + 4, 1);
        e.set(node, BSFaceGenNiNode::bAnimationUpdate, 1);
        e.set(node, BSFaceGenNiNode::bCycleFaceGenCoord, 1);
        e.set(node, BSFaceGenNiNode::pActor, 0x7400);
        answer(&mut e, ACTOR_PROCESS_008D8520, 0);
        e.map(TIMER, 0x20);
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 2.0);
        answer(&mut e, SETTING_VALUE, scale);
        answer_float(&mut e, FRAME_TIME_0084D030, 0.25);
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(
            calls(&e, SLOTS + ANIMATION + 0xd0),
            vec![vec![animation, 0]]
        );
        assert_eq!(
            calls(&e, SLOTS + ANIMATION + 0x128),
            vec![vec![animation, 0.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, MANAGER_00652510), vec![vec![0x7400]]);
        // The timer's byte at +0x1d: 0 instead of the frame time.
        e.mem.set_u8(TIMER + 0x1d, 1);
        e.call_log = Some(vec![]);
        e.call(0x0066_3510, &args![node, update]);
        assert_eq!(
            calls(&e, SLOTS + ANIMATION + 0x128),
            vec![vec![animation, 0]]
        );
    }

    #[test]
    fn downward_pass_and_on_visible() {
        let (mut e, node, _, update) = update_engine();
        quiet_all(
            &mut e,
            &[
                NI_NODE_UPDATE_DOWNWARD_PASS,
                NI_NODE_ON_VISIBLE,
                NI_AV_OBJECT_GET_WORLD_BOUND,
                START_GROUPING_ALPHAS,
                STOP_GROUPING_ALPHAS,
            ],
        );
        e.call(0x0066_3a70, &args![node, update, 3u32]);
        assert_eq!(calls(&e, SLOTS + ANIMATION + 0xcc).len(), 1);
        assert_eq!(
            calls(&e, NI_NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![node.addr(), update, 3]]
        );
        let culler = e.mem.alloc(0xd0);
        let accumulator = e.mem.alloc(0x1a0);
        e.mem.set_u32(culler + 0xc4, accumulator);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        answer(&mut e, NI_AV_OBJECT_GET_WORLD_BOUND, 0x7500);
        e.call_log = Some(vec![]);
        e.call(0x0066_3ae0, &args![node, culler]);
        assert_eq!(
            calls(&e, START_GROUPING_ALPHAS),
            vec![vec![accumulator, 0x7500, 0]]
        );
        assert_eq!(
            calls(&e, NI_NODE_ON_VISIBLE),
            vec![vec![node.addr(), culler]]
        );
        assert_eq!(calls(&e, STOP_GROUPING_ALPHAS), vec![vec![accumulator]]);
        // Already grouping: only the visit.
        e.mem.set_u32(accumulator + 0x19c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0066_3ae0, &args![node, culler]);
        assert!(calls(&e, START_GROUPING_ALPHAS).is_empty());
        assert!(calls(&e, STOP_GROUPING_ALPHAS).is_empty());
    }

    #[test]
    fn fix_skin_instances_points_bones_at_the_root() {
        let mut e = Engine::new();
        let node = Ptr::<BSFaceGenNiNode>::new(e.mem.alloc(0xf0));
        answer(&mut e, CHILD_COUNT, 1);
        let child = object(&mut e, 0x2000);
        answer(&mut e, CHILD_AT, child);
        answer(&mut e, SLOTS + 0x2000 + 0x1c, 0x6000);
        answer(&mut e, SKIN_INSTANCE_0043FAD0, 0x6600);
        answer(&mut e, SKIN_DATA_005585E0, 0x6700);
        answer(&mut e, BONE_COUNT_008041A0, 2);
        let bones = e.mem.alloc(8);
        e.mem.set_u32(bones, 0x6800);
        e.mem.set_u32(bones + 4, 0x6900);
        answer(&mut e, BONES_00825C00, bones);
        e.register(NAME_00413F40, |_, a| (a[0] + 1).into_ret());
        e.register(NAME_STRING_0043B1B0, |_, a| (a[0] + 2).into_ret());
        let root = object(&mut e, 0x3000);
        e.register(SLOTS + 0x3000 + 0x9c, |_, a| {
            (if a[1] == 0x6801 { 0x6a00 } else { 0 }).into_ret()
        });
        quiet_all(
            &mut e,
            &[SET_BONE_004ADDA0, SET_ROOT_00437730, DEBUG_MESSAGE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0066_42e0, &args![node, 0u32, 1u32]);
        assert!(calls(&e, CHILD_COUNT).is_empty());
        e.call(0x0066_42e0, &args![node, root, 1u32]);
        assert_eq!(calls(&e, SET_BONE_004ADDA0), vec![vec![0x6600, 0, 0x6a00]]);
        assert_eq!(
            calls(&e, DEBUG_MESSAGE),
            vec![vec![NO_BONE_MESSAGE, 0x6903]]
        );
        assert_eq!(
            calls(&e, SET_ROOT_00437730),
            vec![vec![0x6600, node.addr()]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0066_42e0, &args![node, root, 0u32]);
        assert!(calls(&e, DEBUG_MESSAGE).is_empty());
    }
}
