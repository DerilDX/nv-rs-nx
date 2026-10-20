//! `fallout/ai/combat/combatcovermap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VCoverReservation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108D124: u32 = 0x0108_d124;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00983700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VCoverReservation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00983700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108D124);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `CombatCoverMap`'s vtable (`0108d0e4`, named by its RTTI locator).
const COMBAT_COVER_MAP_VTABLE: u32 = 0x0108_d0e4;
/// `NiTMap<unsigned int, ReservedCoverLocation*>`'s vtable (`0108d104`).
const NI_TMAP_VTABLE: u32 = 0x0108_d104;
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, ReservedCoverLocation*>>,
/// unsigned int, ReservedCoverLocation*>`'s vtable (`0108d138`).
const NI_TMAP_BASE_VTABLE: u32 = 0x0108_d138;
/// `NiTMapBase` fields (Xbox PDB names): `m_uiHashSize` +0x04,
/// `m_ppkHashTable` +0x08, the allocator's `m_uiCount` +0x0C.
const HASH_SIZE: u32 = 0x04;
const HASH_TABLE: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
/// `CombatCoverMap`'s pointer member at +0x10.
const COVER_MAP_POINTER: u32 = 0x10;
/// The allocator (`00aa1070`, `cdecl(size)`) and its free (`00aa10f0`,
/// `cdecl(block)`).
const MAP_ALLOCATE: u32 = 0x00aa_1070;
const MAP_FREE: u32 = 0x00aa_10f0;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMapBase::RemoveAll` of this instance (`00438af0`, `thiscall`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `GetFirstPos` (`004b9ba0`, `thiscall`) and `GetNext` (`006b7f20`,
/// `thiscall(position pointer, key pointer, value pointer)`).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// The scalar deleting destructor of a reserved cover location (`00983250`,
/// `thiscall(flags)`).
const RESERVED_LOCATION_DELETE: u32 = 0x0098_3250;
/// `NiPointer` get (`00559450`) and destructor (`0045cec0`).
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Returns the object a pointer's target refers to (`009611e0`,
/// `thiscall`).
const POINTER_TARGET_OWNER: u32 = 0x0096_11e0;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00983100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverMap::scalar deleting destructor` (Xbox PDB): the
/// destructor body (`00983130`), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn combat_cover_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00983130(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00983130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983130 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatCoverMap` destructor body (by its vtable, `0108d0e4`): deletes
/// every reserved cover location the map holds (`GetFirstPos`/`GetNext`,
/// `00983250(1)` on each non-null value), empties the map (`00438af0`);
/// when the pointer at +0x10 holds an object whose `009611e0` is not null,
/// calls that owner's slot 0xE8 with the object; then destroys the pointer
/// (`0045cec0`) and the `NiTMap` base (`00983820`).
pub fn fn_00983130(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_COVER_MAP_VTABLE);
    // The game's locals: key at EBP-0x18, value at EBP-0x14, position at
    // EBP-0x10.
    e.with_stack(0xc, |e, locals| {
        let key = locals.addr();
        let value = key + 4;
        let position = key + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_GET_NEXT, &args![this, position, key, value]);
            let location = e.mem.u32(value);
            if location != 0 {
                e.call(RESERVED_LOCATION_DELETE, &args![location, 1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    let pointer = base + COVER_MAP_POINTER;
    if e.call(POINTER_GET, &args![pointer]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![pointer]).u32();
        if e.call(POINTER_TARGET_OWNER, &args![object]).u32() != 0 {
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            let owner = e.call(POINTER_TARGET_OWNER, &args![object]).u32();
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            e.vcall(owner, 0xe8, &args![object]);
        }
    }
    e.call(POINTER_DESTRUCT, &args![pointer]);
    fn_00983820(e, this);
}

// Translated from 00983670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, ReservedCoverLocation*>` constructor (by the
/// vtable it sets, `0108d104`): the `NiTMapBase` part ([`fn_009837b0`]),
/// then the vtable. Returns `this`.
pub fn fn_00983670(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_009837b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    this
}

// Translated from 00983720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` 8-byte elements at `index` from the
/// `BSSimpleArray<CoverReservation, 1024>` at `this` (buffer +0x04, size
/// +0x08; unnamed in the engine map): `n = 0042f5a0(size - index - count,
/// count)`, destroys the removed elements (`0072ba80`), moves the last `n`
/// elements into the gap (`00692480(gap, end - n, n)`) and lowers the size
/// by `count`.
pub fn fn_00983720(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8);
    let moved = e
        .call(
            0x0042_f5a0,
            &args![size.wrapping_sub(index).wrapping_sub(count), count],
        )
        .u32();
    let gap = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(8));
    e.call(0x0072_ba80, &args![this, gap, count]);
    let buffer = e.mem.u32(base + 4);
    let size = e.mem.u32(base + 8);
    let tail = buffer
        .wrapping_add(size.wrapping_mul(8))
        .wrapping_sub(moved.wrapping_mul(8));
    let gap = buffer.wrapping_add(index.wrapping_mul(8));
    e.call(0x0069_2480, &args![this, gap, tail, moved]);
    let size = e.mem.u32(base + 8).wrapping_sub(count);
    e.mem.set_u32(base + 8, size);
}

