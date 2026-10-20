//! `fallout/misc/saveload/bgsloadgamesubbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00865570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded append wrapper: reads position at +0xc and forwards the low
/// byte of the last stack argument to the positioned save-buffer writer.
pub fn append_bytes(e: &mut Engine, this: Ptr, source: Ptr, size: u32, no_separator: u8) {
    let position = e.mem.u32(this.addr().wrapping_add(0xc));
    e.call(
        0x00865ce0,
        &args![this, source, size, position, no_separator],
    );
}

/// `00aa1070`: the allocator (cdecl, byte count).
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;
/// `00438af0(this)`: the base teardown (`fallout shared/run_00438af0`).
const BASE_TEARDOWN: u32 = 0x0043_8af0;
/// `00aa10f0(pointer)`: releases a block (cdecl, one argument).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `__ehvec_ctor` (`00ec782f`, CRT): constructs an array of elements.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;

// Translated from 008656c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the table (+8) and
/// zeroes it. Returns `this`.
pub fn fn_008656c0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0108_22a8);
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

// Translated from 00865660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008656c0(this, word)`, then the vtable
/// `01082288`. Returns `this`.
pub fn fn_00865660(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    fn_008656c0(e, this, word);
    e.mem.set_u32(this.addr(), 0x0108_2288);
    this
}

// Translated from 00865200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00865660(this, 0x25)`; `EAX` is whatever that
/// returns (it returns `this`).
pub fn fn_00865200(e: &mut Engine, this: Ptr) -> u32 {
    fn_00865660(e, this, 0x25).addr()
}

// Translated from 00865840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, runs
/// `00438af0(this)` and releases the block at +8 (`00aa10f0`). `EAX` is the
/// release call's result, returned here.
pub fn fn_00865840(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_22a8);
    e.call(BASE_TEARDOWN, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![block]).u32()
}

// Translated from 008657e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `01082288`, runs
/// `00438af0(this)` and then `00865840`; `EAX` is the latter's result.
// Translation incomplete: 008657e0 - SEH exception unwinding is unsupported.
pub fn fn_008657e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_2288);
    e.call(BASE_TEARDOWN, &args![this]);
    fn_00865840(e, this)
}

// Translated from 008651d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs the array of three 0x10-byte
/// elements at `this` with `__ehvec_ctor` (`00ec782f`: element constructor
/// `00865200`, element destructor `008657e0`). Returns `this`.
pub fn fn_008651d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this, 0x10u32, 3u32, 0x0086_5200u32, 0x0086_57e0u32],
    );
    this
}

/// The function `fn_00865870` calls on the member (`00865900`).
const CALLEE_00865900: u32 = 0x0086_5900;

// Translated from 00865870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<TESForm*,BGSLoadGameSubBuffer>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00865900` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00865870(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00865900, &args![this.byte_add(0xc)]).ptr()
}

