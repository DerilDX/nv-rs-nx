//! `Havok SDK/run_00dffdb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00cdf760` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfffa0(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00cd_f760, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dffdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_c0d8.
pub fn return_constant_00dffdb0(_e: &mut Engine) -> u32 {
    0x010c_c0d8
}

// Translated from 00dffe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6e34.
pub fn return_constant_00dffe10(_e: &mut Engine) -> u32 {
    0x010c_6e34
}

// Translated from 00dffe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_3838.
pub fn return_constant_00dffe50(_e: &mut Engine) -> u32 {
    0x010d_3838
}

// Translated from 00dffeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6d54.
pub fn return_constant_00dffeb0(_e: &mut Engine) -> u32 {
    0x010c_6d54
}

// Translated from 00dffef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b798.
pub fn return_constant_00dffef0(_e: &mut Engine) -> u32 {
    0x010c_b798
}

// Translated from 00dfff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_a004.
pub fn return_constant_00dfff50(_e: &mut Engine) -> u32 {
    0x010d_a004
}

// Translated from 00dffff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_80b4.
pub fn return_constant_00dffff0(_e: &mut Engine) -> u32 {
    0x010e_80b4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_ffa0, fn_00dfffa0() -> u32),
        entry!(0x00dffdb0, return_constant_00dffdb0() -> u32),
        entry!(0x00dffe10, return_constant_00dffe10() -> u32),
        entry!(0x00dffe50, return_constant_00dffe50() -> u32),
        entry!(0x00dffeb0, return_constant_00dffeb0() -> u32),
        entry!(0x00dffef0, return_constant_00dffef0() -> u32),
        entry!(0x00dfff50, return_constant_00dfff50() -> u32),
        entry!(0x00dffff0, return_constant_00dffff0() -> u32),
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
        fn_00dfffa0_returns_first_word_of_built_object,
        0x00df_ffa0,
        0x00cd_f760
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

    #[test]
    fn return_constant_00dffdb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fdb0, &args![]).u32(), 0x010c_c0d8);
    }

    #[test]
    fn return_constant_00dffe10_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fe10, &args![]).u32(), 0x010c_6e34);
    }

    #[test]
    fn return_constant_00dffe50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fe50, &args![]).u32(), 0x010d_3838);
    }

    #[test]
    fn return_constant_00dffeb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_feb0, &args![]).u32(), 0x010c_6d54);
    }

    #[test]
    fn return_constant_00dffef0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fef0, &args![]).u32(), 0x010c_b798);
    }

    #[test]
    fn return_constant_00dfff50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ff50, &args![]).u32(), 0x010d_a004);
    }

    #[test]
    fn fn_00dfffa0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00cd_f760, 0x0a0b_0c0d);
        let result = e.call(0x00df_ffa0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dffff0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fff0, &args![]).u32(), 0x010e_80b4);
    }
}
