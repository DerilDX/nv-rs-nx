//! `Havok SDK/run_00e00ed0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e00f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e00fb0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e00f90(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e0_0fb0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0e5c.
pub fn return_constant_00e00ed0(_e: &mut Engine) -> u32 {
    0x010d_0e5c
}

// Translated from 00e00f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_102c.
pub fn return_constant_00e00f40(_e: &mut Engine) -> u32 {
    0x010d_102c
}

// Translated from 00e010b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_10c4.
pub fn return_constant_00e010b0(_e: &mut Engine) -> u32 {
    0x010d_10c4
}

// Translated from 00e01130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_901c.
pub fn return_constant_00e01130(_e: &mut Engine) -> u32 {
    0x010e_901c
}

// Translated from 00e00fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010e8e94`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010e8e74`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, 7) and the byte at `+0x15` (3,
/// once). Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e00fb0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
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
    e.mem.set_u32(base, 0x010e_8e94);
    e.mem.set_u32(base + 0x10, 0x010e_8e74);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_0f90, fn_00e00f90() -> u32),
        entry!(0x00e00ed0, return_constant_00e00ed0() -> u32),
        entry!(0x00e00f40, return_constant_00e00f40() -> u32),
        entry!(0x00e010b0, return_constant_00e010b0() -> u32),
        entry!(0x00e01130, return_constant_00e01130() -> u32),
        entry!(0x00e0_0fb0, fn_00e00fb0(Ptr, u32) -> Ptr),
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
        fn_00e00f90_returns_first_word_of_built_object,
        0x00e0_0f90,
        0x00e0_0fb0
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
    fn return_constant_00e00ed0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0ed0, &args![]).u32(), 0x010d_0e5c);
    }

    #[test]
    fn return_constant_00e00f40_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0f40, &args![]).u32(), 0x010d_102c);
    }

    #[test]
    fn fn_00e00f90_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0fb0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0f90, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e010b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_10b0, &args![]).u32(), 0x010d_10c4);
    }

    #[test]
    fn return_constant_00e01130_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1130, &args![]).u32(), 0x010e_901c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Mode 0 leaves +0xc and the byte at +0x15 alone; another mode ends with 7 and 3.
    #[test]
    fn fn_00e00fb0_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00e0_0fb0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_8e94);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_8e74);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.call(0x00e0_0fb0, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 7);
        assert_eq!(e.mem.u8(this + 0x15), 3);
    }
}
