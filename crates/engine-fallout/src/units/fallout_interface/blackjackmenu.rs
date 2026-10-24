//! `fallout/interface/menus/blackjackmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00733fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, one `float`): stores the float at +0xf8 (`FLD`/`FSTP`, so a
/// signalling NaN is stored quieted).
pub fn fn_00733fd0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xf8),
        x87_quiet_bits(value.to_bits()),
    );
}

// ---------------------------------------------------------------------------
// `BlackJackMenu` (batch n051). The PC layout differs from the Xbox PDB's (the flags
// word the PDB has at +0xdb4 is at +0xdb0 here); offsets below are the PC code's.

/// `BlackJackMenu::pMe` (Xbox PDB static, by its use: the menu whose state +0x28 is set).
const BLACKJACK_MENU: u32 = 0x011d_9050;
/// Two statics the bet label reads: an object whose word at +4 is the bet shown
/// (`011d9054`) and the casino whose name component (+0x18) is shown (`011d9058`);
/// their names are not confirmed.
const BET_DATA_011D9054: u32 = 0x011d_9054;
const CASINO_011D9058: u32 = 0x011d_9058;
/// The trait id static the bet label is set with (`011d9044`).
const LABEL_TRAIT_011D9044: u32 = 0x011d_9044;
/// The table model holder (`011d9098`, an `NiPointer`).
const TABLE_MODEL_011D9098: u32 = 0x011d_9098;
/// `pTiles` (Xbox PDB name) at +0x34 on PC, ten tiles.
const BLACKJACK_TILES: u32 = 0x34;
/// The flags word at +0xdb0 (`iFlags` in the Xbox PDB at +0xdb4).
const BLACKJACK_FLAGS: u32 = 0xdb0;
/// The class number.
const BLACKJACK_MENU_CLASS: u32 = 0x439;

/// `MemoryManager` free (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;
/// The `NiPointer` constructor (`00633c90`, `RET 4`) and assignment from another
/// `NiPointer` (`006e5cc0`, `RET 4`; the map's folded name).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_COPY_006E5CC0: u32 = 0x006e_5cc0;
/// The interface manager (`004b7210`), its menu tile (`0045cd60`), the tile float setter
/// (`00a012d0`, `RET 0xc`) and the string trait setter (`00a01350`, `RET 0xc`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const WORD_AT_0X28: u32 = 0x0045_cd60;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `255.0` (`01023cd8`) and `-5.0` (`0102caf8`), `float`s.
const FULL_ALPHA: u32 = 0x0102_3cd8;
const MINUS_FIVE_0102CAF8: u32 = 0x0102_caf8;
/// The first word of an object (`00559450`), a list node's item slot (`006815c0`) and
/// next node (`00726070`).
const FIRST_WORD: u32 = 0x0055_9450;
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// A tile's child list walk: `0073ac90` (the current item's slot), `006317a0` (the child
/// at a node, `RET 4`) and `007b52d0` (the next node, `RET 4`).
const CHILD_LIST_ITEM_0073AC90: u32 = 0x0073_ac90;
const CHILD_LIST_AT_006317A0: u32 = 0x0063_17a0;
const CHILD_LIST_FOLLOWING_007B52D0: u32 = 0x007b_52d0;
/// The light's three-float set (`004bc1f0`, `RET 0xc`).
const SET_FLOATS_004BC1F0: u32 = 0x004b_c1f0;
/// A named child object (`004aae30`, cdecl: the root and the name) and the app-culled
/// setter (`00450f90`, `RET 4`).
const FIND_OBJECT_004AAE30: u32 = 0x004a_ae30;
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
/// The update data's initialisation (`0043d410`, `RET 0xc`), the node getter
/// `0043b4a0` (`RET 4`) and the object update (`00a59c60`, `RET 4`).
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
const NODE_0043B4A0: u32 = 0x0043_b4a0;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// A setting's string (`00403df0`), the name of a name component (`00408da0`), the
/// formatter (`00406d00`, cdecl, variadic) and `strlen` (`0044a670`, cdecl).
const SETTING_STRING: u32 = 0x0040_3df0;
const NAME_OF_00408DA0: u32 = 0x0040_8da0;
const FORMAT_TEXT: u32 = 0x0040_6d00;
const STRING_LENGTH_0044A670: u32 = 0x0044_a670;
/// `"Hand1_Arrow:0"`, `"Hand1_Arrow:0@#2..."` (`01070e58`, `01070e44`), `"%s %s%i"`
/// (`01070e80`), `"%s %s"` (`01012058`) and `"%s%i"` (`01029ca8`).
const HAND_ARROW_NAME: u32 = 0x0107_0e58;
const SPLIT_HAND_ARROW_NAME: u32 = 0x0107_0e44;
const LABEL_VALUE_FORMAT: u32 = 0x0107_0e80;
const LABEL_FORMAT: u32 = 0x0101_2058;
const PADDED_VALUE_FORMAT: u32 = 0x0102_9ca8;

/// The menu singleton's address.
fn blackjack_menu(e: &Engine) -> u32 {
    e.global::<u32>(BLACKJACK_MENU)
}

// Translated from 00731710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731710 - the C++ exception-unwinding frame (FS:[0] around the first pointer) is not translated.
/// Unnamed (`thiscall`; a card's constructor): the two `NiPointer`s at +8 and +0xc
/// empty, the words at +0 and +4 set to 0xe and 5. Returns `this`.
pub fn fn_00731710(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 8, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0xc, 0u32]);
    e.mem.set_u32(this.addr(), 0xe);
    e.mem.set_u32(this.addr() + 4, 5);
    this
}

// Translated from 00731790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::GetClass` (Xbox PDB): the class number `0x439`.
pub fn black_jack_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    BLACKJACK_MENU_CLASS
}

// Translated from 007317a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::AttachTileByID` (Xbox PDB, `RET 8`): ids 0 to 9 store the tile in
/// `pTiles` (+0x34); others are ignored.
pub fn black_jack_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0xa).contains(&id) {
        e.mem
            .set_u32(this.addr() + BLACKJACK_TILES + 4 * id as u32, tile);
    }
}

// Translated from 007317d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::`scalar deleting destructor'` (Xbox PDB): the destructor body
/// (`00731800`), then the block freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn black_jack_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00731800(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00731d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): with the menu open, sets its state (+0x28) to 0xd and the
/// interface manager's menu tile's alpha (trait `0xfa9`) to 255.
pub fn fn_00731d30(e: &mut Engine) {
    if blackjack_menu(e) == 0 {
        return;
    }
    let me = blackjack_menu(e);
    e.mem.set_u32(me + 0x28, 0xd);
    let alpha = x87_quiet_bits(e.global::<u32>(FULL_ALPHA));
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let tile = e.call(WORD_AT_0X28, &args![manager]).u32();
    e.call(TILE_SET_FLOAT, &args![tile, 0xfa9u32, alpha, 1u32]);
}

// Translated from 007324f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; a card's assignment): copies the two words and assigns
/// the two `NiPointer`s (+8, +0xc, `006e5cc0`). Returns `this`.
pub fn fn_007324f0(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    let word = e.mem.u32(other);
    e.mem.set_u32(this.addr(), word);
    let word = e.mem.u32(other + 4);
    e.mem.set_u32(this.addr() + 4, word);
    e.call(NI_POINTER_COPY_006E5CC0, &args![this.addr() + 8, other + 8]);
    e.call(
        NI_POINTER_COPY_006E5CC0,
        &args![this.addr() + 0xc, other + 0xc],
    );
    this
}

// Translated from 00733f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): clears the words at +0 and +4 and the 16-bit word at +8.
/// Returns `this`.
pub fn fn_00733f50(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 8, 0);
    this
}

// Translated from 00733f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the blackjack menu's class number `0x439`.
pub fn fn_00733f80(_e: &mut Engine) -> u32 {
    BLACKJACK_MENU_CLASS
}

// Translated from 00733f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): whether all ten tiles are attached.
pub fn fn_00733f90(e: &mut Engine, this: Ptr) -> bool {
    (0..0xa).all(|i| e.mem.u32(this.addr() + BLACKJACK_TILES + 4 * i) != 0)
}

// Translated from 00734d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): whether the flags word (+0xdb0) has a bit of the
/// mask (0 or 1).
pub fn fn_00734d30(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + BLACKJACK_FLAGS) & mask != 0
}

// Translated from 00734d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a mask and a `bool`): sets the mask's bits in the flags
/// word (+0xdb0), or clears them.
pub fn fn_00734d50(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = e.mem.u32(this.addr() + BLACKJACK_FLAGS);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.mem.set_u32(this.addr() + BLACKJACK_FLAGS, flags);
}

// Translated from 00737e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the word at +0x210.
pub fn fn_00737e30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x210)
}

// Translated from 00738c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::SetTileTextAlpha` (Xbox PDB, `thiscall`, `RET 8`: a tile index and an
/// alpha; `this` is not read): sets the alpha (trait `0xfa9`) of every child of the open
/// menu's tile at that index (its child list at +4, walked with `0073ac90`, `006317a0`
/// and `007b52d0`).
pub fn black_jack_menu_set_tile_text_alpha(e: &mut Engine, _this: Ptr, index: i32, alpha: f32) {
    let tile_of = |e: &Engine| {
        let me = blackjack_menu(e);
        e.mem.u32(
            me.wrapping_add(BLACKJACK_TILES)
                .wrapping_add((index as u32).wrapping_mul(4)),
        )
    };
    let list = tile_of(e).wrapping_add(4);
    let mut node = e.call(FIRST_WORD, &args![list]).u32();
    let list = tile_of(e).wrapping_add(4);
    while node != 0 && list != 0 {
        let item = e.call(CHILD_LIST_ITEM_0073AC90, &args![list]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let slot = e.call(CHILD_LIST_AT_006317A0, &args![list, node]).u32();
        let child = e.mem.u32(slot);
        e.call(
            TILE_SET_FLOAT,
            &args![child, 0xfa9u32, x87_quiet_bits(alpha.to_bits()), 1u32],
        );
        node = e
            .call(CHILD_LIST_FOLLOWING_007B52D0, &args![list, node])
            .u32();
    }
}

// Translated from 00738fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; `this` is not read): a card's points: 10 for ranks 9 to
/// 11, 11 for rank 12 (the ace), else the rank plus 2.
pub fn fn_00738fb0(_e: &mut Engine, _this: Ptr, rank: i32) -> i32 {
    if rank >= 9 {
        if rank <= 0xb {
            return 0xa;
        }
        if rank == 0xc {
            return 0xb;
        }
    }
    rank.wrapping_add(2)
}

// Translated from 00738ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a hand's first node): the hand's value: the points of
/// every card (`00738fb0` of its rank, the card's first word), each ace (rank 12) counted
/// as 1 instead of 11 while the total is above 21.
pub fn fn_00738ff0(e: &mut Engine, this: Ptr, hand: u32) -> i32 {
    let mut total = 0i32;
    let mut aces = 0i32;
    let mut node = hand;
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_SELF, &args![node]).u32();
        let card = e.mem.u32(slot);
        let rank = e.mem.i32(card);
        total = fn_00738fb0(e, this, rank).wrapping_add(total);
        if e.mem.u32(card) == 0xc {
            aces += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    while total > 0x15 && aces > 0 {
        total -= 0xa;
        aces -= 1;
    }
    total
}

// Translated from 007395a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; whether the player may split): true when the player's hand
/// (+0xd94) holds exactly two cards of the same rank and the chips (+0x74) are at least
/// twice the bet (+0x6c); stored in flag 4 (`00734d50`).
pub fn fn_007395a0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let mut node = e.mem.u32(base + 0xd94);
    let mut first = 0;
    let mut index = 0;
    let mut split = false;
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        if index == 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            first = e.mem.u32(slot);
        } else if index == 1 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            let card = e.mem.u32(slot);
            if e.mem.u32(first) == e.mem.u32(card)
                && e.mem.i32(base + 0x74) >= e.mem.i32(base + 0x6c).wrapping_shl(1)
            {
                split = true;
            }
        } else {
            split = false;
            break;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
        index += 1;
    }
    fn_00734d50(e, this, 4, u8::from(split));
    split
}

