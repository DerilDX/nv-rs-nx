//! `fallout shared/packagedatauseweapon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067d9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01068a88` (RTTI `.?AVUseWeaponActorPackageData@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `0067d970`. One word, plain `RET`, returns nothing.
pub fn fn_0067d9c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_8a88);
    e.call(0x0067_d970, &args![this]);
}

layout! {
    /// `TESUseWeaponPackageData` (Xbox PDB), 0x24 bytes (the same on PC).
    pub struct TESUseWeaponPackageData: 0x24 {
        /// `pAttackTarget` (Xbox PDB): `PackageTarget*`.
        0x04 pAttackTarget: u32,
        /// `pTargetLocation` (Xbox PDB): `PackageLocation*`.
        0x08 pTargetLocation: u32,
        /// `bAlwaysHit` (Xbox PDB).
        0x0C bAlwaysHit: u8,
        /// `bDoNoDamage` (Xbox PDB).
        0x0D bDoNoDamage: u8,
        /// `bCrouch` (Xbox PDB).
        0x0E bCrouch: u8,
        /// `bHoldFire` (Xbox PDB).
        0x0F bHoldFire: u8,
        /// `bVolleyFire` (Xbox PDB).
        0x10 bVolleyFire: u8,
        /// `bRepeatFire` (Xbox PDB).
        0x11 bRepeatFire: u8,
        /// `sBurstCount` (Xbox PDB).
        0x12 sBurstCount: u16,
        /// `sVolleyShotsMin` (Xbox PDB).
        0x14 sVolleyShotsMin: u16,
        /// `sVolleyShotsMax` (Xbox PDB).
        0x16 sVolleyShotsMax: u16,
        /// `fVolleyWaitMin` (Xbox PDB), as stored bits.
        0x18 fVolleyWaitMin: u32,
        /// `fVolleyWaitMax` (Xbox PDB), as stored bits.
        0x1C fVolleyWaitMax: u32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP*` (a form id until
        /// `InitItem`).
        0x20 pWeapon: u32,
    }
}

layout! {
    /// `UseWeaponActorPackageData` (Xbox PDB), 0x8 bytes.
    pub struct UseWeaponActorPackageData: 0x08 {
        /// `pAttackTarget` (Xbox PDB): `TESObjectREFR*`.
        0x04 pAttackTarget: u32,
    }
}

/// `TESUseWeaponPackageData`'s vtable (RTTI locator `01113fc8`).
const USE_WEAPON_DATA_VTABLE: u32 = 0x0106_8a64;
/// `UseWeaponActorPackageData`'s vtable (stored by `0067d9c0` too).
const USE_WEAPON_ACTOR_DATA_VTABLE: u32 = 0x0106_8a88;
/// The `ActorPackageData` base vtable (RTTI locator `01114094`).
const ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0106_8aa4;
/// The `TESPackageData` base constructor (`0067ad20`) and destructor
/// (`0067ad70`), plain `RET`.
const PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
const PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;
/// The `float` the default constructor puts in `fVolleyWaitMax` (`010162c0`).
const DEFAULT_VOLLEY_WAIT_MAX: u32 = 0x0101_62c0;
/// The `MemoryManager` allocation (`00401000`) and free (`00401030`)
/// wrappers, cdecl.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVTESObjectWEAP@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
/// The two forms `InitItem` compares the location reference's form with
/// (pointers at `011ca244` and `011ca248`).
const SPECIAL_FORM_A: u32 = 0x011c_a244;
const SPECIAL_FORM_B: u32 = 0x011c_a248;

// Translated from 004a60f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData::GetPackageType` (Xbox PDB): 0x10.
pub fn use_weapon_actor_package_data_get_package_type(
    _e: &mut Engine,
    _this: Ptr<UseWeaponActorPackageData>,
) -> u32 {
    0x10
}

// Translated from 0067caa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::TESUseWeaponPackageData` (Xbox PDB): the
/// `TESPackageData` base (`0067ad20`), the vtable, no target, location or
/// weapon, every flag clear but `bCrouch` and `bHoldFire`, a burst count
/// and minimum volley of 1 shot, a maximum of 6, a minimum wait of 1.0 and
/// a maximum wait of the `float` at `010162c0`. Returns `this`.
pub fn tes_use_weapon_package_data_tes_use_weapon_package_data(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
) -> Ptr<TESUseWeaponPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.set(this, TESUseWeaponPackageData::pWeapon, 0);
    e.set(this, TESUseWeaponPackageData::bAlwaysHit, 0);
    e.set(this, TESUseWeaponPackageData::bDoNoDamage, 0);
    e.set(this, TESUseWeaponPackageData::bCrouch, 1);
    e.set(this, TESUseWeaponPackageData::bHoldFire, 1);
    e.set(this, TESUseWeaponPackageData::bVolleyFire, 0);
    e.set(this, TESUseWeaponPackageData::bRepeatFire, 0);
    e.set(this, TESUseWeaponPackageData::sBurstCount, 1);
    e.set(this, TESUseWeaponPackageData::sVolleyShotsMin, 1);
    e.set(this, TESUseWeaponPackageData::sVolleyShotsMax, 6);
    // FLD1; FSTP.
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMin,
        1.0f32.to_bits(),
    );
    let wait_max = x87_quiet_bits(e.global::<u32>(DEFAULT_VOLLEY_WAIT_MAX));
    e.set(this, TESUseWeaponPackageData::fVolleyWaitMax, wait_max);
    this
}

