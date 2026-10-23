//! `BSShader/run_00b57e70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::{NiTListItem, NiTPointerList};

/// The list-item allocation `0043a010` (`thiscall` on the list's allocator at +8, no stack
/// words; answers the new item).
const LIST_NEW_ITEM_0043A010: u32 = 0x0043_a010;

// Translated from 00b58570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b57e70`); `thiscall` on an `NiTPointerList` with one stack
/// word, the address of the element (a reference): adds the element at the tail. A new item
/// from the allocator at +8 (`0043a010`) gets the element, no next item and the old tail as
/// its previous one; the old tail (if any) links to it, the count at +8 goes up by one and
/// the item becomes the tail (and also the head when the list was empty).
pub fn fn_00b58570(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) {
    let item: Ptr<NiTListItem> = e
        .call(LIST_NEW_ITEM_0043A010, &args![this.addr() + 8])
        .ptr();
    let value = e.mem.u32(element.addr());
    e.set(item, NiTListItem::m_element, value);
    e.set(item, NiTListItem::m_pkNext, 0);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    e.set(item, NiTListItem::m_pkPrev, tail);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    if tail != 0 {
        e.mem.set_u32(tail, item.addr());
        e.set(this, NiTPointerList::m_uiCount, count);
    } else {
        e.set(this, NiTPointerList::m_uiCount, count);
        e.set(this, NiTPointerList::m_pkHead, item.addr());
    }
    e.set(this, NiTPointerList::m_pkTail, item.addr());
}

use crate::types::NiTPointerMap;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The map's virtual slots (byte offsets) the code calls: the hash of a key (+4), key
/// equality (+8), filling a new item with key and value (+0xc) and a new item (+0x14).
const SLOT_KEY_TO_HASH_INDEX: u32 = 4;
const SLOT_IS_KEYS_EQUAL: u32 = 8;
const SLOT_SET_VALUE: u32 = 0xc;
const SLOT_NEW_ITEM: u32 = 0x14;
/// The object at `011f9644` whose word at `+0x30` `fn_00b58070` changes.
const OBJECT_011F9644: u32 = 0x011f_9644;
/// The map clean-up `00b601a0` (`thiscall`) and the free `00aa10f0` (`cdecl`, one word).
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The vtables of the map destructor `fn_00b580a0`: its own (`010adbac`) and its base's
/// (`010ade34`).
const MAP_VTABLE_010ADBAC: u32 = 0x010a_dbac;
const MAP_VTABLE_010ADE34: u32 = 0x010a_de34;
/// The allocation `00aa1070` (`cdecl`, one word), `_memset` (`00ec61c0`) and `_atexit`
/// (`00ec658f`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const ATEXIT: u32 = 0x00ec_658f;

/// One of the two function-local static maps: its guard word, the map, the vtable during
/// construction and after it, and the clean-up given to `_atexit`.
struct StaticMap {
    guard: u32,
    map: u32,
    first_vtable: u32,
    vtable: u32,
    at_exit: u32,
}

const STATIC_MAP_00B585C0: StaticMap = StaticMap {
    guard: 0x011f_966c,
    map: 0x011f_965c,
    first_vtable: 0x010a_de18,
    vtable: 0x010a_f6b0,
    at_exit: 0x00fd_cc40,
};

