//! The thread host: the game's threads, stepped cooperatively.
//!
//! The game makes its threads with `CreateThread` (every one through
//! `BSThread::Initialize`, `00aa6430`, whose start `00aa64d0` calls the
//! `BSThread`'s `ThreadProc`, vtable +4) and coordinates them with events and
//! semaphores. Here they run on the main thread's turn, one at a time, behind
//! the platform imports in `units/platform.rs` (`CreateThread`,
//! `ResumeThread`, `SuspendThread`, `ExitThread`, `WaitForSingleObject`,
//! `WaitForMultipleObjects`, `Sleep`, `SetEvent`, `ReleaseSemaphore`,
//! `GetCurrentThreadId`), which dispatch to the [`ThreadHost`] the engine uses
//! ([`host`]). Real OS threads can replace it behind the same trait.
//!
//! **One runs at a time.** Each game thread runs as a coroutine. Its stack is
//! an OS thread used strictly as a coroutine: it holds a baton (the engine,
//! handed over with it) only between being resumed and yielding, and the main
//! thread waits while it runs, so exactly one thread touches the engine at
//! any moment. This avoids stack switching by hand (no `unsafe` assembly, no
//! platform-specific context code); the cost is one OS thread per game thread
//! and a channel hand-off per switch.
//!
//! **When threads run (the switch points), in a fixed order:**
//! - The main thread waits on an object that is not signalled
//!   (`WaitForSingleObject`, `WaitForMultipleObjects`, any timeout) or calls
//!   `Sleep`: the host runs the other threads ([`run_until`]) until the object
//!   is signalled, until a whole pass changes no object's state, or, for a
//!   timed wait, after one pass, once the clock passes its timeout.
//! - A game thread waits on an object that is not signalled, or sleeps: it
//!   yields back to the main thread, which goes on with the next thread of the
//!   pass.
//! - The host's own frame points call [`run_threads`].
//!
//! A pass resumes, in creation order, every thread that is runnable: started
//! for the first time (`CreateThread` without `CREATE_SUSPENDED`, or resumed
//! to a suspend count of 0), sleeping, or waiting on objects that are now
//! signalled. `CreateThread`, `ResumeThread`, `SetEvent` and
//! `ReleaseSemaphore` never switch by themselves: what they make runnable runs
//! at the next switch point. So runs are deterministic.
//!
//! **Per thread** ([`GameThread`]): its id, handle, start and parameter, its
//! suspend count, its run state, its TLS block (`Engine::tls`) and the call
//! stack of the functions it has entered (`Engine::call_stack`), swapped in
//! when it runs; the TLS slot values (`TlsGetValue`) are keyed by its id. Its
//! x87 control word is the word in its TLS block (`units::crt::x87_control_word`), loaded
//! into the FPU register the x87 helpers read whenever it runs (`load_fpu`).
//!
//! A game thread that stops (an untranslated function, a fault) is marked
//! stopped and the stop is raised on the main thread, naming the thread, so
//! the run that switched to it stops with it.

use crate::Engine;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};

/// The main thread's id (`GetCurrentThreadId` on the main thread).
pub const MAIN_THREAD_ID: u32 = 1;
/// The stack an OS thread gets as a game thread's coroutine (the game's code
/// nests deeply in debug builds).
const COROUTINE_STACK: usize = 64 << 20;

/// What a game thread is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunState {
    /// Made, not run yet.
    NotStarted,
    /// Running, or paused at a yield it will go on from when resumed.
    Running,
    /// Waiting on these handles (all of them, or any).
    Waiting { handles: Vec<u32>, all: bool },
    /// Gave up its turn (`Sleep`): runnable at the next pass.
    Sleeping,
    /// Its start function returned or it called `ExitThread`.
    Finished { code: u32 },
    /// It stopped (an untranslated function, a fault): the first line.
    Stopped(String),
}

/// One game thread, as the platform seam keeps it. The main thread is the
/// first, made when first needed.
#[derive(Debug)]
pub struct GameThread {
    pub id: u32,
    /// Its handle (0 for the main thread).
    pub handle: u32,
    pub start: u32,
    pub parameter: u32,
    pub suspend_count: u32,
    pub state: RunState,
    /// Its TLS block (0 until it first runs) and its entered functions,
    /// kept here while another thread runs.
    tls: u32,
    call_stack: Vec<u32>,
    /// The coroutine (made when it first runs).
    link: Option<Link>,
}

