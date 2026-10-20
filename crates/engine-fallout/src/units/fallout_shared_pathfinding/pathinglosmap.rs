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

/// `NiTMap<unsigned int, unsigned int>`'s vtable (`0106cf68`) and its
/// base `NiTMapBase<DFALL<NiTMapItem<unsigned int, unsigned int>>, ...>`'s
/// (`0106cf88`), by their RTTI.
const VTABLE_NI_T_MAP: u32 = 0x0106_cf68;
const VTABLE_NI_T_MAP_BASE: u32 = 0x0106_cf88;
/// `00438af0`: removes every item of the map (`thiscall`).
const REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0`: frees the bucket table (`cdecl`, one word).
const FREE_TABLE: u32 = 0x00aa_10f0;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 006e20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, unsigned int>`'s scalar deleting destructor
/// (engine map): [`fn_006e2150`], then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e2150(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e2150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e2150 - the C++ exception frame (FS:[0]) is not translated
/// The `NiTMap<unsigned int, unsigned int>` destructor (by the vtable it
/// sets): removes every item (`00438af0`), then the base destructor
/// ([`fn_006e21b0`]).
pub fn fn_006e2150(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP);
    e.call(REMOVE_ALL, &args![this]);
    fn_006e21b0(e, this);
}

// Translated from 006e21b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor (by the vtable it sets): removes every item
/// (`00438af0`), then frees the bucket table at `+8` (`00aa10f0`).
pub fn fn_006e21b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_BASE);
    e.call(REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(FREE_TABLE, &args![table]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e2080, construct(Ptr, u32) -> Ptr),
        entry!(0x006e20e0, construct_base(Ptr, u32) -> Ptr),
        entry!(0x006e_20b0, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_2150, fn_006e2150(Ptr)),
        entry!(0x006e_21b0, fn_006e21b0(Ptr)),
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

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn destructors_clear_free_and_delete() {
        let mut e = Engine::new();
        e.register_double(REMOVE_ALL, |e, a| {
            // The vtable in place while the items go.
            let vtable = e.mem.u32(a[0]);
            assert!(vtable == VTABLE_NI_T_MAP || vtable == VTABLE_NI_T_MAP_BASE);
            Ret::default()
        });
        e.register(FREE_TABLE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4240);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_20b0, &args![map, 1u32]).u32(), map);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (REMOVE_ALL, vec![map]),
                (REMOVE_ALL, vec![map]),
                (FREE_TABLE, vec![0x4240]),
                (OPERATOR_DELETE, vec![map]),
            ]
        );
        assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE);
        e.call_log = Some(vec![]);
        e.call(0x006e_20b0, &args![map, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != OPERATOR_DELETE));
    }
}
