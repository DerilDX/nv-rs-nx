//! `fallout shared/pathfinding/pathinglosgridmap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006a62f0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const OFFSET_TARGET_006A62F0: u32 = 0x006a_62f0;

// Translated from 006e0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (callers `SaveGame` and `LoadGame` of the pathing LOS grid map):
/// forwards `this` and one word to `006a62f0`, which returns the word at `this+4` plus the
/// argument (`ADD EAX,[EBP+8]`). The wrapper returns that address.
pub fn fn_006e0ed0(e: &mut Engine, this: Ptr, offset: u32) -> Ptr {
    e.call(OFFSET_TARGET_006A62F0, &args![this, offset]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_0ed0, fn_006e0ed0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_006e0ed0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(OFFSET_TARGET_006A62F0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x006e_0ed0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == OFFSET_TARGET_006A62F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}
