//! `fallout/magic/magiccaster.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MagicCaster` (Xbox PDB, 0x0C bytes: vtable, `pLight` +4, `eState` +8)
//! is the casting side of actors and non-actor casters. Its virtual slots
//! (PC vtable `0107ac5c`, the Xbox PDB's order): +0x18 `GetTouchTarget`,
//! +0x24 `SpellCast`, +0x2C `GetCasterStatsObject`, +0x30
//! `GetMagicNode`, +0x34 `GetCurrentSpell`, +0x38 `TargetSelf`, +0x3C
//! `GetMagicEffectivenessModifier`, +0x40 `SetCurrentSpell`, +0x44
//! `GetDesiredTarget`, +0x48 `SetDesiredTarget`, +0x4C
//! `CreateActiveEffect`. The argument counts used here were checked
//! against the PC implementations (`MagicCaster`, `NonActorMagicCaster`,
//! `Actor`, `Character`, `Creature`, `PlayerCharacter`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MagicCaster`'s vtable (RTTI `.?AVMagicCaster@@`).
const MAGIC_CASTER_VTABLE: u32 = 0x0107_ac5c;
/// The `MagicLight` deleting destructor (`0080fcc0`, `this`; 1).
const MAGIC_LIGHT_DELETE: u32 = 0x0080_fcc0;
/// `__RTDynamicCast` (cdecl: object, 0, source type, target type, 0).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors.
const TYPE_MAGIC_CASTER: u32 = 0x011a_1224;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_NON_ACTOR_MAGIC_CASTER: u32 = 0x0118_4638;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
const TYPE_ENCHANTMENT_ITEM: u32 = 0x0118_3594;
const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// The word getter `0084e3a0` (`ECX` = object): the word at +0xC (a form's
/// id, an active effect's effect item, an effect item's range).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// The default listener position (`NiPoint3`, three words) passed by value.
const DEFAULT_POINT: u32 = 0x011f_426c;
/// `EffectArchetypes::Activate` (cdecl: caster, magic item, effect item,
/// source).
const EFFECT_ARCHETYPES_ACTIVATE: u32 = 0x0040_3ab0;
/// `hkpEntity::activate` (Xbox PDB name of `00c9c1d0`) and the getter of
/// the entity's motion (`009d9f40`).
const HKP_ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
const HKP_ENTITY_MOTION: u32 = 0x009d_9f40;
/// `MagicItem::GetAVEffect` (`this`; 0): the effect setting of the item's
/// actor-value effect.
const GET_AV_EFFECT: u32 = 0x0040_a300;
/// The size of the effect setting's model (`0048cee0`, `ECX` = its
/// `TESModel` at +0x18).
const MODEL_SIZE: u32 = 0x0048_cee0;
/// `ModelLoader::ReleaseModel` (`ECX` = the loader; model, 0, 1).
const MODEL_LOADER_RELEASE: u32 = 0x0044_5300;
/// The `ModelLoader*` global.
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The magic node's count (`00453470`) and its stop (`004def90`), `ECX`
/// = the node.
const MAGIC_NODE_COUNT: u32 = 0x0045_3470;
const MAGIC_NODE_STOP: u32 = 0x004d_ef90;
/// The magic node's position (`0045bb80`, `ECX` = node): `NiPoint3*`.
const MAGIC_NODE_POSITION: u32 = 0x0045_bb80;
/// `"Cannot Cast"` and `"Find Targets"` (the state's unused second word).
const TEXT_CANNOT_CAST: u32 = 0x0107_acf0;
const TEXT_FIND_TARGETS: u32 = 0x0107_acfc;
/// `"SpecialIdle_AreaEffect"`.
const TEXT_SPECIAL_IDLE_AREA_EFFECT: u32 = 0x0102_054c;

/// `FindTargets`'s callees.
const CAN_BE_POISON: u32 = 0x0040_5da0;
const HAS_TOUCH_EFFECT: u32 = 0x0040_5fe0;
const HAS_TARGET_EFFECT: u32 = 0x0040_5f30;
const GET_COSTLIEST_EFFECT: u32 = 0x0040_64d0;
const GET_ASSOCIATED_SKILL: u32 = 0x0040_65e0;
const IS_VALID_EFFECT_FOR_ENCHANTMENT: u32 = 0x0040_9720;
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
const EFFECT_ITEM_IS_HOSTILE: u32 = 0x0040_4710;
const EFFECT_ITEM_GET_RAW_COST: u32 = 0x0040_3f00;
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
const SPELL_HAS_FLAG: u32 = 0x0040_e210;
const ALCHEMY_HAS_FLAG: u32 = 0x0040_29d0;
const SOURCE_KIND: u32 = 0x0040_1170;
const ENCHANTMENT_MOVE_MODE: u32 = 0x005f_36f0;
const SETTING_VALUE: u32 = 0x0040_3e20;
const GET_SCALE: u32 = 0x0056_7400;
const SCALE_FACTOR: u32 = 0x0040_4010;
const DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
const LOCAL_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LOCAL_LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LOCAL_LIST_DESTRUCT: u32 = 0x0046_ffb0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const COLLECT_TARGETS: u32 = 0x0081_8ce0;
const SETTING_DURATION: u32 = 0x0063_9b20;
const ACTIVE_EFFECT_SET_FLAG: u32 = 0x0080_3e80;
const ACTIVE_EFFECT_ADJUST_EFFECTIVENESS: u32 = 0x0080_4bc0;
const ACTIVE_EFFECT_SET_MAGNITUDE: u32 = 0x0069_3d50;
const ACTIVE_EFFECT_SET_DURATION: u32 = 0x0080_4cb0;
const ACTIVE_EFFECT_FLAGS: u32 = 0x007d_f1d0;
const MAGNITUDE_AND_DURATION: u32 = 0x0064_82f0;
const MAGNITUDE_ONLY: u32 = 0x0064_8620;
const DURATION_ONLY: u32 = 0x0064_8810;
const AT_LEAST: u32 = 0x004b_d510;
const ALCHEMY_MAGNITUDE_SCALE: u32 = 0x008b_ebb0;
const ALCHEMY_DURATION_SCALE: u32 = 0x008b_ec80;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
const LOCATION: u32 = 0x008d_6f30;
const FIND_TARGETS_IN_AREA: u32 = 0x0081_6f10;
const DO_EXPLOSION: u32 = 0x0081_7b90;
const CHECK_WILL_KILL: u32 = 0x0081_7970;
const EXPLODE_ACTOR: u32 = 0x0081_8310;
const FIRE_PROJECTILE: u32 = 0x0081_8610;
const CHECK_ADDICTION: u32 = 0x0082_4e70;
const ANGLES: u32 = 0x0043_0830;
const ANGLE_FIRST: u32 = 0x005b_9e80;
const ANGLE_SECOND: u32 = 0x005c_53d0;
const POINT_SET: u32 = 0x0041_6870;
const START_EFFECT_SHADER: u32 = 0x0068_90b0;
const SET_IDLE: u32 = 0x0068_9920;
const SHADER_NODE: u32 = 0x0049_0e40;
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// The player reference pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// Game settings `fMagicDefaultTouchDistance`, `fMagicAreaScale`,
/// `fMagicAreaScaleMax`, `fMagicAreaScaleMin` (their values through
/// `00403e20`).
const TOUCH_DISTANCE: u32 = 0x011d_1a90;
const AREA_SCALE: u32 = 0x011d_1bbc;
const AREA_SCALE_MAX: u32 = 0x011d_1acc;
const AREA_SCALE_MIN: u32 = 0x011d_15a4;

fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// The address `00403e20` gives for a setting: the value's address.
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let p = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(p)
}

/// `ActiveEffect` flags 4, 2 and 8 (`00803e80` three times).
fn flag_secondary(e: &mut Engine, effect: u32) {
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 4u32]);
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 2u32]);
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 8u32]);
}

// Translated from 008153b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster`'s destructor body: puts back its vtable (`0107ac5c`),
/// deletes the `pLight` (+4) when there is one (`0080fcc0` with 1) and
/// clears it.
pub fn fn_008153b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAGIC_CASTER_VTABLE);
    let light = e.mem.u32(this.addr().wrapping_add(4));
    // The exe tests the word, then its copy.
    let object = light;
    if light != 0 && object != 0 {
        e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
    }
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
}

// Translated from 00815410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetCasterAsActor` (Xbox PDB): when the caster's stats
/// object (slot +0x2C) answers true to its slot +0x100 (the actor test),
/// the `Actor` the caster is part of (`this - 0x88`, null for a null
/// `this`); otherwise null.
pub fn magic_caster_get_caster_as_actor(e: &mut Engine, this: Ptr) -> Ptr {
    let stats = e.vcall(this.addr(), 0x2c, &[]).u32();
    if stats != 0 && e.vcall(stats, 0x100, &[]).bool() {
        if this.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(this.addr().wrapping_sub(0x88))
        }
    } else {
        Ptr::NULL
    }
}

// Translated from 00815600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastAbility` (Xbox PDB): for a spell whose type (its
/// `MagicItem` part at +0x18, slot +0x18) is 4, makes it the current spell
/// (`SetCurrentSpell` with that part, or null) and runs `FindTargets(0.0,
/// 0, flag, 1.0, 0)`.
pub fn magic_caster_cast_ability(e: &mut Engine, this: Ptr, spell: u32, flag: u8) {
    if e.vcall(spell.wrapping_add(0x18), 0x18, &[]).u32() == 4 {
        let item = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0x18)
        };
        e.vcall(this.addr(), 0x40, &args![item]);
        magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
    }
}

// Translated from 00815740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastSpellImmediate` (Xbox PDB): sets the desired target
/// when given (slot +0x48), the current spell (slot +0x40), and runs
/// `FindTargets(0.0, 0, flag, effectiveness, hostile_only)`.
pub fn magic_caster_cast_spell_immediate(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    flag: u8,
    target: u32,
    effectiveness: f32,
    hostile_only: u8,
) {
    if target != 0 {
        e.vcall(this.addr(), 0x48, &args![target]);
    }
    e.vcall(this.addr(), 0x40, &args![spell]);
    let effectiveness = f32::from_bits(x87_quiet_bits(effectiveness.to_bits()));
    magic_caster_find_targets(e, this, 0.0, 0, flag, effectiveness, hostile_only);
}

// Translated from 008157a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::TransferDisease` (Xbox PDB): for a spell of type 1 and a
/// non-null `target`, makes it the current spell and runs `FindTargets(0.0,
/// 0, flag, 1.0, 0)` (`target` is only tested).
pub fn magic_caster_transfer_disease(e: &mut Engine, this: Ptr, spell: u32, target: u32, flag: u8) {
    if e.vcall(spell.wrapping_add(0x18), 0x18, &[]).u32() == 1 && target != 0 {
        let item = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0x18)
        };
        e.vcall(this.addr(), 0x40, &args![item]);
        magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
    }
}

// Translated from 00815820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastWornEnchantment` (Xbox PDB): for a non-null
/// enchantment, makes it the current spell and runs `FindTargets(0.0,
/// source, flag, 1.0, 0)`.
pub fn magic_caster_cast_worn_enchantment(
    e: &mut Engine,
    this: Ptr,
    enchantment: u32,
    source: u32,
    flag: u8,
) {
    if enchantment == 0 {
        return;
    }
    e.vcall(this.addr(), 0x40, &args![enchantment]);
    magic_caster_find_targets(e, this, 0.0, source, flag, 1.0, 0);
}

// Translated from 00815b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::InterruptCast` (Xbox PDB): for an actor caster, stops the
/// magic node when it has something (`00453470`, `004def90`) and deletes
/// the light; then, with a current spell, sets state 7 (`"Cannot Cast"`),
/// reports the cast (`SpellCast`, slot +0x24, with the spell and 0),
/// releases the spell's actor-value effect model when the node was stopped,
/// and clears the current spell and desired target.
pub fn magic_caster_interrupt_cast(e: &mut Engine, this: Ptr) {
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let mut stopped = false;
    if actor != 0 {
        let caster = actor.wrapping_add(0x88);
        let node = e.vcall(caster, 0x30, &[]).u32();
        if node != 0 && e.call(MAGIC_NODE_COUNT, &args![node]).u32() != 0 {
            stopped = true;
            e.call(MAGIC_NODE_STOP, &args![node]);
        }
        let light = e.mem.u32(this.addr().wrapping_add(4));
        let object = light;
        if light != 0 && object != 0 {
            e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
        }
        e.mem.set_u32(this.addr().wrapping_add(4), 0);
    }
    if e.vcall(this.addr(), 0x34, &[]).u32() == 0 {
        return;
    }
    fn_00818af0(e, this, 7, TEXT_CANNOT_CAST);
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    e.vcall(this.addr(), 0x24, &args![spell, 0u32]);
    if stopped {
        let spell = e.vcall(this.addr(), 0x34, &[]).u32();
        let setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
        if setting != 0 && e.call(MODEL_SIZE, &args![setting.wrapping_add(0x18)]).u32() != 0 {
            let model = e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_LOADER_RELEASE, &args![loader, model, 0u32, 1u32]);
        }
    }
    e.vcall(this.addr(), 0x40, &args![0u32]);
    e.vcall(this.addr(), 0x48, &args![0u32]);
}

// Translated from 00815c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::TargetSelf` (Xbox PDB): when the caster is also a
/// `MagicTarget` (`__RTDynamicCast`), applies `effect` to itself
/// (`00817940` with the current spell, `effect` and 0, the default point
/// passed along unread); false otherwise.
pub fn magic_caster_target_self(e: &mut Engine, this: Ptr, effect: u32) -> bool {
    let target = dynamic_cast(e, this.addr(), TYPE_MAGIC_CASTER, TYPE_MAGIC_TARGET);
    if target == 0 {
        return false;
    }
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    let x = e.global::<u32>(DEFAULT_POINT);
    let y = e.global::<u32>(DEFAULT_POINT + 4);
    let z = e.global::<u32>(DEFAULT_POINT + 8);
    fn_00817940(e, this, target, x, y, z, spell, effect, 0) as u8 != 0
}

