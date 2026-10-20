//! `BSCore/run_00aa5fc0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the two words at `this` and returns
/// `this` (a two-word constructor; `CombatCoverMap`'s constructor builds its
/// +0x14 member with it).
pub fn fn_00aa5fc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_5fc0, fn_00aa5fc0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_both_words_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00aa_5fc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
    }
}