// Translated from 00739650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: whether for the split hand; whether the player may double
/// down): false when the chips (+0x74) are below the bet (+0x6c) plus twice the split
/// bet (+0x70) for the split hand, or the split bet plus twice the bet otherwise; stored
/// in flag 0x200 (split) or 0x40.
pub fn fn_00739650(e: &mut Engine, this: Ptr, split: u8) -> bool {
    let base = this.addr();
    let mut allowed = true;
    let bet = |e: &Engine| e.mem.i32(base + 0x6c);
    let split_bet = |e: &Engine| e.mem.i32(base + 0x70);
    let chips = |e: &Engine| e.mem.i32(base + 0x74);
    if split != 0 && chips(e) < bet(e).wrapping_add(split_bet(e).wrapping_mul(2)) {
        allowed = false;
    }
    if split == 0 && chips(e) < split_bet(e).wrapping_add(bet(e).wrapping_mul(2)) {
        allowed = false;
    }
    let mask = if split != 0 { 0x200 } else { 0x40 };
    fn_00734d50(e, this, mask, u8::from(allowed));
    allowed
}

// Translated from 00739970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `bool`): sets the three floats of the hand geometry
/// (+0xdbc, `004bc1f0`) to 0, 0 and -5.0 without the flag, else to zero.
pub fn fn_00739970(e: &mut Engine, this: Ptr, flag: u8) {
    let zero = x87_store32(0.0);
    let last = if flag == 0 {
        x87_quiet_bits(e.global::<u32>(MINUS_FIVE_0102CAF8))
    } else {
        zero
    };
    let geometry = e.mem.u32(this.addr() + 0xdbc);
    e.call(SET_FLOATS_004BC1F0, &args![geometry, zero, zero, last]);
}

// Translated from 007399d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the cards remaining (+0xda0) become the deck count (+0xd9c)
/// times 0x34.
pub fn fn_007399d0(e: &mut Engine, this: Ptr) {
    let decks = e.mem.u32(this.addr() + 0xd9c);
    e.mem.set_u32(this.addr() + 0xda0, decks.wrapping_mul(0x34));
}

/// Shows or hides a named object of the table (`004aae30` on +0xdb4, then `00450f90`).
fn set_table_object_culled(e: &mut Engine, this: Ptr, name: u32, culled: u32) {
    let root = e.mem.u32(this.addr() + 0xdb4);
    let object = e.call(FIND_OBJECT_004AAE30, &args![root, name]).u32();
    e.call(SET_APP_CULLED_00450F90, &args![object, culled]);
}

// Translated from 007399f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; points the hand arrow): with flag 0x10 shows the split hand's
/// arrow (`"Hand1_Arrow:0@#2..."`) and hides the hand's (`"Hand1_Arrow:0"`), else the
/// other way round; then updates the table model's node (`0043b4a0(1)` of `011d9098`).
pub fn fn_007399f0(e: &mut Engine, this: Ptr) {
    if fn_00734d30(e, this, 0x10) {
        set_table_object_culled(e, this, HAND_ARROW_NAME, 1);
        set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 0);
    } else {
        set_table_object_culled(e, this, HAND_ARROW_NAME, 0);
        set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 1);
    }
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
        let node = e.call(NODE_0043B4A0, &args![model, 1u32]).u32();
        e.call(UPDATE_00A59C60, &args![node, data]);
    });
}

// Translated from 00739ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): hides both hand arrows.
pub fn fn_00739ad0(e: &mut Engine, this: Ptr) {
    set_table_object_culled(e, this, HAND_ARROW_NAME, 1);
    set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 1);
}

// Translated from 0073ab40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0073ab40 - the /GS cookie check (00ec408c) on the label buffer is not translated.
/// Unnamed (`thiscall`; the bet label): `"%s %s%i"` of the casino's name, the label
/// setting (`011d3e08`) and the bet (the word at +4 of `011d9054`) into a 0x104-byte
/// buffer; when shorter than 23 characters it is written again as `"%s %s"`, padded with
/// spaces to 23 (each step writes a space over the last character and ends the text
/// after it), and the bet appended (`"%s%i"`). Set on `pTiles[2]` (+0x3c) with the trait
/// id at `011d9044`.
pub fn fn_0073ab40(e: &mut Engine, this: Ptr) {
    // The frame below `EBP`: the buffer is at `EBP - 0x110`.
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        let data = e.global::<u32>(BET_DATA_011D9054);
        let value = e.mem.u32(data + 4);
        let label = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
        let casino = e.global::<u32>(CASINO_011D9058);
        let name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
        e.call(
            FORMAT_TEXT,
            &args![buffer, 0x104u32, LABEL_VALUE_FORMAT, name, label, value],
        );
        let length = e.call(STRING_LENGTH_0044A670, &args![buffer]).i32();
        if length < 0x17 {
            let label = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
            let casino = e.global::<u32>(CASINO_011D9058);
            let name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, LABEL_FORMAT, name, label],
            );
            let mut i = 0i32;
            while i < 0x17 - length {
                let at = length.wrapping_add(i).wrapping_sub(1) as u32;
                e.mem.set_u8(buffer.wrapping_add(at), 0x20);
                let at = length.wrapping_add(i) as u32;
                e.mem.set_u8(buffer.wrapping_add(at), 0);
                i += 1;
            }
            let data = e.global::<u32>(BET_DATA_011D9054);
            let value = e.mem.u32(data + 4);
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, PADDED_VALUE_FORMAT, buffer, value],
            );
        }
        let which = e.global::<u32>(LABEL_TRAIT_011D9044);
        let tile = e.mem.u32(this.addr() + 0x3c);
        e.call(TILE_SET_STRING, &args![tile, which, buffer, 1u32]);
    });
}

