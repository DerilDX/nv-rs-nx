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
