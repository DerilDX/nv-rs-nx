//! `fallout shared/bgstalkingactivator.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSTalkingActivator` on the PC: the Xbox PDB layout less the 0x10 bytes
//! `TESForm` loses (editor id and version-control fields), so the full name
//! is at `+0x30`, the model at `+0x3C`, the scriptable form at `+0x5C`, the
//! destructible form at `+0x68`, `pSoundLoop` at `+0x74`, `pRadioTemplate`
//! at `+0x7C`, `pTempRef` at `+0x90` and `pVoiceType` at `+0x94`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; thiscall(file)).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`tesform.cpp`, thiscall(flag)).
const TES_FORM_SET_FLAG_484AB0: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB): the current chunk's id, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (thiscall on the file): moves to the next chunk; true when
/// there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk's size (`00401660`, thiscall on the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`; thiscall(buffer,
/// size)).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB; thiscall(word address)).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00403570` (thiscall on the file): the 16-bit value the texture swap
/// unpacking takes last.
const FILE_WORD: u32 = 0x0040_3570;
/// The full-name component's loader (`00487050`, cdecl(component, file)).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB; cdecl(model, file)).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB;
/// thiscall(buffer, size, value)).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB; cdecl(component,
/// file)).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `006ecd40` (thiscall(script form id) on the scriptable form; the engine
/// map's name sits on folded code).
const SCRIPTABLE_SET_SCRIPT: u32 = 0x006e_cd40;
/// `TESScriptableForm::InitItem` (Xbox PDB; thiscall(form)).
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_INAM: u32 = 0x4d41_4e49;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_VNAM: u32 = 0x4d41_4e56;

/// The form's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The form's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The form's virtual slot giving its editor id (no words).
const SLOT_GET_EDITOR_ID: u32 = 0x130;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

/// Component offsets on the PC.
const FULL_NAME: u32 = 0x30;
const MODEL: u32 = 0x3c;
const SCRIPTABLE: u32 = 0x5c;
const DESTRUCTIBLE: u32 = 0x68;

/// `004013e0` (thiscall on the form, bool): when true, `InitItem` does
/// nothing.
const FORM_TEST_4013E0: u32 = 0x0040_13e0;
/// `TESObjectACTI::InitItem` (Xbox PDB).
const ACTIVATOR_INIT_ITEM: u32 = 0x0051_0770;
/// `TESForm::GetFile` (Xbox PDB; thiscall(index)).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB; cdecl(form id address, file)).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by id (`004839c0`, cdecl(id)).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (cdecl(object, offset, source type, target type,
/// is reference)).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors of that cast (`TESForm` to `BGSVoiceType`, by
/// the record it resolves; not read from the descriptors here).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_6318;
/// The form's id (`0084e3a0`, thiscall).
const FORM_ID_84E3A0: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, cdecl(format, ...)).
const WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find voice type (%08X) for activator '%s' (%08X)."`.
const VOICE_TYPE_WARNING: u32 = 0x0102_5720;
/// `"DEFAULT: Radio station exterior position ref '%s' (%08X) is not placed
/// in an exterior."`.
const POSITION_REF_WARNING: u32 = 0x0102_57e0;
/// `"DEFAULT: Radio station ref '%s' (%08X) is set to worldspace and
/// connected interiors but is not placed in an exterior."`.
const WORLDSPACE_WARNING: u32 = 0x0102_5768;

/// The form-type test (`00401170`, thiscall, the type in `EAX`).
const FORM_TYPE: u32 = 0x0040_1170;
/// The type of a talking activator, as `IsRadioStation` compares it.
const TALKING_ACTIVATOR_TYPE: u32 = 0x16;
/// `004011d0` (thiscall on the form).
const FORM_CALL_4011D0: u32 = 0x0040_11d0;
/// The reference's base form (`007af430`, the engine map's
/// `BGSSaveFormBuffer::GetForm` on folded code).
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// `BGSTalkingActivator::RadioStationList` (Xbox PDB static): a
/// `BSSimpleList<TESObjectREFR*>`.
const RADIO_STATION_LIST: u32 = 0x011c_8264;
/// The list's add (`005ae3d0`, thiscall(item address)).
const LIST_ADD: u32 = 0x005a_e3d0;
/// The list's clear (`00470470`, thiscall).
const LIST_CLEAR: u32 = 0x0047_0470;
/// The list node's end test (`008256d0`, thiscall, bool).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The list node's item address (`006815c0`, returns `this`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// The list node's next node (`00726070`, thiscall).
const NODE_NEXT: u32 = 0x0072_6070;

