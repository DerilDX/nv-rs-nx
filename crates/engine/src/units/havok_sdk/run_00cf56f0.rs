//! `Havok SDK/run_00cf56f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cf5890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_5750`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cf5890(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_5750, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cf5830 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (slot of the vtable at `010cc3b8`, no map name): the number at +0xC
/// minus the unsigned 16-bit number at +0x16.
pub fn fn_00cf5830(e: &mut Engine, this: Ptr) -> i32 {
    let count = e.mem.i32(this.addr() + 0xc);
    count.wrapping_sub(i32::from(e.mem.u16(this.addr() + 0x16)))
}

// Translated from 00cf5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (no map name): the index of the next set bit after `index` in the
/// 256-bit set at +0x40, or `-1` when none is below the count at +0xC. An index of 256 or
/// more (or a negative one, compared as unsigned) that is still below the count is
/// returned as it is.
pub fn fn_00cf5840(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    let count = e.mem.i32(this.addr() + 0xc);
    let mut next = index.wrapping_add(1);
    while next < count {
        if next as u32 >= 0x100 {
            return next;
        }
        let word = e.mem.u32(this.addr() + 0x40 + (next as u32 >> 5) * 4);
        if word & (1u32 << (next & 0x1f)) != 0 {
            return next;
        }
        next += 1;
    }
    -1
}

// Translated from 00cf5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (no map name): `00cf5840(this, -1)`, the first set bit.
pub fn fn_00cf5880(e: &mut Engine, this: Ptr) -> i32 {
    fn_00cf5840(e, this, -1)
}

// Translated from 00cf5820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00cf5890`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00cf5820(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00cf_5890, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_5830, fn_00cf5830(Ptr) -> i32),
        entry!(0x00cf_5840, fn_00cf5840(Ptr, i32) -> i32),
        entry!(0x00cf_5880, fn_00cf5880(Ptr) -> i32),
        entry!(0x00cf_5890, deleting_destructor_00cf5890(Ptr, u8) -> Ptr),
        entry!(0x00cf_5820, fn_00cf5820(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    fn bit_set(e: &mut Engine, count: i32, bits: &[u32]) -> u32 {
        let this = e.mem.alloc(0x80);
        e.mem.set_i32(this + 0xc, count);
        for bit in bits {
            let at = this + 0x40 + (bit >> 5) * 4;
            let word = e.mem.u32(at);
            e.mem.set_u32(at, word | (1 << (bit & 31)));
        }
        this
    }

    #[test]
    fn count_minus_the_16_bit_number() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 10, &[]);
        e.mem.set_u16(this + 0x16, 0xffff);
        assert_eq!(e.call(0x00cf_5830, &args![this]).i32(), 10 - 0xffff);
    }

    #[test]
    fn next_set_bit_walks_up_to_the_count() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 100, &[3, 40, 99]);
        let next = |e: &mut Engine, from: i32| e.call(0x00cf_5840, &args![this, from]).i32();
        assert_eq!(next(&mut e, -1), 3);
        assert_eq!(next(&mut e, 3), 40);
        assert_eq!(next(&mut e, 40), 99);
        assert_eq!(next(&mut e, 99), -1);
        assert_eq!(next(&mut e, 5000), -1);
        // a bit at or past the count is not looked at
        let this = bit_set(&mut e, 50, &[60]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 2i32]).i32(), -1);
    }

    #[test]
    fn next_set_bit_returns_indices_from_256_up_unexamined() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 300, &[]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 253i32]).i32(), 256);
        // 255 is below 256 and its bit is clear: the walk reaches 256 and returns it
        let this = bit_set(&mut e, 300, &[]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 255i32]).i32(), 256);
        // a negative start: -3 + 1 = -2 is below the count and compares as >= 256 unsigned
        assert_eq!(e.call(0x00cf_5840, &args![this, -3i32]).i32(), -2);
    }

    #[test]
    fn first_set_bit_starts_from_minus_one() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 64, &[0, 7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), 0);
        let this = bit_set(&mut e, 64, &[7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), 7);
        let this = bit_set(&mut e, 0, &[7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), -1);
    }

    #[test]
    fn deleting_destructor_00cf5890_frees_only_when_bit_zero_is_set() {
        check(0x00cf_5890, 0x00cf_5750);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00cf5820() {
        assert_eq!(check(0x00cf_5820, 0x00cf_5890, 0x10), 0xdead_beef);
    }
}
