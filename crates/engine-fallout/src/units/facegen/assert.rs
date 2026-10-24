//! `FaceGen/assert.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005b5e40` (unnamed; `cdecl` with a format and its arguments): where the assert's
/// message goes, and the format `"DEFAULT: FR2 ASSERT violation in %s line %i. Code may
/// crash."` (`010a2c9c`).
const PRINT_005B5E40: u32 = 0x005b_5e40;
const ASSERT_FORMAT: u32 = 0x010a_2c9c;

// Translated from 00aafb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fr2Assert` (engine map; `cdecl`, two words: the source file name and the line):
/// `005b5e40` with the format at `010a2c9c`, `file` and `line`.
pub fn fr2_assert(e: &mut Engine, file: u32, line: u32) {
    e.call(PRINT_005B5E40, &args![ASSERT_FORMAT, file, line]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_fb90, fr2_assert(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_prints_the_file_and_line() {
        let mut e = Engine::new();
        e.register(PRINT_005B5E40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_fb90, &args![0x0106_5830u32, 0x113u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(PRINT_005B5E40, vec![ASSERT_FORMAT, 0x0106_5830, 0x113])]
        );
    }
}
