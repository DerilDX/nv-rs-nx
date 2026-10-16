//! `(unplaced)/run_0083d8b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0083e0a0` calls on the member (`004048e0`).
const CALLEE_004048E0: u32 = 0x0040_48e0;

// Translated from 0083e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `004048e0` with `ECX` = `this + 0x18` and passes on its result.
pub fn fn_0083e0a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_004048E0, &args![this.byte_add(0x18)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_e0a0, fn_0083e0a0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0083e0a0_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_004048E0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e0a0, &args![0x4100u32]).u32(), 0x144d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_004048E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}
