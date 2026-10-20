//! `fallout shared/tesobjecttree.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a5e930`, a destructor that stores the vtable
/// `0109b5ac`, releases the references that vtable slot +0xec returns for each of the
/// entries counted by the half-word at `+0xa6`, and tail-jumps into the base destructor
/// `00a5a390`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00a5e930` ends in `JMP
/// 00a5a390` (which itself ends in `JMP 00a5c240` and then `JMP 00a5d3d0`); no value is
/// computed for the caller and Ghidra types it void.
pub fn fn_0051cfe0(e: &mut Engine, this: Ptr) {
    e.call(0x00a5_e930, &args![this]);
}

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the form
/// type of the record, in `AL`.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the byte at `TESFile` `+0x299`, the swap flag.
const SWAP_FLAG: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESTexture::LoadTextureChunk` (`0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, `cdecl`: component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `00414290` (`ECX` = the `CNAM` block at `+0x6c`): its swap routine.
const SWAP_CNAM: u32 = 0x0041_4290;
/// `00401080` (`cdecl`: address, flag byte): byte-swaps the word at the address.
const SWAP_4_BYTES: u32 = 0x0040_1080;
/// `00401000` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block from `00401000`.
const FREE: u32 = 0x0040_1030;
/// `00403d30` (`cdecl`: destination, value, count): forwards to `_memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `005e03d0` (`ECX` = the list at `+0x5c`): clears it before the `SNAM` entries.
const LIST_CLEAR: u32 = 0x005e_03d0;
/// `0096a610` (`ECX` = the list at `+0x5c`, one word: the address of an entry).
const LIST_ADD: u32 = 0x0096_a610;

/// The form type of the record this loader accepts (`0x25`).
const FORM_TYPE: u8 = 0x25;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_BNAM: u32 = 0x4d41_4e42;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

/// Offsets from `this` of the members the chunks go to.
const MODEL_OFFSET: u32 = 0x30;
const TEXTURE_OFFSET: u32 = 0x48;
const DESTRUCTIBLE_OFFSET: u32 = 0x54;
const LIST_OFFSET: u32 = 0x5c;
const CNAM_OFFSET: u32 = 0x6c;
/// The signed word the `CNAM` chunk handling raises to `0x80` when negative.
const CNAM_LIMIT_OFFSET: u32 = 0x80;
const BNAM_OFFSET: u32 = 0x8c;
const BNAM_SECOND_OFFSET: u32 = 0x90;

/// A member address, or null when the object is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

