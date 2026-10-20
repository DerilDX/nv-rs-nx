//! `fallout/ai/combat/combatstate.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is translated in address order over several sessions. The first
//! session took `006f2290` to `0099cb00` (60 functions); the next one starts
//! at `0099cb70`.
//!
//! Notes for the next session:
//! - `CombatState` (Xbox PDB, 0x22c bytes) is not a `TESForm`; its PC
//!   offsets are the PDB's, confirmed by the constructor `00997600` (every
//!   member it initialises sits where the PDB puts it). The layout below
//!   names the fields used so far.
//! - The build is unoptimised: every field is re-read from memory where the
//!   code re-reads it, and the translations do the same.
//! - Small callees confirmed from their bodies: `0097ae90` returns
//!   `CombatController` +0xbc (`pAttacker`), `004030b0` +0xc0 (`pTarget`),
//!   `004fb070` +0x80 (`pCombatGroup`); `00403e20` returns the address of a
//!   `Setting`'s value (+4, or a zeroed scratch float for a null setting);
//!   `00435dd0` loads the game time (`011f1bf0`); `00435de0` stores a float
//!   into a `CombatTimeStamp`; `00435e00` is the time since a stamp;
//!   `008d7f40` starts a `CombatTimer` (start = now, delay = argument),
//!   `008d7f80` says it has expired, `0097fa50` that its delay is not zero,
//!   `00910b10` clears the delay (the map names it
//!   `TESWeightForm::InitializeDataComponent`: folded code).
//! - Functions with a C++ exception-unwinding frame (`FS:[0]`) carry a
//!   `Translation incomplete` marker: the unwinding is not translated.
//! - Floats copied with `FLD`/`FSTP` go through [`x87_quiet`] /
//!   [`x87_quiet_bits`]; floats pushed as arguments are passed as bit
//!   patterns; `-FLT_MAX` is built as the exe does (`FLD` of `FLT_MAX`, then
//!   `FCHS`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};
use crate::units::fallout_ai::highprocess::CombatTimer;

layout! {
    /// `CombatState` (Xbox PDB), 0x22c bytes; PC offsets as in the PDB.
    pub struct CombatState: 0x22c {
        /// `bBlocking` (Xbox PDB).
        0x000 bBlocking: bool,
        /// `iLocationUnreachable` (Xbox PDB).
        0x004 iLocationUnreachable: i32,
        /// `fThreatRatio` (Xbox PDB).
        0x008 fThreatRatio: f32,
        /// `pCombatWeapons` (Xbox PDB): six `TESObjectWEAP*`.
        0x00c pCombatWeapons: u32,
        /// `EmbeddedWeaponsArray` (Xbox PDB):
        /// `BSSimpleArray<TESObjectWEAP *, 1024>`.
        0x024 EmbeddedWeaponsArray: Inline<BSSimpleArray>,
        /// `pCurrentEmbeddedWeapon` (Xbox PDB).
        0x034 pCurrentEmbeddedWeapon: u32,
        /// `iHasCombatWeapon` (Xbox PDB): one bit per weapon type.
        0x038 iHasCombatWeapon: u32,
        /// `fCombatWeaponsDPS` (Xbox PDB): seven floats.
        0x03c fCombatWeaponsDPS: f32,
        /// The seventh entry of `fCombatWeaponsDPS` (+0x54).
        0x054 fUnarmedDPS: f32,
        /// `fMaxCombatWeaponDPS` (Xbox PDB).
        0x058 fMaxCombatWeaponDPS: f32,
        /// `fMaxRangedWeaponDPS` (Xbox PDB).
        0x05c fMaxRangedWeaponDPS: f32,
        /// `fMaxModifiedCombatWeaponDPS` (Xbox PDB).
        0x060 fMaxModifiedCombatWeaponDPS: f32,
        /// `fDisarmedDPS` (Xbox PDB).
        0x064 fDisarmedDPS: f32,
        /// `fBetterWeaponOutOfRangeDistance` (Xbox PDB).
        0x068 fBetterWeaponOutOfRangeDistance: f32,
        /// `iTargetDetectionLevel` (Xbox PDB).
        0x06c iTargetDetectionLevel: i32,
        /// `bTargetLineOfSight` (Xbox PDB).
        0x070 bTargetLineOfSight: u8,
        /// `bTarget360LineOfSight` (Xbox PDB).
        0x071 bTarget360LineOfSight: u8,
        /// `bTargetBlocked` (Xbox PDB).
        0x072 bTargetBlocked: u8,
        /// `cTargetFlushed` (Xbox PDB).
        0x073 cTargetFlushed: i8,
        /// `bTargetSpaceDifferent` (Xbox PDB).
        0x074 bTargetSpaceDifferent: u8,
        /// `bHasHandToHandEquip` (Xbox PDB).
        0x075 bHasHandToHandEquip: u8,
        /// `bGrenadeThrowHighArc` (Xbox PDB).
        0x076 bGrenadeThrowHighArc: u8,
        /// `bInventoryDirty` (Xbox PDB).
        0x077 bInventoryDirty: u8,
        /// `pExplosiveProjectileBlocked` (Xbox PDB): `BGSExplosion*`.
        0x078 pExplosiveProjectileBlocked: u32,
        /// `eTargetSegmentInView` (Xbox PDB).
        0x07c eTargetSegmentInView: u32,
        /// `fThreatScore` (Xbox PDB).
        0x080 fThreatScore: f32,
        /// `LastSeenTargetLocation` (Xbox PDB): an `NiPoint3`.
        0x084 LastSeenTargetLocation: Inline<NiPoint3>,
        /// `iCombatPermissions` (Xbox PDB).
        0x090 iCombatPermissions: u32,
        /// `fLastTargetDetectedTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x094 fLastTargetDetectedTimeStamp: u32,
        /// `fLastTarget360LineOfSightTimeStamp` (Xbox PDB).
        0x098 fLastTarget360LineOfSightTimeStamp: u32,
        /// `fLastAttackTimeStamp` (Xbox PDB).
        0x09c fLastAttackTimeStamp: u32,
        /// `fLastDamagedByAttackerTimeStamp` (Xbox PDB).
        0x0a0 fLastDamagedByAttackerTimeStamp: u32,
        /// `fFiringStartedTimeStamp` (Xbox PDB).
        0x0a4 fFiringStartedTimeStamp: u32,
        /// `AttackDialogueTimer` (Xbox PDB): a `CombatTimer`.
        0x0a8 AttackDialogueTimer: Inline<CombatTimer>,
        /// `fThreatsConsideredTimeStamp` (Xbox PDB).
        0x0c0 fThreatsConsideredTimeStamp: u32,
        /// `pLocationTarget` (Xbox PDB): `CombatLocationTarget*`.
        0x0c4 pLocationTarget: u32,
        /// `cStartingConfidence` (Xbox PDB).
        0x0c8 cStartingConfidence: u8,
        /// `cCurrentConfidence` (Xbox PDB).
        0x0c9 cCurrentConfidence: u8,
        /// `fConfidenceModifier` (Xbox PDB).
        0x0cc fConfidenceModifier: f32,
        /// `fCombatStrength` (Xbox PDB).
        0x0d0 fCombatStrength: f32,
        /// `fDamagePerSecond` (Xbox PDB).
        0x0d4 fDamagePerSecond: f32,
        /// `cFleeState` (Xbox PDB).
        0x0d8 cFleeState: u8,
        /// `iLastFleeDoor` (Xbox PDB).
        0x0dc iLastFleeDoor: u32,
        /// `fLastFleeDoorTimeStamp` (Xbox PDB).
        0x0e0 fLastFleeDoorTimeStamp: u32,
        /// `pCoverLocation` (Xbox PDB): `CombatCoverLocation*`.
        0x0e4 pCoverLocation: u32,
        /// `pSecondaryCoverLocation` (Xbox PDB).
        0x0e8 pSecondaryCoverLocation: u32,
        /// `CoverSearchInfo` (Xbox PDB): a `CombatCoverSearchInfo`.
        0x0ec CoverSearchInfo: Inline<CombatCoverSearchInfo>,
        /// `SecondaryCoverSearchInfo` (Xbox PDB).
        0x128 SecondaryCoverSearchInfo: Inline<CombatCoverSearchInfo>,
        /// `UnreachableCoverLocations` (Xbox PDB).
        0x164 UnreachableCoverLocations: Inline<BSSimpleArray>,
        /// `UnreachableLocations` (Xbox PDB).
        0x174 UnreachableLocations: Inline<BSSimpleArray>,
        /// `fLastDodgeSearch` (Xbox PDB).
        0x184 fLastDodgeSearch: u32,
        /// `cLastDodgeDirection` (Xbox PDB).
        0x188 cLastDodgeDirection: u8,
        /// `pCombatLocation` (Xbox PDB): `CombatLocation*`.
        0x18c pCombatLocation: u32,
        /// `pCombatItems` (Xbox PDB): five `CombatItem*`.
        0x190 pCombatItems: u32,
        /// `pUsableCombatItems` (Xbox PDB): two `TESBoundObject*`.
        0x1a4 pUsableCombatItems: u32,
        /// `UsableCombatItemTimers` (Xbox PDB): two `CombatTimer`s.
        0x1ac UsableCombatItemTimers: Inline<CombatTimer>,
        /// `iHasCombatItem` (Xbox PDB).
        0x1bc iHasCombatItem: u32,
        /// `pLastTarget` (Xbox PDB): `Actor*`.
        0x1c0 pLastTarget: u32,
        /// `pCombatController` (Xbox PDB).
        0x1c4 pCombatController: u32,
        /// `bInitializing` (Xbox PDB).
        0x1c8 bInitializing: u8,
        /// `TargetUnreachableTimer` (Xbox PDB).
        0x1cc TargetUnreachableTimer: Inline<CombatTimer>,
        /// `FindBetterWeaponTimer` (Xbox PDB).
        0x1d4 FindBetterWeaponTimer: Inline<CombatTimer>,
        /// `ExplosiveProjectileBlockedTimer` (Xbox PDB).
        0x1dc ExplosiveProjectileBlockedTimer: Inline<CombatTimer>,
        /// `CombatPermissionsTimer5sec` (Xbox PDB).
        0x1e4 CombatPermissionsTimer5sec: Inline<CombatTimer>,
        /// `CombatPermissionsTimer10sec` (Xbox PDB).
        0x1ec CombatPermissionsTimer10sec: Inline<CombatTimer>,
        /// `GuardSneakTimer` (Xbox PDB).
        0x1f4 GuardSneakTimer: Inline<CombatTimer>,
        /// `DynamicCoverLostTimer` (Xbox PDB).
        0x1fc DynamicCoverLostTimer: Inline<CombatTimer>,
        /// `CombatStrengthUpdateTimer` (Xbox PDB).
        0x204 CombatStrengthUpdateTimer: Inline<CombatTimer>,
        /// `ThreatRatioUpdateTimer` (Xbox PDB).
        0x20c ThreatRatioUpdateTimer: Inline<CombatTimer>,
        /// `EmbeddedWeaponUpdateTimer` (Xbox PDB).
        0x214 EmbeddedWeaponUpdateTimer: Inline<CombatTimer>,
        /// `UpdateInventoryTimer` (Xbox PDB).
        0x21c UpdateInventoryTimer: Inline<CombatTimer>,
        /// `spPathCheckQueue` (Xbox PDB): `NiPointer<ActorPathingMessageQueue>`.
        0x224 spPathCheckQueue: u32,
        /// `iQueuedPathCheck` (Xbox PDB).
        0x228 iQueuedPathCheck: i32,
    }

    /// `CombatCoverSearchInfo` (Xbox PDB), 0x3c bytes.
    pub struct CombatCoverSearchInfo: 0x3c {
        /// `TargetLocation` (Xbox PDB): an `NiPoint3`.
        0x00 TargetLocation: Inline<NiPoint3>,
        /// `Center` (Xbox PDB): an `NiPoint3`.
        0x0c Center: Inline<NiPoint3>,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `fTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x1c fTimeStamp: u32,
        /// `CoverLocations` (Xbox PDB):
        /// `BSSimpleArray<PathingCoverLocation, 1024>`.
        0x20 CoverLocations: Inline<BSSimpleArray>,
        /// `iPreviousCoverLocationKey` (Xbox PDB).
        0x30 iPreviousCoverLocationKey: u32,
        /// `ePreviousCoverState` (Xbox PDB).
        0x34 ePreviousCoverState: u32,
        /// `bUsedLastSeenLocation` (Xbox PDB).
        0x38 bUsedLastSeenLocation: u8,
    }

    /// `CombatLocation` (Xbox PDB), 0x20 bytes: a `BGSWorldLocation`
    /// (point and space) then the reference, type, radius and flags.
    pub struct CombatLocation: 0x20 {
        /// `WorldLoc` (Xbox PDB): a `BGSWorldLocation` (16 bytes).
        0x00 WorldLoc: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x10 pRef: u32,
        /// `iLocation` (Xbox PDB).
        0x14 iLocation: i32,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `iFlags` (Xbox PDB).
        0x1c iFlags: u32,
    }

    /// `CombatLocationTarget` (Xbox PDB), 0x20 bytes.
    pub struct CombatLocationTarget: 0x20 {
        /// `cType` (Xbox PDB).
        0x00 cType: u8,
        /// `bNearTarget` (Xbox PDB).
        0x01 bNearTarget: u8,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x04 pRef: u32,
        /// `WorldLoc` (Xbox PDB): a `BGSWorldLocation` (16 bytes).
        0x08 WorldLoc: u32,
        /// `fMinDistance` (Xbox PDB).
        0x18 fMinDistance: f32,
        /// `fMaxDistance` (Xbox PDB).
        0x1c fMaxDistance: f32,
    }
}

/// `FLT_MAX` (`float`); `-FLT_MAX` is built from it with `FCHS`.
const MAX_FLOAT: u32 = 0x0108_df70;
/// `100.0` (`float`).
const HUNDRED: u32 = 0x0101_6410;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `2.0` (`float`).
const TWO: u32 = 0x0101_62c0;
/// `0.25` (`float`).
const QUARTER: u32 = 0x0101_622c;
/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `0.1` (`float`).
const TENTH: u32 = 0x0101_e2bc;
/// `5.0` (`float`).
const FIVE: u32 = 0x0101_712c;
/// `10.0` (`float`).
const TEN: u32 = 0x0101_7b78;
/// `3.0` (`float`).
const THREE: u32 = 0x0101_7718;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `256.0` (`double`).
const TWO_HUNDRED_FIFTY_SIX_DOUBLE: u32 = 0x0102_31d8;
/// `100.0` (`double`).
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// `120.0` (`double`).
const HUNDRED_TWENTY_DOUBLE: u32 = 0x0107_18b8;
/// `10.0` (`double`).
const TEN_DOUBLE: u32 = 0x0102_0758;
/// `65536.0` (`double`).
const SIXTY_FIVE_THOUSAND_DOUBLE: u32 = 0x0108_e058;
/// Three floats at `011f426c` (zero in the exe's data) that the code copies
/// as a default point.
const ZERO_POINT: u32 = 0x011f_426c;
/// The pointer at `011f1958` that `00992720` and `009a5670`'s getter are
/// called on (the combat manager in other units).
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// The pointer at `011f1708` that `CombatDialogueManager::StartDialogue`
/// (`00983970`) is called on.
const COMBAT_DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// The pointer at `011dea3c` (the player character in other units).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A byte switch at `011f1ae4` that turns the location-target search of
/// `00997cf0` on.
const LOCATION_TARGET_SEARCH_011F1AE4: u32 = 0x011f_1ae4;
/// A byte switch at `011a4d34` (1 in the exe's data) choosing between the
/// two location-target searches of `00997cf0`.
const LOCATION_TARGET_MODE_011A4D34: u32 = 0x011a_4d34;
/// Three floats at `011a5308`, `011a530c`, `011a5310` (1024, 768, 384 in the
/// exe's data) that `00997cf0` passes to the location-target search.
const LOCATION_TARGET_MAX_DISTANCE: u32 = 0x011a_5308;
const LOCATION_TARGET_SECOND_DISTANCE: u32 = 0x011a_530c;
const LOCATION_TARGET_FIRST_DISTANCE: u32 = 0x011a_5310;
/// The 8-byte entries per combat location type that `0099ac20` reads: a byte
/// at +0, a byte at +1 and a float at +4.
const LOCATION_TYPE_TABLE: u32 = 0x011a_5650;

/// `Setting` value address (`00403e20`): +4 of the setting, or a zeroed
/// scratch float for a null setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The game time in ST0 (`00435dd0`).
const GAME_TIME: u32 = 0x0043_5dd0;
/// Stores its float argument into a `CombatTimeStamp` (`00435de0`).
const TIME_STAMP_SET: u32 = 0x0043_5de0;
/// The time since a `CombatTimeStamp`, in ST0 (`00435e00`).
const TIME_STAMP_ELAPSED: u32 = 0x0043_5e00;
/// `CombatTimer`: start with a delay (`008d7f40`).
const TIMER_START: u32 = 0x008d_7f40;
/// `CombatTimer`: the delay has passed (`008d7f80`).
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// `CombatTimer`: the delay is not zero (`0097fa50`).
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `CombatTimer`: clears the delay (`00910b10`).
const TIMER_STOP: u32 = 0x0091_0b10;
/// `CombatController` +0xbc (`0097ae90`).
const CONTROLLER_ATTACKER: u32 = 0x0097_ae90;
/// `CombatController` +0xc0 (`004030b0`).
const CONTROLLER_TARGET: u32 = 0x0040_30b0;
/// `CombatController` +0x80 (`004fb070`).
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// `RandomFloat` (Xbox PDB, `00476b70`): cdecl, two floats.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `TESObjectREFR::GetWorldLocation` (Xbox PDB, `00575650`).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `TESObjectREFR::GetSpace` (Xbox PDB, `00575ca0`).
const GET_SPACE: u32 = 0x0057_5ca0;
/// `memset`-shaped cdecl helper (`00403d30`): address, byte, size.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `BSSimpleArray` size getter (`0044ddc0`, +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `CombatState::ClearCoverLocation` (Xbox PDB, `009a03d0`).
const CLEAR_COVER_LOCATION: u32 = 0x009a_03d0;
/// `CombatState::ClearSecondaryCoverLocation` (Xbox PDB, `009a0470`).
const CLEAR_SECONDARY_COVER_LOCATION: u32 = 0x009a_0470;

/// `-FLT_MAX` as the exe builds it (`FLD FLT_MAX`, `FCHS`), as bits.
fn negative_max_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(MAX_FLOAT)) ^ 0x8000_0000
}

/// A float constant pushed through `FLD`/`FSTP`, as bits.
fn constant_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// The address of a `Setting`'s value (`00403e20`).
fn setting_value_address(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// A `Setting`'s value loaded with `FLD` and pushed with `FSTP`, as bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = setting_value_address(e, setting);
    x87_quiet_bits(e.mem.u32(value))
}

/// A `Setting`'s value as `FLD` loads it for a comparison.
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let value = setting_value_address(e, setting);
    e.mem.f32(value)
}

/// The word a `CombatTimeStamp` local holds after `00435de0` stored `bits`
/// into it (the code then copies the local into the member).
fn time_stamp_word(e: &mut Engine, bits: u32) -> u32 {
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, bits]);
        e.mem.u32(stamp.addr())
    })
}

/// The game time (`00435dd0`) as the `FSTP` to the stack rounds it, as bits.
fn game_time_bits(e: &mut Engine) -> u32 {
    e.call(GAME_TIME, &args![]).f32().to_bits()
}

fn state_controller(e: &Engine, this: Ptr<CombatState>) -> u32 {
    e.get(this, CombatState::pCombatController)
}

fn attacker(e: &mut Engine, this: Ptr<CombatState>) -> u32 {
    let controller = state_controller(e, this);
    e.call(CONTROLLER_ATTACKER, &args![controller]).u32()
}

fn target(e: &mut Engine, this: Ptr<CombatState>) -> u32 {
    let controller = state_controller(e, this);
    e.call(CONTROLLER_TARGET, &args![controller]).u32()
}

fn timer_start(e: &mut Engine, timer: u32, delay_bits: u32) {
    e.call(TIMER_START, &args![timer, delay_bits]);
}

fn timer_expired(e: &mut Engine, timer: u32) -> bool {
    e.call(TIMER_EXPIRED, &args![timer]).bool()
}

fn timer_running(e: &mut Engine, timer: u32) -> bool {
    e.call(TIMER_RUNNING, &args![timer]).bool()
}

fn timer_stop(e: &mut Engine, timer: u32) {
    e.call(TIMER_STOP, &args![timer]);
}

/// `RandomFloat(low, high)` (`00476b70`) as the `FSTP` to the stack rounds
/// it, as bits.
fn random_float_bits(e: &mut Engine, low_bits: u32, high_bits: u32) -> u32 {
    e.call(RANDOM_FLOAT, &args![low_bits, high_bits])
        .f32()
        .to_bits()
}

/// Copies `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The address of member `off` of the state.
fn member(this: Ptr<CombatState>, off: u32) -> u32 {
    this.addr().wrapping_add(off)
}

// Translated from 006f2290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESObjectWEAP *, 1024>::SetSize` (Xbox PDB): resizes the
/// array to `size` elements. Size 0 hands over to `008454f0` (with
/// `shrink`); growing past the reserved size allocates through the array's
/// own vtable slot 4 (empty array) or reallocates (`0042fc00`), then
/// constructs the new elements (`00726bf0`); shrinking destroys the
/// elements past `size` (`0072ba80`) and, with `shrink` and at most a
/// quarter of the reservation in use, reallocates to `size`.
pub fn bs_simple_array_tes_object_weap_set_size(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    size: u32,
    shrink: u8,
) {
    if size == 0 {
        e.call(0x0084_54f0, &args![this, shrink as u32]);
    } else if size > e.get(this, BSSimpleArray::iReservedSize) {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 0x4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let used = e.get(this, BSSimpleArray::iSize);
            e.call(0x0042_fc00, &args![this, size, used]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(used.wrapping_mul(4));
        e.call(0x0072_6bf0, &args![this, start, size.wrapping_sub(used)]);
        e.set(this, BSSimpleArray::iSize, size);
    } else if size < e.get(this, BSSimpleArray::iSize) {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(size.wrapping_mul(4));
        e.call(0x0072_ba80, &args![this, start, used.wrapping_sub(size)]);
        e.set(this, BSSimpleArray::iSize, size);
        if shrink != 0 && size <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(0x0042_fc00, &args![this, size, size]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    } else {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(used.wrapping_mul(4));
        e.call(0x0072_6bf0, &args![this, start, size.wrapping_sub(used)]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0097ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a `CombatLocation`: the `BGSWorldLocation` part through
/// `0097ede0`, then no reference, location -1, radius 0 and no flags;
/// returns `this`. The map's name `BGSWorldLocation::SetSpace` is folded
/// code; the body matches `CombatLocation`'s layout (Xbox PDB).
pub fn fn_0097ee20(e: &mut Engine, this: Ptr<CombatLocation>) -> Ptr<CombatLocation> {
    e.call(0x0097_ede0, &args![this]);
    e.set(this, CombatLocation::pRef, 0);
    e.set(this, CombatLocation::iLocation, -1);
    e.set(this, CombatLocation::fRadius, 0.0);
    e.set(this, CombatLocation::iFlags, 0);
    this
}

// Translated from 009973a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `CombatLocation` at `other` into `this` (the radius through
/// `FLD`/`FSTP`).
pub fn fn_009973a0(e: &mut Engine, this: Ptr<CombatLocation>, other: Ptr<CombatLocation>) {
    copy_words(e, this.addr(), other.addr(), 4);
    let reference = e.get(other, CombatLocation::pRef);
    e.set(this, CombatLocation::pRef, reference);
    let location = e.get(other, CombatLocation::iLocation);
    e.set(this, CombatLocation::iLocation, location);
    let radius = e.get(other, CombatLocation::fRadius);
    e.set(this, CombatLocation::fRadius, x87_quiet(radius));
    let flags = e.get(other, CombatLocation::iFlags);
    e.set(this, CombatLocation::iFlags, flags);
}

// Translated from 00997400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatLocation::SetRef` (Xbox PDB): stores the reference and copies its
/// world location (`TESObjectREFR::GetWorldLocation`) into `WorldLoc`.
pub fn combat_location_set_ref(e: &mut Engine, this: Ptr<CombatLocation>, reference: u32) {
    e.set(this, CombatLocation::pRef, reference);
    combat_location_copy_ref_location(e, this);
}

/// The world location of `pRef` (through a 16-byte local) copied into
/// `WorldLoc`, as `00997400` and `00997440` do.
fn combat_location_copy_ref_location(e: &mut Engine, this: Ptr<CombatLocation>) {
    e.with_stack(16, |e, location| {
        let reference = e.get(this, CombatLocation::pRef);
        let result = e
            .call(GET_WORLD_LOCATION, &args![reference, location])
            .u32();
        copy_words(e, this.addr(), result, 4);
    });
}

// Translated from 00997440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatLocation::UpdateRefLocation` (Xbox PDB): with a reference, copies
/// its current world location into `WorldLoc`.
pub fn combat_location_update_ref_location(e: &mut Engine, this: Ptr<CombatLocation>) {
    if e.get(this, CombatLocation::pRef) != 0 {
        combat_location_copy_ref_location(e, this);
    }
}

// Translated from 00997480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `CombatCoverLocation` (Xbox PDB layout, 0x78 bytes) at `other`
/// into `this`: the three flag bytes, the location, the search point, the
/// `PathingCoverLocation` at +0x14 (through `006e4430` on `this`'s), the key,
/// the time stamp and the six-byte reservation information.
pub fn fn_00997480(e: &mut Engine, this: Ptr, other: Ptr) {
    let (to, from) = (this.addr(), other.addr());
    for off in 0..3 {
        let byte = e.mem.u8(from + off);
        e.mem.set_u8(to + off, byte);
    }
    copy_words(e, to + 0x4, from + 0x4, 1);
    copy_words(e, to + 0x8, from + 0x8, 3);
    e.call(0x006e_4430, &args![to + 0x14, from + 0x14]);
    copy_words(e, to + 0x68, from + 0x68, 1);
    copy_words(e, to + 0x6c, from + 0x6c, 1);
    copy_words(e, to + 0x70, from + 0x70, 1);
    let half = e.mem.u16(from + 0x74);
    e.mem.set_u16(to + 0x74, half);
}

// Translated from 00997520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `CombatCoverSearchInfo` at `other` to `this`; a null `other`
/// resets `this` (`009975c0`). The cover location array is copied by
/// `009a3780`, which the code calls on `other`'s array with `this`'s as the
/// argument.
pub fn fn_00997520(
    e: &mut Engine,
    this: Ptr<CombatCoverSearchInfo>,
    other: Ptr<CombatCoverSearchInfo>,
) {
    if other.is_null() {
        fn_009975c0(e, this);
        return;
    }
    copy_words(e, this.addr(), other.addr(), 3);
    copy_words(e, this.addr() + 0xc, other.addr() + 0xc, 3);
    let radius = e.get(other, CombatCoverSearchInfo::fRadius);
    e.set(this, CombatCoverSearchInfo::fRadius, x87_quiet(radius));
    let stamp = e.get(other, CombatCoverSearchInfo::fTimeStamp);
    e.set(this, CombatCoverSearchInfo::fTimeStamp, stamp);
    e.call(0x009a_3780, &args![other.addr() + 0x20, this.addr() + 0x20]);
    let key = e.get(other, CombatCoverSearchInfo::iPreviousCoverLocationKey);
    e.set(this, CombatCoverSearchInfo::iPreviousCoverLocationKey, key);
    let state = e.get(other, CombatCoverSearchInfo::ePreviousCoverState);
    e.set(this, CombatCoverSearchInfo::ePreviousCoverState, state);
    let used = e.get(other, CombatCoverSearchInfo::bUsedLastSeenLocation);
    e.set(this, CombatCoverSearchInfo::bUsedLastSeenLocation, used);
}

// Translated from 009975c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `CombatCoverSearchInfo`: radius 0, time stamp `-FLT_MAX`, and the
/// cover location array emptied through `006dbde0(1)`.
pub fn fn_009975c0(e: &mut Engine, this: Ptr<CombatCoverSearchInfo>) {
    e.set(this, CombatCoverSearchInfo::fRadius, 0.0);
    let bits = negative_max_bits(e);
    let stamp = time_stamp_word(e, bits);
    e.set(this, CombatCoverSearchInfo::fTimeStamp, stamp);
    e.call(0x006d_bde0, &args![this.addr() + 0x20, 1u32]);
}

// Translated from 00997600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00997600 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatState` constructor: initialises every member (time stamps to
/// `-FLT_MAX`, timers through `00906f60`, the arrays and cover search
/// information through their constructors), stores the controller, clears
/// the weapon, DPS, item and usable-item arrays, starts the threat-ratio
/// timer with a random delay below the setting at `011cf804`, and sets the
/// confidence modifier to a random value between the settings at `011cfdcc`
/// and `011cf7ec`. The second argument word is not read.
pub fn fn_00997600(
    e: &mut Engine,
    this: Ptr<CombatState>,
    combat_controller: u32,
    _unused_2: u32,
) -> Ptr<CombatState> {
    let at = |off: u32| member(this, off);
    e.set(this, CombatState::bBlocking, false);
    e.set(this, CombatState::iLocationUnreachable, 0);
    let ratio = constant_bits(e, HUNDRED);
    e.set(this, CombatState::fThreatRatio, f32::from_bits(ratio));
    e.call(0x009a_33e0, &args![at(0x24)]);
    e.set(this, CombatState::pCurrentEmbeddedWeapon, 0);
    e.set(this, CombatState::iHasCombatWeapon, 0);
    e.set(this, CombatState::fMaxCombatWeaponDPS, 0.0);
    e.set(this, CombatState::fMaxRangedWeaponDPS, 0.0);
    e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, 0.0);
    e.set(this, CombatState::fDisarmedDPS, 0.0);
    e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, 0.0);
    e.set(this, CombatState::iTargetDetectionLevel, -1);
    e.set(this, CombatState::bTargetLineOfSight, 0);
    e.set(this, CombatState::bTarget360LineOfSight, 0);
    e.set(this, CombatState::bTargetBlocked, 0);
    e.set(this, CombatState::cTargetFlushed, -1);
    e.set(this, CombatState::bTargetSpaceDifferent, 0);
    e.set(this, CombatState::bHasHandToHandEquip, 0);
    e.set(this, CombatState::bGrenadeThrowHighArc, 0);
    e.set(this, CombatState::bInventoryDirty, 0);
    e.set(this, CombatState::pExplosiveProjectileBlocked, 0);
    e.set(this, CombatState::eTargetSegmentInView, 3);
    e.set(this, CombatState::fThreatScore, 0.0);
    copy_words(e, at(0x84), ZERO_POINT, 3);
    e.set(this, CombatState::iCombatPermissions, 0);
    for off in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
        let bits = negative_max_bits(e);
        e.call(TIME_STAMP_SET, &args![at(off), bits]);
    }
    for off in [0xa8, 0xb0, 0xb8] {
        e.call(0x0090_6f60, &args![at(off)]);
    }
    e.call(0x0043_00a0, &args![at(0xc0)]);
    e.set(this, CombatState::pLocationTarget, 0);
    e.set(this, CombatState::cStartingConfidence, 0);
    e.set(this, CombatState::cCurrentConfidence, 0);
    let minus_one = f32::from_bits(constant_bits(e, MINUS_ONE));
    e.set(this, CombatState::fCombatStrength, minus_one);
    e.set(this, CombatState::fDamagePerSecond, minus_one);
    e.set(this, CombatState::cFleeState, 0);
    e.set(this, CombatState::iLastFleeDoor, 0);
    let bits = negative_max_bits(e);
    e.call(TIME_STAMP_SET, &args![at(0xe0), bits]);
    e.set(this, CombatState::pCoverLocation, 0);
    e.set(this, CombatState::pSecondaryCoverLocation, 0);
    e.call(0x006d_32e0, &args![at(0xec)]);
    e.call(0x006d_32e0, &args![at(0x128)]);
    e.call(0x009a_3430, &args![at(0x164)]);
    e.call(0x009a_35c0, &args![at(0x174)]);
    e.call(0x0043_00a0, &args![at(0x184)]);
    e.set(this, CombatState::cLastDodgeDirection, 0xff);
    e.set(this, CombatState::pCombatLocation, 0);
    // `vector constructor iterator` (stdcall): two 8-byte timers built by
    // `00906f60`.
    e.call(0x0040_1050, &args![at(0x1ac), 8u32, 2u32, 0x0090_6f60u32]);
    e.set(this, CombatState::iHasCombatItem, 0);
    e.set(this, CombatState::pLastTarget, 0);
    e.set(this, CombatState::pCombatController, combat_controller);
    e.set(this, CombatState::bInitializing, 1);
    for off in [
        0x1cc, 0x1d4, 0x1dc, 0x1e4, 0x1ec, 0x1f4, 0x1fc, 0x204, 0x20c, 0x214, 0x21c,
    ] {
        e.call(0x0090_6f60, &args![at(off)]);
    }
    e.call(0x006e_bf60, &args![at(0x224), 0u32]);
    e.set(this, CombatState::iQueuedPathCheck, -1);
    e.call(FILL_BYTES, &args![at(0xc), 0u32, 0x18u32]);
    e.call(FILL_BYTES, &args![at(0x3c), 0u32, 0x1cu32]);
    e.call(FILL_BYTES, &args![at(0x190), 0u32, 0x14u32]);
    e.call(FILL_BYTES, &args![at(0x1a4), 0u32, 0x8u32]);
    let high = setting_bits(e, 0x011c_f804);
    let delay = random_float_bits(e, 0.0f32.to_bits(), high);
    timer_start(e, at(0x20c), delay);
    let high = setting_bits(e, 0x011c_f7ec);
    let low = setting_bits(e, 0x011c_fdcc);
    let modifier = e.call(RANDOM_FLOAT, &args![low, high]).f32();
    e.set(this, CombatState::fConfidenceModifier, modifier);
    this
}

// Translated from 00997aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00997aa0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatState` destructor body: clears the weapon arrays
/// (`0099a2a0`), both cover locations, the combat location (`009a0680`) and
/// the location target (`009a0a40`), releases the five combat items
/// (`00981340` with indices 0 to 4), then destroys the path-check queue
/// pointer, the two unreachable arrays, both cover search informations and
/// the embedded weapon array.
pub fn fn_00997aa0(e: &mut Engine, this: Ptr<CombatState>) {
    fn_0099a2a0(e, this);
    e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
    e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
    e.call(0x009a_0680, &args![this]);
    e.call(0x009a_0a40, &args![this]);
    for index in 0u32..5 {
        e.call(0x0098_1340, &args![this, index]);
    }
    e.call(0x006e_bf90, &args![member(this, 0x224)]);
    e.call(0x009a_35f0, &args![member(this, 0x174)]);
    e.call(0x009a_3460, &args![member(this, 0x164)]);
    e.call(0x006e_4d00, &args![member(this, 0x128)]);
    e.call(0x006e_4d00, &args![member(this, 0xec)]);
    e.call(0x009a_3410, &args![member(this, 0x24)]);
}

// Translated from 00997ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `first` is zero and `second` is not: clears both cover locations
/// and both cover search informations, and the combat location
/// (`009a0680`).
pub fn fn_00997ba0(e: &mut Engine, this: Ptr<CombatState>, first: u32, second: u32) {
    if first == 0 && second != 0 {
        e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
        fn_00997bf0(e, this);
        e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
        fn_00997c10(e, this);
        e.call(0x009a_0680, &args![this]);
    }
}

// Translated from 00997bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets `CoverSearchInfo` (`009975c0`).
pub fn fn_00997bf0(e: &mut Engine, this: Ptr<CombatState>) {
    fn_009975c0(e, this.at(CombatState::CoverSearchInfo));
}

// Translated from 00997c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets `SecondaryCoverSearchInfo` (`009975c0`).
pub fn fn_00997c10(e: &mut Engine, this: Ptr<CombatState>) {
    fn_009975c0(e, this.at(CombatState::SecondaryCoverSearchInfo));
}

// Translated from 00997c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// First update of the state: records whether the attacker has hand-to-hand
/// equipment (`CombatController::CheckHasEquip(attacker, 0)`), updates the
/// weapons (`009993c0`) and the state (`00997cb0`), sets both confidences
/// from the attacker's +0xa4 object (virtual slot 8 with 1), and ends the
/// initialisation.
pub fn fn_00997c30(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let equipped = e.call(0x0097_f310, &args![attacker, 0u32]).u8();
    e.set(this, CombatState::bHasHandToHandEquip, equipped);
    fn_009993c0(e, this);
    fn_00997cb0(e, this);
    let confidence = e.vcall(attacker.wrapping_add(0xa4), 0x8, &args![1u32]).u8();
    e.set(this, CombatState::cStartingConfidence, confidence);
    let starting = e.get(this, CombatState::cStartingConfidence);
    e.set(this, CombatState::cCurrentConfidence, starting);
    e.set(this, CombatState::bInitializing, 0);
}

// Translated from 00997cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the state: with the attacker's current process type 0
/// (`MobileObject::GetCurrentProcessType`), the full update (`00997cf0`,
/// then `00998a50`); otherwise the reduced one (`00999160`).
pub fn fn_00997cb0(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    if e.call(0x0093_1850, &args![attacker]).u32() == 0 {
        fn_00997cf0(e, this);
        fn_00998a50(e, this);
    } else {
        fn_00999160(e, this);
    }
}

// Translated from 00997cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The full per-frame update of the target information:
/// - whether the target is in another space (and dropping line of sight
///   when it leaves, `0097f200`);
/// - without a target: detection level -1, no 360-degree line of sight, the
///   detection and line-of-sight stamps `-FLT_MAX`, the flush state -1;
///   with one, a detection update through the group (`009887b0`) when the
///   controller asks for it or the target changed, the last-seen location
///   and stamp while the target is in 360-degree view, the flush state;
/// - the combat items (`0099a530`), the dynamic-cover timer, the location
///   target (searched through the group when the switch at `011f1ae4` is
///   on, then kept up to date: its reference's location, whether the
///   attacker is near it, and what that means for each target type), the
///   threat score (`009a5670`), the firing stamp, the out-of-range weapon
///   check, the inventory update (`009993c0`), the confidence (`009a0c50`)
///   and the flee door, and finally remembers the target.
pub fn fn_00997cf0(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let attacker_space = e.call(GET_SPACE, &args![attacker]).u32();

    if target != 0 && attacker_space != e.call(GET_SPACE, &args![target]).u32() {
        if e.get(this, CombatState::bTargetSpaceDifferent) == 0 {
            if e.get(this, CombatState::bTargetLineOfSight) != 0 {
                let controller = state_controller(e, this);
                e.call(0x0097_f200, &args![controller]);
            }
            e.set(this, CombatState::bTargetSpaceDifferent, 1);
        }
    } else {
        e.set(this, CombatState::bTargetSpaceDifferent, 0);
    }

    if target == 0 {
        e.set(this, CombatState::iTargetDetectionLevel, -1);
        e.set(this, CombatState::bTarget360LineOfSight, 0);
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fLastTargetDetectedTimeStamp, stamp);
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fLastTarget360LineOfSightTimeStamp, stamp);
        e.set(this, CombatState::cTargetFlushed, -1);
    } else {
        let controller = state_controller(e, this);
        let updated = fn_00998a10(e, controller) != 0;
        if updated || e.get(this, CombatState::pLastTarget) != target {
            e.with_stack(4, |e, level| {
                e.mem.set_u32(level.addr(), 0xffff_ff9c);
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                e.call(
                    0x0098_87b0,
                    &args![
                        group,
                        attacker,
                        target,
                        level,
                        member(this, 0x94),
                        member(this, 0x70),
                        member(this, 0x71)
                    ],
                );
                let detection = e.mem.i32(level.addr());
                e.set(this, CombatState::iTargetDetectionLevel, detection);
            });
            let controller = state_controller(e, this);
            e.call(0x008f_6630, &args![controller, 0u32]);
        }
        if e.get(this, CombatState::bTarget360LineOfSight) != 0 {
            let now = game_time_bits(e);
            let stamp = time_stamp_word(e, now);
            e.set(this, CombatState::fLastTarget360LineOfSightTimeStamp, stamp);
            let location = e.vcall(target, 0x1f4, &args![]).u32();
            copy_words(e, member(this, 0x84), location, 3);
            let controller = state_controller(e, this);
            let item = e.call(0x004f_9bf0, &args![controller]).u32();
            if item == 0 || e.call(0x0082_5c00, &args![item]).u32() != 0xd {
                e.set(this, CombatState::cTargetFlushed, -1);
                e.set(this, CombatState::bGrenadeThrowHighArc, 0);
            }
        }
        if e.get(this, CombatState::bTarget360LineOfSight) != 0
            && e.get(this, CombatState::bTargetBlocked) == 0
        {
            let controller = state_controller(e, this);
            if e.call(0x0098_1940, &args![controller, 0x18u32]).bool() {
                let controller = state_controller(e, this);
                e.call(0x0097_f200, &args![controller]);
            }
        }
        if !fn_00998980(e, this) && e.get(this, CombatState::cTargetFlushed) == -1 {
            e.set(this, CombatState::cTargetFlushed, 0);
        }
    }

    fn_0099a530(e, this);

    let dynamic_cover_timer = member(this, 0x1fc);
    if timer_running(e, dynamic_cover_timer) {
        if timer_expired(e, dynamic_cover_timer) {
            e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
            timer_stop(e, dynamic_cover_timer);
        }
    } else {
        let cover = e.get(this, CombatState::pCoverLocation);
        if cover != 0 && e.mem.i32(cover + 0x6c) != -1 {
            let cover = e.get(this, CombatState::pCoverLocation);
            let key = e.mem.u32(cover + 0x6c);
            let cover = e.get(this, CombatState::pCoverLocation);
            if !e.call(0x006e_4650, &args![cover + 0x14, key]).bool() {
                let delay = constant_bits(e, TWO);
                timer_start(e, dynamic_cover_timer, delay);
            }
        }
    }

    if e.global::<u8>(LOCATION_TARGET_SEARCH_011F1AE4) != 0 {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if location_target == 0 || e.mem.u8(location_target) == 2 {
            let mut first = x87_quiet(e.global::<f32>(LOCATION_TARGET_FIRST_DISTANCE));
            let mut second = x87_quiet(e.global::<f32>(LOCATION_TARGET_SECOND_DISTANCE));
            let mut maximum = x87_quiet(e.global::<f32>(LOCATION_TARGET_MAX_DISTANCE));
            if e.call(0x009a_9630, &args![attacker]).bool() {
                first = (first as f64 + first as f64) as f32;
                second = (second as f64 + second as f64) as f32;
            }
            let margin: f64 = e.global(TWO_HUNDRED_FIFTY_SIX_DOUBLE);
            if (maximum as f64) < second as f64 + margin {
                let margin: f64 = e.global(TWO_HUNDRED_FIFTY_SIX_DOUBLE);
                maximum = (second as f64 + margin) as f32;
            }
            let first_bits = x87_quiet_bits(first.to_bits());
            let second_bits = x87_quiet_bits(second.to_bits());
            if e.global::<u8>(LOCATION_TARGET_MODE_011A4D34) != 0 {
                let mut reference = 0;
                let location_target = e.get(this, CombatState::pLocationTarget);
                if location_target != 0 && e.mem.u32(location_target + 4) != 0 {
                    let location_target = e.get(this, CombatState::pLocationTarget);
                    let object = e.mem.u32(location_target + 4);
                    if e.vcall(object, 0x100, &args![]).bool() {
                        let location_target = e.get(this, CombatState::pLocationTarget);
                        reference = e.mem.u32(location_target + 4);
                    }
                }
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                let maximum_bits = x87_quiet_bits(maximum.to_bits());
                let found = e
                    .call(
                        0x0098_cd50,
                        &args![group, attacker, reference, maximum_bits],
                    )
                    .u32();
                if found != 0 {
                    e.call(
                        0x009a_0790,
                        &args![this, found, 2u32, first_bits, second_bits],
                    );
                } else {
                    e.call(0x009a_0a40, &args![this]);
                }
            } else {
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                let found = e.call(0x0098_de00, &args![group, attacker]).u32();
                if found != 0 {
                    e.call(
                        0x009a_0910,
                        &args![this, found, 2u32, first_bits, second_bits],
                    );
                } else {
                    e.call(0x009a_0a40, &args![this]);
                }
            }
        }
    } else {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if location_target != 0 && e.mem.u8(location_target) == 2 {
            e.call(0x009a_0a40, &args![this]);
        }
    }

    if e.get(this, CombatState::pLocationTarget) != 0 {
        update_location_target(e, this, attacker, target, attacker_space);
    }

    let previous_score = x87_quiet(e.get(this, CombatState::fThreatScore));
    let controller = state_controller(e, this);
    let combat_plan = e.call(0x0098_0070, &args![controller]).u32();
    if !fn_009989d0(e, combat_plan) {
        let threshold = setting_value(e, 0x011c_fbcc);
        let score = e.get(this, CombatState::fThreatScore);
        if threshold < score || e.get(this, CombatState::iCombatPermissions) & 0x100 != 0 {
            let location = e.vcall(attacker, 0x1f4, &args![]).u32();
            let (x, y, z) = (
                e.mem.u32(location),
                e.mem.u32(location + 4),
                e.mem.u32(location + 8),
            );
            let manager: u32 = e.global(COMBAT_MANAGER);
            let threat_map = e.call(0x0082_5c00, &args![manager]).u32();
            let stamp = e.get(this, CombatState::fThreatsConsideredTimeStamp);
            let score = e
                .call(
                    0x009a_5670,
                    &args![
                        threat_map,
                        x,
                        y,
                        z,
                        attacker,
                        0.0f32.to_bits(),
                        0.0f32.to_bits(),
                        stamp,
                        0.0f32.to_bits()
                    ],
                )
                .f32();
            e.set(this, CombatState::fThreatScore, score);
            let threshold = setting_value(e, 0x011c_fbcc);
            let score = e.get(this, CombatState::fThreatScore);
            if threshold <= score {
                let threshold = setting_value(e, 0x011c_fbcc);
                if threshold > previous_score {
                    let controller = state_controller(e, this);
                    e.call(0x0097_f200, &args![controller]);
                }
            }
        } else {
            e.set(this, CombatState::fThreatScore, 0.0);
        }
    }

    let firing = e.call(0x006a_7f50, &args![member(this, 0xa4)]).f64();
    let lowest = f32::from_bits(negative_max_bits(e)) as f64;
    if lowest != firing && !e.call(0x0089_4900, &args![attacker]).bool() {
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fFiringStartedTimeStamp, stamp);
    }

    if target != 0 {
        let zero: f64 = e.global(ZERO_DOUBLE);
        let range = e.get(this, CombatState::fBetterWeaponOutOfRangeDistance);
        if range as f64 > zero {
            e.with_stack(32, |e, locations| {
                let (mine, theirs) = (locations.addr(), locations.addr() + 16);
                e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
                e.call(GET_WORLD_LOCATION, &args![target, theirs]);
                let distance = e.call(0x0052_7a40, &args![mine, theirs]).f64();
                let range = e.get(this, CombatState::fBetterWeaponOutOfRangeDistance) as f64;
                if range * range > distance {
                    e.call(0x0097_f220, &args![member(this, 0x21c)]);
                }
            });
        }
    }

    let inventory_timer = member(this, 0x21c);
    if e.get(this, CombatState::bInventoryDirty) != 0
        || (timer_running(e, inventory_timer) && timer_expired(e, inventory_timer))
    {
        fn_009993c0(e, this);
    }
    e.call(0x009a_0c50, &args![this, 0u32]);

    if e.get(this, CombatState::iLastFleeDoor) != 0 {
        let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xe0)]).f64();
        let limit = setting_value(e, 0x011c_eed8);
        if (limit as f64) < elapsed {
            e.set(this, CombatState::iLastFleeDoor, 0);
        }
    }
    e.set(this, CombatState::pLastTarget, target);
}

