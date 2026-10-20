//! `fallout shared/run_0048ce80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The length of a string object (`004048e0`: its cached length, or the
/// computed one when the cache holds `0xffff`).
const STRING_LENGTH: u32 = 0x0040_48e0;

// Translated from 0048cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of the string at `this + 4` (`004048e0`, whose `EAX` it
/// passes on).
pub fn fn_0048cee0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(STRING_LENGTH, &args![this.addr() + 4]).u32()
}

/// A `BSStringT`'s text setter (`004037f0`, `RET 8`): (text, length).
const SET_TEXT: u32 = 0x0040_37f0;

// Translated from 0048ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the string at `this + 4`: its text set to (0, 0) with
/// `004037f0`. `ECX`, no stack words. The engine map gives it no name.
pub fn fn_0048ce80(e: &mut Engine, this: Ptr) {
    e.call(SET_TEXT, &args![this.addr() + 4, 0u32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048cee0, fn_0048cee0(Ptr) -> u32),
        entry!(0x0048_ce80, fn_0048ce80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_length_of_the_string_at_4() {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_cee0, &[0x4000]).u32(), 0x4004 ^ 0x5555);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (STRING_LENGTH, vec![0x4004]));
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    #[test]
    fn clearing_sets_the_string_at_4_to_nothing() {
        let mut e = Engine::new();
        e.register(SET_TEXT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_ce80, &[0x4000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SET_TEXT, vec![0x4004, 0, 0])]);
    }
}
