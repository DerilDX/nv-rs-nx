//! `BSHavok/hknitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriStripsData::GetStripLists` (`00a75f10`, `this` = the strips data): answers the pointer
/// to the 16-bit strip indices.
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;

// Translated from 00ca4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the triangle `key` of the shape is degenerate. The key's high 12 bits pick the
/// strips data (an array of 8-byte entries at `this + 0x98`), its low 20 bits the triangle:
/// the strips (lengths at data `+0x48`, count at `+0x44`) are walked, each strip of length
/// `n` holding `n - 2` triangles, until the triangle falls in one; the result is true when
/// two of the three consecutive indices of that triangle are equal.
pub fn fn_00ca4520(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let table = e.mem.u32(this.addr() + 0x98);
    let data = e.mem.u32(table + (key >> 20) * 8);
    let strip_count = e.mem.u16(data + 0x44);
    let mut indices = e.call(GET_STRIP_LISTS, &args![data]).u32();
    let lengths = e.mem.u32(data + 0x48);
    let mut triangle = key & 0xfffff;
    let mut strip: u16 = 0;
    if strip_count != 0 {
        loop {
            let length = u32::from(e.mem.u16(lengths + u32::from(strip) * 2));
            if triangle < length.wrapping_sub(2) {
                break;
            }
            triangle = triangle.wrapping_add(2u32.wrapping_sub(length));
            indices = indices.wrapping_add(length * 2);
            strip = strip.wrapping_add(1);
            if strip >= strip_count {
                break;
            }
        }
    }
    let at = indices.wrapping_add(triangle.wrapping_mul(2));
    let first = e.mem.u16(at);
    let second = e.mem.u16(at + 2);
    let third = e.mem.u16(at + 4);
    first == second || first == third || second == third
}

// Translated from 00ca4ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of the next triangle after `key` that is not degenerate, or -1 when there is none.
/// `this` is the sub-object 0x10 bytes into the shape that `00ca4520` takes. Each step adds 1;
/// when the low 20 bits reach the strips data's count (the word at `+0x40` of the entry at
/// `this + 0x88`), the key moves to the start of the next data (the high 12 bits plus 1); a key
/// whose data index reaches the count at `this + 0x90` ends the search.
pub fn fn_00ca4ac0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut key = key;
    loop {
        let table = e.mem.u32(this.addr() + 0x88);
        key = key.wrapping_add(1);
        let data_index = key >> 20;
        let data = e.mem.u32(table + data_index * 8);
        let count = i32::from(e.mem.u16(data + 0x40));
        if ((key & 0xfffff) as i32) >= count {
            key = (data_index + 1) << 20;
        }
        if ((key >> 20) as i32) >= e.mem.i32(this.addr() + 0x90) {
            return u32::MAX;
        }
        let degenerate = fn_00ca4520(e, Ptr::new(this.addr().wrapping_sub(0x10)), key);
        if !degenerate {
            return key;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_4520, fn_00ca4520(Ptr, u32) -> bool),
        entry!(0x00ca_4ac0, fn_00ca4ac0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A shape with one strips data of two strips (lengths 5 and 4): the indices
    /// `0 1 2 3 4` and `5 5 6 7`. Returns the shape (`+0x98` = table) and the sub-object 0x10
    /// bytes into it (`+0x88` = table, `+0x90` = 1 data; the shape fields are `+0x98` and `+0xa0`).
    fn rig() -> (Engine, u32, u32) {
        let mut e = test_engine();
        let indices = e.mem.alloc(0x20);
        for (i, v) in [0u16, 1, 2, 3, 4, 5, 5, 6, 7].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *v);
        }
        let lengths = e.mem.alloc(8);
        e.mem.set_u16(lengths, 5);
        e.mem.set_u16(lengths + 2, 4);
        let data = e.mem.alloc(0x60);
        e.mem.set_u16(data + 0x40, 5);
        e.mem.set_u16(data + 0x44, 2);
        e.mem.set_u32(data + 0x48, lengths);
        e.register_double(GET_STRIP_LISTS, move |_, a| {
            assert_eq!(a[0], data);
            Ret {
                eax: indices,
                ..Ret::default()
            }
        });
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, data);
        let shape = e.mem.alloc(0xb0);
        e.mem.set_u32(shape + 0x98, table);
        e.mem.set_u32(shape + 0xa0, 1);
        (e, shape, shape + 0x10)
    }

    #[test]
    fn a_triangle_is_degenerate_when_two_indices_match() {
        let (mut e, shape, _) = rig();
        let degenerate = |e: &mut Engine, key: u32| e.call(0x00ca_4520, &args![shape, key]).bool();
        assert!(!degenerate(&mut e, 0));
        assert!(!degenerate(&mut e, 2));
        // The first triangle of the second strip is 5, 5, 6.
        assert!(degenerate(&mut e, 3));
        assert!(!degenerate(&mut e, 4));
    }

    #[test]
    fn the_next_valid_triangle_skips_degenerate_ones() {
        let (mut e, _, sub) = rig();
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 2u32]).u32(), 4);
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 0u32]).u32(), 1);
        // Past the last triangle the key moves to the next data, which does not exist.
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 4u32]).u32(), u32::MAX);
    }
}
