//! `fallout/interface/menus/hackingmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00700270` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const TEST_TARGET_00700270: u32 = 0x0070_0270;

// Translated from 0076a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (hacking menu, only caller `00769dfe`, which pushes the address
/// of a byte local and tests `AL`): forwards `this` and that pointer to `00700270`, which
/// returns 1 when `007002a0` returns 0 in `AL` and 0 otherwise (`MOVZX; NEG; SBB; ADD 1`). The
/// wrapper returns it as a byte.
pub fn fn_0076a3d0(e: &mut Engine, this: Ptr, byte_address: Ptr) -> bool {
    e.call(TEST_TARGET_00700270, &args![this, byte_address])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076_a3d0, fn_0076a3d0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0076a3d0_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(TEST_TARGET_00700270, |_, a| (0x100 | (a[1] & 1)).into_ret());
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0076_a3d0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == TEST_TARGET_00700270)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}
