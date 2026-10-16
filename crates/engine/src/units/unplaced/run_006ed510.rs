//! `(unplaced)/run_006ed510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006ed510` calls on the member (`006f25d0`).
const CALLEE_006F25D0: u32 = 0x006f_25d0;

// Translated from 006ed510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006f25d0` with `ECX` = `this + 0x2c` and passes on nothing (the callee returns no value).
pub fn fn_006ed510(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_006F25D0, &args![this.byte_add(0x2c)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_d510, fn_006ed510(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006ed510_passes_this_plus_2c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006F25D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_d510, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006F25D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x2c]);
    }
}
