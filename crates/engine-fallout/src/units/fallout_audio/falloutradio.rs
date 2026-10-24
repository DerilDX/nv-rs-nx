//! `fallout/audio/falloutradio.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_008321c0` calls on the member (`00836030`).
const CALLEE_00836030: u32 = 0x0083_6030;

// Translated from 008321c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00836030` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_008321c0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00836030, &args![this.byte_add(0x4)]);
}

// Translated from 008372c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0107cf18` (RTTI `.?AVRadioConvTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_008372c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_cf18);
    e.call(0x00c3_cea0, &args![this]);
}

// ---------------------------------------------------------------------------
// The radio's statics (`FalloutRadio`, Xbox PDB, has only statics). Which PDB
// static each address is is not confirmed; they are named by their use.

/// Set by `Shutdown`; every update stops while it is set.
const SHUTTING_DOWN: u32 = 0x011d_d436;
/// The station list (`BSSimpleList` of `FORadioStation *`, first node
/// embedded): its entries are deleted with the station's deleting destructor.
const STATION_LIST: u32 = 0x011d_d554;
/// The receiver list (`BSSimpleList` of `FORadioReceiver *`).
const RECEIVER_LIST: u32 = 0x011d_d58c;
/// The list of stations already reported (references).
const DISCOVERED_LIST: u32 = 0x011d_d59c;
/// The time of the last periodic update.
const LAST_UPDATE: u32 = 0x011d_d424;
/// The setting with the periodic update interval.
const UPDATE_INTERVAL_SETTING: u32 = 0x011d_d5cc;
/// The timer object `00825c00` reads the time from.
const TIMER: u32 = 0x011f_6394;
/// A byte the update sets once the player's check (`00425fd0`) has passed.
const PLAYER_CHECK_SEEN: u32 = 0x011d_d435;
/// The queued and the current radio state flags.
const QUEUED_STATE: u32 = 0x011d_d438;
const RADIO_STATE: u32 = 0x011d_d43c;
/// The sound form the update looks up once (data handler `011c3f2c`).
const ATTENUATION_SOUND: u32 = 0x011d_d440;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The Pip-Boy's current and lost stations.
const CURRENT_PIPBOY_STATION: u32 = 0x011d_d42c;
const LOST_PIPBOY_STATION: u32 = 0x011d_d430;
/// The other saved statics: a word, a byte and a string.
const SAVED_WORD: u32 = 0x011d_d428;
const SAVED_BYTE: u32 = 0x011d_d434;
const SAVED_STRING: u32 = 0x011d_d448;
/// A byte that suppresses the new-station message once.
const SKIP_DISCOVERY_MESSAGE: u32 = 0x011a_179c;
/// The setting `UpdateStations` scales signal strength with.
const SIGNAL_SETTING: u32 = 0x011d_d580;
/// The player (`PlayerCharacter *` global) and the task queue.
const PLAYER: u32 = 0x011d_ea3c;
const TASK_QUEUE: u32 = 0x0120_2d98;

/// List helpers: the address of a node's item, the next node, the
/// emptiness test, the add-at-head (address of the item), the removal (by
/// the item's address), the membership test, the clear and the destructor
/// bodies of a list, and the constructor of one.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_EMPTY: u32 = 0x0082_56d0;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// The float list's clear and destructor bodies (`008373d0` builds one).
const FLOAT_LIST_CLEAR: u32 = 0x0083_7410;
const FLOAT_LIST_DESTRUCTOR: u32 = 0x0083_73f0;
/// `operator new` and `operator delete` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The deleting destructor of a `Conversation` (`this`, flags).
const CONVERSATION_DELETE: u32 = 0x0079_8450;
/// `Conversation::Conversation_ov2` (`this`, two references, topic).
const CONVERSATION_CONSTRUCT: u32 = 0x0083_b850;
/// `RadioConvTask`'s constructor (`this`, station).
const RADIO_TASK_CONSTRUCT: u32 = 0x0083_7250;
/// `TESTopic::GetTopic` (cdecl: kind, word).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `BSSoundHandle` constructor, `Stop` and `Release`.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// The form of a reference (`this` only) and a form's type byte.
const FORM_OF: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
/// The talking activator's radio station form (`this` = base form).
const RADIO_STATION_FORM: u32 = 0x004f_d3c0;
/// `BGSTalkingActivator::GetRadioStationsInRangeOfRef` (cdecl: reference,
/// reference list, float list).
const STATIONS_IN_RANGE: u32 = 0x004f_f1a0;
/// A test on a reference (`this` only).
const REFERENCE_TEST_00440DA0: u32 = 0x0044_0da0;
/// A word of a station reference (`this` only).
const REFERENCE_VALUE_0056B210: u32 = 0x0056_b210;
/// `Script::IsPendingEnabledReference` (cdecl, one word).
const PENDING_ENABLED: u32 = 0x005a_a680;
/// `BSAudio::QInstance` and the audio time (`this` = the audio).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_TIME: u32 = 0x0063_d040;
/// A random number in a range (cdecl: low, high).
const RANDOM_RANGE: u32 = 0x0094_4460;
/// The time of the timer object (`this` = timer).
const TIMER_TIME: u32 = 0x0082_5c00;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `Interface::GetLoadingMenuVisible` (no arguments).
const LOADING_MENU_VISIBLE: u32 = 0x0070_5e80;
/// The player's object and its test of the update (`008d6f30`, `00425fd0`).
const PLAYER_OBJECT: u32 = 0x008d_6f30;
const PLAYER_OBJECT_TEST: u32 = 0x0042_5fd0;
/// `TESDataHandler::GetSound_ov2` (`this`, editor id).
const GET_SOUND: u32 = 0x0046_16c0;
/// The save buffer's `SaveFormID_ov2`, plain data, `SaveString`,
/// `StartVariableSizedValue` and `SaveVariableSizedValue_ov2`.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_STRING: u32 = 0x0086_5e70;
const SAVE_START_SIZED: u32 = 0x0086_5f20;
const SAVE_SIZED: u32 = 0x0086_5ff0;
/// `Conversation::SaveGame` (`this`, buffer).
const CONVERSATION_SAVE: u32 = 0x0083_c270;
/// A station state call `00832490` (`this` = state, flag).
const STATE_CALL_00832490: u32 = 0x0083_2490;
/// The station state's power setter (`this` = state, power byte).
const STATE_SET_POWER: u32 = 0x0045_8b30;
/// `BSStringT` constructor, destructor, `Format` and C string.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_C_STR: u32 = 0x0055_9450;
/// A setting's string (`ECX` = the setting).
const SETTING_STRING: u32 = 0x004c_69f0;
/// The display name of a reference (`this` only).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `BSAudio::GetSoundHandleByName` and `BSSoundHandle::Play`.
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// The message box shower (cdecl, six words).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `FalloutRadio::PipboyRadioEnable` (cdecl, one word) and the Pip-Boy
/// station setter `00832240` (cdecl: reference, word).
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;
const SET_PIPBOY_STATION: u32 = 0x0083_2240;
/// Three float values of a station reference (`this` only; ST0).
const STATION_VALUE_0056B190: u32 = 0x0056_b190;
const STATION_VALUE_0056B1D0: u32 = 0x0056_b1d0;

/// The sound name and flags of the new-station message, its texts and
/// the message time.
const DISCOVERY_SOUND: u32 = 0x0107_ce94;
const DISCOVERY_SETTING: u32 = 0x011d_4348;
const DISCOVERY_TEXT: u32 = 0x0107_ce44;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// Constants of `UpdateStations`.
const LARGEST_FLOAT: u32 = 0x0101_6970;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// The editor id of the sound `Update` looks up.
const ATTENUATION_SOUND_ID: u32 = 0x0107_ce14;

layout! {
    /// `FalloutRadio::FORadioReceiver` (Xbox PDB), 0x20 bytes.
    pub struct FORadioReceiver: 0x20 {
        /// `pRadio` (Xbox PDB): `TESObjectREFR *`.
        0x00 pRadio: u32,
        /// `bShouldBePlaying` (Xbox PDB).
        0x1C bShouldBePlaying: u8,
        /// `bJustStarted` (Xbox PDB).
        0x1D bJustStarted: u8,
        /// `bInRange` (Xbox PDB).
        0x1E bInRange: u8,
        /// `bIsActor` (Xbox PDB).
        0x1F bIsActor: u8,
    }
}

layout! {
    /// `FalloutRadio::StationState` (Xbox PDB), 0x20 bytes.
    pub struct StationState: 0x20 {
        /// `pConvo` (Xbox PDB): `Conversation *`.
        0x00 pConvo: u32,
        /// `pNextConvo` (Xbox PDB).
        0x04 pNextConvo: u32,
        /// `iCurrentItemStartTime` (Xbox PDB).
        0x08 iCurrentItemStartTime: u32,
        /// `iCurrentItemDuration` (Xbox PDB).
        0x0C iCurrentItemDuration: i32,
        /// `cStationPower` (Xbox PDB).
        0x10 cStationPower: u8,
        /// `cTargetPower` (Xbox PDB).
        0x11 cTargetPower: u8,
        /// `iStationFlags` (Xbox PDB).
        0x14 iStationFlags: u32,
    }
}

/// `FalloutRadio::FORadioStation` (Xbox PDB), 0x24 bytes: the station
/// reference, then its `StationState` at +4 (whose `StationUsers` list is
/// at +0x1c of the station).
const STATION_STATE: u32 = 4;
const STATION_USERS: u32 = 0x1c;

fn item_of(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The radio reference of the receiver at a node.
fn radio_of(e: &mut Engine, node: u32) -> u32 {
    let receiver = item_of(e, node);
    e.mem.u32(receiver)
}

/// A flag byte of the receiver at a node.
fn receiver_byte(e: &mut Engine, node: u32, offset: u32) -> u8 {
    let receiver = item_of(e, node);
    e.mem.u8(receiver + offset)
}

/// Whether the reference is an actor (its slot `+0x100`).
fn is_actor(e: &mut Engine, reference: u32) -> bool {
    e.vcall(reference, 0x100, &args![]).bool()
}

fn next_of(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

fn is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(NODE_EMPTY, &args![node]).bool()
}

fn shutting_down(e: &Engine) -> bool {
    e.global::<u8>(SHUTTING_DOWN) != 0
}

/// Stops and releases a sound handle.
fn stop_and_release(e: &mut Engine, handle: u32) {
    e.call(SOUND_HANDLE_STOP, &args![handle]);
    e.call(SOUND_HANDLE_RELEASE, &args![handle]);
}

/// Deletes a conversation through its deleting destructor (when there is
/// one).
fn delete_conversation(e: &mut Engine, conversation: u32) {
    if conversation != 0 {
        e.call(CONVERSATION_DELETE, &args![conversation, 1u32]);
    }
}

/// The audio time (`BSAudio::QInstance()` then `0063d040`).
fn audio_time(e: &mut Engine) -> u32 {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_TIME, &args![audio]).u32()
}

/// Sets or clears `mask` in `iStationFlags`.
fn set_station_flag(e: &mut Engine, this: Ptr<StationState>, mask: u32, on: u8) {
    let flags = e.get(this, StationState::iStationFlags);
    let flags = if on != 0 { flags | mask } else { flags & !mask };
    e.set(this, StationState::iStationFlags, flags);
}

fn station_flag(e: &Engine, this: Ptr<StationState>, mask: u32) -> bool {
    e.get(this, StationState::iStationFlags) & mask != 0
}

// Translated from 00832010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::Shutdown` (Xbox PDB name from the call graph, cdecl):
/// sets the shutting-down flag, clears every receiver
/// (`ClearInactiveReceivers(1)`), deletes each station's next conversation
/// and the station, clears the station list, deletes the receivers and
/// clears the receiver and discovered lists.
pub fn fallout_radio_shutdown(e: &mut Engine) {
    e.set_global(SHUTTING_DOWN, 1u8);
    fallout_radio_clear_inactive_receivers(e, 1);
    let mut node = STATION_LIST;
    while node != 0 && !is_empty(e, node) {
        let station = item_of(e, node);
        if e.mem.u32(station.wrapping_add(4)) != 0 {
            let next_conversation = item_of(e, node);
            let next_conversation = e.mem.u32(next_conversation.wrapping_add(4));
            delete_conversation(e, next_conversation);
            let station = item_of(e, node);
            e.mem.set_u32(station.wrapping_add(4), 0);
        }
        let station = item_of(e, node);
        if station != 0 {
            fn_00832160(e, Ptr::new(station), 1);
        }
        node = next_of(e, node);
    }
    e.call(LIST_CLEAR, &args![STATION_LIST]);
    let mut node = RECEIVER_LIST;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if receiver != 0 {
            fn_00832190(e, Ptr::new(receiver), 1);
        }
        node = next_of(e, node);
    }
    e.call(LIST_CLEAR, &args![RECEIVER_LIST]);
    e.call(LIST_CLEAR, &args![DISCOVERED_LIST]);
}

// Translated from 00832160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The station's deleting destructor: `008321c0` (the station state's
/// destructor at +4) and, when bit 0 of `flags` is set, `operator delete`.
/// Returns `this`.
pub fn fn_00832160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008321c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00832190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The receiver's deleting destructor: `008321e0` and, when bit 0 of
/// `flags` is set, `operator delete`. Returns `this`.
pub fn fn_00832190(e: &mut Engine, this: Ptr<FORadioReceiver>, flags: u32) -> Ptr<FORadioReceiver> {
    fn_008321e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008321e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioReceiver` destructor: destroys the static handle (+0x10)
/// and the sound handle (+4).
// Translation incomplete: 008321e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_008321e0(e: &mut Engine, this: Ptr<FORadioReceiver>) {
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![this.byte_add(0x10)]);
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![this.byte_add(4)]);
}

// Translated from 00832420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x100` of `iStationFlags` is set.
pub fn fn_00832420(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x100)
}

