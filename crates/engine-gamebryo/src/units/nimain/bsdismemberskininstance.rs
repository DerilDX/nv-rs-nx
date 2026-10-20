//! `NiMain/bsdismemberskininstance.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a86380`: the jump target of `00a6c670`.
const JUMP_TARGET_00A6C670: u32 = 0x00a8_6380;

// Translated from 00a6c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::LinkObject` (Xbox PDB): a 5-byte jump to `00a86380` (engine map: `NiSkinInstance::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSDismemberSkinInstance@@UAAXAAVNiStream@@@Z`.
pub fn bs_dismember_skin_instance_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A6C670, &args![this, stream]);
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a5d3a0`: the `NiObject` constructor (engine map; `thiscall`, no stack word).
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `BSDismemberSkinInstance` virtual table (`01069a84`).
const DISMEMBER_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9a84;
/// The size of a `BSDismemberSkinInstance` (0x40 bytes).
const DISMEMBER_SKIN_INSTANCE_SIZE: u32 = 0x40;
/// `NiSkinInstance`'s member copy (`00a86660`; `thiscall` on the source: clone, cloning
/// process).
const SKIN_INSTANCE_COPY_MEMBERS: u32 = 0x00a8_6660;
/// `NiSkinInstance::LoadBinary` (`00a86340`; `thiscall`: stream).
const SKIN_INSTANCE_LOAD_BINARY: u32 = 0x00a8_6340;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map at the cloning
/// process's `+0`: key, address of the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `00aa1070`: the `cdecl` allocate (size).
const ALLOCATE: u32 = 0x00aa_1070;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`; `thiscall`: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

layout! {
    /// `BSDismemberSkinInstance` (Xbox PDB), 0x40 bytes: `NiSkinInstance` (0x34) and the
    /// partition array. Offsets confirmed by the PC code of this unit.
    pub struct BSDismemberSkinInstance: 0x40 {
        /// `m_spSkinData` (Xbox PDB, `NiSkinInstance`): `NiSkinData*`; its word at `+0x44`
        /// is the bone count the clone's bone array is sized by.
        0x08 m_spSkinData: u32,
        /// `m_spSkinPartition` (Xbox PDB, `NiSkinInstance`).
        0x0C m_spSkinPartition: u32,
        /// `m_pkRootParent` (Xbox PDB, `NiSkinInstance`).
        0x10 m_pkRootParent: u32,
        /// `m_ppkBones` (Xbox PDB, `NiSkinInstance`).
        0x14 m_ppkBones: u32,
        /// `m_uiFrameID` (Xbox PDB, `NiSkinInstance`).
        0x18 m_uiFrameID: u32,
        /// `uiPartitionDataArraySize` (Xbox PDB).
        0x34 uiPartitionDataArraySize: u32,
        /// `pPartitionDataArray` (Xbox PDB): 4-byte `PartitionData` items (two bytes, then
        /// a 16-bit word).
        0x38 pPartitionDataArray: u32,
        /// `bVisiblePartitions` (Xbox PDB).
        0x3C bVisiblePartitions: u8,
    }
}

/// The bytes of `count` 4-byte items as the code asks the memory manager for them: the
/// `MUL`/`SETO`/`NEG`/`OR` size computation gives all ones when the product overflows.
fn partition_bytes(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

/// The fields the `BSDismemberSkinInstance` constructor inlined here sets: after the
/// `NiObject` constructor, the words `+0x08`..`+0x14`, `+0x1C`..`+0x28`, `+0x30`..`+0x38`
/// zero, the frame id `-1` and the class's virtual table (the word at `+0x2C` is not
/// written).
fn construct_in_place(e: &mut Engine, object: u32) {
    e.call(OBJECT_CONSTRUCTOR, &args![object]);
    for offset in [0x08, 0x0c, 0x10, 0x14] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object + 0x18, 0xffff_ffff);
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x30] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object, DISMEMBER_SKIN_INSTANCE_VTABLE);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object + 0x38, 0);
}

