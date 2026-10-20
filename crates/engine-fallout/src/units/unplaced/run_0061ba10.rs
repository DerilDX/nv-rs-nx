//! `(unplaced)/run_0061ba10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

// Translated from 0061bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetAt` of an `NiTLargeArray` of plain words
/// (layout [`NiTLargeArray`]; `thiscall`, two stack words, `RET 8`): stores
/// the word `*element` at `index`, without a capacity check. Below the used
/// size, the non-empty count goes up when a zero slot gets a non-zero word
/// and down when a non-zero slot gets zero; at or above it, the used size
/// becomes `index + 1` and the count goes up when the word is non-zero.
pub fn fn_0061bd20(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, element: Ptr) {
    let slot_of = |e: &Engine| {
        e.get(this, NiTLargeArray::m_pBase)
            .wrapping_add(index.wrapping_mul(4))
    };
    if index < e.get(this, NiTLargeArray::m_uiSize) {
        if e.mem.u32(element.addr()) == 0 {
            if e.mem.u32(slot_of(e)) != 0 {
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
        } else if e.mem.u32(slot_of(e)) == 0 {
            let present = e.get(this, NiTLargeArray::m_uiESize);
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    } else {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if e.mem.u32(element.addr()) != 0 {
            let present = e.get(this, NiTLargeArray::m_uiESize);
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    }
    let value = e.mem.u32(element.addr());
    let slot = slot_of(e);
    e.mem.set_u32(slot, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0061bd20,
        fn_0061bd20(Ptr<NiTLargeArray>, u32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An array of capacity 4 holding `items`, and a word holding `value`.
    fn fixture(items: &[u32], value: u32) -> (Engine, Ptr<NiTLargeArray>, u32, u32) {
        let mut e = Engine::new();
        let array = e.new_object::<NiTLargeArray>();
        let base = e.mem.alloc(16);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTLargeArray::m_pBase, base);
        e.set(array, NiTLargeArray::m_uiMaxSize, 4);
        e.set(array, NiTLargeArray::m_uiSize, items.len() as u32);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTLargeArray::m_uiESize, present as u32);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, value);
        (e, array, base, element)
    }

    fn counts(e: &Engine, array: Ptr<NiTLargeArray>) -> (u32, u32) {
        (
            e.get(array, NiTLargeArray::m_uiSize),
            e.get(array, NiTLargeArray::m_uiESize),
        )
    }

    #[test]
    fn filling_an_empty_slot_counts_it() {
        let (mut e, array, base, element) = fixture(&[7, 0], 9);
        e.call(0x0061_bd20, &args![array, 1u32, element]);
        assert_eq!(e.mem.u32(base + 4), 9);
        assert_eq!(counts(&e, array), (2, 2));
    }

    #[test]
    fn clearing_a_used_slot_uncounts_it() {
        let (mut e, array, base, element) = fixture(&[7, 8], 0);
        e.call(0x0061_bd20, &args![array, 0u32, element]);
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(counts(&e, array), (2, 1));
    }

    #[test]
    fn replacing_a_used_slot_keeps_the_count() {
        let (mut e, array, base, element) = fixture(&[7, 8], 5);
        e.call(0x0061_bd20, &args![array, 1u32, element]);
        assert_eq!(e.mem.u32(base + 4), 5);
        assert_eq!(counts(&e, array), (2, 2));
        // Zero into a zero slot changes nothing either.
        let (mut e, array, _, element) = fixture(&[0, 8], 0);
        e.call(0x0061_bd20, &args![array, 0u32, element]);
        assert_eq!(counts(&e, array), (2, 1));
    }

    #[test]
    fn past_the_used_size_grows_it() {
        let (mut e, array, base, element) = fixture(&[7], 6);
        e.call(0x0061_bd20, &args![array, 2u32, element]);
        assert_eq!(e.mem.u32(base + 8), 6);
        assert_eq!(counts(&e, array), (3, 2));
        let (mut e, array, _, element) = fixture(&[7], 0);
        e.call(0x0061_bd20, &args![array, 3u32, element]);
        assert_eq!(counts(&e, array), (4, 1));
    }
}
