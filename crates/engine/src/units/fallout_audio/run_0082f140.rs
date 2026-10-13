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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082f1f0, fn_0082f1f0(Ptr))]
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
}
