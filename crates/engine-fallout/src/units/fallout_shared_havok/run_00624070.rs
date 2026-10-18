//! `fallout shared/havok/run_00624070` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0`: thiscall on the same object.
const INITIALISE: u32 = 0x0068_15c0;

// Translated from 006240d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: runs `006815c0` on `this` and returns `this`.
pub fn fn_006240d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALISE, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006240d0, fn_006240d0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_006815c0_on_this_and_returns_this() {
        let mut e = Engine::new();
        e.register(INITIALISE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x4000);
        assert_eq!(e.call(0x0062_40d0, &args![this]).ptr::<()>(), this);
        let log = e.call_log.take().unwrap();
        let inner = log.iter().find(|(addr, _)| *addr == INITIALISE).unwrap();
        assert_eq!(inner.1, vec![0x4000]);
    }
}
