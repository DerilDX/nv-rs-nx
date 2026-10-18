//! `fallout shared/run_005e0ad0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00537e90` (named `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` in the engine map):
/// the base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const STORE_TARGET_00537E90: u32 = 0x0053_7e90;

// Translated from 005e0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `this` and one word to `00537e90` (the engine map names it
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer`; Xbox PDB
/// `??0BGSUnloadedFormBuffer@@QAA@PAD@Z` takes a `char *`), which stores the word at `this+0`
/// and leaves `this` in `EAX` (`MOV EAX,[EBP-4]` before the store). The wrapper returns that
/// `this`.
pub fn fn_005e0b80(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(STORE_TARGET_00537E90, &args![this, data]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_0b80, fn_005e0b80(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_005e0b80_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(STORE_TARGET_00537E90, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x005e_0b80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == STORE_TARGET_00537E90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}
