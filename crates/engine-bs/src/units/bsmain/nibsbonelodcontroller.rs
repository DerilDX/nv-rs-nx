//! `BSMain/nibsbonelodcontroller.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTPrimitiveSet;

/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;

// Translated from 00a311e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiNode*,NiTMallocInterface<NiNode*>>::Realloc` (engine map; `thiscall`,
/// one stack word): changes the capacity of the set of pointers (layout
/// [`NiTPrimitiveSet`]: data, capacity, used count) to `new_capacity`. Does
/// nothing for the current capacity. Otherwise new storage of `new_capacity`
/// pointers (`00aa1070`; none for 0) takes the used elements, the old storage is
/// freed (`00aa10f0`, also when it is null) and the capacity is stored.
pub fn ni_t_set_realloc(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, new_capacity: u32) {
    if new_capacity == e.get(this, NiTPrimitiveSet::m_uiAlloced) {
        return;
    }
    let storage = if new_capacity == 0 {
        0
    } else {
        let storage = e.call(ALLOCATE, &args![new_capacity.wrapping_mul(4)]).u32();
        let old = e.get(this, NiTPrimitiveSet::m_pBase);
        let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
        for index in 0..used {
            let value = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(storage.wrapping_add(index.wrapping_mul(4)), value);
        }
        storage
    };
    let old = e.get(this, NiTPrimitiveSet::m_pBase);
    e.call(FREE, &args![old]);
    e.set(this, NiTPrimitiveSet::m_pBase, storage);
    e.set(this, NiTPrimitiveSet::m_uiAlloced, new_capacity);
}

// Translated from 00c53e00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031b8 without reading the receiver. The map names this body `NiBSBoneLODController::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c53e00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31b8
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00aa1460`: the sized delete (`cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free (`00aa4060`),
/// both `thiscall` with one stack word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `NiTimeController`'s constructor (`00a6d220`), destructor (`00a6d290`), member copy
/// (`00a6cfe0`; `thiscall` on the source: clone, cloning process), `ProcessClone`
/// (`00a6d0c0`), `LoadBinary` (`00a6d5e0`) and `LinkObject` (`00a6d500`), the last three
/// `thiscall` with one stack word.
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCTOR: u32 = 0x00a6_d290;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
/// The base `PostLinkObject` (`00a89af0`; `thiscall`: stream).
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map at the cloning
/// process's `+0`: key, address of the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiTArray`'s `SetSize` (`00a90180`; `thiscall`: new size) and `SetAt` (`00ba3aa0`;
/// `thiscall`: index, address of the element).
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// Stream helpers: the `cdecl` read of one 32-bit word (`00a26df0`: stream,
/// destination), the link id read (`00a645b0`; `thiscall` on the stream) and the linked
/// object of the next id (`00a63430`; `thiscall` on the stream).
const READ_WORD: u32 = 0x00a2_6df0;
const READ_LINK_ID: u32 = 0x00a6_45b0;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The `NiBSBoneLODController` virtual table (`010c29cc`) and the bone array's virtual
/// tables: the one its constructor stores (`0109a2a4`) and the derived one the controller's
/// constructor stores over it (`0109a2c0`).
const BONE_LOD_CONTROLLER_VTABLE: u32 = 0x010c_29cc;
const ARRAY_BASE_VTABLE: u32 = 0x0109_a2a4;
const BONE_ARRAY_VTABLE: u32 = 0x0109_a2c0;
/// The size of an `NiBSBoneLODController` (0x4C bytes).
const BONE_LOD_CONTROLLER_SIZE: u32 = 0x4c;

