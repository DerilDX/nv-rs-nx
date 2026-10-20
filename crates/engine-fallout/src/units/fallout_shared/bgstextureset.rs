//! `fallout shared/bgstextureset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005922e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005922e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSTextureSet::BGSTextureSet` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 4 (0xa0 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgstextureset_bgstextureset(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0051_d190, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0103_3d1c);
    e.mem.set_u32(base + 0x30, 0x0103_3c7c);
    e.call(
        0x00ec_782f,
        &args![this, base + 0x38, 0xc, 0x6, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(0x0059_2450, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x4]);
    this
}

// Translated from 00592380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `005923c0` and frees 0xa0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_00592380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_23c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xa0u32]);
    }
    this
}

/// The six texture paths (`TESTexture`s of 0xc bytes) from +0x38.
const TEXTURES: u32 = 0x38;
/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// `007b3fa0` (`ECX`, one stack word): a folded scalar deleting destructor.
const DELETE_007B3FA0: u32 = 0x007b_3fa0;
/// `` `eh vector destructor iterator' `` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor) and the `TESTexture` destructor it is given (`0048e2e0`).
const VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
const TEXTURE_DESTRUCTOR: u32 = 0x0048_e2e0;
/// The null texture set (`011ca898`).
const NULL_TEXTURE_SET: u32 = 0x011c_a898;
/// The Gamebryo allocator (`00aa13e0`, `cdecl`, one word: the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `"NullTextureSet"` and `"Data\Textures\"`.
const NULL_TEXTURE_SET_NAME: u32 = 0x0103_3eac;
const TEXTURES_DIRECTORY: u32 = 0x0103_3e9c;
/// The data handler (`011c3f2c`) and `TESDataHandler::AddFormToDataHandler` (Xbox PDB,
/// `004603b0`, `ECX` = data handler, one stack word).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
/// The model loader (`011c3b3c`) and `ModelLoader::QueueTexture` by file entry (Xbox PDB,
/// `00443af0`, `ECX`, three stack words).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const QUEUE_TEXTURE: u32 = 0x0044_3af0;
/// `TESSoundFile::SetSoundFile` (engine map, `00489100`; by its body: copies a C string into
/// the `BSStringT` at `this + 4`), `thiscall`, one stack word.
const SET_STRING_00489100: u32 = 0x0048_9100;
/// `00408da0` (`ECX`): the text of the string at +4, or the empty string.
const STRING_CHARACTERS_00408DA0: u32 = 0x0040_8da0;
/// `BSShaderTextureSet::BSShaderTextureSet` (Xbox PDB, `0051d0d0`, `ECX`).
const SHADER_TEXTURE_SET_CTOR: u32 = 0x0051_d0d0;
/// `strcpy_s` (`00406d30`) and `strcat_s` (`00406d50`), `cdecl`: destination, size, source.
const STRCPY_S: u32 = 0x0040_6d30;
const STRCAT_S: u32 = 0x0040_6d50;
/// The smart pointer constructor (`00633c90`, `ECX`, one stack word), destructor (`0045cec0`,
/// `ECX`) and assignment (`006e5cc0`, `ECX`, one stack word: the source's address).
const NI_POINTER_FROM: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size) and `GetChunkData_ov2` (`00472840`, one stack word: a `u16`).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const GET_CHUNK_DATA_U16: u32 = 0x0047_2840;
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `00593240` (`ECX`): fills a new 0x24-byte decal record with its defaults.
const DECAL_DATA_CTOR: u32 = 0x0059_3240;
/// `00593450` (`ECX` = the decal record, one stack word: the file): reads it.
const DECAL_DATA_LOAD: u32 = 0x0059_3450;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The archive helpers: `00af4200` (`cdecl`: path, out, size), `006815c0` (`ECX`, answers
/// `this`), `00afd270` (`cdecl`: path, directory hash, name hash) and `00af6540` (`cdecl`:
/// type index, directory hash, name hash, path), the file entry it finds.
const ARCHIVE_PATH_00AF4200: u32 = 0x00af_4200;
const IDENTITY_006815C0: u32 = 0x0068_15c0;
const PATH_HASH_00AFD270: u32 = 0x00af_d270;
const ARCHIVE_FIND_00AF6540: u32 = 0x00af_6540;
/// Chunk codes `005928a0` reads (the four characters, first byte lowest).
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_DODT: u32 = 0x5444_4f44;
const CHUNK_TX00: u32 = 0x3030_5854;
const CHUNK_TX01: u32 = 0x3130_5854;
const CHUNK_TX02: u32 = 0x3230_5854;
const CHUNK_TX03: u32 = 0x3330_5854;
const CHUNK_TX04: u32 = 0x3430_5854;
const CHUNK_TX05: u32 = 0x3530_5854;

// Translated from 00592450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTextureSet`'s vtable (`01033d1c`) slot +0x14, the `InitializeData` slot of the Xbox
/// PDB's `TESForm` (`ECX`; the constructor runs it): each of the six textures'
/// `InitializeDataComponent` (their vtable slot +0), the decal data pointer (+0x80) and the
/// flags word (+0x84) cleared, and the six file entries (+0x88) cleared (`memset`).
pub fn fn_00592450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..6u32 {
        e.vcall(base + TEXTURES + index * 0xc, 0, &args![]);
    }
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u16(base + 0x84, 0);
    e.call(MEMSET, &args![base + 0x88, 0u32, 0x18u32]);
}

