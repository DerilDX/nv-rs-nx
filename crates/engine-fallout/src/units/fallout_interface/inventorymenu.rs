//! `fallout/interface/menus/inventorymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Pip-Boy inventory menu (`InventoryMenu`, class `0x3ea`): a `Menu`
//! with 23 tiles from +0x28, the current tab, six page states, the
//! inventory list, the hot-key wheel and the list of items to update. The
//! singleton `InventoryMenu::pMe` is `011d9ea4` and the selected item
//! (`pCurrItem`, an `ItemChange *`) `011d9ea8`. The constructor and the
//! destructor keep a C++ exception-unwinding frame (`FS:[0]`) that is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InventoryMenu::pMe` (Xbox PDB static): the menu singleton.
const INVENTORY_MENU: u32 = 0x011d_9ea4;
/// `InventoryMenu::pCurrItem` (Xbox PDB static): the selected item.
const CURRENT_ITEM: u32 = 0x011d_9ea8;
/// A byte static `DoIdle` sets while the hot-key wheel has a highlight
/// (its name is not confirmed).
const WHEEL_HIGHLIGHTED: u32 = 0x011d_9f38;
/// The menu class number.
const INVENTORY_MENU_CLASS: u32 = 0x3ea;
/// `InventoryMenu`'s vtable.
const INVENTORY_MENU_VTABLE: u32 = 0x0107_39b4;
/// The five trait ids the constructor looks up (the Xbox PDB lists
/// `iUT_Value`, `iUT_ItemType`, `iUT_CurrentTab`, `iUT_KeyringOpen` and
/// `iUT_EquippableItem`; which address is which is not confirmed, so they
/// are named by the address of the trait name each is looked up with).
const TRAIT_FROM_NAME_0106ED18: u32 = 0x011d_9eb4;
const TRAIT_FROM_NAME_010739A0: u32 = 0x011d_9eb0;
const TRAIT_FROM_NAME_01073994: u32 = 0x011d_9eac;
const TRAIT_FROM_NAME_01073984: u32 = 0x011d_9eb8;
const TRAIT_FROM_NAME_01073974: u32 = 0x011d_9ebc;
/// The input singleton (the object `00877720` answers the controls of).
const INPUT_GLOBALS: u32 = 0x011d_ea0c;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;

/// `Menu::Menu`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `ListBox<ItemChange *>::ListBox<ItemChange *>`.
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
/// The hot-key wheel's constructor (the shared body that answers `this`).
const WHEEL_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The hot-key wheel's destructor body.
const WHEEL_DESTRUCTOR: u32 = 0x0070_1520;
/// The item update list's constructor and destructor bodies.
const UPDATE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const UPDATE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
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
/// `Menu::RemoveFromXMLArchive` (cdecl: file name, menu).
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// Sets the byte at +0x1c of `this` to 1 (`bRenderedMenu` on a menu).
const SET_BYTE_AT_0X1C: u32 = 0x0086_7a20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// Sets a string trait of a tile (`this`, trait, text, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// Sets an integer trait of a tile (`this`, trait, value).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `Tile::IsTrue` (`this`, trait).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// The tutorial message shower (`this` = interface + 0x4d4; id, class, time).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// A list box call with one word (`007a1950(0, word)` behind it).
const LIST_BOX_WITH_FLAG: u32 = 0x007a_1930;
/// `Interface::InitializeTabline` (cdecl, eight words).
const INITIALIZE_TABLINE: u32 = 0x0070_7be0;
/// `HotKeysWheel::Initialize` (`this`, tile).
const WHEEL_INITIALIZE: u32 = 0x0070_1610;
/// A setting's string value (`ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// `Interface::IsTopMenuID` (cdecl, one word).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::GetPipboy` (no arguments).
const GET_PIPBOY: u32 = 0x0070_5990;
/// A state test of the Pip-Boy object (`this` only; a byte).
const PIPBOY_READY: u32 = 0x0097_4d90;
/// Whether a gamepad is in use (no arguments).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// `HotKeysWheel::GetVisible`, `SetVisible` and `MoveHighlight`.
const WHEEL_GET_VISIBLE: u32 = 0x0070_1740;
const WHEEL_SET_VISIBLE: u32 = 0x0070_1760;
const WHEEL_MOVE_HIGHLIGHT: u32 = 0x0070_1bd0;
/// The hot-key wheel's item assignment (`this`, item).
const WHEEL_ASSIGN: u32 = 0x0070_19e0;
/// The hot-key wheel's slot count (`this` only).
const WHEEL_COUNT: u32 = 0x006e_4ba0;
/// `ItemChange::GetItemHealth` (`this`, flag; ST0).
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
/// The item's object (`this` only).
const ITEM_OBJECT: u32 = 0x0044_ddc0;
/// A form's type byte (`this` only).
const FORM_TYPE: u32 = 0x0040_1170;
/// Two float getters on the item's object (`this` only; ST0).
const OBJECT_VALUE_004BE180: u32 = 0x004b_e180;
const OBJECT_VALUE_004BE060: u32 = 0x004b_e060;
/// The message box shower (cdecl, six words).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// Adds the object to a list (`this`, address of the object).
const LIST_ADD_OBJECT: u32 = 0x005a_e3d0;
/// `InventoryMenu::UpdateInventoryMenu` (no arguments).
const UPDATE_INVENTORY_MENU: u32 = 0x0078_2a90;
/// The controls object of the input singleton (`this` only).
const INPUT_CONTROLS: u32 = 0x0087_7720;
/// Two key-state queries of the controls (`this`, key, flag).
const CONTROL_STATE: u32 = 0x00a2_4660;
const CONTROL_PRESSED: u32 = 0x00a2_3a50;
/// `Interface::IsInMenuMode` (no arguments).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// Fills the item card tiles (cdecl: three tiles, item, flag).
const SET_ITEM_CARD: u32 = 0x0071_9790;

