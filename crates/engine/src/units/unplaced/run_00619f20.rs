//! `(unplaced)/run_00619f20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A bounds-checked read of a word array held at +4 of an object (the
//! topic code uses it on a topic's per-quest entries for their infos).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00717e50` (`ECX` = object): `this + 4` (the array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = array): the word at +0xc (its count).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00877a30` (`ECX` = array, one word: the index; `RET 4`): the address of
/// that element (`m_pBase + index * 4`).
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;

// Translated from 00619f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (object, index): 0 without an object; otherwise the element
/// `index` of the array at +4 (`00717e50`, count `0084e3a0`, element
/// `00877a30`) when `index` is below the count (unsigned), else 0. The
/// exe's unsigned `index < 0` test never holds and is left out. The
/// engine map gives it no name.
pub fn fn_00619f20(e: &mut Engine, object: Ptr, index: u32) -> u32 {
    if object.is_null() {
        return 0;
    }
    let array = e.call(PLUS_FOUR, &args![object]).u32();
    let count = e.call(WORD_AT_0XC, &args![array]).u32();
    if index >= count {
        return 0;
    }
    let array = e.call(PLUS_FOUR, &args![object]).u32();
    let element = e.call(ELEMENT_ADDRESS, &args![array, index]).u32();
    e.mem.u32(element)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00619f20, fn_00619f20(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn reads_an_element_below_the_count() {
        let mut e = Engine::new();
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ELEMENT_ADDRESS, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
        e.call_log = Some(vec![]);
        let object = e.mem.alloc(0x14);
        let base = e.mem.alloc(8);
        e.mem.set_u32(object + 8, base);
        e.mem.set_u32(object + 0x10, 2);
        e.mem.set_u32(base, 0x111);
        e.mem.set_u32(base + 4, 0x222);
        assert_eq!(e.call(0x0061_9f20, &args![object, 1u32]).u32(), 0x222);
        assert_eq!(e.call(0x0061_9f20, &args![object, 0u32]).u32(), 0x111);
        assert_eq!(e.call(0x0061_9f20, &args![object, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0061_9f20, &args![object, u32::MAX]).u32(), 0);
        assert_eq!(e.call(0x0061_9f20, &args![0u32, 0u32]).u32(), 0);
        let elements: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ELEMENT_ADDRESS)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(elements, vec![vec![object + 4, 1], vec![object + 4, 0]]);
    }
}
