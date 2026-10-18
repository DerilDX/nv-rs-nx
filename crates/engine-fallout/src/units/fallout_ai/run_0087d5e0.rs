//! `fallout/ai/run_0087d5e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004f8960`: thiscall, returns an `int`.
const GET_VALUE: u32 = 0x004f_8960;

// Translated from 0087d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: whether `004f8960(this)` returns exactly 1.
pub fn fn_0087d6a0(e: &mut Engine, this: Ptr) -> bool {
    e.call(GET_VALUE, &args![this]).i32() == 1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0087d6a0, fn_0087d6a0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_only_for_exactly_one() {
        for (value, expected) in [(1u32, true), (0, false), (2, false), (u32::MAX, false)] {
            let mut e = Engine::new();
            e.register_double(GET_VALUE, move |_, _| Ret {
                eax: value,
                ..Ret::default()
            });
            assert_eq!(
                e.call(0x0087_d6a0, &args![Ptr::<()>::new(0x10)]).bool(),
                expected
            );
        }
    }
}
