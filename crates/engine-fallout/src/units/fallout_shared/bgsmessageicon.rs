//! `fallout shared/bgsmessageicon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0047a300` calls on the member (`00408da0`).
const CALLEE_00408DA0: u32 = 0x0040_8da0;

// Translated from 0047a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00408da0` with `ECX` = `this + 0x4` and passes on its result.
pub fn fn_0047a300(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00408DA0, &args![this.byte_add(0x4)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047_a300, fn_0047a300(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0047a300_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00408DA0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a300, &args![0x4100u32]).u32(), 0x1451);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00408DA0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}
