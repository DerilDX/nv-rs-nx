//! `fallout/interface/menus/bartermenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most of these are the `ListBox<ItemChange_P>` instance of the list box template. Their
//! code matches the `ListBox<BGSPerk_P>` instance of `levelupmenu.cpp` instruction for
//! instruction (only the vtable, the statics and the callees in `listbox.cpp` differ).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

/// `00ec7d40` (`cdecl`, one word): the absolute value of an `int`.
const ABS_00EC7D40: u32 = 0x00ec_7d40;
/// `__strupr_s` (`00ecbc5b`, `cdecl`, two words).
const STRUPR_S: u32 = 0x00ec_bc5b;

/// The `ListBox<ItemChange_P>` vtable (`01070890`).
const ITEM_CHANGE_LIST_BOX_VTABLE: u32 = 0x0107_0890;
/// `ListBoxBase` construction (`007240c0`) and the embedded list's constructor
/// (`0096a2d0`) and destructor (`0046ffb0`).
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `007a1950` (`thiscall`, `RET 8`): the list box's flag initialisation.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// `00721130` / `007211c0` (`cdecl`): the list box handed to the menu manager's list
/// and taken out of it.
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `MemoryManager` free (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;
/// The list node helpers: `006815c0` (the node's item slot), `00726070` (the next node),
/// `006ecd40` (sets the next node, `RET 4`), `004702f0` (deletes the node, `RET 4`).
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
/// `004459e0` (`thiscall`, `RET 4`): the item data's deleting call, given 1.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that keeps item tiles alive when set (`011d8909`).
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// `007242c0` (`thiscall`, `RET 4`): whether the list box has a flag (by bit number).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// `007242f0` (`thiscall`, `RET 4`): whether a tile is one of the list's items.
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Tile functions (`BSMenu/tile.cpp`): `Tile::AddUserTrait` (`cdecl`), `Tile::TextToTrait`
/// (`cdecl`), the float setter `00a012d0` (`RET 0xc`), `00a01290` (`RET 8`), the float
/// getter `00a011b0` (`RET 4`, `ST0`), `00a01230` (`RET 4`, a `bool`).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `00700320` (`thiscall`, `RET 8`): sets an integer trait of a tile.
const TILE_SET_INT: u32 = 0x0070_0320;
/// `00715c60` (`thiscall`, `RET 8`): sets an integer value of a tile (`interfacemanager.cpp`).
const TILE_SET_INTEGER_VALUE: u32 = 0x0071_5c60;
/// `Tile::GetMenuTile`-like `00a03d40` and `Tile::GetMenu` `00a03c90` (`thiscall`).
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `_ftol2_sse` (`00ec62c0`): the `ST0` value truncated (passed as an `f64`).
const FTOL2: u32 = 0x00ec_62c0;
/// `0040ebd0` (`cdecl`, two floats, `ST0`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
/// `004b7210` (the interface manager) and its `00715860` (`thiscall`, `RET 0xc`): the
/// current focus target; `004b71d0` (no arguments, a `bool`).
const FOCUS_OWNER_004B7210: u32 = 0x004b_7210;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// The embedded list's sort (`0083fd60`, `thiscall`, `RET 4`).
const LIST_SORT: u32 = 0x0083_fd60;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (`007269d0`, `thiscall`, `RET 4`; the
/// template code the instances share).
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// This instance's methods in `listbox.cpp`: `Synchronize` (`0071a670`),
/// `UpdateHighlightBoxPos` (`0071a740`), `SetTotalListHeight` (`0071ad30`),
/// `NumUnfiltered` (`0071ae60`).
const ITEM_CHANGE_SYNCHRONIZE: u32 = 0x0071_a670;
const ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS: u32 = 0x0071_a740;
const ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT: u32 = 0x0071_ad30;
const ITEM_CHANGE_NUM_UNFILTERED: u32 = 0x0071_ae60;
/// `007315b0` (`thiscall`; in `bartermenu.cpp`): the tile `Filter` highlights when the
/// remembered index has none.
const ITEM_CHANGE_007315B0: u32 = 0x0073_15b0;

/// `"_num_filtered"`, `"_current_value"`, `"_SetInCode"`, `"_enabled"`.
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
const ENABLED_TEXT: u32 = 0x0107_0244;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;

/// A trait id kept in a function-local static: the guard word (bit `bit` says it has been
/// looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// `Filter`'s `"_num_filtered"` id.
const FILTER_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_9038,
    bit: 1,
    cache: 0x011d_9034,
    name: NUM_FILTERED_TEXT,
};
/// `IsEnabled`'s `"_enabled"` id.
const ENABLED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_9040,
    bit: 1,
    cache: 0x011d_903c,
    name: ENABLED_TEXT,
};

