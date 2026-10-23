//! `BSShader/bsshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAVObject::GetProperty` (Xbox PDB, `00a59d30`): `thiscall`, `RET 4`; the object's property
/// of the given type, or null.
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderProperty::SetFlag` (Xbox PDB, `00441130`): `thiscall`, `RET 8` (flag index, value
/// byte).
const BS_SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
/// The virtual slot (byte offset) that answers the object as a geometry, or null (`IsGeometry`).
const SLOT_IS_GEOMETRY: u32 = 0x18;
/// The virtual slot (byte offset) that answers the object as a node, or null (`IsNode`).
const SLOT_IS_NODE: u32 = 0x0c;
/// The property type `NiAVObject::GetProperty` is asked for (the shader property's slot).
const SHADER_PROPERTY_TYPE: u32 = 3;

// Translated from 00ba91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB): `cdecl`, three stack words (an object, a flag
/// index, a value). For a non-null object: a geometry (virtual slot +0x18 non-null) gets the
/// flag set on its shader property (`GetProperty(3)` then `SetFlag(index, value)`), when it has
/// one; any other object that is a node (slot +0xc) passes the call on to each of its non-null
/// children (the `NiTArray` at node `+0x9c`: buffer at +0xa0, count at +0xa6).
pub fn bs_shader_property_set_flag_recurse(e: &mut Engine, object: Ptr, flag: u32, value: u32) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_GEOMETRY, &args![]).u32() != 0 {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            e.call(BS_SHADER_PROPERTY_SET_FLAG, &args![property, flag, value]);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
        let mut index = 0u32;
        if count != 0 {
            loop {
                let buffer = e.mem.u32(node.wrapping_add(0xa0));
                let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
                if child != 0 {
                    bs_shader_property_set_flag_recurse(e, Ptr::new(child), flag, value);
                }
                index = index.wrapping_add(1);
                if index >= count {
                    break;
                }
            }
        }
    }
}

layout! {
    /// `BSShaderProperty::RenderPass` (Xbox PDB), 0x10 bytes; the offsets
    /// match the PC constructor.
    pub struct RenderPass: 0x10 {
        /// `pGeometry` (Xbox PDB).
        0x00 pGeometry: u32,
        /// `usPassEnum` (Xbox PDB).
        0x04 usPassEnum: u16,
        /// `eAccumulationHint` (Xbox PDB).
        0x06 eAccumulationHint: u8,
        /// `bFirstPass` (Xbox PDB).
        0x07 bFirstPass: u8,
        /// `bLastPass` (Xbox PDB).
        0x08 bLastPass: u8,
        /// `ucNumLights` (Xbox PDB).
        0x09 ucNumLights: u8,
        /// `ucMaxNumLights` (Xbox PDB).
        0x0a ucMaxNumLights: u8,
        /// `ucExtraParam` (Xbox PDB).
        0x0b ucExtraParam: u8,
        /// `ppSceneLights` (Xbox PDB).
        0x0c ppSceneLights: u32,
    }
}

/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00ba8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::RenderPass` (Xbox PDB): `cdecl` with a
/// variable argument list: the pass, its geometry, its pass enum (low half
/// of the word), its first-pass flag (low byte), the light count (low byte)
/// and then that many light words. Clears the hint, last-pass flag, maximum
/// light count, extra parameter and light array; with lights allocates
/// `count * 4` bytes, sets the maximum to the count and copies the light
/// words in. Returns the pass. The exe reads the light words from its
/// caller's stack; here they are the words after the count (a missing word
/// stops loudly).
pub fn bs_shader_property_render_pass_render_pass(
    e: &mut Engine,
    this: Ptr<RenderPass>,
    geometry: u32,
    pass_enum: u16,
    first_pass: u8,
    light_count: u8,
    lights: &[u32],
) -> Ptr<RenderPass> {
    e.set(this, RenderPass::pGeometry, geometry);
    e.set(this, RenderPass::usPassEnum, pass_enum);
    e.set(this, RenderPass::ucNumLights, light_count);
    e.set(this, RenderPass::ucMaxNumLights, 0);
    e.set(this, RenderPass::bFirstPass, first_pass);
    e.set(this, RenderPass::bLastPass, 0);
    e.set(this, RenderPass::ppSceneLights, 0);
    e.set(this, RenderPass::eAccumulationHint, 0);
    if light_count == 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let count = u32::from(light_count);
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, RenderPass::ppSceneLights, array);
    e.set(this, RenderPass::ucMaxNumLights, light_count);
    // `TEST EDI,EDI; JLE`: never taken (the count is at least 1 here); the
    // branch stores the zero in AL as the extra parameter.
    if (count as i32) <= 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let mut index = 0u32;
    loop {
        let light = lights[index as usize];
        let array = e.get(this, RenderPass::ppSceneLights);
        e.mem.set_u32(array + index * 4, light);
        index += 1;
        if (index as i32) >= (count as i32) {
            break;
        }
    }
    e.set(this, RenderPass::ucExtraParam, 0);
    this
}

/// The registry entry of [`bs_shader_property_render_pass_render_pass`]:
/// the five fixed words, then the light words.
fn render_pass_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 5,
        "RenderPass::RenderPass needs 5 argument words, got {}",
        words.len()
    );
    bs_shader_property_render_pass_render_pass(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2] as u16,
        words[3] as u8,
        words[4] as u8,
        &words[5..],
    )
    .addr()
    .into_ret()
}

use crate::types::NiTArray;

layout! {
    /// `BSShaderProperty` (Xbox PDB) on PC, 0x60 bytes (the constructor `00ba8e50` and the
    /// derived `TileShaderProperty`, whose own fields start at +0x60, show it): the PDB's
    /// fields up to `fDepthBias` at the same offsets; the PDB's `fLastClearFrame` and
    /// `fLastAccumFrame` (+0x60, +0x64) are not there on PC.
    pub struct BSShaderProperty: 0x60 {
        /// `NiShadeProperty::m_uFlags` (Xbox PDB).
        0x18 m_uFlags: u16,
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1c iShaderPropertyType: i32,
        /// `ulFlags[0]` (Xbox PDB).
        0x20 ulFlags0: u32,
        /// `ulFlags[1]` (Xbox PDB).
        0x24 ulFlags1: u32,
        /// `fAlpha` (Xbox PDB).
        0x28 fAlpha: u32,
        /// `fFadeAlpha` (Xbox PDB).
        0x2c fFadeAlpha: u32,
        /// `fEnvmapScale` (Xbox PDB).
        0x30 fEnvmapScale: u32,
        /// `fCameraDistance` (Xbox PDB).
        0x34 fCameraDistance: u32,
        /// `iLastRenderPassState` (Xbox PDB).
        0x38 iLastRenderPassState: i32,
        /// `pRenderPassList` (Xbox PDB).
        0x3c pRenderPassList: u32,
        /// `pDepthMapRenderPassList` (Xbox PDB).
        0x40 pDepthMapRenderPassList: u32,
        /// `pConstAlphaRenderPassList` (Xbox PDB).
        0x44 pConstAlphaRenderPassList: u32,
        /// `pLocalMapRenderPassList` (Xbox PDB).
        0x48 pLocalMapRenderPassList: u32,
        /// `pSIBlockRenderPassList` (Xbox PDB).
        0x4c pSIBlockRenderPassList: u32,
        /// `pWaterFogRenderPassList` (Xbox PDB).
        0x50 pWaterFogRenderPassList: u32,
        /// `pSilhouettePassList` (Xbox PDB).
        0x54 pSilhouettePassList: u32,
        /// `iShader` (Xbox PDB).
        0x58 iShader: i32,
        /// `fDepthBias` (Xbox PDB).
        0x5c fDepthBias: u32,
    }

    /// `BSShaderProperty::RenderPassArray` (Xbox PDB), 0x14 bytes: the
    /// `NiTObjectArray<RenderPass *>` `pPassA` ([`NiTArray`] layout) at +0, then `iSize`
    /// (the number of passes in use).
    pub struct RenderPassArray: 0x14 {
        /// `iSize` (Xbox PDB).
        0x10 iSize: u32,
    }
}

