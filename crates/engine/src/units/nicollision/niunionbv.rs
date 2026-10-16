//! `NiCollision/niunionbv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `00a90180`: `thiscall(array, new capacity)`, the array `SetSize`.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `00ba3aa0`: `thiscall(array, index, pointer to the value)`, the array `SetAt`.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;

// Translated from 00a64330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Add` body of an `NiTArray`-shaped array of pointers (layout [`NiTArray`]; `thiscall`,
/// one stack word). The engine map names this address
/// `NiUnionBV::AddBoundingVolume`, which is the template instance that identical
/// code folding put the name on: the body works on the array itself (`this` is the
/// array, with its pointer at +4 and 16-bit counts from +8), not on an
/// `NiUnionBV`, whose array sits at +4, so it is named by its address.
///
/// `element` points at the value to add. A null value is refused (result
/// `0xffffffff`). Otherwise the first empty slot below the used size takes the value
/// (the non-null count goes up) and its index is returned. With no empty slot, the
/// index is the used size: when that equals or exceeds the capacity the array is
/// first grown by its grow-by amount (`SetSize` `00a90180`, capacity
/// `used size + grow-by`), then `SetAt` (`00ba3aa0`) stores the value there.
pub fn fn_00a64330(e: &mut Engine, this: Ptr<NiTArray>, element: Ptr) -> u32 {
    let value = e.mem.u32(element.addr());
    if value == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize);
    let mut index = 0u16;
    if size != 0 {
        loop {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add(index as u32 * 4);
            if e.mem.u32(slot) == 0 {
                e.mem.set_u32(slot, value);
                let present = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, present.wrapping_add(1));
                return index as u32;
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTArray::m_usSize) {
                break;
            }
        }
    }
    let used = e.get(this, NiTArray::m_usSize) as u32;
    if used >= e.get(this, NiTArray::m_usMaxSize) as u32 {
        let grow = e.get(this, NiTArray::m_usGrowBy) as u32;
        e.call(ARRAY_SET_SIZE, &args![this, grow.wrapping_add(used)]);
    }
    e.call(ARRAY_SET_AT, &args![this, used, element]);
    used
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_4330, fn_00a64330(Ptr<NiTArray>, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An array with room for `max` pointers holding `items`, grow-by 4.
    fn fixture(max: u16, items: &[u32]) -> (Engine, Ptr<NiTArray>, u32) {
        let mut e = Engine::new();
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(max as u32 * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTArray::m_usESize, present as u16);
        e.set(array, NiTArray::m_usGrowBy, 4);
        (e, array, base)
    }

    fn value(e: &mut Engine, word: u32) -> u32 {
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, word);
        holder
    }

    #[test]
    fn a_null_value_is_refused() {
        let (mut e, array, _) = fixture(4, &[1]);
        let holder = value(&mut e, 0);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), u32::MAX);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
    }

    #[test]
    fn the_first_empty_slot_takes_the_value() {
        let (mut e, array, base) = fixture(4, &[0x10, 0, 0x30]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.mem.u32(base + 4), 0x77);
        assert_eq!(e.get(array, NiTArray::m_usESize), 3);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
    }

    #[test]
    fn a_full_array_with_room_stores_at_the_used_size_through_set_at() {
        let (mut e, array, _) = fixture(4, &[0x10, 0x20]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ARRAY_SET_SIZE));
        let stores: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_AT).collect();
        assert_eq!(stores.len(), 1);
        assert_eq!(stores[0].1, args![array, 2u32, holder]);
    }

    #[test]
    fn a_full_array_without_room_grows_first() {
        let (mut e, array, _) = fixture(2, &[0x10, 0x20]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 2);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|call| call.0)
            .filter(|address| *address != 0x00a6_4330)
            .collect();
        assert_eq!(order, vec![ARRAY_SET_SIZE, ARRAY_SET_AT]);
        let grows: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_SIZE).collect();
        assert_eq!(grows[0].1, args![array, 6u32]);
    }

    #[test]
    fn an_empty_array_goes_straight_to_set_at() {
        let (mut e, array, _) = fixture(0, &[]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let grows: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_SIZE).collect();
        assert_eq!(grows[0].1, args![array, 4u32]);
    }
}
