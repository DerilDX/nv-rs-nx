//! `fallout shared/run_0050ad90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050adf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050adf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2c (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050adf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_886c);
    e.mem.set_u32(base + 0x30, 0x0102_884c);
    e.mem.set_u32(base + 0x4c, 0x0102_8828);
    e.call(0x004f_15a0, &args![this, 0x2c]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050b400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b400 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x34 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b400(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8a64);
    e.mem.set_u32(base + 0x30, 0x0102_8a44);
    e.call(0x004f_15a0, &args![this, 0x34]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 0050b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4c (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8c5c);
    e.mem.set_u32(base + 0x30, 0x0102_8c38);
    e.call(0x004f_15a0, &args![this, 0x4c]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050a820`. `0050a820` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050ade0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_a820, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050aea0`. `0050aea0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050b3f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_aea0, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050aed0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050aea0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_aed0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b4c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b4c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b9b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b9b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0043d4d0` (`thiscall`): answers the address of a static's value.
const STATIC_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// The statics `0050ad90`, `0050b370` and `0050b8d0` read through `0043d4d0`.
const LEV_CREATURE_STATIC: u32 = 0x011c_8e1c;
const LEV_ITEM_STATIC: u32 = 0x011c_8ed4;
const LEV_SPELL_STATIC: u32 = 0x011c_8f00;
/// The `LEVELED_OBJECT` the last `LVLO` chunk added, which a following `COED`
/// chunk gives its extra (one per class: `TESLevCreature`, `TESLevItem`).
const LAST_LEV_CREATURE_OBJECT: u32 = 0x011c_8eec;
const LAST_LEV_ITEM_OBJECT: u32 = 0x011c_8f1c;
/// `TESFile::GetTESForm` (`00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (`004726b0`): the current chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false at the end of the record.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, `RET 4`).
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` overload (`00472890`, `RET 8`: address, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401660` (`thiscall` on the file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`thiscall` on the file): whether form ids read need fixing up
/// (`00488510`); the meaning is not checked here.
const FILE_FIXES_IDS: u32 = 0x0040_1680;
/// `00488510` (`thiscall` on the 12-byte `LVLO` data): fixes it up.
const FIX_UP_LVLO: u32 = 0x0048_8510;
/// `TESForm::LoadForm` (`00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`thiscall`, `RET 4`).
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `00403d30` (`cdecl`, three words: address, byte, count): fills memory
/// (the engine map's name for it is a library one).
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `00487a60` (`thiscall`, `RET 0x10`): adds a `LEVELED_OBJECT`.
const LEVELED_LIST_ADD: u32 = 0x0048_7a60;
/// `0042e680` (`thiscall`, `RET 4`): sets the leveled list's chance of none.
const SET_CHANCE_NONE: u32 = 0x0042_e680;
/// `00401000`: allocate (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
/// `ContainerItemExtra::ContainerItemExtra` (`0040e690`).
const CONTAINER_ITEM_EXTRA_CONSTRUCT: u32 = 0x0040_e690;
/// `0040e780` (`thiscall`, `RET 4`, the file): loads a `ContainerItemExtra`.
const CONTAINER_ITEM_EXTRA_LOAD: u32 = 0x0040_e780;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: the model, the file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `00403570` (`thiscall` on the file): a 16-bit word passed on to the
/// texture-swap unpacking.
const FILE_WORD: u32 = 0x0040_3570;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`, `RET 0xc`).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// The form vtable slots (byte offsets) the loads call: `0x134` with the
/// editor id read, `0xe0` with the file for `OBND`.
const SET_EDITOR_ID_SLOT: u32 = 0x134;
const LOAD_BOUNDS_SLOT: u32 = 0xe0;
/// `TESForm::TESForm`-side helpers the destructors and `InitItem` call:
/// `0050e9f0` (the form base destructor), `0048a490` (the texture-swap model
/// destructor), `004013e0` (a `bool`), `00488570` (the leveled list's
/// `InitItem`, the form passed), `00484d60` and `00488c50` (`RET 4`).
const FORM_BASE_DESTRUCT: u32 = 0x0050_e9f0;
const MODEL_DESTRUCT: u32 = 0x0048_a490;
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
const LEVELED_LIST_INIT_ITEM: u32 = 0x0048_8570;
const FORM_COPY: u32 = 0x0048_4d60;
const LEVELED_LIST_COPY: u32 = 0x0048_8c50;
/// `004876d0`: the `TESLeveledList` destructor body.
const LEVELED_LIST_DESTRUCT: u32 = 0x0048_76d0;
/// `TESLevCreature` and `TESLevItem` vtables (RTTI checked) and their bases'.
const LEV_CREATURE_VTABLE: u32 = 0x0102_886c;
const LEV_CREATURE_LIST_VTABLE: u32 = 0x0102_884c;
const LEV_CREATURE_MODEL_VTABLE: u32 = 0x0102_8828;
const LEV_ITEM_VTABLE: u32 = 0x0102_8a64;
const LEV_ITEM_LIST_VTABLE: u32 = 0x0102_8a44;
/// Chunk tags.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_LVLD: u32 = 0x444c_564c;
const CHUNK_LVLF: u32 = 0x464c_564c;
const CHUNK_LVLG: u32 = 0x474c_564c;
const CHUNK_LVLO: u32 = 0x4f4c_564c;
const CHUNK_COED: u32 = 0x4445_4f43;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;

/// `__alloca_probe_16` (`00ec5ec0`) of the chunk's size: the stack block,
/// rounded up to 16 bytes as the probe does, for the duration of `body`.
fn with_chunk_stack<R>(e: &mut Engine, size: u32, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(size.wrapping_add(15) & !15, |e, block| {
        body(e, block.addr())
    })
}

/// The `EDID` case of both loads: the chunk (at most 0x200 bytes) read into
/// a stack block of the chunk's size, handed to vtable `+0x134`.
fn load_editor_id(e: &mut Engine, this: Ptr, file: Ptr) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    with_chunk_stack(e, size, |e, block| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, block, 0x200u32]);
        e.vcall(this.addr(), SET_EDITOR_ID_SLOT, &args![block]);
    });
}

/// The `LVLO`/`COED` case of both loads (the tag is read again): an `LVLO`
/// (12 bytes, cleared first, fixed up when `00401680` says so) adds an object
/// to the leveled list at +0x30 with its level (+0), count (+8) and form
/// (+4) and remembers it in `last_object`; a `COED` gives the remembered
/// object a new `ContainerItemExtra` (allocation failure leaves null) loaded
/// from the file, and forgets it.
fn load_leveled_object(e: &mut Engine, this: Ptr, file: Ptr, last_object: u32) {
    let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
    if tag == CHUNK_COED {
        if e.global::<u32>(last_object) != 0 {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let extra = if block != 0 {
                e.call(CONTAINER_ITEM_EXTRA_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.call(CONTAINER_ITEM_EXTRA_LOAD, &args![extra, file]);
            let object = e.global::<u32>(last_object);
            e.mem.set_u32(object.wrapping_add(8), extra);
            e.set_global::<u32>(last_object, 0);
        }
    } else if tag == CHUNK_LVLO {
        e.with_stack(0xc, |e, data| {
            let data = data.addr();
            e.call(FILL_MEMORY, &args![data, 0u32, 0xcu32]);
            e.call(GET_CHUNK_DATA_SIZED, &args![file, data, 0xcu32]);
            if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                e.call(FIX_UP_LVLO, &args![data]);
            }
            let form = e.mem.u32(data + 4);
            let count = u32::from(e.mem.u16(data + 8));
            let level = u32::from(e.mem.u16(data));
            let object = e
                .call(
                    LEVELED_LIST_ADD,
                    &args![this.addr().wrapping_add(0x30), level, count, form, 0u32],
                )
                .u32();
            e.set_global::<u32>(last_object, object);
        });
    }
}

/// The `LVLD` case: one byte read (cleared first) as the chance of none.
fn load_chance_none(e: &mut Engine, this: Ptr, file: Ptr) {
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        e.call(GET_CHUNK_DATA_SIZED, &args![file, byte, 0u32]);
        let chance = u32::from(e.mem.u8(byte.addr()));
        e.call(
            SET_CHANCE_NONE,
            &args![this.addr().wrapping_add(0x30), chance],
        );
    });
}

/// The `LVLG` case: the global's form id (cleared first) into the leveled
/// list's `pChanceGlobal` (+0x40).
fn load_chance_global(e: &mut Engine, this: Ptr, file: Ptr) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_DATA, &args![file, word]);
        let value = e.mem.u32(word.addr());
        e.mem.set_u32(this.addr().wrapping_add(0x40), value);
    });
}

// Translated from 0050af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLevCreature`'s load (unnamed in the engine map; the class by the
/// form type 0x2c and the vtables its constructor sets), `thiscall`,
/// `RET 4`: the file. Answers false for another record type. Otherwise
/// `TESForm::LoadForm`, then each chunk: `EDID` (vtable `+0x134`), `OBND`
/// (vtable `+0xe0`), `LVLD` (chance of none), `LVLF` (the flags byte at
/// +0x3d, size 0), `LVLG` (the chance global), `LVLO`/`COED` (objects and
/// their extras), `MODL`/`MODT`/`MODD`/`MOSD` (`TESModel::LoadModelChunk` on
/// the model at +0x4c) and `MODS` (texture swaps: the chunk into a stack
/// block of its size, unpacked with `00403570`'s word). At the end
/// `00484ab0` with 0; answers true.
// Translation incomplete: 0050af60 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_0050af60(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x2c {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        match tag {
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_COED | CHUNK_LVLO => load_leveled_object(e, this, file, LAST_LEV_CREATURE_OBJECT),
            CHUNK_LVLD => load_chance_none(e, this, file),
            CHUNK_LVLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, this.addr().wrapping_add(0x3d), 0u32],
                );
            }
            CHUNK_LVLG => load_chance_global(e, this, file),
            CHUNK_MODL | CHUNK_MODT | CHUNK_MODD | CHUNK_MOSD => {
                let model = if this.is_null() {
                    0
                } else {
                    this.addr().wrapping_add(0x4c)
                };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                with_chunk_stack(e, size, |e, block| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, block, size]);
                    let word = u32::from(e.call(FILE_WORD, &args![file]).u16());
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![this.addr().wrapping_add(0x4c), block, size, word],
                    );
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

