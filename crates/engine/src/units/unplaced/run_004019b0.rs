//! `(unplaced)/run_004019b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00401a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401a10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x77 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_15b4`) is `BGSHungerStage`, so this is the constructor of `BGSHungerStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00401a10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_15b4);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x77u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 004020f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004020f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x78 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_16fc`) is `BGSSleepDeprevationStage`, so this is the constructor of `BGSSleepDeprevationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004020f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_16fc);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x78u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 00402820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402820 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2f (0xd8 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_1964`) is `AlchemyItem`, so this is the constructor of `AlchemyItem`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00402820(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2b40, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.call(0x0040_2c20, &args![base + 0x6c]);
    e.call(0x0047_a130, &args![base + 0x78]);
    e.call(0x0048_cc50, &args![base + 0x88]);
    e.call(0x0048_eac0, &args![base + 0x94]);
    e.call(0x0047_9320, &args![base + 0x9c]);
    e.call(0x0047_5350, &args![base + 0xa4]);
    e.call(0x0047_b4f0, &args![base + 0xac]);
    e.mem.set_u32(base, 0x0101_1964);
    e.mem.set_u32(base + 0x30, 0x0101_191c);
    e.mem.set_u32(base + 0x3c, 0x0101_1900);
    e.mem.set_u32(base + 0x4c, 0x0101_18dc);
    e.mem.set_u32(base + 0x6c, 0x0101_18bc);
    e.mem.set_u32(base + 0x78, 0x0101_18a8);
    e.mem.set_u32(base + 0x88, 0x0101_1894);
    e.mem.set_u32(base + 0x94, 0x0101_1880);
    e.mem.set_u32(base + 0x9c, 0x0101_186c);
    e.mem.set_u32(base + 0xa4, 0x0101_1858);
    e.mem.set_u32(base + 0xac, 0x0101_1844);
    e.call(0x0040_2c20, &args![base + 0xcc]);
    e.call(0x004f_15a0, &args![this, 0x2fu32]);
    e.call(0x0040_2ea0, &args![this]);
    this
}

/// The C runtime's `sqrt(double)` (`00ec6054`, `cdecl`, the result in
/// `ST0`).
const SQRT: u32 = 0x00ec_6054;

// Translated from 004019b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `cdecl` (one float): passes the float through the x87 stack to
/// [`fn_004019d0`] and returns its result in `ST0`.
pub fn fn_004019b0(e: &mut Engine, value: u32) -> f64 {
    let passed = (x87_load32(value) as f32).to_bits();
    fn_004019d0(e, passed)
}

// Translated from 004019d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `cdecl` (one float): the square root of the float as a double
/// (`sqrt`, `00ec6054`), rounded to a float (`FSTP float` then `FLD float`)
/// and returned in `ST0`. (Ghidra shows a `this`; `ECX` is never read.)
pub fn fn_004019d0(e: &mut Engine, value: u32) -> f64 {
    let root = e.call(SQRT, &args![x87_load32(value)]).f64();
    f64::from(root as f32)
}

// Translated from 00401a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00401ac0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00401a90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_1ac0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00402170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004021a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00402170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_21a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_1a10, fn_00401a10(Ptr) -> Ptr),
        entry!(0x0040_20f0, fn_004020f0(Ptr) -> Ptr),
        entry!(0x0040_2820, fn_00402820(Ptr) -> Ptr),
        entry!(0x004019b0, fn_004019b0(u32) -> f64),
        entry!(0x004019d0, fn_004019d0(u32) -> f64),
        entry!(0x0040_1a90, fn_00401a90(Ptr, u32) -> Ptr),
        entry!(0x0040_2170, fn_00402170(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00401a10_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_1a10,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x77)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_15b4)],
        });
    }

    #[test]
    fn fn_004020f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_20f0,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x78)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_16fc)],
        });
    }

    #[test]
    fn fn_00402820_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2820,
            size: 0xd8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2b40, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x0040_2c20, &[A::T(0x6c)]),
                (0x0047_a130, &[A::T(0x78)]),
                (0x0048_cc50, &[A::T(0x88)]),
                (0x0048_eac0, &[A::T(0x94)]),
                (0x0047_9320, &[A::T(0x9c)]),
                (0x0047_5350, &[A::T(0xa4)]),
                (0x0047_b4f0, &[A::T(0xac)]),
                (0x0040_2c20, &[A::T(0xcc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2f)]),
                (0x0040_2ea0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_1964),
                (0x30, 4, 0x0101_191c),
                (0x3c, 4, 0x0101_1900),
                (0x4c, 4, 0x0101_18dc),
                (0x6c, 4, 0x0101_18bc),
                (0x78, 4, 0x0101_18a8),
                (0x88, 4, 0x0101_1894),
                (0x94, 4, 0x0101_1880),
                (0x9c, 4, 0x0101_186c),
                (0xa4, 4, 0x0101_1858),
                (0xac, 4, 0x0101_1844),
            ],
        });
    }

    #[test]
    fn the_square_root_of_a_float_rounded_to_a_float() {
        let mut e = Engine::new();
        e.register(SQRT, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            v.sqrt().into_ret()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x004019b0, &[2.0f32.to_bits()]).f64();
        assert_eq!(r, f64::from(2.0f64.sqrt() as f32));
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, SQRT);
        assert_eq!(log[1].1, args![2.0f64]);
    }
    #[test]
    fn at_00401a90_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_1ac0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_1a90, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_1a90)
                .collect();
            let mut expected = vec![(0x0040_1ac0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00402170_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_21a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_2170, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_2170)
                .collect();
            let mut expected = vec![(0x0040_21a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