/// The engine, handed to the coroutine that runs next.
struct Baton(*mut Engine);
// SAFETY: the engine is used by exactly one OS thread at a time: the main
// thread blocks on the coroutine's reply from handing the baton over until
// the coroutine yields it back (module documentation). The channels order
// every access of one thread before the next thread's.
unsafe impl Send for Baton {}

/// What a coroutine hands back.
#[derive(Debug)]
enum Yield {
    Paused,
    Finished(u32),
    Stopped(String),
}

/// The main thread's end of a coroutine.
struct Link {
    resume: Sender<Baton>,
    yielded: Receiver<Yield>,
}

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Link")
    }
}

/// The game threads (`Kernel::threads`).
#[derive(Debug, Default)]
pub struct Threads {
    /// Every thread, in creation order; the main thread first.
    pub list: Vec<GameThread>,
    /// The running thread's index in [`Threads::list`].
    pub current: usize,
    /// Counts every change of an object's state that can let a waiter go on
    /// (an event set, a semaphore released, a wait satisfied, a thread
    /// ended): a pass that leaves it unchanged made no progress.
    pub signals: u64,
}

thread_local! {
    /// A coroutine's end of its link, on its own OS thread.
    static COROUTINE: RefCell<Option<(Receiver<Baton>, Sender<Yield>)>> = const { RefCell::new(None) };
}

/// Whether an object can let a waiter go on, without taking it.
pub type Signalled = fn(&Engine, u32) -> bool;

/// The seam: how game threads run. The cooperative host is the one in use.
pub trait ThreadHost {
    /// The current thread would wait on `handles` (all or any); `deadline` is
    /// the clock time ([`Engine::clock_now_us`]) a timed wait ends at, `None`
    /// for `INFINITE`. Returns whether they may now be signalled (the caller
    /// takes them and tries again), false when nothing can signal them or the
    /// wait timed out.
    fn block(
        &self,
        e: &mut Engine,
        handles: &[u32],
        all: bool,
        deadline: Option<u64>,
        signalled: Signalled,
    ) -> bool;
    /// The current thread gives up its turn (`Sleep`).
    fn sleep(&self, e: &mut Engine, signalled: Signalled);
    /// The current game thread ends (`ExitThread`).
    fn exit(&self, e: &mut Engine, code: u32);
}

/// The cooperative host (module documentation).
pub struct Cooperative;

/// The host the platform imports use.
pub fn host() -> &'static dyn ThreadHost {
    &Cooperative
}

fn ensure_main(e: &mut Engine) {
    if e.kernel.threads.list.is_empty() {
        e.kernel.threads.list.push(GameThread {
            id: MAIN_THREAD_ID,
            handle: 0,
            start: 0,
            parameter: 0,
            suspend_count: 0,
            state: RunState::Running,
            tls: 0,
            call_stack: Vec::new(),
            link: None,
        });
    }
}

/// The running thread's id.
pub fn current_id(e: &Engine) -> u32 {
    e.kernel
        .threads
        .list
        .get(e.kernel.threads.current)
        .map_or(MAIN_THREAD_ID, |t| t.id)
}

/// Whether the main thread is running.
pub fn on_main(e: &Engine) -> bool {
    e.kernel.threads.current == 0
}

/// Records a new thread (it runs at the next switch point once its suspend
/// count is 0). Returns its id.
pub fn create(e: &mut Engine, handle: u32, start: u32, parameter: u32, suspended: bool) -> u32 {
    ensure_main(e);
    let id = MAIN_THREAD_ID + e.kernel.threads.list.len() as u32;
    e.kernel.threads.list.push(GameThread {
        id,
        handle,
        start,
        parameter,
        suspend_count: u32::from(suspended),
        state: RunState::NotStarted,
        tls: 0,
        call_stack: Vec::new(),
        link: None,
    });
    id
}

