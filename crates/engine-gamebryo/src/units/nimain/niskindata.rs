//! `NiMain/niskindata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSkinData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SKIN_DATA_RTTI: u32 = 0x011f_5b24;

// Translated from 00a84d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_skin_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SKIN_DATA_RTTI)
}

/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor and destructor body (`00a5d3a0`, `00a5d3d0`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The base `LinkObject` (`00a89af0`, a bare `RET 4`).
const NI_OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream` reader of a link id (`00a645b0`) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::SetLastError` (`00a632e0`).
const NI_STREAM_SET_LAST_ERROR: u32 = 0x00a6_32e0;
/// `NiPointer<NiSkinPartition>::operator=` (`0066b0d0`).
const SKIN_PARTITION_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiTransform::LoadBinary` (`00a86c80`) and `NiSphere::LoadBinary`
/// (`00a69ac0`).
const NI_TRANSFORM_LOAD_BINARY: u32 = 0x00a8_6c80;
const NI_SPHERE_LOAD_BINARY: u32 = 0x00a6_9ac0;
/// Frees every bone's vertex data (`00a84ca0`).
const FREE_BONE_VERTEX_DATA: u32 = 0x00a8_4ca0;
/// The `BoneData` constructor the vector constructor runs (`00a84c90`).
const BONE_DATA_CONSTRUCT: u32 = 0x00a8_4c90;
/// `_vector_constructor_iterator` (`00401050`: the block, the size, the
/// count and the constructor).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `MemoryManager::Allocate` / `Deallocate` (`00aa3e40`, `00aa4060`) on the
/// singleton at `011f6238`.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The sized delete (`00aa1460`, cdecl: the block and the size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStricmp` (`00aa1360`), `sprintf` (`00ec623a`), the stream's
/// creator string (+0xE0), file name (+8), user version (+0xDC) and the
/// flag at +0x3B8, the strings and the message callback (`011ac2ac`) of
/// the old-format warning.
const NI_STRICMP: u32 = 0x00aa_1360;
const SPRINTF: u32 = 0x00ec_623a;
const STREAM_FILE_NAME: u32 = 0x08;
const STREAM_USER_VERSION: u32 = 0xdc;
const STREAM_CREATOR: u32 = 0xe0;
const STREAM_FLAG_3B8: u32 = 0x3b8;
const EXPECTED_CREATOR: u32 = 0x0109_e680;
const WARNING_WITH_FILE: u32 = 0x0109_e644;
const WARNING: u32 = 0x0109_e60c;
const WARNING_TITLE: u32 = 0x0109_e600;
const MESSAGE_CALLBACK: u32 = 0x011a_c2ac;
/// The stream's version (+0xD8) and the versions the code tests.
const STREAM_VERSION: u32 = 0xd8;
const VERSION_4_0_0_1: u32 = 0x0400_0001;
const VERSION_4_1_0_9: u32 = 0x0401_0009;
const VERSION_4_1_0_13: u32 = 0x0401_000d;
const VERSION_10_1_0_101: u32 = 0x0a01_0065;
/// `NiStream`'s input stream (+0x24C): its position (+4) and read function
/// (+8, cdecl: input, buffer, bytes, component sizes, component count).
const STREAM_INPUT: u32 = 0x24c;
/// The `NiSkinData` vtable (its RTTI locator names `.?AVNiSkinData@@`).
const NI_SKIN_DATA_VTABLE: u32 = 0x0109_e574;
/// `NiSkinData` on the PC: `m_spSkinPartition` +8, the root-parent-to-skin
/// transform +0xC, `m_pkBoneData` +0x40, `m_uiBones` +0x44; `BoneData`
/// 0x4C bytes: transform +0, bound +0x34, vertex data +0x44, count +0x48.
const PARTITION: u32 = 0x08;
const ROOT_TO_SKIN: u32 = 0x0c;
const BONE_DATA: u32 = 0x40;
const BONES: u32 = 0x44;
const BONE_DATA_SIZE: u32 = 0x4c;

/// Reads `bytes` bytes of components of `size` bytes into `buffer` through
/// the stream's input, and moves its position by what the read answers.
fn read(e: &mut Engine, stream: Ptr, buffer: u32, bytes: u32, size: u32) {
    let input = e.mem.u32(stream.addr() + STREAM_INPUT);
    let function = e.mem.u32(input + 8);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        e.call(function, &args![input, buffer, bytes, sizes, 1u32])
            .u32()
    });
    let position = e.mem.u32(input + 4);
    e.mem.set_u32(input + 4, position.wrapping_add(count));
}