/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InterlockedIncrement` (KERNEL32 import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The vtable `NiTArray<RenderPass *>`'s constructor and the render-pass array install
/// (`010b8480`).
const RENDER_PASS_ARRAY_VTABLE: u32 = 0x010b_8480;
/// The vtable `BSShaderProperty::BSShaderProperty` installs (`010b8488`).
const BS_SHADER_PROPERTY_VTABLE: u32 = 0x010b_8488;
/// The member copy `00a5d7a0` (`thiscall`, two stack words) that `00a81430` starts with.
const COPY_MEMBERS_00A5D7A0: u32 = 0x00a5_d7a0;
/// The base destructor `00a5c240` (`NiMain/niobjectnet.cpp`, `thiscall`).
const BASE_DESTRUCTOR_00A5C240: u32 = 0x00a5_c240;
/// The sized free `00aa1460` (`cdecl`, two words: the block and its size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// The base constructor `00a5c200` (`NiMain/niobjectnet.cpp`, `thiscall`).
const BASE_CONSTRUCTOR_00A5C200: u32 = 0x00a5_c200;
/// `NiObjectNET::PostLinkObject` (`00a5c6a0`, `thiscall`, one word: the stream).
const NI_OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// The base `LoadBinary` `00a95190` (`thiscall`, one word: the stream).
const BASE_LOAD_BINARY_00A95190: u32 = 0x00a9_5190;
/// The array store `00ba3aa0` (`thiscall`, two stack words: index and the address of the
/// element), which `RenderPassArray::Add` uses to append a new pass.
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
/// The virtual slot (byte offset) of `IsTriBasedGeom` (Xbox PDB slot list).
const SLOT_IS_TRI_BASED_GEOM: u32 = 0x1c;

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a81430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with two stack words (a destination and a second
/// word passed along). Calls `00a5d7a0` with both words, then copies `NiShadeProperty`'s
/// `m_uFlags` (`+0x18`) into the destination.
pub fn fn_00a81430(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32, extra: u32) {
    e.call(COPY_MEMBERS_00A5D7A0, &args![this, dest, extra]);
    let flags = e.get(this, BSShaderProperty::m_uFlags);
    e.set(
        Ptr::<BSShaderProperty>::new(dest),
        BSShaderProperty::m_uFlags,
        flags,
    );
}

// Translated from 00a81460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShadeProperty::scalar deleting destructor` (engine map, from the vtable): runs the base
/// destructor `00a5c240`, frees the 0x20-byte object (`00aa1460(this, 0x20)`) when bit 0 of
/// the flags is set, and answers `this`.
pub fn ni_shade_property_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BASE_DESTRUCTOR_00A5C240, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00ba8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetAlpha` (engine map; the body matches): stores the new `fAlpha`
/// (`FLD`/`FSTP`: a signalling NaN is stored quiet) and clears `iLastRenderPassState` when
/// the alpha crosses 1.0: the old one at least 1 and the new one below, or the old one below
/// 1 and the new one at least 1 (NaN compares as neither).
pub fn bs_shader_property_set_alpha(e: &mut Engine, this: Ptr<BSShaderProperty>, alpha: u32) {
    let old = f32::from_bits(e.get(this, BSShaderProperty::fAlpha));
    let new = f32::from_bits(alpha);
    let crosses = if old >= 1.0 {
        new < 1.0
    } else if old < 1.0 {
        new >= 1.0
    } else {
        false
    };
    if crosses {
        e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    }
    e.set(this, BSShaderProperty::fAlpha, x87_quiet_bits(alpha));
}

// Translated from 00ba8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with two stack words (a destination shader property
/// and a word passed along). Runs [`fn_00a81430`] with both, then copies both flag words,
/// `fAlpha`, `fFadeAlpha`, `iShader` and `fDepthBias` into the destination (the floats
/// through `FLD`/`FSTP`, so a signalling NaN arrives quiet) and clears the destination's
/// `iLastRenderPassState`.
pub fn fn_00ba8b10(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32, extra: u32) {
    fn_00a81430(e, this, dest, extra);
    let d = Ptr::<BSShaderProperty>::new(dest);
    let v = e.get(this, BSShaderProperty::ulFlags0);
    e.set(d, BSShaderProperty::ulFlags0, v);
    let v = e.get(this, BSShaderProperty::ulFlags1);
    e.set(d, BSShaderProperty::ulFlags1, v);
    let v = e.get(this, BSShaderProperty::fAlpha);
    e.set(d, BSShaderProperty::fAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::fFadeAlpha);
    e.set(d, BSShaderProperty::iLastRenderPassState, 0);
    e.set(d, BSShaderProperty::fFadeAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::iShader);
    e.set(d, BSShaderProperty::iShader, v);
    let v = e.get(this, BSShaderProperty::fDepthBias);
    e.set(d, BSShaderProperty::fDepthBias, x87_quiet_bits(v));
}

// Translated from 00ba8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BSShaderProperty::RenderPass *>::NiTArray` (engine map; the body matches):
/// `thiscall` with the capacity and the grow step (the low halves of two words). Sets the
/// vtable `010b8480`, the capacity and grow step, clears the size and the element count, and
/// allocates `capacity * 4` bytes for the elements (none for a zero capacity). Answers
/// `this`.
pub fn ni_t_array_render_pass_ni_t_array(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if max_size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
        return this;
    }
    let size = u32::from(max_size).checked_mul(4).unwrap_or(u32::MAX);
    let base = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, NiTArray::m_pBase, base);
    this
}

// Translated from 00ba8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::PostLinkObject` (engine map; the body matches): `NiObjectNET`'s
/// (`00a5c6a0`) with the stream, then `fDepthBias` becomes 1.0 when bit 26 (`0x4000000`) of
/// the first flag word is set.
pub fn bs_shader_property_post_link_object(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    stream: u32,
) {
    e.call(NI_OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(this, BSShaderProperty::ulFlags0) & 0x0400_0000 != 0 {
        e.set(this, BSShaderProperty::fDepthBias, 1.0f32.to_bits());
    }
}

// Translated from 00ba8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` on a `RenderPass` with one word (a light count, its
/// low byte): allocates `count * 4` bytes and stores them as `ppSceneLights` (the old array
/// is not freed).
pub fn fn_00ba8c00(e: &mut Engine, this: Ptr<RenderPass>, count: u8) {
    let size = u32::from(count).checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, RenderPass::ppSceneLights, array);
}

// Translated from 00ba8c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` on a `RenderPass`: frees `ppSceneLights` and
/// clears it, when there is one.
pub fn fn_00ba8c30(e: &mut Engine, this: Ptr<RenderPass>) {
    let array = e.get(this, RenderPass::ppSceneLights);
    if array != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
        e.set(this, RenderPass::ppSceneLights, 0);
    }
}