const STATIC_MAP_00B58670: StaticMap = StaticMap {
    guard: 0x011f_9680,
    map: 0x011f_9670,
    first_vtable: MAP_VTABLE_010ADE34,
    vtable: MAP_VTABLE_010ADBAC,
    at_exit: 0x00fd_cc30,
};

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b57f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b57e70`): an `NiTMapBase` iteration step (`thiscall`,
/// three words: the position, the key and value outputs). Writes the item's key (`+4`) and
/// value (`+8`) out and moves the position to the item's next one (`+0`); at the end of a
/// bucket, to the first item of the next non-empty bucket after the item's (slot +4 gives
/// the bucket of its key), or to null.
pub fn fn_00b57f20(e: &mut Engine, this: Ptr<NiTPointerMap>, position: u32, key: u32, value: u32) {
    let item = e.mem.u32(position);
    let item_key = e.mem.u32(item.wrapping_add(4));
    e.mem.set_u32(key, item_key);
    let item_value = e.mem.u32(item.wrapping_add(8));
    e.mem.set_u32(value, item_value);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position, next);
        return;
    }
    let item_key = e.mem.u32(item.wrapping_add(4));
    let mut index = e
        .vcall(this.addr(), SLOT_KEY_TO_HASH_INDEX, &args![item_key])
        .u32()
        .wrapping_add(1);
    let size = e.get(this, NiTPointerMap::m_uiHashSize);
    if index < size {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        loop {
            let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
            if head != 0 {
                e.mem.set_u32(position, head);
                return;
            }
            index = index.wrapping_add(1);
            if index >= size {
                break;
            }
        }
    }
    e.mem.set_u32(position, 0);
}

// Translated from 00b57fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers 0 (`XOR EAX,EAX; RET 0xc`). A vtable entry (slot +0xb0
/// of the shader-property vtables, `ClarifyShader` in the Xbox PDB's slot list): `thiscall`
/// with three stack words it does not read.
pub fn fn_00b57fc0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

// Translated from 00b58070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `cdecl`, one byte. Sets (non-zero) or clears bit 0 of the
/// word at `+0x30` of the object at `011f9644`, answering the bit as it was.
pub fn fn_00b58070(e: &mut Engine, on: u8) -> u8 {
    let object = e.mem.u32(OBJECT_011F9644);
    let word = object.wrapping_add(0x30);
    let was = e.mem.u8(word) & 1;
    let flags = e.mem.u32(word);
    if on != 0 {
        e.mem.set_u32(word, flags | 1);
    } else {
        e.mem.set_u32(word, flags & 0xffff_fffe);
    }
    was
}

// Translated from 00b580a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiTMap` destructor (`thiscall`). With its own vtable
/// (`010adbac`) and then its base's (`010ade34`) set, the clean-up `00b601a0` runs once each;
/// then the bucket array (`+8`) is freed (`00aa10f0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b580a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b580a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), MAP_VTABLE_010ADBAC);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![this]);
    e.mem.set_u32(this.addr(), MAP_VTABLE_010ADE34);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_00AA10F0, &args![table]);
}