/// The menu file name, the missing-tile message and the list template.
const MENU_FILE: u32 = 0x0107_3a70;
const MISSING_TILE_MESSAGE: u32 = 0x0107_3a18;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_39fc;
/// The two strings and the float the message boxes are shown with.
const MESSAGE_TEXT_ONE: u32 = 0x0102_08a0;
const MESSAGE_TEXT_TWO: u32 = 0x0107_3a94;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The two message settings.
const BROKEN_ITEM_SETTING: u32 = 0x011d_3810;
const NO_HOTKEY_SETTING: u32 = 0x011d_1e18;
/// The `double` stacking types the depth is set for, and 0.0.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The trait name the tab tiles' labels are set with.
const TAB_LABEL_TRAIT_NAME: u32 = 0x0106_ed20;
/// Tile traits.
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_0XFA9: i32 = 0xfa9;

layout! {
    /// `InventoryMenu` (Xbox PDB), 0x124 bytes; the PC code uses the PDB's
    /// offsets.
    pub struct InventoryMenu: 0x124 {
        /// `iCurrTab` (Xbox PDB).
        0x84 iCurrTab: i32,
    }
}

/// Offsets of `pPageStatesA` (six `sPageState`s of two words),
/// `lbInventoryList`, `HKWheel` and `ItemUpdateList` (Xbox PDB).
const PAGE_STATES: u32 = 0x88;
const INVENTORY_LIST: u32 = 0xb8;
const HOT_KEY_WHEEL: u32 = 0xe8;
const ITEM_UPDATE_LIST: u32 = 0x11c;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(INVENTORY_MENU)
}

fn current_item(e: &Engine) -> u32 {
    e.global::<u32>(CURRENT_ITEM)
}

fn tile_at(e: &Engine, menu: u32, offset: u32) -> u32 {
    e.mem.u32(menu.wrapping_add(offset))
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

/// The type byte of the selected item's object.
fn current_item_type(e: &mut Engine) -> u8 {
    let item = current_item(e);
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    e.call(FORM_TYPE, &args![object]).u8()
}

// Translated from 0077fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::InventoryMenu` (Xbox PDB): constructs the `Menu` base,
/// stores the class's vtable, constructs the inventory list, the hot-key
/// wheel and the item update list, looks up the five trait ids the menu
/// keeps in statics, sets the six page states to (-1, 0) and the current
/// tab to 0. Returns `this`.
// Translation incomplete: 0077fa00 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn inventory_menu_inventory_menu(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
) -> Ptr<InventoryMenu> {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), INVENTORY_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(INVENTORY_LIST)]);
    e.call(WHEEL_CONSTRUCTOR, &args![this.byte_add(HOT_KEY_WHEEL)]);
    e.call(
        UPDATE_LIST_CONSTRUCTOR,
        &args![this.byte_add(ITEM_UPDATE_LIST)],
    );
    for (name, slot) in [
        (0x0106_ed18u32, TRAIT_FROM_NAME_0106ED18),
        (0x0107_39a0, TRAIT_FROM_NAME_010739A0),
        (0x0107_3994, TRAIT_FROM_NAME_01073994),
        (0x0107_3984, TRAIT_FROM_NAME_01073984),
        (0x0107_3974, TRAIT_FROM_NAME_01073974),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    for i in 0..6 {
        let state = this.addr() + PAGE_STATES + 8 * i;
        e.mem.set_u32(state, u32::MAX);
        e.mem.set_u32(state + 4, 0);
    }
    e.set(this, InventoryMenu::iCurrTab, 0);
    this
}

