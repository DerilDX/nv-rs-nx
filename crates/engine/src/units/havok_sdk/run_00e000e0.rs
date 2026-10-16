//! `Havok SDK/run_00e000e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e000e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00d50c00` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e000e0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00d5_0c00, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e003b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x170-byte stack buffer by calling `00e003d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e003b0(e: &mut Engine) -> u32 {
    e.with_stack(0x170, |e, buffer| {
        e.call(0x00e0_03d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e004d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xe0-byte stack buffer by calling `00e004f0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e004d0(e: &mut Engine) -> u32 {
    e.with_stack(0xe0, |e, buffer| {
        e.call(0x00e0_04f0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e005f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00e00610` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e005f0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00e0_0610, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e006b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e006d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e006b0(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e0_06d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6cdc.
pub fn return_constant_00e00150(_e: &mut Engine) -> u32 {
    0x010c_6cdc
}

// Translated from 00e00190 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_81a4.
pub fn return_constant_00e00190(_e: &mut Engine) -> u32 {
    0x010e_81a4
}

// Translated from 00e001f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x20-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_79c4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00e001f0(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_79c4
    })
}

// Translated from 00e00290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame zeroes the `u16` at +0x24, stores
/// the vtable word 0x010c_c040 at the start, 0 at +0x20 and 0x8000 in the `u16` at +0x26, runs the
/// constructor at `00cb0a70` on the member at frame+0x28 (argument 0) and returns the constant 0x010d_79e8 (the stored word is not read back).
pub fn fn_00e00290(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u16(base + 0x24, 0);
        e.mem.set_u32(base, 0x010c_c040);
        e.mem.set_u32(base + 0x20, 0);
        e.mem.set_u16(base + 0x26, 0x8000);
        e.call(0x00cb_0a70, &args![base + 0x28, 0u32]);
        0x010d_79e8
    })
}

// Translated from 00e00320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6ea4.
pub fn return_constant_00e00320(_e: &mut Engine) -> u32 {
    0x010c_6ea4
}

// Translated from 00e00360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_83b4.
pub fn return_constant_00e00360(_e: &mut Engine) -> u32 {
    0x010e_83b4
}

// Translated from 00e00480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6dc4.
pub fn return_constant_00e00480(_e: &mut Engine) -> u32 {
    0x010c_6dc4
}

// Translated from 00e005a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_749c.
pub fn return_constant_00e005a0(_e: &mut Engine) -> u32 {
    0x010c_749c
}

// Translated from 00e00780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_c41c.
pub fn return_constant_00e00780(_e: &mut Engine) -> u32 {
    0x010c_c41c
}

// Translated from 00e007e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b5ac.
pub fn return_constant_00e007e0(_e: &mut Engine) -> u32 {
    0x010c_b5ac
}

// Translated from 00e00820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_8824.
pub fn return_constant_00e00820(_e: &mut Engine) -> u32 {
    0x010e_8824
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_00e0, fn_00e000e0() -> u32),
        entry!(0x00e0_03b0, fn_00e003b0() -> u32),
        entry!(0x00e0_04d0, fn_00e004d0() -> u32),
        entry!(0x00e0_05f0, fn_00e005f0() -> u32),
        entry!(0x00e0_06b0, fn_00e006b0() -> u32),
        entry!(0x00e00150, return_constant_00e00150() -> u32),
        entry!(0x00e00190, return_constant_00e00190() -> u32),
        entry!(0x00e001f0, fn_00e001f0() -> u32),
        entry!(0x00e00290, fn_00e00290() -> u32),
        entry!(0x00e00320, return_constant_00e00320() -> u32),
        entry!(0x00e00360, return_constant_00e00360() -> u32),
        entry!(0x00e00480, return_constant_00e00480() -> u32),
        entry!(0x00e005a0, return_constant_00e005a0() -> u32),
        entry!(0x00e00780, return_constant_00e00780() -> u32),
        entry!(0x00e007e0, return_constant_00e007e0() -> u32),
        entry!(0x00e00820, return_constant_00e00820() -> u32),
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
        fn_00e000e0_returns_first_word_of_built_object,
        0x00e0_00e0,
        0x00d5_0c00
    );
    first_word_test!(
        fn_00e003b0_returns_first_word_of_built_object,
        0x00e0_03b0,
        0x00e0_03d0
    );
    first_word_test!(
        fn_00e004d0_returns_first_word_of_built_object,
        0x00e0_04d0,
        0x00e0_04f0
    );
    first_word_test!(
        fn_00e005f0_returns_first_word_of_built_object,
        0x00e0_05f0,
        0x00e0_0610
    );
    first_word_test!(
        fn_00e006b0_returns_first_word_of_built_object,
        0x00e0_06b0,
        0x00e0_06d0
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
    fn fn_00e000e0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_0c00, 0x0a0b_0c0d);
        let result = e.call(0x00e0_00e0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00150_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0150, &args![]).u32(), 0x010c_6cdc);
    }

    #[test]
    fn return_constant_00e00190_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0190, &args![]).u32(), 0x010e_81a4);
    }

    #[test]
    fn fn_00e001f0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00e0_01f0, &args![]).u32();
        assert_eq!(result, 0x010d_79c4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn fn_00e00290_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let frame = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(frame + offset, 0xffff_ffff);
        }
        e.mem.free(frame);
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x28], true);
        let result = e.call(0x00e0_0290, &args![]).u32();
        assert_eq!(result, 0x010d_79e8);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c040);
        }
        assert_eq!(calls[0].3, [0, 0, 0x8000]);
    }

    #[test]
    fn return_constant_00e00320_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0320, &args![]).u32(), 0x010c_6ea4);
    }

    #[test]
    fn return_constant_00e00360_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0360, &args![]).u32(), 0x010e_83b4);
    }

    #[test]
    fn fn_00e003b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_03d0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_03b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00480_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0480, &args![]).u32(), 0x010c_6dc4);
    }

    #[test]
    fn fn_00e004d0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_04f0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_04d0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e005a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_05a0, &args![]).u32(), 0x010c_749c);
    }

    #[test]
    fn fn_00e005f0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0610, 0x0a0b_0c0d);
        let result = e.call(0x00e0_05f0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00e006b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_06d0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_06b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00780_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0780, &args![]).u32(), 0x010c_c41c);
    }

    #[test]
    fn return_constant_00e007e0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_07e0, &args![]).u32(), 0x010c_b5ac);
    }

    #[test]
    fn return_constant_00e00820_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0820, &args![]).u32(), 0x010e_8824);
    }
}
