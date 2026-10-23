//! `BSShader/bsshadernolightingproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSShaderNoLightingProperty` is a `BSShaderProperty` with a base texture, the
//! texture's file name, a clamp mode and a falloff colour. The PC object is 0x80
//! bytes (the allocations in `00b6fe00` and `00b6fe80`); the Xbox PDB's 0x88 has
//! two more `BSShaderProperty` words (`fLastClearFrame`, `fLastAccumFrame`), so
//! the class's own fields sit 8 bytes lower on PC.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// `BSShaderNoLightingProperty`'s vtable (`010ae670`, stored by the constructor).
const VTABLE: u32 = 0x010a_e670;
/// `BSShaderNoLightingProperty::ms_RTTI` (`011fa05c`; its static initializer names it
/// `"BSShaderNoLightingProperty"`).
const RTTI: u32 = 0x011f_a05c;
/// `BSStripParticleSystem::ms_RTTI` (`01202560`; its static initializer names it
/// `"BSStripParticleSystem"`).
const STRIP_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_2560;
/// `BSShaderProperty::BSShaderProperty` (engine map, `00ba8e50`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00ba_8e50;
/// The base class's destructor body `00baa0e0` (`thiscall`, no stack words).
const BASE_DESTRUCT: u32 = 0x00ba_a0e0;
/// The base class's clone member copy `00ba8b10` (unit `bsshaderproperty.cpp`, `thiscall`:
/// the clone, the cloning process).
const BASE_COPY_MEMBERS: u32 = 0x00ba_8b10;
/// The base class's `LoadBinary` `00ba92c0` (`thiscall`: the stream).
const BASE_LOAD_BINARY: u32 = 0x00ba_92c0;
/// `NiObjectNET::PostLinkObject` (engine map, `00a5c6a0`, `thiscall`: the stream).
const NI_OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// The stream's string read `00a64ce0` (`thiscall`: the address of the word that receives
/// the allocated string).
const STREAM_READ_STRING: u32 = 0x00a6_4ce0;
/// The `BSStringT<char>` set `004037f0` (`thiscall`: the text, a length word).
const STRING_SET: u32 = 0x0040_37f0;
/// The Ni free `00aa10f0` (`cdecl`, the block).
const NI_FREE: u32 = 0x00aa_10f0;
/// The Ni allocation `00aa13e0` (`cdecl`, size) and the sized free `00aa1460` (`cdecl`: block,
/// size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `NiAVObject::GetProperty` (`00a59d30`), `NiAVObject::RemoveProperty_ov2` (`00a5b230`),
/// `NiAVObject::AttachProperty` (`00439410`) and `NiAVObject::UpdateProperties` (`00a5a040`)
/// (engine map; `thiscall`; the first three take one word).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The `NiAlphaProperty` constructor `004391c0` (`thiscall`, no stack words) and its setters
/// (`thiscall`, one word each): `0049ed90` (from flag bit 0), `004393c0` (from bit 9),
/// `00439390` (from bits 5..8) and `004393e0` (from bits 10..12).
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const ALPHA_SET_0049ED90: u32 = 0x0049_ed90;
const ALPHA_SET_004393C0: u32 = 0x0043_93c0;
const ALPHA_SET_00439390: u32 = 0x0043_9390;
const ALPHA_SET_004393E0: u32 = 0x0043_93e0;

layout! {
    /// `BSShaderNoLightingProperty` (Xbox PDB), 0x80 bytes on PC. The base class's words this
    /// unit touches are listed with their Xbox PDB names (same offsets on PC up to +0x5c).
    pub struct BSShaderNoLightingProperty: 0x80 {
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `BSShaderProperty::ulFlags[0]` (Xbox PDB).
        0x20 ulFlags0: u32,
        /// `BSShaderProperty::ulFlags[1]` (Xbox PDB).
        0x24 ulFlags1: u32,
        /// `BSShaderProperty::iLastRenderPassState` (Xbox PDB): cleared whenever a flag
        /// changes from clear to set.
        0x38 iLastRenderPassState: u32,
        /// `BSShaderProperty::iShader` (Xbox PDB).
        0x58 iShader: u32,
        /// `spBaseMap` (Xbox PDB): `NiPointer<NiTexture>`.
        0x60 spBaseMap: u32,
        /// `TextureFilename` (Xbox PDB): a `BSStringT<char>` (text, length, capacity).
        0x64 TextureFilename: Inline<BSStringT>,
        /// `TextureClampMode` (Xbox PDB).
        0x6C TextureClampMode: u32,
        /// `FalloffData` (Xbox PDB): an `NiColorA` (four `float`s, +0x70 to +0x7c).
        0x70 FalloffData: f32,
    }
}

