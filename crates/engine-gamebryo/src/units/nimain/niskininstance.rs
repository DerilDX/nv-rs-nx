//! `NiMain/niskininstance.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiAlloc` (`00aa1070`, cdecl, the size).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiObject`'s constructor (`00a5d3a0`, `this` only).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject::LoadBinary` (`00a5d610`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiObject::CopyMembers` (`00a5d450`: `this`, the clone and the cloning
/// process).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// The base `LinkObject` (`00a89af0`, a bare `RET 4`).
const NI_OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream` reader of a link id (`00a645b0`, the stream only).
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`).
const NI_STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`).
const NI_STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// The stream's file version (+0xD8) and the first version with a skin
/// partition link (`10.1.0.101`).
const STREAM_VERSION: u32 = 0xd8;
const VERSION_10_1_0_101: u32 = 0x0a01_0065;
/// `NiPointer<NiSkinPartition>::operator=` (`0066b0d0`: the holder and the
/// object).
const SKIN_PARTITION_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The clone map lookup (`00a53c30`: the map, the key and the address of
/// the result; answers whether it is there).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `InterlockedDecrement` / `InterlockedIncrement` (import slots).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiBound::Update` (`00a7fdd0`: the bound, the source bound and the
/// transform) and `NiBound::Merge` (`00a7f3f0`: the bound and the other).
const NI_BOUND_UPDATE: u32 = 0x00a7_fdd0;
const NI_BOUND_MERGE: u32 = 0x00a7_f3f0;
/// The bound moved by a transform (`00a7fea0`).
const NI_BOUND_TRANSFORM: u32 = 0x00a7_fea0;
/// The inverse of a transform (`008b42e0`: the transform and the result).
const TRANSFORM_INVERT: u32 = 0x008b_42e0;
/// The product of two transforms (`0062c250`: the first, the result and the
/// second; answers the result).
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// The `NiSkinInstance` vtable (its RTTI locator names
/// `.?AVNiSkinInstance@@`).
const NI_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9b14;
/// `NiSkinInstance` (Xbox PDB field names): `m_spSkinData` +8,
/// `m_spSkinPartition` +0xC, `m_pkRootParent` +0x10, `m_ppkBones` +0x14,
/// `m_uiFrameID` +0x18.
const SKIN_DATA: u32 = 0x08;
const SKIN_PARTITION: u32 = 0x0c;
const ROOT_PARENT: u32 = 0x10;
const BONES: u32 = 0x14;
const FRAME_ID: u32 = 0x18;
/// `NiSkinData` on the PC: `m_spSkinPartition` +8, the root-parent-to-skin
/// transform +0xC, `m_pkBoneData` +0x40, `m_uiBones` +0x44; a `BoneData`
/// is 0x4C bytes with its bound at +0x34.
const DATA_PARTITION: u32 = 0x08;
const DATA_ROOT_TO_SKIN: u32 = 0x0c;
const DATA_BONE_DATA: u32 = 0x40;
const DATA_BONES: u32 = 0x44;
const BONE_DATA_SIZE: u32 = 0x4c;
const BONE_BOUND: u32 = 0x34;
/// `NiAVObject`'s world transform (+0x68; its translation +0x8C, scale
/// +0x98).
const WORLD_TRANSFORM: u32 = 0x68;
const WORLD_SCALE: u32 = 0x98;

/// Lets go of a reference-counted object: `InterlockedDecrement` of its
/// count (+4) and its virtual `+4` (`DeleteThis`) at 0.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

fn add_reference(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// `NiPointer::operator=` on the holder at `holder` with `value`.
fn assign(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old == value {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(holder, value);
    if value != 0 {
        add_reference(e, value);
    }
}

// Translated from 00a866f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CreateObject` (the class's stream factory; Ghidra made
/// no function of it): a new 0x34-byte object with the `NiObject`
/// constructor, the class's vtable, its pointers and counts cleared and
/// the frame id -1 (+0x2C is left as it is); null when the allocation
/// fails.
pub fn ni_skin_instance_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    construct_skin_instance(e, object);
    Ptr::new(object)
}

/// The constructor's stores, as `CreateObject` and `CreateClone` inline them.
fn construct_skin_instance(e: &mut Engine, object: u32) {
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SKIN_INSTANCE_VTABLE);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x30] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object + FRAME_ID, 0xffff_ffff);
}

