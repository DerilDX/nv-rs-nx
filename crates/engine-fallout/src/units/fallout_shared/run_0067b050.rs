//! `fallout shared/run_0067b050` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESDialoguePackageData`'s vtable (RTTI `.?AVTESDialoguePackageData@@`).
const TES_DIALOGUE_PACKAGE_DATA_VTABLE: u32 = 0x0106_88ac;
/// The `float` the constructor loads for +0x4 (100.0 in the exe).
const DEFAULT_DISTANCE: u32 = 0x0101_6410;

// Translated from 0067b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData` constructor (the class of the vtable `010688ac`
/// it stores, by its RTTI): the `TESPackageData` constructor (`0067ad20`),
/// the vtable, the `float` at `01016410` into +0x4 (`FLD`/`FSTP`), words
/// +0x8, +0xC, +0x1C = 0, bytes +0x10, +0x11, +0x18 = 0 and the float
/// +0x14 = 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0067b170(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(base, TES_DIALOGUE_PACKAGE_DATA_VTABLE);
    let distance = e.mem.u32(DEFAULT_DISTANCE);
    e.mem.set_u32(base + 0x4, x87_quiet_bits(distance));
    e.mem.set_u32(base + 0x8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    e.mem.set_u8(base + 0x11, 0);
    e.mem.set_u8(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_f32(base + 0x14, 0.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_b170, fn_0067b170(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067b170_builds_the_dialogue_package_data() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(DEFAULT_DISTANCE, 100.0);
        e.register(0x0067_ad20, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        for i in 0..0x24 {
            e.mem.set_u8(this + i, 0xcc);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_b170, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), TES_DIALOGUE_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.f32(this + 4), 100.0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.bytes(this + 0x10, 4), vec![0, 0, 0xcc, 0xcc]);
        assert_eq!(e.mem.f32(this + 0x14), 0.0);
        assert_eq!(e.mem.bytes(this + 0x18, 4), vec![0, 0xcc, 0xcc, 0xcc]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0xcccc_cccc);
    }
}
