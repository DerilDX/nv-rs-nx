//! `fallout/interface/menus/levelupmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;
use crate::units::fallout_interface::startmenu::ListBox;

/// The constructor body `007883b0` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_007883B0: u32 = 0x0078_83b0;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<EffectItemData,...>`).
const VTABLE_00788300: u32 = 0x0107_3e50;

// Translated from 00788300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `levelupmenu.cpp`): stores the vtable `01073e50` (RTTI
/// class `BSSimpleArray<EffectItemData,...>`), then calls `007883b0(this, 0, 0)`. Returns
/// `this`.
pub fn fn_00788300(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_00788300);
    e.call(CONSTRUCTOR_BODY_007883B0, &args![this, 0u32, 0u32]);
    this
}

/// `TESCondition::IsTrue_ov2` (engine map), thiscall (object, 0, result
/// byte, 0), `RET 0x10`: `AL`.
const CONDITION_IS_TRUE: u32 = 0x0068_0c60;
/// Thiscall (word), `RET 4`: `AL` (`effectitem.rs`).
const FLAG_TEST_00403E60: u32 = 0x0040_3e60;
/// Cdecl (value, mask): `AL` (`run_00406cc0.rs`).
const FLAG_TEST_00406D70: u32 = 0x0040_6d70;
/// The `ListBoxBase` constructor (`menumanager.rs`), thiscall.
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
/// List constructor, thiscall (the embedded list at +4).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Thiscall (word, word), `RET 8`: the constructors pass 0 and 1.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// Cdecl (list box): the constructor hands the new list box to it
/// (`menumanager.rs`).
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
/// Cdecl (list box): the destructor hands the list box to it.
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `ListBox<BGSPerk *>`'s vtable (RTTI `.?AV?$ListBox@PAVBGSPerk@@@@`).
const PERK_LIST_BOX_VTABLE: u32 = 0x0107_3e28;
/// `Tile::GetMenu` (engine map), thiscall on a tile.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuTile` (engine map), thiscall on a tile.
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
/// `Menu::RenderTemplate` (engine map), thiscall (parent tile, template
/// name, word), `RET 0xC`: the new tile.
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `Tile::GetValue` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// Thiscall on a tile (trait id, integer), `RET 8`.
const TILE_SET_INT: u32 = 0x0070_0320;
/// Thiscall on a tile (trait id, text, word), `RET 0xC`.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// The allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// The allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// Thiscall on a new 0xC-byte item (tile, address of the data word),
/// `RET 8`: the item.
const ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// `BSSimpleList<ActiveEffect_P>::Insert` (engine map; folded code),
/// thiscall on the embedded list (item, compare function), `RET 8`.
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
/// Thiscall on the embedded list (address of the item word), `RET 4`.
const LIST_APPEND: u32 = 0x0090_5820;
/// Thiscall on a list box (flag bit), `RET 4`: `AL` (`menumanager.rs`).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (engine map; shared
/// body), thiscall (tile), `RET 4`.
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// Thiscall on a list box, no arguments: `EAX` (compared with 1 after an
/// item is added; subtracted from by `NumUnfiltered`).
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
/// `Tile::AddUserTrait` (engine map), cdecl (name, word): the trait id.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// Thiscall on a tile (trait id), `RET 4`: a float in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::GetValueQ` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::Value::ClearActions` (engine map), thiscall.
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction_ov2` (engine map), thiscall (action, tile,
/// trait id), `RET 0xC`.
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Tile::Value::AddAction` (engine map), thiscall (action, float), `RET 8`.
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Value::CalculateValue` (engine map), thiscall (word), `RET 4`.
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// Thiscall on a list node: returns the node itself (`run_006815c0.rs`); the
/// list box code calls it to reach the node's item word.
const NODE_SELF: u32 = 0x0068_15c0;
/// Thiscall on a list node: the next node (`menumanager.rs`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Thiscall on a list node (word), `RET 4`: the `RemoveAll` loop passes 0
/// before deleting the node.
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// Thiscall on a list node (flags), `RET 4`: its deleting destructor.
const NODE_DELETE: u32 = 0x0047_02f0;
/// Thiscall on an item's data (flags), `RET 4`.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that, when set, keeps the item tiles when a list box empties.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// Thiscall on a tile (trait id, float, word), `RET 0xC`.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::Poke` (engine map), thiscall (trait id, float), `RET 8`.
const TILE_POKE: u32 = 0x00a0_1290;
/// `Tile::TextToTrait` (engine map), cdecl (name): the trait id.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::IsTrue` (engine map), thiscall (trait id), `RET 4`: `AL`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// Thiscall on the embedded list (compare function), `RET 4`
/// (`run_0083fd60.rs`).
const LIST_SORT: u32 = 0x0083_fd60;
/// Thiscall on a list box (tile), `RET 4`: `AL`, whether the tile is one of
/// its items (`menumanager.rs`).
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Cdecl, no arguments: an object (`globalfunc.rs`).
const FOCUS_OWNER_004B7210: u32 = 0x004b_7210;
/// `InterfaceManager::SetCurrentFocusTarget` (engine map), thiscall (tile,
/// trait id, word), `RET 0xC`.
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// Cdecl, no arguments: `AL` (`globalfunc.rs`).
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// Cdecl (float, float): a float in `ST0` (`run_0040ea20.rs`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// Cdecl (float): a float in `ST0` (`teswater.rs`).
const ROUND_FLOAT: u32 = 0x004e_4430;
/// Cdecl (integer, integer): `EAX`.
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// Thiscall on a tile: `AL` (the byte the highlight box copies).
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
/// Thiscall on a tile (byte), `RET 4`.
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
/// `Tile::AddNeedsUpdate` (engine map), thiscall (flags), `RET 4`.
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// Cdecl (tile, name): a tile found by name below it.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `Tile::GetChildByName` (engine map), thiscall (name), `RET 4`.
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// Thiscall on the array, no arguments: `AL`, whether it is full
/// (`run_00438af0.rs`).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// Thiscall on the array: the grown reserved size (`run_009a3910.rs`).
const ARRAY_GROWN_SIZE: u32 = 0x009a_3910;
/// Thiscall on the array (new reserved size, element count), `RET 8`.
const ARRAY_RESIZE: u32 = 0x009a_3c50;
/// Cdecl (size, place): placement `new` (`run_006e6da0.rs`): the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// Thiscall: the 0x14-byte element constructor.
const ELEMENT_CONSTRUCT: u32 = 0x0040_3ba0;

