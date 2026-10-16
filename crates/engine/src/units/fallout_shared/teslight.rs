//! `fallout shared/teslight.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The bit of the flags dword at `+8` the function tests.
const FLAG_BIT: u32 = 0x10;

// Translated from 0050d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `teslight.cpp`). `__fastcall`: whether
/// bit `0x10` of the dword at `this + 8` is set.
pub fn fn_0050d4a0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & FLAG_BIT != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050d4a0, fn_0050d4a0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_0x10_of_the_dword_at_8() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(16));
        e.mem.set_u32(this.addr() + 8, 0xffff_ffef);
        assert!(!e.call(0x0050_d4a0, &args![this]).bool());
        e.mem.set_u32(this.addr() + 8, 0x10);
        assert!(e.call(0x0050_d4a0, &args![this]).bool());
    }
}