layout! {
    /// `NiBSBoneLODController` (Xbox PDB), 0x4C bytes: `NiTimeController`, the LOD and the
    /// bone array (an `NiTPrimitiveArray` of node sets at `+0x3C`).
    pub struct NiBSBoneLODController: 0x4C {
        /// `m_pkTarget` (Xbox PDB, `NiTimeController`).
        0x2C m_pkTarget: u32,
        /// `m_iLOD` (Xbox PDB).
        0x34 m_iLOD: u32,
        /// `m_uiNumLODs` (Xbox PDB).
        0x38 m_uiNumLODs: u32,
        /// `m_kBoneArray.m_pBase` (Xbox PDB): node set pointers.
        0x40 m_pBase: u32,
        /// `m_kBoneArray.m_usMaxSize` (Xbox PDB).
        0x44 m_usMaxSize: u16,
        /// `m_kBoneArray.m_usSize` (Xbox PDB).
        0x46 m_usSize: u16,
        /// `m_kBoneArray.m_usESize` (Xbox PDB).
        0x48 m_usESize: u16,
        /// `m_kBoneArray.m_usGrowBy` (Xbox PDB).
        0x4A m_usGrowBy: u16,
    }
}

/// The node set at index `index` of the bone array.
fn bone_set(e: &Engine, this: Ptr<NiBSBoneLODController>, index: u32) -> u32 {
    let base = e.get(this, NiBSBoneLODController::m_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// A new empty node set: 0xC bytes from the memory manager with the inlined
/// `NiTPrimitiveSet` constructor for an initial size of 0 (its size test, `XOR`/`JBE`,
/// always skips the allocation of the storage, which is kept as written); null when the
/// allocation failed.
fn new_bone_set(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if block == 0 {
        return 0;
    }
    let initial_size = 0u32;
    let storage = if initial_size > 0 {
        e.call(ALLOCATE, &args![initial_size << 2]).u32()
    } else {
        0
    };
    e.mem.set_u32(block, storage);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    block
}

/// `m_kBoneArray.SetAtGrow(index, set)` as the exe inlines it on the array at `array`:
/// `SetSize(grow_by + index)` (`00a90180`) when the index is not below the capacity, then
/// `SetAt` (`00ba3aa0`) with the address of a word holding `set`.
fn bone_array_set_at_grow(e: &mut Engine, array: u32, index: u32, set: u32) {
    if index >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(ARRAY_SET_SIZE, &args![array, grow_by.wrapping_add(index)]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), set);
        e.call(ARRAY_SET_AT, &args![array, index, slot]);
    });
}

/// [`ni_t_set_add`] of the word `value` (passed by address, as the exe does).
fn add_to_set(e: &mut Engine, set: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        ni_t_set_add(e, Ptr::new(set), slot);
    });
}

// Translated from 00a54190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node set array's constructor (`NiTPrimitiveArray<NiTPrimitiveSet<NiNode*>*>`,
/// unnamed in the engine map; `thiscall`, `RET 8`: capacity, grow-by, both 16-bit): the
/// array's virtual table (`0109a2a4`), the counts, and storage for `capacity` pointers
/// (`00aa1070`) or none. Answers `this`.
pub fn fn_00a54190(e: &mut Engine, this: Ptr, capacity: u16, grow_by: u16) -> Ptr {
    let array = this.addr();
    e.mem.set_u16(array + 0xe, grow_by);
    e.mem.set_u32(array, ARRAY_BASE_VTABLE);
    e.mem.set_u16(array + 8, capacity);
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    let storage = if capacity != 0 {
        e.call(ALLOCATE, &args![u32::from(capacity) * 4]).u32()
    } else {
        0
    };
    e.mem.set_u32(array + 4, storage);
    this
}

