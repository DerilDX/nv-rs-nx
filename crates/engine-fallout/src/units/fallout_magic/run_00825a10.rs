//! `fallout/magic/run_00825a10` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00825c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x14` of `this` (a plain field getter).
pub fn fn_00825c00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x14)
}

// `NonActorMagicCaster` (Xbox PDB, 0x24 bytes): a `BSExtraData` (type
// 0x32) with a `MagicCaster` base at +0xC; `pCurrentSpell` +0x18,
// `pDesiredTarget` +0x1C, `pCasterObject` +0x20 (the PC code uses the same
// offsets).

/// `NonActorMagicCaster`'s vtables (RTTI `.?AVNonActorMagicCaster@@`): the
/// `BSExtraData` one at +0 and the `MagicCaster` one at +0xC.
const NON_ACTOR_MAGIC_CASTER_VTABLE: u32 = 0x0107_be80;
const NON_ACTOR_MAGIC_CASTER_CASTER_VTABLE: u32 = 0x0107_be2c;
/// The `MagicCaster` base (Xbox PDB) at +0xC.
const MAGIC_CASTER_BASE: u32 = 0xc;
/// `pCurrentSpell` (Xbox PDB).
const CURRENT_SPELL: u32 = 0x18;
/// `pDesiredTarget` (Xbox PDB).
const DESIRED_TARGET: u32 = 0x1c;
/// `pCasterObject` (Xbox PDB).
const CASTER_OBJECT: u32 = 0x20;

/// the allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// The `BSExtraData` constructor (thiscall (data, type)).
const EXTRA_DATA_CONSTRUCT: u32 = 0x0040_ec80;
/// The `BSExtraData` destructor (thiscall).
const EXTRA_DATA_DESTROY: u32 = 0x0040_ecb0;
/// The `MagicCaster` constructor (thiscall).
const MAGIC_CASTER_CONSTRUCT: u32 = 0x0081_5300;
/// The `MagicCaster` destructor (thiscall).
const MAGIC_CASTER_DESTROY: u32 = 0x0081_53b0;
/// thiscall on the object at `011c3f2c`: a flag the destructor checks.
const DESTRUCTOR_FLAG_CHECK: u32 = 0x0042_26e0;
const DESTRUCTOR_FLAG_OBJECT: u32 = 0x011c_3f2c;
/// `TESObjectREFR::CleanUpTraps` (engine map), thiscall.
const REFERENCE_CLEAN_UP_TRAPS: u32 = 0x0057_6760;
/// thiscall on the object at `011e0e80`: the object at its +4, whose lists are walked.
const LISTS_OBJECT: u32 = 0x0071_7e50;
const LISTS_OWNER: u32 = 0x011e_0e80;
/// thiscall (lists, list): the number of entries in a list.
const LIST_COUNT: u32 = 0x005b_e5c0;
/// thiscall (lists, list): the first index walked in a list.
const LIST_FIRST_INDEX: u32 = 0x005d_e0f0;
/// thiscall (lists, index): the entry (a `MobileObject`) at an index.
const LIST_ENTRY: u32 = 0x0096_8670;
/// thiscall (actor, caster): drops the caster from an actor.
const ACTOR_DROP_CASTER: u32 = 0x008c_4f10;
/// `__RTDynamicCast` (engine map), cdecl (object, offset, source type, target type, is reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `MobileObject` and `MagicProjectile`.
const MOBILE_OBJECT_TYPE: u32 = 0x0118_4920;
const MAGIC_PROJECTILE_TYPE: u32 = 0x011a_0e88;
/// The player (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;

/// The entry's vtable slot that says it is an actor (+0x100, no words).
const SLOT_IS_ACTOR: u32 = 0x100;
/// The entry's vtable slot that says it is a magic projectile (+0x224, no words).
const SLOT_IS_MAGIC_PROJECTILE: u32 = 0x224;
/// The magic projectile's vtable slot called with the caster (+0x314).
const PROJECTILE_SLOT_DROP_CASTER: u32 = 0x314;