// Translated from 0051bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051bbc0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESObjectTREE::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`): the `Load`
/// slot of the virtual table `0102bc94` (the form type `0x25` constructor
/// `0051b6e0` stores it). Answers false when the record is not of form type
/// `0x25`; otherwise it runs `TESForm::LoadForm`, reads every chunk and answers
/// true:
/// - `MODL`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`;
/// - `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x48`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x54`
///   (each of these three is null when `this` is null);
/// - `OBND`: virtual slot `0xe0` with the file; `EDID`: read into a stack buffer
///   (at most 0x200 bytes) and passed to virtual slot `0x134`;
/// - `CNAM`: only when the chunk is 0x20 bytes: read into `this + 0x6c`, swapped
///   by `00414290` when the swap flag of the file is set, and the signed word at
///   `+0x80` is set to `0x80` if negative;
/// - `BNAM`: only when the chunk is 8 bytes: read into `this + 0x8c`, and the
///   words at `+0x8c` and `+0x90` are byte-swapped when the swap flag is set;
/// - `SNAM`: when the size is a non-zero multiple of 4, the words are read into
///   a new block (zeroed first), the list at `this + 0x5c` is cleared, and each
///   non-zero word (swapped first when the swap flag is set) is added to it by
///   address (`0096a610`); the block is freed.
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn tes_object_tree_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != FORM_TYPE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_CNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 0x20 {
                    e.call(GET_CHUNK_DATA, &args![file, object + CNAM_OFFSET, 0x20u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_CNAM, &args![object + CNAM_OFFSET]);
                    }
                    if (e.mem.u32(object + CNAM_LIMIT_OFFSET) as i32) < 0 {
                        e.mem.set_u32(object + CNAM_LIMIT_OFFSET, 0x80);
                    }
                }
            }
            CHUNK_BNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 8 {
                    e.call(GET_CHUNK_DATA, &args![file, object + BNAM_OFFSET, 8u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_4_BYTES, &args![object + BNAM_OFFSET, 0u32]);
                        e.call(SWAP_4_BYTES, &args![object + BNAM_SECOND_OFFSET, 0u32]);
                    }
                }
            }
            CHUNK_SNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size != 0 && size % 4 == 0 {
                    let count = size >> 2;
                    // The allocation size is `count * 4`, which cannot overflow.
                    let block = e.call(ALLOCATE, &args![count * 4]).u32();
                    e.call(MEMSET, &args![block, 0u32, size]);
                    e.call(GET_CHUNK_DATA, &args![file, block, size]);
                    e.call(LIST_CLEAR, &args![object + LIST_OFFSET]);
                    for index in 0..count {
                        let entry = block + index * 4;
                        if e.call(SWAP_FLAG, &args![file]).bool() {
                            e.call(SWAP_4_BYTES, &args![entry, 0u32]);
                        }
                        if e.mem.u32(entry) != 0 {
                            e.call(LIST_ADD, &args![object + LIST_OFFSET, entry]);
                        }
                    }
                    e.call(FREE, &args![block]);
                }
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL_OFFSET);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_ICON => {
                let texture = component(object, TEXTURE_OFFSET);
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            }
            CHUNK_DSTD | CHUNK_DEST => {
                let target = component(object, DESTRUCTIBLE_OFFSET);
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![target, file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0051d150 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0120044c without reading the receiver. The map names this body `BSShaderTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d150(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_044c
}

// Translated from 0051d1b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01200444 without reading the receiver. The map names this body `BSTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d1b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_0444
}

/// `00a5ecb0` (`NiMain/ninode.cpp`, `thiscall`, one word): the `NiNode`
/// constructor body the billboard constructors run first.
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The vtable `0102bf44` (RTTI `.?AVNiBillboardNode@@`).
const BILLBOARD_NODE_VTABLE: u32 = 0x0102_bf44;
/// `NiBillboardNode`'s flag word (`u16`; the low three bits are the mode,
/// bit 3 a flag) and its float at `+0xb0`.
const BILLBOARD_FLAGS_OFFSET: u32 = 0xac;
const BILLBOARD_FLOAT_OFFSET: u32 = 0xb0;
/// `00aa1460` (`cdecl`: block, size): gives back a block of the given size.
const FREE_SIZED: u32 = 0x00aa_1460;
/// `00a5d3a0` (`NiMain/niobject.cpp`, `thiscall`): the base constructor
/// `BSTextureSet`'s constructor runs.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtables `0102c0ec` (RTTI `.?AVBSTextureSet@@`) and `0102c04c` (RTTI
/// `.?AVBSShaderTextureSet@@`).
const TEXTURE_SET_VTABLE: u32 = 0x0102_c0ec;
const SHADER_TEXTURE_SET_VTABLE: u32 = 0x0102_c04c;
/// `00ec782f` (`cdecl`: array, element size, count, constructor,
/// destructor): the compiler's array constructor helper.
const ARRAY_CONSTRUCT: u32 = 0x00ec_782f;
/// `00ec5fce` (`eh vector destructor iterator`, `cdecl`: array, element
/// size, count, destructor).
const ARRAY_DESTRUCT: u32 = 0x00ec_5fce;
/// The 8-byte string's constructor (`004037b0`) and destructor (`004037d0`),
/// passed to the array helpers by address.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `004a7090` (`thiscall`): the base destructor `BSShaderTextureSet`'s
/// destructor ends with.
const TEXTURE_SET_BASE_DESTRUCT: u32 = 0x004a_7090;
/// Where `BSShaderTextureSet`'s six strings start, and their size and count.
const TEXTURE_NAMES_OFFSET: u32 = 0x8;
const TEXTURE_NAME_SIZE: u32 = 8;
const TEXTURE_NAME_COUNT: u32 = 6;
/// The text `GetBillboardTextureFilename` starts the name with (`0102bed0`).
const BILLBOARD_TEXTURE_PREFIX: u32 = 0x0102_bed0;
/// The model component's virtual slot that answers the model's file name.
const SLOT_MODEL_NAME: u32 = 0x14;
/// The three doubles `0051d250` compares the two `BNAM` floats with.
const BNAM_LOWER: u32 = 0x0101_2060;
const BNAM_FIRST_UPPER: u32 = 0x0101_79e0;
const BNAM_SECOND_UPPER: u32 = 0x0102_c188;

/// The address `offset` bytes into the object at `object`.
fn field_addr(object: Ptr, offset: u32) -> u32 {
    object.addr().wrapping_add(offset)
}

// Translated from 0051cf00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051cf00 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// An `NiBillboardNode` constructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): runs the `NiNode` constructor body `00a5ecb0` with 0, stores
/// the `NiBillboardNode` vtable `0102bf44`, clears the flag word at `+0xac`
/// and stores 0.0 at `+0xb0`, then sets the mode bits to 1 (`0051d000`) and
/// flag bit 3 (`0051d060` with 1). Returns `this`.
pub fn fn_0051cf00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BILLBOARD_NODE_VTABLE);
    e.mem.set_u16(field_addr(this, BILLBOARD_FLAGS_OFFSET), 0);
    // FLDZ; FSTP float.
    e.mem
        .set_u32(field_addr(this, BILLBOARD_FLOAT_OFFSET), x87_store32(0.0));
    fn_0051d000(e, this, 1);
    fn_0051d060(e, this, 1);
    this
}

