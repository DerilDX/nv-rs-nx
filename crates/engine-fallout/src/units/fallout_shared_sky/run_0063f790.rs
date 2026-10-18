//! `fallout shared/sky/run_0063f790` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chained container's node destructor (`004702f0`, `RET 4`): destroys
/// the node's own chain and, with bit 0 of the flags set, deletes it.
const NODE_DESTRUCT: u32 = 0x0047_02f0;

// Translated from 0063f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the first node of the chained container `this` (head value at
/// `+0`, chain of 8-byte nodes starting at `+4`, see `00470040`): the head
/// value takes the node's value, the chain starts at the node's successor,
/// and the node is detached and deleted. With an empty chain the head value
/// is zeroed instead.
pub fn fn_0063f7b0(e: &mut Engine, this: Ptr) {
    let node = e.mem.u32(this.addr() + 4);
    if node == 0 {
        e.mem.set_u32(this.addr(), 0);
        return;
    }
    let next = e.mem.u32(node + 4);
    e.mem.set_u32(this.addr() + 4, next);
    let value = e.mem.u32(node);
    e.mem.set_u32(this.addr(), value);
    e.mem.set_u32(node + 4, 0);
    // The game tests the node against null again here; it never is.
    e.call(NODE_DESTRUCT, &args![node, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063f7b0, fn_0063f7b0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_the_first_node_into_the_head() {
        let mut e = Engine::new();
        e.register(NODE_DESTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(container.addr(), 1);
        e.mem.set_u32(container.addr() + 4, first);
        e.mem.set_u32(first, 2);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 3);
        e.call_log = Some(vec![]);
        e.call(0x0063_f7b0, &args![container]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NODE_DESTRUCT, vec![first, 1]));
        assert_eq!(e.mem.u32(container.addr()), 2);
        assert_eq!(e.mem.u32(container.addr() + 4), second);
        // The node was detached before it was destroyed.
        assert_eq!(e.mem.u32(first + 4), 0);
    }

    #[test]
    fn an_empty_chain_zeroes_the_head() {
        let mut e = Engine::new();
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(container.addr(), 9);
        e.call_log = Some(vec![]);
        e.call(0x0063_f7b0, &args![container]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(container.addr()), 0);
    }
}
