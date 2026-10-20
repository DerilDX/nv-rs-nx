//! `fallout/ai/combat/combatmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00993bf0` calls on the member (`00993cd0`).
const CALLEE_00993CD0: u32 = 0x0099_3cd0;

// Translated from 00993bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<Actor*,__int64>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00993cd0` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00993bf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00993CD0, &args![this.byte_add(0xc)]).ptr()
}

use crate::types::NiTArray;

/// The vtables of this unit's container templates (RTTI names):
/// `NiTArray<CombatGroup*,NiTMallocInterface<..>>` (`0108db4c`),
/// `NiTPrimitiveArray<CombatGroup*>` (`0108db54`), `NiTMap<Actor*,__int64>`
/// (`0108db2c`) and its `NiTMapBase<DFALL<..>,Actor*,__int64>` (`0108db5c`).
const COMBAT_GROUP_ARRAY_VTABLE: u32 = 0x0108_db4c;
const COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0108_db54;
const ACTOR_MAP_VTABLE: u32 = 0x0108_db2c;
const ACTOR_MAP_BASE_VTABLE: u32 = 0x0108_db5c;
/// `0096afc0` (`cdecl`, one word: a count): allocates the array's elements.
const ARRAY_ALLOCATE: u32 = 0x0096_afc0;
/// `004ede70` (`cdecl`, one word): frees an array's elements.
const ARRAY_FREE: u32 = 0x004e_de70;
/// `00aa1070` (`cdecl`, one word: bytes) and `00aa10f0` (`cdecl`, one word):
/// the map's bucket block allocation and release.
const BUCKETS_ALLOCATE: u32 = 0x00aa_1070;
const BUCKETS_FREE: u32 = 0x00aa_10f0;
/// `00403d30` (`cdecl`: destination, value, count): forwards to `memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `00438af0` (`thiscall`): run on the map by both map destructors.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;

/// The global holding the player character (read at each use).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The global holding the object `0042ce10` tests (`thiscall`, a byte).
const OBJECT_POINTER_011DDF38: u32 = 0x011d_df38;
const OBJECT_TEST: u32 = 0x0042_ce10;
/// `00658930` (`thiscall` on the manager): the number of combat groups;
/// `00877a30` (`thiscall`, one word: an index): the address of that group's
/// pointer.
const GROUP_COUNT: u32 = 0x0065_8930;
const GROUP_SLOT: u32 = 0x0087_7a30;
/// `CombatGroup::IsTarget` (Xbox PDB name of `009865b0`; `thiscall`, one
/// word: the actor).
const GROUP_IS_TARGET: u32 = 0x0098_65b0;
/// `00990890` (`thiscall` on a group): the number of its members;
/// `00903600` (`thiscall`, one word: an index): that member.
const GROUP_MEMBER_COUNT: u32 = 0x0099_0890;
const GROUP_MEMBER: u32 = 0x0090_3600;
/// `00989090` (`thiscall` on a group: out, actor): fills a 4-byte time
/// stamp; `00435e00` (`thiscall` on it): the time since, in `ST0`.
const GROUP_TIME_STAMP: u32 = 0x0098_9090;
const TIME_SINCE: u32 = 0x0043_5e00;
/// `00403e20` (`thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `00992d90` and `009931c0` read (addresses of the setting
/// objects).
const SETTING_011CE528: u32 = 0x011c_e528;
const SETTING_011CEEC0: u32 = 0x011c_eec0;
const SETTING_011CE04C: u32 = 0x011c_e04c;
const SETTING_011CE454: u32 = 0x011c_e454;
const SETTING_011CFE5C: u32 = 0x011c_fe5c;
const SETTING_011CE72C: u32 = 0x011c_e72c;
const SETTING_011CFC14: u32 = 0x011c_fc14;

/// The value word of the setting at `setting` (`00403e20`).
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// The group pointer at index `i` of the manager's groups.
fn group_at(e: &mut Engine, this: Ptr, i: u32) -> u32 {
    let slot = e.call(GROUP_SLOT, &args![this, i]).u32();
    e.mem.u32(slot)
}

/// `00989090` into a stack time stamp, then `00435e00` on it: the time since
/// the group's stamp for `actor`, as the `double` the exe stores it as.
fn group_time_since(e: &mut Engine, group: u32, actor: u32) -> f64 {
    e.with_stack(4, |e, stamp| {
        e.call(GROUP_TIME_STAMP, &args![group, stamp, actor]);
        e.call(TIME_SINCE, &args![stamp]).f64()
    })
}

// Translated from 00993cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTArray<CombatGroup*>` constructor (unnamed in the engine map;
/// `thiscall`, `RET 8`): the vtable `0108db4c`, `m_usMaxSize` = `max_size`,
/// `m_usGrowBy` = `grow_by`, both other counts 0, and `m_pBase` from
/// `0096afc0(max_size)` when `max_size` is not zero, else null. Returns
/// `this`.
pub fn fn_00993cf0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), COMBAT_GROUP_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max_size = e.get(this, NiTArray::m_usMaxSize);
    let base = if max_size > 0 {
        e.call(ARRAY_ALLOCATE, &args![u32::from(max_size)]).u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, base);
    this
}

