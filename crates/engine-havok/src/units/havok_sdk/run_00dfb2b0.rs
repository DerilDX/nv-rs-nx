//! `Havok SDK/run_00dfb2b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfb590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_dc5c.
pub fn return_constant_00dfb590(_e: &mut Engine) -> u32 {
    0x010d_dc5c
}

// Translated from 00dfb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00dfb2d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb2b0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00df_b2d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e2ea20` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb400(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e2_ea20, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e2ebc0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb460(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e2_ebc0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`; the class is unconfirmed). It writes the vtable word at `+0` with four
/// values in turn (`010305a8`, `010ce3c8`, `010d0d70`, `010d0dac`) and the word at `+0x30`
/// (`010c6fd0`); when `mode` is not zero the intermediate stages also write `+0xc` (0, then 8)
/// and the byte at `+0x10` (2, then 0). When `mode` is 1 it sets `+0xc` to `0xa` and copies the
/// 16 bytes at `[this + 0x14] + 0x10` to `this + 0x20` (`MOVAPS`, which faults on an address
/// that is not 16-byte aligned; here a plain copy) and the words at `[this + 0x14] + 0x20` and
/// `+ 0x24` to `this + 0x18` and `this + 0x1c`. Returns `this` (`EAX`).
pub fn fn_00dfb2d0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base, 0x010c_e3c8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 8);
        e.mem.set_u8(base + 0x10, 2);
    }
    e.mem.set_u32(base, 0x010d_0d70);
    if mode != 0 {
        e.mem.set_u8(base + 0x10, 0);
    }
    e.mem.set_u32(base, 0x010d_0dac);
    e.mem.set_u32(base + 0x30, 0x010c_6fd0);
    if mode == 1 {
        let source = e.mem.u32(base + 0x14);
        e.mem.set_u32(base + 0xc, 0xa);
        for lane in 0..4 {
            let word = e.mem.u32(source + 0x10 + 4 * lane);
            e.mem.set_u32(base + 0x20 + 4 * lane, word);
        }
        let low = e.mem.u32(source + 0x20);
        e.mem.set_u32(base + 0x18, low);
        let high = e.mem.u32(source + 0x24);
        e.mem.set_u32(base + 0x1c, high);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_b2b0, fn_00dfb2b0() -> u32),
        entry!(0x00df_b400, fn_00dfb400() -> u32),
        entry!(0x00df_b460, fn_00dfb460() -> u32),
        entry!(0x00dfb590, return_constant_00dfb590() -> u32),
        entry!(0x00df_b2d0, fn_00dfb2d0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfb590_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_b590, &args![]).u32(), 0x010d_dc5c);
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
        fn_00dfb2b0_returns_first_word_of_built_object,
        0x00df_b2b0,
        0x00df_b2d0
    );
    first_word_test!(
        fn_00dfb400_returns_first_word_of_built_object,
        0x00df_b400,
        0x00e2_ea20
    );
    first_word_test!(
        fn_00dfb460_returns_first_word_of_built_object,
        0x00df_b460,
        0x00e2_ebc0
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
    fn fn_00dfb2b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_b2d0, 0x0a0b_0c0d);
        let result = e.call(0x00df_b2b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfb400_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e2_ea20, 0x0a0b_0c0d);
        let result = e.call(0x00df_b400, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfb460_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e2_ebc0, 0x0a0b_0c0d);
        let result = e.call(0x00df_b460, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Mode 0 writes only the vtable (final value) and the word at +0x30.
    #[test]
    fn fn_00dfb2d0_mode_zero_writes_the_vtable_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x77; 0x40]);
        assert_eq!(e.call(0x00df_b2d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_0dac);
        assert_eq!(e.mem.u32(this + 0x30), 0x010c_6fd0);
        assert_eq!(e.mem.u32(this + 0xc), 0x7777_7777);
        assert_eq!(e.mem.u8(this + 0x10), 0x77);
        assert_eq!(e.mem.u32(this + 0x20), 0x7777_7777);
    }

    /// Mode 2 (not 0, not 1): the stages clear +0xc... to 8 and the byte to 0, no copy.
    #[test]
    fn fn_00dfb2d0_other_modes_leave_the_staged_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x77; 0x40]);
        e.call(0x00df_b2d0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this), 0x010d_0dac);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x7777_7777);
        assert_eq!(e.mem.u32(this + 0x18), 0x7777_7777);
    }

    /// Mode 1 also copies the source's vector and words and sets +0xc to 0xa.
    #[test]
    fn fn_00dfb2d0_mode_one_copies_from_the_source() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, source);
        for word in 0..12u32 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.call(0x00df_b2d0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xa);
        for lane in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * lane), 0x104 + lane);
        }
        assert_eq!(e.mem.u32(this + 0x18), 0x108);
        assert_eq!(e.mem.u32(this + 0x1c), 0x109);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }
}