// Translated from 00832440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 1 of `iStationFlags`.
pub fn fn_00832440(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 1, on);
}

// Translated from 008325b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns a radio reference on or off (cdecl). Nothing while shutting down,
/// when its receiver already is in the wanted state, or when the reference
/// is not a talking activator (`0x15`) with a radio station. On: finds or
/// creates the station (`00832cb0`), creates the receiver when there is
/// none (added to the receiver list), marks it playing and just started,
/// adds it to the station's users, sets the station's flag 1 and, with
/// flag `0x100`, restarts its item in 50 ms (`00832490(0)`). Off: stops and
/// releases both handles, marks it not playing and clears the inactive
/// receivers.
// Translation incomplete: 008325b0 - the C++ exception-unwinding frame (FS:[0]) that frees the new receiver is not translated
pub fn fn_008325b0(e: &mut Engine, radio: u32, on: u8) {
    if shutting_down(e) {
        return;
    }
    let receiver = fallout_radio_get_active_receiver(e, radio);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), receiver);
        let mut added = false;
        if receiver != 0 {
            let playing = e.get(
                Ptr::<FORadioReceiver>::new(receiver),
                FORadioReceiver::bShouldBePlaying,
            );
            if (on != 0 && playing != 0) || (on == 0 && playing == 0) {
                return;
            }
        }
        let mut station = 0u32;
        if radio != 0 {
            let base = e.call(FORM_OF, &args![radio]).u32();
            if e.call(FORM_TYPE, &args![base]).u32() == 0x15 {
                let base = e.call(FORM_OF, &args![radio]).u32();
                station = e.call(RADIO_STATION_FORM, &args![base]).u32();
            }
        }
        if station == 0 {
            return;
        }
        let mut entry = fn_00832830(e, station);
        if on != 0 {
            if entry == 0 {
                entry = fn_00832cb0(e, radio, 0);
                if entry == 0 {
                    return;
                }
            }
            if e.mem.u32(slot.addr()) == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let created = if block != 0 {
                    fn_008327d0(e, Ptr::new(block)).addr()
                } else {
                    0
                };
                e.mem.set_u32(slot.addr(), created);
                added = true;
            }
            let receiver = Ptr::<FORadioReceiver>::new(e.mem.u32(slot.addr()));
            e.set(receiver, FORadioReceiver::pRadio, radio);
            e.set(receiver, FORadioReceiver::bShouldBePlaying, 1);
            e.set(receiver, FORadioReceiver::bJustStarted, 1);
            e.set(receiver, FORadioReceiver::bIsActor, 0);
            if added {
                e.call(LIST_ADD_HEAD, &args![RECEIVER_LIST, slot]);
            }
            e.call(LIST_ADD_HEAD, &args![entry + STATION_USERS, slot]);
            let state = Ptr::<StationState>::new(entry + STATION_STATE);
            fn_00832440(e, state, 1);
            if fn_00832420(e, state) {
                let now = audio_time(e);
                e.set(
                    state,
                    StationState::iCurrentItemStartTime,
                    now.wrapping_add(0x32),
                );
                e.call(STATE_CALL_00832490, &args![state, 0u32]);
            }
        } else if receiver != 0 {
            stop_and_release(e, receiver + 4);
            stop_and_release(e, receiver + 0x10);
            e.set(
                Ptr::<FORadioReceiver>::new(receiver),
                FORadioReceiver::bShouldBePlaying,
                0,
            );
            fallout_radio_clear_inactive_receivers(e, 0);
        }
    });
}

// Translated from 008327d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioReceiver` constructor: constructs the sound handle (+4) and
/// the static handle (+0x10). Returns `this`. (The engine map's name,
/// `BSSimpleList<FalloutRadio::FORadioReceiver_P>::AddHead`, does not fit
/// the body.)
// Translation incomplete: 008327d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_008327d0(e: &mut Engine, this: Ptr<FORadioReceiver>) -> Ptr<FORadioReceiver> {
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this.byte_add(4)]);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this.byte_add(0x10)]);
    this
}

// Translated from 00832830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the station entry of a station form (cdecl): among the stations
/// whose reference's form is `form`, the first; a later one replaces it
/// when the first's reference passes `00440da0` and the later's does not,
/// when the best's `0056b210` word is 0 and the later's is not, or when the
/// best's word is not 1 and the later's is 1. 0 while shutting down.
pub fn fn_00832830(e: &mut Engine, form: u32) -> u32 {
    let mut best = 0u32;
    let mut best_word = 0i32;
    if shutting_down(e) {
        return 0;
    }
    let mut node = STATION_LIST;
    while node != 0 && !is_empty(e, node) {
        let station = item_of(e, node);
        let reference = e.mem.u32(station);
        if e.call(FORM_OF, &args![reference]).u32() == form {
            let take = best == 0 || {
                let best_reference = e.mem.u32(best);
                let replaces = e
                    .call(REFERENCE_TEST_00440DA0, &args![best_reference])
                    .bool()
                    && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        !e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool()
                    };
                replaces
                    || (best_word == 0 && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        e.call(REFERENCE_VALUE_0056B210, &args![reference]).u32() != 0
                    })
                    || (best_word != 1 && {
                        let station = item_of(e, node);
                        let reference = e.mem.u32(station);
                        e.call(REFERENCE_VALUE_0056B210, &args![reference]).u32() == 1
                    })
            };
            if take {
                best = item_of(e, node);
                let reference = e.mem.u32(best);
                best_word = e.call(REFERENCE_VALUE_0056B210, &args![reference]).i32();
            }
        }
        node = next_of(e, node);
    }
    best
}

// Translated from 00832930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::GetActiveReceiver` (Xbox PDB name from the call graph,
/// cdecl): the first receiver whose radio is `radio`, or 0.
pub fn fallout_radio_get_active_receiver(e: &mut Engine, radio: u32) -> u32 {
    let mut found = 0u32;
    let mut done = false;
    let mut node = RECEIVER_LIST;
    while node != 0 && !is_empty(e, node) && !done {
        let receiver = item_of(e, node);
        if radio == e.mem.u32(receiver) {
            found = item_of(e, node);
            done = true;
        }
        node = next_of(e, node);
    }
    found
}

// Translated from 008329a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::ClearInactiveReceivers` (Xbox PDB name from the call
/// graph, cdecl): removes every receiver that should not be playing (all
/// of them with `force`) from the receiver list and from every station's
/// users (stopping and releasing its sound handle each time), then deletes
/// it; the walk restarts after each removal.
pub fn fallout_radio_clear_inactive_receivers(e: &mut Engine, force: u8) {
    e.with_stack(4, |e, slot| {
        let mut node = RECEIVER_LIST;
        while node != 0 && !is_empty(e, node) {
            let receiver = item_of(e, node);
            let playing = e.mem.u8(receiver.wrapping_add(0x1c));
            if playing != 0 && force == 0 {
                node = next_of(e, node);
                continue;
            }
            let receiver = item_of(e, node);
            e.mem.set_u32(slot.addr(), receiver);
            let item = e.call(NODE_ITEM, &args![node]).u32();
            e.call(LIST_REMOVE, &args![RECEIVER_LIST, item]);
            node = RECEIVER_LIST;
            let mut station_node = STATION_LIST;
            while station_node != 0 && !is_empty(e, station_node) {
                let station = item_of(e, station_node);
                if station != 0 {
                    while e
                        .call(LIST_CONTAINS, &args![station + STATION_USERS, slot])
                        .bool()
                    {
                        let receiver = e.mem.u32(slot.addr());
                        stop_and_release(e, receiver.wrapping_add(4));
                        e.call(LIST_REMOVE, &args![station + STATION_USERS, slot]);
                    }
                }
                station_node = next_of(e, station_node);
            }
            let receiver = e.mem.u32(slot.addr());
            if receiver != 0 {
                fn_00832190(e, Ptr::new(receiver), 1);
            }
        }
    });
}

// Translated from 00832ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::Update` (Xbox PDB, cdecl): decides whether the stations
/// get a full update (the update interval setting `011dd5cc` passed and no
/// loading menu; the player's check `00425fd0` once, the first time it
/// passes; or `force`), applies the queued state flags (1 or 2) to the
/// radio state, clears the inactive receivers, looks up the sound
/// `0107ce14` once, updates the stations, clears the inactive receivers
/// again and clears the radio state. Answers true.
pub fn fallout_radio_update(e: &mut Engine, force: u8) -> bool {
    let mut full = false;
    if e.global::<u32>(LAST_UPDATE) == 0 {
        let now = e.call(TIMER_TIME, &args![TIMER]).u32();
        e.set_global(LAST_UPDATE, now);
    }
    let elapsed = e
        .call(TIMER_TIME, &args![TIMER])
        .u32()
        .wrapping_sub(e.global::<u32>(LAST_UPDATE));
    let interval = e
        .call(SETTING_VALUE_ADDRESS, &args![UPDATE_INTERVAL_SETTING])
        .u32();
    if elapsed > e.mem.u32(interval) && !e.call(LOADING_MENU_VISIBLE, &args![]).bool() {
        full = true;
        let now = e.call(TIMER_TIME, &args![TIMER]).u32();
        e.set_global(LAST_UPDATE, now);
    }
    let player = e.global::<u32>(PLAYER);
    if player != 0 && e.call(PLAYER_OBJECT, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let object = e.call(PLAYER_OBJECT, &args![player]).u32();
        if e.call(PLAYER_OBJECT_TEST, &args![object]).bool() {
            full = e.global::<u8>(PLAYER_CHECK_SEEN) == 0;
            e.set_global(PLAYER_CHECK_SEEN, 1u8);
        }
    }
    if force != 0 {
        full = true;
    }
    let queued = e.global::<u32>(QUEUED_STATE);
    if queued & 1 != 0 {
        let state = e.global::<u32>(RADIO_STATE) & !2;
        e.set_global(RADIO_STATE, state | 1);
    } else if queued & 2 != 0 {
        let state = e.global::<u32>(RADIO_STATE) & !1;
        e.set_global(RADIO_STATE, state | 2);
    }
    e.set_global(QUEUED_STATE, 0u32);
    fallout_radio_clear_inactive_receivers(e, 0);
    if e.global::<u32>(ATTENUATION_SOUND) == 0 {
        let handler = e.global::<u32>(DATA_HANDLER);
        let sound = e
            .call(GET_SOUND, &args![handler, ATTENUATION_SOUND_ID])
            .u32();
        e.set_global(ATTENUATION_SOUND, sound);
    }
    fallout_radio_update_stations(e, u8::from(full));
    fallout_radio_clear_inactive_receivers(e, 0);
    e.set_global(RADIO_STATE, 0u32);
    true
}

// Translated from 00832cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the station entry of a reference (cdecl): for a radio-kind
/// reference (form types `0x15`, `0x2a`..`0x2d`) that is not an actor, the
/// radio station of its talking activator found in range of itself
/// (`008356e0`); for an actor of those kinds with `other`, the station of
/// `other`'s form; for form type `0x16`, the reference itself. Without one
/// the new entry is deleted and the answer is 0. The entry starts with no
/// conversation, an item start a random 0..30 s in the past, duration 1,
/// flags `0x20`/`0x10` from the form's slot `+0xb4`, flag 2, both powers
/// 100, and is added to the station list.
// Translation incomplete: 00832cb0 - the C++ exception-unwinding frame (FS:[0]) that frees the new entry is not translated
pub fn fn_00832cb0(e: &mut Engine, reference: u32, other: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    let entry = if block != 0 {
        fn_00833150(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let mut radio_kind = false;
    for kind in [0x15u32, 0x2a, 0x2b, 0x2c, 0x2d] {
        let base = e.call(FORM_OF, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == kind {
            radio_kind = true;
            break;
        }
    }
    let base = e.call(FORM_OF, &args![reference]).u32();
    let talker_kind = e.call(FORM_TYPE, &args![base]).u32() == 0x16;
    let delete_entry = |e: &mut Engine| {
        if entry != 0 {
            fn_00832160(e, Ptr::new(entry), 1);
        }
    };
    let mut station = 0u32;
    if radio_kind && !e.vcall(reference, 0x100, &args![]).bool() {
        let base = e.call(FORM_OF, &args![reference]).u32();
        let form = e.call(RADIO_STATION_FORM, &args![base]).u32();
        if form == 0 {
            delete_entry(e);
            return 0;
        }
        station = fn_008356e0(e, form, reference);
        if station == 0 {
            delete_entry(e);
            return 0;
        }
    } else if radio_kind && e.vcall(reference, 0x100, &args![]).bool() && other != 0 {
        let form = e.call(FORM_OF, &args![other]).u32();
        if form == 0 {
            delete_entry(e);
            return 0;
        }
        station = fn_008356e0(e, form, reference);
        if station == 0 {
            delete_entry(e);
            return 0;
        }
    } else if talker_kind {
        station = reference;
    }
    if station == 0 {
        delete_entry(e);
        return 0;
    }
    e.mem.set_u32(entry, station);
    let state = Ptr::<StationState>::new(entry + STATION_STATE);
    e.set(state, StationState::pConvo, 0);
    e.set(state, StationState::pNextConvo, 0);
    let now = audio_time(e);
    let back = e.call(RANDOM_RANGE, &args![0u32, 0x7530u32]).u32();
    e.set(
        state,
        StationState::iCurrentItemStartTime,
        now.wrapping_sub(back),
    );
    e.set(state, StationState::iCurrentItemDuration, 1);
    let form = e.call(FORM_OF, &args![e.mem.u32(entry)]).u32();
    let flag = e.vcall(form, 0xb4, &args![]).u8();
    fn_00833100(e, state, flag);
    let same = u8::from(fn_00833040(e, state));
    fn_008330b0(e, state, same);
    fn_00833060(e, state, 1);
    e.set(state, StationState::cStationPower, 100);
    e.set(state, StationState::cTargetPower, 100);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_ADD_HEAD, &args![STATION_LIST, slot]);
    });
    entry
}

