//! `Havok SDK/run_00dff0e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dff440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00dff470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff440(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f470, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00dff450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00dff470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff450(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f470, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00dff1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00e45010` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dff1a0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00e4_5010, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff310 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00e45120` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dff310(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00e4_5120, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00e46300` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dff4f0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00e4_6300, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e466a0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dff550(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e4_66a0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_676c.
pub fn return_constant_00dff0e0(_e: &mut Engine) -> u32 {
    0x010e_676c
}

// Translated from 00dff130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010e_6834 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dff130(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010e_6834
    })
}

// Translated from 00dff220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame stores the vtable word 0x010c_ce28 at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x38 (argument 0) and
/// returns the constant 0x010d_07f4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dff220(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_ce28);
        e.call(0x00cb_0a70, &args![base + 0x38, 0u32]);
        0x010d_07f4
    })
}

// Translated from 00dff2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x60-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010c_8e94 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dff2b0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010c_8e94
    })
}

// Translated from 00dff380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x40-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_0944 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dff380(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_0944
    })
}

// Translated from 00dff430 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_6dec.
pub fn return_constant_00dff430(_e: &mut Engine) -> u32 {
    0x010e_6dec
}

// Translated from 00dff5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_75cc.
pub fn return_constant_00dff5f0(_e: &mut Engine) -> u32 {
    0x010e_75cc
}

// Translated from 00dff610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_75dc.
pub fn return_constant_00dff610(_e: &mut Engine) -> u32 {
    0x010e_75dc
}

// Translated from 00dff630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_75ec.
pub fn return_constant_00dff630(_e: &mut Engine) -> u32 {
    0x010e_75ec
}

// Translated from 00dff0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010e6834`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dff0f0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010e_6834);
    }
}

// Translated from 00dff1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cce28`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x38`, one stack word `extra`) and
/// writes the vtable word `010d07f4`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dff1f0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_ce28);
        e.call(0x00cb_0a70, &args![object.addr() + 0x38, extra]);
        e.mem.set_u32(object.addr(), 0x010d_07f4);
    }
}

// Translated from 00dff270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`),
/// writes the vtable word `010c8e94` and empties the `hkArray`-shaped triple at `+0x54` (data 0,
/// size 0, capacity and flags `0x80000000`). The function takes two stack words and pops none
/// (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00dff270(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        let base = object.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, extra]);
        e.mem.set_u32(base, 0x010c_8e94);
        e.mem.set_u32(base + 0x54, 0);
        e.mem.set_u32(base + 0x58, 0);
        e.mem.set_u32(base + 0x5c, 0x8000_0000);
    }
}

// Translated from 00dff340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d0944`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dff340(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_0944);
    }
}

// Translated from 00dff470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry point inside `00dff460` (other thunks, `bhkWorld`'s and `run_00cbfba0`'s among them,
/// jump here after moving `this` back; entered with `this` in `ECX` already moved, one stack
/// word `flags`, `RET 4`): the scalar deleting destructor's body. The vtable words at `+0x14`,
/// `+0x10`, `+0xc`, `+8` and `+0` become `010c3b8c`, `010c3bac`, `010c3b98`, `010c3b80` and
/// `0102e388`, and when bit 0 of `flags` is set the block is returned to the memory router's
/// allocator (slot `8`) with the 16-bit size kept at `+4`. Returns `this` (`EAX`). The class is
/// unconfirmed.
pub fn deleting_destructor_00dff470(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x14), 0x010c_3b8c);
    e.mem.set_u32(base.wrapping_add(0x10), 0x010c_3bac);
    e.mem.set_u32(base.wrapping_add(0xc), 0x010c_3b98);
    e.mem.set_u32(base.wrapping_add(0x8), 0x010c_3b80);
    e.mem.set_u32(base, 0x0102_e388);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(base.wrapping_add(4)));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![base, size]);
    }
    this
}

