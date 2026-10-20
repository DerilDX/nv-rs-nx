//! `BSMain/bsparticlesystemmanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5ea70` (named `NiNode::GetViewerStrings` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const VIEWER_STRINGS_TARGET_00A5EA70: u32 = 0x00a5_ea70;

// Translated from 00c51570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::GetViewerStrings` (engine map vtable name, vtable word 010c245c):
/// forwards `this` and the string array to `00a5ea70` (`NiNode::GetViewerStrings`). Returns
/// nothing: Xbox PDB `?GetViewerStrings@NiObject@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z` is void;
/// `strings` is the `NiTPrimitiveArray<char *>` pointer. Parameter count from the `RET 4` of
/// both bodies.
pub fn bs_particle_system_manager_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VIEWER_STRINGS_TARGET_00A5EA70, &args![this, strings]);
}

// Translated from 00c516f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01203110 without reading the receiver. The map names this body `BSParticleSystemManager::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c516f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_3110
}

/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `BSParticleSystemManager`'s vtable (`010c23fc`, RTTI `.?AVBSParticleSystemManager@@`).
const BS_PARTICLE_SYSTEM_MANAGER_VTABLE: u32 = 0x010c_23fc;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// CRT `memset` (`00ec61c0`).
const MEMSET: u32 = 0x00ec_61c0;
/// `BSTaskletManager::QInstance` (engine map, `00b00a00`, no arguments) and
/// `BSTaskletManager::CreateTaskGroup` (engine map, `00b00a80`, `thiscall`, `RET 4`: the
/// group word's address).
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;
const TASKLET_MANAGER_CREATE_TASK_GROUP: u32 = 0x00b0_0a80;

// Translated from 00c503d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::BSParticleSystemManager` (Xbox PDB), `thiscall`, no stack
/// words; answers `this`. Constructs the `NiNode` (child array size 0), sets the vtable, the
/// count at `+0xb0` to 10 and the 0x28-byte array at `+0xac` (memory manager), fills
/// `count * 4` bytes of it with 0xff, makes a one-word block (cleared; null when the
/// allocation fails) at `+0xb4` and has the tasklet manager create a task group into it
/// (`CreateTaskGroup` with the word at `+0xb4`).
// Translation incomplete: 00c503d0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn bs_particle_system_manager_bs_particle_system_manager(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    e.mem.set_u32(base, BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0xb0), 10);
    let array = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x28u32])
        .u32();
    e.mem.set_u32(base.wrapping_add(0xac), array);
    let bytes = e.mem.u32(base.wrapping_add(0xb0)).wrapping_shl(2);
    let array = e.mem.u32(base.wrapping_add(0xac));
    e.call(MEMSET, &args![array, 0xffff_ffffu32, bytes]);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let block = if block != 0 {
        e.mem.set_u32(block, 0);
        block
    } else {
        0
    };
    e.mem.set_u32(base.wrapping_add(0xb4), block);
    let group = e.mem.u32(base.wrapping_add(0xb4));
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    e.call(TASKLET_MANAGER_CREATE_TASK_GROUP, &args![manager, group]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5_03d0,
            bs_particle_system_manager_bs_particle_system_manager(Ptr) -> Ptr
        ),
        entry!(0x00c5_16f0, fn_00c516f0(Ptr) -> u32),
        entry!(
            0x00c5_1570,
            bs_particle_system_manager_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c5_0ec0, bs_particle_system_manager_find_child_index(Ptr, u32) -> u32),
        entry!(0x00c5_0f30, fn_00c50f30(Ptr, u32) -> u32),
        entry!(
            0x00c5_05c0,
            bs_particle_system_manager_add_emitter(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c5_09f0,
            bs_particle_system_manager_attach_child(Ptr, Ptr, u32)
        ),
        entry!(0x00c5_04e0, bs_particle_system_manager_destructor(Ptr)),
        entry!(
            0x00c5_1590,
            bs_particle_system_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_0610, fn_00c50610(Ptr, u32)),
    ]
}

// ---- Batch b012 --------------------------------------------------------------------------

