//! `Havok SDK/run_00d284f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d284f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (input in a register, then one stack word; cdecl `RET`, result in `ST0`; the
/// register is `EAX`, which the exe's callers set, here the first argument `base`): with `a` the
/// first three lanes of the vector at `base + 0xd0` and `b` those at `base + 0xe0`, answers
/// `float(x * x * (bz*bz + (by*by + bx*bx)) + (az*az + (ay*ay + ax*ax)))` where `x` is the stack
/// float (`x * x` and the product in x87 double precision, each lane sum in `SSE` single
/// precision the way the `SHUFPS`/`ADDSS` pairs order it).
pub fn fn_00d284f0(e: &mut Engine, base: Ptr, x: f32) -> f32 {
    let square_sum = |e: &Engine, at: u32| {
        let squares = [0u32, 1, 2].map(|lane| {
            let value = e.mem.f32(at + lane * 4);
            value * value
        });
        squares[2] + (squares[1] + squares[0])
    };
    let loaded = x87_load32(x.to_bits());
    let squared = x87_mul(loaded, loaded);
    let first = square_sum(e, base.addr() + 0xd0);
    let second = square_sum(e, base.addr() + 0xe0);
    let product = x87_mul(squared, x87_m32(second.to_bits()));
    let total = x87_add(product, x87_m32(first.to_bits()));
    f32::from_bits(x87_quiet_bits(x87_store32(total)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_84f0, fn_00d284f0(Ptr, f32) -> f32)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d284f0_adds_the_scaled_square_lengths() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x100);
        for (lane, value) in [1.0f32, 2.0, 2.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(base + 0xd0 + lane as u32 * 4, value);
        }
        for (lane, value) in [3.0f32, 4.0, 0.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(base + 0xe0 + lane as u32 * 4, value);
        }
        // 2 * 2 * 25 + 9
        assert_eq!(e.call(0x00d2_84f0, &args![base, 2.0f32]).f32(), 109.0);
        // x = 0 leaves the first length alone.
        assert_eq!(e.call(0x00d2_84f0, &args![base, 0.0f32]).f32(), 9.0);
    }

    #[test]
    fn fn_00d284f0_keeps_a_nan_from_the_stack_float() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x100);
        assert!(e.call(0x00d2_84f0, &args![base, f32::NAN]).f32().is_nan());
    }
}
