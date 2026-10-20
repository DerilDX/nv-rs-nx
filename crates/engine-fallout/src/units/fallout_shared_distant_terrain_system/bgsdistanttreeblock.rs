//! `fallout shared/distant terrain system/bgsdistanttreeblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map-item allocator: allocates twelve bytes without initializing them.
pub fn allocate_item(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x00401000, &args![12u32]).ptr()
}

/// The function `fn_006f9080` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 006f9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x1c` and passes on nothing (the callee returns no value).
pub fn fn_006f9080(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x1c)]);
}

// Translated from 006f94a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106ded0` (RTTI `.?AVBGSDistantTreeBlockLoadTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_006f94a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_ded0);
    e.call(0x00c3_cea0, &args![this]);
}

/// The constructor body `006fa180` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_006FA180: u32 = 0x006f_a180;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,256>`).
const VTABLE_006FA100: u32 = 0x0106_dfe0;

// Translated from 006fa100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bgsdistanttreeblock.cpp`): stores the vtable `0106dfe0`
/// (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,...>`), then calls
/// `006fa180(this, 0, 0)`. Returns `this`.
pub fn fn_006fa100(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_006FA100);
    e.call(CONSTRUCTOR_BODY_006FA180, &args![this, 0u32, 0u32]);
    this
}

// The functions below: `BGSDistantTreeBlock` (Xbox PDB layout, size 0x4c: `TreeGroups` at
// `+4`, `InstanceMap` at `+0x14`, `TreesQueuedToHide` at `+0x34`), `BGSDistantTreeBlockLoadTask`
// (size 0x48: `pBlock` `+0x30`, `bFinishedQueuing` `+0x34`, `iX` `+0x38`, `iY` `+0x3c`, `iLevel`
// `+0x40`, `bCanceled` `+0x44`, `bFileExists` `+0x45`; the PC code uses the same offsets) and the
// `BSMap`/`BSSimpleArray` template code of this unit. `InstanceData` keeps `uiGroup` at `+0x18`
// and `bHidden` at `+0x1c`; `TreeGroup` keeps `spGeometry` at `+4`, `Instances` at `+8` and
// `bShaderPropertyUpToDate` at `+0x1c` (Xbox PDB).

/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00559450` (thiscall, no stack words): the object a smart pointer holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `0066b0d0` (thiscall, one word): assigns a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `0044ddc0` (thiscall, no stack words): an array's count.
const ARRAY_COUNT_0044DDC0: u32 = 0x0044_ddc0;
/// `00877a30` (thiscall, one word: the index): the address of an array entry.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `008454f0` (thiscall, one word): frees an array (called with 1).
const ARRAY_FREE_008454F0: u32 = 0x0084_54f0;
/// `00406d00` (cdecl, variable arguments: buffer, size, format, ...): formats into a buffer.
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The smart pointers and the flag the static data lives in (`.data`; the Xbox PDB lists the
/// statics `spSharedStencil`, `spSharedAlpha`, `spAtlasTexture`, `TreeTypes` and
/// `bStaticDataLoaded`, which address is which is not confirmed).
const STATIC_POINTER_011D85BC: u32 = 0x011d_85bc;
const STATIC_POINTER_011D85C4: u32 = 0x011d_85c4;
const STATIC_TEXTURE_011D85E0: u32 = 0x011d_85e0;
const STATIC_ARRAY_011D85D0: u32 = 0x011d_85d0;
const STATIC_LOADED_011D85A0: u32 = 0x011d_85a0;
/// The smart pointer `006f7a60` sets (`.data`).
const STATIC_POINTER_012007C4: u32 = 0x0120_07c4;
/// The security cookie (`.data`) and `__security_check_cookie` (`00ec408c`, fastcall).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;

// Translated from 006f77a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f77a0 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::AllocateStaticData` (the map's name, from its strings; cdecl, one
/// word: the worldspace). Formats `"Data\Meshes\Landscape\LOD\%s\Trees\TreeTypes.lst"`
/// (`0106de08`) with the worldspace's slot `0x130` into a 0x104-byte stack buffer, opens it
/// (`00afdf00(path, 0, 0x800)`, cdecl) and when that gives a file reads the tree types
/// (`006f8f80`, cdecl) and deletes the file (its slot 0 with 1). The texture is
/// `"TreeDeadLOD.dds"` (`0106ddf8`), or `"TreeSwampLOD.dds"` (`0106dddc`) when the name starts
/// with `"DLC4"` (`strncmp`, `00ec8a19`, 4 characters); the path `"\Data\Textures\Landscape\
/// Trees\%s"` (`0106ddb8`) goes in a second buffer, and the texture manager `011dea10` makes it
/// into `011d85e0` (`004568c0`, four words: path, `011d85e0`, 1, 1). Without a texture it
/// reports `"LOD tree texture atlas not found!"` (`005b5e40`, `0106dd94`). Then
/// [`fn_006f7a60`] with the texture, `011d85a0` set to 1, a 0x24-byte object (`00aa13e0`,
/// constructed by `0049ec80`, or 0) into `011d85bc` with `0049ee30(3)` on it, and a 0x1c-byte
/// object (constructed by `004391c0`, or 0) into `011d85c4` with `004393c0(1)`, `0049ed90(0)`,
/// `0094db80(0x80)` and `004393e0(4)` on it. Ends with the cookie check.
pub fn bgs_distant_tree_block_allocate_static_data(e: &mut Engine, world: u32) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    e.with_stack(0x210, |e, buffers| {
        let list_path = buffers.addr();
        let texture_path = buffers.addr() + 0x108;
        let name = e.vcall(world, 0x130, &args![]).u32();
        e.call(
            FORMAT_00406D00,
            &args![list_path, 0x104u32, 0x0106_de08u32, name],
        );
        let file = e.call(0x00af_df00, &args![list_path, 0u32, 0x800u32]).u32();
        if file != 0 {
            e.call(0x006f_8f80, &args![file]);
            if file != 0 {
                e.vcall(file, 0, &args![1u32]);
            }
        }
        let mut texture = 0x0106_ddf8u32;
        let name = e.vcall(world, 0x130, &args![]).u32();
        if e.call(0x00ec_8a19, &args![name, 0x0106_ddf0u32, 4u32])
            .u32()
            == 0
        {
            texture = 0x0106_dddc;
        }
        e.call(
            FORMAT_00406D00,
            &args![texture_path, 0x104u32, 0x0106_ddb8u32, texture],
        );
        let manager = e.global::<u32>(0x011d_ea10);
        e.call(
            0x0045_68c0,
            &args![manager, texture_path, STATIC_TEXTURE_011D85E0, 1u32, 1u32],
        );
    });
    if e.call(POINTER_GET_00559450, &args![STATIC_TEXTURE_011D85E0])
        .u32()
        == 0
    {
        e.call(0x005b_5e40, &args![0x0106_dd94u32]);
    }
    let texture = e
        .call(POINTER_GET_00559450, &args![STATIC_TEXTURE_011D85E0])
        .u32();
    fn_006f7a60(e, texture);
    e.mem.set_u8(STATIC_LOADED_011D85A0, 1);
    let block = e.call(0x00aa_13e0, &args![0x24u32]).u32();
    let object = if block != 0 {
        e.call(0x0049_ec80, &args![block]).u32()
    } else {
        0
    };
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_011D85BC, object],
    );
    let held = e
        .call(POINTER_GET_00559450, &args![STATIC_POINTER_011D85BC])
        .u32();
    e.call(0x0049_ee30, &args![held, 3u32]);
    let block = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
    let object = if block != 0 {
        e.call(0x0043_91c0, &args![block]).u32()
    } else {
        0
    };
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_011D85C4, object],
    );
    for (callee, value) in [
        (0x0043_93c0u32, 1u32),
        (0x0049_ed90, 0),
        (0x0094_db80, 0x80),
        (0x0043_93e0, 4),
    ] {
        let held = e
            .call(POINTER_GET_00559450, &args![STATIC_POINTER_011D85C4])
            .u32();
        e.call(callee, &args![held, value]);
    }
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

