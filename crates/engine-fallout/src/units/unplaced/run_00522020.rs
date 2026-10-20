//! `(unplaced)/run_00522020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00522440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x244 of `this` when `second` is non-zero, else the one at
/// +0x220 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522440(e: &mut Engine, this: Ptr, second: u8) -> u32 {
    let offset = if second != 0 { 0x244 } else { 0x220 };
    e.mem.u32(this.addr() + offset)
}

// Translated from 00522470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x248 of `this` when `second` is non-zero, else the one at
/// +0x224 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522470(e: &mut Engine, this: Ptr, second: u8) -> u32 {
    let offset = if second != 0 { 0x248 } else { 0x224 };
    e.mem.u32(this.addr() + offset)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_2440, fn_00522440(Ptr, u8) -> u32),
        entry!(0x0052_2470, fn_00522470(Ptr, u8) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters_pick_their_word_by_the_byte_argument() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x250);
        for offset in [0x220u32, 0x224, 0x244, 0x248] {
            e.mem.set_u32(this + offset, offset);
        }
        assert_eq!(e.call(0x0052_2440, &args![this, 0u32]).u32(), 0x220);
        assert_eq!(e.call(0x0052_2440, &args![this, 1u32]).u32(), 0x244);
        // Only the low byte of the argument word counts.
        assert_eq!(e.call(0x0052_2440, &args![this, 0x100u32]).u32(), 0x220);
        assert_eq!(e.call(0x0052_2470, &args![this, 0u32]).u32(), 0x224);
        assert_eq!(e.call(0x0052_2470, &args![this, 0x80u32]).u32(), 0x248);
    }
}
