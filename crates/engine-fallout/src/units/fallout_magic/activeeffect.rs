//! `fallout/magic/activeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00804b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`activeeffect.cpp`). `cdecl`, one stack word: true (`AL`) when it
/// is 1, 4 or 10, false otherwise. The meaning of those values is unconfirmed.
pub fn fn_00804b30(_e: &mut Engine, value: i32) -> bool {
    value == 1 || value == 4 || value == 10
}

layout! {
    /// `ActiveEffect` (Xbox PDB), 0x48 bytes there; the PC offsets below are
    /// the ones `CopyData` (`00804090`) and `SaveGame` (`00806230`) use.
    pub struct ActiveEffect: 0x48 {
        /// `fElapsedSeconds` (Xbox PDB).
        0x04 fElapsedSeconds: f32,
        /// `pSpell` (Xbox PDB): `MagicItem*`.
        0x08 pSpell: u32,
        /// `pEffect` (Xbox PDB): `EffectItem*`.
        0x0C pEffect: u32,
        /// `bActive` (Xbox PDB).
        0x10 bActive: u8,
        /// `bStarted` (Xbox PDB).
        0x11 bStarted: u8,
        /// `bFinished` (Xbox PDB).
        0x12 bFinished: u8,
        /// `bDone` (Xbox PDB).
        0x13 bDone: u8,
        /// `bWornEnchantment` (Xbox PDB).
        0x14 bWornEnchantment: u8,
        /// `iFlags` (Xbox PDB).
        0x18 iFlags: u32,
        /// `fMagnitude` (Xbox PDB).
        0x1C fMagnitude: f32,
        /// `fDuration` (Xbox PDB).
        0x20 fDuration: f32,
        /// `pTarget` (Xbox PDB): `MagicTarget*`.
        0x24 pTarget: u32,
        /// `pCaster` (Xbox PDB): `MagicCaster*`.
        0x28 pCaster: u32,
        /// `eSpellType` (Xbox PDB).
        0x2C eSpellType: u32,
        /// `pSource` (Xbox PDB): `TESBoundObject*`.
        0x3C pSource: u32,
        /// `pHitEffects` (Xbox PDB): `BSSimpleList<MagicHitEffect*>*`.
        0x40 pHitEffects: u32,
        /// `pDisplacementSpell` (Xbox PDB): `MagicItem*`.
        0x44 pDisplacementSpell: u32,
    }
}

/// `PersistentSound` (Xbox PDB): the effect's `BSSoundHandle` at +0x30.
const PERSISTENT_SOUND: u32 = 0x30;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `005ae3d0` (thiscall on the list, one word: the address of the item; unnamed in
/// the map): the list's insert in front of the head node.
const LIST_INSERT_FRONT_005AE3D0: u32 = 0x005a_e3d0;
/// `00470440` (thiscall on a new 8-byte node, one word: the address of the item;
/// unnamed): the node constructor.
const LIST_NODE_INIT_00470440: u32 = 0x0047_0440;
/// `007df210` (thiscall, returns `AL`; unnamed, filed under `statsmenu.cpp`).
const EFFECT_TEST_007DF210: u32 = 0x007d_f210;
/// `008041a0` (thiscall, returns a word; the map's name
/// `LowProcess::GetGenericLocation` comes from folded code).
const EFFECT_VALUE_008041A0: u32 = 0x0080_41a0;
/// `00803e80` (thiscall, one flag word; unnamed, `fallout/magic`).
const EFFECT_FLAG_00803E80: u32 = 0x0080_3e80;
/// `0059bb30` (thiscall, returns a pointer; the map's name `D3DTexture_LockRect`
/// comes from folded code): the effect's magic target object.
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `006815c0` (thiscall on a list node, returns the address of its item).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node, returns the next node).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `00833c20` (thiscall on an item, returns `AL`; unnamed).
const ITEM_TEST_00833C20: u32 = 0x0083_3c20;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MagicItem`'s RTTI type descriptor (`.?AVMagicItem@@`).
const MAGIC_ITEM_TYPE: u32 = 0x0118_3140;
/// `SpellItem`'s RTTI type descriptor (`.?AVSpellItem@@`).
const SPELL_ITEM_TYPE: u32 = 0x0118_3060;
/// `AlchemyItem`'s RTTI type descriptor (`.?AVAlchemyItem@@`).
const ALCHEMY_ITEM_TYPE: u32 = 0x0118_30cc;
/// `004030b0` (thiscall on the `AlchemyItem`, returns a word; unnamed).
const ALCHEMY_VALUE_004030B0: u32 = 0x0040_30b0;
/// `00403e20` (thiscall, returns the address of a `float`; unnamed, `effectitem.cpp`).
const FLOAT_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// The object `fn_008043f0` asks `00403e20` for its `float`.
const FLOAT_OWNER_011D17DC: u32 = 0x011d_17dc;
/// `006b9130` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_006B9130: u32 = 0x006b_9130;
/// `00406cc0` (cdecl, one `float`, returns `ST0`; unnamed).
const FLOAT_FUNCTION_00406CC0: u32 = 0x0040_6cc0;
/// `0084e3a0` (thiscall, returns a word; unnamed getter).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
/// `00409140` (thiscall, returns a pointer; unnamed, `enchantmentitem.cpp`).
const GETTER_00409140: u32 = 0x0040_9140;
/// `TESCondition::IsTrue` (`00680c30`, thiscall: two words).
const TES_CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// `0044ddc0` (thiscall, returns a pointer; unnamed getter).
const GETTER_0044DDC0: u32 = 0x0044_ddc0;
/// The player pointer (as other units name `011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `Interface::IsInMenuMode` (`00702360`, no arguments).
const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `007df1d0` (thiscall, returns a pointer; unnamed, filed under `statsmenu.cpp`).
const EFFECT_POINTER_007DF1D0: u32 = 0x007d_f1d0;
/// `00403e60` (thiscall, one flag word, returns `AL`; unnamed, `effectitem.cpp`).
const FLAG_TEST_00403E60: u32 = 0x0040_3e60;
/// `0040df90` (cdecl, one word; unnamed).
const WORD_FUNCTION_0040DF90: u32 = 0x0040_df90;
/// `00495460` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_00495460: u32 = 0x0049_5460;
/// `00470470` (thiscall on the hit-effect list; unnamed).
const HIT_LIST_00470470: u32 = 0x0047_0470;
/// `004702f0` (thiscall on the hit-effect list, one word; unnamed).
const HIT_LIST_004702F0: u32 = 0x0047_02f0;
/// `008c7aa0` (no arguments, returns `AL`; unnamed).
const TEST_008C7AA0: u32 = 0x008c_7aa0;
/// `004537b0` (no arguments, returns a pointer; unnamed, `tes.cpp`).
const GETTER_004537B0: u32 = 0x0045_37b0;
/// `0087aec0` (thiscall, one word; unnamed, `taskqueueinterface.cpp`).
const TASK_QUEUE_0087AEC0: u32 = 0x0087_aec0;
/// `MagicHitEffect::InstantiateHitEffects` (`0081b8e0`, cdecl: the effect).
const MAGIC_HIT_EFFECT_INSTANTIATE: u32 = 0x0081_b8e0;
/// `007df1f0` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_007DF1F0: u32 = 0x007d_f1f0;
/// `00825c00` (thiscall, returns a pointer; unnamed getter, `fallout/magic`).
const GETTER_00825C00: u32 = 0x0082_5c00;
/// `BSSoundHandle::IsValid` (`00ad8ce0`).
const BS_SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
/// `BSSoundHandle::Stop` (`00ad88f0`).
const BS_SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSSoundHandle::Release` (`00ad8d10`).
const BS_SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// `BSSoundHandle::SetPosition` (`00ad8b60`, three `float`s).
const BS_SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
/// `BSSoundHandle::Play` (`00ad8830`, one word).
const BS_SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSAudio::QInstance` (`00453a70`).
const BS_AUDIO_Q_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`, thiscall: handle, id, flags).
const BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID: u32 = 0x00ad_73b0;
/// `00408680` (thiscall, one flag word, returns `AL`; unnamed, `effectsetting.cpp`).
const FLAG_TEST_00408680: u32 = 0x0040_8680;
/// `00404040` (cdecl, one `float`, returns `ST0`; unnamed, `effectitem.cpp`).
const FLOAT_FUNCTION_00404040: u32 = 0x0040_4040;
/// `00693d50` (thiscall, one `float`; unnamed).
const SET_REAL_00693D50: u32 = 0x0069_3d50;
/// `004fd3e0` (thiscall, returns a pointer; the map's name `TES::GetWorldSpace`
/// may come from folded code).
const GETTER_004FD3E0: u32 = 0x004f_d3e0;
/// `0041a250` (thiscall: constructs a sound handle; unnamed, `extradatalist.cpp`).
const SOUND_HANDLE_INIT_0041A250: u32 = 0x0041_a250;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `004e75d0` (thiscall, one word: a 0x24-byte buffer; returns a pointer).
const FILL_004E75D0: u32 = 0x004e_75d0;
/// `005e39b0` (cdecl, one word; unnamed).
const WORD_FUNCTION_005E39B0: u32 = 0x005e_39b0;
/// `00933150` (thiscall on the actor, five words; unnamed, `mobileobject.cpp`).
const ACTOR_SOUND_00933150: u32 = 0x0093_3150;
/// `00418900` (thiscall, one word: assigns a sound handle; unnamed).
const SOUND_HANDLE_ASSIGN_00418900: u32 = 0x0041_8900;
/// `00483710` (thiscall: the sound handle's destructor here; unnamed).
const SOUND_HANDLE_DESTROY_00483710: u32 = 0x0048_3710;
/// `00865e50` (thiscall on the save buffer: data, size, word; unnamed).
const SAVE_DATA_00865E50: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (`00865db0`: id, word).
const SAVE_FORM_ID: u32 = 0x0086_5db0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`: form, word).
const SAVE_FORM_ID_OV2: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (`00865f20`).
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865f60`: value).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (`00865ff0`: value, handle).
const SAVE_VARIABLE_SIZED_VALUE_OV2: u32 = 0x0086_5ff0;
/// `MagicCaster::GetMagicCasterFormID` (`00818be0`).
const MAGIC_CASTER_GET_FORM_ID: u32 = 0x0081_8be0;
/// `MagicTarget::GetMagicTargetFormID` (`008254c0`).
const MAGIC_TARGET_GET_FORM_ID: u32 = 0x0082_54c0;
/// `MagicItem::GetMagicItemFormID` (`0040a1e0`).
const MAGIC_ITEM_GET_FORM_ID: u32 = 0x0040_a1e0;
/// `EffectItemList::GetEffectItemIndex` (`00406e70`, thiscall: the effect item).
const EFFECT_ITEM_LIST_GET_INDEX: u32 = 0x0040_6e70;
/// `00810fd0` (thiscall, returns a word; unnamed).
const GETTER_00810FD0: u32 = 0x0081_0fd0;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;

// Translated from 007a7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7eb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSSimpleList<ActiveEffect *>::Insert` (Xbox PDB): inserts `item` (nothing when null)
/// into the list whose head node is `this`, before the first node whose item `compare`
/// (cdecl: new item, node item) does not put it after (result `<= 0`), or at the end. An
/// empty head node takes the item itself. A new node is 8 bytes (`operator new`, then
/// `00470440` with the address of the item); a null allocation links a null node.
/// Insertion before the head goes through `005ae3d0`.
pub fn bs_simple_list_active_effect_insert(e: &mut Engine, this: Ptr, item: u32, compare: u32) {
    if item == 0 {
        return;
    }
    let list = this.addr();
    e.with_stack(4, |e, item_slot| {
        e.mem.set_u32(item_slot.addr(), item);
        let mut previous = 0u32;
        let mut done = false;
        let mut node = list;
        while node != 0 && !done {
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
                done = true;
            } else {
                let current = e.mem.u32(node);
                if e.call(compare, &args![item, current]).i32() <= 0 {
                    if previous == 0 {
                        e.call(LIST_INSERT_FRONT_005AE3D0, &args![list, item_slot]);
                    } else {
                        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                        let new_node = if block == 0 {
                            0
                        } else {
                            e.call(LIST_NODE_INIT_00470440, &args![block, item_slot])
                                .u32()
                        };
                        e.mem.set_u32(previous + 4, new_node);
                        e.mem.set_u32(new_node.wrapping_add(4), node);
                    }
                    done = true;
                } else if e.mem.u32(node + 4) == 0 {
                    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                    let new_node = if block == 0 {
                        0
                    } else {
                        e.call(LIST_NODE_INIT_00470440, &args![block, item_slot])
                            .u32()
                    };
                    e.mem.set_u32(node + 4, new_node);
                    done = true;
                }
            }
            previous = node;
            node = e.mem.u32(node + 4);
        }
    });
}

