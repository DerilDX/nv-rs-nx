//! `fallout/interface/menus/creditsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 007613f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map files it under `creditsmenu.cpp`, a
/// range placement; the body is not menu code): stores the handle of the empty
/// `NiFixedString` (the word at `0109b220`) in the word at `this` and returns
/// `this`, the default constructor of an `NiFixedString`-sized object (the stream
/// code uses it as the element constructor of its fixed string array). The
/// decompiler shows the stored value as 0; the disassembly reads the global.
pub fn fn_007613f0(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr(), empty);
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01072514: u32 = 0x0107_2514;
/// `007615d0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007615D0: u32 = 0x0076_15d0;

// Translated from 007614a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007615d0(this, 1)`.
pub fn fn_007614a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01072514);
    e.call(ARRAY_RELEASE_007615D0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_13f0, fn_007613f0(Ptr) -> Ptr),
        entry!(0x0076_14a0, fn_007614a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_becomes_the_empty_string_handle() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.set_global(EMPTY_FIXED_STRING, 0x0123_4568u32);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0xdead_beef);
        assert_eq!(e.call(0x0076_13f0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0123_4568);
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
    fn fn_007614a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0076_14a0, ARRAY_VTABLE_01072514, ARRAY_RELEASE_007615D0);
    }
}
