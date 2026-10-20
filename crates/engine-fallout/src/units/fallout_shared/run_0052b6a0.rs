//! `fallout shared/run_0052b6a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// What `0052b780` stores where: an offset of the image space and either a float global the
/// value is copied from (`FLD dword [global]`) or a constant (`FLD1`, `FLDZ`).
enum Source {
    Global(u32),
    One,
    Zero,
}

/// The stores of `0052b780`, in the exe's order.
const DEFAULTS_0052B780: [(u32, Source); 32] = [
    (0x18, Source::Global(0x0101_6248)),
    (0x1c, Source::Global(0x0101_8f98)),
    (0x24, Source::One),
    (0x28, Source::One),
    (0x2c, Source::One),
    (0x30, Source::Global(0x0102_2154)),
    (0x34, Source::Global(0x0102_212c)),
    (0x38, Source::One),
    (0x3c, Source::Zero),
    (0x40, Source::Zero),
    (0x44, Source::Global(0x0101_6088)),
    (0x48, Source::Global(0x0101_6088)),
    (0x4c, Source::One),
    (0x20, Source::One),
    (0x54, Source::Global(0x0102_1c9c)),
    (0x5c, Source::Global(0x0101_df44)),
    (0x58, Source::Global(0x0101_8238)),
    (0x60, Source::Global(0x0101_6248)),
    (0x64, Source::Global(0x0101_6248)),
    (0x68, Source::Global(0x0102_d934)),
    (0x78, Source::Global(0x0101_84f4)),
    (0x6c, Source::Global(0x0102_d930)),
    (0x70, Source::Global(0x0101_6248)),
    (0x74, Source::Global(0x0102_d92c)),
    (0x88, Source::One),
    (0x80, Source::Global(0x0101_6248)),
    (0x84, Source::One),
    (0x7c, Source::One),
    (0x98, Source::Zero),
    (0x8c, Source::One),
    (0x90, Source::One),
    (0x94, Source::One),
];

// Translated from 0052b780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`, no stack words): the default image space values that
/// `GenerateDefaultObjects` (`0046d6c2`, `0046d777`) sets on the `DefaultImageSpaceInterior`
/// and `DefaultImageSpaceExterior` forms. Each float of the data from +0x18 to +0x98 (except
/// +0x50) gets 1.0, 0.0 or a copy of a float global (the copies quiet a signalling NaN),
/// stored in the exe's order.
pub fn fn_0052b780(e: &mut Engine, this: Ptr) {
    for (offset, source) in DEFAULTS_0052B780.iter() {
        let bits = match source {
            Source::Global(global) => x87_quiet_bits(e.global::<u32>(*global)),
            Source::One => x87_store32(1.0),
            Source::Zero => x87_store32(0.0),
        };
        e.mem.set_u32(this.addr() + offset, bits);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_b780, fn_0052b780(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0052b780_sets_the_defaults() {
        let mut e = Engine::new();
        for page in [
            0x0101_6000u32,
            0x0101_8000,
            0x0101_d000,
            0x0102_1000,
            0x0102_2000,
            0x0102_d000,
        ] {
            e.map(page, 0x1000);
        }
        for (global, bits) in [
            (0x0101_6248u32, 0x3f00_0000u32),
            (0x0101_8f98, 0x40e0_0000),
            (0x0102_2154, 0x4010_0000),
            (0x0102_212c, 0x3e66_6666),
            (0x0101_6088, 0x3fc0_0000),
            (0x0102_1c9c, 0x3cf5_c28f),
            (0x0101_df44, 0x3e4c_cccd),
            (0x0101_8238, 0x3f33_3333),
            (0x0102_d934, 0x3ca3_d70a),
            (0x0101_84f4, 0x4020_0000),
            (0x0102_d930, 0x3e57_0a3d),
            // A signalling NaN comes out quiet.
            (0x0102_d92c, 0x7f80_0001),
        ] {
            e.mem.set_u32(global, bits);
        }
        let this = e.mem.alloc(0xb0);
        e.mem.write(this, &[0xaa; 0xb0]);
        e.call(0x0052_b780, &args![this]);
        let word = |offset: u32| e.mem.u32(this + offset);
        assert_eq!(word(0x18), 0x3f00_0000);
        assert_eq!(word(0x1c), 0x40e0_0000);
        assert_eq!(word(0x20), 0x3f80_0000);
        assert_eq!(word(0x3c), 0);
        assert_eq!(word(0x48), 0x3fc0_0000);
        assert_eq!(word(0x54), 0x3cf5_c28f);
        assert_eq!(word(0x58), 0x3f33_3333);
        assert_eq!(word(0x5c), 0x3e4c_cccd);
        assert_eq!(word(0x68), 0x3ca3_d70a);
        assert_eq!(word(0x6c), 0x3e57_0a3d);
        assert_eq!(word(0x74), 0x7fc0_0001);
        assert_eq!(word(0x78), 0x4020_0000);
        assert_eq!(word(0x80), 0x3f00_0000);
        assert_eq!(word(0x94), 0x3f80_0000);
        assert_eq!(word(0x98), 0);
        assert_eq!(word(0x9c), 0xaaaa_aaaa);
        let untouched: Vec<u32> = (0x18..0x9c)
            .step_by(4)
            .filter(|offset| word(*offset) == 0xaaaa_aaaa)
            .collect();
        assert_eq!(untouched, vec![0x50]);
    }
}
