//! `NiXenonRenderer/run_00e89ed0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The bucket allocation `00aa1070` (`cdecl`, size), `memset` (`00ec61c0`), the set-up that
/// ends the constructor (`00e8b170`, unnamed; `thiscall`, no stack words) and the map's
/// virtual tables (RTTI `NiTMapBase<..., NiLight *, NiDX9LightManager::LightEntry *>`, then
/// `NiTPointerMap<NiLight *, NiDX9LightManager::LightEntry *>`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const FINISH_00E8B170: u32 = 0x00e8_b170;
const LIGHT_MAP_BASE_VTABLE: u32 = 0x010e_fdd8;
const LIGHT_MAP_VTABLE: u32 = 0x010e_fdf8;

// Translated from 00e8bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the light manager constructor (its map at +0
/// holds `NiDX9LightManager::LightEntry` records by the RTTI), `thiscall` with two words: the
/// shader library and the device. The map (0x25 buckets of `00aa1070`, cleared with `memset`;
/// base then final virtual table), +0x10, +0x14, +0x18 cleared, the library at +0x20, -1 at
/// +0x24, the device at +0x1c (counted: its virtual slot 4), +0x38 cleared and the 0x200 bytes
/// from +0x3c cleared (`memset`), then `00e8b170`. Answers `this`.
pub fn fn_00e8bae0(e: &mut Engine, this: Ptr, library: u32, device: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, LIGHT_MAP_BASE_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0x25);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
    e.mem.set_u32(base.wrapping_add(8), buckets);
    let size = e.mem.u32(base.wrapping_add(4)).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u32(base, LIGHT_MAP_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x18), 0);
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    e.mem.set_u32(base.wrapping_add(0x20), library);
    e.mem.set_u32(base.wrapping_add(0x24), 0xffff_ffff);
    e.mem.set_u32(base.wrapping_add(0x1c), device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(base.wrapping_add(0x38), 0);
    e.call(MEMSET, &args![base.wrapping_add(0x3c), 0u32, 0x200u32]);
    e.call(FINISH_00E8B170, &args![base]);
    this
}

// Translated from 00e8b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the light manager's reset, `thiscall`, no
/// stack words. Calls the render state's (+0x20) virtual slot 0x68 with (0x8b, 0, 0, 0) and
/// (0x89, 0, 0, 0), clears +0x30, the byte +0x2d and +0x34, the same slot with 0x94, 0x93 and
/// 0x91, then clears the byte +0x2c and copies +0x24 to +0x28.
pub fn fn_00e8b170(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let set_state = |e: &mut Engine, state: u32| {
        let render_state = e.mem.u32(base + 0x20);
        e.vcall(render_state, 0x68, &args![state, 0u32, 0u32, 0u32]);
    };
    set_state(e, 0x8b);
    set_state(e, 0x89);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u8(base + 0x2d, 0);
    e.mem.set_u32(base + 0x34, 0);
    set_state(e, 0x94);
    set_state(e, 0x93);
    set_state(e, 0x91);
    e.mem.set_u8(base + 0x2c, 0);
    let value = e.mem.u32(base + 0x24);
    e.mem.set_u32(base + 0x28, value);
}

/// `NiImageConverter::GetImageConverter` (`00a76640`, no arguments): the converter whose
/// virtual slot 0x10 (`thiscall`, two words: source format, candidate format) answers whether
/// it can convert to the candidate, slot 0xc reads an image file (name, 0, 0) and slot 0x14
/// converts pixel data (data, format, data, flag).
const GET_IMAGE_CONVERTER: u32 = 0x00a7_6640;
/// `00e8a240` (`thiscall`, no stack words): the number of a pixel format's channels before
/// the first whose type is 0x13 (at most 4).
const CHANNEL_COUNT_00E8A240: u32 = 0x00e8_a240;
/// The texture data base constructor and destructor bodies (`00a5d3a0`, `00a5d3d0`,
/// `thiscall`, no stack words), `NiPixelFormat::NiPixelFormat` (`00a7cca0`) and the two
/// virtual tables the texture data constructor stores (base, then its own).
const TEXTURE_BASE_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const TEXTURE_BASE_DESTRUCTOR: u32 = 0x00a5_d3d0;
const PIXEL_FORMAT_CONSTRUCTOR: u32 = 0x00a7_cca0;
const TEXTURE_BASE_VTABLE: u32 = 0x010a_080c;
const TEXTURE_DATA_VTABLE: u32 = 0x010e_fb7c;
/// The cube map data's virtual table (stored by `00e8abf0` and `00e8aa40`).
const CUBE_MAP_DATA_VTABLE: u32 = 0x010e_fc34;
/// The cube map data's base constructor (`00e687a0`, `thiscall`, texture and renderer), its
/// base destructor body (`00e68830`), the base's mipmap set-up (`00e68760`, `thiscall`, one
/// word), the per-face copies from a source cube map (`00e8ab70`, `thiscall`, source and
/// face) and the per-level surface copy (`00e68150`, cdecl, four words).
const CUBE_BASE_CONSTRUCTOR: u32 = 0x00e6_87a0;
const CUBE_BASE_DESTRUCTOR: u32 = 0x00e6_8830;
const SET_MIPMAP_00E68760: u32 = 0x00e6_8760;
const COPY_FACE_00E8AB70: u32 = 0x00e8_ab70;
const COPY_SURFACE_00E68150: u32 = 0x00e6_8150;
/// `00aa13e0` (cdecl, size): allocation; `00aa1460` (cdecl, block and size): its release;
/// `00aa10f0` (cdecl, one word): the release of the map's bucket array.
const ALLOCATE: u32 = 0x00aa_13e0;
const DEALLOCATE: u32 = 0x00aa_1460;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `00a29680` (cdecl, format and words): the debug print. `NiXenonErrorString`
/// (`00e89d50`, cdecl, one word: an `HRESULT`).
const DEBUG_PRINT: u32 = 0x00a2_9680;
const ERROR_STRING: u32 = 0x00e8_9d50;
/// The three messages: "NiDX9SourceCubeMapData::Create> No Cube Map texture formats
/// supported\n", "Failed to find texture %s\n", the `CreateCubeTexture` failure and the
/// `GetSurfaceLevel` failure.
const NO_CUBE_FORMATS_MESSAGE: u32 = 0x010e_fd88;
const TEXTURE_NOT_FOUND_MESSAGE: u32 = 0x010e_d4dc;
const CREATE_CUBE_FAILED_MESSAGE: u32 = 0x010e_fcf0;
const SURFACE_LEVEL_FAILED_MESSAGE: u32 = 0x010e_fd38;
/// `00a62a10` (cdecl, texture, error code, 1): the replacement pixel data for a texture that
/// failed; `00e7bd20` (cdecl, one word: an `NiPixelFormat`): its Direct3D format.
const REPLACEMENT_DATA: u32 = 0x00a6_2a10;
const D3D_FORMAT_OF: u32 = 0x00e7_bd20;
/// The renderer singleton (`011f4748`; +0x5c4 adapter, +0x5c8 device type, +0x5d0 adapter
/// format), the Direct3D object (`0126f0d8`), the dynamic-texture flag (`0126f0d0`), the
/// texture memory counters (`0126f9bc` bytes, `0126f9c0` page padding) and the flag at
/// `011f4508` the converted cube map's mipmap choice reads.
const RENDERER: u32 = 0x011f_4748;
const DIRECT3D: u32 = 0x0126_f0d8;
const DYNAMIC_TEXTURES: u32 = 0x0126_f0d0;
const TEXTURE_BYTES: u32 = 0x0126_f9bc;
const TEXTURE_PADDING: u32 = 0x0126_f9c0;
const MIPMAP_FLAG_011F4508: u32 = 0x011f_4508;
/// The head of the renderer's texture list (`011f4500`; next at +0x2c, the texture's data
/// object at +0x24).
const TEXTURE_LIST_HEAD: u32 = 0x011f_4500;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The list nodes' pool lock (`011c5f80` owner, `011c5f84` count; `0040fbf0` takes it,
/// `thiscall`, one word) and the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_LOCK_TAKE: u32 = 0x0040_fbf0;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
/// `00ee8ac0` (`thiscall`): the light map's first item (next, key, value); `00e91fd0`
/// (`thiscall`, one word: the light): drops a light; `00a53c30` (`thiscall`, the light and a
/// slot for its entry): the map's removal; `0057c730` (`thiscall`, a slot holding the light):
/// the list's removal; `004ed900` (`thiscall`): the list's `RemoveAll`; `00b601a0`
/// (`thiscall`): the map's clean-up.
const LIGHT_MAP_FIRST: u32 = 0x00ee_8ac0;
const LIGHT_DROP_00E91FD0: u32 = 0x00e9_1fd0;
const LIGHT_MAP_REMOVE: u32 = 0x00a5_3c30;
const LIGHT_LIST_REMOVE: u32 = 0x0057_c730;
const LIGHT_LIST_REMOVE_ALL: u32 = 0x004e_d900;
const LIGHT_MAP_CLEAN_UP: u32 = 0x00b6_01a0;

/// The first non-zero word of `table` at the given byte offsets, or 0.
fn first_set(e: &Engine, table: u32, offsets: &[u32]) -> u32 {
    for &offset in offsets {
        let value = e.mem.u32(table.wrapping_add(offset));
        if value != 0 {
            return value;
        }
    }
    0
}

