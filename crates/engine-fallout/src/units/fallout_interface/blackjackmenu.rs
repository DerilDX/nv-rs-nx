//! `fallout/interface/menus/blackjackmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00733fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, one `float`): stores the float at +0xf8 (`FLD`/`FSTP`, so a
/// signalling NaN is stored quieted).
pub fn fn_00733fd0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xf8),
        x87_quiet_bits(value.to_bits()),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0073_3fd0, fn_00733fd0(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.call(0x0073_3fd0, &args![this, 1.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xf8), 1.5);
        e.call(0x0073_3fd0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xf8), 0x7fc0_0001);
    }
}
