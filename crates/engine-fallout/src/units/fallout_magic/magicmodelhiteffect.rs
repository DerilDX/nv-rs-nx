//! `fallout/magic/magicmodelhiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MagicModelHitEffect` (Xbox PDB), 0x3c bytes on both builds (the scalar deleting
    /// destructor frees 0x3c): a `MagicHitEffect` (0x28) and its model.
    pub struct MagicModelHitEffect: 0x3c {
        /// `MagicHitEffect::pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x1C pTarget: u32,
        /// `MagicHitEffect::fAge` (Xbox PDB).
        0x20 fAge: u32,
        /// `MagicHitEffect::bFinished` (Xbox PDB).
        0x24 bFinished: u8,
        /// `bOwnPath` (Xbox PDB).
        0x28 bOwnPath: u8,
        /// `b3rdPerson` (Xbox PDB).
        0x29 b3rdPerson: u8,
        /// `pFilename` (Xbox PDB): `const char*`.
        0x2C pFilename: u32,
        /// `spObject` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x30 spObject: u32,
        /// `spTarget` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x34 spTarget: u32,
        /// `LoadedDataSubBuffer` (Xbox PDB): a `BGSLoadGameSubBuffer`.
        0x38 LoadedDataSubBuffer: u32,
    }
}

/// `MagicModelHitEffect`'s vtable (`0107b24c`).
const MAGIC_MODEL_HIT_EFFECT_VTABLE: u32 = 0x0107_b24c;
/// `MagicHitEffect::MagicHitEffect` (`0081b700`, `thiscall`, two words).
const MAGIC_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_b700;
/// `MagicHitEffect::~MagicHitEffect` (`0081b7b0`, `thiscall`).
const MAGIC_HIT_EFFECT_DESTRUCTOR: u32 = 0x0081_b7b0;
/// The `NiPointer` constructor `00633c90` (`thiscall`, one word: the object).
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
/// The `NiPointer` assignment `0066b0d0` (`thiscall`, one word: the object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The `NiPointer` destructor `0045cec0` (`thiscall`).
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `00559450` (`thiscall`): the object a `NiPointer` holds.
const POINTER_GET: u32 = 0x0055_9450;
/// The sub-buffer constructor `0044dee0` and destructor `00435b20` (`thiscall`).
const SUB_BUFFER_CONSTRUCTOR: u32 = 0x0044_dee0;
const SUB_BUFFER_DESTRUCTOR: u32 = 0x0043_5b20;
/// `0044ddc0` (`thiscall`): the word at +8 (the active effect's `pSpell`).
const EFFECT_ITEM_0044DDC0: u32 = 0x0044_ddc0;
/// `MagicItem::GetAVEffect` (engine map, `0040a300`, `thiscall`, one word).
const GET_AV_EFFECT: u32 = 0x0040_a300;
/// `009611e0` (`thiscall`): the word at +0x18 (the hit effect's `pEffect`, an `NiAVObject`'s
/// parent).
const WORD_18_009611E0: u32 = 0x0096_11e0;
/// The model loader (`ModelLoader *`, `011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `ModelLoader::ReleaseModel` (engine map, `00445300`, `thiscall`, three words).
const RELEASE_MODEL: u32 = 0x0044_5300;
/// `ModelLoader::LoadFile` (engine map, `00447080`, `thiscall`, six words).
const LOAD_FILE: u32 = 0x0044_7080;
/// The free `00401030` (`cdecl`, one word).
const FREE: u32 = 0x0040_1030;
/// `006b9130` (`thiscall`): the float at +4 in ST0 (the active effect's `fElapsedSeconds`).
const START_AGE_006B9130: u32 = 0x006b_9130;
/// `007df1d0` (`thiscall`) and `00403e60` (`thiscall`, one word: a flag mask).
const OBJECT_007DF1D0: u32 = 0x007d_f1d0;
const HAS_FLAGS_00403E60: u32 = 0x0040_3e60;
/// The player (`PlayerCharacter *`, `011dea3c`) and `004eaf60` on it (a byte).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_FLAG_004EAF60: u32 = 0x004e_af60;
/// `0043b230` (`thiscall`) and `00653270` (`cdecl`, two words) on `011f36ac`: the
/// controller manager of a model.
const MODEL_ROOT_0043B230: u32 = 0x0043_b230;
const CONTROLLER_MANAGER_00653270: u32 = 0x0065_3270;
const LOOKUP_ARGUMENT_011F36AC: u32 = 0x011f_36ac;
/// The string constructor `00438170` (`thiscall`, one word) and destructor `004381b0`.
const STRING_CONSTRUCTOR: u32 = 0x0043_8170;
const STRING_DESTRUCTOR: u32 = 0x0043_81b0;
/// `"SpecialIdle_HitEffect"`.
const HIT_EFFECT_IDLE: u32 = 0x0103_bdb8;
/// `0047a520` (`thiscall`, one word: the name): the controller sequence of that name.
const GET_SEQUENCE_0047A520: u32 = 0x0047_a520;
/// `MagicProjectile::Clone3D_ov2` (engine map, `0081e460`, `cdecl`, one word).
const CLONE_3D: u32 = 0x0081_e460;
/// `BSUtilities::ProcessSelectiveUpdate` (engine map, `00c4b500`, `cdecl`, one word).
const PROCESS_SELECTIVE_UPDATE: u32 = 0x00c4_b500;
/// `00825c00` (`thiscall`, no words) on `011f6394`: its word +0x14, divided by 1000.0 as a time.
const TIME_00825C00: u32 = 0x0082_5c00;
const TIMER_011F6394: u32 = 0x011f_6394;
/// `1000.0` (`double`, `01017b70`) and `0.0` (`double`, `01012060`).
const THOUSAND: u32 = 0x0101_7b70;
const ZERO: u32 = 0x0101_2060;
/// `0043d410` (`thiscall`, three words: a `float`, two words): fills an update block.
const UPDATE_BLOCK_0043D410: u32 = 0x0043_d410;
/// `00a59c60` (`thiscall`, one word: the update block).
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// `NiControllerManager::DeactivateAll` (engine map, `0048fef0`, `thiscall`, one `float`).
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
/// `0047aab0` (`thiscall`, six words) and `0047aa40` (`thiscall`, one word).
const ACTIVATE_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const SET_FLAG_0047AA40: u32 = 0x0047_aa40;
/// `NiControllerSequence::SetPhase` (engine map, `00a328b0`, `thiscall`, two words).
const SET_PHASE: u32 = 0x00a3_28b0;

