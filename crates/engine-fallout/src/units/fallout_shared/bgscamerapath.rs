//! `fallout shared/bgscamerapath.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058a380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058a380 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5c (0x38 bytes); `ECX` is the object.
/// The engine map names it `BGSCameraPath::BGSCameraPath` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_245c`) is `BGSCameraPath`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058a380(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_245c);
    e.call(0x0068_0890, &args![base + 0x18]);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.call(0x004f_15a0, &args![this, 0x5cu32]);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.call(0x0058_a620, &args![this]);
    this
}

// Translated from 0058a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058a450` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058a420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_a450, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_a380, fn_0058a380(Ptr) -> Ptr),
        entry!(0x0058_a420, fn_0058a420(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058a380_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_a380,
            size: 0x38,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0068_0890, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x20)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5c)]),
                (0x0058_a620, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_245c), (0x2c, 4, 0x0)],
        });
    }
    #[test]
    fn at_0058a420_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_a450, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_a420, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_a420)
                .collect();
            let mut expected = vec![(0x0058_a450, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