/// The thread behind a handle.
pub fn by_handle(e: &mut Engine, handle: u32) -> Option<&mut GameThread> {
    e.kernel
        .threads
        .list
        .iter_mut()
        .find(|t| t.handle == handle && t.handle != 0)
}

/// Whether a thread handle is signalled (its thread ended).
pub fn thread_ended(e: &Engine, handle: u32) -> Option<bool> {
    e.kernel
        .threads
        .list
        .iter()
        .find(|t| t.handle == handle && t.handle != 0)
        .map(|t| matches!(t.state, RunState::Finished { .. } | RunState::Stopped(_)))
}

/// Notes a change that can let a waiter go on.
pub fn signal(e: &mut Engine) {
    e.kernel.threads.signals = e.kernel.threads.signals.wrapping_add(1);
}

fn runnable(e: &Engine, i: usize, signalled: Signalled) -> bool {
    let t = &e.kernel.threads.list[i];
    if t.suspend_count != 0 {
        return false;
    }
    match &t.state {
        RunState::NotStarted | RunState::Sleeping => true,
        RunState::Waiting { handles, all } => {
            if *all {
                handles.iter().all(|&h| signalled(e, h))
            } else {
                handles.iter().any(|&h| signalled(e, h))
            }
        }
        _ => false,
    }
}

/// Runs the other threads from the main thread, pass after pass in creation
/// order, until `until` holds or a pass changes no object's state. Returns
/// `until`'s last answer.
pub fn run_until(
    e: &mut Engine,
    signalled: Signalled,
    until: &mut dyn FnMut(&mut Engine) -> bool,
) -> bool {
    ensure_main(e);
    assert!(
        on_main(e),
        "the thread host runs threads from the main thread only"
    );
    loop {
        if until(e) {
            return true;
        }
        let before = e.kernel.threads.signals;
        let mut ran = false;
        let mut i = 1;
        while i < e.kernel.threads.list.len() {
            if runnable(e, i, signalled) {
                switch_to(e, i);
                ran = true;
                if until(e) {
                    return true;
                }
            }
            i += 1;
        }
        if !ran || e.kernel.threads.signals == before {
            return until(e);
        }
    }
}

/// One pass from the main thread: each runnable thread, in creation order, runs until it yields.
fn one_pass(e: &mut Engine, signalled: Signalled) {
    let mut i = 1;
    while i < e.kernel.threads.list.len() {
        if runnable(e, i, signalled) {
            switch_to(e, i);
        }
        i += 1;
    }
}

/// Runs the game threads until none can go on (the host's frame points).
pub fn run_threads(e: &mut Engine, signalled: Signalled) {
    run_until(e, signalled, &mut |_| false);
}

/// Swaps the running thread's TLS block and call stack for thread `to`'s.
fn swap_context(e: &mut Engine, from: usize, to: usize) {
    let mut to_tls = e.kernel.threads.list[to].tls;
    if to_tls == 0 {
        to_tls = e.new_tls_block();
        e.kernel.threads.list[to].tls = to_tls;
    }
    let from_tls = e.swap_tls(to_tls);
    let to_stack = std::mem::take(&mut e.kernel.threads.list[to].call_stack);
    let from_stack = std::mem::replace(&mut e.call_stack, to_stack);
    let from_thread = &mut e.kernel.threads.list[from];
    from_thread.tls = from_tls;
    from_thread.call_stack = from_stack;
}

/// Hands the engine to thread `i` until it yields (main thread only).
fn switch_to(e: &mut Engine, i: usize) {
    let link = match e.kernel.threads.list[i].link.take() {
        Some(link) => link,
        None => {
            let t = &e.kernel.threads.list[i];
            spawn(t.id, t.start, t.parameter)
        }
    };
    swap_context(e, 0, i);
    e.kernel.threads.current = i;
    e.kernel.threads.list[i].state = RunState::Running;
    let baton = Baton(e as *mut Engine);
    let reply = match link.resume.send(baton) {
        Ok(()) => link.yielded.recv().ok(),
        Err(_) => None,
    };
    // Back on the main thread.
    e.kernel.threads.current = 0;
    swap_context_back(e, i);
    load_fpu(e);
    e.kernel.threads.list[i].link = Some(link);
    match reply {
        Some(Yield::Paused) => {}
        Some(Yield::Finished(code)) => {
            e.kernel.threads.list[i].state = RunState::Finished { code };
            signal(e);
        }
        Some(Yield::Stopped(why)) => {
            let t = &mut e.kernel.threads.list[i];
            t.state = RunState::Stopped(why.clone());
            let (id, start) = (t.id, t.start);
            signal(e);
            panic!("{why} (in game thread {id}, start {start:08x})");
        }
        None => {
            let t = &mut e.kernel.threads.list[i];
            t.state = RunState::Stopped("its coroutine ended".into());
            signal(e);
        }
    }
}

