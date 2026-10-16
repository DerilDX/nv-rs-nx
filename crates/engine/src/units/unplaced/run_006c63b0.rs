//! `(unplaced)/run_006c63b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006c63e0` calls on the member (`006c72f0`).
const CALLEE_006C72F0: u32 = 0x006c_72f0;

// Translated from 006c63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006c72f0` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_006c63e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006C72F0, &args![this.byte_add(0xc)]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_63e0, fn_006c63e0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006c63e0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006C72F0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006c_63e0, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006C72F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}
