//! `fallout/audio/run_0082f140` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `LeaveCriticalSection` (import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 0082f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: leaves the critical section at `this`
/// (`LeaveCriticalSection(this)`).
pub fn fn_0082f1f0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE_CRITICAL_SECTION, &args![this]);
}

/// The function `fn_0082f190` calls on the member (`0082f1b0`).
const CALLEE_0082F1B0: u32 = 0x0082_f1b0;

/// The function `fn_0082f1d0` calls on the member (`0082f1f0`).
const CALLEE_0082F1F0: u32 = 0x0082_f1f0;

// Translated from 0082f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1b0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f190(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1B0, &args![this.byte_add(0x10c)]);
}

// Translated from 0082f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1f0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f1d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1F0, &args![this.byte_add(0x10c)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082f1f0, fn_0082f1f0(Ptr)),
        entry!(0x0082_f190, fn_0082f190(Ptr)),
        entry!(0x0082_f1d0, fn_0082f1d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_the_section_at_this() {
        let mut e = Engine::new();
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082f1f0, &args![0x1234u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (LEAVE_CRITICAL_SECTION, vec![0x1234])
        );
    }

    #[test]
    fn l68d_0082f190_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f190, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10c]);
    }

    #[test]
    fn l68d_0082f1d0_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f1d0, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x10c]);
    }
}
