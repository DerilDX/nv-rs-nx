//! `NiXenonRenderer/run_00e8f190` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRefObject` virtual table and the global object count it counts up (`011f4400`), the
/// interlocked imports, the bucket allocation `00aa1070`, `memset`, the map set-up `00b601a0`
/// (unnamed; `thiscall`, no stack words) and the allocation `00aa13e0` (`cdecl`, size).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const MAP_00B601A0: u32 = 0x00b6_01a0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// The class virtual tables these constructors store (RTTI `NiDX9IndexBufferManager`, its
/// maps' `NiTMapBase<..., unsigned int, NiDX9IndexBufferManager::NiDX9IBInfo *>` and
/// `NiTPointerMap<...>`, `NiDX9TextureManager`, `NiDX9ErrorShader`, `NiDX9RenderState`).
const NI_DX9_INDEX_BUFFER_MANAGER_VTABLE: u32 = 0x010f_02f0;
const IB_MAP_BASE_VTABLE: u32 = 0x010f_02b0;
const IB_MAP_VTABLE: u32 = 0x010f_02d0;
const NI_DX9_TEXTURE_MANAGER_VTABLE: u32 = 0x010f_086c;
const NI_DX9_ERROR_SHADER_VTABLE: u32 = 0x010f_0764;
const NI_DX9_RENDER_STATE_VTABLE: u32 = 0x010f_088c;
/// The renderer's device word (+0x288).
const RENDERER_DEVICE: u32 = 0x288;

/// The `NiRefObject` start every one of these objects makes: its table, count 0, the global
/// object count up.
fn ref_object_start(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e8fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` constructor (the
/// RTTI of the virtual table it stores), `thiscall` with the device. The `NiRefObject` start,
/// the class's table, three maps at +0x1c, +0x2c, +0x3c (0x25 buckets each, cleared), the
/// device at +8 (counted: its slot 4), +0xc to +0x18 cleared, then `00b601a0` on each map.
/// Answers `this`.
pub fn fn_00e8fb20(e: &mut Engine, this: Ptr, device: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
    for offset in [0x1cu32, 0x2c, 0x3c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, IB_MAP_BASE_VTABLE);
        e.mem.set_u32(map.wrapping_add(4), 0x25);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, IB_MAP_VTABLE);
    }
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    for offset in [0xcu32, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in [0x1cu32, 0x2c, 0x3c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    this
}

// Translated from 00e90a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9TextureManager` constructor (the RTTI
/// of the virtual table it stores), `thiscall` with the renderer. The `NiRefObject` start, the
/// class's table, the renderer at +0xc and its device (+0x288) at +8, counted (its slot 4).
/// Answers `this`.
pub fn fn_00e90a80(e: &mut Engine, this: Ptr, renderer: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_TEXTURE_MANAGER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0xc), renderer);
    let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    this
}

/// The shader base constructor (`00e818a0`, unnamed; `thiscall`, no stack words), the error
/// shader's set-up (`00e90850`, unnamed; `thiscall`, no stack words), the fixed-string maker
/// `00a5b690` (`cdecl`, a C string), the name setter `00a5b950` (unnamed; `thiscall`, the
/// address of a fixed string), the empty fixed string (`0109b220`) and the name
/// `"NiDX9ErrorShader"` (`010f0854`).
const SHADER_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const ERROR_SHADER_SET_UP_00E90850: u32 = 0x00e9_0850;
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
const ERROR_SHADER_NAME_010F0854: u32 = 0x010f_0854;

// Translated from 00e909e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The shader base constructor, the
/// bytes +0x24 and +0x25 and the words +0x28 to +0x38 cleared, the class's table, `00e90850`,
/// the name `"NiDX9ErrorShader"` made as a fixed string (in a stack word), set (`00a5b950`) and
/// released (its count at -8 down unless it is the empty string), then +0x20 set to 1.
/// Answers `this`.
pub fn fn_00e909e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base.wrapping_add(0x24), 0);
    e.mem.set_u8(base.wrapping_add(0x25), 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base, NI_DX9_ERROR_SHADER_VTABLE);
    e.call(ERROR_SHADER_SET_UP_00E90850, &args![base]);
    e.with_stack(4, |e, name| {
        let handle = e
            .call(FIXED_STRING_MAKE, &args![ERROR_SHADER_NAME_010F0854])
            .u32();
        e.mem.set_u32(name.addr(), handle);
        e.call(SET_NAME_00A5B950, &args![base, name]);
        let handle = e.mem.u32(name.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    e.mem.set_u8(base.wrapping_add(0x20), 1);
    this
}

/// The render state's base constructor (`00e881a0`, unnamed; `thiscall`, the renderer),
/// `memcpy` (`00ec44d0`), the table of (state, value) word pairs it adjusts (`011bfce8`, ended
/// by state -1, 256 pairs at most), and the object `00e978a0` makes (unnamed; `cdecl`:
/// renderer, caps).
const RENDER_STATE_CONSTRUCT_00E881A0: u32 = 0x00e8_81a0;
const MEMCPY: u32 = 0x00ec_44d0;
const STATE_PAIRS_011BFCE8: u32 = 0x011b_fce8;
const MAKE_00E978A0: u32 = 0x00e9_78a0;

// Translated from 00e91590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): makes the `NiDX9RenderState` (0x1248 bytes; the
/// RTTI of the virtual table it stores), `cdecl`: the renderer, its caps and a byte. A made
/// object gets the base constructor (`00e881a0` with the renderer), its table, the byte
/// +0x1114 cleared and the 0x130 caps bytes copied to +0x1118; null when the allocation fails
/// (its slot 0x104 still runs). Then its virtual slot 0x104; in the pairs at `011bfce8` (until
/// state -1, at most 256), state 7's value becomes 1 when the byte is set, else 0; with the
/// byte set +8 gets bit 1. The object `00e978a0` makes is held at +0x10f0 (`NiPointer`
/// assignment: the old one released, the new one counted). Answers the render state.
pub fn fn_00e91590(e: &mut Engine, renderer: u32, caps: u32, flag: u8) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1248u32]).u32();
    let state = if block != 0 {
        e.call(RENDER_STATE_CONSTRUCT_00E881A0, &args![block, renderer]);
        e.mem.set_u32(block, NI_DX9_RENDER_STATE_VTABLE);
        e.mem.set_u8(block + 0x1114, 0);
        e.call(MEMCPY, &args![block + 0x1118, caps, 0x130u32]);
        block
    } else {
        0
    };
    e.vcall(state, 0x104, &args![]);
    for pair in 0..0x100u32 {
        let at = STATE_PAIRS_011BFCE8 + 8 * pair;
        let id = e.mem.u32(at);
        if id == 0xffff_ffff {
            break;
        }
        if id == 7 {
            e.mem.set_u32(at + 4, u32::from(flag != 0));
        }
    }
    if flag != 0 {
        let flags = e.mem.u32(state.wrapping_add(8)) | 2;
        e.mem.set_u32(state.wrapping_add(8), flags);
    }
    let made = e.call(MAKE_00E978A0, &args![renderer, caps]).u32();
    let slot = state.wrapping_add(0x10f0);
    let held = e.mem.u32(slot);
    if held != made {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(slot, made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    state
}

/// What the error shader's set-up calls: the pass maker `00e804d0` and the render state
/// group maker `00e7edf0` (unnamed; `cdecl`, the address of the word they fill), the state
/// setters `00e88fc0` (three words) and `00e890c0` (four words) (unnamed; `thiscall` on the
/// group's +0x10), the pass attachment `00e802b0` (unnamed; `thiscall` on the pass: its +0x14
/// and the group), the pass releases `00e7ecd0` and `00e80190` (unnamed; `thiscall`) and the
/// global the pass is kept in (`0126f74c`).
const MAKE_PASS_00E804D0: u32 = 0x00e8_04d0;
const MAKE_GROUP_00E7EDF0: u32 = 0x00e7_edf0;
const SET_STATE_00E88FC0: u32 = 0x00e8_8fc0;
const SET_SAMPLER_STATE_00E890C0: u32 = 0x00e8_90c0;
const ATTACH_00E802B0: u32 = 0x00e8_02b0;
const RELEASE_GROUP_00E7ECD0: u32 = 0x00e7_ecd0;
const RELEASE_PASS_00E80190: u32 = 0x00e8_0190;
const ERROR_PASS_0126F74C: u32 = 0x0126_f74c;

// Translated from 00e90850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the error shader's set-up, `thiscall`, no
/// stack words; answers 1. Makes a pass (`00e804d0`, kept at +0x3c) and a state group
/// (`00e7edf0`; +4 and +8 cleared), sets its states (`00e88fc0` with (1, 2, 0), (2, 3, 0), (4,
/// 2, 0), (5, 3, 0), (0xb, 0, 0); `00e890c0` with (1, 3, 0, 0), (2, 3, 0, 0), (5, 2, 0, 0), (6,
/// 2, 0, 0), (7, 0, 0, 0); the byte +0x5e cleared; `00e88fc0` with (0x18, 0, 0)), attaches the
/// group to the pass (`00e802b0` with the pass's +0x14), keeps the pass at `0126f74c`, then
/// drops the local references: the group's count (+0x60) and the pass's count (+0x64) down,
/// each released (`00e7ecd0`, `00e80190`) at zero.
pub fn fn_00e90850(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let pass_slot = locals.addr();
        let group_slot = locals.addr() + 4;
        e.mem.set_u32(pass_slot, 0);
        e.call(MAKE_PASS_00E804D0, &args![pass_slot]);
        let pass = e.mem.u32(pass_slot);
        e.mem.set_u32(base + 0x3c, pass);
        e.mem.set_u32(group_slot, 0);
        e.call(MAKE_GROUP_00E7EDF0, &args![group_slot]);
        let group = e.mem.u32(group_slot);
        e.mem.set_u32(group + 4, 0);
        e.mem.set_u32(group + 8, 0);
        let states = |e: &mut Engine, words: [u32; 3]| {
            let target = e.mem.u32(group + 0x10);
            e.call(
                SET_STATE_00E88FC0,
                &args![target, words[0], words[1], words[2]],
            );
        };
        let samplers = |e: &mut Engine, words: [u32; 4]| {
            let target = e.mem.u32(group + 0x10);
            e.call(
                SET_SAMPLER_STATE_00E890C0,
                &args![target, words[0], words[1], words[2], words[3]],
            );
        };
        states(e, [1, 2, 0]);
        states(e, [2, 3, 0]);
        states(e, [4, 2, 0]);
        states(e, [5, 3, 0]);
        states(e, [0xb, 0, 0]);
        samplers(e, [1, 3, 0, 0]);
        samplers(e, [2, 3, 0, 0]);
        samplers(e, [5, 2, 0, 0]);
        samplers(e, [6, 2, 0, 0]);
        samplers(e, [7, 0, 0, 0]);
        e.mem.set_u8(group + 0x5e, 0);
        states(e, [0x18, 0, 0]);
        let pass = e.mem.u32(pass_slot);
        let stage = e.mem.u32(pass.wrapping_add(0x14));
        e.call(ATTACH_00E802B0, &args![pass, stage, group]);
        let group = e.mem.u32(group_slot);
        let pass = e.mem.u32(pass_slot);
        e.mem.set_u32(ERROR_PASS_0126F74C, pass);
        if group != 0 {
            let count = e.mem.u32(group + 0x60).wrapping_sub(1);
            e.mem.set_u32(group + 0x60, count);
            if count == 0 {
                e.call(RELEASE_GROUP_00E7ECD0, &args![group]);
            }
        }
        let pass = e.mem.u32(pass_slot);
        if pass != 0 {
            let count = e.mem.u32(pass + 0x64).wrapping_sub(1);
            e.mem.set_u32(pass + 0x64, count);
            if count == 0 {
                e.call(RELEASE_PASS_00E80190, &args![pass]);
            }
        }
    });
    1
}

/// What the index buffer manager uses: the report `00a29680` (`cdecl`) and its messages
/// (`"NiDX9IndexBufferManager::CreateIndexBuffer FAILED"` at `010f0184`,
/// `"NiDX9IndexBufferManager::PackBuffer> Failed to create index buffer\n"` at `010f01b8`,
/// `"...> Failed to lock index buffer\n"` at `010eddd0`), `IDirect3DDevice9::CreateIndexBuffer`
/// (slot 0x6c) and `SetIndices` (0x1a0), `IDirect3DIndexBuffer9::Lock` (0x2c), `Unlock`
/// (0x30) and `GetDesc` (0x34) (slot names by the interfaces' order; the arguments match).
const REPORT_00A29680: u32 = 0x00a2_9680;
const CREATE_INDEX_BUFFER_FAILED_010F0184: u32 = 0x010f_0184;
const PACK_CREATE_FAILED_010F01B8: u32 = 0x010f_01b8;
const PACK_LOCK_FAILED_010EDDD0: u32 = 0x010e_ddd0;
const CREATE_INDEX_BUFFER: u32 = 0x6c;
const SET_INDICES: u32 = 0x1a0;
const INDEX_BUFFER_LOCK: u32 = 0x2c;
const INDEX_BUFFER_UNLOCK: u32 = 0x30;
const INDEX_BUFFER_GET_DESC: u32 = 0x34;
/// `D3DFMT_INDEX16` (0x65) and `D3DRTYPE_INDEXBUFFER` (7), the values the code compares.
const FORMAT_INDEX16: u32 = 0x65;
const RESOURCE_INDEX_BUFFER: u32 = 7;