// Translated from 00b582b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiTMap<K, NiPointer<V>>::SetAt` (`thiscall`, two words: the
/// key and the value, an `NiPointer` passed by value). An item with an equal key (slot +8)
/// in the key's bucket (slot +4) gets the value (the `NiPointer` assignment); otherwise a new
/// item (slot +0x14) is filled (slot +0xc with the item, the key and a counted copy of the
/// value), put at the head of the bucket, and the item count (`+0xc`) goes up. Either way the
/// by-value argument is released at the end.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b582b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b582b0(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, SLOT_KEY_TO_HASH_INDEX, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(map, SLOT_IS_KEYS_EQUAL, &args![key, item_key])
            .bool()
        {
            let old = e.mem.u32(item.wrapping_add(8));
            if old != value {
                if old != 0 {
                    release_reference(e, old);
                }
                e.mem.set_u32(item.wrapping_add(8), value);
                if value != 0 {
                    e.call(INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
                }
            }
            if value != 0 {
                release_reference(e, value);
            }
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(map, SLOT_NEW_ITEM, &args![]).u32();
    if value != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
    }
    e.vcall(map, SLOT_SET_VALUE, &args![item, key, value]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(item, head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
    if value != 0 {
        release_reference(e, value);
    }
}

/// The function-local static map's first use: the guard bit set, the vtable before the
/// buckets, 0x25 buckets, no items, the bucket array allocated (`00aa1070`, 0x94 bytes) and
/// cleared (`_memset` with the bucket count times 4), the final vtable, and the clean-up
/// given to `_atexit`. Answers the map.
fn static_map(e: &mut Engine, map: &StaticMap) -> u32 {
    if e.mem.u32(map.guard) & 1 == 0 {
        let guard = e.mem.u32(map.guard);
        e.mem.set_u32(map.guard, guard | 1);
        e.mem.set_u32(map.map, map.first_vtable);
        e.mem.set_u32(map.map + 4, 0x25);
        e.mem.set_u32(map.map + 0xc, 0);
        let table = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        let size = e.mem.u32(map.map + 4).wrapping_mul(4);
        e.mem.set_u32(map.map + 8, table);
        e.call(MEMSET, &args![table, 0u32, size]);
        e.mem.set_u32(map.map, map.vtable);
        e.call(ATEXIT, &args![map.at_exit]);
    }
    map.map
}

// Translated from 00b585c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the function-local static map at `011f965c` (guard `011f966c`,
/// vtables `010ade18` then `010af6b0`, `_atexit` clean-up `00fdcc40`), made on first use;
/// answers its address.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b585c0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b585c0(e: &mut Engine) -> u32 {
    static_map(e, &STATIC_MAP_00B585C0)
}

// Translated from 00b58670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the function-local static map at `011f9670` (guard `011f9680`,
/// vtables `010ade34` then `010adbac`, `_atexit` clean-up `00fdcc30`), made on first use;
/// answers its address.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b58670 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b58670(e: &mut Engine) -> u32 {
    static_map(e, &STATIC_MAP_00B58670)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_8570, fn_00b58570(Ptr<NiTPointerList>, Ptr)),
        entry!(0x00b5_7f20, fn_00b57f20(Ptr<NiTPointerMap>, u32, u32, u32)),
        entry!(0x00b5_7fc0, fn_00b57fc0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00b5_8070, fn_00b58070(u8) -> u8),
        entry!(0x00b5_80a0, fn_00b580a0(Ptr<NiTPointerMap>)),
        entry!(0x00b5_82b0, fn_00b582b0(Ptr<NiTPointerMap>, u32, u32)),
        entry!(0x00b5_85c0, fn_00b585c0() -> u32),
        entry!(0x00b5_8670, fn_00b58670() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NEW_ITEM_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        e
    }

    #[test]
    fn fn_00b58570_adds_the_first_item_as_head_and_tail() {
        let mut e = engine();
        let list = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0xabcd);
        e.call(0x00b5_8570, &args![list, element]);
        let item = e.mem.u32(list);
        assert_ne!(item, 0);
        assert_eq!(e.mem.u32(list + 4), item);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0xabcd);
    }

    #[test]
    fn fn_00b58570_links_after_the_tail() {
        let mut e = engine();
        let list = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 1);
        e.call(0x00b5_8570, &args![list, element]);
        let first = e.mem.u32(list);
        e.mem.set_u32(element, 2);
        e.call(0x00b5_8570, &args![list, element]);
        let second = e.mem.u32(list + 4);
        assert_eq!(e.mem.u32(list), first);
        assert_ne!(second, first);
        assert_eq!(e.mem.u32(first), second);
        assert_eq!(e.mem.u32(second + 4), first);
        assert_eq!(e.mem.u32(second + 8), 2);
        assert_eq!(e.mem.u32(list + 8), 2);
    }
}

#[cfg(test)]
mod map_tests {
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