// Translated from 0077fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::GetClass` (Xbox PDB): the class number `0x3ea`.
pub fn inventory_menu_get_class(_e: &mut Engine, _this: Ptr<InventoryMenu>) -> u32 {
    INVENTORY_MENU_CLASS
}

// Translated from 0077fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn inventory_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
    flags: u32,
) -> Ptr<InventoryMenu> {
    fn_0077fb80(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0077fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InventoryMenu` destructor: stores the class's vtable, clears
/// `pMe`, destroys the item update list, the hot-key wheel and the
/// inventory list (in that order) and runs the `Menu` destructor.
// Translation incomplete: 0077fb80 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0077fb80(e: &mut Engine, this: Ptr<InventoryMenu>) {
    e.mem.set_u32(this.addr(), INVENTORY_MENU_VTABLE);
    e.set_global(INVENTORY_MENU, 0u32);
    e.call(
        UPDATE_LIST_DESTRUCTOR,
        &args![this.byte_add(ITEM_UPDATE_LIST)],
    );
    e.call(WHEEL_DESTRUCTOR, &args![this.byte_add(HOT_KEY_WHEEL)]);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(INVENTORY_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 0077fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::Create` (Xbox PDB, cdecl, no arguments): asks for the
/// maximum depth (the value is not used), deletes an open menu of the
/// class, reads the menu file (`01073a70`), takes its menu as `pMe` and
/// removes the file from the XML archive. For a menu of this class:
/// attaches the tile, marks it rendered, checks all 23 tiles (else prints
/// a message and answers 0), sets the depth for the two stacking types,
/// shows tutorial message `0x22` (time `0x200`), sets up the inventory list
/// on the tile at +0x38, the tab line on the tile at +0x5c (five setting
/// strings), the hot-key wheel on the tile at +0x3c, the text of the tile
/// at +0x50 and the labels of the five tab tiles at +0x60..+0x70 that
/// exist, and prepares the menu for showing (flag 1). Returns the root
/// tile. A menu of another class is deleted (when it has a tile) and the
/// result is 0.
pub fn inventory_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(TILE_GET_MAXIMUM_DEPTH, &args![]);
    let class = fn_0077ff80(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(INVENTORY_MENU, menu);
    let menu = me(e);
    e.call(MENU_REMOVE_FROM_XML_ARCHIVE, &args![MENU_FILE, menu]);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_0077ff80(e) {
            return finish_create(e, root);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`inventory_menu_create`] after the menu of the class was
/// read.
fn finish_create(e: &mut Engine, root: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    if !fn_0077ff90(e, Ptr::new(me(e))) {
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
    let class = fn_0077ff80(e);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0x22u32, class, 0x200u32],
    );
    let container = tile_at(e, me(e), 0x38);
    let list = me(e).wrapping_add(INVENTORY_LIST);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, ITEM_TEMPLATE_NAME],
    );
    let list = me(e).wrapping_add(INVENTORY_LIST);
    e.call(LIST_BOX_WITH_FLAG, &args![list, 0u32]);
    // The tab labels, read in the order the code pushes them.
    let mut labels = [0u32; 5];
    for (label, setting) in labels.iter_mut().zip([
        0x011d_3258u32,
        0x011d_2acc,
        0x011d_40b4,
        0x011d_4d50,
        0x011d_29b8,
    ]) {
        *label = e.call(SETTING_STRING, &args![setting]).u32();
    }
    let tab_line = tile_at(e, me(e), 0x5c);
    e.call(
        INITIALIZE_TABLINE,
        &args![tab_line, 0x18u32, labels[4], labels[3], labels[2], labels[1], labels[0], 0u32],
    );
    let wheel_tile = tile_at(e, me(e), 0x3c);
    let wheel = me(e).wrapping_add(HOT_KEY_WHEEL);
    e.call(WHEEL_INITIALIZE, &args![wheel, wheel_tile]);
    let text = e.call(SETTING_STRING, &args![0x011d_29f4u32]).u32();
    let tile = tile_at(e, me(e), 0x50);
    e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, text, true]);
    let label_trait = e
        .call(TILE_TEXT_TO_TRAIT, &args![TAB_LABEL_TRAIT_NAME])
        .u32();
    for (offset, setting) in [
        (0x60u32, 0x011d_2c7cu32),
        (0x64, 0x011d_412c),
        (0x68, 0x011d_330c),
        (0x6c, 0x011d_20c4),
        (0x70, 0x011d_396c),
    ] {
        if tile_at(e, me(e), offset) != 0 {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let tile = tile_at(e, me(e), offset);
            e.call(SET_STRING_TRAIT, &args![tile, label_trait, text, true]);
        }
    }
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    root
}

