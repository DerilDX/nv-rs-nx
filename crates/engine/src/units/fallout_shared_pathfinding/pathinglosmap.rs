//! `fallout shared/pathfinding/pathinglosmap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map base constructor: installs its vtable, allocates buckets and
/// clears them. The bucket count is reread after allocation.
pub fn construct_base(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106cf88);
    e.mem.set_u32(this.addr().wrapping_add(4), buckets);
    e.mem.set_u32(this.addr().wrapping_add(12), 0);
    let size = e.mem.u32(this.addr().wrapping_add(4)).wrapping_mul(4);
    let table = e.call(0x00aa1070, &args![size]).u32();
    e.mem.set_u32(this.addr().wrapping_add(8), table);
    let size = e.mem.u32(this.addr().wrapping_add(4)).wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00403d30, &args![table, 0u32, size]);
    this
}

// Translated from 006e2080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map constructor: constructs the base then installs its vtable.
pub fn construct(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    construct_base(e, this, buckets);
    e.mem.set_u32(this.addr(), 0x0106cf68);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e2080, construct(Ptr, u32) -> Ptr),
        entry!(0x006e20e0, construct_base(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_reloads_bucket_count_after_allocator_and_wraps_sizes() {
        for (address, final_vtable) in [(0x006e20e0, 0x0106cf88), (0x006e2080, 0x0106cf68)] {
            let mut e = Engine::new();
            let map = e.mem.alloc(20);
            let table = e.mem.alloc(8);
            e.mem.write(map, &[0xaa; 20]);
            e.mem.write(table, &[0xbb; 8]);
            e.register_double(0x00aa1070, move |e, a| {
                assert_eq!(a, &[4]); // 0x40000001 * 4 wraps.
                assert_eq!(e.mem.u32(map), 0x0106cf88);
                assert_eq!(e.mem.u32(map + 12), 0);
                e.mem.set_u32(map + 4, 2);
                table.into_ret()
            });
            assert_eq!(e.call(address, &args![map, 0x40000001u32]).u32(), map);
            assert_eq!(e.mem.u32(map), final_vtable);
            assert_eq!(e.mem.u32(map + 4), 2);
            assert_eq!(e.mem.u32(map + 8), table);
            assert_eq!(e.mem.u32(map + 16), 0xaaaaaaaa);
            assert_eq!(e.mem.bytes(table, 8), [0; 8]);
        }
    }
}
