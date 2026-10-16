//! `fallout/ai/run_0096ad00` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0096ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the capacity of an array of pointers (the engine map gives it no name). The array object
/// (`ECX`) holds the pointer to the elements at `+4`, the capacity at `+8`, the count at `+0xa`
/// and the number of non-null elements at `+0xc`, all three counts being words. A capacity equal
/// to the current one does nothing. A smaller one first drops the elements from the new capacity
/// up to the count (clearing them and counting the non-null ones out) and lowers the count. A
/// new block is then taken from `0096afc0` (none for capacity 0), the elements below the count
/// are copied, the rest cleared, and the old block is given to `004ede70`.
pub fn fn_0096ad30(e: &mut Engine, this: Ptr, new_capacity: u32) {
    let base = this.addr();
    if new_capacity == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    if new_capacity < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = new_capacity as u16;
        while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
            let elements = e.mem.u32(base + 4);
            if e.mem.u32(elements + u32::from(index) * 4) != 0 {
                e.mem.set_u32(elements + u32::from(index) * 4, 0);
                let count = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, count.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.mem.set_u16(base + 0xa, new_capacity as u16);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, new_capacity as u16);
    if new_capacity > 0 {
        let capacity = u32::from(e.mem.u16(base + 8));
        let block = e.call(0x0096_afc0, &args![capacity]).u32();
        e.mem.set_u32(base + 4, block);
        let mut index = 0u16;
        while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
            let value = e.mem.u32(old + u32::from(index) * 4);
            let elements = e.mem.u32(base + 4);
            e.mem.set_u32(elements + u32::from(index) * 4, value);
            index = index.wrapping_add(1);
        }
        index = e.mem.u16(base + 0xa);
        while u32::from(index) < u32::from(e.mem.u16(base + 8)) {
            let elements = e.mem.u32(base + 4);
            e.mem.set_u32(elements + u32::from(index) * 4, 0);
            index = index.wrapping_add(1);
        }
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    e.call(0x004e_de70, &args![old]);
}

// Translated from 0096ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the pointer `*value` at `index` of the array `0096ad30` manages (the engine map gives
/// it no name). An index at or past the count raises the count to `index + 1` (as a word) and
/// counts a non-null pointer in; below the count the number of non-null elements follows the
/// element being replaced.
pub fn fn_0096ae90(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let base = this.addr();
    let pointer = e.mem.u32(value.addr());
    if index >= u32::from(e.mem.u16(base + 0xa)) {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if pointer != 0 {
            let count = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, count.wrapping_add(1));
        }
    } else {
        let elements = e.mem.u32(base + 4);
        let current = e.mem.u32(elements + index.wrapping_mul(4));
        if pointer != 0 {
            if current == 0 {
                let count = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, count.wrapping_add(1));
            }
        } else if current != 0 {
            let count = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, count.wrapping_sub(1));
        }
    }
    let elements = e.mem.u32(base + 4);
    e.mem.set_u32(elements + index.wrapping_mul(4), pointer);
}

// Translated from 0096afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the pointer array `0096ad30` manages (the engine map gives it no name). Stores
/// the vtable `0108c0c8` at `+0`, the low word of `capacity` at `+8` and the low word of
/// `second_word` at `+0xe`, clears the count (`+0xa`) and the non-null count (`+0xc`), and
/// takes the element block (`+4`) from `0096afc0` when the capacity word is above zero
/// (otherwise `+4` is 0). Parameters: two words, `RET 8`, `this` in `ECX`; both are pushed as
/// whole words and only their low 16 bits are read. Returns `this` (`EAX`).
pub fn fn_0096afe0(e: &mut Engine, this: Ptr, capacity: u32, second_word: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0108_c0c8);
    e.mem.set_u16(base + 8, capacity as u16);
    e.mem.set_u16(base + 0xe, second_word as u16);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let words = u32::from(e.mem.u16(base + 8));
    if words > 0 {
        let block = e.call(0x0096_afc0, &args![words]).u32();
        e.mem.set_u32(base + 4, block);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    this
}