// Translated from 00e89ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): picks a pixel format from the renderer's format
/// table `table` (format pointers from +0 to +0x54) for the request at `request` (+0 a kind,
/// +4 a second choice), cdecl with two words. Kind 6 counts as 2 and choice 3 as 0; kinds 0
/// and 5 first try +0x18 (choice 0) or +0x1c (choices 1 and 2), then each kind (the
/// compiler's jump table at `00e8a0d8`) walks its own order of entries and answers the first
/// that is set, or 0.
pub fn fn_00e89ed0(e: &mut Engine, request: Ptr, table: Ptr) -> u32 {
    let mut kind = e.mem.u32(request.addr());
    let mut choice = e.mem.i32(request.addr().wrapping_add(4));
    if kind == 6 {
        kind = 2;
    }
    if choice == 3 {
        choice = 0;
    }
    let t = table.addr();
    let kind_0_or_5 = kind == 0 || kind == 5;
    if kind_0_or_5 && choice == 0 {
        let value = e.mem.u32(t.wrapping_add(0x18));
        if value != 0 {
            return value;
        }
    } else if kind_0_or_5 && (choice == 1 || choice == 2) {
        let value = e.mem.u32(t.wrapping_add(0x1c));
        if value != 0 {
            return value;
        }
    }
    // The orders several kinds share (`00e89ffe`, `00e8a044`, `00e8a04f`).
    let by_choice = |e: &Engine, choice: i32| match choice {
        0 => first_set(e, t, &[0x4, 0x0, 0x8, 0x14, 0xc, 0x10]),
        1 => first_set(e, t, &[0xc, 0x10, 0x14]),
        2 => first_set(e, t, &[0x10, 0x14]),
        _ => 0,
    };
    // Kind 2 (`00e89fb0`), also the end of kind 4.
    let kind_2 = |e: &Engine, choice: i32| match choice {
        0 => first_set(e, t, &[0x8, 0x14, 0x4, 0x0, 0xc, 0x10]),
        1 => first_set(e, t, &[0x14, 0xc, 0x10]),
        2 => first_set(e, t, &[0x14, 0x10]),
        _ => 0,
    };
    match kind {
        0 | 3 | 5 => {
            if choice >= 0 {
                if choice <= 1 {
                    let value = e.mem.u32(t.wrapping_add(0x20));
                    if value != 0 {
                        return value;
                    }
                } else if choice == 2 {
                    let value = e.mem.u32(t.wrapping_add(0x24));
                    if value != 0 {
                        return value;
                    }
                }
            }
            by_choice(e, choice)
        }
        1 => by_choice(e, choice),
        2 => kind_2(e, choice),
        4 => {
            let value = e.mem.u32(t.wrapping_add(0x34));
            if value != 0 {
                return value;
            }
            let value = if choice == 0 {
                first_set(e, t, &[0x2c, 0x30])
            } else {
                first_set(e, t, &[0x30, 0x2c])
            };
            if value != 0 {
                return value;
            }
            kind_2(e, choice)
        }
        7 => {
            if choice == 0 {
                let value = e.mem.u32(t.wrapping_add(0x38));
                if value != 0 {
                    return value;
                }
                by_choice(e, 0)
            } else {
                let value = e.mem.u32(t.wrapping_add(0x3c));
                if value != 0 {
                    return value;
                }
                by_choice(e, 1)
            }
        }
        8 => first_set(e, t, &[0x40, 0x4c, 0x44, 0x50, 0x48, 0x54]),
        9 => first_set(e, t, &[0x4c, 0x50, 0x54, 0x40, 0x44, 0x48]),
        10 => first_set(e, t, &[0x44, 0x50, 0x48, 0x54]),
        11 => first_set(e, t, &[0x50, 0x54, 0x44, 0x48]),
        12 | 13 => first_set(e, t, &[0x48, 0x54]),
        14 => first_set(e, t, &[0x54, 0x48]),
        // Kind 6 (its table entry answers 0; it was turned into 2 above) and kinds above 14.
        _ => 0,
    }
}

// Translated from 00e8a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): reads channel `index` of the pixel format
/// `this` (12-byte records from +0x14): the byte at +0x1d into `out_signed`, the byte at
/// +0x1c into `out_bits`, the word at +0x18 into `out_word` and the type (+0x14) into
/// `out_type`, in that order, and answers whether the type is not 0x13. `thiscall` with five
/// words.
pub fn fn_00e8a1f0(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    out_type: Ptr,
    out_word: Ptr,
    out_bits: Ptr,
    out_signed: Ptr,
) -> bool {
    let channel = this.addr().wrapping_add(index.wrapping_mul(12));
    let signed = e.mem.u8(channel.wrapping_add(0x1d));
    e.mem.set_u8(out_signed.addr(), signed);
    let bits = e.mem.u8(channel.wrapping_add(0x1c));
    e.mem.set_u8(out_bits.addr(), bits);
    let word = e.mem.u32(channel.wrapping_add(0x18));
    e.mem.set_u32(out_word.addr(), word);
    let kind = e.mem.u32(channel.wrapping_add(0x14));
    e.mem.set_u32(out_type.addr(), kind);
    kind != 0x13
}

// Translated from 00e8a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the texture data constructor (`thiscall`, the
/// texture and the renderer): the base constructor `00a5d3a0`, the base virtual table, the
/// texture at +8, +0xc and +0x10 cleared, the pixel format at +0x14 constructed
/// (`NiPixelFormat::NiPixelFormat`), the bytes +0x58..+0x5a set to 0xff and +0x5b cleared,
/// +0x64 and +0x68 cleared, then its own virtual table and the renderer at +0x60. Answers
/// `this`.
pub fn fn_00e8a260(e: &mut Engine, this: Ptr, texture: u32, renderer: u32) -> Ptr {
    let base = this.addr();
    e.call(TEXTURE_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, TEXTURE_BASE_VTABLE);
    e.mem.set_u32(base.wrapping_add(8), texture);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![base.wrapping_add(0x14)]);
    e.mem.set_u8(base.wrapping_add(0x58), 0xff);
    e.mem.set_u8(base.wrapping_add(0x59), 0xff);
    e.mem.set_u8(base.wrapping_add(0x5a), 0xff);
    e.mem.set_u8(base.wrapping_add(0x5b), 0);
    e.mem.set_u32(base.wrapping_add(0x64), 0);
    e.mem.set_u32(base.wrapping_add(0x68), 0);
    e.mem.set_u32(base, TEXTURE_DATA_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x60), renderer);
    this
}

// Translated from 00e8a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the texture data destructor body (`thiscall`,
/// no stack words). With a Direct3D texture at +0x64, the renderer's (+0x60) object at
/// +0x8b8 lets it go (virtual slot 0xe4, the texture), the texture is released (`IUnknown`
/// slot 8) and +0x64 cleared; then the base virtual table and the base destructor `00a5d3d0`.
pub fn fn_00e8a2b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let texture = e.mem.u32(base.wrapping_add(0x64));
    e.mem.set_u32(base, TEXTURE_DATA_VTABLE);
    if texture != 0 {
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let holder = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.vcall(holder, 0xe4, &args![texture]);
        let texture = e.mem.u32(base.wrapping_add(0x64));
        e.vcall(texture, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x64), 0);
    }
    e.mem.set_u32(base, TEXTURE_BASE_VTABLE);
    e.call(TEXTURE_BASE_DESTRUCTOR, &args![this]);
}

/// One candidate of the format table: the candidate when it is set and the image converter
/// (virtual slot 0x10) accepts it.
fn accepted(e: &mut Engine, converter: u32, format: u32, table: u32, offset: u32) -> Option<u32> {
    let candidate = e.mem.u32(table.wrapping_add(offset));
    if candidate != 0 && e.vcall(converter, 0x10, &args![format, candidate]).bool() {
        Some(candidate)
    } else {
        None
    }
}

/// The first candidate at `offsets` the converter accepts.
fn first_accepted(
    e: &mut Engine,
    converter: u32,
    format: u32,
    table: u32,
    offsets: &[u32],
) -> Option<u32> {
    for &offset in offsets {
        if let Some(found) = accepted(e, converter, format, table, offset) {
            return Some(found);
        }
    }
    None
}