// Translated from 00993960 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTPrimitiveArray<CombatGroup*>` constructor (unnamed in the engine
/// map; `thiscall`, `RET 8`): the `NiTArray` constructor (`00993cf0`), then
/// the vtable `0108db54`. Returns `this`.
pub fn fn_00993960(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_00993cf0(e, this, max_size, grow_by);
    e.mem
        .set_u32(this.addr(), COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 009938b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the array's
/// `m_usESize` (`+0xc`), the number of non-null elements.
pub fn fn_009938b0(e: &mut Engine, this: Ptr<NiTArray>) -> u16 {
    e.get(this, NiTArray::m_usESize)
}

// Translated from 00993880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<CombatGroup*>` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the vtable `0108db4c`, then `004ede70` frees
/// `m_pBase`.
pub fn fn_00993880(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), COMBAT_GROUP_ARRAY_VTABLE);
    let base = e.get(this, NiTArray::m_pBase);
    e.call(ARRAY_FREE, &args![base]);
}

// Translated from 00993990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..,Actor*,__int64>` constructor (unnamed in the engine
/// map; `thiscall`, `RET 4`): the vtable `0108db5c`, the hash size at `+4`,
/// the count at `+0xc` cleared, and a bucket block of `hash_size * 4`
/// bytes (`00aa1070`) at `+8`, zeroed (`00403d30`). Returns `this`.
pub fn fn_00993990(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, ACTOR_MAP_BASE_VTABLE);
    e.mem.set_u32(me.wrapping_add(4), hash_size);
    e.mem.set_u32(me.wrapping_add(0xc), 0);
    let bytes = e.mem.u32(me.wrapping_add(4)).wrapping_shl(2);
    let buckets = e.call(BUCKETS_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(me.wrapping_add(8), buckets);
    let bytes = e.mem.u32(me.wrapping_add(4)).wrapping_shl(2);
    let buckets = e.mem.u32(me.wrapping_add(8));
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 00993820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<Actor*,__int64>` constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the base constructor (`00993990`), then the vtable
/// `0108db2c`. Returns `this`.
pub fn fn_00993820(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_00993990(e, this, hash_size);
    e.mem.set_u32(this.addr(), ACTOR_MAP_VTABLE);
    this
}

// Translated from 00993bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..,Actor*,__int64>` destructor (unnamed in the engine
/// map; `thiscall`, plain `RET`): the vtable `0108db5c`, `00438af0` on the
/// map, then `00aa10f0` frees the bucket block at `+8`.
pub fn fn_00993bc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(8));
    e.call(BUCKETS_FREE, &args![buckets]);
}

// Translated from 00993b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00993b60 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<Actor*,__int64>` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the vtable `0108db2c`, `00438af0` on the map,
/// then the base destructor (`00993bc0`).
pub fn fn_00993b60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00993bc0(e, this);
}

// Translated from 00993850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*,__int64>::scalar deleting destructor` (Xbox PDB name in
/// the engine map; `thiscall`, `RET 4`): the destructor (`00993b60`), then
/// `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_actor_int64_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00993b60(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00992920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the manager and an
/// actor). Nothing when `0042ce10` answers true for the object at
/// `011ddf38`. Otherwise every combat group gets `00987820` with the actor;
/// when the actor's virtual slot `+0x100` answers true, `00992a30` (on the
/// manager) and `009a41e0` (on the object at `+0x1c`) get it too; last
/// `009a51e0` on the object at `+0x14` with the actor.
pub fn fn_00992920(e: &mut Engine, this: Ptr, actor: Ptr) {
    let me = this.addr();
    let object = e.mem.u32(OBJECT_POINTER_011DDF38);
    if e.call(OBJECT_TEST, &args![object]).bool() {
        return;
    }
    let mut chosen = 0u32;
    if e.vcall(actor.addr(), 0x100, &[]).bool() {
        chosen = actor.addr();
    }
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            e.call(0x0098_7820, &args![group, actor]);
        }
        i = i.wrapping_add(1);
    }
    if chosen != 0 {
        e.call(0x0099_2a30, &args![this, chosen]);
        let target_map = e.mem.u32(me.wrapping_add(0x1c));
        e.call(0x009a_41e0, &args![target_map, chosen]);
    }
    let threat_map = e.mem.u32(me.wrapping_add(0x14));
    e.call(0x009a_51e0, &args![threat_map, actor]);
}

