//! `fallout shared/teswaterform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0057eba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057eba0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4e (0x194 bytes); `ECX` is the object.
/// The engine map names it `TESWaterForm::TESWaterForm` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_140c`) is `TESWaterForm`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the floats at `01016248` (three times) and `010162c0` into `+0x168..+0x174`.
/// The exception-unwinding frame is not translated.
pub fn fn_0057eba0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0047_fdc0, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_140c);
    e.mem.set_u32(base + 0x18, 0x0103_13f4);
    e.mem.set_u32(base + 0x24, 0x0103_13dc);
    e.call(0x0063_3c90, &args![base + 0x30, 0x0u32]);
    e.call(
        0x0041_4430,
        &args![base + 0x34, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(
        0x0041_4430,
        &args![base + 0x44, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(
        0x0041_4430,
        &args![base + 0x54, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(0x0048_e270, &args![base + 0x64]);
    e.call(0x0040_37b0, &args![base + 0x74]);
    e.call(0x0057_e9a0, &args![base + 0x84]);
    e.call(0x0063_3c90, &args![base + 0x17c, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x4eu32]);
    e.call(0x0057_ef10, &args![this]);
    e.mem.set_u32(base + 0x154, 0x0);
    e.mem.set_u32(base + 0x158, 0x0);
    e.mem.set_u32(base + 0x15c, 0x9);
    e.mem.set_u32(base + 0x160, 0x9);
    e.mem.set_u32(base + 0x164, 0x4);
    let f1 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x168, f1);
    let f2 = e.global::<f32>(0x0101_62c0);
    e.mem.set_f32(base + 0x16c, f2);
    let f3 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x170, f3);
    let f4 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x174, f4);
    e.mem.set_u32(base + 0x178, 0x0);
    e.mem.set_u8(base + 0x2c, 0x1);
    e.mem.set_u8(base + 0x190, 0x0);
    e.mem.set_u32(base + 0x180, 0x0);
    e.mem.set_u32(base + 0x184, 0x0);
    e.mem.set_u32(base + 0x188, 0x0);
    e.mem.set_u32(base + 0x18c, 0x0);
    e.mem.set_u32(base + 0x80, 0x0);
    this
}

// Translated from 0057ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0057ee10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0057ede0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0057_ee10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the type of
/// the current record.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`): `ECX` = form, the file.
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`ECX` = form, a bool): sets or clears form flag 0x8.
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `004013e0` (`ECX` = form): whether form flag 0x8 is set.
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`): the current chunk's type.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk, false at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the flag `Load` tests before swapping the byte
/// order of what it read.
const FILE_SWAPS_BYTES: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): `ECX` = file, the
/// destination.
const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): `ECX` = file, the
/// destination, the most bytes to read.
const FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`): `ECX` = form, the file, the
/// destination, its size.
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00486620` (`ECX` = form): the size `Load` subtracts from the `DATA`
/// chunk's size.
const FORM_DATA_SIZE: u32 = 0x0048_6620;
/// `00487050` (`cdecl`: the `TESFullName`, the file).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`: the
/// texture, the file).
const TEXTURE_LOAD_CHUNK: u32 = 0x0048_e3d0;
/// `004037f0` (`ECX` = `BSStringT`, the text, a word): sets the string.
const STRING_SET: u32 = 0x0040_37f0;
/// `00401080` (`cdecl`: a word's address, a byte), called on each weather
/// form id after a `GNAM` read when the byte-order flag is set.
const SWAP_WORD: u32 = 0x0040_1080;
/// `0057f3a0` (`ECX` = the `WaterShaderData` at +0x84), called after a
/// `DATA`/`DNAM` read when the byte-order flag is set.
const SHADER_DATA_SWAP: u32 = 0x0057_f3a0;
/// `0047ff50` (`ECX` = the `TESAttackDamageForm` at +0x24, a word).
const ATTACK_DAMAGE_SET: u32 = 0x0047_ff50;
/// `004febd0` (`ECX` = form, a word): given the `SNAM` chunk's word.
const SNAM_STORE: u32 = 0x004f_ebd0;
/// `004fc990` (`ECX` = `BGSPlaceableWater`, a flag mask, a bool).
const PLACEABLE_SET_FLAG: u32 = 0x004f_c990;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`; `ECX` = form, the index).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`; `cdecl`: address of a
/// form id, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vfptr offset, source type,
/// target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;
/// The form's name virtual slot (the messages' `%s`).
const NAME_SLOT: u32 = 0x130;
/// The type descriptors of the casts: `.?AVTESForm@@`, `.?AVTESSound@@`,
/// `.?AVSpellItem@@`, `.?AVTESWaterForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_WATER_FORM: u32 = 0x0118_4118;
/// `"MASTERFILE: Could not find sound (%08X) for water type '%s' (%08X)."`
const MISSING_SOUND: u32 = 0x0103_15d8;
/// `"MASTERFILE: Could not find sound (%08X) for spell item '%s' (%08X)."`
/// (the game's own text, printed for the missing spell).
const MISSING_SPELL: u32 = 0x0103_1590;
/// `"MASTERFILE: Could not find WaterForm (%08X) for water type '%s' (%08X)."`
const MISSING_WATER_FORM: u32 = 0x0103_1548;
/// `"PlaceableLODWater"`.
const PLACEABLE_LOD_WATER_NAME: u32 = 0x0103_1630;

/// Record and chunk types (`'NAME'` as the little-endian word the file holds).
const RECORD_WATR: u32 = 0x4e;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_DNAM: u32 = 0x4d41_4e44;
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_FNAM: u32 = 0x4d41_4e46;
const SIG_GNAM: u32 = 0x4d41_4e47;
const SIG_MNAM: u32 = 0x4d41_4e4d;
const SIG_NNAM: u32 = 0x4d41_4e4e;
const SIG_SNAM: u32 = 0x4d41_4e53;
const SIG_XNAM: u32 = 0x4d41_4e58;

