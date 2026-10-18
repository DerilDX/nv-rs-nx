//! `BSMain/nibsbonelodcontroller.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTPrimitiveSet;

/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;

// Translated from 00a311e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiNode*,NiTMallocInterface<NiNode*>>::Realloc` (engine map; `thiscall`,
/// one stack word): changes the capacity of the set of pointers (layout
/// [`NiTPrimitiveSet`]: data, capacity, used count) to `new_capacity`. Does
/// nothing for the current capacity. Otherwise new storage of `new_capacity`
/// pointers (`00aa1070`; none for 0) takes the used elements, the old storage is
/// freed (`00aa10f0`, also when it is null) and the capacity is stored.
pub fn ni_t_set_realloc(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, new_capacity: u32) {
    if new_capacity == e.get(this, NiTPrimitiveSet::m_uiAlloced) {
        return;
    }
    let storage = if new_capacity == 0 {
        0
    } else {
        let storage = e.call(ALLOCATE, &args![new_capacity.wrapping_mul(4)]).u32();
        let old = e.get(this, NiTPrimitiveSet::m_pBase);
        let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
        for index in 0..used {
            let value = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(storage.wrapping_add(index.wrapping_mul(4)), value);
        }
        storage
    };
    let old = e.get(this, NiTPrimitiveSet::m_pBase);
    e.call(FREE, &args![old]);
    e.set(this, NiTPrimitiveSet::m_pBase, storage);
    e.set(this, NiTPrimitiveSet::m_uiAlloced, new_capacity);
}

// Translated from 00c53e00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031b8 without reading the receiver. The map names this body `NiBSBoneLODController::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c53e00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31b8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_3e00, fn_00c53e00(Ptr) -> u32),
        entry!(0x00a3_11e0, ni_t_set_realloc(Ptr<NiTPrimitiveSet>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(capacity: u32, items: &[u32]) -> (Engine, Ptr<NiTPrimitiveSet>, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let set = e.new_object::<NiTPrimitiveSet>();
        let base = e.mem.alloc(capacity * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(set, NiTPrimitiveSet::m_pBase, base);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, capacity);
        e.set(set, NiTPrimitiveSet::m_uiUsed, items.len() as u32);
        (e, set, base)
    }

    #[test]
    fn the_current_capacity_changes_nothing() {
        let (mut e, set, base) = fixture(4, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 4u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), base);
    }

    #[test]
    fn a_new_capacity_copies_the_used_elements_and_frees_the_old_block() {
        let (mut e, set, base) = fixture(2, &[0x11, 0x22]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 8u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![32u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 8);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiUsed), 2);
        let new = e.get(set, NiTPrimitiveSet::m_pBase);
        assert_ne!(new, base);
        assert_eq!([e.mem.u32(new), e.mem.u32(new + 4)], [0x11, 0x22]);
    }

    #[test]
    fn capacity_zero_leaves_a_null_pointer() {
        let (mut e, set, base) = fixture(2, &[0x11]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), 0);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c53e00_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_3e00, &args![0x1000u32]).u32(), 0x0120_31b8);
        assert_eq!(e.call(0x00c5_3e00, &args![0u32]).u32(), 0x0120_31b8);
    }
}
