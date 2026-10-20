//! `NiXenonRenderer/run_00e7acb0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The four-character codes `00e7acb0` answers 0 for above `"MET1"` (their bytes read as ASCII:
/// `"DXT1"` to `"DXT5"`, `"UYVY"`, `"YUY2"`, `"RGBG"`, `"GRGB"`).
const CODES_ANSWERING_0: [u32; 9] = [
    0x3154_5844,
    0x3254_5844,
    0x3354_5844,
    0x3454_5844,
    0x3554_5844,
    0x5956_5955,
    0x3259_5559,
    0x4247_5247,
    0x4742_4752,
];
/// `"MET1"`, the highest code of the compare chain's lower half.
const CODE_MET1: i32 = 0x3154_454d;

// Translated from 00e7acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7acb0`), `cdecl`, one word (a format code; the renderer
/// passes Direct3D format numbers): the bit size it answers. Codes above `"MET1"` (signed) give
/// 0 for the nine four-character codes of [`CODES_ANSWERING_0`] and -1 otherwise; `"MET1"`
/// gives 0; below it, codes above 0x75 (unsigned, so negative ones too) give -1, and 0 to 0x75
/// go through the switch's byte map at `00e7ad64` and jump table at `00e7ad44` (read in
/// Ghidra): 0x14 gives 0x18 (24); 0x15, 0x16, 0x1f to 0x21, 0x23, 0x3e to 0x40, 0x43, 0x47,
/// 0x4b, 0x4d, 0x4f, 0x52, 0x53, 0x66, 0x70 and 0x72 give 0x20 (32); 0x17 to 0x1a, 0x1d, 0x1e,
/// 0x22, 0x28, 0x33, 0x3c, 0x3d, 0x46, 0x49, 0x50, 0x51, 0x65, 0x6f and 0x75 give 0x10 (16);
/// 0x1b, 0x1c, 0x29, 0x32 and 0x34 give 8; 0x24, 0x6e, 0x71 and 0x73 give 0x40 (64); 0x74
/// gives 0x80 (128); 0 and 0x64 give 0; the rest -1.
pub fn fn_00e7acb0(_e: &mut Engine, format: i32) -> i32 {
    if format > CODE_MET1 {
        return if CODES_ANSWERING_0.contains(&(format as u32)) {
            0
        } else {
            -1
        };
    }
    if format == CODE_MET1 {
        return 0;
    }
    match format as u32 {
        0 | 0x64 => 0,
        0x14 => 0x18,
        0x15
        | 0x16
        | 0x1f..=0x21
        | 0x23
        | 0x3e..=0x40
        | 0x43
        | 0x47
        | 0x4b
        | 0x4d
        | 0x4f
        | 0x52
        | 0x53
        | 0x66
        | 0x70
        | 0x72 => 0x20,
        0x17..=0x1a
        | 0x1d
        | 0x1e
        | 0x22
        | 0x28
        | 0x33
        | 0x3c
        | 0x3d
        | 0x46
        | 0x49
        | 0x50
        | 0x51
        | 0x65
        | 0x6f
        | 0x75 => 0x10,
        0x1b | 0x1c | 0x29 | 0x32 | 0x34 => 8,
        0x24 | 0x6e | 0x71 | 0x73 => 0x40,
        0x74 => 0x80,
        _ => -1,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_acb0, fn_00e7acb0(i32) -> i32)]
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    fn size(e: &mut Engine, format: u32) -> i32 {
        e.call(0x00e7_acb0, &args![format]).i32()
    }

    #[test]
    fn fn_00e7acb0_answers_the_bit_sizes() {
        let mut e = Engine::new();
        let cases: [(u32, i32); 16] = [
            (0, 0),
            (0x64, 0),
            (0x14, 24),
            (0x15, 32),
            (0x72, 32),
            (0x17, 16),
            (0x75, 16),
            (0x1b, 8),
            (0x34, 8),
            (0x24, 64),
            (0x73, 64),
            (0x74, 128),
            (0x01, -1),
            (0x76, -1),
            (0x8000_0000, -1),
            (0x2a, -1),
        ];
        for (format, expected) in cases {
            assert_eq!(size(&mut e, format), expected, "format {format:#x}");
        }
    }

    #[test]
    fn fn_00e7acb0_four_character_codes() {
        let mut e = Engine::new();
        for code in CODES_ANSWERING_0 {
            assert_eq!(size(&mut e, code), 0, "code {code:#x}");
        }
        assert_eq!(size(&mut e, 0x3154_454d), 0);
        assert_eq!(size(&mut e, 0x3154_454e), -1);
        assert_eq!(size(&mut e, 0x3154_454c), -1);
        assert_eq!(size(&mut e, 0x7fff_ffff), -1);
    }
}
