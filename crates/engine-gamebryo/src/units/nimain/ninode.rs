//! `NiMain/ninode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a5ece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a5e930`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xac bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_e930;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xacu32]);
    }
    this
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// `NiNode`'s vtable (`0109b5ac`, RTTI `.?AVNiNode@@`).
const NI_NODE_VTABLE: u32 = 0x0109_b5ac;
/// The child array's constructor `004aff00` (`thiscall`, `RET 8`: initial size, grow-by).
const CHILD_ARRAY_CONSTRUCT: u32 = 0x004a_ff00;

// Translated from 00a5ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::NiNode` (unnamed in the engine map; the vtable it sets is `NiNode`'s by its RTTI).
/// `thiscall`, one word (`RET 4`): runs `NiAVObject`'s constructor, sets the vtable and
/// constructs the child array `m_kChildren` (+0x9c) with that initial size and a grow-by of 1
/// (`004aff00`). Answers `this`.
pub fn ni_node_ni_node(e: &mut Engine, this: Ptr, child_count: u32) -> Ptr {
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_NODE_VTABLE);
    e.call(
        CHILD_ARRAY_CONSTRUCT,
        &args![this.addr().wrapping_add(0x9c), child_count, 1u32],
    );
    this
}

/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiAVObject::AttachParent` (engine map; `thiscall` on the child, `RET 4`: the parent).
const NI_AV_OBJECT_ATTACH_PARENT: u32 = 0x00a5_9d00;
/// The child array's first-free-slot store `00a5eb20` (`thiscall` on the array, `RET 4`: the
/// address of the smart pointer to store).
const CHILD_ARRAY_ADD_FIRST_EMPTY: u32 = 0x00a5_eb20;
/// The child array's resize `0060bef0` (`thiscall` on the array, `RET 4`: the new size).
const CHILD_ARRAY_SET_SIZE: u32 = 0x0060_bef0;
/// The child array's store at an index `004b0310` (`thiscall` on the array, `RET 8`: the
/// index, the address of the smart pointer to store).
const CHILD_ARRAY_SET_AT: u32 = 0x004b_0310;

// Translated from 00a5ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::AttachChild` (Xbox PDB). `thiscall`, two words (`RET 8`): the child and a flag
/// (its low byte is read; `bFirstAvail` in the Xbox PDB). Nothing for a null child. Otherwise
/// the child's reference count (+4) is raised, `NiAVObject::AttachParent(child, this)` runs,
/// and a temporary smart pointer to the child (raising the count again) is stored in the
/// child array `m_kChildren` (+0x9c): with the flag, in the first free slot (`00a5eb20`);
/// without it, at index `m_usSize` (+0xa6), after growing the array to `m_usSize +
/// m_usGrowBy` (+0xaa, `0060bef0`) when `m_usSize` is not below `m_usMaxSize` (+0xa4)
/// (`004b0310`). The two references are then dropped, each calling the child's slot +4
/// (its deleting destructor) when the count reaches zero.
pub fn ni_node_attach_child(e: &mut Engine, this: Ptr, child: u32, first_available: u32) {
    if child == 0 {
        return;
    }
    let node = this.addr();
    let count = child + 4;
    e.call(INTERLOCKED_INCREMENT, &args![count]);
    e.call(NI_AV_OBJECT_ATTACH_PARENT, &args![child, node]);
    let children = node + 0x9c;
    // The temporary smart pointer (the game reuses the flag's argument slot for it).
    e.with_stack(4, |e, temp| {
        e.mem.set_u32(temp.addr(), child);
        e.call(INTERLOCKED_INCREMENT, &args![count]);
        if first_available as u8 != 0 {
            e.call(CHILD_ARRAY_ADD_FIRST_EMPTY, &args![children, temp]);
        } else {
            let size = u32::from(e.mem.u16(node + 0xa6));
            let max_size = u32::from(e.mem.u16(node + 0xa4));
            if size >= max_size {
                let grow_by = u32::from(e.mem.u16(children + 0xe));
                e.call(CHILD_ARRAY_SET_SIZE, &args![children, grow_by + size]);
            }
            e.call(CHILD_ARRAY_SET_AT, &args![children, size, temp]);
        }
    });
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        e.vcall(child, 4, &[]);
    }
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        e.vcall(child, 4, &[]);
    }
}

