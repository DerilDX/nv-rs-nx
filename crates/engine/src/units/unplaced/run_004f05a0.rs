//! `(unplaced)/run_004f05a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f0770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f0770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x37 (0x38 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_397c`) is `TESRegion`, so this is the constructor of `TESRegion`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It allocates two objects with `operator new` (0xc and 8 bytes) and, when an allocation succeeds, constructs it (`004f3330(object, 1)` and `0096a2d0(object)`) and keeps the result at `+0x18` and `+0x1c` (null otherwise); it passes three zero floats to `00416870` for the member at `+0x2c`, and copies the float at `01012054` to `+0x28`.
/// The exception-unwinding frame is not translated.
pub fn fn_004f0770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_397c);
    e.call(0x0041_6870, &args![base + 0x2c, 0x0u32, 0x0u32, 0x0u32]);
    let v3 = e.call(0x0040_1000, &args![0xcu32]).u32();
    let v5 = if v3 != 0 {
        e.call(0x004f_3330, &args![v3, 0x1u32]).u32()
    } else {
        0x0
    };
    e.mem.set_u32(base + 0x18, v5);
    let v6 = e.call(0x0040_1000, &args![0x8u32]).u32();
    let v8 = if v6 != 0 {
        e.call(0x0096_a2d0, &args![v6]).u32()
    } else {
        0x0
    };
    e.mem.set_u32(base + 0x1c, v8);
    e.call(0x004f_15a0, &args![this, 0x37u32]);
    e.mem.set_u32(base + 0x20, 0x0);
    e.mem.set_u32(base + 0x24, 0x0);
    let f1 = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x28, f1);
    this
}

// Translated from 004f0890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f08c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f0890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_08c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f3360` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f09e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_3360, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f7740` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f0a10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_7740, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_0770, fn_004f0770(Ptr) -> Ptr),
        entry!(0x004f_0890, fn_004f0890(Ptr, u32) -> Ptr),
        entry!(0x004f_09e0, fn_004f09e0(Ptr, u32) -> Ptr),
        entry!(0x004f_0a10, fn_004f0a10(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f0770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_0770,
            size: 0x38,
            globals: &[(0x0101_2054, 0x0100_0001)],
            returns: &[
                (0x0040_1000, 0x2000_2000),
                (0x004f_3330, 0x2000_3000),
                (0x0096_a2d0, 0x2000_4000),
            ],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0041_6870, &[A::T(0x2c), A::V(0x0), A::V(0x0), A::V(0x0)]),
                (0x0040_1000, &[A::V(0xc)]),
                (0x004f_3330, &[A::V(0x2000_2000), A::V(0x1)]),
                (0x0040_1000, &[A::V(0x8)]),
                (0x0096_a2d0, &[A::V(0x2000_2000)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x37)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_397c),
                (0x18, 4, 0x2000_3000),
                (0x1c, 4, 0x2000_4000),
                (0x20, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0100_0001),
            ],
        });
    }

    #[test]
    fn fn_004f0770_without_memory_leaves_the_objects_null() {
        // `operator new` answers null: neither object is constructed.
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_0770,
            size: 0x38,
            globals: &[(0x0101_2054, 0x0100_0001)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0041_6870, &[A::T(0x2c), A::V(0x0), A::V(0x0), A::V(0x0)]),
                (0x0040_1000, &[A::V(0xc)]),
                (0x0040_1000, &[A::V(0x8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x37)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_397c),
                (0x18, 4, 0x0),
                (0x1c, 4, 0x0),
                (0x20, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0100_0001),
            ],
        });
    }
    #[test]
    fn at_004f0890_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_08c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_0890, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_0890)
                .collect();
            let mut expected = vec![(0x004f_08c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f09e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_3360, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_09e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_09e0)
                .collect();
            let mut expected = vec![(0x004f_3360, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f0a10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_7740, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_0a10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_0a10)
                .collect();
            let mut expected = vec![(0x004f_7740, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
