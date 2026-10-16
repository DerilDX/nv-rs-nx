//! `fallout shared/pathfinding/run_006b5560` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006b5a90` calls on the member (`0044ddc0`).
const CALLEE_0044DDC0: u32 = 0x0044_ddc0;

// Translated from 006b5a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044ddc0` with `ECX` = `this + 0x58` and passes on its result.
pub fn fn_006b5a90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0044DDC0, &args![this.byte_add(0x58)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b_5a90, fn_006b5a90(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006b5a90_passes_this_plus_58_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DDC0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_5a90, &args![0x4100u32]).u32(), 0x140d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DDC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x58]);
    }
}
