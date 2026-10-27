use crate::abi::{AbiFn, Arg, Ret, RetVal};
use crate::mem::Mem;
use crate::ptr::{Field, Layout, Ptr, Scalar};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::path::Path;
use std::rc::Rc;

extern "Rust" {
    /// The registrations of the engine crates above this one, defined once
    /// per program by [`crate::assemble!`] (docs/ENGINE_CRATE.md, "The engine
    /// crates"). A program that links this crate without it does not link.
    fn nv_engine_assembled_funcs(out: &mut Vec<(u32, AbiFn)>);
}

/// Defines which part crates' functions [`Engine::new`] registers besides
/// this crate's own, as a list of `funcs(out)` functions (each part crate's
/// `units::funcs`). Exactly one crate of a program invokes it: the `engine`
/// crate, with every part crate (so the full engine has every translated
/// function), or a part crate under `cfg(test)` for its own unit tests, with
/// itself and the crates below it (generated into its `units/mod.rs` by
/// `ledger units`).
#[macro_export]
macro_rules! assemble {
    ($($funcs:path),* $(,)?) => {
        #[doc(hidden)]
        #[no_mangle]
        pub fn nv_engine_assembled_funcs(out: &mut Vec<(u32, $crate::AbiFn)>) {
            let _ = &out;
            $( $funcs(out); )*
        }
    };
}

/// The C runtime's `__sse2_available` and `__locale_changed` words ([`Engine::new`]).
const CRT_SSE2_AVAILABLE: u32 = 0x0127_0a6c;
const CRT_LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// The C runtime's math-error word (`011c2b60`, [`Engine::new`]) and its image value.
const CRT_MATHERR_SKIP: u32 = 0x011c_2b60;
const CRT_MATHERR_SKIP_IMAGE: u32 = 0x0000_2694;

/// A test double that may keep state (see [`Engine::register_double`]).
pub type Double = Rc<RefCell<dyn FnMut(&mut Engine, &[u32]) -> Ret>>;

/// What runs when a call reaches an address nobody has translated
/// ([`Engine::on_open`]): given the address and the argument words, it may
/// record the miss; `Some(ret)` makes the call return `ret` (a diagnostic
/// run that goes on past the gap), `None` stops as without a handler.
pub type OpenHandler = Rc<RefCell<dyn FnMut(&mut Engine, u32, &[u32]) -> Option<Ret>>>;

#[derive(Clone)]
enum Callable {
    Abi(AbiFn),
    Double(Double),
}

/// The hasher of the function table and the call profile: their keys are exe addresses (and
/// caller/address pairs), so a multiply whose high half is folded back into the low one
/// spreads them well, at a fraction of SipHash's cost ([`Engine::call`] runs millions of
/// times a second).
#[derive(Default, Clone, Copy)]
pub struct AddrHasher(u64);