/// `"_number_of_visible_items"`.
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;
/// `"_num_filtered"`.
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
/// `"_current_value"`.
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
/// `"_SetInCode"`.
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
/// `"_enabled"`.
const ENABLED_TEXT: u32 = 0x0107_0244;
/// `"_selected"`.
const SELECTED_TEXT: u32 = 0x0107_1b64;
/// `"_highlight_y"`.
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
/// `"_selected_height"`.
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
/// `"lb_highlight_box"`.
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
/// `"top"`, `"bottom"`, `"left"`, `"right"`.
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
/// `"_scroll_delta"`.
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
/// `"_number_of_items"`.
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;

/// The function-local statics the list box methods keep trait ids in: the
/// guard word (bit `bit` says the id has been looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// `007877a0`'s `"_selected"` id.
const SELECTED_TRAIT_007877A0: StaticTrait = StaticTrait {
    guard: 0x011d_a094,
    bit: 1,
    cache: 0x011d_a090,
    name: SELECTED_TEXT,
};
/// `NumUnfiltered`'s `"_num_filtered"` id.
const NUM_FILTERED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a09c,
    bit: 1,
    cache: 0x011d_a098,
    name: NUM_FILTERED_TEXT,
};
/// `IsEnabled`'s `"_enabled"` id.
const ENABLED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0a4,
    bit: 1,
    cache: 0x011d_a0a0,
    name: ENABLED_TEXT,
};
/// `00787c40`'s `"_selected"` id.
const SELECTED_TRAIT_00787C40: StaticTrait = StaticTrait {
    guard: 0x011d_a0ac,
    bit: 1,
    cache: 0x011d_a0a8,
    name: SELECTED_TEXT,
};
/// `UpdateHighlightBoxPos`'s `"_highlight_y"` id (bit 1 of the guard).
const HIGHLIGHT_Y_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0b8,
    bit: 1,
    cache: 0x011d_a0b4,
    name: HIGHLIGHT_Y_TEXT,
};
/// `UpdateHighlightBoxPos`'s `"_selected_height"` id (bit 2 of the guard).
const SELECTED_HEIGHT_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0b8,
    bit: 2,
    cache: 0x011d_a0b0,
    name: SELECTED_HEIGHT_TEXT,
};

/// Looks the static trait id up the first time: the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does. Returns the id
/// read back from the static.
fn static_trait(e: &mut Engine, which: StaticTrait) -> u32 {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
    e.mem.u32(which.cache)
}

/// The item word of a list node, reached through `006815c0` as the code
/// does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The first node of the embedded list (`this + 4`; the code guards a null
/// `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: the code goes on only when the
/// value is strictly above the other (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

// Translated from 00785150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `levelupmenu.cpp`): `TESCondition::IsTrue_ov2`
/// (`00680c60`) on the condition at `this + 0x40` with `object`, 0, a stack
/// byte for its out parameter and 0; returns its `AL`.
pub fn fn_00785150(e: &mut Engine, this: Ptr, object: u32) -> bool {
    e.with_stack(4, |e, result| {
        e.call(
            CONDITION_IS_TRUE,
            &args![this.addr() + 0x40, object, 0u32, result, 0u32],
        )
        .bool()
    })
}

// Translated from 007869e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `first = 00403e60(this, 4)`; `second` is 0
/// when `value` is -1, otherwise `00406d70(value, 0x200)`; returns 1 when
/// exactly one of them is set.
pub fn fn_007869e0(e: &mut Engine, this: Ptr, value: i32) -> u8 {
    let first = e.call(FLAG_TEST_00403E60, &args![this, 4u32]).u8();
    let mut second = 0u8;
    if value != -1 {
        second = e.call(FLAG_TEST_00406D70, &args![value, 0x200u32]).u8();
    }
    if (first == 0 || second != 0) && (first != 0 || second == 0) {
        0
    } else {
        1
    }
}

// Translated from 00786bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786bb0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSPerk *>` constructor (engine map unnamed; RTTI of the
/// vtable `01073e28`): the `ListBoxBase` base (`007240c0`), the embedded
/// list (+4, `0096a2d0`), the vtable, the fields cleared, `007a1950(this, 0,
/// 1)` and the list box handed to `00721130`. Returns `this`.
pub fn fn_00786bb0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), PERK_LIST_BOX_VTABLE);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    e.call(LIST_BOX_INIT_FLAGS, &args![this, 0u32, 1u32]);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

