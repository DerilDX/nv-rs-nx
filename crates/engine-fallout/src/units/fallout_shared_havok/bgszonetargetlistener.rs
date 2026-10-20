//! `fallout shared/havok/bgszonetargetlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00620110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104d9e0` (RTTI `.?AVBGSZoneTargetListener@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006201c0`. One word, plain `RET`, returns nothing.
pub fn fn_00620110(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_d9e0);
    e.call(0x0062_01c0, &args![this]);
}

/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4` (a list's first
/// node).
const LIST_FIRST_NODE: u32 = 0x0072_6070;
/// `00620180` (`run_00620180.rs`): `thiscall`; the node after a node of the zone target list.
const ZONE_TARGET_NEXT: u32 = 0x0062_0180;
/// `006206d0` (`run_00620230.rs`): `thiscall`, `RET 8`; (node, target).
const ZONE_TARGET_NODE_REMOVE: u32 = 0x0062_06d0;
/// `006207a0` (unnamed in the engine map): `thiscall`, `RET 4`; called on the member at the
/// target's `+0x10` with one pointer, it answers a pointer (what it does is unconfirmed).
const ZONE_TARGET_LIST_FIND: u32 = 0x0062_07a0;
/// `0087e9b0` (unnamed in the engine map, `actor.cpp`): `cdecl`, one stack word.
const ZONE_TARGET_RELEASE: u32 = 0x0087_e9b0;

// Translated from 00620130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSZoneTargetListener::RemoveTarget` (Xbox PDB): `thiscall`, `RET 4`. With a non-null
/// target, walks the node chain that starts at the listener's `+4` list (`00726070`) and hands
/// every node, with the target, to `006206d0`; the next node is fetched (`00620180`) before the
/// node is handed over. The decompiler's `UMSFreeVirtualProcessorRoot::SchedulerProxy` is
/// `00620180`'s library-match name, which its body is not.
pub fn bgs_zone_target_listener_remove_target(e: &mut Engine, this: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    let mut node = e.call(LIST_FIRST_NODE, &args![this]).u32();
    while node != 0 {
        let next = e.call(ZONE_TARGET_NEXT, &args![node]).u32();
        e.call(ZONE_TARGET_NODE_REMOVE, &args![this, node, target]);
        node = next;
    }
}

// Translated from 0061ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. With a non-null target: asks the target's
/// member at `+0x10` (`006207a0`) about the pointer `*(this + 0x14) + 0x10` (null when the
/// listener has none), stores the answer minus `0x10` (or null) back at `this + 0x14`
/// and hands the target to `0087e9b0`.
pub fn fn_0061ff30(e: &mut Engine, this: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    let current = e.mem.u32(this.addr().wrapping_add(0x14));
    let key = if current == 0 {
        0
    } else {
        current.wrapping_add(0x10)
    };
    let found = e
        .call(
            ZONE_TARGET_LIST_FIND,
            &args![target.addr().wrapping_add(0x10), key],
        )
        .u32();
    let updated = if found == 0 {
        0
    } else {
        found.wrapping_sub(0x10)
    };
    e.mem.set_u32(this.addr().wrapping_add(0x14), updated);
    e.call(ZONE_TARGET_RELEASE, &args![target]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0062_0130,
            bgs_zone_target_listener_remove_target(Ptr, Ptr)
        ),
        entry!(0x0061_ff30, fn_0061ff30(Ptr, Ptr)),
        entry!(0x0062_0110, fn_00620110(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00620110_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0062_01c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_0110, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0062_01c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_d9e0);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_d9e0,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00620130_hands_every_node_to_the_remove_function_with_the_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // A chain 0x100 -> 0x200 -> 0x300 -> end, kept at +4 of the node's own storage.
        e.register(LIST_FIRST_NODE, |_, _| 0x100u32.into_ret());
        e.register(ZONE_TARGET_NEXT, |_, a| match a[0] {
            0x100 => 0x200u32.into_ret(),
            0x200 => 0x300u32.into_ret(),
            _ => Ret::default(),
        });
        e.register(ZONE_TARGET_NODE_REMOVE, |_, _| Ret::default());
        e.call(0x0062_0130, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls(&e, LIST_FIRST_NODE), vec![vec![0x4000]]);
        assert_eq!(
            calls(&e, ZONE_TARGET_NEXT),
            vec![vec![0x100], vec![0x200], vec![0x300]]
        );
        assert_eq!(
            calls(&e, ZONE_TARGET_NODE_REMOVE),
            vec![
                vec![0x4000, 0x100, 0x9000],
                vec![0x4000, 0x200, 0x9000],
                vec![0x4000, 0x300, 0x9000]
            ]
        );
    }

    #[test]
    fn at_00620130_does_nothing_without_a_target_or_nodes() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_FIRST_NODE, |_, _| Ret::default());
        e.call(0x0062_0130, &args![0x4000u32, 0u32]);
        assert!(calls(&e, LIST_FIRST_NODE).is_empty());
        e.call(0x0062_0130, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls(&e, LIST_FIRST_NODE).len(), 1);
        assert!(calls(&e, ZONE_TARGET_NODE_REMOVE).is_empty());
    }

    #[test]
    fn at_0061ff30_replaces_the_stored_node_by_the_one_found_in_the_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_TARGET_LIST_FIND, |_, a| (a[1] + 0x100).into_ret());
        e.register(ZONE_TARGET_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, ZONE_TARGET_LIST_FIND), vec![vec![0x9010, 0x5010]]);
        // found = 0x5110, minus 0x10.
        assert_eq!(e.mem.u32(this + 0x14), 0x5100);
        assert_eq!(calls(&e, ZONE_TARGET_RELEASE), vec![vec![0x9000]]);
    }

    #[test]
    fn at_0061ff30_handles_an_empty_listener_and_a_miss() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ZONE_TARGET_LIST_FIND, |_, _| Ret::default());
        e.register(ZONE_TARGET_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, ZONE_TARGET_LIST_FIND), vec![vec![0x9010, 0]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn at_0061ff30_ignores_a_null_target() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x0061_ff30, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
        assert!(calls(&e, ZONE_TARGET_RELEASE).is_empty());
    }
}