// Translated from 00833040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x20` of `iStationFlags` is set.
pub fn fn_00833040(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x20)
}

// Translated from 00833060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 2 of `iStationFlags`.
pub fn fn_00833060(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 2, on);
}

// Translated from 008330b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x10` of `iStationFlags`.
pub fn fn_008330b0(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x10, on);
}

// Translated from 00833100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x20` of `iStationFlags`.
pub fn fn_00833100(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x20, on);
}

// Translated from 00833150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORadioStation` constructor: constructs the station state at +4.
/// Returns `this`.
pub fn fn_00833150(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00833170(e, Ptr::new(this.addr() + STATION_STATE));
    this
}

// Translated from 00833170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StationState` constructor: constructs the users list (+0x18) and
/// clears both conversations, the start time, the duration and the flags
/// (the two power bytes are left as they are). Returns `this`.
pub fn fn_00833170(e: &mut Engine, this: Ptr<StationState>) -> Ptr<StationState> {
    e.call(LIST_CONSTRUCTOR, &args![this.byte_add(0x18)]);
    e.set(this, StationState::pConvo, 0);
    e.set(this, StationState::pNextConvo, 0);
    e.set(this, StationState::iCurrentItemStartTime, 0);
    e.set(this, StationState::iCurrentItemDuration, 0);
    e.set(this, StationState::iStationFlags, 0);
    this
}

// Translated from 00833c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cStationPower`, in `AL`; the code loads `this` into `EAX` first, so the
/// upper bytes of the result are `this`'s.
pub fn fn_00833c20(e: &mut Engine, this: Ptr<StationState>) -> u32 {
    let power = e.get(this, StationState::cStationPower);
    (this.addr() & !0xff) | u32::from(power)
}

// Translated from 00833c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 8 of `iStationFlags`.
pub fn fn_00833c40(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 8, on);
}

// Translated from 00833d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::UpdateStations` (Xbox PDB, cdecl). With `full`: the
/// stations in range of the player (a reference list and a float list)
/// are walked and each new one (not pending-disabled, not yet reported) is
/// reported with a message (unless the skip byte is set) and remembered;
/// the skip byte is cleared. Then for each station: with `full` and the
/// station in range, its power is computed from the range values
/// (`0056b190`, `0056b1d0`, the signal setting `011dd580`, x87) and stored
/// (`00458b30`) and flag 4 cleared; out of range, flag 4 is set and the
/// power 0. A station in range that is the lost Pip-Boy station is
/// reported and handed back to the Pip-Boy. Each station is then updated
/// (`00834260`). Nothing while shutting down.
// Translation incomplete: 00833d00 - the C++ exception-unwinding frame (FS:[0]) that destroys the lists, strings and sound handles is not translated
pub fn fallout_radio_update_stations(e: &mut Engine, full: u8) {
    e.with_stack(0x10, |e, lists| {
        let references = lists.addr();
        let ranges = lists.addr() + 8;
        e.call(LIST_CONSTRUCTOR, &args![references]);
        fn_008373d0(e, Ptr::new(ranges));
        if shutting_down(e) {
            e.call(FLOAT_LIST_DESTRUCTOR, &args![ranges]);
            e.call(LIST_DESTRUCTOR, &args![references]);
            return;
        }
        if full != 0 {
            report_new_stations(e, references, ranges);
        }
        let signal = e.call(SETTING_FLOAT_ADDRESS, &args![SIGNAL_SETTING]).u32();
        let mut falloff = x87_store32(x87_sub(1.0, x87_m32(e.mem.u32(signal))));
        let mut node = STATION_LIST;
        while node != 0 && !is_empty(e, node) {
            let mut reference_node = references;
            let mut range_node = ranges;
            let mut found = false;
            // The code resets the range to the largest float here; the value
            // is overwritten before any read (only a station in range reads it).
            let station = item_of(e, node);
            if station != 0 {
                while full != 0 && reference_node != 0 && range_node != 0 {
                    if item_of(e, reference_node) != e.mem.u32(station) {
                        reference_node = next_of(e, reference_node);
                        range_node = next_of(e, range_node);
                        continue;
                    }
                    found = true;
                    let mut power = 100u8;
                    fn_00834210(e, Ptr::new(station + STATION_STATE), 0);
                    let slot = e.call(NODE_ITEM, &args![range_node]).u32();
                    let range = x87_quiet_bits(e.mem.u32(slot));
                    let value = e
                        .call(STATION_VALUE_0056B190, &args![e.mem.u32(station)])
                        .f64();
                    let station_range = x87_store32(value);
                    let strength = e
                        .call(STATION_VALUE_0056B1D0, &args![e.mem.u32(station)])
                        .f64();
                    if strength != e.global::<f64>(ZERO_DOUBLE) {
                        let strength = e
                            .call(STATION_VALUE_0056B1D0, &args![e.mem.u32(station)])
                            .f64();
                        let scaled = x87_div(strength, e.global::<f64>(HUNDRED_DOUBLE));
                        falloff = x87_store32(x87_sub(1.0, scaled));
                    }
                    let zero = e.global::<f64>(ZERO_DOUBLE);
                    if e.call(REFERENCE_VALUE_0056B210, &args![e.mem.u32(station)])
                        .u32()
                        == 0
                        && x87_m32(station_range) > zero
                        && x87_m32(range) > zero
                    {
                        let ratio = x87_store32(x87_div(x87_m32(range), x87_m32(station_range)));
                        // FCOMPP; TEST AH,5; JP: only when the falloff is
                        // below the ratio.
                        if x87_m32(falloff) < x87_m32(ratio) {
                            let over = x87_sub(x87_m32(ratio), x87_m32(falloff));
                            let span = x87_sub(1.0, x87_m32(falloff));
                            let lost =
                                x87_mul(x87_div(over, span), e.global::<f64>(HUNDRED_DOUBLE));
                            let lost = crate::x87::fistp_chop(lost);
                            power = 100u8.wrapping_sub(lost as u8);
                        }
                    }
                    e.call(
                        STATE_SET_POWER,
                        &args![station + STATION_STATE, u32::from(power)],
                    );
                    reference_node = 0;
                    range_node = 0;
                }
                if full != 0 && !found {
                    fn_00834210(e, Ptr::new(station + STATION_STATE), 1);
                    e.call(STATE_SET_POWER, &args![station + STATION_STATE, 0u32]);
                }
                if found && e.global::<u32>(LOST_PIPBOY_STATION) == station {
                    e.with_stack(0x14, |e, locals| {
                        let text = locals.addr();
                        let handle = locals.addr() + 8;
                        e.call(STRING_CONSTRUCT, &args![text]);
                        play_discovery_sound(e, handle);
                        show_discovery(e, text, e.mem.u32(station));
                        e.call(PIPBOY_RADIO_ENABLE, &args![1u32]);
                        e.call(SET_PIPBOY_STATION, &args![e.mem.u32(station), 0u32]);
                        e.set_global(LOST_PIPBOY_STATION, 0u32);
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                        e.call(STRING_DESTROY, &args![text]);
                    });
                }
                fn_00834260(e, station);
            }
            node = next_of(e, node);
        }
        e.call(FLOAT_LIST_CLEAR, &args![ranges]);
        e.call(LIST_CLEAR, &args![references]);
        e.call(FLOAT_LIST_DESTRUCTOR, &args![ranges]);
        e.call(LIST_DESTRUCTOR, &args![references]);
    });
}

/// Plays the new-station sound into the handle on the stack (flags
/// `0x100121`, `Play(0)` on the handle).
fn play_discovery_sound(e: &mut Engine, handle: u32) {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(
        AUDIO_SOUND_BY_NAME,
        &args![audio, handle, DISCOVERY_SOUND, 0x0010_0121u32],
    );
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

/// Formats the station's name with the setting `011d4348` into the string
/// and shows it as a message.
fn show_discovery(e: &mut Engine, text: u32, reference: u32) {
    let name = e.call(REFERENCE_NAME, &args![reference]).u32();
    let format = e.call(SETTING_STRING, &args![DISCOVERY_SETTING]).u32();
    e.call(STRING_FORMAT, &args![text, format, name]);
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let c_str = e.call(STRING_C_STR, &args![text]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![c_str, 0u32, DISCOVERY_TEXT, 0u32, time, 0u32],
    );
}

/// The `full` part of [`fallout_radio_update_stations`] that reports the
/// stations newly in range.
fn report_new_stations(e: &mut Engine, references: u32, ranges: u32) {
    let player = e.global::<u32>(PLAYER);
    e.call(STATIONS_IN_RANGE, &args![player, references, ranges]);
    let mut node = references;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let reference = item_of(e, node);
        let skip = e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool() && {
            let reference = item_of(e, node);
            !e.call(PENDING_ENABLED, &args![reference]).bool()
        };
        if !skip {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if !e.call(LIST_CONTAINS, &args![DISCOVERED_LIST, slot]).bool() {
                if e.global::<u8>(SKIP_DISCOVERY_MESSAGE) == 0 {
                    e.with_stack(0x14, |e, locals| {
                        let text = locals.addr();
                        let handle = locals.addr() + 8;
                        e.call(STRING_CONSTRUCT, &args![text]);
                        play_discovery_sound(e, handle);
                        let reference = item_of(e, node);
                        show_discovery(e, text, reference);
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                        e.call(STRING_DESTROY, &args![text]);
                    });
                }
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                e.call(LIST_ADD_HEAD, &args![DISCOVERED_LIST, slot]);
            }
        }
        node = next_of(e, node);
    }
    e.set_global(SKIP_DISCOVERY_MESSAGE, 0u8);
}

// Translated from 00834210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag 4 of `iStationFlags`.
pub fn fn_00834210(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 4, on);
}

/// Callees of `00834260` (`FalloutRadio::UpdateStation` by the name it
/// enters the radio's critical section with).
const RADIO_CRITICAL_SECTION: u32 = 0x011d_d5e8;
const UPDATE_STATION_NAME: u32 = 0x0107_ceb0;
const ENTER_CRITICAL_SECTION: u32 = 0x0045_38a0;
const LEAVE_CRITICAL_SECTION: u32 = 0x0045_38c0;
const AUDIO_LISTENER_POSITION: u32 = 0x00ad_78e0;
const CURRENT_RESPONSE: u32 = 0x0083_c820;
const NEXT_ITEM: u32 = 0x0083_b9f0;
const FIRST_ITEM: u32 = 0x0083_b9a0;
const PRECACHE_SOUNDS: u32 = 0x0083_c470;
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const NEXT_RESPONSE: u32 = 0x0083_c7e0;
const RESPONSE_TOPIC_INFO: u32 = 0x0084_e3a0;
const TOPIC_INFO_TEST: u32 = 0x0057_9200;
const RUN_RESULT: u32 = 0x0061_f170;
const STATE_TEST_00833C00: u32 = 0x0083_3c00;
const STATE_CALL_00835690: u32 = 0x0083_5690;
const STATE_TEST_00835670: u32 = 0x0083_5670;
const RADIO_TEST_00833CE0: u32 = 0x0083_3ce0;
const RADIO_CALL_008325A0: u32 = 0x0083_25a0;
const MEDIA_CLOSE: u32 = 0x0083_04a0;
const MEDIA_PAUSE: u32 = 0x0083_05e0;
const MEDIA_UNPAUSE: u32 = 0x0083_0610;
const MEDIA_PLAY_008300C0: u32 = 0x0083_00c0;
const MEDIA_CALL_008304C0: u32 = 0x0083_04c0;
const MEDIA_DURATION: u32 = 0x0083_08c0;
const PIPBOY_UPDATE: u32 = 0x0083_31c0;
const STATE_VALUE_00824060: u32 = 0x0082_4060;
const STATE_CALL_00632D20: u32 = 0x0063_2d20;
const SOUND_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_PAUSE: u32 = 0x00ad_88b0;
const SOUND_DURATION: u32 = 0x00ad_8b30;
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0;
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_SET_MIN_MAX: u32 = 0x00ad_8be0;
const SOUND_SET_COMPLETION: u32 = 0x00ad_8e60;
const SOUND_SET_SYNCH_START: u32 = 0x00ad_8fd0;
const SOUND_SET_FLAG_004F15A0: u32 = 0x004f_15a0;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const AUDIO_SOUND_BY_FILE: u32 = 0x00ad_7480;
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
const SOUND_FORM_BYTE: u32 = 0x0055_3b90;
const ACTOR_VOICE_TYPE: u32 = 0x0041_81e0;
const TOPIC_INFO_SPEAKER: u32 = 0x0063_9b40;
const RESPONSE_OBJECT: u32 = 0x0044_ddc0;
const RESPONSE_PARENT: u32 = 0x0096_11e0;
const RESPONSE_LOCATION: u32 = 0x0044_1110;
const LIST_COUNT_0044A670: u32 = 0x0044_a670;
const RESPONSE_LIST_00460140: u32 = 0x0046_0140;
const SOUND_FILE_NAME: u32 = 0x0051_1840;
const STRING_FIND: u32 = 0x0048_12f0;
const STRING_FIND_LAST: u32 = 0x0043_b9d0;
const COPY_BOUNDED: u32 = 0x0040_6d30;
const APPEND_BOUNDED: u32 = 0x0040_6d50;
const VOLUME_CURVE: u32 = 0x004d_d150;
const SOUND_SOURCE_CAST: u32 = 0x0040_7840;
const RTTI_CAST: u32 = 0x00ec_43fb;
/// The Pip-Boy's voice and static handles (`PipboyVoiceHndl`,
/// `PipboyStaticHndl` in the Xbox PDB; which is which is not confirmed).
const PIPBOY_HANDLE_5BC: u32 = 0x011d_d5bc;
const PIPBOY_HANDLE_638: u32 = 0x011d_d638;
/// Constants and settings of `00834260`.
const DEFAULT_RANGE: u32 = 0x0104_fca0;
const NO_POSITION: u32 = 0x011f_426c;
const STATIC_SOUND: u32 = 0x0107_ce34;
const VOLUME_EXPONENT: u32 = 0x0101_6264;
const RANGE_SCALE: u32 = 0x0106_ebc8;
const ACTOR_RANGE_SETTING: u32 = 0x011d_d62c;
const ACTOR_RANGE_SCALE: u32 = 0x0102_90f8;
const MIN_RANGE_SETTING: u32 = 0x011d_d5b0;
const VOICE_SUFFIX: u32 = 0x0107_cea8;
const TYPE_FORM_BASE: u32 = 0x0118_3108;
const TYPE_SOUND_SOURCE: u32 = 0x0118_6584;
const COMPLETION_CALLBACK: u32 = 0x0083_57e0;

