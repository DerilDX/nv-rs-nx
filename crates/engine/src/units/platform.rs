//! Platform functions: the game's own versions are replaced by Rust
//! (docs/ENGINE_PORT_PLAN.md, "platform"). Hand-written, owned by the lead.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`011f6238`); [`get_memory_manager`]
/// returns its address.
pub const MEMORY_MANAGER: u32 = 0x011f_6238;

// Translated from 00401020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the `MemoryManager` singleton.
pub fn get_memory_manager(_e: &mut Engine) -> Ptr {
    Ptr::new(MEMORY_MANAGER)
}

// Translated from 00401030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `operator delete`: frees through the `MemoryManager`.
pub fn operator_delete(e: &mut Engine, block: Ptr) {
    let mm: Ptr = get_memory_manager(e);
    e.call(0x00aa_4060, &args![mm, block]);
}

// Platform: replaces 00aa3e40 (MemoryManager::Allocate (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Allocates from [`crate::Mem`]'s heap instead of the game's pools.
/// Blocks are 8-byte aligned and zero-filled.
pub fn memory_manager_allocate(e: &mut Engine, _this: Ptr, size: u32) -> Ptr {
    Ptr::new(e.mem.alloc(size))
}

// Platform: replaces 00aa4060 (MemoryManager::Deallocate (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Frees a block from [`memory_manager_allocate`]; null does nothing.
pub fn memory_manager_deallocate(e: &mut Engine, _this: Ptr, block: Ptr) {
    e.mem.free(block.addr());
}

/// `BSSystemUtility` singleton (`bssystemutility_xenon.cpp`, Xbox PDB): its
/// storage in `.data` (`011f71a8`, 0x96 bytes) and the function-local static's
/// guard word (`011f7240`, bit 0 = constructed).
pub const SYSTEM_UTILITY: u32 = 0x011f_71a8;
const SYSTEM_UTILITY_GUARD: u32 = 0x011f_7240;
/// Vtable stored by the constructor (`010a42b4`, first slot the scalar
/// deleting destructor) and the `atexit` destructor stub (`00fdc2f0`).
const SYSTEM_UTILITY_VTABLE: u32 = 0x010a_42b4;
const SYSTEM_UTILITY_ATEXIT: u32 = 0x00fd_c2f0;
/// The C runtime's `_memset` and `_atexit`.
const MEMSET: u32 = 0x00ec_61c0;
const ATEXIT: u32 = 0x00ec_658f;

// Translated from 00af30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the object `BSSystemUtility::QInstance` builds
/// (`fastcall`, `this` in ECX). The map gives it no name (range of
/// `BSSystemUtilities`); it is called only from `QInstance` and stores the
/// vtable at `010a42b4`, zeroes the fields at +0x84, +0x88, +0x8c (byte),
/// +0x90, +0x94 (byte) and +0x95 (byte), and clears the 0x80 bytes at +4.
/// Returns `this`.
pub fn fn_00af30a0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, SYSTEM_UTILITY_VTABLE);
    e.mem.set_u32(t + 0x84, 0);
    e.mem.set_u32(t + 0x88, 0);
    e.mem.set_u8(t + 0x8c, 0);
    e.mem.set_u32(t + 0x90, 0);
    e.mem.set_u8(t + 0x94, 0);
    e.mem.set_u8(t + 0x95, 0);
    e.call(MEMSET, &args![Ptr::<()>::new(t + 4), 0i32, 0x80u32]);
    this
}

