//! `BSShader/run_00bd8480` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The count word whose `count * 4` bytes `00bd9120` allocates.
const SLOT_COUNT: u32 = 0x011b_f558;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00bd9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a one-word object (unnamed in the engine map; the
/// `BSShaderAccumulator` and batch renderer constructors keep it as their
/// instance renderer): allocates `count * 4` bytes (`0xFFFFFFFF` when the
/// product overflows), the count being the word at `011bf558`, and stores
/// the block in the object's only word. Returns `this`.
pub fn fn_00bd9120(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.mem.u32(SLOT_COUNT);
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(this.addr(), block);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bd_9120, fn_00bd9120(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_allocates_four_bytes_per_slot_and_saturates() {
        for (count, size) in [(5u32, 20u32), (0x4000_0001, u32::MAX)] {
            let mut e = Engine::new();
            e.map(0x011b_f000, 0x1000);
            e.mem.set_u32(SLOT_COUNT, count);
            e.register(MEMORY_ALLOCATE, |_, a| {
                0x0070_0000u32.wrapping_add(a[1]).into_ret()
            });
            let this = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00bd_9120, &[this]).u32(), this);
            assert_eq!(e.mem.u32(this), 0x0070_0000u32.wrapping_add(size));
            let log = e.call_log.unwrap();
            let allocations: Vec<_> = log.iter().filter(|(a, _)| *a == MEMORY_ALLOCATE).collect();
            assert_eq!(allocations[0].1, vec![MEMORY_MANAGER, size]);
        }
    }
}
