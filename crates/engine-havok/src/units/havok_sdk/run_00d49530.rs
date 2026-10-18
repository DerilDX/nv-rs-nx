//! `Havok SDK/run_00d49530` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d495f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_fb34.
pub fn return_constant_00d495f0(_e: &mut Engine) -> u32 {
    0x010c_fb34
}

// Translated from 00d49550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x220-byte frame with
/// the constructor at `00c9ce00` (`this` = the frame, one argument 0), does not destroy it and returns
/// the constant 0x010c_7888 (the word the constructor stored is not read back).
pub fn fn_00d49550(e: &mut Engine) -> u32 {
    e.with_stack(0x220, |e, record| {
        e.call(0x00c9_ce00, &args![record, 0u32]);
    });
    0x010c_7888
}

// Translated from 00d49530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `object` and `argument`): when `object` is not null calls
/// `00c9ce00` on it with `argument` (thiscall) and then writes the vtable word `0x010c_7888` at
/// `object + 0`. (The exe's wrapper around a base destructor; the class is unconfirmed.)
pub fn fn_00d49530(e: &mut Engine, object: u32, argument: u32) {
    if object != 0 {
        e.call(0x00c9_ce00, &args![object, argument]);
        e.mem.set_u32(object, 0x010c_7888);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d495f0, return_constant_00d495f0() -> u32),
        entry!(0x00d49550, fn_00d49550() -> u32),
        entry!(0x00d4_9530, fn_00d49530(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00d495f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_95f0, &args![]).u32(), 0x010c_fb34);
    }
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
    fn fn_00d49550_constructs_a_temporary_and_returns_the_constant() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00c9_ce00, 0x0a0b_0c0d);
        let result = e.call(0x00d4_9550, &args![]).u32();
        assert_eq!(result, 0x010c_7888);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d49530_runs_the_base_and_sets_the_vtable_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00c9_ce00, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d4_9530, &args![object, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00c9_ce00, vec![object, 0x33])]);
        assert_eq!(e.mem.u32(object), 0x010c_7888);
        e.call_log = Some(vec![]);
        e.call(0x00d4_9530, &args![0u32, 0x33u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