/// The volume of a power byte, `power / 100.0` as `float`.
fn power_volume(e: &Engine, power: u8) -> u32 {
    x87_store32(x87_div(f64::from(power), e.global::<f64>(HUNDRED_DOUBLE)))
}

/// The static's volume: `004dd150((100 - power) / 100, exponent)`.
fn static_volume(e: &mut Engine, power: u8) -> u32 {
    let exponent = x87_quiet_bits(e.global::<u32>(VOLUME_EXPONENT));
    let lost = 100i32 - i32::from(power);
    let base = x87_store32(x87_div(f64::from(lost), e.global::<f64>(HUNDRED_DOUBLE)));
    let value = e
        .call(VOLUME_CURVE, &args![x87_quiet_bits(base), exponent])
        .f64();
    x87_store32(value)
}

/// Runs the result script of the response's topic info unless its test
/// (`00579200`) passes, then starts its first response and restarts the
/// item in 50 ms.
fn start_response(e: &mut Engine, station: u32) {
    let conversation = e.mem.u32(station + 4);
    let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
    if e.call(RESPONSE_TOPIC_INFO, &args![response]).u32() != 0 {
        let conversation = e.mem.u32(station + 4);
        let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
        let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
        if !e.call(TOPIC_INFO_TEST, &args![info]).bool() {
            let reference = e.mem.u32(station);
            let conversation = e.mem.u32(station + 4);
            let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
            e.call(RUN_RESULT, &args![info, 0u32, reference]);
        }
    }
    let conversation = e.mem.u32(station + 4);
    let response = e.call(CURRENT_RESPONSE, &args![conversation]).u32();
    e.call(FIRST_RESPONSE, &args![response]);
    let now = audio_time(e);
    e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
    e.mem.set_u32(station + 0x10, 0);
}

/// Stops and releases every user's sound handle.
fn stop_users(e: &mut Engine, station: u32) {
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        e.call(SOUND_HANDLE_STOP, &args![receiver + 4]);
        let receiver = item_of(e, node);
        e.call(SOUND_HANDLE_RELEASE, &args![receiver + 4]);
        node = next_of(e, node);
    }
}

// Translated from 00834260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates one station (cdecl; `FalloutRadio::UpdateStation` by the name
/// it enters the radio's critical section `011dd5e8` with). Nothing for a
/// null station or reference.
///
/// - While shutting down it returns (the critical section is left
///   entered, as the code does); a station that is not the Pip-Boy's and
///   has no users leaves it and returns.
/// - With the station disabled (`00440da0`) or the radio state paused (1)
///   or resumed (2), every user's handles are stopped and released,
///   paused or replayed (playing users), and the Pip-Boy's handles and
///   media follow; then it returns.
/// - Otherwise the item's duration is taken from the Pip-Boy handle or the
///   media (`008308c0(7)`), a start time in the future is pulled back, and
///   when the item is over the conversation moves to its next item
///   (running its result script) or is deleted, the users' handles stop
///   and the Pip-Boy station's media restarts; with no response a new
///   conversation starts when flag `0x20` is set (`GetNextConversation`).
///   A response without a sound restarts in 50 ms.
/// - Each playing user then gets its position (the radio's slot `+0x1f4`),
///   is muted out of the player's worldspace or cell, gets the static
///   (`0107ce34`, volume from the power, `004dd150`) while the power is
///   below 100, and, when it has no sound playing and the station has
///   flag 1, starts the response's voice file (the actor's slot `+0x284`
///   for an NPC whose voice type matches, else by file name with a
///   completion callback `008357e0`) when the listener is within range.
/// - The Pip-Boy update (`008331c0`) runs once, with 1 for a user that
///   did not just start, else with 0 at the end; the critical section is
///   left.
// Translation incomplete: 00834260 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary sound handles and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_00834260(e: &mut Engine, station: u32) {
    if station == 0 || e.mem.u32(station) == 0 {
        return;
    }
    e.with_stack(0x210, |e, frame| {
        update_station(e, station, frame.addr() + 0x210)
    });
}

/// The body of [`fn_00834260`]; `ebp` is the end of the modelled frame
/// (the address-taken locals sit below it at the exe's offsets).
fn update_station(e: &mut Engine, station: u32, ebp: u32) {
    let listener = ebp - 0x24;
    let position = ebp - 0x5c;
    let path = ebp - 0x17c;
    let extension = ebp - 0x184;
    let difference = ebp - 0x1a8;
    let state = Ptr::<StationState>::new(station + STATION_STATE);
    let mut has_voice_file = false;
    let mut has_alternative = false;
    let mut pipboy_updated = false;
    let disabled = e
        .call(REFERENCE_TEST_00440DA0, &args![e.mem.u32(station)])
        .bool();
    let paused = e.global::<u32>(RADIO_STATE) & 1 != 0;
    let resumed = e.global::<u32>(RADIO_STATE) & 2 != 0;
    let is_pipboy = station == e.global::<u32>(CURRENT_PIPBOY_STATION);
    let mut range = x87_quiet_bits(e.global::<u32>(DEFAULT_RANGE));
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_LISTENER_POSITION, &args![audio, listener]);
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![RADIO_CRITICAL_SECTION, UPDATE_STATION_NAME],
    );
    let mut response = if e.mem.u32(station + 4) != 0 {
        e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32()
    } else {
        0
    };
    if shutting_down(e) {
        return;
    }
    if !is_pipboy && is_empty(e, station + STATION_USERS) {
        e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
        return;
    }
    let mut sound = e.global::<u32>(ATTENUATION_SOUND);
    if e.mem.u32(station) != 0 {
        let form = e.call(FORM_OF, &args![e.mem.u32(station)]).u32();
        let source = e
            .call(
                RTTI_CAST,
                &args![form, 0i32, TYPE_FORM_BASE, TYPE_SOUND_SOURCE, 0i32],
            )
            .u32();
        if source != 0 && e.call(SOUND_SOURCE_CAST, &args![source]).u32() != 0 {
            sound = e.call(SOUND_SOURCE_CAST, &args![source]).u32();
        }
    }
    if disabled || paused || resumed {
        pause_or_stop(e, station, disabled, paused, resumed, is_pipboy);
        e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
        return;
    }
    if e.get(state, StationState::iCurrentItemDuration) <= 0 && is_pipboy {
        if e.call(SOUND_IS_VALID, &args![PIPBOY_HANDLE_5BC]).bool() {
            let duration = e.call(SOUND_DURATION, &args![PIPBOY_HANDLE_5BC]).u32();
            e.mem.set_u32(station + 0x10, duration);
        } else if response != 0 {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            if e.call(FORM_OF, &args![current]).u32() != 0
                && e.call(MEDIA_DURATION, &args![7i32]).u32() != 0
            {
                let duration = e.call(MEDIA_DURATION, &args![7i32]).u32();
                e.mem.set_u32(station + 0x10, duration);
            }
        }
    }
    let now = audio_time(e);
    if e.mem.u32(station + 0xc) > now.wrapping_add(100) {
        e.mem.set_u32(station + 0xc, now);
    }
    if e.mem.i32(station + 0x10) > 0
        && e.mem
            .u32(station + 0xc)
            .wrapping_add(e.mem.u32(station + 0x10))
            < now
    {
        if e.mem.u32(station + 4) != 0
            && e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                .u32()
                != 0
        {
            if e.call(NEXT_ITEM, &args![e.mem.u32(station + 4)]).bool() {
                start_response(e, station);
                if is_empty(e, station + STATION_USERS)
                    && e.global::<u32>(CURRENT_PIPBOY_STATION) == 0
                {
                    e.call(STATE_CALL_00832490, &args![state, 1u32]);
                }
            } else {
                let conversation = e.mem.u32(station + 4);
                delete_conversation(e, conversation);
                e.mem.set_u32(station + 4, 0);
                response = 0;
                e.mem.set_u32(station + 0xc, 0);
                e.mem.set_u32(station + 0x10, 0);
                if e.call(STATE_TEST_00833C00, &args![state]).bool() {
                    e.call(STATE_CALL_00835690, &args![state, 0u32]);
                }
            }
            stop_users(e, station);
            if station == e.global::<u32>(CURRENT_PIPBOY_STATION) {
                stop_and_release(e, PIPBOY_HANDLE_5BC);
                if !e.call(RADIO_TEST_00833CE0, &args![]).bool() {
                    e.call(MEDIA_CLOSE, &args![]);
                    e.call(RADIO_CALL_008325A0, &args![1u32]);
                    e.call(
                        MEDIA_PLAY_008300C0,
                        &args![7i32, 0u32, 0x3e8u32, 0u32, 0u32, 0.0f32, 0u32],
                    );
                    e.call(MEDIA_CALL_008304C0, &args![]);
                }
            }
        }
        if e.mem.u32(station + 4) == 0
            || e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                .u32()
                == 0
        {
            if fn_00833040(e, state) {
                fn_008330b0(e, state, 1);
                fn_00837060(e, station, 0);
                e.call(PRECACHE_SOUNDS, &args![e.mem.u32(station + 4)]);
                e.call(FIRST_ITEM, &args![e.mem.u32(station + 4)]);
                if e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                    .u32()
                    != 0
                {
                    response = e
                        .call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
                        .u32();
                    start_response(e, station);
                    stop_users(e, station);
                    if station == e.global::<u32>(CURRENT_PIPBOY_STATION) {
                        stop_and_release(e, PIPBOY_HANDLE_5BC);
                    }
                }
            } else {
                fn_008330b0(e, state, 0);
            }
        }
    }
    if e.mem.u32(station + 4) != 0
        && e.call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32()
            != 0
    {
        response = e
            .call(CURRENT_RESPONSE, &args![e.mem.u32(station + 4)])
            .u32();
        if e.call(CURRENT_RESPONSE, &args![response]).u32() == 0
            && !e.call(NEXT_RESPONSE, &args![response]).bool()
        {
            let now = audio_time(e);
            e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
            e.mem.set_u32(station + 0x10, 1);
        }
    }
    let byte = e.call(STATE_VALUE_00824060, &args![state]).u32() & 0xff;
    e.call(STATE_CALL_00632D20, &args![state, byte]);
    let power = fn_00833c20(e, state) as u8;
    let static_on = power < 100;
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if e.mem.u8(receiver + 0x1c) == 0 {
            node = next_of(e, node);
            continue;
        }
        if e.mem.i32(station + 0x10) <= 0 {
            let receiver = item_of(e, node);
            if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
                let receiver = item_of(e, node);
                if e.call(SOUND_DURATION, &args![receiver + 4]).u32() != 0 {
                    let receiver = item_of(e, node);
                    let duration = e.call(SOUND_DURATION, &args![receiver + 4]).u32();
                    e.mem.set_u32(station + 0x10, duration);
                }
            }
        }
        for i in 0..3 {
            let word = e.mem.u32(NO_POSITION + 4 * i);
            e.mem.set_u32(position + 4 * i, word);
        }
        for i in 0..3 {
            let largest = x87_quiet_bits(e.global::<u32>(LARGEST_FLOAT));
            e.mem.set_u32(position + 4 * i, largest);
        }
        let receiver = item_of(e, node);
        if receiver != 0 && e.mem.u32(receiver) != 0 {
            let radio = radio_of(e, node);
            let at = e.vcall(radio, 0x1f4, &args![]).u32();
            for i in 0..3 {
                let word = e.mem.u32(at + 4 * i);
                e.mem.set_u32(position + 4 * i, word);
            }
        } else if receiver != 0 {
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1c, 0);
            node = next_of(e, node);
            continue;
        }
        if out_of_reach(e, node) {
            let receiver = item_of(e, node);
            e.call(SOUND_SET_VOLUME, &args![receiver + 4, 0.0f32]);
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1c, 0);
        }
        if item_of(e, node) != 0 {
            let receiver = item_of(e, node);
            if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
                let handle = item_of(e, node) + 4;
                set_position(e, handle, position);
            }
        }
        update_static(e, node, power, static_on, position);
        let receiver = item_of(e, node);
        let can_start = response != 0
            && !e.call(SOUND_IS_VALID, &args![receiver + 4]).bool()
            && fn_00835650(e, state)
            && receiver_byte(e, node, 0x1c) != 0
            && radio_of(e, node) != 0;
        if !can_start {
            let receiver = item_of(e, node);
            if e.mem.u32(receiver) == 0 {
                e.mem.set_u8(receiver + 0x1c, 0);
            }
            node = next_of(e, node);
            continue;
        }
        if e.call(CURRENT_RESPONSE, &args![response]).u32() == 0 {
            if !e.call(NEXT_RESPONSE, &args![response]).bool()
                && !e.call(FIRST_RESPONSE, &args![response]).bool()
            {
                let now = audio_time(e);
                e.mem.set_u32(station + 0xc, now.wrapping_add(0x32));
                e.mem.set_u32(station + 0x10, 1);
            }
            node = next_of(e, node);
            continue;
        }
        let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
        if e.call(FORM_OF, &args![current]).u32() != 0 {
            has_voice_file = true;
        }
        let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
        if current != 0 {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let slot = e.call(NODE_ITEM, &args![current]).u32();
            if e.call(STRING_C_STR, &args![slot]).u32() != 0 {
                let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
                let slot = e.call(NODE_ITEM, &args![current]).u32();
                let list = e.call(STRING_C_STR, &args![slot]).u32();
                let count = e.call(LIST_COUNT_0044A670, &args![list]).u32();
                has_alternative = count > 1 && {
                    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
                    let alternatives = e.call(RESPONSE_LIST_00460140, &args![current]).u32();
                    e.call(STRING_C_STR, &args![alternatives]).u32() != 0
                };
            }
        }
        if has_voice_file {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let form = e.call(FORM_OF, &args![current]).u32();
            let name = e.call(SOUND_FILE_NAME, &args![form]).u32();
            e.call(COPY_BOUNDED, &args![path, 0x104u32, name]);
            if e.call(STRING_FIND, &args![path, VOICE_SUFFIX]).u32() == 0 {
                let dot = e.call(STRING_FIND_LAST, &args![path, 0x2ei32]).u32();
                if dot != 0 {
                    e.call(COPY_BOUNDED, &args![extension, 4u32, dot + 1]);
                    e.mem.set_u8(dot, 0);
                    e.call(APPEND_BOUNDED, &args![path, 0x104u32, VOICE_SUFFIX]);
                    e.call(APPEND_BOUNDED, &args![path, 0x104u32, extension]);
                }
            }
            fn_00833c40(e, state, 1);
        } else if has_alternative {
            let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
            let alternatives = e.call(RESPONSE_LIST_00460140, &args![current]).u32();
            let text = e.call(STRING_C_STR, &args![alternatives]).u32();
            e.call(COPY_BOUNDED, &args![path, 0x104u32, text]);
            fn_00833c40(e, state, 0);
        } else {
            e.mem.set_u32(station + 0x10, 1);
            node = next_of(e, node);
            fn_00833c40(e, state, 0);
            continue;
        }
        let mut voice_match = false;
        let mut is_actor_speaker = false;
        let mut actor = 0u32;
        let radio = radio_of(e, node);
        if radio != 0 && is_actor(e, radio) {
            actor = radio_of(e, node);
            if e.call(RESPONSE_TOPIC_INFO, &args![response]).u32() != 0 {
                let voice = e.call(ACTOR_VOICE_TYPE, &args![actor]).u32();
                let info = e.call(RESPONSE_TOPIC_INFO, &args![response]).u32();
                if voice == e.call(TOPIC_INFO_SPEAKER, &args![info]).u32() {
                    voice_match = true;
                }
            }
            if e.vcall(actor, 0x21c, &args![]).bool() {
                is_actor_speaker = true;
            }
        }
        if sound != 0 && !is_actor_speaker {
            let byte = e.call(SOUND_FORM_BYTE, &args![sound]).u8();
            range = x87_store32(x87_mul(f64::from(byte), e.global::<f64>(RANGE_SCALE)));
        } else if is_actor_speaker {
            let setting = e
                .call(SETTING_FLOAT_ADDRESS, &args![ACTOR_RANGE_SETTING])
                .u32();
            range = x87_store32(x87_mul(
                x87_m32(e.mem.u32(setting)),
                e.global::<f64>(ACTOR_RANGE_SCALE),
            ));
        }
        let away = e
            .call(VECTOR_SUBTRACT, &args![listener, difference, position])
            .u32();
        let distance = e.call(VECTOR_LENGTH, &args![away]).f64();
        // FCOMPP; TEST AH,5; JP: a user beyond the range (the range below
        // the distance) is skipped; NaN is not.
        if x87_m32(range) < distance {
            node = next_of(e, node);
            continue;
        }
        audio_time(e);
        if actor != 0 && voice_match {
            speak_with_actor(e, actor, node, response, path);
        } else {
            let voice = Voice {
                station,
                sound,
                is_actor_speaker,
                power,
                position,
                path,
                ebp,
            };
            play_voice_file(e, node, &voice);
        }
        if is_pipboy && receiver_byte(e, node, 0x1d) == 0 && !pipboy_updated {
            pipboy_updated = true;
            e.call(PIPBOY_UPDATE, &args![1u32]);
        }
        if receiver_byte(e, node, 0x1d) != 0 {
            let receiver = item_of(e, node);
            e.mem.set_u8(receiver + 0x1d, 0);
        }
        node = next_of(e, node);
    }
    if station == e.global::<u32>(CURRENT_PIPBOY_STATION) && !pipboy_updated {
        e.call(PIPBOY_UPDATE, &args![0u32]);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![RADIO_CRITICAL_SECTION]);
}

