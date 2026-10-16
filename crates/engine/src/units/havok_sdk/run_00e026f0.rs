//! `Havok SDK/run_00e026f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e026f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a1ac.
pub fn return_constant_00e026f0(_e: &mut Engine) -> u32 {
    0x010e_a1ac
}

// Translated from 00e02730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x20-byte frame with
/// the constructor at `00e4af80` (`this` = the frame, one argument 0) and returns the word at the
/// start of the object, the vtable pointer the constructor stored there. The temporary is not
/// destroyed.
pub fn fn_00e02730(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, record| {
        e.call(0x00e4_af80, &args![record, 0u32]);
        e.mem.u32(record.addr())
    })
}

// Translated from 00e027a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_9314.
pub fn return_constant_00e027a0(_e: &mut Engine) -> u32 {
    0x010c_9314
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e026f0, return_constant_00e026f0() -> u32),
        entry!(0x00e02730, fn_00e02730() -> u32),
        entry!(0x00e027a0, return_constant_00e027a0() -> u32),
    ]
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
    fn return_constant_00e026f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_26f0, &args![]).u32(), 0x010e_a1ac);
    }

    #[test]
    fn fn_00e02730_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_af80, 0x0a0b_0c0d);
        let result = e.call(0x00e0_2730, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e027a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_27a0, &args![]).u32(), 0x010c_9314);
    }
}
