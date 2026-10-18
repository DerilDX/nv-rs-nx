//! `Havok SDK/run_00dfd200` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::release_array;

// Translated from 00dfd200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00cf56f0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd200(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00cf_56f0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00d59830` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd280(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00d5_9830, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xf0-byte stack buffer by calling `00e3dbd0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd320(e: &mut Engine) -> u32 {
    e.with_stack(0xf0, |e, buffer| {
        e.call(0x00e3_dbd0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00e3e380` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd630(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00e3_e380, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x5c-byte frame stores the vtable word 0x010e_311c at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x20, 0x4c in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00dfd800(e: &mut Engine) -> u32 {
    e.with_stack(0x5c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_311c);
        e.call(0x00cb_0a70, &args![base + 0x20, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x4c, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00dfd380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases four `hkArray`-shaped triples (data, size, capacity and
/// flags) of 2-byte elements, at `+0x34`, `+0x28`, `+0x1c` and `+0x10` of `this` (`ECX`), in that
/// order. Each one: the size is cleared, the data is returned through the allocator's slot
/// `0x10` with `capacity * 2` bytes unless the capacity word is negative, then data is cleared
/// and the capacity word becomes `0x80000000`. The class is unconfirmed.
pub fn fn_00dfd380(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x34, 2);
    release_array(e, base + 0x28, 2);
    release_array(e, base + 0x1c, 2);
    release_array(e, base + 0x10, 2);
}

// Translated from 00dfd440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases three `hkArray`-shaped triples of `this` (`ECX`) the way
/// `00dfd380` does: at `+0x28` (2-byte elements), `+0x1c` (1-byte elements) and `+0x10` (2-byte
/// elements), in that order. The class is unconfirmed.
pub fn fn_00dfd440(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x28, 2);
    release_array(e, base + 0x1c, 1);
    release_array(e, base + 0x10, 2);
}

// Translated from 00dfd680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases five `hkArray`-shaped triples of `this` (`ECX`) the way
/// `00dfd380` does, in this order: `+0x30` (1-byte elements), `+0x24` (2 bytes), `+0x18`
/// (4 bytes), `+0x0c` (4 bytes) and `+0` (16 bytes). The class is unconfirmed.
pub fn fn_00dfd680(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x30, 1);
    release_array(e, base + 0x24, 2);
    release_array(e, base + 0x18, 4);
    release_array(e, base + 0x0c, 4);
    release_array(e, base, 16);
}

// Translated from 00dfd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e311c` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 0x14` and
/// then on `object + 0x40`. The function takes two stack words and pops none (`RET`), and
/// returns nothing. The class is unconfirmed.
pub fn fn_00dfd7d0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_311c);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x40, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_d200, fn_00dfd200() -> u32),
        entry!(0x00df_d280, fn_00dfd280() -> u32),
        entry!(0x00df_d320, fn_00dfd320() -> u32),
        entry!(0x00df_d630, fn_00dfd630() -> u32),
        entry!(0x00dfd800, fn_00dfd800() -> u32),
        entry!(0x00df_d380, fn_00dfd380(Ptr)),
        entry!(0x00df_d440, fn_00dfd440(Ptr)),
        entry!(0x00df_d680, fn_00dfd680(Ptr)),
        entry!(0x00df_d7d0, fn_00dfd7d0(Ptr, u32)),
    ]
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
        fn_00dfd200_returns_first_word_of_built_object,
        0x00df_d200,
        0x00cf_56f0
    );
    first_word_test!(
        fn_00dfd280_returns_first_word_of_built_object,
        0x00df_d280,
        0x00d5_9830
    );
    first_word_test!(
        fn_00dfd320_returns_first_word_of_built_object,
        0x00df_d320,
        0x00e3_dbd0
    );
    first_word_test!(
        fn_00dfd630_returns_first_word_of_built_object,
        0x00df_d630,
        0x00e3_e380
    );
}

#[cfg(test)]
mod tests {
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

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfd200_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00cf_56f0, 0x0a0b_0c0d);
        let result = e.call(0x00df_d200, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd280_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_9830, 0x0a0b_0c0d);
        let result = e.call(0x00df_d280, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd320_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_dbd0, 0x0a0b_0c0d);
        let result = e.call(0x00df_d320, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd630_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_e380, 0x0a0b_0c0d);
        let result = e.call(0x00df_d630, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd800_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14, 0x40], false);
        let result = e.call(0x00df_d800, &args![]).u32();
        assert_eq!(result, 0x010e_311c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_311c);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x2c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A memory router whose allocator's slot 0x10 is a double that logs its words.
    fn engine_with_allocator() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
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
        (e, log, allocator)
    }

    /// Sets the array triple at `array` to `data`, size 3 and the capacity word `capacity`.
    fn set_array(e: &mut Engine, array: u32, data: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, capacity);
    }

    fn assert_cleared(e: &Engine, array: u32) {
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    /// Four owned arrays of 2-byte elements are freed in the order +0x34, +0x28, +0x1c, +0x10.
    #[test]
    fn fn_00dfd380_releases_four_arrays_of_words() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x34, 0xa1, 5);
        set_array(&mut e, this + 0x28, 0xa2, 6);
        set_array(&mut e, this + 0x1c, 0xa3, 0x8000_0007);
        set_array(&mut e, this + 0x10, 0xa4, 0x4000_0008);
        e.call(0x00df_d380, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xa1, 10],
                vec![allocator, 0xa2, 12],
                vec![allocator, 0xa4, 16]
            ]
        );
        for offset in [0x34, 0x28, 0x1c, 0x10] {
            assert_cleared(&e, this + offset);
        }
    }

    /// The middle array holds single bytes.
    #[test]
    fn fn_00dfd440_releases_three_arrays() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x28, 0xb1, 5);
        set_array(&mut e, this + 0x1c, 0xb2, 6);
        set_array(&mut e, this + 0x10, 0xb3, 7);
        e.call(0x00df_d440, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xb1, 10],
                vec![allocator, 0xb2, 6],
                vec![allocator, 0xb3, 14]
            ]
        );
        for offset in [0x28, 0x1c, 0x10] {
            assert_cleared(&e, this + offset);
        }
    }

    /// Five arrays with element sizes 1, 2, 4, 4 and 16; the one at +0x24 is not owned.
    #[test]
    fn fn_00dfd680_releases_five_arrays() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x30, 0xc1, 3);
        set_array(&mut e, this + 0x24, 0xc2, 0x8000_0004);
        set_array(&mut e, this + 0x18, 0xc3, 5);
        set_array(&mut e, this + 0x0c, 0xc4, 6);
        set_array(&mut e, this, 0xc5, 7);
        e.call(0x00df_d680, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xc1, 3],
                vec![allocator, 0xc3, 20],
                vec![allocator, 0xc4, 24],
                vec![allocator, 0xc5, 112]
            ]
        );
        for offset in [0x30, 0x24, 0x18, 0x0c, 0] {
            assert_cleared(&e, this + offset);
        }
    }

    /// A null object does nothing; otherwise the string function runs on both members.
    #[test]
    fn fn_00dfd7d0_runs_the_string_function_twice() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00df_d7d0, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x60);
        e.call(0x00df_d7d0, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_311c);
        assert_eq!(
            *log.borrow(),
            vec![vec![object + 0x14, 4], vec![object + 0x40, 4]]
        );
    }
}
