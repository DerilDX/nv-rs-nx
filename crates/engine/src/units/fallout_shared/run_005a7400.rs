//! `fallout shared/run_005a7400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a77b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a77b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6b (0x28 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_69dc`) is `TESRecipeCategory`, so this is the constructor of `TESRecipeCategory`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a77b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_69dc);
    e.mem.set_u32(base + 0x18, 0x0103_69c8);
    e.call(0x004f_15a0, &args![this, 0x6bu32]);
    e.call(0x005a_7840, &args![this, 0x1u32]);
    this
}

// Translated from 005a7c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7c50 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6a (0x5c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_6b2c`) is `TESRecipe`, so this is the constructor of `TESRecipe`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a7c50(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_6b2c);
    e.mem.set_u32(base + 0x18, 0x0103_6b18);
    e.call(0x0068_0890, &args![base + 0x34]);
    e.call(0x0096_a2d0, &args![base + 0x3c]);
    e.call(0x0096_a2d0, &args![base + 0x44]);
    e.call(0x0096_a2d0, &args![base + 0x4c]);
    e.call(0x004f_15a0, &args![this, 0x6au32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_77b0, fn_005a77b0(Ptr) -> Ptr),
        entry!(0x005a_7c50, fn_005a7c50(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005a77b0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_77b0,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6b)]),
                (0x005a_7840, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0103_69dc), (0x18, 4, 0x0103_69c8)],
        });
    }

    #[test]
    fn fn_005a7c50_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_7c50,
            size: 0x5c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0068_0890, &[A::T(0x34)]),
                (0x0096_a2d0, &[A::T(0x3c)]),
                (0x0096_a2d0, &[A::T(0x44)]),
                (0x0096_a2d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6a)]),
            ],
            stores: &[(0x0, 4, 0x0103_6b2c), (0x18, 4, 0x0103_6b18)],
        });
    }
}
