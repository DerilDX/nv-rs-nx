//! `(unplaced)/run_004424c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004424e0` (the engine map's `WaitForSingleObject`, `XAPILIB`):
/// thiscall, no stack words.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x0044_24e0;

// Translated from 004424c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the object embedded at `this + 0xC`: `004424e0` with that
/// address as `this`. Returns nothing.
pub fn fn_004424c0(e: &mut Engine, this: Ptr) {
    e.call(
        WAIT_FOR_SINGLE_OBJECT,
        &args![this.addr().wrapping_add(0xc)],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_24c0, fn_004424c0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waits_on_the_object_at_0xc() {
        let mut e = Engine::new();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_24c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let waits: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == WAIT_FOR_SINGLE_OBJECT)
            .collect();
        assert_eq!(waits.len(), 1);
        assert_eq!(waits[0].1, vec![0x100c]);
    }
}
