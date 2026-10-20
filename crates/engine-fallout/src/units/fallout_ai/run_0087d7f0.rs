//! `fallout/ai/run_0087d7f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0087d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the word at `this + 0x1a8`.
pub fn fn_0087d7f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1a8)
}

// Translated from 0087d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores `value` at `this + 0x1a8`.
pub fn fn_0087d810(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x1a8, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_d7f0, fn_0087d7f0(Ptr) -> u32),
        entry!(0x0087_d810, fn_0087d810(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_at_1a8_round_trips() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1b0);
        e.call(0x0087_d810, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 0x1a8), 0xdead_beef);
        assert_eq!(e.call(0x0087_d7f0, &args![this]).u32(), 0xdead_beef);
    }
}
