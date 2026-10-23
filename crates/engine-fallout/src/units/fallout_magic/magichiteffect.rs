//! `fallout/magic/magichiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MagicHitEffect` (Xbox PDB), 0x28 bytes on both builds (the scalar deleting
    /// destructor frees 0x28): a `BSTempEffect` (0x18) and the effect it shows.
    pub struct MagicHitEffect: 0x28 {
        /// `BSTempEffect::fLifetime` (Xbox PDB).
        0x08 fLifetime: u32,
        /// `BSTempEffect::pCell` (Xbox PDB): `TESObjectCELL*`.
        0x0C pCell: u32,
        /// `pEffect` (Xbox PDB): `ActiveEffect*`.
        0x18 pEffect: u32,
        /// `pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x1C pTarget: u32,
        /// `fAge` (Xbox PDB).
        0x20 fAge: u32,
        /// `bFinished` (Xbox PDB).
        0x24 bFinished: u8,
    }
}

/// `MagicHitEffect`'s vtable (`0107b0ec`).
const MAGIC_HIT_EFFECT_VTABLE: u32 = 0x0107_b0ec;
/// The `BSTempEffect` constructor `006827b0` (`thiscall`, two words: a word and a
/// `float`).
const BS_TEMP_EFFECT_CONSTRUCTOR: u32 = 0x0068_27b0;
/// The `BSTempEffect` destructor body `00682860` (`thiscall`).
const BS_TEMP_EFFECT_DESTRUCTOR: u32 = 0x0068_2860;
/// `008d6f30` (`thiscall`): the reference's cell, stored as `pCell`.
const REFERENCE_CELL_008D6F30: u32 = 0x008d_6f30;
/// The `float` the constructor gives `fLifetime` (`01016970`).
const DEFAULT_LIFETIME: u32 = 0x0101_6970;
/// `00905330` (`thiscall`, one word: the address of a stack word).
const REMOVE_HIT_EFFECT_00905330: u32 = 0x0090_5330;
/// `0059bb30` (`thiscall`): the active effect's magic target (the engine map's name for
/// the folded body, `D3DTexture_LockRect`, does not fit).
const GET_MAGIC_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`, `thiscall`).
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `004181e0` (`thiscall`): the reference's base object.
const BASE_OBJECT_004181E0: u32 = 0x0041_81e0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of the two casts: from `011846e8` to `01183a00`, and from
/// `011846d4` to `011a0f14` (class names not read here).
const CAST_SOURCE_011846E8: u32 = 0x0118_46e8;
const CAST_TARGET_01183A00: u32 = 0x0118_3a00;
const CAST_SOURCE_011846D4: u32 = 0x0118_46d4;
const CAST_TARGET_011A0F14: u32 = 0x011a_0f14;
/// `TESCreature::IsHumanoidCreature` (engine map, `005fbf20`, `thiscall`).
const IS_HUMANOID_CREATURE: u32 = 0x005f_bf20;
/// `007df1d0` (`thiscall`): an object of the active effect `00403e60` is asked about.
const EFFECT_OBJECT_007DF1D0: u32 = 0x007d_f1d0;
/// `00403e60` (`thiscall`, one word: a flag mask).
const HAS_FLAGS_00403E60: u32 = 0x0040_3e60;
/// `00408d60` (`thiscall`) on the object at `011df7f8`: answers the address of a byte.
const BYTE_ADDRESS_00408D60: u32 = 0x0040_8d60;
const OBJECT_011DF7F8: u32 = 0x011d_f7f8;
/// `00804b80` (`thiscall`, one word).
const ACTIVE_EFFECT_TEST_00804B80: u32 = 0x0080_4b80;
/// `00525420` (no words): a byte.
const TEST_00525420: u32 = 0x0052_5420;
/// The allocators `00aa13e0` and `00401000` (`cdecl`, one word: the size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const ALLOCATE_00401000: u32 = 0x0040_1000;
/// `MagicModelHitEffect::MagicModelHitEffect` (engine map, `0081c630`, `thiscall`, two
/// words: the target reference and the active effect).
const MODEL_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_c630;
/// The 0x6c-byte hit effect constructor `0081f350` (`magicshaderhiteffect.cpp`,
/// `thiscall`, two words: the target reference and the active effect).
const SHADER_HIT_EFFECT_CONSTRUCTOR_0081F350: u32 = 0x0081_f350;
/// `ProcessLists::AddTempEffect` (engine map, `00973fd0`, `thiscall`, one word) on the
/// process lists at `011e0e80`.
const ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `00470440` (`thiscall`, one word: the address of the item): makes a one-item list node.
const LIST_NODE_CONSTRUCTOR_00470440: u32 = 0x0047_0440;
/// `005ae3d0` (`thiscall`, one word: the address of the item): adds an item to a list.
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;

// Translated from 0081b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicHitEffect::MagicHitEffect` (engine map; `thiscall`, `RET 8`: the target reference
/// and the active effect): constructs the `BSTempEffect` base (`006827b0` with 0 and 0.0),
/// stores the vtable and `pTarget`, takes `pCell` from the target (`008d6f30`) when there is
/// one, stores `pEffect`, sets `fAge` to 0.0, `fLifetime` to the float at `01016970`
/// (`FLD`/`FSTP`) and clears `bFinished`. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081b700 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_hit_effect_constructor(
    e: &mut Engine,
    this: Ptr<MagicHitEffect>,
    target: u32,
    effect: u32,
) -> Ptr<MagicHitEffect> {
    e.call(BS_TEMP_EFFECT_CONSTRUCTOR, &args![this, 0u32, 0.0f32]);
    e.mem.set_u32(this.addr(), MAGIC_HIT_EFFECT_VTABLE);
    e.set(this, MagicHitEffect::pTarget, target);
    if e.get(this, MagicHitEffect::pTarget) != 0 {
        let target = e.get(this, MagicHitEffect::pTarget);
        let cell = e.call(REFERENCE_CELL_008D6F30, &args![target]).u32();
        e.set(this, MagicHitEffect::pCell, cell);
    }
    e.set(this, MagicHitEffect::pEffect, effect);
    e.set(this, MagicHitEffect::fAge, 0);
    let lifetime = x87_quiet_bits(e.mem.u32(DEFAULT_LIFETIME));
    e.set(this, MagicHitEffect::fLifetime, lifetime);
    e.set(this, MagicHitEffect::bFinished, 0);
    this
}

