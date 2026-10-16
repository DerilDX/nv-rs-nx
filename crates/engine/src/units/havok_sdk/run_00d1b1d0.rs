//! `Havok SDK/run_00d1b1d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The first word (the sort key) of element `index` (8 bytes each).
fn key_00d1b1d0(e: &Engine, array: u32, index: i32) -> u32 {
    e.mem
        .u32(array.wrapping_add((index as u32).wrapping_mul(8)))
}

// Translated from 00d1b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. cdecl, four words: quicksort of the elements `low..=high`
/// of the array of 8-byte pairs at `array`, by their first word as an
/// unsigned number. The fourth word is only passed on to the recursion. The
/// pivot is the key of the middle element; the partition scans, swaps and
/// recursion (left part recursed, right part looped) are those of the code.
pub fn fn_00d1b1d0(e: &mut Engine, array: Ptr, mut low: i32, high: i32, unused_4: u32) {
    let array = array.addr();
    loop {
        let pivot = key_00d1b1d0(e, array, low.wrapping_add(high) >> 1);
        let (mut i, mut j) = (low, high);
        loop {
            while key_00d1b1d0(e, array, i) < pivot {
                i = i.wrapping_add(1);
            }
            while pivot < key_00d1b1d0(e, array, j) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let at = |index: i32| array.wrapping_add((index as u32).wrapping_mul(8));
                let (from, to) = (at(i), at(j));
                let (a0, a1) = (e.mem.u32(to), e.mem.u32(to + 4));
                let (b0, b1) = (e.mem.u32(from), e.mem.u32(from + 4));
                e.mem.set_u32(to, b0);
                e.mem.set_u32(to + 4, b1);
                e.mem.set_u32(from, a0);
                e.mem.set_u32(from + 4, a1);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            fn_00d1b1d0(e, Ptr::new(array), low, j, unused_4);
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1b1d0, fn_00d1b1d0(Ptr, i32, i32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d1b1d0_sorts_the_pairs_by_unsigned_first_word() {
        let mut e = Engine::new();
        let keys = [5u32, 0xffff_fff0, 9, 1, 7, 3, 8, 2, 3];
        let array = e.mem.alloc(8 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(array + 8 * i as u32, *key);
            e.mem.set_u32(array + 8 * i as u32 + 4, 500 + i as u32);
        }
        e.call(
            0x00d1_b1d0,
            &args![Ptr::<()>::new(array), 0i32, keys.len() as i32 - 1, 0u32],
        );
        let sorted: Vec<u32> = (0..keys.len() as u32)
            .map(|i| e.mem.u32(array + 8 * i))
            .collect();
        assert_eq!(sorted, vec![1, 2, 3, 3, 5, 7, 8, 9, 0xffff_fff0]);
        // The second word travelled with its key (key 0xfffffff0 was index 1).
        assert_eq!(e.mem.u32(array + 8 * 8 + 4), 501);
    }
}
