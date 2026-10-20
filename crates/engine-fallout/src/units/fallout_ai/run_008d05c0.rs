//! `fallout/ai/run_008d05c0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d05e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty method (no name in the engine map; it sits in a vtable):
/// `thiscall` with three stack words (`RET 0xc`), none of them read.
pub fn fn_008d05e0(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_05e0, fn_008d05e0(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008d05e0_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x008d_05e0, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(r, Ret::default());
    }
}
