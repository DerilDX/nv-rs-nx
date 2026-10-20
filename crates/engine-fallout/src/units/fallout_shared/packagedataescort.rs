//! `fallout shared/packagedataescort.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESEscortPackageData`'s vtable (RTTI `.?AVTESEscortPackageData@@`).
const TES_ESCORT_PACKAGE_DATA_VTABLE: u32 = 0x0106_8960;
/// `TESPackageData`'s constructor (`0067ad20`).
const TES_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
/// `TESPackageData`'s destructor body (`0067ad70`).
const TES_PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;

// Translated from 0067bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData::`scalar deleting destructor'` (Xbox PDB): the
/// destructor body [`fn_0067bca0`], then the release `00401030` when bit 0
/// of `flags` is set. Returns `this`.
pub fn tes_escort_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0067bca0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0067bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData` constructor from a value: the `TESPackageData`
/// constructor, the vtable `01068960`, +0x4 = 0 and +0x8 = the word at
/// `source` (one stack word). Returns `this`.
pub fn fn_0067bc60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TES_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_ESCORT_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    let value = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr() + 8, value);
    this
}

// Translated from 0067bca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067bca0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESEscortPackageData` destructor body: the vtable `01068960`, then the
/// `PackageLocation` at +0x4, when there is one, deleted through
/// `00670b30` (`PackageLocation::~PackageLocation` in the engine map, called
/// with 1: the deleting form), then the `TESPackageData` destructor body.
/// The pointer at +0x4 is left as it was.
pub fn fn_0067bca0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_ESCORT_PACKAGE_DATA_VTABLE);
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.call(TES_PACKAGE_DATA_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067_bc30,
            tes_escort_package_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0067_bc60, fn_0067bc60(Ptr, Ptr) -> Ptr),
        entry!(0x0067_bca0, fn_0067bca0(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for callee in [
            TES_PACKAGE_DATA_CONSTRUCT,
            TES_PACKAGE_DATA_DESTRUCT,
            0x0067_0b30,
            0x0040_1030,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_0067bc60_copies_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 345);
        e.mem.set_u32(this + 4, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_bc60, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(TES_PACKAGE_DATA_CONSTRUCT, vec![this])));
        assert_eq!(e.mem.u32(this), TES_ESCORT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 345);
    }

    #[test]
    fn fn_0067bca0_deletes_the_location_when_present() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0067_bca0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0067_0b30, vec![0x7000, 1]),
                (TES_PACKAGE_DATA_DESTRUCT, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), TES_ESCORT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x7000);

        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_bca0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(TES_PACKAGE_DATA_DESTRUCT, vec![this])]);
    }

    #[test]
    fn the_deleting_destructor_releases_only_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0067_bc30, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let releases: Vec<_> = log.iter().filter(|c| c.0 == 0x0040_1030).collect();
            assert_eq!(releases.len(), (flags & 1) as usize);
            assert!(log.contains(&(TES_PACKAGE_DATA_DESTRUCT, vec![this])));
        }
    }
}
