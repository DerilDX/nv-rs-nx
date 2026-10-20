//! `fallout/ai/run_008c51f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c5e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the record `ActorPackage::Clear` (`008c5ef0`) resets: clears the words at
/// `+0`, `+4` and `+8`, sets the word at `+0xc` to `0xffffffff`, the float at `+0x10` to the
/// float at `0x01012054` and clears the word at `+0x14`. `ECX` is the object and the answer is
/// the object. The engine map gives it no name.
pub fn fn_008c5e20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x10, value);
    e.mem.set_u32(base + 0xc, 0xffff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    this
}

/// `00441110` (`thiscall`): the word at +0x1c of a stage record (its
/// effect; the engine map's name `PathingLocation::GetWorldspace` is
/// folded code).
const STAGE_EFFECT: u32 = 0x0044_1110;
/// `MagicTarget::Dispel` (engine map), `00824400` (`RET 0xc`).
const MAGIC_TARGET_DISPEL: u32 = 0x0082_4400;
/// `MagicTarget::UpdateTarget` (engine map), `00823c40` (`RET 4`).
const MAGIC_TARGET_UPDATE: u32 = 0x0082_3c40;
/// `00403df0` (`thiscall`): a setting's string (+4; null for a null
/// setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `00408da0` (`thiscall`; engine map `MapMarkerData::GetLocationName`): the
/// name the message shows for the stage's effect (+0x18 of it).
const EFFECT_NAME: u32 = 0x0040_8da0;
/// `sprintf` (engine map, `cdecl`, variadic), `00ec623a`.
const SPRINTF: u32 = 0x00ec_623a;
/// `007052f0` (`cdecl`, six words: text, 0, icon path, 0, `float` time, 0):
/// shows a HUD message (unit `fallout/interface/interface.cpp`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The player (`PlayerCharacter*`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `float` 2.0 (`010162c0`), the message's display time.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `double` 0.0 (`01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_in_pain.dds"`.
const ICON_IN_PAIN: u32 = 0x0104_bfe8;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_very_happy.dds"`.
const ICON_VERY_HAPPY: u32 = 0x0103_a830;
/// `"%s %s"`.
const NAME_FORMAT: u32 = 0x0101_2058;
/// The setting whose string prefixes the new stage's name (`011d3ebc`),
/// shared by the three handlers.
const STAGE_PREFIX_SETTING: u32 = 0x011d_3ebc;

/// What differs between the three stage-change handlers: the function
/// that finds the stage record for a value, and the settings whose strings
/// are shown.
struct StageMessages {
    /// `cdecl` (one `float`): the first stage record whose threshold the
    /// value reaches, 0 for none.
    stage_of: u32,
    /// Shown when `delta` is above 0 (with the in-pain icon).
    rise: u32,
    /// Shown otherwise (with the very-happy icon).
    fall: u32,
    /// Shown when no stage remains (with the very-happy icon).
    no_stage: u32,
}

