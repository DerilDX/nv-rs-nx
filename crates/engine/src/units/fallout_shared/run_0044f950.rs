//! `fallout shared/run_0044f950` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InitializeCriticalSection` (import slot).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;

// Translated from 0044fae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A critical section's constructor (`thiscall`): `InitializeCriticalSection`
/// on `this` (the import, `00fdf054`), then returns `this`.
pub fn fn_0044fae0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044fae0, fn_0044fae0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_the_section_and_returns_it() {
        let mut e = Engine::new();
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044fae0, &[0x5000]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (INITIALIZE_CRITICAL_SECTION, vec![0x5000]));
    }
}
