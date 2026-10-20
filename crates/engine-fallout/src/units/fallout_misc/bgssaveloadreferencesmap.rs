//! `fallout/misc/saveload/bgssaveloadreferencesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00853130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Map lookup, returning success in AL. A failed lookup leaves the output
/// untouched. Bucket pointers, links and values are read after the relevant
/// hash/equality callback, as in the PC instructions.
pub fn lookup(e: &mut Engine, this: Ptr, key: u32, out: Ptr<u32>) -> u8 {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket));
    while node != 0 {
        let existing_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, existing_key]).u8() != 0 {
            let value = e.mem.u32(node.wrapping_add(8));
            e.mem.set_u32(out.addr(), value);
            return 1;
        }
        node = e.mem.u32(node);
    }
    0
}

// Translated from 00853240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the item's value then releases it through the allocator at +0xc.
pub fn delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr().wrapping_add(8), 0);
    e.call(0x006b8310, &args![this.addr().wrapping_add(0xc), item]);
}

/// `00aa1070`: the allocator (cdecl, byte count).
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;
/// The bucket count `00852810` and `00852a70` pass on.
const BUCKET_COUNT: u32 = 0x25;

/// Shared body of `00853270` and `00853370`: stores `vtable`, the bucket
/// count at +4 and 0 at +0xc, allocates `4 * count` bytes for the table
/// (+8) and zeroes it.
fn init_bucket_table(e: &mut Engine, this: Ptr, vtable: u32, bucket_count: u32) {
    let t = this.addr();
    e.mem.set_u32(t, vtable);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
}

// Translated from 00853270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: bucket-table initializer with the vtable
/// `010800a8`. Returns `this`.
pub fn fn_00853270(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_00a8, bucket_count);
    this
}

// Translated from 00853370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: bucket-table initializer with the vtable
/// `010800c8`. Returns `this`.
pub fn fn_00853370(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_00c8, bucket_count);
    this
}

// Translated from 008530c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the bucket-table initializer with the vtable
/// `01080088` (same body as `00853270`). Returns `this`.
pub fn fn_008530c0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_0088, bucket_count);
    this
}

// Translated from 00852fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008530c0(this, word)` and then the vtable
/// `01080028`. Returns `this`.
pub fn fn_00852fa0(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    fn_008530c0(e, this, word);
    e.mem.set_u32(this.addr(), 0x0108_0028);
    this
}

// Translated from 00852fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00853270(this, bucket_count)`, then the
/// vtable `01080048`. Returns `this`.
pub fn fn_00852fd0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00853270(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0108_0048);
    this
}

// Translated from 00853000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00853370(this, bucket_count)`, then the
/// vtable `01080068`. Returns `this`.
pub fn fn_00853000(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00853370(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0108_0068);
    this
}

// Translated from 00852810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00852fa0(this, 0x25)`, then the vtable
/// `0107fef4`. Returns `this`.
pub fn fn_00852810(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00852fa0(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), 0x0107_fef4);
    this
}

// Translated from 00852a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the object that holds three
/// maps: `00852fd0(this, 0x25)`, `00852810` on the member at +0x10 and
/// `00853000(this + 0x20, 0x25)`. Returns `this`.
// Translation incomplete: 00852a70 - SEH exception unwinding is unsupported.
pub fn fn_00852a70(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00852fd0(e, this, BUCKET_COUNT);
    fn_00852810(e, this.byte_add(0x10).cast());
    fn_00853000(e, this.byte_add(0x20).cast(), BUCKET_COUNT);
    this
}

/// `00438af0` (unnamed in the engine map): `thiscall`, no stack words; empties a hash map (the
/// shared `RemoveAll` body of the `NiTMapBase` instances).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (unnamed in the engine map): `cdecl`, one word; frees a block from `00aa1070`.
const FREE: u32 = 0x00aa_10f0;
/// `008528d0` (unnamed in the engine map): `thiscall`, no stack words; empties the map at `+0x10`
/// of `BGSSaveLoadReferencesMap`.
const MAP_0X10_REMOVE_ALL: u32 = 0x0085_28d0;

/// Shared body of the three bucket-table destructors `00853210`, `00853340` and `00853440`:
/// stores `vtable`, empties the map (`00438af0`) and frees its bucket table (`+8`) through
/// `00aa10f0`.
fn destroy_bucket_table(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(FREE, &args![table]);
}