// Translated from 00804090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::CopyData` (Xbox PDB): copies the effect's state into `target`: the
/// elapsed seconds, spell, effect item, the five state bytes, magnitude, duration,
/// target, caster, spell type, source, flags and displacement spell. The three floats
/// go through `FLD`/`FSTP` (a signalling NaN arrives quiet).
pub fn active_effect_copy_data(e: &mut Engine, this: Ptr<ActiveEffect>, target: Ptr<ActiveEffect>) {
    let from = this.addr();
    let to = target.addr();
    let quiet = |e: &mut Engine, offset: u32| {
        let bits = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, bits);
    };
    let word = |e: &mut Engine, offset: u32| {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    };
    let byte = |e: &mut Engine, offset: u32| {
        let value = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, value);
    };
    quiet(e, 0x04);
    word(e, 0x08);
    word(e, 0x0c);
    for offset in 0x10..=0x14 {
        byte(e, offset);
    }
    quiet(e, 0x1c);
    quiet(e, 0x20);
    word(e, 0x24);
    word(e, 0x28);
    word(e, 0x2c);
    word(e, 0x3c);
    word(e, 0x18);
    word(e, 0x44);
}

// Translated from 00804160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::Finalize` (Xbox PDB): when `007df210` is true or `008041a0` is
/// non-zero, re-evaluates the effect's conditions at once (`fn_008043f0` with 0 seconds
/// and `force` set).
pub fn active_effect_finalize(e: &mut Engine, this: Ptr<ActiveEffect>) {
    if e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0
    {
        fn_008043f0(e, this, 0.0, true);
    }
}

// Translated from 00804210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::Dispel` (Xbox PDB): sets `bDone`, calls `00803e80` with 0x200 and,
/// when `now` is set, runs the update (`fn_00804560`) with 0 seconds.
pub fn active_effect_dispel(e: &mut Engine, this: Ptr<ActiveEffect>, now: bool) {
    e.set(this, ActiveEffect::bDone, 1);
    e.call(EFFECT_FLAG_00803E80, &args![this, 0x200u32]);
    if now {
        fn_00804560(e, this, 0.0);
    }
}

// Translated from 00804250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`): whether one of the target's own effects (the list the
/// target object's slot +8 returns) blocks this one: an effect (`00833c20` true) whose
/// spell is this effect's displacement spell, or whose spell is an `AlchemyItem` (type 7
/// from its slot +0x18) whose `004030b0` word is this effect's spell as a `SpellItem`
/// (`__RTDynamicCast`). Only looked at when the displacement spell is set. The answer is
/// false anyway when the target is an actor (target slot +0xc true, the actor at
/// `target - 0x94`) whose slot +8 of the object at +0xa4 returns more than 0 for 0x48.
pub fn fn_00804250(e: &mut Engine, this: Ptr<ActiveEffect>) -> bool {
    let mut found = false;
    let mut excluded = false;
    let target = e.get(this, ActiveEffect::pTarget);
    if e.vcall(target, 0xc, &[]).bool() {
        let actor = if target == 0 {
            0
        } else {
            target.wrapping_sub(0x94)
        };
        if actor != 0 && e.vcall(actor + 0xa4, 8, &args![0x48u32]).i32() > 0 {
            excluded = true;
        }
    }
    let spell = e.get(this, ActiveEffect::pSpell);
    let as_spell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![spell, 0u32, MAGIC_ITEM_TYPE, SPELL_ITEM_TYPE, 0u32],
        )
        .u32();
    if e.get(this, ActiveEffect::pDisplacementSpell) != 0
        && e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() != 0
    {
        let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        let mut node = e.vcall(target_object, 8, &[]).u32();
        while node != 0 && !found {
            let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 && e.call(ITEM_TEST_00833C20, &args![item]).bool() {
                let item_spell = e.mem.u32(item + 8);
                if item_spell == e.get(this, ActiveEffect::pDisplacementSpell) {
                    found = true;
                } else if e.vcall(item_spell, 0x18, &[]).u32() == 7 {
                    let alchemy = e
                        .call(
                            RT_DYNAMIC_CAST,
                            &args![item_spell, 0u32, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE, 0u32],
                        )
                        .u32();
                    if alchemy != 0
                        && e.call(ALCHEMY_VALUE_004030B0, &args![alchemy]).u32() == as_spell
                    {
                        found = true;
                    }
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
    if found && excluded {
        found = false;
    }
    found
}

// Translated from 008043f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`): re-evaluates whether the effect is active. Only when
/// `007df210` is true or `008041a0` is non-zero, the target object (`0059bb30`) is set
/// and its slot +4 returns non-zero. Unless `force` is set, it does so only when the
/// step crosses a boundary: `00406cc0` of `006b9130 / s` and of
/// `(006b9130 + seconds) / s` differ (`s` the `float` `00403e20` gives for `011d17dc`).
/// Then the effect is active when its condition (`TESCondition::IsTrue` on
/// `00409140(0084e3a0(this))` with the target object's slot +4 result and 0) holds and
/// `fn_00804250` does not block it; an effect that becomes active again after it
/// finished (and is not done) clears `bStarted` and `bFinished`.
pub fn fn_008043f0(e: &mut Engine, this: Ptr<ActiveEffect>, seconds: f32, force: bool) {
    if !(e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0)
    {
        return;
    }
    let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    if target_object == 0 || e.vcall(target_object, 4, &[]).u32() == 0 {
        return;
    }
    let scale_address = e
        .call(FLOAT_ADDRESS_00403E20, &args![FLOAT_OWNER_011D17DC])
        .u32();
    let scale = x87_quiet_bits(e.mem.u32(scale_address));
    let changed = if force {
        true
    } else {
        let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
        let quotient = x87_store32(x87_div(now, x87_m32(scale)));
        let first = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(quotient)])
            .f64();
        let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
        let later = x87_add(now, x87_m32(seconds.to_bits()));
        let quotient = x87_store32(x87_div(later, x87_m32(scale)));
        let second = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(quotient)])
            .f64();
        // FSTP/FLD double, then FUCOMPP: equal only when ordered and equal.
        let first = x87_load64(first.to_bits());
        !(first == second)
    };
    if !changed {
        return;
    }
    let value = e.vcall(target_object, 4, &[]).u32();
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let condition = e.call(GETTER_00409140, &args![owner]).u32();
    let holds = e
        .call(TES_CONDITION_IS_TRUE, &args![condition, value, 0u32])
        .u8();
    let blocked = fn_00804250(e, this);
    let active = holds & u8::from(!blocked);
    if active != 0
        && e.get(this, ActiveEffect::bActive) == 0
        && e.get(this, ActiveEffect::bFinished) != 0
        && e.get(this, ActiveEffect::bDone) == 0
    {
        e.set(this, ActiveEffect::bStarted, 0);
        e.set(this, ActiveEffect::bFinished, 0);
    }
    e.set(this, ActiveEffect::bActive, active);
}

