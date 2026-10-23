//! `fallout shared/bgscamerashot.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSCameraShot` (Xbox PDB); the PC `TESForm` is 0x10 smaller, so
    /// every field below sits 0x10 lower than in the PDB.
    pub struct BgsCameraShot: 0x78 {
        /// `Data.eCameraAction` (Xbox PDB, `CAMERA_SHOT_DATA` +0x00).
        0x38 eCameraAction: u32,
        /// `Data.eLocation` (Xbox PDB).
        0x3C eLocation: u32,
        /// `Data.eTarget` (Xbox PDB).
        0x40 eTarget: u32,
        /// `Data.iFlags` (Xbox PDB).
        0x44 iFlags: u32,
        /// `Data.fPlayerTimeMult` (Xbox PDB).
        0x48 fPlayerTimeMult: f32,
        /// `Data.fTargetTimeMult` (Xbox PDB).
        0x4C fTargetTimeMult: f32,
        /// `Data.fGlobalTimeMult` (Xbox PDB).
        0x50 fGlobalTimeMult: f32,
        /// `Data.fMaxTime` (Xbox PDB).
        0x54 fMaxTime: f32,
        /// `Data.fMinTime` (Xbox PDB).
        0x58 fMinTime: f32,
        /// `Data.fTargetPercentBetweenActors` (Xbox PDB).
        0x5C fTargetPercentBetweenActors: f32,
        /// `pTargetRef` (Xbox PDB).
        0x68 pTargetRef: u32,
        /// `bCameraTooClose` (Xbox PDB).
        0x75 bCameraTooClose: u8,
    }
}