/// Reference helpers (thiscall on a reference).
const REFERENCE_CELL: u32 = 0x008d_6f30;
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
/// `00440da0` (bool): with a world space, such a station is skipped.
const STATION_TEST_440DA0: u32 = 0x0044_0da0;
/// The station's range mode (`0056b210`, 0 to 4).
const STATION_RANGE_MODE: u32 = 0x0056_b210;
/// The station's position reference (`0056b140`, null for itself).
const STATION_POSITION_REFERENCE: u32 = 0x0056_b140;
/// The station's range radius (`0056b190`, a float in ST0).
const STATION_RADIUS: u32 = 0x0056_b190;
/// The distance from a reference to a position (`00572380`,
/// thiscall(position), ST0).
const REFERENCE_DISTANCE_TO: u32 = 0x0057_2380;
/// The reference virtual giving its position (`+0x1F4`).
const SLOT_GET_POSITION: u32 = 0x1f4;
/// `PathingLocation::PathingLocation` (Xbox PDB overload `_ov3`;
/// thiscall(reference)) and its destructor body (`004ff7e0`).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cd70;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const PATHING_LOCATION_SIZE: u32 = 0x28;
/// `Pathing::ComputeTeleportDoorPathLength` (Xbox PDB; cdecl(from, to,
/// radius, 0, 3), ST0).
const COMPUTE_TELEPORT_DOOR_PATH_LENGTH: u32 = 0x006d_4eb0;
/// `Pathing::BuildTeleportDoorPath` (Xbox PDB; cdecl(from, to, path, mode,
/// 0.0, 0, 1), bool).
const BUILD_TELEPORT_DOOR_PATH: u32 = 0x006d_4d20;
/// `TeleportPath::TeleportPath` and `~TeleportPath` (Xbox PDB), 0x38 bytes.
const TELEPORT_PATH_CONSTRUCT: u32 = 0x006f_48b0;
const TELEPORT_PATH_DESTRUCT: u32 = 0x006f_4930;
const TELEPORT_PATH_SIZE: u32 = 0x38;
/// The path's space count (`0044ddc0`) and space entry (`006a7af0`,
/// thiscall(index)); an entry's world space is at `+4`.
const PATH_SPACE_COUNT: u32 = 0x0044_ddc0;
const PATH_SPACE: u32 = 0x006a_7af0;
/// `FLT_MAX` and `-1.0` (`float`s in `.rdata`).
const FLOAT_MAX: u32 = 0x0101_6970;
const MINUS_ONE: u32 = 0x0101_2054;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `operator new` (cdecl(size)).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The float list node's copy constructor (`004ffa60`, thiscall(node)).
const FLOAT_NODE_COPY: u32 = 0x004f_fa60;

// Translated from 004fe7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTalkingActivator::InitItem` (Xbox PDB): nothing when `004013e0`
/// says so; else `TESObjectACTI::InitItem`, `pTempRef` cleared, and the
/// voice-type form id in `pVoiceType` (when not 0) resolved: its file's
/// compile index added, the form looked up and cast to the voice type
/// (`__RTDynamicCast`), with a `MASTERFILE` warning (the id, the editor id,
/// the form id) when that gives null. Ends with `00484ab0(1)`.
pub fn bgs_talking_activator_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_TEST_4013E0, &args![this]).bool() {
        return;
    }
    e.call(ACTIVATOR_INIT_ITEM, &args![this]);
    let base = this.addr();
    e.mem.set_u32(base + 0x90, 0);
    let voice_id = e.mem.u32(base + 0x94);
    if voice_id != 0 {
        e.with_stack(4, |e, id| {
            e.mem.set_u32(id.addr(), voice_id);
            let file = e.call(TES_FORM_GET_FILE, &args![this, u32::MAX]).u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
            let id_value = e.mem.u32(id.addr());
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id_value]).u32();
            let voice = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                )
                .u32();
            e.mem.set_u32(base + 0x94, voice);
            if e.mem.u32(base + 0x94) == 0 {
                let form_id = e.call(FORM_ID_84E3A0, &args![this]).u32();
                let name = e.vcall(base, SLOT_GET_EDITOR_ID, &args![]).u32();
                let id_value = e.mem.u32(id.addr());
                e.call(WARNING, &args![VOICE_TYPE_WARNING, id_value, name, form_id]);
            }
        });
    }
    e.call(TES_FORM_SET_FLAG_484AB0, &args![this, 1u32]);
}

/// Reads the current chunk into a word (zeroed first) and returns it.
fn chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

/// `this + offset`, or null for a null `this` (the code's base-class
/// adjustment).
fn component(this: Ptr, offset: u32) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + offset
    }
}

// Translated from 004fe890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe890 - the /GS stack cookie check (00ec408c) is not translated.
/// The talking activator's record load (thiscall(file), answers true; in
/// its vtable at `010255b4`): `TESForm::LoadForm`, `00484ab0(0)`, then each
/// chunk of the record:
/// - `FULL`: `00487050` on the full name (`+0x30`);
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `+0x3C`;
/// - `MODS`: read into a stack buffer of its size and unpacked as a texture
///   swap on `+0x3C`;
/// - `EDID`: read into a stack buffer (at most 0x200 bytes) and passed to
///   virtual `+0x134`;
/// - `OBND`: virtual `+0xE0` with the file;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `+0x68`;
/// - `SCRI`: the word goes to the scriptable form (`006ecd40` on `+0x5C`),
///   then `TESScriptableForm::InitItem(this)`;
/// - `SNAM`, `INAM`, `VNAM`: the word goes to `004febb0`, `004febd0`,
///   `004febf0`.
pub fn fn_004fe890(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(TES_FORM_SET_FLAG_484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                let full_name = component(this, FULL_NAME);
                e.call(LOAD_FULL_NAME, &args![full_name, file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = component(this, MODEL);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(this.addr(), SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(this.addr(), SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![this.addr() + MODEL, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_DSTD | CHUNK_DEST => {
                let destructible = component(this, DESTRUCTIBLE);
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![destructible, file]);
            }
            CHUNK_SCRI => {
                let script = chunk_word(e, file);
                e.call(
                    SCRIPTABLE_SET_SCRIPT,
                    &args![this.addr() + SCRIPTABLE, script],
                );
                e.call(SCRIPTABLE_INIT_ITEM, &args![this.addr() + SCRIPTABLE, this]);
            }
            CHUNK_SNAM => {
                let value = chunk_word(e, file);
                fn_004febb0(e, this, value);
            }
            CHUNK_INAM => {
                let value = chunk_word(e, file);
                fn_004febd0(e, this, value);
            }
            CHUNK_VNAM => {
                let value = chunk_word(e, file);
                fn_004febf0(e, this, value);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 004febb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `pSoundLoop` (`+0x74`).
pub fn fn_004febb0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x74), value);
}

// Translated from 004febd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `pRadioTemplate` (`+0x7C`).
pub fn fn_004febd0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x7c), value);
}

// Translated from 004febf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `pVoiceType` (`+0x94`).
pub fn fn_004febf0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x94), value);
}