// Translated from 00853210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `008530c0` builds (vtable
/// `01080088`): stores the vtable, empties the map through `00438af0` and frees the bucket table
/// (`+8`) through `00aa10f0`.
pub fn fn_00853210(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_0088);
}

// Translated from 00853340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `00853270` builds (vtable
/// `010800a8`), the same body as `00853210`.
pub fn fn_00853340(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_00a8);
}

// Translated from 00853440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `00853370` builds (vtable
/// `010800c8`), the same body as `00853210`.
pub fn fn_00853440(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_00c8);
}

// Translated from 008531b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008531b0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852fa0` builds: stores the vtable
/// `01080028`, empties the map through `00438af0`, then runs the base destructor `00853210` of
/// this unit.
pub fn fn_008531b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0028);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853210(e, this);
}

// Translated from 008532e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008532e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852fd0` builds: stores the vtable
/// `01080048`, empties the map through `00438af0`, then runs the base destructor `00853340` of
/// this unit.
pub fn fn_008532e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0048);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853340(e, this);
}

// Translated from 008533e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008533e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00853000` builds: stores the vtable
/// `01080068`, empties the map through `00438af0`, then runs the base destructor `00853440` of
/// this unit.
pub fn fn_008533e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0068);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853440(e, this);
}

// Translated from 00852870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00852870 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852810` builds: stores the vtable
/// `0107fef4`, empties it through `008528d0`, then runs `008531b0` of this unit.
pub fn fn_00852870(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_fef4);
    e.call(MAP_0X10_REMOVE_ALL, &args![this]);
    fn_008531b0(e, this);
}

