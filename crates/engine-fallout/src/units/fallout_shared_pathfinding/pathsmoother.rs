//! `fallout shared/pathfinding/pathsmoother.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UAvoidNode@PathSmoother@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D88C: u32 = 0x0106_d88c;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006f2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UAvoidNode@PathSmoother@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f2e00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D88C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `0042f5d0` (cdecl, two words: the buffer and its new size in bytes; it
/// runs `00401020` then `00aa4150` on them, the heap's reallocation).
const BUFFER_REALLOCATE_0042F5D0: u32 = 0x0042_f5d0;

// Translated from 006f2e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathSmoother::AvoidNode,1024>` instance's buffer
/// reallocation (thiscall, `RET 8`; `this` unused): `0042f5d0(buffer,
/// count * 16)`, the elements being 16 bytes. Returns nothing.
pub fn fn_006f2e20(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    e.call(BUFFER_REALLOCATE_0042F5D0, &args![buffer, count << 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_2e00, fn_006f2e00(Ptr)),
        entry!(0x006f_2e20, fn_006f2e20(Ptr, u32, u32)),
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
    fn fn_006f2e00_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_2e00, ARRAY_VTABLE_0106D88C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006f2e20_reallocates_sixteen_bytes_per_element() {
        let mut e = Engine::new();
        e.register(BUFFER_REALLOCATE_0042F5D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_2e20, &args![0x1234u32, 0x0abc_0000u32, 5u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == BUFFER_REALLOCATE_0042F5D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x0abc_0000, 0x50]);
    }
}
