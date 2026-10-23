//! `fallout/interface/menus/sleepwaitmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The menu's six tile slots start at +0x28 (one word each).
const TILES: u32 = 0x28;

// Translated from 007a8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::AttachTileByID` (Xbox PDB): stores `tile` in slot `id`
/// of the six tile slots at +0x28; ids outside 0..6 (signed) are ignored.
pub fn sleep_wait_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..6).contains(&id) {
        let slot = this
            .addr()
            .wrapping_add(TILES)
            .wrapping_add((id as u32).wrapping_mul(4));
        e.mem.set_u32(slot, tile);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x007a_8ab0,
        sleep_wait_menu_attach_tile_by_id(Ptr, i32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaches_only_ids_zero_to_five() {
        let mut e = Engine::new();
        let menu = e.mem.alloc(0x48);
        e.call(0x007a_8ab0, &args![menu, 0i32, 0x11u32]);
        e.call(0x007a_8ab0, &args![menu, 5i32, 0x55u32]);
        e.call(0x007a_8ab0, &args![menu, 6i32, 0x66u32]);
        e.call(0x007a_8ab0, &args![menu, -1i32, 0x99u32]);
        assert_eq!(e.mem.u32(menu + 0x28), 0x11);
        assert_eq!(e.mem.u32(menu + 0x3c), 0x55);
        assert_eq!(e.mem.u32(menu + 0x40), 0);
        assert_eq!(e.mem.u32(menu + 0x24), 0);
    }
}
