//! `fallout shared/region system/run_004f3610` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TESRegionDataManager` vtable (RTTI `.?AVTESRegionDataManager@@`).
const REGION_DATA_MANAGER_VTABLE: u32 = 0x0102_3ce4;

layout! {
    /// `TESRegionDataManager` (Xbox PDB), 8 bytes.
    pub struct TESRegionDataManager: 0x08 {
        /// `pLastLoadedRegion` (Xbox PDB): `TESRegion*`.
        0x04 pLastLoadedRegion: u32,
    }
}

// Translated from 004f3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRegionDataManager` constructor (no Xbox PDB name at this
/// address): stores the vtable, clears `pLastLoadedRegion` and returns
/// `this`.
pub fn fn_004f3660(e: &mut Engine, this: Ptr<TESRegionDataManager>) -> Ptr<TESRegionDataManager> {
    e.mem.set_u32(this.addr(), REGION_DATA_MANAGER_VTABLE);
    e.set(this, TESRegionDataManager::pLastLoadedRegion, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x004f_3660,
        fn_004f3660(Ptr<TESRegionDataManager>) -> Ptr<TESRegionDataManager>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_vtable_and_clears_the_region() {
        let mut e = Engine::new();
        let this: Ptr<TESRegionDataManager> = e.new_object();
        e.set(this, TESRegionDataManager::pLastLoadedRegion, 0x1234);
        let back = e.call(0x004f_3660, &args![this]).u32();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), REGION_DATA_MANAGER_VTABLE);
        assert_eq!(e.get(this, TESRegionDataManager::pLastLoadedRegion), 0);
    }
}