/// `BlackJackMenu`'s vtable (`010708fc`), the `Menu` destructor body (`00a1c520`) and
/// `` `eh vector destructor iterator' `` (`00ec5fce`, stdcall: array, size, count,
/// destructor) with the card destructor `00914ae0`.
const BLACKJACK_MENU_VTABLE: u32 = 0x0107_08fc;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
const CARD_DESTRUCTOR_00914AE0: u32 = 0x0091_4ae0;
/// The player (`PlayerCharacter *` global), `00969b20` on it, the camera holder
/// (`011d90bc`, an `NiPointer`), `NiPointer` assignment (`0066b0d0`, `RET 4`) and
/// destruction (`0045cec0`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_00969B20: u32 = 0x0096_9b20;
const CAMERA_011D90BC: u32 = 0x011d_90bc;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The casino's entry by index (`00504e00`, `thiscall`, `RET 4`; its slot +0x14 gives a
/// model path), the model loader (`011c3b3c`) and its release by path (`0045a5e0`,
/// `RET 4`).
const CASINO_ENTRY_00504E00: u32 = 0x0050_4e00;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const MODEL_RELEASE_0045A5E0: u32 = 0x0045_a5e0;
/// The three terminal meshes released (`010709c0`, `01070988`, `01070950`) and the
/// `"_Shadow.NIF"` suffix (`01070944`).
const TABLE_MESH_ONE: u32 = 0x0107_09c0;
const TABLE_MESH_TWO: u32 = 0x0107_0988;
const TABLE_MESH_THREE: u32 = 0x0107_0950;
const SHADOW_SUFFIX: u32 = 0x0107_0944;
/// The shadow scene node (`00450b80`, cdecl: 1), `00b5d180` on it and `00572160` on the
/// table model.
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const SCENE_00B5D180: u32 = 0x00b5_d180;
const MODEL_00572160: u32 = 0x0057_2160;
/// `004702f0` (`thiscall`, `RET 4`): a hand list's deletion.
const LIST_DELETE_004702F0: u32 = 0x0047_02f0;
/// The copier (`00406d30`, cdecl, three words).
const COPY_TEXT: u32 = 0x0040_6d30;
/// The menu render: the renderer (`0043c4b0`) and its `007148c0` (`RET 8`), `0045c670`,
/// the culler (`008d80e0`), `00c4f270` (`RET 4`), `00709d50` (cdecl: the camera and a
/// `float`, 0.15 at `01030ff4`), the interface manager's texture (`007fd640`), the
/// culler's accumulator (`004a0fd0`, `RET 4`), `006629f0`,
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, cdecl, three words), `00b6c0d0` (cdecl,
/// three words) and `00c4f2d0`.
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
const RENDERER_007148C0: u32 = 0x0071_48c0;
const GETTER_0045C670: u32 = 0x0045_c670;
const CULLER_008D80E0: u32 = 0x008d_80e0;
const CULLER_00C4F270: u32 = 0x00c4_f270;
const CAMERA_00709D50: u32 = 0x0070_9d50;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
const TEXTURE_007FD640: u32 = 0x007f_d640;
const CULLER_SET_ACCUMULATOR: u32 = 0x004a_0fd0;
const GETTER_006629F0: u32 = 0x0066_29f0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
const CULLER_00C4F2D0: u32 = 0x00c4_f2d0;

/// The model path of the casino's entry `index` (`00504e00`, its slot +0x14).
fn casino_entry_path(e: &mut Engine, index: u32) -> u32 {
    let casino = e.global::<u32>(CASINO_011D9058);
    let entry = e.call(CASINO_ENTRY_00504E00, &args![casino, index]).u32();
    e.vcall(entry, 0x14, &args![]).u32()
}

/// Releases a model by path (`0045a5e0` on the model loader).
fn release_model(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(MODEL_RELEASE_0045A5E0, &args![loader, path]);
}

// Translated from 00731800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731800 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// The `BlackJackMenu` destructor body (unnamed in the engine map): the vtable; tells the
/// player (`00969b20`), clears the camera holder (`011d90bc`), releases the casino's
/// entry 8 model and the three table meshes, detaches the table model from the shadow
/// scene node (slot +0xe8), `00b5d180` on that node, `00572160` on the model and clears
/// its holder (`011d9098`); clears the geometry and animation pointers (+0xdb4..+0xdc0,
/// +0xdf4..+0xdfc, the three tables of 8 at +0xe0c, +0xe2c, +0xe4c); for the casino's
/// entries 0 to 5 releases each model and its shadow model (the path with everything
/// from its first `.` replaced by `"_Shadow.NIF"`, in a 0x104-byte buffer) and clears
/// the chip pointers (+0xdc4, +0xddc), which a second loop clears again; clears +0xe6c,
/// +0xe04, +0xe08, the three hands (+0xd90..+0xd98) and the statics `011d9054`,
/// `011d9058` and the singleton; deletes the hands (read after clearing, so never),
/// destroys the 0xd0 cards at +0x90 and runs the `Menu` destructor.
pub fn fn_00731800(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BLACKJACK_MENU_VTABLE);
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_00969B20, &args![player]);
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, 0u32]);
    let path = casino_entry_path(e, 8);
    release_model(e, path);
    for mesh in [TABLE_MESH_ONE, TABLE_MESH_TWO, TABLE_MESH_THREE] {
        release_model(e, mesh);
    }
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.vcall(scene, 0xe8, &args![model]);
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    e.call(SCENE_00B5D180, &args![scene]);
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.call(MODEL_00572160, &args![model]);
    e.call(NI_POINTER_ASSIGN, &args![TABLE_MODEL_011D9098, 0u32]);
    for offset in [0xdb4u32, 0xdb8, 0xdbc, 0xdc0, 0xdf4, 0xdf8, 0xdfc] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, 0u32]);
    for i in 0..8 {
        e.mem.set_u32(base + 0xe0c + 4 * i, 0);
        e.mem.set_u32(base + 0xe2c + 4 * i, 0);
        e.mem.set_u32(base + 0xe4c + 4 * i, 0);
    }
    // The frame below `EBP`: the path buffer at `EBP - 0x124`.
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        for i in 0..6 {
            let mut dot = 0;
            let path = casino_entry_path(e, i);
            e.call(COPY_TEXT, &args![buffer, 0x104u32, path]);
            let mut j = 0u32;
            while j < e.call(STRING_LENGTH_0044A670, &args![buffer]).u32() {
                if e.mem.u8(buffer + j) as i8 as i32 == 0x2e {
                    dot = buffer + j;
                    break;
                }
                j += 1;
            }
            if dot != 0 {
                e.call(COPY_TEXT, &args![dot, 0xfu32, SHADOW_SUFFIX]);
            }
            let path = casino_entry_path(e, i);
            release_model(e, path);
            release_model(e, buffer);
            e.mem.set_u32(base + 0xdc4 + 4 * i, 0);
            e.mem.set_u32(base + 0xddc + 4 * i, 0);
        }
    });
    for i in 0..6 {
        e.mem.set_u32(base + 0xdc4 + 4 * i, 0);
        e.mem.set_u32(base + 0xddc + 4 * i, 0);
    }
    for offset in [0xe6cu32, 0xe04, 0xe08, 0xd90, 0xd94, 0xd98] {
        e.mem.set_u32(base + offset, 0);
    }
    e.set_global(BET_DATA_011D9054, 0u32);
    e.set_global(CASINO_011D9058, 0u32);
    e.set_global(BLACKJACK_MENU, 0u32);
    for offset in [0xd94u32, 0xd98, 0xd90] {
        let hand = e.mem.u32(base + offset);
        if hand != 0 {
            e.call(LIST_DELETE_004702F0, &args![hand, 1u32]);
        }
    }
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 0x90, 0x10u32, 0xd0u32, CARD_DESTRUCTOR_00914AE0],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 00733500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00733500 - the C++ exception-unwinding frame (FS:[0] around the local NiPointer) is not translated.
/// Unnamed (no arguments; the table's render): `007148c0(renderer, 0, 4)`; the culler of
/// `0045c670()` (`008d80e0`, `00c4f270(1)`), the camera (`011d90bc`) given 0.15
/// (`00709d50`), a local `NiPointer` to the interface manager's texture (`007fd640`) made
/// the culler's accumulator; the scene is accumulated
/// (`BSShaderUtil::AccumulateScene(camera, shadow scene node, culler)`, the camera being
/// `006629f0` of the object when the holder is empty) and rendered (`00b6c0d0(camera,
/// texture, 0)`); the accumulator is cleared, the local released and destroyed and
/// `00c4f2d0` closes the culler.
pub fn fn_00733500(e: &mut Engine) {
    let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
    e.call(RENDERER_007148C0, &args![renderer, 0u32, 4u32]);
    let object = e.call(GETTER_0045C670, &args![]).u32();
    let culler = e.call(CULLER_008D80E0, &args![object]).u32();
    e.call(CULLER_00C4F270, &args![culler, 1u32]);
    let camera = e.call(FIRST_WORD, &args![CAMERA_011D90BC]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(CAMERA_00709D50, &args![camera, value]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let texture = e.call(TEXTURE_007FD640, &args![manager]).u32();
    // The local `NiPointer` at `EBP - 0x20`.
    e.with_stack(4, |e, pointer| {
        e.call(NI_POINTER_CONSTRUCT, &args![pointer, texture]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, held]);
        let view = if camera != 0 {
            camera
        } else {
            e.call(GETTER_006629F0, &args![object]).u32()
        };
        let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![view, scene, culler]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(RENDER_00B6C0D0, &args![view, held, 0u32]);
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
        e.call(CULLER_00C4F2D0, &args![culler]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// `Tile::Poke` (`00a01290`, `RET 8`) and the integer trait setter (`00700320`,
/// `RET 8`).
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_SET_INT: u32 = 0x0070_0320;
/// The bet tiles' label settings without a bet in play (`011d4c78`, `011d4234`,
/// `011d4e28`, `011d25ec`, `011d30fc`) and with one (`011d3da8`, `011d3354`,
/// `011d487c`, `011d37f8`, `011d33fc`).
const IDLE_BET_LABELS: [u32; 5] = [
    0x011d_4c78,
    0x011d_4234,
    0x011d_4e28,
    0x011d_25ec,
    0x011d_30fc,
];
const PLAY_BET_LABELS: [u32; 5] = [
    0x011d_3da8,
    0x011d_3354,
    0x011d_487c,
    0x011d_37f8,
    0x011d_33fc,
];
/// The random index (`00944460`, cdecl: the low and high bounds), `abs` (`00ec7d40`,
/// cdecl) and the card destructor (`00914ae0`).
const RANDOM_INDEX_00944460: u32 = 0x0094_4460;
const ABS_00EC7D40: u32 = 0x00ec_7d40;
/// A hand list's empty test (`008256d0`, a `bool`), its adds: `00726c60` and
/// `005ae3d0` (`RET 4`, the address of the item word) and `00905820` (`RET 4`).
const LIST_IS_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_ADD_00726C60: u32 = 0x0072_6c60;
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
const LIST_APPEND_00905820: u32 = 0x0090_5820;
/// `NiAVObject::GetProperty` (`00a59d30`, `RET 4`) and the property's type word
/// (`00441110`; the map's folded name is `PathingLocation::GetWorldspace`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE_00441110: u32 = 0x0044_1110;
/// The card mesh names: `"Hand1_0%i:%i"`, `"Hand1_0%i_Back:0"`, `"Hand2_0%i:%i"`,
/// `"Hand2_0%i_Back:0"`, `"Dealer_0%i:0"`, `"Dealer_0%i_Back:0"`.
const HAND_ONE_FRONT: u32 = 0x0107_0e34;
const HAND_ONE_BACK: u32 = 0x0107_0e20;
const HAND_TWO_FRONT: u32 = 0x0107_0dc4;
const HAND_TWO_BACK: u32 = 0x0107_0db0;
const DEALER_FRONT: u32 = 0x0107_0dfc;
const DEALER_BACK: u32 = 0x0107_0de8;

/// The card at `index` of the deck (+0x90, 0x10 bytes each).
fn card_at(this: Ptr, index: u32) -> u32 {
    this.addr()
        .wrapping_add(0x90)
        .wrapping_add(index.wrapping_shl(4))
}

// Translated from 00738cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::SetBetTiles` (Xbox PDB; no `this`, one `bool`): pokes the five bet
/// tiles (+0x44..+0x54 of the open menu, trait `0xfc4` with 1.0) and labels them: without
/// a bet in play with the idle labels and shows the bet tile (+0x40, trait `0xfa3` 1);
/// with one with the play labels, the text of tiles 4 to 8 at full alpha
/// (`SetTileTextAlpha` with 255), and hides the bet tile.
pub fn black_jack_menu_set_bet_tiles(e: &mut Engine, playing: u8) {
    let labels = if playing == 0 {
        IDLE_BET_LABELS
    } else {
        PLAY_BET_LABELS
    };
    for (i, setting) in labels.iter().enumerate() {
        let offset = 0x44 + 4 * i as u32;
        let tile = e.mem.u32(blackjack_menu(e) + offset);
        e.call(TILE_POKE, &args![tile, 0xfc4u32, x87_store32(1.0)]);
        let text = e.call(SETTING_STRING, &args![*setting]).u32();
        let tile = e.mem.u32(blackjack_menu(e) + offset);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        if playing != 0 {
            let alpha = f32::from_bits(x87_quiet_bits(e.global::<u32>(FULL_ALPHA)));
            let me = Ptr::new(blackjack_menu(e));
            black_jack_menu_set_tile_text_alpha(e, me, 4 + i as i32, alpha);
        }
    }
    let tile = e.mem.u32(blackjack_menu(e) + 0x40);
    let shown = if playing == 0 { 1u32 } else { 0 };
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, shown]);
}

/// Adds the address of the card at `index` to a hand through a local holding it: with
/// `first`, `00726c60` (an empty or missing hand) or `005ae3d0`; otherwise `00905820`.
fn add_card(e: &mut Engine, this: Ptr, hand: u32, index: u32, adder: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), card_at(this, index));
        e.call(adder, &args![hand, local]);
    });
}

// Translated from 007393f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; the deal): clears the two hand results (+0x2c, +0x30), deals a
/// card (`00739090(0, 0, 0)`) into the dealer's hand (+0xd90; `00726c60` when it is
/// missing or empty, else `005ae3d0`), one into the player's (+0xd94; the non-empty case
/// adds to the dealer's hand, as the code does), then one more to each with luck (the
/// dealer's with the dealer flag), appended (`00905820`). Returns the player's hand
/// value (`00738ff0`).
pub fn fn_007393f0(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, 0);
    for (hand, other) in [(0xd90u32, 0xd90u32), (0xd94, 0xd90)] {
        let index = fn_00739090(e, this, 0, 0, 0);
        let list = e.mem.u32(base + hand);
        let fresh = list == 0 || e.call(LIST_IS_EMPTY_008256D0, &args![list]).bool();
        if fresh {
            let list = e.mem.u32(base + hand);
            add_card(e, this, list, index, LIST_ADD_00726C60);
        } else {
            let list = e.mem.u32(base + other);
            add_card(e, this, list, index, LIST_ADD_005AE3D0);
        }
    }
    let dealer = e.mem.u32(base + 0xd90);
    let index = fn_00739090(e, this, dealer, 1, 1);
    let dealer = e.mem.u32(base + 0xd90);
    add_card(e, this, dealer, index, LIST_APPEND_00905820);
    let player = e.mem.u32(base + 0xd94);
    let index = fn_00739090(e, this, player, 1, 0);
    let player = e.mem.u32(base + 0xd94);
    add_card(e, this, player, index, LIST_APPEND_00905820);
    let player = e.mem.u32(base + 0xd94);
    fn_00738ff0(e, this, player)
}

