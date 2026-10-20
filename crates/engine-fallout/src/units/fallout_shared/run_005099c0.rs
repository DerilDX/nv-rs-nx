//! `fallout shared/run_005099c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00509a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00509a10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x24 (0x68 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00509a10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_814c);
    e.mem.set_u32(base + 0x30, 0x0102_8128);
    e.call(0x004f_15a0, &args![this, 0x24]);
    e.call(0x0050_9ee0, &args![this]);
    this
}

// Translated from 0050a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a5a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2e (0xac bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050a5a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_a6a0, &args![this]);
    e.mem.set_u32(base, 0x0102_8444);
    e.mem.set_u32(base + 0x30, 0x0102_842c);
    e.mem.set_u32(base + 0x3c, 0x0102_8408);
    e.mem.set_u32(base + 0x5c, 0x0102_83e8);
    e.mem.set_u32(base + 0x68, 0x0102_83d4);
    e.mem.set_u32(base + 0x74, 0x0102_83b4);
    e.mem.set_u32(base + 0x7c, 0x0102_83a0);
    e.mem.set_u32(base + 0x84, 0x0102_838c);
    e.mem.set_u32(base + 0x8c, 0x0102_8378);
    e.mem.set_u32(base + 0x9c, 0x0102_8364);
    e.call(0x004f_15a0, &args![this, 0x2e]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050a770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2d (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050a770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_864c);
    e.mem.set_u32(base + 0x30, 0x0102_862c);
    e.mem.set_u32(base + 0x4c, 0x0102_8608);
    e.call(0x004f_15a0, &args![this, 0x2d]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 00509a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `00508eb0`. `00508eb0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00509a00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_8eb0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 0050a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00509c90`. `00509c90` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050a590(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_9c90, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 0050a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `0050a670`. `0050a670` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050a760(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_a670, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00509c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00509cc0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00509c90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_9cc0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050a6a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050a670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_a6a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050a820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050a850` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050a820(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_a850, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// ---- Shared calls of the loads below -----------------------------------------------------

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the current record's form type.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, thiscall, `RET 4`).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`TESForm`, thiscall, `RET 4`): sets or clears a form flag bit.
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`): the current chunk's tag.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (thiscall): steps to the next chunk; false at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (thiscall): the file's word at +0x25C, the current chunk's size.
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, thiscall, `RET 8`).
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`, thiscall, `RET 4`).
const FILE_GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00401680` (thiscall): the file's byte at +0x299 (the data is byte-swapped).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `00403570` (thiscall): the file's 16-bit word at +0x254 (the record's version).
const FILE_VERSION: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (Xbox PDB, `004892d0`, cdecl: model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`, thiscall, `RET 0xC`: file, buffer, size).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00403d30` (cdecl: pointer, byte, count): `memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `004013e0` (thiscall): bit 3 of the form's flags (`TESForm` +0x08).
const FORM_FLAG_8: u32 = 0x0040_13e0;

/// The vtable slot `EDID` passes the editor id to (byte offset 0x134).
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The vtable slot `OBND` passes the file to (byte offset 0xE0).
const SLOT_LOAD_BOUND: u32 = 0xe0;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_LVLD: u32 = 0x444c_564c;
const CHUNK_LVLF: u32 = 0x464c_564c;
const CHUNK_LVLG: u32 = 0x474c_564c;
const CHUNK_LVLO: u32 = 0x4f4c_564c;
const CHUNK_COED: u32 = 0x4445_4f43;

/// `EDID`: the game takes a stack buffer as large as the chunk, reads at most
/// 0x200 bytes into it and gives it to vtable slot 0x134.
fn load_editor_id(e: &mut Engine, object: u32, file: Ptr) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    let buffer = e.mem.alloc(size.max(1));
    e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
    e.mem.free(buffer);
}

// Translated from 00509ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type-0x24 form's data block start values (called by its constructor
/// `00509a10`): bytes +0x48 = 0x1E, +0x49 = 0, +0x4A = 0x5A, the 16-bit word
/// +0x4C and the word +0x50 zero, the floats +0x54..+0x60 from the constants
/// at `0101e340`, `0101df44`, `01016248` and `01017b78` (`FLD`/`FSTP`), and
/// the flags byte +0x64 zero.
pub fn fn_00509ee0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x48, 0x1e);
    e.mem.set_u8(base + 0x49, 0);
    e.mem.set_u8(base + 0x4a, 0x5a);
    e.mem.set_u16(base + 0x4c, 0);
    e.mem.set_u32(base + 0x50, 0);
    for (offset, constant) in [
        (0x54, 0x0101_e340),
        (0x58, 0x0101_df44),
        (0x5c, 0x0101_6248),
        (0x60, 0x0101_7b78),
    ] {
        let bits = x87_quiet_bits(e.global::<u32>(constant));
        e.mem.set_u32(base + offset, bits);
    }
    e.mem.set_u8(base + 0x64, 0);
}

// Translated from 00509f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless form flag 0x8 is set (`004013e0`), sets form flag bit 1
/// (`00484ab0` with 1).
pub fn fn_00509f50(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8, &args![this]).bool() {
        return;
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 00509f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00509f80 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated; the `EDID` alloca buffer is a heap block.
/// The type-0x24 form's `Load` (`RET 4`: the file): false unless the record
/// is form type 0x24. Clears the 0x20-byte data block at +0x48 (`memset`),
/// runs `TESForm::LoadForm`, clears form flag bit 0 (`00484ab0` with 0), then
/// reads every chunk: `EDID` (vtable slot 0x134), `OBND` (slot 0xE0),
/// `MODL`/`MODT` (`TESModel::LoadModelChunk` on +0x30, or on 0 when `this` is
/// null), `DATA` (0x20 bytes to +0x48 through `TESForm::LoadData`, then
/// byte-swapped by `0050a120` when the file swaps). Returns true.
pub fn fn_00509f80(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != 0x24 {
        return false;
    }
    e.call(MEMSET, &args![base + 0x48, 0u32, 0x20u32]);
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_OBND => {
                e.vcall(base, SLOT_LOAD_BOUND, &args![file]);
            }
            CHUNK_DATA => {
                e.call(FORM_LOAD_DATA, &args![this, file, base + 0x48, 0x20u32]);
            }
            CHUNK_EDID => load_editor_id(e, base, file),
            CHUNK_MODL | CHUNK_MODT => {
                let model = if base == 0 { 0 } else { base + 0x30 };
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(0x0050_a120, &args![base + 0x48]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005099c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float from the 40-entry table at `01189fac` (16 bytes an entry): for
/// `index` 1..=0x28 the entry's first word goes through `00403e20` (thiscall,
/// `effectitem.cpp`), which gives the address of the float returned (`FLD`);
/// any other index gives 0.0. `this` (`ECX`) is not read; `RET 4`.
pub fn fn_005099c0(e: &mut Engine, _this: Ptr, index: u32) -> f32 {
    let mut value = 0.0f32;
    if index > 0 && index <= 0x28 {
        let entry = e.global::<u32>(0x0118_9fac + ((index - 1) << 4));
        let address = e.call(0x0040_3e20, &args![entry]).u32();
        value = x87_quiet(f32::from_bits(e.mem.u32(address)));
    }
    value
}

// Translated from 0050a550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string `00559450` gives for the member at +0x34 (thiscall), or the
/// constant at `01011584` when it gives null (called twice when it does not).
pub fn fn_0050a550(e: &mut Engine, this: Ptr) -> u32 {
    let member = this.addr() + 0x34;
    if e.call(0x0055_9450, &args![member]).u32() != 0 {
        e.call(0x0055_9450, &args![member]).u32()
    } else {
        0x0101_1584
    }
}

// Translated from 00509b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0x1 of the flags byte at +0x64 (`MOVSX`, `AND`, `SETNZ`).
pub fn fn_00509b40(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr() + 0x64) as i8 as i32) & 0x1 != 0
}

// Translated from 00509c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0x4 of the flags byte at +0x64.
pub fn fn_00509c20(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr() + 0x64) as i8 as i32) & 0x4 != 0
}

// Translated from 00509bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0x2 of the flags byte at +0x64.
pub fn fn_00509bb0(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr() + 0x64) as i8 as i32) & 0x2 != 0
}

// Translated from 00509aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x4A (returned in `AL`).
pub fn fn_00509aa0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x4a)
}

// Translated from 00509cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00509cc0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `00509c90` runs: stores the vtables `0102814c` and
/// `01028128` (+0x30), destroys the member at +0x30 (`00489070`) and runs the
/// base destructor `0050e9f0`.
pub fn fn_00509cc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_814c);
    e.mem.set_u32(base + 0x30, 0x0102_8128);
    e.call(0x0048_9070, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 0050a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a6a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `0050a670` runs: stores the ten vtables of the
/// type-0x2E form (as its constructor `0050a5a0` does), then runs `00483710`
/// (`tesform.cpp`) and the base destructor `0051a840`.
pub fn fn_0050a6a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_8444);
    e.mem.set_u32(base + 0x30, 0x0102_842c);
    e.mem.set_u32(base + 0x3c, 0x0102_8408);
    e.mem.set_u32(base + 0x5c, 0x0102_83e8);
    e.mem.set_u32(base + 0x68, 0x0102_83d4);
    e.mem.set_u32(base + 0x74, 0x0102_83b4);
    e.mem.set_u32(base + 0x7c, 0x0102_83a0);
    e.mem.set_u32(base + 0x84, 0x0102_838c);
    e.mem.set_u32(base + 0x8c, 0x0102_8378);
    e.mem.set_u32(base + 0x9c, 0x0102_8364);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0051_a840, &args![this]);
}

// Translated from 0050a850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a850 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `0050a820` runs: stores the vtables `0102864c`,
/// `0102862c` (+0x30) and `01028608` (+0x4C), destroys the members at +0x4C
/// (`0048a490`, `tesmodeltextureswap.cpp`) and +0x30 (`004876d0`), then runs
/// the base destructor `0050f740`.
pub fn fn_0050a850(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_864c);
    e.mem.set_u32(base + 0x30, 0x0102_862c);
    e.mem.set_u32(base + 0x4c, 0x0102_8608);
    e.call(0x0048_a490, &args![base + 0x4c]);
    e.call(0x0048_76d0, &args![base + 0x30]);
    e.call(0x0050_f740, &args![this]);
}

/// The leveled entry `LVLO` left for a following `COED` chunk (`011c8e64`).
const PENDING_LEVELED_ENTRY: u32 = 0x011c_8e64;

// Translated from 0050a930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050a930 - the exception-unwinding frame (FS:[0], frees the `COED` block if its constructor throws) and the security cookie check (`__security_check_cookie`, 00ec408c) are not translated; the `EDID`/`MODS` alloca buffers are heap blocks.
/// The type-0x2D (leveled list) form's `Load` (`RET 4`: the file): false
/// unless the record is form type 0x2D. `TESForm::LoadForm`, then for each
/// chunk (the tag is read again for `LVLO`/`COED`):
/// - `EDID` (slot 0x134) and `OBND` (slot 0xE0) as the other forms;
/// - `LVLO`: a cleared 12-byte entry read (byte-swapped by `00488510` when the
///   file swaps) and added through `00487a60` on the list at +0x30 with its
///   16-bit level (+0), form id (+4), 16-bit count (+8) and 0; the result is
///   kept in `011c8e64`;
/// - `COED`: when an entry is kept, a 0xC-byte block (`00401000`) constructed
///   by `ContainerItemExtra::ContainerItemExtra` (`0040e690`; null if the
///   allocation fails), loaded from the file (`0040e780`), stored at the
///   entry's +8; the kept entry is then forgotten;
/// - `LVLD`: a byte read with size 0, passed to `0042e680` on +0x30;
/// - `LVLF`: read to +0x3D with size 0; `LVLG`: a word (`TESFile::GetChunkData`)
///   to +0x40;
/// - `MODL`/`MODT`/`MODD`/`MOSD`: `TESModel::LoadModelChunk` on +0x4C (0 when
///   `this` is null); `MODS`: the chunk into a buffer its size, unpacked by
///   `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`) on +0x4C
///   with the record version.
///
/// At the end clears form flag bit 0 and returns true.
pub fn fn_0050a930(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != 0x2d {
        return false;
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        match chunk {
            CHUNK_EDID => load_editor_id(e, base, file),
            CHUNK_OBND => {
                e.vcall(base, SLOT_LOAD_BOUND, &args![file]);
            }
            CHUNK_LVLO | CHUNK_COED => {
                let tag = e.call(FILE_GET_CHUNK, &args![file]).u32();
                if tag == CHUNK_COED {
                    let pending = e.global::<u32>(PENDING_LEVELED_ENTRY);
                    if pending != 0 {
                        let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                        let extra = if block != 0 {
                            e.call(0x0040_e690, &args![block]).u32()
                        } else {
                            0
                        };
                        e.call(0x0040_e780, &args![extra, file]);
                        let pending = e.global::<u32>(PENDING_LEVELED_ENTRY);
                        e.mem.set_u32(pending + 8, extra);
                        e.set_global::<u32>(PENDING_LEVELED_ENTRY, 0);
                    }
                } else if tag == CHUNK_LVLO {
                    let entry = e.with_stack(0xc, |e, local| {
                        e.call(MEMSET, &args![local, 0u32, 0xcu32]);
                        e.call(FILE_GET_CHUNK_DATA, &args![file, local, 0xcu32]);
                        if e.call(FILE_SWAPS, &args![file]).bool() {
                            e.call(0x0048_8510, &args![local]);
                        }
                        let level = u32::from(e.mem.u16(local.addr()));
                        let form = e.mem.u32(local.addr() + 4);
                        let count = u32::from(e.mem.u16(local.addr() + 8));
                        e.call(0x0048_7a60, &args![base + 0x30, level, count, form, 0u32])
                            .u32()
                    });
                    e.set_global::<u32>(PENDING_LEVELED_ENTRY, entry);
                }
            }
            CHUNK_LVLD => {
                let value = e.with_stack(4, |e, local| {
                    e.mem.set_u8(local.addr(), 0);
                    e.call(FILE_GET_CHUNK_DATA, &args![file, local, 0u32]);
                    e.mem.u8(local.addr())
                });
                e.call(0x0042_e680, &args![base + 0x30, u32::from(value)]);
            }
            CHUNK_LVLF => {
                e.call(FILE_GET_CHUNK_DATA, &args![file, base + 0x3d, 0u32]);
            }
            CHUNK_LVLG => {
                let value = e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), 0);
                    e.call(FILE_GET_CHUNK_WORD, &args![file, local]);
                    e.mem.u32(local.addr())
                });
                e.mem.set_u32(base + 0x40, value);
            }
            CHUNK_MODL | CHUNK_MODT | CHUNK_MODD | CHUNK_MOSD => {
                let model = if base == 0 { 0 } else { base + 0x4c };
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, size]);
                let version = u32::from(e.call(FILE_VERSION, &args![file]).u16());
                e.call(0x0048_a7b0, &args![base + 0x4c, buffer, size, version]);
                e.mem.free(buffer);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_9a10, fn_00509a10(Ptr) -> Ptr),
        entry!(0x0050_a5a0, fn_0050a5a0(Ptr) -> Ptr),
        entry!(0x0050_a770, fn_0050a770(Ptr) -> Ptr),
        entry!(0x0050_9a00, fn_00509a00(Ptr, u32) -> u32),
        entry!(0x0050_a590, fn_0050a590(Ptr, u32) -> u32),
        entry!(0x0050_a760, fn_0050a760(Ptr, u32) -> u32),
        entry!(0x0050_9c90, fn_00509c90(Ptr, u32) -> Ptr),
        entry!(0x0050_a670, fn_0050a670(Ptr, u32) -> Ptr),
        entry!(0x0050_a820, fn_0050a820(Ptr, u32) -> Ptr),
        entry!(0x0050_9ee0, fn_00509ee0(Ptr)),
        entry!(0x0050_9f50, fn_00509f50(Ptr)),
        entry!(0x0050_9f80, fn_00509f80(Ptr, Ptr) -> bool),
        entry!(0x0050_99c0, fn_005099c0(Ptr, u32) -> f32),
        entry!(0x0050_a550, fn_0050a550(Ptr) -> u32),
        entry!(0x0050_9b40, fn_00509b40(Ptr) -> bool),
        entry!(0x0050_9c20, fn_00509c20(Ptr) -> bool),
        entry!(0x0050_9bb0, fn_00509bb0(Ptr) -> bool),
        entry!(0x0050_9aa0, fn_00509aa0(Ptr) -> u8),
        entry!(0x0050_9cc0, fn_00509cc0(Ptr)),
        entry!(0x0050_a6a0, fn_0050a6a0(Ptr)),
        entry!(0x0050_a850, fn_0050a850(Ptr)),
        entry!(0x0050_a930, fn_0050a930(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00509a10_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_9a10,
            size: 0x68,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x24)]),
                (0x0050_9ee0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_814c), (0x30, 4, 0x0102_8128)],
        });
    }

    #[test]
    fn fn_0050a5a0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_a5a0,
            size: 0xac,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_a6a0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2e)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_8444),
                (0x30, 4, 0x0102_842c),
                (0x3c, 4, 0x0102_8408),
                (0x5c, 4, 0x0102_83e8),
                (0x68, 4, 0x0102_83d4),
                (0x74, 4, 0x0102_83b4),
                (0x7c, 4, 0x0102_83a0),
                (0x84, 4, 0x0102_838c),
                (0x8c, 4, 0x0102_8378),
                (0x9c, 4, 0x0102_8364),
            ],
        });
    }

    #[test]
    fn fn_0050a770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_a770,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2d)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_864c),
                (0x30, 4, 0x0102_862c),
                (0x4c, 4, 0x0102_8608),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00509a00() {
        assert_eq!(
            check(0x0050_9a00, 0x0050_8eb0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050a590() {
        assert_eq!(
            check(0x0050_a590, 0x0050_9c90, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050a760() {
        assert_eq!(
            check(0x0050_a760, 0x0050_a670, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_00509c90_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_9cc0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_9c90, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_9c90)
                .collect();
            let mut expected = vec![(0x0050_9cc0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050a670_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_a6a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_a670, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_a670)
                .collect();
            let mut expected = vec![(0x0050_a6a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050a820_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_a850, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_a820, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_a820)
                .collect();
            let mut expected = vec![(0x0050_a850, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00509ee0_sets_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        for b in 0x48..0x68 {
            e.mem.set_u8(this + b, 0xcc);
        }
        e.map(0x0101_6000, 0x9000);
        e.set_global::<u32>(0x0101_e340, 0x3f80_0000);
        e.set_global::<u32>(0x0101_df44, 0x7f80_0001);
        e.set_global::<u32>(0x0101_6248, 0x4120_0000);
        e.set_global::<u32>(0x0101_7b78, 0xbf00_0000);
        e.call(0x0050_9ee0, &[this]);
        assert_eq!(e.mem.u8(this + 0x48), 0x1e);
        assert_eq!(e.mem.u8(this + 0x49), 0);
        assert_eq!(e.mem.u8(this + 0x4a), 0x5a);
        assert_eq!(e.mem.u8(this + 0x4b), 0xcc);
        assert_eq!(e.mem.u16(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x54), 0x3f80_0000);
        // FLD/FSTP quiets a signalling NaN.
        assert_eq!(e.mem.u32(this + 0x58), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x5c), 0x4120_0000);
        assert_eq!(e.mem.u32(this + 0x60), 0xbf00_0000);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u8(this + 0x65), 0xcc);
    }

    #[test]
    fn at_00509f50_sets_flag_bit_1_unless_flag_8() {
        for (flag, sets) in [(1u32, false), (0x100, true)] {
            let mut e = Engine::new();
            e.register_double(0x0040_13e0, move |_, _| ret(flag));
            e.register(0x0048_4ab0, ok);
            e.call_log = Some(vec![]);
            e.call(0x0050_9f50, &[0x3000]);
            let log = e.call_log.take().unwrap();
            let expected = if sets { vec![vec![0x3000, 1]] } else { vec![] };
            assert_eq!(only(&log, 0x0048_4ab0), expected);
        }
    }

    fn load_engine(form_type: u32, chunks: Vec<u32>, swaps: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        let file = e.mem.alloc(0x300);
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x50]);
        e.mem.set_u32(0x0090_00e0, 0x00a0_00e0);
        e.mem.set_u32(0x0090_0134, 0x00a0_0134);
        e.mem.set_u32(this, 0x0090_0000);
        e.register(0x00a0_00e0, ok);
        e.register(0x00a0_0134, ok);
        e.register_double(0x0047_2660, move |_, _| ret(form_type));
        // GetTESChunk returns the current chunk: the double steps only when
        // the next-chunk call runs.
        let state = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let current = state.clone();
        let list = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            ret(list.get(current.get()).copied().unwrap_or(0))
        });
        let n = chunks.len();
        e.register_double(0x0047_26f0, move |_, _| {
            state.set(state.get() + 1);
            ret(u32::from(state.get() < n))
        });
        e.register(0x0040_1660, |_, _| ret(0x10));
        e.register(0x0047_2890, ok);
        e.register(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], 0x0bad_f00d);
            Ret::default()
        });
        e.register(0x0040_3570, |_, _| ret(0x2000f));
        e.register_double(0x0040_1680, move |_, _| ret(swaps));
        e.register(0x0040_3d30, ok);
        e.register(0x0048_5110, ok);
        e.register(0x0048_4ab0, ok);
        e.register(0x0048_92d0, ok);
        e.register(0x0048_61f0, ok);
        e.register(0x0050_a120, ok);
        (e, this, file)
    }

    #[test]
    fn at_00509f80_loads_each_chunk() {
        let (mut e, this, file) = load_engine(0x25, vec![], 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_9f80, &[this, file]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        let chunks = vec![
            0x4449_4445, // EDID
            0x444e_424f, // OBND
            0x4154_4144, // DATA
            0x5444_4f4d, // MODT
            0x4c44_4f4d, // MODL
            0x4444_4f4d, // MODD: not handled here
        ];
        let (mut e, this, file) = load_engine(0x124, chunks, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_9f80, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0040_3d30), vec![vec![this + 0x48, 0, 0x20]]);
        assert_eq!(only(&log, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 0]]);
        let reads = only(&log, 0x0047_2890);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(only(&log, 0x00a0_0134), vec![vec![this, reads[0][1]]]);
        assert_eq!(only(&log, 0x00a0_00e0), vec![vec![this, file]]);
        assert_eq!(
            only(&log, 0x0048_61f0),
            vec![vec![this, file, this + 0x48, 0x20]]
        );
        assert_eq!(only(&log, 0x0050_a120), vec![vec![this + 0x48]]);
        assert_eq!(only(&log, 0x0048_92d0).len(), 2);
        // The swap is asked only for DATA.
        assert_eq!(only(&log, 0x0040_1680).len(), 1);

        // No swap: DATA is not byte-swapped.
        let (mut e, this, file) = load_engine(0x24, vec![0x4154_4144], 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_9f80, &[this, file]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0050_a120).is_empty());
    }

    #[test]
    fn at_005099c0_reads_the_table_for_1_to_40() {
        let mut e = Engine::new();
        e.map(0x0118_9000, 0x2000);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x7f80_0002);
        e.set_global::<u32>(0x0118_9fac + 16 * 0x27, 0x5550);
        e.set_global::<u32>(0x0118_9fac, 0x5551);
        e.register_double(0x0040_3e20, move |_, a| {
            ret(if a[0] == 0x5550 { value } else { 0 })
        });
        let r = e.call(0x0050_99c0, &[0, 0x28]).f32();
        assert_eq!(r.to_bits(), 0x7fc0_0002);
        e.call_log = Some(vec![]);
        for index in [0u32, 0x29, 0xffff_ffff] {
            assert_eq!(e.call(0x0050_99c0, &[0, index]).f32(), 0.0);
        }
        assert!(only(&e.call_log.take().unwrap(), 0x0040_3e20).is_empty());
        e.mem.set_u32(value, 0x4040_0000);
        e.register_double(0x0040_3e20, move |_, a| {
            ret(if a[0] == 0x5551 { value } else { 0 })
        });
        assert_eq!(e.call(0x0050_99c0, &[0, 1]).f32(), 3.0);
    }

    #[test]
    fn at_0050a550_gives_the_name_or_the_constant() {
        for (name, expected, calls) in [(0x4444u32, 0x4444u32, 2usize), (0, 0x0101_1584, 1)] {
            let mut e = Engine::new();
            e.register_double(0x0055_9450, move |_, _| ret(name));
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0050_a550, &[0x3000]).u32(), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(only(&log, 0x0055_9450), vec![vec![0x3034]; calls]);
        }
    }

    fn flag_byte(addr: u32, byte: u8) -> bool {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        e.mem.set_u8(this + 0x64, byte);
        e.call(addr, &[this]).bool()
    }

    #[test]
    fn at_00509b40_tests_bit_1() {
        assert!(flag_byte(0x0050_9b40, 0x81));
        assert!(!flag_byte(0x0050_9b40, 0xfe));
    }

    #[test]
    fn at_00509c20_tests_bit_4() {
        assert!(flag_byte(0x0050_9c20, 0x04));
        assert!(!flag_byte(0x0050_9c20, 0xfb));
    }

    #[test]
    fn at_00509bb0_tests_bit_2() {
        assert!(flag_byte(0x0050_9bb0, 0x82));
        assert!(!flag_byte(0x0050_9bb0, 0xfd));
    }

    #[test]
    fn at_00509aa0_returns_the_byte_at_4a() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        e.mem.set_u8(this + 0x4a, 0x5a);
        e.mem.set_u8(this + 0x4b, 0x77);
        assert_eq!(e.call(0x0050_9aa0, &[this]).u8(), 0x5a);
    }

    #[test]
    fn at_00509cc0_restores_vtables_and_destroys() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        e.register(0x0048_9070, ok);
        e.register(0x0050_e9f0, ok);
        e.call_log = Some(vec![]);
        e.call(0x0050_9cc0, &[this]);
        let log: Log = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(e.mem.u32(this), 0x0102_814c);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_8128);
        assert_eq!(
            log,
            vec![(0x0048_9070, vec![this + 0x30]), (0x0050_e9f0, vec![this])]
        );
    }

    #[test]
    fn at_0050a6a0_restores_vtables_and_destroys() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xac);
        e.register(0x0048_3710, ok);
        e.register(0x0051_a840, ok);
        e.call_log = Some(vec![]);
        e.call(0x0050_a6a0, &[this]);
        let log: Log = e.call_log.take().unwrap().into_iter().skip(1).collect();
        for (offset, vtable) in [
            (0x00, 0x0102_8444),
            (0x30, 0x0102_842c),
            (0x3c, 0x0102_8408),
            (0x5c, 0x0102_83e8),
            (0x68, 0x0102_83d4),
            (0x74, 0x0102_83b4),
            (0x7c, 0x0102_83a0),
            (0x84, 0x0102_838c),
            (0x8c, 0x0102_8378),
            (0x9c, 0x0102_8364),
        ] {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        assert_eq!(
            log,
            vec![(0x0048_3710, vec![this]), (0x0051_a840, vec![this])]
        );
    }

    #[test]
    fn at_0050a850_restores_vtables_and_destroys() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.register(0x0048_a490, ok);
        e.register(0x0048_76d0, ok);
        e.register(0x0050_f740, ok);
        e.call_log = Some(vec![]);
        e.call(0x0050_a850, &[this]);
        let log: Log = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(e.mem.u32(this), 0x0102_864c);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_862c);
        assert_eq!(e.mem.u32(this + 0x4c), 0x0102_8608);
        assert_eq!(
            log,
            vec![
                (0x0048_a490, vec![this + 0x4c]),
                (0x0048_76d0, vec![this + 0x30]),
                (0x0050_f740, vec![this])
            ]
        );
    }

    fn leveled_engine(chunks: Vec<u32>, swaps: u32) -> (Engine, u32, u32) {
        let (mut e, this, file) = load_engine(0x2d, chunks, swaps);
        e.map(0x011c_8000, 0x1000);
        e.register(0x0047_2890, |e, a| {
            if a[2] == 0xc {
                e.mem.set_u16(a[1], 7);
                e.mem.set_u32(a[1] + 4, 0x0001_2345);
                e.mem.set_u16(a[1] + 8, 3);
            }
            if a[2] == 0 {
                e.mem.set_u8(a[1], 0x42);
            }
            Ret::default()
        });
        e.register(0x0048_8510, ok);
        e.register(0x0048_7a60, |_, _| ret(0x0077_0000));
        e.register(0x0040_1000, |e, _| ret(e.mem.alloc(0xc)));
        e.register(0x0040_e690, |_, a| ret(a[0]));
        e.register(0x0040_e780, ok);
        e.register(0x0042_e680, ok);
        e.register(0x0048_a7b0, ok);
        (e, this, file)
    }

    #[test]
    fn at_0050a930_loads_a_leveled_list() {
        let (mut e, this, file) = leveled_engine(vec![], 0);
        e.register(0x0047_2660, |_, _| ret(0x2c));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_a930, &[this, file]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        let chunks = vec![
            0x4449_4445, // EDID
            0x444e_424f, // OBND
            0x444c_564c, // LVLD
            0x464c_564c, // LVLF
            0x474c_564c, // LVLG
            0x4f4c_564c, // LVLO
            0x4445_4f43, // COED
            0x4445_4f43, // COED without a pending entry
            0x4444_4f4d, // MODD
            0x4453_4f4d, // MOSD
            0x5344_4f4d, // MODS
            0x1111_1111, // not handled
        ];
        let (mut e, this, file) = leveled_engine(chunks, 1);
        let entry = 0x0077_0000;
        e.map(entry, 0x1000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_a930, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(only(&log, 0x00a0_00e0), vec![vec![this, file]]);
        let reads = only(&log, 0x0047_2890);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(only(&log, 0x00a0_0134), vec![vec![this, reads[0][1]]]);
        // LVLD: a byte read with size 0 goes to 0042e680.
        assert_eq!(reads[1][2], 0);
        assert_eq!(only(&log, 0x0042_e680), vec![vec![this + 0x30, 0x42]]);
        // LVLF: straight to +0x3D, size 0.
        assert_eq!(reads[2], vec![file, this + 0x3d, 0]);
        assert_eq!(e.mem.u32(this + 0x40), 0x0bad_f00d);
        // LVLO: cleared, read, swapped, added with level, count, form id, 0.
        assert_eq!(reads[3][2], 0xc);
        assert_eq!(only(&log, 0x0040_3d30), vec![vec![reads[3][1], 0, 0xc]]);
        assert_eq!(only(&log, 0x0048_8510), vec![vec![reads[3][1]]]);
        assert_eq!(
            only(&log, 0x0048_7a60),
            vec![vec![this + 0x30, 7, 3, 0x0001_2345, 0]]
        );
        // COED: one block for the pending entry, stored at its +8.
        let blocks = only(&log, 0x0040_1000);
        assert_eq!(blocks, vec![vec![0xc]]);
        let block = only(&log, 0x0040_e690)[0][0];
        assert_eq!(only(&log, 0x0040_e780), vec![vec![block, file]]);
        assert_eq!(e.mem.u32(entry + 8), block);
        assert_eq!(e.global::<u32>(0x011c_8e64), 0);
        // MODD and MOSD load the model at +0x4C; MODS unpacks with the version.
        assert_eq!(
            only(&log, 0x0048_92d0),
            vec![vec![this + 0x4c, file], vec![this + 0x4c, file]]
        );
        assert_eq!(reads[4][2], 0x10);
        assert_eq!(
            only(&log, 0x0048_a7b0),
            vec![vec![this + 0x4c, reads[4][1], 0x10, 0xf]]
        );
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 0]]);
    }

    #[test]
    fn at_0050a930_lvlo_keeps_the_entry_and_a_failed_allocation_stores_null() {
        let (mut e, this, file) = leveled_engine(vec![0x4f4c_564c, 0x4445_4f43], 0);
        e.map(0x0077_0000, 0x1000);
        e.mem.set_u32(0x0077_0008, 0x5555);
        e.register(0x0040_1000, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_a930, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0048_8510).is_empty());
        assert!(only(&log, 0x0040_e690).is_empty());
        assert_eq!(only(&log, 0x0040_e780), vec![vec![0, file]]);
        assert_eq!(e.mem.u32(0x0077_0008), 0);

        // LVLO alone: the entry stays pending.
        let (mut e, this, file) = leveled_engine(vec![0x4f4c_564c], 0);
        assert!(e.call(0x0050_a930, &[this, file]).bool());
        assert_eq!(e.global::<u32>(0x011c_8e64), 0x0077_0000);
    }
}