/// `pLocationTarget`, re-read.
fn current_location_target(e: &Engine, this: Ptr<CombatState>) -> Ptr<CombatLocationTarget> {
    Ptr::new(e.get(this, CombatState::pLocationTarget))
}

/// The location-target part of `00997cf0` (`pLocationTarget` is set).
fn update_location_target(
    e: &mut Engine,
    this: Ptr<CombatState>,
    attacker: u32,
    target: u32,
    attacker_space: u32,
) {
    let lt = current_location_target(e, this);
    if e.get(lt, CombatLocationTarget::cType) == 3 {
        let lt = current_location_target(e, this);
        let space = e.call(0x0084_e3a0, &args![lt.addr() + 8]).u32();
        if space != attacker_space {
            e.call(0x009a_0c50, &args![this, 1u32]);
        }
    }
    let lt = current_location_target(e, this);
    if e.get(lt, CombatLocationTarget::pRef) != 0 {
        e.with_stack(16, |e, location| {
            let lt = current_location_target(e, this);
            let reference = e.get(lt, CombatLocationTarget::pRef);
            let result = e
                .call(GET_WORLD_LOCATION, &args![reference, location])
                .u32();
            let lt = current_location_target(e, this);
            copy_words(e, lt.addr() + 8, result, 4);
        });
    }

    let distance = e.with_stack(12 + 16, |e, locals| {
        let (scale, location) = (locals.addr(), locals.addr() + 12);
        let quarter = constant_bits(e, QUARTER);
        e.call(
            0x0041_6870,
            &args![scale, 1.0f32.to_bits(), 1.0f32.to_bits(), quarter],
        );
        let mine = e.call(GET_WORLD_LOCATION, &args![attacker, location]).u32();
        let lt = current_location_target(e, this);
        e.call(0x0052_7ac0, &args![lt.addr() + 8, mine, scale])
            .f32()
    });

    let mut near_override = false;
    let lt = current_location_target(e, this);
    let kind = e.get(lt, CombatLocationTarget::cType);
    let lt = current_location_target(e, this);
    if (kind == 0 || e.get(lt, CombatLocationTarget::cType) == 1) && target != 0 {
        let controller = state_controller(e, this);
        if e.call(0x0098_1300, &args![controller]).bool() {
            let controller = state_controller(e, this);
            if !e.call(0x0067_a700, &args![controller]).bool() {
                near_override = true;
            }
        }
    }

    if near_override {
        let lt = current_location_target(e, this);
        e.set(lt, CombatLocationTarget::bNearTarget, 1);
    } else {
        let lt = current_location_target(e, this);
        let maximum = e.get(lt, CombatLocationTarget::fMaxDistance);
        let minimum = e.get(lt, CombatLocationTarget::fMinDistance);
        if minimum < maximum && e.call(0x009a_0a80, &args![this]).bool() {
            let radius = e.call(0x009a_0b70, &args![this]).f32();
            let lt = current_location_target(e, this);
            let squared = radius as f64 * radius as f64;
            if e.get(lt, CombatLocationTarget::bNearTarget) != 0 {
                if distance as f64 > squared {
                    let lt = current_location_target(e, this);
                    e.set(lt, CombatLocationTarget::bNearTarget, 0);
                }
            } else if (distance as f64) < squared {
                let lt = current_location_target(e, this);
                e.set(lt, CombatLocationTarget::bNearTarget, 1);
            }
        } else {
            let lt = current_location_target(e, this);
            e.set(lt, CombatLocationTarget::bNearTarget, 0);
            let lt = current_location_target(e, this);
            let minimum = e.get(lt, CombatLocationTarget::fMinDistance) as f64;
            if (distance as f64) < minimum * minimum {
                let lt = current_location_target(e, this);
                e.set(lt, CombatLocationTarget::bNearTarget, 1);
            }
        }
    }

    let lt = current_location_target(e, this);
    match e.get(lt, CombatLocationTarget::cType) {
        0 => {
            let lt = current_location_target(e, this);
            if e.get(lt, CombatLocationTarget::bNearTarget) != 0 && !near_override {
                e.call(0x009a_0a40, &args![this]);
                let controller = state_controller(e, this);
                e.call(0x0098_1380, &args![controller]);
            }
        }
        1 => {
            let controller = state_controller(e, this);
            if fn_00998a30(e, controller) == 0 {
                return;
            }
            let lt = current_location_target(e, this);
            if e.get(lt, CombatLocationTarget::pRef) == 0 {
                return;
            }
            let lt = current_location_target(e, this);
            let object = e.get(lt, CombatLocationTarget::pRef);
            if !e.vcall(object, 0x100, &args![]).bool() {
                return;
            }
            let first = setting_value_address(e, 0x011c_f45c);
            let second = setting_value_address(e, 0x011c_f45c);
            let near = e.mem.f32(first) as f64 * e.mem.f32(second) as f64;
            if (distance as f64) < near {
                let lt = current_location_target(e, this);
                let reference = e.get(lt, CombatLocationTarget::pRef);
                let theirs = e.call(0x0049_97b0, &args![reference]).u8();
                let mine = e.call(0x0049_97b0, &args![attacker]).u8();
                let guard_sneak_timer = member(this, 0x1f4);
                if theirs == mine {
                    timer_stop(e, guard_sneak_timer);
                } else if !timer_running(e, guard_sneak_timer) {
                    let delay = if reference == e.global::<u32>(PLAYER_CHARACTER) {
                        let low = constant_bits(e, HALF);
                        random_float_bits(e, low, 1.0f32.to_bits())
                    } else {
                        let high = constant_bits(e, HALF);
                        let low = constant_bits(e, TENTH);
                        random_float_bits(e, low, high)
                    };
                    timer_start(e, guard_sneak_timer, delay);
                } else if timer_expired(e, guard_sneak_timer) {
                    let controller = state_controller(e, this);
                    e.call(0x0098_1520, &args![controller, theirs as u32]);
                    timer_stop(e, guard_sneak_timer);
                }
            } else {
                let controller = state_controller(e, this);
                if fn_009989f0(e, controller) != 0 {
                    let controller = state_controller(e, this);
                    e.call(0x0098_1520, &args![controller, 0u32]);
                }
            }
        }
        _ => {}
    }
}

