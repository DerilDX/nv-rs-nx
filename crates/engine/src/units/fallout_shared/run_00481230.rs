//! `fallout shared/run_00481230` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004812d0` calls on the member (`0048cee0`).
const CALLEE_0048CEE0: u32 = 0x0048_cee0;

// Translated from 004812d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0048cee0` with `ECX` = `this + 0x4` and passes on its result.
pub fn fn_004812d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0048CEE0, &args![this.byte_add(0x4)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_12d0, fn_004812d0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004812d0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0048CEE0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_12d0, &args![0x4100u32]).u32(), 0x1451);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0048CEE0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}
