//! `fallout shared/tesreactionform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048bb10` (Ghidra name
/// `TESReactionForm::FreeReactionList`), which empties the reaction list node by node. The
/// Xbox PDB name `TESReactionForm::ClearDataComponent` is the class's vslot +0x004 (pdbdump
/// `type ... TESReactionForm`; engine map row `vt 822dd550`), and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0048bb10`'s disassembly is a loop that ends on its exit test
/// without loading a result (Ghidra types it void).
pub fn tes_reaction_form_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_bb10, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0048_baf0,
        tes_reaction_form_clear_data_component(Ptr)
    )]
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
    fn fwd_a03_0048baf0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_bb10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_baf0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_bb10), vec![vec![0x5000]]);
    }
}
