//! `Havok SDK/hkfreelist.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkFreeList`, so its layout is described by
//! offsets only, each named where a function uses it (0x34 bytes in the free-list
//! allocator's pool):
//! `+0` free-list head, `+4` rounded element size, `+8` (cleared by the
//! constructor, never read by these functions), `+0xc` and `+0x10` block lists
//! (a block's first word links to the next, the word at `+0xc` is its size),
//! `+0x14` block size, `+0x18` alignment, `+0x1c` the first element offset,
//! `+0x20`..`+0x2c` counters (`+0x28` and `+0x2c` are the byte counts
//! `_freeBlocks` takes the block sizes from), `+0x30` the allocator the blocks
//! come from.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dd7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map gives the address no unit; placed here
/// because it clears the same fields that the constructor `00dd74b0` does):
/// `thiscall`, no stack arguments. Clears every word of the 0x34-byte record
/// except the one at `+8`. Returns `this`.
pub fn fn_00dd7110(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [
        0x00, 0x04, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00dd71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::_freeBlocks` (Xbox PDB), `thiscall`, `RET 4`: walks the chain of
/// blocks from `first` (each block's first word links to the next). For each it
/// takes the block's size (the word at `+0xc`) off the two counters at `+0x2c`
/// and `+0x28`, and frees the block through slot +8 of the allocator at `+0x30`.
/// Returns the number of blocks.
pub fn hk_free_list_free_blocks(e: &mut Engine, this: Ptr, first: u32) -> u32 {
    let base = this.addr();
    let mut count = 0u32;
    let mut block = first;
    while block != 0 {
        let size = e.mem.u32(block + 0xc);
        let total = e.mem.u32(base + 0x2c).wrapping_sub(size);
        e.mem.set_u32(base + 0x2c, total);
        let size = e.mem.u32(block + 0xc);
        let used = e.mem.u32(base + 0x28).wrapping_sub(size);
        e.mem.set_u32(base + 0x28, used);
        let allocator = e.mem.u32(base + 0x30);
        let next = e.mem.u32(block);
        count = count.wrapping_add(1);
        e.vcall(allocator, 8, &args![block]);
        block = next;
    }
    count
}

// Translated from 00dd7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): frees the block
/// chains at `+0xc` and then at `+0x10` (`_freeBlocks`, `00dd71c0`), clearing
/// each head, and clears `+0`, `+0x14`, `+0x28`, `+0x2c`, `+0x20`, `+0x24` and
/// `+8`.
pub fn fn_00dd7200(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let blocks = e.mem.u32(base + 0xc);
    hk_free_list_free_blocks(e, this, blocks);
    let blocks = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    hk_free_list_free_blocks(e, this, blocks);
    for offset in [0x10, 0x00, 0x14, 0x28, 0x2c, 0x20, 0x24, 0x08] {
        e.mem.set_u32(base + offset, 0);
    }
}

// Translated from 00dd73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words, the caller removes them): the
/// second word, when it is more than 0x10, plus the first; otherwise the first
/// plus 0x10 (both wrapping). The constructor calls it with `0xff0` and the
/// alignment.
pub fn fn_00dd73d0(_e: &mut Engine, base: u32, extra: u32) -> u32 {
    if extra > 0x10 {
        base.wrapping_add(extra)
    } else {
        base.wrapping_add(0x10)
    }
}

// Translated from 00dd74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: element size, alignment,
/// block size, allocator): the `hkFreeList` constructor. Clears `+0`, `+8`, `+0xc`,
/// `+0x10`, `+0x20`..`+0x2c`; stores the block size at `+0x14`, the alignment at
/// `+0x18`, the allocator at `+0x30`; the element size at `+4` is the alignment
/// when smaller than it (unsigned), otherwise rounded up to a multiple of it
/// (`(size + alignment - 1) & !(alignment - 1)`); `+0x1c` is `00dd73d0(0xff0,
/// alignment)`. The exe answers that last value in `EAX`, which is returned here.
pub fn fn_00dd74b0(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
    allocator: u32,
) -> u32 {
    let base = this.addr();
    for offset in [0x00, 0x0c, 0x10, 0x20, 0x24, 0x28, 0x2c, 0x08] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x14, block_size);
    e.mem.set_u32(base + 0x18, alignment);
    e.mem.set_u32(base + 0x30, allocator);
    let rounded = if element_size < alignment {
        alignment
    } else {
        element_size.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1)
    };
    e.mem.set_u32(base + 4, rounded);
    let first_offset = fn_00dd73d0(e, 0xff0, alignment);
    e.mem.set_u32(base + 0x1c, first_offset);
    first_offset
}

// Translated from 00dd7650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`, the four words of
/// `00dd74b0`): calls the constructor `00dd74b0` with them and returns `this`.
pub fn fn_00dd7650(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
    allocator: u32,
) -> Ptr {
    fn_00dd74b0(e, this, element_size, alignment, block_size, allocator);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd7110, fn_00dd7110(Ptr) -> Ptr),
        entry!(0x00dd71c0, hk_free_list_free_blocks(Ptr, u32) -> u32),
        entry!(0x00dd7200, fn_00dd7200(Ptr)),
        entry!(0x00dd73d0, fn_00dd73d0(u32, u32) -> u32),
        entry!(0x00dd74b0, fn_00dd74b0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd7650, fn_00dd7650(Ptr, u32, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATOR_FREE: u32 = 0x4500_0000;

    /// An allocator whose free slot (+8) records every block it is given.
    fn allocator(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0, ALLOCATOR_FREE]);
        e.register(ALLOCATOR_FREE, |_, _| Ret::default());
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        allocator
    }

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A block of 0x20 bytes: link, size at +0xc.
    fn block(e: &mut Engine, next: u32, size: u32) -> u32 {
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, next);
        e.mem.set_u32(block + 0xc, size);
        block
    }

    #[test]
    fn the_default_constructor_clears_all_but_the_word_at_8() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x34);
        e.mem.write(record, &[0xcd; 0x34]);
        assert_eq!(e.call(0x00dd_7110, &args![record]).u32(), record);
        for offset in (0..0x34).step_by(4) {
            let want = if offset == 8 { 0xcdcd_cdcd } else { 0 };
            assert_eq!(e.mem.u32(record + offset), want, "offset {offset:#x}");
        }
    }

    #[test]
    fn free_blocks_walks_the_chain_and_adjusts_the_counters() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x200);
        e.mem.set_u32(list + 0x30, allocator);
        let second = block(&mut e, 0, 0x30);
        let first = block(&mut e, second, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_71c0, &args![list, first]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, first], vec![allocator, second]]
        );
        assert_eq!(e.mem.u32(list + 0x28), 0xc0);
        assert_eq!(e.mem.u32(list + 0x2c), 0x1c0);
    }

    #[test]
    fn free_blocks_of_an_empty_chain_does_nothing() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_71c0, &args![list, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn clearing_frees_both_chains_and_zeroes_the_fields() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.write(list, &[0xcd; 0x34]);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x100);
        e.mem.set_u32(list + 0x30, allocator);
        let a = block(&mut e, 0, 0x10);
        let b = block(&mut e, 0, 0x20);
        e.mem.set_u32(list + 0xc, a);
        e.mem.set_u32(list + 0x10, b);
        e.call_log = Some(vec![]);
        e.call(0x00dd_7200, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, a], vec![allocator, b]]
        );
        for offset in [0x00, 0x08, 0x0c, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(list + offset), 0, "offset {offset:#x}");
        }
        // Not named by the function: the element size, alignment, first
        // element offset and the allocator.
        assert_eq!(e.mem.u32(list + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x18), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x1c), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x30), allocator);
    }

    #[test]
    fn the_offset_helper_adds_the_larger_of_0x10_and_the_second_word() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 8u32]).u32(), 0x1000);
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 0x10u32]).u32(), 0x1000);
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 0x20u32]).u32(), 0x1010);
    }

    #[test]
    fn the_constructor_rounds_the_element_size_to_the_alignment() {
        // (element size, alignment, rounded size)
        for (size, alignment, rounded) in [
            (0u32, 4u32, 4u32),
            (3, 4, 4),
            (4, 4, 4),
            (5, 4, 8),
            (0x220, 0x80, 0x280),
        ] {
            let mut e = Engine::new();
            let list = e.mem.alloc(0x34);
            e.mem.write(list, &[0xcd; 0x34]);
            let result = e.call(
                0x00dd_74b0,
                &args![list, size, alignment, 0x5000u32, 0x6000u32],
            );
            assert_eq!(e.mem.u32(list + 4), rounded, "size {size:#x}");
            assert_eq!(e.mem.u32(list + 0x14), 0x5000);
            assert_eq!(e.mem.u32(list + 0x18), alignment);
            assert_eq!(e.mem.u32(list + 0x30), 0x6000);
            let offset = 0xff0 + alignment.max(0x10);
            assert_eq!(e.mem.u32(list + 0x1c), offset);
            assert_eq!(result.u32(), offset);
            for field in [0x00, 0x08, 0x0c, 0x10, 0x20, 0x24, 0x28, 0x2c] {
                assert_eq!(e.mem.u32(list + field), 0, "field {field:#x}");
            }
        }
    }

    #[test]
    fn the_wrapper_passes_its_words_on_and_returns_this() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        assert_eq!(
            e.call(
                0x00dd_7650,
                &args![list, 0x30u32, 0x10u32, 0x1000u32, 0x77u32]
            )
            .u32(),
            list
        );
        assert_eq!(e.mem.u32(list + 4), 0x30);
        assert_eq!(e.mem.u32(list + 0x14), 0x1000);
        assert_eq!(e.mem.u32(list + 0x18), 0x10);
        assert_eq!(e.mem.u32(list + 0x30), 0x77);
    }
}