// Translated from 009837b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, ReservedCoverLocation*>` constructor (by
/// the vtable it sets, `0108d138`): `m_uiHashSize = hash_size`, count 0,
/// and a zeroed table of `hash_size` pointers (`00aa1070`, `00403d30`).
/// Returns `this`.
pub fn fn_009837b0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_TMAP_BASE_VTABLE);
    e.mem.set_u32(base + HASH_SIZE, hash_size);
    e.mem.set_u32(base + MAP_COUNT, 0);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.call(MAP_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(base + HASH_TABLE, table);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.mem.u32(base + HASH_TABLE);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00983820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983820 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `NiTMap<unsigned int, ReservedCoverLocation*>` destructor body: sets its
/// vtable, empties the map (`00438af0`) and destroys the `NiTMapBase` part
/// ([`fn_00983880`]).
pub fn fn_00983820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00983880(e, this);
}

// Translated from 00983880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, ReservedCoverLocation*>` destructor body:
/// sets its vtable, empties the map (`00438af0`) and frees the table
/// (`00aa10f0`).
pub fn fn_00983880(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + HASH_TABLE);
    e.call(MAP_FREE, &args![table]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_3700, fn_00983700(Ptr)),
        entry!(
            0x0098_3100,
            combat_cover_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0098_3130, fn_00983130(Ptr)),
        entry!(0x0098_3670, fn_00983670(Ptr, u32) -> Ptr),
        entry!(0x0098_3720, fn_00983720(Ptr, u32, u32)),
        entry!(0x0098_37b0, fn_009837b0(Ptr, u32) -> Ptr),
        entry!(0x0098_3820, fn_00983820(Ptr)),
        entry!(0x0098_3880, fn_00983880(Ptr)),
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
    fn fn_00983700_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0098_3700, ARRAY_VTABLE_0108D124, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod cover_map_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            MAP_REMOVE_ALL,
            MAP_FREE,
            MEMSET,
            RESERVED_LOCATION_DELETE,
            POINTER_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MAP_ALLOCATE, |_, a| (0x0800_0000 + a[0]).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn map_constructors_set_the_fields_and_vtables() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + MAP_COUNT, 9);
        let back = e.call(0x0098_3670, &args![this, 0x25u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(e.mem.u32(this + HASH_SIZE), 0x25);
        assert_eq!(e.mem.u32(this + MAP_COUNT), 0);
        assert_eq!(e.mem.u32(this + HASH_TABLE), 0x0800_0094);
        assert_eq!(calls(&e, MAP_ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0800_0094, 0, 0x94]]);
        e.call(0x0098_37b0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructors_empty_and_free() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + HASH_TABLE, 0x4321);
        e.call(0x0098_3820, &args![this]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, MAP_FREE), vec![vec![0x4321]]);
        e.call_log = Some(vec![]);
        e.call(0x0098_3880, &args![this]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this]]);
    }

    #[test]
    fn cover_map_destructor_deletes_locations_and_notifies_the_owner() {
        let mut e = engine();
        // Two positions: the first yields a location, the second a null one.
        let values = Rc::new(RefCell::new(vec![0x6000u32, 0]));
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let next = values.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let value = next.borrow_mut().remove(0);
            e.mem.set_u32(a[3], value);
            let left = u32::from(!next.borrow().is_empty());
            e.mem.set_u32(a[1], left);
            Ret::default()
        });
        let owner = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xf0);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(vtable + 0xe8, 0x0066_0000);
        e.register(0x0066_0000, |_, _| Ret::default());
        e.register_double(POINTER_TARGET_OWNER, move |_, _| owner.into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + COVER_MAP_POINTER, 0x7777);
        e.call(0x0098_3100, &args![this, 1u32]);
        assert_eq!(calls(&e, RESERVED_LOCATION_DELETE), vec![vec![0x6000, 1]]);
        assert_eq!(calls(&e, MAP_GET_NEXT).len(), 2);
        assert_eq!(calls(&e, 0x0066_0000), vec![vec![owner, 0x7777]]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![this + 0x10]]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn cover_map_destructor_without_a_pointer_or_owner() {
        let mut e = engine();
        e.register(MAP_FIRST_POSITION, |_, _| 0u32.into_ret());
        e.register(POINTER_TARGET_OWNER, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.call(0x0098_3100, &args![this, 0u32]);
        assert!(calls(&e, MAP_GET_NEXT).is_empty());
        assert!(calls(&e, POINTER_TARGET_OWNER).is_empty());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(this + COVER_MAP_POINTER, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0098_3130, &args![this]);
        assert_eq!(calls(&e, POINTER_TARGET_OWNER), vec![vec![0x7777]]);
    }

    #[test]
    fn remove_at_destroys_and_moves_the_tail() {
        let mut e = engine();
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        e.register(0x0072_ba80, |_, _| Ret::default());
        e.register(0x0069_2480, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 10);
        e.call(0x0098_3720, &args![this, 2u32, 3u32]);
        assert_eq!(calls(&e, 0x0042_f5a0), vec![vec![5, 3]]);
        assert_eq!(calls(&e, 0x0072_ba80), vec![vec![this, 0x5010, 3]]);
        assert_eq!(
            calls(&e, 0x0069_2480),
            vec![vec![this, 0x5010, 0x5050 - 0x18, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 7);
    }
}
