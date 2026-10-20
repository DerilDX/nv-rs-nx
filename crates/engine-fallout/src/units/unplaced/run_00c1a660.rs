//! `(unplaced)/run_00c1a660` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_012024D4: u32 = 0x0120_24d4;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00C1AA00: u32 = 0x00c1_aa00;
const EXIT_HANDLER_ARGUMENT_00C1A7D0: u32 = 0x00c1_a7d0;

// Translated from 00c1ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `012024d4`; when it was 0, calls
/// `00aa2080(00c1aa00, 00c1a7d0)`. Returns `this`, which it does not otherwise read.
pub fn fn_00c1ad50(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_012024D4);
    e.set_global(USE_COUNT_012024D4, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00C1AA00, EXIT_HANDLER_ARGUMENT_00C1A7D0],
        );
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// Import slot of `GetCurrentThreadId` (`stdcall`, no arguments).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// Byte `012024d0`: set to 1 when `fn_00c1aa00` has started.
const LOADERS_REGISTERED: u32 = 0x0120_24d0;
/// Word `012024dc`: the thread id `fn_00c1aa00` keeps.
const START_THREAD: u32 = 0x0120_24dc;

/// The class registrations `fn_00c1aa00` makes, in order: (class name string, creation
/// function).
const LOADERS: [(u32, u32); 53] = [
    (0x010b_d404, 0x00c2_1e90),
    (0x010b_d3f4, 0x00c3_0ca0),
    (0x010b_d3e0, 0x00c1_b7f0),
    (0x010b_d3c0, 0x00c3_0790),
    (0x010b_d39c, 0x00c3_0600),
    (0x010b_d384, 0x00c2_f630),
    (0x010b_d368, 0x00c2_f3a0),
    (0x010b_d350, 0x00c2_ef20),
    (0x010b_d33c, 0x00c2_e4c0),
    (0x010b_d320, 0x00c2_d690),
    (0x010b_d30c, 0x00c2_0350),
    (0x010b_d2f4, 0x00c2_d5b0),
    (0x010b_d2e0, 0x00c2_d180),
    (0x010b_d2c8, 0x00c1_fbf0),
    (0x010b_d2bc, 0x00c2_4da0),
    (0x010b_d2a4, 0x00c2_c7b0),
    (0x010b_d290, 0x00c2_c080),
    (0x010b_d27c, 0x00c1_c5e0),
    (0x010b_d264, 0x00c2_ba60),
    (0x010b_d244, 0x00c2_b970),
    (0x010b_d224, 0x00c2_b870),
    (0x010b_d204, 0x00c2_b720),
    (0x010b_d1e8, 0x00c2_b620),
    (0x010b_d1c8, 0x00c2_b520),
    (0x010b_d1a8, 0x00c2_b420),
    (0x010b_d190, 0x00c2_b330),
    (0x010b_d174, 0x00c2_b200),
    (0x010b_d158, 0x00c2_b110),
    (0x010b_d13c, 0x00c2_b010),
    (0x010b_d124, 0x00c2_2ad0),
    (0x010b_d108, 0x00c2_aa50),
    (0x010b_d0ec, 0x00c2_a870),
    (0x010b_d0d4, 0x00c2_a2b0),
    (0x010b_d0b8, 0x00c2_a160),
    (0x010b_d098, 0x00c2_a050),
    (0x010b_d07c, 0x00c2_9f50),
    (0x010b_d05c, 0x00c2_9e50),
    (0x010b_d048, 0x00c1_f970),
    (0x010b_d02c, 0x00c2_17f0),
    (0x010b_d010, 0x00c2_9d00),
    (0x010b_cff8, 0x00c2_9af0),
    (0x010b_cfe0, 0x00c2_8b20),
    (0x010b_cfc4, 0x00c2_8570),
    (0x010b_cfac, 0x00c2_83b0),
    (0x010b_cf94, 0x00c2_0640),
    (0x010b_cf80, 0x00c2_3b20),
    (0x010b_cf6c, 0x00c1_fff0),
    (0x010b_cf54, 0x00c2_82a0),
    (0x010b_cf34, 0x00c2_7350),
    (0x010b_cf20, 0x00c2_70f0),
    (0x010b_cf04, 0x00c2_6790),
    (0x010b_d42c, 0x00c2_5820),
    (0x010b_d41c, 0x00c2_5ca0),
];

// Translated from 00c1aa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00c1ad50` registers; no
/// arguments): once only (byte `012024d0`), registers the 53 class names of `LOADERS` with
/// their creation functions through `00a64900` and keeps the current thread id (the import
/// at `00fdf1d0`) in `012024dc`.
pub fn fn_00c1aa00(e: &mut Engine) {
    if e.mem.u8(LOADERS_REGISTERED) != 0 {
        return;
    }
    e.mem.set_u8(LOADERS_REGISTERED, 1);
    for (name, creator) in LOADERS {
        e.call(REGISTER_LOADER, &args![name, creator]);
    }
    let thread = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    e.mem.set_u32(START_THREAD, thread);
}