/// A reference-counted object released: `InterlockedDecrement` on its count at +4, and
/// deleted through its vtable slot +4 when that answers 0.
fn release_reference(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// Sets `bit` of the flag word at `offset`, clearing `iLastRenderPassState` first when the
/// bit was clear (the property's inlined flag setter).
fn set_flag(e: &mut Engine, this: u32, offset: u32, bit: u32) {
    let flags = e.mem.u32(this + offset);
    if flags & bit == 0 {
        e.mem.set_u32(this + 0x38, 0);
    }
    let flags = e.mem.u32(this + offset);
    e.mem.set_u32(this + offset, flags | bit);
}

/// One read through the stream's reader (`+0x24c`): its function at `+8` called with (reader,
/// destination, 4, the address of the count word, 1) and the answer added to the reader's
/// `+4`.
fn stream_read_word(e: &mut Engine, stream: u32, dest: u32, count: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read = e.mem.u32(reader + 8);
    let done = e.call(read, &args![reader, dest, 4u32, count, 1u32]).u32();
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
}

// Translated from 00b6f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLightingProperty::PostLinkObject` (vtable slot +0x6c, the Xbox PDB's
/// `PostLinkObject` slot; Ghidra made no function of it): `thiscall`, one word (the stream).
/// `NiObjectNET::PostLinkObject`, then, when the texture file name is set,
/// `BSShaderManager::GetTexture(name, 1, &spBaseMap, 0, 0)`.
pub fn bs_shader_no_lighting_property_post_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.call(NI_OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    let name = e.mem.u32(this + 0x64);
    if name != 0 {
        e.call(
            SHADER_MANAGER_GET_TEXTURE,
            &args![name, 1u32, this + 0x60, 0u32, 0u32],
        );
    }
}

// Translated from 00b6f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the clone member copy `CreateClone` (`00b6fe00`) runs
/// (`thiscall`: the clone, the cloning process). The base copy `00ba8b10`, then the clone's
/// `spBaseMap` takes this one's (`NiPointer` assignment: the old one released through
/// `InterlockedDecrement` and slot +4, the new one's count incremented), and the clamp mode
/// and the falloff colour are copied.
pub fn fn_00b6f540(e: &mut Engine, this: Ptr, clone: Ptr, process: u32) {
    let this = this.addr();
    let clone = clone.addr();
    e.call(BASE_COPY_MEMBERS, &args![this, clone, process]);
    let old = e.mem.u32(clone + 0x60);
    if old != e.mem.u32(this + 0x60) {
        if old != 0 {
            release_reference(e, old);
        }
        let map = e.mem.u32(this + 0x60);
        e.mem.set_u32(clone + 0x60, map);
        if map != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![map + 4]);
        }
    }
    for offset in [0x6cu32, 0x70, 0x74, 0x78, 0x7c] {
        let word = e.mem.u32(this + offset);
        e.mem.set_u32(clone + offset, word);
    }
}

