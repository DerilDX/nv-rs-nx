//! `fallout/interface/menus/repairmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The repair menu (`RepairMenu`, class `0x40b`): a `Menu` with thirteen
//! tiles from +0x28 and the repair list (`ListBox<ItemChange *>`) at +0x5c.
//! The singleton `RepairMenu::pMe` is `011da75c`. The constructor and the
//! destructor keep a C++ exception-unwinding frame (`FS:[0]`) that is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `RepairMenu::pMe` (Xbox PDB static): the menu singleton.
const REPAIR_MENU: u32 = 0x011d_a75c;
/// The menu class number.
const REPAIR_MENU_CLASS: u32 = 0x40b;
/// `RepairMenu`'s vtable.
const REPAIR_MENU_VTABLE: u32 = 0x0107_5c5c;
/// The four trait ids the constructor looks up (the Xbox PDB lists the
/// statics `iUT_Title`, `iUT_Value`, `iUT_Enabled` and `iUT_IsRepairItem`;
/// which address is which is not confirmed, so they are named by the
/// address of the trait name each is looked up with).
const TRAIT_FROM_NAME_0106ED20: u32 = 0x011d_a74c;
const TRAIT_FROM_NAME_0106ED18: u32 = 0x011d_a754;
const TRAIT_FROM_NAME_01070244: u32 = 0x011d_a750;
const TRAIT_FROM_NAME_01075C44: u32 = 0x011d_a748;

/// `Menu::Menu`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `ListBox<ItemChange *>::ListBox<ItemChange *>`.
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::RemoveFadeControl` (`this`, trait).
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The interface manager singleton getter.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// `Tile::GetMenuByClass` (cdecl, one word).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this` + file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// Sets the byte at +0x1c of `this` to 1 (the code is shared with
/// `Calendar`; on a menu it is `bRenderedMenu`).
const SET_BYTE_AT_0X1C: u32 = 0x0086_7a20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// A list box call with one word (`007a1950(0, word)` behind it).
const LIST_BOX_WITH_FLAG: u32 = 0x007a_1930;
/// `Tile::Poke` (`this`, trait, float).
const TILE_POKE: u32 = 0x00a0_1290;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;

/// The menu file name, the missing-tile message and the list template.
const MENU_FILE: u32 = 0x0107_5d14;
const MISSING_TILE_MESSAGE: u32 = 0x0107_5cc0;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_5ca4;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// Tile traits.
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_0XFA9: i32 = 0xfa9;

/// Offset of `lbRepairList` (Xbox PDB).
const REPAIR_LIST: u32 = 0x5c;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(REPAIR_MENU)
}

// Translated from 007b5400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::RepairMenu` (Xbox PDB): constructs the `Menu` base, stores
/// the class's vtable, constructs the repair list at +0x5c and looks up the
/// four trait ids the menu keeps in statics. Returns `this`.
// Translation incomplete: 007b5400 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn repair_menu_repair_menu(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), REPAIR_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(REPAIR_LIST)]);
    for (name, slot) in [
        (0x0106_ed20u32, TRAIT_FROM_NAME_0106ED20),
        (0x0106_ed18, TRAIT_FROM_NAME_0106ED18),
        (0x0107_0244, TRAIT_FROM_NAME_01070244),
        (0x0107_5c44, TRAIT_FROM_NAME_01075C44),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    this
}

// Translated from 007b54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::GetClass` (Xbox PDB): the class number `0x40b`.
pub fn repair_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    REPAIR_MENU_CLASS
}

// Translated from 007b54c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 13`.
pub fn repair_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0xd).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(0x28 + 4 * id as u32), tile);
    }
}

// Translated from 007b54f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn repair_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007b5770(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007b5520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::Create` (Xbox PDB, cdecl, no arguments): deletes an open
/// menu of the class, reads the menu file (`01075d14`) and takes its menu
/// as `pMe`. For a menu of this class: attaches the tile, marks it rendered
/// (`00867a20`), checks all thirteen tiles (else prints a message and
/// answers 0), sets the depth for the two stacking types, sets up the repair
/// list on the tile at +0x2c, calls `007a1930(list, 0)`, pokes 1.0 into the
/// trait `011da754` of the tiles at +0x3c and +0x50 and prepares the menu
/// for showing (flag 1). Returns the root tile. A menu of another class is
/// deleted (when it has a tile) and the result is 0.
pub fn repair_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let class = fn_007b5720(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(REPAIR_MENU, menu);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_007b5720(e) {
            return finish_create(e, root);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`repair_menu_create`] after the menu of the class was read.
fn finish_create(e: &mut Engine, root: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    if !fn_007b5730(e, Ptr::new(me(e))) {
        e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE]);
        return 0;
    }
    let kind = e
        .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
        .f64();
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = e
            .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
            .f64();
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            SET_FLOAT_TRAIT,
            &args![root, TRAIT_DEPTH, x87_store32(depth), true],
        );
    }
    let container = e.mem.u32(me(e).wrapping_add(0x2c));
    let list = me(e).wrapping_add(REPAIR_LIST);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, ITEM_TEMPLATE_NAME],
    );
    let list = me(e).wrapping_add(REPAIR_LIST);
    e.call(LIST_BOX_WITH_FLAG, &args![list, 0u32]);
    for offset in [0x3cu32, 0x50] {
        let trait_id = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        let tile = e.mem.u32(me(e).wrapping_add(offset));
        e.call(TILE_POKE, &args![tile, trait_id, 1.0f32]);
    }
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    root
}