/// The `TESModel` base (Xbox PDB +0x28, PC +0x18) and the
/// `TESImageSpaceModifiableForm` base (PC +0x30).
const MODEL: u32 = 0x18;
const IMAGE_SPACE_FORM: u32 = 0x30;
/// The `NiPointer` members (Xbox PDB `spLocationNode`, `spTargetNode`,
/// `spCameraNode`, `spImagespaceTarget`).
const LOCATION_NODE: u32 = 0x60;
const TARGET_NODE: u32 = 0x64;
const CAMERA_NODE: u32 = 0x6c;
const IMAGESPACE_TARGET: u32 = 0x70;

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the flag
/// setter `InitItem` and `Load` end with.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// `"Bad VATS CameraShot Data: '%s' has a Global Time Multiplier that is
/// too small."`
const SMALL_TIME_MULTIPLIER_FORMAT: u32 = 0x0103_2930;
/// The smallest global time multiplier `InitItem` accepts (`double`) and
/// the `float` it puts in its place.
const MIN_GLOBAL_TIME_MULT: u32 = 0x0103_2980;
const DEFAULT_GLOBAL_TIME_MULT: u32 = 0x0101_e2bc;
/// `NiPointer` assignment (`0066b0d0`, `RET 4`) and its raw read
/// (`00559450`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer` destructor (`0045cec0`).
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// `"UIVATSCameraOut"`.
const CAMERA_OUT_SOUND: u32 = 0x0103_2988;
/// The byte that says the VATS camera sound is due when the camera is
/// freed, and the list (`011f2250`) whose count (`005ae380`) must be 0.
const CAMERA_OUT_PENDING: u32 = 0x011c_a729;
const CAMERA_OUT_LIST: u32 = 0x011f_2250;
const LIST_COUNT: u32 = 0x005a_e380;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `memset` (`00403d30`, cdecl, three words).
const MEMSET: u32 = 0x0040_3d30;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 0058be90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058be90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSCameraShot::~BGSCameraShot` (Xbox PDB), the body: puts back the
/// three vtables, frees the camera (`BGSCameraShot::FreeCamera`), destroys
/// the `NiPointer` members at +0x70, +0x6C, +0x64 and +0x60, the model
/// (`00489070`) and the `TESForm` base (`00483630`).
pub fn bgs_camera_shot_destructor(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_27f4);
    e.mem.set_u32(base + 0x18, 0x0103_27cc);
    e.mem.set_u32(base + 0x30, 0x0103_27b8);
    bgs_camera_shot_free_camera(e, this);
    for member in [IMAGESPACE_TARGET, CAMERA_NODE, TARGET_NODE, LOCATION_NODE] {
        e.call(NI_POINTER_DESTROY, &args![base + member]);
    }
    e.call(0x0048_9070, &args![base + MODEL]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The camera shot's data defaults (the engine map has no name): action
/// and location 2, target 0, flags 3, global time multiplier 1.0, the
/// other multipliers, times and percentage 0.0.
pub fn fn_0058bf50(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    e.set(this, BgsCameraShot::eCameraAction, 2);
    e.set(this, BgsCameraShot::eLocation, 2);
    e.set(this, BgsCameraShot::eTarget, 0);
    e.set(this, BgsCameraShot::fGlobalTimeMult, 1.0);
    e.set(this, BgsCameraShot::fPlayerTimeMult, 0.0);
    e.set(this, BgsCameraShot::fTargetTimeMult, 0.0);
    e.set(this, BgsCameraShot::iFlags, 3);
    e.set(this, BgsCameraShot::fMaxTime, 0.0);
    e.set(this, BgsCameraShot::fMinTime, 0.0);
    e.set(this, BgsCameraShot::fTargetPercentBetweenActors, 0.0);
}

// Translated from 0058c090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058c090 - the /GS stack cookie check (00ec408c) is not translated.
/// The camera shot's record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type (all of `EAX`) is 0x5B;
/// otherwise `TESForm::LoadForm` and each chunk: `EDID` (vtable +0x134),
/// `OBND` (vtable +0xE0), `MODL`/`MODT` (`TESModel::LoadModelChunk`),
/// `MNAM` (the image space modifier's form id, `006ecd40` on +0x30) and
/// `DATA`: a 0x20-byte record (the older form, `TESForm::LoadData`, swapped
/// by `0058c2e0`) spread into the data without the target time multiplier
/// and the percentage, or the 0x28-byte data read in place (swapped by
/// `0058c3c0`). Returns true.
pub fn fn_0058c090(e: &mut Engine, this: Ptr<BgsCameraShot>, file: Ptr) -> bool {
    if e.call(FILE_FORM_TYPE, &args![file]).u32() != 0x5b {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODT => {
                let model = if this.is_null() { 0 } else { base + MODEL };
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_DATA => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 0x20 {
                    let old = e.with_stack(0x20, |e, data| {
                        e.call(MEMSET, &args![data, 0u32, 0x20u32]);
                        e.call(0x0048_61f0, &args![this, file, data, 0x20u32]);
                        if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                            e.call(0x0058_c2e0, &args![data]);
                        }
                        let mut words = [0u32; 8];
                        for (i, word) in words.iter_mut().enumerate() {
                            *word = e.mem.u32(data.addr() + 4 * i as u32);
                        }
                        words
                    });
                    e.set(this, BgsCameraShot::eCameraAction, old[0]);
                    e.set(this, BgsCameraShot::eLocation, old[1]);
                    e.set(this, BgsCameraShot::eTarget, old[2]);
                    e.set(this, BgsCameraShot::iFlags, old[3]);
                    // FLD/FSTP through the x87 stack.
                    let floats = [
                        BgsCameraShot::fPlayerTimeMult,
                        BgsCameraShot::fGlobalTimeMult,
                        BgsCameraShot::fMaxTime,
                        BgsCameraShot::fMinTime,
                    ];
                    for (field, word) in floats.into_iter().zip(&old[4..]) {
                        e.set(this, field, f32::from_bits(x87_quiet_bits(*word)));
                    }
                } else {
                    e.call(MEMSET, &args![base + 0x38, 0u32, 0x28u32]);
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, base + 0x38, 0x28u32]);
                    if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                        e.call(0x0058_c3c0, &args![base + 0x38]);
                    }
                }
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_MNAM => {
                let id = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.call(0x006e_cd40, &args![base + IMAGE_SPACE_FORM, id]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0058c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraShot::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// `TESImageSpaceModifiableForm::InitItem`, then a global time multiplier
/// below the minimum (`double` at `01032980`; a NaN passes) is replaced by
/// the `float` at `0101e2bc` after a warning with the shot's name (vtable
/// +0x130), and `00484ab0(1)`.
pub fn bgs_camera_shot_init_item(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    e.call(0x0048_7490, &args![this.addr() + IMAGE_SPACE_FORM, this]);
    let multiplier = e.get(this, BgsCameraShot::fGlobalTimeMult);
    let minimum = e.global::<f64>(MIN_GLOBAL_TIME_MULT);
    // FCOMP; FNSTSW; TEST AH,5; JP: the warning runs only when below.
    if f64::from(multiplier) < minimum {
        let name = e.vcall(this.addr(), 0x130, &args![]).u32();
        e.call(WARNING, &args![SMALL_TIME_MULTIPLIER_FORMAT, name]);
        let default = e.global::<f32>(DEFAULT_GLOBAL_TIME_MULT);
        e.set(this, BgsCameraShot::fGlobalTimeMult, x87_quiet(default));
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 0058ce50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ce50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSCameraShot::FreeCamera` (Xbox PDB): clears the location and target
/// nodes, the target reference, the image space target and the too-close
/// flag; a camera node is cleared and the model's +0x14 slot result given
/// to `0045a5e0` on the object at `011c3b3c`. When the camera-out sound is
/// due and the list at `011f2250` is empty, it is played
/// (`BSAudio::GetSoundHandleByName` `"UIVATSCameraOut"` with flags 0x121,
/// `BSSoundHandle::Play`) and no longer due.
pub fn bgs_camera_shot_free_camera(e: &mut Engine, this: Ptr<BgsCameraShot>) {
    let base = this.addr();
    e.call(NI_POINTER_ASSIGN, &args![base + LOCATION_NODE, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + TARGET_NODE, 0u32]);
    e.set(this, BgsCameraShot::pTargetRef, 0);
    e.call(NI_POINTER_ASSIGN, &args![base + IMAGESPACE_TARGET, 0u32]);
    e.set(this, BgsCameraShot::bCameraTooClose, 0);
    if e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0 {
        e.call(NI_POINTER_ASSIGN, &args![base + CAMERA_NODE, 0u32]);
        let value = e.vcall(base + MODEL, 0x14, &args![]).u32();
        let target = e.global::<u32>(0x011c_3b3c);
        e.call(0x0045_a5e0, &args![target, value]);
    }
    if e.global::<u8>(CAMERA_OUT_PENDING) != 0
        && e.call(LIST_COUNT, &args![CAMERA_OUT_LIST]).u32() == 0
    {
        e.set_global(CAMERA_OUT_PENDING, 0u8);
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(
                0x00ad_7550,
                &args![audio, handle, CAMERA_OUT_SOUND, 0x121u32],
            );
            e.call(0x00ad_8830, &args![handle, 0u32]);
            e.call(0x0048_3710, &args![handle]);
        });
    }
}

