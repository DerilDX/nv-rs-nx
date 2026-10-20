//! `fallout/audio/run_0082f140` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `LeaveCriticalSection` (import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 0082f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: leaves the critical section at `this`
/// (`LeaveCriticalSection(this)`).
pub fn fn_0082f1f0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE_CRITICAL_SECTION, &args![this]);
}

/// The function `fn_0082f190` calls on the member (`0082f1b0`).
const CALLEE_0082F1B0: u32 = 0x0082_f1b0;

/// The function `fn_0082f1d0` calls on the member (`0082f1f0`).
const CALLEE_0082F1F0: u32 = 0x0082_f1f0;

// Translated from 0082f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1b0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f190(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1B0, &args![this.byte_add(0x10c)]);
}

// Translated from 0082f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1f0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f1d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1F0, &args![this.byte_add(0x10c)]);
}

/// `BSThread::BSThread` (engine map; `thiscall`, no stack words).
const BS_THREAD_CONSTRUCT: u32 = 0x00aa_6360;
/// The vtable `0107cbe8` (RTTI `.?AVFAMThread@?A0x94d3b31f@@`, a class in an anonymous
/// namespace).
const FAM_THREAD_VTABLE: u32 = 0x0107_cbe8;
/// `0096a2d0`, run on the member at +0x34 (`thiscall`, no stack words; unnamed in the engine
/// map).
const MEMBER_34_CALL: u32 = 0x0096_a2d0;
/// `CreateEventA` (import slot).
const CREATE_EVENT_A: u32 = 0x00fd_f0cc;
/// `0059bb30` (`thiscall`, no stack words): answers the word at +0x24 of its object. The engine
/// map's name for it (`D3DTexture_LockRect`) does not fit that body.
const READ_WORD_24: u32 = 0x0059_bb30;

// Translated from 0082f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of `FAMThread` (by the vtable it stores; the
/// element constructor `falloutaudiomedia.rs` hands to the array constructor `00ec782f`,
/// 0x4c-byte elements). `thiscall`, no stack words (plain `RET`). Runs `BSThread`'s
/// constructor, stores the vtable, runs `0096a2d0` on +0x34, sets +0x3c to 1, stores
/// `CreateEventA(NULL, FALSE, FALSE, NULL)` at +0x30, clears +0x40 and +0x44, and stores what
/// `0059bb30` answers for `this` (its word at +0x24) at +0x48. Answers `this`.
// Translation incomplete: 0082f610 - C++ exception unwinding (SEH frame state: the BSThread destructor on a throw after its constructor) is not translated.
pub fn fn_0082f610(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(BS_THREAD_CONSTRUCT, &args![this]);
    e.mem.set_u32(at, FAM_THREAD_VTABLE);
    e.call(MEMBER_34_CALL, &args![at + 0x34]);
    e.mem.set_u32(at + 0x3c, 1);
    let event = e.call(CREATE_EVENT_A, &args![0u32, 0u32, 0u32, 0u32]).u32();
    e.mem.set_u32(at + 0x30, event);
    e.mem.set_u32(at + 0x40, 0);
    e.mem.set_u32(at + 0x44, 0);
    let word = e.call(READ_WORD_24, &args![this]).u32();
    e.mem.set_u32(at + 0x48, word);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082f1f0, fn_0082f1f0(Ptr)),
        entry!(0x0082_f190, fn_0082f190(Ptr)),
        entry!(0x0082_f1d0, fn_0082f1d0(Ptr)),
        entry!(0x0082_f610, fn_0082f610(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_the_section_at_this() {
        let mut e = Engine::new();
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082f1f0, &args![0x1234u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (LEAVE_CRITICAL_SECTION, vec![0x1234])
        );
    }

    #[test]
    fn l68d_0082f190_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f190, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10c]);
    }

    #[test]
    fn l68d_0082f1d0_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f1d0, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x10c]);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    #[test]
    fn fn_0082f610_constructs_the_thread_object() {
        let mut e = Engine::new();
        e.register(BS_THREAD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x24, 0x9999);
            a[0].into_ret()
        });
        e.register(MEMBER_34_CALL, |e, a| {
            // Runs after the vtable is stored.
            assert_eq!(e.mem.u32(a[0] - 0x34), FAM_THREAD_VTABLE);
            a[0].into_ret()
        });
        e.register(CREATE_EVENT_A, |_, _| 0x0abcu32.into_ret());
        e.register(READ_WORD_24, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        let this = e.mem.alloc(0x4c);
        e.mem.write(this, &[0xcd; 0x4c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f610, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                BS_THREAD_CONSTRUCT,
                MEMBER_34_CALL,
                CREATE_EVENT_A,
                READ_WORD_24
            ]
        );
        let args_of = |addr: u32| log.iter().find(|c| c.0 == addr).unwrap().1.clone();
        assert_eq!(args_of(BS_THREAD_CONSTRUCT), vec![this]);
        assert_eq!(args_of(MEMBER_34_CALL), vec![this + 0x34]);
        assert_eq!(args_of(CREATE_EVENT_A), vec![0, 0, 0, 0]);
        assert_eq!(args_of(READ_WORD_24), vec![this]);
        assert_eq!(e.mem.u32(this), FAM_THREAD_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x0abc);
        assert_eq!(e.mem.u32(this + 0x3c), 1);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x9999);
    }
}
