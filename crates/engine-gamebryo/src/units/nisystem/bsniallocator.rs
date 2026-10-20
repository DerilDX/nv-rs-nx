//! `NiSystem/bsniallocator.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa2240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Allocate` (Xbox PDB). Kind 7 reserves the byte alignment
/// in addition to the requested size, adjusts the returned address using
/// byte arithmetic and stores that adjustment immediately before it. Other
/// kinds forward the size and final three context words to `00aa4030`.
/// Both paths reload the size after allocation and zero that many bytes.
#[allow(clippy::too_many_arguments)] // Native receiver plus seven stack words.
pub fn allocate(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_4: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> Ptr {
    let buffer = if kind == 7 {
        let alignment = e.mem.u8(alignment.addr());
        let bytes = e.mem.u32(size.addr()).wrapping_add(alignment as u32);
        let raw = e.call(0x00aa_3e40, &args![0x011f_6238u32, bytes]).u32();
        let adjustment = alignment.wrapping_sub(alignment.wrapping_sub(1) & raw as u8);
        let aligned = raw.wrapping_add(adjustment as u32);
        e.mem.set_u8(aligned.wrapping_sub(1), adjustment);
        aligned
    } else {
        let bytes = e.mem.u32(size.addr());
        e.call(
            0x00aa_4030,
            &args![0x011f_6238u32, bytes, context_1, context_2, context_3],
        )
        .u32()
    };
    let bytes = e.mem.u32(size.addr());
    e.call(0x00ec_61c0, &args![buffer, 0u32, bytes]);
    Ptr::new(buffer)
}

// Translated from 00aa22b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Deallocate` (engine map; thiscall, `RET 0xc`, `this` and the third word
/// unread): frees `memory` through the memory manager (`00aa4060` on `011f6238`); for kind 10
/// the block starts the byte stored just before `memory` earlier (and a null `memory` frees
/// nothing).
pub fn deallocate(e: &mut Engine, _this: Ptr, memory: Ptr, kind: u32, _unused_3: u32) {
    if kind == 10 {
        if memory.addr() == 0 {
            return;
        }
        let adjustment = u32::from(e.mem.u8(memory.addr().wrapping_sub(1)));
        let block = memory.addr().wrapping_sub(adjustment);
        e.call(0x00aa_4060, &args![0x011f_6238u32, block]);
        return;
    }
    e.call(0x00aa_4060, &args![0x011f_6238u32, memory]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa2240,
            allocate(Ptr, Ptr<u32>, Ptr<u8>, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_22b0, deallocate(Ptr, Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_allocation_forwards_context_and_reloads_size_before_zeroing() {
        let mut e = Engine::new();
        let size = e.mem.alloc(4);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(size, 4);
        e.mem.write(buffer, &[0xcc; 16]);
        e.register_double(0x00aa4030, move |e, args| {
            assert_eq!(args, &[0x011f6238, 4, 12, 13, 14]);
            e.mem.set_u32(size, 8);
            buffer.into_ret()
        });
        // Alignment is unread outside kind 7; null is intentional.
        let result = e.call(0x00aa2240, &[0, size, 0, 5, 11, 12, 13, 14]).u32();
        assert_eq!(result, buffer);
        assert_eq!(e.mem.bytes(buffer, 8), [0; 8]);
        assert_eq!(e.mem.bytes(buffer + 8, 8), [0xcc; 8]);
    }

    #[test]
    fn aligned_allocation_uses_low_alignment_byte_and_preserves_guards() {
        for raw_offset in [0u32, 3, 8] {
            let mut e = Engine::new();
            let size = e.mem.alloc(4);
            let alignment = e.mem.alloc(4);
            let backing = e.mem.alloc(64);
            e.mem.set_u32(size, 4);
            e.mem.set_u32(alignment, 0xaaaaaa10);
            e.mem.write(backing, &[0xcc; 64]);
            let raw = backing + raw_offset;
            e.register_double(0x00aa3e40, move |e, args| {
                assert_eq!(args, &[0x011f6238, 20]);
                // PC caches alignment before this call, but reloads size.
                e.mem.set_u8(alignment, 8);
                e.mem.set_u32(size, 5);
                raw.into_ret()
            });
            let result = e
                .call(0x00aa2240, &[0, size, alignment, 7, 11, 12, 13, 14])
                .u32();
            assert_eq!(result & 15, 0);
            assert!(result > raw);
            let adjustment = result - raw;
            assert_eq!(e.mem.u8(result - 1), adjustment as u8);
            assert_eq!(e.mem.u8(result - 2), 0xcc);
            assert_eq!(e.mem.bytes(result, 5), [0; 5]);
            assert_eq!(e.mem.u8(result + 5), 0xcc);
        }
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn frees(e: &mut Engine, memory: u32, kind: u32) -> Vec<Vec<u32>> {
        e.register(0x00aa_4060, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_22b0, &args![0x1234u32, memory, kind, 0x40u32]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00aa_4060)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn deallocate_frees_through_the_memory_manager() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x20);
        assert_eq!(frees(&mut e, block, 3), vec![vec![0x011f_6238, block]]);
        assert_eq!(frees(&mut e, 0, 3), vec![vec![0x011f_6238, 0]]);
        // Kind 10: the adjustment byte before the memory gives the block's start.
        e.mem.set_u8(block + 5, 6);
        assert_eq!(frees(&mut e, block + 6, 10), vec![vec![0x011f_6238, block]]);
        assert!(frees(&mut e, 0, 10).is_empty());
    }
}