// Translated from 0077ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x3ea`, without an object (no `this`).
pub fn fn_0077ff80(_e: &mut Engine) -> u32 {
    INVENTORY_MENU_CLASS
}

// Translated from 0077ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all 23 tiles of `pTiles` are set.
pub fn fn_0077ff90(e: &mut Engine, this: Ptr<InventoryMenu>) -> bool {
    (0..0x17).all(|i| tile_at(e, this.addr(), 0x28 + 4 * i) != 0)
}

// Translated from 00781ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoIdle` (Xbox PDB). Only while the menu is the top menu
/// and the Pip-Boy's state test (`00974d90`) passes:
/// - with a gamepad and the hot-key wheel shown: hides the tiles at +0x78
///   and +0x80 and moves the wheel's highlight by -1; a highlight sets the
///   byte static `011d9f38`; without one, when the static is set, the
///   selected item is assigned to the wheel (or a message is shown: for a
///   broken item, health 0.0; for an item of type `0x19` or when the
///   keyring trait is false), and the static is cleared;
/// - with a gamepad and the wheel hidden: for a selected item of type
///   `0x28` shows those tiles and fills them with its card;
/// - with the mouse: for an item of type `0x28` shows those tiles, then the
///   number keys (controls `0x11 + i`, slot 1 skipped) open the wheel on
///   their slot; with none pressed an open wheel is closed (and the tile at
///   +0x54 shown when the list has a highlighted item).
///
/// Then, in menu mode, control 1 with a highlighted item and control `0xe`
/// up clicks button 7 (`this`'s slot `+0xc`); the item card is filled for
/// the current tab (`InventoryMenu::pMe`'s `iCurrTab`: 0 the three tiles at
/// +0x60.., 1 the tiles at +0x6c and +0x70 when the object's two values are
/// not 0.0), the tile at +0x30's trait `0xfa9` is set to 0.0 and the last
/// card is filled with the tile at +0x30 when the player's actor value
/// `0x12` is not 0.0.
pub fn inventory_menu_do_idle(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![INVENTORY_MENU_CLASS]).bool() {
        return;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_READY, &args![pipboy]).bool() {
        return;
    }
    let wheel = me_addr + HOT_KEY_WHEEL;
    if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        if e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
            gamepad_wheel_shown(e, this);
        } else if current_item(e) != 0 && current_item_type(e) == 0x28 {
            show_card_tiles(e, me_addr, 1);
            for offset in [0x78u32, 0x80] {
                let tile = tile_at(e, me_addr, offset);
                let item = current_item(e);
                e.call(SET_ITEM_CARD, &args![0u32, 0u32, tile, item, 1u32]);
            }
        }
    } else {
        mouse_hot_keys(e, this);
    }
    finish_idle(e, this);
}

/// Shows or hides the tiles at +0x78 and +0x80.
fn show_card_tiles(e: &mut Engine, menu: u32, visible: i32) {
    for offset in [0x78u32, 0x80] {
        let tile = tile_at(e, menu, offset);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, visible);
    }
}

/// Shows a message box with the setting's text.
fn show_message(e: &mut Engine, setting: u32) {
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, MESSAGE_TEXT_ONE, MESSAGE_TEXT_TWO, time, 0u32],
    );
}

