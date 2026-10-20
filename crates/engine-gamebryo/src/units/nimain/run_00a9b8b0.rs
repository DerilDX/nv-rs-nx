//! `NiMain/run_00a9b8b0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a9b8b0` (no stack words): the first start-up step `00a9ba80` runs.
const FIRST_STEP: u32 = 0x00a9_b8b0;
/// `00a9b9b0` (no stack words): the step `00a9ba80` ends with, by a jump.
const SECOND_STEP: u32 = 0x00a9_b9b0;

// Translated from 00a9ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): runs `00a9b8b0`, then jumps to `00a9b9b0` (the exe leaves
/// that function's result in `EAX`; neither is read here).
pub fn fn_00a9ba80(e: &mut Engine) {
    e.call(FIRST_STEP, &args![]);
    e.call(SECOND_STEP, &args![]);
}

// Translated from 00a9b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; no arguments): fills the
/// dword table at `011f5fc4` with 0 to 13 (14 words), the byte table at `011f5fb4` with 0 to
/// 13 (14 bytes) and sets the byte at `011f6000` to 1.
pub fn fn_00a9b8b0(e: &mut Engine) {
    for index in 0..14u32 {
        e.mem.set_u32(0x011f_5fc4 + 4 * index, index);
    }
    for index in 0..14u32 {
        e.mem.set_u8(0x011f_5fb4 + index, index as u8);
    }
    e.mem.set_u8(0x011f_6000, 1);
}

// Translated from 00a9b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; no arguments): fills the
/// word table at `011f5f7c` with `0x100 * i` for i = 0 to 8 (9 words), the dword table at
/// `011f5f90` with 0 to 8 (9 dwords) and sets the byte at `011f6001` to 1.
pub fn fn_00a9b9b0(e: &mut Engine) {
    for index in 0..9u32 {
        e.mem
            .set_u16(0x011f_5f7c + 2 * index, (index * 0x100) as u16);
    }
    for index in 0..9u32 {
        e.mem.set_u32(0x011f_5f90 + 4 * index, index);
    }
    e.mem.set_u8(0x011f_6001, 1);
}

/// The critical section `00a9bb60` holds (`011f600c`).
const SEQUENCE_LOCK: u32 = 0x011f_600c;
/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection` (`stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The global `NiTArray` `00a9bb60` empties (`011ac040`; its 16-bit used size at +0xa).
const SEQUENCE_ARRAY: u32 = 0x011a_c040;
/// `00a9ba90` (`thiscall` on the array, `RET 8`: the index and the address of a word).
const REMOVE_AT: u32 = 0x00a9_ba90;
/// `00a47770` (`thiscall`, `RET 4`; engine map
/// `NiTArray<NiPointer<NiControllerSequence>,...>::SetSize`).
const ARRAY_SET_SIZE: u32 = 0x00a4_7770;

// Translated from 00a9bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): inside the critical section `011f600c`, calls `00a9ba90` on the array at
/// `011ac040` for each index below its used size (read again after each call), passing the
/// address of one stack word set to 0 once before the loop, then gives the array size 0
/// (`00a47770`).
pub fn fn_00a9bb60(e: &mut Engine) {
    e.call(ENTER_CRITICAL_SECTION, &args![SEQUENCE_LOCK]);
    e.with_stack(4, |e, word| {
        let mut index = 0u32;
        if e.mem.u16(SEQUENCE_ARRAY + 0xa) > 0 {
            e.mem.set_u32(word.addr(), 0);
            loop {
                e.call(REMOVE_AT, &args![SEQUENCE_ARRAY, index, word]);
                index = index.wrapping_add(1);
                if index >= e.mem.u16(SEQUENCE_ARRAY + 0xa) as u32 {
                    break;
                }
            }
        }
    });
    e.call(ARRAY_SET_SIZE, &args![SEQUENCE_ARRAY, 0u32]);
    e.call(LEAVE_CRITICAL_SECTION, &args![SEQUENCE_LOCK]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_ba80, fn_00a9ba80()),
        entry!(0x00a9_b8b0, fn_00a9b8b0()),
        entry!(0x00a9_b9b0, fn_00a9b9b0()),
        entry!(0x00a9_bb60, fn_00a9bb60()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn runs_the_two_steps_in_order() {
        let mut e = Engine::new();
        e.register(FIRST_STEP, |_, _| Ret::default());
        e.register(SECOND_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a9_ba80, &args![]);
        let order: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(order, vec![0x00a9_ba80, FIRST_STEP, SECOND_STEP]);
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn first_tables_hold_their_counting_values() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x2000);
        e.call(0x00a9_b8b0, &args![]);
        for index in 0..14u32 {
            assert_eq!(e.mem.u32(0x011f_5fc4 + 4 * index), index);
            assert_eq!(e.mem.u8(0x011f_5fb4 + index), index as u8);
        }
        assert_eq!(e.mem.u8(0x011f_6000), 1);
        assert_eq!(e.mem.u32(0x011f_5fc4 + 4 * 14), 0);
    }

    #[test]
    fn second_tables_hold_their_counting_values() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x2000);
        e.call(0x00a9_b9b0, &args![]);
        for index in 0..9u32 {
            assert_eq!(e.mem.u16(0x011f_5f7c + 2 * index), (index * 0x100) as u16);
            assert_eq!(e.mem.u32(0x011f_5f90 + 4 * index), index);
        }
        assert_eq!(e.mem.u16(0x011f_5f7c + 18), 0);
        assert_eq!(e.mem.u8(0x011f_6001), 1);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a9bb60_removes_every_index_inside_the_lock() {
        let mut e = Engine::new();
        e.map(SEQUENCE_ARRAY & !0xfff, 0x1000);
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        // Each removal takes the last element away, as a shrinking array would; the
        // word passed stays 0 until the double changes it.
        e.register(REMOVE_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), if a[1] == 0 { 0 } else { 9 });
            e.mem.set_u32(a[2], 9);
            Ret::default()
        });
        e.mem.set_u16(SEQUENCE_ARRAY + 0xa, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a9_bb60, &args![]);
        let log = e.call_log.take().unwrap();
        let indices: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == REMOVE_AT)
            .map(|call| call.1[1])
            .collect();
        assert_eq!(indices, [0, 1, 2]);
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![SEQUENCE_LOCK]));
        assert_eq!(
            log[log.len() - 2],
            (ARRAY_SET_SIZE, vec![SEQUENCE_ARRAY, 0])
        );
        assert_eq!(
            log[log.len() - 1],
            (LEAVE_CRITICAL_SECTION, vec![SEQUENCE_LOCK])
        );

        // An empty array: no removal.
        e.mem.set_u16(SEQUENCE_ARRAY + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_bb60, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != REMOVE_AT));
        assert_eq!(log.len(), 4);
    }
}