// Translated from 0081b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicHitEffect::~MagicHitEffect` (engine map; `thiscall`): stores the vtable, clears
/// `pTarget`, takes this effect off its active effect when there is one
/// ([`fn_0081b830`] on `pEffect` with `this`), clears `pEffect` and runs the
/// `BSTempEffect` destructor (`00682860`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081b7b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_hit_effect_destructor(e: &mut Engine, this: Ptr<MagicHitEffect>) {
    e.mem.set_u32(this.addr(), MAGIC_HIT_EFFECT_VTABLE);
    e.set(this, MagicHitEffect::pTarget, 0);
    let effect = e.get(this, MagicHitEffect::pEffect);
    if effect != 0 {
        fn_0081b830(e, Ptr::new(effect), this.addr());
    }
    e.set(this, MagicHitEffect::pEffect, 0);
    e.call(BS_TEMP_EFFECT_DESTRUCTOR, &args![this]);
}

// Translated from 0081b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a hit effect): when the active
/// effect's `pHitEffects` list (+0x40) exists, calls `00905330` on it with the address of
/// the stack word holding the hit effect.
pub fn fn_0081b830(e: &mut Engine, this: Ptr, hit_effect: u32) {
    let list = e.mem.u32(this.addr() + 0x40);
    if list == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), hit_effect);
        let list = e.mem.u32(this.addr() + 0x40);
        e.call(REMOVE_HIT_EFFECT_00905330, &args![list, slot]);
    });
}

/// Makes one hit effect: `size` bytes from `00aa13e0`, constructed by `constructor` with
/// the target reference (the magic target's slot +4) and `effect`; null when the
/// allocation fails.
fn new_hit_effect(e: &mut Engine, effect: u32, size: u32, constructor: u32) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let target = e.call(GET_MAGIC_TARGET_0059BB30, &args![effect]).u32();
    let reference = e.vcall(target, 4, &args![]).u32();
    e.call(constructor, &args![block, reference, effect]).u32()
}