/// The body the three handlers share (they differ only in
/// [`StageMessages`]): for an actor (`owner` - 0xA4: `owner` is its
/// `ActorValueOwner` subobject) whose virtual slot 0x360 answers true, the
/// stage of the value before (`old`) and of the current value (virtual slot
/// 0xC of the owner) are compared. When they differ, the old stage's effect
/// (+0x18 of `00441110`'s result, 0 for none) is dispelled from the actor's
/// `MagicTarget` (+0x94) and the new stage's effect cast through its
/// `MagicCaster` (+0x88, virtual slot 8, with 0). For the player two HUD
/// messages follow (`007052f0`, 2 seconds): the rise or fall text by the
/// sign of `delta`, then `"%s %s"` of the prefix and the new stage's
/// effect name, or the no-stage text; then `MagicTarget::UpdateTarget(0)`.
fn stage_changed(
    e: &mut Engine,
    messages: &StageMessages,
    owner: u32,
    actor_value: u32,
    old: f32,
    delta: f32,
) {
    let actor = if owner != 0 {
        owner.wrapping_sub(0xa4)
    } else {
        0
    };
    if actor == 0 || !e.vcall(actor, 0x360, &args![]).bool() {
        return;
    }
    let value = e.vcall(actor + 0xa4, 0xc, &args![actor_value]).f64();
    let value_bits = x87_store32(value);
    let old_stage = e
        .call(messages.stage_of, &args![x87_quiet_bits(old.to_bits())])
        .u32();
    let new_stage = e
        .call(messages.stage_of, &args![x87_quiet_bits(value_bits)])
        .u32();
    if old_stage == new_stage {
        return;
    }
    if old_stage != 0 {
        let effect = e.call(STAGE_EFFECT, &args![old_stage]).u32();
        let target = if effect != 0 { effect + 0x18 } else { 0 };
        e.call(
            MAGIC_TARGET_DISPEL,
            &args![actor + 0x94, target, 0u32, 0u32],
        );
    }
    if new_stage != 0 {
        let effect = e.call(STAGE_EFFECT, &args![new_stage]).u32();
        e.vcall(actor + 0x88, 8, &args![effect, 0u32]);
    }
    if actor != e.mem.u32(PLAYER) {
        return;
    }
    e.with_stack(0x104, |e, text| {
        let show = |e: &mut Engine, icon: u32| {
            let time = x87_store32(x87_load32(e.mem.u32(MESSAGE_TIME)));
            e.call(SHOW_MESSAGE, &args![text, 0u32, icon, 0u32, time, 0u32]);
        };
        // FCOMP 0.0; TEST AH,0x41; JNZ: below, equal or unordered fall.
        let zero = e.mem.f64(ZERO_DOUBLE);
        if x87_load32(delta.to_bits()) > zero {
            let format = e.call(SETTING_STRING, &args![messages.rise]).u32();
            e.call(SPRINTF, &args![text, format]);
            show(e, ICON_IN_PAIN);
        } else {
            let format = e.call(SETTING_STRING, &args![messages.fall]).u32();
            e.call(SPRINTF, &args![text, format]);
            show(e, ICON_VERY_HAPPY);
        }
        if new_stage != 0 {
            let effect = e.call(STAGE_EFFECT, &args![new_stage]).u32();
            let name = e.call(EFFECT_NAME, &args![effect.wrapping_add(0x18)]).u32();
            let prefix = e.call(SETTING_STRING, &args![STAGE_PREFIX_SETTING]).u32();
            e.call(SPRINTF, &args![text, NAME_FORMAT, prefix, name]);
            show(e, ICON_IN_PAIN);
        } else {
            let format = e.call(SETTING_STRING, &args![messages.no_stage]).u32();
            e.call(SPRINTF, &args![text, format]);
            show(e, ICON_VERY_HAPPY);
        }
        e.call(MAGIC_TARGET_UPDATE, &args![actor + 0x94, x87_store32(0.0)]);
    });
}

// Translated from 008c5610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c5610 - the /GS cookie check (00ec408c) of its frame is not translated
/// A stage-change handler (`cdecl`: owner, actor value, old value, delta;
/// no Xbox PDB name), the body in [`stage_changed`]: stages by `004018b0`,
/// texts from the settings at `011d2f40` (rise), `011d4b7c` (fall) and
/// `011d4fd8` (no stage).
pub fn fn_008c5610(e: &mut Engine, owner: u32, actor_value: u32, old: f32, delta: f32) {
    let messages = StageMessages {
        stage_of: 0x0040_18b0,
        rise: 0x011d_2f40,
        fall: 0x011d_4b7c,
        no_stage: 0x011d_4fd8,
    };
    stage_changed(e, &messages, owner, actor_value, old, delta);
}

// Translated from 008c5890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c5890 - the /GS cookie check (00ec408c) of its frame is not translated
/// A stage-change handler (`cdecl`: owner, actor value, old value, delta;
/// no Xbox PDB name), the body in [`stage_changed`]: stages by `00402030`,
/// texts from the settings at `011d1fd4` (rise), `011d4c6c` (fall) and
/// `011d3a14` (no stage).
pub fn fn_008c5890(e: &mut Engine, owner: u32, actor_value: u32, old: f32, delta: f32) {
    let messages = StageMessages {
        stage_of: 0x0040_2030,
        rise: 0x011d_1fd4,
        fall: 0x011d_4c6c,
        no_stage: 0x011d_3a14,
    };
    stage_changed(e, &messages, owner, actor_value, old, delta);
}

// Translated from 008c5b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c5b10 - the /GS cookie check (00ec408c) of its frame is not translated
/// A stage-change handler (`cdecl`: owner, actor value, old value, delta;
/// no Xbox PDB name), the body in [`stage_changed`]: stages by `00402760`,
/// texts from the settings at `011d2fac` (rise), `011d33b4` (fall) and
/// `011d40d8` (no stage).
pub fn fn_008c5b10(e: &mut Engine, owner: u32, actor_value: u32, old: f32, delta: f32) {
    let messages = StageMessages {
        stage_of: 0x0040_2760,
        rise: 0x011d_2fac,
        fall: 0x011d_33b4,
        no_stage: 0x011d_40d8,
    };
    stage_changed(e, &messages, owner, actor_value, old, delta);
}

