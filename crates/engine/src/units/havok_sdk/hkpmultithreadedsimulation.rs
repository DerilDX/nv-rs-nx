//! `Havok SDK/hkpmultithreadedsimulation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d04320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_2da0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d04320(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_2da0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Address of element `index` (8 bytes each) of the array at `array`.
fn element_00d04200(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(8))
}

/// Swaps the two 8-byte elements.
fn swap_00d04200(e: &mut Engine, first: u32, second: u32) {
    let (a0, a1) = (e.mem.u32(first), e.mem.u32(first + 4));
    let (b0, b1) = (e.mem.u32(second), e.mem.u32(second + 4));
    e.mem.set_u32(first, b0);
    e.mem.set_u32(first + 4, b1);
    e.mem.set_u32(second, a0);
    e.mem.set_u32(second + 4, a1);
}

// Translated from 00d04200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpTypedBroadPhaseHandlePair, bool
/// (__cdecl*)(const hkpTypedBroadPhaseHandlePair&, const
/// hkpTypedBroadPhaseHandlePair&)>` (map name). cdecl, four words: sorts
/// elements `low..=high` of the array of 8-byte pairs at `array` with the
/// less-than function `less` (called by address as `less(&a, &b)`, result
/// in AL). The pivot is a copy of the middle element on the stack; the
/// partition scans, swaps and recursion are those of the code (recursion on
/// the left part, a loop on the right part).
pub fn hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(
    e: &mut Engine,
    array: Ptr,
    mut low: i32,
    high: i32,
    less: u32,
) {
    let array = array.addr();
    loop {
        let (mut i, mut j) = (low, high);
        let middle = element_00d04200(array, low.wrapping_add(high) >> 1);
        let (pivot_first, pivot_second) = (e.mem.u32(middle), e.mem.u32(middle + 4));
        e.with_stack(8, |e, pivot| {
            let pivot = pivot.addr();
            e.mem.set_u32(pivot, pivot_first);
            e.mem.set_u32(pivot + 4, pivot_second);
            loop {
                if e.call(less, &args![element_00d04200(array, i), pivot])
                    .bool()
                {
                    loop {
                        i = i.wrapping_add(1);
                        if !e
                            .call(less, &args![element_00d04200(array, i), pivot])
                            .bool()
                        {
                            break;
                        }
                    }
                }
                if e.call(less, &args![pivot, element_00d04200(array, j)])
                    .bool()
                {
                    loop {
                        j = j.wrapping_sub(1);
                        if !e
                            .call(less, &args![pivot, element_00d04200(array, j)])
                            .bool()
                        {
                            break;
                        }
                    }
                }
                if j < i {
                    break;
                }
                if j != i {
                    swap_00d04200(e, element_00d04200(array, i), element_00d04200(array, j));
                }
                j = j.wrapping_sub(1);
                i = i.wrapping_add(1);
                if i > j {
                    break;
                }
            }
        });
        if low < j {
            hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(
                e,
                Ptr::new(array),
                low,
                j,
                less,
            );
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
    vec![
        entry!(
            0x00d04200,
            hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(Ptr, i32, i32, u32)
        ),
        entry!(0x00d0_4320, deleting_destructor_00d04320(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d04320_frees_only_when_bit_zero_is_set() {
        check(0x00d0_4320, 0x00d0_2da0);
    }

    const LESS_00D04200: u32 = 0x0090_5000;

    /// A less-than double on the first word of two pairs, counting calls.
    fn sorted_00d04200(keys: &[u32]) -> (Vec<(u32, u32)>, usize) {
        let mut e = Engine::new();
        let calls = Rc::new(RefCell::new(0usize));
        let counter = Rc::clone(&calls);
        e.register_double(LESS_00D04200, move |e, a| {
            *counter.borrow_mut() += 1;
            Ret {
                eax: (e.mem.u32(a[0]) < e.mem.u32(a[1])) as u32,
                ..Ret::default()
            }
        });
        let array = e.mem.alloc(8 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(array + 8 * i as u32, *key);
            e.mem.set_u32(array + 8 * i as u32 + 4, 1000 + i as u32);
        }
        e.call(
            0x00d0_4200,
            &args![
                Ptr::<()>::new(array),
                0i32,
                keys.len() as i32 - 1,
                LESS_00D04200
            ],
        );
        let sorted = (0..keys.len() as u32)
            .map(|i| (e.mem.u32(array + 8 * i), e.mem.u32(array + 8 * i + 4)))
            .collect();
        let count = *calls.borrow();
        (sorted, count)
    }

    #[test]
    fn hk_algorithm_quick_sort_recursive_sorts_the_pairs_by_the_comparator() {
        let (sorted, calls) = sorted_00d04200(&[5, 3, 9, 1, 7, 3, 8, 2]);
        let keys: Vec<u32> = sorted.iter().map(|pair| pair.0).collect();
        assert_eq!(keys, vec![1, 2, 3, 3, 5, 7, 8, 9]);
        assert!(calls > 0);
        // Each pair kept its second word with its key.
        let second_of_nine = sorted.iter().find(|pair| pair.0 == 9).unwrap().1;
        assert_eq!(second_of_nine, 1002);
        let (single, calls) = sorted_00d04200(&[4]);
        assert_eq!(single, vec![(4, 1000)]);
        assert!(calls > 0);
    }
}