// Translated from 0050b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLevItem`'s load (unnamed in the engine map; the class by the form
/// type 0x34 and the vtables its constructor sets), `thiscall`, `RET 4`: the
/// file. As `0050af60` without the model chunks: answers false for another
/// record type; otherwise `TESForm::LoadForm`, then `EDID`, `OBND`, `LVLD`,
/// `LVLF`, `LVLG` and `LVLO`/`COED` (remembering the last object in its own
/// static), `00484ab0` with 0 at the end; answers true.
// Translation incomplete: 0050b560 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_0050b560(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x34 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        match tag {
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_COED | CHUNK_LVLO => load_leveled_object(e, this, file, LAST_LEV_ITEM_OBJECT),
            CHUNK_LVLD => load_chance_none(e, this, file),
            CHUNK_LVLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, this.addr().wrapping_add(0x3d), 0u32],
                );
            }
            CHUNK_LVLG => load_chance_global(e, this, file),
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

/// The shape of `0050ad90`, `0050b370` and `0050b8d0`: the word at the
/// address `0043d4d0` answers for the static (`this` is unused).
fn static_word(e: &mut Engine, holder: u32) -> u32 {
    let address = e.call(STATIC_VALUE_ADDRESS, &args![holder]).u32();
    e.mem.u32(address)
}