/// Releases the effect's hit-effect list (`00470470`, then `004702f0` with 1) and
/// clears the pointer, as `fn_00804560` does twice.
fn release_hit_effects(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let list = e.get(this, ActiveEffect::pHitEffects);
    if list != 0 {
        e.call(HIT_LIST_00470470, &args![list]);
        let list = e.get(this, ActiveEffect::pHitEffects);
        if list != 0 {
            e.call(HIT_LIST_004702F0, &args![list, 1u32]);
        }
        e.set(this, ActiveEffect::pHitEffects, 0);
    }
}

/// Whether the effect's target reference is the player and the interface is in
/// menu mode, as `fn_00804560` tests it.
fn player_in_menu_mode(e: &mut Engine, target_reference: u32) -> bool {
    target_reference == e.global::<u32>(PLAYER_POINTER)
        && e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
}

// Translated from 00804560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`; the effect's per-frame update): advances the effect by
/// `seconds`. It first re-evaluates the conditions (`fn_008043f0`, when `007df210` or
/// `008041a0` allow). With `seconds > 0` and the effect not done: an active effect that
/// has neither started nor finished starts (sound through `fn_00804cd0`, or flag 0x20
/// for the player in menu mode; `Start`, slot +0x50; then, unless flag 1 or the effect
/// setting's flag 0x8000000 says otherwise, flag 4 for spell type 4, the hit effects
/// (flag 0x40 for the player in menu mode), all as the exe tests them) and is marked
/// started. The elapsed time grows by `seconds`, clamped to the duration for an effect
/// with a duration (`fn_00804b30` false and flag 0x100 clear). An active effect runs
/// the delayed hit effects and sound (flags 0x40 and 0x20) outside menu mode and its
/// `Update` (slot +0xc). It is done when the effect setting has flag 0x80 or the
/// elapsed time reached the duration, or when the target actor's slot +0x22c is true
/// and the setting lacks flag 0x10000000. A started effect that is done or inactive
/// and not finished then finishes (`Finish`, slot +0x58) and stops its persistent sound.
pub fn fn_00804560(e: &mut Engine, this: Ptr<ActiveEffect>, seconds: f32) {
    let base = this.addr();
    if e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0
    {
        fn_008043f0(e, this, seconds, false);
    }
    'update: {
        let zero: f64 = e.global(ZERO);
        // FCOMP; TEST AH,0x41: skipped when below, equal or unordered.
        if !(x87_load32(seconds.to_bits()) > zero) || e.get(this, ActiveEffect::bDone) != 0 {
            break 'update;
        }
        let owner = e.call(GETTER_0044DDC0, &args![this]).u32();
        let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        let target_reference = e.vcall(target_object, 4, &[]).u32();
        let spell_type = e.get(this, ActiveEffect::eSpellType) as i32;
        let no_duration = fn_00804b30(e, spell_type);
        if e.get(this, ActiveEffect::bStarted) == 0
            && e.get(this, ActiveEffect::bFinished) == 0
            && e.get(this, ActiveEffect::bActive) != 0
        {
            if player_in_menu_mode(e, target_reference) {
                e.call(EFFECT_FLAG_00803E80, &args![this, 0x20u32]);
            } else {
                fn_00804cd0(e, this);
            }
            e.vcall(base, 0x50, &[]);
            if e.get(this, ActiveEffect::bDone) == 0 && e.get(this, ActiveEffect::bFinished) == 0 {
                'start: {
                    if fn_00804b80(e, this, 1) {
                        break 'start;
                    }
                    let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                    if e.call(FLAG_TEST_00403E60, &args![setting, 0x0800_0000u32])
                        .bool()
                    {
                        break 'start;
                    }
                    if e.get(this, ActiveEffect::eSpellType) == 4 {
                        let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                        if e.call(WORD_FUNCTION_0040DF90, &args![setting]).u32() == 0 {
                            break 'start;
                        }
                        let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
                        let hundred: f64 = e.global(HUNDRED);
                        // FCOMP; TEST AH,5; JP: skipped unless below.
                        if !(value < hundred) {
                            break 'start;
                        }
                    }
                    if e.get(this, ActiveEffect::eSpellType) == 4 {
                        e.call(EFFECT_FLAG_00803E80, &args![this, 4u32]);
                    }
                    let first = if e.vcall(owner, 0x18, &[]).u32() == 8 {
                        if owner == 0 {
                            0
                        } else {
                            owner.wrapping_sub(0x30)
                        }
                    } else {
                        0
                    };
                    let second = if e.vcall(owner, 0x18, &[]).u32() == 7 {
                        if owner == 0 {
                            0
                        } else {
                            owner.wrapping_sub(0x30)
                        }
                    } else {
                        0
                    };
                    let skip = if first != 0 {
                        e.vcall(first + 0x3c, 4, &[]).u8()
                    } else if second != 0 {
                        e.vcall(second + 0x3c, 4, &[]).u8()
                    } else {
                        0
                    };
                    if skip != 0 {
                        break 'start;
                    }
                    if player_in_menu_mode(e, target_reference) {
                        e.call(EFFECT_FLAG_00803E80, &args![this, 0x40u32]);
                        break 'start;
                    }
                    release_hit_effects(e, this);
                    if e.call(TEST_008C7AA0, &[]).bool() {
                        let queue = e.call(GETTER_004537B0, &[]).u32();
                        e.call(TASK_QUEUE_0087AEC0, &args![queue, this]);
                    } else {
                        let hit_effects = e.call(MAGIC_HIT_EFFECT_INSTANTIATE, &args![this]).u32();
                        e.set(this, ActiveEffect::pHitEffects, hit_effects);
                    }
                }
                e.set(this, ActiveEffect::bStarted, 1);
            }
        }
        if e.get(this, ActiveEffect::bDone) != 0 {
            break 'update;
        }
        let mut step = x87_quiet_bits(seconds.to_bits());
        let mut clamped = false;
        if !no_duration && !fn_00804b60(e, this) {
            let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
            let later = x87_add(now, x87_m32(step));
            let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
            // FCOMP; TEST AH,5; JP: clamped only when the duration is below.
            if duration < later {
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
                step = x87_store32(x87_sub(duration, now));
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                e.mem.set_u32(base + 4, x87_store32(duration));
                clamped = true;
            }
        }
        if !clamped {
            let elapsed = x87_load32(e.mem.u32(base + 4));
            let elapsed = x87_add(elapsed, x87_m32(seconds.to_bits()));
            e.mem.set_u32(base + 4, x87_store32(elapsed));
        }
        if e.get(this, ActiveEffect::bActive) != 0 {
            let hit_effects_due = fn_00804b80(e, this, 0x40);
            let sound_due = fn_00804b80(e, this, 0x20);
            if (hit_effects_due || sound_due)
                && target_reference == e.global::<u32>(PLAYER_POINTER)
                && !e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
            {
                if hit_effects_due {
                    release_hit_effects(e, this);
                    let hit_effects = e.call(MAGIC_HIT_EFFECT_INSTANTIATE, &args![this]).u32();
                    e.set(this, ActiveEffect::pHitEffects, hit_effects);
                    fn_00804ba0(e, this, 0x40);
                }
                if sound_due {
                    fn_00804cd0(e, this);
                    fn_00804ba0(e, this, 0x20);
                }
            }
            e.vcall(base, 0xc, &args![x87_quiet_bits(step)]);
        }
        if !no_duration && !fn_00804b60(e, this) {
            let effect = e.get(this, ActiveEffect::pEffect);
            let setting = e.call(GETTER_00825C00, &args![effect]).u32();
            if e.call(FLAG_TEST_00403E60, &args![setting, 0x80u32]).bool() {
                e.set(this, ActiveEffect::bDone, 1);
            } else {
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                let elapsed = x87_load32(e.mem.u32(base + 4));
                // FCOMPP; TEST AH,1: done unless below or unordered.
                if !(elapsed < duration || elapsed.is_nan() || duration.is_nan()) {
                    e.set(this, ActiveEffect::bDone, 1);
                }
            }
        }
        let target = e.get(this, ActiveEffect::pTarget);
        if e.vcall(target, 0xc, &[]).bool() {
            let target = e.get(this, ActiveEffect::pTarget);
            let actor = if target == 0 {
                0
            } else {
                target.wrapping_sub(0x94)
            };
            if actor != 0 && e.vcall(actor, 0x22c, &args![1u32]).bool() {
                let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                if !e
                    .call(FLAG_TEST_00403E60, &args![setting, 0x1000_0000u32])
                    .bool()
                {
                    e.set(this, ActiveEffect::bDone, 1);
                }
            }
        }
    }
    if e.get(this, ActiveEffect::bStarted) != 0
        && (e.get(this, ActiveEffect::bDone) != 0 || e.get(this, ActiveEffect::bActive) == 0)
        && e.get(this, ActiveEffect::bFinished) == 0
    {
        e.vcall(base, 0x58, &[]);
        e.set(this, ActiveEffect::bFinished, 1);
        if e.call(BS_SOUND_HANDLE_IS_VALID, &args![base + PERSISTENT_SOUND])
            .bool()
        {
            e.call(BS_SOUND_HANDLE_STOP, &args![base + PERSISTENT_SOUND]);
        }
    }
}

