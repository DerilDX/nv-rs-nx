//! `fallout/ai/combat/hitdata.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108F9E8: u32 = 0x0108_f9e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 009b7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009b7970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108F9E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `00761540` (unnamed in the engine map, `creditsmenu.cpp`): reserves the next slot of a
/// `BSSimpleArray`, growing the buffer when it is full, and returns the new slot's index.
const ARRAY_RESERVE_SLOT: u32 = 0x0076_1540;
/// `00726bf0` (unnamed in the engine map): `thiscall`, `RET 8`; constructs `count` four-byte
/// elements starting at the address it is given.
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0072_6bf0;

// Translated from 007cb2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart *, 1024>::AddUninitialized` (name from the engine map; the body
/// reserves a slot through `00761540`, constructs one element in it through `00726bf0`, then
/// copies `*item` into the slot, which is what an `Add` of one pointer does, so the name is
/// unconfirmed). `thiscall`, `RET 4`; `item` points to the pointer to store. Returns the slot's
/// index. The buffer pointer is read again after each call, as the exe does.
pub fn fn_007cb2e0(e: &mut Engine, this: Ptr<BSSimpleArray>, item: Ptr) -> u32 {
    let index = e.call(ARRAY_RESERVE_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let value = e.mem.u32(item.addr());
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), value);
    index
}

layout! {
    /// `HitData` (Xbox PDB), 0x64 bytes on PC as on the Xbox (the offsets below are the ones the
    /// PC code uses).
    pub struct HitData: 0x64 {
        /// `pAggressor` (Xbox PDB): `Actor *`.
        0x00 pAggressor: u32,
        /// `pTarget` (Xbox PDB): `Actor *`.
        0x04 pTarget: u32,
        /// `pSourceRef` (Xbox PDB): `TESObjectREFR *` (the explosion, for explosion hits).
        0x08 pSourceRef: u32,
        /// `eAttackSkill` (Xbox PDB): `ActorValue::Index`.
        0x0C eAttackSkill: u32,
        /// `eDamageLimb` (Xbox PDB): `BGSBodyPart::LIMB_ENUM`.
        0x10 eDamageLimb: u32,
        /// `fHealthDamage` (Xbox PDB).
        0x14 fHealthDamage: u32,
        /// `fTotalDamage` (Xbox PDB).
        0x18 fTotalDamage: u32,
        /// `fFatigueDamage` (Xbox PDB).
        0x1C fFatigueDamage: u32,
        /// `fTargetedLimbDamage` (Xbox PDB).
        0x20 fTargetedLimbDamage: u32,
        /// `fPercentBlocked` (Xbox PDB).
        0x24 fPercentBlocked: u32,
        /// `fArmorDamage` (Xbox PDB).
        0x28 fArmorDamage: u32,
        /// `fDamageToWeapon` (Xbox PDB).
        0x2C fDamageToWeapon: u32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP *`.
        0x30 pWeapon: u32,
        /// `fWeaponHealth` (Xbox PDB).
        0x34 fWeaponHealth: u32,
        /// `pCriticalEffect` (Xbox PDB): `SpellItem *`.
        0x50 pCriticalEffect: u32,
        /// `pVATSCommand` (Xbox PDB): `VATS_COMMAND *`.
        0x54 pVATSCommand: u32,
        /// `uiFlags` (Xbox PDB).
        0x58 uiFlags: u32,
        /// `fBonusMult` (Xbox PDB).
        0x5C fBonusMult: u32,
        /// `iRefCount` (Xbox PDB).
        0x60 iRefCount: u32,
    }
}

/// `HitData::kHitLocation` (Xbox PDB), an `NiPoint3` at `+0x38`.
const HIT_LOCATION: u32 = 0x38;
/// `HitData::kHitDirection` (Xbox PDB), an `NiPoint3` at `+0x44`.
const HIT_DIRECTION: u32 = 0x44;
/// `00407e00` (unnamed in the engine map): `thiscall`, `RET 8`; sets (`value` non-zero) or
/// clears the bits `mask` of `uiFlags`.
const SET_FLAG: u32 = 0x0040_7e00;
/// `0058cba0` (unnamed in the engine map): `thiscall`, `RET 4`; whether any bit of `mask` is set.
const IS_FLAG_SET: u32 = 0x0058_cba0;
/// `00403e20` (unnamed in the engine map, `effectitem.cpp`): on a game setting, the address of
/// its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `BGSEntryPoint::HandleEntryPoint` (`005e58f0`, `cdecl`, five words here).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The default weapon (the unarmed weapon form) at `011ca278`.
const DEFAULT_WEAPON_011CA278: u32 = 0x011c_a278;
/// The player character.
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// `0.0` (`double`).
const ZERO_01012060: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_01012070: u32 = 0x0101_2070;
/// `100.0` (`double`).
const HUNDRED_01017A40: u32 = 0x0101_7a40;
/// `100.0` (`float`).
const HUNDRED_01016410: u32 = 0x0101_6410;
/// `10.0` (`double`).
const TEN_01020758: u32 = 0x0102_0758;
/// `0.05` (`double`, the `float` 0.05 widened).
const FIVE_HUNDREDTHS_0103C7C8: u32 = 0x0103_c7c8;
/// `_ftol2` (`00ec62c0`): the value in `ST0` (an `f64` argument) truncated, in `EAX`.
const FTOL: u32 = 0x00ec_62c0;

/// The x87 condition bits `FCOM` leaves in `AH` (after `FNSTSW AX`) for `st0` compared with
/// `st1`: C0 (0x01) for less, C3 (0x40) for equal, C3|C2|C0 (0x45) unordered, none for greater.
fn fpu_compare(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 < st1 {
        0x01
    } else if st0 == st1 {
        0x40
    } else {
        0
    }
}

/// The parity flag of `TEST AH,mask`: set when the masked byte has an even number of bits.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// `FCOMP` against 0 then `TEST AH,0x41` / `JNZ`: the branch is skipped unless `value > 0`.
fn above_zero(e: &Engine, value: f64) -> bool {
    fpu_compare(value, double_at(e, ZERO_01012060)) & 0x41 == 0
}

/// `FCOMP` against 0 then `TEST AH,0x41` / `JP`: taken (not skipped) when `value <= 0`.
fn at_most_zero(e: &Engine, value: f64) -> bool {
    !parity_even(fpu_compare(value, double_at(e, ZERO_01012060)) & 0x41)
}

fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// The `float` value of the game setting `setting` (through `00403e20`), as its bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

fn field(e: &Engine, this: Ptr<HitData>, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr<HitData>, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

fn set_flag(e: &mut Engine, this: Ptr<HitData>, mask: u32, value: u32) {
    e.call(SET_FLAG, &args![this, mask, value]);
}

/// The weapon, or the default weapon (`011ca278`) when the hit has none.
fn weapon_or_default(e: &Engine, this: Ptr<HitData>) -> u32 {
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0 {
        weapon
    } else {
        e.mem.u32(DEFAULT_WEAPON_011CA278)
    }
}

/// The weapon's current ammunition's effect list for the aggressor
/// (`TESObjectWEAP::GetCurrentAmmo(weapon, aggressor)`, `00525980`, then `00923000` on it), or
/// 0 without ammunition.
fn ammo_effects(e: &mut Engine, this: Ptr<HitData>) -> u32 {
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    if e.call(0x0052_5980, &args![weapon, aggressor]).u32() == 0 {
        return 0;
    }
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    let ammo = e.call(0x0052_5980, &args![weapon, aggressor]).u32();
    e.call(0x0092_3000, &args![ammo]).u32()
}

/// `0059a030(kind, effects, value)` (`cdecl`): the ammunition effects of `kind` applied to the
/// `float` `value`, stored as a `float`.
fn apply_ammo_effects(e: &mut Engine, kind: u32, effects: u32, value: u32) -> u32 {
    x87_store32(
        e.call(0x0059_a030, &args![kind, effects, x87_quiet_bits(value)])
            .f64(),
    )
}

/// The `1 + 0.05 * GetClampedActorFloatValue(player, 8)` factor of `ReduceDamage` (the player's
/// `ActorValueOwner` at `+0xa4`), both steps stored as `float`s.
fn player_factor(e: &mut Engine) -> u32 {
    let player = e.mem.u32(PLAYER_011DEA3C);
    let value = x87_store32(
        e.call(0x0066_ef50, &args![player.wrapping_add(0xa4), 8u32])
            .f64(),
    );
    let scaled = x87_mul(x87_load32(value), double_at(e, FIVE_HUNDREDTHS_0103C7C8));
    x87_store32(x87_add(scaled, 1.0))
}

/// The target's actor value `index` through its `ActorValueOwner` (`+0xa4`) vtable slot `0xc`
/// (`ST0`).
fn target_actor_value(e: &mut Engine, target: u32, index: u32) -> f64 {
    e.vcall(target.wrapping_add(0xa4), 0xc, &args![index]).f64()
}

// Translated from 009a4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart *,1024>::_MoveItems` (Xbox PDB), `RET 8`: drops `count` items
/// at `index`: the items after them (`0042f5a0(size - index - count, count)` of them) are
/// destroyed (`0072ba80(this, buffer + index, count)`) and moved down (`0042fb60(this, buffer +
/// index, buffer + size - moved, moved)`), and the size (`+8`) shrinks by `count`.
pub fn bs_simple_array_body_part_move_items(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8);
    let moved = e
        .call(
            0x0042_f5a0,
            &args![size.wrapping_sub(index).wrapping_sub(count), count],
        )
        .u32();
    let buffer = e.mem.u32(base + 4);
    e.call(
        0x0072_ba80,
        &args![this, buffer.wrapping_add(index.wrapping_mul(4)), count],
    );
    let size = e.mem.u32(base + 8);
    let buffer = e.mem.u32(base + 4);
    let source = buffer
        .wrapping_add(size.wrapping_mul(4))
        .wrapping_sub(moved.wrapping_shl(2));
    let destination = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(0x0042_fb60, &args![this, destination, source, moved]);
    let size = e.mem.u32(base + 8);
    e.mem.set_u32(base + 8, size.wrapping_sub(count));
}

// Translated from 009b4d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::HitData` (Xbox PDB): runs `006815c0` on the two points (`+0x38`, `+0x44`; the
/// engine map's name for that folded body does not fit) and `HitData::ClearData`. Returns
/// `this`.
pub fn hit_data_constructor(e: &mut Engine, this: Ptr<HitData>) -> Ptr<HitData> {
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(HIT_LOCATION)]);
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(HIT_DIRECTION)]);
    hit_data_clear_data(e, this);
    this
}