// Translated from 0081c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::MagicModelHitEffect` (engine map; `thiscall`, `RET 8`: the target
/// reference and the active effect): constructs the `MagicHitEffect` base, stores the
/// vtable, constructs `spObject` and `spTarget` (null) and the sub-buffer, assigns null to
/// both pointers, clears `b3rdPerson` and `pFilename`, then takes the file name from the
/// effect's magic item (`0044ddc0`, `MagicItem::GetAVEffect(0)`, the object at its +0x18
/// answering through its slot +0x14) and clears `bOwnPath`. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c630 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_constructor(
    e: &mut Engine,
    this: Ptr<MagicModelHitEffect>,
    target: u32,
    effect: u32,
) -> Ptr<MagicModelHitEffect> {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this, target, effect]);
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    e.call(POINTER_CONSTRUCTOR, &args![base + 0x30, 0u32]);
    e.call(POINTER_CONSTRUCTOR, &args![base + 0x34, 0u32]);
    e.call(SUB_BUFFER_CONSTRUCTOR, &args![base + 0x38]);
    e.call(POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    e.set(this, MagicModelHitEffect::b3rdPerson, 0);
    e.set(this, MagicModelHitEffect::pFilename, 0);
    let item = e.call(EFFECT_ITEM_0044DDC0, &args![effect]).u32();
    let av_effect = e.call(GET_AV_EFFECT, &args![item, 0u32]).u32();
    let object = av_effect.wrapping_add(0x18);
    let name = e.vcall(object, 0x14, &args![]).u32();
    e.set(this, MagicModelHitEffect::pFilename, name);
    e.set(this, MagicModelHitEffect::bOwnPath, 0);
    this
}

