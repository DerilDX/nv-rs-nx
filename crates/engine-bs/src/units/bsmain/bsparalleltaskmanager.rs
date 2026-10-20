//! `BSMain/bsparalleltaskmanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`this` of `MemoryManager::Allocate`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (Xbox PDB): `thiscall(size)`.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSLinearTaskThread`'s constructor (`008c76a0`, `thiscall`, no stack words).
const LINEAR_TASK_THREAD_CONSTRUCT: u32 = 0x008c_76a0;
/// `sprintf_s(buffer, size, format, ...)` (`cdecl`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `BSThread::Initialize` (Xbox PDB): `thiscall(stack size, name, suspended)`.
const THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `BSThread::SetName` (Xbox PDB): `thiscall(name)`.
const THREAD_SET_NAME: u32 = 0x00aa_6540;
/// `BSThread::SetThreadProcessor` (Xbox PDB): `thiscall(processor)`.
const THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
/// `BSThread::SetThreadPriority` (Xbox PDB): `thiscall(priority)`.
const THREAD_SET_PRIORITY: u32 = 0x00aa_6560;
/// The vtable stored in each thread (`010c1af0`; RTTI name
/// `.?AVBSParallelTaskThread@@`, read through its complete object locator).
const PARALLEL_TASK_THREAD_VTABLE: u32 = 0x010c_1af0;
/// `"Parallel Task Thread %i"`.
const THREAD_NAME_FORMAT: u32 = 0x010c_1ad4;
/// A `.data` table of words, one per parallel task thread: the number in
/// the thread's name and the processor it is given (index 0 holds 5 in the
/// exe's initial data).
const THREAD_PROCESSORS: u32 = 0x011a_f714;
/// The number of threads: the loop bound the code compares against
/// (`CMP [index],1; JNC`).
const THREAD_COUNT: u32 = 1;
/// Size of a `BSParallelTaskThread` on the PC (`0x5C`; the Xbox PDB's 0x6C
/// less the 0x10 the PC's `BSThread` is smaller).
const PARALLEL_TASK_THREAD_SIZE: u32 = 0x5c;
/// `BSParallelTaskThread::pManager` (Xbox PDB `+0x5C`), at `+0x4C` on the PC.
const THREAD_MANAGER: u32 = 0x4c;
/// The name buffer on the stack (`0x104` bytes).
const NAME_SIZE: u32 = 0x104;
/// The thread's stack size passed to `BSThread::Initialize`.
const THREAD_STACK_SIZE: u32 = 0x8000;
/// The priority passed to `BSThread::SetThreadPriority`.
const THREAD_PRIORITY: u32 = 0xf;

layout! {
    /// `BSParallelTaskManager` (Xbox PDB), 0x10 bytes; the PC constructor
    /// clears the same four fields at the same offsets.
    pub struct BSParallelTaskManager: 0x10 {
        /// `pThreads` (Xbox PDB): `BSParallelTaskThread**`.
        0x00 pThreads: u32,
        /// `pTaskFunc` (Xbox PDB).
        0x04 pTaskFunc: u32,
        /// `pData` (Xbox PDB).
        0x08 pData: u32,
        /// `bRunningTask` (Xbox PDB).
        0x0C bRunningTask: bool,
    }
}

// Translated from 00c44980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c44980 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `BSParallelTaskManager::BSParallelTaskManager` (Xbox PDB): clears the four
/// fields, allocates the thread pointer array (4 bytes) and, for each of the
/// one thread, allocates a `BSParallelTaskThread` (0x5C bytes; when the
/// allocation succeeds: the `BSLinearTaskThread` constructor `008c76a0`, its
/// vtable and `pManager = this`; null otherwise), stores it, formats its name
/// `"Parallel Task Thread %i"` with the thread's word of the table at
/// `011af714` into a 0x104-byte stack buffer, and starts it:
/// `BSThread::Initialize(0x8000, name, 0)`, `SetName(name)`,
/// `SetThreadProcessor(table word)`, `SetThreadPriority(0xF)`, each on the
/// pointer read back from the array. Returns `this`.
///
/// The code also tests the constant 1 (`MOV ECX,1; TEST ECX,ECX; JZ`) before
/// allocating the array; it is never zero, so that jump to the end is never
/// taken.
pub fn bs_parallel_task_manager_constructor(
    e: &mut Engine,
    this: Ptr<BSParallelTaskManager>,
) -> Ptr<BSParallelTaskManager> {
    e.set(this, BSParallelTaskManager::pThreads, 0);
    e.set(this, BSParallelTaskManager::pTaskFunc, 0);
    e.set(this, BSParallelTaskManager::pData, 0);
    e.set(this, BSParallelTaskManager::bRunningTask, false);
    let threads = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    e.set(this, BSParallelTaskManager::pThreads, threads);
    let mut index = 0u32;
    while index < THREAD_COUNT {
        let block = e
            .call(
                MEMORY_ALLOCATE,
                &args![MEMORY_MANAGER, PARALLEL_TASK_THREAD_SIZE],
            )
            .u32();
        let thread = if block == 0 {
            0
        } else {
            e.call(LINEAR_TASK_THREAD_CONSTRUCT, &args![block]);
            e.mem.set_u32(block, PARALLEL_TASK_THREAD_VTABLE);
            e.mem.set_u32(block + THREAD_MANAGER, this.addr());
            block
        };
        let threads = e.get(this, BSParallelTaskManager::pThreads);
        e.mem
            .set_u32(threads.wrapping_add(index.wrapping_mul(4)), thread);
        let table_word = THREAD_PROCESSORS.wrapping_add(index.wrapping_mul(4));
        e.with_stack(NAME_SIZE, |e, name| {
            let number = e.mem.u32(table_word);
            e.call(
                SPRINTF_S,
                &args![name, NAME_SIZE, THREAD_NAME_FORMAT, number],
            );
            let thread = thread_at(e, this, index);
            e.call(
                THREAD_INITIALIZE,
                &args![thread, THREAD_STACK_SIZE, name, 0u32],
            );
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_NAME, &args![thread, name]);
            let processor = e.mem.u32(table_word);
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_PROCESSOR, &args![thread, processor]);
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_PRIORITY, &args![thread, THREAD_PRIORITY]);
        });
        index += 1;
    }
    this
}