/// `BSParticleSystemManager` on PC: `NiNode::m_kChildren` elements at `+0xa0`, size
/// `+0xa6`, non-null count `+0xa8`; the index table (one child index per master system,
/// -1 for none) at `+0xac` with its length at `+0xb0`; the task group word at `+0xb4`.
const CHILDREN: u32 = 0xa0;
const CHILD_COUNT: u32 = 0xa6;
const CHILDREN_USED: u32 = 0xa8;
const INDEX_TABLE: u32 = 0xac;
const INDEX_COUNT: u32 = 0xb0;
const TASK_GROUP: u32 = 0xb4;
/// The memory manager's free (`00aa4060`, `thiscall`, one word).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// CRT `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiNode::AttachChild` (`00a5ed10`, `RET 8`) and the `NiNode` destructor body
/// (`00a5e930`).
const NI_NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
const NI_NODE_DESTRUCTOR: u32 = 0x00a5_e930;
/// `BSMasterParticleSystem::GetRTTI` answers this record (`0120436c`).
const MASTER_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_436c;
/// `00c5dcd0` (`thiscall`, `RET 4`): adds an emitter to a master particle system.
const MASTER_SYSTEM_ADD_EMITTER: u32 = 0x00c5_dcd0;
/// `005b5e40` (`cdecl`, variable arguments; engine-map unit `tesscriptfunctions.cpp`): the
/// debug print `AddEmitter` reports a missing system with, and its format (`010c2500`,
/// "DEFAULT: Trying to add an emitter (%s) to a master particle system (idx: %d) ...").
const DEBUG_PRINT: u32 = 0x005b_5e40;
const MISSING_SYSTEM_FORMAT: u32 = 0x010c_2500;
/// `BSTaskletManager::DestroyTaskGroup` (`00b00c20`) and the other task-group calls of
/// `00c50610` (`00b00ae0`, `00b00b40`, `00b00bc0`), all `thiscall` on the manager.
const TASKLET_MANAGER_DESTROY_TASK_GROUP: u32 = 0x00b0_0c20;
const TASK_GROUP_00B00AE0: u32 = 0x00b0_0ae0;
const TASK_GROUP_00B00B40: u32 = 0x00b0_0b40;
const TASK_GROUP_00B00BC0: u32 = 0x00b0_0bc0;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`, five words), the
/// element constructor and destructor `00c50610` gives it (`00c51640`, `00c516d0`) and the
/// `BSTasklet` vtable of its stack tasklet (`0106c5d8`).
const EH_VECTOR_CONSTRUCTOR: u32 = 0x00ec_782f;
const TASK_ELEMENT_CONSTRUCTOR: u32 = 0x00c5_1640;
const TASK_ELEMENT_DESTRUCTOR: u32 = 0x00c5_16d0;
const TASKLET_VTABLE: u32 = 0x0106_c5d8;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00c50ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::FindChildIndex` (Xbox PDB; `thiscall`, `RET 4`): the first
/// index below the non-null child count (`+0xa8`, read once) whose child is `child`, or
/// `0xffffffff`.
pub fn bs_particle_system_manager_find_child_index(e: &mut Engine, this: Ptr, child: u32) -> u32 {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + CHILDREN_USED));
    (0..count)
        .find(|index| e.mem.u32(e.mem.u32(base + CHILDREN) + 4 * index) == child)
        .unwrap_or(0xffff_ffff)
}

// Translated from 00c50f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsparticlesystemmanager.obj`; `thiscall`, `RET 4`): the
/// child the index table names for master system `index`, or 0 when `index` is past the
/// table or its entry is -1.
pub fn fn_00c50f30(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    if index >= e.mem.u32(base + INDEX_COUNT) {
        return 0;
    }
    let child_index = e
        .mem
        .u32(e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index));
    if child_index == 0xffff_ffff {
        return 0;
    }
    e.mem.u32(
        e.mem
            .u32(base + CHILDREN)
            .wrapping_add(child_index.wrapping_mul(4)),
    )
}

// Translated from 00c505c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::AddEmitter` (Xbox PDB; `thiscall`, `RET 8`): hands `emitter`
/// to the master system at `index` (`00c50f30`, then `00c5dcd0`); without one, the debug
/// print `005b5e40` reports the emitter's name (its word at `+8`) and the index.
pub fn bs_particle_system_manager_add_emitter(e: &mut Engine, this: Ptr, index: u32, emitter: Ptr) {
    let system = fn_00c50f30(e, this, index);
    if system != 0 {
        e.call(MASTER_SYSTEM_ADD_EMITTER, &args![system, emitter]);
    } else {
        let name = e.mem.u32(emitter.addr() + 8);
        e.call(DEBUG_PRINT, &args![MISSING_SYSTEM_FORMAT, name, index]);
    }
}