// Translated from 00e8a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): finds the renderer format (from the table
/// `table`, format pointers from +0 to +0x54) closest to the source pixel format `format`
/// that the image converter (`NiImageConverter::GetImageConverter`, virtual slot 0x10) can
/// convert to, following the preferences at `prefs` (+0 a kind, +4 a second choice); 0 when
/// none. A format with a palette (+8) answers 0 at once. Three stack words (`RET 0xc`); the
/// callers pass `ECX`, which it never reads (`_unused_this`).
///
/// Kind 4 tries +0x2c, +0x34, +0x30 (+0x34, +0x30, +0x2c for source types 1 and 9). Other
/// kinds go by the source type (the compiler's byte map `00e8a99c` and jump table
/// `00e8a978`): types 0 and 2 try the +0x20 group for kind 3, then +0x0..+0x14 ordered by
/// kind 1; types 4, 5, 6 try +0x20, +0x24, +0x28 (not for kinds 1 and 2); 8 and 9 the
/// +0x2c..+0x34 group; 11 +0x3c or +0x38 by the second choice; 7 and 10 answer 0. Types 1,
/// 3, 4, 5, 6, 11, 12, 13, 14 then go on: when the first channel (`00e8a1f0`) is not type
/// 0x13 and its word is 1 or 2, the 16- or 32-bit groups from +0x40 by the channel count
/// (`00e8a240`), then the +0x20 group for kind 3, then the +0xc..+0x14 group ordered by kind
/// 1 and the second choice.
pub fn fn_00e8a300(e: &mut Engine, _unused_this: Ptr, format: Ptr, prefs: Ptr, table: Ptr) -> u32 {
    let format = format.addr();
    let prefs = prefs.addr();
    let table = table.addr();
    let source_type = e.mem.u32(format.wrapping_add(4));
    let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
    if e.mem.u32(format.wrapping_add(8)) != 0 {
        return 0;
    }
    let kind = e.mem.u32(prefs);
    if kind == 4 {
        let order: &[u32] = if source_type == 9 || source_type == 1 {
            &[0x34, 0x30, 0x2c]
        } else {
            &[0x2c, 0x34, 0x30]
        };
        return first_accepted(e, converter, format, table, order).unwrap_or(0);
    }
    if source_type > 0xe {
        return 0;
    }
    // The byte map `00e8a99c` sends each source type to one of nine cases.
    match source_type {
        0 | 2 => {
            if kind == 3 {
                let order = [0x20, 0x24, 0x28];
                if let Some(found) = first_accepted(e, converter, format, table, &order) {
                    return found;
                }
            }
            let order: &[u32] = if e.mem.u32(prefs) == 1 {
                &[0x4, 0x0, 0x8, 0x14, 0xc, 0x10]
            } else {
                &[0x8, 0x14, 0x4, 0x0, 0xc, 0x10]
            };
            return first_accepted(e, converter, format, table, order).unwrap_or(0);
        }
        4..=6 => {
            if kind != 1 && kind != 2 {
                let offset = match source_type {
                    4 => 0x20,
                    5 => 0x24,
                    _ => 0x28,
                };
                if let Some(found) = accepted(e, converter, format, table, offset) {
                    return found;
                }
            }
        }
        8 => {
            let order = [0x2c, 0x34, 0x30];
            return first_accepted(e, converter, format, table, &order).unwrap_or(0);
        }
        9 => {
            let order = [0x30, 0x34, 0x2c];
            return first_accepted(e, converter, format, table, &order).unwrap_or(0);
        }
        11 => {
            let offset = if e.mem.u32(prefs.wrapping_add(4)) == 2 {
                0x3c
            } else {
                0x38
            };
            if let Some(found) = accepted(e, converter, format, table, offset) {
                return found;
            }
        }
        7 | 10 => return 0,
        // 1, 3, 12, 13, 14: straight on to the channel test.
        _ => {}
    }

    // `00e8a5dc`: the first channel, read through the stack slots the exe passes (its type
    // slot starts at 0x13, its word slot at 5, its bits byte at 0).
    let (valid, channel_word, bits) = e.with_stack(16, |e, slots| {
        let slots = slots.addr();
        e.mem.set_u32(slots, 0x13);
        e.mem.set_u32(slots + 4, 5);
        e.mem.set_u8(slots + 8, 0);
        let valid = fn_00e8a1f0(
            e,
            Ptr::new(format),
            0,
            Ptr::new(slots),
            Ptr::new(slots + 4),
            Ptr::new(slots + 8),
            Ptr::new(slots + 12),
        );
        (valid, e.mem.u32(slots + 4), e.mem.u8(slots + 8))
    });
    if valid && (channel_word == 2 || channel_word == 1) {
        let count = e.call(CHANNEL_COUNT_00E8A240, &args![format]).u32();
        if bits == 0x10 {
            if count < 2 {
                let order = [0x40, 0x4c];
                if let Some(found) = first_accepted(e, converter, format, table, &order) {
                    return found;
                }
            }
            if count < 3 {
                let order = [0x44, 0x50];
                if let Some(found) = first_accepted(e, converter, format, table, &order) {
                    return found;
                }
            }
            let order = [0x48, 0x54];
            if let Some(found) = first_accepted(e, converter, format, table, &order) {
                return found;
            }
        } else if bits == 0x20 {
            for (offset, below) in [
                (0x4c, 2),
                (0x50, 3),
                (0x54, u32::MAX),
                (0x40, 2),
                (0x44, 3),
                (0x48, u32::MAX),
            ] {
                // `u32::MAX`: the candidate the exe tries whatever the count.
                if below == u32::MAX || count < below {
                    if let Some(found) = accepted(e, converter, format, table, offset) {
                        return found;
                    }
                }
            }
        }
    }

    // `00e8a7b3`.
    if e.mem.u32(prefs) == 3 {
        let order: &[u32] = if e.mem.u32(prefs.wrapping_add(4)) == 1 {
            &[0x20, 0x24, 0x28]
        } else {
            &[0x24, 0x28, 0x20]
        };
        if let Some(found) = first_accepted(e, converter, format, table, order) {
            return found;
        }
    }
    // `00e8a858`.
    if e.mem.u32(prefs) == 1 {
        let order: &[u32] = if e.mem.u32(prefs.wrapping_add(4)) == 1 {
            &[0xc, 0x10, 0x14]
        } else {
            &[0x10, 0x14, 0xc]
        };
        return first_accepted(e, converter, format, table, order).unwrap_or(0);
    }
    // `00e8a8ec`: +0x14 is tried before the second choice is read.
    if let Some(found) = accepted(e, converter, format, table, 0x14) {
        return found;
    }
    let order: &[u32] = if e.mem.u32(prefs.wrapping_add(4)) == 1 {
        &[0xc, 0x10]
    } else {
        &[0x10, 0xc]
    };
    first_accepted(e, converter, format, table, order).unwrap_or(0)
}

// Translated from 00e8a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): walks the renderer's texture list (head
/// `011f4500`, next at +0x2c) and deletes each texture's data object at +0x24 (cleared first,
/// then its virtual slot 0 with 1, the deleting destructor). cdecl with one word (the
/// callers push the renderer), which it never reads.
pub fn fn_00e8a9b0(e: &mut Engine, _unused_1: u32) {
    let mut texture = e.mem.u32(TEXTURE_LIST_HEAD);
    while texture != 0 {
        let data = e.mem.u32(texture.wrapping_add(0x24));
        if data != 0 {
            e.mem.set_u32(texture.wrapping_add(0x24), 0);
            e.vcall(data, 0, &args![1u32]);
        }
        texture = e.mem.u32(texture.wrapping_add(0x2c));
    }
}

// Translated from 00e8aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the cube map data's deleting destructor
/// (`thiscall`, one word of flags): its virtual table, the base destructor body `00e68830`,
/// and with flag bit 0 the 0x88-byte block released (`00aa1460`). Answers `this`.
pub fn fn_00e8aa40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), CUBE_MAP_DATA_VTABLE);
    e.call(CUBE_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this, 0x88u32]);
    }
    this
}

// Translated from 00e8aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): creates the cube map's Direct3D texture
/// (`thiscall`, no stack words). The renderer's (+0x60) device at +0x288 gets
/// `CreateCubeTexture` (virtual slot 0x64) with the edge length (+0xc), the levels (+0x68),
/// usage 0x200 when the dynamic-texture flag `0126f0d0` is set (else 0), the format (+0x20),
/// pool 1 when the flag is clear (else 0), an out slot and 0. A failure prints the message
/// with `NiXenonErrorString` and answers false; success keeps the texture at +0x64 and
/// answers true.
pub fn fn_00e8aa70(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let format = e.mem.u32(base.wrapping_add(0x20));
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let device = e.mem.u32(renderer.wrapping_add(0x288));
    let dynamic = e.mem.u8(DYNAMIC_TEXTURES);
    let pool = u32::from(dynamic == 0);
    let usage = if dynamic != 0 { 0x200u32 } else { 0 };
    let levels = e.mem.u32(base.wrapping_add(0x68));
    let edge = e.mem.u32(base.wrapping_add(0xc));
    let (result, texture) = e.with_stack(4, |e, out| {
        let result = e
            .vcall(
                device,
                0x64,
                &args![edge, levels, usage, format, pool, out, 0u32],
            )
            .i32();
        (result, e.mem.u32(out.addr()))
    });
    if result < 0 {
        let text = e.call(ERROR_STRING, &args![result]).u32();
        e.call(DEBUG_PRINT, &args![CREATE_CUBE_FAILED_MESSAGE, text]);
        return false;
    }
    e.mem.set_u32(base.wrapping_add(0x64), texture);
    true
}

// Translated from 00e8aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): copies one face of the pixel data `source`
/// into the cube texture (`thiscall`, two words). For each level below +0x68 it takes the
/// texture's (+0x64) surface (`GetCubeMapSurface`, virtual slot 0x48: face, level, out
/// slot), copies into it (`00e68150`: source, level, surface, face) and releases it
/// (`IUnknown` slot 8). A failure prints the message with the level and
/// `NiXenonErrorString` and stops. The exe's out slot is the stack word of `face`, whose
/// value it read before.
pub fn fn_00e8aaf0(e: &mut Engine, this: Ptr, source: u32, face: u32) {
    let base = this.addr();
    let texture = e.mem.u32(base.wrapping_add(0x64));
    let mut level = 0u32;
    if e.mem.u32(base.wrapping_add(0x68)) <= level {
        return;
    }
    e.with_stack(4, |e, out| loop {
        let result = e.vcall(texture, 0x48, &args![face, level, out]).i32();
        if result < 0 {
            let text = e.call(ERROR_STRING, &args![result]).u32();
            e.call(
                DEBUG_PRINT,
                &args![SURFACE_LEVEL_FAILED_MESSAGE, level, text],
            );
            return;
        }
        let surface = e.mem.u32(out.addr());
        e.call(COPY_SURFACE_00E68150, &args![source, level, surface, face]);
        let surface = e.mem.u32(out.addr());
        e.vcall(surface, 8, &args![]);
        level = level.wrapping_add(1);
        if level >= e.mem.u32(base.wrapping_add(0x68)) {
            return;
        }
    });
}

/// Adds a texture's size to the texture memory counter (`0126f9bc`) and the padding up to
/// the next 4 KB page to the padding counter (`0126f9c0`).
fn count_texture_memory(e: &mut Engine, size: u32) {
    let bytes = e.mem.u32(TEXTURE_BYTES);
    e.mem.set_u32(TEXTURE_BYTES, bytes.wrapping_add(size));
    let page = size & 0xffff_f000;
    let padding = if page == size {
        0
    } else {
        page.wrapping_sub(size).wrapping_add(0x1000)
    };
    let total = e.mem.u32(TEXTURE_PADDING);
    e.mem.set_u32(TEXTURE_PADDING, total.wrapping_add(padding));
}

