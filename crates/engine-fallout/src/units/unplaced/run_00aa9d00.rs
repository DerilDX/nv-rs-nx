//! `(unplaced)/run_00aa9d00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most of them keep the free lists of `AbstractHeap` (`BSCore/abstractheap.cpp`; the layout
//! is `crate::units::bscore::abstractheap::AbstractHeap`, read here at its offsets). A heap
//! block has an 8-byte header: the previous block at +0 and the size in the low 28 bits of
//! the word at +4 (the high bits are flags, 0x40000000 marking a free block); a free block
//! links to its neighbours in its free list at +8 (previous) and +0xc (next). A free list is
//! 8 bytes: its first block at +0 and its last block at +4.

#[allow(unused_imports)]
use crate::prelude::*;

/// `AbstractHeap::iDefaultAlignment` (Xbox PDB) +0x8.
const HEAP_DEFAULT_ALIGNMENT: u32 = 0x8;
/// `AbstractHeap::iNumFreeBlocks` (Xbox PDB) +0x40.
const HEAP_NUM_FREE_BLOCKS: u32 = 0x40;
/// `AbstractHeap::pSmallFreeList` (Xbox PDB) +0x44: 1024 free lists, one per size step.
const HEAP_SMALL_FREE_LISTS: u32 = 0x44;
/// `AbstractHeap::LargeFreeList` (Xbox PDB) +0x2048: the free list of the larger blocks.
const HEAP_LARGE_FREE_LIST: u32 = 0x2048;
/// `AbstractHeap::pSubBuckets` (Xbox PDB) +0x2058: 8-byte entries, a count at +0.
const HEAP_SUB_BUCKETS: u32 = 0x2058;
/// The number of small free lists; a step index from it on uses the large list.
const SMALL_FREE_LIST_COUNT: i32 = 0x400;
/// Sizes above this have no sub-bucket.
const SUB_BUCKET_MAX_SIZE: i32 = 0x1000;

/// A heap block's header: the size-and-flags word (+4), the free-list links (+8, +0xc).
const BLOCK_SIZE_FLAGS: u32 = 0x4;
const BLOCK_FREE_PREVIOUS: u32 = 0x8;
const BLOCK_FREE_NEXT: u32 = 0xc;
const BLOCK_SIZE_MASK: u32 = 0x0fff_ffff;
const BLOCK_FREE: u32 = 0x4000_0000;
/// A free list's last block (+4); its first block is its first word.
const LIST_LAST: u32 = 0x4;

/// The size in a block's header.
fn block_size(e: &Engine, block: u32) -> u32 {
    e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_SIZE_MASK
}

/// The free list of blocks of `size` bytes: index `size / iDefaultAlignment - 1` (unsigned
/// division; compared signed, so a size below the alignment gives index -1, the 8 bytes
/// before the first small list), the large list from index 0x400 on.
fn free_list_for(e: &Engine, heap: u32, size: u32) -> u32 {
    let alignment = e.mem.u32(heap + HEAP_DEFAULT_ALIGNMENT);
    let index = (size / alignment).wrapping_sub(1) as i32;
    if index >= SMALL_FREE_LIST_COUNT {
        heap + HEAP_LARGE_FREE_LIST
    } else {
        heap.wrapping_add(HEAP_SMALL_FREE_LISTS)
            .wrapping_add((index as u32).wrapping_mul(8))
    }
}

/// The sub-bucket of a size: -1 above 0x1000, else `(size / iDefaultAlignment - 1) / 64`
/// (signed divisions, truncating as `IDIV` and the `CDQ`/`AND 0x3f`/`SAR 6` idiom do).
fn sub_bucket_for(e: &Engine, heap: u32, size: i32) -> i32 {
    if size > SUB_BUCKET_MAX_SIZE {
        -1
    } else {
        let alignment = e.mem.u32(heap + HEAP_DEFAULT_ALIGNMENT) as i32;
        (size / alignment).wrapping_sub(1) / 64
    }
}