impl Hasher for AddrHasher {
    fn finish(&self) -> u64 {
        let p = self.0.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        p ^ (p >> 32)
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(8) ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = self.0.rotate_left(32) ^ u64::from(n);
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = self.0.rotate_left(32) ^ n;
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
}

/// A map keyed by exe address (or a caller/address pair) with [`AddrHasher`].
pub type AddrMap<K, V> = HashMap<K, V, BuildHasherDefault<AddrHasher>>;

/// How many entries [`Engine::call`]'s direct-mapped cache of plain functions has (a power of
/// two).
const CALL_CACHE_LEN: usize = 1 << 14;

/// The cache slot of `addr` (Fibonacci hashing: the top bits of the product).
#[inline(always)]
fn cache_slot(addr: u32) -> usize {
    (addr.wrapping_mul(0x9e37_79b1) >> (32 - CALL_CACHE_LEN.trailing_zeros())) as usize
}

/// The game's memory plus every translated function, by exe address.
pub struct Engine {
    pub mem: Mem,
    funcs: AddrMap<u32, Callable>,
    /// A direct-mapped cache in front of `funcs` for plain functions (doubles never enter it):
    /// (address, function) at [`cache_slot`] of the address, `None` when empty.
    /// [`Engine::register`] and [`Engine::register_double`] empty the slot of the address they
    /// change, so it always answers as `funcs` does.
    call_cache: Box<[(u32, Option<AbiFn>)]>,
    /// When `Some`, every call made through [`Engine::call`] is recorded
    /// as (address, argument words). Tests use it to check what a
    /// translation called.
    pub call_log: Option<Vec<(u32, Vec<u32>)>>,
    tls: u32,
    /// Milliseconds (since the machine started once the real clock is on): the one clock `GetTickCount`, `timeGetTime` and
    /// `QueryPerformanceCounter` report ([`Engine::clock_now_ms`]). `Sleep(n)` moves it on by
    /// `n`; with the real clock on ([`Engine::start_real_clock`], the running game) the time
    /// that really passed is added at each reading. Tests set it directly.
    pub clock_ms: u32,
    /// The microseconds past `clock_ms` (the counter's finer part).
    pub clock_us: u32,
    /// When the real clock is on: the instant the clock was last brought up to date.
    pub real_clock: Option<std::time::Instant>,
    /// The Windows kernel objects the platform imports hand out (events,
    /// semaphores, threads, `VirtualAlloc` regions; `units/platform.rs`).
    pub kernel: crate::units::platform::Kernel,
    /// The functions entered through [`Engine::call`] that haven't returned,
    /// innermost last: the nearest caller entered by address of whatever
    /// runs now (calls within one unit file are direct and not on it). A
    /// stop (a panic) leaves it as it was; whoever catches the stop
    /// truncates it.
    pub call_stack: Vec<u32>,
    /// Runs when a call reaches an untranslated address, before the stop
    /// ([`OpenHandler`]). The host uses it to record which addresses a run
    /// of the game's code needs, and from where.
    pub on_open: Option<OpenHandler>,
    /// When `Some`, every call that returns through [`Engine::call`] is
    /// recorded as (address, argument words, `EAX`), in the order they
    /// return, keeping the last [`TRACE_KEEP`]: a diagnostic of where a run
    /// went wrong before it stopped (the host's `NV_MOVE_TRACE`).
    pub trace: Option<std::collections::VecDeque<(u32, Vec<u32>, u32)>>,
    /// How many returned calls the trace keeps ([`TRACE_KEEP`] unless the host says otherwise).
    pub trace_keep: usize,
    /// When set, how many more calls through [`Engine::call`] may run before the run stops with
    /// "call budget spent" (a diagnostic for a run that loops for ever; the host's
    /// `NV_MOVE_CALL_BUDGET`).
    pub call_budget: Option<u64>,
    /// When `Some`, how many times each (caller, address) pair was called through
    /// [`Engine::call`], keyed `caller << 32 | address` (the caller is the innermost function on
    /// the call stack, 0 at a stage's top): where a long run spends its calls (the host's
    /// `NV_MOVE_CALL_PROFILE`).
    pub call_counts: Option<AddrMap<u64, u64>>,
}

/// How many returned calls [`Engine::trace`] keeps.
pub const TRACE_KEEP: usize = 4000;

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    /// Empty memory, every translated unit registered: this crate's
    /// (`units::crt`, `units::platform`) and those of the engine crates the
    /// program assembles ([`crate::assemble!`]): in the `engine` crate every
    /// part crate's, so every translated function; in a part crate's own
    /// unit tests that crate's and those of the crates below it.
    pub fn new() -> Self {
        let mut list = Vec::new();
        crate::units::funcs(&mut list);
        // SAFETY: `nv_engine_assembled_funcs` is the plain Rust function that
        // `assemble!` defines in the crate assembling the engine; it only
        // appends to `list`.
        unsafe { nv_engine_assembled_funcs(&mut list) };
        let mut funcs = AddrMap::with_capacity_and_hasher(list.len(), Default::default());
        for (addr, f) in list {
            if funcs.insert(addr, Callable::Abi(f)).is_some() {
                panic!("{addr:08x} is registered twice");
            }
        }
        // A new main thread: its block's control word reads as the default
        // until set, and the FPU register the x87 helpers read starts there
        // too (a test that ran earlier on this host thread may have left
        // another word in it).
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        let mut mem = Mem::new();
        // The C runtime words its string and memory routines read first, as the image
        // holds them before the CRT start-up: `__sse2_available` (`01270a6c`) and
        // `__locale_changed` (`0126fdf4`) lie in `.data` past its file contents, so they
        // start as 0. In a real run the start-up's CPU check sets `__sse2_available` from
        // the machine's CPUID answers; a fresh engine (the tests) therefore takes the
        // exe's non-SSE2 paths and the C locale.
        mem.map(CRT_SSE2_AVAILABLE, 4);
        mem.map(CRT_LOCALE_CHANGED, 4);
        // The word `__handle_qnan1`, `__except1`, `__87except` and `__cintrindisp1/2` read
        // before reporting a math error through `_umatherr` (`011c2b60`, `.data` inside its
        // file contents): the image holds 0x2694 and nothing in the exe writes it, so it
        // keeps that value for the whole run (nonzero: no report).
        mem.map(CRT_MATHERR_SKIP, 4);
        mem.write(CRT_MATHERR_SKIP, &CRT_MATHERR_SKIP_IMAGE.to_le_bytes());
        Engine {
            mem,
            funcs,
            call_cache: vec![(0, None); CALL_CACHE_LEN].into_boxed_slice(),
            call_log: None,
            tls: 0,
            clock_ms: 0,
            clock_us: 0,
            real_clock: None,
            kernel: Default::default(),
            call_stack: Vec::new(),
            on_open: None,
            trace: None,
            trace_keep: TRACE_KEEP,
            call_budget: None,
            call_counts: None,
        }
    }

