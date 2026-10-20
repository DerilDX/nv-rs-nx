//! `(unplaced)/run_00507780` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00507780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` at +0x134 (`thiscall`, one word; no Xbox PDB name),
/// through the x87 stack (a signalling NaN is quieted).
pub fn fn_00507780(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x134, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050_7780, fn_00507780(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_at_0x134() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x140);
        e.call(0x0050_7780, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0x134), 2.5);
        e.call(0x0050_7780, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x134), 0x7fc0_0001);
    }
}