/// A `MemoryManager` allocation of `count * size` bytes (all ones when the
/// product overflows).
fn allocate_array(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(0xffff_ffff);
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32()
}

// Translated from 00a84d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinData::CreateObject` (the class's stream factory; Ghidra made no
/// function of it): a new 0x48-byte object with the `NiObject`
/// constructor, the class's vtable, no skin partition and no bone data;
/// null when the allocation fails.
pub fn ni_skin_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x48u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SKIN_DATA_VTABLE);
    e.mem.set_u32(object + PARTITION, 0);
    e.mem.set_u32(object + BONE_DATA, 0);
    Ptr::new(object)
}

// Translated from 00a84ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinData::LinkObject` (Xbox PDB): for versions 4.0.0.1 up to before
/// 10.1.0.101 the skin partition from its link id.
pub fn ni_skin_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LINK_OBJECT, &args![this, stream]);
    let version = e.mem.u32(stream.addr() + STREAM_VERSION);
    if (VERSION_4_0_0_1..VERSION_10_1_0_101).contains(&version) {
        let partition = e
            .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
            .u32();
        e.call(
            SKIN_PARTITION_POINTER_ASSIGN,
            &args![this.addr() + PARTITION, partition],
        );
    }
}

// Translated from 00a84ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinData::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the
/// bones' vertex data and the bone data freed, the skin partition released,
/// the `NiObject` part, then the sized delete of 0x48 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_skin_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_SKIN_DATA_VTABLE);
    e.call(FREE_BONE_VERTEX_DATA, &args![this]);
    let bones = e.mem.u32(base + BONE_DATA);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, bones]);
    let partition = e.mem.u32(base + PARTITION);
    if partition != 0 && e.call(INTERLOCKED_DECREMENT, &args![partition + 4]).i32() == 0 {
        e.vcall(partition, 4, &args![]);
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00a84f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a84f50 - the /GS security cookie check (00ec408c) is not translated
/// `NiSkinData::LoadBinary` (Xbox PDB): the `NiObject` part, the
/// root-parent-to-skin transform, the bone count and the bone data
/// (`MemoryManager` block of `BoneData`s built by the vector constructor),
/// the partition's link id (versions 4.0.0.1 up to before 10.1.0.101), the
/// has-weights byte (from 4.1.0.13, else true). For files of user version
/// below 11 without the +0x3B8 flag, with bones and another creator than
/// the expected one, the stream's error is set to 3 and the warning
/// (with the file name when the stream has one) goes to the message
/// callback when set. Then each bone's transform, bound and vertex count,
/// and with weights its vertex data (`MemoryManager` block of 8-byte
/// entries, each index cleared, then index and weight read), else no
/// vertex data. Version 4.1.0.9 has one more byte, read and dropped.
pub fn ni_skin_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    let file = stream.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(
        NI_TRANSFORM_LOAD_BINARY,
        &args![base + ROOT_TO_SKIN, stream],
    );
    read(e, stream, base + BONES, 4, 4);
    let count = e.mem.u32(base + BONES);
    let block = allocate_array(e, count, BONE_DATA_SIZE);
    let bones = if block != 0 {
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![block, BONE_DATA_SIZE, count, BONE_DATA_CONSTRUCT],
        );
        block
    } else {
        0
    };
    e.mem.set_u32(base + BONE_DATA, bones);
    let version = e.mem.u32(file + STREAM_VERSION);
    if (VERSION_4_0_0_1..VERSION_10_1_0_101).contains(&version) {
        e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    }
    e.with_stack(0x200 + 8, |e, locals| {
        let weights = locals.addr();
        let dropped = locals.addr() + 1;
        let message = locals.addr() + 8;
        if e.mem.u32(file + STREAM_VERSION) >= VERSION_4_1_0_13 {
            read(e, stream, weights, 1, 1);
        } else {
            e.mem.set_u8(weights, 1);
        }
        if e.mem.u32(file + STREAM_USER_VERSION) < 0xb
            && e.mem.u32(file + STREAM_FLAG_3B8) == 0
            && e.mem.u32(base + BONES) != 0
            && e.call(NI_STRICMP, &args![file + STREAM_CREATOR, EXPECTED_CREATOR])
                .u32()
                != 0
        {
            e.call(NI_STREAM_SET_LAST_ERROR, &args![stream, 3u32]);
            if e.mem.u8(file + STREAM_FILE_NAME) != 0 {
                e.call(
                    SPRINTF,
                    &args![
                        message,
                        WARNING_WITH_FILE,
                        file + STREAM_FILE_NAME,
                        file + STREAM_CREATOR
                    ],
                );
            } else {
                e.call(SPRINTF, &args![message, WARNING, file + STREAM_CREATOR]);
            }
            let callback = e.global::<u32>(MESSAGE_CALLBACK);
            if callback != 0 {
                e.call(callback, &args![message, WARNING_TITLE, 0u32]);
            }
        }
        let mut index = 0u32;
        while index < e.mem.u32(base + BONES) {
            let bone = e
                .mem
                .u32(base + BONE_DATA)
                .wrapping_add(index.wrapping_mul(BONE_DATA_SIZE));
            e.call(NI_TRANSFORM_LOAD_BINARY, &args![bone, stream]);
            e.call(NI_SPHERE_LOAD_BINARY, &args![bone + 0x34, stream]);
            read(e, stream, bone + 0x48, 2, 2);
            if e.mem.u8(weights) != 0 {
                let verts = u32::from(e.mem.u16(bone + 0x48));
                let data = allocate_array(e, verts, 8);
                if data != 0 {
                    for vert in 0..verts {
                        e.mem.set_u16(data + 8 * vert, 0);
                    }
                }
                e.mem.set_u32(bone + 0x44, data);
                let mut vert = 0u32;
                while vert < u32::from(e.mem.u16(bone + 0x48)) {
                    let entry = e.mem.u32(bone + 0x44).wrapping_add(8 * vert);
                    read(e, stream, entry, 2, 2);
                    read(e, stream, entry + 4, 4, 4);
                    vert += 1;
                }
            } else {
                e.mem.set_u32(bone + 0x44, 0);
            }
            index += 1;
        }
        if e.mem.u32(file + STREAM_VERSION) == VERSION_4_1_0_9 {
            read(e, stream, dropped, 1, 1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a84d20, ni_skin_data_get_rtti() -> Ptr),
        entry!(0x00a84d30, ni_skin_data_create_object() -> Ptr),
        entry!(0x00a84ce0, ni_skin_data_link_object(Ptr, Ptr)),
        entry!(0x00a84ef0, ni_skin_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a84f50, ni_skin_data_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_skin_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a84d20, &args![]).u32(), NI_SKIN_DATA_RTTI);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn create_object_and_link_object() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        e.register(NI_OBJECT_LINK_OBJECT, nothing);
        e.register(SKIN_PARTITION_POINTER_ASSIGN, nothing);
        e.register(NI_STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| {
            0x900u32.into_ret()
        });
        let data = e.call(0x00a8_4d30, &args![]).u32();
        assert_eq!(e.mem.u32(data), NI_SKIN_DATA_VTABLE);
        let stream = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        for (version, links) in [
            (0x0400_0000u32, 0usize),
            (0x0400_0001, 1),
            (0x0a01_0064, 2),
            (0x0a01_0065, 2),
        ] {
            e.mem.set_u32(stream + STREAM_VERSION, version);
            e.call(0x00a8_4ce0, &args![data, stream]);
            assert_eq!(calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN).len(), links);
        }
        assert_eq!(
            calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN)[0],
            vec![data + PARTITION, 0x900]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_4d30, &args![]).u32(), 0);
    }

    #[test]
    fn deleting_destructor_frees_and_releases() {
        let mut e = Engine::new();
        e.register(FREE_BONE_VERTEX_DATA, nothing);
        e.register(MEMORY_MANAGER_DEALLOCATE, nothing);
        e.register(NI_OBJECT_DESTRUCT, nothing);
        e.register(SIZED_DELETE, nothing);
        e.register(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(0x0620_0004, nothing);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0620_0004);
        let partition = e.mem.alloc(8);
        e.mem.set_u32(partition, vtable);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + PARTITION, partition);
        e.mem.set_u32(this + BONE_DATA, 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_4ef0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SKIN_DATA_VTABLE);
        assert_eq!(calls_to(&e, FREE_BONE_VERTEX_DATA), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x777]]
        );
        assert_eq!(calls_to(&e, 0x0620_0004), vec![vec![partition]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a8_4ef0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x48]]);
    }

    /// A stream whose input reads from `bytes` in order.
    fn stream_with(e: &mut Engine, bytes: Vec<u8>, version: u32) -> u32 {
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, 0x0630_0000);
        let source = std::rc::Rc::new(std::cell::RefCell::new(bytes));
        e.register_double(0x0630_0000, move |e, a| {
            let mut source = source.borrow_mut();
            let taken: Vec<u8> = source.drain(..a[2] as usize).collect();
            e.mem.write(a[1], &taken);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x3c0);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 11);
        stream
    }

    fn load_doubles(e: &mut Engine) {
        e.register(NI_OBJECT_LOAD_BINARY, nothing);
        e.register(NI_TRANSFORM_LOAD_BINARY, nothing);
        e.register(NI_SPHERE_LOAD_BINARY, nothing);
        e.register(NI_STREAM_READ_LINK_ID, nothing);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, nothing);
    }

    #[test]
    fn load_binary_reads_bones_and_weights() {
        let mut e = Engine::new();
        load_doubles(&mut e);
        // 2 bones; has weights; bone 0: 1 vertex (index 7, weight 0.5);
        // bone 1: 0 vertices.
        let mut bytes = vec![2, 0, 0, 0, 1, 1, 0, 7, 0];
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        bytes.extend_from_slice(&[0, 0]);
        let stream = stream_with(&mut e, bytes, 0x1400_0005);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a8_4f50, &args![this, stream]);
        let bones = e.mem.u32(this + BONE_DATA);
        assert_eq!(e.mem.u32(this + BONES), 2);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR),
            vec![vec![bones, 0x4c, 2, BONE_DATA_CONSTRUCT]]
        );
        assert!(calls_to(&e, NI_STREAM_READ_LINK_ID).is_empty());
        let verts = e.mem.u32(bones + 0x44);
        assert_eq!(e.mem.u16(verts), 7);
        assert_eq!(e.mem.f32(verts + 4), 0.5);
        assert_eq!(e.mem.u16(bones + 0x4c + 0x48), 0);
        assert_eq!(
            calls_to(&e, NI_TRANSFORM_LOAD_BINARY)[1],
            vec![bones, stream]
        );
        assert_eq!(
            calls_to(&e, NI_SPHERE_LOAD_BINARY)[1],
            vec![bones + 0x4c + 0x34, stream]
        );
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + 4), 4 + 1 + 2 + 2 + 4 + 2);
    }

    #[test]
    fn load_binary_of_old_files_warns_and_skips_weights() {
        let mut e = Engine::new();
        load_doubles(&mut e);
        e.register(NI_STRICMP, |_, _| 1u32.into_ret());
        e.register(NI_STREAM_SET_LAST_ERROR, nothing);
        e.register(SPRINTF, nothing);
        e.register(0x0630_0100, nothing);
        e.map(MESSAGE_CALLBACK, 4);
        e.set_global(MESSAGE_CALLBACK, 0x0630_0100u32);
        // Version 4.1.0.9: the link id, no weights byte (weights on), the
        // trailing byte. One bone of 0 vertices.
        let stream = stream_with(&mut e, vec![1, 0, 0, 0, 0, 0, 9], VERSION_4_1_0_9);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 10);
        e.mem.set_u8(stream + STREAM_FILE_NAME, b'a');
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a8_4f50, &args![this, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID).len(), 1);
        assert_eq!(
            calls_to(&e, NI_STREAM_SET_LAST_ERROR),
            vec![vec![stream, 3]]
        );
        let printed = calls_to(&e, SPRINTF);
        assert_eq!(
            printed[0][1..],
            [WARNING_WITH_FILE, stream + 8, stream + 0xe0]
        );
        let callback = calls_to(&e, 0x0630_0100);
        assert_eq!(callback[0][0], printed[0][0]);
        assert_eq!(callback[0][1..], [WARNING_TITLE, 0]);
        let bones = e.mem.u32(this + BONE_DATA);
        assert_ne!(e.mem.u32(bones + 0x44), 0);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + 4), 7);
        // Without weights (from 4.1.0.13, flag 0): no vertex data.
        let mut e = Engine::new();
        load_doubles(&mut e);
        let stream = stream_with(&mut e, vec![1, 0, 0, 0, 0, 0, 0], VERSION_4_1_0_13);
        let this = e.mem.alloc(0x48);
        e.call(0x00a8_4f50, &args![this, stream]);
        let bones = e.mem.u32(this + BONE_DATA);
        assert_eq!(e.mem.u32(bones + 0x44), 0);
    }
}