/// `0060bef0`: the pointer array's set-capacity (`thiscall`, `RET 4`: the new
/// capacity).
const POINTER_ARRAY_SET_CAPACITY: u32 = 0x0060_bef0;
/// `004b0310`: stores an element at an index (`thiscall`, `RET 8`: the index,
/// the address of the `NiPointer` to store).
const POINTER_ARRAY_SET_AT: u32 = 0x004b_0310;

// Translated from 00a5eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. On an `NiTArray` of `NiPointer`s (layout
/// [`crate::types::NiTArray`]) it adds the pointer `*element` in the first
/// empty slot: `thiscall`, one word (`RET 4`), the address of the `NiPointer`.
/// A null pointer answers `0xffffffff`. Otherwise it scans
/// `0..m_usSize` for a null slot; there it releases the slot's old object
/// (always null here, the branch is translated as the exe has it: when not the
/// same pointer and non-null, `InterlockedDecrement` of its count and, at
/// zero, vtable slot `+4`), stores the pointer with `InterlockedIncrement`
/// of its count, adds 1 to `m_usESize` and answers the index. Without an
/// empty slot it grows the array to `m_usSize + m_usGrowBy` (`0060bef0`) when
/// `m_usSize` (read before the scan) has reached `m_usMaxSize`, stores the
/// pointer at index `m_usSize` (`004b0310`) and answers that index.
pub fn fn_00a5eb20(e: &mut Engine, this: Ptr<crate::types::NiTArray>, element: Ptr) -> u32 {
    use crate::types::NiTArray;
    let value = e.mem.u32(element.addr());
    if value == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize);
    let mut index: u16 = 0;
    if index < size {
        let base = e.get(this, NiTArray::m_pBase);
        loop {
            let slot = base.wrapping_add(u32::from(index).wrapping_mul(4));
            if e.mem.u32(slot) == 0 {
                let old = e.mem.u32(slot);
                if old != value {
                    if old != 0
                        && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                            .u32()
                            == 0
                    {
                        e.vcall(old, 4, &args![]);
                    }
                    let new = e.mem.u32(element.addr());
                    e.mem.set_u32(slot, new);
                    if new != 0 {
                        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
                    }
                }
                let filled = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, filled.wrapping_add(1));
                return u32::from(index);
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTArray::m_usSize) {
                break;
            }
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        e.call(
            POINTER_ARRAY_SET_CAPACITY,
            &args![this, grow_by.wrapping_add(size)],
        );
    }
    e.call(POINTER_ARRAY_SET_AT, &args![this, size, element]);
    size
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5ece0, ni_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_ecb0, ni_node_ni_node(Ptr, u32) -> Ptr),
        entry!(0x00a5_ed10, ni_node_attach_child(Ptr, u32, u32)),
        entry!(
            0x00a5_eb20,
            fn_00a5eb20(Ptr<crate::types::NiTArray>, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a5ece0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a5ece0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xac]]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn ni_node_ni_node_builds_the_object_and_its_child_array() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CHILD_ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xac);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_ecb0, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00a5_ecb0, NI_AV_OBJECT_CONSTRUCT, CHILD_ARRAY_CONSTRUCT]
        );
        let args_of = |addr: u32| log.iter().find(|c| c.0 == addr).unwrap().1.clone();
        assert_eq!(args_of(NI_AV_OBJECT_CONSTRUCT), vec![this]);
        assert_eq!(args_of(CHILD_ARRAY_CONSTRUCT), vec![this + 0x9c, 3, 1]);
        assert_eq!(e.mem.u32(this), NI_NODE_VTABLE);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const CHILD_VTABLE: u32 = 0x0300_0000;
    const CHILD_DELETE: u32 = 0x0310_0004;

    /// A node with the given child-array sizes and a child whose count starts at `count`
    /// (kept at child +8 for the doubles' checks).
    fn attach_engine(size: u16, max_size: u16, count: i32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], v);
            v.into_ret()
        });
        e.register(NI_AV_OBJECT_ATTACH_PARENT, |e, a| {
            // The count is raised once before the parent is attached.
            assert_eq!(e.mem.i32(a[0] + 4), e.mem.i32(a[0] + 8) + 1);
            Ret::default()
        });
        e.register(CHILD_ARRAY_ADD_FIRST_EMPTY, |e, a| {
            let child = e.mem.u32(a[1]);
            assert_eq!(e.mem.i32(child + 4), e.mem.i32(child + 8) + 2);
            Ret::default()
        });
        e.register(CHILD_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(CHILD_ARRAY_SET_AT, |e, a| {
            let child = e.mem.u32(a[2]);
            assert_eq!(e.mem.i32(child + 4), e.mem.i32(child + 8) + 2);
            Ret::default()
        });
        e.register(CHILD_DELETE, |_, _| Ret::default());
        e.put_vtable(CHILD_VTABLE, &[0, CHILD_DELETE]);
        let node = e.mem.alloc(0xac);
        e.mem.set_u16(node + 0xa4, max_size);
        e.mem.set_u16(node + 0xa6, size);
        e.mem.set_u16(node + 0xaa, 4);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.mem.set_i32(child + 4, count);
        e.mem.set_i32(child + 8, count);
        (e, node, child)
    }

    #[test]
    fn ni_node_attach_child_appends_and_grows_the_array() {
        // Full array: grows to size + grow-by, then stores at the old size. The count ends
        // where it started; the second drop reaching zero deletes the child.
        let (mut e, node, child) = attach_engine(3, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0x100u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, NI_AV_OBJECT_ATTACH_PARENT),
            vec![vec![child, node]]
        );
        assert_eq!(
            calls_to(&log, CHILD_ARRAY_SET_SIZE),
            vec![vec![node + 0x9c, 7]]
        );
        let set_at = calls_to(&log, CHILD_ARRAY_SET_AT);
        assert_eq!(set_at.len(), 1);
        assert_eq!(set_at[0][..2], [node + 0x9c, 3]);
        assert!(calls_to(&log, CHILD_ARRAY_ADD_FIRST_EMPTY).is_empty());
        assert_eq!(e.mem.i32(child + 4), 0);
        assert_eq!(calls_to(&log, CHILD_DELETE), vec![vec![child]]);
        // Room left: no growth; a held child is not deleted.
        let (mut e, node, child) = attach_engine(2, 3, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CHILD_ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&log, CHILD_ARRAY_SET_AT)[0][1], 2);
        assert_eq!(e.mem.i32(child + 4), 5);
        assert!(calls_to(&log, CHILD_DELETE).is_empty());
        // A count that reaches zero on the first drop deletes there.
        let (mut e, node, child) = attach_engine(0, 3, -1);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CHILD_DELETE), vec![vec![child]]);
        assert_eq!(e.mem.i32(child + 4), -1);
    }

    #[test]
    fn ni_node_attach_child_first_available_and_null() {
        let (mut e, node, child) = attach_engine(3, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 1u32]);
        let log = e.call_log.take().unwrap();
        let add = calls_to(&log, CHILD_ARRAY_ADD_FIRST_EMPTY);
        assert_eq!(add.len(), 1);
        assert_eq!(add[0][0], node + 0x9c);
        assert!(calls_to(&log, CHILD_ARRAY_SET_AT).is_empty());
        assert!(calls_to(&log, CHILD_ARRAY_SET_SIZE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, 0u32, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use crate::types::NiTArray;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 1u32.into_ret());
        e.register(POINTER_ARRAY_SET_CAPACITY, |_, _| Ret::default());
        e.register(POINTER_ARRAY_SET_AT, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00a5eb20_fills_the_first_empty_slot_or_appends() {
        let mut e = engine();
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(16);
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, 4);
        e.set(array, NiTArray::m_usSize, 3);
        e.set(array, NiTArray::m_usESize, 2);
        e.set(array, NiTArray::m_usGrowBy, 5);
        e.mem.set_u32(base, 0x111);
        e.mem.set_u32(base + 8, 0x333);
        let object = e.mem.alloc(8);
        let element = e.mem.alloc(4);
        // A null pointer is refused.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a5_eb20, &args![array, element]).u32(),
            0xffff_ffff
        );
        // The empty slot 1 takes it and counts it.
        e.mem.set_u32(element, object);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 1);
        assert_eq!(e.mem.u32(base + 4), object);
        assert_eq!(e.get(array, NiTArray::m_usESize), 3);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT), vec![vec![object + 4]]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        // Full but below capacity: appended at index 3 without growing.
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 3);
        assert!(calls_to(&e, POINTER_ARRAY_SET_CAPACITY).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_ARRAY_SET_AT),
            vec![vec![array.addr(), 3, element]]
        );
        // At capacity: grows by m_usGrowBy first.
        e.set(array, NiTArray::m_usSize, 4);
        e.mem.set_u32(base + 12, 0x444);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 4);
        assert_eq!(
            calls_to(&e, POINTER_ARRAY_SET_CAPACITY),
            vec![vec![array.addr(), 9]]
        );
        // An empty array appends at 0 (the scan does not run).
        e.set(array, NiTArray::m_usSize, 0);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 0);
    }
}
