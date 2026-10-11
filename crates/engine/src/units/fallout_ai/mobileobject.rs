//! `fallout/ai/mobileobject.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Continuation queue: the first 40 address-ordered entries end at 00931850.
//! Six bodies remain open and unregistered after this batch:
//! - 0092ec40: destructor's process-list and reference teardown crosses an
//!   SEH cleanup frame whose normal call ABI still needs verification.
//! - 0092ed60: multi-list cleanup has dynamic casts and repeated virtual
//!   process callbacks whose PC argument order remains unaudited.
//! - 0092f260: the large movement state machine needs branch-by-branch PC
//!   stack and x87 review before a faithful translation.
//! - 009308f0: controller/world synchronization needs its temporary shape,
//!   ragdoll and world callback lifetimes verified.
//! - 00930c70: InitHavok's large physics setup has nested ownership and
//!   cleanup paths that remain unaudited.
//! - 00931710: high-process replacement has allocation, ownership transfer
//!   and compiler SEH cleanup that remain unaudited.
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
        entry!(0x0092f140, fn_0092f140(Ptr) -> Ptr),
        entry!(
            0x0092f160,
            mobile_object_is_talking(Ptr<MobileObject>) -> bool
        ),
        entry!(0x0092f190, fn_0092f190(Ptr<MobileObject>)),
        entry!(0x0092f1c0, fn_0092f1c0(Ptr<MobileObject>, u32)),
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
        entry!(0x00930c50, fn_00930c50(Ptr) -> bool),
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

// Translated from 00930c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the word at offset 0x18 is nonzero.
pub fn fn_00930c50(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) != 0
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
        let object = e.call(0x0081_0660, &args![controller, 1u32]).u32();
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
        assert_eq!(log[2].1, vec![0x4100, 1]);
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
    fn process_type_is_minus_one_without_process() {
        let mut e = Engine::new();
        let this = mapped(&mut e);
        assert_eq!(mobile_object_get_current_process_type(&mut e, this), -1);
        e.mem.set_u32(0x1068, 0x2000);
        stub(&mut e, 0x0045_cd60, 3);
        assert_eq!(mobile_object_get_current_process_type(&mut e, this), 3);
    }
}
