//! `Havok SDK/run_00dff0e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

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
