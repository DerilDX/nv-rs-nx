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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_ba80, fn_00a9ba80()),
        entry!(0x00a9_b8b0, fn_00a9b8b0()),
        entry!(0x00a9_b9b0, fn_00a9b9b0()),
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