// Translated from 00804b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: whether flag 0x100 of `iFlags` is set.
pub fn fn_00804b60(e: &mut Engine, this: Ptr<ActiveEffect>) -> bool {
    e.get(this, ActiveEffect::iFlags) & 0x100 != 0
}

// Translated from 00804b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: whether any of `mask` is set in `iFlags`.
pub fn fn_00804b80(e: &mut Engine, this: Ptr<ActiveEffect>, mask: u32) -> bool {
    e.get(this, ActiveEffect::iFlags) & mask != 0
}

// Translated from 00804ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: clears `mask` in `iFlags`.
pub fn fn_00804ba0(e: &mut Engine, this: Ptr<ActiveEffect>, mask: u32) {
    let flags = e.get(this, ActiveEffect::iFlags);
    e.set(this, ActiveEffect::iFlags, !mask & flags);
}

// Translated from 00804bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::AdjustEffectiveness` (Xbox PDB): scales the effect by `factor`
/// unless the effect setting (`00825c00(0084e3a0(this))`) has flag 0x180 by
/// `00408680`, or `factor` is 1. With the setting's flag 0x100 (`00403e60`) it passes
/// `007df1f0 * factor` to `00693d50`; otherwise the magnitude becomes
/// `00404040(00495460 * factor)`, at least 1.
pub fn active_effect_adjust_effectiveness(e: &mut Engine, this: Ptr<ActiveEffect>, factor: f32) {
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let setting = e.call(GETTER_00825C00, &args![owner]).u32();
    if e.call(FLAG_TEST_00408680, &args![setting, 0x180u32]).bool() {
        return;
    }
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,0x44; JP: returns only when equal.
    if x87_load32(factor.to_bits()) == one {
        return;
    }
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let setting = e.call(GETTER_00825C00, &args![owner]).u32();
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x100u32]).bool() {
        let value = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
        let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
        e.call(SET_REAL_00693D50, &args![this, x87_quiet_bits(scaled)]);
        return;
    }
    let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
    let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
    let result = e
        .call(FLOAT_FUNCTION_00404040, &args![x87_quiet_bits(scaled)])
        .f64();
    // FCOMP; TEST AH,5; JP: 1.0 only when below.
    let magnitude = if result < one {
        x87_store32(1.0)
    } else {
        let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
        let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
        let result = e
            .call(FLOAT_FUNCTION_00404040, &args![x87_quiet_bits(scaled)])
            .f64();
        x87_store32(result)
    };
    fn_00804cb0(e, this, f32::from_bits(x87_quiet_bits(magnitude)));
}

// Translated from 00804cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: stores `magnitude` in `fMagnitude` (`FLD`/`FSTP`: a signalling NaN
/// arrives quiet).
pub fn fn_00804cb0(e: &mut Engine, this: Ptr<ActiveEffect>, magnitude: f32) {
    e.mem
        .set_u32(this.addr() + 0x1c, x87_quiet_bits(magnitude.to_bits()));
}

// Translated from 00804cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00804cd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`activeeffect.cpp`): starts the effect's sound. The sound id is
/// `0084e3a0(004fd3e0(007df1d0(this)))` (none when `004fd3e0` returns null); nothing
/// happens without it, without a target object, or when the effect's owner
/// (`0044ddc0`) is of type 4 (slot +0x18). It is looping when the setting has flag 0x400
/// (`00403e60`), or, when `004fd3e0` is set, when `005e39b0` of the word at +4 of the
/// result of `004e75d0` is non-zero. An actor target plays it through `00933150` (flags
/// 0x112 looping, 0x102 not); another target through `BSAudio`, at the position its
/// reference's slot +0x1f4 gives. A looping sound replaces the effect's persistent sound.
pub fn fn_00804cd0(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let base = this.addr();
    // The frame below `EBP` (0x80 bytes): `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x80 - offset;
        let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        let sound_id = if e.call(GETTER_004FD3E0, &args![pointer]).u32() == 0 {
            0
        } else {
            let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
            let object = e.call(GETTER_004FD3E0, &args![pointer]).u32();
            e.call(GETTER_0084E3A0, &args![object]).u32()
        };
        if sound_id == 0 || e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
            return;
        }
        let owner = e.call(GETTER_0044DDC0, &args![this]).u32();
        if e.vcall(owner, 0x18, &[]).u32() == 4 {
            return;
        }
        let sound = local(0x24);
        e.call(SOUND_HANDLE_INIT_0041A250, &args![sound]);
        let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
            0
        } else {
            let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target_object])
                .u32()
        };
        let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        let mut looping = e.call(FLAG_TEST_00403E60, &args![setting, 0x400u32]).u8();
        let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        if e.call(GETTER_004FD3E0, &args![pointer]).u32() != 0 {
            let buffer = local(0x68);
            let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
            let object = e.call(GETTER_004FD3E0, &args![pointer]).u32();
            let result = e.call(FILL_004E75D0, &args![object, buffer]).u32();
            let word = e.mem.u32(result + 4);
            looping = u8::from(e.call(WORD_FUNCTION_005E39B0, &args![word]).u32() != 0);
        }
        if actor != 0 {
            let flags: u32 = if looping != 0 { 0x112 } else { 0x102 };
            let handle = local(0x74);
            let result = e
                .call(
                    ACTOR_SOUND_00933150,
                    &args![actor, handle, sound_id, u32::from(looping), flags, 0u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN_00418900, &args![sound, result]);
            e.call(SOUND_HANDLE_DESTROY_00483710, &args![handle]);
        } else {
            let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            let reference = e.vcall(target_object, 4, &[]).u32();
            if reference != 0 {
                let handle = local(0x44);
                let audio = e.call(BS_AUDIO_Q_INSTANCE, &[]).u32();
                e.call(
                    BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID,
                    &args![audio, handle, sound_id, 0x102u32],
                );
                let position = e.vcall(reference, 0x1f4, &[]).u32();
                let x = e.mem.u32(position);
                let y = e.mem.u32(position + 4);
                let z = e.mem.u32(position + 8);
                e.call(
                    BS_SOUND_HANDLE_SET_POSITION,
                    &args![
                        handle,
                        x87_quiet_bits(x),
                        x87_quiet_bits(y),
                        x87_quiet_bits(z)
                    ],
                );
                e.call(BS_SOUND_HANDLE_PLAY, &args![handle, u32::from(looping)]);
                e.call(SOUND_HANDLE_DESTROY_00483710, &args![handle]);
            }
        }
        if looping != 0 {
            let persistent = base + PERSISTENT_SOUND;
            if e.call(BS_SOUND_HANDLE_IS_VALID, &args![persistent]).bool() {
                e.call(BS_SOUND_HANDLE_STOP, &args![persistent]);
                e.call(BS_SOUND_HANDLE_RELEASE, &args![persistent]);
            }
            e.call(SOUND_HANDLE_ASSIGN_00418900, &args![persistent, sound]);
        }
        e.call(SOUND_HANDLE_DESTROY_00483710, &args![sound]);
    });
}

// Translated from 00806200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::CheckDeletedCaster` (Xbox PDB): when `caster` is the effect's caster,
/// clears it and returns `true`.
pub fn active_effect_check_deleted_caster(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    caster: u32,
) -> bool {
    if caster == e.get(this, ActiveEffect::pCaster) {
        e.set(this, ActiveEffect::pCaster, 0);
        true
    } else {
        false
    }
}

