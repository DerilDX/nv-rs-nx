//! `Havok SDK/hkrotation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `(1.0, 0, 0, 0)` as four `float`s.
const UNIT_FIRST_LANE_010C7180: u32 = 0x010c_7180;
/// Sign-bit mask for lanes 1 to 3 (`0, -0, -0, -0`).
const SIGN_LANES_1_TO_3_010C8780: u32 = 0x010c_8780;
/// Sign-bit mask for lanes 0, 2 and 3 (`-0, 0, -0, -0`).
const SIGN_LANES_0_2_3_010C8770: u32 = 0x010c_8770;

/// Four lanes of `float` at `addr`.
fn read_lanes_00cb2d90(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(addr + offset))
}

/// `XORPS` with the sign mask stored at `mask`.
fn xor_lanes_00cb2d90(e: &Engine, lanes: [f32; 4], mask: u32) -> [f32; 4] {
    let mut out = lanes;
    for (i, lane) in out.iter_mut().enumerate() {
        *lane = f32::from_bits(lane.to_bits() ^ e.mem.u32(mask + 4 * i as u32));
    }
    out
}

/// Lane-wise `op` of two four-lane values (`MULPS`, `SUBPS`).
fn zip_00cb2d90(a: [f32; 4], b: [f32; 4], op: fn(f32, f32) -> f32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| op(a[i], b[i]))
}

// Translated from 00cb2d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkrotation.cpp`). Thiscall, one word
/// (`RET 4`), SSE (`f32` per lane): writes the rotation matrix of the
/// quaternion `(x, y, z, w)` at `quaternion` as three four-float columns at
/// `this`, `this + 0x10` and `this + 0x20` (the fourth float of the last
/// column is 0):
/// column 0 = `(1 - 2z*z - 2y*y, 2wz + 2yx, 2zx - 2wy, 2zy - 2wx)`,
/// column 1 = `(2yx - 2wz, 1 - 2z*z - 2x*x, 2wx + 2zy, 2wy + 2zx)`,
/// column 2 = `(2wy + 2zx, 2zy - 2wx, 1 - 2x*x - 2y*y, 0)`; every
/// subtraction `a - (-b)` is kept as the code computes it, with the sign
/// flips done by `XORPS` on the masks in the exe's data.
pub fn fn_00cb2d90(e: &mut Engine, this: Ptr, quaternion: Ptr) {
    let q = read_lanes_00cb2d90(e, quaternion.addr());
    let one = read_lanes_00cb2d90(e, UNIT_FIRST_LANE_010C7180);
    let twice = [q[0] + q[0], q[1] + q[1], q[2] + q[2], q[3] + q[3]];
    // (y, x, x, y) and (2y, 2y, 2z, 2z).
    let swapped = [q[1], q[0], q[0], q[1]];
    let twice_pairs = [twice[1], twice[1], twice[2], twice[2]];
    // `MULSS`: only the first lane of the copy of `q` is multiplied.
    let x_twice_x = q[0] * twice[0];
    // (z, w, w, w) * (2z, 2z, 2y, 2x), sign-flipped in lanes 0, 2, 3.
    let products = zip_00cb2d90(
        [q[2], q[3], q[3], q[3]],
        [twice[2], twice[2], twice[1], twice[0]],
        |a, b| a * b,
    );
    let mut flipped = xor_lanes_00cb2d90(e, products, SIGN_LANES_0_2_3_010C8770);
    let pairs = zip_00cb2d90(twice_pairs, swapped, |a, b| a * b);
    // `ADDSS`: first lane only.
    flipped[0] += one[0];
    let pairs_negated = xor_lanes_00cb2d90(e, pairs, SIGN_LANES_1_TO_3_010C8780);
    let column0 = zip_00cb2d90(flipped, pairs_negated, |a, b| a - b);
    let unflipped = xor_lanes_00cb2d90(e, flipped, SIGN_LANES_1_TO_3_010C8780);
    // `MOVSS XMM2, XMM4`: the first lane of the negated pairs is replaced.
    let mut subtrahend = pairs_negated;
    subtrahend[0] = x_twice_x;
    let mut one_minus = one;
    for (i, lane) in column0.iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * i as u32, *lane);
    }
    let difference = zip_00cb2d90(unflipped, subtrahend, |a, b| a - b);
    // `SHUFPS 0xb1`: (1, 0, 3, 2).
    let column1 = [difference[1], difference[0], difference[3], difference[2]];
    for (i, lane) in column1.iter().enumerate() {
        e.mem.set_f32(this.addr() + 0x10 + 4 * i as u32, *lane);
    }
    one_minus[0] -= x_twice_x;
    // `MOVHLPS` then `SHUFPS 0x8d`: (column1[3], column0[3], first, third).
    one_minus[0] -= pairs[0];
    let column2 = [column1[3], column0[3], one_minus[0], one_minus[2]];
    for (i, lane) in column2.iter().enumerate() {
        e.mem.set_f32(this.addr() + 0x20 + 4 * i as u32, *lane);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cb2d90, fn_00cb2d90(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotation_engine_00cb2d90() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_7000, 0x2000);
        e.mem.set_f32(UNIT_FIRST_LANE_010C7180, 1.0);
        for lane in 1..4 {
            e.mem
                .set_u32(SIGN_LANES_1_TO_3_010C8780 + 4 * lane, 0x8000_0000);
        }
        for lane in [0, 2, 3] {
            e.mem
                .set_u32(SIGN_LANES_0_2_3_010C8770 + 4 * lane, 0x8000_0000);
        }
        e
    }

    fn matrix_00cb2d90(e: &mut Engine, quaternion: [f32; 4]) -> [[f32; 4]; 3] {
        let input = e.mem.alloc(16);
        for (i, value) in quaternion.iter().enumerate() {
            e.mem.set_f32(input + 4 * i as u32, *value);
        }
        let out = e.mem.alloc(0x30);
        e.call(
            0x00cb_2d90,
            &args![Ptr::<()>::new(out), Ptr::<()>::new(input)],
        );
        [0, 0x10, 0x20].map(|column| [0, 4, 8, 12].map(|lane| e.mem.f32(out + column + lane)))
    }

    #[test]
    fn fn_00cb2d90_identity_quaternion_gives_the_identity_columns() {
        let mut e = rotation_engine_00cb2d90();
        let m = matrix_00cb2d90(&mut e, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(m[0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(m[1], [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(m[2], [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00cb2d90_quarter_turn_about_z() {
        let mut e = rotation_engine_00cb2d90();
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = matrix_00cb2d90(&mut e, [0.0, 0.0, s, s]);
        let near = |a: f32, b: f32| (a - b).abs() < 1e-6;
        // The columns are the rotated x, y and z axes.
        assert!(near(m[0][0], 0.0) && near(m[0][1], 1.0) && near(m[0][2], 0.0));
        assert!(near(m[1][0], -1.0) && near(m[1][1], 0.0) && near(m[1][2], 0.0));
        assert!(near(m[2][0], 0.0) && near(m[2][1], 0.0) && near(m[2][2], 1.0));
        assert_eq!([m[0][3], m[1][3], m[2][3]], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00cb2d90_half_turn_about_x_flips_y_and_z() {
        let mut e = rotation_engine_00cb2d90();
        let m = matrix_00cb2d90(&mut e, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(m[0][..3], [1.0, 0.0, 0.0]);
        assert_eq!(m[1][..3], [0.0, -1.0, 0.0]);
        assert_eq!(m[2][..3], [0.0, 0.0, -1.0]);
    }
}