// Translated from 006f7a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): assigns the word to the smart pointer at `012007c4` (`0066b0d0`).
pub fn fn_006f7a60(e: &mut Engine, object: u32) {
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_012007C4, object],
    );
}

// Translated from 006f83f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the static data's release. For each entry of the array at
/// `011d85d0` (count `0044ddc0`, entry `006b3120`) clears the smart pointer at the entry's
/// `+0x1c`; clears the smart pointers at `011d85bc`, `011d85c4` and `011d85e0`, runs
/// [`fn_006f7a60`] with 0 and clears the flag at `011d85a0`.
pub fn fn_006f83f0(e: &mut Engine) {
    let mut index = 0u32;
    while index
        < e.call(ARRAY_COUNT_0044DDC0, &args![STATIC_ARRAY_011D85D0])
            .u32()
    {
        let entry = e
            .call(0x006b_3120, &args![STATIC_ARRAY_011D85D0, index])
            .u32();
        e.call(POINTER_SET_0066B0D0, &args![entry.wrapping_add(0x1c), 0u32]);
        index = index.wrapping_add(1);
    }
    e.call(POINTER_SET_0066B0D0, &args![STATIC_POINTER_011D85BC, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![STATIC_POINTER_011D85C4, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![STATIC_TEXTURE_011D85E0, 0u32]);
    fn_006f7a60(e, 0);
    e.mem.set_u8(STATIC_LOADED_011D85A0, 0);
}

// Translated from 006f86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`): for each tree group of `TreeGroups` (`+4`)
/// frees its `Instances` (`008454f0(group + 8, 1)`); when its `spGeometry` (`+4`) holds an
/// object, slot `0xe8` of `004ea9e0(00559450(008041a0(this)))` is called with it and the
/// pointer is cleared; a set group is deleted (`006f8800(group, 1)`). Then `TreeGroups` is freed
/// (`008454f0(this + 4, 1)`).
pub fn fn_006f86e0(e: &mut Engine, this: Ptr) {
    let groups = this.addr() + 4;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT_0044DDC0, &args![groups]).u32() {
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        e.call(ARRAY_FREE_008454F0, &args![group.wrapping_add(8), 1u32]);
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        if e.call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
            .u32()
            != 0
        {
            let first = e.call(0x0080_41a0, &args![this]).u32();
            let second = e.call(POINTER_GET_00559450, &args![first]).u32();
            let parent = e.call(0x004e_a9e0, &args![second]).u32();
            let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
            let group = e.mem.u32(group_at);
            let geometry = e
                .call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
                .u32();
            e.vcall(parent, 0xe8, &args![geometry]);
            let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
            let group = e.mem.u32(group_at);
            e.call(POINTER_SET_0066B0D0, &args![group.wrapping_add(4), 0u32]);
        }
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        if group != 0 {
            e.call(0x006f_8800, &args![group, 1u32]);
        }
        index = index.wrapping_add(1);
    }
    e.call(ARRAY_FREE_008454F0, &args![groups, 1u32]);
}

// Translated from 006f8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `BGSDistantTreeBlockLoadTask::BGSDistantTreeBlockLoadTask` (from its
/// string); it formats the block's file name (cdecl, five words: buffer, worldspace, level,
/// x, y): `"Data\Meshes\Landscape\LOD\%s\Trees\%s.Level%i.X%i.Y%i.DTL"` (`0106de90`, 0x104
/// bytes) with the worldspace's slot `0x130` twice, the level and the two 16-bit coordinates
/// sign-extended.
pub fn fn_006f8890(e: &mut Engine, buffer: u32, world: u32, level: u32, x: u32, y: u32) {
    let y = y as u16 as i16 as i32 as u32;
    let x = x as u16 as i16 as i32 as u32;
    let first = e.vcall(world, 0x130, &args![]).u32();
    let second = e.vcall(world, 0x130, &args![]).u32();
    e.call(
        FORMAT_00406D00,
        &args![buffer, 0x104u32, 0x0106_de90u32, second, first, level, x, y],
    );
}

/// The shape of [`fn_006f9230`] and [`fn_006f92a0`]: looks the id up in `InstanceMap` (`+0x14`;
/// `00853130`, thiscall, two words: the id and the address of a result local set to 0); when
/// found sets `bHidden` (`+0x1c`) of the instance to `hidden` and clears
/// `bShaderPropertyUpToDate` (`+0x1c`) of its group (entry `uiGroup`, `+0x18`, of `TreeGroups`).
/// Returns whether it was found.
fn set_instance_hidden(e: &mut Engine, this: Ptr, id: u32, hidden: u8) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, result| {
        let found = e.call(0x0085_3130, &args![base + 0x14, id, result]).u32() as u8;
        if found == 0 {
            return false;
        }
        let instance = e.mem.u32(result.addr());
        e.mem.set_u8(instance + 0x1c, hidden);
        let group_index = e.mem.u32(instance + 0x18);
        let group_at = e
            .call(ARRAY_AT_00877A30, &args![base + 4, group_index])
            .u32();
        let group = e.mem.u32(group_at);
        e.mem.set_u8(group + 0x1c, 0);
        true
    })
}