/// A one-item list node (8 bytes from `00401000`, `00470440` with the item's address);
/// null when the allocation fails.
fn new_list(e: &mut Engine, item: Ptr) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![8u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(LIST_NODE_CONSTRUCTOR_00470440, &args![block, item])
        .u32()
}

// Translated from 0081b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicHitEffect::InstantiateHitEffects` (engine map; `cdecl`, one word: the active
/// effect). Makes the effect's hit effects and returns the list holding them (null when
/// none was made):
///
/// - When the effect's target (`0059bb30`, `MagicTarget::GetTargetAsActor`) is an actor:
///   its base object (`004181e0`) is cast (`__RTDynamicCast` from `011846e8` to
///   `01183a00`) and the actor itself (from `011846d4` to `011a0f14`); when the first cast
///   fails or `TESCreature::IsHumanoidCreature` answers false, a successful second cast
///   gets its slot +0x218.
/// - When the effect's object (`007df1d0`) has flag 0x1000 (`00403e60`) and the byte the
///   object at `011df7f8` points to (`00408d60`) is non-zero, nothing is made.
/// - Otherwise, unless `00804b80(effect, 4)` or `00525420` answers true, a
///   `MagicModelHitEffect` (0x3c bytes) is made; then, unless `00804b80(effect, 2)`
///   answers true, the 0x6c-byte hit effect of `0081f350`. Each new effect whose slot
///   +0xc4 answers true is added to the process lists (`ProcessLists::AddTempEffect`) and
///   to the list (a new node from `00470440` for the first, `005ae3d0` after); one that
///   answers false is deleted (slot 0 with 1).
///
/// The new effect is kept in a stack word whose address the list functions get.
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081b8e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn magic_hit_effect_instantiate_hit_effects(e: &mut Engine, effect: u32) -> u32 {
    let target = e.call(GET_MAGIC_TARGET_0059BB30, &args![effect]).u32();
    if e.call(GET_TARGET_AS_ACTOR, &args![target]).u32() != 0 {
        let target = e.call(GET_MAGIC_TARGET_0059BB30, &args![effect]).u32();
        let actor = e.call(GET_TARGET_AS_ACTOR, &args![target]).u32();
        let base = e.call(BASE_OBJECT_004181E0, &args![actor]).u32();
        let creature = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, CAST_SOURCE_011846E8, CAST_TARGET_01183A00, 0u32],
            )
            .u32();
        let target = e.call(GET_MAGIC_TARGET_0059BB30, &args![effect]).u32();
        let actor = e.call(GET_TARGET_AS_ACTOR, &args![target]).u32();
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![
                    actor,
                    0u32,
                    CAST_SOURCE_011846D4,
                    CAST_TARGET_011A0F14,
                    0u32
                ],
            )
            .u32();
        let humanoid = creature != 0 && e.call(IS_HUMANOID_CREATURE, &args![creature]).u8() != 0;
        if !humanoid && cast != 0 {
            e.vcall(cast, 0x218, &args![]);
        }
    }

    e.with_stack(4, |e, item| {
        let mut list = 0u32;
        let object = e.call(EFFECT_OBJECT_007DF1D0, &args![effect]).u32();
        let skip = e.call(HAS_FLAGS_00403E60, &args![object, 0x1000u32]).u8() != 0 && {
            let byte = e.call(BYTE_ADDRESS_00408D60, &args![OBJECT_011DF7F8]).u32();
            e.mem.u8(byte) != 0
        };
        if skip {
            return list;
        }
        if e.call(ACTIVE_EFFECT_TEST_00804B80, &args![effect, 4u32])
            .u8()
            == 0
            && e.call(TEST_00525420, &args![]).u8() == 0
        {
            let made = new_hit_effect(e, effect, 0x3c, MODEL_HIT_EFFECT_CONSTRUCTOR);
            e.mem.set_u32(item.addr(), made);
            if e.vcall(made, 0xc4, &args![]).u8() != 0 {
                let made = e.mem.u32(item.addr());
                e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, made]);
                list = new_list(e, item);
            } else {
                let made = e.mem.u32(item.addr());
                if made != 0 {
                    e.vcall(made, 0, &args![1u32]);
                }
            }
        }
        if e.call(ACTIVE_EFFECT_TEST_00804B80, &args![effect, 2u32])
            .u8()
            == 0
        {
            let made = new_hit_effect(e, effect, 0x6c, SHADER_HIT_EFFECT_CONSTRUCTOR_0081F350);
            e.mem.set_u32(item.addr(), made);
            if e.vcall(made, 0xc4, &args![]).u8() != 0 {
                let made = e.mem.u32(item.addr());
                e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, made]);
                if list != 0 {
                    e.call(LIST_ADD_005AE3D0, &args![list, item]);
                } else {
                    list = new_list(e, item);
                }
            } else {
                let made = e.mem.u32(item.addr());
                if made != 0 {
                    e.vcall(made, 0, &args![1u32]);
                }
            }
        }
        list
    })
}

