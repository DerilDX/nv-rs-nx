//! `(unplaced)/run_0046ffb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Releases every node chained at `+4` of the container and clears its `+0`
/// word (`00470470`).
const CLEAR_CHAIN: u32 = 0x0047_0470;

// Translated from 0046ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the chained container: empties it (`00470470`).
pub fn fn_0046ffb0(e: &mut Engine, this: Ptr) {
    e.call(CLEAR_CHAIN, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0046ffb0, fn_0046ffb0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_the_container() {
        let mut e = Engine::new();
        e.register(CLEAR_CHAIN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0046_ffb0, &args![0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CLEAR_CHAIN, vec![0x1234]));
    }
}