// Translated from 00998980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the time since `fLastTarget360LineOfSightTimeStamp` is below the
/// setting at `011cea98` (`FCOMP` of the setting against the elapsed time,
/// true only when the setting is greater).
pub fn fn_00998980(e: &mut Engine, this: Ptr<CombatState>) -> bool {
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
    let limit = setting_value(e, 0x011c_ea98);
    limit as f64 > elapsed
}

// Translated from 009989d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009928c0(this, 0x10)` on a `CombatPlan`-side object (the code passes
/// the result of `00980070` here); returns its `AL`.
pub fn fn_009989d0(e: &mut Engine, this: u32) -> bool {
    e.call(0x0099_28c0, &args![this, 0x10u32]).bool()
}

// Translated from 009989f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bForceSneak` (+0xc7).
pub fn fn_009989f0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xc7))
}

// Translated from 00998a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bUpdatedDetection` (+0xc5).
pub fn fn_00998a10(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xc5))
}

// Translated from 00998a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bAttackerCanSneak` (+0xd4).
pub fn fn_00998a30(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xd4))
}

// Translated from 00998a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00998a50 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The second half of the full update: picks another embedded weapon at
/// random when the embedded-weapon timer allows it (avoiding the current one
/// when the timer was cancelled), retries the blocked explosive projectile
/// through `00992720`, re-evaluates the better-weapon timer (`0099f6d0`),
/// the combat strength (`009a1010`) and threat ratio (`009a1060`), handles a
/// queued path check (`spPathCheckQueue`: its message, `009a1400`) or the
/// target-unreachable timer, updates the permissions (`00999260`), and
/// starts or ends the flee dialogue state.
pub fn fn_00998a50(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
    let count = e.call(ARRAY_SIZE, &args![embedded]).i32();
    let embedded_timer = member(this, 0x214);
    if count > 1 {
        let controller = state_controller(e, this);
        let switching = e.call(0x0098_1970, &args![controller]).u8();
        if !timer_running(e, embedded_timer) && switching == 0 {
            let delay = setting_bits(e, 0x011c_eb50);
            timer_start(e, embedded_timer, delay);
        }
        if (timer_running(e, embedded_timer) && timer_expired(e, embedded_timer))
            || e.get(this, CombatState::bInitializing) != 0
        {
            let cancelled = fn_009990c0(e, Ptr::new(embedded_timer));
            let chance = setting_bits(e, 0x011c_ec7c);
            if e.call(0x004d_ff00, &args![chance]).bool()
                || e.get(this, CombatState::bInitializing) != 0
                || cancelled
            {
                let mut chosen = 0;
                if cancelled && e.get(this, CombatState::pCurrentEmbeddedWeapon) != 0 {
                    let mut skip = e.call(0x0094_4460, &args![0u32, (count - 1) as u32]).i32();
                    let mut index = 0u32;
                    while index < e.call(ARRAY_SIZE, &args![embedded]).u32() {
                        let slot = e.call(0x006a_7ad0, &args![embedded, index]).u32();
                        let weapon = e.mem.u32(slot);
                        if weapon != e.get(this, CombatState::pCurrentEmbeddedWeapon) {
                            if skip == 0 {
                                chosen = weapon;
                                break;
                            }
                            skip -= 1;
                        }
                        index += 1;
                    }
                } else {
                    let index = e.call(0x0094_4460, &args![0u32, count as u32]).u32();
                    let slot = e.call(0x006a_7ad0, &args![embedded, index]).u32();
                    chosen = e.mem.u32(slot);
                }
                if chosen != e.get(this, CombatState::pCurrentEmbeddedWeapon) {
                    e.set(this, CombatState::pCurrentEmbeddedWeapon, chosen);
                    fn_009993c0(e, this);
                    let controller = state_controller(e, this);
                    e.call(0x0097_f200, &args![controller]);
                }
            }
            timer_stop(e, embedded_timer);
        }
    }

    let blocked_timer = member(this, 0x1dc);
    if e.get(this, CombatState::pExplosiveProjectileBlocked) != 0 {
        if !timer_running(e, blocked_timer) {
            let delay = setting_bits(e, 0x011c_f15c);
            timer_start(e, blocked_timer, delay);
        }
        if timer_expired(e, blocked_timer) {
            let mut clear = true;
            if target != 0 {
                let radius = constant_bits(e, TWO);
                let location = e.vcall(target, 0x1f4, &args![]).u32();
                let blocked = e.get(this, CombatState::pExplosiveProjectileBlocked);
                let manager: u32 = e.global(COMBAT_MANAGER);
                clear = e
                    .call(
                        0x0099_2720,
                        &args![manager, attacker, blocked, location, radius],
                    )
                    .bool();
            }
            if clear {
                e.set(this, CombatState::pExplosiveProjectileBlocked, 0);
            } else {
                let delay = setting_bits(e, 0x011c_f15c);
                timer_start(e, blocked_timer, delay);
            }
        }
    } else {
        timer_stop(e, blocked_timer);
    }

    let better_weapon_timer = member(this, 0x1d4);
    if timer_expired(e, better_weapon_timer) {
        e.call(0x0099_f6d0, &args![this]);
        let disarmed = e.get(this, CombatState::fDisarmedDPS);
        let best = e.get(this, CombatState::fMaxCombatWeaponDPS);
        if best > disarmed {
            let delay = setting_bits(e, 0x011c_f9f8);
            timer_start(e, better_weapon_timer, delay);
            e.set(this, CombatState::fDisarmedDPS, 0.0);
        } else {
            let delay = setting_bits(e, 0x011c_e9d8);
            timer_start(e, better_weapon_timer, delay);
        }
    }
    update_strength_and_threat_ratio(e, this);

    let queue = member(this, 0x224);
    let unreachable_timer = member(this, 0x1cc);
    if e.call(0x0055_9450, &args![queue]).u32() != 0 {
        e.with_stack(8, |e, message| {
            e.call(0x006e_9bd0, &args![message]);
            let queue_object = e.call(0x0055_9450, &args![queue]).u32();
            if e.vcall(queue_object, 0x10, &args![message]).bool() {
                if e.mem.u32(message.addr()) == 0 {
                    fn_00999100(e, this);
                    e.set(this, CombatState::iQueuedPathCheck, -1);
                } else if e.get(this, CombatState::iQueuedPathCheck) == 1
                    && e.call(0x0097_fd40, &args![this]).bool()
                    && target != 0
                {
                    if !e.call(0x009a_1400, &args![this, 0u32]).bool() {
                        e.set(this, CombatState::iQueuedPathCheck, -1);
                    }
                } else {
                    e.set(this, CombatState::iQueuedPathCheck, -1);
                }
                if e.get(this, CombatState::iQueuedPathCheck) == -1 {
                    e.call(0x0090_57d0, &args![queue, 0u32]);
                }
            }
            e.call(0x0080_0190, &args![message]);
        });
    } else if e.call(0x0097_fd40, &args![this]).bool() && target != 0 {
        if !timer_running(e, unreachable_timer) {
            let delay = constant_bits(e, FIVE);
            timer_start(e, unreachable_timer, delay);
        }
        if timer_expired(e, unreachable_timer) {
            let mut queued = false;
            if e.call(0x0046_1580, &args![this, 2u32]).bool()
                && e.call(0x009a_1400, &args![this, 1u32]).bool()
            {
                queued = true;
            }
            if !queued {
                e.call(0x009a_1400, &args![this, 0u32]);
            }
            let delay = constant_bits(e, FIVE);
            timer_start(e, unreachable_timer, delay);
        }
    } else {
        timer_stop(e, unreachable_timer);
    }

    fn_00999260(e, this);
    let controller = state_controller(e, this);
    if e.call(0x0098_1990, &args![controller]).bool() {
        let controller = state_controller(e, this);
        if !fn_00999120(e, controller) && e.get(this, CombatState::cFleeState) == 0 {
            e.set(this, CombatState::cFleeState, 1);
            e.call(0x008a_6840, &args![attacker, 0u32]);
            let controller = state_controller(e, this);
            let dialogue: u32 = e.global(COMBAT_DIALOGUE_MANAGER);
            e.call(0x0098_3970, &args![dialogue, controller, 2u32, 3u32, 1u32]);
        }
    } else {
        let mut clear = true;
        let controller = state_controller(e, this);
        if e.call(0x004f_9bf0, &args![controller]).u32() != 0 {
            let controller = state_controller(e, this);
            let item = e.call(0x004f_9bf0, &args![controller]).u32();
            if e.call(0x0082_5c00, &args![item]).u32() == 6
                && e.call(0x009a_1380, &args![this]).bool()
            {
                clear = false;
            }
        }
        if clear {
            e.set(this, CombatState::cFleeState, 0);
        }
    }
}

/// The combat-strength (`009a1010`, setting `011cf198`) and threat-ratio
/// (`CombatState::UpdateThreatRatio`, setting `011cf804`) timers, as
/// `00998a50` and `00999160` run them.
fn update_strength_and_threat_ratio(e: &mut Engine, this: Ptr<CombatState>) {
    let strength_timer = member(this, 0x204);
    if timer_expired(e, strength_timer) {
        e.call(0x009a_1010, &args![this]);
        let delay = setting_bits(e, 0x011c_f198);
        timer_start(e, strength_timer, delay);
    }
    let ratio_timer = member(this, 0x20c);
    if timer_expired(e, ratio_timer) {
        e.call(0x009a_1060, &args![this]);
        let delay = setting_bits(e, 0x011c_f804);
        timer_start(e, ratio_timer, delay);
    }
}

// Translated from 009990c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `CombatTimer`'s delay (+4) is below zero.
pub fn fn_009990c0(e: &mut Engine, this: Ptr<CombatTimer>) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    (e.get(this, CombatTimer::fDelay) as f64) < zero
}

// Translated from 00999100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `006296b0(this, 3)`.
pub fn fn_00999100(e: &mut Engine, this: Ptr<CombatState>) {
    e.call(0x0062_96b0, &args![this, 3u32]);
}

// Translated from 00999120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `006b9130` on the controller's `PlanTimer` (+0xb4) gives exactly
/// zero (`FCOMP` with the `double` 0 at `01012060`, unordered is false).
pub fn fn_00999120(e: &mut Engine, this: u32) -> bool {
    let value = e.call(0x006b_9130, &args![this.wrapping_add(0xb4)]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    value == zero
}

// Translated from 00999160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reduced update (the attacker is not in the high process): the target
/// detection through the group (`009887b0`), the inventory when it is dirty
/// (`009993c0`), and the combat-strength and threat-ratio timers.
pub fn fn_00999160(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    if target != 0 {
        let controller = state_controller(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        e.call(
            0x0098_87b0,
            &args![
                group,
                attacker,
                target,
                member(this, 0x6c),
                member(this, 0x94),
                member(this, 0x70),
                member(this, 0x71)
            ],
        );
    }
    if e.get(this, CombatState::bInventoryDirty) != 0 {
        fn_009993c0(e, this);
    }
    update_strength_and_threat_ratio(e, this);
}

// Translated from 00999260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates `iCombatPermissions` on its two timers: every 5 s the second byte
/// (bit 0x100 from a chance roll `004dff00` on the setting at `011ce954`),
/// every 10 s the low byte (bit 1 from a roll on the combat plan's value
/// `00495460` over 100, bit 2 when that value is above zero).
pub fn fn_00999260(e: &mut Engine, this: Ptr<CombatState>) {
    attacker(e, this);
    let short_timer = member(this, 0x1e4);
    if timer_expired(e, short_timer) {
        let permissions = e.get(this, CombatState::iCombatPermissions) & 0xffff_00ff;
        e.set(this, CombatState::iCombatPermissions, permissions);
        let chance = setting_bits(e, 0x011c_e954);
        if e.call(0x004d_ff00, &args![chance]).bool() {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 0x100;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let delay = constant_bits(e, FIVE);
        timer_start(e, short_timer, delay);
    }
    let long_timer = member(this, 0x1ec);
    if timer_expired(e, long_timer) {
        let permissions = e.get(this, CombatState::iCombatPermissions) & 0xffff_ff00;
        e.set(this, CombatState::iCombatPermissions, permissions);
        let controller = state_controller(e, this);
        let plan = e.call(0x0098_0070, &args![controller]).u32();
        let value = e.call(0x0049_5460, &args![plan]).f32();
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        let chance = (value as f64 / hundred) as f32;
        if e.call(0x004d_ff00, &args![x87_quiet_bits(chance.to_bits())])
            .bool()
        {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 1;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        if value as f64 > zero {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 2;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let delay = constant_bits(e, TEN);
        timer_start(e, long_timer, delay);
    }
}

/// Size of the stack frame `009993c0` keeps its locals in; a local at
/// `EBP - k` is at `frame + INVENTORY_FRAME - k`.
const INVENTORY_FRAME: u32 = 0x160;
/// `9999999.0` (`float`): the DPS `009993c0` gives a weapon the attacker
/// cannot unequip.
const FORCED_WEAPON_DPS: u32 = 0x0108_dfac;
/// `1000000.0` (`double`): the bonus `009993c0` gives the current embedded
/// weapon.
const CURRENT_WEAPON_BONUS_DOUBLE: u32 = 0x0106_b798;
/// `10000.0` (`double`): the divisor `009993c0` applies to out-of-range
/// weapons.
const OUT_OF_RANGE_DIVISOR_DOUBLE: u32 = 0x0102_f078;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\AI\Combat\CombatState.cpp"`.
const SOURCE_FILE_NAME: u32 = 0x0108_dfb0;

// Translated from 009993c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009993c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Re-evaluates the attacker's inventory for combat (inside a scope guard
/// `00404eb0`/`00404ee0`, category 0x2b, line 0x346 of the source file):
/// - clears the weapon arrays and DPS values;
/// - walks the inventory (`004d4830`): weapons the attacker can use
///   (`008bc9d0`) go into a local 0x14-byte-entry list with their health
///   over 100, form type 0x29 entries into a second list (`007cb2e0`), and
///   form type 0x2f items fill the two usable combat items (effect scores
///   for types 0x11 and 0x12);
/// - a weapon the attacker cannot unequip (`ExtraDataList::GetCanNotWear`
///   on the acquired object) becomes the only weapon, with DPS 9999999;
/// - otherwise every listed weapon gets its ammunition checked (queueing a
///   reload or an ammunition change where needed), its DPS (`00646060`;
///   weapons without DPS leave the list), its optimal range against the
///   target distance, then the best weapon per type, the maximum DPS
///   values, the embedded weapons (`0046e8c0`) and the hand-to-hand DPS;
/// - finally checks the projectile of the first weapon against the target
///   (`00992720`, else `00442a80`) and restarts the inventory timer.
pub fn fn_009993c0(e: &mut Engine, this: Ptr<CombatState>) {
    e.with_stack(INVENTORY_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + INVENTORY_FRAME - k;
        let scope = local(0x3c);
        e.call(
            0x0040_4eb0,
            &args![scope, 0x2bu32, 1u32, SOURCE_FILE_NAME, 0x346u32],
        );
        let attacker = attacker(e, this);
        let target = target(e, this);
        let controller = state_controller(e, this);
        let plan = e.call(0x0098_0070, &args![controller]).u32();
        let mut distance = 0.0f32;
        if target != 0 {
            let (mine, theirs) = (local(0x90), local(0x80));
            e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
            e.call(GET_WORLD_LOCATION, &args![target, theirs]);
            distance = e.call(0x0052_7a40, &args![mine, theirs]).f32();
        }
        fn_0099a2a0(e, this);
        e.set(this, CombatState::bInventoryDirty, 0);
        e.set(this, CombatState::fMaxCombatWeaponDPS, 0.0);
        e.set(this, CombatState::fMaxRangedWeaponDPS, 0.0);
        e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, 0.0);
        e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, 0.0);
        e.set(this, CombatState::iHasCombatWeapon, 0);
        e.set(this, CombatState::iHasCombatItem, 0);
        e.call(FILL_BYTES, &args![member(this, 0x3c), 0u32, 0x1cu32]);
        e.call(FILL_BYTES, &args![member(this, 0x1a4), 0u32, 8u32]);
        let item_scores = local(0x34);
        e.call(FILL_BYTES, &args![item_scores, 0u32, 8u32]);
        let weapons = local(0x68);
        e.call(0x009a_37e0, &args![weapons, 0x20u32, 0u32]);
        let others = local(0x54);
        e.call(0x009a_3750, &args![others, 0x20u32, 0u32]);
        let item = local(0x44);
        e.mem.set_u32(item, 0);
        let iterator = local(0x24);
        e.call(0x0052_7270, &args![iterator]);
        let changes = e.call(0x004b_f220, &args![attacker]).u32();

        while e
            .call(0x004d_4830, &args![changes, iterator, item, 0u32])
            .bool()
        {
            let entry = e.mem.u32(item);
            let object = e.call(ARRAY_SIZE, &args![entry]).u32();
            match e.call(0x0040_1170, &args![object]).u32() {
                0x28 => {
                    if e.call(0x008b_c9d0, &args![attacker, object, 1u32]).bool() {
                        let index = e.call(0x0078_81d0, &args![weapons]).u32();
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.call(FILL_BYTES, &args![slot, 0u32, 0x14u32]);
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.mem.set_u32(slot, object);
                        let entry = e.mem.u32(item);
                        let health = e.call(0x004b_cdb0, &args![entry, 1u32]).f64();
                        let hundred: f64 = e.global(HUNDRED_DOUBLE);
                        let health = health / hundred;
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.mem.set_f32(slot + 4, health as f32);
                    }
                }
                0x29 => {
                    let value = local(0x128);
                    e.mem.set_u32(value, object);
                    e.call(0x007c_b2e0, &args![others, value]);
                }
                0x2f => {
                    for slot in 0u32..2 {
                        let mut score = 0.0f32;
                        let kind = slot + 0x11;
                        if kind == 0x11 {
                            let effects = object + 0x3c;
                            score = e
                                .call(0x0040_5c00, &args![effects, 0u32, 0u32, 0x10u32])
                                .f32();
                            let more = e
                                .call(0x0040_5c00, &args![effects, 0x22u32, 0u32, 0x10u32])
                                .f64();
                            score = (more + score as f64) as f32;
                        } else if kind == 0x12
                            && !e.call(0x0099_28c0, &args![plan, 0x100u32]).bool()
                        {
                            score = e
                                .call(0x0040_5c00, &args![object + 0x3c, 0xcu32, 0u32, 0x4du32])
                                .f32();
                        }
                        let best = e.mem.f32(item_scores + 4 * slot);
                        if best < score {
                            e.mem.set_u32(member(this, 0x1a4) + 4 * slot, object);
                            e.mem.set_f32(item_scores + 4 * slot, x87_quiet(score));
                            let items = e.get(this, CombatState::iHasCombatItem);
                            e.set(
                                this,
                                CombatState::iHasCombatItem,
                                1u32.wrapping_shl(kind) | items,
                            );
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        let mut forced = 0;
        let process = e.call(0x008d_8520, &args![attacker]).u32();
        let acquired = e.vcall(process, 0x148, &args![]).u32();
        if acquired != 0 && e.call(0x0055_9450, &args![acquired]).u32() != 0 {
            let extra = e.call(0x0055_9450, &args![acquired]).u32();
            let list = e.call(0x0068_15c0, &args![extra]).u32();
            if e.mem.u32(list) != 0 {
                let extra = e.call(0x0055_9450, &args![acquired]).u32();
                let list = e.call(0x0068_15c0, &args![extra]).u32();
                let first = e.mem.u32(list);
                if e.call(0x0041_8b10, &args![first]).bool() {
                    forced = e.call(ARRAY_SIZE, &args![acquired]).u32();
                }
            }
        }

        if forced == 0 {
            rate_weapons(
                e,
                this,
                frame,
                WeaponContext {
                    attacker,
                    target,
                    plan,
                    changes,
                    acquired,
                    distance,
                },
            );
        } else {
            let kind = e.call(0x0052_2c80, &args![forced]).u32();
            let weapon_slot = member(this, 0xc).wrapping_add(4u32.wrapping_mul(kind));
            e.mem.set_u32(weapon_slot, forced);
            let dps_slot = member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind));
            let dps = constant_bits(e, FORCED_WEAPON_DPS);
            e.mem.set_u32(dps_slot, dps);
            let types = e.get(this, CombatState::iHasCombatWeapon);
            e.set(
                this,
                CombatState::iHasCombatWeapon,
                1u32.wrapping_shl(kind) | types,
            );
            let dps = x87_quiet(
                e.mem
                    .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
            );
            e.set(this, CombatState::fMaxCombatWeaponDPS, dps);
            let dps = x87_quiet(
                e.mem
                    .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
            );
            e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, dps);
            if kind == 1 || kind == 0 {
                let dps = x87_quiet(
                    e.mem
                        .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
                );
                e.set(this, CombatState::fMaxRangedWeaponDPS, dps);
            }
        }

        if target != 0 && e.get(this, CombatState::pCombatWeapons) != 0 {
            let weapon = e.get(this, CombatState::pCombatWeapons);
            let ammo = e.call(0x0052_5a90, &args![weapon, attacker]).u32();
            let projectile = e.call(0x004f_d3c0, &args![ammo]).u32();
            let radius = constant_bits(e, TWO);
            let location = e.vcall(target, 0x1f4, &args![]).u32();
            let manager: u32 = e.global(COMBAT_MANAGER);
            if !e
                .call(
                    0x0099_2720,
                    &args![manager, attacker, projectile, location, radius],
                )
                .bool()
            {
                e.call(0x0044_2a80, &args![this, projectile]);
            }
        }
        let delay = setting_bits(e, 0x011c_f660);
        timer_start(e, member(this, 0x21c), delay);
        e.call(0x0052_72b0, &args![iterator]);
        e.call(0x0096_a5f0, &args![others]);
        e.call(0x009a_3810, &args![weapons]);
        e.call(0x0040_4ee0, &args![scope]);
    });
}

/// What `009993c0` hands to its weapon rating part.
struct WeaponContext {
    attacker: u32,
    target: u32,
    plan: u32,
    changes: u32,
    acquired: u32,
    distance: f32,
}

/// The weapon rating part of `009993c0` (no forced weapon): `frame` is its
/// stack frame (see [`INVENTORY_FRAME`]); the local weapon list is at
/// `EBP-0x68`, the ammunition list at `EBP-0x54`. Each weapon entry is
/// 0x14 bytes: weapon, health (+4), DPS (+8), optimal range (+0xc), in-range
/// flag (+0x10).
fn rate_weapons(e: &mut Engine, this: Ptr<CombatState>, frame: Ptr, context: WeaponContext) {
    let WeaponContext {
        attacker,
        target,
        plan,
        changes,
        acquired,
        distance,
    } = context;
    let local = |k: u32| frame.addr() + INVENTORY_FRAME - k;
    let weapons = local(0x68);
    let others = local(0x54);
    let mut best_out_of_range = 0.0f32;
    let mut best_out_of_range_weapon = 0u32;
    let mut best_in_range = 0.0f32;
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![weapons]).u32() {
        let entry = e.call(0x006d_ad90, &args![weapons, index]).u32();
        let weapon = e.mem.u32(entry);
        let owner = if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
            attacker
        } else {
            0
        };
        let ammo = local(0xc8);
        let current_ammo = e.call(0x0052_5980, &args![weapon, owner]).u32();
        e.mem.set_u32(ammo, current_ammo);
        let has_ammo = current_ammo == 0 || e.call(0x0099_62f0, &args![others, ammo]).bool();
        if has_ammo {
            if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
                let process = e.call(0x008d_8520, &args![attacker]).u32();
                let object = e.vcall(process, 0x14c, &args![]).u32();
                if object != 0 {
                    let equipped = e.vcall(attacker, 0x1e4, &args![]).u32();
                    let acquired_weapon = e.call(ARRAY_SIZE, &args![acquired]).u32();
                    if equipped != 0 && acquired_weapon != 0 {
                        let needs_reload = object != 0
                            && e.call(0x0072_6070, &args![object]).i32() <= 0
                            && acquired_weapon != 0
                            && e.call(0x0052_4b60, &args![acquired_weapon]).u8() != 0;
                        let busy = e.call(0x008a_8870, &args![attacker]).bool()
                            || (equipped != 0 && e.call(0x0088_46c0, &args![equipped]).bool());
                        if needs_reload && !busy {
                            e.call(0x008a_8840, &args![attacker]);
                        }
                    }
                }
            }
        } else {
            let found = local(0xd1);
            e.mem.set_u8(found, 0);
            let other_ammo = local(0xd8);
            let replacement = e.call(0x004c_7300, &args![changes, weapon, found]).u32();
            e.mem.set_u32(other_ammo, replacement);
            if replacement == 0 || !e.call(0x0099_62f0, &args![others, other_ammo]).bool() {
                e.call(0x0098_f1f0, &args![weapons, index, 1u32]);
                // The code steps the index back and the loop forward again.
                continue;
            }
            if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
                let equipped = e.vcall(attacker, 0x1e4, &args![]).u32();
                let busy = e.call(0x008a_8870, &args![attacker]).bool()
                    || (equipped != 0 && e.call(0x0088_46c0, &args![equipped]).bool());
                if !busy {
                    let process = e.call(0x008d_8520, &args![attacker]).u32();
                    let replacement = e.mem.u32(other_ammo);
                    let count = e.call(0x004c_8f30, &args![changes, replacement]).u32();
                    let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                    let change = if block == 0 {
                        0
                    } else {
                        let replacement = e.mem.u32(other_ammo);
                        e.call(0x004b_c550, &args![block, replacement, count]).u32()
                    };
                    e.vcall(process, 0x168, &args![change]);
                    let object = e.vcall(process, 0x14c, &args![]).u32();
                    e.call(0x006e_cd40, &args![object, 0u32]);
                    e.call(0x008a_8840, &args![attacker]);
                }
            }
        }

        let health = x87_quiet_bits(e.mem.u32(entry + 4));
        let dps = e
            .call(
                0x0064_6060,
                &args![attacker, weapon, health, plan, 0u32, 0u32],
            )
            .f32();
        e.mem.set_f32(entry + 8, dps);
        let zero: f64 = e.global(ZERO_DOUBLE);
        let dps = e.mem.f32(entry + 8) as f64;
        if dps <= zero {
            e.call(0x0098_f1f0, &args![weapons, index, 1u32]);
            // The code steps the index back and the loop forward again.
            continue;
        }
        e.mem.set_u8(entry + 0x10, 1);
        e.mem.set_f32(entry + 0xc, 0.0);
        let kind = e.call(0x0052_2c80, &args![weapon]).u32();
        if target != 0 && (kind == 1 || kind == 0) {
            let (minimum, preferred, maximum) = (local(0x108), local(0x104), local(0x100));
            e.mem.set_f32(minimum, 0.0);
            e.mem.set_f32(preferred, 0.0);
            e.mem.set_f32(maximum, 0.0);
            let controller = state_controller(e, this);
            e.call(
                0x009a_9180,
                &args![controller, weapon, minimum, preferred, maximum],
            );
            let range = x87_quiet(e.mem.f32(maximum));
            e.mem.set_f32(entry + 0xc, range);
            let controller = state_controller(e, this);
            if e.call(0x0070_31c0, &args![controller]).u32() == weapon {
                let scale = setting_value_address(e, 0x011c_e4ac);
                let scaled = e.mem.f32(maximum) as f64 * e.mem.f32(scale) as f64;
                e.mem.set_f32(maximum, scaled as f32);
            }
            let range = e.mem.f32(maximum) as f64;
            let dps = e.mem.f32(entry + 8);
            if distance as f64 > range * range {
                e.mem.set_u8(entry + 0x10, 0);
                if best_out_of_range < dps {
                    best_out_of_range = x87_quiet(e.mem.f32(entry + 8));
                    best_out_of_range_weapon = weapon;
                }
            } else if best_in_range < dps {
                best_in_range = x87_quiet(e.mem.f32(entry + 8));
            }
        }
        index = index.wrapping_add(1);
    }

    let zero: f64 = e.global(ZERO_DOUBLE);
    if best_in_range as f64 > zero {
        best_out_of_range_weapon = 0;
    }

    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![weapons]).u32() {
        let entry = e.call(0x006d_ad90, &args![weapons, index]).u32();
        let weapon_local = local(0x114);
        let weapon = e.mem.u32(entry);
        e.mem.set_u32(weapon_local, weapon);
        let kind = e.call(0x0052_2c80, &args![weapon]).u32();
        let mut score = x87_quiet(e.mem.f32(entry + 8));
        if weapon == e.get(this, CombatState::pCurrentEmbeddedWeapon) {
            let bonus: f64 = e.global(CURRENT_WEAPON_BONUS_DOUBLE);
            score = (score as f64 + bonus) as f32;
        }
        if weapon == best_out_of_range_weapon {
            e.mem.set_u8(entry + 0x10, 1);
        }
        if e.mem.u8(entry + 0x10) == 0 {
            let divisor: f64 = e.global(OUT_OF_RANGE_DIVISOR_DOUBLE);
            score = (score as f64 / divisor) as f32;
            let range = e.mem.f32(entry + 0xc);
            if e.get(this, CombatState::fBetterWeaponOutOfRangeDistance) < range
                && best_in_range < e.mem.f32(entry + 8)
            {
                let range = x87_quiet(e.mem.f32(entry + 0xc));
                e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, range);
            }
        }
        if e.call(0x0046_e8c0, &args![weapon]).bool() && e.mem.u8(entry + 0x10) != 0 {
            let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
            e.call(0x007c_b2e0, &args![embedded, weapon_local]);
        }
        let dps = x87_quiet_bits(e.mem.u32(entry + 8));
        let best = x87_quiet_bits(e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits());
        let best = e.call(0x0040_4010, &args![best, dps]).f32();
        e.set(this, CombatState::fMaxCombatWeaponDPS, best);
        if kind == 1 || kind == 0 {
            let dps = x87_quiet_bits(e.mem.u32(entry + 8));
            let best = x87_quiet_bits(e.get(this, CombatState::fMaxRangedWeaponDPS).to_bits());
            let best = e.call(0x0040_4010, &args![best, dps]).f32();
            e.set(this, CombatState::fMaxRangedWeaponDPS, best);
        }
        let modified = x87_quiet_bits(score.to_bits());
        let best = x87_quiet_bits(
            e.get(this, CombatState::fMaxModifiedCombatWeaponDPS)
                .to_bits(),
        );
        let best = e.call(0x0040_4010, &args![best, modified]).f32();
        e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, best);
        let types = e.get(this, CombatState::iHasCombatWeapon);
        e.set(
            this,
            CombatState::iHasCombatWeapon,
            1u32.wrapping_shl(kind) | types,
        );
        let dps_slot = member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind));
        if e.mem.f32(dps_slot) < score {
            let weapon_slot = member(this, 0xc).wrapping_add(4u32.wrapping_mul(kind));
            e.mem.set_u32(weapon_slot, weapon);
            e.mem.set_f32(dps_slot, x87_quiet(score));
        }
        index = index.wrapping_add(1);
    }

    let unarmed = e
        .call(
            0x0064_6060,
            &args![attacker, 0u32, 1.0f32.to_bits(), plan, 0u32, 0u32],
        )
        .f32();
    e.set(this, CombatState::fUnarmedDPS, unarmed);
    let unarmed = x87_quiet_bits(e.get(this, CombatState::fUnarmedDPS).to_bits());
    let best = x87_quiet_bits(e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits());
    let best = e.call(0x0040_4010, &args![best, unarmed]).f32();
    e.set(this, CombatState::fMaxCombatWeaponDPS, best);
    let unarmed = x87_quiet_bits(e.get(this, CombatState::fUnarmedDPS).to_bits());
    let best = x87_quiet_bits(
        e.get(this, CombatState::fMaxModifiedCombatWeaponDPS)
            .to_bits(),
    );
    let best = e.call(0x0040_4010, &args![best, unarmed]).f32();
    e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, best);
    let types = e.get(this, CombatState::iHasCombatWeapon) | 0x40;
    e.set(this, CombatState::iHasCombatWeapon, types);
}