// Translated from 00e8f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager`'s index buffer maker
/// (the name its report gives), `thiscall` with five words: length, usage, format, pool and
/// the address of a description (may be null). The device (+8) creates the buffer into a
/// stack word cleared first (`CreateIndexBuffer`, no shared handle); a failure is reported
/// and null answered. With a description address the buffer's `GetDesc` fills it. Answers the
/// buffer.
pub fn fn_00e8f190(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
    desc: u32,
) -> u32 {
    let device = e.mem.u32(this.addr().wrapping_add(8));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let result = e
            .vcall(
                device,
                CREATE_INDEX_BUFFER,
                &args![length, usage, format, pool, slot, 0u32],
            )
            .i32();
        if result < 0 {
            e.call(REPORT_00A29680, &args![CREATE_INDEX_BUFFER_FAILED_010F0184]);
            return 0;
        }
        let buffer = e.mem.u32(slot.addr());
        if desc != 0 {
            e.vcall(buffer, INDEX_BUFFER_GET_DESC, &args![desc]);
        }
        e.mem.u32(slot.addr())
    })
}

// Translated from 00e8f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager::PackBuffer` (the
/// name its reports give), `thiscall` with seven words: the 16-bit indices, their count, the
/// count the buffer must hold, a buffer that may be reused, the address of the length word,
/// the pool and the usage. Null without a device (+8) or indices. The buffer is reused when
/// its description (`GetDesc` into a cleared stack block) succeeds and says format
/// `D3DFMT_INDEX16`, type index buffer, the usage and pool asked for and a size of at least
/// twice the count to hold; otherwise a new one is created (twice that count, the usage,
/// `D3DFMT_INDEX16`, the pool; into the reuse argument's stack word, cleared first): a failed
/// creation is reported, then a missing buffer is reported and null answered; a new buffer's
/// length goes to the length word. With indices to copy (twice their count in bytes, nonzero),
/// the buffer is locked (offset 0, into the pool argument's stack word, cleared first); the
/// indices are copied in (`memcpy`) and the buffer unlocked; a failed lock is reported and the
/// buffer unlocked and released (its slot 8). Answers the buffer (also a released one).
pub fn fn_00e8f200(
    e: &mut Engine,
    this: Ptr,
    indices: u32,
    count: u32,
    capacity: u32,
    reuse: u32,
    length_out: Ptr,
    pool: u32,
    usage: u32,
) -> u32 {
    let device = e.mem.u32(this.addr().wrapping_add(8));
    if device == 0 || indices == 0 {
        return 0;
    }
    let length = capacity.wrapping_add(capacity);
    let bytes = count.wrapping_add(count);
    let reused = reuse != 0
        && e.with_stack(0x14, |e, desc| {
            let desc = desc.addr();
            if e.vcall(reuse, INDEX_BUFFER_GET_DESC, &args![desc]).i32() < 0 {
                return false;
            }
            e.mem.u32(desc) == FORMAT_INDEX16
                && e.mem.u32(desc + 4) == RESOURCE_INDEX_BUFFER
                && e.mem.u32(desc + 8) == usage
                && e.mem.u32(desc + 0xc) == pool
                && e.mem.u32(desc + 0x10) >= length
        });
    let buffer = if reused {
        reuse
    } else {
        let made = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let device = e.mem.u32(this.addr().wrapping_add(8));
            let result = e
                .vcall(
                    device,
                    CREATE_INDEX_BUFFER,
                    &args![length, usage, FORMAT_INDEX16, pool, slot, 0u32],
                )
                .i32();
            if result < 0 {
                e.call(REPORT_00A29680, &args![CREATE_INDEX_BUFFER_FAILED_010F0184]);
                return 0;
            }
            e.mem.u32(slot.addr())
        });
        if made == 0 {
            e.call(REPORT_00A29680, &args![PACK_CREATE_FAILED_010F01B8]);
            return 0;
        }
        e.mem.set_u32(length_out.addr(), length);
        made
    };
    if bytes != 0 {
        let locked = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let result = e
                .vcall(buffer, INDEX_BUFFER_LOCK, &args![0u32, bytes, slot, 0u32])
                .i32();
            if result < 0 {
                return false;
            }
            let data = e.mem.u32(slot.addr());
            e.call(MEMCPY, &args![data, indices, bytes]);
            true
        });
        if locked {
            e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
            return buffer;
        }
        e.call(REPORT_00A29680, &args![PACK_LOCK_FAILED_010EDDD0]);
        e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
        e.vcall(buffer, 8, &args![]);
    }
    buffer
}

/// The index buffer manager's three maps (+0x1c, +0x2c, +0x3c), the clean-up of an index
/// buffer entry (`00e95b80`, unnamed; `thiscall`, no stack words), the sized free `00aa1460`
/// (`cdecl`: block, size) and the free `00aa10f0` (`cdecl`, a block).
const INDEX_BUFFER_MAPS: [u32; 3] = [0x1c, 0x2c, 0x3c];
const ENTRY_CLEAN_UP_00E95B80: u32 = 0x00e9_5b80;
const FREE_00AA1460: u32 = 0x00aa_1460;
const FREE_00AA10F0: u32 = 0x00aa_10f0;

/// The first item of the first non-empty bucket at or after `bucket` in the map at `map`
/// (the bucket count and array read once), or null.
fn first_item_from(e: &mut Engine, map: u32, bucket: u32) -> u32 {
    let size = e.mem.u32(map.wrapping_add(4));
    if bucket >= size {
        return 0;
    }
    let buckets = e.mem.u32(map.wrapping_add(8));
    for index in bucket..size {
        let item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if item != 0 {
            return item;
        }
    }
    0
}

// Translated from 00e8f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` destructor (the
/// RTTI of the table it stores), `thiscall`. Its table; for each of its three maps (+0x1c,
/// +0x2c, +0x3c) every item is visited (the next one found first: the item's next, else the
/// first item of a later bucket after the map's hash of the key, virtual slot 4, plus one) and
/// its value, an index buffer information block, freed: each of its five entries (+8 to
/// +0x18) that is set has its buffer (+0x20) released (slot 8) when set, gets `00e95b80` and
/// its 0x38 bytes freed (`00aa1460`), then the block's 0x1c bytes are freed. Then each map is
/// emptied (`00b601a0`), the device's `SetIndices` gets null, the objects at +0xc and +0x14
/// are released (slot 8) when set, the device (+8) released and cleared, and the maps are
/// destroyed from the last (their two tables in turn, `00b601a0` with each, the bucket array
/// freed with `00aa10f0`); last the `NiRefObject` table and the global object count down.
pub fn fn_00e8f840(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
    for offset in INDEX_BUFFER_MAPS {
        let map = base.wrapping_add(offset);
        let mut item = first_item_from(e, map, 0);
        while item != 0 {
            let next = e.mem.u32(item);
            let info = e.mem.u32(item.wrapping_add(8));
            let following = if next != 0 {
                next
            } else {
                let key = e.mem.u32(item.wrapping_add(4));
                let bucket = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
                first_item_from(e, map, bucket)
            };
            if info != 0 {
                for slot in 0..5u32 {
                    let entry = e.mem.u32(info.wrapping_add(8 + 4 * slot));
                    if entry != 0 {
                        let held = e.mem.u32(entry.wrapping_add(0x20));
                        if held != 0 {
                            e.vcall(held, 8, &args![]);
                        }
                        e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                        e.call(FREE_00AA1460, &args![entry, 0x38u32]);
                    }
                }
                e.call(FREE_00AA1460, &args![info, 0x1cu32]);
            }
            item = following;
        }
    }
    for offset in INDEX_BUFFER_MAPS {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    let device = e.mem.u32(base.wrapping_add(8));
    e.vcall(device, SET_INDICES, &args![0u32]);
    for offset in [0xcu32, 0x14] {
        let held = e.mem.u32(base.wrapping_add(offset));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
    }
    let device = e.mem.u32(base.wrapping_add(8));
    if device != 0 {
        e.vcall(device, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(8), 0);
    }
    for offset in [0x3cu32, 0x2c, 0x1c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, IB_MAP_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        e.mem.set_u32(map, IB_MAP_BASE_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        let buckets = e.mem.u32(map.wrapping_add(8));
        e.call(FREE_00AA10F0, &args![buckets]);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

/// The index buffer maps' lookup (`00a53c30`, unnamed; `thiscall`: the key, the address of the
/// word it fills; answers a byte) and insertion (`00b57ea0`, unnamed; `thiscall`: the key, the
/// value).
const MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;
const MAP_SET_00B57EA0: u32 = 0x00b5_7ea0;

// Translated from 00e8fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the index buffer information block for a usage
/// and format, `thiscall` with three words: the usage, the format and the pool (0, 1, 2 pick
/// the map at +0x1c, +0x2c, +0x3c; any other pool answers null). The key is
/// `usage << 16 | format`; a block found in the map is answered; otherwise a new 0x1c-byte one
/// (`00aa13e0`; no null check) gets the key at +0, the byte +4 and the words +8 to +0x18
/// cleared, is put in the map (`00b57ea0`) and answered.
pub fn fn_00e8fa80(e: &mut Engine, this: Ptr, usage: u32, format: u32, pool: u32) -> u32 {
    let key = (usage << 16) | format;
    let offset = match pool {
        0 => 0x1c,
        1 => 0x2c,
        2 => 0x3c,
        _ => return 0,
    };
    let map = this.addr().wrapping_add(offset);
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if e.call(MAP_LOOKUP_00A53C30, &args![map, key, slot]).u8() != 0 {
            Some(e.mem.u32(slot.addr()))
        } else {
            None
        }
    });
    if let Some(block) = found {
        return block;
    }
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    e.mem.set_u32(block, 0);
    e.mem.set_u8(block.wrapping_add(4), 0);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(block.wrapping_add(offset), 0);
    }
    e.mem.set_u32(block, key);
    e.call(MAP_SET_00B57EA0, &args![map, key, block]);
    block
}

// Translated from 00e8fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` deleting
/// destructor, `thiscall` with the flags: the destructor ([`fn_00e8f840`]), then with bit 0
/// the 0x4c bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e8fc30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e8f840(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x4cu32]);
    }
    this
}

/// The index buffer entry constructor (`00e95b60`, unnamed; `thiscall`, no stack words).
const ENTRY_CONSTRUCT_00E95B60: u32 = 0x00e9_5b60;

// Translated from 00e8fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): an index buffer from the manager's rotation,
/// `thiscall` with four words: length, usage, format and pool. Null without a device (+8) or
/// an information block ([`fn_00e8fa80`] with usage, format, pool). The block's current entry
/// (the byte +4 indexes the five words from +8): none makes a new 0x38-byte entry (`00e95b60`;
/// null when the allocation fails, the stores still go to it; the entry is not stored in the
/// block) with the pool at +8, the usage at +0xc, 4 at +4, the length at +0x14, the format at
/// +0x18 and +0x1c, +0x20 cleared; an entry whose length (+0x14) is above the length asked
/// has its buffer (+0x20) released (slot 8; no null check) and cleared; otherwise its buffer
/// is the answer and the index moves on. An entry left without a buffer gets a new one ([`fn_00e8f190`]
/// with the entry's +0x24 as the description), its length, format and +0x1c cleared, the
/// index moves on and the buffer is the answer. The index wraps to 0 at 5.
pub fn fn_00e8fc50(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
) -> u32 {
    if e.mem.u32(this.addr().wrapping_add(8)) == 0 {
        return 0;
    }
    let mut answer = 0;
    let info = fn_00e8fa80(e, this, usage, format, pool);
    if info == 0 {
        return answer;
    }
    let index = e.mem.u8(info.wrapping_add(4));
    let mut entry = e.mem.u32(info.wrapping_add(8 + 4 * u32::from(index)));
    if entry != 0 {
        if e.mem.u32(entry.wrapping_add(0x14)) > length {
            let held = e.mem.u32(entry.wrapping_add(0x20));
            e.vcall(held, 8, &args![]);
            e.mem.set_u32(entry.wrapping_add(0x20), 0);
        } else {
            answer = e.mem.u32(entry.wrapping_add(0x20));
            e.mem.set_u8(info.wrapping_add(4), index.wrapping_add(1));
        }
    } else {
        let block = e.call(NEW_OBJECT, &args![0x38u32]).u32();
        entry = if block != 0 {
            e.call(ENTRY_CONSTRUCT_00E95B60, &args![block]);
            block
        } else {
            0
        };
        e.mem.set_u32(entry.wrapping_add(8), pool);
        e.mem.set_u32(entry.wrapping_add(0xc), usage);
        e.mem.set_u32(entry.wrapping_add(4), 4);
        e.mem.set_u32(entry.wrapping_add(0x14), length);
        e.mem.set_u32(entry.wrapping_add(0x18), format);
        e.mem.set_u32(entry.wrapping_add(0x1c), 0);
        e.mem.set_u32(entry.wrapping_add(0x20), 0);
    }
    if e.mem.u32(entry.wrapping_add(0x20)) == 0 {
        let buffer = fn_00e8f190(
            e,
            this,
            length,
            usage,
            format,
            pool,
            entry.wrapping_add(0x24),
        );
        e.mem.set_u32(entry.wrapping_add(0x20), buffer);
        e.mem.set_u32(entry.wrapping_add(0x14), length);
        e.mem.set_u32(entry.wrapping_add(0x18), format);
        e.mem.set_u32(entry.wrapping_add(0x1c), 0);
        let index = e.mem.u8(info.wrapping_add(4)).wrapping_add(1);
        e.mem.set_u8(info.wrapping_add(4), index);
        answer = buffer;
    }
    if e.mem.u8(info.wrapping_add(4)) == 5 {
        e.mem.set_u8(info.wrapping_add(4), 0);
    }
    answer
}

