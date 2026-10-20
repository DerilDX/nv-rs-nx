//! `(unplaced)/run_00b631d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The per-object call `00c4edf0` (`thiscall (object, &local, flag)`,
/// `RET 8`), the object destructor `00c4ed40` (`thiscall`), and the memory
/// manager (`011f6238`) with its free (`00aa4060`, `thiscall`, one word).
const RELEASE_OBJECT: u32 = 0x00c4_edf0;
const OBJECT_DESTRUCTOR: u32 = 0x00c4_ed40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

/// One object slot's clean-up: `00c4edf0` on it (with the local and 1),
/// then, when the slot still holds an object, its destructor and its block
/// freed; the slot cleared.
fn clear_slot(e: &mut Engine, slot: u32, local: u32) {
    let object = e.mem.u32(slot);
    e.call(RELEASE_OBJECT, &args![object, local, 1u32]);
    let object = e.mem.u32(slot);
    if object != 0 {
        e.call(OBJECT_DESTRUCTOR, &args![object]);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, object]);
    }
    e.mem.set_u32(slot, 0);
}

// Translated from 00b631d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases every object this holds (the engine map has no name): each
/// non-null entry of the array at +0x8C (16-bit count at +0x8A), the three
/// slots at +0x48 (0x14 bytes apart, each one's byte at +4 cleared too),
/// then, while the word at +0x148 is nonzero, the object at +0x14 of each
/// node's data walking the list from +0x140 (the exe re-reads only that
/// word, so the walk ends when a clean-up clears it). Each object gets
/// `00c4edf0` with the address of a stack word that starts as `this` (the
/// exe's saved `ECX` slot) and 1, then is destroyed and freed. The byte at
/// +0x40 is cleared, both 16-bit counts zeroed and the array freed.
pub fn fn_00b631d0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, object);
        if e.mem.u16(object + 0x8a) != 0 {
            let mut index = 0u32;
            loop {
                // The array pointer is read again before each access.
                let item = e.mem.u32(e.mem.u32(object + 0x8c).wrapping_add(4 * index));
                if item != 0 {
                    e.call(RELEASE_OBJECT, &args![item, local, 1u32]);
                    let item = e.mem.u32(e.mem.u32(object + 0x8c).wrapping_add(4 * index));
                    if item != 0 {
                        e.call(OBJECT_DESTRUCTOR, &args![item]);
                        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, item]);
                    }
                    let array = e.mem.u32(object + 0x8c);
                    e.mem.set_u32(array.wrapping_add(4 * index), 0);
                }
                index = index.wrapping_add(1);
                if index as i32 >= i32::from(e.mem.u16(object + 0x8a)) {
                    break;
                }
            }
        }
        for i in 0..3 {
            let slot = object + 0x48 + 0x14 * i;
            if e.mem.u32(slot) != 0 {
                clear_slot(e, slot, local);
                e.mem.set_u8(slot + 4, 0);
            }
        }
        e.mem.set_u8(object + 0x40, 0);
        let mut node = e.mem.u32(object + 0x140);
        while e.mem.u32(object + 0x148) != 0 {
            let data = e.mem.u32(node.wrapping_add(8));
            node = e.mem.u32(node);
            if data != 0 && e.mem.u32(data.wrapping_add(0x14)) != 0 {
                clear_slot(e, data.wrapping_add(0x14), local);
            }
        }
    });
    let array = e.mem.u32(object + 0x8c);
    e.mem.set_u16(object + 0x8a, 0);
    e.mem.set_u16(object + 0x88, 0);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, array]);
    e.mem.set_u32(object + 0x8c, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_31d0, fn_00b631d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_held_object_is_released_destroyed_and_freed() {
        let mut e = Engine::new();
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x150);
        // The list's clean-up clears the word at +0x148, which ends the walk.
        e.register_double(RELEASE_OBJECT, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), this);
            assert_eq!(a[2], 1);
            if a[0] == 0x6003 {
                e.mem.set_u32(this + 0x148, 0);
            }
            Ret::default()
        });
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 0x6001);
        e.mem.set_u32(this + 0x8c, array);
        e.mem.set_u16(this + 0x8a, 2);
        e.mem.set_u16(this + 0x88, 4);
        e.mem.set_u32(this + 0x48 + 0x14, 0x6002);
        e.mem.set_u8(this + 0x48 + 0x14 + 4, 1);
        e.mem.set_u8(this + 0x40, 3);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 0x14, 0x6003);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, data);
        e.mem.set_u32(this + 0x140, node);
        e.mem.set_u32(this + 0x148, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_31d0, &args![this]);
        let log = e.call_log.take().unwrap();
        let released: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == RELEASE_OBJECT)
            .map(|call| call.1[0])
            .collect();
        assert_eq!(released, vec![0x6001, 0x6002, 0x6003]);
        let freed: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == MEMORY_MANAGER_FREE)
            .map(|call| call.1[1])
            .collect();
        assert_eq!(freed, vec![0x6001, 0x6002, 0x6003, array]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(this + 0x48 + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x48 + 0x14 + 4), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        assert_eq!(e.mem.u16(this + 0x8a), 0);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
    }
}
