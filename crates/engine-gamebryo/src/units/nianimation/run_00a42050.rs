//! `NiAnimation/run_00a42050` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::set_literal_fixed_string;
use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

// Translated from 00a421c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiAnimation subsystem; reached by the jump
/// that ends `fn_00a2d8a0`; no arguments): sets the static fixed strings at `011f3ec0` and
/// `011f3ec4` from the literals at `010977f4` and `010977ec`, each only when the word does
/// not hold the literal's address already.
pub fn fn_00a421c0(e: &mut Engine) {
    set_literal_fixed_string(e, 0x011f_3ec0, 0x0109_77f4);
    set_literal_fixed_string(e, 0x011f_3ec4, 0x0109_77ec);
}

/// `2.0` (`double`, `01011590`) and `3.0` (`double`, `01021928`).
const TWO: u32 = 0x0101_1590;
const THREE: u32 = 0x0102_1928;

// Translated from 00a42050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three floats, result in ST0): the
/// linear blend `t * b + (1 - t) * a` of `a` and `b` by `t`, in x87 order
/// (`1 - t`, times `a`; `t * b`; added), rounded to a float.
pub fn fn_00a42050(_e: &mut Engine, t: f32, a: f32, b: f32) -> f64 {
    let t = x87_load32(t.to_bits());
    let one_minus_t = x87_sub(1.0, t);
    let from_a = x87_mul(one_minus_t, x87_m32(a.to_bits()));
    let from_b = x87_mul(t, x87_load32(b.to_bits()));
    x87_load32(x87_store32(x87_add(from_b, from_a)))
}

// Translated from 00a42070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five floats, result in ST0): the
/// cubic Hermite value at `t` of the segment from `start` (tangent
/// `start_tangent`) to `end` (tangent `end_tangent`):
/// `start + t * (start_tangent + t * (c2 + t * c3))` with
/// `c3 = (start_tangent + end_tangent) - (end - start) * 2` and
/// `c2 = (end - start) * 3 - (end_tangent + 2 * start_tangent)`, each
/// coefficient rounded to a float (the exe keeps them in the `end` argument's
/// stack word), in the exe's x87 operation order.
pub fn fn_00a42070(
    e: &mut Engine,
    t: f32,
    start: f32,
    start_tangent: f32,
    end: f32,
    end_tangent: f32,
) -> f64 {
    let two = f64::from_bits(e.global::<u64>(TWO));
    let three = f64::from_bits(e.global::<u64>(THREE));
    let end_value = x87_load32(end.to_bits());
    let start_value = x87_load32(start.to_bits());
    let difference = x87_sub(end_value, start_value);
    let start_tangent = x87_load32(start_tangent.to_bits());
    let end_tangent = x87_load32(end_tangent.to_bits());
    let tangent_sum = x87_add(start_tangent, end_tangent);
    let loaded_two = x87_load64(two.to_bits());
    let twice_difference = x87_mul(difference, loaded_two);
    let cubic = x87_store32(x87_sub(tangent_sum, twice_difference));
    let cubic = x87_load32(cubic);
    let t = x87_load32(t.to_bits());
    let cubic_t = x87_mul(cubic, t);
    let thrice_difference = x87_mul(difference, three);
    let twice_start_tangent = x87_mul(loaded_two, start_tangent);
    let tangents = x87_add(end_tangent, twice_start_tangent);
    let quadratic = x87_store32(x87_sub(thrice_difference, tangents));
    let inner = x87_mul(x87_add(cubic_t, x87_m32(quadratic)), t);
    let linear = x87_add(start_tangent, inner);
    let scaled = x87_mul(t, linear);
    x87_load32(x87_store32(x87_add(scaled, start_value)))
}

// Translated from 00a42230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; the counterpart of
/// `fn_00a421c0`): for each of the static fixed strings at `011f3ec0` and
/// `011f3ec4` that is set, stores the handle `00a5b690` gives for a null
/// string and releases the old handle (`InterlockedDecrement` of its count
/// at -8, unless it is the empty string's handle).
pub fn fn_00a42230(e: &mut Engine) {
    for slot in [0x011f_3ec0u32, 0x011f_3ec4] {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            release_fixed_string(e, old);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_21c0, fn_00a421c0()),
        entry!(0x00a42050, fn_00a42050(f32, f32, f32) -> f64),
        entry!(0x00a42070, fn_00a42070(f32, f32, f32, f32, f32) -> f64),
        entry!(0x00a42230, fn_00a42230()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_both_strings_unless_already_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(0x011f_3ec0, empty);
        e.mem.set_u32(0x011f_3ec4, 0x0109_77ec);
        e.call_log = Some(vec![]);
        e.call(0x00a4_21c0, &args![]);
        assert_eq!(e.mem.u32(0x011f_3ec0), 0x0109_77f4 + 0x1000);
        assert_eq!(e.mem.u32(0x011f_3ec4), 0x0109_77ec);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x0109_77f4]]);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn linear_blend() {
        let mut e = Engine::new();
        let r = e.call(0x00a42050, &args![0.25f32, 4.0f32, 8.0f32]).f64();
        assert_eq!(r, 5.0);
        let r = e.call(0x00a42050, &args![1.0f32, 4.0f32, 8.0f32]).f64();
        assert_eq!(r, 8.0);
    }

    #[test]
    fn hermite_value() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        let r = e
            .call(0x00a42070, &args![0.5f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32])
            .f64();
        assert_eq!(r, 1.75);
        // t = 1 lands on the end point.
        let r = e
            .call(0x00a42070, &args![1.0f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32])
            .f64();
        assert_eq!(r, 3.0);
    }

    #[test]
    fn releases_both_static_strings_when_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, _| Ret {
            eax: 0x0109_b228,
            ..Ret::default()
        });
        e.map(0x011f_3000, 0x1000);
        let counted = e.mem.alloc(0x10);
        e.mem.set_u32(counted, 5);
        e.mem.set_u32(0x011f_3ec0, counted + 8);
        e.mem.set_u32(0x011f_3ec4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a42230, &args![]);
        assert_eq!(e.mem.u32(0x011f_3ec0), 0x0109_b228);
        assert_eq!(e.mem.u32(0x011f_3ec4), 0);
        assert_eq!(e.mem.u32(counted), 4);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]]);
        // The empty string handle is not counted down.
        e.mem.set_u32(0x011f_3ec4, 0x0109_b228);
        let empty_count = e.mem.u32(0x0109_b220);
        e.call(0x00a42230, &args![]);
        assert_eq!(e.mem.u32(0x0109_b220), empty_count);
    }
}
