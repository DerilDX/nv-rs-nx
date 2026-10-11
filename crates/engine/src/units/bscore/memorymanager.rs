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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa4030, allocate_with_context(Ptr, u32, u32, u32, u32) -> Ptr)]
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
