//! `(unplaced)/run_00470040` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The chained container: a head value at `+0` and a singly linked chain of
//! 8-byte nodes (value at `+0`, next node at `+4`) whose first node is at
//! `+4` of the container. Each node is itself such a container (its `+4`
//! word is the rest of the chain), so a node's destructor empties the chain
//! behind it.

#[allow(unused_imports)]
use crate::prelude::*;

/// Destructor body of the container (`0046ffb0`): empties it (`00470470`).
const CONTAINER_DESTRUCT: u32 = 0x0046_ffb0;
/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 004702f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Container destructor with the compiler's "scalar deleting" flag
/// (`RET 4`): destroys the contents (`0046ffb0`) and, when bit 0 of `flags`
/// is set, deletes the memory. Returns `this`.
pub fn fn_004702f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CONTAINER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00470470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the container: detaches each chained node from the one after it,
/// destroys and deletes it (`004702f0` with the delete flag), and finally
/// zeroes the head value at `+0`.
pub fn fn_00470470(e: &mut Engine, this: Ptr) {
    loop {
        let node = e.mem.u32(this.addr() + 4);
        if node == 0 {
            break;
        }
        let rest = e.mem.u32(node + 4);
        e.mem.set_u32(node + 4, 0);
        // The game re-reads the chain head and tests it again before the
        // call; it is the same node.
        let node = e.mem.u32(this.addr() + 4);
        if node != 0 {
            fn_004702f0(e, Ptr::new(node), 1);
        }
        e.mem.set_u32(this.addr() + 4, rest);
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00470440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the pointed-to item into a new list node and clears its link.
/// The source is read before either store, including when it aliases a field.
pub fn construct_node(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let item = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), item);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00470440, construct_node(Ptr, Ptr) -> Ptr),
        entry!(0x004702f0, fn_004702f0(Ptr, u32) -> Ptr),
        entry!(0x00470470, fn_00470470(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_copies_before_clearing_an_aliased_source_and_keeps_guards() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        e.mem.write(object, &[0xaa; 16]);
        e.mem.set_u32(object + 8, 0x12345678);
        assert_eq!(
            e.call(0x00470440, &args![object + 4, object + 8]).u32(),
            object + 4
        );
        assert_eq!(e.mem.u32(object), 0xaaaaaaaa);
        assert_eq!(e.mem.u32(object + 4), 0x12345678);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 12), 0xaaaaaaaa);
    }

    fn doubles(e: &mut Engine) {
        e.register(CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    #[test]
    fn destructor_without_the_delete_flag_keeps_the_memory() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0047_02f0, &args![0x1000u32, 0u32]).u32();
        assert_eq!(back, 0x1000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (CONTAINER_DESTRUCT, vec![0x1000]));
    }

    #[test]
    fn destructor_with_the_delete_flag_deletes() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        // Only bit 0 of the flags counts.
        e.call(0x0047_02f0, &args![0x1000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (OPERATOR_DELETE, vec![0x1000]));
        e.call_log = Some(vec![]);
        e.call(0x0047_02f0, &args![0x1000u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn clearing_destroys_each_node_in_chain_order_and_zeroes_the_head() {
        let mut e = Engine::new();
        doubles(&mut e);
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(container.addr(), 77);
        e.mem.set_u32(container.addr() + 4, first);
        e.mem.set_u32(first + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x0047_0470, &args![container]);
        let log = e.call_log.take().unwrap();
        let destroyed: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == CONTAINER_DESTRUCT)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(destroyed, vec![first, second]);
        // Each node was detached from its successor before it was destroyed.
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(container.addr() + 4), 0);
        assert_eq!(e.mem.u32(container.addr()), 0);
    }

    #[test]
    fn clearing_an_empty_container_only_zeroes_the_head() {
        let mut e = Engine::new();
        doubles(&mut e);
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(container.addr(), 77);
        e.call_log = Some(vec![]);
        e.call(0x0047_0470, &args![container]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(container.addr()), 0);
    }
}