// Translated from 0099a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the six combat weapons and empties the embedded weapon array
/// (`008454f0(0)`).
pub fn fn_0099a2a0(e: &mut Engine, this: Ptr<CombatState>) {
    e.call(FILL_BYTES, &args![member(this, 0xc), 0u32, 0x18u32]);
    let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
    e.call(0x0084_54f0, &args![embedded, 0u32]);
}

// Translated from 0099a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Of the seven entries `0097ac90(i)` rates on `this`, returns the
/// `0097c150(i)` of the first highest one above zero (0 for entry 6).
pub fn fn_0099a2d0(e: &mut Engine, this: u32) -> u32 {
    let mut best = 0.0f32;
    let mut result = 0;
    for index in 0u32..7 {
        let value = e.call(0x0097_ac90, &args![this, index]).f32();
        if best < value {
            best = x87_quiet(value);
            result = if index == 6 {
                0
            } else {
                e.call(0x0097_c150, &args![this, index]).u32()
            };
        }
    }
    result
}

// Translated from 0099a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `reference` may be treated as reachable: always for another
/// reference than `pLastTarget`; for the last target, false only when the
/// detection level is above 0 and the world space (`TES::GetWorldSpace`
/// on the controller) has virtual slot 0x34 equal to 7 and `0099a3e0`
/// holds for it. The first argument word is not read.
pub fn fn_0099a360(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, reference: u32) -> bool {
    if reference != e.get(this, CombatState::pLastTarget) {
        return true;
    }
    let mut result = true;
    if e.get(this, CombatState::iTargetDetectionLevel) > 0 {
        let controller = state_controller(e, this);
        let space = e.call(0x004f_d3e0, &args![controller]).u32();
        if space != 0 && e.vcall(space, 0x34, &args![]).u32() == 7 && fn_0099a3e0(e, space) {
            result = false;
        }
    }
    result
}

// Translated from 0099a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at +0x10 of `this` is neither 3 nor 4.
pub fn fn_0099a3e0(e: &mut Engine, this: u32) -> bool {
    let value = e.mem.u32(this.wrapping_add(0x10));
    value != 3 && value != 4
}

/// The first parts of `0099a420` and `0099a530`: the combat items `0097cfc0`
/// returns for indices 0 to 4 that `keep` rejects are released
/// (`00981340`); then, unless `skip_pair` or the items 3 and 4 are still the
/// ones read at the start (`pCombatItems[3]`, `[4]`), both are released.
fn release_combat_items(
    e: &mut Engine,
    this: Ptr<CombatState>,
    skip_pair: bool,
    mut release: impl FnMut(&mut Engine, u32) -> bool,
) {
    let third = e.mem.u32(member(this, 0x19c));
    let fourth = e.mem.u32(member(this, 0x1a0));
    for index in 0u32..5 {
        let item = e.call(0x0097_cfc0, &args![this, index]).u32();
        if item != 0 && release(e, item) {
            e.call(0x0098_1340, &args![this, index]);
        }
    }
    if skip_pair {
        return;
    }
    if third == e.call(0x0097_cfc0, &args![this, 3u32]).u32()
        && fourth == e.call(0x0097_cfc0, &args![this, 4u32]).u32()
    {
        return;
    }
    e.call(0x0098_1340, &args![this, 3u32]);
    e.call(0x0098_1340, &args![this, 4u32]);
}

// Translated from 0099a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets `reference`: releases the combat items that hold it (and, unless
/// `keep_pair`, items 3 and 4 when they changed), clears the location
/// target, the combat location (when `BaseProcess::GetCurrentProcedureIndex`
/// on it gives the reference) and `pLastTarget` that refer to it.
pub fn fn_0099a420(e: &mut Engine, this: Ptr<CombatState>, reference: u32, keep_pair: u8) {
    release_combat_items(e, this, keep_pair != 0, |e, item| {
        e.mem.u32(item) == reference
    });
    let location_target = e.get(this, CombatState::pLocationTarget);
    if location_target != 0 {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if e.mem.u32(location_target + 4) == reference {
            e.call(0x009a_0a40, &args![this]);
        }
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        if e.call(0x0044_edb0, &args![location]).u32() == reference {
            e.call(0x009a_0680, &args![this]);
        }
    }
    if e.get(this, CombatState::pLastTarget) == reference {
        e.set(this, CombatState::pLastTarget, 0);
    }
}

// Translated from 0099a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the combat items whose reference `00437b90` rejects (and items 3
/// and 4 when they changed meanwhile).
pub fn fn_0099a530(e: &mut Engine, this: Ptr<CombatState>) {
    release_combat_items(e, this, false, |e, item| {
        let reference = e.mem.u32(item);
        reference != 0 && e.call(0x0043_7b90, &args![reference]).bool()
    });
}

// Translated from 0099a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records an attack now: `fLastAttackTimeStamp`, and
/// `fFiringStartedTimeStamp` when it was still unset (`-FLT_MAX`, read
/// through `006a7f50`). The argument word is not read.
pub fn fn_0099a5e0(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32) {
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastAttackTimeStamp, stamp);
    let firing = e.call(0x006a_7f50, &args![member(this, 0xa4)]).f64();
    let lowest = f32::from_bits(negative_max_bits(e)) as f64;
    if lowest == firing {
        let now = game_time_bits(e);
        let stamp = time_stamp_word(e, now);
        e.set(this, CombatState::fFiringStartedTimeStamp, stamp);
    }
}

// Translated from 0099a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless `skip`, `0097f220` on the embedded-weapon timer. The first
/// argument word is not read.
pub fn fn_0099a650(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, skip: u8) {
    if skip == 0 {
        e.call(0x0097_f220, &args![member(this, 0x214)]);
    }
}

// Translated from 0099a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fLastAttackTimeStamp` to now. The argument word is not read.
pub fn fn_0099a680(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32) {
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastAttackTimeStamp, stamp);
}

// Translated from 0099a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker was damaged: more than 1 s after the previous hit,
/// `009a0c50(1)`; `fLastDamagedByAttackerTimeStamp` becomes now; in flee
/// state 2, `CombatState::BoostConfidence(1)`. The two argument words are
/// not read.
pub fn fn_0099a6b0(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, _unused_2: u32) {
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xa0)]).f64();
    let one: f64 = e.global(ONE_DOUBLE);
    if elapsed > one {
        e.call(0x009a_0c50, &args![this, 1u32]);
    }
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastDamagedByAttackerTimeStamp, stamp);
    if e.get(this, CombatState::cFleeState) == 2 {
        e.call(0x009a_1280, &args![this, 1u32]);
    }
}

// Translated from 0099a720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker lost its weapon: with a better maximum DPS than the
/// disarmed one, remembers it as the disarmed DPS and starts the
/// better-weapon timer (setting `011ce058`); then re-evaluates the
/// inventory (`009993c0`).
pub fn fn_0099a720(e: &mut Engine, this: Ptr<CombatState>) {
    let disarmed = e.get(this, CombatState::fDisarmedDPS);
    let best = e.get(this, CombatState::fMaxCombatWeaponDPS);
    if best > disarmed {
        let best = x87_quiet(e.get(this, CombatState::fMaxCombatWeaponDPS));
        e.set(this, CombatState::fDisarmedDPS, best);
        let delay = setting_bits(e, 0x011c_e058);
        timer_start(e, member(this, 0x1d4), delay);
    }
    fn_009993c0(e, this);
}

// Translated from 0099a780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::PreActivateReference` (Xbox PDB): when combat item 2 holds
/// `reference`, the controller lets it go (`00981380`); then forgets the
/// reference (`0099a420`, keeping items 3 and 4).
pub fn combat_state_pre_activate_reference(e: &mut Engine, this: Ptr<CombatState>, reference: u32) {
    let item = e.call(0x0097_cfc0, &args![this, 2u32]).u32();
    if item != 0 && e.mem.u32(item) == reference {
        let controller = state_controller(e, this);
        e.call(0x0098_1380, &args![controller]);
    }
    fn_0099a420(e, this, reference, 1);
}

// Translated from 0099a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// While fleeing (`CombatController::IsFleeing`), an activated object whose
/// form (`BGSSaveFormBuffer::GetForm`, then `00401170`) has type 0x1c
/// becomes the last flee door (`0084e3a0` of `activated`), stamped now. The
/// last three argument words are not read.
pub fn fn_0099a7d0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    activated: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let controller = state_controller(e, this);
    if !e.call(0x0098_1990, &args![controller]).bool() {
        return;
    }
    let form = e.call(0x007a_f430, &args![activated]).u32();
    if e.call(0x0040_1170, &args![form]).u32() != 0x1c {
        return;
    }
    let door = e.call(0x0084_e3a0, &args![activated]).u32();
    e.set(this, CombatState::iLastFleeDoor, door);
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastFleeDoorTimeStamp, stamp);
}

// Translated from 0099a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A usable combat item was used: restarts its timer (in
/// `UsableCombatItemTimers`) with the delay of its kind (0x11: the setting
/// at `011cfdd8`, which the code then overwrites with the one at `011cf480`,
/// as for 0x12).
pub fn fn_0099a840(e: &mut Engine, this: Ptr<CombatState>, item: u32) {
    for index in 0u32..2 {
        if e.mem.u32(member(this, 0x1a4) + 4 * index) != item {
            continue;
        }
        let mut delay = 0.0f32.to_bits();
        let kind = index + 0x11;
        if kind == 0x11 {
            delay = setting_bits(e, 0x011c_fdd8);
        }
        if kind == 0x11 || kind == 0x12 {
            delay = setting_bits(e, 0x011c_f480);
        }
        timer_start(e, member(this, 0x1ac) + 8 * index, delay);
        return;
    }
}

// Translated from 0099a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker's cover was hit or the situation changed: with a cover
/// location and a target, unless the cover is still safe
/// (`CombatState::CheckCoverSafe`) and `0097fb20` holds, the cover is marked
/// (`005407b0(1)`); a combat location of type 0 or 1 is cleared
/// (`009a0680`); then `00999100` and the inventory update (`009993c0`).
pub fn fn_0099a8d0(e: &mut Engine, this: Ptr<CombatState>) {
    let target = target(e, this);
    if e.get(this, CombatState::pCoverLocation) != 0 && target != 0 {
        let safe = e.with_stack(16, |e, scratch| {
            let cover = e.get(this, CombatState::pCoverLocation);
            let point = e.call(0x005c_3420, &args![cover + 0x14, scratch]).u32();
            let cover = e.get(this, CombatState::pCoverLocation);
            e.call(0x0099_ccd0, &args![this, cover + 0x14, point, target, 0u32])
                .bool()
        });
        if !(safe && e.call(0x0097_fb20, &args![this]).bool()) {
            let cover = e.get(this, CombatState::pCoverLocation);
            e.call(0x0054_07b0, &args![cover, 1u32]);
        }
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        let kind = e.mem.u32(location + 0x14);
        if kind == 0 || e.mem.u32(location + 0x14) == 1 {
            e.call(0x009a_0680, &args![this]);
        }
    }
    fn_00999100(e, this);
    fn_009993c0(e, this);
}

// Translated from 0099a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears each cover location for which `005dc960` fails.
pub fn fn_0099a9a0(e: &mut Engine, this: Ptr<CombatState>) {
    let cover = e.get(this, CombatState::pCoverLocation);
    if cover != 0 && !e.call(0x005d_c960, &args![cover]).bool() {
        e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
    }
    let cover = e.get(this, CombatState::pSecondaryCoverLocation);
    if cover != 0 && !e.call(0x005d_c960, &args![cover]).bool() {
        e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
    }
}

// Translated from 0099aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A procedure of `kind` ended for `reference`: kind 15 also tells the group
/// (`0098bd70`); a combat location of that kind is cleared (`009a0680`).
pub fn fn_0099aa10(e: &mut Engine, this: Ptr<CombatState>, reference: u32, kind: u32) {
    if kind == 0xf {
        let controller = state_controller(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        e.call(0x0098_bd70, &args![group, reference]);
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        if e.call(0x0082_5c00, &args![location]).u32() == kind {
            e.call(0x009a_0680, &args![this]);
        }
    }
}

// Translated from 0099aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::PathingCoverLocation` (Xbox PDB), the copy
/// constructor: the `PathingLocation` part (`006dcc40`), the vtable
/// `0106cbc0` (RTTI `.?AVPathingCoverLocation@@`), the three edge vectors
/// and the seven bytes from +0x4c.
pub fn pathing_cover_location_pathing_cover_location(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(0x006d_cc40, &args![this, other]);
    let (to, from) = (this.addr(), other.addr());
    e.mem.set_u32(to, 0x0106_cbc0);
    copy_words(e, to + 0x28, from + 0x28, 3);
    copy_words(e, to + 0x34, from + 0x34, 3);
    copy_words(e, to + 0x40, from + 0x40, 3);
    for off in 0x4c..0x53 {
        let byte = e.mem.u8(from + off);
        e.mem.set_u8(to + off, byte);
    }
    this
}

// Translated from 0099ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a target seen in 360-degree view within 120 s (not later; an
/// unordered time counts as within), copies `LastSeenTargetLocation` to
/// `out` and returns true.
pub fn fn_0099ab50(e: &mut Engine, this: Ptr<CombatState>, out: Ptr<NiPoint3>) -> bool {
    if target(e, this) == 0 {
        return false;
    }
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
    let limit: f64 = e.global(HUNDRED_TWENTY_DOUBLE);
    if elapsed > limit {
        return false;
    }
    copy_words(e, out.addr(), member(this, 0x84), 3);
    true
}

// Translated from 0099abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::GetLastSeenTargetFiringLocation` (Xbox PDB): the last seen
/// location (`0099ab50`) raised by `CombatUtilities::GetTargetFiringZOffset
/// (target, 2, 1)`.
pub fn combat_state_get_last_seen_target_firing_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<NiPoint3>,
) -> bool {
    if !fn_0099ab50(e, this, out) {
        return false;
    }
    let target = target(e, this);
    let offset = e.call(0x009a_8460, &args![target, 2u32, 1u32]).f64();
    let z = e.get(out, NiPoint3::z);
    e.set(out, NiPoint3::z, (offset + z as f64) as f32);
    true
}

/// `128.0` (`float`): the dodge distances `0099ac20` passes.
const DODGE_DISTANCE: u32 = 0x0101_e704;
/// `"COMBAT: Invalid combat location type %i passed to
/// CombatState::FindCombatLocation()"`.
const INVALID_LOCATION_TYPE_MESSAGE: u32 = 0x0108_e000;
/// Stack frame of `0099ac20`; a local at `EBP - k` is at
/// `frame + FIND_LOCATION_FRAME - k`.
const FIND_LOCATION_FRAME: u32 = 0x80;

