//! `Havok SDK/hkmap.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to the 64-bit key, 64-bit value instance
//! `hkMapBase<unsigned __int64, unsigned __int64, hkMapOperations<unsigned
//! __int64>>` (the engine map names `insert`, `findKey`, `clearAndDeallocate`
//! and `resizeTable`; the other four have no name). The map is an open
//! addressing hash table: `m_elem` points at slots of 16 bytes (key low word,
//! key high word, value low word, value high word), an empty slot has both key
//! words `0xffffffff`, `m_hashMod` is the slot count minus one, and the
//! `m_numElems` word keeps the element count in its low 31 bits and, in bit 31
//! (`0x80000000`), the flag that the slots are not owned by the map (a buffer
//! given at construction), so they are never freed. A key hashes to
//! `(key low >> 4) * 0x9e3779b1 & m_hashMod`; collisions probe the next slot.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned __int64>>`
    /// (Xbox PDB), 0xC bytes.
    pub struct HkMapBase: 0xC {
        /// `m_elem` (Xbox PDB): the slots.
        0x00 m_elem: u32,
        /// `m_numElems` (Xbox PDB): the element count, bit 31 set when the slots are a
        /// buffer the map does not own.
        0x04 m_numElems: u32,
        /// `m_hashMod` (Xbox PDB): the slot count minus one.
        0x08 m_hashMod: i32,
    }
}

/// The hash multiplier `-0x61c8864f` (`0x9e3779b1`).
const HASH_MULTIPLIER: u32 = 0x9e37_79b1;
/// Bit 31 of `m_numElems`: the slots are not owned by the map.
const NOT_OWNED: u32 = 0x8000_0000;
/// A slot's key words are both this when it is empty.
const EMPTY: u32 = 0xffff_ffff;
/// The slot size.
const SLOT_SIZE: u32 = 0x10;

/// The slot address.
fn slot_address(e: &Engine, this: Ptr<HkMapBase>, slot: u32) -> u32 {
    e.get(this, HkMapBase::m_elem)
        .wrapping_add(slot.wrapping_mul(SLOT_SIZE))
}

/// Whether both key words are `0xffffffff` (`AND`, `CMP -1`).
fn is_empty(low: u32, high: u32) -> bool {
    low & high == EMPTY
}

/// The key's home slot before masking.
fn hash(key_low: u32) -> u32 {
    (key_low >> 4).wrapping_mul(HASH_MULTIPLIER)
}

/// Writes the two key words of a slot as the empty marker.
fn make_empty(e: &mut Engine, address: u32) {
    e.mem.set_u32(address, EMPTY);
    e.mem.set_u32(address + 4, EMPTY);
}

// Translated from 00cb1810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. The size in bytes of the slots for `count` elements:
/// the slot count is the smallest power of two that is at least 8 and at
/// least twice `count`, times 16 bytes. (Like the exe's signed loop, a
/// `count` above 0x20000000 never ends.)
pub fn fn_00cb1810(_e: &mut Engine, count: i32) -> u32 {
    let wanted = count.wrapping_add(count);
    let mut slots = 8i32;
    if wanted > slots {
        loop {
            slots = slots.wrapping_add(slots);
            if slots >= wanted {
                break;
            }
        }
    }
    (slots as u32) << 4
}

// Translated from 00cb1b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Sets the map up over a buffer of `byte_size` bytes at
/// `storage`: `m_elem` is the buffer, `m_numElems` is `0x80000000` (empty,
/// not owned), `m_hashMod` is the slot count minus one, and every slot is
/// marked empty.
pub fn fn_00cb1b80(e: &mut Engine, this: Ptr<HkMapBase>, storage: u32, byte_size: u32) {
    let slots = byte_size >> 4;
    e.set(this, HkMapBase::m_elem, storage);
    e.set(this, HkMapBase::m_numElems, NOT_OWNED);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1) as i32);
    for slot in 0..slots {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
}

