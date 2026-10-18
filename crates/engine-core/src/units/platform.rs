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

// ---- Files ----------------------------------------------------------------------

/// `BSFile` (Xbox PDB, `BSSystem/bsfile_xenon.cpp`, platform): the game's
/// file object, a Gamebryo `NiFile` with a read buffer (data +0x20, size
/// +0x10, bytes held +0x14, cursor +0x18, `FILE*` +0x24, "ready" byte
/// +0x2C) and two copies of its position (+0x38, +0x150; `004720a0`,
/// `00472380` read them). Its size, 0x158 bytes, and its vtable.
pub const BSFILE_SIZE: u32 = 0x158;
pub const BSFILE_VTABLE: u32 = 0x010a_4764;
const BSFILE_READY: u32 = 0x2c;
const BSFILE_POSITION: u32 = 0x38;
const BSFILE_POSITION_COPY: u32 = 0x150;

/// A file the host supplies in memory, behind a `BSFile`: its name, its
/// size, and the byte ranges supplied, at their offsets in the real file.
/// Only what the host has supplied can be read; a read of anything else
/// stops (the engine panics with the range), it never reads zeros.
#[derive(Debug, Clone)]
pub struct MemoryFile {
    pub name: String,
    pub size: u32,
    pieces: std::collections::BTreeMap<u32, Vec<u8>>,
}

impl MemoryFile {
    /// The `n` bytes at `at`, when supplied.
    fn bytes(&self, at: u32, n: u32) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(n as usize);
        let mut pos = at;
        let end = at + n;
        while pos < end {
            let (&start, piece) = self.pieces.range(..=pos).next_back()?;
            let offset = (pos - start) as usize;
            if offset >= piece.len() {
                return None;
            }
            let take = (piece.len() - offset).min((end - pos) as usize);
            out.extend_from_slice(&piece[offset..offset + take]);
            pos += take as u32;
        }
        Some(out)
    }
}

/// Opens an in-memory file (`name`, `size` bytes in the real file) as a
/// `BSFile` and answers the object. Stands in for what the `BSFile`
/// constructor (`00b00260`) and its open (vtable +0x20, `00aff300`) leave
/// after a buffered read: an object with the game's `BSFile` vtable, ready
/// (+0x2C), at position 0. Its reads and seeks go through [`bsfile_read_f`]
/// and [`bsfile_seek`]; supply its bytes with [`supply_file_bytes`].
pub fn open_memory_file(e: &mut Engine, name: &str, size: u32) -> u32 {
    let file = e.mem.alloc(BSFILE_SIZE);
    e.mem.set_u32(file, BSFILE_VTABLE);
    e.mem.set_u8(file + BSFILE_READY, 1);
    e.mem.set_u32(file + BSFILE_POSITION, 0);
    e.mem.set_u32(file + BSFILE_POSITION_COPY, 0);
    e.kernel.files.insert(
        file,
        MemoryFile {
            name: name.to_string(),
            size,
            pieces: std::collections::BTreeMap::new(),
        },
    );
    file
}

/// Supplies `bytes` of the in-memory file behind `file`, at `offset`.
pub fn supply_file_bytes(e: &mut Engine, file: u32, offset: u32, bytes: &[u8]) {
    let f = e
        .kernel
        .files
        .get_mut(&file)
        .unwrap_or_else(|| panic!("{file:08x}: not an in-memory file"));
    f.pieces.insert(offset, bytes.to_vec());
}

fn file_position(e: &Engine, file: u32) -> u32 {
    e.mem.u32(file + BSFILE_POSITION_COPY)
}

fn set_file_position(e: &mut Engine, file: u32, position: u32) {
    e.mem.set_u32(file + BSFILE_POSITION, position);
    e.mem.set_u32(file + BSFILE_POSITION_COPY, position);
}

// Platform: replaces 00b00650 (BSFile::ReadF (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Reads `size` bytes at the file's position into `buffer` and moves the
/// position on (+0x150, and +0x38 with it), answering how many were read:
/// fewer at the end of the file. The game's reads its `NiFile` buffer and
/// refills it with `fread` (`00aa1750`); this one reads the in-memory file.
/// A range the host hasn't supplied stops with the file's name and range.
pub fn bsfile_read_f(e: &mut Engine, this: Ptr, buffer: Ptr, size: u32) -> u32 {
    let file = this.addr();
    let position = file_position(e, file);
    let f = e
        .kernel
        .files
        .get(&file)
        .unwrap_or_else(|| panic!("BSFile {file:08x}: not an in-memory file"));
    let n = size.min(f.size.saturating_sub(position));
    let bytes = f.bytes(position, n).unwrap_or_else(|| {
        panic!(
            "BSFile {}: bytes {position:#x}..{:#x} not supplied",
            f.name,
            position + n
        )
    });
    e.mem.write(buffer.addr(), &bytes);
    set_file_position(e, file, position + n);
    n
}