// Translated from 006f9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`, two words: the instance id and a byte; `RET
/// 8`): hides the instance (see [`set_instance_hidden`]); when it is not in the map and the
/// byte is set, adds the id (by the address of its argument word) to `TreesQueuedToHide`
/// (`+0x34`; `007cb2e0`, thiscall).
pub fn fn_006f9230(e: &mut Engine, this: Ptr, id: u32, queue: u32) {
    if set_instance_hidden(e, this, id, 1) {
        return;
    }
    if queue as u8 != 0 {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), id);
            e.call(0x007c_b2e0, &args![this.addr() + 0x34, argument]);
        });
    }
}

// Translated from 006f92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`, one word: the instance id; `RET 4`): shows the
/// instance (see [`set_instance_hidden`] with 0).
pub fn fn_006f92a0(e: &mut Engine, this: Ptr, id: u32) {
    set_instance_hidden(e, this, id, 0);
}

// Translated from 006f9360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9360 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: the `BGSDistantTreeBlockLoadTask` constructor (by its vtable `0106ded0`; thiscall,
/// six words, `RET 0x18`, the last not read). The base constructor `00c3ce60(this, 3)`; the
/// vtable; `pBlock`, `bFinishedQueuing` 0, `iX`, `iY`, `iLevel`, `bCanceled` 0; the file name
/// ([`fn_006f8890`] with the worldspace, the level and the low 16 bits of x and y) into a
/// 0x108-byte stack buffer, of which `00c3cee0` (thiscall) gets the part after its first five
/// characters; `00c3cf60(this, 0)`; `bFileExists` 0. Ends with the cookie check. Returns
/// `this`.
pub fn fn_006f9360(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    world: u32,
    x: u32,
    y: u32,
    level: u32,
    _unused_6: u32,
) -> Ptr {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    let base = this.addr();
    e.call(0x00c3_ce60, &args![base, 3u32]);
    e.mem.set_u32(base, VTABLE_LOAD_TASK_0106DED0);
    e.mem.set_u32(base + 0x30, block);
    e.mem.set_u8(base + 0x34, 0);
    e.mem.set_u32(base + 0x38, x);
    e.mem.set_u32(base + 0x3c, y);
    e.mem.set_u32(base + 0x40, level);
    e.mem.set_u8(base + 0x44, 0);
    e.with_stack(0x108, |e, buffer| {
        fn_006f8890(e, buffer.addr(), world, level, x & 0xffff, y & 0xffff);
        e.call(0x00c3_cee0, &args![base, buffer.addr() + 5]);
    });
    e.call(0x00c3_cf60, &args![base, 0u32]);
    e.mem.set_u8(base + 0x45, 0);
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    this
}

/// The vtable of `BGSDistantTreeBlockLoadTask` (RTTI).
const VTABLE_LOAD_TASK_0106DED0: u32 = 0x0106_ded0;

// Translated from 006f9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask`'s scalar deleting destructor (the map's name; thiscall, one
/// word, `RET 4`): [`fn_006f94a0`], then with flag bit 0 `operator delete`. Returns `this`.
pub fn bgs_distant_tree_block_load_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006f94a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::QueueMe` (the map's name; thiscall): sets `bFinishedQueuing`
/// and calls its own slot `0x28`.
pub fn bgs_distant_tree_block_load_task_queue_me(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x34, 1);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 006f94e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::CheckFinished` (the map's name; thiscall): when `0043caa0`
/// answers true and `bFinishedQueuing` is set: with `0069b080` true the queue at `01202d98`
/// gets it (its slot `0x48`), otherwise the base `00c3c930` (thiscall).
pub fn bgs_distant_tree_block_load_task_check_finished(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(0x0043_caa0, &args![base]).u32() as u8 == 0 || e.mem.u8(base + 0x34) == 0 {
        return;
    }
    if e.call(0x0069_b080, &args![base]).u32() as u8 != 0 {
        let queue = e.global::<u32>(0x0120_2d98);
        e.vcall(queue, 0x48, &args![base]);
    } else {
        e.call(0x00c3_c930, &args![base]);
    }
}

// Translated from 006f9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Cancel` (the map's name; thiscall, two words, `RET 8`): sets
/// `bCanceled`, then the base `00c3cb70` with both words.
pub fn bgs_distant_tree_block_load_task_cancel(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u8(this.addr() + 0x44, 1);
    e.call(0x00c3_cb70, &args![this, first, second]);
}

// Translated from 006f9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Run` (the map's name; thiscall): opens the file
/// (`00afdf20(0045cd60(this), 0, 0x800, 1)`, cdecl); with a file and `bCanceled` clear the block
/// reads it (`006f8b40(pBlock, file)`), the file is deleted (its slot 0 with 1), `bFileExists`
/// is set and the block's `006f9300` runs; otherwise `bFileExists` is cleared.
pub fn bgs_distant_tree_block_load_task_run(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let name = e.call(0x0045_cd60, &args![base]).u32();
    let file = e
        .call(0x00af_df20, &args![name, 0u32, 0x800u32, 1u32])
        .u32();
    if file == 0 || e.mem.u8(base + 0x44) != 0 {
        e.mem.set_u8(base + 0x45, 0);
        return;
    }
    let block = e.mem.u32(base + 0x30);
    e.call(0x006f_8b40, &args![block, file]);
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.mem.set_u8(base + 0x45, 1);
    let block = e.mem.u32(base + 0x30);
    e.call(0x006f_9300, &args![block]);
}

// Translated from 006f9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Finish` (the map's name; thiscall): with `bFileExists` set
/// and `bCanceled` clear, the block's `006f88e0` (thiscall) runs.
pub fn bgs_distant_tree_block_load_task_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x45) != 0 && e.mem.u8(base + 0x44) == 0 {
        let block = e.mem.u32(base + 0x30);
        e.call(0x006f_88e0, &args![block]);
    }
}

/// The vtables of `BSMap<unsigned int, InstanceData*>`, `BSMap<unsigned int, TreeGroup*>`,
/// their `BSMapBase` bases, and `BSSimpleArray<TreeGroup*, 1024>` (RTTI).
const VTABLE_INSTANCE_MAP_0106DF38: u32 = 0x0106_df38;
const VTABLE_GROUP_MAP_0106DF58: u32 = 0x0106_df58;
const VTABLE_INSTANCE_MAP_BASE_0106DFA0: u32 = 0x0106_dfa0;
const VTABLE_GROUP_MAP_BASE_0106DFC0: u32 = 0x0106_dfc0;
const VTABLE_GROUP_ARRAY_0106DF8C: u32 = 0x0106_df8c;

// Translated from 006f9680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMap<unsigned int, InstanceData*>` constructor (by its vtable; thiscall, one
/// word: the bucket count; `RET 4`): the base [`fn_006f98c0`], then the vtable `0106df38`.
pub fn fn_006f9680(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_006f98c0(e, this, buckets);
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_MAP_0106DF38);
    this
}