/// [`inventory_menu_do_idle`] with a gamepad and the hot-key wheel shown.
fn gamepad_wheel_shown(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let wheel = me_addr + HOT_KEY_WHEEL;
    show_card_tiles(e, me_addr, 0);
    let highlight = e.call(WHEEL_MOVE_HIGHLIGHT, &args![wheel, -1i32]).i32();
    if highlight >= 0 {
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        return;
    }
    if e.global::<u8>(WHEEL_HIGHLIGHTED) == 0 {
        return;
    }
    let broken = current_item(e) != 0 && {
        let item = current_item(e);
        let health = e.call(ITEM_GET_HEALTH, &args![item, true]).f64();
        health == e.global::<f64>(ZERO_DOUBLE)
    };
    if broken {
        show_message(e, BROKEN_ITEM_SETTING);
    } else {
        let keyring_trait = e.global::<u32>(TRAIT_FROM_NAME_01073974);
        let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        let refused = !e.call(TILE_IS_TRUE, &args![tile, keyring_trait]).bool()
            || (current_item(e) != 0 && current_item_type(e) == 0x19);
        if refused {
            show_message(e, NO_HOTKEY_SETTING);
        } else {
            let item = current_item(e);
            e.call(WHEEL_ASSIGN, &args![wheel, item]);
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), object);
                e.call(LIST_ADD_OBJECT, &args![me_addr + ITEM_UPDATE_LIST, slot]);
            });
            e.call(UPDATE_INVENTORY_MENU, &args![]);
            e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
            let tile = tile_at(e, me_addr, 0x54);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        }
    }
    e.set_global(WHEEL_HIGHLIGHTED, 0u8);
}

/// [`inventory_menu_do_idle`] without a gamepad: the number keys.
fn mouse_hot_keys(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let wheel = me_addr + HOT_KEY_WHEEL;
    let mut done = false;
    if current_item(e) != 0 && current_item_type(e) == 0x28 {
        show_card_tiles(e, me_addr, 1);
    }
    let mut slot = 0i32;
    loop {
        if done {
            break;
        }
        let keyring_open = e.global::<u32>(TRAIT_FROM_NAME_01073984);
        let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        if e.call(TILE_IS_TRUE, &args![tile, keyring_open]).bool() {
            break;
        }
        if slot >= e.call(WHEEL_COUNT, &args![wheel]).i32() {
            break;
        }
        let controls = e
            .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
            .u32();
        let key = slot.wrapping_add(0x11);
        let pressed = if e.call(CONTROL_STATE, &args![controls, key, 1u32]).i32() != 0 {
            true
        } else {
            let controls = e
                .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
                .u32();
            e.call(CONTROL_STATE, &args![controls, key, 0u32]).i32() != 0 && slot != 1
        };
        if pressed && slot != 1 {
            if !e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
                e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
                let tile = tile_at(e, me_addr, 0x54);
                set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
            } else {
                show_card_tiles(e, me_addr, 0);
            }
            e.call(WHEEL_MOVE_HIGHLIGHT, &args![wheel, slot]);
            done = true;
        }
        slot = slot.wrapping_add(1);
    }
    if !done && e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
        e.call(WHEEL_SET_VISIBLE, &args![wheel, 0u32]);
        let list = me_addr + INVENTORY_LIST;
        if e.vcall(list, 4, &args![]).u32() != 0 {
            let tile = tile_at(e, me_addr, 0x54);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
        }
    }
}

