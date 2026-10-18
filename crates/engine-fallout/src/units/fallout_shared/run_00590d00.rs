//! `fallout shared/run_00590d00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the flag word at `+0x38` of `this` is set. `ShowMessage`
/// calls it on a message form to tell a message box from a plain message;
/// the engine map has no name for it.
pub fn fn_00590d00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x38) & 1 != 0
}

// Translated from 00591200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591200 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x66 (0x34 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_397c`) is `BGSMusicType`, so this is the constructor of `BGSMusicType`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the float at `0103395c` into `+0x24`.
/// The exception-unwinding frame is not translated.
pub fn fn_00591200(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0059_12d0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_397c);
    e.mem.set_u32(base + 0x18, 0x0103_3964);
    e.call(0x0096_a2d0, &args![base + 0x28]);
    e.call(0x004f_15a0, &args![this, 0x66u32]);
    let f1 = e.global::<f32>(0x0103_395c);
    e.mem.set_f32(base + 0x24, f1);
    this
}

// Translated from 00591b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591b70 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5a (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_3b34`) is `BGSRadiationStage`, so this is the constructor of `BGSRadiationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00591b70(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_3b34);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x5au32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 005912d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x18` of `BGSMusicType` (`00591200` calls it): runs
/// `00402d30` on the object, stores the vtable `0x0103_3ab8` and runs `004037b0` on the string
/// at `+0x4`. Returns `this`. The engine map gives it no name.
pub fn fn_005912d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(base, 0x0103_3ab8);
    e.call(0x0040_37b0, &args![base + 0x4]);
    this
}

// Translated from 00591140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591170` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00591140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1170, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005912a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591320` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005912a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1320, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00591bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591c20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00591bf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1c20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00590d00, fn_00590d00(Ptr) -> bool),
        entry!(0x0059_12d0, fn_005912d0(Ptr) -> Ptr),
        entry!(0x0059_1200, fn_00591200(Ptr) -> Ptr),
        entry!(0x0059_1b70, fn_00591b70(Ptr) -> Ptr),
        entry!(0x0059_1140, fn_00591140(Ptr, u32) -> Ptr),
        entry!(0x0059_12a0, fn_005912a0(Ptr, u32) -> Ptr),
        entry!(0x0059_1bf0, fn_00591bf0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bit_zero_of_the_word_at_0x38() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0b10);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0xffff_ffff);
        assert!(e.call(0x0059_0d00, &args![object]).bool());
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00591200_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_1200,
            size: 0x34,
            globals: &[(0x0103_395c, 0x0100_0001)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0059_12d0, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x28)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x66)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_397c),
                (0x18, 4, 0x0103_3964),
                (0x24, 4, 0x0100_0001),
            ],
        });
    }

    #[test]
    fn fn_00591b70_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_1b70,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5a)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_3b34)],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005912d0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_12d0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0040_37b0, &[A::T(0x4)])],
            stores: &[(0x0, 4, 0x0103_3ab8)],
        });
    }
    #[test]
    fn at_00591140_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1170, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_1140, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_1140)
                .collect();
            let mut expected = vec![(0x0059_1170, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005912a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1320, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_12a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_12a0)
                .collect();
            let mut expected = vec![(0x0059_1320, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00591bf0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1c20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_1bf0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_1bf0)
                .collect();
            let mut expected = vec![(0x0059_1c20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
