//! `fallout/interface/menus/caravanmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVNiAVObject@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071AEC: u32 = 0x0107_1aec;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESCaravanCard@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071B00: u32 = 0x0107_1b00;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCaravanAIPackage@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071B14: u32 = 0x0107_1b14;

// Translated from 007530a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVNiAVObject@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_007530a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071AEC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 007530f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESCaravanCard@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_007530f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B00);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00753210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCaravanAIPackage@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00753210(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B14);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_30a0, fn_007530a0(Ptr)),
        entry!(0x0075_30f0, fn_007530f0(Ptr)),
        entry!(0x0075_3210, fn_00753210(Ptr)),
        entry!(0x0073_cb80, fn_0073cb80(Ptr) -> u8),
        entry!(0x0074_08d0, fn_007408d0(Ptr, i32) -> bool),
        entry!(0x0074_0940, fn_00740940(Ptr) -> u32),
    ]
}

// Translated from 0073cb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the byte at `this + 0x98` (in `AL`).
pub fn fn_0073cb80(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x98)
}

// Translated from 007408d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: when `value` is 2 or 0 stores it at `this + 0x24` and returns
/// `true`; any other value is refused (`false`, nothing stored). `RET 4`.
pub fn fn_007408d0(e: &mut Engine, this: Ptr, value: i32) -> bool {
    if value == 2 || value == 0 {
        e.mem.set_i32(this.addr() + 0x24, value);
        true
    } else {
        false
    }
}

// Translated from 00740940 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x624`.
pub fn fn_00740940(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x624)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0073cb80_reads_the_byte_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x98, 0xaabb_cc07);
        assert_eq!(e.call(0x0073_cb80, &args![this]).u32(), 7);
    }

    #[test]
    fn fn_007408d0_accepts_only_two_and_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        for (value, accepted) in [(2i32, true), (0, true), (1, false), (-1, false), (3, false)] {
            e.mem.set_u32(this + 0x24, 0x55);
            let result = e.call(0x0074_08d0, &args![this, value]).bool();
            assert_eq!(result, accepted, "value {value}");
            let stored = if accepted { value as u32 } else { 0x55 };
            assert_eq!(e.mem.u32(this + 0x24), stored);
        }
    }

    #[test]
    fn fn_00740940_reads_the_word_at_0x624() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x628);
        e.mem.set_u32(this + 0x624, 0x1234_5678);
        assert_eq!(e.call(0x0074_0940, &args![this]).u32(), 0x1234_5678);
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
    fn fn_007530a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_30a0, ARRAY_VTABLE_01071AEC, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_007530f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_30f0, ARRAY_VTABLE_01071B00, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_00753210_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_3210, ARRAY_VTABLE_01071B14, ARRAY_RELEASE_008454F0);
    }
}