// Translated from 0051d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets the low three bits
/// of the billboard flag word at `+0xac` to `mode` (`0051d020` with mask 7,
/// shift 0).
pub fn fn_0051d000(e: &mut Engine, this: Ptr, mode: u16) {
    fn_0051d020(e, this, mode, 7, 0);
}

// Translated from 0051d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): the flag word at
/// `+0xac` becomes `(flags & !mask) | (value << shift)`, kept to 16 bits; the
/// shift count is taken modulo 32 as `SHL` does.
pub fn fn_0051d020(e: &mut Engine, this: Ptr, value: u16, mask: u16, shift: u16) {
    let at = field_addr(this, BILLBOARD_FLAGS_OFFSET);
    let kept = u32::from(e.mem.u16(at)) & !u32::from(mask);
    let bits = u32::from(value) << (u32::from(shift) & 31);
    e.mem.set_u16(at, (kept | bits) as u16);
}

// Translated from 0051d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets (`on` non-zero) or
/// clears bit 3 of the flag word at `+0xac` (`0051d080` with 8).
pub fn fn_0051d060(e: &mut Engine, this: Ptr, on: u8) {
    fn_0051d080(e, this, on, 8);
}

// Translated from 0051d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): ORs `bits` into the flag
/// word at `+0xac` when `on` is non-zero, clears them otherwise.
pub fn fn_0051d080(e: &mut Engine, this: Ptr, on: u8, bits: u16) {
    let at = field_addr(this, BILLBOARD_FLAGS_OFFSET);
    let flags = e.mem.u16(at);
    if on != 0 {
        e.mem.set_u16(at, flags | bits);
    } else {
        e.mem.set_u16(at, flags & !bits);
    }
}

// Translated from 0051d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSTextureSet` constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): runs the base constructor `00a5d3a0` and stores the vtable
/// `0102c0ec` (RTTI `.?AVBSTextureSet@@`). Returns `this`.
pub fn fn_0051d190(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TEXTURE_SET_VTABLE);
    this
}

// Translated from 0051d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d0d0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `BSShaderTextureSet::BSShaderTextureSet` (Xbox PDB; `thiscall`, plain
/// `RET`): the `BSTextureSet` constructor (`0051d190`), the vtable `0102c04c`,
/// then the array helper `00ec782f` constructs the six 8-byte strings at
/// `+0x8` (constructor `004037b0`, destructor `004037d0`). Returns `this`.
pub fn bs_shader_texture_set_bs_shader_texture_set(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0051d190(e, this);
    e.mem.set_u32(this.addr(), SHADER_TEXTURE_SET_VTABLE);
    e.call(
        ARRAY_CONSTRUCT,
        &args![
            field_addr(this, TEXTURE_NAMES_OFFSET),
            TEXTURE_NAME_SIZE,
            TEXTURE_NAME_COUNT,
            STRING_CONSTRUCT,
            STRING_DESTRUCT
        ],
    );
    this
}

