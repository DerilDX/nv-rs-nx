//! `NiAnimation/nigeommorpherupdatetask.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeomMorpherUpdateTask::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOM_MORPHER_UPDATE_TASK_RTTI: u32 = 0x011f_4100;

// Translated from 00a56c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherUpdateTask::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geom_morpher_update_task_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOM_MORPHER_UPDATE_TASK_RTTI)
}

/// The memory manager singleton (`thiscall` object of `00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `00aa4060` (`thiscall` on the memory manager, one stack word): deallocate.
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00e7f900` (`thiscall`, one stack word; unit `NiXenonRenderer/nid3dpass.cpp`).
const D3D_PASS_RELEASE: u32 = 0x00e7_f900;
/// The globals `00a56df0` works on: a word it clears, the block pointer it tears down (not
/// cleared), and the word copied from `011a9420` to `011a941c`.
const CLEARED_WORD: u32 = 0x011a_9414;
const TASK_BLOCK: u32 = 0x011a_9418;
const COPY_DESTINATION: u32 = 0x011a_941c;
const COPY_SOURCE: u32 = 0x011a_9420;

// Translated from 00a56df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorpherupdatetask.cpp`, no arguments): clears
/// `011a9414`; when `011a9418` points to a block, that block's first word is disposed of
/// (when the word in front of it is 0 the block from that word on goes back to the memory
/// manager, otherwise its vtable slot 0 runs with 3), `00e7f900(1)` runs on its third word
/// when that is set, and the block goes back to the memory manager (the pointer is not
/// cleared). Last, copies `011a9420` to `011a941c`.
pub fn fn_00a56df0(e: &mut Engine) {
    let block = e.mem.u32(TASK_BLOCK);
    e.mem.set_u32(CLEARED_WORD, 0);
    if block != 0 {
        let first = e.mem.u32(block);
        if first != 0 {
            let header = first.wrapping_sub(4);
            if e.mem.u32(header) == 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, header]);
            } else {
                e.vcall(first, 0, &args![3u32]);
            }
        }
        let third = e.mem.u32(block + 8);
        if third != 0 {
            e.call(D3D_PASS_RELEASE, &args![third, 1u32]);
        }
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    let value = e.mem.u32(COPY_SOURCE);
    e.mem.set_u32(COPY_DESTINATION, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a56c00, ni_geom_morpher_update_task_get_rtti() -> Ptr),
        entry!(0x00a5_6df0, fn_00a56df0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geom_morpher_update_task_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a56c00, &args![]).u32(),
            NI_GEOM_MORPHER_UPDATE_TASK_RTTI
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a5_6df0)
            .collect()
    }

    #[test]
    fn fn_00a56df0_tears_down_the_block_and_copies_the_word() {
        let mut e = Engine::new();
        e.map(CLEARED_WORD & !0xfff, 0x1000);
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(D3D_PASS_RELEASE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        e.mem.set_u32(CLEARED_WORD, 7);
        e.mem.set_u32(COPY_SOURCE, 0x1234);
        // No block: only the two globals.
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert!(calls(&mut e).is_empty());
        assert_eq!(e.mem.u32(CLEARED_WORD), 0);
        assert_eq!(e.mem.u32(COPY_DESTINATION), 0x1234);

        let block = e.mem.alloc(0xc);
        let counted = e.mem.alloc(8);
        e.mem.set_u32(counted, 1);
        e.mem.set_u32(counted + 4, 0x0150_0000);
        e.mem.set_u32(block, counted + 4);
        e.mem.set_u32(block + 8, 0x6000);
        e.mem.set_u32(TASK_BLOCK, block);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![
                (0x0150_1000, vec![counted + 4, 3]),
                (D3D_PASS_RELEASE, vec![0x6000, 1]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(TASK_BLOCK), block);

        e.mem.set_u32(counted, 0);
        e.mem.set_u32(block + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![
                (DEALLOCATE, vec![MEMORY_MANAGER, counted]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
    }
}
