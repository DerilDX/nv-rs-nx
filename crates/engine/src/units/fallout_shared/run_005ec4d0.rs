//! `fallout shared/run_005ec4d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ed680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed680 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x63 (0x148 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_70ec`) is `BGSRagdoll`, so this is the constructor of `BGSRagdoll`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005ed680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_70ec);
    e.mem.set_u32(base + 0x18, 0x0104_70c4);
    e.call(0x005e_d760, &args![base + 0x40]);
    e.call(0x005e_d850, &args![base + 0x90]);
    e.call(0x005e_d910, &args![base + 0xd0]);
    e.call(0x005e_d980, &args![base + 0xf4]);
    e.call(0x005e_d9f0, &args![base + 0x118]);
    e.call(0x004f_15a0, &args![this, 0x63u32]);
    e.call(0x005e_dd00, &args![this]);
    this
}

/// The function `fn_005edaf0` calls on the member (`005eee60`).
const CALLEE_005EEE60: u32 = 0x005e_ee60;

// Translated from 005edaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `005eee60` with `ECX` = `this + 0x40` and passes on nothing (the callee returns no value).
pub fn fn_005edaf0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_005EEE60, &args![this.byte_add(0x40)]);
}

// Translated from 005ec4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0060ffe0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005ec4d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0060_ffe0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005edac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005edc30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005edac0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_dc30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_d680, fn_005ed680(Ptr) -> Ptr),
        entry!(0x005e_daf0, fn_005edaf0(Ptr)),
        entry!(0x005e_c4d0, fn_005ec4d0(Ptr, u32) -> Ptr),
        entry!(0x005e_dac0, fn_005edac0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005ed680_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_d680,
            size: 0x148,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x005e_d760, &[A::T(0x40)]),
                (0x005e_d850, &[A::T(0x90)]),
                (0x005e_d910, &[A::T(0xd0)]),
                (0x005e_d980, &[A::T(0xf4)]),
                (0x005e_d9f0, &[A::T(0x118)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x63)]),
                (0x005e_dd00, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0104_70ec), (0x18, 4, 0x0104_70c4)],
        });
    }

    #[test]
    fn l68d_005edaf0_passes_this_plus_40_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_005EEE60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_daf0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_005EEE60).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x40]);
    }
    #[test]
    fn at_005ec4d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0060_ffe0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_c4d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_c4d0)
                .collect();
            let mut expected = vec![(0x0060_ffe0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005edac0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_dc30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_dac0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_dac0)
                .collect();
            let mut expected = vec![(0x005e_dc30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