// Translated from 00ba8c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::SetLights` (engine map; the body matches): `cdecl` with a
/// variable argument list: the pass, the light count (low byte) and that many light words.
/// A count above `ucMaxNumLights` frees the old light array and allocates `count * 4`
/// bytes, raising the maximum; `ucNumLights` becomes the count and the light words are
/// copied in. The exe reads the light words from its caller's stack; here they are the
/// words after the count (a missing word stops loudly).
pub fn bs_shader_property_render_pass_set_lights(
    e: &mut Engine,
    this: Ptr<RenderPass>,
    count: u8,
    lights: &[u32],
) {
    if count > e.get(this, RenderPass::ucMaxNumLights) {
        let array = e.get(this, RenderPass::ppSceneLights);
        if array != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
            e.set(this, RenderPass::ppSceneLights, 0);
        }
        let size = u32::from(count).checked_mul(4).unwrap_or(u32::MAX);
        let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
        e.set(this, RenderPass::ppSceneLights, array);
        e.set(this, RenderPass::ucMaxNumLights, count);
    }
    e.set(this, RenderPass::ucNumLights, count);
    if count == 0 {
        return;
    }
    let n = i32::from(count);
    // `TEST ECX,ECX; JLE`: never taken (the count is at least 1 here).
    if n <= 0 {
        return;
    }
    let mut index = 0i32;
    loop {
        let light = lights[index as usize];
        let array = e.get(this, RenderPass::ppSceneLights);
        e.mem
            .set_u32(array.wrapping_add((index as u32).wrapping_mul(4)), light);
        index += 1;
        if index >= n {
            break;
        }
    }
}

/// The registry entry of [`bs_shader_property_render_pass_set_lights`]: the pass, the count,
/// then the light words.
fn set_lights_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "RenderPass::SetLights needs 2 argument words, got {}",
        words.len()
    );
    bs_shader_property_render_pass_set_lights(e, Ptr::new(words[0]), words[1] as u8, &words[2..]);
    Ret::default()
}

// Translated from 00ba8cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::~RenderPass` (engine map; the body matches): clears
/// `usPassEnum`, frees and clears the light array when there is one, and clears
/// `ucExtraParam`.
pub fn bs_shader_property_render_pass_destructor(e: &mut Engine, this: Ptr<RenderPass>) {
    e.set(this, RenderPass::usPassEnum, 0);
    let array = e.get(this, RenderPass::ppSceneLights);
    if array != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
        e.set(this, RenderPass::ppSceneLights, 0);
    }
    e.set(this, RenderPass::ucExtraParam, 0);
}

// Translated from 00ba8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetMotionBlurred` (engine map; the body matches): sets (non-zero low
/// byte) or clears bit 4 (`0x10`) of the first flag word; when the bit changes,
/// `iLastRenderPassState` is cleared.
pub fn bs_shader_property_set_motion_blurred(e: &mut Engine, this: Ptr<BSShaderProperty>, on: u8) {
    let flags = e.get(this, BSShaderProperty::ulFlags0);
    if on != 0 {
        if flags & 0x10 == 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let flags = e.get(this, BSShaderProperty::ulFlags0);
        e.set(this, BSShaderProperty::ulFlags0, flags | 0x10);
    } else {
        if flags & 0x10 != 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let flags = e.get(this, BSShaderProperty::ulFlags0);
        e.set(this, BSShaderProperty::ulFlags0, flags & 0xffff_ffef);
    }
}

// Translated from 00ba8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BSShaderProperty::RenderPass *>::SetSize` (engine map; the body matches):
/// nothing when the word equals the capacity. A size below `m_usSize` clears the slots from
/// the new size up (each non-null one lowers `m_usESize`) and lowers `m_usSize`. The
/// capacity becomes the size (its low half); a zero size leaves no buffer, otherwise a new
/// buffer of `capacity * 4` bytes gets the used slots and zeros after them. The old buffer
/// is freed in every case.
pub fn ni_t_array_render_pass_set_size(e: &mut Engine, this: Ptr<NiTArray>, size: u32) {
    if size == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    let size16 = size as u16;
    let used = e.get(this, NiTArray::m_usSize);
    if size < u32::from(used) {
        if size16 < used {
            let mut index = size16;
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let slot = base.wrapping_add(u32::from(index).wrapping_mul(4));
                if e.mem.u32(slot) != 0 {
                    e.mem.set_u32(slot, 0);
                    let count = e.get(this, NiTArray::m_usESize);
                    e.set(this, NiTArray::m_usESize, count.wrapping_add(0xffff));
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        e.set(this, NiTArray::m_usSize, size16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, size16);
    if size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let bytes = u32::from(size16).checked_mul(4).unwrap_or(u32::MAX);
        let base = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, bytes]).u32();
        e.set(this, NiTArray::m_pBase, base);
        let mut index = 0u16;
        if index < e.get(this, NiTArray::m_usSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let offset = u32::from(index) * 4;
                let value = e.mem.u32(old.wrapping_add(offset));
                index = index.wrapping_add(1);
                e.mem.set_u32(base.wrapping_add(offset), value);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        if index < e.get(this, NiTArray::m_usMaxSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                e.mem.set_u32(base.wrapping_add(u32::from(index) * 4), 0);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usMaxSize) {
                    break;
                }
            }
        }
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
}

// Translated from 00ba8e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::BSShaderProperty` (engine map; the body matches): the base
/// constructor (`00a5c200`), then `m_uFlags` 1, `iShaderPropertyType` -1, the vtable
/// `010b8488`, both flag words, `iLastRenderPassState`, the render-pass lists at +0x3c to
/// +0x50 and `iShader` 0, `fAlpha`, `fFadeAlpha` and `fEnvmapScale` 1.0, `fDepthBias` and
/// `fCameraDistance` 0.0 (`pSilhouettePassList` at +0x54 is not touched). Answers `this`.
pub fn bs_shader_property_bs_shader_property(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
) -> Ptr<BSShaderProperty> {
    e.call(BASE_CONSTRUCTOR_00A5C200, &args![this]);
    let one = 1.0f32.to_bits();
    e.set(this, BSShaderProperty::m_uFlags, 1);
    e.set(this, BSShaderProperty::iShaderPropertyType, -1);
    e.mem.set_u32(this.addr(), BS_SHADER_PROPERTY_VTABLE);
    e.set(this, BSShaderProperty::ulFlags0, 0);
    e.set(this, BSShaderProperty::ulFlags1, 0);
    e.set(this, BSShaderProperty::fAlpha, one);
    e.set(this, BSShaderProperty::fFadeAlpha, one);
    e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    e.set(this, BSShaderProperty::fEnvmapScale, one);
    e.set(this, BSShaderProperty::pRenderPassList, 0);
    e.set(this, BSShaderProperty::pDepthMapRenderPassList, 0);
    e.set(this, BSShaderProperty::fDepthBias, 0);
    e.set(this, BSShaderProperty::pLocalMapRenderPassList, 0);
    e.set(this, BSShaderProperty::pSIBlockRenderPassList, 0);
    e.set(this, BSShaderProperty::fCameraDistance, 0);
    e.set(this, BSShaderProperty::pConstAlphaRenderPassList, 0);
    e.set(this, BSShaderProperty::pWaterFogRenderPassList, 0);
    e.set(this, BSShaderProperty::iShader, 0);
    this
}