/// The `NiDX9RenderedTextureData` table (RTTI) and the base destructor `00e8a2b0` (unnamed;
/// `thiscall`, no stack words).
const NI_DX9_RENDERED_TEXTURE_DATA_VTABLE: u32 = 0x010f_02fc;
const TEXTURE_DATA_DOWN_00E8A2B0: u32 = 0x00e8_a2b0;

// Translated from 00e8fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderedTextureData` deleting
/// destructor (the RTTI of the table it stores), `thiscall` with the flags. Its table; the
/// renderer (+0x60) gets its slot 0x15c with the word at +8; the surface (+0x6c) is released
/// (slot 8) and cleared when set; the base destructor `00e8a2b0`; then with bit 0 the 0x70
/// bytes are freed (`00aa1460`). Answers `this`.
pub fn fn_00e8fdc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let word = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
    e.vcall(renderer, 0x15c, &args![word]);
    let surface = e.mem.u32(base.wrapping_add(0x6c));
    if surface != 0 {
        e.vcall(surface, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x6c), 0);
    }
    e.call(TEXTURE_DATA_DOWN_00E8A2B0, &args![base]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x70u32]);
    }
    this
}

/// What `00e8fe10` uses: the pixel format pick `00e89ed0` (unnamed; `cdecl`: the address of
/// three preference words, the renderer's +0x758), the byte at `0126f0d0`, the multisample
/// type of a sample kind (`00e7ccf0`, unnamed; `cdecl`), `NiXenonErrorString` (`00e89d50`,
/// `cdecl`), the message `"NiDX9RenderedTextureData::CreateSurf> Failed CreateImageSurface -
/// %s"` (`010f03b0`) and `IDirect3DDevice9::CreateRenderTarget` (slot 0x70) and
/// `CreateTexture` (0x5c) (by the interface's order; the arguments match).
const PICK_FORMAT_00E89ED0: u32 = 0x00e8_9ed0;
const POOL_FLAG_0126F0D0: u32 = 0x0126_f0d0;
const MULTISAMPLE_TYPE_00E7CCF0: u32 = 0x00e7_ccf0;
const ERROR_STRING_00E89D50: u32 = 0x00e8_9d50;
const CREATE_SURFACE_FAILED_010F03B0: u32 = 0x010f_03b0;
const CREATE_RENDER_TARGET: u32 = 0x70;
const CREATE_TEXTURE: u32 = 0x5c;

// Translated from 00e8fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedTextureData::CreateSurf` (the
/// name its report gives), `thiscall` with the texture and a sample kind. Null without a
/// texture. Its width (slot 0x94) and height (0x98) go to +0xc and +0x10, +0x68 becomes 1; the
/// pixel format picked for its three preference words (+0x18 to +0x20, copied to the stack) by
/// `00e89ed0` is copied to +0x14 (0x11 words) and gives the Direct3D format (+0xc), replaced
/// by the texture's +0x44 when its byte +0x41 is set. A texture with the byte +0x40 set uses
/// usage 1 and pool 0; else usage 0 and pool 1 when the byte at `0126f0d0` is clear, 0 when
/// set. With a sample kind the device (renderer +0x60, its +0x288) creates a render target
/// surface into +0x6c (width, height, format, `00e7ccf0`'s multisample type, quality 0, not
/// lockable); in any case a texture (width, height, 1 level, usage, format, pool, into a stack
/// word). When either result is negative (their bits or-ed) the failure is reported, +0x64 and
/// +0x6c cleared and null answered; else the texture goes to +0x64 and the pixel format is
/// answered.
pub fn fn_00e8fe10(e: &mut Engine, this: Ptr, texture: u32, kind: u32) -> u32 {
    let base = this.addr();
    if texture == 0 {
        return 0;
    }
    let width = e.vcall(texture, 0x94, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0xc), width);
    let height = e.vcall(texture, 0x98, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x10), height);
    e.mem.set_u32(base.wrapping_add(0x68), 1);
    e.with_stack(0x10, |e, frame| {
        let made = frame.addr();
        let preferences = made + 4;
        for word in 0..3u32 {
            let value = e.mem.u32(texture.wrapping_add(0x18 + 4 * word));
            e.mem.set_u32(preferences + 4 * word, value);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let pixel_format = e
            .call(
                PICK_FORMAT_00E89ED0,
                &args![preferences, renderer.wrapping_add(0x758)],
            )
            .u32();
        for word in 0..0x11u32 {
            let value = e.mem.u32(pixel_format.wrapping_add(4 * word));
            e.mem.set_u32(base.wrapping_add(0x14 + 4 * word), value);
        }
        let mut format = e.mem.u32(pixel_format.wrapping_add(0xc));
        let (usage, pool) = if e.mem.u8(texture.wrapping_add(0x40)) != 0 {
            (1u32, 0u32)
        } else {
            (0, u32::from(e.mem.u8(POOL_FLAG_0126F0D0) == 0))
        };
        if e.mem.u8(texture.wrapping_add(0x41)) != 0 {
            format = e.mem.u32(texture.wrapping_add(0x44));
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
        e.mem.set_u32(made, 0);
        let mut surface_result = 0u32;
        if kind != 0 {
            let samples = e.call(MULTISAMPLE_TYPE_00E7CCF0, &args![kind]).u32();
            let width = e.mem.u32(base.wrapping_add(0xc));
            let height = e.mem.u32(base.wrapping_add(0x10));
            surface_result = e
                .vcall(
                    device,
                    CREATE_RENDER_TARGET,
                    &args![
                        width,
                        height,
                        format,
                        samples,
                        0u32,
                        0u32,
                        base.wrapping_add(0x6c),
                        0u32
                    ],
                )
                .u32();
        }
        let width = e.mem.u32(base.wrapping_add(0xc));
        let height = e.mem.u32(base.wrapping_add(0x10));
        let texture_result = e
            .vcall(
                device,
                CREATE_TEXTURE,
                &args![width, height, 1u32, usage, format, pool, made, 0u32],
            )
            .u32();
        let result = surface_result | texture_result;
        if (result as i32) < 0 {
            let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(
                REPORT_00A29680,
                &args![CREATE_SURFACE_FAILED_010F03B0, name],
            );
            e.mem.set_u32(base.wrapping_add(0x64), 0);
            e.mem.set_u32(base.wrapping_add(0x6c), 0);
            return 0;
        }
        let made = e.mem.u32(made);
        e.mem.set_u32(base.wrapping_add(0x64), made);
        pixel_format
    })
}

/// What `00e8ff60` uses: the base constructor `00e8a260` (unnamed; `thiscall`, two words), the
/// interface identifier at `01101014` (`IID_IDirect3DTexture9`, by the report's text), the
/// texture buffer data maker `00e7d250` (unnamed; `cdecl`: texture, group word, kind, word)
/// and the messages `"NiDX9RenderedTextureData::Create> Failed QueryInterface
/// IID_IDirect3DTexture9 - %s"` (`010f0440`) and `"...> Failed NiDX92DBufferData::Create -
/// %s"` (`010f03f8`, given the string at `01011584`).
const TEXTURE_DATA_CONSTRUCT_00E8A260: u32 = 0x00e8_a260;
const IID_DIRECT3D_TEXTURE_01101014: u32 = 0x0110_1014;
const MAKE_TEXTURE_BUFFER_DATA_00E7D250: u32 = 0x00e7_d250;
const QUERY_FAILED_010F0440: u32 = 0x010f_0440;
const BUFFER_DATA_FAILED_010F03F8: u32 = 0x010f_03f8;
const BUFFER_DATA_FAILED_ARGUMENT_01011584: u32 = 0x0101_1584;

// Translated from 00e8ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedTextureData::Create` (the name
/// its reports give), `cdecl`: the texture, a word handed to the base constructor and the
/// sample kind. A new 0x70-byte object (`00aa13e0`; null when that fails) gets the base
/// constructor (`00e8a260` with the texture and the word) and the class's table; when
/// [`fn_00e8fe10`] fails the object (when made) is deleted (slot 0 with 1) and null answered.
/// Otherwise the object goes to +0x24 of its +8; its Direct3D texture (slot 0x9c) is asked for
/// `IID_IDirect3DTexture9` (`QueryInterface` into a cleared stack word) and the texture's
/// slot 0xa4 gives the render target group word: a failed query is reported, the object
/// deleted and null answered. The texture buffer data is made (`00e7d250` with the queried
/// texture, the group word, the kind and +0x6c), the queried texture released (slot 8) when
/// set; without buffer data the failure is reported, the object deleted and null answered.
/// Answers the object.
pub fn fn_00e8ff60(e: &mut Engine, texture: u32, word: u32, kind: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            TEXTURE_DATA_CONSTRUCT_00E8A260,
            &args![block, texture, word],
        );
        e.mem.set_u32(block, NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
        block
    } else {
        0
    };
    if fn_00e8fe10(e, Ptr::new(data), texture, kind) == 0 {
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        return 0;
    }
    let owner = e.mem.u32(data.wrapping_add(8));
    e.mem.set_u32(owner.wrapping_add(0x24), data);
    let direct3d_texture = e.vcall(data, 0x9c, &args![]).u32();
    let group = e.vcall(texture, 0xa4, &args![]).u32();
    e.with_stack(8, |e, frame| {
        let queried = frame.addr();
        let group_word = queried + 4;
        e.mem.set_u32(group_word, group);
        e.mem.set_u32(queried, 0);
        let result = e
            .vcall(
                direct3d_texture,
                0,
                &args![IID_DIRECT3D_TEXTURE_01101014, queried],
            )
            .i32();
        if result < 0 {
            let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![QUERY_FAILED_010F0440, name]);
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let surface_word = e.mem.u32(data.wrapping_add(0x6c));
        let queried_texture = e.mem.u32(queried);
        let made = e
            .call(
                MAKE_TEXTURE_BUFFER_DATA_00E7D250,
                &args![queried_texture, group_word, kind, surface_word],
            )
            .u32();
        let queried_texture = e.mem.u32(queried);
        if queried_texture != 0 {
            e.vcall(queried_texture, 8, &args![]);
        }
        if made == 0 {
            e.call(
                REPORT_00A29680,
                &args![
                    BUFFER_DATA_FAILED_010F03F8,
                    BUFFER_DATA_FAILED_ARGUMENT_01011584
                ],
            );
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        data
    })
}

// Translated from 00e902d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the word at +0x6c.
pub fn fn_00e902d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x6c))
}

/// The shader base destructor `00e806c0` (`nid3dshader.cpp`; `thiscall`, no stack words).
const SHADER_DOWN_00E806C0: u32 = 0x00e8_06c0;

// Translated from 00e90560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader` deleting destructor,
/// `thiscall` with the flags: its table, the error pass global (`0126f74c`) cleared, the
/// shader base destructor (`00e806c0`), then with bit 0 the 0x40 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn fn_00e90560(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_DX9_ERROR_SHADER_VTABLE);
    e.mem.set_u32(ERROR_PASS_0126F74C, 0);
    e.call(SHADER_DOWN_00E806C0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x40u32]);
    }
    this
}

/// `NiXenonShaderDeclaration::Create` (`00e76700`, `cdecl`: three words).
const SHADER_DECLARATION_CREATE_00E76700: u32 = 0x00e7_6700;

