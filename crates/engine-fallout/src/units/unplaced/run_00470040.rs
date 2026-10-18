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

/// A hash map's vtable slots: the bucket of a key (+4), whether two keys are
/// equal (+8), filling a new node (+0xC); and the node allocator
/// (`00844a70`, `thiscall` on the map).
const MAP_HASH_SLOT: u32 = 0x4;
const MAP_EQUAL_SLOT: u32 = 0x8;
const MAP_SET_NODE_SLOT: u32 = 0xc;
const MAP_NEW_NODE: u32 = 0x0084_4a70;

// Translated from 00470200 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A hash map's `SetAt` (`thiscall`, `RET 8`: key, value). The map: vtable,
/// bucket count (+4), bucket array (+8), entry count (+0xC), copies-keys flag
/// (byte +0x10); a node: next (+0), key (+4), value (+8). When a node of the
/// key's bucket has an equal key, its value is replaced (and its key too
/// when the map doesn't copy keys); otherwise a new node (`00844a70`, filled
/// through slot +0xC) goes at the head of the bucket and the count grows.
pub fn fn_00470200(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, MAP_HASH_SLOT, &args![key]).u32();
    let mut node = e
        .mem
        .u32(e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let stored = e.mem.u32(node + 4);
        let equal = e.vcall(map, MAP_EQUAL_SLOT, &args![key, stored]).u32() as u8;
        if equal != 0 {
            if e.mem.u8(map + 0x10) == 0 {
                e.mem.set_u32(node + 4, key);
            }
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.call(MAP_NEW_NODE, &args![this]).u32();
    e.vcall(map, MAP_SET_NODE_SLOT, &args![node, key, value]);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc).wrapping_add(1);
    e.mem.set_u32(map + 0xc, count);
}

// Translated from 00470320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0046ffd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470320(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0046_ffd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0045d930` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470350(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0045_d930, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00470070` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0047_0070, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004703b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0045d950` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004703b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0045_d950, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004703e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00470110` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004703e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0047_0110, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00464f40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470410(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0046_4f40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00470200, fn_00470200(Ptr, u32, u32)),
        entry!(0x00470440, construct_node(Ptr, Ptr) -> Ptr),
        entry!(0x004702f0, fn_004702f0(Ptr, u32) -> Ptr),
        entry!(0x00470470, fn_00470470(Ptr)),
        entry!(0x0047_0320, fn_00470320(Ptr, u32) -> Ptr),
        entry!(0x0047_0350, fn_00470350(Ptr, u32) -> Ptr),
        entry!(0x0047_0380, fn_00470380(Ptr, u32) -> Ptr),
        entry!(0x0047_03b0, fn_004703b0(Ptr, u32) -> Ptr),
        entry!(0x0047_03e0, fn_004703e0(Ptr, u32) -> Ptr),
        entry!(0x0047_0410, fn_00470410(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_at_adds_a_node_then_replaces_its_value() {
        let mut e = Engine::new();
        // Hash: the key itself; equal: same word; set node: key and value.
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000, 0x0090_2000, 0x0090_3000]);
        e.register(0x0090_1000, |_, a| (a[1] % 4).into_ret());
        e.register(0x0090_2000, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0090_3000, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(MAP_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(16);
        e.mem.set_u32(map, 0x0090_0000);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, buckets);
        e.call(0x00470200, &[map, 6, 100]);
        e.call(0x00470200, &[map, 10, 200]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        // Both in bucket 2, the newer at the head.
        let head = e.mem.u32(buckets + 8);
        assert_eq!(e.mem.u32(head + 4), 10);
        let next = e.mem.u32(head);
        assert_eq!(e.mem.u32(next + 4), 6);
        e.call(0x00470200, &[map, 6, 300]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(e.mem.u32(next + 8), 300);
    }

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
    #[test]
    fn at_00470320_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0046_ffd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0320, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0320)
                .collect();
            let mut expected = vec![(0x0046_ffd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470350_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0045_d930, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0350, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0350)
                .collect();
            let mut expected = vec![(0x0045_d930, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470380_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0047_0070, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0380, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0380)
                .collect();
            let mut expected = vec![(0x0047_0070, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004703b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0045_d950, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_03b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_03b0)
                .collect();
            let mut expected = vec![(0x0045_d950, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004703e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0047_0110, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_03e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_03e0)
                .collect();
            let mut expected = vec![(0x0047_0110, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470410_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0046_4f40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0410, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0410)
                .collect();
            let mut expected = vec![(0x0046_4f40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
