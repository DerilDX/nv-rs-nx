//! `fallout shared/pathfinding/run_0069dd50` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005eeda0` (thiscall, index): the address of the 2-byte element `index`
/// of the array whose data is at +4.
const WORD_ELEMENT: u32 = 0x005e_eda0;
/// `0069d2d0` (thiscall): releases the destination's cell array (+0x24).
const RELEASE_CELLS: u32 = 0x0069_d2d0;
/// `00401000` (cdecl, byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `__ehvec_ctor` (`00ec782f`, CRT): constructs an array of elements.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// The cell constructor and destructor `0069dd50` passes to it.
const CELL_CONSTRUCT: u32 = 0x0069_17d0;
const CELL_DESTRUCT: u32 = 0x0069_1800;
/// `00694f80` (thiscall, destination cell): copies one 0x10-byte cell.
const COPY_CELL: u32 = 0x0069_4f80;

// Translated from 0069dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `005eeda0(this, index)`, the address of the
/// 2-byte element `index`.
pub fn fn_0069dec0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(WORD_ELEMENT, &args![this, index]).u32()
}

// Translated from 0069dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the word at +0x54 is non-zero.
pub fn fn_0069dfb0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x54)) != 0
}

// Translated from 0069dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies this grid into `dest`: releases
/// `dest`'s cells (`0069d2d0`), copies the size word (+0), the two floats
/// (+4, +8, through x87 loads and stores), the two 12-byte vectors (+0xC,
/// +0x18), then allocates `size * size` cells of 0x10 bytes (with the count
/// word in front, constructed by `__ehvec_ctor`; the byte count saturates to
/// 0xFFFFFFFF on overflow as the compiler's array new does), stores them at
/// `dest + 0x24` and copies each cell (`00694f80`).
// Translation incomplete: 0069dd50 - SEH exception unwinding (the allocation's cleanup on unwind) is unsupported.
pub fn fn_0069dd50(e: &mut Engine, this: Ptr, dest: Ptr) {
    let (t, d) = (this.addr(), dest.addr());
    e.call(RELEASE_CELLS, &args![dest]);
    let size = e.mem.u32(t);
    e.mem.set_u32(d, size);
    let first = x87_quiet_bits(e.mem.u32(t + 4));
    e.mem.set_u32(d + 4, first);
    let second = x87_quiet_bits(e.mem.u32(t + 8));
    e.mem.set_u32(d + 8, second);
    for offset in [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
        let word = e.mem.u32(t + offset);
        e.mem.set_u32(d + offset, word);
    }
    let count = e.mem.u32(d).wrapping_mul(e.mem.u32(d));
    let bytes = count.saturating_mul(0x10);
    let bytes = bytes.saturating_add(4);
    let block = e.call(ALLOCATE_BLOCK, &args![bytes]).u32();
    let cells = if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCT,
            &args![block + 4, 0x10u32, count, CELL_CONSTRUCT, CELL_DESTRUCT],
        );
        block + 4
    } else {
        0
    };
    e.mem.set_u32(d + 0x24, cells);
    let mut index = 0u32;
    while index < count {
        let target = (index << 4).wrapping_add(e.mem.u32(d + 0x24));
        let source = (index << 4).wrapping_add(e.mem.u32(t + 0x24));
        e.call(COPY_CELL, &args![source, target]);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_dec0, fn_0069dec0(Ptr, u32) -> u32),
        entry!(0x0069_dfb0, fn_0069dfb0(Ptr) -> bool),
        entry!(0x0069_dd50, fn_0069dd50(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0069dec0_forwards_to_the_element_address() {
        let mut e = Engine::new();
        e.register(WORD_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + 2 * a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x5000);
        assert_eq!(e.call(0x0069_dec0, &args![this, 3u32]).u32(), 0x5006);
    }

    #[test]
    fn fn_0069dfb0_tests_the_word_at_54() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        assert!(!e.call(0x0069_dfb0, &args![this]).bool());
        e.mem.set_u32(this + 0x54, 0x100);
        assert!(e.call(0x0069_dfb0, &args![this]).bool());
    }

    #[test]
    fn fn_0069dd50_copies_the_header_and_every_cell() {
        let mut e = Engine::new();
        e.register(RELEASE_CELLS, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_CELL, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 0x7f80_0001); // a signalling NaN
        e.mem.set_f32(this + 8, 1.5);
        for i in 3..9u32 {
            e.mem.set_u32(this + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(this + 0x24, 0x6000);
        let dest = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x0069_dd50, &args![this, dest]);
        assert_eq!(e.mem.u32(dest), 2);
        assert_eq!(e.mem.u32(dest + 4), 0x7fc0_0001, "FLD/FSTP quiets it");
        assert_eq!(e.mem.f32(dest + 8), 1.5);
        for i in 3..9u32 {
            assert_eq!(e.mem.u32(dest + 4 * i), 0x100 + i);
        }
        let cells = e.mem.u32(dest + 0x24);
        assert_eq!(e.mem.u32(cells - 4), 4);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|c| c.0 == ALLOCATE_BLOCK).collect();
        assert_eq!(allocations[0].1, vec![0x44]);
        let built: Vec<_> = log.iter().filter(|c| c.0 == VECTOR_CONSTRUCT).collect();
        assert_eq!(
            built[0].1,
            vec![cells, 0x10, 4, CELL_CONSTRUCT, CELL_DESTRUCT]
        );
        let copies: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == COPY_CELL)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            copies,
            (0..4)
                .map(|i| vec![0x6000 + 0x10 * i, cells + 0x10 * i])
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fn_0069dd50_wraps_the_cell_count_and_survives_a_failed_allocation() {
        let mut e = Engine::new();
        e.register(RELEASE_CELLS, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this, 0x1_0000);
        let dest = e.mem.alloc(0x28);
        e.mem.set_u32(dest + 0x24, 0x1234);
        e.call_log = Some(vec![]);
        // 0x10000 * 0x10000 wraps to 0: no cells, 4 bytes.
        e.call(0x0069_dd50, &args![this, dest]);
        assert_eq!(e.call_log.take().unwrap()[2], (ALLOCATE_BLOCK, vec![4]));
        assert_eq!(e.mem.u32(dest + 0x24), 0);
    }
}