// Translated from 00aa9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): puts a block on its free list.
/// Clears its links, marks it free, appends it after the list's last block (or makes it the
/// only block of an empty list), counts it in `iNumFreeBlocks` and in its sub-bucket
/// ([`fn_00aa9ef0`]).
pub fn fn_00aa9d30(e: &mut Engine, this: Ptr, block: Ptr) {
    let (heap, block) = (this.addr(), block.addr());
    e.mem.set_u32(block + BLOCK_FREE_NEXT, 0);
    e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, 0);
    // The exe tests a constant 1 here (an inlined flag setter given `true`), so only the
    // setting branch runs; the clearing one is kept as the exe has it.
    let mark_free = true;
    let flags = e.mem.u32(block + BLOCK_SIZE_FLAGS);
    if mark_free {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags | BLOCK_FREE);
    } else {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags & !BLOCK_FREE);
    }
    let size = block_size(e, block);
    let list = free_list_for(e, heap, size);
    let last = e.mem.u32(list + LIST_LAST);
    if last != 0 {
        e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, last);
        let next = e.mem.u32(last + BLOCK_FREE_NEXT);
        e.mem.set_u32(block + BLOCK_FREE_NEXT, next);
        if next != 0 {
            e.mem.set_u32(next + BLOCK_FREE_PREVIOUS, block);
        }
        e.mem.set_u32(last + BLOCK_FREE_NEXT, block);
        e.mem.set_u32(list + LIST_LAST, block);
    } else {
        e.mem.set_u32(list, block);
        e.mem.set_u32(list + LIST_LAST, block);
    }
    let count = e.mem.u32(heap + HEAP_NUM_FREE_BLOCKS).wrapping_add(1);
    e.mem.set_u32(heap + HEAP_NUM_FREE_BLOCKS, count);
    let size = block_size(e, block);
    fn_00aa9ef0(e, this, size as i32);
}

// Translated from 00aa9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): adds one to the count of the
/// sub-bucket of `size` (none above 0x1000).
pub fn fn_00aa9ef0(e: &mut Engine, this: Ptr, size: i32) {
    let bucket = sub_bucket_for(e, this.addr(), size);
    if bucket != -1 {
        let at = this
            .addr()
            .wrapping_add(HEAP_SUB_BUCKETS)
            .wrapping_add((bucket as u32).wrapping_mul(8));
        let count = e.mem.u32(at).wrapping_add(1);
        e.mem.set_u32(at, count);
    }
}

// Translated from 00aa9f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): takes one from the count of the
/// sub-bucket of `size` (none above 0x1000).
pub fn fn_00aa9f50(e: &mut Engine, this: Ptr, size: i32) {
    let bucket = sub_bucket_for(e, this.addr(), size);
    if bucket != -1 {
        let at = this
            .addr()
            .wrapping_add(HEAP_SUB_BUCKETS)
            .wrapping_add((bucket as u32).wrapping_mul(8));
        let count = e.mem.u32(at).wrapping_sub(1);
        e.mem.set_u32(at, count);
    }
}

// Translated from 00aaa5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on a free heap block, no words): unlinks the block from its
/// neighbours (the previous block's next becomes this block's next and the other way round)
/// and clears both of its links.
pub fn fn_00aaa5e0(e: &mut Engine, this: Ptr) {
    let block = this.addr();
    if e.mem.u32(block + BLOCK_FREE_PREVIOUS) != 0 {
        let next = e.mem.u32(block + BLOCK_FREE_NEXT);
        let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
        e.mem.set_u32(previous + BLOCK_FREE_NEXT, next);
    }
    if e.mem.u32(block + BLOCK_FREE_NEXT) != 0 {
        let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
        let next = e.mem.u32(block + BLOCK_FREE_NEXT);
        e.mem.set_u32(next + BLOCK_FREE_PREVIOUS, previous);
    }
    e.mem.set_u32(block + BLOCK_FREE_NEXT, 0);
    e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, 0);
}