// Translated from 0067cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`0067ccd0`), then frees `this` (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn tes_use_weapon_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    flags: u32,
) -> Ptr<TESUseWeaponPackageData> {
    tes_use_weapon_package_data_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0067cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TESUseWeaponPackageData` constructor from the record's data
/// (`PACK_USE_WEAPON_DATA_PKW3`, Xbox PDB, 0x18 bytes, passed by value as
/// six words; `RET 0x18`): the `TESPackageData` base (`0067ad20`), the
/// vtable, no target or location, then the flag bytes, the three counts,
/// the two waits (`FLD`/`FSTP`) and the weapon word copied as they are.
/// Returns `this`.
pub fn fn_0067cb70(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    flags_low: u32,
    flags_high_and_burst: u32,
    volley_shots: u32,
    wait_min: u32,
    wait_max: u32,
    weapon: u32,
) -> Ptr<TESUseWeaponPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
    let [always_hit, no_damage, crouch, hold_fire] = flags_low.to_le_bytes();
    e.set(this, TESUseWeaponPackageData::bAlwaysHit, always_hit);
    e.set(this, TESUseWeaponPackageData::bDoNoDamage, no_damage);
    e.set(this, TESUseWeaponPackageData::bCrouch, crouch);
    e.set(this, TESUseWeaponPackageData::bHoldFire, hold_fire);
    let [volley_fire, repeat_fire, _, _] = flags_high_and_burst.to_le_bytes();
    e.set(this, TESUseWeaponPackageData::bVolleyFire, volley_fire);
    e.set(this, TESUseWeaponPackageData::bRepeatFire, repeat_fire);
    let burst = (flags_high_and_burst >> 16) as u16;
    e.set(this, TESUseWeaponPackageData::sBurstCount, burst);
    e.set(
        this,
        TESUseWeaponPackageData::sVolleyShotsMin,
        volley_shots as u16,
    );
    e.set(
        this,
        TESUseWeaponPackageData::sVolleyShotsMax,
        (volley_shots >> 16) as u16,
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMin,
        x87_quiet_bits(wait_min),
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMax,
        x87_quiet_bits(wait_max),
    );
    this
}

// Translated from 0067ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067ccd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESUseWeaponPackageData::~TESUseWeaponPackageData` (Xbox PDB): stores
/// the vtable, deletes the attack target (`007b3fa0` with 1, the engine map
/// placing that folded body in `racesexmenu.cpp`) and the target location
/// (`00670b30` with 1, named `PackageLocation::~PackageLocation` by the
/// map) when there is one, clearing each pointer, then runs the
/// `TESPackageData` destructor (`0067ad70`).
pub fn tes_use_weapon_package_data_destructor(e: &mut Engine, this: Ptr<TESUseWeaponPackageData>) {
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 {
        e.call(0x007b_3fa0, &args![target, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.call(PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 0067d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067d1c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESUseWeaponPackageData::InitItem` (Xbox PDB), `package` its package
/// (`RET 4`). Without an attack target, the package's location
/// (`0055b980` on `package`) is asked for its type
/// (`PackageLocation::GetLocType`, `00678ca0`) and, for type 0, its
/// reference (`PackageLocation::GetLocReference`, `0067f390`). When that
/// reference's form (`007af430`) is one of the two forms `011ca244` and
/// `011ca248` point at, its extra data (`005d43c0`) is asked for an
/// `ExtraTarget` reference (`ExtraDataList::GetXTarget`, `00421b40`); with
/// one, a new 0x10-byte `PackageTarget` (`00401000`, constructed by
/// `0067ff70` when the allocation succeeds, else null) becomes the attack
/// target, of type 0 (`006800b0`) and with that reference (`00680110`).
/// Otherwise an attack target that exists runs its own `InitItem`
/// (`006803b0` with `package`). Finally the weapon id is looked up and cast
/// to `TESObjectWEAP` (`004839c0`, `__RTDynamicCast`) in place.
pub fn tes_use_weapon_package_data_init_item(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    package: u32,
) {
    let location = e.call(0x0055_b980, &args![package]).u32();
    let mut init_target = true;
    if e.get(this, TESUseWeaponPackageData::pAttackTarget) == 0
        && location != 0
        && e.call(0x0067_8ca0, &args![location]).u32() == 0
    {
        let reference = e.call(0x0067_f390, &args![location]).u32();
        if reference != 0 {
            let form = e.call(0x007a_f430, &args![reference]).u32();
            if form == e.global::<u32>(SPECIAL_FORM_A) || form == e.global::<u32>(SPECIAL_FORM_B) {
                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                if extra != 0 {
                    let target_ref = e.call(0x0042_1b40, &args![extra]).u32();
                    if target_ref != 0 {
                        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
                        let target = if block != 0 {
                            e.call(0x0067_ff70, &args![block]).u32()
                        } else {
                            0
                        };
                        e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
                        let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
                        e.call(0x0068_00b0, &args![target, 0u32]);
                        let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
                        e.call(0x0068_0110, &args![target, target_ref]);
                        init_target = false;
                    }
                }
            }
        }
    }
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 && init_target {
        e.call(0x0068_03b0, &args![target, package]);
    }
    let weapon_id = e.get(this, TESUseWeaponPackageData::pWeapon);
    let form = e.call(LOOKUP_FORM, &args![weapon_id]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0u32],
        )
        .u32();
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
}

// Translated from 0067d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData`'s constructor: the `ActorPackageData` base
/// (`0067d920`), the vtable `01068a88`, no attack target. Returns `this`.
pub fn fn_0067d8f0(
    e: &mut Engine,
    this: Ptr<UseWeaponActorPackageData>,
) -> Ptr<UseWeaponActorPackageData> {
    fn_0067d920(e, this.cast());
    e.mem.set_u32(this.addr(), USE_WEAPON_ACTOR_DATA_VTABLE);
    e.set(this, UseWeaponActorPackageData::pAttackTarget, 0);
    this
}

// Translated from 0067d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorPackageData` constructor: stores the base vtable `01068aa4`.
/// Returns `this`.
pub fn fn_0067d920(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ACTOR_PACKAGE_DATA_VTABLE);
    this
}

// Translated from 0067d970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorPackageData` destructor body: stores the base vtable
/// `01068aa4`.
pub fn fn_0067d970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_PACKAGE_DATA_VTABLE);
}

// Translated from 0067d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData`'s scalar deleting destructor: the
/// destructor body (`0067d9c0`), then frees `this` (`00401030`) when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_0067d990(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0067d9c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0067d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData::HandleDeletedReference` (Xbox PDB): clears
/// the attack target when it is `reference`; `RET 4`.
pub fn use_weapon_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<UseWeaponActorPackageData>,
    reference: u32,
) {
    let target = e.get(this, UseWeaponActorPackageData::pAttackTarget);
    let kept = if target == reference { 0 } else { target };
    e.set(this, UseWeaponActorPackageData::pAttackTarget, kept);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_d9c0, fn_0067d9c0(Ptr)),
        entry!(
            0x004a_60f0,
            use_weapon_actor_package_data_get_package_type(Ptr<UseWeaponActorPackageData>) -> u32
        ),
        entry!(
            0x0067_caa0,
            tes_use_weapon_package_data_tes_use_weapon_package_data(
                Ptr<TESUseWeaponPackageData>,
            )
                -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_cb40,
            tes_use_weapon_package_data_scalar_deleting_destructor(
                Ptr<TESUseWeaponPackageData>,
                u32,
            )
                -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_cb70,
            fn_0067cb70(
                Ptr<TESUseWeaponPackageData>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_ccd0,
            tes_use_weapon_package_data_destructor(Ptr<TESUseWeaponPackageData>)
        ),
        entry!(
            0x0067_d1c0,
            tes_use_weapon_package_data_init_item(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(
            0x0067_d8f0,
            fn_0067d8f0(Ptr<UseWeaponActorPackageData>) -> Ptr<UseWeaponActorPackageData>
        ),
        entry!(0x0067_d920, fn_0067d920(Ptr) -> Ptr),
        entry!(0x0067_d970, fn_0067d970(Ptr)),
        entry!(0x0067_d990, fn_0067d990(Ptr, u32) -> Ptr),
        entry!(
            0x0067_d9e0,
            use_weapon_actor_package_data_handle_deleted_reference(
                Ptr<UseWeaponActorPackageData>,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0067d9c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0067_d970, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_d9c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0067_d970)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_8a88);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_8a88,
            "the vtable was stored before the call"
        );
    }

    mod b023 {
        use super::super::*;

        fn ret(eax: u32) -> Ret {
            Ret {
                eax,
                ..Ret::default()
            }
        }

        fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        }

        fn quiet(e: &mut Engine, addrs: &[u32]) {
            for a in addrs {
                e.register(*a, |_, _| Ret::default());
            }
        }

        #[test]
        fn package_type_is_0x10() {
            let mut e = Engine::new();
            assert_eq!(e.call(0x004a_60f0, &[0x1234]).u32(), 0x10);
        }

        #[test]
        fn default_constructor_values() {
            let mut e = Engine::new();
            quiet(&mut e, &[PACKAGE_DATA_CONSTRUCT]);
            e.map(DEFAULT_VOLLEY_WAIT_MAX & !0xfff, 0x1000);
            // A signalling NaN comes out quiet (FLD/FSTP).
            e.set_global(DEFAULT_VOLLEY_WAIT_MAX, 0x7f80_0001u32);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.mem.write(this.addr(), &[0xcd; 0x24]);
            e.call_log = Some(vec![]);
            let r = e.call(0x0067_caa0, &args![this]).u32();
            assert_eq!(r, this.addr());
            assert_eq!(
                calls_to(e.call_log.as_ref().unwrap(), PACKAGE_DATA_CONSTRUCT),
                vec![vec![this.addr()]]
            );
            let t = this.addr();
            assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.mem.u32(t + 8), 0);
            assert_eq!(e.mem.bytes(t + 0xc, 6), vec![0, 0, 1, 1, 0, 0]);
            assert_eq!(e.mem.u16(t + 0x12), 1);
            assert_eq!(e.mem.u16(t + 0x14), 1);
            assert_eq!(e.mem.u16(t + 0x16), 6);
            assert_eq!(e.mem.f32(t + 0x18), 1.0);
            assert_eq!(e.mem.u32(t + 0x1c), 0x7fc0_0001);
            assert_eq!(e.mem.u32(t + 0x20), 0);
        }

        #[test]
        fn data_constructor_copies_the_record() {
            let mut e = Engine::new();
            quiet(&mut e, &[PACKAGE_DATA_CONSTRUCT]);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.mem.write(this.addr(), &[0xcd; 0x24]);
            let words = [
                this.addr(),
                0x0403_0201,
                0x0007_0605,
                0x0009_0008,
                0x7f80_0002,
                2.5f32.to_bits(),
                0x0001_5cde,
            ];
            assert_eq!(e.call(0x0067_cb70, &words).u32(), this.addr());
            let t = this.addr();
            assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.mem.u32(t + 8), 0);
            assert_eq!(e.mem.bytes(t + 0xc, 6), vec![1, 2, 3, 4, 5, 6]);
            assert_eq!(e.mem.u16(t + 0x12), 7);
            assert_eq!(e.mem.u16(t + 0x14), 8);
            assert_eq!(e.mem.u16(t + 0x16), 9);
            assert_eq!(e.mem.u32(t + 0x18), 0x7fc0_0002);
            assert_eq!(e.mem.f32(t + 0x1c), 2.5);
            assert_eq!(e.mem.u32(t + 0x20), 0x0001_5cde);
        }

        #[test]
        fn destructor_deletes_target_and_location() {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[0x007b_3fa0, 0x0067_0b30, PACKAGE_DATA_DESTRUCT, FREE],
            );
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            let t = this.addr();
            for (target, location) in [(0x111u32, 0x222u32), (0, 0)] {
                e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
                e.set(this, TESUseWeaponPackageData::pTargetLocation, location);
                e.call_log = Some(vec![]);
                e.call(0x0067_ccd0, &args![this]);
                let log = e.call_log.take().unwrap();
                let expect = |p: u32| if p != 0 { vec![vec![p, 1]] } else { vec![] };
                assert_eq!(calls_to(&log, 0x007b_3fa0), expect(target));
                assert_eq!(calls_to(&log, 0x0067_0b30), expect(location));
                assert_eq!(calls_to(&log, PACKAGE_DATA_DESTRUCT), vec![vec![t]]);
                assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
                assert_eq!(e.mem.u32(t + 4), 0);
                assert_eq!(e.mem.u32(t + 8), 0);
            }
            // The deleting destructor frees on bit 0 only.
            for (flags, frees) in [(0u32, false), (2, false), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(0x0067_cb40, &[t, flags]).u32(), t);
                let log = e.call_log.take().unwrap();
                assert_eq!(calls_to(&log, PACKAGE_DATA_DESTRUCT).len(), 1);
                assert_eq!(calls_to(&log, FREE).len(), usize::from(frees));
            }
        }

        /// Doubles for `InitItem`'s path to a new target: location 0x500
        /// of type 0, reference 0x600 whose form is `011ca248`'s, extra
        /// 0x700 with target reference 0x800, allocation 0x900.
        fn init_doubles(e: &mut Engine) {
            e.map(SPECIAL_FORM_A & !0xfff, 0x1000);
            e.set_global(SPECIAL_FORM_A, 0xa0u32);
            e.set_global(SPECIAL_FORM_B, 0xb0u32);
            e.register(0x0055_b980, |_, _| ret(0x500));
            e.register(0x0067_8ca0, |_, _| ret(0));
            e.register(0x0067_f390, |_, _| ret(0x600));
            e.register(0x007a_f430, |_, _| ret(0xb0));
            e.register(0x005d_43c0, |_, _| ret(0x700));
            e.register(0x0042_1b40, |_, _| ret(0x800));
            e.register(ALLOCATE, |_, _| ret(0x900));
            e.register(0x0067_ff70, |_, a| ret(a[0] + 1));
            quiet(e, &[0x0068_00b0, 0x0068_0110, 0x0068_03b0]);
            e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
            e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 0x2000));
        }

        #[test]
        fn init_item_makes_a_target_from_the_location_reference() {
            let mut e = Engine::new();
            init_doubles(&mut e);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.set(this, TESUseWeaponPackageData::pWeapon, 0x42);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0055_b980), vec![vec![0x0abc]]);
            assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x10]]);
            assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0x901);
            assert_eq!(calls_to(&log, 0x0068_00b0), vec![vec![0x901, 0]]);
            assert_eq!(calls_to(&log, 0x0068_0110), vec![vec![0x901, 0x800]]);
            assert!(calls_to(&log, 0x0068_03b0).is_empty());
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST),
                vec![vec![0x1042, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0]]
            );
            assert_eq!(e.get(this, TESUseWeaponPackageData::pWeapon), 0x3042);

            // A failed allocation leaves a null target, still set up.
            e.register(ALLOCATE, |_, _| ret(0));
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0067_ff70).is_empty());
            assert_eq!(calls_to(&log, 0x0068_00b0), vec![vec![0, 0]]);
        }

        #[test]
        fn init_item_otherwise_inits_an_existing_target() {
            let mut e = Engine::new();
            init_doubles(&mut e);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            // An existing target: no new one, its InitItem runs.
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0x333);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0067_8ca0).is_empty());
            assert_eq!(calls_to(&log, 0x0068_03b0), vec![vec![0x333, 0x0abc]]);
            // Each failed test on the way leaves no target and no InitItem.
            let fails: [(u32, u32); 6] = [
                (0x0055_b980, 0),
                (0x0067_8ca0, 1),
                (0x0067_f390, 0),
                (0x007a_f430, 0xc0),
                (0x005d_43c0, 0),
                (0x0042_1b40, 0),
            ];
            for (addr, value) in fails {
                init_doubles(&mut e);
                e.register_double(addr, move |_, _| ret(value));
                e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
                e.call_log = Some(vec![]);
                e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
                let log = e.call_log.take().unwrap();
                assert!(calls_to(&log, ALLOCATE).is_empty(), "{addr:08x}");
                assert!(calls_to(&log, 0x0068_03b0).is_empty());
                assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0);
            }
            // The first special form also qualifies.
            init_doubles(&mut e);
            e.register(0x007a_f430, |_, _| ret(0xa0));
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0x901);
        }

        #[test]
        fn actor_data_constructors_and_destructors() {
            let mut e = Engine::new();
            quiet(&mut e, &[FREE]);
            let this: Ptr<UseWeaponActorPackageData> = e.new_object();
            let t = this.addr();
            e.mem.set_u32(t + 4, 0xcdcd_cdcd);
            assert_eq!(e.call(0x0067_d8f0, &[t]).u32(), t);
            assert_eq!(e.mem.u32(t), USE_WEAPON_ACTOR_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.call(0x0067_d920, &[t]).u32(), t);
            assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
            e.mem.set_u32(t, 0);
            e.call(0x0067_d970, &[t]);
            assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
            for (flags, frees) in [(0u32, false), (0xfffe, false), (1, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(0x0067_d990, &[t, flags]).u32(), t);
                let log = e.call_log.take().unwrap();
                // The body stores the derived vtable, then the base one.
                assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
                assert_eq!(calls_to(&log, FREE).len(), usize::from(frees));
            }
        }

        #[test]
        fn handle_deleted_reference_clears_only_the_target() {
            let mut e = Engine::new();
            let this: Ptr<UseWeaponActorPackageData> = e.new_object();
            e.set(this, UseWeaponActorPackageData::pAttackTarget, 0x55);
            e.call(0x0067_d9e0, &args![this, 0x56u32]);
            assert_eq!(e.get(this, UseWeaponActorPackageData::pAttackTarget), 0x55);
            e.call(0x0067_d9e0, &args![this, 0x55u32]);
            assert_eq!(e.get(this, UseWeaponActorPackageData::pAttackTarget), 0);
        }
    }
}