// Translated from 00825aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NonActorMagicCaster::_scalar_deleting_destructor_` (by the destructor
/// it calls; unnamed in the engine map): runs the destructor (`00825b60`)
/// and, when bit 0 of the flags is set, frees the object. Answers `this`.
pub fn fn_00825aa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00825b60(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00825ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00825ad0 - the C++ exception-unwinding frame (FS:[0]) that destroys the extra data base when the caster constructor throws is not translated
/// `NonActorMagicCaster::NonActorMagicCaster` (by the vtables it sets;
/// unnamed in the engine map), thiscall `(caster object)`: constructs the
/// `BSExtraData` base with type 0x32 and the `MagicCaster` base (+0xC),
/// sets both vtables, keeps the caster object (`pCasterObject`) and clears
/// `pCurrentSpell` and `pDesiredTarget`. Answers `this`.
pub fn fn_00825ad0(e: &mut Engine, this: Ptr, caster_object: u32) -> Ptr {
    let base = this.addr();
    e.call(EXTRA_DATA_CONSTRUCT, &args![base, 0x32u32]);
    e.call(MAGIC_CASTER_CONSTRUCT, &args![base + MAGIC_CASTER_BASE]);
    e.mem.set_u32(base, NON_ACTOR_MAGIC_CASTER_VTABLE);
    e.mem.set_u32(
        base + MAGIC_CASTER_BASE,
        NON_ACTOR_MAGIC_CASTER_CASTER_VTABLE,
    );
    e.mem.set_u32(base + CASTER_OBJECT, caster_object);
    e.mem.set_u32(base + CURRENT_SPELL, 0);
    e.mem.set_u32(base + DESIRED_TARGET, 0);
    this
}

// Translated from 00825b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00825b60 - the C++ exception-unwinding frame (FS:[0]) that destroys the bases when a cleanup throws is not translated
/// `NonActorMagicCaster::~NonActorMagicCaster` (by the vtables it sets;
/// unnamed in the engine map): sets both vtables; unless the object at
/// `011c3f2c` answers true (`004226e0`), cleans up the caster object's
/// traps (when there is one) and drops this caster from everything that
/// holds it (`00825c20`); then destroys the `MagicCaster` and `BSExtraData`
/// bases.
pub fn fn_00825b60(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NON_ACTOR_MAGIC_CASTER_VTABLE);
    e.mem.set_u32(
        base + MAGIC_CASTER_BASE,
        NON_ACTOR_MAGIC_CASTER_CASTER_VTABLE,
    );
    let owner: u32 = e.global(DESTRUCTOR_FLAG_OBJECT);
    if !e.call(DESTRUCTOR_FLAG_CHECK, &args![owner]).bool() {
        let caster_object = e.mem.u32(base + CASTER_OBJECT);
        if caster_object != 0 {
            e.call(REFERENCE_CLEAN_UP_TRAPS, &args![caster_object]);
        }
        fn_00825c20(e, this);
    }
    e.call(MAGIC_CASTER_DESTROY, &args![base + MAGIC_CASTER_BASE]);
    e.call(EXTRA_DATA_DESTROY, &args![base]);
}

// Translated from 00825c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops this caster (its `MagicCaster` base, +0xC, or 0 for a null
/// `this`) from everything that may hold it (unnamed in the engine map):
/// the entries of list 0 (from index 0) and list 1 (from the index
/// `005de0f0` gives) of the object `00717e50` answers for `011e0e80`; an
/// entry that is an actor (vtable +0x100) drops it through `008c4f10`, an
/// entry that is a magic projectile (vtable +0x224, then a dynamic cast
/// from `MobileObject`) through its vtable +0x314; then the player, when
/// there is one, through `008c4f10`.
pub fn fn_00825c20(e: &mut Engine, this: Ptr) {
    let lists = e.call(LISTS_OBJECT, &args![LISTS_OWNER]).u32();
    let mut index = 0u32;
    while index < e.call(LIST_COUNT, &args![lists, 0u32]).u32() {
        drop_from_entry(e, this, lists, index);
        index = index.wrapping_add(1);
    }
    let mut index = e.call(LIST_FIRST_INDEX, &args![lists, 1u32]).u32();
    while index < e.call(LIST_COUNT, &args![lists, 1u32]).u32() {
        drop_from_entry(e, this, lists, index);
        index = index.wrapping_add(1);
    }
    let player: u32 = e.global(PLAYER);
    if player != 0 {
        let caster = caster_base(this);
        let player: u32 = e.global(PLAYER);
        e.call(ACTOR_DROP_CASTER, &args![player, caster]);
    }
}

/// The `MagicCaster` base of `this` (+0xC), 0 for a null `this`.
fn caster_base(this: Ptr) -> u32 {
    if this.addr() != 0 {
        this.addr() + MAGIC_CASTER_BASE
    } else {
        0
    }
}

/// One entry of [`fn_00825c20`]'s loops.
fn drop_from_entry(e: &mut Engine, this: Ptr, lists: u32, index: u32) {
    let entry = e.call(LIST_ENTRY, &args![lists, index]).u32();
    let actor = if entry != 0 && e.vcall(entry, SLOT_IS_ACTOR, &args![]).bool() {
        entry
    } else {
        0
    };
    let projectile = if entry != 0 && e.vcall(entry, SLOT_IS_MAGIC_PROJECTILE, &args![]).bool() {
        e.call(
            DYNAMIC_CAST,
            &args![entry, 0u32, MOBILE_OBJECT_TYPE, MAGIC_PROJECTILE_TYPE, 0u32],
        )
        .u32()
    } else {
        0
    };
    if actor != 0 {
        e.call(ACTOR_DROP_CASTER, &args![actor, caster_base(this)]);
    } else if projectile != 0 {
        e.vcall(
            projectile,
            PROJECTILE_SLOT_DROP_CASTER,
            &args![caster_base(this)],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00825c00, fn_00825c00(Ptr) -> u32),
        entry!(0x00825aa0, fn_00825aa0(Ptr, u32) -> Ptr),
        entry!(0x00825ad0, fn_00825ad0(Ptr, u32) -> Ptr),
        entry!(0x00825b60, fn_00825b60(Ptr)),
        entry!(0x00825c20, fn_00825c20(Ptr)),
    ]
}