/// Looks the static trait id up the first time (the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does); returns the id read back.
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

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The first node of the embedded list (`this + 4`; the code guards a null `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// An item tile's index trait (`0xfac`) as the code reads it: the float getter, then
/// `_ftol2`.
fn tile_index(e: &mut Engine, tile: u32) -> i32 {
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    e.call(FTOL2, &args![value]).i32()
}

// Translated from 0072fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): `0072fa80(value)`, its `EAX` returned (the decompiler
/// shows no return; `SetFilterModeTitle` and other callers use it).
pub fn fn_0072fa60(e: &mut Engine, value: i32) -> i32 {
    fn_0072fa80(e, value)
}

// Translated from 0072fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): `00ec7d40(value)` (the absolute value), its `EAX`
/// returned.
pub fn fn_0072fa80(e: &mut Engine, value: i32) -> i32 {
    e.call(ABS_00EC7D40, &args![value]).i32()
}

// Translated from 00730490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words): `__strupr_s(text, size)`, its `EAX` returned.
pub fn fn_00730490(e: &mut Engine, text: u32, size: u32) -> u32 {
    e.call(STRUPR_S, &args![text, size]).u32()
}

// Translated from 00730810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730810 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// `ListBox<ItemChange_P>::ListBox<ItemChange_P>` (engine map): the `ListBoxBase` base
/// (`007240c0`), the embedded list (+4, `0096a2d0`), the vtable `01070890`, the fields
/// cleared, `007a1950(this, 0, 1)` and the list box handed to `00721130`. Returns `this`.
pub fn list_box_item_change_list_box(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), ITEM_CHANGE_LIST_BOX_VTABLE);
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

// Translated from 007308e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007308e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<ItemChange_P>` destructor (unnamed in the engine map): the vtable,
/// `RemoveAll`, the list box handed to `007211c0`, then the embedded list destroyed
/// (`0046ffb0`).
pub fn fn_007308e0(e: &mut Engine, this: Ptr<ListBox>) {
    e.mem.set_u32(this.addr(), ITEM_CHANGE_LIST_BOX_VTABLE);
    list_box_item_change_remove_all(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each time: the
/// tile through its deleting destructor (vtable slot 0, flag 1) unless `KEEP_ITEM_TILES`
/// is set, the data (`004459e0` with 1) when flag bit 1 is set, then the item freed.
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

// Translated from 00730950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::RemoveAll` (engine map): while the second node (+8) and its item
/// are non-null, deletes the item, clears the node's next pointer (`006ecd40` with 0),
/// deletes the node (`004702f0` with 1) and makes the following node the second; then
/// deletes the first item (+4). Clears the first item, `usNextIndex` and
/// `fTotalListHeight`, sets the container's `"_num_filtered"` to `0.0`, then
/// `SetTotalListHeight` (`0071ad30`), clears `pHighlightedItem` and
/// `UpdateHighlightBoxPos` (`0071a740`).
pub fn list_box_item_change_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
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
    e.call(ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT, &args![this]);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
}

// Translated from 00730b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::Sort` (engine map): sorts the embedded list (`0083fd60` on +4
/// with `compare`), then `Synchronize` (`0071a670`).
pub fn list_box_item_change_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    e.call(ITEM_CHANGE_SYNCHRONIZE, &args![this]);
}

// Translated from 00730bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730bb0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<ItemChange_P>::Filter` (engine map; `thiscall`, `RET 4`): runs `filter`
/// (`cdecl`, the item's data, a `bool`) over every item. An item for which it returns
/// true is marked filtered out (its byte at +8 is 1, its index trait `0xfac` becomes -1,
/// the count of filtered items grows); the others are numbered again from 0 and
/// positioned (`InitTilePosition`). A filtered-out highlighted item is dropped and its old
/// index remembered; the item that takes that index is highlighted, else the list's tile at
/// that index (vtable +0x14 with 0), else `007315b0`. The container's `"_num_filtered"`
/// (a function-local static id) gets the count (`00715c60`), then `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`. A null `filter` filters nothing out.
pub fn list_box_item_change_filter(e: &mut Engine, this: Ptr<ListBox>, filter: u32) {
    let mut filtered_count = 0i32;
    let mut remembered = -1i32;
    static_trait(e, FILTER_TRAIT);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let filtered = filter != 0 && {
            let item = node_item(e, node);
            let data = e.mem.u32(item + 4);
            e.call(filter, &args![data]).bool()
        };
        let item = node_item(e, node);
        e.mem.set_u8(item + 8, u8::from(filtered));
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            if remembered >= 0 && i32::from(e.get(this, ListBox::usNextIndex)) == remembered {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
            }
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        } else {
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            if tile == e.get(this, ListBox::pHighlightedItem).addr() {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                remembered = tile_index(e, tile);
                e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
            }
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, 0xffff_ffffu32]);
            filtered_count += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let num_filtered = e.mem.u32(FILTER_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(
        TILE_SET_INTEGER_VALUE,
        &args![container, num_filtered, filtered_count],
    );
    e.call(ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT, &args![this]);
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = e.vcall(this.addr(), 0x14, &args![remembered, 0u32]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = e.call(ITEM_CHANGE_007315B0, &args![this]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
}

// Translated from 00730e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730e20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<ItemChange_P>::IsEnabled` (engine map): the container exists and its
/// `"_enabled"` trait (a function-local static id) is true.
pub fn list_box_item_change_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    static_trait(e, ENABLED_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return false;
    }
    let enabled = e.mem.u32(ENABLED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

// Translated from 00730ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::Highlight` (engine map): when the list is enabled and `tile` is
/// null, or is one of its items (`007242f0`) and `"_enabled"`, makes it
/// `pHighlightedItem`, moves the highlight box (`0071a740`) and returns true; otherwise
/// false.
pub fn list_box_item_change_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    if !list_box_item_change_is_enabled(e, this) {
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
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
    true
}

// Translated from 00731360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::RestoreListState` (engine map): with a container and a scroll
/// bar, gives the scroll bar `"_current_value"` (the saved scroll bar value, word 0) and
/// pokes `"_SetInCode"` with `1.0`; takes the tile at index `_ftol(0040ebd0(
/// fSavedHighlightIndex, NumUnfiltered - 1.0))` (vtable +0x14 with 0; `NumUnfiltered` is
/// `0071ae60`), makes it the interface's focus target (`00715860` with trait `0xfc3` and
/// `focus`) and highlights it (vtable +0). Unless `004b71d0` says so, also scrolls the
/// container's menu to it (the menu's vtable +0x14 with the tile's index trait `0xfaa` and
/// the tile, when there is a tile) and highlights nothing.
pub fn list_box_item_change_restore_list_state(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
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
    let count = e.call(ITEM_CHANGE_NUM_UNFILTERED, &args![this]).u32();
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

// Translated from 007314c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`: a callback and two indices). Walks the items in order
/// while the node and its item are non-null: stops at the first whose index trait (`0xfac`,
/// read as a float and truncated by `_ftol2`) is at least `end`; for those whose index is at
/// least `start`, calls `callback(tile, data)` (`cdecl`). Signed comparisons.
pub fn fn_007314c0(e: &mut Engine, this: Ptr<ListBox>, callback: u32, start: i32, end: i32) {
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if tile_index(e, tile) >= end {
            return;
        }
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if tile_index(e, tile) >= start {
            let item = node_item(e, node);
            let data = e.mem.u32(item + 4);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(callback, &args![tile, data]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0072_fa60, fn_0072fa60(i32) -> i32),
        entry!(0x0072_fa80, fn_0072fa80(i32) -> i32),
        entry!(0x0073_0490, fn_00730490(u32, u32) -> u32),
        entry!(
            0x0073_0810,
            list_box_item_change_list_box(Ptr<ListBox>) -> Ptr<ListBox>
        ),
        entry!(0x0073_08e0, fn_007308e0(Ptr<ListBox>)),
        entry!(0x0073_0950, list_box_item_change_remove_all(Ptr<ListBox>)),
        entry!(0x0073_0b80, list_box_item_change_sort(Ptr<ListBox>, u32)),
        entry!(0x0073_0bb0, list_box_item_change_filter(Ptr<ListBox>, u32)),
        entry!(
            0x0073_0e20,
            list_box_item_change_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0073_0ed0,
            list_box_item_change_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0073_1360,
            list_box_item_change_restore_list_state(Ptr<ListBox>, u8)
        ),
        entry!(0x0073_14c0, fn_007314c0(Ptr<ListBox>, u32, i32, i32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the statics, the flag byte and the doubles mapped, and the list
    /// node helpers doubled (`006815c0` returns the node, `00726070` its next word).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ADD_USER_TRAIT, |_, _| 0x66u32.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        stub(
            &mut e,
            &[
                ITEM_CHANGE_SYNCHRONIZE,
                ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS,
                ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT,
            ],
        );
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    /// An item (tile, data, filtered byte).
    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    /// Three items in the list box: the first in the embedded node, two more in nodes.
    fn three_items(e: &mut Engine, this: Ptr<ListBox>, items: [u32; 3]) {
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, items[0]);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, items[1]);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, items[2]);
    }

    #[test]
    fn the_forwarders_return_their_callees_results() {
        let mut e = engine();
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        e.register(STRUPR_S, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x0072_fa60, &args![-7i32]).i32(), 7);
        assert_eq!(e.call(0x0072_fa80, &args![9i32]).i32(), 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_0490, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls_to(&e, STRUPR_S), vec![vec![0x100, 0x20]]);
    }

    #[test]
    fn the_constructor_builds_the_item_change_list_box() {
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
        assert_eq!(e.call(0x0073_0810, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), ITEM_CHANGE_LIST_BOX_VTABLE);
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
    fn the_destructor_empties_and_destroys_the_list() {
        let mut e = engine();
        stub(
            &mut e,
            &[LIST_BOX_UNREGISTER, LIST_DESTRUCT, TILE_SET_FLOAT],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_08e0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), ITEM_CHANGE_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT),
            vec![vec![this.addr()]]
        );
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
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
            ],
        );
        e.register(LIST_BOX_HAS_FLAG, |_, a| u32::from(a[1] == 1).into_ret());
        e.register(TEXT_TO_TRAIT, |_, _| 0x55u32.into_ret());
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
        e.call(0x0073_0950, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
        assert_eq!(calls_to(&e, NODE_SET_NEXT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
        assert_eq!(e.get(this, ListBox::fTotalListHeight), 0.0);
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS),
            vec![vec![this.addr()]]
        );
        // With the keep-tiles byte set the tile is not deleted.
        e.set_global(KEEP_ITEM_TILES, 1u8);
        let first = item(&mut e, tile, 0, 0);
        e.mem.set_u32(this.addr() + 4, first);
        e.call_log = Some(vec![]);
        e.call(0x0073_0950, &args![this]);
        assert!(calls_to(&e, 0x0000_7000).is_empty());
        assert_eq!(calls_to(&e, FREE), vec![vec![first]]);
    }

    #[test]
    fn sort_sorts_the_list_then_synchronizes() {
        let mut e = engine();
        stub(&mut e, &[LIST_SORT]);
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_0b80, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&e, LIST_SORT), vec![vec![this.addr() + 4, 0x1234]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_SYNCHRONIZE),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn filter_marks_numbers_and_moves_the_highlight() {
        let mut e = engine();
        stub(
            &mut e,
            &[TILE_SET_INT, INIT_TILE_POSITION, TILE_SET_INTEGER_VALUE],
        );
        // The filter drops data 0xd2.
        e.register(0x00ab_0000, |_, a| u32::from(a[0] == 0xd2).into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        let this = list_box(&mut e);
        let items = [
            item(&mut e, 0x6100, 0xd1, 1),
            item(&mut e, 0x6200, 0xd2, 0),
            item(&mut e, 0x6300, 0xd3, 1),
        ];
        three_items(&mut e, this, items);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6200));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0x00ab_0000u32]);
        assert_eq!(
            [items[0], items[1], items[2]].map(|i| e.mem.u8(i + 8)),
            [0, 1, 0]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6100, 0xfac, 0],
                vec![0x6200, 0xfac, 0xffff_ffff],
                vec![0x6300, 0xfac, 1]
            ]
        );
        // The highlighted tile was filtered out at index 1: the next unfiltered item takes
        // index 1 and the highlight.
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6300);
        assert_eq!(
            calls_to(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![0x3000, 0x66, 1]]
        );
        assert_eq!(e.mem.u32(0x011d_9038) & 1, 1);
        assert_eq!(e.get(this, ListBox::usNextIndex), 2);
    }

    #[test]
    fn filter_falls_back_to_the_tile_at_the_index_then_007315b0() {
        let mut e = engine();
        stub(
            &mut e,
            &[TILE_SET_INT, INIT_TILE_POSITION, TILE_SET_INTEGER_VALUE],
        );
        e.register(0x00ab_0000, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(2.0));
        e.register(0x0000_7014, |_, _| Ret::default());
        e.register(ITEM_CHANGE_007315B0, |_, _| 0x6900u32.into_ret());
        let mut slots = vec![0u32; 6];
        slots[5] = 0x0000_7014;
        e.put_vtable(0x0f00_0000, &slots);
        let this = list_box(&mut e);
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        let first = item(&mut e, 0x6100, 0, 0);
        e.mem.set_u32(this.addr() + 4, first);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6100));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0x00ab_0000u32]);
        assert_eq!(calls_to(&e, 0x0000_7014), vec![vec![this.addr(), 2, 0]]);
        assert_eq!(calls_to(&e, ITEM_CHANGE_007315B0), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6900);
        // A null filter filters nothing out and keeps the highlight.
        e.mem.set_u8(first + 8, 1);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6100));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(first + 8), 0);
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6100);
        assert!(calls_to(&e, ITEM_CHANGE_007315B0).is_empty());
    }

    #[test]
    fn is_enabled_and_highlight_need_an_enabled_container() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| 0x44u32.into_ret());
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6100).into_ret());
        e.register(LIST_BOX_BELONGS_TO, |_, a| {
            u32::from(a[1] != 0x6200).into_ret()
        });
        let this = list_box(&mut e);
        assert!(!e.call(0x0073_0e20, &args![this]).bool());
        assert_eq!(e.mem.u32(0x011d_903c), 0x44);
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6000u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(e.call(0x0073_0e20, &args![this]).bool());
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6100u32]).bool());
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6200u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_0ed0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS),
            vec![vec![this.addr()]]
        );
        assert!(e.call(0x0073_0ed0, &args![this, 0u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
    }

    #[test]
    fn restore_list_state_focuses_and_scrolls_to_the_saved_item() {
        let mut e = engine();
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.register(ITEM_CHANGE_NUM_UNFILTERED, |_, _| 10u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfaa { 7.0 } else { 0.0 })
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FOCUS_OWNER_004B7210, |_, _| 0x7700u32.into_ret());
        e.register(FLAG_004B71D0, |_, _| Ret::default());
        e.register(TILE_GET_MENU_TILE, |_, _| 0x5100u32.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x5200u32.into_ret());
        stub(
            &mut e,
            &[TILE_SET_FLOAT, TILE_POKE, SET_CURRENT_FOCUS_TARGET],
        );
        e.register(0x0000_7014, |_, _| 0x6000u32.into_ret());
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
        e.call(0x0073_1360, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3400, 0x2d8, 12.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x3400, 0x250, 1.0f32.to_bits()]]
        );
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
        e.register(FLAG_004B71D0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_1360, &args![this, 0u32]);
        assert!(calls_to(&e, 0x0000_7114).is_empty());
        e.set(this, ListBox::pScrollBar, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0073_1360, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_007314c0_calls_back_for_the_index_range() {
        let mut e = engine();
        e.register(TILE_GET_FLOAT, |_, a| st0(f64::from(a[0] - 0x6100) / 256.0));
        e.register(0x00ab_0000, |_, _| Ret::default());
        let this = list_box(&mut e);
        let items = [
            item(&mut e, 0x6100, 0xd0, 0),
            item(&mut e, 0x6200, 0xd1, 0),
            item(&mut e, 0x6300, 0xd2, 0),
        ];
        three_items(&mut e, this, items);
        e.call_log = Some(vec![]);
        e.call(0x0073_14c0, &args![this, 0x00ab_0000u32, 1i32, 2i32]);
        assert_eq!(calls_to(&e, 0x00ab_0000), vec![vec![0x6200, 0xd1]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_14c0, &args![this, 0x00ab_0000u32, 0i32, 9i32]);
        assert_eq!(calls_to(&e, 0x00ab_0000).len(), 3);
    }
}
