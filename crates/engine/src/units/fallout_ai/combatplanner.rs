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

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DC80: u32 = 0x0108_dc80;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCA8: u32 = 0x0108_dca8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCBC: u32 = 0x0108_dcbc;

// Translated from 00996540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996540(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DC80);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00996640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996640(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCA8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009966f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009966f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCBC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_5870, fn_00995870(Ptr) -> bool),
        entry!(0x0099_5890, fn_00995890(Ptr)),
        entry!(0x0099_6540, fn_00996540(Ptr)),
        entry!(0x0099_6640, fn_00996640(Ptr)),
        entry!(0x0099_66f0, fn_009966f0(Ptr)),
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
    fn fn_00996540_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6540, ARRAY_VTABLE_0108DC80, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_00996640_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6640, ARRAY_VTABLE_0108DCA8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009966f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_66f0, ARRAY_VTABLE_0108DCBC, ARRAY_RELEASE_008454F0);
    }
}