/// `004b9ba0` (thiscall): the map's first occupied bucket entry, or 0.
const MAP_FIRST_ITEM: u32 = 0x004b_9ba0;
/// `0044dee0` (thiscall): clears the sub-buffer's data pointer (its
/// constructor).
const SUB_BUFFER_CONSTRUCT: u32 = 0x0044_dee0;
/// `00865730` (thiscall, 3 words): takes the map entry at `*position`,
/// writes its form and sub-buffer to the out pointers and moves `*position`
/// to the next entry.
const MAP_GET_NEXT: u32 = 0x0086_5730;
/// `00435b20` (thiscall): releases the sub-buffer's data and clears it.
const SUB_BUFFER_FREE: u32 = 0x0043_5b20;
/// `0043fcd0` (thiscall): non-zero when the reference's 3D is loaded.
const REFERENCE_LOADED_3D: u32 = 0x0043_fcd0;
/// `00864b00` `BGSLoadGameSubBuffer::LoadAnimation` (Xbox PDB name in the
/// engine map; thiscall, form).
const LOAD_ANIMATION: u32 = 0x0086_4b00;
/// `00864c50` `BGSLoadGameSubBuffer::LoadCharController` (Xbox PDB name in
/// the engine map; thiscall, form).
const LOAD_CHAR_CONTROLLER: u32 = 0x0086_4c50;
/// `00864d80` `BGSLoadGameSubBuffer::LoadHavokData` (Xbox PDB name in the
/// engine map; thiscall, form).
const LOAD_HAVOK_DATA: u32 = 0x0086_4d80;
/// `00559450` (thiscall): the word at +0 (the sub-buffer's data).
const SUB_BUFFER_DATA: u32 = 0x0055_9450;
/// `005d43c0` (thiscall): `this + 0x44` (the reference's extra data list).
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `00422940` `ExtraDataList::SetSavedAnimation` (Xbox PDB name in the
/// engine map; thiscall, data).
const SET_SAVED_ANIMATION: u32 = 0x0042_2940;
/// `00422ac0` `ExtraDataList::SetSavedHavokData` (Xbox PDB name in the
/// engine map; thiscall, data).
const SET_SAVED_HAVOK_DATA: u32 = 0x0042_2ac0;
/// `0084e3a0` (thiscall): the word at +0xc (a form's id, a save-form
/// buffer's size).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `00845f70` (cdecl): bytes the variable-sized encoding of a value takes.
const VARIABLE_SIZE_LENGTH: u32 = 0x0084_5f70;
/// `00401000` (cdecl, byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `008a81c0` `BGSLoadFormBuffer::GetVersion` (Xbox PDB name in the engine
/// map; thiscall, byte result).
const BUFFER_GET_VERSION: u32 = 0x008a_81c0;
/// `00845fc0` `SaveVariableSizedValueToBuffer` (Xbox PDB name in the engine
/// map; cdecl: destination, value).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0084_5fc0;
/// `00726070` (thiscall): the word at +4 (the buffer's data).
const WORD_AT_4: u32 = 0x0072_6070;
/// `00401460` (cdecl: destination, source, byte count): copies bytes.
const COPY_BYTES: u32 = 0x0040_1460;
/// `00846080` `LoadVariableSizedValueFromBuffer` (Xbox PDB name in the
/// engine map; cdecl: source, out value): the bytes it read.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0084_6080;
/// `00865f60` `BGSSaveGameBuffer::SaveVariableSizedValue` (Xbox PDB name in
/// the engine map; thiscall, value).
const SAVE_GAME_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `008659c0` `BGSSaveFormBuffer::BGSSaveFormBuffer` (Xbox PDB name in the
/// engine map; thiscall).
const SAVE_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_59c0;
/// The save-load manager singleton pointer.
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// `00851110` (thiscall): returns 0x1b.
const MANAGER_VERSION: u32 = 0x0085_1110;
/// `00401170` (thiscall): the byte at +4 (a form's type).
const BYTE_AT_4: u32 = 0x0040_1170;
/// `008c71b0` (thiscall, word): stores the word at +0 and returns `this`
/// (the body is a constructor; the engine map's name
/// `BGSChangeFlags::IsForcedChange` does not fit it).
const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// `00865ad0` (thiscall, 4 words): sets up the save-form buffer.
const SAVE_FORM_BUFFER_INIT: u32 = 0x0086_5ad0;
/// `0050f9c0` (thiscall, form).
const SAVE_FORM_BUFFER_SET_FORM: u32 = 0x0050_f9c0;
/// `00563650` `TESObjectREFR::SaveAnimation` (Xbox PDB name in the engine
/// map; thiscall, buffer).
const REFERENCE_SAVE_ANIMATION: u32 = 0x0056_3650;
/// `00562de0` `TESObjectREFR::SaveHavokData` (Xbox PDB name in the engine
/// map; thiscall, buffer).
const REFERENCE_SAVE_HAVOK_DATA: u32 = 0x0056_2de0;
/// `00847dd0` (thiscall): the save-form buffer's teardown (the engine map
/// names the folded body `BGSSaveLoadManager::GetMinorVersion`).
const SAVE_FORM_BUFFER_DESTRUCT: u32 = 0x0084_7dd0;
/// `00845cc0` `SaveGameWarning` (Xbox PDB name in the engine map; cdecl,
/// format and its arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"SAVELOAD: Form '%s' %08X passed to
/// BGSLoadGameSubBuffer::SaveAnimation() is not a reference"` (both savers
/// pass it).
const NOT_A_REFERENCE_FORMAT: u32 = 0x0108_2228;
/// The eh vector destructor iterator (`00ec5fce`, stdcall: array, element
/// size, count, element destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

// Translated from 00865390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadQueuedSubBufferMap::LoadQueuedSubBuffers` (Xbox PDB name in
/// the engine map): walks the map of kind `kind` (`this + 0x10 * kind`;
/// 0 animation, 1 character controller, 2 Havok data). For each queued
/// form whose 3D is loaded (`0043fcd0`), or that is not a reference
/// (vtable +0xF0), the kind's loader loads the sub-buffer; otherwise kind 0
/// frees it when the reference's vtable +0x100 says so or else hands it to
/// the reference's extra data as its saved animation, kind 1 frees it, and
/// kind 2 hands it over as saved Havok data. Ends with the map's teardown
/// (`00438af0`).
pub fn load_queued_sub_buffers(e: &mut Engine, this: Ptr, kind: i32) {
    let map = this.addr().wrapping_add((kind as u32) << 4);
    // Locals: position at +0, form at +4, the sub-buffer at +8.
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let form_slot = position + 4;
        let sub_buffer = position + 8;
        let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(form_slot, 0);
            e.call(SUB_BUFFER_CONSTRUCT, &args![sub_buffer]);
            e.call(MAP_GET_NEXT, &args![map, position, form_slot, sub_buffer]);
            let mut load = true;
            let mut reference = 0u32;
            let mut frees = 0u32;
            let form = e.mem.u32(form_slot);
            if e.vcall(form, 0xf0, &[]).bool() {
                reference = e.mem.u32(form_slot);
                load = e.call(REFERENCE_LOADED_3D, &args![reference]).u32() != 0;
                if e.vcall(reference, 0x100, &[]).bool() {
                    frees = reference;
                }
            }
            if load {
                let form = e.mem.u32(form_slot);
                match kind {
                    0 => {
                        e.call(LOAD_ANIMATION, &args![sub_buffer, form]);
                    }
                    1 => {
                        e.call(LOAD_CHAR_CONTROLLER, &args![sub_buffer, form]);
                    }
                    2 => {
                        e.call(LOAD_HAVOK_DATA, &args![sub_buffer, form]);
                    }
                    _ => {}
                }
            } else {
                match kind {
                    0 => {
                        if frees != 0 {
                            e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
                        } else if reference != 0 {
                            let data = e.call(SUB_BUFFER_DATA, &args![sub_buffer]).u32();
                            let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                            e.call(SET_SAVED_ANIMATION, &args![list, data]);
                        }
                    }
                    1 => {
                        e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
                    }
                    2 => {
                        if reference != 0 {
                            let data = e.call(SUB_BUFFER_DATA, &args![sub_buffer]).u32();
                            let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                            e.call(SET_SAVED_HAVOK_DATA, &args![list, data]);
                        }
                    }
                    _ => {}
                }
            }
        }
    });
    e.call(BASE_TEARDOWN, &args![map]);
}