// Translated from 00af2640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af2640 - the SEH frame (FS:[0] handler 00f22f4e, state -1/0 around the constructor call) and the 00ec408c security cookie are not translated.
/// `BSSystemUtility::QInstance` (Xbox PDB name; the exe's body is shared
/// with the other `_Immortalize` singleton getters, Ghidra's name): the
/// function-local static singleton. The first call sets bit 0 of the guard
/// word `011f7240`, constructs the object at `011f71a8` with `00af30a0` and
/// registers its destructor stub `00fdc2f0` with `_atexit`; every call
/// returns `011f71a8`.
pub fn bs_system_utility_q_instance(e: &mut Engine) -> Ptr {
    let guard = e.mem.u32(SYSTEM_UTILITY_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(SYSTEM_UTILITY_GUARD, guard | 1);
        fn_00af30a0(e, Ptr::new(SYSTEM_UTILITY));
        e.call(ATEXIT, &args![Ptr::<()>::new(SYSTEM_UTILITY_ATEXIT)]);
    }
    Ptr::new(SYSTEM_UTILITY)
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00401020, get_memory_manager() -> Ptr),
        entry!(0x00401030, operator_delete(Ptr)),
        entry!(0x00af2640, bs_system_utility_q_instance() -> Ptr),
        entry!(0x00af30a0, fn_00af30a0(Ptr) -> Ptr),
        entry!(0x00aa3e40, memory_manager_allocate(Ptr, u32) -> Ptr),
        entry!(0x00aa4060, memory_manager_deallocate(Ptr, Ptr)),
    ]
}

// Imported Windows functions. The game calls them through its import
// table (`call dword ptr [slot]`); translations call the slot address, and
// `Engine::with_exe` registers these there by DLL and name. One thread, so
// the interlocked operations are plain memory operations and the critical
// sections do nothing.

// Platform: replaces KERNEL32.DLL InterlockedIncrement (import)
fn interlocked_increment(e: &mut Engine, target: Ptr) -> i32 {
    let v = e.mem.i32(target.addr()).wrapping_add(1);
    e.mem.set_i32(target.addr(), v);
    v
}

// Platform: replaces KERNEL32.DLL InterlockedDecrement (import)
fn interlocked_decrement(e: &mut Engine, target: Ptr) -> i32 {
    let v = e.mem.i32(target.addr()).wrapping_sub(1);
    e.mem.set_i32(target.addr(), v);
    v
}

// Platform: replaces KERNEL32.DLL InterlockedExchange (import)
fn interlocked_exchange(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    e.mem.set_i32(target.addr(), value);
    old
}

// Platform: replaces KERNEL32.DLL InterlockedExchangeAdd (import)
fn interlocked_exchange_add(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    e.mem.set_i32(target.addr(), old.wrapping_add(value));
    old
}

// Platform: replaces KERNEL32.DLL InterlockedCompareExchange (import)
fn interlocked_compare_exchange(e: &mut Engine, target: Ptr, exchange: i32, comparand: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    if old == comparand {
        e.mem.set_i32(target.addr(), exchange);
    }
    old
}

// Platform: replaces KERNEL32.DLL critical sections (imports): one thread.
fn critical_section_nop(_e: &mut Engine, _section: Ptr) {}

// Platform: replaces KERNEL32.DLL TryEnterCriticalSection (import)
fn try_enter_critical_section(_e: &mut Engine, _section: Ptr) -> i32 {
    1
}

// Platform: replaces KERNEL32.DLL GetCurrentThreadId (import): the main
// thread's id. Code that compares thread ids sees the main thread.
pub const MAIN_THREAD_ID: u32 = 1;
fn get_current_thread_id(_e: &mut Engine) -> u32 {
    MAIN_THREAD_ID
}

// Platform: replaces KERNEL32.DLL Sleep (import)
fn sleep(_e: &mut Engine, _ms: u32) {}

// Platform: replaces KERNEL32.DLL GetTickCount and WINMM.DLL timeGetTime (imports)
fn tick_count(e: &mut Engine) -> u32 {
    e.clock_ms
}

// Platform: replaces KERNEL32.DLL QueryPerformanceFrequency (import): 1 MHz.
fn query_performance_frequency(e: &mut Engine, out: Ptr) -> i32 {
    e.mem.set_u64(out.addr(), 1_000_000);
    1
}

// Platform: replaces KERNEL32.DLL QueryPerformanceCounter (import): the
// engine clock in microseconds.
fn query_performance_counter(e: &mut Engine, out: Ptr) -> i32 {
    e.mem.set_u64(out.addr(), e.clock_ms as u64 * 1000);
    1
}