// Translated from 0081c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::~MagicModelHitEffect` (engine map; `thiscall`): stores the vtable;
/// when `spObject` holds an object, detaches it from its parent (`009611e0` on it: the
/// word at +0x18; the parent's slot +0xe8 with the object, when there is a parent) and
/// assigns null to `spObject`; assigns null to `spTarget`; releases the model of a non-empty `pFilename`
/// (`ModelLoader::ReleaseModel(name, 0, 1)`) and frees the name when `bOwnPath` is set;
/// clears `pFilename`; then destroys the sub-buffer, `spTarget`, `spObject` and the
/// `MagicHitEffect` base.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c810 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_destructor(e: &mut Engine, this: Ptr<MagicModelHitEffect>) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    if e.call(POINTER_GET, &args![base + 0x30]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
        let parent = e.call(WORD_18_009611E0, &args![object]).u32();
        if parent != 0 {
            let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
            e.vcall(parent, 0xe8, &args![object]);
        }
        e.call(POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    }
    e.call(POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    if name != 0 && e.mem.i8(name) != 0 {
        let loader = e.mem.u32(MODEL_LOADER);
        let name = e.get(this, MagicModelHitEffect::pFilename);
        e.call(RELEASE_MODEL, &args![loader, name, 0u32, 1u32]);
        if e.get(this, MagicModelHitEffect::bOwnPath) != 0 {
            let name = e.get(this, MagicModelHitEffect::pFilename);
            e.call(FREE, &args![name]);
        }
    }
    e.set(this, MagicModelHitEffect::pFilename, 0);
    e.call(SUB_BUFFER_DESTRUCTOR, &args![base + 0x38]);
    e.call(POINTER_DESTRUCTOR, &args![base + 0x34]);
    e.call(POINTER_DESTRUCTOR, &args![base + 0x30]);
    e.call(MAGIC_HIT_EFFECT_DESTRUCTOR, &args![this]);
}

/// The controller sequence named `"SpecialIdle_HitEffect"` of the controller manager
/// `manager` (`0047a520`, given a string the code builds and destroys on the stack).
fn hit_effect_sequence(e: &mut Engine, manager: u32) -> u32 {
    e.with_stack(4, |e, string| {
        let name = e
            .call(STRING_CONSTRUCTOR, &args![string, HIT_EFFECT_IDLE])
            .u32();
        let sequence = e.call(GET_SEQUENCE_0047A520, &args![manager, name]).u32();
        e.call(STRING_DESTRUCTOR, &args![string]);
        sequence
    })
}

