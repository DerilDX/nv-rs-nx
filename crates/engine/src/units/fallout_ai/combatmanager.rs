//! `fallout/ai/combat/combatmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00993bf0` calls on the member (`00993cd0`).
const CALLEE_00993CD0: u32 = 0x0099_3cd0;

// Translated from 00993bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<Actor*,__int64>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00993cd0` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00993bf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00993CD0, &args![this.byte_add(0xc)]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0099_3bf0, fn_00993bf0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00993bf0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00993CD0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_3bf0, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00993CD0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}
