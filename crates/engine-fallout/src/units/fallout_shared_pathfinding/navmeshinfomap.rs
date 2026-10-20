//! `fallout shared/pathfinding/navmeshinfomap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b81d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map key equality: compares full 32-bit words, returns AL.
pub fn keys_equal(_e: &mut Engine, _this: Ptr, left: u32, right: u32) -> u8 {
    u8::from(left == right)
}

/// The virtual slot (byte offset) of the map that answers the bucket index of a key.
const SLOT_BUCKET_OF_KEY: u32 = 0x04;

// Translated from 006b7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`navmeshinfomap.cpp`). `thiscall`, `RET 0xc`: the iterator step of
/// a chained hash map (`this`: vtable, bucket count at +4, bucket array at +8; an entry: next
/// at +0, key at +4, value at +8). `iterator` points to the current entry: its key and value are
/// stored through `key` and `value`, then the iterator moves to the entry's next, or, at the end
/// of the chain, to the first entry of a later non-empty bucket (the current key's bucket index
/// comes from virtual slot +4 of the map), or to null after the last bucket. The key is read
/// from the entry again for the slot call, as the exe does.
pub fn fn_006b7f20(e: &mut Engine, this: Ptr, iterator: Ptr, key: Ptr, value: Ptr) {
    let entry = e.mem.u32(iterator.addr());
    let entry_key = e.mem.u32(entry.wrapping_add(4));
    e.mem.set_u32(key.addr(), entry_key);
    let entry_value = e.mem.u32(entry.wrapping_add(8));
    e.mem.set_u32(value.addr(), entry_value);
    let next = e.mem.u32(entry);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let key_again = e.mem.u32(entry.wrapping_add(4));
    let bucket = e
        .vcall(this.addr(), SLOT_BUCKET_OF_KEY, &args![key_again])
        .u32();
    let mut index = bucket.wrapping_add(1);
    loop {
        if index >= e.mem.u32(this.addr().wrapping_add(4)) {
            e.mem.set_u32(iterator.addr(), 0);
            return;
        }
        let buckets = e.mem.u32(this.addr().wrapping_add(8));
        let first = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if first != 0 {
            e.mem.set_u32(iterator.addr(), first);
            return;
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_7f20, fn_006b7f20(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x006b81d0, keys_equal(Ptr, u32, u32) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_equality_matches_native_low_byte_vectors() {
        // Generated native vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80. Only AL is the boolean result.
        let mut e = Engine::new();
        for (left, right, expected) in [
            (0u32, 0u32, 1u8),
            (0x12345678, 0x12345678, 1),
            (0x12345678, 0x12345679, 0),
            (0, 0x80000000, 0),
        ] {
            assert_eq!(e.call(0x006b81d0, &args![0u32, left, right]).u8(), expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    /// A map with `buckets` buckets whose slot +4 answers `key % buckets`; entries are
    /// (next, key, value).
    fn map(e: &mut Engine, buckets: u32) -> u32 {
        let mut slots = [0u32; 3];
        slots[1] = 0x0f00_a000;
        e.put_vtable(0x0f00_a100, &slots);
        e.register_double(0x0f00_a000, move |_, a| (a[1] % buckets).into_ret());
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map, 0x0f00_a100);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        map
    }

    fn entry(e: &mut Engine, next: u32, key: u32, value: u32) -> u32 {
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, next);
        e.mem.set_u32(entry + 4, key);
        e.mem.set_u32(entry + 8, value);
        entry
    }

    fn step(e: &mut Engine, map: u32, current: u32) -> (u32, u32, u32) {
        let out = e.mem.alloc(0xc);
        e.mem.set_u32(out, current);
        e.call(0x006b_7f20, &args![map, out, out + 4, out + 8]);
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8))
    }

    #[test]
    fn at_006b7f20_follows_the_chain_when_the_entry_has_a_next() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let second = entry(&mut e, 0, 8, 0xbbbb);
        let first = entry(&mut e, second, 4, 0xaaaa);
        assert_eq!(step(&mut e, map, first), (second, 4, 0xaaaa));
    }

    #[test]
    fn at_006b7f20_moves_to_the_next_non_empty_bucket_at_the_end_of_a_chain() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let table = e.mem.u32(map + 8);
        let in_bucket_three = entry(&mut e, 0, 7, 0xdddd);
        e.mem.set_u32(table + 12, in_bucket_three);
        // Key 5 sits in bucket 1; buckets 2 is empty, bucket 3 has an entry.
        let last = entry(&mut e, 0, 5, 0xcccc);
        assert_eq!(step(&mut e, map, last), (in_bucket_three, 5, 0xcccc));
    }

    #[test]
    fn at_006b7f20_ends_with_null_after_the_last_bucket() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let last = entry(&mut e, 0, 7, 0xeeee);
        // Key 7 is in the last bucket (3): nothing follows.
        assert_eq!(step(&mut e, map, last), (0, 7, 0xeeee));
    }
}