// Translated from 009b4dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::ClearData` (Xbox PDB): clears every field; `eAttackSkill` becomes 0x4d and
/// `eDamageLimb` -1, both points the three words at `011f426c`. `iRefCount` is cleared too.
pub fn hit_data_clear_data(e: &mut Engine, this: Ptr<HitData>) {
    e.set(this, HitData::pAggressor, 0);
    e.set(this, HitData::pTarget, 0);
    e.set(this, HitData::pSourceRef, 0);
    e.set(this, HitData::eAttackSkill, 0x4d);
    e.set(this, HitData::eDamageLimb, u32::MAX);
    for offset in (0x14..=0x2c).step_by(4) {
        set_field(e, this, offset, 0);
    }
    for point in [HIT_LOCATION, HIT_DIRECTION] {
        for word in 0..3 {
            let value = e.mem.u32(0x011f_426c + 4 * word);
            set_field(e, this, point + 4 * word, value);
        }
    }
    e.set(this, HitData::pWeapon, 0);
    e.set(this, HitData::fWeaponHealth, 0);
    e.set(this, HitData::pCriticalEffect, 0);
    e.set(this, HitData::pVATSCommand, 0);
    e.set(this, HitData::uiFlags, 0);
    e.set(this, HitData::iRefCount, 0);
    e.set(this, HitData::fBonusMult, 0);
}