// Translated from 009931c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::GetCombatantCount` (Xbox PDB; `thiscall`, `RET 8`):
/// sums the member counts (`00990890`) of the combat groups that have
/// `target` as a target (`CombatGroup::IsTarget`; every group when `target`
/// is null), one less for the group the player's virtual slot `+0x3f8`
/// answers (when its count is not zero). With a target and `recent` not
/// null, `*recent` (cleared first) also gets the counts of the groups whose
/// time since their stamp for the target (`00989090`, `00435e00`) is above
/// the setting at `011ce528`.
pub fn combat_manager_get_combatant_count(
    e: &mut Engine,
    this: Ptr,
    target: Ptr,
    recent: Ptr,
) -> u32 {
    let mut total = 0u32;
    if !recent.is_null() {
        e.mem.set_u32(recent.addr(), 0);
    }
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 && (target.is_null() || e.call(GROUP_IS_TARGET, &args![group, target]).bool())
        {
            let mut count = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
            if count != 0 {
                let player = e.mem.u32(PLAYER_POINTER);
                if e.vcall(player, 0x3f8, &[]).u32() == group {
                    count = count.wrapping_sub(1);
                }
            }
            total = total.wrapping_add(count);
            if !target.is_null() && !recent.is_null() {
                let since = group_time_since(e, group, target.addr());
                let limit = x87_load32(setting_bits(e, SETTING_011CE528));
                if limit < since {
                    let sum = e.mem.u32(recent.addr()).wrapping_add(count);
                    e.mem.set_u32(recent.addr(), sum);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    total
}

// Translated from 009932d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): writes to the
/// save buffer the word at `+0x34` (`00865e50`, 4 bytes); a variable-sized
/// block (`StartVariableSizedValue` .. `SaveVariableSizedValue`) with, for
/// each combat group, the words from `00559450` and `00726070` and the
/// group's own `SaveGame`, then its count; the threat map's `SaveGame`
/// (object at `+0x14`); a variable-sized block with the form id of each key
/// of the actor map at `+0x10` (walked with `004b9ba0` and `GetNext`), then
/// its count; the time stamp at `+0x20` and the timers at `+0x24` and
/// `+0x2c`.
pub fn combat_manager_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(
        0x0086_5e50,
        &args![buffer, me.wrapping_add(0x34), 4u32, 0u32],
    );
    // The frame's locals: iterator +0, key +4, value +8 (8 bytes), two
    // words +0x10 and +0x14.
    e.with_stack(0x18, |e, frame| {
        let f = frame.addr();
        let mut saved = 0u32;
        let start = e.call(0x0086_5f20, &args![buffer]).u32();
        let mut i = 0u32;
        while i < e.call(GROUP_COUNT, &args![this]).u32() {
            let group = group_at(e, this, i);
            if group != 0 {
                let first = e.call(0x0055_9450, &args![group]).u32();
                e.mem.set_u32(f + 0x10, first);
                e.call(0x0086_5e50, &args![buffer, f + 0x10, 4u32, 0u32]);
                let second = e.call(0x0072_6070, &args![group]).u32();
                e.mem.set_u32(f + 0x14, second);
                e.call(0x0086_5e50, &args![buffer, f + 0x14, 4u32, 0u32]);
                e.call(0x0098_de90, &args![group, buffer]);
                saved = saved.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        e.call(0x0086_5ff0, &args![buffer, saved, start]);
        let threat_map = e.mem.u32(me.wrapping_add(0x14));
        e.call(0x009a_59e0, &args![threat_map, buffer]);
        let mut keys = 0u32;
        let start = e.call(0x0086_5f20, &args![buffer]).u32();
        let actor_map = e.mem.u32(me.wrapping_add(0x10));
        let first = e.call(0x004b_9ba0, &args![actor_map]).u32();
        e.mem.set_u32(f, first);
        while e.mem.u32(f) != 0 {
            let actor_map = e.mem.u32(me.wrapping_add(0x10));
            e.call(0x0099_3a80, &args![actor_map, f, f + 4, f + 8]);
            let key = e.mem.u32(f + 4);
            e.call(0x0086_5df0, &args![buffer, key, 0u32]);
            keys = keys.wrapping_add(1);
        }
        e.call(0x0086_5ff0, &args![buffer, keys, start]);
    });
    e.call(0x009a_5f90, &args![me.wrapping_add(0x20), buffer]);
    e.call(0x009a_5eb0, &args![me.wrapping_add(0x24), buffer]);
    e.call(0x009a_5eb0, &args![me.wrapping_add(0x2c), buffer]);
}

/// `006a7f50` (`thiscall`): the float at the object, in `ST0`; `0108d9b0`:
/// the float whose negation marks "not set".
const READ_FLOAT: u32 = 0x006a_7f50;
const UNSET_MAGNITUDE: u32 = 0x0108_d9b0;
/// `008d7f80` (`thiscall`, a byte) and `008d7f40` (`thiscall`, one float)
/// on the timers at `+0x2c` and `+0x24`.
const TIMER_TEST: u32 = 0x008d_7f80;
const TIMER_SET: u32 = 0x008d_7f40;
/// `00910b10`, `0097fa50` (`thiscall` on the timer at `+0x24`).
const TIMER_RESET: u32 = 0x0091_0b10;
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `00435dd0` (no arguments): the current time, in `ST0`; `00435de0`
/// (`thiscall`, one float): stores it into a time stamp.
const CURRENT_TIME: u32 = 0x0043_5dd0;
const STAMP_SET: u32 = 0x0043_5de0;

/// Whether the float at `+0x20` (`006a7f50`) differs from the negated
/// float at `0108d9b0` (`FUCOMPP`; a NaN differs).
fn stamp_is_set(e: &mut Engine, this: Ptr) -> bool {
    let stamp = e
        .call(READ_FLOAT, &args![this.addr().wrapping_add(0x20)])
        .f64();
    let unset = -x87_load32(e.mem.u32(UNSET_MAGNITUDE));
    unset != stamp
}

/// `00435de0` on a stack stamp with the float `bits`: the stamp's word.
fn stamp_word(e: &mut Engine, bits: u32) -> u32 {
    e.with_stack(4, |e, stamp| {
        e.call(STAMP_SET, &args![stamp, bits]);
        e.mem.u32(stamp.addr())
    })
}

// Translated from 00992d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; the combat manager).
/// "Set" means the float at `+0x20` is not the negated float at `0108d9b0`.
/// Answers whether it is set; when the timer at `+0x2c` passes `008d7f80`
/// and `0042ce10` answers false for the object at `011ddf38`, it first
/// restarts that timer (`008d7f40`, setting `011ceec0`) and looks for a
/// reason to set it, group by group (groups that `0097ef30` marks, that
/// `005a4320` answers 0 for or that `0098a2e0` marks are passed over):
/// - a group with the player as a target, stamped (`00989090`, `00435e00`)
///   less than the setting `011ce528` ago: a reason when already set;
///   otherwise when, with `total` the sum of `00453700(00586150(..))` over
///   the members the player passes `009670f0` for and whose slot `+0x428`
///   answers non-null, `total > 0` and the setting `011ce04c` is above
///   `strength / total` (`strength`: the player's `008acbe0` with the
///   float at `01012054`);
/// - another group that `009931a0` passes: when set, a reason when
///   `00989880` with the player and the setting `011ce454` answers
///   non-zero; when not set and `00989880` with the setting `011cfe5c`
///   answers non-zero, `0098cb30(0)` is added to a running sum and, once a
///   member that passes `009670f0` has been seen, the sum above `strength`
///   times the setting `011ce72c` is a reason.
///
/// When the answer changes: becoming set restarts the timer at `+0x24`
/// (`00910b10`) and stamps `+0x20` with the current time (`00435dd0`,
/// `00435de0`); becoming unset either restarts that timer with the setting
/// `011cfc14` (`008d7f40`, when `0097fa50` answers false) or, when
/// `008d7f80` passes, stamps `+0x20` "unset" and resets the timer. It
/// answers the state after that.
pub fn fn_00992d90(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    let set = stamp_is_set(e, this);
    if !e.call(TIMER_TEST, &args![me.wrapping_add(0x2c)]).bool() {
        return set;
    }
    let object = e.mem.u32(OBJECT_POINTER_011DDF38);
    if e.call(OBJECT_TEST, &args![object]).bool() {
        return set;
    }
    let delay = x87_quiet_bits(setting_bits(e, SETTING_011CEEC0));
    e.call(TIMER_SET, &args![me.wrapping_add(0x2c), delay]);
    let mut reason = false;
    let mut sum_bits = x87_store32(0.0);
    let mut seen = false;
    let player = e.mem.u32(PLAYER_POINTER);
    let strength_argument = x87_quiet_bits(e.mem.u32(0x0101_2054));
    let strength = e.call(0x008a_cbe0, &args![player, strength_argument]).f64();
    let strength_bits = x87_store32(strength);
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            if e.call(0x0097_ef30, &args![group]).bool()
                || e.call(0x005a_4320, &args![group]).u32() == 0
                || e.call(0x0098_a2e0, &args![group]).bool()
            {
                // Straight to the next group, without the check below.
                i = i.wrapping_add(1);
                continue;
            }
            let player = e.mem.u32(PLAYER_POINTER);
            if e.call(GROUP_IS_TARGET, &args![group, player]).bool() {
                let player = e.mem.u32(PLAYER_POINTER);
                let since = group_time_since(e, group, player);
                let limit = x87_load32(setting_bits(e, SETTING_011CE528));
                if limit > since {
                    if set {
                        reason = true;
                    } else {
                        let mut total_bits = x87_store32(0.0);
                        let members = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
                        let mut j = 0u32;
                        while j < members {
                            let member = e.call(GROUP_MEMBER, &args![group, j]).u32();
                            let player = e.mem.u32(PLAYER_POINTER);
                            if e.call(0x0096_70f0, &args![player, member]).bool() {
                                let process = e.vcall(member, 0x428, &[]).u32();
                                if process != 0 {
                                    let inner = e.call(0x0058_6150, &args![process]).u32();
                                    let value = e.call(0x0045_3700, &args![inner]).f64();
                                    total_bits = x87_store32(x87_add(value, x87_m32(total_bits)));
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                        let zero: f64 = e.global(0x0101_2060);
                        if x87_load32(total_bits) > zero {
                            let ratio_bits = x87_store32(x87_div(
                                x87_load32(strength_bits),
                                x87_m32(total_bits),
                            ));
                            let limit = x87_load32(setting_bits(e, SETTING_011CE04C));
                            if limit > x87_load32(ratio_bits) {
                                reason = true;
                            }
                        }
                    }
                }
            } else if e.call(0x0099_31a0, &args![group]).bool() {
                if set {
                    let radius = x87_quiet_bits(setting_bits(e, SETTING_011CE454));
                    let player = e.mem.u32(PLAYER_POINTER);
                    if e.call(0x0098_9880, &args![group, player, radius]).u32() != 0 {
                        reason = true;
                    }
                } else {
                    let radius = x87_quiet_bits(setting_bits(e, SETTING_011CFE5C));
                    let player = e.mem.u32(PLAYER_POINTER);
                    if e.call(0x0098_9880, &args![group, player, radius]).u32() != 0 {
                        if !seen {
                            let members = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
                            let mut j = 0u32;
                            while j < members {
                                let member = e.call(GROUP_MEMBER, &args![group, j]).u32();
                                let player = e.mem.u32(PLAYER_POINTER);
                                if e.call(0x0096_70f0, &args![player, member]).bool() {
                                    seen = true;
                                    break;
                                }
                                j = j.wrapping_add(1);
                            }
                        }
                        let value = e.call(0x0098_cb30, &args![group, 0u32]).f64();
                        sum_bits = x87_store32(x87_add(value, x87_m32(sum_bits)));
                        if seen {
                            let factor = setting_bits(e, SETTING_011CE72C);
                            let product = x87_mul(x87_load32(strength_bits), x87_m32(factor));
                            if x87_load32(sum_bits) > product {
                                reason = true;
                            }
                        }
                    }
                }
            }
        }
        if reason {
            break;
        }
        i = i.wrapping_add(1);
    }
    if set != reason {
        let timer = me.wrapping_add(0x24);
        if reason {
            e.call(TIMER_RESET, &args![timer]);
            let now = e.call(CURRENT_TIME, &[]).f64();
            let word = stamp_word(e, x87_store32(now));
            e.mem.set_u32(me.wrapping_add(0x20), word);
        } else if !e.call(TIMER_RUNNING, &args![timer]).bool() {
            let delay = x87_quiet_bits(setting_bits(e, SETTING_011CFC14));
            e.call(TIMER_SET, &args![timer, delay]);
        } else if e.call(TIMER_TEST, &args![timer]).bool() {
            let unset = x87_store32(-x87_load32(e.mem.u32(UNSET_MAGNITUDE)));
            let word = stamp_word(e, unset);
            e.mem.set_u32(me.wrapping_add(0x20), word);
            e.call(TIMER_RESET, &args![timer]);
        }
    }
    stamp_is_set(e, this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_3bf0, fn_00993bf0(Ptr) -> Ptr),
        entry!(
            0x0099_3cf0,
            fn_00993cf0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x0099_3960,
            fn_00993960(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x0099_38b0, fn_009938b0(Ptr<NiTArray>) -> u16),
        entry!(0x0099_3880, fn_00993880(Ptr<NiTArray>)),
        entry!(0x0099_3990, fn_00993990(Ptr, u32) -> Ptr),
        entry!(0x0099_3820, fn_00993820(Ptr, u32) -> Ptr),
        entry!(0x0099_3bc0, fn_00993bc0(Ptr)),
        entry!(0x0099_3b60, fn_00993b60(Ptr)),
        entry!(
            0x0099_3850,
            ni_t_map_actor_int64_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0099_2920, fn_00992920(Ptr, Ptr)),
        entry!(
            0x0099_31c0,
            combat_manager_get_combatant_count(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x0099_32d0, combat_manager_save_game(Ptr, Ptr)),
        entry!(0x0099_2d90, fn_00992d90(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00993bf0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00993CD0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_3bf0, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00993CD0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn array_constructors_set_counts_and_allocate_only_when_sized() {
        let mut e = Engine::new();
        e.register(ARRAY_ALLOCATE, |_, a| (0x0070_0000 + a[0]).into_ret());
        let a: Ptr<NiTArray> = e.new_object();
        e.mem.write(a.addr(), &[0xcc; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_3cf0, &args![a, 5u32, 3u32]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_ARRAY_VTABLE);
        assert_eq!(e.get(a, NiTArray::m_usMaxSize), 5);
        assert_eq!(e.get(a, NiTArray::m_usGrowBy), 3);
        assert_eq!(e.get(a, NiTArray::m_usSize), 0);
        assert_eq!(e.get(a, NiTArray::m_usESize), 0);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0x0070_0005);
        e.call(0x0099_3960, &args![a, 0u32, 1u32]);
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_ALLOCATE), vec![vec![5]]);
    }

    #[test]
    fn array_effective_size_and_destructor() {
        let mut e = Engine::new();
        noop(&mut e, &[ARRAY_FREE]);
        let a: Ptr<NiTArray> = e.new_object();
        e.set(a, NiTArray::m_usESize, 0xbeef);
        e.set(a, NiTArray::m_pBase, 0x1234);
        assert_eq!(e.call(0x0099_38b0, &args![a]).u32(), 0xbeef);
        e.call_log = Some(vec![]);
        e.call(0x0099_3880, &args![a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_ARRAY_VTABLE);
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![0x1234]]);
    }

    #[test]
    fn map_constructors_allocate_and_zero_the_buckets() {
        let mut e = Engine::new();
        e.register(BUCKETS_ALLOCATE, |_, _| 0x0071_0000u32.into_ret());
        noop(&mut e, &[MEMSET]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_3820, &[m, 37]).u32(), m);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(m), ACTOR_MAP_VTABLE);
        assert_eq!(e.mem.u32(m + 4), 37);
        assert_eq!(e.mem.u32(m + 8), 0x0071_0000);
        assert_eq!(e.mem.u32(m + 0xc), 0);
        assert_eq!(calls_to(&log, BUCKETS_ALLOCATE), vec![vec![37 * 4]]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![0x0071_0000, 0, 37 * 4]]);
        e.call(0x0099_3990, &[m, 2]);
        assert_eq!(e.mem.u32(m), ACTOR_MAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructors_empty_free_and_delete_on_flag() {
        let mut e = Engine::new();
        noop(&mut e, &[MAP_REMOVE_ALL, BUCKETS_FREE, FREE]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 8, 0x0071_0000);
        for (flags, deletes) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0099_3850, &[m, flags]).u32(), m);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![m], vec![m]]);
            assert_eq!(calls_to(&log, BUCKETS_FREE), vec![vec![0x0071_0000]]);
            assert_eq!(calls_to(&log, FREE).len(), usize::from(deletes));
            assert_eq!(e.mem.u32(m), ACTOR_MAP_BASE_VTABLE);
        }
    }

    /// A manager with `slots` group slots (all empty; see `set_group`).
    fn manager(e: &mut Engine, slots: u32) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x40));
        let table = e.mem.alloc(4 * slots + 4);
        e.mem.set_u32(this.addr() + 0x38, table);
        e.mem.set_u32(this.addr() + 0x3c, slots);
        e.register(GROUP_COUNT, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(GROUP_SLOT, |e, a| {
            (e.mem.u32(a[0] + 0x38) + 4 * a[1]).into_ret()
        });
        this
    }

    fn set_group(e: &mut Engine, this: Ptr, i: u32, group: u32) {
        let table = e.mem.u32(this.addr() + 0x38);
        e.mem.set_u32(table + 4 * i, group);
    }

    fn object_test(e: &mut Engine, answer: bool) {
        e.map(OBJECT_POINTER_011DDF38, 4);
        e.set_global(OBJECT_POINTER_011DDF38, 0x0055_0000u32);
        if answer {
            e.register(OBJECT_TEST, |_, _| 1u32.into_ret());
        } else {
            e.register(OBJECT_TEST, |_, _| Ret::default());
        }
    }

    #[test]
    fn actor_update_reaches_every_group_and_the_maps() {
        let mut e = Engine::new();
        object_test(&mut e, false);
        let this = manager(&mut e, 3);
        set_group(&mut e, this, 0, 0x0060_0000);
        set_group(&mut e, this, 2, 0x0060_0100);
        e.mem.set_u32(this.addr() + 0x1c, 0x0061_0000);
        e.mem.set_u32(this.addr() + 0x14, 0x0062_0000);
        noop(
            &mut e,
            &[0x0098_7820, 0x0099_2a30, 0x009a_41e0, 0x009a_51e0],
        );
        let actor = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x41]);
        e.mem.set_u32(actor, 0x0090_0000);
        for answer in [1u32, 0] {
            e.register_double(0x0090_1000, move |_, _| answer.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0099_2920, &[this.addr(), actor]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x0098_7820),
                vec![vec![0x0060_0000, actor], vec![0x0060_0100, actor]]
            );
            let chosen = if answer != 0 {
                vec![vec![this.addr(), actor]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&log, 0x0099_2a30), chosen);
            assert_eq!(calls_to(&log, 0x009a_41e0).len(), usize::from(answer != 0));
            assert_eq!(calls_to(&log, 0x009a_51e0), vec![vec![0x0062_0000, actor]]);
        }
        // The object test stops everything.
        object_test(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0099_2920, &[this.addr(), actor]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x009a_51e0).is_empty());
    }

    /// Setting objects at their addresses, the value at `+4`.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        for (setting, value) in values {
            e.map(*setting, 8);
            e.mem.set_f32(*setting + 4, *value);
        }
    }

    /// The player; its slot `+0x3f8` answers `own_group`.
    fn player(e: &mut Engine, own_group: u32) -> u32 {
        let p = e.mem.alloc(8);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(p, table);
        e.mem.set_u32(table + 0x3f8, 0x0090_2000);
        e.register_double(0x0090_2000, move |_, _| own_group.into_ret());
        e.map(PLAYER_POINTER, 4);
        e.set_global(PLAYER_POINTER, p);
        p
    }

    /// A group: its member count at `+0`, "targets the given actor" at `+4`,
    /// the time since its stamp at `+8`.
    fn group(e: &mut Engine, members: u32, targets: bool, since: f32) -> u32 {
        let g = e.mem.alloc(0x10);
        e.mem.set_u32(g, members);
        e.mem.set_u32(g + 4, targets as u32);
        e.mem.set_f32(g + 8, since);
        g
    }

    fn group_doubles(e: &mut Engine) {
        e.register(GROUP_MEMBER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GROUP_IS_TARGET, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GROUP_TIME_STAMP, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        e.register(TIME_SINCE, |e, a| {
            let g = e.mem.u32(a[0]);
            Ret {
                st0: f64::from(e.mem.f32(g + 8)),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn combatant_count_sums_targeting_groups_less_the_players_own() {
        let mut e = Engine::new();
        group_doubles(&mut e);
        settings(&mut e, &[(SETTING_011CE528, 5.0)]);
        let a = group(&mut e, 3, true, 10.0);
        let b = group(&mut e, 4, true, 1.0);
        let c = group(&mut e, 7, false, 10.0);
        player(&mut e, b);
        let this = manager(&mut e, 4);
        set_group(&mut e, this, 0, a);
        set_group(&mut e, this, 2, b);
        set_group(&mut e, this, 3, c);
        let recent = e.mem.alloc(4);
        e.mem.set_u32(recent, 99);
        let target = e.mem.alloc(4);
        // With a target: a (3) + b (4 - 1); recent only a (10 > 5).
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), target, recent]).u32(), 6);
        assert_eq!(e.mem.u32(recent), 3);
        // Without a target every group counts and `recent` stays cleared.
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), 0, recent]).u32(), 13);
        assert_eq!(e.mem.u32(recent), 0);
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), 0, 0]).u32(), 13);
    }

    #[test]
    fn save_game_writes_groups_keys_and_timers_in_order() {
        let mut e = Engine::new();
        let this = manager(&mut e, 3);
        set_group(&mut e, this, 0, 0x0060_0000);
        set_group(&mut e, this, 2, 0x0060_0100);
        e.mem.set_u32(this.addr() + 0x14, 0x0062_0000);
        e.mem.set_u32(this.addr() + 0x10, 0x0063_0000);
        let buffer = 0x0064_0000u32;
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(0x0086_5e50, move |e, a| {
            w.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.register(0x0086_5f20, |_, _| 0x77u32.into_ret());
        e.register(0x0055_9450, |_, a| (a[0] + 1).into_ret());
        e.register(0x0072_6070, |_, a| (a[0] + 2).into_ret());
        noop(
            &mut e,
            &[
                0x0098_de90,
                0x0086_5ff0,
                0x009a_59e0,
                0x0086_5df0,
                0x009a_5f90,
                0x009a_5eb0,
            ],
        );
        // The actor map walk: two keys, 0x15 then 0x14.
        e.register(0x004b_9ba0, |_, _| 2u32.into_ret());
        e.register(0x0099_3a80, |e, a| {
            let left = e.mem.u32(a[1]);
            e.mem.set_u32(a[2], 0x13 + left);
            e.mem.set_u32(a[1], left - 1);
            Ret::default()
        });
        e.mem.set_u32(this.addr() + 0x34, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x0099_32d0, &[this.addr(), buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            *written.borrow(),
            vec![0xabcd, 0x0060_0001, 0x0060_0002, 0x0060_0101, 0x0060_0102]
        );
        assert_eq!(
            calls_to(&log, 0x0098_de90),
            vec![vec![0x0060_0000, buffer], vec![0x0060_0100, buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x0086_5ff0),
            vec![vec![buffer, 2, 0x77], vec![buffer, 2, 0x77]]
        );
        assert_eq!(
            calls_to(&log, 0x0086_5df0),
            vec![vec![buffer, 0x15, 0], vec![buffer, 0x14, 0]]
        );
        assert_eq!(calls_to(&log, 0x009a_59e0), vec![vec![0x0062_0000, buffer]]);
        assert_eq!(
            calls_to(&log, 0x009a_5f90),
            vec![vec![this.addr() + 0x20, buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x009a_5eb0),
            vec![
                vec![this.addr() + 0x24, buffer],
                vec![this.addr() + 0x2c, buffer]
            ]
        );
    }

    /// The engine for `00992d90` with `slots` empty group slots: the timers
    /// pass, the object test fails, the stamp at `+0x20` holds `stamp`,
    /// strength 10.
    fn combat_state(slots: u32, stamp: f32) -> (Engine, Ptr) {
        let mut e = Engine::new();
        object_test(&mut e, false);
        e.map(UNSET_MAGNITUDE, 4);
        e.mem.set_f32(UNSET_MAGNITUDE, f32::MAX);
        e.map(0x0101_2054, 4);
        e.mem.set_f32(0x0101_2054, -1.0);
        e.map(0x0101_2060, 8);
        e.set_global(0x0101_2060, 0.0f64);
        e.register(READ_FLOAT, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(TIMER_TEST, |_, _| 1u32.into_ret());
        e.register(0x008a_cbe0, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(CURRENT_TIME, |_, _| Ret {
            st0: 42.0,
            ..Ret::default()
        });
        e.register(STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        noop(&mut e, &[TIMER_SET, TIMER_RESET, 0x0097_ef30, 0x0098_a2e0]);
        e.register(0x005a_4320, |_, _| 1u32.into_ret());
        group_doubles(&mut e);
        settings(
            &mut e,
            &[
                (SETTING_011CEEC0, 2.0),
                (SETTING_011CE528, 5.0),
                (SETTING_011CE04C, 3.0),
                (SETTING_011CE454, 100.0),
                (SETTING_011CFE5C, 200.0),
                (SETTING_011CE72C, 0.5),
                (SETTING_011CFC14, 7.0),
            ],
        );
        player(&mut e, 0);
        let this = manager(&mut e, slots);
        e.mem.set_f32(this.addr() + 0x20, stamp);
        (e, this)
    }

    #[test]
    fn combat_state_skips_the_search_when_the_timer_has_not_run_out() {
        let (mut e, this) = combat_state(0, 1.0);
        e.register(TIMER_TEST, |_, _| Ret::default());
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let (mut e, this) = combat_state(0, -f32::MAX);
        e.register(TIMER_TEST, |_, _| Ret::default());
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        // A NaN differs from "unset".
        let (mut e, this) = combat_state(0, f32::NAN);
        object_test(&mut e, true);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
    }

    #[test]
    fn a_recent_target_group_with_weak_members_sets_the_state() {
        // Not set; the group targets the player and was stamped 1 < 5 ago;
        // members: one that passes with 2.0, one without a process.
        let (mut e, this) = combat_state(1, -f32::MAX);
        let g = group(&mut e, 2, true, 1.0);
        set_group(&mut e, this, 0, g);
        let member_table = e.mem.alloc(0x430);
        e.mem.set_u32(member_table + 0x428, 0x0090_3000);
        let with_process = e.mem.alloc(4);
        e.mem.set_u32(with_process, member_table);
        let without = e.mem.alloc(4);
        e.mem.set_u32(without, member_table);
        e.register_double(GROUP_MEMBER, move |_, a| {
            [with_process, without][a[1] as usize].into_ret()
        });
        e.register(0x0096_70f0, |_, _| 1u32.into_ret());
        e.register_double(0x0090_3000, move |_, a| {
            let process = if a[0] == with_process {
                0x0066_0000u32
            } else {
                0
            };
            process.into_ret()
        });
        e.register(0x0058_6150, |_, a| a[0].into_ret());
        e.register(0x0045_3700, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        // strength / total = 10 / 2 = 5, not below the setting 3: no reason.
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        // With the setting at 6 it is.
        e.mem.set_f32(SETTING_011CE04C + 4, 6.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this.addr() + 0x20), 42.0);
        assert_eq!(calls_to(&log, TIMER_RESET), vec![vec![this.addr() + 0x24]]);
        assert_eq!(
            calls_to(&log, TIMER_SET),
            vec![vec![this.addr() + 0x2c, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn a_nearby_group_keeps_the_state_set() {
        let (mut e, this) = combat_state(1, 1.0);
        let g = group(&mut e, 0, false, 0.0);
        set_group(&mut e, this, 0, g);
        e.register(0x0099_31a0, |_, _| 1u32.into_ret());
        e.register(0x0098_9880, |_, a| {
            u32::from(f32::from_bits(a[2]) == 100.0).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        // Still set, so nothing changes.
        assert!(calls_to(&log, TIMER_RESET).is_empty());
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(
            calls_to(&log, 0x0098_9880),
            vec![vec![g, player, 100.0f32.to_bits()]]
        );
    }

    #[test]
    fn losing_the_reason_restarts_or_clears_the_stamp() {
        // Set, but the only slot is empty: no reason.
        let (mut e, this) = combat_state(1, 1.0);
        e.register(TIMER_RUNNING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, TIMER_SET)[1],
            vec![this.addr() + 0x24, 7.0f32.to_bits()]
        );
        // With the timer running and passing: the stamp becomes "unset".
        e.register(TIMER_RUNNING, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this.addr() + 0x20), -f32::MAX);
        assert_eq!(calls_to(&log, TIMER_RESET), vec![vec![this.addr() + 0x24]]);
    }

    #[test]
    fn a_far_group_adds_its_strength_once_a_member_was_seen() {
        let (mut e, this) = combat_state(2, -f32::MAX);
        let g = group(&mut e, 1, false, 0.0);
        set_group(&mut e, this, 0, g);
        set_group(&mut e, this, 1, g);
        e.register(0x0099_31a0, |_, _| 1u32.into_ret());
        e.register(0x0098_9880, |_, a| {
            u32::from(f32::from_bits(a[2]) == 200.0).into_ret()
        });
        e.register(GROUP_MEMBER, |_, _| 0x0067_0000u32.into_ret());
        e.register(0x0096_70f0, |_, _| 1u32.into_ret());
        e.register(0x0098_cb30, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        // 3 is not above 10 * 0.5; 6 (second group) is.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0098_cb30), vec![vec![g, 0], vec![g, 0]]);
        // The member search runs only until one was seen.
        assert_eq!(calls_to(&log, GROUP_MEMBER).len(), 1);
    }
}
