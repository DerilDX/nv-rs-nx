//! `fallout/interface/run_00726bf0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0(4, address)` (cdecl): the per-word step; the result is unused.
const WORD_STEP: u32 = 0x006e_6da0;

// Translated from 00726bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `006e6da0(4, address + 4 * i)` for each
/// `i` in `0..count`. `this` is not read.
pub fn fn_00726bf0(e: &mut Engine, _this: Ptr, address: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        e.call(
            WORD_STEP,
            &args![4u32, address.wrapping_add(i.wrapping_mul(4))],
        );
        i += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00726bf0, fn_00726bf0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_once_per_word() {
        let mut e = Engine::new();
        e.register(WORD_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00726bf0, &args![0u32, 0x6000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            &log[1..],
            &[
                (WORD_STEP, vec![4, 0x6000]),
                (WORD_STEP, vec![4, 0x6004]),
                (WORD_STEP, vec![4, 0x6008]),
            ]
        );
    }

    #[test]
    fn a_zero_count_steps_nothing() {
        let mut e = Engine::new();
        e.register(WORD_STEP, |_, _| panic!("no words"));
        e.call(0x00726bf0, &args![0u32, 0x6000u32, 0u32]);
    }
}