    /// [`Engine::new`] with the data sections of the player's FalloutNV.exe
    /// mapped ([`crate::exe::map_data_sections`]).
    pub fn with_exe(exe: &Path) -> Result<Self, String> {
        let mut e = Self::new();
        crate::exe::map_data_sections(&mut e.mem, exe)?;
        let file = std::fs::read(exe).map_err(|err| format!("{}: {err}", exe.display()))?;
        e.register_imports(&crate::exe::imports(&file)?);
        // The process starts in the executable's folder (where Steam starts
        // the game): `GetCurrentDirectoryA` reports it.
        if let Some(folder) = exe.parent() {
            let folder = folder.to_string_lossy().replace('/', "\\");
            e.kernel.current_directory = folder.into_bytes();
        }
        Ok(e)
    }

    /// Registers the platform versions of imported functions
    /// (`units::platform::imports`) at their import-slot addresses, so a
    /// translation calls `e.call(slot, ..)` as the game calls `[slot]`.
    /// As the Windows loader fills the import table, each slot's word (when
    /// mapped) becomes the address of the function behind it: here the slot's
    /// own address, so code that reads a slot and calls the word later (the
    /// CRT's `__mtinit` `00ed1327` keeps `TlsGetValue`, `TlsSetValue` and
    /// `TlsFree` that way) reaches the same stand-in, or stops at the slot when
    /// none is provided. Returns how many were registered.
    pub fn register_imports(&mut self, imports: &[crate::exe::Import]) -> usize {
        let table = crate::units::platform::imports();
        let mut n = 0;
        for i in imports {
            if self.mem.is_mapped(i.slot) {
                self.mem.set_u32(i.slot, i.slot);
            }
            if let Some((_, _, f)) = table
                .iter()
                .find(|(d, name, _)| d.eq_ignore_ascii_case(&i.dll) && *name == i.name)
            {
                self.register(i.slot, *f);
                n += 1;
            }
        }
        n
    }

