//! `BSCore/memorymanager.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa4030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocation wrapper: forwards only the size but consumes all four stack
/// words (RET 0x10). The final three arguments are unread by this body.
pub fn allocate_with_context(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    e.call(0x00aa_3e40, &args![this, size]).ptr()
}

/// The CRT `free` (`_free`, `00ecd291`, cdecl, one word).
const CRT_FREE: u32 = 0x00ec_d291;

// Translated from 00aa42c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the sibling of `00aa4030`, the allocation wrapper above): thiscall with two
/// stack words (`RET 8`), the memory manager as `this`. It passes the first word, the block, to
/// the CRT `free` and returns nothing; `this` and the second word are never read.
pub fn fn_00aa42c0(e: &mut Engine, _this: Ptr, block: Ptr, _unused_2: u32) {
    e.call(CRT_FREE, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa4030, allocate_with_context(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00aa42c0, fn_00aa42c0(Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrapper_consumes_all_words_but_forwards_only_receiver_and_size() {
        let mut e = Engine::new();
        e.register_double(0x00aa3e40, |_, args| {
            assert_eq!(args, &[0x1234, 0x89]);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00aa4030, &[0x1234, 0x89, 3, 4, 5]).u32(), 0);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    #[test]
    fn free_wrapper_passes_only_the_block_to_the_crt_free() {
        let mut e = Engine::new();
        e.register(CRT_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x00aa42c0, &args![0x1111u32, 0x2222u32, 0x3333u32]);
        assert_eq!(result.u32(), 0);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == CRT_FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![0x2222]);
    }

    #[test]
    fn free_wrapper_frees_a_real_heap_block() {
        let mut e = Engine::new();
        let block = e.mem.alloc(32);
        assert!(e.mem.block_size(block).is_some());
        e.call(0x00aa42c0, &args![0u32, block, 0u32]);
        assert!(e.mem.block_size(block).is_none());
    }
}
