//! `fallout shared/havok/testraplistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00620810` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LIST_FIND_TARGET_00620810: u32 = 0x0062_0810;

// Translated from 0062db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `0062dae5`): forwards `this` and one word to
/// `00620810`, which walks the chain starting at `this` (next link from `00726070`) until
/// `0044ddc0(node)` equals the word and returns that node, or null when the chain ends. The
/// wrapper returns it.
pub fn fn_0062db40(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(LIST_FIND_TARGET_00620810, &args![this, value]).ptr()
}

/// `0057c400` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LIST_FIND_TARGET_0057C400: u32 = 0x0057_c400;

// Translated from 0062df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `0062df33`): forwards `this` and one word to
/// `0057c400`, which walks the chain starting at `this` (next link from `00726070`) until
/// `0084e3a0(node)` equals the word and returns that node, or null when the chain ends. The
/// wrapper returns it.
pub fn fn_0062df60(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(LIST_FIND_TARGET_0057C400, &args![this, value]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_db40, fn_0062db40(Ptr, u32) -> Ptr),
        entry!(0x0062_df60, fn_0062df60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0062db40_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LIST_FIND_TARGET_00620810, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_db40, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LIST_FIND_TARGET_00620810)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }

    #[test]
    fn forwarder_fn_0062df60_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LIST_FIND_TARGET_0057C400, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_df60, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LIST_FIND_TARGET_0057C400)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}