// Translated from 00cb1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::insert` (Xbox PDB): stores the pair, replacing the value of an
/// equal key. First grows the table (`resizeTable`, through `allocator`, to
/// `2 * m_hashMod + 2` slots) when twice the element count exceeds
/// `m_hashMod`. The element count rises by one for a new key.
pub fn hk_map_base_insert(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key_low: u32,
    key_high: u32,
    value_low: u32,
    value_high: u32,
) {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let mask = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > mask {
        let slots = mask.wrapping_add(mask).wrapping_add(2);
        hk_map_base_resize_table(e, this, allocator, slots);
    }
    let mut slot = hash(key_low) & e.get(this, HkMapBase::m_hashMod) as u32;
    let mut added = 1u32;
    loop {
        let address = slot_address(e, this, slot);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if is_empty(low, high) {
            break;
        }
        if low == key_low && high == key_high {
            added = 0;
            break;
        }
        slot = slot.wrapping_add(1) & e.get(this, HkMapBase::m_hashMod) as u32;
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_add(added));
    let address = slot_address(e, this, slot);
    e.mem.set_u32(address, key_low);
    e.mem.set_u32(address + 4, key_high);
    e.mem.set_u32(address + 8, value_low);
    e.mem.set_u32(address + 0xc, value_high);
}

// Translated from 00cb1c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::findKey` (Xbox PDB): the slot index holding the key, or
/// `m_hashMod + 1` when it is not in the map (also when `m_hashMod` is not
/// positive).
pub fn hk_map_base_find_key(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
) -> u32 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let mut slot = hash(key_low) & mask as u32;
        loop {
            let address = slot_address(e, this, slot);
            let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
            if is_empty(low, high) {
                break;
            }
            if low == key_low && high == key_high {
                return slot;
            }
            slot = slot.wrapping_add(1) & mask as u32;
        }
    }
    (mask as u32).wrapping_add(1)
}

// Translated from 00cb1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Removes the element in slot `index`: empties the slot,
/// lowers the element count, then walks the slots that follow (up to the next
/// empty one) and moves back into the hole every element whose home slot
/// does not lie cyclically between the hole and itself, so that later
/// lookups still find them.
pub fn fn_00cb1d20(e: &mut Engine, this: Ptr<HkMapBase>, index: u32) {
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_sub(1));
    let address = slot_address(e, this, index);
    make_empty(e, address);
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;

    // The nearest empty slot before the hole, going backwards.
    let mut probe = mask.wrapping_add(index) & mask;
    loop {
        let address = slot_address(e, this, probe);
        if is_empty(e.mem.u32(address), e.mem.u32(address + 4)) {
            break;
        }
        probe = probe.wrapping_add(mask) & mask;
    }
    let first_after = probe.wrapping_add(1) & mask;

    let mut hole = index;
    let mut slot = index;
    loop {
        let mask = e.get(this, HkMapBase::m_hashMod) as u32;
        slot = slot.wrapping_add(1) & mask;
        let address = slot_address(e, this, slot);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if is_empty(low, high) {
            break;
        }
        let home = hash(low) & mask;
        // The three unsigned range tests of the exe, each of which keeps
        // the element where it is.
        let stays = (slot >= first_after && home > hole)
            || (slot < hole && (home > hole || home <= slot))
            || (home > hole && home < first_after);
        if !stays {
            let target = slot_address(e, this, hole);
            e.mem.set_u32(target, low);
            e.mem.set_u32(target + 4, high);
            let value_low = e.mem.u32(address + 8);
            let value_high = e.mem.u32(address + 0xc);
            e.mem.set_u32(target + 8, value_low);
            e.mem.set_u32(target + 0xc, value_high);
            make_empty(e, address);
            hole = slot;
        }
    }
}