// Translated from 007b5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x40b`, without an object (no `this`).
pub fn fn_007b5720(_e: &mut Engine) -> u32 {
    REPAIR_MENU_CLASS
}

// Translated from 007b5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all thirteen tiles of `pTiles` are set.
pub fn fn_007b5730(e: &mut Engine, this: Ptr) -> bool {
    (0..0xd).all(|i| e.mem.u32(this.addr().wrapping_add(0x28 + 4 * i)) != 0)
}

// Translated from 007b5770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `RepairMenu` destructor: stores the class's vtable, removes the
/// fade control `0xfa9` of the tile at +0x48, clears `pMe`, destroys the
/// repair list and runs the `Menu` destructor.
// Translation incomplete: 007b5770 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007b5770(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REPAIR_MENU_VTABLE);
    let tile = e.mem.u32(this.addr().wrapping_add(0x48));
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, TRAIT_0XFA9]);
    e.set_global(REPAIR_MENU, 0u32);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(REPAIR_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_5400, repair_menu_repair_menu(Ptr) -> Ptr),
        entry!(0x007b_54b0, repair_menu_get_class(Ptr) -> u32),
        entry!(0x007b_54c0, repair_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x007b_54f0, repair_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x007b_5520, repair_menu_create() -> u32),
        entry!(0x007b_5720, fn_007b5720() -> u32),
        entry!(0x007b_5730, fn_007b5730(Ptr) -> bool),
        entry!(0x007b_5770, fn_007b5770(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_builds_the_menu_and_looks_up_four_traits() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        stub(&mut e, &[MENU_CONSTRUCTOR, LIST_BOX_CONSTRUCTOR]);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_5400, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), REPAIR_MENU_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_CONSTRUCTOR), vec![vec![this + 0x5c]]);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED20), 0xed20);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED18), 0xed18);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01070244), 0x0244);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01075C44), 0x5c44);
    }

    #[test]
    fn class_number_and_tiles() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x007b_54b0, &args![0u32]).u32(), 0x40b);
        assert_eq!(e.call(0x007b_5720, &args![]).u32(), 0x40b);
        let this = e.mem.alloc(0x8c);
        e.call(0x007b_54c0, &args![this, 12i32, 0x99u32]);
        e.call(0x007b_54c0, &args![this, 13i32, 0x98u32]);
        assert_eq!(e.mem.u32(this + 0x28 + 48), 0x99);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert!(!e.call(0x007b_5730, &args![this]).bool());
        for i in 0..13 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x007b_5730, &args![this]).bool());
    }

    #[test]
    fn deleting_destructor_tears_down_and_frees() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x8c);
        e.set_global(REPAIR_MENU, this);
        e.mem.set_u32(this + 0x48, 0x480);
        stub(
            &mut e,
            &[
                TILE_REMOVE_FADE_CONTROL,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_54f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.global::<u32>(REPAIR_MENU), 0);
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x480, 0xfa9]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_DESTRUCTOR), vec![vec![this + 0x5c]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x007b_5770, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_sets_up_a_complete_menu() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.set_global(STACKING_TYPE_ONE, 102.0f64);
        e.set_global(TRAIT_FROM_NAME_0106ED18, 0x77u32);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x007b_54b0);
        for i in 0..13 {
            e.mem.set_u32(this + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this);
        e.register(GET_FLOAT_TRAIT, |_, _| Ret {
            st0: 102.0,
            ..Ret::default()
        });
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| Ret {
            st0: 9.0,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                SET_FLOAT_TRAIT,
                LIST_BOX_INITIALIZE,
                LIST_BOX_WITH_FLAG,
                TILE_POKE,
                MENU_PREP_FOR_VISIBILITY,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0x4444);
        assert_eq!(e.global::<u32>(REPAIR_MENU), this);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this, 0x4444, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x4444, 0xfad, 9.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![this + 0x5c, 0x104, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_WITH_FLAG), vec![vec![this + 0x5c, 0]]);
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![
                vec![0x114, 0x77, 1.0f32.to_bits()],
                vec![0x128, 0x77, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![this, 1]]);
    }

    #[test]
    fn create_deletes_the_old_menu_and_reports_missing_tiles() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        let old = e.mem.alloc(8);
        let old_vtable = e.mem.alloc(8);
        e.mem.set_u32(old, old_vtable);
        e.mem.set_u32(old_vtable, 0x00c0_0000);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x007b_54b0);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, old);
        answer(&mut e, TILE_GET_MENU, this);
        stub(
            &mut e,
            &[
                0x00c0_0000,
                INTERFACE_SINGLETON,
                TILE_READER,
                TILE_READ_FILE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![old, 1]]);
        assert_eq!(calls_to(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x40b]]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable, 0x00c0_0000);
        e.mem.set_u32(vtable + 0x34, 0x00c0_0034);
        answer(&mut e, 0x00c0_0034, 0x3f5);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_GET_MENU, this);
        answer(&mut e, WORD_AT_FOUR, 1);
        stub(
            &mut e,
            &[
                0x00c0_0000,
                INTERFACE_SINGLETON,
                TILE_READER,
                TILE_READ_FILE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![this, 1]]);
    }
}