// Translated from 00865360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: loads the queued sub-buffers of all three
/// kinds (`00865390` with 0, 1, 2).
pub fn fn_00865360(e: &mut Engine, this: Ptr) {
    load_queued_sub_buffers(e, this, 0);
    load_queued_sub_buffers(e, this, 1);
    load_queued_sub_buffers(e, this, 2);
}

// Translated from 00865120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the save-form buffer `buffer` into a
/// new block stored at `this + 0`: one version byte (`008a81c0`), the
/// buffer's size in the variable-sized encoding (`00845fc0`), then the
/// buffer's bytes. Nothing happens for an empty buffer.
pub fn fn_00865120(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let size = e.call(WORD_AT_C, &args![buffer]).u32();
    if size == 0 {
        return;
    }
    let size_length = e.call(VARIABLE_SIZE_LENGTH, &args![size]).u32();
    let total = size.wrapping_add(size_length).wrapping_add(1);
    let block = e.call(ALLOCATE_BLOCK, &args![total]).u32();
    e.mem.set_u32(this.addr(), block);
    let mut cursor = e.mem.u32(this.addr());
    let version_at = cursor;
    let version = e.call(BUFFER_GET_VERSION, &args![buffer]).u32() as u8;
    e.mem.set_u8(version_at, version);
    cursor = cursor.wrapping_add(1);
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![cursor, size]);
    cursor = cursor.wrapping_add(size_length);
    let source = e.call(WORD_AT_4, &args![buffer]).u32();
    e.call(COPY_BYTES, &args![cursor, source, size]);
}

