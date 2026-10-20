//! `fallout/misc/saveload/bgssaveloadglobaldata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0084d030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0xc]`, the value in `ST0`.
/// `Script::Run` (`005ac1e0`) calls it on the timer object at `011f6394`
/// for the frame's elapsed seconds. The `FLD` quiets a signaling NaN.
pub fn fn_0084d030(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0xc) as f64
}

/// `SaveGameWarning` (`00845cc0`, engine map; `cdecl`, format and
/// arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// "SAVELOAD: Unknown global data type %i in SaveGlobalData()".
const UNKNOWN_TYPE_WARNING: u32 = 0x0107_f5a4;
/// The `TES` singleton pointer.
const TES: u32 = 0x011d_ea10;
/// The player singleton pointer (`PlayerCharacter*`, as the code passes it
/// to `TESObjectREFR::GetWorldSpace`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `ProcessLists` object (passed by address).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The `CombatManager` singleton pointer.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// The data-type handlers of [`bgs_save_load_global_data_save_global_data`].
const MISC_STATS_SAVE_004D5FA0: u32 = 0x004d_5fa0;
const TES_SAVE_GAME: u32 = 0x0045_9490;
const PROCESS_LISTS_SAVE_GAME: u32 = 0x0097_5840;
const COMBAT_MANAGER_SAVE_GAME: u32 = 0x0099_32d0;
const INTERFACE_SAVE_007066D0: u32 = 0x0070_66d0;
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_SAVE_GAME: u32 = 0x0063_eb70;
const ACTOR_CAUSE_SAVE_LIST: u32 = 0x0066_e6d0;
const FALLOUT_RADIO_SAVE_GAME: u32 = 0x0083_66e0;
const FALLOUT_AUDIO_SAVE_GAME: u32 = 0x0082_ec40;
const PROCESS_LISTS_SAVE_004534F0: u32 = 0x0045_34f0;
/// `00865e50` (`thiscall` on the save buffer: data, size, a word passed as
/// 0): writes raw bytes.
const BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (`00865df0`, engine map `_ov2`;
/// `thiscall`: form, a word passed as 0).
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (`00865f20`, engine map;
/// `thiscall`): the position to fill in later.
const BUFFER_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865ff0`, engine map
/// `_ov2`; `thiscall`: value, position).
const BUFFER_SAVE_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// `TES::GetWorldSpace` (`004fd3e0`, engine map; `thiscall`).
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `0059bb30` (`thiscall`): the word at +0x24 (engine map name
/// `D3DTexture_LockRect`, a folded body).
const WORD_24_0059BB30: u32 = 0x0059_bb30;
/// `0045cd60` (`thiscall`): the word at +0x28.
const WORD_28_0045CD60: u32 = 0x0045_cd60;
/// `TESObjectREFR::GetWorldSpace` (`00575d70`, engine map; `thiscall`).
const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `008d6f30` (`thiscall`, no Xbox PDB name): the player's other form
/// saved when it has no worldspace.
const PLAYER_FORM_008D6F30: u32 = 0x008d_6f30;
/// `00436aa0` (`thiscall`): the address of three words to save.
const POSITION_00436AA0: u32 = 0x0043_6aa0;
/// `0078d6b0` (`cdecl`, the buffer; engine map unit `loadingmenu.cpp`).
const LOADING_MENU_SAVE_0078D6B0: u32 = 0x0078_d6b0;
/// `00461190` (`thiscall`): the data handler's global list (first node).
const DATA_HANDLER_GLOBALS_00461190: u32 = 0x0046_1190;
/// `006815c0` (`thiscall`): returns `this`; here a node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the word at +4; here the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00549580` (`thiscall`, a `bool` in AL): globals for which it answers
/// true are skipped.
const GLOBAL_SKIP_00549580: u32 = 0x0054_9580;
/// `TESGlobal`'s value getter (`00526ac0`; result in ST0).
const GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `0043b5d0` (`thiscall` on `TES`): a list (first node) of entries whose
/// objects [`fn_0084cf30`] saves.
const TES_LIST_0043B5D0: u32 = 0x0043_b5d0;
/// `00559450` (`thiscall`): the object a smart-pointer member holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `00529990` (`cdecl`, no words): `FLD dword [011c96fc]`.
const GLOBAL_VALUE_00529990: u32 = 0x0052_9990;
/// `0047c860` (`thiscall`, result in ST0; engine map name
/// `TESActorBaseData::GetKarma`, a folded body).
const FLOAT_GETTER_0047C860: u32 = 0x0047_c860;
/// `00441110` (`thiscall`): a word saved as a form id.
const WORD_GETTER_00441110: u32 = 0x0044_1110;

// Translated from 0084bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveGlobalData` (Xbox PDB, matched by the engine
/// map; `cdecl (type, buffer)`): saves one global data type into the save
/// buffer and answers true; an unknown type is reported (`SaveGameWarning`)
/// and answers false. The types (the exe's jump table at `0084be10` for
/// 0..=11, and 1000): 0 `004d5fa0`, 1 [`bgs_save_load_global_data_save_location_data`],
/// 2 `TES::SaveGame`, 3 [`bgs_save_load_global_data_save_globals`], 4
/// `ProcessLists::SaveGame`, 5 `CombatManager::SaveGame`, 6 `007066d0`, 7
/// [`fn_0084cf30`], 8 `Sky::SaveGame` on `Sky::GetInstance()`, 9
/// `ActorCause::SaveActorCausesList`, 10 `FalloutRadio::SaveGame`, 11
/// `FalloutAudio::SaveGame`, 1000 `004534f0` on the process lists.
pub fn bgs_save_load_global_data_save_global_data(e: &mut Engine, kind: u32, buffer: u32) -> bool {
    match kind {
        0 => {
            e.call(MISC_STATS_SAVE_004D5FA0, &args![buffer]);
        }
        1 => bgs_save_load_global_data_save_location_data(e, buffer),
        2 => {
            let tes: u32 = e.global(TES);
            e.call(TES_SAVE_GAME, &args![tes, buffer]);
        }
        3 => bgs_save_load_global_data_save_globals(e, buffer),
        4 => {
            e.call(PROCESS_LISTS_SAVE_GAME, &args![PROCESS_LISTS, buffer]);
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_SAVE_GAME, &args![manager, buffer]);
        }
        6 => {
            e.call(INTERFACE_SAVE_007066D0, &args![buffer]);
        }
        7 => fn_0084cf30(e, buffer),
        8 => {
            let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
            e.call(SKY_SAVE_GAME, &args![sky, buffer]);
        }
        9 => {
            e.call(ACTOR_CAUSE_SAVE_LIST, &args![buffer]);
        }
        10 => {
            e.call(FALLOUT_RADIO_SAVE_GAME, &args![buffer]);
        }
        11 => {
            e.call(FALLOUT_AUDIO_SAVE_GAME, &args![buffer]);
        }
        1000 => {
            e.call(PROCESS_LISTS_SAVE_004534F0, &args![PROCESS_LISTS, buffer]);
        }
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_TYPE_WARNING, kind]);
            return false;
        }
    }
    true
}