// Translated from 0051cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::GetRTTI` (Xbox PDB; `thiscall`, plain `RET`): answers
/// the constant address `011f4aa0` without reading `this`.
pub fn ni_billboard_node_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_4aa0
}

// Translated from 0051d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderTextureSet::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor `0051d1c0`, then, when bit 0 of `flags` is set,
/// gives the 0x38-byte object back (`00aa1460`). Returns `this`.
pub fn bs_shader_texture_set_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0051d1c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x38u32]);
    }
    this
}

// Translated from 0051d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d1c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSShaderTextureSet` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): destroys the six 8-byte strings at `+0x8` with
/// the array helper `00ec5fce` (destructor `004037d0`), then runs the base
/// destructor `004a7090`.
pub fn fn_0051d1c0(e: &mut Engine, this: Ptr) {
    e.call(
        ARRAY_DESTRUCT,
        &args![
            field_addr(this, TEXTURE_NAMES_OFFSET),
            TEXTURE_NAME_SIZE,
            TEXTURE_NAME_COUNT,
            STRING_DESTRUCT
        ],
    );
    e.call(TEXTURE_SET_BASE_DESTRUCT, &args![this]);
}

// Translated from 0051c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE::GetBillboardTextureFilename` (Xbox PDB; `thiscall`,
/// `RET 4`): writes into `buffer` the text at `0102bed0`, then the model's
/// file name (virtual slot `0x14` of the model component at `this + 0x30`)
/// up to its first `.` or its end; when it stopped at a `.` it appends
/// `.dds`. The result is terminated with a zero byte.
pub fn tes_object_tree_get_billboard_texture_filename(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let mut out = buffer.addr();
    let mut prefix = BILLBOARD_TEXTURE_PREFIX;
    while e.mem.u8(prefix) != 0 {
        let byte = e.mem.u8(prefix);
        e.mem.set_u8(out, byte);
        prefix = prefix.wrapping_add(1);
        out = out.wrapping_add(1);
    }
    let mut name = e
        .vcall(field_addr(this, MODEL_OFFSET), SLOT_MODEL_NAME, &[])
        .u32();
    while e.mem.u8(name) != b'.' && e.mem.u8(name) != 0 {
        let byte = e.mem.u8(name);
        e.mem.set_u8(out, byte);
        name = name.wrapping_add(1);
        out = out.wrapping_add(1);
    }
    if e.mem.u8(name) == b'.' {
        for byte in *b".dds" {
            e.mem.set_u8(out, byte);
            out = out.wrapping_add(1);
        }
    }
    e.mem.set_u8(out, 0);
}

// Translated from 0051d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): tests the two `BNAM`
/// floats at `+0x8c` and `+0x90` against the doubles at `01012060` (lower),
/// `010179e0` and `0102c188` (uppers). False when the first is above the
/// lower and at most its upper; otherwise true unless the second is above
/// the lower and at most its upper. A NaN compares as the exe's flag tests
/// do: it is never "above" nor "at most".
pub fn fn_0051d250(e: &mut Engine, this: Ptr) -> bool {
    let lower: f64 = e.global(BNAM_LOWER);
    let first = x87_load32(e.mem.u32(field_addr(this, BNAM_OFFSET)));
    if first > lower && first <= e.global::<f64>(BNAM_FIRST_UPPER) {
        return false;
    }
    let second = x87_load32(e.mem.u32(field_addr(this, BNAM_SECOND_OFFSET)));
    if !(second > lower) {
        return true;
    }
    !(second <= e.global::<f64>(BNAM_SECOND_UPPER))
}