// Translated from 00815cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CreateActiveEffect` (Xbox PDB): `EffectArchetypes::
/// Activate(this, item, effect, source)` (cdecl); `EAX` is its result.
pub fn magic_caster_create_active_effect(
    e: &mut Engine,
    this: Ptr,
    item: u32,
    effect: u32,
    source: u32,
) -> u32 {
    e.call(
        EFFECT_ARCHETYPES_ACTIVATE,
        &args![this, item, effect, source],
    )
    .u32()
}

/// `FindTargetsInArea` around `center` for one active effect; returns the
/// byte the call fills (true when the area found nothing to hit).
fn find_in_area(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    effect: u32,
    center: u32,
    touch_target: u32,
    list: Ptr,
) -> bool {
    e.with_stack(4, |e, found| {
        let position = e.vcall(center, 0x1f4, &[]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position.wrapping_add(4));
        let z = e.mem.u32(position.wrapping_add(8));
        let location = e.call(LOCATION, &args![center]).u32();
        e.call(
            FIND_TARGETS_IN_AREA,
            &args![
                this,
                spell,
                effect,
                location,
                x,
                y,
                z,
                touch_target,
                0u32,
                list,
                found,
                1.0f32.to_bits()
            ],
        );
        e.mem.u8(found.addr()) != 0
    })
}

/// `FindTargets`' scaling of an alchemy effect by an actor value of the
/// caster (`scale`), with the perk entry point 0xC for value 0x10.
fn alchemy_adjust(e: &mut Engine, actor: u32, effect: u32, scale: u32, effectiveness: f32) {
    let factor = e.call(scale, &args![actor]).f64();
    let value = x87_store32(x87_mul(factor, x87_m32(effectiveness.to_bits())));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        if e.vcall(actor, 0x360, &[]).bool() {
            let item = e.call(WORD_AT_C, &args![effect]).u32();
            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() == 0x10 {
                e.call(HANDLE_ENTRY_POINT, &args![0xcu32, actor, slot]);
            }
        }
        let value = x87_quiet_bits(e.mem.u32(slot.addr()));
        e.call(ACTIVE_EFFECT_ADJUST_EFFECTIVENESS, &args![effect, value]);
    });
}

