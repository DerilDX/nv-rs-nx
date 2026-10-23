//! `NiXenonRenderer/run_00e941a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e98610` (unnamed in the engine map; no arguments): runs first.
const RELEASE_00E98610: u32 = 0x00e9_8610;
/// The head of the singly linked list `fn_00e94290` frees (`0126fa30`; each node's next at
/// +0x3c, 0x44 bytes each).
const LIST_HEAD_0126FA30: u32 = 0x0126_fa30;
/// The node destructor body `00e98640` (unnamed; `thiscall`, no stack words).
const NODE_DESTRUCT: u32 = 0x00e9_8640;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00e94290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): runs `00e98610`, then destroys and frees every
/// node of the list headed at `0126fa30` (the next node read before each one goes: +0x3c;
/// `00e98640`, then the 0x44 bytes freed) and clears the head.
pub fn fn_00e94290(e: &mut Engine) {
    e.call(RELEASE_00E98610, &args![]);
    let mut node = e.mem.u32(LIST_HEAD_0126FA30);
    while node != 0 {
        let next = e.mem.u32(node.wrapping_add(0x3c));
        e.call(NODE_DESTRUCT, &args![node]);
        e.call(NI_DELETE, &args![node, 0x44u32]);
        node = next;
    }
    e.mem.set_u32(LIST_HEAD_0126FA30, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_4290, fn_00e94290())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e94290_frees_every_node_and_clears_the_head() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for callee in [RELEASE_00E98610, NODE_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        // The free clobbers the node's next word: the loop read it before.
        e.register(NI_DELETE, |e, a| {
            e.mem.set_u32(a[0] + 0x3c, 0xdead_beef);
            Ret::default()
        });
        let first = e.mem.alloc(0x44);
        let second = e.mem.alloc(0x44);
        e.mem.set_u32(first + 0x3c, second);
        e.set_global(LIST_HEAD_0126FA30, first);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4290, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (RELEASE_00E98610, vec![]),
                (NODE_DESTRUCT, vec![first]),
                (NI_DELETE, vec![first, 0x44]),
                (NODE_DESTRUCT, vec![second]),
                (NI_DELETE, vec![second, 0x44]),
            ]
        );
        assert_eq!(e.mem.u32(LIST_HEAD_0126FA30), 0);

        // An empty list: only the first call.
        e.call_log = Some(vec![]);
        e.call(0x00e9_4290, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
