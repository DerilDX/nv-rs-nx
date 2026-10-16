//! `fallout shared/havok/run_00622470` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00622570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: stores the byte `value` at `this` and returns `this`.
pub fn fn_00622570(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    e.mem.set_u8(this.addr(), value);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00622570, fn_00622570(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00622570_stores_the_byte_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let result = e.call(0x0062_2570, &args![Ptr::<()>::new(this), 0x5au8]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u8(this), 0x5a);
        assert_eq!(e.mem.u8(this + 1), 0);
    }
}
