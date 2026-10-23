//! `NiXenonRenderer/run_00e6e8a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00e6e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e6e8a0`): a pointer list's item release, a virtual
/// `thiscall` with one stack word (the item, `RET 4`); `this` (the list) is not read. Clears
/// the item's element (+8), enters the item pool's lock (`0040fbf0` on `011c5f80` with 0),
/// clears the item's +4, pushes the item on the free list at `011c5f58` (its +0 takes the old
/// head) and leaves the lock: the count at `011c5f84` drops by one, and at zero the owner word
/// `011c5f80` is cleared.
pub fn fn_00e6e8a0(e: &mut Engine, _unused_this: Ptr, item: Ptr) {
    let item = item.addr();
    e.mem.set_u32(item.wrapping_add(8), 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(item.wrapping_add(4), 0);
    let head = e.mem.u32(NODE_POOL_FREE_LIST);
    e.mem.set_u32(item, head);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_e8a0, fn_00e6e8a0(Ptr, Ptr)),
        entry!(0x00e6_ec20, fn_00e6ec20()),
    ]
}

/// The COM object at `0126f0d8`, released through its vtable slot +8 (the object pushed).
const COM_OBJECT_0126F0D8: u32 = 0x0126_f0d8;
/// The 0x14-byte object at `0126f0e8`: destructor body `00e7bcd0` (`thiscall`, no stack
/// words), then the sized Ni free `00aa1460` (`cdecl`: block, size).
const OBJECT_0126F0E8: u32 = 0x0126_f0e8;
const OBJECT_DESTRUCT: u32 = 0x00e7_bcd0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The module handle at `0126f0d4`, given to the imported `FreeLibrary` (import slot
/// `00fdf0a8`) when not null; it is not cleared.
const MODULE_0126F0D4: u32 = 0x0126_f0d4;
const IMPORT_FREE_LIBRARY: u32 = 0x00fd_f0a8;
/// The word `0126f0dc`, cleared.
const CLEARED_0126F0DC: u32 = 0x0126_f0dc;
/// The pointer list at `0126f0f8` and its `RemoveAll` (engine map
/// `NiTPointerListBase<..., Script*>::RemoveAll`, `004ed900`, `thiscall`, no stack words),
/// reached by a jump.
const LIST_0126F0F8: u32 = 0x0126_f0f8;
const LIST_REMOVE_ALL: u32 = 0x004e_d900;

// Translated from 00e6ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; the last step of the shut-down function
/// `00e91c80`, reached by its jump): releases the COM object at `0126f0d8` (slot +8) and
/// clears the word, destroys and frees the 0x14-byte object at `0126f0e8` and clears the
/// word (both when not null), frees the library at `0126f0d4` when it is loaded, clears
/// `0126f0dc` and empties the list at `0126f0f8` (a jump to its `RemoveAll`).
pub fn fn_00e6ec20(e: &mut Engine) {
    let com = e.mem.u32(COM_OBJECT_0126F0D8);
    if com != 0 {
        e.vcall(com, 8, &args![]);
        e.mem.set_u32(COM_OBJECT_0126F0D8, 0);
    }
    let object = e.mem.u32(OBJECT_0126F0E8);
    if object != 0 {
        e.call(OBJECT_DESTRUCT, &args![object]);
        e.call(NI_DELETE, &args![object, 0x14u32]);
    }
    let module = e.mem.u32(MODULE_0126F0D4);
    e.mem.set_u32(OBJECT_0126F0E8, 0);
    if module != 0 {
        e.call(IMPORT_FREE_LIBRARY, &args![module]);
    }
    e.mem.set_u32(CLEARED_0126F0DC, 0);
    e.call(LIST_REMOVE_ALL, &args![LIST_0126F0F8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(count: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0e10_0000);
        e.mem.set_u32(NODE_POOL_LOCK, 7);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        // The lock enter double counts one more entry.
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e6e8a0_pushes_the_item_on_the_free_list() {
        let mut e = engine(0);
        let item = e.mem.alloc(0xc);
        e.mem.write(item, &[0xcc; 0xc]);
        e.call(0x00e6_e8a0, &args![0x0e20_0000u32, item]);
        assert_eq!(e.mem.u32(item), 0x0e10_0000);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), item);
        // The entry this call made is left again: count back to 0, owner cleared.
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        let log = e.call_log.take().unwrap();
        let locks: Vec<_> = log.iter().filter(|c| c.0 == LOCK_ENTER).collect();
        assert_eq!(locks, vec![&(LOCK_ENTER, vec![NODE_POOL_LOCK, 0])]);
    }

    #[test]
    fn fn_00e6e8a0_keeps_the_owner_of_a_lock_still_held() {
        let mut e = engine(1);
        let item = e.mem.alloc(0xc);
        e.call(0x00e6_e8a0, &args![0u32, item]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}

#[cfg(test)]
mod shut_down_tests {
    use super::*;

    const RELEASE: u32 = 0x0e50_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for callee in [
            RELEASE,
            OBJECT_DESTRUCT,
            NI_DELETE,
            IMPORT_FREE_LIBRARY,
            LIST_REMOVE_ALL,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.mem.set_u32(CLEARED_0126F0DC, 7);
        e
    }

    #[test]
    fn fn_00e6ec20_releases_what_is_held_then_empties_the_list() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, RELEASE);
        let com = e.mem.alloc(4);
        e.mem.set_u32(com, vtable);
        e.mem.set_u32(COM_OBJECT_0126F0D8, com);
        e.mem.set_u32(OBJECT_0126F0E8, 0x0e60_0000);
        e.mem.set_u32(MODULE_0126F0D4, 0x0e70_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e6_ec20, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (RELEASE, vec![com]),
                (OBJECT_DESTRUCT, vec![0x0e60_0000]),
                (NI_DELETE, vec![0x0e60_0000, 0x14]),
                (IMPORT_FREE_LIBRARY, vec![0x0e70_0000]),
                (LIST_REMOVE_ALL, vec![LIST_0126F0F8]),
            ]
        );
        assert_eq!(e.mem.u32(COM_OBJECT_0126F0D8), 0);
        assert_eq!(e.mem.u32(OBJECT_0126F0E8), 0);
        assert_eq!(e.mem.u32(MODULE_0126F0D4), 0x0e70_0000);
        assert_eq!(e.mem.u32(CLEARED_0126F0DC), 0);
    }

    #[test]
    fn fn_00e6ec20_with_nothing_held_only_empties_the_list() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00e6_ec20, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(LIST_REMOVE_ALL, vec![LIST_0126F0F8])]
        );
        assert_eq!(e.mem.u32(CLEARED_0126F0DC), 0);
    }
}