/// The tail of [`inventory_menu_do_idle`] that every mode runs.
fn finish_idle(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let list = me_addr + INVENTORY_LIST;
    if e.call(IS_IN_MENU_MODE, &args![]).bool() {
        let controls = e
            .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
            .u32();
        if e.call(CONTROL_PRESSED, &args![controls, 1i32, 1u32]).bool()
            && e.vcall(list, 4, &args![]).u32() != 0
        {
            let controls = e
                .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
                .u32();
            if e.call(CONTROL_STATE, &args![controls, 0xei32, 1u32]).i32() == 0 {
                e.vcall(me_addr, 0xc, &args![7u32, 0u32]);
            }
        }
    }
    let tab = e
        .get(Ptr::<InventoryMenu>::new(me(e)), InventoryMenu::iCurrTab)
        .wrapping_add(0x18);
    if tab == 0x18 {
        let first = tile_at(e, me_addr, 0x60);
        let second = tile_at(e, me_addr, 0x64);
        let third = tile_at(e, me_addr, 0x68);
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, third, item, 0u32]);
    } else if tab == 0x19 {
        let second = if current_item(e) != 0 && {
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let value = e.call(OBJECT_VALUE_004BE180, &args![object]).f64();
            value != e.global::<f64>(ZERO_DOUBLE)
        } {
            tile_at(e, me_addr, 0x70)
        } else {
            0
        };
        let first = if current_item(e) != 0 && {
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let value = e.call(OBJECT_VALUE_004BE060, &args![object]).f64();
            value != e.global::<f64>(ZERO_DOUBLE)
        } {
            tile_at(e, me_addr, 0x6c)
        } else {
            0
        };
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, 0u32, item, 0u32]);
    }
    let tile = tile_at(e, me_addr, 0x30);
    e.call(SET_FLOAT_TRAIT, &args![tile, TRAIT_0XFA9, 0.0f32, true]);
    let player = e.global::<u32>(PLAYER);
    // The player's object at +0xa4, slot +0xc, asked for actor value 0x12.
    let value = e
        .vcall(player.wrapping_add(0xa4), 0xc, &args![0x12u32])
        .f64();
    let first = if value == e.global::<f64>(ZERO_DOUBLE) {
        0
    } else {
        tile_at(e, me_addr, 0x30)
    };
    let last = tile_at(e, me_addr, 0x7c);
    let item = current_item(e);
    e.call(SET_ITEM_CARD, &args![first, last, 0u32, item, 0u32]);
}