// Translated from 0050ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8e1c` (`0043d4d0`).
pub fn fn_0050ad90(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_CREATURE_STATIC)
}

// Translated from 0050b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8ed4` (`0043d4d0`).
pub fn fn_0050b370(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_ITEM_STATIC)
}

// Translated from 0050b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8f00` (`0043d4d0`).
pub fn fn_0050b8d0(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_SPELL_STATIC)
}

// Translated from 0050adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): passes the word to
/// `00484d60` on the form and to `00488c50` on its leveled list (+0x30).
pub fn fn_0050adb0(e: &mut Engine, this: Ptr, other: u32) {
    e.call(FORM_COPY, &args![this, other]);
    e.call(
        LEVELED_LIST_COPY,
        &args![this.addr().wrapping_add(0x30), other],
    );
}

// Translated from 0050b890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless `004013e0` answers true, runs the
/// leveled list's `InitItem` (`00488570` on +0x30 with the form) and
/// `00484ab0` with 1.
pub fn fn_0050b890(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIPS_INIT, &args![this]).bool() {
        return;
    }
    e.call(
        LEVELED_LIST_INIT_ITEM,
        &args![this.addr().wrapping_add(0x30), this],
    );
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 0050aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLevCreature` destructor body (unnamed in the engine map; its
/// vtables by RTTI): sets the three vtables, destroys the model (+0x4c,
/// `0048a490`) and the leveled list (+0x30, `004876d0`), then the form base
/// (`0050e9f0`).
// Translation incomplete: 0050aed0 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_0050aed0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEV_CREATURE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x30), LEV_CREATURE_LIST_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x4c), LEV_CREATURE_MODEL_VTABLE);
    e.call(MODEL_DESTRUCT, &args![base.wrapping_add(0x4c)]);
    e.call(LEVELED_LIST_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 0050b4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLevItem` destructor body (unnamed in the engine map; its vtables
/// by RTTI): sets the two vtables, destroys the leveled list (+0x30,
/// `004876d0`), then the form base (`0050e9f0`).
// Translation incomplete: 0050b4c0 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_0050b4c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEV_ITEM_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x30), LEV_ITEM_LIST_VTABLE);
    e.call(LEVELED_LIST_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_adf0, fn_0050adf0(Ptr) -> Ptr),
        entry!(0x0050_b400, fn_0050b400(Ptr) -> Ptr),
        entry!(0x0050_b8f0, fn_0050b8f0(Ptr) -> Ptr),
        entry!(0x0050_ade0, fn_0050ade0(Ptr, u32) -> u32),
        entry!(0x0050_b3f0, fn_0050b3f0(Ptr, u32) -> u32),
        entry!(0x0050_aea0, fn_0050aea0(Ptr, u32) -> Ptr),
        entry!(0x0050_b490, fn_0050b490(Ptr, u32) -> Ptr),
        entry!(0x0050_b980, fn_0050b980(Ptr, u32) -> Ptr),
        entry!(0x0050_af60, fn_0050af60(Ptr, Ptr) -> bool),
        entry!(0x0050_b560, fn_0050b560(Ptr, Ptr) -> bool),
        entry!(0x0050_ad90, fn_0050ad90(Ptr) -> u32),
        entry!(0x0050_b370, fn_0050b370(Ptr) -> u32),
        entry!(0x0050_b8d0, fn_0050b8d0(Ptr) -> u32),
        entry!(0x0050_adb0, fn_0050adb0(Ptr, u32)),
        entry!(0x0050_b890, fn_0050b890(Ptr)),
        entry!(0x0050_aed0, fn_0050aed0(Ptr)),
        entry!(0x0050_b4c0, fn_0050b4c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050adf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_adf0,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2c)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_886c),
                (0x30, 4, 0x0102_884c),
                (0x4c, 4, 0x0102_8828),
            ],
        });
    }

    #[test]
    fn fn_0050b400_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b400,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x34)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_8a64), (0x30, 4, 0x0102_8a44)],
        });
    }

    #[test]
    fn fn_0050b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b8f0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4c)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0102_8c5c), (0x30, 4, 0x0102_8c38)],
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
    fn le81_thunk_0050ade0() {
        assert_eq!(
            check(0x0050_ade0, 0x0050_a820, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050b3f0() {
        assert_eq!(
            check(0x0050_b3f0, 0x0050_aea0, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_0050aea0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_aed0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_aea0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_aea0)
                .collect();
            let mut expected = vec![(0x0050_aed0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b490_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b4c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b490, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b490)
                .collect();
            let mut expected = vec![(0x0050_b4c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b980_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b9b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b980, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b980)
                .collect();
            let mut expected = vec![(0x0050_b9b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const EDITOR_ID_FN: u32 = 0x00ab_1000;
    const BOUNDS_FN: u32 = 0x00ab_1004;

    type Chunks = Rc<RefCell<(usize, Vec<(u32, Vec<u8>)>)>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A file of the given record type and chunks (tag, data): the chunk
    /// readers copy the current chunk's data (at most the size asked for, all
    /// of it for size 0).
    fn engine(record_type: u8, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        let state: Chunks = Rc::new(RefCell::new((0, chunks)));
        e.register_double(GET_TES_FORM, move |_, _| u32::from(record_type).into_ret());
        let s = state.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.1.get(s.0).map_or(0, |c| c.0).into_ret()
        });
        let s = state.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.0 += 1;
            u32::from(s.0 < s.1.len()).into_ret()
        });
        let s = state.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            (s.1[s.0].1.len() as u32).into_ret()
        });
        let s = state.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            let n = if a[2] == 0 {
                data.len()
            } else {
                data.len().min(a[2] as usize)
            };
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let s = state;
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONTAINER_ITEM_EXTRA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        e.register(FILE_WORD, |_, _| 0x1_0007u32.into_ret());
        e.register(EDITOR_ID_FN, |_, _| Ret::default());
        e.register(BOUNDS_FN, |_, _| Ret::default());
        for addr in [
            LOAD_FORM,
            FORM_SET_FLAG,
            FIX_UP_LVLO,
            SET_CHANCE_NONE,
            CONTAINER_ITEM_EXTRA_LOAD,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LEVELED_LIST_ADD, |e, _| e.mem.alloc(0xc).into_ret());
        let mut slots = vec![0u32; 0x4e];
        slots[0x134 / 4] = EDITOR_ID_FN;
        slots[0xe0 / 4] = BOUNDS_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e
    }

    fn lvlo(level: u16, form: u32, count: u16) -> Vec<u8> {
        let mut v = level.to_le_bytes().to_vec();
        v.extend([0, 0]);
        v.extend(form.to_le_bytes());
        v.extend(count.to_le_bytes());
        v.extend([0, 0]);
        v
    }

    fn chunks() -> Vec<(u32, Vec<u8>)> {
        vec![
            (CHUNK_EDID, b"LL\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_LVLD, vec![25]),
            (CHUNK_LVLF, vec![3]),
            (CHUNK_LVLG, 0x1234u32.to_le_bytes().to_vec()),
            (CHUNK_LVLO, lvlo(5, 0xabcd, 2)),
            (CHUNK_COED, vec![0; 12]),
            (CHUNK_MODL, b"a.nif\0".to_vec()),
            (CHUNK_MODS, vec![1, 2, 3]),
        ]
    }

    fn form(e: &mut Engine) -> u32 {
        let f = e.mem.alloc(0x70);
        e.mem.set_u32(f, FORM_VTABLE);
        f
    }

    #[test]
    fn fn_0050af60_loads_every_chunk_of_a_leveled_creature() {
        let mut e = engine(0x2c, chunks());
        let this = form(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_af60, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, LOAD_FORM), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, EDITOR_ID_FN).len(), 1);
        assert_eq!(calls_to(&e, BOUNDS_FN), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, SET_CHANCE_NONE), vec![vec![this + 0x30, 25]]);
        assert_eq!(e.mem.u8(this + 0x3d), 3);
        assert_eq!(e.mem.u32(this + 0x40), 0x1234);
        assert_eq!(
            calls_to(&e, LEVELED_LIST_ADD),
            vec![vec![this + 0x30, 5, 2, 0xabcd, 0]]
        );
        assert_eq!(calls_to(&e, FIX_UP_LVLO).len(), 1);
        // The COED gave the object an extra and cleared the static.
        let extra = calls_to(&e, CONTAINER_ITEM_EXTRA_LOAD)[0][0];
        assert_ne!(extra, 0);
        assert_eq!(e.global::<u32>(LAST_LEV_CREATURE_OBJECT), 0);
        assert_eq!(
            calls_to(&e, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x4c, 0x9000]]
        );
        let swap = calls_to(&e, UNPACK_TEXTURE_SWAP);
        assert_eq!(swap.len(), 1);
        assert_eq!((swap[0][0], swap[0][2], swap[0][3]), (this + 0x4c, 3, 7));
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        // Another record type: false, nothing read.
        let mut e = engine(0x34, chunks());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_af60, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0050b560_loads_a_leveled_item_without_models() {
        let mut e = engine(0x34, chunks());
        let this = form(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, LEVELED_LIST_ADD).len(), 1);
        assert_eq!(calls_to(&e, CONTAINER_ITEM_EXTRA_LOAD).len(), 1);
        assert_eq!(e.global::<u32>(LAST_LEV_ITEM_OBJECT), 0);
        assert!(calls_to(&e, LOAD_MODEL_CHUNK).is_empty());
        assert!(calls_to(&e, UNPACK_TEXTURE_SWAP).is_empty());
        assert_eq!(e.mem.u32(this + 0x40), 0x1234);
        // An LVLO without COED leaves the object remembered; a COED without
        // one does nothing.
        let mut e = engine(
            0x34,
            vec![(CHUNK_COED, vec![]), (CHUNK_LVLO, lvlo(1, 2, 3))],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_ne!(e.global::<u32>(LAST_LEV_ITEM_OBJECT), 0);
        let mut e = engine(0x2c, chunks());
        assert!(!e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
    }

    #[test]
    fn static_words_read_through_0043d4d0() {
        let mut e = Engine::new();
        e.register(STATIC_VALUE_ADDRESS, |_, a| (a[0] + 0x100).into_ret());
        e.map(0x011c_8000, 0x2000);
        e.mem.set_u32(LEV_CREATURE_STATIC + 0x100, 11);
        e.mem.set_u32(LEV_ITEM_STATIC + 0x100, 22);
        e.mem.set_u32(LEV_SPELL_STATIC + 0x100, 33);
        assert_eq!(e.call(0x0050_ad90, &args![0u32]).u32(), 11);
        assert_eq!(e.call(0x0050_b370, &args![0u32]).u32(), 22);
        assert_eq!(e.call(0x0050_b8d0, &args![0u32]).u32(), 33);
    }

    #[test]
    fn fn_0050adb0_and_0050b890_pass_on_to_the_form_and_its_list() {
        let mut e = Engine::new();
        for addr in [
            FORM_COPY,
            LEVELED_LIST_COPY,
            LEVELED_LIST_INIT_ITEM,
            FORM_SET_FLAG,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_SKIPS_INIT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_adb0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, FORM_COPY), vec![vec![0x5000, 0x6000]]);
        assert_eq!(calls_to(&e, LEVELED_LIST_COPY), vec![vec![0x5030, 0x6000]]);
        e.call(0x0050_b890, &args![0x5000u32]);
        assert_eq!(
            calls_to(&e, LEVELED_LIST_INIT_ITEM),
            vec![vec![0x5030, 0x5000]]
        );
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![0x5000, 1]]);
        e.register(FORM_SKIPS_INIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_b890, &args![0x5000u32]);
        assert!(calls_to(&e, LEVELED_LIST_INIT_ITEM).is_empty());
    }

    #[test]
    fn destructors_set_vtables_and_destroy_the_bases() {
        let mut e = Engine::new();
        for addr in [MODEL_DESTRUCT, LEVELED_LIST_DESTRUCT, FORM_BASE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x70);
        e.call_log = Some(vec![]);
        e.call(0x0050_aed0, &args![this]);
        assert_eq!(
            [
                e.mem.u32(this),
                e.mem.u32(this + 0x30),
                e.mem.u32(this + 0x4c)
            ],
            [
                LEV_CREATURE_VTABLE,
                LEV_CREATURE_LIST_VTABLE,
                LEV_CREATURE_MODEL_VTABLE
            ]
        );
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (MODEL_DESTRUCT, vec![this + 0x4c]),
                (LEVELED_LIST_DESTRUCT, vec![this + 0x30]),
                (FORM_BASE_DESTRUCT, vec![this]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0050_b4c0, &args![this]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 0x30)],
            [LEV_ITEM_VTABLE, LEV_ITEM_LIST_VTABLE]
        );
        assert_eq!(calls_to(&e, MODEL_DESTRUCT).len(), 0);
        assert_eq!(calls_to(&e, LEVELED_LIST_DESTRUCT), vec![vec![this + 0x30]]);
    }
}
