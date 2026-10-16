//! `fallout/interface/run_007da070` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VVideoRes@StartMenu@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01077200: u32 = 0x0107_7200;
/// `007d8e20` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007D8E20: u32 = 0x007d_8e20;

// Translated from 007da0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VVideoRes@StartMenu@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007d8e20(this, 1)`.
pub fn fn_007da0f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01077200);
    e.call(ARRAY_RELEASE_007D8E20, &args![this, 1u32]);
}

/// The constructor body `007da110` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_007DA110: u32 = 0x007d_a110;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<StartMenu::VideoRes,...>`).
const VTABLE_007DA090: u32 = 0x0107_7200;

// Translated from 007da090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `01077200` (RTTI class
/// `BSSimpleArray<StartMenu::VideoRes,...>`), then calls `007da110(this, 0, 0)`. Returns
/// `this`.
pub fn fn_007da090(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_007DA090);
    e.call(CONSTRUCTOR_BODY_007DA110, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007d_a0f0, fn_007da0f0(Ptr)),
        entry!(0x007d_a090, fn_007da090(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007da0f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x007d_a0f0, ARRAY_VTABLE_01077200, ARRAY_RELEASE_007D8E20);
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
        e.register(CONSTRUCTOR_BODY_007DA110, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a090, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_007DA110),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_007DA090);
    }
}
