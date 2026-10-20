//! `fallout shared/pathfinding/navmeshsearchlos.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `006aabf0` returns (+0x39 of its `this`).
const FLAG_BYTE: u32 = 0x39;

// Translated from 006aabf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter (unnamed in the engine map): the byte at +0x39 of `this`.
pub fn fn_006aabf0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + FLAG_BYTE)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_abf0, fn_006aabf0(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006aabf0_returns_the_byte_at_0x39() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + FLAG_BYTE, 0xa5);
        assert_eq!(e.call(0x006a_abf0, &args![this]).u32() & 0xff, 0xa5);
        e.mem.set_u8(this + FLAG_BYTE, 0);
        assert_eq!(e.call(0x006a_abf0, &args![this]).u32() & 0xff, 0);
    }
}