// Platform: replaces 00b00590 (BSFile seek, vtable +0x14, FalloutNV.exe 1.4.0.525)
/// Moves the file's position: `whence` 0 to `offset`, 1 by `offset`, 2 to
/// the size less `offset` (as the game's computes it), else nowhere. The
/// game's seeks its `NiFile` (`00aa16d0`) when the position changes; this
/// one only sets it.
pub fn bsfile_seek(e: &mut Engine, this: Ptr, offset: i32, whence: i32) {
    let file = this.addr();
    let position = file_position(e, file);
    let size = e.kernel.files.get(&file).map_or(0, |f| f.size);
    let target = match whence {
        0 => offset as u32,
        1 => position.wrapping_add(offset as u32),
        2 => size.wrapping_sub(offset as u32),
        _ => position,
    };
    if target != position {
        set_file_position(e, file, target);
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b00650, bsfile_read_f(Ptr, Ptr, u32) -> u32),
        entry!(0x00b00590, bsfile_seek(Ptr, i32, i32)),
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

// Platform: replaces KERNEL32.DLL InitializeCriticalSectionAndSpinCount
// (import, slot 00fdf130): one thread, nothing to set up; nonzero is success.
fn initialize_critical_section_and_spin_count(_e: &mut Engine, _section: Ptr, _spin: u32) -> i32 {
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

// Kernel objects and virtual memory. Handles are small integers (4, 8, ...,
// never reused) into the table `Engine::kernel`; events and semaphores keep
// their state as Windows does. One thread runs the game, so a wait that
// would block forever (nothing else can signal the object) and a thread
// that would start running stop the engine with a message naming the
// import and handle, as a call to an untranslated function does: how game
// threads run is the lead's decision (docs/ENGINE_PORT_PLAN.md).

/// `WaitForSingleObject` timeout and results.
const INFINITE: u32 = 0xffff_ffff;
const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x102;
const WAIT_FAILED: u32 = 0xffff_ffff;
/// `GetCurrentThread`'s pseudo handle.
pub const CURRENT_THREAD: u32 = 0xffff_fffe;
/// `CreateThread` flag.
const CREATE_SUSPENDED: u32 = 4;
/// `VirtualAlloc`/`VirtualFree` types and the one protection the game
/// passes.
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_DECOMMIT: u32 = 0x4000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 4;
const PAGE_BYTES: u32 = 0x1000;
/// Reservations start on 64 KiB boundaries, as on Windows.
pub const ALLOCATION_GRANULARITY: u32 = 0x1_0000;
/// End of FalloutNV.exe's image (`.reloc` ends at `014089ff`): a reserve
/// at a fixed address inside the image fails, as on Windows.
const IMAGE_END: u32 = 0x0140_9000;

/// A kernel object behind a handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelObject {
    Event {
        manual_reset: bool,
        signalled: bool,
    },
    Semaphore {
        count: i32,
        maximum: i32,
    },
    /// A thread made by `CreateThread`. It never runs (see above); it
    /// exists so its handle can be resumed, prioritized and closed.
    Thread {
        start: u32,
        parameter: u32,
        id: u32,
        suspend_count: u32,
    },
}

/// A `VirtualAlloc` reservation: its size and which pages are committed.
/// `block` is the [`crate::Mem`] heap block behind a reservation the
/// system placed (address 0), 0 for one at an address the caller chose.
#[derive(Debug, Clone)]
pub struct Region {
    size: u32,
    block: u32,
    committed: Vec<bool>,
}

/// The handle table and the `VirtualAlloc` reservations (`Engine::kernel`).
#[derive(Debug, Default)]
pub struct Kernel {
    objects: std::collections::BTreeMap<u32, KernelObject>,
    last_handle: u32,
    threads_created: u32,
    /// The reservations by base address (public for the scrap heap's tests in
    /// `engine-bs`).
    pub regions: std::collections::BTreeMap<u32, Region>,
    /// The in-memory files behind `BSFile`s ([`open_memory_file`]), by the
    /// `BSFile`'s address.
    files: std::collections::BTreeMap<u32, MemoryFile>,
}

impl Kernel {
    /// The object behind `handle`, if it is open.
    pub fn object(&self, handle: u32) -> Option<&KernelObject> {
        self.objects.get(&handle)
    }

    fn insert(&mut self, object: KernelObject) -> u32 {
        self.last_handle += 4;
        self.objects.insert(self.last_handle, object);
        self.last_handle
    }

    /// Base of the reservation holding all of `[start, end)`.
    fn region_containing(&self, start: u64, end: u64) -> Option<u32> {
        let (&base, region) = self.regions.range(..=start as u32).next_back()?;
        (end <= base as u64 + region.size as u64).then_some(base)
    }

    /// Whether `[start, end)` overlaps a reservation.
    fn overlaps_region(&self, start: u64, end: u64) -> bool {
        self.regions
            .iter()
            .any(|(&b, r)| start < b as u64 + r.size as u64 && (b as u64) < end)
    }
}

fn page_up(v: u64) -> u64 {
    v.div_ceil(PAGE_BYTES as u64) * PAGE_BYTES as u64
}

/// Reserves `size` bytes at `address` (0: anywhere). Returns the base.
fn reserve(e: &mut Engine, address: u32, size: u32) -> Option<u32> {
    let (base, len, block) = if address == 0 {
        // From the Mem heap, aligned within an oversized block.
        let len = page_up(size as u64);
        if len > (crate::mem::HEAP_END - crate::mem::HEAP_BASE) as u64 {
            return None;
        }
        let len = len as u32;
        let block = e.mem.alloc(len + ALLOCATION_GRANULARITY);
        let base = block.next_multiple_of(ALLOCATION_GRANULARITY);
        (base, len, block)
    } else {
        let base = address & !(ALLOCATION_GRANULARITY - 1);
        let end = page_up(address as u64 + size as u64);
        let overlaps = |lo: u32, hi: u32| (base as u64) < hi as u64 && end > lo as u64;
        if base < ALLOCATION_GRANULARITY
            || end > 1 << 32
            || overlaps(crate::exe::IMAGE_BASE, IMAGE_END)
            || overlaps(crate::mem::HEAP_BASE, crate::mem::HEAP_END)
            || e.kernel.overlaps_region(base as u64, end)
            || (base as u64..end)
                .step_by(PAGE_BYTES as usize)
                .any(|p| e.mem.is_mapped(p as u32))
        {
            return None;
        }
        (base, (end - base as u64) as u32, 0)
    };
    // Reserved pages are not addressable until committed.
    e.mem.unmap(base, len);
    e.kernel.regions.insert(
        base,
        Region {
            size: len,
            block,
            committed: vec![false; (len / PAGE_BYTES) as usize],
        },
    );
    Some(base)
}

/// Commits the pages of `[start, start + len)` in the reservation at
/// `base`; newly committed pages read as zero.
fn commit(e: &mut Engine, base: u32, start: u32, len: u32) {
    for page in (start..start.wrapping_add(len)).step_by(PAGE_BYTES as usize) {
        let i = ((page - base) / PAGE_BYTES) as usize;
        let region = e.kernel.regions.get_mut(&base).expect("reservation");
        if !region.committed[i] {
            region.committed[i] = true;
            e.mem.map(page, PAGE_BYTES);
            e.mem.write(page, &[0; PAGE_BYTES as usize]);
        }
    }
}

/// Decommits the pages of `[start, start + len)` in the reservation at
/// `base`: they are no longer addressable.
fn decommit(e: &mut Engine, base: u32, start: u32, len: u32) {
    for page in (start..start.wrapping_add(len)).step_by(PAGE_BYTES as usize) {
        let i = ((page - base) / PAGE_BYTES) as usize;
        let region = e.kernel.regions.get_mut(&base).expect("reservation");
        if region.committed[i] {
            region.committed[i] = false;
            e.mem.unmap(page, PAGE_BYTES);
        }
    }
}

// Platform: replaces KERNEL32.DLL VirtualAlloc (import)
/// `VirtualAlloc(address, size, type, protect)` with the types the game
/// passes (`ScrapHeapManager`, `MemoryPool`, the CRT small-block heap):
/// `MEM_RESERVE` at address 0 takes a 64 KiB-aligned range from the
/// [`crate::Mem`] heap (as [`memory_manager_allocate`] does); at a chosen
/// address it succeeds only where nothing is (not the image, the heap
/// region, another reservation or mapped memory) and otherwise returns 0,
/// which the memory pools' address search relies on. `MEM_COMMIT` commits
/// whole pages inside a reservation (0 if the range is not inside one), or
/// reserves and commits at address 0. Reserved pages that are not
/// committed are unmapped, so touching them faults as on Windows.
/// Returns the (rounded) start, 0 on failure.
fn virtual_alloc(e: &mut Engine, address: u32, size: u32, kind: u32, protect: u32) -> u32 {
    if kind == 0 || kind & !(MEM_COMMIT | MEM_RESERVE) != 0 || protect != PAGE_READWRITE {
        panic!(
            "VirtualAlloc({address:08x}, {size:#x}, {kind:#x}, {protect:#x}): only \
             MEM_RESERVE and MEM_COMMIT with PAGE_READWRITE are provided (platform, units/platform.rs)"
        );
    }
    if size == 0 {
        return 0;
    }
    if kind & MEM_RESERVE != 0 || address == 0 {
        let Some(base) = reserve(e, address, size) else {
            return 0;
        };
        if kind & MEM_COMMIT != 0 {
            let len = e.kernel.regions[&base].size;
            commit(e, base, base, len);
        }
        return base;
    }
    let start = address & !(PAGE_BYTES - 1);
    let end = page_up(address as u64 + size as u64);
    match e.kernel.region_containing(start as u64, end) {
        Some(base) => {
            commit(e, base, start, (end - start as u64) as u32);
            start
        }
        None => 0,
    }
}

// Platform: replaces KERNEL32.DLL VirtualFree (import)
/// `VirtualFree(address, size, type)`: `MEM_DECOMMIT` decommits the whole
/// pages of the range (size 0 at a reservation's start: all of it), which
/// must lie in one reservation; `MEM_RELEASE` (size 0, the reservation's
/// start) frees the reservation and its heap block. Returns 1, or 0 for a
/// range Windows would reject.
fn virtual_free(e: &mut Engine, address: u32, size: u32, kind: u32) -> i32 {
    match kind {
        MEM_RELEASE => {
            if size != 0 || !e.kernel.regions.contains_key(&address) {
                return 0;
            }
            let len = e.kernel.regions[&address].size;
            decommit(e, address, address, len);
            let region = e.kernel.regions.remove(&address).expect("reservation");
            e.mem.free(region.block);
            1
        }
        MEM_DECOMMIT => {
            let (start, end) = if size == 0 {
                match e.kernel.regions.get(&address) {
                    Some(r) => (address, address as u64 + r.size as u64),
                    None => return 0,
                }
            } else {
                (
                    address & !(PAGE_BYTES - 1),
                    page_up(address as u64 + size as u64),
                )
            };
            match e.kernel.region_containing(start as u64, end) {
                Some(base) => {
                    decommit(e, base, start, (end - start as u64) as u32);
                    1
                }
                None => 0,
            }
        }
        _ => panic!(
            "VirtualFree({address:08x}, {size:#x}, {kind:#x}): only MEM_DECOMMIT and \
             MEM_RELEASE are provided (platform, units/platform.rs)"
        ),
    }
}

/// Named kernel objects (shared between processes) are not provided.
fn unnamed(call: &str, name: Ptr) {
    if !name.is_null() {
        panic!(
            "{call}: named object at {:08x}; named kernel objects are not provided \
             (platform, units/platform.rs)",
            name.addr()
        );
    }
}

/// Stops the engine: a wait or a thread start the single game thread
/// cannot carry out.
fn stop_one_thread(what: String) -> ! {
    panic!(
        "{what}: one thread runs the game, nothing else can run or signal it \
         (KERNEL32.DLL import, platform; docs/ENGINE_PORT_PLAN.md)"
    )
}

// Platform: replaces KERNEL32.DLL CreateEventA (import)
/// `CreateEventA(security, manual reset, initial state, name)`: a new
/// event's handle. Named events stop the engine.
fn create_event_a(
    e: &mut Engine,
    _security: u32,
    manual_reset: i32,
    initial: i32,
    name: Ptr,
) -> u32 {
    unnamed("CreateEventA", name);
    e.kernel.insert(KernelObject::Event {
        manual_reset: manual_reset != 0,
        signalled: initial != 0,
    })
}

/// Sets an event's state; 0 if `handle` is not an open event.
fn set_event_state(e: &mut Engine, handle: u32, state: bool) -> i32 {
    match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Event { signalled, .. }) => {
            *signalled = state;
            1
        }
        _ => 0,
    }
}

