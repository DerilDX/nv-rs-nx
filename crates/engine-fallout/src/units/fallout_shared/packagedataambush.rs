//! `fallout shared/packagedataambush.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00670b30` (`thiscall`, `RET 4`; engine map `PackageLocation::~PackageLocation`): run
/// with 1 on the location at +4.
const LOCATION_DELETE: u32 = 0x0067_0b30;
/// `0067ad70` (`thiscall`, no stack word): the base destructor.
const PACKAGE_DATA_BASE_DESTRUCTOR: u32 = 0x0067_ad70;

// Translated from 0067ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmbushPackageData::scalar deleting destructor` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): the destructor [`fn_0067adc0`] and, for bit 0 of `flags`,
/// `operator delete` on `this`. Returns `this`.
pub fn tes_ambush_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0067adc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0067adc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067adc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f00ba8, state variable) is not translated
/// Unnamed in the engine map (unit `packagedataambush.cpp`, `thiscall`, no stack word): the
/// `TESAmbushPackageData` destructor. Sets the vtable `01068860`, deletes the location at +4
/// when set (`00670b30(1)`), then the base destructor `0067ad70`.
pub fn fn_0067adc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_8860);
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(LOCATION_DELETE, &args![location, 1u32]);
    }
    e.call(PACKAGE_DATA_BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067_ad90,
            tes_ambush_package_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0067_adc0, fn_0067adc0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_deletes_the_location_then_runs_the_base() {
        let mut e = Engine::new();
        for callee in [
            OPERATOR_DELETE,
            LOCATION_DELETE,
            PACKAGE_DATA_BASE_DESTRUCTOR,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_ad90, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LOCATION_DELETE, vec![0x6000, 1]),
                (PACKAGE_DATA_BASE_DESTRUCTOR, vec![this]),
                (OPERATOR_DELETE, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0106_8860);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_ad90, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(PACKAGE_DATA_BASE_DESTRUCTOR, vec![this])]);
    }
}