// Translated from 009b4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::Copy` (Xbox PDB), `RET 4`: copies every field of `other` but `iRefCount` (the
/// `float`s through `FLD`/`FSTP`, which quiets a signalling NaN).
pub fn hit_data_copy(e: &mut Engine, this: Ptr<HitData>, other: Ptr<HitData>) {
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    for offset in (0x14..=0x2c).step_by(4) {
        let value = x87_quiet_bits(field(e, other, offset));
        set_field(e, this, offset, value);
    }
    for offset in (HIT_LOCATION..HIT_LOCATION + 0x18).step_by(4) {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    let weapon = field(e, other, 0x30);
    set_field(e, this, 0x30, weapon);
    let health = x87_quiet_bits(field(e, other, 0x34));
    set_field(e, this, 0x34, health);
    for offset in [0x50u32, 0x54, 0x58] {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    let bonus = x87_quiet_bits(field(e, other, 0x5c));
    set_field(e, this, 0x5c, bonus);
}

// Translated from 009b5770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeExplosionData` (Xbox PDB), `cdecl` (`hit`, `aggressor`, `target`,
/// `explosion`, `damage`). Nothing for a null `hit`. Stores the aggressor and target and sets
/// flag 0x2000. An explosion (vtable slot `0x220` true) gives the weapon (`009b0900`) with
/// health 1.0; when the explosion's owner (`009acce0`) is a character (vtable slot `0x100`)
/// holding that weapon (`Actor::GetCurrentWeapon`), the health is
/// `ItemChange::GetItemHealth(item, 1)` of the owner's process's vtable slot `0x148` item (1.0
/// without one). Then the source is the explosion, both damages the `damage` and the hit runs
/// `ReduceDamage(1)`, `CheckCriticalStrike` and `DetermineOutcome`.
pub fn hit_data_initialize_explosion_data(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    explosion: u32,
    damage: f32,
) {
    if hit.is_null() {
        return;
    }
    e.set(hit, HitData::pAggressor, aggressor);
    e.set(hit, HitData::pTarget, target);
    set_flag(e, hit, 0x2000, 1);
    if explosion != 0 && e.vcall(explosion, 0x220, &args![]).bool() {
        let weapon = e.call(0x009b_0900, &args![explosion]).u32();
        e.set(hit, HitData::pWeapon, weapon);
        e.set(hit, HitData::fWeaponHealth, 1.0f32.to_bits());
        if e.call(0x009a_cce0, &args![explosion]).u32() != 0 {
            let owner = e.call(0x009a_cce0, &args![explosion]).u32();
            if e.vcall(owner, 0x100, &args![]).bool() {
                let actor = e.call(0x009a_cce0, &args![explosion]).u32();
                if e.call(0x008a_1710, &args![actor]).u32() == e.get(hit, HitData::pWeapon) {
                    let item = if e.call(0x008d_8520, &args![actor]).u32() != 0 {
                        let process = e.call(0x008d_8520, &args![actor]).u32();
                        e.vcall(process, 0x148, &args![]).u32()
                    } else {
                        0
                    };
                    let health = if item != 0 {
                        x87_store32(e.call(0x004b_cdb0, &args![item, 1u32]).f64())
                    } else {
                        1.0f32.to_bits()
                    };
                    e.set(hit, HitData::fWeaponHealth, x87_quiet_bits(health));
                }
            }
        }
    }
    e.set(hit, HitData::pSourceRef, explosion);
    e.set(
        hit,
        HitData::fHealthDamage,
        x87_quiet_bits(damage.to_bits()),
    );
    let health_damage = e.get(hit, HitData::fHealthDamage);
    e.set(hit, HitData::fTotalDamage, x87_quiet_bits(health_damage));
    hit_data_reduce_damage(e, hit, 1);
    hit_data_check_critical_strike(e, hit);
    hit_data_determine_outcome(e, hit);
}

// Translated from 009b5a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::ReduceDamage` (Xbox PDB), `RET 4` (`ignore_block`, its low byte). Only for a
/// character target (vtable slot `0x100`); a target whose `+0x94` object's vtable slot `0x10`
/// is true takes no damage (total, health and fatigue damage cleared). Otherwise the total
/// damage is the health damage, the minimum damage `health damage * setting 011ced4c`, and:
/// - the fatigue damage goes through the ammunition effects of kind 5;
/// - damage resistance: `min(actor value 0x12, 100) / 100`, scaled by the player's `1 + 0.05 *
///   value 8` factor when `00566950(target)`, through the ammunition effects of kind 1 and
///   capped at `setting 011d1020 / 100`; the armor damage is `006462e0(damage, resistance)`
///   unless the fatigue damage is above 0 (then 0), and the health damage is scaled by `1 -
///   resistance`;
/// - damage threshold: `max(actor value 0x4c, 0)`, the same factor, kept unmodified for the
///   test below and through the ammunition effects of kind 2;
/// - a blocked hit (unless `ignore_block`; `Actor::GetBlocked` when vtable slot `0x234` is
///   false) that `009a6ae0(target, source or aggressor, 0)` accepts sets flag 1 (the player's
///   block runs `00944380` with setting `011d0be4`), computes the percentage blocked from the
///   skill (`0x26` for a weapon of type 1 or 2 held, else `0x2d` when unarmed or with type 0;
///   `006463a0`), 0 for an unarmed block of a weapon hit or a hit with a source, adds it to the
///   threshold and sets flag 2 with whether the target's weapon is drawn;
/// - entry points 0x3a (the aggressor's, with the weapon, or `004839c0(500)` stored when
///   there is none) and 0x38 (the target's) lower and raise the threshold, which is then not
///   below 0, the "threshold ate everything" marker being set when the original threshold was
///   above 0 and the result at most 0; a weapon with mod effect 0xc (`ItemChange::
///   HasModEffectActive_ov2`) divides it by `GetNumProjectiles`;
/// - the threshold is subtracted, or for `00524b40`/`006450c0` weapons applied as a fraction of
///   `max(vtable slot 0x10 of the weapon's +0x9c object, 1)`;
/// - the ammunition effects of kind 0 and entry point 0 adjust the health damage; with no
///   fatigue damage the armor damage gains `setting 011cf0fc * min(threshold, damage -
///   minimum)`;
/// - a damage below the minimum is raised to it, with the hit-marker UI (`00709c60` for the
///   player as target, `00709c80` for the player as aggressor unless slot `0x22c`) and a
///   follower bark (`FollowerBarks::TriggerFollowerBark(aggressor, 9)` when `00566950`); the
///   UI also runs for the marker alone;
/// - a weapon enchantment's actor value (`009b6600`, not -1) between 0 and 100% scales the
///   damage again; a positive fatigue damage follows the health damage's ratio to the total;
///   for the player as target with `00525430(011f2250)` true every damage is scaled by setting
///   `011ce9f0`.
pub fn hit_data_reduce_damage(e: &mut Engine, this: Ptr<HitData>, ignore_block: u32) {
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        e.set(this, HitData::fTotalDamage, 0);
        e.set(this, HitData::fHealthDamage, 0);
        e.set(this, HitData::fFatigueDamage, 0);
        return;
    }
    let damage = e.get(this, HitData::fHealthDamage);
    e.set(this, HitData::fTotalDamage, x87_quiet_bits(damage));
    let minimum_scale = setting_bits(e, 0x011c_ed4c);
    let damage = e.get(this, HitData::fHealthDamage);
    let minimum = x87_store32(x87_mul(x87_load32(damage), x87_m32(minimum_scale)));
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        let fatigue = e.get(this, HitData::fFatigueDamage);
        let fatigue = apply_ammo_effects(e, 5, effects, fatigue);
        e.set(this, HitData::fFatigueDamage, fatigue);
    }
    let hundred = x87_quiet_bits(e.mem.u32(HUNDRED_01016410));
    let target = e.get(this, HitData::pTarget);
    let resistance_value = x87_store32(target_actor_value(e, target, 0x12));
    let clamped = e.call(0x0040_ebd0, &args![resistance_value, hundred]).f64();
    let mut resistance = x87_store32(x87_div(clamped, double_at(e, HUNDRED_01017A40)));
    let target = e.get(this, HitData::pTarget);
    if e.call(0x0056_6950, &args![target]).bool() {
        let factor = player_factor(e);
        resistance = x87_store32(x87_mul(x87_load32(resistance), x87_m32(factor)));
    }
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        resistance = apply_ammo_effects(e, 1, effects, resistance);
    }
    let cap = setting_bits(e, 0x011d_1020);
    let cap = x87_store32(x87_div(x87_load32(cap), double_at(e, HUNDRED_01017A40)));
    if !parity_even(fpu_compare(x87_load32(cap), x87_load32(resistance)) & 0x05) {
        resistance = x87_quiet_bits(cap);
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    let armor_damage = if above_zero(e, x87_load32(fatigue)) {
        0
    } else {
        let damage = x87_quiet_bits(e.get(this, HitData::fHealthDamage));
        x87_store32(
            e.call(0x0064_62e0, &args![damage, x87_quiet_bits(resistance)])
                .f64(),
        )
    };
    e.set(this, HitData::fArmorDamage, x87_quiet_bits(armor_damage));
    let damage = e.get(this, HitData::fHealthDamage);
    let kept = x87_sub(1.0, x87_load32(resistance));
    e.set(
        this,
        HitData::fHealthDamage,
        x87_store32(x87_mul(kept, x87_m32(damage))),
    );
    let target = e.get(this, HitData::pTarget);
    let threshold_value = x87_store32(target_actor_value(e, target, 0x4c));
    let mut threshold = x87_store32(e.call(0x0040_4010, &args![threshold_value, 0.0f32]).f64());
    let target = e.get(this, HitData::pTarget);
    if e.call(0x0056_6950, &args![target]).bool() {
        let factor = player_factor(e);
        threshold = x87_store32(x87_mul(x87_load32(threshold), x87_m32(factor)));
    }
    let original_threshold = x87_quiet_bits(threshold);
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        threshold = apply_ammo_effects(e, 2, effects, threshold);
    }
    if ignore_block as u8 == 0 {
        threshold = reduce_damage_block(e, this, threshold);
    }
    threshold = reduce_damage_entry_points(e, this, threshold);
    let marker =
        above_zero(e, x87_load32(original_threshold)) && at_most_zero(e, x87_load32(threshold));
    threshold = x87_store32(
        e.call(0x0040_4010, &args![0.0f32, x87_quiet_bits(threshold)])
            .f64(),
    );
    threshold = reduce_damage_projectiles(e, this, threshold);
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0
        && e.call(0x0052_4b40, &args![weapon]).bool()
        && e.call(0x0064_50c0, &args![weapon]).bool()
    {
        let weapon = e.get(this, HitData::pWeapon);
        let value = e.vcall(weapon.wrapping_add(0x9c), 0x10, &args![]).u16();
        let divisor = e.call(0x009b_65d0, &args![u32::from(value), 1u32]).i32();
        let divisor = x87_store32(f64::from(divisor));
        let fraction = x87_div(x87_load32(threshold), x87_m32(divisor));
        let kept = x87_store32(x87_sub(1.0, fraction));
        let kept = x87_store32(
            e.call(0x0040_4010, &args![x87_quiet_bits(kept), 0.0f32])
                .f64(),
        );
        let damage = e.get(this, HitData::fHealthDamage);
        e.set(
            this,
            HitData::fHealthDamage,
            x87_store32(x87_mul(x87_load32(damage), x87_m32(kept))),
        );
    } else {
        let damage = e.get(this, HitData::fHealthDamage);
        e.set(
            this,
            HitData::fHealthDamage,
            x87_store32(x87_sub(x87_load32(damage), x87_m32(threshold))),
        );
    }
    reduce_damage_finish(e, this, minimum, threshold, marker);
}

