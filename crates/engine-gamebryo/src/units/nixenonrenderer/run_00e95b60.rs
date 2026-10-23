//! `NiXenonRenderer/run_00e95b60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e95b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the engine map; its class is not confirmed),
/// thiscall without stack words: sets the word at `+0` to -1, the word at
/// `+4` to 0xffff and clears the words at `+8` and `+0x10` (the word at
/// `+0xc` is left alone). Answers `this`.
pub fn fn_00e95b60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0xffff_ffff);
    e.mem.set_u32(base + 4, 0xffff);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e95b60, fn_00e95b60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_initial_words_and_keeps_0xc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(this + 4 * word, 0x1234_5678);
        }
        assert_eq!(e.call(0x00e9_5b60, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234_5678);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}
