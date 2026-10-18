//! `(unplaced)/run_0076b6a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0057bd10` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_0057BD10: u32 = 0x0057_bd10;

// Translated from 0076b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `0057bd10(this, 0)`, then clears the word at `+0xc`.
/// Returns `this`.
pub fn fn_0076b930(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0057BD10, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076_b930, fn_0076b930(Ptr) -> Ptr)]
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
    fn calls_the_base_then_clears_the_word() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0057BD10, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0x5555);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_b930, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0057BD10),
            vec![vec![this, 0]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}
