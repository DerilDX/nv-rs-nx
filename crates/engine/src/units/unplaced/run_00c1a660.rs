//! `(unplaced)/run_00c1a660` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_012024D4: u32 = 0x0120_24d4;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00C1AA00: u32 = 0x00c1_aa00;
const EXIT_HANDLER_ARGUMENT_00C1A7D0: u32 = 0x00c1_a7d0;

// Translated from 00c1ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `012024d4`; when it was 0, calls
/// `00aa2080(00c1aa00, 00c1a7d0)`. Returns `this`, which it does not otherwise read.
pub fn fn_00c1ad50(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_012024D4);
    e.set_global(USE_COUNT_012024D4, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00C1AA00, EXIT_HANDLER_ARGUMENT_00C1A7D0],
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c1_ad50, fn_00c1ad50(Ptr) -> Ptr)]
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
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_ad50, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00c1_aa00, 0x00c1_a7d0]]
        );
        e.call(0x00c1_ad50, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}