// Translated from 00815d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::FindTargets` (Xbox PDB): applies the current spell. It
/// sets state 4 (`"Find Targets"`), finds the touch target (for touch or
/// poison spells: the caster's touch target, else the desired target when
/// it is within reach: `fMagicDefaultTouchDistance`, or the caster's own
/// reach (slot +0x380) unless the player touches an actor whose slot
/// +0x214 is 9, times the scaled caster scale), collects the targets around
/// it or the actor caster (`00818ce0`), then creates an active effect for
/// each effect item (an enchantment's only when valid for the source),
/// scales it by the caster's effectiveness (plain spells, all or only
/// hostile effects), the alchemy actor values and entry points (alchemy
/// items) or the ingredient's magnitude and duration (ingredients), marks
/// the secondary effects, and applies it to the caster (self range), to
/// the touch target and its area, or to the actor caster's area (touch
/// range); then runs the explosion (with its effect shader and
/// `SpecialIdle_AreaEffect` idle, scaled by `fMagicAreaScale` within its
/// minimum and maximum) or the actor explosion of a lethal touch effect,
/// fires the projectile for target effects or advances the caster's skill
/// on a hit, releases the actor-value effect model, clears the current
/// spell, desired target and state, and checks addiction for alchemy.
/// `value` is passed on to the ingredient computations; `mark` sets
/// flags 4 and 0x10 on each effect.
// Translation incomplete: 00815d00 - SEH exception frame (FS:[0]) that destroys the local target list on unwinding is not modelled.
pub fn magic_caster_find_targets(
    e: &mut Engine,
    this: Ptr,
    value: f32,
    source: u32,
    mark: u8,
    effectiveness: f32,
    hostile_only: u8,
) {
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    if spell == 0 {
        return;
    }
    fn_00818af0(e, this, 4, TEXT_FIND_TARGETS);
    let caster_ref = e.vcall(this.addr(), 0x2c, &[]).u32();
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let mut touch_target = 0u32;
    let is_alchemy = e.vcall(spell, 0x18, &[]).u32() == 7;
    let poison = is_alchemy
        && e.call(CAN_BE_POISON, &args![spell.wrapping_add(0xc)])
            .bool();
    let alchemy = if is_alchemy {
        dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM)
    } else {
        0
    };
    if e.call(HAS_TOUCH_EFFECT, &args![spell.wrapping_add(0xc)])
        .bool()
        || poison
    {
        let mut blocked = if source != 0 {
            e.call(SOURCE_KIND, &args![source]).u32() == 0x29
        } else {
            false
        };
        if !blocked && e.vcall(spell, 0x18, &[]).u32() == 6 {
            let enchantment = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ENCHANTMENT_ITEM);
            if enchantment != 0 {
                blocked = e.call(ENCHANTMENT_MOVE_MODE, &args![enchantment]).u32() == 2;
            }
        }
        if !blocked && (actor != 0 || e.vcall(this.addr(), 0x44, &[]).u32() == 0) {
            touch_target = e.vcall(this.addr(), 0x18, &[]).u32();
        }
        if touch_target == 0 {
            touch_target = e.vcall(this.addr(), 0x44, &[]).u32();
            if touch_target != 0
                && source == 0
                && !blocked
                && e.vcall(touch_target, 0x4, &[]).u32() != 0
            {
                let mut reach = x87_quiet_bits(setting_value(e, TOUCH_DISTANCE));
                if e.vcall(caster_ref, 0x100, &[]).bool() {
                    let own_reach = if caster_ref == e.global::<u32>(PLAYER) {
                        true
                    } else {
                        let other = e.vcall(touch_target, 0x4, &[]).u32();
                        if e.vcall(other, 0x100, &[]).bool() {
                            let other = e.vcall(touch_target, 0x4, &[]).u32();
                            e.vcall(other, 0x214, &[]).u32() != 9
                        } else {
                            true
                        }
                    };
                    if own_reach {
                        reach = x87_store32(e.vcall(caster_ref, 0x380, &[]).f64());
                    }
                }
                let scale = x87_store32(e.call(GET_SCALE, &args![caster_ref]).f64());
                let factor = e.call(SCALE_FACTOR, &args![1.0f32.to_bits(), scale]).f64();
                reach = x87_store32(x87_mul(factor, x87_m32(reach)));
                let other = e.vcall(touch_target, 0x4, &[]).u32();
                let distance = e
                    .call(
                        DISTANCE_BETWEEN_FACING_ACTORS,
                        &args![caster_ref, other, 0u32],
                    )
                    .f64();
                // `FCOMPP; TEST AH,5; JP`: cleared only when below.
                if x87_m32(reach) < distance {
                    touch_target = 0;
                }
            }
        }
    }
    let mut release_model = false;
    let node = e.vcall(this.addr(), 0x30, &[]).u32();
    if node != 0 && e.call(MAGIC_NODE_COUNT, &args![node]).u32() != 0 && source == 0 {
        release_model = true;
    }
    let target_ref = if touch_target != 0 {
        e.vcall(touch_target, 0x4, &[]).u32()
    } else {
        0
    };
    let mut hit = false;
    let ingredient = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM);
    let list_of_effects = spell.wrapping_add(0xc);
    let self_effect = e
        .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 0u32, 0u32])
        .u32();
    let touch_effect = e
        .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 1u32, 0u32])
        .u32();
    let spell_item = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
    e.with_stack(8, |e, list| {
        e.call(LOCAL_LIST_CONSTRUCT, &args![list]);
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        let collect_around = |e: &mut Engine, center: u32| {
            let position = e.vcall(center, 0x1f4, &[]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position.wrapping_add(4));
            let z = e.mem.u32(position.wrapping_add(8));
            e.call(COLLECT_TARGETS, &args![this, spell, 1u32, x, y, z, list]);
        };
        if target_ref != 0 {
            collect_around(e, target_ref);
        } else if actor != 0
            && spell_item != 0
            && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool()
        {
            collect_around(e, actor);
        }
        let mut effect_node = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0xc)
        };
        while effect_node != 0 {
            let slot = e.call(NODE_ITEM, &args![effect_node.wrapping_add(4)]).u32();
            let mut effect_item = e.mem.u32(slot);
            if effect_item != 0 && e.vcall(spell, 0x18, &[]).u32() == 6 && source != 0 {
                let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
                let valid = e
                    .call(
                        IS_VALID_EFFECT_FOR_ENCHANTMENT,
                        &args![source, 0u32, setting, 0u32],
                    )
                    .bool();
                if !valid {
                    effect_item = 0;
                }
            }
            if effect_item != 0 {
                let active = e
                    .vcall(this.addr(), 0x4c, &args![spell, effect_item, source])
                    .u32();
                let active_item = e.call(WORD_AT_C, &args![active]).u32();
                let active_setting = e.call(EFFECT_ITEM_SETTING, &args![active_item]).u32();
                let duration = x87_store32(e.call(SETTING_DURATION, &args![active_setting]).f64());
                if mark != 0 {
                    e.call(ACTIVE_EFFECT_SET_FLAG, &args![active, 4u32]);
                    e.call(ACTIVE_EFFECT_SET_FLAG, &args![active, 0x10u32]);
                }
                match e.vcall(spell, 0x18, &[]).u32() {
                    0 => {
                        // The exe tests `hostile_only` a second time before
                        // `IsHostile`; that test can only pass here.
                        let apply = if hostile_only == 0 {
                            true
                        } else {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            e.call(EFFECT_ITEM_IS_HOSTILE, &args![item]).bool()
                        };
                        if apply {
                            let modifier = e
                                .vcall(this.addr(), 0x3c, &args![0u32, 0.0f32.to_bits()])
                                .f64();
                            let adjusted =
                                x87_store32(x87_mul(modifier, x87_m32(effectiveness.to_bits())));
                            e.call(ACTIVE_EFFECT_ADJUST_EFFECTIVENESS, &args![active, adjusted]);
                        }
                    }
                    7 => 'alchemy: {
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 4u32]).bool() {
                            alchemy_adjust(
                                e,
                                actor,
                                active,
                                ALCHEMY_MAGNITUDE_SCALE,
                                effectiveness,
                            );
                        }
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 2u32]).bool() {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() != 0x36 {
                                alchemy_adjust(
                                    e,
                                    actor,
                                    active,
                                    ALCHEMY_DURATION_SCALE,
                                    effectiveness,
                                );
                                break 'alchemy;
                            }
                        }
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 2u32]).bool() {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() == 0x36 {
                                let factor = e.with_stack(4, |e, slot| {
                                    e.mem.set_u32(slot.addr(), 1.0f32.to_bits());
                                    e.call(HANDLE_ENTRY_POINT, &args![0x2cu32, actor, slot]);
                                    e.mem.u32(slot.addr())
                                });
                                let adjusted = x87_store32(x87_mul(
                                    x87_m32(effectiveness.to_bits()),
                                    x87_m32(factor),
                                ));
                                e.call(
                                    ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
                                    &args![active, adjusted],
                                );
                            }
                        }
                    }
                    8 => {
                        if ingredient != 0
                            && e.vcall(ingredient.wrapping_add(0x3c), 0x4, &[]).bool()
                        {
                            e.call(
                                ACTIVE_EFFECT_SET_MAGNITUDE,
                                &args![active, 1.0f32.to_bits()],
                            );
                        } else {
                            let flags = e.call(ACTIVE_EFFECT_FLAGS, &args![active]).u32();
                            e.with_stack(8, |e, pair| {
                                // `pair` + 0 is the duration result, + 4 the
                                // magnitude result (`EBP-0x84`, `EBP-0x80`).
                                let first = pair;
                                let second = pair.byte_add(4);
                                e.mem.set_u32(first.addr(), 0.0f32.to_bits());
                                e.mem.set_u32(second.addr(), 0.0f32.to_bits());
                                let value = x87_quiet_bits(value.to_bits());
                                let has = |e: &mut Engine, bit: u32| {
                                    e.call(SETTING_HAS_FLAG, &args![flags, bit]).bool()
                                };
                                'computed: {
                                    if !has(e, 0x80) && !has(e, 0x100) {
                                        e.call(
                                            MAGNITUDE_AND_DURATION,
                                            &args![
                                                first, second, duration, value, 6u32, 0u32, 0u32,
                                                0u32, 0u32, 0u32
                                            ],
                                        );
                                        break 'computed;
                                    }
                                    if !has(e, 0x80) && has(e, 0x100) {
                                        e.call(
                                            MAGNITUDE_ONLY,
                                            &args![
                                                second, duration, value, 6u32, 0u32, 0u32, 0u32,
                                                0u32, 0u32
                                            ],
                                        );
                                        break 'computed;
                                    }
                                    if has(e, 0x80) && !has(e, 0x100) {
                                        e.call(
                                            DURATION_ONLY,
                                            &args![
                                                first, duration, value, 6u32, 0u32, 0u32, 0u32,
                                                0u32, 0u32
                                            ],
                                        );
                                    }
                                }
                                let one = 1.0f32.to_bits();
                                let magnitude = x87_quiet_bits(e.mem.u32(second.addr()));
                                let magnitude =
                                    x87_store32(e.call(AT_LEAST, &args![magnitude, one]).f64());
                                e.mem.set_u32(second.addr(), magnitude);
                                let time = x87_quiet_bits(e.mem.u32(first.addr()));
                                let time = x87_store32(e.call(AT_LEAST, &args![time, one]).f64());
                                e.mem.set_u32(first.addr(), time);
                                let magnitude = x87_quiet_bits(e.mem.u32(second.addr()));
                                e.call(ACTIVE_EFFECT_SET_MAGNITUDE, &args![active, magnitude]);
                                let time = x87_quiet_bits(e.mem.u32(first.addr()));
                                e.call(ACTIVE_EFFECT_SET_DURATION, &args![active, time]);
                            });
                        }
                    }
                    _ => {}
                }
                let range = e.call(WORD_AT_C, &args![effect_item]).u32();
                if range == 0 && !poison {
                    if effect_item != self_effect {
                        flag_secondary(e, active);
                    }
                    if e.vcall(this.addr(), 0x38, &args![active]).bool() {
                        hit = true;
                    }
                } else if e.call(WORD_AT_C, &args![effect_item]).u32() == 1 || poison {
                    let costliest = if poison { self_effect } else { touch_effect };
                    if effect_item != costliest {
                        flag_secondary(e, active);
                    }
                    if touch_target != 0 {
                        let x = e.global::<u32>(DEFAULT_POINT);
                        let y = e.global::<u32>(DEFAULT_POINT + 4);
                        let z = e.global::<u32>(DEFAULT_POINT + 8);
                        if fn_00817940(e, this, touch_target, x, y, z, spell, active, 0) as u8 != 0
                        {
                            hit = true;
                        }
                        if e.call(EFFECT_ITEM_GET_AREA, &args![effect_item]).i32() > 0
                            && target_ref != 0
                            && find_in_area(e, this, spell, active, target_ref, touch_target, list)
                        {
                            hit = false;
                        }
                    } else if !poison
                        && spell_item != 0
                        && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool()
                        && e.call(EFFECT_ITEM_GET_AREA, &args![effect_item]).i32() > 0
                        && actor != 0
                        && find_in_area(e, this, spell, active, actor, touch_target, list)
                    {
                        hit = false;
                    }
                }
                if active != 0 {
                    e.vcall(active, 0x0, &args![1u32]);
                }
            }
            if e.vcall(spell, 0x18, &[]).u32() == 8 {
                effect_node = 0;
            } else {
                let next = e.call(NODE_NEXT, &args![effect_node.wrapping_add(4)]).u32();
                effect_node = if next != 0 { next.wrapping_sub(4) } else { 0 };
            }
        }
        let av_setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
        let explosion = e
            .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 1u32, 1u32])
            .u32();
        let spell_area =
            spell_item != 0 && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool();
        let center = if target_ref != 0 {
            target_ref
        } else if actor != 0 && spell_area {
            actor
        } else {
            0
        };
        if center != 0 {
            e.with_stack(0xc, |e, point| {
                // `006815c0` run on the stack point (its result is unused).
                e.call(NODE_ITEM, &args![point]);
                let magic_node = if actor != 0 && source == 0 {
                    e.vcall(actor.wrapping_add(0x88), 0x30, &[]).u32()
                } else {
                    0
                };
                let position = if magic_node != 0 {
                    e.call(MAGIC_NODE_POSITION, &args![magic_node]).u32()
                } else {
                    e.vcall(center, 0x1f4, &[]).u32()
                };
                for i in 0..3u32 {
                    let word = e.mem.u32(position.wrapping_add(i * 4));
                    e.mem.set_u32(point.addr() + i * 4, word);
                }
                let x = e.mem.u32(point.addr());
                let y = e.mem.u32(point.addr() + 4);
                let z = e.mem.u32(point.addr() + 8);
                if explosion != 0 {
                    let location = e.call(LOCATION, &args![center]).u32();
                    let one = 1.0f32.to_bits();
                    e.call(
                        DO_EXPLOSION,
                        &args![this, x, y, z, location, spell, explosion, list, one, one],
                    );
                    if av_setting != 0
                        && e.call(MODEL_SIZE, &args![av_setting.wrapping_add(0x18)])
                            .u32()
                            != 0
                    {
                        area_effect_shader(e, center, av_setting, explosion, [x, y, z]);
                    }
                } else if e.vcall(center, 0x100, &[]).bool() && touch_effect != 0 {
                    let setting = e.call(EFFECT_ITEM_SETTING, &args![touch_effect]).u32();
                    if e.call(SETTING_HAS_FLAG, &args![setting, 0x2000_0000u32])
                        .bool()
                    {
                        let target = if center != 0 {
                            center.wrapping_add(0x94)
                        } else {
                            0
                        };
                        if e.call(CHECK_WILL_KILL, &args![this, target, spell]).bool() {
                            let cost = x87_store32(
                                e.call(EFFECT_ITEM_GET_RAW_COST, &args![touch_effect]).f64(),
                            );
                            e.call(
                                EXPLODE_ACTOR,
                                &args![this, center, x, y, z, touch_effect, cost],
                            );
                        }
                    }
                }
            });
        }
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        if e.call(HAS_TARGET_EFFECT, &args![spell.wrapping_add(0xc)])
            .bool()
        {
            e.call(FIRE_PROJECTILE, &args![this]);
        } else if hit
            && actor != 0
            && (e.vcall(spell, 0x18, &[]).u32() == 0 || e.vcall(spell, 0x18, &[]).u32() == 5)
        {
            let skill = e
                .call(GET_ASSOCIATED_SKILL, &args![spell.wrapping_add(0xc)])
                .u32();
            e.vcall(actor, 0x480, &args![skill, 1u32, 0.0f32.to_bits()]);
        }
        if release_model
            && av_setting != 0
            && e.call(MODEL_SIZE, &args![av_setting.wrapping_add(0x18)])
                .u32()
                != 0
        {
            let model = e.vcall(av_setting.wrapping_add(0x18), 0x14, &[]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_LOADER_RELEASE, &args![loader, model, 0u32, 1u32]);
        }
        e.vcall(this.addr(), 0x40, &args![0u32]);
        e.vcall(this.addr(), 0x48, &args![0u32]);
        fn_00818af0(e, this, 0, 0);
        if is_alchemy && actor != 0 {
            let alchemy = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM);
            if alchemy != 0 {
                e.call(CHECK_ADDICTION, &args![actor.wrapping_add(0x94), alchemy]);
            }
        }
        e.call(LOCAL_LIST_DESTRUCT, &args![list]);
    });
}

