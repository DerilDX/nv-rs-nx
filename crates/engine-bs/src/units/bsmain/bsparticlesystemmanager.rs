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
    ]
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
