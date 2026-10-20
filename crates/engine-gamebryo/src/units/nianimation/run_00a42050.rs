//! `NiAnimation/run_00a42050` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::set_literal_fixed_string;

// Translated from 00a421c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiAnimation subsystem; reached by the jump
/// that ends `fn_00a2d8a0`; no arguments): sets the static fixed strings at `011f3ec0` and
/// `011f3ec4` from the literals at `010977f4` and `010977ec`, each only when the word does
/// not hold the literal's address already.
pub fn fn_00a421c0(e: &mut Engine) {
    set_literal_fixed_string(e, 0x011f_3ec0, 0x0109_77f4);
    set_literal_fixed_string(e, 0x011f_3ec4, 0x0109_77ec);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_21c0, fn_00a421c0())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_both_strings_unless_already_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(0x011f_3ec0, empty);
        e.mem.set_u32(0x011f_3ec4, 0x0109_77ec);
        e.call_log = Some(vec![]);
        e.call(0x00a4_21c0, &args![]);
        assert_eq!(e.mem.u32(0x011f_3ec0), 0x0109_77f4 + 0x1000);
        assert_eq!(e.mem.u32(0x011f_3ec4), 0x0109_77ec);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x0109_77f4]]);
    }
}
