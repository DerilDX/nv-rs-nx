//! `NiParticle/niparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// `NiParticleSystem::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_24e0;

// Translated from 00c1b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_system_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_SYSTEM_RTTI)
}

/// `00a80680`: the jump target of `00c1ad80`.
const JUMP_TARGET_00C1AD80: u32 = 0x00a8_0680;

// Translated from 00c1ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::SetModelData` (Xbox PDB): a 5-byte jump to `00a80680` (engine map: `NiGeometry::SetModelData`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetModelData@NiParticleSystem@@UAAXPAVNiGeometryData@@@Z`.
pub fn ni_particle_system_set_model_data(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C1AD80, &args![this, arg]);
}

/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The free-node list the list items go back to: its head (`011c5f58`), the two words after
/// the call `0040fbf0` is made on (`011c5f80`, cleared when the counter at `011c5f84`
/// reaches 0) and that counter, decremented once per node.
const FREE_NODES_011C5F58: u32 = 0x011c_5f58;
const FREE_LIST_STATE_011C5F80: u32 = 0x011c_5f80;
const COUNTER_011C5F84: u32 = 0x011c_5f84;
/// `0040fbf0` (unnamed): `thiscall` on `011c5f80`, `RET 4`.
const CALL_0040FBF0: u32 = 0x0040_fbf0;

// Translated from 00e74d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiPSysModifier>>::RemoveAll`
/// (Xbox PDB). `thiscall`, no stack words. Walks the list from its head; for each item: the
/// `NiPointer` element is released (reference count at +4 decremented, deleted through vtable
/// slot +4 when it reaches 0) and cleared when not null; `0040fbf0` runs on `011c5f80` with 0;
/// the item's previous link is cleared and the item is pushed onto the free list at `011c5f58`
/// (its next link takes the old head); the counter at `011c5f84` is decremented and, when it
/// reaches 0, the word at `011c5f80` is cleared. Then the count, head and tail are cleared.
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut item = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkHead));
    while !item.is_null() {
        let element = e.get(item, NiTListItem::m_element);
        let next = e.get(item, NiTListItem::m_pkNext);
        if element != 0 {
            let left = e
                .call(
                    IMPORT_INTERLOCKED_DECREMENT,
                    &args![element.wrapping_add(4)],
                )
                .u32();
            if left == 0 {
                e.vcall(element, 4, &args![]);
            }
            e.set(item, NiTListItem::m_element, 0);
        }
        e.call(CALL_0040FBF0, &args![FREE_LIST_STATE_011C5F80, 0u32]);
        e.set(item, NiTListItem::m_pkPrev, 0);
        let free = e.global::<u32>(FREE_NODES_011C5F58);
        e.set(item, NiTListItem::m_pkNext, free);
        let counter = e.global::<u32>(COUNTER_011C5F84).wrapping_sub(1);
        e.set_global(COUNTER_011C5F84, counter);
        e.set_global(FREE_NODES_011C5F58, item.addr());
        if counter == 0 {
            e.set_global(FREE_LIST_STATE_011C5F80, 0u32);
        }
        item = Ptr::new(next);
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1b5b0, ni_particle_system_get_rtti() -> Ptr),
        entry!(0x00c1_ad80, ni_particle_system_set_model_data(Ptr, Ptr)),
        entry!(
            0x00e7_4d40,
            ni_t_pointer_list_base_remove_all(Ptr<NiTPointerList>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_system_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1b5b0, &args![]).u32(), NI_PARTICLE_SYSTEM_RTTI);
    }

    #[test]
    fn thunk_00c1ad80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c1_ad80;
        const TARGET: u32 = 0x00a8_0680;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    /// A list of three items: an element held once (deleted), a null element, and an
    /// element held twice (kept). The free list starts with one node and the counter at 2.
    #[test]
    fn remove_all_releases_the_elements_and_frees_every_item() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        let free_before = 0x0bad_0000u32;
        e.set_global(FREE_NODES_011C5F58, free_before);
        e.set_global(FREE_LIST_STATE_011C5F80, 0x77u32);
        e.set_global(COUNTER_011C5F84, 2u32);
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        let elements: Vec<u32> = (0..2).map(|_| e.mem.alloc(8)).collect();
        for (element, count) in elements.iter().zip([1u32, 2]) {
            e.mem.set_u32(*element, 0x0f00_1000);
            e.mem.set_u32(element + 4, count);
        }
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        for (i, item) in items.iter().enumerate() {
            let next = items.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(*item, next);
            e.mem.set_u32(item + 4, 0x5555);
        }
        e.mem.set_u32(items[0] + 8, elements[0]);
        e.mem.set_u32(items[1] + 8, 0);
        e.mem.set_u32(items[2] + 8, elements[1]);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[2]);
        e.mem.set_u32(list + 8, 3);

        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.register_double(CALL_0040FBF0, |e, a| {
            assert_eq!(a, &[FREE_LIST_STATE_011C5F80, 0][..]);
            // The word is cleared once the counter has reached 0 (after the second item).
            let counter = e.global::<u32>(COUNTER_011C5F84);
            let word = e.global::<u32>(FREE_LIST_STATE_011C5F80);
            assert_eq!(word == 0, counter == 0, "counter {counter:#x}");
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_4d40, &args![list]);

        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == 0x0f00_2000).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![elements[0]]);
        assert_eq!(log.iter().filter(|c| c.0 == CALL_0040FBF0).count(), 3);
        assert_eq!(e.mem.u32(elements[1] + 4), 1);
        // Items pushed onto the free list in order: the last one is the new head.
        assert_eq!(e.global::<u32>(FREE_NODES_011C5F58), items[2]);
        assert_eq!(e.mem.u32(items[2]), items[1]);
        assert_eq!(e.mem.u32(items[1]), items[0]);
        assert_eq!(e.mem.u32(items[0]), free_before);
        for item in &items {
            assert_eq!(e.mem.u32(item + 4), 0);
            assert_eq!(e.mem.u32(item + 8), 0);
        }
        // 2 - 3 wraps; the word was cleared when the counter reached 0 (second item).
        assert_eq!(e.global::<u32>(COUNTER_011C5F84), 0xffff_ffff);
        assert_eq!(e.global::<u32>(FREE_LIST_STATE_011C5F80), 0);
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
    }

    #[test]
    fn remove_all_on_an_empty_list_only_clears_it() {
        let mut e = Engine::new();
        let list = e.mem.alloc(12);
        e.mem.set_u32(list + 4, 0x1234);
        e.mem.set_u32(list + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4d40, &args![list]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
    }
}
