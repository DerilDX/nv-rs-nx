//! `NiAnimation/niposdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPosData` (Xbox PDB), 0x18 bytes on PC (the same).
    pub struct NiPosData: 0x18 {
        /// `m_uiNumKeys` (Xbox PDB).
        0x08 m_uiNumKeys: u32,
        /// `m_pkKeys` (Xbox PDB): `NiPosKey*`.
        0x0C m_pkKeys: Ptr,
        /// `m_eType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eType: u32,
        /// `m_ucKeySize` (Xbox PDB).
        0x14 m_ucKeySize: u8,
    }
}

/// `NiPosData`'s vtable (stored by the constructor and destructor).
const NI_POS_DATA_VTABLE: u32 = 0x0109_717c;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiObject` constructor (`00a5d3a0`, thiscall).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The `NiObject` destructor (`00a5d3d0`, thiscall).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`, thiscall, stream).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The position keys' size in bytes per key type (one byte each).
const KEY_SIZES: u32 = 0x011f_375e;
/// Per key type: the function that creates `count` keys (cdecl: stream,
/// count).
const KEY_CREATORS: u32 = 0x011f_3dc8;
/// Per key type: the function that loads the keys (cdecl: keys, count, key
/// size).
const KEY_LOADERS: u32 = 0x011f_3798;
/// Per key type: the function that deletes an array of keys (cdecl, keys).
const KEY_DELETERS: u32 = 0x011f_38b8;
/// `NiStream`'s input file (+0x24C): read function at +8, position at +4.
const STREAM_INPUT: u32 = 0x24c;

