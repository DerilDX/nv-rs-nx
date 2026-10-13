//! `(unplaced)/run_005a8080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the embedded struct at `+0x3c` of `this` (`this + 0x3c`).
/// `TESQuest` calls it to reach its `QUEST_DATA`; the engine map has no name
/// for the address, and other classes may share the code.
pub fn fn_005a8080(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x3c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005a8080, fn_005a8080(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_0x3c_to_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005a_8080, &args![0x1000u32]).u32(), 0x103c);
    }
}
