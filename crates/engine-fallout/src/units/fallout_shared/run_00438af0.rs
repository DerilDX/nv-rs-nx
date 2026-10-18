//! `fallout shared/run_00438af0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slot (byte offset) called on the table with each removed entry,
/// first.
const FIRST_SLOT: u32 = 0x10;
/// Virtual slot (byte offset) called with the same entry, second.
const SECOND_SLOT: u32 = 0x18;

// Translated from 00438af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a chained hash table: the word at `+4` is the bucket count, the
/// word at `+8` the bucket array, each bucket the head of a singly linked
/// chain whose entries keep the next entry in their first word. Every entry
/// is unlinked from the front of its bucket and handed to the virtual
/// methods at vtable `+0x10` and then `+0x18` (their names are not known).
/// Finally the word at `+0xc` (the entry count) is zeroed.
pub fn fn_00438af0(e: &mut Engine, this: Ptr) {
    let mut bucket = 0u32;
    while bucket < e.mem.u32(this.addr() + 4) {
        loop {
            let buckets = e.mem.u32(this.addr() + 8);
            let entry = e.mem.u32(buckets + 4 * bucket);
            if entry == 0 {
                break;
            }
            let next = e.mem.u32(entry);
            e.mem.set_u32(buckets + 4 * bucket, next);
            e.vcall(this.addr(), FIRST_SLOT, &args![entry]);
            e.vcall(this.addr(), SECOND_SLOT, &args![entry]);
        }
        bucket += 1;
    }
    e.mem.set_u32(this.addr() + 0xc, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00438af0, fn_00438af0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0300_0000;
    const FIRST: u32 = 0x0070_0000;
    const SECOND: u32 = 0x0070_0010;

    /// A table with three buckets: bucket 0 holds entries a -> b, bucket 1 is
    /// empty, bucket 2 holds c. Returns (table, [a, b, c]).
    fn table(e: &mut Engine) -> (Ptr, [u32; 3]) {
        let mut slots = vec![0; 0x18 / 4 + 1];
        slots[0x10 / 4] = FIRST;
        slots[0x18 / 4] = SECOND;
        e.put_vtable(VTABLE, &slots);
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        let buckets = e.mem.alloc(12);
        let entries = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + 4, 3);
        e.mem.set_u32(this.addr() + 8, buckets);
        e.mem.set_u32(this.addr() + 0xc, 3);
        e.mem.set_u32(buckets, entries[0]);
        e.mem.set_u32(entries[0], entries[1]);
        e.mem.set_u32(buckets + 8, entries[2]);
        (this, entries)
    }

    #[test]
    fn unlinks_and_releases_every_entry_front_to_back() {
        let mut e = Engine::new();
        e.register(FIRST, |_, _| Ret::default());
        e.register(SECOND, |_, _| Ret::default());
        let (this, [a, b, c]) = table(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_8af0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            &log[1..],
            &[
                (FIRST, vec![this.addr(), a]),
                (SECOND, vec![this.addr(), a]),
                (FIRST, vec![this.addr(), b]),
                (SECOND, vec![this.addr(), b]),
                (FIRST, vec![this.addr(), c]),
                (SECOND, vec![this.addr(), c]),
            ]
        );
        let buckets = e.mem.u32(this.addr() + 8);
        assert_eq!(e.mem.u32(buckets), 0);
        assert_eq!(e.mem.u32(buckets + 8), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }

    #[test]
    fn an_empty_table_only_zeroes_the_count() {
        let mut e = Engine::new();
        let (this, _) = table(&mut e);
        e.mem.set_u32(this.addr() + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_8af0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }
}