/// The block part of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_block(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    let mut threshold = threshold;
    let target = e.get(this, HitData::pTarget);
    let blocked = if e.vcall(target, 0x234, &args![]).bool() {
        false
    } else {
        let target = e.get(this, HitData::pTarget);
        e.call(0x0089_4d60, &args![target]).bool()
    };
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let unarmed_block = e.vcall(process, 0x148, &args![]).u32() == 0 && blocked;
    if !blocked {
        return threshold;
    }
    let source = match e.get(this, HitData::pSourceRef) {
        0 => e.get(this, HitData::pAggressor),
        source => source,
    };
    let target = e.get(this, HitData::pTarget);
    if !e.call(0x009a_6ae0, &args![target, source, 0u32]).bool() {
        return threshold;
    }
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let held = if e.vcall(process, 0x148, &args![]).u32() != 0 {
        let target = e.get(this, HitData::pTarget);
        let process = e.call(0x008d_8520, &args![target]).u32();
        let item = e.vcall(process, 0x148, &args![]).u32();
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    set_flag(e, this, 1, 1);
    let player = e.mem.u32(PLAYER_011DEA3C);
    if player == e.get(this, HitData::pTarget) {
        let value = x87_quiet_bits(setting_bits(e, 0x011d_0be4));
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(0x0094_4380, &args![player, value]);
    }
    let mut drawn = 0u32;
    if held != 0 {
        let target = e.get(this, HitData::pTarget);
        if e.call(0x008a_16d0, &args![target]).bool() {
            drawn = 1;
        }
    }
    let skill_index = if e.get(this, HitData::pWeapon) != 0
        && held != 0
        && (e.call(0x0044_6390, &args![held]).u32() == 2
            || e.call(0x0044_6390, &args![held]).u32() == 1)
    {
        Some(0x26u32)
    } else if e.get(this, HitData::pWeapon) == 0
        || (held != 0 && e.call(0x0044_6390, &args![held]).u32() == 0)
    {
        Some(0x2d)
    } else {
        None
    };
    let skill = match skill_index {
        Some(index) => {
            let target = e.get(this, HitData::pTarget);
            e.vcall(target.wrapping_add(0xa4), 8, &args![index]).u32()
        }
        None => 0,
    };
    let weapon = e.get(this, HitData::pWeapon);
    let no_block = unarmed_block
        && ((weapon != 0 && !e.call(0x0064_50c0, &args![weapon]).bool())
            || e.get(this, HitData::pSourceRef) != 0);
    let percent = if no_block {
        0
    } else {
        x87_store32(e.call(0x0064_63a0, &args![skill]).f64())
    };
    e.set(this, HitData::fPercentBlocked, percent);
    let percent = e.get(this, HitData::fPercentBlocked);
    if above_zero(e, x87_load32(percent)) {
        threshold = x87_store32(x87_add(x87_load32(threshold), x87_m32(percent)));
    }
    set_flag(e, this, 2, drawn);
    threshold
}

/// The entry-point part of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_entry_points(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let lowered = locals.addr();
        let raised = lowered + 4;
        e.mem.set_u32(lowered, 0);
        if e.get(this, HitData::pAggressor) != 0 {
            if e.get(this, HitData::pWeapon) == 0 {
                let weapon = e.call(0x0048_39c0, &args![0x1f4u32]).u32();
                e.set(this, HitData::pWeapon, weapon);
            }
            let aggressor = e.get(this, HitData::pAggressor);
            let weapon = e.get(this, HitData::pWeapon);
            let target = e.get(this, HitData::pTarget);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x3au32, aggressor, weapon, target, lowered],
            );
        }
        e.mem.set_u32(raised, 0);
        if e.get(this, HitData::pTarget) != 0 {
            let weapon = weapon_or_default(e, this);
            let target = e.get(this, HitData::pTarget);
            let aggressor = e.get(this, HitData::pAggressor);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x38u32, target, aggressor, weapon, raised],
            );
        }
        let lowered = e.mem.u32(lowered);
        let raised = e.mem.u32(raised);
        let value = x87_sub(x87_load32(threshold), x87_m32(lowered));
        x87_store32(x87_add(value, x87_m32(raised)))
    })
}

/// The projectile split of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_projectiles(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor == 0 || e.call(0x008d_8520, &args![aggressor]).u32() == 0 {
        return threshold;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    let process = e.call(0x008d_8520, &args![aggressor]).u32();
    if e.vcall(process, 0x148, &args![]).u32() == 0 {
        return threshold;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    let process = e.call(0x008d_8520, &args![aggressor]).u32();
    let item = e.vcall(process, 0x148, &args![]).u32();
    if !e.call(0x004b_da70, &args![item, 0xcu32]).bool() {
        return threshold;
    }
    if !above_zero(e, x87_load32(threshold)) {
        return threshold;
    }
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    let projectiles = e
        .call(0x0052_5b20, &args![weapon, 1u32, 0u32, aggressor])
        .u8();
    x87_store32(x87_div(x87_m32(threshold), f64::from(projectiles)))
}

/// Shows the hit marker of `ReduceDamage` (`009b5a30`): `00709c60(marker)` for the player as
/// target, else `00709c80(marker)` for the player as aggressor unless the target's vtable slot
/// `0x22c` (with 0) is true.
fn reduce_damage_marker(e: &mut Engine, this: Ptr<HitData>, marker: bool) {
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pTarget) == player {
        e.call(0x0070_9c60, &args![u32::from(marker)]);
        return;
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pAggressor) != player {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if !e.vcall(target, 0x22c, &args![0u32]).bool() {
        e.call(0x0070_9c80, &args![u32::from(marker)]);
    }
}

/// The end of `ReduceDamage` (`009b5a30`), from the ammunition effects of kind 0 on.
fn reduce_damage_finish(
    e: &mut Engine,
    this: Ptr<HitData>,
    minimum: u32,
    threshold: u32,
    marker: bool,
) {
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        let damage = e.get(this, HitData::fHealthDamage);
        let damage = apply_ammo_effects(e, 0, effects, damage);
        e.set(this, HitData::fHealthDamage, damage);
    }
    if e.get(this, HitData::pWeapon) != 0 {
        let aggressor = e.get(this, HitData::pAggressor);
        let weapon = e.get(this, HitData::pWeapon);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0u32, aggressor, weapon, target, this.addr() + 0x14],
        );
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    if at_most_zero(e, x87_load32(fatigue)) {
        let damage = e.get(this, HitData::fHealthDamage);
        let over = x87_store32(x87_sub(x87_load32(damage), x87_m32(minimum)));
        let least = e
            .call(
                0x0040_ebd0,
                &args![x87_quiet_bits(threshold), x87_quiet_bits(over)],
            )
            .f64();
        let scale = setting_bits(e, 0x011c_f0fc);
        let extra = x87_store32(x87_mul(x87_load32(scale), least));
        let armor = e.get(this, HitData::fArmorDamage);
        e.set(
            this,
            HitData::fArmorDamage,
            x87_store32(x87_add(x87_load32(armor), x87_m32(extra))),
        );
    }
    let damage = e.get(this, HitData::fHealthDamage);
    if fpu_compare(x87_load32(minimum), x87_load32(damage)) & 0x41 == 0 {
        e.set(this, HitData::fHealthDamage, x87_quiet_bits(minimum));
        reduce_damage_marker(e, this, marker);
        let aggressor = e.get(this, HitData::pAggressor);
        if aggressor != 0 && e.call(0x0056_6950, &args![aggressor]).bool() {
            let aggressor = e.get(this, HitData::pAggressor);
            e.call(0x008d_5cb0, &args![aggressor, 9u32]);
        }
    } else if marker {
        reduce_damage_marker(e, this, marker);
    }
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0 && e.call(0x009b_6600, &args![weapon]).i32() != -1 {
        let hundred = x87_quiet_bits(e.mem.u32(HUNDRED_01016410));
        let weapon = e.get(this, HitData::pWeapon);
        let index = e.call(0x009b_6600, &args![weapon]).u32();
        let target = e.get(this, HitData::pTarget);
        let value = x87_store32(target_actor_value(e, target, index));
        let clamped = e.call(0x0040_ebd0, &args![value, hundred]).f64();
        let fraction = x87_store32(x87_div(clamped, double_at(e, HUNDRED_01017A40)));
        let fraction_value = x87_load32(fraction);
        if above_zero(e, fraction_value)
            && !parity_even(fpu_compare(fraction_value, double_at(e, ONE_01012070)) & 0x41)
        {
            let damage = e.get(this, HitData::fHealthDamage);
            let kept = x87_sub(1.0, fraction_value);
            e.set(
                this,
                HitData::fHealthDamage,
                x87_store32(x87_mul(kept, x87_m32(damage))),
            );
        }
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    let total = e.get(this, HitData::fTotalDamage);
    if above_zero(e, x87_load32(fatigue)) && above_zero(e, x87_load32(total)) {
        let damage = e.get(this, HitData::fHealthDamage);
        let total = e.get(this, HitData::fTotalDamage);
        let ratio = x87_store32(x87_div(x87_load32(damage), x87_m32(total)));
        let fatigue = e.get(this, HitData::fFatigueDamage);
        e.set(
            this,
            HitData::fFatigueDamage,
            x87_store32(x87_mul(x87_load32(fatigue), x87_m32(ratio))),
        );
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pTarget) == player && e.call(0x0052_5430, &args![0x011f_2250u32]).bool()
    {
        for offset in [0x14u32, 0x18, 0x1c, 0x20, 0x28, 0x2c] {
            let scale = setting_bits(e, 0x011c_e9f0);
            let value = field(e, this, offset);
            set_field(
                e,
                this,
                offset,
                x87_store32(x87_mul(x87_load32(value), x87_m32(scale))),
            );
        }
    }
}