// Translated from 00806230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveGame_ov2` (Xbox PDB): writes the effect to the save buffer: the
/// elapsed seconds, the five state bytes, magnitude, duration and flags (`00865e50`),
/// the caster's, target's and displacement spell's form ids (0 when unset), the source
/// form, and the hit effects as a variable-sized value: for each, its type byte (slot
/// +0x9c) and its own save (slot +0xa8 with the buffer); the count closes the value.
pub fn active_effect_save_game_ov2(e: &mut Engine, this: Ptr<ActiveEffect>, buffer: u32) {
    let base = this.addr();
    for (offset, size) in [
        (0x04u32, 4u32),
        (0x10, 1),
        (0x11, 1),
        (0x12, 1),
        (0x13, 1),
        (0x14, 1),
        (0x1c, 4),
        (0x20, 4),
        (0x18, 4),
    ] {
        e.call(
            SAVE_DATA_00865E50,
            &args![buffer, base + offset, size, 0u32],
        );
    }
    let caster = e.get(this, ActiveEffect::pCaster);
    let id = if caster == 0 {
        0
    } else {
        e.call(MAGIC_CASTER_GET_FORM_ID, &args![caster]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let target = e.get(this, ActiveEffect::pTarget);
    let id = if target == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_GET_FORM_ID, &args![target]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let displacement = e.get(this, ActiveEffect::pDisplacementSpell);
    let id = if displacement == 0 {
        0
    } else {
        e.call(MAGIC_ITEM_GET_FORM_ID, &args![displacement]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let source = e.get(this, ActiveEffect::pSource);
    e.call(SAVE_FORM_ID_OV2, &args![buffer, source, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = e.get(this, ActiveEffect::pHitEffects);
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let hit_effect = e.mem.u32(slot);
        if hit_effect != 0 {
            let kind = e.vcall(hit_effect, 0x9c, &[]).u8();
            e.with_stack(4, |e, byte| {
                e.mem.set_u8(byte.addr(), kind);
                e.call(SAVE_DATA_00865E50, &args![buffer, byte, 1u32, 0u32]);
            });
            e.vcall(hit_effect, 0xa8, &args![buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE_OV2, &args![buffer, count, handle]);
}

// Translated from 00806840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: save buffer, effect): writes one active effect of a list: its spell's
/// form id (0 when unset), the effect item's index in the spell's list (`+0xc`), the
/// word `00810fd0` gives for its effect setting, then the effect's own save (slot +0x14)
/// as a variable-sized value closed with the number of bytes it wrote (`0084e3a0`
/// before and after).
pub fn fn_00806840(e: &mut Engine, buffer: u32, effect: Ptr<ActiveEffect>) {
    let spell = e.get(effect, ActiveEffect::pSpell);
    let id = if spell == 0 {
        0
    } else {
        e.call(MAGIC_ITEM_GET_FORM_ID, &args![spell]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let item = e.get(effect, ActiveEffect::pEffect);
    let spell = e.get(effect, ActiveEffect::pSpell);
    let index = e
        .call(
            EFFECT_ITEM_LIST_GET_INDEX,
            &args![spell.wrapping_add(0xc), item],
        )
        .u8();
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), index);
        e.call(SAVE_DATA_00865E50, &args![buffer, byte, 1u32, 0u32]);
    });
    let item = e.get(effect, ActiveEffect::pEffect);
    let setting = e.call(GETTER_00825C00, &args![item]).u32();
    let value = e.call(GETTER_00810FD0, &args![setting]).u32();
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, value]);
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let start = e.call(GETTER_0084E3A0, &args![buffer]).u32();
    e.vcall(effect.addr(), 0x14, &args![buffer]);
    let end = e.call(GETTER_0084E3A0, &args![buffer]).u32();
    e.call(
        SAVE_VARIABLE_SIZED_VALUE_OV2,
        &args![buffer, end.wrapping_sub(start), handle],
    );
}

// Translated from 00806a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveActiveEffectList` (Xbox PDB, cdecl: save buffer, list): writes each
/// effect of the list with `fn_00806840` as one variable-sized value closed with their
/// count.
pub fn active_effect_save_active_effect_list(e: &mut Engine, buffer: u32, list: u32) {
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        if effect != 0 {
            fn_00806840(e, buffer, Ptr::new(effect));
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE_OV2, &args![buffer, count, handle]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_4b30, fn_00804b30(i32) -> bool),
        entry!(
            0x007a_7eb0,
            bs_simple_list_active_effect_insert(Ptr, u32, u32)
        ),
        entry!(
            0x0080_4090,
            active_effect_copy_data(Ptr<ActiveEffect>, Ptr<ActiveEffect>)
        ),
        entry!(0x0080_4160, active_effect_finalize(Ptr<ActiveEffect>)),
        entry!(0x0080_4210, active_effect_dispel(Ptr<ActiveEffect>, bool)),
        entry!(0x0080_4250, fn_00804250(Ptr<ActiveEffect>) -> bool),
        entry!(0x0080_43f0, fn_008043f0(Ptr<ActiveEffect>, f32, bool)),
        entry!(0x0080_4560, fn_00804560(Ptr<ActiveEffect>, f32)),
        entry!(0x0080_4b60, fn_00804b60(Ptr<ActiveEffect>) -> bool),
        entry!(0x0080_4b80, fn_00804b80(Ptr<ActiveEffect>, u32) -> bool),
        entry!(0x0080_4ba0, fn_00804ba0(Ptr<ActiveEffect>, u32)),
        entry!(
            0x0080_4bc0,
            active_effect_adjust_effectiveness(Ptr<ActiveEffect>, f32)
        ),
        entry!(0x0080_4cb0, fn_00804cb0(Ptr<ActiveEffect>, f32)),
        entry!(0x0080_4cd0, fn_00804cd0(Ptr<ActiveEffect>)),
        entry!(
            0x0080_6200,
            active_effect_check_deleted_caster(Ptr<ActiveEffect>, u32) -> bool
        ),
        entry!(
            0x0080_6230,
            active_effect_save_game_ov2(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0080_6840, fn_00806840(u32, Ptr<ActiveEffect>)),
        entry!(0x0080_6a10, active_effect_save_active_effect_list(u32, u32)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00804b30_accepts_only_1_4_and_10() {
        let mut e = Engine::new();
        for value in [
            -1i32,
            0,
            1,
            2,
            3,
            4,
            5,
            9,
            10,
            11,
            0x100,
            0x401,
            i32::MIN,
            i32::MAX,
        ] {
            let expected = matches!(value, 1 | 4 | 10);
            assert_eq!(
                e.call(0x0080_4b30, &args![value]).bool(),
                expected,
                "{value}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
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

    /// An object whose vtable has `slots` (byte offset, target); every other slot is 0.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x240);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn effect(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<ActiveEffect> {
        Ptr::new(object(e, slots))
    }

    // ---- BSSimpleList<ActiveEffect *>::Insert ----

    const COMPARE: u32 = 0x0300_0000;

    fn compare_words(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: (a[0] as i32).wrapping_sub(a[1] as i32) as u32,
            ..Ret::default()
        }
    }

    /// Doubles `operator new` and the node constructor (which copies the item
    /// through the address it gets and clears the link).
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(COMPARE, compare_words);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(LIST_NODE_INIT_00470440, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_INSERT_FRONT_005AE3D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn insert_ignores_a_null_item_and_fills_an_empty_head() {
        let mut e = list_engine();
        let head = e.mem.alloc(8);
        e.call(0x007a_7eb0, &args![head, 0u32, COMPARE]);
        assert_eq!(e.mem.u32(head), 0);
        e.call(0x007a_7eb0, &args![head, 5u32, COMPARE]);
        assert_eq!(e.mem.u32(head), 5);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn insert_appends_after_the_last_node_and_before_a_later_one() {
        let mut e = list_engine();
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, 10);
        // 20 sorts after 10: appended.
        e.call(0x007a_7eb0, &args![head, 20u32, COMPARE]);
        let second = e.mem.u32(head + 4);
        assert_eq!(e.mem.u32(second), 20);
        // 15 sorts before 20: a node between them.
        e.call(0x007a_7eb0, &args![head, 15u32, COMPARE]);
        let middle = e.mem.u32(head + 4);
        assert_eq!(e.mem.u32(middle), 15);
        assert_eq!(e.mem.u32(middle + 4), second);
        // 5 sorts before the head: the list's front insert with the item's address.
        e.call(0x007a_7eb0, &args![head, 5u32, COMPARE]);
        let front = calls(&e, LIST_INSERT_FRONT_005AE3D0);
        assert_eq!(front.len(), 1);
        assert_eq!(front[0][0], head);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 2);
    }

    // ---- CopyData, flags, magnitude, caster ----

    #[test]
    fn copy_data_copies_the_state_and_quiets_float_nans() {
        let mut e = Engine::new();
        let from = e.mem.alloc(0x48);
        let to = e.mem.alloc(0x48);
        for offset in (0..0x48).step_by(4) {
            e.mem.set_u32(from + offset, 0x1111_0000 + offset);
            e.mem.set_u32(to + offset, 0xeeee_eeee);
        }
        e.mem.set_u32(from + 4, 0x7f80_0001);
        e.mem.set_u32(from + 0x10, 0x0403_0201);
        e.call(0x0080_4090, &args![from, to]);
        assert_eq!(e.mem.u32(to + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(to + 0x10), 0x0403_0201);
        // Only the byte at +0x14 is copied.
        assert_eq!(e.mem.u32(to + 0x14), 0xeeee_ee14);
        for offset in [
            0x08u32, 0x0c, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x3c, 0x44,
        ] {
            assert_eq!(
                e.mem.u32(to + offset),
                0x1111_0000 + offset,
                "offset {offset:#x}"
            );
        }
        for offset in [0x00u32, 0x30, 0x34, 0x38, 0x40] {
            assert_eq!(e.mem.u32(to + offset), 0xeeee_eeee, "offset {offset:#x}");
        }
    }

    #[test]
    fn flag_helpers_test_and_clear_bits() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::iFlags, 0x160);
        assert!(e.call(0x0080_4b60, &args![this]).bool());
        assert!(e.call(0x0080_4b80, &args![this, 0x40u32]).bool());
        assert!(!e.call(0x0080_4b80, &args![this, 0x81u32]).bool());
        e.call(0x0080_4ba0, &args![this, 0x140u32]);
        assert_eq!(e.get(this, ActiveEffect::iFlags), 0x20);
        assert!(!e.call(0x0080_4b60, &args![this]).bool());
    }

    #[test]
    fn set_magnitude_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.call(0x0080_4cb0, &args![this, 2.5f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 2.5);
        e.call(0x0080_4cb0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x7fc0_0001);
    }

    #[test]
    fn check_deleted_caster_clears_only_its_own_caster() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::pCaster, 0x4444);
        assert!(!e.call(0x0080_6200, &args![this, 0x5555u32]).bool());
        assert_eq!(e.get(this, ActiveEffect::pCaster), 0x4444);
        assert!(e.call(0x0080_6200, &args![this, 0x4444u32]).bool());
        assert_eq!(e.get(this, ActiveEffect::pCaster), 0);
    }

    // ---- Finalize, Dispel ----

    #[test]
    fn finalize_reevaluates_only_when_allowed() {
        for (test, word, runs) in [(0u32, 0u32, false), (1, 0, true), (0, 7, true)] {
            let mut e = Engine::new();
            let this = effect(&mut e, &[]);
            answer(&mut e, EFFECT_TEST_007DF210, test);
            answer(&mut e, EFFECT_VALUE_008041A0, word);
            answer(&mut e, EFFECT_TARGET_0059BB30, 0);
            e.call_log = Some(vec![]);
            e.call(0x0080_4160, &args![this]);
            assert_eq!(!calls(&e, EFFECT_TARGET_0059BB30).is_empty(), runs);
        }
    }

    #[test]
    fn dispel_marks_done_and_updates_when_asked() {
        for now in [false, true] {
            let mut e = Engine::new();
            let this = effect(&mut e, &[]);
            answer(&mut e, EFFECT_FLAG_00803E80, 0);
            answer(&mut e, EFFECT_TEST_007DF210, 0);
            answer(&mut e, EFFECT_VALUE_008041A0, 0);
            e.map(ZERO, 8);
            e.call_log = Some(vec![]);
            e.call(0x0080_4210, &args![this, u32::from(now)]);
            assert_eq!(e.get(this, ActiveEffect::bDone), 1);
            assert_eq!(
                calls(&e, EFFECT_FLAG_00803E80),
                vec![vec![this.addr(), 0x200]]
            );
            assert_eq!(calls(&e, EFFECT_TEST_007DF210).len(), usize::from(now));
        }
    }

    // ---- fn_00804250 ----

    const TARGET_IS_ACTOR: u32 = 0x0300_0100;
    const ACTOR_VALUE: u32 = 0x0300_0104;
    const LIST_FIRST: u32 = 0x0300_0108;
    const SPELL_KIND: u32 = 0x0300_010c;

    /// An effect whose target object's list holds one item whose spell has
    /// type `item_spell_kind`; the target is an actor answering `actor_value`.
    fn blocking_setup(
        e: &mut Engine,
        item_spell_kind: u32,
        actor_value: i32,
    ) -> (Ptr<ActiveEffect>, u32) {
        let this = effect(e, &[]);
        // Target (MagicTarget at actor + 0x94): slot +0xc says actor.
        let actor = e.mem.alloc(0x200);
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 0xc, TARGET_IS_ACTOR);
        e.mem.set_u32(actor + 0x94, target_vtable);
        let value_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(value_vtable + 8, ACTOR_VALUE);
        e.mem.set_u32(actor + 0xa4, value_vtable);
        e.set(this, ActiveEffect::pTarget, actor + 0x94);
        answer(e, TARGET_IS_ACTOR, 1);
        answer(e, ACTOR_VALUE, actor_value as u32);
        let target_object = object(e, &[(8, LIST_FIRST)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        let node = e.mem.alloc(8);
        answer(e, LIST_FIRST, node);
        let item = e.mem.alloc(0x10);
        let item_spell = object(e, &[(0x18, SPELL_KIND)]);
        answer(e, SPELL_KIND, item_spell_kind);
        e.mem.set_u32(item + 8, item_spell);
        e.mem.set_u32(node, item);
        e.register(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        answer(e, LIST_NODE_NEXT, 0);
        answer(e, ITEM_TEST_00833C20, 1);
        (this, item_spell)
    }

    #[test]
    fn fn_00804250_finds_the_displacement_spell_unless_excluded() {
        for (value, want) in [(0i32, true), (1, false)] {
            let mut e = Engine::new();
            let (this, item_spell) = blocking_setup(&mut e, 0, value);
            answer(&mut e, RT_DYNAMIC_CAST, 0);
            e.set(this, ActiveEffect::pDisplacementSpell, item_spell);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_4250, &args![this]).bool(), want);
            let value_calls = calls(&e, ACTOR_VALUE);
            assert_eq!(value_calls.len(), 1);
            assert_eq!(value_calls[0][1], 0x48);
        }
        // No displacement spell: the list is not looked at.
        let mut e = Engine::new();
        let (this, _) = blocking_setup(&mut e, 0, 0);
        answer(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0080_4250, &args![this]).bool());
        assert!(calls(&e, LIST_FIRST).is_empty());
    }

    #[test]
    fn fn_00804250_matches_an_alchemy_item_by_its_word() {
        for (word, want) in [(0x7777u32, true), (0x7778, false)] {
            let mut e = Engine::new();
            let (this, _) = blocking_setup(&mut e, 7, 0);
            e.set(this, ActiveEffect::pDisplacementSpell, 0x9999);
            e.set(this, ActiveEffect::pSpell, 0x1234);
            e.register(RT_DYNAMIC_CAST, |_, a| Ret {
                eax: if a[3] == SPELL_ITEM_TYPE {
                    0x7777
                } else {
                    0x6666
                },
                ..Ret::default()
            });
            answer(&mut e, ALCHEMY_VALUE_004030B0, word);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_4250, &args![this]).bool(), want);
            let casts = calls(&e, RT_DYNAMIC_CAST);
            assert_eq!(
                casts[0],
                vec![0x1234, 0, MAGIC_ITEM_TYPE, SPELL_ITEM_TYPE, 0]
            );
            assert_eq!(casts[1][2..4], [MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE]);
            assert_eq!(calls(&e, ALCHEMY_VALUE_004030B0), vec![vec![0x6666]]);
        }
    }

    // ---- fn_008043f0 ----

    const TARGET_REFERENCE: u32 = 0x0300_0200;

    /// The callees of `fn_008043f0` with the target object's slot +4 returning
    /// `reference`, the condition answering `holds`, and nothing blocking.
    fn evaluate_setup(e: &mut Engine, reference: u32, holds: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        answer(e, EFFECT_TEST_007DF210, 1);
        answer(e, EFFECT_VALUE_008041A0, 0);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        answer(e, TARGET_REFERENCE, reference);
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 2.0);
        answer(e, FLOAT_ADDRESS_00403E20, scale);
        answer(e, GETTER_0084E3A0, 0x5100);
        answer(e, GETTER_00409140, 0x5200);
        answer(e, TES_CONDITION_IS_TRUE, holds);
        // fn_00804250: the target is no actor, no displacement spell.
        let target = object(e, &[(0xc, TARGET_IS_ACTOR)]);
        answer(e, TARGET_IS_ACTOR, 0);
        e.set(this, ActiveEffect::pTarget, target);
        answer(e, RT_DYNAMIC_CAST, 0);
        this
    }

    #[test]
    fn fn_008043f0_forced_sets_the_active_state_and_reopens_a_finished_effect() {
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0x4000, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::bFinished, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert_eq!(e.get(this, ActiveEffect::bActive), 1);
        assert_eq!(e.get(this, ActiveEffect::bStarted), 0);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 0);
        assert_eq!(
            calls(&e, TES_CONDITION_IS_TRUE),
            vec![vec![0x5200, 0x4000, 0]]
        );
        assert_eq!(
            calls(&e, FLOAT_ADDRESS_00403E20),
            vec![vec![FLOAT_OWNER_011D17DC]]
        );
        // The condition fails: inactive, the other state kept.
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0x4000, 0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bFinished, 1);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert_eq!(e.get(this, ActiveEffect::bActive), 0);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
    }

    #[test]
    fn fn_008043f0_unforced_needs_a_changed_boundary() {
        for (first, second, changes) in [
            (3.0f64, 3.0f64, false),
            (3.0, 4.0, true),
            (f64::NAN, f64::NAN, true),
        ] {
            let mut e = Engine::new();
            let this = evaluate_setup(&mut e, 0x4000, 1);
            answer_float(&mut e, EFFECT_REAL_006B9130, 5.0);
            let mut n = 0;
            e.register_double(FLOAT_FUNCTION_00406CC0, move |_, _| {
                n += 1;
                Ret {
                    st0: if n == 1 { first } else { second },
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            e.call(0x0080_43f0, &args![this, 1.0f32, 0u32]);
            // 5 / 2 and (5 + 1) / 2, as floats.
            let floors = calls(&e, FLOAT_FUNCTION_00406CC0);
            assert_eq!(floors, vec![vec![2.5f32.to_bits()], vec![3.0f32.to_bits()]]);
            assert_eq!(e.get(this, ActiveEffect::bActive), u8::from(changes));
        }
    }

    #[test]
    fn fn_008043f0_returns_early_without_a_reference() {
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert!(calls(&e, FLOAT_ADDRESS_00403E20).is_empty());
        assert_eq!(e.get(this, ActiveEffect::bActive), 0);
    }

    // ---- fn_00804560 ----

    const START: u32 = 0x0300_0300;
    const UPDATE: u32 = 0x0300_0304;
    const FINISH: u32 = 0x0300_0308;
    const OWNER_KIND: u32 = 0x0300_030c;
    const ACTOR_SLOT_22C: u32 = 0x0300_0310;

    /// An effect for `fn_00804560`: conditions not re-evaluated, an owner of
    /// type `owner_kind`, a target that is no actor, duration `duration`,
    /// `006b9130` 0.
    fn update_setup(e: &mut Engine, owner_kind: u32, duration: f64) -> Ptr<ActiveEffect> {
        let this = effect(e, &[(0xc, UPDATE), (0x50, START), (0x58, FINISH)]);
        for f in [START, UPDATE, FINISH] {
            answer(e, f, 0);
        }
        answer(e, EFFECT_TEST_007DF210, 0);
        answer(e, EFFECT_VALUE_008041A0, 0);
        e.map(ZERO, 8);
        e.mem.set_f64(ZERO, 0.0);
        e.map(HUNDRED, 8);
        e.mem.set_f64(HUNDRED, 100.0);
        e.map(PLAYER_POINTER, 4);
        e.mem.set_u32(PLAYER_POINTER, 0x0abc_0000);
        let owner = object(e, &[(0x18, OWNER_KIND)]);
        answer(e, OWNER_KIND, owner_kind);
        answer(e, GETTER_0044DDC0, owner);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        answer(e, TARGET_REFERENCE, 0x0def_0000);
        let target = object(e, &[(0xc, TARGET_IS_ACTOR)]);
        answer(e, TARGET_IS_ACTOR, 0);
        e.set(this, ActiveEffect::pTarget, target);
        answer(e, EFFECT_POINTER_007DF1D0, 0x5300);
        answer(e, FLAG_TEST_00403E60, 0);
        answer(e, MAGIC_HIT_EFFECT_INSTANTIATE, 0x5400);
        answer(e, TEST_008C7AA0, 0);
        answer_float(e, EFFECT_REAL_006B9130, 0.0);
        answer_float(e, EFFECT_REAL_007DF1F0, duration);
        answer(e, GETTER_00825C00, 0x5500);
        // fn_00804cd0 stops at once: no sound id.
        answer(e, GETTER_004FD3E0, 0);
        answer(e, BS_SOUND_HANDLE_IS_VALID, 1);
        answer(e, BS_SOUND_HANDLE_STOP, 0);
        this
    }

    #[test]
    fn fn_00804560_starts_an_active_effect_and_advances_it() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::fElapsedSeconds, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, START).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bStarted), 1);
        assert_eq!(e.get(this, ActiveEffect::pHitEffects), 0x5400);
        assert_eq!(e.get(this, ActiveEffect::fElapsedSeconds), 1.5);
        assert_eq!(calls(&e, UPDATE), vec![vec![this.addr(), 0.5f32.to_bits()]]);
        assert_eq!(e.get(this, ActiveEffect::bDone), 0);
        assert!(calls(&e, FINISH).is_empty());
        // The setting flag 0x8000000 was asked, then 0x80 for done.
        let flags: Vec<u32> = calls(&e, FLAG_TEST_00403E60).iter().map(|c| c[1]).collect();
        assert_eq!(flags, vec![0x0800_0000, 0x80]);
    }

    #[test]
    fn fn_00804560_clamps_to_the_duration_and_finishes() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 2.0);
        answer_float(&mut e, EFFECT_REAL_006B9130, 1.75);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::fElapsedSeconds, 1.75);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert!(calls(&e, START).is_empty());
        // Clamped: elapsed becomes the duration, the step what was left of it.
        assert_eq!(e.get(this, ActiveEffect::fElapsedSeconds), 2.0);
        assert_eq!(
            calls(&e, UPDATE),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(e.get(this, ActiveEffect::bDone), 1);
        assert_eq!(calls(&e, FINISH).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
        assert_eq!(
            calls(&e, BS_SOUND_HANDLE_STOP),
            vec![vec![this.addr() + PERSISTENT_SOUND]]
        );
    }

    #[test]
    fn fn_00804560_without_time_only_finishes_an_inactive_started_effect() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 2.0);
        e.set(this, ActiveEffect::bStarted, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.0f32]);
        assert!(calls(&e, UPDATE).is_empty());
        assert_eq!(calls(&e, FINISH).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
        // Already finished: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.0f32]);
        assert!(calls(&e, FINISH).is_empty());
    }

    #[test]
    fn fn_00804560_marks_done_for_an_actor_target() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        let actor = object(&mut e, &[(0x22c, ACTOR_SLOT_22C)]);
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 0xc, TARGET_IS_ACTOR);
        e.mem.set_u32(actor + 0x94, target_vtable);
        e.set(this, ActiveEffect::pTarget, actor + 0x94);
        answer(&mut e, TARGET_IS_ACTOR, 1);
        answer(&mut e, ACTOR_SLOT_22C, 1);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, ACTOR_SLOT_22C), vec![vec![actor, 1]]);
        assert_eq!(e.get(this, ActiveEffect::bDone), 1);
        assert_eq!(calls(&e, FINISH).len(), 1);
    }

    #[test]
    fn fn_00804560_runs_delayed_hit_effects_for_the_player_outside_menus() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        answer(&mut e, TARGET_REFERENCE, 0x0abc_0000);
        answer(&mut e, INTERFACE_IS_IN_MENU_MODE, 0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::iFlags, 0x60);
        e.set(this, ActiveEffect::pHitEffects, 0x5600);
        answer(&mut e, HIT_LIST_00470470, 0);
        answer(&mut e, HIT_LIST_004702F0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, HIT_LIST_00470470), vec![vec![0x5600]]);
        assert_eq!(calls(&e, HIT_LIST_004702F0), vec![vec![0x5600, 1]]);
        assert_eq!(e.get(this, ActiveEffect::pHitEffects), 0x5400);
        assert_eq!(e.get(this, ActiveEffect::iFlags), 0);
    }

    // ---- AdjustEffectiveness ----

    fn adjust_setup(e: &mut Engine, flag_180: u32, flag_100: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        e.map(ONE, 8);
        e.mem.set_f64(ONE, 1.0);
        answer(e, GETTER_0084E3A0, 0x5100);
        answer(e, GETTER_00825C00, 0x5200);
        answer(e, FLAG_TEST_00408680, flag_180);
        answer(e, FLAG_TEST_00403E60, flag_100);
        answer(e, SET_REAL_00693D50, 0);
        answer_float(e, EFFECT_REAL_007DF1F0, 4.0);
        answer_float(e, EFFECT_REAL_00495460, 6.0);
        e.register(FLOAT_FUNCTION_00404040, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn adjust_effectiveness_skips_flagged_settings_and_factor_one() {
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 1, 0);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(calls(&e, FLAG_TEST_00408680), vec![vec![0x5200, 0x180]]);
        assert!(calls(&e, FLAG_TEST_00403E60).is_empty());
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 1.0f32]);
        assert!(calls(&e, FLAG_TEST_00403E60).is_empty());
    }

    #[test]
    fn adjust_effectiveness_scales_the_duration_or_the_magnitude() {
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 1);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(
            calls(&e, SET_REAL_00693D50),
            vec![vec![this.addr(), 2.0f32.to_bits()]]
        );
        // Magnitude 6 * 0.5 = 3.
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 3.0);
        assert_eq!(calls(&e, FLOAT_FUNCTION_00404040).len(), 2);
        // 6 * 0.125 is below 1: the magnitude is 1.
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 0.125f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 1.0);
        assert_eq!(calls(&e, FLOAT_FUNCTION_00404040).len(), 1);
    }

    // ---- fn_00804cd0 ----

    const POSITION_SLOT: u32 = 0x0300_0400;

    fn sound_setup(e: &mut Engine, actor: u32, looping: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        answer(e, EFFECT_POINTER_007DF1D0, 0x5300);
        e.register(GETTER_004FD3E0, |_, a| Ret {
            eax: if a[0] == 0x5300 { 0x5310 } else { 0 },
            ..Ret::default()
        });
        answer(e, GETTER_0084E3A0, 0x77);
        let reference = object(e, &[(0x1f4, POSITION_SLOT)]);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, TARGET_REFERENCE, reference);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        let owner = object(e, &[(0x18, OWNER_KIND)]);
        answer(e, OWNER_KIND, 0);
        answer(e, GETTER_0044DDC0, owner);
        answer(e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(e, FLAG_TEST_00403E60, 0);
        let filled = e.mem.alloc(8);
        e.mem.set_u32(filled + 4, 0x99);
        answer(e, FILL_004E75D0, filled);
        answer(e, WORD_FUNCTION_005E39B0, looping);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_u32(position + 8, 0x7f80_0001);
        answer(e, POSITION_SLOT, position);
        for f in [
            SOUND_HANDLE_INIT_0041A250,
            ACTOR_SOUND_00933150,
            SOUND_HANDLE_ASSIGN_00418900,
            SOUND_HANDLE_DESTROY_00483710,
            BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID,
            BS_SOUND_HANDLE_SET_POSITION,
            BS_SOUND_HANDLE_PLAY,
            BS_SOUND_HANDLE_STOP,
            BS_SOUND_HANDLE_RELEASE,
        ] {
            answer(e, f, 0);
        }
        answer(e, BS_AUDIO_Q_INSTANCE, 0x5800);
        answer(e, BS_SOUND_HANDLE_IS_VALID, 1);
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn fn_00804cd0_needs_a_sound_id() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0, 0);
        answer(&mut e, GETTER_004FD3E0, 0);
        e.call(0x0080_4cd0, &args![this]);
        assert!(calls(&e, SOUND_HANDLE_INIT_0041A250).is_empty());
        assert!(calls(&e, GETTER_0084E3A0).is_empty());
    }

    #[test]
    fn fn_00804cd0_plays_through_the_actor_and_keeps_a_looping_sound() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0x6000, 1);
        e.call(0x0080_4cd0, &args![this]);
        let sound = calls(&e, SOUND_HANDLE_INIT_0041A250)[0][0];
        let played = calls(&e, ACTOR_SOUND_00933150);
        assert_eq!(played.len(), 1);
        assert_eq!(played[0][0], 0x6000);
        assert_eq!(played[0][2..], [0x77, 1, 0x112, 0]);
        assert_eq!(calls(&e, FILL_004E75D0)[0][0], 0x5310);
        assert_eq!(calls(&e, WORD_FUNCTION_005E39B0), vec![vec![0x99]]);
        let persistent = this.addr() + PERSISTENT_SOUND;
        assert_eq!(calls(&e, BS_SOUND_HANDLE_STOP), vec![vec![persistent]]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_RELEASE), vec![vec![persistent]]);
        let assigned = calls(&e, SOUND_HANDLE_ASSIGN_00418900);
        assert_eq!(assigned[0][0], sound);
        assert_eq!(assigned[1], vec![persistent, sound]);
        assert!(calls(&e, BS_AUDIO_Q_INSTANCE).is_empty());
    }

    #[test]
    fn fn_00804cd0_plays_at_the_reference_position_otherwise() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0, 0);
        e.call(0x0080_4cd0, &args![this]);
        let handle = calls(&e, BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID);
        assert_eq!(handle.len(), 1);
        assert_eq!(handle[0][0], 0x5800);
        assert_eq!(handle[0][2..], [0x77, 0x102]);
        let position = calls(&e, BS_SOUND_HANDLE_SET_POSITION);
        assert_eq!(
            position[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 0x7fc0_0001]
        );
        assert_eq!(calls(&e, BS_SOUND_HANDLE_PLAY)[0][1], 0);
        // Not looping: the persistent sound is left alone.
        assert!(calls(&e, BS_SOUND_HANDLE_STOP).is_empty());
        assert!(calls(&e, ACTOR_SOUND_00933150).is_empty());
    }

    // ---- saving ----

    const KIND_SLOT: u32 = 0x0300_0500;
    const SAVE_SLOT: u32 = 0x0300_0504;

    fn save_engine() -> Engine {
        let mut e = Engine::new();
        for f in [
            SAVE_DATA_00865E50,
            SAVE_FORM_ID,
            SAVE_FORM_ID_OV2,
            SAVE_VARIABLE_SIZED_VALUE,
            SAVE_VARIABLE_SIZED_VALUE_OV2,
            SAVE_SLOT,
        ] {
            answer(&mut e, f, 0);
        }
        answer(&mut e, START_VARIABLE_SIZED_VALUE, 0x88);
        answer(&mut e, MAGIC_CASTER_GET_FORM_ID, 0x0c00);
        answer(&mut e, MAGIC_TARGET_GET_FORM_ID, 0x0d00);
        answer(&mut e, MAGIC_ITEM_GET_FORM_ID, 0x0e00);
        answer(&mut e, KIND_SLOT, 3);
        e.register(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e
    }

    /// A list of `items` (8-byte nodes: item, next).
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn save_game_writes_fields_ids_and_hit_effects() {
        let mut e = save_engine();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::pTarget, 0x2400);
        e.set(this, ActiveEffect::pSource, 0x3c00);
        let hit = object(&mut e, &[(0x9c, KIND_SLOT), (0xa8, SAVE_SLOT)]);
        let list = list_of(&mut e, &[hit, 0, hit]);
        e.set(this, ActiveEffect::pHitEffects, list);
        e.call_log = Some(vec![]);
        e.call(0x0080_6230, &args![this, 0xb000u32]);
        let base = this.addr();
        let data = calls(&e, SAVE_DATA_00865E50);
        let fields: Vec<(u32, u32)> = data[..9].iter().map(|c| (c[1] - base, c[2])).collect();
        assert_eq!(
            fields,
            vec![
                (4, 4),
                (0x10, 1),
                (0x11, 1),
                (0x12, 1),
                (0x13, 1),
                (0x14, 1),
                (0x1c, 4),
                (0x20, 4),
                (0x18, 4)
            ]
        );
        assert_eq!(data.len(), 11);
        assert_eq!(data[9][2], 1);
        assert_eq!(
            calls(&e, SAVE_FORM_ID),
            vec![
                vec![0xb000, 0, 0],
                vec![0xb000, 0x0d00, 0],
                vec![0xb000, 0, 0]
            ]
        );
        assert!(calls(&e, MAGIC_CASTER_GET_FORM_ID).is_empty());
        assert_eq!(calls(&e, MAGIC_TARGET_GET_FORM_ID), vec![vec![0x2400]]);
        assert_eq!(calls(&e, SAVE_FORM_ID_OV2), vec![vec![0xb000, 0x3c00, 0]]);
        assert_eq!(
            calls(&e, SAVE_SLOT),
            vec![vec![hit, 0xb000], vec![hit, 0xb000]]
        );
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE_OV2),
            vec![vec![0xb000, 2, 0x88]]
        );
    }

    #[test]
    fn save_effect_and_effect_list_write_each_effect() {
        let mut e = save_engine();
        let first = effect(&mut e, &[(0x14, SAVE_SLOT)]);
        e.set(first, ActiveEffect::pSpell, 0x7000);
        e.set(first, ActiveEffect::pEffect, 0x7100);
        let second = effect(&mut e, &[(0x14, SAVE_SLOT)]);
        e.set(second, ActiveEffect::pSpell, 0);
        e.set(second, ActiveEffect::pEffect, 0x7200);
        answer(&mut e, EFFECT_ITEM_LIST_GET_INDEX, 0x105);
        answer(&mut e, GETTER_00825C00, 0x7300);
        answer(&mut e, GETTER_00810FD0, 0x42);
        let mut position = 0x100u32;
        e.register_double(GETTER_0084E3A0, move |_, _| {
            position += 0x10;
            Ret {
                eax: position,
                ..Ret::default()
            }
        });
        let list = list_of(&mut e, &[first.addr(), 0, second.addr()]);
        e.call_log = Some(vec![]);
        e.call(0x0080_6a10, &args![0xb000u32, list]);
        assert_eq!(
            calls(&e, SAVE_FORM_ID),
            vec![vec![0xb000, 0x0e00, 0], vec![0xb000, 0, 0]]
        );
        let index = calls(&e, EFFECT_ITEM_LIST_GET_INDEX);
        assert_eq!(index, vec![vec![0x700c, 0x7100], vec![0xc, 0x7200]]);
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0xb000, 0x42], vec![0xb000, 0x42]]
        );
        assert_eq!(
            calls(&e, SAVE_SLOT),
            vec![vec![first.addr(), 0xb000], vec![second.addr(), 0xb000]]
        );
        // Each effect's value closes with its byte count; the list's with the count.
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE_OV2),
            vec![
                vec![0xb000, 0x10, 0x88],
                vec![0xb000, 0x10, 0x88],
                vec![0xb000, 2, 0x88]
            ]
        );
    }
}