/// Whether the first word behind the two pointers is the same, non-zero power of two (the
/// width and height of a square power-of-two image).
fn square_power_of_two(e: &Engine, width_at: u32, height_at: u32) -> bool {
    let width = e.mem.u32(e.mem.u32(width_at));
    let height = e.mem.u32(e.mem.u32(height_at));
    width == height && width != 0 && width & width.wrapping_sub(1) == 0
}

// Translated from 00e8abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): creates the cube map data of `texture` for
/// `renderer` (cdecl, two words; the messages name it `NiDX9SourceCubeMapData::Create`).
///
/// Without any cube map format in the renderer's table (+0x7b0, 0x16 entries) it prints
/// the message and answers 0. Otherwise a 0x88-byte object is made (`00e687a0`, its virtual
/// table, +0x84 cleared). The source is the texture's cube map (+0x38), else its pixel data
/// (+0x3c), else the file named at +0x34 read by the image converter (slot 0xc), else the
/// replacement data (`00a62a10` with 0x80000008, marking the object failed at +0x6c). A
/// square power-of-two source gets a format from `00e8a300`; with a cube map the Direct3D
/// format (`00e7bd20`) must pass `CheckDeviceFormat` (the Direct3D object's slot 0x28); every
/// failure takes replacement data (0x80000007, or 0x80000009 for a source that is not a
/// square power of two). A cube map source (not failed) is copied face by face
/// (`00e8ab70`); pixel data is converted (the image converter's slot 0x14) and copied level
/// by level per face (`00e8aaf0`) when it has six faces, its reference taken and dropped
/// (`InterlockedIncrement`/`InterlockedDecrement`, slot 4 at zero). Both count the texture
/// memory, store the object at the texture's +0x24 and answer it; when the Direct3D texture
/// cannot be made (`00e8aa70`) the object is deleted (slot 0 with 1) and 0 answered.
pub fn fn_00e8abf0(e: &mut Engine, texture: Ptr, renderer: Ptr) -> Ptr {
    let texture = texture.addr();
    let renderer = renderer.addr();
    let formats = renderer.wrapping_add(0x7b0);
    let mut index = 0u32;
    loop {
        if e.mem.u32(formats.wrapping_add(index * 4)) != 0 {
            break;
        }
        if e.mem.u32(formats.wrapping_add(index * 4 + 4)) != 0 {
            index += 1;
            break;
        }
        index += 2;
        if index >= 0x16 {
            break;
        }
    }
    if index == 0x16 {
        e.call(DEBUG_PRINT, &args![NO_CUBE_FORMATS_MESSAGE]);
        return Ptr::NULL;
    }

    let object = e.call(ALLOCATE, &args![0x88u32]).u32();
    if object != 0 {
        e.call(CUBE_BASE_CONSTRUCTOR, &args![object, texture, renderer]);
        e.mem.set_u32(object, CUBE_MAP_DATA_VTABLE);
        e.mem.set_u32(object.wrapping_add(0x84), 0);
    }
    let failed_at = object.wrapping_add(0x6c);
    let prefs = texture.wrapping_add(0x18);
    let cube = e.mem.u32(texture.wrapping_add(0x38));
    let mut data = 0u32;
    if cube == 0 {
        data = e.mem.u32(texture.wrapping_add(0x3c));
        if data == 0 {
            let name = e.mem.u32(texture.wrapping_add(0x34));
            if name != 0 {
                let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
                data = e.vcall(converter, 0xc, &args![name, 0u32, 0u32]).u32();
            } else {
                data = e
                    .call(REPLACEMENT_DATA, &args![texture, 0x8000_0008u32, 1u32])
                    .u32();
                e.mem.set_u8(failed_at, 1);
            }
        }
    }
    let mut chosen = 0u32;
    // `00e8ae35`: replacement data for an error code.
    let replace = |e: &mut Engine, code: u32| -> u32 {
        let data = e.call(REPLACEMENT_DATA, &args![texture, code, 1u32]).u32();
        e.mem.set_u8(failed_at, 1);
        data
    };
    if cube == 0 {
        if data == 0 {
            data = e
                .call(REPLACEMENT_DATA, &args![texture, 0x8000_0008u32, 1u32])
                .u32();
            e.mem.set_u8(failed_at, 1);
            let name = e.mem.u32(texture.wrapping_add(0x34));
            e.call(DEBUG_PRINT, &args![TEXTURE_NOT_FOUND_MESSAGE, name]);
        } else {
            let source_format = data.wrapping_add(8);
            if square_power_of_two(e, data.wrapping_add(0x54), data.wrapping_add(0x58)) {
                chosen = fn_00e8a300(
                    e,
                    Ptr::new(object),
                    Ptr::new(source_format),
                    Ptr::new(prefs),
                    Ptr::new(formats),
                );
                let kind = e.mem.u32(source_format.wrapping_add(4));
                if chosen == 0 || kind == 2 || kind == 3 {
                    data = replace(e, 0x8000_0007);
                }
            } else {
                data = replace(e, 0x8000_0009);
            }
        }
    } else {
        let source_format = cube.wrapping_add(0x14);
        let mut code = 0u32;
        if square_power_of_two(e, cube.wrapping_add(0x64), cube.wrapping_add(0x68)) {
            chosen = fn_00e8a300(
                e,
                Ptr::new(object),
                Ptr::new(source_format),
                Ptr::new(prefs),
                Ptr::new(formats),
            );
            if chosen == 0 {
                code = 0x8000_0007;
            }
        } else {
            code = 0x8000_0009;
        }
        if code != 0 {
            data = replace(e, code);
        }
        if e.mem.u8(failed_at) == 0 {
            let d3d_format = e.call(D3D_FORMAT_OF, &args![source_format]).u32();
            if d3d_format == 0 {
                data = replace(e, 0x8000_0007);
            } else {
                let renderer_singleton = e.mem.u32(RENDERER);
                let adapter_format = e.mem.u32(renderer_singleton.wrapping_add(0x5d0));
                let device_type = e.mem.u32(renderer_singleton.wrapping_add(0x5c8));
                let adapter = e.mem.u32(renderer_singleton.wrapping_add(0x5c4));
                let direct3d = e.mem.u32(DIRECT3D);
                let result = e
                    .vcall(
                        direct3d,
                        0x28,
                        &args![adapter, device_type, adapter_format, 0u32, 3u32, d3d_format],
                    )
                    .i32();
                if result >= 0 {
                    e.mem.set_u32(cube.wrapping_add(0x20), d3d_format);
                } else {
                    data = replace(e, 0x8000_0007);
                }
            }
        }
    }

    // `00e8ae46`.
    let failed = e.mem.u8(failed_at) != 0;
    let converted;
    if failed {
        let index = e.mem.u32(renderer.wrapping_add(0x880));
        chosen = e.mem.u32(formats.wrapping_add(index.wrapping_mul(4)));
        converted = data;
    } else if cube != 0 {
        for word in 0..0x11u32 {
            let value = e.mem.u32(cube.wrapping_add(0x14 + word * 4));
            e.mem.set_u32(object.wrapping_add(0x14 + word * 4), value);
        }
        let width = e.mem.u32(e.mem.u32(cube.wrapping_add(0x64)));
        e.mem.set_u32(object.wrapping_add(0xc), width);
        let height = e.mem.u32(e.mem.u32(cube.wrapping_add(0x68)));
        e.mem.set_u32(object.wrapping_add(0x10), height);
        let levels = if e.mem.u8(renderer.wrapping_add(0x6f4)) != 0 {
            e.mem.u32(cube.wrapping_add(0x74))
        } else {
            1
        };
        e.mem.set_u32(object.wrapping_add(0x68), levels);
        e.mem.set_u32(object.wrapping_add(0x60), renderer);
        e.mem
            .set_u8(object.wrapping_add(0x6d), u8::from(levels != 1));
        e.mem.set_u8(failed_at, 0);
        let mipmap = e.mem.u32(cube.wrapping_add(0x90));
        e.call(SET_MIPMAP_00E68760, &args![object, mipmap]);
        let level_sizes = e.mem.u32(cube.wrapping_add(0x6c));
        let level = e.mem.u32(cube.wrapping_add(0x74));
        let size = e
            .mem
            .u32(level_sizes.wrapping_add(level.wrapping_mul(4)))
            .wrapping_mul(e.mem.u32(cube.wrapping_add(0x7c)));
        e.mem.set_u32(object.wrapping_add(0x70), size);
        if fn_00e8aa70(e, Ptr::new(object)) && e.mem.u32(object.wrapping_add(0x64)) != 0 {
            let size = e.mem.u32(object.wrapping_add(0x70));
            count_texture_memory(e, size);
            for face in 0..6u32 {
                e.call(COPY_FACE_00E8AB70, &args![object, cube, face]);
            }
            let owner = e.mem.u32(object.wrapping_add(8));
            e.mem.set_u32(owner.wrapping_add(0x24), object);
            return Ptr::new(object);
        }
        e.vcall(object, 0, &args![1u32]);
        return Ptr::NULL;
    } else {
        let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
        let flag = u32::from(e.mem.u8(object.wrapping_add(0x6d)));
        converted = e
            .vcall(converter, 0x14, &args![data, chosen, data, flag])
            .u32();
    }

    // `00e8af5f`.
    let converted = if converted == 0 {
        let replacement = e
            .call(REPLACEMENT_DATA, &args![texture, 0x8000_0007u32, 1u32])
            .u32();
        let index = e.mem.u32(renderer.wrapping_add(0x880));
        chosen = e.mem.u32(formats.wrapping_add(index.wrapping_mul(4)));
        replacement
    } else {
        converted
    };
    let failed = e.mem.u8(failed_at) != 0;
    let mipmaps = e.mem.u32(data.wrapping_add(0x68));
    e.mem.set_u32(object.wrapping_add(0x84), mipmaps);
    for word in 0..0x11u32 {
        let value = e.mem.u32(chosen.wrapping_add(word * 4));
        e.mem.set_u32(object.wrapping_add(0x14 + word * 4), value);
    }
    let width = e.mem.u32(e.mem.u32(converted.wrapping_add(0x54)));
    e.mem.set_u32(object.wrapping_add(0x10), width);
    e.mem.set_u32(object.wrapping_add(0xc), width);
    let filter = e.mem.u32(prefs.wrapping_add(8));
    let renderer_mipmaps = e.mem.u8(renderer.wrapping_add(0x6f4)) != 0;
    let mipmapped = (!failed && filter == 1)
        || (filter == 2 && e.mem.u8(MIPMAP_FLAG_011F4508) != 0 && renderer_mipmaps);
    e.mem.set_u8(object.wrapping_add(0x6d), u8::from(mipmapped));
    let levels = if renderer_mipmaps {
        e.mem.u32(converted.wrapping_add(0x60))
    } else {
        1
    };
    e.mem.set_u32(object.wrapping_add(0x68), levels);
    if fn_00e8aa70(e, Ptr::new(object))
        && e.mem.u32(object.wrapping_add(0x64)) != 0
        && e.mem.u32(converted.wrapping_add(0x6c)) == 6
    {
        for face in 0..6u32 {
            fn_00e8aaf0(e, Ptr::new(object), converted, face);
        }
        let mipmap = e.mem.u32(data.wrapping_add(0x4c));
        e.call(SET_MIPMAP_00E68760, &args![object, mipmap]);
        let level_sizes = e.mem.u32(converted.wrapping_add(0x5c));
        let level = e.mem.u32(converted.wrapping_add(0x60));
        let size = e
            .mem
            .u32(level_sizes.wrapping_add(level.wrapping_mul(4)))
            .wrapping_mul(e.mem.u32(converted.wrapping_add(0x6c)));
        e.mem.set_u32(object.wrapping_add(0x70), size);
        count_texture_memory(e, size);
        let count = converted.wrapping_add(4);
        e.call(INTERLOCKED_INCREMENT, &args![count]);
        if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
            e.vcall(converted, 4, &args![]);
        }
        let owner = e.mem.u32(object.wrapping_add(8));
        e.mem.set_u32(owner.wrapping_add(0x24), object);
        return Ptr::new(object);
    }
    e.vcall(object, 0, &args![1u32]);
    Ptr::NULL
}

