//! `Havok SDK/run_00d87bd0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d87bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty `thiscall` with one stack word (`RET 4`): does nothing. The class is unconfirmed.
pub fn fn_00d87bd0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d87be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty `thiscall` with one stack word (`RET 4`): does nothing, as `00d87bd0`. The class is
/// unconfirmed.
pub fn fn_00d87be0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_7bd0, fn_00d87bd0(Ptr, u32)),
        entry!(0x00d8_7be0, fn_00d87be0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_empty_functions_change_nothing_and_call_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00d8_7bd0, &args![this, 0x55u32]);
        e.call(0x00d8_7be0, &args![this, 0x66u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
