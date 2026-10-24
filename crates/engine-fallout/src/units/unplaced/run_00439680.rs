//! `(unplaced)/run_00439680` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00439680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A three-word setter (unnamed in the map; `thiscall`, `(&point)`): copies the three words
/// at `point` to `this + 0x18`.
pub fn fn_00439680(e: &mut Engine, this: Ptr, point: Ptr) {
    for word in 0..3u32 {
        let value = e.mem.u32(point.addr().wrapping_add(4 * word));
        e.mem
            .set_u32(this.addr().wrapping_add(0x18 + 4 * word), value);
    }
}

// Translated from 004396b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A nine-word setter (unnamed in the map; `thiscall`, `RET 4`): copies the nine words at
/// `source` (a 3x3 matrix by its size; `REP MOVSD`, first word first) to `this + 0x24`.
pub fn fn_004396b0(e: &mut Engine, this: Ptr, source: Ptr) {
    for word in 0..9u32 {
        let value = e.mem.u32(source.addr().wrapping_add(4 * word));
        e.mem
            .set_u32(this.addr().wrapping_add(0x24 + 4 * word), value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00439680, fn_00439680(Ptr, Ptr)),
        entry!(0x0043_96b0, fn_004396b0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00439680_copies_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        let point = e.mem.alloc(12);
        for word in 0..3u32 {
            e.mem.set_u32(point + 4 * word, 0x10 + word);
        }
        e.call(0x00439680, &args![this, point]);
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.u32(this + 0x18 + o)),
            [0x10, 0x11, 0x12]
        );
    }

    #[test]
    fn fn_004396b0_copies_nine_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let source = e.mem.alloc(0x24);
        for word in 0..9u32 {
            e.mem.set_u32(source + 4 * word, 0x20 + word);
        }
        e.call(0x0043_96b0, &args![this, source]);
        let copied: Vec<u32> = (0..9u32).map(|w| e.mem.u32(this + 0x24 + 4 * w)).collect();
        assert_eq!(copied, (0x20..0x29).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }
}