/// Loads the FPU control register the x87 helpers round with (`crate::x87::set_control_word`,
/// a per-OS-thread value) from the running game thread's own word in its TLS block
/// (`units::crt::x87_control_word`), on the OS thread that runs it next: each game thread
/// keeps its own x87 precision (the AI linear task threads `0x027F`, the main thread `0x007F`
/// once the renderer's device exists, `units::platform::direct3d_device_created`).
fn load_fpu(e: &mut Engine) {
    let word = crate::units::crt::x87_control_word(e);
    crate::x87::set_control_word(word);
}

/// [`swap_context`] from thread `from` back to the main thread.
fn swap_context_back(e: &mut Engine, from: usize) {
    let main_tls = e.kernel.threads.list[0].tls;
    let from_tls = e.swap_tls(main_tls);
    let main_stack = std::mem::take(&mut e.kernel.threads.list[0].call_stack);
    let from_stack = std::mem::replace(&mut e.call_stack, main_stack);
    let t = &mut e.kernel.threads.list[from];
    t.tls = from_tls;
    t.call_stack = from_stack;
}

/// Makes thread `id`'s coroutine: an OS thread that waits for the baton,
/// runs `start(parameter)` on the engine and hands back how it ended.
fn spawn(id: u32, start: u32, parameter: u32) -> Link {
    let (resume, resumed) = channel::<Baton>();
    let (yields, yielded) = channel::<Yield>();
    std::thread::Builder::new()
        .name(format!("game thread {id}"))
        .stack_size(COROUTINE_STACK)
        .spawn(move || {
            let Ok(Baton(engine)) = resumed.recv() else {
                return;
            };
            COROUTINE.with(|c| *c.borrow_mut() = Some((resumed, yields.clone())));
            // SAFETY: see `Baton`.
            let e = unsafe { &mut *engine };
            load_fpu(e);
            let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(start, &[parameter]).eax
            }));
            let reply = match out {
                Ok(code) => Yield::Finished(code),
                Err(payload) => {
                    // The functions the thread was in when it stopped, innermost first, as
                    // for the main thread below.
                    let path: Vec<String> = e
                        .call_stack
                        .iter()
                        .rev()
                        .take(6)
                        .map(|a| format!("{a:08x}"))
                        .collect();
                    let first = panic_line(payload);
                    if path.is_empty() {
                        Yield::Stopped(first)
                    } else {
                        Yield::Stopped(format!("{first} (in {})", path.join(" < ")))
                    }
                }
            };
            let _ = yields.send(reply);
        })
        .expect("a game thread's coroutine");
    Link { resume, yielded }
}

fn panic_line(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().next().unwrap_or("").to_string()
}

/// On a coroutine: hands the engine back to the main thread and waits to be
/// resumed. When the engine is gone (the main thread dropped it), the
/// coroutine parks for good: its stack still holds frames of the dropped
/// engine's calls, so it must never run them.
fn yield_to_main(reply: Yield) {
    COROUTINE.with(|c| {
        let c = c.borrow();
        let (resumed, yields) = c
            .as_ref()
            .expect("a coroutine yields only from its own thread");
        if yields.send(reply).is_err() || resumed.recv().is_err() {
            loop {
                std::thread::park();
            }
        }
    });
}