// Translated from 00865510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::SaveGame` (Xbox PDB name in the engine map): reads
/// the size stored after the version byte of the data at `this + 0`
/// (`00846080`), writes it to the save buffer (`00865f60`) and appends the
/// bytes that follow it (`00865570`, without separator).
pub fn bgs_load_game_sub_buffer_save_game(e: &mut Engine, this: Ptr, save_buffer: Ptr) {
    let mut cursor = e.mem.u32(this.addr()).wrapping_add(1);
    let size = e.with_stack(4, |e, value| {
        let read = e
            .call(LOAD_VARIABLE_SIZED_VALUE, &args![cursor, value])
            .u32();
        cursor = read.wrapping_add(cursor);
        e.mem.u32(value.addr())
    });
    e.call(SAVE_GAME_VARIABLE_SIZED_VALUE, &args![save_buffer, size]);
    append_bytes(e, save_buffer, Ptr::new(cursor), size, 1);
}

/// The body `00864f20` and `00865020` share: when `form` is a reference
/// (vtable +0xF0), builds a save-form buffer on the stack for it (version
/// 0x1b, the form's type byte, change flags 4, its id), lets the reference
/// write into it through `save` and copies it into this sub-buffer
/// (`00865120`); otherwise logs the "not a reference" warning with the
/// form's name (vtable +0x130) and id.
fn save_reference_data(e: &mut Engine, this: Ptr, form: Ptr, save: u32) {
    if e.vcall(form.addr(), 0xf0, &[]).bool() {
        let reference = form;
        e.with_stack(0x24, |e, buffer| {
            e.call(SAVE_FORM_BUFFER_CONSTRUCT, &args![buffer]);
            let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
            let version = e.call(MANAGER_VERSION, &args![manager]).u32();
            let form_type = e.call(BYTE_AT_4, &args![form]).u32();
            // The change flags are an object passed by value: built in its
            // argument slot.
            let flags = e.with_stack(4, |e, slot| {
                e.call(CHANGE_FLAGS_CONSTRUCT, &args![slot, 4u32]);
                e.mem.u32(slot.addr())
            });
            let form_id = e.call(WORD_AT_C, &args![form]).u32();
            e.call(
                SAVE_FORM_BUFFER_INIT,
                &args![buffer, form_id, flags, form_type, version],
            );
            e.call(SAVE_FORM_BUFFER_SET_FORM, &args![buffer, form]);
            e.call(save, &args![reference, buffer]);
            fn_00865120(e, this, buffer);
            e.call(SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);
        });
    } else {
        let form_id = e.call(WORD_AT_C, &args![form]).u32();
        let name = e.vcall(form.addr(), 0x130, &[]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![NOT_A_REFERENCE_FORMAT, name, form_id],
        );
    }
}

// Translated from 00864f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: saves a reference's animation into this
/// sub-buffer (`TESObjectREFR::SaveAnimation`, `00563650`, through a
/// save-form buffer); a form that is not a reference only logs a warning.
// Translation incomplete: 00864f20 - SEH exception unwinding (the stack buffer's destructor on unwind) is unsupported.
pub fn fn_00864f20(e: &mut Engine, this: Ptr, form: Ptr) {
    save_reference_data(e, this, form, REFERENCE_SAVE_ANIMATION);
}

// Translated from 00865020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: saves a reference's Havok data into this
/// sub-buffer (`TESObjectREFR::SaveHavokData`, `00562de0`, through a
/// save-form buffer); a form that is not a reference only logs the same
/// warning as `00864f20`.
// Translation incomplete: 00865020 - SEH exception unwinding (the stack buffer's destructor on unwind) is unsupported.
pub fn fn_00865020(e: &mut Engine, this: Ptr, form: Ptr) {
    save_reference_data(e, this, form, REFERENCE_SAVE_HAVOK_DATA);
}