/// The cloning process's lookup of `key` (`00a53c30`) into the word at `result`.
fn clone_lookup(e: &mut Engine, cloning: u32, key: u32, result: Ptr) -> bool {
    let map = e.mem.u32(cloning);
    e.call(CLONE_MAP_LOOKUP, &args![map, key, result]).bool()
}

// Translated from 00a6c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process). Runs `NiSkinInstance`'s copy (`00a86660`),
/// copies the partition count and, for a count above zero, a new array from the memory
/// manager (`00aa3e40`, count * 4 bytes, all ones on overflow) filled item by item (two
/// bytes, then the 16-bit word; the count read again for each item); with no partitions the
/// clone's array is null. Last the visible-partitions byte.
pub fn bs_dismember_skin_instance_copy_members(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    clone: Ptr<BSDismemberSkinInstance>,
    cloning: Ptr,
) {
    e.call(SKIN_INSTANCE_COPY_MEMBERS, &args![this, clone, cloning]);
    let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
    e.set(
        clone,
        BSDismemberSkinInstance::uiPartitionDataArraySize,
        count,
    );
    if e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) == 0 {
        e.set(clone, BSDismemberSkinInstance::pPartitionDataArray, 0);
    } else {
        let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
        let array = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER, partition_bytes(count)],
            )
            .u32();
        e.set(clone, BSDismemberSkinInstance::pPartitionDataArray, array);
        let mut index = 0u32;
        while index < e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) {
            let offset = index.wrapping_mul(4);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u8(source.wrapping_add(offset));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem.set_u8(target.wrapping_add(offset), value);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u8(source.wrapping_add(offset).wrapping_add(1));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem
                .set_u8(target.wrapping_add(offset).wrapping_add(1), value);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u16(source.wrapping_add(offset).wrapping_add(2));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem
                .set_u16(target.wrapping_add(offset).wrapping_add(2), value);
            index = index.wrapping_add(1);
        }
    }
    let visible = e.get(this, BSDismemberSkinInstance::bVisiblePartitions);
    e.set(clone, BSDismemberSkinInstance::bVisiblePartitions, visible);
}

// Translated from 00a6c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning
/// process). Allocates and constructs a 0x40-byte instance (null when the allocation
/// fails), copies the members into it ([`bs_dismember_skin_instance_copy_members`]), then
/// maps the root parent and every bone through the cloning process (`00a53c30`): the clone
/// found, else the original pointer. The bone array (`00aa1070`) holds as many pointers as
/// the skin data's word at `+0x44`; the lookup's result word is not reset between bones.
pub fn bs_dismember_skin_instance_create_clone(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    cloning: Ptr,
) -> Ptr<BSDismemberSkinInstance> {
    let block = e
        .call(OPERATOR_NEW, &args![DISMEMBER_SKIN_INSTANCE_SIZE])
        .u32();
    if block != 0 {
        construct_in_place(e, block);
    }
    let clone = Ptr::<BSDismemberSkinInstance>::new(block);
    bs_dismember_skin_instance_copy_members(e, this, clone, cloning);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let root = e.get(this, BSDismemberSkinInstance::m_pkRootParent);
        let root = if clone_lookup(e, cloning.addr(), root, found) {
            e.mem.u32(found.addr())
        } else {
            e.get(this, BSDismemberSkinInstance::m_pkRootParent)
        };
        e.set(clone, BSDismemberSkinInstance::m_pkRootParent, root);
        let skin_data = e.get(this, BSDismemberSkinInstance::m_spSkinData);
        let bones = e.mem.u32(skin_data.wrapping_add(0x44));
        let array = e.call(ALLOCATE, &args![bones.wrapping_mul(4)]).u32();
        e.set(clone, BSDismemberSkinInstance::m_ppkBones, array);
        let mut index = 0u32;
        while index < bones {
            let source = e.get(this, BSDismemberSkinInstance::m_ppkBones);
            let bone = e.mem.u32(source.wrapping_add(index.wrapping_mul(4)));
            let bone = if clone_lookup(e, cloning.addr(), bone, found) {
                e.mem.u32(found.addr())
            } else {
                let source = e.get(this, BSDismemberSkinInstance::m_ppkBones);
                e.mem.u32(source.wrapping_add(index.wrapping_mul(4)))
            };
            let target = e.get(clone, BSDismemberSkinInstance::m_ppkBones);
            e.mem
                .set_u32(target.wrapping_add(index.wrapping_mul(4)), bone);
            index = index.wrapping_add(1);
        }
    });
    clone
}

