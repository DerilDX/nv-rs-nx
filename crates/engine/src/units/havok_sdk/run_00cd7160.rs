//! `Havok SDK/run_00cd7160` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Triangle helpers of the capsule collision code. The vectors are SSE (`f32` lanes, the
//! three-component sums added as `z + (y + x)`); the scalar solving runs on the x87 stack,
//! approximated here in `f64` with a rounding to `f32` wherever the code stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesobjectrefr::rsqrt_estimate;

type Vec4 = [f32; 4];

fn load(e: &Engine, at: Ptr) -> Vec4 {
    let at = at.addr();
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// The sum of the three lane products added as `z + (y + x)`.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2]];
    p[2] + (p[1] + p[0])
}

// Translated from 00cd72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reciprocal edge lengths of a triangle, cdecl `(triangle, out)`. `triangle` holds the
/// vertices `a`, `b`, `c` as three 16-byte vectors. Writes to `out` the refined inverse square
/// roots of `|c - b|^2`, `|a - c|^2` and `|b - a|^2` (floats 0 to 2), then that of the squared
/// length of the face normal `(a - c).zxy * (c - b).yzx - (a - c).yzx * (c - b).zxy` (float 3),
/// then the squared normal length times its inverse root (float 4: the normal's length).
/// The root is the hardware estimate (`RSQRTPS`) refined by one Newton step,
/// `(3 - (r * x) * r) * (0.5 * r)`.
pub fn fn_00cd72a0(e: &mut Engine, triangle: Ptr, out: Ptr) {
    let a = load(e, triangle);
    let b = load(e, Ptr::new(triangle.addr() + 0x10));
    let c = load(e, Ptr::new(triangle.addr() + 0x20));
    let to_a = sub(a, c);
    let from_b = sub(c, b);
    let to_b = sub(b, a);
    let normal = [
        to_a[2] * from_b[1] - to_a[1] * from_b[2],
        to_a[0] * from_b[2] - to_a[2] * from_b[0],
        to_a[1] * from_b[0] - to_a[0] * from_b[1],
        0.0,
    ];
    let squares = [
        dot3(from_b, from_b),
        dot3(to_a, to_a),
        dot3(to_b, to_b),
        dot3(normal, normal),
    ];
    let refined = squares.map(|x| {
        let estimate = rsqrt_estimate(x);
        (3.0 - (estimate * x) * estimate) * (0.5 * estimate)
    });
    for (lane, value) in refined.into_iter().enumerate() {
        e.mem
            .set_f32(out.addr() + 4 * lane as u32, x87_quiet(value));
    }
    let length = (f64::from(squares[3]) * f64::from(refined[3])) as f32;
    e.mem.set_f32(out.addr() + 0x10, length);
}

