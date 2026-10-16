//! `NiAnimation/run_00a40980` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a36f80`: the jump target of `00a40a00`.
const JUMP_TARGET_00A40A00: u32 = 0x00a3_6f80;

// Translated from 00a40a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a36f80` (engine map: `NiBlendInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): the final target `00a36f80` is `NiBlendInterpolator::RegisterStreamables` (engine map), `bool` in the Xbox PDB, and its disassembly ends with AL set to 1.
pub fn fn_00a40a00(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A40A00, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_0a00, fn_00a40a00(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a40a00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_0a00;
        const TARGET: u32 = 0x00a3_6f80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
