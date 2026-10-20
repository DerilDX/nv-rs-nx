//! `fallout shared/pathfinding/run_006e3c90` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00692710`: the 0x24-byte element's constructor (`thiscall`).
const ELEMENT_CONSTRUCT: u32 = 0x0069_2710;

// Translated from 006e3c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e3c90 - the C++ exception frame (FS:[0]) is not translated
/// No map name: constructs `count` 0x24-byte elements at `elements`
/// (placement `new`, then `00692710` on each non-null result). `this` is
/// kept but not read.
pub fn fn_006e3c90(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(
                PLACEMENT_NEW,
                &args![0x24u32, index.wrapping_mul(0x24).wrapping_add(elements)],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![element]);
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_3c90, fn_006e3c90(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_each_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.register(ELEMENT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_3c90, &args![0u32, 0x5000u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ELEMENT_CONSTRUCT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(built, vec![0x5000, 0x5024]);
        e.register(PLACEMENT_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_3c90, &args![0u32, 0x5000u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ELEMENT_CONSTRUCT));
    }
}