// Translated from 00b6fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLightingProperty::LoadBinary` (vtable slot +0x4c, the Xbox PDB's `LoadBinary`
/// slot; Ghidra made no function of it): `thiscall`, one word (the stream). The base
/// `LoadBinary` `00ba92c0`, four bytes into the clamp mode, the texture file name (the
/// stream's string read `00a64ce0` into the argument's stack slot, cleared first; the string
/// set into `TextureFilename` with length word 0; the read string freed through `00aa10f0`),
/// and, when the stream's version word `+4` is at least 0x1f, the four falloff floats. The
/// count word handed to the reader is a local holding 4.
pub fn bs_shader_no_lighting_property_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, frame| {
        let count = frame.addr();
        let slot = count + 4;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(count, 4);
        stream_read_word(e, stream, this + 0x6c, count);
        e.mem.set_u32(slot, 0);
        e.call(STREAM_READ_STRING, &args![stream, slot]);
        let text = e.mem.u32(slot);
        e.call(STRING_SET, &args![this + 0x64, text, 0u32]);
        let text = e.mem.u32(slot);
        e.call(NI_FREE, &args![text]);
        if e.mem.u32(stream + 4) >= 0x1f {
            for offset in [0x70u32, 0x74, 0x78, 0x7c] {
                e.mem.set_u32(count, 4);
                stream_read_word(e, stream, this + offset, count);
            }
        }
    });
}

// Translated from 00b6fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLightingProperty::BSShaderNoLightingProperty` (engine map; `thiscall`, no stack
/// words): the base constructor, the vtable, no base map, an empty file name, clamp mode 3,
/// falloff 0, `iShader` 0x21, property type 0x15, and flag bits 0x80000000 of `ulFlags[0]`
/// and 1 of `ulFlags[1]` set (each clearing `iLastRenderPassState` when it was clear).
pub fn bs_shader_no_lighting_property_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u16(base + 0x68, 0);
    e.mem.set_u16(base + 0x6a, 0);
    e.mem.set_u32(base + 0x6c, 3);
    for offset in [0x70u32, 0x74, 0x78, 0x7c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x58, 0x21);
    e.mem.set_u32(base + 0x1c, 0x15);
    set_flag(e, base, 0x20, 0x8000_0000);
    set_flag(e, base, 0x24, 1);
    this
}

// Translated from 00b6fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLightingProperty::GetRTTI` (vtable slot +8; Ghidra made no function of it):
/// `&ms_RTTI` (`011fa05c`).
pub fn bs_shader_no_lighting_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI
}

// Translated from 00b6fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable slot +0xb4, the Xbox PDB's `GetBaseTexture` slot;
/// `thiscall`, no stack words): `spBaseMap`.
pub fn fn_00b6fd20(e: &mut Engine, this: Ptr<BSShaderNoLightingProperty>) -> u32 {
    e.get(this, BSShaderNoLightingProperty::spBaseMap)
}

// Translated from 00b6fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (unnamed in the engine map; the scalar deleting destructor
/// `00b6fef0` runs it; `thiscall`, no stack words): the vtable, `spBaseMap` released and
/// cleared, the file name's text freed through the memory manager (`00aa4060`) when set and
/// the string emptied, `spBaseMap` released again by its `NiPointer` destructor (null by
/// then), then the base destructor `00baa0e0`.
// Translation incomplete: 00b6fd30 - the C++ exception frame (FS:[0] chain, unwind states 2, 1 and 0 for the members) is not translated.
pub fn fn_00b6fd30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    let map = e.mem.u32(base + 0x60);
    if map != 0 {
        release_reference(e, map);
        e.mem.set_u32(base + 0x60, 0);
    }
    let text = e.mem.u32(base + 0x64);
    if text != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, text]);
    }
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u16(base + 0x6a, 0);
    e.mem.set_u16(base + 0x68, 0);
    let map = e.mem.u32(base + 0x60);
    if map != 0 {
        release_reference(e, map);
    }
    e.call(BASE_DESTRUCT, &args![base]);
}

// Translated from 00b6fe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreateClone` (vtable slot +0x48; unnamed in the engine map; `thiscall`, one word: the
/// cloning process): allocates 0x80 bytes, constructs a property there (a null allocation
/// stays null), runs the member copy `00b6f540` from this into it and answers it.
// Translation incomplete: 00b6fe00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b6fe00(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x80u32]).u32();
    let clone = if block != 0 {
        bs_shader_no_lighting_property_constructor(e, Ptr::new(block)).addr()
    } else {
        0
    };
    fn_00b6f540(e, this, Ptr::new(clone), process);
    clone
}

// Translated from 00b6fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words; the class's factory function by its body):
/// allocates 0x80 bytes and answers a property constructed there, 0 when the allocation
/// fails.
// Translation incomplete: 00b6fe80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b6fe80(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x80u32]).u32();
    if block == 0 {
        return 0;
    }
    bs_shader_no_lighting_property_constructor(e, Ptr::new(block)).addr()
}

