//! `Havok SDK/run_00d4b1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d4b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00d4a170` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b1a0(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00d4_a170, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00d49c40` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b200(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00d4_9c40, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x110-byte stack buffer by calling `00d4a180` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b260(e: &mut Engine) -> u32 {
    e.with_stack(0x110, |e, buffer| {
        e.call(0x00d4_a180, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xf0-byte stack buffer by calling `00d56f10` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b330(e: &mut Engine) -> u32 {
    e.with_stack(0xf0, |e, buffer| {
        e.call(0x00d5_6f10, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_b1a0, fn_00d4b1a0() -> u32),
        entry!(0x00d4_b200, fn_00d4b200() -> u32),
        entry!(0x00d4_b260, fn_00d4b260() -> u32),
        entry!(0x00d4_b330, fn_00d4b330() -> u32),
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
        fn_00d4b1a0_returns_first_word_of_built_object,
        0x00d4_b1a0,
        0x00d4_a170
    );
    first_word_test!(
        fn_00d4b200_returns_first_word_of_built_object,
        0x00d4_b200,
        0x00d4_9c40
    );
    first_word_test!(
        fn_00d4b260_returns_first_word_of_built_object,
        0x00d4_b260,
        0x00d4_a180
    );
    first_word_test!(
        fn_00d4b330_returns_first_word_of_built_object,
        0x00d4_b330,
        0x00d5_6f10
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
    fn fn_00d4b1a0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_a170, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b1a0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b200_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_9c40, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b200, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b260_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_a180, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b260, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b330_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_6f10, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b330, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}