// Translated from 008c5d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c5d90 - the /GS cookie check (00ec408c) of its aligned frame is not translated
/// No Xbox PDB name (`thiscall`, `RET 4`): in a local of the exe's frame
/// (`006815c0` called on it first), `004a3e00(local, value)` builds a
/// vector, and its result goes to [`fn_008c5df0`] on `this`.
pub fn fn_008c5d90(e: &mut Engine, this: Ptr, value: u32) {
    e.with_stack(0x18, |e, local| {
        e.call(0x0068_15c0, &args![local]);
        let built = e.call(0x004a_3e00, &args![local, value]).u32();
        fn_008c5df0(e, this, built);
    });
}

// Translated from 008c5df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (`thiscall`, `RET 4`; Ghidra's library match
/// `OnStatusTextChange` is wrong): the object `004ae750` gives for `this`
/// (0 for a null `this`), when not null, gets `00cbb150(value)`.
pub fn fn_008c5df0(e: &mut Engine, this: Ptr, value: u32) {
    let object = e.call(0x004a_e750, &args![this]).u32();
    if object != 0 {
        e.call(0x00cb_b150, &args![object, value]);
    }
}

// Translated from 008c5e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: deletes the object at `this` +4 when there is one
/// (virtual slot 0 with 1, the deleting destructor); the pointer is left
/// as it is.
pub fn fn_008c5e70(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 4) != 0 {
        let object = e.mem.u32(this.addr() + 4);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_5e20, fn_008c5e20(Ptr) -> Ptr),
        entry!(0x008c5610, fn_008c5610(u32, u32, f32, f32)),
        entry!(0x008c5890, fn_008c5890(u32, u32, f32, f32)),
        entry!(0x008c5b10, fn_008c5b10(u32, u32, f32, f32)),
        entry!(0x008c5d90, fn_008c5d90(Ptr, u32)),
        entry!(0x008c5df0, fn_008c5df0(Ptr, u32)),
        entry!(0x008c5e70, fn_008c5e70(Ptr)),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test;

    #[test]
    fn fn_008c5e20_sets_the_start_values() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008c_5e20,
            size: 0x18,
            globals: &[(0x0101_2054, 0x3f80_0000)],
            returns: &[],
            calls: &[],
            stores: &[
                (0x0, 4, 0),
                (0x4, 4, 0),
                (0x8, 4, 0),
                (0xc, 4, 0xffff_ffff),
                (0x10, 4, 0x3f80_0000),
                (0x14, 4, 0),
            ],
        });
    }
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    const STAGE_A: u32 = 0x0040_18b0;

    struct Actor {
        actor: u32,
        owner: u32,
    }

    /// An actor whose slot 0x360 answers `active` and whose owner's slot
    /// 0xC answers `value`; the stage function answers stage 0x51 at 5 and
    /// above, 0x52 at 10 and above, 0 below 5.
    fn engine(active: bool, value: f32) -> (Engine, Actor) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(MESSAGE_TIME, 2.0f32);
        let actor = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + 0x360, 0x00ab_0360);
        e.register_double(0x00ab_0360, move |_, _| Ret {
            eax: active as u32,
            ..Ret::default()
        });
        let owner_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00ab_000c);
        e.register_double(0x00ab_000c, move |_, _| Ret {
            st0: f64::from(value),
            ..Ret::default()
        });
        let caster_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0x88, caster_vtable);
        e.mem.set_u32(caster_vtable + 8, 0x00ab_0008);
        e.register(0x00ab_0008, |_, _| Ret::default());
        e.register(STAGE_A, |_, a| {
            let v = f32::from_bits(a[0]);
            let stage = if v >= 10.0 {
                0x52
            } else if v >= 5.0 {
                0x51
            } else {
                0
            };
            Ret {
                eax: stage,
                ..Ret::default()
            }
        });
        e.register(STAGE_EFFECT, |_, a| Ret {
            eax: a[0] << 8,
            ..Ret::default()
        });
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(EFFECT_NAME, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        for address in [
            MAGIC_TARGET_DISPEL,
            MAGIC_TARGET_UPDATE,
            SPRINTF,
            SHOW_MESSAGE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (
            e,
            Actor {
                actor,
                owner: actor + 0xa4,
            },
        )
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn an_inactive_actor_or_the_same_stage_does_nothing() {
        let (mut e, a) = engine(false, 7.0);
        e.call(0x008c_5610, &args![a.owner, 3u32, 1.0f32, 6.0f32]);
        assert!(calls(&e, STAGE_A).is_empty());
        let (mut e, a) = engine(true, 7.0);
        e.call(0x008c_5610, &args![a.owner, 3u32, 6.0f32, 1.0f32]);
        assert_eq!(calls(&e, STAGE_A).len(), 2);
        assert!(calls(&e, MAGIC_TARGET_DISPEL).is_empty());
        e.call(0x008c_5610, &args![0u32, 3u32, 6.0f32, 1.0f32]);
        assert_eq!(calls(&e, STAGE_A).len(), 2);
    }

    #[test]
    fn a_new_stage_swaps_the_effects_and_messages_the_player() {
        let (mut e, a) = engine(true, 11.0);
        e.set_global(PLAYER, a.actor);
        e.call(0x008c_5610, &args![a.owner, 3u32, 6.0f32, 5.0f32]);
        assert_eq!(
            calls(&e, MAGIC_TARGET_DISPEL),
            vec![vec![a.actor + 0x94, (0x51 << 8) + 0x18, 0, 0]]
        );
        assert_eq!(
            calls(&e, 0x00ab_0008),
            vec![vec![a.actor + 0x88, 0x52 << 8, 0]]
        );
        let sprintf = calls(&e, SPRINTF);
        assert_eq!(sprintf[0][1], 0x011d_2f40 + 4);
        assert_eq!(
            &sprintf[1][1..],
            &[
                NAME_FORMAT,
                STAGE_PREFIX_SETTING + 4,
                (0x52 << 8) + 0x18 + 1
            ]
        );
        let shown: Vec<u32> = calls(&e, SHOW_MESSAGE).iter().map(|w| w[2]).collect();
        assert_eq!(shown, vec![ICON_IN_PAIN, ICON_IN_PAIN]);
        assert_eq!(calls(&e, SHOW_MESSAGE)[0][4], 2.0f32.to_bits());
        assert_eq!(
            calls(&e, MAGIC_TARGET_UPDATE),
            vec![vec![a.actor + 0x94, 0]]
        );
    }

    #[test]
    fn leaving_every_stage_shows_the_fall_and_no_stage_texts() {
        let (mut e, a) = engine(true, 1.0);
        e.set_global(PLAYER, a.actor);
        e.call(0x008c_5610, &args![a.owner, 3u32, 6.0f32, -5.0f32]);
        assert!(calls(&e, 0x00ab_0008).is_empty());
        let formats: Vec<u32> = calls(&e, SPRINTF).iter().map(|w| w[1]).collect();
        assert_eq!(formats, vec![0x011d_4b7c + 4, 0x011d_4fd8 + 4]);
        let shown: Vec<u32> = calls(&e, SHOW_MESSAGE).iter().map(|w| w[2]).collect();
        assert_eq!(shown, vec![ICON_VERY_HAPPY, ICON_VERY_HAPPY]);
    }

    #[test]
    fn other_actors_get_no_messages() {
        let (mut e, a) = engine(true, 11.0);
        e.call(0x008c_5610, &args![a.owner, 3u32, 1.0f32, 10.0f32]);
        assert_eq!(calls(&e, 0x00ab_0008).len(), 1);
        assert!(calls(&e, MAGIC_TARGET_DISPEL).is_empty());
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn the_other_two_handlers_use_their_own_stages_and_texts() {
        for (address, stage_of, rise) in [
            (0x008c_5890u32, 0x0040_2030u32, 0x011d_1fd4u32),
            (0x008c_5b10, 0x0040_2760, 0x011d_2fac),
        ] {
            let (mut e, a) = engine(true, 11.0);
            e.set_global(PLAYER, a.actor);
            e.register(stage_of, |_, a| Ret {
                eax: if f32::from_bits(a[0]) > 8.0 { 7 } else { 0 },
                ..Ret::default()
            });
            e.call(address, &args![a.owner, 3u32, 1.0f32, 10.0f32]);
            assert_eq!(calls(&e, stage_of).len(), 2);
            assert_eq!(calls(&e, SPRINTF)[0][1], rise + 4);
        }
    }

    #[test]
    fn vector_built_in_a_local_goes_to_the_forwarder() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x004a_3e00, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x004a_e750, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(0x00cb_b150, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008c_5d90, &args![0x3000u32, 0x4000u32]);
        let built = calls(&e, 0x004a_3e00);
        assert_eq!(built[0][1], 0x4000);
        assert_eq!(calls(&e, 0x00cb_b150), vec![vec![0x3100, built[0][0]]]);
    }

    #[test]
    fn forwarder_skips_a_null_object() {
        let mut e = Engine::new();
        e.register(0x004a_e750, |_, _| Ret::default());
        e.register(0x00cb_b150, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008c_5df0, &args![0x3000u32, 0x4000u32]);
        assert!(calls(&e, 0x00cb_b150).is_empty());
    }

    #[test]
    fn deletes_the_held_object_only_when_present() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, 0x00ab_0000);
        e.register(0x00ab_0000, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x008c_5e70, &args![this]);
        assert!(calls(&e, 0x00ab_0000).is_empty());
        e.mem.set_u32(this + 4, object);
        e.call(0x008c_5e70, &args![this]);
        assert_eq!(calls(&e, 0x00ab_0000), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(this + 4), object);
    }
}