// Translated from 009b7010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies the three words at `this + 0x104` (a point) to
/// `result` and returns `result`.
pub fn fn_009b7010(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(this.addr().wrapping_add(0x104 + 4 * word));
        e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
    }
    result
}

// Translated from 009b7060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::CheckCriticalStrike` (Xbox PDB). Nothing when the target's `+0x94` object's
/// vtable slot `0x10` is true or flag 0x800 is set. The chance is `00646d80(aggressor's
/// ActorValueOwner or 0, weapon or default)`, adjusted by entry points 1 (the aggressor's) and
/// 0x24 (the target's), plus setting `011ce46c` for a VATS hit, times 10; a sneak attack (the
/// aggressor's vtable slot `0x360`, a character target that is not slot `0x22c`, `004997b0` on
/// the aggressor, not already in combat with it, with no detection
/// (`Actor::GetDetectionLevelAgainstActor` after `Actor::LineOfSight`) multiplies it by
/// setting `011ce858` and sets flag 0x400. A roll `00487f50() % 1000` below the chance
/// (`_ftol2`) is a critical: flag 4, and with a weapon or the default weapon the critical
/// damage (the weapon's `00645c20`, or the health damage times setting `011ce57c` without
/// one) goes through entry point 2 and, when positive, is added to the health damage; flag 8
/// becomes `009b73b0(weapon)` and the critical effect `0051f4d0(weapon)`.
pub fn hit_data_check_critical_strike(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    if target != 0 && e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        return;
    }
    if e.call(IS_FLAG_SET, &args![this, 0x800u32]).bool() {
        return;
    }
    e.with_stack(8, |e, locals| {
        let chance_slot = locals.addr();
        let bonus_slot = chance_slot + 4;
        e.mem.set_u32(bonus_slot, 0);
        let weapon = weapon_or_default(e, this);
        let aggressor = e.get(this, HitData::pAggressor);
        let owner = if aggressor != 0 {
            aggressor.wrapping_add(0xa4)
        } else {
            0
        };
        let chance = x87_store32(e.call(0x0064_6d80, &args![owner, weapon]).f64());
        e.mem.set_u32(chance_slot, chance);
        let weapon = weapon_or_default(e, this);
        let aggressor = e.get(this, HitData::pAggressor);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![1u32, aggressor, weapon, target, chance_slot],
        );
        let weapon = weapon_or_default(e, this);
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0x24u32, target, weapon, aggressor, chance_slot],
        );
        let mut chance = e.mem.u32(chance_slot);
        if e.get(this, HitData::pVATSCommand) != 0 {
            let bonus = setting_bits(e, 0x011c_e46c);
            chance = x87_store32(x87_add(x87_load32(chance), x87_m32(bonus)));
        }
        chance = x87_store32(x87_mul(x87_load32(chance), double_at(e, TEN_01020758)));
        if critical_sneak_attack(e, this) {
            let scale = setting_bits(e, 0x011c_e858);
            chance = x87_store32(x87_mul(x87_load32(chance), x87_m32(scale)));
            set_flag(e, this, 0x400, 1);
        }
        let roll = e.call(0x0048_7f50, &args![]).u32() % 1000;
        let limit = e.call(FTOL, &args![x87_load32(chance)]).u32() as i32;
        if roll as i32 >= limit {
            return;
        }
        set_flag(e, this, 4, 1);
        let weapon = weapon_or_default(e, this);
        if weapon == 0 {
            return;
        }
        let damage = if e.get(this, HitData::pWeapon) != 0 {
            let own = e.get(this, HitData::pWeapon);
            f64::from(e.call(0x0064_5c20, &args![own]).u16())
        } else {
            let scale = setting_bits(e, 0x011c_e57c);
            let health = e.get(this, HitData::fHealthDamage);
            x87_mul(x87_load32(health), x87_m32(scale))
        };
        e.mem.set_u32(bonus_slot, x87_store32(damage));
        let aggressor = e.get(this, HitData::pAggressor);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![2u32, aggressor, weapon, target, bonus_slot],
        );
        let bonus = e.mem.u32(bonus_slot);
        if above_zero(e, x87_load32(bonus)) {
            let health = e.get(this, HitData::fHealthDamage);
            e.set(
                this,
                HitData::fHealthDamage,
                x87_store32(x87_add(x87_load32(health), x87_m32(bonus))),
            );
        }
        let critical = e.call(0x009b_73b0, &args![weapon]).u8();
        set_flag(e, this, 8, u32::from(critical));
        let effect = e.call(0x0051_f4d0, &args![weapon]).u32();
        e.set(this, HitData::pCriticalEffect, effect);
    });
}