// Translated from 00e90970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`; answers 1. The shader declaration
/// made by `NiXenonShaderDeclaration::Create` (with +0x18, 1, 1) is held at +0x28 (`NiPointer`
/// assignment: the old one let go, deleted at zero; the new one counted up when not null),
/// then the held declaration gets its slot 0x8c with (0, 0, 0, 2, 0).
pub fn fn_00e90970(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    let word = e.mem.u32(base.wrapping_add(0x18));
    let made = e
        .call(SHADER_DECLARATION_CREATE_00E76700, &args![word, 1u32, 1u32])
        .u32();
    let held = e.mem.u32(base.wrapping_add(0x28));
    if held != made {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(base.wrapping_add(0x28), made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    let held = e.mem.u32(base.wrapping_add(0x28));
    e.vcall(held, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    1
}

/// `NiD3DShaderInterface::Initialize` (`00e80570`, `thiscall`; answers a byte).
const SHADER_INTERFACE_INITIALIZE_00E80570: u32 = 0x00e8_0570;

// Translated from 00e90a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`: unless the byte +0x10 is set, runs
/// `NiD3DShaderInterface::Initialize` (false when it fails) and then [`fn_00e90970`]. Answers
/// true otherwise.
pub fn fn_00e90a50(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(this.addr().wrapping_add(0x10)) == 0 {
        if e.call(SHADER_INTERFACE_INITIALIZE_00E80570, &args![this])
            .u8()
            == 0
        {
            return false;
        }
        fn_00e90970(e, this);
    }
    true
}

// Translated from 00e90ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9TextureManager` deleting destructor,
/// `thiscall` with the flags: its table, the device (+8) released (slot 8; no null check) and
/// cleared, the `NiRefObject` table and the global object count down, then with bit 0 the 0x10
/// bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e90ac0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let device = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_TEXTURE_MANAGER_VTABLE);
    e.vcall(device, 8, &args![]);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x10u32]);
    }
    this
}

/// What the texture manager's two queries use: the renderer (`011f4748`, whose +0x180 is a
/// `CRITICAL_SECTION`) with the imported `EnterCriticalSection` / `LeaveCriticalSection`
/// (import slots), the source lookup `00653290` (`bsfacegenmanager.cpp` by the engine map;
/// `thiscall` on the texture, the word `011f444c`) and the renderer data maker `00e68ef0`
/// (unnamed; `cdecl`: the source, the manager's renderer).
const RENDERER_011F4748: u32 = 0x011f_4748;
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const SOURCE_OF_00653290: u32 = 0x0065_3290;
const SOURCE_KIND_011F444C: u32 = 0x011f_444c;
const MAKE_RENDERER_DATA_00E68EF0: u32 = 0x00e6_8ef0;

/// The texture's renderer data (+0x24) under the renderer's lock; when there is none it is
/// made from the texture's source (`00653290`, `00e68ef0` with the manager's +0xc) and
/// `made` is called; no source answers null.
fn renderer_data(e: &mut Engine, this: Ptr, texture: u32, made: impl FnOnce(&mut Engine)) -> u32 {
    let lock = e.mem.u32(RENDERER_011F4748).wrapping_add(0x180);
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![lock]);
    let mut data = e.mem.u32(texture.wrapping_add(0x24));
    if data == 0 {
        let source = e
            .call(SOURCE_OF_00653290, &args![texture, SOURCE_KIND_011F444C])
            .u32();
        if source == 0 {
            e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
            return 0;
        }
        let renderer = e.mem.u32(this.addr().wrapping_add(0xc));
        data = e
            .call(MAKE_RENDERER_DATA_00E68EF0, &args![source, renderer])
            .u32();
        made(e);
    }
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
    data
}

/// Whether `value` is a nonzero power of two (`v != 0 && (v & (v - 1)) == 0`).
fn power_of_two(value: u32) -> bool {
    value != 0 && value & value.wrapping_sub(1) == 0
}

// Translated from 00e90b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the texture manager's query of a texture,
/// `thiscall` with four words: the texture and the addresses of three bytes it clears first
/// (`made`, `mipmapped`, `not_power_of_two`). Null without a texture. The texture's renderer
/// data (+0x24, made under the renderer's lock when missing, which sets `made`; null without a
/// source) gives `not_power_of_two` from its width (+0xc) and height (+0x10). When its slot
/// 0xa4 or else 0xac answers an object, that object's slot 0x9c is answered. Otherwise the
/// object of slot 0xa8, when there is one, gets its slot 0xb4 when the data's slot 0x9c
/// answers null (which sets `made`) or when the texture's byte +0x40 is clear; `mipmapped` is
/// whether the data's slot 0x98 answers more than 1, and the data's slot 0x9c is answered.
pub fn fn_00e90b10(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    made: Ptr,
    mipmapped: Ptr,
    not_power_of_two: Ptr,
) -> u32 {
    e.mem.set_u8(made.addr(), 0);
    e.mem.set_u8(mipmapped.addr(), 0);
    e.mem.set_u8(not_power_of_two.addr(), 0);
    if texture == 0 {
        return 0;
    }
    let data = renderer_data(e, this, texture, |e| e.mem.set_u8(made.addr(), 1));
    if data == 0 {
        return 0;
    }
    let width = e.mem.u32(data.wrapping_add(0xc));
    let height = e.mem.u32(data.wrapping_add(0x10));
    let regular = power_of_two(width) && power_of_two(height);
    e.mem.set_u8(not_power_of_two.addr(), u8::from(!regular));
    let mut target = e.vcall(data, 0xa4, &args![]).u32();
    if target == 0 {
        target = e.vcall(data, 0xac, &args![]).u32();
    }
    if target != 0 {
        return e.vcall(target, 0x9c, &args![]).u32();
    }
    let other = e.vcall(data, 0xa8, &args![]).u32();
    if other != 0 {
        if e.vcall(data, 0x9c, &args![]).u32() == 0 {
            e.vcall(other, 0xb4, &args![]);
            e.mem.set_u8(made.addr(), 1);
        } else if e.mem.u8(texture.wrapping_add(0x40)) == 0 {
            e.vcall(other, 0xb4, &args![]);
        }
    }
    let levels = e.vcall(data, 0x98, &args![]).u32();
    e.mem.set_u8(mipmapped.addr(), u8::from(levels > 1));
    e.vcall(data, 0x9c, &args![]).u32()
}

// Translated from 00e90c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the texture manager's second query, `thiscall`
/// with three words of which it reads the first, the texture. False without a texture or
/// without renderer data (made under the renderer's lock as in [`fn_00e90b10`]). When the
/// data's slot 0xa4 or else 0xac answers an object, whether that object's slot 0x9c answers
/// nonzero; with neither, true.
pub fn fn_00e90c70(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    if texture == 0 {
        return false;
    }
    let data = renderer_data(e, this, texture, |_| {});
    if data == 0 {
        return false;
    }
    let mut target = e.vcall(data, 0xa4, &args![]).u32();
    if target == 0 {
        target = e.vcall(data, 0xac, &args![]).u32();
        if target == 0 {
            return true;
        }
    }
    e.vcall(target, 0x9c, &args![]).u32() != 0
}

/// The render state's reset of its shader words (`00e87a50`, unnamed; `thiscall`).
const RESET_SHADERS_00E87A50: u32 = 0x00e8_7a50;

// Translated from 00e90d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` on the render state: clears the byte
/// +0x1100 and the words +0x1104 and +0x110c, sets +0x1108 and +0x1110 to 1, then `00e87a50`
/// (a tail jump).
pub fn fn_00e90d80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base.wrapping_add(0x1100), 0);
    e.mem.set_u32(base.wrapping_add(0x1104), 0);
    e.mem.set_u32(base.wrapping_add(0x1108), 1);
    e.mem.set_u32(base.wrapping_add(0x110c), 0);
    e.mem.set_u32(base.wrapping_add(0x1110), 1);
    e.call(RESET_SHADERS_00E87A50, &args![base]);
}

/// The render state's device word (+0x10f8) and the device's `SetVertexShader` (slot 0x170)
/// and `SetVertexDeclaration` (0x15c), by the interface's order; the arguments match.
const STATE_DEVICE: u32 = 0x10f8;
const SET_VERTEX_SHADER: u32 = 0x170;
const SET_VERTEX_DECLARATION: u32 = 0x15c;

// Translated from 00e90e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot of `NiDX9RenderState`), `thiscall`
/// with the vertex shader and a byte: with the byte set the current one (+0x10e0) is kept at
/// +0x10e4; a different shader becomes current and goes to the device's `SetVertexShader` (a
/// tail jump).
pub fn fn_00e90e00(e: &mut Engine, this: Ptr, shader: u32, keep: u8) {
    let base = this.addr();
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x10e0));
        e.mem.set_u32(base.wrapping_add(0x10e4), current);
    }
    if e.mem.u32(base.wrapping_add(0x10e0)) != shader {
        e.mem.set_u32(base.wrapping_add(0x10e0), shader);
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.vcall(device, SET_VERTEX_SHADER, &args![shader]);
    }
}

// Translated from 00e90e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the current vertex
/// shader (+0x10e0).
pub fn fn_00e90e40(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x10e0))
}

// Translated from 00e90e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`: sets the kept vertex shader
/// (+0x10e4) again through its own virtual slot 0x8c (with 0 for the byte).
pub fn fn_00e90e50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let kept = e.mem.u32(base.wrapping_add(0x10e4));
    e.vcall(base, 0x8c, &args![kept, 0u32]);
}

// Translated from 00e90e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with a vertex shader that goes away:
/// when it is current (+0x10e0) that is cleared and the device's `SetVertexShader` gets null;
/// when it is the kept one (+0x10e4) that is cleared.
pub fn fn_00e90e70(e: &mut Engine, this: Ptr, shader: u32) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x10e0)) == shader {
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.mem.set_u32(base.wrapping_add(0x10e0), 0);
        e.vcall(device, SET_VERTEX_SHADER, &args![0u32]);
    }
    if e.mem.u32(base.wrapping_add(0x10e4)) == shader {
        e.mem.set_u32(base.wrapping_add(0x10e4), 0);
    }
}

// Translated from 00e90fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a vertex
/// declaration and a byte: a null declaration does nothing; nor does the current one (+0x110c)
/// while the byte +0x1100 says one is set. Otherwise the byte +0x1100 becomes 1, with the byte
/// argument set the current one is kept at +0x1110, the declaration becomes current and goes
/// to the device's `SetVertexDeclaration` (a tail jump).
pub fn fn_00e90fc0(e: &mut Engine, this: Ptr, declaration: u32, keep: u8) {
    let base = this.addr();
    if declaration == 0 {
        return;
    }
    if e.mem.u8(base.wrapping_add(0x1100)) != 0
        && e.mem.u32(base.wrapping_add(0x110c)) == declaration
    {
        return;
    }
    e.mem.set_u8(base.wrapping_add(0x1100), 1);
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x110c));
        e.mem.set_u32(base.wrapping_add(0x1110), current);
    }
    e.mem.set_u32(base.wrapping_add(0x110c), declaration);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_VERTEX_DECLARATION, &args![declaration]);
}

// Translated from 00e91020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the current vertex
/// declaration (+0x110c) when the byte +0x1100 says one is set, else 0.
pub fn fn_00e91020(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(0x1100)) != 0 {
        return e.mem.u32(base.wrapping_add(0x110c));
    }
    0
}

// Translated from 00e91060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with a vertex declaration that goes
/// away: when it is not null and the byte +0x1100 is set, the current (+0x110c) and kept
/// (+0x1110) words holding it are cleared.
pub fn fn_00e91060(e: &mut Engine, this: Ptr, declaration: u32) {
    let base = this.addr();
    if declaration == 0 || e.mem.u8(base.wrapping_add(0x1100)) == 0 {
        return;
    }
    if e.mem.u32(base.wrapping_add(0x110c)) == declaration {
        e.mem.set_u32(base.wrapping_add(0x110c), 0);
    }
    if e.mem.u32(base.wrapping_add(0x1110)) == declaration {
        e.mem.set_u32(base.wrapping_add(0x1110), 0);
    }
}

/// The sampler state slots by state (`0126f92c`, 16-bit entries; below 5 the state is cached)
/// and the device's `SetSamplerState` (slot 0x114, by the interface's order).
const SAMPLER_STATE_SLOTS_0126F92C: u32 = 0x0126_f92c;
const SET_SAMPLER_STATE: u32 = 0x114;

// Translated from 00e910a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with four words, the last not read:
/// the sampler, the state and the value. A state whose slot (`0126f92c`) is below 5 is cached
/// at +0xe20 (8 bytes per slot, 5 slots per sampler); a value different from the cached one
/// is cached and goes to the device's `SetSamplerState`.
pub fn fn_00e910a0(
    e: &mut Engine,
    this: Ptr,
    sampler: u32,
    state: u32,
    value: u32,
    _unused_3: u32,
) {
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_STATE_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let base = this.addr();
    let cached = base
        .wrapping_add(0xe20)
        .wrapping_add(sampler.wrapping_mul(5).wrapping_add(slot).wrapping_mul(8));
    if e.mem.u32(cached) == value {
        return;
    }
    e.mem.set_u32(cached, value);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_SAMPLER_STATE, &args![sampler, state, value]);
}

/// The render state's base destructor (`00e88b00`, unnamed; `thiscall`).
const RENDER_STATE_DOWN_00E88B00: u32 = 0x00e8_8b00;

