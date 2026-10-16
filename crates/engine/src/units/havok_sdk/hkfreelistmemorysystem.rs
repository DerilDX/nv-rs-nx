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

// Translated from 00d60500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the sub-object at `+8` (unnamed in the engine map;
/// `thiscall`, `RET 8`: `first`, `second`). Its virtual table (`010d14c8`) is
/// `hkSystemMemoryBlockServer`'s (confirmed by the RTTI record before it), so
/// that is the class. Stores the virtual table, `first` at `+0x10`, clears `+8`
/// and `+0xc`, and stores `second` at `+4` (`0x20000` when it is 0). Returns `this`.
pub fn fn_00d60500(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, first);
    e.mem.set_u32(base, 0x010d_14c8);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem
        .set_u32(base + 4, if second == 0 { 0x2_0000 } else { second });
    this
}

/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkThreadMemory`'s two-word setter (`00d2c3a0`, `thiscall`) and
/// `hkSolverAllocator::setBuffer` (`00d5e960`, `thiscall`: buffer, size).
const THREAD_MEMORY_SETTER: u32 = 0x00d2_c3a0;
const SOLVER_ALLOCATOR_SET_BUFFER: u32 = 0x00d5_e960;
/// The address `InitSystem` passes as the second word of the router set-up
/// (`01094d90`; the exe's own data, not described here).
const ROUTER_NAME_WORD: u32 = 0x0109_4d90;

// Translated from 00cd0740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::mainInit` (Xbox PDB), `thiscall`, `RET 8`: `info`
/// points at two words (the frame information `00cbbfa0` constructs), copied to
/// `+0xb00` and `+0xb04`. When `flags & 1`, calls the virtual slot `+0xc` of
/// the system (`threadInit`, `00cd0830`) with the router at `+0xb0c`, the word
/// `01094d90` and `flags`. When `flags & 2` and the second word of `info` is not
/// 0, takes that many bytes from the allocator at `+4` (its slot `+4`, called with
/// the count) and hands them to the solver allocator at `+0xb30` (`00d5e960`:
/// buffer, count). Returns the router, `this + 0xb0c`.
pub fn hk_free_list_memory_system_main_init(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
    flags: u32,
) -> u32 {
    let base = this.addr();
    let first = e.mem.u32(info.addr());
    e.mem.set_u32(base + FRAME_INFO, first);
    let second = e.mem.u32(info.addr() + 4);
    e.mem.set_u32(base + FRAME_INFO + 4, second);
    if flags & 1 != 0 {
        e.vcall(
            base,
            0xc,
            &args![base + MEMORY_ROUTER, ROUTER_NAME_WORD, flags],
        );
    }
    if flags & 2 != 0 {
        let size = e.mem.u32(info.addr() + 4);
        if size != 0 {
            let allocator = e.mem.u32(base + 4);
            let buffer = e.vcall(allocator, 4, &args![size]).u32();
            e.call(
                SOLVER_ALLOCATOR_SET_BUFFER,
                &args![base + SOLVER_ALLOCATOR, buffer, size],
            );
        }
    }
    base + MEMORY_ROUTER
}

