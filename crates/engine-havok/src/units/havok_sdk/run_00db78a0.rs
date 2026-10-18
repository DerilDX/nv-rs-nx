//! `Havok SDK/run_00db78a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00db78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(this, array, values)` (`RET 8`): walks the zero-terminated list of object pointers
/// `array` next to the list `values`; for every object whose word at `+0xc` is not zero it calls
/// `this`'s virtual slot `+0xc` with that word and the matching value. The walk ends at the
/// first zero value as well as at the first null object pointer. The exe's name for it is
/// unknown.
pub fn fn_00db78a0(e: &mut Engine, this: u32, array: u32, values: u32) {
    if e.mem.u32(array) == 0 {
        return;
    }
    let mut at = array;
    let offset = values.wrapping_sub(array);
    loop {
        let value = e.mem.u32(at.wrapping_add(offset));
        if value == 0 {
            return;
        }
        let first = e.mem.u32(e.mem.u32(at) + 0xc);
        if first != 0 {
            e.vcall(this, 0xc, &args![first, value]);
        }
        at += 4;
        if e.mem.u32(at) == 0 {
            return;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00db_78a0, fn_00db78a0(u32, u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    const SLOT: u32 = 0x0900_000c;

    fn target(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SLOT);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        e.register(SLOT, |_, _| Ret::default());
        this
    }

    /// Objects with the given words at `+0xc`, as a zero-terminated pointer list.
    fn objects(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(4 * (words.len() as u32 + 1));
        for (i, word) in words.iter().enumerate() {
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object + 0xc, *word);
            e.mem.set_u32(array + 4 * i as u32, object);
        }
        array
    }

    fn values(e: &mut Engine, items: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * (items.len() as u32 + 1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *item);
        }
        at
    }

    fn slot_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == SLOT)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn every_object_with_a_word_is_told_its_value() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[0xa1, 0, 0xa3]);
        let list = values(&mut e, &[1, 2, 3]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert_eq!(
            slot_calls(&e),
            vec![vec![this, 0xa1, 1], vec![this, 0xa3, 3]]
        );
    }

    #[test]
    fn a_zero_value_ends_the_walk() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[0xa1, 0xa2, 0xa3]);
        let list = values(&mut e, &[1, 0, 3]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert_eq!(slot_calls(&e), vec![vec![this, 0xa1, 1]]);
    }

    #[test]
    fn an_empty_list_calls_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[]);
        let list = values(&mut e, &[1]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert!(slot_calls(&e).is_empty());
    }
}
