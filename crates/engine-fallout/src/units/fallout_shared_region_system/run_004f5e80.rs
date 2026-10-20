//! `fallout shared/region system/run_004f5e80` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack word): the address of the list's head item word.
const HEAD_ITEM_SLOT: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`, no stack word): removes the list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;

// Translated from 004f5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): empties the list at +4. While its
/// head item (`006815c0`) is not null, removes the head (`0063f7b0`) and, when the byte at
/// +0xc is set, calls the item's vtable slot +8 with 1. Then clears the word at +0x10.
pub fn fn_004f5ef0(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    let slot = e.call(HEAD_ITEM_SLOT, &args![list]).u32();
    let mut item = e.mem.u32(slot);
    while item != 0 {
        e.call(LIST_REMOVE_HEAD, &args![list]);
        if e.mem.u8(this.addr() + 0xc) != 0 && item != 0 {
            e.vcall(item, 8, &args![1u32]);
        }
        let slot = e.call(HEAD_ITEM_SLOT, &args![list]).u32();
        item = e.mem.u32(slot);
    }
    e.mem.set_u32(this.addr() + 0x10, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_5ef0, fn_004f5ef0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn run(owns: u8) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let items = [e.mem.alloc(4), e.mem.alloc(4)];
        for item in items {
            e.mem.set_u32(item, 0x0150_0000);
        }
        let slot = e.mem.alloc(4);
        let left = Rc::new(Cell::new(2usize));
        let (peek, remove) = (left.clone(), left.clone());
        e.register_double(HEAD_ITEM_SLOT, move |e, _| {
            let item = if peek.get() == 0 {
                0
            } else {
                items[2 - peek.get()]
            };
            e.mem.set_u32(slot, item);
            slot.into_ret()
        });
        e.register_double(LIST_REMOVE_HEAD, move |_, _| {
            remove.set(remove.get() - 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xc, owns);
        e.mem.set_u32(this + 0x10, 9);
        e.call_log = Some(vec![]);
        e.call(0x004f_5ef0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(left.get(), 0);
        let log = e.call_log.take().unwrap();
        let mut deleted: Vec<_> = log.into_iter().filter(|c| c.0 == 0x0150_1000).collect();
        for call in &mut deleted {
            let item = call.1[0];
            assert!(items.contains(&item));
            call.1[0] = 0;
        }
        deleted
    }

    #[test]
    fn empties_the_list_deleting_items_only_when_owned() {
        assert_eq!(
            run(1),
            vec![(0x0150_1000, vec![0, 1]), (0x0150_1000, vec![0, 1])]
        );
        assert!(run(0).is_empty());
    }
}
