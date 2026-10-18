//! `(unplaced)/run_00c90350` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c90350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method (it is only referenced from vtables) that
/// ignores `this` and returns 0.
pub fn fn_00c90350(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c90350, fn_00c90350(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c90350_returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_0350, &args![Ptr::<()>::new(0x10)]).u32(), 0);
    }
}
