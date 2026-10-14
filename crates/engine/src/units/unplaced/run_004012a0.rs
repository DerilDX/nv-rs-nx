//! `(unplaced)/run_004012a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// The byte global [`fn_00401500`] answers. Other translations read it as the
/// save writer's big-endian flag; this function only returns it.
const BYTE_ORDER_FLAG_011C54BA: u32 = 0x011c_54ba;

// Translated from 00401500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the byte global at `011c54ba` in `AL` (no arguments, no other
/// effect). The engine map gives this folded getter no name.
pub fn fn_00401500(e: &mut Engine) -> u8 {
    e.global(BYTE_ORDER_FLAG_011C54BA)
}

// Translated from 00401660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `m_actualChunkSize` (Xbox PDB, `+0x25C`) of a `TESFile`: the size
/// of the current chunk. The engine map gives it no name.
pub fn fn_00401660(e: &mut Engine, file: Ptr<TESFile>) -> u32 {
    e.get(file, TESFile::m_actualChunkSize)
}

// Translated from 00401680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `bMustEndianConvert` (Xbox PDB, the byte at `+0x299`) of a
/// `TESFile` in `AL`. The engine map gives it no name.
pub fn fn_00401680(e: &mut Engine, file: Ptr<TESFile>) -> bool {
    e.get(file, TESFile::bMustEndianConvert)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00401500, fn_00401500() -> u8),
        entry!(0x00401660, fn_00401660(Ptr<TESFile>) -> u32),
        entry!(0x00401680, fn_00401680(Ptr<TESFile>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte_global() {
        let mut e = Engine::new();
        e.map(BYTE_ORDER_FLAG_011C54BA, 4);
        assert_eq!(e.call(0x0040_1500, &args![]).u32() & 0xff, 0);
        e.set_global::<u8>(BYTE_ORDER_FLAG_011C54BA, 1);
        // The neighbouring byte is not part of the result.
        e.set_global::<u8>(BYTE_ORDER_FLAG_011C54BA + 1, 0xff);
        assert_eq!(e.call(0x0040_1500, &args![]).u32() & 0xff, 1);
    }

    #[test]
    fn answers_the_chunk_size_and_endian_flag() {
        let mut e = Engine::new();
        let file = e.new_object::<TESFile>();
        e.set(file, TESFile::m_actualChunkSize, 0x1234);
        assert_eq!(e.call(0x0040_1660, &args![file]).u32(), 0x1234);
        assert!(!e.call(0x0040_1680, &args![file]).bool());
        e.set(file, TESFile::bMustEndianConvert, true);
        assert!(e.call(0x0040_1680, &args![file]).bool());
    }
}