// Translated from 00e8b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): a map's item setter, virtual slot 0xc of many
/// `NiTMapBase` virtual tables (`thiscall` with three words; the map in `ECX` is not read):
/// the key at the item's +4 and the value at +8.
pub fn fn_00e8b490(e: &mut Engine, _unused_this: Ptr, item: Ptr, key: u32, value: u32) {
    e.mem.set_u32(item.addr().wrapping_add(4), key);
    e.mem.set_u32(item.addr().wrapping_add(8), value);
}

/// Takes the first node off the light manager's list (+0x10 head, +0x14 tail, +0x18 count),
/// gives the node back to its pool (under the pool lock `011c5f80`) and answers the light it
/// held.
fn pop_light_node(e: &mut Engine, this: u32) -> u32 {
    let node = e.mem.u32(this.wrapping_add(0x10));
    let next = e.mem.u32(node);
    e.mem.set_u32(this.wrapping_add(0x10), next);
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(4), 0);
    } else {
        e.mem.set_u32(this.wrapping_add(0x14), 0);
    }
    let light = e.mem.u32(node.wrapping_add(8));
    e.mem.set_u32(node.wrapping_add(8), 0);
    e.call(NODE_POOL_LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let free = e.mem.u32(NODE_POOL_FREE_LIST);
    e.mem.set_u32(node, free);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, node);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    let left = e.mem.u32(this.wrapping_add(0x18));
    e.mem.set_u32(this.wrapping_add(0x18), left.wrapping_sub(1));
    light
}

/// Clears the light index `index` in the light manager's bit set at +0x3c.
fn clear_light_bit(e: &mut Engine, this: u32, index: u32) {
    let word = this.wrapping_add(0x3c).wrapping_add((index >> 5) * 4);
    let bits = e.mem.u32(word);
    e.mem.set_u32(word, bits & !(1u32 << (index & 0x1f)));
}

// Translated from 00e8b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): turns off every light on the light manager's
/// list (`thiscall`, no stack words): each node is taken off, the device (+0x1c) gets
/// `LightEnable` (virtual slot 0xd4) with the light entry's (the light's +0xec) index
/// (+0x6c) and 0, and the entry's byte +0x71 is cleared. Then +0x28 takes +0x24.
pub fn fn_00e8b560(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x18)) != 0 {
        loop {
            let light = pop_light_node(e, base);
            let entry = e.mem.u32(light.wrapping_add(0xec));
            let device = e.mem.u32(base.wrapping_add(0x1c));
            let index = e.mem.u32(entry.wrapping_add(0x6c));
            e.vcall(device, 0xd4, &args![index, 0u32]);
            e.mem.set_u8(entry.wrapping_add(0x71), 0);
            if e.mem.u32(base.wrapping_add(0x18)) == 0 {
                break;
            }
        }
    }
    let value = e.mem.u32(base.wrapping_add(0x24));
    e.mem.set_u32(base.wrapping_add(0x28), value);
}

// Translated from 00e8b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the light manager's destructor body
/// (`thiscall`, no stack words). While the map (+0xc count) holds lights, its first item
/// (`00ee8ac0`) is dropped (`00e91fd0`), the light's +0xec cleared, the entry's index
/// cleared in the bit set and the 0x74-byte entry released; for an item without a next one
/// the exe also hashes the key (map slot 4) and scans the buckets past it, a result it does
/// not use. Then the list's nodes go back to their pool, the device (+0x1c) is released and
/// cleared, the list's `RemoveAll`, both map virtual tables with the map's clean-up
/// (`00b601a0`), and the bucket array (+8) is freed (`00aa10f0`).
pub fn fn_00e8b630(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    while e.mem.u32(base.wrapping_add(0xc)) != 0 {
        let item = e.call(LIGHT_MAP_FIRST, &args![this]).u32();
        let light = e.mem.u32(item.wrapping_add(4));
        let entry = e.mem.u32(item.wrapping_add(8));
        if e.mem.u32(item) == 0 {
            let mut index = e.vcall(base, 4, &args![light]).u32().wrapping_add(1);
            if index < e.mem.u32(base.wrapping_add(4)) {
                let buckets = e.mem.u32(base.wrapping_add(8));
                while e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4))) == 0 {
                    index = index.wrapping_add(1);
                    if index >= e.mem.u32(base.wrapping_add(4)) {
                        break;
                    }
                }
            }
        }
        e.call(LIGHT_DROP_00E91FD0, &args![this, light]);
        e.mem.set_u32(light.wrapping_add(0xec), 0);
        let index = e.mem.u32(entry.wrapping_add(0x6c));
        clear_light_bit(e, base, index);
        e.call(DEALLOCATE, &args![entry, 0x74u32]);
    }
    while e.mem.u32(base.wrapping_add(0x18)) != 0 {
        pop_light_node(e, base);
    }
    let device = e.mem.u32(base.wrapping_add(0x1c));
    if device != 0 {
        e.vcall(device, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x1c), 0);
    }
    e.call(LIGHT_LIST_REMOVE_ALL, &args![base.wrapping_add(0x10)]);
    e.mem.set_u32(base, LIGHT_MAP_VTABLE);
    e.call(LIGHT_MAP_CLEAN_UP, &args![this]);
    e.mem.set_u32(base, LIGHT_MAP_BASE_VTABLE);
    e.call(LIGHT_MAP_CLEAN_UP, &args![this]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
}

// Translated from 00e8bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): removes `light` from the light manager
/// (`thiscall`, one word). With an entry (the light's +0xec) the map removal (`00a53c30`)
/// runs, the device's `LightEnable` (slot 0xd4) gets the entry's index and 0, the entry's
/// byte +0x71 is cleared, the light dropped (`00e91fd0`) and taken off the list
/// (`0057c730`), +0xec cleared, the index cleared in the bit set and the 0x74-byte entry
/// released. Without one, only when the map removal finds an entry: dropped, off the list,
/// +0xec cleared and the found entry's index cleared. The map removal writes into the stack
/// word of `light`, which the exe first sets to the entry.
pub fn fn_00e8bd40(e: &mut Engine, this: Ptr, light: u32) {
    let base = this.addr();
    let entry = e.mem.u32(light.wrapping_add(0xec));
    e.with_stack(8, |e, slots| {
        let slot = slots.addr();
        let local = slot + 4;
        e.mem.set_u32(slot, entry);
        if entry != 0 {
            e.call(LIGHT_MAP_REMOVE, &args![this, light, slot]);
            let device = e.mem.u32(base.wrapping_add(0x1c));
            let index = e.mem.u32(entry.wrapping_add(0x6c));
            e.vcall(device, 0xd4, &args![index, 0u32]);
            e.mem.set_u8(entry.wrapping_add(0x71), 0);
            e.call(LIGHT_DROP_00E91FD0, &args![this, light]);
            e.mem.set_u32(slot, light);
            e.call(LIGHT_LIST_REMOVE, &args![base.wrapping_add(0x10), slot]);
            e.mem.set_u32(light.wrapping_add(0xec), 0);
            let index = e.mem.u32(entry.wrapping_add(0x6c));
            clear_light_bit(e, base, index);
            e.call(DEALLOCATE, &args![entry, 0x74u32]);
            return;
        }
        if e.call(LIGHT_MAP_REMOVE, &args![this, light, slot]).bool() {
            e.call(LIGHT_DROP_00E91FD0, &args![this, light]);
            e.mem.set_u32(local, light);
            e.call(LIGHT_LIST_REMOVE, &args![base.wrapping_add(0x10), local]);
            let found = e.mem.u32(slot);
            e.mem.set_u32(light.wrapping_add(0xec), 0);
            let index = e.mem.u32(found.wrapping_add(0x6c));
            clear_light_bit(e, base, index);
        }
    });
}

