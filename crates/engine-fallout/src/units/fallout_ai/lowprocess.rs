//! `fallout/ai/lowprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::highprocess::LowProcess;

/// The base constructor `00913810` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_00913810: u32 = 0x0091_3810;
/// The vtable stored at `+0` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`).
const VTABLE_009137B0: u32 = 0x0108_8fd4;

/// The destructor body `00906fe0` (`thiscall`, `this` only).
const DESTRUCTOR_BODY: u32 = 0x0090_6fe0;
/// `00401030` (`cdecl`, one word): frees an object.
const FREE_OBJECT: u32 = 0x0040_1030;
/// `007df1f0` (`thiscall`): the float at `+0x20` (`fHourLastProcessed`, Xbox PDB), in ST0.
const PROCESS_GET_HOUR: u32 = 0x007d_f1f0;
/// `00693d50` (`thiscall`, `RET 4`): stores a float at `+0x20`.
const PROCESS_SET_HOUR: u32 = 0x0069_3d50;
/// `ActorPackage::MoveTo` (Xbox PDB, `008c6130`; `thiscall`, `RET 4`).
const ACTOR_PACKAGE_MOVE_TO: u32 = 0x008c_6130;
/// `0059bb30` (`thiscall`): the word at `+0x24` (`iDateLastProcessed`, Xbox PDB).
const PROCESS_GET_DATE: u32 = 0x0059_bb30;
/// `00937160` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): takes the
/// `ModifierList` (Xbox PDB `pDamageModifiers`) at its argument.
const MODIFIER_LIST_COPY: u32 = 0x0093_7160;

/// The vtable `010886e4` (`_scalar_deleting_destructor_`'s table, the `LowProcess` class)
/// that the destructor body `00906fe0` stores at `+0`.
const LOW_PROCESS_VTABLE: u32 = 0x0108_86e4;
/// `BaseProcess::ClearCurrentPackage` (Xbox PDB, `008d03a0`; `thiscall`, plain `RET`).
const CLEAR_CURRENT_PACKAGE: u32 = 0x008d_03a0;
/// `009070a0` (`thiscall`, plain `RET`, unit `lowprocess.cpp`, unnamed): a 439-byte function
/// the destructor body runs on `this` right after `ClearCurrentPackage`.
const LOW_PROCESS_RELEASE: u32 = 0x0090_70a0;
/// `00937030` (`thiscall`, plain `RET`): the destructor of the `ModifierList` at `+0x94`.
const MODIFIER_LIST_DESTRUCT: u32 = 0x0093_7030;
/// `0046ffb0` (`thiscall`, plain `RET`): the base list destructor, run on the four lists at
/// `+0x7c`, `+0x74`, `+0x6c` and `+0x5c`.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The base destructor `008d0640` (`thiscall`, plain `RET`), last in the body.
const BASE_PROCESS_DESTRUCT: u32 = 0x008d_0640;

/// `CurrentPackage` (Xbox PDB): the embedded `ActorPackage` at `+0x04`.
const PACKAGE: u32 = 0x04;
/// `iDateLastProcessed` (Xbox PDB), `+0x24`.
const DATE_LAST_PROCESSED: u32 = 0x24;
/// `fCombatDelayTimer` (Xbox PDB `CombatTimer`), `+0x38`: two words.
const COMBAT_DELAY_TIMER: u32 = 0x38;
/// `pTarget` (Xbox PDB), `+0x40`.
const TARGET: u32 = 0x40;
/// `iNumberItemsActivate` (Xbox PDB), `+0x58`.
const NUMBER_ITEMS_ACTIVATE: u32 = 0x58;
/// `pAcquireObject` (Xbox PDB), `+0x64`.
const ACQUIRE_OBJECT: u32 = 0x64;
/// `pSavedAcquireObject` (Xbox PDB), `+0x68`.
const SAVED_ACQUIRE_OBJECT: u32 = 0x68;
/// `pDamageModifiers` (Xbox PDB `ModifierList`), `+0x94`.
const DAMAGE_MODIFIERS: u32 = 0x94;
/// `fEssentialDownTimer` (Xbox PDB), `+0xa4`.
const ESSENTIAL_DOWN_TIMER: u32 = 0xa4;
/// `fDeathTime` (Xbox PDB), `+0xa8`.
const DEATH_TIME: u32 = 0xa8;
/// `fTrackedDamage` (Xbox PDB), `+0xac`.
const TRACKED_DAMAGE: u32 = 0xac;
/// Vtable slot `+0x12c`, `SetTarget` (Xbox PDB name; PC slot used by this copy).
const SET_TARGET_SLOT: u32 = 0x12c;