    /// Address of the running thread's TLS block: what the game reaches as
    /// `*(*(FS:[0x2c]) + _tls_index * 4)`, so a field the game reads at
    /// `+0x2b4` of it is at `e.tls() + 0x2b4`. Each game thread has its own
    /// (the thread host, `crate::thread_host`, swaps them when it switches
    /// threads). Allocated on first use ([`Engine::new_tls_block`]).
    pub fn tls(&mut self) -> u32 {
        if self.tls == 0 {
            self.tls = self.new_tls_block();
        }
        self.tls
    }

    /// A new TLS block: 0x1000 bytes, zeroed, then the exe's `.tls` template
    /// when it is mapped, as Windows makes one for each thread.
    pub fn new_tls_block(&mut self) -> u32 {
        let block = self.mem.alloc(0x1000);
        if self.mem.is_mapped(0x0127_2000) {
            let template = self.mem.bytes(0x0127_2000, 0x2c5);
            self.mem.write(block, &template);
        }
        block
    }

    /// The running thread's TLS block as it is (0 before first use), and
    /// the replacement of it: the thread host's switch.
    pub(crate) fn swap_tls(&mut self, block: u32) -> u32 {
        std::mem::replace(&mut self.tls, block)
    }

    /// Turns the real clock on: from now each reading of the clock adds the time that really
    /// passed since the last one (the running game; tests leave it off). On Windows the clock
    /// starts at the host's own `GetTickCount` (milliseconds since the machine started), as the
    /// game sees it: code that takes a zero start stamp as long ago depends on it (the loading
    /// menu's `Update` 00789a90 checks the intro movie only within 3 s of `+0x5b4`, 0 at first).
    pub fn start_real_clock(&mut self) {
        #[cfg(windows)]
        {
            extern "system" {
                fn GetTickCount() -> u32;
            }
            // SAFETY: GetTickCount takes nothing and only reads the system's tick count.
            self.clock_ms = unsafe { GetTickCount() };
            self.clock_us = 0;
        }
        self.real_clock = Some(std::time::Instant::now());
    }

    /// Brings the clock up to date with the real time that passed (real clock on only).
    fn catch_up_clock(&mut self) {
        if let Some(last) = self.real_clock {
            let now = std::time::Instant::now();
            let passed = now.duration_since(last).as_micros() as u64;
            self.real_clock = Some(now);
            self.advance_clock_us(passed);
        }
    }

    /// Moves the clock on by `us` microseconds (milliseconds wrap as a `DWORD` does).
    pub fn advance_clock_us(&mut self, us: u64) {
        let total = u64::from(self.clock_us) + us;
        self.clock_ms = self.clock_ms.wrapping_add((total / 1000) as u32);
        self.clock_us = (total % 1000) as u32;
    }

    /// The clock in milliseconds now (what `GetTickCount` and `timeGetTime` answer).
    pub fn clock_now_ms(&mut self) -> u32 {
        self.catch_up_clock();
        self.clock_ms
    }

    /// The clock in microseconds now (what `QueryPerformanceCounter` answers at 1 MHz).
    pub fn clock_now_us(&mut self) -> u64 {
        self.catch_up_clock();
        u64::from(self.clock_ms) * 1000 + u64::from(self.clock_us)
    }

    /// Number of registered functions.
    pub fn translated_count(&self) -> usize {
        self.funcs.len()
    }

    /// Whether a call at `addr` has something to run: a registered function,
    /// or a row of the atexit-destructor table ([`crate::static_exit`]) whose
    /// destructor is translated.
    pub fn is_translated(&self, addr: u32) -> bool {
        self.funcs.contains_key(&addr) || crate::static_exit::runs(self, addr)
    }

    /// Registers (or replaces) the function at `addr`. Tests use this for
    /// doubles of callees that are not under test.
    /// Whether something (a translation, a platform function or a double) is registered at
    /// `addr`.
    pub fn is_registered(&self, addr: u32) -> bool {
        self.funcs.contains_key(&addr)
    }

