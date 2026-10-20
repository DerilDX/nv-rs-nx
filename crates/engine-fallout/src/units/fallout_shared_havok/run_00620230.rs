//! `fallout shared/havok/run_00620230` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00825c00` (unnamed in the engine map): `thiscall`; the dword at `this + 0x14`.
const NODE_FIELD_14: u32 = 0x0082_5c00;
/// `0057b700` (unnamed in the engine map, `tesobjectrefr.cpp`): `thiscall`, `RET 4`.
const NODE_NOTIFY: u32 = 0x0057_b700;
/// `006205e0` (unnamed in the engine map): `thiscall`, `RET 4`.
const LISTENER_DROP_NODE: u32 = 0x0062_05e0;

// Translated from 006206d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8` (a node and a target). With a non-null
/// target: when the node's field `+0x14` (`00825c00`) is non-zero, hands the target to
/// `0057b700` on the node; then, when that field (read again) is zero, hands the node to
/// `006205e0` on `this`.
pub fn fn_006206d0(e: &mut Engine, this: Ptr, node: Ptr, target: Ptr) {
    if target.is_null() {
        return;
    }
    if e.call(NODE_FIELD_14, &args![node]).u32() != 0 {
        e.call(NODE_NOTIFY, &args![node, target]);
    }
    if e.call(NODE_FIELD_14, &args![node]).u32() == 0 {
        e.call(LISTENER_DROP_NODE, &args![this, node]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_06d0, fn_006206d0(Ptr, Ptr, Ptr))]
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NODE_FIELD_14, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(NODE_NOTIFY, |_, _| Ret::default());
        e.register(LISTENER_DROP_NODE, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_006206d0_notifies_a_node_that_has_the_field_and_keeps_it() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node + 0x14, 5);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0x9000u32]);
        assert_eq!(calls(&e, NODE_NOTIFY), vec![vec![node, 0x9000]]);
        assert!(calls(&e, LISTENER_DROP_NODE).is_empty());
        assert_eq!(calls(&e, NODE_FIELD_14).len(), 2);
    }

    #[test]
    fn at_006206d0_drops_a_node_without_the_field() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0x9000u32]);
        assert!(calls(&e, NODE_NOTIFY).is_empty());
        assert_eq!(calls(&e, LISTENER_DROP_NODE), vec![vec![0x4000, node]]);
    }

    #[test]
    fn at_006206d0_does_nothing_without_a_target() {
        let mut e = rig();
        let node = e.mem.alloc(0x20);
        e.call(0x0062_06d0, &args![0x4000u32, node, 0u32]);
        assert!(calls(&e, NODE_FIELD_14).is_empty());
        assert!(calls(&e, LISTENER_DROP_NODE).is_empty());
    }
}
