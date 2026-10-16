//! `Havok SDK/hkplinearparametriccurveclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e00070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_0000`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e00070(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_0000, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e00990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_8740`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e00990(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_8740, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e01010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_90d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e01010(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_90d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e02090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_1f70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e02090(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_1f70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e02230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_21d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e02230(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_21d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e02430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_23c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e02430(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_23c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e05090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_4f80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e05090(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_4f80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e05970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_57c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e05970(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_57c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e09640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_9450`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e09640(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_9450, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e34920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_4820`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e34920(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_4820, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e41050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_0cb0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e41050(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_0cb0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e46750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_66e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e46750(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_66e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_0070, deleting_destructor_00e00070(Ptr, u8) -> Ptr),
        entry!(0x00e0_0990, deleting_destructor_00e00990(Ptr, u8) -> Ptr),
        entry!(0x00e0_1010, deleting_destructor_00e01010(Ptr, u8) -> Ptr),
        entry!(0x00e0_2090, deleting_destructor_00e02090(Ptr, u8) -> Ptr),
        entry!(0x00e0_2230, deleting_destructor_00e02230(Ptr, u8) -> Ptr),
        entry!(0x00e0_2430, deleting_destructor_00e02430(Ptr, u8) -> Ptr),
        entry!(0x00e0_5090, deleting_destructor_00e05090(Ptr, u8) -> Ptr),
        entry!(0x00e0_5970, deleting_destructor_00e05970(Ptr, u8) -> Ptr),
        entry!(0x00e0_9640, deleting_destructor_00e09640(Ptr, u8) -> Ptr),
        entry!(0x00e3_4920, deleting_destructor_00e34920(Ptr, u8) -> Ptr),
        entry!(0x00e4_1050, deleting_destructor_00e41050(Ptr, u8) -> Ptr),
        entry!(0x00e4_6750, deleting_destructor_00e46750(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e00070_frees_only_when_bit_zero_is_set() {
        check(0x00e0_0070, 0x00e0_0000);
    }

    #[test]
    fn deleting_destructor_00e00990_frees_only_when_bit_zero_is_set() {
        check(0x00e0_0990, 0x00e4_8740);
    }

    #[test]
    fn deleting_destructor_00e01010_frees_only_when_bit_zero_is_set() {
        check(0x00e0_1010, 0x00e4_90d0);
    }

    #[test]
    fn deleting_destructor_00e02090_frees_only_when_bit_zero_is_set() {
        check(0x00e0_2090, 0x00e0_1f70);
    }

    #[test]
    fn deleting_destructor_00e02230_frees_only_when_bit_zero_is_set() {
        check(0x00e0_2230, 0x00e0_21d0);
    }

    #[test]
    fn deleting_destructor_00e02430_frees_only_when_bit_zero_is_set() {
        check(0x00e0_2430, 0x00e0_23c0);
    }

    #[test]
    fn deleting_destructor_00e05090_frees_only_when_bit_zero_is_set() {
        check(0x00e0_5090, 0x00e0_4f80);
    }

    #[test]
    fn deleting_destructor_00e05970_frees_only_when_bit_zero_is_set() {
        check(0x00e0_5970, 0x00e0_57c0);
    }

    #[test]
    fn deleting_destructor_00e09640_frees_only_when_bit_zero_is_set() {
        check(0x00e0_9640, 0x00e0_9450);
    }

    #[test]
    fn deleting_destructor_00e34920_frees_only_when_bit_zero_is_set() {
        check(0x00e3_4920, 0x00e3_4820);
    }

    #[test]
    fn deleting_destructor_00e41050_frees_only_when_bit_zero_is_set() {
        check(0x00e4_1050, 0x00e4_0cb0);
    }

    #[test]
    fn deleting_destructor_00e46750_frees_only_when_bit_zero_is_set() {
        check(0x00e4_6750, 0x00e4_66e0);
    }
}