// Translated from 006f96b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMap<unsigned int, TreeGroup*>` constructor (by its vtable; thiscall, one
/// word: the bucket count; `RET 4`): the base [`fn_006f9a00`], then the vtable `0106df58`.
pub fn fn_006f96b0(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_006f9a00(e, this, buckets);
    e.mem.set_u32(this.addr(), VTABLE_GROUP_MAP_0106DF58);
    this
}

// Translated from 006f9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array resize for 0x20-byte elements (thiscall, one word: the new size; `RET 4`;
/// the array keeps its buffer at `+4`, size at `+8` and capacity at `+0xc`). A size other than
/// the capacity: entries past it are destroyed first (`0072ba80(buffer + size * 0x20, old -
/// size)`, thiscall) and the size lowered, then the buffer is resized ([`fn_006f9eb0`] with the
/// new size and the size) and the capacity set.
pub fn fn_006f9790(e: &mut Engine, this: Ptr, count: u32) {
    let base = this.addr();
    if count == e.mem.u32(base + 0xc) {
        return;
    }
    if count < e.mem.u32(base + 8) {
        let excess = e.mem.u32(base + 8).wrapping_sub(count);
        let start = (count << 5).wrapping_add(e.mem.u32(base + 4));
        e.call(0x0072_ba80, &args![base, start, excess]);
        e.mem.set_u32(base + 8, count);
    }
    let size = e.mem.u32(base + 8);
    fn_006f9eb0(e, this, count, size);
    e.mem.set_u32(base + 0xc, count);
}

// Translated from 006f9870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<TreeGroup*, 1024>` constructor (by its vtable; thiscall): the
/// vtable `0106df8c`, then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_006f9870(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_ARRAY_0106DF8C);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f98a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<TreeGroup*, 1024>` destructor (thiscall): the vtable `0106df8c`,
/// then `008454f0(this, 1)`.
pub fn fn_006f98a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_ARRAY_0106DF8C);
    e.call(ARRAY_FREE_008454F0, &args![this, 1u32]);
}

/// The `BSMapBase` constructor shape (thiscall, one word: the bucket count): the vtable, the
/// count at `+4`, 0 at `+0xc`; with buckets, a zeroed table of `count * 4` bytes at `+8`
/// (`operator new`, `memset` `00ec61c0`). Returns `this`.
fn map_base_construct(e: &mut Engine, this: Ptr, buckets: u32, vtable: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + 4, buckets);
    e.mem.set_u32(base + 0xc, 0);
    if buckets != 0 {
        let bytes = e.mem.u32(base + 4) << 2;
        let table = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.mem.set_u32(base + 8, table);
        let bytes = e.mem.u32(base + 4) << 2;
        let table = e.mem.u32(base + 8);
        e.call(0x00ec_61c0, &args![table, 0u32, bytes]);
    }
    this
}

// Translated from 006f98c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, InstanceData*>` constructor (by its vtable `0106dfa0`;
/// thiscall, one word, `RET 4`): see [`map_base_construct`].
pub fn fn_006f98c0(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, VTABLE_INSTANCE_MAP_BASE_0106DFA0)
}

// Translated from 006f9940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9940 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSMap<unsigned int, InstanceData*>` destructor (thiscall): the vtable
/// `0106df38`, the entries removed (`00438af0`, thiscall), then the base [`fn_006f99a0`].
pub fn fn_006f9940(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_MAP_0106DF38);
    e.call(0x0043_8af0, &args![this]);
    fn_006f99a0(e, this);
}

// Translated from 006f99a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, InstanceData*>` destructor (thiscall): the vtable
/// `0106dfa0`; a set bucket table (`+8`) is freed (`operator delete`).
pub fn fn_006f99a0(e: &mut Engine, this: Ptr) {
    map_base_destroy(e, this, VTABLE_INSTANCE_MAP_BASE_0106DFA0);
}

/// The `BSMapBase` destructor shape: the vtable, then the bucket table at `+8` freed when set.
fn map_base_destroy(e: &mut Engine, this: Ptr, vtable: u32) {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    let table = e.mem.u32(base + 8);
    if table != 0 {
        e.call(OPERATOR_DELETE, &args![table]);
    }
}

// Translated from 006f9a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, TreeGroup*>` constructor (by its vtable `0106dfc0`;
/// thiscall, one word, `RET 4`): see [`map_base_construct`].
pub fn fn_006f9a00(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, VTABLE_GROUP_MAP_BASE_0106DFC0)
}

// Translated from 006f9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<unsigned int, InstanceData*>::SetAt` (the map's name; thiscall, two words: key
/// and value; `RET 8`). The bucket is the map's slot 4 on the key; its chain (`+0` the next
/// item, `+4` the key) is searched with slot 8 (key, item key; result in `AL`). A match is
/// cleared by slot `0x10` and set again by slot `0xc` (item, key, value); otherwise a new item
/// (slot `0x14`) is set by slot `0xc`, put in front of the bucket and the count (`+0xc`) grows.
pub fn bs_map_base_set_at(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    let bucket = e.vcall(base, 4, &args![key]).u32();
    let bucket_at = |e: &Engine| e.mem.u32(base + 8).wrapping_add(bucket.wrapping_mul(4));
    let mut item = e.mem.u32(bucket_at(e));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(base, 8, &args![key, item_key]).u32() as u8 != 0 {
            e.vcall(base, 0x10, &args![item]);
            e.vcall(base, 0xc, &args![item, key, value]);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(base, 0x14, &args![]).u32();
    e.vcall(base, 0xc, &args![item, key, value]);
    let head = e.mem.u32(bucket_at(e));
    e.mem.set_u32(item, head);
    let at = bucket_at(e);
    e.mem.set_u32(at, item);
    let count = e.mem.u32(base + 0xc).wrapping_add(1);
    e.mem.set_u32(base + 0xc, count);
}

// Translated from 006f9b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9b80 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSMap<unsigned int, TreeGroup*>` destructor (thiscall): the vtable `0106df58`,
/// the entries removed (`00438af0`, thiscall), then the base [`fn_006f9be0`].
pub fn fn_006f9b80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_MAP_0106DF58);
    e.call(0x0043_8af0, &args![this]);
    fn_006f9be0(e, this);
}

// Translated from 006f9be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, TreeGroup*>` destructor (thiscall): see
/// [`map_base_destroy`] with the vtable `0106dfc0`.
pub fn fn_006f9be0(e: &mut Engine, this: Ptr) {
    map_base_destroy(e, this, VTABLE_GROUP_MAP_BASE_0106DFC0);
}