// Translated from 00ba8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; vtable slot +0x98 (`CopyToMembers` in the Xbox PDB's slot
/// list) of the shader-property vtables. `thiscall` with one word, the destination: copies
/// both flag words, `fAlpha`, `fFadeAlpha` (through `FLD`/`FSTP`), `iShader` and
/// `fDepthBias` and clears the destination's `iLastRenderPassState`. When this object has a
/// reference-counted object at `+0xc` (`NiObjectNET::m_spControllers` in the Xbox PDB), the
/// destination takes a reference to it (releasing its own first, unless it is the same) and
/// that object's slot +0x98 is called with the destination.
pub fn fn_00ba8f60(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32) {
    let d = Ptr::<BSShaderProperty>::new(dest);
    let v = e.get(this, BSShaderProperty::ulFlags0);
    e.set(d, BSShaderProperty::ulFlags0, v);
    let v = e.get(this, BSShaderProperty::ulFlags1);
    e.set(d, BSShaderProperty::ulFlags1, v);
    let v = e.get(this, BSShaderProperty::fAlpha);
    e.set(d, BSShaderProperty::fAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::fFadeAlpha);
    e.set(d, BSShaderProperty::iLastRenderPassState, 0);
    e.set(d, BSShaderProperty::fFadeAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::iShader);
    e.set(d, BSShaderProperty::iShader, v);
    let v = e.get(this, BSShaderProperty::fDepthBias);
    e.set(d, BSShaderProperty::fDepthBias, x87_quiet_bits(v));
    let source = e.mem.u32(this.addr().wrapping_add(0xc));
    if source == 0 {
        return;
    }
    let current = e.mem.u32(dest.wrapping_add(0xc));
    if current != source {
        if current != 0 {
            release_reference(e, current);
        }
        let source = e.mem.u32(this.addr().wrapping_add(0xc));
        e.mem.set_u32(dest.wrapping_add(0xc), source);
        if source != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![source.wrapping_add(4)]);
        }
    }
    let object = e.mem.u32(dest.wrapping_add(0xc));
    e.vcall(object, 0x98, &args![dest]);
}

// Translated from 00ba9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetMotionBlurRecurse` (engine map; the body matches): `cdecl`, an
/// object and a value. For a non-null object: a tri-based geometry (slot +0x1c non-zero)
/// with a shader property (`GetProperty(3)`) gets [`bs_shader_property_set_motion_blurred`]
/// with the value; any other object that is a node (slot +0xc) passes the call on to each
/// of its non-null children (the array at node `+0xa0`, count at `+0xa6`).
pub fn bs_shader_property_set_motion_blur_recurse(e: &mut Engine, object: Ptr, value: u32) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_TRI_BASED_GEOM, &args![])
        .u32()
        != 0
    {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            bs_shader_property_set_motion_blurred(e, Ptr::new(property), value as u8);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
    let mut index = 0u32;
    if count == 0 {
        return;
    }
    loop {
        let buffer = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_property_set_motion_blur_recurse(e, Ptr::new(child), value);
        }
        index = index.wrapping_add(1);
        if index >= count {
            break;
        }
    }
}

/// One read through the stream's file (`[stream + 0x24c]`): its read function at file `+8`
/// (`cdecl`: file, buffer, 4, the address of a word holding 4, 1); the answer is added to
/// the file's word at `+4`.
pub(crate) fn stream_read_word(e: &mut Engine, stream: u32, buffer: u32) {
    let file = e.mem.u32(stream.wrapping_add(0x24c));
    let read = e.mem.u32(file.wrapping_add(8));
    let n = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![file, buffer, 4u32, size, 1u32]).u32()
    });
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(n));
}

// Translated from 00ba92c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with one word, a stream (a `LoadBinary`). Runs the
/// base load (`00a95190`), then reads `iShader`, the two flag words and `fEnvmapScale`, one
/// word each through the stream's file; between the flags and the scale, a set bit 13
/// (`0x2000`) of the second flag word also sets bit 15 (`0x8000`) and, when that bit was
/// clear, clears `iLastRenderPassState`.
pub fn fn_00ba92c0(e: &mut Engine, this: Ptr<BSShaderProperty>, stream: u32) {
    e.call(BASE_LOAD_BINARY_00A95190, &args![this, stream]);
    stream_read_word(e, stream, this.addr().wrapping_add(0x58));
    let mut flags = this.addr().wrapping_add(0x20);
    let mut remaining = 2u32;
    loop {
        stream_read_word(e, stream, flags);
        flags = flags.wrapping_add(4);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    if e.get(this, BSShaderProperty::ulFlags1) & 0x2000 != 0 {
        if e.get(this, BSShaderProperty::ulFlags1) & 0x8000 == 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let v = e.get(this, BSShaderProperty::ulFlags1);
        e.set(this, BSShaderProperty::ulFlags1, v | 0x8000);
    }
    stream_read_word(e, stream, this.addr().wrapping_add(0x30));
}

/// `fn_00ba9420`'s globals: two guard bytes, the source object, the two 16-word copies, the
/// flag bytes and the three floats.
const GUARD_0120045C: u32 = 0x0120_045c;
const GUARD_0120045D: u32 = 0x0120_045d;
const SOURCE_011F9508: u32 = 0x011f_9508;
const MATRIX_01200470: u32 = 0x0120_0470;
const MATRIX_012004B0: u32 = 0x0120_04b0;
const FLAG_0120045E: u32 = 0x0120_045e;
const FLAG_011ADCDC: u32 = 0x011a_dcdc;
const FLOATS_012004F0: u32 = 0x0120_04f0;

// Translated from 00ba9420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `cdecl`, one object. Unless one of the bytes at `0120045c`
/// and `0120045d` is set: copies 16 words from `+0x980` and 16 from `+0x9c0` of the object
/// at `011f9508` to `01200470` and `012004b0`, sets the byte at `0120045e` and clears the
/// one at `011adcdc`, and copies the object's floats at `+0x68`, `+0x74` and `+0x80`
/// (through `FLD`/`FSTP`) to `012004f0`, `012004f4` and `012004f8`.
pub fn fn_00ba9420(e: &mut Engine, object: Ptr) {
    if e.mem.u8(GUARD_0120045C) != 0 || e.mem.u8(GUARD_0120045D) != 0 {
        return;
    }
    let source = e.mem.u32(SOURCE_011F9508);
    for i in 0..16u32 {
        let v = e.mem.u32(source.wrapping_add(0x980 + 4 * i));
        e.mem.set_u32(MATRIX_01200470 + 4 * i, v);
    }
    for i in 0..16u32 {
        let v = e.mem.u32(source.wrapping_add(0x9c0 + 4 * i));
        e.mem.set_u32(MATRIX_012004B0 + 4 * i, v);
    }
    e.mem.set_u8(FLAG_0120045E, 1);
    e.mem.set_u8(FLAG_011ADCDC, 0);
    for (i, offset) in [0x68u32, 0x74, 0x80].into_iter().enumerate() {
        let v = x87_quiet_bits(e.mem.u32(object.addr().wrapping_add(offset)));
        e.mem.set_u32(FLOATS_012004F0 + 4 * i as u32, v);
    }
}

// Translated from 00ba94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPassArray::RenderPassArray` (engine map; the body matches):
/// constructs the pass array ([`ni_t_array_render_pass_ni_t_array`] with the size and a
/// grow step of 3), sets the vtable `010b8480`, clears `iSize` and sizes the array
/// ([`ni_t_array_render_pass_set_size`]). Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba94b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_render_pass_array_render_pass_array(
    e: &mut Engine,
    this: Ptr<RenderPassArray>,
    size: u32,
) -> Ptr<RenderPassArray> {
    let array = this.cast::<NiTArray>();
    ni_t_array_render_pass_ni_t_array(e, array, size as u16, 3);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.set(this, RenderPassArray::iSize, 0);
    ni_t_array_render_pass_set_size(e, array, size);
    this
}

// Translated from 00ba9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the render-pass array's destructor
/// ([`bs_shader_property_free_render_passes`] runs it before freeing each 0x14-byte array).
/// For each slot below `m_usESize` (+0xc) that holds a pass, runs the pass's destructor
/// inline (`usPassEnum` 0, light array freed, `ucExtraParam` 0) and frees the pass; then
/// stores `0x00ffdead` in `iSize`, sets the vtable `010b8480` and frees the buffer.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9520 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba9520(e: &mut Engine, this: Ptr<RenderPassArray>) {
    let array = this.cast::<NiTArray>();
    let mut index = 0u32;
    if e.get(array, NiTArray::m_usESize) != 0 {
        loop {
            let base = e.get(array, NiTArray::m_pBase);
            let pass = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if pass != 0 {
                let pass = Ptr::<RenderPass>::new(pass);
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.set(pass, RenderPass::usPassEnum, 0);
                if lights != 0 {
                    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, lights]);
                    e.set(pass, RenderPass::ppSceneLights, 0);
                }
                e.set(pass, RenderPass::ucExtraParam, 0);
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, pass]);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(array, NiTArray::m_usESize)) {
                break;
            }
        }
    }
    let base = e.get(array, NiTArray::m_pBase);
    e.set(this, RenderPassArray::iSize, 0x00ff_dead);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
}