impl ThreadHost for Cooperative {
    fn block(
        &self,
        e: &mut Engine,
        handles: &[u32],
        all: bool,
        deadline: Option<u64>,
        signalled: Signalled,
    ) -> bool {
        let ready = |e: &Engine| {
            if all {
                handles.iter().all(|&h| signalled(e, h))
            } else {
                handles.iter().any(|&h| signalled(e, h))
            }
        };
        ensure_main(e);
        if on_main(e) {
            // One pass gives the others a turn; then they run on until the
            // objects are signalled or the clock passes the deadline. A timed
            // wait (the timeout 0 poll in 00b020a0) so ends even while a
            // thread sleeps in a loop (the loading menu's, Sleep(50) a round).
            one_pass(e, signalled);
            let past = |e: &mut Engine| deadline.is_some_and(|d| e.clock_now_us() >= d);
            run_until(e, signalled, &mut |e| ready(e) || past(e));
            return ready(e);
        }
        let i = e.kernel.threads.current;
        loop {
            e.kernel.threads.list[i].state = RunState::Waiting {
                handles: handles.to_vec(),
                all,
            };
            yield_to_main(Yield::Paused);
            load_fpu(e);
            if ready(e) || deadline.is_some() {
                return ready(e);
            }
        }
    }

    fn sleep(&self, e: &mut Engine, signalled: Signalled) {
        ensure_main(e);
        if on_main(e) {
            one_pass(e, signalled);
            return;
        }
        let i = e.kernel.threads.current;
        e.kernel.threads.list[i].state = RunState::Sleeping;
        yield_to_main(Yield::Paused);
        load_fpu(e);
    }

    fn exit(&self, e: &mut Engine, code: u32) {
        ensure_main(e);
        assert!(
            !on_main(e),
            "ExitThread on the main thread ends the process (not provided)"
        );
        yield_to_main(Yield::Finished(code));
        // A finished thread is never resumed.
        loop {
            std::thread::park();
        }
    }
}

/// What the game's main thread did in a turn ([`run_main_frame`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainTurn {
    /// It reached the end of a frame (its message loop found the queue empty,
    /// [`end_main_frame`]) and waits for the next.
    Frame,
    /// Its start function (`main`) returned this.
    Finished(u32),
    /// It stopped (an untranslated function, a fault): the first line, with the innermost
    /// functions it was in.
    Stopped(String),
}

/// The game's main thread run as a coroutine the host drives frame by frame: `main` (the
/// game's `WinMain`) runs on its own OS thread, as the game threads do (module
/// documentation), and is the thread host's main thread while it runs (it switches to the
/// other game threads itself). Its message loop is the frame driver: when it finds no message
/// (`PeekMessageA`, [`end_main_frame`]) it hands the engine back to the host, which resumes it
/// for the next frame. Between frames its entered functions are kept here.
pub struct MainThread {
    link: Link,
    call_stack: Vec<u32>,
    /// How its last turn ended (`None` before the first).
    pub last: Option<MainTurn>,
}

impl std::fmt::Debug for MainThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainThread")
            .field("last", &self.last)
            .finish()
    }
}

thread_local! {
    /// The main thread's coroutine end of its link, on its own OS thread.
    static MAIN_COROUTINE: RefCell<Option<(Receiver<Baton>, Sender<Yield>)>> = const { RefCell::new(None) };
}

/// Makes the game's main thread: `start(words)` (`main`'s address and its four words) on its
/// own OS thread, run from the first [`run_main_frame`].
pub fn start_main_thread(start: u32, words: Vec<u32>) -> MainThread {
    let (resume, resumed) = channel::<Baton>();
    let (yields, yielded) = channel::<Yield>();
    std::thread::Builder::new()
        .name("game main thread".into())
        .stack_size(COROUTINE_STACK)
        .spawn(move || {
            let Ok(Baton(engine)) = resumed.recv() else {
                return;
            };
            MAIN_COROUTINE.with(|c| *c.borrow_mut() = Some((resumed, yields.clone())));
            // SAFETY: see `Baton`.
            let e = unsafe { &mut *engine };
            load_fpu(e);
            let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(start, &words).eax
            }));
            let reply = match out {
                Ok(code) => Yield::Finished(code),
                Err(payload) => {
                    // The functions it was in when it stopped (`Engine::call` pops only on
                    // return), innermost first.
                    let path: Vec<String> = e
                        .call_stack
                        .iter()
                        .rev()
                        .take(6)
                        .map(|a| format!("{a:08x}"))
                        .collect();
                    let first = panic_line(payload);
                    if path.is_empty() {
                        Yield::Stopped(first)
                    } else {
                        Yield::Stopped(format!("{first} (in {})", path.join(" < ")))
                    }
                }
            };
            let _ = yields.send(reply);
        })
        .expect("the game main thread's coroutine");
    MainThread {
        link: Link { resume, yielded },
        call_stack: Vec::new(),
        last: None,
    }
}

