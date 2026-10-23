//! `BSMenu/radialtile.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TileImage::TileImage` (engine map, `0070b5d0`; thiscall, no stack words).
const TILE_IMAGE_CONSTRUCT: u32 = 0x0070_b5d0;
/// `RadialTile`'s vtable (RTTI `.?AVRadialTile@@`).
const RADIAL_TILE_VTABLE: u32 = 0x0109_5750;

// Translated from 00a21660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadialTile::RadialTile` (engine map; thiscall, no stack words): constructs the
/// `TileImage` base (`0070b5d0`), then stores `RadialTile`'s vtable at `+0`. Returns `this`.
pub fn radial_tile_radial_tile(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TILE_IMAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), RADIAL_TILE_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a2_1660, radial_tile_radial_tile(Ptr) -> Ptr)]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn radial_tile_constructs_the_base_then_sets_its_vtable() {
        let mut e = Engine::new();
        e.register(TILE_IMAGE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111_1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        let back = e.call(0x00a2_1660, &args![this]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), RADIAL_TILE_VTABLE);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == TILE_IMAGE_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this]);
    }
}