/// Whether a user's radio is out of the player's reach: another
/// worldspace (when the player has one), or, when the player's object
/// passes `00425fd0`, another object (`008d6f30`).
fn out_of_reach(e: &mut Engine, node: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    if e.call(REFERENCE_WORLD_SPACE, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let mine = e.call(REFERENCE_WORLD_SPACE, &args![player]).u32();
        let radio = radio_of(e, node);
        if mine != e.call(REFERENCE_WORLD_SPACE, &args![radio]).u32() {
            return true;
        }
    }
    let player = e.global::<u32>(PLAYER);
    let object = e.call(PLAYER_OBJECT, &args![player]).u32();
    if !e.call(PLAYER_OBJECT_TEST, &args![object]).bool() {
        return false;
    }
    let radio = radio_of(e, node);
    let theirs = e.call(PLAYER_OBJECT, &args![radio]).u32();
    let player = e.global::<u32>(PLAYER);
    theirs != e.call(PLAYER_OBJECT, &args![player]).u32()
}

/// Sets a handle's position from the three floats at `position` (each
/// copied as an `FLD`/`FSTP` copy).
fn set_position(e: &mut Engine, handle: u32, position: u32) {
    let x = x87_quiet_bits(e.mem.u32(position));
    let y = x87_quiet_bits(e.mem.u32(position + 4));
    let z = x87_quiet_bits(e.mem.u32(position + 8));
    e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
}

/// The disabled / paused / resumed part of [`fn_00834260`].
fn pause_or_stop(
    e: &mut Engine,
    station: u32,
    disabled: bool,
    paused: bool,
    resumed: bool,
    is_pipboy: bool,
) {
    let mut node = station + STATION_USERS;
    while node != 0 && !is_empty(e, node) {
        let receiver = item_of(e, node);
        if e.call(SOUND_IS_VALID, &args![receiver + 4]).bool() {
            if disabled {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_STOP, &args![receiver + offset]);
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_RELEASE, &args![receiver + offset]);
                }
            } else if paused {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_PAUSE, &args![receiver + offset]);
                }
            } else if resumed && receiver_byte(e, node, 0x1c) != 0 {
                for offset in [4u32, 0x10] {
                    let receiver = item_of(e, node);
                    e.call(SOUND_HANDLE_PLAY, &args![receiver + offset, 0u32]);
                }
            }
        }
        node = next_of(e, node);
    }
    if !is_pipboy {
        return;
    }
    if disabled {
        e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
    }
    let current = e.global::<u32>(CURRENT_PIPBOY_STATION);
    if current != 0
        && e.call(STATE_TEST_00835670, &args![current + STATION_STATE])
            .bool()
    {
        if paused {
            e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_638]);
            e.call(MEDIA_PAUSE, &args![]);
        } else if resumed {
            e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_638, 0u32]);
            e.call(MEDIA_UNPAUSE, &args![]);
        }
    } else if paused {
        e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_638]);
        e.call(SOUND_PAUSE, &args![PIPBOY_HANDLE_5BC]);
    } else if resumed {
        e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_638, 0u32]);
        e.call(SOUND_HANDLE_PLAY, &args![PIPBOY_HANDLE_5BC, 0u32]);
    }
}

/// The static of a user in [`fn_00834260`]: started (`0107ce34`) when the
/// power is below 100 and none plays, kept at the power's volume and the
/// position while it plays, stopped when the power is back to 100.
fn update_static(e: &mut Engine, node: u32, power: u8, static_on: bool, position: u32) {
    let handle = item_of(e, node) + 0x10;
    if static_on && !e.call(SOUND_IS_VALID, &args![handle]).bool() {
        let handle = item_of(e, node) + 0x10;
        stop_and_release(e, handle);
        e.with_stack(0xc, |e, temporary| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let created = e
                .call(
                    AUDIO_SOUND_BY_NAME,
                    &args![audio, temporary, STATIC_SOUND, 0x0010_0012u32],
                )
                .u32();
            let handle = item_of(e, node) + 0x10;
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, created]);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
        });
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_FLAG_004F15A0, &args![handle, 1u32]);
        let handle = item_of(e, node) + 0x10;
        set_position(e, handle, position);
        let volume = static_volume(e, power);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_VOLUME, &args![handle, volume]);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
    } else if static_on {
        let volume = static_volume(e, power);
        let handle = item_of(e, node) + 0x10;
        e.call(SOUND_SET_VOLUME, &args![handle, volume]);
        let handle = item_of(e, node) + 0x10;
        set_position(e, handle, position);
    } else {
        let handle = item_of(e, node) + 0x10;
        if e.call(SOUND_IS_VALID, &args![handle]).bool() {
            let handle = item_of(e, node) + 0x10;
            stop_and_release(e, handle);
        }
    }
}

/// An NPC speaking the response itself (its slot `+0x284`, the float
/// result dropped).
fn speak_with_actor(e: &mut Engine, actor: u32, node: u32, response: u32, path: u32) {
    let radio = radio_of(e, node);
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let location = e.call(RESPONSE_LOCATION, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let parent = e.call(RESPONSE_PARENT, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let info = e.call(RESPONSE_TOPIC_INFO, &args![current]).u32();
    let current = e.call(CURRENT_RESPONSE, &args![response]).u32();
    let object = e.call(RESPONSE_OBJECT, &args![current]).u32();
    let handle = item_of(e, node) + 4;
    e.vcall(
        actor,
        0x284,
        &args![
            path, handle, object, info, 0x96u32, parent, location, radio, 0u32, 0u32, 0u32, 1u32,
            1u32
        ],
    );
}

/// What [`play_voice_file`] plays and where.
struct Voice {
    station: u32,
    sound: u32,
    is_actor_speaker: bool,
    power: u8,
    position: u32,
    path: u32,
    ebp: u32,
}

/// Starts the voice file at `path` on a user's handle with the completion
/// callback, start time, position and the power's volume (an actor
/// speaker's handle also gets the two range settings).
fn play_voice_file(e: &mut Engine, node: u32, voice: &Voice) {
    let flags = 0x0050_0102u32;
    let temporary = if voice.is_actor_speaker {
        voice.ebp - 0x1b4
    } else {
        voice.ebp - 0x1c0
    };
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let form = if voice.is_actor_speaker {
        0
    } else {
        voice.sound
    };
    let created = e
        .call(
            AUDIO_SOUND_BY_FILE,
            &args![audio, temporary, voice.path, flags, form],
        )
        .u32();
    let handle = item_of(e, node) + 4;
    e.call(SOUND_HANDLE_ASSIGN, &args![handle, created]);
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![temporary]);
    if voice.is_actor_speaker {
        let far = e
            .call(SETTING_FLOAT_ADDRESS, &args![ACTOR_RANGE_SETTING])
            .u32();
        let far = x87_quiet_bits(e.mem.u32(far));
        let near = e
            .call(SETTING_FLOAT_ADDRESS, &args![MIN_RANGE_SETTING])
            .u32();
        let near = x87_quiet_bits(e.mem.u32(near));
        let handle = item_of(e, node) + 4;
        e.call(SOUND_SET_MIN_MAX, &args![handle, near, far]);
    }
    let handle = item_of(e, node) + 4;
    e.call(
        SOUND_SET_COMPLETION,
        &args![handle, COMPLETION_CALLBACK, voice.station],
    );
    let handle = item_of(e, node) + 4;
    let start = e.mem.u32(voice.station + 0xc);
    e.call(SOUND_SET_SYNCH_START, &args![handle, start]);
    let handle = item_of(e, node) + 4;
    set_position(e, handle, voice.position);
    let volume = power_volume(e, voice.power);
    let handle = item_of(e, node) + 4;
    e.call(SOUND_SET_VOLUME, &args![handle, volume]);
    let handle = item_of(e, node) + 4;
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

// Translated from 00835650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag 1 of `iStationFlags` is set.
pub fn fn_00835650(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 1)
}

// Translated from 008356e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first station reference in range of `reference` (the player when
/// it is 0) whose form is `form` (cdecl), from a temporary reference list
/// (`BGSTalkingActivator::GetRadioStationsInRangeOfRef`); the walk stops
/// at an empty item or one without a form. 0 when none.
// Translation incomplete: 008356e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary list is not translated
pub fn fn_008356e0(e: &mut Engine, form: u32, reference: u32) -> u32 {
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCTOR, &args![list]);
        let center = if reference == 0 {
            e.global::<u32>(PLAYER)
        } else {
            reference
        };
        e.call(STATIONS_IN_RANGE, &args![center, list, 0u32]);
        let mut found = 0u32;
        let mut done = false;
        let mut node = list.addr();
        while !done && node != 0 {
            let candidate = item_of(e, node);
            if candidate == 0 {
                break;
            }
            let candidate = item_of(e, node);
            if e.call(FORM_OF, &args![candidate]).u32() == 0 {
                break;
            }
            let candidate = item_of(e, node);
            if form == e.call(FORM_OF, &args![candidate]).u32() {
                found = item_of(e, node);
                done = true;
            }
            node = next_of(e, node);
        }
        e.call(LIST_DESTRUCTOR, &args![list]);
        found
    })
}

// Translated from 008357e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at +0x10 of `object` to 1 unless `skip` is set or the
/// object is null (cdecl).
pub fn fn_008357e0(e: &mut Engine, object: u32, skip: u8) {
    if skip == 0 && object != 0 {
        e.mem.set_u32(object + 0x10, 1);
    }
}

