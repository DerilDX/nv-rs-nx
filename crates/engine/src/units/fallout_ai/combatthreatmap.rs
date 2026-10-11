//! `fallout/ai/combat/combatthreatmap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00844700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::SetAt` (Xbox PDB, folded instantiation): replaces a
/// value or prepends a new item to the selected bucket. Hash, equality,
/// allocation and key/value storage dispatch through the supplied vtable.
pub fn set_at(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket));
    while node != 0 {
        let existing_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, existing_key]).u8() != 0 {
            e.mem.set_u32(node.wrapping_add(8), value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(this.addr(), 0x14, &args![]).u32();
    e.vcall(this.addr(), 0xc, &args![node, key, value]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let next = e.mem.u32(table.wrapping_add(bucket));
    e.mem.set_u32(node, next);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.mem.set_u32(table.wrapping_add(bucket), node);
    let count = e.mem.u32(this.addr().wrapping_add(0xc));
    e.mem
        .set_u32(this.addr().wrapping_add(0xc), count.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00844700, set_at(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let map = e.mem.alloc(16);
        let table = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u32(vtable + 4, 0xf0100000);
        e.mem.set_u32(vtable + 8, 0xf0100004);
        e.mem.set_u32(vtable + 0xc, 0xf0100008);
        e.mem.set_u32(vtable + 0x14, 0xf010000c);
        e.register_double(0xf0100000, move |_, a| {
            assert_eq!(a, &[map, 22]);
            1u32.into_ret()
        });
        (e, map, table, vtable)
    }

    #[test]
    fn lookup_reloads_buckets_links_and_value_after_callbacks() {
        let (mut e, map, old_table, _) = fixture();
        let table = e.mem.alloc(8);
        let first = e.mem.alloc(12);
        let last = e.mem.alloc(12);
        let out = e.mem.alloc(4);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(first + 4, 11);
        e.mem.set_u32(last + 4, 22);
        e.register_double(0xf0100000, move |e, a| {
            assert_eq!(a, &[map, 22]);
            e.mem.set_u32(map + 8, table);
            1u32.into_ret()
        });
        e.register_double(0xf0100004, move |e, a| {
            if a[2] == 11 {
                assert_eq!(a, &[map, 22, 11]);
                e.mem.set_u32(first, last);
                0x100u32.into_ret()
            } else {
                assert_eq!(a, &[map, 22, 22]);
                e.mem.set_u32(last + 8, 0x12345678);
                0x101u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00853130, &args![map, 22u32, out]).u8(), 1);
        assert_eq!(e.mem.u32(out), 0x12345678);
        assert_eq!(e.mem.u32(old_table + 4), 0);
        e.mem.set_u32(table + 4, 0);
        e.mem.set_u32(out, 0xdeadbeef);
        assert_eq!(e.call(0x00853130, &args![map, 22u32, out]).u8(), 0);
        assert_eq!(e.mem.u32(out), 0xdeadbeef);
    }

    #[test]
    fn set_existing_follows_callback_changed_link_and_does_not_add() {
        let (mut e, map, table, _) = fixture();
        let first = e.mem.alloc(12);
        let last = e.mem.alloc(12);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(first + 4, 11);
        e.mem.set_u32(last + 4, 22);
        e.mem.set_u32(map + 12, 2);
        e.register_double(0xf0100004, move |e, a| {
            assert_eq!(&a[..2], &[map, 22]);
            if a[2] == 11 {
                e.mem.set_u32(first, last);
                0x100u32.into_ret()
            } else {
                assert_eq!(a[2], 22);
                0x101u32.into_ret()
            }
        });
        e.call(0x00844700, &args![map, 22u32, 0xfedcba98u32]);
        assert_eq!(e.mem.u32(last + 8), 0xfedcba98);
        assert_eq!(e.mem.u32(map + 12), 2);
        assert_eq!(e.mem.u32(table + 4), first);
    }

    #[test]
    fn insertion_reloads_vtable_and_buckets_and_wraps_fresh_count() {
        let (mut e, map, old_table, _) = fixture();
        let table = e.mem.alloc(8);
        let new_vtable = e.mem.alloc(0x1c);
        let node = e.mem.alloc(12);
        let old_node = e.mem.alloc(12);
        e.mem.set_u32(new_vtable + 0xc, 0xf0100010);
        e.register_double(0xf010000c, move |e, a| {
            assert_eq!(a, &[map]);
            e.mem.set_u32(map, new_vtable);
            node.into_ret()
        });
        e.register_double(0xf0100010, move |e, a| {
            assert_eq!(a, &[map, node, 22, 77]);
            e.mem.set_u32(node + 4, a[2]);
            e.mem.set_u32(node + 8, a[3]);
            e.mem.set_u32(map + 8, table);
            e.mem.set_u32(table + 4, old_node);
            e.mem.set_u32(map + 12, u32::MAX);
            Ret::default()
        });
        e.call(0x00844700, &args![map, 22u32, 77u32]);
        assert_eq!(e.mem.u32(table + 4), node);
        assert_eq!(e.mem.u32(node), old_node);
        assert_eq!(e.mem.u32(node + 4), 22);
        assert_eq!(e.mem.u32(node + 8), 77);
        assert_eq!(e.mem.u32(map + 12), 0);
        assert_eq!(e.mem.u32(old_table + 4), 0);
    }
}
