//! `fallout/misc/saveload/bgsreconstructformsinallfilesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions marked `Translation incomplete` have an exception-unwinding
//! frame (`FS:[0]`) that is not translated; everything else in them is.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00844db0(this, bucket_count)`: the hash-table base initializer below.
/// `00844130` and `008433d0` pass `0x25` as the bucket count.
const BUCKET_COUNT: u32 = 0x25;
/// `008453a0(this, 0, 0)` (`run_00845230`): runs after `00844a90` stores its
/// vtable.
const MAP_INITIALIZER: u32 = 0x0084_53a0;
/// `00aa1070`: the allocator (cdecl, byte count), as `00853270` uses it.
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;

// Translated from 00844db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed1c`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the bucket table
/// (+8) and zeroes it. Returns `this`.
pub fn fn_00844db0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0107_ed1c);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00844130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00844db0(this, bucket_count)`, then the vtable
/// `0107ec48` is stored. Returns `this`.
pub fn fn_00844130(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00844db0(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0107_ec48);
    this
}

// Translated from 00844a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed08` and runs
/// `008453a0(this, 0, 0)`. Returns `this`.
pub fn fn_00844a90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_ed08);
    e.call(MAP_INITIALIZER, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00843470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00844a90` on `this` and on the member at
/// +0x10. Returns `this`.
// Translation incomplete: 00843470 - SEH exception unwinding is unsupported.
pub fn fn_00843470(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844a90(e, this);
    fn_00844a90(e, this.byte_add(0x10).cast());
    this
}

// Translated from 008433d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::BGSReconstructFormsInAllFilesMap`
/// (Xbox PDB): the base `00844130(this, 0x25)`, the vtable `0107ea74`, the
/// member at +0x10 through `00843470`, and 0 at +0x30. Returns `this`.
// Translation incomplete: 008433d0 - SEH exception unwinding is unsupported.
pub fn bgsreconstructformsinallfilesmap_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844130(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), 0x0107_ea74);
    fn_00843470(e, this.byte_add(0x10).cast());
    e.mem.set_u32(this.addr() + 0x30, 0);
    this
}

// Translated from 00843220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00844ff0`, a destructor with an exception frame
/// that stores the vtable `0107ed3c` and runs `008454f0` and `00844fa0`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00844ff0`'s disassembly ends with its exception-frame
/// unlink and returns no value (Ghidra types it void).
pub fn fn_00843220(e: &mut Engine, this: Ptr) {
    e.call(0x0084_4ff0, &args![this]);
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0107ED08: u32 = 0x0107_ed08;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00844ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00844ac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0107ED08);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// The `NiTMap` iteration (Xbox PDB name unconfirmed): the first position
/// (`004b9ba0`) and the next entry (`006b7f20`, `RET 0xC`: the position,
/// the key and the value, each by address).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
/// Empties the map (`00438af0`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// The element count of one of the two arrays (`0044ddc0`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// The array's reallocation (`006a2770`, `RET 8`: new size, count kept),
/// its element destruction (`0072ba80`, `RET 8`: first element, count) and
/// construction (`00845300`, `RET 8`: first element, count).
const ARRAY_REALLOCATE: u32 = 0x006a_2770;
const ARRAY_DESTROY: u32 = 0x0072_ba80;
const ARRAY_CONSTRUCT: u32 = 0x0084_5300;
/// The `NiTMap<TESFile *, BGSReconstructFormsInFileMap *>` vtable (stored
/// by `00844130` and `00844e20`) and `BGSReconstructFormsInAllFilesMap`'s.
const FILE_MAP_VTABLE: u32 = 0x0107_ec48;
const ALL_FILES_MAP_VTABLE: u32 = 0x0107_ea74;
/// The TLS word whose bit 4 `00843c10` sets or clears (`+0x294`).
const TLS_FLAGS: u32 = 0x294;

// Translated from 00843150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at `+0x14`.
pub fn fn_00843150(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x14)
}

// Translated from 00843170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the words at `+0` and `+4`.
pub fn fn_00843170(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 00843240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties both arrays (at `+0` and `+0x10`) and frees their memory
/// (`00844ae0` with 0 and 1).
pub fn fn_00843240(e: &mut Engine, this: Ptr) {
    fn_00844ae0(e, this, 0, 1);
    fn_00844ae0(e, this.byte_add(0x10), 0, 1);
}

// Translated from 00843440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor (`008434d0`), then frees `this` (`00401030`) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bgs_reconstruct_forms_in_all_files_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bgs_reconstruct_forms_in_all_files_map_destructor(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 008434d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008434d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSReconstructFormsInAllFilesMap::~BGSReconstructFormsInAllFilesMap`
/// (Xbox PDB): stores the vtable `0107ea74`, deletes the per-file maps
/// (`008435a0`), destroys the two arrays at `+0x10` (`00843540`) and runs
/// the `NiTMap` destructor (`00844e20`).
pub fn bgs_reconstruct_forms_in_all_files_map_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ALL_FILES_MAP_VTABLE);
    fn_008435a0(e, this);
    fn_00843540(e, this.byte_add(0x10));
    fn_00844e20(e, this);
}

