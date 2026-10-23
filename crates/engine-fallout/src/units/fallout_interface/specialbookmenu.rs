//! `fallout/interface/menus/specialbookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 00794d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::GetTotalPointsUsed` (engine map; `thiscall`, the menu not read): the
/// sum (32-bit, wrapping) of the player's values 5 to 11 (the object at the player's
/// +0xa4, its slot +0x1c with each index, in order).
pub fn special_book_menu_get_total_points_used(e: &mut Engine, _this: Ptr) -> u32 {
    let mut total = 0u32;
    for index in 5u32..=11 {
        let player = e.mem.u32(PLAYER);
        let value = e
            .vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
            .u32();
        total = total.wrapping_add(value);
    }
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0079_4d80,
        special_book_menu_get_total_points_used(Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_the_seven_values() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(0xa8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, 0x701c);
        e.register(0x701c, |_, a| (a[1] * 10).into_ret());
        e.mem.set_u32(player + 0xa4, vtable);
        e.mem.set_u32(PLAYER, player);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_4d80, &[0]).u32(), 560);
        let log = e.call_log.take().unwrap();
        let indices: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x701c)
            .map(|(_, w)| {
                assert_eq!(w[0], player + 0xa4);
                w[1]
            })
            .collect();
        assert_eq!(indices, vec![5, 6, 7, 8, 9, 10, 11]);
    }
}