// Translated from 0084c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveLocationData` (Xbox PDB, matched by the
/// engine map; `cdecl (buffer)`): saves the data handler's word at +0x208
/// ([`fn_0084c580`]), the current worldspace (`TES::GetWorldSpace`) as a
/// form id, the `TES` words at +0x24 and +0x28, the player's worldspace (or,
/// without one, `008d6f30`'s form) as a form id and the three words at
/// `00436aa0`'s address, then runs `0078d6b0(buffer)`.
pub fn bgs_save_load_global_data_save_location_data(e: &mut Engine, buffer: u32) {
    // The exe's locals from EBP-0x1C: +4, +8 and +0xC are single words,
    // +0x10 three words.
    e.with_stack(0x1c, |e, locals| {
        let base = locals.addr();
        let handler: u32 = e.global(DATA_HANDLER);
        let word = fn_0084c580(e, Ptr::new(handler));
        e.mem.set_u32(base + 8, word);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 8, 4u32, 0u32]);
        let tes: u32 = e.global(TES);
        let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, world, 0u32]);
        let tes: u32 = e.global(TES);
        let first = e.call(WORD_24_0059BB30, &args![tes]).u32();
        e.mem.set_u32(base + 0xc, first);
        let tes: u32 = e.global(TES);
        let second = e.call(WORD_28_0045CD60, &args![tes]).u32();
        e.mem.set_u32(base + 4, second);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 0xc, 4u32, 0u32]);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 4, 4u32, 0u32]);
        let player: u32 = e.global(PLAYER);
        let mut world = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
        if world == 0 {
            let player: u32 = e.global(PLAYER);
            world = e.call(PLAYER_FORM_008D6F30, &args![player]).u32();
        }
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, world, 0u32]);
        let player: u32 = e.global(PLAYER);
        let source = e.call(POSITION_00436AA0, &args![player]).u32();
        for i in 0..3 {
            let value = e.mem.u32(source + 4 * i);
            e.mem.set_u32(base + 0x10 + 4 * i, value);
        }
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 0x10, 0xcu32, 0u32]);
        e.call(LOADING_MENU_SAVE_0078D6B0, &args![buffer]);
    });
}