/// `FindTargets`' explosion effect shader: the direction from the
/// center's angles, the shader started at `position` with the effect
/// setting's model, the `SpecialIdle_AreaEffect` idle, and the shader
/// node scaled by the explosion's area times `fMagicAreaScale` within
/// `fMagicAreaScaleMin`..`fMagicAreaScaleMax`.
fn area_effect_shader(
    e: &mut Engine,
    center: u32,
    av_setting: u32,
    explosion: u32,
    position: [u32; 3],
) {
    e.with_stack(0xc, |e, direction| {
        let angles = e.call(ANGLES, &args![center]).u32();
        let negated = x87_quiet_bits(e.mem.u32(angles)) ^ 0x8000_0000;
        let first = x87_store32(e.call(ANGLE_FIRST, &args![negated]).f64());
        let angles = e.call(ANGLES, &args![center]).u32();
        let negated = x87_quiet_bits(e.mem.u32(angles)) ^ 0x8000_0000;
        let second = e.call(ANGLE_SECOND, &args![negated]).f64();
        let second = x87_store32(-second);
        e.call(
            POINT_SET,
            &args![direction, second, first, 0.0f32.to_bits()],
        );
        let model = e.vcall(av_setting.wrapping_add(0x18), 0x14, &[]).u32();
        let location = e.call(LOCATION, &args![center]).u32();
        let dx = e.mem.u32(direction.addr());
        let dy = e.mem.u32(direction.addr() + 4);
        let dz = e.mem.u32(direction.addr() + 8);
        let one = 1.0f32.to_bits();
        let shader = e
            .call(
                START_EFFECT_SHADER,
                &args![
                    location,
                    one,
                    model,
                    dx,
                    dy,
                    dz,
                    position[0],
                    position[1],
                    position[2],
                    one,
                    7u32,
                    0u32
                ],
            )
            .u32();
        e.call(SET_IDLE, &args![shader, TEXT_SPECIAL_IDLE_AREA_EFFECT]);
        if e.call(SHADER_NODE, &args![shader]).u32() != 0 {
            let area = e.call(EFFECT_ITEM_GET_AREA, &args![explosion]).i32();
            let scale = setting_value(e, AREA_SCALE);
            let mut size = x87_store32(x87_mul(x87_load32(scale), f64::from(area)));
            let maximum = setting_value(e, AREA_SCALE_MAX);
            // `FCOMPP; TEST AH,5; JP`: clamped when the maximum is below.
            if x87_load32(maximum) < x87_m32(size) {
                size = x87_quiet_bits(setting_value(e, AREA_SCALE_MAX));
            } else {
                let minimum = setting_value(e, AREA_SCALE_MIN);
                // `TEST AH,0x41; JNZ`: raised when the minimum is above.
                if x87_load32(minimum) > x87_m32(size) {
                    size = x87_quiet_bits(setting_value(e, AREA_SCALE_MIN));
                }
            }
            let node = e.call(SHADER_NODE, &args![shader]).u32();
            e.call(NODE_SET_SCALE, &args![node, size]);
        }
    });
}

