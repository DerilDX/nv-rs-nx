//! `fallout/ai/combat/combattargetmap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00853130(this, key, &out)` (unit `bgssaveloadreferencesmap`): the map
/// lookup; returns success in `AL` and writes the value to `out` only on
/// success.
const MAP_LOOKUP: u32 = 0x0085_3130;

// Translated from 009a4250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap::GetTargetAttackerArray` (engine map name): looks `key`
/// up in the map with `00853130` and returns the value found, or 0 when the
/// lookup fails.
pub fn combat_target_map_get_target_attacker_array(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        if e.call(MAP_LOOKUP, &args![this, key, found]).bool() {
            e.mem.u32(found.addr())
        } else {
            0
        }
    })
}

/// `CombatTargetMap`'s vtable (`0108e1e8`, named by its RTTI locator).
const COMBAT_TARGET_MAP_VTABLE: u32 = 0x0108_e1e8;
/// `NiTMap<Actor*, CombatTargetData*>`'s vtable (`0108e258`).
const NI_TMAP_VTABLE: u32 = 0x0108_e258;
/// `NiTMapBase<DFALL<NiTMapItem<Actor*, CombatTargetData*>>, Actor*,
/// CombatTargetData*>`'s vtable (`0108e278`).
const NI_TMAP_BASE_VTABLE: u32 = 0x0108_e278;
/// `NiTMapBase` fields (Xbox PDB names): `m_uiHashSize` +0x04,
/// `m_ppkHashTable` +0x08, the allocator's `m_uiCount` +0x0C.
const HASH_SIZE: u32 = 0x04;
const HASH_TABLE: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
/// The allocator (`00aa1070`, `cdecl(size)`).
const MAP_ALLOCATE: u32 = 0x00aa_1070;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMapBase::RemoveAll` of this instance (`00438af0`, `thiscall`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `GetFirstPos` (`004b9ba0`, `thiscall`) and `GetNext` (`006b7f20`,
/// `thiscall(position pointer, key pointer, value pointer)`).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// The scalar deleting destructor of a target's data (`009a3fb0`,
/// `thiscall(flags)`).
const TARGET_DATA_DELETE: u32 = 0x009a_3fb0;
/// The `NiTMapBase` destructor body of this instance (`009a4480`).
const NI_TMAP_BASE_DESTRUCT: u32 = 0x009a_4480;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009a3ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap::scalar deleting destructor` (Xbox PDB): the destructor
/// body (`009a3ef0`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn combat_target_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009a3ef0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a3ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3ef0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatTargetMap` destructor body (by its vtable, `0108e1e8`): deletes
/// every target's data the map holds (`GetFirstPos`/`GetNext`, `009a3fb0(1)`
/// on each non-null value), empties the map (`00438af0`) and destroys the
/// `NiTMap` base ([`fn_009a4420`]).
pub fn fn_009a3ef0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COMBAT_TARGET_MAP_VTABLE);
    // The game's locals: value at EBP-0x18, key at EBP-0x14, position at
    // EBP-0x10.
    e.with_stack(0xc, |e, locals| {
        let value = locals.addr();
        let key = value + 4;
        let position = value + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_GET_NEXT, &args![this, position, key, value]);
            let data = e.mem.u32(value);
            if data != 0 {
                e.call(TARGET_DATA_DELETE, &args![data, 1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_009a4420(e, this);
}

// Translated from 009a4290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*, CombatTargetData*>` constructor (by the vtable it sets,
/// `0108e258`): the `NiTMapBase` part ([`fn_009a43b0`]), then the vtable.
/// Returns `this`.
pub fn fn_009a4290(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_009a43b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    this
}

// Translated from 009a43b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., Actor*, CombatTargetData*>` constructor (by the vtable
/// it sets, `0108e278`): `m_uiHashSize = hash_size`, count 0, and a zeroed
/// table of `hash_size` pointers (`00aa1070`, `00403d30`). Returns `this`.
pub fn fn_009a43b0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
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

// Translated from 009a4420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a4420 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `NiTMap<Actor*, CombatTargetData*>` destructor body: sets its vtable,
/// empties the map (`00438af0`) and destroys the `NiTMapBase` part
/// (`009a4480`).
pub fn fn_009a4420(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(NI_TMAP_BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009a4250,
            combat_target_map_get_target_attacker_array(Ptr, u32) -> u32
        ),
        entry!(
            0x009a_3ec0,
            combat_target_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009a_3ef0, fn_009a3ef0(Ptr)),
        entry!(0x009a_4290, fn_009a4290(Ptr, u32) -> Ptr),
        entry!(0x009a_43b0, fn_009a43b0(Ptr, u32) -> Ptr),
        entry!(0x009a_4420, fn_009a4420(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hit_returns_the_value_a_miss_returns_zero() {
        let mut e = Engine::new();
        e.register(MAP_LOOKUP, |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], 0x1234);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                // A failed lookup may leave a stale low byte in EAX's upper
                // bits; only AL counts.
                Ret {
                    eax: 0x100,
                    ..Ret::default()
                }
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 7u32]).u32(), 0x1234);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 8u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            (log[1].0, log[1].1[..2].to_vec()),
            (MAP_LOOKUP, vec![0x5000, 7])
        );
    }
}

#[cfg(test)]
mod target_map_tests {
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
            MEMSET,
            TARGET_DATA_DELETE,
            NI_TMAP_BASE_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MAP_ALLOCATE, |_, a| (0x0800_0000 + a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn map_constructors_set_the_fields_and_vtables() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + MAP_COUNT, 3);
        assert_eq!(e.call(0x009a_4290, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(e.mem.u32(this + HASH_SIZE), 0x25);
        assert_eq!(e.mem.u32(this + MAP_COUNT), 0);
        assert_eq!(e.mem.u32(this + HASH_TABLE), 0x0800_0094);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0800_0094, 0, 0x94]]);
        assert_eq!(e.call(0x009a_43b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructor_empties_then_destroys_the_base() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call(0x009a_4420, &args![this]);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn target_map_destructor_deletes_each_value() {
        let mut e = engine();
        let values = Rc::new(RefCell::new(vec![0x6000u32, 0, 0x6100]));
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let next = values.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let value = next.borrow_mut().remove(0);
            e.mem.set_u32(a[3], value);
            let left = u32::from(!next.borrow().is_empty());
            e.mem.set_u32(a[1], left);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x009a_3ec0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, TARGET_DATA_DELETE),
            vec![vec![0x6000, 1], vec![0x6100, 1]]
        );
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        // Flag clear and an empty map: no delete at all.
        e.register(MAP_FIRST_POSITION, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009a_3ec0, &args![this, 2u32]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, MAP_GET_NEXT).is_empty());
        e.call(0x009a_3ef0, &args![this]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT).len(), 2);
    }
}