// Translated from 005924d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTextureSet`'s vtable slot +0x18, the `ClearData` slot of the Xbox PDB's `TESForm`
/// (`ECX`): each texture's `ClearDataComponent` (vtable slot +4), the decal data (+0x80), when
/// there is one, deleted (`007b3fa0` with 1; the pointer is left as it is), and the flags word
/// cleared.
pub fn fn_005924d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..6u32 {
        e.vcall(base + TEXTURES + index * 0xc, 4, &args![]);
    }
    let decal = e.mem.u32(base + 0x80);
    if decal != 0 {
        e.call(DELETE_007B3FA0, &args![decal, 1u32]);
    }
    e.mem.set_u16(base + 0x84, 0);
}

// Translated from 005923c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005923c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8449, state variable) is not translated
/// The `BGSTextureSet` destructor body (`ECX`; the scalar deleting destructor `00592380`
/// runs it): the two vtables (`01033d1c` at +0, `01033c7c` at +0x30), the data cleared
/// ([`fn_005924d0`]), the six textures destroyed (the vector destructor iterator with
/// `0048e2e0`), the `BSShaderTextureSet` part (`004a7090` on +0x30) and the `TESBoundObject`
/// base (`0050e9f0`).
pub fn fn_005923c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_3d1c);
    e.mem.set_u32(base + 0x30, 0x0103_3c7c);
    fn_005924d0(e, this);
    e.call(
        VECTOR_DESTRUCTOR,
        &args![base + TEXTURES, 0xcu32, 6u32, TEXTURE_DESTRUCTOR],
    );
    e.call(0x004a_7090, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 00593140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00593140 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8542, state variable) is not translated
