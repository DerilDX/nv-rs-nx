//! `(unplaced)/run_005f9b70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f9b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x12C (`thiscall`; no Xbox PDB name).
pub fn fn_005f9b70(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x12c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_9b70, fn_005f9b70(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_member_at_0x12c() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005f_9b70, &args![0x1000u32]).u32(), 0x112c);
    }
}