// Translated from 00739090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00739090 - the C++ exception-unwinding frame (FS:[0] around the two local cards) is not translated.
/// Unnamed (`thiscall`, `RET 0xc`: a hand, a `bool` for luck, a `bool` for the dealer;
/// draws a card). Reshuffles (`007399d0`) below 5 cards left. Without a hand or luck, or
/// with a luck skill (+0x68) between 5 and 6 for the dealer's side (above 4 for the
/// dealer, below 6 for the player), picks a random index among the remaining cards
/// (`00944460(0, remaining)`, the count lowered first). Otherwise draws `|luck - 5|`
/// candidates and keeps the one that brings the hand closest below 22 (an ace counted 11
/// only up to 21; for hands up to 9 also closest to 11), and when that is not the first
/// candidate marks good luck (+0x78) or, for the dealer, bad luck (+0x79). The chosen
/// card is swapped with the last remaining one (`007324f0` through two local cards,
/// `00731710` and `00914ae0`), the count lowered, and the old count returned.
pub fn fn_00739090(e: &mut Engine, this: Ptr, hand: u32, luck: u8, dealer: u8) -> u32 {
    let base = this.addr();
    // The frame below `EBP`: the candidates at `EBP - 0x50` (five words), then the
    // lower candidate's index, the candidate count and the best distance, the two local
    // cards at `EBP - 0x30` and `EBP - 0x20`, and the chosen index at `EBP - 0x10`.
    e.with_stack(0x50, |e, frame| {
        let ebp = frame.addr() + 0x50;
        let local = |offset: u32| ebp - offset;
        let (picks, low_index, count, best) = (local(0x50), local(0x3c), local(0x38), local(0x34));
        let (swap_out, swap_in, chosen) = (local(0x30), local(0x20), local(0x10));
        fn_00731710(e, Ptr::new(swap_out));
        fn_00731710(e, Ptr::new(swap_in));
        e.mem.set_u32(chosen, 0);
        if e.mem.i32(base + 0xda0) < 5 {
            fn_007399d0(e, this);
        }
        let skill = |e: &Engine| e.mem.i32(base + 0x68);
        let lucky = hand != 0
            && luck != 0
            && ((dealer != 0 && skill(e) <= 4) || (dealer == 0 && skill(e) >= 6));
        if !lucky {
            let remaining = e.mem.u32(base + 0xda0);
            e.mem.set_u32(base + 0xda0, remaining.wrapping_sub(1));
            let index = e.call(RANDOM_INDEX_00944460, &args![0u32, remaining]).u32();
            e.mem.set_u32(chosen, index);
        } else {
            let current = fn_00738ff0(e, this, hand);
            let mut best_index = 0i32;
            e.mem.set_u32(low_index, 0);
            e.mem.set_i32(best, current);
            let mut low = current;
            let distance = e.call(ABS_00EC7D40, &args![skill(e).wrapping_sub(5)]).i32();
            e.mem.set_i32(count, distance);
            let mut i = 0i32;
            while i < e.mem.i32(count) {
                let remaining = e.mem.u32(base + 0xda0);
                let index = e.call(RANDOM_INDEX_00944460, &args![0u32, remaining]).u32();
                e.mem
                    .set_u32(picks.wrapping_add((i as u32).wrapping_mul(4)), index);
                i += 1;
            }
            let pick =
                |e: &Engine, i: i32| e.mem.u32(picks.wrapping_add((i as u32).wrapping_mul(4)));
            let mut j = 0i32;
            while j < e.mem.i32(count) {
                let rank = e.mem.i32(card_at(this, pick(e, j)));
                let mut points = fn_00738fb0(e, this, rank);
                if points == 0xb {
                    points = if points.wrapping_add(current) <= 0x15 {
                        0xb
                    } else {
                        1
                    };
                }
                if current > 9 {
                    let total = points.wrapping_add(current);
                    if total > e.mem.i32(best) && total < 0x16 {
                        e.mem.set_i32(best, total);
                        best_index = j;
                    }
                } else {
                    let total = points.wrapping_add(current);
                    if 0x15i32.wrapping_sub(total) < e.mem.i32(best) && total < 0x16 {
                        e.mem.set_i32(best, 0x15i32.wrapping_sub(total));
                        best_index = j;
                    }
                    if 0xbi32.wrapping_sub(total) < low && total < 0xc {
                        low = 0xbi32.wrapping_sub(total);
                        e.mem.set_i32(low_index, j);
                    }
                }
                let mark = |e: &mut Engine, picked: i32| {
                    if picked != 0 {
                        if dealer != 0 {
                            e.mem.set_u8(base + 0x79, 1);
                        } else {
                            e.mem.set_u8(base + 0x78, 1);
                        }
                    }
                };
                if e.mem.i32(best) < 4 {
                    let value = pick(e, best_index);
                    e.mem.set_u32(chosen, value);
                    mark(e, best_index);
                } else if low < e.mem.i32(best) && low < 2 {
                    let low_pick = e.mem.i32(low_index);
                    let value = pick(e, low_pick);
                    e.mem.set_u32(chosen, value);
                    mark(e, low_pick);
                } else {
                    let value = pick(e, best_index);
                    e.mem.set_u32(chosen, value);
                }
                j += 1;
            }
        }
        let picked = e.mem.u32(chosen);
        fn_007324f0(e, Ptr::new(swap_in), card_at(this, picked));
        let last = e.mem.u32(base + 0xda0);
        fn_007324f0(e, Ptr::new(swap_out), card_at(this, last));
        let last = e.mem.u32(base + 0xda0);
        fn_007324f0(e, Ptr::new(card_at(this, last)), swap_in);
        let picked = e.mem.u32(chosen);
        fn_007324f0(e, Ptr::new(card_at(this, picked)), swap_out);
        let drawn = e.mem.u32(base + 0xda0);
        e.mem.set_u32(base + 0xda0, drawn.wrapping_sub(1));
        e.call(CARD_DESTRUCTOR_00914AE0, &args![swap_in]);
        e.call(CARD_DESTRUCTOR_00914AE0, &args![swap_out]);
        drawn
    })
}

/// The texturing property of a named card mesh of the table (`004aae30` on the table
/// model's node `0043b4a0(hand)`; property 3 when its type word is 8 to 12, else none).
fn card_property(e: &mut Engine, hand: u32, name: u32) -> u32 {
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    let node = e.call(NODE_0043B4A0, &args![model, hand]).u32();
    let object = e.call(FIND_OBJECT_004AAE30, &args![node, name]).u32();
    let usable = e.call(GET_PROPERTY, &args![object, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        e.call(PROPERTY_TYPE_00441110, &args![property]).i32() >= 8 && {
            let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
            e.call(PROPERTY_TYPE_00441110, &args![property]).i32() <= 0xc
        }
    };
    if usable {
        e.call(GET_PROPERTY, &args![object, 3u32]).u32()
    } else {
        0
    }
}

// Translated from 007396e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007396e0 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// `BlackJackMenu::SwapCardTextures` (Xbox PDB, `RET 0xc`: the hand (0 the player's, 1
/// the split, 2 the dealer's), the card's place 1 to 7, the card). Names the card's front
/// and back meshes for the hand (`"Hand1_0%i:%i"` with the place and place - 1 and
/// `"Hand1_0%i_Back:0"`, the `"Hand2_..."` pair, or `"Dealer_0%i:0"` and its back, in
/// two 0x1e-byte buffers) and gives each mesh's texturing property (slot +0xf0 with 0, 0)
/// the card's front (+8) and back (+0xc) textures. Nothing for other places or hands, or
/// without a card.
pub fn black_jack_menu_swap_card_textures(
    e: &mut Engine,
    _this: Ptr,
    hand: i32,
    place: i32,
    card: u32,
) {
    let hand = hand.wrapping_add(1);
    if !(1..=7).contains(&place) || card == 0 {
        return;
    }
    // The frame below `EBP`: the front name at `EBP - 0x28`, the back name at
    // `EBP - 0x4c`.
    e.with_stack(0x48, |e, frame| {
        let back = frame.addr();
        let front = frame.addr() + 0x24;
        match hand {
            1 => {
                e.call(
                    FORMAT_TEXT,
                    &args![front, 0x1eu32, HAND_ONE_FRONT, place, place.wrapping_sub(1)],
                );
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, HAND_ONE_BACK, place]);
            }
            2 => {
                e.call(
                    FORMAT_TEXT,
                    &args![front, 0x1eu32, HAND_TWO_FRONT, place, place.wrapping_sub(1)],
                );
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, HAND_TWO_BACK, place]);
            }
            3 => {
                e.call(FORMAT_TEXT, &args![front, 0x1eu32, DEALER_FRONT, place]);
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, DEALER_BACK, place]);
            }
            _ => return,
        }
        for (name, texture) in [(front, 8u32), (back, 0xc)] {
            let property = card_property(e, hand as u32, name);
            if property != 0 {
                let image = e.call(FIRST_WORD, &args![card + texture]).u32();
                e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
            }
        }
    });
}

/// `"Hand1_0%i_Shadow..."`, `"Dealer_0%i_Shadow..."`, `"Hand2_0%i_Shadow..."`.
const HAND_ONE_SHADOW: u32 = 0x0107_0e0c;
const DEALER_SHADOW: u32 = 0x0107_0dd4;
const HAND_TWO_SHADOW: u32 = 0x0107_0d9c;

/// One hand's card meshes: the root object's offset in the menu, the front name format
/// (and whether it takes the place minus one too), the back and shadow formats, and the
/// offset of the hand's card count.
struct HandMeshes {
    root: u32,
    front: u32,
    front_takes_previous: bool,
    back: u32,
    shadow: u32,
    count: u32,
}

/// The player's (+0xdb8, count +0x84), the dealer's (+0xdc0, count +0x8c) and the split
/// (+0xdbc, count +0x88) hands, in the order the code walks them.
const HAND_MESHES: [HandMeshes; 3] = [
    HandMeshes {
        root: 0xdb8,
        front: HAND_ONE_FRONT,
        front_takes_previous: true,
        back: HAND_ONE_BACK,
        shadow: HAND_ONE_SHADOW,
        count: 0x84,
    },
    HandMeshes {
        root: 0xdc0,
        front: DEALER_FRONT,
        front_takes_previous: false,
        back: DEALER_BACK,
        shadow: DEALER_SHADOW,
        count: 0x8c,
    },
    HandMeshes {
        root: 0xdbc,
        front: HAND_TWO_FRONT,
        front_takes_previous: true,
        back: HAND_TWO_BACK,
        shadow: HAND_TWO_SHADOW,
        count: 0x88,
    },
];

