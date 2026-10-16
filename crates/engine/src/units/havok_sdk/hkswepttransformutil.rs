//! `Havok SDK/hkswepttransformutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cf1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::setTimeInformation` (map name). cdecl, three
/// words: stores the two `float`s at `+0x4c` and `+0x5c` of `target`. Each
/// goes through an `FLD`/`FSTP` pair, which quiets a signalling NaN.
pub fn hk_swept_transform_util_set_time_information(
    e: &mut Engine,
    first: f32,
    second: f32,
    target: Ptr,
) {
    e.mem.set_f32(target.addr() + 0x4c, x87_quiet(first));
    e.mem.set_f32(target.addr() + 0x5c, x87_quiet(second));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cf1480,
        hk_swept_transform_util_set_time_information(f32, f32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_swept_transform_util_set_time_information_stores_both_floats_quieted() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x80);
        e.call(
            0x00cf_1480,
            &args![1.5f32, f32::from_bits(0x7f80_0001), Ptr::<()>::new(target)],
        );
        assert_eq!(e.mem.f32(target + 0x4c), 1.5);
        assert_eq!(e.mem.u32(target + 0x5c), 0x7fc0_0001);
    }
}
