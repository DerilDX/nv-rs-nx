//! `fallout/magic/commandeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401290` (named `DetailedActorPathHandler::IsDetailedPathHandler` in the engine map):
/// the base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const ALWAYS_TRUE_TARGET_00401290: u32 = 0x0040_1290;

// Translated from 0080b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01079ae8, just before `CommandEffect::Start`):
/// forwards `this` and one word to `00401290`, a body shared by many slots (named
/// `DetailedActorPathHandler::IsDetailedPathHandler`) that ignores both and returns `MOV
/// AL,1`. The wrapper returns that byte.
pub fn fn_0080b640(e: &mut Engine, this: Ptr, argument: u32) -> bool {
    e.call(ALWAYS_TRUE_TARGET_00401290, &args![this, argument])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_b640, fn_0080b640(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0080b640_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(ALWAYS_TRUE_TARGET_00401290, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0080_b640, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == ALWAYS_TRUE_TARGET_00401290)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}
