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

/// `005ae380` (thiscall on a list; the map names it `VATS::GetCount`): counts the
/// list's non-null items.
const LIST_COUNT: u32 = 0x005a_e380;

// Translated from 00726c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the item count (`005ae380`) of the list at `this + 4`; the callee's `EAX`
/// is left as the result.
pub fn fn_00726c40(e: &mut Engine, this: Ptr) -> u32 {
    e.call(LIST_COUNT, &args![this.addr() + 4]).u32()
}

// Translated from 00726c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): copies the word at `source` into `this` unless it is 0.
pub fn fn_00726c60(e: &mut Engine, this: Ptr, source: u32) {
    let value = e.mem.u32(source);
    if value != 0 {
        e.mem.set_u32(this.addr(), value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00726bf0, fn_00726bf0(Ptr, u32, u32)),
        entry!(0x0072_6c40, fn_00726c40(Ptr) -> u32),
        entry!(0x0072_6c60, fn_00726c60(Ptr, u32)),
    ]
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

    #[test]
    fn fn_00726c40_counts_the_list_at_four() {
        let mut e = Engine::new();
        e.register(LIST_COUNT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0072_6c40, &args![0x6000u32]).u32(), 0x6005);
    }

    #[test]
    fn fn_00726c60_copies_a_non_zero_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        e.mem.set_u32(this, 7);
        e.call(0x0072_6c60, &args![this, source]);
        assert_eq!(e.mem.u32(this), 7);
        e.mem.set_u32(source, 9);
        e.call(0x0072_6c60, &args![this, source]);
        assert_eq!(e.mem.u32(this), 9);
    }
}