/// (DLL, name, platform version) of the imports with a Rust stand-in.
pub fn imports() -> Vec<(&'static str, &'static str, AbiFn)> {
    let k = "KERNEL32.dll";
    vec![
        (
            k,
            "InterlockedIncrement",
            entry!(0, interlocked_increment(Ptr) -> i32).1,
        ),
        (
            k,
            "InterlockedDecrement",
            entry!(0, interlocked_decrement(Ptr) -> i32).1,
        ),
        (
            k,
            "InterlockedExchange",
            entry!(0, interlocked_exchange(Ptr, i32) -> i32).1,
        ),
        (
            k,
            "InterlockedExchangeAdd",
            entry!(0, interlocked_exchange_add(Ptr, i32) -> i32).1,
        ),
        (
            k,
            "InterlockedCompareExchange",
            entry!(0, interlocked_compare_exchange(Ptr, i32, i32) -> i32).1,
        ),
        (
            k,
            "EnterCriticalSection",
            entry!(0, critical_section_nop(Ptr)).1,
        ),
        (
            k,
            "LeaveCriticalSection",
            entry!(0, critical_section_nop(Ptr)).1,
        ),
        (
            k,
            "InitializeCriticalSection",
            entry!(0, critical_section_nop(Ptr)).1,
        ),
        (
            k,
            "DeleteCriticalSection",
            entry!(0, critical_section_nop(Ptr)).1,
        ),
        (
            k,
            "TryEnterCriticalSection",
            entry!(0, try_enter_critical_section(Ptr) -> i32).1,
        ),
        (
            k,
            "GetCurrentThreadId",
            entry!(0, get_current_thread_id() -> u32).1,
        ),
        (k, "Sleep", entry!(0, sleep(u32)).1),
        (k, "GetTickCount", entry!(0, tick_count() -> u32).1),
        ("WINMM.dll", "timeGetTime", entry!(0, tick_count() -> u32).1),
        (
            k,
            "QueryPerformanceFrequency",
            entry!(0, query_performance_frequency(Ptr) -> i32).1,
        ),
        (
            k,
            "QueryPerformanceCounter",
            entry!(0, query_performance_counter(Ptr) -> i32).1,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_then_delete_through_the_game_calls() {
        let mut e = Engine::new();
        let mm = MEMORY_MANAGER;
        let p: Ptr = e.call_as(0x00aa_3e40, &args![mm, 24u32]);
        assert_eq!(e.mem.block_size(p.addr()), Some(24));
        e.call(0x0040_1030, &args![p]);
        assert_eq!(e.mem.block_size(p.addr()), None);
    }

    #[test]
    fn system_utility_is_built_once_and_registered_for_exit() {
        let mut e = Engine::new();
        e.mem.map(SYSTEM_UTILITY, 0x100);
        e.mem.map(SYSTEM_UTILITY_GUARD, 4);
        e.mem.write(SYSTEM_UTILITY + 4, &[0xaa; 0x80]);
        e.mem.set_u32(SYSTEM_UTILITY + 0x84, 7);
        e.mem.set_u8(SYSTEM_UTILITY + 0x95, 9);
        e.mem.set_u32(SYSTEM_UTILITY_GUARD, 0x10); // other guard bits are kept
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let p: Ptr = e.call_as(0x00af_2640, &args![]);
        assert_eq!(p.addr(), SYSTEM_UTILITY);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY_GUARD), 0x11);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY), SYSTEM_UTILITY_VTABLE);
        assert_eq!(e.mem.bytes(SYSTEM_UTILITY + 4, 0x80), vec![0u8; 0x80]);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY + 0x84), 0);
        assert_eq!(e.mem.u8(SYSTEM_UTILITY + 0x95), 0);
        let log = e.call_log.clone().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == ATEXIT).count(), 1);

        // The second call neither rebuilds nor registers again.
        e.mem.set_u32(SYSTEM_UTILITY + 0x84, 5);
        e.call_log = Some(vec![]);
        let q: Ptr = e.call_as(0x00af_2640, &args![]);
        assert_eq!(q.addr(), SYSTEM_UTILITY);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY + 0x84), 5);
        assert!(e.call_log.unwrap().iter().all(|c| c.0 != ATEXIT));
    }
}
