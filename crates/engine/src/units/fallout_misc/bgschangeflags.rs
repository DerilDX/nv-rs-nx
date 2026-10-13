//! `fallout/misc/saveload/bgschangeflags.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c71b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores the word it is given at `this` and returns `this` (`thiscall`,
/// `RET 4`). The map names it `BGSChangeFlags::IsForcedChange`, but the
/// linker folded identical code here: the body is a one-word constructor
/// (Ghidra matches it to `std::locale::id::id`). `TESForm::ForceChange`
/// (`00484ba2`) builds its change-flags word with it, so every form
/// constructor that flags a change (the quest's enable setter `0060c9c0`)
/// runs it.
pub fn fn_008c71b0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c71b0, fn_008c71b0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_and_returns_this() {
        let mut e = Engine::new();
        let p = e.mem.alloc(8);
        e.mem.set_u32(p + 4, 0x5555_5555);
        assert_eq!(e.call(0x008c71b0, &[p, 0x4000_0002]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x4000_0002);
        assert_eq!(e.mem.u32(p + 4), 0x5555_5555);
    }
}
