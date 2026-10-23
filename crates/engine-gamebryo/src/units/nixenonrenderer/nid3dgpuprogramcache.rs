//! `NiXenonRenderer/nid3dgpuprogramcache.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two `NiTSet<NiPointer<NiMaterialResource>, NiTNewInterface<..>>` members this unit
//! emits read the set at its Xbox PDB offsets (the PC code uses the same): `m_pBase` +0,
//! `m_uiAlloced` +4, `m_uiUsed` +8 (0xc bytes).

#[allow(unused_imports)]
use crate::prelude::*;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `NiPointer` array allocator `00c1d790` (unit `nipsysmeshemitter.cpp`, unnamed; `cdecl`,
/// the element count: an `operator new[]` block of constructed null pointers) and its
/// release `00a47600` (unnamed; `cdecl`, the array: releases every element and frees the
/// block).
const POINTER_ARRAY_NEW_00C1D790: u32 = 0x00c1_d790;
const POINTER_ARRAY_DELETE_00A47600: u32 = 0x00a4_7600;

/// `NiPointer<T>::operator=` from another pointer at `source`, as these two functions expand
/// it: nothing when the slot already holds what `source` holds; else the slot's object is
/// released (deleted through its vtable slot +4 when its count at +4 reaches 0), `source` is
/// read again, stored and its object counted when not null.
fn assign_from(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00e7cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `NiGPUProgramCache::IsNode`; the body is `XOR EAX,EAX; RET` (it
/// reads no register or stack word and answers 0), a body the linker shares with other
/// classes' methods, so the name is not checked. No parameters: callers pass `this` or
/// nothing, and neither is read.
pub fn fn_00e7cd40(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00e8e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTSet<NiPointer<NiMaterialResource>, ..>` reallocation (the engine map has no name;
/// it is the set's `Realloc`, called by `Add` `00e8ea60`), `thiscall`, one word: nothing when
/// `new_max` is `m_uiAlloced`. Else a new array of `new_max` pointers (`00c1d790`; none for 0)
/// takes the first `m_uiUsed` elements (each assigned, so counted once more; with no new array
/// nothing is copied), the old array is released (`00a47600`), and the set keeps the new array
/// and `new_max`. `m_uiUsed` is not changed.
pub fn ni_t_set_material_resource_realloc(e: &mut Engine, this: Ptr, new_max: u32) {
    let set = this.addr();
    if new_max == e.mem.u32(set + 4) {
        return;
    }
    let base = if new_max != 0 {
        let base = e.call(POINTER_ARRAY_NEW_00C1D790, &args![new_max]).u32();
        let mut index = 0u32;
        while index < e.mem.u32(set + 8) {
            let old = e.mem.u32(set).wrapping_add(index.wrapping_mul(4));
            assign_from(e, base.wrapping_add(index.wrapping_mul(4)), old);
            index += 1;
        }
        base
    } else {
        0
    };
    let old = e.mem.u32(set);
    e.call(POINTER_ARRAY_DELETE_00A47600, &args![old]);
    e.mem.set_u32(set, base);
    e.mem.set_u32(set + 4, new_max);
}

// Translated from 00e8ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiPointer<NiMaterialResource>, NiTNewInterface<..>>::Add` (Xbox PDB), `thiscall`, one
/// word (the address of the pointer to add): a full set grows first (`00e8e3a0` with twice
/// `m_uiAlloced`, or 1 when it is 0); then the element at `m_uiUsed` (the count goes up first)
/// is assigned from the pointer.
pub fn ni_t_set_material_resource_add(e: &mut Engine, this: Ptr, element: u32) {
    let set = this.addr();
    let alloced = e.mem.u32(set + 4);
    if e.mem.u32(set + 8) == alloced {
        let grown = if alloced != 0 {
            alloced.wrapping_add(alloced)
        } else {
            1
        };
        ni_t_set_material_resource_realloc(e, this, grown);
    }
    let used = e.mem.u32(set + 8);
    let slot = e.mem.u32(set).wrapping_add(used.wrapping_mul(4));
    e.mem.set_u32(set + 8, used.wrapping_add(1));
    assign_from(e, slot, element);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_cd40, fn_00e7cd40() -> u32),
        entry!(0x00e8_e3a0, ni_t_set_material_resource_realloc(Ptr, u32)),
        entry!(0x00e8_ea60, ni_t_set_material_resource_add(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0e40_0000;
    const DELETE: u32 = 0x0e41_0000;

    /// Interlocked counts on memory, the array allocator handing out zeroed blocks (after a
    /// count word, as `operator new[]`) and a recording delete.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(POINTER_ARRAY_NEW_00C1D790, |e, a| {
            let block = e.mem.alloc(4 + 4 * a[0]);
            e.mem.set_u32(block, a[0]);
            (block + 4).into_ret()
        });
        e.register(POINTER_ARRAY_DELETE_00A47600, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A set of `alloced` slots holding `objects`.
    fn make_set(e: &mut Engine, alloced: u32, objects: &[u32]) -> u32 {
        let base = e.mem.alloc(4 * alloced.max(1));
        for (index, object) in objects.iter().enumerate() {
            e.mem.set_u32(base + 4 * index as u32, *object);
        }
        let set = e.mem.alloc(0xc);
        e.mem.set_u32(set, base);
        e.mem.set_u32(set + 4, alloced);
        e.mem.set_u32(set + 8, objects.len() as u32);
        set
    }

    #[test]
    fn fn_00e7cd40_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e7_cd40, &args![]).u32(), 0);
        assert_eq!(e.call(0x00e7_cd40, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn fn_00e8e3a0_moves_the_elements_to_a_new_array() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        let set = make_set(&mut e, 2, &[first, second]);
        let old = e.mem.u32(set);
        e.call(0x00e8_e3a0, &args![set, 5u32]);
        let base = e.mem.u32(set);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![5]]);
        assert_eq!(calls(&e, POINTER_ARRAY_DELETE_00A47600), vec![vec![old]]);
        assert_eq!(e.mem.u32(base), first);
        assert_eq!(e.mem.u32(base + 4), second);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(first + 4), 2);
        assert_eq!(e.mem.u32(set + 4), 5);
        assert_eq!(e.mem.u32(set + 8), 2);
    }

    #[test]
    fn fn_00e8e3a0_to_zero_frees_the_array_and_same_size_does_nothing() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let set = make_set(&mut e, 2, &[first]);
        e.call(0x00e8_e3a0, &args![set, 2u32]);
        assert!(calls(&e, POINTER_ARRAY_DELETE_00A47600).is_empty());
        let old = e.mem.u32(set);
        e.call(0x00e8_e3a0, &args![set, 0u32]);
        assert!(calls(&e, POINTER_ARRAY_NEW_00C1D790).is_empty());
        assert_eq!(calls(&e, POINTER_ARRAY_DELETE_00A47600), vec![vec![old]]);
        assert_eq!(e.mem.u32(set), 0);
        assert_eq!(e.mem.u32(set + 4), 0);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert_eq!(e.mem.u32(first + 4), 1);
    }

    #[test]
    fn fn_00e8ea60_appends_and_grows_when_full() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let set = make_set(&mut e, 1, &[first]);
        let added = object(&mut e, 0);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, added);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![2]]);
        let base = e.mem.u32(set);
        assert_eq!(e.mem.u32(base), first);
        assert_eq!(e.mem.u32(base + 4), added);
        assert_eq!(e.mem.u32(added + 4), 1);
        assert_eq!(e.mem.u32(set + 4), 2);
        assert_eq!(e.mem.u32(set + 8), 2);
    }

    #[test]
    fn fn_00e8ea60_starts_an_empty_set_at_one_and_adds_null() {
        let mut e = engine();
        let set = make_set(&mut e, 0, &[]);
        let pointer = e.mem.alloc(4);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![1]]);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // Room left: no growth, the slot's old object released.
        let stale = object(&mut e, 1);
        let set = make_set(&mut e, 4, &[]);
        let base = e.mem.u32(set);
        e.mem.set_u32(base, stale);
        let added = object(&mut e, 0);
        e.mem.set_u32(pointer, added);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790).len(), 1);
        assert_eq!(calls(&e, DELETE), vec![vec![stale]]);
        assert_eq!(e.mem.u32(base), added);
    }
}