// Translated from 00786c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786c80 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSPerk *>` destructor: the vtable, `RemoveAll`, the list
/// box handed to `007211c0`, then the embedded list destroyed (`0046ffb0`).
pub fn fn_00786c80(e: &mut Engine, this: Ptr<ListBox>) {
    e.mem.set_u32(this.addr(), PERK_LIST_BOX_VTABLE);
    list_box_bgs_perk_remove_all(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 00786cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786cf0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<BGSPerk_P>::AddItem` (engine map): with a container and a
/// template name (`template`, else `pTemplateName`), renders the template
/// in the container's menu (`Menu::RenderTemplate`), gives the new tile
/// trait `0xfaa` -1 when it has none and the text `text` (trait `0xfc4`)
/// when there is one, and adds an item (tile, `data`): sorted with
/// `compare` (then `Synchronize` when flag 0 is set) or appended (then,
/// with flag 0, `InitTilePosition` and `SetTotalListHeight`, and the tile
/// gets the next index as trait `0xfac`). When the list then counts 1
/// (`00726c40`) and the container's `"_number_of_visible_items"` is above
/// `0.0`, the container's height value (`0xfb0`) is rebuilt from the tile's
/// height times that number. Returns the tile, or 0 without a container or
/// template.
pub fn list_box_bgs_perk_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    if e.get(this, ListBox::pTemplateName).is_null() && template == 0 {
        return 0;
    }
    let name = if template != 0 {
        template
    } else {
        e.get(this, ListBox::pTemplateName).addr()
    };
    let container = e.get(this, ListBox::pContainerRect);
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, container, name, 0u32])
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xffff_ffffu32]);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    // The item is built with the address of the `data` argument's stack
    // slot.
    let item = e.with_stack(4, |e, data_slot| {
        e.mem.set_u32(data_slot.addr(), data);
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        if block != 0 {
            e.call(ITEM_CONSTRUCT, &args![block, tile, data_slot]).u32()
        } else {
            0
        }
    });
    if compare != 0 {
        e.call(LIST_INSERT_SORTED, &args![this.addr() + 4, item, compare]);
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            list_box_bgs_perk_synchronize(e, this);
        }
    } else {
        e.with_stack(4, |e, item_slot| {
            e.mem.set_u32(item_slot.addr(), item);
            e.call(LIST_APPEND, &args![this.addr() + 4, item_slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(INIT_TILE_POSITION, &args![this, tile]);
            list_box_bgs_perk_set_total_list_height(e, this);
        }
        let index = e.get(this, ListBox::usNextIndex);
        e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
        e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
    }
    if e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32() == 1 {
        let visible = e
            .call(
                ADD_USER_TRAIT,
                &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect);
        let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(count, zero) {
            let container = e.get(this, ListBox::pContainerRect);
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let visible = e
                .call(
                    ADD_USER_TRAIT,
                    &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect);
            let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
            e.call(
                VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, x87_store32(count)],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each
/// time: the tile through its deleting destructor (vtable slot 0, flag 1)
/// unless `KEEP_ITEM_TILES` is set, the data (`004459e0` with 1) when flag
/// bit 1 is set, then the item freed.
fn delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.global::<u8>(KEEP_ITEM_TILES) == 0 {
        let entry = item(e);
        let tile = e.mem.u32(entry);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).bool() {
        let entry = item(e);
        let data = e.mem.u32(entry + 4);
        if data != 0 {
            e.call(ITEM_DATA_DELETE, &args![data, 1u32]);
        }
    }
    let entry = item(e);
    e.call(FREE, &args![entry]);
}

// Translated from 00786f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::RemoveAll` (engine map): while the second node
/// (+8) and its item are non-null, deletes the item, clears the node's next
/// pointer (`006ecd40` with 0), deletes the node (`004702f0` with 1) and
/// makes the following node the second; then deletes the first item (+4).
/// Clears the first item, `usNextIndex` and `fTotalListHeight`, sets the
/// container's `"_num_filtered"` to `0.0`, then `SetTotalListHeight`,
/// clears `pHighlightedItem` and `UpdateHighlightBoxPos`.
pub fn list_box_bgs_perk_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = e.call(NODE_NEXT, &args![second]).u32();
        delete_item(e, this, |e| {
            let node = e.mem.u32(base + 8);
            node_item(e, node)
        });
        let second = e.mem.u32(base + 8);
        e.call(NODE_SET_NEXT, &args![second, 0u32]);
        let second = e.mem.u32(base + 8);
        if second != 0 {
            e.call(NODE_DELETE, &args![second, 1u32]);
        }
        e.mem.set_u32(base + 8, next);
    }
    if e.mem.u32(base + 4) != 0 {
        delete_item(e, this, |e| e.mem.u32(base + 4));
    }
    e.mem.set_u32(base + 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let num_filtered = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_TEXT]).u32();
        let container = e.get(this, ListBox::pContainerRect);
        e.call(
            TILE_SET_FLOAT,
            &args![container, num_filtered, 0.0f32.to_bits(), 1u32],
        );
    }
    list_box_bgs_perk_set_total_list_height(e, this);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_bgs_perk_update_highlight_box_pos(e, this);
}

// Translated from 007871c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Sort` (engine map): sorts the embedded list
/// (`0083fd60` on +4 with `compare`), then `Synchronize`.
pub fn list_box_bgs_perk_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    list_box_bgs_perk_synchronize(e, this);
}

// Translated from 007871f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Highlight` (engine map): when the list is enabled
/// and `tile` is null, or is one of its items (`007242f0`) and
/// `"_enabled"`, makes it `pHighlightedItem`, moves the highlight box and
/// returns true; otherwise false.
pub fn list_box_bgs_perk_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    if !list_box_bgs_perk_is_enabled(e, this) {
        return false;
    }
    if tile != 0 {
        if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
            return false;
        }
        let enabled = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        if !e.call(TILE_IS_TRUE, &args![tile, enabled]).bool() {
            return false;
        }
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    list_box_bgs_perk_update_highlight_box_pos(e, this);
    true
}

// Translated from 00787600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::RestoreListState` (engine map): with a container
/// and a scroll bar, gives the scroll bar `"_current_value"` (the saved
/// scroll bar value, word 0) and pokes `"_SetInCode"` with `1.0`; takes the
/// tile at index `_ftol(0040ebd0(fSavedHighlightIndex, NumUnfiltered -
/// 1.0))` (vtable +0x14 with 0), makes it the interface's focus target
/// (`00715860` with trait `0xfc3` and `focus`) and highlights it (vtable
/// +0). Unless `004b71d0` says so, also scrolls the container's menu to it
/// (the menu's vtable +0x14 with the tile's index trait `0xfaa` and the
/// tile, when there is a tile) and highlights nothing.
pub fn list_box_bgs_perk_restore_list_state(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
    if e.get(this, ListBox::pContainerRect).is_null() || e.get(this, ListBox::pScrollBar).is_null()
    {
        return;
    }
    let saved_value = e.mem.u32(this.addr() + 0x28);
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, x87_quiet_bits(saved_value), 0u32],
    );
    let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32.to_bits()]);
    let count = list_box_bgs_perk_num_unfiltered(e, this) as u32;
    let one: f64 = e.global(ONE_DOUBLE);
    let last = x87_store32(x87_sub(f64::from(count), one));
    let saved_index = e.mem.u32(this.addr() + 0x24);
    let index = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(saved_index), x87_quiet_bits(last)],
        )
        .f64();
    let index = x87_fistp64_chop(X87Ext::from(index)) as u32;
    let tile = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
    let owner = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    e.call(
        SET_CURRENT_FOCUS_TARGET,
        &args![owner, tile, 0xfc3u32, u32::from(focus)],
    );
    e.vcall(this.addr(), 0, &args![tile]);
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    if tile != 0 {
        let container = e.get(this, ListBox::pContainerRect);
        let menu_tile = e.call(TILE_GET_MENU_TILE, &args![container]).u32();
        let menu = e.call(TILE_GET_MENU, &args![menu_tile]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfaau32]).f64();
        let item_index = e.call(FTOL2, &args![value]).u32();
        e.vcall(menu, 0x14, &args![item_index, tile]);
    }
    e.vcall(this.addr(), 0, &args![0u32]);
}

// Translated from 00787760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00787c40(this, tile, 0)` (select).
pub fn fn_00787760(e: &mut Engine, this: Ptr<ListBox>, tile: u32) {
    fn_00787c40(e, this, tile, 0);
}

