//! `fallout shared/run_005194d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00519550` calls on the member (`00470470`).
const CALLEE_00470470: u32 = 0x0047_0470;

// Translated from 00519550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00470470` with `ECX` = `this + 0x88` and passes on nothing (the callee returns no value).
pub fn fn_00519550(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00470470, &args![this.byte_add(0x88)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_9550, fn_00519550(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00519550_passes_this_plus_88_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00470470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_9550, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00470470).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x88]);
    }
}