// Translated from 0051cfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor `0051cfe0`, then, when bit 0 of `flags` is set,
/// gives the 0xb4-byte object back (`00aa1460`). Returns `this`.
pub fn ni_billboard_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0051cfe0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0xb4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_d150, fn_0051d150(Ptr) -> u32),
        entry!(0x0051_d1b0, fn_0051d1b0(Ptr) -> u32),
        entry!(0x0051_cfe0, fn_0051cfe0(Ptr)),
        entry!(0x0051_bbc0, tes_object_tree_load(Ptr, Ptr) -> bool),
        entry!(0x0051_cf00, fn_0051cf00(Ptr) -> Ptr),
        entry!(0x0051_d000, fn_0051d000(Ptr, u16)),
        entry!(0x0051_d020, fn_0051d020(Ptr, u16, u16, u16)),
        entry!(0x0051_d060, fn_0051d060(Ptr, u8)),
        entry!(0x0051_d080, fn_0051d080(Ptr, u8, u16)),
        entry!(0x0051_d190, fn_0051d190(Ptr) -> Ptr),
        entry!(0x0051_d0d0, bs_shader_texture_set_bs_shader_texture_set(Ptr) -> Ptr),
        entry!(0x0051_cf90, ni_billboard_node_get_rtti(Ptr) -> u32),
        entry!(
            0x0051_d160,
            bs_shader_texture_set_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0051_d1c0, fn_0051d1c0(Ptr)),
        entry!(
            0x0051_c3b0,
            tes_object_tree_get_billboard_texture_filename(Ptr, Ptr)
        ),
        entry!(0x0051_d250, fn_0051d250(Ptr) -> bool),
        entry!(
            0x0051_cfa0,
            ni_billboard_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod thunk_tests {
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

    #[test]
    fn fwd_a03_0051cfe0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_cfe0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    struct Record {
        chunks: Vec<u32>,
        form_type: u32,
        null_this: bool,
        chunk_size: u32,
        swap: bool,
        /// Words the chunk data call writes at its buffer.
        data: Vec<u32>,
    }

    impl Record {
        fn of(chunks: Vec<u32>) -> Record {
            Record {
                chunks,
                form_type: 0x25,
                null_this: false,
                chunk_size: 0,
                swap: false,
                data: vec![],
            }
        }
    }

    /// Runs the loader; returns the engine, the call log (without the top-level
    /// call), the address of `this` and the result.
    fn run(record: Record) -> (Engine, CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (record.chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = record.chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        let form_type = record.form_type;
        e.register_double(GET_TES_FORM, move |_, _| ret(0x7700 | form_type));
        for addr in [
            TES_FORM_LOAD_FORM,
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_DESTRUCTIBLE_CHUNK,
            SWAP_CNAM,
            SWAP_4_BYTES,
            FREE,
            LIST_CLEAR,
            LIST_ADD,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        let data = record.data.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            for (i, word) in data.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            Ret::default()
        });
        let chunk_size = record.chunk_size;
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        let swap = record.swap;
        e.register_double(SWAP_FLAG, move |_, _| ret(0x5500 | u32::from(swap)));
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if record.null_this {
            0
        } else {
            let object = e.mem.alloc(0xa0);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x0051_bbc0, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (e, log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn a_record_of_another_form_type_is_refused() {
        let mut record = Record::of(vec![CHUNK_ICON]);
        record.form_type = 0x26;
        let (_, log, _, result) = run(record);
        assert!(!result);
        assert_eq!(log, vec![(GET_TES_FORM, vec![FILE])]);
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (_, log, this, result) = run(Record::of(vec![]));
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[2], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let mut record = Record::of(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555]);
        record.chunk_size = 5;
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn the_components_get_their_chunks() {
        let chunks = vec![CHUNK_MODL, CHUNK_MODT, CHUNK_ICON, CHUNK_DSTD, CHUNK_DEST];
        let (_, log, this, _) = run(Record::of(chunks));
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 2]
        );
        assert_eq!(
            calls_to(&log, LOAD_TEXTURE_CHUNK),
            vec![vec![this + 0x48, FILE]]
        );
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this + 0x54, FILE]; 2]
        );
    }

    #[test]
    fn a_null_object_has_null_components() {
        let mut record = Record::of(vec![CHUNK_MODL, CHUNK_ICON, CHUNK_DEST]);
        record.null_this = true;
        let (_, log, _, _) = run(record);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_TEXTURE_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn cnam_needs_exactly_0x20_bytes_and_raises_a_negative_limit() {
        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x1f;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![0; 8];
        let (e, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x6c, 0x20]]
        );
        assert!(calls_to(&log, SWAP_CNAM).is_empty());
        // The block is 0x20 bytes at +0x6c, so +0x80 is its sixth word.
        assert_eq!(e.mem.u32(this + 0x80), 0);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.swap = true;
        record.data = vec![1, 2, 3, 4, 5, 0xffff_fff0];
        let (e, log, this, _) = run(record);
        assert_eq!(calls_to(&log, SWAP_CNAM), vec![vec![this + 0x6c]]);
        assert_eq!(e.mem.u32(this + 0x80), 0x80);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![1, 2, 3, 4, 5, 0x7fff_ffff];
        let (e, _, this, _) = run(record);
        assert_eq!(e.mem.u32(this + 0x80), 0x7fff_ffff);
    }

    #[test]
    fn bnam_needs_exactly_8_bytes_and_swaps_both_words_on_request() {
        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 4;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x8c, 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        record.swap = true;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![this + 0x8c, 0], vec![this + 0x90, 0]]
        );
    }

    #[test]
    fn snam_adds_the_non_zero_words_and_frees_the_block() {
        for size in [0, 6] {
            let mut record = Record::of(vec![CHUNK_SNAM]);
            record.chunk_size = size;
            let (_, log, _, _) = run(record);
            assert!(calls_to(&log, ALLOCATE).is_empty(), "size {size}");
        }

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 12;
        record.data = vec![0x11, 0, 0x33];
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![12]]);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(calls_to(&log, MEMSET), vec![vec![block, 0, 12]]);
        assert_eq!(calls_to(&log, LIST_CLEAR), vec![vec![this + 0x5c]]);
        assert_eq!(
            calls_to(&log, LIST_ADD),
            vec![vec![this + 0x5c, block], vec![this + 0x5c, block + 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());
        assert_eq!(calls_to(&log, FREE), vec![vec![block]]);

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 8;
        record.swap = true;
        record.data = vec![1, 2];
        let (_, log, _, _) = run(record);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![block, 0], vec![block + 4, 0]]
        );
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0051d150_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d150, &args![0x1000u32]).u32(), 0x0120_044c);
        assert_eq!(e.call(0x0051_d150, &args![0u32]).u32(), 0x0120_044c);
    }

    #[test]
    fn fn_0051d1b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d1b0, &args![0x1000u32]).u32(), 0x0120_0444);
        assert_eq!(e.call(0x0051_d1b0, &args![0u32]).u32(), 0x0120_0444);
    }
}

