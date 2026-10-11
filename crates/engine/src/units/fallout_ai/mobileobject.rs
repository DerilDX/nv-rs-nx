//! `fallout/ai/mobileobject.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Continuation queue: the first 40 address-ordered entries end at 00931850.
//! Continuation of those 40 entries:
//! - 0092ec40: normal destructor path registered; compiler SEH cleanup open.
//! - 0092ed60, 009308f0: normal paths registered; runtime validation pending.
//! - 00930c70, 00931710: normal paths registered; compiler SEH cleanup open.
//! - 0092f260: normal movement path registered; compiler SEH cleanup open.
//!
//! Next address-ordered queue entry after this bounded batch: 00931880.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MobileObject` (Xbox PDB), 0x88 bytes on PC after the TESForm size
    /// difference. Constructor stores and process accesses confirm offsets.
    pub struct MobileObject: 0x88 {
        0x68 pCurrentProcess: Ptr,
        0x6c pTalkingActivator: Ptr,
        0x70 pDialogueItemTarget: Ptr,
        0x78 fSayToFailVoiceTimer: f32,
        0x7c bSpeakingDone: bool,
        0x7d bTalkingToPlayer: bool,
        0x7e bIgnoreChangeAnimationCall: bool,
        0x7f bSoundFileDone: bool,
        0x80 bVoiceFileDone: bool,
        0x81 bTalkingActivator: bool,
        0x82 bInTempChangeList: bool,
        0x83 bDoNotRunSayToCallBack: bool,
        0x84 cSoundCallBackSet: u8,
        0x85 bSoundCallBackSuccess: bool,
        0x86 bUseEmotion: bool,
        0x87 bRunsInLow: bool,
    }
}

/// The PC FLD/FSTP pair quiets a signaling single-precision NaN while
/// preserving its sign and payload.
fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(
        if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        },
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0092eb50,
            mobile_object_ctor(Ptr<MobileObject>) -> Ptr<MobileObject>
        ),
        entry!(
            0x0092ec10,
            fn_0092ec10(Ptr<MobileObject>, u32) -> Ptr<MobileObject>
        ),
        entry!(0x0092ec40, mobile_object_dtor(Ptr<MobileObject>)),
        entry!(
            0x0092ed60,
            mobile_object_clean_up_other_mobile_objects(Ptr<MobileObject>, u8)
        ),
        entry!(0x0092f140, fn_0092f140(Ptr) -> Ptr),
        entry!(
            0x0092f160,
            mobile_object_is_talking(Ptr<MobileObject>) -> bool
        ),
        entry!(0x0092f190, fn_0092f190(Ptr<MobileObject>)),
        entry!(0x0092f1c0, fn_0092f1c0(Ptr<MobileObject>, u32)),
        entry!(
            0x0092f260,
            mobile_object_move(Ptr<MobileObject>, f32, Ptr, u32) -> Ptr
        ),
        entry!(0x009304a0, fn_009304a0() -> Ptr),
        entry!(0x009304b0, fn_009304b0(Ptr) -> bool),
        entry!(0x009304d0, fn_009304d0(Ptr) -> Ptr),
        entry!(0x00930540, fn_00930540(Ptr, u8)),
        entry!(0x00930570, fn_00930570(Ptr) -> bool),
        entry!(0x009305a0, fn_009305a0(Ptr) -> bool),
        entry!(0x009305d0, fn_009305d0(Ptr) -> bool),
        entry!(0x00930600, fn_00930600(Ptr, f32)),
        entry!(0x00930620, fn_00930620(Ptr)),
        entry!(0x00930640, fn_00930640(Ptr<MobileObject>) -> Ptr),
        entry!(0x009306b0, fn_009306b0(Ptr) -> bool),
        entry!(
            0x009306d0,
            mobile_object_get_char_controller(Ptr<MobileObject>) -> Ptr
        ),
        entry!(0x00930700, fn_00930700(Ptr<MobileObject>, Ptr)),
        entry!(0x009307d0, fn_009307d0(Ptr<MobileObject>) -> bool),
        entry!(0x00930810, fn_00930810(Ptr<MobileObject>) -> u8),
        entry!(0x00930870, fn_00930870(Ptr<MobileObject>) -> u8),
        entry!(0x009308f0, fn_009308f0(Ptr<MobileObject>, u32)),
        entry!(0x00930c50, fn_00930c50(Ptr) -> bool),
        entry!(0x00930c70, mobile_object_init_havok(Ptr<MobileObject>)),
        entry!(0x009314d0, fn_009314d0(Ptr, f32)),
        entry!(0x009314f0, fn_009314f0(Ptr, u8)),
        entry!(0x00931510, fn_00931510(Ptr, u8)),
        entry!(0x00931530, fn_00931530(Ptr, u8)),
        entry!(0x00931560, fn_00931560(Ptr, f32)),
        entry!(0x00931580, fn_00931580(Ptr) -> Ptr),
        entry!(0x009315a0, fn_009315a0(Ptr, u8)),
        entry!(0x009315c0, fn_009315c0(Ptr<MobileObject>, u8)),
        entry!(0x00931620, fn_00931620(Ptr<MobileObject>, Ptr)),
        entry!(
            0x009316c0,
            mobile_object_set_package_start_location(Ptr<MobileObject>)
        ),
        entry!(
            0x00931710,
            mobile_object_replace_with_high_process(Ptr<MobileObject>) -> bool
        ),
        entry!(
            0x00931850,
            mobile_object_get_current_process_type(Ptr<MobileObject>) -> i32
        ),
    ]
}

// Translated from 0092eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::MobileObject` (Xbox PDB): constructs its reference base,
/// installs the two PC vtables and initializes its own members.
pub fn mobile_object_ctor(e: &mut Engine, this: Ptr<MobileObject>) -> Ptr<MobileObject> {
    e.call(0x0055_a2f0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_a49c);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_a490);
    e.set(this, MobileObject::pTalkingActivator, Ptr::NULL);
    e.set(this, MobileObject::pCurrentProcess, Ptr::NULL);
    e.set(this, MobileObject::pDialogueItemTarget, Ptr::NULL);
    e.set(this, MobileObject::bSpeakingDone, true);
    e.set(this, MobileObject::bTalkingToPlayer, false);
    e.set(this, MobileObject::bIgnoreChangeAnimationCall, false);
    e.set(this, MobileObject::cSoundCallBackSet, 0xff);
    e.set(this, MobileObject::bSoundCallBackSuccess, false);
    e.set(this, MobileObject::bSoundFileDone, true);
    e.set(this, MobileObject::bVoiceFileDone, true);
    e.set(this, MobileObject::bUseEmotion, true);
    e.set(this, MobileObject::bTalkingActivator, false);
    e.set(this, MobileObject::bInTempChangeList, false);
    e.set(this, MobileObject::bRunsInLow, false);
    e.set(this, MobileObject::fSayToFailVoiceTimer, 0.0);
    e.set(this, MobileObject::bDoNotRunSayToCallBack, false);
    this
}

// Translated from 0092ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper: destroys the object and frees it when
/// bit zero is set.
pub fn fn_0092ec10(e: &mut Engine, this: Ptr<MobileObject>, flags: u32) -> Ptr<MobileObject> {
    e.call(0x0092_ec40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0092ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::~MobileObject` (Xbox PDB): leaves the process lists,
/// destroys its current process, and destroys the reference base.
/// The compiler's SEH unwinding frame is not translated.
// Translation incomplete: 0092ec40 - compiler SEH exception cleanup.
pub fn mobile_object_dtor(e: &mut Engine, this: Ptr<MobileObject>) {
    e.mem.set_u32(this.addr(), 0x0108_a49c);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_a490);
    e.call(0x0040_fbf0, &args![0x011f_11a0u32, 0u32]);
    if !e.call(0x0040_77c0, &args![this]).bool() {
        e.call(0x0096_f400, &args![0x011e_0e80u32, this]);
        let extra_data = e.global::<u32>(0x011c_3f2c);
        if !e.call(0x0042_26e0, &args![extra_data]).bool() {
            e.call(0x0057_6760, &args![this]);
        }
        let process = e.get(this, MobileObject::pCurrentProcess);
        if !process.is_null() {
            let process_type = e.call(0x0045_cd60, &args![process]).u32();
            e.call(0x0096_d470, &args![0x011e_0e80u32, this, process_type]);
        }
    }
    let process = e.get(this, MobileObject::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0, &args![1u32]);
    }
    e.call(0x0040_fba0, &args![0x011f_11a0u32]);
    if e.get(this, MobileObject::bTalkingActivator) {
        e.mem.set_u32(this.addr() + 0x40, 0);
    }
    e.call(0x0055_a430, &args![this]);
}

/// Checks whether the saved acquire object targets the same reference as
/// `this`; the PC code fetches that object again before clearing the target.
fn clear_matching_saved_acquire(e: &mut Engine, object: Ptr, this: Ptr<MobileObject>) {
    let saved: Ptr = e.call(0x008d_8520, &args![object]).ptr();
    let target = e.vcall(saved.addr(), 0x3d0, &args![]).u32();
    let own_target = e.vcall(this.addr(), 0x19c, &args![]).u32();
    if target == own_target {
        let saved: Ptr = e.call(0x008d_8520, &args![object]).ptr();
        e.vcall(saved.addr(), 0x3d4, &args![0u32]);
    }
}

/// Casts a mobile object to the target type used by its callback at slot
/// 0x314. The two RTTI pointers are read from the PC instruction operands.
fn cleanup_cast(e: &mut Engine, object: Ptr, target_type: u32) -> Ptr {
    e.call(
        0x00ec_43fb,
        &args![object, 0u32, 0x0118_4920u32, target_type, 0u32],
    )
    .ptr()
}

fn notify_cleanup_target(e: &mut Engine, target: Ptr, this: Ptr<MobileObject>) {
    let reference = e.vcall(this.addr(), 0x198, &args![]).u32();
    e.vcall(target.addr(), 0x314, &args![reference]);
}

// Translated from 0092ed60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::CleanUpOtherMobileObjects` (Xbox PDB): visits two process
/// list ranges and the current dialogue object, dropping references to this
/// object from saved acquire, package, and target state.
pub fn mobile_object_clean_up_other_mobile_objects(
    e: &mut Engine,
    this: Ptr<MobileObject>,
    notify: u8,
) {
    let lists: Ptr = e.call(0x0071_7e50, &args![0x011e_0e80u32]).ptr();
    let mut index = 0;
    while index < e.call(0x005b_e5c0, &args![lists, 0u32]).u32() {
        let other: Ptr = e.call(0x0096_8670, &args![lists, index]).ptr();
        if other != this.cast() {
            if e.vcall(other.addr(), 0x100, &args![]).bool() {
                let saved: Ptr = e.call(0x008d_8520, &args![other]).ptr();
                e.vcall(saved.addr(), 0x664, &args![this]);
                if notify != 0 {
                    let reference = e.vcall(this.addr(), 0x198, &args![]).u32();
                    e.call(0x008c_4f10, &args![other, reference]);
                }
                clear_matching_saved_acquire(e, other, this);
            } else if e.vcall(other.addr(), 0x224, &args![]).bool() {
                e.call(0x009c_4c80, &args![other, this]);
            } else if e.vcall(other.addr(), 0x220, &args![]).bool() {
                e.call(0x009b_28a0, &args![other, this]);
            } else if notify != 0 {
                let target = cleanup_cast(e, other, 0x011a_0e88);
                if !target.is_null() {
                    notify_cleanup_target(e, target, this);
                }
            }
        }
        index += 1;
    }

    let mut index = e.call(0x005d_e0f0, &args![lists, 1u32]).u32();
    while index < e.call(0x005b_e5c0, &args![lists, 1u32]).u32() {
        let other: Ptr = e.call(0x0096_8670, &args![lists, index]).ptr();
        if other != this.cast() {
            let cast = cleanup_cast(e, other, 0x0118_46d4);
            if !cast.is_null() {
                if notify != 0 {
                    let reference = e.vcall(this.addr(), 0x198, &args![]).u32();
                    e.call(0x008c_4f10, &args![cast, reference]);
                }
                clear_matching_saved_acquire(e, cast, this);
            } else if notify != 0 {
                let target = cleanup_cast(e, other, 0x011a_0e88);
                if !target.is_null() {
                    notify_cleanup_target(e, target, this);
                }
            }
        }
        index += 1;
    }

    if !e.global::<Ptr>(0x011d_ea3c).is_null() {
        let dialogue = e.global::<Ptr>(0x011d_ea3c);
        let saved: Ptr = e.call(0x008d_8520, &args![dialogue]).ptr();
        if !saved.is_null() {
            let dialogue = e.global::<Ptr>(0x011d_ea3c);
            let saved: Ptr = e.call(0x008d_8520, &args![dialogue]).ptr();
            e.vcall(saved.addr(), 0x664, &args![this]);
        }
        if notify != 0 {
            let reference = e.vcall(this.addr(), 0x198, &args![]).u32();
            let dialogue = e.global::<Ptr>(0x011d_ea3c);
            e.call(0x008c_4f10, &args![dialogue, reference]);
        }
        let dialogue = e.global::<Ptr>(0x011d_ea3c);
        let saved: Ptr = e.call(0x008d_8520, &args![dialogue]).ptr();
        if !saved.is_null() {
            let dialogue = e.global::<Ptr>(0x011d_ea3c);
            let saved: Ptr = e.call(0x008d_8520, &args![dialogue]).ptr();
            let target = e.vcall(saved.addr(), 0x3d0, &args![]).u32();
            let own_target = e.vcall(this.addr(), 0x19c, &args![]).u32();
            if target == own_target {
                let dialogue = e.global::<Ptr>(0x011d_ea3c);
                let saved: Ptr = e.call(0x008d_8520, &args![dialogue]).ptr();
                e.vcall(saved.addr(), 0x3d4, &args![0u32]);
            }
        }
        let dialogue = e.global::<Ptr>(0x011d_ea3c);
        if !dialogue.is_null() && fn_0092f140(e, dialogue) == this.cast() {
            e.call(0x0052_4d70, &args![dialogue]);
        }
    }
}

// Translated from 0092f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the pointer at offset 0xD2C of its object.
pub fn fn_0092f140(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xd2c))
}

// Translated from 0092f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::IsTalking` (Xbox PDB): forwards the object as one argument
/// to the current process' virtual method at slot 0x4B8.
pub fn mobile_object_is_talking(e: &mut Engine, this: Ptr<MobileObject>) -> bool {
    let process = e.get(this, MobileObject::pCurrentProcess);
    e.vcall(process.addr(), 0x4b8, &args![this]).bool()
}

// Translated from 0092f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the current process' virtual method at slot 0x28 if it exists.
pub fn fn_0092f190(e: &mut Engine, this: Ptr<MobileObject>) {
    let process = e.get(this, MobileObject::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0x28, &args![]);
    }
}

// Translated from 0092f1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Performs the reference damage callback, process callback, then removes
/// an eligible mobile object from the process list and records a temp change.
pub fn fn_0092f1c0(e: &mut Engine, this: Ptr<MobileObject>, _unused_1: u32) {
    e.vcall(this.addr(), 0x140, &args![]);
    let process = e.get(this, MobileObject::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0x10, &args![this]);
    }
    if e.call(0x0057_4900, &args![this]).bool() && !e.call(0x0093_36c0, &args![this]).bool() {
        let value = e.call(0x005e_3fa0, &args![this]).u32();
        if value != 0 {
            let next = e.call(0x005d_43c0, &args![value]).u32();
            e.call(0x0042_e130, &args![next]);
        }
        e.call(0x0096_d470, &args![0x011e_0e80u32, this, 0u32]);
        e.call(0x0096_e870, &args![0x011e_0e80u32, this]);
    }
}

