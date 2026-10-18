//! `NiSystem/run_00aa2820` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa2820`: the jump target of `00aa29d0`.
const JUMP_TARGET_00AA29D0: u32 = 0x00aa_2820;

// Translated from 00aa29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00aa2820`, passing on `this` and its two arguments; returns the target result. Return type (bool): in the target `00aa2820` every return sets AL (`XOR AL,AL` twice, `MOV AL,1` once); the class is unnamed.
pub fn fn_00aa29d0(e: &mut Engine, this: Ptr, first: Ptr, second: u32) -> bool {
    e.call(JUMP_TARGET_00AA29D0, &args![this, first, second])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_29d0, fn_00aa29d0(Ptr, Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa29d0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00aa_29d0;
        const TARGET: u32 = 0x00aa_2820;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]).u32(),
            1
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]).u32(),
            0
        );
    }
}
