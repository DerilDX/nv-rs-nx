//! `Havok SDK/run_00ce0c40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A C runtime function that takes its argument in `ST0` and returns its result in `ST0`
/// (`00ec9650`; unnamed; from the clamp in `00ce0c40` it is the arc sine). In the uniform
/// form its argument is a leading `f64` (two words).
const ARC_FUNCTION: u32 = 0x00ec_9650;
/// The float `1.0` (`01012068`).
const ONE: u32 = 0x0101_2068;
/// The two floats returned when the magnitude is at least 1: `0101ff38` for a positive and
/// `01016b78` for any other argument (the values are 1.5707964 and -1.5707964).
const POSITIVE_RESULT: u32 = 0x0101_ff38;
const OTHER_RESULT: u32 = 0x0101_6b78;

// Translated from 00ce0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(x)`, result in `ST0`: the magnitude `|x|` is rounded to a float (`FABS`, `FSTP`,
/// `FLD`) and compared with `1.0`; below it (or unordered, so also for a NaN) the C runtime
/// function `00ec9650` is called with `x` (as loaded on the x87 stack) and its result is
/// rounded to a float; otherwise the result is the float at `0101ff38` when `0 < x` and the one
/// at `01016b78` when not, each copied through the x87 stack.
pub fn fn_00ce0c40(e: &mut Engine, x: f32) -> f32 {
    let loaded = x87_load32(x.to_bits());
    let magnitude = x87_load32(x87_store32(loaded.abs()));
    let one: f32 = e.global(ONE);
    // FCOMP then TEST AH,1: taken when the magnitude is below 1.0 or unordered.
    if !(magnitude >= f64::from(one)) {
        let result = e.call(ARC_FUNCTION, &args![loaded]).f64();
        return f32::from_bits(x87_store32(result));
    }
    let bits = if 0.0 < loaded {
        e.mem.u32(POSITIVE_RESULT)
    } else {
        e.mem.u32(OTHER_RESULT)
    };
    f32::from_bits(x87_quiet_bits(bits))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ce_0c40, fn_00ce0c40(f32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn rig() -> (Engine, Rc<RefCell<Vec<f64>>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ONE, 1.0);
        e.mem.set_u32(POSITIVE_RESULT, 0x3fc9_0fdb);
        e.mem.set_u32(OTHER_RESULT, 0xbfc9_0fdb);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(ARC_FUNCTION, move |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            sink.borrow_mut().push(x);
            Ret {
                st0: x.asin(),
                ..Ret::default()
            }
        });
        (e, seen)
    }

    /// Below 1 the library function runs on the argument and its float-rounded result comes
    /// back; at 1 and above the constants answer by sign (`0 < x`).
    #[test]
    fn small_magnitudes_call_the_library_and_large_ones_clamp() {
        let (mut e, seen) = rig();
        let got = e.call(0x00ce_0c40, &args![0.5f32]).f32();
        assert_eq!(got, 0.5f64.asin() as f32);
        assert_eq!(*seen.borrow(), vec![0.5]);
        let got = e.call(0x00ce_0c40, &args![-0.25f32]).f32();
        assert_eq!(got, (-0.25f64).asin() as f32);
        for (x, expected) in [
            (1.0f32, 0x3fc9_0fdbu32),
            (7.5, 0x3fc9_0fdb),
            (-1.0, 0xbfc9_0fdb),
            (f32::NEG_INFINITY, 0xbfc9_0fdb),
            (f32::INFINITY, 0x3fc9_0fdb),
        ] {
            let got = e.call(0x00ce_0c40, &args![x]).f32();
            assert_eq!(got.to_bits(), expected, "x = {x}");
        }
        assert_eq!(seen.borrow().len(), 2);
    }

    /// A NaN is unordered against 1.0, so it takes the library path (and its signalling
    /// form is quieted by the load).
    #[test]
    fn a_nan_goes_to_the_library() {
        let (mut e, seen) = rig();
        e.call(0x00ce_0c40, &args![f32::from_bits(0x7f80_0001)]);
        assert_eq!(seen.borrow().len(), 1);
        assert!(seen.borrow()[0].is_nan());
    }
}
