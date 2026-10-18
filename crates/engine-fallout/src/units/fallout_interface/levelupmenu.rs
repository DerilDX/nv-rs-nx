//! `fallout/interface/menus/levelupmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constructor body `007883b0` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_007883B0: u32 = 0x0078_83b0;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<EffectItemData,...>`).
const VTABLE_00788300: u32 = 0x0107_3e50;

// Translated from 00788300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `levelupmenu.cpp`): stores the vtable `01073e50` (RTTI
/// class `BSSimpleArray<EffectItemData,...>`), then calls `007883b0(this, 0, 0)`. Returns
/// `this`.
pub fn fn_00788300(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_00788300);
    e.call(CONSTRUCTOR_BODY_007883B0, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0078_8300, fn_00788300(Ptr) -> Ptr)]
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
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_007883B0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_8300, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_007883B0),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_00788300);
    }
}