/// Reads a 4-byte word from the stream's input into `destination`, the
/// byte-count word at `count` set to 4 first, and advances the position.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32, count: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(8));
    e.mem.set_u32(count, 4);
    let done = e
        .call(read, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(input.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(input.wrapping_add(4), position);
}

/// Deletes the keys at +0xC (when there are any) through the deleter of
/// the type at +0x10.
fn delete_keys(e: &mut Engine, this: Ptr<NiPosData>) {
    let keys = e.get(this, NiPosData::m_pkKeys).addr();
    if keys != 0 {
        let key_type = e.get(this, NiPosData::m_eType);
        let deleter = e.global::<u32>(KEY_DELETERS.wrapping_add(key_type.wrapping_mul(4)));
        e.call(deleter, &args![keys]);
    }
}

// Translated from 00a49020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData`'s create function (no function in Ghidra; read from the
/// bytes): allocates 0x18 bytes, runs the `NiObject` constructor, clears
/// the key fields and stores the vtable; null when the allocation fails.
pub fn fn_00a49020(e: &mut Engine) -> Ptr<NiPosData> {
    let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u8(object + 0x14, 0);
    e.mem.set_u32(object, NI_POS_DATA_VTABLE);
    Ptr::new(object)
}

// Translated from 00a490e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::ReplaceAnim` (Xbox PDB name in the engine map): deletes the
/// old keys, then takes `keys` (`count` of type `key_type`, with that type's
/// key size) when both are non-zero, otherwise clears all four fields.
pub fn ni_pos_data_replace_anim(
    e: &mut Engine,
    this: Ptr<NiPosData>,
    keys: u32,
    count: u32,
    key_type: u32,
) {
    delete_keys(e, this);
    if keys != 0 && count != 0 {
        let size = e.global::<u8>(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiPosData::m_pkKeys, Ptr::new(keys));
        e.set(this, NiPosData::m_ucKeySize, size);
        e.set(this, NiPosData::m_uiNumKeys, count);
        e.set(this, NiPosData::m_eType, key_type);
        return;
    }
    e.set(this, NiPosData::m_uiNumKeys, 0);
    e.set(this, NiPosData::m_pkKeys, Ptr::new(0));
    e.set(this, NiPosData::m_eType, 0);
    e.set(this, NiPosData::m_ucKeySize, 0);
}

// Translated from 00a49240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::LoadBinary` (Xbox PDB name in the engine map):
/// `NiObject::LoadBinary`, then the key count (read into the stream
/// argument's own stack word) and, for a non-zero count, the key type, the
/// type's key size (+0x14), new keys from the type's creator, loaded by the
/// type's loader, handed to `ReplaceAnim`.
pub fn ni_pos_data_load_binary(e: &mut Engine, this: Ptr<NiPosData>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    // Locals: the byte-count word (+0), the key type (+4), the stream
    // argument's stack word that receives the key count (+8).
    e.with_stack(0xc, |e, locals| {
        let size_word = locals.addr();
        let type_word = size_word + 4;
        let count_word = size_word + 8;
        read_word(e, stream, count_word, size_word);
        let count = e.mem.u32(count_word);
        if count == 0 {
            return;
        }
        read_word(e, stream, type_word, size_word);
        let key_type = e.mem.u32(type_word);
        let size = e.global::<u8>(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiPosData::m_ucKeySize, size);
        let creator = e.global::<u32>(KEY_CREATORS.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(creator, &args![stream, count]).u32();
        let size = e.get(this, NiPosData::m_ucKeySize) as u32;
        let loader = e.global::<u32>(KEY_LOADERS.wrapping_add(key_type.wrapping_mul(4)));
        e.call(loader, &args![keys, count, size]);
        ni_pos_data_replace_anim(e, this, keys, count, key_type);
    });
}

// Translated from 00a48fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPosData` destructor: stores the
/// vtable, deletes the keys and ends in the `NiObject` destructor (a tail
/// jump; its `EAX` is returned).
pub fn fn_00a48fb0(e: &mut Engine, this: Ptr<NiPosData>) -> u32 {
    e.mem.set_u32(this.addr(), NI_POS_DATA_VTABLE);
    delete_keys(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_9020, fn_00a49020() -> Ptr<NiPosData>),
        entry!(
            0x00a4_90e0,
            ni_pos_data_replace_anim(Ptr<NiPosData>, u32, u32, u32)
        ),
        entry!(0x00a4_9240, ni_pos_data_load_binary(Ptr<NiPosData>, Ptr)),
        entry!(0x00a4_8fb0, fn_00a48fb0(Ptr<NiPosData>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Key tables for type 2: size 0x10, creator 0x7000_0010, loader
    /// 0x7000_0020, deleter 0x7000_0030.
    fn key_tables(e: &mut Engine) {
        e.map(KEY_SIZES, 4);
        e.set_global(KEY_SIZES + 2, 0x10u8);
        for (table, function) in [
            (KEY_CREATORS, 0x7000_0010u32),
            (KEY_LOADERS, 0x7000_0020),
            (KEY_DELETERS, 0x7000_0030),
        ] {
            e.map(table, 0x10);
            e.set_global(table + 8, function);
        }
        e.register(0x7000_0010, |_, a| Ret {
            eax: 0x4400 + a[1],
            ..Ret::default()
        });
        e.register(0x7000_0020, |_, _| Ret::default());
        e.register(0x7000_0030, |_, _| Ret::default());
    }

    #[test]
    fn fn_00a49020_creates_empty_pos_data() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_9020, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xaa; 0x18]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let data = e.call(0x00a4_9020, &[]).u32();
        assert_eq!(e.mem.u32(data), NI_POS_DATA_VTABLE);
        assert_eq!(e.mem.bytes(data + 8, 0xd), vec![0; 0xd]);
        assert_eq!(e.mem.u8(data + 0x15), 0xaa);
    }

    #[test]
    fn replace_anim_deletes_the_old_keys_and_takes_or_clears() {
        let mut e = Engine::new();
        key_tables(&mut e);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x5500);
        e.mem.set_u32(this + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_90e0, &args![this, 0x6600u32, 5u32, 2u32]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x7000_0030, vec![0x5500])));
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 0x6600);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u8(this + 0x14), 0x10);
        e.call(0x00a4_90e0, &args![this, 0x6600u32, 0u32, 2u32]);
        assert_eq!(e.mem.bytes(this + 8, 0xd), vec![0; 0xd]);
        e.call_log = Some(vec![]);
        e.call(0x00a4_90e0, &args![this, 0u32, 3u32, 2u32]);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x7000_0030));
    }

    #[test]
    fn load_binary_reads_count_and_type_then_builds_the_keys() {
        let mut e = Engine::new();
        key_tables(&mut e);
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        // The input hands out the words 3 then 2.
        let mut words = vec![2u32, 3];
        e.register_double(0x7000_0008, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], words.pop().unwrap());
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9240, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0010, vec![stream, 3])));
        assert!(log.contains(&(0x7000_0020, vec![0x4403, 3, 0x10])));
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0x4403);
        assert_eq!(e.mem.u32(input + 4), 8);
    }

    #[test]
    fn load_binary_stops_after_a_zero_count() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x7000_0008, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call(0x00a4_9240, &args![this, stream]);
        assert_eq!(e.mem.u32(input + 4), 4, "one read");
    }

    #[test]
    fn fn_00a48fb0_deletes_the_keys_and_destroys_the_object() {
        let mut e = Engine::new();
        key_tables(&mut e);
        e.register(NI_OBJECT_DESTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x5500);
        e.mem.set_u32(this + 0x10, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_8fb0, &args![this]).u32(), this + 1);
        assert_eq!(e.mem.u32(this), NI_POS_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x7000_0030, vec![0x5500]));
        assert_eq!(log[2], (NI_OBJECT_DESTRUCT, vec![this]));
    }
}