/// The sneak-attack test of `CheckCriticalStrike` (`009b7060`).
fn critical_sneak_attack(e: &mut Engine, this: Ptr<HitData>) -> bool {
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor == 0 || !e.vcall(aggressor, 0x360, &args![]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target, 0x22c, &args![0u32]).bool() {
        return false;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    if !e.call(0x0049_97b0, &args![aggressor]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    let aggressor = e.get(this, HitData::pAggressor);
    if e.call(0x008b_c700, &args![target, aggressor]).bool() {
        return false;
    }
    e.with_stack(4, |e, sight| {
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        let seen = e
            .call(
                0x0088_b880,
                &args![target, 0u32, aggressor, 1u32, 0u32, 0u32],
            )
            .u8();
        e.mem.set_u8(sight.addr(), seen);
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        let level = e
            .call(
                0x008a_0d10,
                &args![target, 0u32, aggressor, sight, 0u32, 0u32, 0u32, 0u32],
            )
            .i32();
        level <= 0
    })
}

// Translated from 009b73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::DetermineOutcome` (Xbox PDB). Only for a character target (vtable slot `0x100`)
/// whose `+0x94` object's slot `0x10` is false. A positive bonus multiplier applies to the
/// health, total and (for a non-`0x21c` aggressor with skill 0x2d) fatigue damage, first scaled
/// by setting `011ceae0` (no weapon or a `006450c0` weapon) or `011cf794` when it is at least 1
/// and flag 0x400 is set. A VATS command with more than one shot multiplies the damage for the
/// kill test. A hit on limb 0xe (the weapon): when the target's item (`0044ddc0` of its process's
/// slot `0x148` item) would break (`ItemChange::GetItemHealth(item, 0)` minus the limb damage at
/// most 0), flag 0x200 (slot `0x21c` true or `0046e8c0`) or 0x100. Another limb (not -1) of the
/// target's body part data (`004181e0(target)` slot `0x180`, `BGSBodyPartData::GetBodyPart`) that
/// is `008b4360`: the dismember chance (`008b4cf0`), explode flag (`008b4cd0`), and a crippled
/// limb (the limb's actor value, `005e5190`, above 0 and the limb damage taking it to at most 0)
/// sets flag 0x80. When the health (actor value 0x10) minus the scaled damage is at most 0: flag
/// 0x10, and for a limb the dismember / explode outcome (`00646e50`, unless the chance is 100 and
/// the limb explodes) sets flags 0x20 and 0x40.
pub fn hit_data_determine_outcome(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        return;
    }
    let bonus = e.get(this, HitData::fBonusMult);
    if above_zero(e, x87_load32(bonus)) {
        apply_bonus(e, this);
    }
    let mut shots = 1.0f32.to_bits();
    let command = e.get(this, HitData::pVATSCommand);
    if command != 0 && i32::from(e.mem.u8(command.wrapping_add(8))) > 1 {
        let count = e.mem.u8(command.wrapping_add(8));
        shots = x87_store32(f64::from(count));
    }
    let mut chance = 0u32;
    let mut sure = false;
    let mut explodes = 0u8;
    let limb = e.get(this, HitData::eDamageLimb);
    if limb == 0xe {
        outcome_weapon_limb(e, this);
    } else if limb != u32::MAX {
        if let Some((part_chance, part_explodes)) = outcome_body_part(e, this) {
            chance = part_chance;
            explodes = part_explodes;
            sure = explodes != 0 && chance as i32 >= 100;
        }
    }
    let target = e.get(this, HitData::pTarget);
    let health = x87_store32(target_actor_value(e, target, 0x10));
    let damage = e.get(this, HitData::fHealthDamage);
    let scaled = x87_mul(x87_load32(damage), x87_m32(shots));
    let left = x87_sub(x87_m32(health), scaled);
    if !at_most_zero(e, left) {
        return;
    }
    set_flag(e, this, 0x10, 1);
    if e.get(this, HitData::eDamageLimb) == u32::MAX {
        return;
    }
    e.with_stack(4, |e, flags| {
        let dismember = flags.addr();
        let explode = dismember + 1;
        e.mem.set_u8(dismember, 0);
        e.mem.set_u8(explode, u8::from(sure));
        if !sure {
            let weapon = match e.get(this, HitData::pWeapon) {
                0 => {
                    if e.get(this, HitData::eAttackSkill) == 0x2d {
                        e.mem.u32(DEFAULT_WEAPON_011CA278)
                    } else {
                        0
                    }
                }
                weapon => weapon,
            };
            let target = e.get(this, HitData::pTarget);
            let owner = if target != 0 {
                target.wrapping_add(0xa4)
            } else {
                0
            };
            e.call(
                0x0064_6e50,
                &args![
                    owner,
                    weapon,
                    chance,
                    0u32,
                    0u32,
                    u32::from(explodes),
                    dismember,
                    explode
                ],
            );
        }
        if e.mem.u8(dismember) != 0 {
            set_flag(e, this, 0x20, 1);
        }
        if e.mem.u8(explode) != 0 {
            set_flag(e, this, 0x40, 1);
        }
    });
}

/// The bonus-multiplier part of `DetermineOutcome` (`009b73d0`).
fn apply_bonus(e: &mut Engine, this: Ptr<HitData>) {
    let bonus = e.get(this, HitData::fBonusMult);
    let at_least_one = fpu_compare(x87_load32(bonus), double_at(e, ONE_01012070)) & 0x01 == 0;
    if at_least_one && e.call(IS_FLAG_SET, &args![this, 0x400u32]).bool() {
        let weapon = e.get(this, HitData::pWeapon);
        let setting = if weapon == 0 || e.call(0x0064_50c0, &args![weapon]).bool() {
            0x011c_eae0
        } else {
            0x011c_f794
        };
        let scale = setting_bits(e, setting);
        let bonus = e.get(this, HitData::fBonusMult);
        e.set(
            this,
            HitData::fBonusMult,
            x87_store32(x87_mul(x87_load32(bonus), x87_m32(scale))),
        );
    }
    let scale_field = |e: &mut Engine, offset: u32| {
        let value = field(e, this, offset);
        let bonus = e.get(this, HitData::fBonusMult);
        set_field(
            e,
            this,
            offset,
            x87_store32(x87_mul(x87_load32(value), x87_m32(bonus))),
        );
    };
    scale_field(e, 0x14);
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor != 0
        && !e.vcall(aggressor, 0x21c, &args![]).bool()
        && e.get(this, HitData::eAttackSkill) == 0x2d
    {
        scale_field(e, 0x1c);
    }
    scale_field(e, 0x18);
}

/// The weapon-limb (0xe) part of `DetermineOutcome` (`009b73d0`).
fn outcome_weapon_limb(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let item = e.vcall(process, 0x148, &args![]).u32();
    let form = if item != 0 {
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    if form == 0 {
        return;
    }
    let health = x87_store32(e.call(0x004b_cdb0, &args![item, 0u32]).f64());
    let limb_damage = e.get(this, HitData::fTargetedLimbDamage);
    if !at_most_zero(e, x87_sub(x87_load32(health), x87_m32(limb_damage))) {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target, 0x21c, &args![]).bool() || e.call(0x0046_e8c0, &args![form]).bool() {
        set_flag(e, this, 0x200, 1);
    } else {
        set_flag(e, this, 0x100, 1);
    }
}

/// The body-part part of `DetermineOutcome` (`009b73d0`): the dismember chance and explode flag
/// when the target's body part for the limb is `008b4360`.
fn outcome_body_part(e: &mut Engine, this: Ptr<HitData>) -> Option<(u32, u8)> {
    let target = e.get(this, HitData::pTarget);
    let data = e.call(0x0041_81e0, &args![target]).u32();
    let parts = e.vcall(data, 0x180, &args![]).u32();
    if parts == 0 {
        return None;
    }
    let limb = e.get(this, HitData::eDamageLimb);
    let part = e.call(0x005e_50f0, &args![parts, limb]).u32();
    if part == 0 || !e.call(0x008b_4360, &args![part]).bool() {
        return None;
    }
    let chance = u32::from(e.call(0x008b_4cf0, &args![part]).u8());
    let explodes = e.call(0x008b_4cd0, &args![part]).u8();
    let index = e.call(0x005e_5190, &args![part]).u8() as i8 as i32 as u32;
    let target = e.get(this, HitData::pTarget);
    let value = x87_store32(target_actor_value(e, target, index));
    if above_zero(e, x87_load32(value)) {
        let limb_damage = e.get(this, HitData::fTargetedLimbDamage);
        if at_most_zero(e, x87_sub(x87_load32(value), x87_m32(limb_damage))) {
            set_flag(e, this, 0x80, 1);
        }
    }
    Some((chance, explodes))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007c_b2e0, fn_007cb2e0(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x009b_7970, fn_009b7970(Ptr)),
        entry!(
            0x009a_4320,
            bs_simple_array_body_part_move_items(Ptr, u32, u32)
        ),
        entry!(
            0x009b_4d90,
            hit_data_constructor(Ptr<HitData>) -> Ptr<HitData>
        ),
        entry!(0x009b_4dc0, hit_data_clear_data(Ptr<HitData>)),
        entry!(0x009b_4ec0, hit_data_copy(Ptr<HitData>, Ptr<HitData>)),
        entry!(
            0x009b_5770,
            hit_data_initialize_explosion_data(Ptr<HitData>, u32, u32, u32, f32)
        ),
        entry!(0x009b_5a30, hit_data_reduce_damage(Ptr<HitData>, u32)),
        entry!(0x009b_7010, fn_009b7010(Ptr, Ptr) -> Ptr),
        entry!(0x009b_7060, hit_data_check_critical_strike(Ptr<HitData>)),
        entry!(0x009b_73d0, hit_data_determine_outcome(Ptr<HitData>)),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_009b7970_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009b_7970, ARRAY_VTABLE_0108F9E8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007cb2e0_reserves_a_slot_constructs_it_and_stores_the_pointer() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0xabcd_0001);
        e.register(0x0076_1540, |_, _| 3u32.into_ret());
        e.register(0x0072_6bf0, |_, _| Ret::default());
        let index = e.call(0x007c_b2e0, &args![array, item]).u32();
        assert_eq!(index, 3);
        assert_eq!(e.mem.u32(buffer + 12), 0xabcd_0001);
        let log = e.call_log.take().unwrap();
        let reserve: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0076_1540)
            .map(|c| &c.1)
            .collect();
        assert_eq!(reserve, vec![&vec![array]]);
        let construct: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0072_6bf0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(construct, vec![&vec![array, buffer + 12, 1]]);
    }

    #[test]
    fn at_007cb2e0_stores_into_the_buffer_the_reservation_moved_to() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        let old_buffer = e.mem.alloc(0x20);
        let new_buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, old_buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x5555_aaaa);
        e.register_double(0x0076_1540, move |e, a| {
            e.mem.set_u32(a[0] + 4, new_buffer);
            0u32.into_ret()
        });
        e.register(0x0072_6bf0, |_, _| Ret::default());
        assert_eq!(e.call(0x007c_b2e0, &args![array, item]).u32(), 0);
        assert_eq!(e.mem.u32(new_buffer), 0x5555_aaaa);
        assert_eq!(e.mem.u32(old_buffer), 0);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    const SLOT_BASE: u32 = 0x7100_0000;

    type Answers = Rc<RefCell<HashMap<(u32, u32), Ret>>>;

    /// Test objects sharing one vtable whose slots answer from a table by (object, slot).
    struct World {
        vtable: u32,
        answers: Answers,
    }

    impl World {
        fn new(e: &mut Engine) -> World {
            let vtable = e.mem.alloc(0x400);
            let answers: Answers = Default::default();
            for slot in (0..0x400).step_by(4) {
                e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
                let table = answers.clone();
                e.register_double(SLOT_BASE + slot, move |_, a| {
                    table
                        .borrow()
                        .get(&(a[0], slot))
                        .copied()
                        .unwrap_or_default()
                });
            }
            World { vtable, answers }
        }

        /// An object with a vtable at `+0`, `+0x94` and `+0xa4`.
        fn object(&self, e: &mut Engine) -> u32 {
            let object = e.mem.alloc(0x100);
            for offset in [0, 0x94, 0xa4] {
                e.mem.set_u32(object + offset, self.vtable);
            }
            object
        }

        fn answer(&self, object: u32, slot: u32, value: u32) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    eax: value,
                    ..Ret::default()
                },
            );
        }

        fn answer_float(&self, object: u32, slot: u32, value: f64) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    st0: value,
                    ..Ret::default()
                },
            );
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn constants(e: &mut Engine) {
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_7000,
            0x0102_0000,
            0x0103_c000,
            0x011c_a000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(ZERO_01012060, 0f64.to_bits());
        e.mem.set_u64(ONE_01012070, 1f64.to_bits());
        e.mem.set_u64(HUNDRED_01017A40, 100f64.to_bits());
        e.mem.set_u32(HUNDRED_01016410, 100f32.to_bits());
        e.mem.set_u64(TEN_01020758, 10f64.to_bits());
        e.mem
            .set_u64(FIVE_HUNDREDTHS_0103C7C8, f64::from(0.05f32).to_bits());
    }

    /// Settings: `00403e20` answers the address of a float holding the value set here.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let table: HashMap<u32, u32> = values
            .iter()
            .map(|(setting, value)| {
                let slot = e.mem.alloc(4);
                e.mem.set_u32(slot, value.to_bits());
                (*setting, slot)
            })
            .collect();
        e.register_double(SETTING_VALUE, move |_, a| table[&a[0]].into_ret());
    }

    #[test]
    fn move_items_destroys_and_moves_the_tail() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4000);
        e.mem.set_u32(this + 8, 10);
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        quiet(&mut e, &[0x0072_ba80, 0x0042_fb60]);
        e.call_log = Some(vec![]);
        e.call(0x009a_4320, &args![this, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0042_f5a0), vec![vec![5, 3]]);
        assert_eq!(calls(&log, 0x0072_ba80), vec![vec![this, 0x4008, 3]]);
        assert_eq!(
            calls(&log, 0x0042_fb60),
            vec![vec![this, 0x4008, 0x401c, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 7);
    }

    #[test]
    fn fn_009b7010_copies_the_point() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        for word in 0..3 {
            e.mem.set_u32(this + 0x104 + 4 * word, 0x10 + word);
        }
        let result = e.mem.alloc(0xc);
        assert_eq!(e.call(0x009b_7010, &args![this, result]).u32(), result);
        assert_eq!(
            (
                e.mem.u32(result),
                e.mem.u32(result + 4),
                e.mem.u32(result + 8)
            ),
            (0x10, 0x11, 0x12)
        );
    }

    #[test]
    fn constructor_clears_and_copy_copies() {
        let mut e = Engine::new();
        constants(&mut e);
        for word in 0..3 {
            e.mem.set_u32(0x011f_426c + 4 * word, 0x100 + word);
        }
        quiet(&mut e, &[0x0068_15c0]);
        let hit = e.mem.alloc(0x64);
        for offset in (0..0x64).step_by(4) {
            e.mem.set_u32(hit + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_4d90, &args![hit]).u32(), hit);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0068_15c0),
            vec![vec![hit + 0x38], vec![hit + 0x44]]
        );
        let words: Vec<u32> = (0..0x64)
            .step_by(4)
            .map(|offset| e.mem.u32(hit + offset))
            .collect();
        let mut expected = vec![0u32; 25];
        expected[3] = 0x4d;
        expected[4] = u32::MAX;
        for word in 0..3 {
            expected[14 + word] = 0x100 + word as u32;
            expected[17 + word] = 0x100 + word as u32;
        }
        assert_eq!(words, expected);
        let other = e.mem.alloc(0x64);
        for offset in (0..0x64).step_by(4) {
            e.mem.set_u32(other + offset, 0x1000 + offset);
        }
        e.mem.set_u32(other + 0x14, 0x7f80_0001);
        e.call(0x009b_4ec0, &args![hit, other]);
        for offset in (0..0x60).step_by(4) {
            let expected = if offset == 0x14 {
                0x7fc0_0001
            } else {
                0x1000 + offset
            };
            assert_eq!(e.mem.u32(hit + offset), expected, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(hit + 0x60), 0);
    }

    #[test]
    fn initialize_explosion_data_fills_the_hit_and_runs_the_steps() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let explosion = world.object(&mut e);
        world.answer(explosion, 0x220, 1);
        e.register(0x009b_0900, |_, _| 0x600u32.into_ret());
        let owner = world.object(&mut e);
        world.answer(owner, 0x100, 1);
        e.register_double(0x009a_cce0, move |_, _| owner.into_ret());
        e.register(0x008a_1710, |_, _| 0x600u32.into_ret());
        let process = world.object(&mut e);
        e.register_double(0x008d_8520, move |_, _| process.into_ret());
        world.answer(process, 0x148, 0x700);
        e.register(0x004b_cdb0, |_, _| float_ret(0.75));
        // The critical check stops at flag 0x800; with no target the other steps return.
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        quiet(&mut e, &[SET_FLAG]);
        let hit = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        e.call(0x009b_5770, &args![hit, 0x10u32, 0u32, explosion, 12.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_FLAG), vec![vec![hit, 0x2000, 1]]);
        assert_eq!(calls(&log, 0x004b_cdb0), vec![vec![0x700, 1]]);
        let hit = Ptr::<HitData>::new(hit);
        assert_eq!(e.get(hit, HitData::pAggressor), 0x10);
        assert_eq!(e.get(hit, HitData::pWeapon), 0x600);
        assert_eq!(e.get(hit, HitData::fWeaponHealth), 0.75f32.to_bits());
        assert_eq!(e.get(hit, HitData::pSourceRef), explosion);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 12.5f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 12.5f32.to_bits());
        // A null hit does nothing.
        e.call_log = Some(vec![]);
        e.call(0x009b_5770, &args![0u32, 0x10u32, 0u32, explosion, 12.5f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// A hit of 100 damage by `aggressor` on `target` (both world objects).
    fn reduce_world(e: &mut Engine, world: &World) -> (Ptr<HitData>, u32, u32) {
        constants(e);
        let aggressor = world.object(e);
        let target = world.object(e);
        world.answer(target, 0x100, 1);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pAggressor, aggressor);
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::fHealthDamage, 100.0f32.to_bits());
        settings(
            e,
            &[
                (0x011c_ed4c, 0.2),
                (0x011d_1020, 85.0),
                (0x011c_f0fc, 0.5),
                (0x011c_e9f0, 1.0),
            ],
        );
        e.register(0x0040_ebd0, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(0x0040_4010, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(0x0064_62e0, |_, _| float_ret(7.0));
        let process = world.object(e);
        e.register_double(0x008d_8520, move |_, _| process.into_ret());
        quiet(
            e,
            &[
                0x0056_6950,
                0x0089_4d60,
                HANDLE_ENTRY_POINT,
                0x0070_9c60,
                0x0070_9c80,
                0x0048_39c0,
                0x0052_5430,
            ],
        );
        e.register(0x009b_6600, |_, _| u32::MAX.into_ret());
        (hit, aggressor, target)
    }

    #[test]
    fn reduce_damage_applies_resistance_and_threshold() {
        let mut e = Engine::new();
        let world = World::new(&mut e);
        let (hit, aggressor, target) = reduce_world(&mut e, &world);
        // Resistance 30 (0.3), threshold 10: 100 * 0.7 - 10 = 60.
        let values = Rc::new(RefCell::new(HashMap::from([
            (0x12u32, 30.0f64),
            (0x4c, 10.0),
        ])));
        let table = values.clone();
        e.register_double(SLOT_BASE + 0xc, move |_, a| {
            float_ret(table.borrow().get(&a[1]).copied().unwrap_or(0.0))
        });
        e.call_log = Some(vec![]);
        e.call(0x009b_5a30, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::fHealthDamage), 60.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 100.0f32.to_bits());
        // Armor damage: 7 from 006462e0, plus 0.5 * min(10, 60 - 20).
        assert_eq!(e.get(hit, HitData::fArmorDamage), 12.0f32.to_bits());
        assert_eq!(
            calls(&log, 0x0064_62e0),
            vec![vec![100.0f32.to_bits(), 0.3f32.to_bits()]]
        );
        // Entry points 0x3a (with the default 004839c0(500) weapon) and 0x38.
        let entries = calls(&log, HANDLE_ENTRY_POINT);
        assert_eq!(entries[0][..4], [0x3a, aggressor, 0, target]);
        assert_eq!(entries[1][..4], [0x38, target, aggressor, 0]);
        assert!(calls(&log, 0x0070_9c60).is_empty());
        // A huge threshold leaves the minimum (0.2 * 100); the player as target sees the hit
        // marker (0: the threshold was not used up by the entry points).
        values.borrow_mut().insert(0x4c, 500.0);
        let player = target;
        e.mem.set_u32(PLAYER_011DEA3C, player);
        e.set(hit, HitData::fHealthDamage, 100.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x009b_5a30, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::fHealthDamage), 20.0f32.to_bits());
        assert_eq!(calls(&log, 0x0070_9c60), vec![vec![0]]);
    }

    #[test]
    fn reduce_damage_spares_an_immune_target() {
        let mut e = Engine::new();
        let world = World::new(&mut e);
        let (hit, _, target) = reduce_world(&mut e, &world);
        e.mem.set_u32(target + 0x94, world.vtable);
        // The +0x94 object is the target plus 0x94: answer its slot 0x10.
        world.answer(target + 0x94, 0x10, 1);
        e.set(hit, HitData::fFatigueDamage, 5.0f32.to_bits());
        e.call(0x009b_5a30, &args![hit, 0u32]);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 0);
        assert_eq!(e.get(hit, HitData::fTotalDamage), 0);
        assert_eq!(e.get(hit, HitData::fFatigueDamage), 0);
    }

    #[test]
    fn check_critical_strike_rolls_against_the_chance() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let aggressor = world.object(&mut e);
        let target = world.object(&mut e);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pAggressor, aggressor);
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::pWeapon, 0x600);
        e.set(hit, HitData::fHealthDamage, 10.0f32.to_bits());
        e.register(IS_FLAG_SET, |_, _| 0u32.into_ret());
        // Chance 5% -> 50 out of 1000.
        e.register(0x0064_6d80, |_, _| float_ret(5.0));
        quiet(&mut e, &[HANDLE_ENTRY_POINT, SET_FLAG]);
        e.register(0x0064_5c20, |_, _| 15u32.into_ret());
        e.register(0x009b_73b0, |_, _| 1u32.into_ret());
        e.register(0x0051_f4d0, |_, _| 0x900u32.into_ret());
        e.register(0x0048_7f50, |_, _| 1049u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        let log = e.call_log.take().unwrap();
        // 1049 % 1000 = 49 < 50: a critical with 15 damage added.
        assert_eq!(
            calls(&log, 0x0064_6d80),
            vec![vec![aggressor + 0xa4, 0x600]]
        );
        assert_eq!(
            calls(&log, SET_FLAG),
            vec![vec![hit.addr(), 4, 1], vec![hit.addr(), 8, 1]]
        );
        assert_eq!(e.get(hit, HitData::fHealthDamage), 25.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::pCriticalEffect), 0x900);
        let entries = calls(&log, HANDLE_ENTRY_POINT);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[1][..4], [0x24, target, 0x600, aggressor]);
        assert_eq!(entries[2][..4], [2, aggressor, 0x600, target]);
        // 50 is not below 50.
        e.register(0x0048_7f50, |_, _| 50u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        assert!(calls(&e.call_log.take().unwrap(), SET_FLAG).is_empty());
        // Flag 0x800 skips everything.
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        assert!(calls(&e.call_log.take().unwrap(), 0x0064_6d80).is_empty());
    }

    #[test]
    fn determine_outcome_flags_a_kill_and_its_limb() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let target = world.object(&mut e);
        world.answer(target, 0x100, 1);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::fHealthDamage, 30.0f32.to_bits());
        e.set(hit, HitData::fTargetedLimbDamage, 30.0f32.to_bits());
        e.set(hit, HitData::eDamageLimb, 2);
        // Health 20 and the limb's value 25: both go to at most 0.
        let values = HashMap::from([(0x10u32, 20.0f64), (0x1a, 25.0)]);
        e.register_double(SLOT_BASE + 0xc, move |_, a| {
            float_ret(values.get(&a[1]).copied().unwrap_or(0.0))
        });
        let data = world.object(&mut e);
        e.register_double(0x0041_81e0, move |_, _| data.into_ret());
        world.answer(data, 0x180, 0x800);
        e.register(0x005e_50f0, |_, _| 0x810u32.into_ret());
        e.register(0x008b_4360, |_, _| 1u32.into_ret());
        e.register(0x008b_4cf0, |_, _| 40u32.into_ret());
        e.register(0x008b_4cd0, |_, _| 0u32.into_ret());
        e.register(0x005e_5190, |_, _| 0x1au32.into_ret());
        e.register(0x0064_6e50, |e, a| {
            e.mem.set_u8(a[6], 1);
            Ret::default()
        });
        quiet(&mut e, &[SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x009b_73d0, &args![hit]);
        let log = e.call_log.take().unwrap();
        let flags: Vec<u32> = calls(&log, SET_FLAG).iter().map(|a| a[1]).collect();
        assert_eq!(flags, vec![0x80, 0x10, 0x20]);
        let outcome = &calls(&log, 0x0064_6e50)[0];
        assert_eq!(outcome[..6], [target + 0xa4, 0, 40, 0, 0, 0]);
        // A non-character target is skipped.
        world.answer(target, 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_73d0, &args![hit]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn determine_outcome_scales_by_the_bonus() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let target = world.object(&mut e);
        world.answer(target, 0x100, 1);
        world.answer_float(target + 0xa4, 0xc, 1000.0);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::eDamageLimb, u32::MAX);
        e.set(hit, HitData::fHealthDamage, 10.0f32.to_bits());
        e.set(hit, HitData::fTotalDamage, 12.0f32.to_bits());
        e.set(hit, HitData::fFatigueDamage, 3.0f32.to_bits());
        e.set(hit, HitData::fBonusMult, 2.0f32.to_bits());
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        settings(&mut e, &[(0x011c_eae0, 1.5)]);
        quiet(&mut e, &[SET_FLAG]);
        e.call(0x009b_73d0, &args![hit]);
        assert_eq!(e.get(hit, HitData::fBonusMult), 3.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fHealthDamage), 30.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 36.0f32.to_bits());
        // No aggressor: the fatigue damage stays.
        assert_eq!(e.get(hit, HitData::fFatigueDamage), 3.0f32.to_bits());
    }
}