/// `this->pThreads[index]`, read again from memory as the code does before
/// each call.
fn thread_at(e: &mut Engine, this: Ptr<BSParallelTaskManager>, index: u32) -> u32 {
    let threads = e.get(this, BSParallelTaskManager::pThreads);
    e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c44980,
        bs_parallel_task_manager_constructor(
            Ptr<BSParallelTaskManager>,
        ) -> Ptr<BSParallelTaskManager>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    const DOUBLED: [u32; 6] = [
        LINEAR_TASK_THREAD_CONSTRUCT,
        SPRINTF_S,
        THREAD_INITIALIZE,
        THREAD_SET_NAME,
        THREAD_SET_PROCESSOR,
        THREAD_SET_PRIORITY,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in DOUBLED {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(0x011a_f000, 0x1000);
        e.mem.set_u32(THREAD_PROCESSORS, 5);
        e
    }

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_makes_and_starts_one_thread() {
        let mut e = engine();
        let manager: Ptr<BSParallelTaskManager> = e.new_object();
        e.set(manager, BSParallelTaskManager::pTaskFunc, 0x1234);
        e.set(manager, BSParallelTaskManager::pData, 0x5678);
        e.set(manager, BSParallelTaskManager::bRunningTask, true);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_4980, &args![manager]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, manager.addr());
        assert_eq!(e.get(manager, BSParallelTaskManager::pTaskFunc), 0);
        assert_eq!(e.get(manager, BSParallelTaskManager::pData), 0);
        assert!(!e.get(manager, BSParallelTaskManager::bRunningTask));
        assert_eq!(
            calls(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4], vec![MEMORY_MANAGER, 0x5c]]
        );
        let threads = e.get(manager, BSParallelTaskManager::pThreads);
        assert_ne!(threads, 0);
        let thread = e.mem.u32(threads);
        assert_ne!(thread, 0);
        assert_eq!(
            calls(&log, LINEAR_TASK_THREAD_CONSTRUCT),
            vec![vec![thread]]
        );
        assert_eq!(e.mem.u32(thread), PARALLEL_TASK_THREAD_VTABLE);
        assert_eq!(e.mem.u32(thread + 0x4c), manager.addr());
        let format = &calls(&log, SPRINTF_S)[0];
        let name = format[0];
        assert_eq!(&format[1..], &[0x104, THREAD_NAME_FORMAT, 5]);
        assert_eq!(
            calls(&log, THREAD_INITIALIZE),
            vec![vec![thread, 0x8000, name, 0]]
        );
        assert_eq!(calls(&log, THREAD_SET_NAME), vec![vec![thread, name]]);
        assert_eq!(calls(&log, THREAD_SET_PROCESSOR), vec![vec![thread, 5]]);
        assert_eq!(calls(&log, THREAD_SET_PRIORITY), vec![vec![thread, 0xf]]);
    }

    #[test]
    fn constructor_stores_null_when_the_thread_allocation_fails() {
        let mut e = engine();
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, 0xdead_beef);
        e.register_double(MEMORY_ALLOCATE, move |_, a| Ret {
            eax: if a[1] == 4 { array } else { 0 },
            ..Ret::default()
        });
        let manager: Ptr<BSParallelTaskManager> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c4_4980, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(manager, BSParallelTaskManager::pThreads), array);
        assert_eq!(e.mem.u32(array), 0);
        assert!(calls(&log, LINEAR_TASK_THREAD_CONSTRUCT).is_empty());
        // The calls still go to the (null) pointer read back from the array.
        assert_eq!(calls(&log, THREAD_INITIALIZE)[0][0], 0);
        assert_eq!(calls(&log, THREAD_SET_PRIORITY), vec![vec![0, 0xf]]);
    }
}
