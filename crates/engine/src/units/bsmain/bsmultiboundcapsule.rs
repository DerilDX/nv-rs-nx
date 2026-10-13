//! `BSMain/bsmultiboundcapsule.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00526ac0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0x24]`, the value in `ST0`. The
/// map names it `BSMultiBoundCapsule::QMultiBoundRadius`; the script
/// interpreter calls the same body for a `TESGlobal`'s value (operand `G`,
/// `005ac7a0`). The `FLD` quiets a signaling NaN.
pub fn fn_00526ac0(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0x24) as f64
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00526ac0, fn_00526ac0(Ptr) -> f64)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0x24_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x28);
        e.mem.set_f32(object + 0x20, 9.0);
        for value in [1.5f32, -1.0, 0.0, f32::NEG_INFINITY] {
            e.mem.set_f32(object + 0x24, value);
            assert_eq!(e.call(0x00526ac0, &[object]).f64(), value as f64);
        }
        // A signaling single NaN comes back quiet, its payload widened.
        e.mem.set_u32(object + 0x24, 0x7f80_0001);
        assert_eq!(
            e.call(0x00526ac0, &[object]).f64().to_bits(),
            0x7ff8_0000_2000_0000
        );
    }
}
