//! `fallout/magic/run_0080d6b0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `0080d930` (`this` only).
const DESTRUCTOR_BODY: u32 = 0x0080_d930;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0080d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (in three vtables, `0107951c`, `01079a9c`,
/// `01079bdc`; the class is not confirmed here): the destructor body
/// `0080d930`, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0080d6e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_d6e0, fn_0080d6e0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d6e0, &args![0x500u32, 2u32]).u32(), 0x500);
        assert_eq!(e.call(0x0080_d6e0, &args![0x500u32, 3u32]).u32(), 0x500);
        let log = e.call_log.take().unwrap();
        let deletes = log.iter().filter(|(a, _)| *a == OPERATOR_DELETE).count();
        let bodies = log.iter().filter(|(a, _)| *a == DESTRUCTOR_BODY).count();
        assert_eq!((bodies, deletes), (2, 1));
    }
}