// Translated from 00c509f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::AttachChild` (Xbox PDB; `thiscall`, `RET 8`, the child and a
/// byte): only a `BSMasterParticleSystem` (its `GetRTTI`, slot 8, answers `0120436c`)
/// with an index (`+0xc4`, not -1) is attached, and only when its table entry is -1 or
/// past the table. A table too short grows to `((index - length + 1) / 10 + 1) * 10 +
/// length` entries (memory manager; the byte count saturates to `0xffffffff` on overflow),
/// filled with -1, the old entries copied over and the old table freed. Then
/// `NiNode::AttachChild` and the child's index (`FindChildIndex`) stored in the entry.
pub fn bs_particle_system_manager_attach_child(e: &mut Engine, this: Ptr, child: Ptr, flag: u32) {
    let base = this.addr();
    let is_master =
        !child.is_null() && e.vcall(child.addr(), 8, &[]).u32() == MASTER_PARTICLE_SYSTEM_RTTI;
    if !is_master {
        return;
    }
    let index = e.mem.u32(child.addr() + 0xc4);
    if index == 0xffff_ffff {
        return;
    }
    let length = e.mem.u32(base + INDEX_COUNT);
    if index < length {
        if e.mem
            .u32(e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index))
            != 0xffff_ffff
        {
            return;
        }
    } else {
        let blocks = index.wrapping_sub(length).wrapping_add(1) / 10 + 1;
        let new_length = blocks
            .wrapping_mul(10)
            .wrapping_add(e.mem.u32(base + INDEX_COUNT));
        let bytes = if u64::from(new_length) * 4 > u64::from(u32::MAX) {
            0xffff_ffff
        } else {
            new_length * 4
        };
        let table = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        e.call(MEMSET, &args![table, 0xffff_ffffu32, new_length << 2]);
        let old_bytes = e.mem.u32(base + INDEX_COUNT) << 2;
        let old = e.mem.u32(base + INDEX_TABLE);
        e.call(MEMCPY, &args![table, old, old_bytes]);
        let old = e.mem.u32(base + INDEX_TABLE);
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, old]);
        e.mem.set_u32(base + INDEX_TABLE, table);
        e.mem.set_u32(base + INDEX_COUNT, new_length);
    }
    e.call(NI_NODE_ATTACH_CHILD, &args![this, child, flag & 0xff]);
    let child_index = bs_particle_system_manager_find_child_index(e, this, child.addr());
    let entry = e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index);
    e.mem.set_u32(entry, child_index);
}

// Translated from 00c504e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::~BSParticleSystemManager` (Xbox PDB; `thiscall`): sets the
/// vtable, frees the index table (memory manager) and clears it and its length, has the
/// tasklet manager destroy the task group (`DestroyTaskGroup` with the word at `+0xb4`),
/// frees the group's block when there is one (behind a constant-true test), and ends in
/// the `NiNode` destructor body (`00a5e930`).
// Translation incomplete: 00c504e0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn bs_particle_system_manager_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
    let table = e.mem.u32(base + INDEX_TABLE);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, table]);
    e.mem.set_u32(base + INDEX_TABLE, 0);
    e.mem.set_u32(base + INDEX_COUNT, 0);
    let group = e.mem.u32(base + TASK_GROUP);
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    e.call(TASKLET_MANAGER_DESTROY_TASK_GROUP, &args![manager, group]);
    let block = e.mem.u32(base + TASK_GROUP);
    // `MOV EDX,1; AND EDX,1; JZ`: the free always runs for a block.
    if block != 0 && std::hint::black_box(1u32) & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
    }
    e.call(NI_NODE_DESTRUCTOR, &args![this]);
}

// Translated from 00c51590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// the destructor (`00c504e0`), then the sized delete of 0xb8 bytes when bit 0 of `flags`
/// is set. Answers `this`.
pub fn bs_particle_system_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bs_particle_system_manager_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xb8u32]);
    }
    this
}