// Translated from 00e91160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderState` deleting destructor,
/// `thiscall` with the flags: its table, the base destructor (`00e88b00`), then with bit 0 the
/// 0x1248 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e91160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_DX9_RENDER_STATE_VTABLE);
    e.call(RENDER_STATE_DOWN_00E88B00, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x1248u32]);
    }
    this
}

/// What `00e911e0` uses: the member reset `00e94130` (unnamed; `thiscall` on +0xf8), the
/// default colour words (`011f4998` to `011f49a0`) and the `double` 255.0 (`0101e568`).
const MEMBER_RESET_00E94130: u32 = 0x00e9_4130;
const DEFAULT_COLOR_011F4998: u32 = 0x011f_4998;
const SCALE_0101E568: u32 = 0x0101_e568;

/// The low byte of a colour channel: `FISTP` under the chop rounding the code sets.
fn colour_byte(value: f64) -> u32 {
    (crate::x87::fistp_chop(value) as u32) & 0xff
}

// Translated from 00e911e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot of `NiDX9RenderState`), the
/// render state's reset, `thiscall`. Unless the flags at +0x1138 have bit 0x4000, the (state,
/// value) pairs at `011bfce8` (ended by state -1) are compacted in place: pairs of states
/// 0xbe, 0xbf and 0xc0 are dropped (the loop's read and write positions and its end test, on
/// the word at the write position, kept as the code has them). Then its virtual slots 0x58,
/// 0x70, 0xbc and 0xd8, `00e94130` on +0xf8, and its slot 0x68 (render state, value, 0, 0)
/// for states 0x16 (2), 8 (3), 9 (2), 0x1b (0), 0x13 (+0x38), 0x14 (+0x3c), 0xf (0), 0x19 (8),
/// 0x18 (0), 0x17 (8), 0xe (0), 0x1a (0), 0x1d (0), 0x1c (0), 0x23 (3 when bit 0 of +8 is set,
/// else 0), 0x8c (3), 0x30 (0), 0x22 (0), 0x8f (1), 7 (1) and 0x8d (1). Last the colour words
/// from `011f4998` go to +0x8c and their packed form (alpha 0xff, each channel times 255.0
/// truncated to its low byte) to +0x98.
pub fn fn_00e911e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x1138)) & 0x4000 == 0 {
        let pair = |index: u32| STATE_PAIRS_011BFCE8.wrapping_add(index.wrapping_mul(8));
        let mut written = 0u32;
        let mut dropped = 0u32;
        if e.mem.u32(STATE_PAIRS_011BFCE8) != 0xffff_ffff {
            let mut write = STATE_PAIRS_011BFCE8;
            let mut test = STATE_PAIRS_011BFCE8;
            loop {
                let read = pair(dropped.wrapping_add(written));
                let state = e.mem.u32(read);
                if state == 0xbe || state == 0xbf || state == 0xc0 {
                    dropped = dropped.wrapping_add(1);
                } else {
                    e.mem.set_u32(write, state);
                    let value = e.mem.u32(read.wrapping_add(4));
                    e.mem.set_u32(write.wrapping_add(4), value);
                    if e.mem.u32(read) == 0xffff_ffff {
                        dropped = dropped.wrapping_sub(1);
                    }
                    written = written.wrapping_add(1);
                    write = write.wrapping_add(8);
                    test = write;
                }
                if e.mem.u32(test) == 0xffff_ffff {
                    break;
                }
            }
        }
    }
    for slot in [0x58u32, 0x70, 0xbc, 0xd8] {
        e.vcall(base, slot, &args![]);
    }
    e.call(MEMBER_RESET_00E94130, &args![base.wrapping_add(0xf8)]);
    // States 0x13, 0x14 and 0x23 take values read when their call is made (below).
    let states = [
        (0x16u32, 2u32),
        (8, 3),
        (9, 2),
        (0x1b, 0),
        (0x13, 0),
        (0x14, 0),
        (0xf, 0),
        (0x19, 8),
        (0x18, 0),
        (0x17, 8),
        (0xe, 0),
        (0x1a, 0),
        (0x1d, 0),
        (0x1c, 0),
        (0x23, 0),
        (0x8c, 3),
        (0x30, 0),
        (0x22, 0),
        (0x8f, 1),
        (7, 1),
        (0x8d, 1),
    ];
    for (state, value) in states {
        let value = match state {
            0x13 => e.mem.u32(base.wrapping_add(0x38)),
            0x14 => e.mem.u32(base.wrapping_add(0x3c)),
            0x23 if e.mem.u8(base.wrapping_add(8)) & 1 != 0 => 3,
            _ => value,
        };
        e.vcall(base, 0x68, &args![state, value, 0u32, 0u32]);
    }
    for word in 0..3u32 {
        let value = e.mem.u32(DEFAULT_COLOR_011F4998 + 4 * word);
        e.mem.set_u32(base.wrapping_add(0x8c + 4 * word), value);
    }
    let scale = x87_load64(e.mem.u64(SCALE_0101E568));
    // FMUL ST(1),ST(0) (bytes DC C9), then FMUL ST(0),ST(1) (D8 C9), then FMUL dword.
    let red = x87_mul(x87_load32(e.mem.u32(base.wrapping_add(0x8c))), scale);
    let mut packed = (0xffff_ff00 | colour_byte(red)) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(base.wrapping_add(0x90))), scale);
    packed = (packed | colour_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(base.wrapping_add(0x94))));
    packed |= colour_byte(blue);
    e.mem.set_u32(base.wrapping_add(0x98), packed);
}