/// Allocates a 0x14-byte render-pass array and constructs it with `size` (null when the
/// allocation fails).
fn new_render_pass_array(e: &mut Engine, size: u32) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x14]).u32();
    if block == 0 {
        return 0;
    }
    bs_shader_property_render_pass_array_render_pass_array(e, Ptr::new(block), size).addr()
}

// Translated from 00ba95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map; the body matches): when
/// `pRenderPassList` is null, makes a new render-pass array of the given size (-1 means 5).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba95e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_check_create_render_pass_array(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    size: i32,
) {
    if e.get(this, BSShaderProperty::pRenderPassList) != 0 {
        return;
    }
    let size = if size == -1 { 5 } else { size };
    let array = new_render_pass_array(e, size as u32);
    e.set(this, BSShaderProperty::pRenderPassList, array);
}

// Translated from 00ba9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::CheckCreateDepthMapRenderPassArray` (engine map; the body matches):
/// when `pDepthMapRenderPassList` is null, makes a new render-pass array of the given size.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9660 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_check_create_depth_map_render_pass_array(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    size: u32,
) {
    if e.get(this, BSShaderProperty::pDepthMapRenderPassList) != 0 {
        return;
    }
    let array = new_render_pass_array(e, size);
    e.set(this, BSShaderProperty::pDepthMapRenderPassList, array);
}

// Translated from 00ba9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPassArray::Add` (engine map; the body matches): `thiscall`, eight
/// words (geometry, pass enum, first-pass flag, light count, four light words). While
/// `iSize` is below `m_usESize` the pass in slot `iSize` is reused: its lights set
/// ([`bs_shader_property_render_pass_set_lights`] with the count and the four words), its
/// pass enum, first-pass flag and geometry stored and its last-pass flag, accumulation hint
/// and extra parameter cleared. Otherwise a new 0x10-byte pass is made
/// ([`bs_shader_property_render_pass_render_pass`] with all eight words; null when the
/// allocation fails), the array grows by `m_usGrowBy` when it is full, and the pass is
/// stored at index `m_usSize` (`00ba3aa0`). `iSize` goes up by one.
///
/// The light words are the four argument words; a light count above 4 makes the exe read
/// further up its caller's stack, which this translation does not have (it stops loudly).
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9ee0 - the FS:[0] exception frame (unwinding on a throw) is not translated
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_property_render_pass_array_add(
    e: &mut Engine,
    this: Ptr<RenderPassArray>,
    geometry: u32,
    pass_enum: u32,
    first_pass: u32,
    light_count: u32,
    light0: u32,
    light1: u32,
    light2: u32,
    light3: u32,
) {
    let array = this.cast::<NiTArray>();
    let lights = [light0, light1, light2, light3];
    let in_use = e.get(this, RenderPassArray::iSize);
    if in_use < u32::from(e.get(array, NiTArray::m_usESize)) {
        let base = e.get(array, NiTArray::m_pBase);
        let pass = Ptr::<RenderPass>::new(e.mem.u32(base.wrapping_add(in_use.wrapping_mul(4))));
        bs_shader_property_render_pass_set_lights(e, pass, light_count as u8, &lights);
        e.set(pass, RenderPass::usPassEnum, pass_enum as u16);
        e.set(pass, RenderPass::bFirstPass, first_pass as u8);
        e.set(pass, RenderPass::pGeometry, geometry);
        e.set(pass, RenderPass::bLastPass, 0);
        e.set(pass, RenderPass::eAccumulationHint, 0);
        e.set(pass, RenderPass::ucExtraParam, 0);
    } else {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x10]).u32();
        let pass = if block != 0 {
            bs_shader_property_render_pass_render_pass(
                e,
                Ptr::new(block),
                geometry,
                pass_enum as u16,
                first_pass as u8,
                light_count as u8,
                &lights,
            )
            .addr()
        } else {
            0
        };
        let used = u32::from(e.get(array, NiTArray::m_usSize));
        if used >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            ni_t_array_render_pass_set_size(e, array, grow.wrapping_add(used));
        }
        e.with_stack(4, |e, element| {
            e.mem.set_u32(element.addr(), pass);
            e.call(ARRAY_SET_AT_00BA3AA0, &args![this, used, element]);
        });
    }
    let in_use = e.get(this, RenderPassArray::iSize);
    e.set(this, RenderPassArray::iSize, in_use.wrapping_add(1));
}