// Translated from 006f9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array clear (thiscall, one word: a byte; `RET 4`): with a buffer (`+4`), its
/// entries are destroyed (`006fa090(this, buffer, size)`); with the byte set the buffer is freed
/// (`006a8500`, thiscall) and the buffer and capacity cleared; the size is cleared.
pub fn fn_006f9c70(e: &mut Engine, this: Ptr, free: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        return;
    }
    let buffer = e.mem.u32(base + 4);
    let size = e.mem.u32(base + 8);
    e.call(0x006f_a090, &args![base, buffer, size]);
    if free as u8 != 0 {
        e.call(0x006a_8500, &args![base]);
        e.mem.set_u32(base + 4, 0);
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 8, 0);
}

// Translated from 006f9eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array buffer resize (thiscall, two words: the new capacity and the count to
/// keep; `RET 8`). Without a buffer it allocates one (slot 4) and sets the capacity (`+0xc`);
/// when the count equals the capacity it reallocates (slot `0xc` with the buffer and the new
/// capacity); otherwise it allocates a new buffer (slot 4), copies the count over
/// (`006dc640(this, new, old, count)`), frees the old one (`006a8500`) and keeps the new.
pub fn fn_006f9eb0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(0x006d_c640, &args![base, buffer, old, count]);
        e.call(0x006a_8500, &args![base]);
        e.mem.set_u32(base + 4, buffer);
    }
}

