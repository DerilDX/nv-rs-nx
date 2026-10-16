//! `fallout shared/facegen/run_0064dc30` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0064de90` calls on the member (`0064e2d0`).
const CALLEE_0064E2D0: u32 = 0x0064_e2d0;

// Translated from 0064de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0064e2d0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0064de90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0064E2D0, &args![this.byte_add(0x8)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_de90, fn_0064de90(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0064de90_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0064E2D0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_de90, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0064E2D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
}
