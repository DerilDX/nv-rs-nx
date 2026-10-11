//! `fallout shared/tesactorbasedata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047c850 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns false in AL without reading the receiver. This folded body is
/// shared by several vtables and predicates; no Xbox PDB name is asserted.
pub fn fn_0047c850(_e: &mut Engine, _this: Ptr) -> bool {
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047c850, fn_0047c850(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_returns_false_without_reading_receiver_or_writing_memory() {
        // Six native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // FCW 027f / MXCSR 1f80: AL=0, RET 0, no faults or guard writes.
        let mut e = Engine::new();
        let guard = e.mem.alloc(16);
        e.mem.write(guard, &[0xa5; 16]);
        for receiver in [0, 1, 0x1000_0000, 0xdead_beef, u32::MAX, guard] {
            assert!(!e.call(0x0047c850, &[receiver]).bool());
            assert_eq!(e.mem.bytes(guard, 16), [0xa5; 16]);
        }
    }
}
