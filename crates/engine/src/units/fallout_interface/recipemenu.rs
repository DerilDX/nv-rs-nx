//! `fallout/interface/menus/recipemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0072ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks `count` elements from `address` and does
/// nothing to them (the loop body is empty in the exe). `this` and
/// `address` are not read.
pub fn fn_0072ba80(_e: &mut Engine, _this: Ptr, _address: u32, _count: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0072ba80, fn_0072ba80(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touches_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let before = e.mem.alloc(8);
        e.mem.set_u32(before, 0x55);
        e.call(0x0072ba80, &args![0u32, before, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "no other call");
        assert_eq!(e.mem.u32(before), 0x55);
    }
}