// Translated from 00e8bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): adds one to the word at +0x3c (`thiscall`, no
/// stack words).
pub fn fn_00e8bf80(e: &mut Engine, this: Ptr) {
    let at = this.addr().wrapping_add(0x3c);
    let value = e.mem.u32(at);
    e.mem.set_u32(at, value.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_bae0, fn_00e8bae0(Ptr, u32, u32) -> Ptr),
        entry!(0x00e8_b170, fn_00e8b170(Ptr)),
        entry!(0x00e8_9ed0, fn_00e89ed0(Ptr, Ptr) -> u32),
        entry!(0x00e8_a1f0, fn_00e8a1f0(Ptr, u32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e8_a260, fn_00e8a260(Ptr, u32, u32) -> Ptr),
        entry!(0x00e8_a2b0, fn_00e8a2b0(Ptr)),
        entry!(0x00e8_a300, fn_00e8a300(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e8_a9b0, fn_00e8a9b0(u32)),
        entry!(0x00e8_aa40, fn_00e8aa40(Ptr, u8) -> Ptr),
        entry!(0x00e8_aa70, fn_00e8aa70(Ptr) -> bool),
        entry!(0x00e8_aaf0, fn_00e8aaf0(Ptr, u32, u32)),
        entry!(0x00e8_abf0, fn_00e8abf0(Ptr, Ptr) -> Ptr),
        entry!(0x00e8_b490, fn_00e8b490(Ptr, Ptr, u32, u32)),
        entry!(0x00e8_b560, fn_00e8b560(Ptr)),
        entry!(0x00e8_b630, fn_00e8b630(Ptr)),
        entry!(0x00e8_bd40, fn_00e8bd40(Ptr, u32)),
        entry!(0x00e8_bf80, fn_00e8bf80(Ptr)),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e8bae0_builds_the_light_manager() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(FINISH_00E8B170, |_, _| Ret::default());
        e.put_vtable(0x6000, &[0, 0x6100]);
        e.register(0x6100, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x23c);
        assert_eq!(
            e.call(0x00e8_bae0, &args![this, 0x99u32, device]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), LIGHT_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 0x20), 0x99);
        assert_eq!(e.mem.u32(this + 0x1c), device);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ffff);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6100, vec![device])));
        assert!(log.contains(&(MEMSET, vec![this + 0x3c, 0, 0x200])));
        assert_eq!(log.last().unwrap(), &(FINISH_00E8B170, vec![this]));
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e8b170_resets_the_light_states() {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x20).map(|i| 0x6100 + 4 * i).collect();
        e.put_vtable(0x6000, &slots);
        e.register(0x6168, |_, _| Ret::default());
        let state = e.mem.alloc(4);
        e.mem.set_u32(state, 0x6000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, state);
        e.mem.set_u32(this + 0x24, 0x99);
        e.mem.set_u32(this + 0x2c, 0xffff_ffff);
        e.mem.set_u32(this + 0x30, 5);
        e.call_log = Some(vec![]);
        e.call(0x00e8_b170, &args![this]);
        let states: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x6168)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(states, vec![0x8b, 0x89, 0x94, 0x93, 0x91]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xffff_0000);
        assert_eq!(e.mem.u32(this + 0x28), 0x99);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}

#[cfg(test)]
mod b049_tests {
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

    /// An object whose virtual table (at `vtable`) has `slots` entries, slot `i` pointing
    /// at `vtable + 0x1000 + 4 * i`.
    fn object_with_vtable(e: &mut Engine, vtable: u32, slots: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..slots).map(|i| vtable + 0x1000 + 4 * i).collect();
        e.put_vtable(vtable, &targets);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00e89ed0_walks_the_order_of_each_kind() {
        let mut e = Engine::new();
        let request = e.mem.alloc(8);
        let table = e.mem.alloc(0x58);
        let pick = |e: &mut Engine, kind: u32, choice: u32| {
            e.mem.set_u32(request, kind);
            e.mem.set_u32(request + 4, choice);
            e.call(0x00e8_9ed0, &args![request, table]).u32()
        };
        // Kind 0, choice 0: +0x18 first.
        e.mem.set_u32(table + 0x18, 0x18);
        e.mem.set_u32(table + 0x4, 0x4);
        assert_eq!(pick(&mut e, 0, 0), 0x18);
        // Choice 3 counts as 0.
        assert_eq!(pick(&mut e, 5, 3), 0x18);
        // Kind 1 skips +0x18: choice 0 walks +4 first.
        assert_eq!(pick(&mut e, 1, 0), 0x4);
        // Kind 6 counts as 2, which tries +8 before +4.
        e.mem.set_u32(table + 0x8, 0x8);
        assert_eq!(pick(&mut e, 6, 0), 0x8);
        // Kind 4: +0x34 first, then +0x30 before +0x2c for choice 1.
        e.mem.set_u32(table + 0x2c, 0x2c);
        e.mem.set_u32(table + 0x30, 0x30);
        assert_eq!(pick(&mut e, 4, 1), 0x30);
        assert_eq!(pick(&mut e, 4, 0), 0x2c);
        e.mem.set_u32(table + 0x34, 0x34);
        assert_eq!(pick(&mut e, 4, 0), 0x34);
        // Kind 7, choice 1: +0x3c, then +0xc, +0x10, +0x14.
        e.mem.set_u32(table + 0x10, 0x10);
        assert_eq!(pick(&mut e, 7, 1), 0x10);
        // Kind 9 reaches +0x40 after +0x4c..+0x54; kind 14 tries +0x54 then +0x48.
        e.mem.set_u32(table + 0x48, 0x48);
        e.mem.set_u32(table + 0x40, 0x40);
        assert_eq!(pick(&mut e, 9, 0), 0x40);
        assert_eq!(pick(&mut e, 14, 0), 0x48);
        // Kinds above 14, and kind 3 with a negative choice, answer 0.
        assert_eq!(pick(&mut e, 15, 0), 0);
        assert_eq!(pick(&mut e, 3, 0xffff_ffff), 0);
    }

    #[test]
    fn fn_00e8a1f0_reads_a_channel() {
        let mut e = Engine::new();
        let format = e.mem.alloc(0x50);
        // Channel 1: type 7, word 2, bits 0x10, signed 1.
        e.mem.set_u32(format + 0x20, 7);
        e.mem.set_u32(format + 0x24, 2);
        e.mem.set_u8(format + 0x28, 0x10);
        e.mem.set_u8(format + 0x29, 1);
        let out = e.mem.alloc(16);
        let valid = e
            .call(
                0x00e8_a1f0,
                &args![format, 1u32, out, out + 4, out + 8, out + 12],
            )
            .bool();
        assert!(valid);
        assert_eq!(e.mem.u32(out), 7);
        assert_eq!(e.mem.u32(out + 4), 2);
        assert_eq!(e.mem.u8(out + 8), 0x10);
        assert_eq!(e.mem.u8(out + 12), 1);
        e.mem.set_u32(format + 0x14, 0x13);
        let valid = e
            .call(
                0x00e8_a1f0,
                &args![format, 0u32, out, out + 4, out + 8, out + 12],
            )
            .bool();
        assert!(!valid);
    }