// Translated from 00787780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00787c40(this, tile, 2)` (toggle).
pub fn fn_00787780(e: &mut Engine, this: Ptr<ListBox>, tile: u32) {
    fn_00787c40(e, this, tile, 2);
}

// Translated from 007877a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007877a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// Unnamed in the engine map: whether `tile` is selected: the
/// `"_selected"` trait id (a function-local static, `011da090`) is non-zero,
/// `tile` is non-null and `Tile::IsTrue(tile, id)`.
pub fn fn_007877a0(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    let _ = this;
    let selected = static_trait(e, SELECTED_TRAIT_007877A0);
    if selected == 0 || tile == 0 {
        return false;
    }
    let selected = e.mem.u32(SELECTED_TRAIT_007877A0.cache);
    e.call(TILE_IS_TRUE, &args![tile, selected]).bool()
}

// Translated from 007878d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of items whose tile is selected
/// (`007877a0`), walking the nodes while the node and its item are
/// non-null.
pub fn fn_007878d0(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let mut count = 0u32;
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if fn_007877a0(e, this, tile) {
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    count
}

// Translated from 00787950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`, elements of
/// 0x14 bytes): takes the next slot (`007881d0`), constructs one element
/// there (`00788260`) and copies the five words at `element` into it.
/// Returns the slot's index.
pub fn fn_00787950(e: &mut Engine, this: Ptr<BSSimpleArray>, element: u32) -> u32 {
    let index = fn_007881d0(e, this);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    fn_00788260(e, this, index.wrapping_mul(0x14).wrapping_add(buffer), 1);
    let slot = e
        .get(this, BSSimpleArray::pBuffer)
        .wrapping_add(index.wrapping_mul(0x14));
    for word in 0..5 {
        let value = e.mem.u32(element + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    index
}

// Translated from 007879f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007879f0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSPerk_P>::NumUnfiltered` (engine map): `00726c40(this)` minus
/// the container's `"_num_filtered"` (a function-local static id; the float
/// truncated by `FISTP qword`).
pub fn list_box_bgs_perk_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    static_trait(e, NUM_FILTERED_TRAIT);
    let count = e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32();
    let num_filtered = e.mem.u32(NUM_FILTERED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let value = e
        .call(TILE_GET_FLOAT, &args![container, num_filtered])
        .f64();
    let filtered = x87_fistp64_chop(X87Ext::from(value)) as u32;
    count.wrapping_sub(filtered) as i32
}

// Translated from 00787aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Synchronize` (engine map): clears `usNextIndex`
/// and `fTotalListHeight`, then for every node with an item whose filtered
/// byte (+8) is clear gives the item's tile the next index (trait `0xfac`)
/// and its position (`InitTilePosition`); then `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`.
pub fn list_box_bgs_perk_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    list_box_bgs_perk_set_total_list_height(e, this);
    list_box_bgs_perk_update_highlight_box_pos(e, this);
}

// Translated from 00787b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787b70 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSPerk_P>::IsEnabled` (engine map): the container exists and
/// its `"_enabled"` trait (a function-local static id) is true.
pub fn list_box_bgs_perk_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    static_trait(e, ENABLED_TRAIT);
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return false;
    }
    let enabled = e.mem.u32(ENABLED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

// Translated from 00787c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787c40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// Unnamed in the engine map: when the list is enabled and `tile` is one of
/// its items, changes the tile's `"_selected"` trait (a function-local
/// static id): `mode` 0 sets it when clear, 1 clears it when set, 2 flips
/// it. Returns true when it was changed; false otherwise (and for any other
/// mode).
pub fn fn_00787c40(e: &mut Engine, this: Ptr<ListBox>, tile: u32, mode: u32) -> bool {
    if !list_box_bgs_perk_is_enabled(e, this) || tile == 0 {
        return false;
    }
    if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
        return false;
    }
    let selected_trait = static_trait(e, SELECTED_TRAIT_00787C40);
    let selected = e.call(TILE_IS_TRUE, &args![tile, selected_trait]).u8();
    match mode {
        0 => {
            if selected == 0 {
                let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
                e.call(TILE_SET_INT, &args![tile, id, 1u32]);
                return true;
            }
            false
        }
        1 => {
            if selected != 0 {
                let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
                e.call(TILE_SET_INT, &args![tile, id, 0u32]);
                return true;
            }
            false
        }
        2 => {
            let value = u32::from(selected == 0);
            let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
            e.call(TILE_SET_INT, &args![tile, id, value]);
            true
        }
        _ => false,
    }
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an
/// update with flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

// Translated from 00787d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787d80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSPerk_P>::UpdateHighlightBoxPos` (engine map): looks the
/// `"_highlight_y"` and `"_selected_height"` trait ids up (statics); with a
/// container and no highlighted item, sets them to -1 and 0 on the
/// container; with one, makes both values follow the highlighted tile's
/// traits `0xfa2` and `0xfb0` (action `0x7d0`, then calculated) and gives
/// the container's `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`,
/// `"left"` and `"right"` children when `"top"` exists (the other three are
/// not checked for null), the highlighted tile's byte (`0071aa80`, as 1 or
/// 0) and an update.
pub fn list_box_bgs_perk_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    static_trait(e, HIGHLIGHT_Y_TRAIT);
    static_trait(e, SELECTED_HEIGHT_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y_value, 0x7d0u32, highlighted, 0xfa2u32],
    );
    e.call(VALUE_CALCULATE, &args![y_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height_value, 0x7d0u32, highlighted, 0xfb0u32],
    );
    e.call(VALUE_CALCULATE, &args![height_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    // The two branches of the code differ only in the byte they pass.
    let flag = u32::from(
        e.call(TILE_FLAG_GETTER_0071AA80, &args![highlighted])
            .bool(),
    );
    let container = e.get(this, ListBox::pContainerRect);
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, HIGHLIGHT_BOX_TEXT])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, TOP_TEXT])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        for name in [BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

// Translated from 007880a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::SetTotalListHeight` (engine map): with a
/// container, its visible height (trait `0xfb0`, stored as a `float`) and
/// the total list height decide the number of scroll steps: 1, unless both
/// are above `0.0` and the visible height is below the total, then
/// `00647b70(_ftol2(004e4430(unfiltered * (total - visible) / total)) + 1,
/// 2)` (the ratio stored as a `float`). With both above `0.0` and more than
/// one step, the container's `"_scroll_delta"` becomes `(total - visible) /
/// (steps - 1)`; with a scroll bar its `"_number_of_items"` becomes the
/// number of steps.
pub fn list_box_bgs_perk_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the
        // visible height is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = list_box_bgs_perk_num_unfiltered(e, this) as u32;
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let product = x87_mul(f64::from(count), difference);
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let ratio = x87_store32(x87_div(product, x87_m32(total_bits)));
            let rounded = e.call(ROUND_FLOAT, &args![x87_quiet_bits(ratio)]).f64();
            let whole = e.call(FTOL2, &args![rounded]).i32();
            steps = e
                .call(INT_PAIR_00647B70, &args![whole.wrapping_add(1), 2i32])
                .i32();
        }
        if steps > 1 {
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let delta = x87_store32(x87_div(difference, f64::from(steps - 1)));
            let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(
                TILE_SET_FLOAT,
                &args![container, scroll_delta, x87_quiet_bits(delta), 1u32],
            );
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let number_of_items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, number_of_items, steps]);
    }
}

