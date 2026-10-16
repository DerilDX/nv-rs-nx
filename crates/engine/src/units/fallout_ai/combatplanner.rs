//! `fallout/ai/combat/combatplanner.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00995870` calls on the member (`00aa6330`).
const CALLEE_00AA6330: u32 = 0x00aa_6330;

/// The function `fn_00995890` calls on the member (`00aa62e0`).
const CALLEE_00AA62E0: u32 = 0x00aa_62e0;

// Translated from 00995870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa6330` with `ECX` = `this + 0x14` and passes on its `bool` result (`AL`).
pub fn fn_00995870(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_00AA6330, &args![this.byte_add(0x14)]).bool()
}

// Translated from 00995890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa62e0` with `ECX` = `this + 0x14` and passes on nothing (the callee returns no value).
pub fn fn_00995890(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00AA62E0, &args![this.byte_add(0x14)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_5870, fn_00995870(Ptr) -> bool),
        entry!(0x0099_5890, fn_00995890(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00995870_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA6330, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_5870, &args![0x4100u32]).bool());
        assert!(e.call(0x0099_5870, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA6330).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x14]);
    }

    #[test]
    fn l68d_00995890_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA62E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0099_5890, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA62E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x14]);
    }
}