// Translated from 00843540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843540 - the exception-unwinding frame (FS:[0]) is not translated.
/// Destroys the two arrays: `00844ac0` on `+0x10`, then on `+0`.
pub fn fn_00843540(e: &mut Engine, this: Ptr) {
    fn_00844ac0(e, this.byte_add(0x10));
    fn_00844ac0(e, this);
}

// Translated from 008435a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every per-file map: walks the map (`004b9ba0`, `006b7f20` with a
/// cleared key and value), running vtable slot 0 with 1 (the deleting
/// destructor) on each value that is not null; then empties the map
/// (`00438af0`), empties both arrays (`00843240` on `+0x10`) and clears
/// the word at `+0x30`.
pub fn fn_008435a0(e: &mut Engine, this: Ptr) {
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let value = e.mem.u32(value_at.addr());
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00843240(e, this.byte_add(0x10));
    e.mem.set_u32(this.addr() + 0x30, 0);
}

// Translated from 008439e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reconstructs the forms of every file. Sets
/// the TLS flag (`00843c10` with 1 on `*011ddf38`), reserves `+0x30`
/// entries in both arrays (`00843be0`), saves the answer of `004516b0` on
/// `*011c3f2c` and sets it with `00550890(1)`; for every entry of the map
/// (`004b9ba0`, `006b7f20`), `BGSReconstructFormsInFileMap::
/// ReconstructForms` (`008427f0`) runs on the value with the key's
/// thread-safe file (`TESFile::GetThreadSafeFile`, `004739b0`) and the
/// arrays; the saved setting is put back. Then every entry of the arrays
/// (`00843bb0` entries, `008432e0`) whose form answers true to vtable slot
/// 0xF0 but has no base form (`007af430`) is removed (`00843380`) and the
/// index stays; every form without form flag 0x8 (`004013e0`) runs vtable
/// slot 0x88; every entry goes to `00843c90`. Finally the TLS flag is
/// cleared.
pub fn fn_008439e0(e: &mut Engine, this: Ptr) {
    let arrays = this.byte_add(0x10);
    let tls_owner = Ptr::new(e.global::<u32>(0x011d_df38));
    fn_00843c10(e, tls_owner, true);
    let reserve = e.mem.u32(this.addr() + 0x30);
    fn_00843be0(e, arrays, reserve);
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    let saved = e.call(0x0045_16b0, &args![setting_owner]).u8();
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    e.call(0x0055_0890, &args![setting_owner, 1u32]);
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let key = e.mem.u32(key_at.addr());
            let file = e.call(0x0047_39b0, &args![key]).u32();
            let value = e.mem.u32(value_at.addr());
            e.call(0x0084_27f0, &args![value, file, arrays]);
        }
    });
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    e.call(0x0055_0890, &args![setting_owner, u32::from(saved)]);
    let mut index = 0u32;
    while index < fn_00843bb0(e, arrays) {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        let form = e.mem.u32(entry);
        if e.vcall(form, 0xf0, &args![]).bool() {
            let form = e.mem.u32(entry);
            if e.call(0x007a_f430, &args![form]).u32() == 0 {
                e.call(0x0084_3380, &args![arrays, index]);
                index = index.wrapping_sub(1);
            }
        }
        index = index.wrapping_add(1);
    }
    let count = fn_00843bb0(e, arrays);
    for index in 0..count {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        let form = e.mem.u32(entry);
        if !e.call(0x0040_13e0, &args![form]).bool() {
            let form = e.mem.u32(entry);
            e.vcall(form, 0x88, &args![]);
        }
    }
    for index in 0..count {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        e.call(0x0084_3c90, &args![this, entry]);
    }
    let tls_owner = Ptr::new(e.global::<u32>(0x011d_df38));
    fn_00843c10(e, tls_owner, false);
}

// Translated from 00843bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entries of both arrays: `0044ddc0` on `+0` plus on `+0x10`.
pub fn fn_00843bb0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.call(ARRAY_COUNT, &args![this]).u32();
    let second = e.call(ARRAY_COUNT, &args![this.addr() + 0x10]).u32();
    second.wrapping_add(first)
}

// Translated from 00843be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the allocated size of both arrays (`00844c20` on `+0` and
/// `+0x10`); `RET 4`.
pub fn fn_00843be0(e: &mut Engine, this: Ptr, size: u32) {
    fn_00844c20(e, this, size);
    fn_00844c20(e, this.byte_add(0x10), size);
}