// Translated from 007881d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`): makes room
/// for one more element when the array is full (`00438b90`): a first
/// buffer of 4 from vtable +4 when nothing is reserved, otherwise the
/// grown size (`009a3910`) through `009a3c50(new size, iSize)`; then
/// counts the element and returns its index (`iSize - 1`).
pub fn fn_007881d0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let size = 4u32;
            let buffer = e.vcall(this.addr(), 4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let size = e.call(ARRAY_GROWN_SIZE, &args![this]).u32();
            let count = e.get(this, BSSimpleArray::iSize);
            e.call(ARRAY_RESIZE, &args![this, size, count]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    }
    let count = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, count.wrapping_add(1));
    e.get(this, BSSimpleArray::iSize).wrapping_sub(1)
}

// Translated from 00788260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00788260 - the C++ exception frame (FS:[0] chain, unwind state 0 around each element's construction) is not translated.
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`): constructs
/// `count` elements of 0x14 bytes from `place` (placement `new`,
/// `006e6da0`, then `00403ba0` on a non-null place).
pub fn fn_00788260(e: &mut Engine, this: Ptr<BSSimpleArray>, place: u32, count: u32) {
    let _ = this;
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(
                PLACEMENT_NEW,
                &args![0x14u32, index.wrapping_mul(0x14).wrapping_add(place)],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![element]);
        }
        index += 1;
    }
}

// Translated from 00788330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`'s buffer
/// allocation, vtable +4): `count * 0x14` bytes from the allocator.
pub fn fn_00788330(e: &mut Engine, this: Ptr<BSSimpleArray>, count: u32) -> u32 {
    let _ = this;
    e.call(ALLOCATE, &args![count.wrapping_mul(0x14)]).u32()
}

