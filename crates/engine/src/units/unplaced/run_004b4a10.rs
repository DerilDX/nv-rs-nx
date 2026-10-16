//! `(unplaced)/run_004b4a10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b4ad0`: thiscall with two stack words (the body keeps the stack
/// 16-byte aligned itself and returns nothing).
const IMPLEMENTATION: u32 = 0x004b_4ad0;

// Translated from 004b4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall wrapper (`RET 8`): forwards `this` and both stack
/// words, in order, to `004b4ad0`.
pub fn fn_004b4ab0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(IMPLEMENTATION, &args![this, first, second]);
}

/// The function `fn_004b4f20` calls on the member (`006815c0`).
const CALLEE_006815C0: u32 = 0x0068_15c0;

// Translated from 004b4f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006815c0` with `ECX` = `this + 0x10` and passes on its pointer result.
pub fn fn_004b4f20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006815C0, &args![this.byte_add(0x10)]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b4ab0, fn_004b4ab0(Ptr, u32, u32)),
        entry!(0x004b_4f20, fn_004b4f20(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_this_and_both_words() {
        let mut e = Engine::new();
        e.register(IMPLEMENTATION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004b_4ab0, &args![Ptr::<()>::new(0x100), 7u32, 9u32]);
        let log = e.call_log.take().unwrap();
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == IMPLEMENTATION)
            .unwrap();
        assert_eq!(forwarded.1, vec![0x100, 7, 9]);
    }

    #[test]
    fn l68d_004b4f20_passes_this_plus_10_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006815C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_4f20, &args![0x4100u32]).ptr::<()>().addr(),
            0x1445
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006815C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10]);
    }
}
