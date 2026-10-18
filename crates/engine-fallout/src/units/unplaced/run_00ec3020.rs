//! `(unplaced)/run_00ec3020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The counter incremented (`.data`).
const COUNTER_011C15E4: u32 = 0x011c_15e4;
/// `__Mtxinit` (`00ec3acf`, cdecl, one word: the critical section to initialise; Ghidra's
/// library match for the body is `InitializeCriticalSection`).
const MUTEX_INIT: u32 = 0x00ec_3acf;
/// First address after the last 0x18-byte object the loop initialises, and the first object.
const OBJECTS_END: u32 = 0x0126_fb90;
const OBJECTS_START: u32 = 0x0126_fb30;

// Translated from 00ec3289 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a runtime-library one-time initialisation): calls
/// `InterlockedIncrement(&011c15e4)`; when the result is 0, calls `00ec3acf` once for each
/// 0x18-byte object from `0126fb30` while the address is below `0126fb90` (four objects).
/// Returns `this`, which it does not otherwise read.
pub fn fn_00ec3289(e: &mut Engine, this: Ptr) -> Ptr {
    if e.call(INTERLOCKED_INCREMENT, &args![COUNTER_011C15E4])
        .u32()
        == 0
    {
        let mut object = OBJECTS_START;
        loop {
            e.call(MUTEX_INIT, &args![object]);
            object += 0x18;
            if (object as i32) >= OBJECTS_END as i32 {
                break;
            }
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec_3289, fn_00ec3289(Ptr) -> Ptr)]
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
    fn initialises_four_objects_when_the_increment_gives_zero() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MUTEX_INIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3289, &args![0x55u32]).u32(), 0x55);
        assert_eq!(
            calls_to(&mut e, INTERLOCKED_INCREMENT),
            vec![vec![COUNTER_011C15E4]]
        );
        assert_eq!(
            calls_to(&mut e, MUTEX_INIT),
            vec![
                vec![0x0126_fb30],
                vec![0x0126_fb48],
                vec![0x0126_fb60],
                vec![0x0126_fb78]
            ]
        );
    }

    #[test]
    fn does_nothing_more_when_the_increment_is_not_zero() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(MUTEX_INIT, |_, _| panic!("not initialised"));
        assert_eq!(e.call(0x00ec_3289, &args![0x55u32]).u32(), 0x55);
    }
}
