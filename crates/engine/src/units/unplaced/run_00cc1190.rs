//! `(unplaced)/run_00cc1190` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc1710 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00cc1520`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00cc1710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_1520;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00cc1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0126_8590.
pub fn return_constant_00cc1280(_e: &mut Engine) -> u32 {
    0x0126_8590
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc1280, return_constant_00cc1280() -> u32),
        entry!(0x00cc1710, fn_00cc1710(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cc1710_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_1520, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cc1710, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc1710, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00cc_1520), vec![vec![this]; 2]);
    }

    #[test]
    fn return_constant_00cc1280_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_1280, &args![]).u32(), 0x0126_8590);
    }
}