// Offsets of embedded members of `TESWaterForm` (Xbox PDB names, PC
// offsets: 0x10 lower than the PDB's, as for every `TESForm`).
/// `TESFullName` base.
const WATER_FULL_NAME: u32 = 0x18;
/// `TESAttackDamageForm` base.
const WATER_ATTACK_DAMAGE: u32 = 0x24;
/// `NoiseTexture` (`TESTexture`).
const WATER_NOISE_TEXTURE: u32 = 0x64;
/// `cMaterialID` (`BSStringT`).
const WATER_MATERIAL_ID: u32 = 0x74;
/// `Data` (`WaterShaderData`, 0xC4 bytes).
const WATER_DATA: u32 = 0x84;
/// `Data.fHeightUVScale0` .. `Data.fAmplitude2`: six floats.
const WATER_HEIGHT_UV_SCALE: u32 = 0x130;
/// `pWaterWeatherControl` (three `TESWaterForm *`).
const WATER_WEATHER_CONTROL: u32 = 0x148;

layout! {
    /// `TESWaterForm` (Xbox PDB 0x1A4 bytes; PC 0x194).
    pub struct TESWaterForm: 0x194 {
        /// `cAlpha` (Xbox PDB).
        0x70 cAlpha: u8,
        /// `cFlags` (Xbox PDB).
        0x71 cFlags: u8,
        /// `pWaterSound` (Xbox PDB): a form id until `InitItem`.
        0x7C pWaterSound: u32,
        /// `pSpellItem` (Xbox PDB): a form id until `InitItem`.
        0x178 pSpellItem: u32,
        /// `pPlaceableLODWater` (Xbox PDB).
        0x184 pPlaceableLODWater: u32,
        /// `iRadiationDamage` (Xbox PDB).
        0x188 iRadiationDamage: u32,
        /// `iHealthRestoration` (Xbox PDB).
        0x18C iHealthRestoration: u32,
    }
}

/// `FLD dword [src]; FSTP dword [dst]`.
fn copy_float(e: &mut Engine, src: u32, dst: u32) {
    let bits = e.mem.u32(src);
    e.mem.set_u32(dst, x87_quiet_bits(bits));
}

/// One form id of `InitItem` at `field`: compile index, look-up, cast to
/// `target` (stored back at `field`), and `message` when the cast finds
/// nothing.
fn resolve_form(e: &mut Engine, this: Ptr<TESWaterForm>, field: u32, target: u32, message: u32) {
    e.with_stack(4, |e, id_at| {
        let id = e.mem.u32(field);
        e.mem.set_u32(id_at.addr(), id);
        let file = e.call(FORM_GET_FILE, &args![this, u32::MAX]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![id_at, file]);
        let id = e.mem.u32(id_at.addr());
        let form = e.call(FORM_BY_ID, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
            )
            .u32();
        e.mem.set_u32(field, cast);
        if e.mem.u32(field) == 0 {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), NAME_SLOT, &[]).u32();
            let id = e.mem.u32(id_at.addr());
            e.call(PRINT, &args![message, id, name, form_id]);
        }
    });
}

/// A word read with `TESFile::GetChunkData` into a zeroed stack word.
fn chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word_at| {
        e.call(FILE_GET_CHUNK_DATA, &args![file, word_at]);
        e.mem.u32(word_at.addr())
    })
}

// Translated from 0057eef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the form's `WaterShaderData` (`this + 0x84`).
pub fn fn_0057eef0(_e: &mut Engine, this: Ptr<TESWaterForm>) -> Ptr {
    this.byte_add(WATER_DATA)
}

// Translated from 0057e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShaderData`'s constructor: sets the defaults (`0057e9c0`) and
/// returns `this`.
pub fn fn_0057e9a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0057_e9c0, &args![this]);
    this
}

// Translated from 0057ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form's start values: alpha 0x4B, flags 0, no sound, an empty
/// material string (`004037f0` with 0, 0), the shader data's defaults
/// (`0057e9c0` on `0057eef0`'s address) and no weather forms.
pub fn fn_0057ef10(e: &mut Engine, this: Ptr<TESWaterForm>) {
    e.set(this, TESWaterForm::cAlpha, 0x4b);
    e.set(this, TESWaterForm::cFlags, 0);
    e.set(this, TESWaterForm::pWaterSound, 0);
    e.call(
        STRING_SET,
        &args![this.byte_add(WATER_MATERIAL_ID), 0u32, 0u32],
    );
    let data = fn_0057eef0(e, this);
    e.call(0x0057_e9c0, &args![data]);
    for i in 0..3 {
        e.mem
            .set_u32(this.addr() + WATER_WEATHER_CONTROL + 4 * i, 0);
    }
}