// Translated from 00843c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears bit 4 of the word at `+0x294` of the
/// calling thread's TLS block; `this` is not read; `RET 4`.
pub fn fn_00843c10(e: &mut Engine, _this: Ptr, set: bool) {
    let tls = e.tls();
    let flags = e.mem.u32(tls + TLS_FLAGS);
    let flags = if set { flags | 4 } else { flags & 0xffff_fffb };
    e.mem.set_u32(tls + TLS_FLAGS, flags);
}

// Translated from 00844ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray` (8-byte elements: buffer `+4`, size `+8`, allocated
/// size `+0xC`) `SetSize`, `RET 8`: size 0 releases the array
/// (`008454f0` with `free`). Growing past the allocated size allocates
/// (vtable slot 4 with the size, when nothing is allocated) or reallocates
/// (`006a2770` with the size and the old size), then constructs the new
/// elements (`00845300`); growing within it only constructs them;
/// shrinking destroys the dropped elements (`0072ba80`) and, with `free`
/// and the new size at most a quarter of the allocated one, reallocates to
/// the new size.
pub fn fn_00844ae0(e: &mut Engine, this: Ptr, size: u32, free: u8) {
    let t = this.addr();
    if size == 0 {
        e.call(0x0084_54f0, &args![this, u32::from(free)]);
        return;
    }
    let element = |e: &Engine, index: u32| e.mem.u32(t + 4).wrapping_add(index.wrapping_mul(8));
    if size > e.mem.u32(t + 0xc) {
        if e.mem.u32(t + 0xc) == 0 {
            let buffer = e.vcall(t, 4, &args![size]).u32();
            e.mem.set_u32(t + 4, buffer);
            e.mem.set_u32(t + 0xc, size);
        } else {
            let old = e.mem.u32(t + 8);
            e.call(ARRAY_REALLOCATE, &args![this, size, old]);
            e.mem.set_u32(t + 0xc, size);
        }
        let old = e.mem.u32(t + 8);
        let first = element(e, old);
        e.call(ARRAY_CONSTRUCT, &args![this, first, size.wrapping_sub(old)]);
        e.mem.set_u32(t + 8, size);
    } else if size < e.mem.u32(t + 8) {
        let old = e.mem.u32(t + 8);
        let first = element(e, size);
        e.call(ARRAY_DESTROY, &args![this, first, old.wrapping_sub(size)]);
        e.mem.set_u32(t + 8, size);
        if free != 0 && size <= e.mem.u32(t + 0xc) >> 2 {
            e.call(ARRAY_REALLOCATE, &args![this, size, size]);
            e.mem.set_u32(t + 0xc, size);
        }
    } else {
        let old = e.mem.u32(t + 8);
        let first = element(e, old);
        e.call(ARRAY_CONSTRUCT, &args![this, first, size.wrapping_sub(old)]);
        e.mem.set_u32(t + 8, size);
    }
}

// Translated from 00844c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same array's allocated-size setter, `RET 4`: nothing when it is
/// already `size`; elements past `size` are destroyed (`0072ba80`) and the
/// size cut; then the buffer is reallocated (`006a2770` with `size` and
/// the size) and the allocated size set.
pub fn fn_00844c20(e: &mut Engine, this: Ptr, size: u32) {
    let t = this.addr();
    if size == e.mem.u32(t + 0xc) {
        return;
    }
    if size < e.mem.u32(t + 8) {
        let old = e.mem.u32(t + 8);
        let first = e.mem.u32(t + 4).wrapping_add(size.wrapping_mul(8));
        e.call(ARRAY_DESTROY, &args![this, first, old.wrapping_sub(size)]);
        e.mem.set_u32(t + 8, size);
    }
    let count = e.mem.u32(t + 8);
    e.call(ARRAY_REALLOCATE, &args![this, size, count]);
    e.mem.set_u32(t + 0xc, size);
}

// Translated from 00844e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00844e20 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<TESFile *, BGSReconstructFormsInFileMap *>` destructor
/// (vtable `0107ec48`): empties the map (`00438af0`), then runs the base
/// destructor `00844e80`.
pub fn fn_00844e20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FILE_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(0x0084_4e80, &args![this]);
}

