//! `NiXenonRenderer/run_00e7ef80` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

/// Allocates a 0x18-byte block (four zero words, then `size` at `+0x10` and `+0x14`) and
/// keeps it in `slot` (null when the allocation fails).
fn make_array_header(e: &mut Engine, slot: u32, size: u32) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(slot, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, size);
    e.mem.set_u32(block + 0x14, size);
    e.mem.set_u32(slot, block);
}

// Translated from 00e7f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem; no arguments):
/// builds two 0x18-byte blocks from the memory manager, kept in `0126f718` (sizes 8) and
/// `0126f71c` (sizes 0x10); null in a slot whose allocation fails.
pub fn fn_00e7f2b0(e: &mut Engine) {
    make_array_header(e, 0x0126_f718, 8);
    make_array_header(e, 0x0126_f71c, 0x10);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_f2b0, fn_00e7f2b0())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_both_blocks() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            let block = e.mem.alloc(0x18);
            e.mem.write(block, &[0xaa; 0x18]);
            block.into_ret()
        });
        e.call(0x00e7_f2b0, &args![]);
        let first = e.mem.u32(0x0126_f718);
        let second = e.mem.u32(0x0126_f71c);
        assert_ne!(first, second);
        assert_eq!(e.mem.bytes(first, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(first + 0x10), 8);
        assert_eq!(e.mem.u32(first + 0x14), 8);
        assert_eq!(e.mem.u32(second + 0x10), 0x10);
        assert_eq!(e.mem.u32(second + 0x14), 0x10);
    }

    #[test]
    fn failed_allocations_leave_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(0x0126_f718, 1);
        e.mem.set_u32(0x0126_f71c, 1);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e7_f2b0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f718), 0);
        assert_eq!(e.mem.u32(0x0126_f71c), 0);
    }
}
