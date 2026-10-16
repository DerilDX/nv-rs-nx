//! `Havok SDK/run_00e61720` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `1e-7` as a `double` (`.rdata`).
const EPSILON_0109A3A0: u32 = 0x0109_a3a0;

// Translated from 00e63af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, three words (`RET 0xc`). `this + 0` is a plane
/// normal (four floats) and `[this + 0x10]` points at a point on the plane;
/// `dist = (point - plane point) . normal` (SSE, `p2 + (p1 + p0)`) is
/// clamped into the interval `[*low, *high]` with the x87 compares of the
/// code (NaN included, the compares are ordered):
/// - if `*low < dist`: result 1, and `*high` becomes `dist` when
///   `dist < *high`;
/// - otherwise result 2, and `*low` becomes `dist + 1e-7` (a `double`
///   sum, rounded to `float`) when `*low < dist + 1e-7`;
/// - then the result is 4 whenever `*high <= *low`.
pub fn fn_00e63af0(e: &mut Engine, this: Ptr, point: Ptr, low: Ptr, high: Ptr) -> u32 {
    let plane_point = e.mem.u32(this.addr() + 0x10);
    let terms = [0u32, 4, 8].map(|offset| {
        (e.mem.f32(point.addr() + offset) - e.mem.f32(plane_point + offset))
            * e.mem.f32(this.addr() + offset)
    });
    let dist = terms[2] + (terms[1] + terms[0]);
    let result = if e.mem.f32(low.addr()) < dist {
        if dist < e.mem.f32(high.addr()) {
            e.mem.set_f32(high.addr(), dist);
        }
        1
    } else {
        let epsilon = e.mem.f64(EPSILON_0109A3A0);
        let widened = dist as f64 + epsilon;
        if (e.mem.f32(low.addr()) as f64) < widened {
            e.mem.set_f32(low.addr(), widened as f32);
        }
        2
    };
    if e.mem.f32(high.addr()) <= e.mem.f32(low.addr()) {
        4
    } else {
        result
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e63af0, fn_00e63af0(Ptr, Ptr, Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plane with normal (0, 0, 1) through the origin and the point
    /// (0, 0, z); returns the result and the interval afterwards.
    fn clip_00e63af0(z: f32, low: f32, high: f32) -> (u32, f32, f32) {
        let mut e = Engine::new();
        e.map(0x0109_a000, 0x1000);
        e.mem.set_f64(EPSILON_0109A3A0, 1.0e-7);
        let origin = e.mem.alloc(16);
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_u32(this + 0x10, origin);
        let point = e.mem.alloc(16);
        e.mem.set_f32(point + 8, z);
        let interval = e.mem.alloc(8);
        e.mem.set_f32(interval, low);
        e.mem.set_f32(interval + 4, high);
        let result = e
            .call(
                0x00e6_3af0,
                &args![
                    Ptr::<()>::new(this),
                    Ptr::<()>::new(point),
                    Ptr::<()>::new(interval),
                    Ptr::<()>::new(interval + 4)
                ],
            )
            .u32();
        (result, e.mem.f32(interval), e.mem.f32(interval + 4))
    }

    #[test]
    fn fn_00e63af0_narrows_the_high_end_when_the_plane_is_inside() {
        assert_eq!(clip_00e63af0(2.0, 0.0, 5.0), (1, 0.0, 2.0));
        assert_eq!(clip_00e63af0(7.0, 0.0, 5.0), (1, 0.0, 5.0));
    }

    #[test]
    fn fn_00e63af0_raises_the_low_end_a_little_past_the_plane() {
        // dist == low: the low end moves by the epsilon.
        let (result, low, high) = clip_00e63af0(1.0, 1.0, 5.0);
        assert_eq!(result, 2);
        assert_eq!(low, (1.0f64 + 1.0e-7) as f32);
        assert_eq!(high, 5.0);
        // Far below the low end: nothing moves.
        assert_eq!(clip_00e63af0(-3.0, 1.0, 5.0), (2, 1.0, 5.0));
    }

    #[test]
    fn fn_00e63af0_reports_an_empty_interval_as_4() {
        assert_eq!(clip_00e63af0(2.0, 3.0, 1.0).0, 4);
        assert_eq!(clip_00e63af0(2.0, 1.0, 1.0).0, 4);
        // A NaN distance takes the second branch and never compares as empty.
        assert_eq!(clip_00e63af0(f32::NAN, 0.0, 5.0).0, 2);
    }
}