#[cfg(test)]
mod caster_tests {
    use super::*;

    const IS_ACTOR_DOUBLE: u32 = 0xdead_2000;
    const IS_PROJECTILE_DOUBLE: u32 = 0xdead_2010;
    const PROJECTILE_DROP_DOUBLE: u32 = 0xdead_2020;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    #[test]
    fn constructor_builds_both_bases() {
        let mut e = Engine::new();
        quiet(&mut e, &[EXTRA_DATA_CONSTRUCT, MAGIC_CASTER_CONSTRUCT]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + CURRENT_SPELL, 5);
        e.mem.set_u32(this + DESIRED_TARGET, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_5ad0, &args![this, 0x700u32]).u32(), this);
        assert_eq!(calls(&e, EXTRA_DATA_CONSTRUCT), vec![vec![this, 0x32]]);
        assert_eq!(calls(&e, MAGIC_CASTER_CONSTRUCT), vec![vec![this + 0xc]]);
        assert_eq!(e.mem.u32(this), NON_ACTOR_MAGIC_CASTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), NON_ACTOR_MAGIC_CASTER_CASTER_VTABLE);
        assert_eq!(e.mem.u32(this + CASTER_OBJECT), 0x700);
        assert_eq!(e.mem.u32(this + CURRENT_SPELL), 0);
        assert_eq!(e.mem.u32(this + DESIRED_TARGET), 0);
    }

    /// An entry whose two type slots answer `actor` and `projectile`.
    fn entry_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x320);
        e.mem.set_u32(vtable + SLOT_IS_ACTOR, IS_ACTOR_DOUBLE);
        e.mem
            .set_u32(vtable + SLOT_IS_MAGIC_PROJECTILE, IS_PROJECTILE_DOUBLE);
        e.mem
            .set_u32(vtable + PROJECTILE_SLOT_DROP_CASTER, PROJECTILE_DROP_DOUBLE);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, vtable);
        entry
    }

    #[test]
    fn deleting_destructor_drops_the_caster_everywhere() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        quiet(
            &mut e,
            &[
                DESTRUCTOR_FLAG_CHECK,
                REFERENCE_CLEAN_UP_TRAPS,
                MAGIC_CASTER_DESTROY,
                EXTRA_DATA_DESTROY,
                FREE,
                ACTOR_DROP_CASTER,
                PROJECTILE_DROP_DOUBLE,
            ],
        );
        let actor = entry_object(&mut e);
        let projectile = entry_object(&mut e);
        e.register(LISTS_OBJECT, |_, _| ret(0x4000));
        // Both lists have two entries: index 0 is the actor, index 1 the
        // projectile; list 1 is walked from index 1.
        e.register(LIST_COUNT, |_, _| ret(2));
        e.register(LIST_FIRST_INDEX, |_, _| ret(1));
        e.register_double(LIST_ENTRY, move |_, a| {
            ret(if a[1] == 0 { actor } else { projectile })
        });
        e.register_double(IS_ACTOR_DOUBLE, move |_, a| ret(u32::from(a[0] == actor)));
        e.register_double(IS_PROJECTILE_DOUBLE, move |_, a| {
            ret(u32::from(a[0] == projectile))
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.set_global(PLAYER, 0x9000u32);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + CASTER_OBJECT, 0x700);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_5aa0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NON_ACTOR_MAGIC_CASTER_VTABLE);
        assert_eq!(calls(&e, REFERENCE_CLEAN_UP_TRAPS), vec![vec![0x700]]);
        // Index 1 of list 0 is the projectile too; list 1 starts at 1.
        assert_eq!(
            calls(&e, ACTOR_DROP_CASTER),
            vec![vec![actor, this + 0xc], vec![0x9000, this + 0xc]]
        );
        assert_eq!(
            calls(&e, PROJECTILE_DROP_DOUBLE),
            vec![vec![projectile, this + 0xc], vec![projectile, this + 0xc]]
        );
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![projectile, 0, MOBILE_OBJECT_TYPE, MAGIC_PROJECTILE_TYPE, 0]
        );
        assert_eq!(calls(&e, MAGIC_CASTER_DESTROY), vec![vec![this + 0xc]]);
        assert_eq!(calls(&e, EXTRA_DATA_DESTROY), vec![vec![this]]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
        // When the flag is set, nothing is dropped.
        e.register(DESTRUCTOR_FLAG_CHECK, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0082_5aa0, &args![this, 0u32]);
        assert!(calls(&e, LISTS_OBJECT).is_empty());
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(calls(&e, EXTRA_DATA_DESTROY).len(), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0x14() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 0x14, 0x1234_5678);
        assert_eq!(e.call(0x0082_5c00, &args![object]).u32(), 0x1234_5678);
    }
}
