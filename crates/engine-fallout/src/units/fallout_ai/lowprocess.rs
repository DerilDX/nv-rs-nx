//! `fallout/ai/lowprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00913810` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_00913810: u32 = 0x0091_3810;
/// The vtable stored at `+0` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`).
const VTABLE_009137B0: u32 = 0x0108_8fd4;

// Translated from 009137b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`): runs `00913810(this, argument)`, then
/// stores the vtable `01088fd4` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`) at `+0`.
/// Returns `this`.
pub fn fn_009137b0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00913810, &args![this, argument]);
    e.mem.set_u32(this.addr(), VTABLE_009137B0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0091_37b0, fn_009137b0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_00913810, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0091_37b0, &args![this, 0x27u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00913810),
            vec![vec![this, 0x27]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_009137B0);
    }
}
