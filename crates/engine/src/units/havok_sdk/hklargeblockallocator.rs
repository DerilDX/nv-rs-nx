//! `Havok SDK/hklargeblockallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkLargeBlockAllocator`, so the offsets are
//! described where the functions use them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkLargeBlockAllocator`'s virtual table (`010d7c34`; the class name is
/// confirmed by the RTTI record before it).
const VTABLE: u32 = 0x010d_7c34;

// Translated from 00dd7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): the state set-up of
/// the large block allocator. Clears the words at `+8` and `+0xc`, `+0x14`,
/// `+0x18` to `+0x94`, `+0x9c`, `+0xbc` and `+0xc0`; stores `3` at `+0xa0`; points
/// `+0x10` at `this + 0x9c` and `+0xa8` at `this + 0xa4`, and the word at `+0xa4`
/// at itself (a list head linked to itself).
pub fn fn_00dd7f10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0xc0, 0xbc, 0x08, 0x0c] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in (0x18..=0x94).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0xa0, 3);
    e.mem.set_u32(base + 0x10, base + 0x9c);
    e.mem.set_u32(base + 0xa8, base + 0xa4);
    e.mem.set_u32(base + 0xa4, base + 0xa4);
}

// Translated from 00dd8d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::hkLargeBlockAllocator` (RTTI-confirmed class; the map
/// gives the address no name), `thiscall`, `RET 4`: stores the virtual table, clears
/// `+4`, stores `parent` (the memory allocator it takes its memory from) at
/// `+0x98` and sets the rest up (`00dd7f10`). Returns `this`.
pub fn hk_large_block_allocator_hk_large_block_allocator(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0x98, parent);
    fn_00dd7f10(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd7f10, fn_00dd7f10(Ptr)),
        entry!(
            0x00dd8d40,
            hk_large_block_allocator_hk_large_block_allocator(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0xd0;

    #[test]
    fn the_set_up_clears_the_state_and_links_the_list_head() {
        let mut e = Engine::new();
        let object = e.mem.alloc(SIZE);
        e.mem.write(object, &[0xcd; SIZE as usize]);
        e.call(0x00dd_7f10, &args![object]);
        for offset in [0x08, 0x0c, 0x14, 0x18, 0x50, 0x94, 0x9c, 0xbc, 0xc0] {
            assert_eq!(e.mem.u32(object + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(object + 0xa0), 3);
        assert_eq!(e.mem.u32(object + 0x10), object + 0x9c);
        assert_eq!(e.mem.u32(object + 0xa4), object + 0xa4);
        assert_eq!(e.mem.u32(object + 0xa8), object + 0xa4);
        // Untouched: the vtable word, +4, +0x98, +0xac, +0xb0.
        for offset in [0x00, 0x04, 0x98, 0xac, 0xb0, 0xc4] {
            assert_eq!(
                e.mem.u32(object + offset),
                0xcdcd_cdcd,
                "offset {offset:#x}"
            );
        }
    }

    #[test]
    fn the_constructor_stores_the_table_and_the_parent() {
        let mut e = Engine::new();
        let object = e.mem.alloc(SIZE);
        e.mem.write(object, &[0xcd; SIZE as usize]);
        assert_eq!(
            e.call(0x00dd_8d40, &args![object, 0x1234_0000u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 0x98), 0x1234_0000);
        assert_eq!(e.mem.u32(object + 0xa4), object + 0xa4);
    }
}