    pub fn register(&mut self, addr: u32, f: AbiFn) {
        self.funcs.insert(addr, Callable::Abi(f));
        self.call_cache[cache_slot(addr)] = (0, None);
    }

    /// Registers a test double that keeps state (counts calls, returns a
    /// sequence, records arguments). A double that calls itself again
    /// while running panics.
    pub fn register_double(
        &mut self,
        addr: u32,
        f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        self.funcs
            .insert(addr, Callable::Double(Rc::new(RefCell::new(f))));
        self.call_cache[cache_slot(addr)] = (0, None);
    }

    /// Calls the function at exe address `addr` with argument words in
    /// the uniform form ([`crate::abi`]). Panics if nobody has translated
    /// it yet: the translation that called it can only be checked once
    /// that callee exists (or a test supplies a double). An address with no
    /// registered function runs its row of the atexit-destructor table
    /// ([`crate::static_exit`]) if it has one.
    pub fn call(&mut self, addr: u32, args: &[u32]) -> Ret {
        if let Some(log) = &mut self.call_log {
            log.push((addr, args.to_vec()));
        }
        let slot = cache_slot(addr);
        if let (cached, Some(f)) = self.call_cache[slot] {
            if cached == addr {
                return self.run(addr, args, Callable::Abi(f));
            }
        }
        let Some(f) = self.funcs.get(&addr).cloned() else {
            if let Some(row) = crate::static_exit::row(addr) {
                self.call_stack.push(addr);
                let ret = crate::static_exit::run_row(self, row);
                self.call_stack.pop();
                return ret;
            }
            if let Some(handler) = self.on_open.clone() {
                if let Some(ret) = (handler.borrow_mut())(self, addr, args) {
                    return ret;
                }
            }
            panic!("open function {addr:08x}: not translated yet (FalloutNV.exe 1.4.0.525, docs/LEDGER.md)")
        };
        if let Callable::Abi(plain) = f {
            self.call_cache[slot] = (addr, Some(plain));
        }
        self.run(addr, args, f)
    }

    /// The rest of [`Engine::call`] once the callee is found: the budget, the profile, the
    /// call stack, the call itself and the trace.
    #[inline(always)]
    fn run(&mut self, addr: u32, args: &[u32], f: Callable) -> Ret {
        if let Some(budget) = &mut self.call_budget {
            if *budget == 0 {
                panic!("call budget spent (at {addr:08x})");
            }
            *budget -= 1;
        }
        if let Some(counts) = &mut self.call_counts {
            let caller = self.call_stack.last().copied().unwrap_or(0);
            *counts
                .entry(u64::from(caller) << 32 | u64::from(addr))
                .or_default() += 1;
        }
        self.call_stack.push(addr);
        let ret = match f {
            Callable::Abi(f) => f(self, args),
            Callable::Double(d) => (d.borrow_mut())(self, args),
        };
        self.call_stack.pop();
        if let Some(trace) = &mut self.trace {
            // A full ring hands its oldest entry's word buffer to the newest: no allocation
            // per call once full.
            let mut words = if trace.len() >= self.trace_keep {
                trace.pop_front().map(|(_, w, _)| w).unwrap_or_default()
            } else {
                Vec::new()
            };
            words.clear();
            words.extend_from_slice(args);
            trace.push_back((addr, words, ret.eax));
        }
        ret
    }

    /// [`Engine::call`] with the result converted.
    pub fn call_as<R: RetVal>(&mut self, addr: u32, args: &[u32]) -> R {
        R::from_ret(self.call(addr, args))
    }

