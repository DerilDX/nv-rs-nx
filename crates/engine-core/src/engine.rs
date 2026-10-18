use crate::abi::{AbiFn, Arg, Ret, RetVal};
use crate::mem::Mem;
use crate::ptr::{Field, Layout, Ptr, Scalar};
use std::cell::RefCell;
use std::collections::HashMap;
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

/// A test double that may keep state (see [`Engine::register_double`]).
pub type Double = Rc<RefCell<dyn FnMut(&mut Engine, &[u32]) -> Ret>>;

#[derive(Clone)]
enum Callable {
    Abi(AbiFn),
    Double(Double),
}

/// The game's memory plus every translated function, by exe address.
pub struct Engine {
    pub mem: Mem,
    funcs: HashMap<u32, Callable>,
    /// When `Some`, every call made through [`Engine::call`] is recorded
    /// as (address, argument words). Tests use it to check what a
    /// translation called.
    pub call_log: Option<Vec<(u32, Vec<u32>)>>,
    tls: u32,
    /// Milliseconds since start, as `GetTickCount`/`timeGetTime` report
    /// them. The frame driver advances it; nothing else does.
    pub clock_ms: u32,
    /// The Windows kernel objects the platform imports hand out (events,
    /// semaphores, threads, `VirtualAlloc` regions; `units/platform.rs`).
    pub kernel: crate::units::platform::Kernel,
}

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
        let mut funcs = HashMap::with_capacity(list.len());
        for (addr, f) in list {
            if funcs.insert(addr, Callable::Abi(f)).is_some() {
                panic!("{addr:08x} is registered twice");
            }
        }
        Engine {
            mem: Mem::new(),
            funcs,
            call_log: None,
            tls: 0,
            clock_ms: 0,
            kernel: Default::default(),
        }
    }

    /// [`Engine::new`] with the data sections of the player's FalloutNV.exe
    /// mapped ([`crate::exe::map_data_sections`]).
    pub fn with_exe(exe: &Path) -> Result<Self, String> {
        let mut e = Self::new();
        crate::exe::map_data_sections(&mut e.mem, exe)?;
        let file = std::fs::read(exe).map_err(|err| format!("{}: {err}", exe.display()))?;
        e.register_imports(&crate::exe::imports(&file)?);
        Ok(e)
    }

    /// Registers the platform versions of imported functions
    /// (`units::platform::imports`) at their import-slot addresses, so a
    /// translation calls `e.call(slot, ..)` as the game calls `[slot]`.
    /// Returns how many were registered.
    pub fn register_imports(&mut self, imports: &[crate::exe::Import]) -> usize {
        let table = crate::units::platform::imports();
        let mut n = 0;
        for i in imports {
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

    /// Address of the main thread's TLS block: what the game reaches as
    /// `*(*(FS:[0x2c]) + _tls_index * 4)`, so a field the game reads at
    /// `+0x2b4` of it is at `e.tls() + 0x2b4`. One thread: the game's
    /// threads are `platform`. Allocated (0x1000 bytes, zeroed, then the
    /// exe's `.tls` template when mapped) on first use.
    pub fn tls(&mut self) -> u32 {
        if self.tls == 0 {
            self.tls = self.mem.alloc(0x1000);
            if self.mem.is_mapped(0x0127_2000) {
                let template = self.mem.bytes(0x0127_2000, 0x2c5);
                self.mem.write(self.tls, &template);
            }
        }
        self.tls
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
    pub fn register(&mut self, addr: u32, f: AbiFn) {
        self.funcs.insert(addr, Callable::Abi(f));
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
        let Some(f) = self.funcs.get(&addr).cloned() else {
            if let Some(row) = crate::static_exit::row(addr) {
                return crate::static_exit::run_row(self, row);
            }
            panic!("open function {addr:08x}: not translated yet (FalloutNV.exe 1.4.0.525, docs/LEDGER.md)")
        };
        match f {
            Callable::Abi(f) => f(self, args),
            Callable::Double(d) => (d.borrow_mut())(self, args),
        }
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
        let mut words = Vec::with_capacity(args.len() + 1);
        words.push(this);
        words.extend_from_slice(args);
        self.call(target, &words)
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
        let mut w = Vec::with_capacity(A::WORDS);
        a.put(&mut w);
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
}