    /// A map whose hash is the key modulo the bucket count, keys equal when the words are,
    /// new items from the heap and the set-value slot storing key and value; reference
    /// counting on +4, deletion through 0x0f00_0004.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = [0u32; 6];
        slots[1] = 0x0f00_0010;
        slots[2] = 0x0f00_0020;
        slots[3] = 0x0f00_0030;
        slots[5] = 0x0f00_0050;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0010, |e, a| (a[1] % e.mem.u32(a[0] + 4)).into_ret());
        e.register(0x0f00_0020, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0f00_0030, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x0f00_0050, |e, _| e.mem.alloc(0xc).into_ret());
        e.put_vtable(0x0f00_2000, &[0, 0x0f00_0004]);
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, 0x0f00_1000);
        e.mem.set_u32(map + 4, 4);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, table);
        e.call_log = Some(vec![]);
        (e, map)
    }

    fn counted(e: &mut Engine, references: u32) -> u32 {
        let o = e.mem.alloc(8);
        e.mem.set_u32(o, 0x0f00_2000);
        e.mem.set_u32(o + 4, references);
        o
    }

    #[test]
    fn get_next_walks_items_and_buckets() {
        let (mut e, map) = rig();
        let table = e.mem.u32(map + 8);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let third = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 1);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second + 4, 5);
        e.mem.set_u32(second + 8, 0x55);
        e.mem.set_u32(third + 4, 3);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(table + 12, third);
        let position = e.mem.alloc(4);
        let key = e.mem.alloc(4);
        let value = e.mem.alloc(4);
        e.mem.set_u32(position, first);
        e.call(0x00b5_7f20, &args![map, position, key, value]);
        assert_eq!(
            (e.mem.u32(position), e.mem.u32(key), e.mem.u32(value)),
            (second, 1, 0x11)
        );
        e.call(0x00b5_7f20, &args![map, position, key, value]);
        assert_eq!(
            (e.mem.u32(position), e.mem.u32(key), e.mem.u32(value)),
            (third, 5, 0x55)
        );
        e.call(0x00b5_7f20, &args![map, position, key, value]);
        assert_eq!(e.mem.u32(position), 0);
    }

    #[test]
    fn fn_00b57fc0_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b5_7fc0, &args![1u32, 2u32, 3u32, 4u32]).u32(), 0);
    }

    #[test]
    fn fn_00b58070_sets_or_clears_bit_0() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x30, 0x10);
        e.set_global(OBJECT_011F9644, object);
        assert_eq!(e.call(0x00b5_8070, &args![0x101u32]).u32() as u8, 0);
        assert_eq!(e.mem.u32(object + 0x30), 0x11);
        assert_eq!(e.call(0x00b5_8070, &args![0x100u32]).u32() as u8, 1);
        assert_eq!(e.mem.u32(object + 0x30), 0x10);
    }

    #[test]
    fn map_destructor_cleans_with_both_vtables() {
        let mut e = Engine::new();
        e.register_double(MAP_REMOVE_ALL_00B601A0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x9000);
        e.call(0x00b5_80a0, &args![map]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0).len(), 2);
        assert_eq!(e.mem.u32(map), MAP_VTABLE_010ADE34);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x9000]]);
    }

    #[test]
    fn set_at_adds_a_new_item_with_a_counted_copy() {
        let (mut e, map) = rig();
        let value = counted(&mut e, 1);
        e.call(0x00b5_82b0, &args![map, 6u32, value]);
        let table = e.mem.u32(map + 8);
        let item = e.mem.u32(table + 8);
        assert_ne!(item, 0);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (6, value));
        assert_eq!(e.mem.u32(map + 0xc), 1);
        // The copy's count stays with the item; the by-value argument is released.
        assert_eq!(e.mem.u32(value + 4), 1);
        assert_eq!(calls(&e, 0x0f00_0030), vec![vec![map, item, 6, value]]);
        // A second item in the same bucket goes in front.
        e.call(0x00b5_82b0, &args![map, 2u32, 0u32]);
        let head = e.mem.u32(table + 8);
        assert_eq!(e.mem.u32(head), item);
        assert_eq!(e.mem.u32(map + 0xc), 2);
    }

    #[test]
    fn set_at_replaces_the_value_of_an_equal_key() {
        let (mut e, map) = rig();
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 2);
        let table = e.mem.u32(map + 8);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, 7);
        e.mem.set_u32(item + 8, old);
        e.mem.set_u32(table + 12, item);
        e.call(0x00b5_82b0, &args![map, 7u32, new]);
        assert_eq!(e.mem.u32(item + 8), new);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert!(calls(&e, 0x0f00_0050).is_empty());
        assert_eq!(e.mem.u32(map + 0xc), 0);
        // The same value again: only the by-value release.
        e.call(0x00b5_82b0, &args![map, 7u32, new]);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    #[test]
    fn static_maps_are_made_once() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_85c0, &[]).u32(), 0x011f_965c);
        assert_eq!(e.call(0x00b5_85c0, &[]).u32(), 0x011f_965c);
        assert_eq!(e.mem.u32(0x011f_965c), 0x010a_f6b0);
        assert_eq!(e.mem.u32(0x011f_9660), 0x25);
        let table = e.mem.u32(0x011f_9664);
        assert_eq!(calls(&e, MEMSET), vec![vec![table, 0, 0x94]]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![0x00fd_cc40]]);
        assert_eq!(e.call(0x00b5_8670, &[]).u32(), 0x011f_9670);
        assert_eq!(e.mem.u32(0x011f_9670), MAP_VTABLE_010ADBAC);
        assert_eq!(calls(&e, ATEXIT)[1], vec![0x00fd_cc30]);
        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![0x94], vec![0x94]]);
    }
}