// Translated from 00a543b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiNode*,NiTMallocInterface<NiNode*>>::Add` (engine map; `thiscall`, `RET 4`:
/// address of the element): when the set is full its capacity doubles (1 for an empty
/// one) through [`ni_t_set_realloc`]; then the element is appended.
pub fn ni_t_set_add(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, element: Ptr) {
    let capacity = e.get(this, NiTPrimitiveSet::m_uiAlloced);
    if e.get(this, NiTPrimitiveSet::m_uiUsed) == capacity {
        let grown = if capacity == 0 {
            1
        } else {
            capacity.wrapping_add(capacity)
        };
        ni_t_set_realloc(e, this, grown);
    }
    let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
    let base = e.get(this, NiTPrimitiveSet::m_pBase);
    let value = e.mem.u32(element.addr());
    e.mem
        .set_u32(base.wrapping_add(used.wrapping_mul(4)), value);
    e.set(this, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
}

// Translated from 00c52880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): deletes every node set of the
/// bone array (its storage freed by `00aa10f0`, the set by the memory manager; the
/// compiler's `MOV EAX,1; AND EAX,1` test before the second free always passes), then
/// clears every slot below the array's size (16-bit counter) and both the size and the
/// count of set slots.
pub fn fn_00c52880(e: &mut Engine, this: Ptr<NiBSBoneLODController>) {
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            let storage = e.mem.u32(set);
            e.call(FREE, &args![storage]);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, set]);
        }
        index = index.wrapping_add(1);
    }
    let mut slot = 0u16;
    while u32::from(slot) < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let base = e.get(this, NiBSBoneLODController::m_pBase);
        e.mem.set_u32(base.wrapping_add(u32::from(slot) * 4), 0);
        slot = slot.wrapping_add(1);
    }
    e.set(this, NiBSBoneLODController::m_usSize, 0);
    e.set(this, NiBSBoneLODController::m_usESize, 0);
}

// Translated from 00c52960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::SetBoneLOD` (Xbox PDB; `thiscall`, `RET 4`). Refused (false)
/// without LODs, for the current LOD, and for an LOD other than -1 that is negative or not
/// below the LOD count. Otherwise walks the node sets in order; from the set at the
/// requested index on, every node gets flag 2 of its flags word (`NiAVObject` `+0x30`)
/// set, the nodes of the sets before it get it cleared (all cleared for -1). Stores the
/// LOD and answers true.
pub fn ni_bs_bone_lod_controller_set_bone_lod(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    lod: i32,
) -> bool {
    if e.get(this, NiBSBoneLODController::m_uiNumLODs) == 0 {
        return false;
    }
    if e.get(this, NiBSBoneLODController::m_iLOD) == lod as u32 {
        return false;
    }
    if lod != -1 && (lod <= -1 || lod as u32 >= e.get(this, NiBSBoneLODController::m_uiNumLODs)) {
        return false;
    }
    let mut visible = false;
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            if index == lod as u32 {
                visible = true;
            }
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let node = e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4)));
                if node != 0 {
                    // NiAVObject::m_uFlags (Xbox PDB) +0x30.
                    let flags = e.mem.u32(node + 0x30);
                    let flags = if visible { flags | 2 } else { flags & !2 };
                    e.mem.set_u32(node + 0x30, flags);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    e.set(this, NiBSBoneLODController::m_iLOD, lod as u32);
    true
}

// Translated from 00c53d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53d70 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController`'s constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): `NiTimeController`'s constructor (`00a6d220`), the class's virtual table, LOD -1,
/// no LODs, and the bone array constructed empty with grow-by 1 ([`fn_00a54190`]), its
/// virtual table then replaced by `0109a2c0`. Answers `this`.
pub fn fn_00c53d70(e: &mut Engine, this: Ptr<NiBSBoneLODController>) -> Ptr<NiBSBoneLODController> {
    e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BONE_LOD_CONTROLLER_VTABLE);
    e.set(this, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    e.set(this, NiBSBoneLODController::m_uiNumLODs, 0);
    let array = this.addr() + 0x3c;
    fn_00a54190(e, Ptr::new(array), 0, 1);
    e.mem.set_u32(array, BONE_ARRAY_VTABLE);
    this
}

// Translated from 00c537f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c537f0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// The class's create function (unnamed in the engine map; plain `RET`): a new 0x4C-byte
/// controller ([`fn_00c53d70`]), null when the allocation fails.
pub fn fn_00c537f0(e: &mut Engine) -> Ptr<NiBSBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![BONE_LOD_CONTROLLER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c53d70(e, Ptr::new(block))
}

