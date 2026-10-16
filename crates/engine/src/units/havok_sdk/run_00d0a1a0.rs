//! `Havok SDK/run_00d0a1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d0a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_8e80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d0a240(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_8e80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d0a1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4 (`SUB ECX,0x4`) and jumps to `00d0a1a0`. `00d0a1a0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a1d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a1a0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00d0a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a200(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00d0a210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a210(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

// Translated from 00d0a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x1c (`SUB ECX,0x1c`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a220(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x1c), flags])
        .u32()
}

// Translated from 00d0a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a230(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_a240, deleting_destructor_00d0a240(Ptr, u8) -> Ptr),
        entry!(0x00d0_a1d0, fn_00d0a1d0(Ptr, u32) -> u32),
        entry!(0x00d0_a200, fn_00d0a200(Ptr, u32) -> u32),
        entry!(0x00d0_a210, fn_00d0a210(Ptr, u32) -> u32),
        entry!(0x00d0_a220, fn_00d0a220(Ptr, u32) -> u32),
        entry!(0x00d0_a230, fn_00d0a230(Ptr, u32) -> u32),
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

    #[test]
    fn deleting_destructor_00d0a240_frees_only_when_bit_zero_is_set() {
        check(0x00d0_a240, 0x00d0_8e80);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d0a1d0() {
        assert_eq!(
            check(0x00d0_a1d0, 0x00d0_a1a0, 0x4, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a200() {
        assert_eq!(
            check(0x00d0_a200, 0x00d0_a240, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a210() {
        assert_eq!(
            check(0x00d0_a210, 0x00d0_a240, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a220() {
        assert_eq!(
            check(0x00d0_a220, 0x00d0_a240, 0x1c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a230() {
        assert_eq!(
            check(0x00d0_a230, 0x00d0_a240, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