// Translated from 0084c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x208 (`thiscall`; no Xbox PDB name; called on the data
/// handler).
pub fn fn_0084c580(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x208)
}

// Translated from 0084cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveGlobals` (Xbox PDB, matched by the engine
/// map; `cdecl (buffer)`): a variable-sized count followed by, for each
/// global of the data handler's list that `00549580` does not skip, its form
/// id and its value as a `float`.
pub fn bgs_save_load_global_data_save_globals(e: &mut Engine, buffer: u32) {
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(DATA_HANDLER_GLOBALS_00461190, &args![handler]).u32();
    let mut count = 0u32;
    let start = e.call(BUFFER_START_VARIABLE_SIZED, &args![buffer]).u32();
    e.with_stack(4, |e, value| {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let global = e.mem.u32(slot);
            if global != 0 && !e.call(GLOBAL_SKIP_00549580, &args![global]).bool() {
                e.call(BUFFER_SAVE_FORM_ID, &args![buffer, global, 0u32]);
                let v = e.call(GLOBAL_GET_VALUE, &args![global]).f64();
                e.mem.set_u32(value.addr(), x87_store32(v));
                e.call(BUFFER_SAVE_DATA, &args![buffer, value, 4u32, 0u32]);
                count = count.wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(BUFFER_SAVE_VARIABLE_SIZED, &args![buffer, count, start]);
}

// Translated from 0084cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves global data type 7 (`cdecl (buffer)`; no Xbox PDB name): a
/// variable-sized count followed by, for each entry of `TES`'s `0043b5d0`
/// list whose object (`00559450` on the entry) exists, lacks bit 0 of +0x18
/// ([`fn_0084d050`]) and has a vtable +0x94 object, that object's `float`
/// at +0xC ([`fn_0084d030`]), the `float` at `011c96fc` less `0047c860` of
/// it, and `00441110` of it as a form id.
pub fn fn_0084cf30(e: &mut Engine, buffer: u32) {
    let mut count = 0u32;
    let start = e.call(BUFFER_START_VARIABLE_SIZED, &args![buffer]).u32();
    let tes: u32 = e.global(TES);
    let mut node = e.call(TES_LIST_0043B5D0, &args![tes]).u32();
    // The exe's locals at EBP-0x1C (+0) and EBP-0x18 (+4).
    e.with_stack(8, |e, locals| {
        let base = locals.addr();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let object = e.call(POINTER_GET_00559450, &args![slot]).u32();
            if object != 0 && !fn_0084d050(e, Ptr::new(object)) {
                let inner = e.vcall(object, 0x94, &args![]).u32();
                if inner != 0 {
                    let first = fn_0084d030(e, Ptr::new(inner));
                    e.mem.set_u32(base + 4, x87_store32(first));
                    e.call(BUFFER_SAVE_DATA, &args![buffer, base + 4, 4u32, 0u32]);
                    let now = e.call(GLOBAL_VALUE_00529990, &args![]).f64();
                    let then = e.call(FLOAT_GETTER_0047C860, &args![inner]).f64();
                    e.mem.set_u32(base, x87_store32(x87_sub(now, then)));
                    e.call(BUFFER_SAVE_DATA, &args![buffer, base, 4u32, 0u32]);
                    let form = e.call(WORD_GETTER_00441110, &args![inner]).u32();
                    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
                    count = count.wrapping_add(1);
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(BUFFER_SAVE_VARIABLE_SIZED, &args![buffer, count, start]);
}

// Translated from 0084d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the word at +0x18 is set (`thiscall`; no Xbox PDB
/// name).
pub fn fn_0084d050(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) & 1 != 0
}

// Translated from 0084d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash map's set (`thiscall (this; key, value byte)`; no Xbox PDB name;
/// the map's vtable gives +0x04 the bucket of a key, +0x08 whether two keys
/// are equal, +0x0C node set-up, +0x14 a new node; buckets at +8, count at
/// +0xC; nodes hold the next node, the key at +4 and the byte at +8): an
/// existing key's byte is replaced, else a new node is put at the head of
/// the key's bucket and the count grows.
pub fn fn_0084d310(e: &mut Engine, this: Ptr, key: u32, value: u8) {
    let map = this.addr();
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let buckets = e.mem.u32(map + 8);
    let mut node = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, 8, &args![key, node_key]).bool() {
            e.mem.set_u8(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(map, 0x14, &args![]).u32();
    e.vcall(map, 0xc, &args![node, key, u32::from(value)]);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084d030, fn_0084d030(Ptr) -> f64),
        entry!(
            0x0084_bcd0,
            bgs_save_load_global_data_save_global_data(u32, u32) -> bool
        ),
        entry!(
            0x0084_c490,
            bgs_save_load_global_data_save_location_data(u32)
        ),
        entry!(0x0084_c580, fn_0084c580(Ptr) -> u32),
        entry!(0x0084_cc90, bgs_save_load_global_data_save_globals(u32)),
        entry!(0x0084_cf30, fn_0084cf30(u32)),
        entry!(0x0084_d050, fn_0084d050(Ptr) -> bool),
        entry!(0x0084_d310, fn_0084d310(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0xc_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_f32(object + 8, 7.0);
        for value in [0.016_666_668f32, -1.0, 0.0] {
            e.mem.set_f32(object + 0xc, value);
            assert_eq!(e.call(0x0084d030, &[object]).f64(), value as f64);
        }
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Saved = Rc<RefCell<Vec<Vec<u32>>>>;

    const BUFFER: u32 = 0x0000_6000;

    fn word(value: u32) -> Ret {
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

    /// Doubles for the save buffer: raw data is recorded as words, form ids
    /// as `[0xF0F0, form]`, the variable-sized value as `[count, start]`.
    fn buffer_doubles(e: &mut Engine) -> Saved {
        let saved: Saved = Rc::new(RefCell::new(vec![]));
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_DATA, move |e, a| {
            assert_eq!((a[0], a[3]), (BUFFER, 0));
            let words = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            record.borrow_mut().push(words);
            Ret::default()
        });
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_FORM_ID, move |_, a| {
            assert_eq!((a[0], a[2]), (BUFFER, 0));
            record.borrow_mut().push(vec![0xf0f0, a[1]]);
            Ret::default()
        });
        e.register(BUFFER_START_VARIABLE_SIZED, |_, _| word(0x77));
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_VARIABLE_SIZED, move |_, a| {
            record.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        saved
    }

    /// A list of nodes (item, next) holding `items`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn singletons(e: &mut Engine) {
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_1000, 0x1000);
        e.set_global(TES, 0x0000_5100u32);
        e.set_global(PLAYER, 0x0000_5200u32);
        e.set_global(COMBAT_MANAGER, 0x0000_5300u32);
    }

    fn location_doubles(e: &mut Engine) {
        let handler = e.mem.alloc(0x210);
        e.mem.set_u32(handler + 0x208, 0x1208);
        e.set_global(DATA_HANDLER, handler);
        e.register(TES_GET_WORLD_SPACE, |_, a| word(a[0] + 1));
        e.register(WORD_24_0059BB30, |_, _| word(0x24));
        e.register(WORD_28_0045CD60, |_, _| word(0x28));
        e.register(REFR_GET_WORLD_SPACE, |_, _| word(0));
        e.register(PLAYER_FORM_008D6F30, |_, a| word(a[0] + 2));
        let position = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(position + 4 * i, 0x100 + i);
        }
        e.register_double(POSITION_00436AA0, move |_, _| word(position));
        e.register(LOADING_MENU_SAVE_0078D6B0, |_, _| Ret::default());
    }

    #[test]
    fn save_location_data_saves_each_value_in_order() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        location_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_c490, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![0x1208],
                vec![0xf0f0, 0x5101],
                vec![0x24],
                vec![0x28],
                vec![0xf0f0, 0x5202],
                vec![0x100, 0x101, 0x102],
            ]
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(LOADING_MENU_SAVE_0078D6B0, vec![BUFFER])
        );

        // With a player worldspace that one is saved.
        e.register(REFR_GET_WORLD_SPACE, |_, _| word(0x9999));
        saved.borrow_mut().clear();
        e.call(0x0084_c490, &args![BUFFER]);
        assert_eq!(saved.borrow()[4], vec![0xf0f0, 0x9999]);
    }

    #[test]
    fn fn_0084c580_reads_the_word_at_0x208() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this + 0x208, 0x55);
        assert_eq!(e.call(0x0084_c580, &args![this]).u32(), 0x55);
    }

    #[test]
    fn save_globals_saves_the_unskipped_globals_and_their_count() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        let globals = list(&mut e, &[0x4000, 0, 0x4100, 0x4200]);
        let handler = e.mem.alloc(0x10);
        e.set_global(DATA_HANDLER, handler);
        e.register_double(DATA_HANDLER_GLOBALS_00461190, move |_, a| {
            assert_eq!(a[0], handler);
            word(globals)
        });
        e.register(GLOBAL_SKIP_00549580, |_, a| word(u32::from(a[0] == 0x4100)));
        e.register(GLOBAL_GET_VALUE, |_, a| st0(f64::from(a[0] >> 8)));
        e.call(0x0084_cc90, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![0xf0f0, 0x4000],
                vec![64.0f32.to_bits()],
                vec![0xf0f0, 0x4200],
                vec![66.0f32.to_bits()],
                vec![2, 0x77],
            ]
        );
    }

    #[test]
    fn fn_0084cf30_saves_each_usable_entry() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        // Three entries: no object, an object with bit 0 of +0x18, a usable
        // object whose +0x94 object exists.
        let flagged = e.mem.alloc(0x20);
        e.mem.set_u32(flagged + 0x18, 1);
        let usable = e.mem.alloc(0x20);
        let inner = e.mem.alloc(0x10);
        e.mem.set_f32(inner + 0xc, 1.5);
        let mut slots = vec![0u32; 0x98 / 4];
        slots[0x94 / 4] = 0x0001_0010;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(usable, 0x0002_0000);
        e.register_double(0x0001_0010, move |_, _| word(inner));
        let entries = list(&mut e, &[0x10, 0x20, 0x30]);
        e.register_double(TES_LIST_0043B5D0, move |_, a| {
            assert_eq!(a[0], 0x5100);
            word(entries)
        });
        e.register_double(POINTER_GET_00559450, move |e, a| {
            word(match e.mem.u32(a[0]) {
                0x20 => flagged,
                0x30 => usable,
                _ => 0,
            })
        });
        e.register(GLOBAL_VALUE_00529990, |_, _| st0(10.0));
        e.register(FLOAT_GETTER_0047C860, |_, _| st0(2.5));
        e.register_double(WORD_GETTER_00441110, move |_, a| {
            assert_eq!(a[0], inner);
            word(0x3333)
        });
        e.call(0x0084_cf30, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![1.5f32.to_bits()],
                vec![7.5f32.to_bits()],
                vec![0xf0f0, 0x3333],
                vec![1, 0x77],
            ]
        );
    }

    #[test]
    fn fn_0084d050_tests_bit_0_of_0x18() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 2);
        assert!(!e.call(0x0084_d050, &args![this]).bool());
        e.mem.set_u32(this + 0x18, 3);
        assert!(e.call(0x0084_d050, &args![this]).bool());
    }

    #[test]
    fn save_global_data_dispatches_each_type() {
        let mut e = Engine::new();
        singletons(&mut e);
        buffer_doubles(&mut e);
        location_doubles(&mut e);
        e.register(DATA_HANDLER_GLOBALS_00461190, |_, _| word(0));
        e.register(TES_LIST_0043B5D0, |_, _| word(0));
        let handlers = [
            MISC_STATS_SAVE_004D5FA0,
            TES_SAVE_GAME,
            PROCESS_LISTS_SAVE_GAME,
            COMBAT_MANAGER_SAVE_GAME,
            INTERFACE_SAVE_007066D0,
            SKY_SAVE_GAME,
            ACTOR_CAUSE_SAVE_LIST,
            FALLOUT_RADIO_SAVE_GAME,
            FALLOUT_AUDIO_SAVE_GAME,
            PROCESS_LISTS_SAVE_004534F0,
            SAVE_GAME_WARNING,
        ];
        for handler in handlers {
            e.register(handler, |_, _| Ret::default());
        }
        e.register(SKY_GET_INSTANCE, |_, _| word(0x5400));
        let expected: [(u32, u32, Vec<u32>); 13] = [
            (0, MISC_STATS_SAVE_004D5FA0, vec![BUFFER]),
            (1, LOADING_MENU_SAVE_0078D6B0, vec![BUFFER]),
            (2, TES_SAVE_GAME, vec![0x5100, BUFFER]),
            (3, BUFFER_SAVE_VARIABLE_SIZED, vec![BUFFER, 0, 0x77]),
            (4, PROCESS_LISTS_SAVE_GAME, vec![PROCESS_LISTS, BUFFER]),
            (5, COMBAT_MANAGER_SAVE_GAME, vec![0x5300, BUFFER]),
            (6, INTERFACE_SAVE_007066D0, vec![BUFFER]),
            (7, BUFFER_SAVE_VARIABLE_SIZED, vec![BUFFER, 0, 0x77]),
            (8, SKY_SAVE_GAME, vec![0x5400, BUFFER]),
            (9, ACTOR_CAUSE_SAVE_LIST, vec![BUFFER]),
            (10, FALLOUT_RADIO_SAVE_GAME, vec![BUFFER]),
            (11, FALLOUT_AUDIO_SAVE_GAME, vec![BUFFER]),
            (
                1000,
                PROCESS_LISTS_SAVE_004534F0,
                vec![PROCESS_LISTS, BUFFER],
            ),
        ];
        for (kind, handler, words) in expected {
            e.call_log = Some(vec![]);
            assert!(e.call(0x0084_bcd0, &args![kind, BUFFER]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(handler, words), "type {kind}");
        }
        for kind in [12u32, 999, 1001] {
            e.call_log = Some(vec![]);
            assert!(!e.call(0x0084_bcd0, &args![kind, BUFFER]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log.last().unwrap(),
                &(SAVE_GAME_WARNING, vec![UNKNOWN_TYPE_WARNING, kind])
            );
        }
    }

    #[test]
    fn fn_0084d310_replaces_or_inserts() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, buckets);
        e.put_vtable(
            0x0002_0000,
            &[0, 0x0001_0004, 0x0001_0008, 0x0001_000c, 0, 0x0001_0014],
        );
        e.mem.set_u32(map, 0x0002_0000);
        e.register(0x0001_0004, |_, a| word(a[1] % 4));
        e.register(0x0001_0008, |_, a| word(u32::from(a[1] == a[2])));
        e.register(0x0001_0014, |e, _| word(e.mem.alloc(0xc)));
        e.register(0x0001_000c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u8(a[1] + 8, a[3] as u8);
            Ret::default()
        });
        e.call(0x0084_d310, &args![map, 5u32, 7u32]);
        e.call(0x0084_d310, &args![map, 9u32, 8u32]);
        let head = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(head + 4), 9, "the newest node heads the bucket");
        let second = e.mem.u32(head);
        assert_eq!((e.mem.u32(second + 4), e.mem.u8(second + 8)), (5, 7));
        assert_eq!(e.mem.u32(map + 0xc), 2);
        e.call(0x0084_d310, &args![map, 5u32, 0x109u32]);
        assert_eq!(e.mem.u8(second + 8), 9);
        assert_eq!(e.mem.u32(map + 0xc), 2);
    }
}
