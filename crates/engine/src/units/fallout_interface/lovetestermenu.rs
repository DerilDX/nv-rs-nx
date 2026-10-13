//! `fallout/interface/menus/lovetestermenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00792760 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0x40]`, the value in `ST0`.
/// `Script::Run` (`005ac1e0`) calls it on the owner quest for the quest's
/// script delay. The `FLD` quiets a signaling NaN.
pub fn fn_00792760(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0x40) as f64
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00792760, fn_00792760(Ptr) -> f64)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0x40_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x44);
        e.mem.set_f32(object + 0x3c, 9.0);
        for value in [0.0f32, -2.5, 5.0e-39, f32::INFINITY] {
            e.mem.set_f32(object + 0x40, value);
            assert_eq!(e.call(0x00792760, &[object]).f64(), value as f64);
        }
    }
}