// Translated from 007883b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`'s constructor
/// body): clears the buffer, size and reserved size; reserves
/// `max(reserve, size)` elements through vtable +4 when that is non-zero,
/// then constructs `size` elements (`00788260`) and sets the size.
pub fn fn_007883b0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_00788260(e, this, buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0078_8300, fn_00788300(Ptr) -> Ptr),
        entry!(0x0078_5150, fn_00785150(Ptr, u32) -> bool),
        entry!(0x0078_69e0, fn_007869e0(Ptr, i32) -> u8),
        entry!(0x0078_6bb0, fn_00786bb0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0078_6c80, fn_00786c80(Ptr<ListBox>)),
        entry!(
            0x0078_6cf0,
            list_box_bgs_perk_add_item(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0078_6f90, list_box_bgs_perk_remove_all(Ptr<ListBox>)),
        entry!(0x0078_71c0, list_box_bgs_perk_sort(Ptr<ListBox>, u32)),
        entry!(
            0x0078_71f0,
            list_box_bgs_perk_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0078_7600,
            list_box_bgs_perk_restore_list_state(Ptr<ListBox>, u8)
        ),
        entry!(0x0078_7760, fn_00787760(Ptr<ListBox>, u32)),
        entry!(0x0078_7780, fn_00787780(Ptr<ListBox>, u32)),
        entry!(0x0078_77a0, fn_007877a0(Ptr<ListBox>, u32) -> bool),
        entry!(0x0078_78d0, fn_007878d0(Ptr<ListBox>) -> u32),
        entry!(0x0078_7950, fn_00787950(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(
            0x0078_79f0,
            list_box_bgs_perk_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(0x0078_7aa0, list_box_bgs_perk_synchronize(Ptr<ListBox>)),
        entry!(
            0x0078_7b70,
            list_box_bgs_perk_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(0x0078_7c40, fn_00787c40(Ptr<ListBox>, u32, u32) -> bool),
        entry!(
            0x0078_7d80,
            list_box_bgs_perk_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0078_80a0,
            list_box_bgs_perk_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x0078_81d0, fn_007881d0(Ptr<BSSimpleArray>) -> u32),
        entry!(0x0078_8260, fn_00788260(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0078_8330, fn_00788330(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x0078_83b0, fn_007883b0(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_007883B0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_8300, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_007883B0),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_00788300);
    }
}

#[cfg(test)]
mod perk_list_box_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the statics, the flag byte and the two doubles mapped,
    /// and the list node helpers doubled (`006815c0` returns the node,
    /// `00726070` its next word).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(NODE_SELF, |_, a| eax(a[0]));
        e.register(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    /// An item (tile, data, filtered byte) in a new node, or in the embedded
    /// first node when `node` is the list box's +4.
    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    #[test]
    fn fn_00785150_tests_the_condition_at_0x40() {
        let mut e = engine();
        e.register(CONDITION_IS_TRUE, |_, _| eax(1));
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_5150, &args![this, 0x1234u32]).bool());
        let calls = calls_to(&e, CONDITION_IS_TRUE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][..3], [this + 0x40, 0x1234, 0]);
        assert_ne!(calls[0][3], 0);
        assert_eq!(calls[0][4], 0);
    }

    #[test]
    fn fn_007869e0_is_true_when_exactly_one_flag_is_set() {
        for (first, value, second, expected) in [
            (1u32, -1i32, 1u32, 1u8),
            (0, -1, 1, 0),
            (1, 5, 1, 0),
            (1, 5, 0, 1),
            (0, 5, 1, 1),
            (0, 5, 0, 0),
        ] {
            let mut e = engine();
            e.register_double(FLAG_TEST_00403E60, move |_, a| {
                assert_eq!(a[1], 4);
                eax(first)
            });
            e.register_double(FLAG_TEST_00406D70, move |_, a| {
                assert_eq!(a[1], 0x200);
                eax(second)
            });
            let this = e.mem.alloc(8);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0078_69e0, &args![this, value]).u8(), expected);
            assert_eq!(
                calls_to(&e, FLAG_TEST_00406D70).len(),
                usize::from(value != -1)
            );
        }
    }

    #[test]
    fn fn_00786bb0_builds_the_perk_list_box() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                LIST_BOX_BASE_CONSTRUCT,
                LIST_CONSTRUCT,
                LIST_BOX_INIT_FLAGS,
                LIST_BOX_REGISTER,
            ],
        );
        let this = list_box(&mut e);
        for offset in (0x0c..0x30).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_6bb0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), PERK_LIST_BOX_VTABLE);
        for offset in [0x0c, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u16(this.addr() + 0x1c), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x2c), 0);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![this.addr() + 4]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_INIT_FLAGS),
            vec![vec![this.addr(), 0, 1]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_00786c80_empties_and_destroys_the_list() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
                TILE_SET_INT,
                TILE_SET_FLOAT,
            ],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0078_6c80, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), PERK_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
        // Without a container RemoveAll stops before the traits.
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }

    fn add_item_engine() -> Engine {
        let mut e = engine();
        e.register(TILE_GET_MENU, |_, _| eax(0x5000));
        e.register(MENU_RENDER_TEMPLATE, |_, _| eax(0x6000));
        e.register(TILE_GET_VALUE, |_, _| eax(0));
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(ITEM_CONSTRUCT, |e, a| {
            let data = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, data);
            eax(a[0])
        });
        e.register(LIST_BOX_HAS_FLAG, |_, _| eax(1));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(1));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x77));
        e.register(TILE_GET_FLOAT, |_, _| st0(3.0));
        e.register(TILE_GET_VALUE_Q, |_, _| eax(0x9000));
        stub(
            &mut e,
            &[
                TILE_SET_INT,
                TILE_SET_STRING,
                LIST_INSERT_SORTED,
                LIST_APPEND,
                INIT_TILE_POSITION,
                VALUE_CLEAR_ACTIONS,
                VALUE_ADD_ACTION_TILE,
                VALUE_ADD_ACTION_FLOAT,
                VALUE_CALCULATE,
                TILE_SET_FLOAT,
                TEXT_TO_TRAIT,
            ],
        );
        e
    }

    #[test]
    fn add_item_needs_a_container_and_a_template() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0078_6cf0, &args![this, 1u32, 0u32, 0u32, 0x4000u32])
                .u32(),
            0
        );
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(
            e.call(0x0078_6cf0, &args![this, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, MENU_RENDER_TEMPLATE).is_empty());
    }

    #[test]
    fn add_item_appends_and_sizes_the_container() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pTemplateName, Ptr::new(0x4100));
        e.set(this, ListBox::usNextIndex, 5);
        e.call_log = Some(vec![]);
        let tile = e
            .call(0x0078_6cf0, &args![this, 0x42u32, 0x4200u32, 0u32, 0u32])
            .u32();
        assert_eq!(tile, 0x6000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x5000, 0x3000, 0x4100, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6000, 0xfc4, 0x4200, 1]]
        );
        let appended = calls_to(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        let item = e.mem.u32(appended[0][1]);
        assert_eq!((e.mem.u32(item), e.mem.u32(item + 4)), (0x6000, 0x42));
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6000, 0xfaa, 0xffff_ffff], vec![0x6000, 0xfac, 5]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 6);
        assert_eq!(
            calls_to(&e, INIT_TILE_POSITION),
            vec![vec![this.addr(), 0x6000]]
        );
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![vec![0x9000, 0x7d0, 0x6000, 0xfb0]]
        );
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x9000, 0x7d3, 3.0f32.to_bits()]]
        );
        assert!(calls_to(&e, LIST_INSERT_SORTED).is_empty());
    }

    #[test]
    fn add_item_inserts_sorted_with_a_compare_function() {
        let mut e = add_item_engine();
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(2));
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call_log = Some(vec![]);
        let tile = e
            .call(
                0x0078_6cf0,
                &args![this, 0x42u32, 0u32, 0x00ab_cdefu32, 0x4300u32],
            )
            .u32();
        assert_eq!(tile, 0x6000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE)[0],
            vec![0x5000, 0x3000, 0x4300, 0]
        );
        let inserted = calls_to(&e, LIST_INSERT_SORTED);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][0], this.addr() + 4);
        assert_eq!(e.mem.u32(inserted[0][1]), 0x6000);
        assert_eq!(inserted[0][2], 0x00ab_cdef);
        assert!(calls_to(&e, TILE_SET_STRING).is_empty());
        assert!(calls_to(&e, VALUE_ADD_ACTION_TILE).is_empty());
    }

    #[test]
    fn remove_all_deletes_every_item_and_resets() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                FREE,
                NODE_SET_NEXT,
                NODE_DELETE,
                ITEM_DATA_DELETE,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
            ],
        );
        e.register(LIST_BOX_HAS_FLAG, |_, a| eax(u32::from(a[1] == 1)));
        e.register(TEXT_TO_TRAIT, |_, _| eax(0x55));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register(TILE_GET_FLOAT, |_, _| st0(0.0));
        e.register(0x0000_7000, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0x0000_7000]);
        let this = list_box(&mut e);
        let tile = e.mem.alloc(4);
        e.mem.set_u32(tile, 0x0f00_0000);
        let first = item(&mut e, tile, 0x8000, 0);
        let second = item(&mut e, 0, 0, 0);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x3100));
        e.set(this, ListBox::usNextIndex, 4);
        e.set(this, ListBox::fTotalListHeight, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_6f90, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
        assert_eq!(calls_to(&e, NODE_SET_NEXT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
        // The highlight box gets the "nothing highlighted" values.
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x3000, 0x66, 0xffff_ffff], vec![0x3000, 0x66, 0]]
        );
    }

    #[test]
    fn sort_sorts_the_list_then_synchronizes() {
        let mut e = engine();
        stub(&mut e, &[LIST_SORT]);
        let this = list_box(&mut e);
        e.set(this, ListBox::usNextIndex, 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_71c0, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&e, LIST_SORT), vec![vec![this.addr() + 4, 0x1234]]);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
    }

    #[test]
    fn highlight_needs_an_enabled_list_and_an_enabled_item() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register_double(TILE_IS_TRUE, |_, a| eax(u32::from(a[0] != 0x6100)));
        e.register(LIST_BOX_BELONGS_TO, |_, a| eax(u32::from(a[1] != 0x6200)));
        stub(
            &mut e,
            &[
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_SET_INT,
            ],
        );
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(0));
        e.register(TILE_FIND_BY_NAME, |_, _| eax(0));
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_71f0, &args![this, 0x6000u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(!e.call(0x0078_71f0, &args![this, 0x6100u32]).bool());
        assert!(!e.call(0x0078_71f0, &args![this, 0x6200u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert!(e.call(0x0078_71f0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
        assert!(e.call(0x0078_71f0, &args![this, 0u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
    }

    #[test]
    fn restore_list_state_focuses_and_scrolls_to_the_saved_item() {
        let mut e = engine();
        e.register(TEXT_TO_TRAIT, |_, a| eax(a[0] & 0xfff));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(10));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfaa { 7.0 } else { 0.0 })
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FOCUS_OWNER_004B7210, |_, _| eax(0x7700));
        e.register(FLAG_004B71D0, |_, _| eax(0));
        e.register(TILE_GET_MENU_TILE, |_, _| eax(0x5100));
        e.register(TILE_GET_MENU, |_, _| eax(0x5200));
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        stub(
            &mut e,
            &[TILE_SET_FLOAT, TILE_POKE, SET_CURRENT_FOCUS_TARGET],
        );
        e.register(0x0000_7014, |_, _| eax(0x6000));
        e.register(0x0000_7000, |_, _| Ret::default());
        e.register(0x0000_7114, |_, _| Ret::default());
        let mut slots = vec![0u32; 6];
        slots[0] = 0x0000_7000;
        slots[5] = 0x0000_7014;
        e.put_vtable(0x0f00_0000, &slots);
        let mut menu_slots = vec![0u32; 6];
        menu_slots[5] = 0x0000_7114;
        e.put_vtable(0x0f00_1000, &menu_slots);
        e.mem.map(0x5200, 4);
        e.mem.set_u32(0x5200, 0x0f00_1000);
        let this = list_box(&mut e);
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fSavedScrollbarValue, 12.5);
        e.set(this, ListBox::fSavedHighlightIndex, 4.75);
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3400, 0x2d8, 12.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x3400, 0x250, 1.0f32.to_bits()]]
        );
        // 10 items, none filtered: the last index is 9.0.
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![4.75f32.to_bits(), 9.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0000_7014), vec![vec![this.addr(), 4, 0]]);
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x7700, 0x6000, 0xfc3, 1]]
        );
        assert_eq!(calls_to(&e, 0x0000_7114), vec![vec![0x5200, 7, 0x6000]]);
        assert_eq!(
            calls_to(&e, 0x0000_7000),
            vec![vec![this.addr(), 0x6000], vec![this.addr(), 0]]
        );

        // When 004b71d0 answers true it stops after the highlight.
        e.register(FLAG_004B71D0, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 0u32]);
        assert!(calls_to(&e, 0x0000_7114).is_empty());
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![this.addr(), 0x6000]]);

        // Without a scroll bar nothing happens.
        e.set(this, ListBox::pScrollBar, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    fn select_engine() -> Engine {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, a| {
            eax(if a[0] == SELECTED_TEXT { 0x88 } else { 0x66 })
        });
        e.register(LIST_BOX_BELONGS_TO, |_, _| eax(1));
        e.register(TILE_IS_TRUE, |e, a| {
            if a[1] == 0x88 {
                eax(e.mem.u32(0x011d_af00))
            } else {
                eax(1)
            }
        });
        stub(&mut e, &[TILE_SET_INT]);
        e
    }

    #[test]
    fn fn_00787760_and_00787780_select_and_toggle() {
        let mut e = select_engine();
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call_log = Some(vec![]);
        e.call(0x0078_7760, &args![this, 0x6000u32]);
        e.set_global(0x011d_af00, 1u32);
        e.call(0x0078_7780, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6000, 0x88, 1], vec![0x6000, 0x88, 0]]
        );
    }

    #[test]
    fn fn_00787c40_changes_the_selection_by_mode() {
        let mut e = select_engine();
        let this = list_box(&mut e);
        // Disabled (no container): nothing.
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(!e.call(0x0078_7c40, &args![this, 0u32, 2u32]).bool());
        e.call_log = Some(vec![]);
        // Not selected: mode 0 selects, mode 1 does nothing, mode 3 neither.
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 0u32]).bool());
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 1u32]).bool());
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 3u32]).bool());
        e.set_global(0x011d_af00, 1u32);
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 0u32]).bool());
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 1u32]).bool());
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6000, 0x88, 1],
                vec![0x6000, 0x88, 0],
                vec![0x6000, 0x88, 0]
            ]
        );
        // Not one of the list's items.
        e.register(LIST_BOX_BELONGS_TO, |_, _| eax(0));
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
    }

    #[test]
    fn fn_007877a0_and_007878d0_count_selected_tiles() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x88));
        e.register(TILE_IS_TRUE, |_, a| eax(u32::from(a[0] >= 0x6100)));
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_77a0, &args![this, 0u32]).bool());
        assert!(!e.call(0x0078_77a0, &args![this, 0x6000u32]).bool());
        assert!(e.call(0x0078_77a0, &args![this, 0x6100u32]).bool());
        assert_eq!(e.mem.u32(0x011d_a090), 0x88);
        assert_eq!(e.mem.u32(0x011d_a094) & 1, 1);
        let first = item(&mut e, 0x6100, 0, 0);
        let second = item(&mut e, 0x6000, 0, 0);
        let third = item(&mut e, 0x6200, 0, 0);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, second);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, third);
        assert_eq!(e.call(0x0078_78d0, &args![this]).u32(), 2);
        // A zero trait id answers false without asking the tile.
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0));
        e.call_log = Some(vec![]);
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_77a0, &args![this, 0x6100u32]).bool());
        assert!(calls_to(&e, TILE_IS_TRUE).is_empty());
    }

    #[test]
    fn fn_00787950_appends_one_element() {
        let mut e = engine();
        e.register(ARRAY_IS_FULL, |_, _| eax(0));
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        let buffer = e.mem.alloc(0x14 * 4);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, 1);
        e.set(this, BSSimpleArray::iReservedSize, 4);
        let element = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(element + 4 * word, 0x100 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_7950, &args![this, element]).u32(), 1);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT), vec![vec![buffer + 0x14]]);
        for word in 0..5 {
            assert_eq!(e.mem.u32(buffer + 0x14 + 4 * word), 0x100 + word);
        }
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn num_unfiltered_subtracts_the_filtered_count() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x99));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(7));
        e.register(TILE_GET_FLOAT, |_, a| {
            assert_eq!(a[1], 0x99);
            st0(2.9)
        });
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(e.call(0x0078_79f0, &args![this]).i32(), 5);
        e.register(TILE_GET_FLOAT, |_, _| st0(9.5));
        assert_eq!(e.call(0x0078_79f0, &args![this]).i32(), -2);
    }

    #[test]
    fn synchronize_numbers_the_unfiltered_items() {
        let mut e = engine();
        stub(&mut e, &[TILE_SET_INT, INIT_TILE_POSITION]);
        let this = list_box(&mut e);
        let first = item(&mut e, 0x6100, 0, 0);
        let second = item(&mut e, 0x6200, 0, 1);
        let third = item(&mut e, 0x6300, 0, 0);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, second);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, third);
        e.set(this, ListBox::usNextIndex, 9);
        e.set(this, ListBox::fTotalListHeight, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_7aa0, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6100, 0xfac, 0], vec![0x6300, 0xfac, 1]]
        );
        assert_eq!(
            calls_to(&e, INIT_TILE_POSITION),
            vec![vec![this.addr(), 0x6100], vec![this.addr(), 0x6300]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 2);
        assert_eq!(e.get(this, ListBox::fTotalListHeight), 0.0);
    }

    #[test]
    fn is_enabled_asks_the_container_once_the_static_is_set() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x44));
        e.register(TILE_IS_TRUE, |_, a| eax(u32::from(a[1] == 0x44)));
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0078_7b70, &args![this]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(e.call(0x0078_7b70, &args![this]).bool());
        assert_eq!(
            calls_to(&e, ADD_USER_TRAIT),
            vec![vec![ENABLED_TEXT, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(0x011d_a0a0), 0x44);
    }

    #[test]
    fn update_highlight_box_pos_follows_the_highlighted_tile() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, a| {
            eax(if a[0] == HIGHLIGHT_Y_TEXT { 0x31 } else { 0x32 })
        });
        e.register(TILE_GET_VALUE, |_, a| eax(0x9000 + a[1]));
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(1));
        e.register(TILE_FIND_BY_NAME, |_, _| eax(0x7100));
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| {
            eax(if a[1] == TOP_TEXT { 0x7200 } else { 0x7300 })
        });
        stub(
            &mut e,
            &[
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_FLAG_SETTER_0071AA60,
                TILE_ADD_NEEDS_UPDATE,
                TILE_SET_INT,
            ],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(e.mem.u32(0x011d_a0b8) & 3, 3);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x3000, 0x31, 0xffff_ffff], vec![0x3000, 0x32, 0]]
        );
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6000));
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![
                vec![0x9031, 0x7d0, 0x6000, 0xfa2],
                vec![0x9032, 0x7d0, 0x6000, 0xfb0]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_FLAG_SETTER_0071AA60),
            vec![
                vec![0x7200, 1],
                vec![0x7300, 1],
                vec![0x7300, 1],
                vec![0x7300, 1],
                vec![0x7100, 1]
            ]
        );
        assert_eq!(calls_to(&e, TILE_ADD_NEEDS_UPDATE).len(), 5);
        // Without "top" only the box itself, with the byte 0.
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(0));
        e.register(TILE_GET_CHILD_BY_NAME, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_FLAG_SETTER_0071AA60),
            vec![vec![0x7100, 0]]
        );
    }

    #[test]
    fn set_total_list_height_counts_scroll_steps() {
        let mut e = engine();
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfb0 { 25.0 } else { 0.0 })
        });
        e.register(ADD_USER_TRAIT, |_, _| eax(0x99));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(8));
        e.register(ROUND_FLOAT, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).ceil())
        });
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        e.register(INT_PAIR_00647B70, |_, a| {
            eax((a[0] as i32).max(a[1] as i32) as u32)
        });
        e.register(TEXT_TO_TRAIT, |_, a| eax(a[0] & 0xfff));
        stub(&mut e, &[TILE_SET_FLOAT, TILE_SET_INT]);
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fTotalListHeight, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        // 8 * 75 / 100 = 6, + 1 = 7 steps; delta 75 / 6 = 12.5.
        assert_eq!(calls_to(&e, ROUND_FLOAT), vec![vec![6.0f32.to_bits()]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3000, 0x90c, 12.5f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 7]]);
        // Shorter than the visible height: one step, no delta.
        e.set(this, ListBox::fTotalListHeight, 20.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 1]]);
        // NaN total: no steps either.
        e.set(this, ListBox::fTotalListHeight, f32::NAN);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert!(calls_to(&e, ROUND_FLOAT).is_empty());
        // No container: nothing.
        e.set(this, ListBox::pContainerRect, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_007881d0_grows_a_full_array() {
        let mut e = engine();
        e.register(ARRAY_IS_FULL, |_, _| eax(1));
        e.register(0x0000_7004, |_, a| eax(0x5000 + a[1]));
        e.register(ARRAY_GROWN_SIZE, |_, _| eax(8));
        stub(&mut e, &[ARRAY_RESIZE]);
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 0);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5004);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 1);
        assert_eq!(calls_to(&e, ARRAY_RESIZE), vec![vec![this.addr(), 8, 1]]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        e.register(ARRAY_IS_FULL, |_, _| eax(0));
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 2);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn fn_00788260_constructs_each_element() {
        let mut e = engine();
        e.register(PLACEMENT_NEW, |_, a| {
            eax(if a[1] == 0x1014 { 0 } else { a[1] })
        });
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0078_8260, &args![this, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x14, 0x1000], vec![0x14, 0x1014], vec![0x14, 0x1028]]
        );
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCT),
            vec![vec![0x1000], vec![0x1028]]
        );
    }

    #[test]
    fn fn_00788330_allocates_0x14_bytes_per_element() {
        let mut e = engine();
        e.register(ALLOCATE, |_, a| eax(0x4000 + a[0]));
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0078_8330, &args![this, 3u32]).u32(), 0x4000 + 0x3c);
    }

    #[test]
    fn fn_007883b0_reserves_and_sizes() {
        let mut e = engine();
        e.register(0x0000_7004, |_, a| eax(0x5000 + a[1]));
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0078_83b0, &args![this, 2u32, 3u32]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![this.addr(), 3]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5003);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT).len(), 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_83b0, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, 0x0000_7004).is_empty());
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
    }
}
