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

// Translated from 007273f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all fourteen tile pointers from +0x28 of the menu (`this`) are
/// set (the recipe menu's tile table; the class layout is not declared
/// here).
pub fn fn_007273f0(e: &mut Engine, this: Ptr) -> bool {
    (0..0xe).all(|i| e.mem.u32(this.addr().wrapping_add(0x28 + 4 * i)) != 0)
}

/// `qsort` (cdecl: base, count, element size, compare).
const QSORT: u32 = 0x00ec_6f20;

// Translated from 00729970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the array of `this` (a `BSSimpleArray`-shaped object: data pointer
/// at +4, count at +8) with `qsort` and the compare function `compare`,
/// four-byte elements.
pub fn fn_00729970(e: &mut Engine, this: Ptr, compare: u32) {
    let data = e.mem.u32(this.addr().wrapping_add(4));
    let count = e.mem.u32(this.addr().wrapping_add(8));
    e.call(QSORT, &args![data, count, 4u32, compare]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0072ba80, fn_0072ba80(Ptr, u32, u32)),
        entry!(0x0072_ce30, fn_0072ce30(Ptr) -> Ptr),
        entry!(0x0072_73f0, fn_007273f0(Ptr) -> bool),
        entry!(0x0072_9970, fn_00729970(Ptr, u32)),
    ]
}

#[cfg(test)]
mod b070_tests {
    use super::*;

    #[test]
    fn all_fourteen_tiles_must_be_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        for i in 0..14 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1 + i);
        }
        assert!(e.call(0x0072_73f0, &args![this]).bool());
        e.mem.set_u32(this + 0x28 + 4 * 13, 0);
        assert!(!e.call(0x0072_73f0, &args![this]).bool());
    }

    #[test]
    fn sorts_the_array_with_qsort() {
        let mut e = Engine::new();
        e.register(QSORT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 7);
        e.call_log = Some(vec![]);
        e.call(0x0072_9970, &args![this, 0x0072_0000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == QSORT).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x5000, 7, 4, 0x0072_0000]);
    }
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
