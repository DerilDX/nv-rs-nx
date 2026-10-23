//! `BSShader/run_00bae980` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of `SkyShaderProperty` the linker placed after `skyshaderproperty.cpp`: the
//! class's vtable is `010b8ce0` (stored by the constructor `00baeb80`; its `GetRTTI`
//! answers `012005b8`, which the static initializer at `00fba440` names
//! `"SkyShaderProperty"`). The PC object is 0x90 bytes (the allocation in `00baee40`); the
//! Xbox PDB's 0x98 has two more `BSShaderProperty` words, so the class's own fields sit 8
//! bytes lower on PC.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// `SkyShaderProperty`'s vtable (`010b8ce0`).
const VTABLE: u32 = 0x010b_8ce0;
/// `SkyShaderProperty::ms_RTTI` (`012005b8`).
const RTTI: u32 = 0x0120_05b8;
/// The `u16` counter that numbers the cloud layers as they load (`012005b4`): a property of
/// sky object type 3 takes its value as `usCloudLayer` and counts it up.
const CLOUD_LAYER_COUNTER: u32 = 0x0120_05b4;
/// The `NiColorA` the constructor copies into `VertexColor` (`011a9be0`, four `float`s).
const DEFAULT_VERTEX_COLOR: u32 = 0x011a_9be0;
/// `BSShaderProperty::BSShaderProperty` (engine map, `00ba8e50`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00ba_8e50;
/// The base class's destructor body `00baa0e0` (`thiscall`, no stack words).
const BASE_DESTRUCT: u32 = 0x00ba_a0e0;
/// The base class's `LoadBinary` `00ba92c0` (`thiscall`: the stream).
const BASE_LOAD_BINARY: u32 = 0x00ba_92c0;
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
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

layout! {
    /// `SkyShaderProperty` (Xbox PDB), 0x90 bytes on PC.
    pub struct SkyShaderProperty: 0x90 {
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `VertexColor` (Xbox PDB): an `NiColorA` (four `float`s, +0x60 to +0x6c).
        0x60 VertexColor: f32,
        /// `spBaseMap` (Xbox PDB): `NiPointer<NiTexture>`.
        0x70 spBaseMap: u32,
        /// `TextureFilename` (Xbox PDB): a `BSStringT<char>`.
        0x74 TextureFilename: Inline<BSStringT>,
        /// `TextureClampMode` (Xbox PDB).
        0x7C TextureClampMode: u32,
        /// `spBlendTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x80 spBlendTexture: u32,
        /// `fBlendValue` (Xbox PDB).
        0x84 fBlendValue: f32,
        /// `usCloudLayer` (Xbox PDB).
        0x88 usCloudLayer: u16,
        /// `soType` (Xbox PDB): the sky object type (3 for clouds by `LoadBinary`'s use).
        0x8C soType: u32,
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

// Translated from 00baea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty::LoadBinary` (vtable slot +0x4c, the Xbox PDB's `LoadBinary` slot;
/// Ghidra made no function of it): `thiscall`, one word (the stream). The base `LoadBinary`
/// `00ba92c0`, four bytes into the clamp mode, the texture file name (the stream's string
/// read `00a64ce0` into the argument's stack slot, cleared first; set into
/// `TextureFilename` with length word 0; the read string freed through `00aa10f0`), four
/// bytes into `soType`; a cloud property (`soType` 3) takes the cloud layer counter's value
/// as `usCloudLayer` and counts it up. The count word handed to the reader is a local holding
/// 4.
pub fn sky_shader_property_load_binary(e: &mut Engine, this: Ptr<SkyShaderProperty>, stream: u32) {
    let base = this.addr();
    e.call(BASE_LOAD_BINARY, &args![base, stream]);
    e.with_stack(8, |e, frame| {
        let count = frame.addr();
        let slot = count + 4;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(count, 4);
        stream_read_word(e, stream, base + 0x7c, count);
        e.mem.set_u32(slot, 0);
        e.call(STREAM_READ_STRING, &args![stream, slot]);
        let text = e.mem.u32(slot);
        e.call(STRING_SET, &args![base + 0x74, text, 0u32]);
        let text = e.mem.u32(slot);
        e.call(NI_FREE, &args![text]);
        e.mem.set_u32(count, 4);
        stream_read_word(e, stream, base + 0x8c, count);
    });
    if e.get(this, SkyShaderProperty::soType) == 3 {
        let layer = e.global::<u16>(CLOUD_LAYER_COUNTER);
        e.set(this, SkyShaderProperty::usCloudLayer, layer);
        e.set_global(CLOUD_LAYER_COUNTER, layer.wrapping_add(1));
    }
}

// Translated from 00baeb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s constructor (unnamed in the engine map; it stores the class's
/// vtable; `thiscall`, no stack words): the base constructor, the vtable, `VertexColor`
/// zeroed, no base map, an empty file name, clamp mode 3, no blend texture, blend value 0,
/// cloud layer 0, `soType` 8, `VertexColor` copied from `011a9be0`, property type 0xd.
///
/// The compiler kept the `NiPointer` release of the blend texture's old value after storing
/// 0 into it; that value is the 0 it just stored, so the release never runs. It is
/// translated as the code has it.
// Translation incomplete: 00baeb80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the members) is not translated.
pub fn fn_00baeb80(e: &mut Engine, this: Ptr<SkyShaderProperty>) -> Ptr<SkyShaderProperty> {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE);
    for offset in [0x60u32, 0x64, 0x68, 0x6c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x70, 0);
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u16(base + 0x78, 0);
    e.mem.set_u16(base + 0x7a, 0);
    e.mem.set_u32(base + 0x7c, 3);
    e.mem.set_u32(base + 0x80, 0);
    // The old blend texture as the compiler holds it: the 0 just stored.
    let old_blend = 0u32;
    if old_blend != 0 {
        release_reference(e, old_blend);
        e.mem.set_u32(base + 0x80, 0);
    }
    e.mem.set_f32(base + 0x84, 0.0);
    e.mem.set_u16(base + 0x88, 0);
    e.mem.set_u32(base + 0x8c, 8);
    for word in 0..4 {
        let value = e.mem.u32(DEFAULT_VERTEX_COLOR + 4 * word);
        e.mem.set_u32(base + 0x60 + 4 * word, value);
    }
    e.mem.set_u32(base + 0x1c, 0xd);
    this
}

// Translated from 00baec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty::GetRTTI` (vtable slot +8; Ghidra made no function of it): `&ms_RTTI`
/// (`012005b8`).
pub fn sky_shader_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI
}