// Translated from 0099ac20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099ac20 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::FindCombatLocation` (Xbox PDB): looks for a combat location
/// of `kind` for the planner (`005e3fc0` on the controller). A location the
/// planner already has (`00980700`) is reused (`00979f70`); an unreachable
/// one (`0097bd00` gives `FLT_MAX`) fails at once. Otherwise a new
/// `CombatLocation` (0x20 bytes, `0097ee20`) and `CombatCoverLocation` (0x78
/// bytes, `006d3150`) are made, the current combat location of the same kind
/// is reused, or the kind's search runs (target reference or group location,
/// cover search `0099b8e0`, dodge, location target, flee, grenade arc, line
/// of sight, the flag kinds 14 and 15, combat items 17-21). A found location
/// gets the attacker's space if it has none, is checked for movement
/// (`CombatState::CheckMovement`), costed (`CombatAction::GetMovementCost`
/// when the kind's table entry says so) and handed to the planner
/// (`00995980`, `009959c0`); otherwise both objects are freed and the planner
/// is told `FLT_MAX`.
pub fn combat_state_find_combat_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    kind: u32,
) -> bool {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let controller = state_controller(e, this);
    let planner = e.call(0x005e_3fc0, &args![controller]).u32();
    if e.call(0x0098_0700, &args![planner, kind]).u32() != 0 {
        e.call(0x0097_9f70, &args![planner, kind]);
        return true;
    }
    let cost = e.call(0x0097_bd00, &args![planner, kind]).f64();
    let unreachable = f32::from_bits(x87_quiet_bits(e.global::<u32>(MAX_FLOAT))) as f64;
    if unreachable == cost {
        return false;
    }
    e.with_stack(FIND_LOCATION_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + FIND_LOCATION_FRAME - k;
        let mut found = false;
        let block = e.call(0x0040_1000, &args![0x20u32]).u32();
        let location = if block == 0 {
            Ptr::<CombatLocation>::NULL
        } else {
            fn_0097ee20(e, Ptr::new(block))
        };
        let block = e.call(0x0040_1000, &args![0x78u32]).u32();
        let cover = if block == 0 {
            0
        } else {
            e.call(0x006d_3150, &args![block]).u32()
        };
        e.call(0x0060_0ab0, &args![location, kind]);
        let radius = x87_quiet_bits(e.mem.u32(LOCATION_TYPE_TABLE + 4 + 8 * kind));
        e.call(0x0081_9a50, &args![location, radius]);

        if e.get(this, CombatState::pCombatLocation) != 0 {
            let current = e.get(this, CombatState::pCombatLocation);
            if e.call(0x0082_5c00, &args![current]).u32() == kind {
                let current = e.get(this, CombatState::pCombatLocation);
                fn_009973a0(e, location, Ptr::new(current));
                let current_cover = e.get(this, CombatState::pCoverLocation);
                if current_cover != 0 {
                    let current_cover = e.get(this, CombatState::pCoverLocation);
                    if e.mem.u32(current_cover + 4) == kind {
                        let current_cover = e.get(this, CombatState::pCoverLocation);
                        fn_00997480(e, Ptr::new(cover), Ptr::new(current_cover));
                    }
                }
                if e.mem.u8(LOCATION_TYPE_TABLE + 8 * kind) == 0 {
                    combat_location_update_ref_location(e, location);
                    found = true;
                }
            }
        }
        let near_flag = kind == 10 || kind == 0xc || kind == 0xf;

        if !found {
            found = find_location_of_kind(
                e,
                this,
                FindContext {
                    kind,
                    location,
                    cover,
                    attacker,
                    target,
                    group,
                    point: local(0x48),
                    world_location: local(0x3c),
                },
            );
        }

        if found {
            if fn_0099b580(e, location) == 0 {
                let space = e.call(GET_SPACE, &args![attacker]).u32();
                fn_0099b520(e, location, space);
            }
            let world = local(0x64);
            let source = e.call(0x0068_15c0, &args![location]).u32();
            copy_words(e, world, source, 4);
            let point = e.call(0x0068_15c0, &args![world]).u32();
            let at_origin = e.call(0x0043_90c0, &args![point, ZERO_POINT]).bool();
            if at_origin
                || e.call(0x009a_02a0, &args![this, world, 1u32, near_flag as u32])
                    .bool()
            {
                let mut movement_cost = 0.0f32;
                if e.mem.u8(LOCATION_TYPE_TABLE + 1 + 8 * kind) != 0 {
                    let mine = local(0x78);
                    e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
                    let controller = state_controller(e, this);
                    movement_cost = e.call(0x0097_acb0, &args![controller, mine, world]).f32();
                }
                let cost_bits = x87_quiet_bits(movement_cost.to_bits());
                e.call(0x0099_5980, &args![planner, kind, location, cost_bits]);
                if e.mem.u32(cover + 0x68) != 0 {
                    e.call(0x0099_59c0, &args![planner, kind, cover]);
                } else if cover != 0 {
                    e.call(0x006d_3370, &args![cover, 1u32]);
                }
                return true;
            }
        }
        e.call(0x0040_1030, &args![location]);
        if cover != 0 {
            e.call(0x006d_3370, &args![cover, 1u32]);
        }
        let unreachable = constant_bits(e, MAX_FLOAT);
        e.call(0x0099_5980, &args![planner, kind, 0u32, unreachable]);
        false
    })
}

/// What the kind switch of `0099ac20` works on: the new location and cover
/// location, the attacker, target and group, and two of its locals (a point
/// at `EBP-0x48`, a `BGSWorldLocation` at `EBP-0x3c`).
struct FindContext {
    kind: u32,
    location: Ptr<CombatLocation>,
    cover: u32,
    attacker: u32,
    target: u32,
    group: u32,
    point: u32,
    world_location: u32,
}

/// The kind switch of `0099ac20` (its jump table at `0099b4dc`, indices at
/// `0099b508`); returns whether a location was found.
fn find_location_of_kind(e: &mut Engine, this: Ptr<CombatState>, context: FindContext) -> bool {
    let FindContext {
        kind,
        location,
        cover,
        attacker,
        target,
        group,
        point,
        world_location,
    } = context;
    let search_info = this.at(CombatState::CoverSearchInfo).addr();
    match kind {
        0 | 1 => {
            if target == 0 {
                return false;
            }
            if e.call(0x0097_ee90, &args![this]).bool() {
                if !e.call(0x0097_fd40, &args![this]).bool() {
                    combat_location_set_ref(e, location, target);
                    return true;
                }
                false
            } else {
                e.call(0x0097_ede0, &args![world_location]);
                if e.call(0x0098_90e0, &args![group, target, world_location])
                    .bool()
                {
                    e.call(0x0097_ee60, &args![location, world_location]);
                    return true;
                }
                false
            }
        }
        2 | 4 | 5 => {
            let controller = state_controller(e, this);
            let weapon = e.call(0x0070_31c0, &args![controller]).u32();
            fn_0099b8e0(
                e,
                this,
                weapon,
                Ptr::new(search_info),
                cover,
                location.addr(),
                kind,
            )
        }
        3 => {
            let distance = constant_bits(e, DODGE_DISTANCE);
            let distance2 = constant_bits(e, DODGE_DISTANCE);
            e.call(
                0x0099_e190,
                &args![this, location, 3u32, 0xfu32, distance2, distance],
            )
            .bool()
        }
        6 => {
            let location_target = e.get(this, CombatState::pLocationTarget);
            if location_target != 0 && e.mem.u8(location_target) == 0 {
                let location_target = e.get(this, CombatState::pLocationTarget);
                let spot = e.call(0x0068_15c0, &args![location_target + 8]).u32();
                e.call(
                    0x0099_d030,
                    &args![
                        this,
                        search_info,
                        cover,
                        location,
                        kind,
                        spot,
                        0.0f32.to_bits(),
                        0u32
                    ],
                )
                .bool()
            } else if self::target(e, this) != 0 {
                let target = self::target(e, this);
                let source = e.vcall(target, 0x1f4, &args![]).u32();
                copy_words(e, point, source, 3);
                e.call(
                    0x0099_d030,
                    &args![
                        this,
                        search_info,
                        cover,
                        location,
                        kind,
                        point,
                        0.0f32.to_bits(),
                        0u32
                    ],
                )
                .bool()
            } else {
                false
            }
        }
        7..=9 | 0x10 => {
            if e.get(this, CombatState::pLocationTarget) == 0 {
                return false;
            }
            if kind == 8
                && e.call(0x0046_1580, &args![this, 1u32.wrapping_shl(kind)])
                    .bool()
            {
                return false;
            }
            let location_target = e.get(this, CombatState::pLocationTarget);
            if e.mem.u32(location_target + 4) != 0 {
                let location_target = e.get(this, CombatState::pLocationTarget);
                let reference = e.mem.u32(location_target + 4);
                combat_location_set_ref(e, location, reference);
                let location_target = e.get(this, CombatState::pLocationTarget);
                let object = e.mem.u32(location_target + 4);
                if e.vcall(object, 0x100, &args![]).bool() {
                    let location_target = e.get(this, CombatState::pLocationTarget);
                    let reference = e.mem.u32(location_target + 4);
                    let theirs = e.call(0x008b_e1b0, &args![reference]).f64();
                    let mine = e.call(0x008b_e1b0, &args![attacker]).f64();
                    let sum = (mine + theirs) as f32;
                    e.call(0x0081_9a50, &args![location, x87_quiet_bits(sum.to_bits())]);
                }
            } else {
                let location_target = e.get(this, CombatState::pLocationTarget);
                e.call(0x0097_ee60, &args![location, location_target + 8]);
            }
            let location_target = e.get(this, CombatState::pLocationTarget);
            let radius = x87_quiet_bits(e.mem.u32(location_target + 0x18));
            e.call(0x0081_9a50, &args![location, radius]);
            true
        }
        10 | 0xc => e
            .call(0x0099_ebe0, &args![this, cover, location, kind])
            .bool(),
        0xb => {
            let weapon = e.mem.u32(member(this, 0x18));
            fn_0099b5a0(e, this, location, member(this, 0x76), weapon)
        }
        0xd => e.call(0x0099_f480, &args![this, location, kind]).bool(),
        0xe => {
            fn_0099b540(e, location, 1, 1);
            true
        }
        0xf => {
            let spot = e.call(0x0068_15c0, &args![location]).u32();
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            let search = e
                .call(0x0098_bc00, &args![group, attacker, 0x32u32, spot])
                .u32();
            let mut found = false;
            if search != 0 {
                e.call(0x0097_ee60, &args![location, search]);
                found = true;
            } else {
                let own = e.call(0x0097_b940, &args![location]).u32();
                if e.call(0x0043_9090, &args![own, ZERO_POINT]).bool() {
                    found = true;
                }
            }
            if found {
                fn_0099b540(e, location, 2, 1);
            }
            found
        }
        0x11..=0x15 => {
            let item = e.call(0x0097_cfc0, &args![this, kind - 0x11]).u32();
            if item == 0 || e.mem.u32(item) == 0 {
                return false;
            }
            let reference = e.mem.u32(item);
            combat_location_set_ref(e, location, reference);
            let reference = e.mem.u32(item);
            let size = e.call(0x0057_1600, &args![reference]).f64();
            let reach = e.call(0x008b_e420, &args![attacker]).f64();
            let sum = reach + size;
            let extra = setting_value(e, 0x011c_f914);
            let radius = (extra as f64 + sum) as f32;
            e.call(
                0x0081_9a50,
                &args![location, x87_quiet_bits(radius.to_bits())],
            );
            fn_0099b540(e, location, 2, 1);
            true
        }
        _ => {
            e.call(0x005b_5e40, &args![INVALID_LOCATION_TYPE_MESSAGE, kind]);
            false
        }
    }
}

// Translated from 0099b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the location's space through `0041fd00` (the map's name
/// `NonActorMagicCaster::SetCurrentSpell` is folded code).
pub fn fn_0099b520(e: &mut Engine, this: Ptr<CombatLocation>, space: u32) {
    e.call(0x0041_fd00, &args![this, space]);
}

// Translated from 0099b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears the `flags` bits of `CombatLocation::iFlags`.
pub fn fn_0099b540(e: &mut Engine, this: Ptr<CombatLocation>, flags: u32, set: u8) {
    let current = e.get(this, CombatLocation::iFlags);
    if set != 0 {
        e.set(this, CombatLocation::iFlags, current | flags);
    } else {
        e.set(this, CombatLocation::iFlags, !flags & current);
    }
}

// Translated from 0099b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The location's space (`0084e3a0`: the word at +0xc).
pub fn fn_0099b580(e: &mut Engine, this: Ptr<CombatLocation>) -> u32 {
    e.call(0x0084_e3a0, &args![this]).u32()
}

/// Stack frame of `0099b5a0`; a local at `EBP - k` is at
/// `frame + GRENADE_FRAME - k`.
const GRENADE_FRAME: u32 = 0x68;

// Translated from 0099b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks for a spot to throw `weapon`'s projectile at: with a projectile
/// (`00525a90` on the weapon) and a target, tries first the target's group
/// location in the attacker's space, then the last seen location (seen
/// within 10 s); a spot qualifies when it is within the projectile's range
/// (`009a7f00`) but not closer than 256 units (`65536` squared), the combat
/// manager accepts the projectile there (`00992720`), the arc reaches it
/// (`CombatUtilities::CheckProjectileArc`, arc flag in `high_arc`) and, for a
/// projectile with an explosion, the manager also accepts it at the flight
/// time (`CombatUtilities::ComputeProjectileTime`). The spot becomes `out`
/// (`0099b8b0`).
pub fn fn_0099b5a0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<CombatLocation>,
    high_arc: u32,
    weapon: u32,
) -> bool {
    if weapon == 0 {
        return false;
    }
    let attacker = attacker(e, this);
    if e.call(0x0052_5a90, &args![weapon, attacker]).u32() == 0 {
        return false;
    }
    let attacker = self::attacker(e, this);
    let target = target(e, this);
    if target == 0 {
        return false;
    }
    e.with_stack(GRENADE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + GRENADE_FRAME - k;
        let projectile = e.call(0x0052_5a90, &args![weapon, attacker]).u32();
        let range = e
            .call(0x009a_7f00, &args![projectile, 0.0f32.to_bits()])
            .f32();
        let origin = local(0x18);
        e.call(0x009a_8050, &args![origin, attacker]);
        let spot = local(0x48);
        for attempt in 0u32..2 {
            e.call(0x0068_15c0, &args![spot]);
            if attempt == 0 {
                let world = local(0x5c);
                e.call(0x0097_ede0, &args![world]);
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                if !e.call(0x0098_90e0, &args![group, target, world]).bool() {
                    continue;
                }
                let space = e.call(0x0084_e3a0, &args![world]).u32();
                if space != e.call(GET_SPACE, &args![attacker]).u32() {
                    continue;
                }
                let source = e.call(0x0068_15c0, &args![world]).u32();
                copy_words(e, spot, source, 3);
            } else {
                let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
                let limit: f64 = e.global(TEN_DOUBLE);
                if elapsed > limit {
                    continue;
                }
                copy_words(e, spot, member(this, 0x84), 3);
            }
            let offset = e.call(0x009a_8460, &args![target, 2u32, 0u32]).f64();
            let z = e.mem.f32(spot + 8);
            e.mem.set_f32(spot + 8, (offset + z as f64) as f32);
            let difference = local(0x2c);
            e.call(0x0043_9ef0, &args![spot, difference, origin]);
            let distance = e.call(0x0059_5c80, &args![difference]).f32();
            if distance as f64 > range as f64 * range as f64 {
                continue;
            }
            let closest: f64 = e.global(SIXTY_FIVE_THOUSAND_DOUBLE);
            if (distance as f64) < closest {
                continue;
            }
            let explosion = e.call(0x004f_d3c0, &args![projectile]).u32();
            let manager: u32 = e.global(COMBAT_MANAGER);
            if explosion != 0
                && !e
                    .call(
                        0x0099_2720,
                        &args![manager, attacker, explosion, spot, 0.0f32.to_bits()],
                    )
                    .bool()
            {
                continue;
            }
            let landing = local(0x3c);
            e.call(0x0068_15c0, &args![landing]);
            let minus_one = constant_bits(e, MINUS_ONE);
            let (ox, oy, oz) = (
                e.mem.u32(origin),
                e.mem.u32(origin + 4),
                e.mem.u32(origin + 8),
            );
            let (sx, sy, sz) = (e.mem.u32(spot), e.mem.u32(spot + 4), e.mem.u32(spot + 8));
            let (zx, zy, zz) = (
                e.global::<u32>(ZERO_POINT),
                e.global::<u32>(ZERO_POINT + 4),
                e.global::<u32>(ZERO_POINT + 8),
            );
            let reaches = e
                .call(
                    0x009a_7670,
                    &args![
                        attacker, target, projectile, ox, oy, oz, sx, sy, sz, zx, zy, zz, high_arc,
                        landing, minus_one
                    ],
                )
                .bool();
            if !reaches {
                continue;
            }
            if explosion != 0 {
                let arc = e.mem.u8(high_arc) as u32;
                let (ox, oy, oz) = (
                    e.mem.u32(origin),
                    e.mem.u32(origin + 4),
                    e.mem.u32(origin + 8),
                );
                let (sx, sy, sz) = (e.mem.u32(spot), e.mem.u32(spot + 4), e.mem.u32(spot + 8));
                let time = e
                    .call(
                        0x009a_7e00,
                        &args![projectile, ox, oy, oz, sx, sy, sz, arc, 0.0f32.to_bits()],
                    )
                    .f32();
                let manager: u32 = e.global(COMBAT_MANAGER);
                if !e
                    .call(
                        0x0099_2720,
                        &args![
                            manager,
                            attacker,
                            explosion,
                            spot,
                            x87_quiet_bits(time.to_bits())
                        ],
                    )
                    .bool()
                {
                    continue;
                }
            }
            fn_0099b8b0(e, out, spot, 0);
            return true;
        }
        false
    })
}

// Translated from 0099b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the location's point and space (`BGSWorldLocation::SetLocation`,
/// `00989010`) and drops its reference.
pub fn fn_0099b8b0(e: &mut Engine, this: Ptr<CombatLocation>, point: u32, space: u32) {
    e.call(0x0098_9010, &args![this, point, space]);
    e.set(this, CombatLocation::pRef, 0);
}

/// `512.0` (`float`).
const FIVE_HUNDRED_TWELVE: u32 = 0x0102_31a0;
/// `2048.0` (`float`).
const TWO_THOUSAND_FORTY_EIGHT: u32 = 0x0101_8bfc;
/// `10000.0` (`float`).
const TEN_THOUSAND: u32 = 0x0102_2958;
/// `4096.0` (`float`).
const FOUR_THOUSAND_NINETY_SIX: u32 = 0x0101_7a3c;
/// `2.0` (`double`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `1000.0` (`double`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
/// `3.0` (`double`).
const THREE_DOUBLE: u32 = 0x0102_1928;
/// `-1.0` (`double`).
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `256.0` (`float`).
const TWO_HUNDRED_FIFTY_SIX: u32 = 0x0103_2adc;
/// The object `00483710` is called on around the cover search (a byte at
/// `011f1a61`).
const COVER_SEARCH_TOGGLE: u32 = 0x011f_1a61;
/// The object at `011f6394` whose +0x14 word (`00825c00`) the cover search
/// compares cover time stamps with, in milliseconds.
const TIMER_OBJECT_011F6394: u32 = 0x011f_6394;
/// Stack frame of `0099b8e0`; a local at `EBP - k` is at
/// `frame + COVER_SEARCH_FRAME - k`.
const COVER_SEARCH_FRAME: u32 = 0x1e0;

// Translated from 0099b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099b8e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Searches cover for location `kind` (2, 4, 5, 10 or 12) against the
/// target (inside `00483710` on `011f1a61`): the weapon's optimal ranges
/// (`CombatUtilities::ComputeOptimalWeaponRanges`, scaled by the setting at
/// `011ce3f0` and limited by `0040ebd0`) and the kind's parameters give the
/// search; the current cover location is kept when it still passes
/// `0099c230`; otherwise a `PathingRequestCover` (built from the attacker's
/// or the location target's position, the search centre and the kind's
/// flags) fills `search_info`'s cover locations (`006d62e0`) and the first one
/// that passes `0099c230` (skipping ones used in the last 3 s) becomes the
/// cover location. Returns whether a location was found.
pub fn fn_0099b8e0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    weapon: u32,
    search_info: Ptr<CombatCoverSearchInfo>,
    cover: u32,
    location: u32,
    kind: u32,
) -> bool {
    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
    let search_radius = x87_quiet(e.global::<f32>(FIVE_HUNDRED_TWELVE));
    let mut distance = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
    let attacker = attacker(e, this);
    let target = target(e, this);
    if target == 0 {
        return false;
    }
    e.with_stack(COVER_SEARCH_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_SEARCH_FRAME - k;
        let (minimum, range, maximum) = (local(0x30), local(0x3c), local(0x70));
        e.mem.set_f32(minimum, 0.0);
        let mut half_base = x87_quiet(e.global::<f32>(FIVE_HUNDRED_TWELVE));
        let value = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
        e.mem.set_f32(range, value);
        let value = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
        e.mem.set_f32(maximum, value);
        let controller = state_controller(e, this);
        e.call(
            0x009a_9180,
            &args![controller, weapon, minimum, range, maximum],
        );
        let scale = setting_value_address(e, 0x011c_e3f0);
        let scaled = (e.mem.f32(range) as f64 * e.mem.f32(scale) as f64) as f32;
        e.mem.set_f32(range, scaled);
        let upper = x87_quiet_bits(e.mem.u32(maximum));
        let lower = x87_quiet_bits(e.mem.u32(range));
        let limited = e.call(0x0040_ebd0, &args![lower, upper]).f32();
        e.mem.set_f32(range, limited);
        let mut flag_006e4380 = 0u8;
        let mut flag_006e43a0 = 0u8;
        let mut flag_006e43c0 = 0u8;
        let mut flag_006e43e0 = 0u8;
        match kind {
            2 => {
                let controller = state_controller(e, this);
                let plan = e.call(0x0098_0070, &args![controller]).u32();
                distance = e.call(0x004a_7bd0, &args![plan]).f32();
            }
            4 => {
                flag_006e43e0 = 1;
                flag_006e4380 = 1;
            }
            5 => {
                distance = x87_quiet(e.global::<f32>(FOUR_THOUSAND_NINETY_SIX));
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43e0 = 1;
                flag_006e43a0 = 1;
                flag_006e43c0 = 1;
            }
            10 => {
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43e0 = 1;
            }
            12 => {
                half_base = x87_quiet(setting_value(e, 0x011c_ed34));
                distance = x87_quiet(setting_value(e, 0x011c_f8d4));
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43a0 = 1;
                flag_006e43c0 = 1;
                flag_006e43e0 = 1;
            }
            _ => {}
        }

        let state = local(0x168);
        e.mem.set_u32(state, 0);
        let found_distance = local(0x38);
        e.mem.set_f32(found_distance, 0.0);
        if e.mem.u32(cover + 0x68) != 0 {
            let key = e.mem.u32(cover + 0x68);
            e.set(
                search_info,
                CombatCoverSearchInfo::iPreviousCoverLocationKey,
                key,
            );
            e.set(search_info, CombatCoverSearchInfo::ePreviousCoverState, 0);
            if kind == e.mem.u32(cover + 4) {
                let two: f64 = e.global(TWO_DOUBLE);
                let half = (half_base as f64 / two) as f32;
                let key = e.mem.u32(cover + 0x68);
                let keeps = fn_0099c230(
                    e,
                    this,
                    CoverCheck {
                        cover_location: cover + 0x14,
                        out: location,
                        reservation: cover + 0x70,
                        search_point: cover + 8,
                        key,
                        current_key: key,
                        kind,
                        attacker,
                        target,
                        minimum_distance: x87_quiet(half),
                        maximum_distance: x87_quiet(e.mem.f32(range)),
                        state,
                        distance: found_distance,
                        flag_006e43e0,
                    },
                );
                if keeps {
                    e.call(0x0054_07b0, &args![cover, 0u32]);
                    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
                    return true;
                }
                let previous = e.mem.u32(state);
                e.set(
                    search_info,
                    CombatCoverSearchInfo::ePreviousCoverState,
                    previous,
                );
            } else {
                e.set(search_info, CombatCoverSearchInfo::ePreviousCoverState, 0xf);
            }
        } else {
            e.set(
                search_info,
                CombatCoverSearchInfo::iPreviousCoverLocationKey,
                0,
            );
        }

        let origin = local(0x18);
        e.call(0x0068_15c0, &args![origin]);
        let location_target = e.get(this, CombatState::pLocationTarget);
        let near_location_target = location_target != 0 && {
            let target_kind = e.mem.u8(e.get(this, CombatState::pLocationTarget));
            target_kind == 1 || e.mem.u8(e.get(this, CombatState::pLocationTarget)) == 2
        };
        if near_location_target {
            let location_target = e.get(this, CombatState::pLocationTarget);
            let source = e.call(0x0068_15c0, &args![location_target + 8]).u32();
            copy_words(e, origin, source, 3);
            distance = e.call(0x009a_0b00, &args![this]).f32();
        } else {
            let source = e.vcall(attacker, 0x1f4, &args![]).u32();
            copy_words(e, origin, source, 3);
        }

        let request = local(0x164);
        e.call(0x006e_4080, &args![request]);
        let pathing_location = local(0x6c);
        e.call(0x006d_ce10, &args![pathing_location, origin, attacker]);
        e.call(0x006d_3ac0, &args![request, pathing_location]);
        let info = search_info.addr();
        if kind == 5 {
            let (group_point, member_point) = (local(0x198), local(0x17c));
            e.call(0x0068_15c0, &args![group_point]);
            e.call(0x0068_15c0, &args![member_point]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            e.call(0x0098_9eb0, &args![group, group_point]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            e.call(0x0098_9e20, &args![group, member_point]);
            copy_words(e, info, group_point, 3);
            let direction = local(0x18c);
            e.call(0x0043_9ef0, &args![member_point, direction, group_point]);
            e.call(0x006e_4400, &args![request, direction]);
        } else {
            let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
            let limit = setting_value(e, 0x011c_f6cc);
            let seen = limit as f64 > elapsed;
            e.set(
                search_info,
                CombatCoverSearchInfo::bUsedLastSeenLocation,
                seen as u8,
            );
            if e.get(search_info, CombatCoverSearchInfo::bUsedLastSeenLocation) != 0 {
                copy_words(e, info, member(this, 0x84), 3);
            } else {
                let source = e.vcall(target, 0x1f4, &args![]).u32();
                copy_words(e, info, source, 3);
            }
        }
        e.call(0x006d_ad40, &args![request, info]);
        e.call(
            0x006e_4340,
            &args![request, x87_quiet_bits(search_radius.to_bits())],
        );
        e.call(
            0x006e_4320,
            &args![request, x87_quiet_bits(distance.to_bits())],
        );
        e.call(0x0040_30d0, &args![request, 200u32]);
        e.call(0x006e_4380, &args![request, flag_006e4380 as u32]);
        e.call(0x006e_43a0, &args![request, flag_006e43a0 as u32]);
        e.call(0x006e_43c0, &args![request, flag_006e43c0 as u32]);
        let controller = state_controller(e, this);
        let height = e.call(0x0045_36e0, &args![controller]).f32();
        e.call(0x0051_b840, &args![request, height.to_bits()]);
        let controller = state_controller(e, this);
        let sneak_height = e.call(0x0045_3700, &args![controller]).f32();
        e.call(0x0050_76c0, &args![request, sneak_height.to_bits()]);
        let controller = state_controller(e, this);
        let can_sneak = fn_00998a30(e, controller);
        e.call(0x008e_eaa0, &args![request, can_sneak as u32]);
        e.call(0x006e_43e0, &args![request, flag_006e43e0 as u32]);
        e.mem.set_u32(cover + 4, kind);
        copy_words(e, cover + 8, info, 3);
        let now = game_time_bits(e);
        let stamp = time_stamp_word(e, now);
        e.set(search_info, CombatCoverSearchInfo::fTimeStamp, stamp);
        copy_words(e, info + 0xc, origin, 3);
        e.set(
            search_info,
            CombatCoverSearchInfo::fRadius,
            x87_quiet(distance),
        );
        let locations = info + 0x20;
        e.call(0x006d_bde0, &args![locations, 0u32]);
        if e.call(0x006d_62e0, &args![request, locations]).u32() != 0 {
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            fn_0099c210(e, cover_map);
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![locations]).u32() {
                let entry = e.call(0x0090_52c0, &args![locations, index]).u32();
                let entry_state = local(0x1a4);
                e.mem.set_u32(entry_state, 0);
                let entry_distance = local(0x1a8);
                e.mem.set_f32(entry_distance, 0.0);
                let key = e.call(0x006e_4570, &args![entry]).u32();
                if key == e.mem.u32(cover + 0x68) {
                    let previous = e.mem.u8(state);
                    e.mem.set_u8(entry + 0x52, previous);
                    index += 1;
                    continue;
                }
                let moved = e.call(0x006e_4740, &args![entry]).u32();
                if moved != 0 {
                    let now = e.call(0x0082_5c00, &args![TIMER_OBJECT_011F6394]).u32();
                    let seconds = now.wrapping_sub(moved) as f64 / e.global::<f64>(THOUSAND_DOUBLE);
                    if seconds < e.global::<f64>(THREE_DOUBLE) {
                        e.mem.set_u8(entry + 0x52, 1);
                        index += 1;
                        continue;
                    }
                }
                let reservation = local(0x1b4);
                e.call(0x006d_3280, &args![reservation]);
                let passes = fn_0099c230(
                    e,
                    this,
                    CoverCheck {
                        cover_location: entry,
                        out: location,
                        reservation,
                        search_point: cover + 8,
                        key,
                        current_key: 0,
                        kind,
                        attacker,
                        target,
                        minimum_distance: x87_quiet(half_base),
                        maximum_distance: x87_quiet(e.mem.f32(range)),
                        state: entry_state,
                        distance: entry_distance,
                        flag_006e43e0,
                    },
                );
                if passes {
                    e.mem.set_u8(entry + 0x52, 0x13);
                    e.call(0x006e_4430, &args![cover + 0x14, entry]);
                    e.mem.set_u32(cover + 0x68, key);
                    e.mem.set_u32(cover + 0x6c, moved);
                    copy_words(e, cover + 0x70, reservation, 1);
                    let half = e.mem.u16(reservation + 4);
                    e.mem.set_u16(cover + 0x74, half);
                    let height = fn_0099c1f0(e, cover + 0x14);
                    e.mem.set_u8(cover + 2, height);
                    e.call(0x0054_07b0, &args![cover, 0u32]);
                    timer_stop(e, member(this, 0x1fc));
                    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
                    e.call(0x004f_f7e0, &args![pathing_location]);
                    e.call(0x006d_ad70, &args![request]);
                    return true;
                }
                let failed = e.mem.u8(entry_state);
                e.mem.set_u8(entry + 0x52, failed);
                index += 1;
            }
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            e.call(0x0099_5890, &args![cover_map]);
        }
        e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
        e.call(0x004f_f7e0, &args![pathing_location]);
        e.call(0x006d_ad70, &args![request]);
        false
    })
}