// Translated from 00a86340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::LoadBinary` (Xbox PDB): the `NiObject` part, the skin
/// data's link id, the skin partition's (from version 10.1.0.101), the root
/// parent's, then the bones' link ids.
pub fn ni_skin_instance_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    }
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    e.call(NI_STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
}

fn linked(e: &mut Engine, stream: Ptr) -> u32 {
    e.call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
        .u32()
}

// Translated from 00a86380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::LinkObject` (Xbox PDB): the skin data, the skin
/// partition (from version 10.1.0.101) and the root parent from their link
/// ids, then the bone array (`NiAlloc` of the link count) filled from the
/// next links.
pub fn ni_skin_instance_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_LINK_OBJECT, &args![this, stream]);
    let data = linked(e, stream);
    assign(e, base + SKIN_DATA, data);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        let partition = linked(e, stream);
        assign(e, base + SKIN_PARTITION, partition);
    }
    let root = linked(e, stream);
    e.mem.set_u32(base + ROOT_PARENT, root);
    let count = e
        .call(NI_STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream])
        .u32();
    let bones = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
    e.mem.set_u32(base + BONES, bones);
    for index in 0..count {
        let bone = linked(e, stream);
        let bones = e.mem.u32(base + BONES);
        e.mem.set_u32(bones + 4 * index, bone);
    }
}

// Translated from 00a86740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::PostLinkObject` (Xbox PDB): before version 10.1.0.101
/// the skin partition moves from the skin data to the instance (assigned,
/// then the data's pointer released and cleared).
pub fn ni_skin_instance_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        return;
    }
    let data = e.mem.u32(base + SKIN_DATA);
    if data == 0 {
        return;
    }
    let partition = e.mem.u32(data + DATA_PARTITION);
    e.call(
        SKIN_PARTITION_POINTER_ASSIGN,
        &args![base + SKIN_PARTITION, partition],
    );
    let data = e.mem.u32(base + SKIN_DATA);
    let partition = e.mem.u32(data + DATA_PARTITION);
    if partition != 0 {
        release(e, partition);
        e.mem.set_u32(data + DATA_PARTITION, 0);
    }
}

// Translated from 00a86530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The skinned bound (unnamed in the engine map): the first bone's bound
/// placed by its world transform (`NiBound::Update`), merged with every
/// other bone's (moved by the translation only when the bone's world scale
/// is exactly 1.0, else `NiBound::Update`), then moved into skin space by
/// the root parent's inverse world transform times the data's
/// root-parent-to-skin transform (`00a7fea0`).
pub fn fn_00a86530(e: &mut Engine, this: Ptr, bound: Ptr) {
    let base = this.addr();
    let bones = e.mem.u32(base + BONES);
    let data = e.mem.u32(base + SKIN_DATA);
    let first_bone = e.mem.u32(bones);
    let bone_data = e.mem.u32(data + DATA_BONE_DATA);
    e.call(
        NI_BOUND_UPDATE,
        &args![bound, bone_data + BONE_BOUND, first_bone + WORLD_TRANSFORM],
    );
    let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
    e.with_stack(0x10, |e, local| {
        let local = local.addr();
        let mut source = bone_data.wrapping_add(BONE_DATA_SIZE + BONE_BOUND);
        let mut index = 1u32;
        while index < count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            let world = bone + WORLD_TRANSFORM;
            // FCOMP of 1.0 with the scale, C3 and C2 tested: equal.
            if 1.0 == x87_m32(e.mem.u32(bone + WORLD_SCALE)) {
                let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
                e.mem.set_u32(local + 0xc, radius);
                for axis in 0..3 {
                    let value = e.mem.u32(source + 4 * axis);
                    e.mem.set_u32(local + 4 * axis, value);
                }
                let mut moved = [0u32; 3];
                for (axis, slot) in moved.iter_mut().enumerate() {
                    let axis = axis as u32;
                    let offset = x87_load32(e.mem.u32(world + 0x24 + 4 * axis));
                    *slot = x87_store32(x87_add(offset, x87_m32(e.mem.u32(local + 4 * axis))));
                }
                for (axis, value) in moved.iter().enumerate() {
                    e.mem.set_u32(local + 4 * axis as u32, *value);
                }
            } else {
                e.call(NI_BOUND_UPDATE, &args![local, source, world]);
            }
            e.call(NI_BOUND_MERGE, &args![bound, local]);
            index += 1;
            source = source.wrapping_add(BONE_DATA_SIZE);
        }
    });
    e.with_stack(0x68, |e, transforms| {
        let inverse = transforms.addr();
        let product = transforms.addr() + 0x34;
        let root = e.mem.u32(base + ROOT_PARENT);
        e.call(TRANSFORM_INVERT, &args![root + WORLD_TRANSFORM, inverse]);
        let data = e.mem.u32(base + SKIN_DATA);
        let result = e
            .call(
                TRANSFORM_MULTIPLY,
                &args![data + DATA_ROOT_TO_SKIN, product, inverse],
            )
            .u32();
        let bytes = e.mem.bytes(result, 0x34);
        e.mem.write(inverse, &bytes);
        e.call(NI_BOUND_TRANSFORM, &args![bound, inverse]);
    });
}

