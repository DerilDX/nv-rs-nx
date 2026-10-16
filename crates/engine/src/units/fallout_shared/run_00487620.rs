//! `fallout shared/run_00487620` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00476c00`: returns a function-local static object (guard bit at
/// `011c4b4c`, constructed at `011c4180` on first use).
const GET_STATIC_OBJECT: u32 = 0x0047_6c00;
/// `00476be0`: thiscall on that object; its body is `00aa5230(this, -1)`.
const CALL_WITH_MINUS_ONE: u32 = 0x0047_6be0;

// Translated from 00487f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name; other translations in this crate call it the random
/// number (not confirmed here). Calls `00476c00`, then `00476be0` on the
/// object it returned, and returns that call's result in EAX.
pub fn fn_00487f50(e: &mut Engine) -> u32 {
    let object = e.call(GET_STATIC_OBJECT, &args![]).u32();
    e.call(CALL_WITH_MINUS_ONE, &args![object]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00487f50, fn_00487f50() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_second_call_on_the_first_calls_result() {
        let mut e = Engine::new();
        e.register(GET_STATIC_OBJECT, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(CALL_WITH_MINUS_ONE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0048_7f50, &args![]).u32(), 0x1235);
    }
}