// Translated from 007824b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low byte of `InventoryMenu::pMe`'s `iCurrTab`, or 0 without a menu.
pub fn fn_007824b0(e: &mut Engine) -> u8 {
    if me(e) == 0 {
        0
    } else {
        e.mem.u8(me(e) + 0x84)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0077_fa00,
            inventory_menu_inventory_menu(Ptr<InventoryMenu>) -> Ptr<InventoryMenu>
        ),
        entry!(
            0x0077_fb40,
            inventory_menu_get_class(Ptr<InventoryMenu>) -> u32
        ),
        entry!(
            0x0077_fb50,
            inventory_menu_scalar_deleting_destructor(
                Ptr<InventoryMenu>,
                u32,
            ) -> Ptr<InventoryMenu>
        ),
        entry!(0x0077_fb80, fn_0077fb80(Ptr<InventoryMenu>)),
        entry!(0x0077_fc10, inventory_menu_create() -> u32),
        entry!(0x0077_ff80, fn_0077ff80() -> u32),
        entry!(0x0077_ff90, fn_0077ff90(Ptr<InventoryMenu>) -> bool),
        entry!(0x0078_1ba0, inventory_menu_do_idle(Ptr<InventoryMenu>)),
        entry!(0x0078_24b0, fn_007824b0() -> u8),
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

    fn answer_st0(e: &mut Engine, address: u32, st0: f64) {
        e.register_double(address, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn globals(e: &mut Engine) {
        e.map(INVENTORY_MENU & !0xfff, 0x1000);
        e.map(INPUT_GLOBALS & !0xfff, 0x1000);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(MESSAGE_TIME & !0xfff, 0x1000);
        e.map(0x0107_0000, 0x1000);
    }

    #[test]
    fn constructor_builds_the_parts_and_resets_the_pages() {
        let mut e = Engine::new();
        globals(&mut e);
        stub(
            &mut e,
            &[
                MENU_CONSTRUCTOR,
                LIST_BOX_CONSTRUCTOR,
                WHEEL_CONSTRUCTOR,
                UPDATE_LIST_CONSTRUCTOR,
            ],
        );
        e.register(TILE_TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        let this = e.new_object::<InventoryMenu>();
        e.set(this, InventoryMenu::iCurrTab, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fa00, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), INVENTORY_MENU_VTABLE);
        assert_eq!(
            calls_to(&e, WHEEL_CONSTRUCTOR),
            vec![vec![this.addr() + 0xe8]]
        );
        assert_eq!(
            calls_to(&e, UPDATE_LIST_CONSTRUCTOR),
            vec![vec![this.addr() + 0x11c]]
        );
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01073974), 0x3974);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED18), 0xed18);
        for i in 0..6 {
            assert_eq!(e.mem.u32(this.addr() + 0x88 + 8 * i), u32::MAX);
        }
        assert_eq!(e.get(this, InventoryMenu::iCurrTab), 0);
    }

    #[test]
    fn destructor_order_and_delete() {
        let mut e = Engine::new();
        globals(&mut e);
        let this = e.new_object::<InventoryMenu>();
        e.set_global(INVENTORY_MENU, this.addr());
        stub(
            &mut e,
            &[
                UPDATE_LIST_DESTRUCTOR,
                WHEEL_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0077_fb50, &args![this, 1u32]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0077_fb50,
                UPDATE_LIST_DESTRUCTOR,
                WHEEL_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE
            ]
        );
        assert_eq!(e.global::<u32>(INVENTORY_MENU), 0);
        e.call_log = Some(vec![]);
        e.call(0x0077_fb50, &args![this, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn class_tiles_and_current_tab_byte() {
        let mut e = Engine::new();
        globals(&mut e);
        assert_eq!(e.call(0x0077_fb40, &args![0u32]).u32(), 0x3ea);
        assert_eq!(e.call(0x0077_ff80, &args![]).u32(), 0x3ea);
        let this = e.new_object::<InventoryMenu>();
        assert!(!e.call(0x0077_ff90, &args![this]).bool());
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x0077_ff90, &args![this]).bool());
        assert_eq!(e.call(0x0078_24b0, &args![]).u32() & 0xff, 0);
        e.set_global(INVENTORY_MENU, this.addr());
        e.set(this, InventoryMenu::iCurrTab, 0x102);
        assert_eq!(e.call(0x0078_24b0, &args![]).u32() & 0xff, 2);
    }

    #[test]
    fn create_sets_up_the_menu() {
        let mut e = Engine::new();
        globals(&mut e);
        e.set_global(STACKING_TYPE_TWO, 103.0f64);
        let this = e.new_object::<InventoryMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0077_fb40);
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer_st0(&mut e, GET_FLOAT_TRAIT, 103.0);
        answer_st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 8.0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x99);
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                SET_FLOAT_TRAIT,
                SHOW_TUTORIAL,
                LIST_BOX_INITIALIZE,
                LIST_BOX_WITH_FLAG,
                INITIALIZE_TABLINE,
                WHEEL_INITIALIZE,
                SET_STRING_TRAIT,
                MENU_PREP_FOR_VISIBILITY,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fc10, &args![]).u32(), 0x4444);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, MENU_REMOVE_FROM_XML_ARCHIVE),
            vec![vec![MENU_FILE, me_addr]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x4444, 0xfad, 8.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x4d4, 0x22, 0x3ea, 0x200]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![me_addr + 0xb8, 0x110, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(
            calls_to(&e, INITIALIZE_TABLINE),
            vec![vec![0x134, 0x18, 0x29b8, 0x4d50, 0x40b4, 0x2acc, 0x3258, 0]]
        );
        assert_eq!(
            calls_to(&e, WHEEL_INITIALIZE),
            vec![vec![me_addr + 0xe8, 0x114]]
        );
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![
                vec![0x128, 0xfc4, 0x29f4, 1],
                vec![0x138, 0x99, 0x2c7c, 1],
                vec![0x13c, 0x99, 0x412c, 1],
                vec![0x140, 0x99, 0x330c, 1],
                vec![0x144, 0x99, 0x20c4, 1],
                vec![0x148, 0x99, 0x396c, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![me_addr, 1]]
        );
    }

    #[test]
    fn create_reports_missing_tiles() {
        let mut e = Engine::new();
        globals(&mut e);
        let this = e.new_object::<InventoryMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0077_fb40);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_GET_MENU, this.addr());
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_GET_MAXIMUM_DEPTH,
                TILE_READER,
                TILE_READ_FILE,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fc10, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    /// A menu for `DoIdle` with its tiles, the list and the player set up.
    fn idle_menu(e: &mut Engine) -> Ptr<InventoryMenu> {
        globals(e);
        let this = e.new_object::<InventoryMenu>();
        e.set_global(INVENTORY_MENU, this.addr());
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0xc, 0x00c0_000c);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0xb8, list_vtable);
        e.mem.set_u32(list_vtable + 4, 0x00c0_0004);
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c0_00a4);
        answer(e, IS_TOP_MENU_ID, 1);
        answer(e, PIPBOY_READY, 1);
        stub(
            e,
            &[
                GET_PIPBOY,
                SET_TRAIT_FROM_INT,
                SET_FLOAT_TRAIT,
                SET_ITEM_CARD,
                IS_IN_MENU_MODE,
                0x00c0_00a4,
                0x00c0_000c,
            ],
        );
        this
    }

    #[test]
    fn do_idle_does_nothing_unless_on_top() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        answer(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        assert_eq!(calls_to(&e, IS_TOP_MENU_ID), vec![vec![0x3ea]]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn do_idle_fills_the_card_for_the_first_tab() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        answer_st0(&mut e, 0x00c0_00a4, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        let cards = calls_to(&e, SET_ITEM_CARD);
        assert_eq!(
            cards,
            vec![
                vec![0, 0, 0x150, 0x7000, 1],
                vec![0, 0, 0x158, 0x7000, 1],
                vec![0x138, 0x13c, 0x140, 0x7000, 0],
                vec![0x108, 0x154, 0, 0x7000, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x108, 0xfa9, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x00c0_00a4),
            vec![vec![e.global::<u32>(PLAYER) + 0xa4, 0x12]]
        );
        let _ = me_addr;
    }

    #[test]
    fn do_idle_assigns_the_item_when_the_wheel_highlight_ends() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        e.set(this, InventoryMenu::iCurrTab, 5);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, WHEEL_MOVE_HIGHLIGHT, u32::MAX);
        answer_st0(&mut e, ITEM_GET_HEALTH, 0.5);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x1f);
        e.register(LIST_ADD_OBJECT, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7100);
            Ret::default()
        });
        stub(
            &mut e,
            &[WHEEL_ASSIGN, UPDATE_INVENTORY_MENU, WHEEL_SET_VISIBLE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, WHEEL_ASSIGN),
            vec![vec![me_addr + 0xe8, 0x7000]]
        );
        assert_eq!(calls_to(&e, LIST_ADD_OBJECT).len(), 1);
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 1]]
        );
        assert_eq!(e.global::<u8>(WHEEL_HIGHLIGHTED), 0);
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn do_idle_refuses_a_broken_item() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        e.set_global(MESSAGE_TIME, 2.0f32.to_bits());
        e.set(this, InventoryMenu::iCurrTab, 5);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, WHEEL_MOVE_HIGHLIGHT, u32::MAX);
        answer_st0(&mut e, ITEM_GET_HEALTH, 0.0);
        answer(&mut e, SETTING_STRING, 0x5100);
        stub(&mut e, &[SHOW_MESSAGE]);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_STRING),
            vec![vec![BROKEN_ITEM_SETTING]]
        );
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE),
            vec![vec![
                0x5100,
                0,
                MESSAGE_TEXT_ONE,
                MESSAGE_TEXT_TWO,
                2.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(e.global::<u8>(WHEEL_HIGHLIGHTED), 0);
    }

    #[test]
    fn do_idle_number_key_opens_the_wheel_on_its_slot() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set(this, InventoryMenu::iCurrTab, 1);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, WHEEL_COUNT, 8);
        answer(&mut e, INPUT_CONTROLS, 0x6000);
        // Key 0x11 + 2 is held.
        e.register(CONTROL_STATE, |_, a| Ret {
            eax: u32::from(a[1] == 0x13 && a[2] == 0),
            ..Ret::default()
        });
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer_st0(&mut e, OBJECT_VALUE_004BE180, 1.0);
        answer_st0(&mut e, OBJECT_VALUE_004BE060, 0.0);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, FORM_TYPE, 0x1f);
        stub(&mut e, &[WHEEL_SET_VISIBLE, WHEEL_MOVE_HIGHLIGHT]);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 1]]
        );
        assert_eq!(
            calls_to(&e, WHEEL_MOVE_HIGHLIGHT),
            vec![vec![me_addr + 0xe8, 2]]
        );
        // Tab 1: the tile at +0x70 for the first value, none for the second.
        let cards = calls_to(&e, SET_ITEM_CARD);
        assert_eq!(cards[0], vec![0, 0x148, 0, 0x7000, 0]);
    }

    #[test]
    fn do_idle_closes_the_wheel_without_a_key_and_clicks_on_control_1() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set(this, InventoryMenu::iCurrTab, 4);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, WHEEL_COUNT, 3);
        answer(&mut e, INPUT_CONTROLS, 0x6000);
        stub(&mut e, &[CONTROL_STATE, WHEEL_SET_VISIBLE]);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, 0x00c0_0004, 0x55);
        answer(&mut e, IS_IN_MENU_MODE, 1);
        answer(&mut e, CONTROL_PRESSED, 1);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, CONTROL_STATE).len(),
            7,
            "two queries per slot, one more"
        );
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x154 - 0x28, 0xfa3, 1]]
        );
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![me_addr, 7, 0]]);
    }
}
