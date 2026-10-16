//! `Havok SDK/hkfreelistmemorysystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The memory system object is a large block of sub-objects (the Xbox PDB has no
//! entry for `hkFreeListMemorySystem`, so its layout is described by offsets
//! only, each named where the constructor uses it).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkFreeListMemorySystem`'s virtual table.
const VTABLE: u32 = 0x010c_ad74;
/// The virtual table stored at `+0xab8`.
const VTABLE_AT_0AB8: u32 = 0x010c_ad3c;
/// The word `00cd0a60` stores `this + 0x1c` in.
const SYSTEM_ALLOCATOR_GLOBAL: u32 = 0x0126_86cc;

/// The sub-object at `+8` (constructor `00d60500`: two stack words).
const SUB_OBJECT_8: u32 = 0x08;
/// The sub-object at `+0x1c` (constructor `00d60270`: one stack word, the
/// address of the sub-object at `+8`).
const SUB_OBJECT_1C: u32 = 0x1c;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, one stack word) at `+0xad0`
/// and at `+0x336c`.
const CRITICAL_SECTION_A: u32 = 0xad0;
const CRITICAL_SECTION_B: u32 = 0x336c;
/// `hkMemorySystem::FrameInfo` at `+0xb00` (`00cbbfa0`).
const FRAME_INFO: u32 = 0xb00;
/// `hkMemoryRouter::hkMemoryRouter` at `+0xb0c` (`00c85770`).
const MEMORY_ROUTER: u32 = 0xb0c;
/// `hkSolverAllocator::hkSolverAllocator` at `+0xb30` (`00d5e900`).
const SOLVER_ALLOCATOR: u32 = 0xb30;
/// The first of 32 `hkThreadMemory` records (`00d2c720`) and the distance between
/// them.
const THREAD_MEMORIES: u32 = 0xc6c;
const THREAD_MEMORY_SIZE: u32 = 0x138;
const THREAD_MEMORY_COUNT: u32 = 32;

// Translated from 00cd06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word): stores its argument in
/// the words at `+4` and `+0x18` of the memory system (`+0x18` is the word at
/// `+0x10` of the sub-object at `+8`). The Havok start-up calls it with the
/// address of the allocator it has built (`0126 7b54`) right after the
/// constructor.
pub fn fn_00cd06c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 4, value);
    e.mem.set_u32(this.addr() + 0x18, value);
}