// Translated from 00852b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the three maps of `BGSSaveLoadReferencesMap`: the map at
/// `+0` through `00438af0`, the one at `+0x10` through `008528d0`; for the one at `+0x20` it takes
/// the first position from `004b9ba0` and, while the position is non-zero, gets the next entry
/// through `006b7f20(map, &position, &key, &value)` (key and value cleared first) and deletes a
/// non-null value through its vtable slot 0 with 1 (the scalar deleting destructor); then that
/// map is emptied through `00438af0`.
pub fn fn_00852b60(e: &mut Engine, this: Ptr) {
    e.call(MAP_REMOVE_ALL, &args![this]);
    let second = this.addr().wrapping_add(0x10);
    e.call(MAP_0X10_REMOVE_ALL, &args![second]);
    let third = this.addr().wrapping_add(0x20);
    e.with_stack(0xc, |e, locals| {
        let value_slot = locals.addr();
        let key_slot = value_slot + 4;
        let position_slot = value_slot + 8;
        let first = e.call(0x004b_9ba0, &args![third]).u32();
        e.mem.set_u32(position_slot, first);
        while e.mem.u32(position_slot) != 0 {
            e.mem.set_u32(key_slot, 0);
            e.mem.set_u32(value_slot, 0);
            e.call(
                0x006b_7f20,
                &args![third, position_slot, key_slot, value_slot],
            );
            let value = e.mem.u32(value_slot);
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![third]);
}

// Translated from 00852ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00852ae0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of `BGSSaveLoadReferencesMap` (the object `00852a70`
/// builds): empties the maps through `00852b60`, then destroys the members in reverse order:
/// `008533e0` on `+0x20`, `00852870` on `+0x10` and `008532e0` on `this`.
pub fn fn_00852ae0(e: &mut Engine, this: Ptr) {
    fn_00852b60(e, this);
    fn_008533e0(e, this.byte_add(0x20).cast());
    fn_00852870(e, this.byte_add(0x10).cast());
    fn_008532e0(e, this);
}

// Translated from 00852d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadReferencesMap::LoadReferencesForCell` (Xbox PDB). `RET 8`: `cell` and a word
/// passed on to each `LoadChangedReference` (`00852e40`). When `00425fd0(cell)` is true the
/// cell's key is `0084e3a0(cell)` and the map is the one at `+0x10`. Otherwise the key is
/// `00587410(TESObjectCELL::GetDataX(cell), TESObjectCELL::GetDataY(cell))` (`00544c30`,
/// `00544c60`, called Y first) and the map is looked up (`00853130` of this unit) in the map at
/// `+0x20` under `0084e3a0(TESObjectCELL::GetWorldSpace(cell))` (`0054ddd0`); a failed lookup
/// leaves it null. With a map, a successful lookup of the cell's key gives a list: each of its
/// `0044ddc0` entries (`006a7ad0(list, index)` gives the entry's address) is loaded through
/// `00852e40(this, entry, word)`; the key is then removed from the map (`00405430`) and the list
/// is deleted through its vtable slot 0 with 1.
pub fn bgs_save_load_references_map_load_references_for_cell(
    e: &mut Engine,
    this: Ptr,
    cell: Ptr,
    word: u32,
) {
    e.with_stack(8, |e, locals| {
        let map_slot = locals.addr();
        let list_slot = map_slot + 4;
        e.mem.set_u32(map_slot, 0);
        let key = if e.call(0x0042_5fd0, &args![cell]).bool() {
            let key = e.call(0x0084_e3a0, &args![cell]).u32();
            e.mem.set_u32(map_slot, this.addr().wrapping_add(0x10));
            key
        } else {
            let y = e.call(0x0054_4c60, &args![cell]).u32();
            let x = e.call(0x0054_4c30, &args![cell]).u32();
            let key = e.call(0x0058_7410, &args![x, y]).u32();
            let world = e.call(0x0054_ddd0, &args![cell]).u32();
            let world_key = e.call(0x0084_e3a0, &args![world]).u32();
            lookup(e, this.byte_add(0x20).cast(), world_key, Ptr::new(map_slot));
            key
        };
        let map = e.mem.u32(map_slot);
        if map == 0 {
            return;
        }
        e.mem.set_u32(list_slot, 0);
        if lookup(e, Ptr::new(map), key, Ptr::new(list_slot)) == 0 {
            return;
        }
        let mut index = 0u32;
        loop {
            let list = e.mem.u32(list_slot);
            let count = e.call(0x0044_ddc0, &args![list]).u32();
            if index >= count {
                break;
            }
            let list = e.mem.u32(list_slot);
            let entry = e.call(0x006a_7ad0, &args![list, index]).u32();
            let reference = e.mem.u32(entry);
            e.call(0x0085_2e40, &args![this, reference, word]);
            index = index.wrapping_add(1);
        }
        let map = e.mem.u32(map_slot);
        e.call(0x0040_5430, &args![map, key]);
        let list = e.mem.u32(list_slot);
        if list != 0 {
            e.vcall(list, 0, &args![1u32]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00853130, lookup(Ptr, u32, Ptr<u32>) -> u8),
        entry!(0x00853240, delete_item(Ptr, Ptr)),
        entry!(0x00853270, fn_00853270(Ptr, u32) -> Ptr),
        entry!(0x00853370, fn_00853370(Ptr, u32) -> Ptr),
        entry!(0x008530c0, fn_008530c0(Ptr, u32) -> Ptr),
        entry!(0x00852fa0, fn_00852fa0(Ptr, u32) -> Ptr),
        entry!(0x00852fd0, fn_00852fd0(Ptr, u32) -> Ptr),
        entry!(0x00853000, fn_00853000(Ptr, u32) -> Ptr),
        entry!(0x00852810, fn_00852810(Ptr) -> Ptr),
        entry!(0x00852a70, fn_00852a70(Ptr) -> Ptr),
        entry!(0x0085_3210, fn_00853210(Ptr)),
        entry!(0x0085_3340, fn_00853340(Ptr)),
        entry!(0x0085_3440, fn_00853440(Ptr)),
        entry!(0x0085_31b0, fn_008531b0(Ptr)),
        entry!(0x0085_32e0, fn_008532e0(Ptr)),
        entry!(0x0085_33e0, fn_008533e0(Ptr)),
        entry!(0x0085_2870, fn_00852870(Ptr)),
        entry!(0x0085_2b60, fn_00852b60(Ptr)),
        entry!(0x0085_2ae0, fn_00852ae0(Ptr)),
        entry!(
            0x0085_2d20,
            bgs_save_load_references_map_load_references_for_cell(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn the_bucket_table_initializers_differ_only_in_the_vtable() {
        for (addr, vtable) in [(0x00853270, 0x0108_00a8), (0x00853370, 0x0108_00c8)] {
            let mut e = Engine::new();
            doubles(&mut e);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 5);
            let r = e.call(addr, &args![this, 7u32]);
            assert_eq!(r.u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 7);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            let table = e.mem.u32(this + 8);
            assert_eq!(e.mem.bytes(table, 28), vec![0; 28]);
        }
    }

    #[test]
    fn fn_008530c0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008530c0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_0088);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.bytes(table, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![12]));
        assert_eq!(log[2], (MEMSET, vec![table, 0, 12]));
    }

    #[test]
    fn the_wrappers_store_their_own_vtables() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call(0x00852fa0, &args![this, 0x11u32]);
        assert_eq!(e.mem.u32(this), 0x0108_0028);
        assert_eq!(e.mem.u32(this + 4), 0x11, "the base initializer ran");
        e.call(0x00852fd0, &args![this, 2u32]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0x0108_0048, 2));
        e.call(0x00853000, &args![this, 3u32]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0x0108_0068, 3));
        e.call(0x00852810, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0107_fef4);
    }

    #[test]
    fn the_constructor_builds_three_maps() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let r = e.call(0x00852a70, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_0048);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_fef4);
        assert_eq!(e.mem.u32(this + 0x20), 0x0108_0068);
        assert_eq!(e.mem.u32(this + 0x24), 0x25);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;

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

    #[test]
    fn the_bucket_table_destructors_empty_and_free() {
        for (address, vtable) in [
            (0x0085_3210, 0x0108_0088),
            (0x0085_3340, 0x0108_00a8),
            (0x0085_3440, 0x0108_00c8),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[MAP_REMOVE_ALL, FREE]);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7700);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(
                log[1..],
                [(MAP_REMOVE_ALL, vec![this]), (FREE, vec![0x7700])]
            );
        }
    }

    #[test]
    fn the_derived_destructors_chain_to_their_bases() {
        for (address, vtable, base_vtable) in [
            (0x0085_31b0, 0x0108_0028, 0x0108_0088),
            (0x0085_32e0, 0x0108_0048, 0x0108_00a8),
            (0x0085_33e0, 0x0108_0068, 0x0108_00c8),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[FREE]);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7700);
            let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let record = seen.clone();
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            e.call(address, &args![this]);
            assert_eq!(*seen.borrow(), vec![vtable, base_vtable]);
            assert_eq!(e.mem.u32(this), base_vtable);
        }
    }

    #[test]
    fn fn_00852870_empties_then_runs_the_map_destructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, FREE, MAP_0X10_REMOVE_ALL]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0085_2870, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MAP_0X10_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(e.mem.u32(this), 0x0108_0088);
    }

    #[test]
    fn fn_00852b60_deletes_the_values_of_the_third_map() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, MAP_0X10_REMOVE_ALL]);
        let this = e.mem.alloc(0x30);
        // Two entries: the first with a value, the second without; position 2 ends the walk.
        e.register(0x004b_9ba0, |_, _| 1u32.into_ret());
        e.register(0x006b_7f20, |e, a| {
            let position = e.mem.u32(a[1]);
            let value = if position == 1 { 0x6000 } else { 0 };
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], if position == 1 { 2 } else { 0 });
            Ret::default()
        });
        e.map(0x6000, 0x1000);
        e.mem.set_u32(0x6000, 0x6100);
        e.mem.set_u32(0x6100, 0x7000_0010);
        quiet(&mut e, &[0x7000_0010]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2b60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MAP_REMOVE_ALL),
            vec![vec![this], vec![this + 0x20]]
        );
        assert_eq!(calls(&log, MAP_0X10_REMOVE_ALL), vec![vec![this + 0x10]]);
        assert_eq!(calls(&log, 0x006b_7f20).len(), 2);
        assert_eq!(calls(&log, 0x006b_7f20)[0][0], this + 0x20);
        assert_eq!(calls(&log, 0x7000_0010), vec![vec![0x6000, 1]]);
    }

    #[test]
    fn fn_00852ae0_destroys_the_members_in_reverse() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[MAP_REMOVE_ALL, MAP_0X10_REMOVE_ALL, FREE, 0x006b_7f20],
        );
        e.register(0x004b_9ba0, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x0085_2ae0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MAP_0X10_REMOVE_ALL),
            vec![vec![this + 0x10], vec![this + 0x10]]
        );
        assert_eq!(e.mem.u32(this), 0x0108_00a8);
        assert_eq!(e.mem.u32(this + 0x10), 0x0108_0088);
        assert_eq!(e.mem.u32(this + 0x20), 0x0108_00c8);
        let frees = calls(&log, MAP_REMOVE_ALL);
        assert_eq!(*frees.last().unwrap(), vec![this]);
    }

    const HASH: u32 = 0x7000_0004;
    const EQUAL: u32 = 0x7000_0008;
    const MAP_VTABLE: u32 = 0x6800;

    /// A one-bucket map (the hash is always 0) holding `entries` as (key, value).
    fn one_bucket_map(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, MAP_VTABLE);
        let table = e.mem.alloc(4);
        e.mem.set_u32(map + 8, table);
        let mut next = 0;
        for (key, value) in entries {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, *key);
            e.mem.set_u32(node + 8, *value);
            next = node;
        }
        e.mem.set_u32(table, next);
        map
    }

    fn map_doubles(e: &mut Engine) {
        e.map(MAP_VTABLE & !0xfff, 0x1000);
        e.put_vtable(MAP_VTABLE, &[0, HASH, EQUAL]);
        e.register(HASH, |_, _| 0u32.into_ret());
        e.register(EQUAL, |_, a| (a[1] == a[2]).into_ret());
    }

    /// A list of `entries` reference words: `0044ddc0` gives the count at `+4`, `006a7ad0` the
    /// entry's address in the block at `+8`; vtable slot 0 is the deleting destructor.
    fn list(e: &mut Engine, entries: &[u32]) -> u32 {
        let list = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x7000_0020);
        e.mem.set_u32(list, vtable);
        e.mem.set_u32(list + 4, entries.len() as u32);
        let block = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *entry);
        }
        e.mem.set_u32(list + 8, block);
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x006a_7ad0, |e, a| {
            (e.mem.u32(a[0] + 8) + 4 * a[1]).into_ret()
        });
        e.register(0x7000_0020, |_, _| Ret::default());
        list
    }

    #[test]
    fn load_references_for_cell_uses_the_map_at_0x10_for_the_first_kind_of_cell() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let references = list(&mut e, &[0xa1, 0xa2]);
        let inner = one_bucket_map(&mut e, &[(0x55, references)]);
        // The map at +0x10 is the object's own member: copy the built map's words there.
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(inner + offset);
            e.mem.set_u32(this + 0x10 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| true.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x55u32.into_ret());
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 0x99u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0085_2e40),
            vec![vec![this, 0xa1, 0x99], vec![this, 0xa2, 0x99]]
        );
        assert_eq!(calls(&log, 0x0040_5430), vec![vec![this + 0x10, 0x55]]);
        assert_eq!(calls(&log, 0x7000_0020), vec![vec![references, 1]]);
    }

    #[test]
    fn load_references_for_cell_finds_an_exterior_cell_through_its_world() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let references = list(&mut e, &[0xb1]);
        let cells = one_bucket_map(&mut e, &[(0x1234, references)]);
        let worlds = one_bucket_map(&mut e, &[(0x77, cells)]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(worlds + offset);
            e.mem.set_u32(this + 0x20 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| false.into_ret());
        e.register(0x0054_4c30, |_, _| 3u32.into_ret());
        e.register(0x0054_4c60, |_, _| 4u32.into_ret());
        e.register(0x0058_7410, |_, a| {
            assert_eq!(a, [3, 4]);
            0x1234u32.into_ret()
        });
        e.register(0x0054_ddd0, |_, _| 0x900u32.into_ret());
        e.register(0x0084_e3a0, |_, a| {
            assert_eq!(a, [0x900]);
            0x77u32.into_ret()
        });
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [0x0054_4c30, 0x0054_4c60].contains(a))
            .collect();
        assert_eq!(order, [0x0054_4c60, 0x0054_4c30]);
        assert_eq!(calls(&log, 0x0085_2e40), vec![vec![this, 0xb1, 5]]);
        assert_eq!(calls(&log, 0x0040_5430), vec![vec![cells, 0x1234]]);
        assert_eq!(calls(&log, 0x7000_0020), vec![vec![references, 1]]);
    }

    #[test]
    fn load_references_for_cell_stops_without_a_map_or_a_list() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let worlds = one_bucket_map(&mut e, &[]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(worlds + offset);
            e.mem.set_u32(this + 0x20 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| false.into_ret());
        quiet(
            &mut e,
            &[0x0054_4c30, 0x0054_4c60, 0x0058_7410, 0x0054_ddd0],
        );
        e.register(0x0084_e3a0, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        // The world has no map: nothing more is called.
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap().0, HASH);
        // A map without the cell's key: no list is loaded.
        let empty = one_bucket_map(&mut e, &[]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(empty + offset);
            e.mem.set_u32(this + 0x10 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| true.into_ret());
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0085_2e40).is_empty());
        assert!(calls(&log, 0x0040_5430).is_empty());
    }
}