// Translated from 006fa180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array constructor body (thiscall, two words: capacity and size; `RET 8`):
/// clears buffer, size and capacity; the capacity is raised to the size when below it; with a
/// capacity a buffer comes from slot 4 and the capacity is kept; with a size its entries are
/// constructed (`006f9ff0(this, buffer, size)`) and the size kept.
pub fn fn_006fa180(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let capacity = if size > capacity { size } else { capacity };
    if capacity > 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        e.call(0x006f_9ff0, &args![base, buffer, size]);
        e.mem.set_u32(base + 8, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_94a0, fn_006f94a0(Ptr)),
        entry!(0x006f99e0, allocate_item(Ptr) -> Ptr),
        entry!(0x006f_9080, fn_006f9080(Ptr)),
        entry!(0x006f_a100, fn_006fa100(Ptr) -> Ptr),
        entry!(
            0x006f_77a0,
            bgs_distant_tree_block_allocate_static_data(u32)
        ),
        entry!(0x006f_7a60, fn_006f7a60(u32)),
        entry!(0x006f_83f0, fn_006f83f0()),
        entry!(0x006f_86e0, fn_006f86e0(Ptr)),
        entry!(0x006f_8890, fn_006f8890(u32, u32, u32, u32, u32)),
        entry!(0x006f_9230, fn_006f9230(Ptr, u32, u32)),
        entry!(0x006f_92a0, fn_006f92a0(Ptr, u32)),
        entry!(0x006f_9360, fn_006f9360(Ptr, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x006f_9470,
            bgs_distant_tree_block_load_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f_94c0, bgs_distant_tree_block_load_task_queue_me(Ptr)),
        entry!(
            0x006f_94e0,
            bgs_distant_tree_block_load_task_check_finished(Ptr)
        ),
        entry!(
            0x006f_9540,
            bgs_distant_tree_block_load_task_cancel(Ptr, u32, u32)
        ),
        entry!(0x006f_9570, bgs_distant_tree_block_load_task_run(Ptr)),
        entry!(0x006f_9610, bgs_distant_tree_block_load_task_finish(Ptr)),
        entry!(0x006f_9680, fn_006f9680(Ptr, u32) -> Ptr),
        entry!(0x006f_96b0, fn_006f96b0(Ptr, u32) -> Ptr),
        entry!(0x006f_9790, fn_006f9790(Ptr, u32)),
        entry!(0x006f_9870, fn_006f9870(Ptr) -> Ptr),
        entry!(0x006f_98a0, fn_006f98a0(Ptr)),
        entry!(0x006f_98c0, fn_006f98c0(Ptr, u32) -> Ptr),
        entry!(0x006f_9940, fn_006f9940(Ptr)),
        entry!(0x006f_99a0, fn_006f99a0(Ptr)),
        entry!(0x006f_9a00, fn_006f9a00(Ptr, u32) -> Ptr),
        entry!(0x006f_9a80, bs_map_base_set_at(Ptr, u32, u32)),
        entry!(0x006f_9b80, fn_006f9b80(Ptr)),
        entry!(0x006f_9be0, fn_006f9be0(Ptr)),
        entry!(0x006f_9c70, fn_006f9c70(Ptr, u32)),
        entry!(0x006f_9eb0, fn_006f9eb0(Ptr, u32, u32)),
        entry!(0x006f_a180, fn_006fa180(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006f9080_passes_this_plus_1c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_9080, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x1c]);
    }

    #[test]
    fn fn_006f94a0_stores_the_vtable_then_forwards_this() {
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
        e.call(0x006f_94a0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_ded0);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_ded0,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_006FA180, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_a100, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_006FA180),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_006FA100);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    /// An object whose vtable has `slots` entries, the given ones pointing at the given doubles.
    fn object_with(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let count = slots
            .iter()
            .map(|(slot, _)| slot / 4 + 1)
            .max()
            .unwrap_or(1);
        let mut table = vec![0u32; count as usize];
        for (slot, target) in slots {
            table[(slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn b046_006f77a0_loads_the_static_data() {
        let mut e = Engine::new();
        for global in [
            SECURITY_COOKIE_011C16BC,
            0x011d_ea10,
            STATIC_LOADED_011D85A0,
            STATIC_POINTER_011D85BC,
            STATIC_POINTER_011D85C4,
            STATIC_TEXTURE_011D85E0,
            STATIC_POINTER_012007C4,
        ] {
            e.map(global, 4);
        }
        e.set_global(SECURITY_COOKIE_011C16BC, 0x00c0_0c1eu32);
        e.set_global(0x011d_ea10, 0x0ade_0000u32);
        stub(
            &mut e,
            &[
                FORMAT_00406D00,
                0x006f_8f80,
                0x0045_68c0,
                0x005b_5e40,
                POINTER_SET_0066B0D0,
                0x0049_ee30,
                0x0043_93c0,
                0x0049_ed90,
                0x0094_db80,
                0x0043_93e0,
                SECURITY_CHECK_COOKIE,
                0x0330_1000,
                0x0330_1130,
            ],
        );
        let world = object_with(&mut e, 0x0330_0000, &[(0x130, 0x0330_1130)]);
        e.register(0x0330_1130, |_, _| 0x0aa0_0000u32.into_ret());
        let file = object_with(&mut e, 0x0331_0000, &[(0, 0x0330_1000)]);
        e.register_double(0x00af_df00, move |_, _| file.into_ret());
        e.register(0x00ec_8a19, |_, _| 0u32.into_ret());
        e.register(POINTER_GET_00559450, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x00aa_13e0, |_, a| (a[0] << 8).into_ret());
        e.register(0x0049_ec80, |_, a| (a[0] + 1).into_ret());
        e.register(0x0043_91c0, |_, a| (a[0] + 2).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_77a0, &args![world]);
        let formats = calls_to(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[0][1..], [0x104, 0x0106_de08, 0x0aa0_0000]);
        assert_eq!(formats[1][1..], [0x104, 0x0106_ddb8, 0x0106_dddc]);
        assert_eq!(
            calls_to(&e, 0x00af_df00),
            vec![vec![formats[0][0], 0, 0x800]]
        );
        assert_eq!(calls_to(&e, 0x006f_8f80), vec![vec![file]]);
        assert_eq!(calls_to(&e, 0x0330_1000), vec![vec![file, 1]]);
        assert_eq!(
            calls_to(&e, 0x00ec_8a19),
            vec![vec![0x0aa0_0000, 0x0106_ddf0, 4]]
        );
        assert_eq!(
            calls_to(&e, 0x0045_68c0),
            vec![vec![
                0x0ade_0000,
                formats[1][0],
                STATIC_TEXTURE_011D85E0,
                1,
                1
            ]]
        );
        assert!(calls_to(&e, 0x005b_5e40).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![
                vec![STATIC_POINTER_012007C4, STATIC_TEXTURE_011D85E0 + 0x100],
                vec![STATIC_POINTER_011D85BC, 0x2401],
                vec![STATIC_POINTER_011D85C4, 0x1c02]
            ]
        );
        assert_eq!(e.mem.u8(STATIC_LOADED_011D85A0), 1);
        let held = STATIC_POINTER_011D85C4 + 0x100;
        assert_eq!(
            calls_to(&e, 0x0049_ee30),
            vec![vec![STATIC_POINTER_011D85BC + 0x100, 3]]
        );
        assert_eq!(calls_to(&e, 0x0043_93c0), vec![vec![held, 1]]);
        assert_eq!(calls_to(&e, 0x0049_ed90), vec![vec![held, 0]]);
        assert_eq!(calls_to(&e, 0x0094_db80), vec![vec![held, 0x80]]);
        assert_eq!(calls_to(&e, 0x0043_93e0), vec![vec![held, 4]]);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE), vec![vec![0x00c0_0c1e]]);
        // No file, another worldspace, no texture, failed allocations.
        e.register(0x00af_df00, |_, _| 0u32.into_ret());
        e.register(0x00ec_8a19, |_, _| 1u32.into_ret());
        e.register(POINTER_GET_00559450, |_, _| 0u32.into_ret());
        e.register(0x00aa_13e0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_77a0, &args![world]);
        assert!(calls_to(&e, 0x006f_8f80).is_empty());
        assert_eq!(calls_to(&e, FORMAT_00406D00)[1][3], 0x0106_ddf8);
        assert_eq!(calls_to(&e, 0x005b_5e40), vec![vec![0x0106_dd94]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0)[1..],
            [
                vec![STATIC_POINTER_011D85BC, 0],
                vec![STATIC_POINTER_011D85C4, 0]
            ]
        );
    }

    #[test]
    fn b046_006f83f0_and_006f7a60_release_the_static_data() {
        let mut e = Engine::new();
        e.map(STATIC_LOADED_011D85A0, 4);
        e.mem.set_u8(STATIC_LOADED_011D85A0, 1);
        stub(&mut e, &[POINTER_SET_0066B0D0]);
        e.register(ARRAY_COUNT_0044DDC0, |_, _| 2u32.into_ret());
        e.register(0x006b_3120, |_, a| (0x1000 * (a[1] + 1)).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_83f0, &args![]);
        assert_eq!(
            calls_to(&e, 0x006b_3120),
            vec![
                vec![STATIC_ARRAY_011D85D0, 0],
                vec![STATIC_ARRAY_011D85D0, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![
                vec![0x101c, 0],
                vec![0x201c, 0],
                vec![STATIC_POINTER_011D85BC, 0],
                vec![STATIC_POINTER_011D85C4, 0],
                vec![STATIC_TEXTURE_011D85E0, 0],
                vec![STATIC_POINTER_012007C4, 0]
            ]
        );
        assert_eq!(e.mem.u8(STATIC_LOADED_011D85A0), 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_7a60, &args![0x77u32]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![STATIC_POINTER_012007C4, 0x77]]
        );
    }

    #[test]
    fn b046_006f86e0_detaches_and_deletes_the_groups() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                ARRAY_FREE_008454F0,
                POINTER_SET_0066B0D0,
                0x006f_8800,
                0x0332_10e8,
            ],
        );
        let groups = [e.mem.alloc(0x20), 0, e.mem.alloc(0x20)];
        let entries = e.mem.alloc(0xc);
        for (i, group) in groups.iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, *group);
        }
        e.register(ARRAY_COUNT_0044DDC0, |_, _| 3u32.into_ret());
        e.register_double(ARRAY_AT_00877A30, move |_, a| {
            (entries + 4 * a[1]).into_ret()
        });
        // Only the first group has geometry.
        let geometry_of = groups[0] + 4;
        e.register_double(POINTER_GET_00559450, move |_, a| {
            match a[0] {
                at if at == geometry_of => 0x6600u32,
                0x7700 => 0x7710,
                _ => 0,
            }
            .into_ret()
        });
        let parent = object_with(&mut e, 0x0332_0000, &[(0xe8, 0x0332_10e8)]);
        e.register(0x0080_41a0, |_, _| 0x0000_7700u32.into_ret());
        e.register_double(0x004e_a9e0, move |_, _| parent.into_ret());
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x006f_86e0, &args![this]);
        assert_eq!(
            calls_to(&e, ARRAY_FREE_008454F0),
            vec![
                vec![groups[0] + 8, 1],
                vec![8, 1],
                vec![groups[2] + 8, 1],
                vec![this + 4, 1]
            ]
        );
        assert_eq!(calls_to(&e, 0x0080_41a0), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x004e_a9e0), vec![vec![0x7710]]);
        assert_eq!(calls_to(&e, 0x0332_10e8), vec![vec![parent, 0x6600]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![groups[0] + 4, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x006f_8800),
            vec![vec![groups[0], 1], vec![groups[2], 1]]
        );
    }

    #[test]
    fn b046_006f8890_formats_the_block_path() {
        let mut e = Engine::new();
        e.register(FORMAT_00406D00, quiet);
        let world = object_with(&mut e, 0x0333_0000, &[(0x130, 0x0333_1130)]);
        e.register(0x0333_1130, |_, _| 0x0bb0_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x006f_8890,
            &args![0x9000u32, world, 4u32, 0x1_fffeu32, 0x0003u32],
        );
        assert_eq!(
            calls_to(&e, FORMAT_00406D00),
            vec![vec![
                0x9000,
                0x104,
                0x0106_de90,
                0x0bb0_0000,
                0x0bb0_0000,
                4,
                0xffff_fffe,
                3
            ]]
        );
    }

    fn block_world(e: &mut Engine, found: bool) -> (u32, u32, u32) {
        let instance = e.mem.alloc(0x20);
        e.mem.set_u32(instance + 0x18, 2);
        e.mem.set_u8(instance + 0x1c, 0xcc);
        let group = e.mem.alloc(0x20);
        e.mem.set_u8(group + 0x1c, 1);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(entries + 8, group);
        e.register_double(0x0085_3130, move |e, a| {
            if found {
                e.mem.set_u32(a[2], instance);
            }
            (found as u32 | 0x100).into_ret()
        });
        e.register_double(ARRAY_AT_00877A30, move |_, a| {
            (entries + 4 * a[1]).into_ret()
        });
        e.register(0x007c_b2e0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x55);
            Ret::default()
        });
        (e.mem.alloc(0x4c), instance, group)
    }

    #[test]
    fn b046_006f9230_and_006f92a0_hide_show_or_queue() {
        let mut e = Engine::new();
        let (this, instance, group) = block_world(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x006f_9230, &args![this, 0x55u32, 1u32]);
        let lookup = calls_to(&e, 0x0085_3130);
        assert_eq!(lookup[0][..2], [this + 0x14, 0x55]);
        assert_eq!(e.mem.u8(instance + 0x1c), 1);
        assert_eq!(e.mem.u8(group + 0x1c), 0);
        assert_eq!(calls_to(&e, ARRAY_AT_00877A30), vec![vec![this + 4, 2]]);
        assert!(calls_to(&e, 0x007c_b2e0).is_empty());
        e.mem.set_u8(group + 0x1c, 1);
        e.call(0x006f_92a0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u8(instance + 0x1c), 0);
        assert_eq!(e.mem.u8(group + 0x1c), 0);
        let mut e = Engine::new();
        let (this, instance, _) = block_world(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x006f_9230, &args![this, 0x55u32, 0x100u32]);
        assert!(calls_to(&e, 0x007c_b2e0).is_empty());
        e.call(0x006f_9230, &args![this, 0x55u32, 1u32]);
        assert_eq!(calls_to(&e, 0x007c_b2e0).len(), 1);
        assert_eq!(calls_to(&e, 0x007c_b2e0)[0][0], this + 0x34);
        e.call(0x006f_92a0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u8(instance + 0x1c), 0xcc);
    }

    #[test]
    fn b046_006f9360_constructs_the_load_task() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.set_global(SECURITY_COOKIE_011C16BC, 0x00c0_0c1eu32);
        stub(
            &mut e,
            &[
                0x00c3_ce60,
                0x00c3_cee0,
                0x00c3_cf60,
                SECURITY_CHECK_COOKIE,
                FORMAT_00406D00,
            ],
        );
        let world = object_with(&mut e, 0x0334_0000, &[(0x130, 0x0334_1130)]);
        e.register(0x0334_1130, |_, _| 0x0cc0_0000u32.into_ret());
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcc; 0x48]);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0xb10cu32,
            world,
            0x1_0005u32,
            0x2_0006u32,
            3u32,
            0x99u32
        ];
        assert_eq!(e.call(0x006f_9360, &words).u32(), this);
        assert_eq!(calls_to(&e, 0x00c3_ce60), vec![vec![this, 3]]);
        assert_eq!(e.mem.u32(this), VTABLE_LOAD_TASK_0106DED0);
        assert_eq!(e.mem.u32(this + 0x30), 0xb10c);
        assert_eq!(e.mem.u8(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x1_0005);
        assert_eq!(e.mem.u32(this + 0x3c), 0x2_0006);
        assert_eq!(e.mem.u32(this + 0x40), 3);
        assert_eq!(e.mem.u8(this + 0x44), 0);
        assert_eq!(e.mem.u8(this + 0x45), 0);
        let format = calls_to(&e, FORMAT_00406D00);
        assert_eq!(format[0][5..], [3, 5, 6]);
        assert_eq!(
            calls_to(&e, 0x00c3_cee0),
            vec![vec![this, format[0][0] + 5]]
        );
        assert_eq!(calls_to(&e, 0x00c3_cf60), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE), vec![vec![0x00c0_0c1e]]);
    }

    #[test]
    fn b046_load_task_virtuals() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                0x00c3_cea0,
                OPERATOR_DELETE,
                0x0335_1028,
                0x00c3_c930,
                0x00c3_cb70,
                0x0336_1048,
                0x006f_8b40,
                0x0337_1000,
                0x006f_9300,
                0x006f_88e0,
            ],
        );
        let this = object_with(&mut e, 0x0335_0000, &[(0x28, 0x0335_1028)]);
        // 006f9470: the base destructor (`006f94a0`) and the delete.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9470, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x00c3_cea0), vec![vec![this]]);
        e.call(0x006f_9470, &args![this, 0u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
        // 006f94c0: QueueMe.
        let this = object_with(&mut e, 0x0335_0000, &[(0x28, 0x0335_1028)]);
        e.call_log = Some(vec![]);
        e.call(0x006f_94c0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x34), 1);
        assert_eq!(calls_to(&e, 0x0335_1028), vec![vec![this]]);
        // 006f94e0: CheckFinished, by `0043caa0`, the flag and `0069b080`.
        e.map(0x0120_2d98, 4);
        let queue = object_with(&mut e, 0x0336_0000, &[(0x48, 0x0336_1048)]);
        e.set_global(0x0120_2d98, queue);
        e.register(0x0043_caa0, |_, _| 1u32.into_ret());
        e.register(0x0069_b080, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert_eq!(calls_to(&e, 0x0336_1048), vec![vec![queue, this]]);
        e.register(0x0069_b080, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert_eq!(calls_to(&e, 0x00c3_c930), vec![vec![this]]);
        e.mem.set_u8(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert!(calls_to(&e, 0x00c3_c930).is_empty());
        // 006f9540: Cancel.
        e.call_log = Some(vec![]);
        e.call(0x006f_9540, &args![this, 7u32, 8u32]);
        assert_eq!(e.mem.u8(this + 0x44), 1);
        assert_eq!(calls_to(&e, 0x00c3_cb70), vec![vec![this, 7, 8]]);
        // 006f9570: Run, canceled then not; 006f9610: Finish.
        let file = object_with(&mut e, 0x0337_0000, &[(0, 0x0337_1000)]);
        e.register(0x0045_cd60, |_, _| 0x0dd0_0000u32.into_ret());
        e.register_double(0x00af_df20, move |_, _| file.into_ret());
        e.mem.set_u32(this + 0x30, 0xb10c);
        e.call_log = Some(vec![]);
        e.call(0x006f_9570, &args![this]);
        assert_eq!(
            calls_to(&e, 0x00af_df20),
            vec![vec![0x0dd0_0000, 0, 0x800, 1]]
        );
        assert!(calls_to(&e, 0x006f_8b40).is_empty());
        assert_eq!(e.mem.u8(this + 0x45), 0);
        e.call(0x006f_9610, &args![this]);
        assert!(calls_to(&e, 0x006f_88e0).is_empty());
        e.mem.set_u8(this + 0x44, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9570, &args![this]);
        assert_eq!(calls_to(&e, 0x006f_8b40), vec![vec![0xb10c, file]]);
        assert_eq!(calls_to(&e, 0x0337_1000), vec![vec![file, 1]]);
        assert_eq!(calls_to(&e, 0x006f_9300), vec![vec![0xb10c]]);
        assert_eq!(e.mem.u8(this + 0x45), 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_9610, &args![this]);
        assert_eq!(calls_to(&e, 0x006f_88e0), vec![vec![0xb10c]]);
        e.register(0x00af_df20, |_, _| 0u32.into_ret());
        e.call(0x006f_9570, &args![this]);
        assert_eq!(e.mem.u8(this + 0x45), 0);
    }

    #[test]
    fn b046_map_constructors_and_destructors() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x4000 + a[0]).into_ret());
        stub(&mut e, &[0x00ec_61c0, 0x0043_8af0, OPERATOR_DELETE]);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9680, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_0106DF38);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 8), 0x4094);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, 0x00ec_61c0), vec![vec![0x4094, 0, 0x94]]);
        e.mem.set_u32(this + 8, 0xcccc_cccc);
        assert_eq!(e.call(0x006f_96b0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_0106DF58);
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
        assert_eq!(e.call(0x006f_98c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_BASE_0106DFA0);
        assert_eq!(e.call(0x006f_9a00, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_BASE_0106DFC0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9940, &args![this]);
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![this]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x4004]]);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_BASE_0106DFA0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9b80, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_BASE_0106DFC0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_99a0, &args![this]);
        e.call(0x006f_9be0, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn b046_006f9a80_sets_or_adds_the_item() {
        let mut e = Engine::new();
        let this = object_with(
            &mut e,
            0x0338_0000,
            &[
                (4, 0x0338_1004),
                (8, 0x0338_1008),
                (0xc, 0x0338_100c),
                (0x10, 0x0338_1010),
                (0x14, 0x0338_1014),
            ],
        );
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, table);
        e.register(0x0338_1004, |_, a| (a[1] & 3).into_ret());
        e.register(0x0338_1008, |_, a| {
            ((a[1] == a[2]) as u32 | 0x100).into_ret()
        });
        e.register(0x0338_100c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x0338_1010, quiet);
        e.register(0x0338_1014, |e, _| e.mem.alloc(0xc).into_ret());
        e.call(0x006f_9a80, &args![this, 5u32, 0x50u32]);
        e.call(0x006f_9a80, &args![this, 9u32, 0x90u32]);
        let first = e.mem.u32(table + 4);
        assert_eq!(e.mem.u32(first + 4), 9);
        assert_eq!(e.mem.u32(e.mem.u32(first) + 4), 5);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        e.call_log = Some(vec![]);
        e.call(0x006f_9a80, &args![this, 5u32, 0x55u32]);
        let second = e.mem.u32(first);
        assert_eq!(calls_to(&e, 0x0338_1010), vec![vec![this, second]]);
        assert_eq!(e.mem.u32(second + 8), 0x55);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert!(calls_to(&e, 0x0338_1014).is_empty());
    }

    #[test]
    fn b046_array_resize_clear_and_construct() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                0x0072_ba80,
                0x006d_c640,
                0x006a_8500,
                0x006f_a090,
                0x006f_9ff0,
            ],
        );
        let this = object_with(&mut e, 0x0339_0000, &[(4, 0x0339_1004), (0xc, 0x0339_100c)]);
        e.register(0x0339_1004, |_, a| (0x7000 + a[1]).into_ret());
        e.register(0x0339_100c, |_, a| (a[1] + 0x10).into_ret());
        // 006fa180: capacity 2 raised to the size 3.
        e.call_log = Some(vec![]);
        e.call(0x006f_a180, &args![this, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7003);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(calls_to(&e, 0x006f_9ff0), vec![vec![this, 0x7003, 3]]);
        e.call(0x006f_a180, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // 006f9eb0: no buffer; same count as capacity (reallocate); otherwise copy.
        e.call(0x006f_9eb0, &args![this, 4u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        e.call(0x006f_9eb0, &args![this, 6u32, 4u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7014);
        e.call_log = Some(vec![]);
        e.call(0x006f_9eb0, &args![this, 8u32, 2u32]);
        assert_eq!(
            calls_to(&e, 0x006d_c640),
            vec![vec![this, 0x7008, 0x7014, 2]]
        );
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x7008);
        // 006f9790: shrink below the size, then the same capacity does nothing.
        e.mem.set_u32(this + 8, 5);
        e.mem.set_u32(this + 0xc, 4);
        e.call_log = Some(vec![]);
        e.call(0x006f_9790, &args![this, 2u32]);
        assert_eq!(
            calls_to(&e, 0x0072_ba80),
            vec![vec![this, 0x7008 + 0x40, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        e.call_log = Some(vec![]);
        e.call(0x006f_9790, &args![this, 2u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // 006f9c70: clear without and with freeing.
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x006f_a090), vec![vec![this, 0x7002, 2]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 4), 0x7002);
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 1u32]);
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 1u32]);
        assert!(calls_to(&e, 0x006f_a090).is_empty());
    }

    #[test]
    fn b046_006f9870_and_006f98a0_group_array() {
        let mut e = Engine::new();
        stub(&mut e, &[0x006b_3eb0, ARRAY_FREE_008454F0]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9870, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_ARRAY_0106DF8C);
        assert_eq!(calls_to(&e, 0x006b_3eb0), vec![vec![this, 0, 0]]);
        e.mem.set_u32(this, 0);
        e.call(0x006f_98a0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_ARRAY_0106DF8C);
        assert_eq!(calls_to(&e, ARRAY_FREE_008454F0), vec![vec![this, 1]]);
    }
}