// Translated from 009137b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`): runs `00913810(this, argument)`, then
/// stores the vtable `01088fd4` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`) at `+0`.
/// Returns `this`.
pub fn fn_009137b0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00913810, &args![this, argument]);
    e.mem.set_u32(this.addr(), VTABLE_009137B0);
    this
}

// Translated from 00906fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`): runs the
/// destructor body `00906fe0` on `this`, then, when bit 0 of `flags` is set, frees the
/// object with `00401030` (`cdecl`, one word). Returns `this`.
pub fn low_process_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00907510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`; the level copy that
/// `BaseProcess::Copy` `008d0660` calls for the `LowProcess` level, `thiscall`, `RET 4`):
/// moves the low-process state of `source` into `this`. Takes the hour at `+0x20` through
/// `007df1f0`/`00693d50`, moves the embedded package (`ActorPackage::MoveTo`), copies the
/// flags byte, the item pointer, the combat delay timer and the activated-item count,
/// moves the acquire object (`+0x64`, and `+0x68` when it is set; the source's are
/// cleared), takes the date (`0059bb30`), sets the target through vtable slot `+0x12c`,
/// copies the damage modifiers (`00937160`) and the three floats at `+0xa4`, `+0xa8`,
/// `+0xac` (each as an x87 `FLD`/`FSTP` copy, which quiets a signalling NaN).
pub fn fn_00907510(e: &mut Engine, this: Ptr<LowProcess>, source: Ptr<LowProcess>) {
    let to = this.addr();
    let from = source.addr();
    // `FLD`/`FSTP` of the hour the callee returns in ST0, pushed as the argument.
    let hour = e.call(PROCESS_GET_HOUR, &args![from]).f32();
    e.call(PROCESS_SET_HOUR, &args![to, x87_quiet_bits(hour.to_bits())]);
    // CurrentPackage (Xbox PDB) +0x04: `source`'s package moves into `this`'s.
    e.call(
        ACTOR_PACKAGE_MOVE_TO,
        &args![from.wrapping_add(PACKAGE), to.wrapping_add(PACKAGE)],
    );
    let flags = e.get(source, LowProcess::m_uFlags);
    e.set(this, LowProcess::m_uFlags, flags);
    // pAcquireObject (Xbox PDB) +0x64: moved.
    let acquire = e.mem.u32(from.wrapping_add(ACQUIRE_OBJECT));
    e.mem.set_u32(to.wrapping_add(ACQUIRE_OBJECT), acquire);
    e.mem.set_u32(from.wrapping_add(ACQUIRE_OBJECT), 0);
    // fCombatDelayTimer (Xbox PDB) +0x38, two words.
    let timer_start = e.mem.u32(from.wrapping_add(COMBAT_DELAY_TIMER));
    let timer_delay = e.mem.u32(from.wrapping_add(COMBAT_DELAY_TIMER + 4));
    e.mem
        .set_u32(to.wrapping_add(COMBAT_DELAY_TIMER), timer_start);
    e.mem
        .set_u32(to.wrapping_add(COMBAT_DELAY_TIMER + 4), timer_delay);
    let item = e.get(source, LowProcess::pItemBeingUsed);
    e.set(this, LowProcess::pItemBeingUsed, item);
    // pSavedAcquireObject (Xbox PDB) +0x68: moved only when set.
    let saved_acquire = e.mem.u32(from.wrapping_add(SAVED_ACQUIRE_OBJECT));
    if saved_acquire != 0 {
        e.mem
            .set_u32(to.wrapping_add(SAVED_ACQUIRE_OBJECT), saved_acquire);
        e.mem.set_u32(from.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
    }
    // iDateLastProcessed (Xbox PDB) +0x24, through the getter `0059bb30`.
    let date = e.call(PROCESS_GET_DATE, &args![from]).u32();
    e.mem.set_u32(to.wrapping_add(DATE_LAST_PROCESSED), date);
    // SetTarget (Xbox PDB vtable slot +0x12c) with pTarget (+0x40).
    let target = e.mem.u32(from.wrapping_add(TARGET));
    e.vcall(to, SET_TARGET_SLOT, &args![target]);
    let items_activated = e.mem.u32(from.wrapping_add(NUMBER_ITEMS_ACTIVATE));
    e.mem
        .set_u32(to.wrapping_add(NUMBER_ITEMS_ACTIVATE), items_activated);
    // pDamageModifiers (Xbox PDB) +0x94: `00937160(this + 0x94, source + 0x94)`.
    e.call(
        MODIFIER_LIST_COPY,
        &args![
            to.wrapping_add(DAMAGE_MODIFIERS),
            from.wrapping_add(DAMAGE_MODIFIERS)
        ],
    );
    // fEssentialDownTimer, fDeathTime, fTrackedDamage (Xbox PDB) +0xa4, +0xa8, +0xac.
    for offset in [ESSENTIAL_DOWN_TIMER, DEATH_TIME, TRACKED_DAMAGE] {
        let bits = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), x87_quiet_bits(bits));
    }
}

// Translated from 00906fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906fe0 - the C++ exception unwinding states (FS:[0] frame, handler 00f16c52) are not translated
/// Unnamed in the engine map (unit `lowprocess.cpp`; the destructor body that
/// `LowProcess::_scalar_deleting_destructor_` `00906fb0` runs; `thiscall`, plain `RET`).
/// Stores the vtable `010886e4` at `+0`, then runs `BaseProcess::ClearCurrentPackage`
/// (`008d03a0`) and `009070a0` on `this`, the `ModifierList` destructor `00937030` on
/// `+0x94`, the list destructor `0046ffb0` on `+0x7c`, `+0x74`, `+0x6c` and `+0x5c`, and
/// last the base destructor `008d0640` on `this`. The disassembly passes `this` in `ECX` to
/// every call (the decompiler shows `ClearCurrentPackage` with a cookie value instead).
pub fn fn_00906fe0(e: &mut Engine, this: Ptr<LowProcess>) {
    let base = this.addr();
    e.mem.set_u32(base, LOW_PROCESS_VTABLE);
    e.call(CLEAR_CURRENT_PACKAGE, &args![base]);
    e.call(LOW_PROCESS_RELEASE, &args![base]);
    // pDamageModifiers (Xbox PDB `ModifierList`) +0x94.
    e.call(
        MODIFIER_LIST_DESTRUCT,
        &args![base.wrapping_add(DAMAGE_MODIFIERS)],
    );
    for offset in [0x7c_u32, 0x74, 0x6c, 0x5c] {
        e.call(LIST_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
    e.call(BASE_PROCESS_DESTRUCT, &args![base]);
}

/// `007b3fa0` (`thiscall`, `RET 4`): a scalar deleting destructor whose destructor is empty
/// (identical-code folding: many classes share the body); frees the object when bit 0 of
/// its argument is set.
const DELETE_EMPTY_DESTRUCTOR: u32 = 0x007b_3fa0;
/// `008256d0` (`thiscall`): whether both words at `+0` and `+4` of a list head are zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): returns `this` (here the list head, whose first word is the
/// head node's item).
const LIST_HEAD_ITEM: u32 = 0x0068_15c0;
/// `00905330` (`thiscall`, `RET 4`): removes from a `BSSimpleList` the item held in the word
/// its argument points to.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `00470470` (`thiscall`, plain `RET`): empties a list (frees its chained nodes, zeroes the
/// head item).
const LIST_CLEAR: u32 = 0x0047_0470;

/// `pGenericLocation` (Xbox PDB), `+0x44`.
const GENERIC_LOCATION: u32 = 0x44;
/// `ObjectList` (Xbox PDB `BSSimpleList<ObjectstoAcquire *>`), `+0x5c`.
const OBJECT_LIST: u32 = 0x5c;
/// `GenericLocationsList` (Xbox PDB `BSSimpleList<TESObjectREFR *>`), `+0x6c`.
const GENERIC_LOCATIONS_LIST: u32 = 0x6c;
/// `TempObjectList` (Xbox PDB `BSSimpleList<ObjectstoAcquire *>`), `+0x74`.
const TEMP_OBJECT_LIST: u32 = 0x74;
/// `TempLocList` (Xbox PDB `BSSimpleList<TESObjectREFR *>`), `+0x7c`.
const TEMP_LOC_LIST: u32 = 0x7c;
/// `pObjecttoAcquire` (Xbox PDB, typed `TESBoundObject*` there), `+0x84`.
const OBJECT_TO_ACQUIRE: u32 = 0x84;
/// The vtable slot `+0x10` the code calls with the argument 1 on the object at `+0x84`.
const OBJECT_TO_ACQUIRE_DELETE_SLOT: u32 = 0x10;

/// Takes every item off the list at `list` (`008256d0` until empty): reads the head item
/// through `006815c0` into a stack word, removes it with `00905330(list, &word)`, and frees
/// the word's value afterwards, when non-zero, with `007b3fa0(value, 1)`.
fn free_list_items(e: &mut Engine, list: u32) {
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        let item = e.mem.u32(head);
        let removed = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_REMOVE_ITEM, &args![list, slot]);
            // The code reads the stack word again after the call.
            e.mem.u32(slot.addr())
        });
        if removed != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![removed, 1u32]);
        }
    }
}

// Translated from 009070a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`; `thiscall`, plain `RET`; the destructor
/// body `00906fe0` runs it after `ClearCurrentPackage`): releases the low process's acquire
/// state. When `pAcquireObject` (`+0x64`) is set it clears `pSavedAcquireObject` (`+0x68`)
/// if that is the same object, frees the object (`007b3fa0`, flag 1) and clears `+0x64`;
/// then, when `+0x68` is still set, frees and clears it the same way. Every item of
/// `ObjectList` (`+0x5c`) is removed and freed, `GenericLocationsList` (`+0x6c`) is emptied
/// (`00470470`) and `pGenericLocation` (`+0x44`) cleared; every item of `TempObjectList`
/// (`+0x74`) is removed and freed and `TempLocList` (`+0x7c`) emptied. Last, the object at
/// `+0x84` (when set) is called through its vtable slot `+0x10` with the argument 1, and
/// `+0x84` is cleared. Field names are the Xbox PDB's; the code's own offsets match them.
pub fn fn_009070a0(e: &mut Engine, this: Ptr<LowProcess>) {
    let base = this.addr();
    let acquire = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
    if acquire != 0 {
        if e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT)) == acquire {
            e.mem.set_u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
        }
        // The code reads +0x64 again for the delete.
        let acquire = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
        if acquire != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![acquire, 1u32]);
        }
        e.mem.set_u32(base.wrapping_add(ACQUIRE_OBJECT), 0);
    }
    let saved = e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT));
    if saved != 0 {
        e.call(DELETE_EMPTY_DESTRUCTOR, &args![saved, 1u32]);
        e.mem.set_u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
    }
    free_list_items(e, base.wrapping_add(OBJECT_LIST));
    e.call(
        LIST_CLEAR,
        &args![base.wrapping_add(GENERIC_LOCATIONS_LIST)],
    );
    e.mem.set_u32(base.wrapping_add(GENERIC_LOCATION), 0);
    free_list_items(e, base.wrapping_add(TEMP_OBJECT_LIST));
    e.call(LIST_CLEAR, &args![base.wrapping_add(TEMP_LOC_LIST)]);
    let object = e.mem.u32(base.wrapping_add(OBJECT_TO_ACQUIRE));
    if object != 0 {
        e.vcall(object, OBJECT_TO_ACQUIRE_DELETE_SLOT, &args![1u32]);
    }
    e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0090_70a0, fn_009070a0(Ptr<LowProcess>)),
        entry!(0x0090_6fe0, fn_00906fe0(Ptr<LowProcess>)),
        entry!(0x0090_6fb0, low_process_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0090_7510, fn_00907510(Ptr<LowProcess>, Ptr<LowProcess>)),
        entry!(0x0091_37b0, fn_009137b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_00913810, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0091_37b0, &args![this, 0x27u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00913810),
            vec![vec![this, 0x27]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_009137B0);
    }
}

