//! `NiXenonRenderer/run_00aa1b60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function pointer `011ac224` and its default, `00aa1b60`.
const HANDLER: u32 = 0x011a_c224;
const DEFAULT_HANDLER: u32 = 0x00aa_1b60;

// Translated from 00aa1b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa1b60`): installs `handler` in the function pointer
/// `011ac224` (cdecl, one word); a null handler installs the default `00aa1b60` (which the
/// exe stores first either way).
pub fn fn_00aa1b90(e: &mut Engine, handler: u32) {
    e.mem.set_u32(HANDLER, DEFAULT_HANDLER);
    if handler != 0 {
        e.mem.set_u32(HANDLER, handler);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_1b90, fn_00aa1b90(u32))]
}

#[cfg(test)]
mod b049_tests {
    use super::*;

    #[test]
    fn fn_00aa1b90_installs_the_handler_or_the_default() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.call(0x00aa_1b90, &args![0x1234u32]);
        assert_eq!(e.mem.u32(HANDLER), 0x1234);
        e.call(0x00aa_1b90, &args![0u32]);
        assert_eq!(e.mem.u32(HANDLER), DEFAULT_HANDLER);
    }
}
