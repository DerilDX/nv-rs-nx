//! `fallout/ai/sandboxpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00558dd0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_00558DD0: u32 = 0x0055_8dd0;

// Translated from 009f6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (sandbox package; callers `ForgetTargetAndPickProcedure`,
/// `009f4700`): forwards `this` and an index to `00558dd0` (`tesobjectcell.cpp` per the engine
/// map), which returns the word at `this+4` plus `index` shifted left by 4: the address of a
/// 16-byte element. The wrapper returns that address.
pub fn fn_009f6190(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_00558DD0, &args![this, index]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009f_6190, fn_009f6190(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009f6190_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_00558DD0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_6190, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_00558DD0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}
