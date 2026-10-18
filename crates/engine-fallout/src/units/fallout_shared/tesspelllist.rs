//! `fallout shared/tesspelllist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this + 0xC` (`thiscall`, no arguments): the address of a member
/// at `+0xC`. The engine map gives this folded body no name; its many
/// callers use it on different classes (in `TESSpellList` that offset is
/// `leveledSpellList`, Xbox PDB).
pub fn fn_0048d150(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xc))
}

// Translated from 0048d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048d0f0` (Ghidra name
/// `TESSpellList::FreeSpellList`), which empties the spell lists. The Xbox PDB name
/// `TESSpellList::ClearDataComponent` is the class's vslot +0x004 (pdbdump `type ...
/// TESSpellList`; engine map row `vt 822ded00`), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0048d0f0`'s disassembly ends on a loop exit without loading a result (Ghidra types it
/// void).
pub fn tes_spell_list_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_d0f0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_d0d0, tes_spell_list_clear_data_component(Ptr)),
        entry!(0x0048d150, fn_0048d150(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_at_c() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0048_d150, &args![0x0000_5000u32]).u32(),
            0x0000_500c
        );
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0048d0d0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_d0f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_d0d0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_d0f0), vec![vec![0x5000]]);
    }
}
