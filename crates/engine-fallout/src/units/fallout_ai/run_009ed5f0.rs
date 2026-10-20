//! `fallout/ai/run_009ed5f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `DialoguePackage` destructor body (`009edf40`).
const DIALOGUE_PACKAGE_DESTRUCT: u32 = 0x009e_df40;
/// The game's `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// The word `009edd30` compares with (+0xb0 of `this`).
const COMPARED_WORD: u32 = 0xb0;

// Translated from 009edd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` equals the word at +0xb0 of `this` (thiscall, `RET 4`;
/// unnamed in the engine map).
pub fn fn_009edd30(e: &mut Engine, this: Ptr, value: u32) -> bool {
    value == e.mem.u32(this.addr() + COMPARED_WORD)
}

// Translated from 009edd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialoguePackage` scalar deleting destructor (by the destructor it
/// runs, `009edf40`, which stores that class's vtable): destroys the
/// object and frees it (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009edd50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DIALOGUE_PACKAGE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_dd30, fn_009edd30(Ptr, u32) -> bool),
        entry!(0x009e_dd50, fn_009edd50(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009edd30_compares_with_the_word_at_0xb0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this + COMPARED_WORD, 0x1234_5678);
        assert_eq!(
            e.call(0x009e_dd30, &args![this, 0x1234_5678u32]).u32() & 0xff,
            1
        );
        assert_eq!(
            e.call(0x009e_dd30, &args![this, 0x1234_5679u32]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn fn_009edd50_destroys_then_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DIALOGUE_PACKAGE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009e_dd50, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let order: Vec<_> = log
                .iter()
                .filter(|(a, _)| *a == DIALOGUE_PACKAGE_DESTRUCT || *a == FREE)
                .cloned()
                .collect();
            let mut expected = vec![(DIALOGUE_PACKAGE_DESTRUCT, vec![this])];
            if frees {
                expected.push((FREE, vec![this]));
            }
            assert_eq!(order, expected, "flags {flags}");
        }
    }
}
