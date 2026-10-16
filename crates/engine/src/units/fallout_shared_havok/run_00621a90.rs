//! `fallout shared/havok/run_00621a90` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00621a90`: thiscall on the same object; its result is the object the
/// next call runs on.
const FIRST_CALL: u32 = 0x0062_1a90;
/// `0044ddc0`: thiscall on that result; its result is the argument of
/// `00621480`.
const SECOND_CALL: u32 = 0x0044_ddc0;
/// `00621480`: cdecl, one word; the result is left in EAX.
const FINAL_CALL: u32 = 0x0062_1480;

// Translated from 00621ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: `00621480(0044ddc0(00621a90(this)))`, with the
/// callees run on the previous result as their `this`. Returns what
/// `00621480` returns.
pub fn fn_00621ad0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.call(FIRST_CALL, &args![this]).u32();
    let second = e.call(SECOND_CALL, &args![first]).u32();
    e.call(FINAL_CALL, &args![second]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00621ad0, fn_00621ad0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chains_the_three_calls_and_returns_the_last_result() {
        let mut e = Engine::new();
        e.register(FIRST_CALL, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(SECOND_CALL, |_, a| Ret {
            eax: a[0] * 10,
            ..Ret::default()
        });
        e.register(FINAL_CALL, |_, a| Ret {
            eax: a[0] + 5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0062_1ad0, &args![Ptr::<()>::new(2)]).u32(), 35);
    }
}
