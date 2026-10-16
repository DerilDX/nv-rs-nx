//! `fallout/interface/menus/recipemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0072ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks `count` elements from `address` and does
/// nothing to them (the loop body is empty in the exe). `this` and
/// `address` are not read.
pub fn fn_0072ba80(_e: &mut Engine, _this: Ptr, _address: u32, _count: u32) {}

/// The constructor body `006b3eb0` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_006B3EB0: u32 = 0x006b_3eb0;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<TESRecipeCategory*,...>`).
const VTABLE_0072CE30: u32 = 0x0107_0660;

// Translated from 0072ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `recipemenu.cpp`): stores the vtable `01070660` (RTTI
/// class `BSSimpleArray<TESRecipeCategory*,...>`), then calls `006b3eb0(this, 0, 0)`.
/// Returns `this`.
pub fn fn_0072ce30(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_0072CE30);
    e.call(CONSTRUCTOR_BODY_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0072ba80, fn_0072ba80(Ptr, u32, u32)),
        entry!(0x0072_ce30, fn_0072ce30(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touches_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let before = e.mem.alloc(8);
        e.mem.set_u32(before, 0x55);
        e.call(0x0072ba80, &args![0u32, before, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "no other call");
        assert_eq!(e.mem.u32(before), 0x55);
    }
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
        e.register(CONSTRUCTOR_BODY_006B3EB0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ce30, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_006B3EB0),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_0072CE30);
    }
}
