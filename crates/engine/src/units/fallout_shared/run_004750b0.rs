//! `fallout shared/run_004750b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSClipRoundsForm@@`).
const VTABLE: u32 = 0x0101_a644;
/// The `BGSClipRoundsForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `00475100`).
const INITIALIZE_DATA: u32 = 0x0047_5100;

// Translated from 004750b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSClipRoundsForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSClipRoundsForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_004750b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004750b0, fn_004750b0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004750b0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004750b0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0047_5100, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_a644)],
        });
    }
}