// Translated from 0081bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicHitEffect::ClearTarget` (engine map; `thiscall`): clears `pTarget`.
pub fn magic_hit_effect_clear_target(e: &mut Engine, this: Ptr<MagicHitEffect>) {
    e.set(this, MagicHitEffect::pTarget, 0);
}

// Translated from 0081b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicHitEffect` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `0081b7b0` and frees 0x28 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn magic_hit_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0081_b7b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x28u32]);
    }
    this
}

// Translated from 0081b6b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011dc6b4 without reading the receiver. The map names this body `MagicHitEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0081b6b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_c6b4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_b6b0, fn_0081b6b0(Ptr) -> u32),
        entry!(0x0081_b6d0, magic_hit_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0081_b700,
            magic_hit_effect_constructor(Ptr<MagicHitEffect>, u32, u32) -> Ptr<MagicHitEffect>
        ),
        entry!(
            0x0081_b7b0,
            magic_hit_effect_destructor(Ptr<MagicHitEffect>)
        ),
        entry!(0x0081_b830, fn_0081b830(Ptr, u32)),
        entry!(0x0081_b8e0, magic_hit_effect_instantiate_hit_effects(u32) -> u32),
        entry!(
            0x0081_bc50,
            magic_hit_effect_clear_target(Ptr<MagicHitEffect>)
        ),
    ]
}

#[cfg(test)]
mod hit_effect_tests {
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

