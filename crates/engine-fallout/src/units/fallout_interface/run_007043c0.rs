//! `fallout/interface/run_007043c0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007043c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): answers false (`AL` = 0;
/// the rest of `EAX` is not set). Its callers pass words it does not read.
pub fn fn_007043c0(_e: &mut Engine) -> bool {
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_43c0, fn_007043c0() -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_007043c0_answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x0070_43c0, &args![]).bool());
        assert!(!e.call(0x0070_43c0, &args![0x1234u32]).bool());
    }
}