// Translated from 00865220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the three queued sub-buffer
/// maps at `this` (0x10 bytes each): frees every queued sub-buffer, tears
/// each map down (`00438af0`), then runs the eh vector destructor iterator
/// over the array with `008657e0`.
// Translation incomplete: 00865220 - SEH exception unwinding is unsupported.
pub fn fn_00865220(e: &mut Engine, this: Ptr) {
    for index in 0u32..3 {
        let map = this.addr().wrapping_add(index << 4);
        e.with_stack(0xc, |e, locals| {
            let position = locals.addr();
            let form_slot = position + 4;
            let sub_buffer = position + 8;
            let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                e.mem.set_u32(form_slot, 0);
                e.call(SUB_BUFFER_CONSTRUCT, &args![sub_buffer]);
                e.call(MAP_GET_NEXT, &args![map, position, form_slot, sub_buffer]);
                e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
            }
        });
        e.call(BASE_TEARDOWN, &args![map]);
    }
    e.call(VECTOR_DESTRUCT, &args![this, 0x10u32, 3u32, 0x0086_57e0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00865570, append_bytes(Ptr, Ptr, u32, u8)),
        entry!(0x008656c0, fn_008656c0(Ptr, u32) -> Ptr),
        entry!(0x00865660, fn_00865660(Ptr, u32) -> Ptr),
        entry!(0x00865200, fn_00865200(Ptr) -> u32),
        entry!(0x00865840, fn_00865840(Ptr) -> u32),
        entry!(0x008657e0, fn_008657e0(Ptr) -> u32),
        entry!(0x008651d0, fn_008651d0(Ptr) -> Ptr),
        entry!(0x0086_5870, fn_00865870(Ptr) -> Ptr),
        entry!(0x0086_5390, load_queued_sub_buffers(Ptr, i32)),
        entry!(0x0086_5360, fn_00865360(Ptr)),
        entry!(0x0086_5120, fn_00865120(Ptr, Ptr)),
        entry!(0x0086_5510, bgs_load_game_sub_buffer_save_game(Ptr, Ptr)),
        entry!(0x0086_4f20, fn_00864f20(Ptr, Ptr)),
        entry!(0x0086_5020, fn_00865020(Ptr, Ptr)),
        entry!(0x0086_5220, fn_00865220(Ptr)),
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
        e.register(BASE_TEARDOWN, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn fn_008656c0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008656c0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.bytes(table, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![12]));
        assert_eq!(log[2], (MEMSET, vec![table, 0, 12]));
    }

    #[test]
    fn fn_00865660_and_00865200_store_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00865660, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_2288);
        assert_eq!(e.mem.u32(this + 4), 9, "the base initializer ran");
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00865200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0x25);
    }

    #[test]
    fn fn_00865840_releases_the_block_at_8_and_returns_the_result() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00865840, &args![this]).u32(), 0x7001);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_TEARDOWN, vec![this]));
        assert_eq!(log[2], (FREE_BLOCK, vec![0x7000]));
    }

    #[test]
    fn fn_008657e0_tears_down_twice_and_ends_on_the_second_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x008657e0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![BASE_TEARDOWN, BASE_TEARDOWN, FREE_BLOCK]);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
    }

    #[test]
    fn fn_008651d0_constructs_three_elements() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008651d0, &args![0x6000u32]).u32(), 0x6000);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (
                VECTOR_CONSTRUCT,
                vec![0x6000, 0x10, 3, 0x0086_5200, 0x0086_57e0]
            )
        );
    }

    #[test]
    fn l68d_00865870_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00865900, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0086_5870, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00865900).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }

    /// Doubles every callee of the queued-map walkers: `MAP_GET_NEXT`
    /// hands out `forms` in order (the position goes to 0 after the last).
    fn queue_doubles(e: &mut Engine, forms: Vec<u32>) {
        e.register(MAP_FIRST_ITEM, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let mut next = 0usize;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            e.mem.set_u32(a[2], forms[next]);
            e.mem.set_u32(a[3], 0x5100 + next as u32);
            next += 1;
            e.mem.set_u32(a[1], if next < forms.len() { 1 } else { 0 });
            Ret::default()
        });
        for addr in [
            SUB_BUFFER_CONSTRUCT,
            SUB_BUFFER_FREE,
            LOAD_ANIMATION,
            LOAD_CHAR_CONTROLLER,
            LOAD_HAVOK_DATA,
            SET_SAVED_ANIMATION,
            SET_SAVED_HAVOK_DATA,
            BASE_TEARDOWN,
            VECTOR_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SUB_BUFFER_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(REFERENCE_EXTRA_LIST, |_, a| Ret {
            eax: a[0] + 0x44,
            ..Ret::default()
        });
        // A form's byte at +8 says whether it is a reference, +9 whether its
        // 3D is loaded, +10 what vtable +0x100 answers.
        e.register(0x7000_0f00, |e, a| Ret {
            eax: e.mem.u8(a[0] + 8) as u32,
            ..Ret::default()
        });
        e.register(REFERENCE_LOADED_3D, |e, a| Ret {
            eax: e.mem.u8(a[0] + 9) as u32,
            ..Ret::default()
        });
        e.register(0x7000_1000, |e, a| Ret {
            eax: e.mem.u8(a[0] + 10) as u32,
            ..Ret::default()
        });
        e.map(0x9100_0000, 0x200);
        e.mem.set_u32(0x9100_0000 + 0xf0, 0x7000_0f00);
        e.mem.set_u32(0x9100_0000 + 0x100, 0x7000_1000);
    }

    fn queued_form(e: &mut Engine, reference: u8, loaded: u8, frees: u8) -> u32 {
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form, 0x9100_0000);
        e.mem.set_u8(form + 8, reference);
        e.mem.set_u8(form + 9, loaded);
        e.mem.set_u8(form + 10, frees);
        form
    }

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn load_queued_sub_buffers_loads_or_hands_over_by_kind() {
        let mut e = Engine::new();
        let plain = queued_form(&mut e, 0, 0, 0);
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 0);
        let freeing = queued_form(&mut e, 1, 0, 1);
        queue_doubles(&mut e, vec![plain, loaded, unloaded, freeing]);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 0u32]);
        let loads: Vec<u32> = calls_to(&mut e, LOAD_ANIMATION)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(loads, vec![plain, loaded]);
        assert_eq!(calls_to(&mut e, SET_SAVED_ANIMATION).len(), 1);
        assert_eq!(calls_to(&mut e, SET_SAVED_ANIMATION)[0][0], unloaded + 0x44);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 1);
        assert_eq!(calls_to(&mut e, BASE_TEARDOWN), vec![vec![this]]);

        let mut e = Engine::new();
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 0);
        queue_doubles(&mut e, vec![loaded, unloaded]);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 1u32]);
        assert_eq!(calls_to(&mut e, LOAD_CHAR_CONTROLLER).len(), 1);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 1);
        assert_eq!(calls_to(&mut e, MAP_FIRST_ITEM), vec![vec![this + 0x10]]);

        let mut e = Engine::new();
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 1);
        queue_doubles(&mut e, vec![loaded, unloaded]);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 2u32]);
        assert_eq!(calls_to(&mut e, LOAD_HAVOK_DATA)[0][1], loaded);
        let saved = calls_to(&mut e, SET_SAVED_HAVOK_DATA);
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0][0], unloaded + 0x44);
        assert_eq!(calls_to(&mut e, BASE_TEARDOWN), vec![vec![this + 0x20]]);
    }

    #[test]
    fn fn_00865360_loads_all_three_kinds() {
        let mut e = Engine::new();
        queue_doubles(&mut e, vec![]);
        e.register(MAP_FIRST_ITEM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_5360, &args![0x6000u32]);
        assert_eq!(
            calls_to(&mut e, BASE_TEARDOWN),
            vec![vec![0x6000], vec![0x6010], vec![0x6020]]
        );
    }

    #[test]
    fn fn_00865120_copies_the_buffer_behind_version_and_size() {
        let mut e = Engine::new();
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(VARIABLE_SIZE_LENGTH, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BUFFER_GET_VERSION, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(COPY_BYTES, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0086_5120, &args![this, buffer]);
        assert_eq!(e.mem.u32(this), 0, "an empty buffer is not copied");
        assert!(calls_to(&mut e, ALLOCATE_BLOCK).is_empty());

        e.mem.set_u32(buffer + 0xc, 5);
        e.mem.set_u32(buffer + 4, 0x4444);
        e.call(0x0086_5120, &args![this, buffer]);
        let block = e.mem.u32(this);
        assert_eq!(calls_to(&mut e, ALLOCATE_BLOCK), vec![vec![8]]);
        assert_eq!(e.mem.u8(block), 0x34);
        assert_eq!(
            calls_to(&mut e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![block + 1, 5]]
        );
        assert_eq!(
            calls_to(&mut e, COPY_BYTES),
            vec![vec![block + 3, 0x4444, 5]]
        );
    }

    #[test]
    fn save_game_writes_the_size_then_the_bytes() {
        let mut e = Engine::new();
        e.register(LOAD_VARIABLE_SIZED_VALUE, |e, a| {
            e.mem.set_u32(a[1], 7);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        e.register(SAVE_GAME_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(0x00865ce0, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x9000);
        let save = e.mem.alloc(0x10);
        e.mem.set_u32(save + 0xc, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x0086_5510, &args![this, save]);
        assert_eq!(calls_to(&mut e, LOAD_VARIABLE_SIZED_VALUE)[0][0], 0x9001);
        assert_eq!(
            calls_to(&mut e, SAVE_GAME_VARIABLE_SIZED_VALUE),
            vec![vec![save, 7]]
        );
        assert_eq!(
            calls_to(&mut e, 0x00865ce0),
            vec![vec![save, 0x9003, 7, 0x33, 1]]
        );
    }

    fn saver_doubles(e: &mut Engine) {
        for addr in [
            SAVE_FORM_BUFFER_CONSTRUCT,
            SAVE_FORM_BUFFER_INIT,
            SAVE_FORM_BUFFER_SET_FORM,
            REFERENCE_SAVE_ANIMATION,
            REFERENCE_SAVE_HAVOK_DATA,
            SAVE_FORM_BUFFER_DESTRUCT,
            SAVE_GAME_WARNING,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MANAGER_VERSION, |_, _| Ret {
            eax: 0x1b,
            ..Ret::default()
        });
        e.register(BYTE_AT_4, |e, a| Ret {
            eax: e.mem.u8(a[0] + 4) as u32,
            ..Ret::default()
        });
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(0x7000_0f00, |e, a| Ret {
            eax: e.mem.u8(a[0] + 8) as u32,
            ..Ret::default()
        });
        e.register(0x7000_1300, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        e.map(0x9100_0000, 0x200);
        e.mem.set_u32(0x9100_0000 + 0xf0, 0x7000_0f00);
        e.mem.set_u32(0x9100_0000 + 0x130, 0x7000_1300);
        e.map(SAVE_LOAD_MANAGER, 4);
        e.set_global(SAVE_LOAD_MANAGER, 0x6600u32);
    }

    #[test]
    fn fn_00864f20_and_00865020_save_through_a_buffer_or_warn() {
        for (addr, save) in [
            (0x0086_4f20u32, REFERENCE_SAVE_ANIMATION),
            (0x0086_5020, REFERENCE_SAVE_HAVOK_DATA),
        ] {
            let mut e = Engine::new();
            saver_doubles(&mut e);
            let form = e.mem.alloc(0x10);
            e.mem.set_u32(form, 0x9100_0000);
            e.mem.set_u8(form + 4, 0x3b);
            e.mem.set_u32(form + 0xc, 0x0001_2345);
            let this = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, form]);
            assert_eq!(
                calls_to(&mut e, SAVE_GAME_WARNING),
                vec![vec![NOT_A_REFERENCE_FORMAT, 0x0abc_0000, 0x0001_2345]]
            );
            assert!(calls_to(&mut e, save).is_empty());

            e.mem.set_u8(form + 8, 1);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, form]);
            let init = calls_to(&mut e, SAVE_FORM_BUFFER_INIT);
            let buffer = init[0][0];
            assert_eq!(init[0][1..].to_vec(), vec![0x0001_2345, 4, 0x3b, 0x1b]);
            assert_eq!(calls_to(&mut e, MANAGER_VERSION), vec![vec![0x6600]]);
            assert_eq!(calls_to(&mut e, save), vec![vec![form, buffer]]);
            assert_eq!(
                calls_to(&mut e, SAVE_FORM_BUFFER_DESTRUCT),
                vec![vec![buffer]]
            );
            assert!(calls_to(&mut e, SAVE_GAME_WARNING).is_empty());
        }
    }

    #[test]
    fn fn_00865220_frees_every_queued_buffer_and_destroys_the_maps() {
        let mut e = Engine::new();
        queue_doubles(&mut e, vec![0x10, 0x20]);
        // Each map holds two entries: the queue restarts for each map.
        let mut handed = 0u32;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            handed += 1;
            e.mem.set_u32(a[1], handed % 2);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_5220, &args![0x6000u32]);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 6);
        assert_eq!(
            calls_to(&mut e, BASE_TEARDOWN),
            vec![vec![0x6000], vec![0x6010], vec![0x6020]]
        );
        assert_eq!(
            calls_to(&mut e, VECTOR_DESTRUCT),
            vec![vec![0x6000, 0x10, 3, 0x0086_57e0]]
        );
    }
}