/// Runs the game's main thread for one turn: until the end of its next frame, its return or
/// its stop. Called from the host's thread, never from a game thread. A thread that finished or
/// stopped is not resumed (its last turn is answered again).
pub fn run_main_frame(e: &mut Engine, main: &mut MainThread) -> MainTurn {
    if let Some(last @ (MainTurn::Finished(_) | MainTurn::Stopped(_))) = &main.last {
        return last.clone();
    }
    ensure_main(e);
    e.kernel.threads.current = 0;
    let host_stack = std::mem::replace(&mut e.call_stack, std::mem::take(&mut main.call_stack));
    let baton = Baton(e as *mut Engine);
    let reply = match main.link.resume.send(baton) {
        Ok(()) => main.link.yielded.recv().ok(),
        Err(_) => None,
    };
    main.call_stack = std::mem::replace(&mut e.call_stack, host_stack);
    load_fpu(e);
    let turn = match reply {
        Some(Yield::Paused) => MainTurn::Frame,
        Some(Yield::Finished(code)) => MainTurn::Finished(code),
        Some(Yield::Stopped(why)) => MainTurn::Stopped(why),
        None => MainTurn::Stopped("the main thread's coroutine ended".into()),
    };
    main.last = Some(turn.clone());
    turn
}

/// The end of a frame on the game's main thread (its message loop found the queue empty): when
/// the main thread runs as the host's coroutine ([`start_main_thread`]) it hands the engine
/// back to the host and waits for the next frame, then answers true; anywhere else (a test
/// calling the game's code directly) it answers false at once.
pub fn end_main_frame(e: &mut Engine) -> bool {
    let on_coroutine = MAIN_COROUTINE.with(|c| c.borrow().is_some());
    if !on_coroutine || !on_main(e) {
        return false;
    }
    MAIN_COROUTINE.with(|c| {
        let c = c.borrow();
        let (resumed, yields) = c.as_ref().expect("the main coroutine's link");
        if yields.send(Yield::Paused).is_err() || resumed.recv().is_err() {
            // The host dropped the engine: never run the frames on this stack again.
            loop {
                std::thread::park();
            }
        }
    });
    load_fpu(e);
    true
}

#[cfg(test)]
mod main_thread_tests {
    use super::*;

    /// A stand-in `main` whose loop ends a frame three times, then returns 7.
    fn frames(e: &mut Engine, _: &[u32]) -> crate::Ret {
        for _ in 0..3 {
            let n = e.mem.u32(0x2000);
            e.mem.set_u32(0x2000, n + 1);
            assert!(end_main_frame(e));
        }
        crate::Ret {
            eax: 7,
            ..crate::Ret::default()
        }
    }

    #[test]
    fn the_main_thread_runs_frame_by_frame() {
        let mut e = Engine::new();
        e.mem.map(0x2000, 0x1000);
        e.register(0x0040_0000, frames);
        let mut main = start_main_thread(0x0040_0000, vec![]);
        for frame in 1..=3 {
            assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Frame);
            assert_eq!(e.mem.u32(0x2000), frame);
        }
        assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Finished(7));
        assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Finished(7));
        // Outside the coroutine a frame's end doesn't wait.
        assert!(!end_main_frame(&mut e));
    }

    #[test]
    fn a_stop_on_the_main_thread_comes_back_with_where() {
        let mut e = Engine::new();
        e.register(0x0040_0000, |e, _| e.call(0x00ab_cdef, &[]));
        let mut main = start_main_thread(0x0040_0000, vec![]);
        let MainTurn::Stopped(why) = run_main_frame(&mut e, &mut main) else {
            panic!("a stop");
        };
        assert!(why.contains("00abcdef"), "{why}");
    }
}