fn move_copy_vec(e: &mut Engine, destination: u32, source: u32) {
    for component in 0..3 {
        let word = e.mem.u32(source + 4 * component);
        e.mem.set_u32(destination + 4 * component, word);
    }
}

fn move_set_heading_matrix(e: &mut Engine, this: Ptr<MobileObject>, matrix: u32) {
    let heading = if this.addr() == e.global::<u32>(0x011d_ea3c) {
        e.call(0x0080_f790, &args![this, 0u32]).f32()
    } else {
        e.vcall(this.addr(), 0x2bc, &args![0u32]).f32()
    };
    e.call(0x004a_0c90, &args![matrix, heading]);
}

fn move_angle_matrix(e: &mut Engine, this: Ptr<MobileObject>, matrix: u32) {
    e.call(0x0068_15c0, &args![matrix]);
    move_set_heading_matrix(e, this, matrix);
}

fn move_rotate_vec(e: &mut Engine, matrix: u32, output: u32, vector: Ptr) {
    let rotated = e.call(0x004b_4500, &args![matrix, output, vector]).u32();
    move_copy_vec(e, vector.addr(), rotated);
}

fn move_angle_invalid(e: &mut Engine, angle: f32) -> bool {
    (angle as f64) == e.global::<f64>(0x0102_31b0)
        || e.call(0x00ec_75b1, &args![angle as f64]).u32() != 0
        || e.call(0x00ec_7595, &args![angle as f64]).u32() == 0
}

// Translated from 0092f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::Move` (Xbox PDB): applies orientation, navigation, water,
/// controller and world-position paths according to the PC branch conditions.
/// Compiler exception-only SEH cleanup is not modeled.
pub fn mobile_object_move(
    e: &mut Engine,
    this: Ptr<MobileObject>,
    mut delta: f32,
    input: Ptr,
    flags: u32,
) -> Ptr {
    e.with_stack(0x390, |e, stack| {
        let at = |offset: u32| stack.addr() + 0x390 - offset;
        let actor = e.vcall(this.addr(), 0x100, &args![]).bool();
        let controller = mobile_object_get_char_controller(e, this);
        let _initial_state = e.vcall(this.addr(), 0x224, &args![]).u8();
        if actor {
            let process = this.addr() + 0xa4;
            if e.vcall(process, 8, &args![0x33u32]).i32() > 0 {
                let divisor = e.call(0x0071_6440, &args![]).f64();
                delta = (delta as f64 / divisor) as f32;
            }
        }
        let position = e.call(0x0043_6aa0, &args![this]).u32();
        move_copy_vec(e, at(0x30), position);
        let first_input = e.mem.f32(input.addr());
        if e.call(0x00ec_75b1, &args![first_input as f64]).u32() == 0 {
            let second_input = e.mem.f32(input.addr() + 4);
            if e.call(0x00ec_75b1, &args![second_input as f64]).u32() == 0 {
                let third_input = e.mem.f32(input.addr() + 8);
                e.call(0x00ec_75b1, &args![third_input as f64]);
            }
        }
        for component in 0..3 {
            let angle = e.call(0x0043_0830, &args![this]).u32();
            let value = quiet_f32(e.mem.f32(angle + 4 * component));
            e.mem.set_f32(at(0x34) - 4 * component, value);
        }
        let mut invalid_angle = false;
        for component in 0..3 {
            let angle = e.mem.f32(at(0x34) - 4 * component);
            if move_angle_invalid(e, angle) {
                invalid_angle = true;
                break;
            }
        }
        if invalid_angle {
            let object = e.call(0x0084_e3a0, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0108_a7a8u32, name, object]);
            let zero = e
                .call(0x0041_6870, &args![at(0x48), 0.0f32, 0.0f32, 0.0f32])
                .u32();
            let x = e.mem.u32(zero);
            let y = e.mem.u32(zero + 4);
            let z = e.mem.u32(zero + 8);
            e.call(0x0057_5700, &args![this, x, y, z]);
        }
        let physics_count_ptr = e.call(0x0043_d4d0, &args![0x011e_05b4u32]).u32();
        let physics_count = e.mem.u32(physics_count_ptr);
        if physics_count != 0 {
            let period_index = fn_009304a0(e).addr();
            if e.global::<u32>(0x011e_054c) != period_index {
                let divisor_ptr = e.call(0x0043_d4d0, &args![0x011e_05b4u32]).u32();
                let divisor = e.mem.u32(divisor_ptr).wrapping_add(1);
                e.set_global(0x011e_0550, u8::from(period_index % divisor != 0));
                e.set_global(0x011e_054c, period_index);
            }
            if e.global::<u8>(0x011e_0550) != 0 && e.call(0x0088_b0c0, &args![controller]).bool() {
                move_angle_matrix(e, this, at(0x70));
                move_rotate_vec(e, at(0x70), at(0x7c), input);
                let anim = e.vcall(this.addr(), 0x1d0, &args![]).u32();
                let source = e.call(0x0043_c490, &args![anim]).u32();
                let changed = e.call(0x0043_9e90, &args![source, at(0x88), input]).u32();
                move_copy_vec(e, at(0x30), changed);
                let anim = e.vcall(this.addr(), 0x1d0, &args![]).u32();
                e.call(0x0044_0460, &args![anim, at(0x30)]);
                return controller;
            }
            let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            let anim = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            let source = e.call(0x0043_c490, &args![anim]).u32();
            e.call(0x0043_9ef0, &args![source, at(0x94), location]);
            move_angle_matrix(e, this, at(0xb8));
            let inverse = e.call(0x004b_45b0, &args![at(0xb8), at(0xe8)]).u32();
            let rotated = e
                .call(0x004b_4500, &args![inverse, at(0xc4), at(0x94)])
                .u32();
            e.call(0x0063_c8a0, &args![input, rotated]);
        }
        let immobile = actor && e.vcall(this.addr(), 0x4b4, &args![]).bool();
        let movement_mode = e.vcall(this.addr(), 0x214, &args![]).u32();
        let can_move = if movement_mode != 0 && !controller.is_null() {
            let movement_fraction = quiet_f32(e.global::<f32>(0x0101_7d00));
            let movement_ptr = e.call(0x0045_8b20, &args![]).u32();
            let controller_data = e.call(0x0089_d600, &args![controller]).u32();
            e.call(
                0x0056_1730,
                &args![controller_data, movement_ptr, movement_fraction],
            )
            .u32()
                != 0
        } else {
            false
        };
        let controller_branch = movement_mode == 0 || can_move;
        let global_camera = e.call(0x0052_4c90, &args![]).u32();
        let camera_position = e.call(0x0045_bb80, &args![global_camera]).u32();
        let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        e.call(0x0043_9ef0, &args![location, at(0x10c), camera_position]);
        let camera_distance_sq = e.call(0x004a_7290, &args![at(0x10c)]).f32();
        let down = if actor {
            e.call(0x0043_7bd0, &args![this]).bool()
                || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
                || e.vcall(this.addr(), 0x2e8, &args![]).bool()
                || e.vcall(this.addr(), 0x230, &args![]).bool()
        } else {
            false
        };
        let warp_setting = e.call(0x0040_3e20, &args![0x0126_7bd4u32]).u32();
        let warp_distance_sq = e.mem.f32(warp_setting);
        if camera_distance_sq > warp_distance_sq
            && !e.vcall(this.addr(), 0x224, &args![]).bool()
            && !immobile
            && !down
            && this.addr() != e.global::<u32>(0x011d_ea3c)
        {
            move_copy_vec(e, at(0x128), at(0x30));
            move_copy_vec(e, at(0x134), input.addr());
            move_angle_matrix(e, this, at(0x158));
            let rotated = e
                .call(0x004b_4500, &args![at(0x158), at(0x164), at(0x134)])
                .u32();
            move_copy_vec(e, at(0x134), rotated);
            e.call(0x0063_c8a0, &args![at(0x128), at(0x134)]);
            let world = e.call(0x0057_5d70, &args![this]).u32();
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            let navmesh_found = e
                .call(0x006d_97b0, &args![at(0x128), cell, world, at(0x170)])
                .bool();
            if navmesh_found {
                let mut height = e.mem.f32(at(0x128) + 8);
                if !controller.is_null() && e.call(0x005c_0880, &args![controller]).u32() == 5 {
                    let loaded = e.mem.u32(this.addr() + 0x64);
                    if (e.mem.f32(loaded + 8) as f64) == e.global::<f64>(0x0102_41b0) {
                        let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                        let world = e.global::<u32>(0x011d_ea10);
                        let water_height = e.call(0x0045_cbc0, &args![world, location, 0u32]).f32();
                        let loaded = e.mem.u32(this.addr() + 0x64);
                        e.mem.set_f32(loaded + 8, quiet_f32(water_height));
                    }
                    let water_height = e.call(0x0057_b0a0, &args![this]).f32();
                    if (water_height as f64) == e.global::<f64>(0x0102_41b0) {
                        let kind = if e.call(0x0087_f390, &args![this]).bool() {
                            2
                        } else {
                            0
                        };
                        e.call(0x0088_b0a0, &args![controller, kind]);
                        height = e.mem.f32(at(0x170));
                    } else if height <= water_height {
                        let navmesh_height = e.mem.f32(at(0x170));
                        if height < navmesh_height && navmesh_height < water_height {
                            height = navmesh_height;
                        }
                    } else {
                        height = water_height;
                    }
                } else if movement_mode == 0 && !can_move {
                    height = e.mem.f32(at(0x170));
                }
                e.mem.set_f32(at(0x128) + 8, height);
                if !controller.is_null()
                    && e.call(0x0062_1ad0, &args![controller]).u32() != 0
                    && !fn_00930570(e, controller)
                    && (fn_009305a0(e, controller)
                        || fn_009305a0(e, controller)
                        || e.call(0x008a_5170, &args![controller.addr() + 0x410])
                            .bool())
                {
                    e.call(0x0068_15c0, &args![at(0x180)]);
                    e.call(0x006d_cd70, &args![at(0x1a8), this]);
                    e.call(0x006d_9100, &args![at(0x1a8), at(0x180)]);
                    e.call(0x0068_15c0, &args![at(0x1c0)]);
                    e.call(0x004a_3e00, &args![at(0x1c0), at(0x180)]);
                    e.call(0x0092_8d00, &args![controller, at(0x1c0)]);
                    fn_00930620(e, controller);
                    e.call(0x00c6_e820, &args![controller]);
                    e.call(0x00c7_0060, &args![controller]);
                    let angle = e.call(0x0043_0830, &args![this]).u32();
                    let pitch = quiet_f32(e.mem.f32(angle + 8));
                    e.call(0x0062_40d0, &args![at(0x1e0)]);
                    e.call(
                        0x00cb_2450,
                        &args![at(0x1e0), controller.addr() + 0x4c0, -pitch],
                    );
                    e.call(0x00c6_e420, &args![controller, at(0x1e0)]);
                    e.call(0x004f_f7e0, &args![at(0x1a8)]);
                }
                e.vcall(this.addr(), 0x2a8, &args![at(0x128)]);
                if !controller.is_null() && e.call(0x005c_0880, &args![controller]).u32() == 2 {
                    e.call(0x008e_2680, &args![controller, 0u32]);
                }
                return controller;
            }
        }
        if controller_branch {
            if this.addr() == e.global::<u32>(0x011d_ea3c) {
                e.call(
                    0x0062_9670,
                    &args![
                        controller.addr() + 0x410,
                        0x0800_0000u32,
                        u8::from(immobile)
                    ],
                );
            }
            if !controller.is_null() && !e.vcall(this.addr(), 0x22c, &args![1u32]).bool() {
                fn_009304d0(e, Ptr::new(at(0x208)));
                e.mem.set_f32(at(0x208), quiet_f32(delta));
                let angle = e.call(0x0043_0830, &args![this]).u32();
                move_copy_vec(e, at(0x204), angle);
                move_copy_vec(e, at(0x1f8), input.addr());
                e.mem.set_u32(at(0x1ec), flags);
                e.mem.set_u8(at(0x1e4), 1);
                if this.addr() != e.global::<u32>(0x011d_ea3c) && actor {
                    let animation: Ptr = e.vcall(this.addr(), 0x1d0, &args![]).ptr();
                    let target: Ptr = if animation.is_null() {
                        Ptr::NULL
                    } else {
                        e.vcall(animation.addr(), 0x10, &args![]).ptr()
                    };
                    let can_follow = if target.is_null() {
                        true
                    } else {
                        let reach = quiet_f32(e.global::<f32>(0x0103_0ff0));
                        let globals = e.call(0x0052_4c90, &args![]).u32();
                        e.call(0x00b4_e030, &args![target, globals, reach]).bool()
                    };
                    e.mem.set_u8(at(0x1e4), u8::from(can_follow));
                }
                if e.call(0x0062_1240, &args![controller]).bool() && !can_move {
                    e.call(0x0068_15c0, &args![at(0x224)]);
                    move_angle_matrix(e, this, at(0x248));
                    move_rotate_vec(e, at(0x248), at(0x254), input);
                    e.call(0x0063_c8a0, &args![at(0x30), input]);
                    e.call(0x0081_2b00, &args![controller, at(0x224)]);
                    let change = e
                        .call(0x0043_9ef0, &args![at(0x30), at(0x260), at(0x224)])
                        .u32();
                    move_copy_vec(e, input.addr(), change);
                    let inverse = e.call(0x004b_45b0, &args![at(0x248), at(0x290)]).u32();
                    let rotated = e.call(0x004b_4500, &args![inverse, at(0x26c), input]).u32();
                    move_copy_vec(e, at(0x1f8), rotated);
                }
                if fn_009305a0(e, controller) || fn_009305d0(e, controller) {
                    e.call(0x0062_9650, &args![controller.addr() + 0x410, 0u32]);
                    let setting = e.call(0x0040_3e20, &args![0x011e_0574u32]).u32();
                    let factor = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
                    let value = (factor * e.mem.f32(setting) as f64) as f32;
                    fn_00930600(e, controller, value);
                }
                let speed = if actor && e.call(0x0088_46e0, &args![this]).u32() & 0x3f != 0 {
                    e.call(0x008a_0b10, &args![this]).f32()
                } else {
                    0.0
                };
                e.mem.set_f32(at(0x1e8), speed);
                if !immobile
                    || (this.addr() != e.global::<u32>(0x011d_ea3c)
                        && e.vcall(this.addr(), 0x38c, &args![]).bool())
                {
                    e.vcall(controller.addr(), 0xc8, &args![at(0x208)]);
                } else {
                    let is_player = u8::from(this.addr() == e.global::<u32>(0x011d_ea3c));
                    e.call(0x00c7_0b60, &args![controller, at(0x208), is_player]);
                }
                if fn_009304b0(e, Ptr::new(controller.addr() + 0x410)) {
                    e.call(0x0068_15c0, &args![at(0x2a0)]);
                    e.call(0x0081_2b00, &args![controller, at(0x2a0)]);
                    let radius = e.call(0x00c6_e280, &args![controller]).f32();
                    let doubled = (2.0 * e.call(0x0045_87d0, &args![radius]).f64()) as f32;
                    let current_z = e.mem.f32(at(0x2a0) + 8);
                    e.mem
                        .set_f32(at(0x2a0) + 8, (current_z as f64 + doubled as f64) as f32);
                    e.vcall(this.addr(), 0x2a8, &args![at(0x2a0)]);
                }
                if !e.call(0x0062_1240, &args![controller]).bool() {
                    e.call(0x0081_2b00, &args![controller, at(0x30)]);
                    if e.call(0x008a_5170, &args![controller.addr() + 0x410])
                        .bool()
                        && !immobile
                    {
                        let angle_z = quiet_f32(e.mem.f32(at(0x204) + 8));
                        e.vcall(this.addr(), 0x2c4, &args![angle_z]);
                    }
                    let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                    let difference = e
                        .call(0x0043_9ef0, &args![at(0x30), at(0x2b0), location])
                        .u32();
                    let dist_sq = e.call(0x004a_7290, &args![difference]).f64();
                    if dist_sq > e.global::<f64>(0x0108_a7a0) {
                        let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                        move_copy_vec(e, at(0x30), location);
                        e.vcall(this.addr(), 0x2a8, &args![at(0x30)]);
                    }
                    e.mem.set_f32(at(0x2b4), 0.0);
                    let tes = e.global::<u32>(0x011d_ea10);
                    if e.call(0x005f_36f0, &args![tes]).u32() == 0 {
                        let tes = e.global::<u32>(0x011d_ea10);
                        e.call(0x0045_72e0, &args![tes, at(0x30), at(0x2b4)]);
                        let height = e.mem.f32(at(0x2b4));
                        if (height as f64 - e.mem.f32(at(0x30) + 8) as f64)
                            > e.global::<f64>(0x0101_db88)
                        {
                            e.mem.set_f32(at(0x30) + 8, height);
                            e.vcall(this.addr(), 0x2a8, &args![at(0x30)]);
                        }
                    }
                }
                if actor {
                    e.call(0x008b_8f90, &args![this]);
                }
            } else {
                e.call(0x0068_15c0, &args![at(0x2d8)]);
                if e.vcall(this.addr(), 0x224, &args![]).bool() {
                    let looking = e.call(0x0093_1d70, &args![this]).f32();
                    e.call(0x0052_4ac0, &args![at(0x2d8), looking]);
                    move_rotate_vec(e, at(0x2d8), at(0x2e4), input);
                }
                move_set_heading_matrix(e, this, at(0x2d8));
                move_rotate_vec(e, at(0x2d8), at(0x2f0), input);
                e.call(0x0063_c8a0, &args![at(0x30), input]);
            }
        } else if e.vcall(this.addr(), 0x22c, &args![0u32]).bool() && !immobile {
            move_angle_matrix(e, this, at(0x314));
            move_rotate_vec(e, at(0x314), at(0x320), input);
            e.call(0x0063_c8a0, &args![at(0x30), input]);
        }

        if movement_mode != 0 || can_move {
            fn_009304d0(e, Ptr::new(at(0x348)));
            e.mem.set_f32(at(0x348), quiet_f32(delta));
            let angle = e.call(0x0043_0830, &args![this]).u32();
            move_copy_vec(e, at(0x344), angle);
            move_copy_vec(e, at(0x338), input.addr());
            e.mem.set_u32(at(0x32c), flags);
            e.mem.set_u8(at(0x324), 1);
            if !can_move {
                e.call(0x00c7_0b60, &args![controller, at(0x348), 0u32]);
            }
            if can_move {
                fn_00930540(e, controller, 1);
                e.call(0x008e_2680, &args![controller, 0u32]);
            }
            move_angle_matrix(e, this, at(0x36c));
            move_rotate_vec(e, at(0x36c), at(0x378), input);
            e.call(0x0063_c8a0, &args![at(0x30), input]);
        }
        e.call(0x0057_5830, &args![this, at(0x30)]);
        controller
    })
}

