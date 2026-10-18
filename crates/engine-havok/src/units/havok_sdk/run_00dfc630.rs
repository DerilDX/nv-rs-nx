//! `Havok SDK/run_00dfc630` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::release_array;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfd080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_cfd0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfd080(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_cfd0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dfc630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0102_e368.
pub fn return_constant_00dfc630(_e: &mut Engine) -> u32 {
    0x0102_e368
}

// Translated from 00dfc6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0a64.
pub fn return_constant_00dfc6d0(_e: &mut Engine) -> u32 {
    0x010d_0a64
}

// Translated from 00dfc740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0aac.
pub fn return_constant_00dfc740(_e: &mut Engine) -> u32 {
    0x010d_0aac
}

// Translated from 00dfc910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_11ec.
pub fn return_constant_00dfc910(_e: &mut Engine) -> u32 {
    0x010e_11ec
}

// Translated from 00dfc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0af4.
pub fn return_constant_00dfc9a0(_e: &mut Engine) -> u32 {
    0x010d_0af4
}

// Translated from 00dfca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b71c.
pub fn return_constant_00dfca30(_e: &mut Engine) -> u32 {
    0x010c_b71c
}

// Translated from 00dfc7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e3b060` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfc7a0(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e3_b060, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x170-byte stack buffer by calling `00dfc850` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfc830(e: &mut Engine) -> u32 {
    e.with_stack(0x170, |e, buffer| {
        e.call(0x00df_c850, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00e3b960` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfcb70(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00e3_b960, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00d53df0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfce10(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00d5_3df0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x50-byte stack buffer by calling `00dfcf60` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfcf40(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, buffer| {
        e.call(0x00df_cf60, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_a04c.
pub fn return_constant_00dfcaa0(_e: &mut Engine) -> u32 {
    0x010d_a04c
}

// Translated from 00dfcb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0b3c.
pub fn return_constant_00dfcb10(_e: &mut Engine) -> u32 {
    0x010d_0b3c
}

// Translated from 00dfcc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_37e4.
pub fn return_constant_00dfcc00(_e: &mut Engine) -> u32 {
    0x010d_37e4
}

// Translated from 00dfcc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_89e4.
pub fn return_constant_00dfcc70(_e: &mut Engine) -> u32 {
    0x010c_89e4
}

// Translated from 00dfcce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0be4.
pub fn return_constant_00dfcce0(_e: &mut Engine) -> u32 {
    0x010d_0be4
}

// Translated from 00dfcd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_91ec.
pub fn return_constant_00dfcd50(_e: &mut Engine) -> u32 {
    0x010c_91ec
}

// Translated from 00dfcea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0cd4.
pub fn return_constant_00dfcea0(_e: &mut Engine) -> u32 {
    0x010d_0cd4
}

// Translated from 00dfd120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_222c.
pub fn return_constant_00dfd120(_e: &mut Engine) -> u32 {
    0x010e_222c
}

// Translated from 00dfc850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor (`this` in `ECX`, one stack word `extra` that the
/// callee pops). It runs `00cd2c50` on `this` with `extra`, then empties three `hkArray`-shaped
/// triples (data, size, capacity-and-flags = `0x80000000`, "not owned") at `+0x8c`, `+0x98` and
/// `+0x160`, with the vtable word at `+0` set to `010ce15c` after the second one. Returns `this`
/// (`EAX`). The class is unconfirmed; `00dfc830` builds one on the stack.
pub fn fn_00dfc850(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00cd_2c50, &args![this, extra]);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0x8000_0000);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 0x8000_0000);
    e.mem.set_u32(base, 0x010c_e15c);
    e.mem.set_u32(base + 0x160, 0);
    e.mem.set_u32(base + 0x164, 0);
    e.mem.set_u32(base + 0x168, 0x8000_0000);
    this
}

// Translated from 00dfcf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010e2144`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010e2124`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, `0x17`) and the byte at `+0x15`
/// (3, 4). Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00dfcf60(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x010c_c364);
    e.mem.set_u32(base + 0x10, 0x010c_c344);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 3);
    }
    e.mem.set_u32(base, 0x010e_2144);
    e.mem.set_u32(base + 0x10, 0x010e_2124);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0x17);
        e.mem.set_u8(base + 0x15, 4);
    }
    this
}

// Translated from 00dfcfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to the deleting destructor
/// `00dfd080` (`RET 4`); the caller gets its `EAX`, `this - 0x10`. The class is unconfirmed.
pub fn fn_00dfcfc0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00dfd080(e, Ptr::new(this.addr().wrapping_sub(0x10)), flags)
}

// Translated from 00dfcfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`) that releases three `hkArray`-shaped
/// triples in turn, the one at `+0x30` (byte elements), then those at `+0x24` and `+0x18`
/// (16-byte elements), and sets the vtable word at `+0x10` to `010c6fb0` and the one at `+0` to
/// `0102e388`. The class is unconfirmed; `00dfd080` is its deleting form.
pub fn fn_00dfcfd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x30, 1);
    release_array(e, base + 0x24, 16);
    release_array(e, base + 0x18, 16);
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_c7a0, fn_00dfc7a0() -> u32),
        entry!(0x00df_c830, fn_00dfc830() -> u32),
        entry!(0x00df_cb70, fn_00dfcb70() -> u32),
        entry!(0x00df_ce10, fn_00dfce10() -> u32),
        entry!(0x00df_cf40, fn_00dfcf40() -> u32),
        entry!(0x00dfc630, return_constant_00dfc630() -> u32),
        entry!(0x00dfc6d0, return_constant_00dfc6d0() -> u32),
        entry!(0x00dfc740, return_constant_00dfc740() -> u32),
        entry!(0x00dfc910, return_constant_00dfc910() -> u32),
        entry!(0x00dfc9a0, return_constant_00dfc9a0() -> u32),
        entry!(0x00dfca30, return_constant_00dfca30() -> u32),
        entry!(0x00df_d080, deleting_destructor_00dfd080(Ptr, u8) -> Ptr),
        entry!(0x00dfcaa0, return_constant_00dfcaa0() -> u32),
        entry!(0x00dfcb10, return_constant_00dfcb10() -> u32),
        entry!(0x00dfcc00, return_constant_00dfcc00() -> u32),
        entry!(0x00dfcc70, return_constant_00dfcc70() -> u32),
        entry!(0x00dfcce0, return_constant_00dfcce0() -> u32),
        entry!(0x00dfcd50, return_constant_00dfcd50() -> u32),
        entry!(0x00dfcea0, return_constant_00dfcea0() -> u32),
        entry!(0x00dfd120, return_constant_00dfd120() -> u32),
        entry!(0x00df_c850, fn_00dfc850(Ptr, u32) -> Ptr),
        entry!(0x00df_cf60, fn_00dfcf60(Ptr, u32) -> Ptr),
        entry!(0x00df_cfc0, fn_00dfcfc0(Ptr, u8) -> Ptr),
        entry!(0x00df_cfd0, fn_00dfcfd0(Ptr)),
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
    fn deleting_destructor_00dfd080_frees_only_when_bit_zero_is_set() {
        check(0x00df_d080, 0x00df_cfd0);
    }

    #[test]
    fn return_constant_00dfc630_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c630, &args![]).u32(), 0x0102_e368);
    }

    #[test]
    fn return_constant_00dfc6d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c6d0, &args![]).u32(), 0x010d_0a64);
    }

    #[test]
    fn return_constant_00dfc740_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c740, &args![]).u32(), 0x010d_0aac);
    }

    #[test]
    fn return_constant_00dfc910_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c910, &args![]).u32(), 0x010e_11ec);
    }

    #[test]
    fn return_constant_00dfc9a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c9a0, &args![]).u32(), 0x010d_0af4);
    }

    #[test]
    fn return_constant_00dfca30_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ca30, &args![]).u32(), 0x010c_b71c);
    }
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00dfc7a0_returns_first_word_of_built_object,
        0x00df_c7a0,
        0x00e3_b060
    );
    first_word_test!(
        fn_00dfc830_returns_first_word_of_built_object,
        0x00df_c830,
        0x00df_c850
    );
    first_word_test!(
        fn_00dfcb70_returns_first_word_of_built_object,
        0x00df_cb70,
        0x00e3_b960
    );
    first_word_test!(
        fn_00dfce10_returns_first_word_of_built_object,
        0x00df_ce10,
        0x00d5_3df0
    );
    first_word_test!(
        fn_00dfcf40_returns_first_word_of_built_object,
        0x00df_cf40,
        0x00df_cf60
    );
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfc7a0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_b060, 0x0a0b_0c0d);
        let result = e.call(0x00df_c7a0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfc830_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_c850, 0x0a0b_0c0d);
        let result = e.call(0x00df_c830, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcaa0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_caa0, &args![]).u32(), 0x010d_a04c);
    }

    #[test]
    fn return_constant_00dfcb10_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cb10, &args![]).u32(), 0x010d_0b3c);
    }

    #[test]
    fn fn_00dfcb70_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_b960, 0x0a0b_0c0d);
        let result = e.call(0x00df_cb70, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcc00_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cc00, &args![]).u32(), 0x010d_37e4);
    }

    #[test]
    fn return_constant_00dfcc70_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cc70, &args![]).u32(), 0x010c_89e4);
    }

    #[test]
    fn return_constant_00dfcce0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cce0, &args![]).u32(), 0x010d_0be4);
    }

    #[test]
    fn return_constant_00dfcd50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cd50, &args![]).u32(), 0x010c_91ec);
    }

    #[test]
    fn fn_00dfce10_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_3df0, 0x0a0b_0c0d);
        let result = e.call(0x00df_ce10, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcea0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cea0, &args![]).u32(), 0x010d_0cd4);
    }

    #[test]
    fn fn_00dfcf40_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_cf60, 0x0a0b_0c0d);
        let result = e.call(0x00df_cf40, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfd120_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_d120, &args![]).u32(), 0x010e_222c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    /// `00dfc850` calls `00cd2c50(this, extra)` first, then writes its fields.
    #[test]
    fn fn_00dfc850_constructs_over_the_base() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x170);
        e.mem.write(this, &[0x55; 0x170]);
        e.register_double(0x00cd_2c50, move |e, a| {
            assert_eq!(a, [this, 9]);
            assert_eq!(e.mem.u32(this + 0x8c), 0x5555_5555);
            Ret::default()
        });
        assert_eq!(e.call(0x00df_c850, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_e15c);
        for offset in [0x8c, 0x90, 0x98, 0x9c, 0x160, 0x164] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in [0x94, 0xa0, 0x168] {
            assert_eq!(e.mem.u32(this + offset), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x88), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0xa4), 0x5555_5555);
    }

    /// Mode 0 leaves +0xc and the byte at +0x15 alone; another mode ends with 0x17 and 4.
    #[test]
    fn fn_00dfcf60_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00df_cf60, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_2144);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_2124);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.call(0x00df_cf60, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x17);
        assert_eq!(e.mem.u8(this + 0x15), 4);
        assert_eq!(e.mem.u32(this), 0x010e_2144);
    }

    /// The thunk runs the deleting destructor on `this - 0x10`, whose base destructor
    /// `00dfcfd0` is a double here.
    #[test]
    fn fn_00dfcfc0_runs_the_destructor_on_this_minus_0x10() {
        let mut e = Engine::new();
        e.register(0x00df_cfd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20) + 0x10;
        let got = e.call(0x00df_cfc0, &args![this, 0u32]).u32();
        assert_eq!(got, this - 0x10);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00df_cfd0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this - 0x10]);
    }

    /// Three arrays: the first owned (capacity 5, bytes), the second not owned (negative
    /// capacity word: nothing freed), the third owned (capacity 3 of 16-byte elements).
    #[test]
    fn fn_00dfcfd0_releases_the_owned_arrays() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        // +0x30: data 0x1111, size 4, capacity 5 | flag bits 0x40000000.
        e.mem.set_u32(this + 0x30, 0x1111);
        e.mem.set_u32(this + 0x34, 4);
        e.mem.set_u32(this + 0x38, 0x4000_0005);
        // +0x24: not owned.
        e.mem.set_u32(this + 0x24, 0x2222);
        e.mem.set_u32(this + 0x28, 2);
        e.mem.set_u32(this + 0x2c, 0x8000_0002);
        // +0x18: data 0x3333, size 1, capacity 3.
        e.mem.set_u32(this + 0x18, 0x3333);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 3);
        e.call(0x00df_cfd0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![vec![allocator, 0x1111, 5], vec![allocator, 0x3333, 0x30]]
        );
        for (data, size, capacity) in [(0x30, 0x34, 0x38), (0x24, 0x28, 0x2c), (0x18, 0x1c, 0x20)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