    /// Virtual call: reads the vtable pointer at `this + 0` and the
    /// function at byte offset `slot` in that table (as the game's
    /// `call dword ptr [eax + slot]`), then calls it with `this` first.
    pub fn vcall(&mut self, this: u32, slot: u32, args: &[u32]) -> Ret {
        let vtable = self.mem.u32(this);
        let target = self.mem.u32(vtable.wrapping_add(slot));
        // `this` and the arguments on the stack when they fit (no allocation per call).
        const INLINE: usize = 15;
        if args.len() <= INLINE {
            let mut words = [0u32; INLINE + 1];
            words[0] = this;
            words[1..=args.len()].copy_from_slice(args);
            self.call(target, &words[..=args.len()])
        } else {
            let mut words = Vec::with_capacity(args.len() + 1);
            words.push(this);
            words.extend_from_slice(args);
            self.call(target, &words)
        }
    }

    /// Field read through a typed pointer.
    pub fn get<S, T: Scalar>(&self, p: Ptr<S>, f: Field<S, T>) -> T {
        T::load(&self.mem, p.0.wrapping_add(f.off))
    }

    /// Field write through a typed pointer.
    pub fn set<S, T: Scalar>(&mut self, p: Ptr<S>, f: Field<S, T>, v: T) {
        v.store(&mut self.mem, p.0.wrapping_add(f.off))
    }

    /// A global variable at its exe address (in `.data`/`.rdata`).
    pub fn global<T: Scalar>(&self, addr: u32) -> T {
        T::load(&self.mem, addr)
    }

    pub fn set_global<T: Scalar>(&mut self, addr: u32, v: T) {
        v.store(&mut self.mem, addr)
    }

    /// A zeroed heap block the size of `T` (tests and allocators).
    pub fn new_object<T: Layout>(&mut self) -> Ptr<T> {
        Ptr::new(self.mem.alloc(T::SIZE))
    }

    /// Maps zeroed memory at a fixed address (a global or a vtable in a
    /// test that runs without the executable).
    pub fn map(&mut self, addr: u32, len: u32) {
        self.mem.map(addr, len)
    }

    /// Writes a vtable (slot targets) at `addr`; for tests without the
    /// executable. With it mapped, the game's own vtables are in memory.
    pub fn put_vtable(&mut self, addr: u32, slots: &[u32]) {
        self.mem.map(addr, 4 * slots.len() as u32);
        for (i, s) in slots.iter().enumerate() {
            self.mem.set_u32(addr + 4 * i as u32, *s);
        }
    }

    /// Typed argument helper for [`Engine::call`]: `e.call(a, &words![..])`
    /// is usually written with the `args!` macro; this is the one-argument
    /// form.
    pub fn call1<A: Arg, R: RetVal>(&mut self, addr: u32, a: A) -> R {
        let mut w = crate::abi::Words::new();
        a.put_words(&mut w);
        self.call_as(addr, &w)
    }
}