// Translated from 0057ee10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057ee10 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESWaterForm`'s destructor body: stores the three vtables, then
/// destroys the members last to first: the pointer at +0x17C (`0066b0d0`
/// with 0, then `0045cec0`), the material string (`004037d0`), the noise
/// texture (`0048e2e0`), the pointer at +0x30 (`0045cec0`), the
/// attack-damage and full-name bases (`0047fdf0`, `00591300`) and the
/// `TESForm` base (`TESForm::~TESForm`, `00483630`).
pub fn fn_0057ee10(e: &mut Engine, this: Ptr<TESWaterForm>) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_140c);
    e.mem.set_u32(base + 0x18, 0x0103_13f4);
    e.mem.set_u32(base + 0x24, 0x0103_13dc);
    e.call(0x0066_b0d0, &args![base + 0x17c, 0u32]);
    e.call(0x0045_cec0, &args![base + 0x17c]);
    e.call(0x0040_37d0, &args![base + WATER_MATERIAL_ID]);
    e.call(0x0048_e2e0, &args![base + WATER_NOISE_TEXTURE]);
    e.call(0x0045_cec0, &args![base + 0x30]);
    e.call(0x0047_fdf0, &args![base + WATER_ATTACK_DAMAGE]);
    e.call(0x0059_1300, &args![base + WATER_FULL_NAME]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0057ef80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057ef80 - the /GS cookie check (00ec408c) is not translated; the EDID and MNAM buffers from _RtlCheckStack12 (00ec5ec0, the _alloca) are with_stack blocks of the chunk size (their alignment, and the stack growing with each such chunk, are not modelled)
/// `TESWaterForm::Load` (Xbox PDB): false unless the record is a `WATR`
/// (type 0x4E). Otherwise `TESForm::LoadForm`, clears form flag 0x8 and
/// reads every chunk:
/// - `EDID`: into a stack buffer of the chunk's size (at most 0x200 bytes),
///   given to vtable slot 0x134; `OBND`: vtable slot 0xE0 with the file;
///   `FULL`: the `TESFullName` (`00487050`, null when `this` is null).
/// - `DATA`: when the chunk's size equals `00486620`'s, `LoadData` with no
///   destination (0, 0), then the shader data's byte swap when the file
///   swaps; otherwise `LoadData` into the shader data (0xC4 bytes), the
///   swap, the six height/amplitude floats set to their defaults
///   (`01016410` three times, 1.0, `01016248`, `0101622c`) and
///   `0047ff50(0)` on the attack-damage base.
/// - `DNAM`: 0xC4 bytes into the shader data, and the swap.
/// - `ANAM`/`FNAM`: one byte into alpha / flags; `NNAM`: the noise
///   texture; `MNAM`: the material string (a stack buffer of the chunk's
///   size, read with limit 0, then set); `SNAM`: a word given to
///   `004febd0`; `XNAM`: a word stored as the spell (`0057f890`); `GNAM`:
///   0xC bytes into the weather forms, each word swapped (`00401080` with
///   0) when the file swaps.
///
/// Returns true. The compiler's switch on `chunk - 'GNAM'` (a byte table
/// over 0..0x11 and five cases) is the match below.
pub fn teswaterform_load(e: &mut Engine, this: Ptr<TESWaterForm>, file: Ptr) -> bool {
    let record = e.call(FILE_GET_FORM_TYPE, &args![file]).u8();
    if u32::from(record) != RECORD_WATR {
        return false;
    }
    let base = this.addr();
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG_8, &args![this, false]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                let full_name = if this.is_null() {
                    0
                } else {
                    base + WATER_FULL_NAME
                };
                e.call(FULL_NAME_LOAD, &args![full_name, file]);
            }
            SIG_DATA => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let expected = e.call(FORM_DATA_SIZE, &args![this]).u32();
                if size == expected {
                    e.call(FORM_LOAD_DATA, &args![this, file, 0u32, 0u32]);
                    if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                        e.call(SHADER_DATA_SWAP, &args![base + WATER_DATA]);
                    }
                } else {
                    e.call(
                        FORM_LOAD_DATA,
                        &args![this, file, base + WATER_DATA, 0xc4u32],
                    );
                    if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                        e.call(SHADER_DATA_SWAP, &args![base + WATER_DATA]);
                    }
                    let scale = base + WATER_HEIGHT_UV_SCALE;
                    copy_float(e, 0x0101_6410, scale);
                    copy_float(e, 0x0101_6410, scale + 4);
                    copy_float(e, 0x0101_6410, scale + 8);
                    e.mem.set_f32(scale + 0xc, 1.0);
                    copy_float(e, 0x0101_6248, scale + 0x10);
                    copy_float(e, 0x0101_622c, scale + 0x14);
                    e.call(ATTACK_DAMAGE_SET, &args![base + WATER_ATTACK_DAMAGE, 0u32]);
                }
            }
            SIG_DNAM => {
                e.call(
                    FILE_GET_CHUNK_DATA_SIZED,
                    &args![file, base + WATER_DATA, 0xc4u32],
                );
                if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                    e.call(SHADER_DATA_SWAP, &args![base + WATER_DATA]);
                }
            }
            SIG_ANAM => {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, base + 0x70, 1u32]);
            }
            SIG_FNAM => {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, base + 0x71, 1u32]);
            }
            SIG_GNAM => {
                e.call(
                    FILE_GET_CHUNK_DATA_SIZED,
                    &args![file, base + WATER_WEATHER_CONTROL, 0xcu32],
                );
                if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                    for i in 0..3u32 {
                        e.call(
                            SWAP_WORD,
                            &args![base + WATER_WEATHER_CONTROL + 4 * i, 0u32],
                        );
                    }
                }
            }
            SIG_MNAM => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, 0u32]);
                    e.call(STRING_SET, &args![base + WATER_MATERIAL_ID, buffer, 0u32]);
                });
            }
            SIG_NNAM => {
                e.call(TEXTURE_LOAD_CHUNK, &args![base + WATER_NOISE_TEXTURE, file]);
            }
            SIG_SNAM => {
                let word = chunk_word(e, file);
                e.call(SNAM_STORE, &args![this, word]);
            }
            SIG_XNAM => {
                let word = chunk_word(e, file);
                fn_0057f890(e, this, word);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0057f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `spell` as the form's spell (`pSpellItem`, +0x178).
pub fn fn_0057f890(e: &mut Engine, this: Ptr<TESWaterForm>, spell: u32) {
    e.set(this, TESWaterForm::pSpellItem, spell);
}

// Translated from 0057f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm::InitItem` (Xbox PDB): unless form flag 0x8 is set,
/// resolves the form ids `Load` left: the sound (a `TESSound`), the spell
/// (a `SpellItem`) and the three weather forms (`TESWaterForm`s), each
/// through the compile index, the look-up and the cast, with a
/// "MASTERFILE: Could not find ..." message for a missing one. With a
/// spell whose `008256d0` (on the spell + 0x28) is false, walks its effect
/// list (from the spell + 0x24; the item of a node is `*006815c0(node +
/// 4)`, the next node `00726070(node + 4) - 4`, or none for 0) until both
/// were found: the first effect whose `00403ea0` is 0x36 and whose
/// `00403e60(00825c00(effect), 4)` is true gives `iRadiationDamage`
/// (`EffectItem::GetMagnitude`, `00404390`); one whose `00403ea0` is 0x10
/// and whose `00403e60(..)` is false gives `iHealthRestoration`. Finally
/// sets form flag 0x8.
pub fn teswaterform_init_item(e: &mut Engine, this: Ptr<TESWaterForm>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    if e.get(this, TESWaterForm::pWaterSound) != 0 {
        resolve_form(e, this, base + 0x7c, TYPE_TES_SOUND, MISSING_SOUND);
    }
    if e.get(this, TESWaterForm::pSpellItem) != 0 {
        resolve_form(e, this, base + 0x178, TYPE_SPELL_ITEM, MISSING_SPELL);
        let spell = e.get(this, TESWaterForm::pSpellItem);
        if spell != 0 && !e.call(0x0082_56d0, &args![spell + 0x28]).bool() {
            let mut node = spell + 0x24;
            let mut radiation = 0u32;
            let mut health = 0u32;
            while node != 0 && (radiation == 0 || health == 0) {
                let item_at = e.call(0x0068_15c0, &args![node + 4]).u32();
                let effect = e.mem.u32(item_at);
                let mut matched = false;
                if e.call(0x0040_3ea0, &args![effect]).u32() == 0x36 {
                    let setting = e.call(0x0082_5c00, &args![effect]).u32();
                    if e.call(0x0040_3e60, &args![setting, 4u32]).bool() {
                        radiation = effect;
                        let magnitude = e.call(0x0040_4390, &args![radiation]).u32();
                        e.set(this, TESWaterForm::iRadiationDamage, magnitude);
                        matched = true;
                    }
                }
                if !matched && e.call(0x0040_3ea0, &args![effect]).u32() == 0x10 {
                    let setting = e.call(0x0082_5c00, &args![effect]).u32();
                    if !e.call(0x0040_3e60, &args![setting, 4u32]).bool() {
                        health = effect;
                        let magnitude = e.call(0x0040_4390, &args![health]).u32();
                        e.set(this, TESWaterForm::iHealthRestoration, magnitude);
                    }
                }
                let next = e.call(0x0072_6070, &args![node + 4]).u32();
                node = if next != 0 { next.wrapping_sub(4) } else { 0 };
            }
        }
    }
    for i in 0..3u32 {
        let field = base + WATER_WEATHER_CONTROL + 4 * i;
        if e.mem.u32(field) != 0 {
            resolve_form(e, this, field, TYPE_TES_WATER_FORM, MISSING_WATER_FORM);
        }
    }
    e.call(FORM_SET_FLAG_8, &args![this, true]);
}

// Translated from 00580100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The alpha as a fraction: `cAlpha / *(double *)01017a40`, rounded to a
/// `float`.
pub fn fn_00580100(e: &mut Engine, this: Ptr<TESWaterForm>) -> f32 {
    let alpha = i32::from(e.get(this, TESWaterForm::cAlpha));
    let divisor: f64 = e.global(0x0101_7a40);
    f32::from_bits(x87_store32(x87_div(f64::from(alpha), divisor)))
}

// Translated from 005802d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x10 set or cleared (`004fc990`).
pub fn fn_005802d0(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x10u32, on]);
}