// Platform: replaces KERNEL32.DLL SetEvent (import)
fn set_event(e: &mut Engine, handle: u32) -> i32 {
    set_event_state(e, handle, true)
}

// Platform: replaces KERNEL32.DLL ResetEvent (import)
fn reset_event(e: &mut Engine, handle: u32) -> i32 {
    set_event_state(e, handle, false)
}

// Platform: replaces KERNEL32.DLL CreateSemaphoreA (import)
/// `CreateSemaphoreA(security, initial count, maximum count, name)`: a new
/// semaphore's handle, or 0 for counts Windows rejects (maximum not
/// positive, initial outside 0..=maximum). Named semaphores stop the
/// engine.
fn create_semaphore_a(
    e: &mut Engine,
    _security: u32,
    initial: i32,
    maximum: i32,
    name: Ptr,
) -> u32 {
    unnamed("CreateSemaphoreA", name);
    if maximum <= 0 || initial < 0 || initial > maximum {
        return 0;
    }
    e.kernel.insert(KernelObject::Semaphore {
        count: initial,
        maximum,
    })
}

// Platform: replaces KERNEL32.DLL ReleaseSemaphore (import)
/// `ReleaseSemaphore(handle, release count, previous count out)`: adds to
/// the count and stores the previous one (if the pointer is not null).
/// Returns 0 without a change if `handle` is not a semaphore, the release
/// count is not positive or the count would pass the maximum.
fn release_semaphore(e: &mut Engine, handle: u32, release: i32, previous: Ptr) -> i32 {
    let old = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Semaphore { count, maximum })
            if release > 0 && release <= *maximum - *count =>
        {
            let old = *count;
            *count += release;
            old
        }
        _ => return 0,
    };
    if !previous.is_null() {
        e.mem.set_i32(previous.addr(), old);
    }
    1
}

