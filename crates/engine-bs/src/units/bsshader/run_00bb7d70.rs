//! `BSShader/run_00bb7d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` the depth is multiplied by (`010b9e30`, 0.83 in the exe).
const DEPTH_FACTOR: u32 = 0x010b_9e30;
/// The `double` the result is scaled by (`0103a480`, 163840.0 in the exe).
const DISTANCE_SCALE: u32 = 0x0103_a480;
/// The `float` copied to `+0x30` (`010317c4`, 163840.0 in the exe).
const END_DISTANCE: u32 = 0x0103_17c4;

// Translated from 00bb8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader`). `thiscall`, one `float` (`RET 4`): stores
/// `(1 - value * [010b9e30]) * [0103a480]` (x87, the two `double`s) as the `float` at `+0x2c`
/// and copies the `float` at `010317c4` to `+0x30` (`FLD`/`FSTP`: a signalling NaN comes out
/// quiet). On a `BSFogProperty` these are `fStartDistance` and `fEndDistance` (Xbox PDB);
/// `BSFogProperty`'s constructor inlines the same code on its depth.
pub fn fn_00bb8140(e: &mut Engine, this: Ptr, value: f32) {
    let factor: f64 = e.global(DEPTH_FACTOR);
    let scale: f64 = e.global(DISTANCE_SCALE);
    let product = x87_mul(x87_load32(value.to_bits()), factor);
    let start = x87_mul(x87_sub(1.0, product), scale);
    e.mem
        .set_u32(this.addr().wrapping_add(0x2c), x87_store32(start));
    let end = x87_quiet_bits(e.global::<u32>(END_DISTANCE));
    e.mem.set_u32(this.addr().wrapping_add(0x30), end);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bb_8140, fn_00bb8140(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [0x0103_1000, 0x0103_a000, 0x010b_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEPTH_FACTOR, 0.83f64);
        e.set_global(DISTANCE_SCALE, 163_840.0f64);
        e.set_global(END_DISTANCE, 163_840.0f32);
        e
    }

    #[test]
    fn fn_00bb8140_sets_the_two_distances() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0.5f32]);
        let expected = ((1.0 - 0.5 * 0.83f64) * 163_840.0) as f32;
        assert_eq!(e.mem.f32(this + 0x2c), expected);
        assert_eq!(e.mem.f32(this + 0x30), 163_840.0);
    }

    #[test]
    fn fn_00bb8140_quiets_nans() {
        let mut e = rig();
        e.set_global(END_DISTANCE, 0x7f80_0001u32);
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0x7f80_0002u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7fc0_0002);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
    }
}