// Translated from 0081c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect::Init` (engine map; `thiscall`, plain `RET`; true in `AL` when the
/// hit effect model was attached):
///
/// - When `pEffect` (`009611e0`) is set, `fAge` takes the active effect's elapsed time
///   (`006b9130`) and `bFinished` is set when its `007df1d0` object lacks flag 0x400
///   (`00403e60`).
/// - It needs a target whose slot +0x1d0 answers non-zero, and a non-empty `pFilename`;
///   for the player `b3rdPerson` takes `004eaf60`.
/// - When the model loads (`ModelLoader::LoadFile(name, 0, 1, 0, 0, 0)`) and has a
///   controller manager (`0043b230`, `00653270` with `011f36ac`), that manager must have
///   a `"SpecialIdle_HitEffect"` sequence, or the call answers false.
/// - `spObject` becomes a clone of the model (`MagicProjectile::Clone3D_ov2`) and the model
///   is released (`ModelLoader::ReleaseModel(name, 0, 1)`). With a clone and the target
///   still answering, this effect's slot +0xcc runs, the clone gets a selective update
///   (`BSUtilities::ProcessSelectiveUpdate`) and an update at the time of `00825c00` on
///   `011f6394` divided by 1000.0 (`0043d410` fills a 12-byte update block with the time,
///   1 and 0; `00a59c60`). When the clone has a controller manager with that sequence,
///   the manager deactivates all (0.0), activates it (`0047aab0` with 0, 0, 1.0, 0.0, 0),
///   `0047aa40(1)` runs, and a positive `fAge` (`FCOMP` with `01012060`, not NaN) sets the
///   sequence's phase (`NiControllerSequence::SetPhase(fAge, 0)`): the answer is true.
///
/// Any other outcome answers false. The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081c950 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_model_hit_effect_init(e: &mut Engine, this: Ptr<MagicModelHitEffect>) -> bool {
    let base = this.addr();
    if e.call(WORD_18_009611E0, &args![this]).u32() != 0 {
        let effect = e.call(WORD_18_009611E0, &args![this]).u32();
        let age = e.call(START_AGE_006B9130, &args![effect]).f64();
        e.set(this, MagicModelHitEffect::fAge, x87_store32(age));
        let effect = e.call(WORD_18_009611E0, &args![this]).u32();
        let object = e.call(OBJECT_007DF1D0, &args![effect]).u32();
        let has = e.call(HAS_FLAGS_00403E60, &args![object, 0x400u32]).u8() != 0;
        e.set(this, MagicModelHitEffect::bFinished, u8::from(!has));
    }
    let target = e.get(this, MagicModelHitEffect::pTarget);
    if target == 0 || e.vcall(target, 0x1d0, &args![]).u32() == 0 {
        return false;
    }
    let player = e.mem.u32(PLAYER);
    if e.get(this, MagicModelHitEffect::pTarget) == player {
        let third_person = e.call(PLAYER_FLAG_004EAF60, &args![player]).u8();
        e.set(this, MagicModelHitEffect::b3rdPerson, third_person);
    }
    let name = e.get(this, MagicModelHitEffect::pFilename);
    if name == 0 || e.mem.i8(name) == 0 {
        return false;
    }
    let loader = e.mem.u32(MODEL_LOADER);
    let model = e
        .call(
            LOAD_FILE,
            &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if model != 0 {
        let root = e.call(MODEL_ROOT_0043B230, &args![model]).u32();
        let manager = e
            .call(
                CONTROLLER_MANAGER_00653270,
                &args![LOOKUP_ARGUMENT_011F36AC, root],
            )
            .u32();
        if manager != 0 && hit_effect_sequence(e, manager) == 0 {
            return false;
        }
    }
    let name = e.get(this, MagicModelHitEffect::pFilename);
    let clone = e.call(CLONE_3D, &args![name]).u32();
    e.call(POINTER_ASSIGN, &args![base + 0x30, clone]);
    let loader = e.mem.u32(MODEL_LOADER);
    let name = e.get(this, MagicModelHitEffect::pFilename);
    e.call(RELEASE_MODEL, &args![loader, name, 0u32, 1u32]);
    if e.call(POINTER_GET, &args![base + 0x30]).u32() == 0 {
        return false;
    }
    let target = e.get(this, MagicModelHitEffect::pTarget);
    if e.vcall(target, 0x1d0, &args![]).u32() == 0 {
        return false;
    }
    e.vcall(base, 0xcc, &args![]);
    let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
    e.call(PROCESS_SELECTIVE_UPDATE, &args![object]);
    let milliseconds = e.call(TIME_00825C00, &args![TIMER_011F6394]).u32();
    let thousand = f64::from_bits(e.mem.u64(THOUSAND));
    let seconds = x87_store32(x87_div(f64::from(milliseconds), thousand));
    e.with_stack(0xc, |e, block| {
        e.call(
            UPDATE_BLOCK_0043D410,
            &args![block, x87_quiet_bits(seconds), 1u32, 0u32],
        );
        let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
        e.call(UPDATE_00A59C60, &args![object, block]);
    });
    let object = e.call(POINTER_GET, &args![base + 0x30]).u32();
    let root = e.call(MODEL_ROOT_0043B230, &args![object]).u32();
    let manager = e
        .call(
            CONTROLLER_MANAGER_00653270,
            &args![LOOKUP_ARGUMENT_011F36AC, root],
        )
        .u32();
    if manager == 0 {
        return false;
    }
    let sequence = hit_effect_sequence(e, manager);
    if sequence == 0 {
        return false;
    }
    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
    e.call(
        ACTIVATE_SEQUENCE_0047AAB0,
        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
    );
    e.call(SET_FLAG_0047AA40, &args![manager, 1u32]);
    let age = x87_load32(e.get(this, MagicModelHitEffect::fAge));
    let zero = f64::from_bits(e.mem.u64(ZERO));
    // FCOMP double; TEST AH,0x41; JNZ: the phase is set only when fAge > 0.0.
    if age > zero {
        let age = x87_quiet_bits(e.get(this, MagicModelHitEffect::fAge));
        e.call(SET_PHASE, &args![sequence, age, 0u32]);
    }
    true
}