// Translated from 009304a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the pointer stored at the global 0x011A2FE0.
pub fn fn_009304a0(e: &mut Engine) -> Ptr {
    e.global(0x011a_2fe0)
}

// Translated from 009304b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x20 through the object's flag helper at 0x00621270.
pub fn fn_009304b0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this, 0x20u32]).u32() != 0
}

// Translated from 009304d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the 0x25-byte movement temporary from the three global float
/// words, copying their exact bits into both triples.
pub fn fn_009304d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    for index in 0..3 {
        let bits = e.mem.u32(0x011f_426c + 4 * index);
        e.mem.set_u32(this.addr() + 4 + 4 * index, bits);
        e.mem.set_u32(this.addr() + 0x10 + 4 * index, bits);
    }
    e.mem.set_u32(this.addr() + 0x20, 0);
    e.mem.set_u8(this.addr() + 0x24, 1);
    this
}

// Translated from 00930540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 0x100 in the flag set embedded at offset 0x410.
pub fn fn_00930540(e: &mut Engine, this: Ptr, value: u8) {
    e.call(0x0062_9670, &args![this.addr() + 0x410, 0x100u32, value]);
}

// Translated from 00930570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x40000 in the flag set at offset 0x410.
pub fn fn_00930570(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this.addr() + 0x410, 0x40000u32])
        .u32()
        != 0
}

// Translated from 009305a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x2000000 in the flag set at offset 0x410.
pub fn fn_009305a0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this.addr() + 0x410, 0x2000000u32])
        .u32()
        != 0
}

// Translated from 009305d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x4000000 in the flag set at offset 0x410.
pub fn fn_009305d0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this.addr() + 0x410, 0x4000000u32])
        .u32()
        != 0
}

// Translated from 00930600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at offset 0x558.
pub fn fn_00930600(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x558, quiet_f32(value));
}

// Translated from 00930620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the global float at 0x011B014C to the object at offset 0x530.
pub fn fn_00930620(e: &mut Engine, this: Ptr) {
    let value = quiet_f32(e.mem.f32(0x011b_014c));
    e.mem.set_f32(this.addr() + 0x530, value);
}

// Translated from 00930640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales a character-controller property by the reference scale and a
/// setting. Controller state five adds a second setting multiplier.
pub fn fn_00930640(e: &mut Engine, this: Ptr<MobileObject>) -> Ptr {
    let controller = mobile_object_get_char_controller(e, this);
    if !controller.is_null() {
        let setting = e.call(0x0040_3e20, &args![0x011d_0274u32]).u32();
        let scale = e.call(0x0056_7400, &args![this]).f32();
        let mut value = (scale as f64 * e.mem.f32(setting) as f64) as f32;
        if fn_009306b0(e, controller) {
            let setting = e.call(0x0040_3e20, &args![0x011d_13a4u32]).u32();
            value = (value as f64 * e.mem.f32(setting) as f64) as f32;
        }
        e.call(0x00c6_d4b0, &args![controller, value]);
    }
    controller
}

// Translated from 009306b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object's helper at 0x005C0880 returns five.
pub fn fn_009306b0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x005c_0880, &args![this]).u32() == 5
}

// Translated from 009306d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::GetCharController` (Xbox PDB): obtains it through the
/// current process' virtual method at slot 0x28C, or null without a process.
pub fn mobile_object_get_char_controller(e: &mut Engine, this: Ptr<MobileObject>) -> Ptr {
    let process = e.get(this, MobileObject::pCurrentProcess);
    if process.is_null() {
        Ptr::NULL
    } else {
        e.vcall(process.addr(), 0x28c, &args![]).ptr()
    }
}

// Translated from 00930700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the parent cell. For an active controller, updates water height;
/// the special water sentinel uses the loaded reference's height at +8.
pub fn fn_00930700(e: &mut Engine, this: Ptr<MobileObject>, cell: Ptr) {
    let current = e.call(0x008d_6f30, &args![this]).ptr();
    if current == cell {
        return;
    }
    e.call(0x0057_5bb0, &args![this, cell]);
    if cell.is_null() || e.call(0x0057_4900, &args![this]).bool() {
        return;
    }
    let controller = mobile_object_get_char_controller(e, this);
    if controller.is_null() {
        return;
    }
    let height = e.call(0x0057_b0a0, &args![this]).f32();
    e.call(0x0057_b1d0, &args![controller, height]);
    if e.call(0x0087_f390, &args![this]).bool()
        && e.call(0x005c_0880, &args![controller]).u32() == 5
        && (e.call(0x0057_b0a0, &args![this]).f32() as f64) == e.global::<f64>(0x0102_41b0)
    {
        let loaded = e.mem.u32(this.addr() + 0x64);
        if loaded != 0 {
            let location = e.vcall(this.addr(), 0x1f4, &args![0u32]).u32();
            let world = e.global::<u32>(0x011d_ea10);
            let height = e.call(0x0045_cbc0, &args![world, location, 0u32]).f32();
            e.mem.set_f32(loaded + 8, quiet_f32(height));
        }
    }
}

// Translated from 00930810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the controller through virtual slot 0x2A0; if a process exists,
/// also passes a constructed one-word temporary to its slot 0x290.
pub fn fn_00930810(e: &mut Engine, this: Ptr<MobileObject>) -> u8 {
    let removed = e.vcall(this.addr(), 0x2a0, &args![]).u8();
    let process = e.get(this, MobileObject::pCurrentProcess);
    if !process.is_null() {
        e.with_stack(4, |e, slot| {
            e.call(0x0063_3c90, &args![slot, 0u32]);
            let value = e.mem.u32(slot.addr());
            e.vcall(process.addr(), 0x290, &args![value]);
        });
    }
    removed
}

// Translated from 009307d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the controller's display geometry and clears its world, returning
/// whether this object had a controller.
pub fn fn_009307d0(e: &mut Engine, this: Ptr<MobileObject>) -> bool {
    let controller = mobile_object_get_char_controller(e, this);
    if controller.is_null() {
        return false;
    }
    e.call(0x00c6_ffc0, &args![controller]);
    e.call(0x00c7_0c90, &args![controller, 0u32]);
    true
}

// Translated from 00930870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a process with no active mode, removes the character controller,
/// then removes any attached Havok objects and sets bit zero in the result.
pub fn fn_00930870(e: &mut Engine, this: Ptr<MobileObject>) -> u8 {
    let process = e.get(this, MobileObject::pCurrentProcess);
    if !process.is_null() && e.call(0x0045_cd60, &args![process]).u32() == 0 {
        let process = e.get(this, MobileObject::pCurrentProcess);
        e.call(0x008e_56b0, &args![process]);
    }
    let mut result = e.vcall(this.addr(), 0x2a0, &args![]).u32() as u8;
    let havok = e.call(0x0043_fcd0, &args![this]).u32();
    if havok != 0 {
        e.call(0x00c6_9ee0, &args![havok, 1u32, 0u32]);
        result |= 1;
    }
    result
}

// Translated from 009308f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Synchronizes the character controller's world, attached geometry and
/// actor-specific ragdoll state after the reference changes cell or world.
pub fn fn_009308f0(e: &mut Engine, this: Ptr<MobileObject>, _unused_1: u32) {
    let controller = mobile_object_get_char_controller(e, this);
    if controller.is_null() {
        return;
    }
    e.with_stack(0x50, |e, stack| {
        let at = |offset: u32| stack.addr() + 0x50 - offset;
        let actor = e.vcall(this.addr(), 0x100, &args![]).bool();
        let special_actor = actor && e.call(0x004f_8960, &args![this]).u32() == 2;
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        let world = if cell != 0 {
            e.call(0x0045_43c0, &args![cell]).u32()
        } else {
            0
        };
        let old_world = e.call(0x0062_1ad0, &args![controller]).u32();
        if world != old_world {
            e.call(0x0088_b0a0, &args![controller, 0u32]);
            let height = if cell != 0 {
                quiet_f32(e.call(0x0054_71e0, &args![cell]).f32())
            } else {
                0.0
            };
            e.call(0x0057_b1d0, &args![controller, height]);
            if !special_actor {
                e.call(0x00c7_0c90, &args![controller, world]);
            }
            let position = e.call(0x0043_6aa0, &args![this]).u32();
            e.call(0x0056_20e0, &args![controller, position]);
            let process = e.mem.u32(this.addr() + 0x68);
            if e.call(0x0045_cd60, &args![process]).u32() == 0
                && this.addr() != e.global::<u32>(0x011d_ea3c)
                && actor
                && e.call(0x0041_81e0, &args![this]).u32() != 0
                && !e.call(0x0087_eeb0, &args![this]).bool()
            {
                let process = e.mem.u32(this.addr() + 0x68);
                e.call(0x008e_4e50, &args![process, this]);
            }
        }
        let ni_object = e.call(0x0081_0660, &args![controller]).u32();
        let target_object = e.call(0x0043_fcd0, &args![this]).u32();
        if ni_object != target_object {
            e.call(0x00c6_d7a0, &args![controller, target_object]);
        }
        let world_changed = old_world != world;
        if world_changed && old_world != 0 {
            e.call(0x00c6_9ee0, &args![target_object, 1u32, 0u32]);
        }
        if world == 0 {
            return;
        }
        if world_changed {
            e.call(0x0070_c440, &args![controller, at(0x34)]);
            let state = e.call(0x004a_3a20, &args![at(0x34)]).u32();
            let active = fn_00930c50(e, Ptr::new(world));
            if !active {
                e.call(0x00c6_6300, &args![world]);
            }
            e.vcall(world, 0xd0, &args![target_object, 1u32, 0u32, state, 0u32]);
            if !active {
                e.call(0x00c6_b540, &args![world, 0u32]);
            }
        } else {
            e.call(0x00c6_9f50, &args![target_object, 1u32, 0u32]);
        }
        if actor {
            let ragdoll = e.mem.u32(this.addr() + 0xac);
            if ragdoll != 0 && e.call(0x0087_ea20, &args![ragdoll]).bool() {
                if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
                    let ragdoll = e.mem.u32(this.addr() + 0xac);
                    e.call(0x00c7_d810, &args![ragdoll]);
                }
                if e.call(0x0087_eeb0, &args![this]).bool() {
                    let ragdoll = e.mem.u32(this.addr() + 0xac);
                    e.call(0x00c7_c150, &args![ragdoll, 1u32]);
                }
            }
        }
        if e.vcall(this.addr(), 0x214, &args![]).u32() != 0
            && e.call(0x008d_8520, &args![this]).u32() != 0
        {
            let first = e.call(0x008d_8520, &args![this]).u32();
            let second = e.call(0x008d_8520, &args![this]).u32();
            let third = e.call(0x008d_8520, &args![this]).u32();
            let source = e.vcall(first, 0x4d4, &args![]).u32();
            let lookup = e.call(0x0050_94b0, &args![source]).u32();
            let selected = e.vcall(second, 0x4c8, &args![lookup]).u32();
            if e.vcall(third, 0x2b4, &args![selected]).bool() {
                let amount = quiet_f32(e.global::<f32>(0x0101_e2bc));
                e.vcall(this.addr(), 0x2e0, &args![amount]);
                e.call(0x0093_1fb0, &args![this, 1u32]);
                e.call(
                    0x0062_9670,
                    &args![controller.addr() + 0x410, 0x0800_0000u32, 1u32],
                );
            }
        }
    });
}

// Translated from 00930c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the word at offset 0x18 is nonzero.
pub fn fn_00930c50(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) != 0
}