// Platform: replaces KERNEL32.DLL WaitForSingleObject (import)
/// `WaitForSingleObject(handle, timeout)`: a signalled object is taken at
/// once (`WAIT_OBJECT_0`), resetting an auto-reset event or counting a
/// semaphore down. An object that is not signalled stays so (no other
/// thread runs): with a finite timeout the wait times out at once
/// (`WAIT_TIMEOUT`; the clock is not advanced), with `INFINITE` it would
/// block forever and stops the engine. Threads never run, so a thread
/// handle is never signalled. An unknown handle gives `WAIT_FAILED`.
fn wait_for_single_object(e: &mut Engine, handle: u32, timeout: u32) -> u32 {
    let blocked = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Event {
            manual_reset,
            signalled,
        }) => {
            if *signalled {
                if !*manual_reset {
                    *signalled = false;
                }
                return WAIT_OBJECT_0;
            }
            format!(
                "{} event not signalled",
                if *manual_reset {
                    "manual-reset"
                } else {
                    "auto-reset"
                }
            )
        }
        Some(KernelObject::Semaphore { count, maximum }) => {
            if *count > 0 {
                *count -= 1;
                return WAIT_OBJECT_0;
            }
            format!("semaphore count 0 (maximum {maximum})")
        }
        Some(KernelObject::Thread { start, .. }) => {
            format!("thread (start {start:08x}) that never runs")
        }
        None if handle == CURRENT_THREAD => "the current thread".to_string(),
        None => return WAIT_FAILED,
    };
    if timeout != INFINITE {
        return WAIT_TIMEOUT;
    }
    stop_one_thread(format!(
        "WaitForSingleObject(handle {handle:#x}, INFINITE) would block forever on a {blocked}"
    ))
}