impl Engine {
    /// A block of `size` zeroed bytes for the duration of `f`: a local the
    /// game keeps on its stack and passes by address (a scope guard, a
    /// `NiPointer` temporary, an out parameter). Freed when `f` returns.
    pub fn with_stack<R>(&mut self, size: u32, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
        let block = self.mem.alloc(size);
        let r = f(self, Ptr::new(block));
        self.mem.free(block);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args;

    #[test]
    fn doubles_keep_state() {
        let mut e = Engine::new();
        let mut n = 0u32;
        e.register_double(0x0040_1000, move |_, a| {
            n += a[0];
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0040_1000, &args![2u32]).u32(), 2);
        assert_eq!(e.call(0x0040_1000, &args![3u32]).u32(), 5);
    }

    /// A one-line getter in the uniform form, as the game's field getters are translated.
    fn field_getter(e: &mut Engine, this: Ptr) -> u32 {
        e.mem.u32(this.addr() + 4)
    }

    /// A function registered or replaced after it was called answers from then on (the
    /// call cache follows the table).
    #[test]
    fn replacing_a_called_function_takes_effect() {
        let mut e = Engine::new();
        e.register(0x0040_2000, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 1);
        e.register(0x0040_2000, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 2);
        e.register_double(0x0040_2000, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 3);
        e.register(0x0040_2000, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 4);
    }

    /// The trace keeps the last `trace_keep` returned calls with their words, oldest first.
    #[test]
    fn the_trace_keeps_the_last_calls() {
        let mut e = Engine::new();
        e.register(0x0040_3000, |_, a| Ret {
            eax: a.iter().sum(),
            ..Ret::default()
        });
        e.trace = Some(Default::default());
        e.trace_keep = 3;
        e.call(0x0040_3000, &[1, 2, 3]);
        e.call(0x0040_3000, &[4]);
        e.call(0x0040_3000, &[]);
        e.call(0x0040_3000, &[5, 6]);
        let trace: Vec<_> = e.trace.take().unwrap().into_iter().collect();
        assert_eq!(
            trace,
            vec![
                (0x0040_3000, vec![4], 4),
                (0x0040_3000, vec![], 0),
                (0x0040_3000, vec![5, 6], 11)
            ]
        );
    }

    /// Times calls of a registered one-line getter through [`Engine::call`] (the dispatch
    /// cost): `cargo test --release -p engine-core --lib -- --ignored --nocapture
    /// dispatch_speed`; `NV_BENCH_CALLS` sets how many (default 300M).
    #[test]
    #[ignore]
    fn dispatch_speed() {
        let n: u64 = std::env::var("NV_BENCH_CALLS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300_000_000);
        let mut e = Engine::new();
        let (addr, f) = crate::entry!(0x0055_9450, field_getter(Ptr) -> u32);
        e.register(addr, f);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        e.trace = Some(Default::default());
        e.call_budget = Some(u64::MAX);
        for (name, with_args) in [("&[this]", false), ("&args![this]", true)] {
            let start = std::time::Instant::now();
            let mut sum = 0u32;
            for _ in 0..n {
                let ret = if with_args {
                    e.call(addr, &args![std::hint::black_box(this)])
                } else {
                    e.call(addr, &[std::hint::black_box(this)])
                };
                sum = sum.wrapping_add(ret.eax);
            }
            let secs = start.elapsed().as_secs_f64();
            assert_eq!(sum, n as u32);
            println!(
                "{name}: {n} calls in {secs:.2} s: {:.1} M calls/s, {:.1} ns per call (trace and budget on)",
                n as f64 / secs / 1e6,
                secs * 1e9 / n as f64
            );
        }
    }

    #[test]
    fn stack_blocks_are_freed() {
        let mut e = Engine::new();
        let p = e.with_stack(8, |e, p| {
            e.mem.set_u32(p.addr(), 1);
            p
        });
        assert_eq!(e.mem.block_size(p.addr()), None);
    }

    #[test]
    #[should_panic(expected = "open function 00abcdef")]
    fn untranslated_calls_name_the_address() {
        Engine::new().call(0x00ab_cdef, &[]);
    }

    /// The open-function handler sees the address, the arguments and the
    /// nearest caller entered by address; returning a value goes on, and
    /// the call stack is back to empty after the outer call.
    #[test]
    fn the_open_handler_sees_the_miss_and_its_caller() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.on_open = Some(Rc::new(RefCell::new(
            move |e: &mut Engine, addr: u32, args: &[u32]| {
                record
                    .borrow_mut()
                    .push((addr, args.to_vec(), e.call_stack.last().copied()));
                Some(Ret {
                    eax: 7,
                    ..Ret::default()
                })
            },
        )));
        e.register(0x00aa_0000, |e, a| e.call(0x00ab_cdef, &[a[0] + 1]));
        assert_eq!(e.call(0x00aa_0000, &[1]).u32(), 7);
        assert_eq!(
            *seen.borrow(),
            vec![(0x00ab_cdef, vec![2], Some(0x00aa_0000))]
        );
        assert!(e.call_stack.is_empty());
    }
}