// Translated from 00a6c8a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's create function (not a Ghidra function; the bytes at `00a6c8a0`, plain
/// `RET`): allocates 0x40 bytes and, when that succeeds, constructs a
/// `BSDismemberSkinInstance` in them (the `NiObject` constructor `00a5d3a0`, the cleared
/// fields, frame id `-1`, the virtual table `01069a84`; the word at `+0x2C` is not written).
/// Answers the object, null when the allocation failed.
pub fn fn_00a6c8a0(e: &mut Engine) -> Ptr<BSDismemberSkinInstance> {
    let block = e
        .call(OPERATOR_NEW, &args![DISMEMBER_SKIN_INSTANCE_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_in_place(e, block);
    Ptr::new(block)
}

// Translated from 00a6c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream). Runs
/// `NiSkinInstance::LoadBinary` (`00a86340`), reads the partition count (one 4-byte word)
/// and the partition items (count * 4 bytes, component sizes 1, 1, 2) through the stream's
/// input object (`NiStream` `+0x24C`; read function at its `+8`, `cdecl` (input,
/// destination, bytes, sizes, size count); the answer added to its word at `+4`), the array
/// from the memory manager (`00aa3e40`, all ones on overflow). The visible-partitions byte
/// becomes the OR of every item's first byte.
pub fn bs_dismember_skin_instance_load_binary(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    stream: Ptr,
) {
    e.call(SKIN_INSTANCE_LOAD_BINARY, &args![this, stream]);
    let count_field = this.addr() + 0x34;
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, count_field, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
    let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
    let array = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, partition_bytes(count)],
        )
        .u32();
    let bytes = e
        .get(this, BSDismemberSkinInstance::uiPartitionDataArraySize)
        .wrapping_mul(4);
    e.set(this, BSDismemberSkinInstance::pPartitionDataArray, array);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(12, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 1);
        e.mem.set_u32(sizes.addr() + 4, 1);
        e.mem.set_u32(sizes.addr() + 8, 2);
        e.call(read, &args![input, array, bytes, sizes, 3u32]).u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
    e.set(this, BSDismemberSkinInstance::bVisiblePartitions, 0);
    let mut item = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
    let mut index = 0u32;
    while index < e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) {
        let visible = e.get(this, BSDismemberSkinInstance::bVisiblePartitions) | e.mem.u8(item);
        e.set(this, BSDismemberSkinInstance::bVisiblePartitions, visible);
        index = index.wrapping_add(1);
        item = item.wrapping_add(4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6_c670,
            bs_dismember_skin_instance_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a6_c720,
            bs_dismember_skin_instance_copy_members(
                Ptr<BSDismemberSkinInstance>,
                Ptr<BSDismemberSkinInstance>,
                Ptr,
            )
        ),
        entry!(
            0x00a6_c7c0,
            bs_dismember_skin_instance_create_clone(
                Ptr<BSDismemberSkinInstance>,
                Ptr,
            ) -> Ptr<BSDismemberSkinInstance>
        ),
        entry!(0x00a6_c8a0, fn_00a6c8a0() -> Ptr<BSDismemberSkinInstance>),
        entry!(
            0x00a6_c900,
            bs_dismember_skin_instance_load_binary(Ptr<BSDismemberSkinInstance>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a6c670_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_c670;
        const TARGET: u32 = 0x00a8_6380;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SKIN_INSTANCE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(SKIN_INSTANCE_LOAD_BINARY, |_, _| Ret::default());
        e.register(OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn copy_members_copies_every_partition_item() {
        let mut e = engine();
        let source = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x1234_0201);
        e.mem.set_u32(items + 4, 0x5678_0403);
        e.mem.set_u32(source + 0x34, 2);
        e.mem.set_u32(source + 0x38, items);
        e.mem.set_u8(source + 0x3c, 1);
        e.call(0x00a6_c720, &args![source, target, 0x9000u32]);
        assert_eq!(
            calls(&e, SKIN_INSTANCE_COPY_MEMBERS),
            vec![vec![source, target, 0x9000]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 8]]
        );
        let copy = e.mem.u32(target + 0x38);
        assert_ne!(copy, items);
        assert_eq!(e.mem.u32(copy), 0x1234_0201);
        assert_eq!(e.mem.u32(copy + 4), 0x5678_0403);
        assert_eq!(e.mem.u32(target + 0x34), 2);
        assert_eq!(e.mem.u8(target + 0x3c), 1);
    }

    #[test]
    fn copy_members_without_partitions_clears_the_array() {
        let mut e = engine();
        let source = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x38, 0x4444);
        e.call(0x00a6_c720, &args![source, target, 0x9000u32]);
        assert_eq!(e.mem.u32(target + 0x38), 0);
        assert!(calls(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(partition_bytes(0x4000_0000), u32::MAX);
    }

    #[test]
    fn create_clone_maps_the_root_and_the_bones() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x2c, 0x2c2c);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        // The lookup finds the root (0x100 -> 0x111) and bone 0x200, not bone 0x300.
        e.register(CLONE_MAP_LOOKUP, |e, a| {
            let found = match a[1] {
                0x100 => 0x111,
                0x200 => 0x222,
                _ => return Ret::default(),
            };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let source = e.mem.alloc(0x40);
        let skin_data = e.mem.alloc(0x48);
        e.mem.set_u32(skin_data + 0x44, 2);
        let bones = e.mem.alloc(8);
        e.mem.set_u32(bones, 0x200);
        e.mem.set_u32(bones + 4, 0x300);
        e.mem.set_u32(source + 0x08, skin_data);
        e.mem.set_u32(source + 0x10, 0x100);
        e.mem.set_u32(source + 0x14, bones);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x7777);
        let clone = e.call(0x00a6_c7c0, &args![source, cloning]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(block + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x2c), 0x2c2c);
        assert_eq!(e.mem.u32(block + 0x10), 0x111);
        let cloned = e.mem.u32(block + 0x14);
        assert_eq!(e.mem.u32(cloned), 0x222);
        assert_eq!(e.mem.u32(cloned + 4), 0x300);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        let lookups = calls(&e, CLONE_MAP_LOOKUP);
        assert_eq!(lookups.len(), 3);
        assert!(lookups.iter().all(|c| c[0] == 0x7777));
        assert_eq!(
            calls(&e, SKIN_INSTANCE_COPY_MEMBERS),
            vec![vec![source, block, cloning]]
        );
    }

    #[test]
    fn create_function_constructs_or_answers_null() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x2c, 0x2c2c);
        e.mem.set_u32(block + 0x08, 0x0808);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x40);
            block.into_ret()
        });
        assert_eq!(e.call(0x00a6_c8a0, &args![]).u32(), block);
        assert_eq!(calls(&e, OBJECT_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(block + 0x08), 0);
        assert_eq!(e.mem.u32(block + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x2c), 0x2c2c);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a6_c8a0, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_partitions_and_ors_the_visible_bytes() {
        let mut e = engine();
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            if a[4] == 1 {
                assert_eq!(e.mem.u32(a[3]), 4);
                e.mem.set_u32(a[1], 3);
            } else {
                assert_eq!(
                    (e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8)),
                    (1, 1, 2)
                );
                for (index, first) in [2u8, 0, 4].iter().enumerate() {
                    e.mem.set_u8(a[1] + 4 * index as u32, *first);
                }
            }
            a[2].into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 4, 0x10);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x40);
        e.call(0x00a6_c900, &args![this, stream]);
        assert_eq!(
            calls(&e, SKIN_INSTANCE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 12]]
        );
        assert_eq!(e.mem.u8(this + 0x3c), 6);
        assert_eq!(e.mem.u32(input + 4), 0x10 + 4 + 12);
        let reads = calls(&e, READ);
        assert_eq!(
            (reads[0][1], reads[0][2], reads[1][1], reads[1][2]),
            (this + 0x34, 4, e.mem.u32(this + 0x38), 12)
        );
    }
}