// Translated from 00930c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes or reconnects the character controller, optional limb bodies,
/// world state and special actor geometry. Compiler exception-only SEH cleanup
/// for temporary construction is not modeled.
pub fn mobile_object_init_havok(e: &mut Engine, this: Ptr<MobileObject>) {
    e.call(0x0057_b520, &args![this, 0u32]);
    let process = e.mem.u32(this.addr() + 0x68);
    if process == 0 {
        return;
    }
    if e.call(0x0045_cd60, &args![process]).u32() != 0 {
        let process = e.mem.u32(this.addr() + 0x68);
        if e.call(0x0045_cd60, &args![process]).u32() != 1 {
            return;
        }
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let process = e.mem.u32(this.addr() + 0x68);
    let ordinary_process = if e.call(0x0045_cd60, &args![process]).u32() == 0 {
        e.mem.u32(this.addr() + 0x68)
    } else {
        0
    };
    if cell != 0
        && !e.call(0x0045_0ff0, &args![cell]).bool()
        && !e.call(0x0045_23c0, &args![cell]).bool()
    {
        return;
    }
    let world = if cell != 0 {
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    let actor = e.vcall(this.addr(), 0x100, &args![]).bool();
    let special = if actor {
        let last_idle = e.call(0x005a_29b0, &args![this]).u32();
        if last_idle == 2 || last_idle == 4 {
            return;
        }
        let flag = e.call(0x004f_8960, &args![this]).u32() == 2
            || e.call(0x008a_ad40, &args![this, u32::MAX]).bool();
        let geometry = e.call(0x0043_fcd0, &args![this]).u32();
        e.call(0x00c6_c210, &args![geometry]);
        flag
    } else {
        false
    };
    let extras = e.call(0x005d_43c0, &args![this]).u32();
    let dismemberment = e.call(0x0042_e8c0, &args![extras]).u32();
    if dismemberment != 0
        && e.mem.u32(this.addr() + 0x68) != 0
        && e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0
    {
        let count = e.call(0x008b_6800, &args![dismemberment]).u32();
        for index in 0..count {
            let limb_ptr = e.call(0x0044_1420, &args![dismemberment, index]).u32();
            let limb = e.mem.u8(limb_ptr);
            let process = e.mem.u32(this.addr() + 0x68);
            let body = e.vcall(process, 0x6d0, &args![limb]).u32();
            let object = e.call(0x0096_11e0, &args![body]).u32();
            let limb = e.mem.u8(limb_ptr);
            let name = e.mem.u32(0x0119_6d6c + u32::from(limb) * 4);
            e.with_stack(4, |e, name_slot| {
                e.call(0x0043_8170, &args![name_slot, name]);
                let matched = if object != 0 {
                    let candidate = e.call(0x0041_3f40, &args![object, name_slot]).u32();
                    e.call(0x009a_3830, &args![candidate, name_slot]).bool()
                } else {
                    false
                };
                if !matched {
                    let mut rigid_index = 0;
                    loop {
                        let rigid_count = e.call(0x0043_b480, &args![body]).u32();
                        if rigid_index >= rigid_count {
                            break;
                        }
                        let rigid = e.call(0x0043_b4a0, &args![body, rigid_index]).u32();
                        if rigid != 0 {
                            e.call(0x00c6_a200, &args![rigid, 1u32, 1u32, 1u32]);
                        }
                        rigid_index = rigid_index.wrapping_add(1);
                    }
                }
                e.call(0x0043_81b0, &args![name_slot]);
            });
        }
    }
    let mut controller = mobile_object_get_char_controller(e, this);
    if controller.is_null() {
        e.with_stack(0x140, |e, temporary| {
            e.call(0x00c6_da50, &args![temporary, 1u32]);
            let geometry = e.call(0x0043_fcd0, &args![this]).u32();
            e.call(0x008a_c890, &args![temporary, geometry]);
            let location = e.call(0x0043_6aa0, &args![this]).u32();
            e.call(0x0081_0510, &args![temporary, location]);
            e.call(0x0085_7230, &args![temporary, 0u32]);
            let scale = quiet_f32(e.vcall(this.addr(), 0x2e4, &args![]).f32());
            e.call(0x0081_0550, &args![temporary, scale]);
            let saved = if actor {
                e.call(0x007a_f430, &args![this]).u32()
            } else {
                0
            };
            if saved != 0 && e.vcall(this.addr(), 0x21c, &args![]).bool() {
                let saved = e.call(0x007a_f430, &args![this]).u32();
                let flag = e.call(0x005f_90c0, &args![saved]).u8();
                fn_009314f0(e, temporary, flag);
                let saved = e.call(0x007a_f430, &args![this]).u32();
                let flag = e.call(0x005f_90e0, &args![saved]).u8();
                e.call(0x006e_2b50, &args![temporary, flag]);
                let scale = e.call(0x0056_7400, &args![this]).f32();
                fn_009314d0(e, temporary, scale);
            } else {
                fn_00931510(e, temporary, 1);
            }
            let allocation = e.call(0x0055_3300, &args![0x650u32]).u32();
            controller = if allocation != 0 {
                e.call(0x00c7_41e0, &args![allocation, temporary]).ptr()
            } else {
                Ptr::NULL
            };
            if actor && e.call(0x007a_f430, &args![this]).u32() != 0 {
                let value = e.call(0x0041_81e0, &args![this]).u32();
                let flag = e.call(0x0089_d5e0, &args![value + 0x30]).u8();
                e.call(
                    0x0062_9670,
                    &args![controller.addr() + 0x410, 0x1000_4000u32, flag],
                );
            }
            e.with_stack(4, |e, value| {
                e.call(0x0063_3c90, &args![value, controller]);
                let word = e.mem.u32(value.addr());
                let process = e.mem.u32(this.addr() + 0x68);
                e.vcall(process, 0x290, &args![word]);
            });
            e.call(0x0081_0640, &args![controller, this]);
            let scale = e.call(0x0056_7400, &args![this]).f32();
            fn_00931560(e, controller, scale);
            let flag = e.call(0x0052_5420, &args![]).u8();
            e.call(0x0081_0610, &args![controller, flag]);
            e.call(
                0x0062_9670,
                &args![controller.addr() + 0x410, 0x200000u32, 1u32],
            );
            if ordinary_process != 0 {
                let shape = fn_00931580(e, Ptr::new(ordinary_process)).addr();
                if shape != 0 {
                    e.with_stack(8, |e, words| {
                        e.call(0x0043_b4f0, &args![shape, words]);
                        let current = e
                            .call(0x0070_c440, &args![controller, words.addr() + 4])
                            .u32();
                        let transformed = e.call(0x004a_3a20, &args![current]).u32();
                        e.call(0x0059_ce80, &args![words, transformed]);
                        let object = e.mem.u32(words.addr());
                        e.call(0x0062_6cb0, &args![shape, object]);
                    });
                }
            }
            e.call(0x00c6_de10, &args![temporary]);
        });
    }
    if world != 0 && fn_00930c50(e, Ptr::new(world)) {
        fn_00931530(e, controller, 1);
    }
    if actor && special {
        e.with_stack(4, |e, output| {
            let value = e.call(0x0093_1ed0, &args![this, output]).u32();
            let setting = e.call(0x004a_3a20, &args![value]).u32();
            let slot = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            let ragdoll_state = e.mem.u32(this.addr() + 0xb0);
            e.call(0x00ca_2ad0, &args![ragdoll_state, slot, setting]);
        });
    }
    let previous_geometry = e.call(0x0081_0660, &args![controller]).u32();
    let geometry = e.call(0x0043_fcd0, &args![this]).u32();
    if previous_geometry != geometry {
        e.call(0x00c6_d7a0, &args![controller, geometry]);
        e.with_stack(16, |e, quaternion| {
            e.call(
                0x0041_4430,
                &args![quaternion, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            );
            e.mem.set_f32(quaternion.addr(), 1.0);
            e.mem.set_f32(quaternion.addr() + 4, 0.0);
            e.mem.set_f32(quaternion.addr() + 8, 0.0);
            let w = quiet_f32(e.global::<f32>(0x0101_622c));
            e.mem.set_f32(quaternion.addr() + 12, w);
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            let world = if cell != 0 {
                e.call(0x0045_43c0, &args![cell]).u32()
            } else {
                0
            };
            if world != 0 && e.call(0x004b_ae00, &args![world]).bool() {
                e.vcall(controller.addr(), 0xd0, &args![0u32]);
                let object = e.call(0x0081_0660, &args![controller]).u32();
                e.call(0x00b6_b910, &args![object, 1u32]);
            }
            let motion = if special { 1u32 } else { 4u32 };
            e.call(0x00c6_a350, &args![geometry, motion, 1u32, 0u32, 0u32]);
            if actor && !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
                e.call(0x0087_e130, &args![this, geometry, 1u32, 1u32, 0u32]);
                if this.addr() == e.global::<u32>(0x011d_ea3c) && e.mem.u32(this.addr() + 0xac) != 0
                {
                    let ragdoll = e.mem.u32(this.addr() + 0xac);
                    e.call(0x0087_ea60, &args![ragdoll, 0u32]);
                    let ragdoll = e.mem.u32(this.addr() + 0xac);
                    fn_009315a0(e, Ptr::new(ragdoll), 1);
                }
            }
        });
    }
    let process = e.mem.u32(this.addr() + 0x68);
    if process != 0
        && e.call(0x0045_cd60, &args![process]).u32() == 0
        && this.addr() != e.global::<u32>(0x011d_ea3c)
        && actor
        && e.call(0x0041_81e0, &args![this]).u32() != 0
        && !e.call(0x0087_eeb0, &args![this]).bool()
    {
        let process = e.mem.u32(this.addr() + 0x68);
        if !e.call(0x008e_5700, &args![process]).bool() {
            e.call(0x008e_4e50, &args![process, this]);
        }
    }
    e.vcall(this.addr(), 0x1fc, &args![0u32]);
    if special {
        let extras = e.call(0x005d_43c0, &args![this]).u32();
        if e.call(0x0042_2b90, &args![extras]).u32() == 0 {
            let extras = e.call(0x005d_43c0, &args![this]).u32();
            if e.call(0x0041_d6d0, &args![extras]).u32() == 0 {
                e.with_stack(12, |e, velocity| {
                    let z = quiet_f32(e.global::<f32>(0x0101_2054));
                    e.call(0x0041_6870, &args![velocity, 0.0f32, 0.0f32, z]);
                    e.call(0x0062_b8d0, &args![geometry, velocity, 1u32]);
                });
            }
        }
    }
}

// Translated from 009314d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at offset 0xAC.
pub fn fn_009314d0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0xac, quiet_f32(value));
}

// Translated from 009314f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of the argument at offset 0x97.
pub fn fn_009314f0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x97, value);
}

// Translated from 00931510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of the argument at offset 0x96.
pub fn fn_00931510(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x96, value);
}

// Translated from 00931530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 0x400000 in the flag set embedded at offset 0x410.
pub fn fn_00931530(e: &mut Engine, this: Ptr, value: u8) {
    e.call(0x0062_9670, &args![this.addr() + 0x410, 0x400000u32, value]);
}

// Translated from 00931560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at offset 0x55C.
pub fn fn_00931560(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x55c, quiet_f32(value));
}

// Translated from 00931580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the pointer getter on the member at offset 0x434.
pub fn fn_00931580(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0055_9450, &args![this.addr() + 0x434]).ptr()
}

// Translated from 009315a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of the argument at offset 0x241.
pub fn fn_009315a0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x241, value);
}

// Translated from 009315c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the controller geometry and recursively sets wireframe when enabled;
/// otherwise destroys its display geometry.
pub fn fn_009315c0(e: &mut Engine, this: Ptr<MobileObject>, show: u8) {
    let controller = mobile_object_get_char_controller(e, this);
    if controller.is_null() {
        return;
    }
    if show == 0 {
        e.call(0x00c6_ffc0, &args![controller]);
    } else {
        e.vcall(controller.addr(), 0xd0, &args![0u32]);
        let object = e.call(0x0081_0660, &args![controller]).u32();
        e.call(0x00b6_b910, &args![object, 1u32]);
    }
}

// Translated from 00931620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the reference location and updates the controller and attached node
/// when those parts exist.
pub fn fn_00931620(e: &mut Engine, this: Ptr<MobileObject>, location: Ptr) {
    e.call(0x0057_5830, &args![this, location]);
    let controller = mobile_object_get_char_controller(e, this);
    if !controller.is_null() {
        e.call(0x0056_20e0, &args![controller, location]);
        let status = e.call(0x0045_8b20, &args![]).u32();
        e.call(0x0080_fb00, &args![controller, status]);
        e.call(0x005d_10f0, &args![controller]);
        e.call(0x0087_cee0, &args![controller]);
        e.call(0x00c6_e030, &args![controller]);
    }
    let loaded = e.mem.u32(this.addr() + 0x64);
    if loaded != 0 {
        let node = e.call(0x0055_9450, &args![loaded + 0x14]).u32();
        if node != 0 {
            let loaded = e.mem.u32(this.addr() + 0x64);
            let node = e.call(0x0055_9450, &args![loaded + 0x14]).u32();
            e.call(0x0044_0460, &args![node, location]);
        }
    }
}

// Translated from 009316c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::SetPackageStartLocationFromCurrentLocation` (Xbox PDB):
/// forwards the world, cell, reference location and current z coordinate.
pub fn mobile_object_set_package_start_location(e: &mut Engine, this: Ptr<MobileObject>) {
    let position = e.call(0x0043_0830, &args![this]).u32();
    let z = quiet_f32(e.mem.f32(position + 8));
    let location = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = e.call(0x0057_5d70, &args![this]).u32();
    e.call(0x0056_ae10, &args![this, world, cell, location, z]);
}

// Translated from 00931710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the current process with a newly constructed high process,
/// transferring the old process state before notifying the process lists.
/// The compiler's exception-only SEH cleanup for a failed constructor is
/// not modeled; the complete normal path follows the PC disassembly.
pub fn mobile_object_replace_with_high_process(e: &mut Engine, this: Ptr<MobileObject>) -> bool {
    let current = e.get(this, MobileObject::pCurrentProcess);
    let process_type = e.call(0x0045_cd60, &args![current]).u32();
    e.call(0x0040_fbf0, &args![0x011f_11a0u32, 0u32]);
    e.call(0x0096_d470, &args![0x011e_0e80u32, this, process_type]);

    let allocation: Ptr = e.call(0x0040_1000, &args![0x46cu32]).ptr();
    let replacement = if allocation.is_null() {
        Ptr::NULL
    } else {
        e.call(0x008d_7510, &args![allocation]).ptr()
    };
    // Native code calls this virtual function even when allocation failed.
    let old_for_transfer = e.get(this, MobileObject::pCurrentProcess);
    e.vcall(replacement.addr(), 0x4, &args![old_for_transfer]);
    let old_for_destroy = e.get(this, MobileObject::pCurrentProcess);
    if !old_for_destroy.is_null() {
        e.vcall(old_for_destroy.addr(), 0, &args![1u32]);
    }
    e.set(this, MobileObject::pCurrentProcess, replacement);
    e.call(
        0x0096_d450,
        &args![0x011e_0e80u32, this, 0u32, 0u32, 0u32, 0u32],
    );
    e.call(0x0040_fba0, &args![0x011f_11a0u32]);
    e.vcall(this.addr(), 0x1fc, &args![0u32]);
    let installed = e.get(this, MobileObject::pCurrentProcess);
    e.vcall(installed.addr(), 0x48, &args![]);
    true
}