// Translated from 00e914e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` on the render state: bit 0 of +8 set
/// when the caps word +0x113c has both 0x100 and 0x100000, else cleared; +0x6c and +0x70
/// rebuilt from the eleven words at +0x20 (bit `n` when word `n` shares a bit with the masks at
/// +0x1144 resp. +0x1148, read once); bit 2 of +8 from bit 4 of the byte +0x1164; the byte
/// +0x1114 set when the renderer (+0x10fc) has its byte +0x5d4 set and +0x5d5 clear, else
/// cleared.
pub fn fn_00e914e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let caps = e.mem.u32(base.wrapping_add(0x113c));
    let flags = e.mem.u32(base.wrapping_add(8));
    let flags = if caps & 0x100 != 0 && caps & 0x10_0000 != 0 {
        flags | 1
    } else {
        flags & 0xffff_fffe
    };
    e.mem.set_u32(base.wrapping_add(8), flags);
    let second_mask = e.mem.u32(base.wrapping_add(0x1148));
    let first_mask = e.mem.u32(base.wrapping_add(0x1144));
    e.mem.set_u32(base.wrapping_add(0x70), 0);
    e.mem.set_u32(base.wrapping_add(0x6c), 0);
    let mut bit = 1u32;
    for index in 0..0xbu32 {
        let word = e.mem.u32(base.wrapping_add(0x20 + 4 * index));
        if word & first_mask != 0 {
            let value = e.mem.u32(base.wrapping_add(0x6c)) | bit;
            e.mem.set_u32(base.wrapping_add(0x6c), value);
        }
        if word & second_mask != 0 {
            let value = e.mem.u32(base.wrapping_add(0x70)) | bit;
            e.mem.set_u32(base.wrapping_add(0x70), value);
        }
        bit = bit.rotate_left(1);
    }
    let flags = e.mem.u32(base.wrapping_add(8));
    let flags = if e.mem.u8(base.wrapping_add(0x1164)) & 0x10 != 0 {
        flags | 4
    } else {
        flags & 0xffff_fffb
    };
    e.mem.set_u32(base.wrapping_add(8), flags);
    let renderer = e.mem.u32(base.wrapping_add(0x10fc));
    let fixed =
        e.mem.u8(renderer.wrapping_add(0x5d4)) != 0 && e.mem.u8(renderer.wrapping_add(0x5d5)) == 0;
    e.mem.set_u8(base.wrapping_add(0x1114), u8::from(fixed));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_0850, fn_00e90850(Ptr) -> u8),
        entry!(0x00e8_fb20, fn_00e8fb20(Ptr, u32) -> Ptr),
        entry!(0x00e9_0a80, fn_00e90a80(Ptr, u32) -> Ptr),
        entry!(0x00e9_09e0, fn_00e909e0(Ptr) -> Ptr),
        entry!(0x00e9_1590, fn_00e91590(u32, u32, u8) -> u32),
        entry!(0x00e8_f190, fn_00e8f190(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_f200, fn_00e8f200(Ptr, u32, u32, u32, u32, Ptr, u32, u32) -> u32),
        entry!(0x00e8_f840, fn_00e8f840(Ptr)),
        entry!(0x00e8_fa80, fn_00e8fa80(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e8_fc30, fn_00e8fc30(Ptr, u32) -> Ptr),
        entry!(0x00e8_fc50, fn_00e8fc50(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_fdc0, fn_00e8fdc0(Ptr, u32) -> Ptr),
        entry!(0x00e8_fe10, fn_00e8fe10(Ptr, u32, u32) -> u32),
        entry!(0x00e8_ff60, fn_00e8ff60(u32, u32, u32) -> u32),
        entry!(0x00e9_02d0, fn_00e902d0(Ptr) -> u32),
        entry!(0x00e9_0560, fn_00e90560(Ptr, u32) -> Ptr),
        entry!(0x00e9_0970, fn_00e90970(Ptr) -> u8),
        entry!(0x00e9_0a50, fn_00e90a50(Ptr) -> bool),
        entry!(0x00e9_0ac0, fn_00e90ac0(Ptr, u32) -> Ptr),
        entry!(0x00e9_0b10, fn_00e90b10(Ptr, u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e9_0c70, fn_00e90c70(Ptr, u32, u32, u32) -> bool),
        entry!(0x00e9_0d80, fn_00e90d80(Ptr)),
        entry!(0x00e9_0e00, fn_00e90e00(Ptr, u32, u8)),
        entry!(0x00e9_0e40, fn_00e90e40(Ptr) -> u32),
        entry!(0x00e9_0e50, fn_00e90e50(Ptr)),
        entry!(0x00e9_0e70, fn_00e90e70(Ptr, u32)),
        entry!(0x00e9_0fc0, fn_00e90fc0(Ptr, u32, u8)),
        entry!(0x00e9_1020, fn_00e91020(Ptr) -> u32),
        entry!(0x00e9_1060, fn_00e91060(Ptr, u32)),
        entry!(0x00e9_10a0, fn_00e910a0(Ptr, u32, u32, u32, u32)),
        entry!(0x00e9_1160, fn_00e91160(Ptr, u32) -> Ptr),
        entry!(0x00e9_11e0, fn_00e911e0(Ptr)),
        entry!(0x00e9_14e0, fn_00e914e0(Ptr)),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x6000;
    const ADD_REF: u32 = 0x6100;
    const DELETE: u32 = 0x6104;
    const STATE_SLOT_104: u32 = 0x6200;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose virtual table has `AddRef` at slot 4, delete at slot 4 for released
    /// objects (same slot), and the render state's slot 0x104.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x42];
        slots[1] = ADD_REF;
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(OBJECT_VTABLE, &slots);
        for no_op in [
            ADD_REF,
            DELETE,
            STATE_SLOT_104,
            IMPORT_INTERLOCKED_INCREMENT,
            MEMSET,
            MAP_00B601A0,
            SHADER_CONSTRUCT_00E818A0,
            ERROR_SHADER_SET_UP_00E90850,
            SET_NAME_00A5B950,
            MEMCPY,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, OBJECT_VTABLE);
        e.call_log = Some(vec![]);
        (e, device)
    }

    #[test]
    fn fn_00e8fb20_builds_the_index_buffer_manager() {
        let (mut e, device) = rig();
        let this = e.mem.alloc(0x50);
        e.call(0x00e8_fb20, &args![this, device]);
        assert_eq!(e.mem.u32(this), NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        for offset in [0x1c, 0x2c, 0x3c] {
            assert_eq!(e.mem.u32(this + offset), IB_MAP_VTABLE);
        }
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        assert_eq!(calls(&e, MAP_00B601A0).len(), 3);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
    }

    #[test]
    fn fn_00e90a80_keeps_the_renderer_and_its_device() {
        let (mut e, device) = rig();
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00e9_0a80, &args![this, renderer]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_TEXTURE_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(e.mem.u32(this + 0xc), renderer);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
    }

    #[test]
    fn fn_00e909e0_names_the_error_shader() {
        let (mut e, _) = rig();
        e.map(FIXED_STRING_EMPTY, 4);
        e.register(FIXED_STRING_MAKE, |_, _| 0x7008u32.into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e9_09e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_ERROR_SHADER_VTABLE);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE),
            vec![vec![ERROR_SHADER_NAME_010F0854]]
        );
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![0x7000]]);
        // The empty string is not released.
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x7008);
        e.call_log = Some(vec![]);
        e.call(0x00e9_09e0, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00e91590_makes_the_render_state() {
        let (mut e, _) = rig();
        let mut slots = vec![0u32; 0x42];
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(NI_DX9_RENDER_STATE_VTABLE, &slots);
        e.map(STATE_PAIRS_011BFCE8, 0x20);
        for (i, (id, value)) in [(3u32, 9u32), (7, 9), (0xffff_ffff, 0), (7, 9)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32, *id);
            e.mem
                .set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32 + 4, *value);
        }
        e.register(RENDER_STATE_CONSTRUCT_00E881A0, |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            a[0].into_ret()
        });
        let made = e.mem.alloc(8);
        e.register_double(MAKE_00E978A0, move |_, _| made.into_ret());
        let state = e
            .call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 1u32])
            .u32();
        assert_eq!(e.mem.u32(state), NI_DX9_RENDER_STATE_VTABLE);
        assert_eq!(calls(&e, MEMCPY), vec![vec![state + 0x1118, 0x5000, 0x130]]);
        assert_eq!(calls(&e, STATE_SLOT_104).len(), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0x1c), 9);
        assert_eq!(e.mem.u32(state + 8) & 2, 2);
        assert_eq!(e.mem.u32(state + 0x10f0), made);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT).last(),
            Some(&vec![made + 4])
        );
        // Without the byte, state 7 becomes 0.
        e.call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 0u32]);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 0);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e90850_sets_up_the_error_pass() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x14, 0x1414);
        e.mem.set_u32(pass + 0x64, 2);
        let group = e.mem.alloc(0x70);
        e.mem.set_u32(group + 0x10, 0x1010);
        e.mem.set_u32(group + 0x60, 1);
        e.register_double(MAKE_PASS_00E804D0, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register_double(MAKE_GROUP_00E7EDF0, move |e, a| {
            e.mem.set_u32(a[0], group);
            Ret::default()
        });
        for no_op in [
            SET_STATE_00E88FC0,
            SET_SAMPLER_STATE_00E890C0,
            ATTACH_00E802B0,
            RELEASE_GROUP_00E7ECD0,
            RELEASE_PASS_00E80190,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e9_0850, &args![this]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x3c), pass);
        assert_eq!(e.mem.u32(ERROR_PASS_0126F74C), pass);
        let states = calls(&e, SET_STATE_00E88FC0);
        assert_eq!(states.len(), 6);
        assert_eq!(states[0], vec![0x1010, 1, 2, 0]);
        assert_eq!(states[5], vec![0x1010, 0x18, 0, 0]);
        assert_eq!(calls(&e, SET_SAMPLER_STATE_00E890C0).len(), 5);
        assert_eq!(calls(&e, ATTACH_00E802B0), vec![vec![pass, 0x1414, group]]);
        // The group's count reaches zero, the pass's does not.
        assert_eq!(calls(&e, RELEASE_GROUP_00E7ECD0), vec![vec![group]]);
        assert!(calls(&e, RELEASE_PASS_00E80190).is_empty());
        assert_eq!(e.mem.u32(pass + 0x64), 1);
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object of `size` bytes whose table at `vtable` has `slots` (byte offset, target).
    fn object(e: &mut Engine, vtable: u32, size: u32, slots: &[(u32, u32)]) -> u32 {
        let mut words = vec![0u32; 0x80];
        for (slot, target) in slots {
            words[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Doubles answering 0 at `0x7000 + slot` for every slot of the tables these tests make.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for slot in (0..0x400u32).step_by(4) {
            e.register(0x7000 + slot, |_, _| Ret::default());
        }
        for no_op in [
            REPORT_00A29680,
            MEMCPY,
            ENTRY_CLEAN_UP_00E95B80,
            FREE_00AA1460,
            FREE_00AA10F0,
            MAP_00B601A0,
            MAP_SET_00B57EA0,
            ENTRY_CONSTRUCT_00E95B60,
            TEXTURE_DATA_DOWN_00E8A2B0,
            TEXTURE_DATA_CONSTRUCT_00E8A260,
            SHADER_DOWN_00E806C0,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            RESET_SHADERS_00E87A50,
            RENDER_STATE_DOWN_00E88B00,
            MEMBER_RESET_00E94130,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ERROR_STRING_00E89D50, |_, _| 0x5151u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A device whose `CreateIndexBuffer` hands out `buffer` (or fails when it is 0).
    fn index_device(e: &mut Engine, buffer: u32) -> u32 {
        e.register_double(0x786c, move |e, a| {
            e.mem.set_u32(a[5], buffer);
            (if buffer != 0 { 0u32 } else { 0x8000_0000 }).into_ret()
        });
        object(
            e,
            0x8000,
            8,
            &[
                (CREATE_INDEX_BUFFER, 0x786c),
                (8, 0x7008),
                (SET_INDICES, 0x71a0),
            ],
        )
    }

    #[test]
    fn fn_00e8f190_creates_an_index_buffer() {
        let mut e = engine();
        let buffer = object(&mut e, 0x8200, 8, &[(INDEX_BUFFER_GET_DESC, 0x7034)]);
        let device = index_device(&mut e, buffer);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f190,
                &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0u32]
            )
            .u32(),
            buffer
        );
        let create = &calls(&e, 0x786c)[0];
        assert_eq!(create[..5], [device, 0x40, 8, 0x65, 1]);
        assert_eq!(create[6], 0);
        assert!(calls(&e, 0x7034).is_empty());
        e.call(
            0x00e8_f190,
            &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0x9000u32],
        );
        assert_eq!(calls(&e, 0x7034), vec![vec![buffer, 0x9000]]);
        let device = index_device(&mut e, 0);
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f190,
                &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0u32]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_INDEX_BUFFER_FAILED_010F0184]]
        );
    }

    /// An index buffer whose description is (format, type, usage, pool, size) and whose lock
    /// hands out `data` (failing when it is 0).
    fn index_buffer(e: &mut Engine, desc: [u32; 5], data: u32) -> u32 {
        e.register_double(0x7134, move |e, a| {
            for (i, word) in desc.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            0u32.into_ret()
        });
        e.register_double(0x712c, move |e, a| {
            e.mem.set_u32(a[3], data);
            (if data != 0 { 0u32 } else { 0x8000_0000 }).into_ret()
        });
        object(
            e,
            0x8200,
            8,
            &[
                (INDEX_BUFFER_GET_DESC, 0x7134),
                (INDEX_BUFFER_LOCK, 0x712c),
                (INDEX_BUFFER_UNLOCK, 0x7130),
                (8, 0x7108),
            ],
        )
    }

    #[test]
    fn fn_00e8f200_reuses_or_creates_and_fills_the_buffer() {
        let mut e = engine();
        for target in [0x7130u32, 0x7108] {
            e.register(target, |_, _| Ret::default());
        }
        let data = e.mem.alloc(0x40);
        let reuse = index_buffer(&mut e, [0x65, 7, 8, 1, 0x40], data);
        let made = e.mem.alloc(8);
        e.mem.set_u32(made, 0x8200);
        let device = index_device(&mut e, made);
        let this = e.mem.alloc(0x4c);
        let length = e.mem.alloc(4);
        let indices = e.mem.alloc(0x20);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, 0u32, 6u32, 0x20u32, reuse, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        // Large enough and alike: reused, filled, unlocked.
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, reuse);
        assert!(calls(&e, 0x786c).is_empty());
        assert_eq!(
            calls(&e, 0x712c),
            vec![vec![reuse, 0, 0xc, calls(&e, 0x712c)[0][3], 0]]
        );
        assert_eq!(calls(&e, MEMCPY), vec![vec![data, indices, 0xc]]);
        assert_eq!(calls(&e, 0x7130).len(), 1);
        assert_eq!(e.mem.u32(length), 0);
        // Too small: a new one, its length kept.
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x21u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, made);
        let create = &calls(&e, 0x786c)[0];
        assert_eq!(create[..5], [device, 0x42, 8, 0x65, 1]);
        assert_eq!(e.mem.u32(length), 0x42);
        // Nothing to copy: no lock.
        e.call(
            0x00e8_f200,
            &args![this, indices, 0u32, 0x20u32, reuse, length, 1u32, 8u32],
        );
        assert_eq!(calls(&e, 0x712c).len(), 2);
    }

    #[test]
    fn fn_00e8f200_reports_failures() {
        let mut e = engine();
        for target in [0x7130u32, 0x7108] {
            e.register(target, |_, _| Ret::default());
        }
        let device = index_device(&mut e, 0);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        let length = e.mem.alloc(4);
        let indices = e.mem.alloc(0x20);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, 0u32, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_INDEX_BUFFER_FAILED_010F0184],
                vec![PACK_CREATE_FAILED_010F01B8]
            ]
        );
        // A failed lock: reported, unlocked and released, still answered.
        let reuse = index_buffer(&mut e, [0x65, 7, 8, 1, 0x40], 0);
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, reuse);
        assert_eq!(
            calls(&e, REPORT_00A29680)[2],
            vec![PACK_LOCK_FAILED_010EDDD0]
        );
        assert_eq!(calls(&e, 0x7130), vec![vec![reuse]]);
        assert_eq!(calls(&e, 0x7108), vec![vec![reuse]]);
        assert!(calls(&e, MEMCPY).is_empty());
    }

    /// An index buffer manager whose map +0x2c has two items (keys 0 and 1, in buckets 0 and
    /// 1; the hash answers the key), the first holding an information block with one entry
    /// at its +0x10 that holds a buffer.
    fn index_manager(e: &mut Engine) -> (u32, u32, u32, u32, u32) {
        let device = index_device(e, 0);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        let map_table = 0x8400;
        e.register(0x7204, |_, a| a[1].into_ret());
        object(e, map_table, 4, &[(4, 0x7204)]);
        for offset in INDEX_BUFFER_MAPS {
            let buckets = e.mem.alloc(8);
            e.mem.set_u32(this + offset, map_table);
            e.mem.set_u32(this + offset + 4, 2);
            e.mem.set_u32(this + offset + 8, buckets);
        }
        let buckets = e.mem.u32(this + 0x2c + 8);
        let info = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0x38);
        let held = object(e, 0x8600, 8, &[(8, 0x7308)]);
        e.mem.set_u32(entry + 0x20, held);
        e.mem.set_u32(info + 0x10, entry);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0] + 8, info);
        e.mem.set_u32(items[1] + 4, 1);
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 4, items[1]);
        (this, device, info, entry, held)
    }

    #[test]
    fn fn_00e8f840_frees_every_information_block() {
        let mut e = engine();
        let (this, device, info, entry, held) = index_manager(&mut e);
        e.call(0x00e8_f840, &args![this]);
        assert_eq!(calls(&e, 0x7204).len(), 2);
        assert_eq!(calls(&e, 0x7308), vec![vec![held]]);
        assert_eq!(calls(&e, ENTRY_CLEAN_UP_00E95B80), vec![vec![entry]]);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![entry, 0x38], vec![info, 0x1c]]
        );
        assert_eq!(calls(&e, MAP_00B601A0).len(), 9);
        assert_eq!(calls(&e, 0x71a0), vec![vec![device, 0]]);
        assert_eq!(calls(&e, 0x7008), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 3);
        assert_eq!(e.mem.u32(this + 0x1c), IB_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        let mut e = engine();
        let (this, _, info, entry, _) = index_manager(&mut e);
        assert_eq!(e.call(0x00e8_fc30, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![entry, 0x38], vec![info, 0x1c], vec![this, 0x4c]]
        );
    }

    #[test]
    fn fn_00e8fa80_finds_or_makes_the_information_block() {
        let mut e = engine();
        let found = e.mem.alloc(4);
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            let block = e.mem.u32(found);
            e.mem.set_u32(a[2], block);
            u32::from(block != 0).into_ret()
        });
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 3u32]).u32(),
            0
        );
        assert!(calls(&e, MAP_LOOKUP_00A53C30).is_empty());
        e.mem.set_u32(found, 0x1234);
        assert_eq!(
            e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 1u32]).u32(),
            0x1234
        );
        assert_eq!(
            calls(&e, MAP_LOOKUP_00A53C30)[0][..2],
            [this + 0x2c, 0x8_0065]
        );
        e.mem.set_u32(found, 0);
        let block = e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 2u32]).u32();
        assert_eq!(e.mem.u32(block), 0x8_0065);
        assert_eq!(
            calls(&e, MAP_SET_00B57EA0),
            vec![vec![this + 0x3c, 0x8_0065, block]]
        );
    }

    #[test]
    fn fn_00e8fc50_rotates_through_the_entries() {
        let mut e = engine();
        let info = e.mem.alloc(0x1c);
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            e.mem.set_u32(a[2], info);
            1u32.into_ret()
        });
        let buffer = object(
            &mut e,
            0x8200,
            8,
            &[(INDEX_BUFFER_GET_DESC, 0x7034), (8, 0x7108)],
        );
        e.register(0x7108, |_, _| Ret::default());
        let device = index_device(&mut e, buffer);
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x40u32, 8u32, 0x65u32, 0u32])
                .u32(),
            0
        );
        e.mem.set_u32(this + 8, device);
        // No entry: a new one (not kept in the block) and a new buffer.
        let answer = e
            .call(0x00e8_fc50, &args![this, 0x40u32, 8u32, 0x65u32, 0u32])
            .u32();
        assert_eq!(answer, buffer);
        let entry = calls(&e, ENTRY_CONSTRUCT_00E95B60)[0][0];
        assert_eq!(e.mem.u32(entry + 0x20), buffer);
        assert_eq!(e.mem.u32(entry + 0xc), 8);
        assert_eq!(e.mem.u32(entry + 0x14), 0x40);
        assert_eq!(calls(&e, 0x7034)[0][1], entry + 0x24);
        assert_eq!(e.mem.u8(info + 4), 1);
        assert_eq!(e.mem.u32(info + 0xc), 0);
        // An entry no longer than the length asked gives its buffer.
        e.mem.set_u32(info + 0xc, entry);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x50u32, 8u32, 0x65u32, 0u32])
                .u32(),
            buffer
        );
        assert_eq!(e.mem.u8(info + 4), 2);
        assert_eq!(e.mem.u32(entry + 0x14), 0x40);
        // A longer one is released and made again; the index wraps at 5.
        e.mem.set_u8(info + 4, 4);
        e.mem.set_u32(info + 0x18, entry);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x20u32, 8u32, 0x65u32, 0u32])
                .u32(),
            buffer
        );
        assert_eq!(calls(&e, 0x7108), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(entry + 0x14), 0x20);
        assert_eq!(e.mem.u8(info + 4), 0);
    }

    #[test]
    fn fn_00e8fdc0_takes_the_rendered_texture_data_down() {
        let mut e = engine();
        let renderer = object(&mut e, 0x8000, 8, &[(0x15c, 0x715c)]);
        let surface = object(&mut e, 0x8200, 8, &[(8, 0x7108)]);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, renderer);
        e.mem.set_u32(this + 8, 0x42);
        e.mem.set_u32(this + 0x6c, surface);
        assert_eq!(e.call(0x00e8_fdc0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
        assert_eq!(calls(&e, 0x715c), vec![vec![renderer, 0x42]]);
        assert_eq!(calls(&e, 0x7108), vec![vec![surface]]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(calls(&e, TEXTURE_DATA_DOWN_00E8A2B0), vec![vec![this]]);
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, 0x70]]);
    }

    /// A rendered texture data object and a texture for `00e8fe10`; the device's
    /// `CreateTexture` answers `texture_result` and its `CreateRenderTarget` 0.
    fn surface_rig(e: &mut Engine, texture_result: u32) -> (u32, u32, u32) {
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format + 0xc, 0x15);
        e.register_double(PICK_FORMAT_00E89ED0, move |_, _| format.into_ret());
        e.register(MULTISAMPLE_TYPE_00E7CCF0, |_, a| (a[0] * 2).into_ret());
        e.register_double(0x785c, move |e, a| {
            e.mem.set_u32(a[7], 0x7777);
            texture_result.into_ret()
        });
        let device = object(
            e,
            0x8000,
            8,
            &[(CREATE_TEXTURE, 0x785c), (CREATE_RENDER_TARGET, 0x7070)],
        );
        let renderer = e.mem.alloc(0x800);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        e.register(0x7194, |_, _| 256u32.into_ret());
        e.register(0x7198, |_, _| 128u32.into_ret());
        let texture = object(e, 0x8200, 0x48, &[(0x94, 0x7194), (0x98, 0x7198)]);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, renderer);
        (this, texture, format)
    }

    #[test]
    fn fn_00e8fe10_creates_the_surfaces() {
        let mut e = engine();
        let (this, texture, format) = surface_rig(&mut e, 0);
        assert_eq!(e.call(0x00e8_fe10, &args![this, 0u32, 0u32]).u32(), 0);
        e.mem.set_u32(texture + 0x18, 0xa);
        assert_eq!(
            e.call(0x00e8_fe10, &args![this, texture, 0u32]).u32(),
            format
        );
        assert_eq!(e.mem.u32(this + 0xc), 256);
        assert_eq!(e.mem.u32(this + 0x10), 128);
        assert_eq!(e.mem.u32(this + 0x68), 1);
        assert_eq!(e.mem.u32(this + 0x14 + 0xc), 0x15);
        assert_eq!(e.mem.u32(this + 0x64), 0x7777);
        let renderer = e.mem.u32(this + 0x60);
        assert_eq!(calls(&e, PICK_FORMAT_00E89ED0)[0][1], renderer + 0x758);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        let create = &calls(&e, 0x785c)[0];
        assert_eq!(create[..7], [device, 256, 128, 1, 0, 0x15, 1]);
        assert!(calls(&e, 0x7070).is_empty());
        // Byte +0x40: usage 1, pool 0; byte +0x41: the texture's own format; a sample kind.
        e.mem.set_u8(texture + 0x40, 1);
        e.mem.set_u8(texture + 0x41, 1);
        e.mem.set_u32(texture + 0x44, 0x33);
        e.call(0x00e8_fe10, &args![this, texture, 2u32]);
        assert_eq!(calls(&e, 0x785c)[1][..7], [device, 256, 128, 1, 1, 0x33, 0]);
        assert_eq!(
            calls(&e, 0x7070),
            vec![vec![device, 256, 128, 0x33, 4, 0, 0, this + 0x6c, 0]]
        );
    }

    #[test]
    fn fn_00e8fe10_reports_a_failed_texture() {
        let mut e = engine();
        let (this, texture, _) = surface_rig(&mut e, 0x8876_0868);
        e.mem.set_u8(POOL_FLAG_0126F0D0, 1);
        e.mem.set_u32(this + 0x6c, 0x99);
        assert_eq!(e.call(0x00e8_fe10, &args![this, texture, 0u32]).u32(), 0);
        assert_eq!(calls(&e, 0x785c)[0][6], 0);
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8876_0868]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_SURFACE_FAILED_010F03B0, 0x5151]]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
    }

    #[test]
    fn fn_00e8ff60_makes_the_rendered_texture_data() {
        let mut e = engine();
        let (probe, texture, _) = surface_rig(&mut e, 0);
        let renderer = e.mem.u32(probe + 0x60);
        let owner = e.mem.alloc(0x28);
        // The base constructor sets the renderer and the owner.
        e.register_double(TEXTURE_DATA_CONSTRUCT_00E8A260, move |e, a| {
            e.mem.set_u32(a[0] + 0x60, renderer);
            e.mem.set_u32(a[0] + 8, owner);
            Ret::default()
        });
        let queried = object(&mut e, 0x8600, 8, &[(8, 0x7308)]);
        e.register_double(0x7400, move |e, a| {
            e.mem.set_u32(a[2], queried);
            0u32.into_ret()
        });
        let d3d_texture = object(&mut e, 0x8800, 8, &[(0, 0x7400)]);
        e.register_double(0x729c, move |_, _| d3d_texture.into_ret());
        let mut slots = vec![0u32; 0x40];
        slots[0x9c / 4] = 0x729c;
        slots[0] = 0x7200;
        e.put_vtable(NI_DX9_RENDERED_TEXTURE_DATA_VTABLE, &slots);
        e.register(0x71a4, |_, _| 0x4444u32.into_ret());
        let mut texture_slots = vec![0u32; 0x40];
        texture_slots[0x94 / 4] = 0x7194;
        texture_slots[0x98 / 4] = 0x7198;
        texture_slots[0xa4 / 4] = 0x71a4;
        e.put_vtable(0x8200, &texture_slots);
        let made = e.mem.alloc(4);
        e.register_double(MAKE_TEXTURE_BUFFER_DATA_00E7D250, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x4444);
            e.mem.u32(made).into_ret()
        });
        e.mem.set_u32(made, 0x5555);
        let data = e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32();
        assert_ne!(data, 0);
        assert_eq!(
            calls(&e, TEXTURE_DATA_CONSTRUCT_00E8A260),
            vec![vec![data, texture, 0x31]]
        );
        assert_eq!(e.mem.u32(owner + 0x24), data);
        assert_eq!(calls(&e, 0x7400)[0][1], IID_DIRECT3D_TEXTURE_01101014);
        let make = &calls(&e, MAKE_TEXTURE_BUFFER_DATA_00E7D250)[0];
        assert_eq!([make[0], make[2], make[3]], [queried, 1, 0]);
        assert_eq!(calls(&e, 0x7308), vec![vec![queried]]);
        assert!(calls(&e, 0x7200).is_empty());
        // Without buffer data: reported and deleted.
        e.mem.set_u32(made, 0);
        assert_eq!(e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![
                BUFFER_DATA_FAILED_010F03F8,
                BUFFER_DATA_FAILED_ARGUMENT_01011584
            ]]
        );
        assert_eq!(calls(&e, 0x7200)[0][1], 1);
        // A failed query: reported and deleted.
        e.register(0x7400, |_, _| 0x8000_4002u32.into_ret());
        assert_eq!(e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680)[1],
            vec![QUERY_FAILED_010F0440, 0x5151]
        );
        assert_eq!(calls(&e, 0x7200).len(), 2);
        // A failed surface (no texture): the object deleted before anything else.
        assert_eq!(e.call(0x00e8_ff60, &args![0u32, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(calls(&e, 0x7200).len(), 3);
    }

    #[test]
    fn getters_answer_their_words() {
        let mut e = engine();
        let this = e.mem.alloc(0x1248);
        e.mem.set_u32(this + 0x6c, 1);
        e.mem.set_u32(this + 0x10e0, 2);
        e.mem.set_u32(this + 0x110c, 3);
        assert_eq!(e.call(0x00e9_02d0, &args![this]).u32(), 1);
        assert_eq!(e.call(0x00e9_0e40, &args![this]).u32(), 2);
        assert_eq!(e.call(0x00e9_1020, &args![this]).u32(), 0);
        e.mem.set_u8(this + 0x1100, 1);
        assert_eq!(e.call(0x00e9_1020, &args![this]).u32(), 3);
    }

    #[test]
    fn deleting_destructors_of_the_shader_texture_manager_and_render_state() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(ERROR_PASS_0126F74C, 0x99);
        assert_eq!(e.call(0x00e9_0560, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(ERROR_PASS_0126F74C), 0);
        assert_eq!(calls(&e, SHADER_DOWN_00E806C0), vec![vec![this]]);
        let device = object(&mut e, 0x8000, 8, &[(8, 0x7008)]);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager + 8, device);
        e.call(0x00e9_0ac0, &args![manager, 0u32]);
        assert_eq!(calls(&e, 0x7008), vec![vec![device]]);
        assert_eq!(e.mem.u32(manager + 8), 0);
        assert_eq!(e.mem.u32(manager), NI_REF_OBJECT_VTABLE);
        let state = e.mem.alloc(0x10);
        e.call(0x00e9_1160, &args![state, 1u32]);
        assert_eq!(e.mem.u32(state), NI_DX9_RENDER_STATE_VTABLE);
        assert_eq!(calls(&e, RENDER_STATE_DOWN_00E88B00), vec![vec![state]]);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![this, 0x40], vec![state, 0x1248]]
        );
    }

    #[test]
    fn fn_00e90970_and_fn_00e90a50_set_up_the_declaration() {
        let mut e = engine();
        let made = object(&mut e, 0x8000, 8, &[(4, 0x7004), (0x8c, 0x708c)]);
        e.register_double(SHADER_DECLARATION_CREATE_00E76700, move |_, _| {
            made.into_ret()
        });
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x8000);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x18);
        e.mem.set_u32(this + 0x28, old);
        assert_eq!(e.call(0x00e9_0970, &args![this]).u8(), 1);
        assert_eq!(
            calls(&e, SHADER_DECLARATION_CREATE_00E76700),
            vec![vec![0x18, 1, 1]]
        );
        assert_eq!(calls(&e, 0x7004), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0x28), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls(&e, 0x708c), vec![vec![made, 0, 0, 0, 2, 0]]);
        let ok = e.mem.alloc(4);
        e.register_double(SHADER_INTERFACE_INITIALIZE_00E80570, move |e, _| {
            e.mem.u32(ok).into_ret()
        });
        assert!(!e.call(0x00e9_0a50, &args![this]).bool());
        e.mem.set_u32(ok, 1);
        assert!(e.call(0x00e9_0a50, &args![this]).bool());
        assert_eq!(calls(&e, 0x708c).len(), 2);
        e.mem.set_u8(this + 0x10, 1);
        assert!(e.call(0x00e9_0a50, &args![this]).bool());
        assert_eq!(calls(&e, SHADER_INTERFACE_INITIALIZE_00E80570).len(), 2);
    }

    /// Renderer data answering `a4`, `ac`, `a8`, `9c` and `98` from its slots; `target` answers
    /// 0x77 from its slot 0x9c.
    fn texture_rig(e: &mut Engine, words: [u32; 5]) -> (u32, u32, u32) {
        let renderer = e.mem.alloc(0x200);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        for (slot, word) in [0xa4u32, 0xac, 0xa8, 0x9c, 0x98].iter().zip(words) {
            let answer = if word == 3 { 0x8e00 } else { word };
            e.register_double(0x7000 + slot, move |_, _| answer.into_ret());
        }
        let mut slots = vec![0u32; 0x40];
        for slot in [0x98u32, 0x9c, 0xa4, 0xa8, 0xac, 0xb4] {
            slots[(slot / 4) as usize] = 0x7000 + slot;
        }
        e.put_vtable(0x8000, &slots);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data, 0x8000);
        e.mem.set_u32(data + 0xc, 64);
        e.mem.set_u32(data + 0x10, 32);
        let mut target_slots = vec![0u32; 0x40];
        target_slots[0x9c / 4] = 0x7800;
        e.put_vtable(0x8e00, &target_slots);
        e.register(0x7800, |_, _| 0x77u32.into_ret());
        let texture = e.mem.alloc(0x44);
        e.mem.set_u32(texture + 0x24, data);
        let this = e.mem.alloc(0x10);
        (this, texture, data)
    }

    #[test]
    fn fn_00e90b10_answers_the_texture_and_its_flags() {
        let mut e = engine();
        // Slot 0xa4 answers an object (at 0x8e00, whose table is itself).
        let (this, texture, data) = texture_rig(&mut e, [3, 0, 0, 0, 0]);
        e.mem.set_u32(0x8e00, 0x8e00);
        let flags = e.mem.alloc(3);
        e.mem.write(flags, &[9, 9, 9]);
        assert_eq!(
            e.call(0x00e9_0b10, &args![this, 0u32, flags, flags + 1, flags + 2])
                .u32(),
            0
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 0, 0]);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0x77
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 0, 0]);
        e.mem.set_u32(data + 0x10, 48);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(e.mem.u8(flags + 2), 1);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION)[0],
            vec![e.mem.u32(RENDERER_011F4748) + 0x180]
        );
    }

    #[test]
    fn fn_00e90b10_without_a_target_reads_the_levels() {
        let mut e = engine();
        // No 0xa4, 0xac or 0xa8 object; 0x9c answers 0, 0x98 answers 2.
        let (this, texture, data) = texture_rig(&mut e, [0, 0, 0, 0, 2]);
        let flags = e.mem.alloc(3);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 1, 0]);
        // Missing renderer data is made from the texture's source and marks `made`.
        e.mem.set_u32(texture + 0x24, 0);
        e.register(SOURCE_OF_00653290, |_, _| 0x5000u32.into_ret());
        e.register_double(MAKE_RENDERER_DATA_00E68EF0, move |_, _| data.into_ret());
        e.mem.set_u32(this + 0xc, 0x42);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(e.mem.u8(flags), 1);
        assert_eq!(
            calls(&e, SOURCE_OF_00653290),
            vec![vec![texture, SOURCE_KIND_011F444C]]
        );
        assert_eq!(
            calls(&e, MAKE_RENDERER_DATA_00E68EF0),
            vec![vec![0x5000, 0x42]]
        );
        // No source: null, the lock left.
        e.register(SOURCE_OF_00653290, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn fn_00e90b10_touches_the_other_object() {
        let mut e = engine();
        // 0xa8 answers the object at 0x8e00 (its own table; slot 0xb4 recorded).
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 3, 0, 1]);
        e.mem.set_u32(0x8e00, 0x8e00);
        e.mem.set_u32(0x8e00 + 0xb4, 0x78b4);
        e.register(0x78b4, |_, _| Ret::default());
        let flags = e.mem.alloc(3);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        // Slot 0x9c of the data answers 0: the other object's 0xb4 and `made`.
        assert_eq!(calls(&e, 0x78b4), vec![vec![0x8e00]]);
        assert_eq!(e.mem.bytes(flags, 3), vec![1, 0, 0]);
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 3, 1, 1]);
        e.mem.set_u32(0x8e00, 0x8e00);
        e.mem.set_u32(0x8e00 + 0xb4, 0x78b4);
        e.mem.set_u8(texture + 0x40, 1);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(calls(&e, 0x78b4).len(), 1);
        e.mem.set_u8(texture + 0x40, 0);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            1
        );
        assert_eq!(calls(&e, 0x78b4).len(), 2);
        assert_eq!(e.mem.u8(flags), 0);
    }

    #[test]
    fn fn_00e90c70_answers_whether_the_target_has_a_texture() {
        let mut e = engine();
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 0, 0, 0]);
        assert!(!e.call(0x00e9_0c70, &args![this, 0u32, 0u32, 0u32]).bool());
        assert!(e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        let (this, texture, _) = texture_rig(&mut e, [0, 3, 0, 0, 0]);
        e.mem.set_u32(0x8e00, 0x8e00);
        assert!(e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        e.register(0x7800, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        e.mem.set_u32(texture + 0x24, 0);
        e.register(SOURCE_OF_00653290, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
    }

    fn render_state(e: &mut Engine) -> (u32, u32) {
        let device = object(
            e,
            0x8000,
            8,
            &[
                (SET_VERTEX_SHADER, 0x7170),
                (SET_VERTEX_DECLARATION, 0x715c),
                (SET_SAMPLER_STATE, 0x7114),
            ],
        );
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state + STATE_DEVICE, device);
        (state, device)
    }

    #[test]
    fn fn_00e90d80_resets_the_declaration_words() {
        let mut e = engine();
        let (state, _) = render_state(&mut e);
        e.mem.set_u8(state + 0x1100, 1);
        e.mem.set_u32(state + 0x110c, 5);
        e.call(0x00e9_0d80, &args![state]);
        assert_eq!(e.mem.u8(state + 0x1100), 0);
        assert_eq!(e.mem.u32(state + 0x110c), 0);
        assert_eq!(e.mem.u32(state + 0x1108), 1);
        assert_eq!(e.mem.u32(state + 0x1110), 1);
        assert_eq!(calls(&e, RESET_SHADERS_00E87A50), vec![vec![state]]);
    }

    #[test]
    fn vertex_shader_words_follow_the_device() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.call(0x00e9_0e00, &args![state, 0x11u32, 0u32]);
        e.call(0x00e9_0e00, &args![state, 0x11u32, 0u32]);
        assert_eq!(calls(&e, 0x7170), vec![vec![device, 0x11]]);
        e.call(0x00e9_0e00, &args![state, 0x22u32, 1u32]);
        assert_eq!(e.mem.u32(state + 0x10e4), 0x11);
        assert_eq!(e.mem.u32(state + 0x10e0), 0x22);
        let mut slots = vec![0u32; 0x40];
        slots[0x8c / 4] = 0x758c;
        e.put_vtable(0x8c00, &slots);
        e.register(0x758c, |_, _| Ret::default());
        e.mem.set_u32(state, 0x8c00);
        e.call(0x00e9_0e50, &args![state]);
        assert_eq!(calls(&e, 0x758c), vec![vec![state, 0x11, 0]]);
        e.call(0x00e9_0e70, &args![state, 0x11u32]);
        assert_eq!(e.mem.u32(state + 0x10e4), 0);
        assert_eq!(e.mem.u32(state + 0x10e0), 0x22);
        e.call(0x00e9_0e70, &args![state, 0x22u32]);
        assert_eq!(e.mem.u32(state + 0x10e0), 0);
        assert_eq!(calls(&e, 0x7170).last().unwrap(), &vec![device, 0]);
    }

    #[test]
    fn vertex_declaration_words_follow_the_device() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.call(0x00e9_0fc0, &args![state, 0u32, 1u32]);
        assert_eq!(e.mem.u8(state + 0x1100), 0);
        e.call(0x00e9_0fc0, &args![state, 0x31u32, 0u32]);
        e.call(0x00e9_0fc0, &args![state, 0x31u32, 1u32]);
        assert_eq!(calls(&e, 0x715c), vec![vec![device, 0x31]]);
        assert_eq!(e.mem.u8(state + 0x1100), 1);
        e.call(0x00e9_0fc0, &args![state, 0x32u32, 1u32]);
        assert_eq!(e.mem.u32(state + 0x1110), 0x31);
        assert_eq!(e.mem.u32(state + 0x110c), 0x32);
        e.call(0x00e9_1060, &args![state, 0x31u32]);
        assert_eq!(e.mem.u32(state + 0x1110), 0);
        e.mem.set_u8(state + 0x1100, 0);
        e.call(0x00e9_1060, &args![state, 0x32u32]);
        assert_eq!(e.mem.u32(state + 0x110c), 0x32);
        e.mem.set_u8(state + 0x1100, 1);
        e.call(0x00e9_1060, &args![state, 0x32u32]);
        assert_eq!(e.mem.u32(state + 0x110c), 0);
    }

    #[test]
    fn fn_00e910a0_caches_sampler_states() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 6, 3);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 7, 5);
        e.call(0x00e9_10a0, &args![state, 2u32, 6u32, 9u32, 0u32]);
        e.call(0x00e9_10a0, &args![state, 2u32, 6u32, 9u32, 0u32]);
        e.call(0x00e9_10a0, &args![state, 2u32, 7u32, 9u32, 0u32]);
        assert_eq!(calls(&e, 0x7114), vec![vec![device, 2, 6, 9]]);
        assert_eq!(e.mem.u32(state + 0xe20 + (2 * 5 + 3) * 8), 9);
    }

    #[test]
    fn fn_00e911e0_compacts_the_pairs_sets_states_and_packs_the_colour() {
        let mut e = engine();
        e.map(0x011b_f000, 0x2000);
        e.map(0x0101_e000, 0x1000);
        e.mem.write(SCALE_0101E568, &255.0f64.to_le_bytes());
        for (i, (state, value)) in [(1u32, 10u32), (0xbe, 0), (2, 20), (0xffff_ffff, 0)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32, *state);
            e.mem
                .set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32 + 4, *value);
        }
        e.mem.set_u32(DEFAULT_COLOR_011F4998, 1.0f32.to_bits());
        e.mem.set_u32(DEFAULT_COLOR_011F4998 + 4, 0.5f32.to_bits());
        e.mem.set_u32(DEFAULT_COLOR_011F4998 + 8, 0.25f32.to_bits());
        let mut slots = vec![0u32; 0x40];
        for slot in [0x58u32, 0x68, 0x70, 0xbc, 0xd8] {
            slots[(slot / 4) as usize] = 0x7500 + slot;
            e.register(0x7500 + slot, |_, _| Ret::default());
        }
        e.put_vtable(0x8a00, &slots);
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state, 0x8a00);
        e.mem.set_u8(state + 8, 1);
        e.mem.set_u32(state + 0x38, 0x38);
        e.mem.set_u32(state + 0x3c, 0x3c);
        e.call(0x00e9_11e0, &args![state]);
        let pairs: Vec<u32> = (0..8)
            .map(|i| e.mem.u32(STATE_PAIRS_011BFCE8 + 4 * i))
            .collect();
        assert_eq!(pairs, vec![1, 10, 2, 20, 0xffff_ffff, 0, 0xffff_ffff, 0]);
        let set: Vec<(u32, u32)> = calls(&e, 0x7568).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(set.len(), 21);
        assert_eq!(set[4], (0x13, 0x38));
        assert_eq!(set[5], (0x14, 0x3c));
        assert_eq!(set[14], (0x23, 3));
        assert_eq!(set[20], (0x8d, 1));
        assert_eq!(calls(&e, MEMBER_RESET_00E94130), vec![vec![state + 0xf8]]);
        assert_eq!(e.mem.u32(state + 0x90), 0.5f32.to_bits());
        assert_eq!(e.mem.u32(state + 0x98), 0xffff_7f3f);
        // With bit 0x4000 the pairs stay.
        e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8, 0xbe);
        e.mem.set_u32(state + 0x1138, 0x4000);
        e.mem.set_u8(state + 8, 0);
        e.call(0x00e9_11e0, &args![state]);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 8), 0xbe);
        assert_eq!(calls(&e, 0x7568)[35][2], 0);
    }

    #[test]
    fn fn_00e914e0_derives_the_capability_flags() {
        let mut e = engine();
        let renderer = e.mem.alloc(0x5d8);
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state + 0x10fc, renderer);
        e.mem.set_u32(state + 8, 0xf0);
        e.mem.set_u32(state + 0x113c, 0x10_0100);
        e.mem.set_u32(state + 0x1144, 0x1);
        e.mem.set_u32(state + 0x1148, 0x2);
        e.mem.set_u32(state + 0x20, 1);
        e.mem.set_u32(state + 0x20 + 4 * 10, 3);
        e.mem.set_u8(state + 0x1164, 0x10);
        e.mem.set_u8(renderer + 0x5d4, 1);
        e.call(0x00e9_14e0, &args![state]);
        assert_eq!(e.mem.u32(state + 8), 0xf5);
        assert_eq!(e.mem.u32(state + 0x6c), 0x401);
        assert_eq!(e.mem.u32(state + 0x70), 0x400);
        assert_eq!(e.mem.u8(state + 0x1114), 1);
        e.mem.set_u32(state + 0x113c, 0x100);
        e.mem.set_u8(state + 0x1164, 0);
        e.mem.set_u8(renderer + 0x5d5, 1);
        e.call(0x00e9_14e0, &args![state]);
        assert_eq!(e.mem.u32(state + 8), 0xf0);
        assert_eq!(e.mem.u8(state + 0x1114), 0);
    }
}