#[cfg(test)]
mod low_process_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(FREE_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0xb4);
        for (flags, frees) in [(0u32, 0usize), (1, 1), (2, 0), (3, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0090_6fb0, &args![this, flags]).u32(), this);
            assert_eq!(calls_to(&e, DESTRUCTOR_BODY), vec![vec![this]]);
            assert_eq!(calls_to(&e, FREE_OBJECT).len(), frees);
            if frees == 1 {
                assert_eq!(calls_to(&e, FREE_OBJECT), vec![vec![this]]);
            }
        }
    }

    const SET_TARGET: u32 = 0x00ee_0001;

    /// A source and a target process with doubles for every callee outside this file.
    fn copy_fixture() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(PROCESS_GET_HOUR, |_, _| 7.5f32.into_ret());
        e.register(PROCESS_SET_HOUR, |_, _| Ret::default());
        e.register(ACTOR_PACKAGE_MOVE_TO, |_, _| Ret::default());
        e.register(PROCESS_GET_DATE, |_, _| 0x0000_0abcu32.into_ret());
        e.register(MODIFIER_LIST_COPY, |_, _| Ret::default());
        e.register(SET_TARGET, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x12c / 4 + 1];
        slots[0x12c / 4] = SET_TARGET;
        e.put_vtable(0x0300_0000, &slots);
        let to = e.mem.alloc(0xb4);
        let from = e.mem.alloc(0xb4);
        e.mem.set_u32(to, 0x0300_0000);
        e.mem.set_u32(from, 0x0300_0000);
        (e, to, from)
    }

    #[test]
    fn copy_takes_the_state_of_the_other_process() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u8(from + 0x30, 0x5a);
        e.mem.set_u32(from + 0x34, 0x1111);
        e.mem.set_u32(from + 0x38, 0x2222);
        e.mem.set_u32(from + 0x3c, 0x3333);
        e.mem.set_u32(from + 0x40, 0x4444);
        e.mem.set_u32(from + 0x58, 9);
        e.mem.set_u32(from + 0x64, 0x6464);
        e.mem.set_u32(from + 0x68, 0x6868);
        e.mem.set_u32(from + 0xa4, 1.5f32.to_bits());
        e.mem.set_u32(from + 0xa8, 2.5f32.to_bits());
        e.mem.set_u32(from + 0xac, 3.5f32.to_bits());
        e.mem.set_u32(to + 0x64, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(calls_to(&e, PROCESS_GET_HOUR), vec![vec![from]]);
        assert_eq!(
            calls_to(&e, PROCESS_SET_HOUR),
            vec![vec![to, 7.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_PACKAGE_MOVE_TO),
            vec![vec![from + 4, to + 4]]
        );
        assert_eq!(calls_to(&e, PROCESS_GET_DATE), vec![vec![from]]);
        assert_eq!(calls_to(&e, SET_TARGET), vec![vec![to, 0x4444]]);
        assert_eq!(
            calls_to(&e, MODIFIER_LIST_COPY),
            vec![vec![to + 0x94, from + 0x94]]
        );
        assert_eq!(e.mem.u8(to + 0x30), 0x5a);
        assert_eq!(e.mem.u32(to + 0x24), 0xabc);
        assert_eq!(e.mem.u32(to + 0x34), 0x1111);
        assert_eq!(e.mem.u32(to + 0x38), 0x2222);
        assert_eq!(e.mem.u32(to + 0x3c), 0x3333);
        assert_eq!(e.mem.u32(to + 0x58), 9);
        assert_eq!(e.mem.u32(to + 0x64), 0x6464);
        assert_eq!(e.mem.u32(to + 0x68), 0x6868);
        assert_eq!(e.mem.u32(from + 0x64), 0);
        assert_eq!(e.mem.u32(from + 0x68), 0);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xa4)), 1.5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xa8)), 2.5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xac)), 3.5);
    }

    #[test]
    fn copy_keeps_the_saved_acquire_object_when_the_other_has_none() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u32(to + 0x68, 0x7777);
        e.mem.set_u32(from + 0x64, 0);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(e.mem.u32(to + 0x68), 0x7777);
        assert_eq!(e.mem.u32(to + 0x64), 0);
    }

    #[test]
    fn copy_quiets_signalling_nans_in_the_three_floats() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u32(from + 0xa4, 0x7fa0_0001);
        e.mem.set_u32(from + 0xa8, 0xffa0_0002);
        e.mem.set_u32(from + 0xac, 0x7fc0_0003);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(e.mem.u32(to + 0xa4), 0x7fe0_0001);
        assert_eq!(e.mem.u32(to + 0xa8), 0xffe0_0002);
        assert_eq!(e.mem.u32(to + 0xac), 0x7fc0_0003);
    }
}