// Translated from 00baa000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::FreeRenderPasses` (engine map; the body matches): clears
/// `iLastRenderPassState`, then destroys ([`fn_00ba9520`]), frees and clears each of the
/// seven render-pass lists that exists, in the order +0x3c, +0x40, +0x48, +0x4c, +0x44,
/// +0x50, +0x54.
pub fn bs_shader_property_free_render_passes(e: &mut Engine, this: Ptr<BSShaderProperty>) {
    e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    for offset in [0x3cu32, 0x40, 0x48, 0x4c, 0x44, 0x50, 0x54] {
        let slot = this.addr().wrapping_add(offset);
        let list = e.mem.u32(slot);
        if list != 0 {
            fn_00ba9520(e, Ptr::new(list));
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, list]);
            e.mem.set_u32(slot, 0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_91a0,
            bs_shader_property_set_flag_recurse(Ptr, u32, u32)
        ),
        (0x00ba_8ec0, render_pass_entry as AbiFn),
        entry!(0x00a8_1430, fn_00a81430(Ptr<BSShaderProperty>, u32, u32)),
        entry!(
            0x00a8_1460,
            ni_shade_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ba_8ab0,
            bs_shader_property_set_alpha(Ptr<BSShaderProperty>, u32)
        ),
        entry!(0x00ba_8b10, fn_00ba8b10(Ptr<BSShaderProperty>, u32, u32)),
        entry!(
            0x00ba_8b80,
            ni_t_array_render_pass_ni_t_array(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x00ba_8be0,
            bs_shader_property_post_link_object(Ptr<BSShaderProperty>, u32)
        ),
        entry!(0x00ba_8c00, fn_00ba8c00(Ptr<RenderPass>, u8)),
        entry!(0x00ba_8c30, fn_00ba8c30(Ptr<RenderPass>)),
        (0x00ba_8c50, set_lights_entry as AbiFn),
        entry!(
            0x00ba_8cd0,
            bs_shader_property_render_pass_destructor(Ptr<RenderPass>)
        ),
        entry!(
            0x00ba_8d00,
            bs_shader_property_set_motion_blurred(Ptr<BSShaderProperty>, u8)
        ),
        entry!(
            0x00ba_8d30,
            ni_t_array_render_pass_set_size(Ptr<NiTArray>, u32)
        ),
        entry!(
            0x00ba_8e50,
            bs_shader_property_bs_shader_property(Ptr<BSShaderProperty>) -> Ptr<BSShaderProperty>
        ),
        entry!(0x00ba_8f60, fn_00ba8f60(Ptr<BSShaderProperty>, u32)),
        entry!(
            0x00ba_9120,
            bs_shader_property_set_motion_blur_recurse(Ptr, u32)
        ),
        entry!(0x00ba_92c0, fn_00ba92c0(Ptr<BSShaderProperty>, u32)),
        entry!(0x00ba_9420, fn_00ba9420(Ptr)),
        entry!(
            0x00ba_94b0,
            bs_shader_property_render_pass_array_render_pass_array(
                Ptr<RenderPassArray>,
                u32,
            )
                -> Ptr<RenderPassArray>
        ),
        entry!(0x00ba_9520, fn_00ba9520(Ptr<RenderPassArray>)),
        entry!(
            0x00ba_95e0,
            bs_shader_property_check_create_render_pass_array(Ptr<BSShaderProperty>, i32)
        ),
        entry!(
            0x00ba_9660,
            bs_shader_property_check_create_depth_map_render_pass_array(Ptr<BSShaderProperty>, u32)
        ),
        entry!(
            0x00ba_9ee0,
            bs_shader_property_render_pass_array_add(
                Ptr<RenderPassArray>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00ba_a000,
            bs_shader_property_free_render_passes(Ptr<BSShaderProperty>)
        ),
    ]
}

#[cfg(test)]
mod render_pass_tests {
    use super::*;

    #[test]
    fn render_pass_without_lights_clears_the_pass() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for off in (0..0x10).step_by(4) {
            e.mem.set_u32(this + off, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        // The accumulator's call: geometry 0, pass 0x2f5, first pass, no
        // lights, and one more pushed word.
        let result = e.call(0x00ba_8ec0, &[this, 0, 0x0001_02f5, 0x0101, 0, 0]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0x2f5);
        assert_eq!(e.mem.u8(this + 6), 0);
        assert_eq!(e.mem.u8(this + 7), 1);
        assert_eq!(e.mem.bytes(this + 8, 4), vec![0, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let allocations = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .count();
        assert_eq!(allocations, 0);
    }

    #[test]
    fn render_pass_with_lights_copies_them_into_a_new_array() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ba_8ec0, &[this, 0x1234, 7, 0, 3, 0xa, 0xb, 0xc]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.mem.u16(this + 4), 7);
        assert_eq!(e.mem.u8(this + 7), 0);
        assert_eq!(e.mem.u8(this + 9), 3);
        assert_eq!(e.mem.u8(this + 0xa), 3);
        assert_eq!(e.mem.u8(this + 0xb), 0);
        let array = e.mem.u32(this + 0xc);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(array + i * 4)).collect::<Vec<_>>(),
            vec![0xa, 0xb, 0xc]
        );
        let sizes: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(sizes, vec![12]);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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

    /// Objects share one vtable: slot +0x18 answers the dword at +0x10 (non-zero: geometry),
    /// slot +0xc answers the dword at +0x14 (the node view, non-null: a node).
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_GEOMETRY as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(BS_SHADER_PROPERTY_SET_FLAG, |_, _| Ret::default());
        e
    }

    fn geometry(e: &mut Engine, property: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x18, property);
        object
    }

    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x14, object);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xa0, buffer);
        e.mem.set_u16(object + 0xa6, children.len() as u16);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *child);
        }
        object
    }

    #[test]
    fn at_00ba91a0_sets_the_flag_on_a_geometrys_shader_property() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0x7000);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        assert_eq!(
            calls(&e, NI_AV_OBJECT_GET_PROPERTY),
            vec![vec![geometry, 3]]
        );
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7000, 0x30, 1]]
        );
    }

    #[test]
    fn at_00ba91a0_skips_a_geometry_without_the_property_and_a_null_object() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        e.call(0x00ba_91a0, &args![0u32, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 1);
    }

    #[test]
    fn at_00ba91a0_walks_the_children_of_nodes_skipping_null_slots() {
        let mut e = rig();
        let first = geometry(&mut e, 0x7001);
        let second = geometry(&mut e, 0x7002);
        let inner = node(&mut e, &[second]);
        let outer = node(&mut e, &[first, 0, inner]);
        e.call(0x00ba_91a0, &args![outer, 0x30u32, 0u32]);
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7001, 0x30, 0], vec![0x7002, 0x30, 0]]
        );
    }

    #[test]
    fn at_00ba91a0_does_nothing_for_an_object_that_is_neither() {
        let mut e = rig();
        let plain = e.mem.alloc(0x20);
        e.mem.set_u32(plain, 0x0f00_e100);
        e.call(0x00ba_91a0, &args![plain, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).is_empty());
    }

    #[test]
    fn at_00ba91a0_a_node_without_children_has_nothing_to_visit() {
        let mut e = rig();
        let empty = node(&mut e, &[]);
        e.call(0x00ba_91a0, &args![empty, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
    }
}

#[cfg(test)]
mod shader_property_tests {
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

    /// Allocation from the test heap, frees recorded only, interlocked operations on the
    /// word, and the call log on.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00a81430_copies_the_shade_flags_after_the_base_copy() {
        let mut e = rig();
        e.register(COPY_MEMBERS_00A5D7A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x18, 0x1234);
        e.call(0x00a8_1430, &args![this, dest, 7u32]);
        assert_eq!(calls(&e, COPY_MEMBERS_00A5D7A0), vec![vec![this, dest, 7]]);
        assert_eq!(e.mem.u16(dest + 0x18), 0x1234);
    }

    #[test]
    fn ni_shade_property_deleting_destructor_frees_only_with_bit_0() {
        let mut e = rig();
        e.register(BASE_DESTRUCTOR_00A5C240, |_, _| Ret::default());
        e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
        assert_eq!(e.call(0x00a8_1460, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00a8_1460, &args![0x5000u32, 3u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![0x5000, 0x20]]);
        assert_eq!(calls(&e, BASE_DESTRUCTOR_00A5C240).len(), 2);
    }

    #[test]
    fn set_alpha_resets_the_pass_state_when_crossing_one() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        let cases: [(f32, u32, bool); 6] = [
            (1.0, 0.5f32.to_bits(), true),
            (0.5, 1.0f32.to_bits(), true),
            (0.5, 0.25f32.to_bits(), false),
            (2.0, 1.0f32.to_bits(), false),
            (f32::NAN, 0.5f32.to_bits(), false),
            (1.0, 0x7fc0_0000, false),
        ];
        for (old, new, resets) in cases {
            e.mem.set_f32(this + 0x28, old);
            e.mem.set_u32(this + 0x38, 9);
            e.call(0x00ba_8ab0, &args![this, new]);
            assert_eq!(e.mem.u32(this + 0x28), new);
            assert_eq!(e.mem.u32(this + 0x38) == 0, resets, "{old} -> {new:#x}");
        }
        e.call(0x00ba_8ab0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x28), 0x7fc0_0001);
    }

    #[test]
    fn fn_00ba8b10_copies_the_shader_fields() {
        let mut e = rig();
        e.register(COPY_MEMBERS_00A5D7A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x18, 5);
        e.mem.set_u32(this + 0x20, 0x11);
        e.mem.set_u32(this + 0x24, 0x22);
        e.mem.set_u32(this + 0x28, 0x7f80_0001);
        e.mem.set_f32(this + 0x2c, 0.5);
        e.mem.set_u32(this + 0x30, 0x33);
        e.mem.set_u32(this + 0x58, 0x44);
        e.mem.set_f32(this + 0x5c, 2.0);
        e.mem.set_u32(dest + 0x38, 9);
        e.call(0x00ba_8b10, &args![this, dest, 1u32]);
        assert_eq!(calls(&e, COPY_MEMBERS_00A5D7A0), vec![vec![this, dest, 1]]);
        assert_eq!(e.mem.u16(dest + 0x18), 5);
        assert_eq!(e.mem.u32(dest + 0x20), 0x11);
        assert_eq!(e.mem.u32(dest + 0x24), 0x22);
        assert_eq!(e.mem.u32(dest + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.f32(dest + 0x2c), 0.5);
        assert_eq!(e.mem.u32(dest + 0x30), 0);
        assert_eq!(e.mem.u32(dest + 0x38), 0);
        assert_eq!(e.mem.u32(dest + 0x58), 0x44);
        assert_eq!(e.mem.f32(dest + 0x5c), 2.0);
    }

    #[test]
    fn ni_t_array_constructor_allocates_the_capacity() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xdead);
        assert_eq!(
            e.call(0x00ba_8b80, &args![this, 0x1_0000u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        e.call(0x00ba_8b80, &args![this, 5u32, 2u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 20]]);
        assert_ne!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
    }

    #[test]
    fn post_link_object_sets_the_depth_bias_for_bit_26() {
        let mut e = rig();
        e.register(NI_OBJECT_NET_POST_LINK_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_8be0, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        e.mem.set_u32(this + 0x20, 0x0400_0000);
        e.call(0x00ba_8be0, &args![this, 0x77u32]);
        assert_eq!(e.mem.f32(this + 0x5c), 1.0);
        assert_eq!(
            calls(&e, NI_OBJECT_NET_POST_LINK_OBJECT)[0],
            vec![this, 0x77]
        );
    }

    #[test]
    fn fn_00ba8c00_and_fn_00ba8c30_make_and_free_the_light_array() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.call(0x00ba_8c00, &args![pass, 0x103u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        let array = e.mem.u32(pass + 0xc);
        assert_ne!(array, 0);
        e.call(0x00ba_8c30, &args![pass]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert_eq!(e.mem.u32(pass + 0xc), 0);
        e.call(0x00ba_8c30, &args![pass]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn set_lights_grows_the_array_only_past_the_maximum() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.call(0x00ba_8c50, &[pass, 2, 0xa, 0xb]);
        let first = e.mem.u32(pass + 0xc);
        assert_eq!(e.mem.u8(pass + 9), 2);
        assert_eq!(e.mem.u8(pass + 0xa), 2);
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (0xa, 0xb));
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ba_8c50, &[pass, 1, 0xc]);
        assert_eq!(e.mem.u32(pass + 0xc), first);
        assert_eq!(e.mem.u8(pass + 9), 1);
        assert_eq!(e.mem.u32(first), 0xc);
        e.call(0x00ba_8c50, &[pass, 3, 1, 2, 3]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, first]]
        );
        let second = e.mem.u32(pass + 0xc);
        assert_eq!(e.mem.u8(pass + 0xa), 3);
        assert_eq!(e.mem.u32(second + 8), 3);
        e.call(0x00ba_8c50, &[pass, 0]);
        assert_eq!(e.mem.u8(pass + 9), 0);
    }

    #[test]
    fn render_pass_destructor_clears_and_frees() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.mem.set_u16(pass + 4, 7);
        e.mem.set_u8(pass + 0xb, 1);
        e.mem.set_u32(pass + 0xc, 0x9000);
        e.call(0x00ba_8cd0, &args![pass]);
        assert_eq!(e.mem.u16(pass + 4), 0);
        assert_eq!(e.mem.u8(pass + 0xb), 0);
        assert_eq!(e.mem.u32(pass + 0xc), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9000]]
        );
    }

    #[test]
    fn set_motion_blurred_resets_the_state_only_on_a_change() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_8d00, &args![this, 0x100u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0, 9));
        e.call(0x00ba_8d00, &args![this, 1u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0x10, 0));
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_8d00, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0x38), 9);
        e.call(0x00ba_8d00, &args![this, 0u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0, 0));
    }

    fn array_with(e: &mut Engine, max: u16, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x14);
        let base = e.mem.alloc(4 * u32::from(max).max(1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *item);
        }
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, max);
        e.mem.set_u16(this + 0xa, items.len() as u16);
        e.mem
            .set_u16(this + 0xc, items.iter().filter(|i| **i != 0).count() as u16);
        e.mem.set_u16(this + 0xe, 2);
        this
    }

    #[test]
    fn set_size_shrinks_and_copies() {
        let mut e = rig();
        let this = array_with(&mut e, 4, &[1, 0, 3, 4]);
        let old = e.mem.u32(this + 4);
        e.call(0x00ba_8d30, &args![this, 4u32]);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        e.call(0x00ba_8d30, &args![this, 2u32]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 8), 2);
        let base = e.mem.u32(this + 4);
        assert_ne!(base, old);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 4)), (1, 0));
        assert_eq!(e.mem.u32(old + 8), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old]]
        );
    }

    #[test]
    fn set_size_grows_with_zeros_and_zero_drops_the_buffer() {
        let mut e = rig();
        // New buffers come filled with 0xee bytes, so the zeros are the translation's.
        e.register(MEMORY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for i in 0..a[1] {
                e.mem.set_u8(block + i, 0xee);
            }
            block.into_ret()
        });
        let grown = array_with(&mut e, 2, &[5, 6]);
        let old = e.mem.u32(grown + 4);
        e.mem.set_u32(old + 8, 0x1234);
        e.call(0x00ba_8d30, &args![grown, 3u32]);
        let base = e.mem.u32(grown + 4);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(base + 4 * i)).collect::<Vec<_>>(),
            vec![5, 6, 0]
        );
        e.call(0x00ba_8d30, &args![grown, 0u32]);
        assert_eq!(e.mem.u32(grown + 4), 0);
        assert_eq!(e.mem.u16(grown + 0xa), 0);
        assert_eq!(e.mem.u16(grown + 0xc), 0);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).last().unwrap()[1], base);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = rig();
        e.register(BASE_CONSTRUCTOR_00A5C200, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for off in (0..0x60).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        assert_eq!(e.call(0x00ba_8e50, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
        for off in [0x28, 0x2c, 0x30] {
            assert_eq!(e.mem.f32(this + off), 1.0);
        }
        for off in [
            0x20, 0x24, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x58, 0x5c,
        ] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
    }

    #[test]
    fn fn_00ba8f60_copies_and_shares_the_object_at_0xc() {
        let mut e = rig();
        e.put_vtable(0x0f00_1000, &[0; 0x27]);
        e.mem.set_u32(0x0f00_1000 + 4, 0x0f00_0004);
        e.mem.set_u32(0x0f00_1000 + 0x98, 0x0f00_0098);
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.register(0x0f00_0098, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, 0x7f80_0001);
        e.mem.set_u32(this + 0x58, 3);
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(dest + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0x58), 3);
        assert!(calls(&e, 0x0f00_0098).is_empty());
        let shared = e.mem.alloc(8);
        e.mem.set_u32(shared, 0x0f00_1000);
        e.mem.set_u32(shared + 4, 1);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0f00_1000);
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(this + 0xc, shared);
        e.mem.set_u32(dest + 0xc, old);
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(dest + 0xc), shared);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![old]]);
        assert_eq!(calls(&e, 0x0f00_0098), vec![vec![shared, dest]]);
        // The same object: no reference changes.
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls(&e, 0x0f00_0098).len(), 2);
    }

    #[test]
    fn set_motion_blur_recurse_reaches_geometry_through_nodes() {
        let mut e = rig();
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_TRI_BASED_GEOM as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        let property = e.mem.alloc(0x60);
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, 0x0f00_e100);
        e.mem.set_u32(geometry + 0x10, 1);
        e.mem.set_u32(geometry + 0x18, property);
        let bare = e.mem.alloc(0x20);
        e.mem.set_u32(bare, 0x0f00_e100);
        e.mem.set_u32(bare + 0x10, 1);
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, 0x0f00_e100);
        e.mem.set_u32(node + 0x14, node);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, geometry);
        e.mem.set_u32(buffer + 8, bare);
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, 3);
        e.call(0x00ba_9120, &args![node, 1u32]);
        assert_eq!(e.mem.u32(property + 0x20), 0x10);
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 2);
        e.call(0x00ba_9120, &args![0u32, 1u32]);
        e.call(0x00ba_9120, &args![geometry, 0u32]);
        assert_eq!(e.mem.u32(property + 0x20), 0);
    }

    #[test]
    fn fn_00ba92c0_reads_shader_flags_and_scale() {
        let mut e = rig();
        e.register(BASE_LOAD_BINARY_00A95190, |_, _| Ret::default());
        // The read function writes the next value of a counter at +0x10 of the file.
        e.register(0x0f00_0100, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let next = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[1], e.mem.u32(a[0] + 0x14 + 4 * next));
            e.mem.set_u32(a[0] + 0x10, next + 1);
            4u32.into_ret()
        });
        let file = e.mem.alloc(0x40);
        e.mem.set_u32(file + 8, 0x0f00_0100);
        for (i, v) in [0x21u32, 0x1, 0x2000, 0x3f80_0000].iter().enumerate() {
            e.mem.set_u32(file + 0x14 + 4 * i as u32, *v);
        }
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_92c0, &args![this, stream]);
        assert_eq!(
            calls(&e, BASE_LOAD_BINARY_00A95190),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x58), 0x21);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        assert_eq!(e.mem.u32(this + 0x24), 0xa000);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.f32(this + 0x30), 1.0);
        assert_eq!(e.mem.u32(file + 4), 16);
        let reads: Vec<u32> = calls(&e, 0x0f00_0100).iter().map(|w| w[1]).collect();
        assert_eq!(
            reads,
            vec![this + 0x58, this + 0x20, this + 0x24, this + 0x30]
        );
    }

    #[test]
    fn fn_00ba9420_copies_the_matrices_unless_guarded() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        let source = e.mem.alloc(0xa00);
        for i in 0..32 {
            e.mem.set_u32(source + 0x980 + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(SOURCE_011F9508, source);
        e.mem.set_u8(FLAG_011ADCDC, 1);
        let object = e.mem.alloc(0x90);
        e.mem.set_f32(object + 0x68, 1.0);
        e.mem.set_u32(object + 0x74, 0x7f80_0001);
        e.mem.set_f32(object + 0x80, 3.0);
        e.mem.set_u8(GUARD_0120045D, 1);
        e.call(0x00ba_9420, &args![object]);
        assert_eq!(e.mem.u8(FLAG_0120045E), 0);
        e.mem.set_u8(GUARD_0120045D, 0);
        e.call(0x00ba_9420, &args![object]);
        assert_eq!(e.mem.u32(MATRIX_01200470), 0x100);
        assert_eq!(e.mem.u32(MATRIX_01200470 + 60), 0x10f);
        assert_eq!(e.mem.u32(MATRIX_012004B0 + 60), 0x11f);
        assert_eq!(e.mem.u8(FLAG_0120045E), 1);
        assert_eq!(e.mem.u8(FLAG_011ADCDC), 0);
        assert_eq!(e.mem.f32(FLOATS_012004F0), 1.0);
        assert_eq!(e.mem.u32(FLOATS_012004F0 + 4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(FLOATS_012004F0 + 8), 3.0);
    }

    #[test]
    fn render_pass_array_constructor_sizes_the_array() {
        let mut e = rig();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 9);
        assert_eq!(e.call(0x00ba_94b0, &args![this, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        // The constructor allocates the capacity; SetSize sees the same size and stops.
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
    }

    #[test]
    fn render_pass_array_destructor_frees_the_passes() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.mem.set_u32(pass + 0xc, 0x9100);
        e.mem.set_u16(pass + 4, 3);
        let this = array_with(&mut e, 3, &[0, pass]);
        e.mem.set_u16(this + 0xc, 2);
        let base = e.mem.u32(this + 4);
        e.call(0x00ba_9520, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 0x9100],
                vec![MEMORY_MANAGER, pass],
                vec![MEMORY_MANAGER, base]
            ]
        );
        assert_eq!(e.mem.u16(pass + 4), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x00ff_dead);
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
    }

    #[test]
    fn check_create_render_pass_arrays() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_95e0, &args![this, -1i32]);
        let list = e.mem.u32(this + 0x3c);
        assert_eq!(e.mem.u16(list + 8), 5);
        e.call(0x00ba_95e0, &args![this, 7i32]);
        assert_eq!(e.mem.u32(this + 0x3c), list);
        e.call(0x00ba_9660, &args![this, 2u32]);
        let depth = e.mem.u32(this + 0x40);
        assert_eq!(e.mem.u16(depth + 8), 2);
        e.call(0x00ba_9660, &args![this, 4u32]);
        assert_eq!(e.mem.u32(this + 0x40), depth);
        // A failed allocation leaves the list null.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let other = e.mem.alloc(0x60);
        e.call(0x00ba_95e0, &args![other, 3i32]);
        e.call(0x00ba_9660, &args![other, 3u32]);
        assert_eq!((e.mem.u32(other + 0x3c), e.mem.u32(other + 0x40)), (0, 0));
    }

    #[test]
    fn add_reuses_a_pass_then_appends_new_ones() {
        let mut e = rig();
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            e.mem.set_u16(a[0] + 0xc, size + 1);
            Ret::default()
        });
        let pass = e.mem.alloc(0x10);
        e.mem.set_u8(pass + 6, 1);
        e.mem.set_u8(pass + 8, 1);
        e.mem.set_u8(pass + 0xb, 1);
        let this = array_with(&mut e, 1, &[pass]);
        e.call(
            0x00ba_9ee0,
            &args![
                this,
                0x1234u32,
                0x2_0007u32,
                0x101u32,
                1u32,
                0xau32,
                0u32,
                0u32,
                0u32
            ],
        );
        assert_eq!(e.mem.u32(pass), 0x1234);
        assert_eq!(e.mem.u16(pass + 4), 7);
        assert_eq!(e.mem.u8(pass + 7), 1);
        assert_eq!(e.mem.bytes(pass + 8, 1), vec![0]);
        assert_eq!((e.mem.u8(pass + 6), e.mem.u8(pass + 0xb)), (0, 0));
        assert_eq!(e.mem.u8(pass + 9), 1);
        assert_eq!(e.mem.u32(e.mem.u32(pass + 0xc)), 0xa);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        // The array is full: it grows by 2 and the new pass goes to index 1.
        e.call(
            0x00ba_9ee0,
            &args![this, 0x5678u32, 9u32, 0u32, 2u32, 0xbu32, 0xcu32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u16(this + 8), 3);
        let set_at = calls(&e, ARRAY_SET_AT_00BA3AA0);
        assert_eq!(set_at.len(), 1);
        assert_eq!(set_at[0][1], 1);
        let added = e.mem.u32(e.mem.u32(this + 4) + 4);
        assert_eq!(e.mem.u32(added), 0x5678);
        assert_eq!(e.mem.u16(added + 4), 9);
        let lights = e.mem.u32(added + 0xc);
        assert_eq!((e.mem.u32(lights), e.mem.u32(lights + 4)), (0xb, 0xc));
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    #[test]
    fn free_render_passes_frees_each_list_in_order() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        let mut lists = vec![];
        for offset in [0x3cu32, 0x44, 0x48, 0x54] {
            let list = array_with(&mut e, 1, &[]);
            e.mem.set_u32(this + offset, list);
            lists.push(list);
        }
        e.call(0x00ba_a000, &args![this]);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        for offset in (0x3c..=0x54).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        let freed: Vec<u32> = calls(&e, MEMORY_DEALLOCATE)
            .iter()
            .map(|w| w[1])
            .filter(|a| lists.contains(a))
            .collect();
        // +0x48 before +0x44.
        assert_eq!(freed, vec![lists[0], lists[2], lists[1], lists[3]]);
    }
}