// Platform: replaces KERNEL32.DLL CloseHandle (import)
/// Closes a handle from this table; 0 for one that is not open.
fn close_handle(e: &mut Engine, handle: u32) -> i32 {
    e.kernel.objects.remove(&handle).is_some() as i32
}

// Platform: replaces KERNEL32.DLL GetCurrentThread (import)
fn get_current_thread(_e: &mut Engine) -> u32 {
    CURRENT_THREAD
}

// Platform: replaces KERNEL32.DLL CreateThread (import)
/// `CreateThread(security, stack size, start, parameter, flags, id out)`:
/// a suspended thread is recorded (handle returned, a new thread id stored
/// through the last argument if not null). A thread that would start
/// running at once (no `CREATE_SUSPENDED`) stops the engine.
fn create_thread(
    e: &mut Engine,
    _security: u32,
    _stack_size: u32,
    start: u32,
    parameter: u32,
    flags: u32,
    id_out: Ptr,
) -> u32 {
    if flags & CREATE_SUSPENDED == 0 {
        stop_one_thread(format!(
            "CreateThread(start {start:08x}, parameter {parameter:08x}, flags {flags:#x}) \
             would start the thread running"
        ));
    }
    e.kernel.threads_created += 1;
    let id = MAIN_THREAD_ID + e.kernel.threads_created;
    let handle = e.kernel.insert(KernelObject::Thread {
        start,
        parameter,
        id,
        suspend_count: 1,
    });
    if !id_out.is_null() {
        e.mem.set_u32(id_out.addr(), id);
    }
    handle
}

// Platform: replaces KERNEL32.DLL ResumeThread (import)
/// Lowers a thread's suspend count and returns the previous one. Bringing
/// it to 0 would start the thread running, which stops the engine. The
/// current thread runs (previous count 0); an unknown handle gives -1.
fn resume_thread(e: &mut Engine, handle: u32) -> u32 {
    if handle == CURRENT_THREAD {
        return 0;
    }
    match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Thread {
            start,
            parameter,
            suspend_count,
            ..
        }) => {
            let previous = *suspend_count;
            if previous == 1 {
                stop_one_thread(format!(
                    "ResumeThread(handle {handle:#x}) would start the thread running \
                     (start {start:08x}, parameter {parameter:08x})"
                ));
            }
            *suspend_count = previous.saturating_sub(1);
            previous
        }
        _ => 0xffff_ffff,
    }
}

