//! `(unplaced)/run_007b52a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007b52d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`; the `this` register is saved and never read.
/// Answers the dword at the start of the node (its next-node pointer), or null for a null node.
pub fn fn_007b52d0(e: &mut Engine, _unused_this: Ptr, node: Ptr) -> u32 {
    if node.is_null() {
        0
    } else {
        e.mem.u32(node.addr())
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007b_52d0, fn_007b52d0(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007b52d0_answers_the_first_dword_of_the_node_or_null() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 0x7000);
        e.mem.set_u32(node + 4, 0x8000);
        assert_eq!(e.call(0x007b_52d0, &args![0x1234u32, node]).u32(), 0x7000);
        // A null node is answered with null, without reading memory (address 0 is unmapped).
        assert_eq!(e.call(0x007b_52d0, &args![node, 0u32]).u32(), 0);
    }
}