// Translated from 00c533f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process): `NiTimeController`'s copy (`00a6cfe0`), then
/// the LOD and the LOD count. The bone sets are rebuilt by `ProcessClone`.
pub fn fn_00c533f0(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    clone: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) {
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, cloning]);
    let lod = e.get(this, NiBSBoneLODController::m_iLOD);
    e.set(clone, NiBSBoneLODController::m_iLOD, lod);
    let count = e.get(this, NiBSBoneLODController::m_uiNumLODs);
    e.set(clone, NiBSBoneLODController::m_uiNumLODs, count);
}

// Translated from 00c53360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53360 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process): a
/// new controller ([`fn_00c53d70`]; null when the allocation fails) filled by
/// [`fn_00c533f0`]. Answers the clone.
pub fn ni_bs_bone_lod_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) -> Ptr<NiBSBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![BONE_LOD_CONTROLLER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c53d70(e, Ptr::new(block))
    };
    fn_00c533f0(e, this, clone, cloning);
    clone
}

// Translated from 00c53430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53430 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::ProcessClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process).
/// `NiTimeController::ProcessClone` (`00a6d0c0`), then the clone looked up in the cloning
/// process's map (`00a53c30`; null when absent). For each slot of the bone array the clone
/// gets a null set (for a null slot) or a new set holding, node by node, the node's clone
/// (null for a null node or one not found), stored with the array's grow-and-set. Last the
/// clone's LOD is applied again: its LOD word set to -1 and `SetBoneLOD` called with the old
/// value.
pub fn ni_bs_bone_lod_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let clone = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        e.mem.u32(slot.addr())
    });
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        let array = clone.wrapping_add(0x3c);
        if set == 0 {
            bone_array_set_at_grow(e, array, index, 0);
        } else {
            let copy = new_bone_set(e);
            bone_array_set_at_grow(e, array, index, copy);
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let node = e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4)));
                if node == 0 {
                    add_to_set(e, copy, 0);
                } else {
                    let found = e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), 0);
                        let map = e.mem.u32(cloning.addr());
                        if e.call(CLONE_MAP_LOOKUP, &args![map, node, slot]).bool() {
                            e.mem.u32(slot.addr())
                        } else {
                            0
                        }
                    });
                    add_to_set(e, copy, found);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    let clone = Ptr::<NiBSBoneLODController>::new(clone);
    let lod = e.get(clone, NiBSBoneLODController::m_iLOD);
    e.set(clone, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    ni_bs_bone_lod_controller_set_bone_lod(e, clone, lod as i32);
}