// Translated from 00cd0a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::hkFreeListMemorySystem` (Xbox PDB): stores the
/// virtual table, constructs the sub-objects (`00d60500` and `00d60270`, two
/// critical sections, the frame info, the memory router, the solver allocator and
/// 32 thread memories, each with their last three words cleared), then points the
/// global at `012686cc` and the word at `+0xb08` at `this + 0x1c`. Returns `this`.
pub fn hk_free_list_memory_system_hk_free_list_memory_system(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(0x00d6_0500, &args![base + SUB_OBJECT_8, 0u32, 0u32]);
    e.call(
        0x00d6_0270,
        &args![base + SUB_OBJECT_1C, base + SUB_OBJECT_8],
    );
    e.mem.set_u32(base + 0xab8, VTABLE_AT_0AB8);
    e.mem.set_u32(base + 0xac0, 0);
    e.mem.set_u32(base + 0xac4, 0);
    e.mem.set_u32(base + 0xac8, 0x8000_0000);
    e.mem.set_u32(base + 0xacc, 0);
    e.call(0x00cf_6da0, &args![base + CRITICAL_SECTION_A, 0u32]);
    e.call(0x00cb_bfa0, &args![base + FRAME_INFO]);
    e.call(0x00c8_5770, &args![base + MEMORY_ROUTER]);
    e.call(0x00d5_e900, &args![base + SOLVER_ALLOCATOR]);
    for index in 0..THREAD_MEMORY_COUNT {
        let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
        e.call(0x00d2_c720, &args![record]);
        e.mem.set_u32(record + 0x128, 0);
        e.mem.set_u8(record + 0x12c, 0);
        e.mem.set_u32(record + 0x130, 0);
    }
    e.call(0x00cf_6da0, &args![base + CRITICAL_SECTION_B, 0u32]);
    e.set_global(SYSTEM_ALLOCATOR_GLOBAL, base + SUB_OBJECT_1C);
    e.mem.set_u32(base + 0xb08, base + SUB_OBJECT_1C);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd06c0, fn_00cd06c0(Ptr, u32)),
        entry!(
            0x00cd0a60,
            hk_free_list_memory_system_hk_free_list_memory_system(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0x3400;

    #[test]
    fn the_setter_writes_both_words() {
        let mut e = Engine::new();
        let system = e.mem.alloc(0x40);
        e.call(0x00cd_06c0, &args![system, 0x0126_7b54u32]);
        assert_eq!(e.mem.u32(system + 4), 0x0126_7b54);
        assert_eq!(e.mem.u32(system + 0x18), 0x0126_7b54);
        assert_eq!(e.mem.u32(system), 0);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        for address in [
            0x00d6_0500,
            0x00d6_0270,
            0x00cf_6da0,
            0x00cb_bfa0,
            0x00c8_5770,
            0x00d5_e900,
            0x00d2_c720,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn the_constructor_builds_every_sub_object_in_order() {
        let mut e = constructor_engine();
        let system = e.mem.alloc(SIZE);
        e.mem.write(system, &vec![0xcd; SIZE as usize]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_0a60, &args![system]).u32(), system);
        let log = e.call_log.take().unwrap();
        let log = &log[1..];
        assert_eq!(log[0], (0x00d6_0500, vec![system + 8, 0, 0]));
        assert_eq!(log[1], (0x00d6_0270, vec![system + 0x1c, system + 8]));
        assert_eq!(log[2], (0x00cf_6da0, vec![system + 0xad0, 0]));
        assert_eq!(log[3], (0x00cb_bfa0, vec![system + 0xb00]));
        assert_eq!(log[4], (0x00c8_5770, vec![system + 0xb0c]));
        assert_eq!(log[5], (0x00d5_e900, vec![system + 0xb30]));
        let records: Vec<u32> = log[6..38].iter().map(|call| call.1[0]).collect();
        assert!(log[6..38].iter().all(|call| call.0 == 0x00d2_c720));
        let want: Vec<u32> = (0..32).map(|i| system + 0xc6c + i * 0x138).collect();
        assert_eq!(records, want);
        assert_eq!(log[38], (0x00cf_6da0, vec![system + 0x336c, 0]));
        assert_eq!(log.len(), 39);
    }

    #[test]
    fn the_constructor_sets_the_fields_and_the_global() {
        let mut e = constructor_engine();
        let system = e.mem.alloc(SIZE);
        e.mem.write(system, &vec![0xcd; SIZE as usize]);
        e.call(0x00cd_0a60, &args![system]);
        assert_eq!(e.mem.u32(system), 0x010c_ad74);
        assert_eq!(e.mem.u32(system + 4), 0);
        assert_eq!(e.mem.u32(system + 0xab8), 0x010c_ad3c);
        assert_eq!(e.mem.u32(system + 0xac0), 0);
        assert_eq!(e.mem.u32(system + 0xac4), 0);
        assert_eq!(e.mem.u32(system + 0xac8), 0x8000_0000);
        assert_eq!(e.mem.u32(system + 0xacc), 0);
        assert_eq!(e.mem.u32(system + 0xb08), system + 0x1c);
        assert_eq!(e.global::<u32>(SYSTEM_ALLOCATOR_GLOBAL), system + 0x1c);
        // Each of the 32 records has its last words cleared; the bytes the
        // constructor does not name stay as they were.
        for i in 0..32 {
            let record = system + 0xc6c + i * 0x138;
            assert_eq!(e.mem.u32(record + 0x128), 0);
            assert_eq!(e.mem.u8(record + 0x12c), 0);
            assert_eq!(e.mem.u8(record + 0x12d), 0xcd);
            assert_eq!(e.mem.u32(record + 0x130), 0);
            assert_eq!(e.mem.u32(record + 0x134), 0xcdcd_cdcd);
        }
    }
}
