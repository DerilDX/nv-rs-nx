//! `fallout shared/pathfinding/run_0069e090` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00416870`: a point's constructor from three floats (`thiscall`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;

// Translated from 0069e0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the 2D point at `this` scaled by `scale` as a 3D point in
/// `result` (`z` 0), each product rounded to a `float`. Returns `result`.
pub fn fn_0069e0c0(e: &mut Engine, this: Ptr, result: Ptr, scale: f32) -> Ptr {
    let base = this.addr();
    let y = x87_store32(x87_mul(
        x87_load32(scale.to_bits()),
        x87_m32(e.mem.u32(base + 4)),
    ));
    let x = x87_store32(x87_mul(
        x87_load32(scale.to_bits()),
        x87_m32(e.mem.u32(base)),
    ));
    e.call(POINT_CONSTRUCT, &args![result, x, y, 0u32]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0069_e0c0, fn_0069e0c0(Ptr, Ptr, f32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_into_a_3d_point() {
        let mut e = Engine::new();
        e.register(POINT_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_f32(this, 1.5);
        e.mem.set_f32(this + 4, -2.0);
        let result = e.mem.alloc(12);
        e.mem.set_f32(result + 8, 9.0);
        assert_eq!(
            e.call(0x0069_e0c0, &args![this, result, 2.0f32]).u32(),
            result
        );
        assert_eq!(e.mem.f32(result), 3.0);
        assert_eq!(e.mem.f32(result + 4), -4.0);
        assert_eq!(e.mem.f32(result + 8), 0.0);
    }
}
