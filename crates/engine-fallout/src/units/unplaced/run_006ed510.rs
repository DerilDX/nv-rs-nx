//! `(unplaced)/run_006ed510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006ed510` calls on the member (`006f25d0`).
const CALLEE_006F25D0: u32 = 0x006f_25d0;

// Translated from 006ed510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006f25d0` with `ECX` = `this + 0x2c` and passes on nothing (the callee returns no value).
pub fn fn_006ed510(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_006F25D0, &args![this.byte_add(0x2c)]);
}

/// `00877a30` (`thiscall`, `RET 4`; unit `fallout/misc/main.cpp`).
const SET_INDEX: u32 = 0x0087_7a30;

// Translated from 006f2270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): calls `00877a30` on `this` with the
/// word at +8 minus 1 and answers its value (the exe ends on that call and leaves its `EAX`,
/// the address the callers read through: `00877a30` answers an array element's address).
pub fn fn_006f2270(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.u32(this.addr() + 8).wrapping_sub(1);
    e.call(SET_INDEX, &args![this, value]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_d510, fn_006ed510(Ptr)),
        entry!(0x006f_2270, fn_006f2270(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006ed510_passes_this_plus_2c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006F25D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_d510, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006F25D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x2c]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_006f2270_passes_the_word_minus_one() {
        let mut e = Engine::new();
        e.register(SET_INDEX, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x006f_2270, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SET_INDEX, vec![this, u32::MAX])]);
    }

    #[test]
    fn fn_006f2270_answers_the_element_address() {
        let mut e = Engine::new();
        e.register(SET_INDEX, |_, a| (a[0] + 0x100 + 4 * a[1]).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 3);
        assert_eq!(e.call(0x006f_2270, &args![this]).u32(), this + 0x108);
    }
}
