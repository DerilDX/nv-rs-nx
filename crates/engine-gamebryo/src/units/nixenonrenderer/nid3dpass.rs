//! `NiXenonRenderer/nid3dpass.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00e7ec90` (no stack words): the step `fn_00e80620` ends with, by a jump.
const NEXT_STEP: u32 = 0x00e7_ec90;

// Translated from 00e80150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`; no arguments): allocates 0x18 bytes from
/// the memory manager, zeroes the first four words, sets the words at `+0x10` and `+0x14`
/// to 8 and keeps the block in `0126f72c` (null when the allocation fails).
pub fn fn_00e80150(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f72c, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f72c, block);
}

// Translated from 00e80620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; no arguments): runs `fn_00e80150`, then jumps to
/// `00e7ec90`.
pub fn fn_00e80620(e: &mut Engine) {
    fn_00e80150(e);
    e.call(NEXT_STEP, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_0150, fn_00e80150()),
        entry!(0x00e8_0620, fn_00e80620()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_and_runs_the_next_step() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.register(NEXT_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_0620, &args![]);
        assert_eq!(e.mem.u32(0x0126_f72c), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        let order: Vec<u32> = e
            .call_log
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| *a != MEMORY_MANAGER_ALLOCATE)
            .collect();
        assert_eq!(order, vec![0x00e8_0620, NEXT_STEP]);
    }

    #[test]
    fn failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(0x0126_f72c, 5);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e8_0150, &args![]);
        assert_eq!(e.mem.u32(0x0126_f72c), 0);
    }
}
