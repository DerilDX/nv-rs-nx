//! `fallout/ai/combat/combatprocedurebeincover.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 7 (`ECX` not read, no stack words). The map names it
/// `CombatProcedureBeInCover::GetType` (through that class's vtable), but
/// the body is shared by identical-code folding: several vtables point at
/// it, and `TESAttributes`' save and load (`00480100`, `00480130`) call it
/// for the size of the attribute data.
pub fn fn_009d2510(_e: &mut Engine, _this: Ptr) -> u32 {
    7
}

/// The function `fn_009d3370` calls on the member (`006e4820`).
const CALLEE_006E4820: u32 = 0x006e_4820;

// Translated from 009d3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006e4820` with `ECX` = `this + 0x14` and passes on its `bool` result (`AL`).
pub fn fn_009d3370(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_006E4820, &args![this.byte_add(0x14)]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d2510, fn_009d2510(Ptr) -> u32),
        entry!(0x009d_3370, fn_009d3370(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_seven() {
        let mut e = Engine::new();
        let back = e.call(0x009d_2510, &args![Ptr::<()>::new(0x1234)]).u32();
        assert_eq!(back, 7);
    }

    #[test]
    fn l68d_009d3370_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006E4820, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009d_3370, &args![0x4100u32]).bool());
        assert!(e.call(0x009d_3370, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006E4820).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x14]);
    }
}