/// `0048cee0` (`thiscall` on a `TESModel`: its file name's length, 0 without one).
const MODEL_NAME_LENGTH: u32 = 0x0048_cee0;

// Translated from 0058d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraShot::IsInitialized` (engine map; `ECX` only, the answer in `AL`): false for
/// camera action 3; true when the shot has no model file name; otherwise whether the
/// camera node (`NiPointer` at +0x6C) is set.
pub fn bgs_camera_shot_is_initialized(e: &mut Engine, this: Ptr<BgsCameraShot>) -> bool {
    if e.get(this, BgsCameraShot::eCameraAction) == 3 {
        return false;
    }
    let base = this.addr();
    if e.call(MODEL_NAME_LENGTH, &args![base + MODEL]).u32() == 0 {
        return true;
    }
    e.call(NI_POINTER_GET, &args![base + CAMERA_NODE]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0058_d630,
            bgs_camera_shot_is_initialized(Ptr<BgsCameraShot>) -> bool
        ),
        entry!(0x0058_be90, bgs_camera_shot_destructor(Ptr<BgsCameraShot>)),
        entry!(0x0058_bf50, fn_0058bf50(Ptr<BgsCameraShot>)),
        entry!(0x0058_c090, fn_0058c090(Ptr<BgsCameraShot>, Ptr) -> bool),
        entry!(0x0058_c550, bgs_camera_shot_init_item(Ptr<BgsCameraShot>)),
        entry!(0x0058_ce50, bgs_camera_shot_free_camera(Ptr<BgsCameraShot>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
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

    /// A shot whose own vtable (+0x130, +0x134, +0xE0) and model vtable
    /// (+0x14) slots are doubles.
    fn shot(e: &mut Engine) -> Ptr<BgsCameraShot> {
        let this: Ptr<BgsCameraShot> = e.new_object();
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this.addr(), vtable);
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0x14, 0x7100_0014);
        e.mem.set_u32(this.addr() + MODEL, model);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        e.register(0x7100_0014, |_, _| ret(0x6666));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    fn free_camera_doubles(e: &mut Engine, camera: u32, pending: u8, listed: u32) {
        e.map(0x011c_3000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.set_global(0x011c_3b3cu32, 0x4444u32);
        e.set_global(CAMERA_OUT_PENDING, pending);
        quiet(
            e,
            &[
                NI_POINTER_ASSIGN,
                0x0045_a5e0,
                0x00ad_7550,
                0x00ad_8830,
                0x0048_3710,
                NI_POINTER_DESTROY,
                0x0048_9070,
                0x0048_3630,
            ],
        );
        e.register_double(NI_POINTER_GET, move |_, _| ret(camera));
        e.register_double(LIST_COUNT, move |_, _| ret(listed));
        e.register(0x0045_3a70, |_, _| ret(0x3333));
    }

    #[test]
    fn free_camera_clears_the_members_and_plays_the_sound() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.set(this, BgsCameraShot::pTargetRef, 9);
        e.set(this, BgsCameraShot::bCameraTooClose, 1);
        free_camera_doubles(&mut e, 0x8000, 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_ce50, &args![this]);
        let base = this.addr();
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![
                vec![base + 0x60, 0],
                vec![base + 0x64, 0],
                vec![base + 0x70, 0],
                vec![base + 0x6c, 0]
            ]
        );
        assert_eq!(e.get(this, BgsCameraShot::pTargetRef), 0);
        assert_eq!(e.get(this, BgsCameraShot::bCameraTooClose), 0);
        assert_eq!(calls(&e, 0x0045_a5e0), vec![vec![0x4444, 0x6666]]);
        assert_eq!(calls(&e, LIST_COUNT), vec![vec![CAMERA_OUT_LIST]]);
        let handle = calls(&e, 0x00ad_7550);
        assert_eq!(handle.len(), 1);
        assert_eq!(handle[0][0], 0x3333);
        assert_eq!(&handle[0][2..], &[CAMERA_OUT_SOUND, 0x121]);
        assert_eq!(calls(&e, 0x00ad_8830), vec![vec![handle[0][1], 0]]);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![handle[0][1]]]);
        assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), 0);
    }

    #[test]
    fn free_camera_without_camera_or_with_listed_entries_skips_both() {
        for (pending, listed) in [(0u8, 0u32), (1, 2)] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            free_camera_doubles(&mut e, 0, pending, listed);
            e.call_log = Some(vec![]);
            e.call(0x0058_ce50, &args![this]);
            assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 3);
            assert!(calls(&e, 0x0045_a5e0).is_empty());
            assert!(calls(&e, 0x00ad_7550).is_empty());
            assert_eq!(e.global::<u8>(CAMERA_OUT_PENDING), pending);
        }
    }

    #[test]
    fn destructor_frees_the_camera_and_destroys_the_members() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        free_camera_doubles(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_be90, &args![this]);
        let base = this.addr();
        assert_eq!(e.mem.u32(base), 0x0103_27f4);
        assert_eq!(e.mem.u32(base + 0x18), 0x0103_27cc);
        assert_eq!(e.mem.u32(base + 0x30), 0x0103_27b8);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 3);
        assert_eq!(
            calls(&e, NI_POINTER_DESTROY),
            vec![
                vec![base + 0x70],
                vec![base + 0x6c],
                vec![base + 0x64],
                vec![base + 0x60]
            ]
        );
        assert_eq!(calls(&e, 0x0048_9070), vec![vec![base + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![base]]);
    }

    #[test]
    fn defaults_fill_the_data() {
        let mut e = Engine::new();
        let this: Ptr<BgsCameraShot> = e.new_object();
        for offset in (0x38..0x60).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call(0x0058_bf50, &args![this]);
        let words: Vec<u32> = (0x38..0x60)
            .step_by(4)
            .map(|offset| e.mem.u32(this.addr() + offset))
            .collect();
        assert_eq!(words, vec![2, 2, 0, 3, 0, 0, 0x3f80_0000, 0, 0, 0]);
    }

    #[test]
    fn init_item_replaces_a_too_small_global_multiplier() {
        for (value, warned) in [(0.00005f32, true), (0.5, false), (f32::NAN, false)] {
            let mut e = Engine::new();
            let this = shot(&mut e);
            e.map(0x0103_2000, 0x1000);
            e.map(0x0101_e000, 0x1000);
            e.set_global(MIN_GLOBAL_TIME_MULT, 0.0001f64);
            e.set_global(DEFAULT_GLOBAL_TIME_MULT, 0.1f32);
            e.set(this, BgsCameraShot::fGlobalTimeMult, value);
            e.register(FORM_INIT_SKIP, |_, _| ret(0));
            quiet(&mut e, &[0x0048_7490, WARNING, FORM_SET_FLAG]);
            e.call_log = Some(vec![]);
            e.call(0x0058_c550, &args![this]);
            assert_eq!(
                calls(&e, 0x0048_7490),
                vec![vec![this.addr() + 0x30, this.addr()]]
            );
            if warned {
                assert_eq!(
                    calls(&e, WARNING),
                    vec![vec![SMALL_TIME_MULTIPLIER_FORMAT, 0x5555]]
                );
                assert_eq!(e.get(this, BgsCameraShot::fGlobalTimeMult), 0.1);
            } else {
                assert!(calls(&e, WARNING).is_empty());
                assert_eq!(
                    e.get(this, BgsCameraShot::fGlobalTimeMult).to_bits(),
                    value.to_bits()
                );
            }
            assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this.addr(), 1]]);
        }
        // Skipped: nothing.
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        quiet(&mut e, &[0x0048_7490]);
        e.call_log = Some(vec![]);
        e.call(0x0058_c550, &args![this]);
        assert!(calls(&e, 0x0048_7490).is_empty());
    }

    /// A file whose chunks are `chunks` (type, size, data words).
    fn file(e: &mut Engine, form_type: u32, chunks: Vec<(u32, u32, Vec<u32>)>, swap: bool) -> Ptr {
        let file = Ptr::new(e.mem.alloc(0x10));
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let c = c.borrow();
            ret(c.0[c.1].1)
        });
        let write = |e: &mut Engine, at: u32, words: &[u32]| {
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *w);
            }
        };
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            write(e, a[1], &c.0[c.1].2);
            Ret::default()
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            write(e, a[1], &c.0[c.1].2);
            Ret::default()
        });
        let c = chunks;
        e.register_double(0x0048_61f0, move |e, a| {
            let c = c.borrow();
            write(e, a[2], &c.0[c.1].2);
            Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(
            e,
            &[
                0x0048_5110,
                0x0048_92d0,
                0x006e_cd40,
                0x0058_c2e0,
                0x0058_c3c0,
            ],
        );
        file
    }

    #[test]
    fn load_reads_the_older_data_record() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        e.set(this, BgsCameraShot::fTargetTimeMult, 7.0);
        let data = vec![
            1,
            2,
            3,
            4,
            0x7f80_0001,
            1.5f32.to_bits(),
            2.5f32.to_bits(),
            3.5f32.to_bits(),
        ];
        let f = file(
            &mut e,
            0x5b,
            vec![
                (CHUNK_EDID, 8, vec![0x6f6c]),
                (CHUNK_OBND, 12, vec![]),
                (CHUNK_MODL, 8, vec![]),
                (CHUNK_MODT, 8, vec![]),
                (CHUNK_MNAM, 4, vec![0x1234]),
                (CHUNK_DATA, 0x20, data),
            ],
            true,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_c090, &args![this, f]).bool());
        let base = this.addr();
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![base, f.addr()]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![base, f.addr()]]);
        assert_eq!(calls(&e, 0x0048_92d0), vec![vec![base + 0x18, f.addr()]; 2]);
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![base + 0x30, 0x1234]]);
        let loaded = calls(&e, 0x0048_61f0);
        assert_eq!(loaded.len(), 1);
        assert_eq!(calls(&e, 0x0058_c2e0), vec![vec![loaded[0][2]]]);
        assert_eq!(e.get(this, BgsCameraShot::eCameraAction), 1);
        assert_eq!(e.get(this, BgsCameraShot::eLocation), 2);
        assert_eq!(e.get(this, BgsCameraShot::eTarget), 3);
        assert_eq!(e.get(this, BgsCameraShot::iFlags), 4);
        // The signalling NaN comes back quiet through FLD/FSTP.
        assert_eq!(e.mem.u32(base + 0x48), 0x7fc0_0001);
        assert_eq!(e.get(this, BgsCameraShot::fTargetTimeMult), 7.0);
        assert_eq!(e.get(this, BgsCameraShot::fGlobalTimeMult), 1.5);
        assert_eq!(e.get(this, BgsCameraShot::fMaxTime), 2.5);
        assert_eq!(e.get(this, BgsCameraShot::fMinTime), 3.5);
    }

    #[test]
    fn load_reads_the_full_data_in_place_and_refuses_other_records() {
        let mut e = Engine::new();
        let this = shot(&mut e);
        let f = file(&mut e, 0x5b, vec![(CHUNK_DATA, 0x28, vec![9, 8, 7])], true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_c090, &args![this, f]).bool());
        let base = this.addr();
        assert_eq!(calls(&e, MEMSET), vec![vec![base + 0x38, 0, 0x28]]);
        assert_eq!(
            calls(&e, FILE_CHUNK_DATA_SIZED),
            vec![vec![f.addr(), base + 0x38, 0x28]]
        );
        assert_eq!(calls(&e, 0x0058_c3c0), vec![vec![base + 0x38]]);
        assert_eq!(e.get(this, BgsCameraShot::eTarget), 7);

        // A form type with the right low byte but more in `EAX`: refused.
        let mut e = Engine::new();
        let this = shot(&mut e);
        let f = file(&mut e, 0x015b, vec![(CHUNK_DATA, 0x28, vec![])], false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0058_c090, &args![this, f]).bool());
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn is_initialized_checks_the_action_the_model_and_the_node() {
        let mut e = Engine::new();
        e.register(MODEL_NAME_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let shot: Ptr<BgsCameraShot> = e.new_object();
        let ask = |e: &mut Engine| e.call(0x0058_d630, &args![shot]).bool();
        // No model name: initialized.
        assert!(ask(&mut e));
        // A model without its node: not yet.
        e.mem.set_u32(shot.addr() + MODEL + 4, 5);
        assert!(!ask(&mut e));
        e.mem.set_u32(shot.addr() + CAMERA_NODE, 0x7000);
        assert!(ask(&mut e));
        // Action 3 is never initialized.
        e.set(shot, BgsCameraShot::eCameraAction, 3);
        assert!(!ask(&mut e));
    }
}