// Translated from 0099c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::cCoverHeight` (+0x4c).
pub fn fn_0099c1f0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0x4c))
}

// Translated from 0099c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReadWriteLock::LockForWrite` on the lock at +0x14.
pub fn fn_0099c210(e: &mut Engine, this: u32) {
    e.call(0x00aa_6060, &args![this.wrapping_add(0x14)]);
}

/// The fourteen argument words of `0099c230`, in order.
pub struct CoverCheck {
    /// The `PathingCoverLocation` to check.
    pub cover_location: u32,
    /// The `CombatLocation` set on success (`0099b8b0`).
    pub out: u32,
    /// The `CoverReservationInfo` the cover map fills (`009835d0`).
    pub reservation: u32,
    /// The point the cover is searched from.
    pub search_point: u32,
    /// The cover location's key.
    pub key: u32,
    /// The current cover location's key (equal to `key` when re-checking
    /// it, 0 otherwise).
    pub current_key: u32,
    /// The combat location kind.
    pub kind: u32,
    pub attacker: u32,
    pub target: u32,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    /// Out: the failure reason.
    pub state: u32,
    /// Out: the squared distance from the search point.
    pub distance: u32,
    /// The flag the search also passes to `006e43e0`.
    pub flag_006e43e0: u8,
}

/// Stack frame of `0099c230`; a local at `EBP - k` is at
/// `frame + COVER_CHECK_FRAME - k`.
const COVER_CHECK_FRAME: u32 = 0x110;

