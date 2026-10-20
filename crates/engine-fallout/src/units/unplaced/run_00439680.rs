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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00439680, fn_00439680(Ptr, Ptr))]
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
}