// Translated from 00cb1e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Empties every slot and zeroes the element count, keeping
/// the not-owned flag in bit 31.
pub fn fn_00cb1e00(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    for slot in 0..slots.max(0) as u32 {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00cb1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::clearAndDeallocate` (Xbox PDB): empties the map, then, unless
/// the slots are not owned, returns them to `allocator` (vtable slot 8, the
/// slots and their size in bytes).
pub fn hk_map_base_clear_and_deallocate(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr) {
    fn_00cb1e00(e, this);
    if e.get(this, HkMapBase::m_numElems) & NOT_OWNED == 0 {
        let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1) as u32;
        let storage = e.get(this, HkMapBase::m_elem);
        e.vcall(allocator.addr(), 8, &args![storage, slots << 4]);
    }
}

// Translated from 00cb1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::resizeTable` (Xbox PDB): gives the map `max(slots, 8)` slots
/// from `allocator` (vtable slot 4, the size in bytes), marks them empty,
/// clears the element count (and the not-owned flag), then inserts every
/// element of the old table again and, if the old slots were owned, returns
/// them to the allocator (vtable slot 8, the slots and their size).
pub fn hk_map_base_resize_table(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, slots: i32) {
    let slots = if slots > 8 { slots } else { 8 };
    let was_not_owned = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_storage = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let storage = e
        .vcall(allocator.addr(), 4, &args![(slots as u32) << 4])
        .u32();
    e.set(this, HkMapBase::m_elem, storage);
    for slot in 0..slots as u32 {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
    e.set(this, HkMapBase::m_numElems, 0);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    for old in 0..old_slots.max(0) as u32 {
        let address = old_storage.wrapping_add(old * SLOT_SIZE);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if !is_empty(low, high) {
            let value_low = e.mem.u32(address + 8);
            let value_high = e.mem.u32(address + 0xc);
            hk_map_base_insert(e, this, allocator, low, high, value_low, value_high);
        }
    }
    if was_not_owned == 0 {
        let size = (old_slots as u32) << 4;
        e.vcall(allocator.addr(), 8, &args![old_storage, size]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb1810, fn_00cb1810(i32) -> u32),
        entry!(0x00cb1b80, fn_00cb1b80(Ptr<HkMapBase>, u32, u32)),
        entry!(
            0x00cb1bc0,
            hk_map_base_insert(Ptr<HkMapBase>, Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00cb1c60,
            hk_map_base_find_key(Ptr<HkMapBase>, u32, u32) -> u32
        ),
        entry!(0x00cb1d20, fn_00cb1d20(Ptr<HkMapBase>, u32)),
        entry!(0x00cb1e00, fn_00cb1e00(Ptr<HkMapBase>)),
        entry!(
            0x00cb1e40,
            hk_map_base_clear_and_deallocate(Ptr<HkMapBase>, Ptr)
        ),
        entry!(
            0x00cb1e70,
            hk_map_base_resize_table(Ptr<HkMapBase>, Ptr, i32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The allocator double's calls: (slot, first word, second word).
    type AllocatorLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// An allocator whose vtable slot 4 hands out blocks and slot 8 frees them;
    /// both are logged.
    fn allocator(e: &mut Engine) -> (Ptr, AllocatorLog) {
        let log: AllocatorLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        let allocate_log = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            allocate_log.borrow_mut().push((4, a[1], 0));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let free_log = Rc::clone(&log);
        e.register_double(0x0900_0008, move |_, a| {
            free_log.borrow_mut().push((8, a[1], a[2]));
            Ret::default()
        });
        (Ptr::new(object), log)
    }

    /// A map over a fresh `slots`-slot buffer it does not own.
    fn buffer_map(e: &mut Engine, slots: u32) -> Ptr<HkMapBase> {
        let this = e.new_object::<HkMapBase>();
        let storage = e.mem.alloc(slots * 16);
        e.call(0x00cb_1b80, &args![this, storage, slots * 16]);
        this
    }

    fn insert(e: &mut Engine, map: Ptr<HkMapBase>, alloc: Ptr, key: (u32, u32), value: (u32, u32)) {
        e.call(
            0x00cb_1bc0,
            &args![map, alloc, key.0, key.1, value.0, value.1],
        );
    }

    fn find(e: &mut Engine, map: Ptr<HkMapBase>, key: (u32, u32)) -> u32 {
        e.call(0x00cb_1c60, &args![map, key.0, key.1]).u32()
    }

    fn value_at(e: &Engine, map: Ptr<HkMapBase>, slot: u32) -> (u32, u32) {
        let address = slot_address(e, map, slot);
        (e.mem.u32(address + 8), e.mem.u32(address + 0xc))
    }

    /// `n` keys whose home slot in a table of `mask + 1` slots is `home`.
    fn keys_with_home(mask: u32, home: u32, n: usize) -> Vec<u32> {
        (0u32..)
            .map(|k| k << 4)
            .filter(|&key| hash(key) & mask == home)
            .take(n)
            .collect()
    }

    #[test]
    fn slot_bytes_is_a_power_of_two_slot_count() {
        let mut e = Engine::new();
        for (count, bytes) in [(0, 128), (4, 128), (5, 256), (100, 4096)] {
            assert_eq!(e.call(0x00cb_1810, &args![count]).u32(), bytes);
        }
    }

    #[test]
    fn buffer_setup_marks_every_slot_empty() {
        let mut e = Engine::new();
        let this = e.new_object::<HkMapBase>();
        let storage = e.mem.alloc(64);
        e.mem.write(storage, &[0u8; 64]);
        e.call(0x00cb_1b80, &args![this, storage, 64u32]);
        assert_eq!(e.get(this, HkMapBase::m_elem), storage);
        assert_eq!(e.get(this, HkMapBase::m_numElems), 0x8000_0000);
        assert_eq!(e.get(this, HkMapBase::m_hashMod), 3);
        for slot in 0..4 {
            assert_eq!(e.mem.u32(storage + 16 * slot), 0xffff_ffff);
            assert_eq!(e.mem.u32(storage + 16 * slot + 4), 0xffff_ffff);
        }
    }

    #[test]
    fn insert_and_find_key() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (111, 1));
        insert(&mut e, map, alloc, (0x20, 5), (222, 2));
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0002);
        let first = find(&mut e, map, (0x10, 0));
        let second = find(&mut e, map, (0x20, 5));
        assert_eq!(value_at(&e, map, first), (111, 1));
        assert_eq!(value_at(&e, map, second), (222, 2));
        // Same low word, other high word: not present; so is an absent key.
        assert_eq!(find(&mut e, map, (0x20, 6)), 8);
        assert_eq!(find(&mut e, map, (0x30, 0)), 8);
        // An equal key replaces the value and does not count again.
        insert(&mut e, map, alloc, (0x10, 0), (333, 3));
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0002);
        assert_eq!(value_at(&e, map, first), (333, 3));
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn find_key_in_a_map_without_slots_returns_one_past_the_mask() {
        let mut e = Engine::new();
        let map = e.new_object::<HkMapBase>();
        assert_eq!(find(&mut e, map, (0x10, 0)), 1);
    }

    #[test]
    fn colliding_keys_probe_the_next_slot() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        let keys = keys_with_home(7, 2, 3);
        for (i, key) in keys.iter().enumerate() {
            insert(&mut e, map, alloc, (*key, 0), (i as u32, 0));
        }
        let slots: Vec<u32> = keys.iter().map(|k| find(&mut e, map, (*k, 0))).collect();
        assert_eq!(slots, vec![2, 3, 4]);
    }

    #[test]
    fn insert_grows_a_buffer_map_and_keeps_the_buffer() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        // A 4-slot buffer map (mask 3): the third insert finds 2 * 2 > 3.
        let map = buffer_map(&mut e, 4);
        let old = e.get(map, HkMapBase::m_elem);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        insert(&mut e, map, alloc, (0x20, 0), (2, 0));
        assert!(log.borrow().is_empty());
        insert(&mut e, map, alloc, (0x30, 0), (3, 0));
        // 2 * 3 + 2 = 8 slots from the allocator (128 bytes), no free of the buffer.
        assert_eq!(*log.borrow(), vec![(4, 128, 0)]);
        assert_ne!(e.get(map, HkMapBase::m_elem), old);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 7);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 3);
        for (key, value) in [(0x10, 1), (0x20, 2), (0x30, 3)] {
            let slot = find(&mut e, map, (key, 0));
            assert_eq!(value_at(&e, map, slot), (value, 0));
        }
    }

    #[test]
    fn resize_frees_owned_slots_with_their_size() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        // An owned table: clear the flag as an earlier resize would have.
        e.set(map, HkMapBase::m_numElems, 1);
        let old = e.get(map, HkMapBase::m_elem);
        // Fewer than 8 slots still gets 8, by the clamp.
        e.call(0x00cb_1e70, &args![map, alloc, 3i32]);
        assert_eq!(*log.borrow(), vec![(4, 128, 0), (8, old, 128)]);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 7);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 1);
        let slot = find(&mut e, map, (0x10, 0));
        assert_eq!(value_at(&e, map, slot), (1, 0));
    }

    #[test]
    fn remove_moves_displaced_elements_back() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 16);
        // Three keys with home slot 5 (slots 5, 6, 7) and one with home slot 6
        // that probed to slot 8.
        let same = keys_with_home(15, 5, 3);
        let other = keys_with_home(15, 6, 1)[0];
        for (i, key) in same.iter().enumerate() {
            insert(&mut e, map, alloc, (*key, 0), (i as u32 + 10, 0));
        }
        insert(&mut e, map, alloc, (other, 0), (99, 0));
        assert_eq!(find(&mut e, map, (other, 0)), 8);
        // Removing the first one (slot 5) shifts the later ones back.
        e.call(0x00cb_1d20, &args![map, 5u32]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0003);
        assert_eq!(find(&mut e, map, (same[0], 0)), 16);
        for (i, key) in same.iter().enumerate().skip(1) {
            let slot = find(&mut e, map, (*key, 0));
            assert_eq!(value_at(&e, map, slot), (i as u32 + 10, 0));
        }
        let slot = find(&mut e, map, (other, 0));
        assert_eq!(value_at(&e, map, slot), (99, 0));
        // Every element is still reachable from its home slot without a gap.
        for key in [same[1], same[2], other] {
            assert_ne!(find(&mut e, map, (key, 0)), 16);
        }
    }

    #[test]
    fn remove_leaves_an_element_in_its_own_home_slot() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 16);
        let first = keys_with_home(15, 5, 1)[0];
        let in_place = keys_with_home(15, 6, 1)[0];
        insert(&mut e, map, alloc, (first, 0), (1, 0));
        insert(&mut e, map, alloc, (in_place, 0), (2, 0));
        assert_eq!(find(&mut e, map, (in_place, 0)), 6);
        e.call(0x00cb_1d20, &args![map, 5u32]);
        // The element at slot 6 has its home there: it must not move to 5.
        assert_eq!(find(&mut e, map, (in_place, 0)), 6);
        assert_eq!(find(&mut e, map, (first, 0)), 16);
    }

    #[test]
    fn clear_keeps_only_the_not_owned_flag() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        e.call(0x00cb_1e00, &args![map]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0000);
        assert_eq!(find(&mut e, map, (0x10, 0)), 8);
        e.set(map, HkMapBase::m_numElems, 5);
        e.call(0x00cb_1e00, &args![map]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0);
    }

    #[test]
    fn clear_and_deallocate_frees_only_owned_slots() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        e.call(0x00cb_1e40, &args![map, alloc]);
        assert!(log.borrow().is_empty());
        e.set(map, HkMapBase::m_numElems, 0);
        let storage = e.get(map, HkMapBase::m_elem);
        e.call(0x00cb_1e40, &args![map, alloc]);
        assert_eq!(*log.borrow(), vec![(8, storage, 128)]);
    }
}