// Translated from 00aa9e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, two words: a free list and a block): takes the
/// block off the list. Unlinks it ([`fn_00aaa5e0`]), clears its free mark, moves the list's
/// first or last block on to its old neighbour when it was that block, takes it out of its
/// sub-bucket ([`fn_00aa9f50`]) and from `iNumFreeBlocks`.
pub fn fn_00aa9e50(e: &mut Engine, this: Ptr, list: Ptr, block: Ptr) {
    let (list, block) = (list.addr(), block.addr());
    let next = e.mem.u32(block + BLOCK_FREE_NEXT);
    let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
    fn_00aaa5e0(e, Ptr::new(block));
    // The exe tests a constant 0 here (an inlined flag setter given `false`), so only the
    // clearing branch runs; the setting one is kept as the exe has it.
    let mark_free = false;
    let flags = e.mem.u32(block + BLOCK_SIZE_FLAGS);
    if mark_free {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags | BLOCK_FREE);
    } else {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags & !BLOCK_FREE);
    }
    if block == e.mem.u32(list) {
        e.mem.set_u32(list, next);
    }
    if block == e.mem.u32(list + LIST_LAST) {
        e.mem.set_u32(list + LIST_LAST, previous);
    }
    let size = block_size(e, block);
    fn_00aa9f50(e, this, size as i32);
    let count = e
        .mem
        .u32(this.addr() + HEAP_NUM_FREE_BLOCKS)
        .wrapping_sub(1);
    e.mem.set_u32(this.addr() + HEAP_NUM_FREE_BLOCKS, count);
}

// Translated from 00aaa660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): takes a free block off the free
/// list of its size ([`fn_00aa9e50`]).
pub fn fn_00aaa660(e: &mut Engine, this: Ptr, block: Ptr) {
    let size = block_size(e, block.addr());
    let list = free_list_for(e, this.addr(), size);
    fn_00aa9e50(e, this, Ptr::new(list), block);
}

