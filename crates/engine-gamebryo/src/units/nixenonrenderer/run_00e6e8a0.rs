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
    vec![entry!(0x00e6_e8a0, fn_00e6e8a0(Ptr, Ptr))]
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
