//! `NiMain/nidevimageconverter.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

// Translated from 00a7b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7aff0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xa00 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_dev_image_converter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_aff0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xa00u32]);
    }
    this
}

/// `NiImageConverter`'s constructor (`00a76650`, engine map: `niimageconverter.cpp`;
/// `thiscall`, no stack words).
const IMAGE_CONVERTER_CONSTRUCT: u32 = 0x00a7_6650;
/// `NiDevImageConverter`'s virtual table (`0109d9c4`; its RTTI names `NiDevImageConverter`).
const NI_DEV_IMAGE_CONVERTER_VTABLE: u32 = 0x0109_d9c4;
/// `00a8bd20` (unnamed; `thiscall`, no stack words), run on the member at +0x780.
const MEMBER_780_CONSTRUCT: u32 = 0x00a8_bd20;
const MEMBER_780: u32 = 0x780;
/// The reader list (`NiTPointerList` at +0x990: head, tail, count).
const READERS: u32 = 0x990;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// Bytes of each reader the constructor makes.
const READER_SIZE: u32 = 0x180;
/// The three reader constructors, in the exe's order (`00a8e410` and `00a8d750` unnamed,
/// `00a8c190` in `niddsreader.cpp`; `thiscall`, no stack words).
const READER_CONSTRUCTORS: [u32; 3] = [0x00a8_e410, 0x00a8_d750, 0x00a8_c190];
/// `0043a010` (engine map: `NiTPointerListBase<..>::AddHead`; its body takes a cleared list
/// item from the item pool and answers it), `thiscall` on the list's count word.
const LIST_ITEM_NEW: u32 = 0x0043_a010;

// Translated from 00a7b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::NiDevImageConverter` (Xbox PDB), `thiscall`, no stack words: the
/// `NiImageConverter` constructor, the class's virtual table, the member at +0x780
/// (`00a8bd20`), an empty reader list at +0x990, then three readers of 0x180 bytes (from the
/// memory manager; a failed allocation puts a null reader) made by `00a8e410`, `00a8d750` and
/// `00a8c190` and each put at the head of the list (`0043a010` gives the item). Answers `this`.
pub fn ni_dev_image_converter_ni_dev_image_converter(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(IMAGE_CONVERTER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_DEV_IMAGE_CONVERTER_VTABLE);
    e.call(
        MEMBER_780_CONSTRUCT,
        &args![this.addr().wrapping_add(MEMBER_780)],
    );
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr().wrapping_add(READERS));
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    for constructor in READER_CONSTRUCTORS {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, READER_SIZE])
            .u32();
        let reader = if block != 0 {
            e.call(constructor, &args![block]).u32()
        } else {
            0
        };
        let count_word = list.addr().wrapping_add(8);
        let item: Ptr<NiTListItem> = e.call(LIST_ITEM_NEW, &args![count_word]).ptr();
        e.set(item, NiTListItem::m_element, reader);
        e.set(item, NiTListItem::m_pkPrev, 0);
        let head = e.get(list, NiTPointerList::m_pkHead);
        e.set(item, NiTListItem::m_pkNext, head);
        let head = e.get(list, NiTPointerList::m_pkHead);
        if head != 0 {
            e.set(
                Ptr::<NiTListItem>::new(head),
                NiTListItem::m_pkPrev,
                item.addr(),
            );
        } else {
            e.set(list, NiTPointerList::m_pkTail, item.addr());
        }
        let count = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(list, NiTPointerList::m_uiCount, count);
        e.set(list, NiTPointerList::m_pkHead, item.addr());
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7b260, ni_dev_image_converter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_b110,
            ni_dev_image_converter_ni_dev_image_converter(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dev_image_converter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_aff0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7b260, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7b260, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xa00]]);
        assert_eq!(calls_to(&e, 0x00a7_aff0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000s_tests {
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

    fn rig(failing_allocation: u32) -> Engine {
        let mut e = Engine::new();
        e.register(IMAGE_CONVERTER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMBER_780_CONSTRUCT, |_, a| a[0].into_ret());
        let mut allocations = 0;
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            allocations += 1;
            if allocations == failing_allocation {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[1]).into_ret()
            }
        });
        for constructor in READER_CONSTRUCTORS {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(LIST_ITEM_NEW, |e, _| {
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item, 0xdead);
            item.into_ret()
        });
        e
    }

    #[test]
    fn ni_dev_image_converter_puts_its_three_readers_at_the_head() {
        let mut e = rig(0);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this + READERS, 0x1111);
        assert_eq!(e.call(0x00a7_b110, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DEV_IMAGE_CONVERTER_VTABLE);
        assert_eq!(calls(&e, MEMBER_780_CONSTRUCT), vec![vec![this + 0x780]]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, READER_SIZE]; 3]
        );
        assert_eq!(calls(&e, LIST_ITEM_NEW), vec![vec![this + READERS + 8]; 3]);
        let readers: Vec<u32> = READER_CONSTRUCTORS
            .iter()
            .map(|c| calls(&e, *c)[0][0])
            .collect();
        // Head to tail: the last reader made first.
        let head = e.mem.u32(this + READERS);
        let tail = e.mem.u32(this + READERS + 4);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
        let mut item = head;
        let mut walked = vec![];
        let mut previous = 0;
        while item != 0 {
            assert_eq!(e.mem.u32(item + 4), previous);
            walked.push(e.mem.u32(item + 8));
            previous = item;
            item = e.mem.u32(item);
        }
        assert_eq!(previous, tail);
        assert_eq!(walked, vec![readers[2], readers[1], readers[0]]);
    }

    #[test]
    fn ni_dev_image_converter_keeps_a_null_reader_when_allocation_fails() {
        let mut e = rig(2);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.call(0x00a7_b110, &args![this]);
        assert!(calls(&e, READER_CONSTRUCTORS[1]).is_empty());
        let head = e.mem.u32(this + READERS);
        let middle = e.mem.u32(head);
        assert_eq!(e.mem.u32(middle + 8), 0);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
    }
}
