//! `(unplaced)/run_00453850` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004538e0`: enters the critical section at `this` (`RET 4`, one unused
/// stack word); its own translation is in `run_004538e0.rs`.
const ENTER: u32 = 0x0045_38e0;

// Translated from 004538a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (`RET 4`): calls `004538e0(this, argument)`, which
/// enters the critical section at `this`.
pub fn fn_004538a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ENTER, &args![this, argument]);
}

/// `0082f1f0(this)`: leaves the critical section at `this`.
const LEAVE: u32 = 0x0082_f1f0;

// Translated from 004538c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (no stack words): calls `0082f1f0(this)`, which
/// leaves the critical section at `this`. The engine map names this address
/// `RtlLeaveCriticalSection` after the Xbox import it stands for; on PC it is
/// this ordinary function.
pub fn fn_004538c0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004538a0, fn_004538a0(Ptr, u32)),
        entry!(0x004538c0, fn_004538c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004538c0_forwards_this() {
        let mut e = Engine::new();
        e.register(LEAVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LEAVE, vec![0x1000]));
    }

    #[test]
    fn forwards_this_and_the_stack_word() {
        let mut e = Engine::new();
        e.register(ENTER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38a0, &args![0x1000u32, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER, vec![0x1000, 7]));
    }
}