// Translated from 00baec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s destructor body (unnamed in the engine map; the scalar deleting
/// destructor `00baeeb0` runs it; `thiscall`, no stack words): the vtable; the blend texture
/// released and cleared, blend value 0, `soType` 8, cloud layer 0; the base map released
/// and cleared; the blend texture's `NiPointer` destructor (null by then); the file name's
/// text freed through the memory manager (`00aa4060`) when set and the string emptied; the
/// base map's `NiPointer` destructor (null by then); the base destructor `00baa0e0`.
// Translation incomplete: 00baec90 - the C++ exception frame (FS:[0] chain, unwind states 3 to 0 for the members) is not translated.
pub fn fn_00baec90(e: &mut Engine, this: Ptr<SkyShaderProperty>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    let blend = e.mem.u32(base + 0x80);
    if blend != 0 {
        release_reference(e, blend);
        e.mem.set_u32(base + 0x80, 0);
    }
    e.mem.set_f32(base + 0x84, 0.0);
    e.mem.set_u32(base + 0x8c, 8);
    e.mem.set_u16(base + 0x88, 0);
    let map = e.mem.u32(base + 0x70);
    if map != 0 {
        release_reference(e, map);
        e.mem.set_u32(base + 0x70, 0);
    }
    let blend = e.mem.u32(base + 0x80);
    if blend != 0 {
        release_reference(e, blend);
    }
    let text = e.mem.u32(base + 0x74);
    if text != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, text]);
    }
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u16(base + 0x7a, 0);
    e.mem.set_u16(base + 0x78, 0);
    let map = e.mem.u32(base + 0x70);
    if map != 0 {
        release_reference(e, map);
    }
    e.call(BASE_DESTRUCT, &args![base]);
}

// Translated from 00baee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words; `SkyShaderProperty`'s factory function by
/// its body): allocates 0x90 bytes and answers a property constructed there (`00baeb80`), 0
/// when the allocation fails.
// Translation incomplete: 00baee40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00baee40(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00baeb80(e, Ptr::new(block)).addr()
}

