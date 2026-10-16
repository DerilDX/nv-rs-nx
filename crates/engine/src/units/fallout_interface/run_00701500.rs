//! `fallout/interface/run_00701500` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00701500` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00701500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x18` and passes on nothing (the callee returns no value).
pub fn fn_00701500(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x18)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_1500, fn_00701500(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00701500_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_1500, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}