// Translated from 00cd7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Barycentric coordinates of the point `p` against the triangle `a`, `b`, `c`, cdecl
/// `(p, a, b, c, out)`: writes the weights of `a`, `b` and `c` (3 floats at `out`) of the
/// point of the triangle closest to `p`. With `e1 = a - b`, `e2 = c - b` and a positive
/// determinant `|e1|^2 * |e2|^2 - (e1 . e2)^2` the weights are the plane projection's.
/// For a degenerate triangle the point is projected on the longest edge instead (`a`-`b`,
/// `b`-`c` or `a`-`c`; the weights of the other vertex are 0), and when every edge has
/// length 0 all three weights are 0.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn fn_00cd7530(e: &mut Engine, p: Ptr, a: Ptr, b: Ptr, c: Ptr, out: Ptr) {
    let point = load(e, p);
    let va = load(e, a);
    let vb = load(e, b);
    let vc = load(e, c);
    let e1 = sub(va, vb);
    let e2 = sub(vc, vb);
    let n1 = dot3(e1, e1);
    let n2 = dot3(e2, e2);
    let d = dot3(e1, e2);
    let n1n2 = (f64::from(n1) * f64::from(n2)) as f32;
    let d2 = (f64::from(d) * f64::from(d)) as f32;
    let det = (f64::from(n1n2) - f64::from(d2)) as f32;
    let out = out.addr();
    if det > 0.0 {
        let inv = (1.0 / f64::from(det)) as f32;
        let from_point = sub(vb, point);
        let p1 = dot3(from_point, e1);
        let p2 = dot3(from_point, e2);
        let (d, n1, n2) = (f64::from(d), f64::from(n1), f64::from(n2));
        let x = (f64::from(p2) * d - n2 * f64::from(p1)) as f32;
        let y = (f64::from(p1) * d - n1 * f64::from(p2)) as f32;
        let (x_d, y_d, inv_d) = (f64::from(x), f64::from(y), f64::from(inv));
        e.mem.set_f32(out, (x_d * inv_d) as f32);
        e.mem
            .set_f32(out + 4, (((f64::from(det) - x_d) - y_d) * inv_d) as f32);
        e.mem.set_f32(out + 8, (y_d * inv_d) as f32);
        return;
    }
    let e3 = sub(va, vc);
    let n3 = dot3(e3, e3);
    // The fraction of the edge `from` + `edge`, `length` long, that `p` projects on.
    let fraction = |from: Vec4, edge: Vec4, length: f32| -> f32 {
        let along = dot3(sub(point, from), edge);
        (f64::from(along) / f64::from(length)) as f32
    };
    if !(n1 >= n2) {
        if !(n3 >= n2) {
            let r = fraction(vb, e2, n2);
            e.mem.set_f32(out, 0.0);
            e.mem.set_f32(out + 8, r);
            e.mem.set_f32(out + 4, 1.0 - r);
        } else {
            let q = fraction(vc, e3, n3);
            e.mem.set_f32(out, q);
            e.mem.set_f32(out + 4, 0.0);
            e.mem.set_f32(out + 8, 1.0 - q);
        }
    } else if !(n3 >= n1) {
        let r = fraction(vb, e1, n1);
        e.mem.set_f32(out, r);
        e.mem.set_f32(out + 4, 1.0 - r);
        e.mem.set_f32(out + 8, 0.0);
    } else {
        e.mem.set_f32(out + 4, 0.0);
        if 0.0 < n3 {
            let q = fraction(vc, e3, n3);
            e.mem.set_f32(out, q);
            e.mem.set_f32(out + 8, 1.0 - q);
        } else {
            e.mem.set_f32(out, 0.0);
            e.mem.set_f32(out + 8, 0.0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd72a0, fn_00cd72a0(Ptr, Ptr)),
        entry!(0x00cd7530, fn_00cd7530(Ptr, Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x40);
        for (i, c) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, c);
        }
        e.mem.set_f32(at + 12, 1.0);
        Ptr::new(at)
    }

    fn close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 1e-4 * expected.abs().max(1.0)
    }

    #[test]
    fn edge_lengths_and_normal_length_are_inverted() {
        let mut e = Engine::new();
        let triangle = e.mem.alloc(0x30);
        for (i, v) in [[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 4.0, 0.0]]
            .into_iter()
            .enumerate()
        {
            for (j, c) in v.into_iter().enumerate() {
                e.mem.set_f32(triangle + 0x10 * i as u32 + 4 * j as u32, c);
            }
        }
        let out = e.mem.alloc(0x20);
        e.call(0x00cd72a0, &args![triangle, out]);
        // |c - b| = 5, |a - c| = 4, |b - a| = 3, |normal| = 12.
        assert!(close(e.mem.f32(out), 0.2));
        assert!(close(e.mem.f32(out + 4), 0.25));
        assert!(close(e.mem.f32(out + 8), 1.0 / 3.0));
        assert!(close(e.mem.f32(out + 12), 1.0 / 12.0));
        assert!(close(e.mem.f32(out + 16), 12.0));
    }

    fn weights(points: [[f32; 3]; 4]) -> [f32; 3] {
        let mut e = Engine::new();
        let [p, a, b, c] = points.map(|v| vector(&mut e, v));
        let out = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(out + 4 * i, 9.0);
        }
        e.call(0x00cd7530, &args![p, a, b, c, out]);
        [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)]
    }

    const TRIANGLE: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    #[test]
    fn a_nondegenerate_triangle_gives_the_plane_coordinates() {
        let [a, b, c] = TRIANGLE;
        assert_eq!(weights([a, a, b, c]), [1.0, 0.0, 0.0]);
        assert_eq!(weights([c, a, b, c]), [0.0, 0.0, 1.0]);
        assert_eq!(weights([b, a, b, c]), [0.0, 1.0, 0.0]);
    }

    #[test]
    fn collinear_points_project_on_the_longest_edge() {
        // n1 == n2: the edge from c to a (n3 is longest).
        let w = weights([
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.0, 0.5]);
        // n1 < n2 and n3 >= n2: the edge a-c again.
        let w = weights([
            [1.5, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.0, 0.5]);
    }

    #[test]
    fn collinear_points_can_project_on_either_edge_at_b() {
        // n3 < n2: the edge b-c.
        let w = weights([
            [-1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [-3.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.0, 0.5, 0.5]);
        // n1 > n2 and n3 < n1: the edge a-b.
        let w = weights([
            [1.5, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.5, 0.0]);
    }

    #[test]
    fn a_point_triangle_has_zero_weights() {
        let origin = [0.0, 0.0, 0.0];
        assert_eq!(weights([origin; 4]), [0.0, 0.0, 0.0]);
    }
}