// Translated from 00c1a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// A 16-bit setter (Ghidra made no function here; unnamed in the map; `thiscall`,
/// `(value)`, `RET 4`): `value` stored at `this + 0xC4`.
pub fn fn_00c1a640(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr().wrapping_add(0xc4), value);
}

/// `00a63520` (`cdecl`, `(class name)`): removes a class's loader (the counterpart of
/// `NiStream::RegisterLoader`), and `00c25ac0` (no arguments), the tail call that ends
/// `fn_00c1a7d0`.
const UNREGISTER_LOADER: u32 = 0x00a6_3520;
const AFTER_UNREGISTER_00C25AC0: u32 = 0x00c2_5ac0;
/// The class names `fn_00c1a7d0` unregisters, in its order (the pushed immediates).
const UNREGISTERED_NAMES: [u32; 51] = [
    0x010b_d404,
    0x010b_d3f4,
    0x010b_d3e0,
    0x010b_d3c0,
    0x010b_d39c,
    0x010b_d384,
    0x010b_d368,
    0x010b_d350,
    0x010b_d33c,
    0x010b_d320,
    0x010b_d30c,
    0x010b_d2f4,
    0x010b_d2e0,
    0x010b_d2c8,
    0x010b_d2bc,
    0x010b_d2a4,
    0x010b_d290,
    0x010b_d27c,
    0x010b_d264,
    0x010b_d244,
    0x010b_d224,
    0x010b_d204,
    0x010b_d1e8,
    0x010b_d1c8,
    0x010b_d1a8,
    0x010b_d190,
    0x010b_d174,
    0x010b_d158,
    0x010b_d13c,
    0x010b_d124,
    0x010b_d108,
    0x010b_d0ec,
    0x010b_d0d4,
    0x010b_d0b8,
    0x010b_d098,
    0x010b_d07c,
    0x010b_d05c,
    0x010b_d048,
    0x010b_d02c,
    0x010b_d010,
    0x010b_cff8,
    0x010b_cfe0,
    0x010b_cfc4,
    0x010b_cfac,
    0x010b_cf94,
    0x010b_cf80,
    0x010b_cf6c,
    0x010b_cf54,
    0x010b_cf34,
    0x010b_cf20,
    0x010b_cf04,
];

// Translated from 00c1a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The exit handler `fn_00c1ad50` registers (Ghidra made no function here; unnamed in the
/// map; no arguments): when the loaders were registered (byte `012024d0`), the flag is
/// cleared, each of the 51 class names is unregistered (`00a63520`) and `00c25ac0` runs.
pub fn fn_00c1a7d0(e: &mut Engine) {
    if e.global::<u8>(LOADERS_REGISTERED) == 0 {
        return;
    }
    e.set_global::<u8>(LOADERS_REGISTERED, 0);
    for name in UNREGISTERED_NAMES {
        e.call(UNREGISTER_LOADER, &args![name]);
    }
    e.call(AFTER_UNREGISTER_00C25AC0, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_ad50, fn_00c1ad50(Ptr) -> Ptr),
        entry!(0x00c1_aa00, fn_00c1aa00()),
        entry!(0x00c1a640, fn_00c1a640(Ptr, u16)),
        entry!(0x00c1a7d0, fn_00c1a7d0()),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_ad50, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00c1_aa00, 0x00c1_a7d0]]
        );
        e.call(0x00c1_ad50, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn registers_every_loader_once_and_keeps_the_thread_id() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x4321u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c1_aa00, &args![]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(log.len(), 54);
        assert_eq!(log[0], (REGISTER_LOADER, vec![0x010b_d404, 0x00c2_1e90]));
        assert_eq!(log[53].0, GET_CURRENT_THREAD_ID);
        assert_eq!(e.mem.u32(START_THREAD), 0x4321);
        assert_eq!(e.mem.u8(LOADERS_REGISTERED), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_aa00, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c1a640_stores_the_word_at_0xc4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        e.call(0x00c1a640, &args![this, 0x1_2345u32]);
        assert_eq!(e.mem.u16(this + 0xc4), 0x2345);
        assert_eq!(e.mem.u16(this + 0xc6), 0);
    }

    #[test]
    fn fn_00c1a7d0_unregisters_once() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(UNREGISTER_LOADER, |_, _| Ret::default());
        e.register(AFTER_UNREGISTER_00C25AC0, |_, _| Ret::default());
        e.map(LOADERS_REGISTERED, 1);
        e.call(0x00c1a7d0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global::<u8>(LOADERS_REGISTERED, 1);
        e.call(0x00c1a7d0, &args![]);
        assert_eq!(e.global::<u8>(LOADERS_REGISTERED), 0);
        let log = e.call_log.take().unwrap();
        let names: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == UNREGISTER_LOADER)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(names.len(), 51);
        assert_eq!(names[0], 0x010b_d404);
        assert_eq!(names[50], 0x010b_cf04);
        assert_eq!(log.last().unwrap().0, AFTER_UNREGISTER_00C25AC0);
    }
}
