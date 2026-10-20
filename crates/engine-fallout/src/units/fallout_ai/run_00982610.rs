//! `fallout/ai/run_00982610` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00982610 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder (no name in the engine map): hands `this` (`ECX`) to
/// `00aa5460` and returns nothing (plain `RET`, no stack words).
pub fn fn_00982610(e: &mut Engine, this: Ptr) {
    e.call(0x00aa_5460, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_2610, fn_00982610(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00982610_passes_this_on() {
        let mut e = Engine::new();
        e.register(0x00aa_5460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_2610, &args![0x4440u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00aa_5460).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4440]);
    }
}
