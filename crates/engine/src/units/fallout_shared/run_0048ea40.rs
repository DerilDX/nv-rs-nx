//! `fallout shared/run_0048ea40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESWeightForm@@`).
const VTABLE: u32 = 0x0101_d874;
/// The `TESWeightForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `00910b10`).
const INITIALIZE_DATA: u32 = 0x0091_0b10;

// Translated from 0048eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESWeightForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESWeightForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0048eac0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048eac0, fn_0048eac0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048eac0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048eac0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0091_0b10, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_d874)],
        });
    }
}