// Translated from 00dff460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and continues in the body at `00dff470`
/// (the same stack word `flags`); the caller gets its `EAX`, the moved `this`. The class is
/// unconfirmed.
pub fn deleting_destructor_00dff460(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00dff470(e, Ptr::new(this.addr().wrapping_sub(0x14)), flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f1a0, fn_00dff1a0() -> u32),
        entry!(0x00df_f310, fn_00dff310() -> u32),
        entry!(0x00df_f4f0, fn_00dff4f0() -> u32),
        entry!(0x00df_f550, fn_00dff550() -> u32),
        entry!(0x00df_f440, fn_00dff440(Ptr, u32) -> u32),
        entry!(0x00df_f450, fn_00dff450(Ptr, u32) -> u32),
        entry!(0x00dff0e0, return_constant_00dff0e0() -> u32),
        entry!(0x00dff130, fn_00dff130() -> u32),
        entry!(0x00dff220, fn_00dff220() -> u32),
        entry!(0x00dff2b0, fn_00dff2b0() -> u32),
        entry!(0x00dff380, fn_00dff380() -> u32),
        entry!(0x00dff430, return_constant_00dff430() -> u32),
        entry!(0x00dff5f0, return_constant_00dff5f0() -> u32),
        entry!(0x00dff610, return_constant_00dff610() -> u32),
        entry!(0x00dff630, return_constant_00dff630() -> u32),
        entry!(0x00df_f0f0, fn_00dff0f0(Ptr, u32)),
        entry!(0x00df_f1f0, fn_00dff1f0(Ptr, u32)),
        entry!(0x00df_f270, fn_00dff270(Ptr, u32)),
        entry!(0x00df_f340, fn_00dff340(Ptr, u32)),
        entry!(0x00df_f460, deleting_destructor_00dff460(Ptr, u8) -> Ptr),
        entry!(0x00df_f470, deleting_destructor_00dff470(Ptr, u8) -> Ptr),
    ]
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
    fn thunk_00dff440() {
        assert_eq!(check(0x00df_f440, 0x00df_f470, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00dff450() {
        assert_eq!(check(0x00df_f450, 0x00df_f470, 0xc), 0xdead_beef);
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
        fn_00dff1a0_returns_first_word_of_built_object,
        0x00df_f1a0,
        0x00e4_5010
    );
    first_word_test!(
        fn_00dff310_returns_first_word_of_built_object,
        0x00df_f310,
        0x00e4_5120
    );
    first_word_test!(
        fn_00dff4f0_returns_first_word_of_built_object,
        0x00df_f4f0,
        0x00e4_6300
    );
    first_word_test!(
        fn_00dff550_returns_first_word_of_built_object,
        0x00df_f550,
        0x00e4_66a0
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
    fn return_constant_00dff0e0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f0e0, &args![]).u32(), 0x010e_676c);
    }

    #[test]
    fn fn_00dff130_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_f130, &args![]).u32();
        assert_eq!(result, 0x010e_6834);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn fn_00dff1a0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_5010, 0x0a0b_0c0d);
        let result = e.call(0x00df_f1a0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dff220_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x38], false);
        let result = e.call(0x00df_f220, &args![]).u32();
        assert_eq!(result, 0x010d_07f4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_ce28);
        }
    }

    #[test]
    fn fn_00dff2b0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_f2b0, &args![]).u32();
        assert_eq!(result, 0x010c_8e94);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn fn_00dff310_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_5120, 0x0a0b_0c0d);
        let result = e.call(0x00df_f310, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dff380_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_f380, &args![]).u32();
        assert_eq!(result, 0x010d_0944);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn return_constant_00dff430_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f430, &args![]).u32(), 0x010e_6dec);
    }

    #[test]
    fn fn_00dff4f0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_6300, 0x0a0b_0c0d);
        let result = e.call(0x00df_f4f0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dff550_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_66a0, 0x0a0b_0c0d);
        let result = e.call(0x00df_f550, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dff5f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f5f0, &args![]).u32(), 0x010e_75cc);
    }

    #[test]
    fn return_constant_00dff610_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f610, &args![]).u32(), 0x010e_75dc);
    }

    #[test]
    fn return_constant_00dff630_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f630, &args![]).u32(), 0x010e_75ec);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    /// Runs `address` on a null object (nothing happens) and on a real one, with the string
    /// function `00cb0a70` as a double that checks its words and that the first vtable word is
    /// already in place; returns the object, whose vtable word must end as `last`.
    fn check_string_member(address: u32, first: u32, member: u32, last: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x80);
        e.mem.write(object, &[0x44; 0x80]);
        let calls = Rc::new(RefCell::new(0));
        let seen = Rc::clone(&calls);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + member, 9]);
            assert_eq!(e.mem.u32(object), first);
            *seen.borrow_mut() += 1;
            Ret::default()
        });
        e.call(address, &args![0u32, 9u32]);
        assert_eq!(*calls.borrow(), 0);
        e.call(address, &args![object, 9u32]);
        assert_eq!(*calls.borrow(), 1);
        assert_eq!(e.mem.u32(object), last);
        (e, object)
    }

    #[test]
    fn fn_00dff0f0_vtable_words_around_the_string_function() {
        check_string_member(0x00df_f0f0, 0x010c_c2fc, 0x14, 0x010e_6834);
    }

    #[test]
    fn fn_00dff1f0_vtable_words_around_the_string_function() {
        check_string_member(0x00df_f1f0, 0x010c_ce28, 0x38, 0x010d_07f4);
    }

    #[test]
    fn fn_00dff340_vtable_words_around_the_string_function() {
        check_string_member(0x00df_f340, 0x010c_c2fc, 0x14, 0x010d_0944);
    }

    /// Also empties the array triple at +0x54.
    #[test]
    fn fn_00dff270_also_empties_the_array() {
        let (e, object) = check_string_member(0x00df_f270, 0x010c_c2fc, 0x14, 0x010c_8e94);
        assert_eq!(e.mem.u32(object + 0x54), 0);
        assert_eq!(e.mem.u32(object + 0x58), 0);
        assert_eq!(e.mem.u32(object + 0x5c), 0x8000_0000);
        assert_eq!(e.mem.u32(object + 0x50), 0x4444_4444);
        assert_eq!(e.mem.u32(object + 0x60), 0x4444_4444);
    }

    /// The vtable words are written for the object `this - 0x14`; the block (size kept at +4)
    /// is freed only when bit 0 of the flags is set; `this - 0x14` is returned.
    #[test]
    fn deleting_destructor_00dff460_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
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
        let block = e.mem.alloc(0x40);
        e.mem.set_u16(block + 4, 0x1234);
        let got = e
            .call(0x00df_f460, &args![block + 0x14, 0xffff_fffeu32])
            .u32();
        assert_eq!(got, block);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(block + 0x14), 0x010c_3b8c);
        assert_eq!(e.mem.u32(block + 0x10), 0x010c_3bac);
        assert_eq!(e.mem.u32(block + 0xc), 0x010c_3b98);
        assert_eq!(e.mem.u32(block + 8), 0x010c_3b80);
        assert_eq!(e.mem.u32(block), 0x0102_e388);
        let got = e.call(0x00df_f460, &args![block + 0x14, 1u32]).u32();
        assert_eq!(got, block);
        assert_eq!(*log.borrow(), vec![vec![allocator, block, 0x1234]]);
    }

    /// The body at `00dff470` is an entry of its own (`this` as given), and the unit's thunks
    /// (`00dff440` moves `this` back by 8, `00dff450` by 0xc) reach it through the engine.
    #[test]
    fn deleting_destructor_00dff470_is_an_entry_of_its_own() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
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
        let block = e.mem.alloc(0x40);
        e.mem.set_u16(block + 4, 0x2468);
        assert_eq!(e.call(0x00df_f470, &args![block, 0u32]).u32(), block);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(block), 0x0102_e388);
        assert_eq!(e.mem.u32(block + 0x14), 0x010c_3b8c);
        assert_eq!(e.call(0x00df_f470, &args![block, 1u32]).u32(), block);
        assert_eq!(*log.borrow(), vec![vec![allocator, block, 0x2468]]);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00df_f440, &args![block + 8, 1u32]).u32(), block);
        assert_eq!(e.call(0x00df_f450, &args![block + 0xc, 1u32]).u32(), block);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, block, 0x2468],
                vec![allocator, block, 0x2468]
            ]
        );
    }
}
