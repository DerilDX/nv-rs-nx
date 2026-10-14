//! `fallout shared/pathfinding/run_006d03a0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// Bytes per element of the arrays `fn_006d03d0` prepares.
const ELEMENT_SIZE: u32 = 0xc;

// Translated from 006d03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `count` 12-byte elements from `first` on (unnamed in the map;
/// `ECX` not read, two stack words, `RET 8`): for each element, placement
/// `new` (`006e6da0`, size 12) and nothing else, the element having no
/// constructor of its own.
pub fn fn_006d03d0(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = index.wrapping_mul(ELEMENT_SIZE).wrapping_add(first);
        e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, place]);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d03d0, fn_006d03d0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_each_of_the_elements() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x0800_0000);
        e.call(0x006d_03d0, &args![this, 0x0900_0000u32, 3u32]);
        let places: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == PLACEMENT_NEW)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            places,
            vec![
                vec![12, 0x0900_0000],
                vec![12, 0x0900_000c],
                vec![12, 0x0900_0018],
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x006d_03d0, &args![this, 0x0900_0000u32, 0u32]);
        let log = e.call_log.unwrap();
        assert!(log.iter().all(|c| c.0 != PLACEMENT_NEW));
    }
}
