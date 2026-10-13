//! `(unplaced)/run_004538e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of `EnterCriticalSection` (`KERNEL32`, `0x00fdf05c`);
/// takes the section's address.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;

// Translated from 004538e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enters the critical section at `this` (`EnterCriticalSection`). The
/// function ends with `RET 4`, so it takes one stack word it never reads.
pub fn fn_004538e0(e: &mut Engine, this: Ptr, _unused_0: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004538e0, fn_004538e0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enters_the_section_at_this() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38e0, &args![0x2000u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![0x2000]));
    }
}
