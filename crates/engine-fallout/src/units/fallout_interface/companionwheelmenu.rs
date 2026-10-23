//! `fallout/interface/menus/companionwheelmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The companion wheel menu's class (0x433).
const COMPANION_WHEEL_MENU_CLASS: u32 = 0x433;
/// `Tile::GetMenuByClass` (engine map, `00a09030`; `cdecl`, one word) and `Tile::GetMenu`
/// (engine map, `00a03c90`; `thiscall`, no stack words).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the type descriptors
/// `.?AVMenu@@` and `.?AVCompanionWheelMenu@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MENU: u32 = 0x0119_f34c;
const TYPE_COMPANION_WHEEL_MENU: u32 = 0x0119_ff50;
/// `CompanionWheelMenu::Create` (engine map, `00754de0`; `cdecl`, one word).
const COMPANION_WHEEL_MENU_CREATE: u32 = 0x0075_4de0;

// Translated from 00754cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::CreateCompanionWheelMenu` (engine map; `cdecl`, one word; the
/// answer in `AL`): when a menu of class 0x433 is open, its menu (`Tile::GetMenu`) cast to
/// `CompanionWheelMenu` (`__RTDynamicCast`) is deleted (its vtable slot 0 with 1); then
/// answers whether `CompanionWheelMenu::Create(argument)` made one.
pub fn companion_wheel_menu_create_companion_wheel_menu(e: &mut Engine, argument: u32) -> bool {
    let tile = e
        .call(GET_MENU_BY_CLASS, &args![COMPANION_WHEEL_MENU_CLASS])
        .u32();
    let menu = if tile != 0 {
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![menu, 0u32, TYPE_MENU, TYPE_COMPANION_WHEEL_MENU, 0u32],
        )
        .u32()
    } else {
        0
    };
    if menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(COMPANION_WHEEL_MENU_CREATE, &args![argument]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0075_4cf0,
        companion_wheel_menu_create_companion_wheel_menu(u32) -> bool
    )]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn rig(tile: u32, cast: u32, created: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(GET_MENU_BY_CLASS, move |_, _| tile.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x0c00_0000u32.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| cast.into_ret());
        e.register_double(COMPANION_WHEEL_MENU_CREATE, move |_, _| created.into_ret());
        e.put_vtable(0x0090_1000, &[0x0090_0000]);
        e.register(0x0090_0000, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn an_open_wheel_is_deleted_before_the_new_one() {
        let mut e = rig(0x5000, 0, 0x6000);
        let menu = e.mem.alloc(4);
        e.mem.set_u32(menu, 0x0090_1000);
        e.register_double(RT_DYNAMIC_CAST, move |_, _| menu.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4cf0, &args![7u32]).bool());
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x433]]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                0x0c00_0000,
                0,
                TYPE_MENU,
                TYPE_COMPANION_WHEEL_MENU,
                0
            ]]
        );
        assert_eq!(calls(&e, 0x0090_0000), vec![vec![menu, 1]]);
        assert_eq!(calls(&e, COMPANION_WHEEL_MENU_CREATE), vec![vec![7]]);
    }

    #[test]
    fn without_an_open_wheel_only_the_create_runs() {
        let mut e = rig(0, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0075_4cf0, &args![7u32]).bool());
        assert!(calls(&e, TILE_GET_MENU).is_empty());
        assert!(calls(&e, 0x0090_0000).is_empty());
        // A cast that finds nothing deletes nothing.
        let mut e = rig(0x5000, 0, 0x6000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4cf0, &args![7u32]).bool());
        assert!(calls(&e, 0x0090_0000).is_empty());
    }
}
