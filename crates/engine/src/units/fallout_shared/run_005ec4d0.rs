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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_d680, fn_005ed680(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
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
}
