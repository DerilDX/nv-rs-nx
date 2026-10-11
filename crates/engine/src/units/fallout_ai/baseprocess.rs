//! `fallout/ai/baseprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d0360 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns true in AL without reading the receiver. This folded body is
/// also used by a reference vtable; its mapped unit does not identify all
/// callers, and no Xbox PDB name is asserted.
pub fn fn_008d0360(_e: &mut Engine, _this: Ptr) -> bool {
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d0360, fn_008d0360(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_returns_true_without_reading_receiver_or_writing_memory() {
        // Six native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // FCW 027f / MXCSR 1f80: AL=1, RET 0, no faults or guard writes.
        let mut e = Engine::new();
        let guard = e.mem.alloc(16);
        e.mem.write(guard, &[0xa5; 16]);
        for receiver in [0, 1, 0x1000_0000, 0xdead_beef, u32::MAX, guard] {
            assert!(e.call(0x008d0360, &[receiver]).bool());
            assert_eq!(e.mem.bytes(guard, 16), [0xa5; 16]);
        }
    }

    #[test]
    fn load_buffer_type_queries_reach_both_shared_predicates() {
        // BGSLoadFormBuffer::GetReference / GetActor (Xbox PDB) use
        // reference vtable slots +0xf0 / +0x100 on PC. Exercise the real
        // registered translations, with generated receivers and vtables.
        let mut e = Engine::new();
        let buffer = e.mem.alloc(0x30);
        let buffer_vtable = e.mem.alloc(16);
        let reference = e.mem.alloc(4);
        let reference_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(buffer, buffer_vtable);
        e.mem.set_u32(buffer + 0x24, reference);
        e.mem.set_u32(buffer_vtable + 4, 0x008645b0);
        e.mem.set_u32(buffer_vtable + 8, 0x00864610);
        e.mem.set_u32(buffer_vtable + 12, 0x00864660);
        e.mem.set_u32(reference, reference_vtable);
        e.mem.set_u32(reference_vtable + 0xf0, 0x008d0360);
        e.mem.set_u32(reference_vtable + 0x100, 0x0047c850);
        assert_eq!(e.call(0x00864610, &[buffer]).u32(), reference);
        assert_eq!(e.call(0x00864660, &[buffer]).u32(), 0);
        e.mem.set_u32(buffer + 0x24, 0);
        assert_eq!(e.call(0x00864610, &[buffer]).u32(), 0);
    }
}
