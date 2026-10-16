//! `Havok SDK/hkvector4.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! All four are SSE code on 16-byte aligned `hkVector4` / `hkTransform` memory: the
//! transform is four columns (rotation columns at +0, +0x10, +0x20, translation at +0x30)
//! and every product is a `MULPS` of a broadcast lane with a column, summed in the order
//! the instructions add them (`f32` arithmetic, all four lanes).

#[allow(unused_imports)]
use crate::prelude::*;

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

/// `SHUFPS v, v, 0x00 / 0x55 / 0xaa`: one lane broadcast to all four.
fn splat(v: Lanes, lane: usize) -> Lanes {
    [v[lane]; 4]
}

/// `(y * column1 + x * column0) + z * column2`, the order of `00c7f700` and `00c7f750`.
fn rotate(e: &Engine, matrix: u32, vector: Lanes) -> Lanes {
    let column0 = load(e, matrix);
    let column1 = load(e, matrix + 0x10);
    let column2 = load(e, matrix + 0x20);
    let by_y = mul(splat(vector, 1), column1);
    let by_x = mul(splat(vector, 0), column0);
    let by_z = mul(splat(vector, 2), column2);
    add(add(by_y, by_x), by_z)
}

// Translated from 00c7f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedPos` (Xbox PDB): `this = transform * vector`, the rotation
/// columns weighted by x, y, z plus the translation column.
pub fn set_transformed_pos(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let column0 = load(e, transform.addr());
    let column1 = load(e, transform.addr() + 0x10);
    let column2 = load(e, transform.addr() + 0x20);
    let translation = load(e, transform.addr() + 0x30);
    let by_y = mul(splat(v, 1), column1);
    let by_x = mul(splat(v, 0), column0);
    let by_z = mul(splat(v, 2), column2);
    let rotated = add(by_y, by_x);
    let moved = add(by_z, translation);
    store(e, this.addr(), add(rotated, moved));
}

// Translated from 00c7f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedInversePos` (Xbox PDB): `this = transpose(rotation) *
/// (vector - translation)`. The compiler's transpose of the three columns leaves lane 3 of
/// the first two rows as `column2.y` and of the third as `column2.w` (the `SHUFPS`
/// immediates `0xd4` / `0xe4`), which is what the fourth lane of the result uses.
pub fn set_transformed_inverse_pos(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let translation = load(e, transform.addr() + 0x30);
    let v: Lanes = {
        let raw = load(e, vector.addr());
        std::array::from_fn(|lane| raw[lane] - translation[lane])
    };
    let c0 = load(e, transform.addr());
    let c1 = load(e, transform.addr() + 0x10);
    let c2 = load(e, transform.addr() + 0x20);
    let row0 = [c0[0], c1[0], c2[0], c2[1]];
    let row1 = [c0[1], c1[1], c2[1], c2[3]];
    let row2 = [c0[2], c1[2], c2[2], c2[3]];
    let by_y = mul(splat(v, 1), row1);
    let by_x = mul(splat(v, 0), row0);
    let by_z = mul(splat(v, 2), row2);
    store(e, this.addr(), add(add(by_y, by_x), by_z));
}

// Translated from 00c7f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setRotatedDir` (Xbox PDB): `this = rotation * vector` (no translation).
pub fn set_rotated_dir(e: &mut Engine, this: Ptr, rotation: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let result = rotate(e, rotation.addr(), v);
    store(e, this.addr(), result);
}

// Translated from 00c7f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setMul3` (Xbox PDB): the same instructions as `setRotatedDir`
/// (`00c7f700`), a 3x3 matrix times a vector.
pub fn set_mul3(e: &mut Engine, this: Ptr, matrix: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let result = rotate(e, matrix.addr(), v);
    store(e, this.addr(), result);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c7_f640, set_transformed_pos(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f690, set_transformed_inverse_pos(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f700, set_rotated_dir(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f750, set_mul3(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 4x4 transform with distinct columns, a vector and an output block.
    fn setup(e: &mut Engine) -> (u32, u32, u32) {
        let transform = e.mem.alloc(0x40);
        let columns: [Lanes; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [0.5, 0.25, 0.125, 2.0],
        ];
        for (i, c) in columns.iter().enumerate() {
            store(e, transform + 16 * i as u32, *c);
        }
        let vector = e.mem.alloc(0x10);
        store(e, vector, [1.0, 2.0, 3.0, 9.0]);
        let out = e.mem.alloc(0x10);
        (transform, vector, out)
    }

    #[test]
    fn transformed_pos_adds_the_translation_column() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f640, &args![out, t, v]);
        // x*c0 + y*c1 + z*c2 + c3, per lane
        assert_eq!(load(&e, out), [38.5, 44.25, 50.125, 58.0]);
    }

    #[test]
    fn transformed_inverse_pos_uses_the_transposed_rows() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f690, &args![out, t, v]);
        // d = vector - translation; the rows as the shuffles leave them are (1,5,9,10),
        // (2,6,10,12), (3,7,11,12): lane 3 of rows 0 and 1 is column2.y and column2.w.
        let (dx, dy, dz) = (0.5f32, 1.75f32, 2.875f32);
        let want = [
            dy * 2.0 + dx * 1.0 + dz * 3.0,
            dy * 6.0 + dx * 5.0 + dz * 7.0,
            dy * 10.0 + dx * 9.0 + dz * 11.0,
            dy * 12.0 + dx * 10.0 + dz * 12.0,
        ];
        assert_eq!(load(&e, out), want);
    }

    #[test]
    fn rotated_dir_and_mul3_ignore_the_translation() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f700, &args![out, t, v]);
        assert_eq!(load(&e, out), [38.0, 44.0, 50.0, 58.0 - 2.0]);
        let out2 = e.mem.alloc(0x10);
        e.call(0x00c7_f750, &args![out2, t, v]);
        assert_eq!(load(&e, out2), load(&e, out));
    }
}