// Translated from 005802f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x20 set or cleared (`004fc990`).
pub fn fn_005802f0(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x20u32, on]);
}

// Translated from 00580540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x2 set or cleared (`004fc990`).
pub fn fn_00580540(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x2u32, on]);
}

// Translated from 00580560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x80 set or cleared (`004fc990`).
pub fn fn_00580560(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x80u32, on]);
}

// Translated from 00580580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x100 set or cleared (`004fc990`).
pub fn fn_00580580(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x100u32, on]);
}

// Translated from 005805a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater` flag 0x8000000 set or cleared (`004fc990`).
pub fn fn_005805a0(e: &mut Engine, this: Ptr, on: u8) {
    e.call(PLACEABLE_SET_FLAG, &args![this, 0x0800_0000u32, on]);
}

/// The LOD placeable water, as `GetPlaceableLODWater` reloads it before each
/// call.
fn lod_water(e: &Engine, this: Ptr<TESWaterForm>) -> Ptr {
    Ptr::new(e.get(this, TESWaterForm::pPlaceableLODWater))
}

// Translated from 00580330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00580330 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
/// `TESWaterForm::GetPlaceableLODWater` (Xbox PDB): the form's LOD
/// `BGSPlaceableWater`, made on the first call: 0x50 bytes (`00401000`),
/// constructed (`BGSPlaceableWater::BGSPlaceableWater`, `004fc1b0`; null
/// when the allocation fails), made temporary (`TESForm::SetTemporary`,
/// `00484490`), its flags set (0x8000000 on, `004fc360(1)`,
/// `004fc3e0(0)`, 0x2 off, `004fc380(0)`, `004fc3a0(1)`, 0x10 on, 0x20
/// off, `004fc3c0(1)`, 0x80 off, 0x100 off), named "PlaceableLODWater"
/// (copied into a 0x104-byte stack buffer by `00406d00`, given to vtable
/// slot 0x134) and given this water form (`00589f50`).
pub fn teswaterform_get_placeable_lod_water(e: &mut Engine, this: Ptr<TESWaterForm>) -> u32 {
    if e.get(this, TESWaterForm::pPlaceableLODWater) == 0 {
        let block = e.call(0x0040_1000, &args![0x50u32]).u32();
        let water = if block != 0 {
            e.call(0x004f_c1b0, &args![block]).u32()
        } else {
            0
        };
        e.set(this, TESWaterForm::pPlaceableLODWater, water);
        let p = lod_water(e, this);
        e.call(0x0048_4490, &args![p]);
        let p = lod_water(e, this);
        fn_005805a0(e, p, 1);
        let p = lod_water(e, this);
        e.call(0x004f_c360, &args![p, true]);
        let p = lod_water(e, this);
        e.call(0x004f_c3e0, &args![p, false]);
        let p = lod_water(e, this);
        fn_00580540(e, p, 0);
        let p = lod_water(e, this);
        e.call(0x004f_c380, &args![p, false]);
        let p = lod_water(e, this);
        e.call(0x004f_c3a0, &args![p, true]);
        let p = lod_water(e, this);
        fn_005802d0(e, p, 1);
        let p = lod_water(e, this);
        fn_005802f0(e, p, 0);
        let p = lod_water(e, this);
        e.call(0x004f_c3c0, &args![p, true]);
        let p = lod_water(e, this);
        fn_00580560(e, p, 0);
        let p = lod_water(e, this);
        fn_00580580(e, p, 0);
        e.with_stack(0x104, |e, name| {
            e.call(
                0x0040_6d00,
                &args![name, 0x104u32, PLACEABLE_LOD_WATER_NAME],
            );
            let p = lod_water(e, this);
            e.vcall(p.addr(), 0x134, &args![name]);
        });
        let p = lod_water(e, this);
        e.call(0x0058_9f50, &args![p, this]);
    }
    e.get(this, TESWaterForm::pPlaceableLODWater)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0057_eba0, fn_0057eba0(Ptr) -> Ptr),
        entry!(0x0057_ede0, fn_0057ede0(Ptr, u32) -> Ptr),
        entry!(0x0057_eef0, fn_0057eef0(Ptr<TESWaterForm>) -> Ptr),
        entry!(0x0057_e9a0, fn_0057e9a0(Ptr) -> Ptr),
        entry!(0x0057_ef10, fn_0057ef10(Ptr<TESWaterForm>)),
        entry!(0x0057_ee10, fn_0057ee10(Ptr<TESWaterForm>)),
        entry!(
            0x0057_ef80,
            teswaterform_load(Ptr<TESWaterForm>, Ptr) -> bool
        ),
        entry!(0x0057_f890, fn_0057f890(Ptr<TESWaterForm>, u32)),
        entry!(0x0057_f8b0, teswaterform_init_item(Ptr<TESWaterForm>)),
        entry!(0x0058_0100, fn_00580100(Ptr<TESWaterForm>) -> f32),
        entry!(0x0058_02d0, fn_005802d0(Ptr, u8)),
        entry!(0x0058_02f0, fn_005802f0(Ptr, u8)),
        entry!(0x0058_0540, fn_00580540(Ptr, u8)),
        entry!(0x0058_0560, fn_00580560(Ptr, u8)),
        entry!(0x0058_0580, fn_00580580(Ptr, u8)),
        entry!(0x0058_05a0, fn_005805a0(Ptr, u8)),
        entry!(
            0x0058_0330,
            teswaterform_get_placeable_lod_water(Ptr<TESWaterForm>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0057eba0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0057_eba0,
            size: 0x194,
            globals: &[(0x0101_6248, 0x0100_0001), (0x0101_62c0, 0x0100_0002)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0047_fdc0, &[A::T(0x24)]),
                (0x0063_3c90, &[A::T(0x30), A::V(0x0)]),
                (
                    0x0041_4430,
                    &[A::T(0x34), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (
                    0x0041_4430,
                    &[A::T(0x44), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (
                    0x0041_4430,
                    &[A::T(0x54), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (0x0048_e270, &[A::T(0x64)]),
                (0x0040_37b0, &[A::T(0x74)]),
                (0x0057_e9a0, &[A::T(0x84)]),
                (0x0063_3c90, &[A::T(0x17c), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4e)]),
                (0x0057_ef10, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_140c),
                (0x18, 4, 0x0103_13f4),
                (0x24, 4, 0x0103_13dc),
                (0x154, 4, 0x0),
                (0x158, 4, 0x0),
                (0x15c, 4, 0x9),
                (0x160, 4, 0x9),
                (0x164, 4, 0x4),
                (0x168, 4, 0x0100_0001),
                (0x16c, 4, 0x0100_0002),
                (0x170, 4, 0x0100_0001),
                (0x174, 4, 0x0100_0001),
                (0x178, 4, 0x0),
                (0x2c, 1, 0x1),
                (0x190, 1, 0x0),
                (0x180, 4, 0x0),
                (0x184, 4, 0x0),
                (0x188, 4, 0x0),
                (0x18c, 4, 0x0),
                (0x80, 4, 0x0),
            ],
        });
    }
    #[test]
    fn at_0057ede0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0057_ee10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0057_ede0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0057_ede0)
                .collect();
            let mut expected = vec![(0x0057_ee10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(e: &mut Engine, skip: u32) -> Log {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn b020_0057eef0_is_the_shader_data_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0057_eef0, &args![0x2100u32]).u32(), 0x2184);
    }

    #[test]
    fn b020_0057e9a0_sets_the_defaults_and_returns_this() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0057_e9c0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0057_e9a0, &args![0x3000u32]).u32(), 0x3000);
        assert_eq!(
            calls(&mut e, 0x0057_e9a0),
            vec![(0x0057_e9c0, vec![0x3000])]
        );
    }

    #[test]
    fn b020_0057ef10_sets_the_start_values() {
        let mut e = Engine::new();
        nop(&mut e, &[STRING_SET, 0x0057_e9c0]);
        let this = e.new_object::<TESWaterForm>();
        let base = this.addr();
        for i in 0..3 {
            e.mem.set_u32(base + 0x148 + 4 * i, 7);
        }
        e.mem.set_u32(base + 0x7c, 9);
        e.call_log = Some(vec![]);
        e.call(0x0057_ef10, &args![this]);
        assert_eq!(e.mem.u8(base + 0x70), 0x4b);
        assert_eq!(e.mem.u8(base + 0x71), 0);
        assert_eq!(e.mem.u32(base + 0x7c), 0);
        for i in 0..3 {
            assert_eq!(e.mem.u32(base + 0x148 + 4 * i), 0);
        }
        assert_eq!(
            calls(&mut e, 0x0057_ef10),
            vec![
                (STRING_SET, vec![base + 0x74, 0, 0]),
                (0x0057_e9c0, vec![base + 0x84]),
            ]
        );
    }

    #[test]
    fn b020_0057ee10_destroys_the_members_in_order() {
        let mut e = Engine::new();
        let callees = [
            0x0066_b0d0,
            0x0045_cec0,
            0x0040_37d0,
            0x0048_e2e0,
            0x0047_fdf0,
            0x0059_1300,
            0x0048_3630,
        ];
        nop(&mut e, &callees);
        let this = e.new_object::<TESWaterForm>();
        let b = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x0057_ee10, &args![this]);
        assert_eq!(e.mem.u32(b), 0x0103_140c);
        assert_eq!(e.mem.u32(b + 0x18), 0x0103_13f4);
        assert_eq!(e.mem.u32(b + 0x24), 0x0103_13dc);
        assert_eq!(
            calls(&mut e, 0x0057_ee10),
            vec![
                (0x0066_b0d0, vec![b + 0x17c, 0]),
                (0x0045_cec0, vec![b + 0x17c]),
                (0x0040_37d0, vec![b + 0x74]),
                (0x0048_e2e0, vec![b + 0x64]),
                (0x0045_cec0, vec![b + 0x30]),
                (0x0047_fdf0, vec![b + 0x24]),
                (0x0059_1300, vec![b + 0x18]),
                (0x0048_3630, vec![b]),
            ]
        );
    }

    /// An engine whose file hands out `chunks` one after the other (the
    /// next-chunk call answers false after the last), with `size` as every
    /// chunk's size, `expected` as `00486620`'s and `swaps` as the flag.
    fn load_engine(chunks: Vec<u32>, size: u32, expected: u32, swaps: bool) -> Engine {
        let mut e = Engine::new();
        let queue = Rc::new(RefCell::new(chunks));
        let q = queue.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| {
            let v = q.borrow().first().copied().unwrap_or(0);
            ret(v)
        });
        let q = queue;
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut v = q.borrow_mut();
            v.remove(0);
            ret(u32::from(!v.is_empty()))
        });
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x4e));
        e.register_double(FILE_CHUNK_SIZE, move |_, _| ret(size));
        e.register_double(FORM_DATA_SIZE, move |_, _| ret(expected));
        e.register_double(FILE_SWAPS_BYTES, move |_, _| ret(u32::from(swaps)));
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0abc);
            Ret::default()
        });
        nop(
            &mut e,
            &[
                FORM_LOAD_FORM,
                FORM_SET_FLAG_8,
                FILE_GET_CHUNK_DATA_SIZED,
                FORM_LOAD_DATA,
                FULL_NAME_LOAD,
                TEXTURE_LOAD_CHUNK,
                STRING_SET,
                SWAP_WORD,
                SHADER_DATA_SWAP,
                ATTACK_DAMAGE_SET,
                SNAM_STORE,
                0x0000_1134,
                0x0000_10e0,
            ],
        );
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0000 + 0x134, 0x0000_1134);
        e.mem.set_u32(0x0150_0000 + 0xe0, 0x0000_10e0);
        e
    }

    fn water(e: &mut Engine) -> Ptr<TESWaterForm> {
        let this = e.new_object::<TESWaterForm>();
        e.mem.set_u32(this.addr(), 0x0150_0000);
        this
    }

    #[test]
    fn b020_0057ef80_refuses_other_records() {
        let mut e = load_engine(vec![], 0, 0, false);
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x14f));
        let this = water(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0057_ef80, &args![this, 0x9000u32]).bool());
        assert_eq!(
            calls(&mut e, 0x0057_ef80),
            vec![(FILE_GET_FORM_TYPE, vec![0x9000])]
        );
    }

    #[test]
    fn b020_0057ef80_reads_each_chunk() {
        let chunks = vec![
            SIG_EDID,
            SIG_OBND,
            SIG_FULL,
            SIG_DNAM,
            SIG_ANAM,
            SIG_FNAM,
            SIG_GNAM,
            SIG_MNAM,
            SIG_NNAM,
            SIG_SNAM,
            SIG_XNAM,
            0x1234_5678,
        ];
        let mut e = load_engine(chunks, 8, 4, true);
        let this = water(&mut e);
        let b = this.addr();
        let file = 0x9000u32;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0057_ef80, &args![this, file]).bool());
        assert_eq!(calls_to(&e, FORM_LOAD_FORM), vec![vec![b, file]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG_8), vec![vec![b, 0]]);
        let sized = calls_to(&e, FILE_GET_CHUNK_DATA_SIZED);
        assert_eq!(sized.len(), 6);
        assert_eq!(sized[0][2], 0x200);
        assert_eq!(sized[1], vec![file, b + 0x84, 0xc4]);
        assert_eq!(sized[2], vec![file, b + 0x70, 1]);
        assert_eq!(sized[3], vec![file, b + 0x71, 1]);
        assert_eq!(sized[4], vec![file, b + 0x148, 0xc]);
        assert_eq!(sized[5][2], 0);
        assert_eq!(calls_to(&e, 0x0000_1134), vec![vec![b, sized[0][1]]]);
        assert_eq!(calls_to(&e, 0x0000_10e0), vec![vec![b, file]]);
        assert_eq!(calls_to(&e, FULL_NAME_LOAD), vec![vec![b + 0x18, file]]);
        assert_eq!(calls_to(&e, SHADER_DATA_SWAP), vec![vec![b + 0x84]]);
        assert_eq!(
            calls_to(&e, SWAP_WORD),
            vec![vec![b + 0x148, 0], vec![b + 0x14c, 0], vec![b + 0x150, 0]]
        );
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![b + 0x74, sized[5][1], 0]]
        );
        assert_eq!(calls_to(&e, TEXTURE_LOAD_CHUNK), vec![vec![b + 0x64, file]]);
        assert_eq!(calls_to(&e, SNAM_STORE), vec![vec![b, 0xabc]]);
        assert_eq!(e.get(this, TESWaterForm::pSpellItem), 0xabc);
        assert_eq!(calls_to(&e, FILE_NEXT_CHUNK).len(), 12);
    }

    #[test]
    fn b020_0057ef80_data_chunk_of_the_expected_size_loads_nothing() {
        let mut e = load_engine(vec![SIG_DATA], 4, 4, true);
        let this = water(&mut e);
        let b = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x0057_ef80, &args![this, 0x9000u32]);
        assert_eq!(calls_to(&e, FORM_LOAD_DATA), vec![vec![b, 0x9000, 0, 0]]);
        assert_eq!(calls_to(&e, SHADER_DATA_SWAP), vec![vec![b + 0x84]]);
        assert!(calls_to(&e, ATTACK_DAMAGE_SET).is_empty());
    }

    #[test]
    fn b020_0057ef80_data_chunk_of_another_size_sets_the_defaults() {
        let mut e = load_engine(vec![SIG_DATA], 8, 4, false);
        e.map(0x0101_6000, 0x1000);
        e.set_global::<u32>(0x0101_6410, 0x3f00_0000);
        e.set_global::<u32>(0x0101_6248, 0x7f80_0001);
        e.set_global::<u32>(0x0101_622c, 0x4000_0000);
        let this = water(&mut e);
        let b = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x0057_ef80, &args![this, 0x9000u32]);
        assert_eq!(
            calls_to(&e, FORM_LOAD_DATA),
            vec![vec![b, 0x9000, b + 0x84, 0xc4]]
        );
        assert!(calls_to(&e, SHADER_DATA_SWAP).is_empty());
        let floats: Vec<u32> = (0..6).map(|i| e.mem.u32(b + 0x130 + 4 * i)).collect();
        assert_eq!(
            floats,
            vec![
                0x3f00_0000,
                0x3f00_0000,
                0x3f00_0000,
                0x3f80_0000,
                0x7fc0_0001,
                0x4000_0000
            ]
        );
        assert_eq!(calls_to(&e, ATTACK_DAMAGE_SET), vec![vec![b + 0x24, 0]]);
    }

    #[test]
    fn b020_0057f890_stores_the_spell() {
        let mut e = Engine::new();
        let this = e.new_object::<TESWaterForm>();
        e.call(0x0057_f890, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x178), 0x55);
    }

    fn init_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0x77));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids ending in 0 are missing; others are found as id + 0x10.
        e.register(FORM_BY_ID, |_, a| {
            ret(if a[0] & 0xf == 0 { 0 } else { a[0] + 0x10 })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_ID, |_, _| ret(0x0400_0001));
        e.register(0x0000_1130, |_, _| ret(0x0000_5000));
        nop(&mut e, &[PRINT, FORM_SET_FLAG_8]);
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0000 + 0x130, 0x0000_1130);
        e
    }

    #[test]
    fn b020_0057f8b0_does_nothing_once_initialized() {
        let mut e = init_engine();
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        let this = water(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0057_f8b0, &args![this]);
        assert_eq!(
            calls(&mut e, 0x0057_f8b0),
            vec![(FORM_FLAG_8_SET, vec![this.addr()])]
        );
    }

    #[test]
    fn b020_0057f8b0_resolves_forms_and_reports_missing_ones() {
        let mut e = init_engine();
        let this = water(&mut e);
        let b = this.addr();
        e.mem.set_u32(b + 0x7c, 0x20);
        e.mem.set_u32(b + 0x148, 0x31);
        e.mem.set_u32(b + 0x150, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0057_f8b0, &args![this]);
        assert_eq!(e.mem.u32(b + 0x7c), 0);
        assert_eq!(e.mem.u32(b + 0x148), 0x0100_0041);
        assert_eq!(e.mem.u32(b + 0x150), 0);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![
                vec![MISSING_SOUND, 0x0100_0020, 0x5000, 0x0400_0001],
                vec![MISSING_WATER_FORM, 0x0100_0040, 0x5000, 0x0400_0001],
            ]
        );
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts[0], vec![0, 0, TYPE_TES_FORM, TYPE_TES_SOUND, 0]);
        assert_eq!(casts[1][3], TYPE_TES_WATER_FORM);
        assert_eq!(calls_to(&e, FORM_GET_FILE)[0], vec![b, u32::MAX]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG_8), vec![vec![b, 1]]);
    }

    #[test]
    fn b020_0057f8b0_reads_the_spell_effects() {
        let mut e = init_engine();
        let this = water(&mut e);
        let b = this.addr();
        // The spell: id 0x01 resolves to 0x0100_0011, a block of the engine.
        e.map(0x0100_0000, 0x1000);
        let spell = 0x0100_0011u32;
        e.mem.set_u32(b + 0x178, 0x01);
        e.register(0x0082_56d0, |_, _| ret(0));
        // Nodes at 0x0100_0100 (+0x24 of the spell is the first), items
        // through `006815c0` (the word at node + 4 + 4 holds the item's
        // address), next through `00726070` (node + 4 + 0xc).
        let first = spell + 0x24;
        let second = 0x0100_0200u32;
        e.mem.set_u32(first + 8, 0x0100_0300);
        e.mem.set_u32(first + 0x10, second + 4);
        e.mem.set_u32(second + 8, 0x0100_0304);
        e.mem.set_u32(second + 0x10, 0);
        e.mem.set_u32(0x0100_0300, 0x0000_0a00);
        e.mem.set_u32(0x0100_0304, 0x0000_0b00);
        e.register(0x0068_15c0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        // Effect 0xa00: archetype 0x36 with the setting flag; 0xb00: 0x10
        // without it.
        e.register(0x0040_3ea0, |_, a| {
            ret(if a[0] == 0xa00 { 0x36 } else { 0x10 })
        });
        e.register(0x0082_5c00, |_, a| ret(a[0] + 1));
        e.register(0x0040_3e60, |_, a| ret(u32::from(a[0] == 0xa01)));
        e.register(0x0040_4390, |_, a| ret(a[0] + 5));
        e.call_log = Some(vec![]);
        e.call(0x0057_f8b0, &args![this]);
        assert_eq!(e.mem.u32(b + 0x178), spell);
        assert_eq!(e.get(this, TESWaterForm::iRadiationDamage), 0xa05);
        assert_eq!(e.get(this, TESWaterForm::iHealthRestoration), 0xb05);
        assert_eq!(calls_to(&e, 0x0082_56d0), vec![vec![spell + 0x28]]);
        assert_eq!(
            calls_to(&e, 0x0040_3ea0),
            vec![vec![0xa00], vec![0xb00], vec![0xb00]]
        );
        assert_eq!(
            calls_to(&e, 0x0072_6070),
            vec![vec![first + 4], vec![second + 4]]
        );
        assert!(calls_to(&e, PRINT).is_empty());
    }

    #[test]
    fn b020_00580100_divides_the_alpha() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global::<f64>(0x0101_7a40, 255.0);
        let this = e.new_object::<TESWaterForm>();
        e.set(this, TESWaterForm::cAlpha, 0x80);
        let r = e.call(0x0058_0100, &args![this]).f32();
        assert_eq!(r, (128.0f64 / 255.0) as f32);
    }

    #[test]
    fn b020_placeable_water_flag_setters_pass_their_masks() {
        let mut e = Engine::new();
        nop(&mut e, &[PLACEABLE_SET_FLAG]);
        let cases = [
            (0x0058_02d0, 0x10),
            (0x0058_02f0, 0x20),
            (0x0058_0540, 0x2),
            (0x0058_0560, 0x80),
            (0x0058_0580, 0x100),
            (0x0058_05a0, 0x0800_0000),
        ];
        for (addr, mask) in cases {
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x4000u32, 1u32]);
            e.call(addr, &args![0x4000u32, 0x100u32]);
            assert_eq!(
                calls_to(&e, PLACEABLE_SET_FLAG),
                vec![vec![0x4000, mask, 1], vec![0x4000, mask, 0]]
            );
        }
    }

    #[test]
    fn b020_00580330_returns_an_existing_lod_water() {
        let mut e = Engine::new();
        let this = e.new_object::<TESWaterForm>();
        e.set(this, TESWaterForm::pPlaceableLODWater, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_0330, &args![this]).u32(), 0x6000);
        assert!(calls(&mut e, 0x0058_0330).is_empty());
    }

    #[test]
    fn b020_00580330_makes_the_lod_water() {
        let mut e = Engine::new();
        e.register(0x0040_1000, |_, _| ret(0x0160_0000));
        e.register(0x004f_c1b0, |_, a| ret(a[0]));
        e.register(0x0040_6d00, |e, a| {
            e.mem.set_u32(a[0], 0x6c50);
            Ret::default()
        });
        nop(
            &mut e,
            &[
                0x0048_4490,
                0x004f_c360,
                0x004f_c3e0,
                0x004f_c380,
                0x004f_c3a0,
                0x004f_c3c0,
                PLACEABLE_SET_FLAG,
                0x0058_9f50,
                0x0000_1134,
            ],
        );
        e.map(0x0160_0000, 0x100);
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0000 + 0x134, 0x0000_1134);
        e.mem.set_u32(0x0160_0000, 0x0150_0000);
        let this = e.new_object::<TESWaterForm>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_0330, &args![this]).u32(), 0x0160_0000);
        let p = 0x0160_0000u32;
        let log = calls(&mut e, 0x0058_0330);
        let name = log.iter().find(|c| c.0 == 0x0040_6d00).unwrap().1[0];
        assert_eq!(
            log,
            vec![
                (0x0040_1000, vec![0x50]),
                (0x004f_c1b0, vec![p]),
                (0x0048_4490, vec![p]),
                (PLACEABLE_SET_FLAG, vec![p, 0x0800_0000, 1]),
                (0x004f_c360, vec![p, 1]),
                (0x004f_c3e0, vec![p, 0]),
                (PLACEABLE_SET_FLAG, vec![p, 0x2, 0]),
                (0x004f_c380, vec![p, 0]),
                (0x004f_c3a0, vec![p, 1]),
                (PLACEABLE_SET_FLAG, vec![p, 0x10, 1]),
                (PLACEABLE_SET_FLAG, vec![p, 0x20, 0]),
                (0x004f_c3c0, vec![p, 1]),
                (PLACEABLE_SET_FLAG, vec![p, 0x80, 0]),
                (PLACEABLE_SET_FLAG, vec![p, 0x100, 0]),
                (0x0040_6d00, vec![name, 0x104, PLACEABLE_LOD_WATER_NAME]),
                (0x0000_1134, vec![p, name]),
                (0x0058_9f50, vec![p, this.addr()]),
            ]
        );
    }

    #[test]
    fn b020_00580330_keeps_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(0x0040_1000, |_, _| ret(0));
        e.register(0x0048_4490, |_, a| {
            assert_eq!(a[0], 0);
            Ret::default()
        });
        nop(
            &mut e,
            &[
                0x004f_c360,
                0x004f_c3e0,
                0x004f_c380,
                0x004f_c3a0,
                0x004f_c3c0,
                PLACEABLE_SET_FLAG,
                0x0040_6d00,
            ],
        );
        let this = e.new_object::<TESWaterForm>();
        // With no water every setter gets null, and the virtual call reads
        // the vtable at address 0, where the exe faults too.
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x0058_0330, &args![this]);
        }));
        assert!(r.is_err());
    }
}
