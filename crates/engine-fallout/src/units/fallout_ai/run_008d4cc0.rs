//! `fallout/ai/run_008d4cc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` constant -1.0 the code loads from the exe's data.
const MINUS_ONE: u32 = 0x0101_2054;

// Translated from 008d4e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two armour values of a `Character` (`fArmorRating` at +0x1b8 and
/// `fArmorThreshold` at +0x1bc, Xbox PDB names, same offsets on PC) to -1.0,
/// the constant at `01012054`. The engine map has no name for it.
pub fn fn_008d4e80(e: &mut Engine, this: Ptr) {
    let minus_one: f32 = e.global(MINUS_ONE);
    e.mem.set_f32(this.addr() + 0x1b8, minus_one);
    e.mem.set_f32(this.addr() + 0x1bc, minus_one);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d4e80, fn_008d4e80(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_armour_values_get_the_constant() {
        let mut e = Engine::new();
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        let object = e.mem.alloc(0x1d0);
        e.mem.set_f32(object + 0x1b8, 5.0);
        e.mem.set_f32(object + 0x1c0, 6.0);
        e.call(0x008d_4e80, &args![object]);
        assert_eq!(e.mem.f32(object + 0x1b8), -1.0);
        assert_eq!(e.mem.f32(object + 0x1bc), -1.0);
        // Neighbours are untouched.
        assert_eq!(e.mem.f32(object + 0x1c0), 6.0);
    }
}