// Translated from 00baeeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s scalar deleting destructor (vtable slot 0; unnamed in the engine
/// map): the destructor body `00baec90`, then the 0x90 bytes freed through `00aa1460` when
/// `flags & 1`. Answers `this`.
pub fn fn_00baeeb0(
    e: &mut Engine,
    this: Ptr<SkyShaderProperty>,
    flags: u32,
) -> Ptr<SkyShaderProperty> {
    fn_00baec90(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x90u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_ea50,
            sky_shader_property_load_binary(Ptr<SkyShaderProperty>, u32)
        ),
        entry!(
            0x00ba_eb80,
            fn_00baeb80(Ptr<SkyShaderProperty>) -> Ptr<SkyShaderProperty>
        ),
        entry!(0x00ba_ec70, sky_shader_property_get_rtti(Ptr) -> u32),
        entry!(0x00ba_ec90, fn_00baec90(Ptr<SkyShaderProperty>)),
        entry!(0x00ba_ee40, fn_00baee40() -> u32),
        entry!(
            0x00ba_eeb0,
            fn_00baeeb0(Ptr<SkyShaderProperty>, u32) -> Ptr<SkyShaderProperty>
        ),
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

    /// `InterlockedDecrement` as a plain memory operation (only `Engine::with_exe` registers
    /// the platform's).
    fn interlocked(e: &mut Engine) {
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

    fn load_rig(so_type: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_LOAD_BINARY, STRING_SET, NI_FREE]);
        e.register(STREAM_READ_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x00ab_cd00);
            Ret::default()
        });
        let mut values = vec![so_type, 2u32];
        e.register_double(0x0000_2220, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], values.pop().unwrap());
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let reader = e.mem.alloc(12);
        e.mem.set_u32(reader + 8, 0x0000_2220);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        e.map(CLOUD_LAYER_COUNTER, 2);
        e.set_global(CLOUD_LAYER_COUNTER, 7u16);
        let this = e.mem.alloc(0x90);
        (e, this, stream)
    }

    #[test]
    fn load_binary_numbers_only_cloud_layers() {
        let (mut e, this, stream) = load_rig(3);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ea50, &args![this, stream]);
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + 0x7c), 2);
        assert_eq!(e.mem.u32(this + 0x8c), 3);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![this + 0x74, 0x00ab_cd00, 0]]
        );
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x00ab_cd00]]);
        let reader = e.mem.u32(stream + 0x24c);
        assert_eq!(e.mem.u32(reader + 4), 8);
        assert_eq!(e.mem.u16(this + 0x88), 7);
        assert_eq!(e.global::<u16>(CLOUD_LAYER_COUNTER), 8);

        let (mut e, this, stream) = load_rig(4);
        e.call(0x00ba_ea50, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(e.global::<u16>(CLOUD_LAYER_COUNTER), 7);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.map(DEFAULT_VERTEX_COLOR, 16);
        for word in 0..4u32 {
            e.mem
                .set_u32(DEFAULT_VERTEX_COLOR + 4 * word, 0x3f80_0000 + word);
        }
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x80, 0x1234);
        assert_eq!(e.call(0x00ba_eb80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        for word in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x60 + 4 * word), 0x3f80_0000 + word);
        }
        assert_eq!(e.mem.u32(this + 0x7c), 3);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 8);
        assert_eq!(e.mem.u32(this + 0x1c), 0xd);
    }

    #[test]
    fn rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ba_ec70, &args![0u32]).u32(), RTTI);
    }

    #[test]
    fn destructor_releases_both_textures_and_frees_the_name() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, MEMORY_FREE, 0x0000_1110]);
        interlocked(&mut e);
        let this = e.mem.alloc(0x90);
        let map = counted(&mut e, 1, 0x0000_1110);
        let blend = counted(&mut e, 2, 0x0000_1110);
        e.mem.set_u32(this + 0x70, map);
        e.mem.set_u32(this + 0x80, blend);
        e.mem.set_u32(this + 0x74, 0x7770);
        e.mem.set_u16(this + 0x88, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ec90, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(calls(&e, 0x0000_1110), vec![vec![map]]);
        assert_eq!(e.mem.u32(blend + 4), 1);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 8);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0x7770]]);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ec90, &args![this]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn factory_constructs_into_the_allocation() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.map(DEFAULT_VERTEX_COLOR, 16);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ba_ee40, &args![]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x90]]);
        assert_eq!(e.mem.u32(made), VTABLE);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ba_ee40, &args![]).u32(), 0);
        assert_eq!(calls(&e, BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, NI_FREE_SIZED]);
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x00ba_eeb0, &args![this, flags]).u32(), this);
        }
        assert_eq!(calls(&e, BASE_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0x90]; 2]);
    }
}