    #[test]
    fn constructor_sets_the_fields() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(DEFAULT_LIFETIME, 0x7f80_0003);
        e.register(BS_TEMP_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0], &[0xee; 0x28]);
            Ret::default()
        });
        e.register(REFERENCE_CELL_008D6F30, |_, a| (a[0] + 0x100).into_ret());
        let this = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_b700, &[this, 0x50, 0x60]).u32(), this);
        assert_eq!(
            called(&e, BS_TEMP_EFFECT_CONSTRUCTOR),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c), 0x50);
        assert_eq!(e.mem.u32(this + 0xc), 0x150);
        assert_eq!(e.mem.u32(this + 0x18), 0x60);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0003);
        assert_eq!(e.mem.u8(this + 0x24), 0);

        // Without a target the cell is left as the base made it.
        e.call_log = Some(vec![]);
        e.call(0x0081_b700, &[this, 0, 0x60]);
        assert!(called(&e, REFERENCE_CELL_008D6F30).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0xeeee_eeee);
    }

    #[test]
    fn destructor_detaches_from_the_effect() {
        let mut e = Engine::new();
        e.register(BS_TEMP_EFFECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(REMOVE_HIT_EFFECT_00905330, |e, a| {
            // The word at the address is the hit effect.
            e.mem.u32(a[1]).into_ret()
        });
        let effect = e.mem.alloc(0x48);
        e.mem.set_u32(effect + 0x40, 0x4040);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x18, effect);
        e.mem.set_u32(this + 0x1c, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0081_b7b0, &[this]);
        assert_eq!(e.mem.u32(this), MAGIC_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        let removes = called(&e, REMOVE_HIT_EFFECT_00905330);
        assert_eq!(removes[0][0], 0x4040);
        assert_eq!(called(&e, BS_TEMP_EFFECT_DESTRUCTOR), vec![vec![this]]);

        // No effect: no removal.
        e.call_log = Some(vec![]);
        e.call(0x0081_b7b0, &[this]);
        assert!(called(&e, REMOVE_HIT_EFFECT_00905330).is_empty());
    }

    #[test]
    fn remove_from_effect_passes_the_word_address() {
        let mut e = Engine::new();
        e.register(REMOVE_HIT_EFFECT_00905330, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        let effect = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x0081_b830, &[effect, 0x1234]);
        assert!(called(&e, REMOVE_HIT_EFFECT_00905330).is_empty());
        e.mem.set_u32(effect + 0x40, 0x4040);
        e.call(0x0081_b830, &[effect, 0x1234]);
        assert_eq!(called(&e, REMOVE_HIT_EFFECT_00905330).len(), 1);
    }

    #[test]
    fn clear_target_clears_only_the_target() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.write(this, &[0xab; 0x28]);
        e.call(0x0081_bc50, &[this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0xabab_abab);
        assert_eq!(e.mem.u32(this + 0x20), 0xabab_abab);
    }

    /// The instantiate callees: no actor target, the skip check false, `00804b80` and
    /// `00525420` as given, effects made from fresh blocks whose slot +0xc4 answers
    /// `keep` and whose slot 0 is a recorded delete.
    fn instantiate_engine(test4: u32, test2: u32, keep: u32) -> Engine {
        let mut e = Engine::new();
        e.register(GET_MAGIC_TARGET_0059BB30, |_, _| 0x2000u32.into_ret());
        e.register(GET_TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        e.register(EFFECT_OBJECT_007DF1D0, |_, _| 0x3000u32.into_ret());
        e.register(HAS_FLAGS_00403E60, |_, _| 0u32.into_ret());
        e.register_double(ACTIVE_EFFECT_TEST_00804B80, move |_, a| {
            if a[1] == 4 { test4 } else { test2 }.into_ret()
        });
        e.register(TEST_00525420, |_, _| 0u32.into_ret());
        let vtable = e.mem.alloc(0xc8);
        e.mem.set_u32(vtable, 0x6000);
        e.mem.set_u32(vtable + 0xc4, 0x60c4);
        e.register(0x6000, |_, _| Ret::default());
        e.register_double(0x60c4, move |_, _| keep.into_ret());
        e.register_double(ALLOCATE_00AA13E0, move |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u32(block, vtable);
            block.into_ret()
        });
        // The magic target's slot +4: the reference.
        let target_vtable = e.mem.alloc(8);
        e.mem.set_u32(target_vtable + 4, 0x7004);
        e.register(0x7004, |_, _| 0x4444u32.into_ret());
        e.map(0x0000_2000, 0x10);
        e.mem.set_u32(0x2000, target_vtable);
        e.register(MODEL_HIT_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SHADER_HIT_EFFECT_CONSTRUCTOR_0081F350, |_, a| {
            a[0].into_ret()
        });
        e.register(ADD_TEMP_EFFECT, |_, _| Ret::default());
        e.register(ALLOCATE_00401000, |_, _| 0x8000u32.into_ret());
        e.register(LIST_NODE_CONSTRUCTOR_00470440, |_, a| a[0].into_ret());
        e.register(LIST_ADD_005AE3D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn instantiate_makes_both_effects_and_lists_them() {
        let mut e = instantiate_engine(0, 0, 1);
        assert_eq!(e.call(0x0081_b8e0, &[0x1111]).u32(), 0x8000);
        let model = called(&e, MODEL_HIT_EFFECT_CONSTRUCTOR);
        assert_eq!(model[0][1..], [0x4444, 0x1111]);
        let shader = called(&e, SHADER_HIT_EFFECT_CONSTRUCTOR_0081F350);
        assert_eq!(shader[0][1..], [0x4444, 0x1111]);
        let added: Vec<u32> = called(&e, ADD_TEMP_EFFECT).iter().map(|w| w[1]).collect();
        assert_eq!(added, vec![model[0][0], shader[0][0]]);
        assert_eq!(called(&e, LIST_NODE_CONSTRUCTOR_00470440).len(), 1);
        assert_eq!(called(&e, LIST_ADD_005AE3D0)[0][0], 0x8000);
        let sizes: Vec<u32> = called(&e, ALLOCATE_00AA13E0).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x3c, 0x6c]);
    }

    #[test]
    fn instantiate_deletes_unwanted_effects_and_honours_the_tests() {
        // Effects answering false are deleted; no list.
        let mut e = instantiate_engine(0, 0, 0);
        assert_eq!(e.call(0x0081_b8e0, &[0x1111]).u32(), 0);
        assert_eq!(called(&e, 0x6000).len(), 2);
        assert!(called(&e, ADD_TEMP_EFFECT).is_empty());

        // Test 4 true: only the shader effect, which starts the list.
        let mut e = instantiate_engine(1, 0, 1);
        assert_eq!(e.call(0x0081_b8e0, &[0x1111]).u32(), 0x8000);
        assert!(called(&e, MODEL_HIT_EFFECT_CONSTRUCTOR).is_empty());
        assert_eq!(called(&e, LIST_NODE_CONSTRUCTOR_00470440).len(), 1);
        assert!(called(&e, LIST_ADD_005AE3D0).is_empty());

        // Test 2 true: only the model effect.
        let mut e = instantiate_engine(0, 1, 1);
        assert_eq!(e.call(0x0081_b8e0, &[0x1111]).u32(), 0x8000);
        assert!(called(&e, SHADER_HIT_EFFECT_CONSTRUCTOR_0081F350).is_empty());
    }

    #[test]
    fn instantiate_skips_everything_when_flagged() {
        let mut e = instantiate_engine(0, 0, 1);
        e.register(HAS_FLAGS_00403E60, |_, _| 1u32.into_ret());
        e.map(0x0000_9000, 0x10);
        e.mem.set_u8(0x9000, 1);
        e.register(BYTE_ADDRESS_00408D60, |_, _| 0x9000u32.into_ret());
        assert_eq!(e.call(0x0081_b8e0, &[0x1111]).u32(), 0);
        assert!(called(&e, ALLOCATE_00AA13E0).is_empty());
        assert_eq!(called(&e, HAS_FLAGS_00403E60), vec![vec![0x3000, 0x1000]]);
        // A zero byte does not skip.
        e.mem.set_u8(0x9000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0081_b8e0, &[0x1111]);
        assert_eq!(called(&e, ALLOCATE_00AA13E0).len(), 2);
    }

    #[test]
    fn instantiate_lets_non_humanoid_actors_react() {
        for (creature, humanoid, cast, reacts) in [
            (0u32, 0u32, 0x5000u32, true),
            (0x5100, 0, 0x5000, true),
            (0x5100, 1, 0x5000, false),
            (0, 0, 0, false),
        ] {
            let mut e = instantiate_engine(1, 1, 0);
            e.register(GET_TARGET_AS_ACTOR, |_, _| 0x5555u32.into_ret());
            e.register(BASE_OBJECT_004181E0, |_, _| 0x5600u32.into_ret());
            e.register_double(DYNAMIC_CAST, move |_, a| {
                if a[2] == CAST_SOURCE_011846E8 {
                    creature
                } else {
                    cast
                }
                .into_ret()
            });
            e.register_double(IS_HUMANOID_CREATURE, move |_, _| humanoid.into_ret());
            let vtable = e.mem.alloc(0x21c);
            e.mem.set_u32(vtable + 0x218, 0x5218);
            e.register(0x5218, |_, _| Ret::default());
            e.map(0x0000_5000, 0x10);
            e.mem.set_u32(0x5000, vtable);
            e.call(0x0081_b8e0, &[0x1111]);
            let casts = called(&e, DYNAMIC_CAST);
            assert_eq!(
                casts[0],
                vec![0x5600, 0, CAST_SOURCE_011846E8, CAST_TARGET_01183A00, 0]
            );
            assert_eq!(
                casts[1],
                vec![0x5555, 0, CAST_SOURCE_011846D4, CAST_TARGET_011A0F14, 0]
            );
            assert_eq!(!called(&e, 0x5218).is_empty(), reacts);
        }
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
    fn magic_hit_effect_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0081_b6d0, 0x0081_b7b0, 0x28);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0081b6b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_b6b0, &args![0x1000u32]).u32(), 0x011d_c6b4);
        assert_eq!(e.call(0x0081_b6b0, &args![0u32]).u32(), 0x011d_c6b4);
    }
}