/// `BGSTextureSet::CreateNullTextureSet` (Xbox PDB), no words: when there is no null texture
/// set (`011ca898`) yet, one is made (the Gamebryo allocator's 0xa0 bytes and
/// `BGSTextureSet::BGSTextureSet`, null when the allocation fails) with the form id 0x28
/// (vtable slot +0x128 with 1) and the editor id `"NullTextureSet"` (slot +0x134), and filed
/// in the data handler.
pub fn bgs_texture_set_create_null_texture_set(e: &mut Engine) {
    if e.global::<u32>(NULL_TEXTURE_SET) != 0 {
        return;
    }
    let block = e.call(NI_ALLOCATE, &args![0xa0u32]).u32();
    let set = if block != 0 {
        bgstextureset_bgstextureset(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set_global(NULL_TEXTURE_SET, set);
    let set = e.global::<u32>(NULL_TEXTURE_SET);
    e.vcall(set, 0x128, &args![0x28u32, 1u32]);
    let set = e.global::<u32>(NULL_TEXTURE_SET);
    e.vcall(set, 0x134, &args![NULL_TEXTURE_SET_NAME]);
    let set = e.global::<u32>(NULL_TEXTURE_SET);
    let data_handler = e.global::<u32>(DATA_HANDLER);
    e.call(ADD_FORM_TO_DATA_HANDLER, &args![data_handler, set]);
}

// Translated from 005930e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB), `ECX` and three stack words (`RET 0xc`; the
/// third is not read): each nonzero file entry (+0x88) queued with the model loader
/// (`ModelLoader::QueueTexture` with the entry and the first two words).
pub fn bgs_texture_set_queue_texture_set(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    _unused_3: u32,
) {
    for index in 0..6u32 {
        let entry = e.mem.u32(this.addr() + 0x88 + index * 4);
        if entry != 0 {
            let loader = e.global::<u32>(MODEL_LOADER);
            let entry = e.mem.u32(this.addr() + 0x88 + index * 4);
            e.call(QUEUE_TEXTURE, &args![loader, entry, first, second]);
        }
    }
}

// Translated from 00592cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` and two stack words, `RET 8`): for `index` below 6 (signed) sets
/// that texture's path to `text` (`00489100` on `+0x38 + index * 0xc`).
pub fn fn_00592cc0(e: &mut Engine, this: Ptr, index: i32, text: Ptr) {
    if index < 6 {
        let texture = this
            .addr()
            .wrapping_add(TEXTURES)
            .wrapping_add((index as u32).wrapping_mul(0xc));
        e.call(SET_STRING_00489100, &args![texture, text]);
    }
}

// Translated from 00592cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTextureSet::GetAsShaderTextureSet` (Xbox PDB), `ECX`: its `BSShaderTextureSet` part
/// (+0x30), null for null.
pub fn bgs_texture_set_get_as_shader_texture_set(_e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 0x30
    }
}

// Translated from 00592e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x8c of the `BSShaderTextureSet` part's vtable (`01033c7c`; `ECX` is that part,
/// the texture set's +0x30; one stack word, `RET 4`): the path text of texture `index`
/// (`00408da0`; the exe's jump table in `.text`, `00592f14`, is these arms: 0 to 5 read the
/// part's +0x8, +0x14, +0x2c, +0x38, +0x44, +0x20), null for an empty path or another index.
pub fn fn_00592e70(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let string = match index {
        0 => 0x8,
        1 => 0x14,
        2 => 0x2c,
        3 => 0x38,
        4 => 0x44,
        5 => 0x20,
        _ => return 0,
    };
    let text = e
        .call(STRING_CHARACTERS_00408DA0, &args![this.addr() + string])
        .u32();
    if text != 0 && e.mem.u8(text) == 0 {
        0
    } else {
        text
    }
}

// Translated from 00592d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00592d20 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef84c2, state variable) and the /GS cookie check (00ec408c) are not translated
/// `BGSTextureSet::CreateShaderTextureSet` (Xbox PDB), `ECX`: a new `BSShaderTextureSet`
/// (the Gamebryo allocator's 0x38 bytes, `0051d0d0`, null when the allocation fails) whose
/// texture `index` is set (its vtable slot +0x94 with the index and the path) to
/// `"Data\Textures\"` + each nonempty path the part's slot +0x8c answers, built in a
/// 0x104-byte stack buffer. Answers the new set.
pub fn bgs_texture_set_create_shader_texture_set(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
    let set = if block != 0 {
        e.call(SHADER_TEXTURE_SET_CTOR, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(0x104, |e, path| {
        for index in 0..6u32 {
            let text = e.vcall(this.addr() + 0x30, 0x8c, &args![index]).u32();
            if text == 0 || e.mem.u8(text) == 0 {
                continue;
            }
            e.call(STRCPY_S, &args![path, 0x104u32, TEXTURES_DIRECTORY]);
            e.call(STRCAT_S, &args![path, 0x104u32, text]);
            e.vcall(set, 0x94, &args![index, path]);
        }
    });
    set
}

// Translated from 00592f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00592f30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef84fb, state variable) and the /GS cookie check (00ec408c) are not translated
/// `BGSTextureSet::GetTexture` (Xbox PDB; `ECX` is the `BSShaderTextureSet` part, two stack
/// words, `RET 8`: a texture index and the `NiPointer` to fill): a stack `NiPointer` (null,
/// `00633c90`); when the part's slot +0x8c answers a nonempty path, `"Data\Textures\"` + path
/// (a 0x104-byte stack buffer) goes to `BSShaderManager::GetTexture` (with 1, the stack
/// pointer, 1, 0). The result is assigned to `out` (`006e5cc0`) and the stack pointer
/// destroyed (`0045cec0`).
pub fn bgs_texture_set_get_texture(e: &mut Engine, this: Ptr, index: u32, out: Ptr) {
    e.with_stack(0x108, |e, locals| {
        let pointer = locals.addr();
        let path = locals.addr() + 4;
        e.call(NI_POINTER_FROM, &args![pointer, 0u32]);
        let text = e.vcall(this.addr(), 0x8c, &args![index]).u32();
        if text != 0 && e.mem.u8(text) != 0 {
            e.call(STRCPY_S, &args![path, 0x104u32, TEXTURES_DIRECTORY]);
            e.call(STRCAT_S, &args![path, 0x104u32, text]);
            e.call(
                SHADER_MANAGER_GET_TEXTURE,
                &args![path, 1u32, pointer, 1u32, 0u32],
            );
        }
        e.call(NI_POINTER_ASSIGN, &args![out, pointer]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

// Translated from 005928a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005928a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef847e, state variable) and the /GS cookie check (00ec408c) are not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `BGSTextureSet`'s vtable slot +0x20, the `Load` slot of the Xbox PDB's `TESForm` (`ECX`
/// and one stack word, the file, `RET 4`; no form type test): `TESForm::LoadForm`, form flag
/// 8 cleared, then each chunk: `TX00` to `TX05` (the six texture paths), `DNAM` (the flags
/// word at +0x84, `GetChunkData_ov2`), `DODT` (the decal data at +0x80, made first when there
/// is none: `00401000` of 0x24 bytes and `00593240`, null when the allocation fails; read by
/// `00593450`), `EDID` (a stack buffer of the chunk's size, read with at most 0x200 bytes,
/// given to vtable slot +0x134), `OBND` (vtable slot +0xe0 with the file). Then each nonempty
/// texture path's archive file entry (+0x88): `"Data\Textures\"` + path, made an archive path
/// (`00af4200`), hashed (`00afd270`) and looked up (`00af6540` with type index 1). True.
pub fn fn_005928a0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_TX00 | CHUNK_TX01 | CHUNK_TX02 | CHUNK_TX03 | CHUNK_TX04 | CHUNK_TX05 => {
                // The last character is the texture index.
                let index = (chunk >> 24) - u32::from(b'0');
                e.call(
                    LOAD_TEXTURE_CHUNK,
                    &args![base + TEXTURES + index * 0xc, file],
                );
            }
            CHUNK_DNAM => {
                e.call(GET_CHUNK_DATA_U16, &args![file, base + 0x84]);
            }
            CHUNK_DODT => {
                if e.mem.u32(base + 0x80) == 0 {
                    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
                    let decal = if block != 0 {
                        e.call(DECAL_DATA_CTOR, &args![block]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(base + 0x80, decal);
                }
                let decal = e.mem.u32(base + 0x80);
                e.call(DECAL_DATA_LOAD, &args![decal, file]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    // The exe's stack: directory hash (8 bytes), archive path (0x104), name hash (8 bytes),
    // full path (0x104), one after the other.
    e.with_stack(0x218, |e, locals| {
        let directory_hash = locals.addr();
        let archive_path = locals.addr() + 8;
        let name_hash = locals.addr() + 0x10c;
        let full_path = locals.addr() + 0x114;
        for index in 0..6u32 {
            let text = e
                .call(
                    STRING_CHARACTERS_00408DA0,
                    &args![base + TEXTURES + index * 0xc],
                )
                .u32();
            if text == 0 || e.mem.u8(text) == 0 {
                continue;
            }
            e.call(STRCPY_S, &args![full_path, 0x104u32, TEXTURES_DIRECTORY]);
            e.call(STRCAT_S, &args![full_path, 0x104u32, text]);
            e.call(
                ARCHIVE_PATH_00AF4200,
                &args![full_path, archive_path, 0x104u32],
            );
            e.call(IDENTITY_006815C0, &args![directory_hash]);
            e.call(IDENTITY_006815C0, &args![name_hash]);
            e.call(
                PATH_HASH_00AFD270,
                &args![archive_path, directory_hash, name_hash],
            );
            let entry = e
                .call(
                    ARCHIVE_FIND_00AF6540,
                    &args![1u32, directory_hash, name_hash, archive_path],
                )
                .u32();
            e.mem.set_u32(base + 0x88 + index * 4, entry);
        }
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_2380, fn_00592380(Ptr, u32) -> Ptr),
        entry!(0x0059_22e0, bgstextureset_bgstextureset(Ptr) -> Ptr),
        entry!(0x0059_2450, fn_00592450(Ptr)),
        entry!(0x0059_24d0, fn_005924d0(Ptr)),
        entry!(0x0059_23c0, fn_005923c0(Ptr)),
        entry!(0x0059_3140, bgs_texture_set_create_null_texture_set()),
        entry!(
            0x0059_30e0,
            bgs_texture_set_queue_texture_set(Ptr, u32, u32, u32)
        ),
        entry!(0x0059_2cc0, fn_00592cc0(Ptr, i32, Ptr)),
        entry!(0x0059_2cf0, bgs_texture_set_get_as_shader_texture_set(Ptr) -> u32),
        entry!(0x0059_2e70, fn_00592e70(Ptr, u32) -> u32),
        entry!(0x0059_2d20, bgs_texture_set_create_shader_texture_set(Ptr) -> u32),
        entry!(0x0059_2f30, bgs_texture_set_get_texture(Ptr, u32, Ptr)),
        entry!(0x0059_28a0, fn_005928a0(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgstextureset_bgstextureset_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_22e0,
            size: 0xa0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0051_d190, &[A::T(0x30)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x0),
                        A::T(0x38),
                        A::V(0xc),
                        A::V(0x6),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (0x0059_2450, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4)]),
            ],
            stores: &[(0x0, 4, 0x0103_3d1c), (0x30, 4, 0x0103_3c7c)],
        });
    }

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn fn_00592380_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0059_2380, 0x0059_23c0, 0xa0);
    }
}

#[cfg(test)]
mod texture_set_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const TEXTURE_VTABLE: u32 = 0x0f02_0000;

    /// A texture set whose six textures share a vtable with slots +0 and +4.
    fn texture_set(e: &mut Engine) -> u32 {
        e.put_vtable(TEXTURE_VTABLE, &[0x0f02_0100, 0x0f02_0104]);
        e.register(0x0f02_0100, |_, _| Ret::default());
        e.register(0x0f02_0104, |_, _| Ret::default());
        let set = e.mem.alloc(0xa0);
        for index in 0..6u32 {
            e.mem.set_u32(set + 0x38 + index * 0xc, TEXTURE_VTABLE);
        }
        set
    }

    #[test]
    fn fn_00592450_initializes_the_textures_and_fields() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, a| a[0].into_ret());
        let set = texture_set(&mut e);
        e.mem.set_u32(set + 0x80, 0x1234);
        e.mem.set_u32(set + 0x84, 0xaaaa_aaaa);
        e.call_log = Some(vec![]);
        e.call(0x0059_2450, &args![set]);
        let textures: Vec<u32> = calls(&e, 0x0f02_0100).iter().map(|c| c[0] - set).collect();
        assert_eq!(textures, vec![0x38, 0x44, 0x50, 0x5c, 0x68, 0x74]);
        assert_eq!(e.mem.u32(set + 0x80), 0);
        assert_eq!(e.mem.u32(set + 0x84), 0xaaaa_0000);
        assert_eq!(calls(&e, MEMSET), vec![vec![set + 0x88, 0, 0x18]]);
    }

    #[test]
    fn fn_005924d0_clears_and_deletes_the_decal_data() {
        let mut e = Engine::new();
        e.register(DELETE_007B3FA0, |_, _| Ret::default());
        let set = texture_set(&mut e);
        e.mem.set_u32(set + 0x80, 0x1234);
        e.mem.set_u16(set + 0x84, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0059_24d0, &args![set]);
        assert_eq!(calls(&e, 0x0f02_0104).len(), 6);
        assert_eq!(calls(&e, DELETE_007B3FA0), vec![vec![0x1234, 1]]);
        assert_eq!(e.mem.u32(set + 0x80), 0x1234);
        assert_eq!(e.mem.u16(set + 0x84), 0);
        e.mem.set_u32(set + 0x80, 0);
        e.call(0x0059_24d0, &args![set]);
        assert_eq!(calls(&e, DELETE_007B3FA0).len(), 1);
    }

    #[test]
    fn fn_005923c0_destroys_last_first() {
        let mut e = Engine::new();
        for addr in [DELETE_007B3FA0, VECTOR_DESTRUCTOR, 0x004a_7090, 0x0050_e9f0] {
            e.register(addr, |_, _| Ret::default());
        }
        let set = texture_set(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0059_23c0, &args![set]);
        assert_eq!(e.mem.u32(set), 0x0103_3d1c);
        assert_eq!(e.mem.u32(set + 0x30), 0x0103_3c7c);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 != 0x0f02_0104)
            .skip(1)
            .cloned()
            .collect();
        assert_eq!(
            order,
            vec![
                (
                    VECTOR_DESTRUCTOR,
                    vec![set + 0x38, 0xc, 6, TEXTURE_DESTRUCTOR]
                ),
                (0x004a_7090, vec![set + 0x30]),
                (0x0050_e9f0, vec![set]),
            ]
        );
    }

    #[test]
    fn fn_00593140_makes_the_null_texture_set_once() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0555_0000u32);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // The constructor's callees.
        for addr in [
            0x0050_e820u32,
            0x0051_d190,
            0x00ec_782f,
            MEMSET,
            0x004f_15a0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.put_vtable(0x0103_3000, &[0; 0x400]);
        e.mem.set_u32(0x0103_3d1c + 0x128, 0x0f02_1128);
        e.mem.set_u32(0x0103_3d1c + 0x134, 0x0f02_1134);
        e.register(0x0f02_1128, |_, _| Ret::default());
        e.register(0x0f02_1134, |_, _| Ret::default());
        e.register(ADD_FORM_TO_DATA_HANDLER, |_, _| Ret::default());
        // The texture components' InitializeDataComponent, run by `00592450`.
        e.put_vtable(0x0f02_2000, &[0x0f02_2100]);
        e.register(0x0f02_2100, |_, _| Ret::default());
        e.register_double(0x00ec_782f, |e, a| {
            for index in 0..6u32 {
                e.mem.set_u32(a[1] + index * 0xc, 0x0f02_2000);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_3140, &args![]);
        let set = e.global::<u32>(NULL_TEXTURE_SET);
        assert_ne!(set, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xa0]]);
        assert_eq!(e.mem.u32(set), 0x0103_3d1c);
        assert_eq!(calls(&e, 0x0f02_1128), vec![vec![set, 0x28, 1]]);
        assert_eq!(
            calls(&e, 0x0f02_1134),
            vec![vec![set, NULL_TEXTURE_SET_NAME]]
        );
        assert_eq!(
            calls(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x0555_0000, set]]
        );
        e.call(0x0059_3140, &args![]);
        assert_eq!(calls(&e, NI_ALLOCATE).len(), 1);
    }

    #[test]
    fn fn_005930e0_queues_each_file_entry() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x0666_0000u32);
        e.register(QUEUE_TEXTURE, |_, _| Ret::default());
        let set = e.mem.alloc(0xa0);
        e.mem.set_u32(set + 0x88, 0x0e01);
        e.mem.set_u32(set + 0x88 + 5 * 4, 0x0e06);
        e.call_log = Some(vec![]);
        e.call(0x0059_30e0, &args![set, 0x11u32, 0x22u32, 0x33u32]);
        assert_eq!(
            calls(&e, QUEUE_TEXTURE),
            vec![
                vec![0x0666_0000, 0x0e01, 0x11, 0x22],
                vec![0x0666_0000, 0x0e06, 0x11, 0x22]
            ]
        );
    }

    #[test]
    fn fn_00592cc0_and_00592cf0() {
        let mut e = Engine::new();
        e.register(SET_STRING_00489100, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_2cc0, &args![0x5000u32, 2u32, 0x7000u32]);
        e.call(0x0059_2cc0, &args![0x5000u32, 6u32, 0x7000u32]);
        e.call(0x0059_2cc0, &args![0x5000u32, u32::MAX, 0x7000u32]);
        assert_eq!(
            calls(&e, SET_STRING_00489100),
            vec![vec![0x5050, 0x7000], vec![0x5000 + 0x38 - 0xc, 0x7000]]
        );
        assert_eq!(e.call(0x0059_2cf0, &args![0x5000u32]).u32(), 0x5030);
        assert_eq!(e.call(0x0059_2cf0, &args![0u32]).u32(), 0);
    }

    #[test]
    fn fn_00592e70_answers_nonempty_paths() {
        let mut e = Engine::new();
        let part = e.mem.alloc(0x50);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"a.dds");
        let empty = e.mem.alloc(4);
        // The string at +0x14 has text; the others answer the empty string.
        e.register_double(STRING_CHARACTERS_00408DA0, move |_, a| {
            if a[0] == part + 0x14 {
                text.into_ret()
            } else {
                empty.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_2e70, &args![part, 1u32]).u32(), text);
        for index in [0u32, 2, 3, 4, 5] {
            assert_eq!(e.call(0x0059_2e70, &args![part, index]).u32(), 0);
        }
        assert_eq!(e.call(0x0059_2e70, &args![part, 6u32]).u32(), 0);
        let read: Vec<u32> = calls(&e, STRING_CHARACTERS_00408DA0)
            .iter()
            .map(|c| c[0] - part)
            .collect();
        assert_eq!(read, vec![0x14, 0x8, 0x2c, 0x38, 0x44, 0x20]);
    }

    /// The `BSShaderTextureSet` part's slot +0x8c answers a path for index 1 and 4 only.
    fn path_rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let set = e.mem.alloc(0xa0);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"x.dds");
        let empty = e.mem.alloc(4);
        e.put_vtable(0x0f02_3000, &[0; 0x26]);
        e.mem.set_u32(0x0f02_308c, 0x0f02_3100);
        e.mem.set_u32(0x0f02_3094, 0x0f02_3200);
        e.register_double(0x0f02_3100, move |_, a| match a[1] {
            1 | 4 => text.into_ret(),
            2 => empty.into_ret(),
            _ => 0u32.into_ret(),
        });
        e.register(0x0f02_3200, |_, _| Ret::default());
        e.mem.set_u32(set + 0x30, 0x0f02_3000);
        for addr in [STRCPY_S, STRCAT_S] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, set, text)
    }

    #[test]
    fn fn_00592d20_builds_a_shader_texture_set() {
        let (mut e, set, text) = path_rig();
        let block = e.mem.alloc(0x38);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(SHADER_TEXTURE_SET_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0f02_3000);
            a[0].into_ret()
        });
        assert_eq!(e.call(0x0059_2d20, &args![set]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x38]]);
        let set_texture = calls(&e, 0x0f02_3200);
        assert_eq!(set_texture.len(), 2);
        assert_eq!(set_texture[0][..2], [block, 1]);
        assert_eq!(set_texture[1][..2], [block, 4]);
        let path = set_texture[0][2];
        assert_eq!(
            calls(&e, STRCPY_S)[0],
            vec![path, 0x104, TEXTURES_DIRECTORY]
        );
        assert_eq!(calls(&e, STRCAT_S)[1], vec![path, 0x104, text]);
    }

    #[test]
    fn fn_00592f30_loads_the_texture_into_the_pointer() {
        let (mut e, set, _) = path_rig();
        let part = set + 0x30;
        for addr in [NI_POINTER_FROM, NI_POINTER_DESTRUCT, NI_POINTER_ASSIGN] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SHADER_MANAGER_GET_TEXTURE, |_, _| Ret::default());
        e.call(0x0059_2f30, &args![part, 4u32, 0x9000u32]);
        let pointer = calls(&e, NI_POINTER_FROM)[0][0];
        assert_eq!(calls(&e, NI_POINTER_FROM), vec![vec![pointer, 0]]);
        let got = calls(&e, SHADER_MANAGER_GET_TEXTURE);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0][1..], [1, pointer, 1, 0]);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![0x9000, pointer]]);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT), vec![vec![pointer]]);
        // An empty or missing path: no texture asked, the pointer still assigned.
        e.call(0x0059_2f30, &args![part, 2u32, 0x9000u32]);
        e.call(0x0059_2f30, &args![part, 0u32, 0x9000u32]);
        assert_eq!(calls(&e, SHADER_MANAGER_GET_TEXTURE).len(), 1);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 3);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>) -> (Engine, u32) {
        let mut e = Engine::new();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        for addr in [
            GET_CHUNK_DATA_SIZED,
            GET_CHUNK_DATA_U16,
            LOAD_TEXTURE_CHUNK,
            DECAL_DATA_LOAD,
            STRCPY_S,
            STRCAT_S,
            ARCHIVE_PATH_00AF4200,
            IDENTITY_006815C0,
            PATH_HASH_00AFD270,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DECAL_DATA_CTOR, |_, a| a[0].into_ret());
        e.register(ARCHIVE_FIND_00AF6540, |_, _| 0x0eee_0000u32.into_ret());
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"t.dds");
        let empty = e.mem.alloc(4);
        let set = e.mem.alloc(0xa0);
        // Only the third texture has a path.
        e.register_double(STRING_CHARACTERS_00408DA0, move |_, a| {
            if a[0] == set + 0x50 {
                text.into_ret()
            } else {
                empty.into_ret()
            }
        });
        e.put_vtable(0x0f02_4000, &[0; 0x4e]);
        e.mem.set_u32(0x0f02_40e0, 0x0f02_4100);
        e.mem.set_u32(0x0f02_4134, 0x0f02_4200);
        e.register(0x0f02_4100, |_, _| Ret::default());
        e.register(0x0f02_4200, |_, _| Ret::default());
        e.mem.set_u32(set, 0x0f02_4000);
        e.call_log = Some(vec![]);
        (e, set)
    }

    #[test]
    fn fn_005928a0_reads_the_chunks_and_finds_the_entries() {
        let chunks = vec![
            (CHUNK_EDID, 7),
            (CHUNK_OBND, 0xc),
            (CHUNK_TX00, 4),
            (CHUNK_TX01, 4),
            (CHUNK_TX02, 4),
            (CHUNK_TX03, 4),
            (CHUNK_TX04, 4),
            (CHUNK_TX05, 4),
            (0x3630_5854, 4),
            (CHUNK_DNAM, 2),
            (CHUNK_DODT, 0x24),
            (CHUNK_DODT, 0x24),
        ];
        let (mut e, set) = load_rig(chunks);
        assert!(e.call(0x0059_28a0, &args![set, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![set, FILE]]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![set, 0]]);
        let textures: Vec<u32> = calls(&e, LOAD_TEXTURE_CHUNK)
            .iter()
            .map(|c| c[0] - set)
            .collect();
        assert_eq!(textures, vec![0x38, 0x44, 0x50, 0x5c, 0x68, 0x74]);
        assert_eq!(calls(&e, GET_CHUNK_DATA_U16), vec![vec![FILE, set + 0x84]]);
        // The decal data is made once and read twice.
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x24]]);
        let decal = e.mem.u32(set + 0x80);
        assert_eq!(calls(&e, DECAL_DATA_LOAD), vec![vec![decal, FILE]; 2]);
        assert_eq!(calls(&e, 0x0f02_4100), vec![vec![set, FILE]]);
        assert_eq!(calls(&e, 0x0f02_4200).len(), 1);
        // One path: one archive lookup, into the third entry.
        let find = calls(&e, ARCHIVE_FIND_00AF6540);
        assert_eq!(find.len(), 1);
        let path = calls(&e, ARCHIVE_PATH_00AF4200)[0].clone();
        assert_eq!(path[2], 0x104);
        assert_eq!(find[0][0], 1);
        assert_eq!(find[0][3], path[1]);
        assert_eq!(
            calls(&e, PATH_HASH_00AFD270),
            vec![vec![path[1], find[0][1], find[0][2]]]
        );
        assert_eq!(e.mem.u32(set + 0x88 + 2 * 4), 0x0eee_0000);
        assert_eq!(e.mem.u32(set + 0x88), 0);
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![vec![path[0], 0x104, TEXTURES_DIRECTORY]]
        );
    }
}