// Translated from 00b6fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor (vtable slot 0; unnamed in the engine map): the destructor
/// body `00b6fd30`, then the 0x80 bytes freed through `00aa1460` when `flags & 1`. Answers
/// `this`.
pub fn fn_00b6fef0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b6fd30(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x80u32]);
    }
    this
}

// Translated from 00b700d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLightingProperty::SetupGeometry` (engine map; `thiscall`, one word: the
/// geometry). Sets flag 0x20 of `ulFlags[1]` when the geometry data (`+0xb8`) has vertex
/// colours (`m_pkColor`, `+0x28`) and flag 2 of `ulFlags[0]` when the geometry has a skin
/// instance (`+0xbc`). For a geometry whose vtable slot +0x30 answers non-zero and whose
/// `GetRTTI` is not `BSStripParticleSystem`'s, with an alpha property (`+0x9c`) whose flags
/// have bits 1..4 equal to 6 and bits 5..8 clear: removes the geometry's property 0
/// (`GetProperty(0)`, `RemoveProperty_ov2(0)`), makes a new `NiAlphaProperty` (0x1c bytes;
/// a null allocation is used as it is) from the old one's flag bits (bit 0, bit 9, bits 5..8,
/// bits 1..4 cleared, bits 10..12) and test reference, attaches it, updates the properties
/// and sets flag 0x80000 of `ulFlags[1]`. Answers 1 (`AL`).
// Translation incomplete: 00b700d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the alpha property allocation) is not translated.
pub fn bs_shader_no_lighting_property_setup_geometry(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
) -> bool {
    let this = this.addr();
    let data = e.mem.u32(geometry + 0xb8);
    if e.mem.u32(data + 0x28) != 0 && e.mem.u8(this + 0x24) & 0x20 == 0 {
        e.mem.set_u32(this + 0x38, 0);
        let flags = e.mem.u32(this + 0x24);
        e.mem.set_u32(this + 0x24, flags | 0x20);
    }
    if e.mem.u32(geometry + 0xbc) != 0 {
        set_flag(e, this, 0x20, 2);
    }
    if e.vcall(geometry, 0x30, &args![]).u32() == 0 {
        return true;
    }
    if e.vcall(geometry, 8, &args![]).u32() == STRIP_PARTICLE_SYSTEM_RTTI {
        return true;
    }
    let old = e.mem.u32(geometry + 0x9c);
    if old == 0 {
        return true;
    }
    let flags = e.mem.u16(old + 0x18);
    if flags as u8 & 0x1e != 0xc || flags & 0x1e0 != 0 {
        return true;
    }
    if e.call(GET_PROPERTY, &args![geometry, 0u32]).u32() != 0 {
        e.call(REMOVE_PROPERTY, &args![geometry, 0u32]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let alpha = if block != 0 {
        e.call(ALPHA_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let bit0 = e.mem.u8(old + 0x18) & 1;
    e.call(ALPHA_SET_0049ED90, &args![alpha, bit0 as u32]);
    let bit9 = (e.mem.u16(old + 0x18) >> 9) as u8 & 1;
    e.call(ALPHA_SET_004393C0, &args![alpha, bit9 as u32]);
    let bits5 = (e.mem.u16(old + 0x18) as u32 >> 5) & 0xf;
    e.call(ALPHA_SET_00439390, &args![alpha, bits5]);
    let cleared = e.mem.u16(alpha + 0x18) & 0xffe1;
    e.mem.set_u16(alpha + 0x18, cleared);
    let bits10 = (e.mem.u16(old + 0x18) as u32 >> 10) & 7;
    e.call(ALPHA_SET_004393E0, &args![alpha, bits10]);
    let reference = e.mem.u8(old + 0x1a);
    e.mem.set_u8(alpha + 0x1a, reference);
    e.call(ATTACH_PROPERTY, &args![geometry, alpha]);
    e.call(UPDATE_PROPERTIES, &args![geometry]);
    set_flag(e, this, 0x24, 0x80000);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_f4f0,
            bs_shader_no_lighting_property_post_link_object(Ptr, u32)
        ),
        entry!(0x00b6_f540, fn_00b6f540(Ptr, Ptr, u32)),
        entry!(
            0x00b6_fb20,
            bs_shader_no_lighting_property_load_binary(Ptr, u32)
        ),
        entry!(0x00b6_fc90, bs_shader_no_lighting_property_constructor(Ptr) -> Ptr),
        entry!(0x00b6_fd00, bs_shader_no_lighting_property_get_rtti(Ptr) -> u32),
        entry!(
            0x00b6_fd20,
            fn_00b6fd20(Ptr<BSShaderNoLightingProperty>) -> u32
        ),
        entry!(0x00b6_fd30, fn_00b6fd30(Ptr)),
        entry!(0x00b6_fe00, fn_00b6fe00(Ptr, u32) -> u32),
        entry!(0x00b6_fe80, fn_00b6fe80() -> u32),
        entry!(0x00b6_fef0, fn_00b6fef0(Ptr, u32) -> Ptr),
        entry!(0x00b7_00d0, bs_shader_no_lighting_property_setup_geometry(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// The two interlocked imports as plain memory operations (only `Engine::with_exe`
    /// registers the platform's).
    fn interlocked(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
    }

    /// A reference-counted object whose delete slot (+4) is `delete`.
    fn counted(e: &mut Engine, count: u32, delete: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, delete);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn post_link_object_gets_the_texture_only_with_a_file_name() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[NI_OBJECT_NET_POST_LINK_OBJECT, SHADER_MANAGER_GET_TEXTURE],
        );
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00b6_f4f0, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, NI_OBJECT_NET_POST_LINK_OBJECT),
            vec![vec![this, 0x55]]
        );
        assert!(calls(&e, SHADER_MANAGER_GET_TEXTURE).is_empty());
        e.mem.set_u32(this + 0x64, 0x1234);
        e.call(0x00b6_f4f0, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, SHADER_MANAGER_GET_TEXTURE),
            vec![vec![0x1234, 1, this + 0x60, 0, 0]]
        );
    }

    #[test]
    fn member_copy_swaps_the_base_map_and_copies_the_words() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_COPY_MEMBERS, 0x0000_1110]);
        interlocked(&mut e);
        let this = e.mem.alloc(0x80);
        let clone = e.mem.alloc(0x80);
        let old = counted(&mut e, 1, 0x0000_1110);
        let new = counted(&mut e, 3, 0x0000_1110);
        e.mem.set_u32(clone + 0x60, old);
        e.mem.set_u32(this + 0x60, new);
        for (i, offset) in [0x6cu32, 0x70, 0x74, 0x78, 0x7c].into_iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 + i as u32);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b6_f540, &args![this, clone, 7u32]);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![this, clone, 7]]);
        assert_eq!(calls(&e, 0x0000_1110), vec![vec![old]]);
        assert_eq!(e.mem.u32(clone + 0x60), new);
        assert_eq!(e.mem.u32(new + 4), 4);
        for (i, offset) in [0x6cu32, 0x70, 0x74, 0x78, 0x7c].into_iter().enumerate() {
            assert_eq!(e.mem.u32(clone + offset), 0x100 + i as u32);
        }
        // Same map: no reference traffic.
        e.call_log = Some(vec![]);
        e.call(0x00b6_f540, &args![this, clone, 7u32]);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    fn load_rig(version: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_LOAD_BINARY, STRING_SET, NI_FREE]);
        e.register(STREAM_READ_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x00ab_cd00);
            Ret::default()
        });
        let mut value = 0x10u32;
        e.register_double(0x0000_2220, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], value);
            value += 1;
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let reader = e.mem.alloc(12);
        e.mem.set_u32(reader + 8, 0x0000_2220);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 4, version);
        let this = e.mem.alloc(0x80);
        (e, this, stream, reader)
    }

    #[test]
    fn load_binary_reads_the_falloff_only_from_version_0x1f() {
        let (mut e, this, stream, reader) = load_rig(0x1e);
        e.call_log = Some(vec![]);
        e.call(0x00b6_fb20, &args![this, stream]);
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + 0x6c), 0x10);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![this + 0x64, 0x00ab_cd00, 0]]
        );
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x00ab_cd00]]);
        assert_eq!(e.mem.u32(reader + 4), 4);
        assert_eq!(e.mem.u32(this + 0x70), 0);

        let (mut e, this, stream, reader) = load_rig(0x1f);
        e.call(0x00b6_fb20, &args![this, stream]);
        assert_eq!(e.mem.u32(reader + 4), 20);
        for (i, offset) in [0x70u32, 0x74, 0x78, 0x7c].into_iter().enumerate() {
            assert_eq!(e.mem.u32(this + offset), 0x11 + i as u32);
        }
    }

    #[test]
    fn constructor_sets_the_defaults_and_flags() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x38, 9);
        e.mem.set_u32(this + 0x24, 1);
        assert_eq!(e.call(0x00b6_fc90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x6c), 3);
        assert_eq!(e.mem.u32(this + 0x58), 0x21);
        assert_eq!(e.mem.u32(this + 0x1c), 0x15);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x24), 1);
        // Bit 31 was clear, so the pass state was cleared.
        assert_eq!(e.mem.u32(this + 0x38), 0);
        // Both bits already set: the pass state stays.
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00b6_fc90, &args![this]);
        assert_eq!(e.mem.u32(this + 0x38), 9);
    }

    #[test]
    fn rtti_and_base_texture() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x60, 0x4444);
        assert_eq!(e.call(0x00b6_fd00, &args![this]).u32(), RTTI);
        assert_eq!(e.call(0x00b6_fd20, &args![this]).u32(), 0x4444);
    }

    #[test]
    fn destructor_releases_the_map_and_frees_the_name() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, MEMORY_FREE, 0x0000_1110]);
        interlocked(&mut e);
        let this = e.mem.alloc(0x80);
        let map = counted(&mut e, 1, 0x0000_1110);
        e.mem.set_u32(this + 0x60, map);
        e.mem.set_u32(this + 0x64, 0x7770);
        e.mem.set_u32(this + 0x68, 0x0010_0005);
        e.call_log = Some(vec![]);
        e.call(0x00b6_fd30, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(calls(&e, 0x0000_1110), vec![vec![map]]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0x7770]]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]]);
        // Nothing held: no release and no free.
        e.call_log = Some(vec![]);
        e.call(0x00b6_fd30, &args![this]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn create_clone_and_factory_construct_into_the_allocation() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT, BASE_COPY_MEMBERS]);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x6c, 5);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00b6_fe00, &args![this, 3u32]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x80]]);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x6c), 5);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![this, clone, 3]]);
        let made = e.call(0x00b6_fe80, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b6_fe80, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, NI_FREE_SIZED]);
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x00b6_fef0, &args![this, flags]).u32(), this);
        }
        assert_eq!(calls(&e, BASE_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0x80]; 2]);
    }

    struct Geometry {
        e: Engine,
        this: u32,
        geometry: u32,
        alpha: u32,
    }

    fn geometry_rig(slot_30: u32, rtti: u32, alpha_flags: u16) -> Geometry {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                REMOVE_PROPERTY,
                ALPHA_SET_0049ED90,
                ALPHA_SET_004393C0,
                ALPHA_SET_00439390,
                ALPHA_SET_004393E0,
                ATTACH_PROPERTY,
                UPDATE_PROPERTIES,
            ],
        );
        e.register(GET_PROPERTY, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let new_alpha = e.mem.alloc(0x1c);
        e.mem.set_u16(new_alpha + 0x18, 0xffff);
        e.register_double(NI_ALLOCATE, move |_, _| Ret {
            eax: new_alpha,
            ..Ret::default()
        });
        e.register(ALPHA_PROPERTY_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(0x0000_3330, move |_, _| Ret {
            eax: slot_30,
            ..Ret::default()
        });
        e.register_double(0x0000_3338, move |_, _| Ret {
            eax: rtti,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x34);
        e.mem.set_u32(vtable + 8, 0x0000_3338);
        e.mem.set_u32(vtable + 0x30, 0x0000_3330);
        let data = e.mem.alloc(0x40);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(geometry + 0xb8, data);
        let old = e.mem.alloc(0x1c);
        e.mem.set_u16(old + 0x18, alpha_flags);
        e.mem.set_u8(old + 0x1a, 0x80);
        e.mem.set_u32(geometry + 0x9c, old);
        let this = e.mem.alloc(0x80);
        Geometry {
            e,
            this,
            geometry,
            alpha: new_alpha,
        }
    }

    #[test]
    fn setup_geometry_sets_the_colour_and_skin_flags() {
        let mut g = geometry_rig(0, 0, 0);
        let data = g.e.mem.u32(g.geometry + 0xb8);
        g.e.mem.set_u32(data + 0x28, 1);
        g.e.mem.set_u32(g.geometry + 0xbc, 1);
        g.e.mem.set_u32(g.this + 0x38, 9);
        assert!(g.e.call(0x00b7_00d0, &args![g.this, g.geometry]).bool());
        assert_eq!(g.e.mem.u32(g.this + 0x24), 0x20);
        assert_eq!(g.e.mem.u32(g.this + 0x20), 2);
        assert_eq!(g.e.mem.u32(g.this + 0x38), 0);
        // Bits already set: the pass state stays.
        g.e.mem.set_u32(g.this + 0x38, 9);
        g.e.call(0x00b7_00d0, &args![g.this, g.geometry]);
        assert_eq!(g.e.mem.u32(g.this + 0x38), 9);
    }

    #[test]
    fn setup_geometry_replaces_an_additive_alpha_property() {
        // Bits 1..4 = 6, bit 0, bit 9 and bits 10..12 = 5.
        let flags: u16 = 0xc | 1 | 0x200 | (5 << 10);
        let mut g = geometry_rig(1, 0x1234, flags);
        g.e.call_log = Some(vec![]);
        assert!(g.e.call(0x00b7_00d0, &args![g.this, g.geometry]).bool());
        let alpha = g.alpha;
        assert_eq!(calls(&g.e, REMOVE_PROPERTY), vec![vec![g.geometry, 0]]);
        assert_eq!(calls(&g.e, ALPHA_SET_0049ED90), vec![vec![alpha, 1]]);
        assert_eq!(calls(&g.e, ALPHA_SET_004393C0), vec![vec![alpha, 1]]);
        assert_eq!(calls(&g.e, ALPHA_SET_00439390), vec![vec![alpha, 0]]);
        assert_eq!(calls(&g.e, ALPHA_SET_004393E0), vec![vec![alpha, 5]]);
        assert_eq!(g.e.mem.u16(alpha + 0x18), 0xffe1);
        assert_eq!(g.e.mem.u8(alpha + 0x1a), 0x80);
        assert_eq!(calls(&g.e, ATTACH_PROPERTY), vec![vec![g.geometry, alpha]]);
        assert_eq!(calls(&g.e, UPDATE_PROPERTIES), vec![vec![g.geometry]]);
        assert_eq!(g.e.mem.u32(g.this + 0x24), 0x80000);
    }

    #[test]
    fn setup_geometry_leaves_other_alpha_properties_and_strip_particles() {
        for (slot, rtti, flags) in [
            (0u32, 0u32, 0xcu16),
            (1, STRIP_PARTICLE_SYSTEM_RTTI, 0xc),
            (1, 0, 0xe),
            (1, 0, 0xc | 0x20),
        ] {
            let mut g = geometry_rig(slot, rtti, flags);
            g.e.call_log = Some(vec![]);
            assert!(g.e.call(0x00b7_00d0, &args![g.this, g.geometry]).bool());
            assert!(calls(&g.e, ATTACH_PROPERTY).is_empty());
            assert_eq!(g.e.mem.u32(g.this + 0x24), 0);
        }
        // No alpha property at all.
        let mut g = geometry_rig(1, 0, 0xc);
        g.e.mem.set_u32(g.geometry + 0x9c, 0);
        g.e.call_log = Some(vec![]);
        g.e.call(0x00b7_00d0, &args![g.this, g.geometry]);
        assert!(calls(&g.e, GET_PROPERTY).is_empty());
    }
}
