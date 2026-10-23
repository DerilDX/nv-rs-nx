//! `fallout/interface/run_0073ac90` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0073ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at `this` plus 8 (the
/// address 8 bytes into the object `this` points to; nothing is read there).
pub fn fn_0073ac90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr()).wrapping_add(8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0073_ac90, fn_0073ac90(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_pointed_to_address_plus_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234_5678);
        assert_eq!(e.call(0x0073_ac90, &[this]).u32(), 0x1234_5680);
        e.mem.set_u32(this, 0xffff_fffc);
        assert_eq!(e.call(0x0073_ac90, &[this]).u32(), 4);
    }
}
