//! `Havok SDK/hkthreadmemory.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkThreadMemory`, so the offsets are described
//! where the code uses them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkThreadMemory`'s virtual table (`010cacd0`; the class name is confirmed by
/// the RTTI record before it).
const VTABLE: u32 = 0x010c_acd0;

/// The size class of a block size, from the ladder both table loops of `00d2c100`
/// use (every comparison signed): sizes up to 0x10 are class 1, then up to 0x20,
/// 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280, 0x400, 0x800,
/// 0x1000 and 0x2000 are classes 2 to 16. `None` is where the exe executes `INT3`
/// (a size over 0x2000); the loops never reach it (their sizes stop at 0x2000).
fn size_class(size: i32) -> Option<u32> {
    const LIMITS: [i32; 16] = [
        0x10, 0x20, 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280, 0x400, 0x800,
        0x1000, 0x2000,
    ];
    LIMITS
        .iter()
        .position(|limit| size <= *limit)
        .map(|index| index as u32 + 1)
}

// Translated from 00d2c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): fills the
/// size-class tables of an `hkThreadMemory`. For the 41 sizes `16 * n`
/// (`n` = 0 to 0x28) the class goes to the byte table at `+0xd8 + n`; then for
/// the sizes 0x400, 0x800, ... (below 0x2400) the class goes to the word
/// table at `+0x104`, one word each. Every size is also stored in the word at
/// `+0x94 + 4 * class`. The exe's `INT3` for a size over 0x2000 is a panic here;
/// no size of these loops reaches it.
pub fn fn_00d2c100(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for n in 0..=0x28u32 {
        let size = n << 4;
        let class = size_class(size as i32).unwrap_or_else(|| {
            panic!("FalloutNV.exe executes INT3 at 00d2c1fe: size {size:#x} has no size class")
        });
        e.mem.set_u8(base + 0xd8 + n, class as u8);
        e.mem.set_u32(base + 0x94 + class * 4, size);
    }
    let mut size = 0x400u32;
    let mut slot = base + 0x104;
    while (size as i32) < 0x2400 {
        let class = size_class(size as i32).unwrap_or_else(|| {
            panic!("FalloutNV.exe executes INT3 at 00d2c31e: size {size:#x} has no size class")
        });
        e.mem.set_u32(slot, class);
        e.mem.set_u32(base + 0x94 + class * 4, size);
        size += 0x400;
        slot += 4;
    }
}

// Translated from 00d2c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `first`, `second`): when
/// `first` is not zero it is stored at `+4` and `second` at `+8`; for zero only
/// `+4` is cleared (`+8` is left as it was).
pub fn fn_00d2c3a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    if first != 0 {
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, second);
    } else {
        e.mem.set_u32(this.addr() + 4, 0);
    }
}

// Translated from 00d2c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::hkThreadMemory` (Xbox PDB), `thiscall`, no stack arguments:
/// stores the virtual table, clears the 0x88 bytes from `+0xc` to `+0x93`, fills
/// the size-class tables (`00d2c100`) and calls `00d2c3a0` with `(0, 8)`, which
/// clears the word at `+4`. Returns `this`.
pub fn hk_thread_memory_hk_thread_memory(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for word in 0..0x22u32 {
        e.mem.set_u32(base + 0xc + word * 4, 0);
    }
    fn_00d2c100(e, this);
    fn_00d2c3a0(e, this, 0, 8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2c100, fn_00d2c100(Ptr)),
        entry!(0x00d2c3a0, fn_00d2c3a0(Ptr, u32, u32)),
        entry!(0x00d2c720, hk_thread_memory_hk_thread_memory(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_class_ladder_matches_the_exes_comparisons() {
        assert_eq!(size_class(0), Some(1));
        assert_eq!(size_class(0x10), Some(1));
        assert_eq!(size_class(0x11), Some(2));
        assert_eq!(size_class(0x50), Some(5));
        assert_eq!(size_class(0x280), Some(12));
        assert_eq!(size_class(0x2000), Some(16));
        assert_eq!(size_class(0x2001), None);
    }

    #[test]
    fn the_tables_are_filled_from_the_sizes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x138);
        e.mem.write(object, &[0xcd; 0x138]);
        e.call(0x00d2_c100, &args![object]);
        let classes = e.mem.bytes(object + 0xd8, 0x29);
        assert_eq!(classes[0], 1);
        assert_eq!(classes[1], 1);
        assert_eq!(classes[2], 2);
        assert_eq!(classes[5], 5);
        assert_eq!(classes[0x28], 12);
        assert_eq!(e.mem.u8(object + 0x101), 0xcd);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(object + 0x104 + i * 4)).collect();
        assert_eq!(words, vec![13, 14, 15, 15, 16, 16, 16, 16]);
        assert_eq!(e.mem.u32(object + 0x124), 0xcdcd_cdcd);
        // The size of each class: the last (largest) size stored for it.
        assert_eq!(e.mem.u32(object + 0x94 + 4), 0x10);
        assert_eq!(e.mem.u32(object + 0x94 + 12 * 4), 0x280);
        assert_eq!(e.mem.u32(object + 0x94 + 16 * 4), 0x2000);
        assert_eq!(e.mem.u32(object + 0x94 + 13 * 4), 0x400);
        assert_eq!(e.mem.u32(object + 0x94 + 15 * 4), 0x1000);
    }

    #[test]
    fn the_setter_stores_both_words_or_clears_the_first() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0xcd; 0x20]);
        e.call(0x00d2_c3a0, &args![object, 0x77u32, 0x88u32]);
        assert_eq!(e.mem.u32(object + 4), 0x77);
        assert_eq!(e.mem.u32(object + 8), 0x88);
        e.call(0x00d2_c3a0, &args![object, 0u32, 8u32]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0x88);
    }

    #[test]
    fn the_constructor_clears_stores_the_table_and_returns_this() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x138);
        e.mem.write(object, &[0xcd; 0x138]);
        assert_eq!(e.call(0x00d2_c720, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        // Not named by the constructor: +8.
        assert_eq!(e.mem.u32(object + 8), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        // +0x98 is the class-1 size, written after the clear.
        assert_eq!(e.mem.u32(object + 0x98), 0x10);
        assert_eq!(e.mem.u32(object + 0x90), 0);
        assert_eq!(e.mem.u8(object + 0xd8), 1);
        // Past the second table nothing is written.
        assert_eq!(e.mem.u32(object + 0x124), 0xcdcd_cdcd);
    }
}
