//! `fallout/ai/run_008cfbc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d0270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the object `00906dc0` builds (unnamed in the engine
/// map; the class is unconfirmed). `ECX` is the object. It stores the virtual
/// table `01085e2c`, runs `008c5e20` on the member at `+4`, clears the word at
/// `+0x2c` and returns `this`.
pub fn fn_008d0270(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0108_5e2c);
    e.call(0x008c_5e20, &args![base + 4]);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// Callees of `fn_008cffa0`: `00620b80` (`thiscall`, no arguments, answers an object),
/// `00a29680` (`thiscall`, no arguments; an empty function in this build), `005609f0`
/// (`thiscall`, no arguments, answers a byte) and, for `fn_008cfff0`, `00622570`
/// (`thiscall`, `(flag)`).
const OBJECT_00620B80: u32 = 0x0062_0b80;
const EMPTY_00A29680: u32 = 0x00a2_9680;
const BYTE_OF_005609F0: u32 = 0x0056_09f0;
const SET_FLAG_00622570: u32 = 0x0062_2570;

// Translated from 008cfff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&result)`): `00622570` on `result` with whether the
/// word at `this + 8` is non-zero. Answers `result`.
pub fn fn_008cfff0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let set = u32::from(e.mem.u32(this.addr() + 8) != 0);
    e.call(SET_FLAG_00622570, &args![result, set]);
    result
}

// Translated from 008cffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): false when `00620b80` answers no object;
/// otherwise, between two calls of `00a29680` on `this`, the byte `005609f0` answers for the
/// local `fn_008cfff0` fills from the object.
pub fn fn_008cffa0(e: &mut Engine, this: Ptr) -> u8 {
    let object = e.call(OBJECT_00620B80, &args![this]).u32();
    if object == 0 {
        return 0;
    }
    e.call(EMPTY_00A29680, &args![this]);
    let answer = e.with_stack(4, |e, local| {
        let filled = fn_008cfff0(e, Ptr::new(object), local);
        e.call(BYTE_OF_005609F0, &args![filled]).u8()
    });
    e.call(EMPTY_00A29680, &args![this]);
    answer
}

// Translated from 008d01e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float getter (unnamed in the map; `thiscall`, no arguments): the float at
/// `this + 0x98`, in ST0.
pub fn fn_008d01e0(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x98))))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_0270, fn_008d0270(Ptr) -> Ptr),
        entry!(0x008cfff0, fn_008cfff0(Ptr, Ptr) -> Ptr),
        entry!(0x008cffa0, fn_008cffa0(Ptr) -> u8),
        entry!(0x008d01e0, fn_008d01e0(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_008d0270_builds_the_base() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008d_0270,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[(0x008c_5e20, &[A::T(4)])],
            stores: &[(0x00, 4, 0x0108_5e2c), (0x2c, 4, 0)],
        });
    }

    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_008cfff0_passes_whether_the_word_is_set() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(SET_FLAG_00622570, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x008cfff0, &args![this, 0x7000u32]).u32(), 0x7000);
        e.mem.set_u32(this + 8, 0x55);
        e.call(0x008cfff0, &args![this, 0x7000u32]);
        assert_eq!(
            calls_at(&e, SET_FLAG_00622570),
            vec![vec![0x7000, 0], vec![0x7000, 1]]
        );
    }

    #[test]
    fn fn_008cffa0_reads_the_objects_byte() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OBJECT_00620B80, |_, _| Ret::default());
        e.register(EMPTY_00A29680, |_, _| Ret::default());
        e.register(SET_FLAG_00622570, |e, a| {
            e.mem.set_u8(a[0], 0x40 + a[1] as u8);
            Ret::default()
        });
        e.register(BYTE_OF_005609F0, |e, a| Ret {
            eax: 0x100 | u32::from(e.mem.u8(a[0])),
            ..Ret::default()
        });
        assert_eq!(e.call(0x008cffa0, &args![0x5000u32]).u8(), 0);
        assert!(calls_at(&e, EMPTY_00A29680).is_empty());
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 1);
        e.register_double(OBJECT_00620B80, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        assert_eq!(e.call(0x008cffa0, &args![0x5000u32]).u8(), 0x41);
        assert_eq!(calls_at(&e, EMPTY_00A29680), vec![vec![0x5000]; 2]);
    }

    #[test]
    fn fn_008d01e0_reads_the_float_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x9c);
        e.mem.set_f32(this + 0x98, -2.0);
        assert_eq!(e.call(0x008d01e0, &args![this]).f64(), -2.0);
    }
}