/// For each hand and card place 1 to 7, names the front, back and shadow meshes (three
/// 0x1e-byte buffers: `EBP - 0x44`, `- 0x68`, `- 0x24`) and sets each one's app-culled
/// flag (`00450f90`) to what `culled` answers for the place (the first mesh is looked up
/// before it is asked); then updates the table model (`00a59c60` with a fresh update
/// data).
fn cull_card_meshes(e: &mut Engine, this: Ptr, culled: impl Fn(&Engine, &HandMeshes, i32) -> u32) {
    let base = this.addr();
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let front = ebp - 0x44;
        let back = ebp - 0x68;
        let shadow = ebp - 0x24;
        let data = ebp - 0x80;
        for hand in &HAND_MESHES {
            for place in 1..=7i32 {
                if hand.front_takes_previous {
                    e.call(
                        FORMAT_TEXT,
                        &args![front, 0x1eu32, hand.front, place, place - 1],
                    );
                } else {
                    e.call(FORMAT_TEXT, &args![front, 0x1eu32, hand.front, place]);
                }
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, hand.back, place]);
                e.call(FORMAT_TEXT, &args![shadow, 0x1eu32, hand.shadow, place]);
                let root = e.mem.u32(base + hand.root);
                let object = e.call(FIND_OBJECT_004AAE30, &args![root, front]).u32();
                let flag = culled(e, hand, place);
                e.call(SET_APP_CULLED_00450F90, &args![object, flag]);
                for name in [back, shadow] {
                    let root = e.mem.u32(base + hand.root);
                    let object = e.call(FIND_OBJECT_004AAE30, &args![root, name]).u32();
                    e.call(SET_APP_CULLED_00450F90, &args![object, flag]);
                }
            }
        }
        let data = e
            .call(
                UPDATE_DATA_INIT_0043D410,
                &args![data, x87_store32(0.0), 0u32, 0u32],
            )
            .u32();
        let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
        e.call(UPDATE_00A59C60, &args![model, data]);
    });
}

// Translated from 00737e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00737e50 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// Unnamed (`thiscall`; shows the dealt cards): for the player's, the dealer's and the
/// split hand, shows the card meshes (front, back, shadow) of the places below the hand's
/// card count (+0x84, +0x8c, +0x88) and hides the others; then updates the table model.
pub fn fn_00737e50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    cull_card_meshes(e, this, |e, hand, place| {
        u32::from(place >= e.mem.i32(base + hand.count))
    });
}

// Translated from 00738270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00738270 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// Unnamed (`thiscall`): shows every card mesh (front, back, shadow) of the three hands'
/// seven places, then updates the table model.
pub fn fn_00738270(e: &mut Engine, this: Ptr) {
    cull_card_meshes(e, this, |_, _, _| 0);
}