// Translated from 00817940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies `effect` to `target` (a `MagicTarget`): its virtual slot 0 with
/// the caster, `spell`, `effect` and `flag`. The point passed by value
/// (three words) is not read. `EAX` is the slot's result (its `AL` is the
/// callers' bool).
pub fn fn_00817940(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    spell: u32,
    effect: u32,
    flag: u8,
) -> u32 {
    e.vcall(target, 0x0, &args![this, spell, effect, u32::from(flag)])
        .u32()
}

// Translated from 008182d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::applyLinearImpulse` (the engine map's name for this
/// body): activates the entity (`00c9c1d0`) and hands `impulse` to its
/// motion's virtual slot +0x4C (the motion from `009d9f40`).
pub fn hkp_rigid_body_apply_linear_impulse(e: &mut Engine, this: Ptr, impulse: u32) {
    e.call(HKP_ENTITY_ACTIVATE, &args![this]);
    let motion = e.call(HKP_ENTITY_MOTION, &args![this]).u32();
    e.vcall(motion, 0x4c, &args![impulse]);
}

// Translated from 00818af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the caster's state (`eState`, +8). Two words are taken (`RET 8`);
/// the second (callers pass a description string) is not read.
pub fn fn_00818af0(e: &mut Engine, this: Ptr, state: u32, _unused_2: u32) {
    e.mem.set_u32(this.addr().wrapping_add(8), state);
}

