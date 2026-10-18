//! `fallout shared/run_0047b470` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSPickupPutdownSounds@@`).
const VTABLE: u32 = 0x0101_abd8;
/// The `BGSPickupPutdownSounds::InitializeDataComponent` body the constructor calls (Xbox PDB name of `0047b540`).
const INITIALIZE_DATA: u32 = 0x0047_b540;

// Translated from 0047b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSPickupPutdownSounds` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSPickupPutdownSounds@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0047b4f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047b4f0, fn_0047b4f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0047b4f0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0047b4f0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0047_b540, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_abd8)],
        });
    }
}
