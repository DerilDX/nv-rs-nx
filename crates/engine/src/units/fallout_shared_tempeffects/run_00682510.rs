//! `fallout shared/tempeffects/run_00682510` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006825e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00682600`, a destructor with an exception frame
/// that clears `+8` through `00528820`, frees the object at `+0xc` through `006826a0` and
/// finishes through `00682680`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `00682600`'s
/// disassembly ends with its exception-frame unlink and returns no value (Ghidra types it
/// void).
pub fn fn_006825e0(e: &mut Engine, this: Ptr) {
    e.call(0x0068_2600, &args![this]);
}

// Translated from 00682680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006826d0`, which frees the block held at `+0`
/// through the cdecl `004ede70`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `006826d0`'s
/// disassembly ends with the stack cleanup of that call (`ADD ESP,4`) and computes no result
/// (Ghidra types it void).
pub fn fn_00682680(e: &mut Engine, this: Ptr) {
    e.call(0x0068_26d0, &args![this]);
}

/// The constructor body `00682530` (`this` plus one stack word, `RET 4`).
const CONSTRUCTOR_BODY_00682530: u32 = 0x0068_2530;

// Translated from 00682510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: forwards `this` and its one stack word to `00682530` and
/// returns `this`.
pub fn fn_00682510(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(CONSTRUCTOR_BODY_00682530, &args![this, argument]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_25e0, fn_006825e0(Ptr)),
        entry!(0x0068_2680, fn_00682680(Ptr)),
        entry!(0x0068_2510, fn_00682510(Ptr, u32) -> Ptr),
    ]
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
    fn fwd_a03_006825e0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0068_2600, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_25e0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0068_2600), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00682680_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0068_26d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_2680, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0068_26d0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn forwards_this_and_the_word_and_returns_this() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_00682530, |_, _| Ret {
            eax: 0x999,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0068_2510, &args![0x2000u32, 0x33u32]).u32(),
            0x2000
        );
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_00682530),
            vec![vec![0x2000, 0x33]]
        );
    }
}
