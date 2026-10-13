//! `fallout shared/magic/run_00403720` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// The string object's initializer (`00403970`): zeroes the word at `+0` and
/// then calls `004039c0` and `00403a00` with 0.
const INITIALIZE: u32 = 0x0040_3970;
/// The string object's text setter (`004037f0`, `RET 8`): two arguments, the
/// destructor passes (0, 0).
const SET_TEXT: u32 = 0x0040_37f0;

// Translated from 004037b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the string object (see `00406f60`, `Calendar::GetDateString`
/// in `00867970`): initializes it (`00403970`) and returns `this`.
pub fn fn_004037b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALIZE, &args![this]);
    this
}

// Translated from 004037d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the string object: clears its text (`004037f0` with (0, 0)).
/// The decompiler's library name for this address is a false match.
pub fn fn_004037d0(e: &mut Engine, this: Ptr) {
    e.call(SET_TEXT, &args![this, 0u32, 0u32]);
}

// Translated from 00403970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the string object (`BSStringT<char>`, Xbox PDB): no text
/// (`pString` 0) and a length and capacity of 0.
pub fn fn_00403970(e: &mut Engine, this: Ptr<BSStringT>) {
    e.set(this, BSStringT::pString, 0);
    fn_004039c0(e, this, 0);
    fn_00403a00(e, this, 0);
}

// Translated from 004039c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the string's length (`sLen`), limited to 16 bits: a
/// value of 0x10000 or more is stored as 0xFFFF.
pub fn fn_004039c0(e: &mut Engine, this: Ptr<BSStringT>, value: u32) {
    e.set(this, BSStringT::sLen, value.min(0xffff) as u16);
}

// Translated from 00403a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the string's capacity (`sMaxLen`), limited to 16 bits: a
/// value of 0x10000 or more is stored as 0xFFFF.
pub fn fn_00403a00(e: &mut Engine, this: Ptr<BSStringT>, value: u32) {
    e.set(this, BSStringT::sMaxLen, value.min(0xffff) as u16);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004037b0, fn_004037b0(Ptr) -> Ptr),
        entry!(0x004037d0, fn_004037d0(Ptr)),
        entry!(0x00403970, fn_00403970(Ptr<BSStringT>)),
        entry!(0x004039c0, fn_004039c0(Ptr<BSStringT>, u32)),
        entry!(0x00403a00, fn_00403a00(Ptr<BSStringT>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_initializes_and_returns_this() {
        let mut e = Engine::new();
        e.register(INITIALIZE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0040_37b0, &args![0x1000u32]).u32();
        assert_eq!(back, 0x1000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (INITIALIZE, vec![0x1000]));
    }

    #[test]
    fn initializer_clears_the_text_and_both_counts() {
        let mut e = Engine::new();
        let string: Ptr<BSStringT> = e.new_object();
        e.set(string, BSStringT::pString, 0x1234);
        e.set(string, BSStringT::sLen, 7);
        e.set(string, BSStringT::sMaxLen, 9);
        e.call(0x0040_3970, &args![string]);
        assert_eq!(e.get(string, BSStringT::pString), 0);
        assert_eq!(e.get(string, BSStringT::sLen), 0);
        assert_eq!(e.get(string, BSStringT::sMaxLen), 0);
    }

    #[test]
    fn length_and_capacity_setters_saturate_at_16_bits() {
        let mut e = Engine::new();
        let string: Ptr<BSStringT> = e.new_object();
        for (value, stored) in [
            (0u32, 0u16),
            (0x1234, 0x1234),
            (0xffff, 0xffff),
            (0x1_0000, 0xffff),
            (u32::MAX, 0xffff),
        ] {
            e.call(0x0040_39c0, &args![string, value]);
            e.call(0x0040_3a00, &args![string, value]);
            assert_eq!(e.get(string, BSStringT::sLen), stored);
            assert_eq!(e.get(string, BSStringT::sMaxLen), stored);
        }
    }

    #[test]
    fn destructor_sets_empty_text() {
        let mut e = Engine::new();
        e.register(SET_TEXT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_37d0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SET_TEXT, vec![0x1000, 0, 0]));
    }
}