    #[test]
    fn fn_00e8a260_constructs_the_texture_data() {
        let mut e = Engine::new();
        e.register(TEXTURE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x6c);
        e.mem.write(this, &[0xcd; 0x6c]);
        e.call_log = Some(vec![]);
        let back = e.call(0x00e8_a260, &args![this, 0x11u32, 0x22u32]).u32();
        assert_eq!(back, this);
        assert_eq!(calls_to(&e, TEXTURE_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, PIXEL_FORMAT_CONSTRUCTOR),
            vec![vec![this + 0x14]]
        );
        assert_eq!(e.mem.u32(this), TEXTURE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x11);
        assert_eq!(e.mem.u32(this + 0x60), 0x22);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0x00ff_ffff);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }

    #[test]
    fn fn_00e8a2b0_releases_the_texture_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(TEXTURE_BASE_DESTRUCTOR, |_, _| Ret::default());
        let holder = object_with_vtable(&mut e, 0x7000, 0x40, 4);
        e.register(0x7000 + 0x10e4, |_, _| Ret::default());
        let texture = object_with_vtable(&mut e, 0x9000, 4, 4);
        e.register(0x9000 + 0x1008, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x8b8, holder);
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x60, renderer);
        e.mem.set_u32(this + 0x64, texture);
        e.call_log = Some(vec![]);
        e.call(0x00e8_a2b0, &args![this]);
        assert_eq!(calls_to(&e, 0x7000 + 0x10e4), vec![vec![holder, texture]]);
        assert_eq!(calls_to(&e, 0x9000 + 0x1008), vec![vec![texture]]);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this), TEXTURE_BASE_VTABLE);
        assert_eq!(calls_to(&e, TEXTURE_BASE_DESTRUCTOR), vec![vec![this]]);
        // Without a texture only the base runs.
        e.call_log = Some(vec![]);
        e.call(0x00e8_a2b0, &args![this]);
        assert!(calls_to(&e, 0x9000 + 0x1008).is_empty());
        assert_eq!(calls_to(&e, TEXTURE_BASE_DESTRUCTOR).len(), 1);
    }

    /// An image converter that accepts the candidates in `accepts`.
    fn converter_engine(accepts: &'static [u32]) -> Engine {
        let mut e = Engine::new();
        let converter = object_with_vtable(&mut e, 0x7000, 8, 4);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| converter.into_ret());
        e.register_double(0x7000 + 0x1010, move |_, a| {
            accepts.contains(&a[2]).into_ret()
        });
        e.register(CHANNEL_COUNT_00E8A240, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn fn_00e8a300_follows_the_source_type_and_preferences() {
        let mut e = converter_engine(&[0x2c, 0x14, 0x40, 0x0c]);
        let format = e.mem.alloc(0x50);
        let prefs = e.mem.alloc(8);
        let table = e.mem.alloc(0x58);
        // Each entry holds its own offset (+0 holds 0: unset).
        for offset in (0..0x58).step_by(4) {
            e.mem.set_u32(table + offset, offset);
        }
        let find = |e: &mut Engine, source_type: u32, kind: u32, choice: u32| {
            e.mem.set_u32(format + 4, source_type);
            e.mem.set_u32(prefs, kind);
            e.mem.set_u32(prefs + 4, choice);
            e.call(0x00e8_a300, &args![0u32, format, prefs, table])
                .u32()
        };
        // Kind 4: +0x2c is tried first and accepted.
        assert_eq!(find(&mut e, 0, 4, 0), 0x2c);
        // Type 0, kind 2: +8 then +0x14, which is accepted.
        assert_eq!(find(&mut e, 0, 2, 0), 0x14);
        // Type 7 answers 0.
        assert_eq!(find(&mut e, 7, 2, 0), 0);
        // Type 1: the channel test (not type 0x13, word 1, 16 bits, one channel): +0x40.
        e.mem.set_u32(format + 0x14, 0);
        e.mem.set_u32(format + 0x18, 1);
        e.mem.set_u8(format + 0x1c, 0x10);
        assert_eq!(find(&mut e, 1, 2, 0), 0x40);
        // Without a usable channel: kind 1, choice 1 tries +0xc first.
        e.mem.set_u32(format + 0x14, 0x13);
        assert_eq!(find(&mut e, 1, 1, 1), 0x0c);
        // Kind 2: +0x14 before the second choice.
        assert_eq!(find(&mut e, 1, 2, 0), 0x14);
        // A palette answers 0 before anything else.
        e.mem.set_u32(format + 8, 1);
        assert_eq!(find(&mut e, 0, 4, 0), 0);
    }

    #[test]
    fn fn_00e8a9b0_deletes_every_texture_data_object() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let data = object_with_vtable(&mut e, 0x7000, 1, 4);
        e.register(0x8000, |_, _| Ret::default());
        let second = e.mem.alloc(0x30);
        let first = e.mem.alloc(0x30);
        e.mem.set_u32(first + 0x2c, second);
        e.mem.set_u32(second + 0x24, data);
        e.mem.set_u32(TEXTURE_LIST_HEAD, first);
        e.call_log = Some(vec![]);
        e.call(0x00e8_a9b0, &args![0x4321u32]);
        assert_eq!(calls_to(&e, 0x8000), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(second + 0x24), 0);
    }

    #[test]
    fn fn_00e8aa40_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(CUBE_BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x88);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_aa40, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CUBE_MAP_DATA_VTABLE);
        assert_eq!(calls_to(&e, CUBE_BASE_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        e.call(0x00e8_aa40, &args![this, 1u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this, 0x88]]);
    }

    fn cube_device_engine(result: i32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let device = object_with_vtable(&mut e, 0x7000, 0x20, 4);
        e.register_double(0x7000 + 0x1064, move |e, a| {
            e.mem.set_u32(a[6], 0x5150);
            result.into_ret()
        });
        e.register(ERROR_STRING, |_, _| 0x4444u32.into_ret());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x28c);
        e.mem.set_u32(renderer + 0x288, device);
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x60, renderer);
        e.mem.set_u32(this + 0xc, 64);
        e.mem.set_u32(this + 0x68, 7);
        e.mem.set_u32(this + 0x20, 21);
        (e, this)
    }

    #[test]
    fn fn_00e8aa70_creates_the_cube_texture() {
        let (mut e, this) = cube_device_engine(0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_aa70, &args![this]).bool());
        let call = &calls_to(&e, 0x7000 + 0x1064)[0];
        assert_eq!(&call[1..6], &[64, 7, 0, 21, 1]);
        assert_eq!(call[7], 0);
        assert_eq!(e.mem.u32(this + 0x64), 0x5150);
        // Dynamic textures: usage 0x200, pool 0.
        e.mem.set_u8(DYNAMIC_TEXTURES, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_aa70, &args![this]);
        let call = &calls_to(&e, 0x7000 + 0x1064)[0];
        assert_eq!(&call[1..6], &[64, 7, 0x200, 21, 0]);

        let (mut e, this) = cube_device_engine(-5);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e8_aa70, &args![this]).bool());
        assert_eq!(calls_to(&e, ERROR_STRING), vec![vec![(-5i32) as u32]]);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![CREATE_CUBE_FAILED_MESSAGE, 0x4444]]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
    }

    #[test]
    fn fn_00e8aaf0_copies_each_level_of_a_face() {
        let mut e = Engine::new();
        let texture = object_with_vtable(&mut e, 0x7000, 0x20, 4);
        let surface = object_with_vtable(&mut e, 0x9000, 4, 4);
        e.register_double(0x7000 + 0x1048, move |e, a| {
            if a[2] == 2 {
                return (-1i32).into_ret();
            }
            e.mem.set_u32(a[3], surface);
            0u32.into_ret()
        });
        e.register(0x9000 + 0x1008, |_, _| Ret::default());
        e.register(COPY_SURFACE_00E68150, |_, _| Ret::default());
        e.register(ERROR_STRING, |_, _| 0x4444u32.into_ret());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x64, texture);
        e.mem.set_u32(this + 0x68, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e8_aaf0, &args![this, 0x3000u32, 4u32]);
        assert_eq!(
            calls_to(&e, COPY_SURFACE_00E68150),
            vec![vec![0x3000, 0, surface, 4], vec![0x3000, 1, surface, 4]]
        );
        assert_eq!(calls_to(&e, 0x9000 + 0x1008).len(), 2);
        assert!(calls_to(&e, DEBUG_PRINT).is_empty());
        // A third level fails: the message names it.
        e.mem.set_u32(this + 0x68, 3);
        e.call_log = Some(vec![]);
        e.call(0x00e8_aaf0, &args![this, 0x3000u32, 4u32]);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![SURFACE_LEVEL_FAILED_MESSAGE, 2, 0x4444]]
        );
        // No levels: nothing.
        e.mem.set_u32(this + 0x68, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_aaf0, &args![this, 0x3000u32, 4u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e8abf0_without_cube_formats_answers_null() {
        let mut e = Engine::new();
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x900);
        let texture = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_abf0, &args![texture, renderer]).u32(), 0);
        assert_eq!(
            calls_to(&e, DEBUG_PRINT),
            vec![vec![NO_CUBE_FORMATS_MESSAGE]]
        );
    }

    /// The renderer, texture and doubles for `00e8abf0`'s paths; answers (engine, texture,
    /// renderer, the object the allocation hands out).
    fn cube_map_engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        let object = e.mem.alloc(0x88);
        e.register_double(ALLOCATE, move |_, _| object.into_ret());
        e.register(CUBE_BASE_CONSTRUCTOR, |e, a| {
            // The base keeps the texture at +8 and the renderer at +0x60.
            e.mem.set_u32(a[0] + 8, a[1]);
            e.mem.set_u32(a[0] + 0x60, a[2]);
            Ret::default()
        });
        e.register(SET_MIPMAP_00E68760, |_, _| Ret::default());
        e.register(COPY_FACE_00E8AB70, |_, _| Ret::default());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.register(D3D_FORMAT_OF, |_, _| 21u32.into_ret());
        // The converter accepts every candidate.
        let converter = object_with_vtable(&mut e, 0x7000, 8, 4);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| converter.into_ret());
        e.register(0x7000 + 0x1010, |_, _| 1u32.into_ret());
        // The device makes the cube texture.
        let device = object_with_vtable(&mut e, 0xa000, 0x20, 4);
        e.register(0xa000 + 0x1064, |e, a| {
            e.mem.set_u32(a[6], 0x5150);
            0u32.into_ret()
        });
        // `CheckDeviceFormat` passes.
        let direct3d = object_with_vtable(&mut e, 0xc000, 0x10, 4);
        e.register(0xc000 + 0x1028, |_, _| 0u32.into_ret());
        e.mem.set_u32(DIRECT3D, direct3d);
        let singleton = e.mem.alloc(0x600);
        e.mem.set_u32(RENDERER, singleton);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        // One cube map format, entry 3 of the table at +0x7b0.
        let chosen = e.mem.alloc(0x44);
        e.mem.set_u32(renderer + 0x7b0 + 12, chosen);
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture + 0x18, 1);
        (e, texture, renderer, object)
    }

    #[test]
    fn fn_00e8abf0_copies_a_cube_map_source_face_by_face() {
        let (mut e, texture, renderer, object) = cube_map_engine();
        // The source's format resolves to table entry +0x14.
        let candidate = e.mem.alloc(0x44);
        e.mem.set_u32(renderer + 0x7b0 + 0x14, candidate);
        let cube = e.mem.alloc(0x94);
        let width = e.mem.alloc(4);
        e.mem.set_u32(width, 64);
        e.mem.set_u32(cube + 0x64, width);
        e.mem.set_u32(cube + 0x68, width);
        let sizes = e.mem.alloc(8);
        e.mem.set_u32(sizes + 4, 0x2000);
        e.mem.set_u32(cube + 0x6c, sizes);
        e.mem.set_u32(cube + 0x74, 1);
        e.mem.set_u32(cube + 0x7c, 6);
        e.mem.set_u32(cube + 0x90, 0x77);
        e.mem.set_u32(cube + 0x14, 0xabcd);
        e.mem.set_u32(texture + 0x38, cube);
        e.mem.set_u8(renderer + 0x6f4, 1);
        e.call_log = Some(vec![]);
        let made = e.call(0x00e8_abf0, &args![texture, renderer]).u32();
        assert_eq!(made, object);
        assert_eq!(e.mem.u32(object), CUBE_MAP_DATA_VTABLE);
        assert_eq!(e.mem.u32(cube + 0x20), 21);
        assert_eq!(e.mem.u32(object + 0x14), 0xabcd);
        assert_eq!(e.mem.u32(object + 0xc), 64);
        assert_eq!(e.mem.u32(object + 0x68), 1);
        assert_eq!(e.mem.u8(object + 0x6d), 0);
        assert_eq!(e.mem.u32(object + 0x70), 0xc000);
        assert_eq!(e.mem.u32(TEXTURE_BYTES), 0xc000);
        assert_eq!(e.mem.u32(TEXTURE_PADDING), 0);
        assert_eq!(calls_to(&e, SET_MIPMAP_00E68760), vec![vec![object, 0x77]]);
        assert_eq!(calls_to(&e, COPY_FACE_00E8AB70).len(), 6);
        assert_eq!(e.mem.u32(texture + 0x24), object);
    }

    #[test]
    fn fn_00e8abf0_failed_pixel_data_takes_the_replacement() {
        let (mut e, texture, renderer, object) = cube_map_engine();
        // No cube map, no pixel data, no name: replacement data (0x80000008), which is not
        // a square power of two (0x80000009); it has five faces, so the object is deleted.
        let replacement = e.mem.alloc(0x70);
        let width = e.mem.alloc(4);
        e.mem.set_u32(width, 3);
        e.mem.set_u32(replacement + 0x54, width);
        e.mem.set_u32(replacement + 0x58, width);
        e.mem.set_u32(replacement + 0x6c, 5);
        e.register_double(REPLACEMENT_DATA, move |_, _| replacement.into_ret());
        e.mem.set_u32(renderer + 0x880, 3);
        e.put_vtable(CUBE_MAP_DATA_VTABLE, &[0xe100]);
        e.register(0xe100, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00e8_abf0, &args![texture, renderer]).u32();
        assert_eq!(made, 0);
        let codes: Vec<u32> = calls_to(&e, REPLACEMENT_DATA)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(codes, vec![0x8000_0008, 0x8000_0009]);
        assert_eq!(e.mem.u8(object + 0x6c), 1);
        assert_eq!(e.mem.u32(object + 0xc), 3);
        assert_eq!(calls_to(&e, 0xe100), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
    }

    #[test]
    fn fn_00e8b490_sets_key_and_value() {
        let mut e = Engine::new();
        let item = e.mem.alloc(12);
        e.call(0x00e8_b490, &args![0u32, item, 5u32, 6u32]);
        assert_eq!(e.mem.u32(item + 4), 5);
        assert_eq!(e.mem.u32(item + 8), 6);
    }

    /// A light manager whose list holds `lights` lights, light `i` with an entry of index
    /// `33 + i`; answers the manager and the entries.
    fn light_manager(e: &mut Engine, lights: u32) -> (u32, Vec<u32>) {
        e.map(0x011c_5000, 0x1000);
        e.register(NODE_POOL_LOCK_TAKE, |_, _| Ret::default());
        let device = object_with_vtable(e, 0x7000, 0x40, 4);
        e.register(0x7000 + 0x10d4, |_, _| Ret::default());
        e.register(0x7000 + 0x1008, |_, _| Ret::default());
        let this = e.mem.alloc(0x240);
        e.mem.set_u32(this + 0x1c, device);
        let mut nodes = vec![];
        let mut entries = vec![];
        for i in 0..lights {
            let entry = e.mem.alloc(0x74);
            e.mem.set_u32(entry + 0x6c, 33 + i);
            e.mem.set_u8(entry + 0x71, 1);
            let light = e.mem.alloc(0xf0);
            e.mem.set_u32(light + 0xec, entry);
            let node = e.mem.alloc(12);
            e.mem.set_u32(node + 8, light);
            nodes.push(node);
            entries.push(entry);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
            e.mem.set_u32(pair[1] + 4, pair[0]);
        }
        if let (Some(first), Some(last)) = (nodes.first(), nodes.last()) {
            e.mem.set_u32(this + 0x10, *first);
            e.mem.set_u32(this + 0x14, *last);
        }
        e.mem.set_u32(this + 0x18, lights);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.mem.set_u32(NODE_POOL_LOCK, 0x99);
        (this, entries)
    }

    #[test]
    fn fn_00e8b560_turns_off_each_listed_light() {
        let mut e = Engine::new();
        let (this, entries) = light_manager(&mut e, 2);
        e.mem.set_u32(this + 0x24, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00e8_b560, &args![this]);
        let device = e.mem.u32(this + 0x1c);
        assert_eq!(
            calls_to(&e, 0x7000 + 0x10d4),
            vec![vec![device, 33, 0], vec![device, 34, 0]]
        );
        assert_eq!(e.mem.u8(entries[0] + 0x71), 0);
        assert_eq!(e.mem.u8(entries[1] + 0x71), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x55);
        assert_eq!(calls_to(&e, NODE_POOL_LOCK_TAKE).len(), 2);
        // The lock count reached 0 on the first node, which cleared the owner.
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_ne!(e.mem.u32(NODE_POOL_FREE_LIST), 0);
        // An empty list only copies +0x24.
        e.mem.set_u32(this + 0x24, 0x66);
        e.call(0x00e8_b560, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0x66);
    }

    #[test]
    fn fn_00e8b630_empties_the_map_and_list_then_cleans_up() {
        let mut e = Engine::new();
        let (this, _) = light_manager(&mut e, 1);
        e.put_vtable(LIGHT_MAP_VTABLE, &[0, 0x6104]);
        e.register(0x6104, |_, _| 0u32.into_ret());
        e.mem.set_u32(this, LIGHT_MAP_VTABLE);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, buckets);
        e.mem.set_u32(this + 0xc, 1);
        let entry = e.mem.alloc(0x74);
        e.mem.set_u32(entry + 0x6c, 37);
        e.mem.set_u32(this + 0x3c + 4, 0xffff_ffff);
        let light = e.mem.alloc(0xf0);
        e.mem.set_u32(light + 0xec, 0x1234);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, light);
        e.mem.set_u32(item + 8, entry);
        e.register_double(LIGHT_MAP_FIRST, move |_, _| item.into_ret());
        e.register(LIGHT_DROP_00E91FD0, |e, a| {
            // The drop empties the map.
            e.mem.set_u32(a[0] + 0xc, 0);
            Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(LIGHT_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(LIGHT_MAP_CLEAN_UP, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_b630, &args![this]);
        assert_eq!(calls_to(&e, 0x6104), vec![vec![this, light]]);
        assert_eq!(calls_to(&e, LIGHT_DROP_00E91FD0), vec![vec![this, light]]);
        assert_eq!(e.mem.u32(light + 0xec), 0);
        assert_eq!(e.mem.u32(this + 0x40), !(1u32 << 5));
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![entry, 0x74]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(calls_to(&e, 0x7000 + 0x1008).len(), 1);
        assert_eq!(calls_to(&e, LIGHT_LIST_REMOVE_ALL), vec![vec![this + 0x10]]);
        assert_eq!(calls_to(&e, LIGHT_MAP_CLEAN_UP).len(), 2);
        assert_eq!(e.mem.u32(this), LIGHT_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, FREE_00AA10F0), vec![vec![buckets]]);
    }

    #[test]
    fn fn_00e8bd40_removes_a_light_with_or_without_an_entry() {
        let mut e = Engine::new();
        let (this, entries) = light_manager(&mut e, 1);
        let entry = entries[0];
        let light = e.mem.u32(e.mem.u32(this + 0x10) + 8);
        e.mem.set_u32(this + 0x3c + 4, 0xffff_ffff);
        e.register(LIGHT_MAP_REMOVE, |_, _| 0u32.into_ret());
        e.register(LIGHT_DROP_00E91FD0, |_, _| Ret::default());
        e.register(LIGHT_LIST_REMOVE, |e, a| {
            // Records the light the slot holds at the list's +0x20 (the manager's +0x30).
            let held = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 0x20, held);
            Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_bd40, &args![this, light]);
        let device = e.mem.u32(this + 0x1c);
        assert_eq!(calls_to(&e, 0x7000 + 0x10d4), vec![vec![device, 33, 0]]);
        assert_eq!(e.mem.u8(entry + 0x71), 0);
        assert_eq!(e.mem.u32(this + 0x30), light);
        assert_eq!(e.mem.u32(light + 0xec), 0);
        assert_eq!(e.mem.u32(this + 0x40), !(1u32 << 1));
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![entry, 0x74]]);

        // Without an entry: the map's removal finds one (index 40).
        let found = e.mem.alloc(0x74);
        e.mem.set_u32(found + 0x6c, 40);
        e.register_double(LIGHT_MAP_REMOVE, move |e, a| {
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        e.mem.set_u32(this + 0x30, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_bd40, &args![this, light]);
        assert_eq!(calls_to(&e, LIGHT_DROP_00E91FD0), vec![vec![this, light]]);
        assert_eq!(e.mem.u32(this + 0x30), light);
        assert_eq!(e.mem.u32(this + 0x40), !(1u32 << 1) & !(1u32 << 8));
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        assert!(calls_to(&e, 0x7000 + 0x10d4).is_empty());

        // Not found: nothing else.
        e.register(LIGHT_MAP_REMOVE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e8_bd40, &args![this, light]);
        assert!(calls_to(&e, LIGHT_DROP_00E91FD0).is_empty());
    }

    #[test]
    fn fn_00e8bf80_counts_up() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, 41);
        e.call(0x00e8_bf80, &args![this]);
        assert_eq!(e.mem.u32(this + 0x3c), 42);
    }
}