// Platform: replaces KERNEL32.DLL SetThreadPriority (import)
/// Accepts any priority for a thread handle or the current thread (the
/// threads do not run, so it has no effect); 0 for another handle.
fn set_thread_priority(e: &mut Engine, handle: u32, _priority: i32) -> i32 {
    (handle == CURRENT_THREAD
        || matches!(
            e.kernel.objects.get(&handle),
            Some(KernelObject::Thread { .. })
        )) as i32
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
            "InitializeCriticalSectionAndSpinCount",
            entry!(0, initialize_critical_section_and_spin_count(Ptr, u32) -> i32).1,
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
        (
            k,
            "VirtualAlloc",
            entry!(0, virtual_alloc(u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "VirtualFree",
            entry!(0, virtual_free(u32, u32, u32) -> i32).1,
        ),
        (
            k,
            "CreateEventA",
            entry!(0, create_event_a(u32, i32, i32, Ptr) -> u32).1,
        ),
        (k, "SetEvent", entry!(0, set_event(u32) -> i32).1),
        (k, "ResetEvent", entry!(0, reset_event(u32) -> i32).1),
        (
            k,
            "CreateSemaphoreA",
            entry!(0, create_semaphore_a(u32, i32, i32, Ptr) -> u32).1,
        ),
        (
            k,
            "ReleaseSemaphore",
            entry!(0, release_semaphore(u32, i32, Ptr) -> i32).1,
        ),
        (
            k,
            "WaitForSingleObject",
            entry!(0, wait_for_single_object(u32, u32) -> u32).1,
        ),
        (k, "CloseHandle", entry!(0, close_handle(u32) -> i32).1),
        (
            k,
            "GetCurrentThread",
            entry!(0, get_current_thread() -> u32).1,
        ),
        (
            k,
            "CreateThread",
            entry!(0, create_thread(u32, u32, u32, u32, u32, Ptr) -> u32).1,
        ),
        (k, "ResumeThread", entry!(0, resume_thread(u32) -> u32).1),
        (
            k,
            "SetThreadPriority",
            entry!(0, set_thread_priority(u32, i32) -> i32).1,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_memory_file_reads_and_seeks_what_was_supplied() {
        let mut e = Engine::new();
        // The vtable is in the exe's data; its seek slot (+0x14).
        e.put_vtable(BSFILE_VTABLE, &[0, 0, 0, 0, 0, 0x00b0_0590]);
        let f = open_memory_file(&mut e, "Test.esm", 0x100);
        assert_eq!(e.mem.u32(f), BSFILE_VTABLE);
        assert_eq!(e.mem.u8(f + 0x2c), 1);
        supply_file_bytes(&mut e, f, 0x10, b"abcdefgh");
        supply_file_bytes(&mut e, f, 0x18, b"ijkl");
        let buffer = e.mem.alloc(0x20);
        // Seek through the vtable's slot +0x14, as TESFile does.
        e.vcall(f, 0x14, &[0x12, 0]);
        assert_eq!(e.mem.u32(f + 0x150), 0x12);
        // Across two supplied pieces.
        assert_eq!(e.call(0x00b00650, &[f, buffer, 8]).u32(), 8);
        assert_eq!(e.mem.bytes(buffer, 8), b"cdefghij");
        assert_eq!(e.mem.u32(f + 0x150), 0x1a);
        assert_eq!(e.mem.u32(f + 0x38), 0x1a);
        // Relative seek.
        e.vcall(f, 0x14, &[(-2i32) as u32, 1]);
        assert_eq!(e.mem.u32(f + 0x150), 0x18);
        // At the end of the file, fewer.
        supply_file_bytes(&mut e, f, 0xfc, b"wxyz");
        e.vcall(f, 0x14, &[0xfe, 0]);
        assert_eq!(e.call(0x00b00650, &[f, buffer, 8]).u32(), 2);
        assert_eq!(e.mem.bytes(buffer, 2), b"yz");
    }

    #[test]
    #[should_panic(expected = "not supplied")]
    fn bytes_not_supplied_stop() {
        let mut e = Engine::new();
        e.put_vtable(BSFILE_VTABLE, &[0, 0, 0, 0, 0, 0x00b0_0590]);
        let f = open_memory_file(&mut e, "Test.esm", 0x100);
        supply_file_bytes(&mut e, f, 0x10, b"abcd");
        let buffer = e.mem.alloc(8);
        e.vcall(f, 0x14, &[0x12, 0]);
        e.call(0x00b00650, &[f, buffer, 4]);
    }

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

    // Import slots of FalloutNV.exe 1.4.0.525 (Ghidra `PTR_<name>_<slot>`).
    const CLOSE_HANDLE: u32 = 0x00fd_f07c;
    const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
    const CREATE_EVENT_A: u32 = 0x00fd_f0cc;
    const SET_EVENT: u32 = 0x00fd_f0d0;
    const VIRTUAL_FREE: u32 = 0x00fd_f10c;
    const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
    const SET_THREAD_PRIORITY: u32 = 0x00fd_f11c;
    const GET_CURRENT_THREAD: u32 = 0x00fd_f120;
    const CREATE_THREAD: u32 = 0x00fd_f124;
    const RESUME_THREAD: u32 = 0x00fd_f158;
    const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
    const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
    const RESET_EVENT: u32 = 0x00fd_f1cc;

    /// An engine with the kernel-object and virtual-memory stand-ins
    /// registered at their import slots, as `Engine::with_exe` does.
    fn kernel_engine() -> Engine {
        let mut e = Engine::new();
        let slots = [
            (CLOSE_HANDLE, "CloseHandle"),
            (CREATE_SEMAPHORE_A, "CreateSemaphoreA"),
            (CREATE_EVENT_A, "CreateEventA"),
            (SET_EVENT, "SetEvent"),
            (VIRTUAL_FREE, "VirtualFree"),
            (VIRTUAL_ALLOC, "VirtualAlloc"),
            (SET_THREAD_PRIORITY, "SetThreadPriority"),
            (GET_CURRENT_THREAD, "GetCurrentThread"),
            (CREATE_THREAD, "CreateThread"),
            (RESUME_THREAD, "ResumeThread"),
            (RELEASE_SEMAPHORE, "ReleaseSemaphore"),
            (WAIT_FOR_SINGLE_OBJECT, "WaitForSingleObject"),
            (RESET_EVENT, "ResetEvent"),
        ];
        let imports: Vec<crate::exe::Import> = slots
            .iter()
            .map(|&(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            })
            .collect();
        assert_eq!(e.register_imports(&imports), slots.len());
        e
    }

    fn call(e: &mut Engine, slot: u32, a: &[u32]) -> u32 {
        e.call(slot, a).u32()
    }

    #[test]
    fn handles_are_small_distinct_and_closed_once() {
        let mut e = kernel_engine();
        let a = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        let b = call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        assert_eq!((a, b), (4, 8));
        assert_eq!(
            e.kernel.object(b),
            Some(&KernelObject::Semaphore {
                count: 0,
                maximum: 1
            })
        );
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[a]), 1);
        assert_eq!(e.kernel.object(a), None);
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[a]), 0, "already closed");
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[a, 0]), WAIT_FAILED);
        assert_eq!(call(&mut e, SET_EVENT, &[a]), 0);
        // Handles are not reused.
        assert_eq!(call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]), 12);
    }

    #[test]
    fn auto_reset_event_is_consumed_by_a_wait_manual_reset_is_not() {
        let mut e = kernel_engine();
        let auto = call(&mut e, CREATE_EVENT_A, &[0, 0, 1, 0]);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, INFINITE]), 0);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, 100]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut e, SET_EVENT, &[auto]), 1);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, 0]), 0);

        let manual = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut e, SET_EVENT, &[manual]), 1);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, INFINITE]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, INFINITE]), 0);
        assert_eq!(call(&mut e, RESET_EVENT, &[manual]), 1);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, 0]),
            WAIT_TIMEOUT
        );
        // Not an event.
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 1, 1, 0]);
        assert_eq!(call(&mut e, RESET_EVENT, &[s]), 0);
    }

    #[test]
    fn semaphore_counts_waits_and_releases() {
        let mut e = kernel_engine();
        e.mem.map(0x1000, 0x1000);
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 1, 2, 0]);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, INFINITE]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), WAIT_TIMEOUT);
        e.mem.set_i32(0x1000, -5);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 2, 0x1000]), 1);
        assert_eq!(e.mem.i32(0x1000), 0, "previous count");
        // Past the maximum, or a count that is not positive: no change.
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 1, 0]), 0);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 0, 0]), 0);
        assert_eq!(
            e.kernel.object(s),
            Some(&KernelObject::Semaphore {
                count: 2,
                maximum: 2
            })
        );
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), WAIT_TIMEOUT);
        // Counts Windows rejects.
        assert_eq!(call(&mut e, CREATE_SEMAPHORE_A, &[0, 3, 2, 0]), 0);
        assert_eq!(call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 0, 0]), 0);
        // Not a semaphore.
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[ev, 1, 0]), 0);
    }

    #[test]
    #[should_panic(
        expected = "WaitForSingleObject(handle 0x4, INFINITE) would block forever on a semaphore count 0 (maximum 1)"
    )]
    fn infinite_wait_on_an_empty_semaphore_stops() {
        let mut e = kernel_engine();
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, INFINITE]);
    }

    #[test]
    #[should_panic(
        expected = "WaitForSingleObject(handle 0x4, INFINITE) would block forever on a manual-reset event not signalled"
    )]
    fn infinite_wait_on_an_unsignalled_event_stops() {
        let mut e = kernel_engine();
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[ev, INFINITE]);
    }

    #[test]
    #[should_panic(expected = "named kernel objects are not provided")]
    fn named_events_stop() {
        let mut e = kernel_engine();
        call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0x0100_0000]);
    }

    #[test]
    fn suspended_thread_is_recorded_prioritized_and_closed() {
        let mut e = kernel_engine();
        e.mem.map(0x1000, 0x1000);
        assert_eq!(call(&mut e, GET_CURRENT_THREAD, &[]), CURRENT_THREAD);
        let t = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0x1_0000, 0x00aa_64d0, 0x4444, CREATE_SUSPENDED, 0x1000],
        );
        assert_eq!(t, 4);
        assert_eq!(e.mem.u32(0x1000), MAIN_THREAD_ID + 1);
        assert_eq!(
            e.kernel.object(t),
            Some(&KernelObject::Thread {
                start: 0x00aa_64d0,
                parameter: 0x4444,
                id: MAIN_THREAD_ID + 1,
                suspend_count: 1
            })
        );
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[t, -1i32 as u32]), 1);
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[CURRENT_THREAD, 2]), 1);
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[0x99, 0]), 0);
        assert_eq!(call(&mut e, RESUME_THREAD, &[CURRENT_THREAD]), 0);
        assert_eq!(call(&mut e, RESUME_THREAD, &[0x99]), 0xffff_ffff);
        // A thread never runs, so it is never signalled.
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, 10]), WAIT_TIMEOUT);
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[t]), 1);
    }

    #[test]
    #[should_panic(expected = "ResumeThread(handle 0x4) would start the thread running \
                               (start 00aa64d0, parameter 00004444)")]
    fn resuming_a_thread_to_run_stops() {
        let mut e = kernel_engine();
        let t = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0, 0x00aa_64d0, 0x4444, CREATE_SUSPENDED, 0],
        );
        call(&mut e, RESUME_THREAD, &[t]);
    }

    #[test]
    #[should_panic(
        expected = "CreateThread(start 00aa64d0, parameter 00004444, flags 0x0) \
                               would start the thread running"
    )]
    fn creating_a_running_thread_stops() {
        let mut e = kernel_engine();
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00aa_64d0, 0x4444, 0, 0]);
    }

    #[test]
    fn reserve_commit_decommit_release_on_the_heap() {
        let mut e = kernel_engine();
        let base = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x80_0000, MEM_RESERVE, PAGE_READWRITE],
        );
        assert_ne!(base, 0);
        assert_eq!(base % ALLOCATION_GRANULARITY, 0);
        assert!(!e.mem.is_mapped(base), "reserved, not committed");
        // Commit rounds out to whole pages and returns the page start.
        let p = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base + 0x10, 0x1ff0, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(p, base);
        assert_eq!(e.mem.bytes(base, 0x2000), vec![0; 0x2000]);
        assert!(!e.mem.is_mapped(base + 0x2000));
        e.mem.set_u32(base + 0x1000, 7);
        // Committing again keeps the contents.
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base, 0x2000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(e.mem.u32(base + 0x1000), 7);
        // Outside the reservation: fails.
        let end = base + 0x80_0000;
        assert_eq!(
            call(
                &mut e,
                VIRTUAL_ALLOC,
                &[end - 0x1000, 0x2000, MEM_COMMIT, PAGE_READWRITE]
            ),
            0
        );
        // Decommit the tail page; recommitting gives zeros again.
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[base + 0x1000, 0x1000, MEM_DECOMMIT]),
            1
        );
        assert!(!e.mem.is_mapped(base + 0x1000));
        assert!(e.mem.is_mapped(base));
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base + 0x1000, 0x1000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(e.mem.u32(base + 0x1000), 0);
        // Release needs size 0 and the reservation's start.
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0x1000, MEM_RELEASE]), 0);
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[base + 0x1000, 0, MEM_RELEASE]),
            0
        );
        let block = e.kernel.regions[&base].block;
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 1);
        assert!(!e.mem.is_mapped(base));
        assert_eq!(e.mem.block_size(block), None, "heap block freed");
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 0);
    }

    #[test]
    fn commit_at_address_zero_reserves_and_commits() {
        let mut e = kernel_engine();
        let base = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x1800, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(base % ALLOCATION_GRANULARITY, 0);
        assert_eq!(e.mem.bytes(base, 0x2000), vec![0; 0x2000]);
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_DECOMMIT]), 1);
        assert!(!e.mem.is_mapped(base));
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 1);
    }

    #[test]
    fn reserve_at_a_fixed_address_only_where_nothing_is() {
        let mut e = kernel_engine();
        let reserve = |e: &mut Engine, at: u32| {
            call(
                e,
                VIRTUAL_ALLOC,
                &[at, 0x2_0000, MEM_RESERVE, PAGE_READWRITE],
            )
        };
        // The image, the heap region: fail (the memory pools then try the
        // next 16 MiB step).
        assert_eq!(reserve(&mut e, 0x0100_0000), 0);
        assert_eq!(reserve(&mut e, 0x2000_0000), 0);
        assert_eq!(reserve(&mut e, 0x0200_0000), 0x0200_0000);
        assert_eq!(reserve(&mut e, 0x0201_0000), 0, "overlaps a reservation");
        e.mem.map(0x0300_0000, 4);
        assert_eq!(reserve(&mut e, 0x0300_0000), 0, "mapped memory");
        let page = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0x0201_0000, 0x1000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(page, 0x0201_0000);
        assert!(e.mem.is_mapped(0x0201_0000));
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[0x0200_0000, 0, MEM_RELEASE]),
            1
        );
        assert!(!e.mem.is_mapped(0x0201_0000));
        assert_eq!(reserve(&mut e, 0x0200_0000), 0x0200_0000, "free again");
    }

    #[test]
    #[should_panic(expected = "only MEM_RESERVE and MEM_COMMIT with PAGE_READWRITE")]
    fn other_allocation_types_stop() {
        let mut e = kernel_engine();
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x1000, MEM_COMMIT | 0x10_0000, PAGE_READWRITE],
        );
    }
}