// Translated from 00cd0830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::threadInit` (Xbox PDB), `thiscall`, `RET 0xc`: the
/// virtual slot `+0xc`. When `flags & 1`, takes the first free `hkThreadMemory`
/// record (the byte at `+0x12c` of the records at `+0xc6c`, 0x138 apart) inside
/// the critical section at `+0x336c` and marks it used; when all 32 are used the
/// exe goes on with index 32 (the bytes after the table) and so does this. Stores
/// `name` at `+0x128` of the record, calls `00d2c3a0` on it with the word at
/// `+0xb08` and 8, and fills `router`: `+0`, `+4`, `+8` and `+0xc`, `+0x18` cleared,
/// `+0x10` the record, `+0x1c` the word at `+0xb08`, `+0x14` the word at `+4`
/// and `+0x20` the record index. When `flags & 2`, with the router's index (`+0x20`,
/// read from the router, which only the `flags & 1` path sets) and the byte count
/// at `+0xb00` not 0, allocates that many bytes from the allocator at `+0x1c`
/// (its slot `+4`) into `router` `+4`, `+0` and `+8` (end), records the count and the
/// router in the record (`+0x130`, `+0x134`) and sets `router + 0xc` to the record
/// and `router + 0x18` to the solver allocator at `+0xb30`.
pub fn hk_free_list_memory_system_thread_init(
    e: &mut Engine,
    this: Ptr,
    router: u32,
    name: u32,
    flags: u32,
) {
    let base = this.addr();
    let mut index = 0u32;
    if flags & 1 != 0 {
        e.call(ENTER_CRITICAL_SECTION, &args![base + CRITICAL_SECTION_B]);
        let mut flag = base + THREAD_MEMORIES + 0x12c;
        while e.mem.u8(flag) != 0 {
            index += 1;
            flag += THREAD_MEMORY_SIZE;
            if index >= THREAD_MEMORY_COUNT {
                break;
            }
        }
        if index < THREAD_MEMORY_COUNT {
            e.mem.set_u8(flag, 1);
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![base + CRITICAL_SECTION_B]);
        let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
        e.mem.set_u32(record + 0x128, name);
        let word = e.mem.u32(base + 0xb08);
        e.call(THREAD_MEMORY_SETTER, &args![record, word, 8u32]);
        e.mem.set_u32(router + 0xc, 0);
        e.mem.set_u32(router + 0x10, record);
        e.mem.set_u32(router + 0x1c, word);
        let allocator = e.mem.u32(base + 4);
        e.mem.set_u32(router + 0x14, allocator);
        e.mem.set_u32(router + 0x18, 0);
        e.mem.set_u32(router + 0x20, index);
        e.mem.set_u32(router + 4, 0);
        e.mem.set_u32(router, 0);
        e.mem.set_u32(router + 8, 0);
    }
    if flags & 2 != 0 {
        let record_index = e.mem.u32(router + 0x20);
        let size = e.mem.u32(base + 0xb00);
        let record = base + THREAD_MEMORIES + record_index.wrapping_mul(THREAD_MEMORY_SIZE);
        if size != 0 {
            let buffer = e.vcall(base + SUB_OBJECT_1C, 4, &args![size]).u32();
            e.mem.set_u32(router + 4, buffer);
            e.mem.set_u32(router, buffer);
            e.mem.set_u32(router + 8, buffer.wrapping_add(size));
            e.mem.set_u32(record + 0x130, size);
            e.mem.set_u32(record + 0x134, router);
        }
        e.mem.set_u32(router + 0xc, record);
        e.mem.set_u32(router + 0x18, base + SOLVER_ALLOCATOR);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd06c0, fn_00cd06c0(Ptr, u32)),
        entry!(0x00d60500, fn_00d60500(Ptr, u32, u32) -> Ptr),
        entry!(0x00cd0740, hk_free_list_memory_system_main_init(Ptr, Ptr, u32) -> u32),
        entry!(
            0x00cd0830,
            hk_free_list_memory_system_thread_init(Ptr, u32, u32, u32)
        ),
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

    #[test]
    fn the_block_server_constructor_defaults_its_size() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0xcd; 0x20]);
        assert_eq!(
            e.call(0x00d6_0500, &args![object, 0x55u32, 0u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), 0x010d_14c8);
        assert_eq!(e.mem.u32(object + 4), 0x2_0000);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0x55);
        e.call(0x00d6_0500, &args![object, 0x66u32, 0x4000u32]);
        assert_eq!(e.mem.u32(object + 4), 0x4000);
        assert_eq!(e.mem.u32(object + 0x10), 0x66);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
    }

    fn main_init_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(THREAD_MEMORY_SETTER, |_, _| Ret::default());
        e.register(SOLVER_ALLOCATOR_SET_BUFFER, |_, _| Ret::default());
        // The system's vtable: slot 0xc is thread_init (real).
        e.put_vtable(0x4400_0000, &[0, 0, 0, 0x00cd_0830]);
        let system = e.mem.alloc(0x3400);
        e.mem.write(system, &vec![0; 0x3400]);
        e.mem.set_u32(system, 0x4400_0000);
        (e, system)
    }

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn main_init_with_the_router_flag_runs_thread_init_and_answers_the_router() {
        let (mut e, system) = main_init_engine();
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 0x11);
        e.mem.set_u32(info + 4, 0x2000);
        e.mem.set_u32(system + 0xb08, system + 0x1c);
        e.mem.set_u32(system + 4, 0x0126_7b54);
        // Record 0 is in use, so the router gets record 1.
        e.mem.set_u8(system + 0xd98, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x00cd_0740, &args![system, info, 1u32]);
        assert_eq!(result.u32(), system + 0xb0c);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(system + 0xb00), 0x11);
        assert_eq!(e.mem.u32(system + 0xb04), 0x2000);
        let router = system + 0xb0c;
        assert_eq!(e.mem.u32(router + 0x20), 1);
        let record = system + 0xc6c + 0x138;
        assert_eq!(e.mem.u32(router + 0x10), record);
        assert_eq!(e.mem.u32(router + 0x14), 0x0126_7b54);
        assert_eq!(e.mem.u32(router + 0x1c), system + 0x1c);
        assert_eq!(e.mem.u8(system + 0xd98 + 0x138), 1);
        assert_eq!(e.mem.u32(record + 0x128), 0x0109_4d90);
        assert_eq!(
            calls_to(&log, THREAD_MEMORY_SETTER),
            vec![vec![record, system + 0x1c, 8]]
        );
        assert_eq!(
            calls_to(&log, 0x00cd_0830),
            vec![vec![system, router, 0x0109_4d90, 1]]
        );
        // No buffer without the second flag.
        assert!(calls_to(&log, SOLVER_ALLOCATOR_SET_BUFFER).is_empty());
    }

    #[test]
    fn main_init_with_the_buffer_flag_hands_the_solver_allocator_a_buffer() {
        let (mut e, system) = main_init_engine();
        // The allocator at +4: slot 4 returns a buffer.
        e.put_vtable(0x4410_0000, &[0, 0x4500_0000]);
        e.register(0x4500_0000, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x4410_0000);
        e.mem.set_u32(system + 4, allocator);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info + 4, 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0740, &args![system, info, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00cd_0830).is_empty());
        assert_eq!(calls_to(&log, 0x4500_0000), vec![vec![allocator, 0x2000]]);
        assert_eq!(
            calls_to(&log, SOLVER_ALLOCATOR_SET_BUFFER),
            vec![vec![system + 0xb30, 0x7000, 0x2000]]
        );
        // A zero count takes nothing.
        e.mem.set_u32(info + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0740, &args![system, info, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn thread_init_with_both_flags_allocates_the_router_buffer() {
        let (mut e, system) = main_init_engine();
        e.put_vtable(0x4420_0000, &[0, 0x4500_0001]);
        e.register(0x4500_0001, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.mem.set_u32(system + 0x1c, 0x4420_0000);
        e.mem.set_u32(system + 0xb00, 0x100);
        let router = e.mem.alloc(0x24);
        e.call(0x00cd_0830, &args![system, router, 0x55u32, 3u32]);
        let record = system + 0xc6c;
        assert_eq!(e.mem.u32(router), 0x9000);
        assert_eq!(e.mem.u32(router + 4), 0x9000);
        assert_eq!(e.mem.u32(router + 8), 0x9100);
        assert_eq!(e.mem.u32(router + 0xc), record);
        assert_eq!(e.mem.u32(router + 0x18), system + 0xb30);
        assert_eq!(e.mem.u32(record + 0x130), 0x100);
        assert_eq!(e.mem.u32(record + 0x134), router);
        assert_eq!(e.mem.u32(record + 0x128), 0x55);
    }

    #[test]
    fn thread_init_when_every_record_is_used_goes_on_with_index_32() {
        let (mut e, system) = main_init_engine();
        for i in 0..32 {
            e.mem.set_u8(system + 0xd98 + i * 0x138, 1);
        }
        let router = e.mem.alloc(0x24);
        e.call(0x00cd_0830, &args![system, router, 0x55u32, 1u32]);
        assert_eq!(e.mem.u32(router + 0x20), 32);
        assert_eq!(e.mem.u32(router + 0x10), system + 0xc6c + 32 * 0x138);
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
