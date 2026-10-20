//! `BSMain/run_00c54030` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c54030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual function (virtual table word `010c2adc`) that
/// answers `this`; `thiscall`, `RET 4` (the stack word is not read).
pub fn fn_00c54030(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c5_4030, fn_00c54030(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_4030, &args![0x1234u32, 9u32]).u32(), 0x1234);
    }
}