// Translated from 0081c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicModelHitEffect` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `0081c810` and frees 0x3c bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn magic_model_hit_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0081_c810, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 0081c5f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011dc724 without reading the receiver. The map names this body `MagicModelHitEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0081c5f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_c724
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_c5f0, fn_0081c5f0(Ptr) -> u32),
        entry!(0x0081_c600, magic_model_hit_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0081_c630,
            magic_model_hit_effect_constructor(
                Ptr<MagicModelHitEffect>,
                u32,
                u32,
            ) -> Ptr<MagicModelHitEffect>
        ),
        entry!(
            0x0081_c810,
            magic_model_hit_effect_destructor(Ptr<MagicModelHitEffect>)
        ),
        entry!(
            0x0081_c950,
            magic_model_hit_effect_init(Ptr<MagicModelHitEffect>) -> bool
        ),
    ]
}

#[cfg(test)]
mod model_hit_effect_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// `NiPointer` doubles over plain words, and silent doubles for the rest.
    fn pointer_engine() -> Engine {
        let mut e = Engine::new();
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_18_009611E0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        for address in [
            MAGIC_HIT_EFFECT_CONSTRUCTOR,
            MAGIC_HIT_EFFECT_DESTRUCTOR,
            POINTER_DESTRUCTOR,
            SUB_BUFFER_CONSTRUCTOR,
            SUB_BUFFER_DESTRUCTOR,
            RELEASE_MODEL,
            FREE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(MODEL_LOADER, 0x3b3c);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_takes_the_file_name_from_the_base_effect() {
        let mut e = pointer_engine();
        e.register(EFFECT_ITEM_0044DDC0, |_, _| 0x4400u32.into_ret());
        let av_effect = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x18);
        e.mem.set_u32(vtable + 0x14, 0x7014);
        e.register(0x7014, |_, _| 0x0abcu32.into_ret());
        e.mem.set_u32(av_effect + 0x18, vtable);
        e.register_double(GET_AV_EFFECT, move |_, _| av_effect.into_ret());
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xee; 0x3c]);
        assert_eq!(e.call(0x0081_c630, &[this, 0x10, 0x20]).u32(), this);
        assert_eq!(
            called(&e, MAGIC_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![this, 0x10, 0x20]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_MODEL_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(called(&e, SUB_BUFFER_CONSTRUCTOR), vec![vec![this + 0x38]]);
        assert_eq!(called(&e, EFFECT_ITEM_0044DDC0), vec![vec![0x20]]);
        assert_eq!(called(&e, GET_AV_EFFECT), vec![vec![0x4400, 0]]);
        assert_eq!(called(&e, 0x7014), vec![vec![av_effect + 0x18]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x0abc);
        assert_eq!(e.mem.u8(this + 0x28), 0);
        assert_eq!(e.mem.u8(this + 0x29), 0);
    }

    #[test]
    fn destructor_detaches_releases_and_frees_an_owned_name() {
        for owned in [0u8, 1] {
            let mut e = pointer_engine();
            let parent_vtable = e.mem.alloc(0xec);
            e.mem.set_u32(parent_vtable + 0xe8, 0x70e8);
            e.register(0x70e8, |_, _| Ret::default());
            let parent = e.mem.alloc(8);
            e.mem.set_u32(parent, parent_vtable);
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object + 0x18, parent);
            let name = e.mem.alloc(4);
            e.mem.set_u8(name, b'a');
            let this = e.mem.alloc(0x3c);
            e.mem.set_u32(this + 0x30, object);
            e.mem.set_u32(this + 0x34, 0x3434);
            e.mem.set_u32(this + 0x2c, name);
            e.mem.set_u8(this + 0x28, owned);
            e.call(0x0081_c810, &[this]);
            assert_eq!(e.mem.u32(this), MAGIC_MODEL_HIT_EFFECT_VTABLE);
            assert_eq!(called(&e, 0x70e8), vec![vec![parent, object]]);
            assert_eq!(e.mem.u32(this + 0x30), 0);
            assert_eq!(e.mem.u32(this + 0x34), 0);
            assert_eq!(called(&e, RELEASE_MODEL), vec![vec![0x3b3c, name, 0, 1]]);
            assert_eq!(called(&e, FREE).len(), usize::from(owned));
            assert_eq!(e.mem.u32(this + 0x2c), 0);
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .filter(|a| {
                    [
                        SUB_BUFFER_DESTRUCTOR,
                        POINTER_DESTRUCTOR,
                        MAGIC_HIT_EFFECT_DESTRUCTOR,
                    ]
                    .contains(a)
                })
                .collect();
            assert_eq!(
                order,
                vec![
                    SUB_BUFFER_DESTRUCTOR,
                    POINTER_DESTRUCTOR,
                    POINTER_DESTRUCTOR,
                    MAGIC_HIT_EFFECT_DESTRUCTOR
                ]
            );
        }
        // Nothing held, an empty name: no detach, no release.
        let mut e = pointer_engine();
        let this = e.mem.alloc(0x3c);
        let name = e.mem.alloc(4);
        e.mem.set_u32(this + 0x2c, name);
        e.call(0x0081_c810, &[this]);
        assert!(called(&e, WORD_18_009611E0).is_empty());
        assert!(called(&e, RELEASE_MODEL).is_empty());
    }

    struct Init {
        e: Engine,
        this: u32,
        clone: u32,
    }

    /// An effect whose target answers `target_answer` from slot +0x1d0, a file name, a
    /// clone with a controller manager whose sequence lookup answers `sequence`, and the
    /// loaded model answering `model` (0: not loaded).
    fn init_world(target_answer: u32, model: u32, sequence: u32) -> Init {
        let mut e = pointer_engine();
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x011d_e000,
            0x011f_3000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(THOUSAND, 1000.0);
        e.mem.set_f64(ZERO, 0.0);
        let target_vtable = e.mem.alloc(0x1d4);
        e.mem.set_u32(target_vtable + 0x1d0, 0x71d0);
        e.register_double(0x71d0, move |_, _| target_answer.into_ret());
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, target_vtable);
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xcc, 0x70cc);
        e.register(0x70cc, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x1c, target);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, b'm');
        e.mem.set_u32(this + 0x2c, name);
        let clone = e.mem.alloc(0x20);
        e.register_double(LOAD_FILE, move |_, _| model.into_ret());
        e.register_double(CLONE_3D, move |_, _| clone.into_ret());
        e.register(MODEL_ROOT_0043B230, |_, a| (a[0] + 1).into_ret());
        e.register(CONTROLLER_MANAGER_00653270, |_, a| {
            (a[1] + 0x100).into_ret()
        });
        e.register(STRING_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register_double(GET_SEQUENCE_0047A520, move |_, _| sequence.into_ret());
        e.register(PROCESS_SELECTIVE_UPDATE, |_, _| Ret::default());
        e.register(TIME_00825C00, |_, _| 2500u32.into_ret());
        e.register(UPDATE_BLOCK_0043D410, |_, _| Ret::default());
        e.register(UPDATE_00A59C60, |_, _| Ret::default());
        e.register(DEACTIVATE_ALL, |_, _| Ret::default());
        e.register(ACTIVATE_SEQUENCE_0047AAB0, |_, _| Ret::default());
        e.register(SET_FLAG_0047AA40, |_, _| Ret::default());
        e.register(SET_PHASE, |_, _| Ret::default());
        Init { e, this, clone }
    }

    #[test]
    fn init_attaches_the_clone_and_starts_the_sequence() {
        let mut w = init_world(1, 0, 0x5e9);
        let this = w.this;
        // pEffect with an elapsed time of 0.75 and flag 0x400 present.
        let effect = w.e.mem.alloc(8);
        w.e.mem.set_f32(effect + 4, 0.75);
        w.e.mem.set_u32(this + 0x18, effect);
        w.e.register(START_AGE_006B9130, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        w.e.register(OBJECT_007DF1D0, |_, _| 0x7df1u32.into_ret());
        w.e.register(HAS_FLAGS_00403E60, |_, _| 1u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[this]).bool());
        assert_eq!(w.e.mem.f32(this + 0x20), 0.75);
        assert_eq!(w.e.mem.u8(this + 0x24), 0);
        assert_eq!(w.e.mem.u32(this + 0x30), w.clone);
        assert_eq!(called(&w.e, HAS_FLAGS_00403E60), vec![vec![0x7df1, 0x400]]);
        let block = called(&w.e, UPDATE_BLOCK_0043D410);
        assert_eq!(block[0][1..], [2.5f32.to_bits(), 1, 0]);
        assert_eq!(called(&w.e, UPDATE_00A59C60)[0], vec![w.clone, block[0][0]]);
        let manager = w.clone + 1 + 0x100;
        assert_eq!(called(&w.e, DEACTIVATE_ALL), vec![vec![manager, 0]]);
        assert_eq!(
            called(&w.e, ACTIVATE_SEQUENCE_0047AAB0),
            vec![vec![manager, 0x5e9, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(called(&w.e, SET_FLAG_0047AA40), vec![vec![manager, 1]]);
        assert_eq!(
            called(&w.e, SET_PHASE),
            vec![vec![0x5e9, 0.75f32.to_bits(), 0]]
        );
        assert_eq!(called(&w.e, RELEASE_MODEL)[0][2..], [0, 1]);
    }

    #[test]
    fn init_without_age_skips_the_phase_and_marks_finished() {
        let mut w = init_world(1, 0, 0x5e9);
        let this = w.this;
        let effect = w.e.mem.alloc(8);
        w.e.mem.set_u32(this + 0x18, effect);
        w.e.register(START_AGE_006B9130, |_, _| 0.0f64.into_ret());
        w.e.register(OBJECT_007DF1D0, |_, _| 0x7df1u32.into_ret());
        w.e.register(HAS_FLAGS_00403E60, |_, _| 0u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[this]).bool());
        assert_eq!(w.e.mem.u8(this + 0x24), 1);
        assert!(called(&w.e, SET_PHASE).is_empty());
    }

    #[test]
    fn init_answers_false_early() {
        // The target answering 0.
        let mut w = init_world(0, 0, 0x5e9);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, LOAD_FILE).is_empty());
        // An empty file name.
        let mut w = init_world(1, 0, 0x5e9);
        let name = w.e.mem.u32(w.this + 0x2c);
        w.e.mem.set_u8(name, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, LOAD_FILE).is_empty());
        // A loaded model with a controller manager but no hit-effect sequence.
        let mut w = init_world(1, 0x6000, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, CLONE_3D).is_empty());
        // The clone's manager has no sequence.
        let mut w = init_world(1, 0, 0);
        assert!(!w.e.call(0x0081_c950, &[w.this]).bool());
        assert!(called(&w.e, DEACTIVATE_ALL).is_empty());
        assert_eq!(called(&w.e, 0x70cc).len(), 1);
    }

    #[test]
    fn init_records_third_person_for_the_player() {
        let mut w = init_world(1, 0, 0x5e9);
        let target = w.e.mem.u32(w.this + 0x1c);
        w.e.mem.set_u32(PLAYER, target);
        w.e.register(PLAYER_FLAG_004EAF60, |_, _| 1u32.into_ret());
        assert!(w.e.call(0x0081_c950, &[w.this]).bool());
        assert_eq!(w.e.mem.u8(w.this + 0x29), 1);
    }
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
    fn magic_model_hit_effect_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0081_c600, 0x0081_c810, 0x3c);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0081c5f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_c5f0, &args![0x1000u32]).u32(), 0x011d_c724);
        assert_eq!(e.call(0x0081_c5f0, &args![0u32]).u32(), 0x011d_c724);
    }
}
