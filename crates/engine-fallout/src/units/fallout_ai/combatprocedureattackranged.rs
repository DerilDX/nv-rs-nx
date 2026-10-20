//! `fallout/ai/combat/combatprocedureattackranged.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d1b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float getter (unnamed in the map; `thiscall`, no arguments): the float at
/// `this + 0x54`, in ST0.
pub fn fn_009d1b50(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x54))))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009d1b50, fn_009d1b50(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009d1b50_reads_the_float_at_0x54() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        e.mem.set_f32(this + 0x54, 3.25);
        assert_eq!(e.call(0x009d1b50, &args![this]).f64(), 3.25);
    }
}
