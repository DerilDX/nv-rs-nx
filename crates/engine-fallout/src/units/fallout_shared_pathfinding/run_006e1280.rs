//! `fallout shared/pathfinding/run_006e1280` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;

// Translated from 006e1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: "constructs" `count` 1-byte elements at `elements`
/// (placement `new` of size 1 on each; the type has no constructor). `this`
/// is kept but not read.
pub fn fn_006e1280(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        e.call(PLACEMENT_NEW, &args![1u32, elements.wrapping_add(index)]);
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_1280, fn_006e1280(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_each_byte() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_1280, &args![0u32, 0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == PLACEMENT_NEW)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            placed,
            vec![vec![1, 0x5000], vec![1, 0x5001], vec![1, 0x5002]]
        );
    }
}