#[cfg(test)]
mod b015_tests {
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

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn billboard_constructor_sets_vtable_mode_one_and_flag_bit_3() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_NODE_CONSTRUCT]);
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xfff0);
        e.mem.set_u32(node.addr() + 0xb0, 0x7fc0_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_cf00, &args![node]).u32(), node.addr());
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCT), vec![vec![node.addr(), 0]]);
        assert_eq!(e.mem.u32(node.addr()), BILLBOARD_NODE_VTABLE);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x9);
        assert_eq!(e.mem.u32(node.addr() + 0xb0), 0);
    }

    #[test]
    fn mode_setter_replaces_the_low_three_bits() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xff0f);
        e.call(0x0051_d000, &args![node, 0x0002u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0xff0a);
    }

    #[test]
    fn masked_setter_shifts_modulo_32_and_keeps_16_bits() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xffff);
        e.call(0x0051_d020, &args![node, 0x3u32, 0x00f0u32, 4u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0xff3f);
        // A shift of 36 is 4 for SHL; bits shifted past 16 are lost.
        e.mem.set_u16(node.addr() + 0xac, 0);
        e.call(0x0051_d020, &args![node, 0xf001u32, 0u32, 36u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0010);
    }

    #[test]
    fn flag_bit_3_setter_and_bit_helper_set_and_clear() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0x0101);
        e.call(0x0051_d060, &args![node, 0x100u32]);
        // Only the low byte of the flag counts: 0x100 is "off".
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0101);
        e.call(0x0051_d060, &args![node, 1u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0109);
        e.call(0x0051_d080, &args![node, 0u32, 0x0101u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0008);
        e.call(0x0051_d080, &args![node, 7u32, 0x8000u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x8008);
    }

    #[test]
    fn texture_set_constructor_runs_the_base_and_sets_its_vtable() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_OBJECT_CONSTRUCT]);
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_d190, &args![set]).u32(), set.addr());
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT), vec![vec![set.addr()]]);
        assert_eq!(e.mem.u32(set.addr()), TEXTURE_SET_VTABLE);
    }

    #[test]
    fn shader_texture_set_constructor_builds_six_strings() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_OBJECT_CONSTRUCT, ARRAY_CONSTRUCT]);
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_d0d0, &args![set]).u32(), set.addr());
        assert_eq!(e.mem.u32(set.addr()), SHADER_TEXTURE_SET_VTABLE);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&e, ARRAY_CONSTRUCT),
            vec![vec![set.addr() + 8, 8, 6, 0x0040_37b0, 0x0040_37d0]]
        );
    }

    #[test]
    fn billboard_rtti_is_a_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_cf90, &args![0u32]).u32(), 0x011f_4aa0);
    }

    #[test]
    fn shader_texture_set_destructors_free_on_flag_bit_0() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[ARRAY_DESTRUCT, TEXTURE_SET_BASE_DESTRUCT, FREE_SIZED],
        );
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        for (flags, frees) in [(0u32, false), (2, false), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0051_d160, &args![set, flags]).u32(), set.addr());
            assert_eq!(
                calls_to(&e, ARRAY_DESTRUCT),
                vec![vec![set.addr() + 8, 8, 6, 0x0040_37d0]]
            );
            assert_eq!(
                calls_to(&e, TEXTURE_SET_BASE_DESTRUCT),
                vec![vec![set.addr()]]
            );
            let freed = calls_to(&e, FREE_SIZED);
            if frees {
                assert_eq!(freed, vec![vec![set.addr(), 0x38]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn billboard_deleting_destructor_frees_0xb4_bytes_on_flag_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[0x00a5_e930, FREE_SIZED]);
        for (flags, frees) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0051_cfa0, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
            assert_eq!(calls_to(&e, FREE_SIZED).len(), usize::from(frees));
        }
    }

    fn texture_name(model_name: &[u8]) -> Vec<u8> {
        let mut e = Engine::new();
        e.map(BILLBOARD_TEXTURE_PREFIX, 0x10);
        e.mem.set_cstr(BILLBOARD_TEXTURE_PREFIX, b"Trees/");
        let tree: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0, 0x0090_1000]);
        e.mem.set_u32(tree.addr() + 0x30, 0x0090_0000);
        let name = e.mem.alloc(0x40);
        e.mem.set_cstr(name, model_name);
        e.register_double(0x0090_1000, move |_, _| name.into_ret());
        let buffer = e.mem.alloc(0x80);
        e.mem.write(buffer, &[0xaa; 0x80]);
        e.call(0x0051_c3b0, &args![tree, buffer]);
        e.mem.cstr(buffer)
    }

    #[test]
    fn billboard_texture_name_swaps_the_extension_for_dds() {
        assert_eq!(
            texture_name(b"Joshua01.nif"),
            b"Trees/Joshua01.dds".to_vec()
        );
        assert_eq!(texture_name(b"a.b.nif"), b"Trees/a.dds".to_vec());
        // No dot: the name is copied whole, without an extension.
        assert_eq!(texture_name(b"Plain"), b"Trees/Plain".to_vec());
    }

    #[test]
    fn bnam_test_follows_the_flag_tests() {
        let mut e = Engine::new();
        for (addr, value) in [
            (BNAM_LOWER, 0.0f64),
            (BNAM_FIRST_UPPER, 10.0),
            (BNAM_SECOND_UPPER, 20.0),
        ] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
        let tree: Ptr = Ptr::new(e.mem.alloc(0x100));
        let mut check = |first: f32, second: f32| {
            e.mem.set_f32(tree.addr() + 0x8c, first);
            e.mem.set_f32(tree.addr() + 0x90, second);
            e.call(0x0051_d250, &args![tree]).bool()
        };
        // First in (0, 10]: false whatever the second is.
        assert!(!check(5.0, 100.0));
        assert!(!check(10.0, 100.0));
        // First outside: true unless the second is in (0, 20].
        assert!(!check(11.0, 20.0));
        assert!(check(11.0, 0.0));
        assert!(check(0.0, 21.0));
        assert!(check(f32::NAN, f32::NAN));
        assert!(!check(f32::NAN, 1.0));
    }
}
