//! `(unplaced)/run_00aa9d00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aaa650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 and ignores its three stack words (`__thiscall`, `RET 0xC`;
/// `XOR EAX,EAX`). The map gives it no name; it is, for example, slot +0xC
/// of the `BSSaveDataSystemUtility` vtable (`010a4288`), which `0084fc70`
/// calls with `"-SAVE-"`.
pub fn fn_00aaa650(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aaa650, fn_00aaa650(Ptr, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_for_any_arguments() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let back = e.call(0x00aa_a650, &args![this, 0x0107_fabcu32, 1u32, 2u32]);
        assert_eq!(back.u32(), 0);
    }
}
