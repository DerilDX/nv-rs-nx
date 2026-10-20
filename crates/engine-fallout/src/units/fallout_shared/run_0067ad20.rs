//! `fallout shared/run_0067ad20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESPackageData`'s vtable (RTTI `.?AVTESPackageData@@`).
pub(crate) const TES_PACKAGE_DATA_VTABLE: u32 = 0x0106_8884;

// Translated from 0067ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackageData` constructor (the class of the vtable `01068884` it
/// stores, by its RTTI): stores the vtable and returns `this`.
pub fn fn_0067ad20(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), TES_PACKAGE_DATA_VTABLE);
    this
}

// Translated from 0067ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackageData` destructor body: stores the vtable `01068884` back.
pub fn fn_0067ad70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_PACKAGE_DATA_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_ad20, fn_0067ad20(Ptr) -> Ptr),
        entry!(0x0067_ad70, fn_0067ad70(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn the_constructor_and_destructor_store_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234);
        assert_eq!(e.call(0x0067_ad20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TES_PACKAGE_DATA_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x0067_ad70, &args![this]);
        assert_eq!(e.mem.u32(this), TES_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x1234);
    }
}
