//! `fallout/ai/run_009ab9c0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009c5870` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_009C5870: u32 = 0x009c_5870;

// Translated from 009abef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c448, 0108eae8, 0108f378 and 0108f6f8):
/// forwards `this` and one pointer to `009c5870` (`projectile.cpp` per the engine map), which
/// calls `00932d60` and a virtual call on the word, then walks the list at `this+0x88`.
/// Returns nothing: its list loop ends with no value.
pub fn fn_009abef0(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_009C5870, &args![this, argument]);
}

/// `009c58e0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_009C58E0: u32 = 0x009c_58e0;

// Translated from 009abf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c434 and 0108ead4): forwards `this` and one
/// pointer to `009c58e0` (`projectile.cpp` per the engine map), which calls `00932f60` and
/// fills fields from the list at `this+0x88`. Returns nothing: it ends with a conditional
/// virtual call and no result.
pub fn fn_009abf10(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_009C58E0, &args![this, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_bef0, fn_009abef0(Ptr, Ptr)),
        entry!(0x009a_bf10, fn_009abf10(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009abef0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_009C5870, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_bef0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_009C5870)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_009abf10_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_009C58E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_bf10, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_009C58E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