// Translated from 00c50610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsparticlesystemmanager.obj`; `thiscall`, `RET 4`):
/// updates the children with `time`, then runs the manager's own slot `0xfc`. Each
/// non-null child gets its slot `0xa4` with `(time, 0)`. A local flag the exe sets to
/// false (so the rest never runs; translated as the exe has it) selects a threaded form:
/// an array of 0x20-byte task entries (count word first; `eh vector constructor iterator`
/// with `00c51640`/`00c516d0`), the task group reset (its object's slot 8 with 0, then
/// `00b00ae0`), one `BSTasklet` per child queued with `00b00b40` (the entry holding the
/// child at `+0x18` and `time` at `+0x1c`), then `00b00bc0`, the group object's slot 8 with
/// 99 and slot 4 with 0 (when its byte at `+4` is clear) after queuing, and the array
/// deleted (its slot 0 with 3, or a plain free when the count word is 0).
// Translation incomplete: 00c50610 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) and the /GS cookie word are not translated.
pub fn fn_00c50610(e: &mut Engine, this: Ptr, time: u32) {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + CHILD_COUNT));
    let threaded = std::hint::black_box(false);
    let mut queued = 0u32;
    let mut entries = 0u32;
    if threaded {
        let bytes = if u64::from(count) * 0x20 > u64::from(u32::MAX) {
            0xffff_ffff
        } else {
            count * 0x20
        };
        let bytes = bytes.saturating_add(4);
        let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        if block != 0 {
            e.mem.set_u32(block, count);
            e.call(
                EH_VECTOR_CONSTRUCTOR,
                &args![
                    block + 4,
                    0x20u32,
                    count,
                    TASK_ELEMENT_CONSTRUCTOR,
                    TASK_ELEMENT_DESTRUCTOR
                ],
            );
            entries = block + 4;
        }
        // The word at +0xb4 is the block holding the group object.
        let group = e.mem.u32(base + TASK_GROUP);
        let object = e.mem.u32(group);
        if object != 0 {
            e.vcall(object, 8, &args![0u32]);
        }
        let group = e.mem.u32(base + TASK_GROUP);
        let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
        e.call(TASK_GROUP_00B00AE0, &args![manager, group]);
    }
    let mut index = 0u32;
    while index < count {
        let child = e.mem.u32(e.mem.u32(base + CHILDREN) + 4 * index);
        if child != 0 {
            if threaded {
                e.with_stack(8, |e, tasklet| {
                    e.mem.set_u32(tasklet.addr(), TASKLET_VTABLE);
                    e.mem.set_u32(tasklet.addr() + 4, 0);
                    let entry = entries.wrapping_add(index << 5);
                    e.mem.set_u32(entry + 0x18, child);
                    e.mem.set_u32(entry + 0x1c, time);
                    e.mem.set_u32(tasklet.addr() + 4, entry);
                    let group = e.mem.u32(base + TASK_GROUP);
                    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
                    e.call(TASK_GROUP_00B00B40, &args![manager, group, tasklet, 0u32]);
                    queued += 1;
                    e.mem.set_u32(tasklet.addr(), TASKLET_VTABLE);
                });
            } else {
                e.vcall(child, 0xa4, &args![time, 0u32]);
            }
        }
        index += 1;
    }
    if threaded {
        let group = e.mem.u32(base + TASK_GROUP);
        let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
        e.call(TASK_GROUP_00B00BC0, &args![manager, group]);
        if queued != 0 {
            let group = e.mem.u32(base + TASK_GROUP);
            if e.mem.u32(group) != 0 {
                e.vcall(e.mem.u32(group), 8, &args![99u32]);
            }
            let group = e.mem.u32(base + TASK_GROUP);
            let object = e.mem.u32(group);
            if object != 0 && e.mem.u8(object + 4) == 0 {
                e.vcall(object, 4, &args![0u32]);
            }
        }
        if entries != 0 {
            if e.mem.u32(entries - 4) != 0 {
                e.vcall(entries, 0, &args![3u32]);
            } else {
                e.call(MEMORY_FREE, &args![MEMORY_MANAGER, entries - 4]);
            }
        }
    }
    e.vcall(base, 0xfc, &[]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_particle_system_manager_get_viewer_strings_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET_00A5EA70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_1570, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VIEWER_STRINGS_TARGET_00A5EA70)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c516f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_16f0, &args![0x1000u32]).u32(), 0x0120_3110);
        assert_eq!(e.call(0x00c5_16f0, &args![0u32]).u32(), 0x0120_3110);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    fn rig(block_fails: bool) -> Engine {
        let mut e = Engine::new();
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            if a[1] == 4 && block_fails {
                0u32.into_ret()
            } else {
                let block = e.mem.alloc(a[1]);
                e.mem.set_u32(block, 0xdddd_dddd);
                block.into_ret()
            }
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| 0x0777_0000u32.into_ret());
        e.register(TASKLET_MANAGER_CREATE_TASK_GROUP, |_, _| Ret::default());
        e
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_fills_the_array_and_creates_the_task_group() {
        let mut e = rig(false);
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_03d0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_NODE_CONSTRUCT), vec![vec![this, 0]]);
        assert_eq!(e.mem.u32(this), BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb0), 10);
        let array = e.mem.u32(this + 0xac);
        assert_eq!(calls(&log, MEMSET), vec![vec![array, 0xffff_ffff, 40]]);
        assert_eq!(e.mem.u32(array + 36), 0xffff_ffff);
        let block = e.mem.u32(this + 0xb4);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(
            calls(&log, TASKLET_MANAGER_CREATE_TASK_GROUP),
            vec![vec![0x0777_0000, block]]
        );
    }

    #[test]
    fn constructor_passes_a_null_group_when_its_allocation_fails() {
        let mut e = rig(true);
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        e.call(0x00c5_03d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(
            calls(&log, TASKLET_MANAGER_CREATE_TASK_GROUP),
            vec![vec![0x0777_0000, 0]]
        );
    }
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    const CHILD_VTABLE: u32 = 0x0300_0000;
    const CHILD_RTTI: u32 = 0x0300_1000;
    const CHILD_UPDATE: u32 = 0x0300_2000;
    const OWN_VTABLE: u32 = 0x0300_3000;
    const OWN_SLOT_FC: u32 = 0x0300_4000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            MEMORY_FREE,
            MEMCPY,
            NI_NODE_ATTACH_CHILD,
            NI_NODE_DESTRUCTOR,
            MASTER_SYSTEM_ADD_EMITTER,
            DEBUG_PRINT,
            TASKLET_MANAGER_DESTROY_TASK_GROUP,
            SIZED_DELETE,
            CHILD_UPDATE,
            OWN_SLOT_FC,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| 0x7100u32.into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMSET, |e, a| {
            for at in 0..a[2] {
                e.mem.set_u8(a[0] + at, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CHILD_RTTI, |_, _| MASTER_PARTICLE_SYSTEM_RTTI.into_ret());
        let mut slots = vec![0u32; 0x2a];
        slots[2] = CHILD_RTTI;
        slots[0xa4 / 4] = CHILD_UPDATE;
        e.put_vtable(CHILD_VTABLE, &slots);
        let mut own = vec![0u32; 0x40];
        own[0xfc / 4] = OWN_SLOT_FC;
        e.put_vtable(OWN_VTABLE, &own);
        e.call_log = Some(vec![]);
        e
    }

    /// A manager with `children` and an index table of `table`.
    fn manager(e: &mut Engine, children: &[u32], table: &[u32]) -> u32 {
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this, OWN_VTABLE);
        let elements = e.mem.alloc(4 * (children.len() as u32).max(1));
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *child);
        }
        e.mem.set_u32(this + CHILDREN, elements);
        e.mem.set_u16(this + CHILD_COUNT, children.len() as u16);
        e.mem.set_u16(this + CHILDREN_USED, children.len() as u16);
        let entries = e.mem.alloc(4 * (table.len() as u32).max(1));
        for (index, value) in table.iter().enumerate() {
            e.mem.set_u32(entries + 4 * index as u32, *value);
        }
        e.mem.set_u32(this + INDEX_TABLE, entries);
        e.mem.set_u32(this + INDEX_COUNT, table.len() as u32);
        this
    }

    fn child(e: &mut Engine, index: u32) -> u32 {
        let at = e.mem.alloc(0xd0);
        e.mem.set_u32(at, CHILD_VTABLE);
        e.mem.set_u32(at + 0xc4, index);
        at
    }

    #[test]
    fn finding_and_naming_children() {
        let mut e = engine();
        let (a, b) = (child(&mut e, 0), child(&mut e, 1));
        let this = manager(&mut e, &[a, b], &[1, 0xffff_ffff]);
        assert_eq!(e.call(0x00c5_0ec0, &args![this, b]).u32(), 1);
        assert_eq!(
            e.call(0x00c5_0ec0, &args![this, 0x9999u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.call(0x00c5_0f30, &args![this, 0u32]).u32(), b);
        assert_eq!(e.call(0x00c5_0f30, &args![this, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00c5_0f30, &args![this, 2u32]).u32(), 0);
    }

    #[test]
    fn emitters_go_to_their_system_or_are_reported() {
        let mut e = engine();
        let a = child(&mut e, 0);
        let this = manager(&mut e, &[a], &[0, 0xffff_ffff]);
        let emitter = e.mem.alloc(0x10);
        e.mem.set_u32(emitter + 8, 0x4242);
        e.call(0x00c5_05c0, &args![this, 0u32, emitter]);
        assert_eq!(calls(&e, MASTER_SYSTEM_ADD_EMITTER), vec![vec![a, emitter]]);
        e.call(0x00c5_05c0, &args![this, 1u32, emitter]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![MISSING_SYSTEM_FORMAT, 0x4242, 1]]
        );
    }

    #[test]
    fn attaching_fills_the_index_table() {
        let mut e = engine();
        let a = child(&mut e, 1);
        let this = manager(&mut e, &[], &[0xffff_ffff, 0xffff_ffff]);
        e.register(NI_NODE_ATTACH_CHILD, |e, a| {
            // The node puts the child at index 0.
            let elements = e.mem.alloc(4);
            e.mem.set_u32(elements, a[1]);
            e.mem.set_u32(a[0] + CHILDREN, elements);
            e.mem.set_u16(a[0] + CHILDREN_USED, 1);
            Ret::default()
        });
        e.call(0x00c5_09f0, &args![this, a, 0x101u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD), vec![vec![this, a, 1]]);
        let table = e.mem.u32(this + INDEX_TABLE);
        assert_eq!(e.mem.u32(table + 4), 0);
        // Already present: nothing more.
        e.call(0x00c5_09f0, &args![this, a, 0u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD).len(), 1);
        // Past the table: it grows to 2 + 10 entries, -1 filled, old entries kept.
        let far = child(&mut e, 5);
        e.call(0x00c5_09f0, &args![this, far, 0u32]);
        assert_eq!(e.mem.u32(this + INDEX_COUNT), 12);
        let grown = e.mem.u32(this + INDEX_TABLE);
        assert_ne!(grown, table);
        assert_eq!(calls(&e, MEMCPY), vec![vec![grown, table, 8]]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, table]]);
        assert_eq!(e.mem.u32(grown + 4 * 11), 0xffff_ffff);
        // No index, or not a master system: ignored.
        let none = child(&mut e, 0xffff_ffff);
        e.call(0x00c5_09f0, &args![this, none, 0u32]);
        e.register(CHILD_RTTI, |_, _| 0x1234u32.into_ret());
        let other = child(&mut e, 3);
        e.call(0x00c5_09f0, &args![this, other, 0u32]);
        e.call(0x00c5_09f0, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD).len(), 2);
    }

    #[test]
    fn the_destructors_free_the_table_and_the_group() {
        let mut e = engine();
        let this = manager(&mut e, &[], &[0xffff_ffff]);
        let table = e.mem.u32(this + INDEX_TABLE);
        e.mem.set_u32(this + TASK_GROUP, 0x6600);
        assert_eq!(e.call(0x00c5_1590, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
        assert_eq!(
            (e.mem.u32(this + INDEX_TABLE), e.mem.u32(this + INDEX_COUNT)),
            (0, 0)
        );
        assert_eq!(
            calls(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, table], vec![MEMORY_MANAGER, 0x6600]]
        );
        assert_eq!(
            calls(&e, TASKLET_MANAGER_DESTROY_TASK_GROUP),
            vec![vec![0x7100, 0x6600]]
        );
        assert_eq!(calls(&e, NI_NODE_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0xb8]]);
        e.mem.set_u32(this + TASK_GROUP, 0);
        e.call(0x00c5_04e0, &args![this]);
        assert_eq!(calls(&e, MEMORY_FREE).len(), 3);
    }

    #[test]
    fn updating_runs_each_child_then_slot_0xfc() {
        let mut e = engine();
        let (a, b) = (child(&mut e, 0), child(&mut e, 1));
        let this = manager(&mut e, &[a, 0, b], &[]);
        e.call(0x00c5_0610, &args![this, 0x3f80_0000u32]);
        assert_eq!(
            calls(&e, CHILD_UPDATE),
            vec![vec![a, 0x3f80_0000, 0], vec![b, 0x3f80_0000, 0]]
        );
        assert_eq!(calls(&e, OWN_SLOT_FC), vec![vec![this]]);
    }
}