/// The casino's data (`00504e60`, `thiscall`) and its entries by index (`00504e20`,
/// `RET 4`); `ModelLoader::LoadFile` (`00447080`, `RET 0x18`); the texture manager
/// (`011dea10`) and `TES::CreateTextureImage` (`004568c0`, `RET 0x10`);
/// `SwapPlatformLanguageTexturePath` (`004b7240`, cdecl, three words); `004812f0`
/// (cdecl: a text and a set of characters; the text from the first of them).
const CASINO_DATA_00504E60: u32 = 0x0050_4e60;
const CASINO_DECK_00504E20: u32 = 0x0050_4e20;
const MODEL_LOAD_FILE: u32 = 0x0044_7080;
const TEXTURE_MANAGER_011DEA10: u32 = 0x011d_ea10;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const SWAP_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
const FIND_CHARACTERS_004812F0: u32 = 0x0048_12f0;
/// `"\"` (`01013444`), `"%s%s"` (`0101996c`), `"Textures\Terminals\..."` (`01070a18`),
/// `"%s%s_Back.dds"` (`01070a08`), `"%s%s_%c%c%c.dds"` (`010709f8`).
const BACKSLASH_TEXT: u32 = 0x0101_3444;
const TWO_STRINGS_FORMAT: u32 = 0x0101_996c;
const CARD_TEXTURE_FOLDER: u32 = 0x0107_0a18;
const CARD_BACK_FORMAT: u32 = 0x0107_0a08;
const CARD_FACE_FORMAT: u32 = 0x0107_09f8;
/// The defaults of the two casino values: 1.5 (`01016088`) and 0.25 (`0101622c`), and
/// the dimmed alpha 128 (`0101e704`).
const DEFAULT_WIN_RATIO: u32 = 0x0101_6088;
const DEFAULT_SHUFFLE: u32 = 0x0101_622c;
const DIM_ALPHA: u32 = 0x0101_e704;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `ModelLoader::LoadFile(path, 0, 1, 0, 0, 0)` on the model loader.
fn load_model(e: &mut Engine, path: u32) -> u32 {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(
        MODEL_LOAD_FILE,
        &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00731d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731d70 - the C++ exception-unwinding frame (FS:[0]: the local card and texture pointer) and the /GS cookie check (00ec408c) are not translated.
/// `BlackJackMenu::InitCasinoData` (Xbox PDB; `thiscall`, a `bool`): false without a
/// casino (`011d9058`). Copies the casino data's two floats (+4 into +0x60, +0 into
/// +0x64), deck count (+0x24 into +0xd9c) and flag (+0x34 into +0xdac); loads the table
/// model (entry 8, into +0xdb4) and the six chip models and their shadow models (the path
/// with everything from its first `.` replaced by `"_Shadow.NIF"`; +0xdc4, +0xddc). At
/// least one deck; zero floats become 1.5 and 0.25. For each deck: the deck entry
/// (`00504e20(deck + 7)`) names a texture path; past its first two `\` the folder (up to
/// the next `\`, kept with it) and the card name (up to `_`) are cut out (byte copies into
/// stack buffers); the folder is prefixed with `"Textures\Terminals\..."`; the back
/// texture (`"%s%s_Back.dds"`) and the 52 face textures (`"%s%s_%c%c%c.dds"` with the
/// suit letter `h`, `c`, `d`, `s` and the two-digit number 01 to 13) are made
/// (`SwapPlatformLanguageTexturePath`, `TES::CreateTextureImage`) and the cards (rank
/// `(i + 12) % 13`, suit `i / 13`, face and back) copied into the deck (+0x90). Then the
/// cards remaining (+0xda0) are 0x34 per deck, the three counts (+0x84..) 1, the byte at
/// +0x80 cleared; true.
pub fn black_jack_menu_init_casino_data(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if e.global::<u32>(CASINO_011D9058) == 0 {
        return false;
    }
    let data = |e: &mut Engine| {
        let casino = e.global::<u32>(CASINO_011D9058);
        e.call(CASINO_DATA_00504E60, &args![casino]).u32()
    };
    let d = data(e);
    e.mem.set_u32(base + 0x60, x87_quiet_bits(e.mem.u32(d + 4)));
    let d = data(e);
    e.mem.set_u32(base + 0x64, x87_quiet_bits(e.mem.u32(d)));
    let d = data(e);
    let decks = e.mem.u32(d + 0x24);
    e.mem.set_u32(base + 0xd9c, decks);
    let d = data(e);
    let flag = e.mem.u8(d + 0x34);
    e.mem.set_u8(base + 0xdac, flag);
    let path = casino_entry_path(e, 8);
    let table = load_model(e, path);
    e.mem.set_u32(base + 0xdb4, table);
    // The frame below `EBP`: `local(n)` is `EBP - n` (0x4d0 bytes).
    e.with_stack(0x4d0, |e, frame| {
        let ebp = frame.addr() + 0x4d0;
        let local = |offset: u32| ebp - offset;
        let model_path = local(0x49c);
        for i in 0..6u32 {
            let path = casino_entry_path(e, i);
            let chip = load_model(e, path);
            e.mem.set_u32(base + 0xdc4 + 4 * i, chip);
            let mut dot = 0;
            let path = casino_entry_path(e, i);
            e.call(COPY_TEXT, &args![model_path, 0x104u32, path]);
            let mut j = 0u32;
            while j < e.call(STRING_LENGTH_0044A670, &args![model_path]).u32() {
                if e.mem.u8(model_path + j) as i8 as i32 == 0x2e {
                    dot = model_path + j;
                    break;
                }
                j += 1;
            }
            if dot != 0 {
                e.call(COPY_TEXT, &args![dot, 0xfu32, SHADOW_SUFFIX]);
                let shadow = load_model(e, model_path);
                e.mem.set_u32(base + 0xddc + 4 * i, shadow);
            }
        }
        if e.mem.u32(base + 0xd9c) == 0 {
            e.mem.set_u32(base + 0xd9c, 1);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        // FCOMP against 0.0; TEST AH,0x44; JP: the default only for exactly 0.0.
        if x87_load32(e.mem.u32(base + 0x60)) == zero {
            let value = x87_quiet_bits(e.global::<u32>(DEFAULT_WIN_RATIO));
            e.mem.set_u32(base + 0x60, value);
        }
        if x87_load32(e.mem.u32(base + 0x64)) == zero {
            let value = x87_quiet_bits(e.global::<u32>(DEFAULT_SHUFFLE));
            e.mem.set_u32(base + 0x64, value);
        }
        let card = local(0x14c);
        fn_00731710(e, Ptr::new(card));
        let suits = local(0x388);
        e.mem.write(suits, b"hcds");
        let path = local(0x11c);
        let folder = local(0x13c);
        let name = local(0x274);
        let full = local(0x384);
        let swapped = local(0x254);
        let texture = local(0x4b0);
        let mut deck = 0i32;
        while deck < e.mem.i32(base + 0xd9c) {
            let casino = e.global::<u32>(CASINO_011D9058);
            let entry = e
                .call(CASINO_DECK_00504E20, &args![casino, (deck + 7) as u32])
                .u32();
            let text = e.call(NAME_OF_00408DA0, &args![entry]).u32();
            e.call(COPY_TEXT, &args![path, 0x104u32, text]);
            let mut p = e
                .call(FIND_CHARACTERS_004812F0, &args![path, BACKSLASH_TEXT])
                .u32()
                .wrapping_add(1);
            p = e
                .call(FIND_CHARACTERS_004812F0, &args![p, BACKSLASH_TEXT])
                .u32()
                .wrapping_add(1);
            let mut k = 0u32;
            while p != 0 && e.mem.u8(p) as i8 as i32 != 0x5c {
                let byte = e.mem.u8(p);
                e.mem.set_u8(folder.wrapping_add(k), byte);
                p = p.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            p = p.wrapping_add(1);
            e.mem.set_u8(folder.wrapping_add(k), 0x5c);
            k = k.wrapping_add(1);
            e.mem.set_u8(folder.wrapping_add(k), 0);
            let mut k = 0u32;
            while p != 0 && e.mem.u8(p) as i8 as i32 != 0x5f {
                let byte = e.mem.u8(p);
                e.mem.set_u8(name.wrapping_add(k), byte);
                p = p.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            e.mem.set_u8(name.wrapping_add(k), 0);
            e.call(
                FORMAT_TEXT,
                &args![
                    path,
                    0x104u32,
                    TWO_STRINGS_FORMAT,
                    CARD_TEXTURE_FOLDER,
                    folder
                ],
            );
            let mut tens = b'0';
            let mut units = b'1';
            let mut suit = 0u32;
            e.call(
                FORMAT_TEXT,
                &args![full, 0x104u32, CARD_BACK_FORMAT, path, name],
            );
            e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
            e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![full, swapped, 0x104u32]);
            let manager = e.global::<u32>(TEXTURE_MANAGER_011DEA10);
            e.call(
                CREATE_TEXTURE_IMAGE,
                &args![manager, swapped, texture, 0u32, 0u32],
            );
            for i in 0..0x34i32 {
                e.mem.set_i32(card, (i + 0xc) % 0xd);
                e.mem.set_i32(card + 4, i / 0xd);
                let letter = e.mem.u8(suits.wrapping_add(suit)) as i8 as i32;
                e.call(
                    FORMAT_TEXT,
                    &args![
                        full,
                        0x104u32,
                        CARD_FACE_FORMAT,
                        path,
                        name,
                        letter,
                        tens as i8 as i32,
                        units as i8 as i32
                    ],
                );
                e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![full, swapped, 0x104u32]);
                let manager = e.global::<u32>(TEXTURE_MANAGER_011DEA10);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![manager, swapped, card + 8, 0u32, 0u32],
                );
                e.call(NI_POINTER_COPY_006E5CC0, &args![card + 0xc, texture]);
                let slot = (deck.wrapping_mul(0x34).wrapping_add(i)) as u32;
                fn_007324f0(e, Ptr::new(card_at(this, slot)), card);
                units = units.wrapping_add(1);
                if units as i8 as i32 > 0x39 {
                    units = b'0';
                    tens = tens.wrapping_add(1);
                }
                if tens == b'1' && units == b'4' {
                    tens = b'0';
                    units = b'1';
                    suit = suit.wrapping_add(1);
                }
            }
            e.call(NI_POINTER_DESTRUCT, &args![texture]);
            deck += 1;
        }
        let decks = e.mem.u32(base + 0xd9c);
        e.mem.set_u32(base + 0xda0, decks.wrapping_mul(0x34));
        for i in 0..3 {
            e.mem.set_u32(base + 0x84 + 4 * i, 1);
        }
        e.mem.set_u8(base + 0x80, 0);
        e.call(CARD_DESTRUCTOR_00914AE0, &args![card]);
    });
    true
}

// Translated from 00738560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::UpdateTileAlpha` (Xbox PDB, `RET 4`: a tile index; nothing in state
/// 0xa). Dims (128) or lights (255) the text of the button tiles by the flags
/// (`00734d30`; jump table `00738c08`): tile 3 by 0x800 (split hand, flag 0x10) or 0x400;
/// tile 4 lit by 0x100 (split) or 0x40; tile 5 dimmed on the split hand, else lit by 4;
/// tile 6 dimmed by 0x400 (split), else lit only by 8 without 0x800; tile 7 dimmed by
/// 1; tile 8 lit; tile 9 untouched; tile 10 updates them all (on the split hand 3 and 4
/// by 0x800 and 0x100, 5 and 7 dimmed, 8 lit, 6 by 0x400; otherwise all of 3, 4, 5, 7
/// dimmed by 0x400 or 0x20, else each by its flag, then 8 dimmed by 0x20 unless 8 is set
/// without 0x800, and 6 lit only by 8 without 0x800).
pub fn black_jack_menu_update_tile_alpha(e: &mut Engine, this: Ptr, tile: i32) {
    if e.mem.u32(this.addr() + 0x28) == 0xa {
        return;
    }
    let dim = f32::from_bits(x87_quiet_bits(e.global::<u32>(DIM_ALPHA)));
    let full = f32::from_bits(x87_quiet_bits(e.global::<u32>(FULL_ALPHA)));
    let flag = |e: &mut Engine, mask: u32| fn_00734d30(e, this, mask);
    let set = |e: &mut Engine, index: i32, alpha: f32| {
        black_jack_menu_set_tile_text_alpha(e, this, index, alpha);
    };
    let pick = |lit: bool| if lit { full } else { dim };
    match tile {
        3 => {
            let alpha = if flag(e, 0x10) {
                pick(!flag(e, 0x800))
            } else {
                pick(!flag(e, 0x400))
            };
            set(e, 3, alpha);
        }
        4 => {
            let alpha = if flag(e, 0x10) {
                pick(flag(e, 0x100))
            } else {
                pick(flag(e, 0x40))
            };
            set(e, 4, alpha);
        }
        5 => {
            if flag(e, 0x10) {
                set(e, 5, dim);
            } else {
                let alpha = pick(flag(e, 4));
                set(e, 5, alpha);
            }
        }
        6 => {
            let alpha = if flag(e, 0x10) {
                pick(!flag(e, 0x400))
            } else {
                pick(!flag(e, 0x800) && flag(e, 8))
            };
            set(e, 6, alpha);
        }
        7 => {
            let alpha = pick(!flag(e, 1));
            set(e, 7, alpha);
        }
        8 => set(e, 8, full),
        0xa => {
            if flag(e, 0x10) {
                if flag(e, 0x800) {
                    set(e, 3, dim);
                    set(e, 4, dim);
                } else {
                    set(e, 3, full);
                    let alpha = pick(flag(e, 0x100));
                    set(e, 4, alpha);
                }
                set(e, 5, dim);
                set(e, 7, dim);
                set(e, 8, full);
                let alpha = pick(!flag(e, 0x400));
                set(e, 6, alpha);
            } else {
                if flag(e, 0x400) || flag(e, 0x20) {
                    for index in [3, 4, 5, 7] {
                        set(e, index, dim);
                    }
                } else {
                    set(e, 3, full);
                    let alpha = pick(flag(e, 0x40));
                    set(e, 4, alpha);
                    let alpha = pick(flag(e, 4));
                    set(e, 5, alpha);
                    let alpha = pick(!flag(e, 1));
                    set(e, 7, alpha);
                }
                let dimmed = flag(e, 0x20) && (!flag(e, 8) || flag(e, 0x800));
                set(e, 8, pick(!dimmed));
                let alpha = pick(!flag(e, 0x800) && flag(e, 8));
                set(e, 6, alpha);
            }
        }
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_1d70, black_jack_menu_init_casino_data(Ptr) -> bool),
        entry!(0x0073_8560, black_jack_menu_update_tile_alpha(Ptr, i32)),
        entry!(0x0073_7e50, fn_00737e50(Ptr)),
        entry!(0x0073_8270, fn_00738270(Ptr)),
        entry!(0x0073_8cc0, black_jack_menu_set_bet_tiles(u8)),
        entry!(0x0073_93f0, fn_007393f0(Ptr) -> i32),
        entry!(0x0073_9090, fn_00739090(Ptr, u32, u8, u8) -> u32),
        entry!(
            0x0073_96e0,
            black_jack_menu_swap_card_textures(Ptr, i32, i32, u32)
        ),
        entry!(0x0073_1800, fn_00731800(Ptr)),
        entry!(0x0073_3500, fn_00733500()),
        entry!(0x0073_3fd0, fn_00733fd0(Ptr, f32)),
        entry!(0x0073_1710, fn_00731710(Ptr) -> Ptr),
        entry!(0x0073_1790, black_jack_menu_get_class(Ptr) -> u32),
        entry!(
            0x0073_17a0,
            black_jack_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(
            0x0073_17d0,
            black_jack_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0073_1d30, fn_00731d30()),
        entry!(0x0073_24f0, fn_007324f0(Ptr, u32) -> Ptr),
        entry!(0x0073_3f50, fn_00733f50(Ptr) -> Ptr),
        entry!(0x0073_3f80, fn_00733f80() -> u32),
        entry!(0x0073_3f90, fn_00733f90(Ptr) -> bool),
        entry!(0x0073_4d30, fn_00734d30(Ptr, u32) -> bool),
        entry!(0x0073_4d50, fn_00734d50(Ptr, u32, u8)),
        entry!(0x0073_7e30, fn_00737e30(Ptr) -> u32),
        entry!(
            0x0073_8c30,
            black_jack_menu_set_tile_text_alpha(Ptr, i32, f32)
        ),
        entry!(0x0073_8fb0, fn_00738fb0(Ptr, i32) -> i32),
        entry!(0x0073_8ff0, fn_00738ff0(Ptr, u32) -> i32),
        entry!(0x0073_95a0, fn_007395a0(Ptr) -> bool),
        entry!(0x0073_9650, fn_00739650(Ptr, u8) -> bool),
        entry!(0x0073_9970, fn_00739970(Ptr, u8)),
        entry!(0x0073_99d0, fn_007399d0(Ptr)),
        entry!(0x0073_99f0, fn_007399f0(Ptr)),
        entry!(0x0073_9ad0, fn_00739ad0(Ptr)),
        entry!(0x0073_ab40, fn_0073ab40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.call(0x0073_3fd0, &args![this, 1.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xf8), 1.5);
        e.call(0x0073_3fd0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xf8), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        FREE,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_COPY_006E5CC0,
        INTERFACE_MANAGER,
        WORD_AT_0X28,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        CHILD_LIST_ITEM_0073AC90,
        CHILD_LIST_AT_006317A0,
        CHILD_LIST_FOLLOWING_007B52D0,
        SET_FLOATS_004BC1F0,
        FIND_OBJECT_004AAE30,
        SET_APP_CULLED_00450F90,
        UPDATE_DATA_INIT_0043D410,
        NODE_0043B4A0,
        UPDATE_00A59C60,
        SETTING_STRING,
        NAME_OF_00408DA0,
        FORMAT_TEXT,
        STRING_LENGTH_0044A670,
        MENU_DESTRUCT,
        VECTOR_DESTRUCTOR_ITERATOR,
        CARD_DESTRUCTOR_00914AE0,
        PLAYER_00969B20,
        NI_POINTER_ASSIGN,
        NI_POINTER_DESTRUCT,
        CASINO_ENTRY_00504E00,
        MODEL_RELEASE_0045A5E0,
        SHADOW_SCENE_NODE,
        SCENE_00B5D180,
        MODEL_00572160,
        LIST_DELETE_004702F0,
        COPY_TEXT,
        RENDERER_0043C4B0,
        RENDERER_007148C0,
        GETTER_0045C670,
        CULLER_008D80E0,
        CULLER_00C4F270,
        CAMERA_00709D50,
        TEXTURE_007FD640,
        CULLER_SET_ACCUMULATOR,
        GETTER_006629F0,
        ACCUMULATE_SCENE,
        RENDER_00B6C0D0,
        CULLER_00C4F2D0,
        TILE_POKE,
        TILE_SET_INT,
        RANDOM_INDEX_00944460,
        ABS_00EC7D40,
        LIST_IS_EMPTY_008256D0,
        LIST_ADD_00726C60,
        LIST_ADD_005AE3D0,
        LIST_APPEND_00905820,
        GET_PROPERTY,
        PROPERTY_TYPE_00441110,
        CASINO_DATA_00504E60,
        CASINO_DECK_00504E20,
        MODEL_LOAD_FILE,
        CREATE_TEXTURE_IMAGE,
        SWAP_LANGUAGE_TEXTURE_PATH,
        FIND_CHARACTERS_004812F0,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// An engine with the statics and constants mapped, the callees doubled (list nodes
    /// are `[item, next]`), and the test vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_9000,
            0x011d_e000,
            0x011c_3000,
            0x0102_3000,
            0x0102_c000,
            0x0103_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FULL_ALPHA, 255.0f32);
        e.set_global(MINUS_FIVE_0102CAF8, -5.0f32);
        e.set_global(CAMERA_VALUE_01030FF4, 0.15f32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    fn menu(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0xe80);
        e.mem.set_u32(this, VTABLE);
        e.set_global(BLACKJACK_MENU, this);
        Ptr::new(this)
    }

    /// A hand of cards of the given ranks: the node list and the cards.
    fn hand(e: &mut Engine, ranks: &[u32]) -> u32 {
        let mut next = 0;
        for &rank in ranks.iter().rev() {
            let card = e.mem.alloc(0x10);
            e.mem.set_u32(card, rank);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, card);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn cards_and_the_small_accessors() {
        let mut e = engine();
        let card = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0073_1710, &args![card]).u32(), card);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [0xe, 5]);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other, 3);
        e.mem.set_u32(other + 4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_24f0, &args![card, other]).u32(), card);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [3, 1]);
        assert_eq!(
            calls_to(&e, NI_POINTER_COPY_006E5CC0),
            vec![vec![card + 8, other + 8], vec![card + 0xc, other + 0xc]]
        );
        e.mem.set_u16(card + 8, 7);
        e.call(0x0073_3f50, &args![card]);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [0, 0]);
        assert_eq!(e.mem.u16(card + 8), 0);
        let this = menu(&mut e);
        assert_eq!(e.call(0x0073_1790, &args![this]).u32(), 0x439);
        assert_eq!(e.call(0x0073_3f80, &args![]).u32(), 0x439);
        e.mem.set_u32(this.addr() + 0x210, 0x99);
        assert_eq!(e.call(0x0073_7e30, &args![this]).u32(), 0x99);
        e.mem.set_u32(this.addr() + 0xd9c, 2);
        e.call(0x0073_99d0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 0x68);
    }

    #[test]
    fn tiles_attach_and_all_are_needed() {
        let mut e = engine();
        let this = menu(&mut e);
        for id in 0..9 {
            e.call(0x0073_17a0, &args![this, id, 0x5000u32 + id as u32]);
        }
        e.call(0x0073_17a0, &args![this, 0xai32, 0x6000u32]);
        e.call(0x0073_17a0, &args![this, -1i32, 0x6000u32]);
        assert!(!e.call(0x0073_3f90, &args![this]).bool());
        e.call(0x0073_17a0, &args![this, 9i32, 0x5009u32]);
        assert!(e.call(0x0073_3f90, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0x34 + 4 * 9), 0x5009);
    }

    #[test]
    fn flags_set_clear_and_test() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call(0x0073_4d50, &args![this, 0x14u32, 1u32]);
        assert!(e.call(0x0073_4d30, &args![this, 0x10u32]).bool());
        e.call(0x0073_4d50, &args![this, 0x10u32, 0u32]);
        assert!(!e.call(0x0073_4d30, &args![this, 0x10u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0), 4);
    }

    #[test]
    fn the_deleting_destructor_and_the_body() {
        let mut e = engine();
        let this = menu(&mut e);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, VTABLE);
        answer(&mut e, CASINO_ENTRY_00504E00, entry);
        e.register(slot(0x14), |_, _| 0x7700u32.into_ret());
        // The path copy writes "a.nif" into the buffer.
        e.register(COPY_TEXT, |e, a| {
            if a[1] == 0x104 {
                e.mem.write(a[0], b"a.nif\0");
            }
            Ret::default()
        });
        e.register(STRING_LENGTH_0044A670, |_, _| 5u32.into_ret());
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, SHADOW_SCENE_NODE, scene);
        e.mem.set_u32(this.addr() + 0xd94, 0x8800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_17d0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BLACKJACK_MENU_VTABLE);
        assert_eq!(e.global::<u32>(BLACKJACK_MENU), 0);
        // Entry 8's model, three meshes, six models and six shadow models.
        assert_eq!(calls_to(&e, MODEL_RELEASE_0045A5E0).len(), 16);
        let shadows: Vec<_> = calls_to(&e, COPY_TEXT)
            .into_iter()
            .filter(|c| c[1] == 0xf)
            .collect();
        assert_eq!(shadows.len(), 6);
        assert_eq!(shadows[0][2], SHADOW_SUFFIX);
        assert!(calls_to(&e, LIST_DELETE_004702F0).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0xd94), 0);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![
                this.addr() + 0x90,
                0x10,
                0xd0,
                CARD_DESTRUCTOR_00914AE0
            ]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn closing_sets_the_state_and_the_alpha() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0073_1d30, &args![]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        let this = menu(&mut e);
        answer(&mut e, WORD_AT_0X28, 0x5100);
        e.call_log = Some(vec![]);
        e.call(0x0073_1d30, &args![]);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0xd);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5100, 0xfa9, 255.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn the_table_renders_through_the_culler() {
        let mut e = engine();
        answer(&mut e, CULLER_008D80E0, 0x6100);
        answer(&mut e, TEXTURE_007FD640, 0x6200);
        answer(&mut e, GETTER_006629F0, 0x6300);
        answer(&mut e, SHADOW_SCENE_NODE, 0x6400);
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_3500, &args![]);
        assert_eq!(calls_to(&e, RENDERER_007148C0)[0][1..], [0, 4]);
        assert_eq!(calls_to(&e, CULLER_00C4F270), vec![vec![0x6100, 1]]);
        assert_eq!(
            calls_to(&e, CULLER_SET_ACCUMULATOR),
            vec![vec![0x6100, 0x6200], vec![0x6100, 0]]
        );
        assert_eq!(
            calls_to(&e, ACCUMULATE_SCENE),
            vec![vec![0x6300, 0x6400, 0x6100]]
        );
        assert_eq!(calls_to(&e, RENDER_00B6C0D0), vec![vec![0x6300, 0x6200, 0]]);
        assert_eq!(calls_to(&e, CULLER_00C4F2D0), vec![vec![0x6100]]);
    }

    #[test]
    fn text_alpha_walks_the_tiles_children() {
        let mut e = engine();
        let this = menu(&mut e);
        let tile = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + 0x34 + 8, tile);
        e.mem.set_u32(tile + 4, 0x7000);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 1);
        answer(&mut e, CHILD_LIST_ITEM_0073AC90, item);
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, 0x7100);
        answer(&mut e, CHILD_LIST_AT_006317A0, child);
        e.register(CHILD_LIST_FOLLOWING_007B52D0, |_, a| {
            u32::from(a[1] == 0x7000).wrapping_mul(0x7001).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_8c30, &args![this, 2i32, 0.5f32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x7100, 0xfa9, 0.5f32.to_bits(), 1],
                vec![0x7100, 0xfa9, 0.5f32.to_bits(), 1]
            ]
        );
        assert_eq!(calls_to(&e, CHILD_LIST_AT_006317A0)[1][1], 0x7001);
    }

    #[test]
    fn card_points_and_hand_values() {
        let mut e = engine();
        let this = menu(&mut e);
        let points: Vec<i32> = [0i32, 7, 9, 11, 12, 13]
            .iter()
            .map(|&r| e.call(0x0073_8fb0, &args![this, r]).i32())
            .collect();
        assert_eq!(points, vec![2, 9, 10, 10, 11, 15]);
        let two_aces_and_nine = hand(&mut e, &[12, 12, 7]);
        assert_eq!(
            e.call(0x0073_8ff0, &args![this, two_aces_and_nine]).i32(),
            21
        );
        let king_queen = hand(&mut e, &[11, 10]);
        assert_eq!(e.call(0x0073_8ff0, &args![this, king_queen]).i32(), 20);
    }

    #[test]
    fn split_and_double_need_the_chips() {
        let mut e = engine();
        let this = menu(&mut e);
        let pair = hand(&mut e, &[5, 5]);
        e.mem.set_u32(this.addr() + 0xd94, pair);
        e.mem.set_i32(this.addr() + 0x6c, 10);
        e.mem.set_i32(this.addr() + 0x74, 20);
        assert!(e.call(0x0073_95a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 4, 4);
        let three = hand(&mut e, &[5, 5, 2]);
        e.mem.set_u32(this.addr() + 0xd94, three);
        assert!(!e.call(0x0073_95a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 4, 0);
        e.mem.set_i32(this.addr() + 0x70, 4);
        // Split: 20 < 10 + 2 * 4 is false; not split: 20 < 4 + 2 * 10.
        assert!(e.call(0x0073_9650, &args![this, 1u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 0x200, 0x200);
        assert!(!e.call(0x0073_9650, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 0x40, 0);
    }

    #[test]
    fn hand_geometry_and_arrows() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xdbc, 0x6600);
        e.mem.set_u32(this.addr() + 0xdb4, 0x6700);
        e.call_log = Some(vec![]);
        e.call(0x0073_9970, &args![this, 0u32]);
        e.call(0x0073_9970, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, SET_FLOATS_004BC1F0),
            vec![
                vec![0x6600, 0, 0, (-5.0f32).to_bits()],
                vec![0x6600, 0, 0, 0]
            ]
        );
        e.register(FIND_OBJECT_004AAE30, |_, a| (a[1] & 0xfff).into_ret());
        e.mem.set_u32(this.addr() + 0xdb0, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0073_99f0, &args![this]);
        assert_eq!(
            calls_to(&e, SET_APP_CULLED_00450F90),
            vec![vec![0xe58, 1], vec![0xe44, 0]]
        );
        assert_eq!(calls_to(&e, NODE_0043B4A0)[0][1], 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_9ad0, &args![this]);
        assert_eq!(
            calls_to(&e, SET_APP_CULLED_00450F90),
            vec![vec![0xe58, 1], vec![0xe44, 1]]
        );
    }

    #[test]
    fn the_bet_label_is_padded() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0x3c, 0x5300);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data + 4, 25);
        e.set_global(BET_DATA_011D9054, data);
        e.set_global(LABEL_TRAIT_011D9044, 0x45u32);
        e.register(FORMAT_TEXT, |e, a| {
            if a[2] == LABEL_FORMAT {
                e.mem.write(a[0], b"Bet\0");
            }
            Ret::default()
        });
        answer(&mut e, STRING_LENGTH_0044A670, 20);
        e.call_log = Some(vec![]);
        e.call(0x0073_ab40, &args![this]);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats.len(), 3);
        assert_eq!(formats[0][5], 25);
        let buffer = formats[0][0];
        // Three steps from length 20: spaces at 19, 20, 21, the end at 22.
        assert_eq!(e.mem.u8(buffer + 21), 0x20);
        assert_eq!(e.mem.u8(buffer + 22), 0);
        assert_eq!(formats[2][2..], [PADDED_VALUE_FORMAT, buffer, 25]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5300, 0x45, buffer, 1]]
        );
        answer(&mut e, STRING_LENGTH_0044A670, 30);
        e.call_log = Some(vec![]);
        e.call(0x0073_ab40, &args![this]);
        assert_eq!(calls_to(&e, FORMAT_TEXT).len(), 1);
    }

    #[test]
    fn bet_tiles_take_their_labels() {
        let mut e = engine();
        let this = menu(&mut e);
        let mut tiles = vec![];
        for i in 0..6 {
            let tile = e.mem.alloc(8);
            tiles.push(tile);
            e.mem.set_u32(this.addr() + 0x40 + 4 * i, tile);
        }
        e.register(SETTING_STRING, |_, a| (a[0] & 0xffff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_8cc0, &args![0u32]);
        assert_eq!(calls_to(&e, TILE_POKE).len(), 5);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING)[0],
            vec![tiles[1], 0xfc4, 0x4c78, 1]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![tiles[0], 0xfa3, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_8cc0, &args![1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING)[4],
            vec![tiles[5], 0xfc4, 0x33fc, 1]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![tiles[0], 0xfa3, 0]]);
        assert_eq!(calls_to(&e, CHILD_LIST_ITEM_0073AC90).len(), 0);
    }

    #[test]
    fn drawing_a_card_swaps_it_to_the_end() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xda0, 10);
        e.register(RANDOM_INDEX_00944460, |_, a| (a[1] / 2).into_ret());
        for i in 0..11u32 {
            e.mem.set_u32(this.addr() + 0x90 + 0x10 * i, i);
        }
        e.register(NI_POINTER_COPY_006E5CC0, |_, _| Ret::default());
        // No luck: a random index below the count (lowered first).
        assert_eq!(e.call(0x0073_9090, &args![this, 0u32, 0u32, 0u32]).u32(), 9);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 8);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 9), 5);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 5), 9);
        // With good luck (skill 7) two candidates; the one that makes 21 wins.
        e.mem.set_i32(this.addr() + 0x68, 7);
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        let low = hand(&mut e, &[9]);
        e.register_double(RANDOM_INDEX_00944460, {
            let mut n = 0;
            move |_, _| {
                n += 1;
                (if n == 1 { 1u32 } else { 7 }).into_ret()
            }
        });
        e.mem.set_u32(this.addr() + 0x90 + 0x10, 3);
        e.mem.set_u32(this.addr() + 0x90 + 0x10 * 7, 9);
        assert_eq!(e.call(0x0073_9090, &args![this, low, 1u32, 0u32]).u32(), 8);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 8), 9);
        // Neither the best nor the lower candidate is close enough to mark luck.
        assert_eq!(e.mem.u8(this.addr() + 0x78), 0);
    }

    #[test]
    fn the_deal_gives_two_cards_each() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xda0, 20);
        let dealer = e.mem.alloc(8);
        let player = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0xd90, dealer);
        e.mem.set_u32(this.addr() + 0xd94, player);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_93f0, &args![this]).i32(), 0);
        assert_eq!(calls_to(&e, LIST_ADD_005AE3D0).len(), 2);
        assert!(calls_to(&e, LIST_ADD_005AE3D0)
            .iter()
            .all(|c| c[0] == dealer));
        let appended = calls_to(&e, LIST_APPEND_00905820);
        assert_eq!(appended.len(), 2);
        assert_eq!([appended[0][0], appended[1][0]], [dealer, player]);
        // The plain draws lower the count twice, the dealer's lucky one (no candidates)
        // once.
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 13);
        answer(&mut e, LIST_IS_EMPTY_008256D0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_93f0, &args![this]);
        assert_eq!(
            calls_to(&e, LIST_ADD_00726C60)
                .iter()
                .map(|c| c[0])
                .collect::<Vec<_>>(),
            vec![dealer, player]
        );
    }

    #[test]
    fn card_textures_go_to_the_named_meshes() {
        let mut e = engine();
        let this = menu(&mut e);
        let property = e.mem.alloc(4);
        e.mem.set_u32(property, VTABLE);
        answer(&mut e, GET_PROPERTY, property);
        answer(&mut e, PROPERTY_TYPE_00441110, 9);
        let card = e.mem.alloc(0x10);
        e.mem.set_u32(card + 8, 0x7300);
        e.mem.set_u32(card + 0xc, 0x7400);
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 2i32, 3i32, card]);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats[0][2..], [DEALER_FRONT, 3]);
        assert_eq!(formats[1][2..], [DEALER_BACK, 3]);
        assert_eq!(calls_to(&e, NODE_0043B4A0)[0][1], 3);
        assert_eq!(
            calls_to(&e, slot(0xf0)),
            vec![vec![property, 0, 0, 0x7300], vec![property, 0, 0, 0x7400]]
        );
        answer(&mut e, PROPERTY_TYPE_00441110, 13);
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 0i32, 3i32, card]);
        assert!(calls_to(&e, slot(0xf0)).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 0i32, 8i32, card]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn casino_data_builds_the_decks() {
        let mut e = engine();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(DEFAULT_WIN_RATIO, 1.5f32);
        e.set_global(DEFAULT_SHUFFLE, 0.25f32);
        let this = menu(&mut e);
        assert!(!e.call(0x0073_1d70, &args![this]).bool());
        let casino = e.mem.alloc(0x40);
        e.set_global(CASINO_011D9058, casino);
        let data = e.mem.alloc(0x40);
        e.mem.set_f32(data + 4, 2.0);
        e.mem.set_u32(data + 0x24, 0);
        e.mem.set_u8(data + 0x34, 1);
        answer(&mut e, CASINO_DATA_00504E60, data);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, VTABLE);
        answer(&mut e, CASINO_ENTRY_00504E00, entry);
        answer(&mut e, MODEL_LOAD_FILE, 0x6600);
        // The deck's path: "a\b\folder\card_x".
        e.register(COPY_TEXT, |e, a| {
            if a[1] == 0x104 {
                e.mem.write(a[0], b"a\\b\\folder\\card_x\0");
            }
            Ret::default()
        });
        e.register(FIND_CHARACTERS_004812F0, |e, a| {
            let mut p = a[0];
            while e.mem.u8(p) != b'\\' {
                p += 1;
            }
            p.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_1d70, &args![this]).bool());
        assert_eq!(e.mem.f32(this.addr() + 0x60), 2.0);
        assert_eq!(e.mem.f32(this.addr() + 0x64), 0.25);
        assert_eq!(e.mem.u32(this.addr() + 0xd9c), 1);
        assert_eq!(e.mem.u8(this.addr() + 0xdac), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xdb4), 0x6600);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 0x34);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats.len(), 2 + 0x34);
        let folder = formats[0][4];
        assert_eq!(e.mem.u8(folder), b'f');
        assert_eq!(e.mem.u8(folder + 6), b'\\');
        let name = formats[1][4];
        assert_eq!(e.mem.u8(name), b'c');
        assert_eq!(e.mem.u8(name + 4), 0);
        // The last face: spades 13.
        assert_eq!(
            formats[0x35][5..],
            [u32::from(b's'), u32::from(b'1'), u32::from(b'3')]
        );
        // Card 0 is the ace (rank 12) of suit 0.
        assert_eq!(e.mem.u32(this.addr() + 0x90), 12);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 0x33 + 4), 3);
        assert_eq!(calls_to(&e, CREATE_TEXTURE_IMAGE).len(), 0x35);
    }

    #[test]
    fn tile_alpha_follows_the_flags() {
        let mut e = engine();
        e.map(0x0101_e000, 0x1000);
        e.set_global(DIM_ALPHA, 128.0f32);
        let this = menu(&mut e);
        // Each tile's child list holds one child, 0x100 plus the tile index.
        for i in 0..10u32 {
            let tile = e.mem.alloc(8);
            e.mem.set_u32(tile + 4, 0x100 + i);
            e.mem.set_u32(this.addr() + 0x34 + 4 * i, tile);
        }
        for i in 0..10u32 {
            let tile = e.mem.u32(this.addr() + 0x34 + 4 * i);
            let list = e.mem.alloc(4);
            e.mem.set_u32(list, 0x100 + i);
            e.mem.set_u32(tile + 4, list);
        }
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 1);
        answer(&mut e, CHILD_LIST_ITEM_0073AC90, item);
        e.register(CHILD_LIST_AT_006317A0, |e, a| {
            let slot = e.mem.alloc(4);
            let child = e.mem.u32(a[1]);
            e.mem.set_u32(slot, child);
            slot.into_ret()
        });
        let alphas = |e: &Engine| -> Vec<(u32, f32)> {
            calls_to(e, TILE_SET_FLOAT)
                .iter()
                .map(|c| (c[0], f32::from_bits(c[2])))
                .collect()
        };
        e.mem.set_u32(this.addr() + 0xdb0, 0x400);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 3i32]);
        e.call(0x0073_8560, &args![this, 7i32]);
        e.call(0x0073_8560, &args![this, 9i32]);
        assert_eq!(alphas(&e), vec![(0x103, 128.0), (0x107, 255.0)]);
        e.mem.set_u32(this.addr() + 0xdb0, 0x10 | 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 0xai32]);
        assert_eq!(
            alphas(&e),
            vec![
                (0x103, 255.0),
                (0x104, 255.0),
                (0x105, 128.0),
                (0x107, 128.0),
                (0x108, 255.0),
                (0x106, 255.0)
            ]
        );
        e.mem.set_u32(this.addr() + 0x28, 0xa);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 3i32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }

    #[test]
    fn card_meshes_show_up_to_the_counts() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_i32(this.addr() + 0x84, 3);
        e.mem.set_i32(this.addr() + 0x8c, 0);
        e.mem.set_i32(this.addr() + 0x88, 8);
        e.call_log = Some(vec![]);
        e.call(0x0073_7e50, &args![this]);
        let culled: Vec<u32> = calls_to(&e, SET_APP_CULLED_00450F90)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(culled.len(), 63);
        assert_eq!(culled[..9], [0, 0, 0, 0, 0, 0, 1, 1, 1]);
        assert!(culled[21..42].iter().all(|&c| c == 1));
        assert!(culled[42..].iter().all(|&c| c == 0));
        assert_eq!(calls_to(&e, UPDATE_00A59C60).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_8270, &args![this]);
        assert!(calls_to(&e, SET_APP_CULLED_00450F90)
            .iter()
            .all(|c| c[1] == 0));
        assert_eq!(calls_to(&e, FORMAT_TEXT)[0][2..], [HAND_ONE_FRONT, 1, 0]);
    }
}
