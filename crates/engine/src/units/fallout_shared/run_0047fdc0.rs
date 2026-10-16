//! `fallout shared/run_0047fdc0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESAttackDamageForm@@`).
const VTABLE: u32 = 0x0101_bf68;
/// The `TESAttackDamageForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `0047fe10`).
const INITIALIZE_DATA: u32 = 0x0047_fe10;

// Translated from 0047fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESAttackDamageForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESAttackDamageForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0047fdc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047fdc0, fn_0047fdc0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0047fdc0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0047fdc0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0047_fe10, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_bf68)],
        });
    }
}