// Translated from 00a86660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CopyMembers` (by its use in `CreateClone`): the
/// `NiObject` part, then the clone shares the skin data and the skin
/// partition (reference-counted assignments).
pub fn fn_00a86660(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    let base = this.addr();
    let target = clone.addr();
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, clone, cloning]);
    for offset in [SKIN_DATA, SKIN_PARTITION] {
        let old = e.mem.u32(target + offset);
        if old == e.mem.u32(base + offset) {
            continue;
        }
        if old != 0 {
            release(e, old);
        }
        let value = e.mem.u32(base + offset);
        e.mem.set_u32(target + offset, value);
        if value != 0 {
            add_reference(e, value);
        }
    }
}

/// The clone map's entry for `key` written at `out`; whether there is one.
fn cloned(e: &mut Engine, cloning: Ptr, key: u32, out: u32) -> bool {
    let map = e.mem.u32(cloning.addr());
    e.call(CLONE_MAP_GET_AT, &args![map, key, out]).bool()
}

// Translated from 00a862d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the cloning process clones the root parent or any bone
/// (unnamed in the engine map; `CreateClone` clones only then).
pub fn fn_00a862d0(e: &mut Engine, this: Ptr, cloning: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, out| {
        let root = e.mem.u32(base + ROOT_PARENT);
        if cloned(e, cloning, root, out.addr()) {
            return true;
        }
        let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
        for index in 0..count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            if cloned(e, cloning, bone, out.addr()) {
                return true;
            }
        }
        false
    })
}