/// `TESForm::GetFile` (engine map, `00484e60`; `thiscall`, one word: the index, -1 for the
/// last file).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `00440e30` (`thiscall` on a form, no stack words): the text `AddForm`'s warning prints
/// as the form's type.
const TYPE_NAME_00440E30: u32 = 0x0044_0e30;
/// `0084e3a0` (`thiscall`: the word at +0xC, a form's id).
const FORM_ID: u32 = 0x0084_e3a0;
/// The forms' vtable slot `AddForm` and `AddReference` print as the name (+0x130).
const FORM_NAME_SLOT: u32 = 0x130;
/// `SaveGameWarning` (engine map, `00845cc0`; variadic `cdecl`: format and values).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"Form %08X %s with type %s and no file passed to
/// BGSReconstructFormsInAllFilesMap::AddForm()"` and `"Cell %08X %s with no file passed to
/// BGSReconstructFormsInAllFilesMap::AddReference()"`.
const NO_FILE_FORM_MESSAGE: u32 = 0x0107_ea90;
const NO_FILE_CELL_MESSAGE: u32 = 0x0107_eaf0;
/// The file map's look-up (`00853130`, `thiscall`, two words: key and the value's
/// address; answers whether found) and `SetAt` (`00844700`, `thiscall`: key, value).
const FILE_MAP_GET_AT: u32 = 0x0085_3130;
const FILE_MAP_SET_AT: u32 = 0x0084_4700;
/// `operator new` (`00401000`, `cdecl`, one word) and
/// `BGSReconstructFormsInFileMap::BGSReconstructFormsInFileMap` (engine map, `00842180`;
/// `thiscall`, no stack words) on a 0x1C-byte block.
const OPERATOR_NEW: u32 = 0x0040_1000;
const FILE_MAP_CONSTRUCT: u32 = 0x0084_2180;
const FILE_MAP_SIZE: u32 = 0x1c;
/// `BGSReconstructFormsInFileMap::AddForm` (engine map, `00842310`; `thiscall`, two words)
/// and `AddCellForm` (`00842600`; `thiscall`, four words).
const FILE_MAP_ADD_FORM: u32 = 0x0084_2310;
const FILE_MAP_ADD_CELL_FORM: u32 = 0x0084_2600;
/// The count of added forms (`+0x30`).
const ADDED_COUNT: u32 = 0x30;
/// `00425fd0` (`thiscall` on a form: a byte), `00460140` (`thiscall`, no stack words),
/// `TESObjectCELL::GetWorldSpace` (engine map, `0054ddd0`; `thiscall`, no stack words) and
/// `BGSReconstructFormsInAllFilesMap::GetFileForFormID` (engine map, `00843fe0`;
/// `thiscall`, two words).
const FORM_TEST_00425FD0: u32 = 0x0042_5fd0;
const FORM_KEY_00460140: u32 = 0x0046_0140;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const GET_FILE_FOR_FORM_ID: u32 = 0x0084_3fe0;

/// The file's `BGSReconstructFormsInFileMap`: looked up in the map (`00853130`), or a new
/// one (`00842180` on a 0x1C-byte block, null when the allocation fails) set at the file
/// (`00844700`).
fn file_map_for(e: &mut Engine, this: Ptr, file: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if !e.call(FILE_MAP_GET_AT, &args![this, file, slot]).bool() {
            let block = e.call(OPERATOR_NEW, &args![FILE_MAP_SIZE]).u32();
            let map = if block != 0 {
                e.call(FILE_MAP_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), map);
            e.call(FILE_MAP_SET_AT, &args![this, file, map]);
        }
        e.mem.u32(slot.addr())
    })
}

/// `+0x30` counts up.
fn count_added(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + ADDED_COUNT);
    e.mem
        .set_u32(this.addr() + ADDED_COUNT, count.wrapping_add(1));
}

// Translated from 00843630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843630 - the compiler's exception frame (FS:[0] chain, handler 00f0b23b, state 0 that frees the 0x1c-byte block when its constructor throws) is not translated
/// `BGSReconstructFormsInAllFilesMap::AddForm` (engine map; `thiscall`, two words: the form
/// and a word handed on; `RET 8`). Nothing for a null form. A form without a file
/// (`TESForm::GetFile(-1)`) gets the warning `"Form %08X %s with type %s and no file ..."`
/// (its id, its name from vtable +0x130 and `00440e30`'s text). Otherwise the form goes to
/// its file's map (found, or made and set) with `BGSReconstructFormsInFileMap::AddForm`,
/// and `+0x30` counts up.
pub fn bgs_reconstruct_forms_in_all_files_map_add_form(
    e: &mut Engine,
    this: Ptr,
    form: u32,
    data: u32,
) {
    if form == 0 {
        return;
    }
    let file = e.call(FORM_GET_FILE, &args![form, u32::MAX]).u32();
    if file == 0 {
        let type_text = e.call(TYPE_NAME_00440E30, &args![form]).u32();
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![NO_FILE_FORM_MESSAGE, id, name, type_text],
        );
        return;
    }
    let map = file_map_for(e, this, file);
    e.call(FILE_MAP_ADD_FORM, &args![map, form, data]);
    count_added(e, this);
}