#[cfg(test)]
mod destructor_body_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    #[test]
    fn stores_the_vtable_and_runs_the_destructors_in_order() {
        let mut e = Engine::new();
        let log: Log = Log::default();
        for address in [
            CLEAR_CURRENT_PACKAGE,
            LOW_PROCESS_RELEASE,
            MODIFIER_LIST_DESTRUCT,
            LIST_DESTRUCT,
            BASE_PROCESS_DESTRUCT,
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this, 0x1234_5678);
        e.call(0x0090_6fe0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0108_86e4);
        assert_eq!(
            *log.borrow(),
            [
                (CLEAR_CURRENT_PACKAGE, vec![this]),
                (LOW_PROCESS_RELEASE, vec![this]),
                (MODIFIER_LIST_DESTRUCT, vec![this + 0x94]),
                (LIST_DESTRUCT, vec![this + 0x7c]),
                (LIST_DESTRUCT, vec![this + 0x74]),
                (LIST_DESTRUCT, vec![this + 0x6c]),
                (LIST_DESTRUCT, vec![this + 0x5c]),
                (BASE_PROCESS_DESTRUCT, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const OBJECT_DELETE: u32 = 0x00ee_0010;

    /// Doubles for every callee outside this file. A list is modelled as its two head words:
    /// empty when both are zero, removal shifts the second word into the first.
    fn fixture() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let logged = |log: &Log, address: u32, words: &[u32]| {
            log.borrow_mut().push((address, words.to_vec()));
        };
        let l = log.clone();
        e.register_double(DELETE_EMPTY_DESTRUCTOR, move |_, a| {
            logged(&l, DELETE_EMPTY_DESTRUCTOR, a);
            a[0].into_ret()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        let l = log.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            logged(&l, LIST_REMOVE_ITEM, &[a[0], e.mem.u32(a[1])]);
            let next = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0], next);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let l = log.clone();
        e.register_double(LIST_CLEAR, move |_, a| {
            logged(&l, LIST_CLEAR, a);
            Ret::default()
        });
        let l = log.clone();
        e.register_double(OBJECT_DELETE, move |_, a| {
            logged(&l, OBJECT_DELETE, a);
            Ret::default()
        });
        let mut slots = vec![0u32; 5];
        slots[4] = OBJECT_DELETE;
        e.put_vtable(0x0300_0000, &slots);
        let this = e.mem.alloc(0xb4);
        (e, log, this)
    }

    #[test]
    fn frees_the_shared_acquire_object_once_and_empties_the_lists() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x44, 0x4444);
        e.mem.set_u32(this + 0x64, 0x6400);
        e.mem.set_u32(this + 0x68, 0x6400);
        // ObjectList: a null item first, then 0x5c01.
        e.mem.set_u32(this + 0x5c, 0);
        e.mem.set_u32(this + 0x60, 0x5c01);
        // TempObjectList: two items.
        e.mem.set_u32(this + 0x74, 0x7401);
        e.mem.set_u32(this + 0x78, 0x7402);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0300_0000);
        e.mem.set_u32(this + 0x84, object);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6400, 1]),
                (LIST_REMOVE_ITEM, vec![this + 0x5c, 0]),
                (LIST_REMOVE_ITEM, vec![this + 0x5c, 0x5c01]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x5c01, 1]),
                (LIST_CLEAR, vec![this + 0x6c]),
                (LIST_REMOVE_ITEM, vec![this + 0x74, 0x7401]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x7401, 1]),
                (LIST_REMOVE_ITEM, vec![this + 0x74, 0x7402]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x7402, 1]),
                (LIST_CLEAR, vec![this + 0x7c]),
                (OBJECT_DELETE, vec![object, 1]),
            ]
        );
        for offset in [0x44, 0x64, 0x68, 0x84] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
    }

    #[test]
    fn frees_both_acquire_objects_when_they_differ() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x64, 0x6400);
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6400, 1]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6800, 1]),
                (LIST_CLEAR, vec![this + 0x6c]),
                (LIST_CLEAR, vec![this + 0x7c]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }

    #[test]
    fn frees_only_the_saved_acquire_object_when_the_current_is_null() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            log.borrow()
                .iter()
                .filter(|(a, _)| *a == DELETE_EMPTY_DESTRUCTOR)
                .cloned()
                .collect::<Vec<_>>(),
            [(DELETE_EMPTY_DESTRUCTOR, vec![0x6800, 1])]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }
}