// Translated from 00835980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::DisableNPCRadio` (Xbox PDB name from the call graph,
/// cdecl): for an actor with a receiver, stops and releases its sound
/// handle and clears its playing flag and radio.
pub fn fallout_radio_disable_npc_radio(e: &mut Engine, actor: u32) {
    if !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    let receiver = fallout_radio_get_active_receiver(e, actor);
    if receiver == 0 {
        return;
    }
    stop_and_release(e, receiver + 4);
    let receiver = Ptr::<FORadioReceiver>::new(receiver);
    e.set(receiver, FORadioReceiver::bShouldBePlaying, 0);
    e.set(receiver, FORadioReceiver::pRadio, 0);
}

// Translated from 00835f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a receiver: its radio's form id and its four flag bytes.
pub fn fn_00835f10(e: &mut Engine, this: Ptr<FORadioReceiver>, buffer: u32) {
    let radio = e.get(this, FORadioReceiver::pRadio);
    e.call(SAVE_FORM_ID, &args![buffer, radio, 0u32]);
    for offset in [0x1cu32, 0x1d, 0x1e, 0x1f] {
        e.call(SAVE_DATA, &args![buffer, this.addr() + offset, 1u32, 0u32]);
    }
}

// Translated from 00836030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StationState` destructor: deletes the next conversation (when it
/// is not the current one) and the current one, clears both and destroys
/// the users list.
// Translation incomplete: 00836030 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_00836030(e: &mut Engine, this: Ptr<StationState>) {
    if e.get(this, StationState::pNextConvo) != e.get(this, StationState::pConvo) {
        let next = e.get(this, StationState::pNextConvo);
        delete_conversation(e, next);
        e.set(this, StationState::pNextConvo, 0);
    }
    e.set(this, StationState::pNextConvo, 0);
    if e.get(this, StationState::pConvo) != 0 {
        let current = e.get(this, StationState::pConvo);
        delete_conversation(e, current);
        e.set(this, StationState::pConvo, 0);
    }
    e.set(this, StationState::pConvo, 0);
    e.call(LIST_DESTRUCTOR, &args![this.byte_add(0x18)]);
}

// Translated from 00836110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a station state: the time into the current item (0 without a
/// duration), the duration, both powers and the flags, the users' radio
/// form ids (a sized value with their count), whether there is a
/// conversation and then the conversation.
pub fn fn_00836110(e: &mut Engine, this: Ptr<StationState>, buffer: u32) {
    e.with_stack(8, |e, locals| {
        let elapsed = locals.addr();
        let has_conversation = locals.addr() + 4;
        let now = audio_time(e);
        let start = e.get(this, StationState::iCurrentItemStartTime);
        e.mem.set_u32(elapsed, now.wrapping_sub(start));
        if e.get(this, StationState::iCurrentItemDuration) <= 0 {
            e.mem.set_u32(elapsed, 0);
        }
        e.call(SAVE_DATA, &args![buffer, elapsed, 4u32, 0u32]);
        for (offset, size) in [(0xcu32, 4u32), (0x10, 1), (0x11, 1), (0x14, 4)] {
            e.call(SAVE_DATA, &args![buffer, this.addr() + offset, size, 0u32]);
        }
        let mut count = 0u32;
        let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
        let mut node = this.addr() + 0x18;
        while node != 0 {
            let receiver = item_of(e, node);
            if receiver != 0 {
                e.call(SAVE_FORM_ID, &args![buffer, e.mem.u32(receiver), 0u32]);
                count = count.wrapping_add(1);
            }
            node = next_of(e, node);
        }
        e.call(SAVE_SIZED, &args![buffer, count, mark]);
        let has = u8::from(e.get(this, StationState::pConvo) != 0);
        e.mem.set_u8(has_conversation, has);
        e.call(SAVE_DATA, &args![buffer, has_conversation, 1u32, 0u32]);
        if has != 0 {
            let conversation = e.get(this, StationState::pConvo);
            e.call(CONVERSATION_SAVE, &args![conversation, buffer]);
        }
    });
}

// Translated from 00836600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a station: its reference's form id, then its state.
pub fn fn_00836600(e: &mut Engine, this: Ptr, buffer: u32) {
    let reference = e.mem.u32(this.addr());
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
    fn_00836110(e, Ptr::new(this.addr() + STATION_STATE), buffer);
}