// Translated from 00843750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843750 - the compiler's exception frame (FS:[0] chain, handler 00f0b23b, state 0 that frees the 0x1c-byte block when its constructor throws) is not translated
/// `BGSReconstructFormsInAllFilesMap::AddReference` (engine map; `thiscall`, three words: a
/// form, then two words handed on; `RET 0xc`). Nothing for a null form. The file is
/// `GetFileForFormID(00460140(form), first)` when the form's `00425fd0` answers true, else
/// `GetFileForFormID(00460140(GetWorldSpace(form)), first)`. Without a file the warning
/// `"Cell %08X %s with no file ..."` (its id and its name from vtable +0x130); otherwise the
/// form goes to the file's map (found, or made and set) with
/// `BGSReconstructFormsInFileMap::AddCellForm(form, 0, first, second)`, and `+0x30` counts
/// up.
pub fn bgs_reconstruct_forms_in_all_files_map_add_reference(
    e: &mut Engine,
    this: Ptr,
    form: u32,
    first: u32,
    second: u32,
) {
    if form == 0 {
        return;
    }
    let file = if e.call(FORM_TEST_00425FD0, &args![form]).bool() {
        let key = e.call(FORM_KEY_00460140, &args![form]).u32();
        e.call(GET_FILE_FOR_FORM_ID, &args![this, key, first]).u32()
    } else {
        let world = e.call(CELL_GET_WORLD_SPACE, &args![form]).u32();
        let key = e.call(FORM_KEY_00460140, &args![world]).u32();
        e.call(GET_FILE_FOR_FORM_ID, &args![this, key, first]).u32()
    };
    if file == 0 {
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(SAVE_GAME_WARNING, &args![NO_FILE_CELL_MESSAGE, id, name]);
        return;
    }
    let map = file_map_for(e, this, file);
    e.call(
        FILE_MAP_ADD_CELL_FORM,
        &args![map, form, 0u32, first, second],
    );
    count_added(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0084_3630,
            bgs_reconstruct_forms_in_all_files_map_add_form(Ptr, u32, u32)
        ),
        entry!(
            0x0084_3750,
            bgs_reconstruct_forms_in_all_files_map_add_reference(Ptr, u32, u32, u32)
        ),
        entry!(0x0084_3150, fn_00843150(Ptr) -> u8),
        entry!(0x0084_3170, fn_00843170(Ptr)),
        entry!(0x0084_3240, fn_00843240(Ptr)),
        entry!(
            0x0084_3440,
            bgs_reconstruct_forms_in_all_files_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0084_34d0,
            bgs_reconstruct_forms_in_all_files_map_destructor(Ptr)
        ),
        entry!(0x0084_3540, fn_00843540(Ptr)),
        entry!(0x0084_35a0, fn_008435a0(Ptr)),
        entry!(0x0084_39e0, fn_008439e0(Ptr)),
        entry!(0x0084_3bb0, fn_00843bb0(Ptr) -> u32),
        entry!(0x0084_3be0, fn_00843be0(Ptr, u32)),
        entry!(0x0084_3c10, fn_00843c10(Ptr, bool)),
        entry!(0x0084_4ae0, fn_00844ae0(Ptr, u32, u8)),
        entry!(0x0084_4c20, fn_00844c20(Ptr, u32)),
        entry!(0x0084_4e20, fn_00844e20(Ptr)),
        entry!(0x0084_3220, fn_00843220(Ptr)),
        entry!(0x00844db0, fn_00844db0(Ptr, u32) -> Ptr),
        entry!(0x00844130, fn_00844130(Ptr, u32) -> Ptr),
        entry!(0x00844a90, fn_00844a90(Ptr) -> Ptr),
        entry!(0x00843470, fn_00843470(Ptr) -> Ptr),
        entry!(0x008433d0, bgsreconstructformsinallfilesmap_constructor(Ptr) -> Ptr),
        entry!(0x0084_4ac0, fn_00844ac0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MAP_INITIALIZER, |_, _| Ret::default());
    }

    #[test]
    fn fn_00844db0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 77);
        let r = e.call(0x00844db0, &args![this, 5u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed1c);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.block_size(table), Some(24), "20 bytes, rounded up");
        assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
    }

    #[test]
    fn fn_00844130_replaces_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let r = e.call(0x00844130, &args![this, 3u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ec48);
        assert_eq!(e.mem.u32(this + 4), 3);
    }

    #[test]
    fn fn_00844a90_stores_the_vtable_and_initializes() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00844a90, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
    }

    #[test]
    fn fn_00843470_initializes_both_members() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        let r = e.call(0x00843470, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
        assert_eq!(log[2], (MAP_INITIALIZER, vec![this + 0x10, 0, 0]));
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
    }

    #[test]
    fn the_constructor_builds_the_base_member_and_clears_the_tail() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 9);
        let r = e.call(0x008433d0, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ea74, "the final vtable");
        assert_eq!(e.mem.u32(this + 4), 0x25, "base bucket count");
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
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
    fn fwd_a03_00843220_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0084_4ff0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_3220, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0084_4ff0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00844ac0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0084_4ac0, ARRAY_VTABLE_0107ED08, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b023_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn small_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + 0x14, 0x5a);
        assert_eq!(e.call(0x0084_3150, &[this]).u8(), 0x5a);
        e.mem.set_u32(this, 7);
        e.mem.set_u32(this + 4, 8);
        e.mem.set_u32(this + 8, 9);
        e.call(0x0084_3170, &[this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 9);
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0x18, 4);
        assert_eq!(e.call(0x0084_3bb0, &[this]).u32(), 7);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_FLAGS, 0x10);
        e.call(0x0084_3c10, &[0, 0x100]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x10);
        e.call(0x0084_3c10, &[0, 1]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x14);
        e.call(0x0084_3c10, &[0, 0]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x10);
    }

    /// An array (buffer `+4`, size `+8`, allocated `+0xC`) whose vtable slot
    /// 4 is `0x7000_0004`.
    fn array(e: &mut Engine, buffer: u32, size: u32, allocated: u32) -> u32 {
        e.put_vtable(0x0600_0000, &[0, 0x7000_0004]);
        let a = e.mem.alloc(0x10);
        e.mem.set_u32(a, 0x0600_0000);
        e.mem.set_u32(a + 4, buffer);
        e.mem.set_u32(a + 8, size);
        e.mem.set_u32(a + 0xc, allocated);
        a
    }

    #[test]
    fn array_set_size() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x0084_54f0,
                ARRAY_REALLOCATE,
                ARRAY_CONSTRUCT,
                ARRAY_DESTROY,
            ],
        );
        e.register(0x7000_0004, |_, _| ret(0x0800_0000));
        // Size 0: released with the byte as given.
        let a = array(&mut e, 0x1000, 4, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 0, 2]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0084_54f0), vec![vec![a, 2]]);
        // Growing an empty array: allocated through slot 4.
        let a = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 3, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![a, 3]]);
        assert_eq!(
            calls_to(&log, ARRAY_CONSTRUCT),
            vec![vec![a, 0x0800_0000, 3]]
        );
        assert_eq!(
            e.mem.bytes(a + 4, 12),
            [0x0800_0000u32, 3, 3]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        // Growing past the allocation: reallocated, then constructed.
        let a = array(&mut e, 0x1000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 6, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 6, 2]]);
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT), vec![vec![a, 0x1010, 4]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (6, 6));
        // Growing within it: constructed only.
        let a = array(&mut e, 0x1000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 5, 1]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT), vec![vec![a, 0x1010, 3]]);
        assert_eq!(e.mem.u32(a + 8), 5);
        // Shrinking: destroyed; with the byte set and a quarter or less of
        // the allocation, reallocated to the size.
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 2, 1]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_DESTROY), vec![vec![a, 0x1010, 4]]);
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 2, 2]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (2, 2));
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 3, 1]);
        assert!(calls_to(&e.call_log.take().unwrap(), ARRAY_REALLOCATE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 1, 0x100]);
        assert!(calls_to(&e.call_log.take().unwrap(), ARRAY_REALLOCATE).is_empty());
    }

    #[test]
    fn array_allocated_size() {
        let mut e = Engine::new();
        quiet(&mut e, &[ARRAY_REALLOCATE, ARRAY_DESTROY]);
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 8]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_DESTROY), vec![vec![a, 0x1020, 2]]);
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 4, 4]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (4, 4));
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_DESTROY).is_empty());
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 16, 4]]);
        // Both arrays of the pair.
        let pair = e.mem.alloc(0x20);
        e.mem.set_u32(pair + 0xc, 1);
        e.mem.set_u32(pair + 0x1c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_3be0, &[pair, 5]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![pair, 5, 0], vec![pair + 0x10, 5, 0]]
        );
        // 00843240 empties both, freeing.
        quiet(&mut e, &[0x0084_54f0]);
        e.call_log = Some(vec![]);
        e.call(0x0084_3240, &[pair]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0084_54f0),
            vec![vec![pair, 1], vec![pair + 0x10, 1]]
        );
    }

    /// A map of `entries` (key, value) walked through positions 1..=n.
    fn map_doubles(e: &mut Engine, entries: Vec<(u32, u32)>) {
        let n = entries.len() as u32;
        e.register_double(MAP_FIRST_POSITION, move |_, _| ret(u32::from(n > 0)));
        let entries = Rc::new(entries);
        e.register_double(MAP_NEXT, move |e, a| {
            let position = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(a[2]), 0);
            assert_eq!(e.mem.u32(a[3]), 0);
            let (key, value) = entries[position as usize - 1];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let next = if position < entries.len() as u32 {
                position + 1
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
    }

    #[test]
    fn deleting_the_file_maps_and_the_destructor() {
        let mut e = Engine::new();
        e.put_vtable(0x0610_0000, &[0x7000_0000]);
        quiet(
            &mut e,
            &[
                0x7000_0000,
                MAP_REMOVE_ALL,
                0x0084_54f0,
                0x0084_4e80,
                0x0040_1030,
            ],
        );
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first, 0x0610_0000);
        e.mem.set_u32(second, 0x0610_0000);
        map_doubles(&mut e, vec![(1, first), (2, 0), (3, second)]);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 9);
        e.call_log = Some(vec![]);
        e.call(0x0084_35a0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x7000_0000),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(
            calls_to(&log, 0x0084_54f0),
            vec![vec![this + 0x10, 1], vec![this + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x30), 0);

        // The destructor: vtables stored, the arrays' destructors, then the
        // map's.
        map_doubles(&mut e, vec![]);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0084_3440, &[this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0040_1030).len(), frees);
            assert_eq!(e.mem.u32(this), FILE_MAP_VTABLE);
            assert_eq!(e.mem.u32(this + 0x10), ARRAY_VTABLE_0107ED08);
            assert_eq!(e.mem.u32(this + 0x20), ARRAY_VTABLE_0107ED08);
            // 00843540 releases +0x20 then +0x10, after 008435a0's two.
            assert_eq!(
                calls_to(&log, 0x0084_54f0),
                vec![
                    vec![this + 0x10, 1],
                    vec![this + 0x20, 1],
                    vec![this + 0x20, 1],
                    vec![this + 0x10, 1],
                ]
            );
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![this]; 2]);
            assert_eq!(calls_to(&log, 0x0084_4e80), vec![vec![this]]);
        }
    }

    #[test]
    fn reconstruct_every_file() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.set_global(0x011c_3f2c, 0x0aaa_0000u32);
        e.set_global(0x011d_df38, 0x0bbb_0000u32);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 0x40);
        // Two arrays of form entries: forms are [vtable, kind]; kind 1
        // answers true to slot 0xF0, kind 2 also has flag 0x8.
        e.put_vtable(0x0620_0000, &{
            let mut s = vec![0u32; 0xf4 / 4];
            s[0x88 / 4] = 0x7000_0088;
            s[0xf0 / 4] = 0x7000_00f0;
            s
        });
        e.register(0x7000_00f0, |e, a| ret(u32::from(e.mem.u32(a[0] + 4) == 1)));
        e.register(0x7000_0088, |_, _| Ret::default());
        e.register(0x0040_13e0, |e, a| ret(u32::from(e.mem.u32(a[0] + 4) == 2)));
        let form = |e: &mut Engine, kind: u32| {
            let f = e.mem.alloc(8);
            e.mem.set_u32(f, 0x0620_0000);
            e.mem.set_u32(f + 4, kind);
            f
        };
        let forms = vec![
            form(&mut e, 0),
            form(&mut e, 1),
            form(&mut e, 2),
            form(&mut e, 1),
        ];
        // Entries are the address of a slot holding the form; the count is
        // the list's length (all in the first array).
        let list = Rc::new(RefCell::new(forms.clone()));
        let slots = e.mem.alloc(0x40);
        let l = list.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            ret(if a[0] == this + 0x10 {
                l.borrow().len() as u32
            } else {
                0
            })
        });
        let l = list.clone();
        e.register_double(0x0084_32e0, move |e, a| {
            let f = l.borrow()[a[1] as usize];
            e.mem.set_u32(slots + 4 * a[1], f);
            ret(slots + 4 * a[1])
        });
        let l = list.clone();
        e.register_double(0x0084_3380, move |_, a| {
            l.borrow_mut().remove(a[1] as usize);
            Ret::default()
        });
        // Base forms: only the second kind-1 form has one.
        let keep = forms[3];
        e.register_double(0x007a_f430, move |_, a| ret(u32::from(a[0] == keep)));
        quiet(
            &mut e,
            &[
                ARRAY_REALLOCATE,
                ARRAY_DESTROY,
                0x0055_0890,
                0x0084_27f0,
                0x0084_3c90,
            ],
        );
        e.register(0x0045_16b0, |_, _| ret(0x1203));
        e.register(0x0047_39b0, |_, a| ret(a[0] + 0x100));
        map_doubles(&mut e, vec![(0x10, 0x20), (0x11, 0x21)]);
        e.call_log = Some(vec![]);
        e.call(0x0084_39e0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![this + 0x10, 0x40, 0], vec![this + 0x20, 0x40, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0055_0890),
            vec![vec![0x0aaa_0000, 1], vec![0x0aaa_0000, 3]]
        );
        assert_eq!(
            calls_to(&log, 0x0084_27f0),
            vec![
                vec![0x20, 0x110, this + 0x10],
                vec![0x21, 0x111, this + 0x10]
            ]
        );
        // The first kind-1 form (no base form) was removed.
        assert_eq!(*list.borrow(), vec![forms[0], forms[2], forms[3]]);
        assert_eq!(calls_to(&log, 0x0084_3380), vec![vec![this + 0x10, 1]]);
        // Slot 0x88 for the forms without flag 0x8.
        let initialized: Vec<u32> = calls_to(&log, 0x7000_0088).iter().map(|w| w[0]).collect();
        assert_eq!(initialized, vec![forms[0], forms[3]]);
        let finished: Vec<u32> = calls_to(&log, 0x0084_3c90)
            .iter()
            .map(|w| {
                assert_eq!(w[0], this);
                e.mem.u32(w[1])
            })
            .collect();
        assert_eq!(finished, vec![forms[0], forms[2], forms[3]]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_FLAGS) & 4, 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// The forms' name slot (vtable +0x130) as the tests place it.
    const NAME_SLOT: u32 = 0x0090_0130;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// `known` is the file map's answer for file 0x7000 (0 when the map has none).
    fn rig(file: u32, known: u32) -> (Engine, Ptr, u32) {
        let mut e = Engine::new();
        e.register_double(FORM_GET_FILE, move |_, _| file.into_ret());
        e.register_double(GET_FILE_FOR_FORM_ID, move |_, _| file.into_ret());
        e.register(TYPE_NAME_00440E30, |_, _| 0x0300_0000u32.into_ret());
        e.register(NAME_SLOT, |_, _| 0x0300_1000u32.into_ret());
        e.register(FORM_ID, |_, _| 0x0001_2345u32.into_ret());
        e.register(FORM_KEY_00460140, |_, a| (a[0] ^ 0xff).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |_, _| 0x0c00_0000u32.into_ret());
        e.register_double(FILE_MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], known);
            u32::from(known != 0).into_ret()
        });
        e.register(OPERATOR_NEW, |_, _| 0x0d00_0000u32.into_ret());
        e.register(FILE_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        for callee in [
            SAVE_GAME_WARNING,
            FILE_MAP_SET_AT,
            FILE_MAP_ADD_FORM,
            FILE_MAP_ADD_CELL_FORM,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_1000, &[0; 0x4d]);
        e.mem.set_u32(0x0090_1000 + 0x130, NAME_SLOT);
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, 0x0090_1000);
        let this = Ptr::new(e.mem.alloc(0x34));
        (e, this, form)
    }

    #[test]
    fn add_form_adds_to_a_new_file_map() {
        let (mut e, this, form) = rig(0x7000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_GET_FILE), vec![vec![form, u32::MAX]]);
        assert_eq!(
            calls(&log, FILE_MAP_SET_AT),
            vec![vec![this.addr(), 0x7000, 0x0d00_0000]]
        );
        assert_eq!(
            calls(&log, FILE_MAP_ADD_FORM),
            vec![vec![0x0d00_0000, form, 9]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 1);
    }

    #[test]
    fn add_form_uses_the_known_map_or_warns_without_a_file() {
        let (mut e, this, form) = rig(0x7000, 0x0e00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls(&log, FILE_MAP_ADD_FORM),
            vec![vec![0x0e00_0000, form, 9]]
        );
        let (mut e, this, form) = rig(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_GAME_WARNING),
            vec![vec![
                NO_FILE_FORM_MESSAGE,
                0x0001_2345,
                0x0300_1000,
                0x0300_0000
            ]]
        );
        assert!(calls(&log, FILE_MAP_ADD_FORM).is_empty());
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 0);
        // A null form: nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, 0u32, 9u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn add_reference_finds_the_file_by_the_form_or_its_world() {
        let (mut e, this, form) = rig(0x7000, 0x0e00_0000);
        e.register(FORM_TEST_00425FD0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GET_FILE_FOR_FORM_ID),
            vec![vec![this.addr(), form ^ 0xff, 5]]
        );
        assert_eq!(
            calls(&log, FILE_MAP_ADD_CELL_FORM),
            vec![vec![0x0e00_0000, form, 0, 5, 6]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 1);
        // Through the world space when the test answers false.
        e.register(FORM_TEST_00425FD0, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, CELL_GET_WORLD_SPACE), vec![vec![form]]);
        assert_eq!(
            calls(&log, GET_FILE_FOR_FORM_ID),
            vec![vec![this.addr(), 0x0c00_00ff, 5]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 2);
    }

    #[test]
    fn add_reference_warns_without_a_file() {
        let (mut e, this, form) = rig(0, 0);
        e.register(FORM_TEST_00425FD0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_GAME_WARNING),
            vec![vec![NO_FILE_CELL_MESSAGE, 0x0001_2345, 0x0300_1000]]
        );
        assert!(calls(&log, FILE_MAP_ADD_CELL_FORM).is_empty());
    }
}