// Translated from 0099c230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099c230 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Checks one cover location (one or two passes, one per open edge, for an
/// edge cover outside a long-range search): failure reasons go to `state`
/// (2 closed edges, 13 blocked from the target's position, 3 a cover the
/// state rejects `009a0d70`, 9 reserved `009835d0`, 6 ledge angle, 4/5
/// invalid locations, 16 movement `009a0260`, 7 too far, 8 too close, 10 a
/// target near it, 11/12 threatened `009a5670`, 13 blocked `0099cc60` /
/// `0099ca00`, 14 no line of sight); the highest reason of the last two
/// passes is kept. On success the location is stored into `out`
/// (`0099b8b0`).
pub fn fn_0099c230(e: &mut Engine, this: Ptr<CombatState>, check: CoverCheck) -> bool {
    let CoverCheck {
        cover_location,
        out,
        reservation,
        search_point,
        key,
        current_key,
        kind,
        attacker: mover,
        target,
        minimum_distance,
        maximum_distance,
        state,
        distance,
        flag_006e43e0,
    } = check;
    let attacker = attacker(e, this);
    let near_radius = x87_quiet(e.global::<f32>(TWO_HUNDRED_FIFTY_SIX));
    e.with_stack(COVER_CHECK_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_CHECK_FRAME - k;
        if e.call(0x0098_2be0, &args![cover_location]).bool()
            && flag_006e43e0 == 0
            && e.mem.u8(cover_location + 0x4f) == 0
            && e.mem.u8(cover_location + 0x50) == 0
        {
            e.mem.set_u32(state, 2);
            return false;
        }
        if key == current_key {
            let target_location = e.vcall(target, 0x1f4, &args![]).u32();
            let point = e
                .call(0x005c_3420, &args![cover_location, local(0xd8)])
                .u32();
            if !e
                .call(
                    0x0069_6540,
                    &args![
                        point,
                        target_location,
                        cover_location + 0x28,
                        cover_location + 0x34
                    ],
                )
                .bool()
            {
                e.mem.set_u32(state, 0xd);
                return false;
            }
        }
        if e.call(0x009a_0d70, &args![this, key]).bool() {
            e.mem.set_u32(state, 3);
            return false;
        }
        let mut reason = 0u32;
        let mut previous = 0u32;
        e.mem.set_f32(distance, 0.0);
        let mut travel_time = x87_quiet(e.global::<f32>(MINUS_ONE));
        let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
        let mut side = 1u32;
        if e.mem.u8(reservation + 1) != 0 {
            if e.mem.u8(reservation + 5) != 0 {
                side = 0;
            }
        } else {
            let mover_location = e.vcall(mover, 0x1f4, &args![]).u32();
            if e.call(0x006e_4840, &args![cover_location, mover_location])
                .bool()
            {
                side = 0;
            }
        }
        let both_edges = e.call(0x0098_2be0, &args![cover_location]).bool() && flag_006e43e0 == 0;
        let lean = (e.call(0x0098_2be0, &args![cover_location]).bool()
            || e.call(0x006e_4820, &args![cover_location]).bool())
            && flag_006e43e0 == 0;
        let passes = both_edges as u32 + 1;
        let (point, lean_point) = (local(0x64), local(0x48));
        let (location, lean_location) = (local(0x90), local(0xbc));
        for pass in 0..passes {
            if reason != 0 {
                previous = reason;
            }
            let mut edge = 0u8;
            if both_edges {
                if pass == side && e.mem.u8(cover_location + 0x4f) != 0 {
                    edge = 1;
                } else if pass != side && e.mem.u8(cover_location + 0x50) != 0 {
                    edge = 0;
                } else {
                    continue;
                }
            }
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            let controller = state_controller(e, this);
            if !e
                .call(
                    0x0098_35d0,
                    &args![
                        cover_map,
                        key,
                        cover_location,
                        controller,
                        x87_quiet_bits(radius.to_bits()),
                        reservation,
                        flag_006e43e0 as u32,
                        edge as u32
                    ],
                )
                .bool()
            {
                reason = 9;
                continue;
            }
            e.call(0x0068_15c0, &args![point]);
            e.call(0x0068_15c0, &args![lean_point]);
            let controller = state_controller(e, this);
            e.call(
                0x009a_93a0,
                &args![controller, cover_location, reservation, point, lean_point],
            );
            if e.call(0x006e_4820, &args![cover_location]).bool()
                && target != 0
                && !e
                    .call(
                        0x0099_cec0,
                        &args![this, cover_location, point, search_point, target],
                    )
                    .bool()
            {
                reason = 6;
                continue;
            }
            e.call(0x006d_d0d0, &args![location, point, cover_location]);
            let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
            if !e
                .call(0x006d_ed70, &args![location, radius.to_bits()])
                .bool()
            {
                reason = 4;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if lean {
                e.call(
                    0x006d_d0d0,
                    &args![lean_location, lean_point, cover_location],
                );
                let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
                if !e
                    .call(0x006d_ed70, &args![lean_location, radius.to_bits()])
                    .bool()
                {
                    reason = 5;
                    e.call(0x004f_f7e0, &args![lean_location]);
                    e.call(0x004f_f7e0, &args![location]);
                    continue;
                }
                e.call(0x004f_f7e0, &args![lean_location]);
            }
            let space = e.call(GET_SPACE, &args![mover]).u32();
            if !e
                .call(0x009a_0260, &args![this, point, space, 1u32, 0u32])
                .bool()
            {
                reason = 0x10;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let difference = e
                .call(0x0043_9ef0, &args![search_point, local(0xe4), point])
                .u32();
            let squared = e.call(0x004a_7290, &args![difference]).f32();
            e.mem.set_f32(distance, squared);
            let squared = e.mem.f32(distance) as f64;
            if squared > maximum_distance as f64 * maximum_distance as f64 {
                reason = 7;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if squared < minimum_distance as f64 * minimum_distance as f64 {
                reason = 8;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let world = local(0x58);
            e.call(0x0097_ede0, &args![world]);
            let space = e.call(GET_SPACE, &args![mover]).u32();
            e.call(0x0098_9010, &args![world, point, space]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            if e.call(
                0x0098_9700,
                &args![group, world, x87_quiet_bits(near_radius.to_bits()), 0u32],
            )
            .u32()
                != 0
            {
                reason = 0xa;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if travel_time as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
                let from = e.vcall(mover, 0x1f4, &args![]).u32();
                let offset = local(0xc8);
                e.call(0x0043_9ef0, &args![from, offset, point]);
                let length = e.call(0x0045_7990, &args![offset]).f64();
                let speed = e.call(0x008a_0b10, &args![mover]).f64();
                travel_time = (length / speed) as f32;
            }
            if threat_at(e, point, mover, travel_time) {
                reason = 0xb;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if lean && threat_at(e, lean_point, mover, travel_time) {
                reason = 0xc;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let mut blocked = e
                .call(0x0099_cc60, &args![location, search_point, 1u32])
                .bool();
            if !blocked {
                blocked = fn_0099ca00(e, this, location, target);
            }
            if blocked {
                reason = 0xd;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let check_sight = !(kind == 5 || flag_006e43e0 != 0 || key == current_key);
            if check_sight
                && !e
                    .call(
                        0x009a_7940,
                        &args![
                            mover,
                            target,
                            lean_point,
                            search_point,
                            1u32,
                            0xffff_ffffu32
                        ],
                    )
                    .bool()
            {
                reason = 0xe;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            fn_0099b8b0(e, Ptr::new(out), point, 0);
            e.call(0x004f_f7e0, &args![location]);
            return true;
        }
        if (previous as i32) > (reason as i32) {
            e.mem.set_u32(state, previous);
        } else {
            e.mem.set_u32(state, reason);
        }
        false
    })
}

/// The threat test of `0099c230`: the threat map (`00825c00` on the combat
/// manager) value at `point` for `mover` arriving after `travel_time`
/// (`009a5670`, with a `CombatTimeStamp` of 0 built on the argument stack by
/// `00435de0`), true when it is above zero.
fn threat_at(e: &mut Engine, point: u32, mover: u32, travel_time: f32) -> bool {
    let stamp = time_stamp_word(e, 0.0f32.to_bits());
    let (x, y, z) = (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8));
    let manager: u32 = e.global(COMBAT_MANAGER);
    let threat_map = e.call(0x0082_5c00, &args![manager]).u32();
    let threat = e
        .call(
            0x009a_5670,
            &args![
                threat_map,
                x,
                y,
                z,
                mover,
                x87_quiet_bits(travel_time.to_bits()),
                0.0f32.to_bits(),
                stamp,
                0.0f32.to_bits()
            ],
        )
        .f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    threat > zero
}

/// Stack frame of `0099ca00`; a local at `EBP - k` is at
/// `frame + BLOCKING_FRAME - k`.
const BLOCKING_FRAME: u32 = 0x30;

// Translated from 0099ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether another member of the attacker's group (`00968630` over
/// `005a4320` members, `ignored` excepted) blocks `location`: each member's
/// position (its group location from `00989140` within 3 units, or its
/// current one) is tested with `0099cc60`.
pub fn fn_0099ca00(e: &mut Engine, this: Ptr<CombatState>, location: u32, ignored: u32) -> bool {
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let count = e.call(0x005a_4320, &args![group]).u32();
    e.with_stack(BLOCKING_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + BLOCKING_FRAME - k;
        let mut index = 0u32;
        while index < count {
            let ally = e.call(0x0096_8630, &args![group, index]).u32();
            if ally == ignored {
                index += 1;
                continue;
            }
            let world = local(0x1c);
            e.call(0x0097_ede0, &args![world]);
            let position = local(0x28);
            e.call(0x0068_15c0, &args![position]);
            let three = constant_bits(e, THREE);
            if e.call(0x0098_9140, &args![group, index, world, three])
                .bool()
            {
                let source = e.call(0x0068_15c0, &args![world]).u32();
                copy_words(e, position, source, 3);
            } else {
                let source = e.vcall(ally, 0x1f4, &args![]).u32();
                copy_words(e, position, source, 3);
            }
            if e.call(0x0099_cc60, &args![location, position, 0u32]).bool() {
                return true;
            }
            index += 1;
        }
        false
    })
}

// Translated from 0099cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any point of the array `points` (`006a7af0`, three words each)
/// blocks `location` (`0099cc60`). cdecl.
pub fn fn_0099cb00(e: &mut Engine, location: u32, points: u32) -> bool {
    e.with_stack(12, |e, point| {
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![points]).u32() {
            let source = e.call(0x006a_7af0, &args![points, index]).u32();
            copy_words(e, point.addr(), source, 3);
            if e.call(0x0099_cc60, &args![location, point, 0u32]).bool() {
                return true;
            }
            index += 1;
        }
        false
    })
}

/// The uniform form of `0099c230` (fourteen argument words after `this`).
fn fn_0099c230_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let check = CoverCheck {
        cover_location: words[1],
        out: words[2],
        reservation: words[3],
        search_point: words[4],
        key: words[5],
        current_key: words[6],
        kind: words[7],
        attacker: words[8],
        target: words[9],
        minimum_distance: f32::from_bits(words[10]),
        maximum_distance: f32::from_bits(words[11]),
        state: words[12],
        distance: words[13],
        flag_006e43e0: words[14] as u8,
    };
    RetVal::into_ret(fn_0099c230(e, Ptr::new(words[0]), check))
}

/// `009052c0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_009052C0: u32 = 0x0090_52c0;

// Translated from 009a3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (callers `SaveGame`, `LoadGame` of the combat state and
/// `CalculateLocationToExamine`): forwards `this` and an index to `009052c0` (in
/// `highprocess.cpp` per the engine map), which returns the word at `this+4` plus `index`
/// times 0x54: the address of a 0x54-byte element. The wrapper returns that address.
pub fn fn_009a3280(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_009052C0, &args![this, index]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESObjectWEAP@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E068: u32 = 0x0108_e068;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableCoverLocation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E07C: u32 = 0x0108_e07c;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableLocation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E090: u32 = 0x0108_e090;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UWeaponData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E0A4: u32 = 0x0108_e0a4;

// Translated from 009a3410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESObjectWEAP@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3410(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E068);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableCoverLocation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3460(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E07C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableLocation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a35f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E090);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UWeaponData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3810(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E0A4);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, two stack words: whether the 32-bit words the two pointers
/// point to are equal (`SETZ AL`, the result is `AL`).
pub fn fn_009a3830(e: &mut Engine, left: Ptr, right: Ptr) -> bool {
    e.mem.u32(left.addr()) == e.mem.u32(right.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_3830, fn_009a3830(Ptr, Ptr) -> bool),
        entry!(0x009a_3280, fn_009a3280(Ptr, u32) -> Ptr),
        entry!(
            0x006f2290,
            bs_simple_array_tes_object_weap_set_size(Ptr<BSSimpleArray>, u32, u8)
        ),
        entry!(
            0x0097ee20,
            fn_0097ee20(Ptr<CombatLocation>) -> Ptr<CombatLocation>
        ),
        entry!(
            0x009973a0,
            fn_009973a0(Ptr<CombatLocation>, Ptr<CombatLocation>)
        ),
        entry!(
            0x00997400,
            combat_location_set_ref(Ptr<CombatLocation>, u32)
        ),
        entry!(
            0x00997440,
            combat_location_update_ref_location(Ptr<CombatLocation>)
        ),
        entry!(0x00997480, fn_00997480(Ptr, Ptr)),
        entry!(
            0x00997520,
            fn_00997520(Ptr<CombatCoverSearchInfo>, Ptr<CombatCoverSearchInfo>)
        ),
        entry!(0x009975c0, fn_009975c0(Ptr<CombatCoverSearchInfo>)),
        entry!(
            0x00997600,
            fn_00997600(Ptr<CombatState>, u32, u32) -> Ptr<CombatState>
        ),
        entry!(0x00997aa0, fn_00997aa0(Ptr<CombatState>)),
        entry!(0x00997ba0, fn_00997ba0(Ptr<CombatState>, u32, u32)),
        entry!(0x00997bf0, fn_00997bf0(Ptr<CombatState>)),
        entry!(0x00997c10, fn_00997c10(Ptr<CombatState>)),
        entry!(0x00997c30, fn_00997c30(Ptr<CombatState>)),
        entry!(0x00997cb0, fn_00997cb0(Ptr<CombatState>)),
        entry!(0x00997cf0, fn_00997cf0(Ptr<CombatState>)),
        entry!(0x00998980, fn_00998980(Ptr<CombatState>) -> bool),
        entry!(0x009989d0, fn_009989d0(u32) -> bool),
        entry!(0x009989f0, fn_009989f0(u32) -> u8),
        entry!(0x00998a10, fn_00998a10(u32) -> u8),
        entry!(0x00998a30, fn_00998a30(u32) -> u8),
        entry!(0x00998a50, fn_00998a50(Ptr<CombatState>)),
        entry!(0x009990c0, fn_009990c0(Ptr<CombatTimer>) -> bool),
        entry!(0x00999100, fn_00999100(Ptr<CombatState>)),
        entry!(0x00999120, fn_00999120(u32) -> bool),
        entry!(0x00999160, fn_00999160(Ptr<CombatState>)),
        entry!(0x00999260, fn_00999260(Ptr<CombatState>)),
        entry!(0x009993c0, fn_009993c0(Ptr<CombatState>)),
        entry!(0x0099a2a0, fn_0099a2a0(Ptr<CombatState>)),
        entry!(0x0099a2d0, fn_0099a2d0(u32) -> u32),
        entry!(0x0099a360, fn_0099a360(Ptr<CombatState>, u32, u32) -> bool),
        entry!(0x0099a3e0, fn_0099a3e0(u32) -> bool),
        entry!(0x0099a420, fn_0099a420(Ptr<CombatState>, u32, u8)),
        entry!(0x0099a530, fn_0099a530(Ptr<CombatState>)),
        entry!(0x0099a5e0, fn_0099a5e0(Ptr<CombatState>, u32)),
        entry!(0x0099a650, fn_0099a650(Ptr<CombatState>, u32, u8)),
        entry!(0x0099a680, fn_0099a680(Ptr<CombatState>, u32)),
        entry!(0x0099a6b0, fn_0099a6b0(Ptr<CombatState>, u32, u32)),
        entry!(0x0099a720, fn_0099a720(Ptr<CombatState>)),
        entry!(
            0x0099a780,
            combat_state_pre_activate_reference(Ptr<CombatState>, u32)
        ),
        entry!(
            0x0099a7d0,
            fn_0099a7d0(Ptr<CombatState>, u32, u32, u32, u32)
        ),
        entry!(0x0099a840, fn_0099a840(Ptr<CombatState>, u32)),
        entry!(0x0099a8d0, fn_0099a8d0(Ptr<CombatState>)),
        entry!(0x0099a9a0, fn_0099a9a0(Ptr<CombatState>)),
        entry!(0x0099aa10, fn_0099aa10(Ptr<CombatState>, u32, u32)),
        entry!(0x0099aa80, pathing_cover_location_pathing_cover_location(Ptr, Ptr) -> Ptr),
        entry!(
            0x0099ab50,
            fn_0099ab50(Ptr<CombatState>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x0099abc0,
            combat_state_get_last_seen_target_firing_location(
                Ptr<CombatState>,
                Ptr<NiPoint3>,
            ) -> bool
        ),
        entry!(
            0x0099ac20,
            combat_state_find_combat_location(Ptr<CombatState>, u32) -> bool
        ),
        entry!(0x0099b520, fn_0099b520(Ptr<CombatLocation>, u32)),
        entry!(0x0099b540, fn_0099b540(Ptr<CombatLocation>, u32, u8)),
        entry!(0x0099b580, fn_0099b580(Ptr<CombatLocation>) -> u32),
        entry!(
            0x0099b5a0,
            fn_0099b5a0(Ptr<CombatState>, Ptr<CombatLocation>, u32, u32) -> bool
        ),
        entry!(0x0099b8b0, fn_0099b8b0(Ptr<CombatLocation>, u32, u32)),
        entry!(
            0x0099b8e0,
            fn_0099b8e0(Ptr<CombatState>, u32, Ptr<CombatCoverSearchInfo>, u32, u32, u32) -> bool
        ),
        entry!(0x0099c1f0, fn_0099c1f0(u32) -> u8),
        entry!(0x0099c210, fn_0099c210(u32)),
        (0x0099c230, fn_0099c230_entry as AbiFn),
        entry!(0x0099ca00, fn_0099ca00(Ptr<CombatState>, u32, u32) -> bool),
        entry!(0x0099cb00, fn_0099cb00(u32, u32) -> bool),
        entry!(0x009a_3410, fn_009a3410(Ptr)),
        entry!(0x009a_3460, fn_009a3460(Ptr)),
        entry!(0x009a_35f0, fn_009a35f0(Ptr)),
        entry!(0x009a_3810, fn_009a3810(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the translations call directly.
    const EXTERNAL: &[u32] = &[
        0x0040_1000,
        0x0040_1030,
        0x0040_1050,
        0x0040_1170,
        0x0040_30d0,
        0x0040_3d30,
        0x0040_4010,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_5c00,
        0x0040_ebd0,
        0x0041_6870,
        0x0041_8b10,
        0x0041_fd00,
        0x0042_fc00,
        0x0043_00a0,
        0x0043_7b90,
        0x0043_9090,
        0x0043_90c0,
        0x0043_9ef0,
        0x0044_2a80,
        0x0044_ddc0,
        0x0044_edb0,
        0x0045_36e0,
        0x0045_3700,
        0x0045_7990,
        0x0046_1580,
        0x0046_e8c0,
        0x0047_6b70,
        0x0048_3710,
        0x0049_5460,
        0x0049_97b0,
        0x004a_7290,
        0x004a_7bd0,
        0x004b_c550,
        0x004b_cdb0,
        0x004b_f220,
        0x004c_7300,
        0x004c_8f30,
        0x004d_4830,
        0x004d_ff00,
        0x004f_9bf0,
        0x004f_d3c0,
        0x004f_d3e0,
        0x004f_f7e0,
        0x0050_76c0,
        0x0051_b840,
        0x0052_2c80,
        0x0052_4b60,
        0x0052_5980,
        0x0052_5a90,
        0x0052_7270,
        0x0052_72b0,
        0x0052_7a40,
        0x0052_7ac0,
        0x0054_07b0,
        0x0055_9450,
        0x0057_1600,
        0x0057_5650,
        0x0057_5ca0,
        0x0059_5c80,
        0x005a_4320,
        0x005b_5e40,
        0x005c_3420,
        0x005d_c960,
        0x005e_3fc0,
        0x0060_0ab0,
        0x0062_96b0,
        0x0064_6060,
        0x0067_a700,
        0x0068_15c0,
        0x0069_6540,
        0x006a_7ad0,
        0x006a_7af0,
        0x006a_7f50,
        0x006b_3160,
        0x006b_9130,
        0x006d_3150,
        0x006d_3280,
        0x006d_32e0,
        0x006d_3370,
        0x006d_3ac0,
        0x006d_62e0,
        0x006d_ad40,
        0x006d_ad70,
        0x006d_ad90,
        0x006d_bde0,
        0x006d_cc40,
        0x006d_ce10,
        0x006d_d0d0,
        0x006d_ed70,
        0x006e_4080,
        0x006e_4320,
        0x006e_4340,
        0x006e_4380,
        0x006e_43a0,
        0x006e_43c0,
        0x006e_43e0,
        0x006e_4400,
        0x006e_4430,
        0x006e_4570,
        0x006e_4650,
        0x006e_4740,
        0x006e_4820,
        0x006e_4840,
        0x006e_4d00,
        0x006e_9bd0,
        0x006e_bf60,
        0x006e_bf90,
        0x006e_cd40,
        0x0070_31c0,
        0x0072_6070,
        0x0072_6bf0,
        0x0072_ba80,
        0x0078_81d0,
        0x007a_f430,
        0x007c_b2e0,
        0x0080_0190,
        0x0081_9a50,
        0x0082_5c00,
        0x0084_54f0,
        0x0084_e3a0,
        0x0088_46c0,
        0x0089_4900,
        0x008a_0b10,
        0x008a_6840,
        0x008a_8840,
        0x008a_8870,
        0x008b_c9d0,
        0x008b_e1b0,
        0x008b_e420,
        0x008d_8520,
        0x008e_eaa0,
        0x008f_6630,
        0x0090_52c0,
        0x0090_57d0,
        0x0090_6f60,
        0x0093_1850,
        0x0094_4460,
        0x0096_11e0,
        0x0096_8630,
        0x0096_a5f0,
        0x0097_9f70,
        0x0097_ac90,
        0x0097_acb0,
        0x0097_b940,
        0x0097_bd00,
        0x0097_c150,
        0x0097_cfc0,
        0x0097_ede0,
        0x0097_ee60,
        0x0097_ee90,
        0x0097_f200,
        0x0097_f220,
        0x0097_f310,
        0x0097_fb20,
        0x0097_fd40,
        0x0098_0070,
        0x0098_0700,
        0x0098_1300,
        0x0098_1340,
        0x0098_1380,
        0x0098_1520,
        0x0098_1940,
        0x0098_1970,
        0x0098_1990,
        0x0098_2be0,
        0x0098_35d0,
        0x0098_3970,
        0x0098_87b0,
        0x0098_9010,
        0x0098_90e0,
        0x0098_9140,
        0x0098_9700,
        0x0098_9e20,
        0x0098_9eb0,
        0x0098_bc00,
        0x0098_bd70,
        0x0098_cd50,
        0x0098_de00,
        0x0098_f1f0,
        0x0099_2720,
        0x0099_28c0,
        0x0099_5890,
        0x0099_5980,
        0x0099_59c0,
        0x0099_62f0,
        0x0099_cc60,
        0x0099_ccd0,
        0x0099_cec0,
        0x0099_d030,
        0x0099_e190,
        0x0099_ebe0,
        0x0099_f480,
        0x0099_f6d0,
        0x009a_0260,
        0x009a_02a0,
        0x009a_03d0,
        0x009a_0470,
        0x009a_0680,
        0x009a_0790,
        0x009a_0910,
        0x009a_0a40,
        0x009a_0a80,
        0x009a_0b00,
        0x009a_0b70,
        0x009a_0c50,
        0x009a_0d70,
        0x009a_1010,
        0x009a_1060,
        0x009a_1280,
        0x009a_1380,
        0x009a_1400,
        0x009a_33e0,
        0x009a_3410,
        0x009a_3430,
        0x009a_3460,
        0x009a_35c0,
        0x009a_35f0,
        0x009a_3750,
        0x009a_3780,
        0x009a_37e0,
        0x009a_3810,
        0x009a_5670,
        0x009a_7670,
        0x009a_7940,
        0x009a_7e00,
        0x009a_7f00,
        0x009a_8050,
        0x009a_8460,
        0x009a_9180,
        0x009a_93a0,
        0x009a_9630,
        0x00aa_6060,
    ];

    /// A shared vtable whose slot `s` calls `SLOTS + s` (all doubled).
    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;
    /// The start-time word that makes the timer double report "expired".
    const EXPIRED: u32 = 1;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| st0(value));
    }

    /// Returns `results` in turn (the last one again afterwards).
    fn sequence(e: &mut Engine, addr: u32, results: Vec<u32>) {
        let mut next = 0;
        e.register_double(addr, move |_, _| {
            let value = results[next.min(results.len() - 1)];
            next += 1;
            ret(value)
        });
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size.max(0x10));
        e.mem.set_u32(object, VTABLE);
        object
    }

    fn put_f32(e: &mut Engine, addr: u32, value: f32) {
        e.mem.set_f32(addr, value);
    }

    fn put_f64(e: &mut Engine, addr: u32, value: f64) {
        e.mem.set_f64(addr, value);
    }

    /// Every outside callee doubled (returning 0), the constants the code
    /// reads, and working doubles for the small getters, time stamps and
    /// timers (game time 5).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let mut slots = vec![];
        for slot in (0..0x800).step_by(4) {
            slots.push(SLOTS + slot);
            stub(&mut e, SLOTS + slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_0000, 0x0008_0000);
        e.map(0x0108_d000, 0x2000);
        e.map(0x011a_4000, 0x2000);
        e.map(0x011c_e000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_1000, 0x6000);
        for (addr, value) in [
            (MAX_FLOAT, f32::MAX),
            (HUNDRED, 100.0),
            (MINUS_ONE, -1.0),
            (TWO, 2.0),
            (QUARTER, 0.25),
            (HALF, 0.5),
            (TENTH, 0.1),
            (FIVE, 5.0),
            (TEN, 10.0),
            (THREE, 3.0),
            (FORCED_WEAPON_DPS, 9_999_999.0),
            (DODGE_DISTANCE, 128.0),
            (FIVE_HUNDRED_TWELVE, 512.0),
            (TWO_THOUSAND_FORTY_EIGHT, 2048.0),
            (TEN_THOUSAND, 10000.0),
            (FOUR_THOUSAND_NINETY_SIX, 4096.0),
            (TWO_HUNDRED_FIFTY_SIX, 256.0),
            (LOCATION_TARGET_MAX_DISTANCE, 1024.0),
            (LOCATION_TARGET_SECOND_DISTANCE, 768.0),
            (LOCATION_TARGET_FIRST_DISTANCE, 384.0),
        ] {
            put_f32(&mut e, addr, value);
        }
        for (addr, value) in [
            (ZERO_DOUBLE, 0.0),
            (ONE_DOUBLE, 1.0),
            (TWO_HUNDRED_FIFTY_SIX_DOUBLE, 256.0),
            (HUNDRED_DOUBLE, 100.0),
            (HUNDRED_TWENTY_DOUBLE, 120.0),
            (TEN_DOUBLE, 10.0),
            (SIXTY_FIVE_THOUSAND_DOUBLE, 65536.0),
            (CURRENT_WEAPON_BONUS_DOUBLE, 1_000_000.0),
            (OUT_OF_RANGE_DIVISOR_DOUBLE, 10000.0),
            (TWO_DOUBLE, 2.0),
            (THOUSAND_DOUBLE, 1000.0),
            (THREE_DOUBLE, 3.0),
            (MINUS_ONE_DOUBLE, -1.0),
        ] {
            put_f64(&mut e, addr, value);
        }
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(CONTROLLER_ATTACKER, |e, a| ret(e.mem.u32(a[0] + 0xbc)));
        e.register(CONTROLLER_TARGET, |e, a| ret(e.mem.u32(a[0] + 0xc0)));
        e.register(CONTROLLER_GROUP, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(GAME_TIME, |_, _| st0(5.0));
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(TIME_STAMP_ELAPSED, |e, a| {
            st0((5.0 - e.mem.f32(a[0])) as f64)
        });
        e.register(TIMER_START, |e, a| {
            e.mem.set_f32(a[0], 5.0);
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e.register(TIMER_EXPIRED, |e, a| {
            ret((e.mem.u32(a[0]) == EXPIRED) as u32)
        });
        e.register(TIMER_RUNNING, |e, a| {
            ret((e.mem.f32(a[0] + 4) != 0.0) as u32)
        });
        e.register(TIMER_STOP, |e, a| {
            e.mem.set_f32(a[0] + 4, 0.0);
            ret(0)
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e
    }

    struct Fixture {
        state: Ptr<CombatState>,
        controller: u32,
        attacker: u32,
        target: u32,
    }

    /// A state with its controller, an attacker (with an object at +0xa4)
    /// and, if asked, a target; the attacker's process is an object too.
    fn fixture(e: &mut Engine, with_target: bool) -> Fixture {
        let state: Ptr<CombatState> = e.new_object();
        let controller = e.mem.alloc(0x188);
        let attacker = object(e, 0x200);
        e.mem.set_u32(attacker + 0xa4, VTABLE);
        let target = if with_target { object(e, 0x200) } else { 0 };
        let group = object(e, 0x40);
        e.mem.set_u32(controller + 0xbc, attacker);
        e.mem.set_u32(controller + 0xc0, target);
        e.mem.set_u32(controller + 0x80, group);
        e.set(state, CombatState::pCombatController, controller);
        let process = object(e, 0x40);
        stub(e, 0x008d_8520, process);
        Fixture {
            state,
            controller,
            attacker,
            target,
        }
    }

    fn timer(state: Ptr<CombatState>, off: u32) -> u32 {
        state.addr() + off
    }

    #[test]
    fn set_size_grows_an_empty_array_through_its_vtable() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), VTABLE);
        stub(&mut e, SLOTS + 4, 0x5000);
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 3, 0)
        });
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x5000);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(
            calls(&log, 0x0072_6bf0),
            vec![vec![array.addr(), 0x5000, 3]]
        );

        // Shrinking to a quarter of the reservation with `shrink` reallocates.
        e.set(array, BSSimpleArray::iReservedSize, 8);
        e.set(array, BSSimpleArray::iSize, 8);
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 2, 1)
        });
        assert_eq!(
            calls(&log, 0x0072_ba80),
            vec![vec![array.addr(), 0x5008, 6]]
        );
        assert_eq!(calls(&log, 0x0042_fc00), vec![vec![array.addr(), 2, 2]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);

        // Size 0 hands over with the shrink flag.
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 0, 1)
        });
        assert_eq!(calls(&log, 0x0084_54f0), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn combat_location_initialisation_sets_defaults() {
        let mut e = engine();
        let location: Ptr<CombatLocation> = e.new_object();
        e.set(location, CombatLocation::pRef, 7);
        assert_eq!(fn_0097ee20(&mut e, location), location);
        assert_eq!(e.get(location, CombatLocation::pRef), 0);
        assert_eq!(e.get(location, CombatLocation::iLocation), -1);
        assert_eq!(e.get(location, CombatLocation::iFlags), 0);
    }

    #[test]
    fn combat_location_copy_quiets_the_radius() {
        let mut e = engine();
        let (to, from): (Ptr<CombatLocation>, Ptr<CombatLocation>) =
            (e.new_object(), e.new_object());
        for i in 0..8 {
            e.mem.set_u32(from.addr() + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(from.addr() + 0x18, 0x7f80_0001);
        fn_009973a0(&mut e, to, from);
        assert_eq!(e.mem.u32(to.addr() + 0xc), 0x103);
        assert_eq!(e.mem.u32(to.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(e.get(to, CombatLocation::iFlags), 0x107);
    }

    fn world_location_double(e: &mut Engine) {
        e.register(GET_WORLD_LOCATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[1] + 4 * i, a[0] + i);
            }
            ret(a[1])
        });
    }

    #[test]
    fn set_ref_copies_the_reference_location() {
        let mut e = engine();
        world_location_double(&mut e);
        let location: Ptr<CombatLocation> = e.new_object();
        combat_location_set_ref(&mut e, location, 0x40);
        assert_eq!(e.get(location, CombatLocation::pRef), 0x40);
        assert_eq!(e.mem.u32(location.addr() + 0xc), 0x43);
    }

    #[test]
    fn update_ref_location_needs_a_reference() {
        let mut e = engine();
        world_location_double(&mut e);
        let location: Ptr<CombatLocation> = e.new_object();
        let log = logged(&mut e, |e| combat_location_update_ref_location(e, location));
        assert!(calls(&log, GET_WORLD_LOCATION).is_empty());
        e.set(location, CombatLocation::pRef, 0x80);
        combat_location_update_ref_location(&mut e, location);
        assert_eq!(e.mem.u32(location.addr()), 0x80);
    }

    #[test]
    fn cover_location_copy_copies_fields_and_the_pathing_location() {
        let mut e = engine();
        let (to, from) = (Ptr::new(e.mem.alloc(0x78)), Ptr::new(e.mem.alloc(0x78)));
        e.mem.set_u8(from.addr() + 2, 9);
        e.mem.set_u32(from.addr() + 0x6c, 0x1234);
        e.mem.set_u16(from.addr() + 0x74, 0xbeef);
        let log = logged(&mut e, |e| fn_00997480(e, to, from));
        assert_eq!(e.mem.u8(to.addr() + 2), 9);
        assert_eq!(e.mem.u32(to.addr() + 0x6c), 0x1234);
        assert_eq!(e.mem.u16(to.addr() + 0x74), 0xbeef);
        assert_eq!(
            calls(&log, 0x006e_4430),
            vec![vec![to.addr() + 0x14, from.addr() + 0x14]]
        );
    }

    #[test]
    fn cover_search_info_assignment_and_reset() {
        let mut e = engine();
        let (to, from): (Ptr<CombatCoverSearchInfo>, Ptr<CombatCoverSearchInfo>) =
            (e.new_object(), e.new_object());
        e.set(from, CombatCoverSearchInfo::fRadius, 3.0);
        e.set(from, CombatCoverSearchInfo::bUsedLastSeenLocation, 1);
        let log = logged(&mut e, |e| fn_00997520(e, to, from));
        assert_eq!(e.get(to, CombatCoverSearchInfo::fRadius), 3.0);
        assert_eq!(e.get(to, CombatCoverSearchInfo::bUsedLastSeenLocation), 1);
        assert_eq!(
            calls(&log, 0x009a_3780),
            vec![vec![from.addr() + 0x20, to.addr() + 0x20]]
        );
        let log = logged(&mut e, |e| fn_00997520(e, to, Ptr::NULL));
        assert_eq!(e.get(to, CombatCoverSearchInfo::fRadius), 0.0);
        assert_eq!(
            e.get(to, CombatCoverSearchInfo::fTimeStamp),
            f32::MAX.to_bits() ^ 0x8000_0000
        );
        assert_eq!(calls(&log, 0x006d_bde0), vec![vec![to.addr() + 0x20, 1]]);
    }

    #[test]
    fn reset_cover_search_info_stamps_minus_max() {
        let mut e = engine();
        let info: Ptr<CombatCoverSearchInfo> = e.new_object();
        e.set(info, CombatCoverSearchInfo::fRadius, 9.0);
        fn_009975c0(&mut e, info);
        assert_eq!(e.get(info, CombatCoverSearchInfo::fRadius), 0.0);
        assert_eq!(
            f32::from_bits(e.get(info, CombatCoverSearchInfo::fTimeStamp)),
            -f32::MAX
        );
    }

    #[test]
    fn constructor_initialises_the_state() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_f804 + 4, 8.0);
        e.register(RANDOM_FLOAT, |_, a| {
            st0(f32::from_bits(a[0]) as f64 + f32::from_bits(a[1]) as f64)
        });
        put_f32(&mut e, 0x011c_fdcc + 4, 0.25);
        put_f32(&mut e, 0x011c_f7ec + 4, 0.5);
        let log = logged(&mut e, |e| {
            assert_eq!(fn_00997600(e, state, 0x4444, 0), state);
        });
        assert_eq!(e.get(state, CombatState::fThreatRatio), 100.0);
        assert_eq!(e.get(state, CombatState::cTargetFlushed), -1);
        assert_eq!(e.get(state, CombatState::eTargetSegmentInView), 3);
        assert_eq!(e.get(state, CombatState::fCombatStrength), -1.0);
        assert_eq!(e.get(state, CombatState::cLastDodgeDirection), 0xff);
        assert_eq!(e.get(state, CombatState::pCombatController), 0x4444);
        assert_eq!(e.get(state, CombatState::bInitializing), 1);
        assert_eq!(e.get(state, CombatState::iQueuedPathCheck), -1);
        assert_eq!(e.get(state, CombatState::fConfidenceModifier), 0.75);
        assert_eq!(calls(&log, TIME_STAMP_SET).len(), 6);
        assert_eq!(calls(&log, 0x0090_6f60).len(), 14);
        assert_eq!(
            calls(&log, 0x0040_1050),
            vec![vec![state.addr() + 0x1ac, 8, 2, 0x0090_6f60]]
        );
        assert_eq!(
            calls(&log, TIMER_START),
            vec![vec![state.addr() + 0x20c, 8.0f32.to_bits()]]
        );
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997aa0(e, state));
        assert_eq!(calls(&log, 0x0098_1340).len(), 5);
        assert_eq!(calls(&log, 0x009a_3410), vec![vec![state.addr() + 0x24]]);
        assert_eq!(calls(&log, 0x006e_4d00).len(), 2);
        assert_eq!(
            calls(&log, CLEAR_COVER_LOCATION),
            vec![vec![state.addr(), 0]]
        );
    }

    #[test]
    fn clearing_cover_needs_first_zero_and_second_set() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997ba0(e, state, 1, 1));
        assert!(calls(&log, CLEAR_COVER_LOCATION).is_empty());
        let log = logged(&mut e, |e| fn_00997ba0(e, state, 0, 1));
        assert_eq!(calls(&log, CLEAR_COVER_LOCATION).len(), 1);
        assert_eq!(calls(&log, 0x006d_bde0).len(), 2);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![state.addr()]]);
    }

    #[test]
    fn cover_search_resets_pick_their_member() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997bf0(e, state));
        assert_eq!(
            calls(&log, 0x006d_bde0),
            vec![vec![state.addr() + 0xec + 0x20, 1]]
        );
        let log = logged(&mut e, |e| fn_00997c10(e, state));
        assert_eq!(
            calls(&log, 0x006d_bde0),
            vec![vec![state.addr() + 0x148, 1]]
        );
    }

    #[test]
    fn first_update_sets_confidence_and_ends_initialisation() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0097_f310, 1);
        stub(&mut e, 0x0093_1850, 1);
        stub(&mut e, SLOTS + 8, 3);
        e.set(f.state, CombatState::bInitializing, 1);
        let log = logged(&mut e, |e| fn_00997c30(e, f.state));
        assert_eq!(e.get(f.state, CombatState::bHasHandToHandEquip), 1);
        assert_eq!(e.get(f.state, CombatState::cStartingConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::cCurrentConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::bInitializing), 0);
        assert_eq!(calls(&log, 0x0097_f310), vec![vec![f.attacker, 0]]);
        assert_eq!(calls(&log, SLOTS + 8), vec![vec![f.attacker + 0xa4, 1]]);
    }

    #[test]
    fn update_chooses_the_reduced_update_outside_the_high_process() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0093_1850, 2);
        let strength = timer(f.state, 0x204);
        e.mem.set_u32(strength, EXPIRED);
        let log = logged(&mut e, |e| fn_00997cb0(e, f.state));
        assert_eq!(calls(&log, 0x009a_1010), vec![vec![f.state.addr()]]);
        assert!(calls(&log, 0x009a_0c50).is_empty());
    }

    #[test]
    fn full_update_without_a_target_resets_the_target_information() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::iTargetDetectionLevel, 3);
        e.set(f.state, CombatState::bTargetSpaceDifferent, 1);
        let log = logged(&mut e, |e| fn_00997cf0(e, f.state));
        let minus_max = f32::MAX.to_bits() ^ 0x8000_0000;
        assert_eq!(e.get(f.state, CombatState::iTargetDetectionLevel), -1);
        assert_eq!(e.get(f.state, CombatState::cTargetFlushed), -1);
        assert_eq!(e.get(f.state, CombatState::bTargetSpaceDifferent), 0);
        assert_eq!(
            e.get(f.state, CombatState::fLastTargetDetectedTimeStamp),
            minus_max
        );
        // `006a7f50` gives 0, not `-FLT_MAX`: the firing stamp is reset.
        assert_eq!(
            e.get(f.state, CombatState::fFiringStartedTimeStamp),
            minus_max
        );
        assert_eq!(calls(&log, 0x009a_0c50), vec![vec![f.state.addr(), 0]]);
        assert_eq!(e.get(f.state, CombatState::fThreatScore), 0.0);
    }

    #[test]
    fn full_update_with_a_target_in_view_records_where_it_was_seen() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 1.0);
        e.mem.set_f32(point + 8, 3.0);
        stub(&mut e, SLOTS + 0x1f4, point);
        e.set(f.state, CombatState::bTarget360LineOfSight, 1);
        e.set(f.state, CombatState::pLastTarget, f.target);
        let log = logged(&mut e, |e| fn_00997cf0(e, f.state));
        assert_eq!(
            e.get(f.state, CombatState::fLastTarget360LineOfSightTimeStamp),
            5.0f32.to_bits()
        );
        assert_eq!(e.mem.f32(f.state.addr() + 0x84), 1.0);
        assert_eq!(e.mem.f32(f.state.addr() + 0x8c), 3.0);
        // Flushed state -1, then 0 since the 360-degree stamp is fresh.
        assert_eq!(e.get(f.state, CombatState::cTargetFlushed), 0);
        // The target did not change and no detection update was asked for.
        assert!(calls(&log, 0x0098_87b0).is_empty());
        assert_eq!(e.get(f.state, CombatState::pLastTarget), f.target);
    }

    #[test]
    fn line_of_sight_stamp_check_compares_with_the_setting() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_ea98 + 4, 2.0);
        e.mem.set_f32(state.addr() + 0x98, 4.0);
        assert!(fn_00998980(&mut e, state));
        e.mem.set_f32(state.addr() + 0x98, 1.0);
        assert!(!fn_00998980(&mut e, state));
    }

    #[test]
    fn controller_flag_getters_read_their_bytes() {
        let mut e = engine();
        let controller = e.mem.alloc(0x188);
        e.mem.set_u8(controller + 0xc5, 1);
        e.mem.set_u8(controller + 0xc7, 2);
        e.mem.set_u8(controller + 0xd4, 3);
        assert_eq!(fn_00998a10(&mut e, controller), 1);
        assert_eq!(fn_009989f0(&mut e, controller), 2);
        assert_eq!(fn_00998a30(&mut e, controller), 3);
        stub(&mut e, 0x0099_28c0, 1);
        let log = logged(&mut e, |e| assert!(fn_009989d0(e, 0x50)));
        assert_eq!(calls(&log, 0x0099_28c0), vec![vec![0x50, 0x10]]);
    }

    #[test]
    fn second_update_restarts_the_better_weapon_timer_and_ends_fleeing() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_f32(&mut e, 0x011c_f9f8 + 4, 6.0);
        e.mem.set_u32(timer(f.state, 0x1d4), EXPIRED);
        e.set(f.state, CombatState::fMaxCombatWeaponDPS, 10.0);
        e.set(f.state, CombatState::fDisarmedDPS, 4.0);
        e.set(f.state, CombatState::cFleeState, 2);
        e.mem.set_f32(timer(f.state, 0x1dc) + 4, 1.0);
        let log = logged(&mut e, |e| fn_00998a50(e, f.state));
        assert_eq!(calls(&log, 0x0099_f6d0), vec![vec![f.state.addr()]]);
        assert_eq!(e.mem.f32(timer(f.state, 0x1d4) + 4), 6.0);
        assert_eq!(e.get(f.state, CombatState::fDisarmedDPS), 0.0);
        assert_eq!(e.get(f.state, CombatState::cFleeState), 0);
        // No blocked projectile: its timer is cleared.
        assert_eq!(e.mem.f32(timer(f.state, 0x1dc) + 4), 0.0);
    }

    #[test]
    fn negative_timer_delay_is_detected() {
        let mut e = engine();
        let timer: Ptr<CombatTimer> = e.new_object();
        e.set(timer, CombatTimer::fDelay, -1.0);
        assert!(fn_009990c0(&mut e, timer));
        e.set(timer, CombatTimer::fDelay, f32::NAN);
        assert!(!fn_009990c0(&mut e, timer));
    }

    #[test]
    fn small_helpers_pass_their_arguments() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00999100(e, state));
        assert_eq!(calls(&log, 0x0062_96b0), vec![vec![state.addr(), 3]]);
        stub_st0(&mut e, 0x006b_9130, 0.0);
        assert!(fn_00999120(&mut e, 0x100));
        stub_st0(&mut e, 0x006b_9130, f64::NAN);
        assert!(!fn_00999120(&mut e, 0x100));
    }

    #[test]
    fn reduced_update_detects_the_target_and_runs_the_threat_timer() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        put_f32(&mut e, 0x011c_f804 + 4, 7.0);
        e.mem.set_u32(timer(f.state, 0x20c), EXPIRED);
        let log = logged(&mut e, |e| fn_00999160(e, f.state));
        let group = e.mem.u32(f.controller + 0x80);
        let s = f.state.addr();
        assert_eq!(
            calls(&log, 0x0098_87b0),
            vec![vec![
                group,
                f.attacker,
                f.target,
                s + 0x6c,
                s + 0x94,
                s + 0x70,
                s + 0x71
            ]]
        );
        assert_eq!(calls(&log, 0x009a_1060), vec![vec![s]]);
        assert_eq!(e.mem.f32(timer(f.state, 0x20c) + 4), 7.0);
    }

    #[test]
    fn permissions_follow_their_timers() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::iCombatPermissions, 0xffff);
        e.mem.set_u32(timer(f.state, 0x1e4), EXPIRED);
        e.mem.set_u32(timer(f.state, 0x1ec), EXPIRED);
        stub(&mut e, 0x004d_ff00, 1);
        stub_st0(&mut e, 0x0049_5460, 50.0);
        let log = logged(&mut e, |e| fn_00999260(e, f.state));
        assert_eq!(e.get(f.state, CombatState::iCombatPermissions), 0x103);
        assert_eq!(calls(&log, 0x004d_ff00)[1], vec![0.5f32.to_bits()]);
        assert_eq!(e.mem.f32(timer(f.state, 0x1e4) + 4), 5.0);
        assert_eq!(e.mem.f32(timer(f.state, 0x1ec) + 4), 10.0);
    }

    #[test]
    fn inventory_update_without_items_keeps_the_unarmed_dps() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub_st0(&mut e, 0x0064_6060, 3.0);
        e.register(0x0040_4010, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64)
        });
        put_f32(&mut e, 0x011c_f660 + 4, 4.0);
        e.set(f.state, CombatState::bInventoryDirty, 1);
        let log = logged(&mut e, |e| fn_009993c0(e, f.state));
        assert_eq!(e.get(f.state, CombatState::fUnarmedDPS), 3.0);
        assert_eq!(e.get(f.state, CombatState::fMaxCombatWeaponDPS), 3.0);
        assert_eq!(
            e.get(f.state, CombatState::fMaxModifiedCombatWeaponDPS),
            3.0
        );
        assert_eq!(e.get(f.state, CombatState::iHasCombatWeapon), 0x40);
        assert_eq!(e.get(f.state, CombatState::bInventoryDirty), 0);
        assert_eq!(e.mem.f32(timer(f.state, 0x21c) + 4), 4.0);
        assert_eq!(
            calls(&log, 0x0040_4eb0)[0][1..],
            [0x2b, 1, SOURCE_FILE_NAME, 0x346]
        );
        assert_eq!(calls(&log, 0x0040_4ee0).len(), 1);
    }

    #[test]
    fn inventory_update_with_a_forced_weapon_uses_it_alone() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let acquired = object(&mut e, 0x40);
        stub(&mut e, SLOTS + 0x148, acquired);
        let extra = e.mem.alloc(4);
        e.mem.set_u32(extra, 0x77);
        stub(&mut e, 0x0055_9450, extra);
        stub(&mut e, 0x0041_8b10, 1);
        stub(&mut e, ARRAY_SIZE, 0x900);
        stub(&mut e, 0x0052_2c80, 1);
        fn_009993c0(&mut e, f.state);
        assert_eq!(e.mem.u32(f.state.addr() + 0x10), 0x900);
        assert_eq!(e.mem.f32(f.state.addr() + 0x40), 9_999_999.0);
        assert_eq!(
            e.get(f.state, CombatState::fMaxRangedWeaponDPS),
            9_999_999.0
        );
        assert_eq!(e.get(f.state, CombatState::iHasCombatWeapon), 2);
    }

    #[test]
    fn clearing_weapons_fills_and_empties() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_0099a2a0(e, state));
        assert_eq!(
            calls(&log, FILL_BYTES),
            vec![vec![state.addr() + 0xc, 0, 0x18]]
        );
        assert_eq!(calls(&log, 0x0084_54f0), vec![vec![state.addr() + 0x24, 0]]);
    }

    #[test]
    fn best_rated_entry_wins() {
        let mut e = engine();
        e.register(0x0097_ac90, |_, a| {
            st0([1.0, 4.0, 2.0, 4.0, 0.0, 0.0, 9.0][a[1] as usize])
        });
        e.register(0x0097_c150, |_, a| ret(0x100 + a[1]));
        assert_eq!(fn_0099a2d0(&mut e, 0x10), 0);
        e.register(0x0097_ac90, |_, a| {
            st0([1.0, 4.0, 2.0, 4.0, 0.0, 0.0, 0.0][a[1] as usize])
        });
        assert_eq!(fn_0099a2d0(&mut e, 0x10), 0x101);
    }

    #[test]
    fn reachability_depends_on_the_world_space() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        assert!(fn_0099a360(&mut e, f.state, 0, 5));
        e.set(f.state, CombatState::iTargetDetectionLevel, 1);
        let space = object(&mut e, 0x20);
        e.mem.set_u32(space + 0x10, 2);
        stub(&mut e, 0x004f_d3e0, space);
        stub(&mut e, SLOTS + 0x34, 7);
        assert!(!fn_0099a360(&mut e, f.state, 0, 0));
        e.mem.set_u32(space + 0x10, 3);
        assert!(fn_0099a360(&mut e, f.state, 0, 0));
        assert!(!fn_0099a3e0(&mut e, space));
    }

    #[test]
    fn forgetting_a_reference_releases_what_holds_it() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 1 { item } else { 0 })
        });
        let target_location = e.mem.alloc(0x20);
        e.mem.set_u32(target_location + 4, 0x55);
        e.set(state, CombatState::pLocationTarget, target_location);
        e.set(state, CombatState::pLastTarget, 0x55);
        let log = logged(&mut e, |e| fn_0099a420(e, state, 0x55, 1));
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![state.addr(), 1]]);
        assert_eq!(calls(&log, 0x009a_0a40).len(), 1);
        assert_eq!(e.get(state, CombatState::pLastTarget), 0);
        // Without `keep_pair`, items 3 and 4 changed (they were 7 and 0).
        e.mem.set_u32(state.addr() + 0x19c, 7);
        let log = logged(&mut e, |e| fn_0099a420(e, state, 0x66, 0));
        assert_eq!(
            calls(&log, 0x0098_1340),
            vec![vec![state.addr(), 3], vec![state.addr(), 4]]
        );
    }

    #[test]
    fn invalid_combat_items_are_released() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 2 { item } else { 0 })
        });
        stub(&mut e, 0x0043_7b90, 1);
        let log = logged(&mut e, |e| fn_0099a530(e, state));
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![state.addr(), 2]]);
        assert_eq!(calls(&log, 0x0043_7b90), vec![vec![0x55]]);
    }

    #[test]
    fn attack_stamps_set_the_firing_start_once() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        stub_st0(&mut e, 0x006a_7f50, -(f32::MAX as f64));
        fn_0099a5e0(&mut e, state, 0);
        assert_eq!(
            e.get(state, CombatState::fLastAttackTimeStamp),
            5.0f32.to_bits()
        );
        assert_eq!(
            e.get(state, CombatState::fFiringStartedTimeStamp),
            5.0f32.to_bits()
        );
        e.set(state, CombatState::fFiringStartedTimeStamp, 0);
        stub_st0(&mut e, 0x006a_7f50, 1.0);
        fn_0099a5e0(&mut e, state, 0);
        assert_eq!(e.get(state, CombatState::fFiringStartedTimeStamp), 0);
        e.set(state, CombatState::fLastAttackTimeStamp, 0);
        fn_0099a680(&mut e, state, 0);
        assert_eq!(
            e.get(state, CombatState::fLastAttackTimeStamp),
            5.0f32.to_bits()
        );
    }

    #[test]
    fn embedded_weapon_timer_poke_unless_skipped() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_0099a650(e, state, 0, 1));
        assert!(calls(&log, 0x0097_f220).is_empty());
        let log = logged(&mut e, |e| fn_0099a650(e, state, 0, 0));
        assert_eq!(calls(&log, 0x0097_f220), vec![vec![state.addr() + 0x214]]);
    }

    #[test]
    fn damage_after_a_second_lowers_confidence_and_boosts_when_fleeing() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        e.mem.set_f32(state.addr() + 0xa0, 2.0);
        e.set(state, CombatState::cFleeState, 2);
        let log = logged(&mut e, |e| fn_0099a6b0(e, state, 0, 0));
        assert_eq!(calls(&log, 0x009a_0c50), vec![vec![state.addr(), 1]]);
        assert_eq!(calls(&log, 0x009a_1280), vec![vec![state.addr(), 1]]);
        assert_eq!(e.mem.f32(state.addr() + 0xa0), 5.0);
        let log = logged(&mut e, |e| fn_0099a6b0(e, state, 0, 0));
        assert!(calls(&log, 0x009a_0c50).is_empty());
    }

    #[test]
    fn disarm_remembers_the_best_dps() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_f32(&mut e, 0x011c_e058 + 4, 3.0);
        e.set(f.state, CombatState::fMaxCombatWeaponDPS, 8.0);
        fn_0099a720(&mut e, f.state);
        // The inventory update then clears the maximum again.
        assert_eq!(e.get(f.state, CombatState::fDisarmedDPS), 8.0);
        assert_eq!(e.mem.f32(timer(f.state, 0x1d4) + 4), 3.0);
    }

    #[test]
    fn pre_activation_lets_item_two_go() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 2 { item } else { 0 })
        });
        let log = logged(&mut e, |e| {
            combat_state_pre_activate_reference(e, f.state, 0x55)
        });
        assert_eq!(calls(&log, 0x0098_1380), vec![vec![f.controller]]);
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![f.state.addr(), 2]]);
    }

    #[test]
    fn flee_door_is_remembered_while_fleeing() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0098_1990, 1);
        stub(&mut e, 0x0040_1170, 0x1c);
        stub(&mut e, 0x0084_e3a0, 0x99);
        fn_0099a7d0(&mut e, f.state, 0x10, 0, 0, 0);
        assert_eq!(e.get(f.state, CombatState::iLastFleeDoor), 0x99);
        assert_eq!(
            e.get(f.state, CombatState::fLastFleeDoorTimeStamp),
            5.0f32.to_bits()
        );
        stub(&mut e, 0x0040_1170, 0x1d);
        e.set(f.state, CombatState::iLastFleeDoor, 0);
        fn_0099a7d0(&mut e, f.state, 0x10, 0, 0, 0);
        assert_eq!(e.get(f.state, CombatState::iLastFleeDoor), 0);
    }

    #[test]
    fn usable_item_timer_takes_the_second_setting() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_fdd8 + 4, 1.0);
        put_f32(&mut e, 0x011c_f480 + 4, 2.0);
        e.mem.set_u32(state.addr() + 0x1a4, 0x30);
        let log = logged(&mut e, |e| fn_0099a840(e, state, 0x30));
        assert_eq!(
            calls(&log, TIMER_START),
            vec![vec![state.addr() + 0x1ac, 2.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, SETTING_VALUE).len(), 2);
    }

    #[test]
    fn unsafe_cover_is_marked() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let cover = e.mem.alloc(0x78);
        e.set(f.state, CombatState::pCoverLocation, cover);
        let location = e.mem.alloc(0x20);
        e.mem.set_u32(location + 0x14, 1);
        e.set(f.state, CombatState::pCombatLocation, location);
        stub(&mut e, 0x005c_3420, 0x600);
        let log = logged(&mut e, |e| fn_0099a8d0(e, f.state));
        assert_eq!(
            calls(&log, 0x0099_ccd0),
            vec![vec![f.state.addr(), cover + 0x14, 0x600, f.target, 0]]
        );
        assert_eq!(calls(&log, 0x0054_07b0), vec![vec![cover, 1]]);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![f.state.addr()]]);
        assert_eq!(calls(&log, 0x0062_96b0), vec![vec![f.state.addr(), 3]]);
    }

    #[test]
    fn failing_cover_locations_are_cleared() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        e.set(state, CombatState::pCoverLocation, 0x10);
        e.set(state, CombatState::pSecondaryCoverLocation, 0x20);
        sequence(&mut e, 0x005d_c960, vec![1, 0]);
        let log = logged(&mut e, |e| fn_0099a9a0(e, state));
        assert!(calls(&log, CLEAR_COVER_LOCATION).is_empty());
        assert_eq!(
            calls(&log, CLEAR_SECONDARY_COVER_LOCATION),
            vec![vec![state.addr(), 0]]
        );
    }

    #[test]
    fn ended_procedure_clears_its_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::pCombatLocation, 0x40);
        stub(&mut e, 0x0082_5c00, 0xf);
        let log = logged(&mut e, |e| fn_0099aa10(e, f.state, 0x77, 0xf));
        let group = e.mem.u32(f.controller + 0x80);
        assert_eq!(calls(&log, 0x0098_bd70), vec![vec![group, 0x77]]);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![f.state.addr()]]);
    }

    #[test]
    fn pathing_cover_location_copy_sets_its_vtable() {
        let mut e = engine();
        let (to, from) = (Ptr::new(e.mem.alloc(0x54)), Ptr::new(e.mem.alloc(0x54)));
        e.mem.set_u32(from.addr() + 0x44, 0xabc);
        e.mem.set_u8(from.addr() + 0x52, 4);
        let log = logged(&mut e, |e| {
            assert_eq!(
                pathing_cover_location_pathing_cover_location(e, to, from),
                to
            );
        });
        assert_eq!(e.mem.u32(to.addr()), 0x0106_cbc0);
        assert_eq!(e.mem.u32(to.addr() + 0x44), 0xabc);
        assert_eq!(e.mem.u8(to.addr() + 0x52), 4);
        assert_eq!(calls(&log, 0x006d_cc40), vec![vec![to.addr(), from.addr()]]);
    }

    #[test]
    fn last_seen_location_is_valid_for_two_minutes() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<NiPoint3> = e.new_object();
        e.mem.set_f32(f.state.addr() + 0x8c, 7.0);
        e.mem.set_f32(f.state.addr() + 0x98, 0.0);
        assert!(fn_0099ab50(&mut e, f.state, out));
        assert_eq!(e.get(out, NiPoint3::z), 7.0);
        stub_st0(&mut e, 0x009a_8460, 1.5);
        assert!(combat_state_get_last_seen_target_firing_location(
            &mut e, f.state, out
        ));
        assert_eq!(e.get(out, NiPoint3::z), 8.5);
        e.mem.set_f32(f.state.addr() + 0x98, -200.0);
        assert!(!fn_0099ab50(&mut e, f.state, out));
    }

    #[test]
    fn find_combat_location_reuses_a_planned_one_or_sets_flags() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x005e_3fc0, 0x700);
        stub(&mut e, 0x0098_0700, 1);
        let log = logged(&mut e, |e| {
            assert!(combat_state_find_combat_location(e, f.state, 3))
        });
        assert_eq!(calls(&log, 0x0097_9f70), vec![vec![0x700, 3]]);

        stub(&mut e, 0x0098_0700, 0);
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x006d_3150, |_, a| ret(a[0]));
        stub(&mut e, 0x0043_90c0, 1);
        let log = logged(&mut e, |e| {
            assert!(combat_state_find_combat_location(e, f.state, 0xe))
        });
        let planned = calls(&log, 0x0099_5980);
        assert_eq!(planned.len(), 1);
        let location = planned[0][2];
        assert_eq!(planned[0][1], 0xe);
        assert_eq!(e.mem.u32(location + 0x1c), 1);
        assert_eq!(calls(&log, 0x0041_fd00), vec![vec![location, 0]]);
        assert_eq!(calls(&log, 0x006d_3370).len(), 1);

        // `0097bd00` gives `FLT_MAX`: no search.
        stub_st0(&mut e, 0x0097_bd00, f32::MAX as f64);
        let log = logged(&mut e, |e| {
            assert!(!combat_state_find_combat_location(e, f.state, 0xe))
        });
        assert!(calls(&log, 0x0040_1000).is_empty());
    }

    #[test]
    fn location_flags_and_space() {
        let mut e = engine();
        let location: Ptr<CombatLocation> = e.new_object();
        fn_0099b540(&mut e, location, 6, 1);
        fn_0099b540(&mut e, location, 2, 0);
        assert_eq!(e.get(location, CombatLocation::iFlags), 4);
        stub(&mut e, 0x0084_e3a0, 0x33);
        assert_eq!(fn_0099b580(&mut e, location), 0x33);
        let log = logged(&mut e, |e| fn_0099b520(e, location, 0x44));
        assert_eq!(calls(&log, 0x0041_fd00), vec![vec![location.addr(), 0x44]]);
        e.set(location, CombatLocation::pRef, 9);
        let log = logged(&mut e, |e| fn_0099b8b0(e, location, 0x500, 0x44));
        assert_eq!(
            calls(&log, 0x0098_9010),
            vec![vec![location.addr(), 0x500, 0x44]]
        );
        assert_eq!(e.get(location, CombatLocation::pRef), 0);
    }

    #[test]
    fn grenade_spot_from_the_group_location() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<CombatLocation> = e.new_object();
        assert!(!fn_0099b5a0(&mut e, f.state, out, 0, 0));
        stub(&mut e, 0x0052_5a90, 0x300);
        stub_st0(&mut e, 0x009a_7f00, 1000.0);
        stub(&mut e, 0x0098_90e0, 1);
        stub_st0(&mut e, 0x0059_5c80, 100_000.0);
        stub(&mut e, 0x009a_7670, 1);
        let arc = e.mem.alloc(4);
        let log = logged(&mut e, |e| {
            assert!(fn_0099b5a0(e, f.state, out, arc, 0x200))
        });
        let set = calls(&log, 0x0098_9010);
        assert_eq!(set.len(), 1);
        assert_eq!((set[0][0], set[0][2]), (out.addr(), 0));
        let arc_check = calls(&log, 0x009a_7670);
        assert_eq!(arc_check[0][..3], [f.attacker, f.target, 0x300]);
        assert_eq!(arc_check[0][12], arc);
        // Too close: no spot.
        stub_st0(&mut e, 0x0059_5c80, 100.0);
        assert!(!fn_0099b5a0(&mut e, f.state, out, arc, 0x200));
    }

    #[test]
    fn cover_search_builds_the_request_when_no_cover_is_kept() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let info: Ptr<CombatCoverSearchInfo> = e.new_object();
        let cover = e.mem.alloc(0x78);
        let point = e.mem.alloc(12);
        stub(&mut e, SLOTS + 0x1f4, point);
        stub_st0(&mut e, 0x0040_ebd0, 1500.0);
        stub_st0(&mut e, 0x004a_7bd0, 300.0);
        let log = logged(&mut e, |e| {
            assert!(!fn_0099b8e0(e, f.state, 0x400, info, cover, 0x900, 2))
        });
        assert_eq!(e.mem.u32(cover + 4), 2);
        assert_eq!(e.get(info, CombatCoverSearchInfo::fRadius), 300.0);
        assert_eq!(
            e.get(info, CombatCoverSearchInfo::iPreviousCoverLocationKey),
            0
        );
        assert_eq!(
            e.get(info, CombatCoverSearchInfo::fTimeStamp),
            5.0f32.to_bits()
        );
        let request = calls(&log, 0x006e_4080)[0][0];
        assert_eq!(
            calls(&log, 0x006e_4320),
            vec![vec![request, 300.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x006e_4340),
            vec![vec![request, 512.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, 0x0040_30d0), vec![vec![request, 200]]);
        assert_eq!(calls(&log, 0x0048_3710).len(), 2);
        assert_eq!(calls(&log, 0x006d_ad70), vec![vec![request]]);
        // No target: nothing at all.
        e.mem.set_u32(f.controller + 0xc0, 0);
        assert!(!fn_0099b8e0(&mut e, f.state, 0x400, info, cover, 0x900, 2));
    }

    #[test]
    fn cover_height_and_lock() {
        let mut e = engine();
        let cover = e.mem.alloc(0x54);
        e.mem.set_u8(cover + 0x4c, 2);
        assert_eq!(fn_0099c1f0(&mut e, cover), 2);
        let log = logged(&mut e, |e| fn_0099c210(e, 0x100));
        assert_eq!(calls(&log, 0x00aa_6060), vec![vec![0x114]]);
    }

    #[test]
    fn cover_check_reports_its_reasons() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let state = e.mem.alloc(4);
        let distance = e.mem.alloc(4);
        let cover_location = e.mem.alloc(0x54);
        let reservation = e.mem.alloc(8);
        let check = |key| CoverCheck {
            cover_location,
            out: 0x900,
            reservation,
            search_point: 0x800,
            key,
            current_key: 0,
            kind: 2,
            attacker: f.attacker,
            target: f.target,
            minimum_distance: 0.0,
            maximum_distance: 100.0,
            state,
            distance,
            flag_006e43e0: 0,
        };
        stub(&mut e, 0x0098_2be0, 1);
        assert!(!fn_0099c230(&mut e, f.state, check(5)));
        assert_eq!(e.mem.u32(state), 2);
        stub(&mut e, 0x0098_2be0, 0);
        stub(&mut e, 0x009a_0d70, 1);
        assert!(!fn_0099c230(&mut e, f.state, check(5)));
        assert_eq!(e.mem.u32(state), 3);
        stub(&mut e, 0x009a_0d70, 0);
        let log = logged(&mut e, |e| assert!(!fn_0099c230(e, f.state, check(5))));
        assert_eq!(e.mem.u32(state), 9);
        let reserve = calls(&log, 0x0098_35d0);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][1..4], [5, cover_location, f.controller]);
    }

    #[test]
    fn group_member_blocks_the_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x005a_4320, 2);
        let member = object(&mut e, 0x40);
        e.register_double(0x0096_8630, move |_, a| {
            ret(if a[1] == 0 { 0x66 } else { member })
        });
        let point = e.mem.alloc(12);
        stub(&mut e, SLOTS + 0x1f4, point);
        stub(&mut e, 0x0099_cc60, 1);
        let log = logged(&mut e, |e| assert!(fn_0099ca00(e, f.state, 0x700, 0x66)));
        let tests = calls(&log, 0x0099_cc60);
        assert_eq!(tests.len(), 1);
        assert_eq!((tests[0][0], tests[0][2]), (0x700, 0));
        stub(&mut e, 0x0099_cc60, 0);
        assert!(!fn_0099ca00(&mut e, f.state, 0x700, 0x66));
    }

    #[test]
    fn any_blocking_point_counts() {
        let mut e = engine();
        stub(&mut e, ARRAY_SIZE, 2);
        let point = e.mem.alloc(12);
        e.mem.set_u32(point, 0x42);
        stub(&mut e, 0x006a_7af0, point);
        sequence(&mut e, 0x0099_cc60, vec![0, 1]);
        let log = logged(&mut e, |e| assert!(fn_0099cb00(e, 0x700, 0x800)));
        assert_eq!(calls(&log, 0x0099_cc60).len(), 2);
        stub(&mut e, 0x0099_cc60, 0);
        assert!(!fn_0099cb00(&mut e, 0x700, 0x800));
    }

    #[test]
    fn forwarder_fn_009a3280_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_009052C0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009a_3280, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_009052C0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
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
    fn fn_009a3410_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3410, ARRAY_VTABLE_0108E068, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a3460_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3460, ARRAY_VTABLE_0108E07C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a35f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_35f0, ARRAY_VTABLE_0108E090, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a3810_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3810, ARRAY_VTABLE_0108E0A4, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_009a3830_compares_the_words_the_pointers_point_to() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0x1234_5678);
        e.mem.set_u32(b, 0x1234_5678);
        assert!(e.call(0x009a_3830, &args![a, b]).bool());
        e.mem.set_u32(b, 0x1234_5679);
        assert!(!e.call(0x009a_3830, &args![a, b]).bool());
        // The same pointer twice is trivially equal.
        assert!(e.call(0x009a_3830, &args![a, a]).bool());
    }
}
