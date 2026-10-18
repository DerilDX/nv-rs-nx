//! `(unplaced)/run_009d9f00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d9f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Fastcall (`ECX` = `this`): returns the address of the member at `this + 0xe0`
/// (`ADD EAX, 0xe0`), without reading memory.
pub fn fn_009d9f40(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xe0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009d_9f40, fn_009d9f40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_this_plus_0xe0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_9f40, &args![0x1000u32]).u32(), 0x10e0);
    }

    #[test]
    fn wraps_like_the_32_bit_add() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_9f40, &args![0xffff_ff80u32]).u32(), 0x60);
    }
}