// Translated from 00a86830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CreateClone` (Xbox PDB): `this` itself when nothing it
/// points at is cloned; otherwise a new instance (`CreateObject`'s stores,
/// null when the allocation fails, used as is) sharing the data, whose root
/// parent and bones are their clones where there is one.
pub fn ni_skin_instance_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let base = this.addr();
    if !fn_00a862d0(e, this, cloning) {
        return this;
    }
    let clone = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if clone != 0 {
        construct_skin_instance(e, clone);
    }
    fn_00a86660(e, this, Ptr::new(clone), cloning);
    e.with_stack(4, |e, out| {
        let out = out.addr();
        let root = e.mem.u32(base + ROOT_PARENT);
        let root = if cloned(e, cloning, root, out) {
            e.mem.u32(out)
        } else {
            e.mem.u32(base + ROOT_PARENT)
        };
        e.mem.set_u32(clone.wrapping_add(ROOT_PARENT), root);
        let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
        let bones = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
        e.mem.set_u32(clone.wrapping_add(BONES), bones);
        for index in 0..count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            let bone = if cloned(e, cloning, bone, out) {
                e.mem.u32(out)
            } else {
                e.mem.u32(e.mem.u32(base + BONES) + 4 * index)
            };
            let bones = e.mem.u32(clone.wrapping_add(BONES));
            e.mem.set_u32(bones + 4 * index, bone);
        }
    });
    Ptr::new(clone)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a866f0, ni_skin_instance_create_object() -> Ptr),
        entry!(0x00a86340, ni_skin_instance_load_binary(Ptr, Ptr)),
        entry!(0x00a86380, ni_skin_instance_link_object(Ptr, Ptr)),
        entry!(0x00a86740, ni_skin_instance_post_link_object(Ptr, Ptr)),
        entry!(0x00a86530, fn_00a86530(Ptr, Ptr)),
        entry!(0x00a86660, fn_00a86660(Ptr, Ptr, Ptr)),
        entry!(0x00a862d0, fn_00a862d0(Ptr, Ptr) -> bool),
        entry!(0x00a86830, ni_skin_instance_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn counting(e: &mut Engine) {
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
    }

    /// A reference-counted object with `count` references and a
    /// `DeleteThis` at slot +4 (`0x0620_0004`).
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0620_0004);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn create_object_sets_the_start_values() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        let object = e.call(0x00a8_66f0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(object + FRAME_ID), 0xffff_ffff);
        assert_eq!(e.mem.u32(object + 0x30), 0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_66f0, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_links_by_version() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing);
        e.register(NI_STREAM_READ_LINK_ID, nothing);
        e.register(NI_STREAM_READ_MULTIPLE_LINK_IDS, nothing);
        let stream = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6340, &args![0x10u32, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID).len(), 2);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        e.call(0x00a8_6340, &args![0x10u32, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID).len(), 5);
        assert_eq!(calls_to(&e, NI_STREAM_READ_MULTIPLE_LINK_IDS).len(), 2);
    }

    #[test]
    fn link_object_assigns_the_links() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_OBJECT_LINK_OBJECT, nothing);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(0x0620_0004, nothing);
        let data = counted(&mut e, 0);
        let partition = counted(&mut e, 0);
        let old = counted(&mut e, 1);
        let links = std::rc::Rc::new(std::cell::RefCell::new(vec![
            data, partition, 0x700, 0x801, 0x802,
        ]));
        let next = links.clone();
        e.register_double(NI_STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        e.register(NI_STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 2u32.into_ret());
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, old);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6380, &args![this, stream]);
        assert_eq!(e.mem.u32(this + SKIN_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + SKIN_PARTITION), partition);
        assert_eq!(calls_to(&e, 0x0620_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + ROOT_PARENT), 0x700);
        let bones = e.mem.u32(this + BONES);
        assert_eq!(e.mem.u32(bones), 0x801);
        assert_eq!(e.mem.u32(bones + 4), 0x802);
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![8]]);
    }

    #[test]
    fn post_link_object_moves_the_partition_for_old_files() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(SKIN_PARTITION_POINTER_ASSIGN, nothing);
        e.register(0x0620_0004, nothing);
        let partition = counted(&mut e, 1);
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_PARTITION, partition);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6740, &args![this, stream]);
        assert!(calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN).is_empty());
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0064);
        e.call(0x00a8_6740, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN),
            vec![vec![this + SKIN_PARTITION, partition]]
        );
        assert_eq!(e.mem.u32(data + DATA_PARTITION), 0);
        assert_eq!(calls_to(&e, 0x0620_0004), vec![vec![partition]]);
    }

    #[test]
    fn skinned_bound_merges_every_bone() {
        let mut e = Engine::new();
        e.register(NI_BOUND_UPDATE, |e, a| {
            e.mem.set_f32(a[0], 100.0);
            Ret::default()
        });
        e.register(NI_BOUND_MERGE, nothing);
        e.register(TRANSFORM_INVERT, nothing);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            a[1].into_ret()
        });
        e.register(NI_BOUND_TRANSFORM, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        let bone_data = e.mem.alloc(0x4c * 3);
        // Bone 1's bound: centre (1, 2, 3), radius 4.
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(bone_data + 0x4c + 0x34 + 4 * i as u32, *v);
        }
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_BONE_DATA, bone_data);
        e.mem.set_u32(data + DATA_BONES, 3);
        let bone = |e: &mut Engine, scale: f32| {
            let b = e.mem.alloc(0x9c);
            e.mem.set_f32(b + WORLD_SCALE, scale);
            for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
                e.mem.set_f32(b + 0x8c + 4 * i as u32, *v);
            }
            b
        };
        let bones = e.mem.alloc(12);
        let b0 = bone(&mut e, 1.0);
        let b1 = bone(&mut e, 1.0);
        let b2 = bone(&mut e, 2.0);
        for (i, b) in [b0, b1, b2].iter().enumerate() {
            e.mem.set_u32(bones + 4 * i as u32, *b);
        }
        let root = e.mem.alloc(0x9c);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + BONES, bones);
        e.mem.set_u32(this + ROOT_PARENT, root);
        let bound = e.mem.alloc(0x10);
        let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = merged.clone();
        e.register_double(NI_BOUND_MERGE, move |e, a| {
            record.borrow_mut().push(e.mem.bytes(a[1], 16));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_6530, &args![this, bound]);
        let updates = calls_to(&e, NI_BOUND_UPDATE);
        assert_eq!(updates[0], vec![bound, bone_data + 0x34, b0 + 0x68]);
        assert_eq!(updates[1][1..], [bone_data + 0x4c * 2 + 0x34, b2 + 0x68]);
        let merged = merged.borrow();
        assert_eq!(merged.len(), 2);
        let floats: Vec<f32> = merged[0]
            .chunks(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(floats, vec![11.0, 22.0, 33.0, 4.0]);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT)[0][0], root + WORLD_TRANSFORM);
        assert_eq!(
            calls_to(&e, TRANSFORM_MULTIPLY)[0][0],
            data + DATA_ROOT_TO_SKIN
        );
        assert_eq!(calls_to(&e, NI_BOUND_TRANSFORM)[0][0], bound);
    }

    #[test]
    fn copy_members_shares_data_and_partition() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(0x0620_0004, nothing);
        let data = counted(&mut e, 1);
        let partition = counted(&mut e, 1);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + SKIN_PARTITION, partition);
        let clone = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6660, &args![this, clone, 0x99u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.u32(clone + SKIN_DATA), data);
        assert_eq!(e.mem.u32(clone + SKIN_PARTITION), partition);
        assert_eq!(e.mem.u32(data + 4), 2);
        e.call(0x00a8_6660, &args![this, clone, 0x99u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    /// A clone map double: `key` maps to `key + 0x10000` for keys in `known`.
    fn clone_map(e: &mut Engine, known: Vec<u32>) {
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            if known.contains(&a[1]) {
                e.mem.set_u32(a[2], a[1] + 0x10000);
                true.into_ret()
            } else {
                false.into_ret()
            }
        });
    }

    fn instance(e: &mut Engine) -> (u32, u32) {
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_BONES, 2);
        let bones = e.mem.alloc(8);
        e.mem.set_u32(bones, 0x801);
        e.mem.set_u32(bones + 4, 0x802);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + BONES, bones);
        e.mem.set_u32(this + ROOT_PARENT, 0x700);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x5555);
        (this, cloning)
    }

    #[test]
    fn cloned_when_the_root_or_a_bone_is() {
        let mut e = Engine::new();
        let (this, cloning) = instance(&mut e);
        clone_map(&mut e, vec![]);
        assert!(!e.call(0x00a8_62d0, &args![this, cloning]).bool());
        clone_map(&mut e, vec![0x802]);
        assert!(e.call(0x00a8_62d0, &args![this, cloning]).bool());
        clone_map(&mut e, vec![0x700]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a8_62d0, &args![this, cloning]).bool());
        assert_eq!(calls_to(&e, CLONE_MAP_GET_AT).len(), 1);
        assert_eq!(calls_to(&e, CLONE_MAP_GET_AT)[0][..2], [0x5555, 0x700]);
    }

    #[test]
    fn create_clone_maps_root_and_bones() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        let (this, cloning) = instance(&mut e);
        clone_map(&mut e, vec![]);
        assert_eq!(e.call(0x00a8_6830, &args![this, cloning]).u32(), this);
        clone_map(&mut e, vec![0x802]);
        let clone = e.call(0x00a8_6830, &args![this, cloning]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(clone + ROOT_PARENT), 0x700);
        let bones = e.mem.u32(clone + BONES);
        assert_eq!(e.mem.u32(bones), 0x801);
        assert_eq!(e.mem.u32(bones + 4), 0x10802);
    }
}