// Translated from 00c53870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53870 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiTimeController::LoadBinary` (`00a6d5e0`), the LOD and LOD count (`00a26df0`), the
/// number of sets, then for each set a new empty set stored with the array's grow-and-set,
/// its node count, and per node the controller itself added as a placeholder (resolved by
/// `LinkObject`) and the node's link id read (`00a645b0`).
pub fn ni_bs_bone_lod_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_WORD, &args![stream, this.addr() + 0x34]);
    e.call(READ_WORD, &args![stream, this.addr() + 0x38]);
    e.with_stack(8, |e, counts| {
        let sets_count = counts.addr();
        let nodes_count = sets_count + 4;
        e.mem.set_u32(sets_count, 0);
        e.call(READ_WORD, &args![stream, sets_count]);
        let mut index = 0u32;
        while index < e.mem.u32(sets_count) {
            let set = new_bone_set(e);
            bone_array_set_at_grow(e, this.addr() + 0x3c, index, set);
            e.mem.set_u32(nodes_count, 0);
            e.call(READ_WORD, &args![stream, nodes_count]);
            let mut node = 0u32;
            while node < e.mem.u32(nodes_count) {
                add_to_set(e, set, this.addr());
                e.call(READ_LINK_ID, &args![stream]);
                node = node.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c53a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::LinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiTimeController::LinkObject` (`00a6d500`), then every node slot of every set is
/// replaced by the next linked object (`00a63430`), the slot index checked against the
/// set's count again after each lookup.
pub fn ni_bs_bone_lod_controller_link_object(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                if item < e.mem.u32(set + 8) {
                    let base = e.mem.u32(set);
                    e.mem
                        .set_u32(base.wrapping_add(item.wrapping_mul(4)), object);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c53ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::PostLinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream): the
/// base `PostLinkObject` (`00a89af0`), then the LOD applied again (its word set to -1,
/// [`ni_bs_bone_lod_controller_set_bone_lod`] with the old value).
pub fn ni_bs_bone_lod_controller_post_link_object(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    let lod = e.get(this, NiBSBoneLODController::m_iLOD);
    e.set(this, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    ni_bs_bone_lod_controller_set_bone_lod(e, this, lod as i32);
}

// Translated from 00c53e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53e40 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController`'s destructor (unnamed in the engine map; `thiscall`): the
/// class's virtual table, every node set deleted ([`fn_00c52880`]), the bone array's
/// destructor inlined (its virtual table `0109a2a4`, its storage freed by `00aa10f0`), then
/// `NiTimeController`'s destructor (`00a6d290`).
pub fn fn_00c53e40(e: &mut Engine, this: Ptr<NiBSBoneLODController>) {
    e.mem.set_u32(this.addr(), BONE_LOD_CONTROLLER_VTABLE);
    fn_00c52880(e, this);
    e.mem.set_u32(this.addr() + 0x3c, ARRAY_BASE_VTABLE);
    let storage = e.get(this, NiBSBoneLODController::m_pBase);
    e.call(FREE, &args![storage]);
    e.call(TIME_CONTROLLER_DESTRUCTOR, &args![this]);
}

// Translated from 00c53e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// the destructor ([`fn_00c53e40`]) and, when bit 0 of `flags` is set, the sized delete of
/// 0x4C bytes. Answers `this`.
pub fn ni_bs_bone_lod_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    flags: u32,
) -> Ptr<NiBSBoneLODController> {
    fn_00c53e40(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, BONE_LOD_CONTROLLER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_3e00, fn_00c53e00(Ptr) -> u32),
        entry!(0x00a3_11e0, ni_t_set_realloc(Ptr<NiTPrimitiveSet>, u32)),
        entry!(0x00a5_4190, fn_00a54190(Ptr, u16, u16) -> Ptr),
        entry!(0x00a5_43b0, ni_t_set_add(Ptr<NiTPrimitiveSet>, Ptr)),
        entry!(0x00c5_2880, fn_00c52880(Ptr<NiBSBoneLODController>)),
        entry!(
            0x00c5_2960,
            ni_bs_bone_lod_controller_set_bone_lod(Ptr<NiBSBoneLODController>, i32) -> bool
        ),
        entry!(
            0x00c5_3d70,
            fn_00c53d70(Ptr<NiBSBoneLODController>) -> Ptr<NiBSBoneLODController>
        ),
        entry!(0x00c5_37f0, fn_00c537f0() -> Ptr<NiBSBoneLODController>),
        entry!(
            0x00c5_33f0,
            fn_00c533f0(Ptr<NiBSBoneLODController>, Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3360,
            ni_bs_bone_lod_controller_create_clone(
                Ptr<NiBSBoneLODController>,
                Ptr,
            ) -> Ptr<NiBSBoneLODController>
        ),
        entry!(
            0x00c5_3430,
            ni_bs_bone_lod_controller_process_clone(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3870,
            ni_bs_bone_lod_controller_load_binary(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3a10,
            ni_bs_bone_lod_controller_link_object(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3ab0,
            ni_bs_bone_lod_controller_post_link_object(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(0x00c5_3e40, fn_00c53e40(Ptr<NiBSBoneLODController>)),
        entry!(
            0x00c5_3e10,
            ni_bs_bone_lod_controller_scalar_deleting_destructor(
                Ptr<NiBSBoneLODController>,
                u32,
            )
                -> Ptr<NiBSBoneLODController>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(capacity: u32, items: &[u32]) -> (Engine, Ptr<NiTPrimitiveSet>, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let set = e.new_object::<NiTPrimitiveSet>();
        let base = e.mem.alloc(capacity * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(set, NiTPrimitiveSet::m_pBase, base);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, capacity);
        e.set(set, NiTPrimitiveSet::m_uiUsed, items.len() as u32);
        (e, set, base)
    }

    #[test]
    fn the_current_capacity_changes_nothing() {
        let (mut e, set, base) = fixture(4, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 4u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), base);
    }

    #[test]
    fn a_new_capacity_copies_the_used_elements_and_frees_the_old_block() {
        let (mut e, set, base) = fixture(2, &[0x11, 0x22]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 8u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![32u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 8);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiUsed), 2);
        let new = e.get(set, NiTPrimitiveSet::m_pBase);
        assert_ne!(new, base);
        assert_eq!([e.mem.u32(new), e.mem.u32(new + 4)], [0x11, 0x22]);
    }

    #[test]
    fn capacity_zero_leaves_a_null_pointer() {
        let (mut e, set, base) = fixture(2, &[0x11]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), 0);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c53e00_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_3e00, &args![0x1000u32]).u32(), 0x0120_31b8);
        assert_eq!(e.call(0x00c5_3e00, &args![0u32]).u32(), 0x0120_31b8);
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
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        for address in [
            TIME_CONTROLLER_CONSTRUCTOR,
            TIME_CONTROLLER_DESTRUCTOR,
            TIME_CONTROLLER_COPY_MEMBERS,
            TIME_CONTROLLER_PROCESS_CLONE,
            TIME_CONTROLLER_LOAD_BINARY,
            TIME_CONTROLLER_LINK_OBJECT,
            BASE_POST_LINK_OBJECT,
            SIZED_DELETE,
            READ_LINK_ID,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        // The array's SetSize grows the storage; SetAt stores the element.
        e.register(ARRAY_SET_SIZE, |e, a| {
            let storage = e.mem.alloc(4 * a[1] + 16);
            let old = e.mem.u32(a[0] + 4);
            for index in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(storage + 4 * index, value);
            }
            e.mem.set_u32(a[0] + 4, storage);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let storage = e.mem.u32(a[0] + 4);
            e.mem.set_u32(storage + 4 * a[1], value);
            if a[1] >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A controller whose bone array holds the given sets of nodes (0 = null set).
    fn controller(e: &mut Engine, sets: &[&[u32]], lods: u32) -> u32 {
        let this = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        let base = e.mem.alloc(4 * sets.len() as u32 + 4);
        for (index, nodes) in sets.iter().enumerate() {
            if nodes.is_empty() {
                continue;
            }
            let set = e.mem.alloc(0xc);
            let storage = e.mem.alloc(4 * nodes.len() as u32);
            for (slot, node) in nodes.iter().enumerate() {
                e.mem.set_u32(storage + 4 * slot as u32, *node);
            }
            e.mem.set_u32(set, storage);
            e.mem.set_u32(set + 4, nodes.len() as u32);
            e.mem.set_u32(set + 8, nodes.len() as u32);
            e.mem.set_u32(base + 4 * index as u32, set);
        }
        e.mem.set_u32(this + 0x40, base);
        e.mem.set_u16(this + 0x44, sets.len() as u16);
        e.mem.set_u16(this + 0x46, sets.len() as u16);
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        e.mem.set_u32(this + 0x38, lods);
        this
    }

    fn node(e: &mut Engine, flags: u32) -> u32 {
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x30, flags);
        node
    }

    #[test]
    fn set_bone_lod_flags_the_sets_from_the_lod_on() {
        let mut e = engine();
        let nodes: Vec<u32> = (0..3).map(|_| node(&mut e, 0x10)).collect();
        let this = controller(&mut e, &[&[nodes[0]], &[], &[nodes[1], 0, nodes[2]]], 3);
        let set_lod =
            |e: &mut Engine, lod: i32| e.call(0x00c5_2960, &args![this, lod as u32]).bool();
        assert!(set_lod(&mut e, 2));
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert_eq!(e.mem.u32(nodes[0] + 0x30), 0x10);
        assert_eq!(e.mem.u32(nodes[1] + 0x30), 0x12);
        assert_eq!(e.mem.u32(nodes[2] + 0x30), 0x12);
        // The same LOD, one out of range and a negative one other than -1 are refused.
        assert!(!set_lod(&mut e, 2));
        assert!(!set_lod(&mut e, 3));
        assert!(!set_lod(&mut e, -2));
        assert!(set_lod(&mut e, 0));
        assert_eq!(e.mem.u32(nodes[0] + 0x30), 0x12);
        assert!(set_lod(&mut e, -1));
        assert!(nodes.iter().all(|n| e.mem.u32(n + 0x30) == 0x10));
        e.mem.set_u32(this + 0x38, 0);
        assert!(!set_lod(&mut e, 1));
    }

    #[test]
    fn deleting_the_sets_frees_them_and_clears_the_array() {
        let mut e = engine();
        let this = controller(&mut e, &[&[0x100], &[], &[0x200]], 1);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(base);
        let last = e.mem.u32(base + 8);
        e.mem.set_u16(this + 0x48, 2);
        e.call(0x00c5_2880, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, first], vec![MEMORY_MANAGER, last]]
        );
        assert_eq!(calls(&e, FREE).len(), 2);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 8)), (0, 0));
        assert_eq!((e.mem.u16(this + 0x46), e.mem.u16(this + 0x48)), (0, 0));
    }

    #[test]
    fn set_add_doubles_a_full_set() {
        let mut e = engine();
        let set = e.mem.alloc(0xc);
        let value = e.mem.alloc(4);
        for item in 1..=3u32 {
            e.mem.set_u32(value, item * 0x10);
            e.call(0x00a5_43b0, &args![set, value]);
        }
        assert_eq!((e.mem.u32(set + 4), e.mem.u32(set + 8)), (4, 3));
        let storage = e.mem.u32(set);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [0x10, 0x20, 0x30]
        );
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4], vec![8], vec![16]]);
    }

    #[test]
    fn array_constructor_allocates_only_a_capacity() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x55);
        assert_eq!(e.call(0x00a5_4190, &args![array, 0u32, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_BASE_VTABLE);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!((e.mem.u16(array + 8), e.mem.u16(array + 0xe)), (0, 1));
        e.call(0x00a5_4190, &args![array, 0x10003u32, 2u32]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![12]]);
        assert_eq!(e.mem.u16(array + 8), 3);
        assert_ne!(e.mem.u32(array + 4), 0);
    }

    #[test]
    fn constructor_and_create_function_build_an_empty_controller() {
        let mut e = engine();
        let block = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x4c);
            block.into_ret()
        });
        assert_eq!(e.call(0x00c5_37f0, &args![]).u32(), block);
        assert_eq!(calls(&e, TIME_CONTROLLER_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), BONE_LOD_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(block + 0x34), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x38), 0);
        assert_eq!(e.mem.u32(block + 0x3c), BONE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(block + 0x4a), 1);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c5_37f0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_lod_words() {
        let mut e = engine();
        let block = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        let this = controller(&mut e, &[], 4);
        e.mem.set_u32(this + 0x34, 2);
        assert_eq!(e.call(0x00c5_3360, &args![this, 0x700u32]).u32(), block);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this, block, 0x700]]
        );
        assert_eq!((e.mem.u32(block + 0x34), e.mem.u32(block + 0x38)), (2, 4));
    }

    #[test]
    fn process_clone_rebuilds_the_sets_with_cloned_nodes() {
        let mut e = engine();
        let original = node(&mut e, 0);
        let missing = node(&mut e, 0);
        let copy = node(&mut e, 0);
        let this = controller(&mut e, &[&[original, 0, missing], &[]], 2);
        let clone = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.mem.set_u32(clone + 0x34, 0);
        e.mem.set_u32(clone + 0x38, 2);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = if a[1] == this {
                clone
            } else if a[1] == original {
                copy
            } else {
                return Ret::default();
            };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00c5_3430, &args![this, cloning]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_PROCESS_CLONE),
            vec![vec![this, cloning]]
        );
        let set_at = calls(&e, ARRAY_SET_AT);
        assert_eq!(set_at.len(), 2);
        assert_eq!((set_at[0][0], set_at[0][1]), (clone + 0x3c, 0));
        assert_eq!((set_at[1][0], set_at[1][1]), (clone + 0x3c, 1));
        let cloned_base = e.mem.u32(clone + 0x40);
        let cloned_set = e.mem.u32(cloned_base);
        assert_eq!(e.mem.u32(cloned_base + 4), 0);
        assert_eq!(e.mem.u32(cloned_set + 8), 3);
        let storage = e.mem.u32(cloned_set);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [copy, 0, 0]
        );
        // The LOD is applied again: set 0 holds the copy, which becomes visible.
        assert_eq!(e.mem.u32(clone + 0x34), 0);
        assert_eq!(e.mem.u32(copy + 0x30), 2);
    }

    #[test]
    fn load_binary_reads_the_sets_with_placeholders() {
        let mut e = engine();
        let this = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        // Words read: LOD 1, LOD count 2, two sets of 2 and 1 nodes.
        let mut words = vec![1u32, 2, 2, 2, 1];
        words.reverse();
        e.register_double(READ_WORD, move |e, a| {
            e.mem.set_u32(a[1], words.pop().unwrap());
            Ret::default()
        });
        e.call(0x00c5_3870, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LOAD_BINARY),
            vec![vec![this, 0x900]]
        );
        assert_eq!((e.mem.u32(this + 0x34), e.mem.u32(this + 0x38)), (1, 2));
        assert_eq!(e.mem.u16(this + 0x46), 2);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(base);
        let second = e.mem.u32(base + 4);
        assert_eq!((e.mem.u32(first + 8), e.mem.u32(second + 8)), (2, 1));
        assert_eq!(e.mem.u32(e.mem.u32(first)), this);
        assert_eq!(calls(&e, READ_LINK_ID).len(), 3);
        assert_eq!(calls(&e, ARRAY_SET_SIZE).len(), 2);
    }

    #[test]
    fn link_object_resolves_every_slot() {
        let mut e = engine();
        let this = controller(&mut e, &[&[1, 2], &[], &[3]], 1);
        let mut next = 0x500u32;
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next += 0x10;
            next.into_ret()
        });
        e.call(0x00c5_3a10, &args![this, 0x900u32]);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(e.mem.u32(base));
        let last = e.mem.u32(e.mem.u32(base + 8));
        assert_eq!(
            [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(last)],
            [0x510, 0x520, 0x530]
        );
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LINK_OBJECT),
            vec![vec![this, 0x900]]
        );
    }

    #[test]
    fn post_link_object_applies_the_lod_again() {
        let mut e = engine();
        let shown = node(&mut e, 0);
        let this = controller(&mut e, &[&[shown]], 1);
        e.mem.set_u32(this + 0x34, 0);
        e.call(0x00c5_3ab0, &args![this, 0x900u32]);
        assert_eq!(calls(&e, BASE_POST_LINK_OBJECT), vec![vec![this, 0x900]]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(shown + 0x30), 2);
    }

    #[test]
    fn destructors_delete_the_sets_and_the_storage() {
        let mut e = engine();
        let this = controller(&mut e, &[&[0x100]], 1);
        let base = e.mem.u32(this + 0x40);
        e.call(0x00c5_3e10, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), BONE_LOD_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), ARRAY_BASE_VTABLE);
        assert_eq!(calls(&e, FREE).last().unwrap(), &vec![base]);
        assert_eq!(calls(&e, TIME_CONTROLLER_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x4c]]);
        e.call(0x00c5_3e10, &args![this, 0u32]);
        assert_eq!(calls(&e, SIZED_DELETE).len(), 1);
    }
}