// Translated from 004fedc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004011d0` on the activator, its result passed through (`EAX`).
pub fn fn_004fedc0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FORM_CALL_4011D0, &args![this]).u32()
}

// Translated from 004fede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTalkingActivator::IsRadioStation` (Xbox PDB; cdecl(form)): true for a
/// non-null form of type 0x16 for which `004fedc0` says yes.
pub fn bgs_talking_activator_is_radio_station(e: &mut Engine, form: Ptr) -> bool {
    if form.is_null() {
        return false;
    }
    if e.call(FORM_TYPE, &args![form]).u32() != TALKING_ACTIVATOR_TYPE {
        return false;
    }
    fn_004fedc0(e, form) as u8 != 0
}

// Translated from 004ff150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `reference` to `RadioStationList` when it is not null and its base
/// form is a radio station. (The code pushes a 0 before reading the base
/// form, and `IsRadioStation`'s caller cleans it with its argument: the
/// word is never read.)
pub fn fn_004ff150(e: &mut Engine, reference: Ptr) {
    if reference.is_null() {
        return;
    }
    let form = e.call(REFERENCE_BASE_FORM, &args![reference]).ptr::<()>();
    if bgs_talking_activator_is_radio_station(e, form) {
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), reference.addr());
            e.call(LIST_ADD, &args![RADIO_STATION_LIST, item]);
        });
    }
}

// Translated from 004ff190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTalkingActivator::ClearRadioStationList` (Xbox PDB): clears the list
/// (`00470470`).
pub fn bgs_talking_activator_clear_radio_station_list(e: &mut Engine) {
    e.call(LIST_CLEAR, &args![RADIO_STATION_LIST]);
}

/// True when the reference's and the station's cells (`008d6f30`) are both
/// not null and the same (the code calls each getter again for the
/// comparison).
fn same_cell(e: &mut Engine, reference: Ptr, station: u32) -> bool {
    if e.call(REFERENCE_CELL, &args![reference]).u32() == 0 {
        return false;
    }
    if e.call(REFERENCE_CELL, &args![station]).u32() == 0 {
        return false;
    }
    let mine = e.call(REFERENCE_CELL, &args![reference]).u32();
    let theirs = e.call(REFERENCE_CELL, &args![station]).u32();
    mine == theirs
}

/// The two stack `PathingLocation`s of the path calls: `from` built on the
/// reference first, `to` on the station; `f` gets `(to, from)`; then `to`'s
/// destructor and `from`'s.
fn with_locations<R>(
    e: &mut Engine,
    reference: Ptr,
    station: u32,
    f: impl FnOnce(&mut Engine, u32, u32) -> R,
) -> R {
    e.with_stack(PATHING_LOCATION_SIZE, |e, from| {
        e.with_stack(PATHING_LOCATION_SIZE, |e, to| {
            let from = e
                .call(PATHING_LOCATION_CONSTRUCT, &args![from, reference])
                .u32();
            let to = e
                .call(PATHING_LOCATION_CONSTRUCT, &args![to, station])
                .u32();
            let result = f(e, to, from);
            e.call(PATHING_LOCATION_DESTRUCT, &args![to]);
            e.call(PATHING_LOCATION_DESTRUCT, &args![from]);
            result
        })
    })
}

