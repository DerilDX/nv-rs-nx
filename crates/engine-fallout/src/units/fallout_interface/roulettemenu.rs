//! `fallout/interface/menus/roulettemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chained container's destructor body (`0046ffb0`, `thiscall`, no stack words).
const CONTAINER_DESTRUCT: u32 = 0x0046_ffb0;

// Translated from 007b9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a forwarder that runs the chained
/// container's destructor body (`0046ffb0`) on `this`.
pub fn fn_007b9000(e: &mut Engine, this: Ptr) {
    e.call(CONTAINER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007b_9000, fn_007b9000(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_007b9000_forwards_this_to_the_container_destructor() {
        let mut e = Engine::new();
        e.register(CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007b_9000, &args![0x0e00_5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(CONTAINER_DESTRUCT, vec![0x0e00_5000])]
        );
    }
}