// Translated from 0096af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A subclass constructor of the array `0096afe0` builds (the engine map gives it no name): runs
/// `0096afe0(this, capacity, second_word)` of this unit, then overwrites the vtable at `+0`
/// with `0108c0d0`. Returns `this`. Parameters: two words, `RET 8`, `this` in `ECX`.
pub fn fn_0096af30(e: &mut Engine, this: Ptr, capacity: u32, second_word: u32) -> Ptr {
    fn_0096afe0(e, this, capacity, second_word);
    e.mem.set_u32(this.addr(), 0x0108_c0d0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0096_ad30, fn_0096ad30(Ptr, u32)),
        entry!(0x0096_ae90, fn_0096ae90(Ptr, u32, Ptr)),
        entry!(0x0096_af30, fn_0096af30(Ptr, u32, u32) -> Ptr),
        entry!(0x0096_afe0, fn_0096afe0(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use super::*;

    /// An array object with `capacity` slots holding `elements`, `count` of them in use.
    fn array(e: &mut Engine, capacity: u16, elements: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let block = e.mem.alloc(u32::from(capacity.max(1)) * 4);
        for (index, value) in elements.iter().enumerate() {
            e.mem.set_u32(block + index as u32 * 4, *value);
        }
        e.mem.set_u32(this + 4, block);
        e.mem.set_u16(this + 8, capacity);
        e.mem.set_u16(this + 0xa, elements.len() as u16);
        let used = elements.iter().filter(|value| **value != 0).count();
        e.mem.set_u16(this + 0xc, used as u16);
        (this, block)
    }

    fn doubles(e: &mut Engine) {
        e.register_double(0x0096_afc0, |e, _| {
            let block = e.mem.alloc(0x40);
            e.mem.write(block, &[0xcd; 0x40]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(0x004e_de70, |_, _| Ret::default());
    }

    #[test]
    fn resize_to_the_same_capacity_does_nothing() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 4, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 4]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 4), block);
    }

    #[test]
    fn growing_copies_the_elements_and_clears_the_rest() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 2, &[7, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0096_afc0, vec![4]));
        assert_eq!(log[2], (0x004e_de70, vec![block]));
        let new = e.mem.u32(this + 4);
        assert_ne!(new, block);
        assert_eq!(e.mem.u16(this + 8), 4);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        let words: Vec<u32> = (0..4).map(|index| e.mem.u32(new + index * 4)).collect();
        assert_eq!(words, [7, 0, 0, 0]);
    }

    #[test]
    fn shrinking_drops_the_elements_past_the_new_capacity() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 4, &[1, 0, 3, 4]);
        e.call(0x0096_ad30, &[this, 2]);
        assert_eq!(e.mem.u16(this + 8), 2);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 12), 0);
        let new = e.mem.u32(this + 4);
        assert_eq!((e.mem.u32(new), e.mem.u32(new + 4)), (1, 0));
    }

    #[test]
    fn resizing_to_zero_frees_the_block() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 2, &[5, 6]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x004e_de70, vec![block])]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
    }

    #[test]
    fn setting_an_element_keeps_the_non_null_count() {
        let mut e = Engine::new();
        let (this, block) = array(&mut e, 4, &[0, 9]);
        let value = e.mem.alloc(4);
        // Replacing a null with a pointer.
        e.mem.set_u32(value, 0x1234);
        e.call(0x0096_ae90, &[this, 0, value]);
        assert_eq!((e.mem.u32(block), e.mem.u16(this + 0xc)), (0x1234, 2));
        // Clearing a pointer.
        e.mem.set_u32(value, 0);
        e.call(0x0096_ae90, &[this, 1, value]);
        assert_eq!((e.mem.u32(block + 4), e.mem.u16(this + 0xc)), (0, 1));
        // Clearing a null changes nothing.
        e.call(0x0096_ae90, &[this, 1, value]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        // Replacing a pointer with another.
        e.mem.set_u32(value, 0x77);
        e.call(0x0096_ae90, &[this, 0, value]);
        assert_eq!((e.mem.u32(block), e.mem.u16(this + 0xc)), (0x77, 1));
    }

    #[test]
    fn setting_past_the_count_extends_it() {
        let mut e = Engine::new();
        let (this, block) = array(&mut e, 4, &[3]);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x55);
        e.call(0x0096_ae90, &[this, 2, value]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        assert_eq!(e.mem.u32(block + 8), 0x55);
        e.mem.set_u32(value, 0);
        e.call(0x0096_ae90, &[this, 3, value]);
        assert_eq!(e.mem.u16(this + 0xa), 4);
        assert_eq!(e.mem.u16(this + 0xc), 2);
    }
}

#[cfg(test)]
mod process_list_tests {
    use super::*;

    fn block_double(e: &mut Engine) {
        e.register(0x0096_afc0, |_, _| Ret {
            eax: 0x0009_0000,
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_takes_the_element_block_for_a_positive_capacity() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xee; 0x10]);
        e.call_log = Some(vec![]);
        // The high halves of the pushed words are not read.
        let result = e.call(0x0096_afe0, &[this, 0xabcd_0007, 0x1234_0001]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0096_afc0).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, vec![7]);
        assert_eq!(e.mem.u32(this), 0x0108_c0c8);
        assert_eq!(e.mem.u32(this + 4), 0x0009_0000);
        assert_eq!(e.mem.u16(this + 8), 7);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }

    #[test]
    fn constructor_without_capacity_allocates_nothing() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xee; 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0096_afe0, &[this, 0, 1]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != 0x0096_afc0));
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }

    #[test]
    fn subclass_constructor_replaces_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        let result = e.call(0x0096_af30, &[this, 0, 1]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x0108_c0d0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }
}
