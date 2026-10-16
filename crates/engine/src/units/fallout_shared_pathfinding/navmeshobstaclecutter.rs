//! `fallout shared/pathfinding/navmeshobstaclecutter.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bc300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ObstacleUndoData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006bc330` and frees 0x70 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn obstacle_undo_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006b_c330, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x70u32]);
    }
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UBoundsTriangleInfo@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106C2E8: u32 = 0x0106_c2e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UPortalEdgeSwap@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106C2FC: u32 = 0x0106_c2fc;

// Translated from 006bf6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UBoundsTriangleInfo@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006bf6c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006bf710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UPortalEdgeSwap@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006bf710(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2FC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_c300, obstacle_undo_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_f6c0, fn_006bf6c0(Ptr)),
        entry!(0x006b_f710, fn_006bf710(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn obstacle_undo_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006b_c300, 0x006b_c330, 0x70);
    }
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
    fn fn_006bf6c0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006b_f6c0, ARRAY_VTABLE_0106C2E8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006bf710_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006b_f710, ARRAY_VTABLE_0106C2FC, ARRAY_RELEASE_008454F0);
    }
}
