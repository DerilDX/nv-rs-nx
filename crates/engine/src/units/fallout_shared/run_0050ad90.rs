//! `fallout shared/run_0050ad90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050adf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050adf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2c (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050adf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_886c);
    e.mem.set_u32(base + 0x30, 0x0102_884c);
    e.mem.set_u32(base + 0x4c, 0x0102_8828);
    e.call(0x004f_15a0, &args![this, 0x2c]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050b400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b400 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x34 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b400(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8a64);
    e.mem.set_u32(base + 0x30, 0x0102_8a44);
    e.call(0x004f_15a0, &args![this, 0x34]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 0050b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4c (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8c5c);
    e.mem.set_u32(base + 0x30, 0x0102_8c38);
    e.call(0x004f_15a0, &args![this, 0x4c]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_adf0, fn_0050adf0(Ptr) -> Ptr),
        entry!(0x0050_b400, fn_0050b400(Ptr) -> Ptr),
        entry!(0x0050_b8f0, fn_0050b8f0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050adf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_adf0,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2c)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_886c),
                (0x30, 4, 0x0102_884c),
                (0x4c, 4, 0x0102_8828),
            ],
        });
    }

    #[test]
    fn fn_0050b400_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b400,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x34)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_8a64), (0x30, 4, 0x0102_8a44)],
        });
    }

    #[test]
    fn fn_0050b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b8f0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4c)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0102_8c5c), (0x30, 4, 0x0102_8c38)],
        });
    }
}
