//! `fallout shared/run_0044a170` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0044a380` calls on the member (`0044a560`).
const CALLEE_0044A560: u32 = 0x0044_a560;

// Translated from 0044a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044a560` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_0044a380(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_0044A560, &args![this.byte_add(0xc)]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_a380, fn_0044a380(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0044a380_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044A560, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_a380, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044A560).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}