// Translated from 00818be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetMagicCasterFormID` (Xbox PDB): the form id (+0xC,
/// `0084e3a0`) of the actor caster, or of the reference a
/// `NonActorMagicCaster` holds (slot +0x2C of its part at +0xC); 0
/// otherwise.
pub fn magic_caster_get_magic_caster_form_id(e: &mut Engine, this: Ptr) -> u32 {
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let non_actor = dynamic_cast(
        e,
        this.addr(),
        TYPE_MAGIC_CASTER,
        TYPE_NON_ACTOR_MAGIC_CASTER,
    );
    if actor != 0 {
        return e.call(WORD_AT_C, &args![actor]).u32();
    }
    if non_actor != 0 && e.vcall(non_actor.wrapping_add(0xc), 0x2c, &[]).u32() != 0 {
        let reference = e.vcall(non_actor.wrapping_add(0xc), 0x2c, &[]).u32();
        return e.call(WORD_AT_C, &args![reference]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_53b0, fn_008153b0(Ptr)),
        entry!(0x0081_5410, magic_caster_get_caster_as_actor(Ptr) -> Ptr),
        entry!(0x0081_5600, magic_caster_cast_ability(Ptr, u32, u8)),
        entry!(
            0x0081_5740,
            magic_caster_cast_spell_immediate(Ptr, u32, u8, u32, f32, u8)
        ),
        entry!(
            0x0081_57a0,
            magic_caster_transfer_disease(Ptr, u32, u32, u8)
        ),
        entry!(
            0x0081_5820,
            magic_caster_cast_worn_enchantment(Ptr, u32, u32, u8)
        ),
        entry!(0x0081_5b00, magic_caster_interrupt_cast(Ptr)),
        entry!(0x0081_5c50, magic_caster_target_self(Ptr, u32) -> bool),
        entry!(
            0x0081_5cd0,
            magic_caster_create_active_effect(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x0081_5d00,
            magic_caster_find_targets(Ptr, f32, u32, u8, f32, u8)
        ),
        entry!(
            0x0081_7940,
            fn_00817940(Ptr, u32, u32, u32, u32, u32, u32, u8) -> u32
        ),
        entry!(0x0081_82d0, hkp_rigid_body_apply_linear_impulse(Ptr, u32)),
        entry!(0x0081_8af0, fn_00818af0(Ptr, u32, u32)),
        entry!(0x0081_8be0, magic_caster_get_magic_caster_form_id(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Gives `object` a vtable of 0x130 slots whose slot `offset` is
    /// `target` for each pair; other slots stay 0.
    fn vtable(e: &mut Engine, object: u32, at: u32, slots: &[(u32, u32)]) {
        let mut table = vec![0u32; 0x130];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(at, &table);
        e.mem.set_u32(object, at);
    }

    /// A double at `addr` returning `value` in `EAX`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    #[test]
    fn destructor_body_deletes_the_light() {
        let mut e = Engine::new();
        returns(&mut e, MAGIC_LIGHT_DELETE, 0);
        let caster = e.mem.alloc(0xc);
        e.mem.set_u32(caster + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0081_53b0, &args![caster]);
        assert_eq!(e.mem.u32(caster), MAGIC_CASTER_VTABLE);
        assert_eq!(e.mem.u32(caster + 4), 0);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE), vec![vec![0x5000, 1]]);
        e.call(0x0081_53b0, &args![caster]);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE).len(), 1);
    }

    #[test]
    fn caster_as_actor_needs_an_actor_stats_object() {
        let mut e = Engine::new();
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        let is_actor = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let flag = is_actor.clone();
        e.register_double(0x00a1_1000, move |_, _| ret(flag.get()));
        let caster = e.mem.alloc(0x100) + 0x88;
        vtable(&mut e, caster, 0x00a2_0000, &[(0x2c, 0x00a2_1000)]);
        returns(&mut e, 0x00a2_1000, stats);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), caster - 0x88);
        is_actor.set(0x100);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), 0);
        returns(&mut e, 0x00a2_1000, 0);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), 0);
    }

    /// A caster whose current spell (slot +0x34) is null, so `FindTargets`
    /// stops at once; slots +0x40 and +0x48 record.
    fn idle_caster() -> (Engine, u32) {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x34, 0x00a3_1034),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
            ],
        );
        returns(&mut e, 0x00a3_1034, 0);
        returns(&mut e, 0x00a3_1040, 0);
        returns(&mut e, 0x00a3_1048, 0);
        (e, caster)
    }

    /// A spell object whose `MagicItem` part (+0x18) reports `kind`.
    fn spell_of_kind(e: &mut Engine, kind: u32) -> u32 {
        let spell = e.mem.alloc(0x40);
        vtable(
            e,
            spell + 0x18,
            0x00a4_0000 + kind * 0x1000,
            &[(0x18, 0x00a5_0000 + kind)],
        );
        returns(e, 0x00a5_0000 + kind, kind);
        spell
    }

    #[test]
    fn cast_ability_only_for_kind_four() {
        let (mut e, caster) = idle_caster();
        let ability = spell_of_kind(&mut e, 4);
        let other = spell_of_kind(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x0081_5600, &args![caster, ability, 1u32]);
        e.call(0x0081_5600, &args![caster, other, 1u32]);
        assert_eq!(
            calls_to(&e, 0x00a3_1040),
            vec![vec![caster, ability + 0x18]]
        );
        // FindTargets ran once and stopped without a current spell.
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 1);
    }

    #[test]
    fn cast_spell_immediate_sets_target_and_spell() {
        let (mut e, caster) = idle_caster();
        e.call_log = Some(vec![]);
        let words = args![caster, 0x77u32, 1u32, 0x88u32, 2.0f32, 1u32];
        e.call(0x0081_5740, &words);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0x88]]);
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0x77]]);
        e.call(
            0x0081_5740,
            &args![caster, 0x77u32, 1u32, 0u32, 2.0f32, 1u32],
        );
        assert_eq!(calls_to(&e, 0x00a3_1048).len(), 1);
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 2);
    }

    #[test]
    fn transfer_disease_and_worn_enchantment() {
        let (mut e, caster) = idle_caster();
        let disease = spell_of_kind(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0081_57a0, &args![caster, disease, 0u32, 0u32]);
        assert!(calls_to(&e, 0x00a3_1040).is_empty());
        e.call(0x0081_57a0, &args![caster, disease, 0x99u32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x00a3_1040),
            vec![vec![caster, disease + 0x18]]
        );
        e.call(0x0081_5820, &args![caster, 0u32, 5u32, 0u32]);
        e.call(0x0081_5820, &args![caster, 0x66u32, 5u32, 0u32]);
        assert_eq!(calls_to(&e, 0x00a3_1040)[1], vec![caster, 0x66]);
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 2);
    }

    #[test]
    fn interrupt_cast_stops_the_node_and_releases_the_model() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x00a6_0000u32);
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        returns(&mut e, 0x00a1_1000, 1);
        let actor = e.mem.alloc(0x100);
        let caster = actor + 0x88;
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x24, 0x00a3_1024),
                (0x2c, 0x00a3_102c),
                (0x30, 0x00a3_1030),
                (0x34, 0x00a3_1034),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
            ],
        );
        returns(&mut e, 0x00a3_102c, stats);
        returns(&mut e, 0x00a3_1030, 0x00a7_0000);
        returns(&mut e, 0x00a3_1034, 0x00a8_0000);
        for slot in [0x00a3_1024, 0x00a3_1040, 0x00a3_1048] {
            returns(&mut e, slot, 0);
        }
        returns(&mut e, MAGIC_NODE_COUNT, 2);
        returns(&mut e, MAGIC_NODE_STOP, 0);
        returns(&mut e, MAGIC_LIGHT_DELETE, 0);
        let setting = e.mem.alloc(0x40);
        vtable(&mut e, setting + 0x18, 0x00a9_0000, &[(0x14, 0x00a9_1014)]);
        returns(&mut e, 0x00a9_1014, 0x00aa_0000);
        returns(&mut e, GET_AV_EFFECT, setting);
        returns(&mut e, MODEL_SIZE, 1);
        returns(&mut e, MODEL_LOADER_RELEASE, 0);
        e.mem.set_u32(caster + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0081_5b00, &args![caster]);
        assert_eq!(calls_to(&e, MAGIC_NODE_STOP), vec![vec![0x00a7_0000]]);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE), vec![vec![0x5000, 1]]);
        assert_eq!(e.mem.u32(caster + 8), 7);
        assert_eq!(
            calls_to(&e, 0x00a3_1024),
            vec![vec![caster, 0x00a8_0000, 0]]
        );
        assert_eq!(calls_to(&e, GET_AV_EFFECT), vec![vec![0x00a8_0000, 0]]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_RELEASE),
            vec![vec![0x00a6_0000, 0x00aa_0000, 0, 1]]
        );
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0]]);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0]]);
        // Without a current spell: nothing after the actor part.
        returns(&mut e, 0x00a3_1034, 0);
        e.call_log = Some(vec![]);
        magic_caster_interrupt_cast(&mut e, Ptr::new(caster));
        assert!(calls_to(&e, 0x00a3_1024).is_empty());
    }

    #[test]
    fn target_self_applies_to_the_caster_as_target() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let caster = e.mem.alloc(0xc);
        vtable(&mut e, caster, 0x00a3_0000, &[(0x34, 0x00a3_1034)]);
        returns(&mut e, 0x00a3_1034, 0x00a8_0000);
        let target = e.mem.alloc(8);
        vtable(&mut e, target, 0x00ab_0000, &[(0x0, 0x00ab_1000)]);
        returns(&mut e, 0x00ab_1000, 0x101);
        returns(&mut e, RT_DYNAMIC_CAST, target);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0081_5c50, &args![caster, 0x55u32]).bool());
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![caster, 0, TYPE_MAGIC_CASTER, TYPE_MAGIC_TARGET, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00ab_1000),
            vec![vec![target, caster, 0x00a8_0000, 0x55, 0]]
        );
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        assert!(!magic_caster_target_self(&mut e, Ptr::new(caster), 0x55));
    }

    #[test]
    fn create_active_effect_forwards_to_the_archetypes() {
        let mut e = Engine::new();
        returns(&mut e, EFFECT_ARCHETYPES_ACTIVATE, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x0081_5cd0, &args![0x10u32, 0x20u32, 0x30u32, 0x40u32]);
        assert_eq!(r.u32(), 0x1234);
        assert_eq!(
            calls_to(&e, EFFECT_ARCHETYPES_ACTIVATE),
            vec![vec![0x10, 0x20, 0x30, 0x40]]
        );
    }

    #[test]
    fn apply_to_target_skips_the_point() {
        let mut e = Engine::new();
        let target = e.mem.alloc(8);
        vtable(&mut e, target, 0x00ab_0000, &[(0x0, 0x00ab_1000)]);
        returns(&mut e, 0x00ab_1000, 0x7);
        e.call_log = Some(vec![]);
        let words = args![0x10u32, target, 1u32, 2u32, 3u32, 0x40u32, 0x50u32, 0x1ffu32];
        assert_eq!(e.call(0x0081_7940, &words).u32(), 7);
        assert_eq!(
            calls_to(&e, 0x00ab_1000),
            vec![vec![target, 0x10, 0x40, 0x50, 0xff]]
        );
    }

    #[test]
    fn linear_impulse_activates_then_hands_to_the_motion() {
        let mut e = Engine::new();
        returns(&mut e, HKP_ENTITY_ACTIVATE, 0);
        let motion = e.mem.alloc(8);
        vtable(&mut e, motion, 0x00ac_0000, &[(0x4c, 0x00ac_104c)]);
        returns(&mut e, 0x00ac_104c, 0);
        returns(&mut e, HKP_ENTITY_MOTION, motion);
        e.call_log = Some(vec![]);
        e.call(0x0081_82d0, &args![0x900u32, 0x1234u32]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0081_82d0,
                HKP_ENTITY_ACTIVATE,
                HKP_ENTITY_MOTION,
                0x00ac_104c
            ]
        );
        assert_eq!(calls_to(&e, 0x00ac_104c), vec![vec![motion, 0x1234]]);
    }

    #[test]
    fn state_setter_ignores_the_second_word() {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        e.call(0x0081_8af0, &args![caster, 7u32, 0x5555u32]);
        assert_eq!(e.mem.u32(caster + 8), 7);
        assert_eq!(e.mem.u32(caster + 4), 0);
    }

    #[test]
    fn caster_form_id_of_actor_or_non_actor() {
        let mut e = Engine::new();
        returns(&mut e, WORD_AT_C, 0x0001_4000);
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        let is_actor = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let flag = is_actor.clone();
        e.register_double(0x00a1_1000, move |_, _| ret(flag.get()));
        let caster = e.mem.alloc(0x100) + 0x88;
        vtable(&mut e, caster, 0x00a2_0000, &[(0x2c, 0x00a2_1000)]);
        returns(&mut e, 0x00a2_1000, stats);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_8be0, &args![caster]).u32(), 0x0001_4000);
        assert_eq!(calls_to(&e, WORD_AT_C), vec![vec![caster - 0x88]]);
        // A non-actor caster with a reference.
        is_actor.set(0);
        let non_actor = e.mem.alloc(0x20);
        vtable(&mut e, non_actor + 0xc, 0x00ad_0000, &[(0x2c, 0x00ad_102c)]);
        returns(&mut e, 0x00ad_102c, 0x00ae_0000);
        returns(&mut e, RT_DYNAMIC_CAST, non_actor);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_8be0, &args![caster]).u32(), 0x0001_4000);
        assert_eq!(calls_to(&e, WORD_AT_C), vec![vec![0x00ae_0000]]);
        // No reference: 0.
        returns(&mut e, 0x00ad_102c, 0);
        assert_eq!(
            magic_caster_get_magic_caster_form_id(&mut e, Ptr::new(caster)),
            0
        );
    }

    /// A non-actor caster casting a plain spell (kind 0) of one self-range
    /// effect: the effect is created, its effectiveness scaled by the
    /// caster's modifier, applied to the caster, and deleted; the state and
    /// current spell are cleared.
    #[test]
    fn find_targets_applies_a_self_effect() {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        let stats = e.mem.alloc(8);
        let spell = e.mem.alloc(0x40);
        let effect = e.mem.alloc(0x20);
        let active = e.mem.alloc(0x40);
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x2c, 0x00a3_102c),
                (0x30, 0x00a3_1030),
                (0x34, 0x00a3_1034),
                (0x38, 0x00a3_1038),
                (0x3c, 0x00a3_103c),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
                (0x4c, 0x00a3_104c),
            ],
        );
        returns(&mut e, 0x00a3_102c, stats);
        returns(&mut e, 0x00a3_1030, 0);
        returns(&mut e, 0x00a3_1034, spell);
        returns(&mut e, 0x00a3_1038, 1);
        e.register(0x00a3_103c, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        returns(&mut e, 0x00a3_1040, 0);
        returns(&mut e, 0x00a3_1048, 0);
        returns(&mut e, 0x00a3_104c, active);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        returns(&mut e, 0x00a1_1000, 0);
        vtable(&mut e, spell, 0x00a4_0000, &[(0x18, 0x00a4_1018)]);
        returns(&mut e, 0x00a4_1018, 0);
        vtable(&mut e, active, 0x00af_0000, &[(0x0, 0x00af_1000)]);
        returns(&mut e, 0x00af_1000, 0);
        // The spell's effect list: one node at +0x10 holding `effect`.
        e.mem.set_u32(spell + 0x10, effect);
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.mem.set_u32(active + 0xc, 0x00b0_0000);
        returns(&mut e, EFFECT_ITEM_SETTING, 0x00b1_0000);
        e.register(SETTING_DURATION, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        returns(&mut e, HAS_TOUCH_EFFECT, 0);
        returns(&mut e, HAS_TARGET_EFFECT, 0);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        e.register(GET_COSTLIEST_EFFECT, |e, a| {
            // The costliest self effect is the one effect.
            if a[1] == 0 {
                ret(e.mem.u32(a[0] + 4))
            } else {
                ret(0)
            }
        });
        for callee in [
            LOCAL_LIST_CONSTRUCT,
            LOCAL_LIST_REMOVE_ALL,
            LOCAL_LIST_DESTRUCT,
            ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
            GET_AV_EFFECT,
        ] {
            returns(&mut e, callee, 0);
        }
        e.call_log = Some(vec![]);
        let words = args![caster, 0.0f32, 0u32, 0u32, 2.0f32, 0u32];
        e.call(0x0081_5d00, &words);
        assert_eq!(
            calls_to(&e, 0x00a3_104c),
            vec![vec![caster, spell, effect, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00a3_103c),
            vec![vec![caster, 0, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTIVE_EFFECT_ADJUST_EFFECTIVENESS),
            vec![vec![active, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x00a3_1038), vec![vec![caster, active]]);
        assert_eq!(calls_to(&e, 0x00af_1000), vec![vec![active, 1]]);
        assert!(calls_to(&e, ACTIVE_EFFECT_SET_FLAG).is_empty());
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0]]);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0]]);
        assert_eq!(e.mem.u32(caster + 8), 0);
        assert_eq!(calls_to(&e, LOCAL_LIST_DESTRUCT).len(), 1);
        // Without a current spell nothing else happens.
        returns(&mut e, 0x00a3_1034, 0);
        e.call_log = Some(vec![]);
        magic_caster_find_targets(&mut e, Ptr::new(caster), 0.0, 0, 0, 1.0, 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