// Translated from 00aaa650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 and ignores its three stack words (`__thiscall`, `RET 0xC`;
/// `XOR EAX,EAX`). The map gives it no name; it is, for example, slot +0xC
/// of the `BSSaveDataSystemUtility` vtable (`010a4288`), which `0084fc70`
/// calls with `"-SAVE-"`.
pub fn fn_00aaa650(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aaa650, fn_00aaa650(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa9d30, fn_00aa9d30(Ptr, Ptr)),
        entry!(0x00aa9ef0, fn_00aa9ef0(Ptr, i32)),
        entry!(0x00aa9f50, fn_00aa9f50(Ptr, i32)),
        entry!(0x00aaa5e0, fn_00aaa5e0(Ptr)),
        entry!(0x00aa9e50, fn_00aa9e50(Ptr, Ptr, Ptr)),
        entry!(0x00aaa660, fn_00aaa660(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_for_any_arguments() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let back = e.call(0x00aa_a650, &args![this, 0x0107_fabcu32, 1u32, 2u32]);
        assert_eq!(back.u32(), 0);
    }

    /// A heap with alignment 16.
    fn heap(e: &mut Engine) -> u32 {
        let heap = e.mem.alloc(0x2100);
        e.mem.set_u32(heap + HEAP_DEFAULT_ALIGNMENT, 16);
        heap
    }

    /// A block of `size` bytes with the given flags.
    fn block(e: &mut Engine, size: u32, flags: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, size | flags);
        block
    }

    fn sub_bucket(e: &Engine, heap: u32, index: u32) -> u32 {
        e.mem.u32(heap + HEAP_SUB_BUCKETS + index * 8)
    }

    #[test]
    fn fn_00aa9d30_appends_to_the_list_of_its_size() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // Size 0x40: index 3, sub-bucket 0.
        let list = h + HEAP_SMALL_FREE_LISTS + 3 * 8;
        let a = block(&mut e, 0x40, 0x2000_0000);
        e.mem.set_u32(a + BLOCK_FREE_NEXT, 0x77);
        e.call(0x00aa9d30, &args![h, a]);
        assert_eq!(e.mem.u32(a + BLOCK_SIZE_FLAGS), 0x6000_0040);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), 0);
        let b = block(&mut e, 0x40, 0);
        // A block after `a` already: the new one goes between them.
        let c = block(&mut e, 0x40, 0);
        e.mem.set_u32(a + BLOCK_FREE_NEXT, c);
        e.call(0x00aa9d30, &args![h, b]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, b));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), b);
        assert_eq!(e.mem.u32(b + BLOCK_FREE_PREVIOUS), a);
        assert_eq!(e.mem.u32(b + BLOCK_FREE_NEXT), c);
        assert_eq!(e.mem.u32(c + BLOCK_FREE_PREVIOUS), b);
        assert_eq!(e.mem.u32(h + HEAP_NUM_FREE_BLOCKS), 2);
        assert_eq!(sub_bucket(&e, h, 0), 2);
    }

    #[test]
    fn fn_00aa9d30_puts_large_blocks_on_the_large_list() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // Size 0x4010: index 0x400, the large list, no sub-bucket.
        let a = block(&mut e, 0x4010, 0);
        e.call(0x00aa9d30, &args![h, a]);
        let list = h + HEAP_LARGE_FREE_LIST;
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        assert!((0..16).all(|i| sub_bucket(&e, h, i) == 0));
    }

    #[test]
    fn sub_bucket_counts_follow_the_size() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // 0x1000: index 0xff, bucket 3; 0x1010 has none; 8 (below the alignment): bucket 0.
        e.call(0x00aa9ef0, &args![h, 0x1000i32]);
        e.call(0x00aa9ef0, &args![h, 0x1010i32]);
        e.call(0x00aa9ef0, &args![h, 8i32]);
        assert_eq!(sub_bucket(&e, h, 3), 1);
        assert_eq!(sub_bucket(&e, h, 0), 1);
        e.call(0x00aa9f50, &args![h, 0x1000i32]);
        e.call(0x00aa9f50, &args![h, 0x1010i32]);
        e.call(0x00aa9f50, &args![h, 0x10i32]);
        assert_eq!(sub_bucket(&e, h, 3), 0);
        assert_eq!(sub_bucket(&e, h, 0), 0);
        e.call(0x00aa9f50, &args![h, 0x100i32]);
        assert_eq!(sub_bucket(&e, h, 0), u32::MAX);
    }

    #[test]
    fn fn_00aaa5e0_unlinks_a_block() {
        let mut e = Engine::new();
        let (a, b, c) = (
            block(&mut e, 0x10, 0),
            block(&mut e, 0x10, 0),
            block(&mut e, 0x10, 0),
        );
        e.mem.set_u32(a + BLOCK_FREE_NEXT, b);
        e.mem.set_u32(b + BLOCK_FREE_PREVIOUS, a);
        e.mem.set_u32(b + BLOCK_FREE_NEXT, c);
        e.mem.set_u32(c + BLOCK_FREE_PREVIOUS, b);
        e.call(0x00aaa5e0, &args![b]);
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), c);
        assert_eq!(e.mem.u32(c + BLOCK_FREE_PREVIOUS), a);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (0, 0));
        // Alone: nothing else is touched.
        e.call(0x00aaa5e0, &args![b]);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (0, 0));
    }

    #[test]
    fn fn_00aa9e50_and_fn_00aaa660_take_blocks_off_their_list() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        let list = h + HEAP_SMALL_FREE_LISTS + 8;
        let a = block(&mut e, 0x20, 0);
        let b = block(&mut e, 0x20, 0);
        let c = block(&mut e, 0x20, 0);
        for x in [a, b, c] {
            e.call(0x00aa9d30, &args![h, x]);
        }
        assert_eq!(sub_bucket(&e, h, 0), 3);
        // The middle one: the list's ends stay.
        e.call(0x00aa9e50, &args![h, list, b]);
        assert_eq!(e.mem.u32(b + BLOCK_SIZE_FLAGS), 0x20);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, c));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), c);
        // The last one, through its size.
        e.call(0x00aaa660, &args![h, c]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        // The only one left.
        e.call(0x00aaa660, &args![h, a]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(h + HEAP_NUM_FREE_BLOCKS), 0);
        assert_eq!(sub_bucket(&e, h, 0), 0);
    }
}