/// Whether the path's spaces pass: with `world_space` `Some(w)`, none has a
/// world space other than null and `w`; with `None`, none has a world
/// space. Stops at the first that fails.
fn path_spaces_pass(e: &mut Engine, path: Ptr, world_space: Option<u32>) -> bool {
    let count = e.call(PATH_SPACE_COUNT, &args![path]).u32();
    let mut index = 0u32;
    while index < count {
        let space = e.call(PATH_SPACE, &args![path, index]).u32();
        let space_world = e.mem.u32(space.wrapping_add(4));
        let fails = match world_space {
            Some(allowed) => space_world != 0 && space_world != allowed,
            None => space_world != 0,
        };
        if fails {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 004ff1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ff1a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BGSTalkingActivator::GetRadioStationsInRangeOfRef` (Xbox PDB;
/// cdecl(reference, stations, distances)). Nothing unless both the
/// reference and the station list are not null and the reference has a
/// cell. With the reference's world space (or, without one, its cell), for
/// each station of `RadioStationList` (skipped when `00440da0` says so and
/// the reference has a world space), by the station's range mode:
/// - 0 (radius): the distance from the reference to the station's position
///   reference (itself when `0056b140` gives null) when both are in the same
///   world space, or the teleport-door path length (radius, 0, 3) when the
///   reference is in none, else `FLT_MAX`; in range when it is below the
///   station's radius (a `DEFAULT` warning instead when the position
///   reference is in no world space);
/// - 1: in range (distance -1);
/// - 2 (world space and connected interiors): with a world space, in range
///   when it is the station's; without one, when a teleport-door path
///   (mode 1) exists through no other world space; a `DEFAULT` warning when
///   the station is in no world space;
/// - 3 (only without a world space): when both are in the same cell, or a
///   teleport-door path (mode 2) exists, and the path has no world space;
/// - 4: when both are in the same cell.
///
/// A station in range is added to `stations` and, when `distances` is not
/// null, its distance (`FLT_MAX` or -1 unless measured) to `distances`
/// (`004ff980`).
pub fn bgs_talking_activator_get_radio_stations_in_range_of_ref(
    e: &mut Engine,
    reference: Ptr,
    stations: Ptr,
    distances: Ptr,
) {
    if stations.is_null()
        || reference.is_null()
        || e.call(REFERENCE_CELL, &args![reference]).u32() == 0
    {
        return;
    }
    let world_space = e.call(REFERENCE_WORLD_SPACE, &args![reference]).u32();
    let cell = if world_space == 0 {
        e.call(REFERENCE_CELL, &args![reference]).u32()
    } else {
        0
    };
    let float_max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
    e.with_stack(4, |e, station_slot| {
        e.with_stack(4, |e, distance| {
            e.mem.set_u32(distance.addr(), float_max);
            let mut node = RADIO_STATION_LIST;
            while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                let item = e.call(NODE_ITEM, &args![node]).u32();
                let station = e.mem.u32(item);
                e.mem.set_u32(station_slot.addr(), station);
                let mut in_range = false;
                let float_max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
                e.mem.set_u32(distance.addr(), float_max);
                let skipped =
                    e.call(STATION_TEST_440DA0, &args![station]).bool() && world_space != 0;
                if !skipped {
                    let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
                    match e.call(STATION_RANGE_MODE, &args![station]).u32() {
                        0 => {
                            let mut position =
                                e.call(STATION_POSITION_REFERENCE, &args![station]).u32();
                            if position == 0 {
                                position = station;
                            }
                            let position_world =
                                e.call(REFERENCE_WORLD_SPACE, &args![position]).u32();
                            if position_world == 0 {
                                let form_id = e.call(FORM_ID_84E3A0, &args![position]).u32();
                                let name = e.vcall(position, SLOT_GET_EDITOR_ID, &args![]).u32();
                                e.call(WARNING, &args![POSITION_REF_WARNING, name, form_id]);
                            } else {
                                if world_space != 0 && world_space == position_world {
                                    let point =
                                        e.vcall(position, SLOT_GET_POSITION, &args![]).u32();
                                    let measured = e
                                        .call(REFERENCE_DISTANCE_TO, &args![reference, point])
                                        .f64();
                                    e.mem.set_u32(distance.addr(), x87_store32(measured));
                                } else if world_space == 0 {
                                    let radius = e.call(STATION_RADIUS, &args![station]).f64();
                                    let radius = x87_store32(radius);
                                    let measured =
                                        with_locations(e, reference, station, |e, to, from| {
                                            e.call(
                                                COMPUTE_TELEPORT_DOOR_PATH_LENGTH,
                                                &args![to, from, radius, 0u32, 3u32],
                                            )
                                            .f64()
                                        });
                                    e.mem.set_u32(distance.addr(), x87_store32(measured));
                                }
                                let radius = e.call(STATION_RADIUS, &args![station]).f64();
                                // `FLD distance; FCOMPP; TEST AH,5; JP`: in range
                                // only for distance < radius (not for NaN).
                                if x87_load32(e.mem.u32(distance.addr())) < radius {
                                    in_range = true;
                                }
                            }
                        }
                        1 => {
                            in_range = true;
                            e.mem.set_u32(distance.addr(), minus_one);
                        }
                        2 => {
                            let station_world =
                                e.call(REFERENCE_WORLD_SPACE, &args![station]).u32();
                            if station_world == 0 {
                                let form_id = e.call(FORM_ID_84E3A0, &args![station]).u32();
                                let name = e.vcall(station, SLOT_GET_EDITOR_ID, &args![]).u32();
                                e.call(WARNING, &args![WORLDSPACE_WARNING, name, form_id]);
                            } else if world_space != 0 {
                                if world_space == station_world {
                                    in_range = true;
                                    e.mem.set_u32(distance.addr(), minus_one);
                                }
                            } else {
                                e.with_stack(TELEPORT_PATH_SIZE, |e, path| {
                                    e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
                                    let built =
                                        with_locations(e, reference, station, |e, to, from| {
                                            e.call(
                                                BUILD_TELEPORT_DOOR_PATH,
                                                &args![to, from, path, 1u32, 0u32, 0u32, 1u32],
                                            )
                                            .bool()
                                        });
                                    if built {
                                        in_range = true;
                                        e.mem.set_u32(distance.addr(), minus_one);
                                        if !path_spaces_pass(e, path, Some(station_world)) {
                                            in_range = false;
                                        }
                                    }
                                    e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
                                });
                            }
                        }
                        3 => {
                            if cell != 0 {
                                e.with_stack(TELEPORT_PATH_SIZE, |e, path| {
                                    e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
                                    let connected = same_cell(e, reference, station)
                                        || with_locations(e, reference, station, |e, to, from| {
                                            e.call(
                                                BUILD_TELEPORT_DOOR_PATH,
                                                &args![to, from, path, 2u32, 0u32, 0u32, 1u32],
                                            )
                                            .bool()
                                        });
                                    if connected {
                                        in_range = true;
                                        e.mem.set_u32(distance.addr(), minus_one);
                                        if !path_spaces_pass(e, path, None) {
                                            in_range = false;
                                        }
                                    }
                                    e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
                                });
                            }
                        }
                        4 => {
                            if same_cell(e, reference, station) {
                                in_range = true;
                                e.mem.set_u32(distance.addr(), minus_one);
                            }
                        }
                        _ => {}
                    }
                    if in_range {
                        e.call(LIST_ADD, &args![stations, station_slot]);
                        if !distances.is_null() {
                            fn_004ff980(e, distances, distance);
                        }
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        });
    });
}

// Translated from 004ff980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ff980 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Adds the float at `value` to the front of the float list at `this`
/// (a `BSSimpleList<float>`: item, next), unless it is 0.0: an item of 0.0
/// is replaced; otherwise a new 8-byte node copies the head (`004ffa60`)
/// and is linked after it, and the head takes the value. (A failed
/// allocation is linked as a null node, whose `+4` the code still writes.)
pub fn fn_004ff980(e: &mut Engine, this: Ptr, value: Ptr) {
    let zero: f64 = e.global(ZERO);
    // `FCOMP; TEST AH,0x44; JP`: equal (ordered) means 0.0.
    if x87_load32(e.mem.u32(value.addr())) == zero {
        return;
    }
    let head = this.addr();
    if x87_load32(e.mem.u32(head)) == zero {
        let bits = x87_quiet_bits(e.mem.u32(value.addr()));
        e.mem.set_u32(head, bits);
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let node = if block == 0 {
        0
    } else {
        e.call(FLOAT_NODE_COPY, &args![block, this]).u32()
    };
    let next = e.mem.u32(head + 4);
    e.mem.set_u32(node.wrapping_add(4), next);
    e.mem.set_u32(head + 4, node);
    let bits = x87_quiet_bits(e.mem.u32(value.addr()));
    e.mem.set_u32(head, bits);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_e7c0, bgs_talking_activator_init_item(Ptr)),
        entry!(0x004f_e890, fn_004fe890(Ptr, Ptr) -> bool),
        entry!(0x004f_ebb0, fn_004febb0(Ptr, u32)),
        entry!(0x004f_ebd0, fn_004febd0(Ptr, u32)),
        entry!(0x004f_ebf0, fn_004febf0(Ptr, u32)),
        entry!(0x004f_edc0, fn_004fedc0(Ptr) -> u32),
        entry!(
            0x004f_ede0,
            bgs_talking_activator_is_radio_station(Ptr) -> bool
        ),
        entry!(0x004f_f150, fn_004ff150(Ptr)),
        entry!(
            0x004f_f190,
            bgs_talking_activator_clear_radio_station_list()
        ),
        entry!(
            0x004f_f1a0,
            bgs_talking_activator_get_radio_stations_in_range_of_ref(Ptr, Ptr, Ptr)
        ),
        entry!(0x004f_f980, fn_004ff980(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// An object whose vtable sends slot `s` to `base + s` (doubles that
    /// return `base + s` in `EAX`).
    fn object(e: &mut Engine, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register_double(base + slot, move |_, _| (base + slot).into_ret());
        }
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn init_item_resolves_the_voice_type() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_TEST_4013E0,
                ACTIVATOR_INIT_ITEM,
                TES_FORM_ADD_COMPILE_INDEX,
                TES_FORM_SET_FLAG_484AB0,
                WARNING,
            ],
        );
        e.register(TES_FORM_GET_FILE, |_, _| 0x0f11_e000u32.into_ret());
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let this = object(&mut e, 0x7100_0000);
        e.mem.set_u32(this + 0x90, 5);
        e.mem.set_u32(this + 0x94, 0x0123);
        e.call_log = Some(vec![]);
        e.call(0x004f_e7c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x94), 0x1123);
        assert_eq!(calls(&log, TES_FORM_GET_FILE), vec![vec![this, u32::MAX]]);
        assert_eq!(calls(&log, TES_FORM_ADD_COMPILE_INDEX)[0][1], 0x0f11_e000);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![0x1123, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert!(calls(&log, WARNING).is_empty());
        assert_eq!(calls(&log, TES_FORM_SET_FLAG_484AB0), vec![vec![this, 1]]);
    }

    #[test]
    fn init_item_warns_for_a_missing_voice_type_and_skips_when_told() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_TEST_4013E0,
                ACTIVATOR_INIT_ITEM,
                TES_FORM_GET_FILE,
                TES_FORM_ADD_COMPILE_INDEX,
                TES_FORM_SET_FLAG_484AB0,
                WARNING,
                LOOKUP_FORM_BY_ID,
                RT_DYNAMIC_CAST,
            ],
        );
        e.register(FORM_ID_84E3A0, |_, _| 0x0abc_def0u32.into_ret());
        let this = object(&mut e, 0x7100_0000);
        e.mem.set_u32(this + 0x94, 0x0123);
        e.call_log = Some(vec![]);
        e.call(0x004f_e7c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WARNING),
            vec![vec![
                VOICE_TYPE_WARNING,
                0x0123,
                0x7100_0000 + SLOT_GET_EDITOR_ID,
                0x0abc_def0
            ]]
        );
        e.register(FORM_TEST_4013E0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004f_e7c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ACTIVATOR_INIT_ITEM).is_empty());
        assert!(calls(&log, TES_FORM_SET_FLAG_484AB0).is_empty());
    }

    #[test]
    fn load_hands_each_chunk_to_its_reader() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TES_FORM_LOAD_FORM,
                TES_FORM_SET_FLAG_484AB0,
                GET_CHUNK_DATA,
                LOAD_FULL_NAME,
                LOAD_MODEL_CHUNK,
                UNPACK_TEXTURE_SWAP,
                LOAD_DESTRUCTIBLE_CHUNK,
                SCRIPTABLE_SET_SCRIPT,
                SCRIPTABLE_INIT_ITEM,
            ],
        );
        e.register(CHUNK_SIZE, |_, _| 0x20u32.into_ret());
        e.register(FILE_WORD, |_, _| 0x0003_0007u32.into_ret());
        let chunks = [
            CHUNK_FULL,
            CHUNK_MODT,
            CHUNK_EDID,
            CHUNK_OBND,
            CHUNK_MODS,
            CHUNK_DEST,
            CHUNK_SCRI,
            CHUNK_SNAM,
            CHUNK_INAM,
            CHUNK_VNAM,
            0x1234_5678,
        ];
        e.register_double(GET_TES_CHUNK, {
            let mut next = 0usize;
            move |_, _| {
                next += 1;
                chunks.get(next - 1).copied().unwrap_or(0).into_ret()
            }
        });
        e.register_double(NEXT_CHUNK, {
            let mut left = chunks.len();
            move |_, _| {
                left -= 1;
                ((left > 0) as u32).into_ret()
            }
        });
        e.register_double(GET_CHUNK_WORD, {
            let mut value = 0x100u32;
            move |e, a| {
                value += 1;
                e.mem.set_u32(a[1], value);
                Ret::default()
            }
        });
        let this = object(&mut e, 0x7100_0000);
        let file = 0x0f11_e000u32;
        e.call_log = Some(vec![]);
        let loaded = e.call(0x004f_e890, &args![this, file]).bool();
        let log = e.call_log.take().unwrap();
        assert!(loaded);
        assert_eq!(calls(&log, TES_FORM_SET_FLAG_484AB0), vec![vec![this, 0]]);
        assert_eq!(calls(&log, LOAD_FULL_NAME), vec![vec![this + 0x30, file]]);
        assert_eq!(calls(&log, LOAD_MODEL_CHUNK), vec![vec![this + 0x3c, file]]);
        let reads = calls(&log, GET_CHUNK_DATA);
        assert_eq!(reads[0][2], EDITOR_ID_MAXIMUM);
        assert_eq!(
            calls(&log, 0x7100_0000 + SLOT_SET_EDITOR_ID),
            vec![vec![this, reads[0][1]]]
        );
        assert_eq!(
            calls(&log, 0x7100_0000 + SLOT_LOAD_BOUNDS),
            vec![vec![this, file]]
        );
        assert_eq!(
            calls(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![this + 0x3c, reads[1][1], 0x20, 7]]
        );
        assert_eq!(
            calls(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this + 0x68, file]]
        );
        assert_eq!(
            calls(&log, SCRIPTABLE_SET_SCRIPT),
            vec![vec![this + 0x5c, 0x101]]
        );
        assert_eq!(
            calls(&log, SCRIPTABLE_INIT_ITEM),
            vec![vec![this + 0x5c, this]]
        );
        assert_eq!(e.mem.u32(this + 0x74), 0x102);
        assert_eq!(e.mem.u32(this + 0x7c), 0x103);
        assert_eq!(e.mem.u32(this + 0x94), 0x104);
    }

    #[test]
    fn setters_store_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.call(0x004f_ebb0, &args![this, 1u32]);
        e.call(0x004f_ebd0, &args![this, 2u32]);
        e.call(0x004f_ebf0, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0x74), 1);
        assert_eq!(e.mem.u32(this + 0x7c), 2);
        assert_eq!(e.mem.u32(this + 0x94), 3);
    }

    #[test]
    fn is_radio_station_needs_type_0x16_and_the_flag() {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_CALL_4011D0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let form = e.mem.alloc(8);
        assert!(!e.call(0x004f_ede0, &args![0u32]).bool());
        e.mem.set_u32(form, 0x15);
        e.mem.set_u32(form + 4, 1);
        assert!(!e.call(0x004f_ede0, &args![form]).bool());
        e.mem.set_u32(form, 0x16);
        assert!(e.call(0x004f_ede0, &args![form]).bool());
        // Only `AL` counts.
        e.mem.set_u32(form + 4, 0x100);
        assert!(!e.call(0x004f_ede0, &args![form]).bool());
        assert_eq!(e.call(0x004f_edc0, &args![form]).u32(), 0x100);
    }

    #[test]
    fn station_list_add_and_clear() {
        let mut e = Engine::new();
        e.register(REFERENCE_BASE_FORM, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_TYPE, |_, _| 0x16u32.into_ret());
        e.register(FORM_CALL_4011D0, |_, _| 1u32.into_ret());
        e.register(LIST_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.register(LIST_CLEAR, |_, _| Ret::default());
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x004f_f150, &args![reference]);
        e.call(0x004f_f150, &args![0u32]);
        e.call(0x004f_f190, &args![]);
        let log = e.call_log.take().unwrap();
        let adds = calls(&log, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], RADIO_STATION_LIST);
        assert_eq!(calls(&log, LIST_CLEAR), vec![vec![RADIO_STATION_LIST]]);
        // A base form that is no radio station is not added.
        e.register(FORM_CALL_4011D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_f150, &args![reference]);
        assert!(calls(&e.call_log.take().unwrap(), LIST_ADD).is_empty());
    }

    const REFERENCE: u32 = 0x0aaa_0000;
    const STATIONS: u32 = 0x0bbb_0000;

    /// One station of the given range mode in the list; the reference has a
    /// cell; `world` is the reference's world space. Doubles return 0 unless
    /// set otherwise.
    fn station_setup(e: &mut Engine, mode: u32, world: u32) -> u32 {
        quiet(
            e,
            &[
                STATION_TEST_440DA0,
                NODE_IS_EMPTY,
                NODE_NEXT,
                STATION_POSITION_REFERENCE,
                WARNING,
                FORM_ID_84E3A0,
                PATHING_LOCATION_DESTRUCT,
                TELEPORT_PATH_CONSTRUCT,
                TELEPORT_PATH_DESTRUCT,
                BUILD_TELEPORT_DOOR_PATH,
                PATH_SPACE_COUNT,
                PATH_SPACE,
                FLOAT_NODE_COPY,
            ],
        );
        e.map(0x0101_2000, 0x5000);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ZERO, 0.0f64);
        let station = object(e, 0x7100_0000);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, station);
        e.register_double(NODE_ITEM, move |_, _| item.into_ret());
        e.register(REFERENCE_CELL, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(REFERENCE_WORLD_SPACE, move |_, a| {
            if a[0] == REFERENCE {
                world.into_ret()
            } else {
                0x0e0e_0000u32.into_ret()
            }
        });
        e.register_double(STATION_RANGE_MODE, move |_, _| mode.into_ret());
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(PATHING_LOCATION_CONSTRUCT, |_, a| a[0].into_ret());
        station
    }

    fn run(e: &mut Engine, distances: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(0x004f_f1a0, &args![REFERENCE, STATIONS, distances]);
        e.call_log.take().unwrap()
    }

    #[test]
    fn station_modes_1_and_4_and_the_skip() {
        let mut e = Engine::new();
        let station = station_setup(&mut e, 1, 0x0e0e_0000);
        let distances = e.mem.alloc(8);
        let log = run(&mut e, distances);
        let adds = calls(&log, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], STATIONS);
        assert_eq!(e.mem.u32(adds[0][1]), station);
        assert_eq!(e.mem.f32(distances), -1.0);

        // Mode 4: the same cell only (the doubles give each its own).
        e.register(STATION_RANGE_MODE, |_, _| 4u32.into_ret());
        let log = run(&mut e, 0);
        assert!(calls(&log, LIST_ADD).is_empty());
        assert_eq!(calls(&log, REFERENCE_CELL).len(), 5);
        e.register(REFERENCE_CELL, |_, _| 0x0ce1_0000u32.into_ret());
        let log = run(&mut e, 0);
        assert_eq!(calls(&log, LIST_ADD).len(), 1);

        // Skipped when 00440da0 says so and the reference has a world space.
        e.register(STATION_TEST_440DA0, |_, _| 1u32.into_ret());
        let log = run(&mut e, 0);
        assert!(calls(&log, STATION_RANGE_MODE).is_empty());
        // Nothing at all without a cell.
        e.register(REFERENCE_CELL, |_, _| Ret::default());
        let log = run(&mut e, 0);
        assert!(calls(&log, NODE_IS_EMPTY).is_empty());
    }

    #[test]
    fn station_mode_0_measures_the_distance() {
        let mut e = Engine::new();
        let station = station_setup(&mut e, 0, 0x0e0e_0000);
        e.register(STATION_RADIUS, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.register(REFERENCE_DISTANCE_TO, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        let distances = e.mem.alloc(8);
        let log = run(&mut e, distances);
        assert_eq!(
            calls(&log, REFERENCE_DISTANCE_TO),
            vec![vec![REFERENCE, 0x7100_0000 + SLOT_GET_POSITION]]
        );
        assert_eq!(
            calls(&log, 0x7100_0000 + SLOT_GET_POSITION),
            vec![vec![station]]
        );
        assert_eq!(calls(&log, LIST_ADD).len(), 1);
        assert_eq!(e.mem.f32(distances), 50.0);

        // Out of range, and NaN is never in range.
        for far in [150.0, f64::NAN] {
            e.register_double(REFERENCE_DISTANCE_TO, move |_, _| Ret {
                st0: far,
                ..Ret::default()
            });
            let log = run(&mut e, 0);
            assert!(calls(&log, LIST_ADD).is_empty());
        }

        // Without a world space: the teleport-door path length.
        e.register(COMPUTE_TELEPORT_DOOR_PATH_LENGTH, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(REFERENCE_WORLD_SPACE, |_, a| {
            if a[0] == REFERENCE {
                Ret::default()
            } else {
                0x0e0e_0000u32.into_ret()
            }
        });
        let log = run(&mut e, 0);
        let length = calls(&log, COMPUTE_TELEPORT_DOOR_PATH_LENGTH);
        let locations = calls(&log, PATHING_LOCATION_CONSTRUCT);
        assert_eq!(locations[0][1], REFERENCE);
        assert_eq!(locations[1][1], station);
        assert_eq!(
            length,
            vec![vec![
                locations[1][0],
                locations[0][0],
                100.0f32.to_bits(),
                0,
                3
            ]]
        );
        assert_eq!(
            calls(&log, PATHING_LOCATION_DESTRUCT),
            vec![vec![locations[1][0]], vec![locations[0][0]]]
        );
        assert_eq!(calls(&log, LIST_ADD).len(), 1);

        // A position reference in no world space: the warning.
        e.register(REFERENCE_WORLD_SPACE, |_, _| Ret::default());
        e.register(FORM_ID_84E3A0, |_, _| 0x0abcu32.into_ret());
        let log = run(&mut e, 0);
        assert_eq!(
            calls(&log, WARNING),
            vec![vec![
                POSITION_REF_WARNING,
                0x7100_0000 + SLOT_GET_EDITOR_ID,
                0x0abc
            ]]
        );
        assert!(calls(&log, STATION_RADIUS).is_empty());
    }

    #[test]
    fn station_mode_2_world_space_or_path() {
        let mut e = Engine::new();
        station_setup(&mut e, 2, 0x0e0e_0000);
        let log = run(&mut e, 0);
        assert_eq!(calls(&log, LIST_ADD).len(), 1);
        assert!(calls(&log, TELEPORT_PATH_CONSTRUCT).is_empty());

        // Without a world space: a path through no other world space.
        e.register(REFERENCE_WORLD_SPACE, |_, a| {
            if a[0] == REFERENCE {
                Ret::default()
            } else {
                0x0e0e_0000u32.into_ret()
            }
        });
        e.register(BUILD_TELEPORT_DOOR_PATH, |_, _| 1u32.into_ret());
        e.register(PATH_SPACE_COUNT, |_, _| 2u32.into_ret());
        let spaces = e.mem.alloc(0x10);
        e.mem.set_u32(spaces + 4, 0x0e0e_0000);
        e.register_double(PATH_SPACE, move |_, a| (spaces + 8 * a[1]).into_ret());
        let log = run(&mut e, 0);
        let path = calls(&log, TELEPORT_PATH_CONSTRUCT)[0][0];
        let build = calls(&log, BUILD_TELEPORT_DOOR_PATH);
        assert_eq!(build[0][2..], [path, 1, 0, 0, 1]);
        assert_eq!(calls(&log, LIST_ADD).len(), 1);
        assert_eq!(calls(&log, TELEPORT_PATH_DESTRUCT), vec![vec![path]]);
        // Another world space on the path: not in range.
        e.mem.set_u32(spaces + 12, 0x0f0f_0000);
        let log = run(&mut e, 0);
        assert!(calls(&log, LIST_ADD).is_empty());
    }

    #[test]
    fn station_mode_3_needs_a_cell_without_world_space() {
        let mut e = Engine::new();
        station_setup(&mut e, 3, 0);
        // Different cells: the path (mode 2) is built; it fails.
        let log = run(&mut e, 0);
        let path = calls(&log, TELEPORT_PATH_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&log, BUILD_TELEPORT_DOOR_PATH)[0][2..],
            [path, 2, 0, 0, 1]
        );
        assert!(calls(&log, LIST_ADD).is_empty());
        // The same cell: no path call, in range with no world space on it.
        e.register(REFERENCE_CELL, |_, _| 0x0ce1_0000u32.into_ret());
        let log = run(&mut e, 0);
        assert!(calls(&log, BUILD_TELEPORT_DOOR_PATH).is_empty());
        assert!(calls(&log, PATHING_LOCATION_CONSTRUCT).is_empty());
        assert_eq!(calls(&log, LIST_ADD).len(), 1);
        // With a world space, mode 3 does nothing.
        e.register(REFERENCE_WORLD_SPACE, |_, _| 0x0e0e_0000u32.into_ret());
        let log = run(&mut e, 0);
        assert!(calls(&log, TELEPORT_PATH_CONSTRUCT).is_empty());
        assert!(calls(&log, LIST_ADD).is_empty());
    }

    #[test]
    fn float_list_add_replaces_zero_and_links_nodes() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.register(OPERATOR_NEW, |e, _| e.mem.alloc(8).into_ret());
        e.register(FLOAT_NODE_COPY, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            a[0].into_ret()
        });
        let list = e.mem.alloc(8);
        let value = e.mem.alloc(4);
        // 0.0 is not added.
        e.call(0x004f_f980, &args![list, value]);
        assert_eq!(e.mem.u32(list + 4), 0);
        // An empty (0.0) head takes the value.
        e.mem.set_f32(value, 2.5);
        e.call(0x004f_f980, &args![list, value]);
        assert_eq!(e.mem.f32(list), 2.5);
        assert_eq!(e.mem.u32(list + 4), 0);
        // Otherwise the head is copied into a new node after it.
        e.mem.set_f32(value, 4.0);
        e.call(0x004f_f980, &args![list, value]);
        assert_eq!(e.mem.f32(list), 4.0);
        let node = e.mem.u32(list + 4);
        assert_eq!(e.mem.f32(node), 2.5);
        assert_eq!(e.mem.u32(node + 4), 0);
    }
}
