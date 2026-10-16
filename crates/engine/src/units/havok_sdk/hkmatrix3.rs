//! `Havok SDK/hkmatrix3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkMatrix3` is three 16-byte columns (`x`, `y`, `z` lanes used, the fourth lane is
//! padding). The SSE code multiplies with `MULPS` of a broadcast lane by a column and adds
//! the three products in the order the instructions add them (`f32`, all four lanes).

#[allow(unused_imports)]
use crate::prelude::*;

/// The SSE sign-clearing mask (four words, `0x7fffffff`) `isApproximatelyEqual` loads.
const ABS_MASK: u32 = 0x0101_7d10;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// `FLD dword [from]; FSTP dword [to]`: a copy that quiets a signalling NaN.
fn copy_through_x87(e: &mut Engine, to: u32, from: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00cd4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setTranspose` (Xbox PDB): copies the diagonal (+0, +0x14, +0x28), zeroes the
/// padding lanes (+0xC, +0x1C, +0x2C) and swaps the off-diagonal pairs, element by element
/// in the exe's order (so `this == source` gives the exe's partial result), every copy
/// through the x87.
pub fn hk_matrix3_set_transpose(e: &mut Engine, this: Ptr, source: Ptr) {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 0x14, 0x28] {
        copy_through_x87(e, to + offset, from + offset);
    }
    for offset in [0xc, 0x1c, 0x2c] {
        e.mem.set_u32(to + offset, 0);
    }
    for (destination, origin) in [
        (0x4, 0x10),
        (0x10, 0x4),
        (0x8, 0x20),
        (0x20, 0x8),
        (0x18, 0x24),
        (0x24, 0x18),
    ] {
        copy_through_x87(e, to + destination, from + origin);
    }
}

// Translated from 00cd4e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::isApproximatelyEqual` (Xbox PDB): writes 1 to the byte `out` when, for each of
/// the three columns, `|this - other| <= epsilon` in lanes 0..2 (`CMPLEPS`: false for a NaN;
/// the absolute value is the `ANDPS` with the mask at `01017d10`), else 0. Stops at the
/// first column that differs. Returns `out`.
pub fn hk_matrix3_is_approximately_equal(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    other: Ptr,
    epsilon: f32,
) -> Ptr {
    let mut equal = true;
    for column in 0..3u32 {
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        let theirs = load_lanes(e, other.addr() + 0x10 * column);
        let within = (0..3).all(|lane| {
            let mask = e.mem.u32(ABS_MASK + 4 * lane as u32);
            let difference = mine[lane] - theirs[lane];
            f32::from_bits(difference.to_bits() & mask) <= epsilon
        });
        if !within {
            equal = false;
            break;
        }
    }
    e.mem.set_u8(out.addr(), u8::from(equal));
    out
}

// Translated from 00cd55b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setMul` (second overload, Xbox PDB): `this = a * b`. The columns of `a` are
/// loaded first; then for the columns of `b` from the last to the first,
/// `this.column = (x * a0 + y * a1) + z * a2` (`x`, `y`, `z` the lanes of that column of
/// `b`), four lanes.
pub fn hk_matrix3_set_mul_ov2(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let columns = [
        load_lanes(e, a.addr()),
        load_lanes(e, a.addr() + 0x10),
        load_lanes(e, a.addr() + 0x20),
    ];
    for column in (0..3u32).rev() {
        let v = load_lanes(e, b.addr() + 0x10 * column);
        let result: Lanes = std::array::from_fn(|lane| {
            (v[0] * columns[0][lane] + v[1] * columns[1][lane]) + v[2] * columns[2][lane]
        });
        store_lanes(e, this.addr() + 0x10 * column, result);
    }
}

// Translated from 00cd5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::mul` (Xbox PDB): `this = this * other`, through a temporary
/// (`setMul_ov2(temp, this, other)`, then the three columns copied back).
pub fn hk_matrix3_mul(e: &mut Engine, this: Ptr, other: Ptr) {
    e.with_stack(0x30, |e, temp| {
        hk_matrix3_set_mul_ov2(e, temp, this, other);
        for offset in (0..0x30).step_by(4) {
            let word = e.mem.u32(temp.addr() + offset);
            e.mem.set_u32(this.addr() + offset, word);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_4ba0, hk_matrix3_set_transpose(Ptr, Ptr)),
        entry!(
            0x00cd_4e00,
            hk_matrix3_is_approximately_equal(Ptr, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x00cd_55b0, hk_matrix3_set_mul_ov2(Ptr, Ptr, Ptr)),
        entry!(0x00cd_5680, hk_matrix3_mul(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(e: &mut Engine, columns: [[f32; 3]; 3]) -> u32 {
        let m = e.mem.alloc(0x30);
        for (i, c) in columns.iter().enumerate() {
            store_lanes(e, m + 16 * i as u32, [c[0], c[1], c[2], 9.0]);
        }
        m
    }

    fn elements(e: &Engine, m: u32) -> [[f32; 3]; 3] {
        std::array::from_fn(|c| {
            let lanes = load_lanes(e, m + 16 * c as u32);
            [lanes[0], lanes[1], lanes[2]]
        })
    }

    #[test]
    fn transpose_swaps_pairs_and_zeroes_the_padding() {
        let mut e = Engine::new();
        let source = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let dest = e.mem.alloc(0x30);
        e.mem.set_u32(dest + 0xc, 0xffff_ffff);
        e.call(0x00cd_4ba0, &args![dest, source]);
        assert_eq!(
            elements(&e, dest),
            [[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]]
        );
        for padding in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(dest + padding), 0);
        }
    }

    #[test]
    fn transpose_quiets_signalling_nans() {
        let mut e = Engine::new();
        let source = matrix(&mut e, [[0.0; 3]; 3]);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 0x10, 0xff80_0002);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cd_4ba0, &args![dest, source]);
        assert_eq!(e.mem.u32(dest), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 4), 0xffc0_0002);
    }

    fn setup_mask(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
    }

    #[test]
    fn approximate_equality_checks_each_column_and_ignores_the_fourth_lane() {
        let mut e = Engine::new();
        setup_mask(&mut e);
        let a = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = matrix(&mut e, [[1.1, 1.9, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let out = e.mem.alloc(4);
        let got = e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cd_4e00, &args![a, out, b, 0.05f32]);
        assert_eq!(e.mem.u8(out), 0);
        // only the third column differs
        store_lanes(&mut e, b, [1.0, 2.0, 3.0, 50.0]);
        store_lanes(&mut e, b + 0x20, [7.0, 8.0, 9.5, 9.0]);
        e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]);
        assert_eq!(e.mem.u8(out), 0);
        // the padding lane of every column differs; still equal
        store_lanes(&mut e, b + 0x20, [7.0, 8.0, 9.0, 123.0]);
        e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]);
        assert_eq!(e.mem.u8(out), 1);
        // NaN compares false
        e.mem.set_f32(b + 4, f32::NAN);
        e.call(0x00cd_4e00, &args![a, out, b, 100.0f32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn set_mul_ov2_multiplies_column_by_column() {
        let mut e = Engine::new();
        let a = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]);
        let b = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cd_55b0, &args![dest, a, b]);
        // column of b = (x, y, z): x * a0 + y * a1 + z * a2
        assert_eq!(
            elements(&e, dest),
            [[1.0, -3.0, 2.0], [4.0, -6.0, 5.0], [7.0, -9.0, 8.0]]
        );
        // the fourth lane: (1 + 2) * 9 + 3 * 9 from the padding lanes of a
        assert_eq!(e.mem.f32(dest + 12), 54.0);
    }

    #[test]
    fn mul_replaces_this_with_this_times_other() {
        let mut e = Engine::new();
        let this = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]);
        let other = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        e.call(0x00cd_5680, &args![this, other]);
        assert_eq!(
            elements(&e, this),
            [[1.0, -3.0, 2.0], [4.0, -6.0, 5.0], [7.0, -9.0, 8.0]]
        );
    }
}
