//! `fallout shared/pathfinding/pathinglosgridmap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006a62f0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const OFFSET_TARGET_006A62F0: u32 = 0x006a_62f0;

// Translated from 006e0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (callers `SaveGame` and `LoadGame` of the pathing LOS grid map):
/// forwards `this` and one word to `006a62f0`, which returns the word at `this+4` plus the
/// argument (`ADD EAX,[EBP+8]`). The wrapper returns that address.
pub fn fn_006e0ed0(e: &mut Engine, this: Ptr, offset: u32) -> Ptr {
    e.call(OFFSET_TARGET_006A62F0, &args![this, offset]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@D$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106CF0C: u32 = 0x0106_cf0c;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006e10a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@D$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006e10a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106CF0C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_0ed0, fn_006e0ed0(Ptr, u32) -> Ptr),
        entry!(0x006e_10a0, fn_006e10a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_006e0ed0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(OFFSET_TARGET_006A62F0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x006e_0ed0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == OFFSET_TARGET_006A62F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
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
    fn fn_006e10a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006e_10a0, ARRAY_VTABLE_0106CF0C, ARRAY_RELEASE_008454F0);
    }
}
