//! `fallout/ai/modifierlist.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this`. The engine map has no name for it; the quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_009373f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr())
}

// Translated from 00937b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `009370b0` (Ghidra name
/// `ModifierList::DeleteAllModifiers`), which deletes every item of the list and the
/// fast-modifier collection at `+0xc`. The engine map names this address
/// `ModifierList::Revert` from the call graph only (row `cg`), which this translation does
/// not confirm, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `009370b0`'s disassembly ends on
/// the store `MOV dword ptr [ECX+0xc],0x0` with no result (Ghidra types it void).
pub fn fn_00937b30(e: &mut Engine, this: Ptr) {
    e.call(0x0093_70b0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0093_7b30, fn_00937b30(Ptr)),
        entry!(0x009373f0, fn_009373f0(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_first_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x1122_33a5);
        assert_eq!(e.call(0x0093_73f0, &[object]).u32() & 0xff, 0xa5);
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
    fn fwd_a03_00937b30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0093_70b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0093_7b30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0093_70b0), vec![vec![0x5000]]);
    }
}