// Translated from 00931850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObject::GetCurrentProcessType` (Xbox PDB): -1 without a process,
/// otherwise asks the current process for its type.
pub fn mobile_object_get_current_process_type(e: &mut Engine, this: Ptr<MobileObject>) -> i32 {
    let process = e.get(this, MobileObject::pCurrentProcess);
    if process.is_null() {
        -1
    } else {
        e.call(0x0045_cd60, &args![process]).i32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapped(e: &mut Engine) -> Ptr<MobileObject> {
        e.map(0x1000, 0x2000);
        Ptr::new(0x1000)
    }

    fn stub(e: &mut Engine, address: u32, result: u32) {
        e.register_double(address, move |_, _| result.into_ret());
    }

    fn virtual_slot(e: &mut Engine, object: u32, vtable: u32, slot: u32, address: u32) {
        e.map(vtable, 0x1000);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + slot, address);
        stub(e, address, 0);
    }

    fn move_fixture(e: &mut Engine) -> (Ptr<MobileObject>, Ptr) {
        e.map(0x1000, 0x6000);
        e.map(0x0102_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_f64(0x0102_31b0, f32::MAX as f64);
        e.mem.set_f32(0x4000, 10.0);
        e.mem.set_f32(0x4004, 20.0);
        e.mem.set_f32(0x4008, 30.0);
        e.mem.set_f32(0x4200, 1.0);
        e.mem.set_f32(0x4204, 2.0);
        e.mem.set_f32(0x4208, 3.0);
        e.mem.set_f32(0x4400, 100.0);
        virtual_slot(e, 0x1000, 0x7000, 0x100, 0x8000);
        for (slot, address) in [(0x224, 0x8010), (0x214, 0x8020), (0x1f4, 0x8030)] {
            e.mem.set_u32(0x7000 + slot, address);
        }
        e.mem.set_u32(0x7000 + 0x2bc, 0x8040);
        for (address, value) in [(0x8000, 0), (0x8010, 0), (0x8020, 0), (0x8030, 0x4000)] {
            stub(e, address, value);
        }
        e.register_double(0x8040, |_, _| 0.0f32.into_ret());
        for (address, result) in [
            (0x0043_6aa0, 0x4000),
            (0x0043_0830, 0x4100),
            (0x00ec_75b1, 0),
            (0x00ec_7595, 1),
            (0x0043_d4d0, 0x4300),
            (0x0052_4c90, 0x4600),
            (0x0045_bb80, 0x4500),
            (0x0040_3e20, 0x4400),
        ] {
            stub(e, address, result);
        }
        e.register_double(0x0043_9e90, |e, args| {
            for component in 0..3 {
                let left = e.mem.f32(args[0] + 4 * component);
                let right = e.mem.f32(args[2] + 4 * component);
                e.mem
                    .set_f32(args[1] + 4 * component, (left as f64 + right as f64) as f32);
            }
            args[1].into_ret()
        });
        e.register_double(0x0043_9ef0, |e, args| {
            for component in 0..3 {
                let left = e.mem.f32(args[0] + 4 * component);
                let right = e.mem.f32(args[2] + 4 * component);
                e.mem
                    .set_f32(args[1] + 4 * component, (left as f64 - right as f64) as f32);
            }
            args[1].into_ret()
        });
        e.register_double(0x004a_7290, |_, _| 0.0f32.into_ret());
        e.register_double(0x0068_15c0, |_, args| args[0].into_ret());
        stub(e, 0x004a_0c90, 0);
        e.register_double(0x004b_4500, |e, args| {
            for component in 0..3 {
                let word = e.mem.u32(args[2] + 4 * component);
                e.mem.set_u32(args[1] + 4 * component, word);
            }
            args[1].into_ret()
        });
        e.register_double(0x0063_c8a0, |e, args| {
            for component in 0..3 {
                let left = e.mem.f32(args[0] + 4 * component);
                let right = e.mem.f32(args[1] + 4 * component);
                e.mem.set_f32(args[0] + 4 * component, left + right);
            }
            args[0].into_ret()
        });
        e.register_double(0x0057_5830, |e, args| {
            for component in 0..3 {
                let word = e.mem.u32(args[1] + 4 * component);
                e.mem.set_u32(0x3000 + 4 * component, word);
            }
            0u32.into_ret()
        });
        (Ptr::new(0x1000), Ptr::new(0x4200))
    }

    #[test]
    fn constructor_initializes_mobile_members() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        stub(&mut e, 0x0055_a2f0, 0);
        assert_eq!(mobile_object_ctor(&mut e, this), this);
        assert_eq!(e.mem.u32(0x1000), 0x0108_a49c);
        assert_eq!(e.mem.u32(0x1018), 0x0108_a490);
        assert_eq!(e.mem.u32(0x1068), 0);
        assert_eq!(e.mem.u32(0x1070), 0);
        assert_eq!(e.mem.u32(0x1078), 0);
        assert_eq!(e.mem.u8(0x107c), 1);
        assert_eq!(e.mem.u8(0x1084), 0xff);
        assert_eq!(e.mem.u8(0x1087), 0);
    }

    #[test]
    fn deleting_destructor_honors_low_flag_bit() {
        for flags in [0, 1, 2] {
            let mut e = Engine::new();
            stub(&mut e, 0x0092_ec40, 0);
            stub(&mut e, 0x0040_1030, 0);
            e.call_log = Some(vec![]);
            let this = Ptr::new(0x1000);
            assert_eq!(fn_0092ec10(&mut e, this, flags), this);
            let log = e.call_log.unwrap();
            assert_eq!(log.iter().any(|call| call.0 == 0x0040_1030), flags & 1 != 0);
        }
    }

    #[test]
    fn destructor_removes_list_membership_then_destroys_process_and_base() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(0x011c_3f2c, 0x4100);
        e.mem.set_u32(0x1068, 0x2000);
        e.mem.set_u8(0x1081, 1);
        e.mem.set_u32(0x1040, 0xaabb_ccdd);
        virtual_slot(&mut e, 0x2000, 0x3000, 0, 0x6000);
        for address in [
            0x0040_fbf0,
            0x0096_f400,
            0x0057_6760,
            0x0096_d470,
            0x0040_fba0,
            0x0055_a430,
        ] {
            stub(&mut e, address, 0);
        }
        stub(&mut e, 0x0040_77c0, 0);
        stub(&mut e, 0x0042_26e0, 0);
        stub(&mut e, 0x0045_cd60, 3);
        stub(&mut e, 0x6000, 0x715);
        e.call_log = Some(vec![]);
        mobile_object_dtor(&mut e, this);
        assert_eq!(e.mem.u32(0x1000), 0x0108_a49c);
        assert_eq!(e.mem.u32(0x1018), 0x0108_a490);
        assert_eq!(e.mem.u32(0x1040), 0);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            [
                0x0040_fbf0,
                0x0040_77c0,
                0x0096_f400,
                0x0042_26e0,
                0x0057_6760,
                0x0045_cd60,
                0x0096_d470,
                0x6000,
                0x0040_fba0,
                0x0055_a430,
            ]
        );
        assert_eq!(log[6].1, [0x011e_0e80, 0x1000, 3]);
        assert_eq!(log[7].1, [0x2000, 1]);
        assert_eq!(log[8].1, [0x011f_11a0]);
    }

    #[test]
    fn destructor_guard_skips_lists_and_handles_null_process() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        for (address, result) in [
            (0x0040_fbf0, 0),
            (0x0040_77c0, 1),
            (0x0040_fba0, 0),
            (0x0055_a430, 0),
        ] {
            stub(&mut e, address, result);
        }
        e.call_log = Some(vec![]);
        mobile_object_dtor(&mut e, this);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            [0x0040_fbf0, 0x0040_77c0, 0x0040_fba0, 0x0055_a430]
        );
        assert_eq!(log[2].1, [0x011f_11a0]);
    }

    #[test]
    fn cleanup_first_list_clears_matching_saved_acquire() {
        let mut e = Engine::new();
        e.map(0x1000, 0x5000);
        e.map(0x7000, 0x3000);
        e.map(0x011d_e000, 0x1000);
        let this = Ptr::new(0x1000);
        virtual_slot(&mut e, 0x2000, 0x7000, 0x100, 0xa000);
        virtual_slot(&mut e, 0x3000, 0x8000, 0x664, 0xa010);
        virtual_slot(&mut e, 0x1000, 0x9000, 0x198, 0xa020);
        e.mem.set_u32(0x9000 + 0x19c, 0xa030);
        e.mem.set_u32(0x8000 + 0x3d0, 0xa040);
        e.mem.set_u32(0x8000 + 0x3d4, 0xa050);
        stub(&mut e, 0xa000, 1);
        stub(&mut e, 0xa020, 0x1234);
        stub(&mut e, 0xa030, 0x5678);
        stub(&mut e, 0xa040, 0x5678);
        for address in [0xa050, 0x008c_4f10] {
            stub(&mut e, address, 0);
        }
        stub(&mut e, 0x0071_7e50, 0x5000);
        stub(&mut e, 0x005b_e5c0, 1);
        stub(&mut e, 0x005d_e0f0, 1);
        stub(&mut e, 0x0096_8670, 0x2000);
        stub(&mut e, 0x008d_8520, 0x3000);
        e.call_log = Some(vec![]);
        mobile_object_clean_up_other_mobile_objects(&mut e, this, 1);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0xa010 && call.1 == [0x3000, 0x1000]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x008c_4f10 && call.1 == [0x2000, 0x1234]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0xa050 && call.1 == [0x3000, 0]));
        assert_eq!(log.iter().filter(|call| call.0 == 0x008d_8520).count(), 3);
    }

    #[test]
    fn cleanup_first_list_prefers_package_callbacks_before_dynamic_cast() {
        for (slot, callback) in [(0x224, 0x009c_4c80), (0x220, 0x009b_28a0)] {
            let mut e = Engine::new();
            e.map(0x1000, 0x4000);
            e.map(0x7000, 0x1000);
            e.map(0x011d_e000, 0x1000);
            virtual_slot(&mut e, 0x2000, 0x7000, 0x100, 0xa000);
            e.mem.set_u32(0x7000 + 0x224, 0xa010);
            e.mem.set_u32(0x7000 + 0x220, 0xa020);
            stub(&mut e, 0xa000, 0);
            stub(&mut e, 0xa010, u32::from(slot == 0x224));
            stub(&mut e, 0xa020, 1);
            stub(&mut e, callback, 0);
            stub(&mut e, 0x0071_7e50, 0x5000);
            stub(&mut e, 0x005b_e5c0, 1);
            stub(&mut e, 0x005d_e0f0, 1);
            stub(&mut e, 0x0096_8670, 0x2000);
            e.call_log = Some(vec![]);
            mobile_object_clean_up_other_mobile_objects(&mut e, Ptr::new(0x1000), 1);
            let log = e.call_log.unwrap();
            assert!(log
                .iter()
                .any(|call| call.0 == callback && call.1 == [0x2000, 0x1000]));
            assert!(!log.iter().any(|call| call.0 == 0x00ec_43fb));
        }
    }

    #[test]
    fn cleanup_second_list_uses_cast_target_and_dialogue_finalizer() {
        let mut e = Engine::new();
        e.map(0x1000, 0x5000);
        e.map(0x7000, 0x3000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x011d_ea3c, 0x4000);
        e.mem.set_u32(0x4000 + 0xd2c, 0x1000);
        virtual_slot(&mut e, 0x1000, 0x7000, 0x198, 0xa000);
        virtual_slot(&mut e, 0x3000, 0x8000, 0x314, 0xa010);
        stub(&mut e, 0xa000, 0x1234);
        stub(&mut e, 0x0071_7e50, 0x5000);
        e.register_double(0x005b_e5c0, |_, args| u32::from(args[1] == 1).into_ret());
        stub(&mut e, 0x005d_e0f0, 0);
        stub(&mut e, 0x0096_8670, 0x2000);
        e.register_double(0x00ec_43fb, |_, args| {
            if args[3] == 0x011a_0e88 {
                0x3000u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        stub(&mut e, 0x008d_8520, 0);
        stub(&mut e, 0x008c_4f10, 0);
        stub(&mut e, 0x0052_4d70, 0);
        e.call_log = Some(vec![]);
        mobile_object_clean_up_other_mobile_objects(&mut e, Ptr::new(0x1000), 1);
        let log = e.call_log.unwrap();
        assert!(log.iter().any(|call| {
            call.0 == 0x00ec_43fb && call.1 == [0x2000, 0, 0x0118_4920, 0x0118_46d4, 0]
        }));
        assert!(log
            .iter()
            .any(|call| call.0 == 0xa010 && call.1 == [0x3000, 0x1234]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x008c_4f10 && call.1 == [0x4000, 0x1234]));
        assert_eq!(log.last().unwrap().0, 0x0052_4d70);
        assert_eq!(log.last().unwrap().1, [0x4000]);
    }

    #[test]
    fn cleanup_dialogue_reloads_global_after_mutating_callbacks() {
        let mut e = Engine::new();
        e.map(0x1000, 0x5000);
        e.map(0x7000, 0x4000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x011d_ea3c, 0x4000);
        e.mem.set_u32(0x4400 + 0xd2c, 0x1000);
        virtual_slot(&mut e, 0x1000, 0x7000, 0x198, 0xb000);
        e.mem.set_u32(0x7000 + 0x19c, 0xb010);
        virtual_slot(&mut e, 0x3200, 0x8000, 0x664, 0xb020);
        virtual_slot(&mut e, 0x3400, 0x9000, 0x3d0, 0xb030);
        virtual_slot(&mut e, 0x3500, 0xa000, 0x3d4, 0xb040);
        e.register_double(0xb000, |e, _| {
            e.mem.set_u32(0x011d_ea3c, 0x4200);
            0x1234u32.into_ret()
        });
        e.register_double(0xb010, |e, _| {
            e.mem.set_u32(0x011d_ea3c, 0x4400);
            0x5678u32.into_ret()
        });
        stub(&mut e, 0xb030, 0x5678);
        stub(&mut e, 0x0071_7e50, 0x5000);
        stub(&mut e, 0x005b_e5c0, 0);
        stub(&mut e, 0x005d_e0f0, 0);
        e.register_double(0x008d_8520, |e, args| {
            match args[0] {
                0x4000 => {
                    e.mem.set_u32(0x011d_ea3c, 0x4100);
                    0x3000u32
                }
                0x4100 => 0x3200u32,
                0x4200 => {
                    e.mem.set_u32(0x011d_ea3c, 0x4300);
                    0x3300u32
                }
                0x4300 => 0x3400u32,
                0x4400 => 0x3500u32,
                _ => panic!("unexpected dialogue pointer {:08x}", args[0]),
            }
            .into_ret()
        });
        stub(&mut e, 0x008c_4f10, 0);
        stub(&mut e, 0x0052_4d70, 0);
        e.call_log = Some(vec![]);
        mobile_object_clean_up_other_mobile_objects(&mut e, Ptr::new(0x1000), 1);
        let log = e.call_log.unwrap();
        let getter_args: Vec<_> = log
            .iter()
            .filter(|call| call.0 == 0x008d_8520)
            .map(|call| call.1[0])
            .collect();
        assert_eq!(getter_args, [0x4000, 0x4100, 0x4200, 0x4300, 0x4400]);
        assert!(log
            .iter()
            .any(|call| call.0 == 0xb020 && call.1 == [0x3200, 0x1000]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x008c_4f10 && call.1 == [0x4200, 0x1234]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0xb040 && call.1 == [0x3500, 0]));
        assert_eq!(log.last().unwrap().1, [0x4400]);
    }

    #[test]
    fn move_without_controller_rotates_delta_and_commits_position() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.call_log = Some(vec![]);
        assert!(mobile_object_move(&mut e, this, 1.0, input, 0).is_null());
        assert_eq!(e.mem.f32(0x3000), 11.0);
        assert_eq!(e.mem.f32(0x3004), 22.0);
        assert_eq!(e.mem.f32(0x3008), 33.0);
        let log = e.call_log.unwrap();
        assert_eq!(log.last().unwrap().0, 0x0057_5830);
        assert_eq!(log.last().unwrap().1[0], 0x1000);
        assert_eq!(log.iter().filter(|call| call.0 == 0x0043_0830).count(), 3);
    }

    #[test]
    fn move_far_navmesh_path_returns_before_reference_commit() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.register_double(0x004a_7290, |_, _| 200.0f32.into_ret());
        stub(&mut e, 0x0057_5d70, 0x5000);
        stub(&mut e, 0x008d_6f30, 0x5100);
        e.register_double(0x006d_97b0, |e, args| {
            assert_eq!(args[1..3], [0x5100, 0x5000]);
            e.mem.set_f32(args[3], 25.0);
            1u32.into_ret()
        });
        virtual_slot(&mut e, 0x1000, 0x7000, 0x2a8, 0x8050);
        e.register_double(0x8050, |e, args| {
            for component in 0..3 {
                let word = e.mem.u32(args[1] + 4 * component);
                e.mem.set_u32(0x3000 + 4 * component, word);
            }
            0u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(mobile_object_move(&mut e, this, 1.0, input, 0).is_null());
        assert_eq!(e.mem.f32(0x3000), 11.0);
        assert_eq!(e.mem.f32(0x3004), 22.0);
        assert_eq!(e.mem.f32(0x3008), 25.0);
        let log = e.call_log.unwrap();
        assert_eq!(log.last().unwrap().0, 0x8050);
        assert!(!log.iter().any(|call| call.0 == 0x0057_5830));
    }

    #[test]
    fn move_controller_path_commits_controller_position() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0108_a000, 0x1000);
        e.mem.set_f64(0x0108_a7a0, 1.0);
        e.mem.set_u32(0x011d_ea10, 0x6200);
        e.mem.set_f64(0x0101_db88, 0.0);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        virtual_slot(&mut e, 0x5000, 0xa000, 0xc8, 0x8200);
        e.mem.set_u32(0x7000 + 0x22c, 0x8110);
        stub(&mut e, 0x8110, 0);
        for address in [0x0062_1240, 0x0062_1270, 0x008a_5170] {
            stub(&mut e, address, 0);
        }
        e.register_double(0x005f_36f0, |e, _| {
            e.mem.set_u32(0x011d_ea10, 0x6300);
            0u32.into_ret()
        });
        e.register_double(0x0045_72e0, |e, args| {
            assert_eq!(args[0], 0x6300);
            e.mem.set_f32(args[2], 34.0);
            0u32.into_ret()
        });
        e.register_double(0x0043_9ef0, |e, args| {
            assert_ne!(args[0], args[1]);
            if args[0] != 0x4000 {
                assert_eq!(e.mem.f32(args[0]), 12.0);
                assert_eq!(e.mem.f32(args[2]), 10.0);
            }
            for component in 0..3 {
                let value = e.mem.f32(args[0] + 4 * component) - e.mem.f32(args[2] + 4 * component);
                e.mem.set_f32(args[1] + 4 * component, value);
            }
            args[1].into_ret()
        });
        e.register_double(0x0081_2b00, |e, args| {
            for (component, value) in [12.0, 23.0, 34.0].into_iter().enumerate() {
                e.mem.set_f32(args[1] + 4 * component as u32, value);
            }
            args[1].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            mobile_object_move(&mut e, this, 1.0, input, 0).addr(),
            0x5000
        );
        assert_eq!(e.mem.f32(0x3000), 12.0);
        assert_eq!(e.mem.f32(0x3004), 23.0);
        assert_eq!(e.mem.f32(0x3008), 34.0);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8200 && call.1[0] == 0x5000));
        assert_eq!(log.last().unwrap().0, 0x0057_5830);
    }

    #[test]
    fn move_controller_position_delta_uses_original_vector_as_receiver() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        virtual_slot(&mut e, 0x5000, 0xa000, 0xc8, 0x8200);
        e.mem.set_u32(0x7000 + 0x22c, 0x8110);
        stub(&mut e, 0x8110, 0);
        stub(&mut e, 0x0062_1240, 1);
        stub(&mut e, 0x0062_1270, 0);
        e.register_double(0x0081_2b00, |e, args| {
            for (component, value) in [12.0, 23.0, 34.0].into_iter().enumerate() {
                e.mem.set_f32(args[1] + 4 * component as u32, value);
            }
            args[1].into_ret()
        });
        e.register_double(0x0043_9ef0, |e, args| {
            if args[0] != 0x4000 {
                assert_eq!(e.mem.f32(args[0]), 11.0);
                assert_eq!(e.mem.f32(args[2]), 12.0);
            }
            for component in 0..3 {
                let value = e.mem.f32(args[0] + 4 * component) - e.mem.f32(args[2] + 4 * component);
                e.mem.set_f32(args[1] + 4 * component, value);
            }
            args[1].into_ret()
        });
        e.register_double(0x004b_45b0, |_, args| args[0].into_ret());
        assert_eq!(
            mobile_object_move(&mut e, this, 1.0, input, 0).addr(),
            0x5000
        );
        assert_eq!(e.mem.f32(input.addr()), -1.0);
        assert_eq!(e.mem.f32(input.addr() + 4), -1.0);
        assert_eq!(e.mem.f32(input.addr() + 8), -1.0);
    }

    #[test]
    fn move_snapshots_reach_before_global_lookup_callback() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0103_0000, 0x1000);
        e.mem.set_f32(0x0103_0ff0, 2.5);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        virtual_slot(&mut e, 0x5000, 0xa000, 0xc8, 0x8200);
        stub(&mut e, 0x8000, 1);
        e.mem.set_u32(0x10a4, 0xb000);
        e.map(0xb000, 0x1000);
        e.mem.set_u32(0xb008, 0x8300);
        stub(&mut e, 0x8300, 0);
        e.mem.set_u32(0x7000 + 0x4b4, 0x8310);
        e.mem.set_u32(0x7000 + 0x1d0, 0x8320);
        e.mem.set_u32(0x7000 + 0x22c, 0x8330);
        stub(&mut e, 0x8310, 0);
        stub(&mut e, 0x8320, 0x6000);
        stub(&mut e, 0x8330, 0);
        virtual_slot(&mut e, 0x6000, 0xc000, 0x10, 0x8340);
        stub(&mut e, 0x8340, 0x6100);
        stub(&mut e, 0x0043_7bd0, 1);
        let mut camera_lookups = 0;
        e.register_double(0x0052_4c90, move |e, _| {
            camera_lookups += 1;
            if camera_lookups == 2 {
                e.mem.set_f32(0x0103_0ff0, 4.5);
            }
            0x4600u32.into_ret()
        });
        e.register_double(0x00b4_e030, |_, args| {
            assert_eq!(args, [0x6100, 0x4600, 2.5f32.to_bits()]);
            1u32.into_ret()
        });
        stub(&mut e, 0x0062_1240, 1);
        stub(&mut e, 0x0062_1270, 0);
        stub(&mut e, 0x008a_5170, 0);
        stub(&mut e, 0x0088_46e0, 0);
        stub(&mut e, 0x008b_8f90, 0);
        e.register_double(0x0081_2b00, |_, args| args[1].into_ret());
        e.register_double(0x004b_45b0, |_, args| args[0].into_ret());
        assert_eq!(
            mobile_object_move(&mut e, this, 1.0, input, 0).addr(),
            0x5000
        );
        assert_eq!(e.mem.f32(0x0103_0ff0), 4.5);
    }

    #[test]
    fn move_periodic_water_path_updates_animation_and_returns_early() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011a_2000, 0x1000);
        e.map(0x011e_0000, 0x1000);
        e.mem.set_u32(0x011a_2fe0, 7);
        e.mem.set_u32(0x011e_054c, 7);
        e.mem.set_u8(0x011e_0550, 1);
        e.mem.set_u32(0x4300, 1);
        e.mem.set_f32(0x5100, 40.0);
        e.mem.set_f32(0x5104, 50.0);
        e.mem.set_f32(0x5108, 60.0);
        stub(&mut e, 0x0088_b0c0, 1);
        virtual_slot(&mut e, 0x1000, 0x7000, 0x1d0, 0x8060);
        stub(&mut e, 0x8060, 0x5000);
        stub(&mut e, 0x0043_c490, 0x5100);
        e.register_double(0x0043_9e90, |e, args| {
            assert_eq!(args[0], 0x5100);
            for component in 0..3 {
                let left = e.mem.f32(args[0] + 4 * component);
                let right = e.mem.f32(args[2] + 4 * component);
                e.mem.set_f32(args[1] + 4 * component, left + right);
            }
            args[1].into_ret()
        });
        e.register_double(0x0044_0460, |e, args| {
            assert_eq!(e.mem.f32(args[1]), 41.0);
            assert_eq!(e.mem.f32(args[1] + 4), 52.0);
            assert_eq!(e.mem.f32(args[1] + 8), 63.0);
            0u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(mobile_object_move(&mut e, this, 1.0, input, 0).is_null());
        let log = e.call_log.unwrap();
        assert_eq!(log.last().unwrap().0, 0x0044_0460);
        assert!(!log.iter().any(|call| call.0 == 0x0057_5830));
        assert_eq!(log.last().unwrap().1[0], 0x5000);
    }

    #[test]
    fn move_uses_original_vectors_as_receivers_for_world_and_camera_subtraction() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011a_2000, 0x1000);
        e.map(0x011e_0000, 0x1000);
        e.mem.set_u32(0x011a_2fe0, 7);
        e.mem.set_u32(0x011e_054c, 7);
        e.mem.set_u32(0x4300, 1);
        for (component, value) in [40.0, 50.0, 60.0].into_iter().enumerate() {
            e.mem.set_f32(0x5100 + 4 * component as u32, value);
        }
        e.mem.set_u32(0x7000 + 0x1d0, 0x8060);
        stub(&mut e, 0x8060, 0x5000);
        stub(&mut e, 0x0043_c490, 0x5100);
        e.register_double(0x004b_45b0, |_, args| args[0].into_ret());
        e.register_double(0x004a_7290, |e, args| {
            assert_eq!(e.mem.f32(args[0]), 10.0);
            assert_eq!(e.mem.f32(args[0] + 4), 20.0);
            assert_eq!(e.mem.f32(args[0] + 8), 30.0);
            0.0f32.into_ret()
        });
        e.call_log = Some(vec![]);
        mobile_object_move(&mut e, this, 1.0, input, 0);
        let log = e.call_log.unwrap();
        let vector_calls = log
            .iter()
            .filter(|call| call.0 == 0x0043_9ef0)
            .collect::<Vec<_>>();
        assert_eq!(vector_calls.len(), 2);
        assert_eq!(vector_calls[0].1[0], 0x5100);
        assert_eq!(vector_calls[0].1[2], 0x4000);
        assert_eq!(vector_calls[1].1[0], 0x4000);
        assert_eq!(vector_calls[1].1[2], 0x4500);
        assert_ne!(vector_calls[0].1[0], vector_calls[0].1[1]);
        assert_ne!(vector_calls[1].1[0], vector_calls[1].1[1]);
    }

    #[test]
    fn move_mode_without_controller_sends_immobile_request_then_commits() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011f_4000, 0x1000);
        stub(&mut e, 0x8020, 1);
        e.mem.set_u32(0x7000 + 0x22c, 0x8110);
        stub(&mut e, 0x8110, 0);
        stub(&mut e, 0x00c7_0b60, 0);
        e.call_log = Some(vec![]);
        assert!(mobile_object_move(&mut e, this, 1.5, input, 0x45).is_null());
        let log = e.call_log.unwrap();
        let request = log.iter().find(|call| call.0 == 0x00c7_0b60).unwrap();
        assert_eq!(request.1[0], 0);
        assert_eq!(request.1[2], 0);
        assert_eq!(e.mem.f32(0x3000), 11.0);
        assert_eq!(e.mem.f32(0x3004), 22.0);
        assert_eq!(e.mem.f32(0x3008), 33.0);
    }

    #[test]
    fn move_snapshots_movement_fraction_before_callbacks() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x0101_7000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(0x0101_7d00, 2.5);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        stub(&mut e, 0x8020, 1);
        e.mem.set_u32(0x7000 + 0x22c, 0x8110);
        stub(&mut e, 0x8110, 0);
        e.register_double(0x0045_8b20, |e, _| {
            e.mem.set_f32(0x0101_7d00, 3.5);
            0x6200u32.into_ret()
        });
        e.register_double(0x0089_d600, |e, _| {
            e.mem.set_f32(0x0101_7d00, 4.5);
            0x6500u32.into_ret()
        });
        stub(&mut e, 0x0056_1730, 0);
        stub(&mut e, 0x00c7_0b60, 0);
        e.call_log = Some(vec![]);
        mobile_object_move(&mut e, this, 1.0, input, 0);
        let log = e.call_log.unwrap();
        let setting_call = log.iter().find(|call| call.0 == 0x0056_1730).unwrap();
        assert_eq!(setting_call.1, [0x6500, 0x6200, 2.5f32.to_bits()]);
        assert_eq!(e.mem.f32(0x0101_7d00), 4.5);
    }

    #[test]
    fn move_keeps_delta_divisor_at_x87_precision_until_float_store() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x011f_4000, 0x1000);
        stub(&mut e, 0x8000, 1);
        e.mem.set_u32(0x10a4, 0xb000);
        e.map(0xb000, 0x1000);
        e.mem.set_u32(0xb008, 0x8300);
        stub(&mut e, 0x8300, 1);
        e.mem.set_u32(0x7000 + 0x4b4, 0x8310);
        e.mem.set_u32(0x7000 + 0x22c, 0x8320);
        stub(&mut e, 0x8310, 0);
        stub(&mut e, 0x8320, 0);
        stub(&mut e, 0x8020, 1);
        stub(&mut e, 0x0043_7bd0, 1);
        e.register_double(0x0071_6440, |_, _| 1.000_000_059_604_644_8f64.into_ret());
        e.register_double(0x00c7_0b60, |e, args| {
            let delta = e.mem.f32(args[1]);
            e.mem.set_f32(0x4500, delta);
            0u32.into_ret()
        });
        mobile_object_move(&mut e, this, 1.0, input, 0);
        assert_eq!(e.mem.f32(0x4500).to_bits(), 0x3f7f_ffff);
    }

    #[test]
    fn move_reloads_loaded_data_after_water_height_callback() {
        let mut e = Engine::new();
        let (this, input) = move_fixture(&mut e);
        e.map(0x0102_4000, 0x1000);
        e.mem.set_f64(0x0102_41b0, -(f32::MAX as f64));
        e.mem.set_u32(0x1064, 0x5200);
        e.mem.set_f32(0x5208, -f32::MAX);
        e.mem.set_f32(0x5308, -1.0);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        e.register_double(0x004a_7290, |_, _| 200.0f32.into_ret());
        stub(&mut e, 0x0057_5d70, 0x5000);
        stub(&mut e, 0x008d_6f30, 0x5100);
        e.register_double(0x006d_97b0, |e, args| {
            e.mem.set_f32(args[3], 25.0);
            1u32.into_ret()
        });
        stub(&mut e, 0x005c_0880, 5);
        e.register_double(0x0045_cbc0, |e, _| {
            e.mem.set_u32(0x1064, 0x5300);
            42.0f32.into_ret()
        });
        e.register_double(0x0057_b0a0, |_, _| 100.0f32.into_ret());
        stub(&mut e, 0x0062_1ad0, 0);
        virtual_slot(&mut e, 0x1000, 0x7000, 0x2a8, 0x8050);
        assert_eq!(
            mobile_object_move(&mut e, this, 1.0, input, 0).addr(),
            0x5000
        );
        assert_eq!(e.mem.f32(0x5208), -f32::MAX);
        assert_eq!(e.mem.f32(0x5308), 42.0);
    }

    #[test]
    fn controller_world_sync_moves_between_worlds_and_attaches_new_world() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        stub(&mut e, 0x008d_6f30, 0x3000);
        stub(&mut e, 0x0045_43c0, 0x5300);
        stub(&mut e, 0x0062_1ad0, 0x5400);
        stub(&mut e, 0x0088_b0a0, 0);
        e.register_double(0x0054_71e0, |_, _| 123.0f32.into_ret());
        stub(&mut e, 0x0057_b1d0, 0);
        stub(&mut e, 0x00c7_0c90, 0);
        stub(&mut e, 0x0056_20e0, 0);
        stub(&mut e, 0x0045_cd60, 1);
        stub(&mut e, 0x0081_0660, 0x6000);
        stub(&mut e, 0x0043_fcd0, 0x6100);
        stub(&mut e, 0x00c6_d7a0, 0);
        stub(&mut e, 0x00c6_9ee0, 0);
        stub(&mut e, 0x0070_c440, 0);
        stub(&mut e, 0x004a_3a20, 0x6200);
        stub(&mut e, 0x00c6_6300, 0);
        stub(&mut e, 0x00c6_b540, 0);
        virtual_slot(&mut e, 0x5300, 0xa000, 0xd0, 0x8200);
        e.call_log = Some(vec![]);
        fn_009308f0(&mut e, this, 0);
        let log = e.call_log.unwrap();
        let calls: Vec<_> = log.iter().map(|entry| entry.0).collect();
        assert!(calls
            .windows(2)
            .any(|pair| pair == [0x00c6_9ee0, 0x0070_c440]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0057_b1d0 && call.1 == [0x5000, 123.0f32.to_bits()]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c6_9ee0 && call.1 == [0x6100, 1, 0]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8200 && call.1 == [0x5300, 0x6100, 1, 0, 0x6200, 0]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c6_b540 && call.1 == [0x5300, 0]));
    }

    #[test]
    fn controller_world_sync_reloads_ragdoll_and_saved_acquire_objects() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f32(0x0101_e2bc, 0.1);
        e.mem.set_u32(0x1068, 0x2000);
        e.mem.set_u32(0x10ac, 0x6000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        stub(&mut e, 0x8000, 1);
        stub(&mut e, 0x004f_8960, 0);
        stub(&mut e, 0x008d_6f30, 0x3000);
        stub(&mut e, 0x0045_43c0, 0x5300);
        stub(&mut e, 0x0062_1ad0, 0x5300);
        stub(&mut e, 0x0081_0660, 0x6100);
        stub(&mut e, 0x0043_fcd0, 0x6100);
        stub(&mut e, 0x00c6_9f50, 0);
        e.register_double(0x0087_ea20, |e, _| {
            e.mem.set_u32(0x10ac, 0x6200);
            1u32.into_ret()
        });
        e.mem.set_u32(0x7000 + 0x22c, 0x8110);
        stub(&mut e, 0x8110, 0);
        stub(&mut e, 0x00c7_d810, 0);
        e.register_double(0x0087_eeb0, |e, _| {
            e.mem.set_u32(0x10ac, 0x6300);
            1u32.into_ret()
        });
        stub(&mut e, 0x00c7_c150, 0);
        stub(&mut e, 0x8020, 1);
        let mut acquisition = 0;
        e.register_double(0x008d_8520, move |_, _| {
            acquisition += 1;
            [0x6400u32, 0x6500, 0x6600, 0x6700][acquisition - 1].into_ret()
        });
        virtual_slot(&mut e, 0x6500, 0xb000, 0x4d4, 0x8400);
        virtual_slot(&mut e, 0x6600, 0xc000, 0x4c8, 0x8410);
        virtual_slot(&mut e, 0x6700, 0xd000, 0x2b4, 0x8420);
        stub(&mut e, 0x8400, 0x6800);
        stub(&mut e, 0x0050_94b0, 0x6900);
        stub(&mut e, 0x8410, 0x6a00);
        stub(&mut e, 0x8420, 1);
        e.mem.set_u32(0x7000 + 0x2e0, 0x8430);
        stub(&mut e, 0x8430, 0);
        stub(&mut e, 0x0093_1fb0, 0);
        stub(&mut e, 0x0062_9670, 0);
        e.call_log = Some(vec![]);
        fn_009308f0(&mut e, this, 0);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c7_d810 && call.1 == [0x6200]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c7_c150 && call.1 == [0x6300, 1]));
        let getter_count = log.iter().filter(|call| call.0 == 0x008d_8520).count();
        assert_eq!(getter_count, 4);
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8400 && call.1 == [0x6500]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8410 && call.1 == [0x6600, 0x6900]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8420 && call.1 == [0x6700, 0x6a00]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8430 && call.1 == [0x1000, 0.1f32.to_bits()]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0062_9670 && call.1 == [0x5410, 0x0800_0000, 1]));
    }

    #[test]
    fn init_havok_removes_water_before_process_guard() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        stub(&mut e, 0x0057_b520, 0);
        e.call_log = Some(vec![]);
        mobile_object_init_havok(&mut e, this);
        let log = e.call_log.unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, 0x0057_b520);
        assert_eq!(log[0].1, [0x1000, 0]);
    }

    #[test]
    fn init_havok_existing_controller_keeps_attached_geometry() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0x5000);
        stub(&mut e, 0x0057_b520, 0);
        stub(&mut e, 0x0045_cd60, 0);
        stub(&mut e, 0x008d_6f30, 0);
        stub(&mut e, 0x005d_43c0, 0x4000);
        stub(&mut e, 0x0042_e8c0, 0);
        stub(&mut e, 0x0081_0660, 0x6100);
        stub(&mut e, 0x0043_fcd0, 0x6100);
        e.mem.set_u32(0x7000 + 0x1fc, 0x8110);
        stub(&mut e, 0x8110, 0);
        e.call_log = Some(vec![]);
        mobile_object_init_havok(&mut e, this);
        let log = e.call_log.unwrap();
        assert_eq!(log.first().unwrap().1, [0x1000, 0]);
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8110 && call.1 == [0x1000, 0]));
        assert!(!log.iter().any(|call| call.0 == 0x00c6_da50));
        assert!(!log.iter().any(|call| call.0 == 0x00c6_d7a0));
    }

    #[test]
    fn init_havok_special_actor_uses_ragdoll_pointer_and_receiver_only_getter() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(0x1068, 0x2000);
        e.mem.set_u32(0x10b0, 0x4500);
        e.mem.set_u32(0x7000 + 0x1d0, 0x8110);
        e.mem.set_u32(0x7000 + 0x1fc, 0x8120);
        e.mem.set_u32(0x7000 + 0x22c, 0x8130);
        stub(&mut e, 0x8000, 1);
        stub(&mut e, 0x8110, 0x123);
        stub(&mut e, 0x8120, 0);
        stub(&mut e, 0x8130, 1);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8140);
        stub(&mut e, 0x8140, 0x5000);
        virtual_slot(&mut e, 0x5000, 0xa000, 0xd0, 0x8150);
        for (address, result) in [
            (0x0057_b520, 0),
            (0x0045_cd60, 0),
            (0x008d_6f30, 0x4000),
            (0x0045_0ff0, 1),
            (0x0045_43c0, 0x4300),
            (0x005a_29b0, 0),
            (0x004f_8960, 2),
            (0x0043_fcd0, 0x6100),
            (0x00c6_c210, 0),
            (0x005d_43c0, 0x4200),
            (0x0042_e8c0, 0),
            (0x0093_1ed0, 0x4400),
            (0x004a_3a20, 7),
            (0x0081_0660, 0x6000),
            (0x00ca_2ad0, 0),
            (0x00c6_d7a0, 0),
            (0x0041_4430, 0),
            (0x004b_ae00, 1),
            (0x8150, 0),
            (0x00b6_b910, 0),
            (0x00c6_a350, 0),
            (0x0041_81e0, 0),
            (0x0042_2b90, 1),
        ] {
            stub(&mut e, address, result);
        }
        e.call_log = Some(vec![]);
        mobile_object_init_havok(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00ca_2ad0 && call.1 == [0x4500, 0x123, 7]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0081_0660 && call.1 == [0x5000]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c6_a350 && call.1 == [0x6100, 1, 1, 0, 0]));
    }

    #[test]
    fn init_havok_constructs_controller_and_passes_owned_handle_to_process() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        stub(&mut e, 0x8100, 0);
        e.mem.set_u32(0x9000 + 0x290, 0x8110);
        stub(&mut e, 0x8110, 0);
        e.mem.set_u32(0x7000 + 0x2e4, 0x8120);
        e.register_double(0x8120, |_, _| 0.75f32.into_ret());
        e.mem.set_u32(0x7000 + 0x1fc, 0x8130);
        stub(&mut e, 0x8130, 0);
        for address in [
            0x0057_b520,
            0x0045_cd60,
            0x008d_6f30,
            0x0042_e8c0,
            0x00c6_da50,
            0x008a_c890,
            0x0081_0510,
            0x0085_7230,
            0x0081_0550,
            0x0081_0640,
            0x0081_0610,
            0x0062_9670,
            0x0055_9450,
            0x00c6_de10,
        ] {
            stub(&mut e, address, 0);
        }
        stub(&mut e, 0x005d_43c0, 0x4000);
        stub(&mut e, 0x0043_fcd0, 0x6100);
        stub(&mut e, 0x0055_3300, 0x5500);
        stub(&mut e, 0x00c7_41e0, 0x5000);
        e.register_double(0x0063_3c90, |e, args| {
            assert_eq!(args[1], 0x5000);
            e.mem.set_u32(args[0], 0x1234);
            args[0].into_ret()
        });
        e.register_double(0x0056_7400, |_, _| 1.5f32.into_ret());
        stub(&mut e, 0x0052_5420, 1);
        stub(&mut e, 0x0081_0660, 0x6100);
        e.call_log = Some(vec![]);
        mobile_object_init_havok(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8110 && call.1 == [0x2000, 0x1234]));
        assert_eq!(e.mem.f32(0x5000 + 0x55c), 1.5);
        let constructor = log.iter().position(|call| call.0 == 0x00c6_da50).unwrap();
        let destructor = log.iter().position(|call| call.0 == 0x00c6_de10).unwrap();
        assert!(constructor < destructor);
    }

    #[test]
    fn init_havok_disables_each_unmatched_dismembered_rigid_body() {
        let mut e = Engine::new();
        let (this, _) = move_fixture(&mut e);
        e.map(0x0119_6000, 0x1000);
        e.mem.set_u32(0x0119_6d74, 0x4300);
        e.mem.set_u8(0x4200, 2);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x9000, 0x28c, 0x8100);
        e.mem.set_u32(0x9000 + 0x6d0, 0x8110);
        stub(&mut e, 0x8100, 0x5000);
        stub(&mut e, 0x8110, 0x4400);
        e.mem.set_u32(0x7000 + 0x1d0, 0x8120);
        e.mem.set_u32(0x7000 + 0x1fc, 0x8130);
        stub(&mut e, 0x8120, 1);
        stub(&mut e, 0x8130, 0);
        stub(&mut e, 0x0057_b520, 0);
        stub(&mut e, 0x0045_cd60, 0);
        stub(&mut e, 0x008d_6f30, 0);
        stub(&mut e, 0x005d_43c0, 0x4000);
        stub(&mut e, 0x0042_e8c0, 0x4100);
        stub(&mut e, 0x008b_6800, 1);
        stub(&mut e, 0x0044_1420, 0x4200);
        stub(&mut e, 0x0096_11e0, 0);
        stub(&mut e, 0x0043_8170, 0);
        stub(&mut e, 0x0043_81b0, 0);
        stub(&mut e, 0x0043_b480, 1);
        stub(&mut e, 0x0043_b4a0, 0x4500);
        stub(&mut e, 0x00c6_a200, 0);
        stub(&mut e, 0x0081_0660, 0x6100);
        stub(&mut e, 0x0043_fcd0, 0x6100);
        e.call_log = Some(vec![]);
        mobile_object_init_havok(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x8110 && call.1 == [0x2000, 2]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0043_8170 && call.1[1] == 0x4300));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x00c6_a200 && call.1 == [0x4500, 1, 1, 1]));
        assert_eq!(log.iter().filter(|call| call.0 == 0x0043_81b0).count(), 1);
    }

    #[test]
    fn distant_word_accessor() {
        let mut e = Engine::new();
        e.map(0x1000, 0x2000);
        e.mem.set_u32(0x1d2c, 0x3412);
        assert_eq!(fn_0092f140(&mut e, Ptr::new(0x1000)).addr(), 0x3412);
    }

    #[test]
    fn talking_forwards_this_to_process_slot() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x4b8, 0x6000);
        stub(&mut e, 0x6000, 1);
        e.call_log = Some(vec![]);
        assert!(mobile_object_is_talking(&mut e, this));
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000, 0x1000]);
    }

    #[test]
    fn process_slot_28_requires_process() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        fn_0092f190(&mut e, this);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x28, 0x6000);
        e.call_log = Some(vec![]);
        fn_0092f190(&mut e, this);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000]);
    }

    #[test]
    fn cleanup_callback_stops_when_reference_is_ineligible() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x140, 0x6000);
        stub(&mut e, 0x0057_4900, 0);
        e.call_log = Some(vec![]);
        fn_0092f1c0(&mut e, this, 0xdead);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn cleanup_callback_runs_eligible_path_in_order() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x140, 0x6000);
        for (address, result) in [
            (0x0057_4900, 1),
            (0x0093_36c0, 0),
            (0x005e_3fa0, 0x4000),
            (0x005d_43c0, 0x5000),
            (0x0042_e130, 0),
            (0x0096_d470, 0),
            (0x0096_e870, 0),
        ] {
            stub(&mut e, address, result);
        }
        e.call_log = Some(vec![]);
        fn_0092f1c0(&mut e, this, 0);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            vec![
                0x6000,
                0x0057_4900,
                0x0093_36c0,
                0x005e_3fa0,
                0x005d_43c0,
                0x0042_e130,
                0x0096_d470,
                0x0096_e870
            ]
        );
        assert_eq!(log[5].1, vec![0x5000]);
        assert_eq!(log[6].1, vec![0x011e_0e80, 0x1000, 0]);
        assert_eq!(log[7].1, vec![0x011e_0e80, 0x1000]);
    }

    #[test]
    fn global_pointer_accessor() {
        let mut e = Engine::new();
        e.map(0x011a_2000, 0x2000);
        e.mem.set_u32(0x011a_2fe0, 0x4321);
        assert_eq!(fn_009304a0(&mut e).addr(), 0x4321);
    }

    macro_rules! flag_test {
        ($name:ident, $function:ident, $offset:expr, $mask:expr) => {
            #[test]
            fn $name() {
                let mut e = Engine::new();
                let this: Ptr = Ptr::new(0x1000);
                e.register_double(0x0062_1270, |_, args| (args[1] == $mask).into_ret());
                e.call_log = Some(vec![]);
                assert!($function(&mut e, this));
                assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000 + $offset, $mask]);
            }
        };
    }

    flag_test!(test_bit_20, fn_009304b0, 0, 0x20);
    flag_test!(test_bit_40000, fn_00930570, 0x410, 0x40000);
    flag_test!(test_bit_2000000, fn_009305a0, 0x410, 0x2000000);
    flag_test!(test_bit_4000000, fn_009305d0, 0x410, 0x4000000);

    #[test]
    fn movement_temporary_copies_three_global_words_twice() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for index in 0..3 {
            e.mem.set_u32(0x011f_426c + index * 4, index + 7);
        }
        e.map(0x1000, 0x1000);
        let this = Ptr::new(0x1000);
        assert_eq!(fn_009304d0(&mut e, this), this);
        for index in 0..3 {
            assert_eq!(e.mem.u32(0x1004 + index * 4), index + 7);
            assert_eq!(e.mem.u32(0x1010 + index * 4), index + 7);
        }
        assert_eq!(e.mem.u8(0x1024), 1);
    }

    macro_rules! flag_set_test {
        ($name:ident, $function:ident, $mask:expr) => {
            #[test]
            fn $name() {
                let mut e = Engine::new();
                stub(&mut e, 0x0062_9670, 0);
                e.call_log = Some(vec![]);
                $function(&mut e, Ptr::new(0x1000), 1);
                assert_eq!(e.call_log.unwrap()[0].1, vec![0x1410, $mask, 1]);
            }
        };
    }

    flag_set_test!(set_bit_100, fn_00930540, 0x100);
    flag_set_test!(set_bit_400000, fn_00931530, 0x400000);

    macro_rules! store_float_test {
        ($name:ident, $function:ident, $offset:expr) => {
            #[test]
            fn $name() {
                let mut e = Engine::new();
                e.map(0x1000, 0x1000);
                $function(&mut e, Ptr::new(0x1000), 2.75);
                assert_eq!(e.mem.f32(0x1000 + $offset), 2.75);
            }
        };
    }

    store_float_test!(store_float_558, fn_00930600, 0x558);
    store_float_test!(store_float_ac, fn_009314d0, 0xac);
    store_float_test!(store_float_55c, fn_00931560, 0x55c);

    macro_rules! store_byte_test {
        ($name:ident, $function:ident, $offset:expr) => {
            #[test]
            fn $name() {
                let mut e = Engine::new();
                e.map(0x1000, 0x1000);
                $function(&mut e, Ptr::new(0x1000), 0xa5);
                assert_eq!(e.mem.u8(0x1000 + $offset), 0xa5);
            }
        };
    }

    store_byte_test!(store_byte_97, fn_009314f0, 0x97);
    store_byte_test!(store_byte_96, fn_00931510, 0x96);
    store_byte_test!(store_byte_241, fn_009315a0, 0x241);

    #[test]
    fn reset_float_copies_global_bits() {
        let mut e = Engine::new();
        e.map(0x011b_0000, 0x1000);
        e.map(0x1000, 0x1000);
        e.mem.set_u32(0x011b_014c, 0x7f80_1234);
        fn_00930620(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1530), 0x7fc0_1234);
    }

    #[test]
    fn x87_store_accessors_quiet_signaling_nans() {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        let nan = f32::from_bits(0xff80_1234);
        for (address, call) in [
            (0x1558, fn_00930600 as fn(&mut Engine, Ptr, f32)),
            (0x10ac, fn_009314d0 as fn(&mut Engine, Ptr, f32)),
            (0x155c, fn_00931560 as fn(&mut Engine, Ptr, f32)),
        ] {
            call(&mut e, Ptr::new(0x1000), nan);
            assert_eq!(e.mem.u32(address), 0xffc0_1234);
        }
    }

    #[test]
    fn state_five_predicate() {
        for (value, expected) in [(5, true), (4, false)] {
            let mut e = Engine::new();
            stub(&mut e, 0x005c_0880, value);
            assert_eq!(fn_009306b0(&mut e, Ptr::new(0x1000)), expected);
        }
    }

    #[test]
    fn controller_scale_uses_optional_second_setting() {
        for (mode, expected) in [(4, 6.0f32), (5, 12.0f32)] {
            let mut e = Engine::new();
            let this = mapped(&mut e);
            assert!(fn_00930640(&mut e, this).is_null());
            e.mem.set_u32(0x1068, 0x2000);
            virtual_slot(&mut e, 0x2000, 0x3000, 0x28c, 0x6000);
            stub(&mut e, 0x6000, 0x4100);
            e.map(0x7000, 0x1000);
            e.mem.set_f32(0x7000, 3.0);
            e.mem.set_f32(0x7004, 2.0);
            e.register_double(0x0040_3e20, |_, args| {
                if args[0] == 0x011d_0274 {
                    0x7000u32.into_ret()
                } else {
                    0x7004u32.into_ret()
                }
            });
            e.register(0x0056_7400, |_, _| 2.0f32.into_ret());
            stub(&mut e, 0x005c_0880, mode);
            stub(&mut e, 0x00c6_d4b0, 0);
            e.call_log = Some(vec![]);
            assert_eq!(fn_00930640(&mut e, this).addr(), 0x4100);
            let log = e.call_log.unwrap();
            assert_eq!(log.last().unwrap().1, vec![0x4100, expected.to_bits()]);
            assert_eq!(
                log.iter().filter(|call| call.0 == 0x0040_3e20).count(),
                if mode == 5 { 2 } else { 1 }
            );
        }
    }

    #[test]
    fn char_controller_uses_process_virtual_slot() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        assert!(mobile_object_get_char_controller(&mut e, this).is_null());
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x28c, 0x6000);
        stub(&mut e, 0x6000, 0x4100);
        assert_eq!(
            mobile_object_get_char_controller(&mut e, this).addr(),
            0x4100
        );
    }

    #[test]
    fn parent_cell_update_exits_for_same_cell_and_null_new_cell() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        stub(&mut e, 0x008d_6f30, 0x4000);
        stub(&mut e, 0x0057_5bb0, 0);
        e.call_log = Some(vec![]);
        fn_00930700(&mut e, this, Ptr::new(0x4000));
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        fn_00930700(&mut e, this, Ptr::NULL);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            vec![0x008d_6f30, 0x0057_5bb0]
        );
        assert_eq!(log[1].1, vec![0x1000, 0]);
    }

    #[test]
    fn parent_cell_water_sentinel_updates_loaded_height() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.mem.set_u32(0x1064, 0x6000);
        e.map(0x6000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.mem.set_f64(0x0102_41b0, f32::MIN as f64);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x011d_ea10, 0x8000);
        stub(&mut e, 0x008d_6f30, 0);
        stub(&mut e, 0x0057_5bb0, 0);
        stub(&mut e, 0x0057_4900, 0);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x28c, 0x7000);
        stub(&mut e, 0x7000, 0x9000);
        e.register(0x0057_b0a0, |_, _| f32::MIN.into_ret());
        stub(&mut e, 0x0057_b1d0, 0);
        stub(&mut e, 0x0087_f390, 1);
        stub(&mut e, 0x005c_0880, 5);
        virtual_slot(&mut e, 0x1000, 0x4000, 0x1f4, 0x7100);
        stub(&mut e, 0x7100, 0xa000);
        e.register(0x0045_cbc0, |_, _| 25.0f32.into_ret());
        e.call_log = Some(vec![]);
        fn_00930700(&mut e, this, Ptr::new(0x5000));
        assert_eq!(e.mem.f32(0x6008), 25.0);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x7100 && call.1 == [0x1000, 0]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0045_cbc0 && call.1 == [0x8000, 0xa000, 0]));
    }

    #[test]
    fn controller_removal_passes_constructed_word_to_process() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x2a0, 0x6000);
        stub(&mut e, 0x6000, 0x102);
        assert_eq!(fn_00930810(&mut e, this), 2);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x4000, 0x290, 0x6100);
        e.register_double(0x0063_3c90, |e, args| {
            e.mem.set_u32(args[0], 0xa500);
            args[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(fn_00930810(&mut e, this), 2);
        let log = e.call_log.unwrap();
        assert_eq!(log.last().unwrap().1, vec![0x2000, 0xa500]);
    }

    #[test]
    fn controller_detach_clears_geometry_and_world() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        assert!(!fn_009307d0(&mut e, this));
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x28c, 0x6000);
        stub(&mut e, 0x6000, 0x4100);
        stub(&mut e, 0x00c6_ffc0, 0);
        stub(&mut e, 0x00c7_0c90, 0);
        e.call_log = Some(vec![]);
        assert!(fn_009307d0(&mut e, this));
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x4100, 0]);
    }

    #[test]
    fn remove_controller_sets_result_bit_for_attached_havok() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x2a0, 0x6000);
        stub(&mut e, 0x6000, 2);
        stub(&mut e, 0x0043_fcd0, 0x4000);
        stub(&mut e, 0x00c6_9ee0, 0);
        assert_eq!(fn_00930870(&mut e, this), 3);
    }

    #[test]
    fn remove_controller_reloads_process_after_mode_callback() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        e.register_double(0x0045_cd60, |e, _| {
            e.mem.set_u32(0x1068, 0x2100);
            0u32.into_ret()
        });
        stub(&mut e, 0x008e_56b0, 0);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x2a0, 0x6000);
        stub(&mut e, 0x0043_fcd0, 0);
        e.call_log = Some(vec![]);
        fn_00930870(&mut e, this);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|call| call.0 == 0x008e_56b0 && call.1 == [0x2100]));
    }

    #[test]
    fn word_at_18_is_nonzero_predicate() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        assert!(!fn_00930c50(&mut e, this.cast()));
        e.mem.set_u32(0x1018, 7);
        assert!(fn_00930c50(&mut e, this.cast()));
    }

    #[test]
    fn pointer_getter_uses_member_address() {
        let mut e = Engine::new();
        stub(&mut e, 0x0055_9450, 0x2000);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00931580(&mut e, Ptr::new(0x1000)).addr(), 0x2000);
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1434]);
    }

    #[test]
    fn display_geometry_does_nothing_without_controller() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        fn_009315c0(&mut e, this, 1);
    }

    #[test]
    fn display_geometry_show_and_hide_call_exact_controller_paths() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x2000, 0x3000, 0x28c, 0x6000);
        stub(&mut e, 0x6000, 0x4100);
        e.map(0x4000, 0x1000);
        virtual_slot(&mut e, 0x4100, 0x5000, 0xd0, 0x6100);
        stub(&mut e, 0x0081_0660, 0x7000);
        stub(&mut e, 0x00b6_b910, 0);
        stub(&mut e, 0x00c6_ffc0, 0);
        e.call_log = Some(vec![]);
        fn_009315c0(&mut e, this, 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            vec![0x6000, 0x6100, 0x0081_0660, 0x00b6_b910]
        );
        assert_eq!(log[1].1, vec![0x4100, 0]);
        assert_eq!(log[2].1, vec![0x4100]);
        assert_eq!(log[3].1, vec![0x7000, 1]);
        e.call_log = Some(vec![]);
        fn_009315c0(&mut e, this, 0);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            vec![0x6000, 0x00c6_ffc0]
        );
        assert_eq!(log[1].1, vec![0x4100]);
    }

    #[test]
    fn position_update_stops_after_reference_without_controller_or_node() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        stub(&mut e, 0x0057_5830, 0);
        e.call_log = Some(vec![]);
        fn_00931620(&mut e, this, Ptr::new(0x4000));
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000, 0x4000]);
    }

    #[test]
    fn position_update_reloads_loaded_data_after_first_getter() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.mem.set_u32(0x1064, 0x6000);
        stub(&mut e, 0x0057_5830, 0);
        e.register_double(0x0055_9450, |e, args| {
            if args[0] == 0x6014 {
                e.mem.set_u32(0x1064, 0x7000);
                0x8000u32.into_ret()
            } else {
                0x9000u32.into_ret()
            }
        });
        stub(&mut e, 0x0044_0460, 0);
        e.call_log = Some(vec![]);
        fn_00931620(&mut e, this, Ptr::new(0x4000));
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0055_9450 && call.1 == [0x7014]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0044_0460 && call.1 == [0x9000, 0x4000]));
    }

    #[test]
    fn package_start_forwards_world_cell_location_and_z() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.map(0x4000, 0x1000);
        e.mem.set_f32(0x4008, 9.5);
        stub(&mut e, 0x0043_0830, 0x4000);
        stub(&mut e, 0x008d_6f30, 0x5000);
        stub(&mut e, 0x0057_5d70, 0x6000);
        stub(&mut e, 0x0056_ae10, 0);
        virtual_slot(&mut e, 0x1000, 0x3000, 0x1f4, 0x7000);
        stub(&mut e, 0x7000, 0x8000);
        e.call_log = Some(vec![]);
        mobile_object_set_package_start_location(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x7000 && call.1 == [0x1000]));
        assert_eq!(
            log.last().unwrap().1,
            vec![0x1000, 0x6000, 0x5000, 0x8000, 9.5f32.to_bits()]
        );
    }

    #[test]
    fn high_process_replacement_reloads_process_after_callbacks() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        e.map(0x3000, 0x1000);
        e.mem.set_u32(0x1068, 0x2000);
        virtual_slot(&mut e, 0x1000, 0x5000, 0x1fc, 0x7400);
        virtual_slot(&mut e, 0x2200, 0x6000, 0, 0x7100);
        virtual_slot(&mut e, 0x3000, 0x7000, 0x4, 0x7200);
        e.mem.set_u32(0x7048, 0x7300);
        stub(&mut e, 0x7300, 0);
        virtual_slot(&mut e, 0x3200, 0x8000, 0x48, 0x7500);
        stub(&mut e, 0x0045_cd60, 2);
        stub(&mut e, 0x0040_fbf0, 0);
        stub(&mut e, 0x0040_1000, 0x4000);
        stub(&mut e, 0x008d_7510, 0x3000);
        stub(&mut e, 0x0096_d450, 0);
        stub(&mut e, 0x0040_fba0, 0);
        e.register_double(0x0096_d470, |e, _| {
            e.mem.set_u32(0x1068, 0x2100);
            0u32.into_ret()
        });
        e.register_double(0x7200, |e, _| {
            e.mem.set_u32(0x1068, 0x2200);
            0u32.into_ret()
        });
        e.register_double(0x7400, |e, _| {
            e.mem.set_u32(0x1068, 0x3200);
            0u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(mobile_object_replace_with_high_process(&mut e, this));
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            vec![
                0x0045_cd60,
                0x0040_fbf0,
                0x0096_d470,
                0x0040_1000,
                0x008d_7510,
                0x7200,
                0x7100,
                0x0096_d450,
                0x0040_fba0,
                0x7400,
                0x7500,
            ]
        );
        assert_eq!(log[0].1, vec![0x2000]);
        assert_eq!(log[1].1, vec![0x011f_11a0, 0]);
        assert_eq!(log[2].1, vec![0x011e_0e80, 0x1000, 2]);
        assert_eq!(log[3].1, vec![0x46c]);
        assert_eq!(log[4].1, vec![0x4000]);
        assert_eq!(log[5].1, vec![0x3000, 0x2100]);
        assert_eq!(log[6].1, vec![0x2200, 1]);
        assert_eq!(log[7].1, vec![0x011e_0e80, 0x1000, 0, 0, 0, 0]);
        assert_eq!(log[8].1, vec![0x011f_11a0]);
        assert_eq!(log[9].1, vec![0x1000, 0]);
        assert_eq!(log[10].1, vec![0x3200]);
    }

    #[test]
    fn process_type_is_minus_one_without_process() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        assert_eq!(mobile_object_get_current_process_type(&mut e, this), -1);
        e.mem.set_u32(0x1068, 0x2000);
        stub(&mut e, 0x0045_cd60, 3);
        assert_eq!(mobile_object_get_current_process_type(&mut e, this), 3);
    }
}
