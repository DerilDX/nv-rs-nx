//! `fallout shared/pathfinding/pathingrequest.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e25e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006e2620` and frees 0xb0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_2620, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 006e3a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `006e39d0` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_006e3a50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_39d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VPathingAvoidNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D218: u32 = 0x0106_d218;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006e3ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VPathingAvoidNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006e3ab0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D218);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006e2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`pathingrequest.cpp`). `thiscall`, `RET 4`: stores the byte
/// argument at `+0x98` of `this`.
pub fn fn_006e2b50(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x98), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_2b50, fn_006e2b50(Ptr, u8)),
        entry!(0x006e_25e0, pathing_request_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_3a50, fn_006e3a50(Ptr, u32) -> Ptr),
        entry!(0x006e_3ab0, fn_006e3ab0(Ptr)),
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
    fn pathing_request_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_25e0, 0x006e_2620, 0xb0);
    }

    #[test]
    fn fn_006e3a50_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_3a50, 0x006e_39d0, 0x18);
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
    fn fn_006e3ab0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006e_3ab0, ARRAY_VTABLE_0106D218, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_006e2b50_stores_the_byte_at_offset_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x98, 0x1122_3344);
        e.call(0x006e_2b50, &args![this, 0xa5u8]);
        // Only the low byte changes.
        assert_eq!(e.mem.u32(this + 0x98), 0x1122_33a5);
        e.call(0x006e_2b50, &args![this, 0u8]);
        assert_eq!(e.mem.u32(this + 0x98), 0x1122_3300);
    }
}