// Translated from 008366e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio`'s save (cdecl): the saved word, byte and string, the
/// receivers and the stations (each a sized value with its count), the
/// forms of the current and the lost Pip-Boy stations, and the reported
/// stations.
pub fn fn_008366e0(e: &mut Engine, buffer: u32) {
    e.call(SAVE_DATA, &args![buffer, SAVED_WORD, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, SAVED_BYTE, 1u32, 0u32]);
    e.call(SAVE_STRING, &args![buffer, SAVED_STRING, 0u32]);
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = RECEIVER_LIST;
    while node != 0 {
        let receiver = item_of(e, node);
        if receiver != 0 {
            fn_00835f10(e, Ptr::new(receiver), buffer);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = STATION_LIST;
    while node != 0 {
        let station = item_of(e, node);
        if station != 0 {
            fn_00836600(e, Ptr::new(station), buffer);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
    for global in [CURRENT_PIPBOY_STATION, LOST_PIPBOY_STATION] {
        let station = e.global::<u32>(global);
        let form = if station != 0 {
            e.call(FORM_OF, &args![e.mem.u32(station)]).u32()
        } else {
            0
        };
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    let mut count = 0u32;
    let mark = e.call(SAVE_START_SIZED, &args![buffer]).u32();
    let mut node = DISCOVERED_LIST;
    while node != 0 {
        let reference = item_of(e, node);
        if reference != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
            count = count.wrapping_add(1);
        }
        node = next_of(e, node);
    }
    e.call(SAVE_SIZED, &args![buffer, count, mark]);
}

// Translated from 00837060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutRadio::GetNextConversation` (name used by the decompiler at its
/// call site, cdecl): with a next conversation queued, deletes the current
/// one, moves the next one in, deletes it through its deleting destructor
/// as the code does, clears the next slot and queues a `RadioConvTask`
/// (0x38 bytes, `00837250`) on the task queue (`01202d98`, slot `+0x48`),
/// answering true; otherwise deletes the current conversation and creates
/// a new one (0xc bytes, `0083b850`) on the station's reference with
/// `topic` (topic `7` from `TESTopic::GetTopic` when it is 0), answering
/// false.
// Translation incomplete: 00837060 - the C++ exception-unwinding frame (FS:[0]) that frees the new objects is not translated
pub fn fn_00837060(e: &mut Engine, station: u32, topic: u32) -> bool {
    let topic = if topic == 0 {
        e.call(GET_TOPIC, &args![7u32, 0u32]).u32()
    } else {
        topic
    };
    // The station's words: +4 the current conversation (`pConvo` of its
    // state), +8 the next one.
    if e.mem.u32(station + 8) != 0 {
        if e.mem.u32(station + 4) != 0 {
            let old = e.mem.u32(station + 4);
            delete_conversation(e, old);
            e.mem.set_u32(station + 4, 0);
        }
        let next = e.mem.u32(station + 8);
        e.mem.set_u32(station + 4, next);
        let next = e.mem.u32(station + 8);
        delete_conversation(e, next);
        e.mem.set_u32(station + 8, 0);
        let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
        let task = if block != 0 {
            e.call(RADIO_TASK_CONSTRUCT, &args![block, station]).u32()
        } else {
            0
        };
        let queue = e.global::<u32>(TASK_QUEUE);
        e.vcall(queue, 0x48, &args![task]);
        return true;
    }
    if e.mem.u32(station + 4) != 0 {
        let old = e.mem.u32(station + 4);
        delete_conversation(e, old);
        e.mem.set_u32(station + 4, 0);
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let reference = e.mem.u32(station);
        e.call(
            CONVERSATION_CONSTRUCT,
            &args![block, reference, reference, topic],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(station + 4, conversation);
    false
}

// Translated from 008373d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a list of floats (first node embedded): item 0.0,
/// no next node. Returns `this`.
pub fn fn_008373d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// The byte `008325a0` stores (`011dd313`; `falloutaudiomedia.rs` reads it as the media
/// hold byte) and the byte `00833ce0` answers (`011dd437`).
const HOLD_FLAG_011DD313: u32 = 0x011d_d313;
const STORED_FLAG_011DD437: u32 = 0x011d_d437;

// Translated from 008325a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores its low byte at `011dd313`.
pub fn fn_008325a0(e: &mut Engine, value: u8) {
    e.set_global(HOLD_FLAG_011DD313, value);
}

// Translated from 00832490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x100` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00832490(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x100, on);
}

// Translated from 00833c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x80` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833c00(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x80)
}

// Translated from 00833ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dd437`, in `AL`.
pub fn fn_00833ce0(e: &mut Engine) -> u8 {
    e.global(STORED_FLAG_011DD437)
}

// Translated from 00835670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `8` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00835670(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 8)
}

// Translated from 00835690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x80` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00835690(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x80, on);
}

// Translated from 00833bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `4` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833bc0(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 4)
}

// Translated from 00833be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x10` of `iStationFlags` is set (`EAX` exactly 0 or 1).
pub fn fn_00833be0(e: &mut Engine, this: Ptr<StationState>) -> bool {
    station_flag(e, this, 0x10)
}

// Translated from 00833c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears flag `0x40` of `iStationFlags` (`thiscall`, `RET 4`).
pub fn fn_00833c90(e: &mut Engine, this: Ptr<StationState>, on: u8) {
    set_station_flag(e, this, 0x40, on);
}

/// The byte [`fn_00833cf0`] answers (`011dcfa4`; the dialogue menu sets and clears it).
const DIALOGUE_FLAG_011DCFA4: u32 = 0x011d_cfa4;

// Translated from 00833cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dcfa4`, in `AL`.
pub fn fn_00833cf0(e: &mut Engine) -> u8 {
    e.global(DIALOGUE_FLAG_011DCFA4)
}

/// The task base constructor `RadioConvTask`'s constructor runs (`00c3ce60`, unit
/// `queuedfiles.obj`, unnamed; `thiscall`, one word) and `RadioConvTask`'s vtable
/// (`0107cf18`, RTTI `.?AVRadioConvTask@@`).
const TASK_BASE_CONSTRUCT: u32 = 0x00c3_ce60;
const RADIO_CONV_TASK_VTABLE: u32 = 0x0107_cf18;

// Translated from 00837250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadioConvTask`'s constructor (the engine map gives no name; the vtable's RTTI names the
/// class; `thiscall`, `RET 4`): the base (`00c3ce60` with 0), the vtable, the station at
/// +0x30. Returns `this`.
pub fn fn_00837250(e: &mut Engine, this: Ptr, station: u32) -> Ptr {
    e.call(TASK_BASE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), RADIO_CONV_TASK_VTABLE);
    e.mem.set_u32(this.addr() + 0x30, station);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_25a0, fn_008325a0(u8)),
        entry!(0x0083_2490, fn_00832490(Ptr<StationState>, u8)),
        entry!(0x0083_3c00, fn_00833c00(Ptr<StationState>) -> bool),
        entry!(0x0083_3ce0, fn_00833ce0() -> u8),
        entry!(0x0083_3bc0, fn_00833bc0(Ptr<StationState>) -> bool),
        entry!(0x0083_3be0, fn_00833be0(Ptr<StationState>) -> bool),
        entry!(0x0083_3c90, fn_00833c90(Ptr<StationState>, u8)),
        entry!(0x0083_3cf0, fn_00833cf0() -> u8),
        entry!(0x0083_7250, fn_00837250(Ptr, u32) -> Ptr),
        entry!(0x0083_5670, fn_00835670(Ptr<StationState>) -> bool),
        entry!(0x0083_5690, fn_00835690(Ptr<StationState>, u8)),
        entry!(0x0083_72c0, fn_008372c0(Ptr)),
        entry!(0x0083_21c0, fn_008321c0(Ptr)),
        entry!(0x0083_2010, fallout_radio_shutdown()),
        entry!(0x0083_2160, fn_00832160(Ptr, u32) -> Ptr),
        entry!(
            0x0083_2190,
            fn_00832190(Ptr<FORadioReceiver>, u32) -> Ptr<FORadioReceiver>
        ),
        entry!(0x0083_21e0, fn_008321e0(Ptr<FORadioReceiver>)),
        entry!(0x0083_2420, fn_00832420(Ptr<StationState>) -> bool),
        entry!(0x0083_2440, fn_00832440(Ptr<StationState>, u8)),
        entry!(0x0083_25b0, fn_008325b0(u32, u8)),
        entry!(
            0x0083_27d0,
            fn_008327d0(Ptr<FORadioReceiver>) -> Ptr<FORadioReceiver>
        ),
        entry!(0x0083_2830, fn_00832830(u32) -> u32),
        entry!(0x0083_2930, fallout_radio_get_active_receiver(u32) -> u32),
        entry!(0x0083_29a0, fallout_radio_clear_inactive_receivers(u8)),
        entry!(0x0083_2ad0, fallout_radio_update(u8) -> bool),
        entry!(0x0083_2cb0, fn_00832cb0(u32, u32) -> u32),
        entry!(0x0083_3040, fn_00833040(Ptr<StationState>) -> bool),
        entry!(0x0083_3060, fn_00833060(Ptr<StationState>, u8)),
        entry!(0x0083_30b0, fn_008330b0(Ptr<StationState>, u8)),
        entry!(0x0083_3100, fn_00833100(Ptr<StationState>, u8)),
        entry!(0x0083_3150, fn_00833150(Ptr) -> Ptr),
        entry!(
            0x0083_3170,
            fn_00833170(Ptr<StationState>) -> Ptr<StationState>
        ),
        entry!(0x0083_3c20, fn_00833c20(Ptr<StationState>) -> u32),
        entry!(0x0083_3c40, fn_00833c40(Ptr<StationState>, u8)),
        entry!(0x0083_3d00, fallout_radio_update_stations(u8)),
        entry!(0x0083_4210, fn_00834210(Ptr<StationState>, u8)),
        entry!(0x0083_4260, fn_00834260(u32)),
        entry!(0x0083_5650, fn_00835650(Ptr<StationState>) -> bool),
        entry!(0x0083_56e0, fn_008356e0(u32, u32) -> u32),
        entry!(0x0083_57e0, fn_008357e0(u32, u8)),
        entry!(0x0083_5980, fallout_radio_disable_npc_radio(u32)),
        entry!(0x0083_5f10, fn_00835f10(Ptr<FORadioReceiver>, u32)),
        entry!(0x0083_6030, fn_00836030(Ptr<StationState>)),
        entry!(0x0083_6110, fn_00836110(Ptr<StationState>, u32)),
        entry!(0x0083_6600, fn_00836600(Ptr, u32)),
        entry!(0x0083_66e0, fn_008366e0(u32)),
        entry!(0x0083_7060, fn_00837060(u32, u32) -> bool),
        entry!(0x0083_73d0, fn_008373d0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_008321c0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00836030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_21c0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00836030).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }

    #[test]
    fn fn_008372c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_cea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_72c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_cf18);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_cf18,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b070_tests {
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

    /// The list node helpers as the exe has them: the item's address is
    /// the node, the next node is at +4, a node is empty when both words
    /// are 0.
    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(NODE_EMPTY, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0),
            ..Ret::default()
        });
    }

    /// Maps the radio's statics.
    fn globals(e: &mut Engine) {
        for page in [
            0x011d_d000u32,
            0x011a_1000,
            0x011d_e000,
            0x0101_6000,
            0x011c_3000,
            0x0104_f000,
        ] {
            e.map(page, 0x1000);
        }
    }

    /// Writes a list into the embedded first node at `head` (more nodes
    /// allocated).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    /// An object whose vtable slots are `base + slot`.
    fn object_with_vtable(e: &mut Engine, size: u32, base: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        object
    }

    #[test]
    fn station_flags() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        for (setter, mask) in [
            (0x0083_2440u32, 1u32),
            (0x0083_3060, 2),
            (0x0083_4210, 4),
            (0x0083_3c40, 8),
            (0x0083_30b0, 0x10),
            (0x0083_3100, 0x20),
        ] {
            e.call(setter, &args![state, 1u32]);
            assert_eq!(e.get(state, StationState::iStationFlags) & mask, mask);
        }
        assert!(e.call(0x0083_5650, &args![state]).bool());
        assert!(e.call(0x0083_3040, &args![state]).bool());
        assert!(!e.call(0x0083_2420, &args![state]).bool());
        e.set(state, StationState::iStationFlags, 0x13f);
        assert!(e.call(0x0083_2420, &args![state]).bool());
        e.call(0x0083_3100, &args![state, 0u32]);
        e.call(0x0083_2440, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x11e);
        assert!(!e.call(0x0083_5650, &args![state]).bool());
    }

    #[test]
    fn station_power_byte_keeps_the_upper_bytes_of_this() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::cStationPower, 0x42);
        let result = e.call(0x0083_3c20, &args![state]).u32();
        assert_eq!(result, (state.addr() & !0xff) | 0x42);
    }

    #[test]
    fn station_and_state_constructors() {
        let mut e = Engine::new();
        stub(&mut e, &[LIST_CONSTRUCTOR]);
        let station = e.mem.alloc(0x24);
        e.mem.write(station, &[0xcd; 0x24]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_3150, &args![station]).u32(), station);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::pConvo), 0);
        assert_eq!(e.get(state, StationState::iStationFlags), 0);
        assert_eq!(
            e.get(state, StationState::cStationPower),
            0xcd,
            "left as is"
        );
        assert_eq!(calls_to(&e, LIST_CONSTRUCTOR), vec![vec![station + 0x1c]]);
        let list = e.mem.alloc(8);
        e.mem.write(list, &[0xcd; 8]);
        assert_eq!(e.call(0x0083_73d0, &args![list]).u32(), list);
        assert_eq!(e.mem.u64(list), 0);
    }

    #[test]
    fn receiver_construction_and_deletion() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
                0x0083_6030,
            ],
        );
        let receiver = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_27d0, &args![receiver]).u32(), receiver);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_CONSTRUCT),
            vec![vec![receiver + 4], vec![receiver + 0x10]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_2190, &args![receiver, 1u32]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTRUCTOR),
            vec![vec![receiver + 0x10], vec![receiver + 4]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![receiver]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_2160, &args![receiver, 0u32]);
        assert_eq!(calls_to(&e, 0x0083_6030), vec![vec![receiver + 4]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn active_receiver_lookup() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first, 0x5001);
        e.mem.set_u32(second, 0x5002);
        fill(&mut e, RECEIVER_LIST, &[first, second]);
        assert_eq!(e.call(0x0083_2930, &args![0x5002u32]).u32(), second);
        assert_eq!(e.call(0x0083_2930, &args![0x5003u32]).u32(), 0);
    }

    #[test]
    fn station_lookup_prefers_the_enabled_one() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let first = e.mem.alloc(0x24);
        let second = e.mem.alloc(0x24);
        e.mem.set_u32(first, 0x5001);
        e.mem.set_u32(second, 0x5002);
        fill(&mut e, STATION_LIST, &[first, second]);
        answer(&mut e, FORM_OF, 0x6000);
        e.register(REFERENCE_TEST_00440DA0, |_, a| Ret {
            eax: u32::from(a[0] == 0x5001),
            ..Ret::default()
        });
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        assert_eq!(e.call(0x0083_2830, &args![0x6000u32]).u32(), second);
        assert_eq!(e.call(0x0083_2830, &args![0x6001u32]).u32(), 0);
        e.set_global(SHUTTING_DOWN, 1u8);
        assert_eq!(e.call(0x0083_2830, &args![0x6000u32]).u32(), 0);
    }

    #[test]
    fn clearing_removes_silent_receivers_from_the_stations() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let silent = e.mem.alloc(0x20);
        let playing = e.mem.alloc(0x20);
        e.mem.set_u8(playing + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[playing, silent]);
        let station = e.mem.alloc(0x24);
        fill(&mut e, STATION_LIST, &[station]);
        // The receiver list's removal drops the item it is given.
        e.register(LIST_REMOVE, |e, a| {
            if a[0] == RECEIVER_LIST {
                let node = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node, 0);
                e.mem.set_u32(a[0] + 4, 0);
            }
            Ret::default()
        });
        let mut contained = 1;
        e.register_double(LIST_CONTAINS, move |_, _| {
            let answer = contained;
            contained = 0;
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_29a0, &args![0u32]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_STOP), vec![vec![silent + 4]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![silent]]);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 2);
    }

    #[test]
    fn shutdown_deletes_stations_and_receivers() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station + 4, 0x7000);
        fill(&mut e, STATION_LIST, &[station]);
        stub(
            &mut e,
            &[
                CONVERSATION_DELETE,
                0x0083_6030,
                OPERATOR_DELETE,
                LIST_CLEAR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_2010, &args![]);
        assert_eq!(e.global::<u8>(SHUTTING_DOWN), 1);
        assert_eq!(calls_to(&e, CONVERSATION_DELETE), vec![vec![0x7000, 1]]);
        assert_eq!(e.mem.u32(station + 4), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![station]]);
        assert_eq!(
            calls_to(&e, LIST_CLEAR),
            vec![
                vec![STATION_LIST],
                vec![RECEIVER_LIST],
                vec![DISCOVERED_LIST]
            ]
        );
    }

    #[test]
    fn turning_a_radio_on_creates_its_receiver() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x15);
        answer(&mut e, RADIO_STATION_FORM, 0x6000);
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        let block = e.mem.alloc(0x20);
        answer(&mut e, OPERATOR_NEW, block);
        stub(&mut e, &[SOUND_HANDLE_CONSTRUCT, LIST_ADD_HEAD]);
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 1u32]);
        let receiver = Ptr::<FORadioReceiver>::new(block);
        assert_eq!(e.get(receiver, FORadioReceiver::pRadio), 0x5100);
        assert_eq!(e.get(receiver, FORadioReceiver::bShouldBePlaying), 1);
        assert_eq!(e.get(receiver, FORadioReceiver::bJustStarted), 1);
        let adds = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(adds[0][0], RECEIVER_LIST);
        assert_eq!(adds[1][0], station + 0x1c);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::iStationFlags), 1);
    }

    #[test]
    fn turning_a_radio_off_stops_it() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        // Removing from the receiver list empties it.
        e.register(LIST_REMOVE, |e, a| {
            if a[0] == RECEIVER_LIST {
                e.mem.set_u32(RECEIVER_LIST, 0);
                e.mem.set_u32(RECEIVER_LIST + 4, 0);
            }
            Ret::default()
        });
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, 0x5100);
        e.mem.set_u8(receiver + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x15);
        answer(&mut e, RADIO_STATION_FORM, 0x6000);
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                LIST_CONTAINS,
                SOUND_HANDLE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 0u32]);
        assert_eq!(e.mem.u8(receiver + 0x1c), 0);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_STOP)[..2],
            [vec![receiver + 4], vec![receiver + 0x10]]
        );
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0083_25b0, &args![0x5100u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn creating_a_station_for_a_talker() {
        let mut e = Engine::new();
        globals(&mut e);
        let block = e.mem.alloc(0x24);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, FORM_TYPE, 0x16);
        let form = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        answer(&mut e, FORM_OF, form);
        answer(&mut e, 0x00c1_00b4, 1);
        answer(&mut e, AUDIO_INSTANCE, 0x900);
        answer(&mut e, AUDIO_TIME, 50_000);
        answer(&mut e, RANDOM_RANGE, 1000);
        stub(&mut e, &[LIST_CONSTRUCTOR, LIST_ADD_HEAD]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_2cb0, &args![0x5000u32, 0u32]).u32(), block);
        let state = Ptr::<StationState>::new(block + 4);
        assert_eq!(e.mem.u32(block), 0x5000);
        assert_eq!(e.get(state, StationState::iCurrentItemStartTime), 49_000);
        assert_eq!(e.get(state, StationState::iCurrentItemDuration), 1);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x32);
        assert_eq!(e.get(state, StationState::cStationPower), 100);
        assert_eq!(calls_to(&e, RANDOM_RANGE), vec![vec![0, 0x7530]]);
        assert_eq!(calls_to(&e, LIST_ADD_HEAD)[0][0], STATION_LIST);
        assert_eq!(e.call(0x0083_2cb0, &args![0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn creating_a_station_without_a_kind_deletes_the_entry() {
        let mut e = Engine::new();
        globals(&mut e);
        let block = e.mem.alloc(0x24);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, FORM_OF, 0x6000);
        answer(&mut e, FORM_TYPE, 0x10);
        stub(&mut e, &[LIST_CONSTRUCTOR, 0x0083_6030, OPERATOR_DELETE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_2cb0, &args![0x5000u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![block]]);
    }

    #[test]
    fn update_applies_the_queued_state_and_updates_the_stations() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.set_global(QUEUED_STATE, 2u32);
        e.set_global(RADIO_STATE, 1u32);
        e.set_global(LAST_UPDATE, 100u32);
        answer(&mut e, TIMER_TIME, 400);
        let interval = e.mem.alloc(4);
        e.mem.set_u32(interval, 200);
        answer(&mut e, SETTING_VALUE_ADDRESS, interval);
        answer(&mut e, LOADING_MENU_VISIBLE, 0);
        answer(&mut e, GET_SOUND, 0x7700);
        stub(
            &mut e,
            &[LIST_CONSTRUCTOR, FLOAT_LIST_DESTRUCTOR, LIST_DESTRUCTOR],
        );
        e.set_global(SHUTTING_DOWN, 1u8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_2ad0, &args![0u32]).bool());
        assert_eq!(e.global::<u32>(LAST_UPDATE), 400);
        assert_eq!(e.global::<u32>(QUEUED_STATE), 0);
        assert_eq!(e.global::<u32>(RADIO_STATE), 0, "cleared at the end");
        assert_eq!(e.global::<u32>(ATTENUATION_SOUND), 0x7700);
        assert_eq!(calls_to(&e, LIST_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn update_stations_computes_the_power_of_a_station_in_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        // Not a station with users: 00834260 returns at once.
        fill(&mut e, STATION_LIST, &[station]);
        let signal = e.mem.alloc(4);
        e.mem.set_f32(signal, 0.5);
        answer(&mut e, SETTING_FLOAT_ADDRESS, signal);
        e.register(STATIONS_IN_RANGE, |e, a| {
            e.mem.set_u32(a[1], 0x5000);
            e.mem.set_f32(a[2], 75.0);
            Ret::default()
        });
        answer(&mut e, REFERENCE_TEST_00440DA0, 0);
        answer(&mut e, LIST_CONTAINS, 1);
        e.register(STATION_VALUE_0056B190, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.register(STATION_VALUE_0056B1D0, |_, _| Ret::default());
        answer(&mut e, REFERENCE_VALUE_0056B210, 0);
        stub(
            &mut e,
            &[
                LIST_CONSTRUCTOR,
                STATE_SET_POWER,
                FLOAT_LIST_CLEAR,
                LIST_CLEAR,
                FLOAT_LIST_DESTRUCTOR,
                LIST_DESTRUCTOR,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_3d00, &args![1u32]);
        // Ratio 0.75 over the falloff 0.5: (0.25 / 0.5) * 100 = 50 lost.
        assert_eq!(calls_to(&e, STATE_SET_POWER), vec![vec![station + 4, 50]]);
        assert_eq!(e.global::<u8>(SKIP_DISCOVERY_MESSAGE), 0);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn update_stations_cuts_a_station_out_of_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        fill(&mut e, STATION_LIST, &[station]);
        let signal = e.mem.alloc(4);
        answer(&mut e, SETTING_FLOAT_ADDRESS, signal);
        stub(
            &mut e,
            &[
                LIST_CONSTRUCTOR,
                STATIONS_IN_RANGE,
                STATE_SET_POWER,
                FLOAT_LIST_CLEAR,
                LIST_CLEAR,
                FLOAT_LIST_DESTRUCTOR,
                LIST_DESTRUCTOR,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                REFERENCE_TEST_00440DA0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_3d00, &args![1u32]);
        assert_eq!(calls_to(&e, STATE_SET_POWER), vec![vec![station + 4, 0]]);
        let state = Ptr::<StationState>::new(station + 4);
        assert_eq!(e.get(state, StationState::iStationFlags), 4);
    }

    #[test]
    fn update_station_pauses_its_users() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.map(0x0104_f000, 0x1000);
        e.set_global(RADIO_STATE, 1u32);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        let receiver = e.mem.alloc(0x20);
        fill(&mut e, station + 0x1c, &[receiver]);
        answer(&mut e, SOUND_IS_VALID, 1);
        stub(
            &mut e,
            &[
                REFERENCE_TEST_00440DA0,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                FORM_OF,
                RTTI_CAST,
                SOUND_PAUSE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![station]);
        assert_eq!(
            calls_to(&e, SOUND_PAUSE),
            vec![vec![receiver + 4], vec![receiver + 0x10]]
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![RADIO_CRITICAL_SECTION, UPDATE_STATION_NAME]]
        );
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn update_station_starts_the_voice_file_of_a_user_in_range() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        for page in [0x0104_f000u32, 0x011f_4000, 0x0101_7000, 0x0106_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_RANGE, 1000.0f32.to_bits());
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        let state = Ptr::<StationState>::new(station + 4);
        e.set(state, StationState::iCurrentItemStartTime, 1000);
        e.set(state, StationState::iCurrentItemDuration, 10);
        e.set(state, StationState::cStationPower, 100);
        e.set(state, StationState::iStationFlags, 1);
        let radio = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        let at = e.mem.alloc(12);
        e.mem.set_f32(at, 3.0);
        answer(&mut e, 0x00c1_01f4, at);
        answer(&mut e, 0x00c1_0100, 0);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, radio);
        e.mem.set_u8(receiver + 0x1c, 1);
        e.mem.set_u8(receiver + 0x1d, 1);
        fill(&mut e, station + 0x1c, &[receiver]);
        answer(&mut e, AUDIO_TIME, 1005);
        // The conversation's current response is 0x7100, whose own current
        // item 0x7200 has a voice form.
        e.register(CURRENT_RESPONSE, |_, a| Ret {
            eax: if a[0] == 0x7000 { 0x7100 } else { 0x7200 },
            ..Ret::default()
        });
        answer(&mut e, FORM_OF, 0x7300);
        answer(&mut e, SOUND_FILE_NAME, 0x7400);
        answer(&mut e, STRING_FIND, 1);
        answer(&mut e, STRING_C_STR, 0);
        answer(&mut e, VECTOR_SUBTRACT, 0x7500);
        e.register(VECTOR_LENGTH, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        answer(&mut e, AUDIO_SOUND_BY_FILE, 0x7600);
        answer(&mut e, STATE_VALUE_00824060, 0);
        stub(
            &mut e,
            &[
                REFERENCE_TEST_00440DA0,
                AUDIO_INSTANCE,
                AUDIO_LISTENER_POSITION,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                RTTI_CAST,
                SOUND_IS_VALID,
                STATE_CALL_00632D20,
                REFERENCE_WORLD_SPACE,
                PLAYER_OBJECT,
                PLAYER_OBJECT_TEST,
                COPY_BOUNDED,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCTOR,
                SOUND_SET_COMPLETION,
                SOUND_SET_SYNCH_START,
                SOUND_SET_POSITION,
                SOUND_SET_VOLUME,
                SOUND_HANDLE_PLAY,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0083_4260, &args![station]);
        let by_file = calls_to(&e, AUDIO_SOUND_BY_FILE);
        assert_eq!(by_file.len(), 1);
        assert_eq!(by_file[0][3], 0x0050_0102);
        assert_eq!(by_file[0][4], 0, "no attenuation sound");
        assert_eq!(
            calls_to(&e, SOUND_SET_COMPLETION),
            vec![vec![receiver + 4, COMPLETION_CALLBACK, station]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_SYNCH_START),
            vec![vec![receiver + 4, 1000]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_VOLUME),
            vec![vec![receiver + 4, 1.0f32.to_bits()]]
        );
        let positions = calls_to(&e, SOUND_SET_POSITION);
        assert_eq!(positions[0][1], 3.0f32.to_bits());
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![receiver + 4, 0]]);
        assert_eq!(e.mem.u8(receiver + 0x1d), 0, "no longer just started");
        assert_eq!(e.get(state, StationState::iStationFlags), 9);
    }

    #[test]
    fn stations_in_range_lookup() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        e.set_global(PLAYER, 0x3000u32);
        e.register(STATIONS_IN_RANGE, |e, a| {
            e.mem.set_u32(a[1], 0x5001);
            let next = e.mem.alloc(8);
            e.mem.set_u32(a[1] + 4, next);
            e.mem.set_u32(next, 0x5002);
            Ret::default()
        });
        e.register(FORM_OF, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        stub(&mut e, &[LIST_CONSTRUCTOR, LIST_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_56e0, &args![0x6002u32, 0u32]).u32(), 0x5002);
        assert_eq!(calls_to(&e, STATIONS_IN_RANGE)[0][0], 0x3000);
        assert_eq!(e.call(0x0083_56e0, &args![0x6009u32, 0x4000u32]).u32(), 0);
    }

    #[test]
    fn completion_callback_marks_the_item_over() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        e.call(0x0083_57e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        e.call(0x0083_57e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x10), 1);
        e.call(0x0083_57e0, &args![0u32, 0u32]);
    }

    #[test]
    fn disabling_an_npc_radio() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        let actor = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        answer(&mut e, 0x00c1_0100, 1);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, actor);
        e.mem.set_u8(receiver + 0x1c, 1);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        stub(&mut e, &[SOUND_HANDLE_STOP, SOUND_HANDLE_RELEASE]);
        e.call(0x0083_5980, &args![actor]);
        assert_eq!(e.mem.u32(receiver), 0);
        assert_eq!(e.mem.u8(receiver + 0x1c), 0);
    }

    #[test]
    fn saving_receivers_states_and_the_radio() {
        let mut e = Engine::new();
        globals(&mut e);
        lists(&mut e);
        stub(
            &mut e,
            &[
                SAVE_FORM_ID,
                SAVE_DATA,
                SAVE_STRING,
                SAVE_SIZED,
                CONVERSATION_SAVE,
            ],
        );
        answer(&mut e, SAVE_START_SIZED, 0x99);
        answer(&mut e, AUDIO_INSTANCE, 0x900);
        answer(&mut e, AUDIO_TIME, 1500);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(receiver, 0x5100);
        fill(&mut e, RECEIVER_LIST, &[receiver]);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        e.mem.set_u32(station + 0xc, 1000);
        e.mem.set_i32(station + 0x10, 5);
        fill(&mut e, station + 0x1c, &[receiver]);
        fill(&mut e, STATION_LIST, &[station]);
        fill(&mut e, DISCOVERED_LIST, &[0x5200]);
        e.set_global(CURRENT_PIPBOY_STATION, station);
        answer(&mut e, FORM_OF, 0x6000);
        e.register(SAVE_DATA, |e, a| {
            if a[2] == 4 && a[1] != SAVED_WORD && e.mem.u32(a[1]) == 500 {
                e.mem.set_u32(a[1], 0xe1a5);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_66e0, &args![0x8000u32]);
        let forms = calls_to(&e, SAVE_FORM_ID);
        assert_eq!(
            forms,
            vec![
                vec![0x8000, 0x5100, 0],
                vec![0x8000, 0x5000, 0],
                vec![0x8000, 0x5100, 0],
                vec![0x8000, 0x6000, 0],
                vec![0x8000, 0, 0],
                vec![0x8000, 0x5200, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_SIZED),
            vec![
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99],
                vec![0x8000, 1, 0x99]
            ]
        );
        assert_eq!(calls_to(&e, CONVERSATION_SAVE), vec![vec![0x7000, 0x8000]]);
        let data = calls_to(&e, SAVE_DATA);
        assert_eq!(data[0], vec![0x8000, SAVED_WORD, 4, 0]);
        // The receiver's four flag bytes, then the elapsed time 500.
        assert_eq!(data[2][1], receiver + 0x1c);
        assert_eq!(data[6][2], 4, "the elapsed time");
    }

    #[test]
    fn state_destructor_deletes_both_conversations() {
        let mut e = Engine::new();
        stub(&mut e, &[CONVERSATION_DELETE, LIST_DESTRUCTOR]);
        let state = e.new_object::<StationState>();
        e.set(state, StationState::pConvo, 0x7000);
        e.set(state, StationState::pNextConvo, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x0083_6030, &args![state]);
        assert_eq!(
            calls_to(&e, CONVERSATION_DELETE),
            vec![vec![0x7100, 1], vec![0x7000, 1]]
        );
        assert_eq!(e.get(state, StationState::pConvo), 0);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCTOR),
            vec![vec![state.addr() + 0x18]]
        );
        e.set(state, StationState::pConvo, 0x7000);
        e.set(state, StationState::pNextConvo, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0083_6030, &args![state]);
        assert_eq!(calls_to(&e, CONVERSATION_DELETE), vec![vec![0x7000, 1]]);
    }

    #[test]
    fn next_conversation_queues_a_task_or_creates_one() {
        let mut e = Engine::new();
        globals(&mut e);
        e.map(TASK_QUEUE & !0xfff, 0x1000);
        let queue = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.set_global(TASK_QUEUE, queue);
        stub(&mut e, &[CONVERSATION_DELETE, 0x00c1_0048]);
        answer(&mut e, GET_TOPIC, 0x6100);
        let block = e.mem.alloc(0x38);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, RADIO_TASK_CONSTRUCT, block);
        let station = e.mem.alloc(0x24);
        e.mem.set_u32(station, 0x5000);
        e.mem.set_u32(station + 4, 0x7000);
        e.mem.set_u32(station + 8, 0x7100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0083_7060, &args![station, 0u32]).bool());
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![7, 0]]);
        assert_eq!(
            calls_to(&e, CONVERSATION_DELETE),
            vec![vec![0x7000, 1], vec![0x7100, 1]]
        );
        assert_eq!(e.mem.u32(station + 4), 0x7100);
        assert_eq!(e.mem.u32(station + 8), 0);
        assert_eq!(calls_to(&e, 0x00c1_0048), vec![vec![queue, block]]);
        answer(&mut e, CONVERSATION_CONSTRUCT, 0x7200);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0083_7060, &args![station, 0x6200u32]).bool());
        assert_eq!(
            calls_to(&e, CONVERSATION_CONSTRUCT),
            vec![vec![block, 0x5000, 0x5000, 0x6200]]
        );
        assert_eq!(e.mem.u32(station + 4), 0x7200);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn the_two_radio_bytes() {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        e.call(0x0083_25a0, &args![0x1234_5607u32]);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313), 7);
        assert_eq!(e.global::<u8>(HOLD_FLAG_011DD313 + 1), 0);
        e.set_global(STORED_FLAG_011DD437, 0x5au8);
        assert_eq!(e.call(0x0083_3ce0, &args![]).u8(), 0x5a);
    }

    #[test]
    fn station_flags_0x100_0x80_and_8() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::iStationFlags, 0x0001_0008);
        // Only the low byte of the flag word counts.
        e.call(0x0083_2490, &args![state, 0x100u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0008);
        e.call(0x0083_2490, &args![state, 1u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0108);
        assert!(!e.call(0x0083_3c00, &args![state]).bool());
        e.call(0x0083_5690, &args![state, 1u32]);
        assert_eq!(e.call(0x0083_3c00, &args![state]).u32(), 1);
        assert_eq!(e.call(0x0083_5670, &args![state]).u32(), 1);
        e.call(0x0083_5690, &args![state, 0u32]);
        e.call(0x0083_2490, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x0001_0008);
        e.set(state, StationState::iStationFlags, !8u32);
        assert_eq!(e.call(0x0083_5670, &args![state]).u32(), 0);
    }

    #[test]
    fn station_flags_4_0x10_and_0x40() {
        let mut e = Engine::new();
        let state = e.new_object::<StationState>();
        e.set(state, StationState::iStationFlags, 0x0001_0004);
        assert_eq!(e.call(0x0083_3bc0, &args![state]).u32(), 1);
        assert_eq!(e.call(0x0083_3be0, &args![state]).u32(), 0);
        e.set(state, StationState::iStationFlags, 0x10);
        assert_eq!(e.call(0x0083_3bc0, &args![state]).u32(), 0);
        assert_eq!(e.call(0x0083_3be0, &args![state]).u32(), 1);
        // Only the low byte of the flag word counts.
        e.call(0x0083_3c90, &args![state, 0x100u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x10);
        e.call(0x0083_3c90, &args![state, 1u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x50);
        e.call(0x0083_3c90, &args![state, 0u32]);
        assert_eq!(e.get(state, StationState::iStationFlags), 0x10);
    }

    #[test]
    fn dialogue_flag_byte() {
        let mut e = Engine::new();
        e.map(DIALOGUE_FLAG_011DCFA4, 4);
        e.set_global(DIALOGUE_FLAG_011DCFA4, 0x0000_0207u32);
        assert_eq!(e.call(0x0083_3cf0, &args![]).u8(), 7);
    }

    #[test]
    fn radio_conv_task_constructor() {
        let mut e = Engine::new();
        e.register(TASK_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_7250, &args![this, 0x5150u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(TASK_BASE_CONSTRUCT, vec![this, 0])]);
        assert_eq!(e.mem.u32(this), RADIO_CONV_TASK_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x5150);
    }
}
