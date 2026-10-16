//! `fallout shared/pathfinding/navmeshpovsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3B0: u32 = 0x0106_b3b0;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3D4: u32 = 0x0106_b3d4;
/// `0049faf0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_0049FAF0: u32 = 0x0049_faf0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3E8: u32 = 0x0106_b3e8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3FC: u32 = 0x0106_b3fc;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B410: u32 = 0x0106_b410;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B424: u32 = 0x0106_b424;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B44C: u32 = 0x0106_b44c;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B460: u32 = 0x0106_b460;

// Translated from 006a14e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a14e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3B0);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a1700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a1700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3D4);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1890(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a18e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a18e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3FC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a19e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B410);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1a30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B424);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2580(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B44C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2900(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B460);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_14e0, fn_006a14e0(Ptr)),
        entry!(0x006a_1700, fn_006a1700(Ptr)),
        entry!(0x006a_1890, fn_006a1890(Ptr)),
        entry!(0x006a_18e0, fn_006a18e0(Ptr)),
        entry!(0x006a_19e0, fn_006a19e0(Ptr)),
        entry!(0x006a_1a30, fn_006a1a30(Ptr)),
        entry!(0x006a_2580, fn_006a2580(Ptr)),
        entry!(0x006a_2900, fn_006a2900(Ptr)),
    ]
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
    fn fn_006a14e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_14e0, ARRAY_VTABLE_0106B3B0, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a1700_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1700, ARRAY_VTABLE_0106B3D4, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1890_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1890, ARRAY_VTABLE_0106B3E8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a18e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_18e0, ARRAY_VTABLE_0106B3FC, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a19e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_19e0, ARRAY_VTABLE_0106B410, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1a30_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1a30, ARRAY_VTABLE_0106B424, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2580_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2580, ARRAY_VTABLE_0106B44C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2900_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2900, ARRAY_VTABLE_0106B460, ARRAY_RELEASE_008454F0);
    }
}
